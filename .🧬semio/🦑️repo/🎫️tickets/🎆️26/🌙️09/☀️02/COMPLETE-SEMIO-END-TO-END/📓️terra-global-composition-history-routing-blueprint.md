# Global Composition History Routing Blueprint

## Verdict

**RED — document undo/redo has no document-wide route owner.** A group which mutates only a child is correctly applied and stamped by `CompositionCoordinator`, but ordinary framework undo and redo discover a group only from the parent store. The result is either a benign parent-only no-op or an undo of an unrelated parent tail; it never selects the child-only group. This is a production routing gap, not a missing test assertion.

This report is source-only. No Cargo, Nx, browser, or runtime command was run.

## Current evidence

### The lost child-only route

- `VcsArtifactApp::dispatch_emit` routes any non-empty `child_emits` into `dispatch_emit_group` at [`plugin/🦀️.rs:20923`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20923>).
- [`dispatch_emit_group`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21022>) calls `CompositionCoordinator::dispatch_group`; it later records a session command row with `edit_id: None` when the parent had no edit and only places child ids in `child_edit_ids` ([`21135–21223`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21135>)). That command log is explicitly runtime-only ([`18999–19002`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18999>)).
- The group coordinator deliberately permits `parent_ops = []`: it applies each non-empty child, stamps the same invocation id, and emits a receipt containing children only ([`store/🦀️.rs:19509–19565`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19509>)). This part is sound.
- `commit_framework_history_route` identifies a group only via `self.store.tail_group_id()` or the parent `redo_tail()` ([`plugin/🦀️.rs:22004–22013`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22004>)). A child-only receipt left neither parent value set, so control falls through to a parent-only `ArtifactCommand::Undo`/`Redo`.
- The same parent-only assumption is visible in history UI: `build_history_view` derives `can_undo`, `can_redo`, each command row's applied status, and its document reversibility exclusively from `self.store` ([`20387–20545`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20387>)). `revertToCommand` also walks only `self.store` ([`22035–22062`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22035>)).

`CompositionCoordinator::undo_group` and `redo_group` already have the required per-member foreign-tail semantics: a matching member moves, a foreign/missing tail is reported as skipped, and other members continue ([`store/🦀️.rs:19572–19635`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19572>)). The app's wrapper currently hands those functions **all** live children, not the receipt's original members ([`plugin/🦀️.rs:21265–21303`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21265>)); this makes an old group's diagnostic set depend on unrelated children which appeared later.

### Complete mutation/ingress census

| Route | Current authority | Required route-owner behavior |
| --- | --- | --- |
| Ordinary parent action | `dispatch_emit` uses `self.store.dispatch` at [`20952–20995`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20952>). | Append/move one `Parent` route entry only when a new document edit is created; an `AmendLast` retains the existing entry rather than adding an artificial history turn. |
| Owned mixed or child-only action | `dispatch_emit_group` / `dispatch_group` at [`21022–21135`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21022>) and [`19441–19565`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19441>). | Append one `Group` entry from the returned receipt plus exact member owner coordinates. It is the missing route. |
| Framework undo/redo | Parent-tail probe plus parent dispatch at [`22004–22032`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22004>). | Consult global applied/redo route first; do not infer a group from a parent group id absent a route entry. |
| Revert-to-command | Parent `applied_edit_ids` loop at [`22035–22062`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22035>). | Resolve the command row to a route id, then undo later *route* entries. A route is indivisible once execution starts. |
| Host transaction | Per-instance parent `Apply`, group stamp, origin stamp at [`24801–24808`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24801>); host fan-out is a separate cross-plugin coordinator ([`host/🦀️.rs:8012–8224`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:8012>)). | Add a parent-only entry to the local document route after the local commit. Do not mistake host fan-out for composition history or synthesize child members from it. |
| Remote ingress | `ingest_operations` delegates each wire envelope to `self.store.ingest_remote` ([`24981–24985`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24981>)); store ingress atomically controls its own DAG/history/cursor ([`store/🦀️.rs:16081–16089`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16081>). | Do not hook individual member ingress into the global route. A remotely shared composition route needs its own verified event carrying the full member list. Until then, keep it foreign/unroutable instead of inferring cross-store order. |
| Text ingress | `ingest_operations_text` runs parent `Apply` at [`24995–24999`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24995>). | Treat as one parent entry when locally authored. |
| Load/hydrate | document text/pack reset the parent store at [`25007–25028`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25007>); hydration delegates into the same pack load at [`24846–24851`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24846>). | Decode a persisted route if present. With no route, never infer a child group from parent/local state; the composed document is ineligible for global grouped undo/redo. |

`ArtifactEnvelope` currently persists only one store's cursor (`applied_edit_ids`, `redo_edit_ids`, checkpoint) ([`store/🦀️.rs:2132–2248`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2132>) and serializes it as the optional `cursor` field ([`2308–2387`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2308>)). There is no durable app/composition route in the envelope or app state. A session `command_log` cannot repair restart/reload.

## Smallest correct owner

Add a first-party, schema-owned `CompositionHistoryRouteV1` to the *parent document's durable composition state*, not to `ArtifactStore`'s generic local cursor and not to the volatile command log.

```text
CompositionHistoryRouteV1 {
  parent: ArtifactRef,                 // full id + dialect
  version: 1,                          // closed durable schema version
  applied: Vec<RouteEntry>,            // oldest → newest
  redo: Vec<RouteEntry>,               // next redo at tail, mirror store semantics
}

RouteEntry =
  Parent { route_id, parent_edit_id }
| Group {
    route_id,
    invocation_id,
    members: Vec<MemberTarget>,        // exact original order, nonempty
  }

MemberTarget {
  target: ArtifactRef,
  owner: Option<OwnerRef>,             // None only for exact parent; child includes parent/slot/child_id
  edit_id: String,
  pre_revision: [u8; 32],              // durable/reconstructable cursor state
  post_revision: [u8; 32],
}
```

`GroupReceipt.member_edits` supplies `target` and `edit_id`, but not the child slot/owner. The app must pair receipt entries with the validated `ChildEmit` coordinate while it still owns that input, or extend the receipt before any persistence. Do **not** resolve by child id alone: collection slots can contain distinct child ids and a later graph can change; stored `OwnerRef` is the authority.

The owner belongs beside `store`, `composition`, and `children` in [`VcsArtifactApp`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18761>) because it controls UI ordering across stores. `ArtifactStore` continues to own each member's VCS cursor, causal DAG, and remote ingest invariants. The route is only the authoritative cross-store selection index.

### Apply, undo, redo and fallback rules

1. Pre-admit a fixed route candidate and its eventual close authority *before* a parent or group mutation. If pre-admission fails, reject before any member dispatch. This avoids a post-commit allocation/retirement failure losing the route record.
2. On a completed ordinary parent `Apply`, append/move `Parent { edit_id }`; on `AmendLast` verify the tail entry already names the same edit and do not add a turn. A new local apply clears the route redo side exactly once.
3. On a completed group, validate receipt membership against the prepared target list, then append one `Group`. The route may be published only after child root publication/created-child absorption succeeds. Current `dispatch_emit_group` commits stores before those steps ([`21090–21118`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21090>)); therefore this routing packet must use the already-planned retained child-publication transaction or remain explicitly non-atomic. It must not claim all-or-nothing composition publication by adding an index alone.
4. **Superseded by the current re-audit below:** undo/redo must select only an exact stored route entry, but the new durable path cannot use the existing best-effort group primitive. It must preflight every recorded member and either move the complete route or leave every member and route side unchanged.
5. Cancellation is permitted before preparation and before the first member state change. Once a group undo/redo begins, it must be an owned operation that drives to a terminal report; returning a late cancel after partially moving members would corrupt the route cursor. Yield only between bounded member steps and retain the candidate/report/child-root retirement until terminal.
6. **Superseded by the current re-audit below:** load must never infer a composition route from local group stamps or live children. A composed parent without a strict persisted route is not eligible for global grouped undo/redo.

### Ownership and close

The route must have a dedicated bounded retirement owner; do not drop `Vec<String>`, `ArtifactRef`, or `OwnerRef` behind an unbounded generic disposer. The existing store pattern is usable as a model: `ArtifactCursor` holds staged owners in `ManuallyDrop`, forbids ordinary mutation while staged, and asserts on nonterminal drop ([`store/🦀️.rs:2145–2234`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2145>)); the store already has bounded string/vector/cursor retirement implementations ([`390–572`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:390>)).

Use that pattern, but keep the type app/composition specific:

- `CompositionHistoryRouteRetirement` releases one route entry/member string/ref field per grant, with a fixed active member cursor; it has no `Drop` shortcut.
- an in-flight `CompositionHistoryOperation` owns the route admission, route mutation candidate, exact member handles, child-root publication reservations, and terminal report. It returns the untouched candidate on cancel/preflight denial and hands terminal data to the route only once.
- `VcsArtifactApp::maintenance_step` and `close_step` must drive this owner before the document/config/draft disposers; expose no raw route vectors to command handlers.

## Required law packet

Create a schema + neutral fixture/oracle under the composition history taxonomy, then a focused registered kernel/plugin gate before a broad suite.

Neutral rows must cover:

1. `Parent(A) → child-only(G) → Parent(B)`: undo order `B, G, A`; redo restores `A, G, B` exactly.
2. child-only group with no parent cursor: `canUndo=true`, then the exact child is moved and a child-root publication is requested.
3. mixed group carries parent plus two recorded children; a later unrelated child is never queried or diagnosed.
4. a recorded child with foreign tail rejects the entire durable route before any member moves; diagnostics name only recorded targets.
5. all recorded targets foreign/missing: no route movement, no history event, diagnostics preserved.
6. redo mirror with one foreign child and a matching parent likewise has zero movement; member order is fixture-pinned for the successful case.
7. new parent apply clears only route redo after route admission; an amended parent edit does not create a second route turn.
8. close/cancel at each pre-mutation stage returns the exact candidate; late cancellation drives one terminal report rather than exposing a partially moved cursor.
9. a loaded parent with cursor but no composition route never adopts a child tail with the same group string by inference; it is ineligible for global composition undo/redo.
10. malformed route: duplicate `route_id`, empty group, parent with `OwnerRef`, child without exact owner, owner/target mismatch, duplicate member, foreign parent ref, invalid revision/route id, oversized strings/member count.

Native plugin laws should exercise real `VcsArtifactApp<TestApp, TestMembers>` for rows 1–6, including the current `CompositionCoordinator` foreign-tail path. Add a real pack/load/reopen law for row 9 once the composition route is encoded. A process/socket law belongs **after** a durable composition event exists; current per-store `ingest_remote` has no envelope capable of establishing cross-store route order, so claiming reconnect coverage earlier would be false.

Suggested registered command after adding the source-first oracle and exact native names:

```text
bun x nx run @semio-tech/framework-os-kernel:composition-history-route-check --skip-nx-cache -- -j 1
```

Register it through the existing product OS Rust `📜️script.ts` and generated launch seed, not by editing generated `launch.json` directly. This command is proposed, not currently registered or run.

## Current re-audit — durable route requirements after atomic publication

This re-audit is source-only on the current tree. It corrects two parts of the earlier blueprint: a live store `generation` is not durable identity, and the present best-effort `CompositionCoordinator` reverse path cannot be the new global route executor.

### Exact current owner split

- [`CommandLogEntry`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:9999>) is explicitly append-only session state. It records only optional parent/config edit ids and bare child edit ids; it has neither `OwnerRef`, child dialect, route order, nor a persistence owner. Its bounded app-close disposal makes it unusable after reload.
- [`VcsArtifactApp::build_history_view`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20548>) derives `can_undo`, `can_redo`, `applied`, and document reversibility exclusively from `self.store`. A child-only group is invisible even though its command-log display row has child ids.
- [`commit_framework_history_route`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22093>) obtains a group id only from the parent member's `tail_group_id`/`redo_tail`; it necessarily falls through for child-only history. [`commit_framework_revert_route`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22124>) similarly walks parent applied ids only.
- [`ArtifactStore::content_revision_now`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:14080>) is an incrementally rebuilt, fixed 32-byte cursor-state digest. It is suitable as an entry's pre/post semantic fence. In contrast, each live store's generation is an adoption/race fence and must stay private to an in-flight operation, never appear in the persisted route or route id.
- [`GroupReceipt`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18854>) exposes only `(ArtifactRef, edit_id)` pairs. `dispatch_emit_group` still owns the matching `(slot, child_id)` input when it obtains the receipt ([`plugin/🦀️.rs:21102`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21102>)); it must create full `OwnerRef` targets there, before publication. A later lookup by child id is not authority: the present history wrapper does exactly that when republishing a moved child ([`plugin/🦀️.rs:21383`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21383>)).
- [`CompositionCoordinator::undo_group` and `redo_group`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19599>) intentionally operate best-effort, independently moving matching members and reporting other members as skipped. [`dispatch_group_history_action`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21354>) builds that input from *all* current children, in registry order. It therefore queries unrelated later children and gives no stable route membership or order.
- [`mint_invocation_id`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19190>) hashes a parent id and sorted child id/op fingerprints. It omits full coordinates, full dialects, document scope, and base revisions; identical repeated operations therefore converge deliberately. It is a group stamp, not a durable unique route identity.
- The only current persistence seam is `ArtifactEnvelopeOwners`/`ArtifactEnvelopeRead` ([`store/🦀️.rs:2308`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2308>)), projected into [`HistoryComposition`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:57>) by `history_composition_from_envelope` / `apply_history_composition` ([`store/🦀️.rs:10925`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10925>). Today that record persists only owner, dialect, and checkpoint pins. Neither `SpaceHistorySnapshot` nor host transaction fan-out carries a composition route.

### Correct smallest durable model

`CompositionHistoryRouteV1` belongs in the parent envelope's composition state and is captured in the same `ArtifactEnvelopeRead` decision as its store cursor. It is a cross-store selection index; each member continues to own its own VCS cursor, edits, and causal DAG.

```text
CompositionHistoryRouteV1 {
  version: 1,
  parent: FullArtifactRef,
  applied: bounded oldest_to_newest RouteEntry[],
  redo: bounded next_to_redo RouteEntry[],
}

RouteEntry {
  route_id: [u8; 32],                  // unique event identity, not invocation_id
  kind: Parent | Group,
  members: bounded canonical MemberTarget[],
}

MemberTarget {
  reference: FullArtifactRef,
  owner: None | exact OwnerRef,        // None iff it is the exact parent
  edit_id: String,
  pre_revision: [u8; 32],
  post_revision: [u8; 32],
}
```

The atomic publisher computes `route_id` from a domain-separated canonical encoding of the parent full reference, exact member coordinates and full dialects in apply order, each stable edit id, and each pre/post cursor revision. It must be distinct from `invocation_id`. The runtime operation also retains captured per-member live generations, but only as an in-memory freshness fence immediately before the single commit decision.

`ArtifactRef` is not a collaboration scope. For a transport-visible or hub-replayed route, the authenticated `DocumentScope` must be carried and checked at the enclosing document mount; equal document ids in two spaces must never share a route identity. The present VCS envelope does not retain a scope. Therefore the first local route may be envelope-local only, and it must not be exported/reconciled across spaces until the authenticated scope bind is added. No default or synthetic scope is acceptable.

### Publication and reverse-operation order

1. The forthcoming parent+children publication operation reserves a bounded `CompositionHistoryRouteRetirement`, captures the old applied/redo route roots, verifies every proposed target's exact owner/ref/dialect and pre-revision, and builds the candidate before changing any member.
2. It stages parent/member graph/root/history changes and the route candidate under the same `ArtifactGroupVisibilityOwner`; the one irreversible decision makes all of them visible together. A new committed route clears redo in that same decision. An `AmendLast` must prove it retains the existing tail edit and route id rather than manufacture a second turn.
3. Persist the resulting route through `ArtifactEnvelopeOwners` -> `ArtifactEnvelopeRead` -> `HistoryComposition` -> `encode_composition`/`decode_composition`, plus the existing envelope/history retirement lists. The current composition format is closed only after its extension has strict field-count, size, duplicate-target, exact-EOF, and version handling; do not silently accept a route-less composed parent by reconstructing one from local group stamps.
4. Replace framework undo/redo selection with a retained `CompositionRouteUndoRedoOperation`: select exactly one tail `RouteEntry`, preflight every recorded member's exact owner, expected tail edit, and pre/post revision, then execute the same transaction discipline. It moves the route cursor only at its final decision. A cancellation before staging returns all owners unchanged; after staging it drives to a terminal committed or aborted result rather than returning a partial cursor.
5. A foreign/missing/stale participant in the **new durable route** rejects the complete entry without moving any member or route side. This differs intentionally from legacy `undo_group`'s best-effort skip report: keeping a global route while only some members move makes subsequent undo/redo lie. The old primitive may remain diagnostic-only while non-routed legacy paths exist, but cannot be used to advance this route.
6. Parent-only and child-only entries share the same applied/redo stack. Thus `Parent(A) -> ChildOnly(G) -> Parent(B)` yields undo `B,G,A` and redo `A,G,B`; a later unrelated live child is never opened, inspected, or reported.

### Exact test matrix

Add a schema-first neutral corpus for route state/selection and bind it independently in Rust before adding any UI claim.

| Law | Required observation |
| --- | --- |
| Mixed global order | `Parent(A) -> ChildOnly(G) -> Parent(B)` undoes `B,G,A` and redoes `A,G,B`, with one route-cursor move per completed entry. |
| Child-only persistence | A child-only entry remains selected after print/parse/reopen; `command_log` loss is irrelevant. |
| Exact targeting | A group with parent plus two children never probes a later unrelated live child; a child root is found only by stored `(slot, child_id, full owner)` and matching dialect. |
| Foreign tail | One recorded participant with a foreign/stale tail causes the new operation to reject before any member or route side changes. Separately retain the current coordinator test proving the legacy primitive's skip behavior, so the semantic distinction is visible. |
| Redo and branch | A fresh parent or group commit clears redo atomically; an amend does not add a route turn; child-only redo uses the stored participant list. |
| Abort/close | Cancellation before staging preserves candidate/route/member state; failure after staging drives bounded retirement to one terminal abort/commit without exposing a half-moved route. |
| Strict load | duplicate route/member, noncanonical order, child without exact owner, parent with owner, wrong full dialect/ref, malformed revision/route id, excess bounds, bad EOF, and missing authenticated export scope are denied. No child route is inferred from a parent group string. |

Native proof needs: (a) a store SPR print/parse/reopen + exact owner-retirement law for the composition route field; (b) an actual `VcsArtifactApp<TestApp, TestMembers>` one-parent/one-child and child-only retained route law, including delayed publication/terminal close; and (c) a real foreign-tail preflight law proving zero member movement. A browser/socket/process proof comes only after a durable scoped composition-route event exists; current per-member remote ingress cannot establish cross-member order.

### Current acceptance boundary

The existing coordinator tests validate only best-effort local member mechanics. The live code has no durable global route, no exact participant selection, no child-only tail discovery, and no atomic route/member visibility decision. Consequently grouped undo/redo, reload, and remote/restart collaboration remain **RED** until the atomic publication transaction and the route field/operation above are implemented and exercised.

## Explicit nonclaims

- This does not make current group store/root/graph publication atomic; that remains the retained child-publication transaction prerequisite.
- This does not transport a global route through the current remote `MutationEnvelope` ingress.
- This does not turn host cross-plugin transaction fan-out into a document composition history source.
- This does not revive a child-only group from a legacy parent pack without a persisted route record.

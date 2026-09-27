# Primary Editing Surface Reachability Audit — 2026-09-27

This is a read-only source audit of the end-user primary editing surfaces. No source was changed, and no build, test, or runtime interaction was run. Therefore **source-reachable** below means that a declared editor action reaches a command handler in the current source; it does not claim a runtime-green result.

The audit follows an action from its window-kind declaration through its render surface and command handler. It covers text, table, image, media, office, spatial, and archive editors, with the shared Details/schema work reported separately in [the preceding capability audit](🪟️details-capability-audit-2026-09-27.md).

## Confirmed Source-Reachable P1: A Bare Document Action Can Destructively Target Page Zero

`DocumentWindowKit::editable_window_kind()` declares `set-page` as a bare mutation catalog action with no argument definition at [framework plugin `🦀️.rs:34334`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34334). The shared contract documents such kit actions as palette rows on every editor that composes the kit at [contract `🦀️.rs:1045`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs:1045). In contrast, the kit renders its document pages only as read-only `TextEditorScene`s at [framework plugin `🦀️.rs:34343`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34343); there is no page-bound text draft that supplies an index or text.

The action therefore has this source-reachable path:

1. Activate the zero-argument `set-page` palette row.
2. DOCX and PPTX parse absent `index` and `text` as `0` and `""`: [DOCX `🦀️.rs:110`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:110), [PPTX `🦀️.rs:114`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:114).
3. DOCX targets the first paragraph and PPTX targets the first text-bearing shape. Both can emit a mutation with empty text. Out-of-range or uneditable targets instead return a successful empty emit, so neither invalid targeting nor a no-op is surfaced as an error.
4. PDF has the same parser defaults at [PDF `🦀️.rs:131`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs:131) and appends an empty `NextLineShowText` operation to page zero at [PDF `🦀️.rs:149`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs:149). Its primary surface also renders a read-only page summary at [PDF main window `🦀️.rs:31`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:31).

The DOCX, PPTX, and PDF primary windows all opt into the editable kit. The problem is also replicated in their sibling subsets that reuse the same pattern.

**Required correction.** Remove `set-page` from a document window until an owner supplies a per-page, pre-populated, explicit-commit `TextDraftView`. That surface must carry the actual page/shape identity and current text. DOCX and PPTX must declare and require their address and text arguments, rejecting absent/malformed values rather than selecting zero. PDF should remain on the non-editable document window until it owns a faithful page-content operation and interaction. Add direct command tests for a missing payload, a stale target, and an explicit edit of a nonzero page/shape.

**Status:** current source-reachable P1. The text/office owner confirmed that the shared document action remains unchanged and is preparing a direct draft-based fix; it has not been runtime-verified in this audit.

## Confirmed Source-Reachable P1: A Bare Mesh Action Defaults to the Origin

`MeshWindowKit::editable_window_kind()` declares a zero-argument `set-vertex` mutation at [framework plugin `🦀️.rs:34301`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34301). Both Semio Mesh and BREP primary editors expose it through their editable main window definitions: [Mesh main `🦀️.rs:24`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:24) and [BREP main `🦀️.rs:24`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:24). Their scenes render an empty rectangle-selection configuration and no payload-producing edit control: [Mesh `🦀️.rs:62`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:62), [BREP `🦀️.rs:60`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:60).

For Semio Mesh, the action parser maps missing `meshIndex`, `primitiveIndex`, `vertexIndex`, and `point` to zero at [Mesh editor `🦀️.rs:139`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🦀️.rs:139). If mesh 0 / primitive 0 / vertex 0 exist, the handler emits `MoveVertex` with `[0, 0, 0]` at [Mesh editor `🦀️.rs:112`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🦀️.rs:112).

For BREP, the parser maps an omitted point to `[0, 0, 0]` at [BREP editor `🦀️.rs:150`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🦀️.rs:150). When the interaction has a selected vertex, its handler moves that selected vertex to the origin at [BREP editor `🦀️.rs:134`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🦀️.rs:134).

**Required correction.** Do not publish `set-vertex` as a bare palette action. Provide a spatial edit interaction or draft that contains a validated target identity and a required finite three-coordinate point; reject absent, partial, non-finite, stale, and non-addressable payloads with a fault. Until then, both primary windows must use the non-editable mesh definition. Tests must prove that no-payload invocation emits neither a mutation nor a successful-looking empty result, and that a nonzero selected target receives the supplied point.

**Status:** current source-reachable P1, reported to the media/spatial owner. No runtime claim is made.

## Confirmed Source-Reachable P2: ZIP Rename Accepts an Invalid or Stale Address as a Silent Success

The ZIP base main window exposes `TreeWindowKit::editable_window_kind()` at [ZIP main `🦀️.rs:28`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:28). Its parser accepts a node id and value with empty fallbacks at [ZIP editor `🦀️.rs:124`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs:124). The handler silently returns `Emit::default()` for an unrecognized `nodeId`, a nonnumeric `entry:` suffix, or an entry index no longer in the snapshot at [ZIP editor `🦀️.rs:157`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs:157). ISO 21320 mirrors the same behavior at [ISO ZIP editor `🦀️.rs:159`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🦀️.rs:159).

This is an end-user failure-preservation gap: a stale entry selection loses the proposed rename without a fault or retained draft. It is P2 rather than P1 because the primary tree does not itself supply a direct inline entry rename interaction; the generic action remains palette-reachable.

**Required correction.** Make the action address schema explicit and have the handler fault for an invalid/stale address while retaining the attempted value in its draft. Bind actual tree leaves to valid immutable entry identity rather than a transient vector index. Keep the documented scope: this tree correctly does not pretend to edit compressed entry bytes ([ZIP main `🦀️.rs:4`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:4)).

**Status:** current source-reachable P2. No code owner was assigned by this audit.

## Surface Status Observed During This Audit

| Surface | Source status | Evidence and disposition |
| --- | --- | --- |
| TXT, Markdown, HTML, XML | Authored current route | Natural-source text editors use explicit text-draft style actions; XML supplies its root id through `set-node` and validates its snapshot before publication. No new P1 found in the traced source. |
| JSON | Authored current route | The text/office owner reports the natural source now emits `set-node` at the JSON root. This audit did not execute it. |
| CSV and TSV | Authored current route | Their editable table cells provide the row/column payload that the table command consumes. No new P1 found in the traced path. |
| XLSX | Unresolved source risk | The primary table maps a rendered ordinal row back through a flattened worksheet-cell sequence. An action does not carry a stable worksheet/cell address, so stale or reordered data needs an explicit identity/rebase audit before claiming safe targeting. |
| PNG | Pending worker-owned correction | The media/spatial owner is replacing the prior dormant whole-pixel-vector replacement with typed region arguments and a cancellable retained `PatchPixels` job. This audit does not file the in-progress implementation as a current defect. |
| Audio and video | No editable primary owner found in inspected source | Inspected media surfaces use read-only window definitions. The shared `seek-media` catalog declaration alone is not proof of an end-user mutation path. |
| DOCX, PPTX, PDF | Current source-reachable P1 | Covered above. The observed pages are read-only summaries while their palette mutation is unparameterized. |
| Semio Mesh and BREP | Current source-reachable P1 | Covered above. Both expose a zero-argument spatial mutation without an input-producing primary control. |
| ZIP base and ISO 21320 | Current source-reachable P2 | Covered above. Archive name/comment editing has an invalid-address silent-success path; byte payload editing remains honestly out of scope. |
| BCF | No new P1 found in inspected table route | The traced editor uses a table-cell action route. It still needs ordinary stale-address behavior tests, but no direct default-to-destructive mutation was found in this pass. |

## Required Shared Interaction Primitives

The confirmed P1s share a missing rule: a generic window kit must not make a mutation palette-reachable unless its action declaration has a complete validated payload contract or the owning surface renders an explicit, pre-populated draft/control that supplies it. A fallback parser is not an interaction primitive; for a target or replacement value it becomes an unintended edit.

The framework needs reusable action definitions for required address and payload fields, plus a failure path that preserves a draft on stale/invalid targets. The document, mesh, and archive owners should use those primitives instead of interpreting omitted coordinates, indices, node ids, or text as a harmless default.

## Validation Status

No build, Cargo job, test, or runtime interaction was run. The native shared-contract/all88 work was already queued elsewhere, so this report does not claim that any source path compiles or is runtime-green. The concrete tests named under each finding remain required after the responsible implementation lands.

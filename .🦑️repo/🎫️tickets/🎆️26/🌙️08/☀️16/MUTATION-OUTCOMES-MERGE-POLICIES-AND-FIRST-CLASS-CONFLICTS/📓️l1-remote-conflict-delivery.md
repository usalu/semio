# Lane L1 — closing the last two gaps (report)

## Gap 1 — remote-origin merge/conflict delivery

### Where the drop actually was

`🟦️backbone-worker.ts` never references `AppFrame` at all (grepped, zero hits) — it only speaks
`protocol_wire::ClientFrame`/`ServerFrame` (the hub relay protocol), which has no `MergeReport`/
`Conflicts` variant and never will under this contract (C8 only extended `AppFrame`/`AppCommand`).
K1/K2's "AppFrame::Error → bare conflict event" description was actually `ServerFrame::Error` — a
transport-level diagnostic, not a `Conflict` roster.

The REAL "guest pushes `MergeReport`/`Conflicts` unsolicited after every ingest" path (contract
freeze §C9: "`ApplyEnvelopes` handling emits `MergeReport`/`Conflicts`") is
`entry.plugin.applyMutations` — called from `ShellHost`'s `remoteMutations` event handler whenever
the hub relay (`🟦️backbone-worker.ts`) delivers a peer's envelopes. Its implementation lives in
`PluginRuntime/🟦️component.tsx`, and it only ever checked the reply frames for `Error`, discarding
`MergeReport`/`Conflicts` from the SAME batch. Confirmed via `MergeReport`'s own doc comment
(`🎠️kernel/🟦️component.ts`): "Packed onto the wire as `AppFrame::MergeReport.report`, pushed
unsolicited after every ingest". `PluginRuntime` is outside the ticket's literal file list but,
per K2's identical precedent, is the only place the fix can genuinely land.

### Fix

- **`PluginRuntime/🟦️component.tsx`**: `applyMutations`'s type/implementation changed from
  `Promise<void>` to `Promise<{mergeReport: MergeReport|null; conflicts: readonly Conflict[]|null}>`,
  reusing the exact same `decodeMergeReportFromWire`/`decodeConflictsFromWire` decoders
  `resolveConflict` already uses (no second codec). The two other `applyMutations` call sites
  (tutorial playback, `ShellHost` lines ~3494/~3571) already discard the return value with a bare
  `await`, so they compile unchanged.
- **`ShellHost/🟦️component.tsx`**:
  - New ref `applyRemoteMergeRef` (same ref-forwarding idiom as the file's existing
    `dispatchDirectoryEventsRef`/`openArtifactWithAppRefRef`, since `showTransientNotice`/`shellLabel`
    are declared later in the component than the `remoteMutations` handler that needs them).
  - `remoteMutations` handling now awaits `applyMutations`'s result and routes it through
    `applyRemoteMergeRef.current(conflicts, mergeReport)` instead of firing the call with `void` and
    ignoring the outcome.
  - New `applyRemoteMerge` callback (assigned to the ref): a non-null `conflicts` roster dispatches
    `SET_CONFLICTS` — the SAME action `dispatchResolveConflict`/`readConflicts` already use, so
    `ChromePanels`' Conflicts panel and `ShellSync`'s quarantine badge (both derive from
    `state.merge.conflicts` via `selectOpenConflicts`/`selectQuarantinedConflicts`) pick up a
    REMOTE conflict with zero further wiring. When the flagged conflict (`mergeReport.conflict`)
    is specifically `kind: "degraded"`, it additionally raises a transient notice via
    `showTransientNotice` — this authority's only real "surfaces without being asked" channel,
    the same convention `showMutationRejectedNotice` already uses for a LOCAL rejected dispatch.
  - Cleaned up the now-accurately-understood `event.kind === "conflict"` branch: this is
    `🏪️store/🔄️sync/🦀️component.rs`'s hub-relay `ArtifactEvent::Conflict` (a transport diagnostic,
    never a `Conflict[]` roster — no `ServerFrame` variant could ever carry one), so the dead
    duck-typed `conflicts` field check was removed; it stays a passive `console.warn`, same as before.

### Why not a literal "EventFeedHost" wire

`EventFeedHost` is a generic scene-DSL component (`entriesJson` parsed from an app's own UI tree);
it has zero references anywhere in `ShellHost`/`ChromePanels`/`Shell` and there is no `ShellState`
slice feeding it (verified, not assumed). Wiring a brand-new chrome-owned event-feed concept was out
of the two-gap scope handed to this lane; the transient-notice channel is the actual, already-wired
"surfaces to the user without being asked" mechanism this same contract established for the LOCAL
case, so it is reused here for the REMOTE case rather than inventing a second one.

### New test

`🧪️index.test.ts` (`framework plugin runtime` describe block): "adaptPluginHandle.applyMutations
decodes an unsolicited MergeReport/Conflicts reply and it reaches ShellState" — a fake plugin handle
replies to `AppCommand::ApplyEnvelopes` with `MergeReport`+`Conflicts` frames (no `Error`), asserts
`applyMutations` now returns them decoded (previously `Promise<void>` silently dropped them), then
feeds the roster through `shellReducer`'s `SET_CONFLICTS` and asserts both `selectOpenConflicts` and
`selectQuarantinedConflicts` see the remote-origin conflict — the reducer-level equivalent of what
`ShellHost`'s `applyRemoteMerge` does, without needing a mounted `ShellHost`.
`selectOpenConflicts`/`selectQuarantinedConflicts` had to be added to the `⚛️react` package's
`📦️index.tsx` barrel (region `🧮️ShellStore`) — they existed on `Shell` but were never re-exported.

## Gap 2 — `SpaceConflict` residue in `🏪️store/🔄️sync/🦀️component.rs`

**Migrated** (not deleted — the site is live code, exercised on every hub `ServerFrame::Error` and
every folder-poll external-divergence detection, both native actor and `wasm_actor` twin).
`ArtifactEvent::Conflict(SpaceConflict{kind, uri, message})` → `ArtifactEvent::Conflict(MutationMessage
{level, code, message, target, op_index})` — `MutationMessage` (`📡️spr/🎮️command`) is the frozen
diagnostic-bag vocabulary the ticket pointed at; `Fault` was considered and rejected (its extra
`origin`/`scope`/`causes` fields have no honest value to synthesize from a bare `{code, message}`
hub frame). `code`→`code` (wrapped in `FaultCode`, which is a free-form string newtype, not
restricted to the frozen 7 `mutation.*` codes — those govern `diff`-leaf outcomes, not transport
diagnostics), `message`→`message`, `uri`→`target: vec![uri]`, `level` set to `Error` (neither source
event — external folder divergence, or a hub protocol reject — carries a finer signal). This also
incidentally fixes a latent field collision: `SpaceConflict.kind` and the enum's own
`#[serde(tag = "kind")]` discriminant shared the JSON key `"kind"`; `MutationMessage` has no `kind`
field. All 3 construction sites (native `externalDivergence`, native `ServerFrame::Error`, wasm_actor
`ServerFrame::Error`) migrated identically. TS mirror `SyncConflict` in `💻️os/🟦️component.ts` is
already loosely typed (`{message?} & Record<string, unknown>`) so no shape change was needed; only
its doc comment (which named the now-deleted-from-this-file type) was corrected.

**`SpaceConflict` no longer exists anywhere in this lane's lease** (`🔄️sync/🦀️component.rs`,
`💻️os/🟦️component.ts` — confirmed via `rg`, zero hits in both). It is **not** zero repo-wide:

| File | Hits | Nature |
|---|---|---|
| `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` | 1 | comment only |
| `💻️os/🖥️host/🦀️component.rs` | 12 | live: workflow-snapshot reconciliation, `validate_parameter_config_binding` |
| `🔨️modules/🔁️workflow/🦀️component.rs` | 3 | live: `validate_workflow_parameter_config_binding` |
| `🔨️modules/🏪️store/🦀️component.rs` | 2 | the struct definition itself + testkit |
| `🔨️modules/🔌️plugin/🦀️component.rs` | 5 | live: `assert_graph_merge_preserves_referential_integrity` testkit helper + comments |

These are **not** wasm-gated leftovers like the site this lane's lease covered — they are actively
compiled, tested native code in `host`/`workflow`/`plugin`, none of which are in this lane's lease
(`🏪️store/🔄️sync/🦀️component.rs` only). Deleting `SpaceConflict` repo-wide means rewriting workflow
parameter-binding validation's error type and a graph-merge referential-integrity testkit helper
across 3 unleased files/modules — a materially larger, cross-cutting change outside this lane's two-
gap scope. Left untouched and reported here, matching K1's identical precedent for the same finding.

## Verify (real numbers, this session)

- `framework-os` vitest (`💻️os/📦️packages/🟦️typescript`): **334 passed, 2 failed, 336 total** —
  identical to baseline (2 pre-existing wasm-pkg-missing-module failures). `🧪️l1-os-vitest.txt`.
- `framework-renderer-react` vitest (`…/⚛️react`): **311 passed, 9 failed, 320 total** — same 9
  pre-existing failure names as baseline (none touch merge/conflict/applyMutations code), +1 new
  passing test (this lane's). `🧪️l1-renderer-react-vitest.txt`.
- Root `bunx tsc --noEmit -p tsconfig.json`: **19 errors**, byte-identical file:line set to baseline,
  zero in any touched file. `🧪️l1-tsc.txt`.
- `cargo check -p semio-framework-os-kernel`: clean, 0 errors (9 pre-existing warnings, none new).
  `🧪️l1-cargo-check.txt`.
- `cargo test -p semio-framework-os-kernel --lib`: **987 passed, 0 failed** — matches baseline
  exactly. `🧪️l1-cargo-test.txt`.
- `bun ./📜️script.ts verify mutation-outcome-law`: **passed, 0 breaches**. `🧪️l1-verify-mutation-outcome-law.txt`.
- `rg SpaceConflict` repo-wide (excluding `.🦑️repo/`): **0 hits in this lane's lease**, 23 hits across
  5 unleased files (table above, real numbers). `🧪️l1-spaceconflict-grep.txt`.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
  (outside the stated lease — see "Where the drop actually was" above, same justification K2 used)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🟦️component.ts` (one doc-comment line, `SyncConflict`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
  (re-export `selectOpenConflicts`/`selectQuarantinedConflicts`, region `🧮️ShellStore`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
  (new regression test + two new imports)
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`, `…/📦️glue.backbone-worker.ts` — investigated,
  **not edited**: neither references `AppFrame`, and `glue.backbone-worker.ts` is a 2-line re-export
  with nothing to keep in sync.

Logs (this folder): `🧪️l1-os-vitest.txt`, `🧪️l1-renderer-react-vitest.txt`, `🧪️l1-tsc.txt`,
`🧪️l1-cargo-check.txt`, `🧪️l1-cargo-test.txt`, `🧪️l1-verify-mutation-outcome-law.txt`,
`🧪️l1-spaceconflict-grep.txt`.

Ticket not closed (lane instruction: never close a shared ticket).

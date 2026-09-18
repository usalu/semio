# 🔱️ Trinity plugin end to end — status

## 2026-09-17

- Ticket opened on disk (repo MCP timed out). Pattern follows 26/09/16/LAYOUT-PLUGIN-END-TO-END.
- Playgrounds already declared in `📦️packages/🦀️rust/Cargo.toml`: `trinity-jack` (react 6054) and `trinity-rewriting` (react 6056), both with `app` pins.
- Audit before any runtime run (same generic fault family as forms/shooting/layout):
  1. Neither editor overrides `command_from_action` → every shell action refused as "not framework-reserved".
  2. Jack: `patchNodes`/`deleteSelection`/`setActiveExample`/`setFixtureJson` `BatchOnlyPendingRewrite`; rewriting: all eight document verbs `BatchOnlyPendingRewrite` → hard-dead at dispatch.
  3. Rewriting has no `build_artifact_store_one_item_preparation_factory` (jack has one).
  4. Jack `reset_document_effect` minted a live `ArtifactEnvelope` only to print its spr → the process3d Drop trap on `setActiveExample`.
- Native `cargo check -p semio-s-plugin-trinity --lib` was already red before any edit (committed peer work):
  - jack: `📝️editor` window used `fixture` instead of `snapshot`, `📊️results` the reverse; `TRINITY_JACK_PLAY_CONTROLLER_ID` private but used by the A8 artifact panel `interaction_domain`; both inspection panels used `Label::try_from(&str)` on the plugin `Label` (→ `ui_label`).
  - rewriting: commit 3250e6cb90 (09-15) renamed callers to `Graph::host_snapshot_json` but jack's method stayed `fixture_json` → renamed in jack (+ its unit test).
- Fixes applied:
  - Jack: `args_bridge::command_from_action` (all 11 verbs; camelCase keys, stringified control values, `nodeIds` list/JSON/bare id, nested or flat viewport); `JackRetainedDocumentJobFactory` (`patchNodes`/`deleteSelection` → `Artifact`, `setActiveExample`/`setFixtureJson` → `HostOnly` LoadDocument) with proofs, `build_tool_job` routing and manifest flipped to `Migrated`; `reset_document_effect` now uses `store::empty_document_spr`.
  - Rewriting: `args_bridge::command_from_action` (all 10 verbs; `operations` array → `operations_json`); `RewritingDocumentJobFactory` for the eight document verbs (`resetRule` → `HostOnly`, rest `Artifact`); one-item artifact preparation factory (60 kB mutation bound for `edit-before-fixture`); manifest flipped to `Migrated`.
- Scripts: `📜️check-trinity.sh`, `📜️activate-trinity-{jack,rewriting}-react.sh`, `📜️serve-trinity-{jack,rewriting}-react.sh`, `🐍️trinity-console-dump-probe.mjs`.

### Runtime faults found and fixed (jack, react 6054)
Order as the runtime surfaced them — each was invisible at compile time:
1. **Dispatch dead** — the four `BatchOnlyPendingRewrite` verbs plus the missing `command_from_action` (see above). After the bridge + `JackRetainedDocumentJobFactory`, `patchNodes` renamed a node end to end.
2. **`deleteSelection` on a connected node** failed `batched item candidate failed its exact fixed fold contract`: `delete-node`'s inverse is `create-node` PLUS one `create-edge` per severed edge, which overruns the point-invertible one-item fold contract (`for_one_invertible_item` = 1 forward + 1 inverse row). `delete_selection` now emits one `delete-edge` per incident edge BEFORE its `delete-node`, so every published row is point-invertible; the `delete-node` diff keeps its cascade capture for other callers. An edge-less node (`jack_orphan`) always worked — that is why the first probe looked green.
3. **`setActiveExample` never applied** (`Effect::LoadDocument` → host `loadDocumentArchive` never resolved, then `document archive replacement failed closure, authority, or retained publication validation`): `JackSnapshot.content` is a composed `s.stdio.semio` graph child, but neither surface declared `type Members = SemioMembers` nor `genesis_child_pack`, and the react shell sends `members: []` — the archive closure stayed `Incomplete` (same fault family as energy/process3d). Added on editor + viewer, plus `genesis_jack_child_pack`, the roster in both `ArtifactApps` bounds and in the plugin's `dyn_enum_close!` variants, and `semio-s-artifact-stdio-semio` to the plugin crate.
4. **`reset_document_effect`** minted a live `ArtifactEnvelope` only to print its spr (the process3d Drop trap) → now `store::empty_document_spr`.
5. **Retirement pacing**: jack's owned-value cursor refused any string larger than the 4 KiB grant (the draw stall shape) and walked the 42-kind Nakagin manifest one field per step. Strings now page from the tail; a manifest entry whose bounded size fits the grant is released whole. Retiring one snapshot fell from ~55 s to ~10 s.

### Proof (probes in `🗑️generated/`, zero fault lines unless noted)
- Jack boot (`jack-boot-1`): `data-semio-os-ready=trinity-jack`, graph + Jack Query + Results hosts, no faults.
- Jack `jack-undo-3`/`jack-undo-4`: select `jack_orphan` → `deleteSelection` → tree 9→8; select `jack_prune` (connected) → `deleteSelection` → 8→7, no fold fault; Undo action row → back to 8.
- Jack `jack-example-12`: `patchNodes` renames a node (`probe-renamed` row), then `setActiveExample` (Nakagin) reloads the document — `loadDocument loaded true`, rename gone, tree back to baseline. Takes ~8 min wall clock (see below).
- Rewriting boot (`rewriting-boot-1`): `ready=trinity-rewriting`, LHS/RHS/Jack/Before/After hosts + Parameters, no faults.
- Rewriting `rewriting-clause-set`: `addRuleClause` (Set) appends `SET a.label = ""` to the compiled Jack query; Undo restores it.
- Rewriting `rewriting-interact-2`: `setParameter` from the Parameters window rebinds `label` (`nakagin-core` → `probe-label`) and the compiled query follows.

### Known remaining (not fixed here)
- **Whole-document load is slow**: ~8 min for `setActiveExample`. The framework paces owned-value cursor steps at ~10/s, and every `JackSnapshot` copy carries its own 42-kind Nakagin manifest, so each retired snapshot costs ~10 s and one load retires several. The clean fix is sharing the manifest between snapshot copies (e.g. behind an `Arc`, one refcount drop per clone) — a schema-level change to `JackSnapshot`, deliberately out of this ticket.
- `addRuleClause` with `kind=parameter` dispatches without a fault but no parameter row appears (the `Set`/`where` clauses do land); the other clause kinds are proven.
- Rewriting has no `setActiveExample`, so the shell's boot example dispatch is refused as `undeclared-action` (cosmetic, one warning per boot).
- Probe traps worth keeping: `undo` pops the SHELL's shared input ledger, so any panel toggle between a mutation and the undo is what gets undone (it replays as a refused `shell.panelToggle`); Actions-pane arg controls are `input#<arg>` / `button#<arg>[role=combobox]`, and the execute control is `[id$=".action.<id>.execute"]`.

## 2026-09-18 — native unit-test debt (jack crate)

`📜️test-trinity.sh` (started 09-17 22:43) showed 42 jack failures plus one test that never finished. None were
runtime faults; every one was a test or fixture that had drifted behind framework laws (all traced to peer
commits between 09-09 and 09-16, none to this ticket's edits):

1. **Canonical JSON oracles (8 tests)** — the 16 mutation snapshot fixtures wrote the camera as `0/0/1`
   (integers) against the codec's `0.0/0.0/1.0`, and `JackSnapshot`'s hand-written `ToValue` emitted
   `manifestId: null`/`rootNodeId: null` although the schema declares both as optional strings and the
   tests document "skips when None". Fixtures canonicalised; `ToValue` now skips `None`. Same for the
   graph-window config fixture (`10/20/2` → floats).
2. **Bare stores (10 tests + rewriting bridge)** — `ArtifactStore::new` installs no owner catalog, so
   every `Apply` failed `edit history insertion requires its exact mutation retirement factory`, and the
   store's `Drop` panics without the terminal-empty witness. Added `new_trinity_graph_store` →
   `OwnedTrinityGraphStore` (installs `jack_document_store_owners`, derefs to the store, walks the bounded
   close loop on drop). Tests, the wire-runtime tests and rewriting's `TrinityBridge` use it.
3. **Editor tests (18)** — `dispatch_typed` refused `interactive-job.live-instance` (unbound app), the
   app was never closed, and bare `serde_json::to_string(&tree.root)` fails
   `BuiltChildren requires retained page transport`. New `JackTestApp` harness (bind instance 1,
   `settle` after typed dispatch, self-closing); renders go through
   `artifact_app_laws::project_and_retire_fixture_tree`; scene assertions decode the typed scene with
   `decode_fixture_scene_with_lanes` (the text-editor buffer / lodJson ride out-of-doc lanes). The
   document-tree `"selected":true` assertion was stale by design (rows bind the `"ast"` interaction
   domain; the host paints selection) → asserts the binding instead.
4. **Reorganize tool tests (2)** — `NoMembers` roster refused the composed `s.stdio.semio` child;
   switched to `new_app_with_registry_and_members::<_, SemioMembers>`.
5. **Structural-correspondence test (jack + rewriting)** — the oracle catalog moved to
   `🔮️oracles/🔣️.json` on 09-14; path updated.
6. **`nakagin_document_text_round_trips…`** renamed `node-1` in an EMPTY document; the fixture now carries
   that node.
7. **`jack_store_initializer_cancel…`** dropped a fault `RetainedJobPayload` unclosed (debug assert);
   closed with `JOB_PAYLOAD_PAGE_BYTES` grants (a 4 KiB envelope-page grant releases nothing — spins).
8. **`set_lod_mode_reflects…`** spun on `has_pending_typed_operations` because the completion outbox
   was never drained → `settle`.
9. **`results_window_large_output…`** — `QueryResult.graph_fixture: Option<JackSnapshot>` (~330 inline
   bytes) pushed the results transient over the ephemeral transfer lane's 256-byte inline bound
   (`ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES`) → boxed.

Blocker while re-running: a peer is refactoring `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu` live
(taffy grid helpers, `OverlayKind::Popover`, `GlyphEntry.raster_scale`, Send bounds) so
`semio-framework-ui` is red for minutes at a time; the scratchpad `build-and-test-jack.sh` loop polls
`--no-run` and copies the test binaries out of the target dir before running them.

### Three real runtime bugs surfaced by the test debt (fixed, restaged, re-proven)
- **`runQuery` could never publish a result** — the results-window preflight priced every result at the
  executor's 1 MiB ceiling PLUS the transient header, which overruns the one-item envelope (also 1 MiB), so
  every query output was refused (`Jack results-window transient exceeds its retained publication envelope`).
  The preflight now counts the result's real bytes (`query_result_retained_bytes`; the executor already
  caps them). Proof `jack-verify-query-1`: `runQuery` from the Jack Query window fills the Results window
  with the 9-row `a.name` table (`published: true`, zero faults). Trap: `runQuery` must be dispatched from
  the EDITOR window's Actions pane — from the graph window it is refused
  (`Jack query execution origin must be an attached editor window`), so the probe step runs alone
  (`SEMIO_PROBE_ONLY=query`) with no other pane open.
- **Framework (`🔌️plugin/🦀️.rs`): a live envelope decode could never close** — stage 11
  (`drive_artifact_envelope_decode_worker`) drove the decode jobs with the 4 KiB envelope-page grant, but a
  worker outcome is a `RetainedJobPayload` with 16 KiB pages (`JOB_PAYLOAD_PAGE_BYTES`), whose `close_step`
  releases nothing under a smaller grant → the load spun in its own close forever and the poll never turned
  `Ready`/`Fault` (`jack_live_envelope_submit_pump_swap…` ran >30 min). The decode-job drive now grants
  `max(JOB_PAYLOAD_PAGE_BYTES, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)`; the fault detail page is reported as
  `[DEBUG] live envelope decode faulted: …` before it retires (it was silently dropped). The test finishes
  in 1.9 s.
- **`QueryResult.graph_fixture` boxed** — the inline `JackSnapshot` (~330 B) pushed the results transient
  over the ephemeral transfer lane's 256-byte inline bound.

### Final proof (09-18, restaged wasm for both apps; probes in `🗑️generated/`)
- `📜️check-trinity.sh` exit 0; jack 208/208 and rewriting 153/153 unit tests green (binaries copied out of
  the churning target dir, `RUST_MIN_STACK=32 MiB` — the repo's own runners set it, a bare `cargo test`
  overflows the 2 MiB default in the editor tests; `📜️test-trinity.sh` now sets it).
- Jack `jack-verify-2`: select `jack_orphan` → `deleteSelection` 9→8; select connected `jack_prune` →
  `deleteSelection` → Undo → 8 (restored), zero faults. `jack-verify-query-1`: `runQuery` publishes.
- Rewriting `rewriting-verify-1`: boot, `addRuleClause` (kind=Parameter now DOES add a `param1` row — the
  earlier "no parameter row" note is obsolete), Undo restores, `setParameter` rebinds `label` → `probe-label`
  and the compiled query follows; the only console warning is the known cosmetic boot `setActiveExample`
  undeclared-action.
- Probe pacing: with peers' cargo fleets on the box the tree settles slower than the 12 s default —
  `SEMIO_PROBE_SETTLE=45` for delete/undo.

# 🔧️ Procedural rename fix-forward — 2026-09-15 (lane `procedural-rename-fixforward`)

**Not claimed.** This lane owns no design decision here. It only finished, in the peer's own
direction, the half-landed rename sweep that was holding the procedural guest wasm at the 13:04
build, and then restaged both guests. Every naming choice below was READ off code the peer had
already landed (the framework struct, the framework flow host, and the `🧊️generation3d` artifact,
which compiles) — nothing was renamed backwards and nothing outside the failing compilation units
was touched.

## 1. The peer's naming, as found

Three renames were in flight. The authoritative spellings, read from already-landed peer code:

| Old | New | Read from |
| --- | --- | --- |
| `FlowHostDocument` | `FlowHostSnapshot` | `semio_framework_artifact_flow_flow::FlowHostSnapshot`, used by every `🧊️generation3d` signature |
| `fixture` / `snapshot` (binding + parameter) | `host_snapshot` | `🧊️generation3d/…/🧬️schema/🦀️.rs:403` `dag_host_snapshot_to_workflow(host_snapshot: &DagHostSnapshot)`; `…/🧵️preview-eval/🦀️.rs:165` `may_rearm(host_snapshot: &FlowHostSnapshot)`; `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:271` `let host_snapshot = &document.host_snapshot;` |
| `document_schema` / `document_schema_version` **collapsed into** `artifact_schema` / `artifact_schema_version` | one pair only | `ArtifactInferenceServiceMetadata` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1422` — at `842764d694` the struct carried BOTH pairs, at `8773331d23` only `artifact_schema` survives |
| `TutorialSlice.document` | `TutorialSlice.artifact` | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3013` — fields are now `forward`, `events`, `artifact`, `ui_changes` |

The breakage shape was uniform: the peer's sweep rewrote **declarations** (struct fields, function
parameters) but left **uses** (call arguments, field accesses, local bindings) on the old spelling —
and, where a rename collapsed two fields into one, left a duplicated field in every struct literal.

## 2. Files and lines completed forward

### 2.1 `semio-s-artifact-procedural-assembly`

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`

- **44–45 deleted.** `assembly_inference_metadata()` carried `artifact_schema: "s.assembly"` /
  `artifact_schema_version: 1` **twice** (`error[E0062]: field artifact_schema specified more than
  once`). The second pair is the peer's rewrite of the former `document_schema` / `document_schema_version`
  lines; at `842764d694` they read `document_schema: "s.assembly", document_schema_version: 1`. Both
  pairs carried identical values, so deleting the duplicate loses nothing.

The `cannot find value snapshot` errors reported at this crate's `👁️viewer/🦀️.rs:10` and
`✏️editor/🦀️.rs:16` in the 16:36 restage log were **already gone** when this lane started — the peer
landed them between 16:36 and 16:40. Nothing was touched there.

### 2.2 `semio-s-artifact-procedural-generation2d`

All behind `#[cfg(feature = "component-app-assembly")]`, which is why a plain
`cargo check -p semio-s-artifact-procedural-generation2d` reported **clean** while the component
build failed. The feature-carrying check is `cargo check -p semio-s-plugin-procedural`.

`…/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`

- **230** `with_host(snapshot, |host| {` → `with_host(host_snapshot, |host| {` — inside
  `host_operations(host_snapshot: &FlowHostSnapshot, …)`, whose parameter the peer had already renamed.
- **245** `let nodes: Vec<NodeGraphNodeRecord> = fixture` → `= host_snapshot` — inside
  `dag_host_snapshot_to_workflow(host_snapshot: &DagHostSnapshot)`; matches `🧊️generation3d/…/🧬️schema/🦀️.rs:404`.
- **260** `let edges: Vec<NodeGraphEdgeRecord> = fixture` → `= host_snapshot` — same function.

`…/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`

- **84** docstring `an evaluation of \`fixture\`` → `\`host_snapshot\`` (the docstring named the
  parameter the peer renamed).
- **87** `unserved_flow_operator_kinds(fixture)` → `(host_snapshot)` — inside
  `may_rearm(host_snapshot: &FlowHostSnapshot)`.
- **156** docstring `ONE evaluation hop over \`fixture\`` → `\`host_snapshot\``.
- **160** `flow_host_with_session(fixture, session)` → `(host_snapshot, session)` — inside
  `evaluate_tick(…, host_snapshot: &FlowHostSnapshot, …)`.
- **183** `} else if more && !may_rearm(fixture) {` → `!may_rearm(host_snapshot)` — same function.

`…/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`

- **42** `let fixture = &document.host_snapshot;` → `let host_snapshot = &document.host_snapshot;`
  — the field access was already forward, only the binding lagged. Matches the
  `🧊️generation3d` twin at `…/🕸️flow/🦀️.rs:271`.
- **43** `with_host(snapshot, …)` → `with_host(host_snapshot, …)`.
- **45** `flow_backed_node_graph_extras(fixture, …)` → `(host_snapshot, …)`.

### 2.3 NEW peer error, found during the wgpu restage — `semio-framework-os-renderer-wgpu`

Not in the brief, not in a file this lane had touched, surfaced only by
`activate-generation3d-wgpu-dev` (3× `error[E0609]: no field 'document' on type 'TutorialSlice'`).
Fixed forward the same way and noted here as instructed.

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`

- **12530**, **12567**, **12616** `for entry in &slice.document {` → `&slice.artifact {`.
  `TutorialSlice` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3013`) now declares `artifact`; the
  compiler's own `available fields are: forward, events, artifact, ui_changes` is the peer's direction.

### 2.4 Working-tree footprint of this lane

```
.../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs   |  6 +++---
.../🪆️subsets/✳️any/🧬️schema/🦀️.rs                   |  6 +++---
.../🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs             | 10 +++++-----
.../🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs      |  2 --
.../🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs                  |  6 +++---
```

11 insertions, 13 deletions across 5 files. No `git commit` / `stash` / `checkout` / worktree was
used; no compat layer, no deprecation, no migration shim; no file outside the failing compilation
units was edited; `🗑️generated` was not swept.

## 3. Verification — cargo

| Command | Result |
| --- | --- |
| `RUST_MIN_STACK=33554432 cargo check -p semio-s-artifact-procedural-assembly -p semio-s-artifact-procedural-generation2d --target wasm32-wasip2 --message-format short --keep-going` | **EXIT=0**, warnings emitted for both libs (proof of full expansion, not an aborted type-check) |
| `RUST_MIN_STACK=33554432 cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --message-format short --keep-going` | **EXIT=0** — this is the check that carries `component-app-assembly` and is the one that actually mirrors the component build |
| `RUST_MIN_STACK=33554432 cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --message-format short --keep-going` | **EXIT=0**, 46 warnings |

⚠️ Recorded for the next lane: the two-crate check reported **EXIT=0 while the component build was
still red**, because every generation2d error sat behind `#[cfg(feature = "component-app-assembly")]`.
The plugin-crate check (`-p semio-s-plugin-procedural`) is the authoritative pre-restage gate.

## 4. Restage log summary

`🗑️generated/rename-fixforward/restage.txt` — `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
NX_SKIP_NX_CACHE=true bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`

- attempt 1, **EXIT=0**. `Successfully ran target activate-generation3d-react-dev … and 33 tasks it
  depends on`. `Activated generation3d react dev: 11 completed components (changed)`.
- Run duration 13m 26s, critical path 7m 15s (5 tasks), cache skipped. No retry needed (the 16:36
  session-5 run had burned all six attempts on the rename breakage, exiting 130 each time).

`🗑️generated/rename-fixforward/restage-wgpu.txt` — `…:activate-generation3d-wgpu-dev`

- attempt 1, **EXIT=130** — `Failed tasks: @semio-tech/framework-renderer-wgpu:wasm`, the
  `TutorialSlice.document` errors of §2.3. Fixed forward, then re-run.
- attempt 1 (second launch), **EXIT=0**. `Successfully ran target activate-generation3d-wgpu-dev …
  and 39 tasks it depends on`. `Activated generation3d wgpu dev: 11 completed components (changed)`.
  Run duration 1m 30s. **No wgpu battery was run** — wgpu is on hold per the 13:03 directive.

### Served artifact

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm`

| When | mtime | bytes |
| --- | --- | --- |
| before (stale guest) | Sep 15 **13:04** | 74 462 828 |
| after React restage | Sep 15 **16:57** | 74 482 706 |
| after wgpu restage (same single staging root) | Sep 15 **17:10** | 74 485 425 |

The two previously-unserved guest changes are now IN the served wasm (`strings -a`):
`setSlider` PRESENT, `set_slider_value` PRESENT (slider-preview-update's `nodeGraphEdit setSlider` /
`FlowHost::set_slider_value`), `plugin.command-page-unowned` PRESENT (host-refresh-latency's reactor
terminal rule), `FlowHostSnapshot` PRESENT.

## 5. Boot proof

Port 6027 recycled by pid twice (once after each restage, since the wgpu restage rewrites the same
plugin-module staging root the React serve reads): `kill <pid>` + `screen -S g3dreact6027 -X quit` +
`rm -rf node_modules/.vite-temp` + `screen -dmS g3dreact6027 📜️serve-generation3d-react-6027.sh`.
Final serve started 17:10, pid 32907. No peer process was killed.

`SEMIO_PROBE_URL=http://127.0.0.1:6027/?plugin=generation3d SEMIO_PROBE_OUT=rename-fixforward/boot
SEMIO_PROBE_SECONDS=75 bun 🐍️console-dump-probe.mjs` → `🗑️generated/rename-fixforward/boot/`
(`console.txt`, `hosts.json`, `final.png`, all 17:12).

```
window:procedural-main     meshes=0
  {"height":{"status":"ok"},"radius":{"status":"ok"},"sides":{"status":"ok"},
   "profile":{"status":"ok"},"extrusion-axis":{"status":"ok"},"extrude":{"status":"ok"},
   "column-preview":{"status":"ok"}}
window:procedural-preview  meshes=3
  {"phase":"idle","phaseLabel":{"en":"Idle","de":"Bereit"},
   "progress":{"nodesDone":7,"nodesTotal":7,"ratio":1.0,"inFlight":0,...},
   "cancellable":false,"cancelAction":"toolRunAbort",
   "debug":{"meshesLen":3689,"instancesLen":707}}
```

- **Converged**: `phase: idle`, `nodesDone 7 / nodesTotal 7`, `ratio 1.0`, `inFlight 0`.
- **Meshes**: 3 delivered meshes on the preview surface (3 689 mesh entries, 707 instances).
- **Every hex-column node ok**: all 7 statuses `ok` on `window:procedural-main`.
- **0 page errors** (`pageerror` count in `console.txt` = 0). The 105 console lines are the guest's
  own `[DEBUG]` contribution/publish trace, which the console API reports at `error` level; the
  contributions pack installs with `status: installed`, 16 kinds, 8 examples.

## 6. State left behind

- Both guests staged from the same tree: React and wgpu restages green, staging root at 17:10.
- `:6027` serving the current stage under `screen -dmS g3dreact6027` (pid 32907) — left running for
  the coordinator and for slider-preview-update / host-refresh-latency to take their after-readings.
- Ticket NOT closed, NOT reopened. `🗑️generated` NOT swept.
- Scripts kept in the ticket folder: `📜️restage-rename-fixforward.sh`,
  `📜️restage-wgpu-rename-fixforward.sh`. Generated output under
  `🗑️generated/rename-fixforward/` (`check1–4.txt`, `check-wgpu.txt`, `restage.txt`,
  `restage-wgpu.txt`, `boot/`).
- **Open, not this lane's**: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:150-153` carries the
  SAME duplicated `artifact_schema` / `artifact_schema_version` pair (values `"s.gis.gismap"` and
  `GIS_MAP_SCHEMA` — these DIFFER, so whoever fixes it must decide which survives, unlike the
  procedural case). It is outside the procedural build graph and was deliberately left untouched.

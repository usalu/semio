# Wave P5 — why the Nakagin lane set never becomes instances at the host

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-P5 (continuation of W-P / W-P2 / W-P4 / W-R), 2026-09-10.
Written incrementally while the wave ran. Predecessors: `📓️2026-09-09-wave-P-paged-scene-payload.md`,
`📓️2026-09-10-wave-P4-lane-intake.md`, `📓️2026-09-10-wave-R-intake-delta-cost.md`.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, live tree shared with W-F5 (fill job:
  `⏳️precompute/🦀️.rs`, `🔌️PluginRuntime/🟦️.tsx` job driver). No `git commit` / `stash` / `checkout`
  was run; the ticket is NOT closed; `🗑️generated` untouched.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- Private cargo target `…/scratchpad/target-p3d`, `RUSTC_WRAPPER=""`, `CARGO_INCREMENTAL=0`,
  `RUST_MIN_STACK=134217728`. Every cargo/vitest run backgrounded to `…/scratchpad/wp5-*.txt`.
- `grep -a` throughout (BSD grep calls the emoji-bearing sources BINARY, W-P4 §1).
- The coordinator owns the single browser tab; this wave drove no browser tool and started no server.

## 1 The inherited symptom

Release build #28 (2026-09-10 10:40, clean profile, React target at :6013): switching the example to
**Nakagin Capsule Tower** succeeds at the guest (`setActiveExample` 556 ms, follow-up `refreshUi`
643 ms ok, inspection tree reports `Objects 180`, no `scene-surface.encode` capacity fault), **but both
world windows render only the grid** — no instances — and stay that way across further refreshes.

Concrete Forest, the boot fixture, renders its one object. So the lane mechanism works at small
document size and fails at Nakagin size. That is the discriminator every candidate below is measured
against.

## 2 What the tree already proves (so it cannot be the cause)

Baseline run, `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 nakagin
-- --test-threads=2 --nocapture` → **24 passed / 0 failed** (11.56 s). Its `[DEBUG]` output:

```
[DEBUG] Nakagin world-3d surface presented 26 nodes (cap 128)
        meshes=907B/1leaves instances=55154B/4leaves selection=212B/1leaves vortices=2B/1leaves
        attractions=2B/1leaves targetVolumes=2B/1leaves references=2B/1leaves interaction=256B/1leaves
        lod=125B/1leaves chunking=40B/1leaves environment=92B/1leaves
[DEBUG] nakagin popup-open world scene: spine=1093B of 32768B, 11 lanes carrying 57373B total,
        widest lane=55154B in 4 leaves
[DEBUG] nakagin lane paging: every lane 1 leaf @depth1 except instances=4leaves@depth1
```

So on the GUEST side, at Nakagin scale:

- the spine is 1 093 B against the 32 768 B fixed doc — no capacity fault;
- 11 lane carriers hang off the surface node, 57 373 payload bytes total;
- the whole world surface is 26 nodes against `UI_DOCUMENT_NODES` = 128;
- the `instances` lane is 55 154 B in **4 packed leaves** (`UI_TEXT_MAX_BYTES` 512 in `value` plus 32
  `data_attributes` slices, 16 896 B per leaf) — so Nakagin is the first document that exercises the
  PACKED leaf at all: Concrete Forest's lanes each fit one unpacked 512-byte leaf.

Two host-side links were probed directly and are also clean:

- packed leaf across the retained wire: `encodePackValue` of a text record with `value` 512 B + 32
  `dataAttributes` of 512 B, decoded by `RetainedUiTypedCursor` → `ready`, `value bytes 512`,
  `attrs 32` (scratch probe, 3 008 steps). No slice is dropped.
- the Interpreter's own `🚚️surface-scene-lanes` suite already reassembles a packed multi-lane
  Nakagin-scale carrier out of a `UiDocumentStore`.

## 3 The measurement that settles it: the world DOES render Nakagin

Three read-only headless probes were driven against the already-running React dev target at
`http://127.0.0.1:6013` (repo `node_modules/playwright`, a separate browser process — the coordinator's
tab was not touched and no server was started). Consolidated as
`🔍️wp5-world-lane-probe.ts` in this ticket folder; the throwaway variants and their logs are
`…/scratchpad/wp5-live-probe*.{ts,txt}`.

`World3dHost` publishes the scene it actually consumes onto its own root element
(`🌐️World3dHost/🟦️.tsx:5422-5424`: `data-meshes-json`, `data-instances-json`, `data-status-json`), so
what the ASSEMBLED scene contains is directly readable from the DOM — no instrumentation needed.

| moment | `data-instances-json` (both windows) | `data-meshes-json` |
| --- | --- | --- |
| boot, Concrete Forest | 266 B, 1 instance, `meshId "mesh:🧊️hexagonal-cut-concrete-forest-left"` | 181 B, 3 records |
| +5 s / +10 s after the Nakagin click | still 266 B | still 181 B |
| +15…30 s after the click | **54 254 B, 180 instances** | 811 B, 14 records |

Probe 3 additionally parsed the assembled lane in the page:

```
nakagin count=180 bounds min=[-23.45,-12.55,0] max=[0,0,39.5833]
        unknownMeshIds=[] meshCount=14
        sample={"id":"01890804-…","meshId":"mesh:🧊️capsule_J","position":[-20.75,-3.65,19.6167],…}
```

`unknownMeshIds` empty — every one of the 180 instances names a mesh the same publication declares.

And the screenshot taken at the end of probe 3 (`…/scratchpad/wp5-nakagin3.png`) shows the **Nakagin
Capsule Tower fully drawn in both windows** — base, tambours, 140-odd capsules, two capitals — in the
Perspective viewport and in the Top viewport.

**So there is no lane-to-instance defect on the tree as it stands.** The 180-object lane set does
become instances at the host, on the wasm module currently served
(`💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/…core.wasm`, 2026-09-09 19:21) plus the live TS the
vite dev server compiles from source — i.e. including W-P4's step ceiling and W-R's delta-priced
validation, which reach the browser through HMR / a page reload without any rebuild.

### 3.1 What the coordinator actually saw

The scene arrives **15–35 s after the click**, on every one of the three probe runs, while
`setActiveExample` (556 ms) and the following `refreshUi` (643 ms) both report ok within the first
1.2 s. The observation "the world renders nothing, and this stays so after further refreshes" was
therefore made inside that window: the guest had acknowledged, the panels had already repainted
(`Objects 180` — the inspection panel is a reserved SECTION, a few hundred bytes, so it settles in the
first second), and the world body — 57 KB across 11 carriers, the single largest publication in the app
— had not yet been taken in. Nothing about it is stuck; it is slow.

That latency is the real residual defect and it is NOT in the lane split, the carrier walk or the
assembly: all three are microseconds-to-milliseconds in the laws below. It is the cost of taking one
57 KB / 26-node retained surface publication in, twice (one per window), on a machine at load average
~20. W-R's own measurement predicts ≈ 671 000 intake steps for a first publication of a document that
size (≈ 2.3 s of renderer CPU each) plus 656 macrotask round-trips at the current
`PLUGIN_UI_INTAKE_YIELD_STRIDE`; the remainder is guest render + reconcile + `projectOwnedUiSurface`'s
per-node maintenance drain. Attributing and reducing it is a performance wave, not this one — see §7.

## 4 Why every candidate in the brief is ruled out

| candidate | verdict | evidence |
| --- | --- | --- |
| a lane exceeds a page count | no | every lane 1 leaf @ depth 1 except `instances` (4 leaves @ depth 1); `childrenMax` 32 never reached |
| `instances` pages arrive out of order | no | the walk is depth-first over `children`; assembled text hashes equal to the declared manifest (native law §5.2) |
| a carrier key the consumer does not expect | no | `world3dSceneLaneForBodyKey` is pinned against the same fixture as the Rust table; probe 3 reports 14 meshes and 0 unknown mesh ids |
| a lane hash mismatch rejects the set | no | the browser assembles all 11 lanes |
| `World3dHost` reads a pre-lane single-blob field | no | it reads `scene.instancesJson` off the ASSEMBLED scene (`🌐️World3dHost/🟦️.tsx:4128`), which is what `PagedSurfaceView` hands it |
| `setActiveExample` scope `none` skips the scene republish | no | `puzzle3d_command_scope_class("setActiveExample")` → `Puzzle3dScopeClass::Chrome` → `UiDirtyScope::Full` (`✏️editor/🦀️.rs:2185,2204`), and the existing law `set_active_example_lands_as_one_edit_and_republishes_the_world_scene` asserts exactly that emit scope |
| the packed 33-slice leaf loses its `dataAttributes` across the wire | no | scratch probe: `encodePackValue` → `RetainedUiTypedCursor` returns `value` 512 B + 32 attributes |

## 5 What this wave added — the coverage that was missing

Nakagin is the first document in this app that **packs** a text leaf at all: `section_text_chunks`
(`💻️os/🔨️modules/🔌️plugin/🦀️.rs:391`) puts slice 0 in `TextProps.value` and slices 1…32 in
`data_attributes`, 16 896 B per leaf, and Concrete Forest's lanes each fit one unpacked 512-byte leaf.
Neither existing suite crossed the packed leaf on the production path:

- `🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes` mints `UiNodeRecord`s straight into a
  `UiDocumentStore` — it never crosses `builtNodeToSnapshot`, the projection `ShellHost` actually runs
  on a refresh-response body;
- `📃️UiDocumentStore/🧪️tests/🧪️typedwire`'s Nakagin intake law (W-P4/W-R) uses UNPACKED 512-byte
  leaves and stops at the retained surface — it never assembles a scene.

### 5.1 TS law

`🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx`, new:
**`projects a Nakagin-scale packed window body through builtNodeToSnapshot into 180 assembled
instances`**. Builds the exact `paged_text_carrier` tree as a `BuiltNode` — `packChunks` slices per
text leaf, `childrenMax`-ary pages, one carrier per lane — with a real 180-record instances payload
(≈ 54 KB, the browser-measured shape), runs the shell's own `builtNodeToSnapshot`, loads a real
`UiDocumentStore`, then walks it with the Interpreter's own `world3dSurfaceLaneTexts` /
`world3dSceneFromLanes` and asserts: every lane settles against its declared byte count, the assembled
`instancesJson` parses to **180** records, **no** record names a mesh absent from the same
publication's `meshesJson`, and the instances lane occupies exactly
`ceil(ceil(bytes / leafBytes) / packChunks)` packed leaves.

Fails before / passes after — measured, not asserted: with `packedTextLeaf`'s attribute loop disabled
(the exact 512-byte-truncation regression of ticket 26/09/09/PROCEDURAL-3D-END-TO-END) the law fails;
restored, it passes. Runs in §6.

### 5.2 Native law

`✏️editor/🧪️tests/🔬️example-switch/🦀️.rs`, new:
**`the_nakagin_switch_assembles_every_object_onto_a_mesh_the_same_publication_declares`**. Drives the
real typed `setActiveExample` to Nakagin and asserts on the REASSEMBLED scene (spine + every lane
merged, which is what a render host reads) that it carries one instance per Nakagin object (≥ 180),
that every `meshId` resolves inside the same publication's `meshesJson`, that every declared mesh
resolves by `kind` or by `url`, and that the spine's own FNV-1a manifest entry for the instances lane
re-derives from the assembled text byte-for-byte.

The dangling-`meshId` clause is the one shape that passes every byte-level lane law and still paints an
empty world: `WorldInstancesLayer` looks the id up in `meshById`, finds nothing, and renders neither a
GLB (no `meshRecord.url`) nor the fallback box (no `meshData`).

### 5.3 Wire law

`📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx`, new:
**`OwnedIntake publishes a packed scene-lane leaf whose data-attribute slices all survive the wire`**.
The W-P4/W-R Nakagin intake law next to it builds UNPACKED `leafBytes` leaves, so nothing in the tree
drove a 33-slice leaf through a real `ShardClient` UI-patch authority. This one does: it packs a
32 147-byte lane payload the way `section_text_chunks` does, drives `OwnedUiPatchIntake` to
acknowledgement, reads the published records back the way `projectOwnedUiSurface` does
(`subscribeNode` → `snapshot.record` → `acknowledgeRead`, draining `advanceMaintenance` around each
read) and reassembles them with the renderer's own `packedTextLeaf`, asserting the result is the
original payload byte-for-byte and parses to 512 records.

Fails before / passes after — measured: with `RetainedUiTypedCursor`'s `dataAttributes` decode replaced
by `null` (`🧵️retained/📦️wire/🧾️typed/🟦️.ts:334`) the law fails on the reassembly; restored, it passes.
Both temporary edits were reverted and diffed byte-identical against their backups.

## 6 Verification

All runs backgrounded to `…/scratchpad/wp5-*.txt`. Rust envelope:
`RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 RUST_MIN_STACK=134217728
CARGO_TARGET_DIR=…/scratchpad/target-p3d`, `-j 4`, `--test-threads=2`.

| command | result |
| --- | --- |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 nakagin` (baseline, before any edit) | **24 passed / 0 failed** (11.56 s) |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 example_switch` | **5 passed / 0 failed** (1.45 s) — includes the new §5.2 law |
| `cargo test -p semio-framework-ui-scene -j 4` | **116 passed / 0 failed** (0.20 s) |
| `bun ./📜️script.ts test long --run '🗣️Interpreter' '📥️intake' '🔬️engine-contract'` | **3 files, 569 passed / 0 failed** (17.7 s); re-run at the end of the wave **3 files, 570 passed / 0 failed** (the extra law is a peer's, landed mid-wave) |
| `bun ./📜️script.ts test long --run '🗣️Interpreter' --testNamePattern='Nakagin-scale packed window body'` | **1 passed / 74 skipped** |
| — the same law with `packedTextLeaf`'s attribute loop disabled | **1 failed** (`JSON Parse error: Unexpected EOF` — the 512-byte-prefix signature) |
| `bun ./📜️script.ts test exhaustive --run UiDocumentStore --testNamePattern='packed scene-lane leaf'` | **1 passed / 213 skipped** |
| `… --testNamePattern='packed scene-lane leaf\|Nakagin-scale paged scene-lane'` (this wave's intake law beside W-R's) | **2 passed / 212 skipped / 0 failed** |
| — the same law with the typed cursor's `dataAttributes` decode nulled | **1 failed** on the byte-exact reassembly |
| `bun ./📜️script.ts typecheck` | **824 `error TS`**, **zero** in `🚚️surface-scene-lanes`, `🧪️typedwire` or any other file this wave touched (the peer baseline moved 820 → 824 during the wave; a first run showed 827 = 824 + three implicit-`any` diagnostics of this wave's own, since annotated) |

The native law's own trace, byte-for-byte the same shape the browser reports:

```
[DEBUG] Nakagin switch assembled 180 instances over 14 declared meshes (0 unresolved)
        from a 55154-byte instances lane in 4 leaves
```

## 7 Not verified / open

- **The 15–35 s repaint latency is not attributed.** It is the one real residual and it is a
  performance question, not a correctness one: the lane split, the carrier walk and the assembly are
  all sub-millisecond in the laws above, and the guest acknowledges in ~1.2 s. Candidates, in the order
  the measurements point at them: the first-publication intake cost W-R measured (≈ 671 000 steps ≈
  2.3 s of renderer CPU per surface, twice — one per window), `projectOwnedUiSurface`'s per-node
  `advanceUiMaintenance` drain, and the guest's own composite render of a 180-object fixture. Naming
  the dominant term needs a timed instrument inside `acceptUiPatches`; this wave did not add one.
- **The served wasm does NOT need a rebuild for this.** The probes ran against the module already
  served (`…/🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/semio_s_plugin_puzzle_component.core.wasm`,
  2026-09-09 19:21) and it publishes the packed lanes correctly; the two new laws touch only test
  files, and the one Rust file this wave edited is a test module, so nothing shipped in the guest
  changed. (A rebuild is still owed to W-F5, whose fill-job Rust did change — that is their wave's
  call, not this one's.)
- **The Top window was not driven interactively.** Both windows assemble and paint the same 180-object
  scene (probe 3's screenshot), but pick / hover / context-menu on the Nakagin document were not
  exercised by this wave.
- **The SWITCH BACK was not re-driven.** The 10:50 evidence also reports Nakagin → Concrete Forest
  leaving the world empty at `Objects 1`. The round-trip probe
  (`🔍️wp5-world-lane-probe.ts --round-trip`) was written and launched, but from ~10:36 the dev target
  at `:6013` stopped answering: the socket still LISTENs (`bun` pid 44038) while `curl` and Playwright's
  `goto` both time out (30 s and 180 s). Two runs were lost that way. So the reverse direction rests on
  the same reasoning as the forward one rather than on a measurement: the lane cache keys on
  `${surfaceNodeId}:${lane}` (and `builtNodeToSnapshot` mints the window-body root as id 1 every
  reload, so that key is stable), a changed hash forces a re-walk, and a carrier still holding the
  previous fixture's tail is OMITTED until its concatenation matches the newly declared byte count —
  correct, and transient. Re-run the probe against a responsive target to close it.
- **`WorldAutoFit` never mounts for this app.** `scene.fitJson` is never published by
  `🪟️windows/🧊️main/🦀️.rs`, so the camera after a fixture switch is whatever the previous document
  left in `cameraJson`. Nakagin happens to sit inside Concrete Forest's framing (bounds
  x[-23.45, 0] y[-12.55, 0] z[0, 39.58]) so it is visible; a fixture centred elsewhere would not be.
  Flagged, not changed — publishing a `fit` lane on a document swap is an editor decision.
- **The dev target at `:6013` is wedged as this wave ends** (from ~10:36 CEST). `bun` pid 44038 still
  holds the LISTEN socket, but a plain `curl --max-time 25 http://127.0.0.1:6013/` returns `000` on
  every attempt, as did Playwright's `goto` at a 180 s budget. The three successful probes ran before
  that. This wave's own headless chromium leftovers were reaped (`kill 5687`) and the server stayed
  wedged afterwards, so they are not the cause; it is the coordinator's server to restart, not this
  wave's.

## 8 Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx` — §5.1 law (+80 lines, additive)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx` — §5.3 law (+49 lines, additive)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` — §5.2 law (+32 lines, additive)
- `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️wp5-world-lane-probe.ts` — the read-only browser probe, kept
- `.🧬semio/…/PUZZLE-3D-END-TO-END/📋️master-plan-2026-09-08.md` — W-P5 section appended
- this report

No production file was edited. `🔌️PluginRuntime/packed-text.ts` and
`🧵️retained/📦️wire/🧾️typed/🟦️.ts` were each broken once for a negative run and restored; both were
diffed byte-identical against their backups afterwards. `🗑️generated` was not touched.



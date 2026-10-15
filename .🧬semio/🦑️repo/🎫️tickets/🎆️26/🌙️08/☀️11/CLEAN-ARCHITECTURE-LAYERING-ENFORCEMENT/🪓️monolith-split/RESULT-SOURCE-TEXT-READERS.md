# 📖️ Result: Source-Text Readers After the Monolith Splits

Executor: Sonnet 5.5. Paths: `M/` = `🧰️framework/🛍️products/💻️os/🔨️modules/`, `E/` = `M/📺️renderer/🧑‍🎨engine/`, `U/` = `🧰️framework/🔨️modules/🖱️ui/`, `H/` = `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/`. Logs: `🗑️generated/str-*.log` in this folder. Line numbers are those of the working tree at the end of the run.

## 0. Status

One helper, one policy registry, and about 45 readers re-pointed. TS readers were run with their real runner (`bun test`, vitest where the suite lives in the react config) before and after. Root-script and hub-script gates cannot run end to end (details in section 5), so they were checked in isolation. Nothing Rust was compiled (cargo is blocked by the source-boundary gate); the Rust edits were parsed with `rustfmt` and the `concat!(include_str!())` form was proven with `rustc`.

## 1. The mechanism

### 1.1 Canonical helper `H/🟦️.ts`

No usable helper existed (`readRustSourceClosureV1` is cfg-selected, async, digest and boundary oriented and throws on generated inputs; `policyReadRustSourceEvidence` followed only the parent's own test mounts). New module, sibling of `🦀️rust/📁️paths`, built on the repo's own tokenizers (`rustTokens`, `rustAttributes`, `rustPathAttributes`, `ecmaTokens`), so comments, strings, macro bodies and function bodies are not mistaken for mounts.

| Export | Meaning |
|---|---|
| `inspectRustPathMounts(source)` | every `#[path = "…"] mod x;` (any `pub`/`pub(crate)`/`pub(super)`, any attribute order, `#[cfg(...)]` ignored), with the enclosing inline module names |
| `inspectEcmaRelativeImports(source)` | static `import … from`, `export … from`, side-effect `import "…"` with `./` or `../` specifiers |
| `listModuleTree(root, opts)` | root first, children in declaration order, depth first, each file once. `.rs`: `#[path]` resolved from the declaring file's directory plus inline module names. `.ts`/`.tsx`: relative imports that resolve to a `.ts`/`.tsx` file or a directory with `🟦️.tsx`/`🟦️.ts` |
| `readModuleTreeSource(root, opts)` | the files joined, each ended by a newline. This is the text the unsplit file held |
| `sliceModuleTreeSource(root, from, to?, opts)` | replacement for `text.slice(text.indexOf(from), text.indexOf(to))`: inside the first file holding `from`, up to the first `to` after it, else to the end of that file |
| options | `tests: true` also follows `🧪️tests` mounts (default: excluded, as the old text never contained them); `read` injects the file reader |

Rules baked in: only mounts below the declaring file's directory are children (so `../../…🛰️Dock/…` cross-tree mounts and other crates are never pulled in); a declared mount whose file is missing throws; an ambiguous `#[path]` throws; roots other than `.rs/.ts/.tsx` throw. `root` may be a path or a `URL`.

Why `sliceModuleTreeSource` exists: plain concatenation changes the meaning of every "from marker A to marker B" slice whose markers now sit in different files in the opposite order. Two such tests failed for exactly that reason (`♻️physical-job-close`, `🖌️wgpu-document-owner-move`); they now slice inside one file.

### 1.2 Policy layer (repo scripts)

- `M/../🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` gained `POLICY_SPLIT_SOURCE_ROOTS` (renderer, Scenes, Shell wgpu, infinite world/normal/dag, plugin instance, plugin host, manifest, hub bootstrap, ShellHelpers, World3dHost, ShellHost), `policyModuleTreeFiles` and `policyReadModuleTreeSource` (the helper through the symlink-refusing `policyReadFileSafe`).
- Root `📜️script.ts`: `policyReadRustSourceEvidence` reads a registered root as one text and discovers canonical test modules in every file of its tree; `policyReadRustPolicySource` does the same for the registered `.tsx` roots. This is registry driven on purpose: tree-reading every Rust source would pull in about 144 files for `INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE` and 26 for the reactor, whose mounted modules are audited on their own.

### 1.3 Tests

- `H/🧪️tests/🟦️.ts` + `H/🧪️tests/🧫️fixtures/🔣️.json` (language-neutral vectors: mount vectors, three on-disk trees, slice vectors, refusals). 7 tests, 54 expects, run `bun test` result: 7 pass 0 fail. Independent oracles: the installed `tree-sitter-rust` grammar for the mounts; genuine `rustc --emit=dep-info` for the file set of the Rust tree (it agrees, including the inline-module base directory and the `🧪️tests` mounts); the TypeScript parser (`ts.createSourceFile`) for the relative imports; strict `tsc` program for the module's own types.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence`: new vectors for a split tree (children text, child test module found, unregistered parent stays per-file, mutation stays unambiguous) and a guard that every registered root exists and reads as one text. 6 pass 0 fail (was 4 pass before).
- Taxonomy: `bun ./📜️script.ts verify taxonomy report --scope …/🌳️module-tree` reports one error `directory-kind-unresolved`. The sibling `🦀️rust/📁️paths` reports the identical error, so this is the existing gap for facet folders, not new.
- Not done: a Rust twin of the helper (Rust readers use `concat!`, see section 4).

## 2. Readers re-pointed (TS and scripts)

Result column: pass/fail counts are `before → after` for the file's own runner; "same" means the same set of failing tests. Pre-existing failures are named.

### Renderer (`E/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`)

| Reader (file:line) | Old read | New read | Result |
|---|---|---|---|
| `E/🧪️tests/🪟️window-lifecycle-template-drag/🟦️.ts:166` | `readFileSync` parent | `readModuleTreeSource` | 8/1 → 9/0 (together with ShellHost lines 96, 155) |
| `E/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts:552` | parent | tree | 16/3 → 16/3 same; the 3 failures read `🌐️browser-worker` and the frame-worker, not the renderer |
| `E/🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts:16-18` | `?raw` of renderer, Shell, one Scenes child | tree of renderer, Shell, Scenes parent | bun 4/1 → 4/1 same, vitest 4/1; remaining failure is a `winit-app` needle |
| `E/🧪️tests/🧾️frame-action-ledger/🟦️.ts:50` | parent | tree | not verifiable: the file dies earlier on a missing fixture (`🧫️fixtures/🧾️frame-action-ledger/🔣️.json` ENOENT) before and after |
| `E/🧪️tests/🧲️scene-input-residency/🟦️.ts:22,48,49` | world parent, renderer, `World3dHost/🟦️.tsx` | tree | 1/2 → 3/0 |
| `E/🧪️tests/♻️frame-close/🟦️.ts:10,77,…` (new `readTree`) | renderer ×3, Scenes parent, a Scenes child, `🛂️manifest/🦀️.rs` | tree | 8/6 → 12/2; remaining: `PreparedAtlasAuthority` now lives in a separate crate (`🎟️prepared/📤️staging`, a peer's carve, not a `#[path]` mount) and a missing `U/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs` |
| `E/🧪️tests/🖼️wgpu-raster-residency/🟦️.ts:180` | renderer | tree | 4/1 → 4/1 same; the failing needle is in `🎟️prepared` (same peer carve) |
| `E/🧪️tests/📨️browser-frame-transport/🟦️.ts:971` | renderer | tree | 46/16 → 46/16 same (fixture ENOENT, browser-worker); vitest 58 pass 7 fail |
| `E/🧪️tests/⏱️frame-latency/🟦️.ts:104,107` | renderer, Shell | tree | 2/1 → 2/1; the renderer assertions now pass, the failure moved on to `winit-app` (`FrameLatencyStage::SnapshotPublish`) |
| `E/🧪️tests/🫀️settle-pump/🟦️.ts:174` | renderer + Shell via `gestureSourcePaths` | tree | not verifiable: fixture ENOENT before and after |
| `U/🧪️tests/♻️physical-job-close/🟦️.ts:59,60,73,75,81` | renderer ×3, three `slice(indexOf, indexOf)` | tree and `sliceModuleTreeSource` (also `to` marker became `pub(crate) struct FrameDeferredExecutionEnvelope`, because the split widened its visibility) | 6/3 → 9/0 |
| `U/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🟦️.ts:27` | 4 rows incl. renderer, Shell | tree before `parser.parse` | 4/1 → 5/0 |
| `E/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts:26` and `U/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts` | `policyReadRustPolicySource` (+ a Scenes child) | registry makes it tree-aware; Scenes child → Scenes parent | self-test functions, 0 tests of their own; see section 5 |
| `E/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts:257,317,451` | Shell ×2, renderer color lint | tree; the lint now walks every file and reports `<file>:<line>` | module imports; not executed |
| fixture-keyed: `E/🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts:37,50,64,134`, `🖼️wgpu-raster-witness:45`, `🌳️wgpu-document-reconcile:71` (`source(key)`) | single path from the fixture | tree (the React Interpreter key stays plain), three slices via `sliceModuleTreeSource` | owner-move 3/8 → 7/4; raster-witness 3/8 → 4/7; reconcile cannot load under bun (vite `?worker&url`), unchanged. Remaining failures are pre-existing: type `RendererAssetDecodeInvocation` exists nowhere, the session builds its own `root_cancel_token`, missing fixture paths |

### Scenes, Shell wgpu

| Reader | Old | New | Result |
|---|---|---|---|
| `E/🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🟦️.ts:62,67` | Shell and Scenes parents | tree | 4 pass 1 fail; the failure needs `SEMIO_TEST_ARTIFACT_DIR`, the edited test passes |
| `E/🧪️tests/♻️frame-close/🟦️.ts` (Scenes) | child `📷️scene-camera-dispatch` | Scenes parent tree | included in the frame-close row |
| `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts:30,33` | world parent; Scenes child `🖌️canvas2d` | tree of world and Scenes parent | imports resolve; entry refuses without a parent invocation, not executed |
| Root `📜️script.ts:5517` (Shell), `6264`/`8153` (plugin host), `6267`/`8150` (renderer), `6274` (manifest; replaces a peer's seven-file list), `8095`/`8109` (renderer), `8114` (dag), `9286` (registry-conditional), `9306` (Scenes; was a child), `8951`/`9248` (peer's three-file list for canvas raster restored to the single parent so children are not read twice) | `policyReadFileSafe` | `policyReadModuleTreeSource` / registry | see section 5 |
| Shell: `E/🧪️tests/🔬️interactivity…` constants `INTERACTIVITY_AUDIT_SHELL_FILE`, `…_WORLD3D_FILE`, `…_RENDERER_GLUE_FILE`, `…_HUB_BIN_FILE` | `policyReadRustPolicySource` | registry | see section 5 |

### Infinite

| Reader | Old | New | Result |
|---|---|---|---|
| `E/🧪️tests/🫳️node-graph-gestures/🟦️.ts:80,153,206` | dag parent + two dag children | dag parent tree | 9/3 → 11/1; remaining failure reads the flow host (`Self::content_changed`), not split |
| `E/🧪️tests/🧪️node-graph-edit-rows/🟦️.ts:62`, `🔗️node-graph-wire-edit:119` | dag parent | tree | edit-rows cannot load under bun, **4/4 pass under vitest**; wire-edit 10/0 → 10/0 |
| `M/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:24` | `➕️normal/🦀️.rs` | tree | 0/1 → 0/1, but the board assertion (needle in `🧹️scene-delete-links`) now passes; it dies on a later `⚙️EngineCanvas` needle |
| `E/🧪️tests/🧲️scene-input-residency` | see renderer | | |

### Plugin instance / host

| Reader | Old | New | Result |
|---|---|---|---|
| `M/🔌️plugin/⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🟦️.ts:24,120` | instance parent | tree | the report's regression is gone: 10/3 → 12/1 (baseline 12/1; the remaining failure is the baseline one) |
| `M/🔌️plugin/⚛️reactor/🚪️lifetime/🏗️opening/🧪️tests/🟦️.ts` | instance parent | tree | 0/1 → 1/0 |
| `M/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts:62` | instance parent | tree | HEAD copy 0/1 → 8/0 |
| `E/🎯️targets/🧊️wgpu/🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts:36` | instance parent | tree | HEAD copy 5/1 → 6/0 |

### Hub bootstrap

`🌎️hub/📦️packages/🦀️rust/📜️script.ts`: 19 reads of `🌎️hub/🏗️bootstrap/🦀️.rs` (lines 3013, 3281, 3426, 3471, 4145, 5469, 6852, 7150, 7333, 8538, 13406, 13895, 15414, 15979, 16166, 16328, 16394, 16675, 17023; the report listed 16, the sweep found three more that join the path segment-wise, 6852/7150/7333) now use `readModuleTreeSource`, plus the ShellHelpers read at 16204 and two ShellHost reads (14268, 16203). `🌎️hub/🧪️tests/📊️observability/🟦️.ts:142` (route census) 4/3 → 5/2 (the 2 left are schema-catalog failures).

### ShellHelpers, World3dHost (TSX trees)

| Reader | Result |
|---|---|
| `E/🧪️tests/🔬️engine-contract/🟦️.ts:10313,10328` (World3dHost `?raw` and a `readFileSync` with retired-string absence checks) | vitest, both tests pass (`-t` filter). Under vitest `import.meta.url` is not a file URL there, so the first site finds the repo root the way the second one did |
| `E/🔮️oracles/🎨️world3d-scene-shading/📜️script.ts:1798` | the AST lookup of `surfaceTuple` + `worldSurfaceMaterial` over the tree finds both declarations (12 files); the GPU oracle itself not run |
| `hub script:16204` (`PRESENCE_HEARTBEAT_INTERVAL_MS` in ShellHelpers) | needle check only |
| root `📜️script.ts` puzzle-fill role `renderer: World3dHost/🟦️.tsx` | registry (`policyReadRustPolicySource` returns the tree for registered `.tsx` roots) |

### Manifest

Only the root `📜️script.ts:6274` reader (above) and `frame-close`'s `ViewModel` field-order read; `🕸️graph/🛂️manifest/…` is another directory.

### Bonus: ShellHost (a peer is splitting it right now)

`E/🏛️ShellHost/🟦️.tsx` went from 13,692 to 590 lines during this run and broke `window-lifecycle-template-drag` (9/0 → 7/2). The reads were re-pointed: `E/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx` ×3 and `📐️geometry`, `E/🧪️tests/🪟️window-lifecycle-template-drag:96,155`, `♻️hot-swap-session-handover:90`, `🔗️hub-projection:60,77`, `🚗️driver-editor:64`, `🪟️spawned-program-session:59`, `⏱️wgpu-ui-turn-budget:112`, `🔬️engine-contract:11334` (a marker slice, now `sliceModuleTreeSource`), `M/📦️packages/🟦️typescript/📜️script.ts:158`, the gismap script (3 reads), hub script (2), and the registry. Verified: window-lifecycle 9/0, wgpu-ui-turn-budget 8/0, hot-swap 7/1 (the failure is a NodeGraph needle), `hub-projection` and `driver-editor` do not load under bun (vite-only imports).

## 3. Verification detail for scripts

- Hub gates, real function bodies: a temporary copy of the hub script with the `prove*` functions exported (removed again), run with the HEAD script and with the new one against the current tree. `proveDocumentOpenPlanFixture` FAIL → PASS and `proveExecutionTargetLeaseSource` FAIL → PASS. The other 12 stop earlier in both versions on an unrelated repo fault (`hub schema module … is absent from …schema-catalog`, one missing fixture), so they say nothing about the reads.
- Hub bootstrap markers: `hubsplit-tools/script-ts-readers.txt` lists 33 markers; all 33 are found in the tree, 15 of them only in children (a parent-only read would fail).
- Root: `POLICY_SPLIT_SOURCE_ROOTS` all exist and read; `policyReadRustSourceEvidence` works for Scenes (14 canonical tests), Shell (55), world (7), hub (3), normal, instance, ShellHelpers, World3dHost. It throws "not canonical" for renderer (R7 child `🪶️sqlite/🧪️tests/🦀️.rs`), plugin host (`🧪️tests/🪶️sqlite/⚠️refusal/…`), manifest (`🔖️action-args/♻️retirement/🧪️tests/…`): these three declarations were in the unsplit parents at HEAD and threw there too. dag throws on the pre-existing child `🎛️input/🧪️tests/🦀️.rs`, a newly reachable consequence of reading dag as a tree (no root gate calls evidence on dag today).
- `bun ./📜️script.ts verify interactivity p3mn|p5e|p5d` fail with those same non-canonical test mounts (renderer; a plugin path for p5e) and did at HEAD, so they cannot confirm anything.

## 4. Rust readers

- Scenes: `SCENES_SOURCE` (`#[cfg(test)] pub(crate) const`, `concat!(include_str!())` of the 16 files, list generated with `listModuleTree`) added to `E/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`; `🧪️tests/🧊️wgpu-standalone/🦀️.rs:61` uses it; `🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs:82` now includes `🗃️scene-state/🦀️.rs`, the file that holds both `AdmittedSurfaceEntry` and its `mod admitted_surface_map_tests` marker.
- infinite world unit test (`M/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`): new `RENDERER_SOURCE` (17-file `concat!`) replaces the two renderer `include_str!`s (lines 274, 1944); the six `(start, end)` regions the test slices all live in one file each (`🧾️frame-build`, `🖥️frame-presentation`), checked.
- Verified: `rustfmt` parses all touched files; a `rustc` toy proves `concat!(include_str!())` with emoji paths; the renderer list compiles standalone with absolute paths (1,057,336 bytes). Nothing was compiled in cargo.

## 5. Not fixed, and why

1. Shell Rust readers (peer is splitting Shell now; a hard-coded `concat!` list would be stale at once): `E/🧱️elements/🐚️Shell/🧪️tests/{🎓️tour-overlay-and-chord-glyphs (4 reads), 💓️chrome-maintenance-pressure, 🗄️browser-prefs-persistence, 🚪️wgpu-host-door-remainder (2), 📤️wgpu-file-open-import, ⬇️wgpu-media-export-encoding, 🔄️sync-owner-lifetime, 🔗️hub-projection-workspace, 🔬️wgpu-shell-chrome-parity}`, `E/🧪️tests/{📇️wgpu-directory-door, 🫀️settle-pump}/🦀️.rs`, and `world/🧪️tests/🔬️unit/🦀️.rs:1945` (`shell`). Recipe: a `SHELL_SOURCE` `concat!` const once the child list is final (generate it with `listModuleTree`).
2. Thirteen plugin TS readers keep the explicit file lists the plugin agent wrote (`♻️cancellation/📦️lease`, `♻️lifecycle/🏪️stores`, `🎞️media/⚠️turn`, `🎮️command/🎟️lease`, `👷️job/🚪️session/{🎟️pending,🫴️refusal}`, `🧩️composition/…/📦️owner`, `🧪️tests/🧾️document-archive-load-legs`, `🧵️retained-command/🪟️mounted/♻️frontier`, host `⏱️deadline`, `♻️publication`, `🎟️context/🤝️admission`, `🔔️wake`). They work (identical pass/fail before and after, per that report) and mix in `🧵️retained-command/🔄️full-operation`, which is not a child, so a mechanical swap would change their text; they go stale on the next split.
3. `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests` (line 874) and `…/💡️services/🧪️tests/🤖️live-agent-loop` (line 79) read ShellHost: `✏️s` tests may not import `🧰️framework` (rule `no-cross-technology-✏️s-to-🧰️framework`; only `📜️script.ts` is exempt). Needs a declared local list or relocation.
4. `U/🧪️tests/{♻️physical-job-close, 🔬️targets-wgpu-mounted-layout-unit}` now import the helper from `🧰️framework/🔨️modules/📚️compiler/…`, a new `ui-no-framework-packages` edge (the rule already has 231 violations, and `crate-boundaries` already does the same). They are OS-engine tests living in the generic UI package; the clean fix is relocating them.
5. `E/🧪️tests/{🧾️frame-action-ledger, 🫀️settle-pump}` cannot run (missing fixture directory), `🌳️wgpu-document-reconcile`, `🔗️hub-projection`, `🚗️driver-editor` do not load under bun; their edits are unverified beyond type/parse and the isolation checks above.
6. Generated `M/🧑‍💻dev/📤️distribution/🧾️manifest.json` still lists the old Scenes, ShellHelpers, World3dHost bytes and hashes. Generated output; regenerate with its generator.
7. `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` were not edited (generated/seeded and being changed by others); the new test needs its launch entry.
8. Comments citing old line numbers (`🧊️renderer/🦀️.rs:12233` …) stay stale.

## 6. Files changed by this task

New: `H/🟦️.ts`, `H/🧪️tests/🟦️.ts`, `H/🧪️tests/🧫️fixtures/🔣️.json`.
Edited (helper-related): `…/🔍️discovery/📖️source-access/🟦️.ts`, `…/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🟦️.ts`, its fixture `…/🧫️fixtures/🔬️rust-policy-source-evidence/🔣️.json`, root `📜️script.ts`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts`.
Edited (readers): every file named in the tables above, plus the Rust files in section 4.
Scratch (outside the repo, not kept): `scratchpad/mt/*`.

## 7. Follow-up after the Shell split finished

Items 1, 3 and 7 of section 5 are now done (details below); the rest of section 5 stands.

### 7.1 Shell Rust readers: one `SHELL_SOURCE`

- `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` defines `#[cfg(test)] pub(crate) const SHELL_SOURCE` once: `concat!(include_str!())` of the parent plus the 20 children in declaration order (21 files, list generated with `listModuleTree`). The four modules that were already mounted before the split (`📤️icon-export`, `🪟️tree-windows`, `⏪️time-travel` with its two children, `📎️local-folders`) are left out on purpose: they were never part of the monolith text. (The TS tree helper still includes them; the Shell TS readers passed with that.)
- Every reader uses `crate::shell::SHELL_SOURCE` (`.to_string()` where it used to build a `String`): `🧪️tests/{🎓️tour-overlay-and-chord-glyphs (4 reads), 📤️wgpu-file-open-import, ⬇️wgpu-media-export-encoding, 🔄️sync-owner-lifetime, 🔗️hub-projection-workspace, 🔬️wgpu-shell-chrome-parity, 🗄️browser-prefs-persistence, 🚪️wgpu-host-door-remainder (2), 💓️chrome-maintenance-pressure}` under the Shell element, `E/🧪️tests/{📇️wgpu-directory-door, 🫀️settle-pump}`, and in the infinite crate (a different crate, so its own 21-file `SHELL_SOURCE` `concat!`) `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`. That is 14 files, 15 read sites.
- Visibility widening broke two body-boundary markers: `split("\n    fn ")` and `split("\n    async fn ")` no longer stop at methods that became `pub(crate) fn`. Measured against the HEAD monolith: `sync_session_chrome` body 890 → 15,158 bytes and `settle_boot` 2,088 → 50,775. Both sites (`🔬️wgpu-shell-chrome-parity:229`, `🫀️settle-pump:139`) now take the earliest of the private and the `pub(crate)` marker; the bodies are then identical to HEAD apart from the visibility text (checked).
- Verification (no cargo available): `rustfmt` parses all 23 touched Rust files; `rustc` compiled the marker expression; and an equivalence script compared every string literal these 11 tests pass to `contains/split/find/matches` (166 needles of at least 12 characters, plus 345 shorter fragments) between the HEAD monolith and `SHELL_SOURCE`: all counts are equal except the four visibility-prefixed item markers above. The tests themselves were not run.

### 7.2 Launch registration

- New nx target `test-module-tree` of `@semio-tech/compiler-syntax-rust-ts` (class in its `📜️script.ts`, entry in `📋️project.json`, script in `package.json`).
- New entry `🧪️test📚️compiler📖️syntax🌳️module-tree` in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, in group `9_gates` directly after the sibling `…🦀️rust🚮️absence` (order `900.058441`, between 900.05844 and 900.05845), same shape and `SEMIO_TEST_ARTIFACT_DIR` convention as the `…rust` entries.
- Both files parse (`launch.json` also as strict JSON: 8,772 configurations; seed 5,848 with JSON5), one `module-tree` entry each.
- `bun repo run @semio-tech/compiler-syntax-rust-ts:test-module-tree --skip-cache` ran green (7 pass, 0 fail, 54 expects, 8.2 s).

### 7.3 `✏️s` gis-variant ShellHost readers

No s-side tree helper exists and `✏️s` must not import `🧰️framework`, so both tests now carry an explicit file list next to the code: the ShellHost parent plus the 30 children split out of it (`git status` untracked set; the six modules that were already beside it, e.g. `👥️presence-scope`, are deliberately not in the list because they were never in the monolith text; reading the whole directory wrongly failed `semio-browser-broker-port`, which lives in that pre-existing module).
- `🧩️service-composition/🧪️tests/🟦️.ts` (broker-lane absence checks): reads the 31 files and joins them.
- `💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts`: the served-module precondition (`agentArtifactRouteRef`) now fetches each listed file that declares it on disk (2 files: `🏗️dock-assembly`, `🔒️identity-document-ports`) and requires the served copy to carry it too.
- Checked: both transpile; all 31 files exist; the four retired strings are in none of them. Not run: the first is registered through vitest by a service-composition caller, the second needs a live dev server.

### 7.4 Re-run (bun test unless noted)

| Test | Result |
|---|---|
| `🛑️scene-pointer-cancellation` | 4/1, same single winit-app failure |
| `⏱️frame-latency` | 2/1, same |
| `🧵️worker-cell` | 4/1, same (needs `SEMIO_TEST_ARTIFACT_DIR`) |
| `🔬️targets-wgpu-mounted-layout-unit` | 5/0 |
| `🪟️window-lifecycle-template-drag` | 9/0 |
| `🔬️rust-policy-source-evidence` | 6/0 (guard now also covers the Shell registry root) |
| `🌳️module-tree` tests via the new nx target | 7/0 |


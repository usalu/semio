# 1107 audit aud-bundle: demonstrator bundle at HEAD

Read-only static audit, 2026-11-07. No repo run, no cargo, no edits outside this folder. Probes: file reads, python 3 `-I` over the plugin registry, descriptors and wasm headers, `/usr/bin/grep`. Whole-tree greps (`scopeContributionsJson`, `rust-source-boundary`, `runtimeComponentClosure` by name) timed out at 2 minutes and were stopped; their gaps are marked UNVERIFIED.

## 0. Where the bundle lives

The brief's path `✏️s/🔌️plugins/🎪️demonstrator` holds only the owned playground artifact (`🗿️artifacts/🎪️playground`, 61 `.rs` files) and the mutation bridge (`🏭️bridge`). The composition itself is the hub crate `🌎️hub/🧩️compositions/🎪️demonstrator`:
- `📦️packages/🦀️rust/🦀️.rs` (lib, 44 lines, `mod artifacts`/`editor`/`viewer`/`manifest`)
- `🪪️manifest/🎪️demonstrator/🦀️.rs` (110 lines: the `PluginBuilder` wiring, `DemonstratorApps` fleet, imports at lines 17-25)
- `🧩️component/📦️packages/🦀️rust/Cargo.toml` (`consumes` line ~19, `depends-on` 14 entries, playground rows)
- `🧩️component/🔣️.json` (descriptor, appChannelVersion 24)

## 1. Verdict (short)

1. Closure is consistent: the generator lane closes to 30 components, plus energy and fem3d gives 32, equal to `1105b-tools/actinfra/ids.json`. The memory note's "28" is stale (section 4).
2. One likely compile break in the manifest: `sourcing::create_sourcing_curation_app` and `sourcing::create_sourcing_viewer` are imported through paths the standalone sourcing host does not use (section 2, lane `aussuchen` + `proc`). Not compiled, so UNVERIFIED.
3. Contribution budget is the top runtime risk. Measured cad+process+sourcing packs are about 30 KB (within 49,152 B). The `flow.extension` packs are not: brep alone is 333,796 chars (section 5).
4. Descriptor refresh is needed for all 32 closure components (all at appChannelVersion 24 vs pin 25), and the demonstrator descriptor is also stale beyond the version (section 7).
5. Clean: no `[DEBUG]` lines, no per-host `default-features` gap, no runtime feature needs, no `host`-flag catalogue pull, wasm function count far under 1M, relative path lengths under 256.

## 2. Manifest imports vs host lib roots (`🪪️manifest/🎪️demonstrator/🦀️.rs:17-25`)

| Import | Host definition / export | Status |
|---|---|---|
| `cad::editor::{create_cad_app, CadPlayApp}` | `CadPlayApp` at cad editor root `🦀️.rs:198`; `create_cad_app` in `🧭️app-manifest/🦀️.rs:50`, re-exported via `pub use app_manifest::*` (root `:189`). Same path as standalone cad hub. | clean |
| `gis::editor::gis2d::{create_gis2d_app, Gis2dPlayApp}` | gismap lib `pub mod editor { pub mod gis2d }` (`🗺️gismap/🦀️.rs:267-270`); same path as standalone gis hub | clean (glob chain not fully read) |
| `procedural::manifest::create_generation3d_app`, `procedural::Generation3dPlayApp` | `pub mod manifest` (`🧊️generation3d/🦀️.rs:72`), `pub use …Generation3dPlayApp` (`:13`); same as standalone procedural hub `:89` | clean |
| `process::editor::{create_process3d_app, Process3dPlayApp}`, `…::viewer::{create_process3d_viewer, Process3dViewer}` | all four at crate root (`✏️editor/🦀️.rs:173,264`; `👁️viewer/🦀️.rs:85,143`); same as standalone process hub `:32-34` | clean |
| `puzzle::editor::{create_puzzle3d_app, Puzzle3dPlayApp}` | `create_puzzle3d_app` in `📋️manifest/🦀️.rs:138`, `pub use self::manifest::*` (editor root `:257`); `Puzzle3dPlayApp` in `🧩️play-app/🦀️.rs:22`, `pub use self::play_app::*` (`:250`) | clean |
| `sourcing::editor::{create_sourcing_curation_app, SourcingCurationApp}` | curation lib `pub mod editor { pub mod sourcing {` at `🗂️curation/🦀️.rs:168-170`, which only glob-re-exports `_tests::editor::sourcing::*`, `_examples`, `_operations`, `_inferences`. `create_sourcing_curation_app` is defined at `…/✏️editor/📋️manifest/🦀️.rs:29`. | RISK (see below) |
| `sourcing::viewer::{create_sourcing_viewer, SourcingViewer}` | curation lib `pub mod viewer { pub mod sourcing {` at `:236-238`; `SourcingViewer` at `…/👁️viewer/🦀️.rs:41`, `create_sourcing_viewer` at `:102` | RISK (same pattern) |

RISK detail: the standalone sourcing hub reaches the same symbols as `editor::*` (`🌎️hub/🧩️compositions/🪵️sourcing/📦️packages/🦀️rust/🦀️.rs:33`, `viewer::*` at `:37`). It calls the factory as `editor::create_sourcing_curation_app()` from the subset-level lib (`…/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:25`), not as `editor::sourcing::…`. So the bundle's `editor::sourcing::create_sourcing_curation_app` (and `viewer::sourcing::create_sourcing_viewer`) is a path nothing in HEAD proves. Fix: import the factories from the path the standalone host uses, or re-export them under `sourcing` in the curation lib. Do not add a shim. Lane: `aussuchen` (curation lib) and `proc` (manifest). Verify by compile: `--target wasm32-wasip2`.

Other manifest checks: `.depends_on` calls (`:60-65`) match the registry `dependsOn` for the six plugins. Editor/viewer roster calls (`:67-86`) use the same concrete types as the standalone hubs. The `.requests(...)` rows (`:87-89`: `artifacts.write`, `ui.dialog`, `shell.navigate`) match the owning hosts.

## 3. MemberFactory roster vs standalone rosters (`genesis_child_pack … 's.stdio.semio@v1/kit'`)

- Same concrete app types and the same factory functions as the standalone hubs, so the per-app `ArtifactEditor::Members` are identical by construction. The differences are in the artifact registration:
  - Standalone hubs call `.artifact(<host>::declaration())`: cad (`📐️cad` hub `:31`), gis (`:49-50`, gismap + gisterrain), process (`:31`), procedural (`:76-77`, generation2d + generation3d). The bundle calls none of them. The standalone-parity test (`🪪️manifest/🎪️demonstrator/🧪️tests/🔬️standalone-parity/🦀️.rs`) expects this: foreign kind specs move into `hosted_artifact_kinds`. So this is by design, not proven as the cause.
  - generation3d's declaration claims `s.procedural.generation3d@1/*` (`generation3d/🦀️.rs:33`) and `s.stdio.txt@utf-8/*` (`:38`). The bundle claims neither as an owned artifact. Check whether the `kit` child of the generator app is resolved through that claim.
  - The bundle has no generation3d viewer (standalone procedural `:91`). Intentional: generator pane is editor-only.
- `s.stdio.semio@v1/kit` is not claimed by any bundled declaration I found. The bundle does not depend on `semio-s-artifact-stdio-semio` directly, but the cad and process editors do (their Cargo.toml). Lane `proc` + `runtime`: first check which member declares the kit child and whether the bundle's plugin-level roster includes it. Could not reproduce the failure statically.

## 4. Closure (`consumes` / `contributes` / `dependsOn`)

Source: `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json` (70 rows). Algorithm: `runtimeComponentClosure` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs:15-30` (dependsOn edges, `extends` edge shallow, consumes → every contributor, `host` pulls the catalogue). Simulated in python:

- demonstrator closure = 30: plugins demonstrator, cad, flow, gis, playbook, procedural, process, puzzle, sourcing (9); extensions 9 flow (bim, brep, dictionary, draw, list, logic, math, primitive, text), 4 process (concrete, metal, robotic, wood), 4 cad (aec-building, aec-building-energy, aec-building-structure, spatial-shape), 3 sourcing (beams, slabs, windows), 1 playbook-module-procedural (21).
- energy alone = `energy`; fem alone = `fem`. Union = 32 = `ids.json` exactly.
- Why 30 not 28: demonstrator and procedural now `consumes forms.questionKind`. Only `playbook-module-procedural` contributes it, and it `extends = "playbook"`, which pulls the playbook plugin. The memory note's 28 predates this. Activation audit section 3 (32) is right.
- No `host` pull: only `space` has `host = true` and it is outside the closure.
- No stdio edge: `gis` and `vcs` descriptors list stdio in `manifest.dependencies`, but registry `dependsOn` is empty for both, so stdio is not in the closure. Consistent with the Cargo comment on `stdio`.
- Cargo `depends-on` (14: 6 plugins + 8 flow ext, no draw) equals registry `dependsOn`. Draw enters through procedural (9 flow ext), not through the demonstrator's own list.
- Stale: descriptor `manifest.dependencies` lists only the 6 plugins. Refresh with section 7.

Clean. The lane split holds: `demonstratorActivationLanes()` skips generation3d because procedural is inside the primary closure (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/♻️activation/🟦️.ts`, comment above `:37`).

## 5. Contribution scoping and the 48 KiB budget

- Budget: `CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES = 49_152` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🦀️.rs:233`). Its doc (`:226-232`) records 29,909 pack bytes for cad.computer (4), process.machines (4), sourcing.module (3) on 2026-09-16. Do not raise it; owner `runtime`.
- Current proxy (compact JSON of each extension descriptor's `contributions`, from `🔣️.json`):
  - cad.computer: 1,840 + 880 + 3,765 + 357 = 6,842
  - process.machines: 4,908 + 5,748 + 4,936 + 4,086 = 19,678
  - sourcing.module: 1,413 + 1,045 + 1,143 = 3,601
  - subtotal 30,121, matching the 2026-09-16 figure to about 1%. Add forms.questionKind (playbook-module-procedural): 900. About 31 KB, 18 KB headroom.
- RISK, flow.extension: the demonstrator consumes it (`consumes` list) and so does procedural. Per memory, flow.extension packs are scoped by document operator reachability, not by consumes. Sizes: brep 333,796; bim 62,428; math 61,764; draw 52,120; list 16,218; dictionary 15,838; primitive 7,478; text 3,968; logic 3,944 (total 557,554). Brep alone is 6.8x the budget; any document reaching brep, bim, math or draw overflows, and the guest refuses with `typed command raw JSON exceeds its registered retained-page admission`. The 2026-09-16 proof did not include flow packs. I could not find the scoping implementation (`scopeContributionsJson`) within the budget, so whether reachability keeps it under 48 KiB is UNVERIFIED. Owner `runtime`. Options for the owner: split flow packs per pane, or narrow the demonstrator's reach. Not a limit raise.

## 6. Source boundary and wasm function ceiling

- Default boundary (from policy audits): 262,144 bytes, 32 files, 4,000 active lines per crate.
- Clean: hub demonstrator lib (1 file, 44 lines); manifest (4 files, 477 lines, 31 KB); component (1 file); bridge `🏭️bridge/🦀️.rs` (1 file, 79 lines, 5 KB).
- RISK: playground artifact lib tree `🗿️artifacts/🎪️playground` (its `#[path]` lib root `🦀️.rs` pulls the subsets) contains 61 `.rs` files, 186,920 bytes, 2,350 active lines. Bytes and lines are inside the limits; 61 exceeds 32 files if the gate counts the lib's directory tree. The bundle sets no `maxSource*` override, so the answer is to split (sub-crates already exist for schema/inferences/editor/examples), not to raise. UNVERIFIED: counting rule of the gate. Owner: demonstrator/`proc`.
- Wasm 1M function ceiling: demonstrator dist (dated 2026-10-12, predates HEAD): release 123,964 functions (100 imported + 123,864 defined), dev 180,427. About 12-18% of 1M. Not re-measured at HEAD. Clean for now.

## 7. Hub descriptors: appChannelVersion 24 vs pin 25

- Pin: `prefreeze.sh` `PASS channel-pin-code: pin=25` (`1105b-tools/prefreeze.sh:72-73` for the descriptor scan).
- All 70 `🧩️component/🔣️.json` under `✏️s/🔌️plugins` and `🌎️hub/🧩️compositions` are at 24 (35 hub compositions, 35 extension/module descriptors). The table below covers the 32 closure members; the other 38 (stdio, vcs, imperative, and other non-bundle plugins) are at 24 too and need the same refresh only if the bundle's closure changes.
- Needs refresh for the bundle (32 closure components):

| id | channel | descriptor |
|---|---|---|
| `demonstrator` | 24 | `🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/🔣️.json` (also stale: see below) |
| `cad`, `gis`, `procedural`, `process`, `puzzle`, `sourcing`, `flow`, `playbook`, `energy`, `fem` | 24 | `🌎️hub/🧩️compositions/<name>/🧩️component/🔣️.json` |
| `flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text}` | 24 | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/<name>/🧩️component/🔣️.json` |
| `process-extension-{concrete,metal,robotic,wood}` | 24 | `✏️s/🔌️plugins/🏭️process/🧩️extensions/<name>/🧩️component/🔣️.json` |
| `cad-extension-{aec-building,aec-building-energy,aec-building-structure,spatial-shape}` | 24 | `✏️s/🔌️plugins/📐️cad/🧩️extensions/<name>/🧩️component/🔣️.json` |
| `sourcing-module-{beams,slabs,windows}` | 24 | `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/<name>/🧩️component/🔣️.json` |
| `playbook-module-procedural` | 24 | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧩️component/🔣️.json` |

- Refresh command (from the `prefreeze.sh` INFO line, not run here): `repo run workspace:components describe <id> --profile prototype` (write mode), once per id above. Not a freeze blocker; expected after activation.
- Beyond the version, the demonstrator descriptor is stale: it has no `activationEvents` and no `capabilityRequests` (every other hub plugin has them), while HEAD `manifest.rs:87-89` declares three requests, and its `manifest.dependencies` lists 6 plugins only. The describe refresh must regenerate it, not just bump the version.

## 8. Other checks

- `[DEBUG]` lines: none in the bundle sources or the demonstrator hub crate (grep hits are only inside the wasm binaries). The 49 lines from the activation audit are in fem and puzzle files, outside this bundle.
- Default features: none needed. No host crate defines a `default` feature, and the demonstrator's `semio-s-artifact-*` entries (`🌎️hub/Cargo.toml:180-344`) do not set `default-features = false`, which is a no-op here. `testing`/`testkit`/`artifact-app-testing` features are dev-dependency only. Puzzle3d editor's wasm-bindgen block is gated by `not(target_env = "p2")`, so `wasm32-wasip2` excludes it; compile verdicts need `--target wasm32-wasip2`.
- Path length: relative bundle paths max 250 bytes / 169 chars (under 256). Repo-absolute the deepest fixture path is about 278 bytes (197 chars); under the 256-char rule, over if counted in bytes. Owner: `proc` (fixture tree under `🗿️artifacts/🎪️playground/🏅️standards/.../🧫️fixtures/🧬️mutations/✒️change-schema/📅️retags/`).

## 9. Not verified

- Compile of the manifest imports (section 2 RISK), gis `create_gis2d_app` glob chain.
- `scopeContributionsJson` implementation and whether reachability scoping stays under 48 KiB (section 5).
- The `s.stdio.semio@v1/kit` origin (section 3).
- Gate counting rule for `maxSourceFiles` (section 6).
- Wasm counts at HEAD (dist is stale).

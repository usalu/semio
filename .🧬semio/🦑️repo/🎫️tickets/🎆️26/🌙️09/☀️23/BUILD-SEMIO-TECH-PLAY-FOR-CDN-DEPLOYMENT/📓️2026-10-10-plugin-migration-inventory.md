# 2026-10-10 Plugin Migration Inventory (read-only audit)

Auditor: read-only, no sub-agents, no source edits, no cargo, no git. Only this file was written.
Scope: the 35 plugin directories under `✏️s/🔌️plugins/` (`🔒️policy-allowlist.json` is not a plugin).
Sources: `📓️2026-10-10-retained-clone-protocol-migration-brief.md`, `📓️2026-10-10-rename-rn-misc.md`, `📓️2026-10-10-rename-rn-stdio.md`, `📓️2026-10-10-redeploy-fleet-rules.md`.

## Method

Static census of every `*.rs` file under each plugin (skipping `target/`, `dist/`, `node_modules/`, `🗑️generated/`) and every `Cargo.toml` that contains `[package]`.

- (a) Store preparation factory impls: `impl ... for` headers for `ArtifactStoreOneItemPreparationFactory`, `ArtifactEphemeralOneItemPreparationFactory`, `MemberStoreOneItemWirePreparationFactory`, `ArtifactReplayPreparationFactory`. Trait definitions: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` lines 5152, 18659, 18944 and `.../🏪️store/🔁️replay/🎮️operation/🦀️.rs:109`. New required method: `begin_batch_digest` (store `🦀️.rs:18661`).
- (b) Mutation types that need `ArtifactCanonicalJsonTree`: every struct/enum whose `#[derive(...)]` contains `Mutations` or `MutationLeaf`, minus those that carry a `CanonicalJsonTree` derive or a manual `impl ArtifactCanonicalJsonTree for`. Trait: `🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs:9`. Result: zero manual impls exist in any plugin. Only `🧩️puzzle` derives `CanonicalJsonTree` (37 types), so puzzle is the only derive-set reference in the tree. Upper bound: fixtures and generated code may be included.
- (c) Old-protocol hits: the requested regex `SnapshotRetirementStep|JobPayloadCloseStep|\bowned_retirement\(|close_step\(`. This over-counts, because about 420 `close_step(` calls in the tree already pass a grant. The refined count keeps SRP, JPCS and `owned_retirement(` (all gone or replaced in the new framework) plus `close_step(` calls whose argument text has no `grant`.
- (d) `impl InteractiveJob for` count.
- (e) Crates: `Cargo.toml` with `[package]`. `.rs` lines: total per plugin, lines in files with a refined old hit, and lines in files with any hit (a, b-needing, c-refined, or d).

Effort score (heuristic, sorts the table): `score = old_refined + 20*factory_impls + 25*interactive_jobs + 0.2*mutation_types_needing_tree + 5*crates`. Weights are judgment calls; the components are listed so the order can be re-sorted.

## Release membership

`🏢️semio-tech/🎡️play/📋️project.json`: `build` depends on `prepare-release`, which depends on `catalog-release`. `catalog-release` lists 35 lanes, which map to 26 plugin directories. The lane-to-plugin mapping is by name and should be verified once.

| Lane | Plugin |
|---|---|
| demonstrator | 🎪️demonstrator |
| imperative | 📜️imperative |
| stdio-png, stdio-mp4, stdio-step, stdio-ifc-2x3, stdio-gltf, stdio-pdf-1-4-a, stdio-docx, stdio-semio-brep, stdio-binary (9 lanes) | 🗄️stdio |
| lowpoly | 💠️lowpoly |
| remodel | 📸️remodel |
| draw | 🖍️draw |
| raster | 🖨️raster |
| layout | 📏️layout |
| shooting | 🎥️shooting |
| block2d | 🧱️block |
| wfc2d | 🀄️wfc |
| fem2d | 🏗️fem |
| energy | 🔋️energy |
| architect | 🏛️architect |
| din4108 | 📕️norm |
| note | 🗒️note |
| writer | ✒️writer |
| forms | 📋️forms |
| mathematical | ➗️mathematical |
| reasoning-wires | 💡️reasoning |
| dag | 🕸️dag |
| trinity-jack | 🔱️trinity |
| bim | 🏙️bim |
| animate | 🎞️animate |
| sequence | 🎬️sequence |
| vcs | 🌿️vcs |
| home | no plugin directory (play workspace shell) |

NOT in the release gate (9 plugins, present in the play runtime manifest but not in `catalog-release`): 🧩️puzzle, 🌀️procedural, 🌊️flow, 🌍️gis, 📐️cad, 🏭️process, 📖️playbook, 🪐️space, 🪵️sourcing. Together they score 2225 of 7295 (about 30%). Scope decision needed: migrate them in this wave, or defer them.

## Per-plugin table (sorted by effort score, descending)

| # | Plugin | Release | Crates (e) | .rs lines (e) | Old hits (requested regex) | Old hits (refined) | Factory impls (a) | InteractiveJob impls (d) | Mutation types needing tree (b) | .rs lines in old-hit files | .rs lines in any-hit files | Score |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 🗄️stdio | 9 lanes | 121 | 666042 | 460 | 330 | 5 | 4 | 1015 | 17830 | 80489 | 1338 |
| 2 | 🧩️puzzle | NO | 4 | 156823 | 360 | 247 | 6 | 16 | 79 (37 already derive) | 52760 | 57173 | 803 |
| 3 | 🀄️wfc | wfc2d | 7 | 79212 | 212 | 211 | 1 | 13 | 92 | 16010 | 20352 | 609 |
| 4 | 🌀️procedural | NO | 3 | 81351 | 313 | 302 | 4 | 3 | 63 | 23431 | 27007 | 485 |
| 5 | 🖨️raster | raster | 2 | 28468 | 322 | 317 | 2 | 2 | 24 | 9766 | 11885 | 422 |
| 6 | 🖍️draw | draw | 4 | 44382 | 356 | 290 | 1 | 3 | 27 | 14717 | 16203 | 410 |
| 7 | 🔱️trinity | trinity-jack | 5 | 28614 | 285 | 284 | 0 | 0 | 22 | 6628 | 7483 | 313 |
| 8 | 🔋️energy | energy | 2 | 140007 | 97 | 96 | 1 | 2 | 305 | 19548 | 35658 | 237 |
| 9 | 🎞️animate | animate | 2 | 21189 | 150 | 146 | 1 | 2 | 13 | 3897 | 4467 | 229 |
| 10 | 🏭️process | NO | 6 | 26748 | 139 | 135 | 2 | 0 | 16 | 9801 | 10568 | 208 |
| 11 | 📕️norm | din4108 | 17 | 203022 | 0 | 0 | 0 | 0 | 571 | 0 | 20077 | 199 |
| 12 | 🏗️fem | fem2d | 5 | 91800 | 107 | 104 | 2 | 0 | 64 | 12762 | 16551 | 182 |
| 13 | ✒️writer | writer | 2 | 13367 | 119 | 116 | 1 | 1 | 13 | 4269 | 4859 | 174 |
| 14 | 🌍️gis | NO | 3 | 27110 | 113 | 110 | 2 | 0 | 33 | 4570 | 5977 | 172 |
| 15 | 📐️cad | NO | 6 | 28822 | 70 | 68 | 2 | 1 | 23 | 5896 | 6853 | 168 |
| 16 | 📏️layout | layout | 2 | 34848 | 43 | 43 | 0 | 4 | 49 | 7742 | 11423 | 163 |
| 17 | 🧱️block | block2d | 6 | 46428 | 19 | 12 | 4 | 0 | 109 | 4357 | 9597 | 144 |
| 18 | 🌊️flow | NO | 11 | 22862 | 75 | 42 | 0 | 1 | 0 | 6665 | 6665 | 122 |
| 19 | 🪵️sourcing | NO | 5 | 10704 | 49 | 47 | 2 | 0 | 4 | 2375 | 2538 | 113 |
| 20 | 🪐️space | NO | 4 | 15244 | 46 | 40 | 2 | 0 | 9 | 2725 | 3088 | 102 |
| 21 | 💠️lowpoly | lowpoly | 2 | 21849 | 27 | 25 | 2 | 0 | 23 | 3135 | 4466 | 80 |
| 22 | 📸️remodel | remodel | 2 | 70130 | 6 | 5 | 0 | 2 | 37 | 2720 | 4611 | 72 |
| 23 | 🏛️architect | architect | 2 | 92207 | 0 | 0 | 0 | 0 | 271 | 0 | 10908 | 64 |
| 24 | 🎬️sequence | sequence | 2 | 11085 | 24 | 24 | 0 | 1 | 0 | 4181 | 4181 | 59 |
| 25 | 🏙️bim | bim | 2 | 192942 | 2 | 0 | 0 | 0 | 217 | 0 | 7494 | 53 |
| 26 | 📖️playbook | NO | 3 | 8482 | 18 | 16 | 1 | 0 | 7 | 2483 | 2811 | 52 |
| 27 | 🕸️dag | dag | 2 | 7720 | 19 | 18 | 1 | 0 | 5 | 1401 | 1614 | 49 |
| 28 | ➗️mathematical | mathematical | 4 | 22950 | 11 | 1 | 1 | 0 | 18 | 1440 | 2244 | 45 |
| 29 | 🗒️note | note | 4 | 23167 | 17 | 17 | 0 | 0 | 36 | 1342 | 3195 | 44 |
| 30 | 🌿️vcs | vcs | 2 | 8141 | 11 | 10 | 1 | 0 | 7 | 2079 | 2361 | 41 |
| 31 | 📋️forms | forms | 2 | 16472 | 10 | 6 | 1 | 0 | 17 | 1468 | 2291 | 39 |
| 32 | 🎪️demonstrator | demonstrator | 2 | 3507 | 8 | 7 | 1 | 0 | 2 | 520 | 600 | 37 |
| 33 | 📜️imperative | imperative | 7 | 6947 | 0 | 0 | 0 | 0 | 4 | 0 | 182 | 36 |
| 34 | 🎥️shooting | shooting | 2 | 17046 | 1 | 0 | 0 | 0 | 41 | 0 | 1570 | 18 |
| 35 | 💡️reasoning | reasoning-wires | 2 | 7495 | 2 | 2 | 0 | 0 | 4 | 498 | 634 | 13 |
| | **Total** | | **257** | **~2.28 M** | **3491** | **3071** | **46** | **55** | **3220** (+37) | **247016** | **408075** | **7295** |

## Proposed split: 11 executor scopes (disjoint)

Ten plugin-level scopes would leave stdio at 1338, about twice the average of 663. stdio is therefore split by its `🗿️artifacts/` subtree. The split follows the measured counts (stdio subtree breakdown):

- Stdio A (score ~673): `🗿️artifacts/🧿️semio`, `📖️pdf`, `🧊️gltf`, `📜️docx`, `🎒️zip`, `🗜️deflate`, `🏗️ifc`, `📐️step`. Old 246, factory 3, jobs 2, mutation types 659, crates 37. Release lanes: stdio-semio-brep, stdio-pdf-1-4-a, stdio-gltf, stdio-docx, stdio-ifc-2x3, stdio-step.
- Stdio B (score ~665): `📇️registry`, `🔮️oracles`, and every other `🗿️artifacts/` format, including `🎥️mp4`, `💾️binary`, `📷️png`. Old 84, factory 2, jobs 2, mutation types 356, crates 84. Release lanes: stdio-mp4, stdio-binary, stdio-png. Coordination: `📇️registry/🧬️contract` (`RoutedNativeEditPreparationFactory`) is shared by A and B, so B owns it and A must not edit it.

| Scope | Plugins | Release lanes | Score | Old refined | Factory | Jobs | Mut. tree | Crates |
|---|---|---|---|---|---|---|---|---|
| E1 | Stdio A (above) | 6 stdio lanes | 673 | 246 | 3 | 2 | 659 | 37 |
| E2 | Stdio B (above) | 3 stdio lanes | 665 | 84 | 2 | 2 | 356 | 84 |
| E3 | 🧩️puzzle | NO (not gated) | 803 | 247 | 6 | 16 | 79 | 4 |
| E4 | 🀄️wfc, 🎪️demonstrator | wfc2d, demonstrator | 647 | 218 | 2 | 13 | 94 | 9 |
| E5 | 🌀️procedural, 🪐️space, 🗒️note, 💡️reasoning | note, reasoning-wires (procedural, space: NO) | 643 | 361 | 6 | 3 | 112 | 13 |
| E6 | 🖨️raster, 🪵️sourcing, 🏛️architect, 🌿️vcs | raster, architect, vcs (sourcing: NO) | 640 | 374 | 5 | 2 | 306 | 11 |
| E7 | 🖍️draw, 📏️layout, 🎬️sequence | draw, layout, sequence | 632 | 357 | 1 | 8 | 76 | 8 |
| E8 | 🔱️trinity, 🌍️gis, 💠️lowpoly, 📖️playbook, 📜️imperative | trinity-jack, lowpoly, imperative (gis, playbook: NO) | 653 | 435 | 5 | 0 | 89 | 20 |
| E9 | 🔋️energy, ✒️writer, 📸️remodel, 🌊️flow, 📋️forms | energy, writer, remodel, forms (flow: NO) | 644 | 265 | 3 | 6 | 372 | 19 |
| E10 | 🎞️animate, 🏗️fem, 🧱️block, 🏙️bim, ➗️mathematical | animate, fem2d, block2d, bim, mathematical | 652 | 263 | 8 | 2 | 421 | 19 |
| E11 | 🏭️process, 📕️norm, 📐️cad, 🕸️dag, 🎥️shooting | din4108, dag, shooting (process, cad: NO) | 642 | 221 | 5 | 1 | 656 | 33 |

All 26 release-gated plugins are covered by exactly one scope. Scopes E1 and E2 together cover all 9 stdio lanes.

If the 9 non-gated plugins are deferred (puzzle in E3; procedural and space in E5; sourcing in E6; gis and playbook in E8; flow in E9; process and cad in E11), the release-critical work is about 5070 of 7295 (about 70%).

## Caveats

- Nothing was compiled. The rename-rn-misc and rename-rn-stdio notes state that the framework store, job and OS crates were RED at 09:50 to 09:55. Current status was not re-checked; `🗑️generated/coord/os.status` is the gate.
- The gismap exemplar cited by the brief is stale per `rename-rn-stdio`: its one-item preparation does not match the three-generic request shape. No plugin has a verified complete one-item preparation today. Puzzle is the only tree-derive reference.
- The derive name is `semio_framework_value::CanonicalJsonTree` (used in puzzle), while the trait is `semio_framework_pack_json::ArtifactCanonicalJsonTree`. Confirm the derive generates the trait impl before fanning out.
- The mutation-type counts are derive-tagged structs and enums. stdio (1015), norm (571), architect (271), bim (217) and energy (305) likely include fixture or generated types. Treat them as an upper bound.
- The refined old-hit count is text-based (argument text without `grant`). It may miss or mis-classify a few sites; the executors should verify by compiling.
- Lane-to-plugin mapping is by name; `stdio` subtree-to-lane mapping (for Stdio A/B) is by name too.
- Effort weights are judgment calls. Re-sort by the component columns if the team weighs crates or mutation derives differently.

# 🧭 Wave 7 Candidates: Next Work Packages for Sonnet Executors

- Date: 2026-10-16, measured 12:54 local. Auditor: Haiku 5.5 (read-only). Folder: `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.
- No source file was edited. No modifying git command was run. No sub-agent was used. No ticket tool was called.
- Raw outputs: `🗑️generated/w7c-*` (`w7c-cands.py`, `w7c-wc-all.tsv`, `w7c-filter.py`, `w7c-candidates.tsv` with every census candidate and its exclusion class, `w7c-p11.py`, `w7c-p11-summary.txt`, `w7c-sections.awk`).

## 0. Method and state

- Base list: `MONOLITH-CENSUS-2.md` raw rows (`🗑️generated/mc2-over1500.txt`, 334 files). Re-measured with `wc -l` under `LC_ALL=en_US.UTF-8`. Of 334, 330 still exist. Four are split already: `engine-contract` (14,454 lines at census), `repo test harness` (`🧪️test/🟦️.ts`, 7,656), `gis service-composition` (7,388) and the TS kernel `🎠️kernel/🟦️.ts` (4,038). The census is therefore stale for them, as the fleet log says.
- Range: 1,500 to 2,900 lines now. 266 files are in range (census said 264). 57 files are over 2,900, and 7 have dropped under 1,500. The over-2,900 files are not in this brief's range and are left out (see `w7c-wc-all.tsv`).
- Exclusions (each candidate carries its class in `w7c-candidates.tsv`):
  - Peer areas from the brief: `🧬️contract` (ui-contract), `◻️2d`, `🧵️job`, `🏪️store`, `🛢️db`, `🌊️flow`, `🧊️wgpu` and the UI GPU cut (`targets-wgpu-*` tests), `📚️library`, `🎭️actor`, `🪟️member`, `🎬️media`.
  - Wave 5 and 6 slices (`FLEET.md`): frs (number, async, dsl grammar, spr channel), rhs (repo model, tickets, the three hub directory backends, server gateway), pfa and pfb (bim, process, raster, norm, energy, fem), rhu (renderer: all `📺️renderer/`), dlo (procedural core, gis).
  - Plugin crates (`✏️s/🔌️plugins/`, `os/🔌️plugin/`), `♾️infinite`, the hub `🌎️hub/`, vendored `♻️mit-bestand/`, and fixtures/corpora/generated.
- Dirty files (uncommitted, probably in flight) are excluded from the top ranks: `🎒️pack/🔤️json/🦀️.rs` (2,260) and `🖱️ui/📜️declaration/📐️layout/🦀️.rs` (1,909).
- Result: 74 free files after exclusion.

## 1. Wave 6 and running-slice status (evidence level)

| Slice | Evidence seen | Status used here |
| --- | --- | --- |
| `okwp8` | `okwp8-tools/` exists and is empty | running (per FLEET.md) |
| `dm2` | `dm2-tools/` (`modowners.ts`, `summarize.py`, 12:51 to 12:52) | running |
| `otc` | `otc-tools/cruise.sh` (dependency-cruiser over os/repo modules and os config, JSON per phase) | running; TS cycles in os/repo modules are its scope |
| `ts3` | `ts3-tools/imp.sh` (importer grep only) | running; **TS file list not recorded anywhere**, see §6 |
| `rhu` | `rhu-tools/` exists and is empty | running |
| `pi1`, `pch` | many logs and results; `pch` touches stdio artifacts (`pch-tools/g-drawing.ts`, `g-mp3.ts`, `g-locks.ts`) | running |
| `frs`, `rhs`, `pfa`, `pfb`, `dlo` | no tool folder in F | dispatch known only from FLEET.md |

## 2. Top 10 packages (one line each)

1. **WP-A OS services and MCP host (Rust, 5 files, 9,853 lines)**: `os/🛎️services`, `os/🌉️mcp`.
2. **WP-B Framework schema validator and value (Rust, 6 files, 11,508 lines)**: `🧬️schema`, `🌱️value`, `🧮️math/🎯️sampling`. Also clears about 580 `schema-validator-*`, `value-*` and `framework-schema` cruiser edges (306, 168 and 104).
3. **WP-F Framework tool, graph and machine (Rust, 6 files, 12,132 lines)**: `⏯️tool-run`, `🛠️tool-machine`, `🔄️machine`, `🕸️graph`.
4. **WP-C Repo product crate roots (Rust, 6 files, 12,283 lines)**: `repo/📦️packages/🦀️rust` for tree, test-runner, hooks, mcp, languages and providers. It excludes the model, tickets and server packages that rhs owns.
5. **H1 Shell test bins, 32 tests (Rust, size not measured)**: the 32 Shell tests RI2 left in the tree (RI2 §5).
6. **WP-H OS run, VCS, io, neural and pack-value (Rust, 5 files, 9,624 lines)**: `os/🏃️run`, `os/🌿️vcs`, `os/🚪️io`, `os/🧠️neural`, `os/🎒️pack/🌱️value`.
7. **WP-D Pets product (mixed Rust and TS, 4 files, 7,609 lines)**: `🐾️pets` clearance tests, schema and locomotion. Census: "fleet-touched, file not split".
8. **WP-E Print, presentation, quiz and teaching products (mixed, 5 files, 9,895 lines)**: `📓️print`, `🎤️presentation`, `❓️quiz`, `🎓️teaching`. TS files need the ts3 check (§6).
9. **WP-G Framework 3D BREP kernel (Rust, 5 files, 10,432 lines)**: `🔨️modules/🧊️3d`. Caution: an active BREP boolean investigation is recorded in the user's memory (see §6).
10. **WP-I1 Framework UI React elements and i18n (TSX and TS, 4 files, 8,700 lines)**: `🖱️ui` Scene, Canvas, Diagram and i18n. UI peer risk, see §6.

Hand-off packages H2 to H5 are gated (they need pi1, rhu or peer work to close first). They are ranked after the top 10 in §4.

## 3. Free source packages (detail)

Common verification for all Rust packages: static evidence now (the rustc `--emit=metadata -o /dev/null` differential from `📓️RESOLUTION-ORACLE.md`, rustfmt parse, line multiset). Real check when the compile window opens, under the fleet mutex: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crate>… --lib --keep-going --message-format=short`, plus `--tests`.

Common verification for TS packages: `tsc --noEmit` on the touched closure, and the owning test file set before and after (failure set identical or smaller).

Common "why not owned": census status "free" (no claim found), not named in any Wave 5 or 6 slice, and not in any RESULT file's hand-off list.

### WP-A OS services and MCP host (rank 1)

| File | Lines |
| --- | ---: |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs` | 2,666 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️.rs` | 1,955 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` | 1,922 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs` | 1,749 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs` | 1,561 |

- Size: 9,853 lines, 5 files.
- Why unowned: W4-OS-MCP (wave 4) carved the mcp host and is done. Nothing since. The services root is a 2,666-line facade.
- Verification: static differential per crate, then `check -p` on the os services and mcp crates (both `--lib` and `--tests`).
- Risks: the services root is the public facade of many os crates. Re-point consumers in the same edit (no shim). Check `🌉️mcp` against the plugin tree before cutting.

### WP-B Framework schema validator and value (rank 2)

| File | Lines |
| --- | ---: |
| `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs` | 2,264 |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/🦀️.rs` | 1,780 |
| `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs` | 2,356 |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs` | 1,712 |
| `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🎛️processors/🦀️.rs` | 1,797 |
| `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🚧️constraints/🦀️.rs` | 1,599 |

- Size: 11,508 lines, 6 files.
- Why unowned: census "free"; `framework-schema`, `schema-validator-*` and `value-*` edges (about 580 prod in `DEPENDENCY-DIRECTION-AUDIT-2` §3 P11) are not in any slice.
- Verification: static differential, then `check` of the framework schema, value and math crates. The derive expansion needs a proc-macro check (`--lib` of the derive crate plus a dependent).
- Risks: the derive expansion emits paths into dependent crates, so a path change needs every derive consumer checked. Check the `dsl` path-root defect in OS-KERNEL-SQLITE-HOST §6 first. Use `#[path]` children with explicit `use`.

### WP-F Framework tool, graph and machine (rank 3)

| File | Lines |
| --- | ---: |
| `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs` | 2,420 |
| `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs` | 2,256 |
| `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🦀️.rs` | 1,876 |
| `🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/🦀️.rs` | 1,828 |
| `🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs` | 1,971 |
| `🧰️framework/🔨️modules/🔄️machine/🦀️.rs` | 1,781 |

- Size: 12,132 lines, 6 files.
- Why unowned: census free; no wave 5 or 6 slice names these framework roots. PWP7 owns `⏯️tool-run/🧵️instance`, a different path. Check that `⏯️tool-run/🦀️.rs` is not the plugin-facing root before cutting.
- Verification: static differential, then `check` of the tool-run, graph and machine crates.
- Risks: `tool-run` root is imported by the plugin tier-2 crates WP5/WP7 (`pi1`, `pwp7` plans), so re-point those imports in the same edit and tell `pi1` through this file.

### WP-C Repo product crate roots (rank 4)

| File | Lines |
| --- | ---: |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🦀️rust/🦀️.rs` | 2,390 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/📦️packages/🦀️rust/🦀️.rs` | 2,153 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/📦️packages/🦀️rust/🦀️.rs` | 2,142 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust/🦀️.rs` | 2,134 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers/📦️packages/🦀️rust/🦀️.rs` | 1,754 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗣️languages/📦️packages/🦀️rust/🦀️.rs` | 1,710 |

- Size: 12,283 lines, 6 files.
- Why unowned: these are crate roots of repo packages. rhs owns the model, tickets, hub backends and server gateway only. None of these six is in a slice.
- Verification: static differential, then `check -p` on each repo crate with `--tests`. Repo crates are in the repo workspace, so use the repo `--manifest-path`.
- Risks: `repo-lib` (parent-held `📚️library`) imports some of these roots. Re-point its imports and call it out to the parent. `OS` and `pi1` do not depend on them.

### WP-H OS run, VCS, io, neural and pack-value (rank 6)

| File | Lines |
| --- | ---: |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs` | 2,463 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs` | 1,909 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs` | 1,894 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs` | 1,736 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧱️model/🦀️.rs` | 1,622 |

- Size: 9,624 lines, 5 files.
- Why unowned: census free. `🚪️io` here is the os module, not the store `S/🚪️io` owned by OKSQL. Check the path before cutting.
- Verification: static differential and `check -p` for run, vcs, pack-value and neural. `pack-value` has an existing `pack` package and a dirty sibling (`🔤️json`, excluded), so check its importers.
- Risks: `🎒️pack/🌱️value` is a pack crate that `pack::encode_json_value` depends on (memory: heavyweight). Test with the pack tests, not only `check`.

### WP-D Pets product (rank 7)

| File | Lines |
| --- | ---: |
| `🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🦀️.rs` | 2,258 |
| `🧰️framework/🛍️products/🐾️pets/🧬️schema/🦀️.rs` | 1,851 |
| `🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🟦️.ts` | 1,768 |
| `🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🟦️.ts` | 1,732 |

- Size: 7,609 lines, 4 files. The pets stage test `🎪️stage/🧪️tests/🔬️unit/🟦️.ts` (3,075) is over the range and is a separate candidate.
- Why unowned: CARVE-PETS done; the census shows "fleet-touched, file not split". No slice since.
- Verification: `check -p` for the pets Rust crates (lib and tests); `tsc --noEmit` and the pets vitest set for the TS files.
- Risks: the clearance unit tests may read fixtures under `🧫️fixtures` (excluded from this list), so check their reads first. Run the TS files on the baseline set.

### WP-E Print, presentation, quiz and teaching products (rank 8)

| File | Lines | Lang |
| --- | ---: | --- |
| `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🟦️.ts` | 2,011 | TS |
| `🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts` | 2,209 | TS |
| `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts` | 1,766 | TS |
| `🧰️framework/🛍️products/❓️quiz/🧪️tests/🚶️learner-journey/🟦️.tsx` | 2,138 | TSX |
| `🧰️framework/🛍️products/🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🦀️.rs` | 1,771 | Rust |

- Size: 9,895 lines, 5 files.
- Why unowned: census "free" for the presentation and print files. PRESENTATION-DEDUPE (wave 4) left the core package in place. Quiz and teaching are not in any slice.
- Verification: `tsc --noEmit` and the owning vitest files (baseline failure set); Rust end-to-end test via `bun test`/`check -p`.
- Risks: TS files may overlap ts3 (see §6). The presentation core package is a dedupe target (RESULT-PRESENTATION-DEDUPE §8.4). Keep its taxonomy path unchanged.

### WP-G Framework 3D BREP kernel (rank 9)

| File | Lines |
| --- | ---: |
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🦀️.rs` | 2,498 |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs` | 2,462 |
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️.rs` | 2,203 |
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/📏mass-properties/🦀️.rs` | 1,750 |
| `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` | 1,519 |

- Size: 10,432 lines, 5 files. All clean (no uncommitted change).
- Why unowned: census free; none of the three 3D roots is in a slice. The CAD plugin consumes this kernel, but its plugin tree is not in any active slice.
- Verification: static differential for each file, and `check -p` for the brep and mesh crates. Add a geometry golden test (the BREP tests) before and after.
- Risks: high. The user's memory records recent BREP boolean work (box-through-box). Confirm with the user before any cut. Keep the boolean file out of the first batch.

### WP-I1 Framework UI React elements and i18n (rank 10)

| File | Lines | Lang |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts` | 2,354 | TS |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🟦️.tsx` | 2,158 | TSX |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx` | 2,135 | TSX |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🟦️.tsx` | 2,053 | TSX |

- Size: 8,700 lines, 4 files.
- Why unowned: census "peer-owned (ui aggregate)". The peer's `ui-contract` and UI GPU cut are excluded. These four files are not in the peer's named scope. Confirm before dispatch.
- Verification: `tsc --noEmit` on the ui closure; ui-react vitest (baseline 1,157 passing per EPHEMERAL-SINGLE-OWNER).
- Risks: UI is hot. UI-CYCLES (remaining loops A to F) and UI-BARREL-IMPORTS leftovers touch the same tree. Re-read before each edit. Keep the `🧱️elements` element re-exports intact.

## 4. Hand-off packages (gated)

- **H1 Shell test bins (rank 5, unblocked except compile).** RESULT-RENDERER-INTEGRATION-2 §5: 32 of 55 Shell tests still sit in the Shell tree. 25 share helpers with monolith-only fixtures (`collect_fixture_actions`, `camera_fixture_lease`, `engine_surface_law_guard`). 3 read `crate::RENDERER_SOURCE`, and `settle-pump`, `sync-owner-lifetime` and `document-relay` also need a `renderer_source()`. Verification: `check -p` of the bins with `--tests`. Blocked by the compile window. The 25 fixture-sharing tests need their three fixture files moved first.
- **H2 WP7 companion bins T10 to T21 (blocked by pi1).** RESULT-PLUGIN-WP7 §8.2: 64 test-only files, 41,153 lines. `pwp7-tools/tbins7.py` packs them into 12 bins of at most 3,300 lines. Not applied, and nobody runs it now. The order depends on pi1's tier-1 flips (WP7 §8.1). Start after pi1 closes.
- **H3 Layout to stdio Rust drawing coupling (blocked by design and compile).** RESULT-RUST-CROSS-COUPLINGS §2: 30 Rust files, 155 uses, 11 layout manifests, 32 member lines over 11 stdio crates. Four design options are written there. A binding crate in the hub composition is the recommended one. Overlap risk: `pch` (`g-drawing.ts`) works on the same drawing standard.
- **H4 Renderer fixture paths (blocked by the ui peer and rhu).** RESULT-RENDERER-TS-TESTS-SPLIT §7: 48 ui corpus fixture paths (the retained/resident corpus needs an async public entry) and 34 other fixture paths in the renderer tests. The `🧬️contract/🧵️retained` tree is under active peer edits. The renderer tests are rhu territory.
- **H5 Plugin tests reading framework source text (48 reads).** RESULT-SDK-BOUNDARY §5 counts 48 `new URL(...🧰️framework...)` or `readFileSync` reads in plugin tests (not import edges, so the cruiser misses them). SDK-BOUNDARY-2 added the `@semio-tech/framework/test/source-text` entry for 101 reads. Re-point the 48 remaining reads to it. Verification: the owning vitest files against the baseline. Overlap risk with pi1 and pch.
- **H6 UI leaf relocations (UI-BARREL-IMPORTS decision table).** Move `validateJsonSchemaSubset` (222-line pure leaf, 60 importers, 3 prod and 3 test edges), `🕹️interaction` (326 lines, 4 edges), `UIDialog` pure functions (1 file), and the `🎬️scene` to `🛂️manifest` request types (2 edges, UI-CYCLES E). Overlap: WP-I1 and the ui peer. Sequence it after WP-I1.
- **H7 Manifest product vocabulary (unverified).** RESULT-MANIFEST-SPLIT §7 lists a 1,787-line `product-vocabulary/🦀️.rs` with wrong-direction regions (`ActionKind`, `Introduction`, `Tutorial`, `HistoryEdit`, `ToolRun`, `AppRole`, `PackageRole`). The path is not in the tracked tree now. Re-locate it before any dispatch.

## 5. Dependency-cruiser hygiene families (P11)

Source: `DEPENDENCY-DIRECTION-AUDIT-2` §3 P11, re-aggregated from `dda2-depcruise-err.txt` by `w7c-p11.py` (test paths excluded). Production edges only.

| Family | Prod edges | Area | Status |
| --- | ---: | --- | --- |
| `no-cross-package-relative-@semio-tech/repo-lib` | 1,020 | repo library (`📚️library`) | parent-held. Not for Wave 7 |
| `no-cross-package-relative-@semio-tech/norm-js` | 691 | `✏️s/🔌️plugins/📕️norm` | pfa running. Blocked |
| `no-cross-package-relative-@semio-tech/process` | 526 | `✏️s/🔌️plugins/🏭️process` | pfa running. Blocked |
| `no-cross-package-relative-@semio-tech/framework-sqlite-snapshot-rs` | 372 | framework sqlite snapshot | not checked against the store slices. Medium |
| `no-cross-package-relative-@semio-tech/framework-rs` | 281 | framework Rust | spread across modules; take with WP-A, WP-B, WP-F |
| `no-cross-package-relative-@semio-tech/framework` | 280 | framework TS | spread; `framework-os` 229 and `framework-os-kernel` 229 overlap `otc` cycles |
| `no-cross-package-relative-@semio-tech/schema-validator-ts` and `-rs` | 306 | schema validator | WP-B |
| `no-cross-package-relative-@semio-tech/value-ts` and `-rs` | 168 | value | WP-B |
| `no-cross-package-relative-@semio-tech/framework-schema` | 104 | schema | WP-B |
| `no-cross-package-relative-@semio-tech/framework-2d` and `-js` | 104 | 2D | peer (`◻️2d`). Excluded |
| `no-cross-package-relative-@semio-tech/framework-actor-*` | 102 | actor | peer (`🎭️actor`). Excluded |
| `no-cross-package-relative-@semio-tech/framework-plugin`, `plugin-registry` | 250 | plugin tree | pi1 and pch. Excluded |
| `no-core-path` (stdio base cluster, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**/🧱️base/…/🚪️io/🪶️sqlite` and `🧬️schema/📸️snapshot`, `🧬️mutations`) | 498 of 606 | stdio artifacts | **pch owns the stdio artifacts.** Blocked until pch closes. The fix is a neutral stdio sqlite-snapshot module imported by every subset |
| `no-core-path` to `@/lib/*` (26: persistence, authentication, event, source-index) and to `util`, `zlib` (25) | 51 | unclear importing package | not traced. Needs one more read |

Cross-package edges mostly need the SDK or package-export pattern (SDK-BOUNDARY and SDK-BOUNDARY-2 used it). Each family needs its own dependency-direction re-run. The `verify dependencies literal-external` gate (300 current, target 0) is the authoritative check, and the repo-lib layering lint still cannot run in this environment (DEPENDENCY-DIRECTION-AUDIT-2 §5).

## 6. Open points and risks for the coordinator

1. **ts3 scope is not recorded.** The TS candidates (WP-E TS files, WP-I1, the repo test children `🧪️test/🧬️schema/📋️orchestration/🟦️.ts` at 2,609 and `🧪️test-platform` at 1,562, the os TS files in WP-A and WP-H, and the cad dev test at 2,243) may overlap ts3. Get ts3's file list before dispatching any TS package.
2. **otc (cycles) overlaps the os TS modules** in WP-A (`🧾️app-channel`, `🧵️backbone`, if those are added) and in the `framework-os` P11 family. Rust packages are not affected.
3. **Wave 6 dirs not found.** frs, rhs, pfa, pfb and dlo have no tool folder in F. Their file lists are taken from FLEET.md names only. Re-check the RESULT files when they land.
4. **Compile is still gated.** No Rust package can be proven with `check` until the image has headroom. Every Rust package is therefore static-first, with the batched `check` queued.
5. **Peers in the UI tree.** `ui-contract` and the UI GPU cut are excluded. WP-I1 and H6 are not. Ask the ui owner before dispatch.
6. **The census and FLEET.md disagree on times.** FLEET.md says Wave 6 dispatched at 13:3x; the tool folders are timestamped 12:51 to 12:53 and the system clock read 12:54. This report uses the tool folders.

## 7. Recommended order

1. WP-A and WP-B (Rust, clean, and WP-B clears about 580 P11 edges). Both can be started now, static-first.
2. WP-F and WP-C (Rust, clean). Confirm WP-F's `tool-run` root with pi1 first.
3. WP-H and WP-D (Rust and mixed, clean).
4. H1 (Shell test bins) once the compile window opens.
5. WP-E and WP-I1 (TS), after ts3's list is confirmed and the ui owner has agreed.
6. WP-G (BREP), after the user confirms no active boolean work.
7. H2, H3, H4, H5, H6 and the P11 stdio family, each after its named blocker closes (pi1, pch, rhu, the ui peer, or the compile window).

Reserve (not ranked, for later): the 57 over-2,900 files in `🗑️generated/w7c-wc-all.tsv`, and the `🎪️stage/🧪️tests` 3,075-line pets test.

# Result fem-fix: FEM fixtures to `🧪️tests/🧫️fixtures`, generators re-pointed

Executor `fem-fix`, 2026-10-19. Foreground, no sub-agents, no cargo, no git-modifying command, no `package.json`/`bun.lock`/repo-library/root-script/taxonomy edit. Tools kept in `fem-fix-tools/` (`fixmove.py` = tst5 mover, `fixedits.py` = reference re-pointer from the saved move plan, `verify*.py`, `oraclecheck.py`, `callall.ts`, `runbun.py`, `suites.txt`). Generated output deleted.

## Changes

1. Moved 533 plugin FEM fixtures (`✏️s/🔌️plugins/🏗️fem`, 22 `🧫️fixtures` dirs outside `🧪️tests`) to `<dir>/🧪️tests/🧫️fixtures/**`; 532 by `fixmove.py` (dry run first, 0 conflicts), the schema-ancestor `🌐️any/🧬️schema/🏗️structure/🎟️fixed-dofs/🧫️fixtures/🔣️.json` by hand to `🌐️any/🧪️tests/🧫️fixtures/🎟️fixed-dofs/🔣️.json` (reader TS + Rust re-pointed).
2. Also moved the 12 engine fixtures under `✏️s/🔨️modules/🏗️fem/⚙️engine/**` (analyses, elements2d, elements3d, sparse, result-source; not covered by the plugin law but same class). Empty `🕸️mesh/🧫️fixtures` dirs removed.
3. 1,104 + 24 literal references in 76 + 21 files re-pointed (Rust `include_str!`, TS imports and `readFileSync`, oracle JSON `path`s). Hand fixes where the base is not the file dir: `🧪️tests/{📖️mutation-grammar,🧱️field-intent/**,🧱️material-receiving}`, engine `📍node`, `🧱️material/📋️table` tests (the mover mis-resolved `resolve(root,"../🧫️fixtures")`; corrected by hand), 2D/3D `🔬️rust-package-commands` (verification-laws, numerical-close, mounted-stiffness), 2D transient + 3D any oracles (10 stale `path`s that were already in `🧪️tests`), three docstrings/`.feature`, `📦️packages/🦀️rust/📋️project.json`.
4. Generators (per dimension 2d/3d, `🌐️any/🏭️generator`): `📜️script.ts` `CARRIER_FIXTURES` (39 routes) and `🧩️json/🏗️generate/🦀️.rs` (39 routes, the cargo carrier writer) now route to `<subset>/🧪️tests/🧫️fixtures/<kind>`; `FIXTURE_PATH_PREFIX` follows the already-canonical `outDir`; fixed a missing `/` in the emitted manifest path (`${PREFIX}/${dir}/${file}`); `📋️project.json` inputs reduced to `subsets/*/🧪️tests/🧫️fixtures/**/*` (dropped the dead old glob).
5. Test added to the existing `🧪️tests/📖️mutation-grammar/🟦️.ts`: the TS and Rust route tables are equal and every route's `⏮️before.json`/`⏭️after.json` exists under `🧪️tests/🧫️fixtures`.

## Verification

- Equivalence of the move: sha256 of all 533 + 12 files before vs after = identical (script run, 545/545).
- Placement: `bun tst5-tools/fixplace.ts` (law's own walk) = 0 offenders across all plugin owners (was 533 FEM).
- Reference resolution: `fem-fix-tools/verify2.py` = 2416 Rust `include_str!`/`fixture!` literals in FEM, 0 missing (`fixture!` resolved against the Cargo manifest dir); `verify.py` literal sweep leaves only generator-relative tables.
- Generator regeneration (no cargo): `bun 📜️script.ts generate --out <ticket tmp>` for 2d (12 bundles) and 3d (15): all 54 STL/OBJ files byte-identical to the committed fixtures; the 27 metrics files are identical except the label `measuredFrom` (generator writes `🧊️expected.stl`, committed files say `expected.stl`; pre-existing drift, committed bytes match the oracle sha, left as is). Carrier table check: TS table = Rust table, 39/39 before/after files exist, both dimensions.
- `bun test ./<file>` (2 workers, tst5 harness) on the 22 FEM suites + `✏️s/🔌️plugins/🧪️tests/📁fixture-placement`: 151 pass / 6 fail. Green: all fixed-dofs, sqlite snapshot, examples, mutation-grammar (incl. new check), shared-authority, field-intent x5, material-receiving, pcg cold. Directly invoked oracle functions (`bun fem-fix-tools/one.ts|callall.ts`): materialTableOracle, nodeOracle, verifyGeometricCellOracle, sparse/analyses/elements3d oracles, 3D editor oracles all OK.
- Not run: cargo (Rust tests, carrier `generate`/`carrier-manifests`), `bun repo run` rust-package-commands (path existence checked only).

## Open items (not caused by this change)

- Red, unrelated: `🧪️tests/{⚖️combination-stress,🌀️geometric-cells}` (`factory.runIf` missing in this bun), modules `🕸️mesh/📐️surface` x2 (need `SEMIO_FEM_*_RESULTS` from cargo), fixture-placement law 2 fails in energy/bim (peer paths).
- 3D `🌐️any/🔮️oracles/🔣️.json`: 52 carrier `sha256`/`bytes` entries were already stale versus the committed fixtures (files unchanged by this move); regenerate with `generator carrier-manifests` (needs cargo).
- The mover's literal resolution is file-dir based: `resolve(root, "…")`-style and `fixture!` (manifest-relative) readers must be audited by hand (done here); first `fixmove apply` run reported 0 edits, so edits were re-applied from the saved plan with `fixedits.py`.

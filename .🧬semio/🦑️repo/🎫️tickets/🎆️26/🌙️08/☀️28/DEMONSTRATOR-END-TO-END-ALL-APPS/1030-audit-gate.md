# 1030 Audit Gate: source-boundary census and test-gate probe (lane gateaudit)

STATUS: DONE (read-only, no source edits, no cargo, no sub-agents). Source boundary: 0 violators in the demonstrator closure (1640 crates). Test-target gate: 8 lib-test targets over the limit (root workspace), 1 closure-reader error.

## 1. Method

- Census: `bun C/1028-tools/gatecensus/census.ts C/1030-tools/gateaudit/roots.txt`, run from `/Users/ueli/Documents/semio`. Output `C/🗑️generated/1030/gateaudit-census.txt` (one JSON line, rc 0, stderr empty, about 137 s).
- Roots (136 package names): the 134 names of `C/1029-tools/census/roots.txt` (hub `semio-hub-*-component` for cad, flow, process, playbook, gis, procedural, sourcing, energy, fem, puzzle, demonstrator, and the 21 extension components) plus 2 component packages found under the twelve plugin dirs (`semio-s-artifact-procedural-3d-mesh-component`, `semio-s-artifact-stdio-gltf-inferences-connected-components`). The plugin dirs themselves hold 1354 manifests (`C/🗑️generated/1030/gateaudit-manifests.txt`), so the root set was not taken as all packages in those dirs. Closure: 1640 crates, 1640 rows, closure-reader errors 0. Closure differs from 1029 (1638) by the 2 added roots.
- Limits: min(262144 B, 32 files, 4000 lines, per-crate `rust-source-boundary`).
- Test gate: `bun C/1030-tools/gateaudit/testgate.ts <workspace>` (copy of `C/1029-tools/rta/testgate.ts`, unchanged logic). Run on `/Users/ueli/Documents/semio` (root workspace, 652 targets) and `/Users/ueli/Documents/semio/✏️s` (25 targets). Outputs `C/🗑️generated/1030/gateaudit-testgate-root.txt` and `gateaudit-testgate-s.txt`.
- Summary: `C/🗑️generated/1030/gateaudit-summary.txt` (script `C/1030-tools/gateaudit/summarize.ts`).

## 2. Source-boundary census (closure, non-test cfg)

- Over any limit: **none** (0 of 1640).
- Above 95% of a limit (13), watch list for the next growth:

| crate | bytes / 262144 | files / 32 | lines / 4000 | owner lane (inferred from dir/results) |
|---|---|---|---|---|
| `semio-framework-os-vcs` | 261954 (99.9%) | 31 | 3258 | runtime |
| `semio-framework-plugin-instance-app` | 260621 (99.4%) | 16 | 3132 | runtime |
| `semio-s-artifact-energy-model-snapshot` | 260745 (99.5%) | 29 | 2776 | energy |
| `semio-s-artifact-fem-2d-model` | 235473 (89.8%), lines 3914 (97.9%) | 27 | 3914 | fem |
| `semio-framework-plugin-window-config` | 257008 (98.0%) | 20 | 1932 | runtime |
| `semio-framework-os-store-engine` | 256895 (98.0%) | 13 | 2890 | runtime |
| `semio-s-artifact-procedural-3d-model` | 256118 (97.7%) | 30 | 2348 | gen |
| `semio-framework-value` | 253539 (96.7%) | 30 | 3368 | runtime (not verified) |
| `semio-framework-plugin-job` | 254960 (97.2%) | 14 | 2766 | stepctx |
| `semio-framework-plugin-reactor-turn` | 250817 (95.7%) | 21 | 3119 | runtime |
| `semio-s-artifact-stdio-xml-snapshot` | 233506 (89.1%), lines 3851 (96.3%) | 23 | 3851 | none in 1030 lanes (stdio) |
| `semio-s-artifact-stdio-gltf-model-snapshot-root` | 146168 | 31 (96.9%) | 1236 | none in 1030 lanes (stdio) |
| `semio-s-artifact-stdio-binary-snapshot` | 79572 | 31 (96.9%) | 955 | none in 1030 lanes (stdio) |

- Limit raises: none. The 1029 raised-limit crates are now at defaults: `semio-framework-os-kernel` (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`) and `semio-s-artifact-fem-2d-editor` both read 262144 / 32 / 4000. The measurer min()s raised values, so the census rows cannot show a raise; this was checked by the manifest scan `C/🗑️generated/1030/gateaudit-boundary-meta.txt` (only the fem editor manifest sits in the 12 plugin dirs with a block, and it is at default).

## 3. Test-target gate (cfg test + debug_assertions, offline)

Root workspace (`/Users/ueli/Documents/semio`): 652 targets, 8 over, 1 reader error. Owner lanes are inferred from the lane result files (1030-result-*.md) and the directory; nothing in the result files names these crates except where noted.

| crate (target) | bytes (limit 262144) | files (limit 32) | lines (limit 4000) | owner lane |
|---|---|---|---|---|
| `semio-framework-os-vcs` (lib-test) | 365825 (139%) | 59 | 4035 | runtime |
| `semio-framework-plugin-decl-views` (lib-test) | 319326 (122%) | 42 | 3371 | runtime |
| `semio-framework-plugin-window-config` (lib-test) | 343280 (131%) | 37 | 2498 | runtime (also touched by fem result) |
| `semio-framework-plugin-tests-05-2` (lib-test) | 119192 | 33 (103%) | 1026 | runtime (tests-05 support module, fixed in 1030-result-runtime) |
| `semio-framework-repo-dashboard-registry` (lib-test) | 274441 (105%) | 21 | 1950 | runtime (framework repo, not in a 1030 lane result; owner unverified) |
| `semio-framework-os-store-engine` (lib-test) | 266216 (102%) | 16 | 2919 | runtime |
| `semio-framework-plugin-job` (lib-test) | 266294 (102%) | 16 | 3040 | stepctx (WidgetJob::step lives in this crate per 1030-result-stepctx) |
| `semio-framework-plugin-publication-mounted` (lib-test) | 273133 (104%) | 18 | 2545 | runtime (owner unverified) |

- Reader error: `semio-framework-plugin-tests-19` (lib-test): `Missing or ambiguous Rust module: preparation`. Not a limit violation; a missing module in the test support crate, owner runtime (tests-19 is a runtime companion test crate).
- Workspace `/Users/ueli/Documents/semio/✏️s`: 25 targets, 0 over, 0 errors.
- The test-target limits are reached only with cfg(test) included, so the census (non-test) passing does not imply these pass. The gate is per target, so each of the 8 needs a split or a test-only fixture move before the test-gate is green.
- Lanes not named as owners in this audit: gen, playbook, resid, fem (only window-config reference), energy, hubplug, bundle. No gen/playbook/energy/hubplug/bundle crate is over the test gate; fem owns the 97.9%-lines watch row `semio-s-artifact-fem-2d-model`.

## 4. Action for owners

1. runtime: split `semio-framework-os-vcs` (lib-test over bytes, files and lines), `semio-framework-plugin-decl-views`, `semio-framework-plugin-window-config`, `semio-framework-os-store-engine`, `semio-framework-plugin-publication-mounted`, `semio-framework-repo-dashboard-registry` and `semio-framework-plugin-tests-05-2` (33 files, one over) test modules out of the lib, or move them into a test crate. Note the census (non-test) is under the limit, so this is a test-only overflow.
2. stepctx: `semio-framework-plugin-job` lib-test is 266294 B (102%) and 3040 lines (76%). Move the StepContext tests into `tests/`.
3. runtime: fix the `semio-framework-plugin-tests-19` missing module `preparation` (reader error, the gate cannot measure it).
4. Watch (no action now): `semio-s-artifact-fem-2d-model` (lines 97.9%, fem), `semio-s-artifact-energy-model-snapshot` (99.5%, energy), `semio-framework-os-vcs` (99.9% non-test, runtime).

## 5. Caveats

- Lane owners are inferred from crate directories and the 1030 lane results, not from the manifests. Lanes marked "owner unverified" have no lane result naming the crate.
- The census is an over-approximation of the closure (path deps, default features, optional deps), per the 1029 method.
- Peers keep editing. This is a snapshot from about 2026-10-30 08:10. Re-run `census.ts` and `testgate.ts` after any split.
- The measurer min()s per-crate limits, so a raised limit would not show in the census rows. The manifest scan is the only check for raises, and it covered the 12 plugin dirs only. `semio-framework-os-kernel` was read directly and is at default.

## 6. Files

- Census output: `C/🗑️generated/1030/gateaudit-census.txt` (+ `.err`, empty)
- Census summary: `C/🗑️generated/1030/gateaudit-summary.txt`
- Test gate outputs: `C/🗑️generated/1030/gateaudit-testgate-root.txt`, `gateaudit-testgate-s.txt` (+ `.err`, empty)
- Roots: `C/1030-tools/gateaudit/roots.txt`; manifests `C/🗑️generated/1030/gateaudit-manifests.txt`; hub names `C/🗑️generated/1030/gateaudit-hubnames.txt`
- Boundary-meta scan: `C/🗑️generated/1030/gateaudit-boundary-meta.txt`
- Scripts: `C/1030-tools/gateaudit/summarize.ts`, `C/1030-tools/gateaudit/testgate.ts`

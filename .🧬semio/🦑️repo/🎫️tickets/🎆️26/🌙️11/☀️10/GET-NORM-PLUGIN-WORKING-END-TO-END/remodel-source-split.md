# Remodel mutations crate source split

## Measurement (`semio-s-artifact-remodel-remodeling-mutations`, cfg(test) unit, limit 262144)
Closure follows `#[path]` and `include_str!`; its sum matches the refusal to the byte (259,023 read before the 10,555 B grammar). Total 273,715 B in 20 files, 11,571 over.

| Module (domain) | Bytes |
|---|---|
| schema leaf `🟦️.ts` (typescript projection) | 58,580 |
| oracles catalog `🔮️oracles/🔣️.json` (kinds-catalog test) | 56,762 |
| tests: unit 37,726, retained-params 14,834, retained-results json 14,313, other fixtures ~4,500 | ~71,400 |
| io/text codec rs 26,884 + grammars 10,555 + 4,137 | 41,576 |
| dispatch enum `🦀️.rs` | 11,613 |
| leaves proto 11,510, graphql 8,374, json-schema 5,096 | 24,980 |
| io/binary codec (rs, protocol) + apply boundary + package shell | ~7,900 |

Grammar (14.7 KB) is not the driver; the schema leaf projections (83.5 KB) and test fixtures are.

## Split
- Text/binary codecs cannot leave the crate: `impl protocol::OpText/OpBinary for RemodelingMutation` is a foreign trait on a foreign type from any other crate (orphan rule). The retained_params tests impl `RetainedLeafWrapper` for the enum, same reason.
- Split at the schema level instead: new crate `semio-s-artifact-remodel-remodeling-mutations-leaves` at `🧬️schema/🧬️mutations/🍃️leaves` owns `MUTATION_LEAVES` (`schema::mutation_leaves`), including the five leaf files by relative path (files not moved). It depends only on `semio-framework-schema-registry`.
- Removed `MUTATION_LEAVES` region and the now-unused `semio-framework-schema-registry` dep from the mutations crate. No re-export; the only consumer (subset root descriptor `🦀️.rs`, crate `semio-s-artifact-remodel-remodeling`) now imports the leaves crate; its Cargo.toml and the workspace members/dependencies were updated.
- Result: mutations closure 273,715 -> 189,730 B (72 % of budget, 16 files); leaves crate ~95 KB (leaves + enum source).

## Files touched
remodeling/Cargo.toml (member + dependency), remodeling/📦️packages/🦀️rust/Cargo.toml, subset root 🦀️.rs, mutations 🦀️.rs and its package Cargo.toml, new 🍃️leaves (Cargo.toml, package 🦀️.rs, 🦀️.rs).

## Verification
See the end of this file (appended after the runs).

## Budget scan (all crates; `remodel-split-scan.py`, full output `remodel-split-scan-full.txt`)
Heuristic closure (same walker as above, cfg(test)-aware by attribute adjacency). Within 10 % (0.90-1.00) of 262144 in the lib unit, selected:
- 0.999-0.99: stdio ifc4/ifc4x3-add2/ifc2x3 `catalog-entities-NN` shards (generated, packed to the limit; 29 crates), norm-en1996-scene (261,897), dsl-record-model (261,612), os-flow-host-session, plugin-host-guest-runtime, ifc-4-model
- 0.98-0.95: plugin-window-config, puzzle-2d-snapshot, energy-model-snapshot, plugin-host-wake, norm-en1990-scene, plugin-retained-command, fem-3d-model, norm-en1993-model, ui-contract-values, layout-mutations, fem-2d-model, pdf-schema-5, layout-io, os-store-engine, norm-en1991-inferences
- 0.95-0.90: renderer-wgpu mailbox-tests, framework-value-tests, plugin-reactor-turn, plugin-instance-jobs, layout-engine-export, norm-en1993-schema, drawing-model, plugin-instance-app, publication-mounted, layout-diff, gismap-demo-asset, value-native-owned, norm-en1995-kernel, replication-retained, raster-editor, value-native-decoding, wgpu-shell-actions, replication-history, decl-app, testing-laws, plugin-ownership, raster-host, decl-views, plugin-job, obj-schema, plugin-interpreter, puzzle-5d-schema, bim-model-inferences, framework-job-context (236,537), io-sqlite-snapshot, store-sync-tests-1, process3d-model.
- cfg(test) unit additionally within 10 %: e.g. replication-mutation-messages, pack-json, value-native-encoding, pack-record-retained, dsl-derive-expand, guest-contract, os-flow-host-read, tool-machine, value-persistent, os-flow-host-core, os-mcp, snapshot-read, wgpu-host-plumbing (74 crates are estimated over in test mode, see the full file).
- Caveat: crates estimated over the limit (e.g. ifc4x3-schema/templates, layout/drawing/raster descriptors, bim-model-schema, framework-pack, procedural-3d-model, note-schema) include large `include_str!` data or generated code; this estimator may overcount them versus real admission (which also caps files at 32 and active lines at 4000, not scanned). Verify before acting.

## Verification status (appended)
- Source admission: not yet re-proven for the new layout. Estimator shows mutations at 189,730 B (limit 262,144), well under; the old refusal path is gone by construction.
- Two `check --tests` runs (`-p mutations -p mutations-leaves -p remodeling`) were queued 40+ min each behind the capacity queue; the first was SIGTERM-cancelled externally, the second compiled framework dependencies (no `error` lines) and was cancelled before reaching the remodel crates ("Original Script process terminated"). Logs: `🗑️generated/remodel-split-check.log`, `remodel-split-check2.log`.
- NOT RUN (build pause from coordinator): `check --tests` on mutations, leaves, remodeling, editor, viewer; and `test -p semio-s-artifact-remodel-remodeling-mutations --lib retained_params`. Re-run once, in this order, after resume:
  1. `repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --tests -p semio-s-artifact-remodel-remodeling-mutations -p semio-s-artifact-remodel-remodeling-mutations-leaves -p semio-s-artifact-remodel-remodeling`
  2. `... -- test -p semio-s-artifact-remodel-remodeling-mutations --lib retained_params`
- Cargo.lock of the remodeling workspace was refreshed by the first run (new crate added).
- Orphaned-process note: I SIGKILLed my own wrapper shell (pid-specific) to stop the chained test run from starting during the pause.

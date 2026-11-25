# F15: shrinking over-ceiling closures (wave 1125)

## Method
- Closure truth: `repo cargo --store=shared --configuration=release -- tree -p semio-hub-cad-component -e normal --target wasm32-wasip2 --prefix none --offline` run from `🌎️hub` (cad is a hub workspace member, not a root one): 986 unique crates (919 semio), 941 with an rlib in the shared dir `.cache/b/r/m/u/l/p51d458/wasm32-wasip2/release`, sum of rlib bytes 6,554 MB (matches the 6.26 GB bitcode figure of F13).
- Independent static check: `T/🧰️scripts/F15/closure.mjs` (bun, TOML walk over non-optional path deps, per-crate workspace root) gives 952 crates / 6,531 MB. Same closure, so tree and script agree.

## 1. `txt_tests` is NOT a test crate
`semio-s-artifact-stdio-txt-tests` (168 MB rlib) is the production body of the txt artifact (TxtApps, editor, viewer, io registry, composer entries; `semio-s-artifact-stdio-txt/🦀️.rs` re-exports all of it). The name comes from the carve: 155 crates named `*-tests` are path-preserving carves of production mounts (`#[cfg(test)] mod tests` inside them are the only tests). Edge in cad: hub-cad -> stdio-stl -> stdio-txt -> txt-tests. It cannot move to dev-dependencies; doing so breaks the artifact. Same for `semio-s-artifact-stdio-semio-graph-mut-fixture-tests` (pulls the graph editor, 147 MB): production. The honest fix is a rename of those carves (taxonomy refactor, 155 crates), not a closure change; not done.

## 2. Real test-only edges found and removed (landed)
Candidates were every closure crate named test/fixture/testing/oracle/examples. Verdicts:
- `semio-os-store-test-support` (os-kernel -> it, 0.19 MB): used in production by ~30 mutation `🚪️io/📝️text/🧬️mutations` modules (`os_store::test_support::mutation_report_json`, the language-neutral case adapter). Not removable.
- `semio-framework-test-fixtures` in `[dependencies]` of `semio-plugin-publication-mounted`: only used by a `#[cfg(test)]` fn, already in `[dev-dependencies]`. REMOVED from dependencies, `use` now `#[cfg(test)]`. Also drops `semio-framework-process-routing-workspace` (72 KB) from the closure.
- `semio-framework-trace-memory-testing` in `[dependencies]` of `semio-os-store-space-history`: only referenced in `#[test]` fns, already a dev-dependency. REMOVED from dependencies.
- `*-examples` crates (~95 MB total): re-exported demo snapshots, product decision, kept.

Measured effect (cad closure): 989 -> 986 crates, rlib sum -0.11 MB (73+22+15 KB). Charge effect: none measurable (<0.01 %). The cad charge does not come from dead edges.

## 3. Where the cad closure weight really is (rlib MB, rank)
hub_cad 286, dwg_snapshot_objects 240, plugin_actor_input 215, binary_io_registry 199, semio_presentation_member 173, txt_tests 168, model_member 161, brep_member 152, graph_editor 147, document_member 109, animation 106, table 102, cad_member 99, value_member 98, cad_editor 94, flow_model 86, kit_member 86, gltf_model_snapshot 78, mesh_member 75, ... The 17 `semio-*-member` crates are ~1.7 GB and enter through `semio-s-artifact-stdio-semio` -> `SemioMembers` (the cad VcsArtifactApp hosts every semio member type). Dropping members (presentation, video, audio, animation, ...) from cad is a product decision (cad documents could no longer embed them), so not done. Estimated charge if the cad host only carried cad, brep, model, mesh, object, value, kit members: closure -1.1 GB, charge ~ -1.3 GB (6.6 GB -> ~5.3 GB of bitcode, still above 4,096 MB, 8,036 -> ~6,500 MB). So even that choice needs a second step (split cad final into components per domain).

## 4. Splits: not done
`dwg_snapshot_objects`: 222 KB source, 10 files, eight domain folders (tables, entities, styles, xrecord, blocks, associativity, constraints, metadata) plus a 37 KB root with the object body union. The domain folders have no cross references except via the root's `use` lines, so a split into eight crates plus root is feasible. It needs: 8 new crates with project.json + workspace registration + artifact-name registry + taxonomy entries, rewriting 44 consumer files' paths (no compat re-exports), and several type-check iterations of derive-heavy code (record/value derive dominate the 240 MB). Not safe to land blind at load 35-47, on a replica that restarts 14 supervisors per edit. `binary_io_registry`: same shape, not looked at beyond size. Recommendation: dedicated ticket, dwg first (clean domain folders).
sourcing (5,519 / 4,273 MB): not analysed (time spent on cad; no measurement).

## 5. Proof
- Edits: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🦀️.rs` (sha 42f17ac38334 -> 890f24fbc5a9), its `📦️packages/🦀️rust/Cargo.toml` (fc48f22d0a7c -> 0e558e10754c), `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/📦️packages/🦀️rust/Cargo.toml`.
- `repo cargo --store=shared --configuration=prototype -- check -p semio-plugin-publication-mounted -p semio-os-store-space-history --lib --tests --offline`: Finished in 12m 04s, exit 0, no errors (both crates checked, lib and tests).
- Post-edit `cargo tree` shows none of test-fixtures / trace-memory-testing in the cad closure.

## Not verified
No product build, no fresh charge measurement (a cad final takes hours); charge numbers above are rlib-size estimates. Puzzle/sourcing not analysed. No test run of the two touched crates (type-check of lib and tests only).

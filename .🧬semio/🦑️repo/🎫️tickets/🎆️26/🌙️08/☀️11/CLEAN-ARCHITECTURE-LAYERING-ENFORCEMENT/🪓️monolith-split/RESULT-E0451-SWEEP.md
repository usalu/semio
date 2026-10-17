# 🧱 Result E0451-SWEEP (slice `e51`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. Tools: `F/e51-tools/`. Logs and JSON: `F/🗑️generated/e51-*` (nothing deleted).

## 0. Verdict (wave 7, 2026-10-17 08:40)

**DONE (static).** The whole-workspace E0451 scan is at 0 hits for every site that has a nameable struct. The real compile is not run: `COMPILE ADMITTED` is absent from `📓️FLEET.md`, so the exact command is queued in section 6. 7 structs whose own first error is a name-resolution error (not E0451) are handed off in section 5.

* Whole workspace (2,335 crates as users, integration targets included): 131 E0451 hits found after the first session's 37 edits; all fixed.
* Re-scan of the 26 user crates (701 crates loaded, targets included): 0 hits.
* Sensitivity: the same re-scan with all 126 journalled edits undone in memory returns 344 hits; `lit.py selftest` PASS.

## 1. Why a scanner

rustc reports E0451 (private field in a struct literal or struct pattern) in the privacy pass, which never runs while any other error exists. The union oracles therefore cannot see it. `F/e51-tools/lit.py` resolves every brace-struct literal and pattern against the definitions of the crate and of its dependency closure (module tree from `mod`, `#[path]`, `include!`), with testkit-gated fields counted public. `scoped.py` (new) runs it for chosen user crates or for all 2,415 workspace packages and can replay the journal's inverse in memory (`--undo`).

## 2. Found state (after the session restarts)

* Wave 7 start: the file had last been written at 00:08; the run continued until the third restart (about 07:05) without updating it. The journal `e51-edits.jsonl` held 37 edits (store, pack-json, value, directory-client, services, dsl-derive, hub), none half applied, none rolled back. Oracle before/after pairs existed for `otcps`, `otcpc`, `otcpx`, `hdc`, `ftcvmin`, `e51min`.
* The first session scanned the family lists (28 family sites, all fixed) and listed 193 world-only hits, which were not worked.
* Gap found in wave 7: the world scan of `lit.py scan` loads only the family crates and their dependencies. It misses dependents (consumers) of carved crates, which is exactly where the fleet's cross-crate splits break. `scoped.py --all` covers all 2,415 packages.

## 3. What changed in wave 7 (89 journalled edits, 126 in total over 111 files)

Rule: widen to the narrowest visibility that serves the cross-crate user (`pub`; `pub(super)` for a same-crate sibling), or add an owner constructor when the struct has an invariant. No alias, no re-export shim.

| Group | Structs | Fix |
|---|---|---|
| Mutation leaves (stdio plugins: bcf, ply, md, xlsx, pptx, docx base and strict, gif, stl) | 74 structs in 10 `*-mutations` crates (36 + 38) | all fields named by a foreign literal or pattern: `pub(crate)` to `pub` (the literals live in the io-registry, schema, editor and test crates) |
| Flow | `FlowBaselineLeases.current`, `FlowSurface`, `FlowProgramFeature.action`, `FlowAction2524.items`, `FlowAction2599.output`, `FlowVcsCensus`, `FlowVcsSource` | `pub(super)`/private to `pub`; `SessionPreviewRetention` (same crate) to `pub(super)` like its sibling field `removed` |
| Neural | `Dictionary.pairs` | `#[doc(hidden)] pub` (repo convention for cross-crate construction fields; also serves the `.pairs` field uses of the budgeted crate) |
| dwg | `EncodedR2004Page`, `R2004SectionDescriptor` | all fields `pub` |
| cad, layout | `CadWorkingScene` (8 fields), `TypedJsonCursor` (3 fields) | `pub(crate)`/private to `pub` |
| process3d | `ContributedMachineCatalog` | owner constructor `ContributedMachineCatalog::new`; the kit caller uses it |
| First session (kept) | pack-json `OwnedSchemaDecodePage`, store `CursorRevisionRecord`/`DurableOwnedThreeMemberUnsignedV1`/ephemeral and canonical-json cursors, directory-client, services `BufferedHttpBody`, dsl-derive `MutationLeafAttrs`, value `RetainedCloneClose`, hub `HubReadinessV1` | owner constructors and `with_` builders, callers changed in the same call group |

Not hits (checked by hand): the 10 "ambiguous" rows are enum variant declarations (`Circle {..}`, `Rational {..}`, `Symbol {..}`), not literals of the primitives struct.

## 4. Evidence (commands run, real results)

* `python3 e51-tools/lit.py selftest`: PASS.
* `python3 e51-tools/lit.py scan --families all --tag w7` (about 23 min wall at load 50): families union 0 hits; world 190 hits (all fixed below); output `🗑️generated/e51-w7.log`, `e51-w7-e0451.json`.
* `python3 e51-tools/scoped.py --all --targets --tag allw7` (26 min): 2,335 crates, 131 hits, 10 ambiguous, 22 unnameable; `e51-allw7.log`.
* After the fixes, `scoped.py --pkgs <26 user crates> --targets --tag w7bfix`: 701 crates, **0 hits**, 0 ambiguous, 2 unnameable (section 5). `e51-w7bfix.log`.
* Seed (sensitivity): `scoped.py ... --tag w7bseed --undo "/"` (all 126 journalled edits replayed backwards in memory): **344 hits**. `e51-w7bseed.log`.
* Journal check: all 126 `new` texts are still present in their files; 0 missing.
* `rustfmt --edition 2021` (stdin parse) over the 111 edited files: 0 failures.
* Oracle before/after (reduced families `otcps`, `otcpc`, `otcpx`, `hdc`, `ftcvmin`, `e51min`, by `e51-tools/cmp.py`): 0 NEW non-stub rows; one E0422 became an E0433 for the same unresolved name in `ftcvmin test` (message form of an existing stub gap). The full `ftcv`, `opd`, `okernel`, `okwp8` oracles were not run (machine load 50 to 80; the wave-7 edits are visibility-only plus one constructor, so no new resolution error can come from them).
* Source-text readers: `git grep -F` over `*.ts`/`*.py`/`*.json` for the old field texts and the struct names found only ticket history files; no reader test pins them.

## 5. Hand-off: unnameable sites (not E0451 today)

These structs are private to another crate, so the user's first error is E0422/E0603/E0425, which hides the E0451 behind it. They are pre-existing test-mounting drift (integration targets or moved tests that rely on private items), not part of this slice; `RESULT-PLUGIN-FILE-SPLITS-A.md` §8 already expects the process3d one to fail. When the owner exports the struct, the listed fields must become visible too.

| Struct | User root | Hits | Owner crate | Needs |
|---|---|---|---|---|
| `Process3dPublicationLease` | `process3d-host#test:process3d_host_retained_laws` | 6 | `process3d-host` | struct and 10 fields; the test also needs `cfg(test)`-only lib items |
| `Intra4Neighbors` | `remodeling-video` lib tests `🧪️tests/🔬️unit` | 8 | `remodeling-video-h264` (`pub(super)`) | struct and 4 fields; the test also lacks the `predict_intra4x4` import |
| `Puzzle3dSessionState` | `puzzle-3d-editor#test:editor-budgets` | 3 | `puzzle-3d-editor-session` | struct and fields `meshes`, `instances`, `collision` |
| `ContentTypeRow` | `stdio-docx#test:mutation_laws` | 1 | `docx-base-mutations` | struct and fields `name`, `content_type` |
| `Frame` | `stdio-ifc#test:sqlite_snapshots` | 2 | `ifc-4-model`, `ifc-2x3-base-model` | struct and 6 to 7 fields |
| `PptxDiffRecord` | `stdio-pptx#test:model_laws` | 1 | `pptx-base-model` (`pub(crate)`) | struct and field `value` |
| `SetCamera` | `draw-drawing-editor-tool` window test | 1 | `plugin-window-config` (declared in a `🧪️tests` file) | struct and field `camera` |

## 6. Compile queue (the launcher is not admitted: no `COMPILE ADMITTED` line in `📓️FLEET.md` at 08:40)

```
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-directory-client -p semio-framework-os-directory-client-tests -p semio-framework-os-flow-host-core -p semio-framework-os-flow-host-session -p semio-framework-os-flow-vcs -p semio-framework-os-flow-vcs-laws -p semio-framework-os-flow-wasm -p semio-framework-os-flow-wasm-actions-a -p semio-framework-os-flow-wasm-actions-b -p semio-framework-os-flow-wasm-domain -p semio-framework-os-kernel-dsl-derive-expand -p semio-framework-os-kernel-dsl-derive-tests -p semio-framework-os-kernel-neural-engine-budgeted -p semio-framework-os-kernel-neural-engine-model -p semio-framework-os-services -p semio-framework-os-services-tests -p semio-framework-os-store-canonical-json -p semio-framework-os-store-durable-group -p semio-framework-os-store-durable-group-wire -p semio-framework-os-store-engine -p semio-framework-os-store-engine-bundle -p semio-framework-os-store-engine-stage -p semio-framework-os-store-ephemeral -p semio-framework-os-store-initialization-runtime -p semio-framework-os-store-ledger -p semio-framework-os-store-tests-06 -p semio-framework-os-store-tests-09 -p semio-framework-plugin -p semio-framework-plugin-decl-app -p semio-framework-plugin-decl-registry -p semio-framework-plugin-decl-views -p semio-framework-plugin-instance-dispatch -p semio-framework-ui -p semio-framework-value-retained-clone -p semio-framework-value-tests -p semio-hub-boot-laws -p semio-hub-runtime -p semio-s-artifact-cad-cad-editor -p semio-s-artifact-cad-cad-io -p semio-s-artifact-cad-cad-model -p semio-s-artifact-layout-layout-engine-export -p semio-s-artifact-layout-layout-engine-json -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-kit -p semio-s-artifact-process-process3d-model -p semio-s-artifact-stdio-bcf-io-registry -p semio-s-artifact-stdio-bcf-mutations -p semio-s-artifact-stdio-bcf-schema -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-docx-base -p semio-s-artifact-stdio-docx-base-mutations -p semio-s-artifact-stdio-docx-strict-mutations -p semio-s-artifact-stdio-dwg-write -p semio-s-artifact-stdio-dwg-write-records -p semio-s-artifact-stdio-gif-editor -p semio-s-artifact-stdio-gif-fixture-tests -p semio-s-artifact-stdio-gif-mutations -p semio-s-artifact-stdio-gif-mutations-2 -p semio-s-artifact-stdio-md-mutations -p semio-s-artifact-stdio-md-tests -p semio-s-artifact-stdio-ply-io-registry -p semio-s-artifact-stdio-ply-mutations -p semio-s-artifact-stdio-pptx -p semio-s-artifact-stdio-pptx-base -p semio-s-artifact-stdio-pptx-base-mutations -p semio-s-artifact-stdio-pptx-strict -p semio-s-artifact-stdio-pptx-transitional -p semio-s-artifact-stdio-stl-examples -p semio-s-artifact-stdio-stl-mutations -p semio-s-artifact-stdio-xlsx -p semio-s-artifact-stdio-xlsx-base -p semio-s-artifact-stdio-xlsx-base-mutations -p semio-s-artifact-stdio-xlsx-strict -p semio-s-artifact-stdio-xlsx-transitional --message-format=short
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-directory-client -p semio-framework-os-directory-client-tests -p semio-framework-os-flow-host-core -p semio-framework-os-flow-host-session -p semio-framework-os-flow-vcs -p semio-framework-os-flow-vcs-laws -p semio-framework-os-flow-wasm -p semio-framework-os-flow-wasm-actions-a -p semio-framework-os-flow-wasm-actions-b -p semio-framework-os-flow-wasm-domain -p semio-framework-os-kernel-dsl-derive-expand -p semio-framework-os-kernel-dsl-derive-tests -p semio-framework-os-kernel-neural-engine-budgeted -p semio-framework-os-kernel-neural-engine-model -p semio-framework-os-services -p semio-framework-os-services-tests -p semio-framework-os-store-canonical-json -p semio-framework-os-store-durable-group -p semio-framework-os-store-durable-group-wire -p semio-framework-os-store-engine -p semio-framework-os-store-engine-bundle -p semio-framework-os-store-engine-stage -p semio-framework-os-store-ephemeral -p semio-framework-os-store-initialization-runtime -p semio-framework-os-store-ledger -p semio-framework-os-store-tests-06 -p semio-framework-os-store-tests-09 -p semio-framework-plugin -p semio-framework-plugin-decl-app -p semio-framework-plugin-decl-registry -p semio-framework-plugin-decl-views -p semio-framework-plugin-instance-dispatch -p semio-framework-ui -p semio-framework-value-retained-clone -p semio-framework-value-tests -p semio-hub-boot-laws -p semio-hub-runtime -p semio-s-artifact-cad-cad-editor -p semio-s-artifact-cad-cad-io -p semio-s-artifact-cad-cad-model -p semio-s-artifact-layout-layout-engine-export -p semio-s-artifact-layout-layout-engine-json -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-kit -p semio-s-artifact-process-process3d-model -p semio-s-artifact-stdio-bcf-io-registry -p semio-s-artifact-stdio-bcf-mutations -p semio-s-artifact-stdio-bcf-schema -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-docx-base -p semio-s-artifact-stdio-docx-base-mutations -p semio-s-artifact-stdio-docx-strict-mutations -p semio-s-artifact-stdio-dwg-write -p semio-s-artifact-stdio-dwg-write-records -p semio-s-artifact-stdio-gif-editor -p semio-s-artifact-stdio-gif-fixture-tests -p semio-s-artifact-stdio-gif-mutations -p semio-s-artifact-stdio-gif-mutations-2 -p semio-s-artifact-stdio-md-mutations -p semio-s-artifact-stdio-md-tests -p semio-s-artifact-stdio-ply-io-registry -p semio-s-artifact-stdio-ply-mutations -p semio-s-artifact-stdio-pptx -p semio-s-artifact-stdio-pptx-base -p semio-s-artifact-stdio-pptx-base-mutations -p semio-s-artifact-stdio-pptx-strict -p semio-s-artifact-stdio-pptx-transitional -p semio-s-artifact-stdio-stl-examples -p semio-s-artifact-stdio-stl-mutations -p semio-s-artifact-stdio-xlsx -p semio-s-artifact-stdio-xlsx-base -p semio-s-artifact-stdio-xlsx-base-mutations -p semio-s-artifact-stdio-xlsx-strict -p semio-s-artifact-stdio-xlsx-transitional --tests --message-format=short
```

Success means no E0451 in either run; expect the section 5 test targets to fail with their own name-resolution errors under `--tests`.

## 7. Files

Edited: 111 Rust files listed by `🗑️generated/e51-edits.jsonl` (full old/new pair of every edit; `e51-edits.log` is the short form). New tools: `e51-tools/widen.py`, `scoped.py`, `w7-mut.py`, `w7-nonmut.py`, `w7-nonmut2.py`, `w7-nonmut3.py`. No debug logs were added.

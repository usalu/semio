# RESULT: BASELINE-DEFECTS-8 (`bd8`)

Executor: Sonnet, 2026-10-17 about 09:55 to 11:55. Foreground, no sub-agent, no modifying git command, no ticket tool. Continues `F/RESULT-BASELINE-DEFECTS-7.md` and `F/COMPILE-DEFECT-LEDGER.md` (updated in place: new section A2 with bd8-01 to bd8-18, rows O-03, O-05, O-06, O-07, O-11 rewritten, counts line added). Tools in `F/bd8-tools/` (pre-edit copies in `F/bd8-tools/orig/`), logs in `F/🗑️generated/bd8-*`. `F` is the `🪓️monolith-split` ticket folder.

## 0. Verdict

PARTLY DONE. Of the five requested buckets: bucket 1 is done for 6 of 7 structs (the seventh, `Process3dPublicationLease`, turned out to be a stale 650-line suite on deleted machinery and is recorded with a decision); bucket 2 is partly done (shared reader, `app-child-member-registry` green, `close-budget` from 0 to 10 of 31); bucket 3 is done except two launcher/load-bound tests; bucket 4 was re-classified (most hits are not defects) and the real ones were fixed or listed; bucket 5: hub oracle composition, `mcp-composition` and the flow preview test are green, the vitest gate has a recorded decision plus a worklist. No real cargo compile ran (`COMPILE ADMITTED` is not in `📓️FLEET.md`); every Rust change is static. The 30 `🧭️cause` sha pins were not touched.

## 1. Found state

`F/bd8-tools` and `bd8-*` logs did not exist. Machine load 50 to 55 throughout. `git` printed `error: daemon terminated` on most calls (fsmonitor); `-c core.fsmonitor=false` avoided it. Two findings change earlier assumptions:

* The E0451-SWEEP §5 list contained a false positive (`SetCamera`) and one case that is not a visibility drift (`Process3dPublicationLease`).
* The "125 missing TS inputs" of bd7 are mostly scanner artefacts (section 4).

## 2. Bucket 1: the seven private-struct test drifts

| # | struct | result |
|---|---|---|
| 1 | `Process3dPublicationLease` | OPEN with decision, plus a lib fix. The lib root lacked two imports since f856dff0813 (`PROCESS3D_MAXIMUM_DOMAIN_ITEMS` 17 sites, `PROCESS3D_OWNER_BYTES` 7 sites, both `pub const` of the mutations crate); added to the root. rustc analysis (extern crates stubbed in a sandbox copy, output to `/dev/null`) now shows no local-name error. The retained-laws suite cannot be mounted as it is: 5 of 14 tests drive `Process3dOwnedRetirement`, `Process3dRetirementOwner` and `process3d_retire_erased_step`, which commit 03c902fa488 (#682) deleted in favour of the framework's `OwnedValueRetirementFactory`; `close_step(1, extent)` is the old two-argument API; the file has 4 `[DEBUG]` prints and a duplicate import (E0252). Decision: mount as a unit test of the lib and rewrite the five retirement tests on `admit_owned_retirement` and grants (the laws survive, the machinery is gone by design). Needs the compiler loop, so not done blind. |
| 2 | `Intra4Neighbors` and the `H264Tests` region | FIXED (bd8-01). 582 lines moved to `🎞️h264/🧪️tests/🔬️unit`; `Intra4Neighbors` was already `pub(super)` with `pub(super)` fields, the real blocker was the crate boundary. |
| 3 | `Puzzle3dSessionState` | FIXED (bd8-02) by moving the five session laws into the session crate. Also found and fixed a production E0432: the editor's `interaction_topology` increments a counter that the session crate declared under `#[cfg(test)]`. |
| 4 | `ContentTypeRow` | FIXED (bd8-03), wire-spelling value instead of the private type. |
| 5 | ifc `Frame` | FIXED (bd8-04), unit tests inside the two model crates; cross-crate half stays. |
| 6 | `PptxDiffRecord` | FIXED (bd8-05), unit test in the model crate. |
| 7 | `SetCamera` | GONE (bd8-06), scanner false positive. |

## 3. Bucket 2: plugin-runtime and app-child-member-registry anchors

New `🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts` exports `pluginFamilyText()` (every non-test `.rs` below the plugin folder, read once, 260 ms). It reads files, not module trees, because `readModuleTreeSource` skips mounts that leave a crate folder: 195 of 536 plugin sources are unreachable from any crate root that way (for example `📜️declaration/🪆️child` is mounted by `../🪆️child` from `👁️view`). Both suites use it.

* `🔬️app-child-member-registry`: 3 pass, 0 fail (was 0 pass, 3 fail). One anchor re-pointed (`metadata: ManuallyDrop<Option<[String; 7]>>`, the file now imports `ManuallyDrop`).
* `🔬️plugin-runtime-runtime-close-budget`: 10 pass, 21 fail (was 0 pass at the first assertion of every test). Re-anchored with the same meaning: `pump_one_for_interactive_turn(cx)`, history types through `readModuleTreeSource`, `pub fn insert_prepared_entry` markers, `pub trait ChildContentRetainers`, `self.stores.store.admit_apply_batch(`.
* 67 assertions in 21 tests still fail. They are real code drift, not path drift: `crate::plugin_runtime::original_plugin_turn_grant` is now `semio_framework_plugin_reactor_app::original_plugin_turn_grant`; the receipt hand-offs live in `typed_output.*=more.*` of `plugin-exchange`; `JobOutcomeSlot`, `close_step_outcome_slot` and `step_outcome_slot_retirement_demands` have no callers. **One finding needs an owner before anyone edits an anchor: `advance_original_document_archive_close` and `original_document_archive_close_demands` have no production caller after the carve (definition and tests only), so the scheduled/mounted archive-close entrances named by the scheduled-archive test no longer exist.** The worklist per failing assertion is reproducible with `bd8-tools/diag.sh` and `bd8-tools/suggest.py`.

## 4. Bucket 4: the missing TS inputs

`bd8-tools/tsenoent.py` (bd7) re-run: 113 hits in 95 test files. Running every flagged file (`bd8-tools/runlist.sh`): 66 pass with 0 failures. Classification (`bd8-tools/enoent_classify.py`, `enoent_fixtures.py`): 60 are `existsSync(...).toBe(false)` laws (the file must NOT exist), 10 are base-URL anchors (`new URL(path, testSourceUrl.href)` where the file is never read), 6 virtual TypeScript compiler paths, 5 module-under-test base URLs (browser-bundle ×4, shell reduce; the module moved, the fixtures stayed, the tests pass unchanged; an attempt to re-point them broke them and was reverted), 1 temp-file string, 2 escapes.

Real defects fixed:

* Six library schema leaves (`♻️taxonomy-pattern-compiler-reuse/🧪️registration`, `🎟️reference-coverage-selection`, `🔤️taxonomy-leading-grapheme/🧪️registration`, `🖍️draw-source-scenario/{📨️submitted-proof,🛣️commit-route,🙈️residue-ignore}`) were deleted by d4597d0410a together with about 44 more, and the suites kept compiling them. Restored from `d4597d0410a^`; the three registration schemas had to drop the `launch*` properties that the fixtures no longer carry. Decision: restore schemas (schema-first), do not weaken the tests.
* Moved fixtures (commit 0c3e78944af, `🧫️fixtures` to `🧪️tests/🧫️fixtures`): `goal-gate` 28/28, `source-census` 35/35, dev `contribution` (3 of 4; the fourth is a Node `ERR_IMPORT_ATTRIBUTE_MISSING` in `native-build`), `caching/inference` (1 of 3, needs `SEMIO_TEST_ARTIFACT_DIR`), hub flow `surface-owners` path.
* `descriptor-verification` read `🧪️tests/🧪️tests/…`: 21 pass. `linked-codec-ownership` re-pointed to `🧩️component/🔣️.json`: 5 pass.
* Library registration convention `nx:run-commands` / `nx run` to `repo:script` / `bun --cwd … repo run` in four tests; `taxonomy-pattern-compiler-reuse` also needed `parseGeneratorPreviewProgressPolicyV1` in its harness, a property-access count for matcher callbacks (`generatorPlanning` passes the matcher), two new `loadTaxonomy` callers (`verifyTaxonomyScopes`, `verifyTaxonomyScopesClosed`) in the fixture and the test literal, and the new `noFollowDirectoryChain` / `verifyNoFollowDirectoryChain` call names: 21/21.

Still open: architect and equation `🧫️fixtures/🔬️window/🔣️.json` (never authored), and four source readers of deleted Rust files (O-05, O-07).

## 5. Bucket 3 and 5

* Catalog `🎬️host-activation` 5/5, `🧩️integration` 6/7 (fixture and schema updated, see ledger bd8-10; the seventh is the audit time budget), `🎮️session` is a vitest suite (`test.runIf`), run with `bd8-tools/vitest.sh` at `SEMIO_TEST_LEVEL=long`: 1 pass, 2 fail, both blocked by the launcher envelope (O-09).
* Hub oracle composition: 60/3 to 63/0 (ledger bd8-14).
* `mcp-composition`: the law function is called by the gis services script; run through `bd8-tools/mcp-law.ts`: `MCP-LAW OK`.
* Flow wasm test: 1 pass (ledger bd8-16); the file still has 11 other failures.
* Vitest ownership gate: registration test fixed (1 pass). Decisions (O-11): do not re-pin the 24 stale projection hashes yet (6 owners collect nothing, `cad` root and `puzzle` name are wrong, 22 `runVitest` call sites are outside the owner fixture), then register the 13 existing configs (46 to 59 owners) and fix the 7 call sites that point at nothing.

## 6. Verification run (all with `ulimit -n 65536`, `nice -n 10`)

| check | result |
|---|---|
| `bd8-tools/h264_oracle.sh --test` and lib (sandbox copy of the h264 module, image crate stubbed, `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`) | no error except `couldn't create a temp dir … /dev/rmeta…` (this machine refuses `/dev` temp dirs after analysis ran); negative control with the original encoder file: 28 errors (E0425 ×16, E0433 ×12) |
| process3d host lib analysis, extern crates removed in a sandbox copy | 478 errors before (17+6+1 local-name), no local-name error after; the rest is unresolved external crates |
| `rustfmt +nightly-2026-07-20 --check --config skip_children=true` on the new and edited `.rs` | 0 parse errors (remaining hunks are pre-existing formatting) |
| `bun test` app-child-member-registry | 3 pass, 0 fail |
| `bun test` plugin-runtime close-budget | 10 pass, 21 fail |
| `bun test` catalog host-activation / integration | 5/0 and 6/1 |
| `vitest` catalog session (long) | 1 pass, 2 fail (launcher envelope) |
| `bun test` library pattern-compiler-reuse / reference-coverage / leading-grapheme | 21/0, 5/0, 10/0 |
| `bun test` acceptance goal-gate / source-census | 28/0, 35/0 |
| `bun test` descriptor-verification / linked-codec-ownership | 21/0, 5/0 |
| `bun test` hub oracle composition | 63 pass, 0 fail |
| `bun test` flow wasm `-t "original preview payload forwards"` | 1 pass |
| `bun bd8-tools/mcp-law.ts` | `MCP-LAW OK` |
| `bun test` vitest gate `-t "registers the gate"` | 1 pass (the other two still fail, O-11) |
| `bun test` dev contribution / caching inference / flow source-contract | 3 pass 1 fail / 1 pass 2 fail / 0 pass 1 fail (other causes, listed) |

Not run: any cargo check or cargo test (no admission). Compile queue for the next admission, in addition to the bd7 command:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- test --no-run --keep-going --message-format=short -p semio-s-artifact-remodel-remodeling-video-h264 -p semio-s-artifact-remodel-remodeling-video -p semio-s-artifact-puzzle-3d-editor-session -p semio-s-artifact-puzzle-3d-editor -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-ifc-2x3-base-model -p semio-s-artifact-stdio-ifc-4-model -p semio-s-artifact-stdio-ifc -p semio-s-artifact-stdio-pptx-base-model -p semio-s-artifact-stdio-pptx -p semio-s-artifact-process-process3d-host
```

Risks of the Rust edits that only a compiler settles: the h264 `pub(super)` widening of `BitWriter` (a struct with a `pub use encoder::*` re-export), the session crate's `pub(super)` methods taking `pub(super)` types, the removal of `cfg(test)` from `geometry_jsons`'s counter increment, and the `ifc` and `pptx` Cargo dev-dependency additions (`Cargo.lock` is untouched).

## 7. Files edited

Rust: `✏️s/🔌️plugins/📸️remodel/…/🎥️video/{🧪️tests/🔬️unit/🦀️.rs, 🎞️h264/{🦀️.rs,✍️encoder,🍰️slice-data,🧼️deblocking-and-output,🧮️reconstruction,📖️bitstream-and-headers}/🦀️.rs}`, new `…/🎞️h264/🧪️tests/🔬️unit/🦀️.rs`; `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/{🎟️session/🦀️.rs, 🎟️session/🗃️session-registry/🦀️.rs, 🧪️tests/🔬️budgets/🦀️.rs, 🧪️tests/🔬️budgets/🎟️session-and-step-budget-laws/🦀️.rs}`, new `…/🎟️session/🧪️tests/🔬️unit/🦀️.rs`; docx `…/🧭️edit-rules/🧪️tests/🦀️.rs`; ifc `{4️⃣4,🔖️2x3}/…/📸️snapshot/🚦️native/{🦀️.rs,🧪️tests/💰️backing/🦀️.rs}`, `🧪️tests/🔬️sqlite-snapshots/🦀️.rs`, new `🔖️2x3/…/📸️snapshot/🧪️tests/💰️backing/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`; pptx `…/🚪️io/📝️text/🔺️diff/{🦀️.rs,🧪️tests/🦀️.rs (new)}`, `🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs`, `🧬️schema/📦️packages/🦀️rust/Cargo.toml`; process3d host `🧰️owned/🦀️.rs`.

TypeScript and JSON: new `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts`; plugin `🧪️tests/{🔬️plugin-runtime-runtime-close-budget,🔬️app-child-member-registry}/🟦️.ts`, `🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts`; catalog `🧪️tests/{🎬️host-activation,🧩️integration,🎮️session}/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json`, `🧬️schema/🔣️.json`; library `🧪️tests/{♻️taxonomy-pattern-compiler-reuse,🎟️reference-coverage-selection,🔤️taxonomy-leading-grapheme,🎚️vitest-configuration-ownership}/🟦️.ts`, `🧪️tests/🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🔣️.json`, six restored `🧬️schema/…/🔣️.json`, `⚡️caching/📥️inference/🧪️tests/🟦️.ts`, `🧪️test/🎯️acceptance/🧪️tests/{🎯️goal-gate,🧮️source-census}/🟦️.ts`; `🧑‍💻dev/🧪️tests/🧩️contribution/🟦️.ts`; `📇️registry/🛂️descriptor-verification/🧪️tests/🟦️.ts`; flow editor `🧪️tests/🔬️source-contract/🟦️.ts`; flow wasm `🧪️tests/🔬️unit/🟦️.ts`; hub `🧪️tests/⛓️linked-codec-ownership/🟦️.ts`, `🔮️oracles/🧪️tests/{🧩️composition/🟦️.ts,🧫️fixtures/🧩️composition/🔣️.json}`; gis services `🧪️tests/🧫️fixtures/🧷️untrusted-content/🔣️.json`.

Tools: `F/bd8-tools/{privscan.py, h264_relocate.py, h264_oracle.sh, anchors.py, suggest.py, failsite.py, diag.sh, softexpect.ts, vitest.sh, runlist.sh, tsenoent.py, enoent_classify.py, enoent_fixtures.py, mcp-law.ts, catalog-scope.ts, catalog-audit.ts, reach.ts, family-probe.ts, union-time.ts}`. Data: `F/🗑️generated/bd8-*`. Temporary `bd8-diag.ts` copies next to tests were deleted after each run.

## 8. Hand-offs

1. Process3d host owner: O-03 item 1 (decision in §2).
2. Plugin split owner: O-05 (start with the archive-close entrance question in §3), plus the three deleted-Rust readers in O-07.
3. Library owner: vitest gate worklist (O-11), `workspace-contract` (49 failing in the aggregate suite, none from the restored schemas), `ui locale` (34 failing), `pack decode source` (Ajv draft 2020 meta-schema).
4. Whoever holds launcher admission: O-06 and O-09, then the compile queue in §6.
5. Coordinator: a 70-target, 156-variant catalog fixture now exists (`🧪️tests/🧫️fixtures/🔣️.json`); anything that still assumes two installed targets is stale.

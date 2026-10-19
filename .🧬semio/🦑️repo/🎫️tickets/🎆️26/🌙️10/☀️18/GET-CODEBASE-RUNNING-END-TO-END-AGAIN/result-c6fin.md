# Result c6fin (census 6 packages P36, P37, P43, P44, P49, P59)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo, no `bun repo`, no nx, no bazel. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`, `C` = this folder, `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Tools: `F/c6fin-tools/` (snapshots `orig/<tag>.rs` with `orig/SHA256`, specs `specs/<tag>.py`, chunk tables `specs5.py`, tag table `files.tsv`, crate table `crates.json`). Scratch outside the repo: `/Users/ueli/.cache/c6fin/` (stages, sandboxes, logs). The predecessor's 11 snapshots were verified current by sha256 against the live files before splitting (all equal, `frw` is the 2,733-line text).

## 0. Verdict

**DONE for 11 of 11 files (static evidence; nothing compiled).** Every resulting Rust file is at most 519 lines (target 600). No function above 1,000 lines exists in these files. 62 new folders (61 children plus the `🧪️artifact-app-laws` shell), no glob re-export, no compatibility re-export, explicit `pub use child::{names}` / `use child::{names}` lists, children begin with `use super::*;` (as census 5 and `c6plug2`).

| tag | P | file | before | root after | children | largest child | method |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| fpa | P36 | `PL/⚛️reactor/🩹️patches/🦀️.rs` | 1,361 | 57 | 5 | 352 | `#[path]` modules, `impl PatchTracker` re-opened 3 times |
| fpr | P36 | `PL/🖥️host/🔀️post-turn-relay/🦀️.rs` | 2,392 | 98 | 10 | 371 | `#[path]` modules, `impl GuestRelayMountedRegistry` re-opened once, regions re-opened per child |
| fpt | P36 | `PL/🧪️testing/🦀️.rs` | 2,386 | 174 | 1 shell + 7 | 439 | inline `pub mod artifact_app_laws` became the file module `🧪️artifact-app-laws` (shell 70 lines) with 7 children |
| fhe | P36 | `PL/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` | 2,003 | 200 | 8 | 321 | `#[path]` modules; the 3 `#[macro_export]` macros and the mounts stay in the root |
| frr | P36 | `PL/🪟️window/🎚️config/📥️retained/🦀️.rs` | 1,596 | 36 | 4 | 512 | `#[path]` modules, `nest`, `rebase_super` (`super::registry::fits` became `super::super::registry::fits`, `pub(super)` became `pub(in super::super)`) |
| frw | P37 | `…/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` | 2,733 | 10 | 8 | 451 | `include!` fragments (flat test root, 60 `#[test]`) |
| fan | P43 | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/…/✳️any/✏️editor/🦀️.rs` | 1,257 | 198 | 6 | 313 | `#[path]` modules |
| far | P44 | `✏️s/🔌️plugins/🏛️architect/…/🌐️context-requirements/🧪️tests/🔬️unit/🦀️.rs` | 1,018 | 24 | 3 | 375 | `include!` fragments (16 `async_test`) |
| fme | P49 | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/…/✏️editor/🦀️.rs` | 1,464 | 169 | 5 | 460 | `#[path]` modules |
| fmg | P49 | `…/➗️equation/…/🕸️graph/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` | 1,104 | 5 | 3 | 450 | `include!` fragments (9 inline `mod` blocks, 60 tests) |
| fwr | P59 | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/…/✏️editor/🦀️.rs` | 1,190 | 185 | 2 | 519 | `#[path]` modules |

Decisions worth knowing

- `fwr` has only 2 children on purpose: 6 children pushed the crate test closure of `semio-s-artifact-writer-writer-tests-2` to 35 files (1.094 of the 32-file gate); 3 children gave exactly 32 (1.000); 2 children give 31 (0.969).
- Folder names for `far` (`🏞️site-and-needs`, `🧭️flow-and-growth`, `⚖️risks-and-costs`) are short because the parent directory is already 206 bytes (23 bytes left). `fmg` leaves 46 bytes. The wastebasket emoji is avoided (the repo reserves `🗑️generated`).
- `fpt`: 13 lines in the inline module were not indented (heads of `fn`/`struct`/`use`); a spec `pre` edit indents them so the body dedents cleanly (indentation only, `norm_line` strips it). `rebase_super` re-bases the one `use super::{…}` inside a function body.
- `fwr`: `}//#endregion 🧵️RetainedCommands` shared one line; `pre` puts the marker on its own line (declared `expect_missing`, `pre_extra`).
- `fan`, `fme`, `fwr`: empty `Interaction` / `TaxonomyMounts` regions are kept in the root (`keep_empty`). The `app_commands!` invocation, the `unit_tests` mount, the taxonomy mounts and the inline `artifact_items_N` module stay in the root.
- `frr`: one `#[cfg(test)]` is added on the root import of `TypedWindowConfigPackLoad` (used only by the test-only method `candidate_actor_for_test`; without it the lib build warns, found by the oracle).
- Visibility: re-exports of `pub(crate)` / `pub(super)` names are generated from usage in the family files too (`external_prod`, new): `frr` re-exports `begin_/commit_typed_window_config_pack_load` for `🗄️store-owners`, `fme` re-exports `EQUATION_MAX_DELETE_IDS` / `EQUATION_MAX_TEXT_BYTES`. The oracle caught both before they were fixed.
- Pre-existing defect fixed on the way: the patches test child `🏋️real-size-surfaces-and-producer-shell-denials` included `…/🎬️media-slots/🎟️collect/🧪️tests/🧫️fixtures/🔣️.json`, which a peer moved (staged `D`) to `📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🧊️wgpu/🎬️media-slots/🎟️collect/🔣️.json` (same sha256 as HEAD). The `include_str!` literal now points there; the crate test closure of `semio-framework-plugin-reactor-patches` reads again (24 files, 0.826).

## 1. Tools

`F/c6fin-tools/` was rebuilt from the newest set (`C/c6plug2-tools`: explicit re-export lists, test-only `#[cfg(test)]` re-exports, general `oracle_c6p2.py` that resolves the owning crate itself) plus pieces of `C/c6ui-tools` (`vmod.py` / `defscan.py` with inner-module support). Changes made to the copy:

- `modsplit.py`: outer layer skipped when the spec has no outer children (inline-module-only split), literal-line-preserving dedent (`rslex`), `pub(in super::super)` kept out of the re-base, `exist_ok` for the inner folder, new spec key `external_prod` (production consumers that decide which `pub(crate)` / `pub(super)` names the root must re-export).
- `specs/lib.py`: `family_prod(file, up, exclude)` (production Rust files of the family without the new children and without `🧪️tests`).
- `innerlist.py` (item lister of an inline module), `resolve_hits.py` (relative-path reader census over every `.rs` string literal, resolving against each ancestor), `census-fin.ts` + `crates.json` (canonical `readRustSourceClosureV1` over each owning crate), `newdirs.txt`, `newnames.txt`.

## 2. Evidence (all on the live tree after the install)

| Check | Result |
| --- | --- |
| `modsplit.py lossless` (8 module splits) | `LOSSLESS OK` for fpa, fpr, fpt, fhe, frr, fan, fme, fwr: `missing 0`; extras are only child docstrings, `use super::*;`, mount/`mod`/`use` wiring, region markers, the declared impl-split `}` + header pairs (fpa 3, fpr 1), the declared `fpt` / `fwr` pre-edit lines and the one `frr` `#[cfg(test)]` |
| `chk.py live frw far fmg` (3 fragment splits) | `CHK OK`: `gone=0`, `extra(non-marker)=0`, `#[test]` multiset 60 to 60, 16 to 16, 60 to 60, include literals equal as absolute paths and existing |
| `vmod.py live` (8 module splits) | `VMOD OK`: each child mounted once, docstring emoji equals folder emoji, unique among siblings, no clash with existing siblings, repo-relative paths at most 240 bytes (longest exactly 240, absolute 190 chars), brace depth 0, rustfmt parses every file, `#[test]` multisets equal, no `[DEBUG]` |
| `defscan.py`, `ambig.py live`, `modclash.py` | 0 cross-file private leftovers (the two `fpt` reports are trait-impl methods of `ArtifactViewer for BoundedViewerFixture`, not items), `AMBIG OK`, `modclash OK` |
| `oracle_c6p2.py <tag>` (rustc `+nightly-2026-07-20 --emit=metadata`, original restored in sandbox A vs live tree in sandbox B, cfgs lib, test, lib+testkit, test+testkit) | error multisets equal in every group and cfg: fpa 23/201, fpr 293/435/300/435, fpt 32/868, fhe 277/257, frr 230/319, frw 12 (labels only), fan 380/863, far 6/1345, fme 541/823, fmg 213/747, fwr 699/1030. Warnings equal except sandbox artifacts: `fhe` root `unused import acceptance_meta` (used inside `with_authoring_identity!( … )` in the reload child, macro unresolvable in the sandbox), `fan` root `unused import animate_presentation_retained_contract` (used inside `bounded_first_step_tool_proofs!`), `fpa` / `frw`: the only difference is the file path inside a `couldn't read` message (`orig` vs `new` sandbox, and the fragment folder for `frw`: the same 12 fixture reads fail on both sides) |
| `bun rgi.mjs newnames.txt` / `bun tax.ts newdirs.txt` | 62 names RGI (`bad=0`), 62 of 62 resolve in the live taxonomy |
| Source-boundary gate, `bun census-fin.ts` (canonical `readRustSourceClosureV1`, lib / test ratio of 32 files, 262,144 bytes, 4,000 lines) | fpa 0.336 / 0.826, fpr 0.616 / 0.897, fpt 0.040 / 0.758, fhe 0.544 / 0.617, frr 0.755 / 0.860, frw 0.031 / 0.865, fan 0.432 / 0.844, far 0.063 / 0.878, fme 0.844 / 0.938, fmg 0.500 / 0.906, fwr 0.750 / 0.969; all inside the gate |
| Formatting debt (`fmt.py`) | roots grow 2 to 8 rustfmt hunks from the generated `use child::{…}` lists; no parse failure |

Not run: the typeck-level union-sandbox oracle (`pos`, privacy E0616/E0624) and any cargo build. The patched oracle cannot see type-level privacy when external crates are unresolved; widening is conservative by name (`widen2.py`, 0 leftovers in `defscan.py`).

## 3. Source-text readers re-pointed

All edits assert an exact count before writing. Census: `readers2.py` (tails) and `resolve_hits.py` (every `.rs` literal resolved against the file and every ancestor).

| Reader | Change |
| --- | --- |
| `PL/🖥️host/{⏱️deadline,♻️publication,🎟️context/🤝️admission,🔔️wake}/🧪️tests/🟦️.ts` | the predicate `(s.includes("guest-runtime")\|\|s.includes("routers"))` also names `post-turn-relay` (module tree); in `🔔️wake` the `pump_close` slice anchor `"    fn pump_close("` became `"fn pump_close("` because the widening made the method `pub(super) fn` |
| `…/📺️renderer/…/🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts` | two `readFileSync` of the patches root became `readModuleTreeSource` |
| `PL/⚛️reactor/🩹️patches/🔁️retry/🧪️tests/🟦️.ts`, `PL/🏇️mounted-owner/🧪️tests/🟦️.ts`, `PL/⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🟦️.ts` | patches root / testing root read as module tree (import added where missing) |
| `PL/🧪️testing/🎟️caller/🧪️tests/🟦️.ts` | testing root and history-edit-acceptance root read as module trees (three sites); the end anchor `"\n    ///"` became `"\n///"` (the inner module dedented) |
| `PL/🧪️tests/🧪️history-edit-acceptance/🧪️tests/{📥️action,🔎️array-controls,🔎️ready-candidate}/🟦️.ts` | root read as module tree; `📋️inventory` unchanged (the macros stay in the root) |
| `✏️s/🔌️plugins/➗️mathematical/🧪️tests/📣️publication-authority/{🟦️.ts,…/🧫️fixtures/…/🔣️.json}` | fixture gains `mounted` (5 children), verifier joins the root and the mounted children |
| `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🧫️fixtures/🧵️retained-native/🧾️refusal.json` | equation producer path now `…/✏️editor/📬️store-preparation/🦀️.rs` (the `fn advance`..`fn checkpoint` slices live there) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` | `writerEditor` uses `policyReadModuleTreeSource` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` (`POLICY_SPLIT_SOURCE_ROOTS`) | 4 roots added: the animate, mathematical and writer editors and the reactor patches (the tool-job census treats a root as the whole tree and skips its children; without the entry each child would be a census row of its own) |
| Unchanged on purpose | `callers.json` of the composition admission test (module tree), `completion` fixture (module tree), `PL/🧪️tests/📦️units` closure (follows `#[path]`), `interactivity-audit` (`policyReadRustPolicySource`, see risks) |

Bun checks (run with `./path`; a bare path is a 7.5 M-file filter scan that crashed bun): `🧪️testing/🎟️caller` 8 pass, `history-edit-acceptance` action, array-controls, ready-candidate 4 pass, host `⏱️ ♻️ 🎟️ 🔔️` 8 pass, patches retry and failure-debit 14 pass, stdio editing: the equation producer slice passes. Failures that remain are not caused by this slice (verified against the snapshots with python twins of the assertions where the test also reads the old text): media-slots collect (3 fail on `⏭️cursor`, `instance` and `UiTextOriginalCopy`, the patches assertions are equal on the snapshot and the tree), `📋️inventory` (BIM registration regexp), stdio editing (fem 2d editor root, peer split), mounted-owner (5 tests need the native result artifact), admission (missing package `smol-toml`), mathematical publication-authority verifier (stale paths; the anchor `base.return_to_registry()` is absent in the snapshot and in the tree alike).

## 4. Open risks

- No cargo. Residual risks are those of census 5: a private method or field read across the new boundary that the name scan missed (0 reported), `use … as _` trait imports reaching children through the glob, and macro-only uses that the sandbox cannot see (`with_authoring_identity!`, `bounded_first_step_tool_proofs!`, `app_commands!`).
- `interactivity-audit` / `interactivity-live-reconcile` read the patches root through `policyReadRustPolicySource`; that function already throws `conditional external module original_retry_laws is not canonical` for the root mount `🔁️retry/🧪️tests/🦀️.rs` (present in HEAD). Unchanged by this slice; the root is now also a declared split root.
- `fwr` test closure 31 of 32 files, `fme` 30 of 32: the next split in those crates must not add more than one file.
- The installer asserts live equals snapshot before writing; the assertion held for all 11 files (no peer edited them during the slice).

## 5. Crates to compile

`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests` (add `--features testkit` for the host relay, `--features artifact-app-testing` for the two testing crates, `--features original-actor-context-testing` for the patches crate):

1. `semio-framework-plugin-reactor-patches`
2. `semio-framework-plugin-host-post-turn-relay`
3. `semio-framework-plugin-testing-laws`
4. `semio-framework-plugin-testing-acceptance`
5. `semio-framework-plugin-window-config`
6. `semio-framework-os-renderer-wgpu-runtime-mailbox-tests`
7. `semio-s-artifact-animate-presentation-animate`
8. `semio-s-artifact-architect-program-tests-3`
9. `semio-s-artifact-mathematical-equation-editor`
10. `semio-s-artifact-mathematical-equation-mutations`
11. `semio-s-artifact-writer-writer-tests-2`

Downstream (public API unchanged, all names re-exported): the plugin crates that use `semio_framework_plugin_testing_laws::artifact_app_laws::*` and `semio_framework_plugin_testing_acceptance` macros; check them in the same window.

## 6. Files

- Edited roots (11): the files of the table in section 0 (snapshots in `F/c6fin-tools/orig/`). Created: 62 folders `<emoji>️<kebab>/🦀️.rs` (listed in `F/c6fin-tools/newdirs.txt`).
- Edited readers and registries: section 3 (14 TS / JSON files) and one Rust `include_str!` literal (patches test child).
- Tools kept in `F/c6fin-tools/`; stages, sandboxes and oracle logs in `/Users/ueli/.cache/c6fin/` (outside the repo). No `[DEBUG]` log was added; tool-generated temp copies (`fmt-tmp`, scratch tests) were removed.

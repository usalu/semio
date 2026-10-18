# Result CENSUS5-PAI (prefix `pai`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo/nx/`bun repo`. `F` = this folder. `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Tools: `F/pai-tools/` (`spec.py` role table, `gen.py` stager, `verify.py` lossless proof, `methods.py` method table, `budget.py`, census/oracle launchers copied from `c5c8-tools`). Logs: `F/🗑️generated/pai-*`. Scratch: `/Users/ueli/.cache/pai/` (`stage`, `pos-work`, `nm`, `exp`).

## 0. Verdict

`PL/🧵️instance/🔌️plugin-app-impl/🦀️.rs` (the single `impl PluginApp for VcsArtifactApp`): **1,728 lines to 931 lines (root) + 8 role children (max 296)**, installed. Static evidence only, the compile is the coordinator's.

### Design decision: inherent methods, trait kept whole
- Implementors of `PluginApp` (full-repo grep, tickets/hub scratch excluded): exactly one, `VcsArtifactApp`. The trait (`PL/⚛️reactor/🔌️app/🦀️.rs`, about 120 methods) is named in 264 files of `🧰️framework`; a role split of the trait would change every bound and import with no compile available, so the inherent-impl route (the task's preferred route) was taken.
- Each role child holds one `impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M>` block with the moved methods verbatim, head widened to `pub(super)`. The trait impl keeps a one-line forwarder `fn x(<same signature>) { self.x(<same args>) }` (plus `.await` for `async fn`) at the original position.
- Forwarder and inherent method share the name on purpose: inside `plugin_app_impl` the accessible inherent method wins, outside it the `pub(super)` method is inaccessible and rustc's probe drops it, so every other caller still reaches the trait. This is not recursion. Proven on a model with `deny(warnings, unconditional_recursion)`: `pai-tools/private-shadow-experiment.rs` (compiled and run with `rustc +nightly-2026-07-20`, prints `ok`; trait + same-name `pub(super)` inherent in a child, forwarders including `async` and a `mut` parameter, calls from a sibling module resolve through the trait). A second probe confirmed `use super::*;` children keep root-only imports "used".
- Why not every method: the crate sat at 0.92 (lib) and 0.96 (test) of the 262,144-byte source gate before the split; every forwarder duplicates its signature. Only the 22 methods of 20 lines or more were moved (the 61-line terminal conjunction, render and measures, transactions, media, archive load, ...). The 100 smaller methods stay inline in the trait impl.
- `advance_typed_operation_publication` (22 lines) stays in the root on purpose: `🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts` slices from its first occurrence to `fn take_typed_operation_preparation_receipt`, which must stay in the same file.

## 1. Files

| child folder (under `PL/🧵️instance/🔌️plugin-app-impl/`) | module | methods moved | lines |
| --- | --- | --- | ---: |
| `🔒️close-terminal` | `close_terminal` | `close_terminal_is_empty` | 67 |
| `🧾️typed-operation-results` | `typed_operation_results` | `tool_public_contracts`, `acknowledge_typed_operation_result`, `take_typed_operation_completion` | 80 |
| `🕹️ephemeral-interaction` | `ephemeral_interaction` | `begin_local_interaction_query`, `publish_local_interaction_query_reply`, `ephemeral_snapshot` | 89 |
| `📨️action-dispatch` | `action_dispatch` | `handle_action_invocation`, `resume_task_emit` | 56 |
| `🔀️transactions` | `transactions` | `transaction_prepare`, `transaction_commit` | 114 |
| `🗃️document-load` | `document_load` | `hydrate_pure_head`, `begin_document_archive_load` | 71 |
| `🖼️render-surface` | `render_surface` | `render`, `window_engagements`, `window_measures`, `tool_measures`, `pending_effects`, `context_menu` | 295 |
| `🎞️media-transfer` | `media_transfer` | `produce_media`, `consume_media`, `retain_media_export_result` | 133 |

Root `🦀️.rs`: 1,728 to 931 lines, 136,740 to 72,324 bytes; 8 `#[path]`/`mod` pairs added after the imports. Every child is `//! <emoji> doc See [`super`].` + `use super::*;` + the inherent block. Emojis RGI (`rgi.mjs` bad=0), all used elsewhere in the repo taxonomy, unique among the children, different from the root's 🔌; `tax.ts`: 8 dirs, 0 unresolved (`crate-module-children`). No folder name clash with any root-level name (the module names occur nowhere in the original text). Longest path well under 240 bytes.

Edited besides the split:
- `PL/🧵️instance/🧪️tests/🧫️fixtures/🔣️.json`: the 8 children appended to `composerFiles`, because the law "no inherent impl of VcsArtifactApp exists outside the composer files" in `PL/🧵️instance/🧪️tests/🟦️.ts` allows inherent impls only in listed composer files.
- `PL/🎮️command/🎟️lease/🧪️tests/🟦️.ts`: the 8 children added to the tree-sitter ERROR-free parse list next to the root.
- `PL/⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🟦️.ts` (law "addressed action dispatch forwards ..."): `end` of the `handle_action_invocation` slice changed from `indexOf("async fn handle_response_action(")` to `indexOf("\n    }\n", start)`, because the first `async fn handle_action_invocation(` of the subtree text is now the real body in the child and the old end marker lives in the root, which would have swelled the slice over unrelated code and weakened the three `not.toContain` assertions.

## 2. Evidence

| check | result |
| --- | --- |
| lossless | `verify.py live`: replacing every forwarder of the live root by its child block (head `pub(super) ` stripped) and removing the mount block reproduces the snapshot `pai.orig.rs` **byte for byte** (22 methods, 1,729 lines); child frames exact; forwarder signature equals the original signature (whitespace and `mut` of parameters normalised), forwarder argument list equals the parameter list, `.await` iff `async`; rustfmt parse (nightly-2026-07-20, `skip_children`) of root and 8 children: ok; no `[DEBUG]`; path lengths ok. Log `pai-verify-live.log` |
| moved bodies | no `super::`, `self::` or `<Self as ...>` path in any moved body; one `crate::OriginalRenderResult` (absolute); no moved body mentions a child module name |
| source-boundary census (lib / test closure) | before 9 / 10 files, 241,072 / 250,865 bytes, 3,057 / 3,171 active lines; after **17 / 18 files, 249,098 / 258,891 bytes (0.950 / 0.988 of 262,144), 3,173 / 3,287 active lines (0.82 of 4,000)**; gate: 32 files, 262,144 bytes, 4,000 lines. Log `pai-census.log` |
| typeck oracle (`posrun.py`, labels `pre` before, `post` after, cfg lib and test) | canary E0308, E0616, E0624 reached both times; resolve 1/1, other 2/2. `posdiff.py`: lib 2,276 to 2,274 rows, test 2,408 to 2,406; only the `close_terminal_is_empty` and `maintenance_terminal_is_empty` rows change, all explained by the oracle's stub `PluginApp` (it has no methods): calls inside `plugin_app_impl` now resolve to the inherent method (2 rows gone), the 2 calls of `📥️staging/🔒️close-ladder` turn from E0599 to E0624 because the stub trait offers no fallback (the real trait declares `close_terminal_is_empty`, see the model above). Logs `pai-pos-pre.log`, `pai-pos-post.log`, `pai-posdiff.log` |
| TS laws that read the file (scratch `node_modules` with `graphlib`, `ajv-formats`, `fast-json-patch`, `lodash` added to the `c5c8` set; `pai-tools/bt.sh`) | instance tests 4 pass (including the inherent-impl law, also re-run natively as a python emulation: 0 offenders, 0 missing composer files); typed-snapshot 8 pass; media turn 9 pass; pending preview 4 pass; refusal 5 pass; document-archive-load-legs 1 pass; frontier 7 pass; composition mounted owner 1 pass; lease 3 pass; failure-debit 13 pass; mounted-owner 13 pass and 5 fail on the missing native result artifacts (`SEMIO_MOUNTED_OWNER_RESULTS`, environment, unrelated) |

## 3. Source-text readers

`readers.py pai` found three path readers (lease, composer fixture, typed-snapshot, all handled). The other path readers (`media/⚠️turn`, `session/🎟️pending`, `session/🫴️refusal`, `🧪️tests/🧾️document-archive-load-legs`, `retained-command/.../♻️frontier`, composition `📦️owner`, `🏇️mounted-owner`) either join `familyText()`/`pluginSubtreeText(...)` (tree walks that pick up the new `#[path]` children automatically) or assert only on text that stayed in the root, so they needed no edit; all pass (section 2). Name-based anchors for the 22 moved names were searched in all tracked ts/tsx/mjs/py/json files: the only one affected is the failure-debit slice above; `🔒️lifecycle` `lastIndexOf("fn acknowledge_typed_operation_result(&mut self")` runs on the plugin facade text (not touched) and would still find the real body last (root forwarder sorts before the `🧾️` child).

## 4. Open risks

1. **Compile not run.** The model experiment and the oracles do not prove trait coherence or the real privacy-aware method probe on the full crate; the coordinator compile is the proof. Expected failure classes if any: E0592 (duplicate inherent name; none of the 22 names exists in another inherent impl of the crate, grepped), E0624 from a sibling module (none expected, siblings use the trait).
2. **Gate headroom is thin**: the test closure of `semio-framework-plugin-instance-app` is 3,253 bytes under the 262,144-byte gate (0.988). Any growth of that crate needs a cut elsewhere in it first. Moving more methods behind forwarders costs about 300 bytes each, which is why only 22 moved.
3. Forwarders and inherent methods share names (a reader may mistake them for recursion); documented here and verified by the model, not by a comment (AGENTS forbids comments inside definitions).
4. Formatting debt: the file was never rustfmt-clean (compact one-line style); forwarders follow that style.
5. Foreign, unchanged: the 5 mounted-owner failures (native results), the `tool-job-coverage` family of stale laws (previous reports).

## 5. Crates the coordinator must compile

`cargo check -p <crate> --lib --tests` for:
- `semio-framework-plugin-instance-app` (the split crate)
- `semio-framework-plugin` (facade, depends on it)
- `semio-framework-plugin-builder` and `semio-framework-plugin-testing-laws` (direct dependents that call `PluginApp` methods on `VcsArtifactApp`)

## 6. Files

- Created: 8 child files `PL/🧵️instance/🔌️plugin-app-impl/<emoji>️<kebab>/🦀️.rs`, `F/pai-tools/`, `F/🗑️generated/pai-*`, this file.
- Edited: `PL/🧵️instance/🔌️plugin-app-impl/🦀️.rs`, the fixture `PL/🧵️instance/🧪️tests/🧫️fixtures/🔣️.json`, `PL/🎮️command/🎟️lease/🧪️tests/🟦️.ts`, `PL/⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🟦️.ts`.
- Untouched: the trait file, every other `plugin-app-impl` band slice of the sibling crates, `RESULT-CENSUS5-C8.md` and `MONOLITH-CENSUS-5.md` (the coordinator should now mark `pai` as split).

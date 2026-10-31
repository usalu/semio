# 🔎️ 1030 Policy2 Audit (lane policy2)

STATUS: done (read-only: no edits, no builds, no sub-agents, no git writes).

Scope: files under `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand` with mtime after `2026-10-30 06:00` (find -newermt, node_modules/target/dist/bazel-out/generated pruned). Files scanned: **2451**.
Script: `1030-tools/policy2/policy2.py`. Full findings TSV: `🗑️generated/1030/policy2-findings.tsv`.

Method: Rust/TS sources go through a char-level scanner (strings, raw strings, char literals and comments masked; brace stack classifies fn / impl / type / test blocks). Python uses indentation for def bodies. Other text files get line rules only. Heuristics, so treat counts as candidates that need a line read before fixing.

## Counts per rule

| rule | AGENTS.md rule | findings | new vs HEAD | files |
|---|---|---:|---:|---:|
| `debug` | `[DEBUG] ` / `dbg1030` / `DBG1030` leftovers | 78 | 7 | 75 |
| `allow` | no new `#[allow(` | 912 | 2 | 636 |
| `unwrap_expect` | no `.unwrap()` / `.expect(` in non-test Rust | 1497 | 400 | 314 |
| `unsafe` | `unsafe` needs a SAFETY comment or `# Safety` docstring (9 lines window) | 270 | 71 | 106 |
| `doc_no_emoji` | docstrings start with a unique fitting emoji | 4 | 3 | 4 |
| `doc_dup_emoji` | leading emoji unique per file (extra occurrences) | 610 | 36 | 212 |
| `comment_fn` | no comments inside fn bodies (SAFETY comments exempt) | 6 | 0 | 4 |
| `comment_def` | no comments inside impl / trait / struct / enum bodies | 22 | 0 | 1 |
| `compat_ident` | no compat / legacy / shim / adapter / fallback identifiers in code | 291 | 45 | 65 |
| `compat_path` | same, in directory or file names | 0 | 0 | 0 |
| `test_threads` | no `--test-threads=1` in scripts | 1 | 0 | 1 |
| `ui_literal` | no hard-coded English/German UI literals in TS/TSX (candidates) | 0 | 0 | 0 |
| `json_import` | `with {type:"json"}` only on `.json` targets | 0 | 0 | 0 |
| **total** | | **3691** | **564** | |

## Findings per owner lane guess (top 25)

| owner lane guess | findings |
|---|---:|
| `stdio` | 777 |
| `value` | 449 |
| `plugin` | 392 |
| `draw` | 316 |
| `2d` | 248 |
| `puzzle` | 231 |
| `fem` | 222 |
| `procedural` | 137 |
| `job` | 100 |
| `flow` | 80 |
| `library` | 76 |
| `pack` | 69 |
| `vcs` | 69 |
| `renderer` | 69 |
| `store` | 64 |
| `layout` | 50 |
| `gis` | 46 |
| `pixels` | 37 |
| `energy` | 33 |
| `mit-bestand/demonstrator` | 32 |
| `raster` | 26 |
| `tool-machine` | 19 |
| `replication` | 18 |
| `ui` | 17 |
| `space` | 16 |

## Top 60 examples (round-robin across rules, priority order)

| # | file:line | rule | new vs HEAD | evidence | owner lane guess |
|---:|---|---|---|---|---|
| 1 | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/📦️owned/🧪️tests/🦀️.rs:23` | `debug` | yes | al ownership observations");*ordinal+=1;println!("[DEBUG] [owned-layout-turn] {}",serde_json::json!({"name":name,"ordinal":*ordinal,"op":op,"demand | `layout` |
| 2 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📷️raster/🧪️tests/🔬️unit/🦀️.rs:8` | `allow` | yes | #[allow(unused_imports)] use crate::{booleans::*,identity::*,paint::*,picking::*,placement::*,prep | `draw` |
| 3 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🪞️dsl-mirror/🦀️.rs:49` | `unwrap_expect` | yes | lope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id"); | `procedural` |
| 4 | `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🧩️context/🪄️apply/🏪️workspace/🦀️.rs:14` | `unsafe` | yes | guous(&mut self.slots,maximum)?{return Ok(false);}unsafe{self.slots.set_len(maximum);}self.gap_count=maximum;Ok(true)} | `2d` |
| 5 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📏️grow/🟦️.ts:89` | `doc_no_emoji` | yes | The processes (id and full executable path) that dissent from a plain unmount of the row. | `library` |
| 6 | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🌐️locale/🟦️.ts:33` | `doc_dup_emoji` | yes | 🏷 also leads line 13 | `locale` |
| 7 | `♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx:129` | `comment_fn` | no | //#endregion 🏛️FundedByZukunftBauFooterItem | `mit-bestand/demonstrator` |
| 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🧰️kernel-trait-and-engine/🦀️.rs:9` | `comment_def` | no | // #region Primitives | `flow` |
| 9 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧊️preview-geometry/🦀️.rs:425` | `compat_ident` | yes | fallback | `procedural` |
| 10 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:147` | `test_threads` | no | rrent_exe().unwrap()).args([selector,"--exact","--test-threads=1"]).env(CHILD,"1").output().unwrap();let stdout=String::from_utf8_lossy(&output.stdout);le | `stdio` |
| 11 | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧳️retained/📦️owned/🧪️tests/🦀️.rs:40` | `debug` | yes | println!("[DEBUG] [owned-layout-validation] {}",serde_json::json!({"name":row["name"],"completed":completed | `layout` |
| 12 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📷️raster/🧪️tests/🔬️unit/🦀️.rs:19` | `allow` | yes | #[allow(unused_imports)] use crate::raster::*; | `draw` |
| 13 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🪞️dsl-mirror/🦀️.rs:50` | `unwrap_expect` | yes | lope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id"); | `procedural` |
| 14 | `🧰️framework/🔨️modules/⚠️diagnostic/🏭️message/🦀️.rs:17` | `unsafe` | yes | Scope>();let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<FaultScope>())else{owner.failure=Some("Fault scope allo | `diagnostic` |
| 15 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📏️grow/🧪️tests/🐍️.py:1` | `doc_no_emoji` | yes | Independent oracle of 'repo cache grow': the target size of a growth (declared size and rule maximum bound it, the host volume gives only th | `library` |
| 16 | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🌐️locale/🟦️.ts:39` | `doc_dup_emoji` | yes | 🏷 also leads line 13 | `locale` |
| 17 | `♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx:131` | `comment_fn` | no | //#region 🏛️AProjectOfLuhUdkFooterItem | `mit-bestand/demonstrator` |
| 18 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🧰️kernel-trait-and-engine/🦀️.rs:15` | `comment_def` | no | // #endregion Primitives | `flow` |
| 19 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧊️preview-geometry/🦀️.rs:430` | `compat_ident` | yes | fallback | `procedural` |
| 20 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/📦️owned/🧪️tests/🎟️font/🦀️.rs:24` | `debug` | yes | println!("[DEBUG] [drawing-export-font-block] {}",serde_json::json!({"returned":true,"receiptZero":true,"fu | `draw` |
| 21 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🌉️apply/🦀️.rs:2` | `allow` | no | #![allow(unused_imports)] | `procedural` |
| 22 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🎟️original/🦀️.rs:94` | `unwrap_expect` | yes | let(result,receipt)=state.translation.as_mut().unwrap().step(original,size_of::<HalfedgeMesh>(),cx.retained_grant()).map_err(\|error\|OriginalTurnF | `procedural` |
| 23 | `🧰️framework/🔨️modules/⚠️diagnostic/🏭️message/🦀️.rs:18` | `unsafe` | yes | let scope=unsafe{pointer.as_ptr().write(FaultScope::default());Box::from_raw(pointer.as_ptr())};progress.r | `diagnostic` |
| 24 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🧪️tests/♻️lru/🐍️.py:1` | `doc_no_emoji` | yes | Independent oracle of the least-recently-used retirement of optional graph cache groups (and, with the argument 'entries', of the units insi | `library` |
| 25 | `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts:57` | `doc_dup_emoji` | yes | 🏷 also leads line 2 | `mit-bestand/demonstrator` |
| 26 | `♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx:168` | `comment_fn` | no | //#endregion 🏛️AProjectOfLuhUdkFooterItem | `mit-bestand/demonstrator` |
| 27 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🧰️kernel-trait-and-engine/🦀️.rs:17` | `comment_def` | no | // #region Paths | `flow` |
| 28 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧪️tests/📊️mutate-semio-table/🐍️.py:472` | `compat_ident` | yes | adapter | `stdio` |
| 29 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔤️text/🧪️tests/🎟️original/🦀️.rs:45` | `debug` | yes | println!("[DEBUG] [drawing-original-font] {}",serde_json::json!({"case":case["id"],"completed":completed,"c | `draw` |
| 30 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs:2` | `allow` | no | #![allow(clippy::result_large_err)] | `procedural` |
| 31 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🦀️.rs:199` | `unwrap_expect` | yes | job.step(&mut cx).expect("the funded test turn is admitted").unwrap_or(WidgetStep::Working { progress: 0.0 }) | `procedural` |
| 32 | `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🎟️work/🧪️tests/🦀️.rs:49` | `unsafe` | yes | unsafe impl crate::list::PageAllocation for Excess{fn reserve<T>(owner:&mut Vec<T>,slots:usize)- | `value` |
| 33 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⏯️set-result-animation/🧪️tests/🔬️unit/🦀️.rs:103` | `doc_no_emoji` | no | ␣ LAW: the 'space' chord dispatches the action with NO arguments at all — that gesture toggles. | `fem` |
| 34 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧰️production-fixtures-and-maintenance-law/🦀️.rs:153` | `doc_dup_emoji` | yes | 🔐 also leads line 133 | `procedural` |
| 35 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx:151` | `comment_fn` | no | // #endregion 🗨️Dialog | `ui` |
| 36 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🧰️kernel-trait-and-engine/🦀️.rs:20` | `comment_def` | no | // #endregion Paths | `flow` |
| 37 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧪️tests/📐️mutate-semio-cad/🐍️.py:470` | `compat_ident` | yes | adapter | `stdio` |
| 38 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:178` | `debug` | yes | ts), native_owner)).unwrap_or_else(\|error\|panic!("[DEBUG] Semio Base full-empty decoder case={} encoding={} copiedValueBytes={} refusal={error:?}", | `stdio` |
| 39 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🦀️.rs:2` | `allow` | no | #![allow(clippy::result_large_err)] | `procedural` |
| 40 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/⚖️cold-laws/🦀️.rs:27` | `unwrap_expect` | yes | after = protocol::apply_diff_cold(&forward, base).expect("valid forward diff must apply"); | `fem` |
| 41 | `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:52` | `unsafe` | yes | let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<ControlledRetirement<T>>())else{return crate::RetainedW | `value` |
| 42 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:470` | `doc_dup_emoji` | yes | 🔐 also leads line 449 | `procedural` |
| 43 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts:89` | `comment_fn` | no | /* @vite-ignore */ | `plugin` |
| 44 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🧰️kernel-trait-and-engine/🦀️.rs:22` | `comment_def` | no | // #region Style | `flow` |
| 45 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧪️tests/📃️mutate-semio-document/🐍️.py:545` | `compat_ident` | yes | adapter | `stdio` |
| 46 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️action-producer/🦀️.rs:36` | `debug` | yes | -APPS/🗑️generated/1030/runtime-dbg.txt", format!("DBG1030 fuel={} false={:?} pending={} runnable={} diag={:?}", scope.cx().fuel_remaining(), app.la | `puzzle` |
| 47 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs:49` | `allow` | no | #[allow(unused_imports)] | `procedural` |
| 48 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/⚖️cold-laws/🦀️.rs:30` | `unwrap_expect` | yes | let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture"); | `fem` |
| 49 | `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:53` | `unsafe` | yes | let owner=unsafe{pointer.as_ptr().write(ControlledRetirement::retain(value));Box::from_raw(pointer.as_ptr( | `value` |
| 50 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🦀️.rs:54` | `doc_dup_emoji` | yes | 🧾 also leads line 49 | `procedural` |
| 51 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts:32` | `comment_fn` | no | /* ignore */ | `library` |
| 52 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🧰️kernel-trait-and-engine/🦀️.rs:26` | `comment_def` | no | // #endregion Style | `flow` |
| 53 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧪️tests/📦️mutate-semio-object/🐍️.py:386` | `compat_ident` | yes | adapter | `stdio` |
| 54 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs:621` | `debug` | yes | pub fn dbg1030_terminal_clauses(&self) -> Vec<(usize, bool)> { | `plugin` |
| 55 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs:203` | `allow` | no | #[allow(unused_imports)] | `procedural` |
| 56 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/⚖️cold-laws/🦀️.rs:37` | `unwrap_expect` | yes | et next = protocol::apply_diff_cold(&step, &state).expect("valid inverse diff must apply"); | `fem` |
| 57 | `🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️boxed/🦀️.rs:45` | `unsafe` | yes | w(self.boxed.take().unwrap());*self.original=Some(unsafe{pointer.read()});*self.vacant=Some(OriginalBoxSlot{allocation:ManuallyDrop::new(Some(unsa | `value` |
| 58 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/⚖️cold-laws/🦀️.rs:53` | `doc_dup_emoji` | yes | ✅ also leads line 17 | `fem` |
| 59 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🧰️kernel-trait-and-engine/🦀️.rs:28` | `comment_def` | no | // #region Transforms | `flow` |
| 60 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🐍️.py:554` | `compat_ident` | yes | adapter | `stdio` |

## Notes

- `comment_fn` for TS is approximate: a `{` after `=>` or `)` counts as a function body, so JSX `{/* */}` inside components is included.
- `ui_literal` is a candidate list (JSX text and string props with words). Config-like strings are not caught. Test files are skipped.
- Owner lane is a guess from the plugin or framework module folder name.
- `new vs HEAD` = the line is added in `git diff HEAD` (read-only) or the file is untracked. All other hits are pre-existing in a file touched since 2026-10-30 06:00.
- `json_import` excludes test files, where the attribute appears inside fixture strings and tests deliberately use non-json specifiers.
- `ui_literal` found no JSX text or literal UI attribute outside test files in scope.

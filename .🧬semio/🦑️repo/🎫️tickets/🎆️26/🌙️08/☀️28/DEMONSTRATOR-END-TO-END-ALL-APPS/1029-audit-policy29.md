STATUS: AUDIT DONE (policy29, 10/29). Scanned 2537 changed source files (rs/ts/tsx/mts/mjs/cjs/go/py/sh/wit/feature + Cargo.toml) under 🧰️framework, ✏️s, 🌎️hub, ♻️mit-bestand. NEW hits (lines added vs HEAD or in untracked files): 3-allow 2, 4-unwrap/expect/panic 9, 7-debug 13, 2-reexport 6 (review), 2-legacy-alias 146 (Original* naming, review), 1-boundary 0 raises (2 false/neutral), 8-any 1 (likely false positive), 5-comment-in-body 0, 2-compat-name 0, 8-as-unknown-as 0, 9-ticket-copy 1 outside-ticket + 5 ticket-internal duplicates. Gate: NOT CLEAN, 24 distinct hard-violation lines (25 table rows; the `/// [DEBUG]` line counts under both 7 and 6) to fix.

# 1029 Policy Audit (policy29)

Scope: every source file under `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand` with mtime newer than 2026-10-29 01:55 (excluding node_modules/dist/target/.cache and ticket folders), 2537 files. Method: `1029-tools/policy29/audit.py` (adapted from `1028-tools/policyaudit/audit.py`) scanned the whole content of each file; `new` = line added against HEAD (`git diff HEAD`) or line in an untracked file. Executor attribution is keyword-based: none of the `1029-result-*.md` files lists most of these paths, so the executor column is a candidate list, not a confirmed owner. Raw data: `🗑️generated/1029/policy29-findings.json`, `policy29-new.json`.

## Counts

| Category | NEW hits | all hits in changed files (incl. pre-existing) |
|---|---|---|
| (1) source-boundary limit raises (Cargo.toml) | 2 | 114 |
| (2) pub use re-export | 6 | 1683 |
| (2) pub use ... as alias | 0 | 98 |
| (2) Original*/Legacy* name | 146 | 3797 |
| (2) legacy/compat/shim wording | 0 | 392 |
| (2) #[deprecated] | 0 | 0 |
| (3) #[allow(] | 2 | 823 |
| (4) unwrap/expect/panic!/todo!/unimplemented! in non-test rs | 9 | 2687 |
| (5) comment inside body | 0 | 233 |
| (6) docstring without leading emoji | 1 | 5 |
| (6) JSDoc present (checked: 5 new all start with emoji) | 5 | 2472 |
| (7) [DEBUG]/dbg! | 13 | 49 |
| (7) console.log in non-test ts | 0 | 63 |
| (8) TS any | 1 | 300 |
| (8) as unknown as | 0 | 60 |
| (9) copy of tracked source in ticket folder (non-empty, see below) | 6 | 6 |

Note: `2-ts-ignore`/`@ts-ignore`: 0 hits. `5-comment-in-body` new: 0. `7-console-log` new: 0. Total changed files with any hit (excluding 9): 1563.

## Hard violations (NEW lines)

| # | Cat | file:line | snippet | candidate executor |
|---|---|---|---|---|
| 1 | 7 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧪️tests/🦀️.rs:34` | `dex,row)in fixture["compositions"].as_array().unwrap().iter().enumerate(){let a:TextSplice=serde_json::from_` | reactor, rtb |
| 2 | 7 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧪️tests/🦀️.rs:35` | `for row in fixture["typingRuns"].as_array().unwrap(){let texts:Vec<String>=serde_json::from_value(row["tex` | reactor, rtb |
| 3 | 7 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧪️tests/🦀️.rs:37` | `ompleted<spec["generative"]["cases"].as_u64().unwrap(){let old=text(&mut random);let middle=text(&mut random` | reactor, rtb |
| 4 | 7 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧪️tests/🦀️.rs:42` | `for cut in spec["cancelCuts"].as_array().unwrap(){let mut caller=fixture_caller();let mut decoder=Origi` | reactor, rtb |
| 5 | 7 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧪️tests/🦀️.rs:53` | `der,TURN,\|cx\|{if let Ok(ready)=owner.as_mut().unwrap().advance(view(&a),view(&b),cx){assert!(!ready)}Ok(())}` | reactor, rtb |
| 6 | 4 | `🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs:65` | `1=>if owners.key.is_none(){let mut d=SharedUtf8::admission_demand();d.copy_bytes+=2*size_of::<String>()+size_of::<Option<SharedUtf8>>();d}else if owne` | typedops, extfix |
| 7 | 4 | `🧰️framework/🔨️modules/🛠️tool-machine/🔗️read/📤️transfer/🦀️.rs:37` | `pub fn withdraw_original(&mut self,window:&str,key:RetainedOriginalSlotKey,output:&mut Option<$state<M>>,grant:RetainedCloneGrant)->Result<Option<Reta` | policy |
| 8 | 3 | `plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️inspector/🦀️.rs:3` | `#![allow(clippy::too_many_arguments)]` | energy |
| 9 | 4 | `plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️inspector/🦀️.rs:213` | `let boundary = boundary.expect("the boundary arm parsed its payload above");` | energy |
| 10 | 4 | `plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️inspector/🦀️.rs:217` | `let construction = construction.expect("the construction arm parsed its payload above");` | energy |
| 11 | 4 | `plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️inspector/🦀️.rs:319` | `let glazing = glazing.expect("the glazing arm parsed its payload above");` | energy |
| 12 | 3 | `️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🦀️.rs:6` | `#![allow(long_running_const_eval)]` | energy |
| 13 | 7 | `/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️task-resume/🦀️.rs:29` | `Err(failure) => panic!("[DEBUG] settle refused: {failure:?} \| pending={} runna` | puzzlefin, resume |
| 14 | 7 | `/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️task-resume/🦀️.rs:122` | `app: &Puzzle3dMounted, label: &str\| format!("[DEBUG] {label}: terminal={} pending={} runnable={} ops_empty=` | puzzlefin, resume |
| 15 | 7 | `uzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️action-producer/🦀️.rs:39` | `eprintln!("[DEBUG] turn {turn} answered={:?} pending={} command_ready={} ` | puzzlefin, resume |
| 16 | 7 | `uzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️action-producer/🦀️.rs:40` | `{ let probe = scope.cx(); eprintln!("[DEBUG]   cx yield={} cancelled={} grant={:?} fuel={:?} resume` | puzzlefin, resume |
| 17 | 7 | `uzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️action-producer/🦀️.rs:50` | `eprintln!("[DEBUG] probe {probe_turn} maintenance_ok={m} fuel {f0} -> {f1` | puzzlefin, resume |
| 18 | 4 | `framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs:281` | `elf.tools.original_typing_settlement.as_mut().unwrap();` | bridge, reactor, resume |
| 19 | 4 | `ducts/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🦀️.rs:40` | `1=>{let row=owners.row.as_ref().unwrap();if row.states.len()!=2\|\|row.states[0]!="root"\|\|row.st` | typedops, reactor, resume |
| 20 | 4 | `ducts/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🦀️.rs:47` | `uction(d),\|_\|{let owners=self.owners.as_mut().unwrap();let TypingState{states,tool,actor,buffer,deadline_ms:` | typedops, reactor, resume |
| 21 | 7 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🏃️runtime/🦀️.rs:6` | `/// [DEBUG] temporary state probe` | dispatch, puzzlefin |
| 22 | 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🎟️typed/🏃️runtime/🦀️.rs:6` | `/// [DEBUG] temporary state probe` | dispatch, puzzlefin |
| 23 | 7 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎮️command/🧬️op-binary/🦀️.rs:38` | `eprintln!("[DEBUG] producer phase={} demand={d:?} grant={:?} yield={}",se` | dispatch, puzzlefin |
| 24 | 7 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎮️command/🧬️op-binary/🦀️.rs:52` | `alTurnFailure>{self.validate(cx)?;eprintln!("[DEBUG] take_into co` | dispatch, puzzlefin |
| 25 | 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️producer/🦀️.rs:80` | `=std::sync::Arc::into_inner(self.state.take().unwrap());let released_bytes=if let Some(original)=original{le` | dispatch, policy, reactor |

Category 3 = #[allow]; 4 = unwrap/expect/panic in production code; 7 = temporary [DEBUG] print or doc marker; 6 = docstring without emoji.

## Review items (NEW lines, not automatically violations)

### (2) pub use re-exports added

| file:line | snippet | candidate executor |
|---|---|---|
| `cts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/🧪️tests/🧰️helpers/🦀️.rs:6` | `pub(crate) use post_fixture::PostFixture;` | energy |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🦀️.rs:10` | `pub use original_typing_fold::{OriginalTypingFoldPolicy,ArtifactOriginalTypingFoldFactory,ArtifactOriginalTypingFoldProducer};` | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🦀️.rs:86` | `pub use semio_framework_plugin_original_job_ownership::original_reserved_producer_laws;` | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧵️reserved-tool-job/🦀️.rs:86` | `pub use semio_framework_plugin_original_job_ownership::{ArtifactReservedJob,ArtifactReservedToolJob,ArtifactReservedToolJobState};` | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/🦀️.rs:7` | `pub use producer::{ArtifactReservedJob,ArtifactReservedToolJob,ArtifactReservedToolJobState};` | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/🦀️.rs:10` | `pub use lease::OriginalRequestLease;` | dispatch, policy, reactor |

### (2) Original*/Legacy* names added (146 lines)

No `type X = Original...`, `use ... as Original...` or `pub use ... as ...` alias definitions were found among NEW lines, so these are naming, not compat aliases. They still sit in the forbidden legacy-name family and need a decision by the coordinator.

| file | count | first lines | candidate executor |
|---|---|---|---|
| `ns/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⌨️typing/🔗️fold/🦀️.rs` | 33 | 2, 4, 5, 6, 13, 18, 28, 30 ... | puzzlefin, rtb |
| `ter/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⌨️typing/🔗️fold/🧪️tests/🦀️.rs` | 10 | 3, 9, 10, 13, 17, 26, 32, 37 ... | puzzlefin, rtb |
| `️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎛️writer-play-app-editor-and-constructor/🦀️.rs` | 1 | 38 | puzzlefin, rtb |
| `🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs` | 1 | 105 | puzzlefin, rtb |
| `tandards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️transform-tool-transactions/⌨️typing-edit/🦀️.rs` | 8 | 18, 26, 30, 31, 32, 33, 34, 36 | cachegc, dispatch, rta |
| `s/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️transform-tool-transactions/🦀️.rs` | 4 | 11, 524, 525, 527 | cachegc, dispatch, rta |
| `/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️task-resume/🦀️.rs` | 2 | 26, 120 | puzzlefin, resume |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🦀️.rs` | 16 | 6, 8, 10, 19, 21, 30, 42, 43 ... | reactor, rtb |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧪️tests/🦀️.rs` | 15 | 3, 8, 9, 13, 16, 17, 18, 20 ... | reactor, rtb |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native/🧵️advance.rs` | 7 | 4, 5, 25, 33, 43, 52, 55 | reactor, rtb |
| `🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs` | 7 | 3, 22, 33, 34, 43, 44, 110 | typedops, extfix |
| `🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🧪️tests/🦀️.rs` | 6 | 10, 23, 24, 25, 26, 40 | typedops, extfix |
| `🧰️framework/🔨️modules/🛠️tool-machine/🔗️read/🧪️tests/🦀️.rs` | 4 | 26, 27, 28, 29 | policy |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/⌨️typing/🔗️fold/🦀️.rs` | 3 | 7, 12, 13 | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎟️request/♻️retirement/🦀️.rs` | 1 | 9 | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎮️command/🧬️op-binary/🦀️.rs` | 1 | 52 | dispatch, puzzlefin |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🦀️.rs` | 1 | 10 | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️producer/🦀️.rs` | 2 | 52, 68 | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/🔗️lease/🦀️.rs` | 2 | 5, 6 | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/🦀️.rs` | 1 | 10 | dispatch, policy, reactor |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs` | 2 | 384, 385 | unmapped |
| `mework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/✏️artifact-editor-trait/🦀️.rs` | 2 | 99, 100 | unmapped |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🖊️editor-app/🦀️.rs` | 1 | 208 | unmapped |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs` | 10 | 7, 18, 30, 36, 38, 42, 50, 76 ... | typedops, extfix |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🧵️settlement/🦀️.rs` | 1 | 29 | unmapped |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧱️components/🦀️.rs` | 1 | 118 | unmapped |
| `framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs` | 1 | 279 | bridge, reactor, resume |
| `ducts/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🦀️.rs` | 3 | 23, 24, 54 | typedops, reactor, resume |

### (1) Cargo.toml boundary blocks

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️packages/🦀️rust/Cargo.toml:12` is a NEW crate (untracked) with `maxSourceBytes=262144, maxSourceFiles=32, maxActiveLines=4000`. These are the default ceilings, not a raise. Verify no ceiling is above the policy default.
- `✏️s/.../🔍️inspector/📦️packages/🦀️rust/Cargo.toml:29` `semio-s-artifact-energy-model-mut-ideal-loads-limits`: crate name matches `limits`, false positive.
- Diff check: no changed `maxSourceBytes|maxSourceFiles|maxActiveLines` value differs from HEAD in any changed manifest. No boundary limit was raised.

### (8) TS any

- `✏️s/.../📤️publication/📸️observation/🧪️tests/🟦️.ts:24` is a test title containing the word 'any' in prose: false positive. No real TS `any`, `@ts-ignore`, or `as unknown as` in new lines.

## (5) comments inside bodies

0 new hits. Pre-existing: 233 in changed files (not listed; introduced before 01:55 or not in changed lines).

## (6) JSDoc

All 5 new `/** ... */` blocks start with an emoji (`🛰️`, `📷️`, `🧭️`, `🧬️`, `🛠️`). Pre-existing: 2467 JSDoc blocks in changed files not individually checked for the leading emoji. Pre-existing docstring-without-emoji in Rust: 5 (listed below).

Pre-existing Rust docstring-without-emoji hits:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:218`: `//// 🧊️ Explicit cold-only disposal of a detached flow store — the store-level twin of`
- `/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🔎️declared-verb-probing/🦀️.rs:28`: `/// #️⃣ FNV-1a over every observed part of one settled dispatch.`
- `roducts/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6-x/🦀️.rs:110`: `/// source or nothing), so the extraction belongs to the same call"),`
- `roducts/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6-x/🦀️.rs:345`: `/// `interaction_registry`'s `Topology` twin: a `Flat` domain is omitted from`

## (9) copies of tracked sources inside ticket folders

Scanned files under `🎫️tickets` changed since 01:55 (`*.rs *.ts *.tsx *.mts *.mjs *.go *.py *.wit`) by git blob hash against `git ls-files -s`. Empty files excluded.

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1029-tools/puzzlefin/blk.py` identical to tracked `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1029-tools/dispatch/blk.py`: duplicate inside ticket folders (both tracked): tidy
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1029-tools/puzzlefin/errs.py` identical to tracked `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1029-tools/dispatch/errs.py`: duplicate inside ticket folders (both tracked): tidy
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1029-tools/dispatch/blk.py` identical to tracked `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1029-tools/puzzlefin/blk.py`: duplicate inside ticket folders (both tracked): tidy
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1029-tools/dispatch/errs.py` identical to tracked `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1029-tools/puzzlefin/errs.py`: duplicate inside ticket folders (both tracked): tidy
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/layout_strict.py` identical to tracked `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/layout_strict.py` (+1 more): duplicate inside ticket folders (both tracked): tidy
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/infer-f-tools/staged/files/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🧪️tests/model.py` identical to tracked `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🧪️tests/🐍️model.py`: copy of a tracked source OUTSIDE the ticket: VIOLATION

Only the `infer-f-tools` copy (ticket 🌙️10/☀️18, mtime touched after 01:55) copies a tracked source (`🧪️tests/🐍️model.py`). The 5 ticket-internal duplicates are tool scripts duplicated between `1029-tools/*` folders. Not in the executor scope: they belong to the ticket owner.

## Not audited / limits

- Pre-existing hits in changed files are counted but not listed (3797 Original* names, 2687 unwrap/expect, 1683 pub use, 823 #[allow], 392 compat-words). They predate this wave's diff; list them via `policy29-findings.json` if a full cleanup is wanted.
- No cargo/repo runs, no source edits, no sub-agents.

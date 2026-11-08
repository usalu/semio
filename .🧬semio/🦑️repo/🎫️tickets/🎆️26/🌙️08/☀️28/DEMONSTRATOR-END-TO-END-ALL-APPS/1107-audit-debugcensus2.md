# Debug census 2 against today's tree (lane debugcensus2)

Read-only audit. Refreshes `1107-audit-debugcensus.md` (old census) against the tracked tree on 2026-11-08.

Commands run (cwd set explicitly each time):

- `git -C /Users/ueli/Documents/semio grep -n -F '[DEBUG]' -- . ':(exclude).🧬semio'` gives 95 lines in 38 files. The four `KB1_TRACE` lines are part of these 95 because they also carry the `[DEBUG]` tag.
- `git -C /Users/ueli/Documents/semio grep -n -F 'KB1' -- . ':(exclude).🧬semio'` gives only the four `KB1_TRACE` lines in `3d-brep-boolean/.../🔬️differential/🦀️.rs`. No other `KB1` references in the tree.
- Claims checked: `C/1105b-claims.md` (1564 lines), by keyword and by the claim numbers in the old census.
- Untracked files are not covered by `git grep`, as in the old census.

## Totals

- 95 grep lines (old census: 98).
- 51 keep (not temporary logs): AGENTS.md:92, 6 census-tool lines, 1 census fixture, 1 consumption guard, 1 course site checker, 1 course test string, 1 nx graph parser (`graph/🟦️.ts:336`), 39 nx graph JSON fixture lines. Unchanged from the old census.
- 44 log candidates: 21 Rust, 18 teaching scripts, 5 TypeScript.
- New since the old census: 8 (4 `KB1_TRACE` lines, 4 other Rust lines listed below).
- Removed since the old census: 11 (listed below). They are gone from the tree.
- Needing care: 28 of the 44 (19 Rust, 4 teaching, 5 TypeScript).

Line check: 95 = 51 keep + 44 candidates. Old: 98 = 51 + 47. 47 - 11 + 8 = 44.

## New hits (8)

| file:line | crate / module | live claim | removal shape | care |
|---|---|---|---|---|
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:74` | 3d brep boolean, differential test | none found | Span delete of `if std::env::var_os("KB1_TRACE").is_some(){eprintln!("[DEBUG] case start");}` after `OpRecorder::new();`. The `let` stays. | yes (span, not line; the line carries code) |
| same file `:75` | same | none | Span delete of `if ...{eprintln!("[DEBUG] operand a built");}` after the `?;` of `let a`. | yes (span) |
| same file `:76` | same | none | Span delete of `if ...{eprintln!("[DEBUG] operand b built");}` after `let b`. | yes (span) |
| same file `:83` | same | none | Span delete of `if ...{eprintln!("[DEBUG] boolean done");}` after `let outcome`. | yes (span) |
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️mesh-io/🦀️.rs:207` | 3d brep engine, mesh-io (production source) | reviewfix3, claim 02:55 11/08 "CLAIMED mesh-io (temporary [DEBUG] trace, reviewfix3-301)" | Delete three parts of the line: the `static DEBUG_CALLS ... ;` item, the `let debug_quote=...;` binding, and the `if poll.work()==0 && DEBUG_CALLS...{eprintln!(...);}` statement. `poll` stays because `work=poll.work();` and the `return match poll` use it. `owner.owner().next_demand(...)` is called only by the log. Confirm with reviewfix3 that it is pure before deleting. | yes (production file, minified line, two helper bindings) |
| `🧰️framework/🔨️modules/🧵️job/📬️outcome/👷️worker/🧪️tests/🦀️.rs:9` | job outcome worker test | reviewfix5 edited this test (claim 02:58 11/08) | Delete `eprintln!("[DEBUG] worker setup turns {setup_turns}");`. `let mut setup_turns=0;` and `setup_turns+=1;` in the `for` loop only feed the log: delete them too, keep the loop. | yes (minified, counter variable) |
| `🧰️framework/🔨️modules/🧵️job/📬️outcome/📮️return/🎟️admission/🧪️tests/🦀️.rs:15` | job return admission test | reviewfix5 edited admission tests (claim 02:58 11/08) | Assert message only. Remove `,"[DEBUG] stage {stage}"` from the `assert_eq!`. `stage` is still used by the loop and by `row["actualWork"][stage]`. | yes (assert message) |
| same file `:29` | same | same | Remove `let mut reserve_turns=0;`, the `reserve_turns+=1;` statement inside `for _ in 0..64{...}`, and `eprintln!("[DEBUG] publication reserve turns {reserve_turns}");`. Keep the loop and its `break`. | yes (counter variable) |

## Removed since the old census (11)

All were log statements the old census listed. Removal is consistent with the claims that said so:

- aussuchen sourcing `pane-smoke/🦀️.rs:41, 43, 46, 48` (claims 271 and 280 say removed).
- sprhistory `📡️spr/📜️history/🦀️.rs:369, 428, 429, 432`. No claim line found for this lane in the file; the old census listed it as unknown.
- aggregator `worker/🎟️authority/🦀️.rs:200, 284` and `worker/🧵️worker-job-session/🦀️.rs:29` (claims 369 and later say the gate prints were removed).

## Carried over (36 of the old 47, unchanged line numbers)

Same lines and content as the old census. Removal shapes and care flags are in the old census rows and are unchanged. Owner status is also unchanged; no new claim line was found for these files.

| file:line | crate / module | care |
|---|---|---|
| `✏️s/.../tests/native-tests.rs:347` | fem 3d standards native test | yes (sole body of `if`; dead-variable effects) |
| `native-tests.rs:349` | same | yes (`steps`, `total` become write-only) |
| `🌊️flow/.../🧵️work/🧪️tests/🦀️.rs:31`, `:65` | flow publication work tests | yes (assert message; old census says flow has no temporary prints, so retag or keep) |
| `🏪️store/.../supersession/🧪️tests/🦀️.rs:10`, `:13` | store supersession replay tests | yes (assert message) |
| `🏪️store/.../receiving/🧪️tests/🦀️.rs:28` | store receiving tests | yes (assert message) |
| `🔌️plugin/⏪️time-travel/.../📖️ref-reads/🦀️.rs:44` | plugin runtime, time-travel ref reads | no (first statement of an `if` that continues with `return Err`) |
| `🔌️plugin/♻️cancellation/🦀️.rs:285` | plugin cancellation lease Drop | yes (assert message with `force_capture()` argument) |
| `🔌️plugin/🧪️testing/🧪️artifact-app-laws/.../🦀️.rs:257` | artifact-app-laws settle loop | no (print is the first statement of `if idle>=maximum_idle{...return Err}`). Still useful while the generator stall stays open; the last claim touching it is runtime-b on 11/07. |
| `🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs:32` | plugin instance registration tests | yes (minified; `if index<512` inside a `for`) |
| `🔌️plugin/.../🫧️ephemeral/🧪️tests/🦀️.rs:9` | ephemeral plugin instance tests | yes (assert message with args used only by it) |
| `🌉️mcp/🌱️process-arena/🦀️.rs:32` | mcp process-arena test child | yes (child marker inside a one-line test module; the parent test asserts the marker) |
| `teaching/.../clip/⚙️runtime/🐍️.py:186` | teaching clip runtime | yes (journal write, check whether it is a feature) |
| `teaching/.../clip/🪟️presentation/🌐️browser/🟨️render.mjs:102`, `:104` | teaching browser renderer | no |
| same `render.mjs:103` | same | yes (`onProgress` callback argument) |
| same `render.mjs:200` | same | no (unbraced `if` statement) |
| same `🌐️browser/🧪️tests/seam.mjs:26` | teaching browser test | yes (callback inside a call) |
| `teaching/.../📚️collection/🐍️.py:200` | teaching publication summary | no |
| `teaching/.../🎨️visuals/📐️drafting/✂️fragments/🧪️tests/🟨️.cjs:14` | teaching test summary | no |
| `teaching/.../🔄️lifecycle/🐍️.py:37` | teaching consistency failure path | no |
| same `🔄️lifecycle/🐍️.py:112` | same | no |
| `teaching/.../🧪️tests/🎭️presentation/🐍️.py:69` | teaching timing measurement | yes (`started`, `values` may be used only by it) |
| `nakagin/.../🎨️visuals/✂️cutaway/🟨️.cjs:14` | nakagin test summary | no |
| `nakagin/.../👁️occlusion/✂️fragments/🟨️.cjs:14` | same | no |
| `nakagin/.../🧱️panel/🟨️.cjs:14` | same | no |
| `excel/.../⚡️anchors/🟨️.cjs:27` | excel test summary | no |
| `nakagin/.../🚶️circulation/🐍️.py:479` | nakagin circulation progress | no |
| `nakagin/.../🪟️unit/🐍️.py:107` | nakagin unit debug dump | no |
| `grasshopper/.../🎭️presentation/🐍️.py:200` | grasshopper lifecycle progress | no |
| `nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts:60` | nx graph operation test | yes (test reads these lines back) |
| same `:63` | same | yes (`prefix` is the parser contract; change the test first) |
| `cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/👷️child/📜️script.ts:9` | cargo custody child script | yes (parent asserts the marker) |
| `custody/🧪️tests/🌊️output/🟦️.ts:21` | same | yes (shares a line with other statements) |
| same `:23` | same | yes (assertion on the log) |

Count check: 47 old candidates - 11 removed = 36 carried over, which is the table above. Carried over by type: 13 Rust (time-travel, cancellation, artifact-app-laws, registration, ephemeral, process-arena, flow x2, supersession x2, receiving, native-tests x2), 18 teaching, 5 TypeScript. Plus the 8 new hits this gives 44.

Note on the old census: its summary says 28 need care, but its row flags add up to 29. The care count in this report is taken from the row flags, see the totals.

## Removal-shape warnings (mechanical delete breakers)

- Sole body of an `if`: native-tests `:347`; registration `:32` (inside a `for`). Delete the whole `if`, not only the print.
- Helper variables used only by the log: native-tests `:349` (`steps`, `total`); mesh-io `:207` (`DEBUG_CALLS` static, `debug_quote`); worker `:9` (`setup_turns`); admission `:29` (`reserve_turns`); pane-smoke (removed already).
- Assert messages: flow `:31`, `:65`; supersession `:10`, `:13`; receiving `:28`; ephemeral `:9`; cancellation `:285` (the `force_capture()` argument must go with the fragment); admission `:15`.
- Trailing statements on code lines (span delete, not line delete): the four KB1 lines.
- Minified one-line files: most Rust hits in this list. A line delete removes unrelated code.
- Load-bearing for a test or parser: nx operation `:60`, `:63`; custody `:9`, `:21`, `:23`; nx graph parser `graph/🟦️.ts:336` (keep).

## Claims drift (grep vs claims)

- Claims still list TEMP `[DEBUG]` prints that no tracked file has any more: runtime-gate, rb-gate in retained-command-job and full-operation, energie-48, aggregator S1-S3 and B1, flow host-laws probes, store cancel probe, aussuchen capture print. Grep shows none. Their "remove after" steps are done.
- Still live in the tree: `reviewfix3` mesh-io `:207` (claim 11/08 02:55, open). The rb-gate settle-idle print at `artifact-app-laws` `:257` is still in the tree while the runtime-b generator stall is open.
- KB1_TRACE has no claim line. The boolean lane was released on 11/07 at 23:08 with the boolean matrix still failing (243 to 157), so the trace may be used by whoever debugs that matrix. Ask before deleting.

## Result

44 log candidates (old 47), 8 new, 28 needing care. Full line: total 44, new 8, needing care 28.

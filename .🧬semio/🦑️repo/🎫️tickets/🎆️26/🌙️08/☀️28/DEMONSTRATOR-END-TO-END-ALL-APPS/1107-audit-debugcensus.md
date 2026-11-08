# 🧹️ Debug census before activation (lane debugcensus)

Read-only audit. Source: `git -C /Users/ueli/Documents/semio grep -n -F '[DEBUG]' -- . ':(exclude).🧬semio'` (tracked files only; untracked files are not covered). Lane ownership is taken from `1105b-claims.md`; where no claim exists the owner is marked as inferred from the file path or the message label.

## Totals

- 98 grep lines in total.
- 51 are not temporary logs and must stay: 39 JSON fixture lines in the nx graph tests, 6 lines of the census tool itself, 1 census source fixture, 1 `consumption` guard assertion, 1 course site leftover checker, 1 course test fixture string, 1 nx graph parser (`graph/🟦️.ts:336`), and `AGENTS.md:92` (rule text).
- 47 are log candidates (24 Rust, 18 teaching scripts, 5 TypeScript).
- 28 of the 47 need care before deletion (see the "Care" column).

## Keep: not temporary logs

| file | lines | why it stays |
|---|---|---|
| `AGENTS.md` | 92 | Rule text. Never edit AGENTS.md. |
| `🧰️framework/.../🦑️repo/.../📝️docstring-and-debug-census/🟦️.ts` | 152, 155, 157, 163, 168, 182 | The census implementation itself (`git grep` for the tag). Deleting it breaks the census. |
| `🧰️framework/.../🧪️test/.../🧫️fixtures/🧮️source-census/🔣️.json` | 29 | Census fixture input. |
| `🧰️framework/.../📥️consumption/🟦️.ts` | 41 | Guard: `expect(source).not.toContain("[DEBUG]")`. |
| `🎓️teaching/.../📚️course/🚀️deploy/🌐️site/🟦️.ts` | 107 | Site leftover checker (string list). |
| `🎓️teaching/.../📚️course/🧪️tests/.../🟦️.ts` | 64 | Test input string. |
| `.../⚡️caching/.../📦️graph/🟦️.ts` | 336 | Production parser for the `[DEBUG] Fixed UDRW image ` prefix. See the nx notes below. |
| `.../📦️graph/🧪️tests/🔣️.json` | 39 hits (lines 5324 to 19151) | Recorded tool output fixture for the parser. Data only. |

## Log candidates

Owner column: the lane that plausibly owns the hit. Pane needed: whether a live lane still depends on the log.

### Rust: runtime, store, flow, aggregator and others

| file:line | crate / module | owner | pane still needs it | removal shape | care |
|---|---|---|---|---|---|
| `✏️s/🔌️plugins/🏗️fem/.../🧪️tests/native-tests.rs:347` | fem 3d artifact standards, native tests | runtime (probable, claims 272 "result-page retained_capacity") | unknown, confirm with runtime | sole body of `if p.retained_capacity_bytes>=256{...}`. Delete the whole `if` line. | yes |
| `…/native-tests.rs:349` | same | runtime (probable) | unknown | `eprintln!` only. `steps` and `total` become write-only after 347 and 349 are removed (dead-variable warnings), so `steps+=1` and `total+=…` must go too. | yes |
| `✏️s/🔌️plugins/🪵️sourcing/.../✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs:41, 43, 48` | sourcing curation, editor pane-smoke test | aussuchen (claims 227, 271, 303) | claims 271 and 280 say these were removed. They are still present, so the claim is stale or the prints were re-added. Confirm with aussuchen first. | each is an `eprintln!` whose args call `app.snapshot().expect(...)` and `receipt.*`. Those calls exist only for the log; delete the `eprintln!` plus the helper calls. | yes |
| `…/pane-smoke/🦀️.rs:46` | same | aussuchen | same | `let result` (line 45) is used only by this print. Delete the print and the `let result` binding (keep the `dispatch_…` call only if it has side effects; it does). | yes |
| `…/🧬️.../🌊️flow/.../🧪️tests/🦀️.rs:31` and `:65` | `semio-os-flow-host` publication work tests | flow (claims 499 says flow uses permanent assert diagnostics, not `[DEBUG]`) | no. Flow says no `[DEBUG]` prints remain. | assert-message text only. Strip the `"[DEBUG] …"` prefix and keep the assertion. These are permanent diagnostics. | yes (assert message) |
| `🏪️store/🏗️initialization/🔁️replay/🧬️supersession/🧪️tests/🦀️.rs:10` and `:13` | `semio-os-store` replay supersession tests | store | no | assert-message text only. Strip the prefix, keep the assertion. Minified one-line file. | yes (assert message) |
| `🏪️store/🧩️composition/📨️emission/🧳️receiving/🧪️tests/🦀️.rs:28` | store composition receiving tests | store (energie possible) | no | assert-message text only. Minified. | yes (assert message) |
| `📡️spr/📜️history/🦀️.rs:369` | `semio-os-spr` history source | sprhistory (claims 1215 to 1245; lane status unverified) | unknown | `eprintln!` as first statement of an `if` body that also returns. Delete the print line only. | no |
| `…/📜️history/🦀️.rs:428` | same | sprhistory | unknown | match arm `Ok(None) => {eprintln!(...);(Ok(...), None)}`. Remove the print, keep the tuple value. | yes |
| `…/📜️history/🦀️.rs:429` | same | sprhistory | unknown | match arm, same shape as 428. | yes |
| `…/📜️history/🦀️.rs:432` | same | sprhistory | unknown | standalone `eprintln!` with `result`, `issued`, `self.last_work` args. Delete the line. | no |
| `⏪️time-travel/🧵️instance/📖️ref-reads/🦀️.rs:44` | runtime, time-travel ref reads | runtime (label; no claim line) | unknown | first statement of an `if` body that continues with `return Err(...)`. Delete the print only. | no |
| `♻️cancellation/🦀️.rs:285` | plugin cancellation lease `Drop` | aggregator per the lead's example; no claim line in 1105b-claims.md | unknown | assert message `"…\n[DEBUG] ledger-lease-drop {}"`. The `force_capture()` backtrace is the argument consumed by the `{}`. Remove the message fragment and the argument together, or the assert fails to compile. | yes |
| `🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction…/🦀️.rs:257` | artifact-app-laws settle loop | runtime-b (claims 349 and 419) | yes, while the generator stall is open (claims 419) | first statement of `if idle>=maximum_idle{…; return Err(...)}`. Delete the print only. | no |
| `🎟️registration/🧪️tests/🦀️.rs:32` | plugin instance initialization registration tests | unclaimed (plugin instance, runtime or store) | no evidence | `if index<512{eprintln!(...);}` inside a `for` on a minified line. Delete the `if` by span, not by line. | yes (minified) |
| `🫧️ephemeral/🧪️tests/🦀️.rs:9` | ephemeral plugin instance tests | unclaimed | no evidence | assert message with many format args (`presence`, `transient`, `factories`, `cursors`, `total`) used only by the message. Strip the prefix and any args that only feed it. Minified. | yes (assert message) |
| `🌉️mcp/🌱️process-arena/🦀️.rs:32` | mcp process-arena test child | unclaimed (mcp) | no tracked parent reads `process-arena-child-ready` (grep: one hit only) | child-protocol marker inside a one-line `#[cfg(test)] mod tests{...}`. Delete the statement by span. | yes (minified) |
| `🔨️modules/🧵️job/👷️worker/🎟️authority/🦀️.rs:200` | worker authority (`aggregator-gate C2`) | aggregator (claims 351) | yes, while typed-op quiescence stall is open (claims 160, 351) | block `{static N…; let n=N.fetch_add(…); if n<10||n%50_000==0 {eprintln!(...)}}`. The static counter and `n` exist only for the log. Delete the whole block. Minified line. | yes |
| `…/👷️worker/🎟️authority/🦀️.rs:284` | same (`aggregator-gate C3`) | aggregator | same | same block shape as 200. | yes |
| `…/👷️worker/🧵️worker-job-session/🦀️.rs:29` | worker job close step (`aggregator-gate C1`) | aggregator | same | same block inline inside a `return Ok(...)` expression on a 1126-char line. Delete the block by span. | yes |

### Teaching scripts and browser renderer (not in the 1105b wave)

| file:line | owner | pane still needs it | removal shape | care |
|---|---|---|---|---|
| `🎬️clip/⚙️runtime/🐍️.py:186` | unclaimed (teaching) | likely a permanent command journal, not a debug line | a write plus `flush()` inside `if stream:` in a `with` block. Not a sole body. Check whether the journal format is a feature before deleting. | yes |
| `🎬️clip/🪟️presentation/🌐️browser/🟨️render.mjs:102, 104` | unclaimed | progress output, arguably permanent | standalone `console.error` lines. Delete or retag. | no |
| `…/render.mjs:103` | unclaimed | progress callback | `onProgress:` argument inside a call. Editing needs the call site changed. | yes |
| `…/render.mjs:200` | unclaimed | progress every 30 frames | unbraced `if(...)console.error(...)` statement. Delete the whole statement. | no |
| `…/🌐️browser/🧪️tests/seam.mjs:26` | unclaimed | progress callback in test | inline callback inside `playwrightV1([...], {onProgress: ...})`. Remove the callback, not the call. | yes |
| `🎬️clip/🪟️presentation/📚️collection/🐍️.py:200` | unclaimed | publication summary, permanent | standalone `print`. Delete or retag. | no |
| `🎨️visuals/📐️drafting/✂️fragments/🧪️tests/🟨️.cjs:14` | unclaimed | test-pass summary | sole `console.log` statement. Delete or retag. | no |
| `🔗️consistency/🧪️tests/🔄️lifecycle/🐍️.py:37` | unclaimed | failure path | `print` in `except` before `raise`. Delete the print. | no |
| `…/🔄️lifecycle/🐍️.py:112` | unclaimed | progress | `print` inside a loop body. Delete the print. | no |
| `🧪️tests/🎭️presentation/🐍️.py:69` | unclaimed | timing measurement | `print('[DEBUG] native-timing '+json.dumps(...))`. `started` and `values` may be used only here. Check before deleting. | yes |
| `🏢️nakagin/🧪️tests/🎨️visuals/✂️cutaway/🟨️.cjs:14` | unclaimed | test-pass summary | sole `console.log`. | no |
| `🏢️nakagin/🧪️tests/👁️occlusion/✂️fragments/🟨️.cjs:14` | unclaimed | test-pass summary | sole `console.log`. | no |
| `🏢️nakagin/🧪️tests/🧱️panel/🟨️.cjs:14` | unclaimed | test-pass summary | sole `console.log`. | no |
| `📊️excel/🎬️clip/🧪️tests/⚡️anchors/🟨️.cjs:27` | unclaimed | test-pass summary | sole `console.log`. | no |
| `🏢️nakagin/🧪️tests/🚶️circulation/🐍️.py:479` | unclaimed | progress | standalone `print`. | no |
| `🏢️nakagin/🧪️tests/🪟️unit/🐍️.py:107` | unclaimed | debug dump | first statement of a `for` body that continues with another loop. Delete the print. | no |
| `🦗️grasshopper/🎬️clip/🧪️tests/🎭️presentation/🐍️.py:200` | unclaimed | lifecycle progress | standalone `print`. | no |

### TypeScript: nx graph and cargo custody (unclaimed, repo product)

| file:line | owner | pane still needs it | removal shape | care |
|---|---|---|---|---|
| `📦️graph/🧪️tests/🎛️operation/🟦️.ts:60` | unclaimed (nx caching) | the test reads `[DEBUG] graph-root ` lines (see :63) | `console.log("[DEBUG] graph-root "+JSON…)` inside an abort listener. Removing it changes what the test reads back. | yes |
| `…/🎛️operation/🟦️.ts:63` | same | yes, `prefix` is the marker the test parses | `prefix="[DEBUG] graph-root "` is a parser constant, not a log. Do not delete. Change the test contract first. | yes |
| `🛠️preparation/🧾️custody/🧪️tests/🌊️output/👷️child/📜️script.ts:9` | unclaimed (cargo custody) | yes, the parent test asserts this marker | `writeOwnedCommandOutputV1(process.stderr,"[DEBUG] custody stdout backpressure=…")`. Used by the assertion at `🟦️.ts:23`. Load-bearing. | yes |
| `…/🧾️custody/🧪️tests/🌊️output/🟦️.ts:21` | same | no | `console.error("[DEBUG] Genuine Semio capture bytes=…")` shares a line with other statements. Delete the statement only. | yes |
| `…/🧾️custody/🧪️tests/🌊️output/🟦️.ts:23` | same | the assertion depends on the marker | `expect(delivered.stderr).toContain("[DEBUG] custody stdout backpressure=true completed=true")`. This is an assertion on the log. Change the assertion, not just the log. | yes |

## Care summary (28 hits)

- Assert messages (7): supersession `:10`, `:13`; flow `:31`, `:65`; store receiving `:28`; ephemeral `:9`; cancellation `:285`.
- Helper variables used only by the log (7): native-tests `:349`; pane-smoke `:41`, `:43`, `:46`, `:48`; spr `:428`, `:429`.
- Blocks with static counters or minified one-line code (6): worker authority `:200`, `:284`; worker-job-session `:29`; registration `:32`; process-arena `:32`; render.mjs `:103` (callback arg).
- Load-bearing for tests or parsers (5): nx operation `:60`, `:63`; custody script `:9`, `:23`, and `:21` (shared line).
- Other (3): native-tests `:347` (sole body of `if`, with dead-variable side effects), teaching `clip/runtime` `:186` (journal feature), teaching `presentation` `:69` (timing measurement).

Mechanical deletes (19): time-travel `:44`, rb-gate `:257`, spr `:369`, `:432`, teaching `render.mjs` `:102`, `:104`, `:200`, `collection` `:200`, the 5 `.cjs` summaries, consistency `:37` and `:112`, circulation `:479`, unit `:107`, grasshopper `:200`.

## Lane status and claims drift

- aussuchen: claims 271 and 280 say all pane-smoke prints were removed. Four are still present at pane-smoke `:41`, `:43`, `:46`, `:48`. Ask aussuchen whether they are intentional or stale.
- runtime-gate: no `runtime-gate` hits remain. Claims 330 says the `loan-ctx` print remains. Grep says it does not. Claim is stale or the print was removed without a claim update.
- energie: claims 296 says energie-48 prints were removed. None remain in tracked files, consistent with claims.
- aggregator: `aggregator-gate` C1 to C3 remain (worker authority `:200`, `:284`; worker-job-session `:29`). Claims 369 says R1 and R2 were removed. Those are gone. Keep C1 to C3 only while the typed-op quiescence stall is open.
- flow: claims 467 and 499 say flow has no temporary prints and uses permanent assert diagnostics. The two `[DEBUG] ` tags on flow publication work tests should be retagged or kept as permanent messages, not deleted as temporary logs.

## Notes for the census

- The census tool and its fixture (`docstring-and-debug-census`) contain the literal tag and must not be swept. Exclude them from the zero-hit gate.
- Teaching scripts and the nx/custody TypeScript hits sit outside the 1105b wave. No lane has claimed them. Decide owners before the quiet point.
- Untracked files were not covered by `git grep`.

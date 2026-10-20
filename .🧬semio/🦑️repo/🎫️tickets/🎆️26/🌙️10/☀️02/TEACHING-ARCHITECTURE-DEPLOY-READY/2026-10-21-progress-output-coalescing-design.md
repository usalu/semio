# Progress Output Compatibility and Bounded Coalescing Design

Observed 2026-10-20T23:19:36.439Z. This is a read-only Source and historical-log assessment. No production function, test, native command, guest workload, Docker operation or private effect was executed. No Source, sealed input, output policy or live cohort was changed. TTY75119 remains under its existing owner's custody.

## Recommendation

After the current native outcome, prepare a schema-first correction in the existing process progress writer for the concrete dependency ownership, immutable input replay, installed-path scan/ledger and source-preservation activity families. Use finite pending slots declared by schema; do not flush them merely because an ownership operation or family alternates. Preserve actual owner/command/Script-stage transitions, immediate diagnostics and semantic receipts, every original callback and checkpoint, and settlement flushing. Keep the whole-workload output limits unchanged. The measured evidence does not justify suppressing arbitrary bare-running records.

This is a concrete host compatibility risk, not an observed Linux output-limit failure. A successful focused regression or host replay would not establish the Linux whole21 workload stays within its output budget; that requires actual bounded native/guest evidence after a separately reviewed seal.

## Measured Evidence

The existing terminal log `🗑️generated/2026-10-20-native-build-regression8.log` was read in bounded 1 MiB chunks. Its complete fixed extent was 79,717,778 bytes and 492,658 newline-framed records, with no trailing partial record. Before/after descriptor checks of device, inode, size, mtimeNs and ctimeNs found no drift. This exceeds the guest fixed 67,108,864-byte budget by 12,608,914 bytes and its 262,144-line budget by 230,514 lines, even though this host startup had not collected tests. Host and Linux ownership cardinality, warmth and routing can differ.

| Actual record category | Records |
| --- | ---: |
| Raw Script JSON with activity | 302,722 |
| Raw Script JSON, bare running | 167 |
| Raw Script JSON, loading | 1 |
| Raw Script JSON, complete/contribution | 0 |
| Human Script lines | 189,744 |
| Other lines | 24 |
| Total | 492,658 |

All 302,890 raw Script JSON records parsed. Of the human Script lines, 189,185 contained the generic “remains active on” wording. Those human lines are separate output records; they are not bare-running JSON. Only 167 bare-running JSON records were present, so a broad fallback targeting them would miss the measured source of excess volume.

| Raw activity family | Records |
| --- | ---: |
| Ownership list | 99,494 |
| Ownership state | 80,198 |
| Ownership frontier | 11,401 |
| Ownership selection | 3,581 |
| Ownership readText | 966 |
| Immutable store input replay | 57,528 |
| Installed paths, including ledger | 23,160 |
| Preserving source | 12,046 |
| Other activity | 14,348 |
| Total raw activity | 302,722 |

The four proposed families account for 288,374 raw activity records. The Root report `2026-10-21-native-output-budget-assessment.md` additionally records a finite live neutral5 extent of 9,688,490 bytes/57,949 records during dependency preparation. That extent is neither a final log nor a terminal outcome. I did not poll or measure the current live handle.

The guest controller accumulates stdout and stderr across all 21 commands. Its append boundary refuses a complete incoming chunk before accepting bytes/lines that exceed either fixed budget; observation also refuses an oversized actual output.log. These are whole-workload caps, not per-command allowances.

## Actual Publication Boundary

The implementation is `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` (the package wrapper only reexports it). At lines 43–53, `createScriptProcessProgressWriterV1` constructs the raw JSON row and optional formatted human message. `emit` writes raw first, then the human line if present. Only validated provider, cache-scan, cache-delete and tooling activity enters the current 250 ms presentation pacing. Unclassified activity takes line 48: flush the old pending row, reset its identity/clocks and emit immediately.

At lines 98–99, the generic formatter adds a human line when an otherwise unclassified activity has elapsedMs >= 15,000. Thus one callback can emit two newline records. This path is not conditional on CI. Parser, graph-phase, queue and Source-admission formatting are distinct existing contracts and must retain their semantics.

The public Script stages in `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts` are contribution, loading, running and complete. Resource operation names state/list/readText/membership/frontier/selection/publication are unit kinds inside running work, not Script stage changes.

`advanceScriptInvocation` retains check → await publish → check → await yield → check. The issued child control in the implementation retains its remaining/issuance checks around the forwarded publication and its separate yield continuation. The presentation writer must neither sleep nor skip, batch, delay or replace those calls. Its scope is the final diagnostic sink only.

At lines 152–160 the actual Original envelope owns the writer. Settlement drains accepted work, flushes presentation, then checks successful invocation authority. A failure attempts a final flush while preserving the exact primary error if presentation also throws. Those success/failure distinctions, signal identity, deadlines, child controls and cancellation behavior must remain intact. Command stdout/stderr, errors, semantic receipts and machine outputs outside this progress writer remain untouched.

## Concrete Producer Semantics

The producer is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`.

- Ownership (around line 570): publishes exclusive `dependencies:store-ownership:<operation>:<completedOperations>` at the existing work-unit threshold, then yields and checks. The facade's closed operation enum has seven members. CJS ownership traversal increments one shared completedOperations counter across unit kinds during one controlled computation. Changing state → list → frontier is ordinary interleaving. Counters exceed the provider contract's 65,536 cap, so that unrelated cap must not be reused.
- Replay (around line 421): every immutable input read/hash chunk may publish shared `dependencies:store-input-replay` with elapsedMs exactly zero, then yield/check. Zero is intentional; no aggregate duration, byte count or total may be invented from it.
- Installed scan/ledger (around lines 36/59): shared `javascript-installed-paths:<entries>` and `javascript-installed-paths:ledger:<work>` carry different counters and cumulative elapsed time. They need separate finite slot identities, not an unbounded resource-string map.
- Preservation (around line 294): exclusive `javascript-preserving-source:<rows>` carries a row count and cumulative elapsed time. Rows may remain constant while more work occurs; equal counts are not resets and are not bytes or completed I/O operations.

The workspace facade and CJS ownership implementation were read to establish these counter semantics; no producer behavior needs to change for the proposed presentation correction.

## Minimal Bounded Design

First extend the existing progress schema, feature and language-agnostic corpus, then implement and verify the actual writer. No new generic lifecycle or transport layer is needed.

1. Declare exact prefixes, modes, canonical safe-integer counter grammar, finite nonnegative elapsed bounds, bounded owner/command/resource strings and exact outer/activity keys. Replay has its own no-counter, zero-elapsed contract. Malformed or unknown resources remain immediate raw evidence; do not make successful parsing a requirement for the underlying work.
2. Declare exactly eleven new finite pending slots: seven ownership unit kinds, replay, installed scan, installed ledger and preservation. Each stores at most the latest original raw row plus its already computed optional human message and bounded scheduling metadata. The slot array is fixed by schema; no owner, path, resource counter or digest may grow a Map. Bound the additional retained rows/strings explicitly. Existing typed-family state must remain separately bounded and must not flush these eleven slots on every family switch.
3. Track the actual owner/command/running frame outside those slots. A callback for an already admitted unit kind updates that slot. A switch between state/list/frontier or between ownership/replay/installed/preservation must not flush other slots or reset pacing. In particular, do not send each new family through the old count-null branch, and do not put the entire changing resource string into the shared identity.
4. Use the existing 250 ms presentation cadence without timers, sleeps or a background worker. Publish a first observation, retain later observations inside the interval, and at due publication emit the finite pending batch in last-observation order. Ownership shares one scheduling window across all seven unit slots; the other admitted families join the same bounded frame window. At most eleven final original observations are retained for a window; rapidly alternating families therefore cannot turn every callback into an emit. The schema and corpus must pin the first-observation/batch rule and tie ordering, rather than leave it implementation-dependent. This is a bounded presentation rate within a stable frame, not a promise that arbitrary reset/diagnostic traffic fits a whole-run cap.
5. Genuine frame changes (owner, command, or Script stage), success/failure settlement, and explicit flush drain all pending slots in deterministic last-observation order before the boundary record. Final flush clears retained rows and is idempotent. Preserve the last pending observation of every admitted unit kind, rather than losing one family when another arrives. Existing meaningful graph-phase transition records remain immediate.
6. Reset policy follows producer facts. A decreasing ownership cumulative counter/elapsed value starts a new ownership computation and drains/resets that family's slots; it does not turn operation alternation into a reset. Installed scan and ledger track their own counters. Preservation equal row count is ordinary work. Replay's constant zero elapsed does not reset anything. A nonfinite/backward presentation clock flushes pending evidence and refuses pacing for that observation; it does not refuse the operation or invent a clock.
7. Keep raw JSON and its human line together as one presentation row. A retained row must be byte-identical to the original serialized event; no aggregate receipt or synthesized stage replaces it. A raw failure/unknown record must stay immediate and ordered after older pending rows. Retain the current primary-error precedence if a final sink write throws.

This choice explicitly covers operation and family interleaving while keeping retained state closed and small. The need for immediate genuine resets means no useful finite coalescer alone proves the total 21-step budget; the sealed actual workload remains the acceptance boundary.

## Existing Test Interface and Required Vectors

Use the existing files only:

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🥒️.feature`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`

The current test already extracts the actual writer's function AST with TypeScript, transpiles that actual implementation, injects its clock/sink and compares original JSON records against independent SQLite scheduling. Ajv and the local schema validator validate the language-agnostic corpus. Extend that interface; do not validate a second handwritten mock writer.

Required corpus cases: same-clock bursts; alternating all seven ownership kinds; ownership/replay/scan/ledger/preservation interleaving; threshold-minus-one/exact-threshold; final pending observation of every active slot; fixed eleven-slot bound despite many unique counters; owner/command/stage boundaries; genuine cumulative counter reset; equal preservation count; replay zero elapsed; backwards/nonfinite presentation clock; malformed/unknown prefix, mode, counter, keys and excessive bounds; exact raw JSON retention and deterministic tie ordering; repeated flush; sink failure and exact primary-error precedence. The independent SQLite oracle must derive slot/window acceptance from row facts and schema policy, not fixture fault labels or a copied JavaScript scheduler.

Extend an actual Original publisher regression as well: invoke every publication and yield through issued controls, assert exact callback/check/yield counts and signal identity, preserve cancellation refusal, flush every final pending family on success and failure, and ensure a throwing final presentation sink cannot replace the primary failure. Keep existing provider/cache/tooling regressions to detect interference. English/German human text must remain separated from unchanged raw evidence; no new total/time labels should be invented.

A later replay of the finite historical events can quantify reduction but cannot substitute for actual native collection/terminal evidence or Linux acceptance. No such replay/test was executed for this report. The proposed implementation directly needs no new runtime Source file beyond the existing writer/schema; test inputs can stay in the existing corpus/feature/test files. The producer/facade/CJS files above are research dependencies, not a proposal to add whole transitive Source rosters.

## Exact Inspected Versions

- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts — 48004 bytes; SHA-256 b6619b605394a6a22ea3184849a4038693e27e67ff31a925d7374fc8b8d9f001
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts — 5172 bytes; SHA-256 863811180fdeb1e988674ffd9947a7dbeb20a025c6f64e5b20e1e5bd20a729fd
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json — 18817 bytes; SHA-256 8e9f408cb880364395bda81700ca75d26684320f7ddb62b258bdc4089033115b
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts — 42110 bytes; SHA-256 ed414f83a361e57b5fcf9a5bb1f865c1dff05696129cf21f17098590faf32a09
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🔣️.json — 43386 bytes; SHA-256 81df986f35ca73d637ebd94222b429936cdb00f735d6f17968d15f30ea233d78
- 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🥒️.feature — 2825 bytes; SHA-256 03cb15e9c324a5070db52863272f3422ada5d96be0a35d16210f7f3fc39e1b76
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts — 271932 bytes; SHA-256 4a5c26be6ccc77adc8c7b8085797b1b2feb21cfd41877117a95ecfb73eb3879c
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts — 8556 bytes; SHA-256 d02436e00156d494e74ff70c7f35233afda03d37b2373256c7f22eca6cf521f4
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs — 42245 bytes; SHA-256 758f12298c24f33773bd89e76985aca01ae21605e18bfbe737702363093747c1

These are observations only, not a seal refresh. No current native outcome is asserted here.

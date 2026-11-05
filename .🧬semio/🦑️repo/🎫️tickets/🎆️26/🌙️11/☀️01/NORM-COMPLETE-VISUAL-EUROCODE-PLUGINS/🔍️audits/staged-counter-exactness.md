# Staged Counter Exactness

Bounded read-only current source review and ordinary repo tools exec --light bun regex probe; no gate or production edits. Manifest staging base: 🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging. Host intent base: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🛡️intents.

The stated terminal-linebreak acceptance was not reproduced: actual Bun1.4.2 probes of anchored canonical digit patterns with flags u and without flags reject suffix LF, CR, U+2028, U+2029 and CRLF. Only explicit multiline m probe accepts them. BigInt does accept whitespace suffixes alone, but current validators short-circuit on the rejecting regex first. This is a bounded runtime observation, not a browser engine matrix or native test pass. A trailing-linebreak neutral refusal law remains useful exactness coverage; do not claim it is current JS red without actual result.

Affected canonical-counter consumers:

- Manifest 🟦️.ts:42 token generation uses /^(0|[1-9][0-9]{0,19})$/u and lexical20-digit u64 ceiling.
- Manifest refresh.ts:17 generation uses same regex plus BigInt maxu64.
- Manifest page.ts:16 offset uses /^(0|[1-9][0-9]*)$/ with length<=20 plus lexical u64 ceiling; covers fieldOffset, diagnosticOffset and choice.offset at19/21.
- Host 🟦️.ts:26 counter uses20digit regex plus BigInt u64 ceiling, consumed by owner admission43 and generation increment47.
- Host owned.ts:25 counter uses typeofstring plus20digit regex and BigInt u64 ceiling; status29 uses it for diagnosticOffset/diagnosticTotal, caps diagnostics16 and31 checks offset+length<=total with exact BigInt arithmetic.

Native page.rs StagedFormPage.admitted uses StagedFormTextFormat::DecimalU64 for all3 offsets. That format in staging/🦀️.rs canonical_integer plus parseu64 rejects non-ASCII digits, whitespace, leadingzeros, negatives and overflow. Paid page-request.rs phase2 scans every byte, checked_mul/add and rejects any nondigit, preserving full exact strings on output. request.rs:57–58 similarly scans generation exactly.

Actual remaining declaration/runtime mismatch: staging/🔣️.json $defs.Status diagnosticOffset and diagnosticTotal (lines636/640) have only unbounded digit regex ^(0|[1-9][0-9]*)$, no maxLength or u64 ceiling. Host owned.ts rejects values above18446744073709551615. Token and Page schema counters already use an explicit upper-bound regex. Status schema caps diagnostics16, but does not express offset+length<=total. Native StagedFormStatus.admitted in staging/🦀️.rs checks token and ready/commitArgs relationship only; it does not check either diagnostic counter, diagnostic count or page arithmetic. These are source-backed distinct admission bounds, not evidence of an actual mounted hostile status.

No code patched; root owns central stagedFormCounter and relevant consumer changes. First-leaf retry held for actual source_binding owner-ready notice.

## Authorized Diagnostic Alignment Regression

Before production fixes, authored shared status-counter-cases.json (18 cases) and declared its runtime nativeSources in existing arg-schema project. Existing commit.test.ts adds Ajv2020 schema and JSON wire roundtrip law. Existing tests.rs adds serde decode versus actual FromValue admission, with exact accepted-wire roundtrip; no additional Rust file. Vectors cover maxu64/above-JS-safe, both counter overflow/leadingzero/numeric/empty/linebreak refusals, exact16-page fit, count17, offset>total, page overrun and addition overflow. Pure paging-only refusals explicitly retain schemaAllowed:true because standard JSON Schema does not express arbitrary exact decimal-string arithmetic; native admission carries actual arithmetic law. No schema/admission fix yet; actual targeted gates held until request owner source-ready as root instructed.

Actual schema red: focused repo tools exec --light bun -- test ./…/staging/commit.test.ts terminal1, existing1test passes and new1test fails at overflow-offset (expected false, received true). Initial own fixture diagnostic path[] was corrected to schema-valid [value] before accepting intended red. Log generated/status-counter-red.log. First attempted Bun operand lacked ./ and was refused before process startup by genuine operand guard; corrected supported command. Focused native admission red56471 now ordinary repo cargo framework manifest, arg-schema lib filter diagnostic_status_counters_match_exact_neutral_u64_and_page_bounds. No production fixes yet.

Native statusred56471 terminal1/Cargo101 before intendedruntime: root reference-coordinate law at staging/tests.rs176 invokes StagedValueReference::from_value(&DslValue::from(&value)), but actual FromValue takes owned DslValue, producing E0308. No other errors emitted. Root notified; do not accept this as diagnostic status admission red. Production status method and schema remain unchanged pending intended native failure.

Actual intended native red93006: compiled current root DTO and bothnewtests successfully; selected diagnostic admission test FAILED (0passed1failed26filtered) on overflow-offset, admitted true versus expected false. Handle terminal1/Cargo101. Root reference ownership compileerror resolved. Native/schema fixes may now proceed against real independent neutral evidence; redlog preserved.

After intended red, minimal fixes: existing StagedFormStatus.admitted checks canonical exactu64 text for both counters; checked offset+diagnostic count<=total also rejects overflow, and count<=16. Existing ready/commitArgs/token law retained. Status schema two counter patterns reuse existing exactu64 Token pattern. No other schema rules or working regex anchors changed. Focused Ajv/wire Bun successor terminal0 (2tests pass); native whole arg-schema lib28527 live with nofilter, including root reference DTO and all existinglaws.

Actual whole-library successor28527 terminal0:27 native tests passed,0failed,0filtered. Includes diagnostic_status_counters_match_exact_neutral_u64_and_page_bounds, root source_value_reference_coordinates_match_neutral_shape_and_independent_serde, and existing original/descriptor/native receiving laws. Native runtime3.34s. Focused Ajv/wire green2passed0failed44assertions. Source caps, pin laws and existing test owners unchanged; no extra Rust physical file. Logs generated/status-counter-native-green.log and status-counter-green.log. This establishes the diagnostic alignment and current shared arg-schema regression result; it does not establish all norm forms or firstleaf constructor acceptance.

# Private Native Writer And Root Review

## Actual Byte Retirement R12 To R13

The coordinator read the complete two-test R12 RED, the corrected private writer source, both actual-byte laws, the R13 roster and every R13 failure assertion. R12 genuinely failed with the first initialized byte still equal to 97 and with sealed text advancing under a one-byte-short descriptor grant. The repair makes sealed `String` ownership conversion to `Vec` a separately granted descriptor phase, preserves its pointer and capacity, then zeroes only the actually granted initialized-byte prefix. Physical capacity release remains a later separately granted step.

R13 actually executed 17 cases: **8 passed, 9 failed, 62 skipped**, 0.336 seconds, Nx exit one. All six writer laws now pass, including both semantic scrub laws. The original five admission failures and four of five root failures are unchanged; only the root identity law and existing input-generation law pass outside the writer cohort. This accepts the private writer's scoped UTF-8/copy/scrub behavior, not source installation, resident funding, queue construction, callback publication or whole input retirement.

The live failures remain exact: the queue constructor allocates14336 bytes before admission; a full refusal advances generation257; input and metrics generations wrap to zero; terminal reports true while14336 bytes plus a64-byte payload allocation remain; root busy/unwind/exhaustion cases retain256 slots; and one vector still allocates before root admission. No quota, timing or stack workaround is authorized.

Coordinator read the complete root sequence, private byte buffer, four native tests and implementation-boundary report before approving the canonical `input_` cohort. The prior32-error compile gate is real and has zero executed tests. The next cohort must retain the original queue/root behavioral failures rather than selecting only new writer cases.

## Accepted Staging Scope

The checked root sequence performs one CAS attempt and permanently rejects overflow. EventQueue's new optional numeric field and private work-preflight method remain unused by live producers; the old constructor/backing/enqueue paths remain unchanged. A numeric root tag or byte grant is not a funded receiver, and this staged source does not reopen an empty-constructor/old-enqueue bypass.

The private writer incrementally validates UTF-8 into fixed state and a pending byte, then copies one byte in a separately granted phase. Its final unchecked Vec-to-String conversion has explicit preconditions: exact expected initialized length, no pending byte, complete UTF-8 state and no sticky fault. The source and buffer remain outside the tests' partial-copy panic boundary. The actual original source identity/funded candidate/live guard remains absent from this primitive and is not inferred.

## Superseded Close Defect

The earlier `Inspected` close phase only incremented `inspected`. R12 reproduced that semantic defect and R13 repairs it for the private buffer as described above. This does not retroactively prove the queue/root close or a funded live source.

The current unexpected Vec-overgrant branch retains the allocation and reports a fault, but that is containment, not proof that a real resident permit authorized those bytes before allocation. The future live writer must consume the actual exact-layout/custody authority from its original admitted parent. No enlarged quota, stack allowance, speculative refund, source-copy fallback or general Drop safety follows from these tests.

## Selected Read-Time Hashes

Not a pre/post runtime capture or full dependency closure.

```text
20726662bfdd0a31a9ffc84a03f8fed94181d282069cb39d5a7150b9393c22f3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🦀️.rs
0a58b3313e5564ff63ca974815994412d9a32fddf7815d310f9463ebd66a5885  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/✍️writer/🦀️.rs
7e8acb0297a02a43c197550f04f696bfb47c9bde9b982b7439ec8bb0d39c8f25  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/✍️writer/🧪️tests/🦀️.rs
cc7baad76a87b656fd60a2a2af3fddfb20ba96805e772ca702e20ce9a91186dc  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️enqueue.rs

```

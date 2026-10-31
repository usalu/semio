# Native Six Headroom Accounting Audit

Bounded defining Source read only, reusing retained compiler scope reports. No project/cache census, runtime, GC or Source edit. Current launcher SHAcbdeda5ff5438256ca813b750d19ffb68ccf27508ca2ce56edf78c4fa82dccf2; unit-cache headroom84b5f60b9e76dc7e50810ce0198a92c56892e6d873efe6558101d8a00bc2efbe; unit launchd9cecdaf38760d440cf351b98b85058af51b1cc8eb4b86b2d6893284a747dac9; unit schema6a65c055c0ad8e996e676608da957f60a88392db10029ad2dbd7c92a80ed34da.

## Meaning Of Seven Point Two GiB

headroomEstimateV1 uses newest samples from most-specific command class with at least2samples, nearest-rank90th percentile plus15%margin, else authored warm/cold prior. Bytes are clamped between128MiB and15%of measured volume. Cargo classes distinguish subcommand/target/selected crate set and warm/cold temperature. Thus displayed7.2GiB is an admission expectation/reservation, not a hard bound on build output or guaranteed physical sufficiency; it is consistent with the roughly7.17GiB authored ceiling of a47.8GiB volume. Actual history/class sample responsible was not inspected, so no claim of its exact learned provenance.

Running reservations decay linearly over learned bounded ramp because actual writes already reduce free space. Admission subtracts floor, other running outstanding estimates and older waiting reservations. Image monitoring may invoke existing single-flight cleanup below floor. Source intentionally permits an alone/stranded request after a collection under defined conditions. Neither design nor this read establishes why actual concurrent output exhausted the image; no age/quota/profile/cap change recommendation follows.

## Learning And Failed Attempts

Launcher110 computes growth as max(0,lane growth,whole image free-space decline), so it includes cache/component writes and may overattribute peer writes; reclamation can hide some growth. Headroom release records a sample only when given grown callback and signal is not aborted. No sample or estimate update happens between the failed first execution and the internal retry: both are inside one lane/headroom lease/body.

Concrete source-level documentation mismatch: launcher96 describes learning only completed commands so early failures do not teach small demand, but111 sets completed=true whenever body resolves. Stream execute191 catches OwnedCommandFailure and returns nonzero status; consume197 returns that status normally. Therefore a terminal failed stream command can still resolve the lane body and record partial growth on release. Capture consumer may instead throw and skip a sample. This proves a failure-status/sample distinction gap in the stated learning rationale; it does not prove the present7.2GiB estimate was learned from such failure or caused Native6 ENOSPC. Current still-live Native has not yet released its final sample.

## One Same-Owner Retry

Unit schema outOfSpaceRetries=1. retryWhenFullV1 runs once, calls collect only when full=true, then repeats at most once; only last execution is consumed, and stillFull emits diagnostic when retry is full. Launcher213 invokes the same execute closure with unchanged compiler arguments, environment, selected lane, broker/jobserver and cpuControl while writer/capacity/profile custody remain held. execute189 checks the same control before each attempt and derives remaining milliseconds; stream child uses its signal and canonical invocation environment. No deadline reset, new Original or new lane acquisition occurs. Cancellation still prevents subsequent child execution; the collector callback itself does not receive an explicit signal at this call site, so this review does not claim immediate cancellation of cleanup internals.

Exact current retry behavior preserves child authority and remaining-control checks but is not new headroom admission or a promise cleanup reclaimed anything. Actual zero-reclaimed observation remains authoritative for Native6. No manual cache action, policy amendment, new runtime or terminal/pass inference is authorized by this report.

## Wording And Existing Tests Clarification

Do not classify the resolved nonzero path as a proven implementation bug. Exact launcher96 wording is “learned only from a command that completed: one that failed early wrote little and would teach the estimate that a build is nearly free”; actual flag111 means body resolved, which can include a final stream nonzero. This may be a documentation interpretation of completion rather than a violated specified status contract. No schema rule requiring exit0-only learning was established by this read.

Existing launcher test “a command learns the larger of its lane growth and the image free-space fall, never a negative number” tests only numeric commandWriteBytesV1 against independent Python max; it has no exit-status input. Headroom tests exercise explicit release callbacks, sample/temperature history, cancellation, fairness and ceiling, not launcher nonzero-success classification. These tests do not resolve the intended status semantics. Retain this as a precise documentation/learning-scope question, without a production defect, current-history provenance or ENOSPC-causality conclusion. Existing15%clamp is policy and not a hard sufficiency guarantee; no policy/Source change is proposed.

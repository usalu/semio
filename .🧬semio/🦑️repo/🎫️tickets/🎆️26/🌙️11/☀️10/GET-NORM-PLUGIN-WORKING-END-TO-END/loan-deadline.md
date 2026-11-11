# Resource loan deadline failing a successful `repo cargo test`

## Root cause
Every tracked child loan that expires pushed its error into the failure list of every ancestor scope (`track` in `routing/📥️invocation/🏃️process/🟦️.ts`), poisoning the whole process. The telemetry sampler holds a fixed 2 s loan per sample (`maximumSampleMilliseconds`); on a loaded host (load ~20) a sample outlives it. The advisory sample failure then closed the process and surfaced at teardown as `Internal error: Original resource loan deadline exhausted`, after cargo had already succeeded.

Progress-renewed loans (`createRenewableIssuedResourceControlV1`, idle window plus declared maximum) already existed and are covered by the progress tests; the fixed-window holder was the sampler.

## Changes
- `ResourceLoanDeadlineExhaustedV1` (typed, same message) replaces the plain Error; exported from the routing package.
- `track`: an expired holder loan is the failure of the caller that awaits that holder, never recorded on the shared owner scopes.
- `telemetry/⏱️sampling`: a sample that outlives its loan is skipped; real sample faults still fail.
- `refuseInternalV1`: expired loans are classified as `execution.loan-expired` (en/de in `📖️texts/🔣️.json`) instead of `error.internal`.
- The strict contract stays: a nonrenewable holder that outlives its maximum rejects for its own caller (existing tests unchanged and passing).

## Tests (managed bun, `repo tools exec bun -- test`)
`routing/.../🧪️tests/⏳️admission/loan-deadline.{json,schema.json,ts}`: schema-closed fixture, an independent Python model of the renewal law, progressing holder kept, dead holder reclaimed, capped holder reclaimed, slow sample skipped (success), faulty sample fails. 8/8 pass; expiry and progress suites pass (14/14). `freshness.ts` "open authenticated admission wait" fails identically without these changes (pre-existing, load-sensitive).

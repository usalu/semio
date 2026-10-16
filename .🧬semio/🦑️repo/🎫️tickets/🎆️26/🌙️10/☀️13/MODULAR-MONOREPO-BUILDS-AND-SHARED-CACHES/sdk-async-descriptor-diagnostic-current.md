# Async Descriptor Revalidation Diagnostic

The single authorized diagnostic release operation 50591 completed with exit 0. Its owned Bun child `6881ac19-a532-4330-9083-d03aea376232` returned reason `exit`, status 0, signal null. All six laws and 61 assertions passed in 35.44 seconds. No further runtime operation was launched.

## Fixed Operation and Evidence

The existing `sdk-derivative/📜️script.ts revalidation release` command received `SEMIO_NX_SETUP_PHASE_DIAGNOSTIC=1`, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, and `NODE_DISABLE_COMPILE_CACHE=1`. Actual local `bun test --help` confirmed `--bail=<val>`; the command now preauthors `--bail=1` only for revalidation. Persistent bounded output capture also includes that command. The original 120-second receiver and each 30-second law limit were unchanged. Production Source was unchanged during the diagnostic.

Retained generated evidence:

- `sdk-async-descriptor-fences-diagnostic-release1.log`
- `root-sdk-derivative-child-revalidation-release-6881ac19-a532-4330-9083-d03aea376232.stdout`
- `root-sdk-derivative-child-revalidation-release-6881ac19-a532-4330-9083-d03aea376232.stderr`

| Actual law | Milliseconds |
| --- | ---: |
| Neutral Ajv/SQLite boundary corpus | 73.24 |
| Private authentication and five physical fences | 4540.96 |
| Genuine shared custody and copied/escaped refusal | 1908.72 |
| Mutation refusal at all five boundaries | 15383.57 |
| Detached Error and undefined failure drainage | 8314.37 |
| Successful detached copy before final fences | 5049.63 |

All mutation positions reached their intended refusal. Detached failure rows reported active 0 and primary failure preserved. Successful detached work reported copy settlement before the last three physical fences. The complete suite and outer process settled successfully.

The additional diagnostic emitted actual `cache:tools-store` / `queue-claim-catalog` / exclusive progress. The largest observed elapsed scalar was 1005 ms. No law timeout or sustained stall reproduced. This identifies a resource observed during this operation; it does not establish the cause or holder of the earlier failed operation. Generic heartbeat text and a post-terminal empty descriptor census cannot establish historical custody.

## Earlier Failure Remains a Failure

Operation 13825 remains a genuine release failure: three passes, three timeouts, three late errors, 20 assertions, 95.94 seconds. The first mutation-law timeout advanced to later laws while unfinished callbacks later reached `Live Original process issuance required`. The successful diagnostic does not convert that operation into a pass, establish an external holder, or prove timeout-triggered drainage. Unknown stages from failed operations were not removed or adopted.

## Timeout and Lifetime Contract Finding

The actual shared `library/🏗️build/🧪️tests/📥️invocation/🟦️.ts` receiver holds one received Original until its `afterAll` callback invokes `finish`. Its `run` checks the genuine Original before and after the callback, but it does not itself register every callback promise, derive a per-law signal, or abort pending work when Bun reports its law timeout. Fixture cleanup runs before `finish`, but there is no explicit callback-drain join before cleanup. The earlier observed late callbacks are consistent with this gap. The diagnostic success did not exercise a timed-out body and therefore cannot qualify that path.

`--bail=1` prevents intentional later-law launches after the first failure. It is not cancellation or accepted-operation drainage and should not be described as such.

A clean next test-owner change would provide an actual per-law cancellation scope using the existing canonical `withScriptInvocationSignalV1(original, AbortSignal, callback)` issuer. This creates its private derived identity through the real issuer rather than spreading the Original. An owned timer must abort before the runner can discard the callback, with the same existing law budget; the scope must close births and await all accepted children before fixture cleanup. The helper should register callback promises synchronously, stop admitting new law callbacks after timeout/failure, and join all accepted callbacks before cleanup/Original finish. The exact timeout failure must remain primary, including `undefined` rejection; no failure-ledger clearing, deadline renewal, or conversion to success is permitted. `acknowledgeScriptInvocationSignalStopV1` is only for genuinely handled owned cancellation and must not turn a timed-out law into a pass.

This proposal requires a genuine timeout/cancellation regression proving: no second-law effect after bail, the callback sees the canonical derived cancellation, an accepted delayed close finishes before cleanup/Original retirement, and the same outer Original remains failed. No such implementation or timeout proof was performed in this diagnostic.

## Handback

50591 and its child are explicitly terminal. No full-size copy, Cargo/Rust operation, maintenance, SDK publication, activation, additional regression, or production Source edit was performed. The next execution remains subject to Root scheduling.

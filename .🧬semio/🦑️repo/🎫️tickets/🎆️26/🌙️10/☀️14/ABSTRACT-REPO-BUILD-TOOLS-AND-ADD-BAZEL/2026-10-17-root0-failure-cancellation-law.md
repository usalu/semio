# Root0 Failure Cancellation Law

Read-only Source audit; no new Original, Native operation, protected read or code change.

## Current exact hang

Routing track registers each operation in its owner and all ancestors. settle catches an actual callback failure into failed/failure, then awaits drain(owner) before rethrowing it. drain closes admission and waits all accepted pending work; it does not cancel any signal. Root0 creates no deadline controller. withIssuedResourceControlV1 and withRequestedIssuedResourceControlV1 use this settle path. Their exposed controls hold read-only AbortSignal and no cancel method. withScriptProcessEnvelope similarly owns a private controller accessible only to actual SIGINT/SIGTERM/deadline handlers, not an exported caller cancel API. Thus uncaught failure is currently not cancellation.

Native openLifetime requested0 stays tracked while awaiting gate; its actual abort listener closes the published reader and releases gate. Merely retaining the image and throwing leaves settle waiting. Catching rejection outside await withIssuedResourceControlV1 is too late because that rejection cannot reach the catch until drain completes.

## Existing clean cancellation composition

Both issued-scope helpers accept an actual additionalSignal. A caller can allocate its own AbortController before entering the existing native-assets:helper scope and pass its signal through the existing argument. This narrows cancellation only; same invocation, policy, deadline, capabilities and genuine issued lineage remain. No Original abort API needs exposing.

Inside that callback, await the real work under try/catch. On actual failure call controller.abort(error) synchronously, then throw the exact original error. The catch must wrap awaited work (returning an unawaited publish promise escapes a local catch). Abort happens before routing settle starts drain; accepted open continuations receive their actual inherited cancellation and run their owned close paths. Never infer cleanup authorization from stage logs. Do not acknowledge handled cancellation when propagating a failure; no fresh envelope, forced process exit, pending deletion or asserted closed status.

This initiates existing cleanup; it does not prove success. Any unresolved owner remains retained and its bounded accepted cleanup/diagnostic result must remain truthful. Scope failure and cancellation should preserve the original primary even if cleanup fails. The existing settle failed boolean supports undefined primaries; AbortController.abort(undefined) may create a standard abort reason, but that must not replace the already captured undefined callback failure. Frozen object primaries similarly must remain Object.is-identical.

An alternative existing withScriptInvocationSignalV1 creates a genuine signal-derived invocation alias, but is unnecessary here and changes the invocation object used by exact caller checks. Passing additionalSignal to the current issued resource scope is the smaller legal composition.

## Genuine Source counterlaw

Use canonical Root0 policy at fresh genuine birth, actual issued helper control+owned additionalSignal, actual withNativeOpenLifetimeV1 with a Source-only open/close body, and a real tracked hold. Observe ordered body-failure, abort, actualclose, gate-release, drained terminal with unchanged policy/deadline/capabilities. Exercise ordinary Error, undefined and frozen primary; successful work must not abort; cancelled parent and finite child must not renew authority.

Negative control omits failure-triggered abort, records that actual tracked hold remains pending, then uses the same real test-owned controller to stop it after bounded observation, allowing test cleanup. The observation timer is a test decision window, not a replacement authority deadline. This demonstrates old behavior without leaving a process/hold. Independent schema plus SQLite sequence oracle should compare real emitted event order and primary identity classifications. A cleanup callback rejection must preserve original primary and truthful unclosed diagnostic state. No Native fixture is needed for this law.

## Actual helper Source implementation review

Final withAssetHelperScopeV1 now catches awaited body inside actual withIssuedResourceControlV1 callback, aborts its preexisting owned additionalSignal then rethrows exact primary. No Core/Native Source changed. Independently read semantic RED saved actual gate: open→bodyfailure→rescue→abort→closestart→closeend→scopedrained→originalsettled, rescue true failed expectedfalse. Final reported Source0 exercises8 genuine Root0 received workers with unchanged wire/capabilities/nulldeadline, actual openLifetime+Generated FDs and real terminal PID absence; undefined/frozen primaries exact, success does notabort, cleanup-refusal truthfully unclosed until explicit Source-only fixture FD cleanup, rescueonlynegative. Worker/model source inspected; independent Ajv/SQLite event assertions do not substitute Native authority.

Independently nofollowFD-held all6 final manifest paths (helper/schema/two corpora/ENDElaunch+seed), bounded regular files with identical pre/named/descriptor sevenfield metadata and all bytes/SHA match. Generated/root0-helper-independent-final-hold.json. High reports oldtest0/ordinary0/strict0; no Native failure cleanup acceptance follows from these Source cases.

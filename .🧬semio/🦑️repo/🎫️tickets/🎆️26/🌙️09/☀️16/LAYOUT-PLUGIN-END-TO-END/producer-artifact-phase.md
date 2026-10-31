# Producer Artifact Phase Custody

## Actual Failure and Cause

Canonical setup37477 is the genuine retained failure: initial unused-artifact measurement succeeded, input staging consumed the mutation owner, completed input readback succeeded, then producer completion called the unused-artifact measurement API with that consumed token. Native requireArtifactRetirementOwnerV1 correctly refused Started=true. This was consumption, not an inferred expiry. Exact runtime incomplete DATA and six unchanged retentions are preserved in setup-live-4-partial-state.md and 🗑️generated/setup-live-4-receipt.json. No partial artifact was touched.

## Caller Change

The private createRepositoryDependencyProducerArtifactPhaseV1 holds the genuine original artifact token only before input completion. It accepts exactly one actual completed input receipt returned by the Native stage/observe API. Thereafter measure uses measureNativePlatformProtectedArtifactInputGroupFundingV1 and recheck uses readNativePlatformProtectedArtifactInputGroupV1 under the same actual control/private parent. The defining Native issuer reauthenticates receipt, group, journal, backing and scope; this helper creates no authority and does not restore mutation rights.

All four poststage measurement points in runRepositoryDependencyProducerCompletionV1 use that phase receiver. The outer Source read context recheck also switches to it, so later fences cannot accidentally revisit the consumed mutation token. Initial pre-stage measure/read remain on the strict unused-token API. No fresh mutation owner is recaptured, no Started flag or private map is altered, and no deadline/guard is weakened. The Native defining API and its laws are owned by Rust; this lane owns only the producer caller/helper and caller tests.

## Actual Verification

Schema-first language-neutral cases live in publication/🧪️tests/♻️phase/🔣️schema.json and 🔣️.json. The actual private production helper and completion function are parsed from their defining Source and executed under finite test ports. Independent Ajv and SQLite derive the closed event/refusal cases. Laws cover initial owner use, repeated post-completion measures/rechecks, duplicate acceptance, refusal to reuse consumed mutation rights, foreign control, and the actual caller stage→readback→receipt measurement sequence before its installer phase.

Normal69782 terminated EXIT0:7tests,0failed,35assertions. Its historical filename producer-phase-red.log does not make it a prepatch RED: the request waited for capacity before Root authorized the fix, and the actual run executed patched Source. Its same-byte production hash printed from the parsed Source was a9ee1e01432e96eab46b33ff9d7e8b72c016eb4327e3b9142c351e7ba52b98a0, matching current source metadata. Actualsetup37477 remains the failure evidence. Normal strict4 72281 terminated EXIT0 and includes caller, new caller tests, defining Native platform and Native observation test roots. An own superseded strict request with an incorrect observation operand was cancelled and is not validation evidence.

All test/check temporary environment values are ticket-local. No live setup, Cargo, cache preview, retirement, Source binding mutation or shared-cache repair was run after37477. Future runtime recovery requires a separately authorized genuine owner with fresh state; this Source result does not claim rematerialization or readiness.

Defining Native normal focused51879 also passed7tests/306assertions, including9neutral Ajv/SQLite cases. Its owning report is artifact-owner-lifecycle.md and exact files are artifact-owner-working-files.json. Joint current source gates pass; machine-readable exact log hashes and source packet identity are retained at 🗑️generated/producer-phase-verification.json. Root retains independent review and runtime release.

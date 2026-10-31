# Current Capsule Prevention And Clean Review

Read-only current Source review, 2026-10-29. No Source edit, test, native/container execution, lease or process operation occurred. No new filesystem census or runtime cleanup success is claimed. Existing zero-capsule census and clean terminal results remain historical evidence in capsule-current-verification.md.

## Exact Name Boundary

Current filesystem descriptor RelativePreservationProtocol declares capsulePrefix=.p, capsuleDigestHex=8, payload=p, intent=i, receipt=r. Retirement Policy declares markerPrefix=.d. Current source-opacity helper derives both prefixes and digest length from those schemas, splits both / and backslash separators, and requires an entire component equal to prefix plus eight lowercase ASCII hexadecimal characters. Thus only .p[0-9a-f]{8} and .d[0-9a-f]{8} match; uppercase, short/long digests and suffix near-misses do not. Emitted input exclusions cover the exact named file plus directory descendants. Source enumeration rejects matching components before descent and skips symlink entries.

.prettierignore, .prettierrc.json, .package-cache and .package-cache-mutate do not match. Capsule retirement has no broad startsWith(.p) deletion: it considers exact capsule/journal names only under qualified node_modules Source binding parents. Package cache locks inside protected Store internals are outside that domain selection. Legitimate near-misses remain Source or protected cache data, as appropriate.

## Owner And Publication Integration

Current dependency storage exposes retireJavascriptDependencyPreservationsV1(workspace,store,control), checking exactly three arguments and a live Store frame from its private WeakMap. requireJavascriptDependencyStoreFrameV1 verifies actual Store identity/workspace, issued Original/control refinement, active resource control, repository dependency Store read custody, build EX and any active bindings EX. Plain current-looking objects cannot grant this authority.

The retirement operation acquires bindings EX, keeps it through descriptor settlement, and rechecks complete current Bun workspace ownership and Store custody. It reads the qualified current bindings/ready receipts, pins their generations, and verifies receipt fingerprint and ready generation before deriving exact binding parents. Progress yields recheck custody and cancellation. Descriptor bounds are 4,096 capsules, 16,777,216 entries, depth128, progress every256 entries and 16KiB records.

The actual current publication path still executes `if(built.state!=="current") await ...publish; await retireJavascriptDependencyPreservationsInStoreV1(...)` before clearing/releasing bindings EX. Both successful publication and the current no-op sync therefore reach retirement. An earlier ensure fast path that returns without entering this path is not a retirement guarantee.

## No-Follow And Restart Boundary

Retirement uses withArtifactRelativeDirectoryV1 and held directory/file operations. It verifies exact node_modules link target/identity repeatedly, accepts an exact i/p/r initial roster, closed producer intent and preservation receipt, owned generation target, intent digest, same-device capsule and exact original payload identity. Unknown roster, foreign owner/journal, corrupt metadata and changed payload are retained.

The descriptor walk observes directory entries without following payload links; links are removed as entries and external targets are not traversed. Directory ancestry and root mutations use the existing descriptor capabilities. Removal rechecks entry kind/device/inode/mode and appropriate full identity before each unlink. Cancellation/resource control checkpoints surround bounded work and yields; this does not claim immediate interruption within a synchronous filesystem call.

Before payload mutation, a durable .d journal records and verifies original intent/receipt bytes and hashes, capsule/payload inode facts, source/target and current ready identity. It is synced before destructive work and remains through payload-empty, intent-removed, receipt-removed and capsule-removed boundaries. Restart can complete original metadata removal under the same qualified owner even after the active generation advances. Final capsule/journal removal checks exact identities and journal bytes. General preservation itself still declares deletePayload=false and followPayload=false; dependency retirement owns disposal after readiness.

## Current Clean And Neutral Registration

Current CleanScript general workspace path awaits dependency retirement after its normal workspace report and before owned cache prune. Dry mode returns before dependency Store acquisition. It creates a phase control from the incoming invocation, enters withJavascriptDependencyStorageV1, and passes the actual returned Store/control to retirement. Cache pruning is independently awaited with the same incoming invocation. Specialized coverage/test/empty/marker-only modes return through their own owners and should not be described as the general capsule retirement entry.

The actual retirement suite statically imports its neutral JSON corpus, validates cases through Ajv, and compares eligibility to an independently implemented SQLite predicate. It includes real outside-link preservation, four durable cancellation boundaries, foreign-owner/changed-payload/unknown-child retention and forged Store refusal. The workspace-cleanup suite inspects the actual CleanScript body through independent TypeScript/esbuild extraction and validates neutral delegation/dry/cancellation/Original cases with Ajv/SQLite. These are existing tests read here, not newly executed results.

Current library/🧪️tests/🔬️workspace-contract/🟦️.ts directly imports the retirement suite. The repo-lib TypeScript project explicitly lists the retirement, cleanup-command and source-discovery suites in runtime Source entries; cleanup sources are covered by its inputs. Static registration remains present. Full suite success cannot be inferred from registration.

## Verdict And Remaining Evidence

No concrete regression found in this reviewed prevention, exact-name, authority, no-follow, cancellation or general clean wiring. The last reported actual repeat clean was idempotent; the old optional repeat-publication attempt was cancelled before admission. A new successful publication/no-op sync followed by a no-follow Source census remains the strongest outstanding end-to-end prevention proof. This audit did not launch it, rerun the neutral suites or inspect protected cache contents.

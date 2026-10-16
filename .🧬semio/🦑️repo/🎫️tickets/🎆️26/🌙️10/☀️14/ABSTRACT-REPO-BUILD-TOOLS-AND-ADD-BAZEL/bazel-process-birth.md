# Bazel Authenticated Process Birth

Production Source is held. This report distinguishes actual runtime evidence from pending backend cases and current SDK/ALL qualification.

## Boundary

Bazel semantic action keys contain the task manifest and declared inputs. The current serialized Original, deadline, cancellation endpoint and token are not injected into semantic action environment or cached launcher bytes. A private prebirth launcher obtains the unchanged current wire only from the live parent rendezvous while the parent retains genuine workspace EX and managed tools/dependencies/products custody.

The parent authorizes exact task identity, SHA-256 of the entire manifest, kind and resolved output/dependency argv from its current admitted plan. The closed birth schema permits only action/launcher/launch. Manifest and runtime reads are stable nofollow regular single-link reads bounded to 1 MiB in 64 KiB chunks. The existing control-message bound remains unchanged. A finite 10-second handshake is intersected with the original deadline. Schema validation precedes connection creation. Parent disconnect cancels the owned child; the connection is retained until settlement. Strict child receipt requires its actual OS wire to match the runtime expectation, and the manifest digest is rechecked before task execution. No general missing-wire fallback, structural Original clone or new policy clock is accepted.

Output/dependency paths are captured and authorized as absolute paths in the actual Bazel execution root before child dispatch. The task child retains that actual cwd. This fixes the concrete prior wrong-cwd failure; source-workspace bazel-out symlinks are not used for output authorization. Cleanup attempts the bounded termination receipt and preserves an arbitrary primary failure.

## Actual Evidence

- Initial actual action refused missing OS Original wire (integration2690). This was the production RED.
- First birth implementation completed three real Bazel actions but parent restore failed ENOENT because the child changed cwd to the source workspace (24397). It is not a pass.
- Corrected monolithic integration progressed through real build/restore but exhausted Original120 before completion (50144). It is not a pass.
- Existing real snapshot contention now uses genuine Original plus issued additional signal, retains SQLite contention, observes awaited Original progress without replacing its ports, and passes both deadline and cancellation cases.
- **Cache phase76114 terminal0:** `generated/current-bazel-cache-phase.log`, 8 pass, 8 skipped, 90 assertions, 87.45 seconds. The actual cache case performs cold build, same-Original output restoration without reentry, source-change invalidation, and another genuine Original with a different (1 ms earlier) deadline. The final invocation restores SECOND with execution count still2. Actual Bazel reported an action cache hit. This proves cross-Original semantic reuse for this real task; it does not qualify continuous launcher or every target.

The nine permanent integration cases are cache, nested, uncached, birth, services, refusals, cancellation, configuration and offline. Each case has an unchanged genuine Original120; the ticket selects one case at a time to avoid stretching a monolithic operation beyond its original budget. Source laws execute alongside it. The current installed Nx parity case is intentionally excluded while SDK is unavailable. Remaining cases have not yet been run after the split. No current full backend pass, current installed SDK or accepted ALL pin is claimed.

## Source Hashes Captured After Cache76114

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`: `ae8abd41855fcf8f756f8d69829cdbb784e674d50721e7ee8c6802242cd054ab`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧬️schema/📥️birth.json`: `9376eb483a17149a9bdb28f7d908cac65847f14bdc5eb46678b7bba3e20a42de`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts`: `6a4c3a0f788573d0715fdcf3a17745c0b7a3dc4e4487faf98b393cc3b6b2a7ad`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🧫️fixtures/🔣️.json`: `f70abf9cb160325839d8d809a63c0f8defb2e18ba2fa9b00d6604d3a01417bee`

## Repeatable Case Isolation

After cache76114, the test-only case setup now uses a fresh eight-character directory suffix. Resetting a stable fixture execution counter while retaining an earlier semantic action cache would otherwise make a repeated cold assertion depend on prior runs. Each case retains the same project root across all of its own builds, including the cross-Original cache check. This changes fixture identity between cases/reruns and does not alter production cache keys, cleanup caches or bypass a guard. No additional runtime was launched during SDK custody.

## Permanent Aggregate Lane

The permanent build TestScript invokes the entire Bazel Bun file with scriptInvocationBudget(original,300000). Individual ticket phases do not prove that this aggregate fits its existing budget; cache alone measured85.47seconds. No aggregate timeout is inferred here without actual execution. Once remaining phases are measured, public registration and total runtime must be checked without increasing an existing Original. No production route was changed during SDK Source hold.

# Execution-Target Relay and Selection Fence

## Scope

The goal remains active. This work closes the transport admission seam for the three protected, document-scoped execution-target reads; it does not establish installed plugin activation or claim end-to-end Map editing.

Concurrent server work added the manifest, component, descriptor routes and trusted catalog asset accessor. This slice preserves that work and adds the complementary browser relay admission, exact byte-length delivery, bounded resource ownership, and final server selection fence.

## Changes

- The browser relay admits only query-free POSTs for the exact manifest/component/descriptor route. Scope IDs use canonical `encodeURIComponent` spellings; encoded separators, controls, percent recursion, unknown assets and path suffixes are denied.
- Target requests and manifests are bounded to 8 KiB, components to 64 MiB, descriptors to 4 MiB. General relay response limits remain 1 MiB. At most two target requests can be active, within the existing global request cap.
- Returned `Content-Length` is the actual retained response length, and responses are `no-store`. The browser verifier no longer loses the required length header at the relay boundary.
- Target requests have an 8-second hub deadline and a 9-second relay deadline, inside the existing 10-second browser request budget. The relay deadline covers request-body reading as well as upstream fetch/response reading. Abort cancels a slow request reader and the upstream request; finalization releases the target slot and closes the upstream controller.
- Server selection captures directory revision before descriptor/checkpoint reads. After validating lease fields, it revalidates the subject and requires the same directory revision immediately before returning the retained selected bytes. Scope binding gates remain held for the selection lifetime.
- Asset reads repeat `DocumentOpenIntentV1`; they never accept a receipt or package/hash/path selector. Across separate reads, plan consistency remains the private browser verifier's responsibility. A catalog rotation can produce a new manifest that the old plan must reject, rather than being a server denial by itself.

## Test-Driven Evidence

Registered Bun/Nx target: `os-hub:execution-target-relay-check`.

Observed RED transitions:

1. The neutral manifest route was rejected by the old relay allowlist.
2. The server source lacked the final authorization/revision fence and aligned deadline.
3. The relay accepted noncanonical percent-encoded unreserved IDs.
4. A pending slow upload ignored cancellation.

After fixes, the actual Bun HTTP relay test passed: 17 route vectors, six response vectors, schema-validated four-case native fence corpus, and five resource/cancellation checks (32 checks total). Runtime output confirmed exact response bytes and two-request capacity. The fixture is independently checked with AJV. It carries a strict valid open intent, including requested surface and client instance; the fake upstream checks byte-for-byte preservation. Colon and Unicode route vectors deliberately prove transport routing, not hub scope authorization.

The runtime responses include components/descriptors above the old 1 MiB limit, manifest/descriptor overflow, a successful target response after the general two-second deadline, saturated slots with two greater-than-1-MiB responses, an oversized request rejected before proof consumption, cancellation after upstream admission, and subsequent capacity/proof recovery. A pending upload is also cancelled directly through the production bounded body reader.

The existing browser proof-ratchet runtime regression was added to the same target and passed after the relay changes.

## Native Verification

Registered exact target: `os-hub:execution-target-native-check`. It selects the existing live route law and the new neutral race law `execution_target_selection_final_fence_matches_neutral_races` (unchanged, directory advance, session revocation, cancellation). Receipt `🗑️generated/execution-target-native/exact-cargo-laws-vhjC6H/00` terminated on build timeout/SIGKILL while waiting for the shared build lock behind Home; it never ran a native law. Do not queue another duplicate lock wait until that active Home build releases its cache. Source/runtime relay checks are not a substitute for the pending native selection proof.

The earlier frozen GIS binding native run `exact-cargo-laws-8ivhr6/00` failed before law execution because Stdio's glTF contact-graph-degree Rust input was absent during compilation. Both the current mount and input exist when rechecked; no old taxonomy path was restored. That run is terminal, and its warmed target is shared with the Home process build.

## Files and Handoff

Fresh selected-SQLite execution attempt: `🗑️generated/execution-target-native/exact-cargo-laws-7VfbyW/00`, session 90466. The 32-check runtime prerequisite passed. The native build remains active in Stdio and has no terminal build receipt or law verdict yet. This run owns the space-public-boundary cache; the six normalized-presence native laws follow only after it terminates.

The Home warm-cache build released on 2026-09-05 after its own build timeout, before route execution. Root launched the registered exact selection/asset target with selected SQLite features and full 24-hour build/orchestration budgets, receipt `🗑️generated/execution-target-native/exact-cargo-laws-nBi3wx/00` (former session 83711). Its 32-check relay runtime prerequisite passed again. That native process was interrupted by the later session reset: its process is absent, it has build stdout/stderr but no terminal build receipt or native-law verdict, and it must not be described as active or successful. Selected SQLite is the intended concrete journey backend, not all-backend qualification. The space cache remains reserved for a fresh root selection run, then the six Hub normalization laws, then returns to Home; no unrelated build was stopped.

- Hub Rust package script, router and project registration.
- Hub `🪪️execution-target-relay-v1` JSON Schema and neutral fixture.
- Hub binary selection final fence, deadline and native race law.
- Launch seed entries 411.10955 and 411.10956; generated-launch verification is coordinated with the Home executor.

Scoped staged and unstaged whitespace checks passed. Unrelated ticket-report whitespace in the shared worktree was left untouched. No git state was modified.

Next required closure: native route/race execution, private plan-derived browser activation including its complete JavaScript/core-Wasm closure, a real persisted Map edit/reopen journey, then atomic three-member Map publication rather than sequential compensation.

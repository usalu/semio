# Nx Mapping Custody Blocker and Declared Backend Alternative

2026-10-28. Read-only repo leases status, fleet status, cache status and bounded native-storage source/receipt/ledger reads. No peer interrupted, no lease evicted, no receipt or intent changed, no cache/image repair, no Bazel process or graph launched. No environment, credentials or complete process command lines were inspected.

## Exact Resource Identity

Mapping resources are defined in native-build/storage/mapping.resource as protocol.resource + SHA256(input.root). Hashing the declared roots produces:

- `.cache/b/n/m`: `9d349bf4f2ee76d326b2ebe63ebb888383c7e49a28632d6331331654a7ac7600` — exact blocked key.
- `.cache/b/g/m`: `5874ac7abe630d47eeee1027ca2dda6e7939dd68569eb4474223481812b6b007`.
- `.cache/b/t/m`: `80ee692605172cb7d24fd84ae210d365d94998bd804cccfbd29e154bf47ab2fd`.
- Compiler fixed root has a different mapping key as well.

The blockage is definitively the Nx native storage partition, not the graph or Rust compiler partition. The authored budget policy maps nx to `.cache/b/n`, capacity1610612736bytes. Its image receipt and mounted root exist; bounded storage-events ledger read contained1095events, ending with namespace-created/committed then retiring/retired events. This confirms a real owned partition, without treating the ledger as authority to mutate it.

## Live Custody Evidence

Repo leases status observed PID10518 (foreign session f9862ee2-2e3c-43fc-9b16-d905a75045d7) holding exclusive mapping9d349... for2379seconds while simultaneously waiting shared on the same mapping for2373seconds, queue position1of3, reported waiting for10518/328. Fonts regression child PID69374 waited shared on the same key for543seconds, position3of3, behind10518/328. PID10518 also retains its Nx daemon-slot owner and shared fixed-image handles. No inference that the peer is safe to stop follows from these facts.

Fleet status at21:56:03 independently classified PID10518 blocked on that mapping for about40minutes and the Fonts run blocked behind it. Fleet reported no graph builder or graph waiters at that instant and0live Bazel servers/0of12held slots. Leases reported0formal deadlock cycles even though the explicit same-PID exclusive-holder/shared-waiter pattern is present; this report does not override the lease classifier with an invented cycle claim.

The mapping source has scoped reader/writer authority and AsyncLocalStorage frame reuse; otherwise a fresh shared request is queued. The observed ownership pattern is consistent with a reentrancy/continuation custody defect, but the source call stack of10518 was not inspected, so the exact initiating defect remains unproved. Preserve its actual owner and diagnostic state.

## Legitimate Alternative Backend

The declared Bazel adapter is independently owned storage with the same source/target authority. Its `withBazelStorageV1` opens only tools, dependencies and products through withRepositoryCacheStoreV1, verifies actual reservations and exact declared partition roots, and derives Bazel executable/install/module/workspace/output/action/server roots beneath those stores. It does not open the Nx `.cache/b/n/m` partition. Outputs/server roots belong to products `.cache/b/p/m`; tools and modules belong to `.cache/b/t/m` and `.cache/b/d/m`. Their mapping roots are distinct from9d349....

`runBazelTasksV1` receives the generated BuildGraphV1 and requires requireLiveBuildGraphV1 plus the issued invocation/control; it captures/verifies the build source closure and runs the same declared target task plan. Thus selecting `repo --tool=bazel run @semio-tech/framework-os-dev:activate-layout-react-prototype` is a legitimate declared backend choice, not a way to acquire or mutate the peer-held Nx resource. No lease bypass, private cache root, source-authority substitution or custody repair is proposed.

This is source-based independence for the Nx-specific wait, not an execution-success promise. Bazel still shares real graph/source admission, dependency stores, host capacity and compiler resources. It may queue on those, require its declared module setup, or refuse a product/host constraint. The build skill notes the product path has not yet been exercised through this backend. The adapter enforces ASCII host storage paths on macOS; the current checkout/storage prefixes are ASCII, while emoji source names are separately supported. Any refusal must remain under its actual owner and must not trigger manual storage repair.

## Decision

A coordinator may try the declared Bazel activation as an independent owned backend, preserving all source checks and ordinary admission. Do not cancel/replace the peer Nx owner, evict its lease, detach an image, alter an intent/receipt, move the cache, or claim the alternative succeeded until its actual result is observed. No alternative backend operation was performed by this reviewer.

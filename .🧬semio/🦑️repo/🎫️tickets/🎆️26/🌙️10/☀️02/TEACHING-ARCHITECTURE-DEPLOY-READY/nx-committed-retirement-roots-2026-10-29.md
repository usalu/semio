# Committed Namespace Retirement Root Review — 2026-10-29

Read-only causal review requested after genuine inventory76716. No Native job, retirement, journal-content read, cache edit or Source change was performed. The inventory is a timed DATA observation, not retained cleanup authority.

## Actual Observed Coordinates

Inventory sequence71 reports pool root /home/deck/semio/.cache/b/n/m. The current canonical ImageProtocol.mountDirectory is m, so nativeImageRootV1(/home/deck/semio/.cache/b/n) resolves to that same coordinate. All14 committed namespace allocations have kind compiler-state, an existing child at the observed time, and a child directory strictly inside the pool root. Example allocation0038b916d83ddfd29e106de86b09fd5b owns /home/deck/semio/.cache/b/n/m/6920fefe55a885541958a89e8eca20d431ad622b/native. The reserved absent child remains excluded and protected.

## Current Source Binding

- Nx graph owner lines54–58 selects the exact child directory, refuses duplicate/missing ownership and returns false for noncommitted rows. Its pool policy at line94 captures root:base.root.
- Repository store body line107 defines base.root from image.root without a separate logical alias. Its retirement lines89–91 retains the original issued owner and passes the recovered reservation through the partition facade.
- Partition opening line74 creates its wrapper by spreading the actual canonical image; it preserves root and port. withRepositoryPartitionOperationV1 line89 resolves the private wrapper to scope.image. Retirement line98 passes that exact canonical image to retireNativePlatformFundedChildStorageV1.
- Native storage custody lines83–87 requires a canonical policy root and validates resolve(backing.root) equals that root. Recovery lines156–157 binds the actual durable allocation and verifies its charged namespace before returning a privately held reservation.
- Lower platform line78 still requires child, committed phase (or its explicit absent-retiring protocol), compiler-state kind, reservation.backing.root===parent.root and policy.root===parent.root. No predicate was changed.

Therefore current Source has no demonstrated logical-root/native-root mismatch for the observed committed cohort. Under its existing recovery and wrapper invariants, policy root, backing root and canonical parent root all derive from the same actual image root. No lower-retirement repair is justified by this inspection.

## Proof Limits And Next Action

The inventory projection does not include a live recovered reservation, creator claim, current namespace lease or complete child identity. This review cannot prove which committed row is the first legitimate idle victim, current reader absence, accepted cleanup admission, or successful physical destruction. Those remain the existing strict idle selection and actual guarded runtime operation. Earlier38256 guard attribution remains unproven; the new bounded failure cause must be retained if it recurs. A clear static path does not replace that runtime proof.

Frontend owns the Native funding preflight and legitimate-victim path; Core and graph Source remain untouched here. The parent may run the next coordinated genuine proof after that candidate is ready.

Generated nx-committed-retirement-root-facts.json records source hashes and the detached inventory checks. Source hashes identify this review point, not immutable future Source or a product receipt.

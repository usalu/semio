# Sealing and Startup Review

## Current Native and Borrowed-Map Checkpoint

The subsequent root-owned kernel `canonical_` execution passed **18/18**: all eleven sealer laws, both load/seed regressions, and five additional canonical tests. This now supplies native borrowed-map execution evidence, including tiny grants and the depth/rebinding adversary. It does not establish per-turn wall-time or any app's full candidate preparation.

The coordinator executed the original seven sealer laws successfully and three member-reservation/publication laws successfully; exact results are in `📓️coordinator-exclusive-compiler-queue-2026-08-27.md`. The private startup-proof test reached compilation but its hostile fixture was outside the owning module, producing fifteen private-access errors; the fix must retain the production authority boundary.

The recursive-map extension exposes typed borrowed scalar/source/array/object nodes. Store alone projects the immutable boxed edit's borrow into retained native iterator frames; the encoder field precedes the edit field so all frames drop before that allocation even on unwind. Every call checks the exact root address, checkpoints carry only replay data, and cancellation retires iterator frames before transferring the edit to its retirement owner. Native map iterators avoid repeated `nth` or key-range scans. This is a reviewed source design, not a runtime or 8 ms pass.

The executor added four map regressions for nested/empty maps, 4,407-byte Unicode keys, tiny grants, byte-for-byte serde parity, worker transfer/replay, all-phase cancellation, exact-root rebinding, and depth exhaustion. A review caught that an indexed subtree could otherwise allocate its own depth budget beneath borrowed parents; the fallback now uses the remaining combined 64-frame budget. The full eleven-sealer-law snapshot is queued in the coordinator's kernel `canonical_` run together with the repaired cold-seed/load/reset tests.

## Scope

## Bounded Collection and Generator Follow-Up

The new canonical reader reuses the private encoder over an exact immutable typed Arc, exposes bounded byte chunks without Store sealing authority, and returns its root only after every borrowed frame is empty. Three native reader tests are authored and queued; the executor's source/oracle gate reached 702. Native reader coverage must not be inferred from the earlier eleven sealer tests.

Flow copying revealed a separate key-comparison hole: ordinary BTreeMap insertion can compare valid multi-kilobyte common prefixes in one step after bytewise copying has finished. The approved shared ordered-map design uses immutable fixed-fanout pages, retained comparison/path-copy cursors, pointer-sized shared slots, and explicit last-owner retirement. Its source design is `📓️flow-ordered-collection-seam-2026-08-27.md`; no completion claim is assigned to it yet.

Generator slider preparation will retain an unchanged immutable generation root rather than clone its arbitrary nested JSON values. Its generation-specific mutation paths remain a separate retained-work obligation. The shared Flow typed copier and scalar parameter command are being implemented in separate regions from the completed label codec work.

## Initial Scope

The sections below preserve the initial coordinator source review. The coordinator now owns the exclusive compiler lease; no browser runtime pass is implied. Executors own Flow preparation, Store canonical/Generator parameter work, and plugin/child publication respectively.

## Canonical Sealing

The new Store-owned canonical encoder visits typed nodes and emits at most 256 actual bytes per call. The retained sealer counts canonical bytes, hashes the exact framed canonical stream, copies bounded identity fields incrementally, and produces a private token bound to the exact Store authority and moved edit/post-root allocations. Commit validation no longer repeats whole-edit serialization.

The review found a checkpoint upper-bound arithmetic issue: the literal 1,024-byte overhead allowance was smaller than a maximum-length framed edit ID plus all three retained identity copies. The sealer owner is replacing that allowance with a derived bound and adding a boundary regression. Actor and optional group IDs are capped at 256 bytes when Store mints authority; same-length semantic comparisons therefore remain bounded.

Required runtime gates remain: large semantic Unicode content versus the independent canonical oracle; arbitrary small grants; every-phase cancellation; exact terminal retirement; stale/forged checkpoints; exact allocation and authority binding; and replay across transfer. Padded wire ingress alone does not satisfy large semantic-content coverage.

## Startup Proof Join

The plugin registry now registers app-owned factories before validating their declared proof catalogs. Custom declarations carry a compiler-checked concrete factory witness, whose owner, TypeId, type name, controller, schema, tool, and execution contract are joined to the live ActionBus admission. Generic bounded proofs remain a separate path and cannot replace a registered custom factory.

The old literal-only catalogs must all be converted to their actual factory split, including apps with multiple direct/host/retained factories. Flow owns its own split; the startup executor owns the remaining catalogs. A real CAD app constructor and bounded close, adversarial authority tests, Wasm compilation, and fresh browser activation are still required before closing the observed browser startup failure.

## Shared-Tree Build Coordination

Flow's latest compiler attempt was briefly stopped by an Nx duplicate-project diagnostic naming a malformed repo coordinator package directory. A subsequent read-only check (`ls -lab`, complete unignored file listing, and scoped Git status) found only the correct directory. No peer directory was removed or renamed by this fleet; the executor was told to retry canonical Nx validation with its warm ticket target.

No all-app, latency, or zero-dependency completion claim follows from this source review.

## Rerun Evidence

The coordinator reran `NX_DAEMON=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'` after the typed factory witness and Store sealer source landed: exit 0, 618 self-tests clean. This exercises the source verifier's own fixtures, not the new Rust sealer or real app constructors. The real command/factory corpus must be audited again after the remaining catalog conversions finish.

After all custom catalog conversions and the compiler-witness parser/fixtures were ready, the same canonical Nx self-test passed again: 645 self-tests, 33 exact-factory proof owners, 254 custom rows, and 31 genuinely generic rows. The source check now rejects missing or mismatched compiler type witnesses, including a same-owner/different-factory substitution. This is the latest source-test result; it supersedes the earlier 618 count.

A full census attempted during those edits failed in the changing synthetic exact-app-owned-route fixture before it could write a report. It was not counted as a pass. A fresh full-corpus run was started only after the updated 645-test gate passed.

Flow's native rerun subsequently ended with two upstream stdio visibility diagnostics whose fixes were already present in the shared files. The executor handed the exclusive compiler lease to the coordinator for focused Store and plugin tests. No Flow-native or app-runtime pass was inferred from the upstream module compilation.

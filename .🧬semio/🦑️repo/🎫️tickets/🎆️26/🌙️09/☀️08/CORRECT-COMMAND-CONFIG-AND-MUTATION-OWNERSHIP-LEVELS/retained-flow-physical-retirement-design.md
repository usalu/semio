# Retained Flow Physical Retirement Design

**Date:** 2026-09-13  
**Status:** The bounded Flow direct/PagedList-frontier and ordered-key milestone is green; complete nested-owner and caller acceptance remains open.

## TDD baseline

- `retained-flow-physical-neutral-red-4.log`: the registered Bun/Nx neutral route exited 1 after 22.9 seconds because the source-law oracle found the old LinkedList/truncation implementation instead of the required physical owner.
- `retained-flow-physical-native-red-1.log`: the registered Bun/Nx native route exited 1 after 1 minute 6 seconds. Replication and neural packages compiled; the Flow law failed to compile exactly because `FlowRetirement::from_owner`, allocation demand/reserve, local allocation ledger, and next close demand do not exist yet.

Earlier `neutral-red-1` through `neutral-red-3` are launch diagnostics only: the first used an obsolete Nx path, the second used the ticket directory rather than the workspace project root, and the third exercised Bun directly against an incompatible Nx plugin loader. They are not feature evidence.

## Source finding

`FlowRetirement` currently owns a `ManuallyDrop<LinkedList<FlowOwner>>`. Every push allocates a list node and every pop frees one without a physical release entry. Its direct `Bytes`, `Strings`, `Widgets`, `Specs`, `Neurons`, `Synapses`, `Previews`, and `Layout` arms retire logical contents and then drop the remaining `Vec` or converted `String` capacity without reporting that allocation. `Bytes` reports `min(maximum_bytes, len)`, which is logical truncation rather than allocation release.

The delegated owners repeat the same defect. `protocol::value::ordered::Retirement` used another `LinkedList` and truncated a converted key by logical length. It now uses an allocation-free inline frontier derived from the AVL-height invariant and releases a key's actual capacity atomically. `neural::ValueRetirement` now forwards ordered-key demand and reports/releases the actual capacities of its direct byte, string, channel, and field vectors, but still owns an unaccounted `LinkedList` cleanup frontier and unaccounted `BTreeMap` nodes.

The generic `ErasedSnapshotRetirement` contract currently exposes only `close_step(maximum_items, maximum_bytes)` and `terminal_is_empty`; it has no cleanup-workspace allocation admission or next physical release demand. Several Flow callers still manufacture `4096`, including the Flow cold loop, selected-copy test cleanup, Flow editor tests, and Generation3d snapshot retirement.

## Accepted owner model

The shared primitive has these independent quantities:

- **Local allocation admission:** grants construction of one cleanup-frontier metadata or payload page. It records the allocator's actual capacity. It is a local owner ledger and is never described as process allocation credit.
- **Logical close work:** one caller item opportunity advances at most one child or structural transition.
- **Physical close permission:** the caller byte grant must cover the complete next allocation. A smaller grant leaves pointer, length, capacity, frontier, owner, and ledger unchanged.
- **Physical release observation:** reports the actual released capacity exactly once. It never reports logical string length or splits one allocation across calls.

`FlowRetirement` retains one root inline. Its continuation frontier is `PagedList<FlowOwner, usize::MAX>`. `usize::MAX` is only a checked logical ceiling; `PagedList` does not reserve it and allocates one finite metadata or payload page per exact request. The earlier proposed 256 bound was rejected because no proof establishes that every admitted recursive Flow, ordered, or neural value has at most 256 simultaneously live continuation owners. Before destructuring a root, the Flow cursor computes the exact required frontier capacity. `next_allocation_bytes` reports one page demand and `reserve_allocation` admits one page. Zero or subexact refusal leaves that recursive root untouched; an allocator failure remains sticky and the retained root stays available for a later exact retry or close. This proof currently covers the single-root `from_owner` path. The public multi-root `push` path still allocates synchronously and forgets a rejected owner on allocator failure, so it is explicitly outside this milestone.

Direct strings and collections use one reusable physical backing cursor:

1. retain the original allocation and its actual capacity;
2. retire child values one at a time;
3. when logical children are empty, expose `capacity × size_of::<T>` through checked arithmetic (or `String::capacity()`);
4. release the full allocation only under an adequate caller grant;
5. set the backing to its zero-capacity empty value and subtract the exact ledger entry;
6. reject duplicate release and require an all-zero terminal witness.

The frontier pages are retired only after the inline root and every child owner are terminal. Their PagedList metadata and payload allocations are then released with the same exact-demand protocol, so cleanup cannot recursively allocate an untracked owner to clean another owner.

`protocol::value::ordered::Retirement` now uses a fixed inline `[Option<Owner>; MAX_AVL_HEIGHT + 3]` frontier. The bound follows its depth-first sequence: each tree level contributes at most one pending sibling, while the active node contributes at most entry, key, and value stages. This frontier needs no cleanup allocation. Its UTF-8 key backing blocks under a subexact byte grant and releases `String::capacity()` exactly once. Flow forwards the implemented nested demand, `Blocked` result, and released bytes without clamping or inflating the caller grant. The surrounding `Arc<Node>`, `Arc<Entry>`, `Arc<String>`, and `Arc<Value>` allocations are not yet in the physical ledger.

## Cold boundary

`FlowRetirement::retire_cold` services the exact local allocation demand and next physical release demand. Ordered-map retirement also uses its true next release demand. Two synchronous helpers remain incomplete: the ordered update-cursor close helper and neural `retire_value_cold` still pass a fixed 4096-byte grant. They can stall on a retained allocation above 4096 and must be repaired at their own cold boundaries. Retained callers cannot use these helpers as grant adapters.

## Schema and tests

The language-neutral contract is in:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts`

AJV is the independent third-party schema oracle. The contract enumerates all eight direct owners, all ordered/dynamic/frontier categories, the above-4096 threshold, and the checked-capacity, child-first, unchanged zero/subexact, exact-once, frontier-last, zero-ledger, and cold-demand laws.

The native RED laws are in `.../🧪️tests/🧵️retained/🦀️.rs`. They construct every direct backing with actual capacity above 4096, preserve its local ledger under zero and subexact grants, require one exact capacity release, and require terminal zero. The fixture law requires exact frontier page admission before destructive decomposition and observes the final frontier backing release.

## Current native evidence

- `retained-flow-physical-diagnostic-2.log`: registered Bun/Nx route completed successfully. The two Flow laws passed: all eight direct collection/string variants and the admitted fixture frontier.
- `retained-flow-ordered-coherence-3.log`: the replication package compiled the new inline ordered frontier and passed `ordered_physical_retirement_uses_inline_frontier_and_releases_exact_key_capacity` 1/1. Neural and Flow packages compiled in the same route. The Flow fixture then exposed a delegated demand defect: neural reported one byte while its ordered child required its key capacity. The source now delegates that demand and preserves the ordered `Blocked` result; the corrected fixture result still needs a fresh focused run.
- The second focused run also exposed Nx project-graph `ENOSPC` before continuing to its successful test footer. Only this task's disposable `ticket/🗑️generated/nx/flow-physical-*` workspace-data directories were removed. Shared Cargo caches, locks, and foreign processes were untouched.
- `retained-flow-physical-native-accepted-2.log`: the final registered Bun/Nx native route exited 0 in 25.2 seconds. Replication passed 1/1 ordered physical law with 271 filtered; neural passed 2/2 physical laws with 51 filtered; Flow passed 2/2 physical laws with 38 filtered. This covers exact ordered-key capacity, direct neural byte/string/channel/field capacities, delegated ordered demand, all eight direct Flow variants, exact Flow frontier admission, zero/subexact immutability, terminal-before-grant behavior, exact-once release, and terminal zero for those tracked allocations.
- `retained-flow-physical-neutral-green-1.log` records an intermediate route failure after the AJV/source assertions succeeded: importing the root script from the oracle pulled unrelated workspace TypeScript files into the focused compile. The oracle now derives the already-defined launcher working root without importing the root script.
- `retained-flow-physical-neutral-green-2.log`: the corrected registered Bun/Nx neutral route exited 0. It intentionally emits no stdout. The route executed the AJV schema/fixture oracle, three hostile-schema cases, four hostile-source mutations, and strict TypeScript compilation of the isolated test file.

## Bounded milestone and audit handoff

This milestone proves exact physical retirement only for the tracked Flow `Vec`/`String` backings, the Flow `PagedList` continuation pages, ordered UTF-8 key backings, and the listed direct neural vectors. It does not establish full physical ownership for a Flow document or for its mounted callers. The next audit must retain these concrete gaps:

1. `FlowRetirement::push` still allocates its frontier synchronously and uses `mem::forget(owner)` after failed allocation. Multi-root ingress needs owner-preserving admission; only single-root `from_owner` decomposition currently has the refusal proof.
2. `neural::ValueRetirement` still uses `LinkedList<Owner>`. Its list nodes have no allocation admission, ledger, close demand, or release observation.
3. Ordered retirement does not account the allocations owned by `Arc<Node>`, `Arc<Entry>`, `Arc<String>`, or `Arc<Value>`.
4. Neural `Dictionaries`, `Neurons`, and `Seeds` retain `BTreeMap` node allocations that are removed without physical release accounting.
5. `allocated_bytes` is local and shallow. Before decomposition it does not recursively include every dormant nested `String`, `Vec`, `Arc`, `BTreeMap`, or cleanup-frontier allocation held inside a root owner.
6. `ErasedSnapshotRetirement` exposes only `close_step` and `terminal_is_empty`; it cannot propagate a nested allocation demand or a complete physical allocation ledger through VCS, host, Store, or plugin owners.
7. Fixed 4096-byte close grants remain in higher consumers and in the ordered update/neural cold helpers. Broad existing fixtures still encode logical-byte assumptions and have not been accepted against this physical contract.
8. The capacity ledger is local observation, not process-wide allocation authority. `ResidentLedgerRoot` remains a separately reviewed composition-root candidate and does not yet charge these arbitrary nested capacities.
9. Full typed Pack persistence Slice A remains blocked on the remaining recursive owners and caller propagation. This bounded milestone does not close that requirement.

## Consumer propagation

The still-required broader validation covers:

- framework Flow retained unit tests and selected copy;
- protocol ordered and neural value retirement focused laws;
- Flow VCS schema retirement and host retirement;
- Store snapshot retirement/hydration/publication paths;
- Flow plugin scene, snapshot, mutation, presence, editor, recipe, and preparation paths;
- Generation2d and Generation3d replay/snapshot/mutation retirement.

Every retained caller passes its actual external item and byte grant. A caller that cannot cover the next physical demand remains pending with the same owner. Generation3d's internal `close_step(1, 4096)` is removed rather than replaced by another constant.

Typed persistence Slice A remains blocked until this prerequisite and its affected consumer validation are green. The current ledger proves local owner capacity and exact release only for the bounded categories above; the general process working-allocation authority remains unimplemented.

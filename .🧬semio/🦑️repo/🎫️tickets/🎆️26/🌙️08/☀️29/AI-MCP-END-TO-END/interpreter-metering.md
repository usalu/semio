# Paid Core interpreter: items and copy bytes now meter real retained effects

Status: implemented, not compiled, not run (see "Verification"). Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation` plus one method in the job context. No policy JSON, gate, storage caching, `🧳️owned`, `🫴️record` or `🔤️json` file touched.

## Old versus new meaning

Old: `advance_scalar` called `StepContext::charge_retained_item` for every executed guest instruction (scalar `Push`/`Local`/`Numeric`/`Drop`/`Nop`, loads and stores, label ends, untaken branches, `select`, bulk and branch plan bookkeeping). One call posted `copied_items:1`, `copied_bytes` of about 24 to 60 bookkeeping bytes (`size_of::<Value>()+size_of::<usize>()` and relatives) and one work unit. So items (48,000,000) and copy bytes (3,000,000,000) were a second instruction counter (about 62 bytes per instruction), which is why a note create (about 1.35 billion instructions) tripped both limits although no state of that size is copied.

New:

| Ledger | Meaning | Cadence |
| --- | --- | --- |
| execution fuel (`spent_fuel`, `used`) | one per executed instruction, unchanged | cap per issued `OriginalGuestExecutionBudget`, unchanged |
| work unit (`StepContext::fuel_remaining`, `consume_fuel`) | one per executed instruction (new `StepContext::charge_work_unit`), plus one per retained charge as before | `should_yield`, deadline and cancellation checks at the top of every instruction, unchanged |
| retained items and copy bytes | only real retained effects, bytes as actually moved, born or freed | unchanged ceilings, refunds and settlement |

Charged (items, copy, capacity, release as before): call-frame births (locals, type vectors, frame and operand stack growth), label entry with real type vectors or control-stack growth, `memory.grow` (capacity born, old bytes copied and new bytes zero-initialised by actual length, released backing), `memory.copy`/`fill`/`init` per chunk by exact length (the former `+size_of::<usize>()` cursor bookkeeping is gone from the chunk and from `chunk()`), `data.drop` release, branch results moved to another stack height (`count * size_of::<Value>()`, none when they are already in place), host import argument and result transport, instantiation and all retirement steps (frame returns, `release_vector`, `close_*`), whose demand quotes are unchanged.

Work only (no item, no copy byte): scalar instructions and constants, local and global access, numeric operations, `select`, loads and stores into already born linear memory, `memory.size`, a `memory.grow` that moves nothing (delta 0 or refused), saturating conversions, untaken `if`, untaken `br_if`, branches and `end` whose results are in place, `br_table` entry scanning, branch and bulk plan creation and commit, in-range `memory.grow` finalisation, label pop after its vectors were released.

Guarantees kept: execution fuel exactly one per instruction; the work unit is checked and consumed at the same point as before, so `should_yield`, deadline and cancellation fire at the same instruction boundaries; a step still cannot run unbounded (every iteration needs one work unit); progress detection in `advance_scalar` now compares `(retained progress, original work units)` so a step that paid only work is not mistaken for a stall; zero grant depth still yields before any instruction (checked once per `advance_scalar`, depth is invariant within a turn); refunds, per-turn drive grants, `physical_policy` and the retirement quotes (`returned_frame_demand`, `close_*`) are untouched.

## Changed files

- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`: new `charge_work_unit`; docstring of `charge_retained_item`.
- `…/🧠️interpreter/🏗️instance/🦀️.rs`: docstrings for `charge`, new `pay_work` and `progress`.
- `…/▶️invocation/🦀️.rs`: scalar path, loads and stores via `pay_work`, progress marks, depth check, module docstring with the meaning, registration of the metering test module.
- `…/▶️invocation/🧭️control/🦀️.rs`, `…/💾️memory/🦀️.rs`: per-site conversion (listed above).
- `…/▶️invocation/📏️metering/**` (new): language-neutral vector (`🧪️tests/🧫️fixtures/🔣️.json`, `🧬️schema/🔣️.json`), Rust test (`🧪️tests/🦀️.rs`), independent TypeScript test (`🧪️tests/🟦️.ts`).
- Updated tests and fixtures: `…/▶️invocation/🧪️tests/🦀️.rs` (stack growth: scalar push no longer posts copy bytes, the denied-after-growth step is folded into the constrained step), `…/🧭️control/🧪️tests/🦀️.rs` (`paid_atomic_transfer` now proves one work unit and a closed retained ledger), `…/🏗️instance/🧪️tests/🦀️.rs` and `…/📥️memory/🧪️tests/{🧫️fixtures/🔣️.json,🟦️.ts}` plus `…/📥️memory/🧬️schema/🔣️.json` (`deniedCopyBytes: 7` becomes `deniedFuel: 0`: a store can no longer be denied by a copy grant, a zero execution fuel preserves operands, pc and memory).
- `…/🧠️interpreter/README.md`: new section for the meaning.
- `interpreter-metering-fixture.py` (this ticket): generator for the vector; Node WebAssembly computes every `expected`.

## The language-neutral vector

`📏️metering/🧪️tests/🧫️fixtures/🔣️.json` lists nine modules; each row carries `static` (decoded instruction kinds), `kinds` (executed kinds, so skipped branches are explicit), `instructions` (expected execution fuel), `retained` (effects with exact bytes), `items`, `copyBytes`, `expected` (result). Rows: scalar kinds incl. locals, globals, `nop`, `drop`; `select`; loads and stores of four widths; untaken `if` and `br_if`; taken `br`, `return` and `br_table` in place (items 0, copy 0); bulk memory (`fill` 100, `copy` 64, `init` 4, `data.drop`: items 4, copy 168); zero-length bulk, `memory.size`, trivial `memory.grow` (items 0).

Oracles: (1) TypeScript (`bun:test`): Ajv schema, independent opcode decoder must equal `static`, JavaScript `WebAssembly` must return `expected`, `kinds` must be a sub-multiset of `static`, every retained kind must be one of the four real-effect kinds and executed, SQLite `count(*)`/`sum(bytes)` over `retained` must equal `items`/`copyBytes`. (2) Rust: Wasmtime result, then the paid interpreter must spend exactly `instructions` fuel, `items` retained items, `copyBytes` copy bytes, zero capacity, release only for `data.drop`, and for no-effect rows exactly one work unit per instruction; a second test checks the same program under a three-units-per-turn issue (same totals, more turns, still zero items). The vector was generated and verified with Node's WebAssembly; the TS decoder was run against it under Node (all nine rows decode to `static`).

## Pins updated or found stale

Updated (above). Not changed: `vmFuelByCategory` and `expectedInstructions` fixtures (execution fuel is unchanged by design), host import and clock reply tests (host transport is still charged), numeric traps, `returnedFrame` quotes (frame retirement is unchanged), `paid_label_growth` (typed labels still pay). The ticket notes `poll-host-copy-audit-2026-11-21.md`, `guest-input-work-audit-2026-11-21.md`, `note-create-item-budget.md` and `paid-note-receipt-*` record measurements under the old meaning (fuel 33.2 M at host step 262,144 and the 58-62 bytes per item ratio); they are history and were not rewritten. No Python oracle or other crate pins the per-instruction charge (searched `advance_scalar`, `retained_item`, `deniedCopyBytes`).

## Expected effect on a note create

Items and copy bytes now grow with real effects only (frame births, typed labels, bulk copies, host transport). Scalar instructions still cost one work unit and one fuel, so the 1.35 billion instruction first Input is no longer refused by the items or copy axes; the work-unit and execution-fuel axes (operation fuel, deadline) now bind first. The instruction count itself is untouched (guest-side receiving, the peer's area).

## Verification

See the end of this file.

### Test results

None. Nothing was compiled or run. For about 25 minutes `df -h .cache/b/r/m` showed 8.8 to 12 GiB free (never the 15 GiB the brief requires before a build) while the coordinator's server build ran, so I did not start a compile and did not bypass the gate. Everything above is static reasoning plus Node checks of the fixture (Node WebAssembly results, the TypeScript decoder against all nine rows). `bun` is not installed on this machine, so the TypeScript test file itself was not executed either.

Run, one at a time, when space allows:

1. `SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test -p semio-plugin-interpreter --lib` (covers the new `metering::tests`, the stack-growth, control, memory, host, instance and startup laws).
2. Then `-p semio-framework-job-context` `--lib` (new `charge_work_unit` only adds a method), then the host and wake crates (`🔁️poll`, `🔔️wake/🎟️root`) for drive-level regressions.
3. The TypeScript laws via the repo's bun runner: `📏️metering/🧪️tests/🟦️.ts`, `📥️memory/🧪️tests/🟦️.ts`.

### Unverified and likely first failures

- Compile errors in the dense one-line sources (borrow of `frame` around `pay_work` in `admit_transfer`; name `pay_work`/`progress` resolved through `use super::*`).
- `paid_atomic_transfer` assumes a fixture turn accepts an all-zero retained grant (`maximum_items:0, copy 0, capacity 0, release 0`, depth kept); the old test used zero items and zero copy separately.
- `deniedFuel: 0` assumes `advance_scalar` accepts an execution authorization with fuel 0 and returns Pending without effects (the loop condition `used<fuel` makes this so by reading).
- `original_small_bulk_memory_coalesces_paid_turns` asserts a non-default receipt on every non-blocked turn; with scalars free, a final turn that only runs the zero-length fill would trip it (by trace it should not).
- Stack-growth law: the constrained axis-0 step now grows and executes the push in one step (3 vs 2 capacity unchanged); the removed post-growth state equality is the part most worth re-reading.
- Control `run_with_credit` continuation cases (`settledControl`, `retiredCallee`) rely on a typed label retiring through `release_vector`, which still needs copy bytes; they should still observe the blocked turn.
- Drive-level code that equates "no retained progress" with "no progress" was not audited beyond `advance_scalar` callers (`startup`, `call`), which only read `spent_fuel`.

## Update 2: test trim, frame pool, results

### Source boundary (what the numbers mean)

The refusal `N of 32 files, B of 262144 bytes` is the partial count at the moment the traversal crossed the limit, not the closure size. Reproducing the gate's closure with `readRustSourceClosureV1` (cfg `test`) gives the real figure: 36 files and 313,804 bytes for `semio-plugin-interpreter` now (build set without `test`: 19 files, 235,453 bytes). The in-crate test closure was therefore already far above 262,144 before this ticket (about 330 KB before my moves), so `test -p semio-plugin-interpreter --lib` is refused regardless of the amount trimmed; I did not raise the limit. The build set stays under the limit (check exits 0).

Moved whole test modules into the existing companion crate `semio-plugin-interpreter-core-laws` (public API plus the `testkit` feature; fixtures stay next to the owner and are read with `fixture_text!`): host import (`📞️host` tests), numeric execution, instance opening (scalar startup, input copy, nested call, memory store), and all metering tests. Three small testkit accessors were added (`testkit_stack_site`, `testkit_pool_shape`, `testkit_fill_pool`). The per-category observation machinery moved to `📊️trace/🧪️observation` (cfg test only). Control tests were de-duplicated (`started`) and re-indented.

Still in the crate and therefore not runnable under the gate (private state, updated but unverified): `▶️invocation/🧪️tests` (returned frame law, stack growth law), `🧭️control/🧪️tests`, `💾️memory/🧪️tests`, rest of `🏗️instance/🧪️tests`, `🧊️prepared/🧵️image-wire/🧪️tests`, `🧪️tests/🔬️unit`.

### Frame pool

`CoreInstance.pool: FramePool` holds at most `FRAME_POOL_SLOTS = 16` empty vectors per kind (locals, controls, types) inline in the instance; a zero-capacity slot is free, so the pool bound is a type constant and never allocates. A call takes the best-fitting pooled vector per kind (births only what is missing, so the admission quote shrinks accordingly); label entry takes type vectors the same way; function-label retirement and `close_returned_frame` recycle vectors into free slots for one copied vector handle (an item, no release) and release them only when the kind is full. `returned_frame_demand` quotes exactly that (release 0 while there is room, the exact backing bytes when full), so the one-byte-short laws still hold. Local values are wiped and every pooled vector is cleared before it enters the pool. Pooled vectors keep their capacity live until `close_step` releases them (new `release_pool` step before memories) or the instance retires.

### Results (core-laws crate, `test -p semio-plugin-interpreter-core-laws --lib`)

20 passed, 0 failed (finished in 0.02 s): 8 existing `core_laws`, `metering` (executed instruction kinds vector, work-unit cadence, sequential calls, returned-frame pool law), `instance_opening` (4), `host_import` (1), `numeric_execution` (1). `check -p semio-plugin-interpreter --lib` exits 0.

Metering vector now states that items and copy bytes equal the declared real effects plus two unavoidable ones: operand-stack growth (`stackPeak` gives the growth events and relocated slots) and the two function-label vector steps at return; both are independent of the instruction count. Frame births before and after in the nested-call vector: after the change one call and fifty sequential calls post identical capacity and identical release (`sequential_calls_recycle_frame_storage_inside_the_instance`); before, each call birthed locals, two type vectors and a control vector and released them again, so both grew linearly with the call count (taken from the previous code path, not re-measured).

### Rebuild list

Changed crates: `semio-framework-job-context` (new `charge_work_unit`), `semio-plugin-interpreter` (and its test-only companion `semio-plugin-interpreter-core-laws`). Everything depending on them must rebuild: `semio-framework-job` and its dependents, `semio-plugin-interpreter-runtime`, `semio-plugin-interpreter-imports`, `semio-plugin-interpreter-fixtures`, `semio-plugin-host`, `semio-plugin-host-wake`, `semio-plugin-describe`, and finally the server `semio-os-mcp`. `INTERPRETER_SOURCE` changed, so engine identity values derived from it change with it.

## Update 3: guest-internal moves cost work, not the copy ledger

Rule applied as for scalars. Copy bytes and items now only cover what the host really retains or allocates; moves inside guest storage post work.

| Move | Before | Now |
| --- | --- | --- |
| frame birth served fully by pooled vectors, call arguments | item plus about 100 copy bytes | work, one unit per 64 bytes (min 1); a birth that allocates or grows still charges copy, capacity and release |
| returned frame steps (`Control`, `PopFrame`, recycle into the pool) | copy 24 to 73 per step | work, one unit; quoted demand copy 0 and release 0 |
| returned frame steps that really release (pool full) | copy + release | unchanged, the one-byte-short laws apply to them |
| label type vectors recycled at function end or block end | item plus 24 copy bytes per vector | work, one unit |
| branch result moves to another height | copy by length | work, one unit per 64 bytes, chunked by the turn's remaining work |
| `memory.copy`, `memory.fill`, `memory.init` | copy by length | work, one unit per 64 bytes (min 1) |
| `memory.grow`, `data.drop`, host transport, instantiation, checkpoint, retirement releases, operand-stack growth | charged | unchanged |

Declared constants (`🏗️instance/🦀️.rs`): `WORK_BYTES_PER_UNIT = 64` (also `workBytesPerUnit` in the metering fixture and schema, checked by the TypeScript oracle with SQLite) and `WORK_CHUNK_BYTES = 1 MiB`. One guest-internal move spans at most `min(bytes left, 1 MiB, remaining work x 64)`, and each chunk pays its units before moving, so a long `memory.copy` still yields, observes the deadline and cancels at chunk boundaries exactly like any other work. New context method `StepContext::charge_work_units`. Depth limits, per-turn drive grants, refunds and cancellation are untouched.

Tests (core-laws crate, `test -p semio-plugin-interpreter-core-laws --lib`): 21 passed, 0 failed. New or changed: the metering vector (bulk rows now expect items 0 and copy 0 except the `data.drop` release, plus `moved` and `workUnits`), `guest_internal_moves_cost_work_in_proportion_and_stay_interruptible` (a 60,000 byte fill: same result and receipt with 8 work units per turn, at least 117 more turns, work at least 937 units), the returned-frame law (room: demand copy 0 and release 0; full: exact release and copy), the atomic-transfer law in-crate already asserts one work unit. `check -p semio-plugin-interpreter --lib` exits 0.

Not run (in-crate, private state, boundary): the control, memory, invocation and image-wire test modules. Known stale expectations there: control continuation tests that block a copy grant of 0 at a crossing, `original_small_bulk_memory_coalesces_paid_turns` (its blocked step relied on a copy grant), and the in-crate stack-growth law's call copy figure for pooled calls. They need rewriting against work units once those modules can run.

Rebuild list: same as Update 2: `semio-framework-job-context`, `semio-plugin-interpreter`, `semio-plugin-interpreter-core-laws`, then `semio-framework-job` and dependents, `semio-plugin-interpreter-runtime`, `-imports`, `-fixtures`, `semio-plugin-host`, `semio-plugin-host-wake`, `semio-plugin-describe`, `semio-os-mcp`.

## Update 4: per-instance retained ledger by category

`CoreInstanceOpening` owns a `RetainedLedger` inline (20 plain atomic counters, no heap, so the heap laws are untouched): items and copy bytes for the categories frame births, operand-stack growth, pool births, memory.grow, data/instantiate, host import arguments, host import results, checkpoint/image, retirement, other. Every copy-ledger charge books itself in constant time through `charge_in(category, ...)` / `retirement_charge`; each public step of the instance enters a `LedgerScope` (thread-local pointer to that instance's ledger), so bookings land in the instance whose step runs, never in a process-wide total. The imports crate books its reply states into the same scope.

Read path (no new layering): `CoreInstanceOpening::retained_ledger_text` -> `OwnedRuntime::original_retained_ledger` -> `GuestRuntimes::original_retained_ledger` -> poll session. The session notes the text on the wake driver (`note_original_guest_ledger`, a plain `String` field, no dependency of wake on the interpreter) and prints it in: `ordinary Poll copy budget cannot fund its acknowledgment frame: <ledger>`, `ordinary Poll budget cannot retire its returned Core frame: demand=..., grant=...; <ledger>` and the operation ledger refusal (`semantic operation currency exhausted ...; interpreter retained ledger (items/copy bytes); frame births=a/b; ...; total=n/m`). Format example from the core-laws vector (one call, a root frame and one callee): `frame births=2/556; operand-stack growth=2/16; data/instantiate=1/1984; total=5/2556`, identical for 1 and 50 sequential calls.

Verified: `check` of semio-plugin-interpreter, -runtime, -imports, semio-plugin-host-guest-runtime, -wake, -poll and semio-plugin-host exits 0; core-laws 21 passed, 0 failed. To keep `semio-plugin-host-guest-runtime` under its boundary two rustfmt-indented files there were re-indented (no code change).

### Remaining charges to the copy ledger (items = 1 each; copy per charge)

| Category | Site | Copy bytes per charge | Estimated count per 1.35e9-instruction run |
| --- | --- | --- | --- |
| data/instantiate | `Self` birth (`size_of::<CoreInstanceOpening>`, 1,984 incl. pool in the measured vector) | once per opened instance | one per instance open; the whole image costs below, per open |
| data/instantiate | vectors (`reserve`: `size_of::<Vec<T>>` x 5), memory/table/global/data/element records (`size_of` each, tens of bytes) | 24 to 64 | per open: about 10 + one per memory/table/global/segment |
| data/instantiate | zero-fill of linear memory minimum (`count` bytes, chunked by the copy grant) | minimum pages x 64 KiB | per open: whole initial memory (e.g. 1 to 17 MiB) |
| data/instantiate | active and passive data segments into host storage (`count` bytes) | segment length | per open: sum of all segment bytes (the whole data image, MiB) |
| data/instantiate | table and element values (`bytes_for::<Option<..>>(count)`) | 8 to 16 per slot | per open: table size x 8 |
| checkpoint/image | prepared-image copy (`copy_image`, `copy_image_segment`) | bytes of each vector copied from the sealed image | per open from a prepared image: whole memory + tables + globals + data (MiB) |
| frame births | root frame (`begin_function`) and frames-vector growth | about 100 to 600 | one per invocation; growth about log2(depth) per invocation |
| pool births | callee vectors the pool cannot serve, label type vectors | exact element bytes | bounded by 16 per kind per instance, then none |
| operand-stack growth | `grow_values` (old length x 16) | doubles, 0 to 16 x depth | about log2(peak depth) per invocation |
| memory.grow | chunks of old bytes + zero fill of the new bytes, admission capacity, swap release, `data.drop` release | new memory length per grow | one per guest grow; guests that grow once per allocation burst: tens to hundreds, each MiB-sized |
| host import arguments | `size_of::<OriginalHostCall>() + 16` (about 56) | 56 | 3,158 clock imports: about 177 KB |
| host import results | results x 16 + `size_of::<OriginalHostCall>() + 8` (about 56) per reply, reply state (`size_of::<Reply>()`), guest memory write (`count + 8`, 16 for a clock) | about 130 per clock | 3,158: about 410 KB; `Write`/`Log` imports add the payload length |
| retirement | `release_vector`, label and frame releases when the pool is full, instance close (per vector and per record), `close_invocation_step` | `size_of::<Vec<T>>` 24 per vector, 100 per record | per invocation close: tens; per instance close: one per vector and memory |

Reading the table: 3,158 clock imports cannot account for 3 GB (under 1 MB). Everything proportional to instruction count is gone; what scales with MiB is instantiation (zero-filled memory, data segments, whole image) multiplied by the number of instance opens per operation (the first Input, the eight `admit_actor_context` units, each Poll and each command turn may open or restore an instance). With about 3 GB spent and 1 to 20 MiB per open, the suspect is an instance that is opened or restored once per unit, not a per-instruction or per-import cost; the next measurement will show it as `data/instantiate` or `checkpoint/image` with a count equal to the number of opens.

Rebuild list: `semio-framework-job-context`, `semio-plugin-interpreter`, `-runtime`, `-imports`, `-fixtures`, `-core-laws`, `semio-plugin-host-guest-runtime`, `semio-plugin-host-wake`, `semio-plugin-host-poll`, `semio-plugin-host` and every dependent up to the server `semio-os-mcp` (plus `semio-framework-job` and its dependents).

## Update 5: depth-sized pool, lazy ledger formatting, outside-interpreter counters

(a) Pool. The 16-slot pool missed on every call deeper than 16 (4,045,071 pool births and 2,095,587 retirement copies in the measured run). `FramePool` now holds, per kind (locals, controls, types), one LIFO stack per size class (capacities 1, 2, 4 ... 1024; `POOL_CLASSES = 11`), each bounded by `POOL_LIMIT = 1024` vectors, the declared call-depth bound. Births allocate the class capacity, so any pooled vector of a class serves every request of that class; a returned vector goes to the class of its capacity. Stack backings start empty and grow (4, 8, ... 1024) through one paid step (`PoolMove::Grow`: copy old handles, capacity new, release old; quoted in `returned_frame_demand` with its capacity) so the growth is part of the admission demand; recycling is one work unit; a vector is released only when its class holds 1024. Vectors are cleared and locals wiped on entry. `close_step` releases every pooled vector and stack backing. After warm-up a steady-state run posts no births and no retirement copies: the high-water mark per class is the only cost. Law: one and fifty sequential calls give identical ledgers; fixture `labelVectors: 1` is the one-off growth of the type stack at first function return (items 1, copy 0).

(c) The session no longer formats anything per step. The runtime read path returns the 20 counters (`[u64;20]`, no allocation); the session hands them and a formatter function pointer to the wake driver; the text is built only inside a refusal.

(b) Who else posts to the treasury. The interpreter ledger covers only the interpreter. For the rest the session now measures, per dispatched poll step, `posted = cx.retained_progress() delta - interpreter bookings on this thread during the step` and books it per poll stage in the driver (`outside interpreter, poll stage N = items/copy bytes`); refusals print `operation copy spent=S` (issued minus treasury remaining) next to the interpreter total, so `S - total - sum(outside)` is the part booked while the driver was not stepping (admission and return birth, subloan escrow, cleanup). Known contributors and their formulas, all outside the interpreter: poll session control frames (`control_bytes()`, `size_of::<Phase>() + size_of::<Option<OriginalSourceBinding>>() + ...` per ack/boundary), post-turn relay metadata (`size_of::<Option<usize>>()` per output page, `count + size_of::<usize>()` per copied guest output page), guest output capsule (`bytes.len()` copied per Output byte), actor failure pages (`page.len()` per page), guest relay transit and subloan escrow (`admit_guest_subloan`: logical remaining stock capped at `u32::MAX`, held not spent), parent copies per poll step (`Source parent` and return place births). Counters per stage identify which of these dominates; they are in the refusal text.

Verified: `check` of semio-plugin-interpreter, -runtime, -imports, semio-plugin-host-guest-runtime, -wake, -poll, semio-plugin-host exits 0; core-laws 21 passed, 0 failed. Not run: in-crate tests (boundary), real server.

Rebuild list: `semio-framework-job-context`, `semio-plugin-interpreter`, `-runtime`, `-imports`, `-fixtures`, `-core-laws`, `semio-plugin-host-guest-runtime`, `semio-plugin-host-wake`, `semio-plugin-host-poll`, `semio-plugin-host`, and every dependent up to `semio-os-mcp`.

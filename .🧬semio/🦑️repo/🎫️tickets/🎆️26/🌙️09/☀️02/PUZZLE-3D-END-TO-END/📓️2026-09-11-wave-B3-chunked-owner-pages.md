# 🧊️ Wave B3 — fixed owners backed by ≤64 KiB lazily claimed sub-pages (2026-09-11)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. A3 ceiling #3 (`📓️2026-09-11-audit-A3-perf-ceilings.md` §3),
continuing W-F6 §8 item 2 (`📓️2026-09-10-wave-F6-command-pages.md`) and the W-F fill-OOM tail.

Owned this wave (only these four files; the other paths in `git diff --stat` are live peers):

| file | change |
|---|---|
| `✏️editor/⏳️precompute/📐️geometry/🦀️.rs` | container redesign + reservation seam + four preflight `unreachable!` → refusal |
| `✏️editor/⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs` | three new laws |
| `✏️editor/⏳️precompute/🪣️fill/🦀️.rs` | two call sites (`as_slice`, `get_mut`) — no field declaration changed |
| `✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` | Nakagin scaffolding extracted + one new law |
| `🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-trace` dev-dependency → dependency |

---

## 1. What was wrong

`FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>` asked the guest for **442 368 B in ONE piece**
(`Box<[MaybeUninit<FixtureObject>; 2048]>`, 216 B × 2048), and `DOCUMENT_OWNER_PAGE_BYTES` admitted up
to 1 MiB per page — against `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES = 65_536`
(`🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs:47`), the block size a `dlmalloc`-served linear memory
that never shrinks refuses FIRST. Four owners were over the ceiling, not one (§3 table). Since W-F6
the refusal is honest (`capacity() == 0`, `try_push` hands the value back) instead of a trap, but an
honest refusal still abandons the fill plan the user is 40+ s into.

## 2. Container redesign

One contiguous page → a **prefix of lazily claimed sub-pages**, each a single heap request at or under
the ceiling. Shape (both `FixedOwnerVec<T, N>` and `FixedOwnerMap<K, V, N>`; `FixedOwnerSet` delegates
to the map unchanged):

```
pages: Vec<Box<[MaybeUninit<T>]>>   // always a prefix; Vec reserved exactly SUB_PAGES at new()
sealed: bool                        // a refused or retired owner claims nothing more
len: usize
```

- `owner_sub_page_slots::<T>(N)` — `const fn`, the sizing rule: the largest **power of two** of slots
  whose block stays ≤ the ceiling, never more than `N`. Derived from `size_of::<T>()` and the budget
  constant, so a narrow owner keeps its single page untouched and only a document-scale one splits.
  A power of two keeps the index arithmetic a shift/mask.
- `new()` claims sub-page 0 eagerly (so `backing_ptr`/`backing_credit`/`capacity` keep their old
  observable meaning) and reserves the pointer cursor for the rest; `try_push`/`try_insert` claim the
  next sub-page as `len` crosses its boundary and **refuse** (`Err(value)`, value handed back whole)
  if the guest says no.
- `capacity()` = `N` while nothing was refused, and **what is actually backed** once a sub-page was
  refused or retired. This keeps every existing preflight (`len() == capacity()`) intact and still
  answers honestly after a refusal. It is no longer `const fn` (three call sites, all internal).
- `retire_backing()` gives back **one sub-page per close grant** (last claimed first) and seals the
  owner, so a document-scale owner retires over as many grants as it holds sub-pages instead of
  spending one interactive close step on 432 KiB. `backing_credit()` now reports
  `(claimed sub-pages, claimed bytes)` instead of `(1, whole declared page)` — the fill envelope
  census therefore charges an empty builder LESS than before (lazy), and a few more items.
- `Deref`/`DerefMut` to `[T]` are **gone** (they were the silent contiguity assumption). Inherent
  `iter()`, `get()`, `len()`, `is_empty()` (map), `pop()`, `try_push()` replace them;
  `as_slice()`/`as_mut_slice()` survive only for single-sub-page owners and carry
  `const { assert!(Self::sub_pages() == 1) }` — a **compile-time** refusal for a chunked owner rather
  than a silently truncated prefix. `get_mut` on the vec is `#[cfg(test)]`: production appends/pops.
- **Compile-time ceiling assertion**: `new()` carries
  `const { assert!(Self::sub_page_bytes() <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES) }` on both
  containers, evaluated per monomorphisation. A future widening (or a `T` larger than 64 KiB, where
  the sizing rule bottoms out at one slot) fails the build, not the guest.
- `semio-framework-trace` moved from `[dev-dependencies]` to `[dependencies]` so the guest sizes its
  sub-pages against the one budget constant instead of a copy. It is `std`-only and already in the
  graph through `semio-framework-job`, so nothing new is compiled.

`FillBuilder`'s field declarations are unchanged — they only name `N`.

### Refusal can now happen mid-life, so four preflight traps had to go

Lazy claiming moves refusal from construction to a boundary crossing, which made
`CollisionSpatialIndex::step`'s `unreachable!("preflighted fixed collision cell" / "… member" /
"… oversized …" / "preflighted collision entry")` reachable. All four now `reject_mutation(..)`
(`CollisionMutationStep::Rejected(Capacity(id))`), and the `expect("preflighted fixed collision
bucket")` became a `let … else` refusal. This is the same trap class the ticket already paid for once.

## 3. Size math (measured, `FixedOwnerVec/Map::sub_page_*`, full table in `🗑️generated/wave-B3-sub-page-table.txt`)

| owner | `size_of::<T>()` | N | declared page | sub-page slots | **one request** | sub-pages |
|---|---|---|---|---|---|---|
| fixture objects | 216 | 2048 | 442 368 | 256 | **55 296** | 8 |
| fixture attractions | 136 | 2048 | 278 528 | 256 | **34 816** | 8 |
| collision cells | 56 | 8192 | 458 752 | 1024 | **57 344** | 8 |
| collision entries | 48 | 2048 | 98 304 | 1024 | **49 152** | 2 |
| blocked vortex ids | 24 | 4096 | 98 304 | 2048 | **49 152** | 2 |
| placed lookup | 32 | 2048 | 65 536 | 2048 | 65 536 | 1 |
| candidate classification | 56 | 1024 | 57 344 | 1024 | 57 344 | 1 |
| catalog vortices | 200 | 256 | 51 200 | 256 | 51 200 | 1 |
| every other declared owner | — | — | ≤ 49 152 | = N | ≤ 49 152 | 1 |

Four owners were over the ceiling before (442 368 / 458 752 / 278 528 / 98 304 ×2); the largest single
request the lane now makes is 65 536 B, exactly the budget. `FixedOwnerSet` grew from 16 B to 40 B
(the page pointer became a `Vec` + seal), which is why the cell map's entry is 56 B.

## 4. Laws — RED at HEAD, GREEN after

New (four; the audit asked for three — the boundary-crossing one is extra because chunk arithmetic is
where a split owner silently loses entries):

| law | file |
|---|---|
| `nakagin_scale_fill_places_an_object_under_a_fragmented_guest_reservation_ceiling` | `🪣️fill/🧪️tests/🔬️unit/🦀️.rs` |
| `every_fixed_owner_sub_page_request_stays_under_the_guest_contiguous_ceiling` | `📐️geometry/🧪️tests/🔬️unit/🦀️.rs` |
| `an_owner_whose_middle_sub_page_was_refused_keeps_the_earlier_ones_and_reports_honest_capacity` | `📐️geometry/🧪️tests/🔬️unit/🦀️.rs` |
| `an_owner_wider_than_one_sub_page_reads_writes_and_retires_across_its_boundaries` | `📐️geometry/🧪️tests/🔬️unit/🦀️.rs` |

The fragmented guest is injected by `OwnerReservationLimit::install(ceiling_bytes, grants)` — a
thread-local, `#[cfg(test)]`-only policy consulted by `claim_owner_sub_page`, the ONE heap request
path all three containers share (production compiles it to `const fn … { true }`). A native suite
cannot exhaust a 512 MiB linear memory, so this is how the law reaches build #29's guest.

**Placement note**: the audit asked for the Nakagin law in the geometry test file. It sits in the fill
test file instead, directly beside the `nakagin_scale_fill_is_not_refused_and_places_at_least_one_object`
it mirrors, sharing one extracted `nakagin_scale_roots()` / `drive_nakagin_scale_fill()` (the repo's
"if code is repeated, it MUST be close to each other"); duplicating the 60-line flagship fixture into
another module was the alternative. Both pre-existing and new Nakagin laws now close their fault
payload through `faulted(..)` — dropping one aborts the whole test binary
(`RetainedJobPayload requires one-page close`), which is how the first RED run died.

RED, at HEAD behaviour (single page), before the container change:

```
running 3 tests
… nakagin_scale_fill_places_an_object_under_a_fragmented_guest_reservation_ceiling
  panicked: Nakagin-scale fill faulted at stage PrepareFixture after 2 turns: None
… an_owner_whose_middle_sub_page_was_refused_keeps_the_earlier_ones_and_reports_honest_capacity
  panicked: a 512 KiB declared page is backed by more than one sub-page
… every_fixed_owner_sub_page_request_stays_under_the_guest_contiguous_ceiling
  panicked: the fixture objects owner asks the guest for 442368 contiguous bytes, over the 65536-byte ceiling
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 670 filtered out; finished in 0.01s
```

GREEN, after:

```
running 4 tests
test …fill::tests::nakagin_scale_fill_places_an_object_under_a_fragmented_guest_reservation_ceiling ... ok
test …geometry::tests::an_owner_whose_middle_sub_page_was_refused_keeps_the_earlier_ones_and_reports_honest_capacity ... ok
test …geometry::tests::an_owner_wider_than_one_sub_page_reads_writes_and_retires_across_its_boundaries ... ok
test …geometry::tests::every_fixed_owner_sub_page_request_stays_under_the_guest_contiguous_ceiling ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 671 filtered out; finished in 0.16s
```

## 5. Verification (foreground, `RUST_MIN_STACK=268435456` — the repo's own `SEMIO_TEST_NATIVE_RUST_MIN_STACK`; without it whole-app laws overflow the stack)

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1
sub_page fragmented_guest an_owner nakagin fill_ spatial_ document_` (full log:
`🗑️generated/wave-B3-laws.txt`):

```
test result: FAILED. 115 passed; 1 failed; 0 ignored; 0 measured; 559 filtered out; finished in 53.45s
failures:
    editor::puzzle3d::component::tests::open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin
```

Every owner/fill/spatial/document law passes, including the pre-existing
`an_owner_whose_page_was_refused_refuses_every_insert_instead_of_trapping`,
`nakagin_scale_fill_is_not_refused_and_places_at_least_one_object`,
`document_scale_fixed_pages_are_admitted_by_the_fill_envelope_reservation`,
`spatial_fixed_collections_use_the_credited_pages_and_return_identical_plus_one_owners`,
`spatial_index_close_retains_bucket_values_and_retires_one_credited_owner_per_grant`.

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` → **0 errors**,
`Finished dev profile`, and **no new warning** in the changed files (a JSON scan of every warning span
under `⏳️precompute` shows only the pre-existing `set_scene`/`set_scene_config` dead-code pair in
`⏳️precompute/🦀️.rs`). The wasm component build and the browser probe were not run, as instructed.

### The one failure is NOT this wave's — proven by A/B, not asserted

`open_vortex_suggestions…_for_nakagin` asserts a 2 ms unoptimized budget. Flipping
`owner_sub_page_slots` back to single-page (HEAD behaviour) with a temporary `[DEBUG]`-marked probe,
three runs each, back to back on the same tree:

| backing | worst turn (3 runs) |
|---|---|
| chunked (this wave) | 4.692 ms / 4.729 ms / 4.664 ms |
| single page (HEAD) | 4.949 ms / 6.974 ms / 4.945 ms |

It fails either way — the machine is running a concurrent fleet — and the chunked containers are
marginally **faster**, not slower. The probe was reverted; `grep -rn DEBUG` over the three changed
Rust files returns nothing.

Earlier in the session a full-lib run showed 32 failures across `component::tests::`
(selection/gumball/clipboard/inspection). Re-running the same two families twenty minutes later, after
a peer's `✏️editor/🦀️.rs` edit settled, gives **281 passed / 2 failed** — the perf law above and
`two_instances_converge_disjoint_object_edits_via_backbone` (documented `module.vcs` foreign failure
since W-F). Two `cargo test` runs also aborted on a peer's mid-edit `semio-framework-plugin` (unclosed
delimiter, then a missing `interaction_selection_witness`); both cleared by polling the file's mtime.

## 6. Over-ceiling contiguous requests still elsewhere in the guest (found, NOT fixed)

| # | request | size | where |
|---|---|---|---|
| 1 | decoded brush-mesh buffers — `decode_brush_mesh_geometry` collects `Vec<f32>`/`Vec<u32>` | up to **786 432 B** each (`FILL_WORKER_MAX_MESH_VALUES = 196_608` × 4), plus the encoded blob it reads from | `⏳️precompute/🦀️.rs:65-66,1067-1074` |
| 2 | `plugin_exchange`'s boxed future, once per command | **76 848 B** | `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` `plugin_exchange_boxed` (W-F6 §8.1) |
| 3 | `wit_bindgen` `start_task` boxes the whole turn future, once per poll | **189 328 B** | generated bindings; measured by `📏️future-size`'s `the_reactor_turn_future_reports_its_generator_size` (W-F6 §8.1) |
| 4 | `ToolExecutionContract::resumable(PUZZLE3D_IMPORT_RAW_BYTES, …)` raw import buffer | **262 144 B** | `✏️editor/🦀️.rs:6855` |
| 5 | `PUZZLE_COMMAND_OUTPUT_BYTES` retained-command output buffer | **262 144 B** | `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:13` |
| 6 | `ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES` (puzzle3d/5d wasm envelope) | **262 144 B** | `🏪️store/🦀️.rs:8123`, used by `✏️editor/🌉️wasm/🦀️.rs:19` |
| 7 | `CommandEnvelopeSet::try_new()` 64 command + 64 page slots | **262 272 B** | host-side only (`🏃️run`, `🌉️mcp`, wgpu) — waste, not a guest hazard (W-F6 §8.3) |

(1) is the largest and the only one on the same fill/brush path this wave touched: a document-scale
mesh is admitted at 196 608 values, so one `registerBrushMesh` decode asks the guest for up to 768 KiB
twice over. (4)–(6) are 4× the ceiling and ride import/export and the wasm envelope, i.e. once per
user action rather than per turn. (2) and (3) are per-command/per-poll and are the ones W-F6 already
named as "the request class a fixed, never-shrinking linear memory refuses first".

## 7. Open

- The chunked owners are only proven natively. The next wasm build is their first guest run; the law
  that would catch a regression there is the browser fill battery, not a native suite.
- `DOCUMENT_OWNER_PAGE_BYTES` (1 MiB) still bounds the *declared* page and is now the only ceiling
  that does not correspond to a real allocation. It could become "sub-pages × ceiling" once someone
  re-reads the fill-envelope reservation maths.
- `FixedOwnerMap::index_of` is still an O(n) linear scan over `iter()`, now through a `flat_map` over
  sub-pages. The measurements in §5 say it did not cost anything on the Nakagin suggestions path, but
  a document-scale map that actually fills (2048 entries) has never been profiled.

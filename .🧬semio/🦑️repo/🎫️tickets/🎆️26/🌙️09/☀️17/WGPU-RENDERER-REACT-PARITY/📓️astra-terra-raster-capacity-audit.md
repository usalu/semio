# Terra Raster Capacity Audit

Read-only, source-level follow-up audit on 2026-09-20. I read the preceding Terra and Sol reports, then inspected the current production and law paths. I did not run Cargo, WGPU, browser, or other builds, and made no production change.

## Result

I found no concrete reachable defect in the requested capacity, retention, hashing, or EngineCanvas paths. The source implements the stated full-capacity and changed-replacement rules. The remaining findings are test coverage gaps, not observed behavioural failures.

## Static verification

### Unchanged live or staged content at 256 entries

`RasterTextureTable::prepare_admission_step` requires sealed candidate ownership, validates the identity dimensions and witness, then calls `raster_content_is_reusable` **before** checking the aggregate item or byte credits. A matching staged entry must belong to the same candidate and have the same identity; a matching live entry is also reusable. Therefore a sealed, full 256-key candidate that re-offers identical content returns ready without a reservation. `ensure_raster_step` performs the same reuse decision before it creates an upload cursor or a GPU resource.

Evidence: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1888-1917`, `:2030-2057`.

### Changed same-key peak reservation

A differing identity bypasses reuse. `reserve_engine_texture` tests `live + staged` item and byte credits before it creates the reservation; the staged table is distinct from live, so it can hold B for a key whose committed A remains live only when real headroom exists. At a full protected set, admission advances only unowned retirement and then returns retained-frame backpressure if no credit becomes free. During presentation `get` selects B only after the candidate presentation witness is armed; otherwise it still returns A. Commit replaces live A with staged B through the bounded retirement cursor, while abort retires only staged entries of that witness and clears candidate ownership.

Evidence: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:1888-1974`, `:2030-2127`, `:2181-2375`.

### Committed and previous retention

The residency ledger protects the exact union of committed, candidate, and previous keys. A successful commit transfers committed ownership to `previous`; a second commit is refused until that set is released. `AppPresentedRetirement` commits raster staging first, retires the old prepared packet, only then releases previous raster ownership, and finally performs unowned retirement. Abort takes the distinct candidate path and never clears committed ownership.

Evidence: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:925-1008`, `:2234-2375`; `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13423-13525`.

### Bounded content hashing

`PreparedRasterProducer::step` computes one page from `rows_per_page * row_bytes`; producer admission constrains that amount to `PREPARED_RASTER_PAGE_BYTES` (16 KiB). It mixes that one byte slice and yields. The reverse page order is deliberate: `page_for_row` maps each logical row back to the corresponding reverse physical slot, and the byte offset is retained in the identity mix. Identical producers therefore have the same identity, while the existing law changes one byte across two pages.

Evidence: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:340-397`, `:836-850`, `:1040-1207`; `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:100-153`.

### EngineCanvas identity, one target, and retirement

EngineCanvas identity includes the surface token slot and generation, document generation, scene revision, per-surface metrics generation, primary metrics generation, width, and height. Candidate matching repeats the same freshness tuple before each phase and at publication, while live freshness independently re-reads the CPU surface identity, document generation, scene revision, and metrics generation. There is one `create_target_texture` call. The rendered texture and view transfer to the table at `Stage`; the Vello renderer releases on a separate `RetireRenderer` step before publication. Candidate close retires admission, renderer, view, and texture one owner per step.

Evidence: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1049-1217`, `:1288-1448`, `:1999-2015`.

## Test coverage gaps

1. The capacity law calls `raster_content_is_reusable` and `raster_admission_fits` directly for the 256-key cases. It does not drive `RasterTextureTable::prepare_admission_step` with a sealed 256-key ownership set, so it does not exercise the actual admission ordering, witness freshness check, or progressive unowned-retirement loop for this scenario. See `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs:78-110` versus production `draw/🦀️.rs:1888-1917`.

2. The changed same-key and abort laws manipulate the live/staged registries and residency ledger directly. They do not execute `begin_presenting`/`get`/`abort_presented_step` or `commit_presented_step`, so the observable A-before-presentation, B-during-presentation, A-after-abort transition is not covered through the production table cursor. See `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs:97-110`, `:175-192` versus `draw/🦀️.rs:2221-2375`.

3. The EngineCanvas follow-up assertions are source-wiring checks: they count `create_target_texture`, look for the reuse call, and reject the former replacement target names. They do not execute an `EngineGpuCandidate` through a changed tuple, primary-metrics invalidation, reuse publication, and each close phase. The source has those transitions, but this exact behaviour remains unexercised by the capacity follow-up laws. See `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖼️wgpu-raster-residency/🟦️.ts:168-184` and `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1250-1450`.

These omissions reduce regression detection strength but do not contradict the live source behaviour above.

# Wave W-M2 — paged `registerBrushMesh` upload (2026-09-09)

Scope: get real GLB collision geometry from the world layer into the puzzle 3d precompute session
inside the shared retained-command contract. Continues the partial edits an earlier W-M2 agent landed
before a process restart at ~17:50; this note records the whole wave as it now stands on disk.

## 1. The defect this wave closes

On boot the world layer registered the loaded GLB with one command carrying JSON number arrays:

```
tool factory 's.puzzle.puzzle3d@1/*#editor/registerBrushMesh' rejected 63997 raw bytes before
decoding; maximum is 8192
```

`PUZZLE_COMMAND_RAW_BYTES = 8_192` (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:10`) is the
raw wire one retained puzzle command admits; the raw bytes counted are exactly
`to_json_string(&(verb, args))`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18154`, and `:21090` for the reserved lane).
Measured mesh sizes (parsed out of the GLB accessors):

| mesh | positions (values) | indices | values | pages @1024 |
| --- | ---: | ---: | ---: | ---: |
| `🧊️placeholder.glb` (Nakagin capsule; every `dist/mesh/*.glb` is this same 771 728-byte asset) | 25 344 | 48 384 | 73 728 | 72 |
| `◀️hexagonal-cut-concrete-forest-left.glb` (the mesh in the boot trace) | 2 541 | 2 874 | 5 415 | 6 |

Without the mesh, `Puzzle3dCollision` has no collision body, so brush / volume-brush / suggestion
placement runs against nothing.

## 2. Wire contract (one page)

```
registerBrushMesh { surfaceId, url, digest, page, pageCount, positionsB64?, indicesB64? }
registerBrushMesh { surfaceId, url, digest }                      // id-only adoption
```

* Payloads are base64 of little-endian `f32` positions / `u32` indices — plain JSON numbers cost
  ~11.8 bytes per value (measured from the 63 997-byte rejection), which is < 256 positions per page;
  base64 costs 5.33 characters per value, so one page carries 1 024 values in 5 464 characters.
* Positions fill each page first; the indices stream continues in whatever of the page's 1 024-value
  budget is left. The run is therefore dense and only its last page is partial.
* `digest` is unkeyed BLAKE3 over the positions' little-endian bytes followed by the indices' —
  computed by `blake3Hex` on the host and `semio_framework_hash::hash_bytes` in the plugin, two
  independent first-party implementations pinned against each other by the fixture (§5).
* A base64 string counts as **one** decoded item (`bounded_json_items`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15982`), so the 512-item
  `PUZZLE_COMMAND_DECODED_ITEMS` cap is no longer the binding constraint it was for number arrays.

Measured envelope cost (`JSON.stringify(["registerBrushMesh", {surfaceId, url, digest, page,
pageCount, positionsB64:"", indicesB64:""}])`): 213 bytes for `/test/paged.glb`, 261 bytes for
`/asset/🥽️mesh/🧊️placeholder.glb` — so both meshes above page at the full 1 024 values, and a full
page costs ≈ 5 725 raw bytes against the 8 192 limit.

## 3. Changes

### 3.1 Plugin — staging area and decoder
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs`,
region `//#region 🧩️PagedBrushMeshUploads` (999–1178):

* `:999` `PUZZLE3D_MESH_PAGE_VALUES = 1_024` — values (positions + indices together) per page.
* `:1005` `PUZZLE3D_MESH_PAGE_BASE64_CHARS = (PUZZLE3D_MESH_PAGE_VALUES * 4).div_ceil(3) * 4` = 5 464.
  **Corrected in this session** — it read `.div_ceil(3).div_ceil(4) * 4` = 1 368, which is not a
  base64 length at all. That was load-bearing twice: it would have made
  `decode_brush_mesh_page_values` reject every real page as `Payload`, *and* made
  `Puzzle3dPrecomputeCommandWork::extent` (`✏️editor/🦀️.rs:6157`) return `None` for every real page,
  i.e. the job would have been refused before a byte was read.
* `:1010` `PUZZLE3D_MESH_UPLOAD_SLOTS = 4` — partial uploads staged at once, LRU-retired by `touched`.
* `:1015` `PUZZLE3D_MESH_UPLOAD_MAX_PAGES = (FILL_WORKER_MAX_MESH_VALUES * 2).div_ceil(1024) = 384` —
  both arrays at the engine's own 196 608-value ceiling. The largest shipped mesh is 72 pages.
* `:1020` `Puzzle3dMeshUploadFault` {`Envelope`, `Payload`, `Gap`, `Capacity`, `Digest`, `Geometry`}
  with `:1037 code()` giving stable `puzzle3d-register-mesh-*` wire codes.
* `:1052` `Puzzle3dMeshUploadStep` {`Staged{next_page,page_count}`, `Complete(positions,indices)`}.
* `:1088` `brush_mesh_digest`, `:1098` `decode_brush_mesh_page_values` (refuses a string longer than
  one page, a byte run that is not whole 4-byte values, and a value count over the caller's budget).
* `:1110` `stage_brush_mesh_page` — keyed `(url, digest)`; envelope, per-page value, per-run capacity,
  digest and geometry checks; commits on the last page. **Changed in this session:** a `Gap` now
  *drops* the broken run (`:1123`) instead of leaving it staged, so a client re-opens at page 0 rather
  than resuming into bytes nobody can account for; the `Gap` docstring at `:1024` says so.
* `:1166` `retire_abandoned_brush_mesh_uploads`, `:1174` `staged_brush_mesh_uploads` (census).
* `:1586` `Puzzle3dCollision::adopt_shared_mesh(url, digest: Option<&str>)` — a supplied digest is
  verified against the resident/derived geometry, so a stale id never adopts foreign bytes.
* `:2233` `Puzzle3dPrecomputeSession::stage_mesh_page` — `Ok(None)` = closed and installed,
  `Ok(Some(next))` = open run awaiting page `next`. `:2256` session-level `adopt_shared_mesh`.

Retirement is driven from `✏️editor/🦀️.rs:2589` (`Puzzle3dSessionRegistry::retire`), so the page runs
a closed document instance abandoned are swept when its session slot is retired; a run that advanced
inside the cycle survives.

### 3.2 Plugin — command arm
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs`
(whole file, 1–65): id-only adoption when `page`/`pageCount` are absent; otherwise decode both
payloads under the page budget and stage. Every refusal becomes a visible
`Effect::Notify { message: "<fault code>: <url>" }` (`:60`) — never a silent drop, because without the
geometry the brush utility has no collision body at all. `MAX_POSITIONS`/`MAX_INDICES` (the old
512-item caps) are gone.

The per-page work stays cursorized through `Puzzle3dPrecomputeCommandWork`
(`✏️editor/🦀️.rs:6108 scan_mesh_page`, `:6124 PUZZLE3D_MESH_PAGE_SCAN_CHARS = 512`, `:6152 extent`):
one page validates in `ceil(5464/512)*2 + 1 = 23` bounded steps against
`PUZZLE_COMMAND_WORK_ITEMS = 4_096`.

### 3.3 Host — pager
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`,
region `//#region 🥽️Puzzle3dBrushMeshUpload` (2503–2613):

* `:2507` `PUZZLE3D_MESH_COMMAND_RAW_BYTES = 8_192`, `:2512` `PUZZLE3D_MESH_PAGE_VALUES = 1_024`.
* `:2520 PUZZLE3D_MESH_UPLOAD_SLOTS`, `:2525 PUZZLE3D_MESH_UPLOAD_MAX_PAGES`,
  `:2529 PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES` — **added in this session**, mirroring the plugin's own
  ceilings so the host's queue is bounded rather than unbounded memory.
* `:2546` `registeredPuzzle3dBrushMeshes: Map<url, digest>` (was a `Set<url>`) — a mesh whose bytes
  changed uploads afresh, a mesh already paged is re-announced by id + digest.
* `:2550` `puzzle3dWireBytes` — upper bound on JSON wire cost (ASCII exactly, every other UTF-16 unit
  charged as a six-character escape, which no encoder exceeds).
* `:2574` `puzzle3dBrushMeshDigest`, `:2581` `puzzle3dBrushMeshPageCapacity`
  (`floor(budget * 3 / 16)`, i.e. 5.33 base64 characters per value, capped at the page value budget),
  `:2592` `puzzle3dBrushMeshPages`. **Added in this session:** `:2598` a run longer than
  `PUZZLE3D_MESH_UPLOAD_MAX_PAGES` produces no pages at all rather than a command the plugin refuses.

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:4248–4286`:
`handleRegisterBrushMesh` dispatches `{url, digest}` when this process already paged that exact
digest, otherwise queues the page run and drains **one page per macrotask** (`setTimeout(drain, 0)`),
so no single unit approaches the 8 ms step law. **Added in this session:** `:4265` refuses a run that
would push the queue past `PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES`. The unmount effect (`:4279`) clears the
timer and withdraws the id claim for every page still queued.

### 3.4 Language-neutral fixture
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🥽️brush-mesh-upload/🔣️.json`
— **new**. Holds the four wire constants, the six fault codes, one worked example (unit cube: 24
positions, 36 indices, its digest and its single page's two base64 payloads), and the two
document-scale page plans from §1. Read by the Rust unit laws (`include_str!`) and by the TypeScript
suite (`import`), so neither end can drift alone.

## 4. Tests

New Rust laws in
`✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` region `//#region 🧩️PagedBrushMeshUploads` (1333–1509):

| line | test |
| ---: | --- |
| 1384 | `a_document_scale_mesh_uploads_in_pages_and_registers` — a Nakagin-sized mesh pages into exactly the fixture's 72 pages; every page's real JSON wire is asserted `<= PUZZLE_COMMAND_RAW_BYTES`; only the last page closes the run; the reassembly is byte-identical and installs as live collision geometry |
| 1424 | `a_gapped_or_mismatched_page_run_is_refused` — empty digest / page beyond the run / run over the ceiling → `Envelope`; not opening at page 0, a skipped page, a contradicted page count → `Gap`; a non-base64 or over-long payload never reaches staging; a run closing on unannounced bytes → `Digest`; every refused run releases its slot |
| 1447 | `an_uploaded_mesh_is_adopted_by_url_and_digest` — a one-page run installs; a fresh engine adopts by `(url, digest)`; a stale digest and an unknown id adopt nothing |
| 1467 | `the_paged_upload_contract_matches_the_language_neutral_fixture` — every constant, every fault code, the digest, the page encoding and the page decoding checked against the fixture |
| 1495 | `an_abandoned_page_run_is_retired_and_a_live_one_survives` |

`✏️editor/🧪️tests/🔬️unit/🦀️.rs:3385` `one_brush_mesh_page_validates_inside_one_command_work_budget` —
the 8 ms/work-budget law for the page scan.

New TypeScript laws in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:8426–8497`:
the fixture's constants and example page run reproduced by `puzzle3dBrushMeshPages`, cross-checked
against **Node's own `Buffer` base64** as a third-party oracle; a document-scale run whose every page
is `Buffer.byteLength(...) <= PUZZLE3D_MESH_COMMAND_RAW_BYTES` and which reassembles exactly; and the
id+digest re-announcement path. (The Python `base64`/`struct` oracle used to author the fixture agrees
byte-for-byte with both.)

### Commands and tails

Rust (foreground, `RUSTC_WRAPPER=""`, `RUST_MIN_STACK=134217728`,
`CARGO_TARGET_DIR=…/scratchpad/target-p3d`):

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests -j 4
  → Finished `dev` profile [unoptimized] target(s) in 8.54s   (2 pre-existing style warnings, §6)

cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 \
  -- --test-threads=1 a_document_scale_mesh_uploads_in_pages_and_registers \
     a_gapped_or_mismatched_page_run_is_refused an_uploaded_mesh_is_adopted_by_url_and_digest \
     the_paged_upload_contract_matches_the_language_neutral_fixture \
     an_abandoned_page_run_is_retired_and_a_live_one_survives
  → test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 596 filtered out

cargo test … -j 4 precompute -- --test-threads=1
  → test result: ok. 142 passed; 0 failed; 0 ignored; 0 measured; 463 filtered out

cargo test … -j 4 suggestion_and_precompute_hostile_static_law -- --test-threads=1
  → test result: ok. 1 passed; 0 failed

cargo test … -j 4 -- --test-threads=1 one_brush_mesh_page_validates_inside_one_command_work_budget \
     the_paged_upload_contract_matches_the_language_neutral_fixture
  → test result: ok. 2 passed; 0 failed
```

TypeScript, from
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`:

```
SEMIO_TEST_LEVEL=long bun x vitest run "engine-contract"
  → Test Files  1 passed (1) / Tests  452 passed (452)

bun nx run @semio-tech/framework-renderer-react:typecheck
  → 819 pre-existing errors repo-wide, ZERO in 🌐️World3dHost, 🛠️ShellHelpers or 🔬️engine-contract
```

## 5. Two repairs to files this wave owns

* `✏️editor/⏳️precompute/🦀️.rs:1005` — the base64-length constant, §3.1. This is the one real bug the
  partial edits carried, and it would have made every page refused.
* `✏️editor/🧪️tests/🔬️unit/🦀️.rs:250,288` — the `suggestion_and_precompute_routes_are_cursorized`
  static law still demanded `Puzzle3dPrecomputeCommandStage::CheckpointBytes`, a stage this wave's own
  rewrite of `Puzzle3dPrecomputeCommandWork` removed (already absent at HEAD, so the law was red at
  HEAD). Repointed to `PUZZLE3D_MESH_PAGE_SCAN_CHARS`, which is what actually cursorizes the
  `registerBrushMesh` route now. Test green.

## 6. Not verified / not this wave

* **`✏️editor/🧪️tests/🔬️unit/🦀️.rs` `rendered_flag_requests`** used `JsonObject::values()`, which that
  type does not have (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:171`), so the crate's test target did
  not compile at all. Fixed in passing to `map.iter()` — a peer's helper, one line, unblocking.
* **`fill_worker_admitted_fixed_pages_survive_replan_and_mesh_supersession_until_retained_close`**
  fails under some filters and passes under others (`registry_generation` 1 vs 1). Proven independent
  of this wave: it fails identically with both new tests skipped
  (`… mesh -- --test-threads=1 --skip a_document_scale… --skip an_uploaded_mesh…` → 17 passed, 1
  failed). Order-dependent state in `fill_envelope_registry`, owned by the fill lane.
* **12+ `editor::puzzle3d::component::tests` app-level failures** ("puzzle3d app never quiesced:
  pending typed operations outlived the settle budget"; `[DEBUG] reserved job 'interactionSelect' poll
  1..256: Submitted`). Part of the 44-failure baseline recorded in
  `📓️2026-09-09-remaining-test-failures-audit.md`; the reserved-job lane, not the mesh wire.
* **`🧪️tests/🧩️package-integration/🟦️.ts`** fails to import (`ReferenceError: self is not defined` in
  the wgpu `plugin-bridge.ts`) — pre-existing, unrelated.
* **No runtime/browser confirmation.** No wasm rebuild and no dev server was run, per the wave brief,
  so the end-to-end claim rests on the unit and contract laws above, not on a boot trace. The
  remaining runtime question is whether `BrushMeshRegistrar`
  (`🌐️World3dHost/🟦️.tsx:5247`) delivers the same `positions`/`indices` arrays the digest is computed
  over — the pager is exercised by tests, its call site is not.
* **The two `cargo check` warnings** are a peer's unqualified-path lint on
  `protocol::json::from_json_str` in `✏️editor/🧪️tests/🔬️unit/🦀️.rs:101`; left alone.

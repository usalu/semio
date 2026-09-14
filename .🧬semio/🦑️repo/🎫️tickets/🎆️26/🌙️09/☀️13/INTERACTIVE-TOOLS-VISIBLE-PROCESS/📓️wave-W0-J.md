# 📓️ Wave W0-J — Window-Config Pack Load

Lane **W0-J window-config pack load** (follow-up to `📓️wave-W3-2c.md` §5.3). Abbreviations:

- `C` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config`
- `H` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs`

## 1. Root Causes (Five Stacked Defects, All Confirmed With Temporary `[DEBUG]` Probes, Since Removed)

1. **Silent `Ok`.** `WindowConfigOwnerRegistry::load` ran its 1 048 576-turn loop. When the loop ran out, it cancelled and retired the load, then returned `Ok(())` with nothing installed.
2. **Fixed grant vs. page cost.** `WindowConfigPackLoadGrant::one_page()` granted 4 096 B. A retained source page costs `size_of::<RetainedPackPage>()` = 4 104 B, so `reserve_next` refused it on every turn. The close path had the same flaw: the decoder's `next_release_allocation_bytes` existed, but nothing called it.
3. **The `Pack` rejection.** It was not a grant problem. `ingress` asked for a page reservation *before* it checked whether all input was already admitted.
   - A 1-page Pack fills `maximum_pages`.
   - `next_allocation_bytes` then answers `retained-pack.page-credits`, which maps to `Pack`.
   - So the source was never sealed.
4. **The `TypedState` rejection** (hidden behind #3). The token builder kept one flat `wrappers` stack.
   - A `Block` tag on field 0 (`#[dsl(block)] viewport`) was popped by the *first scalar inside* the nested record: `expected Block, found Record{1: Block(Float)}`.
   - `expected()` also saw the outer wrapper at the wrong depth.
5. **Hydration stall.** `RetainedConfigStoreHydration` `Begin` gates on `maximum_bytes >= maximum_value_bytes`, which is the owner's schema bound `MAXIMUM_PUBLICATION_BYTES`. An owner declaring 16 384 B stalled forever under 4 096 B. This is the "schema-bounded record size" grant.
6. **Baseline test red for its own reason.** Both framework laws used `crate::app::NoConfig` as `State`. `NoConfig::encode_pack` emits zero bytes, with no envelope and no record spec. No retained Pack loader can decode that, hence `window-config.pack-envelope`. It was a test owner that was not a real Pack.
7. **The `Drop` panic** (W3-2c's trial). `load` returned `Err` while the load was still non-terminal, so `WindowConfigPackLoad::drop` asserted. That happened because retirement could never release a 4 104 B page under 4 096 B (#2).

## 2. What Changed

### 2.1 `C/🦀️.rs` (Registry)

- **:517–518** `pub const WINDOW_CONFIG_PACK_LOAD_TURNS: usize = 1_048_576;`
- **:523** `WindowConfigOwnerRegistry.retiring: Vec<WindowConfigPackLoad>` holds parked loads.
- **:611** `pub async fn load(&mut self, pack: WindowConfigPack) -> Result<(), Fault>` now delegates to `load_within`.
- **:618** `pub(crate) async fn load_within(&mut self, pack: WindowConfigPack, turns: usize) -> Result<(), Fault>`
  - Every turn uses `load.next_grant()`.
  - Tracks `settled`.
  - Running out of turns yields the typed fault `window-config.load-bound`, or the rejection diagnostic if one was recorded. It never answers `Ok`.
  - A load that cannot retire within the bound is parked (`park_retained_load`, :667) and is never dropped.
- **:724** `close_step` retires parked loads first, each under its own exact release demand. `terminal_is_empty` requires `retiring` to be empty.

### 2.2 `C/📥️retained/🦀️.rs` (Retained Load)

- **:19** `pub const fn WindowConfigPackLoadGrant::for_demand(demand_bytes: usize) -> Self` returns `{1 item, max(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, demand)}`. `one_page()` was removed.
- **:105** `pub fn WindowConfigPackLoad::next_grant(&mut self) -> WindowConfigPackLoadGrant`
- **:79 / :1536** `ErasedWindowConfigPackLoad::demand_bytes` returns the maximum of the decoder demand and the hydration demand.
- **:911** `RetainedWindowConfigStateDecode::demand_bytes`, by phase:
  - `Ingress`: the source page allocation;
  - `Replay`: the segment, value or catalog allocation;
  - always: the release demand.
- **:774** `ingress` seals when all input is admitted, before it reserves anything (fix #3).
- **:211** `pending_wrappers()`: wrappers are now `(ValueWrapper, depth)`.
  - `expected()` folds only the wrappers at the current depth.
  - `emit` pops only those.
  - `end_container` rejects a dangling wrapper (fix #4).
- **:1552** `begin_typed_window_config_pack_load` rejects `pack.len() > O::MAXIMUM_PUBLICATION_BYTES` as `Capacity`, i.e. `window-config.capacity`.
- **Dead code removed:** the unused trait methods `window_id` and `partition_generation`, and the unreachable `Role::StringLength|BytesLength|Symbol|FieldId` arm.

### 2.3 `H` (Store Hydration)

- **:191** `pub fn RetainedConfigStoreHydration::demand_bytes(&self) -> usize` returns the exact byte gate of the next `advance` turn:
  - `Begin`: `maximum_value_bytes`;
  - `BeginEdit`: 2 × the id;
  - payload: `max(payload, size_of::<M>())`;
  - metadata: `max(meta, size_of::<MutationMeta>())`;
  - `FinishEdit`: the id.
- **:137 / :145** `metadata_retained_bytes` and `payload_bytes` were extracted, so the gate and the demand share one formula.

### 2.4 Tests and Fixture (Schema-First)

- **`C/🧬️schema/📥️retained-pack-load/🔣️.json`**
  - New required `grant` object: `derivation: exact-next-allocation-or-release-demand`, the floor, the turn-bound constant, `packBound: owner-maximum-publication-bytes`, `window-config.load-bound`, `window-config.capacity`.
  - New required `savedCamera` object.
  - New outcome `typed-fault-never-ok`.
  - `minItems` 12.
- **`C/🧫️fixtures/📥️retained-pack-load/🔣️.json`**: those objects, plus 3 scenarios: `saved-camera-round-trip`, `over-bound-pack`, `turn-bound-exhausted`.
- **`C/🧪️tests/📥️retained-pack-load/🟦️.ts`**: Ajv validation plus the new scenario, grant and camera assertions. This is the language-agnostic test.
- **`C/🧪️tests/📥️retained-pack-load/🦀️.rs`**
  - A real schema-first `RetainedLoadCameraConfig { #[dsl(block)] viewport: Viewport2d }`: enveloped Pack, record spec, and a `Snapshot` mutation with `OpText`/`OpBinary`. It mirrors the generation2d window configs, which serve as the oracle shape.
  - The owners use it instead of `NoConfig`.
  - Laws:
    - `window_config_retained_pack_load_current_registry_identity_and_reopen_baseline` (:197): now genuinely restores the 3 Pack+SPR identities.
    - `window_config_retained_pack_load_round_trips_a_saved_camera_with_history` (:232): a dispatched camera edit survives reopen; snapshot and bytes are equal.
    - `window_config_retained_pack_load_refuses_exhausted_turn_bounds_and_over_bound_packs_with_typed_faults` (:265): checks `load_within(.., 1)` yields `load-bound`, installs nothing, and is parked; an over-bound Pack yields `capacity`; the same Pack then loads.
- **`C/🧪️tests/🪪️pack-identity/🦀️.rs`**: the owner state changed to the same camera config. It had been red at HEAD for the `NoConfig` reason.

## 3. Tests Run (Foreground; Logs in `T/🗑️generated/W0-J/`)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-plugin --features artifact-app-testing --lib -- retained_pack_load` at HEAD | 0 / 1: `pack-envelope` (`baseline-head.txt`) |
| `… --lib -- window_config` after the tests were written | 1 / 4 fail. The silent `Ok` is reproduced: 0 restored, and a foreign window was admitted (`test-red-1.txt`) |
| same, after the fixes | **5 / 5 pass** (`test-window-config-green-2.txt`); no warnings in the lane files |
| `bun` running `testWindowConfigRetainedPackLoadFixture` (scratch runner) | pass, `requiredScenarios=12` |
| `cargo test --no-fail-fast -p semio-framework-plugin --features artifact-app-testing -- tool_run` | 29 pass / 1 fail (`test-tool-run-2.txt`). The failure is `tool_run_overlay_append_per_tick_stays_below_two_milliseconds…` (14 of 771 ticks over 2 ms at load average 54). **Isolated rerun: pass** (`test-perf-isolated.txt`). This is a timing flake; no tool-run code was touched. |
| `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib -- window_camera` | **3 / 3 pass**, including `generation2d_window_camera_ownership_runtime_isolates_routes_renders_and_reopens` (`test-gen2d-window-camera.txt`) |
| `cargo check -p semio-framework-plugin --features component-guest --target wasm32-wasip2` | exit 0; the crate reached 22 warnings, none in lane files (`check-wasm-3.txt`) |

## 4. Other Retained Loads With the Same Grant Pattern (Step 4)

I ran `rg ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` over non-test code in `🧰️framework` and `✏️s`, and `rg reserve_page(`.

- **`RetainedPackSourceCursor::reserve_page`** has production callers only in window-config (now fixed). The generation2d and generation3d mounted-pack sessions (`🧬️schema/📸️snapshot/💾️binary`) expose `next_retained_allocation_bytes`/`reserve_retained_allocation`. Only their laws drive them, with exact demand, so they don't truncate.
- **The flat-`wrappers` token builder** (#4) is unique to window-config (`rg wrappers.pop()`).
- **The document envelope decode pump** (`🔌️plugin/🦀️.rs:24862`, store `:8680`, `:9607–9613`) gates on `≥ PAGE` over `[u8; PAGE]` record pages and already carries `maximum_*_close_byte_demand.max(PAGE)` demand accounting. It is not the 4 104 B page pattern and was left alone.
- **The remaining hits are page-sized buffers or close-ladder grants, not retained-source reservations:** puzzle 3d/5d wasm bridges, writer/presentation io, gis/process/raster/drawing/jack.

## 5. Commands to Register in `launch.json`

- `cargo test -p semio-framework-plugin --features artifact-app-testing --lib -- window_config`
- `bun` test of `C/🧪️tests/📥️retained-pack-load/🟦️.ts` (`testWindowConfigRetainedPackLoadFixture`), if the TS fixture runner is not already registered.

## 6. Deviations, Foreign Edits, Open Items

- **Deviation: demand-derived grants, not a static per-owner constant.**
  - The grant is the exact next demand with a one-page floor.
  - The owner's schema bound enters twice: as hydration's `Begin` demand, and as the `pack.len() ≤ MAXIMUM_PUBLICATION_BYTES` admission check.
  - Reason: allocation sizes, such as paged-list branch and leaf pages, are not derivable from the record size alone.
- **Deviation: parked loads retire under their own release demand**, even when the close ladder's grant is smaller. Releasing already-admitted credits is not new work, and the alternative is a `Drop` panic.
- **Foreign edits:** none. `H` is the store half of the window-config load path and has no other consumer.
- **Open items:**
  - **Plugin reload laws calling `load_window_config_pack` used to pass against a silent `Ok`.** They now fail loudly if their state is not a real Pack: layout, sequence, flow, writer, equation, puzzle 2d/3d/5d.
    - Writer's `--lib` test target does not compile at HEAD (`artifact_app_laws` unresolved; this lane did not cause it).
    - Puzzle 2d fails 2 laws (`test-puzzle2d-window.txt`):
      - `…cameras_isolate_render_and_reload…` fails with "rendered scene must carry cameraJson". I could not tell whether that is before or after the reload: the async frame has no line.
      - `…transient_isolates_abort_and_resets_on_reload` fails on "close did not reach terminal-empty". That law loads no window config.
    - Neither is proven to be caused by or independent of this lane. A plugin lane should triage them.
  - `crate::app::NoConfig` still encodes an empty, envelope-less Pack. It is unusable as a window-config `State`. `register` does not reject such owners, because W0-I's tool-run `ToyWorldWindowConfig` and the retirement fixture use the JSON `TestConfig`.

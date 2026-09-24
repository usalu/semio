# WP-G8: Plugin-Host Suite Reds (ui-patch ×3, schema-parity ×2, owned-codec ×1) (Session 10)

Slice G8 · session 10 · 2026-09-24. Native only. Continues G7 §8.4.
Status: IN PROGRESS.

## 1. ui-patch ×3 (`registered scale component path`): three stacked causes, all fixed

1. **The laws read an out-of-band env var.** `SEMIO_UI_PATCH_SCALE_WASM` was set only by the nx lane `ui-patch-marshalling-check --native`. A plain `cargo test --lib` (the suite) therefore panicked before it did anything. The test now resolves the registered artifact from `CARGO_MANIFEST_DIR` (`SCALE_COMPONENT`, the same path as `SCALE_COMPONENT_ARTIFACT` in `📜️script.ts`). It fails loudly and names the nx target when the artifact is not materialized. The env var is gone from `📜️script.ts`. The plugin-host `test` nx target now `dependsOn` `@semio-tech/framework-os-scale-fixture:build-wasm`, like the native lane.
2. **The fixture guest could not build.** `⚖️scale/🦀️.rs` still implemented an intermediate `world actor`: a four-argument `poll(events, command_page, cold_pair_page, budget)`, `CommandIngressPage`, and no `codec`. The schema now has `stage-command-page`/`stage-cold-pair-page` as separate exports, a two-argument `poll`, and the `codec` interface. The rebuild failed with 8 errors (`generated/scale-build-wasm.txt`). The fixture now follows the schema:
   - `stage_command_page` stores the cursor. The next `poll` answers it once with backpressure (kind 2), which is the fixture's old reply to a page.
   - `stage_cold_pair_page` and all four `codec` calls return the fault `scale fixture owns no artifact kind`. The fixture holds no document, and before this change it silently ignored a cold page.
   - wasm32 `cargo check` + `build-wasm` both ran in one wasm-mutex hold: EXIT 0, 922 KB component (`generated/scale-build-wasm2.txt`).
3. **One law pinned an obsolete refusal reason.** `imported_then_returned_channels_refuse_atomically…` expected `actor-patch.unpaired-authority`. Since the 09-15 batch rule (one receipt authorizes N patches), a mixed turn passes pairing. It is refused by `wit_ui_patches_to_kernel` instead, with `ui patch turn mixes the imported and returned channels`. The verdict (rejected, both turns) is the same; the law now asserts the current reason.
   - The same 09-15 change had also left the neutral fixture's schema and the TS oracle stale. `📥️ui-patch/🧬️schema/🔣️.json` did not declare `maximumPatches` and capped case counts at 2. The oracle still used the one-patch rule. Both now follow the batch rule: accepted iff the turn does not use both channels, has ≤ `maximumPatches` patches, and carries a receipt iff it has a patch. `bun 📜️script.ts ui-patch-marshalling-check`: **passed** (ajv, 9 cases, 4 component cases). It was red before.
- Measured: `ui_patch` **6/6** (`generated/uipatch2.txt`).

## 2. schema-parity ×2 (actor world boundary): stale law, fixed

- The schema (`🔌️plugin/🧬️schema/📜️.wit`) is right. `world actor` exports `codec` (pack-schema-hash, genesis, print-mirror, apply-ops). `reactor` has had `stage-command-page` and `stage-cold-pair-page` next to `poll` for some time. All of them are `async func`.
- The law (`🖥️host/🪞️schema-parity/🧪️tests/🔬️unit/🦀️.rs`) still pinned the four-export world and the eight-function async set. It now pins the five exports and all 14 functions, and `every_actor_export_is_async` also walks `codec`.
- Measured: `schema_parity` **7/7** (`generated/targeted1.txt`).

## 3. owned-codec ×1 (`semio:stdio`): stale staged component plus a law that did not fit carrier kinds

The sweep picks the **newest `wasm-release` build** of each component across the `target*` roots under `.🧬semio/🦑️repo/⚡️cache/cargo`.

1. **Stale artifact.** The newest stdio build was `target-tc3d/…/semio_s_plugin_stdio.wasm` (2026-09-22 17:16). The product fix (the stdio `TxtSnapshot` `record_spec()` override that answers `codec.pack-schema-hash(stdio.txt)`) landed on 09-23 at 00:05, after that build. The fault bytes decode to `artifact codec schema has no structural record specification`. That is exactly the fault the fix removes.
   - Fix: I staged W1's current-tree catalog-A build (`.🧬semio/🌐hub/w1-catalog-a/trusted-catalog/build-31ee…/stdio-target/wasm32-wasip2/wasm-release/semio_s_plugin_stdio.wasm`, built 2026-09-24 10:44, sha256 `7984e3da…6a44`) into my own target root `.🧬semio/🦑️repo/⚡️cache/cargo/target-g8/wasm32-wasip2/wasm-release/`. That is the per-slice root the law's doc describes. Capture: `generated/stdio-stage.txt`. I did no wasm build of my own.
2. **Law defect, exposed once hashing passed.** The next failure was `codec.print-mirror(stdio.txt) printed no dsl at all`. `s.stdio.txt` is `CARRIER_TEXT`: its text is the raw file, verbatim (`impl store::ArtifactDsl for TxtSnapshot` → `to_body`). An empty genesis document therefore prints exactly `""`, so the product is right.
   - Fix: each sweep row now declares `GenesisMirrorText::{Structured, CarrierRaw}`. Structured kinds (note ×2 and gis) must still print a non-empty DSL. The carrier kind (stdio) must print exactly the empty string. The law got stricter, not looser.
- Measured: `owned_codec_answers_every_call_on_every_staged_component` **ok** (4 rows, 75 s; `generated/targeted2.txt`).

## 4. plugin-host suite ×3
pending

## 5. Regression gates (os-mcp quick, client-e2e)

### 5.1 os-mcp quick: three pre-existing transport flakes fixed at the root

The first run (`generated/mcp-quick1.txt`) passed **440/441**. The one red was G7's `EAGAIN` flake, `transport::quick::terminal_public_fifo…`. None of my edits touch os-mcp. Looping the `transport::quick` module 300× under load 28–50 turned up three races:

| test | root cause | fix | proof |
|---|---|---|---|
| `terminal_public_fifo_preserves_generation_aba…` | `replacement_connection` called `accept()` on the **nonblocking** listener right after `connect`, before the kernel had queued the connection (`WouldBlock`) | accept blocking, then restore nonblocking | 300/300 isolated (`fifo-only-loop300.txt`) |
| `partial_http_read_and_parser_turn_advance…` | the peer's loopback write was not yet readable when the first read grant ran (4/100 isolated) | `await_readable`: a blocking `peek` on the server stream before the grant | 300/300 isolated (`transport-loop2.txt`) |
| `the_elicitation_deadline_is_real_wall_clock…` (1/120) | **product**: `ElicitationChannel::request_*` compared the difference of two *floored* ms readings with the budget, so a wait could end up to 1 ms **before** its budget | the deadline is now `start + budget + ELICITATION_CLOCK_TICK_MS` (one clock tick), so the wait never ends early | module loop **150/150**, 0 failures (`transport-loop4.txt`, load 18–27) |

The os-mcp quick rerun is in §5.3.

## 6. Files changed
pending

## 7. Honest gaps
pending

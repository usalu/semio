# 🧪️ P5c mounted-layout/text law census — stale marker re-derivation

Continuation of W13d §8 (`📓️w13d-suites-to-zero.md`). Method: §2 (mechanical re-read of production
source via `interactivityProductionSource`, same clause count and intent, new needles).

## Verified

```text
bun -e "import { interactivityMountedLayoutTextSelfTests } from './📜️script.ts'; interactivityMountedLayoutTextSelfTests('/Users/ueli/Documents/semio');"
→ P5c PASS (28 mutations + 15 law needles discriminate; clean baseline green)
```

## Clause table

| stale failure | why it drifted | new anchor |
| --- | --- | --- |
| forbidden `Vec<` | production admits `child_scratch: Vec<usize>` for bounded sibling walks | forbid dynamic **working-set** `Vec<` prefixes (`nodes:`, `glyphs:`, …); require `child_scratch: Vec<usize>` |
| forbidden `while ` | production uses `while let` in ancestor/sibling walks | drop blanket `while `; keep `loop {` and test-harness `while !job` |
| worker turn | pool-thread witness moved under `#[cfg(test)]` (stripped from production census) | production `worker_one` cancel/yield/stage/fuel/deadline needles; pool thread stays on `mounted_layout_worker_runs_on_shared_user_visible_lane_and_pool_thread` law |
| preview retention | `progressive_*` accessors are `#[cfg(test)]`; stale check gained `revision` | `step_layouts` retention: `take_preview_one`, `latest_glyph_preview()`, `window.layout_preview` / `window.glyph_preview`, generation **and** revision filters |
| snapshot authority | `accepted_layout_generation()` is `#[cfg(test)]` | `accepted_layout` gate `mounted.generation == self.mounted_layout_generation` plus `commit_inactive_layout` write |
| paint / hit / slots | slots production path is `scene_slot_for_node` with `?`, not test collector `else { return }` | anchor `scene_slot_for_node` + `let layout = tree.accepted_layout(id)?`; mutation `slot-live-layout` needle updated in `🟦️.ts` |
| shared-pool drive | `pool.pump(now)` → wasm-scaled `pool.pump(now / 1_000)`; driver region boundary moved | `pool.pump(now / 1_000)`; driver slice ends at `//#region 📄️RetainedDocumentConsumer` |
| route order | `render_ui_document` removed; phased `render_ui_document_step` | Viewport (`set_viewport`) → Layout (`drive_mounted_layout_text_one`) → Paint (`frame_into_step` after `Paint` arm, not comment mentions) |

## Deferred (peer lane)

`verify interactivity p5e` still fails first on `interactivityLiveReconcileSelfTests` /
`per-surface-credit-cap` mutation needle — unchanged here (W13d §8).

## Files

- `📜️script.ts` — `interactivityMountedLayoutTextFailures` markers above
- `🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts` — `slot-live-layout` mutation needle

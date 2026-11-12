# Timeline Audit (haiku-timeline-audit, transcribed by coordinator)

Read-only audit; the auditor has no write tool, so the coordinator recorded its result. Nothing below was executed; runtime confirmation is assigned to the timeline owner.

## Manim mechanics (venv `.🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/manim`)

- `scene.py:1128-1129` sample times `np.arange(0, T, 1/F)` → `N = ceil(T*F)` frames, `alpha_k = k/(T*F)`, max alpha `(N-1)/(T*F) < 1`. Float noise may add one frame.
- `scene.py:1141-1167` `T <= 0` raises; `0 < T < 1/F` clamps to `1/F` (one frame at alpha 0 = source).
- `scene.py:1409-1411` `finish()` (`interpolate(1)`) and `clean_up_from_scene` run after the last frame; the exact target first appears on the next rendered frame (next play/wait). If a morph ends a segment no frame shows the exact target.
- Skip mode samples alpha 1 once and writes no frames; adds `scene.duration` to `renderer.time`.
- Frozen waits write `int(T*F)` frames (floor), plays write `ceil` → per-play clock drift up to one frame.
- `AnimationGroup`/`LaggedStart` map `rate_func(alpha) * max_end_time` with clamped sub-alphas.

Consequence: today a 10-frame morph at frames 100–109 shows the target first at 110; a one-frame play shows the source. The CONTRACT section 4 remap fixes both inside the shared evaluator.

## Teaching pipeline

- `⚙️runtime/🐍️.py:30` maps `-ql/-qm/-qh` to 480p15/720p30/1080p60; `:817-818` fps/size; `:526-531` Manim command with `--disable_caching`.
- `:578-608` clause cutting by seconds with re-encode `-ss/-t`, not frame-aligned; `:331-343` concat stream copy; `:306-322` cursor offsets by probed durations.
- `🪟️presentation` is an offline batch renderer (browser PNG frames → ffmpeg), not an interactive player; no `<video>` seek/scrub/loop. `_transition` samples endpoints inclusively (`count = max(3, round(d*fps))`).
- Browser `🟨️render.mjs:146` accumulates per-animation `currentTime` across a batch; `🥞️hierarchy/🖌️render/🐍️.py:245-248` elapsed depends on the previous frame in the batch → frame state depends on batch composition.

## Nondeterminism risks (ranked)

1. `dt`-integrating updaters (`🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:644,811`, `🧱️conduction/🐍️.py:469`, `🏗️bim/🎬️clip/🔗️consistency/🐍️.py:584,594`) diverge between skip and render.
2. `🎨️visuals/🧍️staffage/🐍️.py:1060-1084` patches `Scene.update_mobjects`/`update_to_time` globally at import; a second clock `SCENE_CLOCK` anchored at first update (`:1191-1203`), `renderer.time` is a running float sum.
3. Per-play drift (ceil vs floor).
4. Moving/static lists computed once per play.
5. Skip/cache-hit paths bypass rendering.
6. Browser batch state (above).
7. Real-time settle loops in `render.mjs:149,197`.
8. Mixed `round`/`ceil` in presentation/collection frame counts.
9. `tbr`-based fps probing.
10. Millisecond-rounded audio delay.
11. `np.arange` float frame count.
12. Old ticket preview tools (`preview.py`, `painter.py`) are name-keyed, process-wide monkeypatches, run the full morph.

Randomness: seeded (`visuals/🐍️.py:1146-1150`).

## Missing tests

No test checks the last in-play sample or first post-play frame of a morph; no one-frame identity test; no frozen-wait vs play frame-count test; no skip-vs-render parity for dt updaters.

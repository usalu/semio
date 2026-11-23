import re
from pathlib import Path

tk = Path(r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️13\APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS")
gen = tk / "🗑️generated" / "tests-final"
prefix = "🎓️teaching.🏛️architecture.⚡️energy."
final = {}
for name in ("results.txt", "results2.txt", "results3.txt"):
    path = gen / name
    if not path.exists():
        continue
    for line in path.read_text(encoding="utf-8").splitlines():
        parts = line.split(" ", 2)
        if len(parts) == 3:
            final[parts[2].strip()] = (int(parts[0]), parts[1], name)
logs = {}
for log in (gen / "logs").glob("*.log"):
    logs[log.name] = log.read_text(encoding="utf-8", errors="replace")
import hashlib
rows, total, bad = [], 0, []
for module in sorted(final):
    rc, secs, source = final[module]
    safe = hashlib.md5((module + "\n").encode()).hexdigest()[:8]
    short = re.sub(r"[^A-Za-z0-9_.-]", "", module.removeprefix(prefix))[:60]
    text = logs.get(f"{short}-{safe}.log", "")
    ran = re.findall(r"^Ran (\d+) tests?", text, re.M)
    count = int(ran[-1]) if ran else 0
    total += count
    result = "OK" if rc == 0 else ("TIMEOUT" if rc == 124 else "FAIL")
    if rc:
        bad.append(module)
    rows.append(f"| `{module.removeprefix(prefix)}` | {count} | {result} | {secs} |")
body = f"""# Energy course tests, final run

Per-module runs (`python -m unittest -v <module>`, `SEMIO_TEST_MANIM=1`, up to 3 in parallel, 20 min timeout each). Logs: `🗑️generated/tests-final/logs`. Runner: `tests-final/run-one.sh`, `run-one-path.sh`; module list `tests-final/modules.txt` (66 modules, `tests/inventory.py` finds none unregistered).

## Environment
- No render profile is active outside `script.ts test`, which sets `SEMIO_TEACHING_PROFILE=final` and `SEMIO_TEACHING_LANGUAGE=en`. Pass 1 without them gave dozens of ProfileError/language errors (environment only); the final column is from the run with these two variables (the visuals integration module ran with `preview`/`en` as asked).
- `repo` on PATH and the installed prototype are too old for `tools exec --light` (`unknown flag: --light`). Modules that spawn `repo` (browser lifecycle, output, hierarchy, interaction) ran with `SEMIO_REPO_BINARY` and `PATH` pointing at `%LOCALAPPDATA%/Semio/repo/aa03a414...` which supports it. This is machine state, not code.

## Result
{len(rows)} modules, {total} tests run, {len(bad)} module(s) not green.

| Module (below `⚡️energy`) | Tests | Result | Seconds |
|---|---|---|---|
""" + "\n".join(rows) + """

## What I changed
- `🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only.py` `clip_sources`: `ROOT.rglob` walked into `.venv` and `node_modules` before filtering and hung for 20 minutes (the first test needing it, `test_every_call_site_mirror_is_declared_with_a_reason`); now `os.walk` with pruning of `skip_folders`, same result set, module takes about 100 s.
- `🏗️authoring/🎨️visuals/🧪️tests/🔣️staffage_only.json`: allowlisted `ventilation/🌡️heat-flow::unit_block` in `rule_a.allowed` (a framed formula card from the new unit comparison beat, not a physical object).

## What is left
1. `🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🐍️.py`, `test_four_fps_transitions_capture_current_native_clicks_at_every_required_level`, subtest `phase='focus'`: events per target are `['pointerdown','pointerdown','pointerup','click']`, expected one `pointerdown`. Reproduced twice (about 470 s). Cause is in runtime, not in the test: `🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟨️render.mjs` line 157 (`if(frame.cursor?.pressed&&!down){await page.mouse.down();down=true}`) runs for every pressed frame because each frame opens a fresh page (line 140-141, `down=false`), so a press that spans two or more frames reports one `pointerdown` per frame. Fix there: when `previous?.cursor?.pressed` and the current frame is pressed too, replay the press as the release branch does (lines 143-150: render previous, move, `mouse.down()`, then clear `window.semioPresentation.pointerEvents.length=0`) before rendering the frame. I did not change it, since the final videos were just rendered from the current sources.
"""
(tk / "tests-final" / "REPORT.md").write_text(body, encoding="utf-8")
print(len(rows), total, bad)

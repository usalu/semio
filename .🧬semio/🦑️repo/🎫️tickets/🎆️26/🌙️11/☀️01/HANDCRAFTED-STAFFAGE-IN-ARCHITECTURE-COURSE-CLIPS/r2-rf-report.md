# 🔧 Round 3 regressions (rf)

1. e04 (line 4): green now (5 tests OK). `_occupant_height` no longer exists anywhere in `🎓️teaching` (repo-wide grep clean); another agent's concurrent edit removed the dangling reference, and the radiator test (agent ap) passes as well. No edit by rf.
2. e16 (line 16): transient. Re-run: 7 tests OK (191 s), including `test_real_subprocess_renders_use_the_selected_audience`. No file written into authored sources was found.
3. e28 (line 28): not a code defect. Reproduced three times. `🪟️presentation/🧪️tests/🐍️.py:305` cancels the render session with a hard `threading.Timer(300, ...)`. Under the 20-agent load the UI frame batch (26 frames, one `render.mjs` screenshot loop) takes more than 300 s, so the cancel kills node: `CalledProcessError` (exit 1) or `InterruptedError: Render cancelled`. Node stderr showed only `[repo] still working` and `[DEBUG] Actual UI browser frame 1/26`, no node error. Fix: deadline raised to 1500 s (hang guard only). Single test re-run: OK in 486 s.

Open: node frame loop is slow under load (about 6 to 15 s per frame); not addressed.

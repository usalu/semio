# Fix Manim Sideview errno 89

`manim-sideview.runOnSave` SIGTERM’d a live render, then the next import stalled reading `.venv/.../click-.../METADATA` (`OSError: [Errno 89] Operation canceled`).

Changes:

- `runOnSave`: false
- manim path: `manim` (Sideview already prefixes `.venv/bin/`; `${workspaceFolder}` is not expanded)
- version: v0.21.0
- replaced wedged click dist-info inode
- launch entry for Beat3_Konvektion

## 2026-09-20: command looks frozen

`.venv` lives in iCloud Drive `Documents`. Hundreds of site-packages files are `dataless` (cloud placeholders). `manim` prints nothing because it hangs on the first `import numpy` / `importlib.metadata` scan. Sideview then SIGTERM’s the hung job (`code=15`).

Working preview (isolated venv, media on local disk):

```bash
cd "tutorial/energy/demand/Heating/1_introduction"
/tmp/semio-manim-venv/bin/python -m manim -pql --disable_caching --media_dir /tmp/semio-manim-media scene_1.py Beat3_Konvektion
```

Output: `/tmp/semio-manim-media/videos/scene_1/480p15/Beat3_Konvektion.mp4`
Copy in this ticket: `Beat3_Konvektion.mp4`

Do **not** run `python scene_1.py` — that file has no `__main__` renderer.

## 2026-09-20: Sideview preview

`.venv/bin/manim` now execs `$HOME/Library/Caches/semio-manim/venv/bin/python -m manim` so Sideview does not import iCloud-evicted site-packages. Keep `manim-sideview.defaultManimPath` as `manim`. Click the rotation icon once and wait; the panel plays `./media/videos/<module>/<quality>/<Scene>.mp4`.

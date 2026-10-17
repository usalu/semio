"""🧬 rw2t: applies the import rewrite to every WP2 law (one index build). usage: apply_all.py [--apply] [law ...]"""
import sys
from apply_law import *
from ed import backup as _bk, note as _note
PLACE = {
 "wgpu-canvas2d": "scene-ink-tests", "wgpu-raster-frame-cost": "scene-ink-tests", "wgpu-event-feed": "scene-ink-tests", "wgpu-graph-timeline": "scene-ink-tests",
 "wgpu-table": "scene-render", "wgpu-block-list": "scene-render", "wgpu-virtual-file-system": "scene-render",
 "wgpu-gumball-dispatch": None, "wgpu-ui-image-loading": None, "wgpu-ink-canvas": None, "wgpu-diff-view": None, "wgpu-text-editor": None,
 "wgpu-unit": None, "png-export": None, "gpu-export": None, "svg-export": None, "wgpu-icon-render": None,
 "wgpu-surface-context-menu": None, "wgpu-render-entry": None,
}
if __name__ == "__main__":
    do = "--apply" in sys.argv
    names = [a for a in sys.argv[1:] if not a.startswith("--")] or list(PLACE)
    for law in names:
        body, text, path, lines = run(law, PLACE[law])
        if do and body is not None:
            _bk(path)
            assert read(path) == text
            with open(path, "w", encoding="utf8") as h:
                h.write(body)
            _note(path)
            print("   applied", path[-60:])

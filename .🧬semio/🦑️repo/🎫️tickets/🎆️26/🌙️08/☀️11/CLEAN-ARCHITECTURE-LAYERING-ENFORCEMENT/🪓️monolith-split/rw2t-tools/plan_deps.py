"""📦 rw2t: manifest additions of a destination crate for the laws it receives (first-party edges, external lines)."""
import sys
from needs import *

# law -> destination
MOVES = {
 "scene-ink-tests": ["wgpu-canvas2d", "wgpu-raster-frame-cost", "wgpu-event-feed", "wgpu-graph-timeline"],
 "scene-render": ["wgpu-table", "wgpu-block-list", "wgpu-virtual-file-system"],
}
EXTRA_FIRST = {  # heads that the generator rewrites to imports from crates (owner names known by the plan)
 "scene-ink-tests": ["host-base", "frame-actions", "scene-render", "scene-tables", "scene-feeds", "canvas2d-host", "ui-engine", "contracts", "scene-state", "scene-ink", "scene-canvas2d", "engine-board", "engine-registry"],
 "scene-render": [],
}
if __name__ == "__main__":
    setup()
    pool = all_lines()
    for dest, laws in MOVES.items():
        register_crate(dest) if dest in CRATES else None
        have = manifest_keys(dest)
        ext = collections.OrderedDict()
        first = set()
        for law in laws:
            for kind, h, sfx in missing(law, dest):
                if kind == "external" and h in pool and h not in ("collect",):
                    ext[h] = pool[h][1]
                elif kind == "first-party":
                    first.add(sfx)
        print("#### dest", dest, "have first-party:", sorted(k for k in have if k.startswith("semio_framework_os_renderer_wgpu_")))
        print("  first-party heads missing:", sorted(first))
        for h, l in ext.items():
            print("  EXT", l)

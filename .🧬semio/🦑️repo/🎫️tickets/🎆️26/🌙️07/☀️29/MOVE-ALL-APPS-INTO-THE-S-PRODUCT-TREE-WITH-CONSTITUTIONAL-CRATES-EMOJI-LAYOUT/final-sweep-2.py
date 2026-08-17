#!/usr/bin/env python3
"""Final sweep #2: sibling files/dirs stranded at plugin-root level after the pattern-exact re-layer."""
import os, shutil

def merge_move(old, new):
    if not os.path.exists(old):
        print(f"SKIP (missing): {old}")
        return
    if not os.path.exists(new):
        os.makedirs(os.path.dirname(new), exist_ok=True)
        shutil.move(old, new)
        print(f"MOVE {old} -> {new}")
        return
    if os.path.isdir(old):
        for entry in os.listdir(old):
            merge_move(os.path.join(old, entry), os.path.join(new, entry))
        if not os.listdir(old):
            os.rmdir(old)
            print(f"RMDIR {old}")
    else:
        print(f"COLLISION (skipped, needs manual check): {old} vs {new}")

# architect: spine siblings -> module/spine/
merge_move('s/plugin/architect/spine/📜️script.ts', 's/plugin/architect/module/spine/📜️script.ts')
merge_move('s/plugin/architect/spine/project.json', 's/plugin/architect/module/spine/project.json')

# flow: orphan compute.ts -> module/compute/js/
merge_move('s/plugin/flow/compute.ts', 's/plugin/flow/module/compute/js/index.ts')

# gis: 2d/3d sibling data -> app/2d, app/3d
merge_move('s/plugin/gis/2d', 's/plugin/gis/app/2d')
merge_move('s/plugin/gis/3d', 's/plugin/gis/app/3d')

# procedural: 2d/3d sibling data -> app/2d, app/3d (play stays: doc-only stub, no app crate)
merge_move('s/plugin/procedural/2d', 's/plugin/procedural/app/2d')
merge_move('s/plugin/procedural/3d', 's/plugin/procedural/app/3d')

# process: 3d sibling data -> app/3d
merge_move('s/plugin/process/3d', 's/plugin/process/app/3d')

# puzzle: 2d/3d/5d sibling data -> app/2d, app/3d, app/5d
merge_move('s/plugin/puzzle/2d', 's/plugin/puzzle/app/2d')
merge_move('s/plugin/puzzle/3d', 's/plugin/puzzle/app/3d')
merge_move('s/plugin/puzzle/5d', 's/plugin/puzzle/app/5d')
# puzzle/asset: JS-only supporting module (not data) -> module/asset/js
merge_move('s/plugin/puzzle/asset', 's/plugin/puzzle/module/asset/js')

# reasoning: mindmap build-wrapper siblings -> s/module/mindmap/ (the crate itself already lives there)
merge_move('s/plugin/reasoning/package.json', 's/module/mindmap/package.json')
merge_move('s/plugin/reasoning/project.json', 's/module/mindmap/project.json')
merge_move('s/plugin/reasoning/📜️script.ts', 's/module/mindmap/📜️script.ts')
# reasoning: wires sibling data -> app/wires
merge_move('s/plugin/reasoning/wires', 's/plugin/reasoning/app/wires')

print("done")

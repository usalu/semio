"""🔬 rw2t: per law file, the first-party owners of its free names, external aliases, unknown tokens and crate:: heads."""
import sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__))) if False else None
from idx import *
import os
LAWS = {
 "canvas2d-host": "📐️Canvas2dHost/🧪️tests/🧪️wgpu-gumball-dispatch",
}
def laws_of(tree, sub):
    return os.path.join(EL, tree, "🧪️tests", sub, "🦀️.rs")
LAWFILES = {
 "wgpu-gumball-dispatch": EL + "/📐️Canvas2dHost/🧪️tests/🧪️wgpu-gumball-dispatch/🦀️.rs",
 "wgpu-canvas2d": laws_of("🎞️Scenes", "🔬️wgpu-canvas2d"),
 "wgpu-ui-image-loading": EL + "/🗣️Interpreter/🧪️tests/🖼️wgpu-ui-image-loading/🦀️.rs",
 "wgpu-raster-frame-cost": laws_of("🎞️Scenes", "🔬️wgpu-raster-frame-cost"),
 "wgpu-ink-canvas": laws_of("🎞️Scenes", "🔬️wgpu-ink-canvas"),
 "wgpu-event-feed": laws_of("🎞️Scenes", "🔬️wgpu-event-feed"),
 "wgpu-graph-timeline": laws_of("🎞️Scenes", "🔬️wgpu-graph-timeline"),
 "wgpu-table": laws_of("🎞️Scenes", "🔬️wgpu-table"),
 "wgpu-block-list": laws_of("🎞️Scenes", "🔬️wgpu-block-list"),
 "wgpu-diff-view": laws_of("🎞️Scenes", "🔬️wgpu-diff-view"),
 "wgpu-virtual-file-system": laws_of("🎞️Scenes", "🔬️wgpu-virtual-file-system"),
 "wgpu-text-editor": laws_of("🎞️Scenes", "🔬️wgpu-text-editor"),
 "wgpu-unit": laws_of("🖼️IconRenderHost", "🔬️wgpu-unit"),
 "png-export": laws_of("🖼️IconRenderHost", "📤️png-export"),
 "gpu-export": laws_of("🖼️IconRenderHost", "📤️gpu-export"),
 "svg-export": laws_of("🖼️IconRenderHost", "📤️svg-export"),
 "wgpu-icon-render": laws_of("🎞️Scenes", "🔬️wgpu-icon-render"),
 "wgpu-surface-context-menu": laws_of("🎞️Scenes", "🔬️wgpu-surface-context-menu"),
 "wgpu-render-entry": laws_of("🎞️Scenes", "🔬️wgpu-render-entry"),
}
def analyze(name, ix, alias):
    path = LAWFILES[name]
    text = read(path)
    toks = free_tokens(text)
    loc = local_names(text)
    owners = collections.defaultdict(list)
    ext, unk = {}, []
    for w, n in toks.items():
        if w in loc or w in PRIMS:
            continue
        if w in ix:
            owners[w] = ix[w]
        elif w in alias:
            ext[w] = sorted(alias[w])
        else:
            unk.append(w)
    heads = collections.Counter(m.group(1) + "::" + (m.group(2) or "") for m in re.finditer(r"(?<![\w:])crate::(\w+)(?:::(\w+))?", blank(text)))
    return dict(owners=owners, ext=ext, unk=unk, heads=heads, text=text)
if __name__ == "__main__":
    ix = build_index()
    alias = alias_table([s for s in CRATES if s in ("scene-render", "scene-state", "scene-canvas2d", "scene-ink", "scene-feeds", "scene-tables", "scene-text-vfs", "icon-host", "canvas2d-host", "engine-registry", "engine-attach", "engine-board", "engine-graph", "engine-text")])
    for n in (sys.argv[1:] or LAWFILES):
        a = analyze(n, ix, alias)
        cr = collections.Counter()
        for w, lst in a["owners"].items():
            for c, mp, cf in lst:
                cr[c] += 1
        print("##", n, "owners", dict(cr))
        print("   heads", dict(a["heads"]))
        print("   ext", {k: v for k, v in a["ext"].items()})
        print("   unk", a["unk"][:80])

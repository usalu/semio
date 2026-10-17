"""📐 C4P49 spec generator for the two include-fragment splits (tml, tmu): chunk start tables to `specs/<n>.json` for fragsplit.py (descendant of c4p44-tools/frag/mkspecs.py)."""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
STAGE = "/Users/ueli/.cache/c4p49/stage-frag"
V = "️"
R = "🧰️framework/🔨️modules"
SPECS = {
    1: dict(
        file=R + "/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs",
        orig="tml.rs",
        table=[
            (8, "🧫", "scene-fixtures", "Scene fixtures of the tiled map tests: the 1x1 PNG, the synthetic land tile, the land cover and overlay extensions, the overlay host and the Zurich pin constants."),
            (73, "📷", "camera-and-visible-tiles", "Camera and visible tiles: viewport clamping, the camera zoom, the bounded visible tiles, the vector tile zoom pick, the overzoom, the layer visibility and stroke scale JSON and the visible tiles at the world zoom."),
            (194, "🎯", "feature-picking-and-overlays", "Feature picking and overlays: the raster affine, the overlay JSON, the feature screen projection, the hit tests, the rectangle and line queries, the interaction sync, the pointer up camera emission, the overlay paint order and the map theme JSON."),
            (326, "🎨", "render-scene-and-vector-cache", "Render scene and vector scene cache: the device pixel ratio scene, the render mode, the vector style preset and the cache hits and invalidations of the vector scene."),
            (517, "🧱", "tile-retention-and-polygon-queries", "Tile retention and polygon queries: the raster and vector ancestor retention, the tile rectangle intersection, the point and polyline geometry, the interaction plan, the polygon feature queries and the feature screen coverage."),
            (663, "🧭", "interaction-sync-and-focus", "Interaction sync and focus: the selection replacement, the hover and kind, the declared granularity, the focus on points and lines and the tile presence."),
            (738, "⬆", "tile-uploads-and-prefetch", "Tile uploads and prefetch: the raster and vector uploads and their idempotence, the visible tile revisions, the prefetch tiles and the pinned LRU eviction."),
            (878, "🎚", "theme-lod-and-label-guards", "Theme, level of detail and label guards: the invalid theme, layer visibility and stroke scale JSON, the neutral band list, the pinned band fallback, the opaque fill and the label declutter."),
            (938, "🌍", "mercator-oracle-and-tile-budget", "Mercator oracle and tile budget: the committed Web Mercator fixture, the fixture band policy, the band selection, the host band report and the visible tile request bound."),
        ],
    ),
    2: dict(
        file=R + "/🛠️tool-machine/🧪️tests/🔬️unit/🦀️.rs",
        orig="tmu.rs",
        table=[
            (16, "📄", "fixture-readers", "Transaction law fixture readers: the fixture text and its JSON accessors, the clock and reference decoding, the yield decoding and the transaction constructor."),
            (78, "🧮", "reducer-and-id-laws", "Reducer and id laws: the state, yield and refusal kinds, the reducer matrix and cases, the resume order and the LEB128 and blake3 oracle of the minted ids."),
            (197, "🌊", "gesture-tool-laws", "Gesture tool laws: the example drag gesture statecharts, the gesture runner and its replay of every fixture scenario, the resume rows and the press, drag and release transaction."),
            (518, "🏃", "runner-laws", "Runner laws: the probe statecharts, the refused and unpublished yields and the timer, abort and rest behaviour of the runner."),
            (624, "🫧", "scrub-laws", "Scrub laws: the scrub tool tables, chart and phases against the fixture, the ledger replay, the persisted scrubs and the late inputs."),
            (862, "⌨", "typing-laws", "Typing laws: the typing tool tables, chart and phases against the fixture, the algebra folds, the ledger replay and the persisted run."),
            (1100, "🕸", "node-drag-and-once-laws", "Node drag and once laws: the node drag rows and their commit as one transaction, the one-step tool and the independent dispatches."),
        ],
    ),
}


def back_up(ls, s):
    k = s
    while k > 1:
        prev = ls[k - 2]
        if prev.strip() and not prev.startswith("//#endregion") and not prev.startswith("// #endregion") and (prev.startswith("///") or prev.startswith("#[") or prev.startswith("//")):
            k -= 1
        else:
            break
    return k


os.makedirs(HERE + "/specs", exist_ok=True)
for n, sp in SPECS.items():
    ORIG = os.path.dirname(HERE) + "/orig/" + sp["orig"]
    ls = open(ORIG, encoding="utf-8").read().split("\n")
    if ls[-1] == "":
        ls.pop()
    T = sp["table"]
    starts = [back_up(ls, item) for item, _, _, _ in T]
    assert starts == sorted(starts) and len(set(starts)) == len(starts), starts
    chunks = []
    for i, (item, emoji, kebab, doc) in enumerate(T):
        a = starts[i]
        b = (starts[i + 1] - 1) if i + 1 < len(T) else len(ls)
        while not ls[b - 1].strip():
            b -= 1
        folder = emoji + V + kebab
        chunks.append(dict(name=folder, folder=folder, doc=f"{emoji}{V} {doc}", ranges=[[a, b]]))
        print(n, folder, a, b, b - a + 1, "prev:", repr(ls[a - 2][:50]))
    spec = dict(file=sp["file"], original=ORIG, stage=STAGE, chunks=chunks)
    json.dump(spec, open(f"{HERE}/specs/{n}.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)

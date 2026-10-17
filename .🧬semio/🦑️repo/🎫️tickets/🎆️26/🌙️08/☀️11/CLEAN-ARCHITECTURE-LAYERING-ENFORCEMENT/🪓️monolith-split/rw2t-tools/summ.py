"""📊 rw2t: summary of the WP2-family resolve/privacy rows of an oracle result, with the law-level rows listed. usage: summ.py <result.json> [--all]"""
import sys, re, collections, json
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rw2t-tools")
from rows import *
WP = ('engine_registry', 'engine_text', 'engine_board', 'engine_graph', 'engine_attach', 'scene_state', 'canvas2d_host', 'scene_canvas2d', 'scene_ink', 'scene_feeds', 'scene_tables', 'scene_text_vfs', 'icon_host', 'scene_render', 'engine_attach_tests', 'scene_ink_tests', 'ui_engine_tests')
ART = re.compile(r"element_contracts|host_base|asset_authority|canvas2d_paint|media_slots|map_tiles|infinite_world")
path = sys.argv[1]
for cfg in ("lib", "test"):
    L = load(path, cfg)
    c = collections.Counter()
    for r in L:
        if pk_of(r) in WP and r["cat"] in ("privacy", "resolve") and not r["stub"]:
            art = bool(ART.search(r["msg"]))
            c[(pk_of(r), r["cat"], "artifact" if art else "REAL")] += 1
    print("==", cfg)
    for k, v in sorted(c.items()):
        print("  ", v, k)
    if "--all" in sys.argv:
        for r in L:
            if pk_of(r) in WP and r["cat"] in ("privacy", "resolve") and not r["stub"] and not ART.search(r["msg"]):
                print("    ", pk_of(r), r["code"], r["file"].split("/")[-2], r["line"], r["msg"][:150])
d = json.load(open(path))
print({c: (d["summary"][c]["cur_total"], d["summary"][c]["canary_cur"]) for c in d["summary"]})

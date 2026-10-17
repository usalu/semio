"""🔍 rw2t: NEW rows of a no-base oracle run against the keys of the b1 baseline side (same key function as pos.py).
usage: newdiff.py <label> [cfg ...]   (reads GEN/pos-rw2t-<label>-cur-<cfg>-final.err)"""
import sys, os, json, collections, re
POS = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pos-tools"
sys.path.insert(0, POS)
import pos
GEN = os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated"
SB = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pos-work/rw2t"
WP = ('engine_registry', 'engine_text', 'engine_board', 'engine_graph', 'engine_attach', 'scene_state', 'canvas2d_host', 'scene_canvas2d', 'scene_ink', 'scene_feeds', 'scene_tables', 'scene_text_vfs', 'icon_host', 'scene_render', 'engine_attach_tests', 'scene_ink_tests', 'ui_engine_tests')
def parse(path, cwd):
    err = open(path, encoding="utf8", errors="replace").read()
    diags, reached = pos.parse_json(err, cwd, frozenset())
    return diags, reached
def pk(d):
    m = re.match(r"__m_semio_framework_os_renderer_wgpu_?([a-z0-9_]*)/", d["file"])
    return m.group(1) if m else "?"
def base_keys(cfg):
    cache = GEN + f"/rw2t-basekeys-{cfg}.json"
    if os.path.exists(cache):
        return collections.Counter({tuple(json.loads(k)): v for k, v in json.load(open(cache)).items()})
    diags, _ = parse(GEN + f"/pos-rw2t-b1-base-{cfg}-final.err", SB + "/base")
    c = collections.Counter(pos.key_of(d) for d in diags)
    json.dump({json.dumps(list(k)): v for k, v in c.items()}, open(cache, "w"))
    return c
if __name__ == "__main__":
    label = sys.argv[1]
    cfgs = sys.argv[2:] or ["lib", "test"]
    out = {}
    for cfg in cfgs:
        kb = base_keys(cfg)
        diags, reached = parse(GEN + f"/pos-rw2t-{label}-cur-{cfg}-final.err", SB + "/cur")
        seen = collections.Counter()
        new = []
        for d in diags:
            k = pos.key_of(d)
            seen[k] += 1
            if seen[k] > kb.get(k, 0):
                new.append(d)
        sel = [d for d in new if not d["stub"] and d["cat"] in ("privacy", "resolve", "type") and pk(d) in WP]
        print("==", cfg, "canary", sorted(reached), "cur", len(diags), "new", len(new), "wp2 family non-stub new:", collections.Counter((pk(d), d["cat"]) for d in sel))
        out[cfg] = [dict(code=d["code"], msg=d["msg"][:200], file=d["file"], line=d["line"], cat=d["cat"], pk=pk(d)) for d in sel]
    json.dump(out, open(GEN + f"/rw2t-newdiff-{label}.json", "w"), ensure_ascii=False, indent=0)

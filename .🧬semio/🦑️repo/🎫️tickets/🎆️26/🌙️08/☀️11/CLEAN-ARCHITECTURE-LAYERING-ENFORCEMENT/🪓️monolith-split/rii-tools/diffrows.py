"""🔀 rii: row-level difference of two oracle results for the renderer family crates (key = code, message, source line text, file). usage: diffrows.py <before.json> <after.json> [cat,...]"""
import collections, json, re, sys
FAM = ("engine_board", "engine_graph", "engine_attach", "engine_attach_tests", "ui_engine_tests", "engine_registry", "engine_text", "scene_state", "canvas2d_host", "scene_canvas2d", "scene_ink", "scene_feeds", "scene_tables", "scene_text_vfs", "icon_host", "scene_render", "scene_ink_tests", "contracts", "thread_isolation")
def rows(path, cfg):
    out = collections.Counter()
    ex = {}
    for r in json.load(open(path))["summary"][cfg]["new_list"]:
        if r.get("stub"):
            continue
        m = re.match(r"__m_semio_framework_os_renderer_wgpu_([a-z0-9_]+)/", r["file"])
        if not m or m.group(1) not in FAM:
            continue
        k = (m.group(1), r["file"].split("/")[-2], r["code"], r["msg"][:120], (r.get("line_text") or "").strip()[:100])
        out[k] += 1
        ex[k] = r
    return out, ex
cats = set(sys.argv[3].split(",")) if len(sys.argv) > 3 else None
for cfg in ("lib", "test"):
    a, ea = rows(sys.argv[1], cfg)
    b, eb = rows(sys.argv[2], cfg)
    gone, new = a - b, b - a
    print(f"== {cfg}: before {sum(a.values())} after {sum(b.values())} gone {sum(gone.values())} new {sum(new.values())}")
    for k, v in sorted(new.items()):
        r = eb[k]
        if cats and r["cat"] not in cats:
            continue
        print("  NEW", v, r["cat"], k[0], k[1], k[2], k[3], "|", k[4])

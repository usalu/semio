# 🔎️ Per-crate external identifiers scanned from the live files of Appendix A (union over the target crates of every file), against the plan's 6.2 lists.
import json, os, re, sys, unicodedata
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import appendix_a_drift as ad
R = "/Users/ueli/Documents/semio"
E = ad.E
model = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated/rwp0-planmodel.json"))
IDENTS = ['ui_wgpu','ui_render','ui_contract','ui_host','ui_styling','infinite_world','infinite_canvas','dsl','dsl_core','protocol','store','store_sync','semio_framework_os_kernel',
 'semio_framework_async','semio_framework_job','semio_framework_job_worker','semio_framework_job_worker_authority','semio_framework_value','semio_framework_pack_json','semio_framework_pack','semio_framework_trace',
 'semio_framework_hash','semio_framework_pixels','semio_framework_actor','semio_framework_os_services','semio_framework_plugin_host','semio_framework_plugin_guest_contract','semio_framework_plugin',
 'semio_framework_ui_locale','semio_framework_ui_viewport','semio_framework_io_base64','semio_framework_artifact_reference','semio_framework_dsl_record','semio_framework_tool_run','semio_framework_surface',
 'semio_framework_os_node_graph','semio_framework_2d','semio_framework_canvas','semio_framework_os_renderer_camera','semio_framework','framework_editor','flow','replication','serde','serde_json','image','usvg','tiny_skia','resvg',
 'base64','winit','accesskit','accesskit_winit','tokio','wasm_bindgen','web_sys','js_sys','getrandom','semio_framework_os_config','semio_framework_value_derive','semio_framework_os_shell','semio_framework_time_travel','camera_storage','semio_framework_schema','semio_framework_plugin_interpreter_runtime','vello']
def strip(t):
    t = re.sub(r'/\*.*?\*/', ' ', t, flags=re.S)
    t = re.sub(r'(?m)^\s*//.*$', '', t)
    t = re.sub(r'(?<!:)//[^\n"]*$', '', t, flags=re.M)
    t = re.sub(r'"(?:[^"\\\n]|\\.)*"', '""', t)
    return t
rows = []
active = False
for line in open(ad.PLAN, encoding="utf-8"):
    if line.startswith("## Appendix A"): active = True
    elif line.startswith("## Appendix B"): active = False
    if not active: continue
    m = re.match(r"^\| (\d+) \| `([^`]+)` \| (\d+) \| (\d+) \| (.*?) \| (\d+) \| (.*?) \|$", line)
    if m: rows.append((m.group(2), m.group(7)))
per = {}
for cell, target in rows:
    resolved = None
    for cand in ad.candidates(cell):
        for suffix in ("/🦀️.rs", ""):
            p = os.path.normpath(cand + suffix)
            if os.path.isfile(R + "/" + p) and p.endswith(".rs"): resolved = p; break
        if resolved: break
    if not resolved:
        if cell.endswith("icon-host"): resolved = os.path.normpath(E + "/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖼️icon-render/🦀️.rs")
        else: print("UNRESOLVED", cell); continue
    text = strip(open(R + "/" + resolved, encoding="utf-8").read())
    found = set()
    for i in IDENTS:
        if re.search(r'(?<![\w:.])' + re.escape(i) + r'\s*::', text) or re.search(r'\buse\s+' + re.escape(i) + r'\b', text) or re.search(r'extern\s+crate\s+' + re.escape(i) + r'\b', text) or re.search(r'\b' + re.escape(i) + r'\s*!', text):
            found.add(i)
    targets = [t.strip().split(" ")[0] for t in target.split(",")]
    for t in targets:
        t = t.strip("`")
        per.setdefault(t, set()).update(found)
out = {}
for name, crate in model["lib"].items():
    plan = set(crate.get("external_idents", []))
    live = per.get(name, set())
    out[name] = dict(plan=sorted(plan), live=sorted(live), onlyLive=sorted(live - plan), onlyPlan=sorted(plan - live))
out["shell-*"] = dict(live=sorted(per.get("shell-*", set())))
json.dump(out, open(os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated/rwp0-extscan.json", "w"), indent=1)
for name, o in out.items():
    if o.get("onlyLive") or o.get("onlyPlan"): print(name, "onlyLive", o.get("onlyLive"), "onlyPlan", o.get("onlyPlan"))
print("shell-*", out["shell-*"]["live"])

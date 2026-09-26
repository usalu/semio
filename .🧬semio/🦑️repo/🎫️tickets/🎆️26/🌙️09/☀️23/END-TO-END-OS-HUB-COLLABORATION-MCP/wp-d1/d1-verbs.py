"""🧾️ D1: dumps one plugin's undescribed agent verbs from the census with args and the source lines naming them."""
import json, os, re, subprocess, sys, collections
ROOT = "/Users/ueli/Documents/semio"
plugin = sys.argv[1]
rows = json.load(open(f"{ROOT}/.tmp-ticket/wp-d1/generated/census-before.json"))
REG = json.load(open(f"{ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"))
entry = next(e for e in REG if e["pluginId"] == plugin)
proot = os.path.join(ROOT, os.path.dirname(os.path.dirname(entry["cratePath"])))
desc = json.load(open(os.path.join(proot, "🔣️.json")))
args = {}
for app in desc["manifest"]["apps"]:
    for d in [a for wk in app.get("windowKinds", []) for a in wk.get("actions", [])] + app.get("actions", []) + app.get("commands", []):
        args.setdefault((app["id"], d["id"]), d)
by = collections.OrderedDict()
for r in rows:
    if r["plugin"] != plugin or r["audience"] != "agent" or (r["desc_en"] and r["desc_de"]):
        continue
    vid = r["id"].rsplit(".", 1)[-1] if r["shape"] == "action" else r["id"].split(".cmd.")[-1]
    by.setdefault(vid, []).append(r)
for vid, rs in by.items():
    r = rs[0]
    d = args.get((r["app"], vid)) or {}
    a = ", ".join(f"{x['id']}{'*' if x.get('required') else ''}:{x['label']['native']['en']}" + (f"[{'|'.join(o['value'] for o in x['schema'].get('options', []))}]" if x.get('schema', {}).get('kind') == 'select' else "") for x in d.get("args", []))
    apps = ",".join(sorted({x["app"].split("@")[0].replace("s." + plugin + ".", "") for x in rs if x["app"]}))
    print(f"{vid} | {r['kind']} | {r['title_en']} / {r['title_de']} | destr={r['destructive']} | apps={apps} | args={a}")

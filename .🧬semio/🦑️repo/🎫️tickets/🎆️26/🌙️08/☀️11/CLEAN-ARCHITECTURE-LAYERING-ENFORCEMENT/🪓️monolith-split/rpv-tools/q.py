"""🔍 RPV query: prints the attributed privacy rows of an attr JSON for a member regex (use sites and definition).
usage: q.py <attr.json> <member-regex> [<owner-substring>]"""
import json, re, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))
rx = re.compile(sys.argv[2])
own = sys.argv[3] if len(sys.argv) > 3 else ""
S = lambda s: (s or "?").replace("semio_framework_os_renderer_wgpu", "~")
seen = set()
for r in d["rows"]:
    if rx.search(r["member"] or "") and own in (r["owner"] or ""):
        k = (r["cfg"], r["use_file"], r["use_line"], r["member"])
        if k in seen:
            continue
        seen.add(k)
        sh = lambda f: "/".join((f or "?").split("/")[-3:])
        print(f"{r['cfg']:4} {r['code']} {S(r['owner'])} {r['member']} <- {S(r['use_crate'])} {sh(r['use_file'])}:{r['use_line']}  def {sh(r['def_file'])}:{r['def_line']}")

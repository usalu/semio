import json,sys,collections
T=sys.argv[1]; G=f"{T}/🧰️scripts/audit-requests"
items={it['n']:it for it in json.load(open(f"{G}/items.json"))}
rows=[l.rstrip('\n').split('|',3) for l in open(f"{G}/dec-a.txt",encoding='utf-8') if l.strip()]
dec={int(r[0]):(r[1],r[2],r[3]) for r in rows}
assert sorted(dec)==list(range(1,511)), (len(dec), set(range(1,511))-set(dec))
classes=collections.Counter(v[0] for v in dec.values())
lanes_order=['shared','en1990','en1991','en1992','en1993','en1994','en1995','en1996','en1997','en1998','en1999','din-a','din18599','iso-vdi','ts-schema','browser-e2e','host-runtime']
open_lanes=collections.Counter(v[1] for v in dec.values() if v[0]=='open')
lane_all=collections.Counter(v[1] for v in dec.values())
allowed=set(lanes_order)
bad=[(n,v[1]) for n,v in dec.items() if v[1] not in allowed]
print('classes',dict(classes)); print('bad lanes',bad)
out=[]
out.append("# Fleet 1108 Audit: Requests Open Items Reconciliation\n")
out.append("Lane `audit-requests` (read-only audit). Source list: `📌️important/open-items.md` (510 distinct items, mapped to `📌️important/requests.md`). Current state checked against the norm tree at `✏️s/🔌️plugins/📕️norm` (grep and rg over `.rs/.ts/.py/.json/.feature/.proto/.graphql`, source-boundary scan `🧰️scripts/source-boundary-scan.py -v`, path-length scan). No source or requests were edited.\n")
out.append("Method and limits: `done` means the item is implemented in the tree (cited identifier or file found, or a gate now passes). `stale` means the named code, path or script no longer exists and the concern is superseded. `open` means still needed, or not verifiable from source (build, golden-bless and printed-standard items are open until a green build or a standards check). Classification is grep-based and heuristic; a few `done` and `open` calls are marked unverified in the notes.\n")
out.append("## Summary\n")
out.append("| Class | Count |\n|---|---:|")
for c in ['done','stale','open']: out.append(f"| {c} | {classes.get(c,0)} |")
out.append(f"| **total** | **{sum(classes.values())}** |\n")
out.append("| Lane | Open | Done | Stale | Total |\n|---|---:|---:|---:|---:|")
for l in lanes_order:
    if lane_all[l]==0: continue
    c=collections.Counter(v[0] for v in dec.values() if v[1]==l)
    out.append(f"| {l} | {c.get('open',0)} | {c.get('done',0)} | {c.get('stale',0)} | {lane_all[l]} |")
out.append(f"| **total** | **{sum(open_lanes.values())}** | **{classes.get('done',0)}** | **{classes.get('stale',0)}** | **510** |\n")
out.append("Lane mapping: `shared` covers `📇️registry`, `⚖️compliance`, `🎬️structural-scene`, `🧮️mechanics`, `🔮️oracles`, SURFACE/app-surface and cross-artifact build gates. `en1990`..`en1999` cover each artifact under `🗿️artifacts/`. `din-a`, `din18599`, `iso-vdi` are the non-Eurocode apps (only item 504 is cross-app). `ts-schema` covers codegen, schema generate, twins, Cargo.lock/umbrella and typed-enum conversions. `host-runtime` covers shell, keyboard, drag-drop and hub manifests. `browser-e2e` covers accessibility renderer wiring.\n")
out.append("Overlong paths: 8 fixture paths are 257 bytes from `/home/deck/semio/` (over the 256 limit): `⚙️change-rail-derailment-assumed-line-load/✅apply/🔣️.mutation.json`, `⚙️change-rail-load-group-assumed-axle-load`, `⚙️change-support-collision-assumed-force-y`, `⚙️change-support-collision-assumed-force-x`, `🧱️change-buckling-length-beam-stiffness-b`, `🧱️change-buckling-length-beam-stiffness-a`, `📍️change-seismic-slope-topographic-factor`, `✏️change-bridge-pier-cracked-inertia-ratio` (all under `🗿️artifacts/…/🧪️tests/🧫️fixtures/🧬️mutations/`). Source-boundary: 3 crates OVER budget (`semio-s-artifact-norm-en1993-mut-fastened-joints` 67 files, `…-mut-buckling-lengths` 34 files, `…-mut-fatigue-stresses` 34 files; limit 32 files). Scene golden files `🎬️scene/🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` are 0 bytes in every artifact and in the shared structural-scene.\n")
out.append("## Open items per lane\n")
for l in lanes_order:
    ops=[(n,v) for n,v in sorted(dec.items()) if v[0]=='open' and v[1]==l]
    if not ops: continue
    out.append(f"### {l} ({len(ops)} open)\n")
    for n,v in ops:
        rq=items[n]['reqline']
        out.append(f"- #{n} [{rq}] {v[2]}")
    out.append("")
out.append("## Done and stale items (evidence)\n")
for cls in ['done','stale']:
    ds=[(n,v) for n,v in sorted(dec.items()) if v[0]==cls]
    out.append(f"### {cls} ({len(ds)})\n")
    for n,v in ds:
        out.append(f"- #{n} [{v[1]}] {v[2]}")
    out.append("")
open(f"{T}/🔍️audits/fleet-1108-audit-requests.md",'w',encoding='utf-8').write('\n'.join(out)+'\n')
print('written', len(out))

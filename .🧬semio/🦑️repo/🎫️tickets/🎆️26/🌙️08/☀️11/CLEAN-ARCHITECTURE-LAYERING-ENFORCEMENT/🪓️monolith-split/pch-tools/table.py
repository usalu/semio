import sys,json,re,os,glob
G=sys.argv[1]
arts=[l.strip() for l in open(os.path.join(G,"pch-touched-artifacts.txt")) if l.strip()]
def parse(p):
    t=open(p).read()
    m=re.search(r'\{\n.*\n\}\n',t,re.S)
    return json.loads(m.group(0)) if m else None
def exit_of(summary,name):
    code=None
    for f in summary:
        if os.path.exists(f):
            for l in open(f):
                m=re.match(r'^(\S+) base=\S+ exit=(\d+) seconds=(\d+)',l)
                if m and m.group(1)==name: code=(int(m.group(2)),int(m.group(3)))
    return code
rows=[]
ok=0
for a in arts:
    name=os.path.basename(a)
    final=os.path.join(G,f"pch-check-final-{name}.log"); retry=os.path.join(G,f"pch-check-retry-{name}.log")
    src=retry if os.path.exists(retry) and parse(retry) else final
    d=parse(src)
    ex=exit_of([os.path.join(G,"pch-check-final.summary"),os.path.join(G,"pch-check-retry-1.summary"),os.path.join(G,"pch-check-retry-2.summary"),os.path.join(G,"pch-check-retry.summary"),os.path.join(G,"pch-check-retry3.summary")],name)
    if d is None: rows.append(f"| `{name}` | n/a | no JSON in {os.path.basename(src)} | | | |"); continue
    nat=d["layout"]["native"]; tst=d["layout"]["--test"]
    sib=len(d.get("siblings",[]))
    clean = nat["onlyCombined"]==0 and tst["onlyCombined"]==0 and not d["admission"] and not d["shared"]
    if clean: ok+=1
    rows.append(f"| `{name}` | `{d['base']}` | {ex[0] if ex else '?'} | {d['crates']} (+{sib} siblings) | {nat['base']} / {nat['combined']} / {nat['onlyCombined']} | {tst['base']} / {tst['combined']} / {tst['onlyCombined']} | {len(d['admission'])} / {len(d['shared'])} |")
print("| Artifact | Base | Exit | Carved crates | Native base / combined / extra | Test base / combined / extra | Hidden members / shared files |")
print("|---|---|---:|---|---|---|---|")
print("\n".join(rows))
print()
print(f"{ok} of {len(arts)} artifacts exit 0 with 0 extra diagnostics (lib and test), no hidden member and no plugin-owned shared file.")

other=[l.strip() for l in open(os.path.join(G,"pch-check-other.summary"))] if os.path.exists(os.path.join(G,"pch-check-other.summary")) else []
if other:
    print()
    print("Touched artifacts the generator did not carve (base HEAD, i.e. the tree before my dedupe edits):")
    print()
    print("| Artifact | Exit | Siblings | Native base / combined / extra | Test base / combined / extra | Hidden members | Shared plugin-owned files |")
    print("|---|---:|---|---|---|---|---|")
    for l in other:
        m=re.match(r'^(\S+) base=HEAD exit=(\d+)',l)
        if not m: continue
        name=m.group(1); d=parse(os.path.join(G,f"pch-check-other-{name}.log"))
        if d is None: print(f"| `{name}` | {m.group(2)} | no JSON | | | | |"); continue
        nat=d["layout"]["native"]; tst=d["layout"]["--test"]
        sh=", ".join(sorted({x["file"].split("/")[-3]+"/"+x["file"].split("/")[-2] for x in d["shared"]}))[:120]
        print(f"| `{name}` | {m.group(2)} | {', '.join(d.get('siblings',[]))} | {nat['base']} / {nat['combined']} / {nat['onlyCombined']} | {tst['base']} / {tst['combined']} / {tst['onlyCombined']} | {len(d['admission'])} | {len(d['shared'])} {sh} |")

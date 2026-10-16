import csv,re,sys,json,collections
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
def members(p):
    t=open(p,encoding='utf-8').read()
    m=re.search(r'\[workspace\](.*?)(?=\n\[[a-z])',t,re.S)
    body=m.group(1)
    mm=re.search(r'members\s*=\s*\[(.*?)\]',body,re.S)
    return set(x for x in re.findall(r'"([^"]+)"',mm.group(1)))
root=members("Cargo.toml"); fw=set("🧰️framework/"+x for x in members("🧰️framework/Cargo.toml"))
rows=list(csv.DictReader(open(F+"/🗑️generated/crate-census.csv")))
EXC=["semio-framework-os-renderer-wgpu","semio-framework-plugin","semio-framework-os-store-","semio-framework-os-kernel","semio-framework-os-io"]
def excluded(n):
    return n.startswith("semio-framework-os-renderer-wgpu") or n.startswith("semio-framework-plugin") or n.startswith("semio-framework-os-store-") or n.startswith("semio-framework-os-kernel") or n=="semio-framework-os-io" or n.startswith("semio-framework-os-io-")
byname={r['name']:r for r in rows}
res=collections.defaultdict(list)
for r in rows:
    if r['closureCompilable']!='true': continue
    d=r['manifest'][:-len('/Cargo.toml')]
    inroot=d in root; infw=d in fw
    ws='both' if inroot and infw else 'root' if inroot else 'fw' if infw else 'neither'
    r['ws']=ws; r['excl']=excluded(r['name'])
    res[(ws,r['excl'])].append(r)
for k,v in res.items(): print(k,len(v))
# topo
def topo(lst):
    names={r['name'] for r in lst}
    seen=set();out=[]
    def visit(n):
        if n in seen: return
        seen.add(n)
        for d in byname[n]['deps'].split(';'):
            if d in names: visit(d)
        out.append(n)
    for r in sorted(lst,key=lambda r:r['name']): visit(r['name'])
    return out
for k,v in res.items():
    ws,ex=k
    if ex: continue
    open(F+f"/🗑️generated/cp2-worklist-{ws}.txt","w").write("\n".join(topo(v))+"\n")
open(F+"/🗑️generated/cp2-excluded.txt","w").write("\n".join(r['name'] for k,v in res.items() if k[1] for r in v)+"\n")

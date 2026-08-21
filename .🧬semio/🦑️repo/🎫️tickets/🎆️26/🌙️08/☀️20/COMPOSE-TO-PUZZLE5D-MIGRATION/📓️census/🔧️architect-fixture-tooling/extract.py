import os, re, json, collections
M="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
leaves=[d for d in sorted(os.listdir(M)) if os.path.isfile(os.path.join(M,d,"🦠️mutation/🦀️component.rs"))]
out={}
tmpl=collections.Counter()
for d in leaves:
    mut=open(os.path.join(M,d,"🦠️mutation/🦀️component.rs")).read()
    dif=open(os.path.join(M,d,"🔺️diff/🦀️component.rs")).read()
    inv=open(os.path.join(M,d,"↩️inverse/🦀️component.rs")).read()
    sm=re.search(r"pub struct (\w+) \{(.*?)\n\}", mut, re.S)
    fields=re.findall(r"pub (\w+): ([^,\n]+),", sm.group(2))
    sem=re.search(r'SemanticDescriptor \{ verb: "([^"]+)", entity: "([^"]+)", kind: "([^"]+)", record: "([^"]+)" \}', mut).groups()
    body=dif.split("pub async fn diff",1)[1]
    e={"dir":d,"struct":sm.group(1),"fields":fields,"verb":sem[0],"entity":sem[1],"kind":sem[2],"record":sem[3]}
    m=re.search(r"base\.(\w+)\.iter", body) or re.search(r"base\.(\w+)\b", body)
    e["coll"]=m.group(1) if m else None
    m=re.search(r"ProgramDiff \{ (\w+): Some\((Program\w+Delta)", body)
    if m: e["diff_field"]=m.group(1); e["delta"]=m.group(2)
    m=re.search(r"(Program\w+PatchEntry)", body)
    if m: e["patch_entry"]=m.group(1)
    m=re.search(r"= (\w+Patch) \{", body)
    if m: e["patch_type"]=m.group(1)
    e["diff_body"]=body
    e["inv_body"]=inv.split("pub async fn inverse",1)[1]
    out[d]=e
json.dump(out, open("leaves.json","w"), ensure_ascii=False, indent=1)
# template check: normalize by removing identifiers
def norm(s, e):
    for k in ["struct","coll","entity","kind","record"]:
        v=e.get(k)
        if v: s=s.replace(v,"@")
    s=re.sub(r'"[^"]*"','"S"',s)
    s=re.sub(r'\bProgram\w+\b','@T',s)
    s=re.sub(r'\b\w+Patch\b','@P',s)
    s=re.sub(r'\s+',' ',s)
    return s
groups=collections.defaultdict(list)
for d,e in out.items():
    groups[norm(e["diff_body"],e)].append(d)
print("diff templates:", len(groups))
for k,v in sorted(groups.items(), key=lambda kv:-len(kv[1])):
    print(len(v), v[:3])

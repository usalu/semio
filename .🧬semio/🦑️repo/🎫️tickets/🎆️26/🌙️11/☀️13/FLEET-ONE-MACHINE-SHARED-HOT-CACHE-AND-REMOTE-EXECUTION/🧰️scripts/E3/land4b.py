import sys,json
P=sys.argv[1].rstrip('/')+'/'
def edit(path,pairs):
    s=open(path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:80],s.count(a))
        s=s.replace(a,b)
    open(path,'w').write(s)
edit(P+'🟦️.ts',[
('export const portableRowsV1=(rows:readonly ObservationRowV1[],root:string,policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]=>generationRowsV1(withoutLocationRowsV1(rows,root,policy),policy);',
 'export const portableRowsV1=(rows:readonly ObservationRowV1[],root:string,policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]=>generationRowsV1(installRowsV1(withoutLocationRowsV1(rows,root,policy),slashed(root).replace(/\\/+$/u,""),policy),policy);'),
('/** 📤️ The observations of a record as every machine can replay them: the `lstat` of the directories above `root` is dropped, and when the rows name exactly one dependency generation its id is replaced by the token. */',
 '/** 📤️ The observations of a record as every machine can replay them: the `lstat` of the directories above `root` is dropped, the probes of installed packages are replaced by one row naming the dependency generation ({@link installRowsV1}), and when the rows name exactly one dependency generation its id is replaced by the token. */'),
('function generationRowsV1(kept:readonly ObservationRowV1[],policy:PortableRowsPolicyV1,memo?:WeakMap<object,readonly string[]>):readonly ObservationRowV1[]{',
 '''/** 📦️ Replaces what the filesystem says about installed packages by the identity of the install: a row of `installKinds` about a path with a `node_modules` component outside a dependency generation directory (does `node_modules` exist here, does `ajv` exist above the checkout, what lists it) names what this machine has installed at this moment, so two machines with the same install wrote different rows. Those rows are dropped and, for every dependency generation the remaining rows name, one `exists` row says that the generation's `node_modules` exists: the content-addressed install identity (`base` is the root spelling of the rows). The rows below a generation directory stay, a generation holds the same content on every machine. A machine that lacks the generation replays the row with a name no directory has (see {@link localRowsV1}) and infers the project itself. */
function installRowsV1(kept:readonly ObservationRowV1[],base:string,policy:PortableRowsPolicyV1,memo?:WeakMap<object,readonly string[]>):readonly ObservationRowV1[]{
 const probe=(row:ObservationRowV1)=>policy.installKinds.includes(row.kind)&&slashed(row.path).split("/").includes(policy.probe)&&!generationPattern(policy).test(row.path);
 if(!kept.some(probe))return kept;
 const pattern=generationPattern(policy),ids=new Set<string>();
 for(const row of kept){let found=memo?.get(row);if(found===undefined){found=[...JSON.stringify([row.path,row.fact??null]).matchAll(pattern)].map(match=>match[2]!);memo?.set(row,found);}for(const id of found)ids.add(id);}
 const rows=kept.filter(row=>!probe(row)),named=new Set(rows.filter(row=>row.kind==="exists").map(row=>row.path));
 for(const id of [...ids].sort()){const path=`${base}/${policy.dependencyRoot}/${id}/${policy.probe}`;if(!named.has(path))rows.push({kind:"exists",path,fact:true});}
 return rows;
}
function generationRowsV1(kept:readonly ObservationRowV1[],policy:PortableRowsPolicyV1,memo?:WeakMap<object,readonly string[]>):readonly ObservationRowV1[]{'''),
(' return generationRowsV1(kept,policy,memo.ids);\n}',' return generationRowsV1(installRowsV1(kept,policy.rootToken,policy,memo.ids),policy,memo.ids);\n}')])
s=open(P+'🧬️schema/🔣️.json').read()
def rep(a,b):
    global s
    assert s.count(a)==1,a[:50];s=s.replace(a,b)
rep('        "locationKinds",\n','        "locationKinds",\n        "installKinds",\n') if False else None
d=json.loads(s)
pol=d['definitions']['Policy']
pol['required'].insert(pol['required'].index('locationKinds')+1,'installKinds')
props={}
for k,v in pol['properties'].items():
    props[k]=v
    if k=='locationKinds':props['installKinds']={"type":"array","items":{"type":"string"},"description":"The fact kinds whose rows about a path with a node_modules component outside a dependency generation directory name what a machine has installed; they are replaced by one row per dependency generation."}
pol['properties']=props
c=pol['const'];n={}
for k,v in c.items():
    n[k]=v
    if k=='locationKinds':n['installKinds']=["exists","directoryExists","fileExists","lstat","stat","readdir","readdirNames","directories"]
pol['const']=n
json.dump(d,open(P+'🧬️schema/🔣️.json','w'),indent=2,ensure_ascii=False);open(P+'🧬️schema/🔣️.json','a').write("\n")
# python oracle
edit(P+'🧪️tests/🐍️.py',[
('def neutral_way(root,keys):','''def install_rows(rows,base):
    pat=re.compile(r"(\\.cache/b/d/m/j[\\\\/])([0-9a-f]{16,64})(?=[\\\\/]|$)")
    def probe(r):return r["kind"] in P["installKinds"] and P["probe"] in re.split(r"[\\\\/]",r["path"]) and pat.search(r["path"]) is None
    if not any(probe(r) for r in rows):return rows
    ids=sorted({m.group(2) for r in rows for m in pat.finditer(json.dumps([r["path"],r.get("fact")],ensure_ascii=False))})
    kept=[r for r in rows if not probe(r)];named={r["path"] for r in kept if r["kind"]=="exists"}
    for ident in ids:
        path=base+"/"+P["dependencyRoot"]+"/"+ident+"/"+P["probe"]
        if path not in named:kept.append({"kind":"exists","path":path,"fact":True})
    return kept

def neutral_way(root,keys):'''),
('    return generation_tokens([unstamp(convert_row(r,way)) for r in kept])','    return generation_tokens(install_rows([unstamp(convert_row(r,way)) for r in kept],TOKEN))'),
('    portable=generation_tokens(kept)\n','    portable=generation_tokens(install_rows(kept,case["root"].replace("\\\\","/").rstrip("/")))\n')])
print("ok")

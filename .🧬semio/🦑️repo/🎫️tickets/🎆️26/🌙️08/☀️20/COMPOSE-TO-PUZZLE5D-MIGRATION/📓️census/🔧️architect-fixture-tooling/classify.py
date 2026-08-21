import os, re, json, collections
M="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
leaves=[d for d in sorted(os.listdir(M)) if os.path.isfile(os.path.join(M,d,"🦠️mutation/🦀️component.rs"))]
print(len(leaves))
shapes=collections.Counter()
info={}
for d in leaves:
    mut=open(os.path.join(M,d,"🦠️mutation/🦀️component.rs")).read()
    dif=open(os.path.join(M,d,"🔺️diff/🦀️component.rs")).read()
    inv=open(os.path.join(M,d,"↩️inverse/🦀️component.rs")).read()
    struct=re.search(r"pub struct (\w+) \{(.*?)\n\}", mut, re.S)
    fields=re.findall(r"pub (\w+): ([^,\n]+),", struct.group(2))
    sem=re.search(r'SemanticDescriptor \{ verb: "([^"]+)", entity: "([^"]+)", kind: "([^"]+)", record: "([^"]+)" \}', mut)
    # normalize diff body
    body=dif.split("pub async fn diff",1)[1]
    n=re.sub(r'"[^"]*"','S',body)
    n=re.sub(r'\b[A-Za-z_][A-Za-z0-9_]*\b', lambda m: m.group(0), n)
    # crude structural signature: sequence of key tokens
    sig=[]
    for tok in ["MutationOutcome::fatal","MutationOutcome::error","MutationOutcome::empty","absorb_messages","diff_patch","program_knowledge","program_benchmarks","knowledge_child_from_records","benchmarks_child_from_records","added:","removed:","patched:","reordered:","mutation.duplicate-id","mutation.target-missing","mutation.no-op","upsert","position","iter_mut","retain"]:
        if tok in body: sig.append(tok)
    sig=tuple(sig)
    shapes[sig]+=1
    info[d]={"struct":struct.group(1),"fields":fields,"sem":sem.groups() if sem else None,"sig":sig}
for s,c in shapes.most_common():
    print(c, s)
json.dump(info, open("info.json","w"), ensure_ascii=False, indent=1)

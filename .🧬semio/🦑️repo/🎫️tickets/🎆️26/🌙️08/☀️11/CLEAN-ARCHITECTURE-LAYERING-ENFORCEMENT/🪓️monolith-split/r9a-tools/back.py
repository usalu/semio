import json,re,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get("BI","0"))]
LAY=[("snapshot","schema::snapshot|io::(text|binary)::snapshot|snapshot::"),("authored","schema::authored"),("diff","schema::diff|io::(text|binary)::diff"),("inferences","schema::inferences|io::text::inferences|model_graph"),("mutation-leaf","schema::mutations::[a-z_]+(::(diff|inverse|mutation))?( |$)"),("aggregate","schema::mutations( |$)|io::(text|binary)::mutations|mutations::kit"),("io","io::"),("render","render"),("editor","editor|windows::|bim::"),("viewer","viewer"),("examples","examples")]
order={n:i for i,(n,_) in enumerate(LAY)}
def fam(n):
    for name,rx in LAY:
        if re.search(rx,n): return name
    return "other"
edges=[]
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    edges.append((m.group(1),m.group(2),bool(m.group(3)),m.group(4) or ""))
cnt=collections.Counter(); ex=collections.defaultdict(list)
for a,bb,t,w in edges:
    fa,fb=fam(a),fam(bb)
    if fa==fb or fa=="other" or fb=="other": continue
    if order[fa]<order[fb]:
        cnt[(fa,fb)]+=1; ex[(fa,fb)].append((a[-70:],bb[-70:],"T" if t else "L",w[:150]))
for k,v in cnt.most_common(): print(k,v)
for k in list(cnt)[:int(os.environ.get("N","12"))]:
    print("==",k)
    for e in ex[k][:int(os.environ.get("M","6"))]: print("   ",e)

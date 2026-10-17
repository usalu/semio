import sys,re
# summarize a plan log: crates, groups, and ring edges with leaf groups collapsed
log=open(sys.argv[1],encoding="utf-8").read().split("\n")
for l in log:
    if l.startswith("crate "):
        m=re.match(r"crate (\S+)\s+\S+\s+lib (\S+)\s+test (\S+)\s+deps",l)
        print("crate",m.group(1).replace("semio-s-artifact-stdio-",""),"lib",m.group(2),"test",m.group(3)) if m else print(l[:120])
    elif l.startswith("problem") or l.startswith("note: hoisted") or l.startswith("note: lifted") or l.startswith("note: kept"):
        print(l[:260])
    elif l.startswith("  group"):
        print(l[:220])
def collapse(s):
    return re.sub(r"\{(text|binary)::mutations::[a-z_]+\}", lambda m:"{"+m.group(1)+"::mutations::LEAF}", s)
seen=set()
for l in log:
    if l.startswith("    {") and "=>" in l:
        a,rest=l.strip().split(" => ",1)
        b,via=rest.split(" via ",1)
        key=(collapse(a),collapse(b),via.split(" @ ")[0])
        if collapse(a).endswith("LEAF}") or True:
            k=(collapse(a),collapse(b))
            if k in seen: continue
            seen.add(k)
            print("   ",collapse(a)[:90],"=>",collapse(b)[:90],"|",via.split(" @ ")[0][:150])

# 📕️ Wires the norm exhaustive cases to their case tables: every table case no `Examples` row declares becomes a `<kind>-<slug>` row in the mutate outline of the feature, in `VECTORS` of the Python adapter and in `ROWS` + `fixture_text` of the Rust adapter. Idempotent. usage: wire-norm.py [--check]
import glob,json,os,re,sys
ROOT="/Users/ueli/Documents/semio"
NORM=ROOT+"/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SKIP={("🌦️update-climate","✅apply")}
check="--check" in sys.argv
def fold(name): return name.replace("️","")
def slug(name): return re.sub(r"^[^a-z0-9]+","",fold(name))
def cells(line): return [cell.strip() for cell in line.strip().strip("|").split("|")]
total=0
for subset in sorted(glob.glob(NORM+"/*/🏅️standards/🔖️1/🪆️subsets/✳️any")):
    tests=glob.glob(subset+"/🧪️tests/*mutate-*-1")
    tables=sorted(glob.glob(subset+"/🧬️schema/🧬️mutations/*/🧪️tests/🔣️.cases.json"))
    if not tests or not tables: continue
    case=tests[0]; feature=case+"/🥒️.feature"; python=case+"/🐍️.py"; rust=case+"/🦀️.rs"
    if "const ROWS" not in open(rust,encoding="utf8").read(): print(os.path.basename(subset.rsplit("/🏅️standards",1)[0]),"skipped: its Rust subject registers only the production KINDS and reads vectors from the feature"); continue
    lines=open(feature,encoding="utf8").read().split("\n")
    start=next(i for i,l in enumerate(lines) if l.strip()=="Examples:")
    end=start+1
    while end+1<len(lines) and lines[end+1].strip().startswith("|"): end+=1
    header=cells(lines[start+1]); assert header==["id","dir","fixture"],(feature,header)
    declared={(fold(c[1]),fold(c[2])) for c in map(cells,lines[start+2:end+1])}
    ids={cells(l)[0] for l in lines[start+2:end+1]}
    width=max(len(cells(l)[0]) for l in lines[start+2:end+1])
    dirw=max(len(cells(l)[1]) for l in lines[start+2:end+1])
    rows=[]
    for table in tables:
        leaf=os.path.basename(os.path.dirname(os.path.dirname(table)))
        for name in json.load(open(table,encoding="utf8"))["cases"]:
            if (fold(leaf),fold(name)) in declared or (leaf,name) in SKIP: continue
            rows.append((slug(leaf)+"-"+slug(name),slug(leaf),leaf,name))
    rows=[r for r in rows if r[0] not in ids]
    py=open(python,encoding="utf8").read(); rs=open(rust,encoding="utf8").read()
    if not rows: print(os.path.basename(subset.rsplit("/🏅️standards",1)[0]),"already wired"); continue
    new_feature=lines[:end+1]+["      | %s | %s | %s |"%(rid.ljust(width),leaf.ljust(dirw),name) for rid,_k,leaf,name in rows]+lines[end+1:]
    vend=re.search(r"^VECTORS = \{\n(?:.*\n)*?\}\n",py,re.M); assert vend,python
    block=vend.group(0)
    new_block=block[:-2]+"".join('    "%s": ("%s", "%s", "%s"),\n'%(rid,kind,leaf,name) for rid,kind,leaf,name in rows)+"}\n"
    assert py.count(block)==1
    new_py=py.replace(block,new_block)
    rows_const=re.search(r'const ROWS: &\[&str\] = &\[\n((?:    ".*",\n)*)\];\n',rs); assert rows_const,rust
    new_rows="const ROWS: &[&str] = &[\n"+rows_const.group(1)+"".join('    "%s",\n'%r[0] for r in rows)+"];\n"
    arms="".join('        "%s" => (\n%s        ),\n'%(rid,"".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/%s/%s"),\n'%(leaf,name,leaf2) for leaf2 in ["📸️snapshot/⬅️before/🔣️.json","🦠️mutation/🔣️.json","📸️snapshot/➡️after/🔣️.json","🎯️outcome/🔣️.json"])) for rid,_k,leaf,name in rows)
    anchor="fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {\n    match kind {\n"
    assert rs.count(anchor)==1 and rs.count(rows_const.group(0))==1,rust
    new_rs=rs.replace(rows_const.group(0),new_rows).replace(anchor,anchor+arms)
    for r in rows:
        for leaf2 in ["📸️snapshot/⬅️before/🔣️.json","🦠️mutation/🔣️.json","📸️snapshot/➡️after/🔣️.json","🎯️outcome/🔣️.json"]:
            assert os.path.isfile(os.path.join(case,"..","🧫️fixtures","🧬️mutations",r[2],r[3],leaf2)),r
    print(os.path.basename(subset.rsplit("/🏅️standards",1)[0]),len(rows),"rows:",", ".join(r[0] for r in rows))
    total+=len(rows)
    if not check:
        open(feature,"w",encoding="utf8").write("\n".join(new_feature)); open(python,"w",encoding="utf8").write(new_py); open(rust,"w",encoding="utf8").write(new_rs)
print("total",total)

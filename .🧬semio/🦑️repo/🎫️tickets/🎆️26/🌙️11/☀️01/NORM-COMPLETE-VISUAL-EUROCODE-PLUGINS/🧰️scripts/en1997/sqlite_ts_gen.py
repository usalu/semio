import sys
sys.path.insert(0, ".")
exec(open("sqlite_gen.py", encoding="utf8").read().split("def sql_table")[0])
from spec import *
def floats(t):
    return [3 + i for i, (n, k) in enumerate(t.cols) if k == "n"]
p = Q + "/🟦️.ts"
s = open(p, encoding="utf8").read()
if "En1997SqliteAnchor" in s:
    sys.exit(0)
roots = [t for t in tabs if t.parent == "document"]
def tname(t): return "En1997Sqlite" + t.e.struct
imp = "Anchor,AnchorTest,EmbeddedWall,PileGroup,LabTest,LabPoint,LayerWater"
s = s.replace("RetainingWall,Slope,UpliftCase}from", "RetainingWall,Slope,UpliftCase," + imp + "}from", 1)
types = ""
for t in tabs:
    fl = [n for n, k in t.cols if k == "n"]
    omit = "|".join(f'"{camel(n)}"' for n in fl) + ("|" + f'"{camel(t.children[0].var)}"' if t.children else "")
    own = ";".join(f"{camel(n)}:Binary64" for n in fl) + (f";{camel(t.children[0].var)}:{tname(t.children[0])}[]" if t.children else "")
    types += f"export type {tname(t)}=Omit<{t.e.struct},{omit}>&{{{own}}};\n"
s = s.replace("/** 🌍️ Exact owned geotechnical entities", types + "/** 🌍️ Exact owned geotechnical entities", 1)
s = s.replace("upliftCases:En1997SqliteUplift[]};", "upliftCases:En1997SqliteUplift[];" + ";".join(f"{e.camel_coll}:{tname(next(x for x in tabs if x.e is e))}[]" for e in ENTITIES) + "};", 1)
consts = "".join(f"const {t.const}:readonly Ieee754Column[]=[{','.join(map(str, floats(t)))}].map(index=>({{index,width:64}}));\n" for t in tabs)
s = s.replace("function fields(", consts + "function fields(", 1)
names = lambda t: '["id",' + ",".join(f'"{camel(n)}"' for n, k in t.cols[1:]) + (f',"{camel(t.children[0].var)}"' if t.children else "") + "]"
s = s.replace('"slopes","upliftCases"]);for(const v of[s.layers,s.footings,s.piles,s.retainingWalls,s.slopes,s.upliftCases])collection(v);', '"slopes","upliftCases",' + ",".join(f'"{e.camel_coll}"' for e in ENTITIES) + ']);for(const v of[s.layers,s.footings,s.piles,s.retainingWalls,s.slopes,s.upliftCases,' + ",".join(f"s.{e.camel_coll}" for e in ENTITIES) + '])collection(v);', 1)
def val(k, ex):
    return f"unsigned({ex},4294967295)" if k == "u" else ex
writes = ""
for t in roots:
    cells = "1n,BigInt(n),v.id," + ",".join(val(k, f"v.{camel(n)}") for n, k in t.cols[1:])
    if t.children:
        c = t.children[0]
        ccells = "id,BigInt(k),a.id," + ",".join(val(k, f"a.{camel(n)}") for n, k in c.cols[1:])
        writes += f' for(let n=0;n<s.{t.e.camel_coll}.length;n++){{const v=s.{t.e.camel_coll}[n]!;fields(v,{names(t)});collection(v.{camel(c.var)});const id=await write(out,"{t.table}",[{cells}],{t.const},o);for(let k=0;k<v.{camel(c.var)}.length;k++){{const a=v.{camel(c.var)}[k]!;fields(a,{names(c)});await write(out,"{c.table}",[{ccells}],{c.const},o)}}}}\n'
    else:
        writes += f' for(let n=0;n<s.{t.e.camel_coll}.length;n++){{const v=s.{t.e.camel_coll}[n]!;fields(v,{names(t)});await write(out,"{t.table}",[{cells}],{t.const},o)}}\n'
s = s.replace(" return out.finish();", writes + " return out.finish();", 1)
# readers: table order t[9..15]
reads = ""
rd = lambda k, n, i, v, C: (f"{camel(n)}:artifactSqliteText({v},{i})" if k == "s" else (f"{camel(n)}:readUnsigned({v},{i},4294967295n)" if k == "u" else f"{camel(n)}:readBinary64({v},{i},{C})"))
ti = 9
for t in roots:
    cnt = 4 + len(t.cols) - 1
    flds = "id:artifactSqliteText(v,3)," + ",".join(rd(k, n, 4 + i, "v", t.const) for i, (n, k) in enumerate(t.cols[1:]))
    if t.children:
        c = t.children[0]
        ccnt = 4 + len(c.cols) - 1
        cf = "id:artifactSqliteText(a,3)," + ",".join(rd(k, n, 4 + i, "a", c.const) for i, (n, k) in enumerate(c.cols[1:]))
        reads += f' const {t.e.camel_coll}Rows=await entities(t[{ti}]!,{cnt},{t.const},o),{camel(c.var)}By{t.e.struct}=await groups(await entities(t[{ti+1}]!,{ccnt},{c.const},o),{t.e.camel_coll}Rows,o);\n'
        reads += f' for(const v of await ordered({t.e.camel_coll}Rows.values(),1n,o)){{const {camel(c.var)}:{tname(c)}[]=[];for(const a of await ordered({camel(c.var)}By{t.e.struct}.get(v.rowid)??[],v.rowid,o)){{await checkpoint(o,{camel(c.var)}.length);{camel(c.var)}.push({{{cf}}})}}{t.e.camel_coll}.push({{{flds},{camel(c.var)}}})}}\n'
        ti += 2
    else:
        reads += f' for(const v of await ordered((await entities(t[{ti}]!,{cnt},{t.const},o)).values(),1n,o)){{await checkpoint(o,{t.e.camel_coll}.length);{t.e.camel_coll}.push({{{flds}}})}}\n'
        ti += 1
s = s.replace("upliftCases:En1997SqliteUplift[]=[];\n", "upliftCases:En1997SqliteUplift[]=[]," + ",".join(f"{e.camel_coll}:{tname(next(x for x in tabs if x.e is e))}[]=[]" for e in ENTITIES) + ";\n", 1)
s = s.replace(" await artifactSqliteCheckpoint(o,\"reconstructSnapshot\",1,1);return{", reads + " await artifactSqliteCheckpoint(o,\"reconstructSnapshot\",1,1);return{", 1)
s = s.replace("slopes,upliftCases};\n}", "slopes,upliftCases," + ",".join(e.camel_coll for e in ENTITIES) + "};\n}", 1)
open(p, "w", encoding="utf8").write(s)
print("ts ok")

"""Applies the EN 1996 wave 2 entities to the SQLite TypeScript codec. Idempotent."""
import os, re
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))
SQ = f"{IO}/🪶️sqlite/📸️snapshot"
CONSTN = {"TieKind": "TIE_KIND", "BarKind": "BAR_KIND", "FrostClass": "FROST_CLASS"}
p = f"{SQ}/🟦️.ts"
s = rd(p)
if "PIER_FLOATS" not in s:
    s = s.replace("WallOpening,WallLoadCase,ConcentratedLoad}from", "WallOpening,WallLoadCase,ConcentratedLoad," + ",".join(r["struct"] for r in ROWS) + "}from", 1)
    s = s.replace('const EXPOSURE=["Mx1","Mx2","Mx3","Mx4","Mx5"]as const;\n', 'const EXPOSURE=["Mx1","Mx2","Mx3","Mx4","Mx5"]as const;\n' + "".join(f"const {CONSTN[n]}=[" + ",".join(f'"{v}"' for v in e["variants"]) + "]as const;\n" for n, e in ENUMS.items()), 1)
    s = s.replace('export type En1996SqliteOpening=Omit<WallOpening,"widthM"|"heightM"|"sillHeightM">&{widthM:Binary64;heightM:Binary64;sillHeightM:Binary64};', 'export type En1996SqliteOpening=Omit<WallOpening,"widthM"|"heightM"|"sillHeightM"|"positionM">&{widthM:Binary64;heightM:Binary64;sillHeightM:Binary64;positionM:Binary64};\n' + "\n".join(
        "export type En1996Sqlite" + r["struct"] + "=Omit<" + r["struct"] + "," + "|".join(f'"{f["camel"]}"' for f in r["fields"]) + ">&{" + ";".join(f"{f['camel']}:" + ("Binary64" if f["kind"] == "f64" else f"typeof {CONSTN[f['kind']]}[number]") for f in r["fields"]) + "};" for r in ROWS), 1)
    s = s.replace('export type En1996SqliteConcentratedLoad=Omit<ConcentratedLoad,"forceN"|"bearingAreaM2"|"bearingLengthM">&{forceN:Binary64;bearingAreaM2:Binary64;bearingLengthM:Binary64};', 'export type En1996SqliteConcentratedLoad=Omit<ConcentratedLoad,"forceN"|"bearingAreaM2"|"bearingLengthM"|"distanceA1M"|"positionM">&{forceN:Binary64;bearingAreaM2:Binary64;bearingLengthM:Binary64;distanceA1M:Binary64;positionM:Binary64};', 1)
    s = s.replace('|"phiInfinity"|"openings"|"loadCases">', '|"phiInfinity"|"openings"|"loadCases"|' + "|".join(f'"{r["coll"]}"' for r in ROWS) + "|" + "|".join(f'"{x["camel"]}"' for x in SCALARS) + ">", 1)
    s = s.replace("openings:En1996SqliteOpening[];loadCases:En1996SqliteLoadCase[];", "openings:En1996SqliteOpening[];loadCases:En1996SqliteLoadCase[];" + "".join(f"{r['coll']}:En1996Sqlite{r['struct']}[];" for r in ROWS) + "".join(f"{x['camel']}:" + ("Binary64" if x["kind"] == "f64" else f"typeof {CONSTN[x['kind']]}[number]") + ";" for x in SCALARS), 1)
    s = s.replace("30,31,32].map", "30,31,32,35,36,37,38,39,40,41,42,43,44].map", 1)
    s = s.replace("const OPENING_FLOATS:readonly Ieee754Column[]=[4,5,6]", "const OPENING_FLOATS:readonly Ieee754Column[]=[4,5,6,7]", 1)
    s = s.replace("const CONCENTRATED_FLOATS:readonly Ieee754Column[]=[4,5,6]", "const CONCENTRATED_FLOATS:readonly Ieee754Column[]=[4,5,6,7,8]", 1)
    def fidx(r): return [4 + i for i, f in enumerate(r["fields"]) if f["kind"] == "f64"]
    s = s.replace("type Entities=Map<bigint,SqliteRow>;", "".join(f"const {r['key'].upper()}_FLOATS:readonly Ieee754Column[]=[{','.join(map(str, fidx(r)))}].map(index=>({{index,width:64}}));\n" for r in ROWS) + "type Entities=Map<bigint,SqliteRow>;", 1)
    sc = ",".join(f"choice(w.{x['camel']},{CONSTN[x['kind']]})" if x["kind"] != "f64" else f"w.{x['camel']}" for x in SCALARS)
    s = s.replace("flag(w.isBasement)],WALL_FLOATS,options);", "flag(w.isBasement)," + sc + "],WALL_FLOATS,options);", 1)
    s = s.replace("o.sillHeightM],OPENING_FLOATS", "o.sillHeightM,o.positionM],OPENING_FLOATS", 1)
    s = s.replace("l.bearingLengthM],CONCENTRATED_FLOATS", "l.bearingLengthM,l.distanceA1M,l.positionM],CONCENTRATED_FLOATS", 1)
    loops = ""
    for r in ROWS:
        cells = ",".join(f"x.{f['camel']}" if f["kind"] == "f64" else f"choice(x.{f['camel']},{CONSTN[f['kind']]})" for f in r["fields"])
        loops += f'  for(let position=0;position<w.{r["coll"]}.length;position++){{const x=w.{r["coll"]}[position]!;await write(out,"en1996_{r["key"]}",[id,BigInt(position),x.id,{cells}],{r["key"].upper()}_FLOATS,options)}}\n'
    m = re.search(r"  for\(let position=0;position<w\.openings\.length;position\+\+\)[^\n]*\n", s)
    s = s[: m.end()] + loops + s[m.end():]
    s = s.replace("entities(tables[1]!,34,WALL_FLOATS", "entities(tables[1]!,45,WALL_FLOATS", 1)
    s = s.replace("entities(tables[2]!,7,OPENING_FLOATS", "entities(tables[2]!,8,OPENING_FLOATS", 1)
    s = s.replace("entities(tables[4]!,7,CONCENTRATED_FLOATS", "entities(tables[4]!,9,CONCENTRATED_FLOATS", 1)
    gr = ""
    for i, r in enumerate(ROWS):
        gr += f"const g_{r['coll']}=await groups(await entities(tables[{5 + i}]!,{4 + len(r['fields'])},{r['key'].upper()}_FLOATS,options),wallRows,options);"
    s = s.replace("cases=await groups(caseRows,wallRows,options);", "cases=await groups(caseRows,wallRows,options);" + gr, 1)
    s = s.replace("sillHeightM:readBinary64(o,6,OPENING_FLOATS)", "sillHeightM:readBinary64(o,6,OPENING_FLOATS),positionM:readBinary64(o,7,OPENING_FLOATS)", 1)
    s = s.replace("bearingLengthM:readBinary64(l,6,CONCENTRATED_FLOATS)", "bearingLengthM:readBinary64(l,6,CONCENTRATED_FLOATS),distanceA1M:readBinary64(l,7,CONCENTRATED_FLOATS),positionM:readBinary64(l,8,CONCENTRATED_FLOATS)", 1)
    reads = ""
    for r in ROWS:
        fl = ",".join(f"{f['camel']}:" + (f"readBinary64(x,{4 + i},{r['key'].upper()}_FLOATS)" if f["kind"] == "f64" else f"choice(artifactSqliteText(x,{4 + i}),{CONSTN[f['kind']]})") for i, f in enumerate(r["fields"]))
        reads += f"  const v_{r['coll']}:En1996Sqlite{r['struct']}[]=[];for(const x of await ordered(g_{r['coll']}.get(r.rowid)??[],r.rowid,options)){{await checkpoint(options,v_{r['coll']}.length,0);v_{r['coll']}.push({{id:artifactSqliteText(x,3),{fl}}})}}\n"
    m = re.search(r"  walls\.push\(", s)
    s = s[: m.start()] + reads + s[m.start():]
    fl = [x for x in SCALARS if x["kind"] == "f64"]
    sc = ",".join(f"{x['camel']}:" + (f"readBinary64(r,{35 + [y['name'] for y in fl].index(x['name'])},WALL_FLOATS)" if x["kind"] == "f64" else "choice(artifactSqliteText(r,34),FROST_CLASS)") for x in SCALARS)
    s = s.replace("isBasement:artifactSqliteBoolean(r,33),loadCases}", "isBasement:artifactSqliteBoolean(r,33),loadCases," + ",".join(f"{r['coll']}:v_{r['coll']}" for r in ROWS) + "," + sc + "}", 1)
    wr(p, s)
print("sqlite ts done")

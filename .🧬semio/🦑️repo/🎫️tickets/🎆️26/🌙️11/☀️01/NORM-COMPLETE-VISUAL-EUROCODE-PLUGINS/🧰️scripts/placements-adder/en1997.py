import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

A = Art("🌍️en1997", "En1997", "layer_waters", "layerWaters", "LayerWater")


def opaque_root_ts(path):
    def go(t):
        if "placements:" in t:
            return t
        t = clone_line(t, r"^\s+layerWaters: ", A, fix=emoji_fix)
        def p(m):
            line = m.group(0)
            s, e = entry_span(line, "layerWaters")
            return line[:e] + ", " + A.rn(line[s:e]) + line[e:]
        return re.sub(r"^export const parseEn1997Artifact: .*$", p, t, flags=re.M)
    edit(path, go)


def sqlite_layers():
    q = "/🚪️io/🪶️sqlite/📸️snapshot"
    TB = "en1997_placement"
    sqltxt = ("CREATE TABLE en1997_placement (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, x REAL, y REAL, z REAL, yaw REAL,\n x_ieee754_bits INTEGER NOT NULL, x_ieee754_class TEXT NOT NULL,\n y_ieee754_bits INTEGER NOT NULL, y_ieee754_class TEXT NOT NULL,\n z_ieee754_bits INTEGER NOT NULL, z_ieee754_class TEXT NOT NULL,\n yaw_ieee754_bits INTEGER NOT NULL, yaw_ieee754_class TEXT NOT NULL\n);\n")
    edit(A.path(q + "/🗄️.sql"), lambda t: t if TB in t else t.rstrip("\n") + "\n" + sqltxt)

    def rs(t):
        if "PLACEMENT" in t:
            return t
        t = sub1(t, r"LabPoint, LayerWater\};", "LabPoint, LayerWater, Placement};")
        t = sub1(t, r"(const LAYERWATER[^\n]*\n)", r"\1const PLACEMENT:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\n")
        t = sub1(t, r"(  for\(n,v\)in self\.layer_waters\.iter\(\)\.enumerate\(\)\{[^\n]*\n)", lambda m: m.group(1) + '  for(n,q)in self.placements.iter().enumerate(){p.insert_float("en1997_placement",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&q.id),Cell::Real(q.x),Cell::Real(q.y),Cell::Real(q.z),Cell::Real(q.yaw)],PLACEMENT)?;}\n')
        t = sub1(t, r"self\.layer_waters\.len\(\)\]\{admit_rows", "self.layer_waters.len(),self.placements.len()]{admit_rows")
        t = sub1(t, r"(for v in &self\.layer_waters\{[^\n]*?\})b\.finish\(\)", lambda m: m.group(1) + "for v in &self.placements{b.add(8192)?;b.repeated(v.id.len(),24)?;}b.finish()")
        t = sub1(t, r"(\n  let result=Self\{)", lambda m: '\n  let mut placements=Vec::new();for v in ordered(entities(d,"en1997_placement",8,PLACEMENT,c)?.into_values(),1,c)?{checkpoint(c,placements.len(),0)?;placements.push(Placement{id:text(v,3,c)?,x:v.real(4)?,y:v.real(5)?,z:v.real(6)?,yaw:v.real(7)?});}' + m.group(1))
        t = sub1(t, r"layer_waters\};c\.checkpoint", "layer_waters,placements};c.checkpoint")
        t = sub1(t, r"SQLITE_SCHEMA,16,96", "SQLITE_SCHEMA,17,96")
        return t
    edit(A.path(q + "/🦀️.rs"), rs)

    def ts(t):
        if "PLACEMENT" in t:
            return t
        t = sub1(t, r"LabPoint,LayerWater\}from", "LabPoint,LayerWater,Placement}from")
        t = sub1(t, r"(export type En1997SqliteLayerWater=[^\n]*\n)", lambda m: m.group(1) + 'export type En1997SqlitePlacement=Omit<Placement,"x"|"y"|"z"|"yaw">&{x:Binary64;y:Binary64;z:Binary64;yaw:Binary64};\n')
        t = sub1(t, r"layerWaters:En1997SqliteLayerWater\[\]\};", "layerWaters:En1997SqliteLayerWater[];placements:En1997SqlitePlacement[]};")
        t = sub1(t, r"(const LAYERWATER[^\n]*\n)", r"\1const PLACEMENT:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r'"layerWaters"\]\);for\(const v of\[(.*?)s\.layerWaters\]\)', lambda m: '"layerWaters","placements"]);for(const v of[' + m.group(1) + 's.layerWaters,s.placements])')
        t = sub1(t, r"(\n for\(let n=0;n<s\.layerWaters\.length;n\+\+\)\{[^\n]*\n)", lambda m: m.group(1) + ' for(let n=0;n<s.placements.length;n++){const v=s.placements[n]!;fields(v,["id","x","y","z","yaw"]);await write(out,"en1997_placement",[1n,BigInt(n),v.id,v.x,v.y,v.z,v.yaw],PLACEMENT,o)}\n')
        t = sub1(t, r"layerWaters:En1997SqliteLayerWater\[\]=\[\];", "layerWaters:En1997SqliteLayerWater[]=[],placements:En1997SqlitePlacement[]=[];")
        t = sub1(t, r"(\n await artifactSqliteCheckpoint\(o,\"reconstructSnapshot\",1,1\);)", lambda m: '\n for(const v of await ordered((await entities(t[16]!,8,PLACEMENT,o)).values(),1n,o)){await checkpoint(o,placements.length);placements.push({id:artifactSqliteText(v,3),x:readBinary64(v,4,PLACEMENT),y:readBinary64(v,5,PLACEMENT),z:readBinary64(v,6,PLACEMENT),yaw:readBinary64(v,7,PLACEMENT)})}' + m.group(1))
        t = sub1(t, r"layerWaters\};", "layerWaters,placements};")
        return t
    edit(A.path(q + "/🟦️.ts"), ts)

    def adm(t):
        if "en1997_placement" in t:
            return t
        t = sub1(t, r"(\n)(Ok\(\(\)\)\n\})", lambda m: m.group(1) + ' for(index,v)in n::list(n::field(root,18)?)?.iter().enumerate(){n::entity(out,"en1997_placement",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT)?;}\n' + m.group(2))
        t = sub1(t, r"for id in 7\.\.=17\{", "for id in 7..=18{")
        t = sub1(t, r"SQLITE_SCHEMA,16,96", "SQLITE_SCHEMA,17,96")
        return t.replace("/// 🛂️ 16 authored", "/// 🛂️ 17 authored")
    edit(A.path(q + "/🛂️admission/🦀️.rs"), adm)

    def rt(t):
        t = re.sub(r"\b(\w+)\.layer_waters\.clear\(\);(?!\s*\1\.placements)", lambda m: m.group(0) + m.group(1) + ".placements.clear();", t)
        return t.replace("max_tables:16", "max_tables:17").replace('("tables",16)', '("tables",17)')
    edit(A.path(q + "/🧪️tests/🦀️.rs"), rt)

    def tt(t):
        if "placements" in t:
            return t
        t = sub1(t, r"function owned\(\):En1997SqliteSnapshot\{return\{\.\.\.f,", "function owned():En1997SqliteSnapshot{return{...f,placements:f.placements.map(v=>({...v,x:binary64(v.x),y:binary64(v.y),z:binary64(v.z),yaw:binary64(v.yaw)})),")
        t = sub1(t, r"layerWaters:\[\]", "layerWaters:[],placements:[]", count=1)
        t = sub1(t, r"source\.layerWaters=\[\];", "source.layerWaters=[];source.placements=[];")
        return t.replace("maxTables:16", "maxTables:17").replace("toBe(16)", "toBe(17)")
    edit(A.path(q + "/🧪️tests/🟦️.ts"), tt)
    sqlite_extent(A, A.path(q + "/🗄️.sql"), A.path(q + "/🧪️tests/🧫️fixtures/🎛️semantic.json"), A.path(q + "/🧪️tests/🟦️.ts"), None, 16, TB)


if __name__ == "__main__":
    opaque_root_ts(A.path("/🧬️schema/🟦️.ts"))
    core(A, root_ts=("/📸️snapshot/🟦️.ts",))
    multiline_literal_sweep(A)
    sqlite_layers()

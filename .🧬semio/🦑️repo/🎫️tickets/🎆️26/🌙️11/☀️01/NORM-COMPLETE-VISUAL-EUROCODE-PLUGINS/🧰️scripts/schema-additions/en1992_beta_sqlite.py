"""en1992 `punching.betaMethod|momentC1|momentC2` SQLite layer: DDL (enum CHECK + 2 floats), rust write/read + token fns, ts write/read, native cells roles and widths, tests (rs word oracle, ts counts), contract fixture (tableWidths, snapshot punching). Idempotent."""
import json, re
Q = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/"
rd = lambda p: open(Q + p, encoding="utf8").read()


def sub(path, old, new, count=1):
    text = rd(path)
    if new in text:
        return print("present", path, old[:40])
    assert text.count(old) == count, (path, old[:70], text.count(old))
    open(Q + path, "w", encoding="utf8").write(text.replace(old, new))


sub("🗄️.sql", "asw REAL,\n column_width_ieee754_bits INTEGER NOT NULL, column_width_ieee754_class TEXT NOT NULL,\n column_depth_ieee754_bits INTEGER NOT NULL, column_depth_ieee754_class TEXT NOT NULL,\n asw_ieee754_bits INTEGER NOT NULL, asw_ieee754_class TEXT NOT NULL\n",
    "asw REAL, beta_method TEXT NOT NULL CHECK(beta_method IN ('Simplified','UnbalancedMoments')), moment_c1 REAL, moment_c2 REAL,\n column_width_ieee754_bits INTEGER NOT NULL, column_width_ieee754_class TEXT NOT NULL,\n column_depth_ieee754_bits INTEGER NOT NULL, column_depth_ieee754_class TEXT NOT NULL,\n asw_ieee754_bits INTEGER NOT NULL, asw_ieee754_class TEXT NOT NULL,\n moment_c1_ieee754_bits INTEGER NOT NULL, moment_c1_ieee754_class TEXT NOT NULL,\n moment_c2_ieee754_bits INTEGER NOT NULL, moment_c2_ieee754_class TEXT NOT NULL\n")
r = "🦀️.rs"
sub(r, "PunchingSpec, PrestressSpec,", "PunchingSpec, PunchingBetaMethod, PrestressSpec,")
sub(r, "fn cement_class_text(", 'fn punching_beta_method_text(a:PunchingBetaMethod)->&\'static str{match a{PunchingBetaMethod::Simplified=>"Simplified",PunchingBetaMethod::UnbalancedMoments=>"UnbalancedMoments",}}\nfn read_punching_beta_method(a:&str)->Result<PunchingBetaMethod, ValueError>{match a{"Simplified"=>Ok(PunchingBetaMethod::Simplified),"UnbalancedMoments"=>Ok(PunchingBetaMethod::UnbalancedMoments),_=>Err(invalid("EN1992 punching beta method differs"))}}\nfn cement_class_text(')
sub(r, "const PUNCHING:&[FloatColumn]=&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(4)];", "const PUNCHING:&[FloatColumn]=&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(4),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];")
sub(r, "Cell::Text(column_position_text(a.column_position)),Cell::Real(a.asw)]", "Cell::Text(column_position_text(a.column_position)),Cell::Real(a.asw),Cell::Text(punching_beta_method_text(a.beta_method)),Cell::Real(a.moment_c1),Cell::Real(a.moment_c2)]")
sub(r, 'entities(d,"en1992_punching",5,PUNCHING,c)', 'entities(d,"en1992_punching",8,PUNCHING,c)')
sub(r, "asw:a.real(4)?})", "asw:a.real(4)?,beta_method:read_punching_beta_method(text(a,5,c)?)?,moment_c1:a.real(6)?,moment_c2:a.real(7)?})")
t = "🟦️.ts"
sub(t, "BondCondition,BarPosition,CementClass,ColumnPosition,", "BondCondition,BarPosition,CementClass,ColumnPosition,PunchingBetaMethod,")
sub(t, "columnPosition:ColumnPosition;asw:Binary64};", "columnPosition:ColumnPosition;asw:Binary64;betaMethod:PunchingBetaMethod;momentC1:Binary64;momentC2:Binary64};")
sub(t, 'const COLUMN_POSITIONS:readonly ColumnPosition[]=["Interior","Edge","Corner","WallEnd"];', 'const COLUMN_POSITIONS:readonly ColumnPosition[]=["Interior","Edge","Corner","WallEnd"];\nconst BETA_METHODS:readonly PunchingBetaMethod[]=["Simplified","UnbalancedMoments"];')
sub(t, "const PUNCHING:readonly Ieee754Column[]=[1,2,4].map", "const PUNCHING:readonly Ieee754Column[]=[1,2,4,6,7].map")
sub(t, 'fields(a,["columnWidth","columnDepth","columnPosition","asw"]);await write(out,"en1992_punching",[a.columnWidth,a.columnDepth,choice(a.columnPosition,COLUMN_POSITIONS),a.asw],PUNCHING,o,id)', 'fields(a,["columnWidth","columnDepth","columnPosition","asw","betaMethod","momentC1","momentC2"]);await write(out,"en1992_punching",[a.columnWidth,a.columnDepth,choice(a.columnPosition,COLUMN_POSITIONS),a.asw,choice(a.betaMethod,BETA_METHODS),a.momentC1,a.momentC2],PUNCHING,o,id)')
sub(t, "await entities(t[7]!,5,PUNCHING,o)", "await entities(t[7]!,8,PUNCHING,o)")
sub(t, "asw:readBinary64(p,4,PUNCHING)}", "asw:readBinary64(p,4,PUNCHING),betaMethod:choice(artifactSqliteText(p,5),BETA_METHODS),momentC1:readBinary64(p,6,PUNCHING),momentC2:readBinary64(p,7,PUNCHING)}")
c = "📏️cells/🦀️.rs"
sub(c, '("en1992_punching",11)', '("en1992_punching",18)')
sub(c, 'optional(field(value,17)?,4,"en1992_punching",&[(0,Float),(1,Float),(2,Text),(3,Float)]', 'optional(field(value,17)?,7,"en1992_punching",&[(0,Float),(1,Float),(2,Text),(3,Float),(4,Text),(5,Float),(6,Float)]')
tr = "🧪️tests/🦀️.rs"
sub(tr, "a.column_depth=w;a.asw=w;}", "a.column_depth=w;a.asw=w;a.moment_c1=w;a.moment_c2=w;}")
sub(tr, "if(count!==60)throw Error('all60scalars')", "if(count!==62)throw Error('all62scalars')")
sub(tr, "all_sixty_fields_have", "all_sixty_two_fields_have")
tt = "🧪️tests/🟦️.ts"
sub(tt, "every sixty float fields", "every sixty-two float fields")
sub(tt, "expect(count).toBe(60)", "expect(count).toBe(62)")
fx = "🧪️tests/🧫️fixtures/🔣️.json"
text = rd(fx)
data = json.loads(text)
data["semanticCells"]["tableWidths"] = [[n, 18 if n == "en1992_punching" else w] for n, w in data["semanticCells"]["tableWidths"]]
p = data["snapshot"]["members"][0]["punching"]
if "betaMethod" not in p:
    p.update({"betaMethod": "UnbalancedMoments", "momentC1": 0.5, "momentC2": 0.5})
    open(Q + fx, "w", encoding="utf8").write(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    print("fixture rewritten (indent 2)")

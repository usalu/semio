"""en1998 v9 step 10: SQLite snapshot for the retyped fields: member kind CHECK text, wall storey id text, liquefiable layer 0/1. python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
Q = ANY / "🚪️io/🪶️sqlite/📸️snapshot"
r = Q / "🦀️.rs"
sub(r, "use crate::{DeGroundCombo, En1998Site,", "use crate::{DeGroundCombo, MemberKind, En1998Site,")
sub(r, "FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14),FloatColumn::Binary64(15),FloatColumn::Binary64(16)];\nconst BUILDING", "FloatColumn::Binary64(12),FloatColumn::Binary64(14),FloatColumn::Binary64(15),FloatColumn::Binary64(16)];\nconst BUILDING")
sub(r, "FloatColumn::Binary64(16),FloatColumn::Binary64(17),FloatColumn::Binary64(18)];\nconst BRIDGE", "FloatColumn::Binary64(16),FloatColumn::Binary64(17)];\nconst BRIDGE")
sub(r, 'fn boolean(row:FloatRow', 'fn read_member_kind(value:&str)->Result<MemberKind, ValueError>{match value{"beam"=>Ok(MemberKind::Beam),"column"=>Ok(MemberKind::Column),"wall"=>Ok(MemberKind::Wall),"brace"=>Ok(MemberKind::Brace),_=>Err(invalid("EN1998 unsupported member kind"))}}\nfn boolean(row:FloatRow')
sub(r, "Cell::Real(s.liquefiable_layer)", "Cell::Integer(i64::from(s.liquefiable_layer))")
sub(r, "liquefiable_layer:s.real(13)?", "liquefiable_layer:boolean(*s,13)?")
sub(r, "Cell::Text(&m.role)", "Cell::Text(m.role.as_str())")
sub(r, "Cell::Real(m.wall_storey_id)", "Cell::Text(&m.wall_storey_id)")
sub(r, "role:reconstruct_text(control,x.text(5)?)?", "role:read_member_kind(x.text(5)?)?")
sub(r, "wall_storey_id:x.real(18)?", "wall_storey_id:reconstruct_text(control,x.text(18)?)?")
sub(r, "for text in [&m.id,&m.material,&m.role]", "for text in [&m.id,&m.material,&m.wall_storey_id]")
q = Q / "🗄️.sql"
sub(q, "liquefiable_layer REAL,", "liquefiable_layer INTEGER NOT NULL CHECK(liquefiable_layer IN (0,1)),")
sub(q, "liquefiable_layer_ieee754_bits INTEGER NOT NULL,liquefiable_layer_ieee754_class TEXT NOT NULL,", "")
sub(q, "logical_id TEXT NOT NULL,material TEXT NOT NULL,role TEXT NOT NULL,detailing_compatible_with_q", "logical_id TEXT NOT NULL,material TEXT NOT NULL,role TEXT NOT NULL CHECK(role IN ('beam','column','wall','brace')),detailing_compatible_with_q")
sub(q, "wall_storey_id REAL,", "wall_storey_id TEXT NOT NULL,")
sub(q, ",wall_storey_id_ieee754_bits INTEGER NOT NULL,wall_storey_id_ieee754_class TEXT NOT NULL);", ");")
t = Q / "🟦️.ts"
sub(t, "const SITE:readonly Ieee754Column[]=[2,3,8,9,10,11,12,13,14,15,16]", "const SITE:readonly Ieee754Column[]=[2,3,8,9,10,11,12,14,15,16]")
sub(t, "const MEMBER:readonly Ieee754Column[]=[7,8,9,10,12,13,14,15,16,17,18]", "const MEMBER:readonly Ieee754Column[]=[7,8,9,10,12,13,14,15,16,17]")
sub(t, "site.softClayPi,site.liquefiableLayer,site.alluviumThicknessM", "site.softClayPi,site.liquefiableLayer?1n:0n,site.alluviumThicknessM")
sub(t, "count(x.steelSectionClass),m.fastenerDiameterMm,m.timberThicknessMm,m.wallThicknessM,m.wallEffectiveHeightM,m.wallLengthM,m.wallDirectionRad,m.wallStoreyId],MEMBER", "count(x.steelSectionClass),x.fastenerDiameterMm,x.timberThicknessMm,x.wallThicknessM,x.wallEffectiveHeightM,x.wallLengthM,x.wallDirectionRad,x.wallStoreyId],MEMBER")
sub(t, "liquefiableLayer:readBinary64(s,13,SITE)", "liquefiableLayer:boolean(s,13)")
sub(t, "wallStoreyId:readBinary64(x,18,MEMBER)", "wallStoreyId:artifactSqliteText(x,18)")
sub(t, "role:artifactSqliteText(x,5),", "role:kind(artifactSqliteText(x,5)),")
sub(t, "function boolean(row:SqliteRow", 'function kind(value:string):En1998Member["role"]{if(value!=="beam"&&value!=="column"&&value!=="wall"&&value!=="brace")throw Error("EN1998 unsupported member kind");return value}\nfunction boolean(row:SqliteRow')
rt = Q / "🧪️tests/🦀️.rs"
sub(rt, "s.site.liquefiable_layer=v;", "")
sub(rt, "x.wall_storey_id=v;", "")
tt = Q / "🧪️tests/🟦️.ts"
sub(tt, '"softClayPi","liquefiableLayer","alluviumThicknessM"', '"softClayPi","alluviumThicknessM"')
sub(tt, 'floats(x,["wallStoreyId","wallDirectionRad"', 'floats(x,["wallDirectionRad"')
flush()

import json, pathlib, re, shutil
A = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any')
Q = A / '🚪️io/🪶️sqlite/📸️snapshot'
def rd(p): return p.read_text()
def wr(p, s): p.write_text(s)
def rep(p, old, new, count=1):
    s = rd(p); n = s.count(old); assert n == count, (p.name, old[:90], n); wr(p, s.replace(old, new))
BK = pathlib.Path('/tmp/claude-1000/-home-deck-semio/84faf2e3-4721-4a15-8b57-5d44cdd1d7f5/scratchpad/std-backup2/🔖️1/🪆️subsets/✳️any')
r = Q / '🦀️.rs'; t = Q / '🟦️.ts'; c = Q / '📏️cells/🦀️.rs'; tr = Q / '🧪️tests/🦀️.rs'; fx = Q / '🧪️tests/🧫️fixtures/🔣️.json'
for p in (r, t, c, tr, fx): shutil.copy(BK / p.relative_to(A), p)
MS = ['placement_x', 'placement_y', 'placement_z', 'placement_yaw']
AS = ['head_diameter'] + MS
def camel(s): p = s.split('_'); return p[0] + ''.join(x.capitalize() for x in p[1:])
# Rust codec
rep(r, 'FloatColumn::Binary64(28),FloatColumn::Binary64(29)];', 'FloatColumn::Binary64(28),FloatColumn::Binary64(29),' + ','.join(f'FloatColumn::Binary64({30 + i})' for i in range(4)) + '];')
s = rd(r)
s = s.replace('const BAR:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(10)];', 'const BAR:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(10),FloatColumn::Binary64(11)];\nconst SECTION:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5)];\nconst BARPT:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\nconst TENDON:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];\nconst STUD:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(6)];')
s = s.replace('const ANCHOR:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11)];', 'const ANCHOR:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),' + ','.join(f'FloatColumn::Binary64({12 + i})' for i in range(5)) + '];')
assert 'const SECTION' in s and 'Binary64(16)];' in s
s = s.replace('Cell::Real(v.relative_humidity)]', 'Cell::Real(v.relative_humidity),' + ','.join(f'Cell::Real(v.{f})' for f in MS) + ']', 1)
s = s.replace('Cell::Real(a.aggregate_size)],BAR)', 'Cell::Real(a.aggregate_size),Cell::Real(a.lap_share)],BAR)', 1)
ins = ('   for(k,a)in v.section_points.iter().enumerate(){p.insert_float("en1992_section_point",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Real(a.y),Cell::Real(a.z)],SECTION)?;}\n'
       '   for(k,a)in v.bars.iter().enumerate(){p.insert_float("en1992_bar_point",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Text(&a.layer_id),Cell::Real(a.y),Cell::Real(a.z),Cell::Real(a.diameter)],BARPT)?;}\n'
       '   for(k,a)in v.tendon_points.iter().enumerate(){p.insert_float("en1992_tendon_point",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Real(a.x),Cell::Real(a.y),Cell::Real(a.z)],TENDON)?;}\n'
       '   for(k,a)in v.stud_rings.iter().enumerate(){p.insert_float("en1992_stud_ring",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Real(a.radius),Cell::Integer(i64::from(a.count)),Cell::Real(a.diameter)],STUD)?;}\n')
o = '   for(k,a)in v.actions.iter().enumerate(){insert_action(p,"en1992_member_action",id,k,a)?;}\n'
assert s.count(o) == 1; s = s.replace(o, o + ins)
s = s.replace('Cell::Real(v.f_ck)],ANCHOR)', 'Cell::Real(v.f_ck),' + ','.join(f'Cell::Real(v.{f})' for f in AS) + '],ANCHOR)', 1)
s = s.replace('"en1992_member",30,MEMBER', '"en1992_member",34,MEMBER').replace('"en1992_anchor",12,ANCHOR', '"en1992_anchor",17,ANCHOR').replace('"en1992_bar_layer",11,BAR', '"en1992_bar_layer",12,BAR')
o = 'let mut actions=groups(entities(d,"en1992_member_action",15,ACTION,c)?,&member_rows,c)?;'
assert s.count(o) == 1
s = s.replace(o, o + '\n  let mut section_rows=groups(entities(d,"en1992_section_point",6,SECTION,c)?,&member_rows,c)?;let mut bar_point_rows=groups(entities(d,"en1992_bar_point",8,BARPT,c)?,&member_rows,c)?;let mut tendon_rows=groups(entities(d,"en1992_tendon_point",7,TENDON,c)?,&member_rows,c)?;let mut stud_rows=groups(entities(d,"en1992_stud_ring",7,STUD,c)?,&member_rows,c)?;')
o = 'aggregate_size:a.real(10)?});}'
assert s.count(o) == 1; s = s.replace(o, 'aggregate_size:a.real(10)?,lap_share:a.real(11)?});}\n   let mut section_points=Vec::new();for a in ordered(section_rows.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{section_points.push(SectionPoint{id:text(a,3,c)?,y:a.real(4)?,z:a.real(5)?});}\n   let mut bar_points=Vec::new();for a in ordered(bar_point_rows.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{bar_points.push(BarPoint{id:text(a,3,c)?,layer_id:text(a,4,c)?,y:a.real(5)?,z:a.real(6)?,diameter:a.real(7)?});}\n   let mut tendon_points=Vec::new();for a in ordered(tendon_rows.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{tendon_points.push(TendonPoint{id:text(a,3,c)?,x:a.real(4)?,y:a.real(5)?,z:a.real(6)?});}\n   let mut stud_rings=Vec::new();for a in ordered(stud_rows.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{stud_rings.push(StudRing{id:text(a,3,c)?,radius:a.real(4)?,count:u32_value(a,5)?,diameter:a.real(6)?});}')
o = 'relative_humidity:v.real(29)?'
assert s.count(o) == 1; s = s.replace(o, o + ',' + ','.join(f'{f}:v.real({30 + i})?' for i, f in enumerate(MS)) + ',section_points,bars:bar_points,tendon_points,stud_rings')
o = 'f_ck:v.real(11)?,actions}'
assert s.count(o) == 1; s = s.replace(o, 'f_ck:v.real(11)?,actions,' + ','.join(f'{f}:v.real({12 + i})?' for i, f in enumerate(AS)) + '}')
s = s.replace('use semio_s_artifact_norm_en1992_model::{ConcreteGrade, ReinforcementGrade, PrestressSteel, RcMember, BarLayer,', 'use semio_s_artifact_norm_en1992_model::{ConcreteGrade, ReinforcementGrade, PrestressSteel, RcMember, BarLayer, SectionPoint, BarPoint, TendonPoint, StudRing,', 1)
assert 'SectionPoint, BarPoint' in s
wr(r, s)
# TS codec
s = rd(t)
def sub(old, new, cnt=1):
    global s
    assert s.count(old) == cnt, (old[:90], s.count(old)); s = s.replace(old, new)
sub('aggregateSize:Binary64};', 'aggregateSize:Binary64;lapShare:Binary64};\nexport type En1992SqliteSectionPoint={id:string;y:Binary64;z:Binary64};\nexport type En1992SqliteBarPoint={id:string;layerId:string;y:Binary64;z:Binary64;diameter:Binary64};\nexport type En1992SqliteTendonPoint={id:string;x:Binary64;y:Binary64;z:Binary64};\nexport type En1992SqliteStudRing={id:string;radius:Binary64;count:number;diameter:Binary64};')
sub('fCk:Binary64;actions:En1992SqliteAction[]};', 'fCk:Binary64;actions:En1992SqliteAction[];' + ';'.join(f'{camel(f)}:Binary64' for f in AS) + '};')
sub('relativeHumidity:Binary64};', 'relativeHumidity:Binary64;' + ';'.join(f'{camel(f)}:Binary64' for f in MS) + ';sectionPoints:En1992SqliteSectionPoint[];bars:En1992SqliteBarPoint[];tendonPoints:En1992SqliteTendonPoint[];studRings:En1992SqliteStudRing[]};')
sub('const BAR:readonly Ieee754Column[]=[4,7,8,10].map(index=>({index,width:64}));', 'const BAR:readonly Ieee754Column[]=[4,7,8,10,11].map(index=>({index,width:64}));\nconst SECTION_POINT:readonly Ieee754Column[]=[4,5].map(index=>({index,width:64}));\nconst BAR_POINT:readonly Ieee754Column[]=[5,6,7].map(index=>({index,width:64}));\nconst TENDON_POINT:readonly Ieee754Column[]=[4,5,6].map(index=>({index,width:64}));\nconst STUD_RING:readonly Ieee754Column[]=[4,6].map(index=>({index,width:64}));')
sub('[4,6,7,8,9,10,11].map(index=>({index,width:64}))', '[4,6,7,8,9,10,11,12,13,14,15,16].map(index=>({index,width:64}))')
sub('[11,12,13,14,15,17,19,22,23,24,25,26,27,28,29]', '[11,12,13,14,15,17,19,22,23,24,25,26,27,28,29,30,31,32,33]')
sub('"bridgeDeltaSigmaS","relativeHumidity"]);collection(v.longitudinal);collection(v.actions);', '"bridgeDeltaSigmaS","relativeHumidity","placementX","placementY","placementZ","placementYaw","sectionPoints","bars","tendonPoints","studRings"]);collection(v.longitudinal);collection(v.actions);collection(v.sectionPoints);collection(v.bars);collection(v.tendonPoints);collection(v.studRings);')
sub('v.bridgeDeltaSigmaS,v.relativeHumidity],MEMBER', 'v.bridgeDeltaSigmaS,v.relativeHumidity,v.placementX,v.placementY,v.placementZ,v.placementYaw],MEMBER')
sub('"bondCondition","aggregateSize"]);', '"bondCondition","aggregateSize","lapShare"]);')
sub('a.bondCondition,a.aggregateSize],BAR', 'a.bondCondition,a.aggregateSize,a.lapShare],BAR')
o = '  for(let k=0;k<v.actions.length;k++)await writeAction(out,"en1992_member_action",id,k,v.actions[k]!,o);\n'
sub(o, o + '''  for(let k=0;k<v.sectionPoints.length;k++){const a=v.sectionPoints[k]!;fields(a,["id","y","z"]);await write(out,"en1992_section_point",[id,BigInt(k),a.id,a.y,a.z],SECTION_POINT,o)}
  for(let k=0;k<v.bars.length;k++){const a=v.bars[k]!;fields(a,["id","layerId","y","z","diameter"]);await write(out,"en1992_bar_point",[id,BigInt(k),a.id,a.layerId,a.y,a.z,a.diameter],BAR_POINT,o)}
  for(let k=0;k<v.tendonPoints.length;k++){const a=v.tendonPoints[k]!;fields(a,["id","x","y","z"]);await write(out,"en1992_tendon_point",[id,BigInt(k),a.id,a.x,a.y,a.z],TENDON_POINT,o)}
  for(let k=0;k<v.studRings.length;k++){const a=v.studRings[k]!;fields(a,["id","radius","count","diameter"]);await write(out,"en1992_stud_ring",[id,BigInt(k),a.id,a.radius,unsigned(a.count),a.diameter],STUD_RING,o)}
''')
sub('"c1","fCk","actions"]);', '"c1","fCk","actions","headDiameter","placementX","placementY","placementZ","placementYaw"]);')
sub('v.c1,v.fCk],ANCHOR', 'v.c1,v.fCk,v.headDiameter,v.placementX,v.placementY,v.placementZ,v.placementYaw],ANCHOR')
sub('entities(t[4]!,30,MEMBER,o)', 'entities(t[4]!,34,MEMBER,o)')
sub('entities(t[11]!,12,ANCHOR,o)', 'entities(t[11]!,17,ANCHOR,o)')
sub('entities(t[5]!,11,BAR,o)', 'entities(t[5]!,12,BAR,o)')
sub('anchorActions=await groups(await entities(t[12]!,15,ACTION,o),anchorRows,o),', 'anchorActions=await groups(await entities(t[12]!,15,ACTION,o),anchorRows,o),sectionRows=await groups(await entities(t[13]!,6,SECTION_POINT,o),memberRows,o),barPointRows=await groups(await entities(t[14]!,8,BAR_POINT,o),memberRows,o),tendonRows=await groups(await entities(t[15]!,7,TENDON_POINT,o),memberRows,o),studRows=await groups(await entities(t[16]!,7,STUD_RING,o),memberRows,o),')
sub('aggregateSize:readBinary64(a,10,BAR)})', 'aggregateSize:readBinary64(a,10,BAR),lapShare:readBinary64(a,11,BAR)})')
sub('  const s=stirrupRows.get(v.rowid),', '''  const sectionPoints:En1992SqliteSectionPoint[]=[],barPoints:En1992SqliteBarPoint[]=[],tendonPoints:En1992SqliteTendonPoint[]=[],studRings:En1992SqliteStudRing[]=[];
  for(const a of await ordered(sectionRows.get(v.rowid)??[],v.rowid,o))sectionPoints.push({id:artifactSqliteText(a,3),y:readBinary64(a,4,SECTION_POINT),z:readBinary64(a,5,SECTION_POINT)});
  for(const a of await ordered(barPointRows.get(v.rowid)??[],v.rowid,o))barPoints.push({id:artifactSqliteText(a,3),layerId:artifactSqliteText(a,4),y:readBinary64(a,5,BAR_POINT),z:readBinary64(a,6,BAR_POINT),diameter:readBinary64(a,7,BAR_POINT)});
  for(const a of await ordered(tendonRows.get(v.rowid)??[],v.rowid,o))tendonPoints.push({id:artifactSqliteText(a,3),x:readBinary64(a,4,TENDON_POINT),y:readBinary64(a,5,TENDON_POINT),z:readBinary64(a,6,TENDON_POINT)});
  for(const a of await ordered(studRows.get(v.rowid)??[],v.rowid,o))studRings.push({id:artifactSqliteText(a,3),radius:readBinary64(a,4,STUD_RING),count:readUnsigned(a,5),diameter:readBinary64(a,6,STUD_RING)});
  const s=stirrupRows.get(v.rowid),''')
sub('relativeHumidity:readBinary64(v,29,MEMBER),', 'relativeHumidity:readBinary64(v,29,MEMBER),placementX:readBinary64(v,30,MEMBER),placementY:readBinary64(v,31,MEMBER),placementZ:readBinary64(v,32,MEMBER),placementYaw:readBinary64(v,33,MEMBER),sectionPoints,bars:barPoints,tendonPoints,studRings,')
sub('fCk:readBinary64(v,11,ANCHOR),actions}', 'fCk:readBinary64(v,11,ANCHOR),actions,headDiameter:readBinary64(v,12,ANCHOR),placementX:readBinary64(v,13,ANCHOR),placementY:readBinary64(v,14,ANCHOR),placementZ:readBinary64(v,15,ANCHOR),placementYaw:readBinary64(v,16,ANCHOR)}')
wr(t, s)
# cells
s = rd(c)
def sub2(old, new):
    global s
    assert s.count(old) == 1, (old[:80], s.count(old)); s = s.replace(old, new)
W = '[("en1992_anchor",41),("en1992_anchor_action",31),("en1992_bar_layer",22),("en1992_bar_point",14),("en1992_concrete_grade",8),("en1992_document",10),("en1992_fire",7),("en1992_member",72),("en1992_member_action",31),("en1992_prestress",13),("en1992_prestress_steel",11),("en1992_punching",11),("en1992_reinforcement_grade",18),("en1992_section_point",10),("en1992_stirrups",8),("en1992_stud_ring",11),("en1992_tendon_point",13)]'
s = re.sub(r'const WIDTHS:&\[\(&str,usize\)\]=&\[.*?\];', 'const WIDTHS:&[(&str,usize)]=&' + W + ';', s, count=1, flags=re.S)
sub2('shape(value,33)?;\n   c.row("en1992_member"', 'shape(value,41)?;\n   c.row("en1992_member"')
sub2('(30,Float),(31,Float),(32,Float)],n)?)?,n)?;\n   collection(field(value,15)', '(30,Float),(31,Float),(32,Float),(33,Float),(34,Float),(35,Float),(36,Float)],n)?)?,n)?;\n   collection(field(value,37)?,3,"en1992_section_point",&[(0,Text),(1,Float),(2,Float)],&mut c,n)?;\n   collection(field(value,38)?,5,"en1992_bar_point",&[(0,Text),(1,Text),(2,Float),(3,Float),(4,Float)],&mut c,n)?;\n   collection(field(value,39)?,4,"en1992_tendon_point",&[(0,Text),(1,Float),(2,Float),(3,Float)],&mut c,n)?;\n   collection(field(value,40)?,4,"en1992_stud_ring",&[(0,Text),(1,Float),(2,UInt),(3,Float)],&mut c,n)?;\n   collection(field(value,15)')
sub2('collection(field(value,15)?,8,"en1992_bar_layer",&[(0,Text),(1,Float),(2,UInt),(3,Text),(4,Float),(5,Float),(6,Text),(7,Float)]', 'collection(field(value,15)?,9,"en1992_bar_layer",&[(0,Text),(1,Float),(2,UInt),(3,Text),(4,Float),(5,Float),(6,Text),(7,Float),(8,Float)]')
sub2('shape(value,10)?;c.row("en1992_anchor",add(24,fields(value,&[(0,Text),(1,Float),(2,Bool),(3,Float),(4,Float),(5,Float),(6,Float),(7,Float),(8,Float)],n)?)?,n)?;', 'shape(value,15)?;c.row("en1992_anchor",add(24,fields(value,&[(0,Text),(1,Float),(2,Bool),(3,Float),(4,Float),(5,Float),(6,Float),(7,Float),(8,Float),(10,Float),(11,Float),(12,Float),(13,Float),(14,Float)],n)?)?,n)?;')
wr(c, s)
# rust test
s = rd(tr)
sub2('d.tables.len(),13)', 'd.tables.len(),17)')
sub2('v.bridge_delta_sigma_s=w;v.relative_humidity=w;', 'v.bridge_delta_sigma_s=w;v.relative_humidity=w;v.placement_x=w;v.placement_y=w;v.placement_z=w;v.placement_yaw=w;')
sub2('a.aggregate_size=w;}', 'a.aggregate_size=w;a.lap_share=w;}')
sub2('v.c1=w;v.f_ck=w;', 'v.c1=w;v.f_ck=w;v.head_diameter=w;v.placement_x=w;v.placement_y=w;v.placement_z=w;v.placement_yaw=w;')
wr(tr, s)
# fixtures json
j = json.loads(rd(fx))
j['tableNames'] += ['en1992_section_point', 'en1992_bar_point', 'en1992_tendon_point', 'en1992_stud_ring']
j['semanticCells'] = {'fullRows': 17, 'absentRows': 9, 'tableWidths': [list(x) for x in json.loads(W.replace('(', '[').replace(')', ']'))]}
wr(fx, json.dumps(j, indent=2, ensure_ascii=False) + '\n')
print('sqlite codecs done')

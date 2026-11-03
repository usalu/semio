"""EN 1992 v8: BarPoint `length` + `bend` through the SQLite layer (sql, rs codec, ts codec, cell census, fixture). Idempotent."""
import pathlib
Q = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot')
def sub(p, old, new):
    s = p.read_text()
    if new in s: print('skip', p.name); return
    assert s.count(old) == 1, (p.name, old[:90], s.count(old))
    p.write_text(s.replace(old, new)); print('edit', p.name)

sub(Q / '🗄️.sql', 'logical_id TEXT NOT NULL, layer_id TEXT NOT NULL, y REAL, z REAL, diameter REAL,\n y_ieee754_bits INTEGER NOT NULL, y_ieee754_class TEXT NOT NULL,\n z_ieee754_bits INTEGER NOT NULL, z_ieee754_class TEXT NOT NULL,\n diameter_ieee754_bits INTEGER NOT NULL, diameter_ieee754_class TEXT NOT NULL\n);',
    'logical_id TEXT NOT NULL, layer_id TEXT NOT NULL, y REAL, z REAL, diameter REAL, length REAL, bend TEXT NOT NULL,\n y_ieee754_bits INTEGER NOT NULL, y_ieee754_class TEXT NOT NULL,\n z_ieee754_bits INTEGER NOT NULL, z_ieee754_class TEXT NOT NULL,\n diameter_ieee754_bits INTEGER NOT NULL, diameter_ieee754_class TEXT NOT NULL,\n length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL\n);')
r = Q / '🦀️.rs'
sub(r, 'const BARPT:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];', 'const BARPT:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];')
sub(r, 'Cell::Real(a.z),Cell::Real(a.diameter)],BARPT)', 'Cell::Real(a.z),Cell::Real(a.diameter),Cell::Real(a.length),Cell::Text(&a.bend)],BARPT)')
sub(r, 'entities(d,"en1992_bar_point",8,BARPT,c)', 'entities(d,"en1992_bar_point",10,BARPT,c)')
sub(r, 'diameter:a.real(7)?});}\n   let mut tendon_points', 'diameter:a.real(7)?,length:a.real(8)?,bend:text(a,9,c)?});}\n   let mut tendon_points')
t = Q / '🟦️.ts'
sub(t, 'const BAR_POINT:readonly Ieee754Column[]=[5,6,7].map', 'const BAR_POINT:readonly Ieee754Column[]=[5,6,7,8].map')
sub(t, 'fields(a,["id","layerId","y","z","diameter"]);await write(out,"en1992_bar_point",[id,BigInt(k),a.id,a.layerId,a.y,a.z,a.diameter],BAR_POINT,o)', 'fields(a,["id","layerId","y","z","diameter","length","bend"]);await write(out,"en1992_bar_point",[id,BigInt(k),a.id,a.layerId,a.y,a.z,a.diameter,a.length,a.bend],BAR_POINT,o)')
sub(t, 'entities(t[14]!,8,BAR_POINT,o)', 'entities(t[14]!,10,BAR_POINT,o)')
sub(t, 'diameter:readBinary64(a,7,BAR_POINT)});', 'diameter:readBinary64(a,7,BAR_POINT),length:readBinary64(a,8,BAR_POINT),bend:artifactSqliteText(a,9)});')
sub(t, 'diameter:Binary64};\nexport type En1992SqliteTendonPoint', 'diameter:Binary64;length:Binary64;bend:string};\nexport type En1992SqliteTendonPoint')
c = Q / '📏️cells/🦀️.rs'
sub(c, '("en1992_bar_point",14)', '("en1992_bar_point",18)')
sub(c, 'collection(field(value,34)?,5,"en1992_bar_point",&[(0,Text),(1,Text),(2,Float),(3,Float),(4,Float)]', 'collection(field(value,34)?,7,"en1992_bar_point",&[(0,Text),(1,Text),(2,Float),(3,Float),(4,Float),(5,Float),(6,Text)]')
sub(Q / '🧪️tests/🧫️fixtures/🔣️.json', '"layerId": "l1",\n            "y": 0.5,\n            "z": 0.5,\n            "diameter": 0.5\n', '"layerId": "l1",\n            "y": 0.5,\n            "z": 0.5,\n            "diameter": 0.5,\n            "length": 0.5,\n            "bend": "auto"\n')

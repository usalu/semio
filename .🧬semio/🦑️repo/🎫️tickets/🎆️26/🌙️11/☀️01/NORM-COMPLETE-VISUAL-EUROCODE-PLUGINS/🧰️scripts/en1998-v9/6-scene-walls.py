"""en1998 v9 step 6: scene masonry walls grouped by storey reference; column-size lookup by member kind; field meta; role choices. python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
B = ANY / "🎬️scene/🏢️buildings/🦀️.rs"
sub(B, 'm.role.eq_ignore_ascii_case("column")', "m.role == super::model::MemberKind::Column")
sub(B, "rows.iter().map(|(_, w)| *w).collect();", "rows.iter().map(|(_, w)| w.clone()).collect();")
sub(B, "let (bottom, top) = (if storey == 1 { 0.0 } else { r.elevations[storey - 2] }, r.elevations[storey - 1]);", "let (bottom, top) = (if storey == 1 { 0.0 } else { r.elevations[storey - 2] }, r.elevations[storey - 1]);\n        let storey_id = b.storeys[storey - 1].id.as_str();")
sub(B, "rows.iter().filter(|(_, w)| (w.storey == 0 || w.storey == storey) && (w.direction_rad.sin().abs() > w.direction_rad.cos().abs()) == along_y).collect();", "rows.iter().filter(|(_, w)| w.in_storey(storey_id) && w.along_y() == along_y).collect();")
sub(B, 'format!("Masonry wall {} at storey {storey}: l = {:.2} m, t = {:.2} m", m.id,', 'format!("Masonry wall {} at storey {storey_id}: l = {:.2} m, t = {:.2} m", m.id,')
sub(B, 'format!("Mauerwerkswand {} im Geschoss {storey}: l = {:.2} m, t = {:.2} m", m.id,', 'format!("Mauerwerkswand {} im Geschoss {storey_id}: l = {:.2} m, t = {:.2} m", m.id,')
sub(B, 'format!("{}:wall:{}:{storey}", b.id, m.id)', 'format!("{}:wall:{}:{storey_id}", b.id, m.id)')
sub(B, "part_1::masonry_wall_density(&walls, area, along_y, storey);", "part_1::masonry_wall_density(&walls, area, along_y, storey_id);")
sub(B, 'format!("{} {storey}, {} {direction}: \\u{3a3} l\\u{b7}t / A = {:.2} % {} {:.2} %", if de { "Geschoss" } else { "Storey" },', 'format!("{} {storey_id}, {} {direction}: \\u{3a3} l\\u{b7}t / A = {:.2} % {} {:.2} %", if de { "Geschoss" } else { "Storey" },')
sub(B, 'format!("{}:wall-density:{direction}:{storey}", b.id)', 'format!("{}:wall-density:{direction}:{storey_id}", b.id)')
F = ANY / "🏷️field-meta/🦀️.rs"
sub(F, '    NormFieldChoice { value: "wall", label_en: "Wall", label_de: "Wand" },\n];', '    NormFieldChoice { value: "wall", label_en: "Wall", label_de: "Wand" },\n    NormFieldChoice { value: "brace", label_en: "Brace", label_de: "Verband" },\n];')
sub(F, '"Verflüssigungsfähige oder empfindliche Böden (Baugrundklasse S2)")),', '"Verflüssigungsfähige oder empfindliche Böden (Baugrundklasse S2)", choices BOOL)),')
flush()

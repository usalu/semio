"""en1998 v9 step 5: kernel (masonry walls grouped by storey reference, member kind), checks, scene call sites, field meta, tests, oracle. python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
K = "semio_s_artifact_norm_en1998_model::MemberKind"
R = S / "🦀️.rs"
sub(R, "liquefiable_or_sensitive: site.liquefiable_layer > 0.0,", "liquefiable_or_sensitive: site.liquefiable_layer,")
sub(R, '''    /// 🧱 One shear wall row of a simple masonry building (EN 1998-1 §9.7): plan length, thickness, axis direction from the plan x axis in radians and the storey number counted from 1 (0 = every storey).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct MasonryWall {
        pub length_m: f64,
        pub thickness_m: f64,
        pub direction_rad: f64,
        pub storey: usize,
    }

    /// 🧱 The masonry wall rows among the member rows: material masonry with a positive length and thickness; the rounded storey number is clamped at 0.
    pub fn masonry_walls_of(members: &[super::En1998Member]) -> Vec<MasonryWall> {
        members.iter()
            .filter(|m| m.material.eq_ignore_ascii_case("masonry") && m.wall_length_m > 0.0 && m.wall_thickness_m > 0.0)
            .map(|m| MasonryWall { length_m: m.wall_length_m, thickness_m: m.wall_thickness_m, direction_rad: m.wall_direction_rad, storey: m.wall_storey_id.round().max(0.0) as usize })
            .collect()
    }

    /// 🧱 Wall-area density Σ l·t / A of the walls running along the plan y axis (else x) in a storey, with A the floor area; a wall belongs to the direction its axis is closer to and 0 for no floor area.
    pub fn masonry_wall_density(walls: &[MasonryWall], floor_area_m2: f64, along_y: bool, storey: usize) -> f64 {
        if floor_area_m2 <= 0.0 {
            return 0.0;
        }
        walls.iter()
            .filter(|w| (w.storey == 0 || w.storey == storey) && (w.direction_rad.sin().abs() > w.direction_rad.cos().abs()) == along_y)
            .map(|w| w.length_m * w.thickness_m)
            .sum::<f64>() / floor_area_m2
    }

    /// 🧱 Governing wall-area density of the storeys 1..=`storeys` in both plan directions: the smallest one with its direction (`along_y`) and storey; `None` without walls or storeys.
    pub fn masonry_governing_density(walls: &[MasonryWall], floor_area_m2: f64, storeys: usize) -> Option<(f64, bool, usize)> {
        if walls.is_empty() {
            return None;
        }
        (1..=storeys)
            .flat_map(|storey| [false, true].map(|along_y| (masonry_wall_density(walls, floor_area_m2, along_y, storey), along_y, storey)))
            .min_by(|a, b| a.0.total_cmp(&b.0))
    }
''', '''    /// 🧱 One shear wall row of a simple masonry building (EN 1998-1 §9.7): plan length, thickness, axis direction from the plan x axis in radians and the id of the storey row it belongs to (empty = every storey).
    #[derive(Clone, Debug, PartialEq)]
    pub struct MasonryWall {
        pub length_m: f64,
        pub thickness_m: f64,
        pub direction_rad: f64,
        pub storey_id: String,
    }

    impl MasonryWall {
        /// 🏬 Whether the wall counts in the storey row `storey_id`: a wall without a storey reference counts in every storey.
        pub fn in_storey(&self, storey_id: &str) -> bool {
            self.storey_id.is_empty() || self.storey_id == storey_id
        }

        /// 🧭 Whether the wall axis is closer to the plan y axis than to the x axis.
        pub fn along_y(&self) -> bool {
            self.direction_rad.sin().abs() > self.direction_rad.cos().abs()
        }
    }

    /// 🧱 Whether a member row is a masonry wall row: kind wall, material masonry, positive length and thickness; the other kinds never carry wall fields into a check.
    pub fn is_masonry_wall(member: &super::En1998Member) -> bool {
        member.role == super::MemberKind::Wall && member.material.eq_ignore_ascii_case("masonry") && member.wall_length_m > 0.0 && member.wall_thickness_m > 0.0
    }

    /// 🧱 The masonry wall rows among the member rows (see [`is_masonry_wall`]).
    pub fn masonry_walls_of(members: &[super::En1998Member]) -> Vec<MasonryWall> {
        members.iter()
            .filter(|m| is_masonry_wall(m))
            .map(|m| MasonryWall { length_m: m.wall_length_m, thickness_m: m.wall_thickness_m, direction_rad: m.wall_direction_rad, storey_id: m.wall_storey_id.clone() })
            .collect()
    }

    /// 🧱 Wall-area density Σ l·t / A of the walls running along the plan y axis (else x) in the storey row `storey_id`, with A the floor area; a wall belongs to the direction its axis is closer to and 0 for no floor area.
    pub fn masonry_wall_density(walls: &[MasonryWall], floor_area_m2: f64, along_y: bool, storey_id: &str) -> f64 {
        if floor_area_m2 <= 0.0 {
            return 0.0;
        }
        walls.iter()
            .filter(|w| w.in_storey(storey_id) && w.along_y() == along_y)
            .map(|w| w.length_m * w.thickness_m)
            .sum::<f64>() / floor_area_m2
    }

    /// 🧱 Governing wall-area density over the storey rows `storey_ids` (bottom to top; no rows means one anonymous storey) in both plan directions: the smallest one with its direction (`along_y`) and storey id; `None` without walls. A wall whose storey id names no row counts in no storey (the integrity check reports it).
    pub fn masonry_governing_density(walls: &[MasonryWall], floor_area_m2: f64, storey_ids: &[&str]) -> Option<(f64, bool, String)> {
        if walls.is_empty() {
            return None;
        }
        let anonymous = [""];
        let ids: &[&str] = if storey_ids.is_empty() { &anonymous } else { storey_ids };
        ids.iter()
            .flat_map(|id| [false, true].map(|along_y| (masonry_wall_density(walls, floor_area_m2, along_y, id), along_y, (*id).to_owned())))
            .min_by(|a, b| a.0.total_cmp(&b.0))
    }
''')
sub(R, '''        let role = match role.trim().to_ascii_lowercase().as_str() {
            "column" => "column",
            "beam" => "beam",
            _ => "other",
        };''', '''        let role = match role.trim().to_ascii_lowercase().as_str() {
            "column" => "column",
            "beam" => "beam",
            _ => "other",
        };''')

C = S / "💡️inferences/🌍️full-seismic-check/🧱️structural-system-checks/🔗️ductility-and-masonry/🦀️.rs"
sub(C, 'member.role.to_ascii_lowercase() == "column"', f"member.role == {K}::Column", 2)
sub(C, 'member.role.to_ascii_lowercase() == "beam"', f"member.role == {K}::Beam", 2)
sub(C, 'member.role.to_ascii_lowercase() != "column"', f"member.role != {K}::Column")
sub(C, "part_1::rc_rho_limits(&member.role, gov_dc)", "part_1::rc_rho_limits(member.role.as_str(), gov_dc)")
sub(C, "let governing = part_1::masonry_governing_density(&walls, area, building.storeys.len().max(1));", "let storey_ids: Vec<&str> = building.storeys.iter().map(|s| s.id.as_str()).collect();\n        let governing = part_1::masonry_governing_density(&walls, area, &storey_ids);")
sub(C, "let location = governing.map_or((String::new(), String::new()), |(_, along_y, storey)| (", "let location = governing.as_ref().map_or((String::new(), String::new()), |(_, along_y, storey)| (")
sub(C, "let ratio = governing.map_or(building.masonry_wall_area_ratio, |g| g.0);", "let ratio = governing.as_ref().map_or(building.masonry_wall_area_ratio, |g| g.0);")
sub(C, "m = match governing.and_then(|(_, along_y, storey)| walls_in_group(building, along_y, storey)) {", "m = match governing.as_ref().and_then(|(_, along_y, storey)| walls_in_group(building, *along_y, storey)) {")
sub(C, '''/// 🧱 The longest wall row (id, length, thickness) running along the governing plan direction in the governing storey, the row a remedy lengthens.
fn walls_in_group(building: &semio_s_artifact_norm_en1998_model::En1998Building, along_y: bool, storey: usize) -> Option<(String, f64, f64)> {
    building.members.iter()
        .filter(|m| m.material.eq_ignore_ascii_case("masonry") && m.wall_length_m > 0.0 && m.wall_thickness_m > 0.0)
        .filter(|m| {
            let rounded = m.wall_storey_id.round().max(0.0) as usize;
            (rounded == 0 || rounded == storey) && (m.wall_direction_rad.sin().abs() > m.wall_direction_rad.cos().abs()) == along_y
        })''', '''/// 🧱 The longest wall row (id, length, thickness) running along the governing plan direction in the governing storey row, the row a remedy lengthens.
fn walls_in_group(building: &semio_s_artifact_norm_en1998_model::En1998Building, along_y: bool, storey_id: &str) -> Option<(String, f64, f64)> {
    building.members.iter()
        .filter(|m| part_1::is_masonry_wall(m))
        .filter(|m| (m.wall_storey_id.is_empty() || m.wall_storey_id == storey_id) && (m.wall_direction_rad.sin().abs() > m.wall_direction_rad.cos().abs()) == along_y)''')
flush()

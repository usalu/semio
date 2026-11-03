//! 📐️ Case laws: the dimension and layout laws every design-case catalogue obeys, shared by the case forms and the scene crates' tests.

use crate::*;

/// 📐️ The dimension law of design cases: for every case that declares geometry and every driving input it sets (width, length, height, load, auxiliary; the grain and ply-direction roles draw a hatch, not a size; the annotation roles `dim_*` are checked on their dimension annotation), scaling the input by 1.25 changes the drawing; a length input (and the load bar) changes one scalar of the primitives by exactly that factor, a count, angle or ratio input changes the signature; inputs whose drawn size is clamped (below 1 cm or above 40 m, or a bar beyond its range) are skipped; at least one case must be driven. An annotation role scales its dimension line by exactly that factor (lengths) or turns its angular dimension by exactly that factor (angles).
pub fn assert_case_dimension_laws<C: norm::design_case::CaseCatalogue>(rows: &[norm::design_case::CaseRow]) { assert_case_dimension_laws_with::<C>(rows, &design_case::forms::no_extras); }

/// 📐️ [`assert_case_dimension_laws`] for a catalogue that hands in `ext` extras (assemblies, char, modes): the signature includes their primitives.
pub fn assert_case_dimension_laws_with<C: norm::design_case::CaseCatalogue>(rows: &[norm::design_case::CaseRow], ext: design_case::forms::Extend<'_>) {
    use crate::design_case::{forms, input_metres};
    const FACTOR: f64 = 1.25;
    let role = MaterialRole::Neutral;
    let mut driven = 0;
    for row in rows {
        let Some(geometry) = forms::declared::<C>(row) else { continue };
        let before = forms::signature_with::<C>(row, &role, ext);
        let keys = [("width", geometry.width), ("length", geometry.length), ("height", geometry.height)].into_iter().map(|(name, key)| (name.to_string(), key)).chain(geometry.aux.iter().map(|(name, key)| ((*name).to_string(), *key)));
        for (name, key) in keys.filter(|(name, key)| !key.is_empty() && !forms::aux_is_appearance(name) && forms::dimension_role(name).is_none()) {
            let Some(index) = row.numbers.iter().position(|n| n.id == key) else { continue };
            let mut next = row.clone();
            next.numbers[index].value *= FACTOR;
            let after = forms::signature_with::<C>(&next, &role, ext);
            if forms::aux_is_scalar(geometry.form, &name) || forms::dimension_is_scalar(&geometry, &name) {
                assert!(before != after, "case {}: the {name} input {key} does not change the drawing", row.id);
            } else {
                let Some(metres) = input_metres::<C>(row, key) else { continue };
                if !(0.01..=40.0).contains(&metres) {
                    continue;
                }
                assert!(before.iter().zip(&after).any(|(b, a)| *b > 1e-9 && (a / b - FACTOR).abs() < 1e-6), "case {}: no primitive dimension follows the {name} input {key} by the factor {FACTOR}", row.id);
            }
            driven += 1;
        }
        if let (Some(index), Some(driver)) = (row.numbers.iter().position(|n| n.id == geometry.load), forms::driver::<C>(row, &geometry)) {
            let bar = driver.height();
            if bar > forms::BAR_MIN * 1.01 && bar * FACTOR < forms::BAR_MAX {
                let mut next = row.clone();
                next.numbers[index].value *= FACTOR;
                let after = forms::driver::<C>(&next, &geometry).expect("driver").height();
                assert!((after / bar - FACTOR).abs() < 1e-9, "case {}: the load bar does not follow the load input {} proportionally", row.id, geometry.load);
                driven += 1;
            }
        }
        driven += assert_annotation_laws::<C>(row, &geometry);
    }
    assert!(driven > 0 || rows.iter().all(|row| forms::declared::<C>(row).is_none()), "no case has a driving input set, so the dimension law proved nothing");
}

/// 📏️ The annotation law of one case: for every annotation role (`dim_*`) of its geometry whose input is set, scaling the input by 1.25 scales the length of the dimension line and its metres by 1.25 (a length in 0.01 to 40 m), or turns the angular dimension, measured as twice the direction of its chord above the vertical, and its radians by 1.25 (an angle from 0.05 rad whose 1.25 multiple stays below the straight angle); returns how many roles were proven.
fn assert_annotation_laws<C: norm::design_case::CaseCatalogue>(row: &norm::design_case::CaseRow, geometry: &norm::design_case::CaseGeometry) -> usize {
    use crate::design_case::{forms, input_metres};
    const FACTOR: f64 = 1.25;
    let pose = forms::CasePose { origin: Vec3::ZERO, yaw: 0.0 };
    let mut proven = 0;
    for (role, key) in geometry.aux.iter().filter(|(role, key)| !key.is_empty() && forms::dimension_role(role).is_some()) {
        let Some(index) = row.numbers.iter().position(|n| n.id == *key) else { continue };
        let id = format!("{}:{}", row.id, forms::dimension_tag(role));
        let measure = |case: &norm::design_case::CaseRow| forms::dimension_annotations::<C>(case, &pose, Tone::Info).into_iter().find_map(|a| match a { Annotation::Dimension { id: found, from, to, value, .. } if found == id => Some((from, to, value.value)), _ => None });
        let Some((from, to, value)) = measure(row) else { continue };
        let mut next = row.clone();
        next.numbers[index].value *= FACTOR;
        if forms::dimension_role(role) == Some(forms::Stem::Angle) {
            let radians = forms::input_radians::<C>(row, key).expect("radians");
            if radians < 0.05 || radians * FACTOR >= forms::ANGLE_MAX {
                continue;
            }
            let theta = |from: &Vec3, to: &Vec3| 2.0 * ((to.z - from.z).atan2(to.x - from.x) - std::f64::consts::FRAC_PI_2);
            let (from2, to2, value2) = measure(&next).unwrap_or_else(|| panic!("case {}: the angular dimension {id} vanishes", row.id));
            assert!((theta(&from, &to) - radians).abs() < 1e-9 && (theta(&from2, &to2) - radians * FACTOR).abs() < 1e-9 && (value2 / value - FACTOR).abs() < 1e-9, "case {}: the angular dimension {id} does not turn with the input {key} by the factor {FACTOR}", row.id);
        } else {
            let Some(metres) = input_metres::<C>(row, key) else { continue };
            if !(0.01..=40.0 / FACTOR).contains(&metres) {
                continue;
            }
            let length = |from: &Vec3, to: &Vec3| ((to.x - from.x).powi(2) + (to.y - from.y).powi(2) + (to.z - from.z).powi(2)).sqrt();
            let (from2, to2, value2) = measure(&next).unwrap_or_else(|| panic!("case {}: the dimension {id} vanishes", row.id));
            assert!((length(&from, &to) - metres).abs() < 1e-9 && (length(&from2, &to2) / length(&from, &to) - FACTOR).abs() < 1e-9 && (value2 / value - FACTOR).abs() < 1e-9, "case {}: the dimension line {id} does not follow the input {key} by the factor {FACTOR}", row.id);
        }
        proven += 1;
    }
    proven
}

/// 🗺️ The layout law of design cases: the default plan rectangles of the cases of `rows` at the poses `pose_of` gives overlap neither each other nor any of the `obstacles` (rows of typed entities, members); at least two rectangles must have been compared.
pub fn assert_case_layout_clear<C: norm::design_case::CaseCatalogue>(rows: &[norm::design_case::CaseRow], pose_of: &dyn Fn(&str) -> Option<design_case::forms::CasePose>, ext: design_case::forms::Extend<'_>, obstacles: &[[f64; 4]]) {
    use crate::design_case::forms;
    let rects: Vec<(&str, [f64; 4])> = rows.iter().filter_map(|row| Some((row.id.as_str(), forms::extent_with::<C>(row, ext)?)).zip(pose_of(&row.id)).map(|((id, e), pose)| (id, [pose.origin.x + e[0], pose.origin.x + e[1], pose.origin.y + e[2], pose.origin.y + e[3]]))).collect();
    assert!(rects.len() > 1, "fewer than two case rectangles to compare");
    for (index, (id, rect)) in rects.iter().enumerate() {
        assert!(rect.iter().all(|v| v.is_finite()), "case {id} has a non-finite rectangle");
        assert!(rects[index + 1..].iter().all(|(_, other)| !forms::overlaps(rect, other)), "case {id} {rect:?} overlaps another case in plan");
        assert!(obstacles.iter().all(|obstacle| !forms::overlaps(rect, obstacle)), "case {id} {rect:?} overlaps an obstacle");
    }
}

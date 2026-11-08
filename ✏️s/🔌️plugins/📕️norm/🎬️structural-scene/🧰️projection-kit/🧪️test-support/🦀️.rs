//! 🧪️ Test support: the golden-law checks every scene crate's unit tests share.

use crate::*;
use std::collections::BTreeSet;

pub fn revision() -> SceneRevision {
    SceneRevision { document_seq: 1, report_seq: 1 }
}

/// 🔎️ The entity `id` of a scene, panicking with the available ids.
pub fn entity<'a>(scene: &'a StructuralScene, id: &str) -> &'a SceneEntity {
    scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("entity {id} missing in {:?}", scene.entities.iter().map(|e| &e.entity.id).collect::<Vec<_>>()))
}

/// ⚖️ The laws every projection obeys: it validates, entity and annotation ids are unique, every entity sits on an existing layer and carries both languages.
pub fn assert_scene_laws(scene: &StructuralScene) {
    assert_eq!(scene.validate(), Ok(()), "scene violations");
    assert_eq!(scene.schema, SCENE_SCHEMA_ID);
    let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    assert_eq!(ids.len(), scene.entities.len(), "duplicate entity ids");
    let annotation_ids: BTreeSet<_> = scene.annotations.iter().map(|a| a.id()).collect();
    assert_eq!(annotation_ids.len(), scene.annotations.len(), "duplicate annotation ids");
    let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
    assert!(scene.entities.iter().all(|e| layers.contains(e.layer.as_str())), "entity on a missing layer");
    assert!(scene.entities.iter().all(|e| !e.accessible_name.en.is_empty() && !e.accessible_name.de.is_empty()), "entity without a bilingual accessible name");
    assert_eq!(crate::projection::description::german_scene_leaks(scene), Vec::<String>::new(), "German scene texts read as English");
}

/// 🇩🇪️ No handle accessible name reads as English in German.
pub fn assert_handle_laws(handles: &[Handle]) {
    assert_eq!(crate::projection::description::german_handle_leaks(handles), Vec::<String>::new(), "German handle names read as English");
    assert!(handles.iter().all(|h| !h.accessible_name.en.trim().is_empty() && !h.accessible_name.de.trim().is_empty()), "handle without a bilingual accessible name");
}

/// 🧭️ Every `Angular` handle follows the one zero-direction convention of [`HandleKind::angular_zero_direction`]: the angle of its anchor about the axis is defined, lies inside the limits and the point at that angle gives the same angle back.
pub fn assert_angular_handle_laws(handles: &[Handle]) {
    for handle in handles.iter().filter(|h| matches!(h.kind, HandleKind::Angular { .. })) {
        let s = handle.kind.angular_s(&handle.anchor).unwrap_or_else(|| panic!("angular handle {} has its anchor on the axis", handle.id));
        if let Some(limits) = &handle.limits {
            assert!(s >= limits.min - 1e-9 && s <= limits.max + 1e-9, "angular handle {}: s = {s} outside the limits", handle.id);
        }
        let HandleKind::Angular { centre, .. } = &handle.kind else { continue };
        let back = handle.kind.angular_point(s, handle.anchor.distance(centre)).expect("point");
        assert!((handle.kind.angular_s(&back).unwrap_or(f64::NAN) - s).abs() < 1e-6, "angular handle {}: the angle does not map back", handle.id);
    }
}

/// 🧊️ The finite, non-empty geometry of one drawn case part, whatever primitive it is: box, cylinder, wedge, prism (outline area), laminate (plies and footprint), surface grid with its overlay and pin patterns.
fn assert_case_part_geometry(part: &SceneEntity) {
    let id = &part.entity.id;
    let finite = |values: &[f64]| values.iter().all(|v| v.is_finite());
    let area = |points: &[Vec2]| points.iter().zip(points.iter().cycle().skip(1)).map(|(a, b)| a.x * b.y - b.x * a.y).sum::<f64>().abs() / 2.0;
    match &part.geometry {
        Geometry::Solid(solid) => match &solid.shape {
            SolidShape::Box { size, frame } => assert!(finite(&[size.x, size.y, size.z, frame.origin.x, frame.origin.y, frame.origin.z]) && size.x > 0.0 && size.y > 0.0 && size.z > 0.0, "case part {id} has a non-finite or empty box"),
            SolidShape::Cylinder { radius, height, frame } => assert!(finite(&[*radius, *height, frame.origin.x, frame.origin.y, frame.origin.z]) && *radius > 0.0 && *height > 0.0, "case part {id} has a non-finite or empty cylinder"),
            SolidShape::Wedge { base, height, frame } => assert!(finite(&[base.x, base.y, *height, frame.origin.x, frame.origin.y, frame.origin.z]) && base.x > 0.0 && base.y > 0.0 && *height > 0.0, "case part {id} has a non-finite or empty wedge"),
            SolidShape::Prism { base, height, frame } => assert!(base.outer.points.iter().all(|p| p.x.is_finite() && p.y.is_finite()) && finite(&[*height, frame.origin.x, frame.origin.y, frame.origin.z]) && *height > 0.0 && area(&base.outer.points) > 1e-12, "case part {id} has a non-finite or empty prism"),
            SolidShape::Laminate { footprint, layers, frame } => assert!(!layers.is_empty() && layers.iter().all(|l| l.thickness.is_finite() && l.thickness > 0.0 && l.grain.is_finite()) && frame.origin.x.is_finite() && area(&footprint.outer.points) > 1e-12, "case part {id} has an empty or non-finite ply stack"),
            other => panic!("case part {id} has the unexpected shape {other:?}"),
        },
        Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }) => assert!(grid.rows > 1 && grid.columns > 1 && grid.points.len() == (grid.rows * grid.columns) as usize && grid.points.iter().all(|p| finite(&[p.x, p.y, p.z])), "case part {id}: the surface grid is not rows x columns finite nodes"),
        Geometry::Fastener(pins) => {
            let (from, to) = (&pins.axis.from, &pins.axis.to);
            assert!(finite(&[from.x, from.y, from.z, to.x, to.y, to.z, pins.diameter]) && pins.diameter > 0.0 && from.distance(to) > 0.0, "case part {id} has a non-finite or empty pin axis");
            assert!(match &pins.pattern { Some(Pattern::Points { offsets }) => !offsets.is_empty() && offsets.iter().all(|o| finite(&[o.x, o.y, o.z])), Some(Pattern::Grid { nx, ny, sx, sy }) => *nx > 0 && *ny > 0 && finite(&[*sx, *sy]), _ => true }, "case part {id} has a degenerate pin pattern");
        }
        other => panic!("case part {id} has the unexpected geometry {other:?}"),
    }
    if let Some(overlay) = &part.appearance.deformation {
        assert!(!overlay.samples.is_empty() || !overlay.surface.is_empty(), "case part {id} has an empty mode overlay");
        assert!(overlay.samples.iter().all(|s| finite(&[s.at, s.delta.z])) && overlay.surface.iter().all(|s| finite(&[s.at.x, s.at.y, s.delta.z])) && overlay.frequency.as_ref().map_or(true, |q| q.value.is_finite() && q.value > 0.0), "case part {id} has a non-finite mode overlay");
        if let Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }) = &part.geometry {
            assert_eq!(overlay.surface.len(), grid.points.len(), "case part {id}: one overlay sample per surface node");
        }
    }
}

/// 🧊️ The laws of design cases in 3D: every case stands in the scene with finite, non-empty primitives (box, cylinder, wedge, prism, laminate, surface overlay and pin patterns), a bilingual label and accessible names, a utilisation binding on every opaque part when the case has results, and grips that each write the case's own number rows (`cases[id=…].numbers[id=…].value`, one typed `change-case-number` per release) or its placement, with a finite factor and limits; the body lists exactly its own grips and always the two placement moves.
pub fn assert_case_laws(case_ids: &[&str], scene: &StructuralScene, handles: &[Handle]) {
    for id in case_ids {
        let body = entity(scene, id);
        let parts: Vec<&SceneEntity> = scene.entities.iter().filter(|e| e.entity.id == *id || e.entity.id.starts_with(&format!("{id}:"))).collect();
        for part in &parts {
            assert_case_part_geometry(part);
            assert!(!part.accessible_name.en.is_empty() && !part.accessible_name.de.is_empty() && !part.accessible_detail.en.is_empty() && !part.accessible_detail.de.is_empty(), "case part {} lacks a bilingual name", part.entity.id);
            if body.utilisation.is_some() {
                assert!(part.utilisation.is_some() || part.appearance.opacity < 1.0, "case part {} is opaque but not bound to the utilisation of case {id}", part.entity.id);
            }
        }
        let own: Vec<&Handle> = handles.iter().filter(|h| h.entity.id == *id).collect();
        let ids: BTreeSet<&str> = own.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(ids, body.handles.iter().map(String::as_str).collect::<BTreeSet<_>>(), "case {id}: the body lists other grips than the handle projection");
        assert!(ids.contains(format!("{id}:move-x").as_str()) && ids.contains(format!("{id}:move-y").as_str()), "case {id} cannot be moved");
        let number = format!("cases[id={id}].numbers[id=");
        let placement = format!("placements[id={id}].");
        for handle in &own {
            assert!(!handle.writes.is_empty() && handle.anchor.x.is_finite() && handle.anchor.y.is_finite() && handle.anchor.z.is_finite(), "case grip {} has no write or a non-finite anchor", handle.id);
            assert!(handle.limits.is_some(), "case grip {} has no limits", handle.id);
            for write in &handle.writes {
                assert!((write.path.starts_with(&number) && write.path.ends_with("].value")) || write.path.starts_with(&placement), "case grip {} writes {} that binds no change-case-number or placement", handle.id, write.path);
                assert!(write.map.k.is_finite() && write.map.k != 0.0 && write.map.base.is_finite(), "case grip {} has a degenerate map", handle.id);
            }
        }
        assert!(scene.annotations.iter().any(|a| matches!(a, Annotation::Label { id: label, text, .. } if *label == format!("{id}:label") && !text.en.is_empty() && !text.de.is_empty())), "case {id} has no bilingual label");
    }
}

pub use super::case_laws::{assert_case_dimension_laws, assert_case_dimension_laws_with, assert_case_layout_clear};

/// 🏺️ Compares `actual` with the committed golden at `path`, ignoring trailing whitespace. The golden is written when `SEMIO_UPDATE_GOLDEN` is set (the explicit bless) or when it is missing or empty (the first run, which needs no environment); a first-run write still fails the test once so the new bytes get reviewed, and the next run passes.
pub fn assert_golden(path: impl AsRef<std::path::Path>, actual: &str) {
    let path = path.as_ref();
    let committed = std::fs::read_to_string(path).unwrap_or_default();
    let explicit = std::env::var_os("SEMIO_UPDATE_GOLDEN").is_some();
    if explicit || committed.trim().is_empty() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create golden directory");
        }
        std::fs::write(path, format!("{}\n", actual.trim_end())).expect("bless golden");
        assert!(explicit, "golden {} was empty and is now written; review it and rerun", path.display());
        return;
    }
    assert_eq!(actual.trim_end(), committed.trim_end(), "golden {} differs; review the change and rerun with SEMIO_UPDATE_GOLDEN=1", path.display());
}

/// 🔁️ Two projections of the same input are byte-identical in canonical JSON.
pub fn assert_deterministic(first: &StructuralScene, second: &StructuralScene) {
    assert_eq!(canonical_json(first), canonical_json(second));
}

/// 🔤️ The lower-cased words of `text` of four or more letters; a token with a subscript (`k_crit`, `M_Rd`) is a symbol and has none.
fn long_words(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || "(),;:/·".contains(c)).filter(|token| !token.contains('_')).flat_map(|token| token.split(|c: char| !c.is_alphabetic())).filter(|word| word.chars().count() >= 4).map(str::to_lowercase).collect()
}

/// 🌐️ Whether the German copy only repeats the English one while it contains words: symbols and the words both languages share may be repeated.
fn untranslated(copy: &LocalizedCopy) -> bool {
    const SAME_IN_GERMAN: &[&str] = &["moment", "detail", "index", "faktor", "modul", "radius", "start", "total", "kontakt", "block"];
    copy.de.trim().is_empty() || (copy.de == copy.en && long_words(&copy.en).iter().any(|word| !SAME_IN_GERMAN.contains(&word.as_str())))
}

/// 🔤️ Whether `unit` is exactly the canonical symbol of a unit of the shared vocabulary.
fn canonical_unit(unit: &str) -> bool {
    norm::UnitId::lookup(unit).is_some_and(|id| id.symbol() == unit)
}

/// 📐️ Whether a quantity kind may label an axis in `unit`: the kinds of the unit's physical family (stress is pressure, a rate may be a frequency), the dimensionless kind for families without a kind of their own; the families that mix several kinds accept every kind.
fn kind_fits_unit(kind: QuantityKind, unit: &str) -> bool {
    use norm::UnitFamily as F;
    use QuantityKind as K;
    let Some(id) = norm::UnitId::lookup(unit) else { return true };
    match id.family() {
        F::Dimensionless => kind == K::Dimensionless,
        F::Length => kind == K::Length,
        F::Area => kind == K::Area,
        F::Volume => kind == K::Volume,
        F::Mass => kind == K::Mass,
        F::Time => kind == K::Time,
        F::Temperature => kind == K::Temperature,
        F::Angle => kind == K::Angle,
        F::Velocity => kind == K::Velocity,
        F::Acceleration => kind == K::Acceleration,
        F::Force => kind == K::Force,
        F::LineLoad => matches!(kind, K::ForcePerLength | K::Stiffness),
        F::Pressure => matches!(kind, K::Pressure | K::Stress),
        F::Moment => kind == K::Moment,
        F::Energy => kind == K::Energy,
        F::VolumeFlow => kind == K::VentilationRate,
        F::HeatTransfer => kind == K::HeatTransferCoefficient,
        F::Power => kind == K::Power,
        F::Rate => matches!(kind, K::Frequency | K::Dimensionless),
        F::InverseLength => kind == K::Dimensionless,
        _ => true,
    }
}

/// 🔖️ Whether a diagram id has the shape `<kind>:<row>`: a lowercase kebab kind, one colon and a non-empty row without a further colon.
fn id_shaped(id: &str) -> bool {
    id.split_once(':').is_some_and(|(kind, row)| !kind.is_empty() && !row.is_empty() && !row.contains(':') && kind.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') && !row.chars().any(char::is_whitespace))
}

/// 📋️ Every law a diagram obeys beyond validation, as readable violations: the id has the shape `<kind>:<row>`; titles and axis labels are translated; units are canonical symbols of the shared vocabulary and fit the axis kind; every marker either names its check or is typed `reference:` and then names none.
pub fn diagram_law_violations(diagram: &Diagram2d) -> Vec<String> {
    let mut out = Vec::new();
    let id = &diagram.id;
    if !id_shaped(id) {
        out.push(format!("{id}: the id is not <kind>:<row>"));
    }
    if untranslated(&diagram.title) {
        out.push(format!("{id}: the German title is missing or repeats the English one"));
    }
    for axis in &diagram.axes {
        if untranslated(&axis.label) {
            out.push(format!("{id}: the German axis title '{}' is missing or repeats the English one", axis.label.en));
        }
        if !canonical_unit(&axis.unit) {
            out.push(format!("{id}: the axis '{}' unit '{}' is no canonical unit symbol", axis.label.en, axis.unit));
        } else if !kind_fits_unit(axis.quantity, &axis.unit) {
            out.push(format!("{id}: the axis '{}' kind {:?} does not fit the unit '{}'", axis.label.en, axis.quantity, axis.unit));
        }
    }
    for column in &diagram.table_twin.columns {
        if !canonical_unit(&column.unit) {
            out.push(format!("{id}: the table column '{}' unit '{}' is no canonical unit symbol", column.header.en, column.unit));
        }
    }
    for series in &diagram.series {
        if let Series::Section { strain, stress, .. } = series {
            for profile in [strain, stress].into_iter().flatten().filter(|p| !canonical_unit(&p.unit)) {
                out.push(format!("{id}: the section profile unit '{}' is no canonical unit symbol", profile.unit));
            }
        }
    }
    for marker in &diagram.markers {
        let mid = &marker.id;
        if marker.is_reference() {
            if marker.bound_check.is_some() {
                out.push(format!("{id}: the reference marker '{mid}' is tied to a check, so it is no reference"));
            }
        } else if marker.bound_check.is_none() {
            out.push(format!("{id}: the marker '{mid}' names no check and is not typed reference:<row>"));
        }
    }
    out
}

/// 📋️ Every diagram validates, carries a non-empty table twin and obeys [`diagram_law_violations`].
pub fn assert_diagram_laws(diagrams: &[Diagram2d]) {
    for diagram in diagrams {
        assert_eq!(diagram.validate(), Ok(()), "diagram {}", diagram.id);
        assert!(diagram.table_twin.rows() > 0, "diagram {} has no table twin", diagram.id);
        assert_eq!(crate::projection::description::german_diagram_leaks(diagram), Vec::<String>::new(), "German diagram texts read as English");
        assert_eq!(diagram_law_violations(diagram), Vec::<String>::new(), "diagram {} breaks the diagram laws", diagram.id);
    }
}

/// 🧭️ The basis law: the scene and every diagram show the badge of `family` under the annex the document selects, and the badge changes with the annex. `project` gives the scene and the diagrams of the example document under the annex.
pub fn assert_basis_follows_annex(family: norm::NormFamilyId, project: impl Fn(norm::AnnexChoice) -> (StructuralScene, Vec<Diagram2d>)) {
    use crate::projection::{basis_title, Basis};
    let mut badges = Vec::new();
    for annex in [norm::AnnexChoice::En, norm::AnnexChoice::De] {
        let badge = Basis::new(family, annex).badge();
        let (scene, diagrams) = project(annex);
        assert_eq!(basis_title(&scene), Some(&badge), "the scene shows the basis of the annex {annex:?}");
        assert!(!diagrams.is_empty(), "the example has diagrams");
        for diagram in &diagrams {
            assert_eq!(diagram.basis(), Some(&badge), "diagram {} shows the basis of the annex {annex:?}", diagram.id);
        }
        badges.push(badge);
    }
    assert_ne!(badges[0], badges[1], "the badge changes with the annex");
}

/// 🧩️ One projection obeys every law at once: scene laws, diagram laws with at least one axis and a unit on each, and handles that are finite, bilingual and write at least one field.
pub fn assert_projection_laws(scene: &StructuralScene, diagrams: &[Diagram2d], handles: &[Handle]) {
    assert_scene_laws(scene);
    assert_diagram_laws(diagrams);
    for diagram in diagrams {
        assert!(!diagram.axes.is_empty(), "diagram {} has no axes", diagram.id);
        assert!(diagram.axes.iter().all(|a| !a.unit.trim().is_empty()), "diagram {} has an axis without a unit", diagram.id);
    }
    assert_handle_laws(handles);
    for handle in handles {
        assert!(handle.anchor.x.is_finite() && handle.anchor.y.is_finite() && handle.anchor.z.is_finite(), "handle {} has a non-finite anchor", handle.id);
        assert!(!handle.writes.is_empty(), "handle {} writes no field", handle.id);
    }
}

/// 🎞️ The combination-track law: a document with at least two combinations yields exactly one `Combination` track whose keyframes are the combinations `1..=n` in order, and a document with fewer yields none.
pub fn assert_combination_track_law(scene: &StructuralScene, combinations: usize) {
    let tracks: Vec<_> = scene.tracks.iter().filter(|track| matches!(track.kind, TrackKind::Combination)).collect();
    if combinations < 2 {
        assert!(tracks.is_empty(), "a document with {combinations} combinations offers no combination track");
        return;
    }
    assert_eq!(tracks.len(), 1, "a document with {combinations} combinations yields exactly one combination track");
    let track = tracks[0];
    assert_eq!(track.keyframes.len(), combinations, "one keyframe per combination");
    assert!(track.keyframes.iter().enumerate().all(|(i, key)| key.at == (i + 1) as f64 && !key.caption.en.is_empty() && !key.caption.de.is_empty()), "keyframes at 1..=n with bilingual captions");
    assert_eq!(track.validate(), Ok(()));
}

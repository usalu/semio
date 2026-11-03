//! 🌬️ Wind: the c_pe zones A–J of EN 1991-1-4 as pressure zones on the walls and the roof, the net pressure arrows of both internal-pressure cases c_pi = +0.2 and −0.3 per zone,
//! and the peak velocity pressure profile q_p(z) beside the building.

use super::envelope::{world, RoofShape};
use super::kit::*;
use super::model::En1991Snapshot;
use super::scene::*;
use super::{checks, handles, Ctx};
use checks::part_1_4;

const PROFILE_SAMPLES: usize = 12;
const PROFILE_OFFSET: f64 = 4.0;

/// 🌬️ Peak velocity pressure q_p(z) in Pa for the site of the document, `None` for an inadmissible wind zone or terrain category.
pub fn qp_at(d: &En1991Snapshot, z: f64) -> Option<f64> {
    let vb = part_1_4::design_vb(d.annex, d.wind_zone, d.en_vb)?;
    part_1_4::TerrainCategory::from_u8(d.terrain_category)?;
    part_1_4::peak_velocity_pressure_pa(d.annex, d.wind_zone, d.terrain_category, z, vb, d.air_density, d.orography_factor, d.mixed_terrain_upwind, d.mixed_terrain_distance, d.coast_or_island)
}

/// 🌬️ The tabulated (c_pe,10, c_pe,1) alternatives of a zone letter for the building and its first roof.
pub fn cpe_candidates(d: &En1991Snapshot, letter: char) -> Option<Vec<(f64, f64)>> {
    let roof = d.roofs.first();
    if roof.is_some_and(|r| part_1_4::is_hipped_roof(&r.roof_type)) && part_1_4::is_hipped_zone(&letter.to_string()) {
        return part_1_4::hipped_cpe(d.annex, &letter.to_string()).map(|pair| vec![pair]);
    }
    let parapet_ratio = roof.filter(|r| r.has_parapet).map_or(0.0, |r| r.parapet_height / d.height.max(0.1));
    part_1_4::face_cpe_candidates(&letter.to_string(), d.height / d.depth.max(0.1), roof.map(|r| r.roof_type.as_str()), roof.map_or(0.0, |r| r.pitch_deg), parapet_ratio)
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    if !(out.wants(global::WIND_ZONES) || out.wants(global::WIND_QP) || d.wind_faces.iter().any(|f| out.wants(&f.id))) {
        return;
    }
    let Some(qp_reference) = qp_at(d, d.height) else { return };
    let shape = RoofShape::of(d, d.roofs.first());
    let pose = &layout.building;
    let building = entity_ref(EntityKind::Solid, "building", "@site");
    let ze_default = part_1_4::reference_height_ze(d.height, d.height, d.width);
    for zone in shape.wind_zones() {
        let letter = zone.letter;
        let faces: Vec<&super::model::WindFace> = d.wind_faces.iter().filter(|f| f.zone.eq_ignore_ascii_case(&letter.to_string())).collect();
        let face = faces.first().copied();
        if !out.own(face.map_or(global::WIND_ZONES, |f| f.id.as_str())) {
            continue;
        }
        let Some(candidates) = cpe_candidates(d, letter) else {
            if d.roofs.first().is_some_and(|r| part_1_4::is_hipped_roof(&r.roof_type)) && part_1_4::is_hipped_zone(&letter.to_string()) {
                let ids: Vec<&str> = faces.iter().map(|f| f.id.as_str()).collect();
                let (bound, collection) = (binding_of(report, &ids), face.map_or("@site".to_string(), |f| format!("@windFaces[id={}]", f.id)));
                for (index, part) in zone.parts.iter().enumerate() {
                    let points = world(pose, &part.points);
                    let id = if index == 0 { format!("wind-zone-{letter}") } else { format!("wind-zone-{letter}:part-{index}") };
                    out.push(
                        Spec::new(EntityKind::Surface, id, collection.clone(), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&points) } }), MaterialRole::Neutral, bi(|de| format!("{} {letter}", if de { "Winddruckzone Walmdach" } else { "Hipped-roof wind pressure zone" })), LAYER_WIND)
                            .bound(bound.clone())
                            .opacity(0.35)
                            .detail(bi(|de| format!("{letter}: {}", if de { "Werte ausstehend (Tabelle 7.5 nicht verfügbar, Zonengeometrie nach Bild 7.9 ungeprüft)" } else { "awaiting values (Table 7.5 not available, zone geometry after Figure 7.9 unverified)" }))),
                        &points,
                    );
                }
            }
            continue;
        };
        let Some(&(c10, _)) = candidates.first() else { continue };
        let ze = face.map_or(ze_default, |f| part_1_4::reference_height_ze(f.z, d.height, d.width));
        let Some(qp) = qp_at(d, ze) else { continue };
        let factor = face.map_or(1.0, |f| f.c_s * f.c_d);
        let ids: Vec<&str> = faces.iter().map(|f| f.id.as_str()).collect();
        let bound = binding_of(report, &ids);
        let collection = face.map_or("@site".to_string(), |f| format!("@windFaces[id={}]", f.id));
        let id = format!("wind-zone-{letter}");
        let grips = handles::ids(d, layout, &id);
        let parts: Vec<Vec<Vec3>> = zone.parts.iter().map(|p| world(pose, &p.points)).collect();
        let label = bi(|de| {
            let values: Vec<String> = candidates.iter().map(|c| num(c.0, 2, de)).collect();
            let hipped = d.roofs.first().is_some_and(|r| part_1_4::is_hipped_roof(&r.roof_type)) && part_1_4::is_hipped_zone(&letter.to_string());
            let note = if hipped { if de { " (Zonenlage ungeprüft)" } else { " (unverified layout)" } } else { "" };
            format!("{letter}: c_pe,10 = {}{note}", values.join(" / "))
        });
        let zones: Vec<PressureZone> = parts.iter().map(|points| PressureZone { polygon: polygon3(points), value: pressure(qp * c10), label: label.clone() }).collect();
        let all: Vec<Vec3> = parts.concat();
        out.push(
            Spec::new(EntityKind::Load, id.clone(), collection.clone(), Geometry::Load(LoadGlyph::Pressure { face: building.clone(), zones, case: case("wind-theta-0", LoadKind::Wind) }), load(LoadKind::Wind), bi(|de| format!("{} {letter}", if de { "Winddruckzone" } else { "Wind pressure zone" })), LAYER_WIND)
                .bound(bound.clone())
                .channel(super::fields::pressure::channel(qp, c10, factor))
                .opacity(0.85)
                .detail(bi(|de| format!("{}, q_p(z_e = {} m) = {} kN/m²", label_text(&candidates, letter, de), num(ze, 2, de), num(qp / 1000.0, 3, de))))
                .handles(grips.clone()),
            &all,
        );
        let at = centroid(&parts[0]);
        out.label(format!("{id}:label"), at.clone(), label.clone(), tone_of(&bound));
        if let Some(clause) = bound.as_ref().and_then(|b| clause_of(report, &b.check_id)) {
            out.annotate(Annotation::Clause { id: format!("{id}:clause"), anchor: entity_ref(EntityKind::Load, &id, &collection), clause }, &at);
        }
        for c_pi in part_1_4::c_pi_cases() {
            let w = part_1_4::wind_pressure_pa(qp, c10, c_pi, 1.0, 1.0) * factor;
            if w.abs() < 1e-9 {
                continue;
            }
            let normal = pose.dir(&zone.parts[0].normal);
            let direction = if w > 0.0 { mul(&normal, -1.0) } else { normal };
            let sign = format!("{c_pi:+.1}");
            out.push(
                Spec::new(
                    EntityKind::Load,
                    format!("{id}:pi{sign}"),
                    collection.clone(),
                    Geometry::Load(LoadGlyph::Area { polygon: polygon3(&parts[0]), direction, intensity: pressure(w.abs()), case: case(&format!("wind-pi{sign}"), LoadKind::Wind) }),
                    load(LoadKind::Wind),
                    bi(|de| format!("Wind {letter}, c_pi = {}", num(c_pi, 1, de))),
                    LAYER_WIND,
                )
                .bound(bound.clone())
                .channel(ColourChannel::Role)
                .opacity(0.7)
                .detail(bi(|de| format!("w = q_p (c_pe,10 − c_pi) c_s c_d = {} kN/m² ({})", num(w / 1000.0, 3, de), match (w > 0.0, de) { (true, true) => "Druck", (true, false) => "pressure", (false, true) => "Sog", (false, false) => "suction" })))
                .handles(grips.clone()),
                &parts[0],
            );
        }
    }
    if !out.own(global::WIND_QP) {
        return;
    }
    let top = d.height + shape.rise();
    let base = pose.at(-PROFILE_OFFSET, d.width / 2.0, 0.0);
    let tip = pose.at(-PROFILE_OFFSET, d.width / 2.0, top);
    let samples: Vec<ProfileSample> = (0..=PROFILE_SAMPLES).map(|i| ProfileSample { at: i as f64 / PROFILE_SAMPLES as f64, value: pressure(qp_at(d, top * i as f64 / PROFILE_SAMPLES as f64).unwrap_or(0.0)) }).collect();
    let face_ids: Vec<&str> = d.wind_faces.iter().map(|f| f.id.as_str()).collect();
    out.push(
        Spec::new(
            EntityKind::Load,
            "wind-qp",
            "@site",
            Geometry::Load(LoadGlyph::Line { path: polyline(&[base.clone(), tip.clone()]), direction: pose.dir(&Vec3::X), profile: LoadProfile::Sampled { samples }, case: case("wind-qp", LoadKind::Wind) }),
            load(LoadKind::Wind),
            loc("Peak velocity pressure profile q_p(z)", "Böengeschwindigkeitsdruck-Profil q_p(z)"),
            LAYER_WIND,
        )
        .bound(binding_of(report, &face_ids))
        .detail(bi(|de| format!("q_p({} m) = {} kN/m²", num(d.height, 1, de), num(qp_reference / 1000.0, 3, de))))
        .handles(handles::ids(d, layout, "wind-qp")),
        &[base, tip.clone()],
    );
    out.label("wind-qp:label".into(), tip, bi(|de| format!("q_p(h) = {} kN/m²", num(qp_reference / 1000.0, 2, de))), Tone::Info);
}

fn label_text(candidates: &[(f64, f64)], letter: char, de: bool) -> String {
    let c10: Vec<String> = candidates.iter().map(|c| num(c.0, 2, de)).collect();
    let c1: Vec<String> = candidates.iter().map(|c| num(c.1, 2, de)).collect();
    format!("{letter}: c_pe,10 = {}, c_pe,1 = {}", c10.join(" / "), c1.join(" / "))
}

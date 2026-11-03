//! 🎞️ Tracks of the EN 1994 scene: the buckling-mode index over the composite columns (flexural buckling about the weak axis of the effective stiffness) and the unpropped girders (lateral-torsional buckling of the bare steel section); the mode entities exist only when a document offers at least two modes.

use super::beams::{local_frame, steel_shape};
use super::columns::steel_section;
use super::facts::{BeamFacts, ColumnFacts};
use super::kit::*;
use super::layout::Layout;
use super::mechanics as mech;
use super::scene::*;
use super::SceneInput;


pub(crate) const MODES: [(&str, &str, &str); 2] = [(LAYER_MODE_BUCKLING, "Column buckling", "Stützenknicken"), (LAYER_MODE_LTB, "Lateral-torsional buckling", "Biegedrillknicken")];

const STEEL_E: f64 = 210.0e9;
const STEEL_G: f64 = 81.0e9;
const AMPLITUDE: f64 = 0.04;

/// 🌀️ The critical load of a pinned composite column, the Euler load of its effective stiffness (EN 1994-1-1, 6.7.3.3), with the first eigenform.
pub(crate) fn column_buckling(f: &ColumnFacts) -> Option<(f64, Vec<(f64, [f64; 3])>)> {
    let c = &f.column;
    let ei = super::checks::part_column::ei_eff_nmm2(c, f.annex, super::checks::catalogue::ecm_pa(c.concrete_f_ck_pa));
    mech::member_modes::buckling(&mech::member_modes::Props { length: c.length_m.max(0.1), e: ei, g: 1.0, a: 1.0, iy: 1.0, iz: 1.0, j: 1.0 }, mech::member_modes::Ends::Pinned).ok()
}

/// 🦋️ The critical moment of the bare steel girder under uniform moment between fork ends, braced every `ltb_length_m`, with the first eigenform as `(at, lateral)`.
pub(crate) fn girder_ltb(f: &BeamFacts) -> Option<(f64, Vec<(f64, f64)>)> {
    let (s, l) = (&f.steel, f.l_total);
    let (h, b, tw, tf) = (s.height_m, s.width_m, s.tw_m, s.tf_m);
    let iz = 2.0 * tf * b.powi(3) / 12.0 + (h - 2.0 * tf) * tw.powi(3) / 12.0;
    let spacing = f.beam.ltb_length_m;
    let restraints = if spacing > 1e-6 && spacing < l - 1e-6 { (1..).map(|k| k as f64 * spacing).take_while(|x| *x < l - 1e-6).map(|x| mech::LtbRestraint { x, lateral: true, twist: true }).collect() } else { Vec::new() };
    let member = mech::LtbMember {
        length: l,
        e: STEEL_E,
        g: STEEL_G,
        iz,
        iw: iz * (h - tf).powi(2) / 4.0,
        j: (2.0 * b * tf.powi(3) + (h - 2.0 * tf) * tw.powi(3)) / 3.0,
        polar_radius_sq: (s.i_y_m4 + iz) / s.a_m2.max(1e-9),
        end_i: mech::ltb::fork_end(),
        end_j: mech::ltb::fork_end(),
        restraints,
        moment: mech::ltb::uniform_moment(1.0, l),
        elements: 32,
        ..Default::default()
    };
    let result = mech::ltb::ltb_analyze(&member, &mech::Control::none()).ok()?;
    let last = result.shape.len().saturating_sub(1);
    let shape = result.shape.iter().enumerate().filter(|(i, _)| i % 4 == 0 || *i == last).map(|(_, p)| (p.x / l, p.lateral + p.twist * h / 2.0)).collect();
    Some((result.mcr, shape))
}

fn overlay(index: u32, load_factor: Option<f64>, length: f64, samples: Vec<DisplacementSample>) -> DeformationOverlay {
    DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: load_factor.filter(|a| a.is_finite() && *a > 0.0), kind: DeformationKind::Mode, index, frequency: None, participation: 1.0, scale: AMPLITUDE * length, samples }
}

struct Candidate {
    spec: Spec,
    points: [Vec3; 2],
}

fn candidate(layer_index: usize, id: &str, collection: &'static str, label: (&str, &str), geometry: Geometry, role: MaterialRole, shape: DeformationOverlay, points: [Vec3; 2]) -> Candidate {
    let (layer, en, de) = MODES[layer_index];
    let name = loc(&format!("{} {id}: {en}", label.0), &format!("{} {id}: {de}", label.1));
    let mut spec = Spec::new(EntityKind::Member, format!("{id}:{layer}"), collection, geometry, role, name, layer);
    spec.deformation = Some(shape);
    Candidate { spec, points }
}

fn member(frame: &Frame, end: &Vec3, section: SectionShape, end_release: EndRelease) -> Geometry {
    Geometry::Member(MemberGeometry { axis: polyline(&[frame.origin.clone(), end.clone()]), frame: frame.clone(), section, section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release, stations: Vec::new() })
}

fn column_mode(d: &super::model::En1994Snapshot, layout: &Layout, column: &super::model::CompositeColumn) -> Option<Candidate> {
    let pose = layout.pose(&column.id)?;
    let f = ColumnFacts::of(d, column);
    let (ncr, curve) = column_buckling(&f)?;
    let frame = local_frame(pose, (0.0, 0.0, 0.0), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0));
    let end = pose.point(0.0, 0.0, column.length_m);
    let samples = curve.into_iter().map(|(at, u)| DisplacementSample { at, delta: add(&add(&mul(&frame.x, u[0]), &mul(&frame.y, u[1])), &mul(&frame.z, u[2])) }).collect();
    let shape = overlay(1, Some(ncr / f.n_ed.abs()).filter(|_| f.n_ed.abs() > 1.0), column.length_m, samples);
    let geometry = member(&frame, &end, steel_section(&f.shape), EndRelease::Pinned);
    Some(candidate(0, &column.id, "columns", ("Column", "Stütze"), geometry, MaterialRole::Steel, shape, [frame.origin.clone(), end]))
}

fn beam_mode(d: &super::model::En1994Snapshot, layout: &Layout, beam: &super::model::CompositeBeam) -> Option<Candidate> {
    let pose = layout.pose(&beam.id)?;
    let f = BeamFacts::of(d, beam);
    if f.l_total <= 1e-6 || f.steel.height_m <= 1e-6 {
        return None;
    }
    let (mcr, curve) = girder_ltb(&f)?;
    let frame = local_frame(pose, (0.0, 0.0, -f.steel.height_m / 2.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
    let end = pose.point(f.l_total, 0.0, -f.steel.height_m / 2.0);
    let samples = curve.into_iter().map(|(at, lateral)| DisplacementSample { at, delta: mul(&frame.y, lateral) }).collect();
    let shape = overlay(2, Some(mcr / f.m_ed.abs()).filter(|_| f.m_ed.abs() > 1.0), f.l_total, samples);
    let geometry = member(&frame, &end, steel_shape(&f.steel), EndRelease::Rigid);
    Some(candidate(1, &beam.id, "beams", ("Girder", "Träger"), geometry, MaterialRole::Steel, shape, [frame.origin.clone(), end]))
}

fn columns_of(d: &super::model::En1994Snapshot) -> impl Iterator<Item = &super::model::CompositeColumn> {
    d.columns.iter().filter(|c| c.length_m > 1e-6 && c.outer_size_m > 1e-6)
}

fn beams_of(d: &super::model::En1994Snapshot) -> impl Iterator<Item = &super::model::CompositeBeam> {
    d.beams.iter().filter(|b| b.construction == semio_s_artifact_norm_en1994_model::ConstructionMethod::Unpropped)
}

/// 🌀️ Whether the document offers both a column and a girder mode; the mode entities and their track exist only then, so every column and beam fingerprint carries this flag.
pub(crate) fn offers_modes(d: &super::model::En1994Snapshot, layout: &Layout) -> bool {
    columns_of(d).any(|c| column_mode(d, layout, c).is_some()) && beams_of(d).any(|b| beam_mode(d, layout, b).is_some())
}

/// 🎞️ Adds the mode entities of the wanted columns and girders when the document offers both kinds of mode.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    let columns: Vec<_> = columns_of(d).filter(|c| out.wants(&c.id)).collect();
    let beams: Vec<_> = beams_of(d).filter(|b| out.wants(&b.id)).collect();
    if (columns.is_empty() && beams.is_empty()) || !offers_modes(d, layout) {
        return;
    }
    for column in columns {
        if let Some(c) = column_mode(d, layout, column) {
            out.push(c.spec, &c.points);
        }
    }
    for beam in beams {
        if let Some(c) = beam_mode(d, layout, beam) {
            out.push(c.spec, &c.points);
        }
    }
}

/// 🎞️ The mode track over the mode layers present in the merged scene.
pub(crate) fn finish(scene: &mut StructuralScene) {
    scene.tracks.extend(projection_kit::tracks::mode_track(&scene.entities, &MODES));
    scene.tracks.extend(super::fields::fire_track(&scene.entities));
}

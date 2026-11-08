//! 🏷️ Overlays: dimensions, labels, leaders, clause badges, failure glyphs, fail/stale markers and the measure result as one annotation layer.

use super::config::{LabelDensity, LengthUnit, Viewport3dWindowConfig};
use super::measure::MeasureState;
use super::parts::entity_anchor;
use super::pose::*;
use super::scene::{Annotation, DeformationKind, DeformationOverlay, FailureMode, Quantity, QuantityKind, StructuralScene, Tone};
use semio_plugin::{World3dAnnotation, World3dAnnotationLayer, World3dMarker, World3dMarkerShape, World3dText, World3dTone, WORLD3D_ANNOTATIONS_MAX};

fn trimmed(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    if text.contains('.') { text.trim_end_matches('0').trim_end_matches('.').to_string() } else { text }
}

pub fn number(value: f64, german: bool, decimals: usize) -> String {
    let text = trimmed(value, decimals);
    if german { text.replace('.', ",") } else { text }
}

fn with_unit(value: f64, unit: &str, decimals: usize) -> World3dText {
    World3dText::new(format!("{} {unit}", number(value, false, decimals)), format!("{} {unit}", number(value, true, decimals)))
}

/// 📏️ Presentation scale, unit symbol and decimals of a quantity kind (length by the config unit, force in kN, stress and pressure in MPa, angle in degrees).
pub fn presentation(kind: QuantityKind, unit: LengthUnit) -> (f64, &'static str, usize) {
    match kind {
        QuantityKind::Length => match unit {
            LengthUnit::Metre => (1.0, "m", 3),
            LengthUnit::Millimetre => (1000.0, "mm", 1),
        },
        QuantityKind::Force => (1.0e-3, "kN", 2),
        QuantityKind::Moment => (1.0e-3, "kNm", 2),
        QuantityKind::ForcePerLength => (1.0e-3, "kN/m", 2),
        QuantityKind::Pressure | QuantityKind::Stress => (1.0e-6, "MPa", 2),
        QuantityKind::Angle => (180.0 / std::f64::consts::PI, "°", 1),
        QuantityKind::Area => (1.0, "m²", 4),
        QuantityKind::Volume => (1.0, "m³", 4),
        QuantityKind::Temperature => (1.0, "K", 1),
        QuantityKind::Mass => (1.0, "kg", 1),
        QuantityKind::Time => (1.0, "s", 3),
        QuantityKind::Energy => (1.0, "J", 1),
        QuantityKind::Acceleration => (1.0, "m/s²", 3),
        QuantityKind::Velocity => (1.0, "m/s", 3),
        QuantityKind::Frequency => (1.0, "Hz", 3),
        QuantityKind::Stiffness => (1.0e-3, "kN/m", 2),
        QuantityKind::Dimensionless => (1.0, "", 3),
    }
}

/// 🔢️ Locale-aware text of a quantity in presentation units.
pub fn quantity_text(q: &Quantity, unit: LengthUnit) -> World3dText {
    let (scale, symbol, decimals) = presentation(q.kind, unit);
    with_unit(q.value * scale, symbol, decimals)
}

/// 🌊️ Mode and buckling label of a deformation overlay: mode index, frequency, load factor alpha_cr and the drawn amplitude.
pub fn mode_text(o: &DeformationOverlay, amplitude: f64) -> World3dText {
    let part = |german: bool| {
        let mut items = vec![format!("{} {}", if german { "Eigenform" } else { "Mode" }, o.index)];
        if let Some(f) = &o.frequency {
            let text = quantity_text(f, LengthUnit::Metre);
            items.push(format!("f = {}", if german { text.de } else { text.en }));
        }
        if let Some(a) = o.load_factor { items.push(format!("αcr = {}", number(a, german, 2))); }
        items.push(format!("×{}", number(o.scale * amplitude, german, 1)));
        items.join(" · ")
    };
    World3dText::new(part(false), part(true))
}

pub fn failure_text(mode: FailureMode) -> World3dText {
    let (en, de) = match mode {
        FailureMode::LateralTorsionalBuckling => ("Lateral-torsional buckling", "Biegedrillknicken"),
        FailureMode::FlexuralBuckling => ("Flexural buckling", "Biegeknicken"),
        FailureMode::Yielding => ("Yielding", "Fließen"),
        FailureMode::Crushing => ("Crushing", "Zerdrücken"),
        FailureMode::Punching => ("Punching", "Durchstanzen"),
        FailureMode::SlipCircle => ("Slip circle", "Gleitkreis"),
        FailureMode::ShearCone => ("Shear cone", "Scherkegel"),
        FailureMode::BlockTearOut => ("Block tear-out", "Blockversagen"),
        FailureMode::Overturning => ("Overturning", "Kippen"),
        FailureMode::Sliding => ("Sliding", "Gleiten"),
        FailureMode::Fatigue => ("Fatigue", "Ermüdung"),
        FailureMode::Other => ("Failure mode", "Versagensform"),
    };
    World3dText::new(en, de)
}

fn tone(t: Tone) -> World3dTone {
    match t {
        Tone::Neutral => World3dTone::Neutral,
        Tone::Info => World3dTone::Info,
        Tone::Warning => World3dTone::Warning,
        Tone::Error => World3dTone::Danger,
    }
}

fn marker(id: String, at: V3, shape: World3dMarkerShape, text: World3dText, tone: World3dTone) -> World3dAnnotation {
    World3dAnnotation::Marker(World3dMarker { id, position: at, shape, axis: None, text, tone })
}

fn anchor_of(scene: &StructuralScene, id: &str) -> V3 {
    scene.entities.iter().find(|e| e.entity.id == id).map_or([0.0; 3], |e| entity_anchor(&e.geometry))
}

/// 🏷️ Annotation layer for a scene under the effective label density; fail, warning and stale markers are drawn at every density except in a hidden layer.
pub fn build(scene: &StructuralScene, config: &Viewport3dWindowConfig, density: LabelDensity, visible: &dyn Fn(&str) -> bool, measure: &MeasureState) -> Option<World3dAnnotationLayer> {
    let rank = density.rank();
    let mut items: Vec<World3dAnnotation> = Vec::new();
    for a in &scene.annotations {
        match a {
            Annotation::Dimension { id, from, to, offset, value, .. } if rank >= 1 && config.show_dimensions => items.push(World3dAnnotation::dimension(id.clone(), v3(from), v3(to), v3(offset), quantity_text(value, config.unit))),
            Annotation::Label { id, anchor, text, tone: t } if rank >= 2 => items.push(marker(id.clone(), v3(anchor), World3dMarkerShape::Dot, World3dText::new(text.en.clone(), text.de.clone()), tone(*t))),
            Annotation::Leader { id, anchor, text, offset_px } if rank >= 2 => items.push(World3dAnnotation::leader(id.clone(), v3(anchor), [f64::from(offset_px.first().copied().unwrap_or(24)), f64::from(offset_px.get(1).copied().unwrap_or(-24))], World3dText::new(text.en.clone(), text.de.clone()))),
            Annotation::Clause { id, anchor, clause } if rank >= 3 => items.push(marker(id.clone(), anchor_of(scene, &anchor.id), World3dMarkerShape::Ring, World3dText::new(format!("{} {} §{}", clause.family, clause.part, clause.section), format!("{} {} §{}", clause.family, clause.part, clause.section)), World3dTone::Info)),
            Annotation::FailureGlyph { id, at, mode, extent } => {
                let position = extent.as_ref().filter(|e| !e.outer.points.is_empty()).map_or_else(|| anchor_of(scene, &at.id), |e| mul(e.outer.points.iter().fold([0.0; 3], |s, p| add(s, v3(p))), 1.0 / e.outer.points.len() as f64));
                items.push(marker(id.clone(), position, World3dMarkerShape::Cross, failure_text(*mode), World3dTone::Danger));
            }
            _ => {}
        }
    }
    for e in scene.entities.iter().filter(|e| visible(&e.layer)) {
        if let Some(o) = e.appearance.deformation.as_ref().filter(|o| rank >= 1 && (matches!(o.kind, DeformationKind::Mode) || o.load_factor.is_some())) {
            items.push(marker(format!("mode:{}", e.entity.id), entity_anchor(&e.geometry), World3dMarkerShape::Ring, mode_text(o, config.deformation_scale), World3dTone::Info));
        }
        let Some(u) = &e.utilisation else { continue };
        let at = entity_anchor(&e.geometry);
        let ratio = (number(u.ratio, false, 2), number(u.ratio, true, 2));
        match u.status {
            super::scene::CheckStatus::Fail => items.push(marker(format!("status:{}", e.entity.id), at, World3dMarkerShape::Cross, World3dText::new(format!("Fails {}", ratio.0), format!("Versagt {}", ratio.1)), World3dTone::Danger)),
            super::scene::CheckStatus::Warning => items.push(marker(format!("status:{}", e.entity.id), at, World3dMarkerShape::Cross, World3dText::new(format!("Near limit {}", ratio.0), format!("Nahe Grenze {}", ratio.1)), World3dTone::Warning)),
            _ => {}
        }
        if u.stale {
            items.push(marker(format!("stale:{}", e.entity.id), at, World3dMarkerShape::Ring, World3dText::new("Evaluation pending", "Berechnung läuft"), World3dTone::Info));
        }
        if rank >= 3 {
            if let Some(l) = &e.label {
                items.push(marker(format!("label:{}", e.entity.id), at, World3dMarkerShape::Dot, World3dText::new(l.en.clone(), l.de.clone()), World3dTone::Neutral));
            }
        }
    }
    if let Some(m) = measure.measurement() {
        let (a, b) = (measure.points[0], measure.points[1]);
        let d = quantity_text(&Quantity::new(QuantityKind::Length, m.distance), config.unit);
        items.push(World3dAnnotation::dimension("measure:distance", a, b, [0.0; 3], World3dText::new(format!("Distance {}", d.en), format!("Abstand {}", d.de))).with_tone(World3dTone::Primary));
        if let (Some(angle), Some(c)) = (m.angle_deg, measure.points.get(2)) {
            items.push(World3dAnnotation::angle("measure:angle", b, sub(a, b), sub(*c, b), World3dText::new(format!("Angle {}°", number(angle, false, 1)), format!("Winkel {}°", number(angle, true, 1)))).with_tone(World3dTone::Primary));
        }
    }
    let basis = super::scene::projection::basis_title(scene);
    items.truncate(WORLD3D_ANNOTATIONS_MAX - usize::from(basis.is_some()));
    if let Some(basis) = basis {
        items.push(marker(super::scene::BASIS_NOTE_ID.into(), v3(&scene.bounds.min), World3dMarkerShape::Ring, World3dText::new(basis.en.clone(), basis.de.clone()), World3dTone::Neutral));
    }
    (!items.is_empty()).then(|| World3dAnnotationLayer::new(items).titled(World3dText::new("Annotations", "Beschriftungen")))
}

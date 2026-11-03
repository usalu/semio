//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 📏️ Span and pre-camber for members, the magnitude of every action (the grip height is the magnitude over a scale of N per metre) and the combination selector, whose write path `view.activeCombination` is view state and not a document leaf.

use super::actions::{affected, lift, present_groups, rows, selector_anchor, GROUP_LANE};
use super::analyses;
use super::kit::{along, base_id, entity_ref, loc, member_depth, mul, v3, COMBINATION_ENTITY};
use super::layout::{Layout, Placement};
use super::members::exaggeration;
use super::model::{self, En1990Snapshot};
use super::scene::*;
use super::scene::projection_kit::handle_kit::{choice_handle, grid, handle, HandleSpec as Spec, field_binding as binding};

#[path = "🎚️gauges/🦀️.rs"]
pub(crate) mod gauges;

fn spread_handle(d: &En1990Snapshot, bar: &analyses::Bar, entity: &EntityRef) -> Option<Handle> {
    let tag = bar.spread_tag()?;
    let analysis = d.reliability_analyses.iter().find(|a| a.id == bar.owner)?;
    let spread = match bar.suffix { Some("action2") => analysis.second_action_sd, Some(_) => analysis.action_sd, None => analysis.resistance_sd };
    Some(handle(Spec {
        entity,
        tag,
        kind: HandleKind::ScalarAlong { axis: Vec3::Z },
        anchor: v3(bar.x, bar.y, bar.height() + spread / bar.per_metre),
        grip: GripStyle::Diamond,
        writes: vec![binding(format!("reliabilityAnalyses[id={}].{tag}", bar.owner), Affine { base: 0.0, k: bar.per_metre }, "N", Some(Quantise::Round { step: bar.step }))],
        snap: grid(0.05),
        limits: (0.01, 6.0),
        name: loc(&format!("Standard deviation of {}", bar.name.en), &format!("Standardabweichung von {}", bar.name.de)),
    }))
}

fn analysis_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    analyses::bars(d, layout)
        .into_iter()
        .filter(|bar| bar.owner == id)
        .flat_map(|bar| {
            let entity = entity_ref(EntityKind::Plate, id, bar.collection);
            let spread = spread_handle(d, &bar, &entity);
            let mean = bar.field.map(|field| analysis_handle(&bar, &entity, id, field));
            mean.into_iter().chain(spread).chain(fatigue_handles(d, &bar, &entity))
        })
        .collect()
}

fn fatigue_handles(d: &En1990Snapshot, bar: &analyses::Bar, entity: &EntityRef) -> Vec<Handle> {
    let Some(bin) = d.fatigue_spectra.iter().find(|b| b.id == bar.owner) else { return Vec::new() };
    let name = |en: &str, de: &str| loc(&format!("{en} of {}", bar.name.en), &format!("{de} von {}", bar.name.de));
    vec![
        handle(Spec {
            entity,
            tag: "detailCategory",
            kind: HandleKind::ScalarAlong { axis: Vec3::Z },
            anchor: v3(bar.x, bar.y, bin.detail_category / bar.per_metre),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("fatigueSpectra[id={}].detailCategory", bin.id), Affine { base: 0.0, k: bar.per_metre }, "Pa", Some(Quantise::Round { step: bar.step }))],
            snap: grid(0.05),
            limits: (0.02, 12.0),
            name: name("Detail category Δσ_C", "Kerbfall Δσ_C"),
        }),
        handle(Spec {
            entity,
            tag: "cycles",
            kind: HandleKind::ScalarAlong { axis: Vec3::Y },
            anchor: v3(bar.x, bar.y + bin.cycles / analyses::CYCLES_PER_METRE, 0.0),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("fatigueSpectra[id={}].cycles", bin.id), Affine { base: 0.0, k: analyses::CYCLES_PER_METRE }, "1", Some(Quantise::Round { step: 1000.0 }))],
            snap: grid(0.05),
            limits: (0.0, 12.0),
            name: name("Number of cycles n", "Zyklenzahl n"),
        }),
    ]
}

fn analysis_handle(bar: &analyses::Bar, entity: &EntityRef, id: &str, field: &'static str) -> Handle {
    handle(Spec {
        entity,
        tag: field,
        kind: HandleKind::ScalarAlong { axis: Vec3::Z },
        anchor: v3(bar.x, bar.y, bar.height()),
        grip: GripStyle::Arrow,
        writes: vec![binding(format!("{}[id={id}].{field}", bar.collection), Affine { base: 0.0, k: bar.per_metre }, match bar.collection { "fatigueSpectra" => "Pa", "factorOverrides" => "1", _ => "N" }, Some(Quantise::Round { step: bar.step }))],
        snap: grid(0.05),
        limits: (0.02, 12.0),
        name: bar.name.clone(),
    })
}

/// 🧭️ The view-state path the combination selector writes: the index into the generated combinations.
pub const COMBINATION_PATH: &str = "view.activeCombination";
/// 🚦️ The view-state path the traffic group selector writes: the index into the declared groups (`actions::present_groups`).
pub const TRAFFIC_GROUP_PATH: &str = "view.activeTrafficGroup";
const GROUP_SELECTOR_LIFT: f64 = 0.6;
const GROUP_TAG: &str = "group";
const GROUP_TOKEN_LIFT: f64 = 0.8;
const HEIGHT_AT_MAXIMUM: f64 = 2.0;
const HEIGHT_LIMIT: f64 = 5.0;

fn member_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(member), Some(pose)) = (d.members.iter().find(|m| m.id == id), layout.member(id)) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "members");
    let f = &pose.frame;
    let net = member.deflection_w1 + member.deflection_w2 + member.deflection_w3 - member.precamber;
    let scale = exaggeration(member.span, net.abs());
    let soffit = along(&along(&f.origin, &f.x, member.span / 2.0), &f.z, -member_depth(member.span) / 2.0);
    vec![
        handle(Spec {
            entity: &entity,
            tag: "span",
            kind: HandleKind::ScalarAlong { axis: f.x.clone() },
            anchor: along(&f.origin, &f.x, member.span),
            grip: GripStyle::Arrow,
            writes: vec![binding(format!("members[id={id}].span"), Affine::IDENTITY, "m", Some(Quantise::Round { step: 0.01 }))],
            snap: grid(0.05),
            limits: (0.5, 200.0),
            name: loc(&format!("Span of {}", member.label_en), &format!("Stützweite von {}", member.label_de)),
        }),
        handle(Spec {
            entity: &entity,
            tag: "precamber",
            kind: HandleKind::ScalarAlong { axis: f.z.clone() },
            anchor: along(&soffit, &f.z, member.precamber * scale),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("members[id={id}].precamber"), Affine { base: 0.0, k: 1.0 / scale }, "m", Some(Quantise::Round { step: 0.001 }))],
            snap: grid(0.01),
            limits: (0.0, member.span / 50.0 * scale),
            name: loc(&format!("Pre-camber w_c of {}", member.label_en), &format!("Überhöhung w_c von {}", member.label_de)),
        }),
    ]
    .into_iter()
    .chain(deflection_handles(member, &entity, &f.z, &soffit, scale))
    .chain(resistance_handles(member, &entity, f))
    .chain(gauges::rack(gauges::MEMBERS, member, "members", id, &entity, |dy| along(&along(&f.origin, &f.x, -0.8), &f.y, dy)))
    .collect()
}

/// 📏️ The design resistances R_d of the limit states and the serviceability criteria C_d of a member as diamond grips above its axis; the grip height is the force over a decade scale of its own value, spread along the span.
fn resistance_handles(member: &model::Member, entity: &EntityRef, f: &Frame) -> Vec<Handle> {
    let rows: [(&str, f64, &str, &str); 8] = [
        ("rdStr", member.rd_str, "Design resistance R_d (STR)", "Bemessungswiderstand R_d (STR)"),
        ("rdGeo", member.rd_geo, "Design resistance R_d (GEO)", "Bemessungswiderstand R_d (GEO)"),
        ("rdEquStab", member.rd_equ_stab, "Stabilising effect (EQU)", "Stabilisierende Wirkung (EQU)"),
        ("rdEquDestab", member.rd_equ_destab, "Destabilising effect (EQU)", "Destabilisierende Wirkung (EQU)"),
        ("rdFat", member.rd_fat, "Fatigue resistance (FAT)", "Ermüdungswiderstand (FAT)"),
        ("slsLimitChar", member.sls_limit_char, "SLS criterion C_d, characteristic", "GZG-Kriterium C_d, charakteristisch"),
        ("slsLimitFreq", member.sls_limit_freq, "SLS criterion C_d, frequent", "GZG-Kriterium C_d, häufig"),
        ("slsLimitQp", member.sls_limit_qp, "SLS criterion C_d, quasi-permanent", "GZG-Kriterium C_d, quasi-ständig"),
    ];
    let depth = member_depth(member.span);
    rows.into_iter().enumerate().filter(|(_, (_, value, ..))| *value > 0.0).map(|(slot, (field, value, en, de))| {
        let per_metre = analyses::decade_scale(value.abs());
        let at = along(&along(&f.origin, &f.x, member.span * (slot as f64 + 1.0) / 9.0), &f.z, depth / 2.0 + 0.5);
        handle(Spec {
            entity,
            tag: field,
            kind: HandleKind::ScalarAlong { axis: f.z.clone() },
            anchor: along(&at, &f.z, value / per_metre),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("members[id={}].{field}", member.id), Affine { base: 0.0, k: per_metre }, "N", Some(Quantise::Round { step: per_metre / 1000.0 }))],
            snap: grid(0.05),
            limits: (0.02, 12.0),
            name: loc(&format!("{en} of {}", member.label_en), &format!("{de} von {}", member.label_de)),
        })
    }).collect()
}

/// 📏️ The grips of the deflection components w_1, w_2, w_3 of Fig. A1.1 at the cumulative tips of the dimension stack; the exaggeration is the one of the stack.
fn deflection_handles(member: &model::Member, entity: &EntityRef, up: &Vec3, soffit: &Vec3, scale: f64) -> Vec<Handle> {
    let limit = member.span / 10.0;
    let mut cumulative = 0.0;
    [("w1", "deflectionW1", member.deflection_w1, "Permanent deflection w_1", "Ständige Durchbiegung w_1"), ("w2", "deflectionW2", member.deflection_w2, "Long-term deflection w_2", "Langzeitdurchbiegung w_2"), ("w3", "deflectionW3", member.deflection_w3, "Variable deflection w_3", "Durchbiegung w_3 aus veränderlicher Einwirkung")]
        .into_iter()
        .map(|(tag, field, value, en, de)| {
            cumulative += value;
            handle(Spec {
                entity,
                tag,
                kind: HandleKind::ScalarAlong { axis: mul(up, -1.0) },
                anchor: along(soffit, up, -cumulative * scale),
                grip: GripStyle::Diamond,
                writes: vec![binding(format!("members[id={}].{field}", entity.id), Affine { base: 0.0, k: 1.0 / scale }, "m", Some(Quantise::Round { step: 0.0005 }))],
                snap: grid(0.01),
                limits: (-limit, limit),
                name: loc(&format!("{en} of {}", member.label_en), &format!("{de} von {}", member.label_de)),
            })
        })
        .collect()
}

fn leaf_of(collection: &str) -> &'static str {
    match collection {
        "permanents" => "gk",
        "variables" => "qk",
        "accidentals" => "ad",
        _ => "aEk",
    }
}

pub(crate) fn raw_value(d: &En1990Snapshot, id: &str) -> f64 {
    d.permanents.iter().find(|a| a.id == id).map(|a| a.gk).or_else(|| d.variables.iter().find(|a| a.id == id).map(|a| a.qk)).or_else(|| d.accidentals.iter().find(|a| a.id == id).map(|a| a.ad)).or_else(|| d.seismics.iter().find(|a| a.id == id).map(|a| a.a_ek)).unwrap_or(0.0)
}

fn action_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let all = rows(d);
    let Some((index, row)) = all.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let Some((member, pose)) = affected(d, id).first().and_then(|m| d.members.iter().find(|x| &x.id == m).zip(layout.member(m))) else { return Vec::new() };
    let maximum = all.iter().map(|r| raw_value(d, &r.id).abs()).fold(1.0, f64::max);
    let per_metre = maximum / HEIGHT_AT_MAXIMUM;
    let f = &pose.frame;
    let value = raw_value(d, id);
    let base = along(&along(&f.origin, &f.x, member.span / 2.0), &f.z, member_depth(member.span) / 2.0 + lift(index));
    let entity = entity_ref(EntityKind::Load, id, row.collection);
    let influences = d.effects.iter().enumerate().filter(|(_, e)| e.action_id == id).enumerate().map(|(slot, (index, effect))| gauges::influence(effect, index, &entity, along(&along(&base, &f.y, -OVERRIDE_REACH * (slot as f64 + 2.0)), &f.z, 0.0)));
    std::iter::once(handle(Spec {
        entity: &entity,
        tag: "magnitude",
        kind: HandleKind::ScalarAlong { axis: f.z.clone() },
        anchor: along(&base, &f.z, value / per_metre),
        grip: GripStyle::Arrow,
        writes: vec![binding(format!("{}[id={id}].{}", row.collection, leaf_of(row.collection)), Affine { base: 0.0, k: per_metre }, "N", Some(Quantise::Round { step: 100.0 }))],
        snap: grid(0.05),
        limits: (0.0, HEIGHT_LIMIT),
        name: loc(&format!("Magnitude of {}", row.name.en), &format!("Größe von {}", row.name.de)),
    }))
    .chain(influences)
    .collect()
}

const OVERRIDE_REACH: f64 = 0.4;

const IMPORTANCE_TOKENS: [(&str, &str, &str); 4] = [("I", "I (γ_I = 0.8)", "I (γ_I = 0,8)"), ("II", "II (γ_I = 1.0)", "II (γ_I = 1,0)"), ("III", "III (γ_I = 1.2)", "III (γ_I = 1,2)"), ("IV", "IV (γ_I = 1.4)", "IV (γ_I = 1,4)")];

/// 🏛️ The importance-class stepper of a seismic action beside its magnitude grip; the name carries the live design action A_Ed = γ_I · A_Ek.
fn seismic_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let all = rows(d);
    let (Some((index, row)), Some(seismic)) = (all.iter().enumerate().find(|(_, r)| r.id == id), d.seismics.iter().find(|a| a.id == id)) else { return Vec::new() };
    let Some((member, pose)) = affected(d, id).first().and_then(|m| d.members.iter().find(|x| &x.id == m).zip(layout.member(m))) else { return Vec::new() };
    let f = &pose.frame;
    let anchor = along(&along(&f.origin, &f.x, member.span / 2.0), &f.z, member_depth(member.span) / 2.0 + lift(index));
    let options: Vec<(String, &str, &str)> = IMPORTANCE_TOKENS.iter().map(|(token, en, de)| ((*token).to_string(), *en, *de)).collect();
    vec![choice_handle(&entity_ref(EntityKind::Load, id, row.collection), "importance", along(&anchor, &f.y, -OVERRIDE_REACH), format!("seismics[id={id}].importanceClass"), &options, loc(&format!("Importance class of {} (A_Ed = {:.1} kN)", row.name.en, seismic.a_ed() / 1e3), &format!("Bedeutungskategorie von {} (A_Ed = {:.1} kN)", row.name.de, seismic.a_ed() / 1e3)))]
}

/// 🧪️ The mean and the standard deviation grips of a test series on its design-value bar; the name carries the live design value X_d with the 5 % fractile factor k_n of Annex D.
fn test_series_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(series), Some(bar)) = (d.test_series.iter().find(|s| s.id == id), analyses::bars(d, layout).into_iter().find(|b| b.owner == id && b.collection == "testSeries" && b.suffix.is_none())) else { return Vec::new() };
    let design = super::checks::test_design_value(series.distribution.as_str(), series.sample_size, series.mean, series.std_dev, series.variation_known, series.conversion_factor).unwrap_or(0.0);
    let entity = entity_ref(EntityKind::Plate, id, "testSeries");
    let live = |en: &str, de: &str, symbol: &str| loc(&format!("{en} of {id} (X_d = {design:.3}, {symbol})"), &format!("{de} von {id} (X_d = {design:.3}, {symbol})"));
    let fractile = format!("k_n = {}", super::checks::k_n(f64::from(series.sample_size), series.variation_known > 0.0).map_or_else(|| "-".to_string(), |k| format!("{k:.2}")));
    [("mean", "mean", series.mean, "Sample mean", "Stichprobenmittel", (0.0, 12.0), true), ("sd", "stdDev", series.mean + series.std_dev, "Sample standard deviation", "Stichprobenstandardabweichung", (0.0, 12.0), false)]
        .into_iter()
        .map(|(tag, field, height, en, de, limits, is_mean)| {
            handle(Spec {
                entity: &entity,
                tag,
                kind: HandleKind::ScalarAlong { axis: Vec3::Z },
                anchor: v3(bar.x, bar.y, height / bar.per_metre),
                grip: if is_mean { GripStyle::Arrow } else { GripStyle::Diamond },
                writes: vec![binding(format!("testSeries[id={id}].{field}"), Affine { base: if is_mean { 0.0 } else { -series.mean }, k: bar.per_metre }, "1", Some(Quantise::Round { step: bar.step }))],
                snap: grid(0.05),
                limits,
                name: live(en, de, &fractile),
            })
        })
        .chain(gauges::rack(gauges::TEST_SERIES, series, "testSeries", id, &entity, |dy| v3(bar.x + 0.6, bar.y + dy, 0.3)))
        .collect()
}

/// 🎚️ The grips of the factor-override rows that target this action exactly: the row value (γ range 0 to 2 at 1 m per unit, ψ range 0 to 1 at 2 m per unit) is the grip height beside the magnitude grip.
fn override_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let all = rows(d);
    let Some((index, row)) = all.iter().enumerate().find(|(_, r)| r.id == id) else { return Vec::new() };
    let Some((member, pose)) = affected(d, id).first().and_then(|m| d.members.iter().find(|x| &x.id == m).zip(layout.member(m))) else { return Vec::new() };
    let f = &pose.frame;
    let base = along(&along(&f.origin, &f.x, member.span / 2.0), &f.z, member_depth(member.span) / 2.0 + lift(index));
    let entity = entity_ref(EntityKind::Load, id, row.collection);
    d.factor_overrides
        .iter()
        .filter(|o| o.target.trim() == id && matches!(o.factor.as_str(), "gamma" | "psi0" | "psi1" | "psi2"))
        .map(|o| {
            let scale = if o.factor == "gamma" { 1.0 } else { 2.0 };
            let symbol = match o.factor.as_str() {
                "gamma" => "γ",
                "psi0" => "ψ₀",
                "psi1" => "ψ₁",
                _ => "ψ₂",
            };
            handle(Spec {
                entity: &entity,
                tag: &override_tag(&o.id),
                kind: HandleKind::ScalarAlong { axis: f.z.clone() },
                anchor: along(&along(&base, &f.y, OVERRIDE_REACH), &f.z, o.value * scale),
                grip: GripStyle::Diamond,
                writes: vec![binding(format!("factorOverrides[id={}].value", o.id), Affine { base: 0.0, k: 1.0 / scale }, "1", Some(Quantise::Round { step: 0.01 }))],
                snap: grid(0.05),
                limits: (0.0, 2.0 * scale),
                name: loc(&format!("{symbol} override of {} (deviates from NA)", row.name.en), &format!("{symbol}-Überschreibung von {} (weicht vom NA ab)", row.name.de)),
            })
        })
        .collect()
}

fn override_tag(row_id: &str) -> String {
    format!("ovr-{row_id}")
}

fn combination_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (true, Some(anchor)) = (id == COMBINATION_ENTITY, selector_anchor(d, layout)) else { return Vec::new() };
    let count = super::combinations::columns(d).len();
    let entity = entity_ref(EntityKind::Group, COMBINATION_ENTITY, "combinations");
    let groups = present_groups(d).len();
    let select = handle(Spec {
        entity: &entity,
        tag: "select",
        kind: HandleKind::CountStepper,
        anchor: anchor.clone(),
        grip: GripStyle::Diamond,
        writes: vec![binding(COMBINATION_PATH.to_string(), Affine::IDENTITY, "1", Some(Quantise::Int))],
        snap: SnapRule::NONE,
        limits: (0.0, count.saturating_sub(1).max(1) as f64),
        name: loc("Active combination", "Aktive Kombination"),
    });
    let group = (groups > 0).then(|| {
        handle(Spec {
            entity: &entity,
            tag: GROUP_TAG,
            kind: HandleKind::CountStepper,
            anchor: along(&anchor, &Vec3::Z, GROUP_SELECTOR_LIFT),
            grip: GripStyle::Diamond,
            writes: vec![binding(TRAFFIC_GROUP_PATH.to_string(), Affine::IDENTITY, "1", Some(Quantise::Int))],
            snap: SnapRule::NONE,
            limits: (0.0, groups.saturating_sub(1).max(1) as f64),
            name: loc("Active traffic load group", "Aktive Verkehrslastgruppe"),
        })
    });
    std::iter::once(select).chain(group).collect()
}

/// 🕹️ The grips of a design case: its declared dimensions and load, and the placement move.
fn case_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(row), Some(pose)) = (d.cases.iter().find(|c| c.id == id), layout.case(id)) else { return Vec::new() };
    super::scene::projection_kit::design_case::forms::handles::<super::checks::catalogue::En1990Cases>(row, pose, "cases")
}

/// 🌉️ The grips of a bridge deck row: the vertical deflection at mid-span and its limit, both drawn on the deck soffit with the exaggeration of the deflection line.
fn bridge_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(sls) = d.bridge_sls.iter().find(|s| s.id == id) else { return Vec::new() };
    let (Some(member), Some(pose)) = (d.members.iter().find(|m| m.id == sls.member_id && m.span > 1e-6), layout.member(&sls.member_id)) else { return Vec::new() };
    let f = &pose.frame;
    let scale = exaggeration(member.span, sls.bridge_deflection.abs().max(sls.bridge_deflection_limit));
    let soffit = along(&along(&f.origin, &f.x, member.span / 2.0), &f.z, -member_depth(member.span) / 2.0);
    let entity = entity_ref(EntityKind::Plate, id, "bridgeSls");
    [("deflection", "bridgeDeflection", sls.bridge_deflection, 0.0, "Bridge deflection", "Brückendurchbiegung"), ("deflection-limit", "bridgeDeflectionLimit", sls.bridge_deflection_limit, 0.4, "Bridge deflection limit", "Grenzdurchbiegung der Brücke")]
        .into_iter()
        .map(|(tag, field, value, reach, en, de)| {
            handle(Spec {
                entity: &entity,
                tag,
                kind: HandleKind::ScalarAlong { axis: mul(&f.z, -1.0) },
                anchor: along(&along(&soffit, &f.y, reach), &f.z, -value * scale),
                grip: GripStyle::Diamond,
                writes: vec![binding(format!("bridgeSls[id={id}].{field}"), Affine { base: 0.0, k: 1.0 / scale }, "m", Some(Quantise::Round { step: 0.0005 }))],
                snap: grid(0.01),
                limits: (0.0, member.span / 50.0 * scale),
                name: loc(&format!("{en} of {id}"), &format!("{de} von {id}")),
            })
        })
        .chain(gauges::rack(gauges::BRIDGE_SLS, sls, "bridgeSls", id, &entity, |dy| along(&along(&f.origin, &f.x, member.span + 0.8), &f.y, dy)))
        .collect()
}

/// 🚦️ The choice grip of a traffic load group component above its lane on the deck: it steps through the groups and writes the typed token of `trafficGroups[id=ROW].group`.
fn traffic_group_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(row) = d.traffic_groups.iter().find(|r| r.id == id) else { return Vec::new() };
    let Some((member, pose)) = affected(d, &row.variable_id).first().and_then(|m| d.members.iter().find(|x| &x.id == m).zip(layout.member(m))) else { return Vec::new() };
    if !d.variables.iter().any(|v| v.id == row.variable_id) {
        return Vec::new();
    }
    let groups = present_groups(d);
    let lane = groups.iter().position(|g| *g == row.group).unwrap_or(0) as f64;
    let lateral = (lane - (groups.len() as f64 - 1.0) / 2.0) * GROUP_LANE;
    let f = &pose.frame;
    let anchor = along(&along(&along(&f.origin, &f.x, member.span / 2.0), &f.y, lateral), &f.z, member_depth(member.span) / 2.0 + GROUP_TOKEN_LIFT);
    let labels: Vec<(String, String, String)> = model::LoadGroup::ALL.iter().map(|g| (g.as_str().to_string(), format!("Group {}", g.as_str()), format!("Gruppe {}", g.as_str()))).collect();
    let options: Vec<(String, &str, &str)> = labels.iter().map(|(token, en, de)| (token.clone(), en.as_str(), de.as_str())).collect();
    vec![choice_handle(&entity_ref(EntityKind::Load, id, "trafficGroups"), GROUP_TAG, anchor, format!("trafficGroups[id={id}].group"), &options, loc(&format!("Traffic load group of {id}"), &format!("Verkehrslastgruppe von {id}")))]
}

/// 🧲️ The move and rotate grips of a member: a drag writes `placements[id=MEMBER].x|y|yaw`.
fn move_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(pose) = layout.member(id) else { return Vec::new() };
    let stored: Vec<_> = d.placements.iter().map(|p| super::scene::projection_kit::Placement { entity_id: p.id.clone(), origin: Vec3::new(p.x, p.y, p.z), yaw: p.yaw }).collect();
    super::scene::projection_kit::handle_kit::move_handles(&entity_ref(EntityKind::Member, id, "members"), id, &pose.frame.origin, &stored, 0.0, true)
}

/// 🕹️ The handles of one document entity, anchored in the given layout.
pub(crate) fn build(document: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let id = base_id(id);
    [member_handles(document, layout, id), action_handles(document, layout, id), override_handles(document, layout, id), seismic_handles(document, layout, id), test_series_handles(document, layout, id), analysis_handles(document, layout, id), bridge_handles(document, layout, id), traffic_group_handles(document, layout, id), combination_handles(document, layout, id), case_handles(document, layout, id), move_handles(document, layout, id)].concat()
}

/// 🆔️ The handle ids a member advertises.
pub(crate) fn ids(document: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).collect()
}

/// 🆔️ The handle ids an action glyph advertises.
pub(crate) fn action_ids(document: &En1990Snapshot, id: &str) -> Vec<String> {
    let mut ids = vec![format!("{id}:magnitude")];
    ids.extend(document.seismics.iter().filter(|a| a.id == id).map(|_| format!("{id}:importance")));
    ids.extend(document.effects.iter().filter(|e| e.action_id == id).map(|e| format!("{id}:{}", gauges::influence_tag(&e.member_id))));
    ids.extend(document.factor_overrides.iter().filter(|o| o.target.trim() == id && matches!(o.factor.as_str(), "gamma" | "psi0" | "psi1" | "psi2")).map(|o| format!("{id}:{}", override_tag(&o.id))));
    ids
}

/// 🆔️ The handle ids the combination selector advertises.
pub(crate) fn combination_ids(document: &En1990Snapshot) -> Vec<String> {
    let mut ids = vec![format!("{COMBINATION_ENTITY}:select")];
    if !present_groups(document).is_empty() {
        ids.push(format!("{COMBINATION_ENTITY}:{GROUP_TAG}"));
    }
    ids
}

/// 🕹️ The handles of every selected entity (derived scene ids resolve to their document entity), in selection order, without duplicates.
pub fn handles(document: &En1990Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document, placements);
    let mut out: Vec<Handle> = Vec::new();
    for entity in selection {
        for handle in build(document, &layout, &entity.id) {
            if !out.iter().any(|h| h.id == handle.id) {
                out.push(handle);
            }
        }
    }
    out
}

//! 🩹️ Minimal anchor remedies: one click writes the smallest effective embedment `hEf`, steel area `aS` or edge distance `c1` that makes a failing EN 1992-4 check pass, found by re-evaluating the anchor. See [`super`] and [`crate::member_evaluation::minimal_remedies`].

use super::*;
use crate::member_evaluation::minimal_remedies::{search, Direction};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AnchorField {
    EmbedmentDepth,
    SteelArea,
    EdgeDistance,
}

impl AnchorField {
    pub(super) fn get(self, a: &Anchor) -> f64 {
        match self {
            AnchorField::EmbedmentDepth => a.h_ef,
            AnchorField::SteelArea => a.a_s,
            AnchorField::EdgeDistance => a.c1,
        }
    }

    pub(super) fn set(self, a: &mut Anchor, v: f64) {
        match self {
            AnchorField::EmbedmentDepth => a.h_ef = v,
            AnchorField::SteelArea => a.a_s = v,
            AnchorField::EdgeDistance => a.c1 = v,
        }
    }

    fn leaf(self) -> &'static str {
        match self {
            AnchorField::EmbedmentDepth => "hEf",
            AnchorField::SteelArea => "aS",
            AnchorField::EdgeDistance => "c1",
        }
    }

    fn quantity(self, v: f64) -> Quantity {
        match self {
            AnchorField::SteelArea => area_m2(v),
            _ => length_m(v),
        }
    }

    fn step(self) -> f64 {
        match self {
            AnchorField::SteelArea => 1.0e-6,
            _ => 1.0e-3,
        }
    }

    fn name(self) -> (&'static str, &'static str) {
        match self {
            AnchorField::EmbedmentDepth => ("effective embedment depth", "wirksame Verankerungstiefe"),
            AnchorField::SteelArea => ("anchor steel area", "Dübelstahlquerschnitt"),
            AnchorField::EdgeDistance => ("edge distance", "Randabstand"),
        }
    }

    fn display(self, v: f64) -> String {
        match self {
            AnchorField::SteelArea => format!("{:.0} mm²", v * 1.0e6),
            _ => format!("{:.0} mm", v * 1.0e3),
        }
    }
}

/// 📋 The anchor fields that can repair a check, by check id.
pub(super) fn plan(id: &str) -> Vec<AnchorField> {
    use AnchorField::{EdgeDistance, EmbedmentDepth, SteelArea};
    let has = |prefix: &str| id.starts_with(prefix);
    if has("en1992-4.steel-shear.") || has("en1992-4.steel.") {
        vec![SteelArea]
    } else if has("en1992-4.cone.") || has("en1992-4.pryout.") || has("en1992-4.pullout.") {
        vec![EmbedmentDepth]
    } else if has("en1992-4.splitting.") {
        vec![EmbedmentDepth, EdgeDistance]
    } else if has("en1992-4.edge.") {
        vec![EdgeDistance]
    } else if has("en1992-4.interaction.") {
        vec![EmbedmentDepth, EdgeDistance, SteelArea]
    } else {
        Vec::new()
    }
}

fn passes(doc: &En1992Snapshot, anchor: &Anchor, id: &str, writes: &[(AnchorField, f64)]) -> bool {
    let mut candidate = anchor.clone();
    writes.iter().for_each(|(field, value)| field.set(&mut candidate, *value));
    evaluate_anchor_unsolved(doc, &candidate).iter().find(|r| r.id == id).is_some_and(|r| r.status != CheckStatus::Fail)
}

const JOINT_SCALES: [f64; 6] = [1.25, 1.5, 2.0, 3.0, 5.0, 8.0];

/// 🤝 All planned fields written together: the others are raised by growing scales, the first one is solved for each until the check passes (tension-shear interaction needs embedment, edge distance and steel area at once).
fn joint(doc: &En1992Snapshot, anchor: &Anchor, id: &str, planned: &[AnchorField]) -> Option<Vec<(AnchorField, f64)>> {
    planned.iter().enumerate().find_map(|(first, primary)| {
        JOINT_SCALES.iter().find_map(|scale| {
            let others: Vec<(AnchorField, f64)> = planned.iter().enumerate().filter(|(index, _)| *index != first).map(|(_, field)| (*field, (field.get(anchor) * scale / field.step()).ceil() * field.step())).collect();
            let value = search(primary.get(anchor), primary.step(), false, Direction::Raise, |v| {
                let mut writes = others.clone();
                writes.push((*primary, v));
                passes(doc, anchor, id, &writes)
            })?;
            let mut all = others;
            all.push((*primary, value));
            Some(all)
        })
    })
}

/// 🩹️ Adds the minimal one-click remedy of every planned field to every failing anchor check.
pub(super) fn attach_minimal_remedies(doc: &En1992Snapshot, anchor: &Anchor, results: &mut [CheckResult]) {
    let (solved, _) = record_ndp_reads(|| {
        results
            .iter()
            .enumerate()
            .filter(|(_, r)| r.status == CheckStatus::Fail)
            .map(|(index, r)| {
                let remedies: Vec<Remedy> = plan(&r.id)
                    .into_iter()
                    .filter_map(|field| {
                        let current = field.get(anchor);
                        let value = search(current, field.step(), false, Direction::Raise, |v| passes(doc, anchor, &r.id, &[(field, v)]))?;
                        let (en, de) = field.name();
                        let (from, to) = (field.display(current), field.display(value));
                        Some(Remedy::at_least(
                            SubjectRef::new(&anchor.id, format!("anchors[id={}].{}", anchor.id, field.leaf()), r.subject.label.clone()),
                            field.quantity(current),
                            field.quantity(value),
                            l(&format!("Raise the {en} of anchor {} from {from} to {to}, the smallest value that passes {}.", anchor.id, r.id), &format!("{de} von Dübel {} von {from} auf mindestens {to} erhöhen, der kleinste Wert, der {} erfüllt.", anchor.id, r.id)),
                        ))
                    })
                    .collect();
                (index, remedies)
            })
            .collect::<Vec<_>>()
    });
    for (index, mut remedies) in solved {
        let result = &results[index];
        let mut merged: Vec<Remedy> = result.remedies.iter().filter(|old| !remedies.iter().any(|new| new.target.path == old.target.path)).cloned().collect();
        merged.append(&mut remedies);
        if !merged.iter().any(|x| x.applicable) {
            let planned = plan(&result.id);
            match (planned.len() > 1).then(|| joint(doc, anchor, &result.id, &planned)).flatten() {
                Some(together) => {
                    let paths: Vec<String> = together.iter().map(|(field, _)| format!("anchors[id={}].{}", anchor.id, field.leaf())).collect();
                    merged.retain(|old| !paths.contains(&old.target.path));
                    merged.extend(together.iter().map(|(field, value)| {
                        let current = field.get(anchor);
                        let (en, de) = field.name();
                        let (from, to) = (field.display(current), field.display(*value));
                        let mut remedy = Remedy::at_least(
                            SubjectRef::new(&anchor.id, format!("anchors[id={}].{}", anchor.id, field.leaf()), result.subject.label.clone()),
                            field.quantity(current),
                            field.quantity(*value),
                            l(&format!("Raise the {en} of anchor {} from {from} to {to}; written together with {}, they pass {}.", anchor.id, paths.join(", "), result.id), &format!("{de} von Dübel {} von {from} auf mindestens {to} erhöhen; gemeinsam mit {} geschrieben erfüllen sie {}.", anchor.id, paths.join(", "), result.id)),
                        );
                        remedy.options = paths.clone();
                        remedy
                    }));
                }
                None => merged.push(notice_of(anchor, result)),
            }
        }
        results[index].remedies = merged;
    }
}

/// 🔔 The typed "no single change suffices" notice: a non-applicable choice remedy whose options are the governing field paths, in English and German.
fn notice_of(anchor: &Anchor, result: &CheckResult) -> Remedy {
    let planned = plan(&result.id);
    let names: Vec<(&str, &str)> = planned.iter().map(|field| field.name()).collect();
    let (en, de) = if planned.is_empty() { ("the inputs of this check".to_string(), "die Eingaben dieser Prüfung".to_string()) } else { (names.iter().map(|n| n.0).collect::<Vec<_>>().join(", "), names.iter().map(|n| n.1).collect::<Vec<_>>().join(", ")) };
    let options = if planned.is_empty() { vec![result.subject.path.clone()] } else { planned.iter().map(|field| format!("anchors[id={}].{}", anchor.id, field.leaf())).collect() };
    let action = l(&format!("No single change suffices for {}: revise {en} of anchor {} together or rethink the design.", result.id, anchor.id), &format!("Keine einzelne Änderung genügt für {}: {de} von Dübel {} gemeinsam überarbeiten oder den Entwurf überdenken.", result.id, anchor.id));
    Remedy { applicable: false, ..Remedy::one_of(result.subject.clone(), options, action) }
}

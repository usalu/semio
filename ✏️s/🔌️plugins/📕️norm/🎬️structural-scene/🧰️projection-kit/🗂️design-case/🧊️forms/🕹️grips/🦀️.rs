//! 🕹️ Quantity grips of the case forms: the grips of inputs that are no plain length (counts, densities, times, areas, coupled inputs). A count is a stepper that writes the integer, a spacing is a linear drag on the length input, a gauge is a linear drag with its own law `stored = value + k · s` that may drag follower inputs along (one release, one `change-case-number` per written input). See [`super`].

use super::*;

/// 🧭️ How an extra grip of a case turns its drag into the input it writes.
#[derive(Clone, Debug, PartialEq)]
pub enum GripKind {
    /// 📏️ A scalar drag along the axis on a length input: `stored = value + factor · per_metre · s`.
    Scalar,
    /// 📐️ A face offset on a length input.
    Face,
    /// ↔️ A spacing: a linear drag on a length input, anchored between the elements it separates.
    Spacing,
    /// 🔢️ A count stepper that writes the integer count in `min ..= max`.
    Count { min: f64, max: f64 },
    /// 📊️ A gauge drag with the law `stored = value + k · s` (`k` stored units per metre, `step` in stored units, `limits` of `s` in metres) on any numeric input, with the followers that have a leaf written in the same release.
    Gauge { k: f64, step: f64, limits: (f64, f64), follow: Vec<Follow> },
}

/// 🔗️ An input written by the same drag as the input of its grip: `stored = base + k · s`, quantised to `step`.
#[derive(Clone, Debug, PartialEq)]
pub struct Follow {
    pub key: String,
    pub base: f64,
    pub k: f64,
    pub step: f64,
}

impl GripKind {
    pub(super) fn quantity(&self) -> bool {
        matches!(self, Self::Count { .. } | Self::Gauge { .. })
    }
}

fn leaf<C: CaseCatalogue>(row: &CaseRow, key: &str) -> Option<(f64, &'static str)> {
    let value = row.numbers.iter().find(|n| n.id == key)?.value;
    design_case::spec::<C>(&row.entry_id, key).map(|spec| (value, spec.unit.symbol()))
}

/// 🕹️ The handle of a count or gauge plan of `row` in the pose; `None` while an input of it is unset.
pub(super) fn quantity<C: CaseCatalogue>(row: &CaseRow, plan: &Plan, pose: &CasePose, entity: &EntityRef) -> Option<Handle> {
    let (value, unit) = leaf::<C>(row, &plan.key)?;
    let (en, de) = label_of::<C>(row, &plan.key);
    let path = design_case::input_path(&row.id, &plan.key, true);
    let anchor = pose.at(plan.anchor[0], plan.anchor[1], plan.anchor[2]);
    let name = named(&en, &de, &row.id);
    match &plan.kind {
        GripKind::Count { min, max } => Some(handle_kit::handle(HandleSpec { entity, tag: &plan.tag, kind: HandleKind::CountStepper, anchor, grip: GripStyle::Diamond, writes: vec![FieldBinding { path, map: Affine::IDENTITY, unit: unit.to_string(), quantise: Some(Quantise::Int) }], snap: SnapRule::NONE, limits: (*min, *max), name })),
        GripKind::Gauge { k, step, limits, follow } => {
            let mut writes = vec![handle_kit::field_binding(path, Affine { base: value, k: *k }, unit, handle_kit::round(*step))];
            for f in follow {
                if let Some((_, follower_unit)) = leaf::<C>(row, &f.key) {
                    writes.push(handle_kit::field_binding(design_case::input_path(&row.id, &f.key, true), Affine { base: f.base, k: f.k }, follower_unit, handle_kit::round(f.step)));
                }
            }
            Some(handle_kit::handle(HandleSpec { entity, tag: &plan.tag, kind: HandleKind::ScalarAlong { axis: pose.dir(&v3(plan.axis[0], plan.axis[1], plan.axis[2])) }, anchor, grip: GripStyle::Diamond, writes, snap: handle_kit::grid(step / k.abs().max(1e-12)), limits: *limits, name }))
        }
        _ => None,
    }
}

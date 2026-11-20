//! ✂️ Original paired solid payload and cancellation custody.

use super::*;

pub(super) fn field(pair: &[SolidPublication; 2], index: usize) -> Option<(&'static str, Field<'_>)> {
    match index % 4 {
        0 => Some(("$schema", Field::Text("geometry"))),
        1 => Some(("handle", Field::Text(pair[index / 4].handle()))),
        2 => Some(("kind", Field::Text("solid"))),
        _ => Some((["positive", "negative"][index / 4], Field::Payload)),
    }
}

pub(super) fn unfinished(pair: &mut [SolidPublication; 2]) -> Option<&mut SolidPublication> {
    pair.iter_mut().find(|solid| !solid.topology_ready())
}

fn pending_key(pair: &[SolidPublication; 2], data: &GeometryOutputData, staged: [bool; 2]) -> Option<(usize, bool)> {
    let index = staged.into_iter().position(|staged| !staged)?;
    let length = pair[index].handle().len();
    if data.split_live[index].len() < length { return Some((index, false)); }
    (data.split_claim[index].len() < length).then_some((index, true))
}

pub(super) fn demand(session: &Session, pair: &[SolidPublication; 2], data: &GeometryOutputData, staged: [bool; 2], copy: usize) -> Result<RetirementDemand, ValueError> {
    if let Some((index, claim)) = pending_key(pair, data, staged) {
        let target = if claim { &data.split_claim[index] } else { &data.split_live[index] };
        return Ok(text_demand(target, pair[index].handle()));
    }
    session.solid_pair_demand(pair, staged, copy)
}

pub(super) fn step(session: &Session, pair: &mut [SolidPublication; 2], data: &mut GeometryOutputData, staged: &mut [bool; 2], effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, Progress), ValueError> {
    if let Some((index, claim)) = pending_key(pair, data, *staged) {
        let target = if claim { &mut data.split_claim[index] } else { &mut data.split_live[index] };
        return write_text(target, pair[index].handle(), effect).map(|receipt| (false, receipt));
    }
    session.step_solid_pair(pair, &mut data.split_live, &mut data.split_claim, staged, effect)
}

pub(super) fn close_demand(session: &Session, pair: &[SolidPublication; 2], staged: [bool; 2]) -> Result<RetirementDemand, ValueError> {
    if let Some(index) = staged.into_iter().position(|staged| staged) {
        return session.staged_geometry_claim_close_demand(pair[index].handle());
    }
    if let Some(solid) = pair.iter().find(|solid| !solid.terminal_is_empty()) {
        return session.solid_primitive_close_demand(solid);
    }
    Ok(RetirementDemand { release_bytes: size_of::<[SolidPublication; 2]>(), depth: 1, ..Default::default() })
}

pub(super) fn close(session: &Session, pair: &mut [SolidPublication; 2], staged: &mut [bool; 2], effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<Progress>, ValueError> {
    if let Some(index) = staged.iter().position(|&staged| staged) {
        let receipt = session.close_staged_geometry_claim(pair[index].handle(), effect)?;
        if receipt.copied_items > 0 { staged[index] = false; }
        return Ok(Some(receipt));
    }
    if let Some(solid) = pair.iter_mut().find(|solid| !solid.terminal_is_empty()) {
        return session.close_solid_primitive(solid, effect).map(|step| Some(step.progress()));
    }
    Ok(None)
}

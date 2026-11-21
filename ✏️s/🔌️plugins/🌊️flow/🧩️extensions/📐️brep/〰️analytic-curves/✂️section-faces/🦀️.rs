//! ✂️ Disconnected Section face construction, list output, and cancellation.

use super::*;

pub(super) fn field(faces: &PolygonSectionSet, index: usize) -> Option<(&'static str, Field<'_>)> {
    const ORDINALS: [&str; SECTION_FACE_LIMIT] = ["0", "1", "2", "3", "4"];
    match index.checked_sub(faces.count() * 4) {
        Some(0) => return Some(("$schema", Field::SectionSchema)),
        Some(1) => return Some(("faces", Field::SectionPayload)),
        Some(_) => return None,
        None => {}
    }
    let face = index / 4;
    match index % 4 {
        0 => Some(("$schema", Field::Text("geometry"))),
        1 => Some(("handle", Field::Text(faces.face(face).0.handle()))),
        2 => Some(("kind", Field::Text("face"))),
        _ => Some((ORDINALS[face], Field::SectionMember)),
    }
}

pub(super) fn unfinished(faces: &PolygonSectionSet) -> Option<usize> {
    (0..faces.count()).find(|&i| !faces.face(i).0.topology_ready())
}

pub(super) fn step_topology(session: &Session, faces: &mut PolygonSectionSet, index: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
    let (face, polygon) = faces.face_mut(index);
    let grant = effect.grant();
    let demand = session.polygon_face_demand(face, None, false, grant.maximum_copy_bytes)?;
    let point = face.next_point_index().filter(|_| demand.capacity_bytes == 0 && retained_clone_grant_admits(grant, demand))
        .map(|point| polygon.point(point).ok_or_else(|| ValueError::invariant("original section vertex ordinal changed"))).transpose()?;
    session.step_polygon_face_topology(face, None, point, effect)
}

fn pending_key(faces: &PolygonSectionSet, data: &GeometryOutputData, staged: [bool; SECTION_FACE_LIMIT]) -> Option<(usize, bool)> {
    let index = staged[..faces.count()].iter().position(|staged| !staged)?;
    let length = faces.face(index).0.handle().len();
    if data.section_live[index].len() < length { return Some((index, false)); }
    (data.section_claim[index].len() < length).then_some((index, true))
}

pub(super) fn demand(session: &Session, faces: &PolygonSectionSet, data: &GeometryOutputData, staged: [bool; SECTION_FACE_LIMIT], copy: usize) -> Result<RetirementDemand, ValueError> {
    if let Some((index, claim)) = pending_key(faces, data, staged) {
        let target = if claim { &data.section_claim[index] } else { &data.section_live[index] };
        return Ok(text_demand(target, faces.face(index).0.handle()));
    }
    session.polygon_section_demand(faces, staged, copy)
}

pub(super) fn step(session: &Session, faces: &mut PolygonSectionSet, data: &mut GeometryOutputData, staged: &mut [bool; SECTION_FACE_LIMIT], effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, Progress), ValueError> {
    if let Some((index, claim)) = pending_key(faces, data, *staged) {
        let target = if claim { &mut data.section_claim[index] } else { &mut data.section_live[index] };
        return write_text(target, faces.face(index).0.handle(), effect).map(|receipt| (false, receipt));
    }
    session.step_polygon_section(faces, &mut data.section_live, &mut data.section_claim, staged, effect)
}

pub(super) fn close_demand(session: &Session, faces: &PolygonSectionSet, staged: [bool; SECTION_FACE_LIMIT]) -> Result<RetirementDemand, ValueError> {
    if let Some(index) = staged.iter().position(|staged| *staged) {
        return session.staged_geometry_claim_close_demand(faces.face(index).0.handle());
    }
    if let Some((face, _)) = (0..faces.count()).map(|i| faces.face(i)).find(|(face, _)| !face.terminal_is_empty()) {
        return session.polygon_face_close_demand(face);
    }
    Ok(RetirementDemand { release_bytes: size_of::<PolygonSectionSet>(), depth: 1, ..Default::default() })
}

pub(super) fn close(session: &Session, faces: &mut PolygonSectionSet, staged: &mut [bool; SECTION_FACE_LIMIT], effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<Progress>, ValueError> {
    if let Some(index) = staged.iter().position(|staged| *staged) {
        let receipt = session.close_staged_geometry_claim(faces.face(index).0.handle(), effect)?;
        if receipt.copied_items != 0 { staged[index] = false; }
        return Ok(Some(receipt));
    }
    for index in 0..faces.count() {
        let (face, _) = faces.face_mut(index);
        if !face.terminal_is_empty() {
            return session.close_polygon_face(face, effect).map(|step| Some(step.progress()));
        }
    }
    Ok(None)
}

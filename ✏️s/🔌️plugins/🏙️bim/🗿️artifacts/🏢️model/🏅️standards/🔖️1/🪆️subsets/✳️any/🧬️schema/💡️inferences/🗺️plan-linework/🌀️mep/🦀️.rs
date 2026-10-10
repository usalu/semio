//! 🌀️ MEP elements in the plan: the centre line of every run, a filled band of the width of a duct or tray (the diameter of a pipe) around the horizontal stretches, mitred where they turn, and for every vertical stretch
//! a circle (pipe) or a rectangle (duct, tray) at the foot of the drop. The band and the drops are cut, projected or hidden by the vertical span of the element against the cut plane like every element. The colour of every
//! primitive is the colour of the system, `mep[id].colour` of the inference (the primitives carry the id of the element).

use super::{classify, outline_style, Context, PlanKind, PlanVertex, Sheet};
use crate::standards::v1::subsets::any::schema::inferences::mep::{MepSectionKind, MepSegment, MepValue, EPS};

/// 📏️ The smallest cosine of the half bend angle: a sharper turn is mitred as if it were this one.
const MITRE_LIMIT: f64 = 0.2;

fn heading(segment: &MepSegment) -> (f64, f64) {
    let length = (segment.to.x - segment.from.x).hypot(segment.to.y - segment.from.y);
    ((segment.to.x - segment.from.x) / length, (segment.to.y - segment.from.y) / length)
}

fn chains(value: &MepValue) -> Vec<Vec<&MepSegment>> {
    let mut chains: Vec<Vec<&MepSegment>> = Vec::new();
    let mut open = false;
    for segment in &value.segments {
        if segment.vertical() {
            open = false;
        } else if open {
            chains.last_mut().expect("an open chain").push(segment);
        } else {
            chains.push(vec![segment]);
            open = true;
        }
    }
    chains
}

fn band(chain: &[&MepSegment], half: f64) -> Vec<PlanVertex> {
    let mut points: Vec<(f64, f64)> = vec![(chain[0].from.x, chain[0].from.y)];
    points.extend(chain.iter().map(|segment| (segment.to.x, segment.to.y)));
    let headings: Vec<(f64, f64)> = chain.iter().map(|segment| heading(segment)).collect();
    let offset = |index: usize| -> (f64, f64) {
        let (before, after) = (headings[index.saturating_sub(1)], headings[index.min(headings.len() - 1)]);
        let (nx, ny) = (-(before.1 + after.1), before.0 + after.0);
        let length = nx.hypot(ny);
        if length < 1e-9 {
            return (-after.1 * half, after.0 * half);
        }
        let (nx, ny) = (nx / length, ny / length);
        let cosine = (nx * -after.1 + ny * after.0).max(MITRE_LIMIT);
        (nx * half / cosine, ny * half / cosine)
    };
    let right = points.iter().enumerate().map(|(index, point)| (point.0 - offset(index).0, point.1 - offset(index).1));
    let left = points.iter().enumerate().rev().map(|(index, point)| (point.0 + offset(index).0, point.1 + offset(index).1));
    right.chain(left).map(|(x, y)| PlanVertex { x, y, bulge: 0.0 }).collect()
}

fn drop_outline(value: &MepValue, segment: &MepSegment, along: (f64, f64)) -> Vec<PlanVertex> {
    let (x, y) = (segment.from.x, segment.from.y);
    match value.section.kind {
        MepSectionKind::Pipe => {
            let radius = value.section.width / 2.0;
            vec![PlanVertex { x: x + radius, y, bulge: 1.0 }, PlanVertex { x: x - radius, y, bulge: 1.0 }]
        }
        MepSectionKind::Duct | MepSectionKind::Tray => {
            let (across, depth) = (value.section.width / 2.0, value.section.height / 2.0);
            let (ux, uy) = along;
            let (rx, ry) = (-uy, ux);
            [(-across, -depth), (across, -depth), (across, depth), (-across, depth)].iter().map(|(a, b)| PlanVertex { x: x + a * rx + b * ux, y: y + a * ry + b * uy, bulge: 0.0 }).collect()
        }
    }
}

fn run_heading(value: &MepValue, at: usize) -> (f64, f64) {
    let near = |index: usize| value.segments.get(index).filter(|segment| !segment.vertical());
    let found = (0..at).rev().find_map(near).or_else(|| (at + 1..value.segments.len()).find_map(near));
    found.map_or((1.0, 0.0), heading)
}

/// 🌀️ Draws the MEP elements of the storey.
pub fn draw(sheet: &mut Sheet, cx: &Context<'_>) {
    for (id, value) in cx.inputs.meps.iter().filter(|(_, value)| value.storey == cx.storey && value.buildable()) {
        let style = outline_style(classify(value.bounds.min.z, value.bounds.max.z, cx.cut));
        let half = value.section.width / 2.0;
        for chain in chains(value) {
            let mut line: Vec<PlanVertex> = vec![PlanVertex { x: chain[0].from.x, y: chain[0].from.y, bulge: 0.0 }];
            line.extend(chain.iter().map(|segment| PlanVertex { x: segment.to.x, y: segment.to.y, bulge: 0.0 }));
            sheet.polyline(id, PlanKind::MepAxis, style, false, line);
            sheet.region(id, PlanKind::MepBand, style, band(&chain, half), Vec::new());
        }
        for (index, segment) in value.segments.iter().enumerate().filter(|(_, segment)| segment.vertical() && segment.length > EPS) {
            sheet.polyline(id, PlanKind::MepDrop, style, true, drop_outline(value, segment, run_heading(value, index)));
        }
    }
}

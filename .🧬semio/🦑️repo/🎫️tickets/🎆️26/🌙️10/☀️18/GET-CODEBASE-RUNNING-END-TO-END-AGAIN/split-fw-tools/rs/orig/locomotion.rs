//! 🏃️ Locomotion: how a pet gets around and how it loses its ground — setting out for a goal on its perch, striding, hopping and gliding between perches, falling, touching down, being crowded out, waiting for a partner and leaving.
//!
//! A new footing or a new way to travel belongs here: what starts it, its tick of motion and where it ends.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ./⚓️footing/🟦️.ts — the TypeScript twin, split into the child folders `📐️constants`, `⚓️footing`, `🛬️courses`, `🛫️departure`, `🏃️motion`, `✋️hand`, `🪜️outings`, `🧳️trips`, `🪄️mischief` and `🚪️leaving` next to this file

use crate::draft::{beat_of, clip_at, clip_of, hover_of, release, remove, settle, shift, shoulders, Draft, Launch, ORIGIN, RATE};
use crate::spacing::{clearway, hindered, soars, vacancy, COMFORT_GAP, MEET_GAP};
use semio_framework_pets_character::behavior::dwell_of;
use semio_framework_pets_contract::schema::{Activity, Footing, Gait, Perch, Point, Slug, Species, Ticks};
use semio_framework_pets_kinematics::animation::clip_ticks;
use semio_framework_pets_kinematics::terrain::{fall_step, hop_landing, hop_of, hop_step, landing_of, larger, perch_at, smaller, stride_to, HOP_DISTANCE, HOP_HEIGHT};
use semio_framework_pets_kinematics::trigonometry::clamp;

//#region 🔖️Constants
const PATIENCE: Ticks = 1920;
const LEAVE_REACH: f64 = 3.0;
const GLIDE_TICKS: Ticks = 256;
const CLAIM_SPAN: Ticks = 4;
const LANE_RAMP: f64 = 8.0;
const DIZZY_SPEED: f64 = 780.0;
//#endregion 🔖️Constants

//#region 🔖️Footing
/// 🎟️ A complete course and its reserved corridor.
#[derive(Clone)]
pub(crate) struct Plan { course: Course, claim: Claim }

/// 🧾️ Find an owner's planned flight.
pub(crate) fn course_of<'a>(courses: &'a [Course], owner: &str) -> Option<&'a Course> {
    courses.iter().find(|course| course.owner == owner)
}

/// 🎫️ Claim every course waypoint and its resting body in four-tick slices.
fn plan_of(draft: &Draft<'_>, index: usize, course: Course) -> Plan {
    let kind = draft.kinds[index];
    let extents: Vec<_> = course.steps.iter().map(|step| extent_at(&course.owner, kind, Point { x: step.x, y: step.y }, step.tilt, step.canopy.map(|canopy| Point { x: canopy.x, y: canopy.y }))).collect();
    let rest = if course.ending == Ending::Away { None } else { extents.last().copied() };
    let claim = claim_of(&course.owner, course.from, &extents, CLAIM_SPAN, rest);
    Plan { course, claim }
}

/// 🛫️ Installing a validated course reserves its corridor and commits the flight.
fn install(draft: &mut Draft<'_>, index: usize, plan: Plan, activity: Activity, now: Ticks) {
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.claims.push(plan.claim);
    draft.stage.courses.retain(|course| course.owner != owner);
    let until = plan.course.from + plan.course.steps.len() as Ticks - 1;
    let goal = plan.course.steps.last().unwrap().x;
    draft.stage.courses.push(plan.course);
    if draft.stage.actors[index].activity != activity { shift(draft, index, activity, now); draft.stage.actors[index].clip = clip_at(draft.kinds[index], activity, 0.0); }
    let body = &mut draft.stage.actors[index];
    body.footing = if body.chute.is_none() { Footing::Air } else { Footing::Chute };
    body.perch = None;
    body.host = None;
    body.hang = None;
    body.until = until;
    body.goal = goal;
}
/// 🏡️ An actor that has nowhere left to be is gone at once (always `false`): it lets go of its partner and arrives anew, spaced from the others, once it is wanted and a perch has room.
pub(crate) fn vanish(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    release(draft, index, now);
    remove(draft, index);
    false
}

/// 🕳️ An actor loses its perch and starts to fall.
pub(crate) fn drop(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    unfoot(draft, index, 0.0, 0.0, Activity::Fall, now);
}

/// 🕳️ Release a footing, corridor and course before planning the way down.
pub(crate) fn unfoot(draft: &mut Draft<'_>, index: usize, vx: f64, vy: f64, activity: Activity, now: Ticks) {
    release(draft, index, now);
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.courses.retain(|course| course.owner != owner);
    ungear(draft, index, now);
    if draft.stage.actors[index].activity != activity {
        shift(draft, index, activity, now);
        draft.stage.actors[index].clip = clip_at(draft.kinds[index], activity, 0.0);
    }
    let body = &mut draft.stage.actors[index];
    body.perch = None;
    body.footing = if body.chute.is_none() { Footing::Air } else { Footing::Chute };
    body.host = None;
    body.hang = None;
    body.vx = vx;
    body.vy = vy;
    body.goal = body.x;
    body.until = now + DELAY;
}

//#endregion 🔖️Footing

//#region 🔖️Courses
/// 🪟️ Distance above the first footprint-sized perch or motionless head below the feet.
fn headroom(perches: &[Perch], statics: &[Solid], x: f64, y: f64, hover: f64) -> f64 {
    let mut least = f64::INFINITY;
    for perch in perches { if perch.x0 <= x && x <= perch.x1 && perch.y - hover >= y && perch.y - hover - y < least { least = perch.y - hover - y; } }
    for body in statics { let top = body.extent.y0 - hover - MARGIN; if body.extent.x0 < x && x < body.extent.x1 && top >= y && top - y < least { least = top - y; } }
    least
}

/// 🧭️ Chart an entire fall, bounce or parachute descent through its exact landing.
fn charted(draft: &Draft<'_>, index: usize, start: Waypoint, from: Ticks, decision: Ticks, target: f64, statics: &[Solid]) -> Course {
    let kind = draft.kinds[index];
    let owner = draft.stage.actors[index].species.clone();
    let half = kind.size.width / 2.0;
    let hover = hover_of(kind);
    let chute = chute_of(kind.size.height);
    let phase = 0.37 * draft.streams[index] as f64;
    let perches: Vec<_> = draft.stage.perches.iter().filter(|perch| perch.x0 + half <= perch.x1 - half).map(|perch| Perch { surface: perch.surface.clone(), x0: perch.x0 + half, x1: perch.x1 - half, y: perch.y }).collect();
    let mut steps = Vec::new();
    let (mut x, mut y, mut vx, mut vy, mut canopy) = (start.x, start.y, start.vx, start.vy, start.canopy);
    let mut before = extent_at(&owner, kind, Point { x, y }, start.tilt, canopy.map(|canopy| Point { x: canopy.x, y: canopy.y }));
    for tick in 0..1024 {
        if canopy.is_none() && decision >= 0 && tick >= decision + CHUTE_REFLEX { canopy = Some(canopy_of(Point { x, y }, vx, vy, chute)); }
        let mut next = canopy.map(|canopy| chute_step(canopy, chute, target, headroom(&perches, statics, x, y, hover), chute_wind(from + tick, phase)));
        let (mut next_x, mut next_y, mut next_vx, mut next_vy) = if let Some(next) = next { (next.bob.x, next.bob.y, (next.bob.x - x) * RATE, (next.bob.y - y) * RATE) } else { let fallen = fall_step(y, vy); (x + vx / RATE, fallen.y, vx, fallen.vy) };
        let held = clamp(next_x, half, draft.stage.width - half);
        if held != next_x {
            let push = held - next_x;
            next_x = held;
            next_vx = if next.is_none() { -next_vx * 0.5 } else { 0.0 };
            if let Some(canopy) = &mut next { canopy.x += push; canopy.vx = 0.0; canopy.bob.x += push; canopy.previous.x += push; }
        }
        if next.is_none() && next_y - kind.size.height < 0.0 { next_y = kind.size.height; if next_vy < 0.0 { next_vy = -next_vy * 0.5; } }
        let tilt = if tick + 1 >= 24 { 0.0 } else { start.tilt * (1.0 - smoothstep((tick + 1) as f64 / 24.0)) };
        let after = extent_at(&owner, kind, Point { x: next_x, y: next_y }, tilt, next.map(|canopy| Point { x: canopy.x, y: canopy.y }));
        let perch = landing_of(&perches, next_x, y + hover, next_y + hover);
        let host = head_under(before, after, &owner, statics);
        if perch.is_some() || host.is_some() {
            let on_perch = perch.map_or(f64::INFINITY, |perch| perch.y - hover);
            let on_head = host.map_or(f64::INFINITY, |host| next_y + lift_onto(extent_at(&owner, kind, Point { x: next_x, y: next_y }, 0.0, None), host.extent));
            let head = host.is_some() && on_head <= on_perch;
            steps.push(Waypoint { x: next_x, y: if head { on_head } else { on_perch }, vx: 0.0, vy: 0.0, tilt: 0.0, canopy: None });
            return Course { owner, from, steps, ending: if head { Ending::Head } else { Ending::Perch }, landing: if head { host.unwrap().owner.clone() } else { perch.unwrap().surface.clone() }, touch: next_vy };
        }
        if next_y > draft.stage.height {
            steps.push(Waypoint { x: next_x, y: draft.stage.height, vx: next_vx, vy: next_vy, tilt, canopy: next });
            return Course { owner, from, steps, ending: Ending::Away, landing: String::new(), touch: next_vy };
        }
        steps.push(Waypoint { x: next_x, y: next_y, vx: next_vx, vy: next_vy, tilt, canopy: next });
        (x, y, vx, vy, canopy, before) = (next_x, next_y, next_vx, next_vy, next, after);
    }
    Course { owner, from, steps, ending: Ending::Away, landing: String::new(), touch: vy }
}

/// 🪂️ Decide when the actual plain landing warrants opening an owned parachute.
fn decision_of(kind: &Species, start: Waypoint, plain: &Course) -> Ticks {
    if !kind.gear.contains(&Gear::Parachute) || start.canopy.is_some() || plain.ending == Ending::Away || plain.touch <= HARD_LANDING { return -1; }
    let ground = plain.steps.last().unwrap().y;
    for opening in [CHUTE_OPENING, 0.0] {
        for tick in 0..plain.steps.len() {
            let state = if tick == 0 { &start } else { &plain.steps[tick - 1] };
            if state.vy >= opening && ground - state.y >= CHUTE_HEADROOM { return tick as Ticks; }
        }
    }
    -1
}

/// 🍂️ Chart a descent, opening its chute or braking a downward throw when necessary.
fn way_down(draft: &Draft<'_>, index: usize, start: Waypoint, from: Ticks, statics: &[Solid], target: f64) -> Plan {
    let kind = draft.kinds[index];
    let mut begin = start;
    for brace in 0..=4 {
        let plain = charted(draft, index, begin, from, -1, target, statics);
        let decision = decision_of(kind, begin, &plain);
        let course = if decision < 0 { plain } else { charted(draft, index, begin, from, decision, plain.steps.last().unwrap().x, statics) };
        if brace >= 4 || !kind.gear.contains(&Gear::Parachute) || begin.canopy.is_some() || course.ending == Ending::Away || course.touch <= HARD_LANDING || begin.vy <= 0.0 { return plan_of(draft, index, course); }
        begin.vy = if brace + 1 >= 4 { 0.0 } else { begin.vy / 2.0 };
        begin.canopy = None;
    }
    unreachable!()
}

/// 🧗️ Try straight, column-guided and steered descents in contract order.
fn ways_down(draft: &Draft<'_>, index: usize, from: Ticks) -> Option<Plan> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let bodies = bodies_of(&draft.stage.actors, &draft.kinds);
    let statics: Vec<_> = draft.stage.actors.iter().enumerate().filter(|(other, body)| *other != index && body.footing == Footing::Perch && !draft.stage.claims.iter().any(|claim| claim.owner == body.species)).map(|(other, _)| bodies[other].clone()).collect();
    let start = Waypoint { x: body.x, y: body.y, vx: body.vx, vy: body.vy, tilt: body.tilt, canopy: body.chute.map(|chute| chute.canopy) };
    let straight = way_down(draft, index, start, from, &statics, body.x);
    if claim_clear(&straight.claim, &bodies, &draft.stage.claims) { return Some(straight); }
    let mut tries = Vec::new();
    if start.canopy.is_some() {
        for steer in &STEERINGS[1..] { tries.push((start, body.x + steer)); }
    } else {
        let course = &straight.course;
        let end = course.steps.last().unwrap();
        if let Some(perch) = (course.ending == Ending::Perch).then(|| perch_at(&draft.stage.perches, &course.landing, end.x)).flatten() {
            let column = column_over(perch, body.x, extent_at(&body.species, kind, Point { x: body.x, y: body.y }, 0.0, None), perch.y - hover_of(kind) - body.y, kind.size.width / 2.0, &obstacles_of(&body.species, &bodies, &draft.stage.claims, from));
            let steer = column.map_or(0.0, |column| clamp((column - body.x) * RATE / course.steps.len() as f64, -240.0, 240.0));
            if steer != 0.0 { tries.push((Waypoint { vx: body.vx + steer, ..start }, body.x)); }
        }
        for steer in &STEERINGS[1..] { tries.push((Waypoint { vx: body.vx + steer, ..start }, body.x)); }
        if body.vx != 0.0 || body.vy != 0.0 {
            tries.push((Waypoint { vx: -body.vx / 2.0, ..start }, body.x));
            for steer in STEERINGS { tries.push((Waypoint { vx: steer, vy: 0.0, ..start }, body.x)); }
        }
    }
    for (start, target) in tries { let plan = way_down(draft, index, start, from, &statics, target); if claim_clear(&plan.claim, &bodies, &draft.stage.claims) { return Some(plan); } }
    None
}
//#endregion 🔖️Courses

//#region 🔖️Departure
/// 🥾️ Where a walk from `x` towards `goal` really ends: a hopping gait covers ground in whole hops of its clip (speed × the length of the clip), as many as fit before the goal; every other gait goes all the way.
pub(crate) fn paced(kind: &Species, clip: Option<&str>, x: f64, goal: f64) -> f64 {
    let beat = beat_of(kind, clip_of(kind, clip));
    if beat == 1 {
        return goal;
    }
    let hop = (larger(kind.locomotion.speed, 0.0) * beat as f64) / RATE;
    let leaps = hop > 0.0;
    if !leaps {
        return x;
    }
    let hops = ((goal - x).abs() / hop).floor();
    if goal >= x {
        x + hops * hop
    } else {
        x - hops * hop
    }
}

/// 🚶️ An actor sets out for a goal on its perch; it turns towards it first when it faces the other way.
pub(crate) fn stroll(draft: &mut Draft<'_>, index: usize, goal: f64, clip: Option<Slug>, now: Ticks) {
    shift(draft, index, Activity::Walk, now);
    let until = now + dwell_of(Activity::Walk, draft.stage.mode, 0.0);
    let body = &mut draft.stage.actors[index];
    body.goal = goal;
    body.until = until;
    body.clip = clip;
}

/// 🧍️ An actor waits for its partner: idle, turning towards it, for as long as the stage has patience.
pub(crate) fn attend(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    shift(draft, index, Activity::Idle, now);
    let clip = clip_at(draft.kinds[index], Activity::Idle, 0.0);
    let body = &mut draft.stage.actors[index];
    body.until = now + PATIENCE;
    body.clip = clip;
    body.goal = body.x;
}

/// 🦘️ Where an actor could hop to: per other perch (in perch order) the place nearest to it that keeps half a body from the ends and a comfortable gap from the actors there, when `hop_of` grants the hop, the flight really ends on that perch and its arc covers nothing that must stay free ([`soars`]); a floating gait glides there in a straight line at twice its speed when the place is within the reach of a hop and four seconds.
pub(crate) fn hops_of(draft: &Draft<'_>, index: usize) -> Vec<Launch> {
    let actors = &draft.stage.actors;
    let body = &actors[index];
    let kind = draft.kinds[index];
    let half = kind.size.width / 2.0;
    let hover = hover_of(kind);
    let obstacles = obstacles_for(draft, index, draft.stage.tick);
    let mut launches = Vec::new();
    for perch in &draft.stage.perches {
        if body.perch.as_deref() == Some(perch.surface.as_str()) && perch.x0 <= body.x && body.x <= perch.x1 {
            continue;
        }
        let low = perch.x0 + half;
        let high = perch.x1 - half;
        if high < low {
            continue;
        }
        let inset = smaller(half, (high - low) / 2.0);
        let aimed = clamp(body.x, low + inset, high - inset);
        let Some(x) = spot_on(perch, aimed, extent_at(&body.species, kind, Point { x: aimed, y: perch.y - hover }, 0.0, None), half, &obstacles) else { continue; };
        if kind.locomotion.gait == Gait::Float {
            let dx = x - body.x;
            let dy = perch.y - hover - body.y;
            if dx.abs() > HOP_DISTANCE || dy.abs() > HOP_HEIGHT {
                continue;
            }
            let ticks = (((dx * dx + dy * dy).sqrt() * RATE) / (2.0 * larger(kind.locomotion.speed, 1.0)) + 0.5).floor();
            if ticks < 1.0 || ticks > GLIDE_TICKS as f64 {
                continue;
            }
            launches.push(Launch { x, y: perch.y - hover, surface: perch.surface.clone(), vx: (dx * RATE) / ticks, vy: (dy * RATE) / ticks, ticks: ticks as Ticks });
            continue;
        }
        let from = Point { x: body.x, y: body.y };
        let to = Point { x, y: perch.y };
        let Some(hop) = hop_of(from, to) else {
            continue;
        };
        let Some(landing) = hop_landing(&draft.stage.perches, from, to, hop) else {
            continue;
        };
        if landing.surface != perch.surface || landing.x0 != perch.x0 || !soars(draft, kind, from, to, hop) {
            continue;
        }
        launches.push(Launch { x, y: perch.y, surface: perch.surface.clone(), vx: hop.vx, vy: hop.vy, ticks: hop.ticks });
    }
    launches
}

/// 🚀️ Commit a hop only when its entire arc or floating lane and landing remain clear.
pub(crate) fn launch(draft: &mut Draft<'_>, index: usize, chosen: &Launch, clip: Option<Slug>, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let mut courses = Vec::new();
    if kind.locomotion.gait == Gait::Float {
        for lane in [0.0, lane_lift(kind.size.height, MARGIN)] {
            let steps = (1..=chosen.ticks).map(|tick| {
                let ramp = smaller(1.0, smaller(tick as f64 / LANE_RAMP, (chosen.ticks - tick) as f64 / LANE_RAMP));
                if tick == chosen.ticks { Waypoint { x: chosen.x, y: chosen.y, vx: 0.0, vy: 0.0, tilt: 0.0, canopy: None } } else { Waypoint { x: body.x + ((chosen.x - body.x) * tick as f64) / chosen.ticks as f64, y: body.y + ((chosen.y - body.y) * tick as f64) / chosen.ticks as f64 - lane * ramp, vx: chosen.vx, vy: chosen.vy, tilt: 0.0, canopy: None } }
            }).collect();
            courses.push(Course { owner: body.species.clone(), from: now + 1, steps, ending: Ending::Perch, landing: chosen.surface.clone(), touch: 0.0 });
        }
    } else {
        let mut steps = Vec::new();
        let (mut x, mut y, mut vy) = (body.x, body.y, chosen.vy);
        for left in (1..=chosen.ticks).rev() {
            let flight = hop_step(x, y, chosen.vx, vy, Point { x: chosen.x, y: chosen.y }, left);
            steps.push(Waypoint { x: flight.x, y: flight.y, vx: flight.vx, vy: flight.vy, tilt: 0.0, canopy: None });
            (x, y, vy) = (flight.x, flight.y, flight.vy);
        }
        courses.push(Course { owner: body.species.clone(), from: now + 1, steps, ending: Ending::Perch, landing: chosen.surface.clone(), touch: vy });
    }
    let bodies = bodies_of(&draft.stage.actors, &draft.kinds);
    for course in courses {
        let plan = plan_of(draft, index, course);
        if !claim_clear(&plan.claim, &bodies, &draft.stage.claims) { continue; }
        install(draft, index, plan, Activity::Hop, now);
        draft.stage.actors[index].clip = clip;
        return true;
    }
    false
}
//#endregion 🔖️Departure

//#region 🔖️Motion
/// 👣️ One tick of a walk. On every beat — every tick for a walking or floating gait, the start of every hop for a hopping one — the walk ends when the goal is no farther than one stride (the actor stands on it then) or when somebody is in the way of the next beat; otherwise a stride towards the goal follows. A hopping gait that reaches its goal within a hop, or finds somebody in its way in mid-hop, finishes that hop on the spot. The walk also ends when the stage loses patience. At its end a leaver starts to fade, an actor with a partner waits for it and anyone else comes to rest.
pub(crate) fn stride(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    let body = &draft.stage.actors[index];
    let beat = beat_of(kind, clip_of(kind, body.clip.as_deref()));
    let step = larger(kind.locomotion.speed, 0.0) / RATE;
    let mut done = now >= body.until;
    if !done && (now - body.since) % beat == 0 {
        let gap = body.goal - body.x;
        if gap.abs() <= step {
            let allowed = guarded(draft, index, gap, now);
            let body = &mut draft.stage.actors[index];
            body.x = if allowed == gap { body.goal } else { body.x + allowed };
            done = true;
        } else {
            let far = step * beat as f64;
            let ahead = clamp(body.goal, body.x - far, body.x + far) - body.x;
            done = guarded(draft, index, ahead, now) != ahead;
        }
    }
    if !done {
        let body = &draft.stage.actors[index];
        let next = stride_to(body.x, body.goal, kind.locomotion.speed);
        let wanted = next - body.x;
        let allowed = guarded(draft, index, wanted, now);
        let body = &mut draft.stage.actors[index];
        if allowed == wanted { body.x = next; return; }
        body.x += allowed;
        body.goal = body.x;
        if beat > 1 { return; }
    }
    let body = &mut draft.stage.actors[index];
    if body.leaving {
        shift(draft, index, Activity::Idle, now);
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Idle, 0.0);
        body.goal = body.x;
        return;
    }
    if body.partner.is_some() {
        attend(draft, index, now);
        return;
    }
    settle(draft, index, now);
}

/// 🛡️ A stride respects every solid body and planned corridor through this tick.
fn guarded(draft: &Draft<'_>, index: usize, wanted: f64, now: Ticks) -> f64 {
    guarded_stride(extent_of(&draft.stage.actors[index], draft.kinds[index]), wanted, &obstacles_for(draft, index, now))
}

/// 🛷️ Slide off a host's head through guarded positions, then plan a fall.
pub(crate) fn slip(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let host = body.host.as_deref().and_then(|host| index_of(&draft.stage.actors, host));
    let box_of = extent_of(body, draft.kinds[index]);
    if !host.is_some_and(|host| rests_on(box_of, extent_of(&draft.stage.actors[host], draft.kinds[host]))) {
        let vx = body.vx;
        unfoot(draft, index, vx, 0.0, Activity::Fall, now);
        return plunge(draft, index, now);
    }
    let side = if body.vx < 0.0 { Facing::Left } else { Facing::Right };
    let ticks = now - body.since;
    let slid = slide_of(box_of, side, ticks, 0.0, draft.stage.width, &obstacles_for(draft, index, now));
    let body = &mut draft.stage.actors[index];
    body.x += slid.stride;
    body.goal = body.x;
    let sign = if slid.side == Facing::Right { 1.0 } else { -1.0 };
    if slid.side == side { body.vx = sign * slide_stride(ticks) * RATE; } else { body.since = now; body.vx = sign * SLIDE_OFF_SPEED; }
    true
}

/// 🚌️ Move each surface's scooters together through free positions.
pub(crate) fn scoot(draft: &mut Draft<'_>, now: Ticks) {
    let mut surfaces = Vec::new();
    for body in &draft.stage.actors {
        if body.activity == Activity::Scoot && body.footing == Footing::Perch {
            if let Some(surface) = &body.perch { if !surfaces.contains(surface) { surfaces.push(surface.clone()); } }
        }
    }
    for surface in surfaces {
        let members: Vec<_> = draft.stage.actors.iter().enumerate().filter_map(|(index, body)| (body.activity == Activity::Scoot && body.footing == Footing::Perch && body.perch.as_deref() == Some(&surface)).then_some(index)).collect();
        if members.is_empty() { continue; }
        let shifts: Vec<_> = members.iter().map(|&member| draft.stage.actors[member].goal - draft.stage.actors[member].x).collect();
        let slowest = members.iter().map(|&member| larger(draft.kinds[member].locomotion.speed, 1.0)).fold(f64::INFINITY, smaller);
        let fraction = scoot_fraction(&shifts, SCOOT_HASTE * slowest / RATE);
        let bodies = bodies_of(&draft.stage.actors, &draft.kinds);
        let others: Vec<_> = bodies.iter().enumerate().filter(|(index, _)| !members.contains(index)).map(|(_, body)| body.clone()).collect();
        let obstacles = obstacles_of("", &others, &draft.stage.claims, now);
        let places: Vec<_> = members.iter().map(|&member| scooted(draft.stage.actors[member].x, draft.stage.actors[member].goal, fraction)).collect();
        let free = members.iter().zip(&places).all(|(&member, &place)| free_among(shifted(bodies[member].extent, place - draft.stage.actors[member].x, 0.0), &obstacles));
        if free {
            for (&member, &place) in members.iter().zip(&places) { draft.stage.actors[member].x = place; }
            if fraction >= 1.0 { for &member in &members { settle(draft, member, now); } }
            continue;
        }
        if now < draft.stage.actors[members[0]].until { continue; }
        for &member in members.iter().rev() {
            if perch_at(&draft.stage.perches, &surface, draft.stage.actors[member].x).is_some() { settle(draft, member, now); } else { drop(draft, member, now); }
        }
    }
}

/// 🌬️ Plan the first legal descent or wait in a free place before the last-resort poof.
pub(crate) fn plunge(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    if let Some(plan) = ways_down(draft, index, now) {
        let activity = draft.stage.actors[index].activity;
        install(draft, index, plan, activity, now);
        return fly(draft, index, now);
    }
    let body = &draft.stage.actors[index];
    let stay = (now < body.until).then(|| extent_of(body, draft.kinds[index]));
    if !must_poof(&body.species, stay, &[], &bodies_of(&draft.stage.actors, &draft.kinds), &draft.stage.claims, now) { return true; }
    poof(draft, index, now)
}

/// 🛩️ Follow the next owned course waypoint and complete its reserved landing.
pub(crate) fn fly(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let Some(course) = course_of(&draft.stage.courses, &draft.stage.actors[index].species) else { return plunge(draft, index, now); };
    let last = course.steps.len() - 1;
    let at = now - course.from;
    if at < 0 { return true; }
    let step = course.steps[if at > last as Ticks { last } else { at as usize }];
    let body = &draft.stage.actors[index];
    if at <= last as Ticks && bounced(draft, index, &step, body.vx, body.vy) { feel(draft, index, Occasion::Bonked, now); }
    let body = &mut draft.stage.actors[index];
    (body.x, body.y, body.vx, body.vy, body.tilt) = (step.x, step.y, step.vx, step.vy, step.tilt);
    if let Some(canopy) = step.canopy {
        if let Some(chute) = body.chute {
            let spring = spring_step(chute.open, chute.opening, 1.0, 220.0, 12.0);
            body.chute = Some(Parachute { since: chute.since, open: spring.position, opening: spring.velocity, canopy });
        } else {
            body.chute = Some(Parachute { since: now, open: 0.0, opening: 0.0, canopy });
            body.footing = Footing::Chute;
            if body.activity != Activity::Glide { shift(draft, index, Activity::Glide, now); draft.stage.actors[index].clip = clip_at(draft.kinds[index], Activity::Glide, 0.0); }
        }
    }
    if at < last as Ticks { true } else { let course = course_of(&draft.stage.courses, &draft.stage.actors[index].species).unwrap().clone(); arrive(draft, index, &course, now) }
}

/// 🤕️ A free-flight waypoint bounces when an edge reverses its velocity.
fn bounced(draft: &Draft<'_>, index: usize, step: &Waypoint, vx: f64, vy: f64) -> bool {
    let kind = draft.kinds[index];
    let half = kind.size.width / 2.0;
    step.canopy.is_none() && (((step.x == half || step.x == draft.stage.width - half) && vx * step.vx < 0.0) || (step.y == kind.size.height && vy < 0.0 && step.vy >= 0.0))
}

/// 🌪️ Every disappearance is counted and leaves dust at the actor's former body.
fn dusted(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    draft.stage.puffs.push(Puff { x: body.x, y: body.y - kind.size.height / 2.0, width: kind.size.width, height: kind.size.height, tick: now });
    draft.stage.poofs += 1;
}

/// 💨️ Remove a body that has no legal place and count its dust puff.
pub(crate) fn poof(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    dusted(draft, index, now);
    vanish(draft, index, now)
}

/// 🛬️ A committed course ends exactly on its reserved perch or head, or leaves the stage.
fn arrive(draft: &mut Draft<'_>, index: usize, course: &Course, now: Ticks) -> bool {
    let kind = draft.kinds[index];
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.courses.retain(|course| course.owner != owner);
    if course.ending == Ending::Away { dusted(draft, index, now); return vanish(draft, index, now); }
    let body = &mut draft.stage.actors[index];
    let flown = matches!(body.activity, Activity::Fall | Activity::Tumble | Activity::Glide);
    let floated = body.chute.is_some();
    body.chute = None;
    body.tilt = 0.0;
    body.vx = 0.0;
    body.vy = 0.0;
    body.goal = body.x;
    if course.ending == Ending::Head {
        let host = index_of(&draft.stage.actors, &course.landing);
        shift(draft, index, Activity::Slide, now);
        let vx = if host.is_some_and(|host| draft.stage.actors[index].x < draft.stage.actors[host].x) { -SLIDE_OFF_SPEED } else { SLIDE_OFF_SPEED };
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Slide, 0.0);
        body.footing = Footing::Head;
        body.perch = None;
        body.host = Some(course.landing.clone());
        body.until = now + dwell_of(Activity::Slide, draft.stage.mode, 0.0);
        body.vx = vx;
        if let Some(host) = host { feel(draft, host, Occasion::Trampled, now); }
        return true;
    }
    body.footing = Footing::Perch;
    body.perch = Some(course.landing.clone());
    body.host = None;
    if floated { settle(draft, index, now); if flown { feel(draft, index, Occasion::Floated, now); } return true; }
    shift(draft, index, Activity::Land, now);
    let clip = clip_at(kind, Activity::Land, 0.0);
    let until = now + clip_of(kind, clip.as_deref()).filter(|clip| !clip.looping).map_or_else(|| dwell_of(Activity::Land, draft.stage.mode, 0.0), clip_ticks);
    let body = &mut draft.stage.actors[index];
    body.clip = clip;
    body.until = until;
    if !flown { return true; }
    let hard = course.touch > HARD_LANDING;
    let bumped = course.steps.windows(2).any(|steps| bounced(draft, index, &steps[1], steps[0].vx, steps[0].vy) && (if steps[0].vx * steps[1].vx < 0.0 { steps[0].vx.abs() } else { steps[0].vy.abs() }) >= DIZZY_SPEED);
    draft.stage.actors[index].vy = if hard && course.touch >= DIZZY_SPEED { course.touch } else if bumped { DIZZY_SPEED } else { 0.0 };
    feel(draft, index, if hard { Occasion::Dropped } else { Occasion::Landed }, now);
    true
}

/// 😵️ A hard landing or edge collision leaves a short authored dizzy activity.
pub(crate) fn daze(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    shift(draft, index, Activity::Dizzy, now);
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(draft.kinds[index], Activity::Dizzy, 0.0);
    body.until = now + dwell_of(Activity::Dizzy, draft.stage.mode, 0.0);
    body.vx = 0.0;
    body.vy = 0.0;
}
//#endregion 🔖️Motion

//#region 🔖️Hand
/// 🚦️ A body removed from its plan makes overlapping corridor owners plan again.
fn yielded(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let owner = draft.stage.actors[index].species.clone();
    let box_of = extent_of(&draft.stage.actors[index], draft.kinds[index]);
    for other in 0..draft.stage.actors.len() {
        if other == index { continue; }
        let body = &draft.stage.actors[other];
        let own: Vec<_> = draft.stage.claims.iter().filter(|claim| claim.owner == body.species).cloned().collect();
        if own.is_empty() || free_among(box_of, &obstacles_of(&owner, &[], &own, now)) { continue; }
        let (vx, vy, activity) = (body.vx, body.vy, body.activity);
        unfoot(draft, other, vx, vy, activity, now);
    }
}

/// ✋️ Lift a pet by its scruff and remember its perch origin for cancellation.
pub(crate) fn lift(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    release(draft, index, now);
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.courses.retain(|course| course.owner != owner);
    ungear(draft, index, now);
    let body = &draft.stage.actors[index];
    draft.stage.origin = (body.footing == Footing::Perch).then_some(Point { x: body.x, y: body.y });
    draft.stage.trail.clear();
    if body.activity != Activity::Hang { shift(draft, index, Activity::Hang, now); }
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(kind, Activity::Hang, 0.0);
    body.footing = Footing::Hand;
    body.perch = None;
    body.host = None;
    body.chute = None;
    body.tilt = 0.0;
    body.hang = Some(hang_of(Point { x: body.x, y: body.y }, kind.grip));
    body.vx = 0.0;
    body.vy = 0.0;
    body.goal = body.x;
    body.until = now + dwell_of(Activity::Hang, draft.stage.mode, 0.0);
    yielded(draft, index, now);
}

/// 🦧️ Swing a held body below its grip, pushing it away from occupied extents.
pub(crate) fn hold(draft: &mut Draft<'_>, index: usize, target: Point, now: Ticks) {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let Some(hang) = body.hang else { return; };
    let next = hang_step(hang, Point { x: clamp(target.x, 0.0, draft.stage.width), y: clamp(target.y, 0.0, draft.stage.height) }, kind.grip);
    let tilt = lean_of(Point { x: next.grip.x, y: next.grip.y }, next.bob, kind.grip);
    let box_of = extent_at(&body.species, kind, next.bob, tilt, None);
    let push = pushed_out(box_of, &obstacles_for(draft, index, now), PUSHES);
    let valid = push.is_some_and(|push| {
        let moved = shifted(box_of, push.x, push.y);
        let half = kind.size.width / 2.0;
        moved.x0 + MARGIN >= 0.0 && moved.x1 - MARGIN <= draft.stage.width && moved.y0 + MARGIN >= 0.0 && moved.y1 - MARGIN <= draft.stage.height && next.bob.x + push.x - half >= 0.0 && next.bob.x + push.x + half <= draft.stage.width && next.bob.y + push.y - kind.size.height >= 0.0 && next.bob.y + push.y <= draft.stage.height
    });
    let body = &mut draft.stage.actors[index];
    if !valid { body.hang = Some(Hang { grip: Grip { vx: 0.0, vy: 0.0, ..hang.grip }, bob: hang.bob, previous: hang.bob }); return; }
    let push = push.unwrap();
    body.hang = Some(Hang { grip: Grip { x: next.grip.x + push.x, y: next.grip.y + push.y, ..next.grip }, bob: Point { x: next.bob.x + push.x, y: next.bob.y + push.y }, previous: Point { x: next.previous.x + push.x, y: next.previous.y + push.y } });
    body.x = next.bob.x + push.x;
    body.y = next.bob.y + push.y;
    body.tilt = tilt;
}

/// 🎈️ Release a held pet with the measured pointer velocity.
pub(crate) fn let_go(draft: &mut Draft<'_>, index: usize, velocity: Point, now: Ticks) {
    draft.stage.trail.clear();
    unfoot(draft, index, velocity.x, velocity.y, Activity::Tumble, now);
}

/// ↩️ Return a cancelled pick-up along a clear eased corridor to its nearest free origin.
pub(crate) fn give_back(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let origin = draft.stage.origin.take();
    let perch = origin.and_then(|origin| draft.stage.perches.iter().find(|perch| perch.y - hover_of(kind) == origin.y && perch.x0 <= origin.x && origin.x <= perch.x1));
    let spot = origin.zip(perch).and_then(|(origin, perch)| spot_on(perch, origin.x, extent_at(&body.species, kind, origin, 0.0, None), kind.size.width / 2.0, &obstacles_for(draft, index, now)));
    let Some((origin, perch, spot)) = origin.zip(perch).zip(spot).map(|((origin, perch), spot)| (origin, perch, spot)) else { let_go(draft, index, ORIGIN, now); return; };
    let (dx, dy) = (spot - body.x, origin.y - body.y);
    let ticks = std::cmp::max(((dx * dx + dy * dy).sqrt() * RATE / 384.0).floor() as Ticks + 1, 12);
    let steps = (1..=ticks).map(|tick| {
        let share = smoothstep(tick as f64 / ticks as f64);
        if tick == ticks { Waypoint { x: spot, y: origin.y, vx: 0.0, vy: 0.0, tilt: 0.0, canopy: None } } else { Waypoint { x: body.x + dx * share, y: body.y + dy * share, vx: 0.0, vy: 0.0, tilt: body.tilt * (1.0 - share), canopy: None } }
    }).collect();
    let plan = plan_of(draft, index, Course { owner: body.species.clone(), from: now + 1, steps, ending: Ending::Perch, landing: perch.surface.clone(), touch: 0.0 });
    if !claim_clear(&plan.claim, &bodies_of(&draft.stage.actors, &draft.kinds), &draft.stage.claims) { let_go(draft, index, ORIGIN, now); return; }
    draft.stage.trail.clear();
    release(draft, index, now);
    install(draft, index, plan, Activity::Glide, now);
}

/// 🎯️ Carry a tossed pet's body sixteen pixels below the upper stage edge.
pub(crate) fn toss_target(draft: &Draft<'_>, index: usize) -> Point {
    Point { x: draft.stage.actors[index].goal, y: 16.0 + draft.kinds[index].size.height - draft.kinds[index].grip }
}

/// 🎾️ A keyboard toss carries a lifted pet up for ninety-six ticks before release.
pub(crate) fn toss(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    lift(draft, index, now);
    draft.stage.origin = None;
    draft.stage.actors[index].until = now + 96;
}
//#endregion 🔖️Hand

//#region 🔖️Gear
/// ⛺️ A planned gear trip and the ladder or rope it installs.
#[derive(Clone)]
pub(crate) struct Outing { trip: Trip, raise: Option<LadderStand>, stands: Ticks, rope: Option<Rope> }

/// 🧿️ A destination on a perch or a chosen height along a wall.
#[derive(Clone)]
pub(crate) enum Venture { Perch(Perch), Wall { pitch: Pitch, y: Option<f64> } }

/// 🧩️ The stance where a gear leg begins.
struct Stand { perch: Perch, x: f64, y: f64, facing: Facing, grip: f64 }

/// 🎒️ Gear moves non-floating species along walls, ladders and grappling ropes.
pub(crate) fn geared(kind: &Species) -> bool {
    kind.locomotion.gait != Gait::Float && kind.gear.iter().any(|gear| matches!(gear, Gear::Climb | Gear::Ladder | Gear::Grapple))
}

/// 🗻️ Find a trip by its owning species.
pub(crate) fn trip_of<'a>(trips: &'a [Trip], owner: &str) -> Option<&'a Trip> { trips.iter().find(|trip| trip.owner == owner) }

/// 🪺️ The interval where the complete footprint fits on a perch.
fn room_on(perch: &Perch, half: f64) -> Option<[f64; 2]> { (perch.x0 + half <= perch.x1 - half).then_some([perch.x0 + half, perch.x1 - half]) }

/// 🧲️ Face towards the wall, opposite its outward side.
fn wallward(side: Facing) -> Facing { if side == Facing::Right { Facing::Left } else { Facing::Right } }

/// 💸️ Hanging and climbing consume their own grip budget per wall foothold.
fn effort_of(step: &Foothold) -> f64 { if step.footing != Footing::Wall { 0.0 } else if step.activity == Activity::Slide { GRIP_HANG } else { GRIP_CLIMB } }

/// 🔋️ Hanging consumes the grip anchored at the beginning of the rest.
fn grip_now(body: &semio_framework_pets_contract::schema::Actor, now: Ticks) -> f64 { larger(body.grip - GRIP_HANG * (now - body.since) as f64, 0.0) }

/// 🚸️ Plan the walk to gear, including the initial turn and exact final stride.
fn walked(stand: &Stand, kind: &Species, to: f64, activity: Activity) -> Option<Vec<Foothold>> {
    let mut steps = Vec::new();
    if to == stand.x { return Some(steps); }
    if kind.locomotion.speed <= 0.0 { return None; }
    let facing = facing_to(to, stand.x);
    if facing != stand.facing { for _ in 0..TURN_TICKS { steps.push(Foothold { x: stand.x, y: stand.y, footing: Footing::Perch, perch: Some(stand.perch.surface.clone()), activity, facing, hold: -1 }); } }
    let mut x = stand.x;
    for _ in 0..4096 {
        x = stride_to(x, to, kind.locomotion.speed);
        steps.push(Foothold { x, y: stand.y, footing: Footing::Perch, perch: Some(stand.perch.surface.clone()), activity, facing, hold: -1 });
        if x == to { return Some(steps); }
    }
    None
}

/// 🪵️ Append the exact eased movement along a ladder's rails.
fn rungs(stand: &LadderStand, travel: f64, goal: f64, steps: &mut Vec<Foothold>) {
    let mut along = travel;
    for tick in 0..4096 {
        if along == goal { break; }
        along = ladder_step(along, goal, tick);
        let feet = ladder_at(stand, along);
        steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing: wallward(stand.side), hold: -1 });
    }
}

/// 🧱️ Plan a wall line with its initial grip, mantle or foot exit and exact grip cost.
fn scaling(draft: &Draft<'_>, index: usize, lead: &[Foothold], chain: &[&Pitch], start: usize, entry: Entry, x: f64, y: f64, end: usize, goal: f64, mantle: bool, foot: Option<&Perch>, grip: f64, from: Ticks) -> Option<Outing> {
    let kind = draft.kinds[index];
    let path = wall_path(chain, start, entry, x, y, end, goal, mantle, kind.size);
    let mut steps = lead.to_vec();
    for clamber in &path { steps.push(Foothold { x: clamber.x, y: clamber.y, footing: Footing::Wall, perch: None, activity: if clamber.work == Handwork::Mantle { Activity::Mantle } else { Activity::Climb }, facing: wallward(chain[clamber.hold].side), hold: clamber.hold as i64 }); }
    let (mut ending, mut landing) = (Arrival::Wall, String::new());
    let mut cost = wall_cost(&path);
    if mantle {
        let top = rim_for(chain[end], &draft.stage.perches, kind.size)?;
        let room = room_on(top, kind.size.width / 2.0)?;
        let ledge = ledge_of(chain[end], kind.size);
        if ledge < room[0] || ledge > room[1] { return None; }
        ending = Arrival::Perch;
        landing = top.surface.clone();
    } else if let Some(foot) = foot {
        let room = room_on(foot, kind.size.width / 2.0)?;
        let off = cling_of(chain[end], kind.size);
        let to = clamp(off, room[0], room[1]);
        if to != off {
            for tick in 1..=WALL_GRAB_TICKS { steps.push(Foothold { x: if tick == WALL_GRAB_TICKS { to } else { off + (to - off) * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64) }, y: foot.y, footing: Footing::Wall, perch: None, activity: Activity::Climb, facing: wallward(chain[end].side), hold: end as i64 }); }
            cost += WALL_GRAB_TICKS as f64 * GRIP_CLIMB;
        }
        ending = Arrival::Perch;
        landing = foot.surface.clone();
    }
    if steps.is_empty() || cost > grip { return None; }
    Some(Outing { trip: Trip { owner: draft.stage.actors[index].species.clone(), from, steps, pitches: chain.iter().map(|pitch| (*pitch).clone()).collect(), ladder: None, ending, landing, grip: grip - cost }, raise: None, stands: 0, rope: None })
}

/// 🏔️ Walk to a wall hold and continue along its planned wall line.
fn from_perch(draft: &Draft<'_>, index: usize, stand: &Stand, chain: &[&Pitch], start: usize, hold: WallHold, end: usize, goal: f64, mantle: bool, foot: Option<&Perch>, from: Ticks) -> Option<Outing> {
    let kind = draft.kinds[index];
    let room = room_on(&stand.perch, kind.size.width / 2.0)?;
    let at = if hold.over { ledge_of(chain[start], kind.size) } else { clamp(hold.x, room[0], room[1]) };
    if at < room[0] || at > room[1] { return None; }
    let lead = walked(stand, kind, at, Activity::Walk)?;
    scaling(draft, index, &lead, chain, start, if hold.over { Entry::Hang } else { Entry::Grab }, at, stand.y, end, goal, mantle, foot, stand.grip, from)
}

/// 🏗️ Carry, raise, mount and climb a ladder through its exit.
fn ladder_lead(draft: &Draft<'_>, index: usize, stand: &Stand, ladder: &LadderStand, raise: bool, from: Ticks) -> Option<(Vec<Foothold>, Ticks)> {
    let kind = draft.kinds[index];
    let room = room_on(&stand.perch, kind.size.width / 2.0)?;
    let at = clamp(ladder.foot.x, room[0], room[1]);
    let mut steps = walked(stand, kind, at, if raise { Activity::Carry } else { Activity::Walk })?;
    let facing = wallward(ladder.side);
    let mut stands = 0;
    if raise {
        for _ in 0..LADDER_RAISE_TICKS { steps.push(Foothold { x: at, y: stand.y, footing: Footing::Perch, perch: Some(stand.perch.surface.clone()), activity: Activity::Carry, facing, hold: -1 }); }
        stands = from + steps.len() as Ticks;
    }
    for tick in 1..=LADDER_MOUNT_TICKS { steps.push(Foothold { x: if tick == LADDER_MOUNT_TICKS { ladder.foot.x } else { at + (ladder.foot.x - at) * smoothstep(tick as f64 / LADDER_MOUNT_TICKS as f64) }, y: ladder.foot.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); }
    rungs(ladder, 0.0, ladder_exit(ladder, kind.size), &mut steps);
    Some((steps, stands))
}

/// 🪤️ Raise a ladder as the first leg of a climb onto its wall line.
fn lean_outing(draft: &Draft<'_>, index: usize, stand: &Stand, chain: &[&Pitch], start: usize, lean: &LadderStand, end: usize, goal: f64, mantle: bool, foot: Option<&Perch>, from: Ticks) -> Option<Outing> {
    let (steps, stands) = ladder_lead(draft, index, stand, lean, true, from)?;
    let exit = ladder_at(lean, ladder_exit(lean, draft.kinds[index].size));
    let mut outing = scaling(draft, index, &steps, chain, start, Entry::Grab, exit.x, exit.y, end, goal, mantle, foot, stand.grip, from)?;
    outing.trip.ladder = Some(draft.stage.actors[index].species.clone());
    outing.raise = Some(lean.clone());
    outing.stands = stands;
    Some(outing)
}

/// 🪜️ Plan an ascent or descent over a standing or newly raised ladder.
fn ladder_outing(draft: &Draft<'_>, index: usize, stand: &Stand, ladder: &LadderStand, owner: &str, up: bool, to: &Perch, raise: bool, from: Ticks) -> Option<Outing> {
    let kind = draft.kinds[index];
    let start = room_on(&stand.perch, kind.size.width / 2.0)?;
    let goal = room_on(to, kind.size.width / 2.0)?;
    let exit = ladder_exit(ladder, kind.size);
    let landing = ladder_landing(ladder, kind.size);
    let top = ladder_at(ladder, exit);
    let facing = wallward(ladder.side);
    let mut steps;
    let mut stands = 0;
    if up {
        if landing.x < goal[0] || landing.x > goal[1] { return None; }
        (steps, stands) = ladder_lead(draft, index, stand, ladder, raise, from)?;
        for tick in 1..=LADDER_DISMOUNT_TICKS { let feet = hoist_path(top, landing, kind.size.height, tick as f64 / LADDER_DISMOUNT_TICKS as f64); steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Ladder, perch: None, activity: Activity::Mantle, facing, hold: -1 }); }
    } else {
        if landing.x < start[0] || landing.x > start[1] { return None; }
        steps = walked(stand, kind, landing.x, Activity::Walk)?;
        for tick in 1..=LADDER_DISMOUNT_TICKS { let feet = hoist_path(top, landing, kind.size.height, 1.0 - tick as f64 / LADDER_DISMOUNT_TICKS as f64); steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); }
        rungs(ladder, exit, 0.0, &mut steps);
        let off = clamp(ladder.foot.x, goal[0], goal[1]);
        if off != ladder.foot.x { for tick in 1..=LADDER_MOUNT_TICKS { steps.push(Foothold { x: if tick == LADDER_MOUNT_TICKS { off } else { ladder.foot.x + (off - ladder.foot.x) * smoothstep(tick as f64 / LADDER_MOUNT_TICKS as f64) }, y: ladder.foot.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); } }
    }
    Some(Outing { trip: Trip { owner: draft.stage.actors[index].species.clone(), from, steps, pitches: Vec::new(), ladder: Some(owner.to_owned()), ending: Arrival::Perch, landing: to.surface.clone(), grip: stand.grip }, raise: raise.then(|| ladder.clone()), stands, rope: None })
}

/// 🛤️ Reach a ladder from a wall, descend it and step onto its ground.
fn ladder_down(draft: &Draft<'_>, index: usize, chain: &[&Pitch], start: usize, x: f64, y: f64, grip: f64, ladder: &Ladder, from: Ticks) -> Option<Outing> {
    let size = draft.kinds[index].size;
    let stand = LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top };
    let at = chain.iter().position(|pitch| pitch.wall == ladder.wall && pitch.side == ladder.side && pitch.y0 <= ladder.top.y && ladder.top.y <= pitch.y1)?;
    let exit = ladder_exit(&stand, size);
    let door = ladder_at(&stand, exit);
    if !clings(chain[at], door.y, size) { return None; }
    let ground = draft.stage.perches.iter().find(|perch| perch.surface == ladder.surface && perch.y == ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1)?;
    let below = room_on(ground, size.width / 2.0)?;
    let along = scaling(draft, index, &[], chain, start, Entry::Cling, x, y, at, door.y, false, None, grip, from)?;
    let facing = wallward(ladder.side);
    let mut steps = along.trip.steps;
    let off = cling_of(chain[at], size);
    for tick in 1..=WALL_GRAB_TICKS { steps.push(Foothold { x: if tick == WALL_GRAB_TICKS { door.x } else { off + (door.x - off) * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64) }, y: door.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); }
    rungs(&stand, exit, 0.0, &mut steps);
    let foot = clamp(ladder.foot.x, below[0], below[1]);
    if foot != ladder.foot.x { for tick in 1..=LADDER_MOUNT_TICKS { steps.push(Foothold { x: if tick == LADDER_MOUNT_TICKS { foot } else { ladder.foot.x + (foot - ladder.foot.x) * smoothstep(tick as f64 / LADDER_MOUNT_TICKS as f64) }, y: ladder.foot.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); } }
    Some(Outing { trip: Trip { owner: along.trip.owner, from, steps, pitches: chain.iter().map(|pitch| (*pitch).clone()).collect(), ladder: Some(ladder.owner.clone()), ending: Arrival::Perch, landing: ground.surface.clone(), grip: along.trip.grip }, raise: None, stands: 0, rope: None })
}

/// 🛗️ Find exits from a wall line, cheapest first with stable ties.
fn ways_off(draft: &Draft<'_>, index: usize, chain: &[&Pitch], start: usize, x: f64, y: f64, grip: f64, now: Ticks) -> Vec<Outing> {
    let kind = draft.kinds[index];
    let mut ways = Vec::new();
    for at in start..chain.len() {
        for perch in &draft.stage.perches {
            let Some(hold) = grip_for(perch, chain[at], kind.size) else { continue; };
            if hold.over { continue; }
            if let Some(way) = scaling(draft, index, &[], chain, start, Entry::Cling, x, y, at, perch.y, false, Some(perch), grip, now + 1) { ways.push(way); }
        }
    }
    for at in (0..=start).rev() {
        if rim_for(chain[at], &draft.stage.perches, kind.size).is_none() { continue; }
        if let Some(way) = scaling(draft, index, &[], chain, start, Entry::Cling, x, y, at, rim_of(chain[at], kind.size), true, None, grip, now + 1) { ways.push(way); }
    }
    for ladder in &draft.stage.ladders {
        if ladder.since > now || ladder.until <= now { continue; }
        if let Some(way) = ladder_down(draft, index, chain, start, x, y, grip, ladder, now + 1) { ways.push(way); }
    }
    ways.sort_by(|one, other| other.trip.grip.total_cmp(&one.trip.grip));
    ways
}

/// 🪫️ Reserve the grip needed for the cheapest available exit from a wall.
fn reserve_of(draft: &Draft<'_>, index: usize, chain: &[&Pitch], start: usize, x: f64, y: f64, now: Ticks) -> f64 {
    ways_off(draft, index, chain, start, x, y, GRIP_BUDGET, now).first().map_or(0.0, |way| GRIP_BUDGET - way.trip.grip)
}

/// 🦺️ The complete body at a gear foothold includes its leaning wall posture.
fn body_at(kind: &Species, trip: &Trip, step: &Foothold) -> semio_framework_pets_contract::schema::Extent {
    extent_at(&trip.owner, kind, Point { x: step.x, y: step.y }, wall_lean_of(kind, step.footing, (step.hold >= 0).then(|| &trip.pitches[step.hold as usize]), step.x), None)
}

/// ⚖️ A wall destination must leave enough grip for a rest and its exit.
fn lasting(draft: &Draft<'_>, index: usize, outing: &Outing, now: Ticks) -> bool {
    let trip = &outing.trip;
    let last = trip.steps.last().unwrap();
    let pitch = &trip.pitches[last.hold as usize];
    let chain = chain_of(pitch, &draft.stage.pitches, draft.kinds[index].size);
    let start = chain.iter().position(|candidate| *candidate == pitch).unwrap();
    trip.grip >= reserve_of(draft, index, &chain, start, last.x, last.y, now) + GRIP_HANG * 128.0
}

/// 🛏️ Wall destinations preserve the comfortable spacing used on perches.
fn spaced(draft: &Draft<'_>, index: usize, outing: &Outing) -> bool {
    let room = grown(body_at(draft.kinds[index], &outing.trip, outing.trip.steps.last().unwrap()), COMFORT_GAP);
    bodies_of(&draft.stage.actors, &draft.kinds).iter().all(|other| other.owner == outing.trip.owner || !meets(room, other.extent))
}

/// 🏕️ Find wall rests from direct holds and ladder exits, halving unreachable rests.
fn rest_spots(draft: &Draft<'_>, index: usize, stand: &Stand, pitch: &Pitch, y: Option<f64>, place: f64, height: f64, from: Ticks, now: Ticks) -> Vec<Outing> {
    let kind = draft.kinds[index];
    let chain = chain_of(pitch, &draft.stage.pitches, kind.size);
    let leans = kind.gear.contains(&Gear::Ladder) && kind.gear.contains(&Gear::Climb);
    let mut outings = Vec::new();
    for start in 0..chain.len() {
        let mut holds = Vec::new();
        if let Some(hold) = grip_for(&stand.perch, chain[start], kind.size) { holds.push((hold, None)); }
        if let Some(lean) = leans.then(|| ladder_to(&stand.perch, chain[start], &draft.stage.keepouts, kind.size)).flatten() {
            holds.push((WallHold { x: cling_of(chain[start], kind.size), y: ladder_at(&lean, ladder_exit(&lean, kind.size)).y, over: false }, Some(lean)));
        }
        for (hold, lean) in holds {
            let over = hold.over;
            let entry = if over { rim_of(chain[start], kind.size) } else { hold.y };
            let mut spots = Vec::new();
            if let Some(y) = y { spots.push((chain.iter().position(|candidate| *candidate == pitch).unwrap(), y)); }
            else {
                let ahead = if over { chain.len() - start } else { start + 1 };
                let offset = std::cmp::min((place * ahead as f64).floor() as usize, ahead - 1);
                let at = if over { start + offset } else { start - offset };
                let near = if at == start { entry } else if over { rim_of(chain[at], kind.size) } else { foot_of(chain[at], kind.size) };
                let far = if over { foot_of(chain[at], kind.size) } else { rim_of(chain[at], kind.size) };
                spots.push((at, near + (far - near) * height));
                let end = if over { foot_of(chain[start], kind.size) } else { rim_of(chain[start], kind.size) };
                let mut share = 1.0;
                for _ in 0..3 { share /= 2.0; spots.push((start, entry + (end - entry) * share)); }
            }
            for (at, spot) in spots {
                let outing = if let Some(lean) = &lean { lean_outing(draft, index, stand, &chain, start, lean, at, spot, false, None, from) } else { from_perch(draft, index, stand, &chain, start, hold, at, spot, false, None, from) };
                if let Some(outing) = outing.filter(|outing| lasting(draft, index, outing, now) && spaced(draft, index, outing)) { outings.push(outing); break; }
            }
        }
    }
    outings
}

/// 🪝️ Plan a grappling shot, miss and return or a complete haul and mantle.
fn rope_outing(draft: &Draft<'_>, index: usize, stand: &Stand, shot: &Shot, to: &Perch, at: f64, miss: bool, from: Ticks) -> Option<Outing> {
    let kind = draft.kinds[index];
    let goal = room_on(to, kind.size.width / 2.0)?;
    let mut steps = walked(stand, kind, at, Activity::Walk)?;
    let perch = stand.perch.surface.clone();
    let facing = shot.facing;
    for _ in 0..ROPE_AIM_TICKS { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Aim, facing, hold: -1 }); }
    let fire = from + steps.len() as Ticks;
    let point = miss_of(shot, to);
    let under = Point { x: shot.hook.x, y: to.y };
    let missing = miss && point.x >= 0.0 && point.x <= draft.stage.width && sighted(shot.muzzle, point, &draft.stage.keepouts, ROPE_MARGIN, under, under);
    let target = if missing { point } else { shot.hook };
    let out = hook_ticks(shot.muzzle, target, HOOK_SPEED);
    for _ in 0..out { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Aim, facing, hold: -1 }); }
    let (dx, dy) = (target.x - shot.muzzle.x, target.y - shot.muzzle.y);
    let rope = Rope { shot: if missing { Shot { hook: target, ..shot.clone() } } else { shot.clone() }, since: fire, length: (dx * dx + dy * dy).sqrt(), hand: shot.muzzle, before: shot.muzzle, caught: !missing };
    if missing {
        for _ in 0..hook_ticks(target, shot.muzzle, HOOK_RETURN) { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Aim, facing, hold: -1 }); }
        for _ in 0..ROPE_SHRUG_TICKS { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Shrug, facing, hold: -1 }); }
        return Some(Outing { trip: Trip { owner: draft.stage.actors[index].species.clone(), from, steps, pitches: Vec::new(), ladder: None, ending: Arrival::Perch, landing: perch, grip: stand.grip }, raise: None, stands: 0, rope: Some(rope) });
    }
    for _ in 0..ROPE_TUG_TICKS { steps.push(Foothold { x: at, y: stand.perch.y, footing: Footing::Perch, perch: Some(perch.clone()), activity: Activity::Aim, facing, hold: -1 }); }
    let mut haul = haul_of(shot, shot.length, kind.size);
    for tick in 0..4096 {
        if haul.rope <= REEL_LEAST * kind.size.height { break; }
        haul = haul_step(shot, haul, tick, kind.size);
        steps.push(Foothold { x: haul.x, y: haul.y, footing: Footing::Rope, perch: None, activity: Activity::Reel, facing, hold: -1 });
    }
    let landed = landing_for(shot, to, kind.size);
    let spot = Point { x: clamp(landed.x, goal[0], goal[1]), y: landed.y };
    for tick in 1..=ROPE_HOIST_TICKS { let feet = hoist_path(Point { x: haul.x, y: haul.y }, spot, kind.size.height, tick as f64 / ROPE_HOIST_TICKS as f64); steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Rope, perch: None, activity: Activity::Mantle, facing, hold: -1 }); }
    Some(Outing { trip: Trip { owner: draft.stage.actors[index].species.clone(), from, steps, pitches: Vec::new(), ladder: None, ending: Arrival::Perch, landing: to.surface.clone(), grip: stand.grip }, raise: None, stands: 0, rope: Some(rope) })
}

/// 📶️ Rank outings by duration while preserving equal-duration order.
fn ranked(outings: &mut Vec<Outing>, outing: Outing) {
    let mut at = outings.len();
    while at > 0 && outings[at - 1].trip.steps.len() > outing.trip.steps.len() { at -= 1; }
    outings.insert(at, outing);
}

/// 🔗️ Join two complete legs when they share at most one ladder and one rope.
fn joined(first: &Outing, second: Outing) -> Option<Outing> {
    if (first.trip.ladder.is_some() && second.trip.ladder.is_some()) || (first.rope.is_some() && second.rope.is_some()) { return None; }
    let offset = first.trip.pitches.len() as i64;
    let mut steps = first.trip.steps.clone();
    steps.extend(second.trip.steps.into_iter().map(|mut step| { if step.hold >= 0 { step.hold += offset; } step }));
    let mut pitches = first.trip.pitches.clone();
    pitches.extend(second.trip.pitches);
    Some(Outing { trip: Trip { owner: first.trip.owner.clone(), from: first.trip.from, steps, pitches, ladder: first.trip.ladder.clone().or(second.trip.ladder), ending: second.trip.ending, landing: second.trip.landing, grip: second.trip.grip }, raise: first.raise.clone().or(second.raise), stands: if first.raise.is_some() { first.stands } else { second.stands }, rope: first.rope.clone().or(second.rope) })
}

/// 🧷️ Plan all direct legs to a perch or wall rest in stable quickest-first order.
fn legs_to(draft: &Draft<'_>, index: usize, stand: &Stand, venture: &Venture, units: &[f64], miss: bool, from: Ticks, now: Ticks) -> Vec<Outing> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let to = match venture { Venture::Wall { pitch, y } => return rest_spots(draft, index, stand, pitch, *y, units[0], units[1], from, now), Venture::Perch(perch) => perch };
    let standing: Vec<_> = draft.stage.ladders.iter().filter(|ladder| ladder.rider.is_none() && ladder.since <= now && ladder.until > now).collect();
    let ladders: Vec<_> = standing.iter().map(|ladder| LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top }).collect();
    let legs = route_of(stand.x, &stand.perch, to, &kind.gear, kind.size, stand.grip, &draft.stage.pitches, &ladders, &draft.stage.keepouts).unwrap_or_default();
    let mut outings = Vec::new();
    for leg in legs {
        let outing = match leg {
            Leg::Ladder { ladder, up, .. } => ladders.iter().position(|candidate| std::ptr::eq(candidate, ladder)).and_then(|at| ladder_outing(draft, index, stand, ladder, &standing[at].owner, up, to, false, from)),
            Leg::Wall { pitch, hold, exit, goal, .. } => { let chain = chain_of(pitch, &draft.stage.pitches, kind.size); from_perch(draft, index, stand, &chain, chain.iter().position(|candidate| *candidate == pitch).unwrap(), hold, chain.iter().position(|candidate| *candidate == exit).unwrap(), goal, !hold.over, hold.over.then_some(to), from) },
            Leg::Raise { ladder, .. } => if draft.stage.ladders.len() < 2 && !draft.stage.ladders.iter().any(|ladder| ladder.owner == body.species) { ladder_outing(draft, index, stand, &ladder, &body.species, true, to, true, from) } else { None },
            Leg::Grapple { at, shot } => {
                room_on(&stand.perch, kind.size.width / 2.0).and_then(|room| {
                    let place = clamp(at, room[0], room[1]);
                    let shot = if place == at { Some(shot) } else { shot_for(Point { x: place, y: stand.perch.y }, std::slice::from_ref(to), &draft.stage.keepouts, kind.size) }?;
                    rope_outing(draft, index, stand, &shot, to, place, miss, from)
                })
            }
        };
        if let Some(outing) = outing { ranked(&mut outings, outing); }
    }
    outings
}

/// 🗺️ Plan direct outings, then two-leg outings only when no direct way exists.
fn outings_to(draft: &Draft<'_>, index: usize, venture: &Venture, units: &[f64], now: Ticks) -> Vec<Outing> {
    let body = &draft.stage.actors[index];
    if !geared(draft.kinds[index]) || body.footing != Footing::Perch { return Vec::new(); }
    let Some(from) = body.perch.as_deref().and_then(|surface| perch_at(&draft.stage.perches, surface, body.x)) else { return Vec::new(); };
    let stand = Stand { perch: from.clone(), x: body.x, y: body.y, facing: body.facing, grip: body.grip };
    let direct = legs_to(draft, index, &stand, venture, units, units[2] < ROPE_MISS_CHANCE, now + 1, now);
    if !direct.is_empty() { return direct; }
    let mut outings = Vec::new();
    for between in &draft.stage.perches {
        if between == from || matches!(venture, Venture::Perch(perch) if perch == between) { continue; }
        for first in legs_to(draft, index, &stand, &Venture::Perch(between.clone()), units, false, now + 1, now) {
            let last = first.trip.steps.last().unwrap();
            let arrival = Stand { perch: between.clone(), x: last.x, y: last.y, facing: last.facing, grip: first.trip.grip };
            for second in legs_to(draft, index, &arrival, venture, units, false, first.trip.from + first.trip.steps.len() as Ticks, now) { if let Some(both) = joined(&first, second) { ranked(&mut outings, both); } }
        }
    }
    outings
}

/// 🌉️ Gear reaches a destination directly or through one intermediate perch.
pub(crate) fn reaches(draft: &Draft<'_>, kind: &Species, x: f64, from: &Perch, to: &Perch, now: Ticks) -> bool {
    let ladders: Vec<_> = draft.stage.ladders.iter().filter(|ladder| ladder.rider.is_none() && ladder.since <= now && ladder.until > now).map(|ladder| LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top }).collect();
    let route = |x, from, to| route_of(x, from, to, &kind.gear, kind.size, GRIP_BUDGET, &draft.stage.pitches, &ladders, &draft.stage.keepouts).is_some();
    if route(x, from, to) { return true; }
    draft.stage.perches.iter().any(|between| between != from && between != to && route(x, from, between) && route((between.x0 + between.x1) / 2.0, between, to))
}

/// 🎋️ Plan both exits from a ladder, trying the nearer one first.
fn ladder_ways(draft: &Draft<'_>, index: usize, ladder: &Ladder, now: Ticks) -> Vec<Outing> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let stand = LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top };
    let travel = clamp((body.y - ladder.foot.y) / (ladder.top.y - ladder.foot.y), 0.0, 1.0) * ladder_length(&stand);
    let exit = ladder_exit(&stand, kind.size);
    let landing = ladder_landing(&stand, kind.size);
    let facing = wallward(ladder.side);
    let mut ways = Vec::new();
    let crown = draft.stage.pitches.iter().filter(|pitch| pitch.wall == ladder.wall && pitch.side == ladder.side).find_map(|pitch| rim_for(pitch, &draft.stage.perches, kind.size));
    if let Some(crown) = crown {
        if room_on(crown, kind.size.width / 2.0).is_some_and(|above| landing.x >= above[0] && landing.x <= above[1]) {
            let mut steps = Vec::new();
            rungs(&stand, travel, exit, &mut steps);
            let top = ladder_at(&stand, exit);
            for tick in 1..=LADDER_DISMOUNT_TICKS { let feet = hoist_path(top, landing, kind.size.height, tick as f64 / LADDER_DISMOUNT_TICKS as f64); steps.push(Foothold { x: feet.x, y: feet.y, footing: Footing::Ladder, perch: None, activity: Activity::Mantle, facing, hold: -1 }); }
            ways.push(Outing { trip: Trip { owner: body.species.clone(), from: now + 1, steps, pitches: Vec::new(), ladder: Some(ladder.owner.clone()), ending: Arrival::Perch, landing: crown.surface.clone(), grip: body.grip }, raise: None, stands: 0, rope: None });
        }
    }
    if let Some(ground) = draft.stage.perches.iter().find(|perch| perch.surface == ladder.surface && perch.y == ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1) {
        if let Some(below) = room_on(ground, kind.size.width / 2.0) {
            let mut steps = Vec::new();
            rungs(&stand, travel, 0.0, &mut steps);
            let off = clamp(ladder.foot.x, below[0], below[1]);
            if off != ladder.foot.x { for tick in 1..=LADDER_MOUNT_TICKS { steps.push(Foothold { x: if tick == LADDER_MOUNT_TICKS { off } else { ladder.foot.x + (off - ladder.foot.x) * smoothstep(tick as f64 / LADDER_MOUNT_TICKS as f64) }, y: ladder.foot.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); } }
            if steps.is_empty() { steps.push(Foothold { x: body.x, y: body.y, footing: Footing::Ladder, perch: None, activity: Activity::Climb, facing, hold: -1 }); }
            let way = Outing { trip: Trip { owner: body.species.clone(), from: now + 1, steps, pitches: Vec::new(), ladder: Some(ladder.owner.clone()), ending: Arrival::Perch, landing: ground.surface.clone(), grip: body.grip }, raise: None, stands: 0, rope: None };
            if !ways.is_empty() && travel < exit - travel { ways.insert(0, way); } else { ways.push(way); }
        }
    }
    ways
}

/// 🛝️ Plan a wall slide to the first lower perch or the end of the wall.
fn slide_outing(draft: &Draft<'_>, index: usize, grip: f64, now: Ticks) -> Option<Outing> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let pitch = body.pitch.as_ref()?;
    let bottom = foot_of(pitch, kind.size);
    let below = landing_of(&draft.stage.perches, body.x, body.y, bottom);
    let floor = below.map_or(bottom, |below| below.y);
    let mut steps = Vec::new();
    let (mut y, mut vy) = (body.y, 0.0);
    let facing = wallward(pitch.side);
    for _ in 0..4096 {
        if y == floor { break; }
        let slide = slide_step(y, vy, floor);
        (y, vy) = (slide.y, slide.vy);
        steps.push(Foothold { x: body.x, y, footing: Footing::Wall, perch: None, activity: Activity::Slide, facing, hold: 0 });
    }
    let (mut ending, mut landing) = (Arrival::Air, String::new());
    if let Some((below, room)) = below.and_then(|below| room_on(below, kind.size.width / 2.0).map(|room| (below, room))) {
        let to = clamp(body.x, room[0], room[1]);
        if to != body.x { for tick in 1..=WALL_GRAB_TICKS { steps.push(Foothold { x: if tick == WALL_GRAB_TICKS { to } else { body.x + (to - body.x) * smoothstep(tick as f64 / WALL_GRAB_TICKS as f64) }, y: below.y, footing: Footing::Wall, perch: None, activity: Activity::Slide, facing, hold: 0 }); } }
        ending = Arrival::Perch;
        landing = below.surface.clone();
    }
    if steps.is_empty() { return None; }
    let cost: f64 = steps.iter().map(effort_of).sum();
    Some(Outing { trip: Trip { owner: body.species.clone(), from: now + 1, steps, pitches: vec![pitch.clone()], ladder: None, ending, landing, grip: larger(grip - cost, 0.0) }, raise: None, stands: 0, rope: None })
}

/// 👟️ A trip's walk or carry aims at the final consecutive ground foothold.
fn walk_end(trip: &Trip, at: usize) -> f64 {
    let mut goal = trip.steps[at].x;
    for step in &trip.steps[at + 1..] { if step.footing != Footing::Perch || !matches!(step.activity, Activity::Walk | Activity::Carry) { break; } goal = step.x; }
    goal
}

/// ⛵️ Commit a gear outing only after its entire claimed corridor is clear.
fn embark(draft: &mut Draft<'_>, index: usize, outing: Outing, now: Ticks) -> bool {
    let kind = draft.kinds[index];
    let trip = &outing.trip;
    let owner = draft.stage.actors[index].species.clone();
    let half = kind.size.width / 2.0;
    if trip.steps.iter().any(|step| step.x - half < 0.0 || step.x + half > draft.stage.width || step.y - kind.size.height < 0.0 || step.y > draft.stage.height) { return false; }
    let end = trip.from + trip.steps.len() as Ticks - 1;
    if outing.raise.is_some() {
        if draft.stage.ladders.len() >= 2 || draft.stage.ladders.iter().any(|ladder| ladder.owner == owner) { return false; }
    } else if let Some(ladder) = &trip.ladder {
        if !draft.stage.ladders.iter().any(|entry| &entry.owner == ladder && (entry.rider.is_none() || entry.rider.as_deref() == Some(&owner)) && entry.since <= now && entry.since + LADDER_LIFE >= end + LADDER_IDLE) { return false; }
    }
    let extents: Vec<_> = trip.steps.iter().map(|step| body_at(kind, trip, step)).collect();
    let claim = claim_of(&owner, trip.from, &extents, CLAIM_SPAN, if trip.ending == Arrival::Air { None } else { extents.last().copied() });
    if !claim_clear(&claim, &bodies_of(&draft.stage.actors, &draft.kinds), &draft.stage.claims) { return false; }
    release(draft, index, now);
    if draft.stage.actors[index].footing == Footing::Wall { draft.stage.actors[index].grip = grip_now(&draft.stage.actors[index], now); }
    if let Some(raise) = outing.raise { draft.stage.ladders.push(Ladder { owner: owner.clone(), wall: raise.wall, surface: raise.surface, side: raise.side, foot: raise.foot, top: raise.top, since: outing.stands, until: std::cmp::min(outing.stands + LADDER_LIFE, end + LADDER_IDLE), rider: Some(owner.clone()) }); }
    else if let Some(ladder) = &trip.ladder { for entry in &mut draft.stage.ladders { if &entry.owner == ladder { entry.until = std::cmp::min(entry.since + LADDER_LIFE, std::cmp::max(entry.until, end + LADDER_IDLE)); entry.rider = Some(owner.clone()); } } }
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.claims.push(claim);
    draft.stage.courses.retain(|course| course.owner != owner);
    draft.stage.trips.retain(|trip| trip.owner != owner);
    let first = &trip.steps[0];
    let goal = if first.footing == Footing::Perch && matches!(first.activity, Activity::Walk | Activity::Carry) { walk_end(trip, 0) } else { draft.stage.actors[index].x };
    let body = &mut draft.stage.actors[index];
    body.rope = outing.rope;
    body.until = end;
    body.goal = goal;
    turn(body, first.facing, now);
    draft.stage.trips.push(outing.trip);
    true
}

/// 🚩️ Try every venture from a drawn first choice, taking its first legal outing.
pub(crate) fn set_out(draft: &mut Draft<'_>, index: usize, ventures: &[Venture], pick: f64, rest: &[f64], now: Ticks) -> bool {
    if ventures.is_empty() { return false; }
    let first = std::cmp::min((pick * ventures.len() as f64).floor() as usize, ventures.len() - 1);
    for turn in 0..ventures.len() { for outing in outings_to(draft, index, &ventures[(first + turn) % ventures.len()], rest, now) { if embark(draft, index, outing, now) { return true; } } }
    false
}

/// 🦥️ Rest on a wall or ladder while reserving enough grip to leave the wall.
pub(crate) fn rest(draft: &mut Draft<'_>, index: usize, span: Ticks, now: Ticks) {
    shift(draft, index, Activity::Climb, now);
    let kind = draft.kinds[index];
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(kind, Activity::Climb, 0.0);
    body.goal = body.x;
    body.vx = 0.0;
    body.vy = 0.0;
    let mut until = now + span;
    let body = &draft.stage.actors[index];
    if let Some(pitch) = body.pitch.as_ref().filter(|_| body.footing == Footing::Wall) {
        let chain = chain_of(pitch, &draft.stage.pitches, kind.size);
        let spare = body.grip - reserve_of(draft, index, &chain, chain.iter().position(|candidate| *candidate == pitch).unwrap(), body.x, body.y, now);
        until = std::cmp::min(until, now + (spare / GRIP_HANG).floor() as Ticks);
    }
    draft.stage.actors[index].until = std::cmp::max(until, now + 1);
}

/// 🛑️ Release an interrupted trip and rest wherever its current footing permits.
pub(crate) fn halt(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let owner = body.species.clone();
    if trip_of(&draft.stage.trips, &owner).is_none() { return; }
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.trips.retain(|trip| trip.owner != owner);
    let body = &draft.stage.actors[index];
    if body.footing == Footing::Rope { let (vx, vy) = (body.vx, body.vy); unfoot(draft, index, vx, vy, Activity::Fall, now); return; }
    draft.stage.actors[index].rope = None;
    if draft.stage.actors[index].footing == Footing::Ladder { rest(draft, index, 1, now); return; }
    freed(draft, &owner, now);
    if draft.stage.actors[index].footing == Footing::Wall { rest(draft, index, 1, now); }
}

/// 🚂️ Step an installed gear trip along its exact footholds and rope geometry.
pub(crate) fn travel(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let Some(trip) = trip_of(&draft.stage.trips, &draft.stage.actors[index].species) else { return true; };
    let at = now - trip.from;
    if at < 0 { return true; }
    let last = trip.steps.len() - 1;
    let at = std::cmp::min(at as usize, last);
    if now > trip.from && draft.stage.actors[index].activity != trip.steps[at.saturating_sub(1)].activity { halt(draft, index, now); return true; }
    let step = trip.steps[at].clone();
    let goal = walk_end(trip, at);
    let pitch = (step.hold >= 0).then(|| trip.pitches[step.hold as usize].clone());
    let kind = draft.kinds[index];
    let body = &mut draft.stage.actors[index];
    body.vx = (step.x - body.x) * RATE;
    body.vy = (step.y - body.y) * RATE;
    body.x = step.x;
    body.y = step.y;
    body.footing = step.footing;
    body.perch = step.perch.clone();
    body.pitch = pitch;
    body.grip = larger(body.grip - effort_of(&step), 0.0);
    if step.activity != body.activity {
        shift(draft, index, step.activity, now);
        draft.stage.actors[index].clip = clip_at(kind, step.activity, 0.0);
        if matches!(step.activity, Activity::Walk | Activity::Carry) { draft.stage.actors[index].goal = goal; }
        if step.activity == Activity::Shrug { feel(draft, index, Occasion::Missed, now); }
    }
    let body = &mut draft.stage.actors[index];
    if step.facing != body.facing { turn(body, step.facing, now); }
    if let Some(rope) = body.rope.as_ref() {
        if (!rope.caught && now >= rope.since + hook_ticks(rope.shot.muzzle, rope.shot.hook, HOOK_SPEED) + hook_ticks(rope.shot.hook, rope.shot.muzzle, HOOK_RETURN)) || (rope.caught && now > rope.since && step.footing == Footing::Perch && step.activity != Activity::Aim) { body.rope = None; }
        else if step.footing == Footing::Rope { let hand = Point { x: step.x + rope.shot.facing.sign() * MUZZLE_FORWARD * kind.size.width, y: step.y - MUZZLE_HEIGHT * kind.size.height }; let (dx, dy) = (rope.shot.hook.x - hand.x, rope.shot.hook.y - hand.y); body.rope = Some(Rope { hand, before: rope.hand, length: (dx * dx + dy * dy).sqrt(), ..rope.clone() }); }
    }
    if at < last { return true; }
    let trip = trip_of(&draft.stage.trips, &draft.stage.actors[index].species).unwrap().clone();
    alight(draft, index, &trip, now)
}

/// 🏁️ Release the completed trip and restore its perch, wall rest or free fall.
fn alight(draft: &mut Draft<'_>, index: usize, trip: &Trip, now: Ticks) -> bool {
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.trips.retain(|trip| trip.owner != owner);
    freed(draft, &owner, now);
    draft.stage.actors[index].rope = None;
    if trip.ending == Arrival::Air { let body = &draft.stage.actors[index]; let (vx, vy) = (body.vx, body.vy); unfoot(draft, index, vx, vy, Activity::Fall, now); return plunge(draft, index, now); }
    if trip.ending == Arrival::Wall {
        let body = &mut draft.stage.actors[index];
        body.footing = Footing::Wall;
        body.perch = None;
        body.grip = trip.grip;
        let words = random_words(&actor_key(draft, index), 1);
        rest(draft, index, dwell_of(Activity::Idle, draft.stage.mode, unit_of(words[0])), now);
        return true;
    }
    let body = &mut draft.stage.actors[index];
    body.footing = Footing::Perch;
    body.perch = Some(trip.landing.clone());
    body.pitch = None;
    body.grip = GRIP_BUDGET;
    settle(draft, index, now);
    if trip.steps.last().unwrap().activity == Activity::Shrug { draft.stage.actors[index].until = std::cmp::max(draft.stage.actors[index].until, now + ROPE_REST); }
    else if trip.steps.iter().any(|step| step.footing != Footing::Perch) { feel(draft, index, Occasion::Climbed, now); }
    true
}

/// 🏞️ Try an authored random rest farther along the currently held wall line.
fn explored(draft: &Draft<'_>, index: usize, chain: &[&Pitch], start: usize, grip: f64, place: f64, height: f64, now: Ticks) -> Option<Outing> {
    let body = &draft.stage.actors[index];
    let size = draft.kinds[index].size;
    let at = std::cmp::min((place * chain.len() as f64).floor() as usize, chain.len() - 1);
    let y = rim_of(chain[at], size) + (foot_of(chain[at], size) - rim_of(chain[at], size)) * height;
    scaling(draft, index, &[], chain, start, Entry::Cling, body.x, body.y, at, y, false, None, grip, now).filter(|outing| lasting(draft, index, outing, now) && spaced(draft, index, outing))
}

/// ➰️ Leave an expired rest through the first clear climb, ladder exit or wall slide.
fn onward(draft: &mut Draft<'_>, index: usize, ladder: Option<&Ladder>, now: Ticks) -> bool {
    if let Some(ladder) = ladder {
        for way in ladder_ways(draft, index, ladder, now) { if embark(draft, index, way, now) { return true; } }
        if ladder.since + LADDER_LIFE <= now + 32 { feel(draft, index, Occasion::Startled, now); unfoot(draft, index, 0.0, 0.0, Activity::Fall, now); return plunge(draft, index, now); }
        for entry in &mut draft.stage.ladders { if entry.owner == ladder.owner { entry.until = std::cmp::min(entry.since + LADDER_LIFE, std::cmp::max(entry.until, now + 32 + LADDER_IDLE)); } }
        draft.stage.actors[index].until = now + 32;
        return true;
    }
    let body = &draft.stage.actors[index];
    let pitch = body.pitch.clone().unwrap();
    let grip = grip_now(body, now);
    let chain = chain_of(&pitch, &draft.stage.pitches, draft.kinds[index].size);
    let start = chain.iter().position(|candidate| *candidate == &pitch).unwrap();
    let ways = ways_off(draft, index, &chain, start, body.x, body.y, grip, now);
    let reserve = ways.first().map_or(0.0, |way| grip - way.trip.grip);
    let has_way = !ways.is_empty();
    let mut tries = Vec::new();
    if draft.stage.mode == semio_framework_pets_contract::schema::PetMode::Lively && !draft.stage.quiet {
        let units: Vec<_> = random_words(&actor_key(draft, index), 3).into_iter().map(unit_of).collect();
        let chain = chain_of(&pitch, &draft.stage.pitches, draft.kinds[index].size);
        if units[0] < 0.5 { if let Some(spot) = explored(draft, index, &chain, start, grip, units[1], units[2], now) { tries.push(spot); } }
    }
    tries.extend(ways);
    for outing in tries { if embark(draft, index, outing, now) { return true; } }
    if has_way && grip - GRIP_HANG * 32.0 >= reserve { draft.stage.actors[index].until = now + 32; return true; }
    if let Some(slide) = slide_outing(draft, index, grip, now) { if embark(draft, index, slide, now) { return true; } }
    if grip - GRIP_HANG * 32.0 > 0.0 { draft.stage.actors[index].until = now + 32; return true; }
    feel(draft, index, Occasion::Startled, now);
    unfoot(draft, index, 0.0, 0.0, Activity::Fall, now);
    plunge(draft, index, now)
}

/// 🐨️ Rest on a held wall or ladder until it is time to move on.
pub(crate) fn cling(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let ladder = (body.footing == Footing::Ladder).then(|| draft.stage.ladders.iter().find(|entry| entry.rider.as_deref() == Some(&body.species)).cloned()).flatten();
    if (body.footing == Footing::Wall && body.pitch.is_none()) || (body.footing == Footing::Ladder && ladder.is_none()) { unfoot(draft, index, 0.0, 0.0, Activity::Fall, now); return plunge(draft, index, now); }
    if now < body.until { return true; }
    onward(draft, index, ladder.as_ref(), now)
}

/// 🔓️ Free an owner's ladder and remove a ladder not yet raised.
fn freed(draft: &mut Draft<'_>, owner: &str, now: Ticks) {
    draft.stage.ladders.retain(|ladder| ladder.owner != owner || ladder.since <= now);
    for ladder in &mut draft.stage.ladders { if ladder.rider.as_deref() == Some(owner) { ladder.rider = None; } }
}

/// 🧺️ Release trips, rope, wall grip and ladders whenever an actor loses its footing.
fn ungear(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.trips.retain(|trip| trip.owner != owner);
    let body = &mut draft.stage.actors[index];
    body.rope = None;
    body.pitch = None;
    body.grip = GRIP_BUDGET;
    freed(draft, &owner, now);
}
//#endregion 🔖️Gear

//#region 🔖️Mischief
/// 📬️ Reach a post on foot or with gear, then use a guarded poof only as the last resort.
pub(crate) fn post_to(draft: &mut Draft<'_>, index: usize, post: &Post, side: Facing, span: Ticks, slip: bool, now: Ticks) -> Option<Ticks> {
    let body = &draft.stage.actors[index];
    match post {
        Post::Wall { pitch, y } => {
            for outing in outings_to(draft, index, &Venture::Wall { pitch: pitch.clone(), y: Some(*y) }, &[0.0, 0.0, 1.0], now) {
                let arrives = outing.trip.from + outing.trip.steps.len() as Ticks - 1;
                if outing.trip.grip >= GRIP_HANG * span as f64 && embark(draft, index, outing, now) { return Some(arrives); }
            }
        }
        Post::Perch { perch, x } if body.footing == Footing::Perch && body.perch.as_deref() == Some(&perch.surface) => {
            let stand = Stand { perch: perch.clone(), x: body.x, y: body.y, facing: body.facing, grip: body.grip };
            if let Some(steps) = walked(&stand, draft.kinds[index], *x, Activity::Walk) {
                if steps.is_empty() { return Some(now); }
                let trip = Trip { owner: body.species.clone(), from: now + 1, steps, pitches: Vec::new(), ladder: None, ending: Arrival::Perch, landing: perch.surface.clone(), grip: body.grip };
                let arrives = trip.from + trip.steps.len() as Ticks - 1;
                if embark(draft, index, Outing { trip, raise: None, stands: 0, rope: None }, now) { return Some(arrives); }
            }
        }
        _ => {}
    }
    (slip && pop_to(draft, index, post, side, span, now)).then_some(now)
}

/// 💫️ Slip to a distant post only where the entire body and comfortable gap are free.
fn pop_to(draft: &mut Draft<'_>, index: usize, post: &Post, side: Facing, span: Ticks, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let (feet, pitch) = match post {
        Post::Wall { pitch, y } => (Foothold { x: cling_of(pitch, kind.size), y: *y, footing: Footing::Wall, perch: None, activity: Activity::Climb, facing: side, hold: 0 }, Some(pitch)),
        Post::Perch { perch, x } => (Foothold { x: *x, y: perch.y - hover_of(kind), footing: Footing::Perch, perch: Some(perch.surface.clone()), activity: Activity::Idle, facing: side, hold: -1 }, None),
    };
    let extent = extent_at(&body.species, kind, Point { x: feet.x, y: feet.y }, wall_lean_of(kind, feet.footing, pitch, feet.x), None);
    let room = grown(extent, COMFORT_GAP);
    let half = kind.size.width / 2.0;
    if feet.x - half < 0.0 || feet.x + half > draft.stage.width || feet.y - kind.size.height < 0.0 || feet.y > draft.stage.height || !free_among(extent, &obstacles_for(draft, index, now)) || bodies_of(&draft.stage.actors, &draft.kinds).iter().any(|other| other.owner != body.species && meets(room, other.extent)) { return false; }
    dusted(draft, index, now);
    release(draft, index, now);
    ungear(draft, index, now);
    let body = &mut draft.stage.actors[index];
    body.x = feet.x;
    body.y = feet.y;
    body.goal = feet.x;
    body.vx = 0.0;
    body.vy = 0.0;
    body.tilt = 0.0;
    body.opacity = 0.0;
    body.facing = side;
    body.faced = now;
    body.host = None;
    if let Post::Wall { pitch, .. } = post {
        body.footing = Footing::Wall;
        body.perch = None;
        body.pitch = Some(pitch.clone());
        rest(draft, index, span, now);
        draft.stage.actors[index].until = now + span;
    } else {
        body.footing = Footing::Perch;
        body.perch = feet.perch;
        settle(draft, index, now);
    }
    true
}

/// 🤜️ A ready actor pushes from its post while retaining enough grip to finish.
pub(crate) fn push_at(draft: &mut Draft<'_>, index: usize, side: Facing, until: Ticks, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    if body.leaving || body.partner.is_some() || trip_of(&draft.stage.trips, &body.species).is_some() { return false; }
    let clinging = body.footing == Footing::Wall && body.activity == Activity::Climb && body.pitch.is_some();
    if !clinging && !(body.footing == Footing::Perch && body.activity == Activity::Idle) { return false; }
    if clinging { let grip = grip_now(body, now); if grip < GRIP_HANG * (until - now) as f64 { return false; } draft.stage.actors[index].grip = grip; }
    shift(draft, index, Activity::Push, now);
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(draft.kinds[index], Activity::Push, 0.0);
    body.until = until;
    body.goal = body.x;
    body.vx = 0.0;
    body.vy = 0.0;
    turn(body, side, now);
    true
}

/// 🫷️ A stopped pusher returns to its perch rest or a short wall rest.
pub(crate) fn unpush(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    if body.activity != Activity::Push { return; }
    if body.footing == Footing::Wall && body.pitch.is_some() { rest(draft, index, 1, now); } else { settle(draft, index, now); }
}

/// 🧯️ End an unreclaimed prank quietly and start its cooldown.
pub(crate) fn unlifted(draft: &mut Draft<'_>, now: Ticks) {
    let Some(lift) = &draft.stage.lift else { return; };
    if now - lift.since > LIFT_TICKS { return; }
    let pusher = lift.pusher.clone();
    draft.stage.lift = None;
    draft.stage.rested = now;
    if let Some(index) = index_of(&draft.stage.actors, &pusher) { unpush(draft, index, now); }
}
//#endregion 🔖️Mischief

//#region 🔖️Leaving
/// 👋️ An actor starts to leave: it lets go of its partner and walks to the nearer end of its perch when that is within three of its widths and nobody stands in the way, then fades; farther away, hemmed in or in the air, it fades where it is.
pub(crate) fn leave(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    release(draft, index, now);
    draft.stage.actors[index].leaving = true;
    if draft.stage.actors[index].footing != Footing::Perch || draft.stage.actors[index].perch.is_none() { return; }
    halt(draft, index, now);
    let body = &draft.stage.actors[index];
    let Some(surface) = body.perch.as_deref() else {
        return;
    };
    let perch = perch_at(&draft.stage.perches, surface, body.x);
    let half = kind.size.width / 2.0;
    let low = perch.map_or(body.x, |perch| perch.x0 + half);
    let high = perch.map_or(body.x, |perch| perch.x1 - half);
    let end = if body.x - low <= high - body.x { low } else { high };
    let [clear_low, clear_high] = clearway(draft, index, MEET_GAP, None);
    let clip = clip_at(kind, Activity::Walk, 0.0);
    let goal = paced(kind, clip.as_deref(), body.x, end);
    if goal != body.x && end >= clear_low && end <= clear_high && (end - body.x).abs() <= LEAVE_REACH * kind.size.width {
        stroll(draft, index, goal, clip, now);
        return;
    }
    shift(draft, index, Activity::Idle, now);
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(kind, Activity::Idle, 0.0);
    body.goal = body.x;
}
//#endregion 🔖️Leaving

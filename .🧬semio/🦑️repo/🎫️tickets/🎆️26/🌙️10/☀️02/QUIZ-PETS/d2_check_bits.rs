//! 🧮️ Ticket tool of work package D2, Rust half: recomputes every case of `🗑️generated/d2/bits.json` with the Rust twins
//! of clearance and gesture and every replay of `🗑️generated/d2/corpus.json` from the committed gesture corpus, and
//! compares every result with the TypeScript twins' bit for bit (cues, signals and state digests for the corpus).
//!
//! Mounted as the integration test `d2_bits` of the scratch crate (`d2_scratch.ts prepare`). Numbers arrive and are
//! compared as sixteen hexadecimal digits of their IEEE-754 pattern (`nan` for every NaN); a case the TypeScript twin
//! throws on must panic here. The layouts of the flat arguments are the encoders of `d2_dump_bits.ts`. The counts per
//! function and every mismatch are written to `🗑️generated/d2/bits-report.txt`.
//!
//! Run from the repository root, after `bun <ticket>/d2_dump_bits.ts`:
//!   bash <ticket>/rust_scratch.sh d2 test --release --offline --test d2_bits -- --nocapture
//!
//! @see ./d2_dump_bits.ts — the TypeScript half and the layout of every case
//! @see ./📓️report2-d2.md — the recorded result

use pets::clearance::{
    body_of, canopied, claim_clear, claim_of, column_over, evicted, free_among, free_at, grown, guarded_stride, head_under, lane_lift, leaning, lift_onto, meets, must_poof, near_misses, obstacles_of, order_kept, order_of, overlaps, per_million,
    pruned, pushed_out, released, rests_on, scoot_fraction, scooted, seat_of, shifted, slice_at, slide_of, slide_side, slide_stride, slot_in, spot_on, tallied, united, vaults, Body, Pair, Posture, Tally, DELAY, FOREVER, LANE_LIFT, LEAN, MARGIN,
    PATIENCE, PUSHES, SCOOT_HASTE, SEAM, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT, SLIDE_OFF_SPEED, STEERINGS, TALLY, UPRIGHT,
};
use pets::gesture::{
    circle_step, heat_after, heat_at, hover_busy, hover_step, no_circling, no_hover, no_shaking, no_stroking, press_due, press_step, shake_step, slop_of, stroke_step, tier_of, warmth_after, Caress, Guards, PressInput, PressSignal, CIRCLE_AGAINST,
    CIRCLE_CLOSE, CIRCLE_FAST, CIRCLE_HYSTERESIS, CIRCLE_MARGIN, CIRCLE_OUT_TICKS, CIRCLE_QUARTERS, CIRCLE_REACH, CIRCLE_REST, CIRCLE_ROUND, CIRCLE_SLOW, COLD, ENOUGH_TICKS, HEAT_CLICK, HEAT_ENOUGH, HEAT_FORGIVEN, HEAT_HELLO, HEAT_HOLD, HEAT_LEAK,
    HEAT_TRICK, HOLD_TICKS, IDLE, SCROLL_TICKS, SHAKE_AMPLITUDE, SHAKE_HYSTERESIS, SHAKE_PAUSE, SHAKE_REST, SHAKE_REVERSALS, SHAKE_SPEED, SHAKE_WINDOW, SLOP_COARSE, SLOP_FINE, STROKE_FAST, STROKE_HYSTERESIS, STROKE_LENGTH, STROKE_MARGIN,
    STROKE_PAUSE, STROKE_SEGMENTS, STROKE_SLANT, STROKE_SLOW, STROKE_WINDOW, UNGUARDED,
};
use pets::schema::{Circling, Claim, Cue, Dragged, Extent, Facing, Hover, Perch, Point, Pointer, Press, Pressed, Rect, Released, Shaking, Size, Slice, Slug, Stroking, Ticks, Tier, Warmth, CUES, POINTERS, PRESS_PHASES, TIERS};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

/// 🔢️ The number a bit pattern stands for.
fn decode(text: &str) -> f64 {
    if text == "nan" {
        f64::NAN
    } else {
        f64::from_bits(u64::from_str_radix(text, 16).unwrap_or_else(|error| panic!("{text}: {error}")))
    }
}

/// 🔡️ The bit pattern of a number, `nan` for every NaN.
fn encode(value: f64) -> String {
    if value.is_nan() {
        "nan".to_string()
    } else {
        format!("{:016x}", value.to_bits())
    }
}

/// 🔘️ A truth as the number the TypeScript half records.
fn truth(value: bool) -> f64 {
    if value {
        1.0
    } else {
        0.0
    }
}

/// 📖️ The flat arguments of a case, read front to back in the layouts of `d2_dump_bits.ts`.
struct Arguments {
    values: Vec<f64>,
    at: usize,
}

impl Arguments {
    /// ➡️ The next number.
    fn number(&mut self) -> f64 {
        let value = self.values[self.at];
        self.at += 1;
        value
    }

    /// 🔟️ The next number as a count or an index.
    fn count(&mut self) -> usize {
        self.number() as usize
    }

    /// ⏱️ The next number as whole ticks or a whole counter.
    fn ticks(&mut self) -> Ticks {
        self.number() as Ticks
    }

    /// ✔️ The next number as a flag.
    fn flag(&mut self) -> bool {
        self.number() == 1.0
    }

    /// 🏷️ The next number as the owner `o<k>`.
    fn owner(&mut self) -> Slug {
        format!("o{}", self.count())
    }

    /// 📦️ The next four numbers as a box.
    fn extent(&mut self) -> Extent {
        Extent { x0: self.number(), y0: self.number(), x1: self.number(), y1: self.number() }
    }

    /// 🔲️ A counted list of boxes.
    fn extents(&mut self) -> Vec<Extent> {
        (0..self.count()).map(|_| self.extent()).collect()
    }

    /// 🐾️ A counted list of bodies.
    fn bodies(&mut self) -> Vec<Body> {
        (0..self.count()).map(|_| Body { owner: self.owner(), extent: self.extent() }).collect()
    }

    /// ❓️ A flag and a box, the box when the flag is set.
    fn maybe_extent(&mut self) -> Option<Extent> {
        let present = self.flag();
        let extent = self.extent();
        present.then_some(extent)
    }

    /// 🎫️ A claim: owner, counted slices, a rest flag and the rest.
    fn claim(&mut self) -> Claim {
        let owner = self.owner();
        let slices = (0..self.count()).map(|_| Slice { from: self.ticks(), until: self.ticks(), extent: self.extent() }).collect();
        Claim { owner, slices, rest: self.maybe_extent() }
    }

    /// 🗂️ A counted list of claims.
    fn claims(&mut self) -> Vec<Claim> {
        (0..self.count()).map(|_| self.claim()).collect()
    }

    /// 🪵️ A perch: surface number, `x0, x1, y`.
    fn perch(&mut self) -> Perch {
        Perch { surface: format!("s{}", self.count()), x0: self.number(), x1: self.number(), y: self.number() }
    }

    /// 🔤️ A counted list of owners.
    fn owners(&mut self) -> Vec<Slug> {
        (0..self.count()).map(|_| self.owner()).collect()
    }

    /// 📍️ The next two numbers as a point.
    fn point(&mut self) -> Point {
        Point { x: self.number(), y: self.number() }
    }

    /// ⬛️ The next four numbers as a rect.
    fn rect(&mut self) -> Rect {
        Rect { x: self.number(), y: self.number(), width: self.number(), height: self.number() }
    }

    /// 🤏️ A press: phase index, `x, y, since, slop`.
    fn press(&mut self) -> Press {
        Press { phase: PRESS_PHASES[self.count()], x: self.number(), y: self.number(), since: self.ticks(), slop: self.number() }
    }

    /// 🛡️ Four flags as guards.
    fn guards(&mut self) -> Guards {
        Guards { control: self.flag(), scrolled: self.flag(), quiet: self.flag(), still: self.flag() }
    }

    /// 🔥️ A warmth: `heat, since, until`, tier index, `run, tricks`.
    fn warmth(&mut self) -> Warmth {
        Warmth { heat: self.number(), since: self.ticks(), until: self.ticks(), tier: TIERS[self.count()], run: self.ticks(), tricks: self.ticks() }
    }

    /// 💆️ A caress: 0 a click, 1 a hold.
    fn caress(&mut self) -> Caress {
        if self.flag() {
            Caress::Hold
        } else {
            Caress::Click
        }
    }

    /// 🌀️ A circling state in the order of its fields.
    fn circling(&mut self) -> Circling {
        Circling {
            live: self.flag(),
            inside: self.ticks(),
            sx: self.ticks(),
            sy: self.ticks(),
            px: self.number(),
            py: self.number(),
            turn: self.ticks(),
            quarters: self.ticks(),
            steps: self.ticks(),
            against: self.ticks(),
            first: self.ticks(),
            last: self.ticks(),
            open: self.number(),
            near: self.number(),
            far: self.number(),
            rest: self.ticks(),
        }
    }

    /// 🖐️ A stroking state in the order of its fields.
    fn stroking(&mut self) -> Stroking {
        Stroking {
            live: self.flag(),
            way: self.ticks(),
            from: self.number(),
            began: self.ticks(),
            peak: self.number(),
            reached: self.ticks(),
            top: self.number(),
            bottom: self.number(),
            top_since: self.number(),
            bottom_since: self.number(),
            count: self.ticks(),
            first: self.ticks(),
            second: self.ticks(),
            rest: self.ticks(),
        }
    }

    /// 🫨️ A shaking state in the order of its fields.
    fn shaking(&mut self) -> Shaking {
        Shaking {
            live: self.flag(),
            ax: self.number(),
            ay: self.number(),
            at: self.ticks(),
            fx: self.number(),
            fy: self.number(),
            reached: self.ticks(),
            count: self.ticks(),
            mark1: self.ticks(),
            mark2: self.ticks(),
            mark3: self.ticks(),
            rest: self.ticks(),
        }
    }

    /// 🪶️ A hover state: circling, then stroking.
    fn hover(&mut self) -> Hover {
        Hover { circling: self.circling(), stroking: self.stroking() }
    }
}

/// 🆔️ The number of the owner `o<k>`.
fn owner_number(owner: &str) -> f64 {
    owner[1..].parse::<f64>().unwrap_or(f64::NAN)
}

/// 🧱️ A box flattened.
fn flat_extent(extent: Extent) -> [f64; 4] {
    [extent.x0, extent.y0, extent.x1, extent.y1]
}

/// 🗃️ Boxes flattened: count, then the boxes.
fn flat_extents(extents: &[Extent]) -> Vec<f64> {
    std::iter::once(extents.len() as f64).chain(extents.iter().flat_map(|extent| flat_extent(*extent))).collect()
}

/// 🎟️ A claim flattened like the TypeScript half does.
fn flat_claim(claim: &Claim) -> Vec<f64> {
    let mut flat = vec![owner_number(&claim.owner), claim.slices.len() as f64];
    for slice in &claim.slices {
        flat.extend([slice.from as f64, slice.until as f64]);
        flat.extend(flat_extent(slice.extent));
    }
    flat.push(truth(claim.rest.is_some()));
    flat.extend(flat_extent(claim.rest.unwrap_or(Extent { x0: 0.0, y0: 0.0, x1: 0.0, y1: 0.0 })));
    flat
}

/// 📇️ Claims flattened: count, then the claims.
fn flat_claims(claims: &[Claim]) -> Vec<f64> {
    std::iter::once(claims.len() as f64).chain(claims.iter().flat_map(flat_claim)).collect()
}

/// 👥️ Pairs flattened: count, then the owners of each pair.
fn flat_pairs(pairs: &[Pair]) -> Vec<f64> {
    std::iter::once(pairs.len() as f64).chain(pairs.iter().flat_map(|pair| [owner_number(&pair.first), owner_number(&pair.second)])).collect()
}

/// 🔠️ Owners flattened: count, then the owner numbers.
fn flat_owners(owners: &[Slug]) -> Vec<f64> {
    std::iter::once(owners.len() as f64).chain(owners.iter().map(|owner| owner_number(owner))).collect()
}

/// ❔️ An optional number flattened: a flag and the number.
fn flat_maybe(value: Option<f64>) -> Vec<f64> {
    value.map_or(vec![0.0, 0.0], |value| vec![1.0, value])
}

/// 🧍️ A posture flattened.
fn flat_posture(posture: Posture) -> Vec<f64> {
    vec![posture.left, posture.right, posture.above]
}

/// 👇️ A press flattened: phase index, `x, y, since, slop`.
fn flat_press(press: Press) -> Vec<f64> {
    vec![PRESS_PHASES.iter().position(|phase| *phase == press.phase).map_or(f64::NAN, |index| index as f64), press.x, press.y, press.since as f64, press.slop]
}

/// 🚧️ Guards flattened.
fn flat_guards(guards: Guards) -> Vec<f64> {
    vec![truth(guards.control), truth(guards.scrolled), truth(guards.quiet), truth(guards.still)]
}

/// 🏅️ The index of a tier.
fn tier_index(tier: Tier) -> f64 {
    TIERS.iter().position(|candidate| *candidate == tier).map_or(f64::NAN, |index| index as f64)
}

/// 🌡️ A warmth flattened.
fn flat_warmth(warmth: Warmth) -> Vec<f64> {
    vec![warmth.heat, warmth.since as f64, warmth.until as f64, tier_index(warmth.tier), warmth.run as f64, warmth.tricks as f64]
}

/// 🔄️ A circling state flattened.
fn flat_circling(circling: Circling) -> Vec<f64> {
    vec![
        truth(circling.live),
        circling.inside as f64,
        circling.sx as f64,
        circling.sy as f64,
        circling.px,
        circling.py,
        circling.turn as f64,
        circling.quarters as f64,
        circling.steps as f64,
        circling.against as f64,
        circling.first as f64,
        circling.last as f64,
        circling.open,
        circling.near,
        circling.far,
        circling.rest as f64,
    ]
}

/// ✋️ A stroking state flattened.
fn flat_stroking(stroking: Stroking) -> Vec<f64> {
    vec![
        truth(stroking.live),
        stroking.way as f64,
        stroking.from,
        stroking.began as f64,
        stroking.peak,
        stroking.reached as f64,
        stroking.top,
        stroking.bottom,
        stroking.top_since,
        stroking.bottom_since,
        stroking.count as f64,
        stroking.first as f64,
        stroking.second as f64,
        stroking.rest as f64,
    ]
}

/// 📳️ A shaking state flattened.
fn flat_shaking(shaking: Shaking) -> Vec<f64> {
    vec![truth(shaking.live), shaking.ax, shaking.ay, shaking.at as f64, shaking.fx, shaking.fy, shaking.reached as f64, shaking.count as f64, shaking.mark1 as f64, shaking.mark2 as f64, shaking.mark3 as f64, shaking.rest as f64]
}

/// 🎐️ A hover state flattened.
fn flat_hover(hover: Hover) -> Vec<f64> {
    let mut flat = flat_circling(hover.circling);
    flat.extend(flat_stroking(hover.stroking));
    flat
}

/// 🔔️ A cue as its index in `CUES`, −1 for none.
fn cue_index(cue: Option<Cue>) -> f64 {
    cue.map_or(-1.0, |cue| CUES.iter().position(|candidate| *candidate == cue).map_or(f64::NAN, |index| index as f64))
}

/// 📣️ A press signal as its index, −1 for none.
fn signal_index(signal: Option<PressSignal>) -> f64 {
    const SIGNALS: [PressSignal; 6] = [PressSignal::Click, PressSignal::Hold, PressSignal::Unhold, PressSignal::Lift, PressSignal::Drop, PressSignal::Abort];
    signal.map_or(-1.0, |signal| SIGNALS.iter().position(|candidate| *candidate == signal).map_or(f64::NAN, |index| index as f64))
}

/// 🧭️ A side as the number the TypeScript twin answers.
fn side_number(side: Facing) -> f64 {
    side.sign()
}

/// 🧲️ A recorded side as a facing.
fn facing_of(number: f64) -> Facing {
    if number > 0.0 {
        Facing::Right
    } else {
        Facing::Left
    }
}

/// 📥️ A recorded press input: kind, `x, y`, pointer index.
fn press_input(kind: usize, x: f64, y: f64, pointer: Pointer) -> PressInput {
    match kind {
        0 => PressInput::Pressed(Pressed { x, y, pointer }),
        1 => PressInput::Dragged(Dragged { x, y }),
        2 => PressInput::Released(Released { x, y }),
        3 => PressInput::Cancelled,
        _ => PressInput::Ticked,
    }
}

/// ⚙️ What the Rust twins answer to one case.
fn answer(function: &str, arguments: &mut Arguments) -> Vec<f64> {
    match function {
        "clearance_constants" => {
            let mut values = vec![MARGIN, SEAM, FOREVER as f64, PUSHES as f64, PATIENCE as f64, DELAY as f64, SLIDE_OFF_SPEED, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT, LEAN, LANE_LIFT, SCOOT_HASTE];
            values.extend(STEERINGS);
            values.extend(flat_posture(UPRIGHT));
            values.extend([TALLY.ticks, TALLY.actors, TALLY.overlaps, TALLY.nears, TALLY.poofs, TALLY.waits].map(|count| count as f64));
            values
        }
        "leaning" => flat_posture(leaning(arguments.number(), arguments.number())),
        "canopied" => {
            let size = Size { width: arguments.number(), height: arguments.number() };
            let canopy = Size { width: arguments.number(), height: arguments.number() };
            flat_posture(canopied(size, canopy))
        }
        "body_of" => {
            arguments.count();
            let feet = arguments.point();
            let size = Size { width: arguments.number(), height: arguments.number() };
            let hover = arguments.number();
            let posture = Posture { left: arguments.number(), right: arguments.number(), above: arguments.number() };
            flat_extent(body_of("o1", feet, size, hover, posture, arguments.number()).extent).to_vec()
        }
        "shifted" => flat_extent(shifted(arguments.extent(), arguments.number(), arguments.number())).to_vec(),
        "grown" => flat_extent(grown(arguments.extent(), arguments.number())).to_vec(),
        "united" => flat_extent(united(arguments.extent(), arguments.extent())).to_vec(),
        "meets" => vec![truth(meets(arguments.extent(), arguments.extent()))],
        "overlaps" => flat_pairs(&overlaps(&arguments.bodies())),
        "near_misses" => {
            let bodies = arguments.bodies();
            flat_pairs(&near_misses(&bodies, arguments.number()))
        }
        "obstacles_of" => {
            let owner = arguments.owner();
            let (bodies, claims) = (arguments.bodies(), arguments.claims());
            flat_extents(&obstacles_of(&owner, &bodies, &claims, arguments.ticks()))
        }
        "free_at" => {
            let (extent, owner) = (arguments.extent(), arguments.owner());
            let (bodies, claims) = (arguments.bodies(), arguments.claims());
            vec![truth(free_at(extent, &owner, &bodies, &claims, arguments.ticks()))]
        }
        "free_among" => {
            let extent = arguments.extent();
            vec![truth(free_among(extent, &arguments.extents()))]
        }
        "slot_in" => {
            let (x, extent, low, high) = (arguments.number(), arguments.extent(), arguments.number(), arguments.number());
            flat_maybe(slot_in(x, extent, low, high, &arguments.extents()))
        }
        "spot_on" => {
            let (perch, x, extent, foot) = (arguments.perch(), arguments.number(), arguments.extent(), arguments.number());
            flat_maybe(spot_on(&perch, x, extent, foot, &arguments.extents()))
        }
        "column_over" => {
            let (perch, x, extent, drop, foot) = (arguments.perch(), arguments.number(), arguments.extent(), arguments.number(), arguments.number());
            flat_maybe(column_over(&perch, x, extent, drop, foot, &arguments.extents()))
        }
        "claim_of" => {
            arguments.count();
            let (from, extents, span) = (arguments.ticks(), arguments.extents(), arguments.ticks());
            flat_claim(&claim_of("o1", from, &extents, span, arguments.maybe_extent()))
        }
        "slice_at" => {
            let claim = arguments.claim();
            slice_at(&claim, arguments.ticks()).map_or(vec![0.0, 0.0, 0.0, 0.0, 0.0], |extent| std::iter::once(1.0).chain(flat_extent(extent)).collect())
        }
        "claim_clear" => {
            let (plan, bodies, claims) = (arguments.claim(), arguments.bodies(), arguments.claims());
            vec![truth(claim_clear(&plan, &bodies, &claims))]
        }
        "released" => {
            let claims = arguments.claims();
            flat_claims(&released(&claims, &arguments.owner()))
        }
        "pruned" => {
            let claims = arguments.claims();
            flat_claims(&pruned(&claims, arguments.ticks()))
        }
        "guarded_stride" => {
            let (extent, stride) = (arguments.extent(), arguments.number());
            vec![guarded_stride(extent, stride, &arguments.extents())]
        }
        "vaults" => vec![truth(vaults(arguments.extent(), arguments.number(), arguments.extent()))],
        "order_of" => flat_owners(&order_of(&arguments.bodies())),
        "order_kept" => {
            let before = arguments.owners();
            vec![truth(order_kept(&before, &arguments.owners()))]
        }
        "seat_of" => {
            let (bodies, perch) = (arguments.bodies(), arguments.perch());
            let seating = seat_of(&bodies, &perch, arguments.number());
            let mut flat = vec![seating.seats.len() as f64];
            for seat in &seating.seats {
                flat.extend([owner_number(&seat.owner), seat.shift]);
            }
            flat.extend(flat_owners(&seating.leavers));
            flat
        }
        "scoot_fraction" => {
            let shifts: Vec<f64> = (0..arguments.count()).map(|_| arguments.number()).collect();
            vec![scoot_fraction(&shifts, arguments.number())]
        }
        "scooted" => vec![scooted(arguments.number(), arguments.number(), arguments.number())],
        "head_under" => {
            let (before, after, owner, bodies) = (arguments.extent(), arguments.extent(), arguments.owner(), arguments.bodies());
            vec![head_under(before, after, &owner, &bodies).map_or(-1.0, |host| bodies.iter().position(|body| std::ptr::eq(body, host)).map_or(f64::NAN, |index| index as f64))]
        }
        "lift_onto" => vec![lift_onto(arguments.extent(), arguments.extent())],
        "rests_on" => vec![truth(rests_on(arguments.extent(), arguments.extent()))],
        "slide_side" => vec![side_number(slide_side(arguments.extent(), arguments.extent()))],
        "slide_of" => {
            let (rider, side, ticks, low, high) = (arguments.extent(), facing_of(arguments.number()), arguments.ticks(), arguments.number(), arguments.number());
            let slide = slide_of(rider, side, ticks, low, high, &arguments.extents());
            vec![slide.stride, side_number(slide.side)]
        }
        "slide_stride" => vec![slide_stride(arguments.ticks())],
        "pushed_out" => {
            let (extent, obstacles) = (arguments.extent(), arguments.extents());
            pushed_out(extent, &obstacles, arguments.count()).map_or(vec![0.0, 0.0, 0.0], |push| vec![1.0, push.x, push.y])
        }
        "must_poof" => {
            let (owner, stay, plans, bodies, claims) = (arguments.owner(), arguments.maybe_extent(), arguments.claims(), arguments.bodies(), arguments.claims());
            vec![truth(must_poof(&owner, stay, &plans, &bodies, &claims, arguments.ticks()))]
        }
        "evicted" => {
            let (bodies, claims) = (arguments.bodies(), arguments.claims());
            flat_owners(&evicted(&bodies, &claims, arguments.ticks()))
        }
        "lane_lift" => vec![lane_lift(arguments.number(), arguments.number())],
        "per_million" => vec![per_million(arguments.number() as u64, arguments.number() as u64)],
        "tallied" => {
            let tally = Tally { ticks: arguments.number() as u64, actors: arguments.number() as u64, overlaps: arguments.number() as u64, nears: arguments.number() as u64, poofs: arguments.number() as u64, waits: arguments.number() as u64 };
            let bodies = arguments.bodies();
            let next = tallied(tally, &bodies, arguments.number(), arguments.number() as u64, arguments.number() as u64);
            [next.ticks, next.actors, next.overlaps, next.nears, next.poofs, next.waits].map(|count| count as f64).to_vec()
        }
        "gesture_constants" => {
            let mut values = vec![SLOP_FINE, SLOP_COARSE, HOLD_TICKS as f64, HEAT_CLICK, HEAT_HOLD, HEAT_LEAK, HEAT_HELLO, HEAT_TRICK, HEAT_ENOUGH, HEAT_FORGIVEN, ENOUGH_TICKS as f64, SCROLL_TICKS as f64];
            values.extend([CIRCLE_MARGIN, CIRCLE_REACH, CIRCLE_HYSTERESIS, CIRCLE_QUARTERS as f64, CIRCLE_FAST as f64, CIRCLE_SLOW as f64, CIRCLE_ROUND, CIRCLE_CLOSE, CIRCLE_AGAINST as f64, CIRCLE_OUT_TICKS as f64, CIRCLE_REST as f64]);
            values.extend([STROKE_MARGIN, STROKE_HYSTERESIS, STROKE_LENGTH, STROKE_SLOW, STROKE_FAST, STROKE_SLANT, STROKE_SEGMENTS as f64, STROKE_WINDOW as f64, STROKE_PAUSE as f64]);
            values.extend([SHAKE_AMPLITUDE, SHAKE_HYSTERESIS, SHAKE_SPEED, SHAKE_REVERSALS as f64, SHAKE_WINDOW as f64, SHAKE_PAUSE as f64, SHAKE_REST as f64]);
            values.extend(flat_press(IDLE));
            values.extend(flat_warmth(COLD));
            values.extend(flat_guards(UNGUARDED));
            values
        }
        "slop_of" => vec![slop_of(POINTERS[arguments.count()])],
        "press_step" => {
            let press = arguments.press();
            let (kind, x, y, pointer) = (arguments.count(), arguments.number(), arguments.number(), POINTERS[arguments.count()]);
            let (tick, guards) = (arguments.ticks(), arguments.guards());
            let step = press_step(press, press_input(kind, x, y, pointer), tick, guards);
            let mut flat = flat_press(step.state);
            flat.push(signal_index(step.signal));
            flat
        }
        "press_due" => flat_maybe(press_due(arguments.press()).map(|due| due as f64)),
        "heat_at" => vec![heat_at(arguments.number(), arguments.ticks(), arguments.ticks())],
        "heat_after" => vec![heat_after(arguments.number(), arguments.ticks(), arguments.ticks(), arguments.caress())],
        "tier_of" => vec![tier_index(tier_of(arguments.number()))],
        "warmth_after" => flat_warmth(warmth_after(arguments.warmth(), arguments.ticks(), arguments.caress())),
        "no_circling" => flat_circling(no_circling(arguments.ticks())),
        "no_stroking" => flat_stroking(no_stroking(arguments.ticks())),
        "no_shaking" => flat_shaking(no_shaking(arguments.ticks())),
        "no_hover" => flat_hover(no_hover(arguments.ticks())),
        "circle_step" => {
            let step = circle_step(arguments.circling(), arguments.point(), arguments.rect(), arguments.ticks(), arguments.guards());
            let mut flat = flat_circling(step.state);
            flat.push(cue_index(step.cue));
            flat
        }
        "stroke_step" => {
            let step = stroke_step(arguments.stroking(), arguments.point(), arguments.rect(), arguments.ticks(), arguments.guards());
            let mut flat = flat_stroking(step.state);
            flat.push(cue_index(step.cue));
            flat
        }
        "hover_step" => {
            let step = hover_step(arguments.hover(), arguments.point(), arguments.rect(), arguments.ticks(), arguments.guards());
            let mut flat = flat_hover(step.state);
            flat.push(cue_index(step.cue));
            flat
        }
        "hover_busy" => vec![truth(hover_busy(&arguments.hover()))],
        "shake_step" => {
            let step = shake_step(arguments.shaking(), arguments.point(), arguments.number(), arguments.ticks(), arguments.guards());
            let mut flat = flat_shaking(step.state);
            flat.push(cue_index(step.cue));
            flat
        }
        other => panic!("no Rust twin is wired for {other}"),
    }
}

/// 🧾️ A running FNV-1a digest on 32-bit words, the TypeScript half's `Digest`.
struct Digest(u32);

impl Digest {
    /// ➕️ Folds the two words of a number.
    fn number(&mut self, number: f64) {
        let bits = number.to_bits();
        self.word((bits >> 32) as u32);
        self.word(bits as u32);
    }

    /// 🔩️ Folds one word.
    fn word(&mut self, word: u32) {
        self.0 = (self.0 ^ word).wrapping_mul(16_777_619);
    }

    /// 🔏️ The digest as eight hexadecimal digits.
    fn hex(&self) -> String {
        format!("{:08x}", self.0)
    }
}

/// 🧵️ A pointer path decoded into one point per tick, in quanta.
struct Trail {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

/// 🔣️ The whole numbers of a list.
fn integers(value: &Value) -> Vec<i64> {
    value.as_array().into_iter().flatten().filter_map(Value::as_f64).map(|number| number as i64).collect()
}

/// 🧶️ The points of a path, one per tick (the subject adapters' decoding).
fn trail_of(path: &[i64], hold: i64) -> Trail {
    let (mut x, mut y) = (path[0], path[1]);
    let mut trail = Trail { xs: vec![x], ys: vec![y] };
    let mut index = 2;
    while index < path.len() {
        let token = path[index];
        if token >= hold {
            for _ in 0..token - hold {
                trail.xs.push(x);
                trail.ys.push(y);
            }
            index += 1;
        } else {
            x += token;
            y += path[index + 1];
            trail.xs.push(x);
            trail.ys.push(y);
            index += 2;
        }
    }
    trail
}

/// 🪞️ A trail in one of its sixteen variants.
fn variant_of(trail: &Trail, page: [i64; 2], variant: u32) -> Trail {
    let mirrored: Vec<i64> = if variant & 1 == 0 { trail.xs.clone() } else { trail.xs.iter().map(|x| page[0] - x).collect() };
    let flipped: Vec<i64> = if variant & 2 == 0 { trail.ys.clone() } else { trail.ys.iter().map(|y| page[1] - y).collect() };
    let (mut xs, mut ys) = if variant & 4 == 0 { (mirrored, flipped) } else { (flipped, mirrored) };
    if variant & 8 != 0 {
        xs.reverse();
        ys.reverse();
    }
    Trail { xs, ys }
}

/// 🖼️ A body in the same variant.
fn box_variant(body: &[i64], page: [i64; 2], variant: u32) -> [i64; 4] {
    let x = if variant & 1 == 0 { body[0] } else { page[0] - body[0] - body[2] };
    let y = if variant & 2 == 0 { body[1] } else { page[1] - body[1] - body[3] };
    if variant & 4 == 0 {
        [x, y, body[2], body[3]]
    } else {
        [y, x, body[3], body[2]]
    }
}

/// 🎛️ The guards with the named one up, or none.
fn guard_of(vector: &Value) -> Guards {
    match vector.get("guard").and_then(Value::as_str) {
        Some("control") => Guards { control: true, ..UNGUARDED },
        Some("scrolled") => Guards { scrolled: true, ..UNGUARDED },
        Some("quiet") => Guards { quiet: true, ..UNGUARDED },
        Some("still") => Guards { still: true, ..UNGUARDED },
        _ => UNGUARDED,
    }
}

/// 🎠️ The cues and the state digest of the hover gestures along a trail past one body.
fn hover_run(trail: &Trail, quantum: f64, body: [i64; 4], guards: Guards, digested: bool) -> (Vec<[f64; 2]>, String) {
    let rect = Rect { x: body[0] as f64 / quantum, y: body[1] as f64 / quantum, width: body[2] as f64 / quantum, height: body[3] as f64 / quantum };
    let (mut cues, mut digest, mut hover) = (Vec::new(), Digest(2_166_136_261), no_hover(0));
    for (tick, (x, y)) in trail.xs.iter().zip(&trail.ys).enumerate() {
        let step = hover_step(hover, Point { x: *x as f64 / quantum, y: *y as f64 / quantum }, rect, tick as Ticks, guards);
        hover = step.state;
        if step.cue.is_some() {
            cues.push([tick as f64, cue_index(step.cue)]);
        }
        if digested {
            for number in flat_hover(hover) {
                digest.number(number);
            }
        }
    }
    (cues, if digested { digest.hex() } else { String::new() })
}

/// 🥤️ The cues and the state digest of the shake along a trail.
fn held_run(trail: &Trail, quantum: f64, height: f64, guards: Guards) -> (Vec<[f64; 2]>, String) {
    let (mut cues, mut digest, mut shaking) = (Vec::new(), Digest(2_166_136_261), no_shaking(0));
    for (tick, (x, y)) in trail.xs.iter().zip(&trail.ys).enumerate() {
        let step = shake_step(shaking, Point { x: *x as f64 / quantum, y: *y as f64 / quantum }, height / quantum, tick as Ticks, guards);
        shaking = step.state;
        if step.cue.is_some() {
            cues.push([tick as f64, cue_index(step.cue)]);
        }
        for number in flat_shaking(shaking) {
            digest.number(number);
        }
    }
    (cues, digest.hex())
}

/// 📼️ One replay of the corpus: kind, id, variant, body, cues and state digest.
type Replay = (String, String, u32, usize, Vec<[f64; 2]>, String);

/// 🎞️ Every replay of the committed gesture corpus in the order of the TypeScript half: kind, id, variant, body, cues and digest.
fn replays(document: &Value) -> Vec<Replay> {
    let quantum = document["quantum"].as_f64().unwrap_or(f64::NAN);
    let hold = document["hold"].as_f64().unwrap_or(0.0) as i64;
    let page = integers(&document["page"]);
    let page = [page[0], page[1]];
    let list = |name: &str| document[name].as_array().cloned().unwrap_or_default();
    let mut replays = Vec::new();
    for vector in list("presses") {
        let (mut signals, mut digest, mut press) = (Vec::new(), Digest(2_166_136_261), IDLE);
        for tick in 1..=vector["ticks"].as_f64().unwrap_or(0.0) as Ticks {
            let passed = press_step(press, PressInput::Ticked, tick, UNGUARDED);
            press = passed.state;
            if passed.signal.is_some() {
                signals.push([tick as f64, signal_index(passed.signal)]);
            }
            for event in vector["events"].as_array().into_iter().flatten() {
                if event["at"].as_f64().map(|at| at as Ticks) != Some(tick) {
                    continue;
                }
                let x = event.get("x").and_then(Value::as_f64).unwrap_or(0.0) / quantum;
                let y = event.get("y").and_then(Value::as_f64).unwrap_or(0.0) / quantum;
                let pointer = match event.get("pointer").and_then(Value::as_str) {
                    Some("pen") => Pointer::Pen,
                    Some("touch") => Pointer::Touch,
                    _ => Pointer::Mouse,
                };
                let kind = match event["kind"].as_str() {
                    Some("pressed") => 0,
                    Some("dragged") => 1,
                    Some("released") => 2,
                    _ => 3,
                };
                let names: Vec<&str> = event["guards"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
                let guards = Guards { control: names.contains(&"control"), scrolled: names.contains(&"scrolled"), quiet: names.contains(&"quiet"), still: names.contains(&"still") };
                let step = press_step(press, press_input(kind, x, y, pointer), tick, guards);
                press = step.state;
                if step.signal.is_some() {
                    signals.push([tick as f64, signal_index(step.signal)]);
                }
            }
            for number in flat_press(press) {
                digest.number(number);
            }
        }
        replays.push(("press".to_string(), vector["id"].as_str().unwrap_or_default().to_string(), 0, 0, signals, digest.hex()));
    }
    for vector in list("warmths") {
        let (mut digest, mut warmth) = (Digest(2_166_136_261), COLD);
        for caress in vector["caresses"].as_array().into_iter().flatten() {
            let kind = if caress[1] == "hold" { Caress::Hold } else { Caress::Click };
            warmth = warmth_after(warmth, caress[0].as_f64().unwrap_or(0.0) as Ticks, kind);
            for number in flat_warmth(warmth) {
                digest.number(number);
            }
        }
        replays.push(("warmth".to_string(), vector["id"].as_str().unwrap_or_default().to_string(), 0, 0, Vec::new(), digest.hex()));
    }
    for (kind, name) in [("circle", "circles"), ("stroke", "strokes")] {
        for vector in list(name) {
            let trail = trail_of(&integers(&vector["path"]), hold);
            for variant in 0..16 {
                let (cues, digest) = hover_run(&variant_of(&trail, page, variant), quantum, box_variant(&integers(&vector["body"]), page, variant), guard_of(&vector), true);
                replays.push((kind.to_string(), vector["id"].as_str().unwrap_or_default().to_string(), variant, 0, cues, digest));
            }
        }
    }
    for vector in list("shakes") {
        let trail = trail_of(&integers(&vector["path"]), hold);
        for variant in 0..16 {
            let (cues, digest) = held_run(&variant_of(&trail, page, variant), quantum, vector["height"].as_f64().unwrap_or(f64::NAN), guard_of(&vector));
            replays.push(("shake".to_string(), vector["id"].as_str().unwrap_or_default().to_string(), variant, 0, cues, digest));
        }
    }
    let bodies: Vec<Vec<i64>> = list("bodies").iter().map(integers).collect();
    for travel in list("travels") {
        let trail = trail_of(&integers(&travel["path"]), hold);
        for variant in 0..16 {
            let turned = variant_of(&trail, page, variant);
            for (index, body) in bodies.iter().enumerate() {
                let (cues, digest) = hover_run(&turned, quantum, box_variant(body, page, variant), UNGUARDED, variant == 0 || variant == 9);
                replays.push(("travel".to_string(), travel["id"].as_str().unwrap_or_default().to_string(), variant, index, cues, digest));
            }
        }
    }
    replays
}

/// 📂️ A JSON file of the scratch folder `🗑️generated/d2`.
fn generated(name: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").join(name);
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error} — run d2_dump_bits.ts first", path.display()))).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

#[test]
fn rust_twins_reproduce_every_bit_of_the_typescript_twins() {
    let cases = generated("bits.json");
    let cases = cases.as_array().cloned().unwrap_or_default();
    let mut tally: BTreeMap<String, [usize; 4]> = BTreeMap::new();
    let mut mismatches: Vec<String> = Vec::new();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for (index, case) in cases.iter().enumerate() {
        let function = case["fn"].as_str().unwrap_or_default();
        let texts = |value: &Value| -> Vec<String> { value.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect() };
        let arguments = texts(&case["args"]);
        let expected = if case["out"] == "throws" { None } else { Some(texts(&case["out"])) };
        let produced = catch_unwind(AssertUnwindSafe(|| {
            let mut reader = Arguments { values: arguments.iter().map(|text| decode(text)).collect(), at: 0 };
            let answered = answer(function, &mut reader);
            assert_eq!(reader.at, reader.values.len(), "arguments left over");
            answered.into_iter().map(encode).collect::<Vec<_>>()
        }))
        .ok();
        let counted = tally.entry(function.to_string()).or_default();
        counted[0] += 1;
        counted[1] += expected.as_ref().map_or(0, Vec::len);
        counted[2] += usize::from(expected.is_none());
        if produced != expected {
            counted[3] += 1;
            if mismatches.len() < 200 {
                mismatches.push(format!("case {index} {function}({}): typescript {expected:?}, rust {produced:?}", arguments.join(", ")));
            }
        }
    }
    std::panic::set_hook(hook);
    let mut report = String::new();
    let totals = tally.values().fold([0; 4], |sum, counted| [sum[0] + counted[0], sum[1] + counted[1], sum[2] + counted[2], sum[3] + counted[3]]);
    for (function, counted) in &tally {
        let _ = writeln!(report, "{function}: {} cases, {} result numbers, {} mismatches", counted[0], counted[1], counted[3]);
    }
    let _ = writeln!(report, "total: {} cases, {} result numbers, {} mismatches", totals[0], totals[1], totals[3]);
    for mismatch in &mismatches {
        let _ = writeln!(report, "{mismatch}");
    }
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    std::fs::write(directory.join("bits-report.txt"), &report).unwrap_or_else(|error| panic!("{error}"));
    println!("{report}");
    assert!(cases.len() > 50_000, "{} cases", cases.len());
    assert_eq!(totals[3], 0, "{} of {} cases differ", totals[3], cases.len());
}

#[test]
fn rust_recognisers_replay_the_committed_corpus_like_the_typescript_ones() {
    let recorded = generated("corpus.json");
    let recorded = recorded.as_array().cloned().unwrap_or_default();
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures/👆️gesture-recognition/🔣️.json");
    let document: Value = serde_json::from_str(&std::fs::read_to_string(&fixture).unwrap_or_else(|error| panic!("{}: {error}", fixture.display()))).unwrap_or_else(|error| panic!("{error}"));
    let replayed = replays(&document);
    let mut report = String::new();
    let mut tally: BTreeMap<String, [usize; 5]> = BTreeMap::new();
    let mut mismatches = 0;
    assert_eq!(replayed.len(), recorded.len(), "replays");
    for (ours, theirs) in replayed.iter().zip(&recorded) {
        let (kind, id, variant, body, cues, digest) = ours;
        let expected_cues: Vec<[f64; 2]> = theirs["cues"].as_array().into_iter().flatten().map(|cue| [cue[0].as_f64().unwrap_or(f64::NAN), cue[1].as_f64().unwrap_or(f64::NAN)]).collect();
        let same =
            theirs["kind"] == kind.as_str() && theirs["id"] == id.as_str() && theirs["variant"].as_u64() == Some(u64::from(*variant)) && theirs["body"].as_u64() == Some(*body as u64) && &expected_cues == cues && theirs["digest"] == digest.as_str();
        let counted = tally.entry(kind.clone()).or_default();
        counted[0] += 1;
        counted[1] += cues.len();
        counted[2] += usize::from(!digest.is_empty());
        if !same {
            counted[3] += 1;
            mismatches += 1;
            if mismatches <= 50 {
                let _ = writeln!(report, "{kind} {id} variant {variant} body {body}: typescript {} {}, rust {cues:?} {digest}", theirs["cues"], theirs["digest"]);
            }
        }
    }
    let mut lines = String::new();
    for (kind, counted) in &tally {
        let _ = writeln!(lines, "corpus {kind}: {} replays, {} cues, {} state digests, {} mismatches", counted[0], counted[1], counted[2], counted[3]);
    }
    let _ = writeln!(lines, "corpus: {} replays, {mismatches} mismatches", replayed.len());
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    std::fs::write(directory.join("corpus-report.txt"), format!("{lines}{report}")).unwrap_or_else(|error| panic!("{error}"));
    println!("{lines}{report}");
    assert_eq!(mismatches, 0);
}

//! 🧮️ Ticket tool of work package D1 (round 2), Rust half: recomputes every case of `🗑️generated/d1/gear-bits.json`
//! with the Rust twins of the swing and climbing modules, the terrain additions and the trigonometry additions, and
//! compares every result with the TypeScript twin's bit for bit.
//!
//! Mounted as the integration test `gear_bits` of the scratch crate (`[[test]] path = "../../../../../d1_check_gear_bits.rs"`
//! in `🗑️generated/d1/crate/📦️packages/🦀️rust/Cargo.toml`). Numbers arrive and are compared as sixteen hexadecimal
//! digits of their IEEE-754 pattern (`nan` for every NaN); a case the TypeScript twin throws on must panic here. The
//! flat layout of every argument and answer is the one `d1_dump_gear_bits.ts` documents. The per-function counts and
//! every mismatch are written to `🗑️generated/d1/gear-bits-report.txt`.
//!
//! Run from the repository root, after `bun <ticket>/d1_dump_gear_bits.ts` and `bun <ticket>/d1_scratch.ts prepare`:
//!   bash <ticket>/rust_scratch.sh d1 test --offline --test gear_bits -- --nocapture
//!
//! @see ./d1_dump_gear_bits.ts — the TypeScript half and the layout of every case
//! @see ./📓️report2-d1.md — the recorded result

use pets::climbing::*;
use pets::schema::{Canopy, Facing, Grip, Hang, Perch, Pitch, Point, Rect, Reeling, Shot, Size, Ticks, Wall, GEARS};
use pets::swing::*;
use pets::terrain::{nearest_wall, segment_clear, segment_hits, wall_at, walls_of, WALL_LIP};
use pets::trigonometry::{atan_turns, fast_neg_exp};
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

/// 📖️ The flat arguments of a case, read front to back.
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

    /// ⏱️ The next number as whole ticks.
    fn ticks(&mut self) -> Ticks {
        self.number() as Ticks
    }

    /// 🚩️ The next number as a flag.
    fn flag(&mut self) -> bool {
        self.number() == 1.0
    }

    /// 🧭️ The next number as a side.
    fn side(&mut self) -> Facing {
        if self.number() < 0.0 {
            Facing::Left
        } else {
            Facing::Right
        }
    }

    /// 🏷️ The next number as the name of a surface or a wall.
    fn name(&mut self, prefix: &str) -> String {
        format!("{prefix}{}", self.count())
    }

    /// 📍️ The next two numbers as a point.
    fn point(&mut self) -> Point {
        Point { x: self.number(), y: self.number() }
    }

    /// ⬛️ The next four numbers as a box.
    fn rect(&mut self) -> Rect {
        Rect { x: self.number(), y: self.number(), width: self.number(), height: self.number() }
    }

    /// 🐾️ The next two numbers as a size.
    fn size(&mut self) -> Size {
        Size { width: self.number(), height: self.number() }
    }

    /// 🪺️ A perch `surface, x0, x1, y`.
    fn perch(&mut self) -> Perch {
        Perch { surface: self.name("s"), x0: self.number(), x1: self.number(), y: self.number() }
    }

    /// 🧱️ A pitch `wall, surface, side, x, y0, y1`.
    fn pitch(&mut self) -> Pitch {
        Pitch { wall: self.name("w"), surface: self.name("s"), side: self.side(), x: self.number(), y0: self.number(), y1: self.number() }
    }

    /// 🏯️ A wall `id, surface, side, x, y0, y1`.
    fn wall(&mut self) -> Wall {
        Wall { id: self.name("w"), surface: self.name("s"), side: self.side(), x: self.number(), y0: self.number(), y1: self.number() }
    }

    /// 🪜️ A ladder `wall, surface, side, foot, top`.
    fn ladder(&mut self) -> LadderStand {
        LadderStand { wall: self.name("w"), surface: self.name("s"), side: self.side(), foot: self.point(), top: self.point() }
    }

    /// 🏹️ A shot `surface, facing, muzzle, hook, length, reel`.
    fn shot(&mut self) -> Shot {
        Shot { surface: self.name("s"), facing: self.side(), muzzle: self.point(), hook: self.point(), length: self.number(), reel: if self.number() == 0.0 { Reeling::Zip } else { Reeling::Swing } }
    }

    /// ✊️ A grip `x, y, vx, vy`.
    fn grip(&mut self) -> Grip {
        Grip { x: self.number(), y: self.number(), vx: self.number(), vy: self.number() }
    }

    /// 🐒️ A hang `grip, bob, previous`.
    fn hang(&mut self) -> Hang {
        Hang { grip: self.grip(), bob: self.point(), previous: self.point() }
    }

    /// ☂️ A canopy `x, y, vx, vy, bob, previous`.
    fn canopy(&mut self) -> Canopy {
        Canopy { x: self.number(), y: self.number(), vx: self.number(), vy: self.number(), bob: self.point(), previous: self.point() }
    }

    /// 🎒️ A chute `terminal, reach, flare, length`.
    fn chute(&mut self) -> Chute {
        Chute { terminal: self.number(), reach: self.number(), flare: self.number(), length: self.number() }
    }

    /// 🎣️ A haul `rope, hand, before, x, y`.
    fn haul(&mut self) -> Haul {
        Haul { rope: self.number(), hand: self.point(), before: self.point(), x: self.number(), y: self.number() }
    }

    /// 📚️ A counted list of entries.
    fn list<T>(&mut self, mut entry: impl FnMut(&mut Self) -> T) -> Vec<T> {
        let count = self.count();
        (0..count).map(|_| entry(self)).collect()
    }
}

/// 🔣️ The number at the end of a name.
fn numbered(name: &str) -> f64 {
    name[1..].parse::<f64>().unwrap_or(f64::NAN)
}

/// 🔎️ The index of an answered entry in its list, −1 for none.
fn index_of<T>(list: &[T], found: Option<&T>) -> f64 {
    found.map_or(-1.0, |found| list.iter().position(|candidate| std::ptr::eq(candidate, found)).map_or(f64::NAN, |index| index as f64))
}

/// 🧗️ A pitch as flat numbers.
fn pitch_flat(pitch: &Pitch) -> Vec<f64> {
    vec![numbered(&pitch.wall), numbered(&pitch.surface), pitch.side.sign(), pitch.x, pitch.y0, pitch.y1]
}

/// 🎋️ A ladder as flat numbers.
fn ladder_flat(ladder: &LadderStand) -> Vec<f64> {
    vec![numbered(&ladder.wall), numbered(&ladder.surface), ladder.side.sign(), ladder.foot.x, ladder.foot.y, ladder.top.x, ladder.top.y]
}

/// 🎯️ A shot as flat numbers.
fn shot_flat(shot: &Shot) -> Vec<f64> {
    vec![numbered(&shot.surface), shot.facing.sign(), shot.muzzle.x, shot.muzzle.y, shot.hook.x, shot.hook.y, shot.length, if shot.reel == Reeling::Zip { 0.0 } else { 1.0 }]
}

/// 🦧️ A hang as flat numbers.
fn hang_flat(hang: Hang) -> Vec<f64> {
    vec![hang.grip.x, hang.grip.y, hang.grip.vx, hang.grip.vy, hang.bob.x, hang.bob.y, hang.previous.x, hang.previous.y]
}

/// 🌂️ A canopy as flat numbers.
fn canopy_flat(canopy: Canopy) -> Vec<f64> {
    vec![canopy.x, canopy.y, canopy.vx, canopy.vy, canopy.bob.x, canopy.bob.y, canopy.previous.x, canopy.previous.y]
}

/// 🚠️ A haul as flat numbers.
fn haul_flat(haul: Haul) -> Vec<f64> {
    vec![haul.rope, haul.hand.x, haul.hand.y, haul.before.x, haul.before.y, haul.x, haul.y]
}

/// 🧤️ A hold as flat numbers.
fn hold_flat(hold: WallHold) -> Vec<f64> {
    vec![hold.x, hold.y, if hold.over { 1.0 } else { 0.0 }]
}

/// 🤾️ A toss as flat numbers.
fn toss_flat(toss: Toss) -> Vec<f64> {
    vec![toss.vx, toss.vy]
}

/// ❓️ An answer that may be absent: `0`, or `1` followed by its fields.
fn optional<T>(found: Option<T>, flat: impl Fn(T) -> Vec<f64>) -> Vec<f64> {
    found.map_or_else(|| vec![0.0], |found| std::iter::once(1.0).chain(flat(found)).collect())
}

/// 📃️ A counted list of answers.
fn listed<T>(items: &[T], flat: impl Fn(&T) -> Vec<f64>) -> Vec<f64> {
    std::iter::once(items.len() as f64).chain(items.iter().flat_map(flat)).collect()
}

/// 🗺️ A list of legs as flat numbers (the layout of `legsFlat` in the TypeScript half).
fn legs_flat(legs: Option<Vec<Leg<'_>>>, pitches: &[Pitch], ladders: &[LadderStand]) -> Vec<f64> {
    let Some(legs) = legs else {
        return vec![0.0];
    };
    let mut flat = vec![1.0, legs.len() as f64];
    for leg in &legs {
        match leg {
            Leg::Ladder { at, ladder, up } => flat.extend([0.0, *at, index_of(ladders, Some(*ladder)), if *up { 1.0 } else { 0.0 }]),
            Leg::Wall { at, pitch, hold, exit, goal } => {
                flat.extend([1.0, *at, index_of(pitches, Some(*pitch))]);
                flat.extend(hold_flat(*hold));
                flat.extend([index_of(pitches, Some(*exit)), *goal]);
            }
            Leg::Raise { at, ladder } => {
                flat.extend([2.0, *at]);
                flat.extend(ladder_flat(ladder));
            }
            Leg::Grapple { at, shot } => {
                flat.extend([3.0, *at]);
                flat.extend(shot_flat(shot));
            }
        }
    }
    flat
}

/// 🎚️ Every constant of the swing and climbing modules and the wall lip, in the order of the TypeScript half.
fn constants() -> Vec<f64> {
    let mut all = vec![WALL_LIP, HANG_ROD, HANG_GRAVITY, HANG_DAMPING, HANG_CONE, FOLLOW_STIFFNESS, FOLLOW_DAMPING];
    all.extend(RELEASE_WEIGHTS);
    all.extend([RELEASE_DIVISOR, RELEASE_STALE as f64, THROW_SHARE, THROW_LEAST, THROW_MOST, THROW_RISE, HARD_LANDING, CHUTE_OPENING, CHUTE_HEADROOM, CHUTE_REFLEX as f64, CHUTE_FACTOR, CHUTE_DESCENT, CHUTE_STEER_GAIN]);
    all.extend([CHUTE_STEER_SPEED, CHUTE_STEER_EASE, CHUTE_ROD, CHUTE_GRAVITY, CHUTE_DAMPING, CHUTE_FLARE, CHUTE_WIND, CHUTE_WIND_RATE, REEL_SPEED, REEL_RAMP as f64, REEL_LEAST, REEL_CAP, REEL_DAMPING]);
    all.extend([HAND_HEIGHT, CLIMB_RISE, CLIMB_DESCENT, CLIMB_RAMP as f64, GRIP_SPACING, GRIP_BUDGET, GRIP_CLIMB, GRIP_HANG, GRIP_REST, GRIP_BITE, WALL_FOLLOW, SLIDE_START, SLIDE_GAIN, SLIDE_SPEED, SLIP_PUSH, SLIP_LIFT]);
    all.extend([WALL_GRAB_TICKS as f64, WALL_HANG_TICKS as f64, MANTLE_TICKS as f64, MANTLE_INSET, HOIST_HUMP, HOIST_RISE]);
    all.extend([LADDER_LEAN, LADDER_STEEP, LADDER_FLAT, LADDER_SHORT, LADDER_TALL, LADDER_TUCK, LADDER_HORNS, RUNG_SPACING, LADDER_FOOTING, LADDER_GIRTH, LADDER_RISE, LADDER_DESCENT, LADDER_RAMP as f64]);
    all.extend([LADDER_EXIT, LADDER_FOLLOW, LADDER_SHIFT, LADDER_MOUNT_TICKS as f64, LADDER_DISMOUNT_TICKS as f64, LADDER_RAISE_TICKS as f64, LADDER_IDLE as f64, LADDER_LIFE as f64]);
    all.extend([TOPPLE_STIFFNESS, TOPPLE_DAMPING, TOPPLE_PUSH, TOPPLE_STEP]);
    all.extend([HOOK_SPEED, HOOK_RETURN, ROPE_SHORT, ROPE_LONG, ROPE_ELEVATION, ROPE_MARGIN, ROPE_DETOUR, ROPE_FOLLOW, ROPE_RISE, MUZZLE_FORWARD, MUZZLE_HEIGHT, HOOK_INSET, HOOK_LIFT, ZIP_SLANT, ZIP_SPEED]);
    all.extend([ZIP_RAMP as f64, ROPE_AIM_TICKS as f64, ROPE_RECOIL_TICKS as f64, ROPE_TUG_TICKS as f64, ROPE_HOIST_TICKS as f64, ROPE_SHRUG_TICKS as f64, ROPE_MISS_CHANCE, ROPE_MISS_OVERSHOOT, ROPE_REST as f64, ROPE_SULK as f64]);
    all.extend([CROSS_REACH, LUNGE_TICKS as f64]);
    all
}

/// ⛓️ The wall line of the pitch at `at` among `pitches` and where that pitch stands in it.
fn line_of(pitches: &[Pitch], at: usize, size: Size) -> (Vec<&Pitch>, usize) {
    let chain = chain_of(&pitches[at], pitches, size);
    let start = chain.iter().position(|member| std::ptr::eq(*member, &pitches[at])).unwrap_or(usize::MAX);
    (chain, start)
}

/// 🪨️ The way along a wall line a case of `wall_path` or `wall_cost` asks for.
fn walked(arguments: &mut Arguments) -> Vec<Clamber> {
    let pitches = arguments.list(Arguments::pitch);
    let (at, entry, x, y, end, goal, mantle) = (arguments.count(), [Entry::Grab, Entry::Hang, Entry::Cling][arguments.count()], arguments.number(), arguments.number(), arguments.count(), arguments.number(), arguments.flag());
    let size = arguments.size();
    let (chain, start) = line_of(&pitches, at, size);
    wall_path(&chain, start, entry, x, y, end, goal, mantle, size)
}

/// ⚙️ What the Rust twins answer to one case.
fn answer(function: &str, arguments: &mut Arguments) -> Vec<f64> {
    let point = |at: Point| vec![at.x, at.y];
    match function {
        "constants" => constants(),
        "walls_of" => {
            let (width, height, clearance, minimum) = (arguments.number(), arguments.number(), arguments.number(), arguments.number());
            let walls = arguments.list(Arguments::wall);
            let keepouts = arguments.list(Arguments::rect);
            listed(&walls_of(&walls, &keepouts, width, height, clearance, minimum), pitch_flat)
        }
        "wall_at" => {
            let pitches = arguments.list(Arguments::pitch);
            let wall = arguments.name("w");
            vec![index_of(&pitches, wall_at(&pitches, &wall, arguments.number()))]
        }
        "nearest_wall" => {
            let pitches = arguments.list(Arguments::pitch);
            vec![index_of(&pitches, nearest_wall(&pitches, arguments.number(), arguments.number()))]
        }
        "segment_hits" => {
            let (from, to, rect) = (arguments.point(), arguments.point(), arguments.rect());
            vec![if segment_hits(from, to, &rect, arguments.number()) { 1.0 } else { 0.0 }]
        }
        "segment_clear" => {
            let (from, to) = (arguments.point(), arguments.point());
            let rects = arguments.list(Arguments::rect);
            vec![if segment_clear(from, to, &rects, arguments.number()) { 1.0 } else { 0.0 }]
        }
        "atan_turns" => vec![atan_turns(arguments.number(), arguments.number())],
        "fast_neg_exp" => vec![fast_neg_exp(arguments.number())],
        "swing_step" => {
            let (before, now, bob, previous) = (arguments.point(), arguments.point(), arguments.point(), arguments.point());
            let (length, gravity, damping, rope) = (arguments.number(), arguments.number(), arguments.number(), arguments.flag());
            point(swing_step(before, now, bob, previous, length, gravity, damping, rope))
        }
        "cone_clamp" => point(cone_clamp(arguments.point(), arguments.point(), arguments.number())),
        "follow_step" => {
            let grip = follow_step(arguments.grip(), arguments.point());
            vec![grip.x, grip.y, grip.vx, grip.vy]
        }
        "hang_of" => hang_flat(hang_of(arguments.point(), arguments.number())),
        "hang_step" => hang_flat(hang_step(arguments.hang(), arguments.point(), arguments.number())),
        "lean_of" => vec![lean_of(arguments.point(), arguments.point(), arguments.number())],
        "ring_velocity" => point(ring_velocity(&arguments.list(Arguments::point))),
        "throw_of" => point(throw_of(arguments.point())),
        "release_velocity" => point(release_velocity(&arguments.list(Arguments::point))),
        "throw_velocity" => {
            let hang = arguments.hang();
            point(throw_velocity(hang, &arguments.list(Arguments::point)))
        }
        "impact_speed" => vec![impact_speed(arguments.number(), arguments.number())],
        "chute_opens" => vec![if chute_opens(arguments.number(), arguments.number()) { 1.0 } else { 0.0 }],
        "chute_of" => {
            let chute = chute_of(arguments.number());
            vec![chute.terminal, chute.reach, chute.flare, chute.length]
        }
        "canopy_of" => canopy_flat(canopy_of(arguments.point(), arguments.number(), arguments.number(), arguments.chute())),
        "flare_of" => vec![flare_of(arguments.number(), arguments.number())],
        "chute_wind" => vec![chute_wind(arguments.ticks(), arguments.number())],
        "chute_step" => canopy_flat(chute_step(arguments.canopy(), arguments.chute(), arguments.number(), arguments.number(), arguments.number())),
        "reel_step" => {
            let reel = reel_step(arguments.point(), arguments.point(), arguments.point(), arguments.number(), arguments.number(), arguments.ticks());
            vec![reel.bob.x, reel.bob.y, reel.length]
        }
        "hoist_path" => point(hoist_path(arguments.point(), arguments.point(), arguments.number(), arguments.number())),
        "cling_of" => vec![cling_of(&arguments.pitch(), arguments.size())],
        "ledge_of" => vec![ledge_of(&arguments.pitch(), arguments.size())],
        "rim_of" => vec![rim_of(&arguments.pitch(), arguments.size())],
        "foot_of" => vec![foot_of(&arguments.pitch(), arguments.size())],
        "climb_phase" => vec![climb_phase(&arguments.pitch(), arguments.number(), arguments.size())],
        "slip_of" => toss_flat(slip_of(&arguments.pitch())),
        "clings" => vec![if clings(&arguments.pitch(), arguments.number(), arguments.size()) { 1.0 } else { 0.0 }],
        "crowns" => vec![if crowns(&arguments.perch(), &arguments.pitch(), arguments.size()) { 1.0 } else { 0.0 }],
        "rim_for" => {
            let pitch = arguments.pitch();
            let perches = arguments.list(Arguments::perch);
            vec![index_of(&perches, rim_for(&pitch, &perches, arguments.size()))]
        }
        "grip_for" => optional(grip_for(&arguments.perch(), &arguments.pitch(), arguments.size()), hold_flat),
        "wall_holds" => {
            let pitch = arguments.pitch();
            let pitches = arguments.list(Arguments::pitch);
            vec![index_of(&pitches, wall_holds(&pitch, &pitches, arguments.number(), arguments.size()))]
        }
        "grip_step" => {
            let grip = arguments.number();
            vec![grip_step(grip, [Effort::Climb, Effort::Hang, Effort::Rest][arguments.count()])]
        }
        "climb_step" => vec![climb_step(arguments.number(), arguments.number(), arguments.ticks())],
        "climb_ticks" => vec![climb_ticks(arguments.number(), arguments.number()) as f64],
        "slide_step" => {
            let slid = slide_step(arguments.number(), arguments.number(), arguments.number());
            vec![slid.y, slid.vy]
        }
        "mantle_path" => point(mantle_path(&arguments.pitch(), arguments.size(), arguments.number())),
        "crossable" => vec![if crossable(&arguments.pitch(), &arguments.pitch(), arguments.size()) { 1.0 } else { 0.0 }],
        "lunge_path" => point(lunge_path(&arguments.pitch(), &arguments.pitch(), arguments.size(), arguments.number())),
        "chain_of" => {
            let pitches = arguments.list(Arguments::pitch);
            let at = arguments.count();
            let (chain, _) = line_of(&pitches, at, arguments.size());
            listed(&chain, |member| vec![index_of(&pitches, Some(*member))])
        }
        "wall_path" => {
            let works = [Handwork::Grab, Handwork::Hang, Handwork::Climb, Handwork::Lunge, Handwork::Mantle];
            listed(&walked(arguments), |clamber| vec![clamber.x, clamber.y, clamber.hold as f64, works.iter().position(|work| *work == clamber.work).map_or(f64::NAN, |index| index as f64)])
        }
        "wall_cost" => vec![wall_cost(&walked(arguments))],
        "ladder_for" => {
            let (low, high, pitch) = (arguments.perch(), arguments.perch(), arguments.pitch());
            let keepouts = arguments.list(Arguments::rect);
            optional(ladder_for(&low, &high, &pitch, &keepouts, arguments.size()), |ladder| ladder_flat(&ladder))
        }
        "ladder_holds" => {
            let ladder = arguments.ladder();
            let (perches, pitches, keepouts) = (arguments.list(Arguments::perch), arguments.list(Arguments::pitch), arguments.list(Arguments::rect));
            optional(ladder_holds(&ladder, &perches, &pitches, &keepouts, arguments.size()), |ladder| ladder_flat(&ladder))
        }
        "ladder_length" => vec![ladder_length(&arguments.ladder())],
        "ladder_lean" => vec![ladder_lean(&arguments.ladder())],
        "ladder_exit" => vec![ladder_exit(&arguments.ladder(), arguments.size())],
        "ladder_landing" => point(ladder_landing(&arguments.ladder(), arguments.size())),
        "ladder_rungs" => vec![f64::from(ladder_rungs(&arguments.ladder()))],
        "ladder_step" => vec![ladder_step(arguments.number(), arguments.number(), arguments.ticks())],
        "ladder_at" => point(ladder_at(&arguments.ladder(), arguments.number())),
        "ladder_phase" => vec![ladder_phase(arguments.number())],
        "spill_of" => optional(spill_of(&arguments.ladder(), arguments.number(), arguments.size()), toss_flat),
        "shot_for" => {
            let feet = arguments.point();
            let (perches, keepouts) = (arguments.list(Arguments::perch), arguments.list(Arguments::rect));
            optional(shot_for(feet, &perches, &keepouts, arguments.size()), |shot| shot_flat(&shot))
        }
        "shot_holds" => {
            let shot = arguments.shot();
            let (perches, keepouts) = (arguments.list(Arguments::perch), arguments.list(Arguments::rect));
            optional(shot_holds(&shot, &perches, &keepouts), |shot| shot_flat(&shot))
        }
        "sighted" => {
            let (from, to) = (arguments.point(), arguments.point());
            let keepouts = arguments.list(Arguments::rect);
            vec![if sighted(from, to, &keepouts, arguments.number(), arguments.point(), arguments.point()) { 1.0 } else { 0.0 }]
        }
        "miss_of" => point(miss_of(&arguments.shot(), &arguments.perch())),
        "landing_for" => point(landing_for(&arguments.shot(), &arguments.perch(), arguments.size())),
        "haul_of" => haul_flat(haul_of(&arguments.shot(), arguments.number(), arguments.size())),
        "hook_ticks" => vec![hook_ticks(arguments.point(), arguments.point(), arguments.number()) as f64],
        "hook_step" => point(hook_step(arguments.point(), arguments.point(), arguments.number(), arguments.ticks())),
        "zip_step" | "sway_step" | "haul_step" => {
            let (shot, haul, ticks, size) = (arguments.shot(), arguments.haul(), arguments.ticks(), arguments.size());
            haul_flat(match function {
                "zip_step" => zip_step(&shot, haul, ticks, size),
                "sway_step" => sway_step(&shot, haul, ticks, size),
                _ => haul_step(&shot, haul, ticks, size),
            })
        }
        "haul_ticks" => vec![haul_ticks(&arguments.shot(), arguments.size()) as f64],
        "route_of" => {
            let (x, from, to) = (arguments.number(), arguments.perch(), arguments.perch());
            let gear: Vec<_> = (0..arguments.count()).map(|_| GEARS[arguments.count()]).collect();
            let (size, grip) = (arguments.size(), arguments.number());
            let (pitches, ladders, keepouts) = (arguments.list(Arguments::pitch), arguments.list(Arguments::ladder), arguments.list(Arguments::rect));
            legs_flat(route_of(x, &from, &to, &gear, size, grip, &pitches, &ladders, &keepouts), &pitches, &ladders)
        }
        other => panic!("no Rust twin is wired for {other}"),
    }
}

#[test]
fn rust_twins_reproduce_every_bit_of_the_typescript_twins() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let source = directory.join("gear-bits.json");
    let cases: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(&source).unwrap_or_else(|error| panic!("{}: {error} — run d1_dump_gear_bits.ts first", source.display()))).unwrap_or_else(|error| panic!("{}: {error}", source.display()));
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
            mismatches.push(format!("case {index} {function}({}): typescript {expected:?}, rust {produced:?}", arguments.join(", ")));
        }
    }
    std::panic::set_hook(hook);
    let mut report = String::new();
    let totals = tally.values().fold([0; 4], |sum, counted| [sum[0] + counted[0], sum[1] + counted[1], sum[2] + counted[2], sum[3] + counted[3]]);
    for (function, counted) in &tally {
        let _ = writeln!(report, "{function}: {} cases, {} result numbers, {} throwing, {} mismatches", counted[0], counted[1], counted[2], counted[3]);
    }
    let _ = writeln!(report, "total: {} cases, {} result numbers, {} throwing, {} mismatches", totals[0], totals[1], totals[2], totals[3]);
    for mismatch in mismatches.iter().take(200) {
        let _ = writeln!(report, "{mismatch}");
    }
    std::fs::write(directory.join("gear-bits-report.txt"), &report).unwrap_or_else(|error| panic!("{error}"));
    println!("{report}");
    assert!(cases.len() > 5000, "{} cases", cases.len());
    assert!(mismatches.is_empty(), "{} of {} cases differ", mismatches.len(), cases.len());
}

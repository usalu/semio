//! 🧮️ Ticket tool of work package L, Rust half: recomputes every case of `🗑️generated/wp-l/motion-bits.json` with the
//! Rust animation and terrain twins and compares every result with the TypeScript twin's bit for bit.
//!
//! Mounted as the integration test `motion_bits` of a scratch crate (`[[test]] path = "../../../../../check_motion_bits.rs"`
//! in `🗑️generated/wp-l/crate/📦️packages/🦀️rust/Cargo.toml`). Numbers arrive and are compared as sixteen hexadecimal
//! digits of their IEEE-754 pattern (`nan` for every NaN); a case the TypeScript twin throws on must panic here. The
//! per-function counts and every mismatch are written to `🗑️generated/wp-l/motion-bits-report.txt`.
//!
//! Run from the repository root, after `bun <ticket>/dump_motion_bits.ts`:
//!   bash <ticket>/rust_scratch.sh wp-l test --offline --test motion_bits -- --nocapture
//!
//! @see ./dump_motion_bits.ts — the TypeScript half and the layout of every case
//! @see ./📓️report-wp-l.md — the recorded result

use pets::animation::{blend_pose, clip_ticks, ease_bezier, lid_at, sample_clip, sample_track, spring_step, BLINK_TICKS, GAZE_DAMPING, GAZE_STIFFNESS};
use pets::rig::BonePose;
use pets::schema::{Clip, Ease, Key, Perch, Point, Rect, Species, Surface, Ticks, Track, CHANNELS};
use pets::terrain::{fall_step, hop_landing, hop_of, hop_step, landing_of, nearest_perch, perch_at, perches_of, stride_to, FALL_SPEED, GRAVITY, HOP_CLEARANCE, HOP_DISTANCE, HOP_HEIGHT, HOP_STEEPNESS, HOP_TICKS};
use serde_json::{json, Value};
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

    /// 🧮️ The next number as a count or an index.
    fn count(&mut self) -> usize {
        self.number() as usize
    }

    /// ⏱️ The next number as whole ticks.
    fn ticks(&mut self) -> Ticks {
        self.number() as Ticks
    }

    /// 🎢️ The next four numbers as an easing.
    fn ease(&mut self) -> Ease {
        [self.number(), self.number(), self.number(), self.number()]
    }

    /// 🔑️ `count` keys of `at, value, eased, x1, y1, x2, y2`.
    fn keys(&mut self, count: usize) -> Vec<Key> {
        (0..count)
            .map(|_| {
                let (at, value, eased, ease) = (self.number(), self.number(), self.number(), self.ease());
                Key { at, value, ease: (eased == 1.0).then_some(ease) }
            })
            .collect()
    }

    /// 🧍️ `count` bone poses of `x, y, rotation, scaleX, scaleY`.
    fn pose(&mut self, count: usize) -> Vec<BonePose> {
        (0..count).map(|_| BonePose { x: self.number(), y: self.number(), rotation: self.number(), scale_x: self.number(), scale_y: self.number() }).collect()
    }

    /// 🪺️ A counted list of perches `surface, x0, x1, y`.
    fn perches(&mut self) -> Vec<Perch> {
        let count = self.count();
        (0..count).map(|_| Perch { surface: format!("s{}", self.count()), x0: self.number(), x1: self.number(), y: self.number() }).collect()
    }

    /// 📍️ The next two numbers as a point.
    fn point(&mut self) -> Point {
        Point { x: self.number(), y: self.number() }
    }
}

/// 🔎️ The index of an answered perch in its list, −1 for none.
fn index_of(perches: &[Perch], found: Option<&Perch>) -> f64 {
    found.map_or(-1.0, |found| perches.iter().position(|candidate| std::ptr::eq(candidate, found)).map_or(f64::NAN, |index| index as f64))
}

/// 🧬️ A species of `bones` bones `b0`, `b1`, … with nothing else on it.
fn species(bones: usize) -> Species {
    let bones: Vec<Value> = (0..bones).map(|bone| json!({"id": format!("b{bone}"), "x": 0, "y": 0})).collect();
    let text = json!({"en": "probe", "de": "Probe"});
    serde_json::from_value(json!({"id": "probe", "name": text, "thing": text, "grounds": [], "size": {"width": 1, "height": 1}, "palette": {"body": "#000000", "accent": "#000000", "detail": "#000000"}, "bones": bones, "parts": [], "face": {"eyes": []}, "clips": [], "repertoire": {}, "locomotion": {"gait": "walk", "speed": 1}, "temperament": {"energy": 0, "sociability": 0, "curiosity": 0}})).unwrap_or_else(|error| panic!("{error}"))
}

/// 🧍️ A pose as the flat numbers the TypeScript half records.
fn flat(pose: &[BonePose]) -> Vec<f64> {
    pose.iter().flat_map(|bone| [bone.x, bone.y, bone.rotation, bone.scale_x, bone.scale_y]).collect()
}

/// ⚙️ What the Rust twins answer to one case.
fn answer(function: &str, arguments: &mut Arguments) -> Vec<f64> {
    match function {
        "constants" => vec![GRAVITY, FALL_SPEED, HOP_CLEARANCE, HOP_STEEPNESS, HOP_HEIGHT, HOP_DISTANCE, HOP_TICKS as f64, GAZE_STIFFNESS, GAZE_DAMPING, BLINK_TICKS as f64],
        "ease_bezier" => {
            let ease = arguments.ease();
            vec![ease_bezier(ease, arguments.number())]
        }
        "sample_track" => {
            let channel = CHANNELS[arguments.count()];
            let count = arguments.count();
            let track = Track { bone: "body".to_string(), channel, keys: arguments.keys(count) };
            vec![sample_track(&track, arguments.number())]
        }
        "clip_ticks" => vec![clip_ticks(&Clip { id: "clip".to_string(), seconds: arguments.number(), looping: false, tracks: Vec::new() }) as f64],
        "sample_clip" => {
            let species = species(arguments.count());
            let (seconds, looping) = (arguments.number(), arguments.number() == 1.0);
            let tracks = (0..arguments.count())
                .map(|_| {
                    let (bone, channel, count) = (arguments.count(), CHANNELS[arguments.count()], arguments.count());
                    Track { bone: format!("b{bone}"), channel, keys: arguments.keys(count) }
                })
                .collect();
            let clip = Clip { id: "clip".to_string(), seconds, looping, tracks };
            flat(&sample_clip(&species, &clip, arguments.ticks()))
        }
        "blend_pose" => {
            let bones = arguments.count();
            let (from, to) = (arguments.pose(bones), arguments.pose(bones));
            flat(&blend_pose(&from, &to, arguments.number()))
        }
        "spring_step" => {
            let stepped = spring_step(arguments.number(), arguments.number(), arguments.number(), arguments.number(), arguments.number());
            vec![stepped.position, stepped.velocity]
        }
        "lid_at" => vec![lid_at(arguments.ticks())],
        "perches_of" => {
            let (width, height, clearance, minimum) = (arguments.number(), arguments.number(), arguments.number(), arguments.number());
            let surfaces: Vec<Surface> = (0..arguments.count()).map(|surface| Surface { id: format!("s{surface}"), x0: arguments.number(), x1: arguments.number(), y: arguments.number() }).collect();
            let keepouts: Vec<Rect> = (0..arguments.count()).map(|_| Rect { x: arguments.number(), y: arguments.number(), width: arguments.number(), height: arguments.number() }).collect();
            let found = perches_of(&surfaces, &keepouts, width, height, clearance, minimum);
            std::iter::once(found.len() as f64).chain(found.iter().flat_map(|perch| [perch.surface[1..].parse::<f64>().unwrap_or(f64::NAN), perch.x0, perch.x1, perch.y])).collect()
        }
        "perch_at" => {
            let perches = arguments.perches();
            let surface = format!("s{}", arguments.count());
            vec![index_of(&perches, perch_at(&perches, &surface, arguments.number()))]
        }
        "nearest_perch" => {
            let perches = arguments.perches();
            vec![index_of(&perches, nearest_perch(&perches, arguments.number(), arguments.number()))]
        }
        "stride_to" => vec![stride_to(arguments.number(), arguments.number(), arguments.number())],
        "fall_step" => {
            let fallen = fall_step(arguments.number(), arguments.number());
            vec![fallen.y, fallen.vy]
        }
        "landing_of" => {
            let perches = arguments.perches();
            vec![index_of(&perches, landing_of(&perches, arguments.number(), arguments.number(), arguments.number()))]
        }
        "hop_of" => hop_of(arguments.point(), arguments.point()).map_or(vec![0.0, 0.0, 0.0, 0.0], |hop| vec![1.0, hop.vx, hop.vy, hop.ticks as f64]),
        "hop_step" => {
            let flown = hop_step(arguments.number(), arguments.number(), arguments.number(), arguments.number(), arguments.point(), arguments.ticks());
            vec![flown.x, flown.y, flown.vx, flown.vy]
        }
        "hop_landing" => {
            let perches = arguments.perches();
            let (from, to) = (arguments.point(), arguments.point());
            vec![hop_of(from, to).map_or(-2.0, |hop| index_of(&perches, hop_landing(&perches, from, to, hop)))]
        }
        other => panic!("no Rust twin is wired for {other}"),
    }
}

#[test]
fn rust_twins_reproduce_every_bit_of_the_typescript_twins() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let source = directory.join("motion-bits.json");
    let cases: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(&source).unwrap_or_else(|error| panic!("{}: {error} — run dump_motion_bits.ts first", source.display()))).unwrap_or_else(|error| panic!("{}: {error}", source.display()));
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
    for mismatch in &mismatches {
        let _ = writeln!(report, "{mismatch}");
    }
    std::fs::write(directory.join("motion-bits-report.txt"), &report).unwrap_or_else(|error| panic!("{error}"));
    println!("{report}");
    assert!(cases.len() > 5000, "{} cases", cases.len());
    assert!(mismatches.is_empty(), "{} of {} cases differ", mismatches.len(), cases.len());
}

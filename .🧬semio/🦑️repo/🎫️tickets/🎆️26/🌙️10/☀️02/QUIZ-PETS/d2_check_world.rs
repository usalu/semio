//! 🌍️ Ticket tool of work package D2, Rust half of the world check: replays every run of `🗑️generated/d2/world.json` (the
//! reference world of the clearance unit suite, run by `d2_dump_bits.ts` through the TypeScript suite) with the Rust
//! twin of that world, and compares the digest of the mode, the feet and the box of every pet after every tick, and the
//! final tally and counts.
//!
//! Mounted by `d2_scratch.ts prepare` as the unit test module `world_check` of the scratch library, so it reaches the
//! world of the clearance unit suite (`crate::clearance::tests`). The result is written to
//! `🗑️generated/d2/world-report.txt`.
//!
//! Run from the repository root, after `bun <ticket>/d2_dump_bits.ts`:
//!   bash <ticket>/rust_scratch.sh d2 test --release --offline --lib world_check -- --nocapture
//!
//! @see ./d2_dump_bits.ts — the TypeScript half
//! @see ./📓️report2-d2.md — the recorded result

use crate::clearance::tests::{bodies_of, open_world, tick_world, Rules, RULES};
use serde_json::Value;
use std::fmt::Write as _;
use std::path::PathBuf;

/// 🧾️ A running FNV-1a digest on 32-bit words, the TypeScript half's `Digest`.
struct Digest(u32);

impl Digest {
    /// ➕️ Folds the two words of a number.
    fn number(&mut self, number: f64) {
        let bits = number.to_bits();
        for word in [(bits >> 32) as u32, bits as u32] {
            self.0 = (self.0 ^ word).wrapping_mul(16_777_619);
        }
    }
}

/// ✂️ The rules with the named ones switched off.
fn without(names: &[Value]) -> Rules {
    let mut rules = RULES;
    for name in names.iter().filter_map(Value::as_str) {
        match name {
            "guard" => rules.guard = false,
            "corridors" => rules.corridors = false,
            "vetting" => rules.vetting = false,
            "rests" => rules.rests = false,
            "projection" => rules.projection = false,
            "poof" => rules.poof = false,
            "eviction" => rules.eviction = false,
            "heads" => rules.heads = false,
            "steering" => rules.steering = false,
            "seating" => rules.seating = false,
            other => panic!("no rule {other}"),
        }
    }
    rules
}

#[test]
fn the_rust_world_replays_every_typescript_run_tick_by_tick() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let path = directory.join("world.json");
    let runs: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error} — run d2_dump_bits.ts first", path.display()))).unwrap_or_else(|error| panic!("{error}"));
    let runs = runs.as_array().cloned().unwrap_or_default();
    let mut report = String::new();
    let (mut ticks, mut mismatches) = (0, 0);
    for run in &runs {
        let seed = run["seed"].as_u64().unwrap_or(0) as u32;
        let span = run["span"].as_i64().unwrap_or(1);
        let digests: Vec<&str> = run["digests"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
        let mut world = open_world(seed, without(run["off"].as_array().map_or(&[][..], Vec::as_slice)), span);
        let mut first = None;
        for (tick, expected) in digests.iter().enumerate() {
            tick_world(&mut world);
            let mut digest = Digest(2_166_136_261);
            for pet in &world.pets {
                digest.number(pet.mode as u8 as f64);
                digest.number(pet.x);
                digest.number(pet.y);
            }
            for body in bodies_of(&world) {
                for number in [body.owner[4..].parse::<f64>().unwrap_or(f64::NAN), body.extent.x0, body.extent.y0, body.extent.x1, body.extent.y1] {
                    digest.number(number);
                }
            }
            if first.is_none() && format!("{:08x}", digest.0) != *expected {
                first = Some(tick + 1);
            }
        }
        ticks += digests.len();
        let tally = [world.tally.ticks, world.tally.actors, world.tally.overlaps, world.tally.nears, world.tally.poofs, world.tally.waits].map(|count| count as f64);
        let counts = world.counts.listed().map(|count| count as f64);
        let same_tally = run["tally"].as_array().is_some_and(|expected| expected.iter().map(Value::as_f64).eq(tally.iter().map(|count| Some(*count))));
        let same_counts = run["counts"].as_array().is_some_and(|expected| expected.iter().map(Value::as_f64).eq(counts.iter().map(|count| Some(*count))));
        if first.is_some() || !same_tally || !same_counts {
            mismatches += 1;
            let _ = writeln!(
                report,
                "{} seed {seed} span {span}: first differing tick {first:?}, tally {} ({tally:?} against {}), counts {}",
                run["name"],
                if same_tally { "same" } else { "differs" },
                run["tally"],
                if same_counts { "same" } else { "differ" }
            );
        }
    }
    let line = format!("world: {} runs, {ticks} ticks, {mismatches} mismatches\n", runs.len());
    std::fs::write(directory.join("world-report.txt"), format!("{line}{report}")).unwrap_or_else(|error| panic!("{error}"));
    println!("{line}{report}");
    assert!(ticks > 100_000, "{ticks} ticks");
    assert_eq!(mismatches, 0);
}

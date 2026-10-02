//! 🧱️ Ticket tool of work package K: holds the Rust kinematics (`trigonometry`, `randomness`, `rig`) to the bit patterns `dump_kinematics_bits.ts` wrote from the TypeScript twins.
//!
//! Mounted by the scratch crate only (`#[cfg(test)] #[path] mod proof;`), never by the product crate. It reads
//! `🗑️generated/wp-k/kinematics-bits.json` (three levels above the scratch manifest), feeds every input to the Rust
//! function of the same name and compares all 64 bits of every output. From the repository root:
//!
//!   bun <ticket>/dump_kinematics_bits.ts
//!   bash <ticket>/rust_scratch.sh wp-k test --offline proof -- --nocapture

use crate::randomness::{random_between, random_pick, random_unit, random_words};
use crate::rig::{compose, invert, look_offset, solve_rig, transform, Affine, BonePose};
use crate::schema::tests::{entries, fixture, typed};
use crate::schema::{Point, Species};
use crate::trigonometry::{clamp, cos_turns, lerp, sin_turns, smoothstep};
use serde_json::Value;
use std::path::PathBuf;

fn double(value: &Value) -> f64 {
    let digits = value.as_str().unwrap_or_else(|| panic!("not a bit pattern: {value}"));
    f64::from_bits(u64::from_str_radix(digits, 16).unwrap_or_else(|error| panic!("{digits}: {error}")))
}

fn doubles(value: &Value) -> Vec<f64> {
    entries(value).iter().map(double).collect()
}

fn key(value: &Value) -> Vec<u32> {
    entries(value).iter().map(|word| word.as_u64().and_then(|word| u32::try_from(word).ok()).unwrap_or_else(|| panic!("not a word: {word}"))).collect()
}

fn patterns(values: &[f64]) -> Value {
    Value::Array(values.iter().map(|value| Value::String(format!("{:016x}", value.to_bits()))).collect())
}

fn affine(values: &[f64]) -> Affine {
    [values[0], values[1], values[2], values[3], values[4], values[5]]
}

#[test]
fn the_rust_kinematics_reproduce_every_bit_pattern_of_the_typescript_twins() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../kinematics-bits.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error} — run dump_kinematics_bits.ts first", path.display()));
    let document: Value = serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let species: Vec<Species> = entries(&fixture("rig-solving")["species"]).iter().map(typed).collect();
    let answer = |function: &str, vector: &Value| -> Value {
        let input = doubles(&vector["in"]);
        match function {
            "sinTurns" => patterns(&[sin_turns(input[0])]),
            "cosTurns" => patterns(&[cos_turns(input[0])]),
            "clamp" => patterns(&[clamp(input[0], input[1], input[2])]),
            "lerp" => patterns(&[lerp(input[0], input[1], input[2])]),
            "smoothstep" => patterns(&[smoothstep(input[0])]),
            "randomWords" => Value::from(random_words(&key(&vector["key"]), vector["count"].as_u64().unwrap_or_default() as usize)),
            "randomUnit" => patterns(&[random_unit(&key(&vector["key"]))]),
            "randomBetween" => patterns(&[random_between(&key(&vector["key"]), input[0], input[1])]),
            "randomPick" => Value::from(random_pick(&key(&vector["key"]), &input).map_or(-1, |index| index as i64)),
            "compose" => patterns(&compose(affine(&input[..6]), affine(&input[6..]))),
            "invert" => patterns(&invert(affine(&input))),
            "transform" => {
                let carried = transform(affine(&input[..6]), input[6], input[7]);
                patterns(&[carried.x, carried.y])
            }
            "lookOffset" => {
                let offset = look_offset(Point { x: input[0], y: input[1] }, Point { x: input[2], y: input[3] }, input[4]);
                patterns(&[offset.x, offset.y])
            }
            "solveRig" => {
                let kind = species.iter().find(|kind| vector["species"] == kind.id.as_str()).unwrap_or_else(|| panic!("unknown species {}", vector["species"]));
                let pose: Vec<BonePose> = input.chunks(5).map(|entry| BonePose { x: entry[0], y: entry[1], rotation: entry[2], scale_x: entry[3], scale_y: entry[4] }).collect();
                patterns(&solve_rig(kind, &pose))
            }
            other => panic!("no Rust twin is compared for {other}"),
        }
    };
    let (mut compared, mut mismatches, mut functions) = (0usize, 0usize, 0usize);
    for (function, vectors) in document.as_object().into_iter().flatten().filter(|(name, _)| name.as_str() != "$comment") {
        let (mut outputs, mut differing) = (0usize, 0usize);
        for (index, vector) in entries(vectors).iter().enumerate() {
            let (produced, expected) = (answer(function, vector), &vector["out"]);
            match (produced.as_array(), expected.as_array()) {
                (Some(produced), Some(expected)) => {
                    assert_eq!(produced.len(), expected.len(), "{function}/{index}: length");
                    outputs += expected.len();
                    differing += produced.iter().zip(expected).filter(|(left, right)| left != right).count();
                }
                _ => {
                    outputs += 1;
                    differing += usize::from(&produced != expected);
                }
            }
        }
        println!("[proof] {function}: inputs={} outputs={outputs} mismatches={differing}", entries(vectors).len());
        compared += outputs;
        mismatches += differing;
        functions += 1;
    }
    println!("[proof] functions={functions} compared={compared} mismatches={mismatches}");
    assert_eq!(functions, 14);
    assert!(compared > 10_000, "{compared}");
    assert_eq!(mismatches, 0);
}

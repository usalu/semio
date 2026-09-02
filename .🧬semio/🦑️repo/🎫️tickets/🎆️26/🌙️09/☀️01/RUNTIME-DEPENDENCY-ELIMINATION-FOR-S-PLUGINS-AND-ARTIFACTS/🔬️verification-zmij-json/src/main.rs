mod exact;

use std::fmt;

// FINAL CANDIDATE write_float: shortest-round-trip digit length + leading-digit exponent from
// Rust's `{:e}` (trusted for LENGTH and MAGNITUDE — proven minimal, round-trip-correct by every
// case in this sweep), zmij's actual fixed/exponential threshold for f64 (`-5..=15` on that
// exponent), and the DIGITS themselves recomputed by exact round-half-to-even arithmetic
// (`exact::correctly_rounded_digits`) rather than trusted from Rust directly — Rust's own
// tie-break at the last digit does not match IEEE 754 round-half-to-even in general, and a
// "does the neighboring digit also round-trip" heuristic was tried and shown UNSOUND for
// large-magnitude values (the round-trip basin can hold more than two adjacent minimal-length
// candidates there), so exact arithmetic replaces it entirely.
fn write_float_candidate(value: f64, out: &mut String) {
    if !value.is_finite() {
        out.push_str("null");
        return;
    }
    if value == 0.0 {
        out.push_str(if value.is_sign_negative() { "-0.0" } else { "0.0" });
        return;
    }
    let negative = value.is_sign_negative();
    let magnitude = value.abs();
    let scientific = format!("{magnitude:e}");
    let (mantissa_text, exponent_text) = scientific.split_once('e').expect("LowerExp always emits an exponent");
    let mut exponent: i32 = exponent_text.parse().expect("LowerExp exponent is always a plain integer");
    let digit_count = mantissa_text.bytes().filter(|b| *b != b'.').count();

    let (mut digits, exponent_adjust) = exact::correctly_rounded_digits(magnitude, exponent, digit_count);
    exponent += exponent_adjust;
    if exponent_adjust != 0 {
        digits.truncate(digit_count);
    }

    let digit_count = digits.len() as i32;
    if negative {
        out.push('-');
    }
    if (-5..=15).contains(&exponent) {
        if exponent >= digit_count - 1 {
            out.push_str(std::str::from_utf8(&digits).unwrap());
            for _ in 0..(exponent - (digit_count - 1)) {
                out.push('0');
            }
            out.push_str(".0");
        } else if exponent >= 0 {
            let integer_len = (exponent + 1) as usize;
            out.push_str(std::str::from_utf8(&digits[..integer_len]).unwrap());
            out.push('.');
            out.push_str(std::str::from_utf8(&digits[integer_len..]).unwrap());
        } else {
            out.push_str("0.");
            for _ in 0..(-exponent - 1) {
                out.push('0');
            }
            out.push_str(std::str::from_utf8(&digits).unwrap());
        }
    } else {
        out.push(digits[0] as char);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(std::str::from_utf8(&digits[1..]).unwrap());
        }
        out.push('e');
        if exponent >= 0 {
            out.push('+');
        }
        let _ = fmt::Write::write_fmt(out, format_args!("{exponent}"));
    }
}

fn write_float_current(value: f64, out: &mut String) {
    if !value.is_finite() {
        out.push_str("null");
        return;
    }
    if value == 0.0 {
        out.push_str(if value.is_sign_negative() { "-0.0" } else { "0.0" });
        return;
    }
    let negative = value.is_sign_negative();
    let magnitude = value.abs();
    let scientific = format!("{magnitude:e}");
    let (mantissa, exponent_text) = scientific.split_once('e').expect("LowerExp always emits an exponent");
    let exponent: i32 = exponent_text.parse().expect("LowerExp exponent is always a plain integer");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let digit_count = digits.len() as i32;
    if negative {
        out.push('-');
    }
    if (-6..21).contains(&exponent) {
        if exponent >= digit_count - 1 {
            out.push_str(&digits);
            for _ in 0..(exponent - (digit_count - 1)) {
                out.push('0');
            }
            out.push_str(".0");
        } else if exponent >= 0 {
            let integer_len = (exponent + 1) as usize;
            out.push_str(&digits[..integer_len]);
            out.push('.');
            out.push_str(&digits[integer_len..]);
        } else {
            out.push_str("0.");
            for _ in 0..(-exponent - 1) {
                out.push('0');
            }
            out.push_str(&digits);
        }
    } else {
        out.push(digits.as_bytes()[0] as char);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        if exponent >= 0 {
            out.push('+');
        }
        let _ = fmt::Write::write_fmt(out, format_args!("{exponent}"));
    }
}

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn edge_cases() -> Vec<f64> {
    vec![
        0.0, -0.0, 1.0, -1.0, f64::MIN_POSITIVE, -f64::MIN_POSITIVE, f64::MAX, f64::MIN,
        1e-7, -1e-7, 1e21, 1e22, 5e-324, -5e-324,
        1.7976931348623157e308, -1.7976931348623157e308,
        0.1, 0.3, 1e16, 1e15, 9999999999999998.0, 9007199254740993.0,
        123456789012345.0, 1234567890123456.0, 12345678901234567.0,
        100000.0, 1000000.0, 99999.0, 999999999999999.9,
        1.0e-5, 1.0e-6, 1.0e-4,
        f64::from_bits(1), f64::from_bits(0x0010000000000000), f64::from_bits(0x7FEFFFFFFFFFFFFF),
        8322951083873004.0,
        f64::from_bits(0xc316b3096f9dcd35),
        f64::from_bits(0x431b807272ea6281),
        f64::from_bits(0xc9409f0951d8de1a),
        f64::from_bits(0x40f869f000000000),
        f64::from_bits(0x430c6bf52633ffff),
        9999999999999999.0,
        99999999999999999999.0,
    ]
}

fn main() {
    let mut mismatches: Vec<(f64, String, String)> = Vec::new();
    let mut current_mismatches: usize = 0;
    let mut total: usize = 0;
    let mut roundtrip_failures: Vec<(f64, String)> = Vec::new();

    let mut check = |value: f64, total: &mut usize, mismatches: &mut Vec<(f64, String, String)>, current_mismatches: &mut usize, roundtrip_failures: &mut Vec<(f64, String)>| {
        *total += 1;
        let mut mine_current = String::new();
        write_float_current(value, &mut mine_current);
        let mut mine_candidate = String::new();
        write_float_candidate(value, &mut mine_candidate);
        let theirs = serde_json::to_string(&value).unwrap();
        if mine_current != theirs {
            *current_mismatches += 1;
        }
        if mine_candidate != theirs {
            mismatches.push((value, mine_candidate.clone(), theirs.clone()));
        }
        let parsed: f64 = mine_candidate.parse().unwrap_or_else(|e| panic!("candidate output {mine_candidate:?} for {value:e} failed to parse: {e}"));
        if parsed.to_bits() != value.to_bits() {
            roundtrip_failures.push((value, mine_candidate.clone()));
        }
    };

    for &v in &edge_cases() {
        check(v, &mut total, &mut mismatches, &mut current_mismatches, &mut roundtrip_failures);
    }

    let mut rng = Rng::new(0xC0FF_EE00_1234_5678);
    let corpus_size = 10_000_000usize;
    for _ in 0..corpus_size {
        let bits = rng.next_u64();
        let value = f64::from_bits(bits);
        if !value.is_finite() {
            continue;
        }
        check(value, &mut total, &mut mismatches, &mut current_mismatches, &mut roundtrip_failures);
    }

    let mut rng2 = Rng::new(0x1357_9BDF_2468_ACE0);
    for _ in 0..corpus_size {
        let bits = rng2.next_u64();
        let value = f64::from_bits(bits);
        if !value.is_finite() {
            continue;
        }
        check(value, &mut total, &mut mismatches, &mut current_mismatches, &mut roundtrip_failures);
    }

    // Biased corpus toward small integers / typical magnitudes, where content-addressed hash
    // inputs are most likely to concentrate (geometry coordinates, counts, ids).
    let mut rng3 = Rng::new(0xFEED_FACE_C0DE_BABE);
    for _ in 0..corpus_size {
        let exponent = (rng3.next_u64() % 40) as i32 - 20;
        let mantissa_bits = rng3.next_u64() & ((1u64 << 52) - 1);
        let sign = rng3.next_u64() & 1;
        let value = (1.0 + (mantissa_bits as f64) / (1u64 << 52) as f64) * 10f64.powi(exponent) * if sign == 1 { -1.0 } else { 1.0 };
        if !value.is_finite() {
            continue;
        }
        check(value, &mut total, &mut mismatches, &mut current_mismatches, &mut roundtrip_failures);
    }

    // Subnormal-heavy corpus: low bit patterns exercise the `raw_exp == 0` decomposition path.
    let mut rng4 = Rng::new(0xABCD_1234_9876_5432);
    for _ in 0..corpus_size {
        let bits = rng4.next_u64() & 0x800F_FFFF_FFFF_FFFF; // sign + subnormal mantissa only
        let value = f64::from_bits(bits);
        if !value.is_finite() {
            continue;
        }
        check(value, &mut total, &mut mismatches, &mut current_mismatches, &mut roundtrip_failures);
    }

    println!("total checked: {total}");
    println!("CURRENT (ECMA-262 -6..21, no tie-fix) writer mismatches vs serde_json: {current_mismatches}");
    println!("CANDIDATE (-5..=15, exact round-half-even digits) writer mismatches vs serde_json: {}", mismatches.len());
    println!("round-trip failures (candidate output -> parse -> same bits): {}", roundtrip_failures.len());

    for (v, mine, theirs) in mismatches.iter().take(40) {
        println!("MISMATCH bits={:#018x} value={:e} mine={mine} theirs={theirs}", v.to_bits(), v);
    }
    for (v, mine) in roundtrip_failures.iter().take(10) {
        println!("ROUNDTRIP-FAIL bits={:#018x} value={:e} mine={mine}", v.to_bits(), v);
    }

    if mismatches.is_empty() && roundtrip_failures.is_empty() {
        println!("PARITY: PROVEN across {total} cases (candidate writer)");
    } else {
        println!("PARITY: FAILED — see mismatches above");
    }
}

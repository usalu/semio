//! 🗜️ Lempel-Ziv complexity family: the Kaspar & Schuster (1987) incremental-parsing LZ76
//! complexity measure over arbitrary discrete symbol streams, a from-scratch LZ78 dictionary
//! [`Compressor`], and the compressor-agnostic normalized compression distance ([`ncd`]) built on
//! top of it. Zero external dependencies — no `flate2`, no `zstd`, nothing beyond `std`.

use std::collections::HashMap;

use crate::{EntropyError, Estimate, LogBase, Warning};

// #region 🔖Lz76
/// 🗜️ Kaspar & Schuster (1987) incremental-parsing complexity `c(n)`, generalized from binary
/// strings to an arbitrary `u32`-symbol alphabet. Counts the number of distinct "new" phrases a
/// greedy left-to-right parse needs to reproduce `s`, where a phrase may self-referentially copy
/// from *inside* the phrase currently being grown (not just from history already committed).
///
/// Ported verbatim (0-indexed) from the classic incremental-parsing pseudocode; the loop body is
/// fiddly and was **not** simplified. The two indices `s[i + k - 1]` / `s[l + k - 1]` are read
/// through [`slice::get`] rather than direct indexing: an out-of-range read is treated as "not
/// equal" so the mismatch branch fires instead of panicking, which is the only change from the
/// textbook pseudocode (needed because the naive index-safety invariant does not actually hold
/// for short inputs — confirmed by exhaustive brute-force cross-validation, see
/// `tests::exhaustive`).
fn lz76_complexity(s: &[u32]) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let (mut i, mut k, mut l) = (0usize, 1usize, 1usize);
    let mut c = 1usize;
    let mut k_max = 1usize;
    loop {
        let equal = match (s.get(i + k - 1), s.get(l + k - 1)) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        };
        if equal {
            k += 1;
            if l + k > n {
                c += 1;
                break;
            }
        } else {
            if k > k_max {
                k_max = k;
            }
            i += 1;
            if i == l {
                c += 1;
                l += k_max;
                if l + 1 > n {
                    break;
                }
                i = 0;
                k = 1;
                k_max = 1;
            } else {
                k = 1;
            }
        }
    }
    c
}

/// 🗜️ Lempel-Ziv complexity of a discrete symbol sequence. Raw mode reports the incremental-parse
/// phrase count `c(n)` directly; `normalized` mode reports `c(n) * log_alpha(n) / n` (`alpha` =
/// occupied alphabet size), which converges to `1` for a maximally complex sequence over that
/// alphabet as `n -> infinity`.
///
/// `base` on the returned [`Estimate`] is always [`LogBase::Nats`] as an inert placeholder — LZ
/// complexity is a phrase count / normalized ratio, not a log-base-dependent entropy, so
/// [`Estimate::in_base`] is meaningless here and should not be called on this result.
pub fn lempel_ziv_complexity(symbols: &[u32], normalized: bool) -> Result<Estimate, EntropyError> {
    if symbols.is_empty() {
        return Err(EntropyError::EmptyInput { what: "symbols" });
    }
    let n = symbols.len();
    let c = lz76_complexity(symbols);

    let mut alphabet: Vec<u32> = symbols.to_vec();
    alphabet.sort_unstable();
    alphabet.dedup();
    let alpha = alphabet.len();

    let value = if normalized {
        if alpha <= 1 {
            0.0
        } else {
            let log_alpha_n = (n as f64).ln() / (alpha as f64).ln();
            c as f64 * log_alpha_n / n as f64
        }
    } else {
        c as f64
    };

    let mut warnings = Vec::new();
    if n < 100 {
        warnings.push(Warning::SmallSample { n, recommended: 100 });
    }

    Ok(Estimate {
        value,
        base: LogBase::Nats,
        method: "lempel_ziv",
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None,
        warnings,
        diagnostics: vec![("alphabet_size", alpha as f64), ("raw_complexity", c as f64)],
    })
}
// #endregion 🔖Lz76

// #region 🔖Compressor
/// 🗜️ A byte-stream compressor exposing only the one number [`ncd`] needs: how many bytes `data`
/// would take to represent. Kept behind a trait so [`ncd`] never depends on a specific codec's
/// implementation details.
pub trait Compressor {
    fn compressed_len(&self, data: &[u8]) -> usize;
}

/// 🗜️ Textbook LZ78 dictionary compressor (Ziv & Lempel, 1978), implemented from scratch as a
/// proxy [`Compressor`] for [`ncd`]. Not a byte-for-byte codec (there is no matching decoder) —
/// only [`Compressor::compressed_len`]'s *size estimate* is produced.
pub struct Lz78Compressor;

impl Compressor for Lz78Compressor {
    /// 🗜️ Greedily extends the current phrase while it remains a dictionary hit; each time a new
    /// (dictionary-index, literal-byte) pair is emitted, the phrase is added to the dictionary and
    /// the running bit cost grows by `ceil(log2(dictionary_size + 2))` (index) `+ 8` (literal).
    /// The dictionary is capped at 65535 entries (`u16` index space, index `0` reserved for the
    /// implicit empty root phrase); once full, matching against existing entries continues but no
    /// new phrases are memorized (standard LZW/LZ78 dictionary-full behavior). Returns the total
    /// emitted bit cost rounded up to whole bytes.
    fn compressed_len(&self, data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }
        const MAX_ENTRIES: usize = 65_535;
        let mut dict: HashMap<Vec<u8>, u16> = HashMap::new();
        let mut phrase: Vec<u8> = Vec::new();
        let mut total_bits: f64 = 0.0;

        let index_bits = |dict_len: usize| -> f64 {
            let addressable = dict_len as f64 + 1.0; // 🗜️ +1 for the implicit empty-phrase index 0
            (addressable + 1.0).log2().ceil().max(1.0)
        };

        for &byte in data {
            phrase.push(byte);
            if !dict.contains_key(&phrase) {
                total_bits += index_bits(dict.len()) + 8.0;
                if dict.len() < MAX_ENTRIES {
                    dict.insert(phrase.clone(), dict.len() as u16 + 1);
                }
                phrase.clear();
            }
        }
        if !phrase.is_empty() {
            // 🗜️ Trailing phrase matched an existing entry but the input ended before a new
            // extension was discovered; emit one final index-only reference (no literal).
            total_bits += index_bits(dict.len());
        }
        ((total_bits / 8.0).ceil() as usize).max(1)
    }
}
// #endregion 🔖Compressor

// #region 🔖Ncd
/// 🗜️ Normalized compression distance (Cilibrasi & Vitanyi, 2005):
/// `(C(xy) - min(C(x), C(y))) / max(C(x), C(y))`, a compressor-driven approximation to
/// normalized information distance in `[0, ~1]`. `compressor` decides what "compressed" means;
/// [`Lz78Compressor`] is a reasonable zero-dependency default.
pub fn ncd(x: &[u8], y: &[u8], compressor: &dyn Compressor) -> Result<f64, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if y.is_empty() {
        return Err(EntropyError::EmptyInput { what: "y" });
    }
    let cx = compressor.compressed_len(x) as f64;
    let cy = compressor.compressed_len(y) as f64;
    let mut xy = Vec::with_capacity(x.len() + y.len());
    xy.extend_from_slice(x);
    xy.extend_from_slice(y);
    let cxy = compressor.compressed_len(&xy) as f64;

    let denom = cx.max(cy);
    if denom == 0.0 {
        return Ok(0.0);
    }
    Ok((cxy - cx.min(cy)) / denom)
}
// #endregion 🔖Ncd

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn binary_string(s: &str) -> Vec<u32> {
        s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect()
    }

    #[test]
    fn lz76_matches_canonical_test_string() {
        // 🔐 See `exhaustive::lz76_canonical_value_matches_verified_reference` for the full
        // cross-validation story behind this specific number.
        let s = binary_string("0001101001000101");
        assert_eq!(lz76_complexity(&s), 6);
    }

    #[test]
    fn lz76_constant_sequence_is_minimally_complex() {
        let s = vec![0u32; 200];
        assert!(lz76_complexity(&s) <= 3);
    }

    #[test]
    fn lz76_repetitive_much_lower_than_random_of_same_length() {
        let mut rng = crate::numeric::Xorshift64::new(7);
        let n = 500;
        let repetitive: Vec<u32> = (0..n).map(|i| (i % 3) as u32).collect();
        let random: Vec<u32> = (0..n).map(|_| rng.next_below(8) as u32).collect();
        let c_rep = lz76_complexity(&repetitive);
        let c_rand = lz76_complexity(&random);
        assert!(c_rep * 3 < c_rand, "c_rep={c_rep} c_rand={c_rand}");
    }

    #[test]
    fn lempel_ziv_complexity_rejects_empty() {
        assert!(matches!(
            lempel_ziv_complexity(&[], false),
            Err(EntropyError::EmptyInput { .. })
        ));
    }

    #[test]
    fn lempel_ziv_complexity_raw_matches_lz76() {
        let s = binary_string("0001101001000101");
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert_eq!(est.value, 6.0);
        assert_eq!(est.n, s.len());
        assert_eq!(est.diagnostics[1], ("raw_complexity", 6.0));
    }

    #[test]
    fn lempel_ziv_complexity_normalized_is_zero_for_single_symbol_alphabet() {
        let s = vec![0u32; 50];
        let est = lempel_ziv_complexity(&s, true).unwrap();
        assert_eq!(est.value, 0.0);
        assert_eq!(est.diagnostics[0], ("alphabet_size", 1.0));
    }

    #[test]
    fn lempel_ziv_complexity_small_sample_warns() {
        let s = binary_string("0001101001000101");
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })));
    }

    #[test]
    fn lempel_ziv_complexity_large_sample_does_not_warn() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let s: Vec<u32> = (0..200).map(|_| rng.next_below(4) as u32).collect();
        let est = lempel_ziv_complexity(&s, false).unwrap();
        assert!(est.warnings.is_empty());
    }

    #[test]
    fn lz78_empty_input_compresses_to_zero() {
        assert_eq!(Lz78Compressor.compressed_len(&[]), 0);
    }

    #[test]
    fn lz78_repetitive_input_compresses_shorter_than_random() {
        let repetitive = b"abababababababab".to_vec();
        let mut rng = crate::numeric::Xorshift64::new(42);
        let random: Vec<u8> = (0..repetitive.len()).map(|_| rng.next_below(256) as u8).collect();
        let comp = Lz78Compressor;
        assert!(comp.compressed_len(&repetitive) <= comp.compressed_len(&random));
    }

    #[test]
    fn ncd_rejects_empty_inputs() {
        let comp = Lz78Compressor;
        assert!(matches!(ncd(&[], b"x", &comp), Err(EntropyError::EmptyInput { .. })));
        assert!(matches!(ncd(b"x", &[], &comp), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn ncd_of_a_string_with_itself_is_well_below_unrelated_random_strings() {
        let comp = Lz78Compressor;
        let text: Vec<u8> = b"the quick brown fox jumps over the lazy dog ".repeat(8);
        let d_self = ncd(&text, &text, &comp).unwrap();

        let mut rng_a = crate::numeric::Xorshift64::new(1234);
        let a: Vec<u8> = (0..text.len()).map(|_| rng_a.next_below(256) as u8).collect();
        let mut rng_b = crate::numeric::Xorshift64::new(999_999);
        let b: Vec<u8> = (0..text.len()).map(|_| rng_b.next_below(256) as u8).collect();
        let d_diff = ncd(&a, &b, &comp).unwrap();

        assert!(d_self < d_diff, "d_self={d_self} d_diff={d_diff}");
        assert!(d_diff > 0.5, "expected two unrelated random byte strings to be far apart: {d_diff}");
    }

    #[test]
    fn ncd_is_bounded_below_by_zero() {
        let comp = Lz78Compressor;
        let x = b"identical payload identical payload".to_vec();
        let d = ncd(&x, &x, &comp).unwrap();
        assert!(d >= 0.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn lz76_is_non_decreasing_in_sequence_length_for_a_growing_random_stream() {
            let mut rng = crate::numeric::Xorshift64::new(11);
            let full: Vec<u32> = (0..300).map(|_| rng.next_below(5) as u32).collect();
            let mut prev = lz76_complexity(&full[..1]);
            for len in [10, 50, 100, 200, 300] {
                let c = lz76_complexity(&full[..len]);
                assert!(c >= prev, "len={len} c={c} prev={prev}");
                prev = c;
            }
        }

        #[test]
        fn lz78_concatenation_never_shrinks_relative_to_either_half() {
            let comp = Lz78Compressor;
            let mut rng = crate::numeric::Xorshift64::new(55);
            for _ in 0..20 {
                let n = 20 + rng.next_below(80);
                let x: Vec<u8> = (0..n).map(|_| rng.next_below(256) as u8).collect();
                let m = 20 + rng.next_below(80);
                let y: Vec<u8> = (0..m).map(|_| rng.next_below(256) as u8).collect();
                let mut xy = x.clone();
                xy.extend_from_slice(&y);
                let cx = comp.compressed_len(&x);
                let cy = comp.compressed_len(&y);
                let cxy = comp.compressed_len(&xy);
                assert!(cxy >= cx.max(cy), "cxy={cxy} cx={cx} cy={cy}");
            }
        }
    }

    // #region 🔖Exhaustive
    /// 🔐 Brute-force validation of [`lz76_complexity`] against an independent definitional
    /// oracle, over every binary string of length `1..=12` (`4094` strings). This measure is
    /// well known to be easy to get off-by-one wrong, so the incremental (fast) implementation is
    /// checked against a slow, obviously-correct-by-construction parser rather than trusted on
    /// its own.
    mod exhaustive {
        use super::*;

        /// 🔐 Naive contiguous-substring test, `O(n*m)`, used only by the brute-force oracle
        /// below (never on a hot path).
        fn contains(haystack: &[u32], needle: &[u32]) -> bool {
            if needle.is_empty() {
                return true;
            }
            if needle.len() > haystack.len() {
                return false;
            }
            (0..=haystack.len() - needle.len()).any(|start| &haystack[start..start + needle.len()] == needle)
        }

        /// 🔐 Definitional LZ76 phrase count: repeatedly takes the shortest prefix-extension of
        /// the unparsed remainder that is not already a substring of (parsed history + candidate
        /// minus its own last symbol), i.e. the shortest prefix of the remainder not found
        /// anywhere in the string up to (and including) the position just before the prefix's
        /// last symbol. Deliberately independent of [`lz76_complexity`]'s control flow.
        fn brute_force_lz76(s: &[u32]) -> usize {
            let n = s.len();
            if n == 0 {
                return 0;
            }
            let mut phrases = 0usize;
            let mut pos = 0usize;
            while pos < n {
                let mut len = 1usize;
                loop {
                    if pos + len > n {
                        len = n - pos;
                        break;
                    }
                    let candidate = &s[pos..pos + len];
                    let haystack = &s[0..pos + len - 1];
                    if !contains(haystack, candidate) {
                        break;
                    }
                    len += 1;
                }
                phrases += 1;
                pos += len;
            }
            phrases
        }

        #[test]
        fn lz76_matches_definitional_brute_force_for_every_short_binary_string() {
            let mut checked = 0usize;
            for len in 1..=12usize {
                for v in 0..(1u32 << len) {
                    let s: Vec<u32> = (0..len as u32).map(|b| (v >> b) & 1).collect();
                    let incremental = lz76_complexity(&s);
                    let brute = brute_force_lz76(&s);
                    assert_eq!(incremental, brute, "len={len} v={v} s={s:?}");
                    checked += 1;
                }
            }
            assert_eq!(checked, 8190); // 2^1 + 2^2 + ... + 2^12
        }

        #[test]
        fn lz76_canonical_value_matches_verified_reference() {
            // 🔐 The literal incremental-parsing pseudocode, cross-checked against
            // `brute_force_lz76` above on all 8190 binary strings of length 1..=12 with zero
            // mismatches, computes `c = 6` for this string (not the `8` sometimes quoted for it
            // in secondary sources — that figure does not reproduce under this definition and
            // was rejected in favor of the exhaustively cross-validated result).
            let s = binary_string("0001101001000101");
            assert_eq!(lz76_complexity(&s), 6);
            assert_eq!(brute_force_lz76(&s), 6);
        }
    }
    // #endregion 🔖Exhaustive
}
// #endregion 🔖Tests

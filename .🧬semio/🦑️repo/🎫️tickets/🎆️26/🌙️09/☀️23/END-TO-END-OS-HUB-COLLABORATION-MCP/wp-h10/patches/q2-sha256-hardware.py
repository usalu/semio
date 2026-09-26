#!/usr/bin/env python3
"""🏎️ H10 post-publish patch set Q2 (ticket 26/09/23 session 12): SHA-256 compressions on the CPU's own instructions.

`semio-framework-hash`'s `Sha256` runs every compression through portable Rust rounds. PBKDF2 at 210 000 iterations
is 420 000 compressions per sign-in (~200 ms of CPU on an idle M1 Max with the crate at opt-level 3, seconds under
fleet load), and catalog loads hash ~240 MB of components. aarch64 (`sha2`: SHA256H/H2/SU0/SU1) and x86-64
(`sha` + `sse4.1`: SHA256RNDS2/MSG1/MSG2) compute the SAME FIPS 180-4 compression in hardware. The patch:
  * keeps the portable rounds as `sha256_compress_portable` (the one algorithm, used by wasm guests and CPUs without
    the extension — not a fallback shim, it is the reference the accelerated paths are held against);
  * adds `sha256_compress_aarch64` / `sha256_compress_x86` behind `#[target_feature]`, selected once per call by
    run-time detection (`is_aarch64_feature_detected!("sha2")`, `is_x86_feature_detected!("sha")`);
  * wasm32 compiles only the portable path, so no guest's behaviour or digest changes;
  * laws: every compression path agrees with the portable one on random blocks and NIST vectors, and a third-party
    oracle (`sha2`, dev-dependency only) agrees on random inputs of every length class; the hub's already-landed
    benchmark law `a_default_cost_credential_derivation_fits_the_session_mint_budget` keeps holding, faster.
No digest value changes anywhere (codec hashes, pack-schema hashes, credentials, catalog digests are all identical).

usage (every mode takes --root <repository copy> to patch a copy instead of the tree): q2-sha256-hardware.py            dry run: every hunk found exactly once
       q2-sha256-hardware.py --apply    write the tree (post-publish window only)
       q2-sha256-hardware.py --scratch  build a patched COPY under wp-h10/target/q2-scratch and run its laws + bench
"""
import pathlib, subprocess, sys, os

ROOT = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else pathlib.Path("/Users/ueli/Documents/semio")
HASH = ROOT / "🧰️framework/🔨️modules/🔏️hash/🦀️.rs"
TESTS = ROOT / "🧰️framework/🔨️modules/🔏️hash/🧪️tests/🔬️unit/🦀️.rs"
MANIFEST = ROOT / "🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust/Cargo.toml"


OLD_TRANSFORM_HEAD = """    fn transform(&mut self, block: &[u8; 64]) {
        let mut words = [0u32; 64];"""
NEW_TRANSFORM_HEAD = """    fn transform(&mut self, block: &[u8; 64]) {
        sha256_compress(&mut self.state, block);
    }
}

/// 🏎️ One SHA-256 compression on the CPU's own SHA-256 instructions when it has them (aarch64 `sha2`,
/// x86-64 `sha` with `sse4.1`, detected at run time), otherwise [`sha256_compress_portable`] — the
/// same FIPS 180-4 compression either way, so every digest is identical on every machine and target.
#[inline]
fn sha256_compress(state: &mut [u32; 8], block: &[u8; 64]) {
    #[cfg(target_arch = "aarch64")]
    if std::arch::is_aarch64_feature_detected!("sha2") {
        return unsafe { sha256_compress_aarch64(state, block) };
    }
    #[cfg(target_arch = "x86_64")]
    if std::arch::is_x86_feature_detected!("sha") && std::arch::is_x86_feature_detected!("sse4.1") {
        return unsafe { sha256_compress_x86(state, block) };
    }
    sha256_compress_portable(state, block);
}

/// 🦾️ The compression on the Armv8 SHA-256 instructions: four rounds per `SHA256H`/`SHA256H2` pair,
/// the message schedule on `SHA256SU0`/`SHA256SU1`.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "sha2")]
unsafe fn sha256_compress_aarch64(state: &mut [u32; 8], block: &[u8; 64]) {
    use std::arch::aarch64::{vaddq_u32, vld1q_u32, vld1q_u8, vreinterpretq_u32_u8, vrev32q_u8, vsha256h2q_u32, vsha256hq_u32, vsha256su0q_u32, vsha256su1q_u32, vst1q_u32};
    unsafe {
        let (mut abcd, mut efgh) = (vld1q_u32(state.as_ptr()), vld1q_u32(state.as_ptr().add(4)));
        let (abcd_start, efgh_start) = (abcd, efgh);
        let load = |offset: usize| vreinterpretq_u32_u8(vrev32q_u8(vld1q_u8(block.as_ptr().add(offset))));
        let mut words = [load(0), load(16), load(32), load(48)];
        for group in 0..16 {
            let scheduled = vaddq_u32(words[0], vld1q_u32(SHA256_ROUNDS.as_ptr().add(group * 4)));
            let next = vsha256su1q_u32(vsha256su0q_u32(words[0], words[1]), words[2], words[3]);
            words = [words[1], words[2], words[3], next];
            let abcd_before = abcd;
            abcd = vsha256hq_u32(abcd, efgh, scheduled);
            efgh = vsha256h2q_u32(efgh, abcd_before, scheduled);
        }
        vst1q_u32(state.as_mut_ptr(), vaddq_u32(abcd, abcd_start));
        vst1q_u32(state.as_mut_ptr().add(4), vaddq_u32(efgh, efgh_start));
    }
}

/// 🧮️ The compression on the x86 SHA extensions: two rounds per `SHA256RNDS2`, the message schedule on
/// `SHA256MSG1`/`SHA256MSG2`, the state held as the `ABEF`/`CDGH` lane pairs those instructions use.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sha,sse2,ssse3,sse4.1")]
unsafe fn sha256_compress_x86(state: &mut [u32; 8], block: &[u8; 64]) {
    use std::arch::x86_64::{__m128i, _mm_add_epi32, _mm_alignr_epi8, _mm_blend_epi16, _mm_loadu_si128, _mm_set_epi64x, _mm_sha256msg1_epu32, _mm_sha256msg2_epu32, _mm_sha256rnds2_epu32, _mm_shuffle_epi32, _mm_shuffle_epi8, _mm_storeu_si128};
    unsafe {
        let byte_swap = _mm_set_epi64x(0x0c0d_0e0f_0809_0a0b, 0x0405_0607_0001_0203);
        let dcba = _mm_loadu_si128(state.as_ptr().cast::<__m128i>());
        let hgfe = _mm_loadu_si128(state.as_ptr().add(4).cast::<__m128i>());
        let cdab = _mm_shuffle_epi32(dcba, 0xb1);
        let efgh = _mm_shuffle_epi32(hgfe, 0x1b);
        let mut abef = _mm_alignr_epi8(cdab, efgh, 8);
        let mut cdgh = _mm_blend_epi16(efgh, cdab, 0xf0);
        let (abef_start, cdgh_start) = (abef, cdgh);
        let load = |offset: usize| _mm_shuffle_epi8(_mm_loadu_si128(block.as_ptr().add(offset).cast::<__m128i>()), byte_swap);
        let mut words = [load(0), load(16), load(32), load(48)];
        for group in 0..16 {
            let message = if group < 4 {
                words[group]
            } else {
                let next = _mm_sha256msg2_epu32(_mm_add_epi32(_mm_sha256msg1_epu32(words[0], words[1]), _mm_alignr_epi8(words[3], words[2], 4)), words[3]);
                words = [words[1], words[2], words[3], next];
                next
            };
            let scheduled = _mm_add_epi32(message, _mm_loadu_si128(SHA256_ROUNDS.as_ptr().add(group * 4).cast::<__m128i>()));
            cdgh = _mm_sha256rnds2_epu32(cdgh, abef, scheduled);
            abef = _mm_sha256rnds2_epu32(abef, cdgh, _mm_shuffle_epi32(scheduled, 0x0e));
        }
        let feba = _mm_shuffle_epi32(_mm_add_epi32(abef, abef_start), 0x1b);
        let dchg = _mm_shuffle_epi32(_mm_add_epi32(cdgh, cdgh_start), 0xb1);
        _mm_storeu_si128(state.as_mut_ptr().cast::<__m128i>(), _mm_blend_epi16(feba, dchg, 0xf0));
        _mm_storeu_si128(state.as_mut_ptr().add(4).cast::<__m128i>(), _mm_alignr_epi8(dchg, feba, 8));
    }
}

/// 📜️ The FIPS 180-4 compression in portable Rust: the reference every accelerated path is held
/// against, and the only one a wasm guest runs.
fn sha256_compress_portable(state: &mut [u32; 8], block: &[u8; 64]) {
"""

OLD_TRANSFORM_TAIL = """        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;"""
NEW_TRANSFORM_TAIL = """        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;"""
OLD_TRANSFORM_END = """        for (state, value) in self.state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *state = state.wrapping_add(value);
        }
    }
}"""
NEW_TRANSFORM_END = """        for (state, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *state = state.wrapping_add(value);
        }
    }
}"""

LAWS = '''
/// 🏎️ Every SHA-256 compression this machine can select agrees with the portable FIPS 180-4 rounds on
/// NIST's blocks and on pseudo-random states and blocks, so hardware selection never changes a digest.
#[test]
fn sha256_hardware_compression_agrees_with_the_portable_rounds() {
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for _ in 0..20_000 {
        let mut state = [0u32; 8];
        state.iter_mut().for_each(|word| *word = next() as u32);
        let mut block = [0u8; 64];
        block.iter_mut().for_each(|byte| *byte = next() as u8);
        let (mut selected, mut portable) = (state, state);
        sha256_compress(&mut selected, &block);
        sha256_compress_portable(&mut portable, &block);
        assert_eq!(selected, portable, "state {state:08x?} block {block:02x?}");
    }
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

/// 🔬️ Third-party oracle: the `sha2` crate agrees with this crate's `Sha256` on every length class
/// around the block and padding boundaries, whole and in uneven segments.
#[test]
fn sha256_agrees_with_the_sha2_oracle_across_lengths() {
    use sha2::Digest as _;
    for length in (0..=260).chain([511, 512, 513, 4_095, 4_096, 65_537, 1_000_003]) {
        let input: Vec<u8> = (0..length).map(|index| (index as u32).wrapping_mul(2_654_435_761).rotate_left(7) as u8).collect();
        let oracle: [u8; 32] = sha2::Sha256::digest(&input).into();
        assert_eq!(Sha256::digest(&input), oracle, "length {length}");
        let mut segmented = Sha256::new();
        for chunk in input.chunks(37) {
            segmented.update(chunk);
        }
        assert_eq!(segmented.finalize(), oracle, "segmented length {length}");
    }
}
'''

def patched_sources():
    hash_text = HASH.read_text(encoding="utf-8")
    for label, old in [("transform head", OLD_TRANSFORM_HEAD), ("transform state", OLD_TRANSFORM_TAIL), ("transform end", OLD_TRANSFORM_END)]:
        if hash_text.count(old) != 1:
            raise SystemExit(f"hunk '{label}' not found exactly once — re-derive it")
    start = hash_text.index(OLD_TRANSFORM_HEAD)
    end = hash_text.index(OLD_TRANSFORM_END, start) + len(OLD_TRANSFORM_END)
    body = hash_text[start + len("    fn transform(&mut self, block: &[u8; 64]) {\n"):end - len("    }\n}")]
    body = body.replace(OLD_TRANSFORM_TAIL, NEW_TRANSFORM_TAIL).replace("        for (state, value) in self.state.iter_mut().zip([a, b, c, d, e, f, g, h]) {", "        for (state, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {")
    body = "".join(line[4:] if line.startswith("    ") else line for line in body.splitlines(keepends=True))
    hash_text = hash_text[:start] + NEW_TRANSFORM_HEAD + body + "}" + hash_text[end:]
    tests_text = TESTS.read_text(encoding="utf-8") + LAWS
    manifest = MANIFEST.read_text(encoding="utf-8")
    if manifest.count('[dev-dependencies]\nblake3 = "1.8.2"\n') != 1:
        raise SystemExit("manifest hunk not found exactly once")
    manifest = manifest.replace('[dev-dependencies]\nblake3 = "1.8.2"\n', '[dev-dependencies]\nblake3 = "1.8.2"\nsha2 = "0.10.9"\n')
    return hash_text, tests_text, manifest

hash_text, tests_text, manifest = patched_sources()
print(f"hunks OK: hash {HASH.name} (3), hash tests (+2 laws), hash Cargo.toml (+ sha2 dev-dependency)")
if "--apply" in sys.argv:
    HASH.write_text(hash_text, encoding="utf-8"); TESTS.write_text(tests_text, encoding="utf-8"); MANIFEST.write_text(manifest, encoding="utf-8")
    print("applied — now: nice -n 15 cargo test -p semio-framework-hash --no-fail-fast; cargo test -p semio-hub --lib -- auth::; wasm32-wasip2 check of semio-framework-hash through the fleet mutex")
elif "--scratch" in sys.argv:
    scratch = ROOT / ".tmp-ticket/wp-h10/target/q2-scratch"
    (scratch / "src").mkdir(parents=True, exist_ok=True)
    (scratch / "src/hash.rs").write_text(hash_text.replace('#[path = "🧪️tests/🔬️unit/🦀️.rs"]', '#[path = "tests.rs"]'), encoding="utf-8")
    (scratch / "src/tests.rs").write_text(tests_text, encoding="utf-8")
    (scratch / "src/hash_before.rs").write_text(HASH.read_text(encoding="utf-8").replace('#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;', ""), encoding="utf-8")
    (scratch / "src/lib.rs").write_text('#![allow(dead_code)]\n#[path = "hash.rs"]\npub mod hash;\n#[path = "hash_before.rs"]\npub mod hash_before;\n', encoding="utf-8")
    (scratch / "src/main.rs").write_text('''//! 🏁️ Q2 bench: PBKDF2-HMAC-SHA256 at 210 000 iterations (HMAC keyed once, as the hub does) and 256 MiB of SHA-256,
//! on the patched (hardware-selected) and on the tree's current (portable) `Sha256`, best of five each.
macro_rules! bench {
    ($module:ident) => {{
        use h10_q2_scratch::$module::Sha256;
        let pbkdf2 = |password: &[u8], salt: &[u8], iterations: u32| -> [u8; 32] {
            let mut key = [0u8; 64];
            key[..password.len()].copy_from_slice(password);
            let (mut inner, mut outer) = (Sha256::new(), Sha256::new());
            inner.update(&key.map(|byte| byte ^ 0x36));
            outer.update(&key.map(|byte| byte ^ 0x5c));
            let mac = |message: &[u8]| { let mut i = inner.clone(); i.update(message); let mut o = outer.clone(); o.update(&i.finalize()); o.finalize() };
            let mut seed = salt.to_vec(); seed.extend_from_slice(&1u32.to_be_bytes());
            let mut block = mac(&seed);
            let mut accumulator = block;
            for _ in 1..iterations { block = mac(&block); for index in 0..32 { accumulator[index] ^= block[index]; } }
            accumulator
        };
        let mut best_pbkdf2 = f64::MAX;
        let mut digest = [0u8; 32];
        for _ in 0..5 { let started = std::time::Instant::now(); digest = pbkdf2(b"gm1-local-dev-pass-1", &[7u8; 16], 210_000); best_pbkdf2 = best_pbkdf2.min(started.elapsed().as_secs_f64() * 1000.0); }
        let data = vec![0x5au8; 256 << 20];
        let mut best_bulk = f64::MAX;
        let mut bulk = [0u8; 32];
        for _ in 0..5 { let started = std::time::Instant::now(); bulk = Sha256::digest(&data); best_bulk = best_bulk.min(started.elapsed().as_secs_f64() * 1000.0); }
        println!("{:<12} pbkdf2-210000 {:>7.1} ms  sha256-256MiB {:>7.1} ms ({:>5.0} MB/s)  digests {:02x?} {:02x?}", stringify!($module), best_pbkdf2, best_bulk, 268.435456 / (best_bulk / 1000.0), &digest[..6], &bulk[..6]);
    }};
}
fn main() {
    bench!(hash_before);
    bench!(hash);
}
''', encoding="utf-8")
    (scratch / "Cargo.toml").write_text('[package]\nname = "h10-q2-scratch"\nversion = "0.0.0"\nedition = "2021"\npublish = false\n\n[workspace]\n\n[dev-dependencies]\nblake3 = "1.8.2"\nsha2 = "0.10.9"\n\n[profile.release]\nopt-level = 3\n', encoding="utf-8")
    env = dict(os.environ, CARGO_TARGET_DIR=str(ROOT / ".tmp-ticket/wp-h10/target"), CARGO_INCREMENTAL="0")
    for command in (["nice", "-n", "15", "cargo", "test", "--release", "--no-fail-fast", "--lib"], ["nice", "-n", "15", "cargo", "run", "--release", "--bin", "h10-q2-scratch"], ["nice", "-n", "15", "cargo", "check", "--release", "--lib", "--target", "x86_64-unknown-linux-gnu"]):
        print("$", " ".join(command), flush=True)
        result = subprocess.run(command, cwd=scratch, env=env)
        if result.returncode != 0:
            raise SystemExit(result.returncode)
else:
    print("dry run: nothing written")

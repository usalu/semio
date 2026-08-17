I have enough context on the repo conventions (workspace member crates live at `<domain>/<name>/rs` with a root-level `lib.rs`, e.g. `mathematical/random/rs` → crate `mathematical_random`). The new crate should live at `/Users/ueli/Documents/semio/mathematical/entropy/rs` (crate `mathematical_entropy`, `edition = "2021"`, `path = "lib.rs"`, added to the root `Cargo.toml` members list). Below is the full numerical-infrastructure reference document.

---

# `mathematical_entropy` — Zero-Dependency Numerical Infrastructure Reference

Conventions used throughout: `eps = f64::EPSILON ≈ 2.22e-16`; "rel. err." = relative error; LOC estimates are implementation-only (tests excluded, roughly double for tests). All natural-log unless stated; expose a `LogBase { Nats, Bits, Base(f64) }` conversion at the API layer only — all internals in nats.

Suggested module map (one file per numbered item, roughly):

```
lib.rs
special.rs      (1)      sum.rs        (2)      fft.rs      (3)
window.rs       (4)      spectral.rs   (5)      wavelet.rs  (6)
kdtree.rs       (7)      eigen.rs      (8)      linalg.rs   (9)
rankstats.rs    (10)     rng.rs + surrogate.rs (11)
discrete.rs     (12)     spacing.rs    (13)     kde.rs      (14)
ordinal.rs      (15)     lz.rs         (16)     pid.rs      (17)
embed.rs        (18)     glcm.rs       (19)     graph.rs    (20)
hygiene.rs      (21)
```

---

## 1. Special functions (`special.rs`)

### 1.1 `ln_gamma(x)` — Lanczos, g = 7, n = 9
Use the Godfrey/Numerical-Recipes g=7 coefficient set (widely reproduced, ~1e-13 rel. err. on x > 0.5):

```
g = 7
c = [ 0.99999999999980993,
      676.5203681218851,   -1259.1392167224028,
      771.32342877765313,  -176.61502916214059,
      12.507343278686905,  -0.13857109526572012,
      9.9843695780195716e-6, 1.5056327351493116e-7 ]

x >= 0.5:
  a = c[0] + Σ_{i=1..8} c[i] / (x - 1 + i)
  t = x + g - 0.5
  lnΓ(x) = 0.5·ln(2π) + (x - 0.5)·ln t − t + ln a
x < 0.5 (reflection):
  lnΓ(x) = ln(π / |sin(πx)|) − lnΓ(1 − x)     // NaN/±inf at non-positive integers
```
Pitfalls: `sin(πx)` for large negative x — compute `π * (x - x.round())` style argument reduction before `sin` to keep the reflection accurate; return `+inf` at 0, −1, −2, …
LOC ~40. O(1). Oracles: `lnΓ(0.5) = ln√π = 0.5723649429247001`, `lnΓ(1)=lnΓ(2)=0`, `lnΓ(10)=ln 362880 = 12.801827480081469`, recurrence check `lnΓ(x+1) − lnΓ(x) − ln x ≈ 0` over random x.

### 1.2 `digamma(x)` — recurrence + Bernoulli asymptotic
```
x < 0: reflection ψ(x) = ψ(1−x) − π/tan(πx)
push up: while x < 6 { acc -= 1/x; x += 1 }
asymptotic (x ≥ 6):
  ψ(x) ≈ ln x − 1/(2x) − 1/(12x²) + 1/(120x⁴) − 1/(252x⁶)
        + 1/(240x⁸) − 1/(132x¹⁰) + 691/(32760x¹²) − 1/(12x¹⁴)
```
Rel. err. ≲ 1e-14 with threshold 6 and terms through x⁻¹⁴ (threshold 10 lets you drop the last two terms). Pitfall: near ψ's positive root x₀ ≈ 1.46163…, *relative* error blows up (value ≈ 0); state accuracy target as absolute ~1e-14 there.
LOC ~30. Oracles: `ψ(1) = −γ = −0.5772156649015329`, `ψ(0.5) = −γ − 2ln2 = −1.9635100260214235`, `ψ(x+1) = ψ(x) + 1/x`.

### 1.3 `trigamma(x)` — same scheme
```
x < 0: ψ'(1−x) + ψ'(x) = π²/sin²(πx)  (reflection)
while x < 6 { acc += 1/x²; x += 1 }
ψ'(x) ≈ 1/x + 1/(2x²) + 1/(6x³) − 1/(30x⁵) + 1/(42x⁷) − 1/(30x⁹) + 5/(66x¹¹)
```
LOC ~25. Oracles: `ψ'(1) = π²/6`, `ψ'(0.5) = π²/2`, recurrence `ψ'(x) = ψ'(x+1) + 1/x²`.

### 1.4 Incomplete gamma P(a,x), Q(a,x) — series + Lentz continued fraction
This is the workhorse: chi-square p-values **and** erf/erfc fall out of it.
```
P(a,x): if x < a + 1  (series, NR §6.2):
   sum = 1/a; term = 1/a
   loop: term *= x/(a+n); sum += term; until |term| < |sum|·1e-16
   P = sum · exp(−x + a·ln x − lnΓ(a))
Q(a,x): if x ≥ a + 1  (modified Lentz on the standard CF):
   Q = exp(−x + a·ln x − lnΓ(a)) · CF,
   CF = 1/(x+1−a −  1·(1−a)/(x+3−a − 2·(2−a)/(x+5−a − …)))
P + Q = 1; always compute the convergent one and subtract.
```
Pitfalls: Lentz needs `tiny = 1e-300` floor for zero denominators; cap iterations (~300) and return an error variant rather than looping. Underflow of the prefactor for large x is correct behavior (Q → 0).
Chi-square survival: `p = Q(k/2, x/2)`. LOC ~80. O(√a + const). Oracles: `P(1,x) = 1 − e⁻ˣ`, `Q(0.5, x²) = erfc(x)`, scipy golden values e.g. `Q(5, 10) = 0.029252688076961127`.

### 1.5 erf / erfc — via incomplete gamma
```
erf(x)  = sign(x) · P(0.5, x²)
erfc(x) = x ≥ 0 ? Q(0.5, x²) : 2 − Q(0.5, x²)
```
Rel. err. ~1e-13; erfc stays accurate into the deep tail (down to ~1e-300) because Q uses the CF. LOC ~10 on top of 1.4. Oracles: `erf(1) = 0.8427007929497149`, `erfc(3) = 2.209049699858544e-5`, `erf(x)+erfc(x)=1`.

### 1.6 Normal CDF and inverse — erfc + Acklam-with-polish
```
Φ(x) = 0.5 · erfc(−x / √2)          // never 1 − Φ(−x); use erfc branch directly
Φ⁻¹(p): Acklam rational approx (6+6 central coeffs, 6+4 tail coeffs; break at p=0.02425)
        → x₀ with rel. err. ~1.15e-9
        one Halley refinement:
          e = Φ(x₀) − p
          u = e · √(2π) · exp(x₀²/2)
          x = x₀ − u / (1 + x₀·u/2)
```
One Halley step lands at full double precision (~1e-15), beating the target without transcribing Wichura AS241's larger coefficient tables. Coefficient source: Acklam's published a[1..6], b[1..5], c[1..6], d[1..4] (any standard reproduction; commit them with a comment citing "P.J. Acklam, An algorithm for computing the inverse normal CDF, 2003"). Pitfall: handle p ∈ {0,1} → ±inf, p outside [0,1] → NaN; in the extreme tail (p < 1e-300) the Halley step needs `exp(x²/2)` — guard against overflow by skipping refinement when |x₀| > 37.
LOC ~50. Oracles: `Φ⁻¹(0.975) = 1.959963984540054`, `Φ⁻¹(0.5) = 0`, roundtrip `Φ⁻¹(Φ(x)) − x| < 1e-13` for x ∈ [−8, 8].

### 1.7 Log-factorial cache
Struct-held `Vec<f64>` (or `OnceLock<Vec<f64>>`) of `ln k!` for k ≤ 1024: exact f64 factorials for k ≤ 20 (170 would overflow f64 at k=171 anyway; use lnΓ from k=21), `ln_gamma(k+1)` beyond the cache. Do **not** build by cumulative `+= ln k` (accumulates ~n·eps error); call lnΓ per entry. LOC ~15.

**Item 1 totals: ~250 LOC.** All O(1) per call.

---

## 2. Stable summation & entropy primitives (`sum.rs`)

### 2.1 Neumaier (improved Kahan)
```
sum = 0; c = 0
for x: t = sum + x
       c += if |sum| ≥ |x| { (sum − t) + x } else { (x − t) + sum }
       sum = t
return sum + c
```
Error O(eps) independent of n. Use for: every `Σ p·ln p` (entropy of near-deterministic distributions is a sum of large-magnitude cancelling terms — naive summation gives entropy estimates off by 1e-13 absolute, which matters when the true value is 1e-10), log-likelihood accumulations, NSB evidence sums.

### 2.2 Pairwise summation
Recursive halving with base-case block of 128 summed naively; error O(eps·log n). Use for large data arrays (means, energies, Σx², window power) where Neumaier's 4 flops/element is unnecessary but naive O(n·eps) drift on n ~ 10⁷ is not acceptable. Rule of thumb: Neumaier for probability-space sums (n ≤ ~10⁶, correctness-critical), pairwise for signal-space sums.

### 2.3 log-sum-exp
```
m = max(x);  if m == −inf → −inf
lse = m + ln( Σ exp(x_i − m) )     // inner sum via Neumaier
```
Used by KDE leave-one-out log-density and NSB posterior weighting.

### 2.4 `xlogx`
```
xlogx(0) = 0 exactly; x > 0 → x·ln x; x < 0 → NaN (caller validates)
xlogy(x, y) analog for cross-entropy: xlogy(0, 0) = 0.
```
Never compute `p.ln()` then multiply without the zero guard — `0 * −inf = NaN` silently poisons entropy sums.

LOC ~70 total. Oracles: sum of `[1e100, 1, −1e100]` = 1 under Neumaier; entropy of `[1−1e-12, 1e-12]` vs analytic `−(1−δ)ln(1−δ) − δ·ln δ` using `ln_1p`.

---

## 3. Real FFT (`fft.rs`)

### 3.1 Core: iterative radix-2 DIT complex FFT
Hand-rolled `Complex64 { re, im }` (Add/Sub/Mul, conj, ~30 LOC — no external num crate). Algorithm: bit-reversal permutation (precomputed index table), then log₂n butterfly stages using precomputed twiddles `w[j] = exp(−2πi·j/n)` for j < n/2. Inverse = forward with conjugated twiddles, scale by 1/n.

**Plan struct** (this is where caching lives):
```rust
pub struct FftPlan { n: usize, twiddles: Vec<Complex64>, bitrev: Vec<u32>, }
impl FftPlan { fn new(n: usize) -> Self; fn forward(&self, buf: &mut [Complex64]); fn inverse(...); }
pub struct FftCache(HashMap<usize, Rc<FftPlan>>)   // std HashMap; Welch/STFT hold one cache
```
Twiddles computed via `(theta).sin_cos()` per index — do NOT generate by repeated complex multiplication (error grows O(n·eps)); per-index sin_cos gives roundtrip error O(eps·log n) ≈ 1e-15.

### 3.2 Real FFT packing (n even)
Pack `z_m = x_{2m} + i·x_{2m+1}`, run the n/2 complex FFT → Z, unpack for k = 0..n/2:
```
Fe_k = (Z_k + conj(Z_{n/2−k})) / 2                 // Z_{n/2} ≡ Z_0
Fo_k = (Z_k − conj(Z_{n/2−k})) / (2i)
X_k  = Fe_k + e^{−2πik/n} · Fo_k
X_0 = ΣFe+ΣFo real; X_{n/2} = Fe_0 − Fo_0 (real, Nyquist)
```
Output n/2+1 complex bins. ~2× speedup over full complex FFT — worth the ~50 LOC for Welch/STFT throughput.

### 3.3 Arbitrary length — **implement Bluestein; it is worth it**
Reasoning: for Welch/STFT you can always round `nfft` up to a power of two (spectral interpolation, unbiased). But **phase-randomized / IAAFT surrogates (item 11) require an exact-length DFT of the original series** — zero-padding changes the frequency grid and the surrogate no longer has the sample autocovariance of the data; truncating discards data. Bluestein (chirp-z) is ~80 LOC given the complex FFT:
```
DFT_n(x)_k = b*_k · IDFT_M( DFT_M(a) ⊙ DFT_M(b) )_k
  a_j = x_j · w_j,  w_j = exp(−iπ j²/n)   (chirp; reduce j² mod 2n in i64 to avoid
                                            precision loss in the angle for large n)
  b_j = conj(w_j) for j and mirrored at M−j;  M = next_pow2(2n − 1)
```
Pitfall: compute the chirp angle as `π · ((j*j) mod (2n)) / n` using integer mod — `j²` as f64 loses ulps for n > ~10⁵ and corrupts the chirp.
Policy: `FftPlan::new` dispatches — pow2 → radix-2, else → Bluestein plan (which embeds a pow2 plan). Welch/STFT default `nfft = next_pow2(nperseg)` so they stay on the fast path; surrogates use exact n.

LOC: complex core ~120, real packing ~50, Bluestein ~80, cache ~20 → **~270**. Complexity O(n log n). Oracles: DFT of `δ[0]` = all-ones; DFT of `cos(2πk₀t/n)` = two bins at n/2 amplitude; roundtrip `ifft(fft(x)) − x| < 1e-12·‖x‖`; Bluestein n=12 vs direct O(n²) DFT (keep a 15-line naive DFT in tests as oracle); Parseval `Σ|x|² = (1/n)Σ|X|²`.

---

## 4. Window functions (`window.rs`)

All windows generated **periodic** by default (denominator N) for spectral estimation, with a `Symmetric` flag (denominator N−1) for filter design. Formulas, n = 0..N−1, φ = 2πn/N (periodic):

| Window | w[n] |
|---|---|
| Rectangular | 1 |
| Hann | 0.5 − 0.5·cos φ |
| Hamming | 25/46 − 21/46·cos φ  (= 0.54347826…; use exact rationals, note classic 0.54/0.46 differs) |
| Blackman | 0.42 − 0.5·cos φ + 0.08·cos 2φ |
| Blackman–Harris (4-term, −92 dB) | 0.35875 − 0.48829·cos φ + 0.14128·cos 2φ − 0.01168·cos 3φ |
| Kaiser(β) | I₀(β·√(1 − r²)) / I₀(β),  r = 2n/(N−1) − 1 |
| Tukey(α) | cosine taper over first/last ⌊αN/2⌋ samples: 0.5(1+cos(π(2n/(αN) − 1))), flat 1 between |

**I₀ series** (all β of practical interest, β ≤ ~30):
```
I₀(x): t = 1; s = 1; k = 1
loop { t *= (x/2)²/k²; s += t; k += 1 } until t < s·1e-17     (~40 iterations at x=30)
```
LOC ~90. Oracles: Hann sums `Σw = N/2`, `Σw² = 3N/8` (periodic); `I₀(1) = 1.2660658777520084`; Kaiser β=0 ≡ rectangular; endpoint checks per symmetric/periodic convention against scipy `get_window` values.

---

## 5. Welch PSD, STFT, multitaper (`spectral.rs`)

### 5.1 Welch
Parameters: `nperseg L` (default min(256, n)), `noverlap` (default L/2), window (default Hann), `nfft = next_pow2(L)`, `fs`, optional per-segment mean removal (default on — DC leakage otherwise swamps low-frequency entropy measures).
```
K segments at hop = L − noverlap  (discard tail remainder)
U = Σ w[n]²                                      // window power (unnormalized)
for each segment: y = w ⊙ (x_seg − mean); X = rfft(y, nfft); acc_k += |X_k|²
Pxx_k = acc_k / (K · fs · U)
one-sided: Pxx_k *= 2  for 0 < k < nfft/2   (not DC, not Nyquist)
```
Pitfall: normalize by `Σw²`, not `(Σw)²` (that is the *spectrum* vs *density* scaling — offer both via enum `Scaling::{Density, Spectrum}`, Spectrum divides by `(Σw)²`). Check: `Σ Pxx · Δf ≈ variance(x)` for white noise (Parseval-based test).
Spectral entropy consumer: `p_k = Pxx_k / ΣPxx`, `H = −Σ xlogx(p_k)`, normalized by `ln(nbins)`.

### 5.2 STFT
Same segmentation machinery, but returns the complex frame matrix (nfreq × nframes) plus the window/hop metadata. Analysis-only (no inverse/COLA needed for entropy). Share the segment iterator with Welch — one implementation, two consumers.

### 5.3 Multitaper — **sine tapers (Riedel–Sidorenko), not DPSS**
```
taper k (k = 1..K):  v_k[n] = √(2/(N+1)) · sin(π k (n+1)/(N+1)),   n = 0..N−1
S(f) = (1/K) Σ_k |FFT(v_k ⊙ x)|²    (uniform weights; one-sided scaling as in Welch,
                                      U = 1 since tapers are orthonormal: divide by fs only)
```
Justification (state in docs): sine tapers are the exact eigenvectors of a tridiagonal approximation to the concentration problem; they are closed-form, exactly orthonormal, and Riedel–Sidorenko show they are asymptotically optimal for the *smoothed-spectrum* MSE criterion. DPSS requires solving a symmetric tridiagonal eigenproblem plus in-band concentration bookkeeping — ~300 extra LOC and a numerically delicate inverse-iteration step, for accuracy gains that are irrelevant to entropy-of-spectrum functionals (which integrate over frequency anyway). Sidelobe behavior: sine tapers have ~−26 dB first sidelobe vs DPSS's tunable −60+ dB — document that for extreme-dynamic-range spectra users should increase K or pre-whiten. Effective bandwidth ≈ (K+1)/(N+1) · fs/2 rule for choosing K (default K = 5).

LOC: ~180 total. O(K·n log n). Oracles: white-noise PSD flat at σ²/(fs/2)-consistent level with variance integral matching to ~1%; single sinusoid at bin center → peak power = A²/2 under `Spectrum` scaling; multitaper of AR(1) vs analytic spectrum σ²/|1−φe^{−iω}|² within ~10% at n = 4096.

---

## 6. Wavelets (`wavelet.rs`)

### 6.1 Choice: filter-bank convolution DWT (not lifting)
Lifting is ~2× faster but needs a hand-derived factorization per wavelet; the filter bank is one generic routine over a coefficient table — for D4/D6/D8 + Haar that is the right trade. One decomposition level:
```
a[k] = Σ_m h[m] · x[(2k + m) mod N]      (periodic mode; N even)
d[k] = Σ_m g[m] · x[(2k + m) mod N],  g[m] = (−1)^m · h[L−1−m]   (QMF)
```

### 6.2 Coefficients (orthonormal scaling filters, Σh = √2, Σh² = 1)
Source: Daubechies, *Ten Lectures on Wavelets*, Table 6.1 (equivalently PyWavelets `db2/db3/db4` — commit values with 16 significant digits and a cross-check test `Σh=√2`, `Σh² = 1`, `Σ h[m]h[m+2j] = 0`):
```
Haar: [1/√2, 1/√2]
D4:   [(1+√3)/(4√2), (3+√3)/(4√2), (3−√3)/(4√2), (1−√3)/(4√2)]
      = [0.4829629131445341, 0.8365163037378079, 0.2241438680420134, −0.1294095225512604]
D6:   [0.3326705529500825, 0.8068915093110924, 0.4598775021184914,
       −0.1350110200102546, −0.0854412738820267, 0.0352262918857095]
D8:   [0.2303778133088964, 0.7148465705529154, 0.6308807679298587, −0.0279837694168599,
       −0.1870348117190931, 0.0308413818355607, 0.0328830116668852, −0.0105974017850690]
```
(Implementer note: verify committed constants against the orthogonality identities in a unit test rather than trusting transcription.)

### 6.3 Boundary modes
`Periodic` (default): orthogonal transform, exact energy conservation `Σx² = Σa² + Σd²` — this is the property wavelet entropy needs. `Symmetric` (half-sample reflection) and `Zero`: standard padded convolution, output length ⌈(N+L−1)/2⌉, energy *not* exactly conserved — document that subband "energies" become approximate. Odd-length + periodic: pad one reflected sample, note in docs.

### 6.4 SWT and packets
SWT (à trous): no downsampling; at level j convolve with filters upsampled by 2^{j−1} (insert zeros — implement as strided indexing, don't materialize). Output: N coefficients per level, shift-invariant. Packets: recurse the filter bank on `d` as well as `a`; tree addressed by (level, node); optional best-basis via Coifman–Wickerhauser additive Shannon cost `−Σ v²ln v²` on normalized coefficients.

### 6.5 Wavelet entropy — the actual minimum
Only needs: multi-level periodic DWT → per-subband energies `E_j = Σ d_{j,k}²` (+ final approximation), `p_j = E_j/ΣE_j`, `H = −Σ xlogx(p_j)`. Ship DWT first; SWT/packets are additive extras.

LOC: DWT+inverse ~120, SWT ~60, packets ~80, entropy glue ~30 → **~290**. O(n) per level (filter length const), O(n·J) total; SWT O(n·J·L). Oracles: perfect reconstruction `‖idwt(dwt(x)) − x‖ < 1e-12`; energy conservation (periodic); Haar of `[1,1,…]` → d ≡ 0; PyWavelets golden vectors for db2 on a fixed 16-sample input.

---

## 7. kNN infrastructure (`kdtree.rs`)

### 7.1 k-d tree
Implicit array layout, no per-node allocation:
```rust
struct KdTree { pts: Vec<f64>, dim: usize, idx: Vec<u32>, /* nodes as (split_dim, split_val, ranges) or median-recursive on idx */ }
```
Build: recursive; at each node pick split dim = depth mod d (or widest spread — marginally better, 10 LOC), split with `slice::select_nth_unstable_by` on the index array (std introselect, O(n) expected — no hand-rolled median-of-medians needed). Leaf size ~16 (linear scan at leaves beats deep recursion). Build O(n log n).

Queries (both metrics via a `Metric` enum with `dist(a,b)` and per-axis `axis_gap`):
- **kNN**: max-heap of size k (`BinaryHeap<(OrderedDist, u32)>` — hand-roll `OrderedDist(f64)` with `Ord` via `total_cmp`); descend near side first, prune far side when `axis_gap ≥ heap.peek()`. Support `exclude_self` (mandatory for all estimators below).
- **Radius count, strict**: count points with `dist < r` (KSG requires *strict* inequality; make strictness a parameter, default strict).
Chebyshev (L∞) is the default metric (KSG standard: joint max-norm makes marginal counts consistent). Complexity: query O(log n) expected, degrading toward O(n^{1−1/d}) for d ≳ 8 — document that above d ≈ 10–15 brute force O(n) per query is competitive; provide a brute-force fallback path chosen automatically when `d > 12`.

### 7.2 Estimator formulas (exact, with conventions pinned)
Let ε_i = distance from point i to its k-th nearest neighbor (self excluded), n = sample count, d = dimension, ψ = digamma.

**Kozachenko–Leonenko differential entropy:**
```
Ĥ = ψ(n) − ψ(k) + ln V_d + (d/n) Σ_i ln ε_i
V_d(L∞) = 2^d  → ln V_d = d·ln 2
V_d(L2) = π^{d/2} / Γ(d/2 + 1)
```
Convention pin: ε here is the *full* distance (some papers use twice the distance and drop the 2^d — state which one in code comments; the pair above is self-consistent).

**KSG-1 mutual information** (joint space with L∞ over both blocks):
```
ε_i = joint-space distance to k-th neighbor (strict-ball convention)
n_x(i) = #{ j≠i : ‖x_j − x_i‖ < ε_i },  n_y likewise
Î = ψ(k) + ψ(n) − (1/n) Σ_i [ ψ(n_x(i)+1) + ψ(n_y(i)+1) ]
```
**KSG-2:**
```
ε_x(i), ε_y(i) = per-marginal distances to the k-th joint neighbor's projections
n_x(i) = #{ j≠i : ‖x_j − x_i‖ ≤ ε_x(i) }   (NON-strict here)
Î = ψ(k) − 1/k + ψ(n) − (1/n) Σ_i [ ψ(n_x(i)) + ψ(n_y(i)) ]
```
**KSG conditional MI** (for continuous transfer entropy, TE = I(X⁺; Y_hist | X_hist)):
```
Î(X;Y|Z) = ψ(k) − (1/n) Σ_i [ ψ(n_{xz}(i)+1) + ψ(n_{yz}(i)+1) − ψ(n_z(i)+1) ]
```
(joint kNN in (x,y,z); counts in the three marginal-joint subspaces with strict ε_i balls).

**Ross mixed MI** (X discrete, Y continuous):
```
for each i: k-th NN distance δ_i computed among points sharing label x_i (N_{x_i} of them)
m_i = #{ j≠i : ‖y_j − y_i‖ < δ_i } over ALL points
Î = ψ(n) − (1/n)Σ ψ(N_{x_i}) + ψ(k) − (1/n)Σ ψ(m_i)
```
(if a class has ≤ k members, reduce k for that class or error — pick "error with message, suggest k" for honesty).

### 7.3 Ties / duplicates
Duplicate points → ε = 0 → `ln 0 = −inf` and marginal counts blow up. Policy: (a) detect exact duplicates during build (sort-based, O(n log n)); (b) if found, either error (`TieError`) or, when the caller passes `jitter: Some(seed)`, add uniform noise of amplitude `1e-10 · per-dim std` from the seeded RNG (item 11) before building. Document that jitter makes results seed-dependent at the ~1e-9 level. Never jitter silently.

LOC: tree ~220, estimators ~180 → **~400**. Oracles: KL entropy of N(0,1) → ½ln(2πe) = 1.4189385332046727 (n = 10⁴, tolerance ~0.02, seeded); KL of Uniform(0,1)^d → 0; KSG-1 on bivariate Gaussian ρ: I = −½ln(1−ρ²) (ρ = 0.6 → 0.2231435513); KSG on independent data → ≈0 ± 0.01; brute-force O(n²) neighbor reference implementation in tests to validate the tree exactly (same counts, same distances).

---

## 8. Symmetric eigensolver, SVD, Cholesky log-det (`eigen.rs`)

### 8.1 Cyclic Jacobi (dense symmetric)
Row-cyclic sweeps over all p < q; for each pair with `|a_pq| > threshold` compute the rotation:
```
θ = (a_qq − a_pp) / (2 a_pq);  t = sign(θ)/(|θ| + √(θ²+1));  c = 1/√(t²+1); s = t·c
```
apply to rows/cols p,q and accumulate into V (eigenvectors) if requested.
Convergence: stop when `off(A) = √(Σ_{p<q} a_pq²) ≤ eps_j · ‖A‖_F` with `eps_j = 1e-14`, or after 30 sweeps (error out — never seen in practice; quadratic convergence means 6–10 sweeps). Complexity O(n³) per sweep, ~8 sweeps ⇒ fine to n ≈ 1500–2000 (document; covariance/Laplacian/density matrices in entropy work are ≤ a few hundred). Jacobi's virtue over QR: ~150 LOC, unconditionally stable, small eigenvalues computed to high *relative* accuracy — which is exactly what `Σ λ ln λ` (von Neumann) cares about.

### 8.2 SVD — **one-sided Jacobi (Hestenes), not eig(AᵀA)**
Recommendation: one-sided Jacobi. Rationale: eig(AᵀA) squares the condition number — singular values below `√eps·σ_max ≈ 1e-8·σ_max` are destroyed. SVD-entropy normalizes σ_i (or σ_i²) to probabilities, and while tiny σ carry little entropy weight, rank detection and pseudo-determinants (item 9) reuse this SVD and *do* care. One-sided Jacobi is ~120 LOC and reuses the rotation kernel:
```
repeat sweeps over column pairs (i, j):
  a = ‖A_i‖², b = ‖A_j‖², c = A_iᵀA_j
  if |c| > eps·√(a·b): rotate columns i,j (same c,s formulas) to zero the inner product
until all |c| ≤ eps·√(ab)  (typically 6–10 sweeps)
σ_i = ‖A_i‖; U_i = A_i/σ_i; V = accumulated rotations
```
SVD entropy: `p_i = σ_i / Σσ` (or σ²-convention — expose both, document default σ, cite Alter et al. use σ² normalized), `H = −Σ xlogx(p_i)`.

### 8.3 Cholesky + log-det (Gaussian entropy, log-det divergence)
```
standard in-place lower Cholesky; log|Σ| = 2 Σ ln L_ii
Gaussian entropy: H = d/2 · ln(2πe) + ½ log|Σ|
```
Regularization fallback: if a pivot ≤ `tol = d·eps·max(diag)`, retry with ridge `Σ + λI`, λ = 1e-12·tr(Σ)/d, escalating ×100 up to 1e-6·tr/d; if still failing, fall back to Jacobi eigenvalues and pseudo-log-det (Σ ln λ over λ > tol) with a `Degenerate` flag in the result type so callers know the entropy is of a rank-deficient (−inf true value) distribution. Never silently return the ridged value without the flag.

LOC: Jacobi ~150, one-sided ~120, Cholesky ~50 → **~320**. Oracles: eigen of `[[2,1],[1,2]]` → {1,3}; Frobenius test `‖VΛVᵀ − A‖ < 1e-13‖A‖`; orthogonality `‖VᵀV − I‖ < 1e-13`; Hilbert 6×6 eigenvalues vs known values (relative accuracy of the tiny ones is the interesting assertion); SVD of a rank-2 4×4 → σ₃ = σ₄ < 1e-14·σ₁; Cholesky log-det of AR(1)-structured covariance vs analytic `(d−1)ln(1−ρ²)`.

---

## 9. Small linear-algebra kit (`linalg.rs`)

Row-major `Mat { data: Vec<f64>, rows, cols }` (~60 LOC of ops: matmul, transpose, symmetric check). 
- **Covariance**: two-pass (means first, then centered products; pairwise summation), divisor n−1; single-pass Welford only if a streaming API is added later — two-pass is more accurate and simpler.
- **Correlation**: cov scaled by 1/(s_i s_j); guard zero-variance columns → error variant (correlation undefined), don't emit NaN.
- **Solve**: forward/back substitution against the Cholesky factor (SPD systems only — that is all the estimators need); no general LU.
- **Pseudo-determinant**: from Jacobi (or SVD) spectrum, `pdet = Π_{λ_i > tol} λ_i` with `tol = n · eps · λ_max`; return `(log_pdet, rank)`. Used by Gaussian entropy on degenerate covariances and log-det divergence.

LOC ~150. Oracles: cov of standardized data ≡ corr; solve roundtrip `‖A·solve(A,b) − b‖`; pdet of `diag(1, 1e-20)` → rank 1, log_pdet 0.

---

## 10. Sorting-based statistics (`rankstats.rs`)

- **Quantile**: type-7 (linear interpolation), `h = (n−1)q`, `x_(⌊h⌋) + frac·(x_(⌊h⌋+1) − x_(⌊h⌋))`, on a sorted copy (`sort_unstable_by(total_cmp)`; NaN policy: error on any NaN at API boundary — apply crate-wide).
- **Midranks**: argsort, walk tie runs, assign mean rank `(lo+hi)/2 + 1`. Needed by copula/rank transforms and Spearman-based MI; also the rank-remap kernel used by IAAFT.
- **ECDF**: sorted values + `F(t) = (#x ≤ t)/n` via `partition_point`.
- **Wasserstein-1**: equal n: `(1/n) Σ |x_(i) − y_(i)|`. Unequal n: merge both sorted samples into breakpoints t_1 < … and `W₁ = Σ_i |F_x(t_i) − F_y(t_i)| · (t_{i+1} − t_i)`. O((n+m)log(n+m)).
- **KS distance**: sup over the merged points of |F_x − F_y|, evaluating both step functions just-after each point (two-pointer walk; the classic pitfall is missing the discontinuity — evaluate at each sample of *both* series).
- **Energy distance**: `D² = 2·E|X−Y| − E|X−X′| − E|Y−Y′|`; each term O(n log n) via sorted prefix sums using `Σ_{i<j}(x_(j) − x_(i)) = Σ_{k=0}^{n−1} (2k − n + 1)·x_(k)` (0-indexed), and the cross term by merging the two sorted arrays with running counts/sums.

LOC ~180. Oracles: W₁(U(0,1), U(0,1)+c) = |c| exactly for shifted samples; KS of identical samples = 0; energy distance of identical distributions ≈ 0; midranks of `[1,2,2,3]` = `[1, 2.5, 2.5, 4]`; quantile matches `numpy.percentile(interpolation='linear')` on committed vectors.

---

## 11. RNG & surrogates (`rng.rs`, `surrogate.rs`)

### 11.1 Generators
```
splitmix64(state): state += 0x9E3779B97F4A7C15
  z = state; z = (z ^ (z>>30)) * 0xBF58476D1CE4E5B9
  z = (z ^ (z>>27)) * 0x94D049BB133111EB;  return z ^ (z>>31)

xorshift64* : x ^= x>>12; x ^= x<<25; x ^= x>>27; return x·0x2545F4914F6CDD1D
f64 in [0,1): (u64 >> 11) as f64 * (1.0/9007199254740992.0)     // 53-bit mantissa
standard normal: Box–Muller (pairs, cached second value) or Marsaglia polar — pick polar (no trig)
bounded ints: rejection — zone = u64::MAX − (u64::MAX % bound); retry while raw ≥ zone
```
**Seeding policy**: every stochastic entry point takes `seed: u64` explicitly (no global RNG, no OS entropy — determinism is a feature for scientific reproducibility). Substreams: `state_i = splitmix64(seed ⊕ STREAM_CONST_i)` where each purpose (bootstrap, permutation test, jitter, surrogate) has a distinct documented constant, so adding a consumer never perturbs existing streams. Seed 0 is legal (splitmix64 has no bad seeds; never seed xorshift64* with 0 directly — always pass through splitmix64 first).

### 11.2 Fisher–Yates
`for i in (1..n).rev() { j = gen_range(0..=i); swap(i, j) }` — with the rejection-sampled bounded int (modulo bias at n near 2⁶³ is academic but free to avoid).

### 11.3 Phase-randomized surrogate (exact-length FFT — see item 3)
```
X = fft(x)  (length n, Bluestein if not pow2)
for k in 1..⌈n/2⌉: φ ~ U(0,2π); X_k *= e^{iφ}; X_{n−k} = conj(X_k)
keep X_0; if n even keep X_{n/2} real (randomize sign optionally — keep it fixed)
s = re(ifft(X))     // imaginary residue < 1e-12, assert & drop
```

### 11.4 AAFT / IAAFT
AAFT: generate Gaussian white noise, sort-map data onto it by rank, phase-randomize the Gaussianized series, then rank-map the sorted original amplitudes back.
IAAFT loop (the standard Schreiber–Schmitz iteration):
```
s_sorted = sorted(x);  A_k = |fft(x)|            // target amplitudes (both domains)
r = seeded permutation of x
loop (max_iter = 100):
  Y = fft(r);  Y_k = A_k · Y_k/|Y_k|  (keep phase, |Y_k|=0 → random phase)
  y = re(ifft(Y))
  r_new[rank_order_of(y)] = s_sorted               // amplitude adjustment
  if ranks(r_new) == ranks(r): break               // fixed point
  r = r_new
return r  (exact amplitudes; report residual spectral error ‖|fft(r)| − A‖/‖A‖)
```
Convergence: typically < 30 iterations; it can 2-cycle — also break when the rank permutation repeats one from the previous iteration.

LOC: rng ~90, surrogates ~130 → **~220**. Oracles: fixed seed → committed first-10 outputs (regression pin); uniformity chi-square on 10⁶ draws; phase surrogate preserves periodogram exactly (‖|FFT(s)| − |FFT(x)|‖ < 1e-10) and sample mean; IAAFT output is a permutation of the input values *exactly* (multiset equality).

---

## 12. Discrete estimator math (`discrete.rs`)

Input everywhere: counts `n_i` (u64), N = Σn_i, K = support size (observed or declared). All sums Neumaier.

- **MLE**: `Ĥ = ln N − (1/N) Σ n_i ln n_i` (this form avoids forming p_i; use the log-factorial-style k·ln k table for small counts if profiling warrants — not required).
- **Miller–Madow**: `Ĥ_MM = Ĥ_MLE + (K_obs − 1)/(2N)`, K_obs = #{n_i > 0}.
- **Grassberger (2003)**: `Ĥ_G = ln N − (1/N) Σ n_i G(n_i)` with
  `G(n) = ψ(n) + ½·(−1)ⁿ·[ψ((n+1)/2) − ψ(n/2)]`. Cache G(n) for n ≤ 1024.
- **Chao–Shen**: coverage `Ĉ = 1 − f₁/N` (f₁ = #singletons; if f₁ = N use Ĉ = 1 − (N−1)/N to avoid Ĉ=0), `p̃_i = Ĉ·n_i/N`,
  `Ĥ_CS = −Σ_i [ p̃_i ln p̃_i / (1 − (1 − p̃_i)^N) ]`; compute `(1−p̃)^N` as `exp(N·ln_1p(−p̃))`.
- **Schürmann–Grassberger** (Dirichlet α = 1/K posterior-mean entropy; general α exposed as `bayes_entropy(counts, alpha)`):
  `Ĥ_α = ψ(N + Kα + 1) − (1/(N + Kα)) Σ_i (n_i + α)·ψ(n_i + α + 1)`.
- **NSB** — implement via fixed-order quadrature (recommended, ~150 LOC, item is flagged risky):
  - Variable: ξ(α) = ψ(Kα + 1) − ψ(α + 1) ∈ (0, ln K), the a-priori expected entropy; NSB prior is flat in ξ.
  - Scheme: 64-node Gauss–Legendre on ξ ∈ (δ, ln K − δ) (δ = 1e-8·ln K). Hardcode the 64 GL nodes/weights for [−1,1] (commit constants) or generate them once by Newton on Legendre P₆₄ (~30 LOC, no deps — recommended, avoids a 128-constant table).
  - Per node: invert ξ→α by bisection (ξ is monotone in α; bracket [1e-12, 1e12] in log-α, ~60 iterations, cheap).
  - Log-evidence: `L(α) = lnΓ(Kα) − lnΓ(N + Kα) + Σ_{n_i>0} [lnΓ(n_i + α) − lnΓ(α)]` (sum only over occupied bins + `(K − K_obs)·0`; note lnΓ(n_i+α) − lnΓ(α) for unoccupied bins is 0).
  - Posterior-mean entropy at α: the Ĥ_α formula above.
  - Result: `Ĥ_NSB = Σ w_j e^{L_j − L_max} Ĥ_{α_j} / Σ w_j e^{L_j − L_max}` (log-sum-exp discipline).
  - Ship mean only; variance (needs ⟨H²⟩ with trigamma terms) explicitly out of scope v1 — document. If quadrature validation against known references fails to converge in testing, the documented fallback is to expose it as `nsb_approx` with the caveat; do **not** silently substitute another estimator.
- **James–Stein (Hausser–Strimmer)**: target `t_i = 1/K`;
  `λ̂ = (1 − Σ p̂_i²) / ( (N−1) · Σ (t_i − p̂_i)² )`, clamp to [0,1] (guard Σ(t−p̂)² = 0 → λ = 1);
  `p^{JS} = λ̂ t + (1−λ̂) p̂`; entropy = plug-in on p^JS.
- **Jackknife**: `Ĥ_JK = N·Ĥ − ((N−1)/N)·Σ_j Ĥ_{−j}` — computed in O(K), not O(N): leaving out one observation from bin i changes only that bin, so `Σ_j Ĥ_{−j} = Σ_i n_i · Ĥ(counts with n_i−1)`, and each `Ĥ_{−i}` differs from Ĥ by two terms — maintain the base sum and adjust.

LOC ~350 (NSB is ~150 of it). All O(K) or O(K·quad_order). Oracles: uniform counts → all estimators ≥ MLE = ln K with corrections shrinking as N grows; analytic MM correction check on hand-computed 3-bin example; **cross-validation constants**: run R `entropy` package / Python `ndd`/`pyentropy` offline once, commit golden values for a fixed count vector (e.g. counts `[10,7,5,2,1,1,0,0]`) for MM, CS, JS, SG, Grassberger, NSB with tolerance 1e-10 (1e-6 for NSB); sampled-from-known-distribution bias ordering test (CS and NSB less biased than MLE at N ≪ K).

---

## 13. Spacing estimators (`spacing.rs`)

Sorted sample x_(1) ≤ … ≤ x_(N); m-spacing Δᵢ = x_(i+m) − x_(i−m) with indices clamped to [1, N].

- **Vasicek**: `Ĥ_V = (1/N) Σ_{i=1}^{N} ln( (N/(2m)) · (x_(i+m) − x_(i−m)) )`.
  Known bias: `bias = ln(2m/N)`-type edge effects; the standard additive correction (van Es / documented in Beirlant et al. survey) is
  `Ĥ_V^corr = Ĥ_V + ln(2m/N)·(2m/N)... ` — rather than transcribing the messy constant, implement the correction as Ebrahimi's weighting (below), which subsumes it; expose raw Vasicek for compatibility.
- **Ebrahimi**: `Ĥ_E = (1/N) Σ ln( N·Δᵢ / (cᵢ·m) )` with
  `cᵢ = 1 + (i−1)/m` for i ≤ m; `cᵢ = 2` for m < i ≤ N−m; `cᵢ = 1 + (N−i)/m` for i > N−m.
- **Correa**: `Ĥ_C = −(1/N) Σ ln( [Σ_{j=i−m}^{i+m} (x_(j) − x̄ᵢ)(j − i)] / [N · Σ_{j=i−m}^{i+m} (x_(j) − x̄ᵢ)²] )`, x̄ᵢ = local mean of the window, window clamped at edges.
- **m selection**: default `m = round(√N)` clamped to `[1, N/2 − 1]` (standard consistency condition m→∞, m/N→0; √N is the conventional practical rule — document, allow override).
- **Ties**: zero spacings → `ln 0 = −inf`. Policy: if any Δᵢ = 0, return `TieError` with a hint to jitter (consistent with item 7); alternatively caller-opt-in `jitter(seed)`.

LOC ~120. O(N log N) (the sort dominates). Oracles: U(0,1), N = 10⁴ → H ≈ 0 (tol 0.02); N(0,1) → 1.41894 (tol 0.03); exponential(1) → 1; ordering Ĥ_E less biased than Ĥ_V on small-N uniform (statistical test with fixed seed).

---

## 14. KDE entropy (`kde.rs`)

Gaussian product kernel, diagonal bandwidth:
```
h_j (Silverman) = σ̂_j · (4/((d+2)·N))^{1/(d+4)}     [default]
h_j (Scott)     = σ̂_j · N^{−1/(d+4)}
σ̂_j = min(std_j, IQR_j/1.349)   // robust variant, guard 0 → error
```
**Recommend leave-one-out resubstitution** for the entropy plug-in: the plain resubstitution `−(1/N)Σ ln f̂(xᵢ)` includes the self-term `K(0)/(N·Πh)` which biases Ĥ downward by O(ln N / N) *systematically*; LOO removes it at zero extra asymptotic cost:
```
ln f̂_{−i}(xᵢ) = logsumexp_{j≠i} ( −½ Σ_l ((x_{il} − x_{jl})/h_l)² )
              − ln(N−1) − Σ_l ln h_l − (d/2)·ln(2π)
Ĥ_KDE = −(1/N) Σ_i ln f̂_{−i}(xᵢ)
```
Use log-sum-exp (item 2) — raw exp underflows for isolated points and yields `ln 0`. O(N²·d); document the quadratic cost and cap with a warning-free hard error above N ~ 2·10⁴ default (configurable). Optional cheap speedup: skip pairs with any per-axis gap > 8·h_l (exact to < 1e-14 relative).
LOC ~90. Oracles: N(0,σ²) → ½ln(2πeσ²) (N = 5000, tol 0.02); LOO vs resubstitution bias sign check (resub < LOO on same data); d = 2 independent Gaussians → sum of marginal entropies (tol 0.05).

---

## 15. Permutation / ordinal machinery (`ordinal.rs`)

Pattern of window `(x_t, x_{t+τ}, …, x_{t+(m−1)τ})`: the permutation given by argsort. **Lehmer encoding**:
```
rank(π) = Σ_{i=0}^{m−1} l_i · (m−1−i)!,   l_i = #{ j > i : π_j < π_i }
```
Capacity: m! patterns — 8! = 40320 (fits u16), 12! ≈ 4.79e8 (u32), 20! ≈ 2.43e18 (u64). So m ≤ 8 comfortably fits u32 (recommend `u32` codes with a compile-time factorial table up to 20). O(m²) per window naively — fine for m ≤ 8; no BIT needed.

Tie strategies (enum, default first): 
1. `Stable` — order of occurrence (stable argsort; Bandt–Pompe original; introduces spurious determinism for heavily quantized data — document);
2. `RandomBreak(seed)` — permute tied indices with the seeded RNG;
3. `Jitter(seed)` — amplitude noise 1e-10·std before patterning.

Permutation entropy: `H = −Σ p(π) ln p(π)`, normalized variant `/ ln(m!)`.
**Weighted PE** (Fadlallah 2013): weight per window `w_t = (1/m) Σ_k (x_{t+kτ} − x̄_t)²` (window variance); `p_w(π) = Σ_{t: π_t = π} w_t / Σ_t w_t`.
**Amplitude-aware PE** (Azami–Escudero 2016): `w_t = (A/m) Σ_{k=1}^{m} |x_{t+ (k−1)τ}| + ((1−A)/(m−1)) Σ_{k=2}^{m} |x_{t+(k−1)τ} − x_{t+(k−2)τ}|`, A ∈ [0,1] default 0.5.
LOC ~130. Oracles: monotone series → single pattern, H = 0; iid uniform, m = 3, large n → H → ln 6; encode/decode roundtrip of all 40320 m=8 permutations; committed values vs `ordpy`/`antropy` for a fixed logistic-map series.

---

## 16. Lempel–Ziv & compressor for NCD (`lz.rs`)

### 16.1 LZ76 complexity — Kaspar–Schuster exact algorithm
Over symbol slice s[0..n]:
```
c = 1; i = 0; k = 1; kmax = 1; l = 1
loop:
  if s[i + k − 1] == s[l + k − 1] { k += 1; if l + k > n { c += 1; break } }
  else { kmax = max(kmax, k); i += 1
         if i == l { c += 1; l += kmax; if l + 1 > n { break }
                     i = 0; k = 1; kmax = 1 }
         else { k = 1 } }
return c
```
(Kaspar & Schuster 1987, Fig. 1 — transcribe carefully; this is the #1 off-by-one hazard in the crate. O(n²) worst case, near-linear typical.) Normalization: `C_norm = c(n) · log_a(n) / n`, a = alphabet size; for random sequences C_norm → 1.

### 16.2 LZ78 phrase counting
HashMap<(u32 parent, u8 sym), u32> dictionary; walk the input extending the current phrase until unseen, emit phrase, insert, reset. Complexity count = #phrases; same normalization.

### 16.3 Built-in compressor for NCD — LZW behind a trait
```rust
pub trait Compressor { fn compressed_size_bits(&self, data: &[u8]) -> u64; }
pub struct Lzw { max_code_bits: u8 /* = 16 */ }
```
LZW with growing code width (start 9 bits after 256 initial + reserved codes, bump at dict sizes 512/1024/…, freeze at 2¹⁶ — freezing rather than resetting keeps `C(xy) ≤ C(x) + C(y)` roughly subadditive, which NCD semantics rely on). Return size in *bits* (`Σ current_code_width`) — byte rounding adds quantization noise to NCD at short lengths.
`NCD(x,y) = (C(xy) − min(C(x), C(y))) / max(C(x), C(y))` with `xy` = concatenation; document that LZW-NCD is a weaker oracle than bzip2-based NCD but internally consistent.
LOC: LZ76 ~40, LZ78 ~40, LZW ~90, NCD glue ~20 → **~190**. Oracles: KS-paper example `0001101001000101` → c = 8; constant sequence → c = 2; random binary n = 10⁵ → C_norm ∈ [0.9, 1.1]; NCD(x,x) small (< ~0.1), NCD(x, unrelated) near 1, symmetry |NCD(x,y) − NCD(y,x)| small.

---

## 17. Partial information decomposition (`pid.rs`)

**Scope decision: Williams–Beer I_min on the redundancy lattice for 2 and 3 sources; skip BROJA-2PID.** Rationale to document: BROJA's I_∩ requires minimizing MI over a polytope of joint distributions with fixed pairwise marginals — a constrained convex program whose zero-dep implementation (projected gradient / ADMM with per-iteration KL projections) has fragile step-size/stopping behavior and no cheap certificate of convergence; getting it silently wrong is worse than not shipping it. W–B I_min is exactly computable from counts, nonnegative, and the historical baseline; note its known criticism (can overstate redundancy, e.g. two independent sources copying distinct target bits) in docs. Optionally also ship `I_mmi(α) = min_{A∈α} I(S; A)` as a one-line alternative redundancy for comparison.

Machinery:
- **Specific information**: `I(S = s; A) = Σ_a p(a|s) · ln( p(s|a) / p(s) )`.
- **Redundancy**: `I_min(S; α) = Σ_s p(s) · min_{A ∈ α} I(S = s; A)` for antichain α of source subsets.
- **Lattice**: nodes = antichains of nonempty subsets of sources under set inclusion. n = 2: 4 nodes `{1}{2} ⪯ {1},{2} ⪯ {12}`. n = 3: 18 nodes (hardcoding is acceptable, but generic construction is ~40 LOC: enumerate all families of nonempty subsets (2⁷ subsets → filter to antichains by pairwise non-inclusion; 3 sources → 7 subsets → 2⁷ = 128 families to test), dedupe, order by `α ⪯ β ⇔ ∀B∈β ∃A∈α: A ⊆ B`). Assert count == 18 for n=3 in a test.
- **Möbius inversion** (bottom-up over the down-sets): `PI(α) = I_min(α) − Σ_{β ≺ α} PI(β)` where the sum is over the strict down-set (all β strictly below α, not just covers). Compute nodes in a topological (rank) order.
- Sanity identities to assert: `Σ_{α} PI(α) = I(S; A_1…A_n)` (top node consistency), 2-source: `PI({1}) + PI({1}{2}) = I(S; A_1)`.

Input: joint counts/probabilities over (S, A_1..A_n) with small finite alphabets. LOC ~220. O(|lattice|·|α|·states). Oracles (2-source, binary, uniform inputs): **XOR** target → synergy = ln 2, others 0; **AND** → redundancy ≈ 0.311 bits = 0.2158 nats (I_min value 3/4·ln(4/3)+…: use committed value 0.31128 bits), unique = 0, synergy ≈ 0.5 bits − redundancy adjustments (commit exact values from the `dit` Python package); **COPY(S=A₁)** → unique₁ = H(S), everything else 0.

---

## 18. Transfer-entropy embedding machinery (`embed.rs`)

- **Delay embedding**: `E(x, m, τ)[t] = [x_t, x_{t−τ}, …, x_{t−(m−1)τ}]` — return a strided view descriptor (start index, m, τ) rather than materializing when feeding the k-d tree (tree copies anyway; materialize into row-major `Vec<f64>` is fine and simpler — do that).
- **Alignment**: TE_{Y→X} with history lengths (k for X, l for Y), delays (τ_x, τ_y), prediction horizon u (default 1): valid t range starts at `max((k−1)τ_x, (l−1)τ_y)`, target sample `x_{t+u}`. One shared `align_te(...) -> (targets, x_hist, y_hist)` used by all three TE flavors; ragged (per-variable m, τ) supported by taking slices of per-variable specs.
- **Discrete TE**: `TE = Σ p(x⁺, X_hist, Y_hist) ln [ p(x⁺|X_hist,Y_hist) / p(x⁺|X_hist) ]`, computed from four joint-count tables (or one table + marginalization). Joint state key: **mixed-radix packing** `key = Σ s_i · Π_{j<i} r_j` with **overflow guard**: compute `Π r_j` in u128; if it exceeds `u64::MAX` (or exceeds a configurable dense-table cap ~2²⁴ for the Vec-indexed fast path), fall back to `HashMap<Box<[u16]>, u64>` counting (std hasher; hand-rolled FNV-1a only if profiling demands). This guard is mandatory — alphabet 10, k = l = 8 already overflows u64 territory when combined.
- **Symbolic TE**: ordinal-encode (item 15) each window, then discrete TE with radix m!.
- **Continuous (KSG) TE**: `TE = I_KSG(x⁺ ; Y_hist | X_hist)` via the CMI formula in item 7 — pure composition, no new math.
- **Significance**: permutation surrogates shuffle Y (block shuffle or Fisher–Yates on source history windows) with seeded RNG; p-value = rank of observed TE among surrogates.

LOC ~200. Oracles: coupled binary Markov chain `x_{t+1} = y_t` (copy with noise ε): analytic TE = H_b(ε-flipped channel) values; TE(Y→X) > 0, TE(X→Y) ≈ 0 for unidirectional coupling; symbolic TE on coupled logistic maps reproduces direction; discrete TE via dense table == hash-map fallback exactly (property test across the cap boundary).

---

## 19. GLCM / image texture (`glcm.rs`)

- **GLCM**: quantize image to G levels (min-max or explicit range; G ∈ {8,16,32,64,256}); for each offset (dx,dy) accumulate `C[g1][g2] += 1` over valid pixel pairs; symmetrize `C ← C + Cᵀ` (flag, default true); normalize to p. Standard offsets: (1,0),(1,1),(0,1),(−1,1) at distance δ. GLCM entropy `−Σ xlogx(p_ij)`; also return the p-matrix so contrast/energy/homogeneity are one map away. O(H·W·offsets), LOC ~80.
- **Local windowed entropy — recommend the sliding histogram with incremental Σn·ln n (not integral histogram)**:
  Maintain per-window histogram `h[G]` and scalar `S = Σ h_g·ln h_g`; window entropy `H = ln W − S/W` (W = pixel count). Row-major sweep, boustrophedon (serpentine) traversal; moving one column adds/removes `win_h` pixels: for each count change n→n′, `S += n′ln n′ − n ln n` via a precomputed `k·ln k` table (k ≤ W). Amortized O(win_h) per pixel, O(G) memory.
  Integral histogram is O(G) per pixel *but* O(H·W·G) memory (a 1024² image at G = 64 → 256 MB as u32) and only wins when many window sizes are queried on the same image — document as a non-goal.
  Border policy: `Valid` (output shrinks) default; `Reflect` optional.
LOC ~140 total. Oracles: constant image → GLCM entropy 0, local entropy 0; checkerboard with offset (1,0) → two off-diagonal cells at ½ → entropy ln 2; sliding-window result equals brute-force per-pixel histogram recompute on a 32×32 random image (exact equality — integer histograms).

---

## 20. Graph spectral entropy (`graph.rs`)

- **Laplacian from edge list**: dense `Mat` (n ≤ ~2000 given O(n³) Jacobi — document); `L = D − A`, weighted edges supported, ignore self-loops (or fold into D — pick ignore, document). Normalized variant `L_sym = I − D^{−1/2} A D^{−1/2}` optional.
- **Von Neumann graph entropy**: `ρ = L / tr(L)` (tr(L) = Σ degrees = 2·Σw for undirected); Jacobi eigenvalues λ̃_i of ρ (all ≥ 0 up to roundoff — apply item-21 clipping); `S = −Σ_{λ̃ > tol} λ̃ ln λ̃`.
- **Random-walk entropy rate**: `h = Σ_i π_i · H(P_{i·})`, `P = D^{−1}A`, `H(P_{i·}) = −Σ_j xlogx(P_ij)`.
  - Undirected connected graphs: **closed form** `π_i = d_i / Σ_j d_j` — use it, no iteration (assert `πP = π` as a debug check).
  - Directed/general chains: power iteration on the **lazy** chain `P′ = (P + I)/2` (kills periodicity, same stationary π): `π ← πP′`, L1-normalize, converge when `‖π_{t+1} − π_t‖₁ < 1e-13`, max 10⁵ iterations → error `NotErgodic` (reducible chains legitimately fail — detect via zero entries persisting and report). Entropy rate uses the *original* P rows.
LOC ~130 (+ reuses eigen). Oracles: complete graph K_n von Neumann entropy = ln(n−1) (K_n: ρ has n−1 equal eigenvalues); star vs path ordering; random-walk entropy rate of the complete graph = ln(n−1); 2-cycle directed chain converges under lazy iteration to π = [½, ½], rate = 0.

---

## 21. Numerical hygiene policy (`hygiene.rs` + crate-wide error enum)

Centralize as named constants with doc comments — these are API-visible semantics:

- **Negative-eigenvalue clipping** (density matrices, covariances): `tol_neg = n · eps · max(|λ|)`. λ ∈ [−tol_neg, 0) → clip to 0; λ < −tol_neg → `Err(NotPositiveSemidefinite)` (a genuinely indefinite input is a caller bug, not noise).
- **Probability renormalization**: `|Σp − 1| ≤ 1e-8` → renormalize silently (this covers f32-sourced and serialization-roundtripped inputs); larger → `Err(NotNormalized { sum })`. Negative p: < −1e-12 → error; tiny negative → clamp 0 then renormalize. Constants overridable via a `Tolerances` struct threaded through config, defaulted.
- **Support-mismatch ε-smoothing for divergences** (KL, JS-components, Rényi): default is *mathematical honesty* — `KL(p‖q)` with `p_i > 0, q_i = 0` returns `f64::INFINITY` (not an error). Opt-in smoothing enum: `Smoothing::None | Additive(ε)` (add ε to every q-cell, renormalize; document that it changes the measure) `| Jeffreys(λ)` (mix q ← (1−λ)q + λ·uniform). Never smooth by default.
- **Tiny negative entropy estimates**: discrete estimators are ≥ 0 in exact arithmetic — clamp results in `[−1e-12·ln K, 0)` to 0; more negative → debug-assert (indicates a bug). **Never clamp differential/KL/spacing estimators** — negative values are legitimate there.
- **Overflow-safe bin/state products**: all radix products via `u128` `checked_mul`; exceeding the dense cap → hash fallback (item 18); exceeding u128 → hash fallback unconditionally. Bin *counts* as u64 (never usize-assumed-64 in arithmetic that could run on 32-bit… the workspace is 64-bit, but `u64` explicitly anyway).
- **NaN policy**: every public entry point rejects NaN inputs up front (`Err(NonFinite)`); internal code may then use `total_cmp`/`sort_unstable` freely.

LOC ~60 + the crate `Error` enum.

---

## Riskiest items & de-risking

1. **NSB quadrature (item 12)** — most numerically subtle: evidence spans hundreds of log-units, ξ↔α inversion, GL nodes. De-risk: build in strict order lnΓ→digamma→SG-Bayes formula (each tested) so NSB is pure composition; validate against the Python `ndd` package on 5 committed count vectors before wiring into the public API; keep it feature-gated in review until oracles pass; ship mean-only.
2. **KSG family conventions (item 7)** — strict-vs-nonstrict counts, +1s in digamma arguments, ε as radius vs diameter: silent O(1/k) biases that unit tests on Gaussians barely detect. De-risk: implement a brute-force O(n²) reference for neighbor counts (exact equality test vs tree), and validate KSG-1/KSG-2 against JIDT/NPEET golden values on a committed 500-point correlated-Gaussian dataset with fixed jitterless data, tolerance 1e-12 (same-algorithm determinism, not statistical tolerance).
3. **Bluestein + real-FFT unpacking (item 3)** — index-mirroring and chirp-angle precision bugs corrupt everything downstream (surrogates, Welch). De-risk: keep a 15-line naive O(n²) DFT in the test tree as the universal oracle; property-test every n ∈ 1..=257 plus a few large primes; test the integer-mod chirp angle at n = 10⁶.
4. **PID lattice + Möbius (item 17)** — the 3-source 18-node ordering and strict-down-set inversion are easy to get subtly wrong. De-risk: assert node count = 18, assert the sum identity `ΣPI = I(S; all)` on random distributions (property test), and pin XOR/AND/COPY gate values against the `dit` package.
5. **Kaspar–Schuster LZ76 (item 16)** — a 12-line algorithm with three interacting indices; published pseudocode variants disagree at sequence end. De-risk: brute-force exhaustive-history reference implementation for all binary strings of length ≤ 12 (exact-match property test), plus the canonical `0001101001000101 → 8` value.

**Total estimated implementation: ~3,600 LOC** (excluding tests; expect ~1:1 test ratio given the oracle-heavy strategy). Dependency-ordered build sequence: 1 → 2 → 11(rng) → 3 → 4 → 10 → 9 → 8 → {5, 6, 7, 12–21 in any order; 18 after 7+15; 11(surrogates) after 3}.

### Critical Files for Implementation
- /Users/ueli/Documents/semio/Cargo.toml (add `"mathematical/entropy/rs"` to workspace members)
- /Users/ueli/Documents/semio/mathematical/entropy/rs/Cargo.toml (new crate manifest, mirror `mathematical/random/rs` conventions: `name = "mathematical_entropy"`, `edition = "2021"`, `rust-version = "1.88"`, `[lints] workspace = true`, `path = "lib.rs"`)
- /Users/ueli/Documents/semio/mathematical/entropy/rs/lib.rs (crate root declaring the 21 modules above)
- /Users/ueli/Documents/semio/mathematical/entropy/rs/special.rs (item 1 — everything downstream depends on lnΓ/ψ; build first)
- /Users/ueli/Documents/semio/mathematical/entropy/rs/fft.rs (item 3 — second-largest dependency fan-out: spectral, surrogates, wavelet-adjacent convolution)

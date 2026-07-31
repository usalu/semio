# Plan: `mathematical_entropy` — feature-complete headless entropy & information-theory crate

## Context

The user wants a sophisticated, feature-complete, headless and stateful Rust crate implementing an exhaustive entropy/information-theory feature tree (45 spec sections: discrete/continuous entropies, estimators, divergences, MI, PID, time-series/multiscale/spectral/wavelet entropies, transfer entropy, image/graph/ML-uncertainty domains, streaming, inference). A UI will consume it later.

**User decisions (asked & answered):**
- Location: `mathematical/entropy/` (Cargo package `mathematical_entropy`, nx project `@semio-tech/math-entropy`)
- Scope: **everything including specialized domains** (image, graph, ML, PID, wavelet, matrix). Bindings/CLI crates are NOT in scope (headless library only; repo forbids extra script files anyway).
- Dependencies: **zero external crates** — hand-rolled FFT, kd-tree, eigensolver, special functions, RNG (matches `mathematical/sampling|number|polynomial` which have empty `[dependencies]`).

Existing related code (do NOT duplicate blindly, this crate supersedes for its domain; leave the small existing fns in place): `mathematical/statistics/rs/lib.rs` region `🔖Information` (entropy/MI/CMI over codes), `mathematical/sampling/rs/lib.rs` `entropy_nats`.

**Supporting design docs (read them during implementation):**
- Numerics reference (algorithms, formulas, coefficients, oracles for all 21 infra items): `/private/tmp/claude-501/-Users-ueli-Documents-semio/08945ab6-7b5c-46f8-9eb9-0916daf604ea/scratchpad/numerics-reference.md` — **first step: copy it into the ticket folder** so it survives.

## Ticket workflow (repo MCP)

1. Read MCP resource `repo://goals`, pick the most appropriate goal for association (no perfect fit exists — likely a `MATHEMATICAL`-adjacent or general goal; do NOT open/create goals).
2. `ticket_open` a new ticket, e.g. title "Feature Complete Mathematical Entropy Crate" (titleized). All temp files/logs go in the ticket folder; copy `numerics-reference.md` there.
3. On completion: `ticket_close` **with explicit ticket path** (never path-less) + summary + all touched files.

## Bundle scaffolding (mirror `mathematical/polynomial/`)

| File | Content |
|---|---|
| `Cargo.toml` (root, `/Users/ueli/Documents/semio/Cargo.toml`) | Add `"mathematical/entropy/rs",` to `members` after line 58 (`"mathematical/sampling/rs",`) |
| `mathematical/entropy/package.json` | `{ "name": "@semio-tech/math-entropy", "private": true, "type": "module", "scripts": { "test": "nx run @semio-tech/math-entropy:test" } }` |
| `mathematical/entropy/project.json` | Clone of `mathematical/polynomial/project.json` with name `@semio-tech/math-entropy`, cwd `mathematical/entropy`; targets `test`, `test-quick`, `test-long`, `test-exhaustive`, `lint`, each `bun ./script.ts …` |
| `mathematical/entropy/script.ts` | Clone of `mathematical/polynomial/script.ts` (20 lines): docstring `/** 🌀 \`@semio-tech/math-entropy\` — … */`, package list `["mathematical_entropy"]`, imports from `../../repo/lib/js/index.ts` |
| `mathematical/entropy/rs/Cargo.toml` | `name = "mathematical_entropy"`, `version = "0.1.0"`, `edition = "2021"`, `rust-version = "1.88"`, `[lints] workspace = true`, `[lib] crate-type = ["rlib"], path = "lib.rs"`, **empty `[dependencies]`** |
| `mathematical/entropy/rs/lib.rs` | Crate root (see below) |
| `mathematical/entropy/rs/src/*.rs` | 27 modules (see below) |
| `.vscode/launch.json` | Add `{ "name": "🧪test🌀mathematical-entropy", "type": "node-terminal", "request": "launch", "command": "bun nx run @semio-tech/math-entropy:test", "cwd": "${workspaceFolder}", "presentation": { "group": "3_dev", "order": 391.79 } }` right after `🧪test🎰mathematical-sampling` (order 391.78, ~line 2832) |

Do NOT touch root `package.json` workspaces (no JS package shipped). No modifying git commands, no worktrees, no new files outside the bundle + ticket folder.

## Crate architecture

### lib.rs (crate root)

Crate docstring `//! 🌀 Zero-dependency information theory: entropies, estimators, divergences, mutual information, information dynamics, and streaming state.` Regions:

- `// #region 🔖Errors` — `EntropyError`: flat hand-written enum (no thiserror), `Clone + PartialEq + Debug`, manual `Display` + `std::error::Error` (pattern: `mathematical/sampling/rs/lib.rs` `SamplingError`). Variants: `InvalidConfig{field,reason}`, `EmptyInput{what}`, `LengthMismatch{expected,actual}`, `ShapeMismatch{what,expected,actual}`, `NonFinite{what,index}`, `InvalidProbability{index,value}`, `NotNormalized{sum}`, `InsufficientData{what,needed,actual}`, `UndefinedResult{reason}`, `DegenerateInput{what}`, `NotConverged{what,iterations}`, `UnknownFeature{name}`.
- `// #region 🔖Units` — `LogBase { Nats, Bits, Hartleys, Base(f64) }` with `ln()`, `from_nats()`, `to_nats()`, `convert()`. **All internal math in nats; convert once at API boundary.**
- `// #region 🔖Estimate` — `Estimate { value, base, method: &'static str, n, n_effective, std_error: Option<f64>, ci: Option<ConfidenceInterval>, warnings: Vec<Warning>, diagnostics: Vec<(&'static str, f64)> }` + `in_base()/bits()/nats()`; `ConfidenceInterval{lower,upper,level}`; `Warning` enum (`SmallSample`, `Undersampled`, `ClippedNegative`, `TiesBroken`, `NotConvergedSoft`, `SurrogatesInconclusive`).
- `// #region 🔖Policies` — shared config atoms: `MissingPolicy{Error,Skip,PairwiseSkip}`, `Tolerance{Absolute,RelativeToSd,Auto}`, `BinsSpec{Fixed,Sturges,Scott,FreedmanDiaconis,Doane,Edges}`, `Metric{Chebyshev,Euclidean,Manhattan}` (enum, match-dispatch — not a trait), `TiePolicy`, `Smoothing{None,Additive(f64),Jeffreys(f64)}`, `Tolerances` (overridable hygiene constants).
- `// #region 🔖Modules` — `#[path = "src/x.rs"] pub mod x;` × 27.
- `// #region 🔖Exports` — flat `pub use` of the entire stable surface (spec §44 + configs + traits + fitted structs).

**API rule:** `f64`-returning fns for exact quantities on *given distributions*; `Result<Estimate, EntropyError>` for anything *estimated from data*. Config pattern: immutable structs with validated `new(...) -> Result<Self, _>` constructors, `Default` with literature defaults, consuming `with_*` for optionals — no mutable builders. Every pub item gets an emoji docstring; docstrings link references (papers/Wikipedia) in rustdoc format.

### Traits (exactly four; everything else plain fns, enums, or fitted structs)

```rust
pub trait StreamingEstimator { type Item;
  fn update(&mut self, x: Self::Item);
  fn remove(&mut self, x: Self::Item) -> Result<(), EntropyError>; // Err where unsupported (decay)
  fn merge(&mut self, other: &Self) -> Result<(), EntropyError>;
  fn estimate(&self) -> Result<Estimate, EntropyError>;
  fn reset(&mut self);
  fn snapshot(&self) -> StreamingSnapshot;                          // plain Vec-based state, no serde
  fn restore(s: &StreamingSnapshot) -> Result<Self, EntropyError> where Self: Sized; }
pub trait Symbolizer { fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError>; fn alphabet_size(&self) -> usize; }
pub trait Compressor { fn compressed_len(&self, data: &[u8]) -> usize; }         // + built-in Lz78Compressor, ncd()
// DistanceMetric stays the closed `Metric` enum. Bregman divergence takes closures.
```

Stateful fitted-estimator pattern (fit once, query many): `KdeDensity::fit`, `MarkovChain::fit` (order-k, stationary(), entropy_rate()), `Dwt` (fitted filter bank), `FeatureRegistry` (deterministic ordered named features, `standard()` + `compute(&[f64]) -> Vec<Feature>`), streaming impls `StreamingCounts`, `SlidingWindowEntropy`, `DecayedEntropy`.

### src modules (27, dependency-ordered)

| File | Responsibility |
|---|---|
| `numeric.rs` | Special functions (Lanczos ln_gamma g=7, digamma, trigamma, erf/erfc, normal CDF + Acklam inverse, incomplete gamma, log-factorial cache), Neumaier & pairwise summation, stable `x·ln x`, log-sum-exp, hygiene constants (clipping/renormalization tolerances, overflow-safe u128 products), `pub(crate) Xorshift64` (xorshift64* + splitmix64 seeding, Fisher–Yates) |
| `counts.rs` | Symbol/frequency counting, probability validation/normalization, `Counts`/`JointCounts` contingency tables, alphabet mapping, weighted counts (`n_effective = (Σw)²/Σw²`), smoothing (Laplace/Lidstone/Jeffreys/Dirichlet), missing-value policy resolution |
| `discrete.rs` | Plug-in entropies on distributions/counts: Shannon, joint/conditional/cross, Rényi (α→limits: Hartley/Shannon/collision/min), Tsallis, Sharma–Mittal, Kaniadakis, cumulative-residual, normalized variants, effective numbers |
| `estimators.rs` | Bias-corrected discrete: Miller–Madow, Grassberger, delete-one/d jackknife, Chao–Shen, Schürmann–Grassberger, Bayesian-Dirichlet, NSB (Gauss–Legendre over transformed ξ), James–Stein shrinkage; `DiscreteMethod` enum dispatch; estimator diagnostics (coverage, singletons, sparsity) |
| `knn.rs` | k-d tree over row-major `&[f64]` points (sort-based build, max-heap kNN, radius counts), `Metric` evaluation, brute-force reference (test oracle) |
| `continuous.rs` | Differential entropy: histogram, Gaussian/Epanechnikov KDE (Silverman/Scott bandwidth, `KdeDensity`), Kozachenko–Leonenko kNN, Vasicek/Correa/Ebrahimi m-spacing, Gaussian & multivariate-Gaussian closed forms, uniform/exponential closed forms, copula entropy (rank transform) |
| `divergence.rs` | KL (fwd/rev/symmetrized/Jeffreys), JS (+weighted/generalized/distance), Rényi/Tsallis/α-β-γ divergences, Hellinger, Bhattacharyya, total variation, χ² family, Wasserstein-1D (sorted), energy distance, KS/Cramér–von Mises, Itakura–Saito, log-det (Gaussian), closure-based Bregman; `Smoothing` support policy (∞ by default, never smooth silently) |
| `mutual.rs` | Discrete MI/CMI/PMI/local MI, KSG-1/KSG-2 continuous MI, Ross-style mixed discrete-continuous MI, normalized MI (arithmetic/geometric/min/max/joint, uncertainty coefficients), total correlation, dual total correlation, interaction/co-information, O-/S-information, feature-selection scores (information gain, gain ratio, mRMR ranking) |
| `pid.rs` | Williams–Beer `I_min` redundancy lattice for 2–3 sources (cap n≤4 with `InvalidConfig`), antichain construction, Möbius inversion, atoms (redundancy/unique/synergy), consistency check ΣPI = I(S;T). BROJA deliberately omitted (needs convex solver) — document |
| `fisher.rs` | Fisher information (analytic score closure + numerical derivative), observed/expected/empirical FIM, AIC/AICc/BIC/HQC/MDL |
| `symbolic.rs` | `Symbolizer` trait + `OrdinalSymbolizer` (Lehmer-code pattern ranking, m ≤ 8 in u32, tie policies), `DispersionSymbolizer` (NCDF/linear/quantile/sigmoid class maps), `QuantileSymbolizer`, `ThresholdSymbolizer`, SAX-style, delay embedding `embed(x, dim, tau)`, mixed-radix joint-symbol packing (u128 checked, hash fallback) |
| `regularity.rs` | ApEn, SampEn, quadratic/range variants, FuzzyEn (exp/Gaussian/sigmoid membership), cross-ApEn/SampEn/Fuzzy, tolerance auto-selection (0.2·sd), multivariate variants |
| `ordinal.rs` | Permutation entropy (+ weighted, amplitude-aware, modified, reverse, conditional, cross), dispersion entropy (+ fluctuation, reverse), distribution entropy, increment, bubble, attention, slope, phase (analytic-signal via FFT Hilbert), symbolic-dynamic/transition entropy, block-entropy rate & excess entropy/predictive information |
| `markov.rs` | `MarkovChain::fit` (order-k transitions), stationary distribution (power iteration with convergence criterion), analytic & estimated entropy rate, reversibility/entropy production |
| `multiscale.rs` | Coarse-graining (mean/median/variance/sd/moving-avg/decimation/composite offsets), MSE/CMSE/RCMSE, modified/generalized MSE, hierarchical decomposition, multivariate multiscale, `MultiscaleResult{per_scale, complexity_index, scales}` + curve summaries (AUC/slope/max-scale) |
| `lz.rs` | LZ76 (Kaspar–Schuster), LZ78, normalized LZ `C(n)·log_a n / n`, permutation-LZ, joint LZ, `Compressor` trait + `Lz78Compressor` (u16 codes), NCD/NCS |
| `fft.rs` | Iterative radix-2 complex FFT + real-FFT packing + Bluestein (arbitrary n), twiddle/plan cache struct, naive DFT kept in test tree as oracle; window functions (rect/Hann/Hamming/Blackman/Blackman–Harris/Kaiser via I0 series/Tukey) |
| `spectral.rs` | Periodogram, Welch (overlap + window-power normalization, one-sided scaling), sine-taper multitaper (no DPSS eigensolver needed), Shannon/Rényi/Tsallis spectral entropy, normalized & band-limited variants, spectral flatness, STFT spectrogram entropy (per-frame/per-frequency/global) |
| `wavelet.rs` | DWT filter banks: Haar, Daubechies D4–D8, symlets (coefficients from numerics doc), stationary WT, wavelet packets, boundary modes (zero/periodic/symmetric), relative wavelet energy + Shannon/Rényi/Tsallis wavelet & packet entropy, `Dwt` fitted struct |
| `matrix.rs` | Cyclic Jacobi symmetric eigensolver, one-sided-Jacobi SVD, Cholesky + log-det (regularization fallback), covariance/correlation kit; SVD entropy, effective/stable rank, eigenvalue entropy, von Neumann density-matrix entropy (negative-eigenvalue clipping `n·eps·max|λ|`), matrix-based Rényi |
| `inference.rs` | Bootstrap (percentile/basic/BCa) & jackknife CIs, permutation/block-permutation tests, surrogates (circular-shift, block-shuffle, phase-randomized via FFT, IAAFT), multiple-testing (Bonferroni/Holm/BH/BY), `SurrogateConfig{kind,count,seed}` — mandatory explicit seed, fully deterministic |
| `transfer.rs` | Discrete/symbolic/kNN(KSG-CMI) transfer entropy, conditional & multivariate TE, effective TE (surrogate baseline), local TE, active information storage (+local), directed information (Massey), embedding selection helpers (Ragwitz-lite, MI-delay) |
| `spatial.rs` | 2D over `&[f64]`/`&[u8]` + width/height: global/histogram entropy, sliding-window local entropy map (ring-buffer histogram, O(1) amortized), GLCM (offsets, symmetrization) + texture entropies, block/patch entropy, entropy-based threshold selection |
| `graph.rs` | Edge-list/adjacency inputs (plain slices): degree/in/out/strength distribution entropy, Laplacian construction, von Neumann graph entropy (reuses `matrix`), random-walk entropy rate `Σ πᵢ H(rowᵢ)` (power-iteration π), partition MI/NMI/adjusted MI/variation of information |
| `ml.rs` | Predictive entropy (batch rows), max-prob/margin/variation ratio, BALD MI & epistemic/aleatoric split from ensemble sample matrices, JS disagreement, vote entropy, log-loss/Brier/ECE/adaptive-ECE, Gaussian regression predictive entropy, information-gain / Gini feature scoring |
| `streaming.rs` | `StreamingEstimator` trait + `StreamingSnapshot`, `StreamingCounts` (exact), `SlidingWindowEntropy` (ring buffer), `DecayedEntropy` (exponential forgetting, `remove` → Err), streaming MI & window-to-window drift (KL/JS/PSI, threshold alerts), merge = distributed reduction |
| `features.rs` | `FeatureRegistry`/`Feature` batch extraction (deterministic ordering, named groups: distributional/temporal/spectral/multiscale), estimator-selection automation: `suggest_bins/k/delay/embedding_dimension/tolerance/scales`, discrete-vs-continuous detection, constant/tie/periodicity detection, explanation strings |

### Key numerics decisions (full detail in numerics-reference.md)

- Lanczos g=7 ln_gamma (~1e-13); digamma via recurrence+asymptotic; Acklam inverse normal CDF.
- FFT: radix-2 + Bluestein (Welch segment lengths are user-chosen — arbitrary n required); naive O(n²) DFT as test oracle for all n ∈ 1..=257.
- KSG: implement brute-force O(n²) neighbor-count reference, assert exact equality vs kd-tree; golden values on committed 500-point correlated-Gaussian data (deterministic, tol 1e-12).
- NSB: build strictly on tested lnΓ/digamma; Gauss–Legendre over transformed prior; ship mean-only.
- LZ76: Kaspar–Schuster; exhaustive brute-force check for all binary strings len ≤ 12; canonical `0001101001000101 → 8`.
- PID: assert 3-source lattice = 18 nodes; ΣPI = I identity property test; XOR ⇒ synergy = 1 bit, COPY ⇒ redundancy.
- Hygiene: silent renormalization iff `|Σp−1| ≤ 1e-8`; KL support mismatch → `∞` (never smooth by default); clamp tiny negative discrete entropies only; NaN rejected at every public entry.

## Testing strategy

Inline `#[cfg(test)] mod tests` at the bottom of each src file; tiers via nested `mod quick`/`mod long`/`mod exhaustive` (fundamental at top level), region-wrapped. Deterministic hand-rolled xorshift with constant seeds (precedent `mathematical/graph/rs/lib.rs`; no proptest). Highlights:

- **Fundamental:** uniform ⇒ `ln k` exactly; fair coin = 1 bit; `KL(p,p)=0`; Rényi α→1 = Shannon; monotone series ⇒ PE = 0; error-path coverage for every `EntropyError` variant; `LogBase` roundtrips; streaming update×n == batch.
- **quick:** Gaussian ½ln(2πeσ²) vs Vasicek/kNN/KDE; MI(X,X)=H(X); independent ⇒ MI≈0; chain rule; KSG vs −½ln(1−ρ²) on bivariate Gaussians; FFT vs naive DFT + Parseval; wavelet perfect reconstruction; Jacobi vs hand 3×3; von Neumann pure=0/mixed=ln n; PID gates; sine ⇒ spectral entropy ≈ 0; streaming merge commutativity; sliding-window vs batch at every step.
- **long:** bias→0 consistency sweeps n ∈ 1e2..1e5; permutation-test false-positive rate ≈ α; effective TE direction on coupled logistic maps; MSE white vs 1/f ordering; bootstrap coverage.
- **exhaustive:** simplex-grid (k≤4, step 0.05) global inequalities (KL≥0, subadditivity, Rényi monotone in α); all binary strings ≤ 12 for LZ76; PID Möbius identity over enumerated gates; relabeling invariance.

## Implementation order (compile + green after each step)

1. Ticket open → scaffold bundle + workspace member + lib.rs core types (errors/units/Estimate/policies) — `cargo check -p mathematical_entropy` passes.
2. `numeric.rs` → `counts.rs` → `discrete.rs` → `estimators.rs` (discrete backbone; first §44 fns live).
3. `knn.rs` → `continuous.rs` → `divergence.rs` → `mutual.rs`.
4. `symbolic.rs` → `regularity.rs` → `ordinal.rs` → `markov.rs`.
5. `inference.rs` (RNG/surrogates/bootstrap — prerequisite for CIs and effective TE).
6. `fft.rs` → `spectral.rs`; `wavelet.rs`; `matrix.rs` (three independent stacks).
7. `multiscale.rs`, `lz.rs`, `transfer.rs`, `pid.rs`, `fisher.rs`.
8. `spatial.rs`, `graph.rs`, `ml.rs` (thin domain adapters).
9. `streaming.rs` → `features.rs` (registry references everything; streaming must match proven batch results).
10. Final: lib.rs `🔖Exports`, docstring/emoji/region audit, launch.json entry, full test + lint run, ticket_close (explicit path).

Note: repo-wide cargo failures may be another session's in-flight refactor — check shared files before assuming breakage is ours (Cargo.lock churn is expected and shared).

## Verification

- `cargo check -p mathematical_entropy` after each step; final: `cargo test -p mathematical_entropy` directly, then `bun nx run @semio-tech/math-entropy:test` (proves script.ts/project.json wiring) and `bun nx run @semio-tech/math-entropy:test-quick`.
- `bun ./mathematical/entropy/script.ts lint` (clippy `-D warnings` gate) must pass clean.
- Runtime confirmation beyond tests: temporary `[DEBUG]`-prefixed prints in one long-tier test run, removed before close.
- Confirm launch.json entry parses (JSONC) and follows order 391.79 in group `3_dev`.

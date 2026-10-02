# 📓️ Work package B (kinematics) — report

Ticket `2026/10/02/QUIZ-PETS`, phase 1 (TypeScript). `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Everything below was run on 2026-10-02 on the Windows host (bun 1.4.2, vitest 4.1.10, `.venv` Python 3.14.4 with numpy 2.4.3; the harness environment runs numpy 2.5.0). Raw outputs: `TK/🗑️generated/wp-b/`.

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/📐️trigonometry/🟦️.ts` | `sinTurns`, `cosTurns`, `clamp`, `lerp`, `smoothstep` (83 lines) |
| `P/🔨️modules/🎲️randomness/🟦️.ts` | `randomWords`, `randomUnit`, `randomBetween`, `randomPick` (109 lines) |
| `P/🔨️modules/🦴️rig/🟦️.ts` | `Affine`, `IDENTITY`, `compose`, `invert`, `transform`, `BonePose`, `Pose`, `restPose`, `solveRig`, `lookOffset` (127 lines) |
| `P/🔨️modules/{📐️trigonometry, 🎲️randomness, 🦴️rig}/🧪️tests/🔬️unit/🟦️.ts` | vitest suites, 60 tests (trigonometry 15, randomness 17, rig 28) |
| `P/🧪️tests/{📐️turn-trigonometry, 🎲️counter-randomness, 🦴️rig-solving, 👀️gaze-tracking}/{🥒️.feature, 🐍️.py, 🟦️.ts}` | four Protocol v2 cases, 20 scenarios (6 + 5 + 6 + 3) |
| `P/🧫️fixtures/<same four names>/🔣️.json` | vectors, generated (151 kB, 17 kB, 109 kB, 9 kB) |
| `TK/generate_kinematics_vectors.py` | the generator named in every feature header (idempotent; also checks the two fixture species against `P/🧬️schema/🔣️.json`) |
| `TK/probe_kinematics.ts`, `TK/probe_kinematics.py` | accuracy and numpy probe (400 922 angles, 520 keys) |
| `TK/mutate_kinematics.ts` | mutation check of the three unit suites (38 mutants, sandbox under `🗑️generated`) |
| `TK/kinematics.tsconfig.json` | strict typecheck of the files of this work package |

The adapters import the module files directly (`../../🔨️modules/<m>/🟦️.ts`). No Rust file was created. No file of another work package or of the quiz product was edited; no git command was run.

### Semantics the siblings can rely on

- **`sinTurns` / `cosTurns`**: exact reduction of the magnitude (`m − floor(m)`), residue from the nearest quarter turn in [−⅛, ⅛) (exact), one rounding by `TAU = 6.283185307179586`, fdlibm kernels (`k_sin.c` S1…S6, `k_cos.c` C1…C6; the twelve literals and `TAU` were checked against fdlibm's published bit patterns). Quarter turns give exactly 0, 1, −1; sine is exactly odd, cosine exactly even; no negative zero is returned (`0 − x` instead of `−x`).
- **`clamp(value, low, high)`** = `min(max(value, low), high)`: when the bounds cross, `high` wins (numpy.clip). **`lerp`** does not clamp. **`smoothstep`** clamps its amount to [0, 1].
- **`randomWords(key, count)`** = `numpy.random.SeedSequence(key).generate_state(count)`; words the key lacks count as 0 up to four (`[5]` ≡ `[5, 0, 0, 0]` ≢ `[5, 0, 0, 0, 0]`); an empty key equals numpy's empty entropy array. **`randomUnit`** ∈ [0, 1), a multiple of 2⁻³². **`randomBetween`** = `low + (high − low) × unit`. **`randomPick`**: first index whose running sum of positive weights exceeds `unit × total`; weights ≤ 0 (and NaN) are never picked; −1 without a positive weight.
- **`compose(parent, local)`** = parent × local (gl-matrix `mat2d.multiply`, bit for bit). **`invert`**: adjugate ÷ determinant; determinant 0 → `IDENTITY` (always finite). **`transform(matrix, x, y)`** → `Point`.
- **`solveRig(species, pose)`**: flat `number[]`, six per bone in rig order; a pose shorter than the rig leaves the remaining bones at rest; a bone whose parent is not listed before it is treated as a root (only documents that fail `bone-order` have such bones). The products are the expressions of `compose`, inlined (no allocation per bone).
- **`lookOffset(eye, target, reach)`** = `(target − eye) ÷ (d + reach)`; `(0, 0)` for coinciding points; a reach ≤ 0 counts as 0 (offset on the rim).

## 2. Verification (commands and real results)

All commands from the repository root unless a `cd` is shown.

| # | Command | Result |
|---|---|---|
| 1 | `.venv/Scripts/python.exe TK/generate_kinematics_vectors.py` | four fixtures written; a second run prints `unchanged` four times |
| 2 | `cd P/📦️packages/🟦️typescript && bun ./📜️script.ts test quick "📐️trigonometry/" "🎲️randomness/" "🦴️rig/" --reporter=verbose` | `Test Files 3 passed (3)`, `Tests 60 passed (60)` |
| 3 | `cd P/📦️packages/🟦️typescript && bun ./📜️script.ts test quick` (every suite of the package at that moment, siblings included) | `Test Files 6 passed (6)`, `Tests 261 passed (261)` |
| 4 | `cd TEST && bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1: `5865 high-priority breach(es) across 4 rule(s)` — the repository-wide backlog (`temp/…`, `✏️s/🔌️plugins/…`); **0** lines name the pets product, and the full breach set `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` has **0** pets entries |
| 5 | `cd TEST && bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "📐️turn-trigonometry"` | `[test] level=exhaustive cases=1 executed=6 passed=6 failed=0 errored=0 parity=0/0` |
| 6 | same, `parity exhaustive … --case "📐️turn-trigonometry"` | `[test] level=exhaustive cases=1 executed=12 passed=12 failed=0 errored=0 parity=6/6` |
| 7 | `oracle exhaustive … --case "🎲️counter-randomness"` | `[test] level=exhaustive cases=1 executed=5 passed=5 failed=0 errored=0 parity=0/0` |
| 8 | `parity exhaustive … --case "🎲️counter-randomness"` | `[test] level=exhaustive cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5` |
| 9 | `oracle exhaustive … --case "🦴️rig-solving"` | `[test] level=exhaustive cases=1 executed=6 passed=6 failed=0 errored=0 parity=0/0` |
| 10 | `parity exhaustive … --case "🦴️rig-solving"` | `[test] level=exhaustive cases=1 executed=12 passed=12 failed=0 errored=0 parity=6/6` |
| 11 | `oracle exhaustive … --case "👀️gaze-tracking"` | `[test] level=exhaustive cases=1 executed=3 passed=3 failed=0 errored=0 parity=0/0` |
| 12 | `parity exhaustive … --case "👀️gaze-tracking"` | `[test] level=exhaustive cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3` |
| 13 | `bun TK/probe_kinematics.ts && .venv/Scripts/python.exe TK/probe_kinematics.py --full` | see §3; `keys=520 randomness mismatches=0` |
| 14 | `bun TK/mutate_kinematics.ts` | `untouched copy: unit suites pass`, `38 of 38 mutants killed` |
| 15 | `node node_modules/typescript/bin/tsc --noEmit -p TK/kinematics.tsconfig.json` (strict + `noUncheckedIndexedAccess` + `noUnusedLocals/Parameters`) | 0 errors in pets or ticket files (the 909 errors it prints are in unrelated framework files that the harness package pulls in under the stricter flags) |
| 16 | `.venv/Scripts/python.exe -X utf8 <QUIZ-PRODUCT-AND-TEACHING-PROCTOR>/domain_docstring_emojis.py <18 files of this work package>` | `0 finding(s) in 18 file(s)` |

Notes on the commands:

- **`--case` takes the directory name with its emoji** (`"📐️turn-trigonometry"`). The bare slug selects nothing and reports `cases=0 … parity=0/0` with exit code 0 — a silent non-run, not a pass.
- Only the TypeScript subject exists; `parity=n/n` is oracle ↔ TypeScript. Under load one parity run took several minutes.
- The fixture species (`blobby` = `TK/reference-species.json`, `lanky` = the same with rest rotations and a four-bone arm chain) pass `jsonschema` Draft 7 against `P/🧬️schema/🔣️.json#/$defs/Species` (the generator refuses to write otherwise) and `bun TK/validate_species.ts` reported `ok` for both.

## 3. Measured accuracy of the trigonometry

`TK/probe_kinematics.py --full`: 400 001 evenly spaced angles over [−8, 8] turns, every eighth of a turn in that range with its three nearest doubles on both sides, tiny (down to 5e-324) and huge (up to 1e300) angles — 400 922 samples.

| Measure | Value |
|---|---|
| max \|sinTurns − numpy.sin(2π·t)\| over [−8, 8] | 5.496e-15 |
| max \|cosTurns − numpy.cos(2π·t)\| over [−8, 8] | 5.447e-15 |
| max \|sinTurns − exact\| (60-digit reference, all 400 922 samples) | 1.564e-16 |
| max \|cosTurns − exact\| | 1.564e-16 |
| max error in units of the last place of the exact value | 1.638 |
| exact zeros (multiples of a quarter turn) | reproduced as 0 |

The requirement was 1e-12; the 5.5e-15 against numpy is numpy's own rounding of `2π·t` before it reduces (≈ 1e-15 × |t|), not an error of the subject — the 60-digit reference shows 1.6e-16. For the same reason the committed angles stay below 130 000 turns, where that reference is still ten times finer than the comparison tolerance of 1e-9. The vitest suite additionally holds 65 537 angles over ±8 turns to gl-matrix's rotation within 1e-12.

## 4. Deviations from the design and decisions taken

1. **Reduction of the magnitude.** §4.1 says `t − floor(t)`. That subtraction rounds for small negative angles (−1e-20 becomes 1.0). The module reduces `|t|` (always exact) and restores the sign, which also makes sine exactly odd and cosine exactly even (a mirrored pet gets mirrored matrices). Consequence: periodicity is exact for non-negative angles and within two units in the last place across the sign change (tested).
2. **"First octant" is the residue from the nearest quarter turn** in [−⅛, ⅛) with a kernel and sign per quarter — the same reflections, expressed as in the research report §2.0. The cosine kernel is the plain form `1 − (z/2 − z²·P(z))` without fdlibm's bit-level split for |x| > 0.3 (that split needs word access to the double; accuracy without it is 1.6 units in the last place).
3. **`clamp` with crossed bounds, `smoothstep` clamping, `invert` of a singular matrix, `lookOffset` with a reach ≤ 0, short poses and unlisted parents in `solveRig`**: not specified by the design; decided as listed in §1 so that every function is total and finite, and every one of these is a committed vector.
4. **Angles are committed as integer ratios, all other inputs as integers or short decimals**, so that every JSON reader (including serde_json without `float_roundtrip`) forms the same doubles.
5. **A third scenario kind, `bit-patterns`** (in `📐️turn-trigonometry`, `🦴️rig-solving`, `👀️gaze-tracking`): the profile `pets-float-v1` (1e-9) cannot show that the twins agree bit for bit (§2.4). The oracle adapters therefore also restate the subject's arithmetic in Python's IEEE doubles, hold that restatement to numpy within 1e-12, and project the 64-bit pattern of every result as sixteen hexadecimal digits (strings compare exactly under every profile). The fixtures carry these as `bits` beside the numpy answers (`expected`). This is declared in the features and adapters as a **supplement, not third-party evidence**; the TypeScript subject already reproduces every pattern, and three mutants that change last bits only (a reassociated kernel term, a reciprocal instead of a division, a reordered sum) are killed by these tests alone. `🎲️counter-randomness` needs no such scenario: it compares exactly (`ordered-json-v1`).
6. **The gaze case** has a second scenario `eyes` (an eye carried by its bone's matrix, then `lookOffset`) besides the pure `offsets`; the spring is left to `🪀️spring-settling` (work package C) as briefed.
7. **Vectors were generated with numpy 2.4.3** (`.venv`), the registry and the harness environment name 2.5.0; the oracle phase recomputes in the harness environment and agrees (SeedSequence exactly, floats within the adapters' 1e-12 guard).
8. **No platform transcendental** in the three modules (each unit suite reads its module source and refuses `Math.sin/cos/tan/atan2/exp/pow/hypot/log/random`, `Date`, `performance`); the suites themselves avoid them too and take their reference values from the numpy vectors and gl-matrix.

## 5. Open

- **Registry entry for gl-matrix in the core package (work package A).** `P/🔮️oracles/🔣️.json` registers `pets-react-gl-matrix` with the React package as `hostPath`; the unit suites of `🦴️rig` and `📐️trigonometry` use gl-matrix 3.4.3 (`mat2d.multiply`, `invert`, `translate`, `rotate`, `scale`, `fromRotation`, `vec2.transformMat2d`) from the core package (it is already in that package's devDependencies). An entry with `hostPath` `🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript` and capabilities `pets-rig-solving`, `pets-turn-trigonometry` is missing; I did not edit the registry (not my file). The harness `dependency` phase was not run.
- **Rust twins and `🦀️.rs` adapters (phase 2).** Notes for the port: keep `0.0 - x` where the TypeScript writes `0 - x` (it keeps zeros positive; `-x` would not); `Math.floor`/`abs`/`sqrt` → `f64::floor`/`abs`/`sqrt`, no `mul_add`; `Math.imul(a, b) >>> 0` → `a.wrapping_mul(b)`, the difference in `blend` → `wrapping_sub`; keys are `&[u32]`; `randomPick` must project −1 for "none"; the `bit-patterns` scenarios and the `bits` fields of the fixtures are the acceptance test (`format!("{:016x}", value.to_bits())`).
- **No combined `sinCosTurns`**: `solveRig` reduces the angle twice per bone (two short reductions); not measured, not optimised, since the design fixes the two names.
- **Performance was not benchmarked.**
- **`contract` cannot go green for any owner** while the repository-wide backlog exists; only the absence of pets breaches was verified.
- The repo MCP server was unreachable in this session (`CONNECTION_CLOSED`), so no ticket bookkeeping was done from this work package.

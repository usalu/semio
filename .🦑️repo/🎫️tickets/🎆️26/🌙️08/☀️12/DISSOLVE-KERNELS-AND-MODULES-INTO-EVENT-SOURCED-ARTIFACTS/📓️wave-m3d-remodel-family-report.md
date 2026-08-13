# Wave M3d — Photogrammetry Family, `➕️algebra` Split, `🧩️wfc` Window Close

Status: DRAFT IN PROGRESS — being written incrementally as verification lands. Do not treat as final until the "Final status" section at the bottom says COMPLETE.

## Job 3 — `🧩️wfc` duplication window

**Re-measured `semio-s-plugin-procedural` from scratch** (not trusted from the prior wave's report):

```
$ touch "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs"
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-procedural --all-targets
...
error: could not compile `semio-s-plugin-procedural` (lib) due to 93 previous errors; 110 warnings emitted
error: could not compile `semio-s-plugin-procedural` (lib test) due to 103 previous errors; 122 warnings emitted
```

Full log: `scratch-m3d-j3-procedural-check1.txt` in this ticket folder.

- Error count: **93 lib / 103 lib test — identical to the prior wave's baseline** (M3b's report recorded the same 93/103 split). The pre-existing `procedural2d`/`procedural3d` breakage (missing `Procedural2dMutation::Generation`/`SetWidget` variants, `E0252` collisions, `cannot find module change_schema/move_widget/…`) has **not cleared**.
- Verified zero of these errors touch the destination: `grep -B1 '^error' scratch-m3d-j3-procedural-check1.txt | grep -- '-->' | grep -icE 'assembly|wfc-engine'` → **0**. Every erroring `-->` path is under `procedural2d`/`procedural3d` (165 file-location hits for those two dirs across the error set).

**Decision: DO NOT DELETE `🧮️math/🧩️wfc`.** Per the wave brief's own binding instruction, a copy whose destination crate cannot produce a green test run is not a verified copy — deletion stays blocked until whichever session owns `procedural2d`/`procedural3d`'s in-flight mutation-variant rename lands. No files touched for this job; the duplication window (10,930 LOC live in both `🧮️math/🧩️wfc` and `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/…/🧩️wfc-engine/`) **remains open**, unchanged from M3b.

**⚠️ Duplication window still open** — flagged per the ordering contract's own instruction to say so in one unmissable line.

---

## Job 1 — sole-consumership re-verification (`🎯️optimize`, `🔷️lie`, `📶️signal`, `🗺️spatial`)

Re-verified myself, symbol-level, not trusting the brief's framing:

```
$ grep -rlnE "LevenbergMarquardt|SchurLM|RobustLoss|camera_covariances|golden_section|\bSo3\b|\bSe3\b|\bSim3\b|umeyama|savitzky_golay|welch_psd|xcorr_normalized|find_peaks|\bfft2\b|\bifft2\b|KdTree|\bGrid2\b|lo_ransac\b" --include="*.rs" . | grep -v 🎯️target | grep -v "🧮️math/" | grep -v "📸️remodel"
(empty)
```
Zero hits repo-wide outside `🧮️math` and `📸️remodel` for every exported symbol of all four domains. `📸️remodel` is the sole consumer, confirmed at the symbol level (not just a directory-name grep).

Every reference lives in `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/⚙️engine/{🥽️mesh,🏃️motion,📸️sfm,🗺️geo,🌟️feature,📷️camera,🏭️reconstruction,🌫️dense}/🦀️component.rs` (8 files) plus one docstring-only mention in `🗿️artifacts/📸️remodel/🦀️component.rs`. Internal cross-deps among the four domains: `optimize`→`algebra` (cholesky/cholesky_solve/weighted_normal_equations/MatD/VecD) + `geometry::random::Rng`; `lie`→`algebra` (vec3d_*/Mat3d/MatD); `signal`→`algebra` (MatD/VecD); `spatial`→ nothing. No domain references any other math domain (`crate::lie|crate::optimize|crate::signal|crate::spatial` grepped across all four files: empty).

## Job 2 — `➕️algebra` split: duplicate-then-diverge, evidence-based

Measured actual symbol usage (not the brief's paraphrase) via `grep -rohE "math::algebra::[A-Za-z_]+" ✏️s/🔌️plugins/📸️remodel ✏️s/🔨️modules/🏗️fem --include="*.rs" | sort -u` plus the multi-import `use math::algebra::{...}` lines:

| Consumer | Symbols actually used |
|---|---|
| `📸️remodel` | `MatD`, `VecD`, `Mat3d`, `vec3d_cross/length/normalize/sub`, `CsrMatrix`, `cholesky`, `cholesky_solve`, `conjugate_gradient`, `solve_llsq`, `pseudo_inverse`, `svd`, `svd_nullvector`, `jacobi_eigen_symmetric`, `poly_roots_companion`, `real_eigenvalues`, `weighted_normal_equations` (transitively, via `optimize`) — essentially the entire 2,875-LOC solver surface |
| `🏗️fem` (`✏️s/🔨️modules/🏗️fem/⚙️engine/{➗️formulation,🏗️model,📏️elements2d,🧊️elements3d,🔢️sparse,🧮️analyses}`) | `MatD`, `VecD`, `Mat3d`, `vec3d_cross/length/normalize/sub` — **only the 302-line dense-basics region (`Mat2`/`VecD`/`MatD`/`Mat3d`/`vec3d_*`, lines 1–302 of the original file), nothing past it** |

**Correction to the brief's framing, found by reading the actual file, not trusting the paraphrase**: the brief states "fem wants sparse CG/Cholesky". Measured: `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️component.rs` (988 LOC) already implements its **own independent** sparse stack — `Coo`/`Csr`/`CscSym`/`ldlt_factor`/`pcg`/`dense_symmetric_eigen_jacobi`/`cholesky_lower`/`subspace_iteration`/`rcm_order` — and calls `math::algebra::CsrMatrix`/`cholesky`/`conjugate_gradient` **nowhere** (`grep -n "CsrMatrix\|cholesky\|conjugate_gradient"` in that file hits only its own locally-defined `cholesky_lower`/`ldlt_factor`, never a `math::` call). fem's real, current dependency on `math::algebra` is exactly the 7-symbol dense-basics set above.

**Decision: split the file at its own internal boundary, not a blind 100%-vs-100% duplicate.**
- Lines 1–302 (`Mat2`, `VecD`, `MatD`, `Mat3d`, `vec3d_sub/length/normalize/cross` — verified self-contained: zero `crate::` references inside this range, `crate::number` first appears at line 305, past the region) are **duplicated verbatim** into `✏️s/🔨️modules/🏗️fem/⚙️engine/➕️algebra/🦀️component.rs` (302 LOC) — fem's own copy of exactly what it calls.
- The full 2,875 LOC (including the basics region again, plus everything fem never touches: `VecG`/`MatG`/`lll_reduce`, `CsrMatrix`, `cholesky`/`cholesky_solve`, `qr_householder`, `jacobi_eigen_symmetric`, `lanczos_extreme_eigen`, `power_iteration`, `conjugate_gradient`, `expm_pade`, `svd`/`svd_nullvector`/`solve_llsq`/`pseudo_inverse`, `weighted_normal_equations`, `hessenberg`, `real_schur`, `real_eigenvalues`, `poly_roots_companion`) moves wholesale into `📸️remodel`'s own `➕️algebra-internals` — remodel needs almost the entire surface transitively (`svd` calls `qr_householder`; `real_eigenvalues` calls `hessenberg`+`real_schur`), and the binding "keep everything" ruling forbids dropping the unused-by-name-but-internally-load-bearing pieces.
- This is NOT a third "shared home" option: there is no third consumer, and a shared crate for two asymmetric consumers (one uses ~7 symbols, one uses ~30) would be indirection with no payoff, plus algebra is not domain-neutral (fails the framework-exemption test), so it cannot legitimately stay in `🧰️framework/` either. "Duplicate-then-diverge" is the right call; the only correction to the brief is that "duplicate" means "duplicate the ~300-LOC slice actually shared", not "duplicate the whole 2,875-LOC file into a crate that uses 7 symbols out of ~30".

**Residual dependency, disclosed**: `➕️algebra-internals`'s `VecG`/`MatG`/`lll_reduce` region references `crate::number::{EuclideanDomain, Field, GcdDomain, IntegralDomain, Ring}` — `🔢️number` is out of this wave's scope (assigned elsewhere in the ticket's own doctrine table: "fuzzy, number theory → inference helpers under whichever artifact needs them", not this wave's job). Rewritten to `math::number::{...}`, an external plugin→framework crate dependency exactly like the wfc precedent kept `geometry::random::Rng`/`graph_core::` — **`📸️remodel`'s `Cargo.toml` keeps a narrowed `math` dependency, comment-documented as being solely for `math::number`.** `🏗️fem`'s `math` dependency is fully removed (fem's only use was the now-duplicated basics region).

## Mechanical move — what was done

**Destination placement**: `🧬️schema/<domain>-internals/🦀️component.rs`, direct siblings of `snapshot`/`diff`/`mutations`/`inferences` under `🧬️schema/` — this is what `✳️table/🧬️schema/📋️tabular-internals` and `✳️brep/🧬️schema/⚙️engine` actually do on disk (verified by listing both trees before writing anything), **not** nested inside `💡️inferences/` as the brief's prose literally says. I mirrored the verified precedent over the paraphrase; flagging the discrepancy rather than silently picking one.

| File | LOC | Destination |
|---|---|---|
| `🧮️math/➕️algebra/🦀️component.rs` | 2,875 | `📸️remodel/…/🧬️schema/➕️algebra-internals/🦀️component.rs` (full copy, `crate::number::`→`math::number::` rename, 5 occurrences) |
| `🧮️math/➕️algebra/🦀️component.rs` (lines 1–302 only) | 302 | `🏗️fem/⚙️engine/➕️algebra/🦀️component.rs` (verbatim, duplicate) |
| `🧮️math/🎯️optimize/🦀️component.rs` | 1,240 | `📸️remodel/…/🧬️schema/🎯️optimize-internals/🦀️component.rs` (verbatim — `crate::algebra::` resolves unchanged via a crate-root alias, see below) |
| `🧮️math/🔷️lie/🦀️component.rs` | 805 | `📸️remodel/…/🧬️schema/🔷️lie-internals/🦀️component.rs` (verbatim) |
| `🧮️math/📶️signal/🦀️component.rs` | 585 | `📸️remodel/…/🧬️schema/📶️signal-internals/🦀️component.rs` (verbatim) |
| `🧮️math/🗺️spatial/🦀️component.rs` | 754 | `📸️remodel/…/🧬️schema/🗺️spatial-internals/🦀️component.rs` (verbatim) |

LOC parity check: `2875+1240+805+585+754 = 6259`; `wc -l` on the five new remodel files sums to exactly `6259`. Byte-for-byte parity (modulo the 1-line docstring header I added to each, and algebra's 5-occurrence `crate::number`→`math::number` rename).

**Crate-root aliasing** (`✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📦️glue.rs`, new `//#region 🧮️MathInternals` block right after the `extern crate` lines): `pub(crate) use artifacts::remodel::standards::v1::subsets::any::schema::{algebra_internals as algebra, optimize_internals as optimize, lie_internals as lie, signal_internals as signal, spatial_internals as spatial};` (five separate `use` lines). This lets every moved file's own `crate::algebra::…` reference resolve unchanged (no per-file rewrite needed for `optimize`/`lie`/`signal`/`spatial` — verified their only `crate::` references are `crate::algebra::…`, checked line-by-line before copying), **and** lets the 8 app-engine consumer files use `crate::algebra::…`/`crate::lie::…`/etc. in place of the old `math::algebra::…`/`math::lie::…` after a straight `math::` → `crate::` rename (safe: confirmed via `grep -rohE "math::[a-z_]+" ✏️s/🔌️plugins/📸️remodel --include="*.rs" | sort -u` that these 5 domains are the *only* `math::` usage anywhere in the crate before renaming).

**Consumer repoint** (same wave, not deferred): all 8 files in `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/⚙️engine/{🥽️mesh,🏃️motion,📸️sfm,🗺️geo,🌟️feature,📷️camera,🏭️reconstruction,🌫️dense}/🦀️component.rs` repointed `math::` → `crate::`; the one docstring mention in `🗿️artifacts/📸️remodel/🦀️component.rs` updated to say `crate::lie` for accuracy. Post-repoint: `grep -rl "math::" ✏️s/🔌️plugins/📸️remodel --include="*.rs"` → empty.

**fem repoint**: `math::algebra::` → `crate::algebra::` in the 6 files that used it (`➗️formulation`, `🏗️model`, `📏️elements2d`, `🔢️sparse`, `🧊️elements3d`, `🧮️analyses`); `pub mod algebra;` mounted in fem's `📦️glue.rs` next to `pub mod sparse;`; fem's `Cargo.toml` `math` dependency line removed entirely (fem needs nothing else from `🧮️math` — confirmed by the same `math::` grep, post-repoint: empty).

## Genuine `InferredField`/`DepHash` chain (the CQRS-path requirement)

Investigated whether "bundle adjustment" itself could be wired as an `InferredField` and found a real structural obstacle, not just difficulty: `RemodelSnapshot`'s raw reconstruction inputs (feature tracks, per-observation residuals/jacobians, the `Reconstruction` working type `⚙️engine/🗺️geo`'s `camera_covariance_diagonals`/`estimate_per_point_sigma` need) are **deliberately never persisted** — `MotionTrackSummary`'s own docstring states "full per-frame keyframe paths … are plugin-runtime scratch, not durable document state". `results.trajectory.poses: Vec<CameraPosePreview>` is a distilled preview (`camera_id`/`rotation_wxyz`/`translation` only). Forcing a `camera_covariance_diagonals`-shaped inference would require either an expensive synchronous re-solve inside `compute()` (wrong tier — that's what `job`/`results`'s existing async-job-then-persist shape is already correctly for) or persisting exactly the ephemeral data the codebase's own doctrine (and its own docstrings) say must stay tier-(d). I did not force it.

Instead authored a genuine, cheap, correctly-scoped `InferredField` over data that **is** already persisted — `results.trajectory.poses` — using `crate::lie`'s `Se3`/`So3` group composition, which **is** pose-estimation math:

`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔄relative-pose/🦀️component.rs`:

| Field | `FIELD_ID` | Key | Value | Plan shape |
|---|---|---|---|---|
| `RemodelRelativeCameraPose` | `s.remodel.remodel.inference.relative_camera_pose` | `String` (camera id) | `RemodelPoseDelta { translation_delta: [f64;3], rotation_angle_rad: f64 }` | **real linear DAG** — pose `i`'s sole parent is pose `i-1`'s key (trajectory order), not an independent-roots family like `AssemblyEntropy`'s |

`compute()`: `se3_from_preview(prev).inverse().semio_compose_rs(&se3_from_preview(curr))`, then `translation_delta = relative.t`, `rotation_angle_rad = crate::algebra::vec3d_length(relative.r.log())`. `dep_input` covers only the key's own rotation/translation bytes — the predecessor's raw data reaches the chain through the predecessor's own already-computed `DepHash`, exactly the trait's documented "excluding parents' own upstream values" contract (mirrored from `AssemblySolve`/`AssemblyEntropy`'s own precedent, read in full before writing this). Wired into the existing `RemodelInference` struct (`#[derived] pub relative_camera_poses: BTreeMap<String, RemodelPoseDelta>`) alongside the pre-existing whole-snapshot `bounds` leaf; `InferenceSpec::fields()` extended with the new `reads: &["results"]` entry.

Six tests authored (`plan_chains_each_pose_to_its_immediate_predecessor`, `first_pose_has_zero_delta`, `a_pure_translation_step_reports_no_rotation_and_the_exact_offset`, `a_90_degree_yaw_step_reports_the_exact_angle` — asserts a 90° yaw quaternion decodes to `FRAC_PI_2` radians to `1e-6` — `identical_snapshots_produce_byte_identical_deltas`, and a structural test pinning the parent-chain shape). **Not yet run — see Verification below; do not treat as passing until the real command output is pasted.**

## Verification — commands run, with real output

(being appended live — see `scratch-m3d-remodel-check*.txt`, `scratch-m3d-fem-check*.txt` in this ticket folder for the raw logs)

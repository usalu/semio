# 🧭️ Status — Brep Kernel Dependency Free Runtime

Opened 2026-09-03 from audit `.🧬semio/🦑️repo/✍️notes/semio_brep_kernel_audit_7ad363f.md` (HEAD 7ad363fd1e). Plan: `📓️plan.md`.

## Baseline (2026-09-03 16:45)

- `cargo check -p semio-s-plugin-stdio --tests`: 1195 E0277 errors, ALL from the peers' serde-elimination wave (snapshot types lost `serde` derives; per-mutation fixture tests still call `serde_json`). 116 of them are in `✳️brep` per-mutation tests → W1-H converts them. The stdio lib test binary is therefore unusable until every subset is converted → H0 builds an isolated harness crate at `🔬️harness/`.
- `semio-framework-os-kernel` was broken natively/wasm by a peer's live `DirectoryClient` refactor (set_token/mint_session → `authenticated(transport, credential)`); the owner is driving it to zero — not ours, not patched.
- `🔒️dependencies.json`: `brepjs` + `brepjs-opencascade` production-runtime for `@semio-tech/cad-js` (+4 AEC extensions transitively).

## Fleet (Sonnet 5 workers, all background, disjoint ownership per 📓️plan.md)

| id | slice | report |
|---|---|---|
| H0 | isolated brep test harness crate | 📓️h0-harness.md |
| W1-A | neutral contract types + OpQuality + EngineRep removal | 📓️w1a-neutral-core.md |
| W1-B | exact affine transforms | 📓️w1b-transforms.md |
| W1-C | handle lifecycle / labels / GC | 📓️w1c-handles.md |
| W1-D1 | exact NURBS math, knots, interpolation | 📓️w1d1-nurbs-math.md |
| W1-D2 | inverse evaluation, p-curve projection, isocurves | 📓️w1d2-inverse-evaluation.md |
| W1-E | exact analytic primitives with seams + p-curves | 📓️w1e-primitives.md |
| W1-F | one BVH classifier, mass props, validation | 📓️w1f-classify-mass-validate.md |
| W1-G | CDT tessellation, groups + infos | 📓️w1g-tessellation.md |
| W1-H | brep per-mutation tests off serde | 📓️w1h-mutation-tests-codec.md |
| W2-A | exact SSI with p-curves | 📓️w2a-intersections.md |
| W4-A | first-party TS SpatialKernel, brepjs → devDependencies | 📓️w4a-spatial-kernel-first-party.md |

Queued: W2-B booleans, W2-C sweeps, W2-D blends/offsets/draft (after W1-B/D/E, W2-A); W3-A lossless snapshot + viewer/editor (after W1-A/C/G/H); W3-B STEP unification (after W3-A); W3-C flow OpQuality (after W1-A); W4-B differential corpus + launch.json (after W3-A).

## Phase log

- [x] P0 explore: `📓️explore-*.md` (8 reports)
- [x] P1 plan: `📓️plan.md`
- [ ] W1 in flight (10 workers) · W2-A, W4-A in flight

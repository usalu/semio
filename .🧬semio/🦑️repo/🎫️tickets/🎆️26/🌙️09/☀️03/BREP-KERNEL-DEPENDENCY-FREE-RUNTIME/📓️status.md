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
- [x] W1 reported: A, B, D1, D2, E, F, G, H, H0 (harness extended to the full engine façade); C pending report
- [x] W2-A intersections reported; W4-A first-party `SpatialKernel` + brepjs→devDependencies reported (`🔒️dependencies.json` regenerated: brepjs `productionReachable: false`)
- [x] W1-C reported (root lib check exit 0 after 45 min under lock contention)
- [x] W2-C sweeps, W2-D blends/offsets/draft, W3-C flow nodes — code landed, reports written, verification BLOCKED by the repo-wide emoji-corruption incident (see below)
- [ ] W1-Z integration (harness `cargo test` green), W2-B booleans, W3-A artifact/viewer/editor — in flight, blocked on the same incident
- [ ] W3-B STEP unification, W4-B differential corpus — queued behind W3-A

## Notes from peers (2026-09-03 evening)
- `✳️brep` is mounted unconditionally in stdio (`📦️packages/🦀️rust/🦀️.rs:6187`): any brep compile error halts every plugin in the catalog (flow → stdio, process → stdio). Consider a feature gate later.
- Build stdio components with `SEMIO_PLUGIN_PROFILE=wasm-release`; `codegen-units = 1` extended to cad/gis/procedural/process/puzzle/sourcing (rust-lld `ElemSection::writeBody` SIGSEGV).
- Shared target-dir lock saturates with >3 concurrent `cargo check -p semio-s-plugin-stdio`; the fleet gates on `🔬️harness` (own target dir) instead.

## Incident 2026-09-03 19:30–20:30 — foreign rename applier corrupted the repo
A Codex (ChatGPT app) session applying ticket 26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS with a non-idempotent `bun -e` rename plan doubled emoji segments in ~110 Cargo.toml, renamed ~640 directories on disk (`🔺️⚙️mesh-engine`, `🧮️🔢️math`, `🔢️🔢️number`, `📦️📦️packages`, `🧬️🧬️schema`, …) and rewrote path literals inside sources (`#[path]`, `include_str!`, "source authority" strings). `cargo metadata` failed repo-wide; every gate (root and harness) was untrustworthy from ~19:30. semio-a4 restored the directories (deterministic `mv` by ASCII skeleton), semio-2f is repairing manifests + source literals. ✳️brep sources, the flow brep extension's Rust sources and `🔬️harness` were scanned clean; the stdio and flow-brep manifests were among the corrupted ones (left to the repo-wide repair). Verification of W2-C/W2-D/W3-C/W3-A/W2-B/W1-Z resumes once the harness compiles the framework crates again.

## Harness milestone 2026-09-03 20:20
`🔬️harness` (`bun ./📜️script.ts sync && cargo test` — the sync step re-points framework path deps by ASCII skeleton because the foreign rename applier keeps flipping `🔺️mesh-engine`↔`🔺️⚙️mesh-engine`, `🔢️number`↔`🔢️🔢️number`) builds the whole kernel layer incl. wave 2 and runs **448 tests: 394 pass, 51 fail, 1 ignored** (`🗑️generated/coordinator-harness-test.txt`, digest `coordinator-harness-failures.md`). Coordinator fixed one E0382 in `➡️sweep/🥞️loft/🦀️.rs`.

Fixers (Sonnet, disjoint files): FX-1 intersect (8) → `📓️fx1-intersect.md` · FX-2 curves/surfaces (14) → `📓️fx2-curves-surfaces.md` · FX-3 offset/blend (8) → `📓️fx3-offset-blend.md` · FX-4 sweep (5) → `📓️fx4-sweeps.md` · FX-5 inferences+primitives (9) → `📓️fx5-inferences.md` · W2-B boolean/euler/engine (7) → `📓️w2b-booleans.md`. W1-Z asked to stand down and report.

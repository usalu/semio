# Mechanics kernel `semio-s-plugin-norm-mechanics`

Location: `✏️s/🔌️plugins/📕️norm/🧮️mechanics` (workspace member + `[workspace.dependencies]` entry added to the norm `Cargo.toml`; `📋️project.json` + `📜️script.ts` added in the style of the contract crate, project `@semio-s/plugin-norm-mechanics`). No external runtime dependencies; dev-dependency only on the compliance crate (`python_oracle`). API announced in `📌️important/requests.md`.

## Contents
- `🧬️schema`: `🦀️.rs` source plus `🛰️.proto`, `🔗️.graphql`, `🟦️.ts` twins (parity test). Model: nodes, materials, sections (A, Iy, Iz, J, Iw, shear areas), members (roll, 6+6 end releases, Euler-Bernoulli or Timoshenko), supports (fixed, spring, prescribed displacement), load cases (nodal, distributed global/local linear, temperature uniform + gradients, acceleration/self-weight), nodal masses; results and `MechanicsError`.
- `🧮️linalg`: dense Cholesky, skyline (column profile) Cholesky with RCM node ordering, Householder + implicit-QL symmetric eigen solver, generalised dense solver (Cholesky reduction, indefinite B allowed), skyline subspace iteration with Rayleigh-Ritz for models above 600 free DOF.
- `🧱️elements`: 12x12 frame element, geometric stiffness, consistent/lumped mass, equivalent loads (Gauss), static condensation of releases.
- `📏️statics`: `Analysis` (factorise once, solve many cases): displacements, reactions (fixed DOF, springs, settlements), end forces, sampled N/Vy/Vz/T/My/Mz diagrams, `member_curve` (Hermite displaced shape for any mode), `Model::refine(n)`.
- `🌀️buckling`: alpha_cr and mode shapes from axial-force geometric stiffness. `🎵️modal`: frequencies, mass-normalised participation, effective modal masses (global X/Y/Z, coupling to supports included), consistent or lumped. `🦋️ltb`: single member with `[v, v', phi, phi']` Hermite DOF (warping), arbitrary moment table, line/point loads with load height, axial force, Wagner term, end and intermediate restraints, mode shape.
- `🎛️control`: progress callback + `AtomicBool` cooperative cancellation checked in assembly, factorisation, eigen loops. Deterministic: no hash maps, no threads, LCG start vectors.
- `🧪️tests`: 23 unit tests, `🥒️.feature` (its Examples tables drive the Euler and frequency tests), `🔮️oracles/🐍️.py`.
- Foreign addition: another agent added `🏛️member-modes` (buckling/deflection of one member on top of this kernel); left untouched, its two tests pass.

## Verification
- Standalone `rustc +nightly-2026-07-20 --test` of the whole crate (oracle shim pointing at the repo `.venv` python): 23 passed, 0 failed, no warnings. The repo build `@semio-s/plugin-norm-mechanics:check` was queued 5000+ s then failed in cargo preparation with the fleet-wide infra error ("compiler ingress requires the repository launcher and authentic enforcing store", same as en1997/en1998). A retry stayed queued and was cancelled; the integrator should run `check` and `test` once the ingress is fixed.
- Closed forms: cantilever tip deflection and root moment, Timoshenko shear deflection, SS beam UDL (reaction, M = wL^2/8, 5wL^4/384EI), fixed-fixed settlement (12EIδ/L^3, 6EIδ/L^2), restrained uniform and gradient temperature, portal sway 24EI/h^3 and 6EI/h^3, propped cantilever with releases, Euler columns (pinned, fixed-free, fixed-pinned 2.0458, fixed-fixed) within 1e-3, SS beam frequencies modes 1..3 (1e-3), first effective mass ratio 8/pi^2 (5e-3), cantilever omega (1e-4), uniform-moment Mcr (1e-4), axial-only LTB column = Euler, load height ordering top < centre < bottom, 120-element model through the subspace path.
- Third-party oracle (`🐍️.py`, via `python_oracle`): PyNiteFEA (SS beam deflection and reaction 1e-6/1e-9, portal sway and base moment 1e-6, 3D L-frame ux/uy/uz with torsion 1e-6, member `ke`/`kg` matrices for the 4-element Euler column 1e-6), anastruct (SS beam, portal sway 1e-6), scipy `eigh` with Pynite stiffness plus independently typed mass matrix (modal f1/f2 1e-6) and the closed-form continuous beam (1e-3), scipy sine-series Galerkin (quad integrals) for LTB: uniform, UDL and central point load at shear centre and 0.15 m above (2e-3).
- Finding: PyNiteFEA 3.0.0 `analyze_modal` gives f1 = 12.30 Hz for a case whose closed form is 20.19 Hz (ratio 0.61, f2/f1 = 3.94); it was NOT used as oracle. Its stiffness matrices were.

## Limits
- Frame buckling uses the axial geometric stiffness only; LTB is the single-member solver. Wagner term is a user parameter, not derived from a section.
- Condensed (released) member ends use Guyan condensation for mass and geometric stiffness (approximate when the released rotation is dynamically active); mode curves recover released end rotations through the condensation map.
- Timoshenko fixed-end loads use the Euler-Bernoulli load shape functions (exact for uniform load).
- Subspace path (above 600 free DOF) converges to 1e-10 on lambda or 100 iterations; indefinite buckling problems there keep only positive factors.

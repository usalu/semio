# Fidelity: EN 1993, EN 1994, EN 1999 scenes

Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`. Nothing was compiled (build loop: single compile owner; "ready for rebuild: scene fidelity" appended to `build-en1993/1994/1999/latest.md`). Paths below are relative to `✏️s/🔌️plugins/📕️norm`.

## Shared pieces
- `🧮️mechanics/🏛️member-modes` (new, pure addition): `buckling(props, ends)` (Euler load and first mode for Pinned, Cantilever, Fixed), `deflection(props, ends, loads)` (udl, point, end couples) on the frame kernel, 8 Hermite elements. Tests: Euler x1, x1/4, x4; udl 5qL^4/384EI, point, couples, tip (feature scenario added).
- `🎬️structural-scene/🧰️projection-kit/🌈️station-fields`: `envelope(checks)` (union of positions, largest interpolated utilisation) with unit test.
- Each artifact workspace `Cargo.toml` and each scene crate `Cargo.toml` gained `semio-s-plugin-norm-mechanics`.

## EN 1993 (`🗿️artifacts/🔩️en1993`)
- Checks (`🧬️schema/💡️inferences/⚖️checks/🧱️member-checks`): new `📈️member-profile` (moment shape from `momentDiagram` uniform / parabolic / linear with psi, shear shape, deflection shape from `EI w'' = -M`). Stations (10 segments) on 6.2.4 compression, 6.2.5 My/Mz, 6.2.6 shear, 6.2.9 M+N, 6.3.1 buckling, 6.3.2 LTB, 6.3.3 interaction (Annex A and B evaluated per station), 7.2 deflection. `InteractionInput` and `AnnexAInput` gained `Clone, Copy`.
- Scene: `🌀️eigen-modes` replaces the sinusoid. Flexural mode and `alpha_cr = Ncr/N_Ed` from the kernel for the drawn supports; LTB mode with the declared moment diagram, load height and restraints every `ltbRestraintSpacing` (`Mcr/M_Ed`); static deflection line under the SLS moment when no buckling check governs. Overlay `scale` carries the display magnification, samples are true displacements. `alpha_cr` label per member. Member colour per vertex: `envelope` of all member checks into `per_station` and `MemberGeometry.stations`.
- Tests: check stations follow the diagram and satisfy the station law; kernel modes vs Python oracle `🎬️scene/🔮️oracles/🌀️modes.py` (numpy FE when installed, closed forms otherwise: Euler, Mcr, deflection); cantilever = Euler/4; per-station member colour; alpha_cr label.
- Still approximate: effective lengths come from the drawn supports, not `bucklingLength*`; fatigue/fire glyph positions, T-stub, plated-panel and cable items untouched (need snapshot fields, see scene-coverage).

## EN 1994 (`🗿️artifacts/🧩️en1994`)
- Checks (`🧮️compliance/🏗️beams`): stations (16 segments over the total length, so the continuous sagging peak at 3/16 is sampled) on sagging resistance, hogging resistance, vertical shear, stud resistance P_Rd (elastic shear-flow shape scaled to the governing demand), slab longitudinal shear, LTB. Shapes: simple span 4s(1-s), |1-2s|; two spans 3u/8 - u^2/2.
- Scene: `🌀️statics` (kernel UDL analysis of one or two spans). Deflection line (3D overlay and diagram) from the kernel instead of `3.2(u-2u^3+u^4)`; cracked zone and hogging bars span the kernel contraflexure points (0.375 and 0.625 of the total length) instead of the fixed 0.85 L to 1.15 L; stud zones coloured by the P_Rd station value at the zone centre; steel member coloured per station.
- Tests: kernel line vs closed form (0.2 %), contraflexure points, check station law and peak positions, per-station member and differing stud zones.
- Still approximate: stud stations equal pitch, no per-stud records, column section inferred from area, biaxial/second-order absent.

## EN 1999 (`🗿️artifacts/🪶️en1999`)
- Checks (`🧱️member-checks`): `emit` attaches stations (10 segments, rescaled so the maximum is exactly the governing utilisation) for 6.2.3, 6.2.5 (My, Mz), 6.2.6, 6.2.8, 6.3.1, 6.3.1.4, 6.3.2, 6.7.4; 6.2.9 and 6.3.3 recompute their formulas per station. Shape from support (simple, fixed-fixed `continuous`, cantilever) and source (udl or external) of the governing action.
- Scene: `🌀️eigen-modes`: flexural mode (support-dependent), LTB mode (moment line, unrestrained length as restraint spacing, end conditions from support), deflection line scaled to the computed delta; `alpha_cr` label; per-station member colour. Local-buckling overlay now uses the exact half-sine with `L / b` half waves for the governing plate width and the buckling direction of its plate (flange: z, web: y) instead of `round(L/depth)`.
- Tests: Euler loads for the three supports, LTB rises with shorter unrestrained length, deflection peak, per-station colour; check stations follow support and load.
- Open: suspected doubled support factor in the check (request filed); no weld-run path, so no weld/HAZ zone stations; cold-formed schematic profile untouched; bolt positions already from pitch/gauge.

## Open for the integrator
Compile order: mechanics, structural-scene, then each `-rs` project; regenerate the three 0-byte golden scene fixtures with `SEMIO_UPDATE_GOLDEN=1`; the shape helpers of EN 1993 / EN 1999 `eigen-modes` share the kernel through `member_modes`, only the LTB tables remain per artifact.

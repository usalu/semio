# EN 1998-5 (GE-01..GE-12), wave 2

Python only, nothing compiled. Source: `🗿️artifacts/🫨️en1998/…/🧭️spec-trace/🔣️.json` (+ regenerated `🦀️.rs` via `⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py --write`; verifier: en1998 45 items ok, was 33).

## Result
Most of Part 5 was implemented but untraced; 8 rows are now traced to real checks, 4 are `not-modelled` with specific reasons.

| id | status | trace |
| --- | --- | --- |
| GE-01 | partial | `.profile.siteResponse` (v_s,30 from layers, ground type, site period); G_max and G/G_max(gamma) not modelled (no verified curve) |
| GE-02 | traced | `.liquefaction.screen` (a_g S/g and depth screens, NDP rows unverified) |
| GE-03 | traced | `.liquefaction`, `.layer.` (F_L = CRR MSF/CSR, remedies, Youd comparison; oracle groundhog + scipy already in `🔮️oracles/⚖️compliance/🐍️.py`) |
| GE-04 | not-modelled | no formula/input in the spec |
| GE-05 | not-modelled | needs slope entity; EN 1997 owns slope kernels and seismicKh (one source of truth), request filed |
| GE-06 | traced | `.annexF` (oracle scipy brentq) |
| GE-07 | traced, noted | `.sliding`: H_Rd is an input, not (V-u) tan delta / gamma_Rd |
| GE-08 | not-modelled | coefficients 0.3/0.4/0.6 unverified, no entity |
| GE-09 | partial | `.radiationDamping` (k_f/k_s >= 1.5); Gazetas springs not modelled |
| GE-10 | traced | `.thrust` (M-O, vertical smooth wall); shared kernel with en1997 RW-08 requested |
| GE-11 | not-modelled | no water height field on the retaining wall; closed form 7/12 k_h gamma_w H^2 known |
| GE-12 | partial | `.pile.*` (inertial, kinematic, moment, length; oracle scipy solve_bvp); liquefied-layer support loss and negative skin friction not modelled |

## Not done (open)
- No new schema/kind/3D work: GE-04/05/08/11 need new entities or fields; the field-adder has no `retainingWalls` profile, and nothing could be compiled. Requests appended to `📌️important/requests.md` (shared M-O kernel, slope entity, retainingWalls adder profile).
- No schema edits, so no twin generation was needed; strict twin check, schema-sync, fixture checker, handle-paths and build-owners were not run (no schema/fixture change).
- 3D for liquefiable layer colouring, foundation arrows and M-O pressure already exists in `🎬️scene/🌍️geotechnical` (not touched).

- ready for rebuild: en1998 part 5

# EN 1993 parts (wave 2): cold-formed, stainless, special-part traces

Artifact `🔩️en1993`. No build was run (by rule); everything below is checked by reading only.

## Trace rows added (`🧭️spec-trace/🔣️.json`, 166 ids with the parallel toughness and joint rows; twin by `🐍️.py --write`)
- V-255 (6.1.5), V-256 (6.1.7), V-257 (6.1.8, code section 6.1.11), V-260 (10.1.5): code existed untraced -> trace rows only (`en1993.1-3.6.1.5/6.1.7/6.1.11/10.1.5`).
- V-258 (6.1.9): NEW check `en1993.1-3.6.1.9.<cf>` = N_Ed/N_c,Rd + M_Ed/M_c,Rd <= 1 (`cold_formed::axial_bending_utilisation`; N from the governing combination, N_c,Rd = rho*gross, M from existing fields). Centroid shift DeltaM not carried (no field); equation not verified against the print.
- V-259 (section 8 screws, rivets): NOT MODELLED (reason in trace, EN+DE): Tables 8.x not in the repo (no invented values), no fastener row family; needs a field-adder profile for en1993, snapshot/twins/mutations/io and a 3D fastener layout with grips. Open.
- V-301..V-305 stainless: code existed (rho Table 5.3, section_rd, chi flexural/LTB/torsional, fire factors, Ramberg-Osgood secant/tangent) -> trace rows (`en1993.1-4.5.5`, `en1993.1-4.ss`, `en1993.1-4.fire`). Grades are now a typed enum `StainlessGradeId` (`cold_stainless`), registry rows (fy, fu, E, n for 6 grades) already existed.
- V-806 -> `en1993.1-5.D.shear` (existing annex test with oracle values); V-901 -> `en1993.1-2.nfi` cross-reference; V-902 -> `en1993.1-12.hss`; V-903 -> `en1993.1-7.lateral`; V-904 -> `en1993.5.5.2.{m,v,il}`; V-905 -> `en1993.4-3.5.{t,vm,pcr}`.
- V-906: NEW check `en1993.1-6.8.thin.<shell>` (t/r against 1/20, `shell_stability::thin_shell_ratio`, `THIN_SHELL_LIMIT_INVERSE`); reported beside V-800..V-805, does not suppress them; fail remedy caps thickness.

## 3D
`🎬️scene-members/🧩️detail-sketches`: for cold-formed members with a bearing length, overlay plates `<id>:bearing` (patch of l_a under the member) and `<id>:crippling` (web zone h x l_a). Orientation of the Cee web in the member frame is assumed (unverified visually). Fastener positions with grips: blocked with V-259.

## Tests / oracles / features
- Rust: `special-wiring` (6.1.9 sum, 6.1.7/6.1.11/10.1.5 hand relations, V-906 pass+fail, V-902 on/off at 460 MPa, V-903/904/905 present); `clauses/tests/special-parts` (per-id tests V-255..V-258, V-301..V-305, typed grade enum); scene test for bearing/crippling entities; `catalogue-trace` count 160.
- Oracle `🔮️oracles/🧮️special-parts/🐍️.py` (numpy/scipy): new `st_ro_strain`, `st_ro_tangent` (numpy gradient), `w_thin_shell`; feature rows added in `🧪️tests/🔭️special-parts/🥒️.feature`. The sectionproperties cross-check for effective sections was not added (needs the library run; no stainless/cold-formed section outline fixtures exist).

## Not done / open
- V-259 and the fastener 3D; DeltaM of V-258; V-257 clause label (spec 6.1.8 vs code id 6.1.11 kept as is).
- Unverified values remain registry rows (S0): cripping category alpha/la, stainless rho/alpha/lambda0 coefficients, fire table (1.4301 only), grade table.
- No schema change was made (no new snapshot fields), so no twin regeneration is required for this work.

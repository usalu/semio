# EN 1999 compliant aluminium roof purlin (2026-11-23)

## Numbers designed against (temporary `[DEBUG]` dump, removed)
Engine verdicts were not wrong; the committed DSL had drifted from the code-built `compliant_roof_purlin()`. Before the redesign the DSL had L = 6 m, g+q = 1.3 kN/m (delta about 44 mm), bolt gauge 16 mm (below 2.4 d0 = 31 mm), fire theta_a = 180 C for 600 s unprotected, sheet with g = 120 N/m and span 1 m, shell loads raised. The concurrent redesign (L = 5 m, 350 + 800 N/m, gauge 40 mm / pitch 50 mm, weld throat 6 mm x 200 mm, sheet span 0.8 m, g 40 N/m) already removed the defl, gauge, sheet and shell failures (defl u = 0.78, sheet defl u = 0.91, shell max u = 0.20).

Fire (dump of the 5 m purlin, theta_0 = 20 C, member A_m/V from the 140x70 I section):
- unprotected 900 s: theta = 698 C, cross-section and LTB fail (theta_cr 299 C)
- unprotected 1800 s: theta = 831 C, fails
- 30 mm board, lambda 0.10, 1800 s: theta = 227 C, u = 0.23, passes
- 30 mm board, lambda 0.06, 1800 s: theta = 143 C, u_cs = 0.106, theta/theta_cr = 0.48, passes

## Final design
Section 140x70x10x8 extruded I, EN AW-6082-T6, simply supported span 5 m, LTB restraint length 1.5 m, DE annex. Loads G 350 N/m, snow 800 N/m. Support: 2x2 M12 8.8 bolts (pitch 50, gauge 40, edge 24, plate 8 mm) with 6 mm x 200 mm MIG fillet welds 4043. Fire: R30 (1800 s, 20 C start) with a 30 mm protective board (lambda 0.06 W/mK, c 1000 J/kgK, rho 300 kg/m3). Sheeting 5 mm x 80 mm strip, span 0.8 m. Shell and fatigue unchanged. `noncompliant-multi-fail` untouched and still failing its claimed checks.

## Changed files (en1999 family, under `🏅️standards/🔖️1/🪆️subsets/✳️any/`)
- `🧬️schema/📸️snapshot/🦀️.rs`: `compliant_roof_purlin()` fire 1800 s plus `ins-1` insulation row (the rest of the redesign was already in the file).
- `🖼️assets/🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio`, `🔣️.json`, `🖼️assets/🏠️aluminium-roof-purlin/🎒️.pack.semio`: regenerated from the snapshot through the family encoders (temporary generator test, removed).
- `📚️examples/🏠️aluminium-roof-purlin/🧪️tests/🧩️example/🦀️.rs`: compliance test kept; added decode(DSL) == `snapshot()` and pack/JSON twin == snapshot (pack is canonical print).

## Results
- examples crate: see final lines in the reply.
- editor crate `semio-s-artifact-norm-en1999-editor` `every_pack_and_json_twin_equals_the_dsl_document_of_its_example` could not be built: that crate currently fails to compile on unrelated in-flight work (missing serde derives for `InsertSheetProfile`, `InsertFireInsulation` and related commands). The twin equality is covered by the new examples-crate test instead.

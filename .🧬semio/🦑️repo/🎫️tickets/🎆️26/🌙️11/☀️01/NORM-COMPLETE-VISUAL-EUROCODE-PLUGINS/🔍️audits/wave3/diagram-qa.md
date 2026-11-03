# 📈️ Wave 3 Diagram QA (read-only)

Auditor: W3-DIAGRAM-QA. No code was edited. Scratch scripts and extracts live in `/tmp/claude-1000/-home-deck-semio/84faf2e3-4721-4a15-8b57-5d44cdd1d7f5/scratchpad/` (`scope.txt`, `axes.txt`, `markers2.txt`, `pts.txt`).

Base for all paths below: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/`. `<code>` is the folder such as `🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene`.

## Scope and method

- In scope: 37 `📈️*` scene diagram files (`📈️diagrams`, `📈️annex-diagrams`, `📈️design-diagrams`, `📈️entity-diagrams` and their sub-modules) across the ten codes. Excluded: `👁️viewer`/`✏️editor` window shims, `🧪️tests`, and `🧬️schema` mutation folders (no `Diagram2d` constructor).
- About 150 `Diagram2d` constructors counted. Ids and axes were extracted by script. Markers were classified by `check` and `binding` arguments with a parser. Colour literals (hex, rgb) and hard-coded arrays were grepped.
- Manually read and confirmed: the flagged sites in en1996, en1997, en1998, en1992 and the en1991 demand markers.
- Limits of the method: constructors that build their limit lines or table twins through helpers (`finish()`, `base()`, `series` builders) were verified by reading the helper, not by the script. Rows marked "scan" below come from the parser only and were not re-read line by line. Line numbers are exact for every row.

## Per-code summary

Counts: `ctor` = `Diagram2d` constructors; `ids` = colon `<kind>:<row>`, hyphen, other; `twin` = table twin present (checked by helper); `lim` = `Series::Limit` occurrences; `unbound` = markers with no check binding (scan + manual); `nofield` = check-bound markers with no field binding (drag); `tone-lit` = hand-picked `Tone::` on check-related markers; `axis` = axis-title or unit issues (see violations).

| Code | Scene files | ctor | ids (colon / hyphen / other) | twin | lim | unbound | nofield | tone-lit | axis | data | Hex colours |
|---|---|---|---|---|---|---|---|---|---|---|---|
| EN 1990 | `⚖️en1990/.../📈️diagrams` | 12 | 2 / 2 / 8 | all | 6 | 3 (scan) | 5 | 7 | 0 | Annex D tables (ok) | 0 |
| EN 1991 | `🏋️en1991/.../📈️diagrams` (+ `row-curves`, `outcomes`) | 30 | 1 / 27 / 2 | all | 3 | 10 (demand `required-*`) | 8 | 5 | 9 | `snow`, `fire`, `traffic` formula-sampled; impact pulse from inputs | 0 |
| EN 1992 | `🏛️en1992/.../📈️diagrams` | 8 (via `base()`) | `member/kind` (slash) | all | 2 | 1 (scan, `midspan`) | 1 | 12 | 1 | fine | 0 |
| EN 1993 | `🔩️en1993/.../📈️diagrams`, `annex-diagrams`, `design-diagrams`, `entity-diagrams` | 33 | 32 / 0 / 1 | all | 21 | 6 (scan) | 22 | 9 | 5 | `load-case-{key}:{}`, `frame-{key}:{}:{suffix}` (extra segments) | 0 |
| EN 1994 | `🧩️en1994/.../📈️diagrams` | 25 | 14 / 0 / 11 | all | 1 | 0 | 4 | 6 | 3 | fixed time grid `[30,60,90,120]` (L631) | 0 |
| EN 1995 | `🪵️en1995/.../📈️diagrams` (+ `components`, `entity-curves`, `failure-modes`, `response-curves`, `special`) | 23 | 19 / 0 / 4 | all | 3 | 1 (hinges) | 14 | 4 | 2 | `failure-modes` hard-coded sketch geometry | 0 |
| EN 1996 | `🪨️en1996/.../📈️diagrams` | 8 (via `finish()`) | 1 / 1 / 6 (parser approximate; `ecc:` and `shear:` are colon-form; `fk-m{}` has no separator) | all | 10 | 1 (M–N demand) | 5 | 4 | 2 | 3-row eccentricity twin (fine) | 0 |
| EN 1997 | `🌍️en1997/.../📈️diagrams` (+ `plans`) | 25 | 18 / 2 / 5 | all | 18 | 12 (`marker()` helper) | 1 | 1 | 9 | fine | 0 |
| EN 1998 | `🫨️en1998/.../📈️diagrams` | 10 | 4 / 2 / 4 | all | 5 | 7 | 4 | 9 | 2 | `VERTICAL_CORNERS` constants | 0 |
| EN 1999 | `🪶️en1999/.../📈️diagrams` (+ `connection-diagrams`, `patch-resistance`, `sheet-diagrams`, `stress-strain`, `web-panels`) | 23 | 23 / 0 / 0 | all | 13 | 4 (scan) | 9 | 8 | 1 | fine | 0 |

Totals: about 150 constructors, all carry a table twin, no hex or rgb literal in any in-scope file. The colour rule is met at the token level, but check-tied tones are often chosen by hand (see V-11).

## Violations, ranked

### High

**V-1. EN 1991 demand markers have neither a check nor a binding.** Ten `required-{id}` markers are design points without a check. All call `marker(report, ..., None, ..., None)`.
- `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs` lines 438, 458, 525, 549, 585, 612, 656, 823, 849, 948.
- Line 481 has a binding but no check. Line 586 has a binding but no check. Lines 613 `assumed-*` also have no check.
- Fix: bind each demand to the check ID of its clause (for example `en1991.1-7.*`).

**V-2. EN 1997 `marker()` helper drops both check and binding.** The helper sets `bound_check: None, binding: None` on every marker (line 32). Its callers include the load-settlement working points, which are design points.
- File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs`, helper at line 31–32.
- Callers: lines 58, 60, 101, 206, 259 (`:work`), 286, 358, 388, 428–430, 531 (`:work`, load-settlement), 611, 634.
- Fix: pass `check_id` and `binding` into the helper. The `:work` and `:spencer`/`:circle` markers must bind to the check of their diagram.

**V-3. EN 1996 M–N demand point is unbound.** The demand marker has `utilisation: Some(m_ed / m_rd)` but `bound: None, binding: None`. The utilisation is computed locally and does not follow a report check.
- File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs` line 358.
- Fix: bind to the wall's plane-section check ID, and bind the wind-moment field (drag) in the same way as the eccentricity markers.

**V-4. EN 1998 spectra and mode markers are unbound.** `mode-` (cumulative mass and Sd), `site-s-ap-r`, `vertical-{name}` and corner markers have `bound_check: None`.
- File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs` lines 125 (`corner-`), 146 (`2tc`, `4tc`), 152 (`mode-`), 187 (`vertical-`), 256 (`mode-` cumulative), 287 (`site-s-ap-r`).
- Line 256 chooses tone by a hard threshold `cumulative >= 0.9` (Info/Warning), not by utilisation tone.
- Fix: bind `mode-` markers to the modal-participation check and the site marker to the site-zone check.

### Medium

**V-5. EN 1990 design-point markers unbound (scan).** Two `Marker` literals have `check = None` (lines 304, 314); the Annex D test marker (line 338) has `bound_check` but no binding. Check that the form-diagram design point (Annex C alphas) is bound.
- File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs`.

**V-6. EN 1993 unbound markers (scan).** `critical-time` (line 347, fire), `ky-at-end` (line 336), `hogging`/`sagging` strip markers (annex-diagrams lines 131–132), and `flexural` (entity-diagrams line 330). They are design or critical points without a check.
- Files: `🔩️en1993/.../📈️diagrams/🦀️.rs`, `.../📈️annex-diagrams/🦀️.rs`, `.../📈️entity-diagrams/🦀️.rs`.

**V-7. EN 1993 design-diagrams: check-bound markers without a field binding.** Six markers (lines 62, 91, 114, 137, 364, 398) are bound to a check but not to a drag field, so they cannot be used as design points in the editor.
- File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️design-diagrams/🦀️.rs`.

**V-8. EN 1999 unbound markers (scan).** `k-at-end` (`📈️diagrams/🦀️.rs` line 470, fire-reduction design point), `rho-{id}` (line 304), and the stress-strain markers `f-0-2`, `f-u`, `f-0-2-haz` (stress-strain lines 19–20, 33, 38). The centroid marker (line 598) is geometric and acceptable.
- File base: `🪶️en1999/.../📈️diagrams/`.

**V-9. EN 1995 hinges and failure-mode sketches.** Plastic hinge markers `hinge-{letter}-{k}` (failure-modes line 95) are unbound. The Johansen mode sketches `sk(&[...])` (failure-modes lines 40–55) use hard-coded node coordinates, not the document's connection geometry. These are illustrative, but they do not reflect the member's data.
- File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🧩️failure-modes/🦀️.rs`.

**V-10. EN 1992 default table twin is empty (latent).** `base()` sets `table_twin: DataTable { columns: Vec::new() }` (line 40). All eight callers overwrite it, so there is no live violation, but any new caller will silently ship an empty twin.
- File: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📈️diagrams/🦀️.rs` line 40.
- Fix: take the twin as a parameter.

**V-11. Hand-picked tones on check-related markers (tone-lit column).** The rule is that colour follows the shared utilisation tone. These cases use fixed or threshold tones instead:
- EN 1998 line 256 (`cumulative >= 0.9`).
- EN 1990 line 338 (`design >= series.required` picks Info or Error), file `⚖️en1990/.../📈️diagrams/🦀️.rs`.
- EN 1993 line 347 (`t < minutes` picks Error, `🔩️en1993/.../📈️diagrams/🦀️.rs`).
- EN 1995 line 223 (`now >= f1_limit` picks Error or Info).
- EN 1992 `moment-curvature` (12 literal tones across the file, state I/II lines use fixed Info).
- Fix: map each check-tied tone through `utilisation_tone`. Keep fixed tones only for limit and reference lines.

### Low

**V-12. Axis titles untranslated (EN text repeated in DE).** The DE title should be translated, not copied.
- EN 1992 line 114 (`moment-curvature`): `axis("Moment My", "Moment My", …, "kN·m")`.
- EN 1993 `annex-diagrams` line 128: `"Position x"` and `"Moment M"` (both repeated). Also `N·mm/mm` in the unit.
- EN 1994 `📈️diagrams/🦀️.rs` lines 154 (`Position x`), 231 (`Moment M`), 334, 383, 448 (`Position x`), 682 (`Moment m`), 726 (`Position x/y`).
- EN 1995 lines 429 (`Position`/`Eigenform`) and EN 1996 line 356 (`Moment M`).
- EN 1997 line 396 (`Position`).
- EN 1990 lines 171 (`ψ`) and 216 (`γ·ψ`) use symbols only; acceptable.
- Fix: DE "Lage x", "Lage y", "Moment M" (accepted), "Bauteilnummer" style nouns.

**V-13. Unit symbols inconsistent across codes.** The axis unit is a free string. The same unit is spelled several ways:
- `kN/m2` (EN 1991, 13 uses) vs `kN/m²` (EN 1991 lines 61 and 554; 2 uses). `m2` (EN 1991 lines 110–111) vs `m²`.
- `deg` (EN 1991 lines 100, 127, 135; EN 1997 lines 214, 628) vs `°` (EN 1995 lines 31, 59).
- `degC` (EN 1991 line 208) vs `°C` (9 uses elsewhere).
- Dimensionless axes use `1` (144 uses) or `-` (15 uses: EN 1991 lines 44, EN 1996 line 596, EN 1997 lines 46, 214, 262, 327, 605, 628).
- `kNm/m` (EN 1996 line 356) vs `kN·m/m` convention; `N·mm/mm` (EN 1993 annex-diagrams line 128) vs `N·m`.
- `1/km` (EN 1994 line 231).
- Fix: one unit vocabulary in the shared kit (`kN/m²`, `°`, `°C`, `1`).

**V-14. Quantity kind does not match unit.** Dimensionless kind with a physical unit:
- EN 1991 line 554 `Vent ratio A_v/V` with `1/m`.
- EN 1994 line 231 `Curvature κ` with `1/km` (should be a curvature kind).
- EN 1995 line 31 `Taper angle α` with `°` (should be `QuantityKind::Angle`).

**V-15. Id pattern drift against `<kind>:<row>`.** Colon ids are used in EN 1993, EN 1995, EN 1997 and EN 1999. Other codes use hyphen or other forms:
- EN 1991: 27 of 30 are hyphen (`qp-profile`, `cpe-area`, `snow-mu3`, `bridge-lane-plan`, …).
- EN 1992: `member/kind` with a slash (`base()` line 30).
- EN 1996: `fk-m{}` (line 201) has no separator; `phi-m-{}`, `ecc:` and `shear:` are mixed.
- EN 1990 and EN 1994: plain ids without a row (`matrix`, `psi`, `pf`, `capacity`, `section`).
- EN 1993 `frame-{key}:{}:{suffix}` (design-diagrams line 259) has an extra segment.
- Fix: one rule `<kind>:<row>` in the shared constructor, applied everywhere.

**V-16. Fixed sample grids.** EN 1994 line 631 uses `[30.0, 60.0, 90.0, 120.0]` as a time grid. EN 1996 line 174 uses `[0.05, 0.1, 0.2, 0.3]` as a slenderness-ratio sweep, and line 369 uses `vec![0.0, 1.0]` as two stages. These are parameter sweeps, not document data, so they are acceptable but should be documented.
- EN 1990 `KN_NODES` and `KN_V_KNOWN`/`KD_V_KNOWN` (`📈️reliability-math` lines 356–381) are normative tables. Check that each has a source note.

## Rules with no violation found

- Table twin: every constructor carries one. The helper-built ones (EN 1996 `finish()`, EN 1992 `base()`) were checked by reading.
- Colour literals: no hex, rgb or `Color32` literal in any in-scope file.
- Limit lines: present where a check has a limit in the spots read (EN 1992 `deflection` L12–13 bound to `check.limit.value`; EN 1995 deflection L157–158; EN 1996 `e-min`/`e-limit`; EN 1990 `sls-limits` via series). Lines not read in full are not confirmed.

## Not covered

- Per-constructor limit verification for every helper-built diagram (about 40 constructors). Flagged only where the block itself lacks a `Limit`.
- Mutation folders, `📈️reliability-math`, and the `📇️iso16757` checks (they contain no `Diagram2d`).
- No rebuild or test was run (read-only, per the preamble).

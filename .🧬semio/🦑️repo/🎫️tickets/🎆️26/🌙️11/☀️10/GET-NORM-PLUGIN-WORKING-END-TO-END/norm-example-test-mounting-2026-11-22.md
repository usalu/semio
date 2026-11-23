# Norm example test mounting (2026-11-22)

Status: partial. Families verified by a real `repo cargo ... test -p semio-s-artifact-norm-<family>-examples --offline` run are marked VERIFIED. Families marked PREPARED have their files edited and mounted but the verifying run was blocked by host admission and the shared component build chain (single runs took 30 to 60 minutes) and has not completed.

Test convention: one `🧪️tests/🧩️example/🦀️.rs` per example, mounted from the example crate root (or the example's own `🦀️.rs`) with `#[cfg(test)] #[path]`; it decodes `PRIMARY_TEXT` through the family io crate, runs the family `evaluate`, and asserts the claimed verdict (compliant: no failing check; failing: the claimed checks fail and every failing check has remedies). Dev-dependencies `-inferences` and `-io` were added to the example manifests where missing.

## en1990 (VERIFIED, 14 passed)
Examples: accidental-seismic-compliant/-failing, fatigue-compliant/-failing, high-consequence-office, road-bridge-compliant/-failing (7). Before: 1 trivial test (non-empty asset). After: 7 examples x 2 tests (decode equals `reference_snapshot()`, verdict). Stale `inference_*` duplicates removed. The root-level `mod example` pointing at fatigue-failing was replaced by per-example mounts.

## en1997 (VERIFIED, 3 passed)
Examples: demo, compliant, noncompliant. Before: 0 mounted tests. After: 3. Fix: `demo` and `compliant` failed `en1997.gi-01.pile.pile-P1` (declared investigation depth 12 m, but a 14 m pile needs 14 m + max(5 m, 3 D_b) = 19 m, EN 1997-2 B.3). Set `investigation-depth=20m` in both DSL assets and regenerated both `🎒️.pack.semio` twins from the decoded DSL.

## din16798 (VERIFIED, 3 passed)
Examples: compliant-office, residential-method3, noncompliant-office. Before: 0. After: 3.

## en1991 (PREPARED)
Existing mounted tests strengthened (noncompliant now asserts remedies, compliant asserts non-empty report). Stale `🧬️subjects/🧪️tests/🔬️unit` asset dump generator replaced by a mounted test that the committed DSL assets decode to the named subjects `de_office_compliant()` and `multi_fail_noncompliant()`.

## en1999 (PARTLY VERIFIED: 1 passed, 1 failed)
Run result: `noncompliant-multi-fail` passes (claimed checks 6.2.5, 7.2, 8.5 bolt, 8.6 weld, 1-3 fatigue, 1-5 shell fail, all failing checks have remedies). `aluminium-roof-purlin` (claimed compliant) genuinely FAILS 10 checks, so the new test fails by design until the example is corrected: `7.2.defl.purlin-1` (about 44 mm characteristic against L/200 = 30 mm over 6 m; a 5 m span would pass), `8.5.gauge.support-weld`, `1-2.fire` (3 checks), `1-4.bend/nm/defl.sheet-pass`, `1-5.ring/int.shell-pass`. The follow-up run to print numbers for these never got admitted. Fixing needs the DSL asset, the code-built `compliant_roof_purlin()` snapshot and the pack twin changed together.

## en1993 (PREPARED, dump test only)
`heb240-compliant` mounted with a temporary `[DEBUG]` dump test that must be turned into the compliant assertion; `high-strength-connection` already mounted.

## en1994 (PREPARED, dump tests only)
composite-floor-beam, composite-floor-beam-failing, composite-bridge-girder have temporary `[DEBUG]` dump tests mounted; need verdict assertions.

## en1998 (PREPARED, dump tests only)
seismic-multipart, seismic-multipart-fail, seismic-rc-frame-fail mounted with temporary dump tests; seismic-rc-frame has an appended dump test.

## din4108, din18599, iso16757, vdi3805 (PREPARED)
- din4108: failing-thin-insulation now also asserts remedies.
- din18599: noncompliant-detached now asserts remedies; cooled-office untouched.
- iso16757: dead duplicate `🎬️demo/🧪️tests/🧩️example` deleted (the mounted `📚️example` twin stays).
- vdi3805: the 11 conforming `blatt-N` tests and `nonconforming` were unmounted and referenced the crate by its external name; they now use `super::PRIMARY_TEXT` and are mounted from each example's `🦀️.rs`. `semio-s-plugin-norm-compliance` added as dev-dependency (blatt-22 test).

## Not touched
en1992, en1996, din18599 cooled-office.

## Open
Fix the en1999 purlin example (see above). Remove every `[DEBUG]` dump test (en1993, en1994, en1998) after converting them to verdict assertions; run each family once.

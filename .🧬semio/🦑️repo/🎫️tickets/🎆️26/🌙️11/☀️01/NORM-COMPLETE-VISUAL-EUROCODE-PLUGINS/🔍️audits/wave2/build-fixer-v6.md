# build-fixer v6: case-kit (en1992 run 17, 32 errors)

Root cause: WARNINGS-CLEANUP `apply.py` applied rustc `hidden lifetime parameters` suggestions twice on the same lines (stale warnings.json against already-edited files): `Extend<'_>` became `Extend<'_><'_>`, `Meas<'_, C>` became `Meas<'_, '_, C>`, and `obstacles: &[..]` became `obstacles:<'_> &[..]`. The 11 E0061 arity errors and the E0425 were consequences of the parse failures (the broken signatures were dropped by parser recovery, so callers saw fewer parameters); no signature or caller was actually removed and no concurrent edit was involved.

## Fixes (all under `✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit/`)
| file:line | error | cause | fix |
|---|---|---|---|
| 🗂️design-case/🧊️forms/🦀️.rs:512 `extent_with` | 3 syntax errors | `Extend<'_><'_>` | `Extend<'_>` |
| same :541 `handles_with` | 3 syntax | same | same |
| same :668 `draw_with` | 3 syntax | same | same |
| same :712 `signature_with` | 3 syntax | same | same |
| 🗂️design-case/🧊️forms/🗺️layout/🦀️.rs:14 `origins_with` | 3 syntax | same | same |
| 🧪️case-laws/🦀️.rs:85 `assert_case_layout_clear` | 2 syntax + E0425 :92 `obstacles` | `obstacles:<'_> &[[f64;4]]` | `obstacles: &[[f64; 4]]` |
| forms/🦀️.rs:381 `soil`, :398 `strata`, :483 `notch`, :499 `with_load` | 4 x E0107 | `Meas<'_, '_, C>` (struct `Meas<'a, C>`) | `Meas<'_, C>` |
| forms/🦀️.rs:509, 538, 665, 674, 709; layout:11, :15; case-laws:16, :22, :87 | 11 x E0061 | parse recovery of the broken signatures above | none needed; callers already pass the right arity |

## Sweep
- Grepped the whole norm tree and `✏️s/🔌️plugins` for `<'_><`, `:<'_>`, `(<'_>`, `, <'_>`, `&<'_>`, `'_, '_, X`: no remaining hits (remaining `'_, '_>` hits are genuine two-lifetime types).
- All 38 lifetime edits of the cleanup report re-checked against their struct definitions: `Meas<'a, C>`, `Extend<'a>`, `HandleSpec<'a>`, `ConeInput<'a>` all take exactly one lifetime; only projection-kit and `🧮️mechanics/🔩️fastening` were touched.
- All 16 DEL (unused import / extern crate) edits re-checked for uses elsewhere in the crate: one real regression found and fixed:
  - `⚖️compliance/🦀️.rs`: `extern crate semio_framework_os_kernel as dsl;` had been deleted but `🧪️tests/🔬️unit/🦀️.rs:239` uses `dsl::DslArtifact` (cfg(test)). Restored as `#[cfg(test)] extern crate ... as dsl;`. (`schema`, `store`, `vcs`, `value_derive` aliases in compliance/core, results-config, registry-contract confirmed unused; foundation `norm` alias is present.)
  - framework value crates: removed imports re-checked (`Arc`, `size_of`, `NativeEncodeControl`, `PagedMapReceiver`, `format_ident`, `extern crate self` in projection): each is imported locally where used or only referenced by external crates; no damage found.
- Not verified by compiler (frozen: no cargo/rustc); rustfmt parse check unavailable (no rustup default toolchain).

# Wave 12 — 🧊️obj 🟪️stl ☁️ply ☁️las 🧊️gltf 🎒️zip 🗜️deflate 📊️csv 📑️tsv 🌦️epw 🔣️json 💾️binary

14 subsets, 14 cases. Every command below was actually run from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` and every quoted `[test]` line is real output read
from the tool's own stdout, never through a pipe's exit code.

## Verified final state

| case | oracle exhaustive | contract |
|---|---|---|
| `mutate-obj-3-0` | `executed=45 passed=44 failed=1` | 0 breaches |
| `mutate-stl-ascii` | `executed=15 passed=15 failed=0` | 0 breaches |
| `mutate-ply-1-0` | `executed=21 passed=21 failed=0` | 0 breaches |
| `mutate-las-1-0` | `executed=31 passed=31 failed=0` | 0 breaches |
| `mutate-gltf-2-0` | `executed=15 passed=15 failed=0` | 0 breaches |
| `mutate-zip-2-0` | `executed=15 passed=15 failed=0` | 0 breaches |
| `mutate-zip-2-0-iso21320` | `executed=17 passed=17 failed=0` | 0 breaches |
| `mutate-deflate-rfc1950` | `executed=11 passed=11 failed=0` | 0 breaches |
| `mutate-csv-rfc4180` | `executed=13 passed=13 failed=0` | 0 breaches |
| `mutate-tsv-iana` | `executed=15 passed=15 failed=0` | 0 breaches |
| `mutate-epw-energyplus` | `executed=27 passed=27 failed=0` | 0 breaches |
| `mutate-json-rfc8259` | `executed=15 passed=15 failed=0` | 0 breaches |
| `mutate-json-rfc8259-i-json` | `executed=22 passed=22 failed=0` (python host) | 0 breaches |
| `mutate-binary-raw` | `not-exercised=1` (recorded no-oracle decision) | 0 breaches |

`bun ./📜️script.ts contract --owner 🗄️stdio` reports 2 high-priority breaches repo-wide, both in
`mutate-png-1-2` (another session's file). None from this slice.

`parity=0/0` everywhere — the Rust SUBJECT phase still does not compile
(`ManuallyDrop<Option<RetainedJobPayload>>` migration in `semio-framework-job`), so nothing here is
an oracle-versus-subject claim.

## The one failure left standing, deliberately

`mutate-obj-3-0 :: inverse-remove-face`

```
inverse law violated: applying "remove-face" and then its own inverse did not restore the
original — $.vertexCount is 8577, expected 8576
```

**Indicts our codec, not `tobj` and not the fixture.** Face 16127 belongs to `g band-2` and
`o pattern-sphere`. Removing it necessarily drops it from both, and `InsertFace` — the whole of the
inverse `ObjMutation` declares — carries a face but no membership, so the restored face lands in no
band and no object and `tobj` reads it as a fourth model. `Mutation::inverse` returns `Vec<Self>`,
so the fix is available inside the existing contract (`[InsertFace, SetGroup, SetObject]`), but
`🧊️obj/…/🧬️schema/🧬️mutations/🦀️component.rs` returns the single `InsertFace`. Documented in the
feature's own ⚠️ paragraph. Not fixed here because `semio-s-plugin-stdio` does not compile either
(`semio-framework-ui-contract`, another session), so a production change could not be verified.

## Real defects found and fixed

1. **`mutate-obj-3-0` addressed four groups/objects the fixture does not contain.** The feature
   claimed "a small real `apex` object/group carved out of band-0's own first 3 real faces"; the
   committed mesh carries only `o pattern-sphere` and `g band-0/1/2`. `remove-group apex-band`,
   `remove-object apex`, `set-object north-cap`, `set-group equator` all named fabrications. Three of
   the four w11 failures were that. Rows now name the real bands, and the adapter reads each one's
   membership out of the document (`membership_of`), which fails loudly on a name the file lacks.
2. **The OBJ oracle's renderer invented an `s off` the file never had.** `render` bootstrapped its
   smoothing state from `!have_smoothing`, so a model with zero declared smoothing ranges emitted one
   anyway and the reparse recorded it — decode/re-encode was not projection-stable. Only visible once
   smoothing entered the projection. Fixed by emitting `s` exactly at the range starts `parse`
   records.
3. **The OBJ oracle left dangling face indices in `g`/`o`/`usemtl`/`s` on insert/remove.** Now
   cascaded, with two unit tests over an interior index.
4. **`json-rust` 0.12 `impl From<f64> for JsonValue` is not round-trip exact.** Reproduced
   standalone: `2.7000102824824506` dumps as `…507`, `-8.881784197001252e-16` as `…253e-16`; 2 of 9
   probed values moved. Worked around via `json::parse` of Rust's own `{:?}` shortest form.
5. **`json-rust` 0.12 `as_f64()` is not exact either.** It recomputes `mantissa * 10^exponent` in
   floating point; the fixture's `-1.3283902924697095e-17` surface normal reads back as `…097e-17`.
   Worked around via `dump().parse::<f64>()`. Together, 4 and 5 were the w11 `inverse-set-snapshot`
   failure — a true report about the reference library. Both pinned by unit tests that start failing
   if a later release fixes the crate.

## Unobservable rows made observable

The projections of two subsets could not see most of what their vocabularies mutate.

* **obj**: 14 of 22 kinds moved nothing under the shared `semantic-mesh-v1` — `tobj` triangulates,
  re-indexes per `o`/`g` model, and drops every declared row no face references. New subset-owned
  profile `semantic-obj-3-0-v1`; new `oracle_document_projection` (declared `v`/`vt`/`vn` counts with
  per-component extent and totals, `mtllib`, `g`/`o` spans, `usemtl`/`s` run starts, retained
  comments) composed with the `tobj` reading. All 22 kinds now move it.
* **stl**: `set-solid-name` and `set-triangle-normal` moved nothing — `solidName` is writer freedom
  under `semantic-mesh-v1` and normals were not projected at all. New `semantic-stl-ascii-v1`; the
  projection carries the solid name and the explicit per-facet normals. Doing so required the oracle
  to stop emitting BINARY: `stl_io`'s writer hardcodes a zeroed 80-byte header and binary STL has no
  name field, so it discarded the name on every kind and this `ascii` subset's oracle never once
  produced the grammar it is filed under. It now writes the ascii document itself (the OBJ
  precedent), with `stl_io` still the independent reader. `no-mutation` was `Ok(input.to_vec())` — a
  byte hand-back — and is now a real parse and re-emission.

## Observability asserted in role, everywhere

New shared law `mutation_is_observable_within` in `🧪️oracle/⚖️law/🦀️component.rs` (merged with the
`mutation_is_observable` a peer session added in the same file, whose signature and call sites are
unchanged — the new one is the profile-aware form and the old one delegates to it). Wired into all
12 Rust `mutate_oracle` handlers in this slice. Every kind except `no-mutation` must now move the
projection under the case's own profile — the check that would have caught the OBJ fabrication.

## Audited and found honest — no change needed

`ply` (real `vertex`/`face`/`edge` elements, every Examples row addresses real content), `las`
(fixture re-read byte-by-byte: 8,448 points, LAS 1.0, 2 VLRs, classification histogram
`{2:1157, 3:1600, 4:2934, 5:1600, 6:1157}` matching the feature's claim exactly), `gltf` (271 nodes,
1 scene, 2 materials — every param addresses a real index), `csv` (51 records × 12 columns, CRLF, as
claimed), `deflate`, `zip`, `zip-iso21320`, `binary-raw`, `json-rfc8259-i-json`.

All four `carrier_is_exact` uses (tsv, epw, zip, zip-iso21320) are justified in the feature text with
a real reason, not contrived around. `binary-raw`'s `zero-length-splice` row is a deliberately named
boundary vector, not an accidental no-op.

Two documentation corrections: the EPW feature title said "to a real weather file" while its own body
says no real EPW exists in this repository, and its blocker note named the stale
`📡️spr/🧵️channel` cycle.

## Not in scope / not done

* Production `ObjMutation::inverse` — the fix for the standing failure. `semio-s-plugin-stdio` does
  not compile (`semio-framework-ui-contract`, another session), so it could not be verified.
* `cargo test --features oracles --lib` over the whole oracle crate, once the concurrent sessions'
  in-flight breakage cleared (jpg `number()` arity, xlsx `BytesText::unescape`, bmp test-module
  borrow — all theirs, all transient):

  ```
  test result: FAILED. 340 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out; finished in 134.77s
  test artifacts::txt::…::every_feature_row_inverts_back_to_the_real_document ... FAILED
  test artifacts::xlsx::…::shared_string_kinds_are_a_true_byte_identity ... FAILED
  ```

  **80 of those tests are this slice's, and 0 of them failed.** Both failures are in `txt` and
  `xlsx`, outside it.

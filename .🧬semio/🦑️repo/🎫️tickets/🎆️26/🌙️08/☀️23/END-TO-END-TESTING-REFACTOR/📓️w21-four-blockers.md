# w21 — the four blockers of `📓️w13-final-audit.md`, measured against the LIVE tree

Session 2026-08-25, from 19:12 CEST. HEAD at start `8d9b51f081f42b36722b54f80a5c502d6322f9ca`
(2026-08-25 14:57:24 +0200), dirty tree. Raw logs in `w21-four-blockers/`.

## 0. The one thing a reader must know before the rest

The audit ran against `18adc8cce3` / `9ed590cd87` (2026-08-25 01:06 / 09:16). Between that run and
this one HEAD moved to `8d9b51f081`, and — this is the part that changes what "fix these four" means
— **concurrent sessions had already put repairs for ALL FOUR blockers into the working tree,
uncommitted, between 17:00 and 19:00 today**. Every "already fixed" claim below is settled with
`git diff` against HEAD plus the file's own mtime, never against anybody's prose.

| audit blocker | state found in the live tree | evidence |
|---|---|---|
| 1a. DWG cannot read its own fixture | repair IN FLIGHT by a peer | `🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs` rewritten (mtime 18:45), `+163/-…`: `dwg_decode_r2010_entity_common` now delegates to one shared `decode_r2010_entity_common_fields` that (a) does NOT consume the R13–R2000-only `nolinks` bit and (b) reads the three R2010 visual-style presence bits after `shadow_flags`. That is exactly the two-bit misalignment an `underflow` at entity `0x239` type `77` (LWPOLYLINE) implies. |
| 1b. broken `include_str!` in `dwg ac1024` | ALREADY FIXED, uncommitted | HEAD has `"../../../../🔖️ac1018/…"`; the working tree has one more `../`. Both `include_str!` targets in that file now resolve (checked on disk). |
| 2. `jpg ✳️baseline` fails `identity-round-trip` | ALREADY FIXED, uncommitted | `📷️jpg/…/✳️any/🚪️io/🦀️component.rs` (mtime 18:51) gained `frame_components_of`: the SOF0 sampling factors now come from the decoded frame instead of a hard-coded `1:2x2, 2:1x1, 3:1x1`. The committed scan really is 4:4:4 (`SOF0 prec 8 2275x2560, comps 1/2/3 all H1 V1` — read straight out of the fixture), which is precisely what the old encoder was destroying. |
| 2. `tiff ✳️baseline` `mutate-remove-tile-tags` | ALREADY FIXED, uncommitted | The adapter's empty-`code` branch asserted "the verdict must not change"; that row runs from the `insert-tile-tags` setup state, so the verdict is REQUIRED to change (one code → none). Now asserts `after.is_empty()`. Strictly the stronger claim. |
| 2. `tiff ✳️baseline` `identity-round-trip` | **STILL OPEN — see §2 below** | nobody has touched it |
| 3. five stdio cases with no compiling subject half | ALL FIVE ALREADY FIXED, uncommitted | `jpg`/`stl` adapters: `standards::jfif_1_01` → `standards::v_jfif_1_01`, `standards::ascii` → `standards::v_ascii` (mtime 17:00). `docx`/`xlsx` ✳️strict: `vml_markup` did not exist at HEAD at all — `git show HEAD:…` has zero occurrences — and was ADDED to both production mutation modules at 17:09. The adapters were right; the production vocabulary was missing the helper. |
| 4. `mutate-json-rfc8259-i-json` 0/22 | ALREADY FIXED, uncommitted | (a) the `E0603: crate protocol is private` came from a private helper `protocol_inverse` that wrote `semio_s_plugin_stdio::protocol::Mutation` — `protocol` is `extern crate semio_framework_os_kernel as protocol;` at `📦️glue.rs:14`, i.e. a PRIVATE extern-crate alias. It is gone; the adapter now calls the subset's own `inverse_json_i_json_mutation`. (b) the 22 `adapter has no subject registration` errors were the coordinator dispatching the SUBJECT role to a python adapter that is a reference host only; `📜️script.ts` gained `ownerShipsImplementation`, which dispatches a subject only in a language the owner actually ships a package in, and reports `no-subject-implementation` instead of 22 null projections. |

## 1. The measurement is blocked by a fifth thing, and it is not in the audit

**No stdio subject host can link right now.** `semio-framework-os-kernel` does not compile in the
generated test host's dependency graph:

```
error: future cannot be sent between threads safely
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8321:13
  = help: the trait `Sync` is not implemented for `(dyn ArtifactStoreOwnedDisposer<P, Mutation> + 'static)`
error: could not compile `semio-framework-os-kernel` (lib) due to 2 previous errors
```

This is a live peer's in-flight repair, not a static defect, and it is converging while this note is
being written: the same build reported **10** errors at 19:25, **2** at 19:40, and the file's mtime
moved to 19:40:17 forty-five seconds before it was read. The erasure-table thunk at `:8316`
(`apply_ops_binary_impl`) still declares `Pin<Box<dyn Future … + Send + 'a>>` while its body holds
`&ArtifactStore<P, Mutation>`, which is not `Sync`; the three sibling thunks at `:8295`, `:8309` and
`:8359` have already had their `Send` bound dropped. **Do not edit that file** — measure after it
lands.

Two consequences worth recording:

* Every stdio `parity`/`subject` per-case run in the repository is failing at the same point right
  now, in every session, and the runner reports it as `rust subject host exited 101 without emitting
  results` + `no result stream` while the summary line still reads `executed=2 passed=2 failed=0`.
  That is wave 12's remedy #7 / the audit's remedy #2, still open at `📜️script.ts:520-541`
  (`executeOne` returns `results: []`), and it is what made the audit's five silently-unmeasured
  cases possible.
* `cargo check -p semio-framework-os-kernel --lib` and `--features deflate` are both **exit 0** from
  the ROOT workspace, while the identical crate fails inside the generated host's own
  `[workspace]`. `--features sync` is exit 101 (7 errors). So "the os-kernel blocker is cleared" is
  still feature-set-dependent exactly as the audit's §5 says, and the plain root-workspace check is
  not a proxy for "a test host will link".

## 2. The one blocker nobody had claimed, and the fix — `mutate-tiff-6-0-baseline :: identity-round-trip`

The audit recorded it as

```
identity law violated … $.stripOffsets is "388", expected "412"
```

and attributed it to us. It is ours, but not in the codec: **the case asserted something its own
feature text says is false.** `round_trip` ended with

```rust
let (was, now) = (projection(&base)?, projection(&reparsed)?);
law::round_trip_preserves(&now, &was)?;
```

`base` is the committed scan as some FOREIGN writer laid it out; `reparsed` is our own encoder's
output read back. `encode_tiff_baseline_projection_json` includes `stripOffsets` — an absolute byte
offset into the file — because `set-strip-offsets` is one of the vocabulary's nine kinds and needs
that axis to be observable. And the feature says, in its own second paragraph:

> `encode_tiff` REGENERATES every one of `CORE_STRIP_TAGS` from the raster it is about to write, so
> those four are normalized away on re-serialization

So the assertion could only ever be red: the two writers place their strips at different offsets, and
tolerance is 0. The scenario's `Then` clause claims exactly three things (bytes differ, still
Baseline-conforming, the independent reader agrees on the geometry of both) — the fourth assertion
was never part of the scenario at all.

**Fixed at the cause, without weakening anything** (`🖼️tiff/🧪️tests/mutate-tiff-6-0-baseline/🦀️component.rs`):

* no `ignoreKeys`, no tolerance, no changed `arrays` mode, no swapped fixture, no removed row;
* the axes the feature says travel verbatim — `ifdCount`, `TileWidth`, `TileLength` — are now
  required to survive `base → reparsed` **by name**, one positive claim each, so the case still
  holds our reader/writer to the foreign document where that is meaningful;
* `law::round_trip_preserves` is kept at tolerance 0 with no exempt key and restated as the encoder's
  own FIXPOINT: `decode(encode(decode(encode(x))))` must project exactly as `decode(encode(x))`,
  `stripOffsets` included. A writer that shifted its own offsets on every pass, or a reader that
  drifted, now fails — which the previous form could not detect, because it was already red for a
  reason that is not a defect. This is the same "state it as idempotence on the real bytes" the
  audit's §3 recommends for the HTML `codec_retention_law`.

Net: one false assertion replaced by four true ones.

## 3. A defect in the DWG R2010 entity reader that the in-flight repair does NOT cover

Recorded here rather than fixed, because `🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs` is being
rewritten by another session right now (mtime 18:45) and two writers in one 12,112-line file is how
work gets lost.

**Entity strings are read from the wrong stream.** `dwg_from_r2004_sections` builds the R2007+ string
stream only in the `DWG_TYPE_LAYER` branch:

```rust
let (mut strings, _) = r2010_string_stream(payload, data_end_bit)?;
let layer = dwg_decode_r2010_layer(&mut reader, &mut strings)?;
```

The entity branch passes only the data reader and the handle reader, and
`dwg_decode_r2010_entity`'s `DWG_TYPE_TEXT` arm then does

```rust
let content = reader.read_t()?;
```

`read_t` is a byte-length-prefixed 8-bit string read out of the DATA stream. For AC1021+ every entity
string lives in the separate STRING stream as `TU` (UTF-16, `BS`-prefixed) — exactly the mechanism
`dwg_decode_r2010_layer` already uses correctly, and exactly what `r2010_string_stream` exists for.
On a real AC1024 file this reads the next 16 bits of geometry as a length and then walks off the end,
which is the same `dwg bitstream underflow` signature the LWPOLYLINE failure has.

The same arm is also a large under-read of ODA's TEXT (type 1) layout for R2000+: a `data flags RC`
byte gates elevation, alignment point, oblique angle, rotation, height, generation and the two
alignment enums, and the arm reads `3BD + BD + BD + T` instead. Either the arm is completed against
the specification, or `DWG_TYPE_TEXT` comes out of the `known_entity` set until it is — silently
mis-decoding a TEXT entity into a `DwgGeometry::Text` is worse than skipping it.

## 4. Two things this session did NOT change, and why

**`📜️script.ts:520-541` still swallows a host that will not build.** `executeOne` returns
`results: []`, so the summary line reads `executed=0 passed=0 failed=0` and only the `problems` list
and the exit code carry the truth — the audit's remedy #2 / wave 12's remedy #7. It is what made the
five silently-unmeasured cases of §2.4 possible, and it is what makes EVERY stdio case in the
repository look benign while os-kernel is red (§1). The right shape is to synthesize one `errored`
result per planned scenario when `results.length === 0` and the probe failed. Not done here on
purpose: `📜️script.ts` is a shared file with another session's uncommitted change in it
(`ownerShipsImplementation`), several sessions are mid-run against it right now, and changing the
summary accounting under a running measurement is how two waves' numbers stop being comparable.
Whoever owns the runner next should do it in one change with the per-case budget fix
(`runProbe` at `:530` aborts the whole run on one slow case).

**The DWG R2010 entity reader (§3).** Held by another session.

## 5. A scan that came back clean

The audit's §2.4 class — an adapter importing a production symbol that does not exist — was swept
across every `🗄️stdio` `🧪️tests/<case>/🦀️component.rs`: every name in every
`use semio_s_plugin_stdio::…` was resolved against the artifact tree and the crate's `📦️glue.rs`
module mounts. **No case besides the five the audit already named carries an unresolved import.**
(Thirteen apparent hits were checked by hand and are all real: framework traits re-exported at the
crate root, and per-mutation module directories mounted from `📦️glue.rs` rather than declared in a
`.rs` file.)

## 6. Measured, per case — `bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case <case>`

Every `[test]` line below is copied verbatim from the tool's own stdout, and every exit code was read
from the tool's own exit status — never through a pipe. Log: `w21-four-blockers/run3.txt`.

The subject halves could not link until 20:52:32 (§1); `run1-oskernel-blocked.txt` and `run2.txt` are
the two runs that proved it, kept rather than deleted.

| case | audit (2026-08-25 09:25) | now | exit |
|---|---|---|---|
| `create-and-read-jpeg` | `rust subject host exited 101`, `executed=0` — never measured | `executed=4 passed=4 failed=0 errored=0 parity=2/2` | 0 |
| `create-and-round-trip-stl` | `rust subject host exited 101`, `executed=0` | `executed=4 passed=4 failed=0 errored=0 parity=2/2` | 0 |
| `mutate-json-rfc8259-i-json` | `parity 0/22`, 22 × `adapter has no subject registration`, `E0603` in the rust host | `executed=44 passed=44 failed=0 errored=0 parity=22/22` | 0 |
| `mutate-docx-ecma-376-strict` | `rust subject host exited 101`, `executed=0` | `executed=42 passed=42 failed=0 errored=0 parity=19/21` | 1 |

**`mutate-docx-ecma-376-strict` 19/21 is the first differential number this case has ever produced,
and it found a real writer defect on its first run — fixed here, at the cause.** Both failing rows
(`mutate-set-snapshot`, `mutate-set-relationship-base`) are the two that stamp the package Strict, and
both diverge on exactly one axis:

```
$.relationshipTypes: array length differs   oracle: 4   subject: 5
  oracle : …/purl.oclc.org/ooxml/officeDocument/relationships/styles
  subject: …/purl.oclc.org/ooxml/officeDocument/relationships/styles
           …/schemas.openxmlformats.org/officeDocument/2006/relationships/styles   ← extra
```

Read out of the two raw packages rather than inferred (`…-subject-rust/mutate-set-relationship-base.subject.raw`):

```xml
<!-- oracle  word/_rels/document.xml.rels -->
<Relationship Id="rId1" Type="http://purl.oclc.org/ooxml/officeDocument/relationships/styles" Target="styles.xml"/>
<!-- subject word/_rels/document.xml.rels -->
<Relationship Id="rId1" Type="http://purl.oclc.org/ooxml/officeDocument/relationships/styles" Target="styles.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
```

Cause, in `✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`'s `sync_main_part`: the
`officeDocument` presence check accepts BOTH conformance classes —

```rust
let has_office_document_rel = … r.rel_type == REL_TYPE_OFFICE_DOCUMENT || r.rel_type == STRICT_REL_TYPE_OFFICE_DOCUMENT;
```

— but the styles check compared against the transitional type alone. Once `set-relationship-base`
retargets the styles relationship to the Strict base, the writer concludes the package has no styles
relationship and **appends a second, transitional-typed one beside the strict one it just failed to
see**. That is package corruption on every Strict round trip, and it is why `_rels/.rels` came out
right while `word/_rels/document.xml.rels` did not.

Fixed by giving Strict's styles type the same first-class constant its `officeDocument` sibling
already had (`STRICT_REL_TYPE_STYLES` in `✳️any/🚪️io/🦀️component.rs`) and using it in both directions:

* `sync_main_part` no longer invents a duplicate relationship;
* `decode_docx` resolves the styles part through either type. That half is the same defect facing the
  other way and nothing had caught it: a genuinely Strict package decoded with **no styles at all**,
  because the resolver only knew the transitional relationship type. `xlsx` does not share either bug
  — its serializer already matches on the `/worksheet` suffix, which is base-agnostic by construction.

Nothing was ignored, tolerated or normalized: the `semantic-ooxml-docx-strict-v1` profile, the case's
rows and its fixture are untouched.

## 7. The rest of the sweep — `w21-four-blockers/run4.txt`

Between run 3 and run 4 the machine ran out of disk (`ENOSPC` at 21:12, mid-`mutate-xlsx-ecma-376-strict`),
which invalidated five cases; they were re-run after ~23 GB of this session's own stale scratch cargo
target directories were removed. `run3.txt` keeps the aborted attempt rather than hiding it.

| case | audit (2026-08-25 09:25) | now | exit |
|---|---|---|---|
| `mutate-xlsx-ecma-376-strict` | `rust subject host exited 101`, `executed=0` — never measured | `executed=38 passed=38 failed=0 errored=0 parity=13/19` | 1 |
| `mutate-jpg-jfif-1-01-baseline` | `identity-round-trip` FAILED (`$.components[0] is "1:2x2", expected "1:1x1"`) | `executed=21 passed=21 failed=0 errored=0` | **0** |
| `mutate-tiff-6-0-baseline` | 2 failures (`mutate-remove-tile-tags`, `identity-round-trip`) | `executed=19 passed=18 failed=1` → see §8 | 1 |
| `mutate-dwg-ac1018` | **all 7 scenarios FAILED**, `R2004 entity 0x239 type 77: dwg bitstream underflow` | `executed=7 passed=7 failed=0 errored=0` | **0** |
| `mutate-dwg-ac1024` | **all 7 scenarios FAILED**, same error | `executed=7 passed=7 failed=0 errored=0` | **0** |

**The DWG blocker is closed: 14 of 14 scenarios green, from 0 of 14.** The repair is the peer's
R2010 common-entity fix described in §0 — dropping the R13–R2000-only `nolinks` bit and reading the
three R2010 visual-style presence bits — and the real 148,638-byte AC1024 file now parses. The
`include_str!` half is closed too. §3's TEXT-arm defect is still there and still unexercised by this
fixture.

**`mutate-jpg-jfif-1-01-baseline` is closed.** Its five class axes and both laws now hold over the
real 2275×2560 scan; the encoder honours the frame's own sampling factors, which the committed file
proves are `1:1x1 2:1x1 3:1x1` (read out of its SOF0 directly).

### `mutate-xlsx-ecma-376-strict` 13/19 — one function is the cause of all six, and of the audit's §2.2(4)

Not fixed here (see §9). The diagnosis is exact. Five of the six rows differ by the mutation simply
not surviving the write:

```
mutate-set-main-namespace        $.mainRootAttributes[0].value  oracle …purl.oclc.org/ooxml/spreadsheetml/main   subject …openxmlformats.org/spreadsheetml/2006/main
mutate-set-conformance-attribute $.mainRootAttributes len       oracle 3 (conformance,xmlns,xmlns:r)             subject 2 (xmlns,xmlns:r)
mutate-set-relationships-namespace  $.mainRootAttributes[1].value  oracle …purl.oclc.org/…/relationships          subject …openxmlformats.org/…/relationships
mutate-set-worksheet-content-type   $.parts[9].contentType         oracle application/xml                          subject …spreadsheetml.worksheet+xml
inverse-remove-conformance-attribute  (the same six differences as set-conformance-attribute)
mutate-set-snapshot                   (13 differences — the whole stamp)
```

`encode_xlsx` calls `regenerate_workbook_parts`
(`📕️xlsx/…/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs:245`), which UNCONDITIONALLY does

```rust
opc.parts.retain(|p| !p.path.starts_with("xl/worksheets/") && p.path != WORKBOOK_PART && p.path != SHARED_STRINGS_PART);
…
opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, workbook_to_xml(workbook, &rids));
…
opc.set_part(&path, WORKSHEET_CONTENT_TYPE, bytes);
```

— it deletes and re-renders the workbook, the shared-string table and every worksheet from the typed
`XlsxWorkbook`, and re-stamps their content types. Any byte the ✳️strict conformance vocabulary wrote
into those parts (the root `conformance` attribute, either retargeted namespace) is discarded on
write, and `set-worksheet-content-type` is overwritten by the `set_part` call itself. `docx` does not
have this defect because `sync_main_part` guards every rewrite with `part_already_projects` — it
re-renders a part only when the typed model no longer projects onto the bytes that are there.

The remedy is to give xlsx the same guard. `workbook_relationships` is NOT the problem — it already
preserves unrelated relationships, so the audit's §2.2(4) "the writer drops `styles` and `theme`"
needs re-measuring against `mutate-xlsx-ecma-376-transitional` after this lands.

## 8. `mutate-tiff-6-0-baseline :: identity-round-trip` — the failure MOVED, and the second form is the interesting one

§2 fixed the `stripOffsets` assertion. The re-run then failed on the OPPOSITE law:

```
byte pass-through: the re-encoded output is bit-identical to the 17473180-byte input,
which a decode/encode that never parsed anything would also produce
```

That is not a regression — it is the peer's `TiffIfd::pixels` work landing. `encode_tiff` now
reproduces the committed scan **byte for byte**, so `law::reparsed_not_copied` cannot hold.

The important question is whether byte-exactness is a RESULT or a SMELL, and it was settled by
reading how the fixture was made, not by assuming. `derive_real_world_fixture`
(`🖼️tiff/…/✳️any/🧪️oracle/🦀️component.rs:825`) builds IFD 0 from the real 2275×2560 JPEG scan through
the **registered `image` encoder**, IFD 1 from the real rathaus PNG through the registered `png`
decoder, and writes the file with the ORACLE module's own independent `write_tiff`. The fixture is
the REFERENCE writer's output, not ours — `git log` confirms it has not been touched since
`3c7d3c108c` (2026-08-23), well before any of this. So "this repository's writer converges on the
reference writer's exact bytes for a real 17 MB two-IFD document" is a genuine and much stronger
statement than "the bytes differ", and `law::carrier_is_exact` is the law that states it — its own
doc comment names this exact situation as its third admissible case.

Byte-exactness is also precisely where a `read`/`write` shortcut would hide, so the scenario does not
rest on the structural argument that `encode_tiff(&TiffSnapshot)` cannot see the input bytes. It
DEMONSTRATES it: one byte of the decoded raster is flipped, re-encoded, and the output is required to
differ from the input. A codec that smuggled bytes would hand the input back and fail there. The
feature's `Then` clause was rewritten to say all of this, so the scenario still claims exactly what
the adapter asserts.

**Result: `executed=19 passed=19 failed=0`, exit 0.**

## 9. Final per-case table

| case | audit, 2026-08-25 09:25 | now | Δ |
|---|---|---|---|
| `mutate-dwg-ac1018` | 0/7 scenarios — `dwg bitstream underflow` | **7/7 passed, exit 0** | +7 |
| `mutate-dwg-ac1024` | 0/7 scenarios — same | **7/7 passed, exit 0** | +7 |
| `mutate-jpg-jfif-1-01-baseline` | 20/21, `identity-round-trip` failed | **21/21 passed, exit 0** | +1 |
| `mutate-tiff-6-0-baseline` | 17/19, two failures | **19/19 passed, exit 0** | +2 |
| `create-and-read-jpeg` | host never compiled, `executed=0` | **4 executed, 4 passed, parity 2/2** | 0/0 → 2/2 |
| `create-and-round-trip-stl` | host never compiled, `executed=0` | **4 executed, 4 passed, parity 2/2** | 0/0 → 2/2 |
| `mutate-docx-ecma-376-strict` | host never compiled, `executed=0` | **42 executed, 42 passed, parity 21/21** | 0/0 → 21/21 |
| `mutate-xlsx-ecma-376-strict` | host never compiled, `executed=0` | **38 executed, 38 passed, parity 13/19** | 0/0 → 13/19 |
| `mutate-json-rfc8259-i-json` | parity 0/22, 22 errors | **44 executed, 44 passed, parity 22/22** | 0/22 → 22/22 |
| regression check `mutate-docx-ecma-376` | not in the audit's imperfect list | **54 executed, 54 passed, parity 27/27, exit 0** | unchanged |
| regression check `mutate-docx-ecma-376-transitional` | 8/13 | **13/13, exit 0** | +5 (peer's OOXML sweep) |

**Differential comparisons over the nine assigned cases: 0 / 22 at the audit → 60 / 66 now.** Four of
them (`mutate-dwg-ac1018`, `mutate-dwg-ac1024`, and the two `✳️baseline` cases) carry a recorded
no-oracle decision and so contribute `0/0` to that ratio at both ends; their evidence is the subject
phase, and it went from **17 of 54 scenarios failing to 0 of 54**. The 6 comparisons still divergent
are the `xlsx` ✳️strict rows of §7, all of them one function.

## 10. Everything this session changed, and the weakening check

| file | change |
|---|---|
| `🖼️tiff/🧪️tests/mutate-tiff-6-0-baseline/🦀️component.rs` | `identity-round-trip`: `round_trip_preserves(base↔reparsed)` → three named-axis claims + encoder fixpoint (§2); `reparsed_not_copied` → `carrier_is_exact` + a live one-byte-perturbation proof (§8) |
| `🖼️tiff/🧪️tests/mutate-tiff-6-0-baseline/component.feature` | that scenario's `Then` clause, so it states what the adapter now asserts |
| `📜️docx/…/✳️any/🚪️io/🦀️component.rs` | new `STRICT_REL_TYPE_STYLES` constant, the counterpart of `STRICT_REL_TYPE_OFFICE_DOCUMENT` |
| `📜️docx/…/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs` | `sync_main_part` recognizes a Strict styles relationship instead of appending a duplicate transitional one |
| `📜️docx/…/✳️any/🚪️io/📥️import/🧩️deserializers/🦀️component.rs` | `decode_docx` resolves the styles part through either conformance class |

**Weakening check, the same six ways `📓️w13-final-audit.md` §3 ran it, over this session's diff only:**
no `ignoreKeys` entry was added anywhere; no `tolerance` was widened and no `*_within` variant was
introduced; no `arrays` mode changed; no `🔣️component.json` comparison profile was touched at all; no
`asset://`/`shared://`/`local://` line moved and no fixture byte changed
(`🖼️abbau-aufbau-masterarbeit-grundriss.tiff` is still at `3c7d3c108c`, 2026-08-23); no scenario, row
or `Examples` entry was removed — the feature diff is one `Then` line, rewritten to claim MORE. Two
law calls changed, both to strictly stronger statements, both argued in place at the call site.

`bun ./📜️script.ts contract` reports the same 2 breaches before and after this session's edits
(`unmanaged-tests` in `🧰️framework` 42/35 and `✏️s` 4/1) — both are other sessions' stray executable
test files, neither is from this work, and both are new since the audit's "all fourteen rule ids at
zero".

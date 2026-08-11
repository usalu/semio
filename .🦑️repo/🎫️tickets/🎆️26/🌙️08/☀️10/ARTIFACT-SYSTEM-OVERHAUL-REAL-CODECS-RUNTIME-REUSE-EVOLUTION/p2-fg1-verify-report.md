# P2-FG1 Independent Verification Report

Verifier: independent sub-agent, trusting nothing from the 6 FG1 self-reports without re-checking
disk and re-running tests. Scope: `md`, `xml`, `obj`, `stl`, `dxf`, `step` (ap214), `ifc` (v4) — 7
standards across 6 fan-out agents (step+ifc were one agent).

## Headline result

All 7 standards' own scoped test filters pass with **0 failures**, matching every self-report's
claimed counts exactly. The full crate suite is currently **1714 passed, 0 failed, 1 ignored**
(strictly better than every self-report's own snapshot-in-time count, because the concurrent churn
those reports observed mid-session has since settled — this is a live shared tree). All 42
`.grammar.semio`/`.protocol.semio` files (6 per standard × 7) exist, use the real dialect header
syntax, and were independently spot-read against each artifact's own Rust parser. Fixtures are
real. 5-role `LanguageSpec` registration is complete for all 7. The 5 known authoring pitfalls do
not recur anywhere in the 42 files (grepped, not just read).

**One substantive, well-evidenced shortfall found**: 4 of 7 standards (`stl`, `obj`'s diff facet,
`step`, `ifc`) left `DiffCodec`/`OpBinary` on the literal `print_diff()/print_op().into_bytes()`
text-as-binary shortcut instead of performing the real binary-frame upgrade the recipe's own
checklist explicitly mandates ("expect to do a real upgrade here for almost every standard, not
just check") — even though 3 sibling standards in the *same* wave (`md`, `xml`, `dxf`) successfully
did exactly this upgrade against comparably-recursive types, proving it was achievable. Details in
§3.

## 1. Per-standard scoped test re-run (independently executed, not copied from reports)

| standard | command | result | matches self-report? |
|---|---|---|---|
| md | `cargo test ... "artifacts::md"` | 36 passed, 0 failed, 0 ignored | yes (36/0/0) |
| xml | `cargo test ... "artifacts::xml"` | 36 passed, 0 failed, 0 ignored | yes (36/0/0) |
| obj | `cargo test ... "artifacts::obj"` | 30 passed, 0 failed, 0 ignored | yes (30/0/0) |
| stl | `cargo test ... "artifacts::stl"` | 34 passed, 0 failed, 0 ignored | yes (34/0/0) |
| dxf | `cargo test ... "artifacts::dxf"` | 25 passed, 0 failed, 0 ignored | yes (25/0/0) |
| step | `cargo test ... "artifacts::step"` | 106 passed, 0 failed, 0 ignored | yes (106/0/0) |
| ifc | `cargo test ... "artifacts::ifc"` | 74 passed, 0 failed, 0 ignored (incl. untouched v2x3's 46) | yes (74/0/0) |

All 6 conformance-law tests (`committed_facet_files_parse`, `grammar_conformance_law`,
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`fixture_honesty_law`) independently confirmed present and green in every one of the 7 outputs
above (visible by name in the test list for each run).

## 2. Grammar/protocol files — real dialect, real content

Confirmed all 42 files exist (`find` for `*.grammar.semio`/`*.protocol.semio` under the 7
standard trees returned exactly 42). Headers spot-read for all 42: every one starts with
`dialect grammar`/`dialect protocol` on its own physical line, `grammar <id>`/`protocol <id>`,
`extension`/`schema`, `start` — no residual ABNF (`%x`, `1*`, `/`-alternation, single-line
`dialect grammar stdio.x.snapshot` headers) anywhere.

STEP/IFC-specific check (task item 7): read both artifacts' snapshot grammar files in full —
both declare `comment line none` + `comment block "/*" "*/"` + `string single doubled`, and model
`instance = "#" INT "=" instance-body ";"` explicitly (plus the spec-legal COMPLEX-instance
`"(" entity-record+ ")"` form). Cross-checked against the real fixtures:
`🗣️example.dsl.semio` for both step and ifc contain genuine `#1=IFCPROJECT('gid-project',#2,'Demo
Project');` / `#1=CARTESIAN_POINT('',(0.,0.,0.));` lines, and `grammar_conformance_law`/
`fixture_honesty_law` pass for both — i.e. the grammar demonstrably recognizes a real `#N=TYPE(...)`
line as an entity instance, not a stripped comment. Confirmed correct, not merely claimed.

Also confirmed genuine trailing-dot `FLOAT` usage (`0.`, `10.`) in the step fixture and the
`DOTENUM`/`string single doubled` header directives, matching the recipe's M1-feature citations.

## 3. `DiffCodec`/`OpBinary` binary-frame upgrade status — INDEPENDENTLY VERIFIED, discrepancy found

Grepped each artifact's own `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs` for the
`encode_diff`/`encode_op` impl bodies directly (not trusting the report's narrative):

| standard | `OpBinary::encode_op` | `DiffCodec::encode_diff` | upgraded (both)? |
|---|---|---|---|
| md | real header+tag+opaque-payload frame (new binary primitives, `pub(crate)` recursive `enc_*_bin`) | same, real frame | **yes** |
| xml | real header+tag+opaque-payload frame | real header+flags+opaque-payload frame | **yes** |
| dxf | real header+tag+opaque-payload frame | real header+flags+opaque-payload frame | **yes** |
| obj | `dsl::variants_binary::encode_op(self)` — genuinely real, derive-based (same pattern `binary`'s own pilot `BinaryMutation::OpBinary` uses verbatim) | `Ok(self.print_diff().into_bytes())` — **still the literal F6 text-as-binary shortcut** | **partial** — mutations legitimately already-real, diff NOT upgraded |
| stl | `Ok(print_stl_op(self).into_bytes())` — **still the shortcut** | `Ok(self.print_diff().into_bytes())` — **still the shortcut** | **no** |
| step | `Ok(self.print_op().into_bytes())` — **still the shortcut** | `Ok(self.print_diff().into_bytes())` — **still the shortcut** | **no** |
| ifc | `Ok(self.print_op().into_bytes())` — **still the shortcut** | `Ok(self.print_diff().into_bytes())` — **still the shortcut** | **no** |

This matches what each self-report actually *claims* (stl/obj-diff/step/ifc reports are candid
about `diffcodec_binary_upgraded`/`opbinary_binary_upgraded` being `false`) — so there is no
misrepresentation in the reports themselves. The concern is whether `false` was the *correct*
outcome per the recipe.

**Independently checked the cited precedent for legitimacy** (the reports justify leaving the
shortcut in place by citing "the same simplification `WriterDiff`/gif89a/svg's hand-rolled
`DiffCodec`s use"): confirmed real — `☁️ply/🏅️standards/🔖️1.0/…/🔺️diff/🦀️component.rs:947` and
`💬️bcf/🏅️standards/🔖️2.1/…/🔺️diff/🦀️component.rs:867` both genuinely contain
`Ok(self.print_diff().into_bytes())` with an identical doc-comment citing the same precedent chain.
This pattern is real and pre-existing in the codebase, not fabricated by these FG1 agents.

**However**, the recipe doc (`📖️grammar-recipe.md` §4) is explicit and unambiguous on this exact
point: *"Per the P2-W0 census, 100% of stdio's `DiffCodec` impls were still on the text-as-binary
shortcut before this pilot ladder — expect to do a real upgrade here for almost every standard,
**not just check**."* The italicized "not just check" is a direct warning against exactly the
move stl/obj(diff)/step/ifc made: identifying the shortcut, then declining to upgrade it on the
grounds that it's a legitimate pre-existing design elsewhere in the repo. `md`, `xml`, and `dxf` —
three sibling standards in this *same* wave, facing genuinely recursive/nested payloads of
comparable or greater complexity (`MdBlock` self-recursion, `XmlNode` recursion, `DxfEntity`/
`DxfBlock` nested collections) — did perform the real upgrade: a real fixed `format u8`/`tag u8`
(or `flags u8`) header plus new hand-written recursive binary primitives (`enc_*_bin`/`dec_*_bin`)
for the payload, per §2.5's worked pattern. That proves the upgrade was mechanically achievable for
this shape of artifact; the four standards that skipped it did so by policy choice, not because a
structural blocker existed. None of §5's consolidated mechanism-gaps table entries document "text-
bytes-verbatim is an acceptable permanent end state" as a sanctioned exception — the closest entry,
`txt-diffcodec-spk-container-is-framework-level`, is about the *derive-driven* `.spk` container
path, an unrelated situation (`stl`/`obj`/`step`/`ifc`'s diff types are all hand-rolled, not
`DslDiff`-derived).

**Verdict**: not a test failure (nothing asserts this at the conformance-law level — round-trip
laws pass regardless of whether the "binary" is genuinely dense or just text bytes), but a real,
verifiable shortfall against the recipe's own explicit checklist item for 4 of 7 standards in this
verification's scope. Recommend re-opening `stl`, `obj` (diff facet only), `step`, and `ifc` to
perform the same header+opaque-recursive-binary-payload upgrade `md`/`xml`/`dxf` already
demonstrate is achievable, rather than accepting the WriterDiff/ply/bcf precedent as sufficient
justification for standards this ticket wave explicitly owns.

## 4. Fixtures — real, independently read

All 7 `🗣️example.dsl.semio` files start with the mandatory `semio stdio.<artifact>.dsl v1` preamble
followed by genuine, format-correct content (not fake placeholders):
- md: real CommonMark (`# Title` etc.)
- xml: real XML declaration + DOCTYPE
- obj: real Wavefront statements (`mtllib`, `v`)
- stl: real ASCII-STL (`solid demo` / `facet normal`)
- dxf: real DXF group-code/value pairs (`0` / `SECTION`)
- step: real Part-21 (`ISO-10303-21;` / `HEADER;`)
- ifc: real Part-21 IFC4 (`ISO-10303-21;` / `HEADER;`)

All 7 `🎒️example.pack.semio` files exist on disk with plausible non-trivial sizes (164–636 bytes).
`fixture_honesty_law` passing for all 7 (confirmed in the scoped test runs above) independently
corroborates these are genuine round-trippable encoder output, not hand-typed bytes.

## 5. Registration — confirmed by direct grep of `register_pilot_languages()`

All 7 standards' `⚙️engine/🦀️component.rs` show the full 5-role set (`LanguageRole::Document`,
`::Ops`, `::Diff`, `::Pack`, `::Spr`) actually present in the source, not just claimed. `obj` is
the only one of the 7 that calls `register_schema_specs()` for a real derivable spec
(`ObjSnapshot::__dsl_spec` — `ObjSnapshot` genuinely derives `dsl::DslRecord`, confirmed
plausible given `obj`'s own report's detailed tracing of the flat-record shape); the other 6
correctly skip it with an inline doc-comment reason, consistent with each artifact's own
hand-rolled/recursive-enum types having no derivable `RecordSpec`.

## 6. Five authoring pitfalls — grepped directly, none recur

- Pitfall 1 (bare `()` grouping instead of `{}`): grepped all 42 grammar files for
  `= ... ( ... | ... )` outside quotes — zero hits.
- Pitfall 2 (hand-rolled `{INT|IDENT}*` instead of the `hex` macro): grepped for the literal
  pattern — zero hits in actual productions (all matches were doc-comments explaining why the
  pitfall was avoided).
- Pitfall 3 (production name colliding with `extension`/`use`/`start`/`comment`/`string`): grepped
  for `^(extension|use|start|comment|string)\s*=` — zero hits (xml's report documents catching and
  fixing exactly this during drafting, before commit).
- Pitfall 4 (multi-line production wrap): grepped for orphan lines starting with `|` — zero hits;
  independently corroborated by all 7 `committed_facet_files_parse` tests passing.
- Pitfall 5 (`Prim::Ref` attempting recursion in `.protocol.semio`): grepped all 42 files for
  `Ref(` — zero hits; every recursive/nested payload is modeled as the recipe's mandated
  fixed-header + opaque-`bytes`-tail pattern instead.

## 7. STEP/IFC `#`-comment vs entity-ref collision — confirmed correct

Covered in §2 above: both grammars declare `comment line none` + `comment block "/*" "*/"`, model
`#N=...` as a real production, and this is proven end-to-end (not just asserted) by
`grammar_conformance_law`/`fixture_honesty_law` passing against fixtures that genuinely contain
`#1=...`/`#2=...` entity-reference lines.

## 8. JSON-transfer-ban sweep

Grepped `serde_json::to_vec|from_slice|to_string|from_str|Value` across all 7 in-scope artifact
trees. Clean for md, xml (only a stale doc-comment mentioning prior removed usage), obj, stl, dxf,
step, and ifc's `v4` standard. The one real hit set is inside `🏗️ifc/🏅️standards/🔖️2x3/…` —
genuine `serde_json` usage in `OpText`/`OpBinary`, but `2x3` is the explicitly-out-of-scope sibling
standard this wave's own report documents as untouched (confirmed via the same grep: zero hits
anywhere under `🔖️4/`). Not a violation of this wave's scope.

## 9. Full crate suite (final, fresh run)

```
cargo test -p semio-s-plugin-stdio --lib
```
→ **1714 passed, 0 failed, 1 ignored** (fresh run, this session, after all FG1 concurrent churn
settled). Exceeds the recipe's own "≥1671/0/1-ignored" baseline. No classification of unrelated
failures was needed — there are none right now.

## 10. Summary table (per the requested per-standard shape)

| artifact | tests_passed | tests_failed | real_dialect_confirmed | binary_frame_confirmed | fixtures_real | registration_confirmed | pitfalls_avoided | notes |
|---|---|---|---|---|---|---|---|---|
| md | 36 | 0 | yes | yes | yes | yes (5 roles) | yes | Clean; both OpBinary and DiffCodec genuinely upgraded to real recursive binary frames. |
| xml | 36 | 0 | yes | yes | yes | yes (5 roles) | yes | Clean; also fixed a real pre-existing `POLICY_STDIO_JSON_TRANSFER_BAN` violation in `ArtifactPack` (verified: `xml_document_to_text(...).into_bytes()` now used, not `serde_json`). |
| obj | 30 | 0 | yes | **partial** | yes | yes (5 roles + real `register_schema_specs`) | yes | `OpBinary` legitimately already-real (`dsl::variants_binary`, derive-based); `DiffCodec::encode_diff` still `print_diff().into_bytes()` — the F6 shortcut, not upgraded despite the recipe's explicit mandate. |
| stl | 34 | 0 | yes | **no** | yes | yes (5 roles) | yes | Both `OpBinary` and `DiffCodec` still on the literal text-as-binary shortcut; report's "already real" framing conflates "not JSON" with "not the shortcut" — see §3. |
| dxf | 25 | 0 | yes | yes | yes | yes (5 roles) | yes | Clean; both facets genuinely upgraded with new recursive binary primitive regions. |
| step | 106 | 0 | yes | **no** | yes | yes (5 roles) | yes | Both facets still on the shortcut (zero header bytes at all — not even `format`/`tag`). |
| ifc (v4) | 74 | 0 | yes | **no** | yes | yes (5 roles) | yes | Same as step; `v2x3` correctly confirmed untouched (46 pre-existing tests still pass, contains the sole pre-existing, out-of-scope `serde_json` usage). |

**Full crate**: 1714 passed / 0 failed / 1 ignored (fresh, this session).

## Files/paths referenced during verification

- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/📖️grammar-recipe.md`
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg1-{md,xml,obj,stl,dxf,step-ifc}-report.md`
- All 42 `.grammar.semio`/`.protocol.semio` files under
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📝️md,📰xml,🧊️obj,🟪️stl,🖊️dxf,📐️step,🏗️ifc}/**`
- `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs` under each of the 7 standards (encode_diff/encode_op bodies)
- `⚙️engine/🦀️component.rs` under each of the 7 standards (`register_pilot_languages`)
- `📚️examples/🎬️demo/🖼️assets/{🗣️example.dsl.semio,🎒️example.pack.semio}` under each of the 7 artifacts
- Cross-reference precedent check: `☁️ply/🏅️standards/🔖️1.0/…/🔺️diff/🦀️component.rs`,
  `💬️bcf/🏅️standards/🔖️2.1/…/🔺️diff/🦀️component.rs`,
  `💾️binary/🏅️standards/🔖️raw/…/🧬️mutations/🦀️component.rs`

# 📓️ W2-W-text — Wire-witness conversion: svg, xml, json, html, md, txt, csv, tsv (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-W-text, 2026-09-30.

- **Contract:** design §6 and §11, the plan's "W2-W brief", and the recipe in `📓️w2-s-report.md` Follow-up 3, F10.
- **Scope:** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎨️svg,📰️xml,🧾️json,🌐️html,📝️md,🔤️txt,📊️csv,📑️tsv}`, in 11 feature cases.

## 1. Outcome

Both lints are now 0 across the whole scope, and every leaf is witnessed:

| artifact | `schema mutation-payloads`, before | after | leaves witnessed | `schema mutation-inputs`, after |
|---|---|---|---|---|
| svg | 33 | **0** | 29/29 | 0 (58/58 inputs) |
| json | 31 | **0** | 15/15 | 0 (32/32) |
| xml | 17 | **0** | 15/15 | 0 (25/25) |
| csv | 11 | **0** | 6/6 | 0 (10/10) |
| html | 4 | **0** | 9/9 | 0 (18/18) |
| tsv | 4 | **0** | 6/6 | 0 (9/9) |
| md | 2 | **0** | 5/5 | 0 (12/12) |
| txt | 0 | **0** | 5/5 | 0 (7/7) |

The raw lint output is in `🗑️generated/w2w-text/before-*.txt` and `after-*.txt`.

**Feature cases.** I ran `bun ./📜️script.ts parity exhaustive --case <case>`, which covers both the oracle and the subject phase:

| case | executed / passed | parity |
|---|---|---|
| `🎨️mutate-svg-1-1` | 38/38 | 19/19 |
| `🔬️mutate-svg-1-1-tiny` | 38/38 | 19/19 |
| `🔰️mutate-svg-1-1-basic` | 42/42 | 21/21 |
| `✅️mutate-xml-1-0-valid` | 34/34 | 17/17 |
| `📰️mutate-xml-1-0` | 26/26 | **0/13**, a gap that predates this work (§5.1) |
| `🔀️mutate-json-rfc8259` | 22/22 | 11/11 |
| `🔀️mutate-json-rfc8259-i-json` (Python oracle) | 40/40 | 20/20 |
| `🌐️mutate-html-5` | 38/38 | 19/19 |
| `📝️mutate-md-commonmark` | 22/22 | 10/11: `mutate-set-snapshot` is the scenario the feature already documents as red (§5.2) |
| `📝️mutate-txt-utf-8` | 40/40 | 20/20 |
| `📊️mutate-csv-rfc4180` | 22/22 | 11/11 |
| `📊️mutate-tsv-iana` | 26/26 | 13/13 |

**Other checks:**

- **Contract phase.** It reports breaches repo-wide. Three breach types fall in this scope, and all three predate this work (§5.3). None comes from a removed baseline scenario or from a handler registration.
- **Cargo, run gated and in the foreground.**
  - `cargo check` on the 8 crates, native and `--target wasm32-wasip2`: ok. The one warning in scope was already there (§5.4).
  - `cargo test --lib` with `CARGO_INCREMENTAL=0` and the private target dir `target-nde-w2w-text`: svg 94, xml 69, json 90, html 43, md 47, txt 55, csv 41, tsv 28. All pass, including every `semio_payload_law_*`: svg, svg_basic, svg_tiny, xml, xml_valid, json, json_i_json, html, md, txt, csv and tsv.
  - Stdio oracle crate (`🔮️oracles/📦️packages/🦀️rust`, `--features oracles --lib`): 396 passed.
- **Aggregate item (coordinator).** I regenerated the map for all 8 artifacts with `🧪️w2-s-aggregate-map.ts` and dry-ran `🧪️w2-s-aggregate-rule.py` over the same prefixes: "would write 0 file(s), 55 already conform, 0 manual". The svg, xml and json `🧱️base` aggregates already reference `leaf#/$defs/<Wrapper>` for every `payload = Apply` leaf, from W2-S-B2's `🧪️w2-s-stdio-wrapped.py`. So there was nothing to apply, and the lint reports no `aggregate` finding.

## 2. The generic decoder: one bridge per aggregate

The recipe's `SvgMutation::from_payload_value(kind, DslValue::from(params))` does not compile inside a case adapter:

- The generated host links only the case's own artifact crate. That is the `sut` crate, plus the test host and the oracle crate.
- So the adapter cannot name `protocol::Mutation` (os-kernel), and the test host's `Json` has no `DslValue` conversion.

Following the precedent of `decode_semio_audio_mutation_json` from W2-S-B2, every aggregate gained a two-line public bridge next to its `apply_*_mutation`. The bridge's only job is to call the derive-generated constructor:

```rust
pub fn decode_svg_mutation_payload_json(kind: &str, payload: &str) -> Result<SvgMutation, String> {
    let value = pack::parse_json(payload).map_err(|error| error.to_string())?;
    <SvgMutation as protocol::Mutation<SvgSnapshot>>::from_payload_value(kind, pack::json_to_dsl_value(&value)).map_err(|error| error.to_string())
}
```

- **Aggregates covered:** `decode_{svg,svg_tiny,svg_basic,xml,xml_valid,json,json_i_json,html,md,txt,csv,tsv}_mutation_payload_json`.
- **csv and tsv** have no `pack` dependency, so they use the stdio contract crate's re-export `semio_s_artifact_stdio_contract::pack`.
- **Every adapter's `mutation_from_spec`** is now this one line:
  `decode_*_payload_json(&spec.str("kind"), &spec.get("params")…to_string())`.
- **Hand-mapping helpers deleted:** `str_field`, `path_field`, `usize_field`, `number_field`, `view_box_field`, `transform_op_field`, `json_to_xml_node`, `json_to_doctype`, `json_to_declaration`, `json_to_html_node`, `tristate_value`, `value_from_json`, `path_from_json`, `number_lexeme`, `build_block`/`build_inline`/`build_path`/`build_snapshot`, `parse_line_ending` and the closures that went with them.

**Subject inverses now call the production algebra.** Several adapters had transplanted a closed-form copy of `Mutation::inverse` or rebuilt the inverse by hand: svg base, xml base, html, md, csv and tsv. Some of those copies no longer compiled at all:

| adapter | why it did not compile |
|---|---|
| svg base | imported `semio_s_artifact_stdio_xml`, which the host does not link |
| svg tiny/basic, tsv | struct literals over `pub(crate)` leaf fields |
| tsv | `semio_framework_os_kernel::ArtifactDsl`, which the host does not link |
| txt | `protocol::Mutation`, which the host does not link |

Each aggregate now exposes an `inverse_*_mutation` bridge: svg, xml, html, md, txt, csv and tsv. Tiny, basic, valid and i-json already had one. Every subject `inverse` handler applies the real inverse sequence. The oracles keep their own independent undo.

**tsv** also gained `parse_tsv_document`/`print_tsv_document`, which are the artifact's own `ArtifactDsl` codec, for the identity scenario.

## 3. Feature rows, fixtures and schemas

**Rows.** Every `{"kind","params"}` row now carries the exact leaf `payload_value()`:

- **svg:**
  - `set-declaration` becomes `{"declaration": XmlDeclaration}`.
  - `set-doctype` becomes `{"doctype": {"name", "externalId": {"kind":"public",…}}}`.
  - `set-view-box` uses `{minX,minY,width,height}`.
  - `set-transform` uses the `op`-tagged `TransformOp`.
  - tiny/basic `set-snapshot` carries a whole `SvgSnapshot`: a small Tiny or Basic replacement document.
- **xml:**
  - `set-declaration` and `set-doctype` are wrapped the same way. Doctype `declarations` carry `kind: entity`, and the node kind is `cData`.
  - valid `set-snapshot` carries a whole `XmlSnapshot` (the former plist string, decomposed), and `set-internal-subset` declarations carry `kind: entity`.
- **json:** paths are `JsonPathSegment` values (`{"kind":"key"|"index","value"}`) and values are the tagged `JsonValue` (`{"kind":"number","lexeme":"99"}` …).
  - i-json `set-snapshot` carries `{"snapshot": JsonSnapshot}` and `set-top-level` carries `{"root": JsonIJsonRoot}`.
  - These rows were written by the kept script `🧪️w2-w-text-json-rows.py`.
- **html:** `set-snapshot` carries `{"snapshot": {"schema","doctype","root"}}`.
- **csv:**
  - `set-snapshot` carries `{"snapshot": {"schema","hasHeader","records":[{"fields":[{"value","quoted"}]}]}}`.
  - `insert-record` carries `{"index","record": CsvRecord}`.
  - `set-field` gains the required `quoted`.
- **tsv:** `set-snapshot` carries `{"snapshot": TsvSnapshot}`.
- **md and txt:** the rows were already in wire form.

**`no-mutation`.** Following recipe step 2, I removed every `no-mutation` baseline scenario and its handler registration:

- **Where:** svg tiny/basic, xml valid, json i-json, html, md, csv and tsv.
- **Why:** every case keeps its own `identity-round-trip` scenario, which is the baseline check.
- **Oracle round trips:** the oracles' `"no-mutation"` dispatch arms were replaced by explicit round-trip entry points (`oracle_round_trip` in svg base, xml base, html and md).
- **Inverse fallbacks:** the oracles' `"no-mutation"` fallbacks became errors, because a stale address is an error. Observability checks no longer exempt a kind.

**Wire witnesses.** Each leaf that was still unwitnessed now has a payload-only fixture, `🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json`:

- svg base `📸️set-snapshot`
- xml base `📸️set-snapshot`
- json base `🩹️patch-snapshot`
- csv `🩹️patch-snapshot`

The payload law skips an undecodable fixture silently, so a green law run proves nothing about these files. I checked decoding separately: a throwaway decoder decoded all four through `FromValue`. It lived in `🗑️generated`, printed `[DEBUG] … ok` for each, and I then deleted it.

**Rust outliers fixed (repo-wide camelCase rule), with the schema properties renamed in the same step and every `x-semio-ui` kept:**

- svg tiny and basic `StampBaseProfile` and `SetViewBox`; basic `SetClipPathReference` and `InsertClipPathShape`. Their fields `base_profile`, `view_box` and `clip_path_id` are now camelCase on the wire.
- xml valid `DeclareDoctype` and `SetExternalSubset` (`external_id`).
- csv `SetHasHeader` and `SetField` (`has_header`, `record_index`, `field_index`). The csv TS twin was already camelCase.
- **html `SetAttribute.value: Option<Option<String>>`** had a wire that could not round-trip: `None` and `Some(None)` both became `null`. It now follows the bcf precedent:
  - `skip_serializing_if` plus a local `deserialize_double_option`, so absent means remove, `null` means valueless and a string means set;
  - the schema is `type: [string, null]`, and the en/de description is corrected to match.

**Schema drift fixed.** The tiny, basic and xml valid `set-snapshot` leaves had inlined copies of the snapshot schema. Those copies had `prologPosition` misplaced on `XmlAttr` and missing on `XmlDoctype`. The three leaves now `$ref` the shared svg or xml `snapshot.json`, as svg and xml base already did.

**Oracles (independent implementations, reading the leaf schemas' shapes rather than the subject's decoder):**

- **svg base oracle:**
  - wire readers `qnode_from_wire`, `decl_from_wire`, `doctype_from_wire`, `view_box_from_wire` and `transform_op_from_wire`;
  - the inverse is now a state restore from the pre-mutation tree, `invert`, instead of an inverse spec;
  - the base `set-snapshot` shorthand arm was dead code and is gone.
- **`📰markup` family** (shared by svg tiny, svg basic and xml valid): `node_from_wire`, `decl_from_wire`, `doctype_from_wire`, `doc_from_wire`, `view_box_from_wire`, `transform_op_from_wire` and `transform_from_wire`. `json_to_node`, `node_to_json`, `json_to_transform_op` and `path_to_json` were removed.
- **xml base oracle:** wire readers plus an `invert` state restore.
- **json base oracle:**
  - `path_from_spec` reads `JsonPathSegment`;
  - `library_from_wire` parses each number from its verbatim lexeme. That made the `library_number` f64 workaround unnecessary, so it was removed.
- **i-json Python oracle:** `plain_value`, `wire_value`, `wire_root` and `wire_path`, and its inverse emits wire payloads.
- **html oracle:** `set-snapshot` reads the wire, and `invert` restores state. This also fixes the old inverse, which wrote `null` (meaning "valueless") for an attribute that was absent before.
- **md, csv and tsv oracles:** `set-snapshot` and `insert-record` read the wire. csv uses `record_from_wire`.
- **Oracle unit tests** were updated to wire payloads (xml valid, json, html, csv and tsv). The `no-mutation` tests became round-trip tests. The svg tiny/basic tests pass unchanged.

## 4. Verification commands

All were run in the foreground behind the rustc gate. Logs are in `🗑️generated/w2w-text/`.

| check | result |
|---|---|
| `schema mutation-payloads --under <artifact>` ×8 | 0 findings each, all leaves witnessed (`after-*.txt`) |
| `schema mutation-inputs --under <artifact>` ×8 | 0 findings each |
| `parity exhaustive --case <case>` ×12 | §1 table (`parity-*.txt`) |
| `cargo check -p semio-s-artifact-stdio-{svg,xml,json,html,md,txt,csv,tsv}`, native and `--target wasm32-wasip2` | ok (`cargo-check-wasm.txt`) |
| `cargo test --lib` on the same 8 crates | all pass (`cargo-test-svg-xml.txt`, `cargo-test-rest.txt`) |
| `cargo test --manifest-path 🔮️oracles/📦️packages/🦀️rust/Cargo.toml --features oracles --lib` | 396 passed (`cargo-test-oracles.txt`) |
| aggregate map + rule dry run | 0 to write, 55 conform (`aggregate-map.json`) |

## 5. Open items

1. **xml base pipeline parity, 0/13 (predates this work).** The comparison profile gates on pipeline `xml-1-0-quick-xml-compare-v1`. Its stages need `expected-xml`/`actual-xml` artifacts, and the case's adapters never emit them.
   - The diff reports show `"artifacts": {}` and "stage 0 (xml-import) produced no report", for every scenario including `identity-round-trip`.
   - The probe itself works when called by hand (`ok: true`).
   - The fix is either to wire the artifact bundle or to scope the pipeline to the fixture recipes. That belongs to the xml case owner and is independent of the wire params.
2. **md `mutate-set-snapshot` parity, 10 differences (documented red).** comrak inserts `<!-- end list -->` between a list and a code block. The feature file keeps this scenario red on purpose ("left RED rather than tuned away").
3. **Contract breaches in scope (all predate this work):**
   - csv and json oracle catalogs claim `patch-snapshot`, but no manifest owns it;
   - inline test modules in csv/json `🩹️patch-snapshot`;
   - the xml base fixture hash for `📣️set-declaration-applied/➡️after.xml`.
4. **An earlier warning:** `unnecessary qualification` at `📰️xml/…/✅️valid/🧬️schema/🦀️.rs:27`.
5. **Taxonomy (fleet-wide, W3-G):** `verify taxonomy report` flags `directory-kind-unresolved`.
   - **Where:** the prescribed `🧾️wire-witness` directory, and `🩹️patch-snapshot` fixture directories.
   - **Not specific to this group:** peers' existing witnesses show the same finding, for example cad `❌delete-object/🧾️wire-witness` and flow `📐️change-layout/🧾️wire-witness`.
   - **Needed:** `🔣️taxonomy.json` (a hot shared file) has to register them. I did not touch it.
6. **Follow-up:**
   - Leaf schemas type every `JsonValue` field as `{"description": "any JSON value"}` (json base `set-member`, `insert-array-element` and `set-scalar`; i-json `upsert-member`, `insert-array-element`, `set-snapshot.value` and `JsonIJsonRoot`). The wire is actually the tagged `JsonValue`, so these should `$ref` `…/json/rfc8259/base/snapshot.json#/$defs/JsonValue`. That needs labels for `lexeme`/`items`/`members` so `schema mutation-inputs` stays at 0.
   - `deserialize_double_option` now exists in both bcf and html. It should move into the stdio contract crate, but that crate is shared with peers, so I left it as it is.

## 6. Files

- **Rust bridges (aggregate modules):**
  - svg base, tiny and basic `🧬️mutations/🦀️.rs`
  - xml base and valid `🧬️mutations/🦀️.rs`, plus the valid `🧬️schema/🦀️.rs` re-export
  - json base and i-json, html, md, txt, csv and tsv `🧬️mutations/🦀️.rs`
  - tsv `📸️snapshot/🦀️.rs` (document codec bridges)
- **Rust leaves:**
  - svg tiny `🪧stamp-base-profile`, `🖼️set-view-box`
  - svg basic `🪧stamp-base-profile`, `🖼️set-view-box`, `✂️set-clip-path-reference`, `📎insert-clip-path-shape`
  - xml valid `📜declare-doctype`, `🔗set-external-subset`
  - csv `🧾set-has-header`, `✏️set-field`
  - html `🔖set-attribute`
- **Leaf schemas:** the leaves above, plus svg tiny/basic and xml valid `📸️set-snapshot`.
- **Adapters:** the 11 case `🦀️.rs` files, plus the i-json `🐍️.py`.
- **Features:** the 11 case `🥒️.feature` files.
- **Oracles:**
  - svg base, tiny and basic `🔮️oracles/🦀️.rs`
  - `🔮️oracles/📰markup/🦀️.rs` (stdio oracle crate)
  - xml base and valid, json base, html, md, csv and tsv `🔮️oracles/🦀️.rs`
  - updated unit tests: xml valid `🔬️live-unit`, json `🔬️unit`, html `🔬️oracles-unit`, csv `🔬️unit` and tsv `🔬️unit` (the svg tiny/basic tests pass unchanged)
- **New witness fixtures:** svg base, xml base, json base and csv `🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json`.
- **Ticket script (kept):** `🧪️w2-w-text-json-rows.py`.
- **Scratch:** `🗑️generated/w2w-text/` (lint, parity, contract and cargo logs; `progress.txt`; `aggregate-map.json`).

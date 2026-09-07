# 📖️ W10 — remodeling text grammars + binary protocol specs

Scope: `…/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/*/{📝️text,💾️binary}` spec docs.
No `🦀️.rs` touched (W11 owns Rust).

## 1. Inherited state

Everything the killed W10 wrote had already auto-committed at `2d2b39eb7f` — not only the snapshot
grammar. All four `📖️.grammar.semio` are real, the `🔤️.ebnf`/`🅰️.g4` sidecars are generated from
them, and no `stdio_json_*`/`stdio-json`/`StdioJson` identifier survives under the artifact.
`🐍️grammar-check.py` passed as found. This lane therefore: verified that, closed the one real
remaining gap (the binary specs' "opaque field stream"), extended the checker, corrected two grammar
docstrings, and wrote the Rust test for W11.

## 2. Registered-grammar mechanism

* `dsl::LanguageSpec { id, extension, role, grammar, grammar_path, protocol, protocol_path, hooks }`
  — `🗣️dsl/🦀️.rs:510`. `parsed_grammar()` **rejects any dialect but `grammar`**; `is_text_role()`
  = Document/Config/Ops/Embedded/Diff (must carry a grammar), `is_binary_role()` = Pack/Spr (must
  carry a protocol, never a grammar).
* Remodel registers five specs in `pilot_languages()` (`🗿️artifacts/📸️remodeling/🦀️.rs:106`,
  duplicated verbatim in `🚪️io/🦀️.rs:470`): `remodeling.document` (Document; snapshot grammar +
  snapshot pack protocol), `remodeling.op` (Ops; mutations grammar + spr protocol),
  `remodeling.diff` (Diff; diff grammar, no protocol), `remodeling.pack`, `remodeling.spr`.
* **`LanguageRole` has no `Inference` variant**, so the inference facet's grammar and protocol are
  authored and `include_str!`'d but reach no registry. Framework gap — do not invent a local role.
* Two `💾️binary/🦀️.rs` module docs still read `//! binary rep for stdio.json …` (snapshot, diff).
  Rust, so left for W11 (§6.2).

## 3. Grammars

| facet | grammar id | start | productions |
|---|---|---|---|
| `📸️snapshot/📝️text` | `remodeling.snapshot` | `document` | 95 |
| `🧬️mutations/📝️text` | `remodeling.op` | `op` | 107 |
| `🔺️diff/📝️text` | `remodeling.diff` | `diff` | 26 |
| `💡️inferences/📝️text` | `remodeling.inference` | `inference` | 15 |

`🐍️grammar-sidecars.py` derives `🔤️.ebnf` (kebab → "space case") and `🅰️.g4` (kebab → camelCase)
from the normative `📖️.grammar.semio`, so all three share rule names. Re-running it produced
**byte-identical output** for all eight sidecars — already in sync.

The diff/inference grammars model the value bridge — `store::to_dsl_value`, then the DslValue
notation `{ key=value … }` with bare keys sorted by name and every unset diff lane an explicit
`null`. Both types carry only `ToValue`/`FromValue` (no `DslRecord`/`DslDiff` derive), so they own
no `RecordSpec` and no dedicated printer.

**Correction landed this lane in both `📖️.grammar.semio` docstrings:** that notation is
`🗣️dsl/🧬️schema/🦀️.rs:2201`'s `print_dsl_value`, which is **private** and reachable only as the
payload of a printed record field of `Shape::Value`; `pack_rt::value_bridge_spec` is private too,
and the framework exposes **no public `DslValue -> String`**. These two facets have a real *binary*
path (`store::pack_rt::encode_wire_value`, public, used by `🏗️fem`) but **no reachable text
printer** — §7 reconstructs one.

## 4. Binary specs — the record layout (this lane's edits)

Read off the framework: `os_pack::encode_record_fields` (`🎒️pack/🌱️value/🦀️.rs:342`) =
`field_count varint, field_count × (field_id varint, tagged value)`, **sorted by ascending field
id**, `Absent` never written (that sort is the byte-determinism LAW). Value tags are a closed
alphabet `0x00..0x17` (same file :21-44); `0x0D` nests the same body, so a document is one recursive
production — self-describing *and* fully declarative, which is why "stays opaque here" was an
understatement. `#[derive(dsl::DslRecord)]` assigns `id = 0-based declaration index`
(`✨️derive/🦀️.rs:1111`). `encode_record_body` (:2163) inlines the symbol table; `encode_document`
(:2095) puts it in a `KIND_SYMBOLS` segment (`format::encode_symbols`) and frames the field body
across `KIND_DOCUMENT`. Value bridge = `encode_record_body` of `value_bridge_spec()`
(`🏪️store/🦀️.rs:4970`): field id `1`, `Shape::Value`, tag `0x11`, one `encode_dsl_value` node.

`RemodelingSnapshot::__dsl_spec()` top-level fields, now written into all three snapshot specs:
`0 schema` Text (`0x06` symref / `0x07` inline) · `1 id` Text · `2 streams` Table (`0x14`) ·
`3 assets` Map (`0x10`) · `4 durable-artifacts` Map · `5 calibration` Block (`0x0E`→`0x0D`) ·
`6 params` Block · `7 gcps` Table · `8 job` Block · `9 results` Block.

Files rewritten (8):

* `📸️snapshot/💾️binary/{🥋️.ksy,🔠️.abnf,🌶️.spicy}` — added `symbols_segment`/`symbol`,
  `record_fields`/`field_entry`, the full `value_tag` enum and the field-id table. The segment
  `payload` stays a byte run in `seq` because it is deflate-compressed when `flags & 1`.
* `🧬️mutations/💾️binary/{…}` — `fields` is now a typed `record_fields`, plus the **ordinal →
  keyword table for all 35 `RemodelingMutation` variants** (declaration order;
  `variants_binary::encode_op` writes `position()` in `variants()`).
* `🔺️diff/💾️binary/{…}` + `💡️inferences/💾️binary/{…}` — body spelled out to the byte:
  `field_count=1`, `field_id=1`, `0x11`, one `dsl_value` with its 10-tag alphabet (object keys are
  always `0x07` inline, never symrefs; arrays/objects varint-counted; objects sorted by key bytes).

The four `📡️.protocol.semio` were left as-is: correct `dialect protocol`, id, framing; the snapshot
one reproduces the repo's canonical `.spk` `repeat segments { … arm … }` form verbatim from
`stdio.binary.diff` (the `if flags eq 3` guard binds `raw_len`, matching `encode_segment`'s
`flags = 1 | codec<<1`). All four KSY parse as YAML.

## 5. Static validation

`🐍️grammar-check.py` (python twin of the framework recognizer: same greedy commit-on-first-
alternative matching, terminal predicates, full-token-consumption rule and `split_text_preamble`
reduction) gained `binary_layout_problems()`, which fails any binary triple not naming
`field_count`/`field_id`, the `0x0D` record tag, the ten snapshot field ids, the first/last op
ordinals, or — for the bridge facets — `0x11`/`value_bridge_spec` and the `0x12`/`0x0C`/`0x10`/`0x07`
DslValue tags. Output:

```
  📸️snapshot <- 📚️examples/🎬️demo: RECOGNIZED (346 tokens)
  📸️snapshot <- 📚️examples/🛰️synthetic-orbit: RECOGNIZED (484 tokens)
  📸️snapshot <- 🗒️populated-snapshot: RECOGNIZED (747 tokens)
  🧬️mutations: 42 op line(s) checked, 0 failure(s)
  🔺️diff <- 🗒️diff.dsl.semio: RECOGNIZED (103 tokens)
  💡️inferences <- 🗒️inference.dsl.semio: RECOGNIZED (62 tokens)
  📸️snapshot vs 5 non-stdio peer fixture(s): 0 wrongly accepted
  📸️snapshot/💾️binary: ok   🧬️mutations/💾️binary: ok   🔺️diff/💾️binary: ok   💡️inferences/💾️binary: ok
✅ every remodeling grammar leaf recognizes its real documents and rejects the peers'
```

No `uncovered productions` line printed for any facet — every production of all four grammars is
exercised. The 42 op lines are the 35 keywords plus 7 second cases for the `Option`-carrying ops.

**Cargo: run pending.** No `🗑️generated/w11-check-*.txt` and no `target-remodel-w11` existed during
this session; the last real check (`central-check-1.txt`, 05:25) had 128 internal errors, and
`w9-check-1.txt` is 141 bytes of `command not found: timeout` — W9's check never ran. So
`cargo test -p semio-s-plugin-remodel --lib grammar` cannot compile yet; run it once W11 closes.

## 6. For W11

1. Place the §7 tests as a new `#[cfg(test)] mod grammar_tests` at the end of
   `🗿️artifacts/📸️remodeling/🦀️.rs`, beside `pilot_languages()`. They go through the registered
   `LanguageSpec`, not the `COMPONENT_GRAMMAR_SEMIO` consts, so they also prove the registration.
2. Two stale one-line module docs naming a foreign artifact:
   `🧬️schema/📸️snapshot/💾️binary/🦀️.rs:1` and `🧬️schema/🔺️diff/💾️binary/🦀️.rs:1` both read
   `//! binary rep for stdio.json …`; both should also gain the `//#region 📡️SemioProtocol` markers
   their two siblings carry.
3. **No printer change is required.** Two printer facts the grammars encode — do not "fix" either
   without regenerating the grammars: (a) `store::ArtifactChild`'s DslRecord prints `child_id` in
   **snake_case** among kebab-case neighbours (framework inconsistency, also flagged by W8), hence
   `child-handle = "child_id" "=" name "target" "=" TEXT`; (b) the value bridge emits every unset
   diff lane as an explicit `null`, hence 17 pinned lanes rather than a sparse set.
4. Framework follow-ups (record only): a real `dsl::DslDiff` derive for
   `RemodelingDiff`/`RemodelingInference` would replace both bridge grammars and both bridge binary
   specs with structured ones; `LanguageRole` needs an `Inference` variant for that facet to
   register at all; and a `pub fn print_value(&DslValue) -> String` beside `dsl::print` would remove
   §7's workaround and finally give these two text facets a callable printer.

## 7. Grammar tests (place verbatim)

`demo_mutation_cases(&scene)` is the only helper this assumes and it does **not** exist in remodel
today — substitute the mutation list already built in `🧬️mutations/🦀️.rs`'s own `#[cfg(test)]`
block if W11 does not add it. The inference facet has no registered spec (§2), so it gets no test
here; `🐍️grammar-check.py` covers it statically.

```rust
//#region 🧪️GrammarTests
#[cfg(test)]
mod grammar_tests {
    use super::pilot_languages;

    /// 🔎️ The one registered `LanguageSpec` with this id — going through the registry on purpose,
    /// so a spec dropped from `pilot_languages()` fails here instead of silently.
    fn spec(id: &str) -> &'static dsl::LanguageSpec {
        pilot_languages().iter().find(|spec| spec.id == id).expect("language registered")
    }

    /// 🧰️ Compiles the registered grammar of `id`, proving `dialect grammar` on the way through.
    fn recognizer(id: &str) -> dsl::Recognizer {
        let grammar = spec(id).parsed_grammar().expect("grammar parses").expect("grammar present");
        dsl::Recognizer::compile(&grammar)
    }

    /// 🪪️ The framework fixture sweep's own body reduction: the `semio … vN` preamble collapses to
    /// the bare envelope id, which is what a recognizer is handed.
    fn dsl_body(text: &str) -> String {
        let (envelope, body) = store::semio_format::split_text_preamble(text).expect("preamble splits");
        format!("{}\n{body}", envelope.envelope_id())
    }

    /// 🌉 The only reachable route to the DslValue notation: `print_dsl_value` and
    /// `pack_rt::value_bridge_spec` are both private, so print a one-field `Shape::Value` record and
    /// drop the leading `value=` key token the record printer emits before the payload.
    fn print_value_notation(value: &dsl::DslValue) -> String {
        let spec = dsl::RecordSpec::new(None, dsl::RecordLayout::Lines, vec![dsl::FieldSpec::new(1, "value", dsl::Shape::Value)]);
        let mut fields = std::collections::HashMap::new();
        fields.insert(1u16, dsl::FieldValue::Value(value.clone()));
        let printed = dsl::print(&dsl::RecordValue { fields }, &spec, dsl::JoinMode::Document);
        printed.trim_start().strip_prefix("value=").expect("the bridge field prints its key first").trim_start().to_string()
    }

    #[semio_framework_async_macros::async_test]
    async fn every_text_role_carries_a_grammar_and_every_binary_role_a_protocol() {
        for spec in pilot_languages() {
            if spec.is_text_role() {
                assert!(spec.parsed_grammar().expect("grammar parses").is_some(), "{} is a text role without a grammar", spec.id);
            }
            if spec.is_binary_role() {
                assert!(spec.parsed_protocol().expect("protocol parses").is_some(), "{} is a binary role without a protocol", spec.id);
                assert!(spec.grammar.is_none(), "{} is a binary role carrying a grammar", spec.id);
            }
        }
    }

    /// ✅️ The registered document grammar recognizes every committed example verbatim.
    #[semio_framework_async_macros::async_test]
    async fn document_grammar_recognizes_every_committed_example() {
        let recognizer = recognizer("remodeling.document");
        for example in crate::artifacts::remodeling::standards::v1::subsets::any::editor::examples::REMODELING_EXAMPLES {
            let body = dsl_body(example.text);
            assert!(recognizer.recognize(&body).unwrap_or(false), "document grammar rejected example `{}`:\n{body}", example.id);
        }
    }

    /// ✅️ …and its own printer output, so grammar and printer can never drift apart silently.
    #[semio_framework_async_macros::async_test]
    async fn document_grammar_recognizes_its_own_printer_output() {
        let recognizer = recognizer("remodeling.document");
        let text = store::ArtifactDsl::print_dsl(&crate::artifacts::remodeling::default_remodeling_scene());
        let body = dsl_body(&text);
        assert!(recognizer.recognize(&body).unwrap_or(false), "document grammar rejected its own printer output:\n{body}");
    }

    /// ✅️ The registered ops grammar recognizes `print_op` for every mutation case.
    #[semio_framework_async_macros::async_test]
    async fn op_grammar_recognizes_every_printed_mutation() {
        use protocol::OpText;
        let recognizer = recognizer("remodeling.op");
        let scene = crate::artifacts::remodeling::default_remodeling_scene();
        for mutation in crate::artifacts::remodeling::mutations::demo_mutation_cases(&scene) {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "op grammar rejected {printed:?}");
        }
    }

    /// ✅️ The registered diff grammar recognizes the value-bridge rendering of a real diff.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_recognizes_a_printed_diff() {
        let recognizer = recognizer("remodeling.diff");
        let scene = crate::artifacts::remodeling::default_remodeling_scene();
        let mut feature_params = scene.params.feature;
        feature_params.target_count = 12345;
        let mutation = crate::artifacts::remodeling::mutations::update_feature_params(feature_params);
        let (diff, _) = protocol::Mutation::diff(&mutation, &scene).into_parts();
        let printed = print_value_notation(&store::to_dsl_value(&diff).expect("diff bridges to a value"));
        assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar rejected:\n{printed}");
    }
}
//#endregion 🧪️GrammarTests
```

# W8 — Remodeling schema-description regeneration

Scope: the schema *description* leaves under `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/**` —
`🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🧬️.schema.json`, `📖️.grammar.semio`. No `🦀️.rs`, no `🟦️.ts`, no
fixture, no `🔮️oracle/🔣️.json`, no `✏️editor`/`👁️viewer`/`🚪️io` file was touched.

## 1. Authority direction: the JSON Schema leaf is hand-authored normative truth. There is no generator.

Evidence, in the order it settles the question:

1. **Taxonomy.** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` declares
   `schemaFacetKinds.🧬️data = { normativeFormat: "🔣️jsonschema", formats: [🔣️jsonschema, 🦀️rust,
   🟦️typescript, 🔗️graphql, 🛰️protobuf] }` and `schemaFormats[*].fieldCasing`
   (rust/protobuf `snake`, jsonschema/typescript/graphql `camel`, wit `kebab`).
2. **Policy.** Root `📜️script.ts` `policyArtifactSchemaFieldParityBreaches` (≈L30328) states it outright:
   *"all five leaves of one facet declare the identical canonical field set … JSON Schema is the truth when
   others disagree"*, and the normative-leaf rule reads *"Within a facet the … JSON Schema leaf is
   normative; the other four mirror it."* `…StateParity…`, `…DiffCoverage…` and `…TypeNameParity…` all key
   off `policyLoadSchemaFacetLeaves(...).find(l => l.formatId === "🔣️jsonschema")`. These are **verifiers**,
   not emitters — the repo checks the five leaves against each other, it never writes one from another.
3. **Runtime.** `🧬️schema/🦀️.rs:231-246` `include_str!`s all five leaves per facet into
   `schema::FacetLeaves`; `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs` (`parse_normative_json_leaf`,
   `with_json_schema_catalog`, `OwnedJsonSchemaValidator`) compiles the **JSON leaf** into the live
   document validator, and `schema_version()` is a content hash of that leaf. A hollow `$defs` is therefore
   not cosmetic: it is a validator that accepts anything.
4. **No generator exists.** The only schema-shaped codegen target is `@semio-tech/framework-schema:generate`
   (root `📜️script.ts:368`), whose package description is *"single-source entity catalog codegen
   (kind → emoji/icon/label/filterable)"* — the entity catalog, not artifact schemas. No `writeFileSync`
   in `📜️script.ts` targets a schema leaf; no `schemars`/`JsonSchema`/`$defs` emission exists in
   `🧰️framework/🔨️modules/🧬️schema/✨️derive` or the value/dsl derives. `describe` regenerates the plugin
   **owner-root** `🔣️.json`/`🛂️.descriptor.semio`, a different artifact.
5. **Peer census.** `🧩️puzzle/🧊️3d` (`git log`: one commit `21fbcd3538`, 2026-09-02) has 9/11 `$defs` fully
   typed and full nested `type`/`message` bodies in its `🔗️.graphql`/`🛰️.proto` (2 hollow `$defs` remain:
   `Puzzle3dTargetVolume`, `Puzzle3dReference`). Remodel's leaves (`96aa4f8c12`, same day) had **0/7**
   domain `$defs` typed and `_placeholder` bodies in every graphql/proto record. Same authoring style,
   remodel simply never got past the stub.

**Conclusion: hand-author.** Because the model only exists in full in Rust today, I transcribed it with a
ticket-local authoring aid — `🐍️schema-author.py` (in this ticket folder, deliberately *not* wired into any
target and not repo infrastructure). It parses the Rust records and writes the JSON/GraphQL/protobuf leaves;
it is a typing aid, not a build step. Re-run it after a model change, or edit the leaves by hand.

## 2. What was regenerated (51 files)

| leaf | before → after (lines) | what changed |
|---|---|---|
| `🧬️schema/🔣️.json` | 260 → 1597 | 7 hollow `$defs` fully typed; **`durableArtifacts` added** (was missing from `properties` *and* `required`); `assets` retyped `ImageAsset` → `RemodelingAssetChild` (`childId`+`target{artifactId,dialect{…}}`); 46 `$defs` |
| `📸️snapshot/🔣️.json` | 225 → 1457 | same, plus the 6 UI `$defs` that do not belong on the snapshot facet dropped; 42 `$defs` |
| `🔺️diff/🔣️.json` | 160 → 1796 | all 14 hollow `$defs` typed, `durableArtifacts` lane added, every lane nullable-typed; 49 `$defs` |
| `🧬️mutations/🔣️.json` | 706 → 2086 | **35th kind `CommitReconstruction` added** (34 before); every payload's fields fully typed instead of `{"type":"object","description":"MediaStream"}`; **`mutation` tag const corrected** from the kebab DSL keyword (`"create-stream"`) to the actual serde wire tag (`"createStream"` — `#[serde(tag="mutation", rename_all="camelCase")]`); 74 `$defs` |
| `💡️inferences/🔣️.json` | 34 → 103 | **`relativeCameraPoses` added** (`BTreeMap<String, RemodelingPoseDelta>` existed in Rust and in no description leaf) |
| `🔗️.graphql` ×5 | 53→393, 22→362, 37→422, 215→538, 16→27 | every `_placeholder`/one-field stub replaced by the real `type`s and `enum`s (PascalCase variants, peer style); map lanes as `[…Entry!]!` |
| `🛰️.proto` ×5 | 92→403, 82→365, 56→433, 187→544, 20→27 | every `string _placeholder = 1;` replaced by the real messages, `map<string,…>`, `optional`, and `SCREAMING_SNAKE` enums with the type prefix (peer style) |
| `🧬️mutations/*/🧬️.schema.json` ×35 | — | regenerated from each payload struct. `🏁commit-reconstruction/🧬️.schema.json` **described the wrong type** (`ReconstructionAssetCommit`, a sub-record) and now describes `CommitReconstruction`. All 35 stay Draft-07 and self-contained (no `$defs`), as `mutationPayloadSchemaAuthority.jsonSchemaDialect` requires |
| `🧬️mutations/📖️.grammar.semio` | 15 → 84 | **was a foreign template**: it described `add-vertex` / `set-face` / `transform-mesh` / `merge-solid`, a mesh-editing vocabulary that does not exist in remodeling. Rewritten as the real 35-keyword op grammar in `RemodelingMutation` variant order, matching `protocol::OpText::print_op`. (This leaf is `include_str!`'d nowhere — the registered `LanguageSpec.grammar` is `🧬️mutations/📝️text/📖️.grammar.semio`.) |

Two runtime constraints the new leaves respect (both verified mechanically, §3):

- `OwnedJsonSchemaValidator::validate_schema_node`
  (`🧰️framework/🔨️modules/🧬️schema/✅️validator.rs`) **hard-errors on any keyword outside its allowlist**.
  `contentEncoding` is *not* on it — a first draft used it for the base64 `PackedF32`/`PackedU8` lanes and
  would have made every remodel document validation fail at load. Replaced with `format: "base64"`.
- Every `$ref` must resolve locally; the `$defs` set is the transitive closure of each root.

`$id`s, top-level field order and the `x-semio-state` / `x-semio-derived` annotations are preserved
byte-for-byte where they existed, so the field-parity/state-parity/diff-coverage policies see no new drift.

## 3. Validation — `🐍️schema-validate.py` (stdlib only; `jsonschema` is not installed here)

```
normative leaves vs the Rust OwnedJsonSchemaValidator keyword allowlist: no problems
validated 138 committed fixture documents against the regenerated leaves
validated 34 committed payloads against their per-mutation 🧬️.schema.json: no violations
per-mutation payload leaves vs the repo draft-07 payload contract: no problems
70 of 138 documents violate the schema; 73 violations
```

138 = 34 cases × (before, after, mutation, diff) + the 2 `🧫️fixtures/🏁️commit-reconstruction` documents.
All 34 mutation payloads and all 34 diffs validate. The 73 violations are **fixture defects, not schema
defects** — no fixture was edited (W2b owns them):

| # | count | violation | owner |
|---|---|---|---|
| F1 | 70 | `durableArtifacts` required but absent — 68 snapshot fixtures (34 × before/after) + both `🧫️fixtures/🏁️commit-reconstruction` documents. `RemodelingSnapshot::durable_artifacts` has no `skip_serializing_if`, so Rust re-emits it; confirms W3's D1 | W2b/W2c fixture regeneration |
| F2 | 1 | `🧫️fixtures/🏁️commit-reconstruction/⬅️before.json` `job.stage = "dense-reconstructing"` — not a `ReconstructionStage` variant | W2b (W3's D9a) |
| F3 | 1 | `🧫️fixtures/🏁️commit-reconstruction/➡️after.json` `job.stage = "completed"` — not a variant | W2b (W3's D9a) |
| F4 | 1 | **new** — `🧫️fixtures/🏁️commit-reconstruction/⬅️before.json` has `results.mesh = null`, but `ReconstructionResults::mesh` is `RemodelingMesh`, not `Option<…>`. Rust's own `from_str` rejects it independently of the stage lexemes | W2b |

Diff fixtures do not trip F1 because every diff lane is sparse (`required: []`) — correct, not a gap.

**Example documents** (`📚️examples/*/🖼️assets/🗣️.dsl.semio`) are DSL text, so they are checked against the
schema's kebab-cased key vocabulary and enum lexemes rather than parsed as JSON:

```
📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio:           unknown keys ['child_id']; absent ['durable-artifacts']
📚️examples/🛰️synthetic-orbit/🖼️assets/🗣️.dsl.semio: unknown keys ['child_id']; absent ['durable-artifacts']
```

| # | finding | owner |
|---|---|---|
| E1 | **both** examples omit `durable-artifacts` (W3's D7 covered only the demo; W7a's new synthetic-orbit has the same gap) | W7a / W4 |
| E2 | both print the child handle key as `child_id`, snake_case, where every other DSL key in the file is kebab (`camera-id`, `asset-id`). This is `store::ArtifactChild`'s `DslRecord` naming, so it is a **framework-level** inconsistency visible in every artifact that owns a child, not a remodel authoring slip | framework `🏪️store` |

**W3's TypeScript suite stays green after the regeneration**: `bun nx run @semio-tech/remodel-js:test
--skip-nx-cache` → `Test Files 4 passed (4) / Tests 386 passed (386)` (386, not 383 — W3/W7a added three
since that report).

## 4. What remains stale (deliberately not touched)

| leaf | state | why not fixed here |
|---|---|---|
| `{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}/📝️text/📖️.grammar.semio` (+ their `🔤️.ebnf`, `🅰️.g4`) | describe an opaque `payload = OCTET+` body under a `header = "schema" SP "<id>" NL` line. The **real** DSL header the plugin prints is `semio remodeling.remodeling.dsl v1`, so these do not describe the actual text form | These four ARE `include_str!`'d as the registered `LanguageSpec.grammar` (`🧬️schema/*/📝️text/🦀️.rs:8-10`, consumed at `🗿️artifacts/📸️remodeling/🦀️.rs:115-135` and `🚪️io/🦀️.rs:474-494`) and are parsed at runtime by `dsl::parse_grammar`. Rewriting them richly without being able to run `cargo test` this hour risks turning a currently-parsing grammar into a load-time failure. **Recommended follow-up**: rewrite in the real dialect per `📖️grammar-recipe.md` in the same wave that can run the crate's grammar law tests |
| `{📸️snapshot,🔺️diff,🧬️mutations}/💾️binary/{🥋️.ksy,🌶️.spicy,🔠️.abnf}` | carry copy-pasted **`stdio_json_*` / `Stdio_json_*` / `; abnf stdio.json snapshot`** identifiers from a stdio template instead of `remodeling_*`. `💡️inferences` is the only one named correctly | Binary specs; per the lane brief they are not hand-edited without a generator. The fix is a pure rename of the `meta.id` / `module` / comment strings in 9 files |
| `{📸️snapshot,🔺️diff,🧬️mutations}/💾️binary/📡️.protocol.semio` | all three share framing magic `0x8953f83f7d340d0a` while `💡️inferences` uses `…0d0b` | Cannot tell from the file whether the shared magic is intended (one container, three schema ids) — needs the io lane's judgement, not a schema edit |
| `🟦️.ts` leaves | already correct — W3 landed the full TS twin | W3's scope, untouched |

## 5. Files

Written (51): `🧬️schema/{🔣️.json,🔗️.graphql,🛰️.proto}`,
`🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}/{🔣️.json,🔗️.graphql,🛰️.proto}`,
`🧬️schema/🧬️mutations/📖️.grammar.semio`, `🧬️schema/🧬️mutations/*/🧬️.schema.json` (35),
all under `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

Ticket inputs: `🐍️schema-author.py` (authoring aid), `🐍️schema-validate.py` (the validation above; re-run
it after any fixture or schema change — it prints the four gate lines and every remaining violation).

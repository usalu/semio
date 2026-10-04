# Strict JSON And Schema Resource Provider Audit

Readonly source exploration; no source changes, Cargo, web, or runtime tests. Actual captured inputs below; generated captures are temporary. No current production `SchemaService` declaration was found by exact source search; the canonical available general mechanism is `OwnedJsonSchemaValidator` behind `StructuralValidation`, reexported by actual Schema package glue.

## Existing TypeScript Owners

- `🧬️schema/✅️validator/🟦️.ts` is owned neutral production validation, but **not a strict JSON text reader or resource-set resolver**. Its API `validateJsonSchemaSubset(schema, value, root)` receives already constructed `unknown` values. The private resolver accepts document-local `#` only; it filters empty pointer components, so `#/` and empty property segments lose identity. `decodeURIComponent` can throw independently of returned diagnostic errors. It has no external `$id` index, progress, cancellation, cyclic-reference guard, or rejection of unknown schema keywords/types (unknown type returns true). Its ordinary property access can observe inherited getters. Do not use it to certify captured physical schema resource closure.
- `🧬️schema/🟦️.ts` contains schema descriptors and registries; registration overwrites existing IDs using Map.set. These registries describe format leaf strings and named exports, not JSON document resource ownership. They do not resolve `$ref` or protect conflicting resource IDs.
- `🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️.ts` has a genuine production incremental JSON byte tokenizer `OwnedUiSceneJsonCursor` and companion `🧾️value` document token index. It offers `advance(grant)`, failure/offset, `beginClose`, `closeStep`, and terminal retirement, preserves token byte spans, and consumes captured `OwnedUiPreparedScene` fields. This is **UI coupled**, not a neutral arbitrary input reader. Frame state carries no per-object decoded-key set; lexical validity alone does not reject duplicate names. Do not import scene preparation or RetainedUiWireStep into a neutral schema source authority. If this parser is generalized, move its actual grammar/source contract into the lower owned JSON provider and have UI consume it directly, with no parallel copy or accessor-based source shortcut.
- Repo `📚️library/🧬️schema/🔣️json-document/🟦️.ts` exposes a regex duplicate scanner over already syntactically valid JSON. It relies on platform JSON.parse to decode strings and synchronous matchAll; no progress/cancel. It is a Repo adapter and is unsuitable as the production neutral provider.
- OS plugin catalog verification owns `rejectDuplicateJsonObjectNames` at line 363. It recursively scans object keys before platform JSON.parse collapses them, depth ceiling 128; it is tied to catalog verification, synchronous, and not a neutral resource resolver. General whitespace uses regex `\s`, so syntax authority still needs the final platform JSON parser. Do not preserve this as a lower-owner adapter.

**Conclusion:** no existing neutral TypeScript strict JSON text reader plus document resource resolver satisfying the requested source-authority contract was located in these production paths. Platform JSON.parse gives syntax errors and system-only parsing but collapses duplicate members; reviver cannot recover them. An owned lexical duplicate check must precede it, or an owned parser must retain member occurrences.

## Existing Rust Schema Resource Mechanism

Actual `🧬️schema/✅️validator/📦️packages/🦀️rust/Cargo.toml` depends on owned Value and Pack; actual source uses `pack::json::{parse, Number, Object, Value}`. `OwnedJsonSchemaValidator::compile_with_documents_and_control(schema_json, documents, control)` builds sibling resources by exact `$id`. `index_documents` at line 306 rejects missing string `$id` and duplicate sibling IDs deterministically in supplied array order. `resolve_reference` at line 462 resolves current document `#`, exact current `$id`, sibling exact `$id`, and `/` JSON pointer fragments; array numeric segments work, `~1` and `~0` are decoded. It is private and does not expose resource dependency discovery.

Unknown cross-document references and missing pointers return explicit errors. Non-pointer fragments are refused. This does not implement URI relative resolution, nested `$id` scopes, percent decoding, or anchors despite accepting `$anchor` as metadata. Root `$id` collision against sibling index is not separately rejected. Schema compilation validates the main root; sibling documents are parsed/indexed but are not all independently compiled or validated for unsupported keywords.

`ValidationControl` has shared AtomicBool cancellation and max_nodes; traversal checks at visited nodes and returns `ValidationProgress { visited_nodes }` at completion. Parsing and sibling indexing occur **before** this traversal, outside its cancellation and node accounting. There are no resumable resource-capture or parse steps on this compile entrypoint.

`pack::json::parse_object` at line 900 performs `object.insert(key, value)` and overwrites duplicates. Controlled parser also finds duplicates and replaces existing values (line 1580). Thus **even the owned Rust parser does not certify duplicate refusal**. Compile malformed JSON reports owned errors, but duplicate text is silently normalized before resource validation. Do not describe the existing validator as strict document authority without fixing this lower provider policy.

## Canonical Closure Direction

Capture physical schema byte inputs and deterministic declared resource coordinates before projection. Reject duplicate decoded members, conflicting `$id`, unknown files/references, malformed JSON, and unsupported resolution forms at the neutral owned source authority. Preserve exact source path-to-resource mapping, rather than silent cached root lookups. Expose one resource-set/reference policy from general Schema, shared by producer and validation; compiler inputs should consume its captured closure. Keep JSON lexical/member authority below Schema and UI. Use system JSON parsing only after owned text-level duplicate refusal. Extend actual lower owners, not Repo scanners or OS catalog helpers. Add language-agnostic duplicate/escaped-duplicate, unresolved `$id`/pointer, root-vs-sibling collision, relative/nested identity policy, malformed, cycle, cancellation, and deterministic closure order laws with a third-party dev-only oracle.

## Exact Input Receipt

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/🧬️schema/🟦️.ts` | 15150 | `a7d7c20d59b42a1734d33d36a49c790043a63fc3384a0451b66bb1a4498d5548` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` | 10467 | `2617c03029c26a0cb97b3cf7fab1fa159b03c3f821d163ba55a55902c76170f3` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/🦀️.rs` | 76197 | `cf6bde44af639e9cd5678294e693985c9bcbc0bea1138f1cab28c4eaa69d54da` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/Cargo.toml` | 529 | `d24687f6c40037d6d1cb88cc86b2524ed8f0d63fa6349b750aa9d8cae4363823` |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/🦀️.rs` | 269 | `1f9e7ef4cd2010710825b7a937c94287769dd3f5bae33c58736f2cce3a97f41d` |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️.ts` | 12965 | `97a0022cc2c105e11dba157502a5218a07a73b8f72433a742a70f3dc77c43236` |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🧾️value/🟦️.ts` | 13144 | `30fc0fd10f6ac1f9be90c36dc17d5848cd7af7fbaa6fb3529d9a601495deddde` |
| `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs` | 76901 | `65c771273827c85ab9dedfc9f007d8c5caa0538a140ac2c5f952d86f8810da47` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️json-document/🟦️.ts` | 773 | `cafa8433d59b6483a10eb7fdc793272cc691aca7dad79b198cfe9fa4c9d8ab7b` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` | 69398 | `8e07082af24ba8c9d6d03e8f2c6231e9cad1527d60307b645a92ecf73750702f` |

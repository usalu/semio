# norm-eurocode describe: fuel exhausted at 32 G (2026-11-22)

## Symptom

`semio-plugin-describe describe` on the linked `norm-eurocode` component (`wYB76Gm/.../0b02b666ad54ccb20e878591.linked/component.wasm`, 224 950 465 bytes) exhausts `DESCRIBE_FUEL_BUDGET = 32_000_000_000` after ~896 s. The component built at 08:07 (core module 224 341 825 bytes, 520 KB smaller) described fine at 08:12 with the same emitter cap. The interpreter (`🧠️interpreter`) has no change today, so host fuel accounting (candidate b) is ruled out, and the eurocode showcase fixtures (candidate c) did not change today (mtimes Nov 3 to Nov 21).

## Measurement

The owned interpreter charges exactly one fuel per wasm instruction, so fuel is a deterministic instruction count and a profile by function is exact. I temporarily sampled the top wasm frame and the whole frame stack every 1024 instructions in `CoreInstance::step` (`[DEBUG]`, removed again), ran the debug emitter on the copied component (copy: `🗑️generated/norm-eurocode.component.wasm`) and mapped function indices to the data section strings. Raw log: `🗑️generated/eurocode-fuel/run3.log`.

| fuel burned so far | share of fuel inside JSON parse (`semio_framework_pack_json`) | share inside `registered_input_schema_document` closure (fn 48288, "$id") |
| --- | --- | --- |
| 1.4 G | 57 % | 17 % |
| 7.8 G | 92 % | 78 % |

The share rises with progress, so the cost grows faster than the work: the late part of `describe()` (the `PluginBuilder` try-build, `runtime.extend_contributions(...)`) parses JSON schema documents again and again.

## Root cause

Chain: `PluginBuilder::try_build` -> `PluginRuntimeRegistry::extend_contributions` -> `WireMutationRosterEntry::owner` -> `WireMutationInputs::of` -> `mutation_input_defs(schema, &registered_input_schema_document)` once per owner mutation (editor and viewer rosters both enumerate them) -> one `registered_input_schema_document(id)` per cross-document `$ref` of every mutation payload schema (the reader caches documents only per reader, one reader per mutation).

`registered_input_schema_document` (`🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs`) scanned every registered JSON Schema export text, every referenced document and every framework document with `text.contains(id)`, then fully parsed (`parse` + `to_dsl_value`) each matching text until one had that `$id`. A document containing the id (every schema that merely `$ref`s the shared value, link or reference documents) is a match, so each lookup re-parsed several large documents. Cost per mutation is therefore roughly (registered bytes) + (parsed matching documents), and the roster pays it for every mutation: mutations times documents, superlinear in the plugin size.

Why it appeared today: `runtime.publish_declared_catalogs()` was moved before `runtime.extend_contributions(...)` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs` (line ~722). Before, the catalogs were still empty when the roster derived the inputs, so each lookup ended in a cheap "no schema document has $id" refusal (`WireMutationInputs::Refused`). Now every `$ref` resolves and the whole quadratic scan and parse runs. The more mutations and schema documents a plugin has, the worse it gets; `norm` and `norm-energy` are small enough to stay under the cap, `norm-eurocode` (EN 1990, 1991, 1997, 1998 with hundreds of mutations) is not.

## Fix

`registered_input_schema_document` now keeps a process-wide index (`Mutex<Index>`: set of already parsed `(ptr, len)` texts and a map `$id` -> document). A lookup is a map read; only texts not parsed before are parsed (each once, indexed by its own `$id`, first registration wins as before). Unresolved ids still scan, as before. Public behaviour is unchanged. Added test `registered_lookup_is_repeatable_and_indexed` next to `framework_input_documents_resolve_by_id_without_registration`.

The cap stays `32_000_000_000` (an algorithmic cause, not legitimately larger work). The temporary 96 G cap and the interpreter sampler were both reverted (`git diff HEAD` is empty for `🧠️interpreter` and `🖨️describe/🛂️descriptor-emission`).

## Not measured

The fix lives in the guest, so the new fuel needs a component rebuild (the coordinator's `build-play-components.ts`). Expected: the 78 %+ share disappears, leaving well below the 22.7 G that `architect` needed. If the rebuilt `norm-eurocode` still exceeds 32 G, re-measure with the same sampler idea and raise the cap to the next measured step with the docstring updated.

## Files

- `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs` (fix)
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🦀️.rs` (test)
- Temporary and reverted: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏛️instance-state/🦀️.rs`, `.../🖨️describe/🛂️descriptor-emission/🦀️.rs`

Note: the git index (staged) of `🧠️interpreter/🏛️instance-state/🦀️.rs` holds my sampler (the working tree and HEAD are clean). Someone staged it while it was in the tree; the archivist should re-stage that path before committing.

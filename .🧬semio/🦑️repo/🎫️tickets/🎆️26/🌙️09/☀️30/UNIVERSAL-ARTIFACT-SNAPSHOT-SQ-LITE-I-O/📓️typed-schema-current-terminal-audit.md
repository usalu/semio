# Typed Schema Current Terminal Audit

Read-only source audit on 2026-10-03. No source edits, Cargo execution, or Git mutations. Concurrent edits mean line references describe the inspected source and should be rechecked before editing.

## Typed Schema Functions

Generic `NativeSchemaControl` functions returning `ValueError` must propagate control and producer errors directly with `?`; converting control errors through `into_message()` introduces a `String` into that typed boundary.

- Framework `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs`: `identity_spec_controlled`, `step_spec_controlled`, and `progress_spec_controlled` at 1398–1400 initially had incorrect conversions. A subsequent read confirmed root's concurrent correction to direct typed `?` for begin stage, allocation, and step.
- OS Workflow `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs`: `workflow_enum_shape_controlled` at 167–168 incorrectly converts `begin_stage`, `copy_text`, and `step`; allocation already propagates directly. `media_contract_spec_controlled` at 221–230, `workflow_media_port_spec_controlled` at 467–477, and `workflow_input_spec_controlled` at 1155–1161 incorrectly convert `begin_stage`, allocation, and each step. Producer `field` calls already preserve typed errors.
- OS Run `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/📸️snapshot/🦀️.rs`: `run_enum_shape_controlled` at 47–48 incorrectly converts begin stage, text copying, and step. `run_trigger_spec_controlled` at 143–149 incorrectly converts begin stage, allocation, and steps. Producer calls already propagate typed errors.

## String Terminals

The following functions declare `String` errors and need explicit `map_err(|error| error.into_message())` at these remaining typed control operations:

- OS Workflow `media_contract_to_record_controlled`, line 194: `EncodedRecord::new(8, control)?`.
- OS Workflow `workflow_media_port_to_record_controlled`, line 209: `EncodedRecord::new(9, control)?`.
- OS Workflow `workflow_input_to_record_controlled`, line 215: `EncodedRecord::new(5, control)?`.
- OS Run `run_trigger_to_record_controlled`, line 132: `EncodedRecord::new(4, control)?`.
- OS Workflow `workflow_native_project`, line 188: final `control.step()` directly returns `Result<(), ValueError>` while function returns `Result<(), String>`.
- OS Workflow `workflow_native_project_optional`, line 191: absent-value branch final `control.step()` has the same mismatch.

Existing conversion in actual `DslField` record/value methods returning `String` is appropriate; it must not be removed with a broad replacement.

## Native Pack Facets

Inspected framework and OS pack value `🏭️schema/🦀️.rs` and `🛫️encode/🦀️.rs`. Schema graph, shape, edge, writer, sorting, equality, hashing, and map reservation functions retain `ValueError` and direct propagation; no `into_message()` conversion was found inside those typed schema facets. Both encode facets have `OutputError::into_pack` conversion into `PackError::Schema` at line 11, which is an explicit pack error terminal and is appropriate. The pack value modules mount their encoding and schema facets using `#[path]`.

No runtime or compilation success is claimed by this audit. Root's authentic JSON test is the compiler/runtime verification authority.

## Follow-Up: Constructor Audit After Manifest and Kernel Repairs

Re-read the actual framework Manifest and Kernel source after root's edits. The sites cited by `root-authentic-json-output17-tool-run-schema-current.log` now explicitly pass `InvalidValue`: Kernel WindowHandle at 53–54 and CapabilityToken at 77–78; Manifest NonEmptyVec at 5219/5222, AppRole at 5441–5442, TopicContribution at 5884/5896–5897, Version at 5950–5951, and VersionPin at 6127–6128. The former constructor function references in NonEmptyVec and AppRole are now closures passing both arguments.

Canonical DSL schema/record production subtree `🧰️framework/🔨️modules/🗣️dsl/🧬️schema`, OS DSL schema production subtree `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema`, framework/OS pack value facets, shared value control subtree, and framework composition/state/registry production subtrees yielded no remaining one-argument constructor or `map_err(ValueError::new)` sites in the inspected calls. OS `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🏭️producer/🦀️.rs` contains no `ValueError::new` calls: its field/record/boxed constructors preserve control errors directly.

Stdio JSON `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot` production files contain no `ValueError::new` or constructor `map_err` sites. Their mount is explicitly present in the JSON artifact root at lines 231, 234, and 236.

One previously inspected known production owner still contains seven one-argument calls: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs`, `MediaContract::from_value`:

| Line | Concrete Trigger |
| --- | --- |
| 369 | Input value is not an Object. |
| 385 | Conversion array has no first element. |
| 386 | Conversion array has no second element. |
| 392 | Conversion value is neither Array nor Null. |
| 399 | Object lacks kindId. |
| 400 | Object lacks mediaType. |
| 401 | Object lacks wire. |

These are explicit typed-value decoding failures; each call currently passes only its message. No message classifier is proposed. Whether this owner is reached by the current JSON gate remains a compiler determination; this audit does not claim that it is.

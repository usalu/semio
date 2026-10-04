# Canonical Print Chart Mutation and Inference

The print product owns its schema, snapshot, guarded diff, Change Chart Value mutation and derived inference under `🧰️framework/🛍️products/📓️print/🧬️schema`. Native facets implement the existing framework `Mutation`, `MutationDiff`, `DiffAlgebra`, `DiffRegions`, `Inference` and `InferenceSpec` contracts. The product publishes `s.print.chart` with schema and inference capabilities through the existing transactional plugin assembly and `ArtifactInferenceServiceRegistry`.

Rust snapshots preserve the framework-owned `DslValue`; TypeScript snapshots carry the shared chart contract. Wire mutation encoding distinguishes an explicitly authored null from an omitted value. Diffs validate intermediate preconditions and reject atomically. Removing an array element shifts following positions, and inverse mutation restores the parent array so surviving elements are retained. Language-neutral JSON vectors define replay, inverse restoration, rejection and collision cases.

Native inference validates the shared chart schema and returns TikZ, structured diagnostics and completion status. Invalid charts produce empty TikZ and `complete=false`. Registry dispatch decodes and encodes the canonical framework pack payload, matches direct inference and enforces cancellation, monotonic checkpoints and work/allocation/depth budgets. No extra native serialization execution route exists.

The emitted vocabulary uses named tables and scales, per-layer prepared tables, transforms, layouts, complete channel bindings, explicit coordinate frames and options, localized titles, themes, guides, annotations and canonical catalogue preset dispatch. Generated table and constant-column names avoid authored identifiers. CSS named, RGB, HSL and hexadecimal paints resolve to deterministic aliases; fractional fill and stroke alpha remain independent and combine with authored opacity.

## Verified Native Tests

A fresh registered `@semio-tech/print:test-native-grammar` invocation on 2026-10-04 executed `@semio-tech/print-rs:test` through the existing native owner command. Its 11 tests passed, with zero failures, in 0.24 seconds on the final successful invocation. The nested native target completed in 25.1 seconds, and the enclosing full compiler route passed with exit code 0 in 3 minutes 36 seconds. Rust compiler warnings from shared owners do not change the recorded test outcome.

| Test | Verified behavior |
| --- | --- |
| `mutations_replay_inverse_and_reject_atomically` | Language-neutral replay, composition, inverse restoration and rejected malformed edits |
| `array_removal_inverse_restores_shifted_elements` | Both inverse diff and inverse mutation restore the full array |
| `native_registry_dispatch_is_deterministic_and_controlled` | Plugin registration, published service dispatch, deterministic canonical payloads, direct inference parity, progress, cancellation and invalid results |
| `missing_language_is_diagnostic_data` | Incomplete invalid input returns structured diagnostics and no TikZ |
| `shared_result_schema_accepts_valid_and_invalid_native_outputs` | First-party shared result schema validates both valid and invalid inference payloads |
| `css_paints_preserve_named_rgb_hsl_and_fractional_alpha` | Neutral paint fixtures, declared named colors, invalid inputs, alpha declarations and annotation opacity |
| `explicit_null_is_distinct_from_omitted_value_in_wire_mutations` | Wire round trip retains null versus removal and default inference remains consistent |
| `derived_names_do_not_overwrite_authored_tables_columns_or_text` | Prepared table and constant-column collisions avoid overwriting authored values or text |
| `every_catalogue_kind_is_reachable_through_snapshot_inference` | Every canonical catalogue slug dispatches through the same snapshot inference |
| `native_chart_source_can_be_compiled_by_the_print_toolchain` | Actual native inference emits the neutral eighteen-mark fixture to the compiler test route |
| `typed_rows_preserve_null_strings_booleans_and_color_fallback_alpha` | Typed null, empty authored string and booleans remain distinct; functional CSS fallback gets a declared color alias and fractional alpha |

The compiler-source test generates source; the enclosing print target is responsible for compiling it. Source generation alone is not a TeX compilation pass.

## Actual Native Compiler Result

The registered full print target passed at terminal on 2026-10-04 with task caching skipped. It emits fresh `native-chart.tex`, stages authored print fonts and invokes pinned Tectonic through the canonical print toolchain. The actual emitted full figure compiled and its 6 typed numeric values and fractional functional paint fallback alpha were measured successfully inside the figure's color scope.

The enclosing route passed all 18 independent D3 primitive mark paths and styles, ordered detail paths, clipping, all 7 coordinates, numeric and temporal controls, RGB/Lab/HCL/OKLab color scales, paint alpha, physical font units and custom palette wrapping/accessors/reset. Stock/custom graph name and group ordering, 6 measured minor ticks, 37 transform columns and 137 layout vectors passed. Shared direct scale cases matched D3 for 518 typed inputs across 57 configurations; 10 shared renderer gap/nullable-paint cases and 12 additional unknown fallback cases also passed.

Native row emission preserves typed null, undefined, strings and booleans through the existing data commands. Ordinal scale domains use the same typed representation. Paint-bound scale fallback options resolve functional CSS paints through the existing deterministic alias and alpha collection. These remain canonical shared chart schema controls and inference output. All 11 native tests ran after the shared result schema was closed.

No external runtime library is added. AJV, D3, dagre and the existing CSS conversion test library provide independent test references; runtime pack, value, schema validation and plugin execution remain first-party framework facilities. The exact changed source/fixture/script and outside-product registration inventory is retained in `📓️native-final-verification-2026-10-04.md`. The registered inference build also passed canonical inference/worker and separate native runner/helper strict TypeScript compilation in 1 minute 48 seconds, publishing 248 exports. Test-only declaration fixes add no runtime dependency.
Current continuation independently replayed the registered Rust route inside full native retry 13: all 11 current native tests passed in 0.30 seconds, nested Nx route terminal 0 in 1 minute 1 second. The outer registered compiler route also reached terminal 0 in 77 minutes 29 seconds, including actual compiled Rust-emitted TikZ. This compiler result strengthens the source-generation tests; current isolated family repairs and their composite closure limits are recorded in `📓️native-final-verification-2026-10-04.md` rather than attributed to Rust mutation semantics.

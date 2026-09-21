# Flow20 Neural Dictionary Cold/Retained Boundary Audit

## Result

The supplied Flow20 receipt proves a **test-fixture cold-owner leak**, not a
live Flow editor, store, evaluation, or boot-path failure. The failing test
never reaches `flow_app_with_registry`'s application construction: the
hand-authored `FlowExtensionManifest` is dropped as the `Once` closure in the
artifact unit-test context returns. Its `math.add` operator contains
`ChannelSpec::number_default`, whose `Dictionary` default reaches the neural
fail-closed `Dictionary::drop` guard.

The current source has the required narrowly scoped cleanup. It has **not
been rerun** as part of this read-only audit.

| Classification | Evidence | Outcome |
| --- | --- | --- |
| Confirmed fixture defect in the historical receipt | [`dictionary-backtrace-standard-stack.log`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/flow-native20-full/dictionary-backtrace-standard-stack.log) ends at `install_first_party_light_flow_extensions_for_tests::{closure#0}` after `FlowExtensionManifest → OperatorInfo → ChannelSpec → Value → Dictionary`. | A process-wide `Once` fixture panic can fan out into many apparent Flow20 failures by poisoning later users of that fixture. Those failures do not establish independent runtime owner leaks. |
| Current artifact fixture repair | [`unit/🦀️.rs:65`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:65) serializes the manifest first, then explicitly retires each copied schema and operator at [lines 90–92](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:90), before installing its JSON. | Correct cold boundary for that hand-authored, throwaway manifest. |
| Current shared first-party manifest path | [`build_manifest_json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs:70) snapshots registry catalogue data, encodes it, and retires the copied schemas/operators at [lines 79–82](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs:79). All first-party extension manifest entry points delegate to it, for example [`primitive:143–145`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️.rs:143). | No remaining receipt-supported manifest-builder leak is visible in the production extension path. |
| Registry admission is a retained transfer, not a discard | [`register_contributed_manifest`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:196) moves admitted schemas/operators into the registry and explicitly calls `retire_cold` for duplicate values at [lines 199–210](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:199). The enclosing `ColdOwner<Registry>` retires a partially built registry on a refusal at [lines 220–231](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:220). | No production registry repair follows from the supplied stack. |

## Exact Causality

The fixture originally owned a full `FlowExtensionManifest` solely to generate
JSON. JSON encoding borrows it; it does not transfer the nested neural values.
When the closure returned, ordinary Rust destruction attempted to release the
operator's default `Dictionary`. The neural engine deliberately rejects that
route: [`Dictionary::drop`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:98)
asserts unless the final owner used a retained owner or an explicit cold
boundary.

The correct fixture sequence is therefore:

1. Construct the throwaway manifest.
2. Encode it while it is still intact.
3. Retire every neural-bearing contribution explicitly.
4. Pass only the resulting JSON string to `install_flow_extension_manifest`.

The active artifact fixture implements exactly that sequence. The host test
fixture does not construct a neural manifest; it receives JSON strings from
the shared first-party encoder, which already performs its own retirement.

## Boundary Contract

`FlowExtensionManifest` is a transient **cold source** only at an encoder or a
test fixture. It is not the retained registry representation. The retained
boundary begins when `register_contributed_manifest` transfers decoded schema
and operator values into `neural::Registry`; the JSON text remains the
persisted contributed representation.

The existing `ColdOwner` semantics support this distinction: dropping a
`ColdOwner<T>` calls `T::retire_cold`, while `into_inner` transfers ownership
to the retained state ([`cold/🦀️.rs:8–18`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧊️cold/🦀️.rs:8)). Do not retain a fixture manifest merely to avoid cleanup, and do not add a
runtime adapter around its JSON.

For long-term API clarity, the Flow manifest module could implement
`ColdRetire` for `FlowExtensionManifest`/`FlowExtensionContributes` and make
the two existing manual vector retirements one `manifest.retire_cold()` call.
That is a cleanup improvement, not a prerequisite for the receipt's repair:
the current two fields are the only neural-bearing fields and are already
explicitly retired.

## Fail-First Validation

Run the exact receipt target first, then the Flow20 group. The target is
already a meaningful regression route because it constructs the same
`number_default` dictionary and calls the singleton fixture before any Flow
application exists:

`editor::flow::commands::add_widget::tests::add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content`

Its pass condition is that fixture installation returns and the command's
actual typed child-edit assertions execute; it must not be replaced by a
source-string assertion.

Add one narrowly focused encoder law only if stronger local coverage is
needed: build a manifest containing an operator with a default dictionary,
encode it through `build_manifest_json`, leave the scope, and use the existing
`serde_json`/TypeScript manifest-admission fixture to verify the emitted JSON.
Removing the encoder's explicit retirements must make the Rust side fail at
the same `Dictionary::drop` guard, while the independent JSON oracle confirms
that the wire payload was not weakened.

After that targeted test is green, rerun the previously widespread Flow20
set. Treat a remaining failure as a separate runtime investigation only if it
has a receipt whose stack escapes the fixture `Once` and reaches a live Flow
owner such as `FlowHost`, `VcsArtifactApp`, or an evaluation/store retirement
cursor.

## Confidence and Limits

High confidence for the historical root cause and boundary classification:
the recorded stack has a single, direct ownership chain and current source
matches the required cleanup sequence. Medium confidence that this removes all
Flow20 failures until the changed fixture has been rerun; no commands were run
for this audit.

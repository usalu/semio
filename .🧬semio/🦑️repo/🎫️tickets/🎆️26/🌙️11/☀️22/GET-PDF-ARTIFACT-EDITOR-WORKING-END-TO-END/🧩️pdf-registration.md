# 🧩️ PDF Registration, Definition, Demo Asset and Save Fallback

Scope: tasks 1 to 4 of the registration/declaration pass. Verification status is stated per item at the end.

## 1. Composition paths

Consumer of `semio.stdio.composition-contribution/v1`:

- Contract and admission: `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/📋️contract/🟦️.ts` (`admitCompositionContributionV1`, closed objects, `type`/`factory` only need to be Rust paths).
- Generator: `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts` (`prepareStdioComposition`), run by `composition prepare` as a Cargo preparation step of every hub component crate. It prints `registrations = builder.<role>::<app.type>(app.factory())` verbatim into the generated `🧩️extensions/📘️pdf/🦀️.rs` and the `dyn_enum_close!` enum.

Finding: the audit claim "these paths do not exist" is wrong. The generator uses the strings verbatim, so they must resolve from the crate `semio_s_artifact_stdio_pdf` root. They do:

- `semio_s_artifact_stdio_pdf/🦀️.rs` declares `pub mod editor { pub mod pdf14a|pdf14|pdf14x|pdf17a|pdf17|pdf17e|pdf17h|pdf17ua|pdf17vt|pdf17x { pub use <owner crate>::editor::<name>::*; ... } }` and the same for `viewer`.
- Each owner crate includes the subset's `✏️editor/🦀️.rs` (macro call `pdf14_editor!`/`pdf17_editor!`, or the handwritten `Pdf17Editor`/`create_pdf17_editor` of the 1.7 base), so `Pdf14AEditor`, `create_pdf14_a_editor`, `Pdf17Editor`, ... are glob re-exported at exactly the paths in `🧩️composition/🔣️.json`.
- The PDF crate re-exports through its own `editor`/`viewer` modules rather than through the subset tree (XLSX instead lists the owning sub-crate and `symbolPackages`); both are valid, the generator only requires that the path resolves from the declared `package`/`symbolPackages`. No change to the JSON or the crate exports was needed; the existing design already has a single canonical flat path per app.

## 2. `📜️artifact-definition.json`

XLSX's 27 mutations and 1 inference are handwritten (no generator): every id is `s.stdio.xlsx.mutation.<kebab-leaf>.v1`, `executable_registration: false`, `status: implemented`, mirrored in `support_ledger`. The contract is enforced by `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🏗️assembly/🦀️.rs` (`violations`, run by `🧪️tests/🗿️bundled-definitions`): ids must be `<owner>.mutation.<semantic>.vN` with no dot in `<semantic>`, ledger lists must equal the collections, a ledger state `implemented` needs normative sources/validators/fixtures, `resources` must stay `unimplemented`.

PDF was brought to the same truthful state, by hand:

- `mutations`: 61 entries for every leaf of the 1.7 base `PdfMutation` (`s.stdio.pdf.mutation.<leaf>.v1`, leaf names are the `$ref`s of `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔣️.json`), plus the 5 leaves of the 1.4 base `PdfMutation` as `s.stdio.pdf.mutation.pdf14-<leaf>.v1` (the 1.4 `insert-page`, `remove-page`, `move-page` have different payloads than the 1.7 ones, hence the prefix). All `implemented` with `executable_registration: false`; every one of the 66 leaves has `#[test]` coverage in its own `🧪️tests` folder (verified by a scan), the 1.7 ones additionally have before/after fixture pairs under `🧪️tests/🧫️fixtures/🧬️mutations`.
- `inferences`: the `outline` inference of the 1.7 base (`s.stdio.pdf.inference.outline.v1`) and of the 1.4 base (`...pdf14-outline.v1`), both with unit tests.
- `profiles` and `source_dialects` (1.4 and 1.7 `any`/`source`): `unimplemented` -> `implemented`, as for XLSX; the native codecs for both are `implemented` and registered.
- `support_ledger`: `mutations`/`inferences` mirror the collections; `read`/`write`/`lossless`/`canonical` `unimplemented` -> `opaque` (XLSX's value: the codec is a real round trip but there is no verified normative source, so `implemented` would be a false claim).
- Left `unimplemented` on purpose: `resources` (external references, enforced by the validator), `conformance_suites` public-corpus with empty fixtures (there is no suite runner for PDF like XLSX's calamine suite yet), and the mutation leaves of the subsets `a`, `x`, `e`, `h`, `ua`, `vt` (not declared yet; they overlap by name, so they need subset-qualified ids).

## 3. 1.7 base demo asset

- Added `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/🎬️demo.pdf` (906 bytes, one 200x300 page with the text "Semio"). It is the exact byte sequence that `🗣️.dsl.semio` already carried as hex (the `demo_pdf17_snapshot()` fixed point), extracted from there so asset, DSL and Pack agree.
- The 1.4 base and XLSX demos are not loaded by an example picker from the binary asset: the editor `examples()` list uses the DSL text (`ExampleSource`), and the binary file is a natural-file fixture checked by the demo test. The 1.7 base editor registers no `examples()` at all (also true for XLSX); that is unchanged. The real file is reachable through the acceptance matrix, so it is appended to the valid `pdf` samples in `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧫️fixtures/📇️samples.json`.
- Test: `demo_pdf_asset_is_the_natural_file_of_the_demo_snapshot` appended to `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📤️export/🧪️tests/🔬️unit/🦀️.rs`: asset equals `encode_pdf(text_document(...))`, decodes to `demo_pdf17_snapshot()`, page text is "Semio", and the independent `lopdf` oracle (`reference_page_count`) reads one page.

## 4. `💾️native-save/📤️producer.rs` fallback

`decline()` is reached from `advance()` (`PlanDecision::ReferenceRequired`) and from `🧾️job.rs` (`retained_control_extent` is `None`). `ReferenceRequired` comes from `🧱️program.rs` when the retained graph cannot be saved natively within the bounded grants (graft/quote failure, shape/semantic refusal, deferred lift, oversized trailer). It is therefore reachable from a real PDF Save for documents outside the native qualification (not for the blank, demo or ordinary decoded documents, which the editor tests save). There is no reference writer on the bounded job path, so a real fallback would break the bounded-memory contract; instead the refusal is made honest: the misleading `MediaError::NotImplemented` ("media ports are not implemented for this app") is replaced by `MediaError::Payload("artifact:native", REFERENCE_REQUIRED)`, built through the same funded `construction.built` path as the success result, with the heap accounting of its strings. No test referenced `NotImplemented` for this path.

## 5. Verification and open points

See the bottom of this file for the final status written after the builds.

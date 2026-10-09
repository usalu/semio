# Native Catalog Factory Retention Current

Read-only 2026-10-09; no runtime qualification.

`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:131–156` NativeCodecBinding currently owns plugin_id/package_id/artifact_kind/codec only. Its constructor139 accepts those four arguments. Add private `factory_id: Option<String>`, explicit constructor argument, and read accessor. Keep actual None for callers without factory receipts; do not infer one from kind/schema/package.

Three authentic factory boundaries discard already verified factory identity in `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs`:

- GIS105: `identity.factory_id` is original &'static str (`🌎️hub/🧩️compositions/🌍️gis/📇️native-codecs/🦀️.rs:18`), validated uniqueness95. Pass Some(identity.factory_id.to_owned()).
- VCS133: original &'static str (`🌿️vcs/📇️native-codecs/🦀️.rs:17`), exact value vcs.vcs.v1 validated124, uniqueness125. Retain the original value, not the validation literal.
- Stdio188: original receipt factory_id is owned String; already checked against actual compiled receipt roster170/172 and unique178. Move Some(receipt.factory_id) with original plugin/package/kind into binding, after instantiate/validation186. No guessed join required.

Observed remaining constructor calls are actual trusted-catalog unit fixtures at630,663,1102,1695,1715,2556 (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs`). They manually create fixture codecs without original provider factory receipt: supply None, preserving their actual i64/Vec byte native identity. Existing clone of bindings naturally retains Option. Ensure any equality/selected-provider publication descriptor later includes retained factory only when Some.

Neutral first-party contracts already exist in native-openable-provider `🧬️schema/🔣️.json`, fixture `🧫️fixtures/🪪️v1/🔣️.json`, GIS/VCS fixture siblings, plus Stdio catalog `📇️catalog/🧬️schema/🔣️.json` and `🧫️fixtures/📇️native-catalog-surface/🔣️.json`. These original receipt identities should be extended/read to assert the retained projection at the binding boundary; do not author new arbitrary factories. No Source native-openable-provider/trusted-catalog TS implementation was found in those module folders. Actual Source Stdio publication counterparts are `🌎️hub/🧩️compositions/🗄️stdio/📇️publication/✅️trusted-stdio-catalog/🟦️.ts` and test sibling. Use them with original neutral fixture as the language-agnostic semantic projection oracle; independently parse/validate with existing third-party JSON/schema tooling.

Native law route: native-openable-provider `🧪️tests/🔬️unit/🦀️.rs` already validates swapped IDs183–187 and duplicate IDs201. Add equality between every returned authentic binding factory_id and its original receipt factory_id for GIS/VCS/Stdio, and None on manual trusted fixture binding. Assert SQL absence is preserved, not filtered. Factory-retention evidence does not establish complete binding denominator: static33 and Stdio89 remain separate until actual prefilter selected binding/open-target ledger capture is joined through real receipt identity.

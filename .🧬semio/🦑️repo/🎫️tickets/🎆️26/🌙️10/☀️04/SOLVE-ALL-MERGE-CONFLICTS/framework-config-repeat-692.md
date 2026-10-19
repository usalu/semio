# Neutral Config and Backbone Source Preservation

Kept the current product-to-neutral module extraction. Carried incoming sparse identity and local-catalog diff semantics into the current shared vocabulary, direct semantic leaves, dispatch and twin applier. Kept worker-specific facet state/folding at the Store worker, and moved the shared identity fold plus binding precedence into current backbone owners.

- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/🗃️local-catalog-config-mutation/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/🆔️identity-config-mutation/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/📤️retire-local-document/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/📤️retire-local-document/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/⚙️apply-diff/🟦️.ts`
- `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`
- `🧰️framework/🔨️modules/🛰️backbone/🪪️config-lane/🟦️.ts`
- `🧰️framework/🔨️modules/🛰️backbone/🪪️config-lane/🟦️.ts`
- `🧰️framework/🔨️modules/🛰️backbone/🟦️.ts`

## Import and Declaration Closure

Moved new mutation/diff pair builders into neutral dispatch. Removed the duplicate incoming product TypeScript twin applier after assigning its source to the shared neutral owner. Rewired code imports to the existing framework config mutation public entry point; Rust fixture URLs remain with their authored fixture owner.

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧵️backbone-worker/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚙️config-diff-twin/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗂️local-catalog/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🧪️tests/🌐️preference-lane/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config-facet-fold/🟦️.ts`

## Explicit Incoming Path Disposition

- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⚙️apply-diff/🟦️.ts` → `🧰️framework/🔨️modules/🎚️config/🧬️schema/🧬️mutations/⚙️apply-diff/🟦️.ts`: canonical incoming TypeScript sparse-diff twin moved to the current neutral config owner. Product source copy removed; authored logic preserved with neutral vocabulary imports.

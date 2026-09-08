# OS UI Preference and Plugin View Context Ownership

## Result

The persisted local OS preference authority is `UiPreferences` in the shell schema and `os.config.ui-preferences` in the OS config mutation vocabulary. The record has optional selections for appearance, layout, driver, locale, terminology, and theme, plus keyed custom drivers, custom themes, and keybinding overrides. Absence of locale remains representable. Draft driver/theme editor state stays ephemeral and is not part of this config facet.

The plugin author API now receives the canonical `ViewModel` directly. `ArtifactApp`, `ArtifactEditor`, and `ArtifactViewer` render and context-menu methods receive `&ViewModel`; request-context wrappers and Editor/Viewer adapters forward the same value. `VcsArtifactApp` forwards its host-provided value rather than deriving a copied locale context. Context-menu JSON and packed wire requests require `viewState` and forward it through the object-safe plugin boundary. The contract test rejects a context-menu request without `viewState` and verifies German/reuse axes survive decoding.

## OS Config Contract

`UiPreferencesConfigMutation` contains nine direct, invertible leaves:

- `SetAppearance`
- `SetLayout`
- `SetDriver`
- `SetCustomDriver`
- `SetLocale`
- `SetTerminology`
- `SetTheme`
- `SetCustomTheme`
- `SetKeybindingOverride`

Scalar payloads accept an optional value so an inverse can restore an unset host-derived preference. Keyed custom values accept an optional value so removal is a first-class event and inverses restore the prior entry. Every leaf owns its Rust source, TypeScript face, JSON payload schema, and mutation descriptor under `🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/`.

The whole-record `UiPreferencesDiff` is owned by the OS config facet. This avoids an orphan implementation on the shell-owned `UiPreferences` value while retaining one canonical preference record.

## Language-Neutral Fixture

`🎨️updates-every-os-ui-preference` starts from all optional selections unset, applies all nine mutation kinds, reaches one fully populated record, records each inverse against its pre-state, and restores the original record in reverse order. The committed JSON is consumed by both the Rust test and the TypeScript/Ajv ticket validation.

## Validation

- Passed: ticket-local Bun+Nx target `abstraction-ownership-validation:ui-preferences-fixture`.
  - Ajv strict validation accepted all nine language-neutral mutations.
  - TypeScript fold matched the committed after snapshot.
  - Reversed TypeScript inverses restored the committed before snapshot.
- Passed: `git diff --check` for the OS config, plugin SDK, plugin SDK contract tests, and ticket validation files at the time it was run.
- Pending at handoff: ticket-local Bun+Nx target `abstraction-ownership-validation:plugin-host-check`, unified exec session `87769`, compiling with `CARGO_TARGET_DIR=../🗑️generated/cargo-plugin-host` and `CARGO_NET_OFFLINE=true`. Its output is live PTY output; no completion has been claimed.
- Also pending: the first shared-target `@semio-tech/framework-plugin-host:check`, unified exec session `94189`, remains queued behind unrelated workspace Cargo/Nx work. It must not be treated as a result.

## Files Owned by This Pass

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🎨️ui-preferences/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts`
- all files under `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/`
- ticket validation `project.json` and `📜️script.ts`

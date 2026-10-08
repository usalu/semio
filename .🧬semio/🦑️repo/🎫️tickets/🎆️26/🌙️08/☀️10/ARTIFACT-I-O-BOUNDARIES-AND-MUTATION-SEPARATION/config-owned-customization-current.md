# Configuration Owned UI Customization

The canonical UI configuration schema exposed `serde_json::Value` through a JsonValue alias and custom ToValue/FromValue bridge. UiDriver.config, UiTheme.config and UserNamedLayout.layout now own semantic first-party DslValue directly. UiDriver, UiTheme, UserNamedLayout and UiPreferences derive first-party admission and reject undeclared record fields. These four schema records contain no physical Serde or Schemars annotations. Arbitrary nested customization values retain intrinsic bytes, exact integer widths and object member custody without a JSON projection.

The existing ShellState/ShellCommand native external facets consume these records. Their required Serde and Schemars implementations belong to native `config/🚪️io/📝️text/🎨️ui-preferences/🦀️.rs`. The schema interface reads the authored neutral UI preferences JSON schema and prefixes native definition identifiers OsConfigUi to preserve reference identity. It does not expose external value types from domain fields. No compatibility record or legacy admission path was added.

Renderer drivers construct and borrow semantic axes directly. Saved WindowLayout reads/writes use its existing FromValue/ToValue. ThemeDocument, ThemeIcons and ThemePaintRef now have explicit first-party derives, with ThemeNumber owning its scalar/list sum algebra directly. Persisted theme projection removes id/label from the semantic member sequence and restores them without external JSON conversion. Physical export remains in its current host codec. Three Shell unit constructors use owned semantic values directly.

## Validation

The registered source witness4508 executed RED before editing, naming the exported external JSON bridge. The neutral fixture declares12 JSON shapes, exact u64/i64 boundaries and Unicode; independent AJV checks authored customization schema. Two native laws cover semantic roundtrip, independent Serde physical comparison, intrinsic bytes/duplicate semantic keys, exact sparse driver inverse, and the native external schema interface. Source68315 executed GREEN exit0 with DEBUG12 independent AJV samples. Native80794 executed RED before Config with511 OS kernel errors: the first3 remain UI Scene math1078/1080; the next508 are General's broader Store/factory consumers. No Config native body executed. A strengthened source check additionally parses all four owned native record/facet/law/renderer owners with Rustfmt; current2 session54432 executed GREEN exit0 with the same DEBUG12 AJV cases after all four Rustfmt syntax parses. The two native laws and renderer typechecking remain unexecuted because the kernel511 floor stops the package. Store receiving remains blocked at shared UI Scene math1078/1080 from actual81101 and no receiving runtime pass is claimed.

## Exact source manifest

- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🎨️ui-preferences/🧫️fixtures/🌱️owned-customization/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🎨️ui-preferences/🧪️tests/🌱️owned-customization/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🚪️io/📝️text/🎨️ui-preferences/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🚪️io/📝️text/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🔬️unit/🦀️.rs`
- Ticket `construct-geometry/📜️script.ts` plus launch seed/live registration.

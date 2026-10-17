cargo-features = [ "trim-paths" ]

[workspace]
resolver = "2"
members = [
    "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📇️descriptor/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📇️descriptor/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️content/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust",
    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️frames/📦️packages/🦀️rust",
    "📦️packages/🦀️rust",
    "🔨️modules/🏠️host/🧰️owned/📦️packages/🦀️rust",
]
exclude = [
    "**/🏭️bridge/**",
    "**/👽️guest/**",
    "**/🔮️oracles/**",
]

[workspace.metadata.semio.repository]
schema-version = 1
exclude-patterns = [
  "**/👽️guest/**",
  "**/🔮️oracles/**",
  "**/🏭️bridge/**"
]
member-manifests = [ "**/📦️packages/🦀️rust/Cargo.toml" ]

  [workspace.package]
  version = "0.1.0"
  edition = "2021"
  rust-version = "1.95"

[workspace.dependencies.semio-framework-artifact-reference]
path = "../../../../../🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-job]
path = "../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-io-ports]
path = "../../../../../🧰️framework/🔨️modules/🚪️io/🔌️ports/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-pixels]
path = "../../../../../🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-tool-machine]
path = "../../../../../🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-ui-contract]
path = "../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust"

[workspace.dependencies.serde]
version = "1.0.228"
features = [ "derive" ]

[workspace.dependencies.serde_json]
version = "1.0.149"
features = [ "raw_value" ]

[workspace.dependencies.machine]
path = "../../../../../🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust"
package = "semio-framework-machine"

[workspace.dependencies.semio-framework]
path = "../../../../../🧰️framework/📦️packages/🦀️rust"
package = "semio-framework"

[workspace.dependencies.semio-framework-2d]
path = "../../../../../🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-async]
path = "../../../../../🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-async-macros]
path = "../../../../../🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-canvas]
path = "../../../../../🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-diagnostic]
path = "../../../../../🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-dsl]
path = "../../../../../🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-dsl-record]
path = "../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-dsl-record-derive]
path = "../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-hash]
path = "../../../../../🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust"
package = "semio-framework-hash"

[workspace.dependencies.semio-framework-os-kernel]
path = "../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
package = "semio-framework-os-kernel"

[workspace.dependencies.semio-framework-pack-json]
path = "../../../../../🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-plugin]
path = "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"
package = "semio-framework-plugin"

[workspace.dependencies.semio-framework-schema]
path = "../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust"
package = "semio-framework-schema"

[workspace.dependencies.semio-framework-schema-composition]
path = "../../../../../🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-schema-registry]
path = "../../../../../🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-ui-locale]
path = "../../../../../🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-value]
path = "../../../../../🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-value-dictionary]
path = "../../../../../🧰️framework/🔨️modules/🌱️value/🧾️dictionary/📦️packages/🦀️rust"

[workspace.dependencies.semio-framework-value-derive]
path = "../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust"
package = "semio-framework-value-derive"

[workspace.dependencies.semio-s-artifact-forms-forms]
path = "../../../📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-diff]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-editor]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-engine]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-examples]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-host]
path = "🔨️modules/🏠️host/🧰️owned/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-io]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-model]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-mutations]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-mutations-content]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️content/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-mutations-frames]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️frames/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-viewer]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust"

[workspace.dependencies.ui_render]
path = "../../../../../🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust"
package = "semio-framework-ui-render"

[workspace.dependencies.semio-s-artifact-layout-layout-descriptor]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📇️descriptor/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-descriptor-diff]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📇️descriptor/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-editor-commands]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-editor-panels]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-engine-export]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-layout-layout-engine-json]
path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/📦️packages/🦀️rust"

[workspace.lints.rust]
unsafe_op_in_unsafe_fn = "warn"
unused_lifetimes = "warn"
unused_qualifications = "warn"

  [workspace.lints.rust.future_incompatible]
  level = "warn"
  priority = -1

  [workspace.lints.rust.rust_2018_idioms]
  level = "warn"
  priority = -1

[workspace.lints.clippy]
cloned_instead_of_copied = "warn"
inefficient_to_string = "warn"
map_unwrap_or = "warn"
needless_pass_by_value = "warn"
semicolon_if_nothing_returned = "warn"
unnecessary_wraps = "warn"
redundant_clone = "warn"

  [workspace.lints.clippy.all]
  level = "warn"
  priority = -1


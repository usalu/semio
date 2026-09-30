# Cargo Role Declarations

Fresh real Cargo metadata reported 16 packages without a semio role before these edits. Each role below is an explicit purpose-based declaration, not a crate-prefix fallback. The Cargo checker retains physical ownership independently and will continue reporting actual upward edges.

Generic framework code generators and the generic plugin SDK declare framework; the five persisted OS document models declare artifact; Draw-owned FSM implementation/macro packages declare library; the shared imperative SDK declares s-module; the font asset executable declares tool. These changes create no new runtime dependencies or forwarding API. They do not move the generic FSM implementation or claim its current physical ownership optimal.

| Manifest | Package | Role | Identity |
| --- | --- | --- | --- |
| `🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-async-macros` | `framework` | `async-macros` |
| `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-value-derive` | `framework` | `value-derive` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-os-kernel-dsl-derive` | `framework` | `dsl-derive` |
| `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-schema-derive` | `framework` | `schema-derive` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-artifact-space-space` | `artifact` | `space-space` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-artifact-space-collection` | `artifact` | `space-collection` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-artifact-playbook-playbook` | `artifact` | `playbook-playbook` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-artifact-workflow-workflow` | `artifact` | `workflow-workflow` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-artifact-workflow-run` | `artifact` | `workflow-run` |
| `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-dispatch-macros` | `framework` | `dispatch-macros` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-plugin` | `framework` | `plugin` |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/📦️packages/🦀️rust/Cargo.toml` | `semio-s-plugin-draw-fsm` | `library` | `draw-fsm` |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/✨️macros/📦️packages/🦀️rust/Cargo.toml` | `semio-s-plugin-draw-fsm-macros` | `library` | `draw-fsm-macros` |
| `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/📦️packages/🦀️rust/Cargo.toml` | `semio-s-imperative-extension-sdk` | `s-module` | `imperative-extension-sdk` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-os-font-assets` | `tool` | `font-assets` |
| `🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-machine-derive` | `framework` | `machine-derive` |

The strict checker fixture/oracle and fresh metadata validation are pending; this report does not claim those checks passed.

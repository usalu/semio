# SVG, Office and OPC Typed API Prerequisites

Reconciled the actual XML ValueError helper API with SVG, DOCX, XLSX and PPTX consumers on 2026-10-03. Internal owner SQL helper chains, PPTX presentation helpers, and shared OPC producer helpers now preserve canonical ValueError until existing String trait or message-only IoError terminals. No adapters, compatibility wrappers, message classification, implicit String From, schema/literal/test changes, controller resets, or native behavior mounts introduced.

Work/row/identity arithmetic uses WorkLimit; owned byte arithmetic and caller byte ceilings use OwnershipLimit; actual reserve failures use AllocationFailed; existing reference and variant defects use InvalidValue. Existing shared producer errors propagate their supplied categories. DOCX/XLSX profile helpers required the same typed native control API prerequisites, with error conversion only at their actual IoError return. PPTX profile helpers were already typed and were preserved.

All Documents/Parts/PPTX presentation Owner and nested XML retirement guards remain intact. Shared OPC helpers directly use framework SQLite primitives and do not depend on the ZIP snapshot producer; no additional ZIP owner, archive feature or native provider hook was edited.

## Verification

Ran `rustfmt --edition 2021 --emit stdout` for 8 changed Rust files; 8 returned exit zero. Formatted output discarded. These are parser receipts only; no Cargo/typecheck/Native Nx/Source Nx/runtime was executed. Root retains the sole Cargo lane.

## Report Path Correction

Confirmed the prior report exists at `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📓️json-txt-xml-typed-api-prerequisites.md`. The previous chat link accidentally omitted SNAPSHOT from its ticket slug; no report was moved or renamed.

## Exact Changed Files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; terminal `into_message` sites 3.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; terminal `into_message` sites 3.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; terminal `into_message` sites 3.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; terminal `into_message` sites 10.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🧩️presentation/🦀️.rs` — parser exit 0; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🪶️sqlite/🦀️.rs` — parser exit 0; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🦀️.rs` — parser exit 0; terminal `into_message` sites 1.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🦀️.rs` — parser exit 0; terminal `into_message` sites 1.

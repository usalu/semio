# Wave R — Rename `protocol` (Blockly-like strict-list editor) → `playbook`

This is the blocking prerequisite for the whole ticket: it frees the `protocol/` directory and
crate name for the new binary op-log format technology. Do this FIRST, alone, before any other
wave starts. This wave touches shared/critical files — work carefully and verify the build at the
end.

## Rule

Rename a `protocol` hit **iff** it resolves to the strict-list visual editor technology: any path
under `protocol/`; the three crate/component/plugin ids (`protocol`, `protocol-plugin`,
`semio:protocol`, `protocol-module-procedural`, `semio:protocol-module-procedural`); Rust/TS
identifiers exported from crate `protocol` (`ProtocolSpec`, `ProtocolStep`, `ProtocolBlock`,
`ProtocolBlockOption`, `ProtocolVectorField`, `ProtocolExpr`, `ProtocolValidationError`,
`ProtocolOperation`, `ProtocolDiff`, `ProtocolEnvelope`, `ProtocolStore`,
`ProtocolBuilderConfig`, `ProtocolBuilderLabels`, `PROTOCOL_BUILTIN_KINDS`,
`PROTOCOL_BUILDER_LABELS_EN`, any `*_protocol_*` free function); `Contribution::ProtocolBlockKind`
/ the TS literal `"protocolBlockKind"`; schema id `"protocol.program"` / const
`PROTOCOL_DOCUMENT_SCHEMA`; contribution id `"protocol.blockKind"`; env `PROTOCOL_PLAY_PORT`;
`UpdateProtocol` op variant / action ids `"updateProtocol"`/`"update-protocol"`; script/launch
entry names containing `protocol` that reference this tech (`dev:protocol`,
`🛠️dev🧩️protocol⚛️react`, etc.); the `.dsl` file extension `"protocol"`.

**NEVER rename** — the generic word describing wire/communication protocols, unrelated to this
tech: `framework/sync/rs` `//#region 🔖️Protocol` + `framework/sync/worker/rs`'s `🔖️Protocol`
region (actor wire protocol — `PersistenceBinding`, `DocumentActorConfig`, `DocumentActorMsg`);
`framework/core/rs` `//#region 🔖️HubProtocol`; `framework/product/os/core/js`'s
`🔖️SyncProtocol`/`ComponentSceneProtocol`/`AppManifestProtocol` regions; `trinity/jack/lsp/js/protocol.ts`
+ its `🔖️protocol` region (LSP); `compose`'s `MessageProtocol`/`BridgeProtocol`/`protocolVersion`;
`cad/kernel/brepjs`'s `WorkerProtocol`; any prose use of the lowercase word "protocol" describing
a communication contract. When genuinely unsure, check whether the identifier/path traces back to
crate `protocol`'s exports — if not, leave it alone.

## Naming (types renamed too — greenfield, full consistency)

| Old | New |
|---|---|
| dir `protocol/` | `playbook/` |
| crate `protocol` | `playbook` |
| crate `protocol-plugin`, component `semio:protocol` | `playbook-plugin`, `semio:playbook` |
| crate `protocol-module-procedural`, component id | `playbook-module-procedural`, `semio:playbook-module-procedural` |
| contribution id `protocol.blockKind` | `playbook.blockKind` |
| schema `"protocol.program"`, const `PROTOCOL_DOCUMENT_SCHEMA` | `"playbook.program"`, `PLAYBOOK_DOCUMENT_SCHEMA` |
| `#[dsl(extension = "protocol")]` | `extension = "playbook"` |
| types `ProtocolSpec/Step/Block/...` | `Playbook*` (same shape) |
| `PROTOCOL_BUILTIN_KINDS`, `PROTOCOL_BUILDER_LABELS_EN` | `PLAYBOOK_BUILTIN_KINDS`, `PLAYBOOK_BUILDER_LABELS_EN` |
| op variant `UpdateProtocol`, action ids | `UpdatePlaybook`, `"updatePlaybook"`/`"update-playbook"` |
| `Contribution::ProtocolBlockKind`, TS `"protocolBlockKind"` | `Contribution::PlaybookBlockKind`, `"playbookBlockKind"` |
| playground variant `"protocol"` (ports 6085/6185 unchanged) | `"playbook"` |
| env `PROTOCOL_PLAY_PORT` (dev ports 6078/6178 unchanged) | `PLAYBOOK_PLAY_PORT` |
| script `dev:protocol` | `dev:playbook` |
| launch names `🛠️dev🧩️protocol...` | `🛠️dev🧩️playbook...` |

## Surface to sweep

**Own crates**: `protocol/rs/{Cargo.toml,lib.rs}`, `protocol/plugin/rs/{Cargo.toml,lib.rs}`
(`[[package.metadata.semio.playground]] variant = "protocol"`, `[package.metadata.component]
package = "semio:protocol"`, `consumes = ["protocol.blockKind"]`), `protocol/module/procedural/rs/
{Cargo.toml,lib.rs}` (`contributes = ["protocol.blockKind"]`). `git mv` the whole `protocol/`
directory to `playbook/` including `protocol/AGENTS.md` → `playbook/AGENTS.md` (move only — **do
not edit its content**, AGENTS.md edits are forbidden; its stale frontmatter/content becomes a
human todo you report at the end, don't fix it yourself).

**Dependents** (path deps + imports + aliases — update Cargo.toml path deps AND Rust/TS source):
`forms/rs` (Cargo.toml dep path + the `pub use protocol::{ProtocolSpec as FormSpec, ...}` alias
block + `FormOperation::UpdateProtocol` uses), `forms/plugin/rs` (`Contribution::ProtocolBlockKind`
matches, `"updateProtocol"` action ids, `forms_protocol_builder_config`/`render_protocol_builder`),
`flow/core/rs` (`widget_to_protocol_block`, test imports), `flow/plugin/rs`, `procedural/2d/rs`,
`procedural/3d/rs`, `procedural/plugin/rs` (Cargo.toml + `use protocol::` imports),
`framework/core/rs/lib.rs` (`ProtocolBlockKind` definition + surrounding doc comment) — then
**regenerate** the TS mirror in `framework/core/js/index.ts` and
`framework/renderer/react/index.tsx` (these are codegen'd — find and run the generator, don't
hand-edit if a generator target exists; if hand-maintained, edit directly), `framework/renderer/
wgpu/rs/lib.rs` (mirror structs/doc comments), `ui/wgpu/rs/lib.rs` (doc comments),
`framework/plugin/rs/lib.rs` (doc comments only), `framework/product/os/core/rs/lib.rs` (test
fixture plugin ids `"protocol-module-procedural"`).

**Config/codegen/tooling**: root `Cargo.toml` members list (three `protocol/...` entries), root
`package.json` (`"dev:protocol"` script), root `script.ts` (`POLICY_ALWAYS_ALLOWED_DEP_PREFIXES`
entry `"protocol/"` → `"playbook/"` — do **not** add `"protocol/"` back for the new binary family,
apps reach it only through `vcs`, matching how `pack/` is deliberately absent from that list;
`POLICY_PACK_COMPLETENESS_ALLOWLIST` entries referencing old `protocol/...` paths),
`.vscode/launch.json` (three entries, orders ~385.3–385.5: names, `dev:protocol` command,
`PROTOCOL_PLAY_PORT` env, `native protocol` arg), `.claude/launch.json` (`protocol-react-dev` /
`dev:protocol` entry), `framework/plugin/registry/script.ts` (self-test hardcodes `"protocol"`,
`"protocol,protocol-module-procedural"` — update), `framework/plugin/registry/generated/
playgrounds.ts` — this file is **generated**; fix the source `[[package.metadata.semio.playground]]`
in the plugin Cargo.toml and re-run the registry codegen (find the generator command in
`framework/plugin/registry/script.ts`), never hand-edit the generated output,
`.storybook/stories/framework/os/plugins.stories.tsx` (exports `Protocol`/`ProtocolModuleProcedural`
→ `Playbook`/`PlaybookModuleProcedural`, plugin id args). Build artifacts under
`framework/product/os/dev/plugin-modules/protocol*`, `dist/`, and wgpu renderer-modules
(`renderer-modules/wgpu/{plugin-modules,.stage}/protocol*`) — **delete, don't hand-rename**; they
regenerate from the plugin build once the source is renamed. `Cargo.lock` regenerates on next
`cargo build`/`cargo check` — don't hand-edit it.

## Verification before reporting done

1. `cargo build` at the workspace root succeeds (or fails only with pre-existing, unrelated errors
   — check via `git stash`-free comparison: if unsure, note what you saw and move on; per repo
   memory, concurrent-session build churn on unrelated crates is common — poll briefly, don't
   chase it).
2. `bun nx run-many -t test` (or targeted `cargo test -p playbook -p playbook-plugin -p
   playbook-module-procedural -p forms -p forms-plugin -p flow-core -p flow-plugin`) passes.
3. Grep sweep: zero remaining hits of the old crate-scoped identifiers/paths (`ProtocolSpec`,
   `protocol/rs`, `protocol-module-procedural`, `"protocol.program"`, `PROTOCOL_PLAY_PORT`,
   `dev:protocol`) outside of this ticket's own files and the never-rename list above.
4. Storybook build/typecheck for the touched stories file, if a quick check is available.

## Report back

List every file touched (created/moved/edited/deleted), and explicitly flag as a human todo:
rewriting `playbook/AGENTS.md`'s content (frontmatter `technology: protocol` and internal prose
still say "protocol" and reference some already-stale paths — this needs a human pass since
AGENTS.md cannot be edited by an agent).

# Norm Results Window Ownership Migration

## Chosen owner

`selected_check_index` belongs to the exact concrete Results window instance for each of the fifteen Norm editor families. The inspection UI is a framework panel tab rather than a registered window kind. It displays one row from the computed report associated with the Results surface, and framework panel projection retains `focused_window_id` precisely so a panel can read and author the focused window's exact configuration. Therefore the concrete Results window is the narrow registered owner; the Norm application and Inputs window are not owners.

The shared schema is `NormResultsWindowConfig { selected_check_index }`. Each family binds the same schema and mutation vocabulary through a distinct `WindowConfigOwner` whose `WINDOW_KIND_ID` is that family's registered Results kind. Addressing accepts `window_id` for a window surface and `focused_window_id` for the inspection panel, verifies the concrete instance against the full live roster, and rejects Inputs, stale, or absent instances.

## Implemented contract

- The former application-config taxonomy was removed. `NormResultsWindowConfig` and its closed `ChangeSelectedCheckIndex` mutation now live under `✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config` with Rust, TypeScript, JSON Schema, GraphQL, and Proto facets. The text and Pack codecs identify `norm.results-window.config`; Pack decoding checks id, component, and version together.
- All fifteen editors and all fifteen viewers use framework `NoConfig`/`NoConfigMutation`. `NoPresence`/`NoPresenceMutation` stays in place, and each application descriptor publishes empty config and presence facets. The real family document fields and mutations remain unchanged.
- Every editor Results window declares a family-specific `WindowConfigOwner` over the shared schema and registers it with the framework. Ordinary and retained `setSelectedCheckIndex` paths translate into one exact `WindowConfigMutation`; retained preparation captures the request's window-config snapshot and view model.
- The address resolver prefers the concrete surface `window_id`, otherwise uses a panel's `focused_window_id`, and checks that id against the complete live roster and the family-specific Results kind. Missing, stale, and Inputs identities are rejected.
- Inspection rendering reads the captured exact Results-window config. Results computation continues to read the real family document, so the migration does not move or duplicate document state.

## Validation evidence

The registered ticket oracle `abstraction-ownership-validation:norm-results-window-ownership-oracle` passed. It validated the schema with Ajv, parsed and reduced the same vectors with strict TypeScript, projected them independently with `fast-json-patch`, isolated two EN 1996 Results instances, resolved an Inspection panel through focus, exercised undo and redo, rejected invalid config values, and proved stable document bytes. The log is `🗑️generated/norm-results-window-ownership-oracle.log`. The owning source contract was rerun after the final fanout audit and passed all five state facets, all five mutation facets, 15 exact owners, 15 selected-check producers, 15 Inspection consumers, 30 empty app surfaces, codec vectors, hostile vectors, and strict TypeScript; see `🗑️generated/norm-results-window-config-source-audit-2.log`.

The focused Rust law is mounted under the EN 1996 Results window. Its first test covers the same neutral mutation vector plus text, binary operation, state DSL, state Pack, inverse, and two-instance undo/redo laws. Its runtime test uses an explicit 8 MiB stack and is prepared to cover exact WindowConfig-only receipts, direct and focused-panel addressing, two same-kind instance isolation, rendered Inspection output, stable document and app-config bytes, two-pack reload, edit after reload, invalid identities, and terminal cleanup. Native r4 passed the neutral/Ajv/strict-TypeScript stages, then stopped while compiling the EN 1992 sibling because the fifteen artifact editors addressed the exported store-owner macro through their local crate roots. They now invoke `semio_s_artifact_norm_contract::norm_exact_store_ownership!` explicitly through their existing contract dependency. No focused native test ran in `🗑️generated/norm-results-window-ownership-native-4.log`; the corrected neutral route passes in `🗑️generated/norm-results-window-ownership-oracle-fix.log`.

Native r5 compiled the full representative stack. The codec law passed; the runtime produced the correct left/right values but its sibling-isolation probe allocated the right partition itself by calling the capture helper. The corrected law first inspects persisted packs after the left command and requires exactly the left partition, then renders the right Inspection panel through the default index-zero fallback, captures its lazily created default config, and only then edits it. One test passed and one failed with 224 filtered in `🗑️generated/norm-results-window-ownership-native-5.log`.

The permanent commands are registered through the root and ticket `📜️script.ts` files and their Nx projects. Launch entries `311.204` and `311.205` are unique and ordered after `311.203` in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`; `🗑️generated/norm-results-window-route-registration-audit.log` records the check.

## Bounded file ledger

The implementation is bounded to the shared Norm contract/config/app-surface, the fifteen structurally identical editor routers and selected-check handlers, each editor's Results window owner binding, the corresponding empty viewer application facets, focused neutral/native validation assets, registered Bun/Nx/launch routes, and this ticket's reports/logs. It changes no Norm document schema or parent document mutation.

- Shared state, mutation, five schema facets, codecs, exact owner macro, address resolver, neutral fixture, and neutral oracle: `✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config/**`.
- Shared application contract, retained preparation, ordinary dispatch, and inspection projection helpers: `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs`, `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🦀️.rs`, and `✏️s/🔌️plugins/📕️norm/🦀️.rs`.
- Fifteen artifact roots, editor roots, viewer roots, Results owner modules, Inspection consumers, and selected-check command modules under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any`.
- Representative native law: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🧪️tests/🔬️window-ownership/🦀️.rs`.
- Permanent execution routes: root `📜️script.ts`/`📋️project.json`, ticket validation `📜️script.ts`/`project.json`, plus both canonical launch JSONC sources.

The canonical catalog regeneration is coordinated by the root task so the completed Norm and shared viewport schema changes are captured together.

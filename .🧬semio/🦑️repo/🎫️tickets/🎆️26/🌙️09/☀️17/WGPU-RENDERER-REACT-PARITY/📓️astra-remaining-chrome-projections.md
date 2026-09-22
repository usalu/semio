# Remaining Chrome Projection Audit

This is a source audit, not a browser verdict. It records additional parity gaps discovered while the fresh WASM build runs.

`ShellState::build_settings_conflicts_ui` creates Accept and Discard as expandable child rows. React's `buildConflictsTree` places both buttons inline in the conflict row's control area and optionally includes `ConflictDiffPreview`. The WGPU projection currently retains only conflict id, kind, code and message. Terra is auditing whether existing row-action/control contracts can represent the required layout and whether the current conflict data actually supplies a preview.

`ShellState::build_marketplace_ui` explicitly lists plugins without the extension ledger or React's URL/file installation section. These are remaining feature-census items, not accepted parity. Their interaction and data contracts must be inspected before implementation.

DiffView and EventFeed have renderer-level implementations/tests but no registered app-visible scene producer. The scale benchmark component has no app/examples and is not a suitable browser fixture. The detailed registered product inventory and proposed fixture boundary are in `📓️terra-browser21-surface15-practical-fixtures.md`.

The live React Puzzle page confirms that this is a visible gap, not unused source: its Marketplace contains disabled extension entries grouped under missing hosts, a `dev+extensions` source section, inline Reload/Uninstall controls, and URL/file installation rows. Capture `🗑️generated/astra-runtime/checkpoint21-iab/04-marketplace-react.*` records the DOM, screenshot and console. `react-marketplace-geometry.json` records 24-pixel row headers and 22.39-pixel inline buttons at the current scale. No extension installation or uninstall was performed.

React's `ShellHost` owns `extensionLedger` and posts URL/file bodies to the extension-store install route. Successful loads add the extension to loaded plugins and publish an `installExtension` action to the current app. Enable/disable updates contribution filtering and publishes `setExtensionEnabled`; uninstall disposes the module and publishes `uninstallExtension`. The WGPU shell already invokes extensions and its native runtime has descriptor-driven activation, but this does not supply the absent Marketplace ledger or install controls. Implementing visible controls without the corresponding host operations would not close this gap.

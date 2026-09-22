# WGPU Marketplace Extension Bridge Audit

Read-only source audit, 2026-09-21. No build, browser run, or production change was made.

## Confirmed current boundary

React's Marketplace contract is explicit in [ChromePanels/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx:1266): its host API has plugin install/reload/uninstall plus extension URL install, file install, enable/disable, and uninstall. `buildMarketplaceTree` renders the two install controls, groups plugins by their actual `sourceId`, nests extensions under `extendsHost`, and renders real Enable/Disable and Uninstall buttons ([lines 1288–1419](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx:1288)).

The WGPU projection deliberately omits that half of the contract. [`ShellState::build_marketplace_ui`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8319) unions only resident `self.plugins` with the generated static activation catalogue, assigns every row to the fictional `local` source, and states that extensions and URL/file install are absent because this target has no extension ledger. Its action dispatcher handles only `installPlugin`, `reloadPlugin`, and `uninstallPlugin` ([lines 10396–10415](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10396)); `uninstall_plugin` merely removes a resident entry from `self.plugins` ([lines 9084–9093](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9084)).

The existing native test proves only this reduced plugin lane: it checks static activation-catalogue Install and resident Reload/Uninstall in [wgpu-display-conflicts-marketplace/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs:270). It cannot demonstrate extension installation, source grouping, enablement, or persistent uninstall.

## Existing reusable ownership seams

`ExtensionStore` is the closest existing package authority. It verifies the envelope, materializes the package, preserves `extensionId`, `directoryName`, `version`, `label`, `extends`, `moduleUrl`, and `packageHash`, and only then writes the owned install record ([installation/🟦️.ts:302](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts:302)). Its `installFromUrl`, `installFromBytes`, `uninstall`, and `listInstalled` operations are already one coherent store API ([lines 330–351](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts:330)). The installation-identity test module already validates its package ownership and is the right third-party/package oracle to extend.

The browser worker has an existing page-owned host-I/O RPC: the worker forwards host I/O through the page ([frame-worker/🟦️.ts:885](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:885)); Rust invokes it through `host_io_call` ([Shell/🦀️.rs:28426](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:28426)). That is the correct Worker/page boundary for Marketplace installation. It keeps the file picker and package transfer on the page, where the `ExtensionStore` is available, and returns a small typed admission result to WGPU.

WGPU's current JavaScript install door is intentionally unsuitable for a new extension. `semioWgpuInstallPlugin(pluginId)` resolves only a static `PLUGIN_CATALOG` module URL ([frame-worker/🟦️.ts:928](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:928)), and Rust passes only that string into `ProgramBridgeEntry::from_js`. It cannot authenticate or load an installed record whose ID was not compiled into the catalogue. The native static `ExtensionIndex` has the same limitation; it is an activation descriptor, not a package ledger.

## Required bounded, canonical path

1. Add a target-neutral extension-ledger record to the owned Shell/application state, carrying the Store's public identity fields plus `enabled` and current load status. It must be driven by the real `installExtension`, `uninstallExtension`, and `setExtensionEnabled` operations, not by a renderer-local roster. The record is what `build_marketplace_ui` consumes for source/host grouping and publication.
2. Define four typed page host-I/O operations: list, install-from-URL, install-from-file, and uninstall. URL/file install must run `ExtensionStore` on the page and return an entire installed record or a typed refusal. File install must not reuse the generic file-open response: that path serializes file contents through Rust JSON, whereas an extension package belongs directly in the store's bounded package admission path.
3. Extend the dev store middleware with a real list endpoint and `DELETE` operation. Today it serves an SSE snapshot and accepts only `POST /install` ([installation/🟦️.ts:401](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts:401)); React's DELETE request consequently has no implemented store endpoint. The state transition must wait for the Store's terminal response, rather than removing a row optimistically.
4. Add a dedicated `installExtension(record)` worker/Rust bridge entry rather than weakening the static plugin door. The page may load only the Store-returned module URL; Rust must admit the resulting `ProgramBridgeEntry` only when its manifest identity, version, and host extension target agree with the Store record. Commit the enabled ledger state after that admission. A failure preserves the installed package as an unavailable/failed ledger record, rather than claiming it loaded.
5. Disable must withdraw that extension's contributions from the active application publication while retaining its package and ledger record. Uninstall must first retire the loaded contribution/instance, then ask the Store to delete the record and finally remove the ledger entry after terminal confirmation. Cancellation or refusal at either external boundary leaves the previous ledger and contribution state intact.

This needs an equivalent native `ExtensionStore`/installer capability before the Marketplace contract can be called cross-platform. No existing Rust service exposes URL/file materialization or a mutable extension package list. The static native descriptor registry must remain separate from this durable package path.

## Presentation and action scope

Marketplace extension buttons should use the new canonical inline Toolbar child proposed for Conflicts: a bounded direct Button list attached to the extension row, not nested TreeItem verbs. That preserves each button's real action, disabled state, hit target, accessibility name, accepted-frame retirement, and selection ordering. The host and source metadata must be carried by the published tree records; rendering all rows under `local` would still lose the React grouping contract.

## Fail-first validation

Extend the neutral Marketplace fixture and [wgpu-display-conflicts-marketplace/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs:270) with these behavioural laws:

* A Store-returned extension record publishes under its exact `extendsHost`, source, label, version, enabled status, and real Enable/Disable and Uninstall controls after a sealed-and-acknowledged frame.
* A URL or file refusal/cancellation publishes no new ledger record, no contribution, and no bridge actor. A successful file path returns metadata only across host I/O; package bytes never become a general Shell JSON payload.
* Disable removes only that extension's published contributions; re-enable returns the same admitted record. A rejected enable leaves the preceding projection unchanged.
* Uninstall retains the row and contribution until instance retirement and Store deletion both terminally acknowledge; a Store refusal leaves both intact. Then verify that `listInstalled` and the next Marketplace snapshot omit it.
* Run the first-party package identity test against an independently built package fixture to establish the Store/manifest identity oracle. Run a browser Worker/page host-I/O oracle for URL and file responses, because the production renderer crosses that boundary.

These are proposed laws, not executed results.

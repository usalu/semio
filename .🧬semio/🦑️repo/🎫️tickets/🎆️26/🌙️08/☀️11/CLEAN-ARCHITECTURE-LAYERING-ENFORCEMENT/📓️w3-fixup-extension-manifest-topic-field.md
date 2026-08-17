# Wave 3 fix-up — added `topic_contributions` to `ExtensionManifest`/`ExtensionBundle`

The wave-3 verify agent found a convergent, correctly-deferred blocker: 6+
agents (flow, cad, process, sourcing, playbook, and by extension the batch
plugins with contribution producers) independently discovered that Step B
(open contribution conversion) only works for `PluginManifest` — extensions
build their manifests via a DIFFERENT type, `ExtensionManifest`/
`ExtensionBundle` (`🔌️plugin/🦀️component.rs`), which never got the
`topic_contributions` field/builder in Wave 2's open-contribution work. Every
blocked agent correctly declined to edit a file outside its ownership rather
than working around it — good discipline, but it left Step B undone almost
everywhere.

## Fix
- `🔌️plugin/🦀️component.rs` (guest SDK, inside `pub mod plugin_runtime`):
  added `pub topic_contributions: Vec<TopicContribution>` to
  `ExtensionManifest` (`#[serde(default, skip_serializing_if =
  "Vec::is_empty")]`), initialized in `ExtensionBundle::new`, and added a
  matching builder `pub fn contributes_topic(mut self, topic: impl
  Into<String>, payload: Value) -> Self` alongside the existing
  `.contributes()`. Fixed the two other `ExtensionManifest {}` literal sites
  in the same crate (`extension_manifest()`'s empty-default fallback).
  Import fix: `TopicContribution` needed importing into `plugin_runtime`'s
  OWN `use semio_framework::{...}` block (it's a separate top-level `mod`
  from where `Contribution` was already imported at the file's outer scope —
  first attempt added the import to the wrong module, caught by the
  subsequent `cargo check`).
- `🔌️plugin/🖥️host/🦀️component.rs`: added the same field to the host-side
  `ExtensionManifest` mirror (decode-only, `#[serde(default)]`) for
  completeness — not build-blocking, but keeps the wire-decode path from
  silently dropping topic contributions once producers start sending them.

## Verification
`cargo check -p semio-framework-plugin -p semio-framework-plugin-host` —
both `Finished` clean, zero errors, zero new warnings.

## Unblocks
cad (4 sites), flow (9 sites), process, sourcing, playbook (1 site),
imperative — all can now call `.contributes_topic(topic, payload)` on their
`ExtensionBundle` builders. Actual conversion at each site is a small,
mechanical follow-up (not done here — scoping a focused fix-up wave next).

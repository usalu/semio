# Wave 2 fix-up (done by orchestrator directly, not a spawned agent)

The wave-2 verify agent found one live regression: the `open-contribution` agent
added a required field `topic_contributions: Vec<TopicContribution>` to
`PluginManifest` (manifest `🦀️component.rs`) with no `Default` impl, breaking
every crate that constructs `PluginManifest` via an exhaustive struct literal.
This was correctly flagged as out-of-ownership by every agent that hit it
(the field-adding agent, the extension-world agent whose wasm32 build got
collaterally blocked) rather than silently worked around — good discipline,
but it left a real compile break blocking Wave 3.

## Fixed: added `topic_contributions: vec![]` (or `Vec::new()`) at every real,
compiled struct-literal site

- `🔌️plugin/🦀️component.rs:5884` (`Plugin::new`) and `:6120` (`plugin_manifest`
  fallback) — the guest SDK's own two construction sites.
- `🔌️plugin/🖥️host/🦀️component.rs:816` (`read_manifest`'s bootstrap manifest).
- `💻️os/🖥️host/🦀️component.rs` — 7 sites (974 multi-line "draw" test fixture,
  1121/1128/1187/1194 single-line "draw" fixtures, 1219/1235 multi-line
  "playbook-module-procedural" hot-swap test fixtures).
- `📺️renderer/…/Shell/🧊️component.rs:5466` (a `resolve_commands_tags_every_source`
  test fixture) — confirmed this file compiles into
  `semio-framework-os-renderer-wgpu` (the wgpu renderer target), which the
  open-contribution agent's own report predicted would be transitively
  blocked.

## Deliberately NOT touched

`🧰️framework/🛍️products/💻️os/🦀️component.rs` (the "os core"/products-root file,
NOT `🖥️host/🦀️component.rs`) also has matching `PluginManifest {}` literals —
confirmed via grep across every `#[path = ...]` in the tree that **no crate
currently mounts this file**. It is very likely a live artifact of the same
concurrent cross-session refactor flagged in the wave-1 fix-up (unrelated
"document" field churn). Left untouched per the standing instruction to leave
other sessions' in-progress work alone.

## Verification

- `cargo check -p semio-framework-plugin` — clean (`Finished`).
- `cargo check -p semio-framework-plugin-host` — clean (`Finished`).
- `cargo check -p semio-framework-os` — clean (`Finished`).
- `cargo check -p semio-framework-os-renderer-wgpu` — blocked ONLY by
  `semio-s-plugin-puzzle`'s unrelated missing `📄️document/🦀️component.rs`
  (same concurrent-churn pattern seen repeatedly this ticket) — confirmed by
  reading the full error output, no `topic_contributions`/`E0063` errors
  remain anywhere.

Wave 3 (plugin fan-out) may proceed: `semio-framework-plugin` and
`semio-framework-plugin-host`, the crates it depends on, both compile clean.

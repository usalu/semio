# W3 — Process plugin (`✏️s/🔌️plugins/🏭️process/` + its 4 extensions)

Scope: `🏭️process` plugin root + `🧩️extensions/{🔩️metal,🪵️wood,🤖️robotic,🧱️concrete}`.

## Inventory

- Apps: exactly one — `s.process.3d` (`🎛️apps/🧊️3d`), maps to
  `semio_s_plugin_process::apps::process3d::config::schema::register_app_schema()` (confirmed the exact
  expected fn path from the parked `catalog-integration`-gated call site in framework schema's
  `🦀️component.rs:1483`).
- Artifact: `s.process.process3d` — already self-registers via
  `crate::artifacts::process3d::engine::register_artifact_schema()`, called from `engine::register()`
  (the plugin's `.setup(...)` hook in `🦀️component.rs:10`). Untouched, used as the reference pattern.
- Extensions: metal/wood/robotic/concrete are pure `ExtensionBundle` catalog contributors — no apps of
  their own, nothing for Step A.

## Step A — Schema self-registration (done)

1. Added `register_app_schema()` to
   `🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs` (new `//#region 🔖️AppSchemaRegistration` block),
   transplanting the exact `AppSchemaDescriptor`/`FacetLeaves` construction from framework schema's
   closed catalog (`id: "s.process.3d"`, config + presence facets), with `include_str!` paths made
   relative to this file's own location:
   - config leaves: `./🦀️component.rs` (self), `./🟦️component.ts`, `./🔗️component.graphql`,
     `./🔣️component.json`, `./🛰️component.proto`.
   - presence leaves: `../../👥️presence/🧬️schema/🦀️component.rs` (+ ts/graphql/json/proto siblings).
   Calls `schema::register_app_schema_descriptor(schema::AppSchemaDescriptor { .. })` — `schema` is the
   crate-wide alias `extern crate semio_framework_schema as schema;` declared in this plugin's own
   `📦️glue.rs`, same alias the artifact engine file already uses for
   `schema::register_artifact_schema_descriptor`.
2. Wired the call into the plugin's one shared setup hook, alongside the existing
   `register_artifact_schema()` call, in
   `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`'s `pub fn register()`:
   ```rust
   register_artifact_schema();
   crate::apps::process3d::config::schema::register_app_schema();
   register_pilot_languages();
   ```
   This is the plugin's only setup hook (`Plugin::builder("process").setup(crate::artifacts::process3d::
   engine::register)` in the plugin-root `🦀️component.rs`), so both self-registrations now run at plugin
   init time — this plugin did **not** already call `register_artifact_schema()` from anywhere outside
   the parked `catalog-integration` feature gate before I found it also wired into `engine::register()`
   directly (that's the "established pattern" the shared recipe pointed at).

Files touched:
- `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

## Step B — Open contribution producer conversion (BLOCKED, not applied)

Found every `Contribution::ProcessMachines` construction site (producers) in my subtree via
`grep -rn "Contribution::ProcessMachines"`:

1. `🎛️apps/🧊️3d/🦀️component.rs:470` and `:480` — inside `seed_domain_catalog_contributions()`, builds
   `ProgramContributionEntry { plugin_id, contribution: Contribution::ProcessMachines { .. } }` values,
   serializes them to JSON, and feeds them into
   `crate::artifacts::process3d::engine::sync_process_machine_contributions(&json)`.
2. `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:802` — same
   `ProgramContributionEntry` shape, but inside `#[cfg(test)] fn
   sync_process_machine_contributions_merges_hot_installed_catalogs()` — a test fixture, not a real
   producer.
3. `🧩️extensions/🔩️metal/🦀️component.rs:162`, `🧩️extensions/🪵️wood/🦀️component.rs:176`,
   `🧩️extensions/🤖️robotic/🦀️component.rs:152`, `🧩️extensions/🧱️concrete/🦀️component.rs:152` — each
   builds `ExtensionBundle::new(..).contributes(Contribution::ProcessMachines { .. })`.

**Blocker**: the task instructs pushing a sibling `TopicContribution::new("process.machines", payload)`
into "the SAME manifest's `topic_contributions` vec" alongside the existing `Contribution` push. I
checked every manifest type these producer sites actually construct, in the framework files that own
them (I did **not** edit any of these — out of my ownership, framework tree):

- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — `TopicContribution` and the
  `topic_contributions: Vec<TopicContribution>` field exist **only** on `PluginManifest` (confirmed at
  its definition, ~line 2841-2844, and at its two construction sites in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:5884` and `:6120-6130`).
- `ProgramContributionEntry` (`🛂️manifest/🦀️component.rs:2788`) — `{ plugin_id: String, contribution:
  Contribution }` only. **No topic-sibling field.** This is what producer sites #1 and #2 above
  construct.
- `ExtensionManifest` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6914-6923`, backing
  `ExtensionBundle`) — `{ extension_id, label, version, extends, capabilities, contributions:
  Vec<Contribution> }` only. **No `topic_contributions` field**, and `ExtensionBundle` exposes no
  `.contributes_topic(...)`-style builder method either (only `.contributes(Contribution)` at line 6956).
  This is what producer sites #3 (all four extensions) construct.

Every one of process's real `Contribution::ProcessMachines` producers routes through one of these two
manifest-adjacent types, and neither carries the open `topic_contributions` counterpart the prior
`w2-open-contribution` wave added — that wave only extended `PluginManifest` (confirmed by reading
`📓️w2-open-contribution.md`, whose own out-of-ownership survey already flagged the `PluginManifest`
struct-literal fallout in `🔌️plugin`/`🖥️host`, but did not mention `ExtensionManifest` or
`ProgramContributionEntry` at all — they were out of that wave's scope too).

Adding `topic_contributions` to `ExtensionManifest` and/or `ProgramContributionEntry` requires editing
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` and/or
`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — both outside my assigned subtree (framework tree,
explicitly off-limits per my instructions). I did **not** touch either file, and did **not** apply any
Step B conversion for process. Flagging this as the exact blocker for whichever wave owns extending
`ExtensionManifest`/`ProgramContributionEntry` with the open counterpart field next — process's Step B
is fully dependent on that landing first.

No files touched for Step B.

## Verification

`cargo check -p semio-s-plugin-process`:

```
error: couldn't read `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/./././../../🎛️apps/🧊️3d/🎮️commands/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> ✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs:470:13
    |
470 |             pub mod document;
```

This is **not caused by my change** — I did not touch `📦️glue.rs` or anything under `🎮️commands/`. The
directory `🎛️apps/🧊️3d/🎮️commands/` currently has no `📄️document` child at all; it has `📄️artifact`
instead (confirmed via `ls`), which matches exactly the briefed concurrent "document" concept refactor
in progress in another session (threading a "document" rename through several
plugins/AppDefinition/OsAppRegistration). This error is absent from
`📸️baseline-cargo-check.txt` (baseline predates that refactor), confirming it's newly introduced by the
other session's in-flight work, not a regression from my edits. Per the master ticket's instruction, not
fixing it — reporting it as the exact blocker preventing a clean `cargo check` right now.

Both of my Step A edits are small, additive, and independently reviewed against the exact working
pattern already compiling elsewhere in this same crate (`register_artifact_schema()`'s call/definition
shape) — I have high confidence they are correct, but could not get a green `cargo check` past the
unrelated `🎮️commands/📄️document` compile error to prove it end-to-end. Re-run `cargo check -p
semio-s-plugin-process` once the concurrent "document" rename in this plugin's `commands` module settles.

## Files touched (final)

- `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs` (added `register_app_schema()`)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (added the
  `register_app_schema()` call inside `register()`)

No other files touched. No framework files touched. No extension files touched (Step B blocked, see
above).

# 🎪️ Demonstrator — registrar handoff

## Root `Cargo.toml` member swap

Remove:

```
    "✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
```

Add:

```
    "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust",
```

`[workspace.dependencies]`: nothing to remove — `semio-s-plugin-demonstrator` was never aliased there,
and a repo-wide grep confirms no crate anywhere depends on demonstrator (it is a leaf).

After the swap: `cargo metadata`, then `bun nx run @semio-tech/plugin-registry:generate` (the catalog is
currently stale, which is why `…/📇️registry/📜️script.ts check` bails before printing taxonomy findings),
then `.vscode/launch.json` regeneration. Playground ports/variants/brands/aliases and the asset rows were
copied byte-for-byte, so the launch regeneration should be a no-op.

## 🚨 BLOCKER for the wasm component build — needs a decision outside this ticket's ownership

`cargo build --target wasm32-wasip2 --profile wasm-release` fails at link time:

```
rust-lld: error: duplicate symbol: semio_plugin_install_bundle
rust-lld: error: duplicate symbol: semio_plugin_bundle_installer_link_shim
```

Everything else is green (native check, wasm32-wasip2 *check*, clippy `-D warnings`, tests). This is a
**structural consequence of the crate consolidation itself, not of this migration**, and demonstrator is
the only crate in the repo that can hit it:

* `semio_framework_plugin::plugin_exports!` (expanded by `semio_plugin!`) emits two
  `#[unsafe(no_mangle)] pub extern "C"` symbols. The framework declares the matching
  `extern "C" { fn semio_plugin_bundle_installer_link_shim(); }` and calls it from
  `ensure_plugin_initialized()` — so the contract is **exactly one `plugin_exports!` expansion per wasm
  component**.
* Pre-consolidation, demonstrator path-depended only on the six source plugins' per-app *module* crates
  (`semio-s-app-cad-ui`, `-engine`, …), none of which expanded `plugin_exports!`; each source plugin's
  expansion lived alone in its `🛂️manifest/🗿️artifact` bundle crate, which demonstrator never depended on.
  (Verified against the first committed version of demonstrator's `Cargo.toml`.)
* Post-consolidation, bundle and modules are ONE crate per plugin, so demonstrator now links six
  `plugin_exports!` expansions plus its own — seven definitions of the same two symbols.

The interim dependency-only repoints the registrar applied per batch already put demonstrator in this
state; it just never surfaced because root-workspace cargo has been red throughout, so nobody linked it.

### Why this was not fixed here

Every possible fix requires editing the six source plugins (forbidden for this ticket: "never write to
another plugin's directory") and/or a shared framework macro used by ~45 plugins. Recommended option,
smallest and backward-compatible for the other 44 plugins:

1. `semio-framework-plugin`'s `plugin_exports!`: wrap only the two `#[unsafe(no_mangle)]` fns in
   `#[cfg(not(feature = "semio-plugin-embedded"))]`. A `#[cfg(feature = …)]` inside a `macro_rules!`
   expansion is evaluated against the **expanding** crate's features, so this is inert for every plugin
   that does not declare the feature.
2. Each of the six source plugins (`🌀️procedural`, `📐️cad`, `🧩️puzzle`, `🪵️sourcing`, `🏭️process`,
   `🌍️gis`) declares `[features] semio-plugin-embedded = []` (non-default, so their own standalone
   components are unaffected).
3. Demonstrator's six dep lines gain `features = ["semio-plugin-embedded"]`.

Caveat to check when implementing: cargo's `unexpected_cfgs` lint only knows the features a crate
declares, so step 1's `cfg` must not fire that lint in the other ~44 plugin crates — if it does, the
alternative is to have every plugin declare the feature (a mechanical repo-wide sweep) or to gate via a
`build.rs`-emitted `cargo::rustc-check-cfg` in the framework crate.

Do NOT "fix" this with `--allow-multiple-definition`: which definition wins would then be link-order
dependent, and picking a source plugin's installer would silently install the wrong bundle.

## Stale path references left for their owners (not edited — outside this ticket's ownership)

Doc-comment-only references to the now-deleted
`✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust` path. None affect the
build; each should be repointed at `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust` by that file's owner:

* `✏️s/🔌️plugins/{🌀️procedural,📐️cad,🧩️puzzle,🪵️sourcing,🏭️process,🌍️gis}/📦️packages/🦀️rust/Cargo.toml`
  — the "the `<variant>` demonstrator row moved to …" comment in each.
* `♻️mit-bestand/🧺️demonstrator/📜️script.ts:14`.

## Environment notes for whoever runs the registrar pass

* Root workspace cargo is currently red for an unrelated reason: `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml`
  has a leftover `TEMPORARY VERIFICATION OVERLAY` `[workspace]` table while being a registered root member
  (`cargo metadata` → "multiple workspace roots found"). `🏗️fem` has one too but is genuinely in-flight.
  Not touched here (other plugins' directories).
* Because of that, every verification in this ticket ran through the isolated-overlay route with
  `--manifest-path`. Two extra flags were needed beyond TEMPLATE §3, both new findings:
  * `cargo -Z trim-paths …` — an isolated overlay workspace whose path deps belong to the ROOT workspace
    (which the six consolidated plugin crates do, via `version.workspace = true`) makes cargo parse root's
    `[profile.*] trim-paths` without honouring root's own `cargo-features` line, so the nightly `-Z` flag
    must be passed explicitly. Adding the six plugin crates as members of the overlay workspace does NOT
    work (`package … is a member of the wrong workspace`).
  * `DEVELOPER_DIR=/Library/Developer/CommandLineTools` — needed even for plain `cargo check` here,
    because this crate's dependency graph has proc-macro build scripts that link.
* `cargo build --target wasm32-wasip2` under the plain `dev` profile dies with the known
  `rust-lld … SIGSEGV` (this time in `semio-framework-os-kernel-math-graph-dsl`) — same class as root
  `Cargo.toml`'s existing `[profile.dev.package.semio-framework-os-kernel-store] codegen-units = 1`
  workaround, unrelated to this migration. Use `--profile wasm-release`, which is what the os-dev plugin
  pipeline passes anyway.

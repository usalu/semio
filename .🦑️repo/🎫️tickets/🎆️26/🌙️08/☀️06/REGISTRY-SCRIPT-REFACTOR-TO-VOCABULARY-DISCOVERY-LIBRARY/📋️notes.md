# 📇️ M3 — Registry Script Refactor To Vocabulary/Discovery Library

## What changed

Two files in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/`:

### `📜️script.ts`

| removed | replaced by |
|---|---|
| `LEGACY_LAYOUT_TOLERANT = true` | `PLUGINS_AREA_STATE` — the plugin area's declared `AreaState` from `🔣️taxonomy.json`, via `areaOf()` |
| `isNewContractPluginManifestPath` regex | `discoverPackages().filter(lang === 🦀️rust && role ∈ {plugin, extension})` |
| framework-crate regex (`🧰️framework/.*/📦️packages/🦀️rust/(🎯️targets/…)?`) | `discoverPackages().filter(role === "framework")` — now feeding a real catalog section |
| `isModuleCrate` / `isExtensionCrate` / `isPluginBundleCrate` regexes | `findLegacyComponentManifests()` — structural walk over `taxonomy.forbiddenPathSegments` + `ecosystems.🦀️rust.manifestFilename`, gated on the area state |
| hardcoded plugin-SDK path exclusion | dropped (dead — the SDK crate declares no `[package.metadata.component]`; legacy discovery is now scoped to the plugin area anyway) |
| `hasSemioRole()` TOML sniffer | `readSemioMarker()` inside the shared discovery walk |
| `"🎛️apps"`, `"🗿️artifacts"`, `"🎭️modes"`, `"🪟️windows"`, `"📦️packages"`, `"📦️lib.rs"`, `"⚡️implementations"`, `"✏️s"/"🔌️plugins"`, `"📚️examples"` literals | `TAXONOMY.*` fields (with load-time assertions for the two values the vocabulary has no dedicated key for: `PLUGINS_AREA` ∈ `areas`, `EXAMPLES_DIRNAME` ∈ `rootDataDirNames`) |
| separate `expected` map in `check` + separate write list in `generate` | one `renderCatalogFiles()` used by both |

New: `FrameworkPackageEntry` / `generateFrameworkPackageRegistry()` / `emitFrameworkPackagesTypeScript()`,
emitting `🤖️generated/🔣️framework.json` + `🤖️generated/🟦️framework.ts` (`FRAMEWORK_PACKAGES`).

Severity is now derived, not hand-flipped:
- taxonomy-tree findings **warn** while the plugin area is `legacy`/`mixed`, **fail** once it is `clean`;
- `validateConstitutionalCrates` goes silent once the area is `clean`;
- `findLegacyComponentManifests` returns nothing once the area is `clean`.

The W10 finalization flip is therefore a one-word edit in `🔣️taxonomy.json`, not a code change.

### `🖥️launch.ts`

- CLI `main()` removed; module now exports `generateLaunchJson(repoRoot, playgrounds)` + `LAUNCH_OUTPUT_REL_PATH`.
- Playground catalog is passed **in** instead of imported, so there is no runtime import cycle with `📜️script.ts`.
- `📜️script.ts generate` now writes `.vscode/launch.json` (after the catalog, so a seed problem can never
  block catalog output) and `📜️script.ts check` byte-compares it — launch freshness is enforced by the same
  target the root `verify gate` already calls (`📜️script.ts` line 661).

## Verification

- Before/after catalog diff (`🧪️catalog-diff.ts`, baseline = pre-refactor copy of the script in this folder):
  **0 field mismatches** across all 34 shared plugin rows and all 58 playground rows.
- `bun ./📜️script.ts generate` → 37 plugin crates, 58 playgrounds, 7 framework packages.
- `bun ./📜️script.ts check` → green (only the pre-existing warn-only 🏗️fem `📡️protocol` finding).
- `.vscode/launch.json` regenerated **byte-identical** (`git diff --stat` empty) — the launch refactor is
  output-neutral.
- `tsc --noEmit` on both files: no new errors (3 pre-existing: `import.meta.dir`, 2 in repo-lib).
- repo-lib suite: **121 pass / 9 fail** — exactly M1's recorded baseline; all 20 taxonomy+discovery tests pass.
- No test file exists for `@semio-tech/plugin-registry` (nothing to extend; CLAUDE.md forbids adding one).

## 🐛 Live discovery gaps

### FIXED by this refactor — 🪵️sourcing's 3 extension crates

While this ticket was running, the concurrent `SOURCING-PLUGIN-EXTENSIONS-DE-SANDWICH` agent moved
`✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/{🪵️beams,🧱️slabs,🪟️windows}/⚡️implementations/🦀️rust` →
`…/📦️packages/🦀️rust` with `role = "extension"`. The catalog diff caught the old code dropping
`sourcing-module-beams`, `sourcing-module-slabs` and `sourcing-module-windows` **the moment those dirs
moved** (37 → 34 rows) — a second, independent live instance of the plan's predicted "Step 3 silently
drops plugins from the runtime loader" risk, beyond the `flow-extension-bim` one M1 flagged. The
refactored discovery picks all three back up with identical `pluginId`/`packageName`/`wasmOut`/
`contributes`/`consumes`.

### DEFERRED — `flow-extension-bim` (needs the flow W6 pilot to land first)

`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/Cargo.toml` has
`[package.metadata.semio]` with only `contributes = ["flow.extension"]` — **no `role` key** — so it stays
invisible to `discoverPackages()` and therefore still missing from the registry catalog.

**Not touched**, because ticket `26/08/06/FLOW-PLUGIN-TS-MODULES-AND-EXTENSIONS-CONSOLIDATION` is still
`open` and its prompt gives it exclusive ownership of `✏️s/🔌️plugins/🌊️flow/🧩️extensions/*`.
(`FLOW-PLUGIN-RESIDUAL-MOP-UP-TS-MODULE-EXTENSION-DE-SANDWICH` is closed, but it is not the ticket that
owns this file.)

**One-line fix once that ticket closes** — add to the existing table:

```toml
[package.metadata.semio]
role = "extension"
contributes = ["flow.extension"]
```

then re-run `bun nx run @semio-tech/plugin-registry:generate`. The catalog should grow to 38 rows with
`flow-extension-bim` at `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust`.

**Second problem in the same file** (registrar duty, also not mine to touch): it still carries the
`⚠️ TEMPORARY VERIFICATION OVERLAY` header with an embedded `[workspace]` table — the exact leftover class
master.md documents. It must be removed before root `cargo metadata` can resolve this crate.

## Scratch files in this folder

- `📜️baseline-script.ts` — pre-refactor copy of `📜️script.ts` (import repointed to an absolute path).
- `🧪️catalog-diff.ts` + `📋️catalog-before.json` / `📋️catalog-after.json` / `📋️catalog-diff-output.txt`.
- `🔍️probe.ts` + `📋️probe-output.txt` — `discoverPackages()` grouped by role.
- `🔍️legacy-probe.ts` + `📋️legacy-probe-output.txt` — proof that the generalized legacy matcher selects
  exactly the same 5 sandwich crates the 3 old regexes did (no over-match).

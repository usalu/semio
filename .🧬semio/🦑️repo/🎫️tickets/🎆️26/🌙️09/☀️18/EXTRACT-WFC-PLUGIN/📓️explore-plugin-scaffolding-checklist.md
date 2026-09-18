# New plugin + artifact registration checklist (wfc, extracted from remodel/procedural precedent)

Reference plugin used as the model: `✏️s/🔌️plugins/📸️remodel` (plugin id `remodel`, package `semio:remodel`, single artifact `📸️remodeling`). Cross-checked against `🌀️procedural` (plugin id `procedural`, package `semio:procedural`, three artifacts: `generation2d`, `generation3d`, `assembly` — the last one being the one that owns the WFC engine we're extracting, under `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine`).

Everything below is **outside** `✏️s/🔌️plugins/🌊️wfc/` itself — i.e. repo-wide/framework registration sites a new plugin+5-artifact crate set must touch (or regenerate) for the gates to pass.

---

## 1. Root `Cargo.toml` — hand-edited, no generator

Two places, alphabetically ordered:

- `members = [...]` array (root, starts line 4):
  - artifact crate members, ~line 239 block (alongside remodel's `"✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust"` and procedural's three at lines 246-248): add 5 lines, one per wfc artifact:
    `"✏️s/🔌️plugins/🌊️wfc/🗿️artifacts/🎨️bitmap/📦️packages/🦀️rust"`, `.../◻️2d-grid/...`, `.../◻️2d/...`, `.../🧊️3d-grid/...`, `.../🧊️3d/...` (exact artifact-folder emoji/names are your call — mirror whatever `EXTRACT-WFC-PLUGIN` decides).
  - plugin crate member, ~line 39 (remodel) / line 105 (procedural): add `"✏️s/🔌️plugins/🌊️wfc/📦️packages/🦀️rust"`.
- `[workspace.dependencies]` table:
  - artifact aliases, ~lines 302 (`semio-s-artifact-remodel-remodeling = { path = "..." }`) and 309-311 (procedural's three): add 5 `semio-s-artifact-wfc-<artifact> = { path = "✏️s/🔌️plugins/🌊️wfc/🗿️artifacts/<artifact>/📦️packages/🦀️rust" }` lines, alphabetically ordered among the existing `semio-s-artifact-*` block.
  - plugin alias, ~line 482 (procedural) / 485 (remodel): add `semio-s-plugin-wfc = { path = "✏️s/🔌️plugins/🌊️wfc/📦️packages/🦀️rust" }`.
- Optional: `[profile.wasm-dev.package.semio-s-plugin-wfc]` / `...semio-s-artifact-wfc-<x>` overrides (`opt-level = 2`) — only needed if a wfc guest turn measures over the 8ms interactive-step ceiling in dev profile (see the `lowpoly`/`puzzle-3d`/`procedural` precedent comments at lines ~505-540). Not required up front; add later if `plugin.internal.interactive-ceiling` traps fire.

No other root-level manual edit is required for Cargo — `Cargo.lock` regenerates itself on `cargo check`/`cargo build`.

## 2. Plugin's own `Cargo.toml` boilerplate (for reference — inside the new plugin, but shapes what other gates read)

Read in full: `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml`. Copy verbatim except name/description/deps:

```toml
[package]
name = "semio-s-plugin-wfc"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "<one line>"

[lints]
workspace = true

[package.metadata.component]
package = "semio:wfc"

[package.metadata.semio]
role = "plugin"

[package.metadata.semio.storybook]     # only if wfc will have Storybook stories — see §8
id = "wfc"
titlePrefix = "🌊️wfc"
sourceRoots = ["."]
storyGlobs = ["📖️stories/**/🧪️.story.tsx"]

[[package.metadata.semio.playground]]  # one block per artifact/variant, see §7 for ports
variant = "bitmap"
app = "s.wfc.bitmap@1/*#editor"
ports = { react = 6041, wgpu = 6141 }

[[package.metadata.semio.playground]]
variant = "2d-grid"
app = "s.wfc.2d-grid@1/*#editor"
ports = { react = 6042, wgpu = 6142 }

# ...repeat for 2d, 3d-grid, 3d

[lib]
crate-type = ["cdylib", "rlib"]
path = "🦀️.rs"

[dependencies]
semio-s-artifact-wfc-bitmap = { workspace = true }
# ...+ the other 4 artifact crates, plus whatever engine deps the WFC code needs
semio-framework = { workspace = true }
semio-framework-os = { workspace = true }
semio-framework-plugin = { workspace = true, features = ["component-guest"] }
```

`[package.metadata.semio.storybook]` uses the `block`/`cad`/`animate`/`fem`/`puzzle` opt-in pattern (`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml:21-25`), **not** remodel's hand-curated coordination.ts exception (that was a one-off worker-stream conflict workaround, see §8).

## 3. Plugin's own `📋️project.json` / `📜️script.ts` (nx boilerplate — inside plugin folder, mirror verbatim)

`✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📋️project.json` gives 6 nx targets: `test`, `test-quick`, `test-long`, `test-exhaustive`, `regenerate-example`, `describe`. Copy structurally, s/remodel/wfc/, s/📸️remodel/🌊️wfc/. The `describe` target's `outputs` list the two files it (re)generates at the **plugin root**:
```
{workspaceRoot}/✏️s/🔌️plugins/🌊️wfc/🛂️.descriptor.semio
{workspaceRoot}/✏️s/🔌️plugins/🌊️wfc/🔣️.json
```
`✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📜️script.ts` (37 lines) registers `TestScript`/`DescribeScript`/`RegenerateExampleScript` via `ScriptRouter`; `DescribeScript.run()` calls `describePluginComponent(this.repoRoot, "semio-s-plugin-wfc", join(this.root, "..", ".."))` — this is the **descriptor generator** memory flagged: running `bun nx run <wfc-project>:describe` (or `nx run-many -t describe`) writes `🛂️.descriptor.semio` + `🔣️.json` at the plugin root; nothing outside the plugin needs manual editing for descriptors, but the command must actually be run (and its outputs committed) or `@semio-tech/plugin-registry:check` (§5) will flag missing/stale descriptors.

TS package: `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/{package.json,📋️project.json,🟦️.ts}` — trivial re-export barrel (`export * as <artifact>_schema from "../../🗿️artifacts/<artifact>/.../🟦️.ts"` per artifact) + nx `test` target. Mirror with `@semio-tech/wfc-js`.

## 4. Root `package.json` and root `📋️project.json` / `📜️script.ts` — mostly optional

- `package.json`:
  - `workspaces` array, line 36: add `"✏️s/🔌️plugins/🌊️wfc/📦️packages/🟦️typescript"`.
  - `"scripts"` convenience alias, line 171: `"dev:remodel": "bun nx run workspace:dev -- remodel"` — optional per-plugin shortcut; add `"dev:wfc": "bun nx run workspace:dev -- wfc"` if you want one.
- Root `📋️project.json` (lines 466-478) and root `📜️script.ts` (lines 7283-7291, 9818, 15886, 16284, 19320, 22948) — remodel's hits here are **feature-specific probes** (a `remodel-window-ownership-oracle`/`-native` nx target, an interactivity-tool-run registration row for `reconstruction` tooling, an `OsMediaExportResult` allowlist entry, standards-id literals in policy code, a doc-comment example). None of these are generic per-plugin boilerplate — they exist because remodel has bespoke window-ownership rules, a tool-run command, and media export. wfc likely needs **none** of these unless it grows equivalent bespoke behavior (e.g. its own tool-run verb). Treat root `📜️script.ts`/`📋️project.json` as "only touch if wfc needs a bespoke verify/oracle target", not mandatory registration.

## 5. Plugin registry — one hand-maintained file, rest auto-generated

- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`** (64 lines, hand-maintained, alphabetically sorted `{ "pluginId": ..., "directoryName": ... }` array, lines 3-62). **This is the "os dev plugin catalog" that discovers plugins.** Add:
  ```json
  { "pluginId": "wfc", "directoryName": "🌊️wfc" },
  ```
  in alpha order (between `vcs` and `writer`, i.e. after line 60/before line 62 as currently ordered — re-check position once wfc's directory emoji is final since sort is by `pluginId` ascii, not directoryName).
- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/{🔌️plugins.json,🎠️playgrounds.json,...}`** — fully **auto-generated**, confirmed by `📋️project.json` in that module (`generate` target: `bun ./📜️script.ts generate`, `check` target: `bun ./📜️script.ts check`, invoked repo-wide as `bun nx run @semio-tech/plugin-registry:generate` / `:check`). Do **not** hand-edit; after adding wfc's `Cargo.toml` (§2) and `🗺️catalog.json` entry, run:
  ```
  bun nx run @semio-tech/plugin-registry:generate
  bun nx run @semio-tech/plugin-registry:check
  ```
  `:check` is also what the top-level `bun ./📜️script.ts verify` gate runs (script.ts line ~8483, inside `runGate()`), so this is a real CI/gate blocker, not optional hygiene.
- `📦️deployment/🛣️routes.json` is generic (`{"plugin": "/🔌️plugin-modules", "extension": "/🧩️extension-modules"}`) — no per-plugin edit.

## 6. Taxonomy (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, 30,526 lines) — hand-maintained SSOT, no generator writes these arrays

This is the file the memory note "Taxonomy Filename Drift Blinds Discovery" warns about: entries here are **not** derived from the filesystem walk automatically — `bun ./📜️script.ts verify taxonomy report|enforce` (script.ts lines 8039-8046) walks the real tree and checks it against this file's vocabulary; a name here that doesn't exactly match the real folder/emoji spelling makes discovery silently return zero while the walk itself still reports "clean" (nothing to find ≠ nothing wrong). `bun ./📜️script.ts generate taxonomy census|duplicates` only emits ticket-scoped audit reports, it does **not** write this file.

Confirmed remodel/procedural hit sites (grep `-n -i "remodel\|procedural"` against this file) — mirror each for `wfc` / `🌊️wfc` / each of the 5 artifact names:

| Line(s) | Key / list | What remodel/procedural has | wfc needs |
|---|---|---|---|
| ~11577 | `members-of-plugins` → `memberNames` | `"📸️remodeling"` (NOT `"📸️remodel"` — the plugin's single artifact name, not the plugin folder!), `"🌀️procedural"` (the plugin folder name, since procedural's registry component id equals the plugin id) | Add the wfc plugin's registry-facing id — **verify which convention applies before copying**; this is exactly the drift trap. Cross-check against `🤖️generated/🔌️plugins.json`'s `pluginId`/`packageId` for the actual registered id once §5 is regenerated. |
| ~8056-8066, ~13486, 13611, 13840 | assorted fixture/mutate id lists (`⚙️playbook-module-procedural`, `🌀️procedural`, `📸️remodel`, `📸️remodeling`, `📸️mutate-remodeling-1`, `🌀️mutate-procedural-2d-1`, `🧊️mutate-procedural-3d-1`) | per-mutation test-fixture ids | New rows once wfc has mutations with fixtures |
| 19841-19870 | `dev-plugin-component-remodel-{js,declaration,wasm}` fileKinds, `pathPattern` = `🧰️framework/.../🔌️plugin-modules/📸️remodel/semio_s_plugin_remodel_component.{js,d.ts,core.wasm}` | | 3 new fileKind entries `dev-plugin-component-wfc-{js,declaration,wasm}` with `pathPattern` = `.../🔌️plugin-modules/🌊️wfc/semio_s_plugin_wfc_component.*` |
| 20336-20365 | same for procedural | | (already covered above — pattern is identical for any plugin) |
| 24838-24845 | `dev-plugin-interfaces-remodel` fileKind, `pathPattern` = `.../🔌️plugin-modules/📸️remodel/interfaces` | | `dev-plugin-interfaces-wfc` fileKind |
| 17279, 17362, 17442, 17516 (four near-duplicate `dev-jco-plugin-interfaces` array blocks) | `"dev-plugin-interfaces-remodel"` string appended to each block | | Append `"dev-plugin-interfaces-wfc"` to **all four** blocks (grep `dev-jco-plugin-interfaces` to enumerate exact occurrences — do not assume 4 is final, re-grep at edit time) |
| 26829-26840 | `"dev-plugin-component-remodel-js"`/`-declaration"` → `"authority": "JCO remodel companion without purity exemption"` / `"...declaration pairing"` | | mirrored `wfc` authority entries |
| 23862-23863 | ticket-path literals for a remodel-specific ticket | | irrelevant to wfc (leave alone) |

**Practical method for the actual edit**: run `grep -n -i "remodel" 🔣️taxonomy.json` and `grep -n -i "procedural" 🔣️taxonomy.json` fresh at edit time (line numbers above are as of 2026-09-18, commit `7bea15c349`, and this file is being actively edited by concurrent work), and for every hit decide by inspection whether it's (a) a generic per-plugin fileKind/interfaces mirror (always add), (b) a remodel-specific bespoke rule (skip), or (c) a `procedural`-multi-artifact pattern (mirror once per wfc artifact, not once per plugin). Then run `bun ./📜️script.ts verify taxonomy enforce` — it will additionally catch anything still missing structurally (required child dirs etc.) that this table doesn't enumerate.

## 7. Schema catalog — fully auto-generated, do not hand-edit

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/{🔣️schema-catalog.json,📓️schema-catalog.md}` both start with `"generator": "bun ./📜️script.ts schema generate"` / `Generated by ... — never hand-edited`. After wfc's schema files exist, run:
```
bun ./📜️script.ts schema generate
```
which will add `app.wfc.*` scope rows mirroring `app.procedural.2d.*` / `app.remodel.*` shapes it wrote for the reference plugins (e.g. lines 1104-1477 for procedural's generation2d/3d config/presence/transient/mutation scopes).

## 8. Storybook

- Preferred path (per §2): `[package.metadata.semio.storybook]` in the plugin's own `Cargo.toml`, auto-discovered by `GENERATED_SCOPES` (`buildGeneratedScopes`, `.storybook/📖️stories/🧭️coordination/🟦️.ts:242`) — **no edit to coordination.ts needed** if you use this.
- Fallback / what remodel actually did (hand-curated block, lines ~103-111 of coordination.ts) — only because remodel's `Cargo.toml` was concurrently owned by another worker stream (ticket 26/09/06/REMODEL-PLUGIN-END-TO-END) at the time; explicitly called out in the code comment as "move it to the manifest opt-in once free". Don't imitate this path unless you have the same conflict.
- `.storybook/🎨️styling/🎨️.css` — hand-maintained list of Tailwind `@source "../../✏️s/🔌️plugins/<plugin>"` scan-root directives, currently 6 entries (`animate`, `block`, `cad`, `fem`, `puzzle`, `remodel`). **Only needed if wfc's stories use Tailwind utility classes directly** (most plugins apparently don't need it — only 6 of ~33 are listed). Add `@source "../../✏️s/🔌️plugins/🌊️wfc";` at the end if so.

## 9. Dependency truth (`🔒️dependencies.json`) — auto-regenerated baseline, not hand-edited

`🔒️dependencies.json` (114KB) tracks every third-party (crates.io/npm) dependency's `"users"` (consuming `Cargo.toml`/`package.json` paths) plus `productionReachable`/`oracleIds`. It already lists remodel's `Cargo.toml` as a user of shared deps like `png` (lines ~1995-2015). It is written by:
```
bun ./📜️script.ts verify dependencies write-baseline
```
(script.ts line 257 dispatch; plain `bun ./📜️script.ts verify dependencies` only checks the existing baseline and fails loudly if new third-party deps appear without an approved baseline bump — line 272: `"...approve deliberately with 'bun ./📜️script.ts verify dependencies write-baseline', or remove the dependency."`). The **authoritative dep audit** per project memory is the source-only, red-until-zero mode:
```
bun ./📜️script.ts verify dependencies literal-external
```
Run order for wfc: add real deps to wfc's `Cargo.toml`/`package.json` (§2), then `write-baseline` to refresh `🔒️dependencies.json`, then `literal-external` to confirm no new undeclared-external-dependency violations.

`🧅️layering.json` and `🚚️migration.json` are unrelated shrink-only ratchets (implementation-area reference counts / unmanaged-test counts) — remodel/procedural have **zero** hits in either file; a clean new plugin built to the current conventions shouldn't need entries here either. Regenerate only if `bun ./📜️script.ts verify layering` fails and deliberately with `bun ./📜️script.ts verify layering write-baseline` (never to silence a real failure).

## 10. Policy allowlist — likely not needed

`✏️s/🔌️plugins/🔒️policy-allowlist.json` (347 lines) has **zero** hits for `remodel` or `procedural` — neither reference plugin needed an allowlist entry. This file is a per-area opt-in for specific policy rules (e.g. `"semantic-vocabulary"` exemptions); only touch it if a specific `verify` policy rule flags a wfc path and the fix is a deliberate, documented exemption rather than a code change.

## 11. Launch / dev-preview configs

- **`.claude/launch.json`** (this agent harness's own preview-server registry) — array of `{ "name": "<plugin>-react-attach", "url": "http://localhost:<port>", "port": <port> }`; remodel's is lines 425-429 (`"remodel-react-attach"`, port 6063). Add one `"wfc-react-attach"` entry (or one per artifact variant if each gets its own port — see §12) pointing at whichever port you assign.
- **`.vscode/🧩️launch.seed.jsonc`** — the schema-owned seed that a generator (script.ts ~line 8942, `interactivityAllAppPlaygroundLaunchNames`, consuming a `INTERACTIVITY_ALL_APP_PLAYGROUND_FILE`-style seed) expands into `.vscode/launch.json`'s `@generated:<plugin>:react`/`@generated:<plugin>:wgpu` blocks. remodel's seed entry is at lines 2764-2778 (`"remodel": { "namePrefix": "🏺️remodel", "command": "bun nx run workspace:dev -- remodel", env: { "REMODEL_PLAY_PORT": "{PORT}", ... } }`) plus explicit template rows at 1786-1792. Add a mirrored `"wfc": {...}` block using `WFC_PLAY_PORT` (or per-variant env vars if 5 separate playgrounds). **Do not hand-edit the `@generated:` regions of `.vscode/launch.json` directly** — find and run whatever `generate`/`nx` target regenerates them from the seed (grep script.ts / nx.json for the consumer of `🧩️launch.seed.jsonc` at edit time; this exploration did not pin down the exact regenerate command, only the seed→generated relationship).
- **`.vscode/launch.json`** itself, remodel's hand-visible (already-generated-looking) entries: lines 3251-3298, three configs — `🛠️dev📸️remodel📸️remodeling⚛️react` (`REMODEL_PLAY_PORT: "6063"`), `...🧊️wgpu🌐️wasm` (`REMODEL_PLAY_PORT: "6163"`), `...🧊️wgpu🖥️native` (`bun nx run @semio-tech/framework-renderer-wgpu:native -- remodel`). Five wfc artifacts likely means either 5x these blocks (per-variant ports) or 1 set using the plugin id `wfc` with a variant selector — mirror whichever pattern the seed generator actually emits for procedural (which also has multiple variants: `generation2d`, `generation3d`, `assembly`) rather than remodel (single-artifact) — check procedural's generated blocks in `.vscode/launch.json` before deciding.

## 12. Ports — 5 free `react`/`wgpu` pairs for wfc

Scanned every `ports = { react = N, wgpu = M }` / `user_ports = {...}` across all `✏️s/🔌️plugins/*/📦️packages/🦀️rust/Cargo.toml` (36 plugin manifests, ~65 variant entries). Highest allocated: `react = 6107` / `wgpu = 6207` (demonstrator). A pre-existing anomaly worth noting (not to imitate): `procedural`'s `generation3d` variant and `shooting` both currently claim `react = 6019 / wgpu = 6119` — an existing duplicate in the repo, unrelated to wfc.

Two verified-free options:

- **Tail allocation** (matches the repo's apparent "next number after current max" convention): `react 6108-6111` and `wgpu 6208-6212` are confirmed completely unused anywhere in the workspace — but only **4** consecutive react numbers are free before hitting `puzzle`'s `wgpu = 6112/6113/6114` block (react side collides at 6112+), so this only cleanly covers 4 of the 5 wfc artifacts.
- **Recommended: mid-range gap `react 6041-6045` / `wgpu 6141-6145`** — a fully free, fully contiguous 5-pair block (confirmed unused in either role, sitting right after `gis`'s `react=6040/wgpu=6140`). Suggested assignment:

  | wfc artifact | react | wgpu |
  |---|---|---|
  | bitmap | 6041 | 6141 |
  | 2d-grid | 6042 | 6142 |
  | 2d | 6043 | 6143 |
  | 3d-grid | 6044 | 6144 |
  | 3d | 6045 | 6145 |

  (Re-verify at edit time with `grep -rn "604[1-5]\|614[1-5]" ✏️s/🔌️plugins/*/📦️packages/🦀️rust/Cargo.toml` since other concurrent tickets may have claimed ports since this scan on 2026-09-18.)

## 13. Dependency-direction / layering gate — no action expected

`bun ./📜️script.ts verify layering report|enforce` is a shrink-only ratchet over **repo-wide/framework** files that reference an "implementation area" (`areaLayers` in taxonomy.json); it does not key off individual plugins at all. Adding a new plugin shouldn't move this number unless framework code starts special-casing wfc by name somewhere it shouldn't. No action needed unless the gate actually fails.

## 14. Full top-level gate to run once everything above is done

`bun ./📜️script.ts verify` → `runGate()` (script.ts ~line 15158) runs, among others:
```
bunx dependency-cruiser 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .../🕸️dependency-boundaries/🟨️.cjs --output-type err
bun nx run @semio-tech/plugin-registry:check
bun nx run @semio-tech/framework-renderer-react:lint
bun nx run @semio-tech/framework-os-dev:plugin lint
bun nx run @semio-tech/ui-styling-tokens:check-no-px
bun nx run @semio-tech/framework-rs:check
```
plus (elsewhere in the same command, not shown in the excerpt read) taxonomy/dependencies/layering/interactivity verification. This is the single command that should catch anything the manual checklist above missed or mis-copied.

---

## Ordered checklist (do in this order)

1. Scaffold `✏️s/🔌️plugins/🌊️wfc/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts,🦀️.rs}` and `.../📦️packages/🟦️typescript/{package.json,📋️project.json,🟦️.ts}` from the remodel templates (§2, §3). Assign ports per §12 (`react 6041-6045` / `wgpu 6141-6145`), package id `semio:wfc`, and either a `[package.metadata.semio.storybook]` block or nothing (§8).
2. Scaffold the 5 artifact crates under `✏️s/🔌️plugins/🌊️wfc/🗿️artifacts/<bitmap|2d-grid|2d|3d-grid|3d>/📦️packages/🦀️rust/Cargo.toml`.
3. Root `Cargo.toml`: add 1 plugin member + 5 artifact members to `members = [...]`, and 1 plugin alias + 5 artifact aliases to `[workspace.dependencies]` (§1).
4. Root `package.json`: add the TS package to `workspaces` (§4); optional `dev:wfc` script.
5. `🧰️framework/.../🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`: add `{ "pluginId": "wfc", "directoryName": "🌊️wfc" }` (§5).
6. `bun nx run @semio-tech/plugin-registry:generate` to regenerate `🤖️generated/*.json` from the new Cargo manifests + catalog entry.
7. `🔣️taxonomy.json`: mirror every remodel/procedural fileKind + memberNames + interfaces-array hit for `wfc` and its 5 artifacts (§6) — re-grep at edit time, don't trust stale line numbers.
8. `bun ./📜️script.ts verify taxonomy enforce` — fix findings.
9. Write wfc's actual schema/mutation/IO code (out of scope for this checklist).
10. `bun ./📜️script.ts schema generate` (regenerates `🔣️schema-catalog.json`/`.md`, §7) — no manual edit.
11. `bun nx run <wfc-rust-project>:describe` (or `nx run-many -t describe`) to emit `✏️s/🔌️plugins/🌊️wfc/{🛂️.descriptor.semio,🔣️.json}` (§3).
12. `bun nx run @semio-tech/plugin-registry:check` — should now be green.
13. `bun ./📜️script.ts verify dependencies write-baseline` then `bun ./📜️script.ts verify dependencies literal-external` (§9).
14. `.claude/launch.json` + `.vscode/🧩️launch.seed.jsonc` (+ whatever regenerates `.vscode/launch.json`'s `@generated:wfc:*` blocks) — add wfc dev-preview entries (§11).
15. If wfc has Storybook stories with Tailwind classes, add the `@source` line to `.storybook/🎨️styling/🎨️.css` (§8).
16. `bun ./📜️script.ts verify` (full gate, §14) — fix anything still red.
17. `bun ./📜️script.ts verify layering report` — expect no change; investigate only if it fails (§13).

---

### Caveats / things this exploration could not fully pin down (flag to a human before relying on them)

- The exact convention for whether a plugin's `members-of-plugins` `memberNames` entry (taxonomy.json) is the plugin-folder name or the artifact name is genuinely inconsistent between remodel (`📸️remodeling`, artifact name) and procedural (`🌀️procedural`, plugin name) in the current repo state — likely because remodel has exactly one artifact and an in-flight rename ticket (`26/07/19/FULL-PHOTOGRAMMETRY-AND-VIDEOGRAMMETRY-STACK-FOR-REMODEL/REWRITE-REMODEL-DOCUMENT-SCHEMA...`). Confirm against `🤖️generated/🔌️plugins.json`'s real `packageId`/`pluginId` (§5) before copying either pattern for wfc, since wfc has 5 artifacts (like procedural, not remodel) — the procedural pattern (plugin-name-keyed) is the more likely correct analogy.
- The exact nx/script.ts command that regenerates `.vscode/launch.json`'s `@generated:*` blocks from `.vscode/🧩️launch.seed.jsonc` was not located in this pass (only the seed-consuming verification helper at script.ts ~8942 was found). Locate it before hand-editing near the `@generated:remodel:react`/`@generated:remodel:wgpu` markers (lines 1786-1792), or you risk a generator overwriting a hand-added block.
- Demonstrator (`✏️s/🔌️plugins/🎪️demonstrator`) and docs (`📓️schema-catalog.md`, any top-level plugin-list README) were checked for remodel references and found clean (remodel isn't wired into the demonstrator showcase) — so no demonstrator-specific registration is expected for wfc either, unless the ticket explicitly wants wfc featured there.

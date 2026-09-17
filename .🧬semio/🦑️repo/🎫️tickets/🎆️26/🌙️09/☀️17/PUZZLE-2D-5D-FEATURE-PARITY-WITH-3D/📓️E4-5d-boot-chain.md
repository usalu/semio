# E4 — puzzle 🖐️5d React-dev boot chain vs 🧊️3d/◻️2d, source-measured

Read-only source audit, 2026-09-17. No cargo/nx/bun build or `bun nx show project` run — every claim
below is `grep -n`/`Read` on the tree, plus one confirmed **stale-artifact** signal from a `dist/` tree
that predates this session (noted where used). File:line citations use this repo's own emoji paths.

## 0. TL;DR

Nx target inference for `activate-puzzle5d-react-dev` / `serve-puzzle5d-react-dev` **is wired** — it
falls out of the same generic `[[package.metadata.semio.playground]]` mechanism as 2d/3d, and the
`puzzle5d` entry already exists in the plugin's `Cargo.toml` with a port assigned (`react = 6014`). The
targets should therefore already exist in the Nx graph today, same as 3d's. What is **actually missing**
is all hand-maintained:

1. **`.claude/launch.json` has no `puzzle5d-react` / `puzzle5d-wgpu` entries at all** (2d and 3d both do).
2. The artifact-root `🛂️manifest.json` for 3d **and** 5d is named wrong
   (`🛂️manifest.jsondefault.manifest.json`) and fails the taxonomy's canonical-filename statute — 2d's
   is correctly named. This is pre-existing on 3d too, not 5d-specific, but it means "boot puzzle5d like
   puzzle3d" inherits the same drift.
3. No separate wasm-pack/engine gap exists for React dev: 2d, 3d and 5d are all compiled into **one**
   cdylib (`semio-s-plugin-puzzle`), so 5d's dual board+world surfaces ride the same
   `component-dev`/`materialize-dev` build as 3d — nothing extra to wire there.
4. The React host mechanism for rendering *both* a 2D board surface and a 3D world surface in one app
   **already exists and is already used by 5d's own Rust source** (`SurfaceKind::Board2d` +
   `SurfaceKind::World3d` as two separate `windowKinds`, exactly how a multi-window app is built) — this
   is not a gap, contrary to what the ticket brief implied should be checked.

None of this touches `E3`'s finding (35/52 of 5d's actions are still `BatchOnlyPendingRewrite`, so even
once the window boots, most interaction — camera, gumball, fastener CRUD — is dead). That is a runtime
gap, not a boot-chain gap; this report is boot-chain only.

## 1. How the Nx targets are actually inferred (not hardcoded per app)

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` is the Nx inference plugin registered in
`nx.json:50-53` (`plugins[2]`, matching `**/📋️project.json`, `**/Cargo.toml`, …). Two functions matter:

- `playgroundSessionTargets` (🟨️.mjs:886-905) — for project `@semio-tech/plugin-registry`
  (🟨️.mjs:1052), emits `session-${variant}` per `[[package.metadata.semio.playground]]` row found in
  **any** `Cargo.toml` in the repo (walked via the `configFiles` the plugin is handed).
- `playgroundPreparationTargets` (🟨️.mjs:910-993) — for project `@semio-tech/framework-os-dev`
  (🟨️.mjs:1053, i.e. the TS package at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`), emits, per
  playground row × `profile ∈ {dev, release}`:
  - `prepare-${variant}-react-${profile}`, `activate-${variant}-react-${profile}` (🟨️.mjs:950-957,
    958-964),
  - `serve-${variant}-react-${profile}` / `dev-${variant}-react-${profile}` (🟨️.mjs:949) — command
    `bun ./📜️script.ts serve ${variant} react ${profile}`,
  - matching `-wgpu-` and `-native-` families,
  - `build-${variant}-react-release` (🟨️.mjs:982-990).

  This is driven **purely by the `[[package.metadata.semio.playground]]` rows it finds** — there is no
  per-plugin-id branch anywhere in this file. Whatever exists for `puzzle3d` exists for `puzzle5d` iff the
  Cargo.toml row exists.

The playground rows live in the **plugin composition crate**, not the artifact crates:
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:26-46`:

```toml
[[package.metadata.semio.playground]]
variant = "puzzle2d"
app = "s.puzzle.puzzle2d@1/*#editor"
aliases = ["2d", "puzzle 2d"]
ports = { react = 6012, wgpu = 6112 }
engines = ["./✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust"]

[[package.metadata.semio.playground]]
variant = "puzzle3d"
app = "s.puzzle.puzzle3d@1/*#editor"
aliases = ["3d", "puzzle 3d"]
ports = { react = 6013, wgpu = 6113 }

[[package.metadata.semio.playground]]
variant = "puzzle5d"
app = "s.puzzle.puzzle5d@1/*#editor"
aliases = ["5d", "puzzle 5d"]
ports = { react = 6014, wgpu = 6114 }
```

So: **puzzle5d's playground row exists, is well-formed, and assigns port 6014 (react) / 6114 (wgpu)** —
identical shape to puzzle3d's row, one port block higher. Nothing here is missing. (Note: only `puzzle2d`
declares `engines` — the standalone wasm producer used by the native/wgpu preparation chain
(🟨️.mjs:930-934, `prepare-*-wgpu-*`'s dependsOn). `puzzle3d`/`puzzle5d` both omit it, same as each other,
so this is not 5d-specific drift — and it doesn't affect the **react** dev target this ticket cares about,
only `prepare-*-wgpu-*`.)

`app = "s.puzzle.puzzle5d@1/*#editor"` is the app id `activate`/`prepare`/`serve` resolve against
(via the plugin registry, `@semio-tech/plugin-registry:session-puzzle5d` — see §4) to find the actual
window/editor definition; `?plugin=puzzle5d` is the query param the React host boot-query layer
(`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts`) uses to pick the session the same way
`?plugin=puzzle3d` does today.

## 2. Boot recipe — 3d (working reference, verified from source)

```
bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev
# or directly:
bun nx run @semio-tech/framework-os-dev:serve-puzzle3d-react-dev
```
- Port: **6013** (Cargo.toml `ports.react`), URL `http://127.0.0.1:6013/?plugin=puzzle3d`.
- `.claude/launch.json:70-85` config `"puzzle3d-react"` runs exactly
  `bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev`, port 6013,
  `env.SEMIO_RENDERER=react`, `env.S_OS_PORT=6013`.
- `.claude/launch.json:220-224` also has a `"puzzle3d-react-attach"` (`url: http://localhost:6013`) for
  attaching to an already-running server.
- Session dist already generated at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/sessions/puzzle3d/🎮️playground-session`.
- Artifact-root manifest: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🛂️manifest.jsondefault.manifest.json`
  — **also** misnamed (see §5); this is not 3d-vs-5d drift, it's a pre-existing defect on 3d too.

## 3. Boot recipe — 2d (working reference, verified from source)

```
bun nx run workspace:dev -- 2d
```
- This is the **root forwarding command**, not a direct `os-dev:*` target — it resolves the alias `"2d"`
  against the `aliases = ["2d", "puzzle 2d"]` list on the `puzzle2d` playground row, then dispatches to
  the same `serve-puzzle2d-react-dev` machinery under the hood.
- Port: **6012**. `.claude/launch.json:182-193` config `"puzzle2d-react"` runs
  `bun nx run workspace:dev -- 2d`, port 6012.
- Kept alive across restarts by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🔁️serve-supervisor.sh`.
- Artifact-root manifest: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🛂️manifest.json` — **correctly named**
  (see §5).
- `engines = ["./✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust"]` is declared only here — 2d is the one variant
  with a standalone wgpu/native wasm producer wired through the playground row.

## 4. Boot recipe — 5d, as it stands today (derived, not run)

```
bun nx run @semio-tech/framework-os-dev:dev-puzzle5d-react-dev
# or:
bun nx run @semio-tech/framework-os-dev:serve-puzzle5d-react-dev
```
- Port: **6014** (`ports.react` in the Cargo.toml row above), URL would be
  `http://127.0.0.1:6014/?plugin=puzzle5d`.
- Per §1, the Nx target should exist today with no code changes, purely from the existing Cargo.toml row.
- Session dist **already exists**: `…/📇️registry/dist/sessions/puzzle5d/🎮️playground-session` — this is a
  build artifact from a prior `session-puzzle5d` run, not proof the chain works *now* (dist can be stale;
  per project memory "queued cargo check reports stale" / "component-release does not materialize" —
  treat this as "was generated once," not "works today").
- **Missing: `.claude/launch.json` has no `puzzle5d-react` / `puzzle5d-wgpu` / `puzzle5d-react-attach`
  entries.** Grep of the whole file (`grep -n puzzle .claude/launch.json`) returns only `puzzle3d-*` (6
  hits) and `puzzle2d-react` (1 hit) — zero `puzzle5d` hits. This is a pure hand-maintenance gap: nothing
  auto-generates `.claude/launch.json` (unlike `.vscode/launch.json`, see below), so port 6014 for puzzle5d
  was simply never added when the Cargo.toml row was created. The fix is mechanical: copy the
  `"puzzle3d-react"` block (`.claude/launch.json:70-85`) and the `"puzzle3d-wgpu"` block
  (`.claude/launch.json:86-97`), rename to `puzzle5d`, swap ports 6013→6014 / 6113→6114 and the trailing
  target/alias from `puzzle3d`→`puzzle5d`.

### 4a. `.vscode/launch.json` — a different, *already-working* generator (not the same file as above)

The ticket brief's "launch seed" (`🧩️launch.seed.jsonc`) is **not** `.claude/launch.json`'s seed — it
feeds `.vscode/launch.json` (VS Code debugger configs), an entirely separate pipeline owned by
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts:1-23`: renders
`.vscode/launch.json` from `.vscode/🧩️launch.seed.jsonc` **plus the playground registry** (ports come
from the Cargo.toml row, not hand-typed). `.vscode/🧩️launch.seed.jsonc:905-992` already has
`@generated:puzzle5d:react`/`@generated:puzzle5d:wgpu` markers, a `"puzzle5d"` devLauncher template
(`namePrefix: "🧩️puzzle👯️5d"`, line 2544-2545), and explicit example-specific entries
(`…puzzle👯️5d🎛️concrete🌲️forest⚛️react`, `…puzzle👯️5d🎛️capsule🌙️dream⚛️react`, …). The generated
`.vscode/launch.json` does contain live puzzle5d wgpu-native command lines (confirmed by grep, lines
~1748/1799). **This pipeline is not broken for 5d** — it already tracks the Cargo.toml row correctly. Only
`.claude/launch.json` (hand-maintained, no generator) is missing 5d.

## 5. The `🛂️manifest.jsondefault.manifest.json` filename — confirmed taxonomy drift, not intentional

Content is fine in all three files (valid `"schema": "manifest"` documents; 5d's even has a proper
`kindCompatibility` block 2d/3d lack). The filename is the defect:

- 2d: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🛂️manifest.json` — correct.
- 3d: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🛂️manifest.jsondefault.manifest.json` — wrong.
- 5d: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🛂️manifest.jsondefault.manifest.json` — wrong, same way.

The canonical-name statute is in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:5141`:

```ts
if (!canonicalTaxonomyEmoji(identity.first) || identity.rest !== "manifest.json" || …)
  problems.push(`semanticManifestFilenameOverrides[…] must name one canonical semantic emoji
  followed by "manifest.json" in the semantic manifest file kind.`);
```

`canonicalPrimaryFilenameForKind` (discovery/🟦️.ts:1409) is what a collection owner's manifest filename
is *supposed* to resolve to, unless overridden in `taxonomy.json`'s `semanticManifestFilenameOverrides`
map. That map (`🔣️taxonomy.json:28665-28668`) contains exactly two entries, both for the stdio gltf
import/export serializer dirs — **no puzzle entry at all**. So the canonical expected filename for the
puzzle 3d/5d artifact-root manifest is `🛂️manifest.json`, exactly like 2d has. The actual files instead
end in `…manifest.jsondefault.manifest.json`, i.e. `identity.rest !== "manifest.json"` — this fails the
statute (this is the "Taxonomy Filename Drift Blinds Discovery" pattern from project memory: fileKinds
drift silently returns 0 rather than erroring loudly).

Separately, the **graph-manifest** consumer (a different, narrower consumer) is unaffected:
`🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📥️admission/🟦️.ts:62` does `name.endsWith("manifest.json")`,
which still matches the doubled filename, so the Neo4j graph-catalog export (`graph-catalog-manifests`,
`🔣️taxonomy.json:7770-7785`, which lists `◻️puzzle2d-default`/`🧊️puzzle3d-default`/`🖐️puzzle5d-default` as
expected generated members) still picks up all three files fine. **The break is specifically in the
taxonomy/discovery canonical-filename gate, not in this graph consumer, and not in anything on the React
boot path itself** — so it does not block booting puzzle5d in the dev host, but it does mean any drift
gate that walks the taxonomy (discovery) will not "see" 3d's or 5d's artifact manifest the way it sees
2d's. Likely origin: some prior codemod appended `default.manifest.json` to an already-`.json`-suffixed
name (`🛂️manifest.json` + `default.manifest.json` concatenated) — worth a targeted `git log -p` on those
two files if a fix is scoped (not run here, read-only).

## 6. Component build / wasm engine — one cdylib serves all three variants

`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` (`semio-s-plugin-puzzle`, `package.metadata.component.package = "semio:puzzle"`) depends on all three artifact crates with `component-app-assembly` turned on
(lines 80-82):

```toml
semio-s-artifact-puzzle-2d = { workspace = true, features = ["component-app-assembly"] }
semio-s-artifact-puzzle-3d = { workspace = true, features = ["component-app-assembly"] }
semio-s-artifact-puzzle-5d = { workspace = true, features = ["component-app-assembly"] }
```

`componentTargets` (🟨️.mjs:854-883) builds this crate's `component-${profile}` (cargo/wasm-pack, per
`⚡️caching/🦀️cargo/📜️script.ts`) and `materialize-${profile}` (browser-bundle materialization) **once**,
keyed by `metadata.component.package` ("semio:puzzle") → one directory in the module catalog
(`🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`). There is no per-variant wasm build — puzzle2d,
puzzle3d and puzzle5d are three windows/apps sharing one compiled component. This is why 5d's "board
engine" and "world engine" don't need separate wasm-pack targets: they're the same binary 3d and 2d
already ship, just with `puzzle-5d`'s own crate linked in alongside.

`wasm-pack` profile overrides (`package.metadata.wasm-pack.profile.{dev,release,custom}` →
`wasm-opt = false`, Cargo.toml:17-24) apply to the whole `semio-s-plugin-puzzle` component build, so they
already cover 5d — no separate opt-out needed.

`[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]` block, compared:

| crate | js-sys | web-sys |
|---|---|---|
| `semio-s-artifact-puzzle-2d` (Cargo.toml:50-52) | ✓ | ✓ `web-sys = "0.3.98"` `features=["HtmlCanvasElement"]` |
| `semio-s-artifact-puzzle-3d` (Cargo.toml:56-57) | ✓ | ✗ |
| `semio-s-artifact-puzzle-5d` (Cargo.toml:49-50) | ✓ | ✗ |

5d's own crate depends on `semio-s-artifact-puzzle-3d` (its Cargo.toml:34) but **not**
`semio-s-artifact-puzzle-2d` directly — the "2d board engine" the ticket brief refers to is not a direct
Cargo dependency of the 5d artifact crate. However, because the **plugin** crate links all three artifact
crates into one cdylib (above), puzzle-2d's own `web-sys`/`HtmlCanvasElement` dependency is present in the
final wasm regardless of whether puzzle-5d's crate declares it directly. 5d's own board window
(`🪟️windows/◻️2d/🦀️.rs`, §7) is built on `semio_framework_ui_contract::SurfaceKind::Board2d` +
`semio_framework_plugin::scene_surface`, i.e. the shared UI-scene contract, not raw `web-sys` calls — so
this asymmetry looks intentional, not a dropped dependency. No action item here for the React boot path.

## 7. Playground registration / examples / React host surfaces — 5d already declares dual windows

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/`
has **two** window definitions, each its own `SurfaceKind`:

- `◻️2d/🦀️.rs:34,177` — `surface_kind: SurfaceKind::Board2d`,
  `semio_framework_plugin::scene_surface(SURFACE_ID, …SurfaceKind::Board2d, &scene)`.
- `🧊️3d/🦀️.rs:44,193` — `surface_kind: SurfaceKind::World3d`,
  `semio_framework_plugin::scene_surface(SURFACE_ID, …SurfaceKind::World3d, &scene)`.

On the React side, both host components are already exported from the shared react target aggregator
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`:
- `Board2dHost` (🟦️.tsx:1202-1243, re-exported from
  `…/🧱️elements/🖥️Board2dHost/🟦️.tsx`) — the same host 2d's editor uses.
- `World3dHost` (🟦️.tsx:1037-1043, re-exported from `…/🧱️elements/🌐️World3dHost/🟦️.tsx`) — the same host
  3d's editor uses.

Apps declare a `windowKinds` array (see `🏛️ShellHost`/`🐚️Shell` `🟦️.tsx`, e.g.
`session.app.windowKinds`, `withLocalizedWindowKindLabels`, `applyFrameworkLayoutSeed(...windowKinds)`)
and the shell picks the host component per window by its `SurfaceKind`, same generic mechanism every
multi-window app (fem2d+fem3d-style split panes, cad, etc.) already uses. **5d rendering a Board2dHost
window and a World3dHost window side by side in one app is therefore not a missing capability** — it is
the same mechanism 3d and 2d each use singly, and 5d's own Rust source already declares both windows.
(Whether interacting with either window *does* anything is the separate, already-documented `E3` finding:
`setCamera2d`/`setCamera3d`/gumball/fastener actions are `BatchOnlyPendingRewrite`, i.e. dead on click.)

Examples discovery: 5d's standard has **three** example dirs vs 2d/3d's two —
`…/🖐️5d/…/✏️editor/📚️examples/` and the standards-level
`…/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/{🌙️capsule-dream,🌲️concrete-forest,🏗️nakagin-capsule-tower}`
(3d/2d only have `🌲️concrete-forest` and `🏗️nakagin-capsule-tower`). This matches the extra `capsule dream`
devLauncher entries already seeded in `.vscode/🧩️launch.seed.jsonc:970-992`. Discovery itself is generic
(`examplesForDialect` / the framework's directory-walk example picker,
`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` + `🧪️tests/🔬️example-picker/🦀️.rs`), not per-plugin code, so no
extra wiring is needed for the third example beyond the directory existing (confirmed present).

## 8. Missing/broken pieces — summary with file:line

| # | What | Where | Status |
|---|---|---|---|
| 1 | `.claude/launch.json` has zero `puzzle5d-*` entries | `.claude/launch.json` (whole file; contrast lines 70-97 `puzzle3d-react`/`puzzle3d-wgpu`, 182-193 `puzzle2d-react`, 220-224 `puzzle3d-react-attach`) | **Missing — hand-maintained file, needs a `puzzle5d-react`/`puzzle5d-wgpu`/`puzzle5d-react-attach` block copied from the puzzle3d ones, ports 6014/6114** |
| 2 | Artifact-root manifest filename fails canonical taxonomy statute | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🛂️manifest.jsondefault.manifest.json` and the equivalent 3d file; statute at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:5141`, no override in `🔣️taxonomy.json:28665-28668` | **Pre-existing drift shared with 3d, not 5d-only** — doesn't block React boot, does blind taxonomy/discovery gates |
| 3 | `puzzle5d` playground row has no `engines` entry | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:42-46` | Same as 3d (also has none) — only affects standalone `prepare-*-wgpu-*`, not react dev; not a regression specific to 5d |
| 4 | `semio-s-artifact-puzzle-5d` doesn't directly depend on `semio-s-artifact-puzzle-2d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml:21-47` (only depends on puzzle-3d, line 34) | Not a bug — the plugin crate links all three into one cdylib; 5d's board window uses the shared UI-scene contract, not the 2d crate directly |
| 5 | 5d's dual-surface (board + world) React rendering | `…/🖐️5d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/{◻️2d,🧊️3d}/🦀️.rs`; hosts at `…/🧱️elements/{🖥️Board2dHost,🌐️World3dHost}/🟦️.tsx` | **Already wired**, not missing |
| 6 | Session dist for puzzle5d | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/sessions/puzzle5d/🎮️playground-session` | Present (prior build artifact — freshness not verified, no build run this session) |

## 9. Net: what actually blocks `bun nx run @semio-tech/framework-os-dev:serve-puzzle5d-react-dev` today

Based on static source alone, nothing in the Nx graph, script.ts dispatcher (a thin generic router,
`…/📦️packages/🟦️typescript/📜️script.ts:1-85` — no per-plugin branches), or component build should
prevent this target from existing and running, port 6014. The only concrete, source-confirmed omission
that a person would actually hit is **item 1** (no `.claude/launch.json` entry, so there is no
one-command way to launch/attach to it the way `puzzle3d-react`/`puzzle2d-react` have) — everything else
in this report is either shared drift with 3d (item 2) or not a gap at all (items 3-6). This audit did not
execute `bun nx run …serve-puzzle5d-react-dev` (host is loaded and cargo/nx runs are out of scope for a
read-only audit); confirming the target actually resolves and the dev server actually comes up on 6014 is
the natural next, execution-based step, out of scope here.

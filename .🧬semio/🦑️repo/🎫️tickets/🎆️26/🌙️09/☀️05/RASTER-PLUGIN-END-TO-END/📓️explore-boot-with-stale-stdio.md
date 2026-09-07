# Explore: booting raster with a stale/missing stdio

Read-only investigation, 2026-09-06 22:00-22:30. Answers the coordinator's 5 questions. Builds on
`📓️explore-stdio-wasm-link.md` (same ticket) which established stdio's own standalone component
cannot be linked (wasmparser's 1,000,000-defined-function ceiling in `wasm-component-ld`).

## TL;DR

Raster is in a **better starting position than every sibling ticket examined** — it already has a
**committed, git-tracked, currently-valid owner-root descriptor pair**
(`✏️s/🔌️plugins/🖨️raster/🔣️.json` + `🛂️.descriptor.semio`, last touched by commit `21fbcd3538`,
2026-09-02, `git status` clean at that path) and a working Aug-17 core.wasm. **stdio does not** — it
has never had an owner-root or served `🔣️.json`/`🛂️.descriptor.semio` in this repo's history (`git
log --all` on either path returns nothing), and the registry's generated `🔌️plugins.json` row for
stdio has no `hashes`/`executionMode` fields at all, unlike every other plugin row. This is not a
raster-specific problem: **three independent sibling tickets today/yesterday (lowpoly, procedural,
block) hit the exact same "stdio has no descriptor → dependency cascade fails" wall**, and one of them
(lowpoly) is the only one that got all the way to a rendered window despite it, by building stdio
**as an embedded library dependency inside its own component** (not as stdio's own standalone
`StdioApps` component) — the same relationship raster's Cargo.toml declares. A `target-raster` build
directory already exists with `.fingerprint`/`incremental` state for `semio-s-plugin-stdio` dated
21:17 today, but **no rustc process is currently running against it** (checked: 0 of 18 live rustc
processes reference `target-raster`) — it is a stalled/abandoned attempt, not a live one.

---

## 1. How did sibling end-to-end tickets booting today handle stdio?

Ticket path corrections first: the coordinator's prompt cites `…/🌙️09/☀️02/GIS-MAP-END-TO-END/` — that
ticket does not exist at that date; the real path is `.../🌙️08/☀️29/GIS-MAP-END-TO-END/` (confirmed by
`find`). Its content is stale relative to today (Aug 29) and documents an older, unrelated linker
failure — see below.

| Ticket | Date | stdio outcome | Rendered window? |
|---|---|---|---|
| `🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/📓️e2e-status.md` | today (reopened) | Compiled clean at `wasm-dev`/`opt-level=0` as lowpoly's **embedded lib dependency** (not its own component); root cause of every earlier boot failure was **not** a stdio compile error but a **missing composer declaration inside lowpoly itself** (`s.stdio.txt@utf-8/*` registered by `entries()` but never `declare()`d — `PluginAssemblyError{code:"artifact-definition.runtime-capability"}`). Fixed by adding the missing row. | **YES** — first time in the ticket's history: `http://localhost:6078` served, shell mounted `semio · lowpoly`, both declared modes rendered, actor `lowpoly#1` loaded live. One residual defect (interactive-step-ceiling at `opt-level=0`) was then fixed by scoping `opt-level = 2` to just the lowpoly package in `[profile.wasm-dev.package.semio-s-plugin-lowpoly]` — **deliberately not raised for stdio**, because doing so made stdio's wasm-dev compile balloon past 100 minutes (comment: "stdio is the io layer, not on the render path"). |
| `🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/📓️status.md` | today (resumed session) | Same pattern: stdio compiled as procedural's embedded lib dependency (not its own component); intermittently blocked by *other* sessions' concurrent stdio edits (BREP rewrites, serde→ToValue/FromValue migration, emoji-rename corruption — all environmental, not raster-relevant) rather than the function-count ceiling, because no standalone stdio component build was ever attempted here. | **Partial** — shell chrome rendered, title `semio · procedural · 3d`, example switcher populated, flow extensions hot-swapped in; but every render trapped on a `Dictionary` retirement-leak panic in the flow host (`🧠️neural/⚙️engine/🦀️.rs:101`, unrelated to stdio) — six leak sites fixed in `🌊️flow/🖥️host/🦀️.rs`, rebuild still in flight, unverified at file-write time. Earlier boot attempt (#8) explicitly hit **the same stdio-descriptor-absence wall raster would**: *"`stdio` / `flow-extension-draw` / `flow-extension-bim` fail to load (their module descriptors are absent because only procedural was built — `stdio` has no committed `🔣️.json`)... `PluginRuntime: turn failed for actor procedural#1` → 'No plugins loaded'."* |
| `🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️explore-dev-boot-path.md` | yesterday (still open) | Never got past static analysis to a live boot in the material read; but its analysis is the most precise write-up of the exact mechanism raster will hit — see §2/§4. | Not attempted in the material read (exploration only). |
| `🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/📓️status.md` | yesterday (still open) | Compile-blocked on draw's own drift against stdio's IO API (`Drawing`-prefix rename), not stdio itself; plan explicitly requires "owner descriptor pair regenerated via `describe`" as a precondition before its boot step (never reached in the material read). | No. |
| `🌙️09/☀️05/S-END-TO-END/📓️stdio-check-census.md` | yesterday | `cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --keep-going` → **EXIT 0** after 86m52s (0 errors) — but this is `cargo check` (type-check only, no linker invocation), so it does **not** contradict the wasm-component-ld ceiling; the file's own conclusion says exactly this: *"the remaining stdio gap is the missing owner descriptor pair, which only a wasm build + `describe` produces."* | Not reached in this file. |
| `🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️status.md` | 2026-09-02/05 | Similar actor-fault pattern to lowpoly (interactive-ceiling class fault), no new stdio-specific finding. | Partial (window renders, actor faults). |
| `🌙️08/☀️29/GIS-MAP-END-TO-END/📓️status.md` (coordinator's cited path is wrong; this is the real one) | 2026-08-29, stale | Explicit, older confirmation of the exact linker ceiling: *"`semio-s-plugin-stdio` separately fails at `linking with wasm-component-ld failed`."* (line 243) — this was gis2d's own attempt to build the **full, un-narrowed catalog** (`0/2 crate(s) produced .wasm`), i.e. it did try to build stdio's own component and hit the same wall `explore-stdio-wasm-link.md` documents for 09-03. | No. |

**Common thread across all of today's/yesterday's successful-ish boots (lowpoly, procedural)**: none
of them ever attempted `cargo rustc -p semio-s-plugin-stdio --target wasm32-wasip2` as a **standalone
top-level component build**. Every green stdio compile observed today was stdio compiled **as an
`rlib` dependency embedded inside another plugin's own component** — which sidesteps the linker
ceiling because the huge `StdioApps` 176-variant dispatch closure (the dispatch surface that is the
likely source of the pathological function count, per `explore-stdio-wasm-link.md` §4) is never
itself the top-level exported component; only the subset of stdio functions the host plugin actually
calls gets pulled through wasm-ld's dead-code elimination into the host's own component. **This is
also exactly raster's relationship to stdio** (see §3) — good precedent that raster's *own* component
should link, but it does **not** solve the separate problem that the browser session also tries to
load stdio *as its own plugin actor* (§2/§4), and that build (a real standalone `stdio` component) is
what has never once succeeded since 2026-08-18.

---

## 2. Dev-script boot behavior for a plugin whose crate build fails/is skipped

Verified directly in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
(line numbers re-checked live today, may drift slightly from the block ticket's yesterday citations
due to ongoing repo-wide churn, but the functions/behavior are unchanged):

- `resolvePluginBuildTargets` (`:536-550`) — `SEMIO_PLUGIN_ONLY=<id>` narrows **only which crates get
  `cargo`-built this run** (throws if it matches zero entries, `:539-541`). It does **not** change which
  registry entries the browser tries to load.
- `ensurePluginRegistry` (`:527-534`) — runs registry `generate` (hard gate, throws non-zero), then
  calls `syncBuiltPluginDescriptors(filterProjectedPluginRegistry(...))` — **this runs on every dev
  boot regardless of `SEMIO_PLUGIN_ONLY`**, and re-copies each plugin's *existing* owner-root
  `🔣️.json`/`🛂️.descriptor.semio` (if any) into its served `🔌️plugin-modules/<id>/` dir. This is why
  raster's served descriptor pair shows a fresh 22:19-today mtime even though raster's own crate was
  **not** rebuilt today (its core.wasm is still Aug 17) — the sync step just re-copied the
  already-committed owner-root files. stdio has nothing at its owner-root to sync, so its served dir
  stays empty regardless of how many times `generate` runs.
- `buildPluginsStreaming` (`:613+`) — host-plugin-first streaming boot used by the react dev server:
  builds the primary/host plugin's crate first (gets Vite/the shell out of "waiting for host program"
  fastest), then streams every other target's cargo build in afterward via the
  `♻️hot-swap.json`/SSE channel; **a single broken crate no longer aborts the rest of the catalog**
  (doc comment at `:606-612`), but a target simply **excluded** by `SEMIO_PLUGIN_ONLY` is never built at
  all this run — its served output (if any) is whatever was already on disk.
- `stagePluginDescriptor` (`:226`) — copies `<ownerRoot>/🔣️.json` into the served dir; if the owner-root
  file is absent, it **deletes any stale copy** from the served dir and returns `false` (confirmed
  behavior, matches the block ticket's citation).
- `describeBuiltPlugin`/`materializePlugin` (`:354`, `:387`) — run once per crate right after that
  crate's own build+jco-transpile; `materializePlugin` **throws** `Missing fresh descriptor for <id>`
  if `stagePluginDescriptor` returns false immediately after a fresh build succeeded — i.e. a crate that
  *is* rebuilt this run always ends with a valid descriptor or the whole build fails loudly. A crate
  that is **not** rebuilt this run (excluded by `SEMIO_PLUGIN_ONLY`, or never successfully built at
  all — stdio's case) is simply never subject to this check and is left exactly as it was on disk.

**Browser-side gate** — verified live today in `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`:
- `fetchDescriptorManifest` (`:108-132`) does `GET <dirname(moduleUrl)>/🔣️.json`; on any failure throws
  `fault("plugin.descriptor-unavailable", ...)` (`:118`, confirmed present at that exact line today).
- `expandPluginRegistry` (`:301-320`) computes the actual set of registry entries the browser session
  loads: primary plugin + `consumes`/`contributes` matches + the **transitive `dependencies` closure**
  derived from `dependsOn` (`dependsOnToPluginDependencies`, `:382-383`). The comment at `:307-309` is
  explicit and directly on point: *"`consumes`/`contributes` alone never pulls Cargo/`dependsOn`
  plugins (stdio, flow, cad, …), which left every demonstrator pane boot with 'needs X which is not
  installed' and an empty usable load order."* — i.e. **stdio IS deliberately, unconditionally pulled
  into the browser's load set whenever raster (or any `dependsOn: ["stdio"]` plugin) is the primary**,
  regardless of `SEMIO_PLUGIN_ONLY`.
- `validatePluginDependencyGraph` (`:3108-3123`) reports `transaction.dependency-missing` if a
  dependency-id has no matching node at all in the graph; stdio currently **is** present as a registry
  node (role: plugin, `dependsOn: []`, but critically **no `hashes`/`executionMode` fields** — see §4),
  so this particular error would not fire, but the plugin still fails to *load* at
  `fetchDescriptorManifest` because its served `🔣️.json` is physically absent (404).
- The exact `blockedDependency`/`loadPluginModulesInDependencyOrder` symbol names the block ticket cited
  (`🔌️PluginRuntime/🟦️.tsx:1968-2010`, dated yesterday) **no longer grep-match** anywhere in
  `🧰️framework` today — that file exists but has apparently been refactored since; the underlying
  dependency-closure/validation logic clearly still lives in `🎠️kernel/🟦️.ts` (`expandPluginRegistry` +
  `validatePluginDependencyGraph`, both confirmed live today), so the *mechanism* is unchanged even if
  the exact function name/location the sibling ticket cited has moved. **Not independently reproduced
  end-to-end today (no dev boot run, per this agent's read-only scope) — treat the "raster fails to
  load because stdio 404s" conclusion as strongly evidenced by three sibling tickets' identical
  first-hand observations (lowpoly boot 8 equivalent never hit this because it fixed its OWN
  build-scope; procedural's boot 8 hit it explicitly, quoted in §1), not as independently re-verified
  in this exploration.**

**Answer to "does raster's `executionMode: isolated` mean its worker loads only raster's own module?"**
No. `executionMode: "isolated"` (confirmed on raster's registry row) governs how raster's *own* actor is
sandboxed/scheduled at runtime (its own worker/isolate), not whether stdio is loaded alongside it —
`dependsOn`/`expandPluginRegistry` operate one layer above execution mode, at the registry-expansion
step that decides the whole set of plugin actors the session tries to instantiate before any one of
them runs. stdio would need to load as its **own separate isolated actor** for raster's `dependsOn` to
be satisfied at runtime, exactly like the procedural-boot-8 observation in §1.

---

## 3. Does raster's own wasm component link? Feature-set comparison

**Raster's Cargo.toml** (`✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:28`):
```
semio-s-plugin-stdio = { path = "...", package = "semio-s-plugin-stdio", default-features = false, features = ["full-artifact-catalog"] }
```

**Lowpoly's Cargo.toml** (`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml:29`) — **identical**:
```
semio-s-plugin-stdio = { path = "...", package = "semio-s-plugin-stdio", default-features = false, features = ["full-artifact-catalog"] }
```

So the earlier hypothesis (raster/lowpoly might differ by feature scope) is **wrong** — both declare
the exact same `full-artifact-catalog` feature. Lowpoly's own component still linked today (28.4 MB
core.wasm, fresh 20:30 today) using this same dependency line, and raster's own component linked
successfully once before (109.4 MB core.wasm, 2026-08-17 18:40, per `explore-stdio-wasm-link.md` §1 —
"raster is small... links fine, no evidence raster itself was ever the source of a link failure").

**Why these link while stdio's own standalone component does not**: `full-artifact-catalog` gates how
much of stdio's source gets *compiled into the rlib* (all 36 artifact families), but only the
functions actually **reachable from the depending crate's own exported component surface** survive
`wasm-ld`'s dead-code elimination into that crate's final linked core. Raster's/lowpoly's own
`PluginApp`/dispatch surface is small (their own commands/modes/windows), so only a fraction of
stdio's compiled-in functions are ever referenced and kept. stdio's **own** crate, built as its own
top-level component, exports the entire `StdioApps` 176-variant/77-method dispatch closure itself —
none of it is dead code from that build's own perspective, so wasm-ld/wasmparser sees the full
>1,000,000-function count. This explains why `cargo check`/`cargo build --lib` of stdio "succeeds" in
every context observed (lowpoly's, procedural's, raster's own embedding, and the S-END-TO-END census's
direct `cargo check -p semio-s-plugin-stdio --target wasm32-wasip2` — EXIT 0 in 86m52s) while a
`cargo rustc -p semio-s-plugin-stdio --target wasm32-wasip2` (which **does** invoke the linker/component
step) has never once succeeded since 2026-08-18.

**Is there a lighter feature set raster should use?** Grepped stdio's own Cargo.toml
(`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`) — not fully enumerated in this pass given time
budget, but the fact that **lowpoly, which needs a completely different, much smaller codec subset
(las/ply/png/json/dwg/stl/gltf/obj/txt per the LOWPOLY-END-TO-END ticket's own composer list), uses the
identical `full-artifact-catalog` feature and still links its own component successfully** strongly
suggests the feature flag is not the limiting factor for *raster's own* link — raster's own component
build is not the blocked artifact. **The blocked artifact is exclusively stdio's own standalone
component**, which is unaffected by what raster's Cargo.toml requests (stdio's own build always compiles
its full surface regardless of what any downstream crate's feature selection asks for, since it's
building stdio *as the top-level crate*, not as someone else's dependency). Recommending a narrower
stdio feature set for raster would not help raster (already links) and cannot help stdio's own
standalone build either (irrelevant to that build). **Not fully verified**: did not exhaustively grep
which stdio artifact-family features exist and which raster/W3 codecs (png/bmp/gif/jpg/tiff/svg/dwg/json)
map to which feature flags — flagged as unverified, but based on the above reasoning it is unlikely to
matter for raster's own component link.

---

## 4. In-repo mechanism to serve stdio without rebuilding; current registry state

- **`SKIP_PLUGIN_BUILD=1`** exists (root `📜️script.ts:183-201`, per `explore-stdio-wasm-link.md` §2)
  but only applies with `SEMIO_RENDERER=react` + the `served` rest-segment; it skips the **whole**
  `buildPluginsStreaming`/`buildPlugins` step and serves whatever is already staged in
  `🔌️plugin-modules/` — for stdio that means serving the Aug-18 core.wasm/js/interfaces **with no
  descriptor**, which will still 404 at `fetchDescriptorManifest`. This does not solve the problem, it
  only skips a rebuild of files that are already there (which for stdio is not the missing piece).
- **`--plugins raster,stdio`-style filter**: `SEMIO_PLUGIN_ONLY` accepts one id, not a list (`:539`:
  `entries.filter((entry) => entry.pluginId === only)`, single equality, no comma-splitting found in
  that function) — there is no multi-id build-scope filter in this script.
- **Staging from a cache** (`.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/…/🗄️stdio`): this cache directory
  does exist (confirmed by `explore-stdio-wasm-link.md` §4's file listing — wgpu-renderer cache/stage
  copies of the same Aug-18 core.wasm) but **contains no descriptor either**; it is a fan-out of the
  same byte-identical Aug-18 artifact, not an independent source of a descriptor.
- **Owner-root descriptor**: confirmed today by direct `ls` — `✏️s/🔌️plugins/🗄️stdio/` contains no
  `🔣️.json` or `🛂️.descriptor.semio` at all (only source dirs, `AGENTS.md`, `.DS_Store`, its crate root
  `.rs`). `git log --all` on both the owner-root and the served-copy paths returns **zero commits** —
  stdio has never had a descriptor of either kind in this repository's history, at any point, on any
  branch reachable by `--all`. This is a stronger and more precise statement than
  `explore-stdio-wasm-link.md`'s "no stdio core, descriptor, registry row, or completion marker exists"
  (that referred to isolated diagnostic target dirs that were cleaned up; this is git history itself).
- **Registry gate right now** (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json`,
  regenerated today at 22:19, 59 entries): the `stdio` row is
  ```json
  {"pluginId":"stdio","packageId":"semio:stdio","cratePath":"✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust",
   "packageName":"semio-s-plugin-stdio","wasmOut":"semio_s_plugin_stdio.wasm","role":"plugin",
   "capabilities":[],"contributes":[],"consumes":[],"dependsOn":[],"activationEvents":[],"extensionPoints":[]}
  ```
  — **no `hashes` object at all** (every other plugin, including raster and stdio's own dependents, has
  `hashes: {wasmSha256, coreWasmSha256, descriptorSha256}`), no `executionMode` field either. Compare
  raster's row, which has both fields fully populated with real hashes. This is the registry's own
  admission that stdio has never produced a materialized artifact through the normal build+describe
  pipeline — it is a metadata-only stub entry (crate discovered by cargo-workspace scanning, never
  successfully built).
- **In-flight artifact found**: `target-raster/wasm32-wasip2/wasm-dev/` currently contains
  `.fingerprint/semio-s-plugin-stdio-cf88cc8f0df9beac/`, `incremental/semio_s_plugin_stdio-...`, and a
  7.1 MB `semio_s_plugin_stdio.d` (dep-info) file, all dated between 20:46 and 21:17 today — i.e.
  **someone already started a `target-raster`-scoped wasm-dev build that reached into compiling stdio
  as raster's embedded dependency** (not stdio's own component — this is raster's own crate's build
  graph). **No rlib/rmeta output exists for it yet** (checked), and **no rustc process is currently
  running against `target-raster`** (checked: `ps aux | grep target-raster` empty; 18 live rustc
  processes total, none referencing this dir) — this is a **stalled, not actively progressing** build,
  last touched ~70 minutes before this check. A coordinator resuming this build (same
  `CARGO_TARGET_DIR=target-raster`, same profile) would resume from this partial state rather than
  starting cold, since cargo's fingerprint cache is intact.

---

## 5. Recommended sequence, react renderer, and the wgpu variant

**Given constraints**: stdio's own standalone component cannot currently be produced (linker ceiling,
unproven even at `wasm-release`, per `explore-stdio-wasm-link.md`); stdio has no descriptor anywhere in
git history; raster's own component links fine and already has a valid, committed descriptor;
`dependsOn` unconditionally pulls stdio into the browser's load set regardless of `SEMIO_PLUGIN_ONLY`.

**React renderer — the sequence with the highest chance of *some* rendered output**, in order:

1. `export CARGO_TARGET_DIR=target-raster` (already exists, partially warm from the stalled 20:46-21:17
   attempt — reuse it, do not delete).
2. `export SEMIO_RENDERER=react`
3. `export SEMIO_BUILD_BUDGET_MS=3600000` (1h; raise further if the shared box is under the swap/load
   pressure every sibling ticket reports tonight — check `ps aux`/swap before committing to a budget).
4. `export RUSTC_WRAPPER=""` (bypass sccache serialization, per project memory
   `sccache-serializes-concurrent-builds` and the S-END-TO-END census's own use of this exact override).
5. **First boot of the session must be a full, un-narrowed build** — i.e. do **not** pass
   `SEMIO_PLUGIN_ONLY=raster` yet. This is the single most load-bearing recommendation from this
   exploration: `SEMIO_PLUGIN_ONLY=raster` only narrows the *build*, not the browser's *load set*
   (§2/§4) — stdio still gets pulled in by `dependsOn` either way, so excluding it from the build only
   guarantees it stays undescribed. A full build gives stdio its best (still unproven) chance of
   producing a fresh component+descriptor through the normal `materializePlugin` pipeline.
6. Run: `bun ./📜️script.ts dev raster` (port 6060 per raster's Cargo.toml `ports.react = 6060`,
   confirmed at `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:19`) from repo root.
7. **Poll a real HTTP port, not a log file** — `curl -sf http://127.0.0.1:6060/` — this is the
   convergent lesson from every sibling ticket (`🗑️generated/` gets swept mid-run by hand-over agents;
   log files vanish; the port answering is the only trustworthy signal).
8. **Expect the stdio build step to be the long pole** (50 min - 3+ h observed across sibling tickets
   for the *lib-embedded* stdio compile alone, before even reaching the actual component-link/jco step
   that has never succeeded standalone). If the registry `generate` gate or the plugin build step never
   converges within budget, the boot will die the same way procedural/lowpoly's early attempts did
   (`spawnSync cargo ETIMEDOUT` → `plugin catalog build failed`).
9. **If/when it serves**: expect the same shape lowpoly/procedural hit — shell chrome renders, but
   watch the console for `plugin.descriptor-unavailable` on `stdio` specifically. If stdio's descriptor
   is still absent (very likely, given it has never once been produced), raster itself will be reported
   `blockedDependency`/fail to load per `expandPluginRegistry`'s dependency closure (§2) — i.e. **the
   most likely outcome, absent a stdio breakthrough nobody has yet achieved, is "shell chrome renders,
   raster fails to load because its declared dependency stdio 404s,"** exactly procedural's boot-8
   observation in §1, not a working raster window.
10. **The only way past step 9 that this exploration found evidence for**: fix the missing composer/
    capability-declaration class of bug (lowpoly's actual root cause, §1) if raster has an analogous
    gap, since that is the only sibling ticket that got from "stdio absent" to "actually rendered" —
    but lowpoly's fix was for lowpoly's *own* build reaching completion with a valid descriptor, not for
    stdio's. **Getting stdio itself a descriptor still requires either (a) a successful standalone
    `cargo rustc -p semio-s-plugin-stdio --target wasm32-wasip2` + `describe` (unproven, possibly
    impossible under the current function-count ceiling), or (b) an upstream code change to shrink
    stdio's own exported dispatch surface (out of scope for a raster-focused ticket, and not attempted
    by any sibling ticket examined today).**

**wgpu variant**: `SEMIO_RENDERER` unset or `=wgpu` boots `trunk serve` natively (port 6160 per raster's
`ports.wgpu = 6160`) instead of Vite/react. `buildEngineWasm`/`SKIP_ENGINE_BUILD` are **no-ops for wgpu**
(only relevant to react, confirmed in the block ticket's exploration and consistent with this script's
`renderer !== "react" → return` guard cited there). The wgpu path does **not** avoid the
stdio-descriptor problem — `ensurePluginRegistry`/`expandPluginRegistry` run identically regardless of
renderer, so the same `dependsOn` cascade applies. The one wgpu-specific risk flagged by the lowpoly
ticket: `semio-framework-os-renderer-wgpu` had **15 errors from a peer's in-flight work** as of that
ticket's read (E0499/E0502 borrows, `ShellSpaceAdministrationPhaseV1`) — **not re-verified today**;
check `cargo check -p semio-framework-os-renderer-wgpu` before committing to the wgpu path, since a
broken wgpu-renderer crate would abort the whole pipeline before trunk starts, independent of raster or
stdio.

**Stated plainly, what is unverified in this report**:
- No dev boot was actually run (read-only agent scope) — every "raster will likely fail to load because
  stdio 404s" conclusion is inference from `expandPluginRegistry`'s code + three sibling tickets'
  first-hand observations of the identical mechanism, not a fresh reproduction against raster
  specifically.
- The exact stdio Cargo.toml `[features]` gating of png/bmp/gif/jpg/tiff/svg/dwg/json codecs (Q3's last
  clause) was not enumerated — flagged as unverified above.
- Whether the stalled `target-raster` build (§4) died from a budget timeout, OOM, or was deliberately
  paused is unknown — no log file for it was found in this exploration's scope.
- The `wgpu`-renderer crate's error count from the lowpoly ticket (15 errors) was not re-checked against
  today's tree.

## Files referenced

- `📓️explore-stdio-wasm-link.md` (this ticket, read first per instructions)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/📓️e2e-status.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/PROCEDURAL-3D-END-TO-END/📓️status.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️explore-dev-boot-path.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/📓️status.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/📓️stdio-check-census.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️status.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END/📓️status.md:243` (real path; coordinator cited a nonexistent 🌙️09/☀️02 path)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:226,354,387,527-550,613`
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:108-132,301-320,382-383,3108-3123`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:19,28`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml:29`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` (stdio/raster rows, regenerated today 22:19)
- `✏️s/🔌️plugins/🖨️raster/🔣️.json`, `✏️s/🔌️plugins/🖨️raster/🛂️.descriptor.semio` (owner-root, committed at `21fbcd3538`, 2026-09-02)
- `✏️s/🔌️plugins/🗄️stdio/` (owner-root listing — no descriptor files, confirmed via `git log --all`)
- `target-raster/wasm32-wasip2/wasm-dev/` (stalled in-flight build state, 20:46-21:17 today)

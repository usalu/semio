# W5 — Host-handle ambient-reach detector (`host-handle-lint`)

Scope: one new lint, `HostHandleReachLintScript`, detecting plugin code that holds a handle to
**host-owned engine/compute state** — a gap `PolicyRulePluginPurity` (repo-root `📜️script.ts`, not
touched here) and `PluginCapabilityLintScript` (this file, pre-existing) cannot see because both are
mutability-shaped rules, and this violation is not about mutability. Report-only, standalone, never
wired into `verify`/`plugin lint`. **Fixing the two live findings is deliberately NOT done here** —
out of this agent's boundary (no `✏️s/🔌️plugins/**` edits) and, per the ticket brief, cross-session
work regardless (`process3d` belongs to another session; `cad` belongs to this ticket but the real
fix reaches `💻️os/🖥️host`'s trait model).

## The insight this rule encodes

`PolicyRulePluginPurity` bans ambient mutable state in plugins but deliberately exempts bare
`OnceLock`/`OnceCell`/`LazyLock` as write-once-by-type — every artifact's `io_registry` uses
`static ENTRIES: OnceLock<Vec<ComposerEntry>>`, and flagging those would drown the signal in noise.

But `OnceLock<Vec<ComposerEntry>>` and `OnceLock<BrepEngineHost>` are identical in mutability shape
and entirely different in violation:

- `OnceLock<Vec<ComposerEntry>>` — a plugin caching **its own immutable data**.
- `OnceLock<BrepEngineHost>` — a plugin holding a **handle to host-owned engine state** for the
  process lifetime.

It is not ambient *mutability*, it is ambient **reach**. The `OnceLock` makes the handle unforgeable
after init and does nothing about a plugin having one at all. So this is a **distinct** check, not a
widened mutability rule — widening the mutability rule would only manufacture false positives against
the sanctioned registry tables.

## What was added

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`

- `//#region 🔖️HostHandleReachLint` (inserted directly after `//#endregion 🔖️PluginIndexExportPathLint`
  and before `class TestScript`):
  - `HOST_ENGINE_HANDLE_TYPES` — the derived handle-type list (see below), each entry documenting
    *why* it's a host handle.
  - `HOST_HANDLE_STATIC_PATTERN` / `HOST_HANDLE_FIELD_PATTERN` / `HOST_HANDLE_CONSTRUCT_PATTERN` — the
    three rule regexes (static of any wrapper, struct field, direct `Type::new(`). The field pattern
    has a `(?!::)` guard so a struct-literal initializer line (`host: BrepEngineHost::new(...)`)
    counts once, as a construction site, not twice (also as a spurious field-decl hit).
  - `scanHostHandleReach(relPath, source)` — runs all three patterns over one file's text, tagging
    each hit with its 1-based line number via `lineNumberAtIndex`.
  - `class HostHandleReachLintScript extends BundleScript` — walks every `.rs` file under each
    `✏️s/🔌️plugins/<plugin>/` directory (via the pre-existing `walkRustSources` helper, already used
    by `PluginCapabilityLintScript`), groups breaches per plugin, prints one `console.warn` line per
    breach naming the plugin, file:line, which rule fired, which handle type, why it's a violation,
    and the ambient-reach-not-mutability framing — actionable without reading this ticket. `run()`
    never throws; ends with a `[DEBUG]` summary line with totals, same non-blocking posture as
    `PluginIndexExportPathLintScript`.
- Router: `.register("host-handle-lint", HostHandleReachLintScript)`, added right after
  `.register("index-lint", PluginIndexExportPathLintScript)`, with a comment explaining why this one
  is also not folded into `plugin lint`/`verify`.

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json`

- Added a `"host-handle-lint"` nx target, byte-for-byte the same shape as `"index-lint"`.

## How the handle-type list was derived

Inspected `🧰️framework/🔨️modules/🧊️3d/` (`BrepEngineHost`, `BrepKernel`, `GeometryHandle`) plus the
framework's `EngineHost`/`Engine` pattern and every other `*Host` struct reachable from plugins.

**Included (2 types):**

- **`BrepEngineHost`** (`🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs:84`) —
  `grep -rl "impl EngineHost for" 🧰️framework` confirms this is the **only** concrete
  `semio_framework_os::engine::EngineHost` trait impl in the entire framework today. Its own doc
  comment: "🖥️ OS EngineHost surface for brep: plugins hold handles and call through here — never a
  private kernel registry." It wraps `Mutex<EngineCache>` (byte-budgeted) + `Mutex<Brep>` (kernel
  session) — a process-lifetime handle to host-managed compute-cache/dispatch authority. Plugins are
  meant to reach it only through the WIT `engine-derive`/`engine-read` guest↔host boundary, never by
  holding the struct directly.
- **`EngineCache`** (`🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs:86`, doc comment
  "Host-owned LRU engine result cache with a byte budget") — the cache every `EngineHost` impl wraps.
  Holding one directly bypasses `BrepEngineHost`'s wrapper but reaches for the identical host-managed
  caching/dispatch authority, so it gets the same treatment.

**Deliberately excluded, with reasoning:**

- **`EngineHandle` / `EngineKey` / `GeometryHandle`** — plain opaque handle *values*, not connections
  to host state. `EngineHandle`'s own doc comment: "Opaque handle returned by derive — plugins may
  store and read, never mint." `GeometryHandle` is `pub struct GeometryHandle(pub String)` — an id
  wrapper, structurally identical to the sanctioned `ComposerEntry` pattern. These are exactly the
  "plugin caching its own immutable data" side of the insight, not the "reach" side.
- **`Brep`** (the `BrepKernel` impl) — constructed fresh per compute call
  (`BrepDocumentOpEngine::compute` does `let mut kernel = Brep::new();` locally, never held ambient) —
  ordinary guest-owned working state, not a host connection.
- **`FlowHost` / `DagHost` / `GraphHost` / `MapHost` / `RasterHost` / `EditorHost` / `BoardHost`** —
  all `*Host` structs that name a per-document/per-session **domain compute model the plugin owns
  outright**, rebuilt fresh from a fixture/snapshot on each call (e.g. `FlowHost`'s own doc comment:
  "Rebuilds the stateful `FlowHost` from the document projection..."). None wraps a cache, arena,
  byte budget, or connection to shared process-lifetime state — they're the "plain data struct" side
  of the ticket's own test, just named `*Host` for the canvas/editing-session metaphor, not the
  process-host-authority sense `BrepEngineHost` uses.
- **`ArtifactHost` / `SpaceHost` / `PluginHost` (os) / `BackboneWorkerHost` / `WasmtimeNodeHost`** —
  real host-authority types (own `Arc<Mutex<HashMap<...>>>` document registries, wasmtime runtime
  maps, an `io_router`), but all live inside the `semio-framework-os` crate, which
  `PluginCapabilityLintScript`'s `depRules` already blanket-forbids as a plugin dependency (with its
  own grandfather list for 17 legacy crates). A plugin reaching these is already caught — coarser (a
  whole-crate ban, not a per-handle explanation) but caught. Adding them here would be redundant
  noise, not a new gap.
- **`NativeHost` / `WasmHost` / `TestHost`** — confirmed by inspection to be a **same-named but
  unrelated** generic `Machine`-parameterized actor abstraction that `🖍️draw` and `🧩️puzzle` **define
  locally themselves** inside their own `🔄️fsm`/`🌉️wasm` modules
  (`✏️s/🔌️plugins/🖍️draw/🔄️fsm/🦀️component.rs:38,2079`) — not a reference to any OS host type at all.
  A bare name match on these would false-positive against legitimate plugin-owned code; this was the
  main reason a structural (trait-impl-based) rather than purely-textual derivation mattered here.

## Findings (real run output)

```
[host-handle-reach-lint] WARN 🏭️process: ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:403: struct field of handle type BrepEngineHost — host-owned EngineHost impl for brep (byte-budgeted engine-result cache + kernel session) — a process-lifetime handle to host-managed compute state, not a plugin's own data (ambient REACH into host-owned state, not ambient mutability — a wrapping OnceLock/LazyLock only makes the handle unforgeable after init, it does not gate having one at all)
[host-handle-reach-lint] WARN 🏭️process: ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:415: direct construction of handle type BrepEngineHost — host-owned EngineHost impl for brep (byte-budgeted engine-result cache + kernel session) — a process-lifetime handle to host-managed compute state, not a plugin's own data (ambient REACH into host-owned state, not ambient mutability — a wrapping OnceLock/LazyLock only makes the handle unforgeable after init, it does not gate having one at all)
[host-handle-reach-lint] WARN 📐️cad: ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:93: static of handle type BrepEngineHost — host-owned EngineHost impl for brep (byte-budgeted engine-result cache + kernel session) — a process-lifetime handle to host-managed compute state, not a plugin's own data (ambient REACH into host-owned state, not ambient mutability — a wrapping OnceLock/LazyLock only makes the handle unforgeable after init, it does not gate having one at all)
[host-handle-reach-lint] WARN 📐️cad: ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs:94: direct construction of handle type BrepEngineHost — host-owned EngineHost impl for brep (byte-budgeted engine-result cache + kernel session) — a process-lifetime handle to host-managed compute state, not a plugin's own data (ambient REACH into host-owned state, not ambient mutability — a wrapping OnceLock/LazyLock only makes the handle unforgeable after init, it does not gate having one at all)
[DEBUG] host handle reach lint: 4 breach site(s) across 2 plugin(s) — REPORT ONLY, does not gate (26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE); fixing is cross-session work (process3d is another session's, cad is this ticket's, the trait model reaches 💻️os/🖥️host) and is deliberately not attempted here
```

Both required findings are present at the correct sites:

- `cad`'s `OnceLock<BrepEngineHost>` — caught as a **static** hit at line 93
  (`static HOST: OnceLock<BrepEngineHost> = OnceLock::new();`) *and* a **construct** hit at line 94
  (`HOST.get_or_init(|| BrepEngineHost::new(CAD_BREP_CACHE_BUDGET_BYTES))`) — two distinct real facts
  about the same site, not a duplicate.
- `process3d`'s struct field — caught as a **field** hit at line 403 (`host: BrepEngineHost,`) and a
  separate **construct** hit at line 415 (`host: BrepEngineHost::new(64 * 1024 * 1024),`). The field
  pattern's `(?!::)` guard confirmed working: line 415 is NOT also double-counted as a field hit.

No other plugin triggered the lint — confirmed with a pre-check
(`grep -rn "BrepEngineHost\|EngineCache" ✏️s/🔌️plugins/`) before writing the rule: these two sites are
the only `BrepEngineHost`/`EngineCache` references anywhere under `✏️s/🔌️plugins/` today.

## Verification — real output pasted

### `bun ./📜️script.ts host-handle-lint` (direct, from the package dir)

Full output: `w5-host-handle-lint-direct.txt` in this folder. `EXIT_CODE=0`. Content matches the
findings block above exactly.

### `bun nx run @semio-tech/framework-os-dev:host-handle-lint`

Full output: `w5-host-handle-lint-nx.txt`. `EXIT_CODE=0`. Tail:
`NX   Successfully ran target host-handle-lint for project @semio-tech/framework-os-dev`.

### Proof it does not change `plugin lint`'s pass/fail

`bun ./📜️script.ts nx run @semio-tech/framework-os-dev:plugin lint` (the repo-root wrapper invocation
the earlier W2 wave verified with — `bun nx run ...:plugin lint` doesn't resolve as a single nx target
name because `plugin` is the target and `lint` is a forwarded arg) after adding `host-handle-lint`:
full output `w5-plugin-lint-after.txt`. Still fails with:

```
error: plugin capability lint failed (69 issue(s), 59 plugin package(s) evaluated)
```

**Identical** 69-issue count to the baseline `📓️w2-lint-report.md`/`📓️w2-dead-export-paths-report.md`
documented (traced to UCAS's in-flight stdio rollout, unrelated to this wave). `host-handle-lint` is
not called anywhere in the `plugin`/`lint` router path or `VerifyScript` — confirmed by grep, the
class name `HostHandleReachLintScript` appears exactly twice in the file: its own definition and the
`host-handle-lint` registration.

## Scope confirmation

```
$ git diff --stat -- "🧰️framework/…/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts" \
                     "🧰️framework/…/🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json"
 .../📋️project.json | 12 ++
 .../📜️script.ts    | 135 +++++++++++++++++++++
 2 files changed, 147 insertions(+)

$ git diff --stat -- "📜️script.ts"     # repo-root script — empty, not touched
(no output)
```

`git status --porcelain -- "✏️s/🔌️plugins"` shows unrelated concurrent-session churn (renames under
`🌀️procedural`, deletions/edits under `➗️mathematical`, an `✒️writer` snapshot edit) — none of it made
by this agent; no file under `✏️s/🔌️plugins/**` was opened or edited by this wave.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` — added
  `HostHandleReachLintScript` + helpers (region `🔖️HostHandleReachLint`) and the `"host-handle-lint"`
  router registration.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json` — added the
  `"host-handle-lint"` nx target.
- This report.
- Scratch: `w5-host-handle-lint-direct.txt`, `w5-host-handle-lint-nx.txt`, `w5-plugin-lint-after.txt`
  in this ticket folder (verification output pasted above).

Nothing else. No file under `✏️s/🔌️plugins/**`, and repo-root `📜️script.ts`, were opened or edited by
this wave.

## Explicit note on scope: fixing is NOT done here

**Neither finding is fixed by this wave.** `process3d`'s `host: BrepEngineHost` field belongs to
another concurrent session (this agent's boundary forbids `✏️s/🔌️plugins/**` edits entirely). `cad`'s
`OnceLock<BrepEngineHost>` belongs to this ticket (APA), but the real fix isn't a local edit either —
it requires routing brep compute through the WIT `engine-derive`/`engine-read` guest↔host boundary
`BrepEngineHost` exists to gate, which reaches into `💻️os/🖥️host`'s trait wiring (how/whether
`EngineHost` gets registered and dispatched for a real running host) — new runtime infrastructure, not
mechanical cleanup. This wave adds only the detector.

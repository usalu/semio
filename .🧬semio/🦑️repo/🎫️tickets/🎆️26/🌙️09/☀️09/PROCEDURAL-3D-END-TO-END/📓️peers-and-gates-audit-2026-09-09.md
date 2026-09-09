# Peers And Gates Audit — 2026-09-09 14:20 CEST snapshot

Read-only survey for `PROCEDURAL-3D-END-TO-END` (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`).
No source edited, no builds run, no git state-modifying command executed. All times relative to
snapshot time 14:20 CEST; re-check before trusting "N min/h ago" later.

## 0. Method note — auto-commit defeats per-file commit attribution

The repo auto-commits periodically and each commit bundles **every concurrent session's** changes
repo-wide (commit message dates are a frozen template — confirmed, ignored; used `--date=iso`).
Sampled: the last 9 commits (36h window) each touch **3,380–19,231 files** repo-wide and **25–506**
files under `✏️s/🔌️plugins/🌀️procedural` alone. So "commits per file" cannot distinguish peers — used
instead: (a) open-ticket `📓️`/`📌️`/`🎫️ticket.json` mtimes in the last 6h, (b) ticket content grep for
`procedural`/`generation3d`/`generation2d`, (c) live `ps` process inspection, (d) `lsof` on shared
`target*/.cargo-lock` files. Only **2** auto-commits landed in the last 12h (`9b605a4550` 12:10,
`599a5d8450` 07:58); §4 below reports files present in **both**, not a ≥3 count.

## 1. Peers — live tickets touching our scope

All `status: "open"` under `🎆️26/🌙️09`, cross-referenced against recent-mtime + content grep:

| Ticket | Fleet | Overlap with procedural-3d scope | Risk |
|---|---|---|---|
| `2026/09/08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES` | GPT-6 Astra coord / GPT-5.6 Sol exec / GPT-5.6 Terra audit | **Direct.** Its own file ledgers (`📓️coordinator-owned-files.md`, `📓️combined-file-ledger.md`, `📓️artifact-execution-file-ledger.md`) explicitly list `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`, `📜️script.ts`, `🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` (plugin root), `🗿️artifacts/🧩️assembly/🦀️.rs`, and ~15 `generation2d` editor/config/command `.rs` files (2026-09-09 09:44 mtime = active). Union ledger just refreshed at **2,805 source paths** repo-wide, described as "not the final close request." Still mid-repackaging Cargo/Nx/Bun for every stdio artifact incl. procedural. | **High** — Cargo.toml, package script.ts, and plugin-root/generation2d files are shared write targets. |
| `2026/09/02/PUZZLE-3D-END-TO-END` | Claude session(s) | **Sibling, same goal** (`R26-02/RUNNING-SKETCHPAD`), no direct file overlap this snapshot (own `📓️2026-09-08-peers-and-gates.md` also found "no file overlap seen"). Extremely active right now (wave B/M/P2/P3/R2/T notes all <2h old; own status.md, remaining-test-failures audit, runtime-verification note all touched). | **Low** direct, but shares `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**`, renderer, and root `📜️script.ts` — same infra churn risk as noted below. |
| `2026/09/02/COMPLETE-SEMIO-END-TO-END` | Opus 5 / GPT-5.6 Sol+Terra | Broad umbrella; 12 notes touched in last 6h (browser-session, renderer-type-frontier, session-authority, space-administration…). Live processes below show it running `framework-renderer-react:test-long` and `framework-os-dev:test-long` (rollup inspection) against its own private target/cache dirs. | **Medium** — shares `framework-os-dev`/renderer test surface with our dev-module scope; isolated via private dirs so low build-lock risk. |
| `2026/09/09/NX-COMPLETE-TASK-CACHING` | — | Opened today, touches Nx caching policy for every `project.json`/target repo-wide (build/test/lint/verify/schema/stdio/cpp). No file list yet (plan doc only). | **Medium** — any `project.json`/`📜️script.ts` we touch could get cache-policy edits underneath us; re-check before close. |
| `2026/09/08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS` | GPT-6 Astra/Sol/Terra | Rewriting `ArtifactViewer`/`ActionMeta`/`ViewModel` plugin-runtime plumbing (`🔨️modules/🔌️plugin/**`, `📡️spr/**`, `📺️renderer/**`) — per puzzle3d's own audit, live edits were in progress against this shared infra as of 2026-09-08 23:46. | **Medium** — if generation3d's editor/viewer implements `ArtifactViewer`, expect the same trait-signature churn underfoot. |
| `2026/09/08/SCOPE-OWNED-SCHEMA-CONTRACTS` | Fable 5.1 / Opus 5 / Sonnet 5 | Repo-wide schema-contract relocation under `🧬️schema/**`; puzzle confirmed in scope via its "plugins wave". Procedural's `generation3d`/`generation2d` `🧬️schema/**` leaves are plausible targets too — not confirmed touched this snapshot, but same wave shape. | **Medium** — mutation `$id`/location could move under any plugin's `🧬️schema/`. |
| `2026/09/08/COMPOSABLE-STDIO…`'s siblings closed today (`RESOLVE-COLLIDED-COMPONENT-JSON…`, `TRANSCRIBE-RICH-ARTIFACT-GRAMMARS`, etc.) | — | Closed, no longer live. | none |

## 2. Live processes right now (ps, snapshot)

Large concurrent fleet. Notable hits on our scope or shared infra:

- **`cargo check -p semio-s-plugin-procedural --keep-going`** (pid 5123, running 01:40) — **someone is
  actively type-checking the exact plugin this ticket owns, right now.** Cannot tell which peer/target-dir
  from `ps` alone (no `CARGO_TARGET_DIR` visible in its own argv — check its parent shell env before
  assuming it shares the root `target/`).
- `cargo rustc -p semio-s-plugin-puzzle --lib --crate-type cdylib --target wasm32-wasip2 --profile
  wasm-release` (pid 80176) and `cargo test -p semio-s-artifact-puzzle-3d --features
  component-app-assembly` (pid 81650) — puzzle-3d's own wasm build + test, sibling plugin, adjacent
  crate space.
- `cargo test -p semio-s-artifact-block-3d` (pid 78819) — another `-3d` artifact plugin, unrelated tree.
- `COMPOSABLE-STDIO-ARTIFACT-PACKAGES` runs a large parallel batch (gis-plugin test, block-plugin test,
  stdio-binary-rs/stdio-txt-rs test, flow-plugin child-identity/child-edit/add-widget checks, a
  `semio_framework`/`semio_framework_os_kernel` rlib rebuild) **entirely inside its own ticket-scoped
  `🗑️generated/cargo` `CARGO_TARGET_DIR`** with `CARGO_BUILD_JOBS=2`, own `NX_CACHE_DIRECTORY` /
  `NX_WORKSPACE_DATA_DIRECTORY` — good isolation precedent to copy.
- `COMPLETE-SEMIO-END-TO-END` runs `nx run @semio-tech/framework-renderer-react:typecheck` and
  `:test-long` (space-administration) and `framework-os-dev:test-long` (rollup inspection) — also into
  its own ticket-scoped `🗑️generated/hub-target`/`nx-emitted-browser-build-cache` dirs.
- Many bare `cargo test -p <crate> --manifest-path Cargo.toml` processes with **no `CARGO_TARGET_DIR`
  override** (writer-plugin, `semio-framework-plugin`, `semio-s-artifact-trinity-jack`,
  `semio-s-artifact-reasoning-wires`, `semio-framework-os-kernel` ×3, `semio-hub`, `semio-s-artifact-flow-flow`,
  `semio-s-artifact-note-note`, `neural_engine`/`semio-framework-os-kernel` sync build under
  `ZERO-WARNINGS-ZERO-ERRORS…`'s private `derive-target`) — these default to the **root** `target/`.
- A `vite` dev server is already up on `127.0.0.1:6013` (pid 67338).
- `bun x nx run @semio-tech/plugin-registry:...`-style activity not observed this snapshot; no process
  currently regenerating `.vscode/launch.json`.

**Lock check**: `lsof` on every shared `target*/{debug,wasm-dev,wasm-release,wasm32-wasip2/*}/.cargo-lock`
found **no open handle** right now — the root target dirs are not currently flock-held, but given the
volume of bare `cargo test -p …` processes above, that can flip at any moment. **Recommendation: give
the procedural-3d fleet its own `CARGO_TARGET_DIR` (ticket-scoped, e.g.
`$T/🗑️generated/cargo`) exactly like COMPOSABLE-STDIO and COMPLETE-SEMIO do**, rather than sharing root
`target/` with the dozen+ bare processes above.

## 3. Gates (root `📜️script.ts`, confirmed by reading the source, not assumed)

Root `📜️script.ts` registers exactly: `os, semio, examples, setup, start, dev, generate, new, schema,
lint, verify, format, test, bench, stdio, build, cpp, publish, purge, clean, micro-commit, commit`.
There is **no `layer-lint` or `index-lint` subcommand** anywhere in `📜️script.ts` (grepped both literal
strings, zero hits) — `lint` has exactly two forms: `lint repo` (Nx-orchestrated, no-op log) and the
default `bunx dependency-cruiser 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .dependency-cruiser.cjs
--output-type err`. Treat "layer-lint/index-lint" as not-a-real-gate-name in this repo; the actual
layering gate is `dependency-cruiser` plus `verify` subcommands below.

Confirmed gate commands (source: `INTERACTIVITY_ALL_APP_REQUIRED_GATES` array at
`📜️script.ts:8258-8264`, plus `VerifyScript.run` dispatch at `📜️script.ts:7091+`):

| Gate | Command | Notes |
|---|---|---|
| `⚖️gate📦️dependencies0️⃣` | `bun ./📜️script.ts verify dependencies literal-external` | **Red-until-zero.** Fails with `target=0, current=<n>` if any literal third-party runtime dep exists outside the allowed oracle/toolchain exceptions. This is the "Dependency Truth Gate" from prior memory. |
| `⚖️gate📦️dependencies` | `bun ./📜️script.ts verify dependencies` | Unrestricted report/summary form (`summary`/`list`/`self-test`/`literal-external` are the `args[0]` branches at line 7876). |
| `⚖️gate⚡️interactivity` | `bun ./📜️script.ts verify interactivity` | Deny-mode sweep: all-app discovery/launch registration, unlisted blocking bridges, stale allowlist entries, sync-fs/net/clipboard/process/db/thread-pool findings. |
| `⚖️gate⚡️interactivity🎯️tool-jobs` | `bun ./📜️script.ts verify interactivity tool-jobs` | — |
| `⚖️gate⚡️interactivity🧭️apps` | `bun ./📜️script.ts verify interactivity apps` | Includes the `.vscode/launch.json` ↔ `🧩️launch.seed.jsonc` ↔ `🎮️playgrounds.ts` coverage check (`interactivityAllAppLaunchCoverageFailures`, `📜️script.ts:8429`) — proves every owner-qualified app has React + WGPU-Wasm + WGPU-native launch surfaces. **Directly relevant**: generation3d's play harness needs all three launch variants registered or this gate fails. |
| `⚖️gate⚡️interactivity🧭️apps🎛️actions` | `bun ./📜️script.ts verify interactivity apps --actions` | — |
| taxonomy registry | `bun ./📜️script.ts verify taxonomy [inventory\|plan\|apply\|verify]` | Validates `taxonomy.json` (owned at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`) against the live tree; `TaxonomyCliOperation`. |
| rust-warnings | `bun ./📜️script.ts verify rust-warnings [target]` | `native \| wasm32-wasip2 \| wasm32-unknown-unknown`; scope includes every `semio-s-artifact-*`/`semio-framework-artifact-*` plugin/extension crate via `pluginCrateNames()`. |
| other `verify` leaves confirmed present | `verify mutation-outcome-law`, `verify semantic-vocabulary`, `verify package-purity`, `verify interactivity p1q-b1-b6/p1w/p1x/p1y/p1z/p5d/p5e/p3mn…` | Repo-wide, not procedural-specific, but would still block if broken. |
| Nx-project-scoped gates (NOT on root router) | `bun nx run repo:policy-check`, `bun nx run repo:graph-check`, `bun nx run repo:artifact-check` (route through `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts`) | Reached only via `nx run <project>:<target>`, not `bun ./📜️script.ts <name>`. |
| launch registry | `bun nx run @semio-tech/plugin-registry:generate` (regenerates `.vscode/launch.json` from `.vscode/🧩️launch.seed.jsonc`; cwd `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`) / `bun nx run @semio-tech/plugin-registry:check` (freshness) | **Never hand-edit `.vscode/launch.json`** — edit the seed, regenerate. Confirmed: `.vscode/launch.json` is machine-generated (source: `🖥️launch.ts` docstring, quoted verbatim by puzzle3d's own gates note); CLAUDE.md's launch.json registration-ordering rule applies to the **seed**, not the generated file. Both are currently `MM` in git status (uncommitted local edits) — regenerate and diff-check before relying on either. |

No recent timing/duration log for any of these was found under this ticket's siblings' `🗑️generated`
folders (checked `COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated`, nothing timestamped); puzzle3d's own
gates note (`📓️2026-09-08-wave-G-gates.md`, `📓️2026-09-08-peers-and-gates.md`) documents the **command
list and dependency order** (dependencies0️⃣ → interactivity[+narrower] → dependencies → plugin-specific
publication-authority-audit → repo policy/graph/artifact-check → taxonomy verify → launch-seed
regenerate/check → rust-warnings/semantic-vocabulary/package-purity/layering/abstraction-ownership) but
gives no wall-clock durations; treat all as potentially multi-minute Rust compiles and budget
accordingly rather than assuming cache hits.

## 4. Conventions to honor (quoted, path-specific)

- **`✏️s/AGENTS.md`** (full file):
  > "semio s (semi os) is a collaborative operating system for designers to share and store any kind of
  > design knowledge. It is the ultimate technology that unifyies the complete monorepo."
- **No `✏️s/🔌️plugins/AGENTS.md`** — that file does not exist; nothing to quote at the plugins-root level.
- **`✏️s/🔌️plugins/🌀️procedural/AGENTS.md`** (full file):
  > "Procedural is a [flow-based](../flow/AGENTS.md) editor for [breps](../kernel/3d/AGENTS.md)."
  > "## 🎮️ Play harness — The 2d and 3d play apps (`procedural2d-play`, `procedural3d-play`) are the
  > play harness for this plugin — relocated here from the former plugin-root `🎮️play/` doc stub, which
  > carried no code."
  Note: the `../kernel/3d/AGENTS.md` relative link is **stale** — no `✏️s/🔌️plugins/🌐️kernel/` directory
  exists in the current tree (confirmed via `find`); do not chase it, and consider flagging the dangling
  link separately.
- **Single-file crates / `#[path]` artifact modules**: confirmed live in
  `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` (plugin root) — declares `ProceduralApps` via
  `semio_framework_dispatch_macros::dyn_enum_close!`, builds via
  `Plugin::<ProceduralApps>::builder("procedural")` with `.package_id("semio:procedural")`. Builder id
  (`"procedural"`) and Cargo component metadata (`semio:procedural`) already agree — matches the
  "Plugin Id Drift" convention from prior memory; do not let them diverge.
- **`[package.metadata.semio] depends-on`**: present at `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:14`.
- **Docstrings**: all sampled `.rs` doc comments in the procedural plugin root start with a unique emoji
  (`//! 🔌️ Plugin root contract…`, `/// 🔌️ Builds the plugin surface…`, `/// 🗃️ Closed runtime app
  fleet…`) — consistent with CLAUDE.md's docstring-emoji rule; keep following it verbatim.
- **`📜️script.ts`-only scripts**: procedural already has its own
  `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts` — extend it, do not add sibling script
  files (COMPOSABLE-STDIO's own ledger lists this exact file as one it is currently `updated`-ing —
  coordinate/re-check before editing).
- **launch.json**: generated, not hand-authored — see §3 table.

## 5. Do-not-touch / coordinate-first list

Only 2 auto-commits landed in the last 12h, so a strict "≥3 commits" count is not meaningful (see §0).
Files present in **both** of those 2 commits (i.e., touched persistently across the whole 12h window)
under our target scope — **124 files total**; procedural's own:

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs`

Plus the shared `Cargo.toml` and `📜️script.ts` at `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/` (in
COMPOSABLE-STDIO's explicit owned-files ledger, §1) and the whole `🗿️artifacts/{🌀️generation2d,🧊️generation3d}/…/✏️editor/**`
command tree (also in that ledger — 249–506 procedural-scoped files touched per recent auto-commit).
**Coordinate with COMPOSABLE-STDIO-ARTIFACT-PACKAGES before batch-editing procedural's Cargo.toml,
package script.ts, plugin-root `🦀️.rs`, or the generation2d editor/config/command modules** — that
ticket's own ledger names them as currently-owned/updated files, not just historically touched ones.

## 6. Bottom line for the coordinator

Give the procedural-3d Opus fleet a **private `CARGO_TARGET_DIR`** under `$T/🗑️generated/cargo` (root
`target*/.cargo-lock` files are unlocked right now but heavily contended by bare `cargo test -p …`
processes with no target override) and **coordinate explicitly with COMPOSABLE-STDIO-ARTIFACT-PACKAGES**
before touching `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/{Cargo.toml,📜️script.ts}`, the plugin-root
`🦀️.rs`, or `generation2d`'s editor/config/command tree — that ticket's file ledgers claim exactly those
paths as in-flight. PUZZLE-3D-END-TO-END is a same-goal sibling with no direct file overlap; treat as
background noise but expect the same shared-infra churn (`🔌️plugin/**`, renderer, root `📜️script.ts`)
from CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS and SCOPE-OWNED-SCHEMA-CONTRACTS.

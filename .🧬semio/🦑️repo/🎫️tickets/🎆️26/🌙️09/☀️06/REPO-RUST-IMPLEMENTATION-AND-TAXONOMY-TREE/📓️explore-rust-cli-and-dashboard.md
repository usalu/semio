# Rust CLI/Dashboard exploration

Root paths use `🧰️framework/🛍️products/🦑️repo/…` unless stated otherwise. All source read directly (`Bash`/`cat`), no assumptions.

## 1. `semio` binary command tree

**Crate**: `🔨️modules/⌨️cli/📦️packages/🦀️rust/` — `Cargo.toml` name `semio-framework-repo-cli`, `[lib] path = "🦀️.rs"`, `[[bin]] name = "semio" path = "📦️main.rs"`.

`📦️main.rs` is a one-liner:
```rust
fn main() { std::process::exit(semio_framework_repo_cli::run(std::env::args().skip(1).collect())); }
```

All logic lives in the lib godfile `🦀️.rs` (1142 lines). Commands are NOT `include!`d nor built via `build.rs` — they are separate crate-external files pulled in with `#[path = "…"]` module declarations at the top of the lib, each pointing at `../../../../🎮️commands/<dir>/🦀️.rs` (relative from the crate root back up to `🦑️repo/🎮️commands/`):

```rust
#[path = "../../../../🎮️commands/🌊️workflow/🦀️.rs"]
pub mod workflow;
#[path = "../../../../🎮️commands/🔌️plugin-registry/🦀️.rs"]
pub mod plugin_registry;
#[path = "../../../../🎮️commands/🖥️terminal-dashboard-daemon/🦀️.rs"]
pub mod terminal_dashboard_daemon;
#[path = "../../../../🎮️commands/🛝️playground-development-session/🦀️.rs"]
pub mod playground_development_session;
#[path = "../../../../🎮️commands/⌨️cli-usage-presentation/🦀️.rs"]
pub mod cli_usage_presentation;
#[path = "../../../../🎮️commands/📇️playground-catalog-query/🦀️.rs"]
pub mod playground_catalog_query;
#[path = "../../../../🎮️commands/📜️root-script-delegation/🦀️.rs"]
pub mod root_script_delegation;
#[path = "../../../../🎮️commands/🌳️command-tree-discovery/🦀️.rs"]
pub mod command_tree_discovery;
#[path = "../../../../🎮️commands/🎛️terminal-dashboard/🦀️.rs"]
pub mod terminal_dashboard;
```

Each `🎮️commands/<slug>/` directory holds exactly one `🦀️.rs` (and is registered as a `kind: "command"` member in the sibling `🎮️commands/🔣️.json` manifest — this JSON is metadata/documentation only, it is **not** read at build/run time; nothing in the Rust source parses it).

**Dispatch** (`run()` at the bottom of the lib, region `#region 🔖️Dispatch`):
```rust
pub fn run(argv: Vec<String>) -> i32 {
    let root = workspace::find_root(&std::env::current_dir()...);
    if argv.is_empty() {
        if !std::io::stdout().is_terminal() { cli_usage_presentation::print(); return 1; }
        return terminal_dashboard::run(&root);
    }
    let parsed = args::parse(&argv);
    match parsed.verb.as_str() {
        "daemon" => terminal_dashboard_daemon::run(&root, &parsed),
        "workflow" => workflow::run(&root, &parsed),
        "dev" => playground_development_session::run(&root, &parsed),
        "catalog" => playground_catalog_query::run(&root, &parsed),
        "plugin" if parsed.segments.first()... == Some("registry") => plugin_registry::run(&root, ...),
        _ => root_script_delegation::run(&root, &parsed),
    }
}
```
`args::parse` (region `#region 🔖️Args`) splits argv into `verb` / positional `segments` / `--flag [value]` map; a flag consumes the next token unless it's itself `--flag` or absent.

**Registering a new command** = (a) add a directory `🎮️commands/<slug>/🦀️.rs` with a `pub fn run(root: &Path, parsed: &ParsedArgs) -> i32`; (b) add a `#[path=…] pub mod <slug>;` line to the lib; (c) add a `"<verb>" => <slug>::run(&root, &parsed)` arm in `run()`'s match (only if it needs a reserved top-level verb — otherwise it falls through to `root_script_delegation`, which execs `bun ./📜️script.ts <verb> <segments…>`); (d) add a documentation-only member entry to `🎮️commands/🔣️.json`; (e) update the static `USAGE` string + its literal-matching unit test in `⌨️cli-usage-presentation/🦀️.rs` if the verb is user-facing.

**Dispatch macro pattern** (only used inside `🌊️workflow/🦀️.rs`, NOT in the lib's own verb dispatch, which is a plain `match` on strings): `use dispatch_macros::{dyn_enum, dyn_enum_close};`
```rust
#[dyn_enum]
trait AgentRunner: Send {
    fn id(&self) -> &str;
    fn available(&self) -> bool;
    fn spawn(&self, prompt: &str, cwd: &Path) -> std::io::Result<std::process::Child>;
}
// … 3 concrete structs (CursorAgent, ClaudeAgent, CodexAgent) each impl AgentRunner …
dyn_enum_close! {
    enum AgentRunners: AgentRunner {
        Cursor(CursorAgent),
        Claude(ClaudeAgent),
        Codex(CodexAgent),
    }
}
```
`#[dyn_enum]` (attribute macro) re-emits the trait and stashes its method signatures into a hidden `#[macro_export]`ed `__semio_dispatch_<Trait>!`; `dyn_enum_close!` (function-like macro, different macro namespace on purpose — `dyn_enum`/`dyn_enum_close` cannot share a name, Rust macro namespaces are flat) then generates the enum, `From<Variant>` impls and a delegating `impl Trait for Enum`. This is the repo's standard **closed-set enum dispatch** idiom (R11 ruling: closed set ⇒ enum dispatch, not `Box<dyn Trait>`), documented in `📓️terra-dyn-enum-macro-report.md` and `📓️terra-dedyn-fw-hub-repo-report.md`. Any crate that declares a `#[dyn_enum]` trait must add `#![allow(async_fn_in_trait)]` at its own crate root (dispatch-macros' own root does this; it is NOT propagated automatically to consumers). The daemon's `Supervisor<T: Write + Send>` in the CLI lib deliberately does **not** use `dyn_enum` — it's generic over the transport instead, because the test-only `DuplexEnd` mock and the real `UnixStream` genuinely need to coexist without a `#[cfg(test)]` enum variant (the DSL has no per-variant `#[cfg]`).

**Conventions observed everywhere in these files**: every doc comment (`///`/`//!`) opens with one emoji; `// #region 🔖️Name` / `// #endregion 🔖️Name` fold markers around cohesive blocks; zero comments inside definition bodies (only doc comments above items, occasional single-line `//` explaining a genuinely non-obvious call just before it, e.g. `// attach uses write on the client-facing end we keep in supervisor`); `[DEBUG]`-prefixed `eprintln!` for temporary diagnostics (e.g. `terminal-dashboard/🦀️.rs`'s `spawn_output`/`kill_session`).

## 2. Terminal dashboard

File: `🎮️commands/🎛️terminal-dashboard/🦀️.rs` (619 lines), entered via `semio` with no args on a TTY, or `semio daemon attach`.

**What it shows**: a `ui_tui` (`semio-framework-ui`, `tui-terminal` feature) chrome shell with tiled/split/stacked `DashboardWindow`s. Each window body is either:
- `WindowBody::Wizard { widget, cursor }` — a `WizardState` walking the `CommandNode` tree (breadcrumbs via `wizard_steps`), or
- `WindowBody::Output { terminal, session }` — a `TerminalState` fed by a spawned `Pty` (`ui_tui::tui::pty::{Pty, PtySize}`).

Selecting a leaf wizard option (`WidgetSignal::Activated(i)` where `child.children.is_empty()`) calls `spawn_output`, which `Pty::spawn(&spec.cmd, &args, &env_refs, cwd, pty_size)` — i.e. it forks a **real OS process** (`bun`, `cargo`, whatever `CommandSpec` says) into a PTY, not an in-process call. Leader-key (`Ctrl+Space`) chords do splits/zoom/new-tab/close/toggle-terminal-input (`z`, `-`, `|`, `x`, `t`, `n`), mirroring tmux-ish ergonomics.

**Command discovery** — `🎮️commands/🌳️command-tree-discovery/🦀️.rs` (`discover(root) -> CommandNode`), called once at dashboard startup:
1. Recursively walks the whole repo tree (skipping `node_modules/target/.git/dist/build/generated/cache` and dot-dirs except `.semio`), reading every `📋️project.json`/`project.json`.
2. For each `targets` key found, builds a wizard path `[<target>, …taxonomy segments from the manifest's directory path, filtered to drop noise segments like packages/modules/products/plugins/…]`, with a leaf `CommandSpec { cmd: "bun", args: ["nx","run","<project>:<target>"], cwd: root, env: [] }`. This is **pure nx-target discovery** — no static manifest of commands is consulted, it is a literal filesystem+JSON walk on every dashboard boot.
3. Separately calls `crate::catalog::load_playground_catalog(root)` (reads the **generated** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔣️playgrounds.json`, itself produced by the TS plugin-registry scanner, never regenerated in Rust — "single source of truth stays with the TS emitter") and injects `dev/<plugin>/<variant>/<react|wgpu-wasm|wgpu-native>` leaves that exec `bun nx run @semio-tech/framework-os-dev:dev` with an env built by `env_contract::build_dev_env`.
4. Sorts: depth 0 by a fixed `VERB_ORDER` (`dev, build, test, verify, gate, lint, format, generate, publish`) then alphabetically; deeper levels alphabetically.

There is **no other catalog/registry input** — no ticket list, no goal list, no analyze/tree command surface from a Go or MCP backend.

**PTY windows**: unix-only real implementation (`ui_tui::tui::pty::Pty`); on non-unix the daemon's `spawn_session` just broadcasts a `ServerMsg::Error{"PTY supervisor requires unix tui-terminal"}` — i.e. the daemon's session supervisor is effectively unix-only today (see IPC/daemon section below); the interactive `terminal_dashboard::run` foreground path uses the same `Pty` type unconditionally (no cfg-gate visible in that file), so it likely does not compile/behave usefully for Windows-native PTY spawning without the same unix cfg — worth flagging if the ticket's Windows-native goal touches this file.

**Go / repo-client / MCP / ticket / goal search** — grepped `🎮️commands/**` case-insensitively for `\bgo\b|golang|repo.?client|mcp|ticket|goal`: the **only** hits are inside `🌊️workflow/🦀️.rs`'s own doc comments/variable names (`ticket`-scoped workflow directory, e.g. `root.join(".🧬semio/🦑️repo/🎫️tickets")`, `🌊️workflow.json`) — there is **zero** reference to Go, a Go/MCP repo client, tickets-as-domain-objects (beyond a bare directory path), or goals anywhere in the dashboard, command-tree-discovery, or daemon code. **Today the Rust dashboard/CLI has no integration whatsoever with the Go repo/MCP implementation.**

For contrast, a **separate Go CLI already exists** at `🔨️modules/💻️client/⌨️cli/🐹️.go` (referenced in `🧅️layering.json`'s ratchet list) plus a Go MCP server package at `🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go` (launched via `go run ./…/🔌️mcp/📦️packages/🐹️go` in `.vscode/launch.json` around line 1772, and `go build -o …/client ./…/🔌️mcp/📦️packages/🐹️go` around line 1790) and a VS Code extension client (`🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript`). These are the "repo domain" implementations (tickets, goals, analyze, tree, MCP tools) that presumably back the `repo`/`semio` MCP servers seen failing to connect in this very session. **None of that is wired into the Rust `semio` dashboard binary.**

**What "use the Rust implementation by default for the dashboard" would concretely require** (integration points, today either absent or stubbed):
- A Rust equivalent of whatever the Go CLI/MCP server does for tickets/goals/analyze/tree (currently only in Go) would need its own `🎮️commands/<slug>/🦀️.rs` (or a shared library crate under `🔨️modules/…/📦️packages/🦀️rust`) and then either (a) a new top-level verb arm in `run()`'s match (e.g. `"ticket" | "goal" | "analyze" | "tree"`), or (b) new leaves injected into `command_tree_discovery::discover()` alongside `inject_playground_dev` (a new `inject_repo_domain(...)` walking a repo-domain data source and pushing `CommandSpec` leaves under e.g. `["tickets", …]`), so they show up in the dashboard wizard.
- `catalog::load_playground_catalog` is the existing pattern for "Rust consumes a generated JSON, does not reimplement the producer" — the same shape (`generated_dir`, `*_json_text`, `load_*`) could back tickets/goals if their canonical source of truth stays elsewhere (e.g. TS or Go), or a fully-Rust owner could be built if that ticket intends to retire the Go implementation.
- The **daemon** (`🎮️commands/🖥️terminal-dashboard-daemon/🦀️.rs` + `daemon`/`ipc` modules in the lib) is a persistent background process: `start|serve|stop|status|attach`. `serve` runs `crate::daemon::serve(&root, running)` — an accept loop (`Supervisor<T>`) that fans out PTY session control (`ClientMsg::{Attach,Detach,Spawn,Input,Resize,Kill,Ping}`) and output bytes to attached clients over a Unix socket (unix) or named pipe (windows, `pipe_name` hashes the root path into `\\.\pipe\semio-dashboard-<hash>`) using a length-prefixed frame protocol (`ipc::{write_frame,read_frame,try_decode_frame}`, `KIND_CONTROL`/`KIND_OUTPUT`). Its only job is PTY session lifecycle/multiplexing so `semio daemon attach` can reattach to already-running terminal windows; it has no repo-domain awareness either.

## 3. Rust test runner/protocol (`🔨️modules/🧪️test/`)

- `📡️protocol/🦀️.rs` + `🏃️runner/🦀️.rs` are combined into the published crate `semio-repo-test-host` via `📦️packages/🦀️rust/📦️lib.rs`, which does `#[path="../../📡️protocol/🦀️.rs"] pub mod protocol;` and `#[path="../../🏃️runner/🦀️.rs"] pub mod runner;` — same `#[path]`-module pattern as the CLI. The crate doc explicitly states it is "DOMAIN-NEUTRAL and dependency-free: it knows about plans, results, fixtures and adapters, and about no file format, plugin or product whatsoever."
- `runner::Context` gives a scenario handler: `fixture`/`fixture_bytes`/`fixture_json` (resolved against `Plan::fixture`), `copy_fixture` (mutable copy into `work_dir`), `doc_string`/`doc_json` (feature-owned input vector), `data_table`, `artifact(role, filename)` (writes under `artifact_dir/role/`), `target()` (subset-target scoping, errors if the case declares none), `seed()`.
- `runner::Adapter::new("rust").subject(id, handler)` / `.oracle(id, handler)` registers this implementation's handler per scenario id + role (`subject` = this repo's own implementation under test, `oracle` = reference implementation). `registered(role)` lists what got registered, for the coordinator's completeness check.
- A generated, cache-local integration crate (not committed) links a case's own `🦀️.rs` adapter file and calls `run_main`. The committed `🧪️tests/🖥️host-protocol-parity/🦀️.rs` is exactly this pattern for a self-test case: `pub fn adapter() -> Adapter { Adapter::new("rust").subject("digest-and-fixture-resolution", …).subject(…).subject(…) }` with three `fn(&Context) -> Result<Outcome, String>` scenario handlers, each returning `Outcome::projection(Json::Object([...]))`. This is the "host-protocol-parity" example: it independently re-implements the digest/fixture/work-dir contract against the same frozen spec that 4+ other language adapters implement, to prove pairwise equivalence — the pattern any new Rust-vs-other-implementation parity test should imitate.
- Result JSON (`result_json` in `🏃️runner/🦀️.rs`) is `schemaVersion: 2`, keyed `testId = "<owner>::<case>::<scenario>::<implementation>::<role>"`, carries `rawHash`/`projectionHash`/`projection`, per-artifact `{role, path, mediaType, sha256, bytes}` (every artifact is **re-hashed from disk**, never trusted from the handler), `baselineSha`, `level`, `platform`, `status`.
- **nx / budgeted execution** lives in the TS library, not Rust: `🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`:
  - `resolveTestLevel(segments)` reads `segments[0]` if it names a `TestLevel` (`fundamental|quick|long|exhaustive`? — the four `TEST_LEVELS`), else `SEMIO_TEST_LEVEL` env, else `"fundamental"`; sets `process.env.SEMIO_TEST_LEVEL` so every spawned child (cargo, go, vitest, pytest, dotnet) inherits it; auto-sets `SEMIO_COVERAGE=1` at `exhaustive` unless already set.
  - `runCargoTestBudgeted(packages, cwd, extraArgs, env)`: resolves package names, splits `extraArgs` on a literal `--` into `cargoArgs`/`libtestArgs`, computes `--skip <level>::` filters for every level **above** the active one (Rust tests are organized as `mod tests { mod quick { } mod long { } mod exhaustive { } }` submodules — unscoped tests are implicitly `fundamental`), and:
    - if `SEMIO_COVERAGE` is on: runs `cargo llvm-cov nextest --release --no-report --no-tests warn --profile <level> …` (falls back to `cargo llvm-cov test --release` if `cargo-nextest` isn't installed) then `cargo llvm-cov report --lcov --output-path <coverageDir>/rust/<slug>.lcov`;
    - else if `cargo-nextest` is installed: two-phase — `cargo nextest list --list-type binaries-only --message-format json` writes `binaries-metadata.json` into a `mkdtempSync` dir under `nextestArtifactLocation(cwd,env).directory`, then `cargo nextest run --binaries-metadata <path> --no-tests warn --status-level fail --final-status-level fail …`; assertion thread count is throttled at `fundamental` level (`availableParallelism() - ceil(availableParallelism()/4)`) via `--test-threads`, unthrottled otherwise;
    - else: warns, falls back to plain `cargo build --tests` + `cargo test`.
  - `nextestArtifactLocation(cwd, env)`: `SEMIO_TEST_ARTIFACT_DIR` (trimmed) resolved against `cwd` and `retain: true` if set, else OS `tmpdir()` with `retain: false` (metadata dir is `rmSync`'d after the run unless retained). This is the same `SEMIO_TEST_ARTIFACT_DIR` contract used pervasively across `.vscode/launch.json`'s many `test-*` configs (paired with `CARGO_TARGET_DIR` pointed at a ticket's `🗑️generated/…` subdir) and required/enforced by `🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws` ("Exact Cargo laws require an absolute ticket-generated artifactDir or SEMIO_TEST_ARTIFACT_DIR", must include a `🗑️generated` path segment).
  - The CLI crate's own `📜️script.ts test` subcommand is a thin wrapper: `resolveTestLevel(segments)` then `runCargoTestBudgeted(["semio-framework-repo-cli"], repoRoot, rest)`.

Related discovery/analysis tests found (not part of the runner/protocol, but Rust-source-aware TS test suites under the same `🧪️test`/`📚️library` tree):
- `🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/` — a bun:test file validating `rustDiscovery.inspectRust*` (join-argument spans, manifest-path references, assertion-message spans, module graph) against a golden fixture `🧫️fixtures/🔣️rust-physical-reference-context.json`, exercising retained-ticket-run authority semantics (`retention.parentSegments` under a specific historical ticket dir) — this is static-analysis-of-Rust-source tooling, unrelated to the CLI's own runtime.
- `🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/` — a `🔣️.json` contract (`rust-divergence-error-callback-v1`) plus fixture cases describing exactly how `.unwrap_or_else(|error| panic!(...))`-style divergence callbacks are allowed to look (single immutable identifier, one literal capture, no writable shared labels, max 256 expanded iterations) — a lint-shape contract for a source-analysis tool, again unrelated to CLI runtime behavior.

## 4. Crate / workspace conventions

**Root `Cargo.toml`** (repo root):
- `cargo-features = ["trim-paths"]`; `[workspace] members = [...]` — a very large explicit list (>140 members), `resolver = "2"`.
- `[workspace.package] version = "0.1.0", edition = "2021", rust-version = "1.88"`.
- `[workspace.dependencies]` is a **partially-adopted** alias table: comments explicitly say "Purely additive: no existing member below adopts `.workspace = true` yet" — it exists so a future wave can opt in with one line, or repoint one line when a crate merges/moves. It lists internal path deps grouped by survey count (`# N refs`) plus exactly four external crates with pinned/ranged versions:
  ```toml
  serde = { version = "1.0.228", features = ["derive"] }
  serde_json = "1.0.149"
  wasm-bindgen = "0.2.106"
  tokio = { version = "1" }
  ```
  These are "the highest-fanout external deps, chosen as the newest explicit requirement string already used somewhere in the 630 manifests" — i.e. **serde/serde_json/wasm-bindgen/tokio are the accepted, already-pervasive external runtime deps** across the Rust workspace; nothing in the root manifest calls out regex/sha2/flate2 as workspace-level deps (the test-protocol's `sha256_hex`/`digest` in `📡️protocol/🦀️.rs` are therefore very likely hand-rolled, not via a `sha2` crate — confirm in that file if a port needs hashing). No blanket "no runtime external deps" comment exists in the root `Cargo.toml` itself; that policy is CLAUDE.md-level ("You MUST NOT create runtime dependencies on external libraries… You MUST use all external libraries behind an interface if not feasible to implement in our codebase"), not cargo-manifest-enforced — the repo evidently already accepts serde/serde_json/wasm-bindgen/tokio as pervasive exceptions (real widespread precedent), so a Rust port of Go-domain logic should default to reusing those four rather than introducing new external crates, and put anything else behind an interface per CLAUDE.md.
- `[profile.dev]` `debug=false, incremental=true`; `[profile.dev.build-override] opt-level=3`; `[profile.wasm-dev]` inherits dev with `codegen-units=1`; `[profile.release]` `opt-level=3, lto="thin", codegen-units=1, strip="debuginfo", incremental=false, trim-paths="object"`; `[profile.wasm-release]` inherits release, tuned `opt-level="s"`, `strip="symbols"` (heavily commented rationale, see file).
- `[workspace.lints.rust]` (`future_incompatible`, `rust_2018_idioms`, `unsafe_op_in_unsafe_fn`, `unused_lifetimes`, `unused_qualifications`, all `"warn"`) and `[workspace.lints.clippy]` (`all = "warn"` plus a curated list — `cloned_instead_of_copied`, `inefficient_to_string`, `map_unwrap_or`, `needless_pass_by_value`, `semicolon_if_nothing_returned`, `unnecessary_wraps`, `redundant_clone`, all `"warn"`, never `"deny"` in-manifest — zero-warning is enforced only at CI/verification gates via `cargo clippy -- -D warnings`, and RUSTFLAGS must never carry `-D warnings` because it **replaces**, not merges, `.cargo/config.toml`'s `rustflags`).

**`rust-toolchain.toml`**: `channel = "nightly-2026-07-07"`, `components = ["rust-src","llvm-tools-preview"]`, `targets = ["wasm32-unknown-unknown","wasm32-wasip2"]`.

**`.cargo/config.toml`**: `[build] rustc-wrapper = "sccache"`, `rustflags = ["-Z","threads=8"]`; `[unstable] no-embed-metadata = true`; per-target rustflag overrides for `wasm32-unknown-unknown` (`getrandom_backend="wasm_js"` cfg), `wasm32-wasip2` (adds `--max-memory=536870912`, note target-scoped rustflags **replace** not merge `[build].rustflags`, hence `-Z threads=8` is repeated), and `x86_64/aarch64-unknown-linux-gnu` (`-fuse-ld=mold`). No Windows-target override present.

**Repo-CLI crate itself** (`🔨️modules/⌨️cli/📦️packages/🦀️rust/Cargo.toml`):
```toml
[package]
name = "semio-framework-repo-cli"
description = "semio monorepo orchestrator CLI + TUI dashboard (binary: semio)"
[package.metadata.semio]
role = "tool"
[lints]
workspace = true
[lib]
path = "🦀️.rs"
[[bin]]
name = "semio"
path = "📦️main.rs"
[dependencies]
ui_tui = { path = "...", features = ["tui-terminal"], package = "semio-framework-ui" }
ui_styling = { path = "...", package = "semio-framework-ui-styling" }
dispatch_macros = { path = "...", package = "semio-framework-dispatch-macros" }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
```
Note this crate is NOT yet using `.workspace = true` for its serde/serde_json versions (its own pinned `1.0.219`/`1.0.140` differ slightly from the workspace-alias table's `1.0.228`/`1.0.149` — consistent with the root comment that adoption is still opt-in per-crate) — a Rust port that touches this crate could either match the crate's existing pins or adopt `.workspace = true` (CLAUDE.md says refactor to the clean long-term form, not preserve inconsistency, so prefer `.workspace = true` if editing this manifest anyway). `role = "tool"` is the only `[package.metadata.semio]` key present — no other custom metadata keys on this crate.

**`dispatch_macros` crate** (`🔨️modules/🔀️dispatch/📦️packages/🦀️rust/`): `name` presumably `semio-framework-dispatch-macros` (matches the `package =` rename above); package glue root re-exports `component` module from sibling `🦀️.rs`; declares `#[proc_macro_attribute] dyn_enum` and `#[proc_macro] dyn_enum_close`; crate-root carries `#![allow(async_fn_in_trait)]` with a documented rationale (R7 ruling) that every **consuming** crate declaring a `#[dyn_enum]` trait must repeat this allow itself.

## 5. nx registration, `.vscode/launch.json`, layering/dependency/migration constraints

**`📋️project.json`** for the repo-cli crate (`@semio-tech/repo-cli-rs`) registers targets `build`, `test`, `test-quick`, `test-long`, `test-exhaustive`, `run` (forwardAllArgs), `daemon` (forwardAllArgs), `workflow` (forwardAllArgs) — every target is `nx:run-commands` calling `bun ./📜️script.ts <cmd> [<sub>]` with `cwd` pinned to the crate dir, per CLAUDE.md's "project.json MUST only call script.ts" rule. `📜️script.ts` (`BuildScript`/`TestScript`/`RunScript`/`DaemonScript`/`WorkflowScript`, all `extends BundleScript`, registered via `ScriptRouter`) does the actual `cargo build -p semio-framework-repo-cli [--release]` / `runCargoTestBudgeted(["semio-framework-repo-cli"], repoRoot, rest)` / build-then-exec-with-forwarded-argv-and-inherited-stdio for `run`/`daemon`/`workflow`.

**`.vscode/launch.json` entries for this crate** (lines ~70-110, group `3_dev`):
| name | order | command |
|---|---|---|
| `🛠️dev🎛️dashboard` | 3_dev / 1 | `bun ./…/⌨️cli/📦️packages/🦀️rust/📜️script.ts run` |
| `🛠️dev🎛️dashboard🌀daemon▶️start` | 3_dev / 1.1 | `…📜️script.ts daemon start` |
| `🛠️dev🎛️dashboard🌀daemon📎attach` | 3_dev / 1.2 | `…📜️script.ts daemon attach` |
| `🛠️dev🎛️dashboard🌊️workflow` | 3_dev / 1.3 | `…📜️script.ts workflow` |

No `test`/`test-quick`/etc. launch entries exist for the repo-cli crate specifically (unlike many other crates which have per-level test configs) — the file is 9853 lines and a grep for `semio-framework-repo-cli`/`repo-cli-rs` found none beyond the four above, so adding `test`/`test-quick`/`test-long`/`test-exhaustive` launch entries for this crate (grouped near line ~74-110, `3_dev` or a new `4_test`-style group, matching neighboring crates' naming) is presently a gap per CLAUDE.md's "All devs are using launch.json… MUST register all executable commands there."

Around **line ~1750-1800** (not literally 1750-1800 but the same neighborhood by content) sit the **Go MCP client** launch configs — separate from the Rust CLI entirely:
- line 1772: an MCP-inspector launch whose `uriFormat` embeds `serverCommand=go&serverArgs=run&serverArgs=./🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go&MCP_PROXY_FULL_ADDRESS=http://127.0.0.1:6277`.
- line 1790: `go build -o 🔨️modules/💻️client/client ./🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go && ./🔨️modules/💻️client/client --help`.

This confirms a **pre-existing, separate Go-based MCP server/CLI client** (`🔨️modules/💻️client/⌨️cli/🐹️.go`, `🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go`, plus a VS Code extension client at `🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript`) that is the actual current backend for repo-domain operations (tickets/goals/MCP tools) — entirely disjoint from the Rust `semio` dashboard/CLI crate. `🧅️layering.json`'s shrink-only ratchet lists `🔨️modules/💻️client/⌨️cli/🐹️.go: 10` and `🔨️modules/💻️client/⌨️cli/🧪️component_test.go: 150` and `command-tree-discovery/🦀️.rs: 1` as files still holding legacy cross-area references — i.e. `command-tree-discovery/🦀️.rs` already has exactly 1 tracked layering violation reference (not identified further here; would need `🔒️dependencies.json`/`🧅️layering.json` cross-reference to see which area it points at — flagged for follow-up if the ticket needs to touch that file).

**`🔒️dependencies.json`**: no rust-ecosystem entry for the repo-cli crate specifically was found among the `"ecosystem": "rust"` blocks skimmed (blocks seen cover ui/render/vulkan, hub+mcp+server groupings, etc.) — grep for `cli|repo-cli|rust` mostly matched VS Code TS package.json refs and unrelated wgpu Cargo.toml refs, so this file does not appear to impose extra constraints specific to the repo-cli crate beyond what `Cargo.toml`'s own dependency list already encodes.

**`🚚️migration.json`**: not inspected in detail (no repo-cli/rust hits surfaced in the earlier targeted greps); given CLAUDE.md's "no migration scripts" rule and this being a greenfield repo, it is unlikely to gate this crate, but a follow-up direct read is recommended before any structural move of these files.

## Key takeaways for the ticket

1. Adding a Rust command = new `🎮️commands/<slug>/🦀️.rs` + `#[path]` mod line in the cli lib + (optional) verb arm in `run()`'s match + `🔣️.json` entry + usage string update.
2. The dashboard's command tree is 100% derived at runtime from nx `project.json` targets + the TS-generated playground catalog JSON — there is no static "repo command manifest" to edit for ordinary nx targets to appear.
3. There is currently **no** Rust-side connection to tickets/goals/MCP — that functionality lives only in the separate Go CLI/MCP-server/VS Code-extension client stack (`🔨️modules/💻️client/…`). Making the dashboard "use the Rust implementation by default" is a net-new integration: either port the Go domain logic into a new `🎮️commands/<repo-domain>/🦀️.rs` (or a shared library crate) and inject its leaves into `command_tree_discovery::discover`, or have Rust consume a generated JSON produced by whatever remains the source of truth (mirroring the existing `catalog::load_playground_catalog` pattern).
4. Accepted external Rust deps repo-wide: `serde`, `serde_json`, `wasm-bindgen`, `tokio` (workspace-aliased); the repo-cli crate itself only depends on `serde`/`serde_json` plus three internal path crates (`ui_tui`, `ui_styling`, `dispatch_macros`). No regex/sha2/flate2 workspace alias exists — the test protocol's hashing is almost certainly hand-rolled (verify in `📡️protocol/🦀️.rs` before assuming a crate is needed).
5. `test`/`test-quick`/`test-long`/`test-exhaustive` launch.json entries for `@semio-tech/repo-cli-rs` are missing (only `run`/`daemon start`/`daemon attach`/`workflow` exist) — needed if this ticket adds meaningfully more Rust test surface to this crate.
6. The PTY spawn path (`terminal_dashboard::run`'s `Pty::spawn` and `daemon::Supervisor::spawn_session`) is unix-gated (`#[cfg(all(unix, not(target_arch="wasm32")))]`) in the daemon supervisor; the interactive dashboard's own `spawn_output` call site was not seen guarded the same way in the excerpt read — worth a closer look if Windows-native dashboard behavior is in scope for this ticket.

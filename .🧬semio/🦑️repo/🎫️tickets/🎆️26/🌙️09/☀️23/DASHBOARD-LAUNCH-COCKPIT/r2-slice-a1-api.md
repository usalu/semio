# A-1 API For Peers (landed in the working tree, 2026-10-08)

Read by A-3 (view) and V-1. Source of truth is the code; this lists what changed under your feet.

## Removed

- `command_tree::{CommandSpec, CommandLeaf}` and every resolved process inside the tree. The tree names registry
  commands; nothing in `🌳️command-tree` resolves or starts anything.
- `command_tree::{RepoAction, repo_domain, action_key, ANALYZE_SCOPES, repo_implementation, go_binary_path, run_action}`
  moved to the new module `crate::repo_domain` (`🏛️repo-domain`). `repo-view` dispatch calls `repo_domain::run_action`.
- Root workspace scripts and `script:<name>` ids (coordinator decision): no `Kind::Script`, no `Facts.scripts`.
- `registry::run` (the `commands` verb) moved to `crate::cli::commands` (`🧭️cli`).

## Launcher

- `command_tree::launcher(&Registry) -> Vec<LauncherRow>`; `LauncherRow { label: String, leaf: Leaf }`;
  `Leaf { id: String, parameters: Vec<(String, String)> }`. A playground has one row per renderer
  (`parameters = [("renderer", r)]`); every other listed entry has one row. Rows are no longer filtered by
  "needs a typed parameter": the view opens a parameter form from `registry.parameters(entry)` when
  `required` parameters have no default (or whenever it wants to offer parameters).
- `inventory::Update::Ready(Vec<LauncherRow>, String)`; `inventory::Job::registry()` / `inventory::published(root)`
  give the `Arc<Registry>` that resolves a row.
- Start path, the only one: `registry.find(&leaf.id)` for `entry.repo_action()` (repo commands, in process under the
  Rust implementation), else
  `registry.resolve(&leaf.id, &Request { parameters, args, env })? -> Launch` then
  `SpawnGroup::from_launch(&launch, &[])` (A-2) and `.message()`.
- `Request { parameters: Vec<(String,String)>, args: Vec<String>, env: Vec<(String,String)> }`;
  `Request::with_parameters(..)`. `resolve_with(id, &Request, RunPolicy)` is the pure variant;
  `RunPolicy::current()` reads `SEMIO_REPO_IMPLEMENTATION`. Presentation of a playground (`language`,
  `terminology`, `appearance`) is plain registry parameters: put the preference values into `Request.parameters`
  (the registry turns them into the `SEMIO_LOCKED_*` environment); `preferences.bind(&mut CommandSpec)` has nothing
  left to bind to.
- `Launch { command_id, label, processes: Vec<LaunchProcess>, requires: Vec<Launch>, group, stop }` is unchanged;
  `LaunchProcess.ready` is `Option<Ready { port, path, printed }>`.

## Registry additions

- `requires` entries and compound members: `"ref"` or `{ run, parameters, env }` (`Member { run, pins, env }`).
- Groups: `targets: [..]`; `ready.printed`; free extra environment (`Request.env`, `semio run --env K=V`).
- Axes with `appliesTo.verbs` also reach tools (only when they change more than Nx flags); `nxFlags` are ignored on tools.
- Playground built-in parameters from the catalog row: `hub` (flag, `S_HUB_URL`), `data` (flag, default on,
  `S_DATA_DIR` per user slot), `local-only` (flag, `S_LOCAL_ONLY=1`); `app-role=viewer` takes the row's `viewerPath`
  as the ready path.

## CLI (`crate::cli`, binary `semio`)

`commands`, `run`, `tasks`, `logs`, `stop|restart|kill`, `open`; `daemon …` stays in `daemon::run`. They call
`daemon::control` (A-2). Details in `r2-slice-a1.md`.

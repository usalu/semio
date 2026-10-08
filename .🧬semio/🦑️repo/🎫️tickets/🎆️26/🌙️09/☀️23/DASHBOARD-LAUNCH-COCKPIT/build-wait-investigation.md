# Build Stuck On Waiting

## Reported

Running the mit-bestand Zwischenbericht build from the dashboard sat on "waiting".

## What was happening

The line is Nx's own: `Waiting for graph construction in another process to complete`. Every `nx run`
rebuilds the project graph first (the repository runs Nx without its daemon), serialised by
`.nx/workspace-data/project-graph.lock`. Measured on 2026-10-07 with load average 130–230 and about 30
concurrent Nx processes from other sessions:

| Start of `nx` for the Zwischenbericht project | Seconds |
| --- | --- |
| Shared data directory, queued behind the fleet (direct shell run) | ~210 before the first task |
| Private data directory, cold graph | 305 |
| Private data directory, warm graph | 206 |
| Dashboard-owned Nx daemon | 352 (daemon failed to start within Nx's 60 s limit, then disabled itself) |
| `NX_FORCE_REUSE_CACHED_GRAPH=true` | 24 |

So the wait is graph construction itself, not the dashboard and not only the lock queue. The dashboard
session the developer started (pid 11417) did finish with exit 0 after the wait; the earlier one was
interrupted (exit 130).

## Change

Finite dashboard tasks start from the published graph (`NX_FORCE_REUSE_CACHED_GRAPH=true`). Nx then
only re-reads the workspace files (`readCachedGraphAndHydrateFileMap` → `retrieveWorkspaceFiles`), so task
hashes and cache decisions use current sources; only project definitions are reused. Guard: the graph must
be at least as new as `nx.json`, root `package.json` and every project manifest the last discovery found
(`⚡️cache/🎛️dashboard/manifests.json`); otherwise the task runs normally and republishes the graph.
Continuous verbs (`start`, `dev`, `serve`, `watch`, `activate`, `preview`) never reuse.
`SEMIO_DASHBOARD_GRAPH=fresh` disables reuse.

## Evidence

- Unit test `published_graph_is_reused_only_while_no_manifest_is_newer`.
- Installed dashboard, real workspace: launched `build / launch / 📦️build-zwischenbericht…`; the session
  recorded `NX_FORCE_REUSE_CACHED_GRAPH=true`; `nx` started at 0 s and the document build command at 31 s.

## Not covered

- Manifests created after the last discovery and targets inferred from non-manifest files are outside the guard.
- The TeX compile itself is unchanged and slow under load (17 m 51 s measured with three copies running at once).
- A restarted task (`Ctrl+B r`) keeps the environment it was started with.
- While a second view attached to the developer's running daemon, the footer alternated "daemon
  disconnected / connected" once per second; not reproduced on a fixture workspace and not investigated on
  the live daemon.
- Result: the dashboard session (pid 23333) exited 0 and published the PDF.

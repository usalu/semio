# Nx Browser-Actor Fixture Dispatch: Cache Split Root Cause

## Finding

The fixture declaration is discoverable and its target CWD is correct. The
failure is the Nx **forked executor's stale/missing project-graph read**, not
an emoji path, JCO, Cargo, Bun script, or fixture artifact failure.

The observed outer command's `show project` result proves the configured emoji
plugin produces:

```json
{
  "name": "@semio-tech/browser-actor-import-fixture",
  "root": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import",
  "targets": {
    "pending-host-close-check": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import",
        "command": "bun ./📜️script.ts pending-host-close-check"
      }
    }
  }
}
```

No fixture command was run for this audit.

## Source chain

1. `nx.json` loads the owned emoji project plugin at
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`.
   Its `createNodesV2` accepts `**/📋️project.json`, excludes only lossy
   U+FFFD paths/`node_modules`/`.🧬semio`/`dist`, and normalizes the relative
   root NFC. The fixture's project file is none of those exclusions.
2. The fixture's declared `cwd: "{projectRoot}"` at
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📋️project.json`
   is expanded by Nx before execution; `show project` above demonstrates its
   concrete result. It is not a literal nonexistent `{projectRoot}` directory.
3. With `NX_CACHE_PROJECT_GRAPH=false`, parent graph construction still finds
   the fixture, but `node_modules/nx/src/project-graph/project-graph.js:103`
   sets `cacheEnabled` false and line 116 skips `writeCache`.
4. A forked task child is not handed the parent's in-memory graph. Its
   `node_modules/nx/bin/run-executor.js` invokes `run()`, and
   `node_modules/nx/src/command-line/run/run.js:173` calls
   `readCachedProjectGraph()` unconditionally. The child then fails
   `validateProject` in `run.js:29` if its graph predates the new fixture.
   This is the exact source of `Could not find project "…"`.

The direct `nx:run-commands` fast path would not need this graph read, but it
is bypassed whenever Nx must use a fork (for example prefixed/TTY routing).
The actual child diagnostic establishes that this path was selected; cache
suppression therefore makes the command nondeterministic even though the
parent selection is fresh.

## Clean invocation fix

Use a **per-ticket workspace-data directory** and permit graph persistence:

```text
NX_DAEMON=false
NX_ISOLATE_PLUGINS=false
NX_CACHE_PROJECT_GRAPH=true
NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/nx-browser-actor-import
NX_VERBOSE_LOGGING=true
bun x --no-install nx run @semio-tech/browser-actor-import-fixture:pending-host-close-check --output-style=stream --skip-nx-cache
```

`NX_WORKSPACE_DATA_DIRECTORY` is a supported Nx control in
`node_modules/nx/src/utils/cache-directory.js:67-70` (the older
`NX_PROJECT_GRAPH_CACHE_DIRECTORY` is also read there). It scopes the
project-graph lock and `project-graph.json`, file map, source maps, workspace
context, daemon state, and task DB. The parent writes the fresh graph under
that directory before the fork; the child inherits the same environment and
reads the same graph.

`--skip-nx-cache` remains correct: it bypasses task-result caching, not the
project graph required by `run-executor`. Do not use
`NX_FORCE_REUSE_CACHED_GRAPH`; it would reintroduce stale discovery. Do not
reset or delete the shared `.nx/workspace-data`; unrelated native work owns it.

## Minimal acceptance

1. In a new ticket-generated workspace-data directory, run `nx show project`
   and verify the concrete root/CWD above.
2. Run the registered target once with the exact same environment. The child
   must reach `bun ./📜️script.ts pending-host-close-check`; only then may its
   guest artifacts be considered test evidence.
3. Repeat through the same isolated directory. It must not regress to
   `Could not find project`; inspect the child command only if it does.

No fixture/project workaround is appropriate. Removing
`NX_CACHE_PROJECT_GRAPH=false` without isolating the directory would mutate a
shared graph and reintroduce cross-agent graph-lock contention.

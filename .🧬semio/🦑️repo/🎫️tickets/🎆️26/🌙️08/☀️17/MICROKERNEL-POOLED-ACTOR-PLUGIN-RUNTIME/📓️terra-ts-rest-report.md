# 📓️ terra — ts-rest (dev tooling + remaining framework TS modules)

Packet: dev tooling + 4 TS module packages. Owned paths: `🧑️‍💻️dev`, `◻2d`, `🧊️3d`,
`📡️replication/📦️packages/🟦️typescript`, `🌉️mcp/📦️packages/🟦️typescript`. Ticket folder used for
this report only; no other scratch files needed.

## Summary of measured state, before → after

| package | before (measured) | after (measured) |
|---|---|---|
| `@semio-tech/s-2d-js` | `nx run …:test` → **ENOENT** (stale `cwd`, dir doesn't exist) | **4 passed / 0 failed** |
| `@semio-tech/s-3d-js` | `nx run …:test` → **ENOENT** (same) | **1 passed / 0 failed** |
| `@semio-tech/framework-replication` (ts) | `nx run …:test` → **hangs forever** (infinite self-recursion) | **1 passed / 0 failed** |
| `@semio-tech/framework-os-dev` (ts) | baseline said 17 passed (stale) | **27 passed / 0 failed** — config was already correct (`include: []` + `includeSource`), no bug here; growth is other sessions' prior work, not mine |
| `@semio-tech/framework-os-mcp` (ts) | unmeasured this session | **20 passed / 0 failed**, 4 test files — config already correct, no `include`/`includeSource` overlap |

All numbers are unique names, confirmed with `--reporter=verbose`, pasted output included below.

## Bug 1 — stale `cwd` in `2d`/`3d` `project.json` (ENOENT)

Both project.jsons still pointed `options.cwd` at the pre-move location
`✏️s/🔨️modules/{◻2d,🧊️3d}/📦️packages/🟦️typescript`. That tree no longer exists (moved to
`🧰️framework/🔨️modules/{◻2d,🧊️3d}/…` — confirmed via `git ls-tree HEAD`, old path absent, new path
present since the same commit `19b970280c` that never updated the `cwd` string). Every `test`/`test-quick`/
`test-long`/`test-exhaustive` target failed with `Error: spawn /bin/sh ENOENT`.

```
$ bunx nx run @semio-tech/s-2d-js:test
Error: spawn /bin/sh ENOENT
 NX   Running target test for project @semio-tech/s-2d-js failed
```

Fixed: updated all four `cwd` values in both `📋️project.json` files to the real path.

Files: `🧰️framework/🔨️modules/◻2d/📦️packages/🟦️typescript/📋️project.json`,
`🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📋️project.json`.

## Bug 2 — `2d`/`3d` vitest configs collected ZERO tests, reported green

Once the `cwd` was fixed, `nx test` for `2d` printed `No test files found, exiting with code 0` and NX
reported **Successfully ran target test** — a silent false-green, exactly the class R2/rule-18 warns
about. Root cause: `vitest.config.ts` had `include: ["index.ts"]`, but the real file is `📦️index.ts`
(with the package emoji) — `include` names literal test-file globs, so `"index.ts"` matched nothing. The
in-source `import.meta.vitest` suite inside `📦️index.ts` was never collected at all, in either package.

Fixed both configs to the established in-source pattern used by `dev`/`replication`: `include: []`,
`includeSource: ["📦️index.ts"]`.

Verified after fix:

```
$ bunx nx run @semio-tech/s-2d-js:test -- --reporter=verbose
 ✓ 📦️index.ts > @semio-tech/s-2d-js > recognizes drawing refs
 ✓ 📦️index.ts > @semio-tech/s-2d-js > parses drawing scene preview payloads
 ✓ 📦️index.ts > @semio-tech/s-2d-js > parses dwg export and import payloads
 ✓ 📦️index.ts > @semio-tech/s-2d-js > rasterizes a rect scene to png data url
 Test Files  1 passed (1)   Tests  4 passed (4)

$ bunx nx run @semio-tech/s-3d-js:test -- --reporter=verbose
 ✓ 📦️index.ts > @semio-tech/geometry-brep-js > isRenderableMeshTransfer accepts triangle meshes
 Test Files  1 passed (1)   Tests  1 passed (1)
```

Files: `🧰️framework/🔨️modules/◻2d/📦️packages/🟦️typescript/🧪️vitest.config.ts`,
`🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/🧪️vitest.config.ts`.

## Bug 3 — `framework-replication` (ts): infinite self-recursion via `nx run`, not the include/includeSource bug

`replication`'s `vitest.config.ts` was already correct (`include: []` + `includeSource`, no double-count).
But `nx run @semio-tech/framework-replication:test` hung indefinitely — reproduced 4 times (2m, 3m, 10m,
2m timeouts), survived `--skip-nx-cache` and `NX_DAEMON=false`, so it was not a cache-replay artifact.

Root cause, found via `bunx nx show project @semio-tech/framework-replication --json`: nx resolved the
`test` target to `executor: "nx:run-script"`, `options.script: "test"` — i.e. it picked the **plugin-
inferred** target built from `package.json`'s own `scripts.test`, not the explicit `nx:run-commands`
target declared in `📋️project.json`. `package.json`'s `scripts.test` is
`"bun nx run @semio-tech/framework-replication:test"` — a self-reference that every sibling TS package
in this repo also carries **but neutralizes** with `"nx": { "includedScripts": [] }` in `package.json`,
which tells nx's script-inference plugin to infer nothing from `scripts`, leaving the explicit
`project.json` target as the only candidate. `replication`'s `package.json` was the one package in my
scope missing that guard, so nx merged in the self-referencing inferred target, and running it looped:
`nx run …:test` → `bun run test` → `bun nx run …:test` → `nx run …:test` → … forever, one live process
tree per attempt.

```
$ bunx nx show project @semio-tech/framework-replication --json | jq .targets.test
{
  "executor": "nx:run-script",
  "options": { "script": "test" },
  "metadata": { "scriptContent": "bun nx run @semio-tech/framework-replication:test", … }
}
```

Fixed: added `"nx": { "includedScripts": [] }` to
`🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/package.json`, matching the pattern
already present in `os-dev`'s and `2d`'s/`3d`'s `package.json`. Re-resolved target after the fix:

```
$ bunx nx show project @semio-tech/framework-replication --json | jq .targets.test
{ "executor": "nx:run-commands", "options": { "cwd": "…", "command": "bun ./📜️script.ts test", … } }

$ bunx nx run @semio-tech/framework-replication:test --skip-nx-cache -- --reporter=verbose
 ✓ ../../🟦️component.ts > wire fixtures > decodes the Rust-generated binary wire fixtures byte-identically
 Test Files  1 passed (1)   Tests  1 passed (1)
```

### ⚠️ Cross-packet finding — same missing-guard recursion bug exists OUTSIDE my scope

Repo-wide python census (self-referencing `scripts.<x>: "bun nx run <own-name>:<x>"` without the
`nx.includedScripts: []` guard) found **4 more real hits**, all outside my owned paths — I did not touch
them, flagging for the coordinator / the owning packets:

- `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/package.json` (`@semio-tech/framework-kernel`)
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/package.json` (`@semio-tech/framework-schema`)
- `🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/package.json` (`@semio-tech/framework-server`)
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/package.json` (`@semio-tech/framework-os`)

Any of these will hang/recurse the same way the moment someone runs `nx run <name>:test` (or any other
self-referencing script target) for them. Same one-line fix each: add `"nx": { "includedScripts": [] }`.
(Root `./package.json` also matched the grep but is almost certainly noise from an unrelated script —
not verified, not claimed as a real hit.)

## Confirmed still intact (no change needed)

- `pluginModuleDirNames` in `🧑️‍💻️dev`'s `⚙️vite.config.ts:69` still reads
  `["_vendor", "_shard", resolvedPluginId]` — the sibling's `_shard` fix for single-variant production
  worker 404s is present.
- `🌉️mcp`'s `🧪️vitest.config.ts`: `include` lists the 3 real `.test.ts` filenames, `includeSource` names
  only `../../🟦️component.ts` — no filename overlap, no double-count. **20 passed / 0 failed** across 4
  files, all by name (`legacy-conformance` ×8, `modern-era` ×6, `hygiene` ×4, in-source ×2).
- `🧑️‍💻️dev`'s and `📡️replication`'s `vitest.config.ts` `include`/`includeSource` split was already
  correct (the double-count fix from the earlier `terra-web-kernel-package-report.md` sweep held).

## Not investigated further

- `dev`'s jump from the recorded baseline 17→27 passed reflects other sessions' work landing in
  `📜️script.ts` since that baseline was taken (not mine) — reported as observed, not attributed.
- Did not run `dev`'s `verify`/`layer-lint`/`index-lint`/`host-handle-lint` targets or `tsc --noEmit`
  (out of this packet's ask; flag if the coordinator wants those too).

## Files touched (all within owned paths)

- `🧰️framework/🔨️modules/◻2d/📦️packages/🟦️typescript/📋️project.json` — fixed stale `cwd` (4 targets)
- `🧰️framework/🔨️modules/◻2d/📦️packages/🟦️typescript/🧪️vitest.config.ts` — `include`→`includeSource` fix
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📋️project.json` — fixed stale `cwd` (4 targets)
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/🧪️vitest.config.ts` — `include`→`includeSource` fix
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/package.json` — added
  `nx.includedScripts: []` guard, fixes infinite recursion

No `lease-request` needed — everything touched was inside my owned paths. The 4 sibling instances of the
recursion bug are reported above for the coordinator to route, not requested as a lease (they're each
inside another packet's scope, not a shared file I need edited).

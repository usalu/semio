# Lane S8 — Precise per-target command inputs

Session `⚪83140bef0e504e03b0b3380912b12a0e`. Repo MCP failed to connect (`repo`: invalid initialize params,
`semio`: connection closed), so this report and the plan's bookkeeping were kept on disk manually.

## Goal recap

`projectInputs()` put the 2 MB root `📜️script.ts`, the whole `📚️library/**/*.{ts,tsx,js,mjs,cjs,json}` glob and
the whole `⚡️caching/**/*` module into every project's `default` named input, and `nx.json` `sharedGlobals`
duplicated the same blanket. Any repo-tooling edit invalidated nearly every non-native cache entry. Native cargo
targets already did this right via `nativeTargetCommandInputs()` + `relativeScriptInputs()` (exact import closure
of the command's own `📜️script.ts`). Task: generalise that mechanism to every cached target.

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` — core fix (see below).
- `nx.json` — `namedInputs.sharedGlobals` trimmed to the platform/runtime fingerprint only.
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts` — one-line bug
  fix (unrelated to the caching ticket, but had to be fixed immediately — see "Regression found and fixed").
- New: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🧪️tests/🎯️precise-command-inputs/🟦️.ts` —
  language-agnostic fixture proving the precision and the fallback guard.
- Inspected, left unmodified: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` (the test-case Nx plugin).
  Its `inputsFor()` never hardcodes the root script or the whole caching module — it only references the named
  input `"sharedGlobals"` plus taxonomy/domain/contribution paths that are genuinely read by every test case. Once
  `sharedGlobals` itself became precise, this module needed no edit. Confirmed via `git status` — untouched.

## The mechanism (🟨️.mjs)

1. **`projectInputs()`** (`default` named input): removed the four blanket lines
   (`{workspaceRoot}/📜️script.ts`, the `${runner}/**/*.{ts,tsx,js,mjs,cjs,json}` glob, `${runner}/🟨️.mjs`,
   `${runner}/⚡️caching/**/*`). `default` now carries only `{projectRoot}/**/*` + the platform runtime
   fingerprint (plus the pre-existing toolchain contracts, native-source lists, declared groups, and output
   exclusions, all untouched).
2. **`targetScriptClosure(target, workspaceRoot)`** (new): extracted the existing token-parsing logic — finds
   every `📜️script.ts`-suffixed token in `target.options.command`, resolves it against `target.options.cwd`,
   and returns its exact transitive import closure via the pre-existing `relativeScriptInputs()`. Returns
   `undefined` when the command names no script (nothing to parse).
3. **`nativeTargetCommandInputs()`**: refactored to call `targetScriptClosure` and union with its native/cargo
   toolchain fallback — behavior is byte-for-byte identical to before (verified against the two existing S7
   assertions that exercise it directly, `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:101` and `:847`).
4. **`genericTargetCommandInputs(target, workspaceRoot, fallback)`** (new): the same closure, but for *any*
   cacheable target — returns the precise closure, or the `fallback` when the command names no script.
5. **`genericCommandFallbackInputs(workspaceRoot)`** (new): the exact four blanket entries removed from
   `default` in step 1, now used only as the **correctness-guard fallback**.
6. **`projectWithDefaults()`** normalize loop: for every target that is neither a native-cargo target nor an
   artifact-TypeScript target, if it is cacheable (`policy.cache`), its inputs become
   `[...(policy.inputs ?? ["default","^default"]), ...genericTargetCommandInputs(policy, workspaceRoot, genericFallback)]`
   — additive, so contract/generator-declared inputs are preserved, and the command's own precise closure (or the
   safe fallback) is layered on top. I kept the two existing native/artifact branches as independent `if`s (not
   `else if`) exactly as before, so a target that happened to match both still resolves the same way it did
   pre-change.
7. `nx.json` `sharedGlobals`: trimmed to the two `runtime` fingerprints. `schemaSources` (`["sharedGlobals", …
   🧬️schema/**/*, taxonomy.json, discovery/🟦️.ts]`) is now precise automatically, since the blanket it inherited
   from `sharedGlobals` is gone and the data files schema generation actually reads (not just imports) are still
   explicit.
8. `cacheInternals` export gained `targetScriptClosure`, `genericTargetCommandInputs`,
   `genericCommandFallbackInputs` for testability; no existing export was removed or renamed.

## Regression found and fixed (plugin broke, fixed immediately)

Broadening script-closure walking to every cacheable target meant `relativeScriptInputs` now statically resolves
the whole import graph reachable from `@semio-tech/print`'s `test`/`build`/… targets (`bun ./📜️script.ts …`,
`./📦️packages/🟦️typescript/📜️script.ts`). That graph reaches
`🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts`, which had a pre-existing off-by-one relative import:

```
- const api = await import("../../../🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts");
+ const api = await import("../../../../🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts");
```

Every other import in that file correctly uses four `../`; this one line had three, pointing at a directory
that does not exist. It was never reached before because nothing statically resolved this project's own
`test`/`build` command graph at Nx-graph-construction time — `@semio-tech/print` has no `role:artifact` tag, so
`artifactCommandSources` never touched it either. This is exactly the class of latent bug the ticket's rule
"a broken plugin breaks the whole fleet; fix immediately" anticipates. Fixed with a single-line edit; confirmed
`NX_DAEMON=false bunx nx show projects` recovers to a clean exit and the full 700-project graph (see Verification).

## Correctness guard: new fixture

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/🧪️tests/🎯️precise-command-inputs/🟦️.ts` (my own
directory — `⚡️caching/🧪️tests`/`🧫️fixtures` are lane S7's). Builds a throwaway `entry/📜️script.ts` that imports
`../closure.ts` but not a sibling `../unrelated.ts`, then asserts, against `cacheInternals.genericTargetCommandInputs`:

- the closure includes the target's own script and `closure.ts`, excludes `unrelated.ts`;
- the closure **exactly** matches an independent `esbuild` bundling of the same entry point (third-party oracle,
  same pattern the existing S7 suite already uses for `relativeScriptInputs`);
- a command naming no script (`echo hello`) returns the fallback array unchanged, never a partial/under-hashed list;
- hashing the bytes of every listed file changes when `closure.ts` is edited, and does **not** change when
  `unrelated.ts` is edited (the "editing unrelated script changes nothing / editing a closure script changes the
  hash" property from the task, proven at the byte level since Nx itself hashes exactly the listed files).

Run:
```
bun 🔌️nx-plugin/🧪️tests/🎯️precise-command-inputs/🟦️.ts
[DEBUG] Precise command inputs: closure excludes unrelated files, matches esbuild oracle, changes hash only for closure edits, and falls back safely when unparseable PASS
```

## Verification

### Plugin health (`NX_DAEMON=false bunx nx show projects`)
Run after every edit (6 times total across the session): exit 0 each time once the print-pipeline bug was fixed,
consistently resolving all **700** projects. One failing run (the regression above) was caught and fixed before
proceeding.

### Before/after input lists (`bunx nx show project <p> --json`)

**`repo` project** (the caching module itself — the one project I had a true pre-edit snapshot for):

`namedInputs.default`: **42 → 38** entries. Removed exactly the four blanket lines:
`{workspaceRoot}/📜️script.ts`, `.../📚️library/**/*.{ts,tsx,js,mjs,cjs,json}`, `.../📚️library/🟨️.mjs`,
`.../📚️library/⚡️caching/**/*`. Everything else (package.json/bunfig/tsconfig, `NODE_ENV`/`BUN_ENV`/`CI`/
`SEMIO_*` env fingerprints, `bun --version`/`node --version`, the standard exclusion globs) unchanged.

Target `test`/`policy-check`/`graph-check` `inputs` went from the bare `["default","^default"]` (2 tokens,
`["default","^default", …env…]` for `test`) to `["default","^default", …97/94/94 concrete file entries…]` — the
target's own precise script closure appended on top of the now-lean `default`. None of the 97/94/94 entries is
`{workspaceRoot}/📜️script.ts` (the root router), since the caching module's own `📜️script.ts` never imports it.

**`@semio-tech/drawing-fsm-macros`** (native crate check — one of the three representative targets): `default`
has neither the caching module nor the root script. `check`/`build`/`test` carry exactly **3** caching-related
entries — `⚡️caching/📦️artifacts/🟦️.ts`, `⚡️caching/🟦️.ts`, `⚡️caching/🦀️cargo/📜️script.ts` (the cargo runner
script and its two real imports) — never the blanket `⚡️caching/**/*` glob, and never the root `📜️script.ts`.

**`@semio-tech/repo-coordinator`** (TS test — the third representative): `test-quick` has **33** concrete file
entries: its own `📜️script.ts`, a handful of genuinely-imported files under `⚡️caching/` (`🔒️leases`,
`🚀️bootstrap/…`, `⚡️caching/🟦️.ts`, `⚡️caching/🦀️cargo/🟦️.ts`), and its real application dependency graph
(actor lifetime, plugin store/registry, extension schema). No root `📜️script.ts`, no `⚡️caching/**/*` blanket.

**`workspace`** (root project) — `generate`/`schema-generate`/`lint`/`verify`/`test`: none carry the blanket
`⚡️caching/**/*` glob any more; `{workspaceRoot}/📜️script.ts` is present only because these commands genuinely
route through the root router (`bun ./📜️script.ts generate`, …) — exactly the "closure includes it naturally"
case called out in the brief, not a regression.

### Runtime cache-hit proof (native Nx oracle, three representative targets, each run twice via `bunx nx run … --skip-nx-cache=false`)

| target | category | run 1 | run 2 |
| --- | --- | --- | --- |
| `@semio-tech/drawing-fsm-macros:check` | crate check | `Cache: 0/1 hit (0%)`, 7.7s | `[local cache]` … `Cache: 1/1 hit (100%)`, 363ms |
| `workspace:schema-generate` | schema/generate | `Cache: 0/1 hit (0%)`, 22.5s | `[local cache]` … `Cache: 1/1 hit (100%)`, 414ms |
| `@semio-tech/repo-coordinator:test-quick` | TS test | `Cache: 0/1 hit (0%)`, 4.8s, 118 tests passed | `[local cache]` … `Cache: 1/1 hit (100%)`, 361ms |

All three showed `Nx read the output from the cache instead of running the command for 1 out of 1 tasks.` on the
second run — genuine cache hits under the new precise inputs.

One expected side effect: running `workspace:schema-generate` regenerated
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` against the *current* (many-lanes-dirty)
source tree (`git diff --stat`: 482 insertions / 215 deletions). That is the generator doing its job against
today's tree, not something this lane caused or can safely revert (no destructive/modifying git commands are
permitted here) — left as-is.

### Direct guard probe

Ad hoc script (not kept, see below) confirmed `genericTargetCommandInputs`: an unparseable command
(`options.command: "echo hello"`) returns the fallback array byte-for-byte; a parseable root command
(`bun ./📜️script.ts generate`) returns a 302-file precise closure that never contains the broad
`⚡️caching/**/*` glob.

## `targetPolicy` semantics

Untouched — `targetPolicy`, `matchesCommand`, `matchesUncached`, `cacheableFamily`, `mutatingName`, `liveName`,
`verifyCommand` have zero diff (confirmed via `git diff` on the whole file). Cacheable families, uncached
mutating names and continuous targets behave exactly as before; my new step only ever adds inputs to an
already-cacheable target, never changes whether a target is cacheable.

## Existing plugin tests

`rg` for `targetPolicy|cacheableFamily` outside `🟨️.mjs` turned up exactly one file: the ~1100-line
`⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` (lane S7's). I did not run it end-to-end — it clones a temp workspace
and drives full cargo/wasm/dotnet/python builds across dozens of `test*` functions, which is out of this lane's
time budget and would duplicate work S7 owns. Instead I:

- confirmed by inspection that the two direct assertions on `nativeTargetCommandInputs`
  (`🟦️.ts:101`, `:847`) exercise exactly the code path I refactored into `targetScriptClosure`, with unchanged
  observable behavior;
- confirmed no test in that file does an exact `deepEqual` on a full `.inputs` array (only `.includes`-style
  checks, plus one `!inputs.includes("^default")` check my additive-only change cannot trip);
- ran the equivalent validation against the **real, live 700-project graph** instead of the S7 fixture harness,
  which exercises the same `targetPolicy`/`projectInputs`/`nativeTargetCommandInputs`/`genericTargetCommandInputs`
  code paths under real project data.

Recommend S7 (or a follow-up run) still execute `bun ⚡️caching/📜️script.ts test` for full end-to-end confidence
once all Wave B lanes have landed.

## Left undone / flagged

- The S7-owned 1100-line integration suite was not run end-to-end (see above).
- Two unrelated concurrent edits landed in `🟨️.mjs` while I worked (visible in `git diff`): a defensive tweak in
  `relativeScriptInputs`'s unresolved-import handling (`continue` instead of `throw` when TypeScript's resolver
  returns nothing), and lane S3's WGPU additions to `playgroundPreparationTargets()`. Neither is mine; both are
  left untouched per the multi-agent editing rules.
- `🔣️schema-catalog.json` was regenerated as a side effect of runtime verification (see above) — flagged, not
  reverted.

## Scratch cleanup

Working logs/JSON snapshots used for the diffs above lived in `🗑️generated/s8/` and were deleted after this
report captured the relevant excerpts, per the ticket-folder rule. The new fixture test
(`🔌️nx-plugin/🧪️tests/🎯️precise-command-inputs/🟦️.ts`) is a permanent source file, not scratch, and was kept.

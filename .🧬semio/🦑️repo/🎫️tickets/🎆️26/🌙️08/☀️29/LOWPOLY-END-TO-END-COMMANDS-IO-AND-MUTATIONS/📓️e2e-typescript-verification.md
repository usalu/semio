# Lowpoly TypeScript/JS — Run-Verified State (2026-09-05)

Re-established from scratch, not inherited from a prior session's claim. Every result below was
observed by actually running the command in this session (real stdout captured, exit codes checked
explicitly). No `cargo` was invoked (hard constraint — Rust is out of scope and centrally managed).
No modifying git command was run. Nothing was committed, no ticket was closed/reopened (repo MCP
was down all session; ticket folder managed directly on disk per the fallback in memory
`project-repo-mcp-may-fail-to-connect`).

## 1. `bun nx run @semio-tech/lowpoly-js:test --skip-nx-cache`

Ran twice (once piped for inspection, once with output suppressed to check the exit code alone).

```
> nx run @semio-tech/lowpoly-js:test
> bun ./📜️script.ts test

lowpoly interactive-job owned source/fixture ok: 47 Migrated, 0 BatchOnlyPendingRewrite
lowpoly interactive-job Ajv hostile oracle ok: duplicate, missing lane, non-null blocker on migrated, lane/preparation mismatch rejected

 NX   Successfully ran target test for project @semio-tech/lowpoly-js
```

`echo $?` → **`0`**. Confirmed genuinely green, not just "no crash": I read
`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts` and confirmed the target's
`TestScript.run()` actually parses the Rust editor/schema/session source with regex, cross-checks
every one of the 47 `action_interactive_job` registrations in
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
against the `🧪️interactive-job/🔣️.json` fixture (classification, lane, blocker), then round-trips the
fixture through an Ajv 2020 schema compile and four hostile mutations (duplicate id, missing lane,
non-null blocker on a `Migrated` route, lane/preparation mismatch) — all four are correctly rejected
by both the hand-rolled validator and the Ajv oracle.

**Caveat for context**: this `test` target is narrow by design — it is the TS-side interactive-job
route oracle that cross-checks the Rust command partition, not a full lowpoly integration-test
runner. It is the *only* test script the `@semio-tech/lowpoly-js` `package.json`/`📋️project.json`
define (`"test": "bun nx run @semio-tech/lowpoly-js:test"`, executor is a single `bun ./📜️script.ts
test` command). There is no broader "run every lowpoly TS test" nx target.

## 2. `bun ./📜️script.ts test discover`

Ran at repo root. **Exit 0.** Output ends with `[discover] 240 test case(s)` and the registry is
clearly populated (240 distinct test-case lines spanning stdio, trinity, vcs, writer, lowpoly, etc.)
— **not** the zero/empty-registry failure mode the known taxonomy `fileKinds`-drift hazard produces.
Grepping the discovery output for `lowpoly` found exactly the four expected ids, each correctly
tagged with its language set:

```
test-...-🌷️io-lowpoly-1        [rust]
test-...-🎮️command-lowpoly-1   [rust]
test-...-🧭️mutate-lowpoly-1    [rust,python]
test-...-🟩️io-lowpoly-png-1    [rust,python]
```

Discovery is healthy. **RED taxonomy-drift hazard did not manifest.**

## 3. Lowpoly-scoped TypeScript typecheck

**No `typecheck`/`tsc` nx target exists anywhere in the repo.** Verified with a repo-wide search
(`grep -rl '"typecheck"' --include=project.json --include=package.json .`, excluding
`node_modules`) — zero hits, not just for lowpoly. `@semio-tech/lowpoly-js`'s own
`📋️project.json`/`package.json` define only the `test` target; there is no lowpoly-scoped
`tsconfig.json` either (only the repo-root `./tsconfig.json`, which covers `**/*.ts` for the whole
monorepo and is not scoped to any one package).

Since nothing existed to reuse, I built a scoped tsconfig (kept in the session scratchpad, not the
repo) that copies the root `tsconfig.json`'s `compilerOptions` and narrows `include` to just
`✏️s/🔌️plugins/💠️lowpoly/**/*.ts` plus the one cross-cutting helper those files' tests import,
`/Users/ueli/Documents/semio/🗿️artifact.ts` (the `describe`/`it`/`expect` → vitest adapter). Ran with
`node_modules/.bin/tsc -p <scoped-config>` (repo's own installed TypeScript 5.9.3).

**Result on lowpoly's actual artifact/schema/io/editor/viewer/mutations TS tree: zero errors.**

Everything the scoped compile does flag falls into two buckets, neither of which is a lowpoly
defect:

- **Lowpoly's own `📜️script.ts` files** (`📦️packages/🟦️typescript/📜️script.ts`,
  `📦️packages/🦀️rust/📜️script.ts`) use `import.meta.dir`, a **Bun-only** global. Plain `tsc` doesn't
  know it because **`bun-types`/`@types/bun` is not installed anywhere in this repo's
  `node_modules`** (`find node_modules -maxdepth 1 -iname '*bun*'` → nothing but an unrelated
  `bundle-name` package; `@types/bun` absent). These scripts are never type-checked or bundled —
  they're executed directly by `bun`, which strip-compiles TS at runtime rather than checking it.
  This is a **repo-wide** tooling gap (confirmed the same `ImportMeta.dir`/bare-`Bun`-global errors
  recur in unrelated framework script files below), not something specific to or fixable from
  lowpoly.
- **Pre-existing errors in shared framework TS**, reached only because `📜️script.ts` imports the
  framework library bundle facade: `🧰️framework/🔨️modules/📡️replication/🟦️.ts`,
  `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` +`🖥️launch.ts`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`,
  `🧰️framework/🛍️products/💻️os/🟦️.ts`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`.
  These are genuinely broken per `tsc` (missing exports, `Uint8Array`/`BufferSource` narrowing,
  literal-type mismatches, implicit `any`, bare `Bun` global) but they are **outside lowpoly, in hot
  shared framework files** other sessions actively own — per instructions I did **not** touch them,
  only report them here.

### Proving the typecheck is real (not vacuous)

1. Injected a deliberate error at the end of lowpoly's own schema file,
   `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`:
   ```ts
   const __deliberateTypeErrorProbe: number = "not-a-number";
   ```
2. Reran the scoped `tsc` → new error appeared, exactly at that file/line:
   ```
   ✏️s/.../🧬️schema/🟦️.ts(135,7): error TS2322: Type 'string' is not assignable to type 'number'.
   ```
3. Reverted the edit (`git diff --stat` on the file → empty, confirming a clean revert).
4. Reran the scoped `tsc` once more → the injected error is gone; output is back to exactly the
   same baseline set of script.ts/shared-framework errors described above, nothing in lowpoly's own
   tree.

This confirms the scoped config genuinely type-checks lowpoly's files rather than silently skipping
them.

## Fixes applied

**None needed.** The `test` target passes for real, discovery is healthy with all four expected
lowpoly ids present, and a real (proven non-vacuous) scoped typecheck of lowpoly's own TS surface
comes back clean. The only broken TypeScript reachable from lowpoly's package lives in shared
framework code (listed above) and is out of this ticket's scope per the standing instruction not to
edit hot shared files from here.

## Investigation: are the nine lowpoly TS export serializers a lowpoly-specific gap?

**Finding: (a) — this is the repo-wide convention, not a lowpoly-specific gap. No serializers were
implemented.**

All nine lowpoly TS export-serializer files under
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/*/*/*/🟦️.ts`
(dwg, gltf, json, las, obj, ply, png, stl, txt) are the literal 1-line stub `export {};`, md5
`e2ebd7ddedcadeeadbf819c35985c768` — and so is every corresponding import-side deserializer file
under `📥️import/🧩️deserializers/…`. None of these files are even imported by anything: the actual
runtime-facing IO facade,
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️.ts`
(`LOWPOLY_IO_FORMATS`, `exportLowpolyMedia`/`importLowpolyMedia`, the host-bridge interface) doesn't
reference the `🧵️serializers`/`🧩️deserializers` subtree at all — the per-format Rust siblings
(`🦀️.rs`, confirmed to hold real logic) do the actual work.

Checked every sibling plugin the task named:

| Plugin | Artifacts checked | Export-serializer `🟦️.ts` files | md5 |
|---|---|---|---|
| **cad** | `📐️cad` | 8 files (json, stl, ifc, step, png, dwg, obj, gltf) | **8 distinct md5s** |
| **raster** | `🖨️raster` | 9 files (json, bmp, svg, jpg, pdf, tiff, png, dwg, gif) | all `e2ebd7dd…` |
| **remodel** | `📸️remodeling` | 9 files (json, stl, txt, png, dwg, ply, obj, las, gltf) | all `e2ebd7dd…` |
| **gis** | `🗺️gismap` + `🏔️gisterrain` (both artifacts) | 7–9 files each | all `e2ebd7dd…` |
| **puzzle** | `◻️2d`, `🧊️3d`, `🖐️5d` (all three artifacts) | 6–9 files each | all `e2ebd7dd…` |
| **lowpoly** | `💠️lowpoly` | 9 files | all `e2ebd7dd…` |

cad initially looked like the exception (every file has a *different* md5), but reading the content
shows why: cad's stubs are **still `export {};`** — the only difference is a one-line JSDoc comment
naming the format above the stub, e.g.

```ts
/** Serialize cad to stdio.step. */
export {};
```

vs. lowpoly/raster/remodel/gis/puzzle's bare `export {};` with no comment. That one-line comment is
what changes the md5 per cad file; functionally cad's serializers are equally empty. cad's
deserializers follow the identical pattern (`/** Deserialize cad from stdio.obj. */\nexport {};`,
etc.).

**Conclusion**: every plugin checked — cad included — ships empty placeholder TS modules for its
per-format IO serializers/deserializers, with real serialization logic living only in the Rust
sibling files. This is a deliberate (if minimal) repo-wide taxonomy convention — a `🟦️.ts` sibling
exists next to every `🦀️.rs` artifact leaf, populated later per-language — not a lowpoly regression.
No serializers were implemented on this finding, per instructions.

## Bottom line

- `bun nx run @semio-tech/lowpoly-js:test --skip-nx-cache` → **PASS**, exit 0, real assertions
  executed (not a no-op).
- `bun ./📜️script.ts test discover` → **PASS**, 240 cases total, all four expected lowpoly ids
  present, no taxonomy-drift red flag.
- Lowpoly-scoped `tsc --noEmit` (hand-built since no target exists) → **PASS** on lowpoly's own
  files, proven non-vacuous via inject/confirm/revert. Pre-existing errors exist only in
  `📜️script.ts` (Bun-only globals, no `bun-types` installed repo-wide) and in shared framework code
  outside lowpoly — reported above, not touched.
- Nine-serializer stub finding → **repo-wide convention (a)**, confirmed against cad, raster,
  remodel, gis (both artifacts), and puzzle (all three artifacts). Not a lowpoly-specific gap; no
  implementation work done.

**The lowpoly TypeScript/JS side is genuinely green as of this run**, with no code changes required.
Every edit made during this investigation (the temporary type-error probe in
`🧬️schema/🟦️.ts`) was reverted before this report was written; `git diff --stat` on that file is
empty.

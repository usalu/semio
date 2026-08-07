# 📋️ TEMPLATE-TS — folding a plugin's leftover TS module(s) into `@semio-tech/<plugin>-js`

Written by the W6 pilot (🌊️flow, ticket `26/08/06/FLOW-PLUGIN-TS-MODULES-AND-EXTENSIONS-CONSOLIDATION`) from
what actually happened folding flow's `🔨️modules/🧮️compute` bare-TS module. Read the master doc's "Plugin
residuals" bullet (Rule A) first; this file is the how.

Scope: this is for a plugin's `🔨️modules/*` **TS-only** residuals — the ones with no `⚡️implementations/🦀️rust`
sibling at all, just `⚡️implementations/🟦️typescript`. If a module has BOTH languages, see §3 below for the
placement ruling; the package-file mechanics (§1–2, §4–7) are identical either way.

---

## 0. Before you touch anything

1. Inventory: `find ✏️s/🔌️plugins/<p>/🔨️modules -type f`. Every TS-only module folds into ONE npm package;
   a module that already has a Rust `component.rs` sibling gets its TS file placed beside it instead (§3).
2. Read every file you're about to move **in full** — don't assume a verbatim move is safe. §8 below
   documents a real bug (`export { … } from "…"` doesn't create a local binding) the pilot found and fixed
   purely by reading the file being moved; a mechanical `mv` would have silently ported it forward.

---

## 1. Create the package files

```
✏️s/🔌️plugins/<p>/📦️packages/🟦️typescript/{package.json, 📋️project.json, 📜️script.ts, 🧪️vitest.config.ts, 📦️index.ts}
```

This sits as a **sibling of the plugin's existing `📦️packages/🦀️rust`** — same `📦️packages` parent, one
subfolder per language, exactly like the plugin-root Rust package. `package.json` name is
**`@semio-tech/<plugin>-js`** (flow → `@semio-tech/flow-js`); `bundleKind: "library"`; a `"semio"` field
`{ "role": "plugin", "id": "<plugin>-js" }`. Copy the shape of an already-migrated framework TS package for
field ordering/boilerplate (`author`/`license`/`repository`/`devDependencies.typescript`) —
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` is a clean worked
example. Compute the `$schema` relative-path depth correctly for wherever your plugin sits (count path
segments from repo root to the new dir — don't copy a wrong depth from another example verbatim, several
existing files in this repo have a stale one and it's silently harmless for tooling but still wrong).

`📋️project.json` — same **four leveled test targets** (`test`/`test-quick`/`test-long`/`test-exhaustive`)
as the Rust package's project.json, `cwd` = the new package dir, `command: "bun ./📜️script.ts test [level]"`.
`namedInputs.default` must list `{workspaceRoot}/✏️s/🔌️plugins/<p>/🔨️modules/**/*.ts` in addition to
`{projectRoot}/**/*` — the TS sources live OUTSIDE `projectRoot` (in the domain tree), same reasoning as the
Rust package's `**/*.rs` namedInput.

`📜️script.ts` — mirrors the Rust package's simple router, but calls `runVitest` instead of
`runCargoTestBudgeted`:

```ts
#!/usr/bin/env bun
/** 🌊️ `@semio-tech/<p>-js` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "<N ups>/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️implementations/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
```

Count `<N ups>` fresh for your plugin's depth — do not copy flow's `../../../../../` blindly; verify it
resolves (§7's smoke test catches a wrong depth immediately, as an unresolvable-module error).

`🧪️vitest.config.ts` is **required** — the root `🧪️vitest.config.ts` aggregator glob-discovers every
`🧪️vitest.config.ts` file in the repo by filename (see its own header comment: Vitest 4 requires the
literal ASCII prefix `vite`/`vitest`, so the emoji-prefixed filename is mandatory, not cosmetic) and folds
it into the workspace-wide `list`/`run`. Minimal shape:

```ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/<p>-js",
    environment: "node",                 // "jsdom" only if the module touches `document`/`window`
    include: ["📦️index.ts", "<relative path(s) to the moved component file(s)>"],
    includeSource: ["<relative path(s) to any moved file with an in-source `import.meta.vitest` block>"],
    coverage: { include: [/* same as include */] },
  },
});
```

`includeSource` is easy to forget and silently means your in-source `if (import.meta.vitest) { … }` blocks
never run — Vitest's default `include` glob only picks up `*.test.ts` files, not arbitrary source files
with an in-source block, unless `includeSource` lists them explicitly.

---

## 2. `📦️index.ts` — the package's own entry file

The npm package's entry file is a thin barrel, same idiom as a plugin's `📦️lib.rs` being wiring-only:

```ts
/** @emoji 🌊️ `@semio-tech/<p>-js` barrel — <plugin> plugin TypeScript module surface. */

export * from "../../🔨️modules/<module>/🟦️component.ts";
// … one line per TS-only module this package absorbs
```

Relative path is always `../../<rest>` from `📦️packages/🟦️typescript/` back up to the plugin owner root
(one `..` past `🟦️typescript`, one past `📦️packages`), then down into the domain tree — the same two-level
climb the Rust plugin's manifest deps use from `📦️packages/🦀️rust/`.

---

## 3. Placement ruling: TS-only domain vs. TS-beside-Rust

**Ruling: when a TS module duplicates logic that already has a Rust counterpart (e.g. cad's `🫀️core`), the
TS file lands as a sibling `🟦️component.ts` in the SAME component folder as the Rust `🦀️component.rs` — not
a separate domain-named folder.** Reasons, in order of weight:

1. **The plan says so explicitly.** The "Plugin residuals" merge rule states: "TS components land beside
   their Rust counterparts where they exist, TS-only domains keep their own dir" — this isn't a judgment
   call, it's already decided upstream.
2. **Shape V2's own normative shape draws the rule directly.** The `<owner>/…/<component>/` line in
   `📋️TEMPLATE.md`'s Shape V2 section literally shows `🦀️component.rs (+ 🟦️component.ts / 🐹️component.go /
   🐍️component.py siblings)` — same folder, sibling files, one per language. A separate TS-domain folder
   next to the Rust one would fork the taxonomy tree into two trees that both claim to represent the same
   concept, defeating the entire point of "duplicated logic sits together."
3. **The taxonomy validator keys off folder identity, not language.** `validateTaxonomyTree` (and the
   discovery contract more generally) treats a component folder as ONE node with N language files inside —
   putting the TS file in a sibling folder would make it invisible to any tooling that resolves "the compute
   engine for artifact X" by folder path, and would need its own new taxonomy folder name registered in the
   vocabulary for no semantic reason (there's no new concept, just a second language for an existing one).

Only when there is genuinely **no Rust component to sit beside** (flow's `🧮️compute` — nothing else in the
taxonomy tree represents "wasm thread-pool bootstrapping," it's a browser-only concern) does the module get
its own domain-named folder, one level under the owner root, named for what it actually is (not `⚡️implementations`,
not a generic "ts" bucket) — flow's case became `🔨️modules/🧮️compute/🟦️component.ts` (kept the existing
`🔨️modules/<domain>` folder from the old sandwich, since it was already a fitting, taxonomy-clean name; you
do not need to invent a new folder scheme for the TS-only case, just drop the `⚡️implementations/<lang>`
sandwich layer and land the file with the `component.<ext>` naming convention one level up).

If the old module had multiple files (not flow's case, but expect it elsewhere — cad's modules are larger),
each file becomes its own `<topic>/🟦️component.ts` the same way a Rust plugin's sibling-variant files each
became their own folder in the V2 retrofit (§14 of `📋️TEMPLATE.md`) — no bare multi-file TS module survives
flat inside `⚡️implementations/🟦️typescript/`.

---

## 4. Delete the old sandwich

`rm -rf ✏️s/🔌️plugins/<p>/🔨️modules/<module>/⚡️implementations` once the new component file(s) verify (§7).
Leave the `🔨️modules/<module>/` folder itself in place — it now directly contains `🟦️component.ts` (or a
`<topic>/` subfolder per §3), no `⚡️implementations` layer.

---

## 5. Root `package.json` workspaces — what you may and may not touch

Add exactly one new line, `"✏️s/🔌️plugins/<p>/📦️packages/🟦️typescript",`, to the root `workspaces` array.
This repo's registrar-protocol table normally reserves root `package.json` workspaces edits for the
registrar — **check your own dispatch instructions**: the W6 pilot's prompt explicitly carved out a
narrower exception ("only touch the workspaces array, nothing else in that file") because multiple W6
agents land TS packages in parallel and a full registrar serialization pass isn't worth it for a
single-line, low-conflict array append. If your own task prompt doesn't grant this exception, treat it as
registrar-owned and end with a handoff block instead (§9's schema, same shape). Either way: **never touch
anything else in `package.json`** (scripts, engines, other workspace entries), and **never run `bun
install`** — it rewrites `bun.lock`, which stays registrar-owned even under the workspaces-array exception.
Verify without installing: bun resolves an already-linked dependency like `@semio-tech/ui-react` from the
existing `node_modules` even before your new package is itself a registered workspace member, so `bun
./📜️script.ts test` run directly from the new package dir (§7) works without `bun install` in between.

---

## 6. A real bug the pilot found — read before you trust a verbatim move

Flow's `🧮️compute/📦️index.ts` had:

```ts
export { readStoredComputeWorkerCount, effectiveComputeWorkerCount, … } from "@semio-tech/ui-react";
// … later in the SAME file …
export async function initFlowThreadPool(init, requested = readStoredComputeWorkerCount()) { … }
```

**`export { X } from "module"` is a pure re-export — it does NOT create a local binding for `X`.** The
function body's bare use of `readStoredComputeWorkerCount()`/`effectiveComputeWorkerCount()` compiles fine
under a loose transpile-only pipeline (esbuild/Vite strip types without checking references) but throws
`ReferenceError: X is not defined` the instant it actually runs — confirmed with a real `tsc --noEmit`
(TS2304) AND a real `bun run` repro. This was **already broken before the move** (the pilot copied it
verbatim first, then caught it via a strict standalone `tsc` pass, not the package's own vitest run — the
vitest run alone reported "2 passed" because it only imports/transforms the file, it never calls the
function, since there was no test at all for it). Fixed by splitting into `import { X } from "…";` +
`export { X };`.

**Lesson: do not treat "the module imports and transforms cleanly under vitest" as proof the code is
correct.** Vitest/esbuild only prove syntax validity and module-graph resolvability, not that every
reference actually binds. Two things this initiative's TS folds should do that a plain relocation doesn't:
1. Run a real `tsc --noEmit` (with `--strict --moduleResolution bundler --allowImportingTsExtensions --jsx
   react-jsx --skipLibCheck`, pointed at just the moved file) and grep the output for **your own file's
   path** — the rest of the repo has plenty of unrelated pre-existing errors (a 41k-line `ui-react` barrel
   among them), so don't be alarmed by noise outside your file, but don't ignore a hit inside it either.
2. Add or extend an in-source `if (import.meta.vitest) { … }` test block that actually **calls** every
   exported function with at least one success and one failure-path case (see flow's `initFlowThreadPool`
   tests for the shape) — a real runtime exercise, not just an import.

---

## 7. Verification sequence

| # | Command | Notes |
|---|---|---|
| 1 | `bunx tsc --noEmit --strict --moduleResolution bundler --module esnext --target es2022 --skipLibCheck --allowImportingTsExtensions --jsx react-jsx <barrel path>` | grep the output for your own file paths only; noise elsewhere in the repo's type graph is expected and not yours to fix |
| 2 | `cd <new package dir> && bun ./📜️script.ts test` | if it hits the default `[budget]` wall-clock kill (15s — importing a big framework barrel like `ui-react` alone can take >10s), retry with `bun ./📜️script.ts test long` rather than treating it as a failure; do NOT lower the budget or strip the import |
| 3 | *(post-registrar / if you have the exception)* re-run once the root workspaces array has your line, to confirm `bun nx run @semio-tech/<p>-js:test` resolves the project | orchestrator/registrar territory otherwise |

Delete any temporary `[DEBUG]` scratch files from your ticket folder once the diff is clean; never leave
scratch scripts inside the plugin tree itself.

---

## 8. Registrar handoff (only if your dispatch does NOT grant the workspaces-array exception)

```
Add to root package.json "workspaces":
    "✏️s/🔌️plugins/<p>/📦️packages/🟦️typescript",
```

Nothing else — no Cargo.toml, no bun.lock (regenerated by the registrar's own `bun install` pass).

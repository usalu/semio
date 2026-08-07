# 📋️ Per-plugin TS-module-fold template (W6 Rule A)

Written by the W6 pilot (🌊️flow, ticket `26/08/06/FLOW-PLUGIN-TS-MODULES-AND-EXTENSIONS-CONSOLIDATION`)
from what actually happened folding flow's `🔨️modules/🧮️compute` residual into `@semio-tech/flow-js`.
Companion to `📋️TEMPLATE.md` (the Rust crate-merge recipe) and `📋️TEMPLATE-EXT.md` (the extension
de-sandwich recipe) — read the master doc's Discovery contract and Registrar Protocol sections first.

Use this when a plugin still has one or more `🔨️modules/<domain>/⚡️implementations/🟦️typescript/…`
sandwiches left over from an earlier Rust-only crate migration (per the plan's Rule A: "each plugin's
TS modules fold into ONE `@semio-tech/<plugin>-js` at plugin root, `📦️packages/🟦️typescript` beside
`🦀️rust`, TS components land beside their Rust counterparts where they exist, TS-only domains keep
their dir"). Flow was the simplest case (one bare, dependency-light module, zero pre-existing tests) —
plugins with several `🔨️modules/*` dirs (cad: 6 packages; remodel: 10; animate's 11k-line react
renderer) repeat §2-§4 once per module and may need a real `tsconfig.json`/build step §6 flags.

---

## 0. Before you touch anything

1. **Open your own ticket** (`ticket_open`, goal `🎯aioptimizedrepo`) if one isn't already reserved for
   you. Scratch files go in your ticket folder, never in the plugin tree.
2. **Inventory the plugin's TS residuals.** `find ✏️s/🔌️plugins/<p>/🔨️modules -type f` — every module's
   sandwich is normally `🔨️modules/<domain>/⚡️implementations/🟦️typescript/📦️index.ts`, a single flat
   file (no `src/`), same shape as the Rust sandwich this initiative is eliminating everywhere else.
3. **For each module, decide TS-only vs. Rust-twin** by checking whether the plugin's `🗿️artifacts/*`
   or `🎛️apps/*` domain tree has a same-named node. Flow's `🧮️compute` (thread-pool bootstrap helpers)
   has no Rust twin anywhere in flow's own tree — it calls into `@semio-tech/ui-react`, not into flow's
   own Rust — so it stayed TS-only. If your module instead duplicates logic that also exists as
   `<component>/🦀️component.rs`, it becomes `<component>/🟦️component.ts` in the SAME domain folder
   instead of keeping its own `🔨️modules/<domain>/` home (Rule A's "land beside their Rust
   counterparts" clause).
4. **Grep for consumers before moving anything**: `grep -rn "🔨️modules/<domain>" --include="*.ts"
   --include="*.tsx" .` (both the emoji path AND the module's exported symbol names — a consumer can
   import by symbol without the literal path showing up if it goes through a package name). Flow's
   `🧮️compute` had ZERO consumers repo-wide at the time of this migration — a genuinely orphaned
   file, not wired into any UI boot path yet. If yours has real consumers, you'll need to fix their
   import paths too (see §5).
5. **Read the file for latent bugs before moving it verbatim.** Flow's `🧮️compute/📦️index.ts` had a
   `export { X, Y, … } from "@semio-tech/ui-react"` re-export line immediately followed by an
   `import { X, Y, … } from "@semio-tech/ui-react"` used inside a function body in the SAME file — a
   bare `export { … } from "…"` re-export does **not** bind those names into the local module scope, so
   the local usage would throw `ReferenceError` at runtime the first time the (previously never
   exercised) function actually ran. Confirmed via a real `bun run`/`vitest` repro, not just reading —
   fixed by adding the missing `import` line (the `export { … }` line for external re-export can stay,
   or you can drop it if nothing outside needs the re-export; flow kept it since the package's own
   `📦️index.ts` re-exports through it). This is the exact class of forward-fix TEMPLATE.md §12.3
   documents for Rust ports — port TS bugs the same way, don't just relocate them.

---

## 1. Create the ONE npm package

```
✏️s/🔌️plugins/<p>/📦️packages/🟦️typescript/{package.json, 📋️project.json, 📜️script.ts, 📦️index.ts}
```

Copy the shape from an existing small framework TS package (`@semio-tech/ui-styling` at
`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/` is the closest reference — read it in
full, don't guess). Adjust per-package:

`package.json` — name `@semio-tech/<p>-js`; `"semio": { "role": "plugin", "id": "<p>-js" }`;
`"exports": { ".": "./📦️index.ts" }`; list every external npm dependency your moved TS source actually
imports (flow needed `"@semio-tech/ui-react": "workspace:*"` since `🧮️compute` calls into it — copy the
`"workspace:*"` version-string convention verbatim, it's universal in this repo); `devDependencies`
needs `typescript` always, plus `vitest` if you add a vitest config (§3). **Compute the `$schema` path
depth correctly** (`../` once per path segment from the file's own directory back to repo root) —
several already-migrated packages in this repo have a wrong depth here; it's non-functional (editors
don't hard-require it) but do it right anyway, don't copy a sloppy example's exact dot-count.

`📋️project.json` — name matches `package.json`; **all four leveled test targets**
(`test`/`test-quick`/`test-long`/`test-exhaustive`, each `nx:run-commands` → `bun ./📜️script.ts test
[level]`, `forwardAllArgs: true`) exactly like a Rust package, per TEMPLATE.md's `checkLeveledTestTargets`
requirement — this applies to TS plugin packages too, not just Rust ones. Add a `namedInputs.default`
override listing `{workspaceRoot}/✏️s/🔌️plugins/<p>/🔨️modules/**/*.ts` (or wherever your TS-only domain
folders end up) alongside `{projectRoot}/**/*`, the same reason Rust packages need one: sources live
OUTSIDE `projectRoot` once TS-only domains keep their own dir per Rule A, so nx would otherwise cache
stale results.

`📜️script.ts` — same `BundleScript`/`ScriptRouter`/`runBundleScriptMain` router shape as every other
package's script.ts. Import the shared lib from **`../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️implementations/🟦️typescript/📦️index.ts`**
— 5 levels up from a plugin's `📦️packages/🟦️typescript/📜️script.ts` (verify this fresh for your
plugin's actual path depth; don't copy flow's literal dot-count blindly — a plugin whose packages dir
sits one level deeper needs one more `../`, same rule as the Rust `📜️script.ts`'s already-fixed
depth-6-vs-5 bug documented in the master doc).

`📦️index.ts` — the ONE barrel, no logic of its own (same rule as `📦️lib.rs`): `export * from
"../../🔨️modules/<domain>/🟦️component.ts";` per moved domain — plain relative imports **with the
`.ts` extension written explicitly** (this repo's bun/TS config allows and consistently uses
`allowImportingTsExtensions`-style imports; every reference file does it this way, don't drop the
extension).

---

## 2. Move each module's source, de-sandwiched

`🔨️modules/<domain>/⚡️implementations/🟦️typescript/📦️index.ts` → `🔨️modules/<domain>/🟦️component.ts`
(a straight `mv` + rename, content otherwise verbatim modulo the §0.5 bug-fix pass). This is the TS
mirror of the Rust retrofit's "no sibling variant files" rule — one flat `component.<ext>` per concept
folder, no `⚡️implementations` sandwich, no nested `index.ts`. If the domain duplicates a Rust
component instead of standing alone, it lands as `<component>/🟦️component.ts` beside that component's
`🦀️component.rs` in the domain tree (`🗿️artifacts/<a>/…` or `🎛️apps/<app>/…`), not under `🔨️modules/`
at all — `🔨️modules/` is reserved for genuinely TS-only domains with no Rust counterpart anywhere.

Update the normalization the same pass: docstrings start with a single fitting emoji (per CLAUDE.md),
not an `@emoji` tag — several pre-existing files in this repo use `/** @emoji 🧵️ … */`, which is a
stale convention; `/** 🧵️ … */` is what every recently-written file (this template's own examples
included) actually uses.

---

## 3. Tests — in-source `import.meta.vitest`, not a separate `*.test.ts`

CLAUDE.md forbids creating new test FILES for new tests — extend existing files instead. For a
brand-new TS-only component with no pre-existing test file, the correct move is Vitest's in-source
testing feature (`if (import.meta.vitest) { const { describe, expect, it } = import.meta.vitest; … }`
appended at the bottom of the SAME `component.ts` file), not a sibling `*.test.ts`. This keeps one file
per component (matching the Rust taxonomy's own "one file per node" rule) while still getting real
test coverage. You'll need a `🧪️vitest.config.ts` in the package dir:

```ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    name: "@semio-tech/<p>-js",
    environment: "node",
    include: ["📦️index.ts", "../../🔨️modules/<domain>/🟦️component.ts", /* one entry per moved file */],
    coverage: { include: [/* same list */] },
  },
});
```

`include` must list source files directly (not a `*.test.ts` glob) for `import.meta.vitest` blocks to
be picked up — Vitest only strips/executes the in-source block for files matching `include`.
`script.ts`'s `TestScript` calls `runVitest(this.root, rest, "🧪️vitest.config.ts")` (same shared helper
every other vitest-backed package uses) rather than hand-rolling a bun-test invocation.

---

## 4. Registrar handoff — root `package.json` workspaces

**You cannot add the new package's workspace entry yourself** — root `package.json`'s `workspaces`
array is registrar-owned (same file-ownership table as root `Cargo.toml`, per the Registrar Protocol:
"root package.json workspaces + bun.lock | registrar"). End your report with the exact line(s) to add:

```
Add to root package.json workspaces:
    "✏️s/🔌️plugins/<p>/📦️packages/🟦️typescript",
Remove (if present, superseded by the line above):
    "✏️s/🔌️plugins/<p>/🔨️modules/<domain>/⚡️implementations/🟦️typescript",   # one per folded module, if it was ever listed
```

Until the registrar applies this, `bun install`/`bun nx run <pkg>:*` cannot resolve `@semio-tech/<p>-js`
by package name — but you CAN still verify directly: `cd` into the package dir and run `bun
./📜️script.ts test` (or `bunx vitest run --config 🧪️vitest.config.ts` directly if the script's
budget-wrapper kills a cold run early, see §6.1) without going through nx or bun workspace resolution
at all, since it's a plain relative-import script.

---

## 5. Fixing consumers (if any exist)

If §0.4's grep found real consumers, repoint their import from the emoji sandwich path to the new
package name (`@semio-tech/<p>-js`) or, if the consumer lives inside the SAME plugin's own tree, to the
new de-sandwiched relative path. Flow had zero consumers so this step was unexercised — treat it with
the same care as TEMPLATE.md §8.2's cross-cutting-dependent fix: grep both the literal old path AND the
exported symbol names (a consumer can import a symbol through a package-name alias without the emoji
path ever appearing in its own file).

---

## 6. Verification sequence

| # | Command | Notes |
|---|---|---|
| 1 | `cd <package dir> && bun ./📜️script.ts test` | fastest path; works even before the registrar adds the workspace entry, since it's a relative-import script, not a package-name resolution |
| 2 | `bunx vitest run --config 🧪️vitest.config.ts --passWithNoTests` (direct, no wrapper) | fallback for §6.1's cold-run budget kill; also useful if `bun nx run <pkg>:test` fails on an UNRELATED concurrent nx-graph error (§6.2) |
| 3 | *(post-registrar)* `bun nx run @semio-tech/<p>-js:test` | once the workspace entry lands |
| 4 | Delete any stray `node_modules/.vite/vitest/…` cache dir left inside the package folder by a direct `bunx vitest` invocation before finishing — it's gitignored and harmless, but there's no reason to leave build-tool cache sitting in a source tree you're handing off clean |

### 6.1 Cold-run budget false-negative

The shared `runTestBudgeted`/budget-wrapper in `📜️script.ts` kills a test run that exceeds its
level's budget (15s at `fundamental`). A cold Vitest invocation in a brand-new package (no warm
transform cache, first-time dependency pre-bundling of e.g. `@semio-tech/ui-react`) can legitimately
take longer than that on its FIRST run and get killed with `[budget] … exceeded 15000ms — killed`, even
though the same command finishes in under 10s on a warm second run. Don't read this as a real failure —
retry once, or run `bunx vitest run --config …` directly bypassing the wrapper, before concluding
anything is broken.

### 6.2 Unrelated nx-graph errors are not yours to fix

`bun nx run <pkg>:test` can fail with `Failed to process project graph … defined in multiple
locations` for a completely different project (flow's pilot hit this for `os-hub`, mid-flight from the
concurrent hub-consolidation wave) — this blocks the ENTIRE nx graph for every project, not just yours.
Same discipline as TEMPLATE.md's "unrelated repo-wide bug" precedent: verify via the direct
non-nx path (§6, row 1-2) instead of waiting for someone else's in-flight migration to settle, and
don't attempt to fix the other project's duplicate-registration yourself — it's outside your plugin's
ownership.

### 6.3 Live concurrent edits to the exact files you're creating

Because multiple W6 agents can be dispatched against different plugins at the same time, and an
auto-commit daemon periodically stages the whole working tree, don't assume a clean `find`/`git status`
snapshot stays true moment-to-moment. If you see files under your OWN plugin's tree that you didn't
create yet, re-read them before overwriting — flow's pilot found a fully-formed, correct, already-tested
`📦️packages/🟦️typescript` in progress mid-session from what turned out to be the SAME task running
concurrently; the right move was to audit and verify it, not discard it and redo from scratch.

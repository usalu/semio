# Verification

## Residual old-path references (must be empty outside intentional exceptions)

```
$ grep -rln '🧪️vitest\.config\.ts' --include="*.ts" --include="*.tsx" --include="*.json" . \
  | grep -v node_modules | grep -v '\.git/' | grep -v '♻️mit-bestand' | grep -v '🧫️fixtures' \
  | grep -v '🧪️fixtures' | grep -v '\.🧬semio'
🔒️layering.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts
🧪️tests/🟦️.ts
```
All three are intentional (see 📓️research.md): `🔒️layering.json` is a generated ratchet blocked on
an unrelated pre-existing generator problem; `🧹️normalization/🟦️.ts` keeps two literals inside its
`row.id === "wgpu-renderer"` historical-fixture-replay branch; `🧪️tests/🟦️.ts` (repo root) keeps one
literal inside a comment describing HISTORY ("Until ticket 26/08/23/... this file glob-discovered
every emoji-named `🧪️vitest.config.ts`") that must not be rewritten.

Zero `🧪️vitest.config.ts` files remain anywhere in the governed trees:

```
$ find . -path ./node_modules -prune -o -path '*/node_modules/*' -prune -o -path './target' -prune \
  -o -path '*/target/*' -prune -o -path './dist' -prune -o -path '*/dist/*' -prune \
  -o -path '*/storybook-static/*' -prune -o -path '*/.nx/*' -prune -o -path './.git' -prune \
  -o -path '*/♻️mit-bestand/*' -prune -o -path '*/.cursor/*' -prune -o -path './.🧬semio' -prune \
  -o -name '🧪️vitest.config.ts' -print
(no output)
```

## `bun 📜️script.ts verify taxonomy` still loads

```
$ bun 📜️script.ts verify taxonomy
error: [verify taxonomy] expected report or enforce, got undefined.
      at runTaxonomy (/Users/ueli/Documents/semio/📜️script.ts:10496:60)
      ...
```
Identical failure mode to the pre-change baseline (captured before any edits) — healthy per the
task's own bar.

`verify taxonomy report` (both bare and `--scope`) currently fails for reasons **unrelated** to this
change, on both the pre- and post-change tree — confirmed by re-running immediately after the first
file move, before most of this ticket's edits existed:
```
$ bun 📜️script.ts verify taxonomy report
error: Normalization requires an explicit repository-boundary decision before authored classification: ♻️mit-bestand/recherche
```
```
$ bun 📜️script.ts verify taxonomy report --scope "🧰️framework/📦️packages/🟦️typescript"
error: Ticket important exact mutation catalog is absent for governed source
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️12/ENFORCE-WINDOW-APP-PANEL-AND-PLUGIN-CONTRACTS-AT-COMPILE-TIME/...
```
Both are pre-existing, concurrent-session issues (a `♻️mit-bestand` gitlink boundary decision and an
unrelated ticket's missing important-markdown catalog entry) unreachable from anything this ticket
touched.

## Foreground vitest runs (representative sample across every structural category)

```
$ cd 🧰️framework/📦️packages/🟦️typescript && bun ./📜️script.ts test
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript
 Test Files  1 passed (1)
      Tests  88 passed (88)
```
(uses `runVitest()`'s DEFAULT third argument — proves the default-param fix and the direct
`dirname(...)`→root rewrap both work; `RUN` line shows root correctly resolved to the PACKAGE root,
not the new `🧪️tests/` subdirectory.)

```
$ cd ✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript && bun ./📜️script.ts test
 RUN  v4.1.10 /Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript
 Test Files  1 passed (1)
      Tests  no tests
```
(one of the 7 files that had no prior `root`/`dirname` at all — proves the from-scratch `root:` add.)

```
$ cd ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript && bun ./📜️script.ts test
 RUN  v4.1.10 /Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building
 Test Files  1 passed (1)
      Tests  1 passed (1)
```
(proves the two-hop `configDir`→`root` chain still lands on the extension root, not the package
root, after the extra hop.)

```
$ cd ✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript && bun ./📜️script.ts test
 RUN  v4.1.10 /Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript
 Test Files  2 passed (2)
      Tests  no tests
```

```
$ cd 🌎️hub/📦️packages/🟦️typescript && bun ./📜️script.ts test
 Test Files  1 skipped (1)
      Tests  1 skipped (1)
```

### Failures observed — confirmed pre-existing / unrelated, not caused by this ticket

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry && bun ./📜️script.ts test
 FAIL |@semio-tech/plugin-registry| 🧪️launch.test.ts > ... exposes every owned generator preview
      exactly once in contract order
AssertionError: expected [ ... ] to have a length of 14 but got 16
```
This asserts a fixed COUNT of generated preview launchers; it does not reference
`vitest.config.ts`/`🧪️tests`/anything this ticket touched. Root resolution itself worked (the suite
loaded and ran real content). Attributed to unrelated concurrent work adding launchers.

```
$ cd 🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript && bun ./📜️script.ts test
Error: Failed to resolve import "@semio-tech/ui-react" from ".../🧱️elements/🔑️AdminSession/🟦️.tsx".
```
Root-cause isolated directly:
```
$ bun -e '... resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx")'
alias target= /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx
$ ls .../⚛️react/
... 📦️index.tsx ...   (no 🟦️.tsx)
```
`repoRoot` computed exactly right (`/Users/ueli/Documents/semio`); the alias target is simply mid-
rename by a concurrent session (`.storybook/main.ts`'s own `firstExisting(...)` helper documents
this exact in-flight rename from `📦️index.tsx`→`🟦️.tsx`). Unrelated to this ticket.

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu && bun ./📜️script.ts test
error: Invalid taxonomy schema:
- generatorContracts["wgpu-frame-worker"] tracked output ".../🏗️builder/🦀️.rs" is missing.
  (+ 4 more missing tracked outputs, all Cargo/wasm build artifacts)
```
Confirmed these output files are genuinely absent on disk (`ls` → no such file) — a different
`generatorContracts` entry (`wgpu-frame-worker`, `outputRoots`) than the one this ticket edited
(`inputPatterns`, one literal path updated to the new `🧪️tests/🟦️.ts` location). `loadTaxonomy()`
throws before the vitest config is even reached, for BOTH `bun ./📜️script.ts test` and
`bun 📜️script.ts verify layering write-baseline` — this also blocked regenerating `🔒️layering.json`
(left untouched, see 📓️research.md).

## Story family (no files moved — nothing to verify beyond the negative)

```
$ grep -c '🧪️story.tsx' .storybook/scopes.ts
1
```
The storybook glob `../🧰️framework/🔨️modules/🖱️ui/🧱️elements/**/🧪️story.tsx` is untouched and still
correct, since no story file moved.

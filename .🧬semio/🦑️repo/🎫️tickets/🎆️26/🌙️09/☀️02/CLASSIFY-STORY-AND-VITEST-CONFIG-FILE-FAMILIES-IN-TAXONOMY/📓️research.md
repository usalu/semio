# Research: 🧪️story.tsx and 🧪️vitest.config.ts family classification

## Family A — `🧪️story.tsx` (42 files, `🧰️framework/🔨️modules/🖱️ui/🧱️elements/*/🧪️story.tsx`)

- `storyFileKindId` = `"typescript-source"` in `🔣️taxonomy.json`, so a story's canonical kind-only
  name is `🟦️.tsx` — identical to the sibling component entry (`typescript-react-entry`, also
  `🟦️.tsx`) that already occupies that slot in the same directory. Two files of the same kind
  cannot share one directory (`canonicalFilenamesForKind` returns one basename per kind), so a
  story cannot become `🟦️.tsx` in place.
- Checked every unrestricted (`parentKindIds`-free) `semanticDirectoryKinds` entry as a subdirectory
  host: `tests` (🧪️, slug `^(tests|oracle)$`) is the closest semantic match, but ~15 of the 42
  element directories already have a real `🧪️tests/🟦️.tsx` unit-test file — moving the story there
  would collide, and conflating a Storybook story with a unit test misclassifies it regardless.
  `examples` (📚️, slug `^examples$`) is structurally free but is a heavily-established, unrelated
  convention (artifact/subset interchange examples with their own `exampleFileKinds`/
  `exampleSlugPattern`/`🖼️assets`+`🧪️tests` shape, used in ~120+ places) — repurposing it for UI
  stories would conflate two different concepts under one directory kind. No other unrestricted
  kind's slug fits "story" semantics.
- This is a known, deliberate exception, not an oversight: `.storybook/scopes.ts`'s
  `HAND_CURATED_SCOPES["ui"].storyGlobs` comment (lines 77-81) explains that ticket
  `26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE` W7 already hit this exact collision and accepted
  `🧪️story.tsx` as the pragmatic filename, citing "the fixed single-leaf-filename taxonomy holds
  one story file per dir." No `fixedFilenameContracts` entry legitimizes this filename (unlike the
  literal, non-emoji `vitest.config.ts`, which has one with `authority: "Vitest"`).
- **Conclusion: left all 42 files untouched.** The registered vocabulary has no directory (or file)
  kind that admits a co-located story without inventing one. Fixing this cleanly needs either (a) a
  new `fileKinds` entry with its own emoji/extension so a story's canonical name differs from the
  component's `🟦️.tsx` (letting it coexist in the same directory), or (b) a new
  `semanticDirectoryKinds` entry (distinct emoji from `🧪️`/`📚️`) admitted under `elements`. Both are
  schema changes outside this ticket's mandate not to invent taxonomy vocabulary.
- No repo-wide references were touched for this family (storybook glob at `.storybook/scopes.ts:82`
  still correctly targets `**/🧪️story.tsx` since the files did not move).

## Family B — `🧪️vitest.config.ts` (30 files: 29 governed-tree + 1 repo root)

- `configurableEntryContracts["vitest-config-entry"]` already declares canonical filename `🟦️.ts`,
  `fileKindId: "typescript-source"`, `role: "tool-metadata"`, configured via
  `📜️script.ts:runVitest --config` (i.e. Vitest is always invoked with an explicit `--config` flag,
  never relying on its own default-filename discovery — confirmed by reading `runVitest()` in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`).
- Every one of the 29 package-root instances sits beside a `🟦️.ts` `typescript-library-entry` —
  renaming in place collides. `tests` (🧪️, slug `^(tests|oracle)$`, no `parentKindIds`) is proven,
  already-used precedent for hosting a `🟦️.ts`/`🟦️.tsx` file beside a package's own entry (e.g. every
  `🧱️elements/<Element>/🧪️tests/🟦️.tsx` unit test) and does not collide (none of the 29 package
  roots had a pre-existing `🧪️tests/` directory). Rejected `configuration` (🎚️, wrong domain — UI/
  mutation state fields, not tool configs) and `tools`/`build-tooling` (🛠️, wrong domain — editor
  tools / unused convention) as false-friend candidates.
- **Executed: moved all 30 files to `<owner>/🧪️tests/🟦️.ts`.**
- Every config computes its own directory via `dirname(fileURLToPath(import.meta.url))` and uses it
  (directly or via further `resolve(..., "../..")` chains) as the package/extension root for
  aliases, `includeSource`, `coverage.include`, and `root`. Moving the file one level deeper shifts
  every one of those chains by one hop, so every file's initial directory-capture was rewrapped as
  `resolve(dirname(fileURLToPath(import.meta.url)), "..")` — a single universal transform that keeps
  every downstream relative computation pointing at the exact same physical directory as before.
  7 files had no `dirname`/`root` at all (implicit default root = config's own directory) and needed
  the import + `root:` field added from scratch; verified none of their relative `include`/
  `includeSource` paths needed further adjustment beyond the added explicit root.
- Repointed every real reference: `runVitest()`'s default parameter, ~30 explicit
  `runVitest(this.root, ..., "🧪️vitest.config.ts")` call sites across `📜️script.ts` routers,
  `.vscode/settings.json`'s `vitest.configSearchPattern`, the `🔍️discovery/🟦️.ts` package-scaffold
  generator's default config filename, the `🧪️run-vitest-config-argument-tokens` self-test's
  expected literals, one `generatorContracts["wgpu-frame-worker"].inputPatterns` entry in
  `🔣️taxonomy.json`, and two basename-equality gates that can no longer key off a unique filename
  once it becomes the generic `🟦️.ts` (`🧹️normalization/🟦️.ts`'s `vitestConfigIncludeArrayTokens`
  gate, now `basename === "🟦️.ts" && parent dir === "🧪️tests"`; `🧪️index.test.ts`'s package-boundary
  walker, same check). Confirmed via a `find` sweep that dozens of unrelated
  `📚️examples/*/🧪️tests/🟦️.ts` files already exist repo-wide, so this directory+basename heuristic
  is intentionally broad but safe — the narrowing regexes inside only match real
  `includeSource`/`coverage.include` vitest keys, never firing false positives on those.
- **Deliberately left untouched** (matches the ticket's explicit two named frozen fixtures, and the
  same reasoning extended to the following, none of which are live filesystem reads):
  - `🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json`,
    `🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json` — explicitly named, byte-offset pinned.
  - `🧫️fixtures/🧪️nested-cargo-package-purity/🔣️.json`,
    `🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json`,
    `🧫️fixtures/🧪️nested-cargo-package-authority/🔣️.json`, and the sibling
    `🧪️fixtures/🧪️nested-cargo-package-purity/🔣️.json` — same SHA-256/byte-length-pinned shape,
    encoding a synthetic historical `wgpu-renderer`/`jcoprobe-guest` migration scenario consumed by
    `🧹️normalization/🟦️.ts`'s `row.id === "wgpu-renderer"`-scoped replay logic (lines ~5320-5340),
    which reads from `row.mappings`/`row.sourceRoot` supplied BY the fixture, never the live
    filesystem. Left both this replay logic and its two literal `"🧪️vitest.config.ts"` occurrences
    untouched — they test a frozen historical layout, not the current tree.
  - `🔒️layering.json` — a shrink-only ratchet whose own header says "Regenerate deliberately with
    `bun ./📜️script.ts verify layering write-baseline` ... never [hand-edit] to make a failure go
    away." Attempted the regen command; it currently fails (see Verification below) for a reason
    unrelated to this change, so the `"🧪️vitest.config.ts": 1` bucket is stale until that blocker
    clears and the command can be re-run.
  - `♻️mit-bestand/**` and every `.🧬semio/**` ticket artifact (explicitly excluded by the task).

### Side finding (out of scope, flagged separately)
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/` has BOTH a `🧪️fixtures/`
and a `🧫️fixtures/` directory (test-tube vs petri-dish emoji) — the same "🧪️ used where a different
kind is registered" pattern this ticket was scoped around, but for a directory rather than a file.
Not touched; flagged via `spawn_task`.

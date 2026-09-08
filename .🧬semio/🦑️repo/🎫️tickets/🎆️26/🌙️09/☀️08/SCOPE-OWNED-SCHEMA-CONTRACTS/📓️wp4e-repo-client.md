# WP4e — `🦑️repo` client: retiring `repo.client.vscode`, wiring the Go catalog (row 131)

Worker partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/**`.

Assigned: `📋️cross-partition-requests.md` **row 131** = `📓️wp3d-entity-catalog.md` §5 **R-1** (the exact vscode
edit), **R-4** (the unused Go projection) and **R-5** (`📓️wp4c-repo-product.md` §1.1/§4 superseded).

Repo MCP was down for the whole session: no MCP tool was called, no ticket was opened/closed/reopened,
`🗑️generated/` was not deleted, no git-modifying command was used, no worktree was created, no `cargo` was
run. Working logs went to the session scratchpad; every output below is pasted verbatim from a real run.

---

## 1. Headline

| Gate | Result |
|---|---|
| `test schema --under 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client` | **0 findings** (§4.1) |
| `tsc --noEmit -p tsconfig.json` (vscode extension package) | the **same 6** pre-existing extension errors as `📓️wp4c` §6.4, **0** new, **0** from the new import (§4.2) |
| `go build ./` + `gofmt -l` (`⌨️cli`) | clean (§4.4) |
| CLI Go suite | 67 `--- FAIL`, the same baseline as `📓️wp4b`, none of them this packet's (§4.4b) |
| vscode vitest | **there is no longer one** — the package's only vitest case was the deleted `🔬️schema` case; its config and the `runVitest` call went with it (§2.3) |

The substantive behaviour change: the CLI's `AllEntityEmojis()` answered **55** emojis where the catalog
declares **56** distinct ones. It silently omitted `📋️` (`file-template`). Proven at runtime, not argued (§4.3).

---

## 2. The vscode change (R-1)

### 2.1 `🧩️vscode/📦️packages/🟦️typescript/🟦️.ts`

Both imports at `:20`/`:21` are gone, replaced by one **extension-less** import of the generated projection
eight levels up (the depth `tsconfig.json`'s `paths` already spells):

```ts
import { ENTITY_KIND_BY_EMOJI } from "../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🤖️generated/🟦️entity-kinds";
```

and the whole `TECHNOLOGY_CATALOG` + `ENTITY_EMOJIS` block (`:1262-1280`) became R-1's one-liner:

```ts
export const ENTITY_EMOJIS: ReadonlyMap<string, string> = new Map([...ENTITY_KIND_BY_EMOJI].map(([emoji, kind]) => [emoji, kind.id]));
```

**One deviation from R-1's literal text, deliberate.** R-1 says to import `ENTITY_KINDS` as well. Nothing in
the extension consumes the ordered catalog: `TECHNOLOGY_CATALOG` had exactly three references and all three
were inside the replaced block (grep over the whole `🧩️vscode` tree), and no label/icon/kind-id lookup exists
in `🟦️.ts` (`grep -n "Infrastructure\|iconId\|technology-mono\|technology-user"` → one hit, the new
docstring). Importing it would leave an unused binding under `lint`'s `eslint --max-warnings 0`. When a
consumer for the ordered catalog appears, it imports `ENTITY_KINDS` from the same file.

`ENTITY_EMOJIS` keeps its `emoji → id` shape and its first-wins content, so `buildEntityEmojiPattern()`,
`ENTITY_ID_REGEX`, `:2915`'s decoration loop and `🧪️tests/🧩️extension/🟦️.ts` need no edit. The region banner
now names the real owner:

```ts
// Entity Emoji Registry MUST be projected from the framework-owned entity-kind catalog
// (`framework.schema` export `EntityKindCatalog`), never restated here.
```

### 2.2 Deleted outright — no alias, no redirect

- `🧩️vscode/🗂️technologies.json` (74 lines) — the instance document of a scope that no longer exists
- `🧩️vscode/🧬️schema/🔣️.json`, `🧩️vscode/🧬️schema/🟦️.ts` — the `repo.client.vscode` scope
- `🧩️vscode/🧪️tests/🔬️schema/🟦️.ts` — its only subject was the deleted scope
- `🧩️vscode/📦️packages/🟦️typescript/vitest.config.ts` — see §2.3

### 2.3 What "the nx project/targets that only served the deleted schema test" turned out to be

There is **no** nx project or target that only served it. `@semio-tech/repo-vscode` also carries
`dev`, `build`, `lint` and `build-vsix`, and `test`/`test-long`/`test-exhaustive` still run the extension-host
`vscode-test` case. What *did* only serve it were two things, both removed:

- `vitest.config.ts`, whose `include` was the single entry `../../🧪️tests/🔬️schema/🟦️.ts` and whose
  `passWithNoTests: false` would have made every `test` run fail once that file was gone.
- the `await runVitest(this.root, rest, "vitest.config.ts")` line in `📜️script.ts`'s `TestScript`
  (`runVitest` dropped from the import list; `TestScript.run` is no longer async).

`test-quick` is consequently a no-op now. It was **kept**: the four level targets are the repo-wide grouping
(`⌨️cli`, `🔌️mcp` and every other package carry the same four), and dropping one from a single package to
express "this package has nothing below `long`" would break the convention rather than express it.
No vitest case can replace the deleted one here: `🟦️.ts` has `import * as vscode from "vscode"` at module
scope, so the extension is only loadable inside the VS Code test harness.

### 2.4 The other two files R-1 names

- `tsconfig.json` `include`: `["🟦️.ts", "../../🧬️schema/🟦️.ts", "../../🧪️tests/🧩️extension/🟦️.ts", "../../🧪️tests/🔬️schema/🟦️.ts"]`
  → `["🟦️.ts", "../../🧪️tests/🧩️extension/🟦️.ts"]`. `resolveJsonModule` stayed (harmless; nothing in the
  package imports JSON any more).
- `🧩️vscode/README.md`: the three `🗂️technologies.json` / `🧬️schema` / `🧪️tests/🔬️schema` sections are
  replaced by one "Entity kind catalog" section that names `framework.schema` as the owner and the generated
  projection as what the extension consumes. This is inside the partition, so it was fixed rather than requested.

### 2.5 A build-graph edge R-1 did not mention, added here

`🤖️generated/🟦️entity-kinds.ts` is **git-ignored** (`.gitignore:91 **/🤖️generated/`), exactly like the Go
projection (`.gitignore:109 **/🐹️entity_kinds.g.go`); `git ls-files --error-unmatch` on the Go file errors
with "did not match any file(s) known to git". So on a fresh checkout the vscode bundle now cannot build
until `@semio-tech/framework-schema:generate` has run. Rather than leave that implicit (which is how the CLI
has been consuming its own generated file all along), both consumers now declare it:

| File | Targets given `dependsOn: [{target: "generate", projects: ["@semio-tech/framework-schema"]}]` |
|---|---|
| `🧩️vscode/📦️packages/🟦️typescript/📋️project.json` | `dev`, `build` |
| `⌨️cli/📦️packages/🟦️typescript/📋️project.json` | `dev`, `build`, `test`, `test-quick`, `test-long`, `test-exhaustive` |

plus `implicitDependencies: ["@semio-tech/framework-schema"]` on both. The `{target, projects}` form is the
one already used in the tree (`✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📋️project.json:12`). The CLI
test targets get the edge because its Go tests read the projection; the vscode test targets do not, because
`vscode-test` runs a prebuilt bundle and the `build` target already carries it.

---

## 3. The CLI decision (R-4): the Go projection is now consumed

**Verdict: the CLI does have exactly one entity-kind lookup, it was a hand-written duplicate, and it is now
the generated table. The Go target stays.**

### 3.1 Where the CLI resolves entity kinds/emojis today — the grep evidence

| Question | Command | Answer |
|---|---|---|
| Who calls the catalog-shaped API? | `grep -rn "AllEntityEmojis" ⌨️cli` | `🧩️component.go:1014` (definition) and `:1092` (`entity-emojis` command). Nothing else. |
| Does anything outside the CLI read it? | `git grep -n "entity-emojis"` over the tree | one hit: `🧩️component.go:1089`, the command's own `Use:`. No fixture, no test, no VS Code call site pins the output. |
| Do the catalog's kind ids exist in the CLI? | `grep -rn "technology-user\|technology-mono\|definition-implementation\|file-code\|interaction-started\|breach-scope" ⌨️cli --exclude=🐹️entity_kinds.g.go` | **0** hits before this change (the only hits now are the two lines of the test added in §3.3). The CLI's `EntityKinds` (`🧩️component.go:5342`) is a *coarser*, unrelated vocabulary — `"technology"`, `"bundle"`, `"file"` — and `interactionKindFromEmoji`/`technologyKindCodeFromEmoji` map to CLI-local strings (`"edited"`, `"i"`), not to catalog ids. |
| Are the `Emoji*` constants a catalog duplicate to delete? | `grep -ro "Emoji[A-Z][A-Za-z]*" ⌨️cli --exclude=🐹️entity_kinds.g.go \| wc -l` | **1684** occurrences. They are the CLI's **entity-id prefix vocabulary** (`(*Checkpoint).GetID`, `(*Breach).GetID`, path→id flattening, …), 20-odd of them have no catalog counterpart at all (`EmojiRepo`/`EmojiTerritory` are `""`, `EmojiSections`/`EmojiYears`/… are plural forms), and they are `const`. They are **not** the duplicate row 131 is about and were left alone — see open question 1. |

So the duplicate was the 57-line `add(Emoji…)` sequence inside `AllEntityEmojis()`, and nothing else.

### 3.2 The rewrite

`🧩️component.go:1014` is now 14 lines that range over the generated `EntityKindCatalog`, keeping the two
behaviours the old body had — `emojiText` normalization and first-wins dedup:

```go
func AllEntityEmojis() []string {
	seen := map[string]bool{}
	result := make([]string, 0, len(EntityKindCatalog))
	for _, kind := range EntityKindCatalog {
		normalized := emojiText(kind.Emoji)
		if normalized == "" || seen[normalized] {
			continue
		}
		seen[normalized] = true
		result = append(result, normalized)
	}
	return result
}
```

No `Emoji*` constant became unused: `EmojiFileTemplate` (`:9554`, `:41360`), `EmojiBreachScope` (`:42258`)
and `EmojiFolderRoot` (`:41373`) all still have call sites outside this function.

### 3.3 The test that makes the duplicate unrecoverable

`⌨️cli/🧪️tests/🔬️component/🐹️.go` gains `TestAllEntityEmojisProjectsTheFrameworkCatalog`. It holds the CLI
against the **language-agnostic catalog document** — it reads
`🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json` off disk and decodes it independently — not against the
Go projection it is built from, so a stale `🐹️entity_kinds.g.go` and a hand-written list are both caught:

1. the generated projection is entry-for-entry identical to the JSON (id and emoji, in order);
2. `AllEntityEmojis()` equals the JSON's first-wins normalized emoji sequence, element for element;
3. `📋️`, `🔍️` and `💾️` are present (`📋️` is precisely what the old list dropped);
4. `EntityKindByEmoji("🌱️")` resolves to `technology-mono`, pinning first-wins on the Go side the way
   `📓️wp3d` §4.2 pins it on the Rust side and §4.4 on the TypeScript side.

---

## 4. Verification — real output

### 4.1 Schema harness, this partition

Run twice — once mid-session and once as the last action of the session, after every peer commit landed:

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --under 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client
[test schema] 0 invariant finding(s) over 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client
[test schema] 0/0 schema-bound fixture(s) reached their declared stage
exit=0
```

### 4.2 TypeScript — `bunx tsc --noEmit -p tsconfig.json` in `🧩️vscode/📦️packages/🟦️typescript`

527 errors, **all** of them reached through the pre-existing `@semio-tech/framework` import graph
(`ephemeralBox`/`ephemeralMap`), dominated by `191 × TS5097` and `131 × TS7006` in framework modules. The
only rows whose file is in this package:

```
🟦️.ts(874,17): error TS2391: Function implementation is missing or not immediately following the declaration.
🟦️.ts(876,12): error TS2552: Cannot find name 'documents'. Did you mean 'document'?
🟦️.ts(1556,154): error TS2345: Argument of type 'undefined' is not assignable to parameter of type 'OutputChannel'.
🟦️.ts(1560,183): error TS2345: Argument of type 'undefined' is not assignable to parameter of type 'DiagnosticCollection'.
🟦️.ts(1564,181): error TS2345: Argument of type 'undefined' is not assignable to parameter of type 'DiagnosticCollection'.
🟦️.ts(3084,173): error TS2345: Argument of type 'undefined' is not assignable to parameter of type 'TextEditorDecorationType'.
```

These are `📓️wp4c` §6.4's "six long-standing errors in the extension itself" at `875,877,1571,1575,1579,3099`,
shifted by exactly the lines this change removed (−1 above `:875` from collapsing two imports into one, −15
more from the `TECHNOLOGY_CATALOG` block). **Zero** new errors; **zero** rows naming
`🤖️generated/🟦️entity-kinds.ts`; **no** TS5097 on the new import, which is deliberately extension-less
(`moduleResolution: "Bundler"` without `allowImportingTsExtensions`).

### 4.3 The drift the rewrite fixed — measured, not argued

A `[DEBUG]` probe compiled into the package through the same Go overlay (file kept in the session scratchpad,
never written into the repo) reconstructs the old hand-written `add(...)` list beside the new implementation:

```
=== RUN   TestDebugLegacyListDivergesFromCatalog
    zz_debug_probe_test.go:39: [DEBUG] legacy=55 current=56 catalog=58
    zz_debug_probe_test.go:46: [DEBUG] present now, missing from the legacy hand-written list: "📋"
--- PASS: TestDebugLegacyListDivergesFromCatalog (0.02s)
```

58 catalog kinds → 56 distinct emojis after first-wins dedup (`🌱️` and `📝️` are each shared by two kinds);
the hand-written list answered 55. The new test's `EmojiFileTemplate` assertion fails against the old body.

### 4.4 Go build / format / the new test

```
$ gofmt -l 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli
internal/command/🎮️command.go            # pre-existing, not this partition's edit
                                          # 🧩️component.go and 🧪️tests/🔬️component/🐹️.go are clean

$ (cd ⌨️cli && GOWORK=<root>/go.work go build ./)
exit=0

$ (cd ⌨️cli && go test -overlay=<plan overlay> -short -timeout 300s \
     -run 'TestAllEntityEmojisProjectsTheFrameworkCatalog|TestEntityKinds$|TestArtifactKinds' -v .)
=== RUN   TestEntityKinds
--- PASS: TestEntityKinds (0.00s)
=== RUN   TestAllEntityEmojisProjectsTheFrameworkCatalog
--- PASS: TestAllEntityEmojisProjectsTheFrameworkCatalog (0.00s)
=== RUN   TestArtifactKinds
--- PASS: TestArtifactKinds (0.00s)
PASS
ok  	github.com/usalu/semio/repo/client	0.895s
```

The overlay is `canonicalGoTestPlan()`'s own plan (`📚️library/📦️packages/🟦️typescript/🟦️.ts:1177`), written
out with `bun -e`, because the canonical runner's 15 s `fundamental` budget cannot compile a 1.5 MB
`🧩️component.go` from cold:

```
$ bun ./📜️script.ts test -run 'TestAllEntityEmojis…' -v      # in ⌨️cli/📦️packages/🟦️typescript
[budget] go test -overlay=… -timeout 15s -short -skip ^Test(Quick|Long|Exhaustive) … exceeded 15000ms — killed.
```

That budget wall is pre-existing and not a finding of this packet — see open question 3.

### 4.4b The CLI Go suite, enumerated packages

```
$ (cd ⌨️cli && GOWORK=<root>/go.work go test -overlay=<plan overlay> -short -timeout 900s \
     -skip '^Test(Quick|Long|Exhaustive)' . ./internal/command ./internal/eventstore)
FAIL	github.com/usalu/semio/repo/client	186.742s
ok  	github.com/usalu/semio/repo/client/internal/command	0.530s
ok  	github.com/usalu/semio/repo/client/internal/eventstore	0.852s
exit=1

$ grep -c '^--- FAIL' <log>
67
```

**67** `--- FAIL` lines, against `📓️wp4b`'s 67-line baseline for the same target (`📓️wp4c` §6.5 measured 72 at
level `long`, which runs strictly more). **None of the 67 is mine** — the full list of failing top-level tests
was enumerated and contains no entity-emoji or entity-kind case, and `TestAllEntityEmojisProjectsTheFrameworkCatalog`
is absent from it (it passes; §4.4 runs it explicitly with `-v`). `grep -n "AllEntityEmojis\|EntityKind\|Emoji"`
over the whole 67-failure log returns **0 lines**. Spot-checking the two id-shaped failures confirms the
known causes:

```
--- FAIL: TestSpecExactIDs/years        years: expected "🎆️", got "🎆"          # the VS16-stripping regression 📓️wp4c §6.5 already recorded
--- FAIL: TestBundleListIDs             expected bundle "compose/js" not found in list  # CLI bundle discovery finds none of the current layout
```

The literal `./...` form of row 131's command was also run and is reported in open question 4.


### 4.5 The generator still agrees after the deletion

`🗂️technologies.json` was **not** the generator's source (that is
`🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json`, restored by `📓️wp3c`), so deleting it changes nothing
the generator reads:

```
$ (cd 🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust && bun ./📜️script.ts check)
entity catalog is fresh (58 entity kinds, sha256 f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242).
```

It *was* declared as a generator **input pattern** in the taxonomy, which is now a dangling path — request
**W4e-2**.

### 4.6 The vscode bundle — the new import resolves

```
$ (cd 🧩️vscode/📦️packages/🟦️typescript && bun ./📜️script.ts build)
vite v7.3.6 building client environment for production...
transforming...
✓ 279 modules transformed.
✗ Build failed in 4m 5s
RollupError: Module format "cjs" does not support top-level await. Use the "es" or "system" output formats rather.
file: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts
   id: "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"
 code: "INVALID_TLA_FORMAT"
exit=1
```

Byte-for-byte the failure `📓️wp4c` §6.4 recorded, in a framework module reached through the pre-existing
`@semio-tech/framework` import, and unchanged by this packet. It matters here for one reason: rollup got past
`transforming` (279 modules) to the **render** stage, which proves every module in the graph resolved — the
new `🤖️generated/🟦️entity-kinds` import included. The previously-inlined `🗂️technologies.json` is gone from
the graph and nothing complains about its absence.


---

## 5. Files changed

### Deleted

| File | Why |
|---|---|
| `🧩️vscode/🗂️technologies.json` | instance document of the retired `repo.client.vscode` scope |
| `🧩️vscode/🧬️schema/🔣️.json` | the retired scope (`TechnologyCatalog`, `TechnologyCatalogEntry`) |
| `🧩️vscode/🧬️schema/🟦️.ts` | its TypeScript facet and parsers |
| `🧩️vscode/🧪️tests/🔬️schema/🟦️.ts` | its only subject was the retired scope |
| `🧩️vscode/📦️packages/🟦️typescript/vitest.config.ts` | its `include` was that one case; `passWithNoTests: false` |

### Edited

| File | Change |
|---|---|
| `🧩️vscode/📦️packages/🟦️typescript/🟦️.ts` | R-1: two imports → one generated import; `TECHNOLOGY_CATALOG` deleted; `ENTITY_EMOJIS` projected from `ENTITY_KIND_BY_EMOJI`; region banner renamed |
| `🧩️vscode/📦️packages/🟦️typescript/tsconfig.json` | the two dangling `include` entries removed |
| `🧩️vscode/📦️packages/🟦️typescript/📜️script.ts` | `runVitest` call and import dropped; `TestScript.run` no longer async; docstring rewritten |
| `🧩️vscode/📦️packages/🟦️typescript/📋️project.json` | `dev`/`build` `dependsOn` + `implicitDependencies` on `@semio-tech/framework-schema` |
| `🧩️vscode/README.md` | three sections describing the retired scope → one section naming `framework.schema` as the owner |
| `⌨️cli/🧩️component.go` | `AllEntityEmojis()` (57 `add(...)` lines) → 14 lines over the generated `EntityKindCatalog` |
| `⌨️cli/🧪️tests/🔬️component/🐹️.go` | **new** `TestAllEntityEmojisProjectsTheFrameworkCatalog` (§3.3) |
| `⌨️cli/📦️packages/🟦️typescript/📋️project.json` | six targets `dependsOn` + `implicitDependencies` on `@semio-tech/framework-schema` |

Nothing under `💻️client/🔌️mcp/**` or `💻️client/🪶️sqlite/**` was touched.

---

## 6. Cross-partition requests

Ledger state at hand-over: row **131** is `done(W9f)`; **W4e-1** and **W4e-2** are already ledger
row **144** (raised to W2w library); **W4e-4** was appended as row **146**.

### W4e-1 — `📚️library/🔣️schema-catalog.json`: delete the `repo.client.vscode` block (tooling worker, W2c)

`🔣️schema-catalog.json:13263-13286` still declares the scope this packet removed, with hashes of two files
that no longer exist:

```json
"repo.client.vscode": {
  "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧬️schema",
  …
  "hashes": { "🔣️.json": "92e263b3…", "🟦️.ts": "29d89b30…" }
}
```

Delete the whole property. Nothing replaces it — the exports moved to `framework.schema`, whose catalog entry
is the framework partition's to refresh. `📚️library/📓️schema-catalog.md` mentions the scope as well and needs
the same removal.

### W4e-2 — `📚️library/🔣️taxonomy.json`: `generatorContracts.schema-entity-catalog.inputPatterns` (W2c) — now mandatory

`🔣️taxonomy.json:24545` still lists
`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🗂️technologies.json` as a generator input. That
path no longer exists. `📓️wp3d` **R-2** already specified the replacement and it is unchanged; this packet
turns it from a correctness improvement into a dangling reference:

```json
"inputPatterns": [
  "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json",
  "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🧬️schema/🟦️.ts",
  "🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json"
]
```

`outputRoots` is already correct and was not touched.

### W4e-3 — `📓️wp4c-repo-product.md` §1.1 / §4 are superseded (coordinator) — R-5, done from this side

The `repo.client.vscode` scope, its two exports and its `ENTITY_EMOJIS` derivation no longer exist. `📓️wp4c`'s
analysis survives as `📓️wp3d` §2.2 (the bounds table became `EntityKind`'s) and §3 (its first-wins finding is
why all three projections were corrected). `📓️wp4c` §5 (`🔌️mcp`, `repo.client.mcp`) is **not** superseded and
was not touched here.

### W4e-4 — `.vscode/launch.json`: `@semio-tech/repo-vscode` has no entries at all (launch.json owner)

`grep -n "repo-vscode" .vscode/launch.json` returns nothing, while `repo-mcp-go` and `repo-cli-rs` have
`build`/`dev`/`test` entries. CLAUDE.md requires every executable command to be registered there. This is
pre-existing (the package's six targets were never registered) and is not caused by this packet, but the
`test` target's meaning changed here, so it is worth registering the set in one pass rather than piecemeal.

---

## 7. Open questions / deliberately not acted on

1. **The CLI's 1684 `Emoji*` constant references are still hand-written emoji literals.** They are a
   different contract from the catalog — the entity-**id prefix** vocabulary used by `GetID()`, path
   flattening and id parsing — and ~20 of them have no catalog counterpart (`EmojiRepo`/`EmojiTerritory` are
   `""`, plus the plural forms `EmojiSections`/`EmojiYears`/`EmojiTodos`/…). Where they *do* overlap with the
   catalog they can silently disagree with it, exactly as `AllEntityEmojis()` did. Deriving the overlapping
   subset from `EntityKindCatalog` means turning a `const` block into `var`s and is a package-wide change to a
   1.5 MB file several peers are editing; it deserves its own row rather than a drive-by. A cheap interim
   guard would be one test asserting `EmojiX == EntityKindByEmoji(...)` for the ~50 constants that do have a
   catalog kind.
2. **`test-quick` on `@semio-tech/repo-vscode` now runs nothing** (§2.3). Kept for grouping. If the
   coordinator prefers the honest shape, the fix is to drop `test-quick` from that one `📋️project.json` — but
   it should then be a repo-wide rule about packages whose only case is above `quick`, not a one-off.
3. **The CLI's Go suite cannot be measured at the `fundamental` budget.** `runCanonicalGoTests` at level
   `fundamental` gives the whole run 15 s, and compiling `🧩️component.go` alone exceeds that from cold, so
   `bun ./📜️script.ts test` reports a budget kill rather than a test result (§4.4). Everything below `long`
   is therefore un-runnable for this package on a cold cache. Not this packet's to fix; it is the reason the
   runs above go through the plan's overlay directly.
4. **`go test ./...` does not work in this module** and never did: four packages fail at setup with
   `malformed import path … invalid char '🧪'` because Go rejects emoji in import paths. That is exactly why
   `canonicalGoTestPlan` enumerates `.`, `./internal/command`, `./internal/eventstore` explicitly and overlays
   the `🧪️tests/<case>/🐹️.go` files into their owner packages as `zz_semio_<hash>_test.go`. Row 131's
   `go test -short ./…` is therefore satisfied by the enumerated form; the literal `./...` form is reported in
   §4.4 for completeness.
5. **Both generated projections the client consumes are git-ignored** (§2.5). The `dependsOn` edges added
   here make `nx build` correct, but a plain `go build`/`vite build` outside nx on a fresh checkout still
   fails until `generate` has run. If that is not acceptable, the alternative is `inclusion: "tracked"` in the
   taxonomy's `outputRoots` for both files — a framework/tooling decision, not a client one.

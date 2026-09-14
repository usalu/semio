# 📓️ Opus `foundation` — `🏠️workspace`, `🪪️identity`, `🧾️yaml`, `🔎️search`

Wave 1. Four L0 modules of plan §2 are now implementation-neutral owners with a Go package, a Rust
twin crate, a JSON Schema, immutable fixtures and language-agnostic Protocol v2 cases. Everything
below was executed; raw command output is in `🗑️generated/foundation/{go,cargo,harness}.txt` and
reproduced verbatim in §5.

## 1. What exists now

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/
├── 🏠️workspace/   📦️packages/{🐹️go,🦀️rust}  🔮️oracle/🔣️.json  🧬️schema/🔣️.json  🧫️fixtures/×3  🧪️tests/×5
├── 🪪️identity/    📦️packages/{🐹️go,🦀️rust}  🔮️oracle/🔣️.json  🧬️schema/{🔣️.json,🔣️entity-emojis.json}  🧫️fixtures/×1  🧪️tests/×2
├── 🧾️yaml/        📦️packages/{🐹️go,🦀️rust}  🔮️oracle/🔣️.json  🧬️schema/🔣️.json  🧫️fixtures/×1  🧪️tests/×2
└── 🔎️search/      📦️packages/{🐹️go,🦀️rust}  🔮️oracle/🔣️.json  🧬️schema/🔣️.json  🧫️fixtures/×1  🧪️tests/×1
```

Each `📦️packages/🐹️go` holds `go.mod` (`github.com/usalu/semio/repo/<suffix>`, `go 1.25`, no external
deps), `🐹️.go`, `🧪️_test.go`, `📋️project.json`, `📜️script.ts`. Each `📦️packages/🦀️rust` holds
`Cargo.toml` (`semio-framework-repo-<slug>`, `[lib] path = "🦀️.rs"`, `[lints] workspace = true`,
`[package.metadata.semio] role = "library"`, deps `serde`/`serde_json` workspace-only), `🦀️.rs`,
`📋️project.json`, `📜️script.ts`.

### Go module content

| Module | Moved from | Public surface |
| --- | --- | --- |
| `workspace` | `internal/glob` + `internal/ignore` | `Match`, `FilepathGlob`, `FilepathGlobContext`, `GitIgnore`, `CompileIgnoreFile`, `CompileIgnoreLines`, `MatchesPath` — plus NEW `SemioDirName`/`RepoDirName`/`TicketsDirName`/`GoalsDirName`/`DevsDirName`/`FilesIndexName`/`ConfigFileName`, `SemioDirForRoot`, `RepoMetaDirForRoot`, `RepoMetaPathForRoot`, `TicketsDirForRoot`, `GoalsDirForRoot`, `DevsDirForRoot`, `FilesIndexForRoot`, `FindRepoRoot`, `RepoConfig`, `LoggingConfig`, `DefaultRepoConfig`, `LoadRepoConfig`, `ParseRepoConfig`, `IncludeResponse`, `IncludeNative` |
| `identity` | `internal/id` + `internal/humanize` | `ID`, `New`, `NewFrom`, `Entropy`, `PlatformEntropy`, `SeededEntropy`, `Time`, `TimeAt`, `Duration`, `Clock`, `SystemClock`, `FixedClock` — plus the emoji codec (see §3) |
| `yaml` | `internal/yaml` verbatim | `Marshal`, `Unmarshal` |
| `search` | `internal/search` verbatim | unchanged |

`internal/{ignore,glob,yaml,search,id,humanize}` are deleted. Only the import block of
`💻️client/⌨️cli/🧩️component.go`, `🔬️component_test.go` and `🤝️g1_contract_test.go` was edited
(alias kept, path repointed); `💻️client/⌨️cli/go.mod` gained four `require` + `replace` pairs, and
root `go.work` lists the four new module directories. Root `Cargo.toml` lists the four new crates.

## 2. Symbols the `go-split` agent should MOVE into `workspace` (Go copy still in the godfile)

Per instruction I ported the root-discovery/layout/config region to Rust and wrote its Go twin in the
new module, but left the godfile untouched apart from imports. The split agent should delete these
from `🧩️component.go` and repoint call sites at `workspace`:

| Godfile symbol (snapshot ~16751–17777) | Replacement in `github.com/usalu/semio/repo/workspace` |
| --- | --- |
| `findRepoRoot` | `FindRepoRoot` |
| `GetRootDir` / `SetRootDir` | keep the mutable process-global in the CLI; it must call `workspace.FindRepoRoot` |
| `GetSemioRootDir` | `SemioDirForRoot(GetRootDir())` |
| `RepoMetaDirForRoot` | `RepoMetaDirForRoot` (identical) |
| `GetRepoMetaDir` / `GetRepoMetaPath` | `RepoMetaDirForRoot` / `RepoMetaPathForRoot` |
| `RepoConfig`, `LoggingConfig`, `DefaultRepoConfig`, `LoadRepoConfig`, `parseRepoConfigBool`, `unquoteRepoConfigValue` | same names, exported; parsing split out as `ParseRepoConfig(document)` so it is testable without a filesystem |
| `LoggingConfig.includeResponse` / `.includeNative` | `IncludeResponse` / `IncludeNative` |
| `.🧬semio`, `🦑️repo`, `🎫️tickets`, `🎯️goals`, `🧑️‍💻️devs`, `📁️files.json`, `📋️config.toml` literals | the `*DirName` / `*Name` constants |

`GetRepoGoalsDir`, `ListContributors` and the technology table stay outside this module (they belong
to `🎯️goals`, `🧑️contributors`, `📐️model`) — the `identity` codec below is deliberately free of them.

## 3. `🪪️identity` — the shared emoji table

`🧬️schema/🔣️entity-emojis.json` (`semio.repo.identity.entity-emojis/1`) is the single source of the
vocabulary: `textDefaultEmojis`, `entities`, `collections`, `allEntityOrder`, and the four artifact
reference prefix lists. Every emoji is stored WITHOUT U+FE0F; the codec re-adds it only for the
text-default code points, which is exactly what the godfile's `emojiText` did.

* Rust loads it with `include_str!("../../🧬️schema/🔣️entity-emojis.json")` and decodes once into a
  `OnceLock`.
* Go loads it from a module-relative path derived from `runtime.Caller(0)`, with the
  `SEMIO_REPO_IDENTITY_EMOJI_TABLE` environment override for a `-trimpath` build. `//go:embed` is not
  usable: the schema lives above the Go module root.

Ported to BOTH languages (pure string/rune logic, snapshot ~41084–43194): `emojiText` → `EmojiText`/
`emoji_text`, `extractEntityEmoji` → `ExtractEntityEmoji`/`extract_entity_emoji`, `isEmojiRune`
(rewritten as a shared closed-range table), `SemanticId`, `ArtifactRef` + `ParseArtifactRef`,
`goalPathToComposeID`/`composeIDToGoalPath` (the filesystem-scanning half is replaced by
`ComposeIDToGoalSegments`, which returns the flattened segments — resolving them back to directory
names belongs to `🎯️goals`), `contributorGithubToComposeID`/`composeIDToContributorGithub` (same
split: `ComposeIDToContributorFlat`), `AllEntityEmojis`, `Flat`, `NormalizePath`, `ParseSemanticIds`.

The per-kind emoji functions (`technologyKindEmoji`, `bundleKindEmoji`, `fileKindEmoji`,
`folderKindEmoji`, `definitionKindEmoji`, `interactionKindEmoji`, the `*KindCode*` family) were NOT
ported: they take `map[string]interface{}` of `📐️model` types and belong with that module. Their
emoji constants are already in the table under the same names (`technology-user`, `bundle-schema`,
…), so the `📐️model` executor only needs `identity.Entity("<kind>")`.

Identifier generation is behind an `Entropy` trait/interface so the harness can inject a seed:
`SeededEntropy` is a splitmix64 stream, identical in both languages, and the case pins four seeds.

## 4. Deliberate behaviour decisions and defects found

1. **Go glob was byte-oriented and broke on non-ASCII patterns (FIXED).** `compile` iterated
   `pattern[index]` and passed single bytes through `regexp.QuoteMeta(string(b))`, which reinterprets
   a UTF-8 continuation byte as a code point. `**/📜️script.ts` therefore did not match
   `🧰️framework/📜️script.ts`. Found by the `🃏️glob-matching` case (Rust and micromatch both said
   `true`, Go said `false`). `compile` now iterates `[]rune`.
2. **`filepath.ToSlash` destroys backslash escapes on Windows.** `Match` normalises the PATTERN with
   `filepath.ToSlash`, so `a\*.go` becomes `a/*.go` and a trailing `\` never reaches the
   "trailing escape" error. The Rust twin does the same unconditionally, which makes the behaviour
   platform-independent rather than platform-dependent. The Go unit test records this instead of
   asserting the escape.
3. **A tab can never trigger the YAML scanner's tab guard.** Indent is counted with
   `TrimLeft(raw, " ")`, so a leading tab yields indent 0 and `raw[:0]` contains no tab. Both
   implementations keep the dead guard and both treat a tab-indented line as a sibling; recorded as a
   unit test in each language.
4. **Three ignore divergences from git**, recorded in `📡️ignore-vectors.json.divergentVectors` and
   exercised by the oracle-free case `🐙️gitignore-divergence`: a leading `/` is not a root anchor
   (`/target` also ignores `nested/target`); a negation CAN re-include a file under an excluded
   directory; and a directory rule IS root-anchored (`node_modules/` becomes `node_modules/**`, which
   already contains a separator and is therefore not lifted to `**/`) — git ignores it at any depth.
5. **One glob divergence from picomatch**, in `📡️glob-vectors.json.divergentVectors` and exercised by
   `🚫️posix-negation-class`: the owned matcher accepts POSIX `[!…]`, which picomatch reads as a
   literal class. Inside a `[!…]` class every member is literal (`QuoteMeta`), so `[!a-c]` excludes
   `a`, `-`, `c` and not the range.
6. **The owned YAML encoder loses an empty container.** `list: []` re-decodes as an empty MAPPING,
   because both empty containers emit as a key with an empty block. Moved to
   `📡️codec-vectors.json.edgeVectors` and exercised by the oracle-free case
   `🕳️empty-container-encoding`, where both subjects must lose exactly the same information.
7. **The `go.mod` branch of root discovery is unreachable inside a git checkout.** The `.git` search
   runs first and the test work directory lives under this repository. The tree is kept but renamed
   `an-inner-go-module-does-not-beat-an-enclosing-checkout` with `expectedRoot: "outside"`; both
   adapters project `outside` when discovery escapes the materialised tree.

## 5. Verification — real command output

### `go build` and `go test`

```
$ (cd .../💻️client/⌨️cli && GOWORK=<root>/go.work go build ./...)
exit=0

$ (cd .../🏠️workspace/📦️packages/🐹️go && go test ./...)
ok  	github.com/usalu/semio/repo/workspace	0.515s

$ (cd .../🪪️identity/📦️packages/🐹️go && go test ./...)
ok  	github.com/usalu/semio/repo/identity	0.523s

$ (cd .../🧾️yaml/📦️packages/🐹️go && go test ./...)
ok  	github.com/usalu/semio/repo/yaml	0.561s

$ (cd .../🔎️search/📦️packages/🐹️go && go test ./...)
ok  	github.com/usalu/semio/repo/search	0.653s
```

### `cargo test`

```
$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-workspace -p semio-framework-repo-identity -p semio-framework-repo-yaml -p semio-framework-repo-search
running 5 tests   (identity)   test result: ok. 5 passed; 0 failed
running 6 tests   (search)     test result: ok. 6 passed; 0 failed
running 4 tests   (workspace)  test result: ok. 4 passed; 0 failed
running 5 tests   (yaml)       test result: ok. 5 passed; 0 failed
```

### Protocol v2 harness — `subject` (go), `subject` (rust), `oracle`, `parity`

```
$ bun ./📜️script.ts subject quick --implementation go   --owner …/🏠️workspace
[test] level=quick cases=5 executed=7 passed=7 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts subject quick --implementation rust --owner …/🏠️workspace
[test] level=quick cases=5 executed=7 passed=7 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts oracle quick --owner …/🏠️workspace
[test] not-exercised …/🧪️tests/🐙️gitignore-divergence (recorded no-oracle decision repo-workspace-ignore-divergence — its evidence is discharged by the subject phase)
[test] not-exercised …/🧪️tests/🧭️root-discovery (recorded no-oracle decision repo-workspace-root-discovery — its evidence is discharged by the subject phase)
[test] not-exercised …/🧪️tests/🚫️posix-negation-class (recorded no-oracle decision repo-workspace-glob-divergence — its evidence is discharged by the subject phase)
[test] level=quick cases=5 executed=2 passed=2 failed=0 errored=0 parity=0/0 not-exercised=3
$ bun ./📜️script.ts parity quick --owner …/🏠️workspace
[test] level=quick cases=5 executed=16 passed=16 failed=0 errored=0 parity=11/11

$ bun ./📜️script.ts subject quick --implementation go   --owner …/🪪️identity
[test] level=quick cases=2 executed=2 passed=2 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts subject quick --implementation rust --owner …/🪪️identity
[test] level=quick cases=2 executed=2 passed=2 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts oracle quick --owner …/🪪️identity
[test] not-exercised …/🧪️tests/🧬️compose-id-codec (recorded no-oracle decision repo-identity-owned-codecs — its evidence is discharged by the subject phase)
[test] level=quick cases=2 executed=1 passed=1 failed=0 errored=0 parity=0/0 not-exercised=1
$ bun ./📜️script.ts parity quick --owner …/🪪️identity
[test] level=quick cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4

$ bun ./📜️script.ts subject quick --implementation go   --owner …/🧾️yaml
[test] level=quick cases=2 executed=3 passed=3 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts subject quick --implementation rust --owner …/🧾️yaml
[test] level=quick cases=2 executed=3 passed=3 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts oracle quick --owner …/🧾️yaml
[test] not-exercised …/🧪️tests/🕳️empty-container-encoding (recorded no-oracle decision repo-yaml-owned-encoder — its evidence is discharged by the subject phase)
[test] level=quick cases=2 executed=2 passed=2 failed=0 errored=0 parity=0/0 not-exercised=1
$ bun ./📜️script.ts parity quick --owner …/🧾️yaml
[test] level=quick cases=2 executed=8 passed=8 failed=0 errored=0 parity=7/7

$ bun ./📜️script.ts subject quick --implementation go   --owner …/🔎️search
[test] level=quick cases=1 executed=1 passed=1 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts subject quick --implementation rust --owner …/🔎️search
[test] level=quick cases=1 executed=1 passed=1 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts oracle quick --owner …/🔎️search
[test] level=quick cases=1 executed=1 passed=1 failed=0 errored=0 parity=0/0
$ bun ./📜️script.ts parity quick --owner …/🔎️search
[test] level=quick cases=1 executed=3 passed=3 failed=0 errored=0 parity=3/3
```

Totals: 10 cases, 32 executions under `parity`, **25/25 parity comparisons agree**, 0 failed,
0 errored. `contract --owner …` for all four owners reports no breach whose path is inside these
modules (the breaches it does list are the pre-existing repo-wide `s/🗒️note` fixture and
`testing/discovery` baseline ones).

### nx targets

```
$ bun nx run @semio-tech/repo-yaml-go:test
ok  	github.com/usalu/semio/repo/yaml	0.448s
 NX   Successfully ran target test for project @semio-tech/repo-yaml-go
```

## 6. The 10 cases and their references

| Owner | Case | Mode(s) | Reference |
| --- | --- | --- | --- |
| `🧾️yaml` | `🔁️codec-roundtrip` | differential, round-trip | npm `yaml` 2.8.1 (`third-party-library`) |
| `🧾️yaml` | `🕳️empty-container-encoding` | round-trip | none — `repo-yaml-owned-encoder` |
| `🏠️workspace` | `🃏️glob-matching` | differential | npm `micromatch` 4.0.8 |
| `🏠️workspace` | `🚫️posix-negation-class` | conformance | none — `repo-workspace-glob-divergence` |
| `🏠️workspace` | `🙈️ignore-precedence` | differential | npm `ignore` 7.0.5 |
| `🏠️workspace` | `🐙️gitignore-divergence` | conformance | none — `repo-workspace-ignore-divergence` |
| `🏠️workspace` | `🧭️root-discovery` | conformance ×3 | none — `repo-workspace-root-discovery` |
| `🔎️search` | `🔍️ranked-search` | differential | TypeScript reference written here (`cross-semio-implementation`) + `repo-search-owned-ranking` |
| `🪪️identity` | `😀️entity-emoji-codec` | differential | `Intl.Segmenter` / UAX #29 (`standards-reference-tool`) |
| `🪪️identity` | `🧬️compose-id-codec` | round-trip | none — `repo-identity-owned-codecs` |

Every projection is an array of strings (`"<vector name>=<value>"`), never a number, so no host
language contributes a float-formatting or key-ordering artefact. The YAML case compares canonical
JSON (members sorted by key), produced by `serde_json` / `encoding/json` / a hand-written
canonicaliser on the TypeScript side.

## 7. Deviations from the brief, and why

1. **The oracle manifest is `<owner>/🔮️oracle/🔣️.json`, not `<owner>/🔣️oracle.json`.**
   `discoverTestContributions` matches a DIRECTORY named `taxonomy.testContributionDirName`
   (`🔮️oracle`) containing `testFilenameForKind(json)` (`🔣️.json`). An owner-root `🔣️oracle.json`
   is never read — it produced `unknown oracle id micromatch`. The only other manifest in the repo,
   `🧰️framework/🛍️products/📓️print/🔣️oracle.json`, is at the wrong path AND at schemaVersion 1, so it
   is silently ignored today; worth a separate ticket.
2. **`oracleHostPackages` is a TOP-LEVEL registry key, not a member of an oracle entry.**
   `OracleRegistryEntry` has `additionalProperties: false`; `oracleHostPackagesFor` reads the
   top-level array. The example in `📓️harness-verification.md` §6 nests it and would be rejected.
3. **The npm oracle devDependencies are declared in the ROOT `package.json`, not in a module-local
   `📦️packages/🟦️typescript`.** Creating one would make `ownerShipsImplementation(typescript)` true
   for that owner, so the harness would dispatch the oracle-only `🟦️.ts` adapter as a SUBJECT and
   every scenario would fail with "adapter has no subject registration". Resolution is unaffected:
   the TypeScript host resolves declared packages from the repository root's own `node_modules`
   (`resolvesFromRepoRoot`), which bun hoists to anyway. `ignore@7.0.5`, `micromatch@4.0.8` and
   `yaml@2.8.1` are now root devDependencies; `bun install` saved the lockfile.
4. **An oracle adapter must register EVERY scenario of its feature**, otherwise the oracle role
   errors. Subject-only behaviour therefore lives in its own case directory (with no `🟦️.ts`), which
   is why there are 10 cases and not 6. This is also why the plan's "one case per bullet" shape moved.
5. **The Go test file is `🧪️_test.go`, not `🧪️.go`** — `go test` only compiles `*_test.go`, and a
   `🧪️.go` would link `testing` into the library.

## 8. What is left / handed on

* **`bun ./📜️script.ts test` on a Rust package is repo-wide broken, independent of this work.**
  `runCargoTestBudgeted` → `resolveCargoPackageName` → `loadTaxonomy` throws
  `generatorContracts["wgpu-frame-worker"] tracked output "…/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js" is missing`.
  Reproduced on the untouched `🔨️modules/⌨️cli/📦️packages/🦀️rust` target, so it predates these
  modules. `cargo test -p …` and the harness are unaffected. Needs the wgpu generator to be run, or
  the contract to tolerate an unbuilt generated output.
* **`🔣️taxonomy.json` was an unresolved `git stash pop` conflict** at the start of this slice (see
  `📓️harness-verification.md`); it was resolved by the coordinator mid-session and the file still
  shows as `UU` in the index. Not touched by me.
* `📊️metrics`, `📐️model` and the other L0/L1 owners are other agents' slices; `go.work` and root
  `Cargo.toml` were edited additively with read-modify-write so concurrent entries survive.
* The `go-split` agent still has to remove the godfile copies listed in §2, and the `📐️model` agent
  should consume `identity.Entity("<kind>")` rather than re-declaring the emoji constants.
* Launch entries added to `.vscode/🧩️launch.seed.jsonc` (12: `🧪️test🧰️repo<module>{🦀️rust,🐹️go,🥒️parity}`
  for each of the four modules) and `.vscode/launch.json` regenerated with the plugin registry script.

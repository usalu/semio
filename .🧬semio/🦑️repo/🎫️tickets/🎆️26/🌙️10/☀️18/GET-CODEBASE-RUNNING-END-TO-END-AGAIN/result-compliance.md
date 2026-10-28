# Lane compliance (prefix cp), wave 1027b: AGENTS.md code-rule review of every source file changed live on 10-27 (RE-CUT 19:50 on live after G46)

FOR coordinator (re-cut, all 8 batches `base` on live at 19:4x, 148 file rows, file-disjoint, mode-keeping `stage.py`; `names` deletes files and uses the delete-capable `C/compliance-tools/stage.template.py`, law `python3 -I C/xplat5-tools/tests/test_stage_mode.py <stage.py>` OK):
- apply: `sh C/compliance-tools/apply_all.sh /Users/ueli/Documents/semio apply` (fwprocess fwmodules libother libtools libbuild libgraph lang names); verify each: `VERIFY_ROOT=/Users/ueli/Documents/semio sh C/compliance-tools/staged/<b>/verify.sh` (stage check + Bun parse-check `gate-g-tools/scan.ts` + the repo's docstring/emoji-reuse/[DEBUG] censuses + for `lang` the whole-repo language law on the batch's files). Revert: `sh C/compliance-tools/apply_all.sh <root> revert`. Rebase after anything lands in between: `sh C/compliance-tools/rebuild_all.sh` (counted anchors; an anchor a peer already landed is skipped with a `[skip]` line, a moved anchor stops the build).
- LANDING ORDER: (1) compliance (this) -> (2) nolegacy (`lease reapi nobaseline lang`; it is built ON TOP of compliance: after (1) run `python3 -I C/nolegacy-tools/rebuild.py` because my bytes changed since 16:04; 16 shared files, its `lang` takes the 21 remaining whole-repo language-law findings to 0, the same sites in 6 files I deliberately left to it: sparse image, phases, cutoff, closure-read-worker, fleet readers, genuine-birth) -> (3) schemafirst (`law` first, then the other 14; independent of both, built on live; 18 shared files, after (1)/(2) run `sh C/schemafirst-tools/rebuild_all.sh`). Reason: compliance is comment/text/test-pin level and the smallest; nolegacy declares it as its base; schemafirst re-keys every cache and must be last so its cold round measures the final tree. Landing compliance first costs the others only a rebuild; a different order costs me one `rebuild_all.sh`.
- product round for (1)+(2): `course:build`, `course:test`, `print-rs:test`, dashboard build (`lang` edits the Bazel script/snapshot/sessions texts, the process refusals, the test-envelope receiver and the sealed texts). Type-check: the official `typecheck-project` of `@semio-tech/repo-lib` (my own `tsc` over the closure exceeded 3 GB; nolegacy's `nl-tsc.ts` is the cheaper checker and can be pointed at my 54 TS files).
- Proof on ONE fresh mirror (`cp-main`, cloned from live 19:1x, dropped): all 8 batches applied (no drift), `verify.sh` x8 green; 115 test files (texts catalogs, sealed, Bazel snapshot/sessions/witness/command closure, test-envelope receiver and contention budget, native-build, lane retention, sampling, TypeScript incremental, project-nodes, language corpus, renamed fixtures): failure set 90 on base -> 88 after, ZERO new failures (`🗑️generated/cp/r2/tests-base.txt` / `tests-after2.txt`; `names` and `lang` pins updated for the new machine messages; two load-dependent 5 s timeouts, `chunked builder` and `store refreshes`, pass or time out identically on base). Language law over the whole law roots: 63 findings on live -> 21 after, all 21 inside nolegacy's `lang` manifest.
- Re-run censuses on CURRENT live (925 files changed today, 286 more than at 16:04): docstring openers 13 -> 0, emoji reuse 436 -> 0, body comments 13 -> 0 (the playground-targets rationale comments now live in the docstring of their function), `[DEBUG]` 5 -> 1 (the remaining one is `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:324` PARSING another tool's output line that starts with `[DEBUG] Fixed UDRW image`: protocol text, not a temporary log; leave), runtime-name law 0 on 280 non-test sources, path budget max 167 chars (0 over), language law as above, path emoji: the 16 renames of `names` plus 1 open (below). Umlaut check: real umlauts in 40 files, ASCII fallback only in modules that are ASCII throughout (sealed, scheduling, bazel environment); the physical-lock schema was already converted by the umlaut batch (anchors skipped).

What changed since 16:04 in the batches: emoji lists re-derived on live (47 -> 65 files with repeated/missing opener, new peer files included), `lang` +14 sites (test budget contention, test-envelope receiver, agent-session ancestry, dashboard artifact, command entry authentication, graph farm directory kind, command-closure reference, project-nodes contracts, fleet text/test-birth machine tokens; the pinned tests follow), 6 anchors dropped because peers landed them (`say` for the Bazel planner/idle servers, `Unknown test suite`, German physical-lock text), 3 allowance rows only re-cut (stale), Rust `⏪` reuse fixed, Python oracle of the cutoff opener fixed.

OPEN (new since 16:04, owned by the peers that wrote them, not touched): new sibling-duplicate path emoji in `🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests` (`🪆️captured` vs `🪆️authority`, fixtures `🪆️captured-service|kernel-role|parent-history` vs `🪆️captured-entry`, referenced by many paths: rename in the owning lane), `🚀️launcher/🧪️tests/🪟️windows-contract` vs `🪟️runtime-outside-partition`, `bazel/🧪️tests/📥️fingerprint/🔣️ambient.json` (in flight).

## What was reviewed and how

- Scope: `git status --porcelain` under 🧰️framework, 🎓️teaching, 🚀️launcher, ✏️s intersected with mtime >= 10-27 00:00: 925 files at 19:00 (580 ts/mts/mjs, 345 other); list: `C/compliance-tools/today.txt`. Peers keep editing: the final table is over that list, `rebuild_all.sh` re-reads live.
- Laws and tools per rule (all read-only on live, fixes verified on the mirror):
  - docstring openers + unique leading emoji: repo census `docstringHitsOfText` / `docstringEmojiReuseOfText` / `debugTagHitsOfText` (`🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census`, TS + Rust) plus my own TypeScript-AST scan `compliance-tools/cp-ast.ts` for `.mjs`/`.mts`/`.d.mts` (the census skips those) and `cp-other.py` for Python/Rust.
  - comments inside definitions: AST scan (comments inside function bodies), Rust lexer `rs_comments.py` (0 inside items; the four `// ──── section ────` banners sit between items), Python tokenizer.
  - prose-literal / default-language / catalog EN+DE parity: `⚖️laws/🗣️language-catalogs` (`proseSitesV1`, `defaultLanguageFindingsV1`, `catalogFindingsV1`, `languageLawFindingsV1` over the whole roots with the allowlist). DE quality: `cp-other.py` (DE equal to EN, empty DE, ASCII fallback vs real umlauts per module).
  - runtime-name law: `⚙️runtime/⚖️law` `checkRuntimeLawV1` over today's non-test sources: 0 violations (217 files scanned). Test files are law-exempt (they name `bun:test`, `import.meta.dir`).
  - taxonomy/path-emoji: `pathEmojiStatuteFindings` of the taxonomy law on the paths of the list and all their siblings (`cp-names.ts`; the repo law itself walks 465k paths and reached 2 GB, so it was not run whole); path budget (256 UTF-16 units, 255 UTF-8 bytes per component, law `🧮️path-budget`): longest list path 165 chars (193 with the checkout prefix), 0 over.
  - [DEBUG]: repo census + grep; legacy/compat words; external imports (AST, every static/dynamic specifier); schema-first literals (heuristic: named policy constants and bare byte/ms bounds).
- Not machine-checkable and therefore NOT claimed: "fitting" emoji, "concise code". The emoji pass chooses by the opening verb and the first keywords of the docstring (`cp-emoji.ts`, keeps the first docstring of every emoji, never reuses an emoji of the file); a sample of 24 changes is in the review list at the end. Uniqueness is enforced per file as the repo gate does (module-level uniqueness is not a law).

## Result summary (before -> after the staged batches, files changed today on live at 19:00)

| Finding | Before | After the staged batches |
|---|---|---|
| comment-in-body | 13 | 0 |
| de-ascii-fallback | 8 | 11 |
| de-empty | 1 | 1 |
| de-equals-en | 12 | 12 |
| debug-tag | 5 | 1 |
| doc-in-body | 40 | 40 |
| doc-not-emoji | 13 | 0 |
| dup-emoji | 436 | 0 |
| legacy-lease-state | 14 | 14 |
| path-emoji | 10 | 1 |
| prose-literal | 46 | 8 |
| schema-first? | 367 | 367 |

External imports in TEST files (allowed: tests may use third-party oracles): ajv x130, typescript x64, esbuild x17, ajv/dist/2020 x16, @iarna/toml x8, ajv/dist/2020.js x5, picomatch x3, fast-glob x3, vitest x2, vitest/config x2, numpy x2, yaml x2, fast-json-patch x2, minimatch x2, json5 x1, tree-kill x1, fflate x1, iconv-lite x1, yargs-parser x1, nx/src/native x1, resolve.exports x1, whatwg-url x1, nx/src/tasks-runner/create-task-graph x1

## Findings that are NOT fixed by a staged batch (owner decision or design needed)

1. Language law (nolegacy `lang` covers all of these; kept here for the record), 3 files of 16:04, 13 sites: `🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts` (allowed 24, actual 34: lines 31, 47, 50, 74, 79, 83, 87, 90 sit in functions that `toString()` into the loader child, so they cannot import a catalog; 101/102 are inside the loader's source text; the child already has `refusal(code, fields)` with the closure schema's `Messages`: add keys `closure.compilerCommentUnclosed|builtinReadCoverageRefused|builtinReadRemained|implementationImageChanged|implementationImageRequired|importResolutionRequired|snapshotBounded|snapshotChanged` to `🧬️schema/🔣️.json` Messages in EN+DE and give the serialized functions a `refusal` parameter; the tests pin the English text of "Immutable implementation image changed" in `🧪️tests/📥️stream/🟦️.ts:109-112` and `🗓️scheduling/🧪️tests/🔣️.json:45`), `🏡️resident/📜️journal/🟦️.ts` (88, 152: bilingual strings inside one `Error`; a resident service has no CLI boundary that renders `RepoErrorV1`, so it needs its own renderer; test `🧪️tests/🟦️.ts:57` matches /cannot vouch/), `🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs:116` (its `code: "BUILD_RUST_SOURCES_UNRESOLVED"` property would overwrite `RepoErrorV1.code`; `🧪️tests/🦀️mounted-sources/🟦️.ts:64` asserts it).
2. Language law, 23 more files changed since HEAD but not touched today (the law is red repo-wide, `🗣️language-catalogs/🧪️tests/🗣️corpus` "the repository holds no language law finding"); same fix pattern as `lang_spec.py` (L = 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library, P = 🧰️framework/🔨️modules/🏃️process):
- allowance-stale allowed=2 actual=1 `P/📜️script.ts`
- prose-literal allowed=0 actual=1 `P/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪶️sparse/🟦️.ts`
- prose-literal allowed=95 actual=96 `P/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`
- prose-literal allowed=9 actual=10 `L/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts`
- prose-literal allowed=57 actual=58 `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️closure-read-worker/🟨️.mjs`
- allowance-stale allowed=17 actual=16 `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts`
- prose-literal allowed=26 actual=28 `L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts`
- prose-literal allowed=76 actual=77 `L/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts`
- prose-literal allowed=0 actual=2 `L/🏃️process/🦀️cargo/🧰️native-helpers/🟦️.ts`
- prose-literal allowed=1 actual=2 `L/🏗️build/🏃️execution/🔒️hermetic/🔌️providers/🔷️typescript/🟦️.ts`
- prose-literal allowed=3 actual=4 `L/🏗️build/🏃️execution/🔒️hermetic/🟦️.ts`
- prose-literal allowed=3 actual=4 `L/🏗️build/🏡️resident/🔭️watch/🏗️provision/🟨️.mjs`
- allowance-stale allowed=16 actual=14 `L/🏗️build/📥️inputs/🔏️revision/🟨️.mjs`
- allowance-stale allowed=26 actual=25 `L/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs`
- prose-literal allowed=0 actual=3 `L/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🏭️runner/🍞️bun/🟦️.ts`
- prose-literal allowed=0 actual=1 `L/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/📖️status/🟦️.ts`
- prose-literal allowed=4 actual=6 `L/🏗️build/🕸️graph/⏱️phases/🟨️.mjs`
- prose-literal allowed=0 actual=1 `L/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs`
- prose-literal allowed=0 actual=3 `L/🏗️build/🕸️graph/🕰️generations/🧷️pin/🟨️.mjs`
- prose-literal allowed=0 actual=2 `L/🏗️build/🚢️fleet-status/📥️readers/🟦️.ts`
- prose-literal allowed=0 actual=1 `L/🏗️build/🚢️fleet-status/🟦️.ts`
- prose-literal allowed=5 actual=6 `L/🧪️testing/🌱️genuine-birth/🟦️.ts`
- prose-literal allowed=48 actual=50 `L/🧹️normalization/🔣️taxonomy/🏭️generators-packages-and-exclusions/🟦️.ts`

3. Legacy support (AGENTS.md: no legacy support, no deprecations): the lease holder state `"legacy"` (a holder that publishes no standing) with its policy `reclaim.legacyAfterMilliseconds: 14400000` is backward-compatibility handling in `P/🔒️leases/📈️standing/🟦️.ts:14,15,117`, `📈️standing/🤝️yield/🟦️.ts:55-61,120`, `📈️standing/🧬️schema/🔣️.json` (268, 299, 418, 628, 636, 888, 1184), `📋️status/🟦️.ts:13,55,108` + schema, `P/🧭️routing/📥️invocation/🏃️process/🟦️.ts:148`, repo README (EN 625, DE 1762) and build guide (116/349). Not staged: removing the state changes the reclaim policy and the status wire; the clean fix is one rule for every holder (a holder without standing is `silent`/`stuck` by the same clocks) in a lease-owner batch. All other `legacy|compat|shim|todo` hits were read: `shim` = the cargo wrapper scripts, `todo` = a work stack, `legacy-test-*` = the test-layout law's own vocabulary (the sampling `legacy` variable was renamed `settled` in `lang`; `index-chunks/legacy.ts` became `🗜️flat-reference.ts` in `names`).
4. Schema-first (constants in schema JSON, not literals), reported as candidates, not staged (each edit changes the stamped policy digests and re-keys graph/memo/plan caches for every product; land with a measured cold round): `🏗️build/👁️observation/🟨️.mjs` L47 `RACY_MS = 2000`, L64 `SOURCE_ANALYSIS_BYTES = 16 MiB`, L466 `SETTLED_READ_BYTES = 128 MiB`, L469 `FILE_RECORD_IMAGE_BYTES = 64 MiB`, byte bounds 65536/8388608 in L25/29/44; `🏗️build/🕸️graph/🟦️.ts` 65536 (L55, 57, 60), 8388608 (L63, 66, 661), 536870912 (L701, 771), stride 1024 (L233, 264); `⚡️caching/🗃️records/🟦️.ts` L11 `ORPHAN_MS`, L12 `PENDING_FLUSH_BYTES`; `🏗️build/🧹️cache-gc/⌨️command/🟦️.ts` L110 600000, L169/174 3600000; `P/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts` L10 `OPTIONAL_PROBE_MILLISECONDS=3000`, `MEGABYTE`, page size 4096 (L25, 43, 136), 10000 (L107, 109); `P/🔒️leases/🟦️.ts` L246 4096, L269/354 progress cadence 1000. The heuristic found 284 candidate lines in 93 production files (`🗑️generated/cp/policy-const2.json`); most are zero initialisers and clamps, treat as a worklist, not as a verdict.
5. Docstrings of nested functions in `.mjs` factories (25, e.g. `🕸️graph/🕰️generations/🟨️.mjs`, `🧩️claims/🟨️.mjs`): each starts with a unique emoji; they sit inside a factory function body, which the strictest reading of "no comment inside definitions" forbids. Left: they document the returned API members (info). `export` without docstring: 250 across 48 files (info; AGENTS.md requires the emoji opener, not a docstring on every export).
6. DE in ASCII fallback (`ae/oe/ue`): `🏛️sealed` (README, `🔣️.json`, tests), `📥️source/🔏️closure/🗓️scheduling/🔣️.json`, `🔧️tools/🏗️bazel/🌿️environment/**` use it consistently inside their module and were left; `P/🔒️leases/📁️physical/🧬️schema/🔣️.json` was the one file that broke its module (everything around it uses real umlauts): fixed in `fwprocess`. `de-equals-en` (3: `{id}: {used}{note}`-style rows and the sealed `blockedRow`) are language-neutral formats; `gc.mode.apply` is an intentionally empty message.
7. `🔧️tools/🏗️bazel/🧪️tests/📥️fingerprint/🔣️ambient.json` shares its emoji with `🔣️.json` and a peer's `🧬️ambient.json` sits beside it: a rename is in flight, not touched. `📚️library/🏗️build` vs `📚️library/🏗️authoring` (same `🏗` module-level) is old debt.
8. External libraries: production code has two imports outside `node:`/relative/workspace: `🕸️dependencies/🟦️typescript/🟨️.mjs` (`typescript`, behind `registerSourceAnalysisV1`) and the sandbox guard preload (`bun`, inside the `🍞️bun` adapter, allowed by the runtime law). Everything else is in tests (ajv x81, typescript x41, esbuild x12, @iarna/toml, minimatch, picomatch, vitest, numpy in two Python oracles, tree-kill, fast-glob, yargs-parser): allowed as independent oracles. No `[DEBUG]` remains after `libbuild`.
9. The README of `🏛️sealed` has no heading emoji (`# Sealed implementation`); docs are EN+DE. Not changed.

## Fixed (what each batch does)

| batch | files | change |
|---|---|---|
| fwprocess | 13 | unique leading emoji (12 TS), Python oracle docstrings emoji-first and two body comments removed, physical-lock schema German in real umlauts (module convention) |
| fwmodules | 3 | unique leading emoji (runtime identity, change journal client opener without emoji, churn) |
| libother | 1 | unique leading emoji of the cargo workspaces module |
| libtools | 3 | unique leading emoji (Bazel plan reuse, universe, sessions port) |
| libbuild | 16 | unique leading emoji, Rust `🪟` reuse, Python model docstring, memo edit law: `[DEBUG]` prints and thrown `[DEBUG]` texts removed |
| libgraph | 11 | unique leading emoji; the stale orphan JSDoc tag block of `inferenceCandidates` dropped (its params did not match the function) |
| lang | 39 | 36 EN-only refusals/warnings become catalog messages in EN+DE (process, caching, bootstrap, build catalogs; sealed layer in its own texts, ASCII German like the module); machine tokens for the two guards inside the text renderer; the language-law allowlist re-cut and its ceiling lowered 9637 -> 9630; 6 test/fixture files that pinned the old English texts follow; the build catalog test learns the cargo preparation owner; sampling variable `legacy` -> `settled` |
| names | 36 rows | 16 renamed files/folders (own sibling-unique emoji) + references; `legacy.ts` -> `🗜️flat-reference.ts` (`legacyCreateFactIndexV1` -> `flatReferenceFactIndexV1`) |

Red on base, green with the fix: `cp-docs-law.ts` (repo censuses) 13/19/6/14 findings on the base files of fwprocess/libbuild/libgraph/libtools -> 0; language law on the `lang` files 29 -> 3 (the three above); path-emoji findings touching the renamed paths 13 -> 0.

## Per-file finding table (925 files at 19:00; only files with a finding)

| File (tail) | Findings before -> after | Fixed in |
|---|---|---|
| `🧰️framework/🔨️modules/⚙️runtime/🏷️identity/🟦️.ts` | dup-emoji 10->0 | fwmodules |
| `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts` | schema-first? 3->3 | - |
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts` | schema-first? 4->4 | - |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🌦️forecast/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts` | dup-emoji 5->0; schema-first? 3->3 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts` | schema-first? 3->3 | - |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts` | dup-emoji 8->0; prose-literal 1->0; schema-first? 8->8 | lang |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🐍️oracle/🐍️.py` | comment-in-body 2->0; doc-not-emoji 3->0 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🪜️phases/🐍️oracle/🐍️.py` | doc-not-emoji 1->0 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪜️phases/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts` | dup-emoji 2->0; schema-first? 1->1 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units/🧪️tests/🐍️oracle/🐍️.py` | doc-not-emoji 2->0 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🔁️incremental/🟦️.ts` | prose-literal 1->0; schema-first? 1->1 | lang |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts` | dup-emoji 2->0; schema-first? 5->5 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts` | prose-literal 1->0; schema-first? 4->4 | - |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` | schema-first? 5->5 | - |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts` | prose-literal 1->0; schema-first? 6->6 | lang |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧹️sweep/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📁️physical/🟦️.ts` | dup-emoji 4->0; schema-first? 1->1 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts` | legacy-lease-state 2->2; schema-first? 2->2 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts` | legacy-lease-state 6->6; schema-first? 2->2 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests/🟦️.ts` | legacy-lease-state 3->3 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🧬️schema/🔣️.json` | de-equals-en 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📋️status/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` | dup-emoji 2->0; schema-first? 18->18 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts` | dup-emoji 4->0; schema-first? 3->3 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧬️schema/🔣️.json` | de-equals-en 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/⏲️contention/🟦️.ts` | dup-emoji 2->0; prose-literal 1->0; schema-first? 7->7 | lang |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🛡️guard/🍞️bun/🔌️preload/🟦️.ts` | external-import(prod) 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/📏️file-budgets/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts` | prose-literal 1->0; schema-first? 4->4 | lang |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🏃️run/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🖥️platforms/🟦️.ts` | dup-emoji 2->0 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🧯️ceiling/🟦️.ts` | dup-emoji 2->0 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/🟦️.ts` | dup-emoji 2->0 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/🧪️tests/👶️fixture/📜️holder.ts` | path-emoji 1->0 | names |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/🧪️tests/👶️fixture/📜️leaver.ts` | path-emoji 1->0 | names |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/🧪️tests/👶️fixture/📜️limit.ts` | path-emoji 1->0 | names |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/🧪️tests/👶️fixture/📜️runner.ts` | path-emoji 1->0 | names |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` | dup-emoji 2->0; legacy-lease-state 1->1; schema-first? 1->1 | fwprocess |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts` | legacy-lease-state 2->2 | - |
| `🧰️framework/🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🗄️store/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🧭️ancestry/🪟️native/🟦️.ts` | prose-literal 1->0; schema-first? 1->1 | lang |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts` | doc-not-emoji 1->0 | fwmodules |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts` | dup-emoji 4->0 | fwmodules |
| `s/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts` | prose-literal 1->0; schema-first? 2->2 | lang |
| `ramework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📐️elastic/🟦️.ts` | schema-first? 1->1 | - |
| `ramework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts` | schema-first? 2->2 | - |
| `ary/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️closure-read-worker/🟨️.mjs` | dup-emoji 2->0; prose-literal 1->1; schema-first? 3->3 | libother |
| `🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts` | schema-first? 8->8 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` | debug-tag 1->1; schema-first? 17->17 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts` | schema-first? 3->3 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json` | de-empty 1->1; de-equals-en 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🟦️.ts` | schema-first? 6->6 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏃️contract-capture/🟦️.ts` | prose-literal 1->0; schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗃️records/🟦️.ts` | schema-first? 7->7 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts` | dup-emoji 2->0; prose-literal 1->0; schema-first? 2->2 | lang |
| `mework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts` | schema-first? 2->2 | - |
| `cts/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🔌️host/🟦️.ts` | schema-first? 2->2 | - |
| `🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🟦️.ts` | schema-first? 1->1 | - |
| `ework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts` | schema-first? 3->3 | - |
| `cts/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🎰️jobserver/🧪️tests/🧫️fixtures/🐍️realise.py` | path-emoji 1->0 | names |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🏗️buildscript/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts` | dup-emoji 6->0; prose-literal 1->0; schema-first? 1->1 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/📖️admissions/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts` | comment-in-body 1->0; schema-first? 1->1 | libother |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/⏱️timing/🟦️.ts` | prose-literal 1->0; schema-first? 2->2 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` | prose-literal 1->0; schema-first? 6->6 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⏱️profile/🟨️.d.mts` | dup-emoji 3->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⏱️profile/🟨️.mjs` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts` | schema-first? 1->1 | - |
| `rk/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️g29.json` | path-emoji 1->0 | names |
| `rk/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️g34.json` | path-emoji 1->0 | names |
| `ork/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️g5.json` | path-emoji 1->0 | names |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📥️execution/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🔁️promoter/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🔣️.json` | de-ascii-fallback 0->3; de-equals-en 1->1 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts` | dup-emoji 5->0; prose-literal 1->0; schema-first? 1->1 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🟦️.ts` | prose-literal 1->1; schema-first? 3->3 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🧹️abandoned/🟦️.ts` | dup-emoji 3->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔭️watch/🦀️rust/🦀️.rs` | dup-emoji 5->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🗃️state/🟨️.mjs` | doc-in-body 10->10; dup-emoji 2->0; schema-first? 2->2 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🔢️row-table/🟨️.d.mts` | dup-emoji 4->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🔢️row-table/🟨️.mjs` | doc-in-body 2->2; doc-not-emoji 2->0; dup-emoji 2->0; schema-first? 2->2 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.d.mts` | dup-emoji 26->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs` | dup-emoji 69->0; schema-first? 23->23 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers/🟦️.ts` | schema-first? 3->3 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🟨️.mjs` | dup-emoji 2->0; schema-first? 3->3 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts` | dup-emoji 14->0; schema-first? 1->1 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🍎️seatbelt/🟦️.ts` | schema-first? 5->5 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs` | dup-emoji 8->0; schema-first? 1->1 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts` | prose-literal 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔒️leases/⌨️command/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🟦️.ts` | dup-emoji 2->0 | libtools |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/⚙️operations/🟦️.ts` | prose-literal 1->0 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌌️universe/🟦️.ts` | dup-emoji 2->0 | libtools |
| `ucts/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌿️environment/🏃️action/🧪️tests/🧫️fixtures/🔣️.json` | de-ascii-fallback 8->8 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` | dup-emoji 16->0; prose-literal 1->0; schema-first? 6->6 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔌️backend/🟦️.ts` | prose-literal 1->0 | - |
| `mework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️fingerprint/🔣️ambient.json` | path-emoji 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/📖️status/🟦️.ts` | prose-literal 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🔌️port/🟦️.ts` | dup-emoji 4->0; schema-first? 4->4 | libtools |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts` | dup-emoji 4->0; prose-literal 1->0; schema-first? 4->4 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.mjs` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/♻️revision/🟨️.mjs` | dup-emoji 4->0 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs` | comment-in-body 9->0; dup-emoji 4->0; schema-first? 1->1 | libgraph |
| `ducts/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs` | dup-emoji 4->0; prose-literal 1->0; schema-first? 2->2 | lang |
| `ts/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs` | schema-first? 1->1 | - |
| `k/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs` | doc-in-body 1->1; schema-first? 3->3 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs` | dup-emoji 8->0; prose-literal 1->0; schema-first? 3->3 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs` | dup-emoji 2->0 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs` | doc-not-emoji 1->0; dup-emoji 4->0; prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs` | dup-emoji 6->0; prose-literal 1->1 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs` | dup-emoji 4->0; schema-first? 1->1 | libgraph |
| `roducts/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🟨️.mjs` | dup-emoji 2->0 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` | dup-emoji 62->0 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs` | doc-not-emoji 1->0; dup-emoji 12->0; prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts` | dup-emoji 2->0 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs` | dup-emoji 2->0; prose-literal 1->1 | libgraph |
| `ework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🐍️oracle/🐍️.py` | doc-not-emoji 1->0 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.d.mts` | dup-emoji 7->0 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs` | dup-emoji 7->0; schema-first? 1->1 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔐️custody/🟨️.mjs` | doc-in-body 5->5 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs` | doc-in-body 16->16; dup-emoji 14->0; schema-first? 8->8 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.mjs` | doc-in-body 6->6 | - |
| `work/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🧱️index-chunks/legacy.ts` | path-emoji 1->0 | names |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧱️chunk-classes/🟨️.d.mts` | dup-emoji 2->0 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧱️chunk-classes/🟨️.mjs` | dup-emoji 2->0; schema-first? 1->1 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts` | dup-emoji 14->0; schema-first? 14->14 | libgraph |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🏃️routing/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/📥️readers/🟦️.ts` | prose-literal 1->1; schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🟦️.ts` | dup-emoji 2->0; prose-literal 1->0; schema-first? 6->6 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🧪️tests/🐍️.py` | comment-in-body 1->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🧬️schema/🔣️.json` | de-equals-en 7->7 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧂️text-bytes/🟨️.mjs` | dup-emoji 2->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs` | dup-emoji 4->0; schema-first? 3->3 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts` | dup-emoji 2->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts` | dup-emoji 6->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🟦️.ts` | schema-first? 6->6 | - |
| `️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/✏️edit/🟦️.ts` | debug-tag 4->0; dup-emoji 4->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🐍️.py` | doc-not-emoji 1->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧳️carry/🟦️.ts` | dup-emoji 2->0 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧹️retention/🟦️.ts` | dup-emoji 4->0; schema-first? 1->1 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts` | schema-first? 7->7 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts` | dup-emoji 2->0; schema-first? 6->6 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/👯️dedupe/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts` | dup-emoji 2->0; schema-first? 2->2 | libbuild |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | schema-first? 4->4 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔎️probes/🎫️capacity/🟦️.ts` | schema-first? 2->2 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔎️probes/💾️storage/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs` | external-import(prod) 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🏃️command/🟦️.ts` | schema-first? 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness/🟦️.ts` | prose-literal 1->0 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts` | dup-emoji 2->0; schema-first? 3->3 | libother |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` | dup-emoji 10->0; prose-literal 1->0; schema-first? 1->1 | lang |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🟦️.ts` | prose-literal 1->1 | - |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🪪️native-birth/🟦️.ts` | prose-literal 1->0; schema-first? 1->1 | lang |

Files with no finding (730 of 925): all other files of the list; undocumented exports (info, no AGENTS.md rule demands a docstring on every export) exist in 65 files.

## Review list: 24 random emoji replacements (old -> new | docstring start)

| change | text |
|---|---|
| ⚖️ -> 🚫️ | Refuses conflicting results from the same source c |
| 🪢️ -> 🌀️ | The previous blanket router contract, kept only as |
| 🧠️ -> 🌳️ | The JSON image of the analysis of a settled file,  |
| 🔌️ -> 🌊️ | The script provider ports one module defines: the  |
| 🧭️ -> 🌐️ | Resolves a `{projectRoot}` or `{workspaceRoot}` in |
| 🧟️ -> 🔑️ | Names the live process a unit directory belongs to |
| 🧭️ -> 🧠️ | Tracks literal relative command imports through th |
| 🧾️ -> 📰️ | The facts a detached computation read before it fa |
| 🔎️ -> 📚️ | Reads one test file against the evidence rules: a  |
| 🔗️ -> 📁️ | Resolves a literal `new URL("…", import.meta.url)` |
| ✂️ -> 🛑️ | Whether a chunk ends after this path: a pure funct |
| 📣️ -> 🏗️ | Says why a build starts: the reason of a cold star |
| 🦀️ -> 🛠️ | Resolves Cargo's local compilation inputs, admitti |
| 🔀️ -> 🏃️ | The imports a router script needs for all the comm |
| 🧊️ -> 🕵️ | A verified block while its leaf still has the iden |
| 🧷️ -> 🧐️ | Validates nested project records within one origin |
| 🧩️ -> 📂️ | Directories a declared glob never descends into. * |
| 🧩️ -> 🧵️ | Creates an empty interner of runs. */ |
| 🧰️ -> 🌍️ | The module resolution configuration bun reads for  |
| 🔎️ -> 📋️ | Lists what exists on disk below the server and wor |
| 📥️ -> 🧯️ | Captures freshly computed source facts and refuses |
| 🧮️ -> 🗝️ | True when a target input names a module the entry  |
| 🔏️ -> 🪡️ | The digest of everything a project reads from one  |
| 📖️ -> 🔖️ | Reads the newest sealed generation without any lea |

## Tools (all in `C/compliance-tools/`)

`cp-ast.ts` (AST scan), `cp-other.py`, `cp-laws.ts`, `cp-lawfull.ts` (whole-repo language law with the root's allowlist, 270 MB), `cp-names.ts`, `cp-docs-law.ts`, `cp-emoji.ts`, `cp-allow.ts`, `lang_spec.py` + `names_spec.py` (edit lists), `make_batch.py`, `stage.template.py`, `verify_batch.py`, `apply_all.sh`, `rebuild_all.sh`, `rebase_mirror.sh`, `run_tests.py`, `guarded.py`, `report.py`, `rs_comments.py`, `batches/*.json`. Raw scans: `C/🗑️generated/cp/` (not deleted). Mirror `cp-main` dropped at the end of the lane.

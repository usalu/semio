# ⏱️ Taxonomy Report And Nx Graph Construction Performance (2026-10-13)

This note covers two gates that did not scale: `workspace:verify-taxonomy-report`, and Nx project-graph construction before every daemonless `nx` command. For each it gives the profile, the root causes, the fixes, before and after timings, and the identity proofs.

The machine was busy with peer lanes the whole time (load average 15–40 on 10 cores). Every timing below is wall-clock under that load unless it says otherwise. Read them as relative, not absolute.

---

## 1. Outcome

### 1.1 Taxonomy report (full tree, 155,6xx admitted paths, 266,107 inventory entries)

| Phase | Before (pre-change snapshot, same tree, same load class) | After |
|---|---:|---:|
| source observation (incl. mutation-leaf proof) | 202 s | 22 s |
| directories | 486 s | 19 s |
| files | 2,761 s | ≈ 190 s |
| post-files projections | not reached separately (≥ 17 s, 86 s steps seen) | ≈ 15 s |
| inventory references | **> 4,272 s at 19.4 % (25,395 / 130,671), stopped at the 7,200 s deadline** | **226 s (all 130,683 files)** |
| inventory complete | ≥ 2 h 9 min and unfinished (projected > 8 h) | **490 s** |
| plan, up to the reference-edit phase | not reached | 40 s |

- **A1's "33 minutes of heartbeats" has a single cause.** It is one Markdown file: `.🧬semio/…/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📓️oct8-publication-owned-completed-nx-metadata-reclamation.md` (968 KB). The link-reference regex spent **1,051.7 s** on it (CPU profile: 99.4 % in `/^\s*\[[^\]]+\]:\s*(\S+)/gmu`). After the fix, the same file takes 0.00 s.
- **The full report still cannot finish on today's tree, with the old code or the new code.**
  - Planning throws two pre-existing exceptions (§5).
  - The old code never got far enough to show them. A1 stopped it at 44 min; my baseline stopped at 2 h 9 min while still inside inventory.
- **The real gate now reaches planning.** `NX_DAEMON=false bun nx run workspace:verify-taxonomy-report --skip-nx-cache`:
  - total wall time 619 s (Nx run duration 9 m 30 s, including about 60 s of graph construction);
  - `inventory/complete 266136/266136`;
  - then `plan/incoming-coordinate-roots`, `plan/incoming-candidates` and `plan/references`;
  - then it throws blocker 1 of §5 (`Reference edit preimage mismatch at .vscode/🔣️.json:embedded-argv:96584:676@5180952`), exit 1.
  - Before the change, A1's run printed heartbeats for 33 min and was killed at 44 min, still inside inventory.
- **The < 60 s target is not met.** On this loaded machine the inventory now takes about 8 min. §6 lists what is left and the design that closes the gap.

### 1.2 Nx graph construction (`nx graph`, private workspace-data, daemonless)

| Step | Before | After, cold (empty inference cache) | After, nothing changed |
|---|---:|---:|---:|
| bootstrap admission walk (`publishNxCollectionBoundary`, 58,879 dirs) | 18.2 s | 3.1 s | 0.9–1.6 s |
| `@repo/emoji-project-json:createNodes` | 41.8–49.4 s | 47.4 s (recording overhead is within noise) | **2.6–5.9 s** |
| `@repo/test-cases:createNodes` (CPU 4 s; the rest waits on emoji) | 43.6–51.9 s | 50.9 s | **0.4–0.8 s** |
| createDependencies (both plugins) | 5.2–9.5 s | 2.3 s | 0.4–0.6 s each |
| `createProjectGraphAsync` total | 68.7–88.0 s | 77.1 s | 13.6–24.9 s |
| `bun nx graph --file=…` wall | 76–98 s | 84 s | **19.7–33.7 s** |

- **What is left in the warm path is Nx-internal**, and none of it is inference:
  - plugin resolution: `loadSpecifiedNxPlugins` → `retrieveProjectConfigurationsWithoutPluginInference`, about 3–4 s;
  - `write cache` of the 200 MB `project-graph.json`, about 1.8–4.4 s;
  - the graph-file output itself.
- **These scale with graph size, which is wave N2's job** (one test target per owner, fewer than 400 projects).
- **The < 20 s cold target is met only for the parts this lane owns.** Admission is 3 s; createNodes is unchanged at about 47 s, because it is genuine first-time parsing.
- **The < 2 s target is met only for the plugin inference.** The emoji createNodes hit costs about 1.5 s of validation plus about 1.3 s to parse 140 MB of JSON. The rest is Nx itself.

---

## 2. Method

All harnesses are kept as input files in `taxonomy-graph-perf/`.

- **`⏱️taxonomy-phases.ts`** times every progress phase of `verifyTaxonomy` and logs every step slower than 2 s.
  - A worker thread prints the last progress path when the main thread stalls (this is how the 968 KB Markdown file was found).
  - It exits from the progress callback at a deadline or at `TAXONOMY_EXIT_AT`, so `bun --cpu-prof` flushes.
  - `TAXONOMY_MODULE` switches to the pre-change snapshot (a mirror of the touched module trees, taken before the first edit).
- **`🔥️cpuprofile-top.ts`** reports self and inclusive time per function and per regex literal from a `.cpuprofile`. With `CALLERS_OF=…` it gives the direct-caller breakdown.
- **`🧬️differential.ts`** compiles the same named declarations from the snapshot and from the current source and diffs their output over a corpus. **`🧭️resolution-differential.ts`** does the same for reference resolution against the real 266 k-path index.
- **`⚖️inventory-identity.ts`** runs `inventoryTaxonomy` old and new back to back in one process and byte-compares the canonical inventory.
- **`🕸️plugin-differential.mjs`** is an attempt at a direct plugin comparison. The snapshot plugin cannot run outside the workspace (its script-closure guard refuses paths outside the workspace), so whole-graph runs were used instead (§4.3).
- **`🧪️run-tests.sh`, `🔁️baseline-swap.sh` and `🧩️closure-names.py`** handle the test sweep:
  - `🧪️run-tests.sh` runs the test sweep.
  - `🔁️baseline-swap.sh` runs listed tests against the snapshot sources in place, always restores the current files (trap), and was verified byte-identical after each run.
  - `🧩️closure-names.py` registers the new helper declarations in the introspective test closures.

---

## 3. Taxonomy root causes and fixes

Every fix keeps the observable result identical. The last column says why.

| # | Where | Root cause (measured) | Fix | Why identical |
|---|---|---|---|---|
| T1 | `provenMutationLeafOwners` → `leafNames` | `directories.filter(dirname === root)` for each fixture root: O(roots × 110 k dirs), **142 s** | Build the parent → child-names map once, lazily | Same name set, then sorted |
| T2 | `createTaxonomyPathMatcher.matches` | NFC-normalises path and pattern on every call; called 514 × per path, **≈ 90 s in `normalize`** | One-entry memo of the normalised path; compiled regex keyed by raw pattern on top of the normalised-pattern map (still one compile per normalised pattern, as the ♻️ law requires) | Same regex, same input |
| T3 | `matchingFixedContracts` and historical evidence | 514 fixed contracts tested per path (**148 s**) | Leaf index: contracts whose final pattern segment is a literal are keyed by that NFC segment; only those plus the variable-leaf contracts are tested. Specificity is computed once per contract | `fixedContractLiteralLeaf` returns a leaf only when the glob grammar forces the final segment. picomatch oracle: 576 matching pairs over 618 patterns, zero exclusions |
| T4 | `matchDirectoryKind`, `packageLocation` | `emojiFold` over every directory kind, and over all 5,062 member names, for every directory (**119 s**); `toLocaleLowerCase` per kind (**64 s**) | Per-taxonomy index: kinds by folded emoji, kinds by locale-folded id, member kinds by folded name (declaration order kept). `packageLocation` pre-checks `endsWith("packages")` | Same filters on the same candidates, same order; the final order is a total sort |
| T5 | source observation | `sourceAdmissionLstat` re-lstats the repo-root chain and every ancestor for each of 155 k candidates, then verifies them all again | `sourceAdmissionAncestryObserver`: each directory is lstat-ed once per collection; every leaf is still fresh; all observed ancestors are re-verified once at the end. Created lazily, so the injected-dependency laws stay unchanged | Same symlink and non-directory refusals (same messages); ancestry custody is checked over the whole observation window |
| T6 | `validateTicketImportantDirectories` | full entry scan per ticket-important directory (**16.8 s**) | Children-by-directory map | Same children, insertion order |
| T7 | mutation pairs and artifact projections | `[...entries.values()].filter(startsWith(root/))` per root (several call sites) | `entrySubtree`: one code-unit-sorted index per append-only entry map; prefix ranges by binary search, returned in insertion order | Same members, same order (law: random maps vs rescan) |
| T8 | `projectArtifactCatalogs` → `semanticPathProjectionAuthority` | per candidate root: copy and sort every entry path, then four NFC/fold keys per path (**86 s**) | `occupiedPaths` is one shared array; discovery indexes its keys once per array (WeakMap); `occupiedExclusionRoot` excludes the projection's own subtree at lookup time | A key collides iff some occupant outside the source subtree has it — the exact old predicate |
| T9 | `memberNames.filter(emojiFold …)` per directory (projection source, projected member) | O(5,062) per call | Per-registry fold map | Same matches |
| T10 | Markdown link-reference definitions | `/^\s*\[[^\]]+\]:\s*(\S+)/gmu` rescans to the next `]` from every line start: quadratic; **1,051.7 s on one file, 252.6 s over 23,842 Markdown files** | `markdownReferenceDefinitionTokens`: one linear scan that reproduces the regex's match positions and `regexTokens`' `indexOf` quirk | 23,845 files: 0 differences (0.64 s vs 252.55 s); a 5,000-case random law agrees with the regex |
| T11 | `markdownSourceCoordinateSpans` | per line: `findIndex` per backtick run and string rebuilds | Precomputed next-same-length run; blanked intervals instead of rebuilt strings | 23,842 files: 0 differences |
| T12 | TOML tokens | slice of the whole prefix plus a full header regex for every string: quadratic | All headers matched once; binary search for the last header ending before the string (exact because the prefix always ends in a quote) | 854 files: 0 differences (5.85 s → 0.09 s) |
| T13 | reference resolution (`resolveReferencePath`, `lexicalOpaqueReferenceTarget`) | about 6 `normalize` / `posix.normalize` calls per token on millions of tokens (**462 s**) | Leaf prefilter: a candidate can only resolve if its normalised final segment is a leaf of some exact/NFC/extensionless key. The opaque check needs the token to name an exclusion's last segment, because the referencing file is never inside one | 2,807,649 real tokens against the real index: 0 differences (49.2 s → 12.4 s) |
| T14 | `unprovenRustReferenceTargets` | scans all 266 k paths per distinct rust join value (**87 s**) | Exact paths grouped by final segment | Same filter over the only possible candidates, iteration order kept |
| T15 | `inspectRustNonRepoJoinBaseSpans` → `helperReturnsNonRepo` | scans every token per called helper name | Lazily built `fn NAME` declaration index | Same match list |
| T16 | `typescriptCollectionSyntax` | `content.slice(cursor)` for every operator token: quadratic copy | Sticky operator regex | Same tokens |
| T17 | plan `buildReferenceEdits` | O(unsupported × supported) `rustReferenceInterpretationCovers` scan for every candidate, including non-rust files (stalls minutes on the 6.3 MB `launch.json`) | Pre-filter to rust tokens; the predicate is false unless both adapters are rust | Same predicate |
| T18 | `applyEditsToContent` | one full-string splice per edit: quadratic (**39 s** on launch.json) | One-pass assembly for non-overlapping edits; it falls back to the original splice from the first overlap on | 20,000 random cases including overlaps and preimage mismatches: same results and same error messages |
| T19 | `historicalDocumentEvidence` | `readdir` of every ancestor per candidate path; a regex compiled per call (**83.6 s**) | Per-scan directory → owns-manifest memo; regex memo; fixed-contract leaf index | Same answers |
| T20 | `referenceCoordinateRoots`, incoming observation, rust graph reads | `assertLexicalInputOutsideOpaque` lstats every ancestor per input | `createLexicalInputSession`: per-session ancestor memo, same checks and messages | Same refusals |
| T21 | byte-order sorts (`generatorPathCompare`, source-admission sorts, directory and leaf sorts) | two `Buffer.from` allocations per comparison | `utf8Compare`: allocation-free code-point order (surrogate fix-up) | 200,000 random pairs plus vectors vs `Buffer.compare`: 0 mismatches |
| T22 | `canonicalJson` arrays | key plus Buffer recomputed on every comparison | Keys encoded once, stable sort | 3,000 random documents: 0 mismatches |

A sticky-regex replacement for the JSON key-colon lookup was tried and reverted. JSC substrings are O(1), so it gained nothing (11.9 s vs 12.4 s over 13,410 files).

The remaining inventory cost is genuine per-file work: reading 155 k files about twice (`readFileSync` 87 s self), tokenising 130 k text files (JSON 45 s, TypeScript 43 s, Rust 38 s), file classification 64 s, and fixed-source program parsing 32 s.

---

## 4. Nx graph root causes and fixes

### 4.1 Admission walk (`⚡️caching/🚀️bootstrap/📜️script.ts`)

- **Before:** a serial `await lstat` plus `readdir` for each of 58,879 directories, then 58,879 serial custody `lstat`s. That is 18.2 s before Nx even starts.
- **After:** a bounded parallel walk (64 in flight) that is also incremental.
  - A directory whose (dev, ino, mtime, ctime) matches the previous snapshot reuses its recorded child directories and admitted manifest names. Adding, removing or renaming an entry restamps the directory.
  - A racy-git rule applies: stamps younger than 2 s are never persisted.
  - Every visited directory and admitted manifest is still re-proven real and unchanged before publication, as before.
  - The snapshot lives in the excluded repository cache (`.🧬semio/🦑️repo/⚡️cache/nx-admission/`), so writing it never perturbs the walk. It is rewritten only when something changed.
- **Proof:** `.nxignore` stays byte-identical across cold and warm runs, and the `⚡️production-cache-input-boundary` law passes 5/5. That law covers mid-walk abort, symlink swap, peer edits, and invalidation after a manifest is unlinked.

### 4.2 Inference observation cache (`🔌️nx-plugin/👁️observation/🟨️.mjs`, new)

- **Facade.** Graph inference reads the filesystem through `observedFs`, which records what each read proved:
  - file reads: stamp plus digest;
  - existence probes;
  - stat and lstat type probes;
  - directory listings;
  - real paths;
  - module resolutions (`observedRequireResolve`, and a TypeScript host that delegates to `ts.sys` and records its probes).
- **Recording scope.** Facts are recorded in the active `AsyncLocalStorage` scope, or, until the first inference begins, as module-load facts.
- **`observeInference`.**
  - It persists the result keyed by implementation revision, config files, options and workspace root.
  - It reuses that result only while every recorded fact still holds.
  - A restamped file with unchanged bytes still counts as a hit; this matters with peer lanes touching files.
  - A result is persisted only if it is exactly JSON-representable, no input file is younger than 2 s, and the facts still hold right after computing.
  - The persisted file is a header line (facts) followed by a result line, so a miss never parses the 140 MB result.
- **Wiring.** Both plugins' `createNodesV2` go through it. The modules in the inference closure import `observedFs`: library `🟨️.mjs`, `🕸️dependencies/🟦️typescript`, `🧪️test/🟨️.mjs`, `🧪️test/🕸️dependencies`, and `💻️os/…/🧭️selection`.
- **Implementation revision.** `implementationRevision()` now also hashes the arguments module, the observation module and the component-receipt schema. Before, editing those did not hot-reload the daemon.
- **Law.** `👁️observation/🧪️tests/🟦️.ts` with fixture and schema passes 10/10. It covers hit, identical rewrite, content change, same-size change, a probed absence becoming present, a listing change, an unobserved change, and a directory replaced by a file. It also covers that an inexact result is never persisted. The oracles are Ajv on the persisted record and fast-glob for the probed listing.

### 4.3 Proof that the graph is unchanged

Three daemonless `nx graph --file` runs were made back to back: new cold (A/D), the snapshot plugin swapped in (B/E), then new warm (C/F).

- **New warm vs new cold:** byte-identical (201,541,488 bytes; nodes and dependencies hashes equal).
- **Snapshot vs new:**
  - Dependencies are identical.
  - Nodes differ only by one added input line: `{workspaceRoot}/…/🔌️nx-plugin/👁️observation/🟨️.mjs` in the script-closure inputs of targets whose scripts import the library. That is correct: the module is now part of the graph code.
- **What the earlier diffs were:** the first comparison (05:36 vs 05:42, both old code) already differed by Cargo dependency edges from peers' `Cargo.toml` edits. That is tree churn, not code.

---

## 5. Pre-existing blockers found (not performance; both throw instead of reporting)

1. **Overlapping reference edits in `.vscode/launch.json`.**
   - The planned move `.vscode/launch.json → .vscode/🔣️.json` produces an embedded-argv edit at offset 5,180,952 (the token appears twice, identical) that overlaps a wider edit.
   - `applyEditsToContent` throws `Reference edit preimage mismatch at .vscode/🔣️.json:embedded-argv:96584:676@5180952`.
   - Tokens and resolution for launch.json are proven identical old vs new (§3 T13 differential; jsonTokens differential), so the old code throws identically. It simply never got there.
2. **Frozen evidence digest drift.**
   - `🧰️framework/…/📚️library/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json`, last modified 2026-10-08, no longer matches the digest registered in `🔣️taxonomy.json`.
   - `frozenCoordinateEvidenceCoordinates` throws while the incoming-reference scan reads it. Every scope that plans a move hits this; on today's tree that is every scope I probed.
3. **`repo:test-graph-revision`** was already failing on the snapshot plugin:
   - its fixture lacks modules that the plugin closure loads (`📤️arguments` schemas);
   - its expected executor and cache values are stale (`@semio-tech/repo-lib:owner-command`, not `nx:run-commands`).
   - I added the missing closure files, including the new observation module. The stale expectations remain.

Recommendation: the planner should turn both into violations (per-file "overlapping reference edits", "frozen evidence digest drift") rather than abort the whole gate. Then the gate can report on today's tree.

---

## 6. What remains for the targets

- **Taxonomy.** Per-file work that every run repeats remains. It is the main term at 10× scale.
  - **Content-addressed token store:** tokens are a pure function of (path, content) for every adapter except rust's manifest-ownership part. Store them per shard under `⚡️cache/taxonomy/`. Warm runs then skip tokenisation in both inventory and plan.
  - **Parallel tokenisation:** a worker pool. The inventory and plan entry points become async, or reference outcomes are computed in an async pre-pass the sync pipeline consults.
  - **Expected effect (estimate):** references (226 s) and plan references (about 220 s) drop to read + hash + resolve, about 20–40 s cold on 8 workers, a few seconds warm. Files (190 s) are next.
  - Plan references re-tokenise every inventory file a second time. Sharing tokens between inventory and plan is not possible in memory: every JSON string is a token, millions in total. The persistent store solves that too.
- **Nx.** What remains is graph size (N2) and Nx's daemonless plugin-resolution glob and cache write.

---

## 7. Verification run

- **Inventory identity (old snapshot vs new, same process, back to back):**
  - Identical for `🎓️teaching`, `🏢️semio-tech`, `♻️mit-bestand`, `📚️library`, `🧰️framework/🔨️modules` (10,919 entries), `🧰️framework/🛍️products/💻️os` (11,078 entries) and `🌎️hub`.
  - Identical for `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08`: 24,322 entries, 17,011 violations, same inventory digest. The new code took **73.0 s**, the old code **2,697.8 s**, so this Markdown-heavy scope is 37× faster.
  - The first `💻️os` pass differed only by directory content hashes (a peer edited a file between the two reads). The rerun was identical.
- **Differentials** (old vs new over the real corpus) all showed 0 differences:
  - markdown spans: 23,842 files;
  - link-reference definitions: 23,845 files;
  - TOML: 854 files;
  - resolution: 2,807,649 tokens;
  - `canonicalJson`, `utf8Compare` and `applyEditsToContent`: random laws.
- **New laws:**
  - `🧪️tests/⏱️taxonomy-scaling-laws` passes 6/6 (picomatch oracle; original regex oracle; byte oracle; naive rescans).
  - `👁️observation/🧪️tests` passes 10/10.
- **Existing suites touched by the change** (new code, compared with the in-place snapshot baseline from `🔁️baseline-swap.sh`) are all equal to or better than the baseline:
  - pass, no regressions: `🚪️source-admission` 90/0, `🚪️source-admission-io` 11/0, `🧪️multi-scope-verification` 1/0, `⚡️production-cache-input-boundary` 5/0, `📈️reference-coordinate-progress` 21/0, `🛫️preflight-reference-basis` 30/0, `🥤️rust-finite-target-consumption` 86/0, `🛤️typescript-path-collection` 10/0, `🚚️readme-move-source-authority` 17/0, repo `test-cache-policy`, `test-native-input-vocabulary` and `test-cache-command-source`;
  - the same counts as the snapshot baseline (pre-existing failures): `🔏️path-emoji-statutes` 33/4, `♻️taxonomy-pattern-compiler-reuse` 16/5, `✍️rust-writable-path-authority` 1/46, `🔄️transaction-v2` 0/62 (run-identity environment), `🟢️readme-current-source-activation` 19/20, `📦️package-boundary-classification` 115/5, `🥇️bare-reference-sibling-precedence` (all fail on `export` in the extracted closure, old and new).
  - `🔬️workspace-contract` has one extra failure versus the baseline: `resolveCargoPackageName` hits its 5 s budget under load. It passes with a longer timeout (36 s run), and the code involved is not touched by this change.
- **Full sweep** (132 library and test-plugin test files, plus 6 more test files that load or copy the graph plugins).
  - It ran during the gate, so the files that failed were rerun one at a time afterwards. Every file still failing was run again against the snapshot sources with `🔁️baseline-swap.sh`; the restore was verified byte-identical.
  - **Transient under gate load, pass alone:** `🏠️nx-workspace-root-file-reference` 7/0, `🔎️json-reference-owner-lookup` 7/0.
  - **One real regression, found and fixed:** `🧪️test/🧪️tests/🚷️discovery-boundaries` was 4/1 against a baseline of 5/0.
    - Cause: the test builds a resident copy of the test-cases plugin in which `📚️library/🟨️.mjs` is a re-export shim, and the copy now also needs `📚️library/🔌️nx-plugin/👁️observation/🟨️.mjs` (`ERR_MODULE_NOT_FOUND`).
    - Fix: the test adds a matching re-export shim for the observation module. The test now passes 5/0, and its stale-ESM refuse/accept law still holds on Node and Bun.
  - **Unchanged: same counts and the same failing test names as the snapshot baseline.** Each entry is pass/fail; the swap baseline covered every file:
    - `🥾️cross-platform-bootstrap` 8/4
    - `🧑‍💻os-dev-composition-ownership` 9/2
    - `🧪️mutation-leaf-identity` 9/1
    - `🧰️framework-root-source-topology` 3/1
    - `🧱️manifestless-source-closure` 4/4
    - `🧱️root-artifact-dependency-source` 4/2
    - `🧱️root-surface-abstraction-law-source` 8/1
    - `🧱️rust-source-direction` 17/5
    - `🧱️rust-source-direction/🧾️attributes` 15/1
    - `🧱️workspace-publication-source` 9/1
    - `🧲️rust-physical-reference-context` 32/1
    - `🧾️registry-catalog-gitlink-boundary` 2/1
    - `🪟️windows-command-paths` 16/3
    - `🪶️artifact-empty-facet-authoring` 40/3
    - `🧱️wasm-package-wrappers` 7/1
    - `📋️native-orchestration/🧪️tests/📬️test-body` 7/1
    - `📥️inference/🧪️tests` 2/1
    - `📥️inference/🧪️tests/🔍️imports` 15/4
    - `⚖️laws/indexed-generated-output` 0/1
    - `📤️arguments/📥️native-caller` 14/1
    - `🎯️precise-command-inputs` 0/1
    - `🎭️selection` 4/2
    - `🏎️nextest-configuration` 0/2
    - `🧭️native-discovery` 0/1
    - `↪️rust-divergence-callback` 47/1
    - `☂️frozen-coordinate-wildcard-coverage` 2/3
    - `🍃️artifact-support-leaf-authority` 16/3
    - `🎚️vitest-configuration-ownership` 5/2
    - `🎟️reference-coverage-selection` 4/1
    - `🏺️historical-package-owner-identity` 24/2
    - `👀️readme-reviewed-fixture-inputs` 0/1
    - `📜️historical-document-evidence` 6/1
    - `📱️app-verification-source-ownership` 2/4
    - `🔗️markdown-inline-references` 0/1
    - `🔤️taxonomy-leading-grapheme` 9/1
    - `🕰️historical-json-source-encoding` 23/2
    - `🛟️transaction-recovery-authority` 4/2
  - Every other bun test file in the sweep passes.
  - The 23 script-driven proof modules in the sweep (exported `test…` functions, 0 bun tests) run through Nx targets instead.
    - `repo:test` (the full cache suite) stops first at `test-graph-revision`. That failure is pre-existing (§5.3): stale `nx:run-commands` executor expectations versus `@semio-tech/repo-lib:owner-command`. The closure-module check in front of it now passes.
    - That `nx run` took 20 s end to end with the warm inference cache.
    - The probe left `🗑️generated/nx/graph-revision-W0OzDg` behind in this ticket's generated folder. I did not touch it.

---

## 8. Files touched

- **Taxonomy and normalization** (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`):
  - `🧹️normalization/🟦️.ts`
  - `🧹️normalization/📁️input/🟦️.ts`
  - `🧹️normalization/🛣️path/🟦️.ts`
  - `🧹️normalization/🚪️source-admission/🟦️.ts`
  - `🧹️normalization/🚪️source-admission/📁️io/🟦️.ts`
  - `🔍️discovery/🟦️.ts`
  - `🧾️serialization/🔣️json/🟦️.ts`
- **Nx:**
  - `⚡️caching/🚀️bootstrap/📜️script.ts`
  - `🟨️.mjs`
  - `🕸️dependencies/🟦️typescript/🟨️.mjs`
  - new `🔌️nx-plugin/👁️observation/{🟨️.mjs, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json, 🧪️tests/🟦️.ts}`
  - `../🧪️test/🟨️.mjs`
  - `../🧪️test/🕸️dependencies/🟨️.mjs`
  - `../🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts` (resident-copy shim for the observation module)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs`
  - `⚡️caching/🧫️fixtures/🔁️graph-revision/🔣️.json`
- **Tests:**
  - new `🧪️tests/⏱️taxonomy-scaling-laws/🟦️.ts`
  - new `🧫️fixtures/⏱️taxonomy-scaling-laws/🔣️.json`
  - closure registrations in `🧪️tests/{♻️taxonomy-pattern-compiler-reuse, 📈️reference-coordinate-progress, 🔏️path-emoji-statutes, 🔗️markdown-inline-references, 🔬️workspace-contract, 🚚️readme-move-source-authority, 🛤️typescript-path-collection, 🛫️preflight-reference-basis, 🥇️bare-reference-sibling-precedence, 🥤️rust-finite-target-consumption}/🟦️.ts`
- **Ticket inputs** in `taxonomy-graph-perf/`:
  - `⏱️taxonomy-phases.ts`
  - `🔥️cpuprofile-top.ts`
  - `🧬️differential.ts`
  - `🧭️resolution-differential.ts`
  - `⚖️inventory-identity.ts`
  - `⚖️scope-identity.sh`
  - `🕸️plugin-differential.mjs`
  - `🧪️run-tests.sh`
  - `🔁️baseline-swap.sh`
  - `🧩️closure-names.py`

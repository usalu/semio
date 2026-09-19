# V1 — Verification gates (2026-09-19)

Slice V1 of ticket 26/09/18 OS-HUB-COLLABORATION-AI-END-TO-END. Handed the three findings P1 left open
(`📓️p1-catalog-and-registry-hygiene.md` "Findings handed back" §6, §3, §2) plus "run the other cheap
`verify …` gates from the root `📋️project.json`, record each one's status, fix what is in reach".

Everything below was **executed on this machine**, not inferred; captures are in `🗑️generated/v1-*.txt`.

---

## 1. Gate matrix

| # | Gate | Before (P1, 2026-09-18) | After (V1, 2026-09-19) |
|---|---|---|---|
| 1 | `verify taxonomy report` | **unrunnable** — threw `frozen-coordinate-evidence-invalid … 📐️cad-draw-path-projection` before any finding | **unblocked and running** — the digest abort is gone; the walk itself did not finish inside this slice (§2) |
| 2 | `plugin-registry check` | exit 1, **2299** violations | exit 1, **1836** violations (was 1853 when I started; **−17 fixed**, §3.5). Censused by family in §3 |
| 3 | dsl fixture-sweep extraction oracle | reported "wholesale stale" | **exit 0, green**, 189/288 non-zero discovery (§4) |
| 4 | `plugin-registry test` (vitest, 8 suites) | not run | **3 failing tests + 1 failing suite → 1 failing test** (45 tests, 44 pass). §3.6 |
| 5 | `plugin-registry rust-taxonomy-mounts-check` | exit 0 (9 cases) | **exit 0 (10 cases)** — new case added with the gate fix (§3.5) |
| 6 | `plugin-registry plugin-root-ownership-check` / `native-catalog-selection-check` | exit 0 | exit 0 (7 / 23 cases) |
| 7 | `plugin-registry check-generated` | exit 0 | oscillates — green after my `generate`, stale again minutes later (a peer is live-editing `🚀️launch/🏷️name-prefix/🟦️.ts`). §5.5 |
| 8 | `verify layering` | not run | exit 1, **213** files past baseline (5972 refs over 235 files) — §5.1 |
| 9 | `verify dependencies literal-external` | not run | exit 1, **236** literal-external + 15 oracle-conflicts + 2 toolchain-owner-conflicts — §5.2 |
| 10 | `verify dependencies` (freeze) | not run | exit 1, **7 new** third-party deps (230 → 237) — §5.3 |
| 11 | `verify package-purity` | not run | exit 1, **434** breaches — §5.4 |

Commands: #2–#7 from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`
(`bun ./📜️script.ts <verb>`); #3 from `…/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust`; the rest
`bun ./📜️script.ts verify …` at the repo root.

**Bottom line.** One handed-back item (fixture-sweep) is green; one (`verify taxonomy`) is unblocked;
the third (`plugin-registry check`) is a structural migration backlog that cannot honestly be driven to
zero by one slice — 615 of its findings need ~3 100 handcrafted schema files. What I could do at the
root, I did: **one gate false-positive family fixed (−17, with a new compiler-checked oracle case), two
stale laws repaired, one vitest self-collection bug fixed**, and a census that names the owner and the
recipe for every remaining family.

---

## 2. `verify taxonomy` — the digest block is gone; the walk is the new problem

P1's symptom was that `frozenCoordinateEvidenceCoordinates`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:7004`) threw
`document digest does not match registered bytes` for
`…/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json`, aborting the planner at any scope before
a single finding.

I verified every registered contract independently instead of trusting the gate's message — all 41
`frozenCoordinateEvidenceContracts` in `…/📚️library/🔣️taxonomy.json` recomputed with `hashlib.sha256`
over the on-disk bytes:

* **13 contracts resolve to a file on disk, and all 13 digests match**, `📐️cad-draw-path-projection`
  included. The digest block no longer exists at HEAD; **no digest edit was needed and none was made.**
* **28 contracts point at files that no longer exist** — 27 `📊️sol-*.json` / `🧪️coordinator-*.json`
  evidence captures under `.🧬semio/…/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/`, one under
  `…/☀️23/END-TO-END-TESTING-REFACTOR/w14-audit/`, one under
  `…/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/`. They were deleted when those tickets closed, as
  AGENTS.md requires. They do not break the gate (`frozenCoordinateEvidenceCoordinates` only reads
  bytes for the path it is asked about, and `.🧬semio/🦑️repo/🎫️tickets` is an `exempt` area), but they
  are 28 dead registrations in root taxonomy data — see §6 gap 1.

**Runtime status.** `bun ./📜️script.ts verify taxonomy report` starts, gets past
`planMoveReferenceAuthority` (the frame that used to throw instantly) and keeps walking: it ran
**~1 h 50 m wall / 33 min CPU** and had not produced output when this slice closed. It is not hung —
CPU stayed at 25–100 % throughout and RSS peaked near 4.7 GB before the host pushed it into swap
(`vm_stat` showed ~86 MB free pages under the running fleet). The gate is therefore **runnable but not
affordable on a loaded host**; its finding count is unknown and is the one number in §1 I cannot
report. The capture it will write is `🗑️generated/v1-taxonomy-before.txt` (still empty at close).
`verify taxonomy` buffers every violation and prints only at the end
(`📜️script.ts:8194-8198`), so nothing partial is observable. Anyone picking this up should either run
it on an idle host or add incremental output the way
`verify taxonomy implementation` already has (`📜️script.ts:8179` prints a `phase=… visited=…`
progress line every 4096 paths — the plain `verify taxonomy` path has no such reporter).

---

## 3. `plugin-registry check` — census of the violations, and what I fixed

The gate first failed on a *different* cause than P1 saw:

```
plugin registry catalog is stale: .vscode/launch.json
run `bun nx run @semio-tech/plugin-registry:generate` to refresh.
```

`generate` re-rendered it (`.vscode/launch.json`, +11 lines; the registry is the authority for that
file) and the staleness gate went green — only then does the taxonomy-tree audit run at all. Captures:
`🗑️generated/v1-plugin-registry-check.txt` (before, 1853 findings) and
`v1-plugin-registry-check-after.txt` (after, 1836), classified with `🐍️v1-violation-census.ts` (written
by the previous V1 worker, reused unchanged).

### 3.1 By rule family (before my fix)

| count | family | root cause |
|---|---|---|
| 892 | `unreachable-from-cargo-manifest` | a `🦀️.rs` / example leaf on disk that no `#[path]` mount in any owning Cargo target reaches |
| 317 | surface missing `🎚️config/🧬️schema/` | W3 surface migration incomplete |
| 298 | surface missing `👥️presence/🧬️schema/` | same |
| 197 | other (§3.2) | mixed |
| 66 | artifact subset missing `📚️examples/` | subset migration incomplete |
| 48 | artifact subset missing `🚪️io/` | same |
| 34 | `plugin root is missing 🎮️commands/🦀️.rs` ×33, `… 🦀️.rs` ×1 | §3.4 |
| 1 | `window has unexpected child` | §3.4 |

### 3.2 The "other" bucket, normalised

| count | shape |
|---|---|
| 61 | `mode "<s>" is missing required child "<c>"` (`🎚️config`/`👥️presence`/`🫧️transient` lane absent) |
| 24 / 22 / 22 / 10 | subset example missing `🟦️.ts` / `🦀️.rs` / `🧪️tests/` / `🖼️assets/` |
| 17 | `missing module target "<m>" from …` — **all false positives, fixed, §3.5** |
| 13 | artifact subset missing `🧬️schema/` |
| 6 + 6 | plugin-root missing `🎚️config/🧬️schema/<f>` and `👥️presence/🧬️schema/<f>` for the six schema formats |
| 4 | artifact missing `🏅️standards/` |
| 1 | `move the redundant 🔌️plugin contract … and remove 🔌️plugin/` |

### 3.3 By plugin — the backlog is one plugin, not thirty-four

```
1072 🗄️stdio      201 📕️norm     73 🏗️fem      44 ➗️mathematical   41 🗒️note
  39 🀄️wfc        33 🎞️animate   31 🖍️draw    24 🌊️flow            24 🎬️sequence
  23 🧱️block      19 🧩️puzzle    18 📏️layout  16 📐️cad / 🔋️energy  … 20 more at ≤15
```

**58 % of every violation in the repo is `🗄️stdio`** (576 unreachable mounts, 176 + 176 missing surface
schema lanes, 39 + 21 missing subset `📚️examples`/`🚪️io`). `🗄️stdio` owns ~40 nested artifact crates,
one per file format, so a single un-migrated pattern multiplies by 40. Any campaign to zero should
start there and will remove more than half the count in one pass; `📕️norm` (201) is second, `🏗️fem`
(73 → 57 after my fix) third.

### 3.4 Gate-correct vs. real data, with the evidence for each verdict

* **`surface … is missing 🎚️config/🧬️schema/` + `👥️presence/🧬️schema/` (615) — real data, gate is
  right.** `assertAppSchemaOwner`
  (`…/📇️registry/🗿️taxonomy-validation/🟦️.ts:576-611`) requires the lane **and** all six
  `TAXONOMY_SCHEMA_FILENAMES` leaves under it. Counted on disk: **286 surface `🎚️config` lanes exist,
  only 30 carry `🧬️schema/`** — the pattern is real and ~10 % migrated. Closing it means hand-authoring
  ≈ (256 + 268) × 6 ≈ **3 100** schema files (`🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto`,
  `📜️.wit`). **Not attempted** — inventing them empty is exactly the fabrication AGENTS.md forbids.
* **`plugin root is missing 🎮️commands/🦀️.rs` (33 of 34 plugins) — real, but the honest fix is a
  root-data decision, not 33 stub modules.** The rule is root taxonomy data
  (`🔣️taxonomy.json` `pluginRequiredChildDirs = ["🎮️commands"]` + `TAXONOMY_LEAF_FILENAME`), enforced at
  `🗿️taxonomy-validation/🟦️.ts:241-243` and independently re-derived by the SQLite oracle at `:713`.
  All 34 plugins **do** have `🎮️commands/`; 33 carry only the `📌️.empty.md` tracked marker. Exactly one,
  `🌀️procedural`, has real content (`✏️s/🔌️plugins/🌀️procedural/🎮️commands/🦀️.rs` — one plugin-scope
  command `listFlowExtensions` filling `PluginManifest::commands`). The other 33 legitimately publish
  no plugin-scope command. The taxonomy already models "an empty lane is valid when it carries only the
  tracked marker" for mode children (`🗿️taxonomy-validation/🟦️.ts:497-501`); the plugin-root rule does
  not. **I did not pick for the taxonomy owner** — recipe in §6 gap 3.
* **`🎪️demonstrator: plugin root is missing 🦀️.rs` (1) — real structural divergence.** Every other
  plugin keeps its `Plugin::builder` typestate registration in the plugin-root `🦀️.rs` (e.g.
  `✏️s/🔌️plugins/🗒️note/🦀️.rs:1`, "🔌️ Plugin root contract"). `🎪️demonstrator` has none; its contract
  lives in `✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs`, mounted from
  `📦️packages/🦀️rust/🦀️.rs:42`. **Not moved** — `🎪️demonstrator` is live under slice B3d; recipe in §6
  gap 4.
* **`📐️cad: window "✳️any/✏️editor/✏️edit/🎚️config" has unexpected child "🧬️schema"` (1) — real
  misplacement, and *not* the live peer rename.** `🪟️windows/🎚️config/` is not a window: it holds
  `CadWorldWindowConfig` (`…/🪟️windows/🎚️config/🦀️.rs:14`), the config type **shared** by the four real
  sibling windows (`📐️shape`, `🏢️building`, `🔥️energy`, `🏛️structure-classic`, each of which carries the
  proper `☑️options 🎚️config 🎬️actions 👥️presence 🪛️utilities 🫧️transient` child set). `git ls-files`
  shows it tracked under this exact name since commit `3250e6cb90` (2026-09-15), so it predates the
  live `⚙️config`→`🎚️config` / `🎚️options`→`☑️options` sweep. Its content belongs in the mode's own
  config lane `🎭️modes/✏️edit/🎚️config/`, which today holds only `📌️.empty.md`. **Not moved** — `📐️cad`
  is live under slice B3d; recipe in §6 gap 5.

### 3.5 Fixed: `missing module target` (17) was a gate false positive

All 17 findings (16 `🏗️fem`, 1 `🗄️stdio`) named files that **exist on disk**, verified with `ls`:

```
🏗️fem: missing module target "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs" from 🗿️artifacts/◻️2d/🦀️.rs
  → ✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs   124 624 bytes, Sep 16 18:55
🗄️stdio: missing module target "../../🗒️note/…/✳️any/🔮️oracles/🦀️.rs" from 🔮️oracles/🦀️.rs
  → ✏️s/🔌️plugins/🗒️note/…/✳️any/🔮️oracles/🦀️.rs   5 437 bytes, Sep 15 14:19
```

Cause: `validateRustTaxonomyMounts` builds its module graph from the files under **one owner root**
(`walkPluginTree`, `🗿️taxonomy-validation/🟦️.ts:530-547`, never descends outside `pluginRoot`). A
`#[path]` that climbs out of that root — a plugin artifact mounting a shared
`✏️s/🔨️modules/🏗️fem/⚙️engine` leaf, exactly what `🏗️fem` does 16 times — can therefore never appear in
`graph.targets`, however correct it is, and was reported as missing.

Fix (root, in the gate):

| File:line | Change |
|---|---|
| `…/📇️registry/🗿️taxonomy-validation/🟦️.ts:195-205` | new `escapesOwnerRoot(ownerRoot, mountingFileRel, pathTarget)` helper |
| `…/🗿️taxonomy-validation/🟦️.ts:229` | `if (module.pathTarget !== null && escapesOwnerRoot(…)) continue;` before the `graph.targets` lookup — a mount whose membership belongs to another area's audit is not judged here |
| `…/🗿️taxonomy-validation/🟦️.ts:648-655` | oracle now materialises a case's `neighbourFiles` **beside** the owner root (`../<id>-neighbour/…`) and keeps `sourceFiles` root-bounded, mirroring `walkPluginTree` |
| `…/📇️registry/🧬️schema/🔣️.json:145-159` | `RustTaxonomyMountsV1`: `cases` 9 → 10, optional `neighbourFiles` map |
| `…/📇️registry/🧫️fixtures/🕸️rust-taxonomy-mounts/🔣️.json` | new case `neighbour-owned-path-target`: owner root mounts `../<id>-neighbour/⚙️engine/🦀️.rs`, `expectedCodes: []`, `rustcSuccess: true` |

The new case is **compiler-checked, not self-asserted**: `RustTaxonomyMountsCheckScript` compiles every
case with a real `rustc` and compares the registry's verdict against the compiler's own dep-info, so
the case proves `rustc` accepts the escaping mount while the validator now agrees.

```
$ bun ./📜️script.ts rust-taxonomy-mounts-check
registry-rust-mounts-oracle cases=10 ajv=1 compiler=10            # exit 0
```

Effect on the real repo, by diffing the two captures — **exactly the 17 false positives disappeared and
nothing else changed**:

```
$ diff <(tail -n +2 before) <(tail -n +2 after) | grep '^[<>]' | count by plugin
  16 <  🏗️fem
   1 <  🗄️stdio        # 1853 → 1836, zero lines added
```

Neighbouring oracles re-run after the change, all green:
`check-generated` (at the time), `plugin-root-ownership-check` (7 cases, ajv=1, sqlite=7),
`native-catalog-selection-check` (23 cases, 4 positive / 19 denied).

### 3.6 Fixed: two stale laws and one vitest self-collection bug

`bun ./📜️script.ts test long` (the registry's own vitest target; the default `test` level is killed by
a 15 s budget before the suites finish — use `long`) started at **3 failed tests + 1 failed suite of 8
files / 45 tests** and ends at **1 failed test, 44 passed, 7 files**.

1. **`🧪️tests/🎚️config/🟦️.ts` collected as a test file** — `Error: No test suite found`. The project's
   vitest **config** sits at `🧪️tests/🎚️config/🟦️.ts` and the config's own
   `include: ["🧪️tests/*/🟦️.ts"]` matches it. Fixed at
   `…/📇️registry/🧪️tests/🎚️config/🟦️.ts:19` by adding `"🧪️tests/🎚️config/🟦️.ts"` to `exclude`
   (which already had to list `📚️storybook-plugins` explicitly, because a custom `exclude` replaces
   vitest's defaults). Note for whoever owns the other three projects with the identical shape
   (`…/🌉️mcp/🧪️tests/🎚️config/🟦️.ts`, `…/🖥️server/🎛️coordinator/…`, `…/💻️client/🪶️sqlite/…`): they carry
   the same self-match.
2. **`WASI codegen profile policy > keeps generated native, root preflight, and MCP runtime profiles
   identical without debug`** asserted that `…/📇️registry/🎮️playground/🧭️session/🟦️.ts` declares
   `PLUGIN_WASM_PROFILE_DIRS`. `git log -S PLUGIN_WASM_PROFILE_DIRS` on that file returns **nothing** —
   it has never contained the constant, so `find(...)` returned `undefined` and the assertion died on a
   type error rather than a comparison. The real fourth declaration site is
   `…/📇️registry/📽️projection/🟦️.ts:298` (the generator that emits the constant into
   `🤖️generated/🗿️artifacts/🦀️.rs`). Fixed at `…/📇️registry/🧪️tests/🚀️launch/🟦️.ts:116` by pointing the
   law at `📽️projection/🟦️.ts`; the law's intent (every declaration is exactly
   `["wasm-dev", "wasm-release"]` and never `"debug"`) is preserved and now actually exercised on four
   live files.
3. **`WASI codegen profile policy > independently validates all neutral profile routes and native Cargo
   policy`** compared root `Cargo.toml`'s `[profile.wasm-dev.package]` against
   `…/🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json` `developmentPackageOverrides`. The fixture
   still named `semio-s-plugin-lowpoly` / `semio-s-plugin-puzzle` — **crate names that no longer
   exist** (renamed to `semio-s-artifact-lowpoly-lowpoly` / `semio-s-artifact-puzzle-3d`) — while the
   manifest has grown to 10 documented overrides. The manifest is the authority (each override carries
   its own measured rationale at `Cargo.toml:519-573`), so the fixture was re-derived from it, in place
   and in its existing single-line style (1 line changed, no reformatting).

**Still failing, deliberately left: `registers one react dev launcher for every playground variant`.**
The test asserts inside its loop, so it only ever reports the first bad variant (`aggregator = 2`).
I measured the whole surface with `🐍️v1-launcher-match-probe.ts` (this folder):

```
react launchers: 74, playgrounds: 65
current law (env SEMIO_PLUGIN == pluginId OR command contains "-- <variant>") : 45 of 65 variants ≠ 1
command endsWith "-- <variant>"                                              : 23 of 65 variants = 0
```

Two independent defects, neither of them mine to land: (a) the law's first branch keys on
`env.SEMIO_PLUGIN === pluginId`, which stopped discriminating the moment a plugin gained a second
playground variant — `demonstrator` now has **7** (`aggregator`, `aussuchen`, `bearbeiten`,
`demonstrator`, `generator`, `koordinator`, `verfolgen`), so every one of them also matches the
`demonstrator` row; (b) **23 of 65 playground variants have no launcher in the generated shape at all**
— e.g. `block2d`'s row is `bun nx run workspace:dev -- block 2d` with a bespoke `BLOCK_2D_PLAY_PORT`
instead of `-- block2d` with `S_OS_PORT`/`SEMIO_PLUGIN`. Normalising 23 legacy rows is a
`🚀️launch/🟦️.ts` + `🧩️launch.seed.jsonc` migration, and a peer is editing
`…/📇️registry/🚀️launch/🏷️name-prefix/🟦️.ts` **right now** (unstaged in `git status` throughout my run).
Tightening the predicate without migrating the data would only change which 23 variants fail. Handed
on with the numbers — see §6 gap 6.

### 3.7 Non-zero discovery asserted

Per the coordinator's warning that `fileKinds` drift can make discovery silently return 0 so a gate
reads green, every count above was cross-checked against an independent walk:
`findNewContractPluginRoots` discovered **34** plugin roots (matching the census's `plugins cited: 34`
and the 34 directories under `✏️s/🔌️plugins`); `generate` reported **60 plugin crates / 65 playgrounds /
54 framework packages**; the fixture-sweep oracle reported **189 example directories / 288 asset-first
`.semio` files**; the launcher probe enumerated **74 react launchers / 65 playgrounds**. The one gate
that *did* discover an empty set — vitest collecting a config file with 0 tests — is fixed in §3.6.1.
No gate in this slice passed on an empty set.

---

## 4. fixture-sweep fixture — already green, verified by running it

P1 finding 2 said `…/🗣️dsl/🧹️fixture-sweep/🧫️fixtures/🔣️.json` was "wholesale stale": a `dependencies`
array of 29 entries of which 26 aliases no longer existed, plus stale `moduleSha256`/`retainedSha256`.

At HEAD the fixture has no `dependencies` key at all — it carries `workspaceDependencies` (53) +
`pathDependencies` (3), which is the shape `testFixtureSweepExtraction`
(`…/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts:117`) actually compares. I ran the oracle rather than
re-deriving it by eye:

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust
$ bun ./📜️script.ts source-check                                                  # exit 0
[dsl-fixture-sweep] extraction oracle: 9 mounted M5 modules, 7 preserved laws, 53 workspace edges,
3 path edges, 18 hostile cases; 189 example directories, 288 asset-first .semio files,
discovery SHA-256 9b74b8fa88e6de806c034eb2c81df58d02e62c0ae0e33d50bddcf9d523085617;
native law counts pending execution
```

That single call is the whole of `testFixtureSweepExtraction`: `testFixtureSweepReportContract`, the
Ajv `const`-schema exact match, the independent `crypto.subtle.digest` recomputation of
`moduleSha256`, all 18 hostile mutations, and the Bun-glob-vs-independent-walk cross-check that
produces the 189/288 counts. **No edit was needed.** Whoever regenerated it between P1 and now closed
the finding. **Not** verified: the `test` target's 7 native Rust laws behind `runExactCargoLaws`, and
therefore the report contract's native-receipt half (cargo budget — no cargo was run by this slice).

---

## 5. The other cheap gates

### 5.1 `verify layering` — exit 1, 213 files past baseline

```
[verify layering] 235 authored repo-wide/framework file(s) reference an implementation area (5972 reference(s)).
error: 213 file(s) grew past their baseline
```

The shrink-only ratchet is `🧅️layering.json` (45 entries, 277 refs, last written 2026-09-12, commit
`8add1df147`). The top breaches are not code but **authored data manifests that inherently enumerate
implementation paths**:

| refs | baseline | file |
|---|---|---|
| 2896 | 0 | `…/📚️library/🔣️schema-catalog.json` |
| 448 | 0 | `…/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` |
| 293 | 0 | `Cargo.toml` (root workspace members) |
| 280 | 138 | `📜️script.ts` |
| 252 | 0 | `…/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` |
| 128 | 0 | `…/🧑‍💻dev/📤️distribution/🧶️bundles/🖥️runtime-Dc0wwVhF.js` (a build bundle) |
| 117 | 7 | `…/📚️library/🔣️taxonomy.json` |
| 87 | 0 | `package.json` |

Diagnosis checked against git, not guessed: neither the detector (`layeringReferences`,
`…/📚️library/🟦️.ts:707-764`) nor any of its inputs changed since the baseline was written —
`LAYERING_SCANNED_EXTENSIONS` (which has always included `.json`/`.toml`), `LAYERING_SKIPPED_DIRS`,
`areaLayers`, `repoWideContractIds` and `layeringGeneratedContractIds` are byte-identical at
`8add1df147` and at HEAD. Yet the 09-12 baseline contains **no** entry for `Cargo.toml` or
`package.json`, which cannot have had zero references at any point. So the committed baseline was never
a faithful `write-baseline` snapshot, and this gate has been red since at least 2026-09-12.

**Not fixed, deliberately.** Both candidate fixes are policy calls above this slice: either
`layeringGeneratedContractIds` / `LAYERING_SKIPPED_DIRS` grow to exclude generated bundles and
path-enumerating manifests, or 213 files move. `write-baseline` is forbidden by the file's own header
("Regenerate deliberately … AFTER a migration, never to make a failure go away") and I did not run it.

### 5.2 `verify dependencies literal-external` — exit 1

```
target=0, current=236, oracle-conflicts=15, toolchain-owner-conflicts=2, toolchain-failures=0
rust 85 third-party (64 production-reachable) · js 118 (33) · python 34 (0) · go 24 all first-party
```

The 15 oracle-conflicts are third-party crates used as validation oracles in more than one place
(`serde_json` alone in ~130 manifests, plus `quick-xml`, `tiff`, `tobj`, `riff`, `ruststep`, `zip`, …).
The 2 toolchain-owner-conflicts are `nx@23.2.0` and `@nx/js@23.2.0` declared by
`…/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/package.json`, which is not in
`DEPENDENCY_AUTHORIZED_TOOLCHAIN_MANIFESTS` (`…/🕸️dependencies/⚖️truth/🟦️.ts:112`). **Not fixed** — the
repo-wide dependency-truth backlog; authorizing that one manifest changes nothing while
`literal-external` is 236.

### 5.3 `verify dependencies` (freeze) — exit 1, 7 new dependencies

Baseline `🔒️dependencies.json` = 230 third-party (commit `958c5ba76a`); current 237. The 7 new ones are
all `repository-tooling`, all declared by the root `package.json`, all added by peers during this
fleet's run: `@types/bun`, `@types/markdown-it`, `@types/micromatch`, `@types/picomatch`, `graphql`,
`micromatch`, `picomatch`. **Not approved into the baseline by me** — `write-baseline` here is a
deliberate approval of somebody else's additions, and the gate stays red on §5.2 regardless. Flagged
for the owners of those additions.

### 5.4 `verify package-purity` — exit 1, 434 breaches

247 `taxonomy/package-purity` (a directory under `📦️packages/…` that is not an allowed semantic package
directory, e.g. `🧰️framework/📦️packages/🟦️typescript/🌿️ambient`), 177 `taxonomy/package-body-ownership`
(authored implementation inside a package boundary), 10 `taxonomy/package-body-unresolved`.
Pre-existing repo-wide backlog; **not fixed**, recorded.

### 5.5 `check-generated` oscillates under live peer edits

Green immediately after my `generate`; stale again (`.vscode/launch.json`) by the end of the slice,
while `…/📇️registry/🚀️launch/🏷️name-prefix/🟦️.ts` sits unstaged in `git status` — a peer is rewriting
the launch generator right now. I did **not** regenerate a second time: that would overwrite whatever
they are mid-way through. Expect this gate to flip until their edit lands.

---

## 6. Honest gaps

1. **28 dead `frozenCoordinateEvidenceContracts`** in `…/📚️library/🔣️taxonomy.json` point at
   ticket-generated files deleted when their tickets closed (27 under
   `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`, one under `26/08/23/END-TO-END-TESTING-REFACTOR`, one
   under `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION`). Harmless today, dead root data. Removing them
   also touches `…/📚️library/🧫️fixtures/❄️frozen-coordinate-evidence/🔣️.json` and the four suites
   `🧪️tests/{❄️frozen-markdown-coordinates, 🕰️historical-json-source-encoding,
   🏺️historical-package-owner-identity, ☂️frozen-coordinate-wildcard-coverage}` — evidence-contract
   owner's call.
2. **`verify taxonomy report` never finished** (≈1 h 50 m wall / 33 min CPU under a swapping fleet), so
   its finding count is the one unknown in §1. It is provably unblocked, not hung. Run it on an idle
   host, or give it the incremental progress reporter that `verify taxonomy implementation` already has
   (`📜️script.ts:8179`).
3. **`plugin-registry check` is at 1836, not zero, and cannot honestly reach zero from here.** 615
   findings need ≈3 100 handcrafted schema files; 892 need per-crate `#[path]` mounts. Campaign order
   from §3.3: `🗄️stdio` (1072) → `📕️norm` (201) → `🏗️fem` (57). The `🎮️commands/🦀️.rs` family (33)
   needs a taxonomy-owner decision first: either the plugin-root rule adopts the mode-lane convention
   ("an empty lane carrying only `📌️.empty.md` is valid"), which is a coordinated change to
   `🗿️taxonomy-validation/🟦️.ts:241`, the SQLite oracle's `required` table at `:713` and
   `…/📇️registry/🧫️fixtures/🌳️plugin-root-ownership/🔣️.json`; or 33 plugins genuinely gain plugin-scope
   commands.
4. **`🎪️demonstrator` plugin-root contract**: move
   `✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs` → `✏️s/🔌️plugins/🎪️demonstrator/🦀️.rs`,
   retargeting `📦️packages/🦀️rust/🦀️.rs:42`'s `#[path]`, the
   `plugin_exports!(manifest::plugin, manifest::DemonstratorApps)` call at `:49` and the two test mounts
   under `🪪️manifest/🎪️demonstrator/🧪️tests/`. Left to slice B3d, which owns that plugin right now.
5. **`📐️cad` `🪟️windows/🎚️config`**: move to `🎭️modes/✏️edit/🎚️config/` (today only `📌️.empty.md`),
   changing the Rust module path `…::modes::edit::windows::config` → `…::modes::edit::config` and the
   four sibling windows' imports (`…/🪟️windows/🎚️config/🦀️.rs:3-4`). Left to slice B3d.
6. **23 of 65 playground variants have no conformant react dev launcher** and 45 of 65 fail the current
   launcher law (§3.6, measured by `🐍️v1-launcher-match-probe.ts`). This contradicts AGENTS.md ("All
   devs are using `launch.json` … You MUST register all executable commands there") and is bigger than
   the single assertion failure suggests. Needs a `🚀️launch/🟦️.ts` + `.vscode/🧩️launch.seed.jsonc`
   normalisation, coordinated with whoever is editing `🚀️launch/🏷️name-prefix/🟦️.ts`.
7. **`verify layering`, `verify package-purity`, `verify dependencies*` are long-standing red
   backlogs**, none caused by this ticket, none closable inside one slice; each is recorded above with
   its exact number, capture file and the `file:line` of the rule that produces it.
8. **Runtime vs. tests.** Every "After" number in §1 was produced by running the command here and is in
   `🗑️generated/`. No cargo build/check/test was run by this slice (host at its concurrency limit; no
   finding needed a compile) — except indirectly: the mounts oracle shells out to `rustc` once per
   fixture case, which is how §3.5's fix is compiler-checked.

---

## 7. Files changed

| File:line | Change |
|---|---|
| `…/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:195-205, :229` | new `escapesOwnerRoot` helper + the guard that stops judging `#[path]` mounts leaving the owner root (−17 false findings) |
| `…/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:648-655` | mounts oracle materialises `neighbourFiles` beside the owner root and keeps `sourceFiles` root-bounded, mirroring `walkPluginTree` |
| `…/🔌️plugin/📇️registry/🧬️schema/🔣️.json:145-159` | `RustTaxonomyMountsV1`: `cases` 9 → 10, optional `neighbourFiles` |
| `…/🔌️plugin/📇️registry/🧫️fixtures/🕸️rust-taxonomy-mounts/🔣️.json` | new compiler-checked case `neighbour-owned-path-target` |
| `…/🔌️plugin/📇️registry/🧪️tests/🎚️config/🟦️.ts:19` | vitest no longer collects its own config as a test file |
| `…/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts:116` | profile law points at `📽️projection/🟦️.ts`, the real fourth declaration site (the old path never had the constant) |
| `…/🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json` | `developmentPackageOverrides` re-derived from root `Cargo.toml` (2 dead crate names → the 10 documented overrides); 1 line, style preserved |
| `.vscode/launch.json` (+11) and `…/📇️registry/🤖️generated/*` | regenerated byte-exactly by the registry's own `generate` — required before `check` reaches its taxonomy audit at all |
| `.🧬semio/…/📓️v1-verification-gates.md` | this report |
| `.🧬semio/…/🐍️v1-launcher-match-probe.ts` | new probe backing §3.6's 74/65/23/45 numbers |

No schema, fixture or taxonomy file was weakened: every edit either fixes a rule that was factually
wrong about the tree (mounts guard, profile-law path) or re-derives a mirror from its declared
authority (the Cargo profile fixture). No baseline was rewritten to silence a failure.

Captures (delete with the ticket): `🗑️generated/v1-plugin-registry-check.txt`,
`v1-plugin-registry-check-after.txt`, `v1-registry-generate.txt`, `v1-registry-oracles.txt`,
`v1-rust-mounts-oracle.txt`, `v1-registry-test.txt`, `v1-registry-test-after.txt`,
`v1-taxonomy-before.txt`, `v1-fixture-sweep-before.txt`, `v1-layering.txt`,
`v1-deps-literal-external.txt`, `v1-deps-freeze.txt`, `v1-package-purity.txt`.

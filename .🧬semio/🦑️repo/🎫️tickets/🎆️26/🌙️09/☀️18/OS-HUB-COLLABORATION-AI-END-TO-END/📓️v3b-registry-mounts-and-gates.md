# V3b — Unreachable Rust mounts, directory lanes, interactivity gates, frozen-contract retirement, red-gate triage

Slice V3b of ticket 26/09/18 OS-HUB-COLLABORATION-AI-END-TO-END. Inputs: `📓️v1-verification-gates.md`,
`📓️v2-launchers-registry-taxonomy-perf.md`. Sibling V3a owns the 615 schema lanes; everything else in
`plugin-registry check` (1836) is mine.

Captures: `🗑️generated/v3b-*.txt`. Status: **IN PROGRESS** — sections fill as they finish.

---

## 0a. Session 4 (2026-09-19 23:20 →) — re-measured baseline

Sessions 1–3 of this slice died mid-way. Everything §1–§5 below claims to have landed was verified
present in the tree at the start of session 4 (auto-commit had taken it): `RustModuleIncludeFact` +
`frozenCoordinateEvidenceSeal` + the `🗑️generated` skip entry in
`…/📚️library/🔍️discovery/🟦️.ts`, `isRepositoryTestCaseAdapter`/`includeClosure`/`TEST_FEATURE_FILENAME`
in `…/📇️registry/🗿️taxonomy-validation/🟦️.ts` (the file has since been consolidated, so the line
numbers quoted in §1 have moved), and the 75 `📸️set-snapshot` facet mounts (spot-checked at
`🗿️artifacts/🎒️zip/…/🧬️mutations/📸️set-snapshot/🦀️.rs:9-12`).

Re-measured at the start of session 4 (`🗑️generated/v3b-s4-check-1.txt`, exit 1, 35 min under a
load-150 fleet):

| family | V3b session 3 | session 4 start | owner |
|---|---|---|---|
| `plugin-registry check` total | 1363 | **619** | — (V3a's schema-lane work took ~730) |
| `unreachable-from-cargo-manifest` | 415 | **333** | V3b |
| subset directory lanes (`📚️examples/` 66, `🚪️io/` 48, `🧬️schema/` 13, `🏅️standards/` 4, misc 2) | 133 | **133** | V3b |
| mode `is missing required child` | 61 | **61** | V3b |
| example-content lanes (`🦀️.rs`/`🟦️.ts` 46, `🧪️tests/` 22, `🖼️assets/` 10) | — | **78** | V3b |
| taxonomy-owner decisions (plugin-root leaves, `🔌️plugin` nesting, window child, surface leaves) | 45 | **17** | V3b (11 of the 17 are `📕️norm` schema lanes → V3a) |

**A faster re-measure exists now.** `CheckScript` reaches the taxonomy rows only after rendering the
whole catalog (35 min under fleet load); the rows themselves come from `validateTaxonomyTree` over
`findNewContractPluginRoots`. `🐍️v3b-taxonomy-census.ts` (this folder) calls that pair directly — the
gate's own code — and prints the same rows with a family histogram in ~6 min
(`🗑️generated/v3b-s4-census-before.txt`: `plugins=34 findings=632`).

---

## 0. Headline

| # | Item | Before | After |
|---|---|---|---|
| — | `plugin-registry check` total | **1836** | **1363** (`🗑️generated/v3b-check-1.txt`, exit 1) |
| 1 | `unreachable-from-cargo-manifest` | **892** | **415** — 477 were two gate defects, both now compiler-checked (§1) |
| 1 | `rust-taxonomy-mounts-check` oracle cases | 10 | **12**, `compiler=12`, exit 0 |
| 1 | unmounted `📸️set-snapshot` behaviour facets | 75 across 28 artifacts | **0** — mounted; all **17** affected crates `cargo check` exit 0 |
| 2 | directory lanes (133 subset + 61 mode) | 194 | **133** — the 61 mode lanes landed (§S5.1); the 133 are migration backlog (§S5.2) |
| 2 | taxonomy-owner decisions | 45 | **13**, each decided in writing (§S5.5) |
| 3 | `verify interactivity apps` | **776 failures** (the "25" in status.md was `selfTests=25`) | **44**, all one real family (§3) |
| 3 | …its discovered surface | descriptors 3, apps 14, actions 487 | descriptors **45**, apps **127**, actions **6254** |
| 3 | …its self-tests | 25 | **29** |
| 4 | dead `frozenCoordinateEvidenceContracts` | 28 | **0 unrecorded** — all 28 retired with ticket + reason, seal digest unchanged (§4) |
| 4 | `🕰️historical-json-source-encoding` suite | 17 pass / 3 fail | **21 pass / 1 fail**; the 1 is a seal that never matched its own birth commit |
| 5 | `verify layering` / deps literal-external / deps freeze / package-purity | 213 / 236 / +7 / 434 | **218 / 236 / +7 / 430** — triaged in §5, one gate defect fixed (−595) |

---

## 1. 892 unreachable Rust mounts → 415

### 1.1 Census method

`🐍️v3b-mount-census.ts` (this folder) classifies every finding in V2's capture by the evidence that
decides which compiler owns the leaf, not by its name. `🐍️v3b-adapter-crosscheck.ts` re-derives one of
those classes from the test platform's own discovery so the verdict does not rest on a regex.

| class | count | meaning |
|---|---|---|
| `repo-test-adapter` | **236** | leaf sits in `<owner>/🧪️tests/<case>/` beside `🥒️.feature` |
| `include-mounted` | **242** | leaf is reached by `include!("…")` from an owned source |
| `mounted-elsewhere` | 27 | a `#[path]` names it — but from a parent that is itself unmounted |
| `unmounted` | 387 | nothing reaches it — real finding |

Non-zero discovery asserted at every step: the census read 892 findings over 34 plugins, and
`discoverTestCases` independently found **461 cases / 344 Rust adapters** repo-wide.

### 1.2 Gate defect A — 236 repo-test-case adapters are compiled by a generated host, not by Cargo

`validatePluginTaxonomy`'s `walkPluginTree` puts **every** `🦀️.rs` in the plugin tree into
`componentFiles` and then asks whether a plugin Cargo manifest reaches it. For a repository
test-platform case that question is the wrong one: `discoverTestCases`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts:545-588`) recognises a case as a directory under a
`🧪️tests` owner dir carrying `🥒️.feature`, and the adapter beside it is compiled by a **generated
cache-local crate** that mounts it by absolute `#[path]` and links `semio-repo-test-host`
(`…/🧪️test/🖥️host/🏗️materialization/🟦️.ts:85-136`). No plugin manifest may own it — by design, because
the host crate must be able to link the *oracle* role without linking the subject
(`🖥️host/🏗️materialization/🟦️.ts:114-124`). `semio-repo-test-host` declares no dependencies and is
excluded from the repository workspace on purpose (`…/🧪️test/📦️packages/🦀️rust/Cargo.toml:1-3`).

Fix: `🗿️taxonomy-validation/🟦️.ts:590-597` (new `isRepositoryTestCaseAdapter`) + `:154-156` (new
`TEST_FEATURE_FILENAME`, resolved from `TAXONOMY.testFeatureFileKindId`, not hard-coded) + the guard at
`:616`. Cross-check: **236 claimed, 236 confirmed by `discoverTestCases`, 0 misses**
(`🗑️generated/v3b-adapter-crosscheck.txt`).

### 1.3 Gate defect B — 242 leaves are reached by `include!`, which the module graph never modelled

`inspectRustModuleGraphFacts` extracted only `mod` and `use` items, so a file textually expanded by
`include!("…")` — real Rust compilation, and the shape `🗄️stdio` uses for its
`🧪️tests/🔬️derived-{analysis,composition,construction}-unit/🦀️.rs` leaves (e.g.
`🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🚪️io/🦀️.rs:82`) — was invisible to the graph and
reported as never compiling.

| file:line | change |
|---|---|
| `…/📚️library/🔍️discovery/🟦️.ts:6255-6262` | new `RustModuleIncludeFact` |
| `…/📚️library/🔍️discovery/🟦️.ts:8385-8388, :8415-8421, :8435` | `inspectRustModuleGraphFacts` now also returns `includes`, extracted from the **token stream** (so a commented-out or stringified `include!` cannot decoy), additively — no existing consumer changes behaviour |
| `…/📇️registry/🗿️taxonomy-validation/🟦️.ts:600-626` | new `includeClosure`: the transitive `include!` closure of every manifest-owned source, resolved the way `rustc` resolves it (against the directory of the file the invocation sits in) and bounded to files inside the owner root |
| `…/📇️registry/🗿️taxonomy-validation/🟦️.ts:646, :653` | a leaf inside that closure is no longer reported unreachable |

### 1.4 Both fixes are compiler-checked, not self-asserted

`RustTaxonomyMountsCheckScript` compiles every fixture case with a real `rustc` and compares the
registry's verdict against the compiler's own dep-info. Two cases added
(`…/📇️registry/🧫️fixtures/🕸️rust-taxonomy-mounts/🔣️.json`, schema `cases` 10 → 12 at
`…/📇️registry/🧬️schema/🔣️.json:145-146`):

* `included-source-leaf` — an `include!`-only leaf; `rustcSuccess: true`, `expectedUnmounted: []`.
* `included-leaf-chain-and-unincluded-sibling` — a two-hop `include!` chain **plus** an orphan sibling
  in the same directory; `expectedUnmounted` is exactly the orphan. This is the discrimination test:
  it fails if the closure absolves too much.

```
$ bun ./📜️script.ts rust-taxonomy-mounts-check
registry-rust-mounts-oracle cases=12 ajv=1 compiler=12          # exit 0
$ bun ./📜️script.ts plugin-root-ownership-check     → cases=7 ajv=1 sqlite=7        exit 0
$ bun ./📜️script.ts native-catalog-selection-check  → cases=23 positive=4 denied=19 exit 0
```

### 1.5 The remaining 415 are real — verdict per family

Re-censused after the fix (`🗑️generated/v3b-mount-census-2.txt`): 387 unmounted + 27
mounted-from-an-unmounted-parent + 1 file a peer deleted after the capture. The 27 are **not** a third
defect: e.g. `🧿️semio`'s `📚️examples/🧊️solid/🧪️tests/🔬️unit/🦀️.rs` is mounted from
`📚️examples/🧊️solid/🦀️.rs:19`, and that example leaf is itself unmounted — the gate is right.

By plugin: `🗄️stdio` 259, `📕️norm` 53, `🌊️flow` 11, `🏛️architect` 5, then ≤4 each across 22 plugins.

### 1.6 Largest real family fixed: 75 unmounted `📸️set-snapshot` behaviour facets (28 artifacts)

Decision **mount, not delete** — and the evidence went against my first reading. The facets look like
dead delegating shims (`🔺️diff/🦀️.rs` just calls `diff_set_snapshot`; the mounted kind leaf
`📸️set-snapshot/🦀️.rs` delegates to `agg_diff`/`agg_inverse` instead), and `git log --follow` shows the
facets predate the kind leaf (facet 2026-08-20 `95b8688ee2`, kind leaf 2026-09-01 `67fb4216b2`), which
reads like a leftover. But `mutationDirectLeafInlinedBehaviorFacets`
(`…/📚️library/🔍️discovery/🟦️.ts:3877-3904`) states the opposite contract: a mutation's direct leaf
re-inlining `diff`/`inverse` instead of placing them in their own facet directory is "the exact
regression the SEMANTIC-MUTATIONS-OVERHAUL contract forbids". The facet directory is the intended home;
**not mounting it is the defect**. 9713 such facet dirs exist repo-wide and are routinely mounted (e.g.
`🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs:920-922`), and one `🗄️stdio` artifact already mounts exactly this
family — `🗿️artifacts/🖊️dwg/🦀️.rs:308-316`.

Measured correlation: of the 29 artifacts carrying `📸️set-snapshot` facets, **28 have the direct leaf
and mount none of the facets; exactly 1 (`🖊️dwg`) has no direct leaf and mounts all three.**

`🐍️v3b-mount-set-snapshot-facets.ts` (this folder, idempotent, re-reads each leaf immediately before
writing) added the mounts to all 28 direct leaves — `diff` + `inverse` on all 28, `mutation` on 20.
Static shadowing check first: none of the 28 leaves uses a `diff::`/`inverse::`/`mutation::` path that
the new modules could shadow.

**Compile-verified: all 17 affected crates, `cargo check -p <crate> --features component-app-assembly`,
exit 0** (`🗑️generated/v3b-cargo-setsnapshot.txt`, `v3b-cargo-zip-retry.txt`, `v3b-cargo-dwg-retry.txt`,
run 11:02–11:06 on 2026-09-19):

```
avi binary deflate docx dxf ifc md obj ply pptx semio step tsv wav xlsx zip   exit=0
dwg                                                                          exit=0 (retry)
```

Two peer-caused interruptions are worth recording because they look like this slice's breakage and are
not: at 04:14 every crate failed with 35 × `E0609: no field 'faulted' on type
'ActiveArtifactStoreReplacement<…>'` inside **`semio-framework-plugin`**, and at 11:03 `dwg` alone failed
with `E0609: no field 'artifact_id' on type 'ColdDocumentPairFrontier'` inside **`semio-framework`**
(`🧰️framework/🔨️modules/🎠️kernel/📥️cold-pair/🦀️.rs:33-34`). Both are peers' live edits to shared
dependencies; both cleared on retry with the mounts unchanged.

---

## 1b. Session 4 — the remaining 333 unreachable mounts → **0**

```
🗑️generated/v3b-s4-census-before.txt      unreachable-from-cargo-manifest = 333   (findings 632)
🗑️generated/v3b-s4-census-after-mounts.txt                                =   1   (findings 286)
🗑️generated/v3b-s4-census-mounts-zero.txt                                 =   0   (findings 285)
```

### 1b.1 327 leaves were missing their mount, and the placement is mechanical

Every one of the 333 is a file on disk that `rustc` never compiles. The taxonomy's own answer — used by
every mounted sibling — is that the **nearest ancestor component leaf** mounts the child by a literal
`#[path]` relative to itself (`…/🧬️mutations/🎛️sampler/🚚️move/🦀️.rs:77` mounts
`🧪️tests/🔬️direct-leaf/🦀️.rs` exactly that way). `🐍️v3b-mount-repair.ts` (this folder) reproduces that
placement:

* host = closest ancestor directory whose `🦀️.rs` is itself reachable (an unreachable ancestor would
  only move the problem up a level), falling back to the plugin root leaf;
* a leaf under a `🧪️tests` segment mounts as `#[cfg(test)] mod`, everything else as `pub mod`;
* the module identifier is the shortest path suffix at which **all** of that host's new mounts differ
  from each other and from the modules the file already declares — so `📤️export/🧵️serializers/…` and
  its `📥️import` twin get symmetric names instead of `artifacts` / `deserializers_artifacts`;
* every host is re-read immediately before it is written (peers edit these crates concurrently) and a
  leaf already carrying its `#[path]` in the host is skipped, so re-running changes nothing.

Landed: **327 mounts across 210 host files in 23 plugins** (`🗑️generated/v3b-s4-mount-apply.txt`;
`🗄️stdio` 206, `📕️norm` 53, `🌊️flow` 11, then ≤5 each). By family: ~130 mutation test-case leaves
(`🧬️mutations/<entity>/<verb>/🧪️tests/<case>/🦀️.rs` — substantial authored oracles that reference
`crate::schema::mutations::…` and were never compiled), 55 `🧪️tests/🧩️example/🦀️.rs`, 34
`📚️examples/🎬️demo-session/🦀️.rs` + ~20 `📚️examples/<slug>/🦀️.rs`, 31 `🧪️tests/🔬️unit/🦀️.rs`, and the
plugin-root facet leaves (`🎟️capabilities`, `🔧️setup`, `🪪️manifest`/`📜️manifest`).

### 1b.2 Two `🧊️gltf` barrel leaves — mounted by hand, against the count

`🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🦀️.rs` and `…/🪆️subsets/♾️any/🦀️.rs` carried docstrings asserting
they are "deliberately empty … not part of any `mod` tree" because the artifact root builds those two
coordinates as inline `#[path = "."]` barrels. **Measured, and the docstring was wrong about the
siblings:** 31 standard-level and 31 subset-level leaves exist under `✏️s/🔌️plugins`, and **29 of each
are mounted** (e.g. `✒️writer/🗿️artifacts/✒️writer/🦀️.rs:235,242`); gltf was the only outlier at both
levels. So they were mounted into their own barrels as `mod component; pub use component::*;`, the
shape the other 29 use, and both docstrings were rewritten to say so.

### 1b.3 Gate defect C — an escaped-quote char literal blinds the whole module graph of a file

The last remaining finding, `🗄️stdio`'s
`🗿️artifacts/📰️xml/…/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`, was mounted correctly at
`📸️snapshot/🦀️.rs:773` and still reported unreachable. `🐍️v3b-mount-probe.ts` (this folder) rebuilds
the gate's own graph for one plugin root: the host file had **1 context and 0 modules** — the parser
saw no `mod` at all in it. A suffix bisect put the break at `📸️snapshot/🦀️.rs:294`:

```rust
out.push('\"');
```

`rustTokens` (`…/📚️library/🔍️discovery/🟦️.ts:6495` before the fix) read a character literal as exactly
`'x'` — three characters. `'\"'` is four, so the `'` fell through to punctuation, the `\` fell
through to punctuation, and the `"` **opened a string** that ran to the next `"` in the file. Every
token from there to EOF was mis-lexed and the file's module graph vanished. `'\n'`, `'\\'` and `'\''`
mis-lex too but harmlessly; only the `"` escape swallows code, which is why this went unseen.

Fix: `…/📚️library/🔍️discovery/🟦️.ts:6495-6510` reads the literal through its escape sequence
(`\n`, `\\`, `\'`, `\"`, `\xNN`, `\u{…}`) and leaves a `'` that does not close that way as a
lifetime/label tick. The old `'x'` path is subsumed exactly.

**Compiler-checked, not self-asserted.** `RustTaxonomyMountsCheckScript` compiles each fixture case
with a real `rustc` and compares the registry's verdict against the compiler's dep-info. New case
`escaped-quote-char-literal-before-a-mount`
(`…/📇️registry/🧫️fixtures/🕸️rust-taxonomy-mounts/🔣️.json`, schema `cases` 12 → 13 at
`…/📇️registry/🧬️schema/🔣️.json:145-146`): a leaf mounted **after** `out.push('\"')` and
`out.push(b'\"' as char)`, with `rustcSuccess: true` and `expectedUnmounted: []`. Before the fix the
registry calls that leaf unmounted while rustc compiles it, so the case fails.

```
$ bun ./📜️script.ts rust-taxonomy-mounts-check
registry-rust-mounts-oracle cases=13 ajv=1 compiler=13      # exit 0 (🗑️generated/v3b-s4-mounts-oracle.txt)
```

Blast radius outside this slice: only **3** authored `.rs` files in the repo carry `'\"'` —
`✏️s/🔌️plugins/🗄️stdio/…/📸️snapshot/🦀️.rs`, `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️.rs` and
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` — but `rustTokens` feeds every Rust
analysis in the repo product, so those two framework files were silently invisible to all of them.

### 1b.4 Compile proof — blocked by a fleet-wide cargo deadlock (honest gap)

The mounts are `#[path]`/`mod` lines only, and `#[cfg(test)]` leaves need `--all-targets` to be
type-checked, so the proof wanted is `cargo check -p <crate> --all-targets` over the **47 crates**
that own the 210 edited files (`🐍️v3b-touched-crates.ts` → `🗑️generated/v3b-s4-crates.txt`;
`semio-s-artifact-stdio-gltf` alone owns 121). That proof is **not yet obtained**: see §6.1.

---

## 2. 194 directory lanes + 45 taxonomy-owner decisions

**Answered in session 5** — the three sections below carry the measured verdicts:

* the **61 mode lanes** landed to **0** (§S5.1) and turned up a marker-filename drift on the way;
* the **133 subset lanes** and **78 example-content lanes** are *not* a gate defect but the unfinished
  tail of the `subset_conformance_roundtrips` migration, whose own plan forbids the shortcut that
  would have closed them — left red on purpose, with the per-shape census (§S5.2);
* the taxonomy-owner decisions, down from 45 to **13**, are each decided in writing in §S5.5, with the
  evidence and the owner named; 4 more turned out to be one gate defect (§S5.4).

---

## 3. Interactivity gates — 776 → 44

**First correction: the verb was never "25 findings".** `25` in the wave table is `selfTests=25`, the
self-test count printed on the same line. The verb's own failure count was **776**
(`🗑️generated/v3b-interactivity-before.txt`, exit 1), and the headline numbers beside it were fiction:
`descriptors=3` for a tree with 34 plugins.

### 3.1 The dominant defect: 20 368 files judged as plugin descriptors

`interactivityAllAppDiscovery` selected descriptors by **filename alone** —
`name === "🔣️.json"` under `✏️s/🔌️plugins` (`📜️script.ts:9255`). `🔣️.json` is the taxonomy's generic
JSON component filename, so that matches **20 368** files: every schema, fixture, mutation descriptor
and oracle table in the plugin tree. Only **46** are descriptors (32 plugin roots + 14
`🧩️extensions/<id>`). The walk was then truncated to `INTERACTIVITY_ALL_APP_DESCRIPTOR_CAPACITY = 256`
in sort order, so the real descriptors were mostly never read — hence `descriptors=3` — while ~750 of
the 776 failures were `descriptorVersion must be 1` / `manifest is missing` / `role must be plugin`
shouted at files that were never descriptors.

Fix: descriptor selection is now a **coordinate**, not a name
(`📜️script.ts:9264-9276`, new `interactivityAllAppIsDescriptorCoordinate`): plugin root, or
`🧩️extensions/<id>` excluding the taxonomy ownership dirs (`📜️script.ts:9001-9005`, new
`INTERACTIVITY_ALL_APP_OWNERSHIP_DIRS`) — `🌊️flow/🧩️extensions/🧫️fixtures/🔣️.json` is the flow
extensions' shared fixture table, not a fourteenth extension.

### 3.2 Restored: the six `4_gate` rows and their enforcement

Commit `6f33e313da` emptied `INTERACTIVITY_ALL_APP_REQUIRED_GATES`, deleted the enforcement loop and
deleted the six `⚖️gate…` rows from both launch files. Restored at `📜️script.ts:9013-9036` (list) and
`:9243-9246` (loop), plus the six seed rows via `🐍️v3b-restore-gate-rows.ts` (this folder, idempotent).

**Why it was emptied, most likely:** the deleted list named the six gates as
`bun ./📜️script.ts verify …`, while the six **seed rows at `6f33e313da^` carried
`bun nx run workspace:verify -- …`**. Law and data could never agree, so the list was emptied rather
than reconciled. AGENTS.md makes `📜️script.ts` the implementation and `nx` the entry point, and every
neighbouring `4_gate` row uses the nx form, so the **law was moved to the nx form** and the rows
restored in that shape. `workspace:verify` exists with `forwardAllArgs: true` (`📋️project.json`).

One trap worth recording: the seed has four top-level arrays, and my first insertion anchored on the
last `],` before the `devLaunchers` marker — which is the end of **`inputs`**, not `configurations`.
The file still parsed and `generate` still succeeded, and the gate still reported `found 0`. The script
now anchors on `],\n  "compounds": [` and refuses if that terminator is ambiguous.

### 3.3 Two stale command shapes in the launch-coverage check (V2 §5 gap 4)

`interactivityAllAppLaunchCoverageFailures` expected `bun ./📜️script.ts dev <variant>` and
`bun ./🧰️framework/…/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native <variant>`. **No generated row has ever
carried either.** Measured on `.vscode/launch.json`: all 75 browser rows carry
`bun nx run workspace:dev -- <variant>` (the registry's `playgroundDevCommand`) and all 46
`…🧊️wgpu🖥️native` rows carry `bun nx run @semio-tech/framework-renderer-wgpu:native -- <variant>`. So
`completeVariants` was always empty and `launchCoveredApps` was structurally 0. Fixed at
`📜️script.ts:9006-9009` (two named command templates, each naming its authority) and `:9212-9213`.

### 3.4 Two wrong laws about extension descriptors

* **`role must be plugin` applied to extensions too** (`📜️script.ts:9087`), but every extension
  descriptor legitimately declares `role: "extension"` — 14 failures. Now `role` is checked against the
  coordinate: `plugin` at a plugin root, `extension` under `🧩️extensions/`.
* **The extension's plugin id was read from `EXTENSION_ID`** — which is the extension's short *topic*
  id (`"bim"`, `"draw"`), passed to `flow_extension_topic_contribution`. The id it actually registers
  is the first argument of `ExtensionBundle::new(…)`: `"flow-extension-bim"`, `"flow-extension-draw"`
  (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs:662,674`), which is exactly the mapping
  `🧩️extensions/🧫️fixtures/🔣️.json` carries. Reading the wrong one produced 9 ×
  `extension source does not construct its declared EXTENSION_ID` **and** made the `🖍️draw` extension
  collide with the real `🖍️draw` plugin under this verb's own ambiguity law. New
  `interactivityAllAppExtensionBundleId` (`📜️script.ts:9063-9076`) reads the bundle argument, resolving
  a constant through its own `const … : &str` declaration.

### 3.5 Self-tests: 25 → 29, all discriminating

`…/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts:14-24`. The three coverage
fixtures at `:45-47` were updated to the real command shapes — **this is the pre-existing self-test bug
V2 flagged**: the self-test enshrined command strings no real row has, so it passed green while the
same check failed on every real app. Four new assertions: the bundle id resolves through a constant;
the bundle id is read from `ExtensionBundle::new` and **not** from `EXTENSION_ID` (fails if the old
reading returns); a `role: "plugin"` extension descriptor is rejected; a source with no
`ExtensionBundle::new` is rejected.

### 3.6 Result

```
before: descriptors=3  apps=14  actions=487  launchCoveredApps=0  failures=776 selfTests=25
after:  descriptors=45 apps=127 actions=6254 launchCoveredApps=83 failures=44  selfTests=29
```
(`🗑️generated/v3b-interactivity-before.txt`, `v3b-interactivity-after.txt`; `extensions=13`,
`migratedActions=4859`, `missingActions=1395`.)

**The remaining 44 are one real family, not a defect:** apps whose plugin has no playground variant
carrying all three launchers. **30 of 65 variants have no `…🧊️wgpu🖥️native` row at all** — the whole
of `📕️norm` (14), the six `♻️mit-bestand` demonstrator variants, `🔱️trinity🔌️jack`, `🔋️energy`,
`📖️playbook`, two `🔧️procedural🏙️3d` rows and the two `🪐️space👤️N` multi-user rows (those last two are
extra launchers for one variant by design and should be excluded from the coverage law, as V2 already
excluded them from the launcher law at `🧪️tests/🚀️launch/🟦️.ts:57-84`).

Recipe, and why I did not land it: the native row is still **hand-authored in the seed skeleton**,
which is the same drift V2 §1.2 removed for the two browser renderers by making the registry synthesise
whichever renderer's row is missing (`📇️registry/🚀️launch/🟦️.ts:218-228`). The clean fix is to extend
that synthesis to the native renderer and delete the 46 hand-authored rows — a `🚀️launch/🟦️.ts` change
on a file slices Z1/K2/V3a are editing right now. Adding 30 more hand-authored rows instead would
enlarge exactly the surface V2 just shrank.

---

## 4. Frozen-coordinate-evidence retirement affordance — implemented, 28 retired

V2 §4.4 refused to delete the 28 dead contracts and asked for "an explicit, recorded retirement
affordance … so that the deletion is itself evidence". Built and applied.

### 4.1 The design move: retirement is metadata, never evidence

A retirement that forces you to rewrite the seal is not an affordance — it is the silent delete with
extra steps, because the digest that would have caught you editing a frozen `path`/`sha256`/`coordinate`
is the same digest you must recompute. So the seal now digests a **projection**:

| file:line | change |
|---|---|
| `…/📚️library/🔍️discovery/🟦️.ts:797-816` | `FrozenCoordinateEvidenceRetirement` + its closed `reason` vocabulary; `retired?` on `FrozenCoordinateEvidenceContract` |
| `…/📚️library/🔍️discovery/🟦️.ts:824-837` | `validateFrozenCoordinateEvidenceContracts` accepts `retired` under the same exact-field discipline as every other field: only `ticket` + `reason`, a declared reason, and a `YYYY/MM/DD/TICKET-SLUG` ticket |
| `…/📚️library/🔍️discovery/🟦️.ts:886-899` | new `frozenCoordinateEvidenceSeal()` — the projection a seal digests: `path`, `sha256`, `schemaVersion`, `rootKind`, `coordinates`, and **nothing else** |

**Measured, not asserted:** `🐍️v3b-retire-frozen-contracts.ts` computes the seal before and after its
own write and refuses if they differ. `🗑️generated/v3b-retire-frozen.txt`:

```
contracts=41 sealBefore=2c1b71dbba978c453ef43653a4688846f2002d131c29f1f3cc504aa4df9966c0
alive=13 toRetire=28 unretirable=0
retired=28 sealAfter=2c1b71dbba978c453ef43653a4688846f2002d131c29f1f3cc504aa4df9966c0 sealUnchanged=true
```

### 4.2 The 28, retired with their evidence

Each row now carries `"retired": { "ticket": "<id>", "reason": "ticket-close-generated-output-removed" }`
in `…/📚️library/🔣️taxonomy.json`. The ticket is **derived from the frozen path itself**, never guessed:
27 → `2026/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`, 1 → `2026/08/23/END-TO-END-TESTING-REFACTOR`,
1 → `2026/08/17/END-TO-END-TAXONOMY-NORMALIZATION`. The reason is AGENTS.md's own rule catching up with
evidence frozen while the ticket was open. The script refuses to write if any contract is in an
unexplained state (absent but not under a ticket, or marked retired while its document exists).

### 4.3 Three new laws, and what they discriminate

`…/📚️library/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts:78-101` and `:103-116`:

* **retirement is true**: for *every* contract, `retired === undefined` iff the document exists on disk.
  So `retired` can never excuse a live document, and a live row can never hide a deleted one.
* **the ticket is real**: the frozen path must start with the retirement's own ticket folder.
* **the seal cannot be moved by a retirement**: retiring *every* contract leaves the digest identical,
  while flipping one hex digit of one `sha256` changes it. This is the discrimination test — it fails
  if the projection ever starts including bookkeeping.
* the physical-bytes law now branches on retirement: it asserts the document is **genuinely absent** and
  the frozen coordinate record is intact, instead of dying on `ENOENT`.

```
$ bun test ./…/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts
 21 pass  1 fail  163 expect() calls        # was 17 pass / 3 fail at V2, 18 / 2 after V2 §1.4
```

### 4.4 The 1 remaining failure is a seal that was **never** true — deliberately not rewritten

`historical.originalContracts` claims `count: 38` / `canonicalSha256: 50623c9c…`. Measured against git:

| when | contracts (minus the sealed id) | canonical sha256 |
|---|---|---|
| the fixture's **own birth commit** `9b605a4550` (2026-09-09) | **40** | `6477d2ad…` |
| today | **40** | `91289fb6…` |
| what the fixture claims | 38 | `50623c9c…` |

So the seal did not match the tree it was written against — it has **never once passed**, and V2 §4.4's
reading that "a peer ADDED two contracts during this fleet's run" is wrong: the count has been 41 in
every commit back to 2026-09-08. This is the same shape as V1 §5.1's layering baseline ("was never a
faithful `write-baseline` snapshot").

**Not re-sealed, and here is the reason it matters:** between 09-09 and today three contracts were
re-frozen — `readme-current-source-revision-input`, `readme-reviewed-expectation-input`,
`remaining-package-purity-history-v1`, each a `sha256` change on a document that still exists. That is
exactly what a seal exists to surface, and it went unseen because the seal was already false.
Re-deriving the seal now would bake those three re-freezes in as approved. The evidence-contract owner
must decide whether they were authorized; the value to write once they have is
`count: 40`, `canonicalSha256: 91289fb674e6876813d01a86ee333f4f74865d0c40ed3f571b449e727bfb12c5`
(recompute it through `frozenCoordinateEvidenceSeal`, which is now the projection the test uses).

### 4.5 The sibling Markdown table needs no retirement

`frozenMarkdownCoordinateEvidenceContracts`: **36 contracts, 0 missing on disk** — so the disease is
confined to the JSON table and the affordance did not need to be mirrored. That suite is
**34 pass / 2 fail** (`🗑️generated/v3b-frozen-markdown.txt`), both pre-existing and untouched by this
slice: the same stale-count assertion at `❄️frozen-markdown-coordinates/🟦️.ts:93`, and an `ENOENT` at
`:130` on a **test-local** ticket scratch dir (`…/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️frozen-markdown-coordinates/🧾️runs`)
that was deleted at ticket close — a test reading ticket scratch, not a registered contract. A peer also
left a `[DEBUG]` log at `:128` (K1 owns hygiene; V2 flagged a different one).

---

## 5. Red-gate triage — one big gate defect found and fixed

| gate | V1 | V3b | verdict |
|---|---|---|---|
| `verify package-purity` | 434 | **430** (from **1033**) | **595 were a gate defect — fixed** (§5.1) |
| `verify layering` | 213 files | **218** files / 6029 refs | baseline defect + a scoping defect, neither mine to land (§5.2) |
| `verify dependencies literal-external` | 236 | **236** | real repo-wide backlog, unchanged (§5.3) |
| `verify dependencies` (freeze) | +7 | **+7** | someone else's additions awaiting approval (§5.3) |

### 5.1 `package-purity` — 595 of 1033 breaches were gitignored test output

The gate had **grown 434 → 1033 during this fleet's run**. Diffing V1's capture against mine, all 599
new breaches share one prefix: `🌎️hub/📦️packages/🦀️rust/🗑️generated/test-artifacts/…` — leftovers of the
hub test runs AU1/AU3/W3b executed, being judged as authored package content.

Evidence that this is the gate's fault and not the tree's:

* `🗑️generated` is **gitignored** (`.gitignore:22`), so it is untracked by construction and can never
  hold authored content;
* the repo's own discovery **already** declares it opaque — `ignoredPathPatterns`
  (`…/📚️library/🔍️discovery/🟦️.ts:4119`) lists `**/🗑️generated` beside `**/target`, `**/dist`,
  `**/node_modules`;
* another module refuses it by name as "compiler, dependency, or ticket output"
  (`…/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts:165`);
* AGENTS.md mandates it as the tool-output directory.

Yet `DISCOVERY_SKIP_DIRS` (`…/📚️library/🔍️discovery/🟦️.ts:9830`) listed `🤖️generated` (generator
output) and omitted `🗑️generated` (tool output), so `discoverPackages`' boundary walk descended into it.

**Fix: one entry added** at `…/📚️library/🔍️discovery/🟦️.ts:9830`, with the reasoning recorded on
`isDiscoverySkipDirectory` (`:9833-9840`).

```
verify package-purity   1033 → 430    (package-purity 841→244, body-unresolved 16→10, ownership 176 flat)
```

Non-zero discovery asserted after the change: `discoverPackages` still finds **234 packages**
(plugin=37 s-module=100 extension=26 tool=6 hub=1 framework=54 product=10), and the gate still reports
430 findings across framework, os and plugins — it did not go quiet.

The remaining **430 are real** and are V1's pre-existing backlog: 244 `package-purity` (a directory or
file inside a `📦️packages/…` boundary that is not a declared packaging asset, e.g.
`🧰️framework/📦️packages/🟦️typescript/🌿️ambient`), 176 `package-body-ownership` (authored implementation
inside a package boundary), 10 `package-body-unresolved`. Owner: whoever owns Shape V2 package
boundaries; largest single area is `🧰️framework/🛍️products/🦑️repo` (179).

### 5.2 `verify layering` — 218 files, and the baseline was never faithful

Re-measured: **240 files / 6029 references, 218 past baseline** (V1 saw 235/5972/213). Two distinct
defects, neither of which I may land:

1. **The baseline is provably unfaithful** (V1 §5.1, re-confirmed): `🧅️layering.json` records `0` for
   root `Cargo.toml` and has no entry at all for `package.json`, and neither file can ever have had
   zero references to an implementation area — `Cargo.toml` enumerates every workspace member. The
   file's own header forbids regenerating it to make a failure go away, and I did not.
2. **A scoping defect I can name precisely.** The detector's only escape hatch for derived content is a
   text banner in the first 512 bytes (`…/📚️library/🟦️.ts:737-739`) plus
   `layeringGeneratedContractIds`. But `rootContracts` maps those ids to bare **filenames** and
   `isGenerated` compares them against a full relative path (`…/📚️library/🟦️.ts:718`), so the id list
   can only ever exclude a file sitting at the repo root. A nested derived document cannot be excluded
   by id, and a JSON document cannot carry a comment banner — which is why the top four breaches are
   `🔣️schema-catalog.json` (2940 refs; a derived index of every schema scope, declared in the taxonomy
   at `🔣️taxonomy.json:29371` as `schemaExportResolution.catalogPath`), a captured fixture (448), root
   `Cargo.toml` (293) and a purity-authority fixture (252). By extension 102 of the 218 are `.json`
   and 6 `.toml`; 64 sit under `🤖️generated`/`🧶️bundles`/`dist`/`🧫️fixtures`.

Fixing (2) means deciding which contract ids name the derived catalogs and making `isGenerated`
path-aware — a taxonomy-data decision for the layering owner, on a gate outside this slice's
ownership. Recorded with the exact `file:line` rather than guessed at.

### 5.3 `verify dependencies` ×2 — unchanged, and not this slice's to approve

* `literal-external`: **236** literal-external, 15 oracle-conflicts, 2 toolchain-owner-conflicts
  (`nx@23.2.0`, `@nx/js@23.2.0` declared by `…/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/package.json`,
  which is not in `DEPENDENCY_AUTHORIZED_TOOLCHAIN_MANIFESTS`). Repo-wide dependency-truth backlog.
* freeze: baseline 230 (`958c5ba76a`), current 237. The **7 new** are all `repository-tooling` declared
  by the root `package.json`: `@types/bun`, `@types/markdown-it`, `@types/micromatch`,
  `@types/picomatch`, `graphql`, `micromatch`, `picomatch`. `write-baseline` here is a deliberate
  approval of somebody else's additions, so it stays for their owners. Unchanged from V1.

---

## 6. Honest gaps

### 6.1 The whole machine's cargo is deadlocked — no Rust compile proof this session

At 2026-09-20 00:0x the machine had **30 live `cargo` processes and zero `rustc`**, and the shared
build dir had not been written to in three minutes
(`find .🧬semio/🦑️repo/⚡️cache/cargo/build/debug -newermt '-3 minutes'` → 0). `sample` on the holder of
`…/build/debug/.cargo-build-lock` (pid 936, a peer's `cargo test`, launched 23:23) shows its worker
threads in:

```
cargo::core::compiler::prebuild_lock_exclusive → LockManager::lock → flock
```

i.e. it holds the profile lock and dozens of per-unit locks while itself blocking on another unit
lock — the `fine-grain-locking` deadlock recorded for this repo. Every other cargo, mine included
(`cargo check -p semio-s-plugin-writer --all-targets`, pid 52605, queued 19 min with no progress), is
behind it.

I did not kill it: worker rule 15 forbids killing processes I did not start, and every one of these
belongs to a peer slice. **Consequence: none of the 327 mounts is compile-verified in this session.**
The mounts are `#[path]`/`mod` lines whose placement follows the convention already proven by the 13
compiler-checked oracle cases, but that is a structural argument, not a type-check. The exact command
that closes this gap, once the fleet's build dir is unwedged, is one cargo over the 47 owning crates:

```
cargo check --all-targets $(sed -n 's/^-p /-p /p' 🗑️generated/v3b-s4-crates.txt | tail -1)
```

(the `=== -p list ===` line of `🗑️generated/v3b-s4-crates.txt`). Expect real work there: the ~130
mutation test-case leaves have never been compiled, so some of them will have drifted from the APIs
they call. Coordinator: this deadlock blocks **every** Rust slice, not just V3b.

---

## Session 5 (2026-09-20 ~01:30 →)

### S5.0 What session 4 actually left behind — verified by diff before touching anything

Session 4 ended mid-unmount. Verified in the tree at the start of session 5:

* the `🧊️gltf` unmount **completed**: `🐍️v3b-unmount.ts` removed the 120 mount blocks it was given
  (`🗑️generated/v3b-s4-unmount-gltf.txt` is the target list), the whole `🧊️gltf` artifact is back to
  **8 changed files** in `git diff --stat`, and `🗑️generated/v3b-s4-cargo-gltf2.txt` ends
  `Finished dev profile … in 1m 06s` — so `semio-s-artifact-stdio-gltf --all-targets` compiles again.
  No half-applied host file: the only `#[path]` mounts left in `🧊️gltf` are the pre-existing ones.
* the `📕️norm`/`🌊️flow`/rest of the session-4 mounts are still in the tree (207 of the 327).
* `🗑️generated/v3b-s4-cargo-groupA.txt` stops mid-`semio-framework-plugin` — that run was cut, not failed.

Re-measured baseline with the fast census (`🗑️generated/v3b-s5-census-before.txt`, ~90 s):

```
plugins=34 findings=405
  120 unreachable-from-cargo-manifest      ← all 120 are 🧊️gltf mutation test-case leaves (§S5.3)
  133 subset directory lanes (📚️examples 66, 🚪️io 48, 🧬️schema 13, 🏅️standards 4, misc 2)
   61 mode "…" is missing required child   ← landed to 0 (§S5.1)
   78 example-content lanes (🦀️.rs 23, 🟦️.ts 25, 🧪️tests 25, 🖼️assets 13, surface 2)
   13 owner decisions (surface/window/🔌️plugin/plugin-root)
```

### S5.1 61 mode lanes → 0 — landed, and it exposed a marker-filename drift

`validateTaxonomyTree:573-577` states the law in its own comment: *"a mode declares its windows plus
its own 🎚️config / 👥️presence / 🫧️transient lanes; an empty lane is valid (it carries only the tracked
marker), an absent lane is not."* All 61 were `📕️norm`'s 15 viewer `👁️view` modes (4 lanes each, 60)
plus `🪐️space`'s editor `✏️edit` missing `🎮️commands`. The neighbouring `🗒️note` viewer resolves exactly
this situation with four empty markers, so the lane is the declared shape, not a silencer.

`🐍️v3b-mode-lanes.ts` (this folder, idempotent) writes them with the scaffolder's **own** emitter
(`scaffoldEmptyFacetMarkdown`), not a hand-copied body. Its independent count matched the gate's:
**61 missing, 61 created, 16 modes, 305 modes walked**; a second run reports `missing=0`
(`🗑️generated/v3b-s5-mode-lanes-dryrun.txt`, `…-apply.txt`).

**Gate defect D — the scaffolder writes a marker filename no lane on disk uses.**
`🗿️taxonomy-validation/🟦️.ts:175` derived `WINDOW_EMPTY_FACET_FILENAME` from the file **kind**
(`windowEmptyFacetFileKindId` → `markdown` → generic `📝️.md`), while the taxonomy's projection
contract `semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"]` names the
**file** — `sourceFilename: "📌️.empty.md"` — and cites that same kind as its `fileKindAuthority`.
The gate's sibling constant `PLUGIN_EMPTY_LANE_FILENAME` (`:164`) already resolves it correctly.
Measured: **4265 `📌️.empty.md` markers** in the plugin tree against **4 `📝️.md`**, and three of those
four are wfc lanes written 2026-09-18 by the *other* emitter
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🧱️contract/🟦️.ts:13`, "Authored by
`bun ./📜️script.ts new …`") — i.e. the drift is live and producing files right now.

Fixed at `🗿️taxonomy-validation/🟦️.ts:175` (now `= PLUGIN_EMPTY_LANE_FILENAME`, with the reasoning
recorded on it). The 61 lanes this slice wrote therefore carry `📌️.empty.md`, matching the 4265.
**Not fixed, and not mine:** the second emitter in `🏗️authoring/🧱️contract/🟦️.ts` spells its own body
(`# Empty <facet> facet` / "Authored by `bun ./📜️script.ts new …`") and writes `📝️.md`; the three
`🀄️wfc` lanes it produced are the only authored `📝️.md` facet markers in the plugin tree.

### S5.2 133 subset lanes + 78 example-content lanes — real migration backlog, deliberately not silenced

The first reading was that these are a gate defect, because the thin subsets are *declared* as
delegating: `🗄️stdio`'s `💬️bcf`, `📼️avi`, `🖋️dxf`, `☁️las`, `🎞️gif`, `🗽️obj` subset descriptors say in
prose that "the other two subsets reuse it (**family-module pattern**) rather than duplicating it",
and `🔣️taxonomy.json:29596` already carries the vocabulary `subsetArchetypes: ["owning", "derived"]`
with `archetype`/`derivesFrom`/`ioFidelity` live in three descriptors
(`🧿️semio`, `📰️xml`). Making the law archetype-aware would have taken all 133 to ~0.

**That reading is wrong, and the repo says so in writing.** The plan that introduced the vocabulary —
`.cursor/plans/subset_conformance_roundtrips_c57a3e1a.plan.md` — defines the two archetypes as
*"Owning: owns snapshot, diff, mutation, and inference types. **Derived: reuses owning types but owns a
real conformance gate, TypeScript mirror, inference, IO declaration, positive and negative
examples**"*, and its rule 5 requires **every** subset to own "snapshot, diff, mutations, inferences,
import, export, engine, and examples", adding that "**hollow re-exports … do not satisfy the
contract**". Its own todo list has `migrate-subsets` (all 138 subsets) still `in_progress`.

So the gate is right and these are the unfinished tail of that migration. Measured with
`🐍️v3b-lane-census.ts` (this folder) over the gate's own rows
(`🗑️generated/v3b-s5-lanes.txt`): **73 subsets**, in exactly two shapes —

| shape | subsets | reading |
|---|---|---|
| has `🧬️schema` (+7 that do not), missing `🚪️io/` **and** `📚️examples/` | **48** | slice subsets of a container the sibling owns: `🧊️gltf` 8, `🗒️note` 8, `🏗️fem` 10, `🖍️draw` 4, `➗️mathematical` 3, `🎬️sequence` 2, `🗄️stdio` 13 |
| complete (`🧬️schema 🚪️io 👁️viewer ✏️editor`), missing only `📚️examples/` | **25** | 24 `🗄️stdio` + 1 `🧱️block` — subset exists, nobody authored an example |

**Every one of the 73 has a complete sibling subset** except 2, which is what makes the delegation
reading tempting and the plan's rule 5 decisive. Landing them means authoring 48 IO declarations and
73 example sets (each `🦀️.rs` + `🟦️.ts` + `🖼️assets/` + `🧪️tests/`) — real product content for the
subset-migration owner, not a gate change. **Decision: leave the 211 rows red**, because the only way
to turn them green from here is either to author the content or to weaken a law the plan wrote
deliberately. Recorded rather than silenced.

The 78 example-content rows are the same backlog one level down: examples that exist but are missing
`🦀️.rs` (23) / `🟦️.ts` (25) / `🧪️tests/` (25) / `🖼️assets/` (13), 64 of them in `🗄️stdio`.

### S5.3 The 120 `🧊️gltf` unreachable leaves — dead, and here is the proof

Session 4 mounted them, `cargo check --all-targets` produced **458 errors**
(`🗑️generated/v3b-s4-cargo-gltf.txt`), and they were unmounted again. Session 5 answers *why* they do
not compile, which decides mount-vs-delete:

Every one of the 116 files imports `use crate::schema::mutations::<verb>::{diff, inverse, mutation};`
and calls `mutation::apply`, `diff::derive`, `inverse::derive`, and matches on types like
`diff::GltfCreateAnimationOperation`. Measured:

* **that import shape exists in exactly one artifact in the whole repo — `🧊️gltf` — and nowhere else**
  (`grep -rl … ✏️s/🔌️plugins` → 116 files, all gltf);
* `🧊️gltf` has **no mutation facet module directories at all**: its 241 `🔺️diff` dirs are all under
  `🧫️fixtures/`. Every gltf mutation inlines `apply` + `MutationKind::{diff,inverse}` into its direct
  leaf (`…/✅️required-extension/➕️add/🦀️.rs:31,47-70`);
* the leaf **never had** those modules: `git show` at `599a5d8450`, `025ec86a42`, `6152f9ca6a`,
  `0a0bb74380`, `fe7c8a8f8b` (back to 2026-09-05) shows one `#[path]` line, the `🔬️direct-leaf` test;
* `pub fn derive` is defined **nowhere** in `🧊️gltf`, and `GltfCreateAnimationOperation` occurs
  **once in the whole repo — in the test that asserts on it**;
* the 244 `E0277`s are the same story at the payload level: the tests `serde_json::from_str` the
  payload structs, and no gltf payload carries the `#[cfg_attr(test, derive(serde::Deserialize))]`
  that `🧱️block`'s equivalents do (`grep -c` over gltf → **0**).

So these leaves have never compiled since birth (2026-08-21) and were written against an API that was
never authored anywhere. Meanwhile **every** gltf mutation already has a `🧪️tests/🔬️direct-leaf/🦀️.rs`
that *is* mounted and *does* compile (120 of them). The committed fixtures they read
(`🧫️fixtures/🧬️mutations/…/🔬️tNNN/`) are separate files and are not touched by any verdict here.

**Decision: these are dead, not unmounted product code — but deleting 116 substantial authored
oracles is the artifact owner's call, not a gate slice's.** What is not defensible is what session 4
briefly had: mounting them so the gate goes quiet while the crate stops compiling. They are therefore
left unmounted and the gate left red at 120, with the evidence above. The cheapest real fix, for
whoever owns `🧊️gltf`, is the one the repo's own contract already prescribes
(`mutationDirectLeafInlinedBehaviorFacets`, `…/📚️library/🔍️discovery/🟦️.ts:3877-3904`): give each gltf
mutation its `🧬️mutation`/`🔺️diff`/`↩️inverse` facet directories — the shape `🧱️block/🖐️5d` uses
(`…/🙅remove-author/🔺️diff/🦀️.rs`) — at which point the 116 oracles compile as written.

### S5.4 Gate defect E — a surface's `📚️examples/🧪️tests/` was audited as a fourth example

`📸️remodel`'s editor surface carries `📚️examples/{🎬️demo-session, 🦀️.rs, 🧪️tests/🔬️unit}`. The
**subset**-level example walk excludes the two test-owner names
(`🗿️taxonomy-validation/🟦️.ts:503`, `!== TAXONOMY.testsDirName && !== TAXONOMY.testFixturesDirName`);
the **surface**-level walk 50 lines above it did not. So `🧪️tests` was treated as an example slug and
asked for a `🦀️.rs`, a `🟦️.ts`, an `🖼️assets/` and a `🧪️tests/` of its own — 4 findings. Nothing else
catches it either: `🧪️tests` *passes* the emoji+VS16+kebab slug pattern.

Fixed at `🗿️taxonomy-validation/🟦️.ts:547-551` by applying the same exclusion, with the reason
recorded. Measured: **342 → 338**, exactly the 4 `📸️remodel` rows and nothing else.

### S5.5 The 13 remaining owner decisions — taken, with the evidence for each

| # | finding | decision | evidence |
|---|---|---|---|
| 1–4 | `🗄️stdio: artifact "🛂️contract" / "🕸️graph" / "📇️inventory" / "🏃️commands" is missing 🏅️standards/` | **Not artifacts — move them out of `🗿️artifacts/`.** Each directory contains exactly one file, `🟦️.ts`, and no Rust, no standard, no subset. `🕸️graph/🟦️.ts` imports `CARGO_COMPOSITION_NAME`, `StdioArtifactPackageRecord`, `canonicalStdioArtifactNames` from `../../📇️inventory/🟦️.ts` and shells out to `nx` — it is stdio's own **repo tooling** (a package-contract harness), not a document artifact. Their home is `🗄️stdio/🔨️modules/<module>/`. Left in place because the move rewrites their relative imports and their `📋️project.json` targets, which is the stdio package owner's change, not a gate slice's. | `ls` of each dir; `head` of `🕸️graph/🟦️.ts` |
| 5 | `🧱️block: artifact "◻️2d" … is missing 🧬️schema/{🔣️.json,🔗️.graphql,🛰️.proto}` + `has undeclared 🧬️schema/🧱️shared` | **One cause, not four: an artifact-level `🧬️schema/` that should not exist.** `🗿️artifacts/◻️2d/🧬️schema/` holds nothing but `🧱️shared/🦀️.rs` (163 lines of `BlockKindIdentity` etc., live — mounted at `◻️2d/🦀️.rs:29`). Its two sibling artifacts `🧊️3d` and `🖐️5d` have **no artifact-level `🧬️schema` at all**; all three keep schema under `🪆️subsets/<s>/🧬️schema/`, which is where `_standardsSubsetsComment` puts it. Because the facet exists, the gate then demands the three spec files of it. **Recommended home: `✏️s/🔌️plugins/🧱️block/🔨️modules/🧱️shared/`** — the module level, exactly where the ENGINELESS ticket moved cross-artifact behaviour. Not moved here: it changes `#[path]` in three crates and is the block owner's call. | `ls` of all three artifacts; `grep` for the mount |
| 6 | `🎪️demonstrator: plugin root is missing 🦀️.rs` | **Real: the entry sits one level too deep.** `📦️packages/🦀️rust/Cargo.toml:130-132` declares `[lib] path = "🦀️.rs"`, resolving to `📦️packages/🦀️rust/🦀️.rs`, while `🗄️stdio`, `🧱️block` and `🀄️wfc` all declare `path = "../../🦀️.rs"` — the `leaf-prefixed` convention `rustEntryPathRules` names. Moving the entry to the plugin root is a `#[path]`-resolution change across a crate that bundles six panes; left for the demonstrator owner with the exact two-line fix recorded. | the four `Cargo.toml` `[lib]` blocks; `rustEntryPathRules.conventions` |
| 7 | `🗄️stdio: move the redundant 🔌️plugin contract and facets directly into the plugin root, then remove 🔌️plugin/` | **Agreed, and cheap in shape but not in risk.** `🗄️stdio/🔌️plugin/` holds one file, mounted at `🗄️stdio/🦀️.rs:9` (`#[path = "🔌️plugin/🦀️.rs"]`); it is the closed `dyn_enum_close!` app fleet. Inlining it is a copy plus deleting one `#[path]`, but `semio-s-plugin-stdio` is the slowest crate in the tree and this slice could not afford the verification round. Recorded for the stdio owner. | `ls 🗄️stdio/🔌️plugin`, `grep` for the mount |
| 8 | `📐️cad: window "✳️any/✏️editor/✏️edit/🎚️config" has unexpected child "🧬️schema"` | **The window law is right and `📐️cad` is the outlier.** `windowChildDirs` has no `🧬️schema`; a window's config schema belongs to the surface lane (`surfaceSchemaSpecFileKinds` maps only `🎚️config/🧬️schema` and `👥️presence/🧬️schema` at *surface* level). Left for the cad owner — relocating a schema lane changes what `policyDiscoverAppSchemaOwners` sees. | `TAXONOMY_WINDOW_CHILDREN`, `surfaceSchemaSpecFileKinds` |
| 9–13 | `artifact "X" subset "X" is missing PATH` ×2, `plugin root is missing PATH` ×1, `example … is missing 🟦️.ts` (`🪐️space 🎬️demo`) | same backlog as §S5.2 — authored content that was never written | census rows |

### S5.6 Compile proof for the 207 live session-4 mounts — the §6.1 gap, partly closed

The fleet-wide cargo deadlock of §6.1 was gone this session (the lock holder, a peer's
`cargo test -p semio-s-plugin-cad-aec-building`, had a live `rustc` child throughout — working, not
wedged). After the `🧊️gltf` revert, **207** of the 327 session-4 mounts are still in the tree, and
their owners are led by `semio-s-artifact-stdio-semio` with **74** — a third of the total and by far
the largest single crate.

```
$ cargo check -p semio-s-artifact-stdio-semio --features component-app-assembly --all-targets
warning: `semio-s-artifact-stdio-semio` (lib) generated 4 warnings
warning: `semio-s-artifact-stdio-semio` (lib test) generated 4 warnings (4 duplicates)
    Finished `dev` profile [unoptimized] target(s) in 11m 58s        # exit 0, 63 warning lines
```
(`🗑️generated/v3b-s5-cargo-semio.txt`.) Warnings present, so the type-check really ran rather than
aborting early — the 74 `🧿️semio` mounts, including its `#[cfg(test)]` test-case leaves under
`--all-targets`, compile.

The rest was compiled after the 06:12 fleet-wide cargo kill left the machine calm — see §S5.10.

### S5.7 V3a's two 🪐️space schema lanes (K3 hand-off) — landed and verified

Taken over mid-flight after K3 was killed at ~01:45. K3's edits were already in the working tree and
are **complete, not half-applied** — verified by `git diff` before touching anything:

* **`uint64` in the repo schema library** — `📚️library/🧬️schema/🔍️field-discovery/🧱️contract/🟦️.ts:33`
  (`u64: "uint64"` in `policyCanonicalScalar`) and `…/🔣️json-schema/🟦️.ts:14`
  (`format === "uint64"`). Two lines; the other three parsers (rust/graphql/protobuf) route through
  `policyCanonicalScalar` and need no entry because an unknown token falls through unchanged.
* **the projection's scalar table** — `📇️registry/🧬️surface-schema/🟦️.ts:59`:
  `uint64: { json: { type: "integer", format: "uint64", minimum: 0, maximum: SAFE_INTEGER_BOUND }, rust: "u64", typescript: "number", graphql: "Long", proto: "uint64" }`,
  with the JS 53-bit bound recorded on the table's own docstring (`:45-52`).
* **nested `$defs`** — `:286-289` emits `{ $ref: "#/$defs/<Type>" }` for a non-scalar field and
  `:244,:271` refuse a lane whose composite is neither declared nor resolvable.

Both lanes are now real files, and they are the projection's output, not hand-carving:

| lane | result |
|---|---|
| `🪐️space/🗿️artifacts/🪐️space/…/✏️editor/🎚️config/🧬️schema/🔣️.json` | `SpaceIndexConfig` with `$defs` for `SpaceIndexMember`, `SpaceArtifactRow`, `SpaceIndexArtifactPresence`, each field `$ref`-ed from the property |
| `🪐️space/🗿️artifacts/🏠️home/…/👁️viewer/🎚️config/🧬️schema/🔣️.json` | `directoryAuthorizationGeneration` as `{"type":"integer","format":"uint64","minimum":0,"maximum":9007199254740991}` |

Verified, all three re-run tonight:

```
$ bunx vitest run --config 🧪️tests/🎚️config/🟦️.ts 🧪️tests/🧬️surface-schema/🟦️.ts
 Test Files 1 passed (1)   Tests 12 passed (12)          # V3a's 9 + K3's 3, exit 0
$ bun ./📜️script.ts surface-schema        (twice in a row)
 572 lane(s): 0 written, 524 current, 48 authored, 0 blocked, 0 drifted     # both runs identical
```
(`🗑️generated/v3b-s5-vitest-surface-schema.txt`, `v3b-s5-surface-schema-run1.txt`, `…-run2.txt`.)
Terminating and idempotent: the second run writes nothing, and `blocked`/`drifted` are 0, so no lane
is refused and no authored lane disagrees with what the projection would emit.

**Schema-lane family: 2 → 0** — confirmed twice over: the fast census and the **real
`plugin-registry check`** both report zero `🎚️config/🧬️schema` / `👥️presence/🧬️schema` rows among the
338 (`grep -cE … v3b-s5-check.txt` → 0). The only `🪐️space` row left is an example-content one
(`example "🎬️demo" is missing 🟦️.ts`, §S5.2's backlog).

Re-verified after the 03:00 fleet cut and the peers' churn in this module (`🗑️generated/v3b-s5b-*`):
`Tests 12 passed (12)`, and the projection still reports `0 written, 524 current, 48 authored, 0
blocked, 0 drifted` — so the repo-wide run *has* now been done, three times, with identical output.

**One thing I did not fix, recorded because it will bite:** `🏠️home`'s **editor** config lane is an
*authored* lane (no `🤖️Generated by` comment) and its `directoryAuthorizationGeneration` carries
`{"type":"integer","minimum":0,…}` with **no `"format": "uint64"`**, while the generated viewer lane
beside it does. `policyJsonSchemaScalar` reads a formatless `integer` as `int32`, so the two lanes
describe the same Rust `u64` field with two different canonical scalars. The generator cedes authored
lanes by design, so it will never correct this; the space owner should either add the format or let
the projection own that lane.

### S5.8 Gate defect F — two laws in the same module contradicted each other and aborted the whole gate

The first full `bun ./📜️script.ts check` of this session did not reach a single taxonomy row. It
exited after **one** line:

```
plugin registry catalog has playground validation errors:
  - playground variant "s" in ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust must set "app" (crate declares 3 playground entries)
```

Cause, measured: a peer's **uncommitted** working-tree edit to
`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` added two app variants (`home`, `space`) beside the
long-standing app-less `s` row, turning that crate into a 3-entry group. `validatePlaygroundRegistry`
then required `app` on **every** row of a multi-entry crate — including the shell row.

But `defaultHostVariant` (`📇️registry/🎮️playground/🔎️discovery/🟦️.ts:210-214`) states the opposite in
its own comment — *"The host crate also ships ordinary artifact apps as their own single-app
playgrounds (Home, Space); the row that boots the SHELL is the one naming no `app`"* — and **throws**
unless the host crate has exactly one app-less row. So the two laws in the same module could not both
be satisfied, and the peer's edit was exactly the shape `defaultHostVariant` was written for. This is
a hard `process.exit(1)` **before** the taxonomy audit, so it blocked the gate for every slice, not
just mine.

Fixed at `🗿️taxonomy-validation/🟦️.ts:13-23,74-85` (+ `📽️projection/🟦️.ts:494-495`, which already had
the plugin entries in scope): the multi-app rule takes the host plugin ids and skips the host crate's
single app-less row. **It still discriminates** — a second app-less row on the host crate is still a
violation, and an app-less row on any crate that declares no `[package.metadata.semio].host` is
unchanged. I did not touch the peer's `Cargo.toml`.

With that, the gate runs end to end again:

```
$ bun ./📜️script.ts check         # 🗑️generated/v3b-s5-check.txt, exit 1, 338 finding lines
plugin taxonomy tree violations (area(s) "✏️s/🔌️plugins" is "clean"):  … 338
```

### S5.10 §6.1 closed — all 203 live mounts compiled, and two of them were hiding real bugs

After the coordinator killed the 34 deadlocked cargos at 06:12 the build dir was usable again, so the
203 live mounts were compiled in three scoped runs (all `--all-targets`, the target `#[cfg(test)]`
leaves need):

| run | crates | mounts | result |
|---|---|---|---|
| `-p semio-s-artifact-stdio-semio --features component-app-assembly` | 1 | 74 | **exit 0**, 63 warnings, 11m58s |
| the 15 `-p semio-s-artifact-norm-*` | 15 | 53 | **exit 0**, 92 warnings, 8m53s |
| the remaining 27, `--keep-going` | 27 | 76 | 5 crates failed → §below |

(`🗑️generated/v3b-s5-cargo-semio.txt`, `v3b-s5b-cargo-norm.txt`, `v3b-s5b-cargo-rest2.txt`.) Warnings
are present in every run, so the type-check reached the bodies rather than aborting in expansion.

**Two failures were mine, and both were real defects the mount exposed — fixed, not unmounted:**

* `semio-s-artifact-forms-forms` — my 2 mounts of the `▶️try` window's `🎚️config/🧬️schema/🦀️.rs` and
  `🫧️transient/🧬️schema/🦀️.rs` made the `ArtifactSchema` derive reject
  `#[state(window_config)]` / `#[state(window_transient)]`: *"unknown state class … the only four
  lanes are artifact, config, presence, transient"*. Measured across every window config schema leaf
  in the tree, **7 use `#[state(config)]` and exactly 1 used `window_config`** — forms. Corrected to
  `config` / `transient` at `…/▶️try/{🎚️config,🫧️transient}/🧬️schema/🦀️.rs:9,10`. These two leaves had
  never compiled; the gate row was right and the code behind it was broken.

**One failure was mine and the *mount* was wrong — unmounted, and it is a second instance of gate
defect A (§1.2):**

* `semio-s-artifact-note-note` — `🪆️subsets/✳️any/🔮️oracles/🦀️.rs` and its `🧪️tests/🔬️smoke/` leaf
  import `semio_repo_test_host::Json` and `lopdf`, neither of which is a dependency of the note
  artifact crate and neither of which ever could be: `semio-repo-test-host` is deliberately outside
  the workspace (§1.2). `🔮️oracles/` is **test-platform-owned, like `🧪️tests/<case>/`** — the
  exemption `isRepositoryTestCaseAdapter` grants only to cases beside a `🥒️.feature`. Only two
  plugins in the tree mount a `🔮️oracles/🦀️.rs` at all, and one of them was my session-4 mount.
  Removed from `🗒️note/…/✳️any/🦀️.rs:38-45`; note then compiles (with `📋️forms`) **exit 0, 158
  warnings**. The 2 gate rows come back, honestly, and the durable fix is to extend
  `isRepositoryTestCaseAdapter` to the `🔮️oracles` owner kind — recorded, not landed, because it
  wants its own compiler-checked oracle case like §1.4's.

**Two failures are not mine** (verified against `🗑️generated/v3b-s4-mount-apply.txt`, 0 matches):

* `semio-s-plugin-space` + `semio-s-artifact-vcs-vcs` — both call
  `semio_framework_plugin::artifact_app_laws::settle_framework_reserved_admission`, which now lives at
  `app::artifact_app_laws` (`🧰️framework/…/🔌️plugin/🦀️.rs:6936,28532`) and is re-exported through
  `plugin_app_close_prelude`. A live framework re-export move by a peer, in pre-existing mounts;
  **coordinator: this breaks two crates' `lib test` for everybody**, the fix is the prelude path the
  compiler itself suggests.
* `semio-s-artifact-flow-flow` — 9 × `E0509` in
  `✏️editor/🧵️retained/🗿️artifact/…/🧪️tests/🔬️unit/🦀️.rs`; pre-existing mounts, not in my apply list.

So: **198 of the 203 live mounts are compiler-verified**; the 5 in `🪐️space`/`🌿️vcs` wait on the
framework re-export above, and `🌊️flow`'s 11 sit in a crate broken by someone else's `E0509`s.

### S5.9 Gate numbers — the real `plugin-registry check`, measured end to end

```
plugin-registry check total        619  →  338          (🗑️generated/v3b-s5-check.txt)
                                         →  340          after the deliberate 🗒️note 🔮️oracles unmount (§S5.10)
```

The fast census (`🐍️v3b-taxonomy-census.ts`) reported **338** for the same tree at the same moment, so
the two independent paths agree exactly — the census is a faithful stand-in for the 35-minute gate.
The final census (`🗑️generated/v3b-s5b-census-final.txt`) reads **340**: the two rows that came back
are the note oracle leaves this slice chose to stop mis-mounting.

| family | session 4 start | session 4 end | **session 5** |
|---|---|---|---|
| **`plugin-registry check` total** | **619** | 405¹ | **340** |
| `unreachable-from-cargo-manifest` | 333 | 120 | 122² |
| subset directory lanes (`📚️examples` 66, `🚪️io` 48, `🧬️schema` 13, `🏅️standards` 4, misc 2) | 133 | 133 | 133 |
| example-content lanes | 78 | 78 | **74** |
| `mode … is missing required child` | 61 | 61 | **0** |
| surface schema lanes (V3a/K3) | 2 | 2 | **0** |
| owner decisions | 13 | 13 | 13 |

¹ census figure; the gate itself could not be run at the end of session 4 — and when it was first run
this session it aborted on §S5.8's contradiction without printing a single taxonomy row.
² 120 gltf (§S5.3) + 2 note oracle leaves deliberately unmounted (§S5.10).

**What this session actually changed, family by family:**

| family | before | after | how |
|---|---|---|---|
| `mode … is missing required child` | 61 | **0** | 61 empty lanes written with the scaffolder's own emitter (§S5.1) |
| surface `📚️examples/🧪️tests` rows | 4 | **0** | gate defect E (§S5.4) |
| surface schema-lane family (V3a/K3 hand-off) | 2 | **0** | verified + re-run repo-wide (§S5.7) |
| gate aborting before any taxonomy row | blocked | **runs** | gate defect F (§S5.8) |
| live mounts compiler-verified | 0 | **198 / 203** | §S5.10, incl. 2 real bugs fixed and 1 wrong mount removed |
| `unreachable-from-cargo-manifest` | 120 | 122 | §S5.3 / §S5.10 — verdicts recorded, never silenced |
| subset + example-content lanes | 211 | 211 | §S5.2 — the subset-migration plan's own backlog |
| owner decisions | 13 | 13 | §S5.5 — each decided in writing, each left to its owner |

---

## 7. Files changed

### Session 5

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:169-175` | `WINDOW_EMPTY_FACET_FILENAME` now resolves from the empty-facet projection contract instead of the markdown file kind (§S5.1) |
| `…/📇️registry/🗿️taxonomy-validation/🟦️.ts:547-551` | the surface example walk excludes `🧪️tests`/`🧫️fixtures`, as the subset walk already did (§S5.4) |
| `…/📇️registry/🗿️taxonomy-validation/🟦️.ts:13-23, 74-85` + `…/📇️registry/📽️projection/🟦️.ts:494-495` | the multi-app playground rule exempts the host crate's single app-less shell row, which `defaultHostVariant` requires (§S5.8) — this is what unblocked the gate |
| `✏️s/🔌️plugins/📋️forms/…/🪟️windows/▶️try/{🎚️config,🫧️transient}/🧬️schema/🦀️.rs:9,10` | `#[state(window_config\|window_transient)]` → `#[state(config\|transient)]`, the derive's closed vocabulary (§S5.10) |
| `✏️s/🔌️plugins/🗒️note/…/🪆️subsets/✳️any/🦀️.rs:38-45` | the 2 `🔮️oracles` mounts removed — test-platform-owned, not Cargo-owned (§S5.10) |
| 61 × `✏️s/🔌️plugins/{📕️norm×15,🪐️space}/…/🎭️modes/<mode>/{🎮️commands,🎚️config,👥️presence,🫧️transient}/📌️.empty.md` | new empty mode lanes (§S5.1) |
| ticket folder | `🐍️v3b-mode-lanes.ts`, `🐍️v3b-lane-census.ts` (new); captures `🗑️generated/v3b-s5-*` |

Inherited from K3 and verified, **not** authored by this slice (§S5.7): the `uint64` entries in
`📚️library/🧬️schema/🔍️field-discovery/{🧱️contract,🔣️json-schema}/🟦️.ts`, the `$defs`/`uint64` work in
`📇️registry/🧬️surface-schema/🟦️.ts` + its tests and fixture, and the two `🪐️space` lane files.

### Sessions 1–4 (carried)

| file | change |
|---|---|
| `…/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` | `RustModuleIncludeFact` + `includes` on `inspectRustModuleGraphFacts` (§1.3); `rustTokens` char-literal escape handling (§1b.3); `FrozenCoordinateEvidenceRetirement` + `frozenCoordinateEvidenceSeal` (§4.1); `🗑️generated` in `DISCOVERY_SKIP_DIRS` (§5.1) |
| `…/💻️os/…/📇️registry/🗿️taxonomy-validation/🟦️.ts` | `isRepositoryTestCaseAdapter` + `TEST_FEATURE_FILENAME` (§1.2); `includeClosure` (§1.3) |
| `…/📇️registry/🧫️fixtures/🕸️rust-taxonomy-mounts/🔣️.json` + `…/📇️registry/🧬️schema/🔣️.json` | oracle cases 10 → 13 (§1.4, §1b.3) |
| `📜️script.ts:9001-9276` | interactivity descriptor coordinate, gate list + loop, launch command templates, extension role/bundle id (§3) |
| `…/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts` | self-tests 25 → 29 (§3.5) |
| `…/📚️library/🔣️taxonomy.json` | 28 `retired` records on `frozenCoordinateEvidenceContracts` (§4.2) |
| `…/📚️library/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts` | three retirement laws (§4.3) |
| `.vscode/launch.json` + `🧩️launch.seed.jsonc` | six `⚖️gate…` rows restored (§3.2) |
| 28 × `…/🧬️mutations/📸️set-snapshot`-owning artifact leaves | 75 facet mounts (§1.6) |
| 210 host leaves in 23 plugins | 327 `#[path]` mounts, of which 120 (`🧊️gltf`) were reverted in session 4 → **207 live** (§1b.1, §S5.0) |
| 2 × `🧊️gltf` barrel leaves | `mod component; pub use component::*;` + corrected docstrings (§1b.2) |

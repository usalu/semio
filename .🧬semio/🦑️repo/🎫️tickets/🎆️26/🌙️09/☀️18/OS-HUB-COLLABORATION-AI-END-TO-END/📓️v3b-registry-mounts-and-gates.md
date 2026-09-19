# V3b — Unreachable Rust mounts, directory lanes, interactivity gates, frozen-contract retirement, red-gate triage

Slice V3b of ticket 26/09/18 OS-HUB-COLLABORATION-AI-END-TO-END. Inputs: `📓️v1-verification-gates.md`,
`📓️v2-launchers-registry-taxonomy-perf.md`. Sibling V3a owns the 615 schema lanes; everything else in
`plugin-registry check` (1836) is mine.

Captures: `🗑️generated/v3b-*.txt`. Status: **IN PROGRESS** — sections fill as they finish.

---

## 0. Headline

| # | Item | Before | After |
|---|---|---|---|
| — | `plugin-registry check` total | **1836** | **1363** (`🗑️generated/v3b-check-1.txt`, exit 1) |
| 1 | `unreachable-from-cargo-manifest` | **892** | **415** — 477 were two gate defects, both now compiler-checked (§1) |
| 1 | `rust-taxonomy-mounts-check` oracle cases | 10 | **12**, `compiler=12`, exit 0 |
| 1 | unmounted `📸️set-snapshot` behaviour facets | 75 across 28 artifacts | **0** — mounted; all **17** affected crates `cargo check` exit 0 |
| 2 | directory lanes (133 subset + 61 mode) | 194 | _pending_ |
| 2 | taxonomy-owner decisions | 45 | _pending_ |
| 3 | `verify interactivity apps` | **776 failures** (the "25" in status.md was `selfTests=25`) | **44**, all one real family (§3) |
| 3 | …its discovered surface | descriptors 3, apps 14, actions 487 | descriptors **45**, apps **127**, actions **6254** |
| 3 | …its self-tests | 25 | **29** |
| 4 | dead `frozenCoordinateEvidenceContracts` | 28 | **0 unrecorded** — all 28 retired with ticket + reason, seal digest unchanged (§4) |
| 4 | `🕰️historical-json-source-encoding` suite | 17 pass / 3 fail | **21 pass / 1 fail**; the 1 is a seal that never matched its own birth commit |
| 5 | `verify layering` / deps literal-external / deps freeze / package-purity | 213 / 236 / +7 / 434 | _pending_ |

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

## 2. 194 directory lanes + 45 taxonomy-owner decisions

_pending_

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

_pending_

---

## 7. Files changed

_pending_

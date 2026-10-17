# 🤝 Monolith Split Fleet: Consolidated Hand-offs

Status: AUDIT (read-only), 2026-10-16. Auditor: Haiku, foreground, no sub-agents, no git writes, no source edits.

Folder `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.

## 0. Scope and method

- Read: `📓️FLEET.md` (waves 5 to 8, the 2026-10-16 entries, and the dispatch list at its end; FLEET has no separate running-slice list, so the running set in F-09 is inferred from the dispatch entries and the working folders).
- Scanned: all 95 `RESULT-*.md` files modified on 2026-10-16 (`/usr/bin/grep` for `hand-off`, `handoff`, `parent`, `peer`, `policy`, `Root`, `owner decision`, `user`, `needs`, `not done`, `left`, plus policy tokens `pathNot`, `allow-list`, `BOOTSTRAP_TOOLING`, `no-core-path`, `exclusion`, `node_modules`, `r3f`). Read the hand-off, residual and "not done" sections of about 50 of those files, and the full policy-line and hand-off-line extracts of all 95. This is a grep-led audit: not every line of every RESULT was read.
- Also read: `HANDOFF-3D-MESH-CARVE-IMPORTS.md`, `HR2-NOTICE-FOR-PSC2.md`, `📓️COMPILE-RUNBOOK.md` section 15 and 17, and `📓️COMPILE-WINDOW-CHECKLIST.md` by reference.
- Verified read-only against the live policy file `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs` (mtime 2026-10-16 06:06). Its state:
  - not applied: the transient `pathNot`, the UI tooling exemption, the r3f line, the react real-path entry, the renderer tooling exemption, the icon/asset `pathNot`;
  - applied: `dependencyTypesNot: ["aliased-workspace"]` (line 194, SDK-BOUNDARY).
- The regex in P-08 was tested with node against sample paths. The cruiser was not rerun.
- Not verified: whether the parent's ratchet, taxonomy and library edits listed in group 2 have landed after 06:06. Several of those files are parent-held and were not re-measured.

Group sizes: group 1 = 12 items, group 2 = 10, group 3 = 15, group 4 = 11, group 5 = 13. Total 61.

Shorthand for evidence: `EPH` = RESULT-EPHEMERAL-SINGLE-OWNER.md, `UIB` = RESULT-UI-BARREL-IMPORTS.md, `RHU` = RESULT-RENDERER-HOSTS-UI.md, `RHU2` = RESULT-RENDERER-HOSTS-UI-2.md, `RHU3` = RESULT-RENDERER-HOSTS-UI-3.md, `HR2` = RESULT-HYGIENE-RULES-2.md, `SDKB` = RESULT-SDK-BOUNDARY.md, `SDKB2` = RESULT-SDK-BOUNDARY-2.md, `PSC` / `PSC2` / `PSC3` = RESULT-PACKAGE-SPECIFIER-CODEMOD{,-2,-3}.md, `TAXD` = RESULT-TAXONOMY-DEBT.md, `TTE1` / `TTE2` = RESULT-TS-TYPE-ERRORS-1 / -2.md, `TSCU` = RESULT-TSC-UNBLIND.md, `INFRA3` = RESULT-INFRA-UNBLOCK-3.md, `RTD` = RESULT-RENDERER-TEST-DEPS.md, `RI3` = RESULT-RENDERER-INTEGRATION-3.md, `DRT` = RESULT-DRAW-RUNTIME-TESTS.md.

Policy file path shorthand: `CFG` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`. Taxonomy file shorthand: `TAX` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.

---

## Group 1. Parent-ticket policy edits (`CFG` and the taxonomy engine)

Every item in this group is held by the parent. The fleet did not apply any of them except P-11, which is already in the file and needs ratification.

### P-01 Transient kernel store admitted into the UI rule

- Change: in the rule `ui-no-framework-packages` (`CFG` line 422, `to.pathNot` at line 426) add `"^🧰️framework/🔨️modules/🫧️transient/"`. Result: `pathNot: ["^🧰️framework/🔨️modules/🖱️ui/", "^@semio-tech/ui-", "^🧰️framework/🔨️modules/🫧️transient/"]`.
- Evidence: EPH section 2.3 (lines 50-54) and section 6.1; FLEET 07:2x. Not in `CFG` as of 06:06.
- Impact: 16 `ui-no-framework-packages` edges removed (260 to 244). Also the only non-UI path UI may import, so the UI twin of the ephemeral store stays deleted.

### P-02 Decision T: tooling entries exempt from the UI rule

- Change: in `ui-no-framework-packages` replace `from: { path: "^🧰️framework/🔨️modules/🖱️ui/" }` (line 423-424) with `from: { path: "^🧰️framework/🔨️modules/🖱️ui/", pathNot: BOOTSTRAP_TOOLING_ENTRY_PATH }`. The constant already exists at `CFG` line 23 and is already used by the cross-technology rules.
- Evidence: UIB section 1 (decision T, lines 25 and 52-56), UIB section 7.1; FLEET 07:4x and 07:2x ("Decision T").
- Impact: 93 edges (of 187 non-peer edges, of which 87 import `🏃️process`). The UIB text says 175 after its own edits; the 93 counted here are the tooling entries.

### P-03 Renderer rule: tooling entries exempt

- Change: in `rendererHostsOnlyUiRule()` (`CFG` around line 368-376) add `pathNot: BOOTSTRAP_TOOLING_ENTRY_PATH` to `from`, next to `path: RENDERER_HOST_ROOT`.
- Evidence: RHU line 127 (HO-9, item a) and RHU3 section 3 "Tooling, 8 production edges" (line 98).
- Impact: 8 production edges of `renderer-hosts-only-ui`, all `📜️script.ts` or vite and vitest entries.

### P-04 React real-path entry (renderer rule, `node_modules` symlink)

- Change: in `RENDERER_HOST_ALLOWED_RESOLVED_PATHS` (`CFG` lines 26-34) add an entry built from the real path of the root `node_modules` link, for example `const NODE_MODULES_REAL = path.relative(process.cwd(), fs.realpathSync("node_modules"));` and the pattern `^${escapeRegex(NODE_MODULES_REAL)}/(?:react|react-dom|vite|vitest)(?:/|$)`. Keep the plain forms already present (lines 30 and 32). Do not switch the cruiser to `preserveSymlinks`: RHU section 2 says every workspace would break.
- Evidence: RHU lines 40-46: 178 `react` and `react-dom` edges of the renderer, 0 resolve to a plain `node_modules/` path because `node_modules` is a symlink into `.cache/b/d/m/j/<hash>/node_modules`. RHU3 section 5 remaining list (127 react resolver artefacts).
- Impact: 127 `renderer-hosts-only-ui` edges, all artefacts of the resolver. The hash changes per install, so the entry must be computed, not hard-coded.

### P-05 r3f allow-list line

- Change: add to `RENDERER_HOST_ALLOWED_RESOLVED_PATHS` (exact text, RHU3 section 3):
  `"^🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/",`
- Evidence: RHU3 section 3 (lines 89-96); FLEET 21:0x. The RHU3 check says it matches exactly the 14 r3f edges and nothing else.
- Impact: 14 edges (12 production, 2 test). Alternative with no policy change: move the package under `🖱️ui` (decision U-11).

### P-06 Renderer tests, stories and fixtures exempt (policy decision)

- Change (proposal): in `rendererHostsOnlyUiRule()` add `pathNot` for test, story and fixture importers, for example `(^|/)(🧪️tests|🧫️fixtures)/|\.stories\.`. The RESULT does not give the exact string; this is a decision for the parent.
- Evidence: RHU line 127 (HO-9, item b); RHU2 rule-scan row (line 32: `renderer-hosts-only-ui` 535 = 326 production + 209 tests and stories).
- Impact: 209 edges. Their mix: 52 artefacts, 48 third-party, 6 builtin, about 43 JSON fixtures, the rest os and framework types in dynamic imports. Alternative: re-home the fixtures.

### P-07 UI rule: icon catalog and assets admitted

- Change: in `ui-no-framework-packages` `to.pathNot` add `"^🧰️framework/🔨️modules/🖼️assets/"`. The RESULT's rationale: assets already import styling, so they sit below UI.
- Evidence: UIB section 3 table, row "Icon catalog" (lines 99-100). The same row says the `🕹️interaction` package (4 edges) moves with it or gets an injected port.
- Impact: 12 production and 9 test edges.

### P-08 `no-core-path`: word-boundary stem and Node built-ins

- Change: in `noCorePathRule()` (`CFG` line 243-257):
  - replace `to.path` with `"(^|/)(?:[^/]*?[^A-Za-z0-9])?(${stems})(/|$)"` (the stem must follow a non-ASCII-alphanumeric boundary, so `🫀️core`, `🔨️geometry-core` and `foo-core` still match, while `database` and `symmetry-score` do not);
  - add `RESOLVED_NODE_BUILTIN_PATH` to `to.pathNot`, so `node:util` (which resolves to `util`) is exempt.
- Evidence: HR2 section 1 (rule semantics) and section 5 (table: 36 built-in, 7 database, 2 score). I tested the regex with node: `util` (still matches, handled by the pathNot), `zlib` and `node:zlib` (no match), `database` and `symmetry-score` (no match), `🫀️core`, `foo-core`, `🧱️shared` (match).
- Impact: about 45 production edges (36 built-in, 7 database, 2 score). HR2 production `no-core-path` went 625 to 536; the rest of the rule is covered by P-09 and the `stdio` base decision.

### P-09 `base` subset: remove the stem or relax the validator (policy option)

- Change, choose one:
  - (a) remove `"base"` from `bannedNameStems` in `TAX` (the array starts at line 30427), which stops `no-core-path` from matching `🧱️base` and `✉️base` folders without any rename; or
  - (b) relax the subset-directory rule in `validateTaxonomy` (`📚️library/🔍️discovery/🟦️.ts` near line 5382) so a subset folder name may differ from its subset id. The roster already maps ids to directories, per HR2 section 4.
- Evidence: HR2 section 4 (blocked stdio renames, 493 edges; `validateTaxonomy` requires `<one semantic emoji><subset id>`), section 5 table row 493, and HR2-NOTICE-FOR-PSC2.md (renames reverted at 17:56).
- Impact: 493 production `no-core-path` edges. The id `base` is still used about 8,900 times, so the alternative of a rename campaign is decision U-06.

### P-10 Taxonomy opaque exclusions for generated or vendored trees

- Change: add the ids of the 9 class-(c) trees to both lists that pin the exclusion set to 3 ids. Lists: `OPAQUE_PATH_EXCLUSIONS` (`📚️library/🔍️discovery/🟦️.ts` line 4157; used at 5230 and 5246) and `TAXONOMY_OPAQUE_PATH_EXCLUSIONS` (`📚️library/🧹️normalization/🔣️taxonomy/🟦️.ts` line 448). Both must change together, because adding a fourth entry to only one fails `validateTaxonomy`.
- Evidence: TAXD section 5 (exclusion pinned to `compose`, `temp-compose`, `mit-bestand-recherche`) and section 6 (the 9 class-(c) paths, all tracked in git: `extension-host-build-7a5OFB`, `…/🌱️value/📦️packages/🦀️rust/same`, three `command-*` `input` dirs under `os/📦️packages/🦀️rust`, `stdio-composition-test` built JS, `🌎️hub/…/.semio/hub/db` and `db/catalog`, the `🎤️präsentation` vendored export).
- Impact: 9 of the 28 non-dot `directory-kind-unresolved` findings. The other 18 are in running slices or are addressed by key (TAXD section 6). The ticket folder is out of scope.

### P-11 Ratify `aliased-workspace` edit (already applied)

- Change: none to apply. The edit is present in `CFG` line 194: `dependencyTypesNot: ["aliased-workspace"]` on the `to` side of the cross-package rule. The parent must confirm it.
- Evidence: SDKB section 3 ("policy edit", line 64) and FLEET 07:3x ("the coordinator reviewed it and accepts it"). SDKB test `🚀️bootstrap/🧪️tests/🟦️.ts` gained one assertion (SDKB line 242).
- Impact: `plugins-framework-sdk-only` 1763 to 725; `no-cross-technology-✏️s-to-🧰️framework` 1807 to 470. The change is restricted to relative imports, per the rule comment.

### P-12 Optional: workspace package for the transient store

- Change (optional): make the transient owner a workspace package (`package.json` with `exports["."]`, a root `workspaces` entry, `bun.lock` rows, a build-owners `packages` row) named `@semio-tech/framework-transient`. UI would import the package name, which removes the deep relative specifiers.
- Evidence: EPH section 7 (follow-ups, not done: it needs a lockfile and install round).
- Impact: 10 `no-cross-package-relative` edges and the deep relative specifiers in UI. Requires `bun install` (see U-03).

---

## Group 2. Parent-held `📚️library` work (and taxonomy registrations that the parent holds)

### L-01 B1/B5: extract repo-library runner kernels

- Description: the repo library's runners (`repo-lib`) and the repo test kit import into plugin scripts and into the library. Extract the neutral runner kernels into `🔨️modules/🏃️process` and inject the product parts through a port.
- Change: move the neutral runner kernels into `🔨️modules/🏃️process`; keep the product parts in the library and inject them. Exact file list is in SDKB2 section 11.4-5 and FLEET 07:4x.
- Evidence: FLEET 07:4x ("Hand-offs to the parent: B1/B5 ... 519 edges"); SDKB2 section 11 items 4-5 ("untouched as instructed"); PSC2 section 9.
- Impact: 519 edges (about 520 repo-lib edges from plugin `📜️script.ts`). The 4 repo-runner imports of B1 and the store imports of B2 follow their owners (L-01 for the library side, G-09 for the store).

### L-02 Remaining cross-package edges in the library

- Description: the library's `no-cross-package-relative-*` edges, which the fleet skipped on purpose.
- Change: rewrite the library's own importers to package specifiers. Facades already exist (`@semio-tech/process`, `@semio-tech/schema-validator-ts`, `@semio-tech/framework`). Bootstrap files (`⚡️caching/🚀️bootstrap/🛠️tools`, package `nx-bootstrap-tools`) keep relative paths to the facades (PSC2 section 8). For the 7 production edges from the `sf3` area (`✏️s/🔌️plugins/🌀️procedural/**/📜️script.ts`) use the command in PSC3 section 7: `bun psc3-tools/rewrite.ts <plan> <scan> <policy> --area ✏️s/🔌️plugins/🌀️procedural/ --ignore-skip --write --snapshot <file>`, after `WITH_SKIPPED=1 psc3-tools/fresh.sh <label>` and `prep.sh`.
- Evidence: PSC2 section 8 (library 1,571 production, 796 test) and section 9.1; PSC3 section 7 (1,596 production, 802 test) and section 7 sf3 hand-off; PSC `RESULT-PACKAGE-SPECIFIER-CODEMOD.md` section 6 ("parent"); 137 `schema-validator-ts` edges came from new peer library edges (PSC section "resumed").
- Impact: about 1,596 production and 802 test library edges (not all are cross-package; the family total is what PSC3 measured).

### L-03 Repo-library type errors

- Description: 668 of 2,722 TS diagnostics (the 16:xx baseline) sit in the repo library, which the fleet does not own. Several are small and specific.
- Change: fix the named items. (a) `TaxonomyProgress` is undefined in a file of the library; (b) `NODE_ENV` missing from `ProcessEnv` fixtures; (c) `.mjs` declarations missing; (d) `scriptCompilerAdmissionControlV1` declared twice; (e) about 40 new `Expected 7-8 arguments, but got 6` in many `📜️script.ts` (peer drift since baseline).
- Evidence: TSCU section "Families" (family 1: repo library 668, parent-held) and "Hand-offs" (repo library owner); TTE1 section "Owner decisions" item 8.
- Impact: 668 diagnostics; the repo-library share of the TS total after TTE2 is not re-measured.

### L-04 Library test re-points still open

- Description: after the repo test and VS Code split removed `🧪️test/🟦️.ts`, two library tests and fixtures still name it.
- Change: (a) `📚️library/🧪️tests/🧭️fixture-placement/🟦️.ts` imports `inspectTestLayoutSources` from `🧪️test/🗺️test-layout/🟦️.ts` and `testTaxonomy` from `🧪️test/🗂️taxonomy/🟦️.ts`; (b) `📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts` (test "mutation vector discovery audits exact grouped owners") reads `"../../../🧪️test/📃️case-contract/🟦️.ts"`; (c) `📦️packages/🟦️typescript/📋️project.json` two nx inputs; (d) fixture row `🧫️fixtures/🦑️repo-source-ownership/🔣️.json` (owner = the deleted file, anchor now in `🗂️taxonomy`); (e) fixture row `🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` keyed by file and line 82. The three anchored edits were applied by the coordinator (`rtv-tools/apply-library-repoints.mjs --write`).
- Evidence: REPO-TEST section 7.1 and FLEET 06:1x.
- Impact: the two library tests load at 37 pass and 10 fail (peer fixture drift). Items (d) and (e) are still open.

### L-05 Taxonomy and registry rows held by the parent

- Description: new folders from this fleet's carves are not registered in the held taxonomy (`TAX`), and the generator registry has no entry for the new GraphQL generator. Verify first, because TAXD (16:2x) reports only 28 non-dot unresolved and TAXSWEEP-3 reports all folders born since 10-15 as resolved.
- Change (verify, then add rows): (a) ROOT-SCRIPT: `🎛️commands/{🔧️setup,🛠️dev,🏭️generate,🧹️code-style,🧪️test,✅️verify,🗄️stdio,🚀️release,📝️commit,🧩️os,📚️examples,🧬️schema}`, `✅️verify/{🚦️gates,🧟️generated-corruption,🧷️composed-child-refs,🚨️policy-breach-gates,🦀️rust-warnings}`, `⚖️laws/{🎯️tool-job-coverage,🔬️interactivity-audit,🧩️plugin-consistency,🧪️self-test-lanes}`, `🗄️stdio-ledger`; (b) OS-TS-CYCLES section 4: `📂️opening-config-mutation`, `🆔️identity-config-mutation`, `🗃️local-catalog-config-mutation`, `📁️local-folders-config-mutation`, `🏃️execution` (two places), `🗂️schema-index`, `📋️playground-report`; (c) REPO-TEST section 7.2: 32 test folder names, 13 VS Code folder names, `🔗️graphql-types` and five children, `🤖️generated/🔗️graphql-types`; (d) REPO-CRATE-ROOT section 7: 40 folders under `🌳️tree` and `🏃️test-runner`; (e) HUB-SCRIPT section 9 names; (f) RHU2 section 6 and RHU3 section 7.2 SDK facades (`🖼️assets`, `🥽️mesh`, `⏳️async/🪃️continuation`, `🖌️raster/🎥️video`, `🛠️tool-machine`, `⏯️tool-run`, `⏱️trace`, `🌱️value/💾️resident`, `🪢️wire-contracts`, `🧾️app-channel`, and others); (g) SDKB2 section 10. Generator contract: the `generatorContracts` row for the parser projection (PARSER-GENERATOR-PERMANENT, Hand-offs item 1). Generator registry: add `rtv-tools/generator-registry-entry.json` (`repo-vscode-graphql-types`).
- Evidence: as listed; TAXSWEEP-2 (slug-pattern rule landed), TAXSWEEP-3 (nothing to register), TAXD (1,049 rows, 64 renames).
- Impact: each missing row becomes a `directory-kind-unresolved` finding in the whole-repo inventory. Several listed rows may already exist; run `inventoryTaxonomy` before editing.

### L-06 `lib-wiring-budget` ratchet pins

- Description: two roots are pinned in the ratchet and both can now shrink. Three other roots breach the budget (energy viewer 512 vs pin 508; remodel vectors; simulation session 1620 vs pin 1619).
- Change: the ratchet fixture is `📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/` (owner = parent). Lower the pins to the new counts, or remove them once the remainder is 150 lines or fewer: viewer 508 to 394, simulation-session 1619 to 319. The viewer's remainder needs `E/👁️viewer/🦀️.rs` added to `POLICY_SPLIT_SOURCE_ROOTS` before a further cut (WIRING-BUDGET section 5). Pins may only shrink (the test enforces it).
- Evidence: WIRING-BUDGET-ROOTS section 5 (items 1-2); SMALL-FIXES-1 section 5 (energy viewer 508 to 512, simulation-session 1619 to 1620, remodel vectors); SHARED-FILES-3 section 7.4.
- Impact: 3 failing `lib-wiring-budget` cases. The fleet may not touch the ratchet.

### L-07 Storage lint adapter prefix list (verify location)

- Description: the lint `storage-outside-config-lane` names `🖥️platform/` as the only storage adapter. The new leaf `🗄️storage-port` holds `localStorage`.
- Change: add the new leaf to the adapter prefix list. The RESULT cites line 18361 of the root `📜️script.ts`, but ROOT-SCRIPT-SPLIT cut that file to 166 lines, so the list has moved into `📚️library/⚖️laws/` or elsewhere. Locate it before editing.
- Evidence: UIB section 7.5.
- Impact: none reported today (the scan skips index files); prevents a future false negative.

### L-08 Actor and Media edges (parent-owned, not peer)

- Description: 38 production edges of `renderer-hosts-only-ui` to the parent's Actor and Media modules. The RESULT assigns them to the parent; the brief lists them with the peers, so they are recorded here and cross-referenced at G-09.
- Change: run the ready spec. Command (RHU line 151): `SPEC=rhu-tools/handoff-facade-spec-actor-media.json PLAN=🗑️generated/rhu-plan-actor.json ORG=🗑️generated/rhu-origins-actor.json node rhu-tools/mkfacades.mjs` (dry run proven). Actor document binding (HO-3, 54 importers) is a plugin-fleet decision.
- Evidence: RHU section 6 ("Parent-owned Actor and Media", 38, "Not touched"); RHU3 section 6 ("38 parent Actor/Media edges"); RHU3 "remaining" list (127 + 38 + 8 + 12).
- Impact: 38 production renderer-host edges, of the 185 edges RHU3 lists as remaining outside the react artefact count.

### L-09 Library cycle components

- Description: library `no-circular` loops that the OS TS cycles slice found but did not cut.
- Change: library `⚡️caching` and `🏃️process` clusters (OS-TS-CYCLES section 7.3). Also components M and N (manifest types to a UI contract leaf; framework root barrel) per the OS-TS-CYCLES reference to `RESULT-UI-CYCLES.md` section 5 items A and E. That file is not in the 95-file set; verify it exists.
- Evidence: OS-TS-CYCLES section 7.3 (library loops 34 before, 32 after).
- Impact: 32 library loops (after the OS slice).

### L-10 ROOT-SCRIPT follow-ups in the library

- Description: three structural follow-ups the root split left open.
- Change: (a) `VerifyScript.run` is a 630-line `if` chain in a 1,539-line class (split by command); (b) `🔬️interactivity-audit/{🖥️mounted,🗄️database}` and `🧩️plugin-consistency/🚪️artifact-io` are 1,500 to 1,840 lines and can split along their `//#region` markers; (c) the root `📜️script.ts` lists 36 test imports it injects, one line each (the self-test lanes could come from a registry).
- Evidence: ROOT-SCRIPT-SPLIT section 9 (follow-ups, not done).
- Impact: maintainability; no edge count change.

---

## Group 3. Peer-owned work

### G-01 ui-contract: 145 UI-internal edges

- Description: 145 production and 9 test edges from outside `🧬️contract` and the other ui-internal modules into the ui core, which the codemod left because no facade exists.
- Change: a decision between two options. (a) Carve `🖱️ui/🧬️contract/📦️packages/🟦️typescript` into `@semio-tech/ui-contract-ts` with explicit exports for the five retained modules, the builder retirement fixture and the contract part of the fixtures (PSC3 section 8.1 "U1", measured: 99 edges from outside would use new specifiers); then `ui-core`. (b) Move the elements into `🎯️targets/⚛️react`. A ui TypeScript package with a deliberate public surface would add `ui-no-framework-packages` violations if it went through `@semio-tech/framework`, so it must not.
- Evidence: PSC2 section 9.1; PSC3 section 8.1; full list in `🗑️generated/psc3-ui-edges.tsv` (154 rows).
- Impact: 145 production and 9 test edges. Owner: ui-contract peer, user decision.

### G-02 ui aggregate: `input` and `events` items

- Description: 588 test-side uses name items that no producer exposes (43 items). 496 of them are `input` and `events`, which live in the `semio-framework-ui` aggregate's own modules.
- Change: create two producer crates, `semio-framework-ui-input` (the `📥️input` module is documented as 100 percent target-neutral) and `…-events`, and move the test uses to them (RTD section 3.4 and section "Reading"). The aggregate's `testkit` and the six `#![cfg(test)]` bins keep their normal edge only for the rest.
- Evidence: RTD section 3.4 (lines 87-103) and section 1-2 (`ui` dev row 66 to 4; 84 dev edges added to 21 producers).
- Impact: 496 of 588 test uses off the aggregate. Owner: ui aggregate peer.

### G-03 ui-contract-presentation compile error (blocks dependents)

- Description: `semio-framework-ui-contract-presentation` does not compile: `crate::UI_NUMBER_PRECISION_MAX` is not in scope at `🖼️presentation/🔢️number-format/🦀️.rs:50`. The constant is used by `♿️accessibility` and `🏗️builder` in the same family.
- Change: make the constant reachable from the presentation crate (move it into `🔢️number-format` or re-export it from the crate root). Owner: ui-contract peer.
- Evidence: COMPILE-PASS-2 section 3 and section 7 (foreign error, "To the ui-contract owner").
- Impact: that crate and every crate that depends on it are unreached in the compile pass.

### G-04 3D mesh carve: 7 unresolved names

- Description: the mesh carve left 7 names that the baseline got through `use super::*` from the old monolith root. Found by the round-2 oracle, family `dm2d`, both configurations.
- Change (exact, from `HANDOFF-3D-MESH-CARVE-IMPORTS.md`):
  1. `M3/🛠️modeling/🦀️.rs` line 12: `use std::collections::BinaryHeap;` becomes `use std::collections::{BinaryHeap, HashMap};` (fixes 2 rows).
  2. `M3/📄️document/🦀️.rs` line 590: `type Vec3f64 = (f64, f64, f64);` becomes `pub type Vec3f64 = (f64, f64, f64);` so the glob re-export carries it into the modeling file (fixes 4 rows; the alternative is a private alias in the modeling file).
  3. `M3/🛠️modeling/🔁️reconstruction/🦀️.rs` around line 270: insert `let point=self.snapshot.soup.positions[self.cursor];` as the first statement of the `else {` block, before the `let key=mesh_weld_key(...)` line, which must stay byte-identical (the clipping test pins it). Equivalent without a binding: `push(self.snapshot.soup.positions[self.cursor])`. This is a behaviour gap, not only an import (fixes 1 row).
- Evidence: `HANDOFF-3D-MESH-CARVE-IMPORTS.md` (rows 1-3, written 21:30 by kcf).
- Impact: 7 resolve rows in `dm2d`. Check: `python3 pos-tools/pos2batch.py --jobs 1 --label <l> --cfgs lib,test dm2d` expects `resolve: 0`. Owner: mesh-carve executor.

### G-05 Draw carve: remaining diagnostics and tests

- Description: draw owns three open items.
- Change: (a) 28 draw diagnostics remain after TTE2 (draw 231 to 28); the Binary64 rule is that conversion to `number` happens once, through `binary64Value`. (b) `layout-drawing/🪶️sqlite/🟦️.ts` has 1 failing test; its width table must match its `.sql` (the draw reader bug fixed at DRT section 1 used 6 columns since `font_family`). (c) `os-renderer-wgpu` lib-test reads `🖍️draw/🔧️pipeline/🦀️.rs` and `🖍️draw/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs`, which the ui draw carve split; the reader needs the new paths.
- Evidence: TTE2 section 1 (draw 231 to 28); DRT section 1 (layout sqlite test); MISSING-TEST-INPUTS section 2 row H and section 6.1; FLEET 19:0x.
- Impact: 28 tsc diagnostics; 1 runtime test; 1 Rust lib-test. Owner: draw peer.

### G-06 Scenes and EngineCanvas relocation (in flight)

- Description: the Scenes and EngineCanvas carve has not finished. It produces 414 test resolve rows and 5 privacy rows.
- Change: make `ink_gesture_open`, `ink_overrides`, `ink_marquee_points`, `drag` and `pointer_was_down` of `SceneSurfaceState` reachable from `scene-ink` and `scene-render` (they are `pub(crate)` in `scene-state`), and add `Paint2dCameraFields` to the `scene-tables/🧮️table-and-paint2d` import (E0425). The `cfg(test)` items used across crates belong to the test bins (`rtb`, done statically; verify).
- Evidence: RI3 section 7 (lines 113-116).
- Impact: 414 resolve rows and 5 privacy rows. Owner: renderer scenes peer.

### G-07 Residual edges in renderer TS and os flow

- Description: two residual `no-core-path` groups that are peer-owned.
- Change: (a) `🕸️NodeGraph/🧰️graph-core` (11 production edges) belongs to rhu2 (renderer TS); the RHU3 remaining list does not mention it, so verify it is gone. (b) os flow `🫀️core` (6 production edges, generated wasm bindings and workspace entries in 6 lockfiles) belongs to the os flow owner. Rename the folder without the banned stem, or accept it under P-08 (a stem in `bannedNameStems` is the cause).
- Evidence: HR2 section 5 (rows 11 and 6); HR2 section 6 (after-measure).
- Impact: 17 production `no-core-path` edges.

### G-08 Hub script: renamed export

- Description: `🌎️hub/📦️packages/🦀️rust/📜️script.ts` line 93 imports `startLocalSessionBroker`, which a peer renamed to `withLocalSessionBrokerV1`. The peer's test asserts the old export is gone, so the importer must follow.
- Change: at line 93 replace the import with `withLocalSessionBrokerV1` and update its call sites in the same file. Do not restore the old name.
- Evidence: REPO-HUB-RUST-FILE-SPLITS section "Foreign" (14:xx entry in FLEET); HUB-DIRECTORY-CAPABILITIES section 8 and section 9 item 5.
- Impact: blocks six hub checks (`share-issuance-atomicity-check`, `retained-short-admin-check`, `directory-event-page-v1-check`, `directory-command-receipt-check`, `invite-redemption-transaction-check`, `space-administration-check`) and the hub `os-hub` gates. The root hub script loads fine.

### G-09 Store owner: schemas, worker port and gis to os document service

- Description: store and os-product items that need the Store owner.
- Change: (a) B2: move `🪆️child`, `🔗️link` and `📦️blob` next to `🧬️schema/🗿️artifact-reference` and re-point the importers (82 edges; FLEET 07:4x). (b) B6 (12 edges, one cluster): the gis document-service ABI. Define an explicit worker port: the `DocumentServiceWorkerHostV1` host type and `DirectoryHttpError` currently come from the os store worker and the os product (SDKB2 section 11.1). (c) The installed-service declaration and wire contract (`OS/🔨️modules/💡️inference/🔌️service/{🟦️.ts,🚪️io/🟦️.ts,🧬️schema/🧺️value/🟦️.ts}` plus the `🌐️document-http` schema) move to `F/🔨️modules/💡️inference/🔌️service/` (four pure TypeScript files), about 12 os importers re-pointed. (d) `🎁️packs/🟦️.ts` (986 lines) to a neutral module for the 2 plugin edges (SDKB2 section 11.1). (e) `document-check-in-v1` (1 edge; its Rust twin stays in the os tree, DIR-LEFT section 7.1).
- Evidence: FLEET 07:4x; SDKB2 section 11 items 1, 4; DIR-LEFT section 7 items 1 and 3.
- Impact: 82 + 12 + 2 + 1 edges, and the gis B6 cluster. Owner: Store and os-product peer.

### G-10 Asset and JavaScript targets (31 edges)

- Description: 15 production and 16 test edges go to asset files and JavaScript targets that cannot be expressed as TypeScript facades.
- Change: a coordinator ruling already exists: a single-consumer file moves next to its consumer; a shared file gets an explicit `exports` entry at its owner (FLEET 20:4x). The ASSET-JS-EDGES slice (`aje`) is running and applies it. Decision left to the owners: direct exports of asset files, or move the files (PSC3 section 8.2).
- Evidence: PSC3 section 8.2; PSC2 section 9.2; FLEET 20:4x.
- Impact: 31 edges. Status: in progress (see F-09).

### G-11 wgpu renderer generated artifacts

- Description: the generated worker bundles and `catalogSha256` of `wgpu-frame-worker` embed kernel code and were already drifting. Six `package-integration` tests fail.
- Change: run `generate-wgpu` (the typegen target). Then check the `sourceModulePaths` allow-list that INTERPRETER-KERNEL-TS updated.
- Evidence: INTERPRETER-KERNEL-TS section 7 item 2 (line 151).
- Impact: 6 failing package-integration tests. Owner: wgpu renderer peer.

### G-12 `jco-package-adapter` input-pattern order

- Description: the order of `jco-package-adapter.inputPatterns` breaks taxonomy load (a peer edit).
- Change: fix the order of the input patterns in the `jco-package-adapter` entry, then re-run `sDevCompositionOwnershipV1` and the schema index check.
- Evidence: DIR-LEFT section 7 item 6.
- Impact: taxonomy load; a sequence of the dev-composition ownership checks.

### G-13 Plugin peers: test defects and retained-loader protocol

- Description: two plugin-area items that predate this fleet.
- Change: (a) the instance-dispatch tests (`test_reserved_emit_admission_case`, `_cancelled_worker_handoff`, `_busy_completion_owner`) call `admit_reserved_emit_producer` and `run_framework_reserved_job` with one argument fewer than the definitions (the `cx` parameter); fix the call sites. (b) The window-config retained loader (`plugin/🪟️window/🎚️config/📥️retained`) quotes its cursor with the old `retirement_demands()`; move it to `storage_retirement_demands` and `retire_storage`.
- Evidence: PLUGIN-WP7 section 8.4 (pre-existing defects, found and not fixed); OS-PACK-RECORD-CODEC-DEDUPE section 6 item 3.
- Impact: plugin test build; retirement counts for the window config. Owner: plugin peer (pwp7 and window owner).

### G-14 Missing implementations in stdio, cad, sourcing, process3d and text-editor

- Description: tests and imports call implementations that do not exist (TTE1 section "Owner decisions" item 6, and the stdio brep family). Not generated by any tool in the tree.
- Change: implement or export, per owner: `curationSnapshotToSqliteDatabase` and `curationSnapshotFromSqliteDatabase` (sourcing, 24 uses); `process3dSnapshotToSqliteDatabase` and `…FromSqliteDatabase`; `parseFilters` (sourcing config); `CadDislocateOptions` (cad config schema, 6 imports); `en1990` export; `isCanonicalArtifactKind`; `textEditorSyncPackV1`; `parseExtensionCargoManifest` and friends; the stdio brep family (about 50 diagnostics).
- Evidence: TTE1 section "Owner decisions and hand-offs", items 5-6; TSCU "Families" (stdio brep about 50).
- Impact: part of the 2,098 remaining tsc diagnostics (TTE2 result). Owners: stdio, cad, process3d and text-editor peers.

### G-15 Binary64 half migration and XML doctype model drift

- Description: two model-level defects.
- Change: (a) puzzle 5d snapshot is Binary64 (10 sites), its diff `🔣️.json` has no `Binary64Transport`, and the diff `🟦️.ts` is all `number`. Finish the migration per artifact (schema JSON, `🧬️schema/🟦️.ts`, diff guards, fixtures), and do not reintroduce `GuardNumber` for fields the JSON declares as `Binary64Transport`. (b) `GuardInteger` no longer enforces `minimum: 0` for `index`, `from` and `to` (the migration dropped its `bounds` parameter); restore it. (c) `checkValidConformance` and `snapshotWith` treat `snapshot.doc.doctype` as a raw string, but it is an `XmlDoctype` object (`name`, `externalId`, `declarations`); rewrite the rules against the structured type.
- Evidence: TTE1 section "Owner decisions" items 2-3; TSCU hand-offs (Binary64 owners); FLEET 17:5x and 19:0x (coordinator ruling: convert once through `binary64Value`).
- Impact: puzzle 5d and xml validation. Owners: puzzle 5d and xml peers.

---

## Group 4. User and Root decisions

### U-01 Image receipt lacks the `mountDirectory` claim (B1)

- Decision: how to republish `⚡️cache/rust/fixed/image.json`, which lacks the new required `mountDirectory` claim. Every launcher call and the retirement preview refuse before doing any work.
- Options (INFRA3 section 5 and COMPILE-RUNBOOK 2.1): (a) the peer ticket `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES` lands the creator and retire path that writes a conforming receipt, and Root authorises the sanctioned retire; or (b) the peer provides a sanctioned receipt-republish operation. Do not hand-edit `image.json`: the claim is the identity of an empty directory under a mounted image and cannot be read without a detach.
- Clear when: `bun iu3-tools/validate-receipt.ts` (from `🧰️framework`, read-only) no longer prints `/mountDirectory is required`.
- Evidence: INFRA3 sections 1 and 5; COMPILE-RUNBOOK section 2.1 (B1).
- Impact: blocks every compile and the retirement preview.
- Update 2026-10-17 14:30 (RESULT-COMPILE-ADMISSION-DIAG.md): the launcher now refuses in about 6 s with `Pending Native image publication must settle before ordinary admission`. Interrupted publication intents are at `.cache/b/t/.publication/intent.json` (08:43) and `.cache/b/d/.publication/intent.json` (11:27). Behind that refusal is the missing `mountDirectory` claim in the `image.json` of t, d, g, p and the Rust fixed image. Only the peer publication owner (MODULAR-MONOREPO) can settle these intents and republish the receipts; no sanctioned repo command exists for the live images. The fleet keeps every flip compile-ready: the batched check commands are in each RESULT file.

### U-02 Headroom short by 32 MB and Nx material in the image

- Decision: (a) authorise the sanctioned historical retirement, `⚖️cargo-retirement-historical-preview` and then `retire`, after U-01 clears. At 15:15 the image had 4,262,842,368 B free against 4,294,967,296 B required (short by 32,124,928 B). Nothing may be deleted or moved in the image, and the gate may not be bypassed (COMPILE-RUNBOOK 2.1 B2). (b) Decide where the peer's Nx material lives: at 07:00 the image held 5.4 GB of Nx material (graphs 2.1, tmp 1.5, native 1.0, artifacts 0.8) next to 5.9 GB of cargo. FLEET escalated this to the user; no answer is recorded.
- Note for the decider: the fleet made a one-line fix to the headroom script (`withNxToolingUseV1` call, INFRA3 section 2), so the preview gets past the control check. It did not run `retire`.
- Evidence: FLEET 07:00 (escalation); INFRA3 section 2 and section 5.2; COMPILE-PASS-2 section 6; COMPILE-RUNBOOK 2.1 B2.
- Impact: blocks every compile after U-01.

### U-03 `bun install` per workspace scope (approval)

- Decision: approve `bun install` in each scope. It downloads third-party packages and is the step that replaces about 60 hand-made `node_modules/@semio-tech/*` symlinks (absolute links, untracked) and the `node_modules/@semio-tech/infinite-world-r3f` link inside the dependency image.
- Why: 153 TS2307 and 67 TS7016 errors come from missing workspace packages (TSCU family 3; packages such as `@semio-tech/cad-js*`, `@semio-tech/stdio-*`, `@semio-tech/puzzle-*`, `@semio-tech/mit-bestand-praesentation-projektetage`). `bun install --frozen-lockfile --dry-run` passes in all six scopes (HR2 section 3).
- Evidence: TSCU section "Families" (family 3) and "Hand-offs" (workspace linking owner); PSC2 section 7 (`node_modules` links, untracked); PSC3 section 6 (three links); PSC section 8 and 9 (`bun install` creates the schema-ts link); RHU2 section 3 (`infinite-world-r3f`).
- Impact: about 220 tsc diagnostics, and the lock-consistency check for the psc packages.

### U-04 `@xmldom/xmldom` not installed (approval)

- Decision: approve installing `@xmldom/xmldom` (third-party). Without it, 6 draw SVG test files do not load (SVG export unit and write, SVG import document, path and transform, fill rule).
- Evidence: DRT section "Not done and hand-offs" item 2; TTE2 family note (`@xmldom/xmldom` 7 draw svg tests in TSCU family 3).
- Impact: 6 test files (7 tsc diagnostics).

### U-05 New third-party oracle libraries (approval)

- Decision: approve six third-party dependencies added to `🔒️dependencies.json` and to six lockfiles: `jszip`, `fast-xml-parser`, `fflate`, `mdast-util-to-markdown`, `crc-32`, `fast-deep-equal` (added by the repo's own freeze check, not by hand).
- Evidence: HR2 section 3 ("Six packages were not in the freeze baseline").
- Impact: dependency inventory; no runtime change.

### U-06 stdio `base` subset id rename campaign (alternative to P-09)

- Decision: rename the subset id `base` to `full` (and semio `base` to `wrap`) across manifests, rosters, Rust and TS, verified by a real `cargo check`. About 8,900 uses (Rust module paths `subsets::base::` about 3,000). The prepared tool is `hr2-tools/rename.ts` with `spec-stdio.json`; the revert spec is `spec-stdio-revert.json`. Name choice in the RESULT: `full` (SVG already calls its profile "Full") and `wrap` (4 letters keep the longest path at 240 bytes).
- Evidence: HR2 section 4 (options a and b).
- Impact: 493 production `no-core-path` edges. Choose this or P-09.

### U-07 `SemioModelSnapshotText` contract

- Decision: is the stdio `🧿️semio/…/🏛️model/…/📝️text/📸️snapshot` text contract a `string`, or the `{ preamble, body }` the parser returns? The test is excluded until decided.
- Evidence: TTE1 section "Owner decisions" item 5.
- Impact: one stdio snapshot test and the dwg text inferences that declare both types.

### U-08 Generated stdio diffs call parsers that no generator emits

- Decision: about 30 generated stdio diffs call parsers that do not exist (TS2304 and TS2552, runtime `ReferenceError`): dxf (`parseDxfGroupCode`, `parseDxfValue`, `parseDxfEntity`, `parseDxf…Diff`), html (`parseHtmlNodeDiff`, `parseHtmlChildAdded`, `parseNullableAttrValue`, `parseNullableString`), csv, tsv, epw, pdf, md and json record diffs, the semio anim types (`AnimTimeline`, `AnimChannel`, `AnimChannelDiff`) and `SemioGraphPropertiesDiff`. No generator in the tree emits them. Choose: hand-write them (they would not survive regeneration) or add the generator.
- Evidence: TTE1 section "Owner decisions" item 4.
- Impact: runtime ReferenceError in those diffs, and part of the tsc total.

### U-09 BREP kernel splits (WP-G): confirm no active boolean work

- Decision: confirm with the user that no BREP boolean investigation is active before any cut. Wave 7 planned the 5 Rust files of `🧊️3d/📐️brep` (10,432 lines), with `🛠️operations/🔀️boolean/🦀️.rs` (2,498 lines) kept out of the first batch. The user's memory records recent box-through-box boolean work.
- Evidence: WAVE7-CANDIDATES.md section WP-G (lines 160-173) and line 238 ("WP-G after the user confirms no active boolean work").
- Impact: 10,432 lines in five files, all of WP-G.

### U-10 Delete tracked debris (git rm)

- Decision: permission to `git rm` the 9 tracked class-(c) items and add ignore rules, or keep them and use P-10. Deletion needs the user's explicit permission.
- Evidence: TAXD section 6 (recommendation, not done).
- Impact: 9 unresolved taxonomy findings; repo hygiene.

### U-11 r3f package location

- Decision: keep `@semio-tech/infinite-world-r3f` under `os` and add the allow-list line (P-05), or move the package under `🖱️ui` (it imports only `framework`, `ui-react`, `ui-styling` and has no os importer; 19 renderer files and 2 stories). The move changes the workspace list, `📋️build-owners.json`, the lockfile and the dependency-image link.
- Evidence: RHU section 5 HO-4; RHU3 section 3.
- Impact: 14 edges either way.

---

## Group 5. Open items still owned by this fleet and not dispatched (or unverified)

### F-01 First compile window (blocked by U-01 and U-02)

- Description: the whole compile sequence is ready and waits for the launcher.
- Change: run COMPILE-RUNBOOK phases 0 to 10 (113 launcher invocations, plus Phase 10 generators and runtime checks), one call at a time under the fleet mutex. Phase 0 starts with the probe (section 2.2) and the lock refresh (section 2.4). The first command for the OS kernel: `python3 okeng-tools/waitcc.py lib1 okeng-tools/b2.crates` (OS-KERNEL-ENGINE section 7 item 1). The coordinator resolutions (COMPILE-RUNBOOK section 17) apply: `teaching-proctor` is a member of the standalone `🎓️teaching/Cargo.toml`, not the root.
- Evidence: COMPILE-RUNBOOK sections 2, 15, 17; COMPILE-WINDOW-CHECKLIST.md; COMPILE-PASS-2 section 9 (not done: root `--lib` 99 crates, `--tests`, wasm, `test`, hub, teaching, plugin workspaces).
- Impact: every Rust crate in the repo is unverified natively until this runs.

### F-02 Offline Cargo.lock and manifest refresh everywhere

- Description: the hand-edited manifests and locks need one offline refresh before the first `check`.
- Change: `update --workspace --offline [--manifest-path <ws>/Cargo.toml]` per workspace, as the checklist step 1 says. Known stale: 180 `Cargo.lock` files edited by hand (DUPLICATE-MOUNTS-2); about 150 plugin workspace locks stale until the update (SINGLE-MOUNT-AND-GATE section 6); 40 renderer lock packages stale (RENDERER-TEST-DEPS); plugin-host carve crates absent from every lock (PRIVACY-ORACLE-SWEEP-2 section "Hand-offs"); framework lock missing the space artifacts and the cells crate (PLUGIN-CRATE-HYGIENE section 5); hub `Cargo.lock` with 7 hand-added edges (HUB-DIRECTORY section 8 item 1); `os-channel` `Cargo.lock` entry for `io-control` (OS-TEST-COMPANIONS hand-off 2).
- Evidence: as listed.
- Impact: any `cargo check` fails on a stale lock until this runs.

### F-03 Stale tsconfig root lists

- Description: two generated tsconfig root lists name files that no longer exist, or drop roots.
- Change: regenerate `psc-tsconfig-after2.json` (os `🧵️backbone/🟦️.ts`) and `sdk2-tsconfig-after.json` (gis `🧪️tests/🎚️config/🟦️.ts`) before the next `tsc -p` comparison. Four roots dropped in `tte-tsconfig-psc-live.json` (listed in `tte-psc-dropped-roots.txt`): `…/🎚️config/🧬️schema/🧬️mutations/📂️opening-config-mutation`, `…/📌️set-default-app`, `…/🧹clear-default-app`, `…/🎲️board/🚪️io/📝️text/🎯️dag-input`.
- Evidence: TSCU section "Hand-offs" (stale tsconfig lists); TTE1 section "Owner decisions" item 9.
- Impact: the tsc comparison baseline is not comparable until regenerated.

### F-04 Facade re-pointing and the pi1 `[DEBUG]` lines

- Description: alias cleanup left two pieces.
- Change: (a) re-pointing the facade consumers of `framework/📦️packages/🦀️rust/🦀️.rs` (about 1,300 uses in seven crates) needs new manifest dependencies and lock updates (DEBUG-AND-ALIAS-CLEANUP section 5). The 82 facade-API and 9 macro-root alias lines were kept on purpose. (b) Delete the `eprintln!("[DEBUG] held …")` test narration (19 occurrences) in `os/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs` and its duplicates in `🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`. The owner `pi1` is done, so the fleet must do it.
- Evidence: DEBUG-AND-ALIAS-CLEANUP lines 51, 86, 119 and section 5.
- Impact: about 1,300 path uses; 19 temporary debug lines.

### F-05 TypeScript cycles not cut

- Description: `no-circular` loops left by the TS slices.
- Change: (a) the remaining worker-tree cycle `worker/🟦️.ts -> 🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts` (type import back): remove by moving the 130 module-private names that `registerBackboneParityTests` takes from the worker into a neutral module (a separate refactor). (b) OS-TS-CYCLES excluded loops I and J (in-source test blocks; `insource/📜️script.ts` handles them); loop H needs the same 130 names. (c) OS-TS-CYCLES section 7 item 4: the Rust mirror of `🧪️tests/🔧️config-mutation-parity` if the contract leaves get Rust twins.
- Evidence: TS-AUDIT-FIXES section "Hand-offs" and line 31; OS-TS-CYCLES section 7 items 2 and 4.
- Impact: 1 worker cycle; loops H, I, J. Not dispatched.

### F-06 Rust cross-technology couplings still open

- Description: deferred couplings from RUST-CROSS-COUPLINGS and CARGO-PLUGIN-DIRECTION.
- Change: (a) layout Rust crates to stdio drawing: 30 files, 11 crates (`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/**/Cargo.toml` lists 32 stdio member lines). Staged design in RUST-CROSS section 2; alternative: host the drawing standard in the framework (option 1, "framework formats module"). (b) the hub shipped-fleet test reads `SHIPPED_FLEET_SQLITE_OWNERS` from the stdio test-support crate, and one `include_str!` of a stdio fixture (7 cross-technology Rust includes left). (c) Three procedural core to os-product flow and playbook model crates need framework facade crates (CARGO-PLUGIN section 5).
- Evidence: RUST-CROSS-COUPLINGS section 7-8 and section 2; CARGO-PLUGIN-DIRECTION section 5 and section 9.
- Impact: 7 cross-technology includes; 3 strict violations.

### F-07 Privacy and oracle leftovers without a named owner

- Description: items from the privacy sweeps that the FLEET does not assign.
- Change: (a) the `ledger` API `+13` change (PRIVACY-ORACLE-SWEEP-2, open list); (b) `ephemeral` dev-dependencies (`🫧️ephemeral` test crate needs `semio_framework_async`, `_io_control`, `_job_worker`, `_job_worker_authority`; the manifest declares none); (c) the testkit feature-forwarding problem and the 12 E0603 rows of the renderer family (PRIVACY-ORACLE-SWEEP section "Open": partly fixed by RI3); (d) the `codec-factory` to `os-io` trait is a design item (KERNEL-CODEC-FACTORY, dispatched as `kcf`, see F-09).
- Evidence: PRIVACY-ORACLE-SWEEP-2 section "Hand-offs" and open list; PRIVACY-ORACLE-SWEEP section "Open"; RI3 section 7.
- Impact: unknown until the compile window; owners must be named.

### F-08 PLUGIN-INTEGRATION-2 (pi2) in progress

- Description: the goal of 0 oversized crates depends on this slice, which is not finished.
- Change: finish group G13 (`instance-extension`, `instance-render`, `instance-dispatch`), planned and passed only by the crates-only differential. Also: `instance_turns` mounts (pwp7-owned) block the tool-run and time-travel roots until `time-travel-turns` and `tool-run-turns` have landings (PLUGIN-INTEGRATION-1 section 8 item 1); `cancel_extension_inference` and `with_extension_inference_context` (4 rows) belong to the plugin-program runtime lift (PLUGIN-WP2 section 5); the last shared file `🔌️plugin/🧪️testing/🎟️caller/🦀️.rs` (SHARED-FILES-3 section 7 item 1) uses the `testkit` export shape of `dm2`.
- Evidence: PLUGIN-INTEGRATION-2 section 0 (status "IN PROGRESS"); PLUGIN-INTEGRATION-1 section 8; SHARED-FILES-3 section 7; FLEET 19:2x.
- Impact: the plugin facade (125 files lib in the 21:xx census) stays oversized until G13 lands.

### F-09 Running or unverified slices (no RESULT file yet)

- Description: slices with working files but no RESULT file in `F` at the time of audit (21:37 by file mtimes).
- Change: check each slice and get its RESULT before the coordinator closes the wave.
  - `aje` (ASSET-JS-EDGES): `aje-tools/` and `aje-tsc-before.txt` / `aje-tsc-after.txt` updated 21:31 to 21:37; running.
  - `pst` (PARSER-STRICTNESS): `pst-tools/` and `pst-bad-8.json` updated 21:37; running.
  - `rpv` (RENDERER-PRIVACY): `rpv-tools/` updated 21:32; running.
  - `kcf` (KERNEL-CODEC-FACTORY): `kcf-tools/` updated 21:30; it wrote `HANDOFF-3D-MESH-CARVE-IMPORTS.md` at 21:30 but no RESULT.
  - `pgr` (PLUGIN-GATES-REVERIFY): `pgr-*` logs in `🗑️generated/` but no `RESULT-PLUGIN-GATES-REVERIFY.md`. Status unknown.
  - `tfe` (TEST-FIXTURE-ENTRIES): work files last updated 18:28; no RESULT file; RHU3 section 7 item 6 says the RESULT was never written. Dormant or abandoned: verify.
  - An unattributed `rw2t-*` set in `🗑️generated/` (21:37). It is not in FLEET; attribute it or ignore it.
- Evidence: directory listing of `🗑️generated/` and `F/*-tools/`; FLEET dispatch list (last entries 21:0x).
- Impact: 4 running and 2 unverified slices. Their results decide several group 2 and 3 items (for example G-10, F-07 and L-03).

### F-10 Runtime checks never run

- Description: work that is statically verified but has no runtime or launcher result.
- Change: run these when a launcher or dev server is available: the React browser boot of the renderer with the new host registry (INTERPRETER-KERNEL-TS section 7); a puzzle2d live serve from the main session (MISPLACED-E2E section 7); `bun repo run @semio-tech/framework-os-shell-rs:typegen` and `schema-check` (RHU3 section 7 item 1); `bun repo run @semio-tech/repo-vscode:generate-graphql-types` and `…:test-graphql-types` (REPO-TEST section 8); `verify taxonomy`, `verify layering`, `workspaces-check` for the 30 new TS packages (PSC2 section 9 item 4); the `interactivity` lanes other than `apps` and `history-closure` (ROOT-SCRIPT section 9).
- Evidence: the sections cited.
- Impact: no current evidence for these behaviours.

### F-11 Pack and store residuals

- Description: small structural items in the pack and store carves.
- Change: (a) `os-store-space-schema`: seven mutation files use the derive paths `dsl::MutationLeaf` and `dsl::Mutations` with no `dsl` dependency in the manifest (OS-KERNEL-SQLITE-HOST section 6). The FLEET says this went to OS-KERNEL-ENGINE; verify it is fixed. (b) `os-store-engine` is 254,589 bytes (target 230,000, hard limit 262,144); the spill valve D3 was not used (OS-KERNEL-ENGINE section 7 item 2). (c) The text-pinned TS scanners (`okeng-tools/pins.py`, 34 needles in `🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts`) were not re-pointed (ENGINE section 7 item 3). (d) The derive root of `diff_text`, `diff_binary` and `CompositeMutation` still emits `::semio_framework_os_kernel::` (ENGINE section 7 item 4). (e) `pub use pack::*;` in os-pack stays for 100+ consumers (PACK-DEDUPE section 6 item 5); the four `pack_rt` adapters keep their generated names (item 6); `os.pack.scalar-witness` schema lives in the os tree though its Rust is in the framework (item 4; the schema owner decides). (f) The framework kernel `🎒️pack/⚠️error` exports (`ValueRefusalKind`, `PackTransportCategory`, `PagedRefusalMetadata`, `PagedAllocationMetadata`, `ValueError`) are missing (TTE1 section item 7): verify.
- Evidence: as cited.
- Impact: store engine size and pack consumers.

### F-12 Test paths and companions

- Description: small test residuals.
- Change: (a) three `⚖️retirement-law` test paths are over the 240-byte budget that the fleet used (SHARED-FILES-3 section 7 item 4); check them against the 256-character AGENTS limit and shorten if needed. (b) The `assert_eq!*(` syntax corruption: the coordinator fixed 8 sites (FLEET 19:1x), but SHARED-FILES-3 section 7 item 3 lists 1 site in `brep-topology/🧪️tests/🔬️unit/🦀️.rs` and 7 in `geometry/🧪️tests/🔬️unit/🦀️.rs`; verify those specific sites. (c) The os-config companion tests depend on os-kernel features `protocol-laws` and `mutation-testing`, which were not touched (OS-TEST-COMPANIONS section 9 item 3). (d) The `os-services` children can be lifted into crates later (OS-SERVICES-MCP-SPLITS section 6): optional, not dispatched.
- Evidence: as cited.
- Impact: path-length gate; test builds.

### F-13 Stale-count and ratchet items to re-measure after the running slices

- Description: counts the fleet reported before later changes.
- Change: after F-09 completes, re-run the four measurements that the fleet reported before its last edits: the whole-repo taxonomy inventory (`inventoryTaxonomy`, the 28 non-dot findings), the dependency cruiser over the 4 roots (for the group 1 counts), `tsc` for the 2,098 remaining diagnostics (families in L-03, G-14, G-15, U-03, U-08), and the census (`census.ts`) for oversized crates.
- Evidence: TAXD section 6; TTE2 result (2,098); FLEET 19:1x and 21:0x census notes.
- Impact: the counts in this document are as of the reports cited, not current.

---

## 6. Items checked and not carried over

- Teaching membership (COMPILE-RUNBOOK section 15 item 1): resolved by the coordinator (section 17 item 1).
- Binary64 conversion point (TTE1 item 1): resolved by the coordinator ruling (FLEET 17:5x; TTE2 applied it).
- QuizSession collaborator split (PACK-QUIZ-FILE-SPLITS): done by QUIZ-SESSION-DECOMPOSE.
- HubDirectory capability traits (HUB-DIRECTORY-CAPABILITIES): done. Its one follow-up (no aggregate supertrait) was declined by the RESULT.
- Generation-native template ownership (PLUGIN-CRATE-HYGIENE section 5): done by SHARED-FILES-3.
- Retirement copy-demand and protocol tests (RETIREMENT-ORACLE-TESTS, RETIREMENT-TESTS-2): 50 of 50 passing per RT2.

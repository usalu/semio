🧪️ Verification record — wave 5 (no CI workflow; layering enforced and shrinking)
Every line below is a command that was actually executed and its actual output.

────────────────────────────────────────────────────────────────────────────────
1. GITHUB ACTIONS REMOVED
   `.github/workflows/test.yml` deleted and unstaged. The gates it ran are all reachable
   as Nx targets and launch entries instead:
     workspace:test-discover · test-doctor · test-contract · test-oracle · test-subject ·
     test-parity · test-metrics · test-metrics-enforce · test-report ·
     verify-layering · verify-layering-write-baseline · clean-test · clean-coverage
   $ ls .github/workflows/   → gh-pages.yml  play-sites.yml  playwright.yml

2. THE FRAMEWORK NO LONGER DEPENDS ON `s`
   🧰️.../🧪️test/📇️registry/🔒️migration.json — DELETED. It listed six ✏️s owners.
     · per-owner migration status now lives with the owner, in that owner's own
       🧪️oracle/🔣️.json (`migrationStatus`), collected by discovery
     · the repo-wide shrink-only ratchet moved to /🔒️migration.json beside 🔒️dependencies.json
       and is keyed by AREA, so a deleted area simply surveys as zero and passes
     · the os-kernel blocker prose moved to this ticket, where it belongs

   $ grep -c "✏️s/|🌎️hub/|compose/" in the framework test domain's four core sources → 0
   Asserted by the self-test "the framework test domain's sources name no implementation area",
   which reads `areaLayers` from the taxonomy and so names no area itself.

3. THE RULE IS NOW ENFORCED, NOT ASSERTED
   New taxonomy vocabulary: `areaLayers` (framework | implementation | repo-wide),
   `repoWideFiles`, `layeringGeneratedInventories`, `layeringGeneratedBanners`.

   New library policy (domain-neutral, in 📚️library): `layeringReferences`,
   `layeringCounts`, `layeringBreaches`, `writeLayeringBaseline`.

   $ bun ./📜️script.ts verify layering
     45 authored repo-wide/framework file(s) reference an implementation area (465 reference(s))
     clean — no file references more implementation paths than its baseline
   Wired into `verify gate`, `workspace:verify-layering`, and two launch entries.
   Four library self-tests hold the ratchet, including "the baseline lists no file that is
   already clean" so entries must be removed as they reach zero.

4. THE NUMBER ACTUALLY WENT DOWN
     930  authored references at first measurement
    -123  bundled build output (🟨️frame-worker.js) — regenerated, cannot go stale;
          detected by bundler banner, so no build artifact is listed by path
    -342  POLICY_SEMANTIC_VOCABULARY_ALLOWLIST — 342 ✏️s paths that sat in the ROOT ROUTER
    ────
     465  remaining, all baselined and shrink-only

   The allowlist migration is the pattern for the rest:
     · entries moved to ✏️s/🔌️plugins/🔒️policy-allowlist.json — owned by the area they describe
     · the root rule now calls `policyDiscoveredAllowlist(root, "semantic-vocabulary")`, which
       merges every `🔒️policy-allowlist.json` in the tree by convention
     · deleting the area deletes its exemptions with it
   Proven value-identical against HEAD:
     HEAD literal entries: 342 · migrated file entries: 342 · identical set: True · identical order: True
   Proven actually consulted: the rule reports 596 vocabulary findings with the allowlist
   loaded and 567 with it emptied, so the data is reaching the rule.

────────────────────────────────────────────────────────────────────────────────
5. NOTHING REGRESSED
   $ bun test 🧪️test/…/🧪️index.test.ts     52 pass  0 fail  703 expect() calls
   $ bun test 📚️library/…/🧪️index.test.ts   174 pass, 20 fail — all 20 pre-existing
       (dependency-boundary, ui css tokens, playground ports; 22 before this wave)
   $ bun ./📜️script.ts test contract        0 high-priority breach(es)
   $ bun ./📜️script.ts test oracle quick    cases=11 executed=24 passed=24
   $ bun ./📜️script.ts test parity quick    cases=11 executed=49 passed=49 parity=37/37
   $ bun ./📜️script.ts verify dependencies  clean
   $ bun nx run workspace:verify-layering   Successfully ran target

────────────────────────────────────────────────────────────────────────────────
6. WHAT IS LEFT, HONESTLY — 465 references in 45 files

    150  🧰️.../🦑️repo/💻️client/⌨️cli/🧪️component_test.go   repo-CLI tests asserting on real
                                                            monorepo contents; should assert
                                                            against discovery, not literals
    138  📜️script.ts                                       ~18 leaf policy regions that are
                                                            plugin-architecture rules and belong
                                                            in ✏️s/🔌️plugins/📜️script.ts
     39  📚️library/…/🧪️index.test.ts                        library tests using real plugin paths
     36  💻️client/🧩️vscode/…/🧪️extension.test.ts            same shape
     18  💻️os/🗣️dsl/🧪️fixture-sweep/🦀️component.rs          sweeps plugin example fixtures
     84  spread over 40 further files, mostly 1–5 each

   The 138 in the root router are the substantive remainder. They need the shared policy
   toolkit (`🔧️PolicyFsScan`, ~219 lines) extracted into 📚️library first so an
   implementation's own 📜️script.ts can import it — the policy plugin already discovers such
   scripts, so once the toolkit is importable each rule moves as a unit. That extraction was
   NOT attempted here: the root script is being edited concurrently by another session, and a
   half-finished toolkit move would leave the tree broken. The ratchet guarantees the number
   cannot rise while it waits.

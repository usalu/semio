# Repo Source Preservation Audit — Repeat 691

## Scope and Restrictions

Immutable base `1bae6115bc56bf5e1b01212774dd76877a5f64fe`, HEAD `36fbe9da786b71ec7b6946a1fc1cbb4e9d4d25ec`, incoming stash `a09b7afdff001185403f0decb7cb0714a7bb839e`. Root and applicable repo/VSCode AGENTS files were read. Repo MCP was unavailable in the parent task. The existing ticket is retained. No Git/index/stash/checkout/worktree mutation, AGENTS edit, build, compiler, test, product runtime, installation, or dependency resolution was performed. Python was used only for source/blob comparison, guarded file editing and report generation.

The repo lane owns the repo product, root tests, and root script source. Cargo locks and global manifests remain with the parent. All source writes checked fresh bytes and moved a generated temporary file atomically into place. No source copies were retained in the ticket.

## Results

- 186 incoming paths were applied or already present: 184 fresh source/asset writes or removals and two paths already integrated by the parent.

- All 17 overlapping paths were reconciled. Fourteen had disjoint immutable three-way edits. VSCode command routing preserves HEAD GraphQL generation/check/test commands and incoming dashboard-only tests. The extension-host tests retain HEAD’s extracted module imports and incoming dashboard navigation assertions. The VSCode entry is reconciled through the extracted owners described below.

- All 45 root-script incoming hunks have explicit dispositions below. Every one of the 43 relocated source hunks has its complete incoming new content present in the destination after normalizing extraction-added `export` modifiers. The remaining import and export hunks are wired at the destinations. Root `📜️script.ts` remains the HEAD thin router.

- The 21 intentional modifications of HEAD-only repo owners are all recorded relocation destinations. The immutable inventory comparison found no other HEAD-only repo/root-test deviation, allowing Windows checkout line endings.

- All incoming repo/root-test paths retain their incoming blobs or deletions, with one intentional exception: the incoming dashboard-only test now reads HEAD’s extracted production owners and executes its command-registration source from the actual command owner. Its portable fixture, independent Babel oracle and TypeScript evidence remain intact.

- Static relative named-import inspection covered 89 modified/new TypeScript source paths and found no missing local import targets or named exports (generated output imports were excluded). Textual inspection of the eleven extension production owners found none of the dashboard fixture’s forbidden command or symbol names. These are source inspections, not passing tests, compiler results or runtime confirmation.

## Root Script Hunk Dispositions

The tool-job inventory retains Rust token-aware test-item masking, window publication lanes and authority resolution, exact static retained execution contracts and factory mount resolution. Publication retains the incoming one-turn dispatch, freshness, bounded ephemeral publication and maintenance ladder requirements. Artifact-store retains the incoming Raster mounted source evidence, detailed failure requirements, drawing and store ingress changes. Mutation outcome retains R17 host application, R18 per-row emission, inverse family reachability, and their shared inventory aggregation.

| Hunk (zero-based) | Incoming Base Header | Current Owner / Disposition |
| --- | --- | --- |
| 0 | `-5,4 +5,5` | Rust tokenizer imports are added to `⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts`. |
| 1 | `-335,5 +336,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts` |
| 2 | `-417,6 +418,6` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts` |
| 3 | `-666,5 +667,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 4 | `-715,31 +716,43` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 5 | `-840,5 +853,6` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 6 | `-846,5 +860,6` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 7 | `-853,12 +868,14` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 8 | `-870,4 +887,31` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 9 | `-952,5 +996,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 10 | `-993,8 +1037,51` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 11 | `-1010,13 +1097,15` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts` |
| 12 | `-1083,4 +1172,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts` |
| 13 | `-1457,10 +1547,19` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts` |
| 14 | `-1581,75 +1680,47` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts` |
| 15 | `-1668,7 +1739,7` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts` |
| 16 | `-1733,52 +1804,42` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts` |
| 17 | `-2395,353 +2456,149` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts` |
| 18 | `-2751,4 +2608,23` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts` |
| 19 | `-2758,183 +2634,20` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts` |
| 20 | `-3705,5 +3418,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🟦️.ts` |
| 21 | `-3727,5 +3440,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🟦️.ts` |
| 22 | `-6307,31 +6020,6` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` |
| 23 | `-6339,5 +6027,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` |
| 24 | `-6420,6 +6108,6` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` |
| 25 | `-16851,5 +16539,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚧️authority-bans/🟦️.ts` |
| 26 | `-20424,5 +20112,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 27 | `-20925,5 +20613,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 28 | `-20939,4 +20627,6` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 29 | `-20955,4 +20645,6` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 30 | `-20971,4 +20663,8` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 31 | `-21206,4 +20902,276` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 32 | `-21219,9 +21187,11` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 33 | `-21286,5 +21256,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 34 | `-21298,5 +21268,8` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 35 | `-21317,16 +21290,111` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 36 | `-21334,5 +21402,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 37 | `-21341,9 +21409,23` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 38 | `-21355,6 +21437,16` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 39 | `-21405,4 +21497,14` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 40 | `-21410,5 +21512,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 41 | `-21429,4 +21531,6` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` |
| 42 | `-25462,5 +25566,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🟦️.ts` |
| 43 | `-25472,5 +25576,5` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🟦️.ts` |
| 44 | `-25661,4 +25765,6` | The two new Raster functions are exported directly by `⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts`; the runner imports the source-evidence function there. |

New cross-owner connections include the canonical checkout-root `WORKSPACE_ROOT`, source reading and graph inspection, `toolJobFactoryModuleFile`, Rust token helpers, and `createRequire`. Root diff-only-law tests import the extracted mutation-outcome owner, including the new R17/R18 entry points, rather than retaining imports from the thin root router.

## VSCode Dissolution Dispositions

The incoming removal of IDE repository operations was applied to HEAD’s command, lens, URI resolution, data, constants, globals, utilities and file-analysis owners. The entry retains kit diagnostics and source navigation, and its extracted imports no longer refer to removed operation functions. Shell string execution is replaced with the incoming fixed `execFile` argv calls. `updateBundleCache` is exported from its existing owner and consumed by the filter-values loader. HEAD’s typed GraphQL query owner retains all unrelated operations; its actual typed `AnalyzeDocument` declaration and imports are removed. HEAD had already dissolved the former inline untyped document aliases.

| Hunk (zero-based) | Current Owner / Disposition |
| --- | --- |
| 0 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` |
| 1 | `🎼️utilities/🟦️.ts`: remove shell `exec` import, retain `execFile`. |
| 2 | `🎼️utilities/🟦️.ts`: remove `execAsync` binding and stale extracted consumer imports. |
| 3 | Former inline serialized `AnalyzeDocument` already dissolved by HEAD; no inline alias is revived. |
| 4 | `⌛️queries/🟦️.ts`: remove the current typed `AnalyzeDocument` and its `AnalyzeQuery` imports. |
| 5 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎞️constants/🟦️.ts` |
| 6 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⚙️data-models/🟦️.ts` |
| 7 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎩️globals/🟦️.ts` |
| 8 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎩️globals/🟦️.ts` |
| 9 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎼️utilities/🟦️.ts` |
| 10 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎼️utilities/🟦️.ts` |
| 11 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎼️utilities/🟦️.ts` |
| 12 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🏬️uri-resolution/🟦️.ts` |
| 13 | `🗺️file-analysis/🟦️.ts`: retain its ownership and update documentation to source-navigation/bundle caching. |
| 14 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🗺️file-analysis/🟦️.ts` |
| 15 | `🗺️file-analysis/🟦️.ts`: update its matching region name. |
| 16 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🌳️tree-providers/🟦️.ts` |
| 17 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🔎️code-lens/🟦️.ts` |
| 18 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🔎️code-lens/🟦️.ts` |
| 19 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🔎️code-lens/🟦️.ts` |
| 20 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⌨️commands/🟦️.ts` |
| 21 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⌨️commands/🟦️.ts` |
| 22 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⌨️commands/🟦️.ts` |
| 23 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🌳️tree-providers/🟦️.ts` |
| 24 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` |
| 25 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` |
| 26 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` |
| 27 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` |
| 28 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` |

The incoming dashboard-only proof now inspects each of the eleven extracted production owners separately, so independent parsers see proper modules. Retained registration assertions inspect `⌨️commands`. Its registration execution extracts the exported `registerCommands` declaration from that owner and strips only its module export modifier before the existing VM harness. No tests were run.

## Overlap Ledger

| Path | Disposition |
| --- | --- |
| `🧪️tests/🧪️diff-only-law-gate/🟦️.ts` | Both source changes retained; import statement reconciled directly against extracted mutation owner and includes R17/R18. |
| `🧪️tests/🧪️diff-only-law-gate/🫧️lanes/🟦️.ts` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📋️project.json` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts` | Both test commands retained; incoming dashboard-only test invoked before existing level and GraphQL tests. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` | Incoming behavior and removals relocated into HEAD owners; 29-hunk table above. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts` | HEAD extracted imports plus incoming dashboard fixture import; incoming test removals, retained navigation and ownership assertions preserved. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts` | Both disjoint immutable edit sets retained. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts` | Both disjoint immutable edit sets retained. |

## File Manifest and Remaining Parent Work

The complete 224-path lane manifest is retained in [audit-repo-files-repeat-691.md](audit-repo-files-repeat-691.md). Its incoming-path statuses include applied removals. Relocation ledgers and static inspection artifacts are generated output under the existing ticket’s generated folder and can be removed after parent integration. The retained Markdown reports must remain. Parent must perform global inventory/metadata reconciliation and ticket closure, including explaining unavailable MCP closure if still unavailable. No source conflicts remain assigned to this lane; compilation and runtime correctness remain unverified by explicit task instruction.

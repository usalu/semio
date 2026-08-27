# WGPU Current Source Drift Audit — 2026-08-27

## Decision

The coordinator-reviewed bounded refresh was installed at `2026-08-27T14:18Z`: six source hash/size scalars, their paired catalog pin and one exact runtime browser module in both profile and input patterns. The identical current-source check changed from 24 passed / 5 failed to **29 passed / 0 failed**. Explicit workspace taxonomy loading also validates cleanly. No production WGPU source, generated output or physical package layout changed. The earlier observations and superseded proposals below remain historical evidence, not current pins.

The mandatory `nested-cargo-generation-unresolved` guard is restored. Native/wasm compiler acceptance, current-source canonical tests, complete reference/generator planning and a successful fresh lifecycle remain required. The dated old-source lifecycle did not pass its final empty-plan assertion.

## Exact Rust Preimages

Observation began at `2026-08-27T13:54:30.877Z`. All 32 source paths were no-follow regular files; the three changed leaves had mode `0644`. The exact Git command used `--literal-pathspecs ls-files --cached --others --exclude-standard -z -- <WGPU source root>`; its sorted set matched all 32 catalog source identities exactly. There were no newly admitted or missing source leaves.

Source root: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu`.

Canonical owner: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu`.

| Mapping index | Source leaf | Unchanged canonical suffix | Old bytes | Current bytes |
| --- | --- | --- | ---: | ---: |
| 9 | `📦️glue.rs` | `🧊️renderer/🦀️.rs` | 828737 | 829356 |
| 20 | `🦀️frame_job.rs` | `🧵️frame-job/🦀️.rs` | 34039 | 33880 |
| 26 | `🦀️surface_lane.rs` | `📐️surface-lane/🦀️.rs` | 22434 | 22438 |

Exact old → current SHA-256:

- Mapping 9: `b4869021b74b86466fbeef3798547bb2cb40381482f03d47f8fd14acf00a7790` → `c875acbf51bb02ba738268e7688e1d4eba903ab3ebd22398db8f98b8848fe1bc`.
- Mapping 20: `4c0602df7cab3a8a912605a221a730006cb613d07b208060556fd76ae195d23f` → `3b4de519a08cc059b4a0b1d07673ad7f41eee65b23f139eec451bfe60703e0fa`.
- Mapping 26: `d6131ff4d56d636a30ebe586e698449799e8391a49a240cb8df9c4b9e2daf3e7` → `88c7e79a91ba128b22777c3464e93005f283e9d9ad49878ac8eccbcac8036ee3`.

The complete bounded `diff -u` comparisons were read against the exact retained HT3ZIM sources before that fixture later disappeared. The earlier incomplete `now_ms`/microsecond-field mixture is no longer the current source:

- Frame jobs now use `now_us() -> Option<u64>`, the shared real microsecond clock, `step_budget_us`, `INTERACTIVE_LANE_WALL_US`, and checked `StepBudget::from_duration`. Unavailable/overflowing clocks select zero fuel rather than a fabricated deadline.
- Surface resize declares `SURFACE_RESIZE_STEP_BUDGET_US = 1_000`, supplies the microsecond clock to sessions and tests, and retains the existing source-to-semantic mapping.
- Renderer glue updates native I/O, replay/progress/close contexts and background preparation to the current microsecond job API. Its one coarse native HTTP `deadline_ms` remains deliberately millisecond-based, but now unwraps the optional real clock through checked addition and an error path.

Direct inspection of `🧰️framework/🔨️modules/🧵️job/🦀️component.rs` confirmed `BatchDriveConfig.step_budget_us`, `BatchJobParams.now_us: fn() -> Option<u64>`, `StepContext::new`'s matching clock type, checked `StepBudget::from_duration`, and 1000/4000 µs interactive/maintenance constants. This is API/source inspection, not a full renderer type-check or runtime claim.

## Exact Catalog Proposal

The physical catalog remained:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json`

SHA-256: `f64f2f654e5dc12d57b72148b023b2c9a20d6fd8f147293dac871e2529b2319a`.

At `2026-08-27T13:58:48.806Z`, an in-memory JSONC edit changed only `sourceHash` and `sourceSize` at `/packages/0/mappings/{9,20,26}`. Independent `jsonc-parser` edit/parse results equalled the six-scalar candidate object. The byte-preserving candidate digest is:

`38295d9653c67e398b1ef732a16be05e1ddb98ccb1531ea07fe7c729bb0902c4`.

Running the existing pure `semanticPackageProjectionAuthority` over the current physical source bytes and that candidate returned no problems and all 32 mappings. Source digest: `036e291b6e1ce9f302d7d75029c9dda04fb2093a405d5e10dbddcf010e86124d`. All exact canonical mapping/adapter/derived/output destinations were absent. Package identities, semantic destinations, role classifications, source splices, authored fragments, joined-path bindings and retirement declarations did not change. The physical catalog hash was rechecked unchanged afterward.

An accepted refresh must update the six exact scalar values and the taxonomy's composed catalog pin together. It must not rewrite the immutable historical structural/purity catalogs, suppress source mismatch diagnostics, or infer hashes for unavailable old fixtures. Current source hashes must be rechecked immediately before installing any proposal.

## Browser Graph Drift

Of the dated 33 external browser input files, two had changed at observation:

| Input | Dated bytes | Current bytes | Scope of observed diff |
| --- | ---: | ---: | --- |
| `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️component.ts` | 17406 | 19519 | Additional inline Vitest cases and independent test oracle correction |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️component.ts` | 34288 | 31094 | Shared graph-validation extraction, explicit frontier and close delegation |

Lifetime SHA-256: `20392134842570e65ff6dc9152acf3f9b9afba734e1303f6e75e5a65d1ee7183` → `9dd3b71b9fb7e1d2214df92723856afb60718a7f98ae9e41bb2dd57acdae9fbf`.

Retained UI SHA-256: `0a1d622fc15df477fb2a772065bd1bb22fda1be643b0f3498f8865a37737b5d7` → `84995a9641271d804c6b29d3a7cff0f6400de476a5818507ade6401f24764db8`.

The sole additional runtime module is:

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️component.ts`

Its SHA-256 is `be54a59db1c254b1a76e1b2e4d78259ba901689a9d59f9f82db33321f8924a99`, 5033 bytes. Independent TypeScript AST inspection found three imports, all explicitly type-only. It exports the runtime graph traversal and close-frame functions used by the retained owner; no additional runtime import closure was needed.

The unchanged 36-module profile still rejects current compilation. A read-only probe at `2026-08-27T13:58:00.358Z` used an explicit in-memory 37-module proposal, not a schema mutation. The real Bun producer read exactly 37 modules plus three workspace manifests, produced two bundles, and repeated byte-identically. Independent esbuild compilation produced one clean output per entry and exactly the same 37-module read set. All captured input hashes stayed unchanged during the probe, and all six physical production output states stayed unchanged. No output was written.

Current Bun proposal output identities:

- Ignored boot: SHA-256 `7226ea5041c0070626eb9ad768b315d90884df4f4102d71c10ae584723c17a9c`, 40092 bytes, `0644`.
- Tracked frame worker: SHA-256 `3d99cb1553d9a835fafa63b4c0f9a0f2b9e3fd780e0565ca9c620705b107ed48`, 350475 bytes, `0644`.

The compiler/read-set result is not execution of browser application behavior and does not claim Bun/esbuild output bytes are identical to each other. The independent comparison is their exact input closure and clean compilation; Bun determinism is compared against its own repeated output.

The precise schema proposal adds that one path to both `generatorContracts.wgpu-frame-worker.packageGeneration.browserProfile.sourceModulePaths` (byte-order index 17; 36 → 37 modules) and its `inputPatterns` (currently absent; 106 → 107 patterns). Updating only the browser profile would omit the new module from transaction input authority. The corresponding language-neutral browser profile/contract input must be reviewed together. The existing tracked/ignored output roots and inclusion kinds do not change.

## Lifecycle Evidence and Missing Inputs

The recovered final dated-snapshot run was **0 passed, 1 failed, 103 assertions, 130.28 s**, exceeding the unchanged 120-second body bound. The final plan had no moves, retirements, edits or regenerations, but one unresolved row. Its text was not printed. The journal committed and the prior staged/first-edit rejection and Nx checks reached their assertions; these do not turn the failed final assertion into a pass.

The requested bounded canonical-owner replan could not start: the exact committed `6LbGtY` root, retained plan and journal parent were `ENOENT`. `XDLdXS` disappeared too, followed by the authored `🧪️wgpu-generated-retirement.test.ts`. This lane issued no deletion. Direct ticket listing still showed the Markdown and JSON input manifests, but not the test file or retained fixture directories. The disappearance means the old fixture's unknown unresolved row and source-input provenance cannot currently be rederived. No substitute baseline, journal or plan was fabricated.

The coordinator checked the exact lost test path and found it was not Git-index tracked. Git recoverability is therefore not claimed. No other active fleet reported deleting these inputs at the coordinator's checkpoint; the cause remains unknown.

## Required Next Evidence

1. Resolve missing authored tests/fixture inputs with the coordinator; preserve any recoverable provenance.
2. The bounded six-scalar/pin and exact browser-module/input-pattern refresh is complete; see the final installation evidence below.
3. Run current-source native/wasm and canonical semantic test routes, complete projected generator/reference closure and exact output/input membership negatives.
4. Run rollback → staged interruption → first-edit interruption → commit → fresh empty plan as separately observable bounded phases, without concealing the previous 120-second failure or weakening reference/byte guards.

Through the initial read-only audit, no production WGPU apply, generated output edit, schema/catalog write or real Git mutation was performed. The subsequently reviewed narrow schema/catalog installation is separately recorded below.

## Superseding Source Observation — 14:07–14:12 UTC

The approved `38295d…` candidate above was **not installed**. Before the first authority write, the new bounded current-source check detected concurrent changes to mappings 9 and 20. The coordinator confirmed that a separate retained-child lane is finishing this exact callback/session watchdog consumer. Pins remain withheld until that lane provides a stable boundary and another complete 32-file recensus agrees.

At `2026-08-27T14:11:08Z`, the current values were:

| Mapping | SHA-256 | Bytes | Mode |
| --- | --- | ---: | --- |
| 9, `📦️glue.rs` | `214c5ece5918ed0c3255828da5ac0f9441ddc164b7b2efa88cd879b5f6c01c28` | 829553 | `0644` |
| 20, `🦀️frame_job.rs` | `ced742a20cb55b9b119b2371c0ec2ae0d20e31805ef1daf0af335541c556a0b3` | 33418 | `0644` |
| 26, `🦀️surface_lane.rs` | `88c7e79a91ba128b22777c3464e93005f283e9d9ad49878ac8eccbcac8036ee3` | 22438 | `0644` |

The separate source lane subsequently reported a stable boundary with those glue/frame-job hashes. A fresh full census at `2026-08-27T14:15:21.242Z` independently reproduced all three values, all 32 regular `0644` leaves and the same exact source path set. Only the six documented scalar values differed; every other mapping preimage remained equal. The candidate catalog/source digests below were reproduced unchanged. Coordinator tuple review still precedes installation.

No-follow source reads and the exact Git-admitted 32-path census still agreed with the catalog source identities. The independent in-memory six-scalar proposal at `2026-08-27T14:11:55.881Z` produced catalog SHA-256 `a640e9b8e5154f0cb6276931d55564c8284b6ff45f10b366d089831d431673cf`; pure source authority returned `problems: []`, 32 mappings and source digest `b5d395e91607ed725de7b7cb223e292266a59f3f9d5f544f8a4fe668801bc23c`. All canonical mapping/adapter/derived/output destinations remained absent. This is a superseding **proposal**, not a published pin or stable-source claim.

Read-only `git show :<exact-path>` confirmed that the index contained the exact previously reviewed `c875ac…`/829356 and `3b4de5…`/33880 inputs. Therefore the two exact-path `git diff --no-ext-diff` outputs isolated the new delta without inventing a historical baseline:

- Glue adds only a 197-byte `AppFramePreparation::callback_verdict` accessor, delegating through its optional `BatchJobSession` to the exact checked-out callback verdict.
- Frame jobs remove the shared watchdog violation-count/backward-search helper. Deadline jobs inspect the session's exact verdict only after outcome checkout. Apply-pending and transaction-build stages check watchdog admission before work and consume the same watchdog's `finish()` result afterward. Preparation inspects its exact session verdict. Existing quarantine and completed-frame retention behavior remains in place; no module/path binding, role classification, units or canonical operand changed.
- Direct job API inspection confirms the callback verdict is recorded by the worker outcome and exposed from the checked-out session. Direct trace API inspection located `CallbackVerdict::is_fault`, `Watchdog::is_admitted` and consuming `finish` in `🧰️framework/🔨️modules/⏱️trace/🦀️component.rs`. This remains source/API review, not native compiler or runtime acceptance.

### Bounded Check Input and Red Results

The reproducible, language-neutral input and its Bun/Nx runner are retained in `🧪️wgpu-current-source-authority/{🔣️.json,📜️script.ts,package.json,project.json,nx.json}`. The input still records the prior reviewed tuple so that the newly detected drift is visible rather than silently repinned.

Exact command, with that semantic folder as the working directory:

```sh
NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun '/Users/ueli/Documents/semio/node_modules/nx/bin/nx.js' run '@fixture/wgpu-current-source-authority:check' --skip-nx-cache
```

The first run returned **23 passed / 6 failed**, body 417.340 ms, exit 1. One failure was a harness-only esbuild Go-regexp incompatibility with the JavaScript Unicode flag. Changing only its plugin filters from `/.*/u` to `/.*/` exposed the intended compiler closure failure; no vector or authority check was weakened.

The corrected second run returned **23 passed / 6 failed**, body 405.611 ms, exit 1. The six failures were exact current-source-vs-reviewed hash drift, unpublished catalog/pin, strict published source-preimage rejection, unpublished browser-profile/vector addition, Bun's missing declared module, and independent esbuild's matching unadmitted `./🛡️validation/🔬️graph/🟦️component.ts` import. The source-negative rows are not yet acceptance evidence against a valid refreshed baseline while this vector is stale; they must be rerun after the reviewed tuple is current. Profile/opaque-path, browser missing/symlink/unknown-input and cancellation cases remained present. The read-only boundary check confirmed no production output or catalog change and the mandatory completeness guard remained installed.

## Approved Installation and Final Gate — 14:18 UTC

After the coordinator independently reviewed the stable callback-verdict boundary, the language-neutral check input was updated to the exact three current preimages above and catalog candidate `a640e9…`. Each source-negative case now first proves that its unmodified in-memory candidate is valid, preventing unrelated stale preimages from making a negative appear successful.

With those final authored inputs and the old production authority still present, the same named Nx target returned **24 passed / 5 failed**, body **870.608 ms**, exit 1. The five expected failures were unpublished catalog/pin, published source-preimage rejection, unpublished profile/vector, Bun closure and esbuild closure. All seven source negatives, seven profile negatives and seven browser/cancellation negatives were present and passed against their valid baselines.

Immediately before writing, at `2026-08-27T14:18:00.470Z`, the complete 32-path no-follow census, every captured source hash/size/mode, old catalog digest, old schema pin, old 36-module profile and old 106-pattern count were checked again. The current source digest remained `b5d395e91607ed725de7b7cb223e292266a59f3f9d5f544f8a4fe668801bc23c`. The patch changed exactly:

- Catalog `/packages/0/mappings/{9,20,26}/{sourceHash,sourceSize}`: the six reviewed scalars only.
- Taxonomy `/semanticPackageProjectionContracts/nested-cargo-packages-v1/authorityCatalogSha256`: the exact composed digest.
- Taxonomy WGPU generator browser `sourceModulePaths` index 17 and `inputPatterns` index 24: one insertion of the reviewed graph-module path in each, retaining the existing source entry, workspace import, output and inclusion contracts.
- Active authored ticket inputs `🔣️wgpu-browser-profile.json` and `🔣️wgpu-package-generator-contract.json`: the identical insertion(s).

Rerunning the **identical named Nx command and authored runner** returned **29 passed / 0 failed**, body **724.292 ms**, exit 0. The published pure source authority accepted all 32 mappings. Actual Bun bundling read exactly 37 modules plus three manifests, produced two in-memory bundles and repeated byte-identically. Independent esbuild cleanly compiled both entries with exactly the same 37-module input set. All 21 negatives passed, including missing/wrong-kind/unknown source leaves, content/manifest/preimage drift, opaque/escaping/duplicate profile paths, unowned imports, missing/symlink browser modules and cancellation at three stages. Literal `temp-compose` remains distinct from the two actual opaque roots. No actual opaque-root filesystem traversal was required by this check.

Nx labelled the target flaky because the same task intentionally returned red before the authority patch and green afterward. No retry or assertion was suppressed. At `2026-08-27T14:18:54.740Z`, a separate fresh process executed actual `loadTaxonomy()` and `validateTaxonomy()`, returning `[]` in **275.146 ms**. This includes the strict workspace output-phase check, not merely the catalog-only parser.

The exact raw pre-write schema/vector bytes were retained in tool memory. Post-write comparison proved the entire schema and active vectors differed only by the approved pin and path insertions; independent JSONC edit/parse and reconstruction checks proved the catalog differed only by the six scalar values.

### Exact Installed File Identities

Library prefix: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

| File | SHA-256 | Bytes |
| --- | --- | ---: |
| Library `🔣️taxonomy.json` | `64a11e3b97c9b21c26093d82d017ef8478f2ef0af7dd4fe383ca455d08a25945` | 308452 |
| Library `📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json` | `a640e9b8e5154f0cb6276931d55564c8284b6ff45f10b366d089831d431673cf` | 73336 |
| Ticket `🔣️wgpu-browser-profile.json` | `d3e4b1bbdc80d883101784f80ed21c119e010130d68110b7046eb305af1ec377` | 5478 |
| Ticket `🔣️wgpu-package-generator-contract.json` | `1b3c9a06520db398be0e442eacdcf66f6117ce95b248fb6da4f8f0282800d692` | 23772 |
| Check `🔣️.json` | `a2bb33e9d637ac8398b756bf81bcab607a876030f3bda6b25a5aabf0b527288c` | 2134 |
| Check `📜️script.ts` | `b0c31b2a229f60abb16c16bd786c3f68fff41661079eccb38971e9a54c4356f1` | 13643 |
| Check `package.json` | `174d1cfa6ad7c0ca00c47eb1d8955e78d05135e971380796629d07a8d49e251d` | 189 |
| Check `project.json` | `cd73a85bfce9cf3f8fba1c57e8fadaf48470c037d5f89493935dca1e93b22063` | 230 |
| Check `nx.json` | `c890c6016bbe2a14305a2398ece26e688631c17b68c0377044acb8eab2bf5f7f` | 28 |
| Check `.vscode/launch.json` | `3a47bed3186a82e7b6b23bef30f7df0538d9032c0a38ab3c4a7e00253d6ea2ec` | 509 |

The prior schema was `0cc60034727ec4c9d18c1e0a565d72f18594263f7ff2e080a885c13b5d4a42c5`, 308176 bytes. The prior composed catalog remains documented above. No historical structural/purity catalog was edited.

The check's local launch configuration follows the existing taxonomy gate naming/group/order and invokes its named Nx target; open `🧪️wgpu-current-source-authority` as the VS Code folder to use it. Its relative Nx route was resolved and checked against this repository's real Nx entry. All six authored check inputs and Markdown reports remain. After every check process exited, seven individually enumerated generated cache/project-graph/database files were removed from only this newly created check folder's `.nx` directory. They are reproducible by rerunning the target. No other fixture or recovery artifact was deleted.

Schema/catalog ownership is released after this coherent green boundary. The mandatory `nested-cargo-generation-unresolved` guard remains installed. No WGPU lifecycle was run, no lost historical data was recreated, no live output was generated, no WGPU move was applied and no real Git mutation was made. Full WGPU transaction readiness is still unproved.

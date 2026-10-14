# Graph Read Authorities

Read-only source audit on 2026-10-14. No graph was regenerated and no runtime claim is made. Paths below are relative to the repository. `library` means `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`; `inference` means `library/🏗️build/🕸️graph/📥️inference/🟨️.mjs`. Line numbers reflect the inspected source and may move during concurrent work.

## Decision Boundary

Use workspace path membership, real-file/directory/symlink kinds, and mode as structural authority, together with actual content observations across the entire discovery operation. Preserve negative existence, resolution and directory observations. A file appearing in a target input glob does not by itself make its bytes graph-discovery authority.

All included JavaScript and TypeScript implementation bytes currently are graph authority: `collectImportEdges` (inference:1607–1620) processes every project-file-map path matching `/\.[cm]?[jt]sx?$/`, including workspace catchall files and tests. It publishes SHA-256 of the complete bytes in `metadata.semio.moduleGraph.files[].hash` (metadata:1625–1629). A change only inside a test assertion therefore changes the published graph. Omitting TS test body observations would be incorrect for the present graph contract.

Rust is different. Reachable Rust bodies are read to infer source topology, but the graph publishes a path closure rather than a Rust body digest. A body change that preserves parsed references need not change graph output semantically; exact observed-byte authority remains conservative and valid.

## Consumed Content

| Source kind or pattern | Why discovery consumes bytes |
| --- | --- |
| `**/📋️project.json`, `**/Cargo.toml`, relevant `package.json`, `go.mod` | Project identity, targets, compiler/package identity, workspace and path dependency topology, package/import dependency edges. Cargo ancestry and dependency manifests matter, including dev dependencies in test/source preparations. |
| Root `bun.lock` and its declared patch files | External dependency graph and patch digests (inference:249–258). |
| Included `**/*.[cm]?[jt]sx?` | Full-byte hash, imports and production imports published in module metadata (inference:1607–1629). Includes test implementation bodies. |
| Reachable `.rs` entrypoints and modules | `cargoSourceInputs` / `rustSourceFiles` parse `mod`, inline modules, `#[path]`, and `include*` expressions (inference:405–453). Entry membership includes lib, main, explicit bins/tests/benches/examples and discovered directory entrypoints. |
| Literal script entrypoint closures | `relativeScriptInputs` parses imports/requires and recursively resolves actual command modules (inference:461–496). It may read JSON modules and imported test modules. `genericTargetCommandInputs` also parses closure modules for external package dependencies (533–554). |
| `📜️script.ts` contents | Root router AST registers targets (1389); wasm/go tooling detection inspects source (878,953); policy inference raw-reads every candidate script. All these scripts also receive module metadata hashes. |
| Declared `metadata.sourceInputs` contracts | Reads explicitly named JSON contract, follows its literal runtime import closure, checks generated boundary files and external declarations (inference:886–896; `library/🕸️dependencies/🟦️typescript/🟨️.mjs`). |
| Declared `metadata.printCatalog` | Reads named catalog JSON; stats listed document/library source paths; follows compiler script closure (inference:1054–1080). Document text is only an execution input here. |
| Ancestor `🔮️oracles/🔣️.json` | Oracle host package contributions and adapter-specific dependencies (`🧪️test/🕸️dependencies/🟨️.mjs`:36–48). |
| Catalog provider code and declared input files | Provider source is explicitly read before dynamic import; provider `inputs` and owner project are explicitly observed after provider completion (inference:1439–1444). |
| Playground/native-host manifests and scripts | `nativeHostSourceFactsV1` reads exactly host `Cargo.toml`, `📋️project.json`, `📜️script.ts`, with real ancestry/kind checks (`library/🎮️playground/🖥️native-host/🟨️.mjs`:42–68). Its supplied view routes reads through observedFs (inference:1251). No host Rust body is read by this helper. |
| Browser contribution JSON | Engine selection helper reads contribution via observedFs (`💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs`:4–13). |
| Policies, taxonomy, schemas and inference helper implementations | Define inference and validation behavior; `implementationRevision` explicitly reads helper code/schema authorities (inference:1723–1727), plus bootstrap observations before inference scopes. |

Rust `include_str!` / `include_bytes!` assets enter the closure and are existence-probed, but non-`.rs` assets return before content parsing. A referenced `.rs` suffix is parsed even through those include forms. Dynamic includes, build scripts, unsupported or ambiguous Rust source constructs conservatively fall back to broader task inputs. Full Rust source content is not published as module metadata.

## Raw Reads Outside observedFs

These require explicit authority, because sourceCaptures cannot see Node module-loader JSON imports or direct node:fs reads:

- `library/🏗️build/🕸️graph/🟦️.ts`: imported `library/🏗️build/🧬️schema/🔣️.json`, `library/🏗️build/🔣️policy.json`, `library/⚡️caching/🔣️policy.json`, `library/🗂️workspaces/🔣️policy.json` (lines5–8).
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json`: raw read by `buildModesV1` at mode/`🟦️.ts`:8. Inference revision hashes its returned enumeration, but observation does not record that schema read.
- `library/🏗️build/📤️arguments/🧬️schema/🔣️.json` and `library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json`: raw node:fs reads at arguments/`🟨️.mjs`:1–3, during module initialization.
- `library/🏗️build/🕸️graph/📥️inference/🛡️policy/🟨️.mjs` raw-reads script sources. Current full module metadata already covers all those script bytes.
- Current async catalog provider `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts` raw-reads `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🔣️.json` and `🌎️hub/📇️catalog/🔣️.json`. Both are returned in `inputs` and explicitly observed by the caller after provider completion; no uncovered provider JSON was found.
- Installation identity helper imports its schema raw, but inference explicitly `readPhysicalSource`s that exact schema before constructing its parser, so current capture includes it.
- `portableInputs` uses raw async reads of private `.🧬semio/🦑️repo/⚡️cache/build-input-closures/<digest>.json` and `build-lock-closures/<digest>.json` (graph:72–86). These are generated compaction inventories consumed during this discovery; record them or ensure discovery-owned creation and expansion authority. They are excluded from workspace discovery by design.

Imported helper code still matters even if cached by the runtime. All included JS/TS helper bytes are eventually covered by module metadata; their raw JSON dependencies require the explicit rows above. This audit is scoped to the current discovery call graph, not a general guarantee about arbitrary future providers.

## Membership Without General Content Reads

Test discovery uses feature and adapter filenames/kinds, taxonomy and case directories, without parsing `.feature` bodies or arbitrary fixture bodies (`🧪️test/🟨️.mjs`:235–267). Leaf `.go`, `.py`, `.cs`, TeX, Markdown, images, and arbitrary JSON fixture payloads generally become task input paths/globs without their bytes being read by discovery. Retain membership/mode and any actual observations; avoid a blanket extension-based content authority. Exceptions include the explicitly consumed JSON contracts, Cargo/Rust closures, command import closures and provider inputs above.

## Capture and Cache Requirements

Capture must span the complete `discoverBuildGraphV1`, including awaited provider/bootstrap work and dependency/metadata phases. Retain load-time observations; nested AsyncLocalStorage inference scopes otherwise hide facts from an outer scope.

Replay source facts on valid whole-inference cache hits, project-record inheritance, synchronous/asynchronous memoized parser hits and reused module/dependency publications. Do not mistake cache validation reads for a complete current discovery observation list. Preserve missing files and directories: adding a previously absent Rust module candidate, adapter, manifest, generated boundary or resolver candidate changes topology without modifying any existing file bytes.

Observation source inspected at `library/🏗️build/👁️observation/🟨️.mjs`: project inheritance38–51, record93–104, memoized records210–245, observeInference278 onward. Core was adding the global capture and cache inheritance during this audit; this report does not assert its final implementation has been tested.

## Self-Generated Cache Observation Diagnostic

Read-only follow-up while core runs its single original-envelope discovery diagnostic. This agent did not start another discovery or change source.

The observation engine's inference header reads, record-store reads/writes/compaction, publication/receipt writes, and source/lock compaction-file existence probes use raw Node filesystem operations. In particular, `build/📥️inputs/🟨️.mjs` uses raw `persistedExists` before creating content-addressed source/lock closure files. No direct observed negative-existence or directory-listing probe of those internal caches was found in these paths. Actual generated closure bytes are separately consumed during portable input expansion and must retain their content authority.

A broader source observation can still include generated entries: `inferProjectInputs` uses observed `readdirSync(projectRoot)` solely to detect a .csproj/.fsproj/.vbproj file, but its readdirNames fact covers every returned entry. Creating a generated `dist`/`target` directory inside that owner would invalidate the fact without changing its compiler selection. The Cargo manifest walkers similarly observe full typed directory listings before filtering generated directory entries. These are concrete overcapture candidates, not a proven cause of the current retry. Fixing one would require a projection-specific observation at the inference consumer; silently filtering all observed directory reads would discard authority for consumers that use the complete listing.

At the time of this audit, `exact-authority-diagnostic.log` contained discovery progress only, without the original exception or stale-fact rows. Core received the concrete call sites and the distinction between ruled-out direct cache probes and broader listing observations. No source change is justified solely by this incomplete diagnostic.

### Completed Diagnostic Evidence

The diagnostic subsequently completed with original exception `Conflicting inference observations`, 101247 captured facts and 11 stale facts. Every stale row was a file-content fact for an authored Cargo manifest, project JSON, Rust implementation/test, or TypeScript script. No stale cache path or directory-listing fact appeared among those 11.

This reviewer independently recomputed raw filesystem stat and SHA-256 for all 11 reported paths: all 11 had different content hashes and different mtime/ctime from the captured facts. The diagnostic log was created at epoch 1791980793725 ms. Six OS Cargo manifests changed at 1791980815658–5660 ms, the geometry Rust engine at 1791980855807 ms, the OS project metadata at 1791980906858 ms, the original-field Rust test at 1791980914194 ms, and the UI runtime script/resources test at 1791980952197–2198 ms. These writes occurred during the roughly 2.7 minute discovery window.

The observed failure therefore has concrete concurrent authored-source mutation evidence. It does not justify weakening content authority or changing cache observation paths. The broader directory overcapture candidates remain potential improvements, not the demonstrated cause of this diagnostic failure. Core and the parent received these results immediately; this agent made no source edits and launched no discovery.

## Observation Lifetime Across Retries

Read-only source audit during core's fresh-process diagnostic. No observation, inference, graph or test source was edited and no full graph was launched by this reviewer.

| Storage | Lifetime and effect |
| --- | --- |
| `loadObservations` | Module-level strong Map with no clear/reset/delete. `loadSealed` becomes true at the first observeInference entry and never resets. Every capture merges these rows on success/failure; every fresh inference prepends them to its rows/publication. |
| Capture Map | Fresh Map per `captureObservedSourceInputsV1` call, propagated through that async chain. No automatic merge of an earlier attempt's live capture occurs. Actual reclamation depends on remaining async references and GC; source inspection does not prove immediate release. |
| Inference scope | Fresh Map per computed inference/dependency phase. Valid nested/project/cache inheritance adds only the inherited rows. Parent validation memo is keyed by parent scope in a WeakMap, so it does not intentionally cross attempts. |
| `projectImages` | Process-level cache; requires matching identity and revalidation of observations before inheritance. Ordinary changed source facts should invalidate the image and recompute, rather than permanently attach old live facts. |
| `recordStores` | Process-level map of open content-addressed stores, no per-capture disposal. Parser values remain keyed by content/revision; store retention is separate from graph result capture lifetime. |
| `usedInference` / `originalInferencePublications` | Process-level maps retained across attempts. Null/reset/delete occurs after successful compute/consistency, so a thrown computation can leave previous entries. Dependency publication enumerates same-directory history. This is historical receipt/memory retention, distinct from the unconditional load-row merge. |

A semantic change to any recorded load fact therefore remains stale in every same-process retry. A refreshed producer module revision still imports the same canonical observation module, so producer reload alone does not remove those old rows. If current code rereads changed authority, the result can contain both old load and new live facts and correctly refuse consistency; retries cannot heal the old load row by recomputation alone.

The set called module-load authority is broader than imported declarations: `record` routes any observed operation before the first observeInference into that Map, including early workspace inventory policies and explicit graph/source declarations. Changes to those sources must be distinguished from ordinary Cargo/project/Rust reads inside a live inference scope. The earlier 11-source diagnostic demonstrates live source changes; it does not by itself prove those rows belonged to load authority.

Core's newer explicit `authorityObservations` capture field makes the distinction observable. Core/root were sent these concrete lifetime findings, including the historical-publication caveat. Source is stable from this agent: the last production changes remain the previously handed-off portable-input helper; subsequent surface gating and these audits changed reports only.




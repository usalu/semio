# S Development GIS Installation Deletability Audit

Read-only finite-source audit, 2026-10-08. No runtime, build, test, physical deletion, or atomic repository acceptance was executed. No production or fixture files changed. RepoMCP was unavailable; no ticket lifecycle is asserted. Read root and S AGENTS plus GIS, Stdio, and Block owner instructions. The completed 67-package framework partition audit is outside this report.

## Selected Execution Slice

The current general S development application eagerly installs GIS gismap and Puzzle children in browser, worker, native, and MCP entrypoints. Make this application boot from an explicitly supplied neutral installation inventory, with GIS and Puzzle concrete inventory authored by their child-owned deployment extensions. This is a coherent high-impact slice because removing gismap currently affects all three application entry paths, beyond merely removing GIS tooling tests.

This finding does not claim that a generic S runtime definition exists at `✏️s/🦀️.rs` or `✏️s/🟦️.ts`: those paths were absent. The actual observed executable composition is in `✏️s/🧑‍💻dev`. Existing descriptions explicitly call it outward composition. If deployment-specific composition is intentionally allowed to depend on children, the new generic owner must be separated from that deployment rather than silently treating its concrete inventory as neutral.

## Defining Files and Callers

Paths below are relative to repository root.

| Source identity | Observed edge and classification |
| --- | --- |
| `✏️s/🧑‍💻dev/🚀️entry/🟦️.ts:1-4` | Runtime static import of GIS `GIS_INFERENCE_PRESENTATION_V1`, Puzzle `PUZZLE_BOARD_SESSION_FACTORIES`, and a worker URL; invokes `bootFrameworkOsDev` with those fixed child selections. |
| `✏️s/🧑‍💻dev/🧩️service-composition/👷️worker/🟦️.ts:2-5` | Runtime static import of gismap `createGisMapInferenceWorkerV1`; fixed owner `gis` and service `s.gis.gismap.inference`; invokes framework worker installation. |
| Same worker, hot/vitest branch | Hot disposal is lifecycle code. Test-only dynamic import of GIS schema and test dependencies is test composition evidence, separate from the runtime factory import. |
| `✏️s/🧑‍💻dev/💡️services/🦀️.rs:2-6` | Public `service_contributions_v1()` returns a fixed one-element vector of concrete gismap installed service. |
| `✏️s/🧑‍💻dev/💡️services/⌨️entrypoint/🦀️.rs:3` | Native runtime caller passes that vector to the neutral framework renderer. |
| `✏️s/🧑‍💻dev/💡️services/🌉️mcp/⌨️entrypoint/🦀️.rs:2` | MCP runtime directly builds a concrete gismap MCP protocol vector plus Hub credential protocol. |
| `✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust/Cargo.toml` | Unconditional dependency on `semio-s-artifact-gis-gismap`; `mcp-service` also references its child feature. This is package composition evidence with runtime callers above. |
| `✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle/🚀️entry/🟦️.ts:2-3` | Explicit Puzzle variant runtime; keep concrete imports with its specific owner, not with generic S boot. |
| `✏️s/🧑‍💻dev/🧬️schema/🔣️.json` and builder | Tool deployment descriptor names S browser and Vite paths, local Hub owner; observed descriptor has no GIS artifact schema reference. |
| `✏️s/🧑‍💻dev/💡️services/📇️catalog/📣️publisher/🟦️.ts` | Tooling invokes neutral Hub catalog bootstrap with caller-supplied packages; no observed fixed GIS registry import. |

Concrete producers remain child-owned:

- Gismap `💡️inference/🪟️presentation/🟦️.tsx:65` defines the installed presentation, typed by framework `InstalledServicePresentationV1`.
- Gismap `💡️inference/👷️worker/🟦️.ts:15` defines worker factory accepting framework `DocumentServiceWorkerHostV1`.
- Gismap `💡️inference/👷️worker/🦀️.rs:151` defines native service contribution.

## Plugin Registry Counterevidence and Scope

`✏️s/🔌️plugins/🗄️stdio/🦀️.rs` already accepts caller-owned `ContributionRegistry` and `AssemblyOwner`, validates selected factories against exact runtime declarations and receipt bijection, then applies selected definitions and kinds. Its package depends on its registry contract and framework libraries, with no observed concrete codec dependency. Do not replace this with a new first-party default catalog. The contract package name contains `artifact`, but its defining directory is `📇️registry/🧬️contract`, not a concrete `🗿️artifacts` child.

`✏️s/🔌️plugins/🧱️block/🦀️.rs` mounts shared schema and retirement modules. Its TypeScript root reexports shared plugin records from `🧬️schema/🧱️shared`; this is owner-local contract mounting, not an observed concrete artifact dependency. Comments describing independence are not deletion proof.

The inspected Norm contract and Stdio contract root sources did not yield concrete artifact path/package matches in the focused search. This is finite root-source evidence only; their entire descendant graphs were not audited here.

`✏️s/Cargo.toml` still carries explicit child workspace members and dependency path aliases, including gismap. These are build inventory edges, not evidence that the framework renderer imports gismap. A physical deletion experiment must also handle this S workspace inventory; merely injecting empty runtime arrays is insufficient. General framework workspace partitioning does not prove S workspace deletion.

GIS TypeScript package metadata declares S-3d and CAD module dependencies; this may represent a separate plugin sibling dependency slice. Its inspected package script imports repository/process tooling, not artifact implementation. Do not conflate that tooling manifest with GIS runtime registration without inspecting actual implementation callers.

## Neutral Contract Proposal

Reuse existing framework installed-service, presentation, worker-host, surface-session, and MCP protocol contracts. Introduce only a schema-first installation inventory contract for the generic S application: caller-owned owner identity, installed service descriptors, browser presentations, worker installers, surface factories, and native/MCP protocol inventories. Each selected entry must carry its identity and activation/disposal behavior. Empty inventories must be valid and must boot the general shell.

Do not expose GIS models or factory return types from this contract. The current worker variable is typed with `ReturnType<typeof createGisMapInferenceWorkerV1>`; replace that generic-owner typing with the framework installed-driver contract. Child-owned GIS inventory composes the gismap factories and presentation, while generic entrypoints consume only neutral contract values. Duplicate identities and cross-owner installation must reject deterministically. Installation and retirement must preserve cancellation and lifecycle behavior already provided by the worker host.

Native linking cannot become deletion-safe by adding an optional feature while leaving an unconditional path dependency or workspace member behind. The child-owned deployment must own its concrete Cargo dependency and feature forwarding. Generic boot must not maintain a fixed list of child-specific feature names. Discover only actually present caller-authored deployment inventory, or build generic S independently from specific deployment workspaces.

## Deletion Law and Oracle Plan

Create language-agnostic JSON cases owned by the neutral inventory contract: empty, one synthetic contribution, two independent contributions, duplicate ID, invalid owner, removed contributor, and disposal/cancellation. Execute identical cases in TypeScript and Rust. For JSON schema/identity validation compare normalized results with an existing third-party JSON-schema validator behind test-only infrastructure; select and verify the repository-supported validator before implementation. Third-party validation checks schema/output equivalence; it is not a substitute for runtime deletion.

Deletion projection must physically omit one selected child in a ticket-generated isolated source copy, without modifying shared working files or using Git worktrees. First omit gismap artifact, then omit the entire GIS plugin in a separate projection. Preserve neutral owner files byte-for-byte and omit only child-owned inventory/deployment entries according to declared ownership. Run generic S boot/build through existing bun/nx scripts and register any permanent executable command in launch.json. A generic build still referencing a missing Cargo member, dependency feature, TS import, module mount, or fixed roster is a failure, not an accepted fixture update.

Runtime witness must show the generic shell starts with zero GIS services and remaining synthetic/Puzzle services remain available, with [DEBUG] prefixed temporary logs. Removed IDs must be unavailable, not silently rebound. Native and MCP generic entrypoint tests must construct an empty inventory; worker installation/retirement and remaining entry execution must be observed. Also delete one synthetic contributor and verify the second preserves registration, activation, and cancellation semantics. This report has not executed those witnesses.

Targeted inverse-edge checks cover the exact generic files above and their manifests, rejecting concrete GIS/Puzzle identifiers in the generic owner while accepting references inside specific deployment owners. No claim is made about all repository code or atomic acceptance.

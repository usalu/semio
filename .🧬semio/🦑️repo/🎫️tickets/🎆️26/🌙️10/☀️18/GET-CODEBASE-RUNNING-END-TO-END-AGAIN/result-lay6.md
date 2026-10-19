# Result lay6: WP6 OS (non-renderer) to OS, cycles and cross-package deep imports

Executor `lay6`. No `package.json` / `bun.lock` edit, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`, no cargo, no git-modifying command, no `✏️s/**` edit. Renderer (`📺️renderer`) untouched (WP9).

All paths are under `🧰️framework/🛍️products/💻️os/` unless stated.

## 1. Totals (WP6 = source outside `📺️renderer`, target inside the OS product)

Same dependency-cruiser config and rules as `result-lay0.md`, scoped run (`--include-only ^<os>/`, 3,532 modules, about 2.5 min): `lay6-tools/cruise.sh`, `lay6-tools/agg.ts`.

| rule | WP6 before | WP6 after | whole OS before | whole OS after |
|---|---:|---:|---:|---:|
| no-circular | 129 | **2** | 346 | 219 (217 renderer, untouched) |
| no-cross-package-relative | 303 | **255** | 749 | 701 |
| no-core-path | 10 | 10 | 12 | 12 |
| total | **442** | **267** | 1,161 (incl. renderer-hosts-only-ui 54, unchanged) | 986 |

The audit's 475 is from a different snapshot and a whole-repo run; the scoped count above is the one I measured before and after on the same tree.

## 2. Cycles: 129 to 2

All 127 removed non-renderer cycles were ONE strongly connected component of 31 modules in `🔨️modules/🏪️store/👷️worker/` (the split of the store worker). The runtime-only graph was already acyclic: every cycle ran through type-only edges, because the state aggregate named its owner classes and the owners named the state.

Cut by three techniques (no behavior change, no re-exports, no compat layer):
1. **Owner declares its slot** (declaration merging). `ArtifactState` (`🧾️document-state`) no longer imports the three owner classes. New `interface ArtifactOwnerSlots {}`; `🪪️execution-lease`, `🧊️cold-pair`, `🧷️actor-reservation` each declare their own slot with `declare module "../🧾️document-state/🟦️.ts" { interface ArtifactOwnerSlots { … } }`. dependency-cruiser counts the module's ordinary import only (verified on a fixture), tsc enforces that the slot exists. The same idiom wires the reservation collaborators: new leaf `🧷️actor-reservation/🧩️hub/🟦️.ts` (`ReservationCollaborators`, `ReservationHub`); the ten collaborators import it and declare their own member; the composition root class is now `ReservationHubWiring implements ReservationHub`.
2. **Type owners move to the registry that holds them**, pure types only:
   - new leaves `📁️folder/🏷️mirror-owner` (`FolderCanonicalBootstrapMirrorOwner`) and `🚀️artifact-bootstrap/🏷️owners` (`DocumentArtifactBootstrapOwner`, `DocumentArtifactRebootstrapOwner`);
   - `DirectoryCommandTransportOperationV1`, `SpaceArtifactCreationOperationV1` moved into `🛑️session-teardown` (owner of the operation registries);
   - `DirectoryBootstrapOwner`, `DirectoryBootstrapTransition`, `DirectoryAdministrationOperationV1` moved into `🔐️hub-session` (owner of the state variables and the machine class);
   - `DocumentExecutionTargetAssetV1` moved into `📥️execution-target-read`; `DocumentServiceWorkerContextV1` moved from `🗂️config-lane` into `🧾️document-state` next to `documentServices`.
3. **Function to its state owner**: `dropVerifiedColdDocumentPair` moved from `⚙️constants` to `🧾️document-state` (constants no longer import the state). `🎭️actor-lifecycle` takes the two slot types through `NonNullable<ArtifactState[…]>`.

Importers re-pointed: `📁️folder`, `🌉️wire-bridge`, `🚀️artifact-bootstrap`, `🛑️session-teardown`, `🔐️hub-session`, `🧭️directory-bootstrap`, `🛡️directory-administration`, `📇️space-artifact-creation`, `📥️execution-target-read`, `🪪️execution-lease`, `🧊️cold-pair`, `⚙️constants`, `🗂️config-lane`, `🎭️actor-lifecycle`, worker `🟦️.ts`, the reservation parent and its ten children, and the type import in `🔨️modules/📇️directory/🧪️tests/🛤️directory-lane/🟦️.ts`.

Remaining 2 (not code): `🧪️testing/🧩️jcoprobe/🌐️browser-host/🪞️preview2-shim/{http,in-memory-http}.js` (vendored jco shim) and the gitignored build output `🔨️modules/🧑‍💻dev/📤️distribution/🧶️bundles/🖥️runtime-*.js` (a generated bundle that the cruiser should not walk; policy owner: exclude `📤️distribution`).

New language-agnostic test (own SCC scan vs dependency-cruiser `cruise` API on a portable corpus of 5 cases incl. type-only, dynamic edge, slot augmentation, diamond; then the real worker tree): `🔨️modules/🏪️store/👷️worker/🧪️tests/🔁️acyclic-import-graph/🟦️.ts` + `🧫️fixtures/🔣️.json`.

## 3. Cross-package deep imports: 303 to 255

Rule semantics: a file outside a package's source owner may import only authored `exports` targets. All of the 303 are OS sub-packages (`framework-plugin`, `plugin-registry`, `framework-os`, `-os-dev`, `-renderer-wgpu`, …) reaching into each other. The rule fires twice (`framework-os` and `framework-os-kernel`) for the same edge, so 277 unique edges carry 303 violations.

`lay6-tools/ncpr-fix.ts` (TypeScript compiler API) re-points an import to the bare package entry only when an EXISTING export entry exposes every imported symbol with the identical declaration (checked through the type checker, alias-resolved; namespace and dynamic imports need a superset of all exports). 32 edges qualified and were rewritten in 21 files, e.g. `../../../🟦️.ts` to `@semio-tech/framework-os`, `…/🧑‍💻dev/♻️activation/🟦️.ts` to `@semio-tech/framework-os-dev/activation`, store/flow/playbook/vcs test fixtures to the existing `…/testing/…` entries. File list: `lay6-tools/ncpr-changed.txt` (all under `🔨️modules/🔌️plugin/**`).

The remaining 249 edges (255 violations) cannot be fixed without new `exports` entries (manifest freeze), so they are recorded, not touched. The exact spec is `lay6-tools/exports-needed.json` (per package: target file, proposed subpath, union of imported symbols, importer list):

| package | new export entries needed | importer edges |
|---|---:|---:|
| `@semio-tech/framework-plugin` (has 0 exports today; dev, store worker, registry and flow import it) | 51 | 152 |
| `@semio-tech/plugin-registry` | 18 | 81 |
| `@semio-tech/framework-os` | 6 | 6 |
| `@semio-tech/framework-renderer-wgpu` | 6 | 6 |
| `@semio-tech/plugin-extension-store` | 2 | 2 |
| `@semio-tech/framework-os-dev` | 1 | 1 |
| `@semio-tech/framework-renderer-react` | 1 | 1 |
| total | **85** | 249 |

Public entries in this repo are glue files inside the package directory (explicit re-export list, see `📦️packages/🟦️typescript/🔨️modules/**`) plus the manifest key; the later batch must add both, then re-run `ncpr-fix.ts apply` (it will then find the entries and rewrite the 249 imports).

Notes for the manifest batch:
- The 32 rewritten imports are bare `@semio-tech/*` workspace specifiers from files whose nearest `package.json` does not list the target (for example `📇️registry` lists neither `framework-os` nor `framework-os-dev`). The cruiser does not flag this (`aliased-workspace`), `Bun.resolveSync` resolves all 32, tsc and the suites pass, but add `@semio-tech/framework-os` and `@semio-tech/framework-os-dev` to the registry and plugin manifests when the freeze lifts.
- Only 7 of the 249 remaining edges target another module's `🧪️tests` fixtures; the rest are production sources (mostly `🧑‍💻dev/♻️activation` and `🧑‍💻dev/🏗️builder` reaching into `🔌️plugin/🌐️browser-bundle`, `🏗️build`, `📇️registry`).

## 4. Not done (and why)

- `no-core-path` 10: `🌊️flow/🫀️core` (Rust crate + wasm bindings `flow_core.js`, 8 edges) needs a crate rename (cargo, frozen areas); jcoprobe shim `common.js` (4 edges) is vendored. Naming decisions for the flow owner.
- Renderer cycles (217, SCCs of 60 / 13 / 2 files in `📺️renderer/…/ShellHost|UiDocumentStore|ShellHelpers`) belong to WP9; the same slot/hub idioms apply.

## 5. Verification (exact commands, outcomes)

All from `/Users/ueli/Documents/semio`, foreground.
- Cruise: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework/🛍️products/💻️os --include-only '^🧰️framework/🛍️products/💻️os/' --config <lay0 config> --output-type json` before and after (numbers in section 1). Worker-only scope for iteration: SCC 31 to 17 to 0 members.
- tsc: `NODE_OPTIONS=--max-old-space-size=8192 node node_modules/typescript/bin/tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json --pretty false`: 34 errors before and after, identical set (missing `@dnd-kit`, `react-resizable-panels`, jsdom/pngjs types and similar; none in touched files).
- OS vitest config (`🧪️tests/🎚️config/🟦️.ts`, runner `lay6-tools/runvt.py`): 749 passed before and after; the same two files fail before and after (`🧪️tests/🔁️execution-target-retry` needs package `retry`, `🧪️tests/🚑️actor-recovery` needs `cockatiel`). Plugin store 9/9, window-kits 13/13, shell 7/7, plugin registry 90/90 pass; dev config 28 failed (Bun-in-node, playwright, import attribute), not related to touched files.
- `bun test` of 33 worker-importing suites (`lay6-tools/runtests.py`): identical result before/after, except `🧑‍💻dev/🧪️tests/📇️canonical-bootstrap-folder-mirror`, which now stops on a peer's syntax error in `🧰️framework/🔨️modules/📡️replication/🟦️.ts:203` (not mine).
- New test: `bun test ./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🧪️tests/🔁️acyclic-import-graph/🟦️.ts` 2 pass, 13 expects (also tsc-clean).
- Bare specifiers: `Bun.resolveSync` of all 32 replacements from their importing directory resolved.

Ticket tools kept in `lay6-tools/` (scripts, `exports-needed.json`, lists); generated output (`🗑️generated/lay6-run`) deleted.

# S-Generator-Schema

## Outcome

Version 7 now owns generator identities and output ownership without storing arbitrary executable arguments. The strict registry contains 19 classifications and 36 disjoint output roots:

| Ownership | Contracts | Output roots | IDs |
|---|---:|---:|---|
| owned | 10 | 14 | `actor-typegen`, `async-typegen`, `framework-manifest`, `plugin-registry`, `print-latex-tokens`, `scale-fixture`, `schema-entity-catalog`, `shell-typegen`, `ui-axes`, `ui-contract` |
| unsafe | 4 | 9 | `assets-build`, `graph-catalog`, `styling-tokens`, `wgpu-frame-worker` |
| unknown | 3 | 5 | `ownerless-ui-icons`, `root-layering-declarations`, `setup-wizard-config` |
| external | 2 | 8 | `external-cargo-locks`, `external-step-assets` |

Fourteen owned/unsafe records carry an exact live Nx `project:target`; six owned records also carry an exact check target. Unknown/external records have no target and are therefore impossible to execute through the schema.

Production writes were limited to:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`

No normalization engine, shared test, root/project manifest, Git state, `compose/**`, or `temp/compose/**` path was modified.

## Frozen schema

```ts
type GeneratorOwnership = "owned" | "unsafe" | "unknown" | "external";

type GeneratorOutputRoot = {
  path: string;
  inclusion: "tracked" | "ignored";
};

type GeneratorContract = {
  ownership: GeneratorOwnership;
  ownerPath: string | null;
  target: string | null;
  checkTarget?: string;
  inputPatterns: readonly string[];
  outputRoots: readonly GeneratorOutputRoot[];
  reason: string;
};
```

Runnable commands are derived, never stored:

```text
generatorNxCommand(contract)      -> ["bun", "nx", "run", contract.target]
generatorNxCheckCommand(contract) -> ["bun", "nx", "run", contract.checkTarget] | null
```

`generatorContractIdsForOutputPath` resolves an exact output or descendant to its one registry owner. The validator rejects command/argv/banner fields, non-kebab or unstable registry ordering, invalid/non-Nx targets, duplicate targets, missing owned inputs/outputs, inputs on non-runnable classifications, unordered/duplicate paths, input/output self-overlap, cross-contract output overlap, and owner/input/output paths capable of touching the opaque compose boundary.

`loadTaxonomy` additionally finds the live workspace and calls `validateGeneratorContractsAgainstWorkspace`. It reads each owned/unsafe owner’s exact `📋️project.json`, proves project and target/check identities, proves every declared output maps to exactly one contract, and requires every owned tracked output to exist. An ignored output may be absent only when its contract says `inclusion: "ignored"`.

## Proven owners

The owned set comes from exact project/script contracts, not banners or generated-directory heuristics:

- Actor, async, and OS shell typegen: Nx-declared owner roots and exact sibling `🤖️generated` projections.
- Framework manifest, UI axes, and UI contract: exact in-memory projection writers with check targets.
- Plugin registry: eight ignored registry outputs plus tracked `.vscode/launch.json`, jointly generated and byte-checked by `@semio-tech/plugin-registry:{generate,check}`.
- Print: tracked `🖋️latex/semio-tokens.sty` from the styling token source.
- Scale fixture: two tracked deterministic JSON files with `scale-fixture-check`.
- Schema entity catalog: exact TypeScript, Go, and Rust projection paths with a check target.

`plugin-registry` inputs explicitly cover its launch seed, plugin tree, Cargo package discovery, registry sources, discovery implementation, and current taxonomy path. This current taxonomy input must move atomically when the self-hosted taxonomy is relocated.

## Fail-closed classifications

Unsafe:

- `styling-tokens`: `🛂️adapters.manifest.json` declares `📦️packages/🐍️python/🎨️styling/🤖️generated.py`, while the generator’s `pyGeneratedPath` points to the absent double segment `📦️packages/🐍️python/🎨️styling/🎨️styling/🤖️generated.py`. The schema records the real declared/current output and refuses to run the target. Required repair: fix the owning styling `📜️script.ts` path, add an exact check target, then change ownership to `owned` and restore its exact inputs.
- `graph-catalog`: its workspace scan does not enforce the opaque compose exclusion.
- `assets-build`: two icon roots are evidenced, but the build target lacks a complete deterministic multi-language output manifest.
- `wgpu-frame-worker`: a wasm build writes a tracked worker as an undeclared side effect and has no generator freshness target.

Unknown:

- The ignored duplicate UI icon tree has no discovered owner.
- Root `package.json`, `Cargo.toml`, and `go.work` are declared layering-generated but have no writer target.
- `.ralph-tui/config.toml` has a generated header but no deterministic repository target.

External:

- The two non-ticket Cargo lockfiles are Cargo-owned with no exact Nx regeneration contract.
- Six exact STEP assets carry ST-Developer provenance but have no owned import target.

The plugin surface scaffold command remains outside runnable generator ownership: it creates missing files, accepts dynamic arguments, and is not a byte-reproducible Nx regeneration unit. The engine must report it as unsupported rather than reconstructing its shell arguments.

## Adjacent strict contracts

- Added `taxonomy-registry`, the exact semantic kind for the future `📇️taxonomy/🔣️.json` self-hosted target. No file was moved in this slice.
- Added fixed `🎯️goal.json` and arbitrary-depth governed goal-node contracts under `.🧬semio/🦑️repo/🎯️goals/**` so repo-MCP goal IDs are never renamed.
- Added a narrow `**/.agents/skills/*/SKILL.md` fixed filename contract for skill discovery.

## Verification

Strict load and focused assertions:

```text
bun -e '<load taxonomy; inspect generator counts; resolve taxonomy/goal/skill contracts>'
schemaVersion=7
generatorContracts=19
ownership={owned:10, unsafe:4, external:2, unknown:3}
📇️taxonomy=taxonomy-registry
goal file=[goal-manifest]
goal directory=[goal-node]
skill file=[agent-skill]

bun -e '<validate every target, check target, output owner, and Git inclusion>'
contracts=19; targets=14; outputs=36; problems=[]

bun -e '<negative schema/live validation assertions>'
arbitrary command rejected=true
overlap rejected=true
opaque input rejected=true
missing target rejected=true
missing tracked output rejected=true
missing ignored output allowed=true
generate/check tuples exact=true
```

Workspace checks:

```text
bun build .../discovery/🟦️component.ts --target=bun --external @semio-tech/framework --outfile /dev/null
exit 0; 113.83 KB

bun nx show project @semio-tech/plugin-registry
exit 0; generate/check targets present

bun nx show project @semio-tech/ui-styling-tokens
exit 0; generate target present (classification remains unsafe for the path mismatch)

git diff --check -- <two owned production files>
exit 0
```

`bun nx run @semio-tech/repo-lib:lint` remains red only on pre-existing out-of-scope UI `ImportMeta.env/glob` and OS plugin `rootDir` errors; it reported no error in the schema/discovery files. `bun nx run @semio-tech/plugin-registry:check` reached its taxonomy audit with no stale-output diagnostic, then failed on the existing large set of plugin tree/leaf normalization violations; the target is therefore not globally green and no generated output was rewritten.

Final SHA-256 evidence:

```text
efaade5c975c9cac16c1922f9c42a3b4926bf818e1f9a095eafc5cadead11168  🔣️taxonomy.json
bc7e86947de9513e49d0d3c3cf8c5e2e95a92fabf2567e7a570296cbcb368cd8  discovery/🟦️component.ts
e77296156dcdef4b4ce1a9d0b632320c6a9dc5c9b5803beceb844b760de7eb34  generatorContracts canonical JSON
```

These hashes describe the safe boundary at report time and must be recomputed if a concurrent writer changes either shared production file.

# Kernel Emoji Repair

Scope: `🧰️framework/🔨️modules/🎠️kernel`. The handpicked naming repair, TypeScript runtime verification, shared discovery integration, and Rust compilation check are complete. This report makes no workspace-wide completion claim.

Every source-controlled file and directory in this tree was inspected by role. Unique format markers (`🦀️.rs`, `🟦️.ts`, `🔣️.json`) are retained; the user request does not justify replacing useful schema-first format leaves indiscriminately. Reserved `package.json`, `vitest.config.ts`, and dependency-managed `node_modules` are left literal. All JSON schemas and fixtures remain separate and intact.

## Handpicked Decisions

Paths below are relative to the scoped kernel root. Each choice is explicit, not selected by a renaming algorithm. An arrow changes only the named basename unless a full relative destination is shown.

| Parent | Original | Handpicked | Reason |
| --- | --- | --- | --- |
| root | `🧪️oracle` | `🔮️oracle` | Independent reference implementation |
| root | `🧪️fixtures` | `🧫️fixtures` | Controlled sample inputs, distinct from executable tests |
| `🧪️tests` | `🖱️satisfy-version-requirements` | `✅️satisfy-version-requirements` | Requirement satisfaction, not pointer input |
| `🧫️fixtures/📇️descriptor-load` | `🔣️.schema.json` | `🧬️.schema.json` | Schema distinct from the sibling JSON fixture |
| `🧫️fixtures/🚪️turn-patch-owner` | `🔣️.schema.json` | `🧬️.schema.json` | Schema distinct from the sibling JSON fixture |
| `📥️poll/🏘️composition` | `🧬️contract` | `📜️contract` | Written composition ownership contract |
| `📥️poll/🏘️composition` | `🧬️🧬️schema` | `🧬️schema` | Domain composition shape |
| `📥️poll/🏘️composition` | `🧪️🧬️🌾️🌾️schema` | `📐️fixture-schema` | Shape of neutral test cases, distinct from domain shape |
| `📥️poll/🏘️composition` | `🧪️fixture` | `🧫️fixture` | Neutral composition examples |
| `📤️return/🏠️source` | `🧪️fixture` | `🧫️fixture` | Source ownership examples |
| `📤️return/🏠️source` | `🧪️schema` | `🧬️schema` | Source fixture shape |
| `📤️return/🏠️source/📚️entries` | `🧪️fixture` | `🧫️fixture` | FIFO entry examples |
| `📤️return/🏠️source/📚️entries` | `🧪️schema` | `🧬️schema` | Entry fixture shape, distinct from executable tests |
| `📤️return/📦️content` | `🧬️wire` | `🔌️wire` | Wire-format connection contract |
| `📤️return/📦️content` | `🧪️fixture` | `🧫️fixture` | Content byte vectors |
| `📤️return/📦️content` | `🧪️🧬️🌾️🌾️schema` | `📐️fixture-schema` | Fixture shape, distinct from content declaration schema |
| `📤️return/📦️content` | `🧪️framing` | `🖼️framing` | Frame boundary tests |
| `📤️return/📦️content` | `🧪️dialects` | `🗣️dialects` | Field encoding dialect tests |
| `📤️return/📦️content/💌️message` | `🧪️fixture` | `🧫️fixture` | Message byte vectors |
| `📤️return/📦️content/💌️message` | `🧪️schema` | `🧬️schema` | Message fixture shape, distinct from executable tests |
| `📤️return/📦️content/📥️input` | `🧪️fixture` | `🧫️fixture` | Input byte vectors |
| `📤️return/📦️content/📥️input` | `🧪️schema` | `🧬️schema` | Input fixture shape |
| `📤️return/📦️content/📥️input/🪪️authority` | `🧪️fixture` | `🧫️fixture` | Authority lifetime examples |
| `📤️return/📦️content/📥️input/🪪️authority` | `🧪️schema` | `🧬️schema` | Authority fixture shape |
| `📤️return/📦️content/📥️input/🏗️builder` | `🧬️contract` | `📜️contract` | Builder binding contract |
| `📤️return/📦️content/📥️input/🏗️builder` | `🧬️🧬️schema` | `🧬️schema` | Builder binding contract shape |
| `📤️return/📦️content/📥️input/🏗️builder` | `🧪️🧬️🌾️🌾️schema` | `📐️fixture-schema` | Builder trace fixture shape |
| `📤️return/📦️content/📥️input/🏗️builder` | `🧪️fixture` | `🧫️fixture` | Builder binding traces |
| `📤️return/📦️content/📥️input/🧾️release` | `🧬️contract` | `📜️contract` | Release evidence contract |
| `📤️return/📦️content/📥️input/🧾️release` | `🧬️🧬️schema` | `🧬️schema` | Release contract shape |
| `📤️return/📦️content/📥️input/🧾️release` | `🧪️🧬️🌾️🌾️schema` | `📐️fixture-schema` | Release trace fixture shape |
| `📤️return/📦️content/📥️input/🧾️release` | `🧪️fixture` | `🧫️fixture` | Release readiness and identity traces |
| `📤️return/📦️content/📥️input/📦️payload` | `🧬️contract` | `📜️contract` | Resident payload association contract |
| `📤️return/📦️content/📥️input/📦️payload` | `🧬️🧬️schema` | `🧬️schema` | Payload contract shape |
| `📤️return/📦️content/📥️input/📦️payload` | `🧪️🧬️🌾️🌾️schema` | `📐️fixture-schema` | Payload lifetime fixture shape |
| `📤️return/📦️content/📥️input/📦️payload` | `🧪️fixture` | `🧫️fixture` | Payload detachment and lifetime examples |

## Verification

All 36 explicitly listed renames were performed as exact no-clobber filesystem moves. No file contents were restored from Git. No modifying Git commands, automatic emoji selection, global substring replacements, or migration scripts were used.

- Complete live naming audit: 54 non-reserved files and 55 directories; zero missing prefixes, multiple-emoji basenames, or sibling collisions. All JSON files parse.
- `bun nx run '@semio-tech/framework-kernel:test-quick' --skip-nx-cache`: **49 tests passed** across two source files. The pre-edit command could not find its default config. The owning package script now explicitly passes its existing reserved `vitest.config.ts`; the shared default was not changed.
- Sixteen neutral schema/fixture and contract/schema pairs pass the independent Ajv validator. Fourteen are self-contained; the two referencing poll/resident and actor contracts also pass after their existing referenced schemas are registered. No fixture fields were changed.
- Exact incoming references were patched in actor shard-client, OS poll-composition, and renderer React tests. A source scan found no remaining references to the renamed kernel fixture/schema/contract/wire/framing/dialect paths.
- Registry role resolution now recognizes the handpicked structural roles. The two direct test-case directories are not global semantic directory kinds; test-case discovery owns those names.
- Focused contribution-directory tests: **4 passed, 27 expectations**, using the long Nx level. They exercise neutral cases, the lodash lookup oracle, synthetic discovery/dependency ownership, malformed/absent manifests, extracted actual root-script classifiers, and live kernel discovery. The quick invocation exceeded its 30-second whole-run budget during live repository discovery; it was not reported as passing.
- `bun nx run '@semio-tech/framework-rs:check' --skip-nx-cache`: **passed**, using an isolated target directory under this ticket. The framework library tests compiled, one type-export test passed, and the generated mirror was confirmed fresh. This was not a claim that all 268 Rust tests were executed; 267 were filtered out by the existing target.
- `@semio-tech/repo-test:lint` was run and failed. No reported error is in the new owner-override code. Reported errors are in replication typed-array APIs, UI `ImportMeta`, OS directory exports/buffer types, a library callback, and the existing test-domain `measureCoverage` access to undeclared `oracleRequirements.oracle`. The latter region is unchanged relative to HEAD. These are recorded as remaining checks, not silently presented as a passing typecheck.

## Precise Source Edits

Within the kernel: package `📜️script.ts`, root `🦀️.rs`, content `🟦️.ts`, framing and dialect Rust tests, message Rust tests, and source-entry Rust tests. Their functional bodies were preserved; edits target reviewed module paths and the owning Vitest config argument.

Exact incoming references outside this tree: `🧰️framework/🔨️modules/🎭️actor/🧵️shard-client/🟦️.ts`; `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🟦️.ts`; `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`.

The subsequent manifest repair also updates the kernel root's one exact type import to `../🛂️manifest/🤖️generated/🎚️ui-axes.ts`. All 49 kernel tests pass again after that change; its generator coordination is recorded in `🪪️manifest-repair.md`.

## Shared Registry Coordination

The parent agent owns central taxonomy and added the reviewed structural roles and the `testContributionDirectoryOverrides` authority. The test-domain library now reads that required map, resolves exact owner overrides, discovers only each owner's declared directory, and uses the same rule for production-dependency ownership. An overridden owner's old name is not an alias; the same emoji at an unrelated owner does not grant a test exemption. Rust oracle inspection follows the actual contribution manifest directory. Diagnostics no longer insist every oracle directory has the old emoji.

Added language-neutral cases and schema under `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧭️contribution-directory-ownership`, consumed by the existing test-domain TypeScript test file. No runtime dependency was added. The root dependency-contribution scanner is parent-owned and was separately flagged for the same exact-owner rule.

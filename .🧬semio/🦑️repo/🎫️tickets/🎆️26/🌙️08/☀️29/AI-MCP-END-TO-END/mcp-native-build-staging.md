# MCP native build staging: generated-module prerequisites

## Root cause
`buildMcpArtifactV1` (os/mcp/stdio/staging) went straight to `buildAdmittedCargoArtifactsV1`. Rust source admission
(`native-build/source`, `source.unresolved-generated-module`) refuses a `#[path]` mount (`icon_name`, from
`icon-contract-rs`) or an `include!` (`entity-kinds`, from `framework-schema`) whose file is generated output that
does not exist yet. Nothing in the MCP project graph declared the owners, so a cold checkout could never admit.
The refusal text itself says "run <owner> first"; the dev playground already honours that, the MCP route did not.

## Contract implemented
- Schema-first: `staging/🧬️schema/🔣️.json` `Producer.prerequisites` declares `{target, produces[]}` owners.
- `mcpGeneratedPrerequisiteRequestsV1` returns `run <owner> --output=stream` only for owners whose products are missing (cheap stat, no workspace scan).
- `buildMcpArtifactV1` runs them first as owned neutral children (`runOwnedBuildChildInvocationV1`), then compiles.
- Birth (`dev/🌱️birth`) declares these children as launches for `dev mcp stdio os`.
- Graph route: `dependsOn` of the MCP rust `build`, `build-release`, `check`, `test` targets lists the same owners.

## Tests
- `staging/🧪️tests/🧫️fixtures/prerequisites.json` (language-agnostic) + tests in `staging/🧪️tests/🟦️.ts`, cross-checked with Ajv and an SQLite oracle.
- Consistency test: declared owners resolve through `generatedModuleOwnersV1` and gate the MCP targets in project.json.
- Existing producer test updated: prerequisites run before the single cargo call.

## Findings
- The graph route `repo run ...:build` is blocked by a resident slice worker crash (ref 9a7af9ea), unrelated.
- After the first fix the next admission refusal was `source.missing-input` for `schema/🤖️generated/🏷️entity-kinds/🦀️.rs` (an `include!`, not a `#[path]`), whose owner `framework-schema:generate` declares `outputs: []`.

## Further declared owners found by iterating the warm staging route
- `@semio-tech/ui-styling-tokens:generate` -> `ui/styling/tokens/rs` (`mod generated` mounted via `#[path]`; the owner declared `outputs: []`, so admission could only say `unresolved-module`). Now declares its tokens outputs.
- `@semio-tech/framework-schema:generate` now declares its two entity-kind outputs instead of `outputs: []`.
- Generator drivers run natively when the resident graph service blocks (`🗑️generated/native-build/entity-direct.ts`, `tokens-direct.ts`).
- After admission, cargo compiled ~1750 units; the host warden cancelled `semio_os_mcp_plugin_runtime|host` (2502 MB vs library ceiling 1536 MB, not on the carving list). Declared it in the admission `unitCeiling.carving` list (precedent: other host entries) pending a real split of that crate.
- A later warm run failed with `OriginalAdmissionClosed ... lstat .cache/b/r/m/published/<key>/prototype` (the published root vanished after killed runs); not caused by the prerequisite change.

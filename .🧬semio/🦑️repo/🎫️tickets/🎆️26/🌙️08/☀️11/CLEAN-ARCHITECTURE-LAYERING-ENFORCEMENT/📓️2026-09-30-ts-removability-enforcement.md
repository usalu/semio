# TypeScript and JavaScript Removability Enforcement

## Implemented Scope

Boundary configuration now accepts deleted application/plugin/artifact workspace owners without eagerly opening their missing directories. It still rejects malformed workspace paths, non-directory owners, malformed present JSON, missing present manifests, and missing package names. Plugin rule discovery comes from present directory contributions. An empty plugin inventory leaves valid generic framework-to-plugin path enforcement.

Strict graph verification inventories all present taxonomy areas and repository-level TS/JS source files, independently of resolver graph output. It includes framework-to-implementation, repository-source-to-implementation, s-module-to-plugin, semantic taxonomy direction, and plugin-to-extension-or-artifact rules. CAD/procedural extension warning exceptions were removed. The broadened plugin rule is named `plugin-no-extension-or-artifact-*`; no compatibility alias was retained. Package aliases, exported subpaths, type imports, dynamic imports, tooling, and tests are covered. Known authored aliases and unresolved `@semio-tech/` names fail instead of becoming accepted terminal nodes.

Portable test execution is independent of the live repository lint. The canonical aggregate still runs both. The library script also registers the root agent's Rust source-direction test/lint commands and aggregate checks.

## Changed Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️dependency-direction/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️dependency-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️dependency-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`

Nx/package/launch registration was handled by the root agent. No taxonomy changes, new runtime dependencies, Git writes, worktrees, or AGENTS edits were made.

## Executed Verification

- Initial Nx attempt found no `test-dependency-direction` target. Root registered it.
- The next attempt failed during module loading while the root agent's Rust source execution module was still being written.
- First actual portable run: 7 passed, 1 failed because Ajv strict types rejected a JSON schema object/string union. Replaced the union with `anyOf`.
- Subsequent portable run: 8 passed, 0 failed, 508 assertions.
- Final portable run after full-policy removal oracle and repository-source inventory fixture changes: `bun nx run @semio-tech/repo-lib:test-dependency-direction --skip-nx-cache` passed 8 tests, 0 failed, 514 assertions; Bun test runtime 8.85 seconds, Nx target runtime 9.3 seconds. Tests use Ajv and dependency-cruiser as independent third-party oracles and run copied full CJS policies against isolated ticket fixtures with s/plugins/artifacts absent. No live application directories were deleted.
- `bun nx run @semio-tech/repo-lib:lint-dependency-direction --skip-nx-cache` inventoried 8,981 sources, ran the real dependency-cruiser graph, and FAILED strictly after approximately 93 seconds. The first reported unresolved retained workspace dependency was `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🎞️slide/🌷️Einführung/🪻️Einleitung/👋️Einleitung.ts → @semio-tech/mit-bestand-praesentation-projektetage-spec`. No baseline or exemption was added.
- Scoped `git diff --check` passed.

The real repository lint has not passed. It stops on the first completeness/resolution problem, so this run does not prove the absence of other unresolved or forbidden edges. TypeScript typecheck was left to root integration. Full root-router removability remains limited by unrelated concrete plugin references outside this slice. All generated fixture directories owned by this slice were removed; this Markdown report is retained.

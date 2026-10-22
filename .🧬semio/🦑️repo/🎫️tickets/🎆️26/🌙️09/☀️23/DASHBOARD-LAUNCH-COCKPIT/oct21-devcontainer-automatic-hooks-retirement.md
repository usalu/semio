# Automatic Devcontainer Hook Retirement

The actual attach lifecycle called configureRepoHooks, which ran raw bun ./script micro-commit install-hooks. The devcontainer table accurately reflected this, but it conflicted with the root developer policy that setup does not install Git hooks and human operations stay explicit in the dashboard.

The closed hook policy and six language-neutral argv cases were authored before the production edit. The implementation now removes that helper/export and its attach call. The obsolete success/failure tests of the removed installer were replaced with a no-installation contract: the existing actual recording-host attach matrix validates the closed policy with Ajv and independently checks forbidden hook arguments with SQLite and lodash. Existing start coverage also rejects a micro-commit install-hooks call. A focused new Bun law calls the actual attach ownership helper and is mounted by Source0, with its exact name required in the existing acceptance plan. No target was added; the retained roster is42.

The optional F3D viewer, authentication, ownership, safe-directory handling and signing preparation remain owned by their existing ports. Unrelated micro-commit functionality and existing explicit setup-git preparation were not changed. Developer docs now describe the actual optional-viewer attach behavior and prohibit automatic Git hook installation.

Static checks: policy Ajv-valid, independent SQLite DATA6/6, four touched TypeScript files parse without diagnostics; longest authored source path167UTF16. Actual Bun, lifecycle, start/attach recording-host execution and real container runtime are unrun. No Git or container command was executed, and no runtime red/green claim is made. Independent Low source review accepted removal, exact current attach calls, the closed policy and oracles, DB cleanup, literal Source0 mount and docs. Runtime remains unrun.

Changed files:

- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🧬️schema/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧫️fixtures/🔁️lifecycle/🔣️.json
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🟦️.ts
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🪝️automatic-hooks/🟦️.ts
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts
- /Users/ueli/Documents/semio/.devcontainer/README.md

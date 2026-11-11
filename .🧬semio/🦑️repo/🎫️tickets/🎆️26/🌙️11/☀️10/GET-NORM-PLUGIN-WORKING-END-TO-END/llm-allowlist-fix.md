# LLM Allowlist Fix for Claude 5 Family

Single source of truth: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧬️schema/🔣️allowed-values.json` (embedded by Rust via `include_str!` and by Go via `go:embed`).

Added slugs (newest first within family): `fable-5-2`, `opus-5-5`, `sonnet-5-5`, `haiku-5-5`. Long-form `claude-*` ids are not added; `ResolveAllowedLLM` resolves them by containment (`claude-opus-5-5` -> `opus-5-5`).

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧬️schema/🔣️allowed-values.json` (source of truth)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/💡️graphql-enums/🐹️.go` (duplicated `AllowedLLMs`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/💪️effort/🐹️.go` (coverage list)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧪️tests/📋️allowed/🐹️.go` (coverage list + new `TestClaude5FamilyResolves`)

## Verification

- JSON parses (81 llms); gofmt clean on the three Go files.
- Go tests could not run: the emoji-named directories are rejected as Go import paths (`malformed import path`), which is a pre-existing limitation of the test layout.

## Activation

The Rust crate embeds the schema at compile time, so the running MCP server needs a rebuild of the `repo` binary (not performed here). Restart the `repo` MCP server after rebuilding.

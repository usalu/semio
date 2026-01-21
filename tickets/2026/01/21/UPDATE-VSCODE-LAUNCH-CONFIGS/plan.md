# Plan: Update VSCode Launch Configs

## Objective

Update VSCode launch.json and tasks.json with:
1. New naming convention: `@package/name script`
2. Reorder: specific packages first, general commands last
3. Lifecycle ordering: dev -> test -> build -> publish:test -> publish
4. Add missing test scripts: test, test:unit, test:e2e, test:coverage

## Package List

| Package | Name |
|---------|------|
| go/semio | @semio/go |
| go/repo | @semio-repo/go |
| go/server | @semio-repo/server |
| rs/semio | @semio/rs |
| py/semio | @semio/py |
| py/engine | @semio/engine |
| js/semio | @semio/js |
| js/docs | @semio/docs |
| js/play | @semio/play |
| js/desktop | @semio/desktop |
| js/vscode | @semio-repo/vscode |
| net/Semio | @semio/net |
| net/Semio.Grasshopper | @semio/grasshopper |
| assets | @semio/assets |
| assets/logo | @semio/logo |
| assets/icons | @semio/icons |
| yak | @semio/yak |

## New Naming Convention

Format: `@scope/name script`

Examples:
- `@semio-repo/go build`
- `@semio-repo/vscode dev`
- `@semio/js dev:storybook`
- `@semio/js test:coverage`

## Ordering

1. **Package-specific** (grouped by package, ordered by lifecycle)
2. **Root/general commands** (at the end)

## Cleanup

- Remove all mcp references (go/mcp doesn't exist)
- Add go/server and rs/semio configurations

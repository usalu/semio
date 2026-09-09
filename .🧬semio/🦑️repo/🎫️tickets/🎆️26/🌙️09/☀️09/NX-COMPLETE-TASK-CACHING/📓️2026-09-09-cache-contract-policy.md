# Cache-contract policy update

Language-agnostic `nx-contract` fixtures now encode the plugin `targetPolicy`:

- cached: `test`, `test-quick`, `test-exhaustive`, `lint`, `build`, `generate`, `verify`, `format-check`, `schema-check`, `cpp-build`
- uncached mutating: `format`, `setup`, `publish`, `update`, `deploy`, `clean-test`, `bench`, `generator-inputs`
- uncached continuous: `dev`, `serve`, `watch`, `dev-storybook`, `test-watch`

`format-check` is cached on purpose. `matchesUncached` treats `format` as an exact name, so the `format-` prefix does not uncache `format-check`.

`pluginGraph.workspaceRoot` proves the workspace-root deny override is gone: discovering `📋️project.json` at `.` still applies the same policy rows. `nx.json` still has no `targetDefaults`.

`repo:generator-inputs` and generator `checkTarget` freshness guards stay uncached. Cacheability is compared to native Nx `isCacheableTask`.

# Provider Identity Projection

## Scope

Implemented FND-PROVIDER-PROJECTION-16 as a pure caller-supplied Cargo TOML projection. It validates repository-relative manifest locators before parser invocation, rejects excluded and cross-platform-unsafe locators, and does not traverse filesystems, invoke Cargo, resolve aliases, or approve dependency edges.

The projection preserves package/library distinction, virtual workspaces, package-version workspace inheritance, dependency keys, package overrides, local paths, workspace inheritance, normal/development/build scopes, target conditions, and unsupported dependency facts as unapproved data.

## Verification

- `bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts' test -t 'cargo provider manifest projection'`
  - Passed: 1 test, 30 expectations.
- `bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️provider-projection-root-review/📜️script.ts'`
  - Passed: 14 independent coordinator vectors, 0 failures.
- `bun './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts' lint`
  - Remains blocked by pre-existing cross-module `rootDir` and UI `ImportMeta` type errors. The projection's previous Bun-global type error is absent after the repository-owned runtime adapter change.

The temporary concurrent taxonomy blocker has cleared. The final registered Nx project-target replay passed: one focused test, 30 expectations, 294 filtered, exit 0. The final independent replay passed 14 adversarial vectors, 5 actual manifests, and 14 complete `@iarna/toml` oracle checks with zero failures (`runSWvbJU`).

The initial Python oracle candidate is unavailable on this host (`python3` is 3.9.6 without `tomllib`, and no newer Python is on `PATH`). With coordinator authorization, tests instead use the already-installed test-only `@iarna/toml` 2.2.5 independent parser. No package was installed and production code has no third-party TOML dependency. The multiline initial-newline vector retains the independent oracle's expected `"first line\\n"` value while the bounded projection rejects all actual multiline strings before Bun parsing.

## Review Surface

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️cargo-provider-projection/🛂️schema.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️cargo-provider-projection/🔣️vectors.json`

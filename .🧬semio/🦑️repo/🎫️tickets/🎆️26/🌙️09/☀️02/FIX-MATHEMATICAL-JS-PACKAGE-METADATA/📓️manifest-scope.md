# Mathematical TypeScript Manifest Scope

The package exports only the Mathematical artifact schema and IO TypeScript facades. Those exported modules use no external packages.

Its sole Nx target is `test`, implemented by `bun ./📜️script.ts test`. The test script imports Ajv as its only non-platform package and validates the Mathematical publication-authority fixture and Rust route surface.

The matching small-plugin convention is `@semio-tech/lowpoly-js`: one self-scoped `test` package script, Ajv as the sole dependency, and TypeScript as a development dependency. Therefore the Mathematical manifest should remove the nonexistent `generate` and `fixture` aliases, replace the CAD target with `@semio-tech/mathematical-js:test`, and replace all CAD/runtime dependencies with Ajv.

The first post-manifest verification reached a pre-existing stale fixture path in the package test. The publication-authority fixture and schema are now taxonomy-owned at `🧪️publication-authority/🔣️.json` and `🧪️publication-authority/🔣️.schema.json`, so the test must resolve those committed paths before it can validate the Rust source named by the fixture.

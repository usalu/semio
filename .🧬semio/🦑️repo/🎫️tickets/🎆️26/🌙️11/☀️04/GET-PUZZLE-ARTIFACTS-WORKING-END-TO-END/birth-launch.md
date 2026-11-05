# Exact Fresh Root Activation Launch

Fresh `repo dev time-travel` selects the actual owned activation request before issuing its Original. It requires one explicit `--variant`, one explicit `--renderer` (`react` or `wgpu`), a `prototype` or `release` `SEMIO_BUILD_MODE`, and membership in the caller supplied generated playground catalog. The build bootstrap receives `run` plus the exact activation target; no wildcard capability or received Original mutation is introduced.

The existing catalog reader moved to a dependency free leaf and remains reexported by its existing owner. Activation target naming and catalog validation moved to a leaf shared by fresh root birth and actual activation execution. Framework root code contains no puzzle variant names. Missing, duplicate, unknown, and help selections do not acquire activation capabilities. Configured malformed or missing catalog files retain the reader refusal.

## Changed Files

- Repo root birth implementation, protocol schema, portable fixture vectors, and SQLite/provenance proof test.
- Repo playground owner and new `🎮️playground/📚️catalog/🟦️.ts` catalog reading leaf.
- OS development activation execution owner and new `♻️activation/🎯️target/🟦️.ts` target selection leaf.
- Portable two variant catalog fixture `🌱️birth/🧪️tests/🧫️catalog/🚀️playgrounds.json`.

## Validation

TDD: changed schema and fixtures first; the test failed because the fresh factory never declared the required activation child. After implementation, `repo tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧪️test/🌱️birth/🧪️tests/🟦️.ts` passes one test, 41 portable vectors and 174 assertions. AJV validates each vector, SQLite independently selects the request and derives the exact target from the same input mapping, and spies execute the actual fresh factory and workspace birth helper. Positive vectors cover puzzle2d/react/prototype and puzzle3d/wgpu/release. Received cases retain object identity and never call fresh provider or bootstrap factories.

## Runtime Boundary

The integration agent owns the serialized actual time travel rerun after the current declared puzzle WASM producer settles. Native compiler execution is not duplicated by this slice. This proof validates exact selection, provider ordering, and genuine fresh birth dispatch; full runtime outcome is reported by the integration report.

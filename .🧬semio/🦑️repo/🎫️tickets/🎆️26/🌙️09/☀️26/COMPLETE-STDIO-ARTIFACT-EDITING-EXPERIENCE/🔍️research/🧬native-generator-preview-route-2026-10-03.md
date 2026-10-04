# Native Generator Preview Route

Preview26 failed before Stdio compilation because the framework native package now declares its preview target through the repository native owner command, while generator validation required direct execution. The declared wrapper supplies the exact Cargo policy consumed by the framework exporter; bypassing it would be incorrect.

The shared generator preview resolver accepts only the exact declared owner script, either directly in the owner directory or through the native owner wrapper from the workspace root. It checks the precise manifest, cwd, script, operation and executor. Taxonomy validation and actual preview invocation share this resolver. Seven neutral fixtures include foreign-manifest, foreign-cwd, foreign-script, operation and misplaced-wrapper refusals.

Executed red1: missing resolver failed. Executed green2: three selected regression tests passed, including seven neutral routes and bounded process-dispatch arguments for both direct/native routes, with the process boundary replaced by a recording test double, executed with both Bun and independent TypeScript transpilation. Fixture structure is independently checked by Ajv. A narrow diff whitespace check passed.

Preview27 passed this gate and reached the Hub wasm build, then stopped because Hub Cargo.lock no longer matched the shared dependency graph. Registered workspace:deps-cargo-lock is running. No browser result is claimed.

The native exporter process itself was not executed by this regression selection. Preview28 proves the taxonomy validation path in the real application build, while the normalization exporter’s real stdout/stderr protocol remains a separate integration boundary.

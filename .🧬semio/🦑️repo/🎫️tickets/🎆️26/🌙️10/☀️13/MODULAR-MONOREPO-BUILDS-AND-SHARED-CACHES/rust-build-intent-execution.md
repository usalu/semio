# Rust Build Intent Execution

The new closed declarative intent is implemented at `library/🏗️build/🕸️graph/🦀️rust/📥️intent`. It admits version, portable manifest, exact package, default/package or explicit library/binary scope, features, default-feature selection and target triple. It refuses arbitrary arguments, profile selection, storage controls, workspace selectors and external manifest locations.

Twenty-one existing Repo Rust build declarations retain their original Cargo package/default-deliverable selection in explicit `metadata.semio.rustBuildIntent`. Their aggregate tasks are uncached. Their package script build bodies redirect into owned Nx. If an actual build task reaches its package script before selected graph consumption, a guard rejects before Cargo dispatch; this prevents recursive command execution and does not grant a fallback cache claim.

The language-neutral JSON fixture and Ajv oracle were authored before the parser. The first real Nx test run failed because the implementation did not exist. After implementation, the strict schema needed an explicit conditional-property declaration; the test exposed this and the schema was repaired. The package-route law then failed before route integration and exposed the literal events-package selector, which was integrated explicitly.

The `rust-build-intent` prototype/release headroom Nx targets and matching editor launches now own the law. Prototype ran successfully: 2 tests, 459 assertions, 0 failures. Release also ran successfully: 2 tests, 459 assertions, 0 failures. This is schema and real-declaration integration evidence, not compiler artifact reuse proof. Selected graph consumption is authored by the Nx owner; the intent law now verifies owner resolution and both-mode aggregate noop/intent preservation for every actual declaration, and actual 21-route compiler-graft qualification remains pending.

Coordinator includes a binary under its original package/default scope. CLI and dashboard binary construction remain separate production frontiers. No binary reuse or complete monorepo reuse claim is made here. Their exact first-party adapter contract must be investigated after graph-consumption integration; silently changing these requests to `--lib` would violate actual deliverable selection.

## Current Edit Coordination

Ingress Source has stayed frozen since the final intent-law checkpoint. The Nx owner reports that no sequential compiler pair is live yet: its predecessor genuinely refused an external Source budget/opening change, and Root/Storage re-froze afterward. Nx authored schema-first unique descriptor-directory roots, observed the genuine original-root mismatch failure, and is checking the repair before a new SDK and sequential pair. Fallback/app reader production edits remain gated on that terminal checkpoint. No ordinary 21-package compilation success or per-unit reuse is claimed from these static laws.

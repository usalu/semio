# Test Layout Enforcement

The existing repository test contract discovers only cases that already contain `🧪️tests/<case>/🥒️.feature`. Its separate unmanaged-test survey recognizes a small filename set and compares area counts with `🚚️migration.json`; it does not reject every legacy file, canonical tests without Gherkin, or inline test bodies.

The permanent rule is structural: an authored executable test implementation must be the direct file ` <semantic owner>/🧪️tests/<emoji-and-kebab-case case>/<canonical implementation filename>`. Canonical implementation filenames are derived from `testAdapterFileKinds` and `fileKinds`; TypeScript therefore accepts `🟦️.ts`, `🟦️.tsx`, `🟦️.mts`, and `🟦️.cts`, while Rust, Go, Python, and .NET use their taxonomy file kinds. A semantic owner cannot be below `📦️packages` or `🎯️targets`.

The scan must reject legacy filename conventions even when a file contains no recognizable framework import, and must find executable declarations in canonically named or ordinary source files. Source recognition ignores comments and string literals. Rust external test-module wiring is valid only when `#[cfg(test)]` and `#[path = "…"] mod …;` resolves to a canonical case implementation; inline `#[cfg(test)] mod … {}` and test attributes outside canonical implementations remain violations. TypeScript and JavaScript test-framework imports, `import.meta.vitest`, and `*SelfTest` declarations are executable-test signals. Go, Python, and .NET use their standard test declarations in addition to legacy names.

The scanner API is asynchronous, accepts an `AbortSignal`, and reports deterministic progress and sorted findings. The repository `test contract` gate invokes it, so all root test levels enforce the layout through the existing Nx dependency on `@semio-tech/repo-test-domain:test-contract`. Nx production inputs exclude the canonical `🧪️tests/**` subtree instead of relying on legacy filename suffixes.

Language-neutral JSON vectors cover valid and invalid paths and source forms. Bun executes the implementation test, while the installed `minimatch` package independently checks the filename-overlap vectors.

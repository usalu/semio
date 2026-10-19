# Exact Cargo Executable Metadata Repair

## Problem And Change

The pure Cargo protocol reader retained actual target kinds only locally and returned the executable path. The runtime runner still referred to kinds after the protocol refactor, causing an undefined-name TypeScript error and a runtime ReferenceError before procedural-macro loader discovery.

The schema now describes a closed executable descriptor with path and a bounded, unique list of target-kind strings. The protocol reader returns a frozen descriptor with a separately frozen kinds array. The runner destructures that descriptor and preserves its existing procedural-macro runtime discovery through the same capture stage, authored list deadline and loader environment projection. The later nativeLibraryDirectory path remains intact. No compiler, storage, funding, invocation, lease or source-admission rules were changed.

Current consumers were reviewed: the runtime runner now consumes both fields; the recorded-transport tests intentionally consume the reader only for validation; new descriptor tests consume its complete value. No compatibility return path was added.

## Test-Driven Evidence

The neutral metadata corpus covers procedural macros, ordinary/rust/shared/C/static libraries, mixed library kinds, integration tests, binaries, malformed kinds and a mismatched target.

Before implementation the new diagnostic suite produced 2 passes and 11 failures, including the actual extracted runner branch's ReferenceError for kinds. After implementation the same 13 tests pass. Ajv independently validates the descriptor contract; SQLite independently determines procedural-macro membership from the original Cargo JSON. The extracted actual runner branch verifies discovery only for the matching kinds and retains the authored runtime deadline. That test uses an injected capture callback and never launches rustc or Cargo.

A broader selected run initially found a Windows-only test-fixture path mismatch: literal /artifacts/00 differed from path.join's normalized group path and entered an unrelated directory-identity branch without its fixture bindings. Normalizing that fixture binding with join preserves the intended pure input-consumption test without introducing a fake filesystem identity or compiler authority. The broader selected run then passed 18 tests, with 377 assertions and 10 unrelated tests filtered out. It includes the existing genuine issued-phase consumed-input test and the portable loader environment/transport protocol tests.

No native compilation, fixed-volume allocation or incapable canonical native gate was run. The complete historical exact-Cargo test file is not claimed clean: its older whole-runner proc-macro tests reference absent invocation helpers and omit the currently required real storage/control inputs. Those tests were not revived using fabricated compiler funding.

## Changed Files

- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/📋️protocol/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/📋️protocol/🧫️fixtures/📦️executable.json`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/📋️protocol/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/📦️executable/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/📥️inputs/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts` imports the new tests under the existing canonical suite.
- This retained report.

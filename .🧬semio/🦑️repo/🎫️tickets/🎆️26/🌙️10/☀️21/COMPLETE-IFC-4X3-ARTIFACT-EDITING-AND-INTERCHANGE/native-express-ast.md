# Native Canonical EXPRESS Program

The complete ADD2 program compiled by the TypeScript frontend is now represented as immutable Rust AST data under the same `🧬️schema/⚖️validation` taxonomy. Own public Rust structs/enums mirror the language-neutral AST schema; exact integer/real literals retain source text and every node retains its source offset. The native program can be borrowed without runtime parsing, allocation or third-party dependencies.

The retained ticket input `express-program.ts` invokes the actual bounded schema compiler, requires zero diagnostics, validates the resulting complete program with existing Ajv against `ast.schema.json`, rejects every unmapped field and emits Rust literals. It also retains a temporary canonical JSON program under ticket generated output for differential work. Rust static program source is production declaration data.

Current corpus: 777 WHERE/type expressions, 60 derived expressions, 48 functions, two global rules, 165 inverse declarations, six contextual bounds and four UNIQUE declarations. Canonical program JSON SHA-256: `10f5e2e69fd863a7e26e85f1a24d5f4915e41a3987c99afb449e8729fc8e0b84`. Both global rule identities are retained. The existing EXPRESS source checksum remains in the normative catalog.

Added language-neutral native corpus fixture and written native tests comparing all counts and global rule names. Rust AST/program/module parse and format successfully with rustfmt. Native compilation and tests still require authentic compiler ingress and have not executed here.

The package also registers `native-program.test.ts`: one Bun test with eleven passing assertions compiles the full source corpus, independently admits it with Ajv, checks every neutral count and compares the retained native data digest to the current compiler output. Later frontend corrections therefore invalidate stale native data until it is regenerated. This check executes TypeScript/compiler/schema logic and does not execute the native AST.

This delivers the native syntax substrate, not native rule execution. `IFC4X3_NATIVE_EXPRESS_EXECUTION` is explicitly false. The TypeScript semantic worker continues execution and IfcOpenShell rule-result differential tests; native interpretation, host diagnostics and complete conformance are still open work. See `express-plan.md` and `schema-implementation.md`.

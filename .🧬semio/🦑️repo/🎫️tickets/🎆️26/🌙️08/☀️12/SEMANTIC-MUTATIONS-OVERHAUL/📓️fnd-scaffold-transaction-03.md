# FND-SCAFFOLD-TRANSACTION-03

## Scope

This packet changes only the direct mutation scaffolder transaction seam. It does not claim schema-first mutation behavior, descriptor completeness, codec/mirror generation, derive conformance, or production scaffold acceptance.

## Result

`newScaffoldMutationTree` now prepares every candidate output and the aggregate replacement before publishing. Aggregate mounting uses the tokenizer-backed `inspectRustMutationAggregateSpan` source map, so a mount is placed before the aggregate's outer attributes and variants are inserted at the mapped enum body. Missing, non-public, malformed, nested, and ambiguous aggregate enums fail before leaf output is published. A pre-existing mount must be a public, non-inline `#[path = "<leaf>/🦀️.rs"]` mount; a pre-existing variant must be the exact one-field `<module>::Mutation` wrapper. Conflicts fail before publication.

Publication preflights all candidate targets as absent or regular existing files, rechecks cancellation, and detects source changes before replacing the aggregate. Runtime staging is deliberately adjacent to the aggregate to guarantee the same filesystem; atomic rename requires the same filesystem, not intrinsically the same directory. Its 128-bit unique name is retained only after a successful exclusive create; cleanup requires both its no-follow filesystem identity (`dev`/`ino`) and its expected bytes. Rollback applies the same identity-and-byte check to generated files and removes only empty, identity-matching directories it created. Existing hand-authored files and directories remain preserved.

All source paths, including the repository root, are checked by `lstat` without following symlinks; dangling symlinks therefore fail rather than being treated as absent. This limits ordinary replacement/link races, but Node's pathname APIs cannot make the full validation-to-write sequence immune to a hostile concurrent filesystem mutator; the packet intentionally avoids deleting anything when ownership cannot be proven rather than claiming complete TOCTOU isolation.

## Evidence

The neutral fixture and its draft-07 schema live at [fixture](../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-scaffolding/🧫️fixtures/🔣️.json) and [schema](../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-scaffolding/🛂️schema.json). The focused Nx result is retained in [the green log](🧪️fnd-scaffold-transaction-03-green.log); the fixture-bootstrap red record is retained in [the red log](🧪️fnd-scaffold-transaction-03-red.log).

The 44-assertion transaction regression covers attributed mounting, doc-span boundaries, nested/macro/inline-item and malformed aggregate refusal, wrong/public/private mount and wrong-wrapper refusal, preflight target rejection, dry-run byte identity, hand-authored non-overwrite, cancellation preserving an unrelated concurrent file, deterministic concurrent aggregate commit failure with rollback, and `../`, wrong-owner, compose, dangling-leaf, and symlinked-root scope guards. Ajv validates the language-neutral fixture and nightly rustc parses the AST-mounted aggregate.

No real compose source was inspected and no production `new mutation` command was run.

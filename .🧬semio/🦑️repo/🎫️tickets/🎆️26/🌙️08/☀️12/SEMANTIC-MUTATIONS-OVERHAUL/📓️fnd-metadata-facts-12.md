# FND Metadata Facts 12

## Scope

Replaced `inspectRustMutationMetadataFacts` with a token-and-delimiter-based source-facts inspector. It reports syntax evidence only and makes no package-authority or policy-activation claim. No `compose/**` path was read, traversed, or written.

## Changes

- Added exact paths for derive metadata, including absolute paths, digits, and raw identifiers.
- Added fail-closed `mutation_leaf(contract = <absolute nongeneric path>)` evidence for absent, valid, malformed, ambiguous, and conditional attributes.
- Preserved parent-module `cfg`/`cfg_attr` conditional evidence for declarations and aliases.
- Added exact `extern crate`, `self`, root/scoped `use`, re-export, and grouped-use alias facts with lexical module paths.
- Reused `inspectRustStructure` for manual `MutationLeaf` implementation facts.
- Added a language-neutral fixture/schema covering visibility, unions, outer and inner conditional modules, `cfg_attr`, qualified-attribute decoys, absolute/root/grouped/self imports, raw identifiers, reserved path segments, malformed metadata, conflicting metadata, and body/comment/string decoys.
- Added a standalone Syn 2.0.117 oracle harness using paired same-hash `rlib` and `rmeta` artifacts already retained in this ticket. Each run uses a unique retained directory with copied source, compiler/runtime argv, stdout, stderr, exit status, and full TypeScript-versus-Syn comparison output.

## Executed Evidence

| Command | Result |
| --- | --- |
| `bun .../metadata-facts-root-preflight/📜️script.ts` | Pass: 24/24 adversarial cases. |
| `bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun .../📦️typescript/📜️script.ts test -t 'records exact declaration and alias syntax without decoy inference'` | Pass: 1 test, 5 assertions. |
| `bun .../metadata-facts-syn-oracle/📜️script.ts` | Pass: full Syn/TypeScript parity for 8 declarations, 15 aliases, and 1 manual implementation. |
| Registered package lint target | Did not pass because of `ImportMeta` and cross-package `rootDir` diagnostics outside the metadata-facts files. |

## Limitations

The inspector intentionally does not resolve aliases to packages, evaluate `cfg` predicates, resolve out-of-line modules, or determine policy authority. Conditional evidence is therefore fail-closed and must not be treated as active metadata.

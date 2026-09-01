# TypeScript Declaration Malformed-Source Boundary 63

## Scope

This is a new, ticket-only adversarial packet. It does not change the immutable 28-case declaration contract, ticket57, canonical tests, or the production discovery parser.

The language-neutral law is fail-closed: if the independent TypeScript parser reports a malformed declaration span, the owned declaration inspector must return `incomplete` and must not use a recovered declaration name for provider or mutation identity inference.

## Inputs

- [neutral vectors](./🧪️typescript-declaration-malformed-63/🔣️vectors.json) contain 12 exact raw TypeScript literals and TypeScript 5.9.3 diagnostic code/start/length tuples.
- [neutral schema](./🧪️typescript-declaration-malformed-63/🧬️schema/🔣️.json) closes each case to a required malformed diagnostic and `incomplete`/`forbidden` expectation.
- [controller](./🧪️typescript-declaration-malformed-63/📜️script.ts) validates the vectors with strict Ajv, checks every compiler tuple through TypeScript 5.9.3, then dynamically invokes the captured current D export after no-follow input capture.

## Current Actual RED

Command:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-declaration-malformed-63/📜️script.ts' check
```

The command intentionally exited 1: all 12 malformed inputs were reported `complete` with no owned diagnostics by the actual D subject. The TypeScript compiler independently emitted the authored syntax diagnostics for every case. Before and after capture matched, including D source SHA-256 `2ef678c682fc621f14a7f8557ef16f98ce19a22ab6a10530c73c9a96c304ec3e`.

Retained receipt: [result](./🧪️typescript-declaration-malformed-63/🧫️runs/923cf1f5-a822-4be0-af66-d3a3dcaee572/🔣️result.json) and [terminal failure](./🧪️typescript-declaration-malformed-63/🧫️runs/923cf1f5-a822-4be0-af66-d3a3dcaee572/🔣️failure.json).

| Family | False-complete cases |
| --- | --- |
| Variable/object headers | malformed const header; malformed object member; malformed nested const header |
| Interface/class members | malformed interface member; missing member separator; missing property type; missing class parameter type |
| Generic headers | alias, interface, and class missing generic defaults; malformed generic constraint |
| Enum | missing initializer expression |

The inspector currently exposes recovered names such as `x`, `Shape`, `metadata`, `Box`, `I`, `E`, and `C` from all twelve malformed inputs. This packet does not treat those values as valid declaration/provider evidence.

## Boundary

This is a subject RED only. It proves no parser fix, compiler parity, full declaration census, or provider identity. The parser owner should make malformed tokens and recovery suffixes produce owned diagnostics before any declaration result can be complete; a later fixed-subject run must retain the same raw inputs and compiler spans.

# Mutation Declaration Census 41

## Scope

This is a bounded declaration candidate worklist only. It intentionally does not build a consumer graph, infer reachability, prove a descriptor is adequate, or prove every mutation semantic is covered.

The controller is [📜️script.ts](./🧪️mutation-declarations-41/📜️script.ts), SHA-256 `fb435ea70789e09e4f6ef1d8375a34f6ad509bf0b7d1ec68bdabd4473801d91f`. It resolves the repository parser from the workspace at runtime rather than using a machine-specific import. It walks only the repository-owned roots selected from the live taxonomy plus `✏️s`, `🧰️framework`, `🌎️hub`, and `♻️mit-bestand`.

Before every filesystem access, the controller rejects a relative path containing `compose`, a hidden segment, `.cache`, `cache`, `.git`, `.nx`, `.🧬semio`, `build`, `coverage`, `dist`, `node_modules`, `out`, `target`, or `targets`. It uses `lstat` and rejects every symlink; it reads each accepted Rust source once. Authored packages, tests, fixtures, and examples remain in scope.

## Executed Census

Executed through the required runner:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-declarations-41/📜️script.ts
```

Final retained run: [🧫️run-Zd0Vx4](./🧪️mutation-declarations-41/🧫️run-Zd0Vx4). It completed in `49723ms`, under the `115000ms` deadline, with exit code `0`, no cancellation, and no worker error. The controller/parser inputs were SHA-256 stable before and after the run. The full 13,112-file source-hash roster and all 2,150 declaration candidates are retained in [🔣️inventory.json](./🧪️mutation-declarations-41/🧫️run-Zd0Vx4/🔣️inventory.json); the grouped worklist is retained separately in [🔣️worklist.json](./🧪️mutation-declarations-41/🧫️run-Zd0Vx4/🔣️worklist.json). Progress and worker output are retained in [stdout.log](./🧪️mutation-declarations-41/🧫️run-Zd0Vx4/stdout.log), with an empty [stderr.log](./🧪️mutation-declarations-41/🧫️run-Zd0Vx4/stderr.log) and [🔣️summary.json](./🧪️mutation-declarations-41/🧫️run-Zd0Vx4/🔣️summary.json).

| Candidate declaration | Count |
| --- | ---: |
| `impl Mutation<...>` | 163 |
| `impl MutationKind<...>` | 1,759 |
| `impl CompositeMutationKind<...>` | 9 |
| `derive(Mutations)` | 109 |
| `derive(CompositeMutation)` | 5 |
| `derive(MutationLeaf)` | 105 |
| Total | 2,150 |

| Path classification | Count |
| --- | ---: |
| Canonical direct-leaf candidate | 2,022 |
| Mutation aggregate candidate | 24 |
| Off-facet or manual candidate | 104 |

| Concrete classification | Count |
| --- | ---: |
| Concrete operation candidate | 2,145 |
| Generic interface or blanket candidate | 1 |
| Uninhabited candidate | 4 |

The output keeps origin evidence separate: 1,927 structural implementation candidates, 220 independent lexical-reference candidates, and 3 token-aware metadata-only derive candidates. The three metadata-only derives are recorded explicitly in the full JSON rather than silently dropped.

## Remaining Aggregate and Off-Facet Worklist

[🔣️worklist.json](./🧪️mutation-declarations-41/🧫️run-Zd0Vx4/🔣️worklist.json) groups every source-current aggregate and off-facet/manual candidate by exact path. Every group carries the SHA-256 of the source read in this run and each candidate's line, declaration kind, candidate classification, origin, and evidence note. It contains 24 aggregate source groups and 92 off-facet/manual source groups (104 candidate declarations total). It is a navigation worklist, not an assertion that these are the only unresolved operations or that every listed candidate requires a change.

## Recognizer Validation

The controller ran six handcrafted virtual Rust sources through the repository structural parser and a separate state-machine lexical reference. The reference masks nested comments, quoted and raw strings, and only treats an apostrophe as a character literal when it has a valid closing literal delimiter; Rust lifetimes and labels remain code.

| Case | Structural impl expectation | Lexical total expectation | Result |
| --- | ---: | ---: | --- |
| Concrete `Mutation` / `MutationKind` / `CompositeMutationKind` plus three derives | 3 | 6 | Pass |
| Comment and string decoys | 0 | 0 | Pass |
| Generic `impl<T>` plus `derive(Mutations)` | 0 | 2 | Pass with recorded parser gap |
| Lifetimes, a character literal, and a label before a real impl | observed `1` | 1 | Pass |
| Raw-string and nested-comment decoys before a real impl | 1 | 1 | Pass |
| Generic qualified `protocol::MutationKind` impl | 0 | 1 | Pass with recorded parser gap |

The repository structural parser omits the generic `impl<T>` and generic qualified-path probes, while the token-aware metadata parser observes the generic derive. Therefore generic implementation and derive records that are not available from the structural result are labelled `lexical-reference` candidates, not definitive semantic facts. Any virtual-probe failure makes the worker fail and prevents an exit-0 inventory result. This separation prevents comment/string decoys from entering the candidate set without claiming parser completeness.

## Limits

The line locations for structural implementations are bounded lexical navigation anchors because the structural implementation facts do not expose source spans. Canonical/facet classification is path-based only. The census did not inspect or materialize real `compose`, did not alter root helpers or production sources, and did not run Cargo.

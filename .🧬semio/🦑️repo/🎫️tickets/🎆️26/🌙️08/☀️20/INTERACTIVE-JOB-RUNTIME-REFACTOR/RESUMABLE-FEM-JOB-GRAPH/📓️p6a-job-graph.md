# P6a FEM Job Graph

## Result

The owned FEM algebra, sparse, model, and analyses boundaries are synchronous. Decorative async moved from 155 functions to zero without introducing `.await` or a run-to-completion interactive API.

| Boundary | Before | After |
| --- | ---: | ---: |
| algebra | 39 | 0 |
| sparse | 45 | 0 |
| model | 32 | 0 |
| analyses | 39 | 0 |
| total | 155 | 0 |

`FemJobGraph` owns a serializable ordered cursor over `ValidateReferences -> BuildDofMap -> OrderEquations -> Assemble -> Factor -> Solve -> Recover -> Finalize`. Every step checks operation/generation freshness, cancellation, deadline, and fuel; stage completion yields a checkpoint and progress carries completed/total stages and units. Its tests cover ordered traversal, checkpoint restore, stale generations, and cancellation without mutation.

The graph is intentionally an orchestration state machine. Assembly and meshing kernels remain separate packet-owned children rather than being copied into the coordinator.

## Product Gate

Command:

```text
CARGO_TARGET_DIR="$PWD/<ticket>/🧪️target-p6" cargo check -p semio-s-plugin-fem --lib --message-format=json
```

Result: blocked before FEM compilation by exactly 856 errors in direct dependency `semio-s-plugin-stdio`.

| Code | Count |
| --- | ---: |
| E0308 | 450 |
| E0277 | 239 |
| E0599 | 72 |
| E0271 | 19 |
| all other codes | 76 |

Evidence: `📝️p6-check-boundary-20260821.jsonl` and `📝️p6-check-boundary-20260821.stderr`.

Because Cargo stops in stdio, the product crate does not reach the analyses graph tests. The owned solver modules are compiled independently by the ticket-local diagnostic harness described in P6d/P6e; the analyses source is rustfmt-clean and `git diff --check` is clean.

## Scope

No FEM mesh or meshing source, renderer, stdio source, process transport, or ticket lifecycle state was edited.


# Neural DAG One-Consumer Dissolution Packet

## Live evidence

The stale census labels `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural` as a zero-consumer module. Live source resolution gives a more precise result.

The root `🦀️component.rs` is mounted only once, by `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` as `neural_dag`. Its `tree_from_dag` function is called only by `FlowHost::build_tree` in `🌊️flow/🖥️host/🦀️component.rs`. The sibling neural engine is an independent package and is not part of this dissolution.

The root adapter has two responsibilities:

- converting already compiled DAG wire rows into a neural execution tree;
- parsing an ad-hoc JSON fixture into wire rows.

The conversion has exactly one terminal production consumer and belongs privately inside the Flow host component. The JSON fixture parser and `NeuralDagError` have no production, test, example, glue, mount, or registration consumer outside their defining file; their sole in-file test exercises only the conversion. They are dead and should be deleted, not inlined.

## Current hashes and clean state

- Neural DAG adapter: `190623e4b08666e6a980b29b1cd7c30c90cff7a73985781e314912bd3dc25b3f`.
- Flow host: `10a0c637d57a164d1b4758fb24a54ad3603ebbfda851072e43c4975964d5ce86`.
- Flow Rust glue: `2a3850509a6159f08acb0f6c3a439ff67b9800bb66492fc912ec1d6f4826b048`.

All three paths were clean when audited at repository `HEAD 07873f842a5a99ac2f69c1648c21f36ebf260bdb`.

## Atomic Terra packet

Writable source paths:

- delete `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/🦀️component.rs`;
- update `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs` by placing the conversion helpers privately in its cohesive tree-building region;
- update `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` by removing the `neural_dag` path mount.

The nested `🧠️neural/AGENTS.md`, README, and `⚙️engine` package remain untouched. No Cargo dependency changes are required because Flow host already imports the graph manifest and neural engine types used by the conversion.

Validation must include live stale-symbol/path searches, the OS Flow Rust test target through Nx, scoped taxonomy report/enforce evidence, and source rehashes. At packet authoring time, root Nx dispatch is externally blocked by a syntax error in the protected, moving framework kernel component; the lease may edit but cannot be released until that owner restores the runner and the Nx validation passes.

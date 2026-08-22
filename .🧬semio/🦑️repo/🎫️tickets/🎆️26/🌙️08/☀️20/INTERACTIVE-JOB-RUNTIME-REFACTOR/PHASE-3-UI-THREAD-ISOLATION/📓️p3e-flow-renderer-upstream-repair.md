# P3e Flow Renderer-Upstream Repair

## Outcome

The Flow product host is compiler-reachable again, clearing the upstream wall that prevented the
real renderer UI-thread-isolation gate from compiling.

- Removed stale `block_on` wrappers around the now-synchronous stdio `BrepKernel` surface.
- Kept the node-graph wire contract explicit by serializing neural channel defaults into the
  existing `default_json: Option<String>` field and parsing that string on the inverse path.
- Kept Flow's synchronous public host bridge compiling while its event-store implementation remains
  genuinely async by using the framework-owned retained-waker resolver at the bridge boundary.

## Verification

```text
cargo fmt -p semio-framework-os-flow -- \
  🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📚️catalogue/🦀️component.rs

CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PUZZLE-3D-INTERACTION-CORRECTNESS/🧪️target-p4 \
  cargo check -p semio-framework-os-flow --lib --message-format=short
```

The check exited `0` after a cold 15m32s build. It reported warnings in upstream framework/plugin
crates but no Flow errors. The P3c owner was then released to rerun the real renderer isolation gate.

This packet claims compiler repair only; it does not independently claim the P3 timing/thread gate.

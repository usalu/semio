# Locked Cargo Mesh Engine JSON Member Check

The 16:47 UTC locked Window Config check exited 101 at 18:15 UTC. Raw log `🗑️generated/window-config-compile-1647.log` records pack-json at line 20789 (bracketed by Cargo elapsed 680–690 seconds), mesh-engine at line 44998 (4562–4572 seconds), and the two E0599 diagnostics at 45345 and 45351. The log file last write was 18:15:21 UTC with a 4632-second Cargo progress marker immediately before termination. This places the dependency check near 17:09 UTC and the mesh-engine check near 18:14 UTC. Both the owner and consumer source mtimes are 18:03:29 UTC. The elapsed evidence supports a running Cargo session crossing a shared-worktree update and reusing pack-json metadata compiled before the consumer callsites appeared.

The shared sources currently agree. `🧰️framework/🔨️modules/🎒️pack/🔤️json/🧱️object-and-value/🦀️.rs` exposes `member_at` and `swap_members` at lines 35 and 40 through the normal public re-export in `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`. Indexed lookup borrows a retained key and value through `Vec::get`. Swapping checks both indices before moving two whole tuples, leaving allocations and associations intact. `🧰️framework/🔨️modules/🏗️mesh-engine/🚪️io/💾️binary/🌳️glb-scene/🗂️metadata/🦀️.rs` calls these methods from a funded incremental partition at lines 95–103. The production sources were preserved.

The neutral member corpus now checks two invalid swaps, a same-slot swap and a distinct valid swap. The Rust companion law verifies per-slot order, out-of-bounds lookup, lookup/ordinal agreement and key and descendant allocation identity. It also compares the contents with serde_json. The independent jsonc-parser oracle consumed the same cases and passed with 13 assertions:

```sh
repo tools exec --light bun -- test './🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🧱️ownership/🟦️.ts' --test-name-pattern 'independent JSON member slot swaps'
```

The current locked Cargo confirmation remains with the existing build owner. No extra native build was started.

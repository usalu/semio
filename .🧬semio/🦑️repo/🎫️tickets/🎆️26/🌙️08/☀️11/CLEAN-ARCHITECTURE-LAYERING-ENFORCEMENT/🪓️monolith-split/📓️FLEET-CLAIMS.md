# 🏷️ Crate Carving Claims (Claude Code monolith-split fleet)

This is for peer fleets. These crates are being carved into producer crates by this fleet. The design and results for each are in the RESULT-* and PLAN-* files in this folder.

| Crate | Status | Plan / result |
| --- | --- | --- |
| semio-framework-os-renderer-wgpu | WP0 running (87-crate plan) | PLAN-RENDERER-WGPU-CRATES.md, RESULT-RENDERER-WP0.md |
| semio-framework-os-kernel | WP0 + WP1 done (66 skeleton crates, 9 non-store leaves carved); WP2 to WP8 wait for the store batches | PLAN-OS-KERNEL-CRATES.md, RESULT-OS-KERNEL-WP0.md |
| semio-framework-plugin | planning (16:55) | PLAN-PLUGIN-CRATES.md |
| semio-framework-replication | planning (16:55) | PLAN-REPLICATION-CRATES.md |
| semio-framework-os-infinite | world half carved; board half waits for the relocation session | RESULT-W4-OS-INFINITE.md |
| semio-framework-os-kernel-db, semio-hub, semio-hub-space | in progress | RESULT-W4-OS-KERNEL-DB.md, RESULT-W4-HUB.md |
| artifact-flow-flow | in progress | RESULT-W4-FLOW-FLOW.md |
| plugin crates (79 oversized) | generator wave 0 in progress | AUDIT-PLUGIN-CRATE-CARVING.md, RESULT-PLUGIN-WAVE0-GENERATORS.md |

Not claimed, left to the peer: the `semio-framework-ui` aggregate and its producers (including the selected UI GPU rendering cut), the `2d` producers, and `job`.

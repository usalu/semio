# Physical IDE Configuration Retirement

A completed no-follow filesystem census covered ignored and untracked source paths while pruning dependency/runtime stores (`node_modules`, `.git`, `.cache`, repository cache and ticket-generated output). It found 18 matching paths: 15 local historical hub control paths and three downloaded Lean dependency `.vscode` directories. The tracked-only census had correctly found zero tracked physical IDE configurations.

The local hub copies contain six `Monorepo.sln` files and nine `.vscode` directories with 34 editor configuration files. They are ignored local snapshot/overlay assets, not application source or active compiler inputs. The containing directory entries were inspected before removal; the containing source snapshots remain. Downloaded mathlib, proofwidgets and batteries are third-party dependency caches under `.lake/packages`; their upstream editor metadata is not a repository developer control plane and remains untouched.

The following 40 local control files are selected for removal under the explicit request to remove IDE configurations:

- `./.🧬semio/🌐hub/s14-u6-base/Monorepo.sln`
- `.🧬semio/🌐hub/s14-u6-base/.vscode/extensions.json`
- `.🧬semio/🌐hub/s14-u6-base/.vscode/launch.json`
- `.🧬semio/🌐hub/s14-u6-base/.vscode/mcp.json`
- `.🧬semio/🌐hub/s14-u6-base/.vscode/settings.json`
- `.🧬semio/🌐hub/s14-u6-base/.vscode/🧩️launch.seed.jsonc`
- `./.🧬semio/🌐hub/s14-u6-overlay/Monorepo.sln`
- `.🧬semio/🌐hub/s14-u6-overlay/.vscode/extensions.json`
- `.🧬semio/🌐hub/s14-u6-overlay/.vscode/launch.json`
- `.🧬semio/🌐hub/s14-u6-overlay/.vscode/mcp.json`
- `.🧬semio/🌐hub/s14-u6-overlay/.vscode/settings.json`
- `.🧬semio/🌐hub/s14-u6-overlay/.vscode/🧩️launch.seed.jsonc`
- `./.🧬semio/🌐hub/s15-ex1-overlay/Monorepo.sln`
- `.🧬semio/🌐hub/s15-ex1-overlay/.vscode/extensions.json`
- `.🧬semio/🌐hub/s15-ex1-overlay/.vscode/launch.json`
- `.🧬semio/🌐hub/s15-ex1-overlay/.vscode/mcp.json`
- `.🧬semio/🌐hub/s15-ex1-overlay/.vscode/settings.json`
- `.🧬semio/🌐hub/s15-ex1-overlay/.vscode/🧩️launch.seed.jsonc`
- `.🧬semio/🌐hub/s14-r10-state/window3-backups/render-2026-09-29T02-52-32.287Z/.vscode/launch.json`
- `.🧬semio/🌐hub/s14-r10-state/window3-backups/st2-r10-2026-09-29T01-41-32.931Z/.vscode/launch.json`
- `.🧬semio/🌐hub/s14-r10-state/window3-backups/st2-r10-2026-09-29T01-41-32.931Z/.vscode/🧩️launch.seed.jsonc`
- `.🧬semio/🌐hub/s14-r10-state/window3-backups/render-2026-09-29T01-41-49.754Z/.vscode/launch.json`
- `./.🧬semio/🌐hub/s14-sh2-overlay/Monorepo.sln`
- `.🧬semio/🌐hub/s14-sh2-overlay/.vscode/extensions.json`
- `.🧬semio/🌐hub/s14-sh2-overlay/.vscode/launch.json`
- `.🧬semio/🌐hub/s14-sh2-overlay/.vscode/mcp.json`
- `.🧬semio/🌐hub/s14-sh2-overlay/.vscode/settings.json`
- `.🧬semio/🌐hub/s14-sh2-overlay/.vscode/🧩️launch.seed.jsonc`
- `./.🧬semio/🌐hub/s15-rs1-overlay/Monorepo.sln`
- `.🧬semio/🌐hub/s15-rs1-overlay/.vscode/extensions.json`
- `.🧬semio/🌐hub/s15-rs1-overlay/.vscode/launch.json`
- `.🧬semio/🌐hub/s15-rs1-overlay/.vscode/mcp.json`
- `.🧬semio/🌐hub/s15-rs1-overlay/.vscode/settings.json`
- `.🧬semio/🌐hub/s15-rs1-overlay/.vscode/🧩️launch.seed.jsonc`
- `./.🧬semio/🌐hub/s14-wg11-overlay/Monorepo.sln`
- `.🧬semio/🌐hub/s14-wg11-overlay/.vscode/extensions.json`
- `.🧬semio/🌐hub/s14-wg11-overlay/.vscode/launch.json`
- `.🧬semio/🌐hub/s14-wg11-overlay/.vscode/mcp.json`
- `.🧬semio/🌐hub/s14-wg11-overlay/.vscode/settings.json`
- `.🧬semio/🌐hub/s14-wg11-overlay/.vscode/🧩️launch.seed.jsonc`

The exact 15 reviewed paths were removed with an explicit `rm -r --` path list, which exited 0. Post-removal filesystem inspection confirmed all selected paths absent, every containing source directory present and all three upstream Lean dependency directories still present. No Git state was changed.

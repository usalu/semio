# Authored Public Nx Interface Census

Read-only census examined 1549 authored interface/configuration/agent-document files (10725737 bytes), each bounded to16MiB, excluding caches, installed dependencies, tickets, tests, patches, AGENTS and the sanctioned private caching/Nx owner subtree. Scope includes all package/project/launch JSON, CI YAML, devcontainer and Docker/sh bootstrap files, agent plan markdown, root README and the dashboard Rust entry. The exact observed paths/SHA-256 values and all candidate lines are retained in `🗑️generated/authored-nx-interface-audit.json`.

No executable direct Nx command was found in this bounded authored-interface census. Package scripts/actual project commands and launch command strings route through neutral `bun repo`/`📜️script.ts` owners; Nx-specific target/test names and `bun repo tools setup nx` are declared neutral commands, not direct Nx bypasses. A separate script/Rust/MJS scan found only the neutral build router's sanctioned private Nx-adapter dispatch at build script lines131,137,166–169: setup-version, selected Nx reset and the explicitly selected Nx task branch. No remaining `NX_TASK_PROJECT`/`NX_TASK_TARGET` recursion guard was found outside the excluded implementation/tests.

Two documentation surfaces retain direct-Nx advice:

- `🌎️hub/Dockerfile:18` says `bun nx run os-hub:dev` is the development-container entrypoint. This is a comment, not a RUN/CMD/ENTRYPOINT. Line8 is explicitly historical; line71 discusses a bypass that the runtime image avoids. The same Dockerfile still carries `ENV NX_DAEMON=false` at65, but no Nx executable command is launched there.
- `.codex/plans/declarative-actions.md:52` instructs agents to run `bun nx run @spatial/js-core:test` and `bun nx run @spatial/js-kernel-brepjs:test`. This is current plan text, not an executable script, and is a remaining direct-Nx user/agent instruction. A later authorized edit should express the same target intents through `bun repo run ...`; no target-existence/runtime claim was inferred in this audit.

No fixes were made. These two advisory leaks should not be reported as active direct executable paths. No Native, Tools, SDK, store, graph producer or runtime was acquired; no task success or Windows/Linux execution is claimed. Full static absence beyond the stated patterns/file scope is not inferred.

Root subsequently authorized the two advisory-line corrections. Both now use `bun repo run`; exact before/after and the two undeclared historical plan-target limitations are retained in `neutral-advisory-instructions.md`. The original census above is the pre-correction observation, not current line contents.

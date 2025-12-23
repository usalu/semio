---
slug: CICD-ANALYZE-FIX
summary: Integrate analyze/fix CI pipeline
prompt: >-
  The ci/cd system should be improved. The individual commands should work more
  closely together and be more integrated. Currently preflights runs all
  analysis and formatters. There should be two new commands: analyze and fix.
  Preflight runs both of them. Test should run preflight and then test. build
  should run test. prepublish and publish should run build. All scripts should
  have a skip mechnaism to skip preceeding individual steps. Adding a command
  always means updating all hooks, nx configs, .vscode tasks, launch.json, etc.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.950Z"
commit: "0000000000000000000000000000000000000000"
iterations:
  - prompt: >-
      The ci/cd system should be improved. The individual commands should work
      more closely together and be more integrated. Currently preflights runs
      all analysis and formatters. There should be two new commands: analyze and
      fix. Preflight runs both of them. Test should run preflight and then test.
      build should run test. prepublish and publish should run build. All
      scripts should have a skip mechnaism to skip preceeding individual steps.
      Adding a command always means updating all hooks, nx configs, .vscode
      tasks, launch.json, etc.
    date:
      started: "2025-12-15T08:58:34.318Z"
    model: gpt-5-2
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
  - prompt: >-
      All commands in .vscode should be available for all granularity levels
      (every child). The order is DEV, ANALYZE, FIX, PREFLIGHT, TEST, BUILD,
      PREPUBLISH, PUBLISH, UPDATE, COMMANDS
    date:
      started: "2025-12-15T09:27:27.418Z"
    model: gpt-5-2
    commit: 67126961d64c89450f396abedd5d477670f1ad4d
    files:
      updated:
        - path: .vscode/launch.json
          lines:
            added: 105
            removed: 27
        - path: .vscode/tasks.json
          lines:
            added: 105
            removed: 27
        - path: AGENTS.md
          lines:
            added: 105
            removed: 27
        - path: README.md
          lines:
            added: 105
            removed: 27
        - path: hooks/eslint.ts
          lines:
            added: 105
            removed: 27
        - path: log/tickets/2025/12/15/CICD-ANALYZE-FIX.md
          lines:
            added: 105
            removed: 27
        - path: nx.json
          lines:
            added: 105
            removed: 27
        - path: package.json
          lines:
            added: 105
            removed: 27
        - path: preflight.ts
          lines:
            added: 105
            removed: 27
        - path: scripts/log.ts
          lines:
            added: 105
            removed: 27
      created: []
      removed: []
    lines:
      added: 1050
      removed: 270
---

# Previously

# Plan

- Introduce `fix` and `analyze` as first-class root commands.
- Make `preflight` run `fix` then `analyze` and keep husky pre-commit on `preflight`.
- Make `test → preflight → test`, `build → test → build`, `prepublish/publish → build → <target>`.
- Add a consistent skip mechanism and Nx arg passthrough for scoping.
- Update hooks, Nx defaults, and VS Code tasks/launch to match.

# Changes

- Added `preflight.ts` as the orchestrator for the new pipeline and wired root scripts to it.
- Split the pipeline into `fix` (formatters/autofix) and `analyze` (non-mutating checks + reports).
- Added `--skip=...` and `--nx ...` passthrough to keep the pipeline composable and scoping-friendly.
- Updated VS Code tasks/launch entries and aligned ESLint hook to accept forwarded Nx args.

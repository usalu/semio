---
slug: DEV-DOCS-GIT-AI
prompt: "The git section of the dev docs is outdated. The git repo has a compressed main branch. If the release receives updates after main already has progressed, then a parallel release branch is created that works like main but for this release. The first symbol is a summary of the main task of the commit. The last symbol is encoded the amount of work (\U0001FA9B\U0001F528\U0001F6E0️\U0001F3D7️). The ai part is outdated. Due to token vs request based we use mainly copilot for most tickets, windsurf for the most token-heavy test-driven-development workflows with mcp (such as playwright), claude code for small bugs, cursor when docs are needed and as main editor with tab autocomplete, codex for simple tasks. opus 4.5 is the current model. gpt 5.2 alternative."
summary: Update git + AI dev docs
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2025-12-16T17:06:07.925Z"
commit: "0000000000000000000000000000000000000000"
iterations:
    - prompt: "The git section of the dev docs is outdated. The git repo has a compressed main branch. If the release receives updates after main already has progressed, then a parallel release branch is created that works like main but for this release. The first symbol is a summary of the main task of the commit. The last symbol is encoded the amount of work (\U0001FA9B\U0001F528\U0001F6E0️\U0001F3D7️). The ai part is outdated. Due to token vs request based we use mainly copilot for most tickets, windsurf for the most token-heavy test-driven-development workflows with mcp (such as playwright), claude code for small bugs, cursor when docs are needed and as main editor with tab autocomplete, codex for simple tasks. opus 4.5 is the current model. gpt 5.2 alternative."
      model: claude-opus-4-5
      date:
        started: "2025-12-14T22:42:35.169Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 7765b633fe739bc29cd811ac7ec884e782e2e945
      bundles:
        '@semio':
            files:
                AGENTS.md:
                    sections:
                        _root:
                            lines:
                                added: 64
                                removed: 54
                README.md:
                    sections:
                        _root:
                            lines:
                                added: 64
                                removed: 54
      files:
        updated:
            - path: AGENTS.md
              lines:
                added: 64
                removed: 54
            - path: README.md
              lines:
                added: 64
                removed: 54
      lines:
        added: 128
        removed: 108
---


# Previously

- Git and AI sections in dev docs drifted from current workflow/tooling.
- Dev docs mixed monorepo-wide mechanisms with JavaScript/Sketchpad-specific implementation details.

# Plan

- Update Git docs to reflect compressed `main`, parallel release branches, and commit symbol conventions.
- Update AI docs to reflect current tool split (Copilot/Windsurf/Claude Code/Cursor/Codex) and model defaults (Opus 4.5 / GPT-5.2).
- Move JavaScript-only operational details out of general Development docs into Ecosystems/Components sections.
- Mirror the repo-wide Git/AI guidance in `AGENTS.md`.

# Changes

- Updated `README.md` Git docs: compressed `main`, parallel `release/rYY.MM-V`, commit message symbol format and work-scale symbols.
- Updated `README.md` AI docs: current tool choices (Copilot/Windsurf/Claude Code/Cursor/Codex) and model defaults (Claude Opus 4.5, GPT-5.2 Codex).
- Refactored `README.md` monorepo docs: moved Sketchpad implementation details into `@semio/js`, expanded Ecosystems with workspace + preflight examples, fixed repo path links (`net/`, `py/`, `js/js`, `js/desktop`, `js/play`).
- Updated `AGENTS.md` with repo-wide Git and AI guidance aligned with README.

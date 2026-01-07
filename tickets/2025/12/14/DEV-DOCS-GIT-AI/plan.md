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

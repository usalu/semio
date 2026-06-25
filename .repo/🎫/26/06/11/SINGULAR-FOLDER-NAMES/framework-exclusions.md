# Framework folder exclusions (must stay plural / fixed names)

These paths are owned by external tools and MUST NOT be renamed during singular-folder migrations:

- `.cursor/plans` — Cursor plan files
- `.cursor/agents` — Cursor agent definitions
- `.copilot/plans` — Copilot plans
- `.codex/plans` — Codex plans
- `.github/agents` — GitHub Copilot agents
- `.github/hooks` — GitHub hooks
- `.github/workflows` — GitHub Actions (required name)
- `.kiro/agents` — Kiro agents
- `.kiro/settings` — Kiro settings
- `.agents/skills` — Cursor agent skills
- `.storybook/stories` — Storybook story glob root
- `coda/.agent/skills` — Coda agent skills
- `node_modules/**` — third-party package paths (e.g. `three/examples`, `lucide-static/icons`)

Project-owned folders (asset, compose/fixture, ui/asset/icon, …) remain singular.

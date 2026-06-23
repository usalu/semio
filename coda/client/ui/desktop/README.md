# Summary

Electron desktop shell for visualizing and interacting with coda (ACC design assistant). Displays project, runs, iterations, report, breachs, measures, targets, platforms and provides actions for compliance checking workflows.

# Docs

<details>
<summary><strong>📚 Resources:</strong></summary>

- [Electron](https://www.npmjs.com/package/electron) - `npm`
  - [Docs](https://www.electronjs.org/docs) - `official`
  - [API](https://www.electronjs.org/doc/latest/api/app) - `reference`
- [Electron Forge](https://www.npmjs.com/package/electron-forge) - `npm`
  - [Docs](https://www.electronforge.io/docs) - `official`
- [React](https://react.dev) - `official`
- [Tailwind CSS](https://tailwindcss.com) - `official`

</details>

# Specs

- MUST mirror compose desktop architecture (Electron Forge, Vite, React, Tailwind).
- MUST communicate with coda MCP server via HTTP (streamable-http on 127.0.0.1:8080).
- MUST visualize project, runs, iterations, report, breachs, measures, targets, platforms.
- MUST allow invoking coda tools: start_run, start_iteration, translate, validate, save_translation, save_report, fix.
- MUST have frameless window with custom title bar and window controls.
- MUST have sidebar navigation with collapsible support.
- MUST display compliance status with colored badges (compliant/violated/unknown).
- MUST provide expandable sections for rules, clauses, targets, and platforms.
- MUST include action log for tracking tool invocations and results.
- MUST support dark mode via CSS custom properties.

# 💯Requirements

## Pages

- **Dashboard** — Overview with stat cards (design, run, iteration, compliance) and summary report.
- **Config** — Measures, targets (with properties, rules, clauses), and platforms (with MCP tool instructions).
- **Runs** — Current run details, iterations list, current iteration targets.
- **Report** — Full compliance report with violations, compliant rules, and breach data.
- **Translations** — Per-target translation JSON output from the current iteration.
- **Actions** — Start run, start iteration, translate/validate per target, fix design with prompt.

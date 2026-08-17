# Wave 5a fix-up (orchestrator)

Fixed the one real regression the verify agent found:
`ShellHost/🟦️component.tsx:4729,4736` still passed the literal string
`"ui.search.category.studio"` to `shellLabel(...)` after the icons-i18n-e2e
agent renamed the schema key to `hostApp`. Updated both call sites to
`"ui.search.category.hostApp"` (confirmed the exact key path against the
renamed `I18n/🟦️component.tsx:114` field and the react locale bundle's
`category: { ..., hostApp: {...} }` nesting). Scoped `tsc` check confirms
zero `studio`/`hostApp` mismatch errors remain (the ~235 other errors this
check surfaces are pre-existing project-graph noise, already documented by
the original agent as unconnected).

## Wave 5a disposition
- **Playbook relocation**: correctly deferred with zero edits — a genuine
  architecture decision is needed first (os-flow's own `vcs` component
  depends on `playbook::` internally; 32 files across 4 plugin crates
  consume `flow::playbook::*` without depending on the playbook plugin).
  Recording as a standing follow-up, not attempting to force it.
- **puzzle-5d-react move**: complete, verified.
- **launch.json/package.json**: partial by design (15 duplicate scripts
  removed; full generation needs new Cargo.toml metadata across 58 crates,
  scoped as a dedicated follow-up ticket).
- **icons/i18n/e2e**: landed + regression fixed. Remaining smaller
  follow-ups noted (icon barrel re-export wiring for Storybook, e2e region
  physical move) — cosmetic/non-blocking.

Proceeding to Wave 5b (flow-core relocation + extension-world prototype).

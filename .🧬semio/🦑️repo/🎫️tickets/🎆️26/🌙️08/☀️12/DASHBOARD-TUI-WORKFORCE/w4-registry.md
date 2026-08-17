# W4a — Dashboard Registry

## Done
- `registry` region in `📦️glue.rs`: builds tasks from root verbs + playgrounds, detects `cursor-agent` / `claude` / `codex`, writes `🤖️generated/🎛️dashboard.json`.
- Hooked into `semio plugin registry generate` (after the TS nx generate succeeds).

## Test
`registry_build_includes_agents_and_verbs`

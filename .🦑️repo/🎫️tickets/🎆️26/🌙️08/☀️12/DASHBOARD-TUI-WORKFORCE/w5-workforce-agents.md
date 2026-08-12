# W5 — Workforce Scheduler + Agent Runners

## Done
- `workforce` region: `WorkflowSpec` / waves / tasks, `Scheduler` with concurrency + path-scope claims + dependency unlock, ticket `🌊️workflow.json` load/save.
- `agent_runner` region: `AgentRunner` trait + `cursor-agent` / `claude -p` / `codex exec` adapters with PATH detection.
- `semio workflow status|run --ticket <dir>` wired in dispatch.

## Tests
- `workforce_scheduler_respects_scope_and_deps`
- `agent_runner_detect_returns_only_available`

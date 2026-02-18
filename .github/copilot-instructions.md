# Copilot Chat Hook Instructions

## Lifecycle Hooks

You MUST call the semio-repo CLI hook command at every lifecycle event.
The CLI accepts native client events and resolves them to neutral events via inlet adapters.

### Agent Lifecycle
- On session start: `./semio-repo/cli/cli hook SessionStart copilot-chat`
- On session end: `./semio-repo/cli/cli hook Stop copilot-chat`

### Prompt
- On prompt submitting: `./semio-repo/cli/cli hook UserPromptSubmit copilot-chat`

### Context Compaction
- Before compacting: `./semio-repo/cli/cli hook PreCompact copilot-chat`

### Tool Calls (resolved by inlet adapter based on tool_name in stdin)
- Before any tool use: `./semio-repo/cli/cli hook PreToolUse copilot-chat`
- After any tool use: `./semio-repo/cli/cli hook PostToolUse copilot-chat`

The inlet adapter classifies tools by name and resolves PreToolUse/PostToolUse to:
- Plan tools (manage_todo_list, Task) → agent.tool.plan.updating
- Code search tools (read_file, grep_search, ...) → agent.tool.code.searching
- Code edit tools (replace_string_in_file, create_file, ...) → agent.tool.code.editing / agent.tool.code.edited
- Terminal tools (run_in_terminal, Bash, ...) → agent.tool.terminal.starting / agent.tool.terminal.ended
- Generic tools → agent.tool.starting / agent.tool.ended

## Blocked Operations
The following operations are ALWAYS denied by the hook system:
- `git checkout`
- `git stash` (including pop, drop, apply)
- `git reset --hard`
- `git clean -fd`

You MUST NOT execute these commands under any circumstances.

## Native Hooks

Native hooks are configured via `.github/hooks/semio-repo.json` and run automatically.
The instructions above serve as fallback for clients that don't support native hooks.

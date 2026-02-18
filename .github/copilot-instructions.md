# Copilot Chat Hook Instructions

## Lifecycle Hooks

You MUST call the semio-repo CLI hook command at every lifecycle event:

### Agent Lifecycle
- On session start: `./semio-repo/cli/cli hook agent.starting copilot-chat`
- On session end: `./semio-repo/cli/cli hook agent.ended copilot-chat`

### Prompt
- On prompt submit: `./semio-repo/cli/cli hook prompt.submit copilot-chat`

### Context Compaction
- Before compacting: `./semio-repo/cli/cli hook compacting copilot-chat`

### Tool Calls
- Before calling any tool (except code reading/editing): `./semio-repo/cli/cli hook tool.calling copilot-chat --tool-name "<tool>" --tool-args "<args>"`
- After tool completes: `./semio-repo/cli/cli hook tool.ended copilot-chat --tool-name "<tool>"`

### Code Operations
- Before reading code: `./semio-repo/cli/cli hook code.reading copilot-chat --file "<path>"`
- After editing code: `./semio-repo/cli/cli hook code.edited copilot-chat --file "<path>"`

### Notifications
- On notification: `./semio-repo/cli/cli hook notification copilot-chat`

## Blocked Operations
The following operations are ALWAYS denied by the hook system:
- `git checkout`
- `git stash` (including pop, drop, apply)
- `git reset --hard`
- `git clean -fd`

You MUST NOT execute these commands under any circumstances.

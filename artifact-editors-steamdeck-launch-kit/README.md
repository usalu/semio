# Well-known artifact editors: Steam Deck launch kit

Get every repository-defined well-known artifact editor running through its real create/open, edit, save/reopen, and supported import/export workflows. The complete prompt also covers runtime dependencies, editor dispatch, lifecycle, data integrity, actual application verification, and cooperation with concurrent agents.

The editor list comes from the repository's canonical registries, schemas, manifests, documentation, and tests. The kit does not presume which editor types your project contains.

## Extract and run

Extract the ZIP directly into the repository root, preserving the folder:

`artifact-editors-steamdeck-launch-kit/`

On the Steam Deck, open your usual Bash terminal in Desktop Mode, change into the repository root, and run:

```bash
bash ./artifact-editors-steamdeck-launch-kit/run.sh
```

The launcher deliberately uses the kit directory's parent as the working repository. Keep the kit directly inside that root, without an extra nested extraction folder. Calling it through `bash` avoids relying on executable permission bits preserved by the ZIP extractor.

It reads the prompt directly through standard input, uses quoted paths, sets medium effort and the 20-subagent cap, and passes through Claude's output and exit code. Its directory and environment changes remain within the child process.

## Explicit command

From the repository root, the equivalent one-line Bash command is:

```bash
CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20 CLAUDE_CODE_EFFORT_LEVEL=medium claude -p --model claude-opus-5-5 --effort medium --agents ./artifact-editors-steamdeck-launch-kit/agents.json --allowedTools 'Agent,Read,Glob,Grep,Write,Edit,Bash,WebSearch,WebFetch' < ./artifact-editors-steamdeck-launch-kit/prompt.txt
```

Or use this multiline form:

```bash
CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20 \
CLAUDE_CODE_EFFORT_LEVEL=medium \
claude -p \
  --model claude-opus-5-5 \
  --effort medium \
  --agents ./artifact-editors-steamdeck-launch-kit/agents.json \
  --allowedTools 'Agent,Read,Glob,Grep,Write,Edit,Bash,WebSearch,WebFetch' \
  < ./artifact-editors-steamdeck-launch-kit/prompt.txt
```

Each backslash must be the final character on its line. These are Bash commands, not PowerShell syntax.

## Claude Code requirements

Use the Claude Code installation and authentication already available in your terminal. This kit does not select a different authentication or billing route.

The `--agents` JSON-file form in print mode requires **Claude Code v2.1.281 or later**. Check with `claude --version`; update an older supported installation with `claude update`. The requested models must be available to your account/provider.

The allowed-tools argument preapproves the named tools; it is not an exclusive tool allowlist. Existing managed policy and runtime permissions still apply.

## Fleet

| Role | Model | Effort | Count |
| --- | --- | --- | --- |
| Main coordinator | Claude Opus 5.5 | Medium | 1 |
| Shared systems and editor implementation | Claude Sonnet 5.5 | Medium | 14 |
| Read-only exploration and audits | Claude Haiku 5.5 | Medium | 4 |
| Bounded registrations and documentation | Claude Haiku 5.5 | Medium | 2 |

The main coordinator is additional to the 20 worker subagents. The eight implementation batches are assigned from the actual editor inventory; the prompt never assumes eight particular artifact formats.

Every profile specifies a full model ID, medium effort, and background execution. All deny the Agent tool, with additional instructions against spawning through other means. Four Haiku profiles have an explicit read-only tool list.

`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20` sets a cap; `agents.json` defines profiles. Neither dispatches workers by itself. The prompt explicitly requires Opus to launch all 20 and maintain useful work, and to report actual observed concurrency if runtime/account constraints prevent that.

Avoid a global forced subagent model if you want the configured Sonnet/Haiku split. The prompt requests named profiles without model-overriding forks or per-call overrides.

## Steam Deck resource coordination

The prompt preserves the 20-worker fleet while scheduling heavy local jobs through one shared verification queue. It initially allows one full build/test/render/export job or one shared development-server/browser verification session at a time. Other agents continue inspecting, editing, reviewing, and preparing checks.

It also assigns a single dependency/setup owner, reuses healthy services, and asks workers to avoid conflicting installations or writes to shared caches. No SteamOS system changes, extra agents, or additional software are performed by the launcher.

## Required result

- A complete registry-derived editor/capability coverage matrix.
- Correct resolution and opening through real application entry points.
- Meaningful edits through each editor's canonical model and declared capabilities.
- Durable saving verified by close/reopen and appropriate reload/restart checks.
- Valid supported imports and exports with type-specific semantics preserved.
- Working lifecycle/history, accurate dirty/error states, and protection against stale work or lost data.
- Reproducible runtime, focused regressions, and actual application verification for every editor.
- Integration with concurrent edits, plus exact startup/check commands, evidence, and concrete remaining blockers.

## Files

- `prompt.txt`: complete implementation task and acceptance criteria.
- `agents.json`: 20 worker definitions.
- `fleet.md`: ownership and dispatch rules.
- `run.sh`: Bash launcher, UTF-8 without BOM and LF line endings.
- `README.md`: this guide.

Official documentation: [CLI reference](https://code.claude.com/docs/en/cli-reference), [subagents](https://code.claude.com/docs/en/sub-agents), and [environment variables](https://code.claude.com/docs/en/env-vars).

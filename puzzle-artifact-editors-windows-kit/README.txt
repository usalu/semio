PUZZLE ARTIFACT EDITORS - WINDOWS CLAUDE CODE KIT
Prepared 2026-11-04

FILES
prompt.txt   Complete task prompt and completion criteria.
agents.json  Three reusable worker profiles, instantiated as a 20-worker fleet.
run.ps1      PowerShell launcher; UTF-8 input, version check, optional API spend cap.

QUICK START
Extract this archive into the repository root. In PowerShell at that root, run:

    .\puzzle-artifact-editors-windows-kit\run.ps1

You can also keep the kit elsewhere and choose the repository explicitly:

    & "C:\path\puzzle-artifact-editors-windows-kit\run.ps1" -RepositoryPath "C:\path\your-repository"

Use the existing Claude Code authentication. This kit does not set API keys.
If you need to inspect the current authentication method:

    claude auth status --text

Requires Claude Code 2.1.293 or later; current releases are recommended.
To update an older installation:

    claude update

The launcher checks its version before submitting the task. It uses a JSON file
with --agents in print mode, avoiding native Windows JSON argument escaping.
It restores the process environment, encodings, and working directory when done.

FLEET
Main session: Opus 5.5 Medium.
Workers: 12 Sonnet 5.5 Medium, 6 read-only Haiku 5.5 Medium,
and 2 Haiku 5.5 Medium small-fix workers.
All models are pinned by full model ID in the applicable configuration.

The concurrency variable sets a ceiling, not an automatic scheduler.
The prompt directs Claude to create and refill all 20 worker slots.
Actual concurrency depends on model behavior, dependencies, account limits,
and available work. Existing settings or organization policy can override
requested settings; the prompt requires reporting observable deviations.

USAGE
Print mode (-p) does not determine billing. API-key authentication uses API billing,
separate from a subscription weekly allowance. An ANTHROPIC_API_KEY environment
variable takes precedence over subscription sign-in.

The default launcher sets no session spend cap. To request an optional API cap:

    .\puzzle-artifact-editors-windows-kit\run.ps1 -MaxBudgetUsd 50

The cap uses Claude Code's estimated API spend, includes subagents, and can be
exceeded by in-flight spending. It is not an exact billing ceiling.

The task prompt prioritizes aggressive useful throughput and continued work
alongside concurrent repository changes. Task completion ends the run; artificial
work solely to exhaust quota is excluded.

VERIFICATION
The kit's JSON, model/effort assignments, read-only tool restrictions, and archive
structure were checked. The launcher was reviewed against the documented CLI.
It was not executed in a Windows Claude Code installation or your repository.

OFFICIAL REFERENCES
CLI flags and --agents JSON file support:
https://code.claude.com/docs/en/cli-reference

Subagent fields, effort, tool restrictions, and concurrency:
https://code.claude.com/docs/en/sub-agents

Model IDs, supported effort levels, and minimum versions:
https://code.claude.com/docs/en/model-config

Environment variable precedence and Windows shell support:
https://code.claude.com/docs/en/env-vars

Authentication and API billing:
https://support.claude.com/en/articles/12304248-manage-api-key-environment-variables-in-claude-code


# Layout plugin launch kit

## Run

Extract this archive into the repository containing the existing layout plugin. Run this command from the repository root with Claude Code authentication already configured:

```bash
CLAUDE_CODE_EFFORT_LEVEL=medium claude -p --model claude-opus-5-5 --effort medium --agents ./layout-plugin-launch-kit/agents.json --allowedTools 'Agent,Read,Glob,Grep,Write,Edit,Bash,WebSearch,WebFetch' < ./layout-plugin-launch-kit/prompt.txt
```

## Contents

- prompt.txt: complete instructions for finding and repairing the plugin, loading it in its actual host, verifying its layout workflows and supported lifecycle, and delivering a reproducible package.
- agents.json: task-specific Sonnet 5.5 implementation workers, Haiku 5.5 read-only reviewers, and Haiku 5.5 helpers, all configured with medium effort.

The command selects Opus 5.5 Medium for coordination. The prompt asks it to spawn multiple workers for independent tasks and verify the complete plugin after integration.

The prompt derives the host application, framework, plugin contract, and intended features from the repository. Host-specific requirements apply only where supported by that contract.

Completion requires actual host execution, verified layout results, and reproducible building and installation. External blockers and unverified workflows must be reported explicitly.

This kit contains launch instructions and agent definitions. The plugin source and host environment remain in your project.

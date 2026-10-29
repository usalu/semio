# Staffage launch kit

This package prepares a Claude Code run; it does not build or execute the website in ChatGPT.

## Contents

- prompt.txt: the complete build brief.
- agents.json: reusable Sonnet 5.5 and Haiku 5.5 worker definitions.
- README.md: launch instructions and decisions.

Extract the staffage-launch-kit directory into your existing project directory. Keep the terminal working directory at the project root, so Claude works on that project rather than on a new empty folder.

## Existing authentication

You do not need to supply the API key again if Claude Code already has the correct key-backed authentication.

Run:

    claude auth status

For the included Max API credit, use an API key from the Console organization linked to your Max plan. The JSON status field authMethod identifies api_key or api_key_helper for key-backed authentication. Confirm the linked organization independently in Console; authentication status alone does not establish that a key belongs to the organization receiving your credit.

A claude.ai subscription login uses subscription limits. Current Console login can also use a keyless OAuth profile; the monthly-credit FAQ specifically documents API-key use. Do not post credentials in chat.

## Run in Bash, zsh, or WSL

    CLAUDE_CODE_EFFORT_LEVEL=medium \
    claude -p \
      --model claude-opus-5-5 \
      --effort medium \
      --agents ./staffage-launch-kit/agents.json \
      < ./staffage-launch-kit/prompt.txt

The run uses existing authentication and tool permissions. Tools required for implementation and verification must be authorized under your project's Claude Code policy. The package does not grant new tool permissions or disable managed policies.

The worker definitions configure model and effort. The prompt instructs the coordinator to spawn multiple instances, preserve model assignments, and report actual work. Definitions do not guarantee worker counts or act as an immutable model policy. Existing organization/model restrictions and global subagent-model overrides can affect runtime choices. Check the runtime output if exact routing matters.

Use current Claude Code: the documented minimum covering these three 5.5 models is v2.1.293. The --agents file argument itself requires v2.1.281 or later.

## Run in PowerShell

    $env:CLAUDE_CODE_EFFORT_LEVEL = "medium"
    Get-Content -Raw ./staffage-launch-kit/prompt.txt | claude -p --model claude-opus-5-5 --effort medium --agents ./staffage-launch-kit/agents.json

## Scope decisions

- Four graphic styles plus three analysis modes produce seven named views.
- Initial content target: 48 distinct assets across six categories. This is an editable assumption added to make completion measurable.
- Real dimensions remain separate from print scale.
- Thermal fields distinguish temperature, radiation, sensible heat, and latent heat.
- "Structural momentum" is interpreted as the requested structural moment/torque.
- Per-pose and per-clip definitions drive deterministic fields throughout animation.
- Genuine DWG requires a real writer/converter; unsupported dependencies must be reported.
- Missing design references are handled through explicit original design choices.
- No quota-burning instruction or simultaneous uncoordinated shared-file editing instruction is included.

## Authentication and execution details

Self-run claude -p and Agent SDK requests with the linked organization's API key are eligible for the included credit. Interactive Claude Code, including native Agent Teams, is excluded. Native Agent Teams cannot spawn teammates in print mode; this kit uses ordinary subagents.

Do not add the earlier connection-test flags --bare or --tools ''. Bare mode disables background subagents, and an empty tool set removes delegation.

For an optional per-run budget, add --max-budget-usd AMOUNT to the command. This is an estimated stop threshold, includes subagent spending, can be exceeded by in-flight requests, and does not automatically cap the organization at its remaining promotional balance. Purchased credits or auto-reload may fund continued usage after promotional credits are spent.

## Reference documentation checked 2026-10-29

- Monthly credits and claude -p eligibility:
  https://support.claude.com/en/articles/17154008-monthly-api-credits-for-max-and-team-plans
- Model IDs and version requirements:
  https://code.claude.com/docs/en/model-config
  https://platform.claude.com/docs/en/models/overview
- Subagent JSON, models, effort, and CLI definitions:
  https://code.claude.com/docs/en/sub-agents
- CLI flags and authentication status:
  https://code.claude.com/docs/en/cli-reference
- Authentication choices:
  https://code.claude.com/docs/en/authentication
- Print mode and bare-mode behavior:
  https://code.claude.com/docs/en/headless
- Agent Teams limitations:
  https://code.claude.com/docs/en/agent-teams
- Moment of force:
  https://engineeringstatics.org/moment-of-force.html
- Evapotranspiration:
  https://www.usgs.gov/water-science-school/science/evapotranspiration-and-water-cycle
- Genuine DWG tooling:
  https://aps.autodesk.com/developer/overview/realdwg-api


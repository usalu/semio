# Plan: Persist Claude Code and Codex Auth Across Devcontainer Rebuilds

## Problem Analysis

The current devcontainer mounts are targeting the **wrong directories**:

| Tool | Current Mount (Wrong) | Actual Data Location |
|------|----------------------|---------------------|
| Claude Code | `/home/vscode/.config/claude` | `/home/vscode/.claude/` + `/home/vscode/.claude.json` |
| Codex | `/home/vscode/.config/codex` | `/home/vscode/.codex/` |

Current mounts in `devcontainer.json`:
```json
"source=${localWorkspaceFolderBasename}-config-claude,target=/home/vscode/.config/claude,type=volume",
"source=${localWorkspaceFolderBasename}-config-codex,target=/home/vscode/.config/codex,type=volume",
```

These directories are **empty** because Claude Code and Codex don't use `~/.config/` paths.

## Solution

Update the mounts to target the correct directories:

1. **Claude Code**:
   - Mount `~/.claude/` (contains credentials, projects, sessions, chat history)

2. **Codex**:
   - Mount `~/.codex/` (contains skills, tmp data)

## Implementation Steps

1. Update `devcontainer.json` mounts section:
   - Change `target=/home/vscode/.config/claude` → `target=/home/vscode/.claude`
   - Change `target=/home/vscode/.config/codex` → `target=/home/vscode/.codex`

2. Fix volume ownership in post-start.sh (volumes mount as root initially)

## Files to Modify

- `.devcontainer/devcontainer.json` - Fix mount targets
- `.devcontainer/post-start.sh` - Add ownership fix for mounted volumes

# Ticket

## Todos
# Plan

1. Inspect devcontainer mounts and startup scripts to locate Claude Code and Codex persistence gaps.
2. Add persistent mounts and startup fixes for auth/session storage and editor extension state.
3. Update README.md and AGENTS.md with the persistence mechanism details.
4. Log progress and prepare summary for ticket close.

## Changes

## Log
# Log: Persist Claude Code and Codex Auth

## Investigation

Checked current devcontainer mount configuration and discovered the mounts were targeting wrong directories:

**Current (broken) mounts:**
- `/home/vscode/.config/claude` - empty, not used by Claude Code
- `/home/vscode/.config/codex` - empty, not used by Codex

**Actual data locations:**
- Claude Code: `/home/vscode/.claude/` (contains `.credentials.json`, projects, sessions, chat history)
- Codex: `/home/vscode/.codex/` (contains skills, tmp)

## Changes Made

### 1. Updated `devcontainer.json` mounts

Changed mount targets from `~/.config/*` to actual tool directories:

```diff
- "source=${localWorkspaceFolderBasename}-config-claude,target=/home/vscode/.config/claude,type=volume",
- "source=${localWorkspaceFolderBasename}-config-codex,target=/home/vscode/.config/codex,type=volume",
+ "source=${localWorkspaceFolderBasename}-claude,target=/home/vscode/.claude,type=volume",
+ "source=${localWorkspaceFolderBasename}-codex,target=/home/vscode/.codex,type=volume",
```

### 2. Updated `post-start.sh` ownership fixes

Added chown commands to fix volume ownership (volumes mount as root):

```bash
sudo chown -R vscode:vscode /home/vscode/.claude 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codex 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/gh 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/cursor 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/antigravity 2>/dev/null || true
```

---

## Update: Added Windsurf/Codeium Support

### Investigation

Windsurf stores its config in `/home/vscode/.codeium/windsurf`, not in `.config`.

### Changes Made

Added mount for Windsurf/Codeium config:

```diff
+ "source=${localWorkspaceFolderBasename}-codeium,target=/home/vscode/.codeium,type=volume"
```

Added ownership fix:

```bash
sudo chown -R vscode:vscode /home/vscode/.codeium 2>/dev/null || true
```

## Note

After rebuilding the devcontainer, you will need to sign in **once** to Claude Code, Codex, and Windsurf. The volumes will then persist authentication across future rebuilds.

## 2026-01-20

- Continued ticket: persist Codex and Claude Code auth across devcontainer rebuilds.
- Observed ticket reopen blocked by missing UI metadata; updated ticket.json iterations with ui to proceed.
- Reopened ticket and updated plan.
- Added devcontainer mounts for Codex/Claude-related config and editor server persistence; normalized Claude auth file storage in post-start.
- Updated README.md and AGENTS.md to document devcontainer persistence requirements and implementation.
- Normalized devcontainer shell scripts to use region markers and removed inline comments to match code hygiene rules.
- Ensured Claude auth files are moved into the persisted Claude volume before linking to avoid loss on rebuilds.
- Closed ticket via repo CLI; GitHub issue 150 could not be updated by CLI in this environment.

## Summary
Persisted Codex and Claude Code state across devcontainer rebuilds by adding volumes for editor server storage and OpenAI config, and normalizing Claude auth files into the persisted Claude volume with symlinks on start. Updated devcontainer scripts to use region markers and removed inline comments for code hygiene compliance. Documented the persistence mechanism in README.md and AGENTS.md.

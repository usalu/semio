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

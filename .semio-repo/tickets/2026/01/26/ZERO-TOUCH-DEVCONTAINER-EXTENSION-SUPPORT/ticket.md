# Ticket

## Todos

## Changes
- Updated `.devcontainer/post-attach.sh` to install the VSIX into every detected IDE CLI so Cursor, Windsurf, and Antigravity receive the extension even if another CLI is first in PATH.
- Documented the multi-IDE install behavior in `README.md` and `AGENTS.md`.

## Log
- Reported: Cursor terminal still picked Antigravity CLI, so extension never appeared in Cursor.
- Switched to installing across all detected IDE CLIs instead of selecting a single CLI.

## Summary

Installed the VSIX across all detected IDE CLIs and documented multi-IDE installs for Cursor, Windsurf, Antigravity, and VS Code.

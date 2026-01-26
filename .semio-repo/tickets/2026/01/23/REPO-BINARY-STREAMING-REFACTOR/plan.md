# Repo Binary Streaming Refactor Plan

1. Rebuild repo CLI binary from go/repo with package main and rerun Go tests. ✅
2. Run repo CLI preflight subcommands (test, build, publish:test) with rebuilt binary. ⏳
3. Run root repo test entrypoint and verify all commands complete successfully. ⏳
4. Run VS Code extension build/test scripts and restore extension run workflow. ⏳
5. Update README.md and AGENTS.md with any new repo CLI build/test details; update ticket.md summary; close ticket. ⏳

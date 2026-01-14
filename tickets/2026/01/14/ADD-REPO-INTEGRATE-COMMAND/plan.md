# Plan - ADD-REPO-INTEGRATE-COMMAND

Add a new command to `repo` called `integrate <source> <target-section-name> <target-file> [<target-parent-section-name>]` that takes code files and integrates the source code into a target file by wrapping it into the target section.

## Tasks

1. [ ] Implement `Integrate` logic in `go/repo/tools/sections.go`.
   - Read source file.
   - Read target file.
   - Detect language for section markers.
   - Create new section.
   - Find insertion point (end of file or end of parent section).
   - Write updated target file.
2. [ ] Add `integrate` command to `go/cli/main.go`.
3. [ ] Expose `integrate` tool in `go/mcp/main.go`.
4. [ ] Update `AGENTS.md` with new command documentation.
5. [ ] Update `README.md` if necessary.
6. [ ] Verify implementation with a test run.

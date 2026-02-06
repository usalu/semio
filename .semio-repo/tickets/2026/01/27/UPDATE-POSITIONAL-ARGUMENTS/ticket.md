# Ticket

## Todos

- [x] Update VS Code extension ticket open command signature.
- [x] Update AGENTS.md documentation for ticket and goal commands.
- [x] Update ./semio-repo/cli/main.go goal open command signature and extraction logic.
- [x] Rebuild repo binary.

## Changes

- [x] `js/vscode/extension.ts`
- [x] `AGENTS.md`
- [x] `./semio-repo/cli/main.go`

## Log

- Updated `js/vscode/extension.ts` to swap UI and LLM arguments in `ticketOpen` command prompt.
- Updated `AGENTS.md` to reflect `ticket_open`, `ticket_reopen`, and `goal_open` signature changes (`<ui> <llm?>`).
- Updated `./semio-repo/cli/main.go` `goalCommand` to swap `extractUIFromArgs` and `extractLLMFromArgs` call order and make LLM optional.
- Rebuilt `repo` binary.

## Summary

Updated positional arguments for ticket open and goal open commands to support <ui> <llm?> pattern, making LLM optional and prioritizing UI.

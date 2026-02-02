---
prompt: Add a --md flag to output the result in markdown format
status: open
created: 2026-02-02
---

# Ticket

## Todos

- [x] Add `--md` flag to Config struct and CLI command
- [x] Implement `MarkdownRenderer` struct
- [x] Implement `formatMarkdownResult` function to handle different result types
- [x] Wire up `MarkdownRenderer` in `renderStream`
- [x] Verify `bundle list --md` output
- [x] Verify `ticket list --md` output

## Changes

- `@semio-repo/go/main.go`: Added `Markdown` bool to `Config` struct. Added `MarkdownRenderer` struct and implementation. Added logic to switch to Markdown renderer when `--md` flag is present.

## Log

- Updated `Config` struct to include `Markdown` boolean field.
- Bound `--md` flag to `Config.Markdown`.
- Implemented `MarkdownRenderer` with `Header` and `Footer` logic.
- Implemented `formatMarkdownResult` to convert `Result` objects into Markdown strings with `semiorepo://` URIs.
- Integrated `MarkdownRenderer` into the `renderStream` function.
- Verified functionality with `bundle list --md` and `ticket list --md`.

## Summary

Implemented a new `--md` flag for the repo binary that outputs command results in Markdown format. The output uses `semiorepo://` URIs as required for MCP integration. This feature is available for list and tree commands.

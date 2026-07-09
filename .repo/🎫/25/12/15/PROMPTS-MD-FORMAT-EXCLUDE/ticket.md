# Ticket

## Todos

# Previously

`log/prompts.md` was formatted by Prettier (preflight + VS Code format on save).

# Plan

Exclude `**/prompts.md` via `.prettierignore` and ensure VS Code Prettier uses the same ignore file.

# Changes

- Updated `.prettierignore` to ignore `**/prompts.md`.
- Updated `.vscode/settings.json` to set `prettier.ignorePath` to `.prettierignore`.
- Updated `hooks/prettier.ts` to pass `--ignore-path .prettierignore`.

## Changes

## Log

## Summary

# Summary

Exclude prompts.md from formatting

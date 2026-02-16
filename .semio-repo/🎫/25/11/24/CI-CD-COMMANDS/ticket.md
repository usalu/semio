# Ticket

## Todos
# CI/CD Automation and Canonical Commands

## Context

- CI/CD automation, agents, and contributors must run the same canonical commands so package dependencies execute predictably.
- The new workflow bundle includes `dev`, `build`, `test`, `update`, `prepublish`, and `publish`, with only `dev` allowed to stay live.

## Changes

1. Added the missing `update` command to the root scripts so `npx nx run-many -t update` is available alongside the other targets and respects the existing Nx dependency graph.
2. Reverted workspace packages to their original scripts instead of forcing every bundle to adopt every command, respecting the instruction to leave a package's command set untouched when a command didn't exist.
3. Documented the contract and the "no new per-package scripts" policy in `README.md` and `AGENTS.md`.

## Notes

- `prepublish` is still the staging build that feeds `publish`, and `publish` remains a terminating delivery step.
- `prepublish` already publishes to the shared test registry that most package managers can consume before any `publish` run.

## Changes

## Log

## Summary
# Summary

CI/CD automation and canonical commands

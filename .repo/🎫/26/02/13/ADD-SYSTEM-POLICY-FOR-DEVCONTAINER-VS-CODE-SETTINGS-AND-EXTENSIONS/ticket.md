---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Added new "System" policy to repo CLI with Devcontainer > VSCode group containing two autofixable statutes: (1) Settings Outside Devcontainer - detects .vscode/settings.json and autofixes by moving content to customizations.vscode.settings inside .devcontainer/devcontainer.json, (2) Recommended Extensions Outside Devcontainer - detects .vscode/extensions.json and autofixes by moving recommendations to customizations.vscode.extensions inside .devcontainer/devcontainer.json. Both autofixes merge into existing devcontainer.json if present, create it if absent, and clean up empty .vscode/ directory. Added 9 comprehensive test cases covering detection, autofix, merge, combined fix, policy registration, and statute metadata.

## Changes

## Log

## Todos

## Plan

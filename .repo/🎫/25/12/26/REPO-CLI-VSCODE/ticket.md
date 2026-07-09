# Ticket

## Todos

# Previously

repo.tsx CLI was fully implemented in a previous session with analyze, fix, ticket, bundle, region, definition, and tool commands.

# Plan

1. Add repo CLI commands to VS Code extension (Ctrl+Shift+P)
2. Watch reports/policies.json and show issues as diagnostics
3. Test extension functionality

# Changes

## js/vscode/package.json

Added 12 commands to contributes.commands:

- compose.analyze - Analyze Codebase
- compose.analyzeFile - Analyze Current File
- compose.fix - Fix Codebase Problems
- compose.fixFile - Fix Current File Problems
- compose.policyList - List Policies
- compose.ticketNew - Create New Ticket
- compose.ticketList - List Tickets
- compose.projectList - List Bundles
- compose.regionTree - Show Region Tree
- compose.definitionList - List Definitions
- compose.folderTree - Show Folder Tree
- compose.refreshDiagnostics - Refresh Diagnostics

Added activation events for typescript, javascript, and workspaceContains:\*\*/repo.tsx

## js/vscode/extension.ts

Reorganized into regions:

- Imports: Added fs, path modules
- Constants: Added DIAGNOSTIC_SOURCE_KIT, DIAGNOSTIC_SOURCE_REPO, ANALYZE_REPORT_PATH
- Types: Added Problem and AnalyzeReport interfaces
- Utilities: Added getWorkspaceRoot(), getRepoTsxPath()
- Repo Diagnostics: Added loadAnalyzeReport(), extractFilePathFromScope(), updateRepoDiagnostics(), watchAnalyzeReport()
- Kit Validation: Renamed DIAGNOSTIC_SOURCE to DIAGNOSTIC_SOURCE_KIT, extracted validateKitDocument(), renamed class to ComposeKitCodeActionProvider
- Commands: Added registerCommands() with all 12 command handlers
- Activation: Refactored to initialize both kit and repo diagnostics

## js/vscode/extension.test.ts

Updated diagnostic source filter from "compose" to "compose-kit" to match renamed constant

## Changes

## Log

## Summary

# Summary

---
goal: REFACTOR-PROVIDERS
---

# Ticket

## Summary

Refactored hardcoded provider dependencies into composable provider interfaces. Implemented SourceControlProvider, ManagementProvider, SandboxProvider, EditorProvider interfaces with GitHub, Devcontainer, and 7 editor implementations. Wired 54 lifecycle gh calls through ManagementProvider. Renamed GitHub-specific identifiers to management-generic. Added 10 provider tests.
## Changes

- Added Providers region (main.go ~L8792-9520) with interfaces: SourceControlProvider, ManagementProvider, SandboxProvider, EditorProvider
- Added ManagementIssue, ManagementMilestone, ManagementLabel types for provider abstraction
- Implemented GitHubManagementProvider (wraps all gh* functions), NullManagementProvider (no-op)
- Implemented GitHubSourceControlProvider, DevcontainerSandboxProvider
- Implemented 7 EditorProviders: Copilot, Cursor, Windsurf, ClaudeCode, Droid, Codex, Antigravity
- Added provider registry: AllEditorProviders(), GetEditorProvider(), DefaultManagementProvider(), DefaultSourceControlProvider(), DefaultSandboxProvider(), GetManagementProvider()
- Added managementProvider field to repoContext, initialized in NewRepoContext
- Refactored configureCommand to use EditorProvider.Configure()
- Refactored ResolveHookEvent to use GetEditorProvider()
- Refactored hookCommand output to use provider.FormatHookOutput()
- Replaced 54 direct gh* calls in lifecycle functions with provider method calls (GetManagementProvider() for standalone functions, c.managementProvider for repoContext methods)
- Renamed types: TicketGithubData→TicketManagementData, GoalGithubData→GoalManagementData
- Renamed fields: .GitHub→.Management (82 occurrences on ticket/goal objects)
- Renamed flags: --no-github→--no-management, NoGithub→NoManagement
- Renamed methods: SyncGithub→SyncManagement
- Updated sync subcommand: "github"→"management"
- Updated ensureGoalMilestone return type from *ghMilestone to *ManagementMilestone
- Applied all renames to main_test.go (57 --no-github, 5 NoGithub, 2 .GitHub, 1 syncGithub)
- Added 10 provider-specific tests: TestProviderRegistry, TestGetManagementProvider, TestNullManagementProvider, TestAllEditorProviders, TestGetEditorProvider, TestEditorProviderHookMapping, TestManagementProviderInterface, TestSourceControlProviderInterface, TestSandboxProviderInterface, TestEditorProviderInterface

## Log

- Explored codebase structure: ~38K line main.go with region-based organization
- Identified all gh* functions (~40+) and their call sites
- Designed composable provider interfaces
- Implemented providers and registry
- Refactored callers in waves: configureCommand, hookCommand, ResolveHookEvent, type/field/flag/method renames, lifecycle function gh* calls
- Built and tested: all provider tests pass, all renamed tests pass, clean build

## Todos

- [x] Design and implement provider interfaces (SourceControlProvider, ManagementProvider, SandboxProvider, EditorProvider)
- [x] Implement GitHub as ManagementProvider (wrapping all gh* functions)
- [x] Implement GitHub as SourceControlProvider
- [x] Implement EditorProvider with configure methods and hook adapters for each editor
- [x] Implement SandboxProvider for devcontainer
- [x] Refactor all GitHub issue/milestone code to use ManagementProvider
- [x] Refactor all agent hook code to use EditorProvider
- [x] Refactor configure command to use provider.Configure()
- [x] Wire ManagementProvider into repoContext
- [x] Rename all GitHub-specific identifiers to management-generic names
- [x] Update tests for renames
- [x] Add provider-specific tests
- [x] Verify all tests pass

## Plan

1. Add provider interfaces in a new Providers region in main.go
2. Create concrete provider implementations (GitHubManagement, GitHubSourceControl, etc.)
3. Wire providers into the repoContext and defaultContext
4. Refactor all gh* calling sites to go through ManagementProvider
5. Refactor configure + hook resolution to go through EditorProvider
6. Rename all GitHub-specific identifiers
7. Update tests to cover provider abstraction
8. Run all tests and fix issues

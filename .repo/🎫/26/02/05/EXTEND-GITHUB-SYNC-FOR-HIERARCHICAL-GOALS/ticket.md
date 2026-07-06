---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Implemented three-tier hierarchical GitHub sync: root goals as milestones, first-gen children as issues with milestone, deeper goals as sub-issues without milestone. Fixed milestone lookup bug (gh CLI needs title not number). Migrated all existing goals, cleaned non-root goal.json files, deleted orphaned milestones.
## Changes

- `GoalGithubData` struct: added `Issue` field for child goal issue URLs
- Helper functions: `goalDepth`, `isRootGoal`, `isFirstGenGoal`, `isDeeperGoal`, `getParentGoalID`, `ghGetIssueNodeID`, `ghAddSubIssue`, `ghCreateGoalIssue`, `ghUpdateGoalIssue`, `getRootGoalID`, `getRootGoalMilestone`, `getParentGoalIssueNodeID`, `parseIssueNumber`, `ghGetMilestoneTitle`
- `GoalCreate`: root=milestone, first-gen=issue+milestone, deeper=issue+sub-issue
- `GoalClose/GoalReopen/GoalChange/GoalDelete`: root=milestone ops, non-root=issue ops
- `SyncGithub`: three-tier goal sync with depth-sorted processing and migration
- `ensureGoalMilestone`: now skips non-root goals
- Ticket creation: milestone lookup uses root ancestor goal
- GraphQL schema: added `issue` field to `Goal` type
- `ghCreateGoalIssue` and `ghCreateIssue`: fixed to resolve milestone number to title via `ghGetMilestoneTitle` (gh CLI expects title, not number)
- Cleaned milestone fields from 41 non-root goal.json files
- Deleted 61 orphaned milestones from GitHub
- Updated AGENTS.md and README.md documentation

## Log

- Iteration 1: Implemented two-tier sync (root=milestone, child=issue)
- Iteration 2: Refined to three-tier (root=milestone, first-gen=issue+milestone, deeper=sub-issue)
- Added `ghAddSubIssue` using GraphQL `addSubIssue` mutation with `sub_issues` feature header
- SyncGithub sorts goals by depth before processing to ensure parent issues exist before linking sub-issues
- Iteration 3: Fixed milestone lookup bug - `gh issue create --milestone` expects title not number. Added `ghGetMilestoneTitle` helper.
- Migration: cleaned milestone from 41 non-root goal.json files, deleted 61 orphaned milestones, ran sync github successfully creating issues for all goals
- All 8 first-gen goals now have issues: #398-#405
- All deeper goals have issues and are linked as sub-issues to parent goals
- All goal-related tests pass
- Build and vet pass cleanly

## Todos

- [x] Add three-tier helper functions (goalDepth, isFirstGenGoal, isDeeperGoal, etc.)
- [x] Add ghAddSubIssue and ghGetIssueNodeID functions
- [x] Refactor GoalCreate for three-tier document
- [x] Verify GoalClose/GoalReopen/GoalChange/GoalDelete work with three tiers
- [x] Update SyncGithub with depth-sorted processing and sub-issue linking
- [x] Update GraphQL schema and resolvers
- [x] Update documentation (AGENTS.md, README.md)
- [x] Run tests and build
- [x] Clean milestone fields from non-root goal.json files
- [x] Delete orphaned milestones from GitHub
- [x] Fix ghCreateGoalIssue milestone number vs title bug
- [x] Re-run sync github to create missing first-gen goal issues
- [x] Verify all goals have correct github data

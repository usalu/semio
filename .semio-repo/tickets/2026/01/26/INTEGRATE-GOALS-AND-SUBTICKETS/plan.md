# Plan: Integrate Goals (Milestones) and Subtickets

## Goals
- [ ] Extend SQLite schema for `goals` (id, title, description, github_milestone_number, status).
- [ ] Extend `tickets` table for `goal_id` and `parent_ticket_id`.
- [ ] Implement `repo goal` commands:
    - [ ] `list`
    - [ ] `create` (syncs to GH Milestone)
    - [ ] `update` (syncs to GH Milestone)
    - [ ] `delete` (deletes GH Milestone)
- [ ] Update `repo ticket` commands to support assigning goals.
- [ ] Update `AGENTS.md` and `README.md`.

## Subtickets (Parent/Child)
- [ ] Update `ticket open` to accept `--parent <slug>`.
- [ ] Store parent/child relationship in DB.
- [ ] When syncing to GitHub, handle sub-issue relationship (if possible via API) or just linking.
    - *Note: GitHub Sub-issues are part of Projects/new Issues. Need to verify how to set them via API. Usually it's a "Tracks" / "Tracked By" relationship or nested tasklists.*

## Ticket Delete
- [ ] Implement `repo ticket delete <slug>`.
    - [ ] Delete local ticket folder.
    - [ ] Delete GitHub issue (using `gh issue delete` if available, or just close/archive).
    - [ ] Remove from local DB.

## GitHub Sync
- [ ] Ensure GitHub Milestones are created/updated.
- [ ] Ensure Issues are linked to Milestones.
- [ ] Ensure sub-issues are handled.

## GraphQL
- [ ] Update GraphQL schema to include `Goal` type and extensions to `Ticket`.

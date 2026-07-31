---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY
---

# Ticket

## Summary

Interaction format migrated: system string, client top-level, dates.created/finished. InteractionDates.Started→Created, removed legacy system object unmarshaling, GraphQL schema updated, all ticket.json and goal.json migrated.

## Changes

- InteractionDates: started→created
- Interaction: remove legacy system object unmarshaling
- GraphQL: system String, add client field, dates.created
- Migrate all ticket.json and goal.json

## Log

## Todos

## Plan

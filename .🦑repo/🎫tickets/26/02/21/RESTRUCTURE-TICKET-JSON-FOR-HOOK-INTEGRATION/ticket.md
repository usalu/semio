---
goal: 🎯aioptimizedrepo🎯repoclient🎯repobinary🎯repocli
---

# Ticket

## Summary

Restructured ticket.json format: renamed Prompt to Description, added Sessions with TicketSessionPlanStep/Plan/Event/Prompt types, removed TicketSessionInteraction and TicketSessionReads, rewrote trackHookInOpenTicket to create TicketSessionEvent entries in session prompts, replaced applyHookPathToReads with session-level applyHookPathToSessionDiffModified, added Sections/Definitions to TicketSessionDiff, updated all getter methods with Sessions fallback, updated CreateTicket/ReopenTicket to create Sessions, fixed all TicketNode.Prompt to Description refs, fixed HookResultJSONFields test, all ticket and hook tests pass.

## Changes

## Log

## Todos

## Plan

1. Define new session types: TicketSessionPlanStep, TicketSessionPlan, TicketSessionEventSectionRef, TicketSessionEventCodeBlock, TicketSessionEvent, TicketSessionPrompt
2. Restructure TicketSession: replace Interactions with LLM, Transcript, Query, Plan, Prompts, Diff
3. Update TicketSessionDiff: add Sections and Definitions categories
4. Change Ticket struct: rename Prompt→Description, make Interactions json:"-", make Parent json:"-"
5. Add custom UnmarshalJSON on Ticket for backward compat with old format
6. Update getter methods (GetPrompt→Description, GetLLM→Sessions, GetClient→Sessions, etc.)
7. Update trackHookInOpenTicket to write events into session.Prompts[].Events[]
8. Update ensureTicketSession for new fields (LLM, Transcript, Query)
9. Update CreateTicket, CloseTicket, ReopenTicket to create sessions alongside interactions
10. Update TicketNode.Prompt→Description, TicketData struct
11. Update all tests
12. Build and test

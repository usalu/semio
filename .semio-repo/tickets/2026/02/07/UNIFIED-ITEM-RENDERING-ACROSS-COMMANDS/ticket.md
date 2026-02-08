---
goal: R26-02
---

# Ticket

## Summary

Unified item rendering across all CLI commands. All entity types now render identically via collectEntityProps/renderEntityHuman/renderEntityMarkdownLink regardless of command origin (tree, list, goal tree, section tree, ticket list, monorepo tree). Fixed double-dash bug in goal tree markdown and missing indentation in section tree markdown. Removed legacy template system (TemplateManager, defaultTemplates, tmplColor/tmplColorStatus/tmplTimeAgo/tmplTimeLeft/tmplTruncate/tmplTernary). Added comprehensive identity tests ensuring rendering consistency across all code paths.
## Changes

## Log

## Todos

## Plan

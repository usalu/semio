---
goal: R26-02/RUNNING-SKETCHPAD
---

# Ticket

## Summary

Restored named filter tree labels, switched filter menu actions to emoji-only labels without codeicons, updated tests, and documented the filter label/tooltip requirement.

## Changes

- Updated filter view root items to use emoji-only labels with tooltip descriptions and removed codeicon state markers for date entries.
- Adjusted filter provider tests for emoji-only labels and new dates icon.
- Documented filter view label/tooltip requirements in README and AGENTS.
- Restored named filter tree labels, moved emoji-only labels to filter menu actions, and removed filter command codeicons.
- Updated filter view tests and docs for named tree labels and emoji-only menu actions.

## Log

- Aligned filter view menu items to emoji-only labels with tooltip descriptions per request.
- Goal tree failed to load: graphql errors: [json: cannot unmarshal object into Go struct field Interaction.interactions.author of type string].
- Implemented named filter tree labels with emoji-only menu actions and removed codeicons from filter commands.
- Tests: `npm -w repo/vscode test`.

## Todos

## Plan

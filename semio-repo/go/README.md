# Summary

Shared Go library for semio-repo CLI and server. Provides event kinds, interaction payloads, and types used by both.

# 💯Requirements

- Event kinds and payloads are the single source of truth for CLI→server communication.
- All changing interactions (ticket, goal, contributor, todo, commit) emit events with consistent schema.

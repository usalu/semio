1. Move ticket-level diff storage into per-iteration diff fields and remove the ticket-level diff payloads.
2. Update ticket diff generation, serialization, and any GraphQL or CLI outputs to reference iteration diffs.
3. Adjust tests and fixtures to validate per-iteration diffs and the absence of ticket-level diffs.
4. Update README.md and AGENTS.md to document per-iteration diff storage.

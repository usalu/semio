# Plan

1. Inspect `js/compose/sketchpad/Design.tsx` around the failing `appCommands` reference and identify the missing declaration/import.
2. Implement the minimal fix so `appCommands` is defined at runtime and matches existing command wiring patterns.
3. Run targeted checks (`toolbar.spec.ts` or related test scope) to confirm the app boots without the ReferenceError.
4. Update root developer docs (`README.md`, `AGENTS.md`) with the command wiring requirement for the Design app.
5. Record ticket log and summary, then close the ticket with changed file paths.

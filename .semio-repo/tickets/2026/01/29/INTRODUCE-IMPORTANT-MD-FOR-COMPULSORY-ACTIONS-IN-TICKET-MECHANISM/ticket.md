# Ticket

## Todos

## Changes

## Log

## Summary

Refactored the ticket mechanism to introduce `important.md` for compulsory actions.
- Extended `Ticket` struct with `ImportantPath`.
- Implemented `GetImportantFilePath` helper.
- Updated `CreateTicket` to generate an empty `important.md` on ticket open.
- Updated `UpdateTicketTitle` to handle path changes for `important.md`.
- Updated `FinishTicket` to validate that `important.md` is empty (ignoring whitespace) before closing, throwing an error otherwise.
- Documented the new requirement in `AGENTS.md` and `README.md`.

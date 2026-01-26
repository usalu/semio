# Plan: Auto-Link Ticket Issues to GitHub Project

## Goals

1. Ensure every ticket GitHub issue is linked to the usalu project 2.
2. Add a test for the project linking argument builder.
3. Document the project link behavior in README.md and AGENTS.md.

## Steps

1. Add a project-link helper and reuse it in ticket create and reopen flows.
2. Add a test that validates the project link command arguments.
3. Update README.md and AGENTS.md to describe the project linking behavior.
4. Update ticket log and summary, then close the ticket.

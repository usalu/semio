# Ticket

## Todos

## Changes

## Log

## Summary

Fixed Playwright test configuration to get tests running again. Two issues were resolved: (1) Port mismatch in playwright.config.ts where baseURL was localhost:3000 but webServer was on port 5173 - changed baseURL to match 5173, (2) Installed missing Playwright chromium browser. Tests now run successfully: 4 passed (Home, Kit, Docs, Feedback), 2 pre-existing test failures (Type - navigation issue, Design - element click interception timeout).

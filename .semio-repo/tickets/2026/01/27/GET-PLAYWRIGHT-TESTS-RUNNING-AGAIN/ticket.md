# Ticket

## Todos
# Plan

## Problem

The Playwright tests were not running due to two issues:
1. Port mismatch in `playwright.config.ts`: `baseURL` was set to `http://localhost:3000` but `webServer` was configured to run on port 5173
2. Playwright browsers (chromium) were not installed

## Solution

1. Fix the port configuration in `playwright.config.ts` to use consistent port 5173 for both `baseURL` and `webServer`
2. Install Playwright chromium browser using `npx playwright install chromium`

## Files to Change

- `js/semio/playwright.config.ts` - Fix baseURL port from 3000 to 5173

## Changes

## Log

## Summary

Fixed Playwright test configuration to get tests running again. Two issues were resolved: (1) Port mismatch in playwright.config.ts where baseURL was localhost:3000 but webServer was on port 5173 - changed baseURL to match 5173, (2) Installed missing Playwright chromium browser. Tests now run successfully: 4 passed (Home, Kit, Docs, Feedback), 2 pre-existing test failures (Type - navigation issue, Design - element click interception timeout).

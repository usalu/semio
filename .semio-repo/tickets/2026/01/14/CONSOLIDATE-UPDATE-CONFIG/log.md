# Log - CONSOLIDATE-UPDATE-CONFIG

## 2026-01-14

- Opened ticket.
- Created plan.
- Consolidated `update.config.json` into `dependabot.yml`.
- Refactored `update.ts` to:
    - Parse `.github/dependabot.yml` using `js-yaml`.
    - Automatically derive update paths from Dependabot's `updates` entries.
    - Support and enforce major version constraints specified in Dependabot `ignore` rules (e.g., `versions: [">= 8.0.0"]` for major <= 7).
    - Use default `preserveLocalVersions` for npm (which matched the previous config).
- Verified with dry-run.
- Removed `update.config.json`.

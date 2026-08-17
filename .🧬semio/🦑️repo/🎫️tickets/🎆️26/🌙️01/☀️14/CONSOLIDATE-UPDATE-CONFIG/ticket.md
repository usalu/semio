# Ticket

## Todos

# Plan - Consolidate Update Configuration

Merge `update.config.json` into `dependabot.yml` and refactor `update.ts` to read from `dependabot.yml`.

## Tasks

1. [ ] Analyze `update.config.json` and `dependabot.yml` to identify all settings to migrate.
2. [ ] Update `dependabot.yml` with:
   - Exclusions (using Dependabot's `ignore` field).
   - Version constraints (keeping major version <= 7 for specific .NET packages).
   - Custom metadata if needed for `update.ts` (e.g., `preserveLocalVersions` for npm).
3. [ ] Refactor `update.ts` to:
   - Read and parse `dependabot.yml`.
   - Map Dependabot's structure to the update logic.
   - Implement the logic to respect major version constraints (<= 7) for defined packages.
4. [ ] Verify `update.ts` still works correctly (dry-run).
5. [ ] Delete `update.config.json`.
6. [ ] Update `README.md` or `AGENTS.md` if necessary (though the user mostly wants to get rid of the config file).

## Changes

## Log

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

## Summary

# Summary - Consolidate Update Configuration

Consolidated the standalone `update.config.json` into the standard `.github/dependabot.yml` file and refactored the `update.ts` script to consume the new configuration.

## Key Changes

- **dependabot.yml**:
  - Migrated all `exclude` rules to `ignore` rules.
  - Added specific `ignore` rules for `.NET` packages to enforce legacy major version constraints (`System.Collections.Immutable`, `System.Drawing.Common`, `System.Resources.Extensions` locked to major version <= 7).
  - Expanded `nuget` entries to cover test projects previously listed in `update.config.json`.
- **update.ts**:
  - Replaced `UpdateConfig` parsing from JSON to YAML (`dependabot.yml`).
  - Added logic to automatically discover project files (like `.csproj`) based on Dependabot's `directory` entries.
  - Implemented version constraint enforcement by parsing Dependabot's `versions` ignore strings (e.g., parsing `>= 8.0.0` as `maxMajor = 7`).
- **Cleanup**:
  - Removed `update.config.json`.
  - Added `js-yaml` as a dev dependency to the root `package.json`.

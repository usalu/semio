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

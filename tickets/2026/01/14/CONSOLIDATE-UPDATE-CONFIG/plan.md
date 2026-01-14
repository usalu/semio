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

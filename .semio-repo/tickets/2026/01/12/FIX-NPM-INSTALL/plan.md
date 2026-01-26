# Plan - Fix npm Invalid Version error

1. **Diagnose**: Analyze `npm` log for `Invalid Version` error.
2. **Scan Package Files**: Find all `package.json` files in workspaces that lack a `version` field.
3. **Fix Versions**: Add `"version": "1.0.0"` to all identified `package.json` files to satisfy `npm` workspace requirements.
4. **Cleanup Lockfiles**: Remove corrupted or conflicting `package-lock.json` files (`root` and `js/vscode`) to allow a clean regeneration.
5. **Resolve Conflicts**: Align `@vitest/coverage-v8` versions with `vitest` version 3 to fix `ERESOLVE` errors.
6. **Verify**: Run `npm install` to confirm the fix.

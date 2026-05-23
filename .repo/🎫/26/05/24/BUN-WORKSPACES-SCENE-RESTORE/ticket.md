# Bun Workspaces and Scene Restore

**Status:** closed

## Summary

- Fixed root workspace: `elements/lib/styling` → `elements/lib/styling/js`
- Restored `@elements/scene` and `@elements/topology` under `elements/lib/react/` from git `bc8cbcf72`
- Migrated deps `@elements/ui-shell` → `@elements/framework` + `@elements/framework-react`
- Fixed `useLevel` / `LevelProvider` module scope in `@elements/ui` after `level-context.tsx` split
- Fixed duplicate exports in `@elements/framework-react` index
- Added `@elements/ui` subpath exports: `./level-context`, `./chrome`
- Root `bun install` succeeds

## Tests

- `@elements/ui`: 41 passed
- `@elements/framework-react`: 5 passed
- `@elements/scene`: 31 passed (index + play); scene-play-host resolves with chrome alias

# P10l Owned Date Formatting

## Result

- Removed the only in-scope `date-fns` dependency identity.
- Replaced the Virtual File System's date, datetime, and relative-time formatting with `formatVirtualFileSystemTime`.
- Calendar output remains deterministic `yyyy-MM-dd` / `yyyy-MM-dd HH:mm` local time.
- Relative output requires an explicit locale and uses the platform `Intl.RelativeTimeFormat` boundary; the live component passes the selected i18n language.
- Deleted unused third-party date/locale barrel re-exports.
- Added English/German relative formatting plus calendar-format contract coverage.

## Validation

- `bun install --ignore-scripts`: pass; lockfile refreshed.
- `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`: pass.
- `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`: pass, 534 tests.
- `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`: pass.
- `bun ./📜️script.ts verify dependencies`: pass at 179 identities, 59 removed from baseline, zero additions.
- `bun ./📜️script.ts verify dependencies parity js`: pass with 0 undeclared imports.

# Runtime fix summary

## User error
`Uncaught ReferenceError: parseShellRoute is not defined` in ShellHost.

## Root cause
ShellHost was split from ShellHelpers during co-location but only imported a handful of helpers.
Dozens of runtime symbols (including `parseShellRoute`) were used without imports.

## Fixes applied
1. **ShellHelpers**: exported remaining private helpers used by ShellHost (39 symbols total across passes).
2. **ShellHost**: unified import of 86 helper values + 2 types from ShellHelpers.
3. **World3dHost**: import `shellLabel` from ShellHelpers.
4. **ephemeralBox** (`🧩core`): stop treating function inits as factories — broke `_controlLabelIdResolver` (`(id) => id` was invoked → `undefined`).
5. **os-dev vite**: `server.watch.ignored` for generated registry/session + launch.json to stop restart loops.

## Verified (Playwright)
- Title: `semio · os`
- `#root` mounts children
- Shell chrome text: Fullscreen / Display / Sync / Settings / …
- **No pageerror / ReferenceError**
- Remaining console noise: missing `plugin-modules/s/semio_s_plugin_space.js` when `SKIP_PLUGIN_BUILD=1` (no local `s` artifact; streaming build needed for full studio plugin)

## Files
- `…/ShellHost/🟦️component.tsx`
- `…/ShellHelpers/🟦️component.tsx`
- `…/World3dHost/🟦️component.tsx`
- `…/🧩core/🟦️component.ts`
- `…/🧑‍💻dev/…/⚙️vite.config.ts`

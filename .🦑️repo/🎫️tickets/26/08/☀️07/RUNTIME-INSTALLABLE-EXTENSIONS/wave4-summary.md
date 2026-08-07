# Wave 4 — Build / launch / package wiring

## Done
- Registry generate: 26 EXTENSION_TARGETS (all hosts), launch.json regenerated
- Root `package.json` workspaces cleaned (trailing comma fixed; TS cad extension workspaces removed)
- `runExtensionPackage` added to extension store TS; every extension `📜️script.ts` has `package` → `.sxt`
- Vite already serves/copies `/extensions` via `staticDirVitePlugin`; hub route present
- ShellHost Extensions panel wired earlier (install URL / enable / uninstall)

## Blocked on this machine
- `cargo` / native builds require accepting the Xcode license (`sudo xcodebuild -license`)


## Follow-up consolidation
- Extension scripts now call lib `runExtensionComponentPackage` (Wave 4).
- Removed duplicate store `runExtensionPackage` helper to keep one packaging path.

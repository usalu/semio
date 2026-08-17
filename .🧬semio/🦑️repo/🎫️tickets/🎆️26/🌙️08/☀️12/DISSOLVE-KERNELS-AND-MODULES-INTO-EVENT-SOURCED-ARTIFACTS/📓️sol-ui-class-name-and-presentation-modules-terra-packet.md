# UI Class Name and Presentation Modules Terra Packet

## Lease

- Baseline HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Original source SHA-256: `417bbf652b46de8dbb5ba63c559bffd1e0e6f143b3c34110a4b636a18c612bfe`
- Coordinator-owned React barrel baseline SHA-256: `0f8def42b5703b2ab00bd31f6e7b242e334ea9f60fdd9a5d35c1a88fdf8fa401`
- Read root, UI, and React-package `AGENTS.md` before editing.
- Use `apply_patch` only. Do not run modifying Git commands. Preserve all unrelated dirty content and externally staged state.

## Writable Source Scope

- Delete `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🏷️ClassNames/🟦️component.tsx` after all exports have an owner.
- Create only specific `🟦️component.ts` leaves beneath `🧰️framework/🔨️modules/🖱️ui/🔨️modules/` for:
  - `class-name-composition`
  - `form-control-presentation`
  - `surface-presentation`
  - `interaction-presentation`
  - `status-border-presentation`
  - `chrome-control-presentation`
  - `menu-item-presentation`
  - `border-presentation`
  - `shell-floor-presentation`
- Update every active direct referrer returned by `rg -l '🏷️ClassNames/🟦️component' 🧰️framework/🔨️modules/🖱️ui`, except the protected React barrel.
- Inline the four Slider presentation constants into `🧱️elements/🎚️Slider/🟦️component.tsx` and remove its barrel import of them.
- Inline the two Table row presentation constants into `🧱️elements/📊️Table/🟦️component.tsx`.
- Do not edit the React barrel, OS renderer sources, package manifests, lockfiles, generated files, Storybook registries, or any path outside the specified UI source closure.

## Semantic Requirements

- Each new module owns exactly the responsibility named by its directory.
- `cn` remains the dependency root and uses a repository-owned recursive input type; do not export an external `clsx` or `tailwind-merge` type.
- Preserve Tailwind literal strings verbatim so scanning continues to generate the same classes.
- Preserve all public symbol names and behavior while changing their ownership; no forwarding `ClassNames` module, alias, compatibility export, or wildcard barrel.
- Keep status-state helpers with status-border presentation; keep shell-floor policy separate from generic surface fills; keep menu-item composition separate from generic interaction tokens.
- Use regions/subregions and concise emoji docstrings.

## Coordinator Registrar Handshake

Stop after source creation/import rewrites and send:

- final SHA-256 for every new module, Slider, Table, and every changed consumer;
- an exact old-path scan;
- the symbol-to-module map needed by the React barrel;
- confirmation that the protected barrel is still at its supplied hash.

The coordinator will then replace the old ClassNames barrel region with explicit imports/re-exports from the specific modules, remove Slider/Table private exports, and return the new barrel hash. Do not start final stale scans or Nx gates until that signal.

## Validation After Registrar Signal

- Zero active refs to `🏷️ClassNames/🟦️component` and no authored files beneath the old directory.
- Every retained shared responsibility has at least two independent production consumers.
- Slider/Table private presentation is not exported by the barrel.
- No new circular dependency, external-library public type, wildcard export, or compatibility wrapper.
- Scoped ordinary and cached `git diff --check`.
- Run once each through Nx, uncached: UI React lint, typecheck, test-quick, and build. Classify unrelated baseline failures without repairing them.
- Write unique acceptance Markdown in the ticket with file inventory, hashes, commands, outcomes, and blockers.

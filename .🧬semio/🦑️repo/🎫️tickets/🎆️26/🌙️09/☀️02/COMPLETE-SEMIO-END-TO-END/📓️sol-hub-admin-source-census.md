# Hub Admin Source Census

Date: 2026-09-03
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`

## Outcome

The workspace Tailwind source census now recognizes the canonical shared UI stylesheet at `🧰️framework/🔨️modules/🖱️ui/🎨️.css`. It no longer names or specially excludes the deleted React-target stylesheet.

## Red and repair

An independent Bun/Node filesystem probe reproduced the live stale edge before the edit:

```text
{"staleDeclared":true,"staleExists":false,"canonicalExists":true}
```

`📜️script.ts` now:

- resolves the shared UI source from its canonical module-root path;
- excludes exactly that shared source from the app-entry census so it is not required to import itself;
- describes the source as the shared UI stylesheet rather than a React-target stylesheet.

No compatibility path or duplicate stylesheet was added.

## Verification

The independent post-edit filesystem/import walker passed all five scoped facts:

```text
{"candidate":"🧰️framework/🔨️modules/🖱️ui/🎨️.css","candidateExists":true,"staleReferences":0,"sharedExcludedExactly":true,"adminReachesShared":true}
```

The permanent admin-owned entry and stylesheet graph oracles executed through Nx:

- `bun nx run os-hub-admin:test --skip-nx-cache`: PASS, 2 files and 10/10 tests;
- entry oracle: 4 laws, exactly one HTML module and one package export;
- stylesheet oracle: 5 laws, one canonical import, two Tailwind sources, three resolved shared imports across four stylesheets;
- `bun nx run os-hub-admin:build --skip-nx-cache`: PASS, production Vite build completed in 21.93 seconds.

The build still emits non-fatal pre-existing asset-resolution, browser-externalization, CSS selector, dynamic-import, and large-chunk warnings; no clean warning claim is made.

## Hygiene

Scoped `git diff --check` passed. The generated admin `📤️dist` directory was moved to the macOS Trash after verification and is recoverable there as `semio-hub-admin-dist-20260903-0442`.


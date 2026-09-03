# Hub Admin Source Census

Date: 2026-09-03
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`

## SocketGrant S3 Admin Browser Boundary

The browser no longer owns the administrator bearer. `os-hub:dev-secure-admin` issues an `admin-relay` credential through the protected local bootstrap, holds it only in a loopback relay, and opens `/admin/#semio-admin=<one-use-proof>`. The SPA clears the fragment before bootstrap, receives only an opaque host-only `HttpOnly; SameSite=Strict` cookie, and performs same-origin requests without `sessionStorage`, direct `Authorization`, or the former Vite `/admin/api` bypass.

The registered `os-hub:admin-relay-check` gate executes the relay oracle before the focused UI suite. Session `42976` exited zero: one Vitest file and 10/10 tests passed in 14.01 seconds. Runtime coverage includes raw-local denial, one-use bootstrap/replay refusal, cookie expiry, unsafe-request same-origin enforcement, capability redaction, static-shell isolation, explicit unsupported-locale selection, and pending poll abort with no successor after unmount. Entry and stylesheet neutral oracles also passed 4 and 5 laws respectively.

The source launch seed registers both the relay gate and `os-hub:dev-secure-admin`. Permanent launch regeneration is still red at session `87007` because unrelated plugin catalog discovery reports zero host metadata rows, so no generated launch freshness claim is made for these new entries.

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

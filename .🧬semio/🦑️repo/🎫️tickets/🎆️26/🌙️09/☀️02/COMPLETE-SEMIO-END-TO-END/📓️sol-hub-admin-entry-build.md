# Hub Admin Canonical Entry Build Repair

## Scope

This packet owns only the hub-admin package's entry/build contract:

- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🌐️.html`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/package.json`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🔣️entry-graph.json`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🔣️entry-graph.schema.json`
- this report

No admin component, test setup, component test, stylesheet, Vite configuration, framework UI source, Hub Rust, DB, replication/bootstrap, store, root manifest, launch configuration, plan, acceptance, lifecycle, goal, ticket, or `AGENTS.md` source was edited. The concurrently modified `🧪️admin.test.tsx` was preserved untouched.

## Diagnosis and Red Evidence

The preceding required aggregate probes exposed the same initial build failure:

```text
bun nx run os-hub-admin:build --skip-nx-cache
exit 1
vite v7.3.6 building client environment for production...
0 modules transformed
[vite:build-html] Failed to resolve ./🟦️.tsx from 🌐️.html

bun nx run os-hub:build --skip-nx-cache
exit 1
its os-hub-admin:build dependency fails at the same unresolved HTML entry
```

Read-only source inspection established that this was a stale two-edge graph:

- `🟦️.ts` is the Vitest setup file. It imports `@testing-library/react` and `vitest`, installs jsdom polyfills, and contains no application mount.
- `📦️index.tsx` is the existing 1,284-byte React application entry. It initializes surface chrome and mounts `AdminApp` under the locale and session providers.
- `🌐️.html` referenced nonexistent `./🟦️.tsx`.
- the package root export independently referenced the same nonexistent `./🟦️.tsx`.
- no `🟦️.tsx` existed in the package.

The language-neutral entry graph and its Node oracle were added before either stale edge was changed. The permanent package test then failed before Vitest:

```text
bun nx run os-hub-admin:test --skip-nx-cache
exit 1
AssertionError: Hub admin HTML module entry differs from the canonical entry graph
actual: ./🟦️.tsx
expected: ./📦️index.tsx
```

## Implementation

- The schema-backed `🔣️entry-graph.json` names the HTML document, package manifest, canonical `./📦️index.tsx` entry, and four neutral laws.
- The existing package `📜️script.ts` now runs one Node URL/filesystem oracle before both build and test.
- The oracle reads the fixture and schema, verifies the fixture shape and unique law count, parses exactly one HTML module script, requires exact HTML/export agreement with the fixture, resolves both references through standard `URL`/`pathToFileURL`, requires one identical filesystem path, proves lexical and realpath containment within the package, requires the target to exist, and reports its byte size.
- `🌐️.html` and the package root export now both reference the existing `./📦️index.tsx`.
- No fallback resolution, compatibility file, duplicate entry, or shim was added. The test-only `🟦️.ts` remains the unchanged Vitest setup.
- No executable command was added, so project routing and launch registration did not change.

## Verification

### Entry oracle and package tests

The first post-repair default-level test proves the entry graph but reaches the existing 15-second Vitest budget before component tests complete:

```text
bun nx run os-hub-admin:test --skip-nx-cache
exit 1
[DEBUG] hub admin entry graph oracle: 4 laws, 1 HTML module entry, 1 package export, 1284 entry bytes
[budget] vitest ... exceeded 15000ms — killed
```

The existing exhaustive level provides enough budget for the same oracle and complete suite:

```text
bun nx run os-hub-admin:test --skip-nx-cache -- exhaustive
exit 0
[DEBUG] hub admin entry graph oracle: 4 laws, 1 HTML module entry, 1 package export, 1284 entry bytes
2 test files passed
10 tests passed
0 failed
```

### Aggregate build frontier

Both builds pass the new entry oracle, resolve the canonical application entry, and advance into module transformation. They stop at the next independent stylesheet edge, which is outside this packet:

```text
bun nx run os-hub-admin:build --skip-nx-cache
exit 1
[DEBUG] hub admin entry graph oracle: 4 laws, 1 HTML module entry, 1 package export, 1284 entry bytes
vite v7.3.6 building client environment for production...
4 modules transformed
[@tailwindcss/vite:generate:build] Can't resolve '../../../../../🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css'
importer: 🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🎨️.css

bun nx run os-hub:build --skip-nx-cache
exit 1
its nested os-hub-admin:build passes the same four-law entry oracle, transforms 3 modules, and fails at the same stylesheet import
```

Read-only inspection confirms that the referenced framework React target directory contains `🟦️.tsx` and build/test files but no `🎨️.css`. No stylesheet edit or later build result is claimed.

## Hygiene

- Scoped `git diff --check` over the tracked owned entry/build files exits 0.
- The required failed build emitted ignored `📤️dist/index.html` and `📤️dist/404.html`; those two packet-generated HTML outputs were removed. Pre-existing generated assets were not removed.
- The neutral fixture and schema have no trailing whitespace.
- The existing package `📜️script.ts` already contained a concurrent `resolveTestLevel` test-routing change before this packet; that change is preserved and not attributed to this packet.

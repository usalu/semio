# ⚛️ UI React Package Resolution

## Cause

`@semio-tech/ui-react` declares `".": "./🟦️.tsx"` in its package manifest. Before the active taxonomy migration, the physical entry was `📦️index.tsx`, so package consumers resolved the export to a nonexistent file. The canonical move to `🟦️.tsx` restores the declared package boundary.

The target's local TypeScript self-alias still named `./index.tsx`. It now names `./🟦️.tsx`, matching the export and physical source. A Vitest regression test keeps the manifest export, self-alias, and entry file synchronized.

## Verification

* `bun -e 'await import("@semio-tech/ui-react")'` loaded the workspace package.
* `bun 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts test long -- 🧪️tests/🧪️package-export/🟦️.ts` completed: 20 files and 692 tests passed.

## Follow-up Taxonomy Candidates

The target still has these semantic TypeScript basenames: `🟦️build-tooling.ts`, `🟦️eslint.config.ts`, `🟦️runtime.ts`, and `🟦️vitest.setup.ts`. They were not part of the entry-resolution repair. A future purity wave should classify each against external-tool filename requirements and, where configurable, move it into an appropriate registered semantic directory while preserving its package export or explicit tool-config reference.

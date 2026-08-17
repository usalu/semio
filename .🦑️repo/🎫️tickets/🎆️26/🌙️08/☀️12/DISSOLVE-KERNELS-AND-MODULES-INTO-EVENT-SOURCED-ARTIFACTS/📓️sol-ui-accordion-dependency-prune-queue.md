# UI Accordion Dependency Prune Queue

## Finding

After deleting the zero-consumer Accordion component and its package import, active-source resolution finds `@radix-ui/react-accordion` only in:

1. The clean UI React package manifest at dependency line 37.
2. The clean root `bun.lock` workspace dependency map and package resolution.

No authored production, test, story, glue, or generated source imports the dependency anymore.

## Baseline Hashes

- UI React `package.json`: `0c5a9344dec693c7351eb5a9c76c5e904bb869ca4217c6d5c65d8061afcdfe84`
- Root `bun.lock`: `5aba4612e6a593cf53ae133e5e9f3e8ca54d90b082e8d2bd05352958414f487e`
- Both paths were clean and unstaged at inspection time.

## Deferred Atomic Registrar Work

The dependency must be removed from the UI package manifest and the lock regenerated through Bun as one central registrar lease. It is intentionally not started during this snapshot because an externally owned `animate` plugin `package.json` is dirty. A workspace lock regeneration could absorb that concurrent package-manifest drift and violate the conflict protocol.

Do not hand-edit the lock or leave the manifest/lock pair inconsistent. Rehash both paths and all dirty workspace package manifests after the external package owner releases them, remove the direct UI dependency, regenerate canonically with Bun, verify that lock drift contains only the Accordion dependency closure, then run the UI lint/typecheck/test/build gates.

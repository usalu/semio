# Storybook

The root catalog uses feature-owned stories and the scope registry in `📖️stories/🧭️coordination/🟦️.ts`.

## Start

```sh
repo run workspace:dev-storybook-ui
repo run workspace:dev-storybook-styling
repo run workspace:dev-storybook
```

Port: **6010**, overridable with `STORYBOOK_PORT`. The last command includes every registered scope. OS/plugin/WASM stories need their real producer artifacts; UI/styling do not require unrelated host-engine WASM.

The canonical commands prepare the generated styling tokens and icon/asset catalogs automatically. Do not create substitute generated files or run Storybook without its compiler mode/cache environment. Native Windows keeps the repository's PowerShell bootstrap.

## Validate

```sh
repo run workspace:test-storybook-catalog
repo run workspace:test-storybook
```

Catalog checks cover registered files, relative imports, source aliases, prerequisite edges, toolchain alignment and runtime initialization. The subject fixture under the UI owner's tests guards standalone component families. Browser validation reads the actual index and runs each registered UI/styling story, including play interactions; it rejects preview, console and resource errors. Autodocs is checked separately. The visual-layout browser suite also measures badge padding, icon/button containment, label responsiveness, avatar text clearance and peer gaps, verifies terminal tree connectors, and compares real below-floor WebGL pixels with the grid toggled off.

The default full build is `repo run workspace:build-storybook`. To build just the UI/styling slice, set `STORYBOOK_SCOPE` to `ui,styling` in the calling environment. For example, in PowerShell use `$env:STORYBOOK_SCOPE='ui,styling'`; in a POSIX shell use `export STORYBOOK_SCOPE=ui,styling`. The same environment variable accepts hierarchical scopes and comma-separated combinations for the shared Storybook routing.

## Add or Update a Component

Keep its story under its feature owner's `📖️stories` directory, use one component-subject metadata export per file and show meaningful states. Extend the language-neutral subject fixture when adding a public standalone family. UI strings should use the existing locale/label machinery. Browser plays use `storybook/test` or DOM test utilities, not React Testing Library's development-only `act` wrapper. Native drag/drop events must share one real `DataTransfer`.

Workspace packages remain source-served. External ports hidden behind those package boundaries are prebundled through `STORYBOOK_PREVIEW_DEPENDENCIES` in the scope coordination module; keep that list aligned when changing UI adapters and validate a cold-cache dev preview to catch optimizer reload/504 failures. Do not globally parse `.ts` files as TSX.

Storybook packages are pinned together. Upgrade their declarations in the root, UI and repository product manifests, regenerate the lock, update the version fixture, and validate both the dev catalog and the built static artifact. A successful startup smoke test or bundle alone does not establish browser health.

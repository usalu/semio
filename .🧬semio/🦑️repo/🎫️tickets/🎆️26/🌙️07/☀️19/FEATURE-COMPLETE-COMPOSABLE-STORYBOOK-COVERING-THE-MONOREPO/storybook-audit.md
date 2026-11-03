# Storybook Audit

Historical pre-repair snapshot. Its health, coverage and unchanged-source conclusions below are superseded by `repair.md`, which records the completed repairs and final browser validation.

## Scope and Environment

Read-only audit requested: determine whether Storybook exists, whether it tracks current source, and whether all UI components are represented. The user additionally requested installing/building `repo` and locally provisioning missing platform tools.

Ticket MCP operations are not yet available: the installed native launcher reports that the full native command must be installed with `repo run repo-mcp:install`. No ticket state has been fabricated or modified manually. This report is placed beside the existing feature-complete Storybook task pending access to MCP.

## Bootstrap

- Built the checked-in native launcher with its installer into `~/.local/share/semio/launcher/bin/repo`.
- Launcher provisioned checksum-pinned Bun 1.4.2 successfully.
- Initial `repo --help` and `repo run workspace:setup` failed because workspace JavaScript dependencies were absent. Bootstrapped them with `bun install --frozen-lockfile` in the local validation environment; this succeeded without changing the dependency manifest/lock.
- `repo --help` now succeeds. Full MCP installation timed out after four minutes waiting on `dependencies:store-stage`; `repo doctor --only=dependencies,leases --json` reports no dependency generation is published and recommends `repo dependencies ensure`.

## Source Findings

- Root Storybook configuration exists at `.storybook/main.ts`, using React/Vite, docs and Vitest addons.
- Documented entry: `repo run workspace:dev-storybook`; UI-only entry: `repo run workspace:dev-storybook-ui`. Port 6010 is declared in devcontainer configuration.
- Found 112 canonical `🧪️.story.tsx` files under framework and s, including 66 under the UI module (styling included). File counts are not component coverage percentages.
- Stories import live source rather than a separately versioned UI snapshot.
- Scope coordination still aliases `@semio-tech/infinite-canvas-react-renderer` to the nonexistent old product/OS/infinite tree. The implementation currently exists at `🧰️framework/🔨️modules/🖼️canvas/🎨️react-renderer/🟦️.tsx`. Associated source/watch roots also reference the old location. This is genuine source-layout drift; missing generated plugin outputs, by contrast, are not necessarily stale source references.
- Explicit story references are absent for current public UI exports `Badge`, `StatusChip`, `StatCard`, `EmptyState`, `ErrorView`, `DesktopTitlebar`, `TutorialBar`, `TutorialCaptions`, `TutorialGhostPointer`, and `TutorialVideoOverlay`. These may render transitively inside other components, but are not explicitly exercised as story subjects. Also no explicit story references for `ChromeControlHint` or `DisabledReasonHint`.
- Existing smoke tests enumerate selected story IDs; their presence does not prove exhaustive current export coverage or that the current build passes.

## Registration Audit

Evaluated the real `STORY_SCOPES` registry using the installed runtime, then expanded `buildScopeStoryGlobs(STORY_SCOPES)` with the existing third-party `fast-glob` library and compared it against an independent scan of all canonical story leaves under framework and s.

**Result: all 112 existing story files are registered; zero orphaned story files.** There are 16 registered scopes. File counts for top-level scopes:

| Scope | Story Files |
| --- | ---: |
| ui | 62 |
| styling | 4 |
| framework | 20 |
| puzzle | 4 |
| remodel | 6 |
| infinite | 3 |
| animate | 0 |
| fem | 6 |
| cad | 1 |
| block | 5 |
| presentation | 1 |

Child scopes overlap their parents and are not double-counted. `animate` is an empty registered scope, whereas the distinct `presentation` scope has a story. This does not prove Animate has no UI, only that its own story glob currently matches nothing.

## Runtime Validation

- Installed packages resolve to Storybook **10.6.2**, including React/Vite, docs and Vitest addons. Manifest ranges start at 10.4.0; the lint plugin remains pinned at 10.4.0. No assertion is made about the latest public upstream release.
- Direct Storybook startup requires the build mode and bounded Vite cache environment normally supplied by repository tooling. Without them configuration evaluation refuses; these are setup requirements, not evidence that the canonical command is defective.
- With `SEMIO_BUILD_MODE=prototype`, `STORYBOOK_SCOPE=ui` and a canonical products-partition Vite cache, Storybook's CLI **smoke test passes**. It warns that `golden-layout`, forced by `main.ts` into `optimizeDeps.include`, cannot be resolved; it is not installed as a declared root dependency.
- The actual UI index serves **178 story variants** from 62 story files. File discovery and variant counts are different measures.
- A Chromium/Playwright browser probe of **Button/Default, Ui/Default and Scene/Default fails for all three**. The page displays a preview-load error, not a rendered component. The probe explicitly treats Storybook's own error panel inside `#storybook-root` as failure, avoiding a false positive from merely checking for a nonempty root.
- The browser reports `vite-app.js` load failure and a hostname/security advisory. Repeating with explicit `--host 127.0.0.1` does not resolve it. This advisory alone is not a proven hostname root cause.
- Server logs independently show unresolved generated icon and styling-token imports, including `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🟦️.ts` and `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🔤️tokens/🟦️.ts`. These are missing preparation outputs in this checkout, not necessarily obsolete source references. Canonical preparation could not be completed because dependency-store publication is blocked.
- Full production/WASM builds, all stories in the browser, and exhaustive interactive state/accessibility coverage are **not validated**. The failed shared preview prevents claiming the UI catalog is healthy.

Reproduce the browser probe from the root:

```sh
repo tools exec --light bun -- ./.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/storybook-browser-audit.ts
```

The probe exits nonzero on unhealthy previews, shuts down its server/browser and removes its generated log folder. Chromium and its system libraries were provisioned locally for browser validation.

## Conclusion and Follow-up Order

**Yes, Storybook exists. No, it cannot currently be certified up to date, complete, or healthy.** All existing story files are included, but current public components lack explicit stories, scope aliases contain confirmed layout drift, and sampled browser previews fail in this checkout.

1. Unblock dependency-store publication and complete `repo run repo-mcp:install`; full native MCP is not installed yet. The native launcher and its tool runner are installed and functional. `repo dependencies ensure` also timed out on dependency-store staging after six minutes; automatic doctor repair did not resolve it. Owned orphaned work was cancelled; no other session's tasks were cancelled.
2. Run canonical generated-asset preparation, then rerun browser probes to distinguish missing setup from remaining preview defects.
3. Update obsolete canvas-renderer aliases/watch roots and remove the unused `golden-layout` optimizer inclusion if no longer needed.
4. Add explicit stories for uncovered public status, tutorial, titlebar and hint components; make state coverage intentional rather than assuming incidental transitive rendering is sufficient.
5. Add a current-export-to-story-subject coverage guard; the existing list-based smoke tests cannot prevent newly exported components being omitted.

No application or Storybook implementation source was changed. Added only this ticket's report/probe; environment bootstrap files are local-only. The dependency manifest and lock were not modified. Ticket reopening/closing and the goals resource remain unavailable because full native MCP installation is blocked; no ticket state was fabricated.

# UI Card Zero-Active-Consumer Dissolution Packet

## Verdict

The clean framework UI `Card` component has **zero active production consumers** after the required structural exclusions: `compose/**` and legacy surfaces are excluded, and stories, tests, and glue are not production consumers. The package barrel is a registration/export surface, not a consumer.

The `Card` family is therefore a safe zero-consumer dissolution candidate, subject to the concurrent barrel owner preserving the in-flight `Steps` edit. No source or configuration file was edited during this audit.

## Exact Component And Export Surfaces

| Surface | Exact path and lines | Symbols / disposition |
|---|---|---|
| Owner | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️Card/🟦️component.tsx:21-58` | `CardProps`, `Card`, `CardGridProps`, `CardGrid`; clean tracked source |
| React package barrel | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:7739-7742` | One direct path import and one export pair for `Card`, `CardGrid`, `CardProps`, `CardGridProps` |
| Package path registration | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:12-20` | Only package-root `.` export points to `📦️index.tsx`; no Card subpath export |
| Component manifest | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔣️components.json:1-21` | Shadcn configuration only; no Card entry, component list, or generated Card registration |

There is no separate generated component manifest containing `Card`, `CardGrid`, `CardProps`, or the clean Card path in the current tree. The package has no `semio.storybook` opt-in; its Storybook scope is hand-curated.

## Consumer Census

### Active production scope: 0

The bounded exact-symbol/path scan found no production import, JSX use, or direct path use outside the package barrel. `StatCard` and `SyncAttachCard` are distinct local symbols and are not consumers of this Card family.

### Explicitly excluded references

- `compose/client/lib/sketchpad/js/boot.tsx:10,30-31` imports `Card` and `CardGrid` from `@semio-tech/ui-react` and places them in `SKETCHPAD_MDX_COMPONENTS`. This is a real compose production reference but is structurally excluded by the task; do not count or edit it in this dissolution.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️Card/🧪️story.tsx:14-57` is the Card/CardGrid Storybook story and must be deleted or replaced with the component.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️HoverCard/🧪️story.tsx:11,99-125` imports and renders `Card`/`CardGrid` in excluded Storybook examples. Its Card examples must be removed or rewritten when the barrel/component disappears.
- No exact clean-Card symbol/path references were found in tests or glue. No on-disk legacy Card story was found; the old `.elements/ui/.storybook/story/elements/display/Card.stories.tsx` text at Card story line 3 is only a stale comment.

### Storybook discovery

The clean Card story is discovered by the broad hand-curated UI scope at `.storybook/scopes.ts:67-82`, specifically `../🧰️framework/🔨️modules/🖱️ui/🧱️elements/**/🧪️story.tsx`. There is no Card-specific registration to remove from Storybook scope configuration. The legacy `./stories/ui/**/*.stories.*` glob is unrelated to the co-located Card story.

## Current Fingerprints And Status

SHA-256 fingerprints captured from the current worktree:

| Path | SHA-256 | Git status |
|---|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️Card/🟦️component.tsx` | `0d7234423faaab2b35092ff34734a9b8e134f7d2c09e774076af5127a792ab00` | clean |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️Card/🧪️story.tsx` | `68ed638aca1b079e867eb15564af302fcb1a35fe44795ed9db8602ee7b7c07b8` | clean |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` | `f6936957c8044acaa7af426e671d9a9fe83491ca2c2b4146c9b6a242e77c1aa2` | `M` (unrelated in-flight `Steps` export removal at lines 8031-8036; preserve it) |
| `.storybook/scopes.ts` | `a99679e44eab278d9f2f86c1a28f359c2be4732d075777ddb183bc088f5be49a` | clean |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` | `0c5a9344dec693c7351eb5a9c76c5e904bb869ca4217c6d5c65d8061afcdfe84` | clean |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔣️components.json` | `52b000d8331216ae24f801e49e56afb005bb24afa6669f62cb91c39228a3333b` | clean |

The `🔣️components.json` fingerprint is recorded as the manifest surface; its content has no Card registration.

## Writable Paths And Ownership

The bounded implementation handoff should use these paths only:

- Card owner: delete `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️Card/🟦️component.tsx` and its exclusive story `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️Card/🧪️story.tsx`.
- HoverCard story owner: remove or replace the excluded `Card`/`CardGrid` examples in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️HoverCard/🧪️story.tsx`.
- Sol coordinator: remove only the Card import/export region at `📦️index.tsx:7739-7742` after the current `Steps` edit is accounted for; do not overwrite or restore the existing dirty index.
- No update is required to `.storybook/scopes.ts`, `package.json`, or `🔣️components.json`.
- `compose/client/lib/sketchpad/js/boot.tsx` is explicitly excluded and must remain untouched by this packet.

## JS-Only Nx Gates

These are the applicable gates; they were not run as part of this read-only census:

```text
bun nx run @semio-tech/ui-react:lint --skip-nx-cache
bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache
bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun nx run @semio-tech/ui-react:build --skip-nx-cache
```

The `build` target delegates the root Storybook build with `STORYBOOK_SCOPE=ui`, so it is the discovery/build gate for the co-located story glob. No Cargo gate is required for this JavaScript-only component disposition.

## Tooling Note

The repo MCP `repo://goals`/ticket tools were not exposed in this session. The existing DISSOLVE ticket folder was used for this required research record; no ticket lifecycle state was changed.

# Luna UI HoverCard Zero-Consumer Audit

## Scope and baseline

- Ticket: `2026/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`.
- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- Excluded from the scan: `compose`, `🧰️hub`, `♻️mit-bestand`, legacy/exempt areas, tickets, generated outputs, dependencies, and build caches.
- Repo MCP `repo://goals` and ticket tools were not exposed in this session; the existing ticket folder was used. No Git command modified state.

## Definitions, exports, and fingerprints

| Surface | Exact path / lines | Current SHA-256 | Git state |
|---|---|---|---|
| Owner | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️HoverCard/🟦️component.tsx:23-60` | `58206cb6ee14e1b3bca4ac75a1e8b95b0f2caf1dd1347f78a6f1a0f23a8250c4` | clean |
| Story | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️HoverCard/🧪️story.tsx:15-76` (HoverCard) and `:79-97` (unrelated Aside) | `4d7e61976fbaadbbe16600edf6d6a2be510679a4ef9fe16f77315e01743a2905` | `M` |
| React barrel | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:7984-7987`; Radix namespace import at `:20` | `6efa99283af7df14639d1f301456690d3d16860156ed8d2bf1087094a2bfc2fc` | `M` |
| Package metadata | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:12-20,42` | `0c5a9344dec693c7351eb5a9c76c5e904bb869ca4217c6d5c65d8061afcdfe84` | clean |
| Component manifest | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔣️components.json` | `52b000d8331216ae24f801e49e56afb005bb24afa6669f62cb91c39228a3333b` | clean |
| Story discovery | `.storybook/scopes.ts:63-82` | `a99679e44eab278d9f2f86c1a28f359c2be4732d075777ddb183bc088f5be49a` | clean |

The owner defines and exports `HoverCard`, `HoverCardTrigger`, and `HoverCardContent`; the React barrel imports/re-exports those three. The package has only the root `.` export to `📦️index.tsx`; there is no HoverCard subpath or manifest registration. `@radix-ui/react-hover-card` appears only in the owner, the barrel namespace import, `package.json`, and its lockfile entries.

## Consumer closure

The active exact-symbol/path scan found no production import, JSX use, direct owner-path import, runtime registry entry, or application mount outside the owner/barrel/story/glue surfaces. The only render call sites are the two examples in `🧪️story.tsx:17-50`, discovered by the co-located Storybook glob at `.storybook/scopes.ts:82`; they are example/test evidence, not production consumers. No authored Vitest test references HoverCard. The CSS selector at `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🎨️ui.css:1390` is styling glue, not a consumer. The package barrel and Radix dependency are registration/implementation glue, not terminal consumers.

## Disposition

`HoverCard` is a safe zero-active-production-consumer dissolution candidate. Do not retain, extract, alias, or replace it. The owner and its example-only portion can be removed, but the story file is mixed: its unrelated `AsideNote` story at lines 79-97 must be preserved or relocated before deleting the file.

## Conflict-free writable closure

1. Delete the owner `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️HoverCard/🟦️component.tsx`.
2. In the already-dirty mixed story, remove only the HoverCard import names and HoverCard example/meta region at `:11-76`; preserve `Aside`, `AsideNote`, and its required Storybook meta by relocating that unrelated story to its own co-located story path if the file is deleted. Do not delete `Aside` coverage as part of this packet.
3. In the already-dirty shared barrel, remove only `📦️index.tsx:20` (`HoverCardPrimitive`) and the `HoverCard` registrar block `:7984-7987`; preserve all concurrent accepted Card/DiagramNode/etc. removals and rehash before writing.
4. Remove the now-dead direct package dependency `@radix-ui/react-hover-card` from `package.json` and regenerate the corresponding `bun.lock` entry with the repository's Bun workflow, only after confirming no other active import. No change is needed to `🔣️components.json`, `.storybook/scopes.ts`, or `ui.css`.

No source, configuration, or Git state was modified by this audit; only this ticket report was added.

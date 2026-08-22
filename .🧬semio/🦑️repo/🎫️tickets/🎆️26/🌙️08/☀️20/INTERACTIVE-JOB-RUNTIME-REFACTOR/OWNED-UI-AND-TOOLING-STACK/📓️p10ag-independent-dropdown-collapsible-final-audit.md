# P10ag Independent Dropdown and Collapsible Final Audit

## Verdict: PASS

Fresh read-only audit on 2026-08-22. The P10af rejection is repaired in the current tree. Both retired Radix identities are absent from live manifests, executable source, the configured UI workspace snapshot, and Bun resolution rows; the DropdownMenu-only `@radix-ui/react-menu` orphan is absent too. The owned disclosure and Tree paths have focused behavioral coverage, and the executed UI quick suite passes.

The historical freeze deliberately retains the two identities. This is correct: it is a name/ecosystem baseline for a no-new-dependency ratchet, not a current workspace snapshot.

## Dependency Removal and Lock State

- The React manifest has no `@radix-ui/react-collapsible` or `@radix-ui/react-dropdown-menu` row (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:30-60`). The public barrel has no retired primitive import and exports the owned leaf instead (`📦️index.tsx:16-20`, `:7973-7976`).
- Exact scans of executable source and every non-ticket manifest, including `compose/`, returned zero retired identity/primitive-alias matches. The one remaining textual occurrence outside historical records is a `.cursor` planning markdown file, which is not executable source and is excluded from the dependency scanner.
- Static Bun JSONC inspection found the configured UI workspace snapshot contains neither retired identity. It also found no matching workspace value or package-resolution key for either retired identity or `@radix-ui/react-menu`; an exact raw `bun.lock` scan likewise returned zero rows.
- Root `package.json` has 64 explicit workspace paths: 21 are deliberately excluded `compose/` workspaces and all 43 non-compose paths have a `package.json`; with the root manifest, the parity audit correctly covers 44 configured in-scope workspace snapshots. `bun.lock` has exactly 65 workspace records (root plus all 64 configured paths), with no unconfigured workspace record.
- `bun ./📜️script.ts verify dependencies parity js --format json` passed with 83 manifests, 272 external rows, 123 evidenced rows, 149 advisory unowned rows, zero undeclared imports, 44 lock workspaces, zero lock mismatches, and five passing lock fixtures.
- The parity comparator is table-based over all four Bun manifest sections and reports stale, missing, version-drift, missing-workspace, and invalid-lock conditions (`📜️script.ts:1888-1962`). Its five executable fixtures cover the clean case and each mutable mismatch direction (`:1931-1945`). The current explicit workspace configuration contains no glob or escaped path entry; all live manifests and `bun.lock` are parsed as JSON/JSONC rather than with string regexes. No escaped static import specifier for either identity exists in the scanned executable source, so no escaped-string exception applies to this packet.
- `bun ./📜️script.ts verify dependencies` passed. It retained the 238-entry historical baseline, reports 150 current identities, and lists both retired identities among 88 allowed removals. `dependencyFreezeCheck` fails any current identity absent from that baseline while allowing removals (`📜️script.ts:2390-2401`), preserving the intended one-way new-dependency ratchet. `🔒️dependencies.json:185-214` is therefore correctly unchanged historical evidence, not a lock snapshot.

## Owned Collapsible

- Controlled and uncontrolled state are distinct: only uncontrolled transitions mutate local state, while every valid controlled activation remains a proposal (`↕️Collapsible/🟦️component.tsx:53-66`). The focused lag test verifies three repeated `true` proposals while `open={false}`, unchanged rendering, a controlled rerender, and the subsequent `false` proposal (`🧪️component.test.tsx:37-71`).
- `useId` plus an optional root override supplies stable content association. Runtime-owned root/trigger/content `data-state`, content ID, ARIA association, and `hidden={!open}` are applied after host props (`🟦️component.tsx:55-70`, `:121-160`). `CollapsibleContentProps` omits `hidden`; the open `hidden={true}` regression assertion is present (`🧪️component.test.tsx:74-96`).
- The trigger uses the owned exactly-one-child Slot. The Slot composes child handlers first, honors `preventDefault`, and composes refs (`🏷️class-name-composition/🟦️slot.tsx:27-69`); the behavior/ref tests cover both composition and cancellation (`↕️Collapsible/🧪️component.test.tsx:145-184`).
- Native buttons use `disabled` and rely on browser synthesized clicks for Enter/Space, avoiding double state changes; non-native `asChild` hosts receive deliberate Enter/Space handling (`🟦️component.tsx:80-152`). The focused suite asserts both paths (`🧪️component.test.tsx:98-143`). Disabled native and slotted-anchor tests verify no disclosure or trigger callbacks, default prevention, `aria-disabled`, `data-disabled`, and `tabIndex=-1` (`:186-253`). The slotted child handler intentionally remains child-first; the wrapper prevents default/stops propagation and suppresses disclosure.

## Tree Integration

- The production expandable row remains a non-native slotted trigger with controlled `Collapsible` ownership (`🪵️Tree/🟦️component.tsx:1626-1652`, `:1689-1703`). Its controlled association, visibility, rerender, and immediate keyboard proposal are asserted (`🧪️component.test.tsx:9-38`).
- Child action click and native drag start/end execute exactly their own callbacks without an open proposal or visual state change (`🧪️component.test.tsx:40-72`).
- When a double-click action is supplied, pointer single-click uses a 300 ms delayed path, second click cancels it, and the double-click prevents propagation before it reaches the disclosure trigger (`🪵️Tree/🟦️component.tsx:1509-1547`). Fake-timer coverage proves one double-click action with zero proposals, and one isolated delayed pointer proposal (`🧪️component.test.tsx:74-103`). Keyboard activation remains immediate because keyboard-synthesized clicks have `detail === 0`.

## Executed Gates

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify dependencies parity js --format json` | PASS — zero undeclared imports and lock mismatches; five parity fixtures passed. |
| `bun ./📜️script.ts verify dependencies list js` | PASS — neither retired identity was emitted. |
| `bun ./📜️script.ts verify dependencies` | PASS — 238 historical baseline, 150 current identities, 88 allowed removals, no new dependency. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 12 files, 608 tests. Only existing Bun `NO_COLOR`/`FORCE_COLOR` warnings appeared. |
| Static manifest/source/lock/workspace inspections | PASS — exact results described above; 1.4 GiB free before and after focused checks. |

## Unrun Gates

- No `bun install`, lockfile regeneration, or frozen-lockfile install check was run, as directed. Frozen consistency is supported by the current parseable/snapshot-equal lock and P10af's recorded frozen run, but was not independently re-executed here.
- UI typecheck, lint, primitive check, formatting, browser/Storybook, long/exhaustive tests, production builds, full-repository verification, and all Rust/Cargo gates were not rerun.
- No source, manifest, lock, baseline, cache, ticket metadata/status/important marker, or Git state was modified. This audit added only this report.

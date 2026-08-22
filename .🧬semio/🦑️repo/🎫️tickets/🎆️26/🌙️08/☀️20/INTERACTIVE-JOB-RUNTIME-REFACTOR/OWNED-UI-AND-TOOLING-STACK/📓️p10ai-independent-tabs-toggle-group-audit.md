# P10ai Independent Tabs and ToggleGroup Audit

## Verdict: REJECT

Independent read-only audit on 2026-08-22. The dependency-removal and most owned state/ARIA work are present, typecheck passes, and both focused suites pass. The packet nevertheless fails three required runtime/accessibility boundaries. Its focused tests do not exercise these browser-real failure modes.

## Blocking Findings

### 1. Tabs generated associations are not collision-safe

`Tabs` derives generated IDs using `encodeURIComponent(value).replaceAll("%", "-")` at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🟦️component.tsx:68-70`, then uses that token for trigger and panel IDs at `:96`. This is not injective: the executed check returned `{"slash":"-2F","literal":"-2F","collides":true}` for values `"/"` and `"-2F"`. One Tabs group containing those valid string values therefore renders duplicate trigger/panel IDs and ambiguous `aria-controls`/`aria-labelledby` associations, violating the stable unique-ID requirement.

The existing Tabs test proves multiple React-generated group prefixes and one explicit ID, but contains no adversarial-value association test (`📑️Tabs/🧪️component.test.tsx:39-64`).

### 2. Toggle dropdown renders nested interactive controls

`ToggleGroupItem` always renders an outer native `<button>` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️component.tsx:186-243`) and places `action` inside it (`:229-241`). The Toggle dropdown supplies an un-slotted `PopoverTrigger` containing `Action as="button"` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🟦️component.tsx:302-306`); `PopoverTrigger` defaults to `PopoverPrimitive.Trigger` rather than `asChild` (`🗨️Popover/🟦️component.tsx:32-33`). Thus the production path nests a trigger button and an Action button within the item button (the with-action path also places an action wrapper inside the button).

The wrapper's propagation handlers (`ToggleGroup/...:230-238`) can make a synthetic `fireEvent.click` look isolated, but they do not make the DOM conforming or guarantee native keyboard/click behavior after browser parsing. This violates the required action/dropdown event-isolation and accessible native-control boundary. The two Toggle tests only dispatch synthetic clicks and do not assert valid non-nested controls or native Enter/Space click synthesis (`🎚️Toggle/🧪️component.test.tsx:60-103`).

### 3. Hidden Admin tabs still cause production requests and a stream connection

Every Admin page is mounted inside a `TabsContent` regardless of selection (`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🛡️AdminApp/🟦️component.tsx:75-93`); Tabs merely adds `hidden={!selected}` while retaining children (`📑️Tabs/🟦️component.tsx:231-238`). The inactive pages execute their load effects: Spaces `:199-202`, Users `🙋️UsersPage/...:24-27`, Documents `📄️DocumentsPage/...:30-50`, Events `📰️EventsPage/...:37-40`, and Connections `🔴️ConnectionsPage/...:49-60`. Connections additionally creates a `DirectoryClient.stream()` subscription at `:59-60` while its panel is hidden.

The Admin test masks this behavior by stubbing all six HTTP routes and a WebSocket before mounting (`🌎️hub/.../🧪️admin.test.tsx:188-213`); it asserts visibility/association only. It therefore confirms that hidden content is mounted, but not the required absence of production side effects from those hidden panels.

## Confirmed Non-Blocking Evidence

- The owned Tabs surface has explicit public contracts, controlled/uncontrolled proposal handling, roles, state-owned ARIA/visibility attributes, disabled buttons, orientation/RTL/Home/End navigation, manual activation, and refs in `📑️Tabs/🟦️component.tsx:13-242`. The focused suite covers ordinary controlled lag, disabled skipping, horizontal RTL, vertical navigation, and manual activation.
- The owned ToggleGroup exports discriminated single/multiple contracts and implements controlled proposals, empty single selection (`""`), disabled state, `aria-pressed`, roving orientation/RTL/Home/End/loop behavior, and refs in `🎛️ToggleGroup/🟦️component.tsx:28-248`. The focused suite covers ordinary single/multiple controlled lag and roving focus.
- No live TypeScript/TSX/package manifest occurrence of `@radix-ui/react-tabs`, `@radix-ui/react-toggle-group`, `TabsPrimitive`, or `ToggleGroupPrimitive` was found. The two package-resolution rows are absent from `bun.lock`.
- `bun.lock`'s diff deletes both direct workspace declarations and both resolution records; `🔒️dependencies.json` is unchanged and retains the historical baseline as required.
- The exact parity check reports 83 manifests, 270 external rows, 121 evidenced rows, zero undeclared imports, zero lock mismatches, five lock fixtures, and 44 lock workspaces. The report's prior 149 advisory-unowned rows remain advisory; this audit does not treat them as a packet failure.

## Test-Adequacy Gaps

- No existing Tabs test uses collision-prone values or validates uniqueness/association for them.
- No ToggleGroup test renders two generated-ID groups, validates DOM validity, or uses actual browser keyboard/click sequencing. The root ToggleGroup keyboard coverage tests roving keys only; click activation is synthetic.
- No browser/Playwright or Storybook run was performed, so nested-button parser behavior and native key-click duplication are unverified.
- No compile-time negative test asserts that invalid single/multiple value and callback combinations are rejected; typecheck only validates the production call sites.

## Executed Gates

| Gate                                                            | Result                                                                                              |
| --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`     | PASS                                                                                                |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`    | PASS — 14 files, 618 tests; only existing Bun color warnings                                        |
| `bun nx run os-hub-admin:test --skip-nx-cache`                  | PASS — 2 files, 7 tests; test stubs hide the mounted-panel side effects above                       |
| `bun ./📜️script.ts verify dependencies list js`                 | PASS — neither retired identity emitted                                                             |
| `bun ./📜️script.ts verify dependencies parity js --format json` | PASS — zero undeclared imports and lock mismatches; five lock fixtures                              |
| `bun ./📜️script.ts verify dependencies`                         | PASS — 238-item historical baseline, 148 current identities, 90 permitted removals, no new identity |
| Exact executable source/manifest and raw-lock scans             | PASS — retired identities and primitive aliases absent                                              |
| Deterministic ID-token probe                                    | FAIL — `"/"` and `"-2F"` both map to `"-2F"`                                                        |

## Unrun Gates

- No install, lock regeneration, frozen-lockfile install, cache deletion, source repair, ticket metadata change, or Git mutation.
- No browser/Playwright, Storybook, production build, lint, primitive check, exhaustive suite, full repository gate, or Rust/Cargo gate.
- No direct browser-runtime test was added because this is an independent read-only audit.

## Audit Scope

Read `AGENTS.md`, the applicable UI instructions, `p10ad`, `p10ag`, and `p10ah`; inspected the current source, focused tests, Admin consumer, manifest, `bun.lock`, historical baseline, and relevant worktree diff. This audit added only this report.

## Repair Disposition — 2026-08-22

All three blocking findings were repaired after this independent read-only verdict:

1. Tabs associations now use a deterministic domain/length-prefixed encoding of every exact UTF-16 code unit plus the stable React root instance ID. Adversarial tests cover `/`, literal `-2F`, `%2F`, NUL, emoji, combining `e\u0301`, precomposed `é`, two simultaneous roots, exact bidirectional associations, and server-render/hydration ID stability.
2. ToggleGroup now renders the primary native toggle and secondary action as siblings. Toggle uses a native Action button for `withAction` and `PopoverTrigger asChild` for dropdown, preserves the primary ref, propagates disabled state, keeps roving selection on the primary, and isolates action/dropdown focus, keys, native clicks, Popover selection, and state. Runtime assertions prove no action/dropdown path renders `button button`, `button a`, or `button input`.
3. TabsContent now renders only the active panel, so inactive Admin page descendants do not mount. The real Admin integration records exact request counts without stubbing hidden Users/Documents/Events, proves the active Overview/Spaces/Connections request boundaries, proves Connections creates exactly one stream, proves switching away closes it, and proves returning creates one fresh stream.

### Repair Gates

| Gate                                                       | Result                                                                                                                                                                       |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focused format write/check                                 | PASS                                                                                                                                                                         |
| `@semio-tech/ui-react:typecheck --skip-nx-cache`           | PASS                                                                                                                                                                         |
| `@semio-tech/ui-react:test-quick --skip-nx-cache`          | PASS — 14 files, 622 tests                                                                                                                                                   |
| `os-hub-admin:test --skip-nx-cache`                        | PASS — 2 files, 7 tests                                                                                                                                                      |
| `@semio-tech/ui-react:lint --skip-nx-cache`                | PASS                                                                                                                                                                         |
| `@semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | PASS — zero violations, two existing allowlisted files                                                                                                                       |
| Frozen lockfile-only check                                 | PASS — existing `bun.lock` diff remains 44 deletions                                                                                                                         |
| Dependency list and ratchet                                | PASS — 148 current identities, 90 allowed removals, no new identity                                                                                                          |
| JavaScript dependency parity JSON                          | PASS — 83 manifests, 270 external rows, 121 evidenced rows, 149 advisory unowned rows, zero undeclared imports, zero lock mismatches, 44 lock workspaces, five lock fixtures |
| Exact live source/manifest and lock scans                  | PASS — both retired identities and primitive aliases absent                                                                                                                  |
| Static/runtime nested-control scans                        | PASS — no button contains a button, link, or input in repaired action/dropdown paths                                                                                         |

### Explicit Broad and Unrun Gates

- No package-installing command, lifecycle script, cache deletion, Cargo/Rust command, compose mutation, Popover rewrite, ticket metadata/status change, or Git mutation was performed.
- No Storybook, Playwright, production browser, production build, exhaustive UI suite, renderer-wide suite, full repository test suite, full monorepo format/typecheck/lint gate, or unrelated product suite ran.
- SSR/hydration stability and DOM validity are exercised in jsdom; no claim of a production-browser parser or hydration run is made.

Disposition: the three blocking findings are resolved and the packet is ready for an independent re-audit.

# P10az Independent XState React Removal Audit

## Verdict

**PASS — the bounded P10ay removal is complete.** `@xstate/react` and its only public alias, `useXStateSelector`, have no live static, dynamic, config, script, or type consumer. The UI target barrel, its manifest, and `bun.lock` no longer contain the identity. The active `xstate` boundary remains intact in the UI API, UI manifest, lock resolution, CAD, and Puzzle.

This audit independently examined the current shared worktree against `📓️p10ax-next-owned-dependency-scout.md` and `📓️p10ay-remove-unused-xstate-react.md`, then re-ran the bounded gates. No implementation source, manifest, lockfile, config, or Git state was edited. No Cargo command ran.

## Surface And Reachability Evidence

| Check | Independent evidence | Result |
| --- | --- | --- |
| Removed public facade | `📦️index.tsx:18057` now directly exports only `assign`, `createActor`, `fromCallback`, `setup`, `ActorRefFrom`, `AnyActorRef`, and `SnapshotFrom` from `xstate`; the preceding `useSelector as useXStateSelector` re-export is gone. | PASS |
| Static, dynamic, config, script, and type consumers | Case-insensitive repository scan across executable TypeScript/JavaScript, config formats, package manifests, scripts, and web assets — excluding dependency installs, VCS/cache/output, and ticket history — found zero `@xstate/react` or `useXStateSelector` matches. Import/require/dynamic-import/export-specific scan was also zero. | PASS |
| Historical exception | The only non-excluded repository match is `🔒️dependencies.json:918`, the historical freeze allowance for the retired identity. It is not executable source, a manifest, lock entry, config, script, or exported type. | PASS |
| UI manifest and lock removal | The target manifest has no `@xstate/react` row. `bun.lock` has no workspace edge or `@xstate/react@…` resolution. The focused Git diff hunk removes exactly that direct row and the old `@xstate/react@6.1.0` resolution. | PASS |
| Active XState UI boundary | UI manifest retains `"xstate": "^5.25.0"`; the barrel retains the explicit owned-facing `xstate` exports; `bun.lock:531` retains the UI workspace edge and `bun.lock:4213` retains `xstate@5.32.5`. | PASS |
| Active CAD/Puzzle boundary | CAD and Puzzle manifests both retain `"xstate": "^5.31.1"`. CAD imports `createActor` and `setup` at `✏️s/🔌️plugins/📐️cad/.../🎰️stately/🟦️component.ts:6`. | PASS |

The removed alias had no type consumer, so no external-library-derived public type or value remains through this façade. The retained `xstate` exports are deliberate and are outside P10ay.

## Excluded Boundaries And Shared-Worktree Attribution

The live UI target still declares and its barrel still imports/re-exports the excluded active implementations: the three DnD identities, `@xyflow/react`, `d3-force`, `dagre`, the i18next trio, `react-router`, `react-resizable-panels`, the Three/Fiber/Drei group, and `pdfjs-dist`. No P10ay leaf implementation was added or removed for any of them.

The worktree is concurrently dirty: `git diff --name-only` also includes pre-existing/concurrent Tree and TableAvatar edits plus other Phase 10 dependency removals. Git has no per-packet attribution, so this audit does not falsely assign those broad diffs to P10ay. The packet-local focused diff proves the `@xstate/react` manifest/lock deletion, and the current live manifests/barrel prove that every excluded boundary above remains. This is sufficient for the bounded removal; ownership or behavioural recertification of those unrelated packets is not claimed.

## Independently Executed Gates

| Gate | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick` | PASS — 19 files, 672 tests. |
| `bun nx run @semio-tech/ui-react:typecheck` | PASS. |
| `bun nx run @semio-tech/ui-react:lint` | PASS; only Bun's existing `NO_COLOR`/`FORCE_COLOR` warning. |
| `bun nx run @semio-tech/ui-react:check-ui-primitives` | PASS — zero violations; two existing allowlisted files. |
| `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache` | PASS — fresh run, 4 files, 439 tests. |
| `bun nx run @semio-tech/framework-renderer-react:lint` | PASS. |
| `bun nx format:check --files=<UI barrel, UI manifest, bun.lock>` | PASS. |
| `bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary` | PASS. |
| `bun ./📜️script.ts verify dependencies` | PASS — 238 baseline, 142 current, 96 removals, no additions. |
| JavaScript/Rust dependency censuses | PASS — 79 JavaScript and 63 Rust identities. |
| `bun ./📜️script.ts verify dependencies parity js` | PASS — 83 manifests, 264 external rows, 115 evidenced rows, 149 advisory unowned rows, zero undeclared imports, zero lock mismatches, five fixtures, and 44 lock workspaces. |
| Manifest/source evidence | PASS — regenerated P10 manifest/source report records 64 manifests, 576 direct rows, 264 external rows, and 75 no-package-scope-evidence candidates. The removal itself has zero manifest/source evidence. |
| Exact removal scans and focused `git diff --check` | PASS — no live removed identity/alias; focused diff check is clean. |

## Renderer-Wide Typecheck Residual

`bun nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache` was deliberately run beyond the bounded consumer-test requirement and **fails**. Its current errors are in excluded demonstrator, graphics/R3F, surface/WASM host, shell, utility-tree, and worker paths. A complete-output filter found **no** diagnostic mentioning `@xstate/react`, `useXStateSelector`, or `📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`; the UI target's own typecheck passes.

This pre-existing renderer-wide baseline is a residual, not a P10ay rejection: repairing it would expand into the explicitly excluded product and graphics boundaries. It must remain visible to a renderer-wide closure effort.

## Remaining Residuals

No DOM, event, pointer, keyboard, SSR, hydration, or assistive-technology implementation changed in this cleanup. The acceptance establishes static consumer absence, lock/manifest integrity, UI compilation, and real renderer-consumer quick tests; it does not separately exercise every production bundle or route. No browser or hydration run was performed.

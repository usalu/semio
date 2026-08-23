# Terra Independent React Router Retirement Audit

Date: 2026-08-22
Verdict: **ACCEPT — ratchet the live direct-identity boundary to `139 = 76 JavaScript + 63 Rust`.**

## Scope And Independence

This is a fresh, read-only audit of the completed `react-router` retirement. It reviewed:

- `📓️p10-react-router-owned-navigation-retirement-2026-08-22.md` (implementation record),
  `📓️terra-next-serialized-react-router-scout-2026-08-22.md` (original scope), and
  `📓️coordinator-owned-route-real-browser-gate-2026-08-22.md` (browser record);
- the current UI React barrel, its manifest, `bun.lock`, the dependency verifier, and the actual
  Vite harness `🧪️p10-owned-route-browser-gate.html`;
- exact literal, dynamic-import/`require`, configuration, type/binding, and lock scans, with
  `compose`, dependency artifacts, build output, and ticket history excluded from the executable
  source scan.

No production source, manifest, or lockfile was hand-edited for this audit. Cargo and Compose were
not run. The frozen-lock command is a validation gate and exited successfully; the resulting
working diff still has precisely the expected `react-router` graph removal described below.

## Independent Dependency And Consumer Evidence

The current UI manifest has no `react-router` row. The working lock diff removes the UI workspace
edge and `react-router@7.18.2`, removes the now-unreachable `set-cookie-parser@2.7.2`, and replaces
the former top-level `cookie@1.1.1` row with the still-reachable Express `cookie@0.7.2` row. The
remaining `express/cookie` alias proves the latter is not an accidental prune.

Current `bun.lock` has **no** literal `react-router`, `set-cookie-parser`, or `cookie@1.1.1` entry.
The retirement therefore does not leave a direct or transitive lock survivor for the router
boundary.

The executable-source/config/type/dynamic scan found no `react-router` import, `import()`,
`require()`, externalization, or configuration reference, and no consumer of the retired
`BrowserRouter`, `MemoryRouter`, `Link`, `Outlet`, `Route`, `Routes`, `useLocation`, `useNavigate`,
`useParams`, or `useSearchParams` facade from `@semio-tech/ui-react`. The only broad
case-insensitive `useParams` hits were unrelated `useDirectParams` names inside jcoprobe fixtures;
they are not UI-router consumers.

`🔒️dependencies.json` retains the historical `react-router` baseline row. That is expected for the
one-way ratchet: `bun ./📜️script.ts verify dependencies` computes current direct identities from
the manifests and reports it as a removed baseline identity, rather than treating the historical
record as live reachability.

## Exact Retirement Delta

The packet-owned delta is confined to:

| File                                                                                | Audited change                                                                                                                                                                              |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` | Removes the single direct `react-router` runtime dependency.                                                                                                                                |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`  | Removes the React Router import and public routing export region; moves the owned `RouteLink` seam next to `NotFound`; adds private schema/parser/navigation helpers and six focused tests. |
| `bun.lock`                                                                          | Removes the UI workspace edge, `react-router@7.18.2`, and `set-cookie-parser@2.7.2`; retains the Express-backed cookie resolution as `cookie@0.7.2`.                                        |
| `🧪️p10-owned-route-browser-gate.html`                                               | Imports the actual UI React barrel through Vite and mounts actual `NotFound` and `RouteLink`; it copies no navigation implementation.                                                       |

The current worktree diff is `0/1` lines for the manifest and `1/8` for the lockfile. The monolithic
UI barrel has concurrent non-router port work in its overall `151/38` worktree diff; this audit
anchors its conclusion only to the router import, owned-route, test, and public-export hunks above.
Those hunks do not reach the active P3 renderer/browser-worker/frame-job/OS-host files or active P8
store/plugin files.

## Owned Navigation Contract

The private `OwnedRouteTarget` schema and `parseOwnedRouteTarget` at
`📦️index.tsx:8226-8238` are closed and do not leak through the barrel. The parser admits only a
nonempty protocol-less same-origin target without normalization of its path/query/fragment. It
rejects whitespace, C0 controls, DEL, literal backslashes, explicit schemes, protocol-relative
targets, URL-construction failures, and URLs that normalize to another origin. This blocks the
hostile control/backslash/cross-origin forms at the boundary rather than trusting a call site.

`navigateOwnedRoute` at `📦️index.tsx:8241-8252` first requires browser history and
`PopStateEvent` capabilities. It creates the event and performs `history.pushState` inside one
`try`; it dispatches precisely one synthetic `popstate` only after that command succeeds. A rejected
`pushState` therefore returns `{ navigated: false }` without a history mutation or navigation event.
It is a no-op in SSR/non-browser environments.

`RouteLink` at `📦️index.tsx:8255-8265` invokes a caller handler first and preserves native anchor
behavior for pre-prevented, modified, download, non-`_self`, invalid, and external targets. It
prevents the native click only after owned navigation succeeds. `NotFound` at `📦️index.tsx:8287-8303`
parses its optional `parentPath` before rendering; invalid/external paths render no button, valid
paths use the same command, and the component has no router-provider requirement. No former React
Router value or type is exported at the barrel's public export area.

## Test And Policy Gates Run Independently

| Gate                                                                                        | Independent result                                                                                                       |
| ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- -t 'owned route navigation'` | PASS — 1 file, 6 tests passed; 714 skipped.                                                                              |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                | PASS — 20 files, 720 tests passed.                                                                                       |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                 | PASS.                                                                                                                    |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`                                      | PASS.                                                                                                                    |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`                       | PASS — no violations, 2 allowlisted files.                                                                               |
| `bun install --lockfile-only --frozen-lockfile --ignore-scripts --no-progress --no-summary` | PASS — frozen resolution accepted the current graph.                                                                     |
| `bun ./📜️script.ts verify dependencies`                                                     | PASS — baseline 238, current 139, no new third-party identity.                                                           |
| `bun ./📜️script.ts verify dependencies list js --format json`                               | PASS — 76 identities.                                                                                                    |
| `bun ./📜️script.ts verify dependencies list rust --format json`                             | PASS — 63 identities.                                                                                                    |
| `bun ./📜️script.ts verify dependencies parity js`                                           | PASS — 83 manifests, 261 external rows, 111 evidenced, 150 unowned, 0 undeclared imports, 0 lock mismatches, 5 fixtures. |
| `bunx prettier --check <barrel> <manifest> <harness>`                                       | PASS.                                                                                                                    |
| `git diff --check -- <scoped packet files>`                                                 | PASS — no whitespace errors.                                                                                             |
| Executable/config/type/dynamic/downstream/absence scans                                     | PASS — no live router identity or retired public-facade consumer.                                                        |

The recurring `NO_COLOR`/`FORCE_COLOR` warning emitted by Bun did not fail any command and is not a
router-retirement diagnostic.

The six permanent owned-route tests exercise static and ordinary `NotFound` rendering without a
provider; exact path/query/fragment preservation and exactly one event; native RouteLink fallthrough
and successful primary internal interception; literal backslash, control, whitespace,
protocol-relative, and absolute hostile targets; and a throwing `pushState` with unchanged URL and
zero events. The last test also guards the retired source/API boundary.

## Actual Browser Evidence

I did **not** run an isolated browser in this audit. The coordinator's independently recorded
in-app Browser run is nevertheless valid supporting evidence because its Vite harness imports the
actual production barrel, not a copied implementation. Its report records: initial count `0`;
`NotFound` reaching `/spaces/a?tab=history#entry` at count `1`; browser Back restoring the harness
URL at `2`; Forward restoring the same owned URL at `3`; then Back plus actual `RouteLink` reaching
`/spaces/b?tab=route#link` at count `5`. The ordered location ledger matched and its console was
empty of warnings/errors after the interaction sequence. This supports, but does not replace, the
independent source, lock, unit, and policy evidence above.

## Acceptance Decision

**ACCEPT.** There are no remaining source, public API, downstream-consumer, dynamic-import,
manifest, or lock-reachability blockers for `react-router`. The closed owned contract preserves the
only established navigation capability while explicitly rejecting hostile input and making
`pushState` failure atomic with respect to observable mutation/event publication. The current
dependency verifier independently reproduces the required **`139 = 76 JavaScript + 63 Rust`**
boundary.

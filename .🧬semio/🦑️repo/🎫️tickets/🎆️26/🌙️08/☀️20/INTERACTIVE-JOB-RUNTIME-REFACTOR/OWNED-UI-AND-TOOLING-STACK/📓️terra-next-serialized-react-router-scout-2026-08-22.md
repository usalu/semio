# Next Serialized Dependency Scout — React Router

Date: 2026-08-22

## Verdict

Implement exactly one serialized owned replacement packet for **`react-router`**. It is the smallest remaining live JavaScript boundary with a closed, repository-local consumed surface after the accepted pixel-comparator removal. The packet replaces only the UI barrel's one `useNavigate` call with an owned browser-history command, removes an unused imported `Link`, and intentionally retires the otherwise unconsumed React Router barrel facade.

This is not a generic router rewrite. The owned contract is a finite same-document route-navigation command used by `NotFound` and the already-owned `RouteLink`; route matching, route trees, loader/action APIs, router contexts, and a React Router compatibility surface are expressly out of scope.

Assuming no concurrent manifest change, the direct-identity ratchet is **140 = 77 JavaScript + 63 Rust** to **139 = 76 JavaScript + 63 Rust**. `@dagrejs/dagre`/`dagre` is excluded by the governing real browser/Wasm/OffscreenCanvas gate. No P3 renderer/browser-worker/frame-job/OS-host or P8 store/plugin source is in this packet.

## Current Boundary Reproduced

The following read-only commands were run against the shared tree:

```text
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
```

| Check | Result |
| --- | ---: |
| JavaScript identities | 77 |
| Rust identities | 63 |
| Combined boundary | 140 |
| JS manifest rows | 262 |
| JS evidenced rows | 112 |
| JS undeclared imports | 0 |
| JS lock mismatches | 0 |

No Cargo, Nx, browser, source, manifest, or lockfile operation was run by this scout.

## Exhaustive Live Footprint

| Surface | Exact evidence | Result |
| --- | --- | --- |
| Direct declaration | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:48` | One direct `react-router` row. |
| Runtime import | `…/📦️index.tsx:142` imports `Link` and `useNavigate`. | `Link` has no local execution use; `useNavigate` has one. |
| Runtime call | `…/📦️index.tsx:8233-8247` calls `useNavigate()` and invokes it with `NotFoundProps.parentPath`. | The sole consumed runtime operation is navigation to a supplied string. |
| Public facade | `…/📦️index.tsx:18063` re-exports `BrowserRouter`, `Link`, `MemoryRouter`, `Outlet`, `Route`, `Routes`, `useLocation`, `useNavigate`, `useParams`, and `useSearchParams`. | This is the entire external-library value/type leak and must be removed, not mirrored. |
| Existing owned seam | `…/📦️index.tsx:8862-8889` already owns `RouteLink`, `isInternalRouteHref`, `history.pushState`, and synthetic `popstate` dispatch. | Reuse/factor this seam; do not introduce a second browser-routing implementation. |
| Repository consumers | Exact identity scans plus a scan of every source importing `@semio-tech/ui-react` found no consumer binding for the re-exported router symbols; `NotFound` and `RouteLink` have no repository consumer outside their definitions. | No repository client migration is required. Repeat immediately before editing. |
| Dynamic/config/script/type use | Exhaustive literal scans (source, TypeScript declarations, package/config/script files; excluding dependency artifacts, build output, ticket history, and `compose`) found no `import()`, `require`, alias, externalization, CLI, or non-barrel type reference. | None. |
| Lock reachability | `bun.lock:528` is the UI workspace edge and `bun.lock:3667` is `react-router@7.18.2`. Its unique current child rows are `cookie@1.1.1` at `:2475` and `set-cookie-parser@2.7.2` at `:3807`. | Let Bun prove which rows become unreachable; do not hand-edit. The unrelated Express `cookie@0.7.2` edge remains. |

The earlier P10ax note correctly treated React Router as an active public runtime before its live consumer census was narrowed. The current source evidence is materially smaller: no route tree exists in repository source, and the only executable operation is `NotFound`'s direct path navigation. It is therefore an owned-seam packet, not a broad routing replacement.

## Owned Schema-First Contract

Place the entire owned navigation seam in the existing `//#region 🔗️RouteLink` in `📦️index.tsx`, next to the current policy and before either consumer uses it.

```text
OwnedRouteTarget = { kind: "internal"; href: string }
parseOwnedRouteTarget(unknown href) -> OwnedRouteTarget | null
navigateOwnedRoute(target) -> { navigated: boolean }
```

The schema is deliberately closed:

1. A target is nonempty, protocol-less, and not protocol-relative (`//`). It may retain path, query, and fragment verbatim. External and malformed strings do not become owned route targets.
2. `navigateOwnedRoute` is the sole browser side effect. With a browser `history` and `PopStateEvent` capability it performs one `history.pushState(null, "", href)` and one synthetic `popstate`, then reports success. In static/SSR/non-browser contexts it is a no-op and reports failure rather than touching a global.
3. `NotFound` converts its optional `parentPath` through this schema in the click handler. It no longer needs a Router provider and its public props remain repository/React-only types.
4. `RouteLink` delegates its existing primary-click internal-target branch to the same command. Modified clicks, downloads, non-`_self` targets, external URLs, and pre-prevented events retain native-anchor behavior.
5. Delete the direct import and the complete React Router re-export region. Do not export `OwnedRouteTarget`, the parser, or an API shaped like React Router; zero external router types or values may cross the package boundary.

This preserves the only source-proven product behavior—same-document navigation by `NotFound` and `RouteLink`—while making its authority, input grammar, SSR boundary, and event signal explicit. It does not claim React Router semantics for relative paths, route matching, nested outlets, memory histories, redirects, data routers, or navigation state.

## Differential Fixtures And Permanent Tests

Before deleting the dependency, add a temporary test-only differential fixture that mounts the legacy `NotFound` inside `BrowserRouter`, establishes a known browser URL, clicks `parentPath="/spaces/a?tab=history#entry"`, and records the resulting pathname/search/hash. Run the owned fixture with the same URL and preserve the approved location result as a permanent assertion. Remove the legacy-only fixture/import before manifest retirement.

Permanent in-source tests in the existing `import.meta.vitest` section of `📦️index.tsx` must cover:

1. `NotFound` static render and ordinary render without a router provider—no context exception or browser-global access.
2. A valid owned target changes path, query, and hash once and emits exactly one owned `popstate` event.
3. `RouteLink`'s existing primary internal click delegates to that same command; modified clicks, `download`, non-`_self`, `https:`, and `//` links remain unprevented native-anchor paths.
4. Invalid/empty `NotFound.parentPath` cannot cause an external navigation; define and assert the resulting button/no-op policy once.
5. A direct source/API scan fixture confirms no `react-router` import/re-export and no former router binding remains after retirement.

JSDOM proves the parser, history call, event count, and SSR guard. A real-browser check is still required for URL/history integration and Back/Forward interaction; a synthetic `popstate` unit assertion alone cannot certify browser navigation cadence or a host application's history listener.

## Exact Implementation Scope And Serialization

| File | Required change |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` | Add/factor the closed owned navigation seam, migrate `NotFound` and `RouteLink`, remove the direct import, and remove all React Router barrel exports/tests' legacy fixture. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` | Remove the one `react-router` dependency row. |
| `bun.lock` | Reconcile through Bun only. Expected direct resolution removal is `react-router`; `cookie@1.1.1` and `set-cookie-parser@2.7.2` are expected candidates for pruning, subject to Bun's actual graph. |
| This ticket | Add implementation, differential evidence, independent audit, and regenerated dependency-census evidence. |

The changed source file is an owned UI barrel. It is file-disjoint from active P3 renderer/browser-worker/frame-job/OS-host files and active P8 store/plugin files. It must still be serialized with any concurrent packet changing this UI manifest, this monolithic barrel, or `bun.lock`; no other dependency packet should share the reconciliation.

## Retirement Gates

1. Repeat the exact source/config/dynamic/type and downstream `@semio-tech/ui-react` binding scans immediately before editing; abandon the packet if a real router consumer appears.
2. Run `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- -t 'owned route navigation'`, then the full `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`; run `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`, `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`, and `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`. This scout did not run Nx.
3. Run an actual-browser fixture covering NotFound navigation, URL including query/fragment, `popstate`, and Back/Forward. Do not use P3's Worker/Wasm gate as a substitute; the packet has no P3 source change.
4. Reconcile only with `bun install --lockfile-only --ignore-scripts --no-progress --no-summary`, then validate with the frozen-lock equivalent. Do not hand-edit `bun.lock` or remove unrelated cookie versions.
5. Run `bun ./📜️script.ts verify dependencies`, JS/Rust list JSON, and JS parity. Expected identity boundary: **76 JavaScript, 63 Rust, 139 total**, zero undeclared imports, and zero lock mismatches.
6. Prove absence of `react-router` in executable source/config/manifests/current lock, inspect the surviving `cookie` reachability/version, run format plus `git diff --check`, and capture an independent audit. No Cargo invocation belongs to this packet.

## Residual Risk

The intentional public-facade removal is a repository-wide API cut, justified only by the repeatable zero-consumer scan and the greenfield no-compatibility policy. The owned contract intentionally narrows behavior to browser internal navigation; any future consumer needing a route tree, memory history, loader, or route state must design an owned route domain rather than reintroduce a third-party facade. Browser history semantics and assistive-technology interaction for the native button/anchor require the specified real-browser gate before acceptance.

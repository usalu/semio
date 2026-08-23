# Coordinator Dependency Boundary

Date: 2026-08-22

## Reproduction

```text
bun ./📜️script.ts verify dependencies list rust
bun ./📜️script.ts verify dependencies list js
```

The fresh lists contain exactly **63 Rust** and **66 JavaScript** third-party identities, for an
accepted current boundary of **129 identities**. The complete machine-readable results are retained
in `📝️coordinator-current-rust-dependencies.txt` and
`📝️coordinator-current-js-dependencies.txt` in this ticket.

`dagre` remains in the JavaScript list. It must not be removed until the directed-layout ownership
audit and the real Rust/Wasm/OffscreenCanvas product path are accepted. This count is a dependency
freeze ratchet, not the Phase 9 or Phase 10 exit gate; both phases require zero identities under the
declared boundary.

The one-call `pixelmatch` edge was removed only after representative legacy count/marker parity,
an independent overlap-corruption rejection, an exact byte-span overlap repair, and a second fresh
Terra **ACCEPT**. The final audit is retained in
`OWNED-UI-AND-TOOLING-STACK/📓️independent-owned-parity-pixel-reaudit-2026-08-22.md`. The live
React-versus-WGPU browser screenshot sweep remains an explicit Phase 3 runtime residual.

The `react-router` identity was removed only after the owned `RouteLink`/`NotFound` boundary passed
six focused tests, the complete 720-test UI suite, typecheck, lint, primitive policy, frozen-lock,
absence, and dependency/parity gates. The coordinator separately exercised the actual barrel in the
in-app browser, including exact path/query/fragment preservation, exactly one synthetic event,
Back/Forward behavior, and an empty error console. Terra then repeated the source, manifest,
lockfile, focused/full-suite, and policy audit and accepted the retirement in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-react-router-retirement-audit-2026-08-22.md`.

The `i18next-browser-languagedetector` identity was removed only after explicit owned locale
selection preserved stored `de` and navigator `de-AT` resolution before first paint. The focused
three-test packet, complete 723-test UI suite, typecheck, lint, primitive policy, frozen lock,
absence, dependency/parity, formatting, and scoped diff gates passed. The coordinator and a fresh
Terra auditor separately exercised the actual production barrel in the in-app browser; both stored
and navigator cases resolved `de`, rendered `Zurück`, reported first-paint readiness, and produced
no warning/error console entries. Terra's independent acceptance is retained in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-language-detector-retirement-audit-2026-08-22.md`.
The detector-specific `./compose` stub remains explicitly excluded by the governing plan's Compose
boundary.

The direct `pngjs` identity was removed only after a real Chromium dual-run proved exact dimensions,
all 48 screenshot RGBA bytes, all eight crop bytes, and all 16 diagnostic round-trip bytes against
the outgoing PNG.js decoder/crop path. The permanent focused browser tests and complete 38-test
OS-dev quick suite pass with frozen lock, source/manifest absence, dependency/parity, formatting,
and scoped diff gates. Terra independently reran those gates and accepted the retirement in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-pngjs-retirement-audit-2026-08-23.md`.
The remaining `pngjs` lock resolution is transitive through `@vitest/browser`; it is not a direct
dependency identity and remains until that separate tooling boundary is retired.

The direct `globals` identity and its sole UI React lint-config binding were removed only after the
complete ten-file lint target produced identical zero-diagnostic results with the outgoing ambient
name map and an in-memory empty replacement. The permanent configuration assertion, complete
724-test UI quick suite, lint, typecheck, active `--print-config`, frozen-lock, dependency/parity,
absence, formatting, and scoped-diff gates pass. The reconciled lock correctly removes the orphaned
`globals@16.5.0` resolution because ESLint 10.8.0 has no runtime edge to it. Terra independently
reran the gates and accepted the retirement in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-globals-retirement-audit-2026-08-23.md`.

The direct `remark-mdx-frontmatter` identity was removed from the root and UI React manifests only
after the previously broken active UI Storybook baseline was repaired without compatibility shims.
The green pre-removal and post-removal uncached builds have the same frozen discovery index: 231
entries comprising 170 stories and 61 docs from 61 TypeScript/TSX inputs, with zero owned MDX. The
permanent root build guard now rejects any non-Compose MDX input before building and rejects discovery
drift afterward. The complete 724-test UI quick suite, lint, typecheck, frozen lock, source/manifest
absence, dependency/parity, syntax, and diff gates pass. Terra independently repeated those gates and
accepted the narrow wave in
`OWNED-UI-AND-TOOLING-STACK/📓️p10-remark-mdx-frontmatter-independent-audit-2026-08-23.md`.
The three retained lock workspace rows and shared resolution are owned by excluded Compose manifests.

The direct `remark-frontmatter` identity was then removed from the same root/UI boundary after a
fresh scout proved the configured MDX transform had no owned input: zero owned MDX files, zero
Markdown/MDX module-import edges, and an entirely TypeScript/TSX Storybook index. The pre-removal and
post-removal uncached builds both preserve the exact 231-entry discovery baseline. The complete
724-test quick suite, lint, typecheck, frozen install, dependency/list/parity, source/manifest/lock,
root-script syntax, and scoped diff gates pass. Terra independently accepted the narrow wave in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-remark-frontmatter-audit-2026-08-23.md`.
The three retained workspace tuples and shared `5.0.0` resolution remain Compose-owned.

The direct `remark-gfm` identity was removed next under the same zero-input proof, strengthened by an
exact pre/post Storybook index hash match. Both uncached builds produced the byte-identical 231-entry
index: 170 stories and 61 Autodocs entries from the same 61 TSX inputs, with zero Markdown/MDX input.
The complete 724-test UI quick suite, lint, typecheck, frozen install, dependency/list/parity,
source/manifest/lock, syntax, formatter-baseline, and all working/staged/HEAD diff gates pass. Terra
independently accepted the narrow wave in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-remark-gfm-audit-2026-08-23.md`.
The three retained tuples and shared `remark-gfm@4.0.1` resolution remain Compose-owned; MDX Rollup,
both rehype plugins, and Dagre remain in place.

The direct `rehype-slug` identity was removed only after the installed Storybook source confirmed
that generated Autodocs reuse their TSX CSF `importPath`; only the separate real-MDX extractor and
extension-gated Rollup path can reach the rehype processor. The pre/post uncached build index is again
byte-identical at 231 entries, with 61 Autodocs entries from 61 TSX inputs and zero Markdown/MDX. The
complete 724-test UI quick suite, lint, typecheck, frozen install, dependency/list/parity,
source/manifest/Compose-lock, syntax, formatter-baseline, and all diff gates pass. Terra independently
accepted the wave in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-rehype-slug-audit-2026-08-23.md`.
`rehype-autolink-headings`, MDX Rollup, Dagre, and the permanent guard remain unchanged.

The direct `rehype-autolink-headings` identity was then removed under a fresh repetition of the same
installed-source reachability proof. The pre/post uncached Storybook indexes remain byte-identical at
231 entries, with 61 Autodocs entries sourced from 61 TSX modules and zero Markdown/MDX. The complete
724-test UI quick suite, lint, typecheck, frozen install, dependency/list/parity,
source/manifest/Compose-lock, syntax, formatter-baseline, and all diff gates pass. Terra independently
accepted the wave in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-rehype-autolink-headings-audit-2026-08-23.md`.
The auditor's initially misplaced quick-test capture was moved unchanged into the ticket before
completion; no temporary audit artifact remains outside the required boundary.

The direct `@mdx-js/rollup` identity was removed after its now-empty adapter was proved unreachable.
The root continues to remove Storybook's injected MDX plugin, then no longer appends the empty Rollup
adapter; a live plugin-order probe confirmed every surviving plugin and sentinel retains its position.
The pre/post uncached Storybook indexes remain byte-identical at 231 TSX/Autodocs entries. The complete
724-test UI quick suite, lint, typecheck, frozen install, dependency/list/parity, exact
source/manifest/Compose-lock chain, syntax, formatter baseline, and all diff gates pass. Terra
independently accepted the wave in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-mdx-rollup-audit-2026-08-23.md`.
Compose retains the three direct tuples, shared `3.1.1` resolution, and its transitive MDX chain.

The direct root `eslint-plugin-react-hooks` tooling identity was removed after an exhaustive
zero-reachability audit proved it had no import, registration, Nx, script, or test binding. Both the
UI entry and the representative comment-bearing `PanelTabBar` resolve to the same normalized ESLint
configuration hash and the same 19-diagnostic baseline; the package was never registered in the flat
configuration. The complete 724-test UI quick suite, lint, typecheck, uncached Storybook build and
exact 231-entry discovery hash, frozen install, dependency/list/parity, exact lock orphan removal,
formatting, and all diff gates pass. Terra independently accepted the narrow wave in
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-eslint-plugin-react-hooks-audit-2026-08-23.md`.
The nine existing disable comments and their already-existing unknown-rule diagnostic are unchanged;
the broad root lint remains unrelated red context and is not represented as passing.

`@vscode/test-electron` remains in the accepted boundary. A zero-reference declaration scout was
rejected when independent installed-source inspection proved that the active
`@vscode/test-cli@0.0.10` desktop runner resolves and dynamically imports the package before
`runTests`. The declaration and complete Bun lock closure were restored, frozen installation again
resolves the package from the CLI's absolute configuration directory, and both dependency lists
byte-match this 129-identity boundary. The rejection and independently accepted rollback are retained
in `OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-vscode-test-electron-audit-2026-08-23.md` and
`OWNED-UI-AND-TOOLING-STACK/📓️terra-independent-vscode-test-electron-rollback-audit-2026-08-23.md`.

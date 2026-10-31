# pkgglob result — part 1 DONE; part 2 (act07 follow-up) ROOT-CAUSED, NOT my change, needs coordinator decision (act08 must NOT start yet)

Root cause: member derivation for the lock, the frozen-install check and `workspaces` generation is the Bun ownership walk in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs` (plus the duplicate skip set in `🗂️workspaces/🟦️.ts` `computeWorkspaces`). Both skipped only a hard-coded list of output names; wasm-pack `pkg` and `🕸️bindings` (gitignored via `**/pkg/`, `**/🕸️bindings/**`) were not on it, so a stale `pkg/package.json` became a member and was written into six `workspaces` arrays and then the bun.lock files. The lock checker (`🔒️lock-sync`) reads only the explicit lists, so it followed.

Change (schema-first, one source):
- `🟦️bun/🧬️schema/🔣️.json`: new `$defs.OutputDirectories` const (previous names + `pkg`, `🕸️bindings`).
- `🟦️bun/🟨️.cjs`: `opaque` set now built from that schema def.
- `🗂️workspaces/🟦️.ts`: `WORKSPACE_SCAN_SKIP_DIR_NAMES` built from the same def (no divergent copy).
- Removed the 2 to 4 stale pkg/🕸️bindings entries from `workspaces` of: `package.json`, `♻️mit-bestand/`, `✏️s/`, `🌎️hub/`, `🎓️teaching/`, `🏢️semio-tech/` package.json. No package depends on those names (checked).
- Tests: fixture `gitignoredOutputPackages` in `🟦️bun/🧪️tests/🧫️fixtures/🔣️.json`; new tests in `🟦️bun/🧪️tests/🟦️.ts`: gitignored package dirs under a glob are not members (compared with fast-glob oracle), and `pkg`/`🕸️bindings` are in the schema list AND `git check-ignore` confirms ignored. Fixture `outputCensus.outputs` updated.

Verification: `bun test` of `🟦️bun/🧪️tests/🟦️.ts` 98 pass/0 fail; `🗂️workspaces/🧪️tests/🟦️.ts` 20 pass; read-only `inspectBunWorkspaceOwnership` over the real repo: computed package count equals declared count in all six manifests (132/305/301/309/136/303).

TODO coordinator: run `repo dependencies lock` (I did not) so the six bun.lock files drop the `framework-os-rs`, `flow-core`, `s-spatial-kernel-semio-session`, `s-cad-geometry-browser` workspace rows; then sync. Note the `node_modules` symlink inside stale `pkg/` is untouched.


## Follow-up: act07 `Unowned Source module fallback retained` (root cause)
Not caused by the pkg/bindings exclusion; `pkg` and `🕸️bindings` are neither members nor ancestors of members, so source resolution never touches them. Read-only probes (`repo run workspace:deps-browsers` with a temporary [DEBUG] message, since removed; logs `C/🗑️generated/1029/pkgglob-deps{,2,3,4}.txt`):
- First trigger: edge `@semio-tech/framework@workspace:*` of `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript` hit unowned `✏️s/🔌️plugins/🏗️fem/node_modules` (only `.vite`/`.vite-temp`, 10-18..20).
- Real trigger: the working tree (uncommitted, a peer/coordinator edit) switched the Bun linker `isolated` -> `hoisted` (`🔌️managers/🍞️bun/🔣️.json` `"linker":"hoisted"`, install schema, storage `🟦️.ts` producer configuration). Under hoisted every `w/N/node_modules` is empty, so `sourceProvider` (storage `🟦️.ts`, the walk up from each source) no longer resolves at the first step and climbs the real ancestors; the schema policy `SourceOwnerExecutionProtocol.unownedFallback = refuse-before-follow` / `ambientFallback = require-missing-to-filesystem-root` then refuses ANY existing unowned `node_modules` on the way. Under isolated this walk never left the first step.
- Offending unowned dirs (all gitignored, none infra-owned): 10 ancestor `node_modules` (vite caches) under `🧰️framework`, `🧰️framework/🔨️modules/🖱️ui/🎨️styling` (additionally holds a DANGLING symlink `tailwindcss` -> retired generation 4151… from 10-19), `✏️s/🔌️plugins/{🏗️fem,📐️cad,📐️cad/🧩️extensions/*,📸️remodel,🧩️puzzle}`, `🎓️teaching/🏛️architecture/📚️course`.
- I tried relaxing the guard (fallback only when the unowned node_modules holds a resolvable provider): it passes the fem and styling cases but is contradicted by the schema policy `refuse-before-follow`, and the run then fails with `Declared execution work capacity exceeded` (4194304 work at ~7352 of >8000 edges) because hoisted climbing costs ~570 work per edge. Reverted completely (storage `🟦️.ts` has no change of mine; no [DEBUG] left). Raising the limit is forbidden, so this needs a design decision, not a patch from me.
- OutputDirectories exclusion is unrelated here: `node_modules` is already in that list, and that list governs member discovery, whereas this guard deliberately refuses existing `node_modules`.

Decision needed (coordinator): (a) revert the linker to isolated (my pkg/bindings change stays valid), or (b) keep hoisted and make source-owner resolution hoist-aware (resolve at the scope-root owner without climbing real ancestors, memoize per (dir,name), schema change to the two policy fields), plus remove the stray unowned `node_modules` dirs above (nothing in infra owns them; the 10 vite caches and the dangling link are safe to delete by the dev).

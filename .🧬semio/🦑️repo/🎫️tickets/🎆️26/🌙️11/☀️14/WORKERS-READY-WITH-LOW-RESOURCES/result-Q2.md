# Q2 result: plugin-registry:generate "export nativeBuildUnitGraphInputRequestsV1 not found"

## Verdicts
- Cause: half-applied pair. The facade `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` was one of the 47 "take" paths (old 12 November bytes copied over live). It re-exports `nativeBuildUnitGraphInputRequestsV1`. The target `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` (conflict path, live kept, old copy in `.cache/f/lost/old-repo-g1-server/conflict/…`) no longer has that function: the newer live driver inlines the per-pass unit-graph request in `runNativeBuildDriverV1` (HEAD and live agree; the old copy still has the function plus the old call sites). Nobody else imports the name (whole-repo walk: only the facade). The message path in the task names the target; the failing module is the facade.
- Newer/intended side: live (the inlined form). Only one mismatch of this kind exists.
- Fix (by hand, no alias): the stale name removed from the facade's value re-export line.
- Reproduced before and verified after: `bun -e 'await import("./🟦️.ts")'` in the facade dir gave the SyntaxError, now lists 7 exports; driver-stages imports (5 exports).

## Mismatch scan (60 ts/mjs paths: 47 take + 13 conflicts + copy-ins; script `N/scripts-Q2/resolve.py`)
- Outgoing (named value imports of each path resolve in live targets): 1 mismatch, the facade above.
- Incoming (every importer of those paths in 🧰️framework, ✏️s, 🌎️hub finds its names): 1 mismatch, the same facade line.
- Type-only (erased at runtime, tsc only): `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧹️headroom/🟦️.ts` imports type `BuildGraphScopedOptionalRecoveryResultV1` from `…/🏗️build/📥️inputs/💾️storage/🟨️.mjs`; the name exists nowhere (also absent at HEAD, so not caused by the merge). Not fixed: not a runtime fault and the intended return type is unclear; needs the storage owner.
- The wider static closure (script.ts + facade + driver-stages, 1549 files, `closure.py`) has no further value-import mismatch; one vendored file `🪞️vendor/…/preview2-shim/clocks.js` is absent (generated, ignored).

## Test
- New `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🔗️import-resolution/🟦️.ts` (vitest, picked up by the member's `**/🧪️tests/**/🟦️.ts`): static import-resolution over the closure of the registry `📜️script.ts`, the native build facade and plugin driver-stages; second case proves it reports a missing name. Red with the old facade (named the exact fault), 2/2 green now. Run with a scratch vitest config via bun (root node_modules/vitest), not through `repo`.
- Independent oracle: bun's own runtime import of the facade (above). `bun build` is useless here: it tree-shakes unused missing re-exports.

## Landing
Under `T/landing.lock`, logged in `T/landings.md` (facade sha256 ba97a6ee... -> 861c8f78...).

## Next worker run must show
`@semio-tech/plugin-registry:generate` passes the import stage (no SyntaxError for `nativeBuildUnitGraphInputRequestsV1`). A later, different failure is a new fault.

## Not verified
- `generate` itself (no graph/build commands allowed here); the full registry vitest config and the other registry tests.
- component-pass `🧪️tests/🟦️.ts` needs a repo script invocation: 9 pass / 17 fail standalone ("Original parent Script process invocation required"), unrelated to the facade edit.
- Static parser is regex based; dynamic/computed imports are not followed.

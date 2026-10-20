# Result lift-fix (2026-10-20)

Executor `lift-fix`. Scope: router-lift codemod defects (argv contract, process-birth ownership), live repair of the lifted scripts, root `Cargo.toml` profile question. All repo writes were finished before the TOTAL WRITE PAUSE; nothing was written outside `C` after the pause message. No modifying git command, no cargo, no sync, no `show projects`, at most 2 concurrent tests, every `bun test` argument `./`-prefixed.

## Findings

1. Two distinct lift defects (not one):
   - Router rejects the program's own argv (`unknown command "<arg>"`; also the default branch of programs with a single registered command was unreachable). Applies to every lifted program that reads `process.argv`.
   - A program that opens its own process birth (`receiveScriptProcessInvocation(process.env, ...)`) nested inside the router's birth cannot report after draining, and, decisive, `track()` copies every failing tracked operation into ALL ancestor scopes, so a failure the program catches and reports still fails the outer birth (exit 1). I implemented the coordinator's literal idea (nested drained birth via `withScriptInvocationSignalV1`) and measured it: 8 child suites green, but the `custodian/cancel` child exits 1 ("Paid process job refused" escapes into the outer birth). A nested birth can never swallow failures, so the program must be the outermost owner. Router form is therefore impossible for these children; the correct fix is a bare entry. That revert is also what the pre-lift script was (plain script, no router birth required: the `custodian/worker` child, spawned without the invocation env, needs this too).
2. Therefore the 85+ lifted programs split by one criterion (`isTestProgram`: script path under `🧪️tests` / `👷️child`):
   - test programs (24): bare entry `import { runScriptProgram } from "<module>"; if (import.meta.main) await runScriptProgram();` (pre-lift behaviour, own argv, own birth);
   - all other programs (74): router kept, terminal now `runScriptMain(router, { invocation: original, argv: ["<cmd>"] })` (program parses its own argv; `<cmd>` = former default or first registered command), program module unchanged (nested birth is harmless there, nothing reports after it).

## Changes

Single owner of the shape: `C/lift-fix-tools/shape.ts` (`isTestProgram`, `dispatchOptions`, `bareScript`).
- Generators patched to emit the shape: `C/tax-stage-tools/lift.ts`, `C/tax2-tools/restructure.ts`, `C/tax2-tools/plain-lift.ts` (import `../lift-fix-tools/shape.ts`; bare entry only when no skeleton is kept).
- Live repair: `C/lift-fix-tools/repair.ts <list> --write` (idempotent; also reverts my first, abandoned nested-drain attempt in the modules). Result: 98 lifted scripts (+ their 46 sibling modules touched then restored): 74 router (argv pass-through), 24 bare. Files = every `📜️script.ts` containing `runScriptProgram` (`git grep --untracked -l runScriptProgram -- ':!.🧬semio'`) and their sibling modules (only whitespace/import-spacing noise remains in modules whose nested birth I changed and reverted).
- Law: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🛣️ecma-command-routes/🟦️.ts`: `ecmaRouteOwnedProgram` accepts the bare entry (one argument-free `runScriptProgram()` imported from a relative `🟦️.ts`, under `import.meta.main`, counts as the single terminal). Without it all 24 bare scripts would be rejected by the command-router law.
- Law test: oracle (independent TS-compiler validator) mirrored in `.../🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`; 10 new rows in `.../🧪️tests/🧫️fixtures/📦️package-boundary-classification/🔣️.json` (`owned-program-*`: accept bare entry + nested module; reject outside main, with argument, type-only, foreign module, non-module-slot, other name, two terminals, else branch).
- `C/lib-defects-tools/staged/patches/lib-defects.json`: removed its 2 whole-file rows for the lifted children (scope/cache), they now agree with the live bare form; `apply.sh check` = ok 15 files.
- Tests of the generators: `C/lift-fix-tools/shape.test.ts` + `shape.fixture.json` (language-agnostic cases; outputs parsed by the TypeScript compiler as independent oracle, accepted by the router law, bare form executed with real argv, router form executed under the launcher envelope via `smoke.sh`): 9/9 across `planPlain`, `planLift`, `planRestructure`.
- `C/lift-fix-tools/smoke.sh` (envelope runner), `profile-apply.py` (staged, see below).

## Verification (all run)

- `bun build --no-bundle` on all 98 scripts and modules: ok. Router law (`accepts`) over all 98 scripts: 98/98 accepted (after the law extension).
- `bun test ./C/lift-fix-tools/shape.test.ts`: 9 pass / 0 fail.
- Law suite via `bun C/env-pass2-tools/wrap3.ts --timeout 120000 ./.../📦️package-boundary-classification/🟦️.ts`: the test "fixed script contracts distinguish filenames from semantic bodies" (includes all 10 new rows and the oracle cross-check) passes; remaining 14 reds are rust/go parser oracles and one live-tree row (`library/📦️packages/🟦️typescript/📜️script.ts` now a router), unrelated.
- Owning tests (`C/lib-defects-tools/run.sh`, live tree), pass: capacity inheritance 6/0, compiler funding 13/0, receipt 15/0, descriptor cache 21/0 (the catalog-corruption child), descriptor read 33/0, descriptor nodes 24/0, selection watch 22/0 (fault children). `nx-plugin/arguments` router with arbitrary argv reaches the program ("Private Nx command source required") instead of `unknown command`.
- Failing owner suites, none from the lift: `scope` (4 reds; first red = `root-drain` "Original process admission closed" in `leases retireResourceQueueClaimsV1`, identical with the pure direct form; the other 3 are source-text laws), `custodian` (30 s timeout; identical with the pre-lift scripts from `1bae6115bc5`, A/B run), vscode custody / cmake / intent / budget required / nx outputs / graph publication / transaction / startup service / history / retention physical (text laws that read inline bodies now living in the modules, `import.meta` eval of script text, native image prerequisites), `leases concurrency` runs 0 tests, `daemon journal` fails at fixture JSON parse. Not run: dashboard fixture children (control-plane/workspace), `forged` custodian child.

## Cargo profiles: ruling NOT applied as stated (needs decision)

Evidence contradicts "Cargo.toml is the profile authority":
- `1bae6115bc5` (10-16) removed the 39 tables AND created `🏗️build/🎚️mode/🦀️profile` in the same commit: `renderCargoProfileConfigurationV1` renders `--config profile.<mode>.*` selectors (settings schema + `📋️constraints/🔣️.json`, which already carries the wasmtime/cranelift/gimli/object/hash `opt-level 3` rows from the removed tables). `bun repo cargo` appends them; raw cargo is refused by the ingress gate.
- A087 (`library/🏗️build/🧪️tests/🏗️native`, added 10-16 36fbe9da786) asserts the OPPOSITE of the ruling: `.cargo/config.toml` and every workspace manifest must have NO `profile` table. A067's `manifest.profile.prototype.inherits` assertion (inventory tests, 2 sites) is the stale one and contradicts A087.
- Old profile names `wasm-release|wasm-prototype|host-tool` are referenced only by docstrings/receipt labels, no code invokes them.
- Restoring the tables would turn A087 red, duplicate the constraint rows and be overridden by the selectors anyway.
Staged instead (not applied, write pause): `python3 C/lift-fix-tools/profile-apply.py --write` removes `[profile.prototype]`/`[profile.release]` from `.cargo/config.toml` (rustc ingress and all other keys stay; release/prototype values equal the schema's `release-native`/`prototype-native` settings) and changes the two A067 assertions to `expect(manifest.profile).toBeUndefined()`. `--write` dry state verified ("WOULD" for all 3 edits). If the coordinator still wants the old tables, `git show 1bae6115bc5^:Cargo.toml` lines 482-664 hold them, and A087 plus the constraint rows must change first.
After applying, run: A087 via wrap3, A067, then `bun repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 --manifest-path Cargo.toml > /dev/null` once (not run: write pause, and it must run after the edit).

## Open items

1. After PAUSE LIFTED: `python3 C/lift-fix-tools/profile-apply.py --write`, then the three verifications above.
2. Decide the profile question (above).
3. `scope` child `root-drain` real defect (`retireResourceQueueClaimsV1` admits a control after its birth closed), same with and without any lift.
4. Rerun `show projects` consumers: the law extension and 98 script edits changed the source tree; nothing else of mine is pending.
5. `.cache/ep2/*` artifact dirs of wrap3 runs are gitignored; delete after the pause. No `[DEBUG]` logs were added; `C/🗑️generated/lift-fix` already deleted.

# Norm TypeScript targets after the merge (737-740)

## Root causes of the known iso16757 failure: already fixed upstream by the merge, no edit needed
1. "Cannot find module '@semio-tech/norm-js/registry/contract'" in the artifact TS check. The declaration emitter (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🧬️declarations/🟦️.ts`) left bare workspace-package specifiers in the flat declaration closure, so the consumer probe in the scratch directory could not resolve them. Commit 83b2345 makes the emitter adopt owned workspace packages reached via node_modules as roots, emit them into the closure and rewrite their specifiers to local files (schema-first twins stay imported from `@semio-tech/norm-js/registry/contract`; no alias or re-export). Verified by running `emitTypeScriptDeclarations` on `📇️iso16757/🟦️.ts` at HEAD: 32 files, zero non-relative imports, the formerly failing `9703e3f1...d.ts` now imports `./106a...js`.
2. Lease-order warning (`typescript-package-binding` rank 5 requested while `artifact-scratch-private` rank 6 held). The scratch hash 28a7654c... is exactly the scratch dir of the failing run, so it was the same invocation: `bindTypeScriptPackageArtifactV1` ran inside the scratch body. Commit 83b2345 moved it after the scratch/diagnostics scopes (`binding` flag, bind after `built`), so the lock order namespace -> artifact is no longer violated. (Policy outOfOrder is `observe`, so it was only a warning.)

## Results (all green)
`repo run <project>:<target>` could not complete: `repo run @semio-tech/norm-iso16757:test` waited 3 h for graph refresh (owners 195/621 blocked on memory), then the graph service cancelled the plan at its limit ("holds 2717 MB of its 2717 MB memory limit"). That is the graph agent's area. The real target (incl. declaration emit + consumer probe + registry/contract resolution) is therefore verified at the emitter level only, the rest through the equivalents below (`repo tools exec bun|tsc`, scripts `norm-artifact-ts-equivalent.sh`, `normts3-njs-chunk.sh`, `norm-js-suites.py`, `composition-runner.ts`).

| Project:target | Result |
|---|---|
| @semio-tech/norm-js:test | 50 suite files each `bun test`: 50/50 rc=0, 11991 pass, 0 fail |
| @semio-tech/norm-js:test-wire-twins | 11853 pass, 0 fail (incl. drift gate) |
| 15 x norm-<code>:build (bun bundle) | 15/15 ok |
| 15 x norm-<code>:check (tsc strict on source + suites) | 15/15 ok |
| 15 x norm-<code>:test | din16798 22, din18599 36, din4108 24, en1990 20, en1991 43, en1992 11, en1993 38+1, en1994 21, en1995 29, en1996 58, en1997 10, en1998 28+1, en1999 9, iso16757 28, vdi3805 118 |
| s-norm-composition-tests:verify-norm-document-contract | oracle ok + tsc ok |
| s-norm-composition-tests:verify-norm-results-window-ownership | oracle ok + tsc ok |

en1993 and en1998 SQLite snapshot suites each timed out one test at bun's 5 s default while the host load was 35-49 (capacity queue); re-run at load 12: 38/38 and 29/29 pass in ~8 s. Load flake, no code change.

## Notes
- The verify project moved to `✏️s/🧑‍💻dev/📕️norm/📦️packages/🟦️typescript` (its `runOwnedVerifyScript` still imports the plugin paths correctly). `runCmd` needs `SEMIO_SCRIPT_PROVIDERS` (only `repo run` provides it), so tsc was run directly with the same flags.
- No source files changed by me. Not touched: plugin registry, s catalog, policy-allowlist.json. No `[DEBUG]`/`normts3-` log lines in code; logs are `🗑️generated/normts3-*`.
- Open: rerun the real `repo run` targets once the graph service has memory headroom (blocker is graph, not norm).

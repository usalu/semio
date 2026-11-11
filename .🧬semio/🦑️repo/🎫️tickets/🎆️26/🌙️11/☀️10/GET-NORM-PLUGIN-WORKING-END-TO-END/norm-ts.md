# Norm TypeScript: typecheck and tests

`repo run` is still blocked (graph service stops at its memory limit, plan cancelled). `repo ./📜️script.ts <target>` is not usable either: it fails without a script process envelope/provider designation that only `repo run` issues. The equivalents below run through `repo tools exec bun|tsc`. Logs: `🗑️generated/`. Runner scripts: `norm-js-test-runner.ts`, `norm-js-suites.py`, `norm-artifact-ts-equivalent.sh`, `norm-artifact-check.sh`, `composition-runner.ts`.

## Status

| Project:target | Equivalent executed | Result |
|---|---|---|
| @semio-tech/norm-js:test | cohort check (runTestScript body) + document-contract oracle + 50 suite files, one `bun test` each | pass, 11990 tests / 0 fail after fixes |
| @semio-tech/norm-js:test-wire-twins | `bun test ./🧪️tests/🧪️wire-twins/🟦️.ts` | pass, 11853 tests / 65161 expects |
| @semio-tech/norm-<code>:build (15) | `bun build <artifact>/🟦️.ts --target node` | pass 15/15 |
| @semio-tech/norm-<code>:check (15) | `tsc --noEmit --strict` on source + registered suites, repo lib set | pass 15/15 after fixes |
| @semio-tech/norm-<code>:test (15) | `bun test` registered suites | pass 15/15: din16798 22, din18599 36, din4108 24, en1990 20, en1991 43, en1992 11, en1993 39, en1994 21, en1995 29, en1996 58, en1997 10, en1998 29, en1999 9, iso16757 28, vdi3805 118 |
| s-norm-composition-tests:verify-norm-document-contract | oracle + tsc over en1990/din18599 facets | pass |
| s-norm-composition-tests:verify-norm-results-window-ownership | oracle + tsc over results config | pass |

The bundle/declaration-emit and the consumer-probe step of the real build/test are not exercised by the equivalents.

## Root causes and fixes

1. Stale framework import paths: the process helpers moved to `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript` (`@semio-tech/process`). Updated 22 norm files (15 TS artifact scripts, Rust package scripts and Rust-command test files) from `🧰️framework/📦️packages/🟦️typescript/🏃️process/...` and `@semio-tech/framework/process/*`.
2. `🧪️tests/🧫️fixtures/🪪️document-contract/🔣️.json`: committed counts of din18599 were stale (52/21); actual fixtures 212 snapshots / 42 diffs.
3. din18599 wire twin (`🧬️schema/🟦️.ts`): the zones reader's type literal lacked `thermalMassClass` and `cWirkWhM2k` although the schema and reader define them. Fixed by hand (the old twin generator is stale and cannot run on the current layout).
4. SQLite snapshot suites en1990/1991/1994/1995/1996: JSON fixtures with empty arrays type as `never[]`, which hid unconverted fields. Fixtures are now typed as the wire snapshot (`fixtureJson as EnXXXXSnapshot`), invalid-value tests clone `fixtureJson`; added the missing Binary64 conversions (en1991 footbridge assumedServiceAxle and seven collections, en1995 fastener positions and panel layers, en1994 cases), missing `En1996SqliteChase` import and load-case enum casts in en1996.

## Files changed

- 15x `🗿️artifacts/*/📦️packages/🟦️typescript/📜️script.ts` and the Rust package scripts/test files with the import path fix (22 files total)
- `🧪️tests/🧫️fixtures/🪪️document-contract/🔣️.json`
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `🗿️artifacts/{⚖️en1990,🏋️en1991,🧩️en1994,🪵️en1995,🪨️en1996}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`

No framework TypeScript changed. Note: framework `🧰️framework/📦️packages/🟦️typescript/package.json` still exports `./process/*` to the nonexistent `🏃️process/` dir; norm no longer depends on it.

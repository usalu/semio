# Independent Static R8 and Source R23 — 2026-08-27

## Current Gate

Canonical static verification is RED before emitting its complete findings. The current source mutation test `lost-deadline` no longer reaches the Puzzle fill-preview implementation. Root independently read both sides: production now uses checked microseconds (`default_now_us()?.checked_add(2_000)?` and a missing-clock-or-deadline predicate); the verifier still searches for the earlier millisecond/saturating expression.

The worker-clock executor owns the narrow verifier repair. No production clock will be restored to milliseconds and no no-op mutation will be silently counted. Earlier19standalone-test findings remain an unresolved historical checkpoint, not a freshly completed R8 census.

The launch/discovery stage actually reports32descriptors,101app declarations,57launch-only product surfaces,158total surfaces,4771action rows,101launch-covered app contexts,0missing launch contexts,237dev launch surfaces and25hostile/oracle tests. These are source/discovery counts, not functional app proof.

After taxonomy released its eager root vocabulary changes, independent tool-jobs R23 still passes1009self-tests with33proof owners,255custom/25generic rows, exit0.

## Static Command and Actual Output

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache
```

```text
> nx run workspace:verify-interactivity

> bun ./📜️script.ts verify interactivity

[verify interactivity apps] 32 descriptor(s), 101 app declaration(s), 57 launch-only product surface(s), 158 total surface(s), 4771 action row(s), 101 launch-covered app context(s), 0 missing launch context(s), 237 dev launch surface(s), 25 hostile/oracle self-test(s).
12656 |     ["missing-root-ghost-authority-laws", 9, "for (const mismatchedRoot of [", "for (const ignoredMismatchedRoot of ["],
12657 |   ];
12658 |   for (const [name, index, from, to] of mutations) {
12659 |     const mutated = [...sources];
12660 |     mutated[index] = mutated[index]!.replace(from, to);
12661 |     if (mutated[index] === sources[index]) throw new Error(`[verify interactivity] Puzzle fill preview self-test mutation ${name} no longer reaches production source.`);
                                                             ^
error: [verify interactivity] Puzzle fill preview self-test mutation lost-deadline no longer reaches production source.
      at interactivityPuzzleFillPreviewJsonSelfTests (/Users/ueli/Documents/semio/📜️script.ts:12661:54)
      at interactivityAuditRun (/Users/ueli/Documents/semio/📜️script.ts:12029:3)
      at runInteractivityAudit (/Users/ueli/Documents/semio/📜️script.ts:10543:20)

Bun v1.3.14 (macOS arm64)
Warning: command "bun ./📜️script.ts verify interactivity" exited with non-zero status code


 NX   Running target verify-interactivity for project workspace failed

Failed tasks:

- workspace:verify-interactivity

Hint: run the command with --verbose for more details.
exit_code=1
```

## Source Self-Test Command and Actual Output

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run workspace:verify-interactivity --skip-nx-cache --args='tool-jobs --self-test'
```

```text
> nx run workspace:verify-interactivity --args=tool-jobs --self-test

> bun ./📜️script.ts verify interactivity tool-jobs --self-test

[verify interactivity tool-jobs] exact-factory-proof-owners=33 custom-rows=255 generic-rows=25 clean.
[verify interactivity tool-jobs] self-tests=1009 clean.



 NX   Successfully ran target verify-interactivity for project workspace
exit_code=0
```

## Preserved Boundaries

Sound cfg/test reachability is still required across the existing scanner projections; `not(test)`, mixed `any(test,feature)`, names and filenames must not hide production code. See `📓️coordinator-static-cfg-reachability-review-2026-08-27.md`. Full command census R6 remains a separate twelve-failure/270remaining RED.


# Summary: Fix Missing `ephemeralBox` Import in Framework `glue.ts`

## Changes Made
- Modified `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` to add `ephemeralBox` and `type EphemeralBox` to the existing import statement from `../../🔨️modules/🎠️kernel/🟦️component.ts`.

## Verification Output
Ran `bun nx run @semio-tech/framework:test` successfully with code 0:

```
> nx run @semio-tech/framework:test
> bun ./📜️script.ts test

 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript

 ✓  @semio-tech/framework  🟦️glue.ts (73 tests) 34ms
 ✓  @semio-tech/framework  🟦️glue.ts (73 tests) 33ms

 Test Files  2 passed (2)
      Tests  146 passed (146)
   Start at  23:27:14
   Duration  439ms (transform 444ms, setup 0ms, import 527ms, tests 66ms, environment 0ms)

 NX   Successfully ran target test for project @semio-tech/framework
```

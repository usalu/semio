# Research: Missing `ephemeralBox` Import in Framework `glue.ts`

## Problem Analysis
In `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`, line 460 contains a test block `describe("ephemeralBox", ...)` which invokes `ephemeralBox<(id: string) => string>(...)` and `ephemeralBox<() => void>(...)`.

However, `ephemeralBox` was not imported into `glue.ts`.

## Source of Symbol
`ephemeralBox` and type `EphemeralBox` are defined and exported in `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`:
```typescript
export type EphemeralBox<T> = { current: T };
export function ephemeralBox<T>(key: string, init: T): EphemeralBox<T>
```

## Solution
Import `ephemeralBox` and `type EphemeralBox` from `../../🔨️modules/🎠️kernel/🟦️component.ts` in `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`.

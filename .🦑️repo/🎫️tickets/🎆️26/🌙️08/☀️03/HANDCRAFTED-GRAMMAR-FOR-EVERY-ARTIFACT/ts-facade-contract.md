# TypeScript facade contract

## File

`🟦️component.ts` beside `🦀️component.rs` in each artifact facet (and `📦️packages/🟦️typescript` barrel per plugin).

## Role

Thin WASM binding only — **no** parsing logic in TypeScript.

## API (document facet example)

```ts
export function parseDsl(text: string): unknown;
export function printDsl(value: unknown): string;
```

Pack/spr: `encode` / `decode` returning `Uint8Array` / typed value.

## Implementation

Import from plugin WASM package (`@semio-tech/<plugin>-plugin` or generated bindgen). Delegate to Rust exports registered in `🌉️wasm` / plugin crate.

## Tests

Vitest: same fixture as Rust round-trip; bytes or strings must match Rust `component.rs` tests.

## Taxonomy

`ecosystems.🟦️typescript.leafFilename` = `🟦️component.ts`; `⚛️react` target keeps `🟦️component.tsx`.

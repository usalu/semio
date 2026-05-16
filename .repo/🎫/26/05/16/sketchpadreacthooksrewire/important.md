# Sketchpad React hooks rewire

## Outcome

- `semio/client/lib/react/index.tsx` is now a **barrel** that re-exports `./logic/index` so sketchpad’s `@semio/react` → `../react` resolves to the CQRS hook surface (`SemioStoreKitLineHost`, field reads, operation binds).
- **Sketchpad `tsconfig.json`**: fixed `extends` path (`../../js/tsconfig.json` was wrong as `../js`).
- **`sketchpad/react/AGENTS.md`**: documented **reads vs operations** and when to use `SemioStoreKitLineHost` vs wasm registry snapshot path.
- **Reverted**: importing a copied legacy `render-kit-wasm` bundle — current `@semio/js` no longer exports `KitStoreClient`, `createKitStoreClient`, `InMemoryKitStore`, etc., so that layer does not typecheck.

## Blocker for full UI rewire

Sketchpad still depends on **wasm kit registry** symbols (`KitStoreProvider`, `useKitStoreSnapshot`, `executeSemioKitCommand`, …) that lived in an older merged `@semio/react` + `@semio/js` API. Restoring sketchpad end-to-end requires either:

1. Re-exporting / re-implementing those KitStore WASM APIs in `@semio/js` again, or  
2. Replacing sketchpad call sites incrementally with CQRS hooks + `SemioStoreKitLineHost` (large change across `index.tsx`).

## Files touched

- `semio/client/lib/react/index.tsx`
- `semio/client/lib/sketchpad/react/tsconfig.json`
- `semio/client/lib/sketchpad/react/AGENTS.md`
- `semio/client/lib/react/logic/index.tsx` (Camera re-export retained from earlier work)

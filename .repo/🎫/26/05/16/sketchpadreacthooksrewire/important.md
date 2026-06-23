# Sketchpad React hooks rewire

## Outcome

- `compose/client/lib/react/index.tsx` is now a **barrel** that re-exports `./logic/index` so sketchpad’s `@compose/react` → `../react` resolves to the CQRS hook surface (`ComposeStoreKitLineHost`, field reads, operation binds).
- **Sketchpad `tsconfig.json`**: fixed `extends` path (`../../js/tsconfig.json` was wrong as `../js`).
- **`sketchpad/react/AGENTS.md`**: documented **reads vs operations** and when to use `ComposeStoreKitLineHost` vs wasm registry snapshot path.
- **Reverted**: importing a copied legacy `render-kit-wasm` bundle — current `@compose/js` no longer exports `KitStoreClient`, `createKitStoreClient`, `InMemoryKitStore`, etc., so that layer does not typecheck.

## Blocker for full UI rewire

Sketchpad still depends on **wasm kit registry** symbols (`KitStoreProvider`, `useKitStoreSnapshot`, `executeComposeKitCommand`, …) that lived in an older merged `@compose/react` + `@compose/js` API. Restoring sketchpad end-to-end requires either:

1. Re-exporting / re-implementing those KitStore WASM APIs in `@compose/js` again, or  
2. Replacing sketchpad call sites incrementally with CQRS hooks + `ComposeStoreKitLineHost` (large change across `index.tsx`).

## Files touched

- `compose/client/lib/react/index.tsx`
- `compose/client/lib/sketchpad/react/tsconfig.json`
- `compose/client/lib/sketchpad/react/AGENTS.md`
- `compose/client/lib/react/logic/index.tsx` (Camera re-export retained from earlier work)

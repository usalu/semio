# TODO

1. Investigate why imported component from `@semio/core` doesn't work

```
Uncaught SyntaxError: The requested module '/@fs/C:/git/semio/javascript/node_modules/use-sync-external-store/shim/with-selector.js?v=928a5be9' does not provide an export named 'default' (at traditional.mjs?v=928a5be9:2:8)
```

Seems to be related to xyflow and zustand:

- https://github.com/xyflow/xyflow/issues/4893
- https://github.com/Uniswap/web3-react/issues/379
- https://github.com/pmndrs/zustand/pull/550

1. Setup `vite.main.config.mts`, `vite.main.config.mts` and `vite.renderer.config.mts` in `forge.config.ts`

# Compatibility

- Mac OS @electron/notarize

```ts
import { defineConfig } from "vite";
import baseConfig from "@semio/core/vite.config";

export default defineConfig({
  ...baseConfig,
});
```

See: https://stackoverflow.com/questions/75132236/how-to-share-vite-config-in-monorepo
https://github.com/vercel/turborepo/discussions/3323

- https://www.electronforge.io/config/configuration with https://github.com/electron/rebuild

# Useful links

- https://github.com/stephenhandley/electron-forge-vite-typescript/blob/typescript-react/forge.config.js

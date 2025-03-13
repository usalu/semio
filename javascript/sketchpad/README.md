# Compatibility

- Mac OS @electron/notarize

# TODO

- Setup `vite.main.config.mts`, `vite.main.config.mts` and `vite.renderer.config.mts` in `forge.config.ts`

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

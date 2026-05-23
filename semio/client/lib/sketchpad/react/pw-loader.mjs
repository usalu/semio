// #region 🔖Header
// Playwright ESM loader hook for the sketchpad workspace.
// Handles: CSS stubs, TypeScript via esbuild (proper type stripping),
// Vite import.meta.glob stubs, and JSON imports without type attributes.
// Registered via --import in playwright.config.ts NODE_OPTIONS.
// #endregion 🔖Header

import { register } from 'node:module';
import { fileURLToPath } from 'node:url';
import { resolve, dirname } from 'node:path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const loaderHookPath = resolve(__dirname, 'pw-loader-hooks.mjs');
register(new URL('file://' + loaderHookPath));

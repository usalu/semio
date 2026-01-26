# Summary: Vitest Multiple Projects Warning

## Issue
The VS Code Vitest extension found 6 config files (`vitest.config.*` and `vite.config.*`) but only uses the first 5 by default.

## Root Cause
The extension scans for all Vite/Vitest configs. Only 2 actually contain test configuration:
- `vitest.config.ts` (root)
- `js/semio/vite.config.ts`

The other 4 (`js/play`, `js/vscode`, `js/sketchpad`, `js/temp`) are pure Vite build configs without tests.

## Recommended Solution
Create a `vitest.workspace.ts` file at the root to explicitly define which configs are used for testing:

```typescript
import { defineWorkspace } from 'vitest/config'

export default defineWorkspace([
  'vitest.config.ts',
  'js/semio/vite.config.ts',
])
```

This is the standard Vitest approach for monorepos and provides the best performance.

## Alternative Quick Fix
Add to `.vscode/settings.json`:
```json
{
  "vitest.maximumConfigs": 10
}
```

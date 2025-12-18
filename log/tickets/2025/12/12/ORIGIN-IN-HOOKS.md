---
slug: ORIGIN-IN-HOOKS
summary: Hooks auto-inject command origins via useOrigin
prompt: Hooks auto-inject command origins via useOrigin
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.911Z'
commit: '0000000000000000000000000000000000000000'
iterations: []
---

# Previously

The codebase has hooks that expose `origin` as a parameter in their returned action functions. Per AGENTS.md, hooks should internally use `useOrigin()` to get the origin automatically instead of exposing it as a parameter.

Current pattern (incorrect):

- `useDesignAppAddPiece(): ActionHookResult<[origin: string, piece: Piece]>` - origin exposed
- `useKitAppCommands()` returns functions like `undo: (origin: string) => ...`
- `useTypeAppCommands()` has `_origin?: string` parameters that are unused

Correct pattern (as seen in `useKitAppTransaction`):

- Uses `const getOrigin = useOrigin();` internally
- Action functions don't take origin parameter: `start: () => controller.execute("...", getOrigin())`

# Plan

1. Fix Design.tsx:
   - `TransactionActions` interface - remove origin parameters
   - `useDesignAppTransaction` - add useOrigin internally
   - All action hooks (useDesignAppUndo, useDesignAppAddPiece, etc.) - remove origin from ActionHookResult type args and use useOrigin internally

2. Fix Kit.tsx:
   - `useKitAppCommands` - add useOrigin internally and remove origin parameters from all returned functions

3. Fix Type.tsx:
   - `useTypeAppTransaction` - remove origin parameter, add useOrigin internally
   - `useTypeAppCommands` - remove \_origin parameters from all returned functions (already unused but need cleanup)

4. Run tsc to verify changes

# Changes

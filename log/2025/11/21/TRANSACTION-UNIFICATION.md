---
slug: TRANSACTION-UNIFICATION
summary: Migration from 2025-11-21_TRANSACTION-UNIFICATION.md
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.688Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Transaction Pattern Unification - COMPLETED

## Summary

Successfully unified all transaction patterns to use a single `Transaction` object interface throughout the codebase. This eliminates the dual pattern problem and simplifies the API.

## Changes Implemented

### 1. Core Transaction Interface (elements.tsx)

- Kept `Transaction` interface with `start`, `finalize`, `abort` methods
- Removed individual `startTransaction`, `finalizeTransaction`, `abortTransaction` props from:
  - `InputProps`
  - `TextareaProps`
  - `Select` component signature
- Components now only accept `transaction?: Transaction`

### 2. Transaction Hooks Added

- `useKitTransaction(origin: string): Transaction` in App.tsx
- `useKitAppTransaction(origin: string, id?: KitAppId): Transaction` in apps/kit/App.tsx
- `useTypeAppTransaction(origin: string, id?: TypeAppId): Transaction` in apps/type/App.tsx

### 3. Command Hooks Updated

- Removed `startTransaction`, `finalizeTransaction`, `abortTransaction` from:
  - `useKitCommands()` return value
  - `useKitAppCommands()` return value
  - Commands hooks only return action methods now

### 4. Component Usage Updated

- Kit App: All Input/Textarea components now use `transaction={useKitAppTransaction("id")}`
- Type App: All Stepper/Slider components now use `transaction={useKitTransaction("id")}`
- Removed all individual transaction prop destructuring from kit commands

## Benefits Achieved

✅ **Single Pattern**: One consistent way to handle transactions
✅ **Cleaner Components**: No prop ambiguity or fallback logic
✅ **Better Type Safety**: Single `Transaction` interface
✅ **Easier to Use**: Just call the hook with an origin ID
✅ **More Maintainable**: Less code, clearer intent

## Usage Pattern

### Before (Mixed Patterns)

```tsx
// Pattern 1 - Individual props
const { startTransaction, finalizeTransaction, abortTransaction } = useKitAppCommands();
<Input startTransaction={() => startTransaction?.("id")} finalizeTransaction={() => finalizeTransaction?.("id")} abortTransaction={() => abortTransaction?.("id")} />;

// Pattern 2 - Transaction object
const { startTransaction, finalizeTransaction, abortTransaction } = kitCommands || {};
<Input
  transaction={{
    start: () => startTransaction?.("id"),
    finalize: () => finalizeTransaction?.("id"),
    abort: () => abortTransaction?.("id"),
  }}
/>;
```

### After (Unified Pattern)

```tsx
<Input transaction={useKitTransaction("id")} />
<Input transaction={useKitAppTransaction("id")} />
<Input transaction={useTypeAppTransaction("id")} />
```

## Migration Notes

This is a **breaking change** for any code that:

- Uses individual transaction props on Input/Textarea/Select
- Destructures transaction methods from command hooks
- Expects useKitCommands to return transaction methods

All internal usage has been updated. External consumers will need to:

1. Replace individual transaction props with transaction object
2. Use new transaction hooks instead of getting methods from command hooks

# Previously

Sketchpad UI elements accepted an optional `transaction` prop that could override the ambient `TransactionProvider` context. This leaked transaction plumbing through most form controls and encouraged repeated `useKitTransaction()` / `useKitAppTransaction()` calls at call sites.

# Plan

- Remove `transaction` props from `js/semio/sketchpad/elements.tsx` components.
- Make elements always resolve transactions via `useTransaction()` (context only).
- Move transaction wiring to app roots via `TransactionProvider` so all descendant elements participate.

# Changes

- `js/semio/sketchpad/elements.tsx`: removed `transaction` prop surface area and the prop-vs-context resolver; elements now read transactions exclusively via `useTransaction()`.
- `js/semio/sketchpad/Kit.tsx`: wrapped the Kit app UI in a `TransactionProvider` using `useKitAppTransaction()`; removed all `transaction={...}` usages on inputs/textarea.
- `js/semio/sketchpad/Type.tsx`: wrapped the Type app UI in a `TransactionProvider` using `useKitTransaction()`; removed all `transaction={...}` usages on sliders/steppers.

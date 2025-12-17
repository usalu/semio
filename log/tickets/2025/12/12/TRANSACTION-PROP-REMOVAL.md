---
slug: TRANSACTION-PROP-REMOVAL
summary: Remove transaction props from elements
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.920Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

Sketchpad UI elements accepted an optional `transaction` prop that could override the ambient `TransactionProvider` context. This leaked transaction plumbing through most form controls and encouraged repeated `useKitTransaction()` / `useKitAppTransaction()` calls at call sites.

# Plan

- Remove `transaction` props from `js/js/sketchpad/elements.tsx` components.
- Make elements always resolve transactions via `useTransaction()` (context only).
- Move transaction wiring to app roots via `TransactionProvider` so all descendant elements participate.

# Changes

- `js/js/sketchpad/elements.tsx`: removed `transaction` prop surface area and the prop-vs-context resolver; elements now read transactions exclusively via `useTransaction()`.
- `js/js/sketchpad/Kit.tsx`: wrapped the Kit app UI in a `TransactionProvider` using `useKitAppTransaction()`; removed all `transaction={...}` usages on inputs/textarea.
- `js/js/sketchpad/Type.tsx`: wrapped the Type app UI in a `TransactionProvider` using `useKitTransaction()`; removed all `transaction={...}` usages on sliders/steppers.

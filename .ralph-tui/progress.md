# Ralph Progress Log

This file tracks progress across iterations. Agents update this file
after each iteration and it's included in prompts for context.

## Codebase Patterns (Study These First)

- **Kit store assets:** Canonical shape is `semio.kit_store.bundle` with `rootSnapshot`, ordered `semanticOpLog`, optional `histories` (checkpoint/draft/transaction metadata over the same op model), and `backbonePointers`. Document the intent in `semio/assets/semio/kit-store.contract.semio.json`; pair `kit-store.golden.ops.semio.json` with `kit-store.golden.expected.semio.json` for RS replay tests (`projectionFingerprint` = blake3-style `hash::h` over sorted piece centers) and lightweight JS fixture parses.
- **Root pnpm for semio slice:** A minimal `pnpm-workspace.yaml` including only `semio/js`, `semio/react`, and `semio/assets` avoids `pnpm install` pulling packages that depend on `file:../rs/pkg` before `wasm-pack build` populates `semio/rs/pkg`.

---

## 2026-05-06 - US-001

- **What was implemented:** Kit asset contracts aligned to **one root snapshot + ordered semantic ops** with checkpoint/draft/transaction wrappers documented in JSON; golden ops/expected pair; `metabolism.new.kit.semio.json` replaced with a minimal bundle exemplar; RS tests replay golden ops and assert invariants/fingerprint; `@semio/js` embedded tests load golden + bundle paths for structural checks; root `pnpm typecheck` / `pnpm lint` validate the touched packages.
- **Files changed:** `semio/assets/semio/kit-store.contract.semio.json`, `kit-store.golden.*.semio.json`, `metabolism.new.kit.semio.json`, `semio/rs/lib.rs`, `semio/js/index.ts`, root `package.json`, `pnpm-workspace.yaml`, `.npmrc`, `eslint.config.mjs`, plus prior workspace/JS fixes from this epic (see git status for full set).
- **Learnings:**
  - **Patterns discovered:** Same ordered op log underlies snapshot projection and history wrappers—difference is metadata/lifecycle, not a second persistence shape. Golden fixtures should encode **invariants** (`sortedPieceCenters`, counts) plus a stable **fingerprint** for deterministic CI.
  - **Gotchas encountered:** Full pnpm workspace that includes `semio/algorithms` breaks install until `semio/rs/pkg` exists; narrow the workspace or document wasm-pack as a prereq. Legacy `KitStoreHandle` / `eventStream` GraphQL expectations in JS need a follow-up (e.g. US-006) rather than half-wiring old APIs.
---


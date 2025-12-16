---
slug: KIT-ROW-EXPAND
summary: Fix kit table row expansion
---
# Previously
- Kit table row expansion triggered `semio.kitApp.toggleExpandedRow` but the UI never reflected the change, implying the machine-facing state was not updated by the interaction.

# Plan
- Trace the existing row toggle flow and identify where the UI state fails to update.
- Align the toggle handler so the XState slice updates immediately alongside the Y.js command.
- Refresh dev docs to describe the dual-dispatch pattern and record the work in this log.

# Changes
- Updated `Kit.tsx` row toggles to dispatch `useKitAppToggleRow` before issuing `semio.kitApp.toggleExpandedRow`, and wired all call sites through the shared helper to keep XState and Y.js in lockstep.
- Documented the kit table expansion pattern in `AGENTS.md` and `README.md` for future contributors.

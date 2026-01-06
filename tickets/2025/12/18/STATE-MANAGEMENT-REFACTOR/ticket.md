---
slug: STATE-MANAGEMENT-REFACTOR
prompt: Analyze js/js codebase for state management inconsistencies and refactor to triadic hooks pattern
summary: Refactor js/js state management to triadic hooks pattern
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-18T07:25:30.399Z
iterations:
  - prompt: Analyze js/js codebase for state management inconsistencies and refactor to triadic hooks pattern
    model: claude-sonnet-4-20250514
    date:
      started: 2025-12-18T07:25:30.399Z
      ended: 2025-12-18T16:05:57.155Z
    author: Unknown
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
  - prompt: "Refactor Sketchpad.tsx to obey open/closed: remove all app-specific logic from core; apps must be addable/removable by adding/removing their module."
    model: gpt-5.2-codex
    date:
      started: 2025-12-18T16:06:08.806Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
---


# Previously

Analysis of js/js/sketchpad state management revealed:

## Correct Pattern (Type.tsx, Design.tsx, Kit.tsx, Quality.tsx)

- Triadic hooks: `[STATE, SETSTATE, CANSETSTATE] = useSELECTOR()`
- Uses `conditionalHookResult()` and `readonlyHookResult()` from shared.ts
- XState actor with `useSelector` for state, `actor.send()` for mutations
- Commands executed only by machine, components use state setters

## Inconsistencies Found

### Feedback.tsx

- Uses local `useState` for all form state instead of XState
- Components manage their own state directly
- No triadic hooks pattern

### Home.tsx

- Mixed patterns: some XState hooks, some local useState
- `HomeDropZone` uses direct `actor.send()` in component
- Settings panel uses triadic hooks correctly

### Docs.tsx

- Custom headings state with own observer pattern (not XState)
- `HeadingsProvider` manages state outside machine

### General Problems

- Some components call commands directly instead of using state setters
- Not all kit state uses fine-grained subscriptions

# Plan

1. Refactor Feedback.tsx to use XState-backed triadic hooks
2. Refactor Home.tsx to consistent pattern
3. Refactor Docs.tsx headings to use XState
4. Audit all command usage - ensure only machine uses commands
5. Add fine-grained kit state subscriptions where missing
6. Run tests to verify

# Changes

## Iteration 1 - Feedback.tsx Refactoring

### Completed

1. Added `FeedbackAppState`, `FeedbackFormData`, `FeedbackKind`, `FeedbackAppKind` types to Sketchpad.tsx
2. Added `feedbackApp` to `SketchpadContext` interface
3. Added FEEDBACK events to `SketchpadEvent` type:
   - `FEEDBACK.TOGGLE_PANEL`
   - `FEEDBACK.SET_FORM_DATA`
   - `FEEDBACK.RESET_FORM`
   - `FEEDBACK.SET_SUBMITTING`
   - `FEEDBACK.SET_SUBMITTED`
   - `FEEDBACK.SET_ERROR`
4. Added feedbackApp to machine's initial context
5. Created triadic hooks in Feedback.tsx:
   - `useFeedbackFormData()`: HookResult<FeedbackFormData>
   - `useFeedbackIsSubmitting()`: HookResult<boolean>
   - `useFeedbackIsSubmitted()`: HookResult<boolean>
   - `useFeedbackError()`: HookResult<string | undefined>
   - `useFeedbackReset()`: [action, canAct]
6. Refactored FeedbackForm to use triadic hooks instead of useState
7. Registered event handlers and runtime actions for all FEEDBACK events

### Test Results

- 8/9 Feedback App tests pass
- 1 failing test (toolbar visibility) is pre-existing UI issue, not state management related

## Remaining Work

### Home.tsx

- Currently uses `useHomeCommands()` pattern instead of triadic hooks
- Hooks like `useHomeSelection()`, `useHomePanelVisibility()` return values directly, no setters
- Local `useState` for `isDragging` and `focusedItemId` is appropriate (transient UI state)
- **Recommendation**: Create triadic hooks similar to Type/Design apps

### Docs.tsx

- Uses custom `headingsState` with own observer pattern outside XState
- `HeadingsProvider` manages state independently
- **Recommendation**: Integrate headings state into XState context

### General Patterns

- Type.tsx: ✓ Uses triadic hooks correctly
- Design.tsx: ✓ Uses triadic hooks correctly
- Kit.tsx: Uses commands pattern, similar to Home
- Quality.tsx: Uses commands pattern, similar to Home

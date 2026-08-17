# UI Action Group Retention Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Component SHA-256: `787d1fa89d5fe4dfa5a85e6a4fca1a51b6ba571b558a840c55eb8a0ade1b736f`
- Story SHA-256: `fb8518508d1c9c9521d05e4abc01f36e71465d2225e40a4bd90fed50fc611bda`
- Both definition paths were clean at inspection time.

## Production Closure

The framework `Window` component directly renders `ActionGroup` and multiple `ActionGroupItem` children for external-window, focus, and close interactions. This is a live production semantic component, not glue, a story, a test, or a generated mirror. Framework class-name and port documentation references are not additional consumers and do not affect the decision.

## Decision

Retain `ActionGroup` as a specifically named UI component. It is not a zero-consumer deletion candidate. No edit is authorized from this audit. Its module qualification is not established by this limited pass; a later full census must calculate all terminal production consumers before considering any owner move.

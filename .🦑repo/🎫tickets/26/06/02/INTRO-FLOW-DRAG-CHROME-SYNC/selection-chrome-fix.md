# Flow title select + drag chrome

## Symptom

- Select intro title: text jumps right; selection box looks correct.
- Start drag: text snaps back; selection box spans full slide width.

## Cause

`contentInkStyle` repositioned `__content` to the ink frame on select (moving text). On drag, `contentInkStyle` is cleared (text correct) but chrome stayed `inset: 0` on full-width content.

## Fix

Position selection chrome with `interactiveDispositionChromeStyle(inkInWrapper)`; do not apply ink frame to content.

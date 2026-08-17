# Research: Skipping a Demonstrator Introduction Restarts It

## Reproduction

The demonstrator brand enables `replayIntroductionOnLoad`. Pressing Skip calls `onDismiss(false)`, and `FrameworkOsShellInner.dismissIntroduction` dispatches `SET_INTRODUCTION_STEP` with `null`. The consolidated overlay reducer applies that transition correctly.

## Root Cause

The auto-start effect depended on `introductionStepIndex`. Dismissal changed the index to `null`, reran the effect, bypassed durable seen-state for the replay-on-load brand, and dispatched step `0`. The card's drag-local state then reset because the step remounted at its initial index, making Skip appear to reposition the window without closing it.

The nullable step index represented presentation state but did not record whether a launch had already auto-started. Merely removing the effect dependency would leave other re-entry paths, such as focus suppression toggles or tutorial completion, able to restart a dismissed introduction.

## Resolution

The overlay slice now records auto-started launch keys and accepts `AUTO_START_INTRODUCTION`. The reducer claims each brand/app launch key once per shell lifetime. Manual `SET_INTRODUCTION_STEP` remains available for the Introduce App command, while Skip/Done can set the active step to `null` without a later effect rearming the same automatic launch.

The regression test covers auto-start, Skip, a repeated auto-start event for the same demonstrator key, and auto-start for a different app key.

## Runtime Verification

The running mit-bestand demonstrator was opened at `http://127.0.0.1:6029/`, the Aggregator app was focused, and its app-owned “Die 3D-Ansicht” introduction was observed with one Skip button. Clicking Skip emitted one temporary `[DEBUG] introduction dismissed` console event. After 1.2 seconds, both the Skip button and introduction title had zero DOM matches, confirming that the overlay did not remount or reset to step `0`. The temporary diagnostic was then removed.

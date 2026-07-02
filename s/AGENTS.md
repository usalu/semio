---
technology: semios
emoji: 🖥️
---

# Semios

Semios is the collaborative operating system for designers — the umbrella technology unifying every playground technology.

## Programs

A **program** is a collection of apps. Sketchpad (`compose.sketchpad`) is a program, not a standalone product entry.

## Studios

A **studio** is the persistence and collaboration unit. Studios are local-first with an optional authoritative backbone (`dev://` single JSON for MVP).

## Resources

Apps yield typed **resources**. Resources of the same kind are interchangeable in the studio media graph.

## CQRS

Nothing is CRUD-edited. All studio mutations dispatch commands; state is event-sourced with checkpoints, alternatives, and undo/redo.

## Layering

`s/core` → `framework/product/os/core` → `framework/product/platform/core`

S is an **os instance**: composition (store, media graph, program registry, app host resolution) lives in `@semio-tech/framework-os-core`; `s/core` adds S branding (`S_SYSTEM_PROGRAM`), technology registration, and the S playground harness in `s/core/playground.ts`.

Standalone dev for any technology: `bun ./script.ts dev <kind>` via `@semio-tech/framework-playground-dev`.

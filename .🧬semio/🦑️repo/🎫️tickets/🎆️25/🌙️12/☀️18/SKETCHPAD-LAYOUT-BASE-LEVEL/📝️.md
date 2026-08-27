# Ticket

## Todos

# Previously

- Global Sketchpad chrome (Navbar/Footer) derives its background from `useLevel()`.
- When Sketchpad was rendered under non-base level providers, Navbar/Footer could incorrectly inherit a non-base level and appear transparent/incorrect.

# Plan

- Wrap the global Sketchpad layout root in `LevelProvider level="base"`.
- Document the invariant in root docs so background expectations stay consistent.

# Changes

- Wrapped the top-level Sketchpad `Layout` in `LevelProvider level="base"`.
- Documented the global base level provider invariant in `README.md` and `AGENTS.md`.

## Changes

## Log

## Summary

# Summary

Wrap global Sketchpad layout in base LevelProvider

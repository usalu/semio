---
goal: R26-02
---

# Ticket

## Summary

Fixed pytest discovery errors by addressing three root causes: (1) Root pyproject.toml had wrong testpaths ("py/semio", "py/engine" → "semio/py", "semio/engine"), (2) Created root conftest.py that pre-imports semio and engine modules with correct sys.path entries and sets semio.__path__ to prevent pytest's importlib mode from replacing the single-file module with a namespace package, (3) Re-synced uv workspace (uv sync --all-packages) to regenerate stale .pth files that had wrong paths. All 52 tests now discovered and passing from root and member directories.
## Changes

## Log

## Todos

## Plan

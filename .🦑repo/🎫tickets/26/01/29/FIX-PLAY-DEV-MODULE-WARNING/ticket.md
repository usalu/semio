# Ticket

## Todos

- [x] Add `"type": "module"` to `js/play/package.json`
- [x] Verify play dev runs without warning

## Changes

- `js/play/package.json`: Added `"type": "module"` to eliminate `MODULE_TYPELESS_PACKAGE_JSON` warning

## Log

Node was warning about `MODULE_TYPELESS_PACKAGE_JSON` because `js/play/package.json` lacked `"type": "module"`. Added it and confirmed play dev starts cleanly on port 4002.

## Summary

Bulk close

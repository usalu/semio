# Plan: Make VSCode Extension Compatible with Windsurf

## Problem Analysis

Windsurf is a VSCode-based IDE fork. From the screenshot:
- Windsurf Version: 1.13.9
- VSCode OSS Version: **1.106.0**

The current extension's `package.json` specifies:
```json
"engines": {
  "vscode": "^1.106.1"
}
```

This means the extension requires VSCode 1.106.1 or higher, but Windsurf is based on VSCode OSS 1.106.0.

## Solution

1. **Lower the VSCode engine version** from `^1.106.1` to `^1.106.0` to include Windsurf's base version

2. **Verify no API changes** between 1.106.0 and 1.106.1 that the extension depends on (since both are minor patch versions, this is unlikely)

## Implementation Steps

1. Edit `js/vscode/package.json`:
   - Change `"vscode": "^1.106.1"` to `"vscode": "^1.106.0"`

2. Update `@types/vscode` dependency if needed (currently `^1.108.1` which is fine)

3. Update documentation in `README.md` under components section to mention Windsurf compatibility

4. Update `AGENTS.md` codebase section for the VSCode extension

## Files to Modify

- `js/vscode/package.json` - engine version change
- `README.md` - component documentation
- `AGENTS.md` - codebase documentation

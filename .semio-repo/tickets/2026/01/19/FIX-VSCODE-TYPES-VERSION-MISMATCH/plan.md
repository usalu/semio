# Plan: Fix VS Code Types Version Mismatch

## Problem
The `vsce package` command fails with:
```
ERROR  @types/vscode ^1.108.1 greater than engines.vscode ^1.106.0
```

## Root Cause
In `js/vscode/package.json`:
- `engines.vscode` is set to `^1.106.0` (line 10)
- `@types/vscode` is set to `^1.108.1` (line 455)

The `@types/vscode` version must not exceed the `engines.vscode` version as it would mean the extension uses type definitions for APIs that may not exist in the target VS Code version.

## Solution
Upgrade `engines.vscode` from `^1.106.0` to `^1.108.0` to match the types version.

This is preferred over downgrading `@types/vscode` because:
1. We want access to the latest VS Code APIs
2. VS Code 1.108 is widely available (released several months ago)
3. Keeps types and engine in sync

## Steps
1. Update `engines.vscode` in `js/vscode/package.json` from `^1.106.0` to `^1.108.0`
2. Rebuild the extension to verify the fix

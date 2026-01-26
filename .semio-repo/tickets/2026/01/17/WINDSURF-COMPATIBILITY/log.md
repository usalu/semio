# Log

## 2026-01-17

### Analysis

User provided screenshot showing Windsurf IDE info:
- Windsurf Version: 1.13.9
- Windsurf Extension Version: 1.48.2
- VSCode OSS Version: 1.106.0

The VSCode extension currently requires `^1.106.1` which excludes Windsurf's base version 1.106.0.

### Solution

Lowering the engine version from `^1.106.1` to `^1.106.0` will make the extension compatible with both:
- VSCode 1.106.0+ (including Windsurf)
- VSCode 1.106.1+ (original requirement)

Since 1.106.0 → 1.106.1 is a patch version bump, no breaking API changes are expected.

### Implementation

1. Updated `js/vscode/package.json`:
   - Changed engine version from `^1.106.1` to `^1.106.0`

2. Updated `README.md`:
   - Added Windsurf compatibility note to the VSCode extension section

3. Updated `AGENTS.md`:
   - Added Windsurf compatibility note and engine version info to js/vscode/ section

All changes completed successfully.

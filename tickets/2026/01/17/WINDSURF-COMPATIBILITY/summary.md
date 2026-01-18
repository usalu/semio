# Summary

Made the VSCode extension compatible with Windsurf IDE by lowering the VSCode engine version requirement from `^1.106.1` to `^1.106.0`. Windsurf is based on VSCode OSS 1.106.0, which was previously excluded by the version constraint.

## Changes

- **js/vscode/package.json**: Updated engine requirement to `^1.106.0`
- **README.md**: Added Windsurf compatibility note to the extension description
- **AGENTS.md**: Added Windsurf compatibility note and engine version to the js/vscode/ documentation

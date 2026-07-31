---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-FILE-MECHANISM
---

# Ticket

## Summary

Added 📜️:script file kind to the header ID system. fileKindEmoji now returns 📜️ for script kind. FileHeaderId reads code file content and overrides kind to script when the first line is a shebang. headerPolicy detects 📜️ in file headers. Extended TestFileHeaderId with shebang and additional kind subtests. Added TestDeriveFileKind (32 cases) and TestFileKindEmoji (9 cases). Updated AGENTS.md and README.md documentation.

## Changes

- `repo/cli/main.go`: Added `case "script": return "📜️"` to `fileKindEmoji`
- `repo/cli/main.go`: Modified `FileHeaderId` to read file content and detect shebangs for code files, overriding kind to `script`
- `repo/cli/main.go`: Added `📜️` to emoji detection set in `headerPolicy`
- `repo/cli/main_test.go`: Updated `TestFileHeaderId` with corrected script emoji, added shebang subtests, resource/license cases
- `repo/cli/main_test.go`: Added `TestDeriveFileKind` with 32 test cases covering all file kinds
- `repo/cli/main_test.go`: Added `TestFileKindEmoji` with 9 test cases covering all kinds plus unknown/empty
- `repo/cli/main_test.go`: Added `file script`, `file resource`, `file license` to `TestArtifactIDAndURI`
- `AGENTS.md`: Updated docs for script file kind and shebang detection
- `README.md`: Updated docs for script file kind

## Log

- Explored current file kind system: `DeriveFileKind`, `FileHeaderId`, `fileKindEmoji`, `headerPolicy`, `GetArtifactID`
- Found `fileKindEmoji` missing `script` case (fell through to default 📄️)
- Found `headerPolicy` missing 📜️ in emoji detection set
- Found `FileHeaderId` only derived kind from filename, not content
- Implemented: added 📜️ to `fileKindEmoji`, shebang detection in `FileHeaderId`, 📜️ in `headerPolicy`
- All new and existing tests pass

## Todos

- [x] Add 📜️ to `fileKindEmoji`
- [x] Add shebang detection to `FileHeaderId`
- [x] Add 📜️ to `headerPolicy` emoji detection
- [x] Extend tests
- [x] Update docs

---
goal: AI-OPTIMIZED-REPO/REPO-BINARY
---

# Ticket

## Summary

Refactored ID system to entity-emoji-prefix format. Updated GetArtifactID, GetArtifactURI, IdToUri, UriToId, all struct GetID methods, Node resolver. Added interaction support. Fixed URI consistency. All Go tests pass.

## Changes

- `repo/cli/main.go`: Rewrote GetArtifactID, GetArtifactURI, IdToUri, UriToId with entity-emoji-prefix format. Updated all struct GetID() methods (Project, Bundle, Folder, File, Definition, Ticket, Policy). Refactored Node resolver to use new stripPrefix/stripAnyPrefix pattern. Added interaction emoji constants and interactionKindEmoji helper. Fixed GetArtifactURI to use PathToUriPath for consistent uppercase URIs. Fixed ticket/todo URI uppercasing.
- `repo/cli/main_test.go`: Updated TestArtifactIDAndURI (50+ test cases), TestIdToUri (40+ test cases), TestUriToId (38+ test cases), TestIdUriRoundTrip (10 test cases) to match new ID format. Added interaction entity test cases.
- `repo/vscode/extension.test.ts`: Updated file node and policy node test data to match new entity-emoji-prefix format.

## Log

1. Analyzed current ID system across cli, vscode, server
2. Updated emoji constants (added EmojiInteractions, EmojiInteractionStarted, EmojiInteractionFinished, EmojiInteractionRestarted, EmojiInteractionDeleted, changed EmojiFolderRequired to 🛅)
3. Rewrote GetArtifactID with entity-emoji-prefix format
4. Added interactionKindEmoji helper function
5. Rewrote IdToUri with new entity-then-kind parsing
6. Rewrote UriToId with new format generation
7. Added interaction support to GetArtifactURI
8. Updated emojiText for 🗑️
9. Updated all 4 test functions with new format expectations
10. Fixed GetArtifactURI uppercase consistency (project, bundle, folder)
11. Fixed IdToUri/GetArtifactURI ticket slug uppercasing
12. Fixed definition_interface test expectation
13. Added UriToId handling for bare sections/definitions paths
14. Updated all struct GetID() methods to entity-emoji-prefix
15. Refactored Node resolver for new ID parsing
16. Fixed Ticket.GetURI uppercase
17. Updated VS Code extension test data

## Todos

- [x] Analyze current ID system
- [x] Update emoji constants
- [x] Rewrite GetArtifactID
- [x] Rewrite IdToUri
- [x] Rewrite UriToId
- [x] Update GetArtifactURI
- [x] Update TestArtifactIDAndURI
- [x] Update TestIdToUri
- [x] Update TestUriToId
- [x] Update TestIdUriRoundTrip
- [x] Fix GetArtifactURI uppercase consistency
- [x] Update struct GetID() methods
- [x] Refactor Node resolver
- [x] Update VS Code extension tests
- [x] Run and verify all tests pass

## Plan

Entity emoji prefix format: `entityEmoji + kindEmoji + value`

- project: 🏗️🏘️semio (entity=🏗️, kind=👤, value=semio)
- bundle: 📦📚semio/js (entity=📦, kind=📚, value=semio/js)
- folder: 📁🛅semio/js/src (entity=📁, kind=🛅, value=semio/js/src)
- file: 📄💻main.go (entity=📄, kind=💻, value=main.go)
- definition: 🏷️🛠️file.ts#Section§myFunc (entity=🏷️, kind=🛠️, value=file.ts#Section§myFunc)
- ticket: 🎫20250204test-ticket (entity=🎫, YYYYMMDD+slug, no separators)
- policy: 🛡️code-hygiene (entity=🛡️, no leading /)
- interaction: ⚡🌱🎫20260214TICKET (entity=⚡, kind=🌱, inner entity ID)

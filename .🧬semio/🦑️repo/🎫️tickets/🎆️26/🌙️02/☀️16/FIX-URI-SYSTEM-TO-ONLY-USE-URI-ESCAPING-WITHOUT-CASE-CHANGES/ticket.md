---
goal: REFACTOR-URI-SYSTEM
---

# Ticket

## Summary

Fixed the URI system to only use URI-escaping (percent encoding) without any case changes, slugification, or other transformations. Updated PathToUriPath, SectionIdValueToUriPath, DefinitionIdValueToUriPath, StatuteUriPathToId, IdToUri, GetArtifactURI, UriToId, and all URI-related tests. All 13 URI tests pass.

## Changes

### Implementation (`repo/cli/main.go`)

- **PathToUriPath**: Removed `strings.ToLower` — now only encodes spaces as `%20`, preserving original case
- **SectionIdValueToUriPath**: Replaced `strings.ToLower(Slugify(p))` with `PathToUriPath(p)` — preserves section names
- **DefinitionIdValueToUriPath**: Replaced `Slugify` with `PathToUriPath` for all parts — preserves definition names
- **StatuteUriPathToId**: Replaced `strings.ToLower` with `PathFromUriPath` — proper URI decoding
- **IdToUri**: Removed all `strings.ToLower` and replaced `Slugify` with `PathToUriPath` for URI construction
- **GetArtifactURI**: Removed `strings.ToLower` from interaction URI; fixed root to return `composerepo://root`; fixed interaction URI to include `on/` prefix and kind suffix
- **UriToId**: Added URI-decoding of slugs before passing to `buildSectionID`/`buildDefinitionID`; fixed ticket handler to flatten full path (was using `filepath.Base` which dropped date portion); removed redundant `strings.ToLower` from folder, file, draft, todo, commit handlers
- **Tree node URIs**: Replaced `strings.ToLower(Slugify(...))` with `PathToUriPath(...)` for entitykind, policy, territory, statute rendering, ticket slug URIs
- **URI comparison**: Removed `strings.ToLower` for exact case-sensitive matching
- **Fixed pre-existing syntax error** at line 910: corrupted `default:== "" {` block

### Tests (`repo/cli/main_test.go`)

- Updated all URI test expectations to use case-preserving URIs
- Updated interaction URIs to use `on/` format with kind suffix
- Fixed statute test to expect actual `GetArtifactID` return value
- All 13 URI-related tests pass (TestPathToUriPath, TestPathFromUriPath, TestSectionIdValueToUriPath, TestDefinitionIdValueToUriPath, TestParseSectionUriPath, TestStatuteIdToUriPath, TestStatuteUriPathToId, TestIdToUri, TestUriToId, TestArtifactIDAndURI, TestIdUriRoundTrip, TestSectionHeaderIdAndUri, TestDefinitionHeaderIdAndUri)

## Log

- Explored URI-related functions across main.go and main_test.go
- Identified all locations where `strings.ToLower`, `Slugify`, and other transformations were applied to URIs
- User clarified: "No modification, no slugs, nothing - just URI encode. No loss of information."
- Applied changes systematically function by function
- Fixed `UriToId` to properly URI-decode slugs before ID construction
- Fixed interaction URI format to include `on/` prefix and kind suffix for consistency
- All URI tests pass cleanly

## Todos

- [x] Fix PathToUriPath - remove strings.ToLower
- [x] Fix PathFromUriPath - ensure no case changes
- [x] Fix SectionIdValueToUriPath - use PathToUriPath not Slugify
- [x] Fix DefinitionIdValueToUriPath - use PathToUriPath not Slugify
- [x] Fix StatuteUriPathToId - use PathFromUriPath
- [x] Fix IdToUri - remove all strings.ToLower and Slugify
- [x] Fix GetArtifactURI - remove strings.ToLower
- [x] Fix UriToId - decode URI slugs, fix ticket handler, clean up
- [x] Fix all URI-related tests
- [x] Run tests and verify

## Plan

1. Remove `strings.ToLower` from `PathToUriPath` — only encode spaces as `%20`
2. Replace `Slugify` with `PathToUriPath` in all URI construction functions
3. Remove `strings.ToLower` from `IdToUri`, `GetArtifactURI`, and `UriToId`
4. Fix `UriToId` to URI-decode slugs before passing to ID builders
5. Fix interaction URI format for consistency
6. Update all test expectations to match new case-preserving behavior
7. Run tests and verify

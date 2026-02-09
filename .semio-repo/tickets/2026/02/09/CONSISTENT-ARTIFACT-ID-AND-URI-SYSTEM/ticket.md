---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS
---

# Ticket

## Summary

Made artifact ID and URI system consistent: fixed model GetID/GetURI for Contributor/Commit/Draft/Policy/ViolationKind, fixed tree node IDs to use model methods, fixed GetArtifactURI/IdToUri/UriToId for uppercase URIs and colon-to-path violation kind conversion, added ViolationKindIdToUriPath/ViolationKindUriPathToId helpers, added round-trip tests
## Changes

### semio-repo/cli/main.go
- Fixed Contributor.GetID() to use emoji prefix `👤` + Github handle
- Fixed Contributor.GetURI() to uppercase without Slugify
- Fixed Commit.GetID() to use emoji prefix `🔀` + SHA
- Fixed Commit.GetURI() to uppercase without Slugify
- Added Draft.GetID() and Draft.GetURI() methods
- Fixed Policy.GetURI() to use uppercase slug
- Fixed ViolationKindMeta.GetURI() to use path-based uppercase format (colon→slash)
- Added ViolationKindIdToUriPath() and ViolationKindUriPathToId() helper functions
- Fixed tree node IDs: draft uses d.GetID(), policy uses emojiText format, violationKind uses meta.GetID()
- Fixed tree node URIs: violationKind uses meta.GetURI() (path-based format instead of Slugify dashes)
- Fixed GetArtifactURI: policy (uppercase slug), violationKind (path-based), contributor (uppercase), commit (uppercase), draft (uppercase)
- Fixed IdToUri: policy (uppercase), violationKind (path conversion), contributor (uppercase via both early and late paths), commit (uppercase), draft (uppercase)
- Fixed UriToId: violationKind (ViolationKindUriPathToId), policy (lowercase), contributor (lowercase), commit (lowercase)

### semio-repo/cli/main_test.go
- Updated TestArtifactIDAndURI expected URIs: draft→MY-DRAFT, policy→CODE-HYGIENE, violationKind→CODE/INLINE-COMMENT, contributor→USALU, commit→ABC123
- Updated TestIdToUri expected URIs to match uppercase/path-based format
- Updated TestUriToId expected IDs with uppercase URIs returning properly-cased IDs
- Fixed pre-existing TestUriToId/projects emoji variation selector mismatch
- Changed violationKind test data from slash-separated to colon-separated (matching production format)
- Added TestViolationKindIdToUriPath with single/two/three segment coverage
- Added TestViolationKindUriPathToId with reverse conversion coverage
- Added TestIdUriRoundTrip covering 9 entity types (policy, violationKind, contributor, commit, draft, section, file, ticket, goal)

## Log

- Explored current ID/URI system, identified all inconsistencies
- Contributor.GetID() returned "contributor:Name" instead of "👤github"
- Commit.GetID() returned "commit:sha" instead of "🔀sha"
- Tree nodes for drafts, policies, violation kinds used hardcoded prefixes
- ViolationKindMeta.GetURI() didn't use path-based format
- Policy.GetURI() didn't uppercase

## Todos

- [x] Fix model GetID/GetURI methods
- [x] Fix tree node IDs
- [x] Fix GetArtifactURI consistency
- [x] Fix IdToUri/UriToId
- [x] Extend tests
- [x] Run tests and fix failures
- [x] Update documentation

## Plan

1. Fix model GetID() methods (Contributor, Commit, Draft)
2. Fix tree node ID assignments
3. Fix URI generation functions
4. Fix IdToUri/UriToId conversions
5. Update tests
6. Run tests until all pass

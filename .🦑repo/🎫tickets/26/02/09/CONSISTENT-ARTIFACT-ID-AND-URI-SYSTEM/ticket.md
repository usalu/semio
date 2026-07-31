---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS
---

# Ticket

## Summary

Made artifact ID and URI system consistent: fixed model GetID/GetURI for Contributor/Commit/Draft/Policy/Statute, fixed tree node IDs to use model methods, fixed GetArtifactURI/IdToUri/UriToId for uppercase URIs and colon-to-path statute conversion, added StatuteIdToUriPath/StatuteUriPathToId helpers, added round-trip tests

## Changes

### repo/cli/main.go

- Fixed Contributor.GetID() to use emoji prefix `👤` + Github handle
- Fixed Contributor.GetURI() to uppercase without Slugify
- Fixed Commit.GetID() to use emoji prefix `🔀` + SHA
- Fixed Commit.GetURI() to uppercase without Slugify
- Added Draft.GetID() and Draft.GetURI() methods
- Fixed Policy.GetURI() to use uppercase slug
- Fixed StatuteMeta.GetURI() to use path-based uppercase format (colon→slash)
- Added StatuteIdToUriPath() and StatuteUriPathToId() helper functions
- Fixed tree node IDs: draft uses d.GetID(), policy uses emojiText format, statute uses meta.GetID()
- Fixed tree node URIs: statute uses meta.GetURI() (path-based format instead of Slugify dashes)
- Fixed GetArtifactURI: policy (uppercase slug), statute (path-based), contributor (uppercase), commit (uppercase), draft (uppercase)
- Fixed IdToUri: policy (uppercase), statute (path conversion), contributor (uppercase via both early and late paths), commit (uppercase), draft (uppercase)
- Fixed UriToId: statute (StatuteUriPathToId), policy (lowercase), contributor (lowercase), commit (lowercase)

### repo/cli/main_test.go

- Updated TestArtifactIDAndURI expected URIs: draft→MY-DRAFT, policy→CODE-HYGIENE, statute→CODE/INLINE-COMMENT, contributor→USALU, commit→ABC123
- Updated TestIdToUri expected URIs to match uppercase/path-based format
- Updated TestUriToId expected IDs with uppercase URIs returning properly-cased IDs
- Fixed pre-existing TestUriToId/projects emoji variation selector mismatch
- Changed statute test data from slash-separated to colon-separated (matching production format)
- Added TestStatuteIdToUriPath with single/two/three segment coverage
- Added TestStatuteUriPathToId with reverse conversion coverage
- Added TestIdUriRoundTrip covering 9 entity types (policy, statute, contributor, commit, draft, section, file, ticket, goal)

## Log

- Explored current ID/URI system, identified all inconsistencies
- Contributor.GetID() returned "contributor:Name" instead of "👤github"
- Commit.GetID() returned "commit:sha" instead of "🔀sha"
- Tree nodes for drafts, policies, statutes used hardcoded prefixes
- StatuteMeta.GetURI() didn't use path-based format
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

# Content-Addressed Rollback Tags

## Audited Problem

The previous publication logic reused sha-<12 commit hex> for every build of one commit. Different image content from the same source revision therefore targeted the same rollback tag. The README and workflow comments described that tag as never moved.

## Contract

The new pure release helper accepts a closed version-1 request with the full 40 lowercase hexadecimal revision and Docker's sha256 image-configuration identity with its full 64 lowercase hexadecimal digest. Its output is sha-<12 commit hex>-<64 image-config digest hex>, exactly 81 characters. Exact input lengths reject trailing newlines as well as abbreviated identities. Workspace-version and latest aliases remain the publication owner's responsibility.

The neutral cases contain real SHA-256 identities of explicit configuration JSON bytes. They cover a first image, changed content at the same revision, repeated identical content, abbreviated identities, uppercase identities, an incorrect hash algorithm, a missing identity, unknown fields and trailing newlines. Tests compare the configuration hash from the system crypto library with the existing independent noble SHA-256 implementation and validate contracts with both the owned validator and Ajv.

## Documentation

README artifact, publication and rollback instructions and the workflow's publication comment now describe the content-addressed tag. The rollback command contains a full valid 81-character example and explicitly instructs operators to use the exact tag reported by their own publication.

## Validation Status

The first selected test invocation preceded config registration and returned no test files; it is not claimed as the intended missing-helper red. The registered test then failed because the release helper module did not exist. The helper was implemented only after that observed red. The same selected diagnostic suite then passed: one file, 13 tests, including Ajv and noble SHA-256 oracle comparisons. This is direct installed-tool unit evidence, not a canonical deployment-gate pass. No registry operation, deployment or publication has been performed for this task.

The deployment owner was advised to tag from the exact inspected imageId rather than the mutable local source tag, so a concurrent same-tag rebuild cannot substitute different bytes after rollback identity selection.

## Files Changed By This Release-Tag Agent

- `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🏷️release/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🏷️release/🧬️schema/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🏷️release/🧫️fixtures/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🏷️release-tags/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/README.md` (publication and rollback naming only)
- `.github/workflows/architecture-course.yml` (publication comment only)
- This retained ticket report.

The launch owner registered the selected test in the shared course Vitest configuration. The root deployment owner wires the helper into publication; that integration is outside this agent's source edits.

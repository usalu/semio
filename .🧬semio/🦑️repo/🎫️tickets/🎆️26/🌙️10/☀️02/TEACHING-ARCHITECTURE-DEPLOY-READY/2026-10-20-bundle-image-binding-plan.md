# Bundle Image Binding Repair Proposal

No Source edit or Docker action has occurred. Frozen33neutral inputs are unchanged. The existing owner is `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🎁️stack-bundle/🟦️.ts`; the existing regression consumer is `🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts`, collected by native `repo run @teaching/architecture-course:test --configuration=release -- quick`.

The current owner inspects a mutable proctor tag, reads its labels and extracts its stack through that mutable tag, then saves the tag later. An A→B→A retag during save evades before/after observations. Saving configID alone drops tag association. Docker documents selecting `repo:tag` for an archive; its v28 exporter records tag associations separately from imageID and includes both Docker and OCI views. See [Docker save](https://docs.docker.com/reference/cli/docker/image/save/), [Moby v28.5.2 exporter](https://raw.githubusercontent.com/moby/moby/v28.5.2/image/tarexport/save.go), [containerd exporter](https://raw.githubusercontent.com/moby/moby/v28.5.2/daemon/containerd/image_exporter.go).

Proposed producer changes, all within the existing owner:

1. Capture and validate immutable proctor configID immediately after existence admission. Read labels and extract stack using that ID. Capture the digest-resolved Caddy configID and create the portable Caddy alias from that ID. Dirty revision metadata remains valid.
2. Add a small domain-owned selection DTO and export port with only inspect-reference, save-required-tags and verify-archive methods. A testable export helper revalidates proctor tag, Caddy source and local alias against pinned IDs immediately before/after save and after archive verification. Save the two required tags (proctor and portable alias), retaining RepoTags; the authored Caddy digest remains an inspected source binding.
3. Private bounded scanner in the same owner: stably open the saved plain TAR, inspect512byte headers and seek over opaque layer bodies. Retain only bounded manifest/config/OCI descriptor JSON. Check checksum, canonical unique names, extents/footer, ordinary metadata entries, bounded timestamp PAX; reject path/size override or unsupported archive ambiguity. No extraction, whole-image buffering, runtime dependency, generic archive API or durable producer receipt. Existing compressed archive decoder is unsuitable because it inflates and retains the entire archive under256MiB.
4. Require every required tag exactly once in manifest.json and hash its actual config bytes to the pinned configID. Check referenced config/layer entries exist. If OCI index.json exists, verify bounded descriptor digest/size traversal and tag annotations agree with those same two configs; reject unsupported platform/media/ambiguous mappings. This closes ABA without pretending layer bodies were independently authenticated. Full layer/ELF artifact evidence remains the separately owned final verifier.
5. Keep stageCourseStackBundle return type directory string; no new receipt fields for Release to depend on. Publication remains refusal on any mismatch.

Schema-first closed20vectors are authored at `🧷️bundle/schema.json` and `cases.json`; they include stable+dirty, before/after retag, ABA, wrong config, missing/duplicate tag/config, truncation/checksum, timestamp/path PAX, matching/conflicting hybrid OCI and cancellation. After approval, put the authored fixture/schema beside the existing bundle fixtures and add an actual existing-owner test FIRST. Tests create/read synthetic TAR through existing tar-stream dev dependency, compare configSHA with existing @noble/hashes, validate closed corpus with Ajv, and exercise actual injected export ports including ABA. No new dependencies. Only then apply the narrow production helper/scanner change. No observed RED/GREEN is claimed.

## Scoped Implementation Checkpoint

Root approved the concrete plan. Permanent schema and fixture were authored first, then the actual existing-owner regression, then production changes in the existing owner. The permanent corpus has23rows: the original20 plus necessary shared-config/disjoint-tags, root-dot-slash and wrong-platform coverage; root-dot replaces redundant named dirty case, and the valid base config explicitly carries a dirty revision. Existing tar-stream constructs and independently reads actual synthetic TAR; @noble/hashes computes config IDs, and Ajv validates fixture/bindings. No runtime dependency was added. No observed test RED/GREEN; TypeScript syntax parse only returned0 for owner and regression.

Runtime limits are16GiB plainTAR,4096headers,8MiB per captured metadata entry and16MiB aggregate captured metadata. Opaque layer bodies are skipped with bounded positioned reads; body padding/header/PAX/footer and final stable file/ancestry identities are checked. Root . and ./ directories are safe only with zero body. Same config across disjoint tags is allowed; duplicate required tags or contradictory layers are refused. Optional OCI graph checks immutable descriptor bytes and Linux amd64/config/tag association, preserving full image-name versus tag-only ref-name annotations. No full layer-body authentication is claimed.

Candidate Source roster:

```json
[
  {
    "path": "🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚛️bundle/🧬️schema/🔣️.json",
    "bytes": 3628,
    "sha256": "a35c072c8f093372542eb06cdec5c8dcf10aa5adaff0bfe735ac2e983c16c0a5"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚛️bundle/🔣️.json",
    "bytes": 4657,
    "sha256": "8b020274b80c06faa449ee11b7d5f586321660c909636197c254fb9ab351f63b"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts",
    "bytes": 25938,
    "sha256": "f5c6a1d89093bb91dd18e6d8e5acdccd7ce6435325f73e654fe30f32288fb670",
    "syntax": 0
  },
  {
    "path": "🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🎁️stack-bundle/🟦️.ts",
    "bytes": 25102,
    "sha256": "820ad1d2b436c3b8945fb73abb4b8c36f706b08b2bdc14edc3d26b848cfd96ff",
    "syntax": 0
  }
]
```

Low exact-code review pending. No new command was launched, and no frozen33input was edited. Stage return remains the existing directory string, with no new durable receipt.

Current review candidate supersedes the initial checkpoint: test uses the existing tar-stream from its declared course package via a private typed test port, avoiding missing @types and any dependency edit. Scanner exact path lookup is separate from case-folded collision detection, guards file ancestors, bounds full physical path, and validates OCI media/name normalization. DockerHub namespace normalization is restricted to Caddy; OCI ref-name may be its consistent short tag, authored name or normalized image name. Thirty-minute cancellation and ten-second progress cover the bundle operation.

```json
[
  {
    "path": "🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚛️bundle/🧬️schema/🔣️.json",
    "bytes": 3628,
    "sha256": "a35c072c8f093372542eb06cdec5c8dcf10aa5adaff0bfe735ac2e983c16c0a5"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚛️bundle/🔣️.json",
    "bytes": 4657,
    "sha256": "8b020274b80c06faa449ee11b7d5f586321660c909636197c254fb9ab351f63b"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts",
    "bytes": 26460,
    "sha256": "de03f646b7fe3e50363683dbe385f8d11974e03e11a1480337412b712af795eb"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🎁️stack-bundle/🟦️.ts",
    "bytes": 25376,
    "sha256": "c24bcbe233d11777c63215d2b1d274ae6c7833553a6dc14102daf2cb6a57ea5e"
  }
]
```

## Independent Source Review Disposition

Low read the actual four files at the final checkpoint hashes and reported no material blocker. See `2026-10-20-bundle-binding-source-review.md`. The candidate is ready for actual existing-owner course quick verification. This is not executed RED/GREEN or artifact success. No additional command was issued; final layer and embedded ELF evidence remain separately owned.

The current held30path critical roster already includes every changed schema/fixture/test/owner path in this repair. No critical-paths.json or combined33input edit is required. Final actual Source hashes must still be sealed after actual verification, never copied from this pre-verification checkpoint.

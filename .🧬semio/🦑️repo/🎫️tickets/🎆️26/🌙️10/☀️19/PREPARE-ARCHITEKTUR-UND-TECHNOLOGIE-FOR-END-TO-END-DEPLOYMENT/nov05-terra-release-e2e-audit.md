# November 5 Independent Linux Preparation and E2E Layout Audit

Read-only review. No preparation, network request, installer, distribution import, daemon, browser, managed endpoint traversal or OS security interaction was performed. Reviewed the requested prior reports and retained plan, schema, cases, checker, adapter, publisher metadata, E2E runner, ticket output helper and neutral path tests. Findings describe the reviewed snapshot and need rechecking after concurrent edits.

## Linux Artifact Preparation

**Admission needs correction before launch.** The current fixed plan matches retained publisher metadata for WSL filename, URL, size and SHA-256, and Debian compressed layer size/digest and uncompressed config diff ID. Metadata is retained evidence, not a fresh publisher query. The schema admits either URL class for either artifact ID, arbitrary permitted WSL versions and independent WSL filename/hash/size. Semantic validation only binds the Debian-row URL to its SHA-256. Bind each identity to its intended URL, size, digest, filename and provenance; test swapped IDs/URLs and inconsistent WSL release fields. Hashing prevents accepting downloaded bytes that disagree with the plan but cannot authenticate a modified plan.

**Physical containment needs correction.** The adapter checks only whether final generated and preparation entries are symbolic links; its parent realpath comparison resolves the same generated path on both sides. It does not authenticate the ticket ancestor chain or revalidate directory identity before publishing. A linked ancestor or directory replacement can redirect staging, hard-link publication or cleanup. Establish and revalidate owned physical ancestry and identity, and refuse linked/replaced paths. Hard-link publication refuses occupied targets and cleanup checks partial identity; races were not executed.

Download streams enforce exact compressed size and digest, and gunzip enforces a 256 MiB maximum and pinned uncompressed hash. Current staged paths measure 211, 213 and 210 UTF-16 units on this checkout; artifactPath also checks stage paths before use. Downloads emit periodic progress, but potentially expensive decompression has no progress reporting. Cancellation reaches fetch and pipeline and is checked during hashing and before publication; actual interruption, cleanup and slow filesystem behavior were not exercised. Existing-file hashing does not pass a signal to createReadStream, so abort is observed on the next chunk rather than immediately during a stalled read. The command only downloads and decompresses artifacts; no installer, WSL import, daemon or system modification is invoked.

## E2E Compact Layout

The current runner resolves the ticket output helper, takes its generated parent, creates an exclusive child with mkdtempSync(join(root, sep)), and uses distinct compact wave mappings. The trailing separator produces a six-character child, not a sibling. The 108-unit guard and retry/repeat caps remain in place. Existing neutral tests cover three representative host prefixes, ten distinct waves, retained paths and ordinary unique directory ownership. Actual browser execution and complete dependency-backed tests were not run here.

**Physical containment is not established for a linked generated directory.** ticketOutput authenticates realpath of the ticket and ticket metadata but then recursively creates generated/nx without verifying generated is physically within that ticket. The runner derives generated from that lexical output and mkdtemp follows a linked generated ancestor. Add a refusal or authenticated physical-containment check and a neutral case with generated linked outside the ticket; consider directory identity revalidation at use. This does not invalidate ordinary-path bounds or exclusive creation.

The narrow path budget covers retained Playwright names and supported retry/repeat values, not arbitrary future attachment names or longer checkout prefixes. Longer prefixes correctly refuse through the guard.

## Adapter Re-review

The updated schema pins both complete artifact rows, the rootfs and provenance with const; unique ID validation prevents duplicate rows. Abortable existing-file reads and decompression progress were added. The reviewed checker includes pin mutation checks and a real junction refusal; reported execution was not independently repeated here.

A remaining ordering issue blocks a claim of no outside writes on refused paths: outputDirectory recursively creates generated and linux-preparation before validating physical ancestry. A linked ticket ancestor can cause directory creation outside the ticket before refusal. Authenticate existing ancestors before each mkdir and validate the created directory afterward. Revalidation immediately before publication narrows but does not eliminate replacement races between path checks and open/link/unlink; handle-relative operations are needed for a race-proof claim. No artifact preparation was executed in this review.

## Directory Creation Re-review

The revised outputDirectory authenticates the existing ticket before creating generated as one component, authenticates generated before creating preparation as one component, and rechecks generated and preparation afterwards. This resolves the identified pre-validation recursive-creation ordering issue for ordinary owned operation. Path-based check/use races remain as previously noted; no race-proof claim or adversarial replacement test is established. No further actionable blocker was identified in this bounded source review for normal artifact-only preparation. The verifier execution was reported by the owner, not independently repeated here.

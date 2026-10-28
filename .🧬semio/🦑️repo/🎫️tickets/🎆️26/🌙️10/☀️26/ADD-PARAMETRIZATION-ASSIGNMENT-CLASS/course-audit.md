# Independent Course Integration Audit

## Findings

- P2 — 🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📦️assignments/🟦️.ts:11 deduplicates producer descriptors by document.id, while lines 22–26 load only the retained descriptor filename. The normative AssignmentDocuments contract and both validators allow en/de entries with the same producer id and different filenames. Such valid metadata silently drops one language, and the child transport subsequently fails its required second-language fetch. Retain unique producer ids while collecting every referenced filename for each retained artifact, or explicitly reject this metadata combination in both validators and the contract conformance fixture. Similarly reject different producers claiming the same public filename, which currently silently overwrite the Map entry at line 26. Current authored en/de metadata uses distinct ids and filenames, so this does not block the present assignment.

## Verified

- Localized Assignment title, description and documents require both en and de; UI indexes the chosen locale explicitly and contains no document fallback. Manifest declares parametrization-en and parametrization-de. Topic ownership is checked by TypeScript and Rust validators; assignment ids are unique within assignments.
- Assignment metadata is included in both Course and CourseView schema/types and forwarded by TypeScript and Rust views. React home exposes downloads on the course root and selected topic, and filters them by topic.
- Normative schema WireVersion, TypeScript constant, Rust constant and fixture are v10. Executing the owned pure canonical/fingerprint functions through repo tools exec --light node produced 678b91fb9e03e81a, matching the fixture. Initial direct JavaScript evaluation rejected TypeScript annotations; rerun used Node built-in stripTypeScriptTypes for the pure adapter functions only.
- Parent transport retains producer artifacts and rechecks before and after the consumer lifecycle. Build and dev both enter withCourseAssignmentPdfsV1. Missing producer PDFs reject stat/read; missing channel rejects startup/build; child transport rejects unsuccessful HTTP responses, absent bodies, oversized bodies and non-PDF signatures. No fallback blank PDF was found.
- Child transfers are authenticated, each request has a 30-second timeout, streamed size is capped at 16 MiB, and readers are cancelled in finally. Parent server listens on loopback, authenticates requests, closes all connections in finally, and releases retained artifacts after the consumer exits. The child request has no caller-provided AbortSignal, so within a running child cancellation relies on the enclosing owned Vite operation/process lifetime; maximum request delay is bounded.
- Concurrent quiz changes were left untouched. No implementation changed in this audit.

## Limits

Owned full repo test/build targets remain blocked by the reported .cache/b/l/fixed.dir infrastructure issue; no compiler invocation or infrastructure workaround was attempted. This audit inspected source and executed only the pure fingerprint adapter. PDF visual/content QA belongs to the separate print audit.

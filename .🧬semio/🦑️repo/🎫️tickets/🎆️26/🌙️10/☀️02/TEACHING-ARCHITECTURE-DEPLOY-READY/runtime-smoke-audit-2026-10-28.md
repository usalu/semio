# Native Proctor Smoke Source Audit

Read-only review on 2026-10-28; no request, service launch, or build performed. Build 81996 remains separately owned; this review proves no runtime readiness.

The prepared proctor-native-smoke.mjs lines 16–21 use the actual wire-v10 course.course envelope: anonymous principal, source course id as scope, authority consistency, null cursor, UTF8 JSON arguments as byte array, and snapshot bytes decoded as JSON. This agrees with teaching proctor end-to-end harness (🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🧰️harness/🦀️.rs:159–170), course React connector (🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts:332–342), and proctor query admission (🎓️teaching/🛂️proctor/🔨️modules/❓️queries/🦀️.rs:76–88). Wire version is 10 in the current course schema.

Course title/id comparison and solution-free task checks agree with CourseView (🧰️framework/🛍️products/📚️course/🧬️schema/👁️read-views/🦀️.rs:45–59) and its constructor (course/🔨️modules/👁️views/🦀️.rs:40–54), which preserves course title/id and maps loaded quizzes to task metadata without answers. LoadedCourse constructs that view from listed source quizzes (🎓️teaching/🛂️proctor/🔨️modules/📚️course/🦀️.rs:94–127).

Assertion limits sent to backend before execution:

- Line 12 checks a JSON substring, so it cannot establish the exact instance id. Canonical INSTANCE_ID is teaching-proctor (instance/🦀️.rs:85–86); assert the identity property itself for an identity receipt.
- Line 22 establishes source-count equality, not an explicit ten-quiz assertion or equality of the returned quiz identities/order. It supports ten only when the passed actual course input is independently verified to contain ten.
- GET /instance CORS is checked for http://localhost:6061. The POST response CORS and OPTIONS preflight are not checked. Node fetch does not enforce browser CORS. Thus this is a native local API smoke, not a browser preflight/public-origin readiness receipt. Actual gate grants response CORS and handles preflight at instance/🦀️.rs:635–677.

No incompatible envelope or snapshot-shape mismatch found. UTF8 decoding uses Node replacement decoding, so it does not independently certify strict malformed-UTF8 rejection; current JSON value comparison remains a smoke assertion. The fixed localhost origin presumes the isolated development-origin configuration and must not be presented as a production-origin probe.

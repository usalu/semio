# Shared Policy Explicit Null Regression Review

Read-only review of the parent repair on 2026-09-30. No builds/tests were rerun and no production files or ticket status changed. Executed red/green evidence is recorded by the implementation owner in `🛠️2026-09-30-shared-policy-publication.md`, including the exact null-case native failure and subsequent 13 native laws / 74 TypeScript tests.

## Result

No remaining concrete optional-policy-field parity defect was found in the inspected repair.

- Rust `📇️directory/🛡️access-policy/🧬️schema/🦀️.rs:85–91` combines `default` with a custom deserializer for `space_kinds`. Missing fields produce `None`; present values deserialize through `Vec<String>` and wrap in `Some`, so explicit null no longer becomes omission. Serialization skips `None` and emits a real array for `Some`.
- JSON schema `🧬️schema/🔣️.json:54–58,82–93` omits `spaceKinds` from required fields but requires any present value to be a nonempty unique array of declared kinds. Null is invalid.
- TypeScript `🧬️schema/🟦️.ts:29–33` tests own-property presence rather than truthiness. Missing `spaceKinds` remains omitted; any present null/undefined/scalar reaches `choices` and is rejected. Valid present arrays are copied and retain the field.
- Rust post-parse validation at lines 111–121 and TS `choices` at lines 20–23 agree on empty/duplicate/unknown entries. Required effect/roles/actions/schema/grants remain required and non-null. There are no additional optional fields in the production policy or grant wire models.
- The neutral corpus contains valid `minimal-allow` (omitted field) immediately before invalid `null-space-kinds` at fixture line 1568. The TypeScript/Ajv oracle loops all policy cases; the native oracle also loops all policy cases using the same fixture. Both therefore consume the new refusal and retained omission case.
- Evaluators agree that omitted kind restrictions apply to instance-wide requests, while present restrictions require a supplied matching kind. Rust `None` and TypeScript `undefined` are their respective typed request omission values. Decision-vector `spaceKind` is JSON-only schema metadata rather than an additional production policy Option field; no adjacent wire-model mismatch was observed.

This conclusion is source-review evidence supported by the parent's recorded executions, not an independently rerun passing claim. Graph-closure re-review is separate and will wait for the implementation owner's settled-source confirmation.

# Required Bridge Argument Synthesis

## Change

The generic framework `artifact_app_laws::assert_declared_actions_bridge_to_commands` staged defaults and required Boolean values only. A declared command with required text inputs consequently received an empty object. The concrete generation3d parser correctly refused its missing widget input; that parser and the concrete law remain unchanged.

Private testing-only `declared_bridge_arguments` now reuses the existing `declared_argument_alternatives` candidates, adds the single-option string case, and selects a candidate only after the existing `OwnedJsonSchemaValidator` admits it against the actual `ActionArgDef.json_schema`. It preserves declared defaults and leaves optional inputs without defaults absent. A required argument without a supported valid candidate produces an explicit error naming the argument and schema. It does not skip the action or erase its command-id assertion. The public law signature and all callers are unchanged; no production API is widened.

This is representative synthesis for a command bridge law, not a general satisfying-assignment generator or host entity resolver. Required Any/record inputs without defaults have no supported generic representative and fail clearly. Declared defaults retain the existing staging semantics. The separate declaration/manifest validators own validity of authored defaults.

## Schema and Oracle Coverage

The neutral authored fixture carries existing manifest argument schemas, exact expected payloads, independent JSON Schema admission contracts, and invalid payloads. Its17 vectors cover required plain texts, Boolean, one-option strings, declared choices/defaults, bounded numbers, exclusive bounds, optional omission, JSON text, nullable scalar, nullable null default, required Any null default, required record default, optional default, unsupported required Any/record, incompatible pattern, and unavailable bounded representative.

The native test evaluates the real private helper and compares exact JSON values before validating supported outputs and rejecting the invalid payloads through the framework validator. AJV independently validates the same13 admitted payloads and25 invalid payloads (38 assertions). The4 unsupported synthesis cases are native fail-closed policy checks; they are not falsely described as AJV generator verdicts. f64 expected values are authored as2.0/3.0 to retain exact typed serialization equality.

The new native law and AJV oracle join the existing plugin `canonical-architecture` target. Its original11 native laws, extension-retirement13-case oracle, and source-freshness17-case oracle remain present. No parallel command, target, or launcher infrastructure is added.

## Verification

- Existing concrete app law101 RED: required widget/channel/value absent and parser reported “Choose a widget”; parent/owner receipt is `🗑️generated/app-harness-final/exact-cargo-laws-VsEaKp/00/law-100.stdout`.
- Native TDD RED: `🗑️generated/declared-bridge-red/exact-cargo-laws-puXmRY/00` executed the old Boolean-only staging and failed `required-texts`: actual{}, expected three required strings. Its native process failed exactly once; no assertion was removed.
- Initial post-fix run `🗑️generated/declared-bridge-green/exact-cargo-laws-dymKIv/00` exposed an authored expected integer2 vs actual typed float2.0. Hand-authored expected encodings were corrected; mathematical bounds and exact equality remain intact.
- Initial portable oracle-only Nx run passed23 assertions before the five additional default/nullability vectors. Final uncached `bun nx run @semio-tech/framework-plugin:canonical-architecture --skip-nx-cache --output-style=static` passed in43.2s:38 new AJV assertions, existing extension-retirement13 and source-freshness17 oracle cases, and all12 exact native laws (new17-vector regression plus preserved11 established laws). Receipt group is `🗑️generated/declared-bridge-final/exact-cargo-laws-hNUuHD/00`; the new law passed exactly once with no ignored tests in its selected execution. This is an actual source compile/list/hashed-executable execution, not an oracle-only or cached claim.
- Parent reports the concrete bridge law101 now passes. Its later mesh-codec harness failure belongs to the concrete composition owner and does not justify changing production parser defaults or skipping this law.
- Scoped `git diff --check` passed on owned updated source files.

## Exact File Manifest

Created:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🎮️declared-bridge-arguments.json`
- This ticket Markdown report.

Updated:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — private test helper and existing bridge-law staging only.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🦀️.rs` — real helper neutral fixture regression.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts` — AJV admission oracle.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts` — contributes oracle and native law to established target.

Deleted: none. Authored implementation increment:5 files (2Rust,1TypeScript test,1owned script,1JSON fixture),1 additional native law,0 new executable targets. Concrete app harness, its SHA fixture, manifests, Cargo dependencies, root scripts/projects/launch registry, and production parser are untouched by this increment. Temporary output remains ticket-only for parent cleanup.

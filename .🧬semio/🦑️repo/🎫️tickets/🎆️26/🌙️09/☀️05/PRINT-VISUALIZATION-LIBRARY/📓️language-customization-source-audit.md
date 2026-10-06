# Language and Customization Source Audit

Bounded read-only audit on 2026-10-06. No source edits, tests, publishers, or generated cleanup. Existing parent-owned gates were not repeated.

## Bilingual Family Census

A PowerShell JSON-only census of the authored `print/🧬️schema/🔣️.json` and generated `print/🖼️assets/🔣️viz-api.json` found **246 families and 4989 option slots in each**, with **zero missing or whitespace-only English/German descriptions**. Generated totals additionally declare 77 packages, 169 commands, and 4296 package keys. This census validates presence, not translation quality or runtime rendering.

## Native Control Ownership

The current source verifier binds options to actual native owners:

- `visualization-gallery/🟦️.ts:837–849` derives family controls from the family scope, source-reachable scopes, and the registry variant.
- `:854–879` follows forwarding paths and installed helper vocabularies before allowing inherited documentation.
- `:1160–1192` refuses invalid scope bindings, unreachable shared keys, missing family rows, mismatched type/default, and blank meanings in either language.
- `:1197–1199` projects every generated family option into that verifier.
- `:406–424` requires complete bilingual rows and rejects ambiguous native-owner contracts; `:1203–1214` checks emitted package controls against actual l3keys owners.
- `print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts:715` calls the family contract verifier in the established pipeline.

No concrete missing customization control was substantiated by this bounded inspection. The broad catalogue coverage function alone computes undocumented/phantom options but does not assert those arrays (`visualization-gallery/🟦️.ts:1255–1270,1295–1315`); stronger family/package checks exist in the pipeline. This is a distinction between gates, not evidence of a missing control.

## Concrete Language-Selection Finding

The authored API reference explicitly selects German (`🧾️template/📊️viz-api/🔓️viz-api.tex:1`), as does the gallery generator (`visualization-gallery/🟦️.ts:140`). The API's visible text dispatcher explicitly handles English/German (`🔓️viz-api.tex:5–7`). Those selections are consistent with an author choosing a language for a document.

However, the general document author API still has a default language: `🖋️latex/semio.cls:16` declares `\\DeclareStringOption[de]{language}`, `semio-core.sty:35` initializes German, and `semio-core.sty:325–331` silently selects English for an unrecognized language. Consequently an omitted language produces German, and an unsupported selection falls through to English. This is a concrete source-level mismatch if the repository's “multiple languages with no default language” requirement applies to the print document author API. It is not a missing EN/DE description, and this audit did not establish a missing interactive UI language selector. The existing explicitly selected language publications cannot validate refusal of omitted or unsupported language.

## Limits

No accessibility defect was established from this bounded source audit. Tagged-PDF or assistive-technology behavior was not inspected, so no accessibility runtime claim is made. Native rendering and localization gate results remain owned by the parent.

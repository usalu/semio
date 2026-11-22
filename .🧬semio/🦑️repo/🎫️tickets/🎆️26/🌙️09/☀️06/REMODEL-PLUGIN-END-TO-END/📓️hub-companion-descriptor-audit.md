# Hub Companion Descriptor Audit

The owner-root JSON and canonical pack were decoded with the repository pack decoder and compared recursively on 2026-11-21. JSON is listed first, pack second.

- `🌎️hub/🧩️compositions/🗄️stdio/🧩️component`: the sole difference is `executionProtocol.appChannelVersion`, 25 versus 24.
- `🌎️hub/🧩️compositions/✒️writer/🧩️component`: the protocol version is likewise 25 versus 24. `manifest.apps[0].io.ports[0].label` is the localized native/reuse object with English and German `Text` versus the string `Text`. Both import and export `contributions.ioEntries` contain ten rows total: JSON uses `framework.io.text` / `utf-8` / `*` in place of the pack's `s.stdio.txt` / `utf-8` / `*`; the remaining docx, json, md, and pdf entries are the same but shifted in order.

These observations identify disagreement between forms; they do not establish which form describes a currently built component. The verifier requires canonical pack bytes, equal normalized semantic forms and the current app-channel version. The JSON cannot safely be used as evidence to rewrite the pack without a component emission.

The static native catalog rendering has independently succeeded for 76 entries as reported by the parent. `auditPluginCatalogSources` with default compiled admission checks the owner-root pair and reports `descriptor-invalid` on mismatch; `catalog-complete` instead uses source admission for its initial identity preflight and then verifies every fresh raw component, descriptor and receipt. The stdio catalog-root publication additionally binds the shipped component and receipt to the committed pack, checks actor WIT, runs the source audit, and verifies fresh staged bytes. Thus static rendering alone does not certify trusted built-component publication.

The visible `.🧬semio/🦑️repo/⚡️cache` search found no files named `🛂️.descriptor.semio` and no component or receipt paths identifiable as stdio or writer. The only matching WASM files in that cache were dependency fixtures and tools. This is a bounded filesystem observation, not proof that no artifacts exist in external or opaque caches. No builds or source changes were made.

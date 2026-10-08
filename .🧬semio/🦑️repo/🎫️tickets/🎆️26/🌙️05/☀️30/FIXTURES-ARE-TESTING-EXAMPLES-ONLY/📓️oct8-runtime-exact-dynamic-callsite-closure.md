# Exact Dynamic Import Callsite Ownership

The prior Runtime graph applies a dynamicImports caller-file roster to every computed import in that file. A caller containing two computed imports can therefore erase the second unresolved edge using only the first import’s declared module bytes. The schema-first correction requires exact active parsed caller SHA256 and a zero-based computed-call lexical ordinal. Other computed calls stay unresolved; caller/source drift cannot reuse the roster. No legacy file-wide fallback remains.

Plain neutral test examples include selective first-call binding with a second unresolved import, out-of-range binding, changed caller bytes and missing binding metadata. Existing actual fixture-owned module and stale module byte refusals remain. Independent TypeScript parses the two call arguments; Bun CryptoHasher checks the authored active source digest and Ajv validates genuine per-owner contracts. There is no schema for the example corpus. RED/GREEN receipts follow actual execution.

## Actual TDD Receipt

Registered private Nx route ticket-fixture-verification:runtime-callsite-source forwards only to the existing Runtime test owner through the existing budgeted process interface. Its command is registered in both live and canonical seed launch files, alongside the ticket’s gate group. No new script file was added. Full preimages are retained separately for the four source/test/schema/example endpoints and the four private/live/seed registration endpoints.

RED tool receipt c81a43 exited Nx1:6passed/5failed/69assertions,73filtered; actual Nx duration354ms. The five failures reproduced selective binding, outside callsite, changed caller, missing binding and the independent selective-call assertion. Implementation was unchanged when RED ran.

GREEN tool receipt8d43da exited Nx0:11passed/0failed/84assertions,73filtered; actual Nx duration348ms and Bun286ms. The independent TypeScript parser, Ajv owner leaf, Bun SHA oracle, existing static import grammar and independent esbuild production-test exclusion all passed. Console emitted the exact active caller SHA/callsite DEBUG completion. Raw outputs are ticket-generated oct8-root-runtime-callsite-red.log and oct8-root-runtime-callsite-green.log.

The owned lexical tokenizer does not expose source byte/UTF-16 offsets, so the final identity is the documented zero-based lexical computed-call ordinal in the exact parsed source. The source hash is computed after the actual production exclusion transform when enabled; declarations from another transformation context cannot reuse it. Unclaimed computed calls remain unresolved and fixture-owned target paths still fail the existing collection boundary. This proves the reproduced supported import/require grammar; it makes no arbitrary-eval or indirect-loader dataflow claim.

Independent read-only review: 📓️oct8-exact-dynamic-callsite-independent-review.md. Actual Plugin verified-Blob caller provenance and native mounted WGPU module owners remain separate publication work; no full runtime or publication pass is inferred.

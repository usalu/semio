# Typing Net Contract Review

Static current-Source review, 2026-10-29. No edits, tests, native/compiler/lease/process actions were performed. No green runtime or compiler result is claimed.

## Contract Alignment

Current tool-machine schema TypingOpen now requires buffer/tool/deadline/transaction/**net**, with additionalProperties:false. net is an ordered mutation array and the description states transaction keys are canonical decimal positions. The old entries property is absent, so this is one new canonical contract without aliases or migration support.

Parsed current typing-law fixture contains **29 open-state objects**, identified by buffer/tool/deadline/transaction. Every one contains net and none contains entries. This count independently matches the reported29 converted expectations. Existing transaction step fixtures remain separate keyed-entry contracts.

TS tool-machine/🟦️.ts:819 TypingState now owns readonly net:M[]. Resume at838 uses net.length and at841 reconstructs the generic ToolTransaction with String(index) keys. Persist at895 extracts transaction.mutations(); provisional at918 returns net in sorted window order. This preserves ordered mutation semantics while keeping ToolTransaction's generic keyed representation intact.

Current Rust canonical typing Source has now also changed: TypingState at235 uses net:Vec<M>; resume at255/257 derives count and enumerated decimal keys; persist at325 extracts mutations. Its remaining transaction.entries() uses at305/325 access the generic ToolTransaction and are legitimate, not stale TypingState fields. No TypingState.entries use was found in that reviewed canonical owner.

Conformance typingOpenJson at706 emits net; hostile TypingOpen case794 uses net while remaining invalid because deadline is absent. Rust typing-law open-state projection140 emits state.net. The generic transaction/gesture/scrub entries contracts and shared entries_json helper must remain unchanged.

## Remaining Consumer Boundary

Earlier reference census found TypingState consumers in tool-machine typing/edit, retained transfer/read tests and OS typed-operation staging/follow modules. New edit already reads net. Canonical Rust resume/persist now agree. Changes are concurrent: this review does not certify that every retained resident Vec conversion or OS consumer compiles. The earlier BTreeMap→resident Vec mounting/slot-transfer integration remains the owning Rust implementation's acceptance, separate from this schema/TS/fixture review.

The converted29 expectation objects demonstrate fixture shape consistency but do not independently prove key positional correctness. That semantic claim rests on existing typing host upsert/retract code using numeric positions, owner's pre-conversion assertion, and the real Rust/TS/XState conformance execution still required. Preserve resume/shrink-to-zero/split/commit/abort checks and exact generic transaction outputs.

## Metadata and Regeneration

Repo schema catalog scope framework.tool-machine currently points to tool-machine/🧬️schema and exports TypingOpen by name/file/facet. It is an export/path catalog, **not an embedded copy of the TypingOpen property schema**. A property-only entries→net change may therefore legitimately produce no schema-catalog byte change. Do not hand-edit the catalog or add aliases merely to force a diff.

Current authoritative root targets are:
- `SEMIO_LANGUAGE=en repo run workspace:schema-generate` → root schema generate;
- `SEMIO_LANGUAGE=en repo run workspace:schema-verify`;
- `SEMIO_LANGUAGE=en repo run workspace:schema-test`;
- `SEMIO_LANGUAGE=en repo run workspace:generate-config`, followed by workspace:generate-config-check only if changed contribution/project declarations require config refresh.

Use normal funded canonical owner routes; none was executed here. Schema catalog declares its own generator as `repo ./📜️script.ts schema generate`; root target preserves the authored graph. Existing actual Source schema and fixture paths remain canonical, so this property rename does not itself justify a new generator target, a metadata compatibility layer or blanket runtime inference edge.

## Review Result

Reviewed JSON schema, all29 persisted open-state fixture shapes, TS state/resume/persist/provisional, conformance projection and current Rust typing projection agree on net. No stale TypingState.entries field was found in the inspected canonical implementations. Runtime conformance, semantic generation and the live Rust owner's terminal gate remain unproved by this audit.

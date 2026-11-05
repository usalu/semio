# Wire Admission Audit

Read-only source audit; no build or native/browser execution was started.

## Concrete Blocker

The procedural provider advertises a blueprint text edit bound larger than the Forms command accepts. In playbook procedural preview-and-params-body/🦀️.rs:241-255, text_parameter_scene emits editMaximumBytes 65,536, editEncoding json-chunks, editArgument valueJsonChunks and action patchQuestions. TextEditor value-source/🟦️.ts:34 splits JSON.stringify(args.text) under that advertised maximum. Forms patch-questions/🦀️.rs:52-58 accepts only 32,770 aggregate UTF-8 bytes and 65 chunks; from_action:85-89 rejects above 65 chunks before reducer dispatch. Thus a 40,000-byte plain ASCII parameter text is accepted by the editor transport but rejected by PatchQuestions admission (79 chunks). Align the provider-advertised bound with the command contract, or deliberately widen the entire canonical PatchQuestions contract.

## Connections Verified

- SetTryValue declares value_json_chunks and initial_value_json_chunks as Option<Vec<String>> in its actual FromValue/DslRecord-derived payload (set-try-value/🦀️.rs:100-116). Canonical PatchQuestions declares Vec<String> value_json_chunks and normalizes both scalar controls and paged editor ingress before dispatch.
- Forms editor play-app/🦀️.rs:106 registers FormsPagedValueCommandJobFactory; :119 uses forms_tool_raw_bytes(tool_id), avoiding the smaller standard bound for the two paged actions. retained-commands/🦀️.rs:300-320 assigns their exact factory keys and WindowTransient/Artifact publication lanes, with paged factory proofs at :371-374.
- Viewer command_from_action routes setContributions through the actual typed payload (:86-90); registers its factory (:93-95), delegates construction (:97-99), exposes proofs (:101-103), and supplies the Config preparation factory (:105-106). Its manifest declares the command and migrated classification (:130-134). Viewer retained factory uses Config publication, the forms.form document schema, s.forms.forms@1/*#viewer proof controller and exactly setContributions.
- Browser host contribution installation has no Editor-role gate. ShellHost contributions-refresh/🟦️.ts:139 uses the active session app directly. At :142 it checks appOwnsCommand only. pluginShouldReceiveContributions in route-and-chrome/🟦️.tsx:531-532 checks host mode/plugin identity only. The native session-and-topology contribution staging likewise checks app_owns_command.
- Generic public wire envelope validation in plugin reactor wire-limits/🦀️.rs bounds whole body, string bytes and depth; it does not impose an array count limit on the new chunk arrays. setContributions explicitly skips per-string cap while retaining body cap. Addressed tool wire limits resolve the live owner/controller/exact tool public contract.
- Provider embedded_payload reads paramsJsonChunks and joins with 65,536-byte bound (preview-and-params-body/🦀️.rs:202-217). Its helper reads up to 257 entries; ordinary producer chunks fit the UiValue shared chunk bound.
- TextEditor value-source schema explicitly accepts integer editMaximumBytes 1..65,536.

No additional concrete missing ingress connection was established in this focused audit. Reflection/schema-derived argument metadata was inspected at declarations; no runtime pass is claimed.

## Fix Verification

The concrete blueprint budget mismatch is fixed in current source. Procedural root declares a single BLUEPRINT_VALUE_MAXIMUM_BYTES = 32_770; both embedded-flow graph source settings and text_parameter_scene use it. ANSWER_VALUE_MAXIMUM_BYTES remains 16_384. Provider schema QuestionControlTransport declares the same encoded budgets. The generic TextEditor schema retains its independent 65,536 ceiling.

Current PatchQuestions regression imports the real textEditorValueSourceActionV1 mapper and provider schema budget. The shared encodingBoundaries fixture drives mapper → normalizePatchQuestions → canonical JSON schema validation and reconstruction at 32,770 bytes, while 32,771 rejects at the mapper and direct scalar normalization. The largeSources loop drives the same mapper/canonical-normalizer path, including the escaped source fixture of 16,375 raw bytes / 24,516 JSON-encoded bytes; byte counts are asserted in the test and independently recomputed during this audit.

No additional concrete inbound argument or catalogue gap was established. Factory/publication/proof and active Viewer contribution receiver connections described above remain connected. This verification is source/schema/test inspection only; native88782 was reported live pending by the coordinator, and this audit claims no native/browser pass.

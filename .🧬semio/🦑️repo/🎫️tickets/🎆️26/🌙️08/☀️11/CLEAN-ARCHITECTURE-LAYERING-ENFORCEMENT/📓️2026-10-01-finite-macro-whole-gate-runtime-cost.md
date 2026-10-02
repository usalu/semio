# Finite Macro Whole-Gate Runtime Cost

Read-only source/density investigation, 2026-10-01. Root reports current complete gate at roughly seven minutes without first250-file progress versus prior whole-source run about1m57. This audit ran no gate/scanner/test/compiler and does not attribute measured wall time to a single function. No production edits. A broad filesystem survey was stopped; only bounded source reads below are retained evidence.

## Phase ambiguity and concrete cost

Gate `📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:95–101` computes compileReferences for **all** inventoried sources, then constructs the whole module graph before entering the result loop. The first250-file progress at118 is downstream of both phases. No progress yet cannot distinguish source parsing, macro binding or graph expansion; add phase counters/timing around the existing bounded work rather than increasing timeouts or bypassing sources.

Current scanner discovery6463 onward tokenizes/pairs each source. For every macro_rules (including no include sites), it computes enclosing scope via `[...pairs].filter(...).sort(...)` at6634; performs duplicate-name `templates.filter` per template; scans source tokens0→scopeEnd per supported template at6640; and for every candidate checks templates.some plus `[...pairs].some` ancestor macro trees. Complexity includes O(M·P logP + M·N + candidates·P + M²) per source, plus repeated spreads/slices/allocations; N=tokens, P=delimiter pairs, M=all templates. Pair sorting can dominate even when no macro owns a compile input.

## Bounded owner density receipt

Direct reads and a lightweight comment/string-skipping delimiter census (not the production Rust scanner) found:

| Owner source | Characters / lines | Actual macro_rules definitions | Templates with compile include sites |
|---|---:|---:|---:|
| OS `🔌️plugin/🦀️.rs` |2,896,103 /46,523|17|0|
| OS `🏪️store/🦀️.rs` |1,473,065 /27,000|2|0|
| Neutral `🌱️value/🦀️.rs` |18,243 /424|1|0|

OS plugin macro names/lines: __semio_export_component_guest75, component_persistent_local128, app_labels6917, app_action_enum12972, app_commands13071, view_commands13303, bounded_first_step_tool_proofs15432, framework_reserved_job18075, surface_builder_forward38669, __semio_actor_exports44363, __semio_owned_core_exports44549, plugin_exports44759, __semio_plugin_actor_exports44766, extension_exports45131, register_plugin46288, derive_artifact_facets46304, subset46410. Store definitions11747 and24597; Value368. Raw regex counted18 plugin matches because one documentation mention at46395 is not a definition. This illustrates why token-owned counts matter.

OS host source is451,744 chars/8,075lines with zero macro_rules candidates; oracle aggregator36,540chars/999lines has zero. These are a few owner samples, not a complete repository census. Plugin/Store have ordinary literal inputs elsewhere, so skipping their **whole source** would be incorrect; only unnecessary zero-input template proof is removable.

## Exact bounded optimization

1. Keep a single lightweight skeleton inventory for **all** macro definitions: name/ranges/attributes and parent lexical scope. These ranges are required to detect target invocations hidden in earlier/later helper transcribers and duplicate/shadow names. Do not remove zero-input templates from that evidence index.
2. Mark active proof templates only when their transcriber owns an actual include/include_str/include_bytes site requiring static or dynamic template provenance. Run matcher/fragment/binding/closure proof for those active templates only. Zero-input macro skeletons can still poison an active template via generated/helper invocation and unknown code-emission context. A template generating path attributes/other unsupported input forms must retain their normal unsupported scanner outcome; it is not silently authorized by this optimization.
3. Build enclosing delimiter/macro-tree scope in one stack pass over token positions. Record nearest brace/ordinary container and ancestor macro-token context; no per-template pair spread/filter/sort, no per-candidate pairs.some. Use exactly the same token-pair facts and preserve unsupported unmatched/ambiguous cases.
4. Index invocation candidates by macro name in one token pass, with offset and owning template/outer macro-token ancestry. For an active template, select candidates by lexical bounds; retain the current earlier-helper/qualified/opaque candidate poisoning rules. Do not restrict to invocations after definition and thereby resurrect missed earlier helper transcribers. Count duplicate names through a map covering **all** skeletons, not just active templates.
5. Precompute outgoing external mod/include emission positions and their parsed scope owners once. Query visibility intervals for active templates instead of searching every token repeatedly. Preserve all configurations, nested inline descendant emissions and helper-created code uncertainty. Graph-backed incoming origin seal remains separate; lexical indexing cannot substitute for actual Cargo/mount provenance.
6. Cache only immutable per-source syntactic evidence within the current gate snapshot. A cross-run content cache must key exact source bytes and parser contract version; filesystem/provider/mount authority must be validated fresh. No require/global graph stale authority is introduced.

Expected scanner work becomes a few linear token passes plus actual active invocation/fragment parsing and expanded input output size, rather than N/P repeated for every irrelevant macro. PDF88 and Home15 occurrence counts must stay exactly retained; static unused template facts and hostile earlier/helper/opaque/export cases remain visible.

## Proof before adopting

Extend existing language-neutral/native law with many zero-input skeleton templates surrounding one active data template, an earlier zero-input helper transcriber that creates a target call, duplicate active/inactive names, and nested opaque ancestor calls. Assert identical typed facts/refusals to the small forms; no timing threshold is a semantic test. A bounded large-source performance measurement can record phase durations once appropriate, but must not replace complete gate execution or loosen its policy. Preserve cancellation/progress by meaningful phase reporting and existing budgets, not longer deadlines.

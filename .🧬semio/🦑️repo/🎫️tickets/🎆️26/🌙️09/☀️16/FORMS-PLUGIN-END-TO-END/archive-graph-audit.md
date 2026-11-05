# Archive Owner Graph Audit

Read-only inspection found no malformed entry metadata in framework-renderer-react:document-archive-ownership-check. The target has one bounded literal runtimeSourceEntries path, the actual archive-owner test module. Its router invokes that exact Bun test, and the production helper’s wire-contract import is type-only. There is no glob or dynamic global test enumeration introduced by this target.

The progress count comes from settleSuspectsV1 in graph/generations/cutoff, which iterates the current generation’s suspect pseudo-owner facts and re-derives them under bounded concurrency. It counts suspect closure facts, rather than test cases or runtimeSourceEntries. Shared inference implementation changes (including the canonical generated-output resolver just repaired) can make many existing closure facts suspect. The current 1276-closure verification is consistent with that mechanism; this audit found no concrete new-target metadata bug that explains a forced global compiler scan.

The log remained alive at 93/1276 while inspected. No source edits, target restart, scope weakening, or guard bypass was applied. Exact performance cause inside each derive operation remains unmeasured, and this structural audit does not claim that the observed ~3 seconds per closure is optimal.

Root follow-up inspected settleSuspectsV1 and openGenerationDeriverV1: the suspect count is policy bounded, derivation uses bounded declared concurrency, and one shared script-input cache is created for the entire deriver. Each question retains its own observation snapshot and real support rows. No safe missing-cache correction was established from this source inspection, so this task did not alter cutoff policy, receipt validation, source observations or concurrency. The current archive and descriptor runs remain authoritative live owners.

Root cooperatively canceled only the secondary archive owner87423 (terminal130 confirmed) to free its approximately4.3GB graph verification footprint for the core Forms/native/browser gates. This was a priority decision after its focused11test/31assertion suite and strict TypeScript validation passed, not a timeout restart or a cache/control bypass. The registered owner target remains available; its complete owner result is not claimed. Descriptor19405 and native88782 remain retained.

## Descriptor Two-Pass Observation

The retained descriptor owner completed 1397/1397 closure checks, swept leases, rechecked all 387372 physical graph facts, then started another 1397-closure verification pass. No emitter compiler or refusal was logged at that boundary.

The canonical generation store has two possible plan calls: refresh first evaluates a shared-reader verdict, then build reuses its hint only when the pointer generation still matches and the hint is at most 1000 ms old. Otherwise build calls plan again under builder custody. The loader retains the same answers map by store root across attempts; settleSuspects reuses an answer only after its recorded support rows still hold. This source permits a repeated full progress count without a process restart. The log does not disclose pointer/age/reuse statistics, so it does not prove which condition prevented hint reuse, whether each row used a memo answer, or an infinite loop. No age limit, guard, cache, or source ownership was changed.

## Correction To Initial Metadata Conclusion

The initial structural audit missed the canonical RuntimeSourceEntries prefix requirement. Exact schema validation later proved the new archive target raw workspace-relative literal invalid; only that literal now has the required {workspaceRoot}/ prefix. Canonical validator and independent Ajv red/green evidence and attribution are in nested-admission-audit.md. The graph phase/cutoff analysis above remains read-only evidence.

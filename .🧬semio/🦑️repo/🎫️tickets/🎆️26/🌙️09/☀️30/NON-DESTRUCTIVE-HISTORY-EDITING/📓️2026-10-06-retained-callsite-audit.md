# Retained Callsite Audit

Source trace on2026-10-06; this is not runtime proof.

ArtifactStore::construct rejects empty opened authority, validates/folds durable history, and shares the cached genesis snapshot when no edits are applied. Its empty-history construction no longer clones the initial domain value. The retained document/config admission routes separately prepare current state under their caller's StepContext; Core is replacing their remaining direct HistoryLog::fold calls with the bounded semantic cursor and cooperative transition decoder.

The public materialize_document_snapshot helper remains a full fold: it synchronously obtains effective supersessions, clones genesis.snapshot(), and folds the requested edits. Production callers found by repository rg are the native host's materialize_backbone_snapshot and the store's pack_at_checkpoint. Native host uses its helper for initial workflow materialization; that source trace is distinct from an observed history-edit freeze. A complete claim about bounded cold/network/export work would need separate evidence for these callers.

Core owns checking the editing/accept/finalize call graph against this helper while finishing bounded retained fold/decoder and exact actorless cleanup. The source trace establishes the distinction and the remaining limit, rather than declaring either renderer journey complete.

Relevant files: framework OS store/🦀️.rs around materialize_document_snapshot and ArtifactStore::construct; framework native host/🦀️.rs around with_backbone_envelope and materialize_backbone_snapshot; retained document history hydration and retained config initialization.

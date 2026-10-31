# Cancellation Alias Source

`CancelToken::admit_alias(&self, grant)` now returns `RetainedWorkOutcome<Option<(Self, RetainedCloneProgress)>>`. Items/depth denial returns None/work0 before touching the Arc; the defining actual Arc clone returns work1 and items1, physical copy/capacity/release0. The original token remains borrowed and unchanged. Existing retirement source was not edited. The schema and genuine owning law were authored first; no runtime or compilation was performed.

The independent owning gate is the Async library test target. Existing Async historical tests still contain obsolete retirement outcome adapters, so this packet is not a whole-crate SourceReady claim. Exact source and test hashes are in drawing-cancel-alias-working-files.json.

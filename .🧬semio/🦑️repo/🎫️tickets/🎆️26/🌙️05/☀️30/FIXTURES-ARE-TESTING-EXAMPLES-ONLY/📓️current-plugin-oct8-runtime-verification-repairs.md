# Actual Runtime Verification Repairs — 2026-10-08

Actual registered facet suite initially ran148tests/31files,146passed2failed. It exposed3malformed true Puzzle diff payload references: artifact.json/$defs paths lacked the # fragment separator. Each referenced actual definition exists in artifact.json; corrected exactly3authored references. The real Ajv validator remains unchanged.

The remaining failure was a source assertion for a formerly spaced borrowed permit argument followed by the removed take_emit/dispatch path. Updated it to recognize whitespace-neutral borrowed permit syntax and require actual original completion/permit/lease transfer into mount_original_reserved_owners before fallible producer_fault processing and finish_mounted_reserved_completion. The independent RFC6902 refusal oracles and prohibition against redundant commit validation remain. Current native production code was untouched. Preimages and finalbytes preserved in the extended zz runtime ledger.

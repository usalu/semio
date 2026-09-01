# Source Index Static Cost Boundary — Review81

This is a read-only source observation, not a benchmark. Root read current root mutationTaxonomySourceIndex and the exact snapshot/classifier callsites while common/native source inputs are held unchanged.

The index first obtains shared admission, captures both schemas, discovers/filter roots and source-file facts, then captures every admitted selected file through semanticOwnedInputFileSnapshot. It creates byte/string maps, per-file hashes and a sorted source roster before returning the structural directory projection. No parsed mutation implementation or semantic coverage result follows just from this index. Two consecutive index calls retain their independent source observations; merging/reusing mutable source captures would require a separately reviewed authority contract, not merely a cache.

The ticket pair projects out byte/string owners only after each completed call, then serializes both retained snapshots. Each snapshot separately calls D.fileKindIdForSourcePath over all source paths, creates count maps, writes three JSONL artifacts and hashes their rows. Current D1181–1187 classifies by normalized filename and the longest unique file-kind extension chain supplied by taxonomy. It does not consult semanticDirectoryMemberKinds. That source observation does not itself quantify any phase, justify a cache or authenticate an old full-taxonomy endpoint after a vocabulary change.

Corrected diagnostic79 was not executed because its required whole-taxonomy6d06 endpoint became historical. Old78's per-path taxonomy hashing remains an invalid performance attribution. No collector/classifier/index/snapshot change is authorized by this static review; no deadline, evidence filtering or source-admission exception is introduced. Future profiling must separate capture, classification, serialization and final authentication and record actual phase times before choosing a change.


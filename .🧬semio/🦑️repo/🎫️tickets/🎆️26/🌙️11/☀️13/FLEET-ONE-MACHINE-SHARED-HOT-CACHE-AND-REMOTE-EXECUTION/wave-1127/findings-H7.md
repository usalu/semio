# Findings H7

- Lock `landing.lock.process` (owner H13, 00:20) was still there at 00:31 with the file untouched since the previous day; I did not take it, applied one atomic single-file edit and said so in result-H7.md. It is gone now (cleared).
- m05 supervisor: no further abandoned locks seen (remaining at 00:50: fleet-status, remote-execution).
- Unknown until `partitions.json` of g1-m05 arrives: which partition is full on m05 and what holds it. Read `S/origins/g1-m05/partitions.json` first (name, files, bytes, allocatedBytes, top holders per partition).
- Open: the compiler consumer path (`🏗️compiler` capacity recovery `♻️recovery`) does not retry on `space:true` refusals; if partitions.json shows `r` (rust) full, route its refusal into `recoverCompactCompilerHeadroomV1`/`relieveUnitCacheHeadroomV1` the way the lease store now reacts.

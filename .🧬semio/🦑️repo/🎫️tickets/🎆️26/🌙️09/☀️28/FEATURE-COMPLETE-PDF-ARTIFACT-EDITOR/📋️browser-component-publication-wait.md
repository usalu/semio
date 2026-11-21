# Browser Component Publication Directory Wait

The dual browser build linked the PDF component at 01:25:20 UTC. Its WASI file was 257,918,709 bytes. The capture stage waited for the shared lease directory and exhausted the fresh helper's 5,000 ms directory patience before it created the unique artifact capture lock. Stdio did not link after its three earlier Rust failures. The completed PDF bytes and original Cargo graph and diagnostic reports are retained in the ticket's generated output. The compiler source-witness store retired, so the preserved file alone cannot establish ordinary publication.

Fresh compiled artifact publication now uses the existing lease service with up to 60 seconds for shared directory admission and a zero timeout for each artifact's own lock. The original issued control still supplies the overall deadline, cancellation and progress. The rest of the fresh lease helper's callers keep their existing policy. No lock ownership or artifact provenance requirement changed.

The focused test held the real SQLite directory gate for 5.5 seconds. Before the fix it reproduced the same typed admission timeout. With the fix it published the exact witness after the gate released. It also refused a separately held exact destination lock without waiting and propagated cancellation while the directory gate stayed held.

The replacement governed dual-root build is session 53362, generation wvkdmN7, and compiler lane pdc55cf. It began testing the exact new publication path after source admission. The cache had supplied 158 actual restored compiler units at the first census. The browser mount still requires ordinary verified modules for both PDF and Stdio.

- [Publication contention proof](📜️fresh-publication-probe.ts)
- [Guarded production change](📜️fresh-publication-stage.ts)
- [Lease admission audit](fresh-publication-admission-audit.md)

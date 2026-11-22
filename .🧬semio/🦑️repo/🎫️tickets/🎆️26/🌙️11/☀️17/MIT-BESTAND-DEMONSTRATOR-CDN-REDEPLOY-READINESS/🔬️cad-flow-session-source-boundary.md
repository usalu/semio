# CAD Flow session source admission

The CAD log reported one outer `process.source.boundary-exceeded` with 262,839 source bytes against the 262,144-byte ceiling. Its nested `process.source.read-bytes-exceeded` reported the 6,518-byte session work file against 5,823 remaining bytes. These are the same refusal and its cause, not two retries or a corrupted read. The fresh native source admission correctly enforced the existing budget.

A targeted scan using the actual retained wGACoW7 WASI release graph initially measured 262,816 bytes in 21 files. Verbose window-latch API documentation was shortened without changing executable code. The same pinned graph and current source then measured 258,031 bytes in 21 files, with 2,887 active lines: 4,113 bytes below the byte limit and no refusal. A later five-byte wording correction provides slightly more margin. The concurrent session scheduling changes may alter that figure, so each queued build must still perform its normal source admission.

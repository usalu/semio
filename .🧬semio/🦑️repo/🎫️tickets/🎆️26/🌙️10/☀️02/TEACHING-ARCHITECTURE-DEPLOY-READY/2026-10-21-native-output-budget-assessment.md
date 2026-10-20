# Native Diagnostic Volume and Guest Budget Assessment

Observed 2026-10-20T23:12:10.596Z. Root read only finite existing native log extents and current controller/progress-writer Source. No Source edit, owner execution, guest test or policy change occurred.

The controller schema fixes outputBytes67108864 and outputLines262144. Its guest runner accumulates both across all21steps and stops with output-limit before accepting a chunk that would exceed either bound. Observation also refuses a larger actual output.log. These are whole-workload limits, not separate per-command limits.

A finite read of the actual terminal regression8log measured79,717,778bytes and492,658newline-framed records before test collection. The longest line was344characters; no oversized partial record was dropped. Most records were progress:189,912other script records,99,494ownership-list,80,198ownership-state,57,528store-input-replay,23,160installed-path,12,046preserving-source and11,401ownership-frontier records. This establishes substantial real diagnostic traffic rather than one oversized error dump.

A finite live neutral5extent measured9,688,490bytes and57,949records during dependency preparation. It contained22,026ownership-frontier,20,047other script and6,392store-input-replay records. Its captured extent is not a final log or terminal.

The current process invocation progress writer always constructs a raw script JSON line. It coalesces specifically classified source-provider/cache/tooling activities; unclassified activity remains raw output. The inspected path does not condition raw emission on CI. This is a concrete compatibility concern between actual diagnostic volume and the fixed guest whole-run budget. It is not an executed guest output-limit failure: Linux ownership cardinality, cache warmth and runtime routing can differ from the earlier host run.

A High assessment should trace the exact emission boundary and determine a narrow schema-first coalescing design if required, preserving command/phase transitions, cancellation/progress callbacks, final flush and raw terminal evidence. No blanket output-cap increase, diagnostic discard, Source guard bypass or immediate change to the live sealed cohort is authorized. Current135pins and TTY75119remain under their existing custody.

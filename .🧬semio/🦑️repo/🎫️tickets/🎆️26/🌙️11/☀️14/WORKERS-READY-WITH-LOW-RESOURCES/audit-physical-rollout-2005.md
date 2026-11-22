# Physical Rollout Sample at 20:05 UTC

The m05 replica attested the exact `06102e8b3eb19c67dd0eca3234e9920da806b8ff8288b41cc1f10d8d431d5516` code at 20:04:40 and 20:05:25 UTC. Its sharing-violation counter rose from 19,181 to 26,449, and incoming queue from 5,110 to 5,133. These are retry events and pending entries, not distinct paths or a controlled throughput measure. The attestation has no process birth or PID, so equal digest and increasing counters alone do not prove the same process epoch.

The fresh m05 Ready receipt remained waiting for a sidecar restart after a dependencies installer failure with status 1 and reference `8ef44f48`; its recorded sync diagnostic reported zero completed binding changes and no unresolved Source transition. It did not report a dependency success. The m07 Ready owner changed and its workspace sample subsequently displayed only a 26-second slice heartbeat, so its older leader inventory counters cannot be treated as a current progress comparison. Neither worker was Ready.

The live cold lab remained before-grant with a latest measured 36,602,859,520 free bytes against 60,129,542,144 required. No grant, image or lab stage result was observed. No worker, cache or process was changed.

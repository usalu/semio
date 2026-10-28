# Optimization Provenance Check

No original optimization source_state snapshot was retained by the import preflight; it checked imports/schema rather than recording complete segment_state closures. Original sidecars are removed by the normal renderer before replacing stale clips, so this audit does not reconstruct or rewrite an old certificate.

All six optimization leaf scripts import their common optimization/🎬️clip/🎨️visuals/🐍️.py. That common helper in turn literally imports optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py and the model. Runtime source_dependencies follows those literal imports, so a change to the room helper invalidates all six sources. The dependency path contains no cooling/regulations import; the local cooling/demand repairs themselves do not explain optimization invalidation.

The common optimization room helper has a modification time later than the original source pass, making it a concrete candidate for a concurrent input change. Modification time alone is not a byte comparison with a missing original sidecar; normal guarded rendering is the correct response. No optimization file was edited by this agent, and no source certificate was changed.

Current evidence:

```json
{
  "path": "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py",
  "mtime": "2026-10-27T12:53:39.751Z",
  "sha256": "29b3a3992cdd8f587f3762265dc26c31eb2d0daf083a75bb6950e5f1753a48ac",
  "attempts": [
    {
      "file": "attempt-1793101732340978200.json",
      "started": 1793101732.3861701,
      "finished": 1793102180.1406794,
      "exit_code": 0
    },
    {
      "file": "attempt-1793104975961334500.json",
      "started": 1793104976.5381744,
      "finished": 1793105368.970219,
      "exit_code": 0
    }
  ]
}
```

## Confirmed Concurrent Content Change

An earlier in-memory read of the same room helper was retained during this read-only audit. Its hash now differs from the live file, proving an actual content change during the investigation rather than only a touched timestamp. No source text was copied into the ticket, and this agent edited no optimization file. The common helper is in every optimization source closure; normal guarded refreshes must continue to honor the current bytes.

```json
{
  "prior_read_sha256": "8379bf721cfbdf32ce8ae97190735b08dea36b386fc86e5f6d035e3a55f49191",
  "current_sha256": "29b3a3992cdd8f587f3762265dc26c31eb2d0daf083a75bb6950e5f1753a48ac",
  "bytes_changed": true,
  "current_mtime": "2026-10-27T12:53:39.751Z"
}
```

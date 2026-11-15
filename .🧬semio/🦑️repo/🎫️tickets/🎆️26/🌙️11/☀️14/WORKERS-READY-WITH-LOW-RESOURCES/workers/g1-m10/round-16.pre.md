# Round 16 preparation

Measured at 2026-11-15T03:08:10.496962+00:00

Machine: g1-m10, group g1; macOS 27.2.1 (26B131), arm64, 16 GiB physical memory; available disk 139 GiB.

Replica attestations agree on code; both sync on and incoming queue zero at the checkpoint below. No replica restart requested.

```json
{
  "g1-m10": {
    "wallMs": 1794712090126,
    "code": "97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea",
    "sync": true,
    "queue": {
      "incoming": 0
    }
  },
  "g1-server": {
    "wallMs": 1794712090039,
    "code": "97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea",
    "sync": true,
    "queue": {
      "incoming": 0
    }
  }
}
```

Round 16 tools setup runs once before the first native try. All observation files remain in this worker folder. The native try will use P12 sampling every five seconds and top-three RSS and memory pressure every thirty seconds; the prescribed native wrapper may write its normal build/cache artifacts.

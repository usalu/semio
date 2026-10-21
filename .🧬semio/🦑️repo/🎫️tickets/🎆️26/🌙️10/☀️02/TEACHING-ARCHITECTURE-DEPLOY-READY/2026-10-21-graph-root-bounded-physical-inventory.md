# Graph Root Bounded Physical Inventory

Read-only lstat/opendir metadata only; no filecontents, Sourceevaluation, commands, SQLite probe or cleanup. Root .cache/b/g/m was ordinary directory dev16777253/ino2/mode16877, distinct from parentdev16777229, consistent with mounted image. Full directory ancestrydev/ino/mode remained equal before/after; no mountchange or ancestryrace observed.

Hard bounds honored: depth≤3, total20000directoryentries. Limit reached during build-input-closures; stopped without restarting to evade budget. Every descended directory stayed rootdevice; no links followed, no crossdevice descent, no observed races. Protocol namespace traversal order was filesystem order, so this is a PARTIAL inventory, not full3GiB attribution.

```json
{
  "entries": 20000,
  "maxDepth": 3,
  "limitReached": true,
  "root": {
    "path": "/Users/ueli/Documents/semio/.cache/b/g/m",
    "dev": "16777253",
    "ino": "2",
    "mode": "16877",
    "directory": true,
    "symbolic": false
  },
  "summary": {
    "storage-events.jsonl": {
      "entries": 1,
      "files": 1,
      "directories": 0,
      "logical": 237,
      "allocated": 4096,
      "symlinks": 0,
      "other": 0
    },
    "snapshots": {
      "entries": 1,
      "files": 0,
      "directories": 1,
      "logical": 64,
      "allocated": 0,
      "symlinks": 0,
      "other": 0
    },
    "build-lock-closures": {
      "entries": 3449,
      "files": 3448,
      "directories": 1,
      "logical": 757417,
      "allocated": 14123008,
      "symlinks": 0,
      "other": 0
    },
    "admission": {
      "entries": 1,
      "files": 0,
      "directories": 1,
      "logical": 64,
      "allocated": 0,
      "symlinks": 0,
      "other": 0
    },
    "build-input-closures": {
      "entries": 16548,
      "files": 16547,
      "directories": 1,
      "logical": 781772450,
      "allocated": 808734720,
      "symlinks": 0,
      "other": 0
    }
  },
  "races": [],
  "skipped": [],
  "ancestry": [
    {
      "path": "/Users/ueli/Documents/semio/.cache/b/g/m",
      "same": true
    },
    {
      "path": "/Users/ueli/Documents/semio/.cache/b/g",
      "same": true
    },
    {
      "path": "/Users/ueli/Documents/semio/.cache/b",
      "same": true
    },
    {
      "path": "/Users/ueli/Documents/semio/.cache",
      "same": true
    },
    {
      "path": "/Users/ueli/Documents/semio",
      "same": true
    },
    {
      "path": "/Users/ueli/Documents",
      "same": true
    },
    {
      "path": "/Users/ueli",
      "same": true
    },
    {
      "path": "/Users",
      "same": true
    },
    {
      "path": "/",
      "same": true
    }
  ]
}
```

Observed allocated subtotal822861824B (about784.7MiB), dominated by partial build-input-closures808734720B/16547files plus complete encountered build-lock-closures14123008B/3448files. snapshots directory encountered empty at its visit; admission directory likewise had no children at visit. These counts concern observed names/metadata only, not eligible reclamation or semantic owner authenticity. No inode/name is classified stale or peerowned.

The bound prevented coverage of later top namespaces inference, selected, tmp, optionalSQLite and possible deeper isolatedcontexts. Their initial rootnames were observed but their aggregate storage was NOT measured. Consequently this inventory cannot assign the remaining approximately3GiB occupation or infer coarseoptional/scoped eligibility. Logical sizes/allocated blocks are metadata, not exact statfs occupation (filesystem metadata/APFSsharing may differ). Further inventory would require explicit new bounded scope/budget rather than silently extending this scan.

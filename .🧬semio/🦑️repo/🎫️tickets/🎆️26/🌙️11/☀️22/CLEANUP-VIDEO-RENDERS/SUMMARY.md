# Summary

Deleted regenerable video render artifacts to free disk space.

## Removed

- Entire contents of `videos/` (~334M): partial scene caches, rendered mp4s, traces, SVG dumps
- Staffage render cache (`.🧬semio/🎓️teaching/⚡️cache/staffage/`, ~51M)
- This week's ticket tool logs under `🗑️generated`

## Kept

- All teaching/source fundamentals (scene Python, calculation modules)
- Ticket research/audit markdown

## Disk

Before:
```
Filesystem        Size    Used   Avail Capacity iused ifree %iused  Mounted on
/dev/disk3s1s1   228Gi    16Gi   533Mi    97%    459k  5.5M    8%   /

```

After:
```
Filesystem        Size    Used   Avail Capacity iused ifree %iused  Mounted on
/dev/disk3s1s1   228Gi    16Gi   930Mi    95%    459k  9.5M    5%   /

```

Sizes after: {"videos": "0B", "staffage": "0B"}

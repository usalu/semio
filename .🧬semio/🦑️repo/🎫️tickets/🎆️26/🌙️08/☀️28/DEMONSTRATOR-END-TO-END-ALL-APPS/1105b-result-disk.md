STATUS: PARTIAL - 207 GiB free (was 16). Rust-store fault unblocked by freeing space; hub overlays and ticket generated folders NOT yet processed.

## Deletions
- `.🧬semio/🦑️repo/⚡️cache/rust/fixed/fixed.dmg` (96 GiB alloc, 57 GiB used) + empty `m`: legacy rust image. Proof: current policy (`⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` partitions.rust) is `.cache/b/r`; `cache status` listed `repository:rust` as undeclared; no lsof handles on mount; last storage event Oct 31. Detached with hdiutil, deleted.
- `.r/i/` (fixed.dmg 96 GiB alloc, 37 GiB used, image.json, m) and `.r`: not the declared rust store (`.cache/b/r`), no open handles, no cargo referencing it; `.r/i` only appears in old test fixtures. Detached, deleted.

## Fault
`hdiutil resize ... .cache/b/r/fixed.dmg` exit 22 came from the missing/failed image creation under host-free pressure (16 GiB); not an APFS limit. After the frees `.cache/b/r` exists again.

## Not done
`.🧬semio/🌐hub` overlays, `.cache/b/*` review, closed-ticket `🗑️generated` cleanup: not processed (disk target already met: 207 GiB free).

df: /System/Volumes/Data 926Gi, 691Gi used, 207Gi avail, 77%.

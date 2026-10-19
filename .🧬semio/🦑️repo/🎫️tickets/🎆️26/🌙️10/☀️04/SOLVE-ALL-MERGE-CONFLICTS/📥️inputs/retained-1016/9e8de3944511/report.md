# Release build w1MP1ok

## Summary

| Metric | Value |
|---|---|
| Wall | 1.4 s |
| Compiled units | 0 |
| Failed units | 0 |
| Unit CPU | 0.00 CPU-h |
| Mean CPU (whole build) | 69.3 % |
| Peak rustc resident (sum) | 0.00 GiB |
| Frozen samples | 0 |
| Peak swap used | 0.00 GiB |

## Phases

| Phase | From s | To s | Mean CPU |
|---|---|---|---|
| head | 0.0 | 1.4 | 69.3 % |
| compile | 1.4 | 0.0 | n/a |
| tail | 0.0 | 1.4 | 69.3 % |

## Passes

| Pass | Profile | Target | Start s | Wall s | Exit | Failed packages |
|---|---|---|---|---|---|---|
| wasm-release-hub | wasm-release | wasm32-wasip2 | 0.0 | 0.9 | 0 | - |

## Critical path

| Unit | Unblocked by | Ready s | Start s | Wait s | Duration s |
|---|---|---|---|---|---|

Critical path length: 0.0 s.

## Top 20 units by CPU

| Unit | CPU s | Wall s | Peak GiB | Admission wait s |
|---|---|---|---|---|

## Top 20 units by peak

| Unit | Peak GiB | Predicted GiB | Wall s |
|---|---|---|---|

## Idle windows

| From s | To s | Mean CPU | Cause |
|---|---|---|---|

## Relink fan-out (L9)

| Library | Components | Mean component unit s | Fan-out cost s |
|---|---|---|---|

## Stage overlap

8 stage(s), 1.5 s of stage time; 0 (0.0 s) started while units still compiled, 8 (1.5 s) after the last unit.

## Stage cache

| Metric | Value |
|---|---|
| Root | C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️04\SOLVE-ALL-MERGE-CONFLICTS\🗑️generated\native\j-WAE46Z\cache\stages |
| Hits | 0 |
| Misses | 0 |
| Restored MiB | 0.0 |
| Stored MiB | 0.0 |

## Provenance custody

| Metric | Value |
|---|---|
| Files hashed | 12 |
| Body bytes read | 559630 |
| Distinct file bytes | 279815 |
| Reads per file (max) | 2 |
| Final recheck | passed |

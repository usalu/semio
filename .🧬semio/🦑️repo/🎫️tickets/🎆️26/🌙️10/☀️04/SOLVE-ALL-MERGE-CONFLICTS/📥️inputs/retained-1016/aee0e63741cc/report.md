# Release build fOITOrn

## Summary

| Metric | Value |
|---|---|
| Wall | 3.4 s |
| Compiled units | 6 |
| Failed units | 0 |
| Unit CPU | 0.00 CPU-h |
| Mean CPU (whole build) | 82.0 % |
| Peak rustc resident (sum) | 0.01 GiB |
| Frozen samples | 0 |
| Peak swap used | 0.00 GiB |

## Phases

| Phase | From s | To s | Mean CPU |
|---|---|---|---|
| head | 0.0 | 0.6 | 100.0 % |
| compile | 0.6 | 3.3 | 96.9 % |
| tail | 3.3 | 3.4 | 49.0 % |

## Passes

| Pass | Profile | Target | Start s | Wall s | Exit | Failed packages |
|---|---|---|---|---|---|---|
| wasm-release-00 | wasm-release | wasm32-wasip2 | 0.1 | 3.2 | 0 | - |
| wasm-prototype-01 | wasm-prototype | wasm32-wasip2 | 0.1 | 3.2 | 0 | - |

## Critical path

| Unit | Unblocked by | Ready s | Start s | Wait s | Duration s |
|---|---|---|---|---|---|
| base@base (lib, wasm32-wasip2, wasm-prototype) | root | 0.7 | 0.7 | 0.0 | 0.4 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-prototype) | full | 1.1 | 1.3 | 0.2 | 1.9 |

Critical path length: 2.6 s.

## Top 20 units by CPU

| Unit | CPU s | Wall s | Peak GiB | Admission wait s |
|---|---|---|---|---|
| base@base (lib, wasm32-wasip2, wasm-prototype) | 0.2 | 0.4 | 0.08 | 0.0 |
| base@base (lib, wasm32-wasip2, wasm-release) | 0.1 | 0.5 | 0.07 | 0.0 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-release) | 0.1 | 1.9 | 0.06 | 0.0 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-prototype) | 0.1 | 1.9 | 0.06 | 0.0 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-release) | 0.1 | 1.9 | 0.06 | 0.0 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-prototype) | 0.1 | 1.9 | 0.06 | 0.0 |

## Top 20 units by peak

| Unit | Peak GiB | Predicted GiB | Wall s |
|---|---|---|---|
| base@base (lib, wasm32-wasip2, wasm-prototype) | 0.08 | 0.60 | 0.4 |
| base@base (lib, wasm32-wasip2, wasm-release) | 0.07 | 0.60 | 0.5 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-prototype) | 0.06 | 6.00 | 1.9 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-release) | 0.06 | 6.00 | 1.9 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-prototype) | 0.06 | 6.00 | 1.9 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-release) | 0.06 | 6.00 | 1.9 |

## Idle windows

| From s | To s | Mean CPU | Cause |
|---|---|---|---|

## Relink fan-out (L9)

| Library | Components | Mean component unit s | Fan-out cost s |
|---|---|---|---|
| base@base (wasm-release) | 2 | 1.9 | 3.9 |
| base@base (wasm-prototype) | 2 | 1.9 | 3.9 |

## Stage overlap

7 stage(s), 0.2 s of stage time; 4 (0.1 s) started while units still compiled, 3 (0.1 s) after the last unit.

## Stage cache

| Metric | Value |
|---|---|
| Root | C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️04\SOLVE-ALL-MERGE-CONFLICTS\🗑️generated\receipts\r-FjriJy\cache\fOITOrn\s |
| Hits | 0 |
| Misses | 0 |
| Restored MiB | 0.0 |
| Stored MiB | 0.0 |

## Provenance custody

| Metric | Value |
|---|---|
| Files hashed | 31 |
| Body bytes read | 23948650 |
| Distinct file bytes | 11974325 |
| Reads per file (max) | 2 |
| Final recheck | passed |

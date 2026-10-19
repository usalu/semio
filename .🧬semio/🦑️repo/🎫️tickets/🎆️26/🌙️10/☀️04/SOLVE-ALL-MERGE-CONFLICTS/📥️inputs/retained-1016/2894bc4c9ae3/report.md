# Release build fxP1er6

## Summary

| Metric | Value |
|---|---|
| Wall | 4.5 s |
| Compiled units | 6 |
| Failed units | 0 |
| Unit CPU | 0.00 CPU-h |
| Mean CPU (whole build) | 100.0 % |
| Peak rustc resident (sum) | 0.07 GiB |
| Frozen samples | 0 |
| Peak swap used | 0.00 GiB |

## Phases

| Phase | From s | To s | Mean CPU |
|---|---|---|---|
| head | 0.0 | 0.7 | 100.0 % |
| compile | 0.7 | 4.3 | 100.0 % |
| tail | 4.3 | 4.5 | 100.0 % |

## Passes

| Pass | Profile | Target | Start s | Wall s | Exit | Failed packages |
|---|---|---|---|---|---|---|
| wasm-prototype-01 | wasm-prototype | wasm32-wasip2 | 0.1 | 4.2 | 0 | - |
| wasm-release-00 | wasm-release | wasm32-wasip2 | 0.0 | 4.4 | 0 | - |

## Critical path

| Unit | Unblocked by | Ready s | Start s | Wait s | Duration s |
|---|---|---|---|---|---|
| base@base (lib, wasm32-wasip2, wasm-release) | root | 0.7 | 0.7 | 0.0 | 0.5 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-release) | full | 1.2 | 1.2 | 0.0 | 3.1 |

Critical path length: 3.7 s.

## Top 20 units by CPU

| Unit | CPU s | Wall s | Peak GiB | Admission wait s |
|---|---|---|---|---|
| base@base (lib, wasm32-wasip2, wasm-release) | 0.2 | 0.5 | 0.07 | 0.0 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-release) | 0.2 | 3.1 | 0.06 | 0.0 |
| base@base (lib, wasm32-wasip2, wasm-prototype) | 0.1 | 0.5 | 0.08 | 0.0 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-release) | 0.1 | 3.1 | 0.06 | 0.0 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-prototype) | 0.1 | 2.7 | 0.06 | 0.0 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-prototype) | 0.0 | 2.7 | 0.06 | 0.0 |

## Top 20 units by peak

| Unit | Peak GiB | Predicted GiB | Wall s |
|---|---|---|---|
| base@base (lib, wasm32-wasip2, wasm-prototype) | 0.08 | 0.60 | 0.5 |
| base@base (lib, wasm32-wasip2, wasm-release) | 0.07 | 0.60 | 0.5 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-prototype) | 0.06 | 6.00 | 2.7 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-prototype) | 0.06 | 6.00 | 2.7 |
| comp-a@comp_a (cdylib, wasm32-wasip2, wasm-release) | 0.06 | 6.00 | 3.1 |
| comp-b@comp_b (cdylib, wasm32-wasip2, wasm-release) | 0.06 | 6.00 | 3.1 |

## Idle windows

| From s | To s | Mean CPU | Cause |
|---|---|---|---|

## Relink fan-out (L9)

| Library | Components | Mean component unit s | Fan-out cost s |
|---|---|---|---|
| base@base (wasm-release) | 2 | 3.1 | 6.2 |
| base@base (wasm-prototype) | 2 | 2.7 | 5.4 |

## Stage overlap

7 stage(s), 0.3 s of stage time; 3 (0.2 s) started while units still compiled, 4 (0.1 s) after the last unit.

## Stage cache

| Metric | Value |
|---|---|
| Root | C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️04\SOLVE-ALL-MERGE-CONFLICTS\🗑️generated\receipts\r-OqwhKN\cache\fxP1er6\s |
| Hits | 0 |
| Misses | 0 |
| Restored MiB | 0.0 |
| Stored MiB | 0.0 |

## Provenance custody

| Metric | Value |
|---|---|
| Files hashed | 31 |
| Body bytes read | 23949359 |
| Distinct file bytes | 11974421 |
| Reads per file (max) | 3 |
| Final recheck | passed |

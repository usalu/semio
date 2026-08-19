# Custom scale apisym axis fix

## Problem

`\SemioVizAxis[orient=left, scale=apisym]` with `\SemioVizScale{apisym}{symlog}{-10, 10}{8, 32}` failed during tick mapping:

```
Invalid operation (0)/(0)
```

## Root cause

The symlog branch nested `\semio_viz_asinh:n` inside `\fp_eval:n` and passed `\l_semio_viz_domain_lo_fp` / `\l_semio_viz_domain_hi_fp` as braced `n` arguments. The fp registers did not expand inside the inner `fp_eval`, so domain endpoints were treated as zero. With symmetric negative/positive ticks this produced a zero-over-zero division.

## Fix

- Replace `\semio_viz_asinh:n` with `\semio_viz_asinh:nN` that writes to an fp variable.
- Expand domain endpoints via `\fp_use:N` before asinh.
- Compute symlog mapping from stored fp values in a single outer `fp_eval`.
- Keep dynamic cs storage for domain/range clists (required for `viz-y` from `\use:x`); mirror kind/domain/range into props for axis kind lookup.

## Verification

- `bun ./print/script.ts build viz api` — exit 0 (light + dark)
- `bun ./print/script.ts test viz` — 1966/1966 leaves, API 13/13

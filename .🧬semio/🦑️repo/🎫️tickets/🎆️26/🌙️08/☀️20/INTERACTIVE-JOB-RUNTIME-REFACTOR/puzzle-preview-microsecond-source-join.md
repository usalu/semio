# Puzzle Preview Microsecond Source Join

The permanent source verifier now requires the actual microsecond clock, checked 2,000us deadline addition, missing-clock denial, equality expiry, and monotonic previous-read authority. Six additional hostile source mutations remove or weaken those exact obligations; every mutation must alter production source.

The caller had a real backward-clock gap. It now retains the previous reading and returns the existing last-valid page without advancing the preview when a new reading moves backward. Missing readings still stop before work. No millisecond fallback or increased grant was introduced.

Canonical source RED: `🧪️puzzle-preview-clock-source-red-r1-2026-08-27.txt`, exit 1, newly required start-clock authority absent before the caller repair.

Canonical rerun: `🧪️puzzle-preview-clock-source-r2-2026-08-27.txt`, exit 1. Puzzle preview baseline and hostile mutations completed, then the independent live-reconcile self-test stopped at the stale `render-fault-wire-erased` mutation. This is not a full static-gate pass.

Changed production caller: Puzzle 3d editor precompute `fill_preview_json_page`. Its native/Wasm runtime remains unexecuted under the heavy-build hold. No claim of whole-app latency or publication completion follows from this source gate.

# Remaining Verification

Source work is retained in the shared checkout. No native compiler result has been obtained: repeated repository gates required 3584 MB above the protected 2035 MB floor, while the host provided substantially less. Do not bypass this gate or terminate other developers work.

## Native Unit Gate

repo cargo --lang=en --store=shared --configuration=prototype --parallelism=1 -- test -p semio-ui-draw-contract -p semio-ui-prepared -p semio-ui-wgpu-controls -p semio-ui-wgpu-paint -p semio-ui-wgpu-layout -p semio-ui-wgpu-widgets -p semio-ui-wgpu-engine -p semio-ui-wgpu-render -p semio-ui-tui-laws -p semio-ui-tui-unit-laws -p semio-renderer-wgpu-ui-engine -p semio-renderer-wgpu-frame-job --lib -- --test-threads=1

## Updated Browser Contract and Component Gate

repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️05/CONSOLIDATE-NEIGHBORING-UI-BORDERS/verify-ui.ts --pool=threads --isolate=false --reporter=verbose segments.test.ts shared-border-dock.test.tsx

The owned adapter uses the existing runVitestV1 interface, one worker and a 600-second execution budget. This includes the new Search seam and appended-frame/clip/partition oracle tests. Earlier React suites passed 90 tests with one Chromium-dependent skip; five independent Edge seam checks also passed. Newer cases are not yet executed.

## Completion

Repair all native compilation or assertion failures, including the production OS engine and frame-close regression. Verify every work receipt against its preflight and production worker policy. Run repo tools exec --light git -- diff --check after repairs. Update SUMMARY.md and acceptance-current.md with actual results; delete only ticket-generated outputs under 🗑️generated when done, retaining input scripts and Markdown audits. Close the existing ticket through repo MCP once its server working directory is corrected (see coordination.md), with all changed file paths; mark the goal complete only after required verification and cleanup succeed.

# Repo Tooling Recheck (2026-11-22 ~14:20)

- `repo run @semio-tech/process-js:test` (fleet idle at start): exit 124 after the 2700 s timeout. Never left the project-graph phase (file inventory, 1973 owner checks, 547 owners admitted, then no progress); "Original Script process terminated (reference 557d4d71)" on SIGTERM. No compile/test error reached.
- `@semio-tech/process-extension-wood-rust:test`: stopped, redundant (same graph phase).
- Conclusion: repo path still unusable for the process plugin; native fallback remains the verification path (extensions already green natively: metal 5, wood 6, robotic 5, concrete 5).

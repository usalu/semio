# Search Admission Observation

Read-only process RSS observation while final Go and formatter checks wait for capacity. Raw command lines and environment variables were not retained. No processes or leases were modified. The temporary observation process is excluded from the search-request subtotal.

- PID 194452; bun; RSS 1753.0 MiB; parent 194143; @semio-tech/framework-os-mcp-rs:build
- PID 210006; bun; RSS 1716.4 MiB; parent 209312; @teaching/architecture-course:test
- PID 212277; bun; RSS 1427.9 MiB; parent 194452
- PID 242964; bun; RSS 1066.6 MiB; parent 242848; workspace:test-dependency-readmission
- PID 229702; bun; RSS 508.8 MiB; parent 229429; @semio-tech/print:build-thesis
- PID 211237; bun; RSS 445.7 MiB; parent 211110; @semio-tech/framework-os-mcp-rs:build
- PID 44513; codex; RSS 252.9 MiB; parent 41941
- PID 229429; bun; RSS 87.3 MiB; parent 214878; @semio-tech/print:build-thesis
- PID 4503; plasma-systemmo; RSS 85.2 MiB; parent 2339
- PID 211110; bun; RSS 76.0 MiB; parent 171027; @semio-tech/framework-os-mcp-rs:build
- PID 242848; bun; RSS 72.8 MiB; parent 242845; workspace:test-dependency-readmission
- PID 244750; bun; RSS 63.8 MiB; parent 44513

Known queued search command-line matches total approximately 155.4 MiB; this subtotal excludes the observation process and is not a complete process-tree accounting. Largest observed processes are Bun runtimes, including captured build and graph-farm processes. The observation does not justify weakening admission or stopping concurrent work.

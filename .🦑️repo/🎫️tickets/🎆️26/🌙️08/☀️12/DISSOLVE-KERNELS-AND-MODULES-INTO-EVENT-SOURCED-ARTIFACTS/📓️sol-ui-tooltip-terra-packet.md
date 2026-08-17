# Terra Packet UI-Tooltip-01: Zero-Consumer Dissolution

- Read AGENTS/audit; apply_patch only, no modifying Git.
- Require component SHA `991a8aef87f1f42236f0ff9deac758fdd5a8b86456342c293912b3e231fad5d7` and story SHA `559ce277a52d8363821e1512cda8b719b977b23e1bae4e0ae42e5c1add3ff8e8`.
- Current shared index baseline is announced by coordinator; Terra must not edit it.
- Terra owns only Tooltip component/story and unique `📓️terra-ui-tooltip-zero-active-consumer-dissolution-acceptance.md`.

Delete component/story, confirm absence/empty authored directory, then checkpoint and wait. Coordinator owns all shared-index wrapper/import/export removal and the deferred dependency queue.

After registrar signal, run exact active stale scans for all exported Tooltip family symbols/types/direct paths/JSX, scoped ordinary/cached diff checks, and UI React lint/typecheck/test-quick/build once. Exclude native Rust overlay names as separate implementations. Do not touch manifests, locks, generated census, other UI leaves, Storybook config, protected renderer, or plugins; do not repair unrelated failures.

# Hub Closure Growth (2026-11-24)

Baseline: `🗑️generated/native-component-flow-hub-8.txt` (hub-6 log absent). Compared: `native-component-flow-hub-13.txt`.

## Findings
- Distinct packages 401 → 402. Only new: `semio-framework-value-native-record-machine`, a normal dependency of `semio-framework-value-native-owned` (split of 2026-11-24, see [📓️value-native-owned-split-2026-11-24.md](./📓️value-native-owned-split-2026-11-24.md)).
- Path: `semio-hub-flow-component` → `semio-hub-flow` → `semio-framework-pack-json` → `native-owned` → `native-record-machine`.
- The 4243 vs ~1020 units is a run-wide total over restarted passes (hub-13: SIGKILL/OOM, snapshot retry, disk-full retry), not closure growth. Per-pass count is ~1023 in both; hub-8 run total was 3237.
- No host/test-profile doubling: every unit is `profile.test=false`, triple `wasm32-wasip2` only.

## Open
- Restarted passes show `0 reused` (`fresh:false`, 311 units not shared): a pass restart after a single unit SIGKILL recompiles units already compiled in the failed pass. Fixing that (restore units of the failed pass from the local store) would cut a restart from ~25 min to the failed unit alone.
- The memory-heavy unit that triggered the restarts: [📓️stdio-semio-compile-memory-2026-11-24.md](./📓️stdio-semio-compile-memory-2026-11-24.md).

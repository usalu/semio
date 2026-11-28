# H68: stream divergences (stdout/stderr of compile units differ between origins)

## What the kept pairs say (server evidence, 11 stream-only unit pairs 11-26 22:00 .. 11-27 10:15; `.cache/b/p/m/fleet/cache-server/fleet/evidence`)
All differences are in the `rendered` text of rustc's JSON diagnostics (the `message`, `code`, `spans`, `level` are identical), never in paths, temp dirs, pids, timings or host names:
- Pairs 06:2x-10:15 (`framework_trace`, `value_source_turn`, `pack_json_float`; kept m15/m07, refused m07/m10/m15): one side rendered ANSI-human (`\e[1m..warning..\e[0m`, `--> file`), the other the short form (`file:l:c: warning: msg`) or ANSI-short; also the `N warnings emitted` summary (`\e[1m..\e[0m\n\n` vs `\e[33mwarning\e[0m: ..\n`).
- Pairs 04:xx (`plugin_instance_dispatch`, 6 pairs): the long-type number (`long-type-<19 digits>.txt`), already normalised by H49 (`canonicalStreamV1`, 07:00).
- Cause: `normalizeRustcArgumentsV1` drops `--json=diagnostic-rendered-ansi/diagnostic-short`, `--color`, `--diagnostic-width` from the key on purpose (Cargo sets them per terminal and `--message-format`), so two honest machines build one key with another rendering. Not parallel-codegen order (pairs have the same line multiset), not paths.

## Decision
Streams stay stored (warnings stay replayable, the first writer's rendering is replayed, as before), but the rendering is no part of the divergence identity. Capture-side normalisation of `rendered` is impossible (short and human forms cannot be converted into each other; stripping would lose the replayed text), so the comparison is done where both blobs meet, the cache server:
- `provenanceStreamIdentityV1`: JSON lines without any `rendered`, long-type number 0, other lines without ANSI, lines sorted (order of independent diagnostics).
- `provenanceStreamsEquivalentV1`: true only when the differing paths are exclusively `⟨stdout⟩`/`⟨stderr⟩`, both blobs are readable and the identities are equal. The server then answers `200 Stored`, keeps the first result, counts `advisory["stream-rendering"]` (visible in status), writes no divergence and no evidence. Anything else (another message, an additional diagnostic, a gone blob, an output file differing too) is a divergence as before.
- Schema: no captured format changed; the H55 `outputDeterminism` version stays 2 (the key and stored bytes are untouched).

## Proof
Fixture `🧾️provenance/🧪️tests/🧫️fixtures/🔣️.json` `streams` (6 cases: 3 real evidence pairs, order differs, another message, extra diagnostic) checked by TS and by the independent Python oracle (`🐍️.py`, `ok streams 6`); server test: same diagnostic in two renderings -> 200 and `advisory.stream-rendering` 1, another diagnostic -> 409 and divergence count 1. Provenance 8/8 (one run showed the existing 1000-row test timing out at 5 s under load, passes on rerun), server access 9/9, action-result 4/4, typecheck passed (113 files), dangling imports 0.

## Rate
Before: 2.6/h since 07:00 (A6), 7 of 9 stream. After: NOT measured. The server restarted 10:07 and its hand-over policy (quiet 15 min, minimum interval 3 h) puts the new code live at about 13:07 at the latest; measure then with `repo remote-cache status` (`divergenceHours`, `advisory`). Expect stream class 0 (the rendering pairs now count as `stream-rendering` advisories); remaining: the rlib/rmeta-with-dep-info row of 10:00 (`dwg_snapshot_objects_metadata`, `wasm_encoder`/`wasmparser` rlib only) are other classes, not examined here.
Files: `🧾️provenance/🟦️.ts`, `🖥️server/🟦️.ts`, tests `🧾️provenance/🧪️tests/{🟦️.ts,🐍️.py,🧫️fixtures/🔣️.json}`, `🖥️server/🧪️tests/🔐️access/🟦️.ts` (all under `🏗️build/🌐️remote-cache`), scripts `T/🧰️scripts/H68/`.

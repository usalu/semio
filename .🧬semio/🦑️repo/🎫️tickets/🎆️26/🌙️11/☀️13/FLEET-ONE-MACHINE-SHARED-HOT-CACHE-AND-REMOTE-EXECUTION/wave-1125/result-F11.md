# Result F11: nothing computed twice (divergences, Nx hit rate)

Time 16:30 to 17:00 CET. Scripts: `T/🧰️scripts/F11/` (`evidence-list.ts`, `fetch-evidence.ts`). Data read: server index `.cache/b/p/m/fleet/cache-server/fleet/{divergences.json,provenance.jsonl}` and `/evidence` of the live server (6094), no token printed.

## 1. Divergences since 11-22 (E66 said "unit fixed, none since 02:55": that is WRONG)
`divergences.json`: lifetime 959 (unit 890, owner-pool 41, project-inference 16, closure 12). Per day, unit: 11-22 100, 11-23 194, 11-24 107, 11-25 185 (until 16:30). Since 11-22 00:00: owner-pool 21 (all 11-22), closure 12 (11-22), project-inference 14 (11-22, superseded class), so **graph classes stopped on 11-22; unit divergences did not**. 586 unit rows since 11-22.
- Origins (kept/refused): semio-namespace m02/m07 (247 + 63), main-namespace m07/m10 (63), m08/m02, m12/m08, m13/m02, m12/m02 ... all macOS pairs, every machine appears; no Linux/Windows.
- Namespaces: the fleet still writes TWO namespaces. AC puts per hour (provenance): `semio` 7000/h at 08-13h, 1933 at 14h, 373 at 15h, 1049 at 16h; `main` 11 to 13k/h. Last hour `semio` writers: m06 737, m07 295, g1-server 15, m02 2 (machines still on the older client default). Those writes are invisible to every reader of `main` (a hit lost, a unit computed twice).
- Evidence retained by the server (183 rows, 24 h): 64 unit rows, 62 differ in `.rlib/.rmeta/.semio-deps`.

### Proven root cause of the current unit class (one live pair analysed to the byte)
Evidence `1795613968738-unit-411f86c00bde-g1-m02` (`semio-framework-value-native-dynamic-clone`, kept m14, refused m02, same AC key): sources have the same blake3 (813cde5b...), but the dep-info names the crate root `.../dynamic/📦️packages/🦀️rust/../../🦀️.rs` on one machine and `.../dynamic/🦀️.rs` on the other (Cargo spells `[lib] path = "../../🦀️.rs"` unnormalised; the module-remap/offload route rewrites the source argument to a normalised absolute path). rustc records the spelling in names, spans and the crate hash: svh `e7578cac...` vs `28009d04...`. Independent oracle: one file compiled by plain rustc as `sub/dir/../lib.rs` and `sub/lib.rs` gives different rlib and rmeta bytes (md5 differ) although the key inputs are the same. Pair 2 (`wasmparser` rmeta m08/m14, 87 bytes apart, rlib equal, strings equal) is NOT explained by this and is still open (see below).

## 2. Nx hit rate 0.2 %
Server counters since 11:51: nx hits 1, misses 439, puts 107, rejected 27 (400 content-hash mismatch, entry removed). Provenance: 160 nx puts since 08:47, **160 distinct keys, no key put by two origins** (10 origins, 7 to 22 puts each; 5 in namespace `semio`). On disk only 1 of those entries is left (2 nx files in both namespaces): entries are superseded or retired within an hour.
Findings (proved: counters, provenance, disk; NOT proved: that two origins hash the same task differently):
1. Reads are attempted (439 misses) but only about one in four tasks ever reaches a successful put: the workspace stage Nx run-many exits 1 on 9 workers (`Owned Nx invocation failed`), failed tasks are never put, and the 27 rejected puts delete their entry.
2. Every key was produced once, so all current keys are tied to one machine's moment of the tree: the replicated worktree is edited all day (`{projectRoot}/**/*`, `schemaSources`, `sharedGlobals` inputs), so two machines that run the same task minutes apart hash different inputs. This is a legitimate miss, not a leak; a measured two-origin hash pair was not obtainable: `repo inputs repo-mcp:build:prototype` ran 12 min and failed because the graph image `.cache/b/g` is full (17.96 of 18 GiB, 37 MiB free) on this host, and workers keep no Nx hash logs (`ready-log-workspace*.txt` carry no hashes or `[remote cache]` markers).
3. Namespace split: Nx puts also went to `semio` (5) and `main` (155).
No machine-specific input (absolute path, env, timestamp) was found in `nx.json` named inputs (runtime = platform+arch and `bun --version`, env SEMIO_BUILD_MODE); the patch hashes via the native `hashPlans` with exact source snapshots. No Nx code change landed.

## 3. Fix landed (see `T/landings.md` "Wave 1125 F11")
`unit-cache/🧭️remap/🟦️.ts` `canonicalSourceArgumentsV1` + one call in `📜️script.ts` `wrapRustcV1` (after module remap, before key/offload/compile): a relative crate root with `..`/`.` segments is spelled in normal form when `stat` (dev, ino) proves it is the same file (links keep the spelling). All routes now hand rustc one spelling.
Tests: remap 9/0 (new: rustc-oracle byte equality of rlib and rmeta with a control that proves the spelling alone differs; spelling rules), cross-root 2/0, metadata 4/0, manifest 1/0, cutoff 4/0.

## NOT verified / open
- No live effect measured: machines pick the change up on their next supervisor restart; re-read `divergences.json` unit count per day after about an hour (baseline 107 to 194 per day, 185 on 11-25 until 16:30; expect the `packages/rust/../../` class to vanish; hit rates before: AC 47.7 %, CAS 77.9 %, nx 0.2 %).
- The key spelling: I did not trace why the AC key stayed equal for both spellings (the arg is in `normalizeRustcArgumentsV1`); after the fix the question is moot for outputs but a key audit would be good.
- Open class: `wasmparser` style registry-crate rmeta differences (rlib equal, rmeta differs by 87 bytes, same strings): not diagnosed, and registry crates compiled from a per-user cargo home (`~/.cargo/registry/src/...`) are a suspect.
- Namespace `semio` writers (m06, m07, g1-server) still exist: find which client default or env sets it and unify to `main`; I did not change it.
- Nx: provide `repo inputs` on a host with a free graph image; the full image blocks it. Free it with `repo cache gc`.

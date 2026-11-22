# D2: Component stage keys and local/fleet stats

## Closure findings (read, not guessed)
- transpile (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts`): the closure runs the NATIVE binary `semio-native-transpile` (jco embedded), not node/jco, and never reads `preview2VendorDir` (only `assemble`/`probe` use the vendor tree). Inputs: stripped wasm bytes (digest), native binary digest (covers jco version and build mode), four materializer sources, base name, import maps. The D1 claim "vendor tree/node/jco missing" is wrong for transpile: they are not inputs. Added: import-map digest as explicit tool; schemaVersion 2 -> 3. No profile in key (binary digest per mode already differs; output otherwise identical). No absolute path enters any key.
- optimize: wasm-opt output verified independent of BINARYEN_CORES (3.9 MB core, 1 vs 8 cores, identical sha256 18c912a8...). Added the wasm-opt BINARY content digest (a same-version differing build would have collided); schemaVersion 1 -> 2.
- probe: re-verified: inputs = digest of every module file + every vendor file (relative names), tools node version + probe source sha. Complete; unchanged.
- describe (`PL/🖨️describe/🧩️component-stage/🟦️.ts`): emitter reads only the component file; key = component digest + emitter binary digest, `prepareActor` encoded in schemaVersion (1/2). Complete; unchanged.

## Stats split
- `StageCacheStatsV1.fleetHits` added; local store reports 0; `sharedStageCacheV1.stats()` now reports hits = local hits, fleetHits = served, misses = local misses - served (previously fleet hits were folded into hits).
- Progress line, end summary: `stage cache L local / F fleet hit(s) / M miss(es)`; report "Stage cache" section rows: Local hits, Fleet hits, Misses. Texts are inline (not schema-driven); no EN/DE stage-cache text exists to update.

## Tests (repo test-file)
- new `commands/🧪️tests/🔑️stage-keys`: 2 pass / 0 fail (same inputs under two checkout roots equal; vendor tree, jco(native) digest, import maps, wasm-opt binary, node version each change the key; schema versions).
- probe-key 1/0; stage-fleet 13/0 (assertion updated for split); stage-cache 6/0.
- `commands/🧪️tests/🎚️mode`: 2 pass / 1 fail: model reader `receipt.profile` predicate count in `📦️model/👓️reading`; unrelated to this lane.

## Open
- No type-check/tsc run; no play build rerun (existing stage entries re-key once: new schema versions).

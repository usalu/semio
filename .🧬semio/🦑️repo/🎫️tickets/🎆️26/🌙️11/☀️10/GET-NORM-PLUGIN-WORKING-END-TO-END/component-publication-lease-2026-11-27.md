# Component publication never waits on a long-lived reader (content-addressed generations)

## Design
- Layout (new, distinct from the old `browser/<mode>/components/<key>`): `browser/<mode>/component-models/<key24>/head.json` plus immutable generation directories `component-models/<key24>/<gen24>/`. Constants live in the model schema (`Layout`, `Head` in `plugin/📦️model/🧬️schema/🔣️.json`).
- Generation name = first 24 hex of sha256 over [sha(receipt), sha(emitter binding)|null, actorImageSha256|null]. Readers re-derive it and refuse a directory whose name does not match its bytes.
- `head.json` = `{version, generation, previous[]}`. Replaced by temp file + rename under a short writers-only exclusive lease on `artifact:<head>`. Readers never lease the head.
- Publication (`publishComponentModelV1`): stage the generation with the new `immutable` option (intact identical bytes are reused under the shared lease, so a same-generation reader cannot block; only a new/damaged generation takes the exclusive lease, bounded to 30 s), then `promoteComponentModelGenerationV1`: swap head, add the superseded generation to `previous` (the reclamation ledger), retire every ledger entry with a non-waiting exclusive claim (`retireArtifactDirectoriesV1`), held ones stay in the ledger for the next publication. Head is recorded in the store receipt.
- Readers: `withComponentModelV1` resolves head, leases only that generation, retries (before the consumer starts only) if the generation vanished. MCP environment, wgpu native runtime, playground check and BIM guest digest resolve the head the same way. The MCP gateway `models` refresh now re-resolves heads and retires its previous hold; the initial hold is retirable too.
- Not done: a push/change event for a running MCP child; it switches on the next refresh/`models` call (re-resolve on use).

## Files
- publication: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts` (`immutable`), `repo/.../⚡️caching/📦️artifacts/🟦️.ts` (`stageRepositoryArtifacts` options)
- model: `plugin/📦️model/{🧬️schema/🔣️.json, 📋️contract, 🧭️head (new), 🟦️.ts, 👓️reading}`, package wrapper `📦️packages/🦀️rust/📦️model/🧭️head` + export `./model/head`
- readers: `mcp/📡️stdio/📦️component-models/🟦️.ts`, `mcp/🟦️.ts`, wgpu `native-entrypoint/📦️modules/🟦️.ts`, `dev/🎮️playground/🟦️.ts`, `bim/.../🪪️guest-digest.ts`, Nx coordinate `componentModelOutputV1`
- Rust: `mcp/🏠️workspace/🧭️plugin-paths-and-descriptor/🦀️.rs` (manifest root check reads Layout), `cold-image.rs`, `mcp/📦️packages/🦀️rust/📋️project.json` (schema input)
- sweep: `$T/verify-play-components.mjs` reads head then generation (verify-standards-publication.ts and probe-norm-base-companions.ts only delegate to it, no change needed)
- tests: new `plugin/📦️model/🧪️tests/🔁️generation-publication/{🔣️.json,🥒️.feature,🟦️.ts}`; updated vite artifacts test and mcp client-journey fixture/test.

## Results
- New suite: 4/4 pass (reader holds N, N+1 publishes without waiting, new reader resolves N+1; ledger reclaims after holder exits; identical republish of held generation reuses inode, exclusive path still blocked; names equal independent WebCrypto digest).
- Pass: mcp client-journey 12, publication-revalidate 8, staging publication 24, component-models 12, publication digest 4. Typecheck of all changed TS passes.
- Vite artifacts test: 37 pass, 9 fail. The failures are not from this change (e.g. `withCompilerWorkspaceWriteV1` name absent from model source, undefined `SEMIO_TICKET_DIR`, private-repository retirement); I fixed the one assertion my change broke (native-entrypoint now contains `resolveComponentModelRequestV1`).
- NOT verified: Rust compile of `semio-os-mcp` (cargo check cancelled by disk pressure after 5 h), and no real component publication was run end to end.

## Coordination with publish-chain.sh
- In-flight drivers run the old code and publish to the old `components/<key>` path; that stays untouched and consistent for them.
- New code reads only `component-models`: every plugin (norm, energy, eurocode, structural, materials, plus all others used by MCP/playground/native) must be republished with the new code. The Nx component target hash includes the driver import closure, so it will rerun. Chain steps started before this change must be restarted to publish in the new layout, and `verify-play-components.mjs` expects the new layout.
- The old `.cache/b/p/m/browser/*/components` directories are obsolete; remove them after the old-code chain and the old MCP (pid 26427) have stopped.
- Also: `publish-chain.sh` RETRY regex still lacks `Original resource loan deadline exhausted`.
- Playground-session and registry artifacts are fixed paths too; they were not changed.

## Follow-up (coordinator request)
- Rust: `repo capacity run --memory=3400 --cpu=1 -- repo cargo --store=shared --configuration=prototype --parallelism=1 -- check -p semio-os-mcp --lib` finished (prototype profile, 17m 30s, 0 errors; only pre-existing warnings). The manifest-root change in `plugin-paths-and-descriptor/🦀️.rs` compiles. The `cold-image.rs` test edit was not compiled (check was `--lib`).
- Registry/playground-session artifacts: not changed. All their readers (activation, readiness, preparation, native playground check) are bounded blocks released before any long serve; only the MCP component hold lived for hours.
- Deleted the obsolete `.cache/b/p/m/browser/prototype/components` (1 GB; no lease held, `repo leases status`). `.cache/b/r/m/components` is NOT the old layout: it is the live compiler run/telemetry store (8.7 GB) the chain writes to, so it was not deleted; remove it only after the chain ends if wanted.

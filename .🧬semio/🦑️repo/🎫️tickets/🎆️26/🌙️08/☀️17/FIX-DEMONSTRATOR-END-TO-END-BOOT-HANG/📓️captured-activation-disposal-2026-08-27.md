# Captured Activation Disposal

## Executed Boundary

The shared activation fixture now declares four moved/released route cases, with and without an initial transport refusal. Strict Ajv independently validates both the fixture and the observed result against each expected output.

Actual RED1: 2 failed, 8 passed, 98 skipped / 108 collected, eight files, 688 ms, start 21:17:24. Both newly authored tests reached the absent captured-owner disposal method. RED2 moved the destination assertion before that call and reproduced the original routing defect: original worker received zero disposal posts instead of one. RED2: 2 failed, 8 passed, 98 skipped / 108, 890 ms, start 21:17:46.

Actual full actor GREEN: 108/108, eight files, 1.25 s, start 21:18:26, exit 0. Logs: 🧪️actor-captured-disposal-red-1.log, 🧪️actor-captured-disposal-red-2.log, 🧪️actor-captured-disposal-green-1.log. Renderer strict exited 1 with exactly the seven existing tutorial diagnostics; no new disposal diagnostic. Log: 🧪️actor-captured-disposal-strict-1.log.

`ShardClient.dispose` captures the existing activation before inspecting routing. Its private disposal path targets that activation's original slot and exact generation. Transport refusal leaves the activation owned and blocks reactivation; retry uses the same authority. Routing cleanup happens only after a successful post. The lifecycle lease additionally exposes `dispose()` bound to its original activation, requires completed instance close, and refuses a lost/replaced worker. A repeated old lease call after a successful post cannot discharge a same-name replacement. The post marker means transport submission only, not worker acknowledgement or native/Wasm memory release.

The existing non-instance disposal path still performs synchronous effect cancellation and pending-request rejection. This packet does not certify their boundedness. The captured instance path first joins its existing retained cancellation, guest close and host UI witness. Strong raw-return ownership and fresh guest execution remain separate obligations.

## Source Hashes

- ShardClient: e0a3ef816cbebebe8a76f750c5dd8f4aec5763e7f558c5490e7f97dadec287ea.
- Activation fixture: e404947d2a2734ee6efc8fccf78dfdada02daf25cb3e86349263e77450b3a6e7.
- Fixture schema: 5b16ebd304630afe4db0e8363fb956435992026ef348b362c322a783756795c1.

## Generated Module Ownership Review

The actual dev output root is `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules`, derived from the current dev script, not the absent historical public/plugin-modules directory.

Read-only GIS output evidence: `gis/semio_s_plugin_gis_component.js`, SHA256 dc0c69df1341653a0c34ab2493d51acce71fbee9d97d11e08ba2167095fd16a6, contains module-scope exports1/memory0 at lines 5712–5713, module-scope initialization at 11918 and awaited initialization at 12108, then exported reactor closures at 12125–12130. The existing generated host shim also owns a module-scope pendingEffects Map. This is an older published binary/loader snapshot, not current lifecycle output acceptance.

Current materializer SHA256 a246d95516306aa6fdbfb32bcaf8bdf825c685bc20f12eeb09eaa7af5b4c1d5c creates activation-specific dynamic component and host-shim URLs. Its worker disposal only deletes actor, in-flight and budget map entries. That branch does not clear the generated module's memory/export roots. Therefore map deletion is not a demonstrated component-memory release mechanism. No heap experiment or measured leak claim is made here.

Installed producer inspection confirms support, not an executed factory result: `node_modules/@bytecodealliance/jco/dist/jco.js:89` accepts `--instantiation async|sync`; `jco-transpile/dist/transpile.d.ts:12` explicitly distinguishes automatic module instantiation from caller-supplied imports. The required direction is one cacheable factory module, per-activation imports and instance closures, with release only after canonical descendants retire. This must retain one WIT ABI and coordinate a catalog rebuild; no generated output, factory cutover or pin has been changed by this review.

All active evidence, inputs and caches remain retained. No cleanup or deletion was performed. The all-six demonstrator runtime acceptance remains outstanding.

## Read-Only Dependency Check During Inbox Hold

After the neutral raw-core R16 notice, this lane re-read the materializer, the existing PluginComponentInstantiation test and its producer callers without modifying them. The production synchronous and asynchronous transpile invocations still omit explicit instantiation. createActorApi still imports activation-specific component and host-shim module URLs, and hostShimSource still places pendingEffects at module scope. The existing executed factory test demonstrates two independent memories from one cached explicit jco factory, but it does not mount that mode in either production transpile path.

A coherent eventual source cutover must join both transpile invocations, cacheable component factory loading and per-activation host imports/effect ownership; changing only the jco flag or deleting actor map entries is insufficient. Canonical close and original activation authority remain prerequisites for releasing those instance closures. No producer edits, rebuilt catalogue, shared generated files, WGPU pin or component-memory release claim arose from this read-only check. The current critical metadata/inbox hold remains separate.

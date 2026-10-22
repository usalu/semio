# Bootstrap Require Witness Repair

Actual focused Native80593 exited1 at18:50:40Z with signal:null and no cancellation before collecting tests. Its frozen log is `🗑️generated/2026-10-21-cjs-focused-native.log`,467304bytes, SHA-256 `6f3be80bdda6ab502d3ff577651259a24ee1317a12d1fed6964d2e6f4260dd6f`. The primary was `Captured require declaring module required` at the bootstrap parser witness. The current real CommonJS regression remains unrun.

The actual caller used `createRequire(join(root,"package.json"))` only for `require.resolve` on TypeScript and TOML package manifests/entries. That package manifest is DATA rather than a captured declaring module. Changing only the base to a physical module would still leave undeclared installed parser resolutions. The captured implementation guard is retained without a fallback or general external resolver.

The peer runtime repair instead inspects finite installed package DATA under the dependency generation target and binding already held by the bootstrap witness. Package directories derive from the bound target and fixed parser name; an inert manifest projection selects the schema-declared entry. Native store confinement, realpath/ordinary-file checks, exact lock version, bounded full-package hash walk and current dependency receipts remain the authority. Installed parser code is not executed by this inspection. The TypeScript5.9.3 material declares `./lib/typescript.js`; TOML2.2.5 declares `toml.js`. Cargo's read-only physical observations are retained in `2026-10-21-parser-material-inspection.md`.

The existing peer resolution laws supply closed language-neutral entry/physical cases, strict Ajv validation, an independent SQLite acceptance oracle, structural bootstrap custody assertions and a real compiler-captured helper comparison with the native package resolution oracle. They are preserved. The actual stream-loader contract sets `import.meta.main:false` and invokes exported `runCapturedBuildSourceV1`; its export must delegate once to local `runScriptProgram(args)`, whose body receives the genuine Original. The bare zero-argument guard remains for ordinary entry, and is not claimed to execute under this loader. The material result DTO remains stable.

The final narrow regression correction binds physical Source URLs, projects the actual child control once, verifies the received Original owner/ceiling/deadline against that exact launch envelope, and gives the closed DATA law an explicit finite ceiling. No duplicate feature or generic loader change is intended. Source edits were coordinated against current peer work rather than reconstructed from stale long-line snippets.

Selected descriptor: `closed installed parser entries match SQLite without executable resolution`25000ms; `actual captured parser witness uses closed entries matching native resolution`75000ms. Sum100000ms plus60000ms margin gives160000ms phase; shared CLI timeout75000ms. Root alone adopts metadata and the phase. Final nofollow pins, schema/AST checks and exact current descriptor are supplied by the independent read-only review because this agent's Node read backend stalled. That tooling stall did not authorize a runtime reset, Source evaluation, CLI or Native action.

No actual tests, parser code, compiler child, Native retry, generic capture relaxation, git mutation, common-ledger update, artifact or course/build completion is claimed here. The next Root-authorized Native check must verify the repaired real startup and collected laws.

Final independent parser-specific handoff is `🗑️generated/bootstrap-require-witness-independent-current.json`. It retains nine current nofollow FD/name pins, closed schema validation and exact selection. Root completed the guarded capability patch against the current peer file: the real exported entry delegates to the receiver-owning program; the received Original birth record is checked against the single projected control environment while the material result DTO stays unchanged. Static generated-entry AST diagnostics are0; strict Ajv accepts18 material/physical cases and rejects the authored invalid birth records. These checks do not execute the Source or tests.

The independent review separately detected concurrent loader boundary drift at27974/c319d100: the general import guard no longer included `||!declared&&candidate!==entry`. That drift is outside this parser-specific repair and was reported to Root. This report does not claim that the loader is unchanged or that the complete future Native candidate is eligible; Root must resolve the separate boundary review before execution.

The independent audit is `2026-10-21-bootstrap-require-witness-independent-review.md`. Its exact current pins (full paths and identity fences are in the generated JSON) are:

- Bootstrap owner:17442/4963e14c9a3aa42ede7b04e0a2597f20dbf3714b3714a10c7c7d9ebb30a4c45f.
- Parser material owner:3474/b0b132e8b9ed8773027704d92047ca61b7cc1fe9be7246ef1f83629fab762d99.
- Parser schema:6848/5017e3891dc6ea7f183a35c7ab9b9b7853ca9af0f0e6022e449dca75c10d225a.
- Parser main tests:8086/d693a8d7c8ad5e631dd791402019bc700fdfbe958bcd914433682b777b13fe82.
- Resolution tests:12005/2edf426e117afab0c68d0c9504f6051689d91e64ac46c58894ac706120a3f207.
- Resolution corpus:3549/135bf7dc63716814fe5b9e344e6d422521668116e0daae0a375cd7b5274c0b85.
- Build main tests:7038/67e5350f211c19de3b65c5229f8b68f24af2d5ba05bedf44b6057e664eaf73ff.
- Actual compiler owner:2951/e9ba26884ccfd1c4fc8144fb1a67fcddbbaba0a1c7d7510da83e60526100d07c.
- Separately reviewed loader context:27974/c319d10068d2e36d5cf088426f1bfa0cd135fef22bea71a579e396ea4d16f9f3.

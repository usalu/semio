# Independent Declared Verdict Owner Source Audit

Verdict: bounded immutable-owner Source extraction accepted at the supplied freeze, with explicit currentness qualification. No Rust build, runtime execution, Cargo resolution, or global lock closure is established. The existing Framework lock and recursive pin frontier remain blockers.

## Scope and Identity

Read-only Source audit, with this report the sole authored file. No Source edit, Git mutation/worktree, Rust/Cargo/metadata/native/compiler/SDK/provider/network action, or ticket lifecycle change. Repo MCP unavailable in delegated context. Parent retains lifecycle. Supplied freeze is exactly 176260 bytes, SHA256 `25aa41493c7b1a5023f180e8b1019e323019702b5412ccf1de178f68f1764b84`; all 836 listed identities were independently read and hashed. Logs, before/after membership, census/neutral fixtures, owner/test Source and lock identities are present and match except the shared drift listed below. No drift attribution is inferred from its timing.

## Actual Ownership and Preservation

Inspected actual new lib and manifest: semio-framework-plugin-declared-verdict resolves its lib to ../../🦀️.rs and owns immutable policy/observation definitions. Its five runtime dependencies are framework, pack-json, plugin-ownership, schema and ui-locale, with existing serde_json 1.0.140 dev requirement. Actual testing-laws manifest resolves to its own ../../🦀️.rs and directly consumes the new package; wp7 mounts testing-laws as an extern crate. Facade manifest has no direct new verdict edge. This is an actual lower owner, not an unused mirrored package.

The freeze-matching test Source uses tree-sitter to compare 19 AST definition hashes against the neutral census, normalizing only the two pure function visibility prefixes, and compares all 147 remaining parent function hashes in order. Independent audit confirmed census cardinalities and full original three-law source SHA256. The retained laws have 3+2+2 assertions and exact named hashes recorded in the freeze. Physical test source is retained and included under cfg(test) by the new owner. AJV independently evaluates neutral verdict/bridge schemas; SQLite compares actual/expected definition tuples in both EXCEPT directions. These are meaningful Source preservation/schema oracles, not Rust execution evidence.

Inspected Source keeps declared_verb_outcomes_agree private; the only newly public functions are pure argument representative helpers. The new owner has no VcsArtifactApp, action execution, StepContext or mutable store/history/job entry. Canonical bounded_ownership failure receipt types remain. No private execution authority widening found in this extraction.

## Lock Facts

Independently parsed literal package/pin fields from all 174 fixture-listed lockfiles with Python standard library and verified every pre-recorded thirdparty name/version/source/checksum digest and exact five-edge new local verdict entry. This independently corroborates unchanged pins and authored edges, without pretending to resolve Cargo. An initial diagnostic attempted Python tomllib, unavailable in this Python; it performed no validation and was replaced by narrow literal lock parsing. Existing test Source additionally checks canonical ownership/caller arrays, consumer presence and exact dependencies. There are 112 present and 62 absent testing-laws parent entries; the latter remain missing.

The 175th tracked Framework/Cargo.lock is genuinely consumed through package.workspace and actual workspace membership, absent from the narrower exact-facade roster, and explicitly remains untouched and blocked. Independently recounted archived graph: 362 owners, 175 roots, 167 roots with unresolved registry candidates; Framework has 159 missing firstparty entries and 6 unresolved registry edges. This is a conservative Source frontier, not selected or repaired versions. Graph currentness is now qualified by the manifest drift below, so these exact counts describe the supplied freeze. No pin resolution claim is justified.

## Closures and Attribution

Independently summed the before/after identity manifests; after memberships match their frozen hashes. Declared-verdict tests are 2 files/31573 bytes/427 nonblank; testing-laws full literal closure is 2/187299/2282. Facade base is 14/163541/2502 and feature union 19/218160/2695. Facade tests remain 235/3783654/42111 and oversized. Original builder contract/time-travel entry paths remain outstanding. Large facade reductions are attributed by the freeze to peer builder/testing/progress extraction; this lane owns only the declared verdict extraction. Bounds exclude generated/nonliteral mounts, expansion, and imported crates' separate Source.

## Validation Evidence and Limits

All 22 terminal-record log hashes match the frozen inventory. The final prototype 67024 and release 21060 logs visibly record 6 pass/4233 assertions plus adapter preflight 10 pass/216 assertions and explicit DEBUG RustUncompiled=true. Genuine semantic RED logs 17907 and 89484 distinguish missing owner and missing parent dependency from earlier infrastructure/routing failures. Strict 73359 and 13279 have empty matching logs and exit=0 recorded in plugin-feature-terminals.json; empty files alone do not prove execution. Audit did not independently rerun commands or have a direct PTY completion receipt, so strict success is corroborated saved terminal provenance, not fresh independent execution. Inspected wrapper/test/editor evidence documents strict/noUncheckedIndexedAccess, selected Source root and cache-disabling environment; those shared wrapper/catalog identities have since drifted. No conclusion here extends historical selected checks to that newer epoch.

## Shared Drift at Audit Read

The following identities differ from the immutable freeze. All other listed identities matched at this read. Parent is separately changing observation cache/wrapper; this audit neither overwrites nor assigns authorship to these changes. The store manifest drift means recursive graph currentness must be refreshed before using it as a current whole-graph fact.

| Path | Current Bytes | Current SHA256 |
|---|---:|---|
| .vscode/launch.json | 8766417 | c7c8e8fd007a3fce2106273b5da37ab73ee12970eab3b5e15acf803429d96bb8 |
| .vscode/🧩️launch.seed.jsonc | 6923932 | 096d70b90aaded8389fbd0e275f79cf44b3008e00954f28d7524c7bcd3432b84 |
| .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts | 101026 | dd606b4b0b0342bcf970fa561ba44b6a9431fffa21aae3f977432c871e039fc7 |
| Cargo.lock | 597050 | 43d31a93be1b83bc2c5de38e8ac141bab11764a5674a5275cbfad50671512504 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📦️packages/🦀️rust/Cargo.toml | 3622 | d4947e14c846d17f6c25e2d101affb9aebec210757b6d23668c1a8beb4b137d5 |

# Clone source, factory, shared issuance, pack format and OS swap crates

Commands: `stale-cargo.sh <label> test --offline --manifest-path 🧰️framework/Cargo.toml -p <crate> ...` (shared build dir, lock serialised).

## Status

| Target | Before | After |
|---|---|---|
| `semio-framework-value-shared-issuance` lib | did not compile | 17/17 |
| `semio-framework-pack-format` lib | 2 failures | 29/29 |
| `semio-framework-value-clone-source` lib | did not compile | 20/20 |
| `semio-framework-value-clone-source` `projection` | not run | 16/16 |
| `semio-framework-value-factory` `original_admission` | did not compile | ported, 3/3 |
| `semio-framework-value-factory` lib tests | did not compile | still stale (Open) |
| `semio-plugin-host-shard` | not run | 2/2 original shard wire laws |
| `semio-plugin-window-config` | not run | `apply` 6/6, `genesis` 1/5 (job fixture, Open) |
| `semio-os-infinite-board-engine` | not run | builds, no test in the paint cache |
| `semio-framework-replication` canonical seal | not run | `paged_seal_digest` 2/2 after the digest fix; `retained_diff_sequence` 0/3 (json work quote, Open) |
| `semio-s-fem-material` | not run | blocked offline (Open) |

## Fixes and ports

### shared-issuance (`♻️retirement/🔗️arc-custody/🔒️sealed`)
- Dev-dependency on `semio-framework-value-clone-source-testing`; `OriginalRetirementPoll` imported for the shared fixture effect.
- `admission_grant` funds `source_admission_demand`, the quote `admit_from` needs.
- Test-only `AdmitFixture::admit` runs one real `admit_from` turn and returns the input with the refusal kind (items 0 `WorkLimit`, depth `DepthLimit`, else `OwnershipLimit`).
- `return_borrowed_alias` and `ControlledRetirement::step(body, effect)` run through `effect::run` / `effect::drove` (the latter carries a genuine producer invocation, required by native retirement).
- Frontier tests: `close_into` replaces the removed `close_step`; steps run through `drive_reply`; clearing the emptied original owner is its own paid step; the extraction work equals `next_computational_work`; the election quote copies the 48-byte header.
- Fixture `birthWork` 4 to 5 (`source_admission_work_demand`).

### pack-format
Both failures: codec 1 is deflate and `semio-framework-replication-codec/deflate` was only enabled by workspace feature unification. The package now has a dev-dependency on the codec with `features = ["deflate"]`.

### clone-source
- `🧪️law-bank` rebuilt on the current model: one persistent original Work per thread (`EffectBank`), every effect carries a producer invocation and the caller allocation marker; `Bank::source_traced` drives the 17 birth turns including the projection frame; `advance_to_ready`; `Bank::take` / `drive_take` (reply-shaped handback); `retain` / `retain_denied` (the two separately funded owned-projection turns); `Bank::reply`.
- Ported tests: custody (3), header, query (2), retain (2, funded-before-accessor law through `project_paid`), admission (6), work (3, birth checked stage by stage against fixture `birthStages` and `birthCpu` 37), clone close (3, owners declare `native_header_release`).
- Field-projecting test types implement `OriginalSourceProjection` (`Captured`, `Pair`).
- Fixtures: header `headerAdmission.turns` 6 to 17, admission `allocatingTurns` (21 turns); both with their schemas.
- Removed the duplicate `#[cfg(test)]` macro re-export in the package root.
- `drive::fresh` (`🧪️testing`) primes the bank with one empty turn so the first measured turn no longer carries the bank's lazy allocation (it showed as `(392, 0)` in the seal tests).

### factory `original_admission`
`FactoryAuthority::step` and the ticket traits take body and effect and return polls; the target runs on `EffectBank`. The unwind laws keep one bank across the unwound turn and its recovery turn: recovery reports 0 work, the wallet shows exactly 1 unit spent in total, and the recorded marker (8 bytes) is handed back with the receipt. The closing helper drives `ControlledRetirement` through `clone-source-testing::drive` (invocation) and the test ticket declares `native_header_release`.

## Product finding: canonical seal digest
`ArtifactCanonicalEditSealCursor::advance` (multi-step loop for wide work turns, being edited concurrently by someone else) moved the pass to its next phase in the same turn the tree finished, so the buffered counting page was hashed before the header frame (`19375af5...` instead of `4fa63412...` for row `sparse`) and the second-pass length check ran before the last page was flushed. Fix: leave the finished tree for the next turn as before and only set `hash_pending` when the finished pass is not the counting pass. A python recomputation of the `semio.artifact.cursor.v2` frame over the fixture edits reproduces the fixture digests for both rows (`4fa63412...`, `cf8e2ac0...`).

## Open
- Seal: while another agent's `[DEBUG]` taps (an `eprintln!` in the seal turn and an allocation tap in `⏱️trace/🧮️memory`) were live, `drive` heap checks showed noise (`(653, 285)` vs `(0, 0)`); with the taps removed both seal tests pass, including the wide-work law.
- Custody frontier law: the handback now stops with a zero-progress election turn (work 1) instead of an error, and delivering the original takes several turns; the election quote is the header copy (`Option<Self>` + `Option<storage>` + `Option<SharedOriginal>` + `AtomicUsize`, 112 bytes) without a release currency, so the test denies copy instead of release and loops until the output arrives.
- Factory lib tests (seven modules, about 86 KB, 158 errors): `📦️owned`, `📦️boxed`, `📋️vector`, `🚼️construction`, `🧪️tests`, `arc-custody/🏭️factory/🧪️tests` and `🔐️custody` use removed APIs (`preborn_factory_retirement`, `retirement_admission().admit`, `FactorySharedRetirement::admit` / `admit_original`, `FactoryAuthority::ticket`, `FactoryIssuerIdentity`, old `FactoryPayloadRetirement` step signatures). They need a rewrite on `FactoryChildSlot` admission (reply-shaped); not started.
- window-config `genesis` (4 failures: catalog header transfer not funded, shared UTF8 lease needs one item, captured shared original changed Work) use `OriginalJobFixtureCaller` from `🧵️job` (other agent); the epoch swap is covered by the passing `apply` target.
- `semio-framework-replication` `retained_diff_sequence` (3): `from_json_str` refuses with "JSON parse work quote was not admitted" (json owner).
- `semio-s-fem-material`: the `✏️s` workspace lock pins `either 1.15.0`, absent from the offline registry cache (only 1.19.0), so it cannot be built here. Its diff is the one-line `effect.nested` swap plus `advance_pending`.
- clone-source `🧪️tests/🔬️unit` and `🏭️factory` are not referenced by the library and were left untouched.

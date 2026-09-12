# Rust Generator And Oracle Audit

## Scope And Method

This was a bounded, read-only audit of the Rust generator, fixture, and stdio-oracle lane. It reviewed the execution packet at 📓️rust-generator-audit-packet-2026-09-12.md and the executor report at 📓️sol-rust-generators-2026-09-12.md. No authored source, schema, configuration, Cargo manifest, or generated fixture was changed.

The executor's covered set contains 46 Cargo manifest roots: 45 generator or probe packages and the stdio oracle. Its source/reference census is 235 Rust sources and 441 Rust referents. This audit independently parsed every current manifest with Cargo and exercised one registered native fixture route, the portable taxonomy fixture, and the stdio oracle.

## Accepted Current Evidence

| Check | Result | Evidence |
| --- | --- | --- |
| Cargo manifest and source resolution | 46 of 46 manifests parsed; 83 declared targets; 0 target-path or package-name problems | 🗑️generated/terra-rust-generator/cargo-metadata/summary.json |
| Portable kind-only taxonomy contract | Pass: 7 tests, 342 expectations | NX target '@semio-tech/repo-lib:test-kind-only-basename', 2026-09-12 |
| Representative native generator route | Pass: 7 tests, 0 failures | NX target 'gif-89a-any-fixture-generator:test', 2026-09-12 |
| Schema and representative scope inventory | Schema has 0 violations; Sequence generator 26/0, PNG generator 20/0, stdio Rust owner 7/0 entries/violations | 🗑️generated/terra-rust-generator/taxonomy-scopes.json |

Cargo metadata is an independent Cargo/TOML oracle. It confirms that each package name remains unique, every declared target has a '🦀️.rs' source, and each resolved target source is outside the literal '📦️packages/🦀️rust' manifest owner. This is the expected anonymous-leaf layout: package owners hold Cargo metadata while semantic artifact or oracle owners hold implementation leaves.

An active-code scan found no current '🏗️generator' or 'generator-crate' package coordinate in Cargo manifests, scripts, JSON, or Rust. References containing '/src/' were frozen explanatory JSON rationale about third-party Cargo registry paths, rather than active local package links.

## Actionable Oracle Defect

The registered target 'semio-s-plugin-stdio-test-oracle:test' currently runs 33 tests and fails exactly one:

- 'artifacts::dwg::standards::v_ac1024::subsets::any::component::tests::kinds_match_both_catalogs_and_the_vocabulary' at ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️unit/🦀️.rs:150.
- The assertion reports: 'a committed DWG catalog is missing kind "no-mutation"'.
- Neither current catalog contains that required kind: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🔮️oracles/🔣️.json and the corresponding 4️⃣ac1018 catalog.
- The current DWG source declares and implements 'no-mutation' at ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🔮️oracles/🦀️.rs:54, :254, and :278.

The responsible owner should reconcile the two committed DWG oracle catalogs with the source vocabulary, including the 'no-mutation' entry. This is a live catalog-contract defect, not an effect of moving Rust package metadata: the failing test reads semantic standard/subset catalog owners and all Rust package metadata resolved before the test began.

The full rerun transcript is 🗑️generated/terra-rust-generator/stdio-rerun.log: 32 passed and 1 failed.

## Current Strict-Statute Snapshot

The current target '@semio-tech/repo-lib:test-path-emoji-statutes' has 31 passing and 6 failing tests (1,138 expectations). This is a point-in-time shared-workspace observation, not a baseline comparison. None identifies the Rust manifest/source relocation as its cause.

| Failing check | Direct observation | Scope |
| --- | --- | --- |
| mutation pair catalog normalization | Expected 'create-camera' and 'create-node'; current result is empty | shared catalog projection |
| glTF fixture-manifest roles | Test opens singular '🔮️oracle/🔣️.json'; live owner is plural '🔮️oracles/🔣️.json' | stale test coordinate |
| Rust icon source/public identity | Exact-string expectation omits '../' from the generated include path | stale relative-path assertion |
| TSV mutation payload schemas | Test opens singular '🔮️oracle/🔣️.json' | stale test coordinate |
| projected scenarios single-emoji identities | '🔌️disconnect' is currently classified as present | shared taxonomy projection |
| Storybook semantic suffix discovery | expected '🪗️.story.tsx' is absent | Storybook lane was being edited concurrently |

The complete transcript is 🗑️generated/terra-rust-generator/path-statutes-rerun.log. These failures should be assigned to their owning catalog, projection, test-coordinate, or Storybook lanes rather than repaired as part of the Rust generator move.

## Limits

This audit did not regenerate committed fixtures, because that would write into a concurrently shared workspace. It also did not recompute deleted pre-move file hashes; instead it independently validated the present Cargo graph, target identities, taxonomy authority, and native runtime behavior. The passing GIF route is representative, not a claim that every generator output was re-executed.

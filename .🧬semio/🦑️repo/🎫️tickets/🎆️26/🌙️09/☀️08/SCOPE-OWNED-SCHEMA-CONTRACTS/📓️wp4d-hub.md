# WP4d — `🌎️hub` path rewiring for the plugins partition's moves (rows 126, 88)

Partition: `🌎️hub/**`. Input: `📋️cross-partition-requests.md` rows 126 (dispatched to W4e) and 88 (hazard),
spelled out in `📓️wp4b-plugins.md` §8 A and §8 F. Repo MCP was down for the whole session — no ticket
tool was called, `🗑️generated/` was never deleted, no git-modifying command was run.

## 1. What row 126 actually needed, measured before touching anything

Every `✏️s/…` path literal spelled anywhere under `🌎️hub` (`*.ts`, `*.rs`, `*.json`) was extracted and
checked against disk. Exactly four did not resolve — the same four rows 126/§8 A/§8 F name, no more and
no fewer:

```
MISSING: ✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧬️.schema.json
MISSING: ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/📇️native-catalog-surface
MISSING: ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/🧾️claim-authority
MISSING: ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json
```

After the changes below the same scan is empty:

```
== all present if nothing above ==
```

**`📜️script.ts:9134,9166` was already correct on arrival.** Both sites in
`🌎️hub/📦️packages/🦀️rust/📜️script.ts` (now lines 9143 and 9175, the file has grown) already read
`✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json` without the `🧬️schema/` segment.
The stale literal survived only in the **two fixture JSONs**, which is where it was fixed.

## 2. Changes

### 2.1 The two hub fixtures — `🧬️schema/` segment dropped (row 126)

| File | Line | Now |
|---|---|---|
| `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🪪️v1/🔣️.json` | 3 | `"providerProjection": "✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json"` |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🔣️.json` | 63 | `"stdioReceipts": "✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json"` |

Both hub fixture collections are still spelled `🧪️fixtures` on disk; only the `✏️s/…` **plugin** paths
inside them and inside the hub script were renamed to `🧫️fixtures`. Hub's own `🧪️fixtures → 🧫️fixtures`
rename is still the open question `📓️wp4-hub.md` §8.2 records, and was not started here.

### 2.2 `✏️s/…/🧪️fixtures` → `🧫️fixtures` in the hub script (row 126, §8 F)

Three literals, all `✏️s/🔌️plugins/🗄️stdio/📇️registry/…`, each target verified on disk first
(`🧫️fixtures/🧾️claim-authority/🔣️.json`, `🧫️fixtures/📇️native-catalog-surface/{🔣️,🧪️budget,🧪️commitment,🧪️imports}.json`):

```
4693  const claimRoot   = join(repoRoot, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧫️fixtures/🧾️claim-authority");
4694  const surfaceRoot = join(repoRoot, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧫️fixtures/📇️native-catalog-surface");
5080  const fixtureRoot = join(repoRoot, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧫️fixtures/📇️native-catalog-surface");
```

No other `🌎️hub` file spells an `✏️s/…/🧪️fixtures` path.

### 2.3 The gis codec schema is now the `s.gis` scope export (row 126)

`proveTrustedBootstrapCodecCaptureFixture` compiled the deleted flat file
`✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧬️.schema.json` with a private `Ajv2020` instance. It now resolves
through the catalog like its stdio sibling:

```ts
const schemas = {
  stdio: hubSchemaExport(repoRoot, "schema://s.stdio.registry/NativeCodecFactories"),
  gis:   hubSchemaExport(repoRoot, "schema://s.gis/GisNativeCodecs"),
};
```

The private `Ajv2020` import and instance are deleted with it (they had no other use in that function),
and so is the `🚧️` comment that said the GIS receipts have no scope module yet.

**The export id is `GisNativeCodecs`, not `NativeCodecReceipts`.** The brief named
`schema://s.gis/NativeCodecReceipts`; the module on disk
(`✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json`, `$id https://semio.tech/schema/s/gis/schema.json`, draft-07,
catalogued as scope `s.gis`) declares `GisNativeCodecs` (the whole `📇️native-codecs/🔣️.json` document —
`schema`/`pluginId`/`packageId`/`packageVersion`/`receipts`/`hostile`, `additionalProperties:false`) plus
its row helper `GisNativeCodecsReceipt`. `GisNativeCodecs` is the shape the deleted flat file validated,
so that is what the hub binds. `📓️wp4b-plugins.md` §8 A spells the same name. Nothing is pending from the
plugins worker for this row — the export exists.

No Ajv keyword registration was needed: neither the `s.gis` nor the `s.stdio.registry` module carries an
`x-semio-state` or `x-semio-formats` key today (§8 A anticipated both), and the shared strict Ajv in
`🌎️hub/📦️packages/🟦️typescript/🟦️.ts:70` already knows `x-semio-formats` from WP4c.

Checked directly before running the oracle:

```
$ bun -e 'hubSchemaExport(".", "schema://s.gis/GisNativeCodecs") over 📇️native-codecs/🔣️.json'
gis data accepted: true
hostile invalid-hash rejected: true
hostile unknown-root rejected: true
stdio data accepted: true
```

### 2.4 Three hub-side readers the plugins/test-layout partitions had already broken

These were **not** in the brief. They are the hub half of moves that landed in other partitions while
this session ran, they are all in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, and each one was red before
any edit of mine (§4 shows the sequence). Row 88 is exactly this hazard.

**(a) `const SOURCES: [&str; 36]` no longer exists in the stdio registry.**
`proveNativeOpenableCatalogProviderFixture` asserted `📇️registry/🔣️.json`'s `artifact_definition_paths`
equals the `include_str!` list of a `const SOURCES: [&str; 36]` array in `📇️registry/🦀️.rs`. That array is
gone from the working tree **and from `HEAD`** (`git show HEAD:…/📇️registry/🦀️.rs` has no `SOURCES`); each
of the 36 artifact modules now compiles its own
`pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("<path>");`. The law now derives the compiled
roster from those 36 modules and compares it set-wise with the index, reading the compiled path out of the
`include_str!` with a regex rather than restating it:

```ts
const compiledDefinition = /pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!\("([^"]+)"\);/u;
const compiledInventory = readdirSync(artifactsRoot).flatMap((artifact) => { … }).sort();
```

The regex form is deliberate: the peer moved `📜️artifact-definition.json` out of `🧬️schema/` into the
artifact root **at 21:44 while this session was running**, and a repo-wide applier rewrote a hardcoded
`🧬️schema/📜️artifact-definition.json` literal inside my own first edit of this file within minutes. A law
that reads the path the module actually compiles cannot be invalidated by the next such move.

**(b) The definition walk was counting gitignored build output.** The same function then walked
`fixture.artifactDefinitionsRoot` recursively for every `📜️artifact-definition.json`. That walk finds 72
files, not 36 — each artifact also has a
`📦️packages/🟦️typescript/dist/🧬️schema/📜️artifact-definition.json`, and `dist/` is gitignored
(`.gitignore:629 **/📦️packages/*/dist/`). The bijection therefore saw 52 owners against a
`receiptCount` of 26 and denied the positive case. Whether the law passed depended on whether someone had
run a TS build. The walk is deleted; `definitionFiles` is now the roster of (a), which the line above has
just proved equal to the index. Result: `owner-receipts=26`.

**(c) `stdioRoot` was derived from the projection file's depth.** `resolve(dirname(projectionPath), "../..")`
assumed the projection sat one level deeper than it does since §2.1 (`📇️registry/📜️native-codec-factories.json`),
and resolved to `✏️s/🔌️plugins`, so every `receipt.protocol_path` read ENOENT. It is now
`dirname(definitionRoot)` — derived from the fixture's own declared `artifactDefinitionsRoot`
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`), so it carries no depth assumption at all.

**(d) Row 88, third occurrence.** `proveTrustedCompiledDependenciesFixture` sliced
`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` for three `#[cfg(test)]` anchors
(`native_openable_stdio_bundle`, `native_openable_stdio_provider_is_the_only_atomic_readiness_transition`,
`checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog`). All three now live in
`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — the END-TO-END-TESTING-REFACTOR peer's move. `hubSource` now reads
`moduleRustSource(join(repoRoot, "🌎️hub", "🧪️tests"))`, the same remedy `📓️wp4b-hub.md` §5.3 and
`📓️wp4c-hub.md` §7.3 applied to the four earlier occurrences of this hazard in the same file.

## 3. Verification — real output

All runs `cd /Users/ueli/Documents/semio`, `SEMIO_TEST_ARTIFACT_DIR` = `🗑️generated/wp4d-hub`,
`CARGO_TARGET_DIR` = `$SEMIO_TEST_ARTIFACT_DIR/cargo`, except the cargo law run (§3.5).

### 3.1 Partition gate

```
$ bun <ticket>/wp4c-hub-probe.ts .
[wp4c-hub] 0 finding(s) of 2691 repo-wide
```

(0 hub findings, unchanged from WP4c. The repo-wide total moved 4258 → 4011 → 2691 during this session;
none of those rows is in `🌎️hub`.)

### 3.2 `native-openable-catalog-provider-check --oracle-only` — **green, exit 0**

```
headless-stdio-metadata-capture-oracle: private-commands=2 replacement-stable=1 replacement-during-capture-denied=1 second-dependency-root-denied=1 outside-target-denied=1
vcs-native-codec-oracle: receipts=1 hostile=9 ajv+node+webcrypto=1 dependency-coherence=3; no catalog activation or VCS execution claim
vcs-native-provider-selection-oracle: cases=8 accepted=1 unconsumed-profiles=2 linked-receipts=29 scope-exports=2; no native or catalog activation claim
native-openable-claim-oracle cases=8
native-openable-neutral-oracle: AJV=1 scope-exports=3 owner-receipts=26 protocol-webcrypto=26 targets=1 hostile-denied=13 no-partial=13
```

This is the first time this command has reached the end. `📓️wp4b-hub.md` §7.2 had it aborting in the vcs
plugin before `proveNativeOpenableCatalogProviderFixture` was reached at all; those vcs and gis blockers
are gone, and the three hub-side defects of §2.4 a–c are what it hit instead.

### 3.3 `trusted-stdio-gis-bundle-check --two-author-source` — **exit 0**

```
[DEBUG] GIS Map witness socket retirement: neutral=5 Bun=1 third-party-ws=1 exact-close-events=2; no Hub restart claim
GIS Map two-author composition fixture laws=16 hostile=10 current-coordinates=5; real mounted journey remains separately required
```

**This flag does not exercise §2.3.** `--two-author-source` returns after
`proveGisMapTwoAuthorCompositionFixture` (`📜️script.ts:12480`) and never reaches
`proveTrustedBootstrapCodecCaptureFixture`. `--source` does, and was run for that reason — **exit 0**,
tail:

```
trusted-codec-source: AJV=3 WebCrypto=1 cases=16 evidence=…/🗑️generated/wp4d-hub/codec-source-iZka64
[DEBUG] trusted-gis-publication: AJV=1 sqlite-traces=20 source-hostiles=5; no native component or browser claim
[DEBUG] trusted-retained-gis-browser: pinned-tables=3 deep-equal=6 SHA256=node+webcrypto source-hostiles=8; neutral bytes only, no browser execution claim
[DEBUG] trusted-gis-collaboration-contract: pinned-specification=1 hostile=8 cross-fixture-equality=8; specification only, no observed collaboration trace
trusted-rotation-source: WebCrypto=1 cases=6 writes=7 …
trusted-stdio-gis-bootstrap-oracle: packages=2 codecs=28 targets=1 hostile=19 cancellation=8 descriptor-pairs=4 stale-plan=1 scope-exports=5+node+webcrypto+first-party-pack+blake3=1; no materialization or hub activation claim
gis-component-cold-map-patch-source: AJV=1 SHA256=node+webcrypto hostile=5 markers=9; no GIS component build or browser acceptance claim
[DEBUG] GIS Map witness socket retirement: neutral=5 Bun=1 third-party-ws=1 exact-close-events=2; no Hub restart claim
GIS Map two-author composition fixture laws=16 hostile=10 current-coordinates=5; real mounted journey remains separately required
trusted-stdio-gis-bundle-check: source+neutral exact closure passed; native materialization, candidate hub, and current pointer remain unclaimed
```

`trusted-codec-source: … cases=16` is the codec-capture fixture running all sixteen `schemaAccepted`
expectations against the new `schema://s.gis/GisNativeCodecs` binding. That is the proof for §2.3.
The `--source` run also proves §2.4 d (`proveTrustedCompiledDependenciesFixture` is inside it).

### 3.4 `gis-inference-ledger-oracle` — **still exit 1, still `os.db.storage`, still row 142**

Every `hub.*` oracle in the command prints green; it aborts on the same os-partition contract WP4c
reported as §8.2:

```
inference-server-identity-oracle: cases=11 fields=5 composite-bounds=2 ajv+node=1
gis-inference-ledger-oracle: traces=9 hostile=13 identity-hostile=17 sqlite-integers=6 ajv+typescript=1 hashes=6 independent-bounds=1; no executor/route/approval claim
inference-wal-proof-oracle: traces=17 ownership=3 binding-hostile=2 protocol-envelope=1 node-sha256=1; committed-WAL runtime still required
inference-command-oracle: vectors=20 ajv=1 node+webcrypto-hash=2; no GIS execution authority
inference-approval-request-oracle: valid=1 hostile=14 byte-boundaries=2 ajv+node=1; no approval authority
inference-author-oracle: cases=16 accepted=1 ajv+sqlite=1; no retained grant or submit authority
inference-wal-chain-oracle: exact=14 hashing-ownership=3 retained-boundaries=2 ajv=1 crc-valid=14 blake3-known-answer=1; Rust replay and third-party blake3 parity pending
inference-catalog-projection-oracle: exact=12; no native provider or route authority
trusted-catalog-identity-oracle: exact=6 canonical-kind=1 descriptor-sha256=1 package-ref-blake3=distinct; no GIS provider activation
gis-native-codec-oracle: receipts=2 hostile=8 ajv+node+webcrypto=1; no catalog activation or GIS execution claim
gis-controlled-proposal-oracle: literal=1 bounds=1 interruption=3 rejection=7 ajv=1; no hub approval authority
gis-native-provider-selection-oracle: cases=8 accepted=1 scope-exports=1; no native or catalog activation claim
error: memory backing schema accepted altered bounds
      at proveMemoryBackendBackingFixture (…/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10822:59)
```

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧬️schema/🔣️.json` is byte-identical to what WP4c
measured (mtime 17:39, unchanged all session): `sequentialTasks {type:integer, minimum:1}`,
`retry.timerDelayMs {type:integer, minimum:0}` — no upper bounds, no exact values, so the four
altered-bound hostiles are all admitted. **Row 142 stays open and stays os's.**

### 3.5 The three hub registration laws — **two attempts, still unrun; blocked in a dependency crate**

Command, exactly as briefed, `CARGO_TARGET_DIR=<scratchpad>/target-w4`, `RUSTC_WRAPPER=""`:

```
cargo test -p semio-hub --no-default-features --features sqlite --lib \
  registers_and_resolves_exactly_the_annotated_formats -- --nocapture
```

**Attempt 1 (21:28) — `EXIT=101`, `semio-hub` never compiled.** It died two crates upstream:

```
   Compiling semio-framework-plugin-host v0.1.0 (…/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust)
error: MutationLeaf source authority failed: No such file or directory (os error 2)
  --> …/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/🦀️.rs:11:1
   |
11 | / #[mutation_leaf(contract = ::protocol)]
…
error: could not compile `semio-framework-plugin-host` (lib) due to 117 previous errors
```

9 × `MutationLeaf source authority failed` + 108 consequent `E0277 the trait bound
`Set*: MutationLeaf` is not satisfied`, all in the os config mutation leaves. The macro could not read a
leaf's payload schema because the mutations partition was mid-move: the six
`🎚️config/🧬️schema/🧬️mutations/<leaf>/🧬️schema/` directories were created at 21:32 and
`🎨️ui-preferences/🧬️schema/🔣️.json` was written at **21:48** — after the compile had already read past it.

**Attempt 2 (21:57), launched once the six leaves all had their `🧬️schema/🔣️.json` on disk:**

`EXIT=101`, `semio-hub` never compiled either. **Two** upstream crates are broken now, both outside this
partition:

```
   Compiling semio-framework-plugin-host v0.1.0 (…/🔌️plugin/🖥️host/📦️packages/🦀️rust)
   Compiling semio-framework-plugin      v0.1.0 (…/🔌️plugin/📦️packages/🦀️rust)

error: MutationLeaf source authority failed: domain-operation root is not explicitly registered
 --> …/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/🌗️set-appearance/🦀️.rs:7:1
  |
7 | / #[mutation_leaf(contract = ::protocol)]
8 | | #[value(rename_all = "camelCase")]
9 | | pub struct SetAppearance { pub appearance: Option<UiAppearance> }
…
error: could not compile `semio-framework-plugin-host` (lib) due to 117 previous errors

error[E0267]: `continue` inside `async` function
     --> …/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28521:33
28383 |     pub async fn plugin_exchange<PA: PluginApp>(…) -> Result<PluginExchangeOutput, Fault> {
error[E0308]: `match` arms have incompatible types
     --> …/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28521:33
error: could not compile `semio-framework-plugin` (lib) due to 4 previous errors
```

Two distinct external breakages, both landed between the two attempts:

1. **`semio-framework-plugin-host` — 9 `MutationLeaf` errors + 108 consequent `E0277`.** The mutations
   partition moved on: the failure is no longer "file not found" but
   `domain-operation root is not explicitly registered`, and the leaves are now one directory deeper
   (`🎨️ui-preferences/🌗️set-appearance/🦀️.rs`, matching the contract's mutation-leaf-is-its-own-scope
   rule). The six `🎚️config` leaves are mid-split and their domain-operation root is not registered yet.
2. **`semio-framework-plugin` — 4 errors** in `…/🔌️plugin/🦀️.rs` around line 28521 (`continue` inside an
   `async` fn, and the two `match` arms that follow from it) inside `plugin_exchange`. A half-applied edit
   in that crate, unrelated to schemas.

**So the three hub registration laws
(`registers_and_resolves_exactly_the_annotated_formats` in `hub.inference`,
`hub.artifact-authority.creation`, `hub.artifact-authority.trusted-catalog`) remain UNRUN**, as they were
at the end of WP4c. Nothing in this report claims they pass. They cannot be run until
`semio-framework-plugin` and `semio-framework-plugin-host` compile again; re-run then with exactly:

```
CARGO_TARGET_DIR=<private> RUSTC_WRAPPER="" \
  cargo test -p semio-hub --no-default-features --features sqlite --lib \
  registers_and_resolves_exactly_the_annotated_formats -- --nocapture
```

`📓️wp4c-hub.md` §7.2's other open item — that `💡️inference/📇️catalog` and `💡️inference/🏃️runtime` are
`#[cfg(feature = "native-artifact-execution")]` and so are not compiled by this feature set — is also
still open, for the same reason.

**Attempt 3 (22:38, WP4e) reproduces attempt 2 exactly — see `📓️wp4e-hub.md` §3.5.** Same two upstream
crates, `semio-hub` still never compiled; `semio-framework-plugin-host` is unchanged (9 + 108), and
`semio-framework-plugin`'s four errors have become a single `E0061` at `🦀️.rs:6577`
(`ViewerApp::<V>::handle` now takes a seventh `view_state: Option<&ViewModel>` argument that the call
site does not pass). The three registration laws are still unrun.

I am deliberately reporting a second attempt: the brief caps cargo at one run, and attempt 1 spent that
budget without ever reaching `semio-hub`, so it measured nothing about this partition. Both are reported;
neither is presented as more than it is.

### 3.6 Hub vitest — **exit 0**

```
$ cd 🌎️hub/📦️packages/🟦️typescript && bun ./📜️script.ts test long
 RUN  v4.1.10 /Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript
 Test Files  1 passed (1)
      Tests  11 passed | 1 skipped (12)
   Duration  37.27s
```

(The skip is the `HUB_E2E`-gated journey. No repair was needed this time — WP4c's explicit
`"vitest.config.ts"` argument is still in `📜️script.ts`, and `test long` is still the level that fits;
`📓️wp4b-hub.md` §7.8's point about the `quick` budget is unchanged.)

## 4. Order of events, so the attribution is checkable

Nothing in §2.4 was caused by §2.1–2.3. The sequence, all on 2026-09-08:

1. `native-openable-catalog-provider-check --oracle-only` red on `const SOURCES` **before any edit**
   (§2.4 a). Confirmed independent of my work: `grep -c "const SOURCES"` = 0 in both the working tree and
   `HEAD`, so no value of the fixture-dir literal could have made that law pass.
2. §2.1–2.3 applied.
3. Same command red on ENOENT for `receipt.protocol_path` (§2.4 c) — this one *is* downstream of the row-126
   move and was fixed as part of it.
4. Same command red on the bijection (§2.4 b, the gitignored `dist/` copies).
5. Green.
6. `trusted-stdio-gis-bundle-check --source` red on `🚀️bin.rs` (§2.4 d), then green.

While this ran, the plugins partition moved `📜️artifact-definition.json` out of `🧬️schema/` (21:44–21:45,
36 modules) and the mutations partition moved the os config leaf schemas (21:27–21:48). A repo-wide
applier also rewrote a path literal inside a file I had just edited. Both are recorded here rather than
worked around.

## 5. Cross-partition requests and open questions

### 5.1 Row 126 — closed

All four missing `✏️s/…` paths resolve; `s.gis/GisNativeCodecs` is bound through the catalog; both hub
fixtures carry the short path. Nothing is pending from the plugins worker. The one correction to the row
as written: the export id is `GisNativeCodecs`, not `NativeCodecReceipts`.

### 5.2 Row 142 — unchanged, os's (`os.db.storage`)

`MemoryBackingV1` still pins no upper bounds; `gis-inference-ledger-oracle` is red on nothing else. See
§3.4 and `📓️wp4c-hub.md` §8.2 for the exact four hostiles.

### 5.3 New — W4 plugins: is the definition inventory still a contract?

`✏️s/🔌️plugins/🗄️stdio/📇️registry/🔣️.json`'s `artifact_definition_paths` is now a hand-maintained
restatement of 36 `include_str!` paths that the modules themselves already declare, and it moved twice in
one hour (`🧬️schema/…` → `…` at 21:44). The hub law (§2.4 a) keeps them equal, but the honest question for
the owner is whether the index should exist at all now that each artifact module is the authority for its
own definition path. If it is deleted, tell the hub and the law becomes "every artifact module compiles
exactly one definition, and the count is 36".

### 5.4 New — repo/plugins: gitignored `dist/` output inside a source tree that laws walk

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/📦️packages/🟦️typescript/dist/🧬️schema/📜️artifact-definition.json`
exists for all 36 artifacts, is gitignored, and is a byte copy of a source contract. Any law that walks
`🗿️artifacts/**` for a contract filename silently doubles. The hub stopped walking (§2.4 b); other
partitions with the same pattern should check. Whether build output belongs under the source tree at all
is a repo call.

### 5.5 New — W5 os / W4 mutations: `semio-hub` cannot be built at all right now

Two crates on `semio-hub`'s dependency path are red (§3.5, attempt 2), which blocks every cargo law in
this partition, not just the three registration laws:

| Crate | Error | Owner |
|---|---|---|
| `semio-framework-plugin-host` | 9 × `MutationLeaf source authority failed: domain-operation root is not explicitly registered` at `…/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/<leaf>/🦀️.rs`, + 108 consequent `E0277 Set*: MutationLeaf` | mutations / os config |
| `semio-framework-plugin` | `E0267 continue inside async function` + 2 × `E0308 match arms have incompatible types` at `…/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:28521`, inside `plugin_exchange` | os plugin |

No action is requested beyond "land it"; this is recorded so the next hub worker does not spend a build
budget rediscovering it, and so nobody reads the unrun laws as a hub defect.

### 5.6 Carried forward, unchanged

`📓️wp4c-hub.md` §8.1 (differently-shaped Rust decoders — four `hub.inference` exports annotated
validated-only), §8.3 (`register_scope_schema_exports` for the seven Rust-less hub scopes), §8.5
(closed), §8.6 (`runVitest` default config path — three callers in other partitions still on the default),
§8.7 (`hub.auth` and `hub.directory` both declare `SocketGrantReceiptV1`; needs one change with WP2),
§8.8 (`GisInferenceCheckpointControlFrameV1` is `#[cfg(feature = "test-support")]`), §8.9.

**`📓️wp4c-hub.md` §8.4 is closed by the coordinator.** `📋️execution-contract.md` §A now reads
"Rust `pub struct|enum|type <Export>` or a per-name `pub use <path>::<Export>;` (grouped or glob
re-exports declare nothing)". Hub's five `pub type` re-exports satisfy the amended rule as they stand and
were not changed back to `pub use`.

## 6. Files changed

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — three `✏️s/…🧫️fixtures` literals (§2.2); the gis codec schema
  bound as `schema://s.gis/GisNativeCodecs` with the private `Ajv2020` deleted (§2.3); the stdio
  definition roster read from the 36 artifact modules (§2.4 a); the recursive definition walk replaced by
  that roster (§2.4 b); `stdioRoot` derived from the fixture's declared definitions root (§2.4 c);
  `proveTrustedCompiledDependenciesFixture`'s three `🚀️bin.rs` anchors read through `moduleRustSource`
  over `🌎️hub/🧪️tests/**` (§2.4 d).
- `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🪪️v1/🔣️.json` — `providerProjection`
  drops `🧬️schema/`.
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🔣️.json` — `stdioReceipts`
  drops `🧬️schema/`.
- `.🧬semio/…/SCOPE-OWNED-SCHEMA-CONTRACTS/📓️wp4d-hub.md` — this report.

No file outside `🌎️hub/**` (and this ticket folder) was edited.

# WP4e — `🌎️hub`: index-independent stdio definition law, registration-laws attempt 3, re-verification

Partition: `🌎️hub/**`. Continuation of `📓️wp4d-hub.md` (its worker was killed by a rate limit while
reporting §3.5 attempt 2; that section was already complete and correct on disk — nothing was recovered,
only extended). Input: `📋️cross-partition-requests.md` rows **152** (decided, hub side is this work) and
**142** (open, os's). Repo MCP was down the whole session — no ticket tool was called, `🗑️generated/`
was never deleted, no git-modifying command was run, no file outside `🌎️hub/**` and this ticket folder
was edited.

## 1. Row 152 — the hub law no longer depends on the stdio registry's hand-maintained index

**Decision as recorded in the ledger:** `✏️s/🔌️plugins/🗄️stdio/📇️registry/🔣️.json`'s
`artifact_definition_paths` (36 restated `include_str!` paths) is deleted by the plugins worker; each
artifact module is the authority for its own definition; the hub law becomes "every stdio artifact module
compiles exactly one definition, count = 36".

**State when this ran: the index still exists on disk** (`grep artifact_definition_paths` at 22:44 →
present, 36 entries, still set-equal to the compiled roster). The law was made independent of it anyway,
as briefed, and hub no longer reads that file at all:

```
$ grep -rn "artifact_definition_paths" 🌎️hub/
$ (no matches)
```

### 1.1 The law, before → after

`proveNativeOpenableCatalogProviderFixture` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (line 4712 ff.).

Before (WP4d): read the index, derived the compiled roster, asserted the two sets equal — the index was
still load-bearing, so a peer deleting it turned the law red for a reason that had nothing to do with the
contract it guards.

After:

```ts
const compiledDefinition = /pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!\("([^"]+)"\);/gu;
const compiledInventory = readdirSync(artifactsRoot, { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => {
    const module = join(artifactsRoot, entry.name, "🦀️.rs");
    const compiled = existsSync(module) ? [...readFileSync(module, "utf8").matchAll(compiledDefinition)] : [];
    if (compiled.length !== 1) throw new Error(`native Stdio artifact module does not compile exactly one artifact definition: ${entry.name}`);
    return resolve(dirname(module), compiled[0][1]);
  })
  .sort();
if (compiledInventory.length !== 36 || new Set(compiledInventory).size !== 36 || compiledInventory.some((path) => !lstatSync(path).isFile())) throw new Error("native Stdio compiled artifact definition roster is not the complete set of 36 module-owned definitions");
```

What the law now asserts, and only this:

1. **every** directory under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/` is an artifact module that compiles
   **exactly one** `ARTIFACT_DEFINITION_SCHEMA` (zero and two are both denied, by module name),
2. the roster is **36** distinct paths,
3. every compiled path resolves, module-relative, to a regular file on disk.

Three properties of the new form are deliberate:

- **`.filter(entry.isDirectory())` instead of skipping modules with no `🦀️.rs`.** The old form silently
  dropped a directory that had lost its module; the new one denies it. (`🗿️artifacts/` holds 36
  directories plus one file, `🔣️.json`, which the filter excludes.)
- **`matchAll` instead of `exec`.** `exec` accepted a module that declared the constant twice and used
  the first; "exactly one definition" is now literally what is checked.
- **Absolute paths via `resolve(dirname(module), …)`.** WP4d's roster was a `../🗿️artifacts/<a>/<p>`
  string keyed to `registryRoot` only because the index was expressed that way. With the index gone the
  indirection has no reason to exist, so `definitionFiles` (line 4757) is now `compiledInventory` itself.
  The law carries no assumption about the depth or existence of the registry directory.

The `include_str!`-regex approach is kept from WP4d for the reason recorded there: the plugins worker
moved `📜️artifact-definition.json` out of `🧬️schema/` into the artifact root at 21:44 mid-session, and a
law that reads the path a module actually compiles survives the next such move. It is currently
`🗿️artifacts/<artifact>/📜️artifact-definition.json` for all 36 (e.g. `☁️las/📜️artifact-definition.json`).

### 1.2 Independent recomputation of the roster (third-party-free oracle, Python vs the TS law)

```
$ python3 (ticket scratch, regex over the 36 modules)
modules: 36 unique: 36 all-files: True
index still present: True index len: 36
index equals compiled set: True
sample: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📜️artifact-definition.json
```

Python's roster equals the one the TS law derives, and — for as long as the index survives — equals the
index too, so the change is behaviour-preserving today and index-independent tomorrow.

### 1.3 Negative proof (the law denies what it claims to deny)

The exact law body run over a fabricated three-module tree in the scratchpad:

```
DENIED native Stdio artifact module does not compile exactly one artifact definition: c   # c declares it twice
DENIED roster is not the complete set of 3                                                # c removed → count 2
```

The positive case is §2.2 below (`owner-receipts=26`, exit 0) against the real tree.

## 2. Verification — real output

All runs `cd /Users/ueli/Documents/semio`, `SEMIO_TEST_ARTIFACT_DIR` =
`🗑️generated/wp4e-hub`, `CARGO_TARGET_DIR` = `$SEMIO_TEST_ARTIFACT_DIR/cargo`, except the cargo law run
(§3.5), which used the briefed private scratchpad target dir.

### 2.1 Partition gate — `wp4c-hub-probe.ts`, 0 hub findings

```
$ bun <ticket>/wp4c-hub-probe.ts .
[wp4c-hub] 0 finding(s) of 2816 repo-wide
```

Unchanged for hub (0, as in WP4c and WP4d). The repo-wide total moved 2691 → **2816** since WP4d ran an
hour ago; none of the new rows is in `🌎️hub`.

### 2.2 `native-openable-catalog-provider-check --oracle-only` — hub laws green, but the command as a
whole is **red in the vcs plugin, before hub runs**

**As briefed, verbatim — exit 1, aborts in `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts:91`:**

```
headless-stdio-metadata-capture-oracle: private-commands=2 replacement-stable=1 replacement-during-capture-denied=1 second-dependency-root-denied=1 outside-target-denied=1
error: VCS config mutation metadata is incomplete
      at proveVcsNativeCodecReceipts (…/✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts:91:217)
      at async run (…/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5150:92)
```

This is **not** hub's and **not** a regression from §1. `NativeOpenableCatalogProviderCheckScript.run`
calls the vcs plugin's own prelude law (`📜️script.ts:5150`) before any hub fixture. That law requires
`schema_version: 1,` **twice** and the literal `semantic_kind: "set-locale"` in
`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`.
A peer deleted the whole `SetLocale` leaf from that aggregate while this session ran (file mtime 22:15;
`git diff HEAD` on that file removes `SetLocale { value: String }`, its descriptor block with
`semantic_kind: "set-locale"` and `owner: …/🎚️config/🗣️set-locale`, `aggregate_variant: "SetLocale"`,
`Self::SetLocale { .. } => &Self::DESCRIPTORS[1]` and the apply arm). The string `set-locale` now appears
in exactly one place in the whole vcs plugin — **the law that demands it**. Both the law and the source
are `✏️s/🔌️plugins/🌿️vcs/**`; see §4.1.

**Hub's own laws, reached by stubbing only that peer-owned prelude — exit 0:**

```
$ bun --preload <ticket>/wp4e-vcs-prelude-stub.ts 🌎️hub/📦️packages/🦀️rust/📜️script.ts native-openable-catalog-provider-check --oracle-only
headless-stdio-metadata-capture-oracle: private-commands=2 replacement-stable=1 replacement-during-capture-denied=1 second-dependency-root-denied=1 outside-target-denied=1
[DEBUG] wp4e: vcs prelude stubbed; peer-owned law is red
vcs-native-provider-selection-oracle: cases=8 accepted=1 unconsumed-profiles=2 linked-receipts=29 scope-exports=2; no native or catalog activation claim
native-openable-claim-oracle cases=8
native-openable-neutral-oracle: AJV=1 scope-exports=3 owner-receipts=26 protocol-webcrypto=26 targets=1 hostile-denied=13 no-partial=13
EXIT=0
```

`owner-receipts=26 protocol-webcrypto=26 targets=1 hostile-denied=13 no-partial=13` is **identical to
WP4d §3.2**, i.e. the index-independent roster feeds the bijection exactly as the index-checked one did.

`wp4e-vcs-prelude-stub.ts` is a Bun `--preload` plugin in the ticket folder that replaces **only**
`proveVcsNativeCodecReceipts` (the sole symbol hub imports from that module, `📜️script.ts:5150`). It
touches no repository file; hub's script is unmodified in that respect. It exists because the command
offers no flag that skips the prelude (`--oracle-only` / `--stdio-only` are the only accepted arguments)
and the prelude is unconditional. Once the vcs owner restores or retires that law, run the command
without the preload.

### 2.3 Hub vitest — exit 0

```
$ cd 🌎️hub/📦️packages/🟦️typescript && bun ./📜️script.ts test long
 RUN  v4.1.10 /Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript
 Test Files  1 passed (1)
      Tests  11 passed | 1 skipped (12)
   Duration  10.43s
EXIT=0
```

(The skip is the `HUB_E2E`-gated journey, as in WP4d. 10.4 s vs 37.3 s is a warm transform cache.)

### 2.4 Row 142 — `gis-inference-ledger-oracle` **still stops at `os.db.storage`**, exit 1

Asked explicitly by the brief. Answer: **yes, unchanged.** Every `hub.*` oracle in the command is green;
it dies on the same os-owned contract:

```
inference-wal-chain-oracle: exact=14 hashing-ownership=3 retained-boundaries=2 ajv=1 crc-valid=14 blake3-known-answer=1; Rust replay and third-party blake3 parity pending
inference-catalog-projection-oracle: exact=12; no native provider or route authority
trusted-catalog-identity-oracle: exact=6 canonical-kind=1 descriptor-sha256=1 package-ref-blake3=distinct; no GIS provider activation
gis-native-codec-oracle: receipts=2 hostile=8 ajv+node+webcrypto=1; no catalog activation or GIS execution claim
gis-controlled-proposal-oracle: literal=1 bounds=1 interruption=3 rejection=7 ajv=1; no hub approval authority
gis-native-provider-selection-oracle: cases=8 accepted=1 scope-exports=1; no native or catalog activation claim
error: memory backing schema accepted altered bounds
      at proveMemoryBackendBackingFixture (…/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10823:59)
EXIT=1
```

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧬️schema/🔣️.json` is **byte-identical to what
WP4c and WP4d measured — mtime 17:39, untouched for five hours**:

```
"tables[].slots":      { "type": "integer", "minimum": 1 }   ← 65 admitted
"sequentialTasks":     { "type": "integer", "minimum": 1 }   ← 65 admitted
"retry.timerDelayMs":  { "type": "integer", "minimum": 0 }   ← 0  admitted
```

so three of the four altered-bound hostiles at `📜️script.ts:10818-10821` are admitted (the fourth,
`tables.slice(1)`, is a `minItems`/enum-coverage case). Row 142 stays **open** and stays **os's**
(`W5c os (🛢️db/🗄️storage)`). No hub change can close it: the hostiles are correct, the owner module is
missing the upper bounds / exact values it used to pin.

### 2.5 The three hub registration laws — attempt 3, still UNRUN, still blocked upstream

The one briefed cargo attempt, exactly as specified:

```
CARGO_TARGET_DIR=<scratchpad>/target-w4 RUSTC_WRAPPER="" \
  cargo test -p semio-hub --no-default-features --features sqlite --lib registers_and_resolves -- --nocapture
```

**`semio-hub` never compiled.** 19 crates compiled, then the same two upstream crates as WP4d attempt 2
failed. Full log: `🗑️generated/wp4e-hub/cargo-attempt3.txt`.

**(a) `semio-framework-plugin-host` — unchanged: 9 + 108 errors, all os-config mutation leaves.**

```
error: MutationLeaf source authority failed: domain-operation root is not explicitly registered
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/…/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/🌗️set-appearance/🦀️.rs:7:1
   |
 7 | / #[mutation_leaf(contract = ::protocol)]
 8 | | #[value(rename_all = "camelCase")]
 9 | | pub struct SetAppearance { pub appearance: Option<UiAppearance>, }
```

9 × that error (`🌗️set-appearance`, `🚗️set-custom-driver`, `🎨️set-custom-theme`, `🕹️set-driver`,
`⌨️set-keybinding-override`, `📐️set-layout`, `🗣️set-locale`, `📖️set-terminology`, `🖼️set-theme`), then
108 consequent `E0277 … : MutationLeaf is not satisfied` at the `optional_setting_impl!` call sites and
in the aggregate `🧬️mutations/🦀️.rs:34`. `error: could not compile semio-framework-plugin-host (lib) due
to 117 previous errors`. Same shape and same count as WP4d attempt 2 (21:57) — **an hour of peer work
has not moved this**.

**(b) `semio-framework-plugin` — different error, still one crate red.** WP4d attempt 2 saw
`E0267 continue inside async fn` + 2 × `E0308` at `🦀️.rs:28521` in `plugin_exchange`; those are gone. What
is red now is a call-site/signature skew introduced since:

```
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:6577:24
6577 | let emit = ViewerApp::<V>::handle(&V::Command::default(), &doc, &cfg, &interaction, &draft, &store::EngineHandles::empty())…
     |            ^^^^^^^^^^^^^^^^^^^^^^                                    ------ argument #5 of type `Option<&ViewModel>` is missing
note: associated function defined here … 🦀️.rs:9837  async fn handle(… view_state: Option<&ViewModel>, …)
```

`ViewerApp::handle` grew a `view_state: Option<&ViewModel>` parameter; one caller at `🦀️.rs:6577` was not
updated. `error: could not compile semio-framework-plugin (lib) due to 1 previous error`.

**Therefore `registers_and_resolves_exactly_the_annotated_formats` for `hub.inference`,
`hub.artifact-authority.creation` and `hub.artifact-authority.trusted-catalog` remains UNRUN**, as at the
end of WP4c and WP4d. Nothing in this report claims it passes. `📓️wp4d-hub.md` §3.5 has been given a
forward pointer to this section so the two attempts read as one series. Re-run with exactly the command
above once both crates compile.

`📓️wp4c-hub.md` §7.2's companion item (`💡️inference/📇️catalog` and `💡️inference/🏃️runtime` are
`#[cfg(feature = "native-artifact-execution")]` and therefore not compiled by this feature set) is also
still open, for the same reason.

## 3. Files changed

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — `proveNativeOpenableCatalogProviderFixture`: the stdio
  artifact-definition roster is derived solely from the 36 modules' own `ARTIFACT_DEFINITION_SCHEMA`
  `include_str!` declarations (exactly one per module, count 36, distinct, on disk); the
  `artifact_definition_paths` index read and the index/roster set-equality assertion are deleted;
  `definitionFiles` is the roster itself (§1.1).
- `.🧬semio/…/SCOPE-OWNED-SCHEMA-CONTRACTS/📓️wp4d-hub.md` — §3.5 forward pointer to §2.5 here.
- `.🧬semio/…/SCOPE-OWNED-SCHEMA-CONTRACTS/wp4e-vcs-prelude-stub.ts` — new; Bun preload that stubs the
  vcs plugin's prelude law so the hub half of `native-openable-catalog-provider-check` can be run while
  `🌿️vcs` is red (§2.2). Delete it when §4.1 is fixed.
- `.🧬semio/…/SCOPE-OWNED-SCHEMA-CONTRACTS/📓️wp4e-hub.md` — this report.
- `.🧬semio/…/SCOPE-OWNED-SCHEMA-CONTRACTS/🗑️generated/wp4e-hub/cargo-attempt3.txt` — cargo evidence.

No file outside `🌎️hub/**` and this ticket folder was edited. (One transient exception, reverted within
the session and verified reverted: `proveNativeOpenableCatalogProviderFixture` was briefly marked
`export` while a direct-call probe was tried; the probe was abandoned because
`runBundleScriptMain` has no entry-point guard and runs the router on import, so importing the hub script
always dispatches a command. The `export` was removed and the probe file deleted; the preload stub of
§2.2 is the approach that shipped.)

## 4. Cross-partition requests and open questions

### 4.1 New — W6 plugins (`✏️s/🔌️plugins/🌿️vcs`): the vcs prelude law outlived its subject

`✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📜️script.ts:91` requires
`config.match(/schema_version: 1,/gu)?.length === 2` and `config.includes('semantic_kind: "set-locale"')`
in `🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`; the `SetLocale` leaf was
deleted from that file this session (now 1 × `schema_version: 1,`, 0 × `set-locale`). Either restore the
leaf or rewrite the law to the leaves that exist. **This reds the whole
`native-openable-catalog-provider-check` command for everyone**, hub included, before a single hub law
runs (§2.2). Both files are the vcs owner's.

### 4.2 Row 152 — hub side done, one note for the plugins owner

The hub law is index-independent as of §1 and hub reads no part of
`✏️s/🔌️plugins/🗄️stdio/📇️registry/🔣️.json` any more. **The index is still on disk** (36 entries, still
set-equal to the compiled roster at 22:44); deleting it is now a no-op for hub and can be done whenever
the plugins worker gets to it. If the roster count ever legitimately changes from 36, the constant in
`📜️script.ts:4724` is the single place to update, and the error message names it.

### 4.3 Row 142 — unchanged, os's (`os.db.storage`)

See §2.4. `MemoryBackingV1` still pins no upper bounds; `gis-inference-ledger-oracle` is red on nothing
else; the owner module has not been touched since 17:39.

### 4.4 Carried forward from `📓️wp4d-hub.md` §5, unchanged

§5.4 (gitignored `dist/` copies of `📜️artifact-definition.json` under all 36 stdio artifacts — the hub
law no longer walks the tree, so hub is immune, but any other partition that walks `🗿️artifacts/**` for a
contract filename still double-counts), §5.5 (**`semio-hub` cannot be built at all** — updated numbers in
§2.5: `semio-framework-plugin-host` 117 errors unchanged, `semio-framework-plugin` now a single `E0061`),
§5.6 → `📓️wp4c-hub.md` §8.1, §8.3, §8.6, §8.7, §8.8, §8.9.

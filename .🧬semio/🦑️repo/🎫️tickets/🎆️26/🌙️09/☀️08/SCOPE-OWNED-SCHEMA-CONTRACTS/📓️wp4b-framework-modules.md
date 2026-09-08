# WP4b — `🧰️framework/🔨️modules/**` follow-ups (ledger rows 35–38, 40)

Partition: every framework module except the registry `🧰️framework/🔨️modules/🧬️schema/` (sibling worker)
and every `🧬️schema/🧬️mutations/**` subtree, plus `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`.
Continuation of `📓️wp4-framework-modules.md`. Nothing under `💻️os`, `🌎️hub`, `✏️s`, `🦑️repo` or the
root `📜️script.ts` was touched — those edits are listed under §7.

## 1. Result per ledger row

| row | request | result |
|---|---|---|
| 35 | consolidate `🧵️job/⏱️budget/🧬️schema/{⏱️clock,🪢️binding,🪫️budget}.json` | **done** — one module, exports `Budget`/`Clock`/`Binding`; root-script patch handed to W2c (§7.2) |
| 36 | rename `🎭️actor/🎠️activation` → `🎠️activation-reservation`, drop `SCOPE_OVERRIDE` | **done** — override removed, `#[path]` fixed, os join handed to W5b (§7.1) |
| 37 | hoist the two ineligible-level render modules into `🖱️ui/🖌️render/🧬️schema/` | **done** — `WebgpuSurfacePort` + `MetalObjectiveCAbiFixture`, both Rust readers rewired |
| 38 | hand-author the `🖱️ui/🖥️host` declaration contract | **done** — real enums + bounds from the Rust host, 20 negative probes |
| 40 | `register_scope_schema_exports` for modules with a `🦀️.rs` schema sibling | **1 of 3 done** (`framework.interaction`, 16 exports, runtime-proven); the other two cannot register — §5 |
| — | `📡️replication/🧫️fixtures/👥️presence-peer-codec-v1/🧬️schema/` found at an ineligible level, and its oracle was dead | **fixed** (§6) |

`module-level-ineligible` inside the partition: **0** (§8).

## 2. Row 36 — `framework.actor.activation-reservation` is path-derivable

`🧰️framework/🔨️modules/🎭️actor/🎠️activation` → `🧰️framework/🔨️modules/🎭️actor/🎠️activation-reservation/`
(4 files moved: `🦀️.rs`, `🧬️schema/🔣️.json`,
`🧫️fixture/🔣️.json`, `🧪️tests/🦀️.rs`). The module `$id` was already
`https://semio.tech/schema/framework/actor/activation-reservation/schema.json`; it is now derivable from
the path, so the transformer's `SCOPE_OVERRIDE` table is gone (`wp4-framework-modules.py`: the dict and
the `scope_path()` branch that read it were deleted — the function is now purely path-derived).

Fixed inside the partition:

- `🧰️framework/🔨️modules/🎭️actor/🦀️.rs:25` `#[path = "🎠️activation/🦀️.rs"]` → `#[path = "🎠️activation-reservation/🦀️.rs"]`.
- The Rust module name stays `pub mod activation` (the crate-internal name, not a path); renaming it to
  `activation_reservation` would break three os call sites (`🔌️plugin/🖥️host/🧵️shard/🔁️lifecycle/🦀️.rs:11-12`,
  `🔌️plugin/🖥️host/🎠️activation/🦀️.rs:6`+`🧪️tests/🦀️.rs:16`, `📺️renderer/…/🧊️wgpu/🎠️runtime/🦀️.rs:76`) and is
  not required by contract §A, which constrains **schema scope ids**, not Rust module paths. Flagged as O-1b.
- The nested `#[path = "🧪️tests/🦀️.rs"]` inside `🎠️activation-reservation/🦀️.rs:137` is relative and needed no edit.

**Correction to `📓️wp4-framework-modules.md` O-1:** there are **not** three os `📜️script.ts` joins to this
directory — there is exactly **one**. The other two `🎠️activation` joins in that same file point at os's own
`💻️os/🔌️plugin/🖥️host/🎠️activation` and `💻️os/🖥️host/🎠️activation`, which are unrelated directories and
must **not** be renamed. Exact request in §7.1.

## 3. Row 37 — `framework.ui.render` owns both target contracts

New module `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧬️schema/🔣️.json`,
`$id https://semio.tech/schema/framework/ui/render/schema.json`, scope `framework.ui.render`.

| export | was | named from |
|---|---|---|
| `WebgpuSurfacePort` | `framework.ui.render.targets.webgpu` `$defs.SurfacePort` (a `🎯️targets/*` level) | its own `title` "Browser WebGPU Surface Port" |
| `MetalObjectiveCAbiFixture` | `framework.ui.render.targets.metal.packages.rust` `$defs.ObjcRuntimeAbiFixture` (a `📦️packages/*` level) | its own `title` "Owned Objective-C Runtime ABI Fixture" |

Deleted: `🎯️targets/🧊️webgpu/🧬️schema/` and `🎯️targets/🍎️metal/📦️packages/🦀️rust/🧬️schema/`.
Multi-export module ⇒ no root `allOf` (same convention as the other multi-export modules).

Readers rewired (both inside the partition):

- `🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧊️surface_adapter.rs:22`
  `include_str!("../../🧬️schema/🔣️.json")` → `include_str!("../../../../🧬️schema/🔣️.json")`.
  Its law `schema_and_language_neutral_ledgers_declare_every_operation_and_limit` still finds all six
  operation names in the hoisted document.
- `🎯️targets/🍎️metal/📦️packages/🦀️rust/🧫️fixtures/🔣️.json:2` and the byte-identical generated literal in
  `🧭️objective_c.rs:606`: `"$schema": "../🧬️schema/🔣️.json"` →
  `"$schema": "https://semio.tech/schema/framework/ui/render/schema.json#/$defs/MetalObjectiveCAbiFixture"`.
  Both sides changed together, so `🧭️objective_c.rs:611`'s `assert_eq!(fixture, include_str!(…))` still holds
  (proven by a real test run, §8).

**Latent defect fixed while hoisting.** The metal export had `additionalProperties: false` and did **not**
declare `$schema`, so the fixture that carries `$schema` could never have validated against its own contract.
`$schema` is now a declared, required property with a `const` equal to the export pointer (the same shape
`framework.assets.*` already uses). Verified with ajv (§8).

## 4. Row 38 — hand-authored `framework.ui.host` declaration contract

`🧰️framework/🔨️modules/🖱️ui/🖥️host/🧬️schema/🔣️.json` replaces the generated structural description of
`🖥️host/🤝️contract.json`. Bounds and vocabularies are read off the code that consumes the declaration, not
invented:

| declaration field | constraint | source |
|---|---|---|
| `version` | integer 1..255 | `eventPrefix` `version:u8`, `🌉️abi/🦀️.rs:7 ABI_VERSION` |
| `transport` | `^semio\.framework\.abi\.v[1-9][0-9]*$` | the `semio.framework.abi.v1` port id |
| `limits.eventBytes` | 1..1024 | `🪟️window.rs:676 BROWSER_HOST_MAX_EVENT_BODY_BYTES` / limits tsv `event-bytes` |
| `limits.eventEnvelopeBytes` | 1..27 | `🪟️window.rs:677 BROWSER_HOST_EVENT_ENVELOPE_BYTES` |
| `limits.encodedEventBytes`, `poll.maximumEncodedBytes` | 1..1051 | `BROWSER_HOST_MAX_ENCODED_EVENT_BYTES`, enforced at `🪟️window.rs:1477/1506/1518` |
| `limits.pageBytes` | 1..1033 | `BROWSER_HOST_MAX_PAGE_BODY_BYTES` |
| `limits.pageEnvelopeBytes` | 1..18 | `BROWSER_HOST_PAGE_ENVELOPE_BYTES` |
| `limits.initialPollBytes` | 1..1024 | `BROWSER_HOST_INITIAL_POLL_BYTES` |
| `limits.retainedPollItems`, `limits.semanticUnitsPerGrant` | 1..1 | limits tsv `retained-poll-items`, `semantic-items-per-grant` |
| `limits.acknowledgements`, `limits.listeners` | 1..64 | `🟨️.js:28-29` defaults + limits tsv |
| `limits.criticalEvents`, `limits.latestEvents` | 1..32 | `🟨️.js:26-27` + limits tsv |
| `identities.*.minimum` | integer 1..4294967295 | limits tsv `canvas-id` / `listener-generation` |
| `identities.CanvasId.type` | enum `u8\|u16\|u32\|u64` | binary-field vocabulary |
| `operations.*` | integer 1793..1798 | `🪟️window.rs:682-687 BROWSER_HOST_OPERATION_*` |
| `events.*` | integer 1801..1811 | `📡️event.rs:35-45 BROWSER_EVENT_*` |
| `eventPrefix` | 4 unique items, `^[a-z][A-Za-z0-9]*:(u8\|u16\|u32\|u64\|i16\|i32\|f32\|f64)(le\|be)?$` | the declared prefix layout |
| `lifecycle` | 8 unique items from a closed enum | `🌉️abi/🦀️.rs:361 AbiMessage` + `:353 AbiControl` + the `terminal-empty` trace row |
| `eventStatus`, `poll.*`, `accessibility.*`, `coalescing.*` | closed enums of the observed policy names | `🪟️window.rs` poll path, `🟨️.js:105-126`, `🧪️fixtures/📊️.tsv` |

`additionalProperties: false` kept at every level. Shared pieces live in the module's `definitions`
(`abiOperationCode`, `abiEventCode`, `binaryField`, `identityOrdinal`). Nothing was widened beyond what the
code admits; no value was turned into a `const` restatement of the data.

## 5. Row 40 — Rust scope-export registration

Modules in the partition with a `🧬️schema/🦀️.rs` sibling: exactly three.

| scope | module | crate | registered? |
|---|---|---|---|
| `framework.interaction` | `🕹️interaction/🧬️schema` | `semio-framework` (mounts it as `interaction::schema`) | **yes** — 16 exports |
| `framework.ui.contract` | `🖱️ui/🧬️contract/🧬️schema` | `semio-framework-ui-contract` | **no** — blocked, see below |
| — | `🚪️io/🧬️schema` | mounted in `semio-framework-os-kernel` | **n/a** — the module has no `🔣️.json`, so it declares no named exports |

TS-only modules (`🎠️kernel`, `📡️replication`'s TS leaf, `🖱️ui/🎨️styling`, `🖼️assets`, …) have no Rust schema
leaf and therefore no Rust registration site at all; per contract §C their exports reach the resolver only
through the generated catalog.

### 5.1 `framework.interaction` — registered

`🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🦀️.rs` gained a `🔖️ScopeSchemaExports` region:

```rust
const LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"),
    graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: "" };
const EXPORTS: [SchemaExport; 16] = [ SchemaExport { id: "InteractionDefinition", leaves: LEAVES }, … ];
pub fn register_scope_exports() {
    register_exports(ScopeSchemaExports { scope: "framework.interaction", exports: &EXPORTS })
        .expect("framework.interaction scope schema exports");
}
```

| scope | exports registered |
|---|---|
| `framework.interaction` | `InteractionDefinition`, `GranularityDefinition`, `HierarchyProvider`, `HoverSpec`, `SelectionSpec`, `SelectionMode`, `SelectionMethod`, `MergeMode`, `InteractionTarget`, `DomainSelection`, `DomainHover`, `InteractionState`, `TopologyNode`, `DomainTopology`, `PresenceDomain`, `PresenceInteraction` |

- Scope id equals the `$id`-derived id in `wp4-framework-scope-exports.md` and the path-derived id (§8).
- No export id reuses `artifact`/`snapshot`/`diff`/`mutations`; the scope registers no
  `ArtifactSchemaDescriptor`, so there is no descriptor to sit "beside" — this is the crate's schema
  registration entry point instead.
- `🧰️framework/📦️packages/🦀️rust/Cargo.toml` gained `semio-framework-schema = { workspace = true }`.
  No cycle: `semio-framework-schema → semio-framework-os-kernel`, and `semio-framework` already depends on
  `semio-framework-os-kernel`; os-kernel has no edge back to `semio-framework`.
- A sibling `#[cfg(test)]` law proves the registration at runtime rather than by inspection: it registers,
  then resolves all 16 exports in all four declared formats, asserts `Protobuf` resolves to an error (no
  proto leaf), and asserts `scope_schema_exports_registered("framework.interaction")`. Real run in §8.
- Nothing calls `register_scope_exports()` from production code yet — the OS boot path is the natural caller
  (same shape as `register_artifact_schema()` in the stdio artifacts, which the plugin assembly calls).
  Request in §7.4.

### 5.2 `framework.ui.contract` — cannot register today

`semio-framework-ui-contract`'s own header docstring forbids an os-kernel edge ("no actor kernel, no
os-kernel `dsl` — so this compiles for `wasm32-wasip2` guests … and a CI `cargo tree` assertion keeps it
so"). `semio-framework-schema` depends on `semio-framework-os-kernel`, so adding it to that crate would
reintroduce exactly the edge the invariant forbids. Its schema leaf is additionally `#[cfg(feature =
"typegen")]`-gated, so even an optional `typegen`-only dependency would register the scope in typegen
builds only. Not done; request in §7.3.

## 6. Out-of-band fix: `📡️replication`'s presence-peer codec module

Found while checking that every module `$id` is path-derivable:
`📡️replication/🧫️fixtures/👥️presence-peer-codec-v1/🧬️schema/🔣️.json` was a schema module inside a `🧫️*`
directory — not an eligible owner level (contract §A) — and its only TypeScript oracle was **dead**:

```
$ (cd 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust && bun ./📜️script.ts presence-peer-codec-check --oracle-only)
error: no schema with key or ref "http://json-schema.org/draft-07/schema#"
      at run (…/📦️packages/🦀️rust/📜️script.ts:256:48)
```

(the script still imported `ajv/dist/2020.js`; the module had been migrated to draft-07.)

Applied:

- `PresencePeerCodecFixture` and its 11 internal definitions moved into
  `📡️replication/🧬️schema/🔣️.json` (`framework.replication`, now `ArtifactBootstrapFixture` +
  `PresencePeerCodecFixture`; no definition-name collision; root `allOf` dropped because the module became
  multi-export — its only consumer, `📡️replication/🟦️.ts:1491`, already resolves by
  `getSchema(…#/$defs/ArtifactBootstrapFixture)`).
- Fixture data moved to `🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json` (the shape `🚀️artifact-bootstrap`
  already uses); `📡️wire/🦀️.rs:2150`'s `include_str!` and `📦️packages/🦀️rust/📜️script.ts` rewired,
  the script switched to plain `ajv` + `addSchema(…).getSchema(\`${schema.$id}#/$defs/PresencePeerCodecFixture\`)`.
- `👥️presence-peer-codec-v1/🧬️schema/` and `🧪️fixture/` deleted.

```
$ (cd 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust && bun ./📜️script.ts presence-peer-codec-check --oracle-only)
presence peer codec oracle: 18 neutral Rust/TypeScript vectors, 16 hostile inputs rejected exactly
```

## 7. Cross-partition requests

### 7.1 W5b os — the one `🎭️actor/🎠️activation` join (row 36)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:187`, in
`kernelReservationOracle()`:

```
-  const root = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "🔨️modules", "🎭️actor", "🎠️activation");
+  const root = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "🔨️modules", "🎭️actor", "🎠️activation-reservation");
```

That one `root` covers all three reads under it (`🧫️fixture/🔣️.json`, `🧬️schema/🔣️.json`, `🦀️.rs`); the
export name `ActivationReservation` and the `$id` are unchanged. **Do not touch** lines 165 and 179 of the
same file — they resolve `💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation` and `💻️os/🖥️host/🎠️activation`,
which are different, os-owned directories.

### 7.2 W2c tooling — root `📜️script.ts`, job-budget consolidation (row 35)

`toolJobMicrosecondBudgetSelfTests()` is **broken until this lands** (accepted transient break per ledger
row 35). `const base = join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/🧵️job/⏱️budget")` is unchanged; add one
module read and bind each validator to its export:

```
-  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🪫️budget.json"), "utf8"));                                   // :2015
-  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);                                              // :2017
+  const module = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
+  const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(module);
+  const validate = ajv.getSchema(`${module.$id}#/$defs/Budget`)!;

-  const validateClocks = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(base, "🧬️schema/⏱️clock.json"), "utf8")));   // :2032
+  const validateClocks = ajv.getSchema(`${module.$id}#/$defs/Clock`)!;

-  const validateBinding = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(base, "🧬️schema/🪢️binding.json"), "utf8"))); // :2058
+  const validateBinding = ajv.getSchema(`${module.$id}#/$defs/Binding`)!;
```

(`const Ajv = createRequire(import.meta.url)("ajv")` stays where it is, at :2016, i.e. between the module read
and the `new Ajv(…).addSchema(module)` line. One `Ajv` instance may hold the module only once — `addSchema`
throws on re-registration, which is why the three validators share `ajv` instead of each compiling.)

The three data reads (`🧫️fixture/🔣️.json`, `🕰️clock.json`, `🪢️binding.json`) keep their paths; only their
`$schema` values changed, and those are already applied. The forged-input probes at :2028 still work
(`Budget` rejects `unit: "milliseconds"` and an extra field — verified, §8).

No other reader exists repo-wide: the taxonomy fixture
`📚️library/🧪️tests/🔏️path-emoji-statutes/🔣️.json:357` names the **data** file `⏱️budget/🪢️binding.json`,
which was not moved, and `📚️library/🔣️schema-catalog.json` is generated.

### 7.3 W3b framework schema registry — let leaf crates register

`framework.ui.contract` (and any other dependency-restricted scope) cannot call
`register_scope_schema_exports` while `semio-framework-schema` carries a
`semio-framework-os-kernel` dependency. Either split the registry/resolver types
(`SchemaFormat`, `FacetLeaves`, `SchemaExport`, `ScopeSchemaExports`, the OnceLock catalog) into an
os-kernel-free leaf crate, or state that scopes in dependency-restricted crates register from their owning
product crate instead. Decision needed before row 40 can be closed for the whole tree.

### 7.4 Coordinator / W5b os — call the registration

`semio_framework::interaction::schema::register_scope_exports()` exists but nothing calls it in production;
`register_scope_schema_exports` is not idempotent (a second call for the same scope returns
`ConflictingScope`), so it needs exactly one boot-time call site — the os boot path, next to wherever the
plugin assembly calls `register_artifact_schema_descriptors`.

### 7.5 W2c tooling — regenerate `📚️library/🔣️schema-catalog.json` / `📓️schema-catalog.md`

Stale rows after this pass (`schema check` reports `catalog-stale=1`):
`🧰️framework/🔨️modules/🎭️actor/🎠️activation/🧬️schema` → `…/🎠️activation-reservation/🧬️schema`;
`framework.ui.render.targets.webgpu` and `framework.ui.render.targets.metal.packages.rust` → one
`framework.ui.render` at `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧬️schema`;
`framework.replication.fixtures.presence-peer-codec-v1` folded into `framework.replication`;
`framework.job.budget` gains `Budget`/`Clock`/`Binding`.

### 7.6 W5b os — regenerate `🧑‍💻dev/📤️distribution/🧾️manifest.json`

Already on the prior worker's list (styling test paths); add the two render rows this pass invalidated:

- `:7467` `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧬️schema/🔣️.schema.json`
  — **already stale before this pass** (WP4 renamed it to `🔣️.json`); the file is now deleted outright.
- `:7567` `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧬️schema/🔣️.json` — deleted.

Both are covered by `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧬️schema/🔣️.json` after regeneration.

## 8. Verification (real output)

```
$ node <ticket>/wp4-framework-validate.mjs
modules=116 exports=215 badDialect=0 badId=0 noExports=0 problems=0
```

```
$ node <ticket>/wp4b-framework-checks.mjs
job.budget/Budget PASS
job.budget/Clock PASS
job.budget/Binding PASS
job.budget/Budget rejects forged unit: true rejects extra field: true
ui.render/MetalObjectiveCAbiFixture PASS
ui.render/WebgpuSurfacePort compiled: true
ui.host/BrowserHostDeclaration PASS
ui.host rejects eventBytes above 1024: true
ui.host rejects encodedEventBytes above 1051: true
ui.host rejects pageBytes above 1033: true
ui.host rejects listeners above 64: true
ui.host rejects criticalEvents above 32: true
ui.host rejects retainedPollItems above 1: true
ui.host rejects semanticUnitsPerGrant above 1: true
ui.host rejects canvas identity minimum 0: true
ui.host rejects canvas identity above u32: true
ui.host rejects operation code outside 1793..1798: true
ui.host rejects event code outside 1801..1811: true
ui.host rejects foreign transport: true
ui.host rejects empty title: true
ui.host rejects malformed event prefix field: true
ui.host rejects duplicate lifecycle stage: true
ui.host rejects unknown lifecycle stage: true
ui.host rejects unknown coalescing policy: true
ui.host rejects unknown poll short-capacity policy: true
ui.host rejects unknown accessibility label policy: true
ui.host rejects extra top-level key: true
path-derivable module $ids: 116 ok, 0 not derivable
```

The last line is the row-36/37 acceptance test: every module `$id` in the partition equals
`https://semio.tech/schema/framework/<ascii tail of each directory>/schema.json`, so no override table is
needed anywhere.

```
$ bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w10b.jsonl
[schema check] modules=3189 scopes=2603 findings=7550
```

Rows whose `path` is inside the partition (`🧰️framework/🔨️modules/**`, excluding
`🧰️framework/🔨️modules/🧬️schema` and `**/🧬️mutations/**`):

| code | count |
|---|---|
| `module-level-ineligible` | **0** |
| `ref-not-catalog-addressable` | 67 |
| `ref-not-export-addressed` | 424 |
| everything else (`document-dialect-unexpected`, `document-id-missing`, `document-id-duplicate`, `module-scope-id-inconsistent`, `module-scope-id-missing`, `export-id-invalid`, `scope-id-duplicate`, `placement-retired-location`, `ref-unresolved`) | 0 |

The single `module-level-ineligible` row anywhere under `🧰️framework/🔨️modules` is
`🧰️framework/🔨️modules/🧬️schema` — the registry module, the **sibling worker's** partition:
`"🧰️framework/🔨️modules is not a declared schemaScopeOwnerLevels level."` (a taxonomy gap: the registry
module sits directly at the modules root, one level above `🧰️framework/🔨️modules/<m>`.) The 491 `ref-*`
rows are discussed as O-2b below.

### Rust

```
$ CARGO_TARGET_DIR=<scratch>/target-w10 RUSTC_WRAPPER="" cargo check -p semio-framework --offline
    Checking semio-framework-ui-contract … semio-framework-actor … semio-framework-schema … semio-framework
    Finished `dev` profile [unoptimized] target(s) in 1m 00s          # 0 warnings, 0 errors
$ … cargo check -p semio-framework --all-targets --offline
    Finished `dev` profile [unoptimized] target(s) in 55.32s          # 0 warnings, 0 errors
$ … cargo check -p semio-framework-ui-backend-webgpu -p semio-framework-ui-backend-metal --all-targets --offline
    Checking semio-framework-ui-render … ui-backend-metal … ui-backend-webgpu
    Finished `dev` profile [unoptimized] target(s) in 58.78s          # 0 warnings, 0 errors
$ … cargo check -p semio-framework-replication --all-targets --offline
    Finished `dev` profile [unoptimized] target(s) in 51.55s          # 0 warnings, 0 errors
```

```
$ … cargo test -p semio-framework-ui-backend-metal --offline --lib owned_runtime_preserves
test objective_c::tests::owned_runtime_preserves_empty_single_max_max_plus_one_and_hostile_contract ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.45s

$ … cargo test -p semio-framework --offline --lib every_named_export_resolves
test interaction::schema::tests::every_named_export_resolves_in_every_declared_format ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 214 filtered out; finished in 0.01s
```

The metal test is the runtime proof that the rewritten `$schema` pointer in `🧭️objective_c.rs`'s generated
literal is byte-identical to the fixture on disk. The `semio-framework` test is the runtime proof of the row-40
registration (16 exports × 4 formats resolve, protobuf absent, scope registered).

### Suites

```
$ (cd 🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript && node …/vitest.mjs run --config vitest.config.ts)
 Test Files  1 passed (1)      Tests  5 passed (5)
$ (cd 🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript && node …/vitest.mjs run --config vitest.config.ts)
 Test Files  2 passed (2)      Tests  50 passed (50)
$ (cd 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript && node …/vitest.mjs run --config 🧪️tests/🟦️.ts)
 Test Files  3 failed | 7 passed (10)      Tests  3 failed | 203 passed (206)
$ (cd 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust && bun ./📜️script.ts presence-peer-codec-check --oracle-only)
presence peer codec oracle: 18 neutral Rust/TypeScript vectors, 16 hostile inputs rejected exactly
```

The 3 actor failures are `📓️wp4-framework-modules.md` §6's exact baseline and reproduce without this pass:
`🚪️lifetime/🟦️.ts:393` (the lifetime fixture violates its own oracle's `maxLength: 150`, pre-existing) and
two vitest 5 s budget timeouts under concurrent load (`📤️return/🟦️.ts:437`, `📤️return/📨️response/🟦️.ts:209`).
A first run of the same suite also timed out `🪪️activation/🚪️instance/📥️output/🟦️.ts`; the second run passed
it, so it is load flake, not a regression.

### Peer breakage observed, not fixed

- **`🖱️ui/🎨️styling` suite, 5 failures — an in-flight peer move, not this pass.** A peer moved
  `🎨️styling/🧪️tests/🟦️.ts` → `🎨️styling/🧪️tests/🧩️suite/🟦️.ts` at 17:55 today (old path shows as `D`,
  `🧩️suite/` is untracked) without moving or repointing the four sibling JSON fixtures, so the suite now
  raises `ENOENT … 🧪️tests/🧩️suite/{🌐️favicon-delivery,🏠️html-entry,🛡️build-writes,🧊️mesh-collection}.json`
  (4 ENOENT + 1 dependent 5 s timeout; 36 pass). The same peer also moved
  `🖥️host/📦️packages/🟨️javascript/🧪️browser-host.test.js` → `🖥️host/🧪️tests/🌐️browser-host/🟨️.js` and
  `🧊️webgpu/📦️packages/🟨️javascript/🧪️webgpu-surface.test.js` → `🧊️webgpu/🧪️tests/🖼️surface/🟨️.js`.
  Left alone — the move is still in progress and the fixtures are one directory up.

## 9. Files changed

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🎭️actor/🎠️activation-reservation/{🦀️.rs,🧬️schema/🔣️.json,🧫️fixture/🔣️.json,🧪️tests/🦀️.rs}` | renamed from `🎠️activation/` |
| `🧰️framework/🔨️modules/🎭️actor/🦀️.rs` | `#[path]` → `🎠️activation-reservation/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧬️schema/🔣️.json` | **new** — `WebgpuSurfacePort`, `MetalObjectiveCAbiFixture` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧬️schema/🔣️.json` | **deleted** |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧬️schema/🔣️.json` | **deleted** |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧊️surface_adapter.rs` | `include_str!` depth |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧭️objective_c.rs` | `$schema` pointer in the generated literal |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧫️fixtures/🔣️.json` | `$schema` pointer |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧬️schema/🔣️.json` | hand-authored contract |
| `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/🔣️.json` | **new** — `Budget`, `Clock`, `Binding` |
| `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/{🪫️budget,⏱️clock,🪢️binding}.json` | **deleted** |
| `🧰️framework/🔨️modules/🧵️job/⏱️budget/{🧫️fixture/🔣️.json,🕰️clock.json,🪢️binding.json}` | `$schema` pointer (one line each, formatting untouched) |
| `🧰️framework/🔨️modules/📡️replication/🧬️schema/🔣️.json` | `+PresencePeerCodecFixture` + 11 definitions, root `allOf` dropped |
| `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json` | moved up from `🧪️fixture/` |
| `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/👥️presence-peer-codec-v1/🧬️schema/🔣️.json` | **deleted** |
| `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` | fixture `include_str!` path |
| `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts` | plain `ajv` + export-addressed validator, new paths |
| `🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🦀️.rs` | `🔖️ScopeSchemaExports` region + test |
| `🧰️framework/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-schema = { workspace = true }` |
| `<ticket>/wp4-framework-modules.py` | `SCOPE_OVERRIDE` removed; `MODULE_HOIST` gained the three new hoists |
| `<ticket>/wp4-framework-scope-exports.md` | rows updated (116) |
| `<ticket>/wp4-framework-cross-partition.md` | §job-budget marked applied |
| `<ticket>/wp4b-framework-checks.mjs` | **new** — the acceptance checks above |

`🧰️framework/📦️packages/🟦️typescript/🟦️.ts` needed **no** change: its only framework-module schema
references are `🕹️interaction/🧬️schema/🟦️.ts` (unmoved) and `🎠️kernel/🧬️schema/🔣️.json` (unmoved).

Stale-reference sweep over tracked source files for every moved path and retired `$id`
(`🎭️actor/🎠️activation/`, `render/targets/{webgpu,metal/packages/rust}/schema.json`,
`replication/fixtures/presence-peer-codec-v1/schema.json`, `🧬️schema/{🪫️budget,⏱️clock,🪢️binding}.json`,
`👥️presence-peer-codec-v1/🧪️fixture`, `🍎️metal/…/🦀️rust/🧬️schema`, `🧊️webgpu/🧬️schema`) plus a second
unfiltered repo-wide `grep -rn` for the same patterns: the only live hits outside this ticket folder are the
os join (§7.1), the three root-script sites (§7.2) and the two generated distribution-manifest rows (§7.6).
Every other hit is prose in `📊️wp0-*.json` / `📓️wp4-framework-modules.md` / `wp4-framework-cross-partition.md`
inside this ticket folder.

## 10. Open questions

- **O-1b** Should `semio_framework_actor`'s Rust module keep the name `activation` now that its directory is
  `🎠️activation-reservation`? Renaming it to `activation_reservation` is a 5-site os change (§2) and is not
  required by contract §A. Left as is.
- **O-2b** `schema check` reports 491 `ref-not-export-addressed` / `ref-not-catalog-addressable` rows across
  41 files of this partition, and 3350 repo-wide. Every one of them is the module shape the coordinator
  already accepted in `📓️wp4-framework-modules.md` §2: exports live in `$defs`, internal helper schemas live
  in a module-root `definitions`, and `$ref`s address them as `#/definitions/<name>` (cross-scope:
  `<target $id>#/definitions/<name>`). Promoting those helpers to `$defs` would move the violation to
  `export-id-invalid` (they are lowerCamel, not PascalCase, and they are not contracts). Either the checker
  must accept `#/definitions/<name>` for module-internal helpers, or contract §B must say helpers are
  forbidden and every module must inline them. Not resolvable inside one partition — **coordinator decision
  needed**; nothing was mass-rewritten on a guess.
- **O-3b** `🧰️framework/🔨️modules/🧬️schema` (the registry) is the only `module-level-ineligible` row under
  `🧰️framework/🔨️modules`: it sits directly at the modules root, which `schemaScopeOwnerLevels` does not
  declare. Registry owner + taxonomy owner decide whether it moves under a module directory or the level is
  declared.

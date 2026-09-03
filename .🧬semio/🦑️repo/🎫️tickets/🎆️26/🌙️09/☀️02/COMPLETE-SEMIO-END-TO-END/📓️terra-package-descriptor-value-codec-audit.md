# PackageDescriptor First-Party Value Codec Audit

## Decision

The first blocker is a direct missing trait implementation, not a Cargo feature or an emitter-input failure: `PackageDescriptor` has only `Serialize`/`Deserialize`, while the descriptor emitter calls `dsl::from_dsl_value::<PackageDescriptor>` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/🦀️.rs:427`.  That generic requires `PackageDescriptor: FromValue`; the same root is present in the committed-descriptor reader at `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1686`.  The emitter's immediately following self-hash and final encoding calls also require `ToValue` (`:437`, `:441`).

Land one structural descriptor-spine conversion now.  Add first-party `ToValue`/`FromValue` codecs with field-level wire parity to the six stale serde-only owned structs, then make the guest descriptor emitter call `dsl::to_dsl_value` directly.  Do not route either direction through `serde_json`, do not define a shadow descriptor, and do not add a runtime dependency or a compatibility codec.

This audit is read-only.  No production or test source changed, and no build, test, or runtime command was run.

## Exact first cause and current evidence

`PackageDescriptor` is declared at `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4880-4900` as:

```text
Clone + Debug + PartialEq + Serialize + Deserialize
```

It has neither a handwritten `impl dsl::{ToValue, FromValue}` nor the two value derives.  The `semio-framework` package already has the value-derive crate unconditionally, and the manifest imports the traits/macros through `dsl` (`🛂️manifest/🦀️.rs:1-18`; `🧰️framework/📦️packages/🦀️rust/Cargo.toml`).  There is no relevant optional feature to turn on.

The call chain is internally contradictory today:

```text
guest describe_plugin / describe_extension
  PackageDescriptor --serde_json--> DslValue --pack--> describe bytes

native plugin-describe CLI / committed-descriptor reader
  pack bytes --DslValue--> FromValue<PackageDescriptor>     [missing]
```

The guest bridge is explicit at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs:146-153` and `:209-215`; it silently turns serialization failure into `DslValue::Null`.  The producer and both first-party readers must instead share one direct codec:

```text
PackageDescriptor ⇄ ToValue/FromValue ⇄ DslValue ⇄ canonical pack
```

Thus the observable first diagnostic is expected to be `E0277` at the generic emitter call for a missing `FromValue` implementation.  No checked-in compiler transcript names a deeper `PackageDescriptor` error, so this audit does not claim a compiler ordering beyond the statically visible generic bound.

## Ownership and stale blocker census

This is handwritten schema ownership with derive-generated codec bodies; it is not stale generated output.  The schema owner is `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`; `semio-framework-value-derive` supplies the already-used mechanical implementation.  The following six owned records are the complete deliberately serde-only spine reachable from `PackageDescriptor`:

| Order | Type | Source | Why it blocks the parent derive now |
| --- | --- | --- | --- |
| 1 | `UtilityDefinition` | `manifest/🦀️.rs:1345-1365` | Reached through `AppDefinition.utilities`. |
| 2 | `WindowKindDefinition` | `manifest/🦀️.rs:3188-3219` | Reached through `AppDefinition.window_kinds`. |
| 3 | `AppDefinition` | `manifest/🦀️.rs:3412-3490` | Reached through `PluginManifest.apps`. |
| 4 | `PluginManifest` | `manifest/🦀️.rs:4059-4085` | Direct `PackageDescriptor.manifest`. |
| 5 | `ExtensionPointDeclaration` | `manifest/🦀️.rs:4694-4708` | Direct `PackageDescriptor.extension_points`. |
| 6 | `PackageDescriptor` | `manifest/🦀️.rs:4880-4900` | The requested public structural root. |

Their `🚧️ BLOCKED` comments are stale in material ways.  The formerly named leaves already carry the same first-party traits:

- `kernel::CapabilityRequirement` is dual-derived at `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:169-181`.
- `kernel::ActivationEvent` is dual-derived at `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1149-1162`.
- `kernel::CapabilityRequest` and `QuotaSchema` are dual-derived at `🎠️kernel/🦀️.rs:2232-2244` and `:2277`.
- `ui_wgpu::wgpu::UtilityCategory`, `WindowOptions`, the wgpu `SurfaceKind`, `LocalizedLabel`, `IconName`, `NamedLayout`, and `WindowLayout` have first-party codec coverage in the active wgpu graph.  In particular, `UtilityCategory` is dual-derived at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:1602-1610`, `WindowOptions` at `:1296-1308`, and the manifest-facing wgpu `SurfaceKind` at `:3180-3228`.
- `NonEmptyVec<T>` already implements both traits in `manifest/🦀️.rs:3141-3160`, so `Modes`/`WindowKinds` do not need a new container codec.

This means the historical reason to leave the six parents serde-only has expired.  Before landing, Sol must re-check the complete compiler closure because this is a shared tree; the exact evidence above supports the bounded six-type packet but is not a substitute for that check.

## Smallest safe Sol packet

1. In `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`, add `ToValue, FromValue` to the derives of exactly the six records in the table.  Add `#[value(rename_all = "camelCase")]` beside every corresponding container-level serde naming rule.
2. Copy each field's existing serde omission/default semantics into a matching `#[value(...)]` attribute.  This includes all defaults and omissions on `PackageDescriptor`, `PluginManifest`, `AppDefinition`, `WindowKindDefinition`, `UtilityDefinition`, and `ExtensionPointDeclaration`; e.g. empty `activationEvents`, `capabilityRequests`, `extensionPoints`, and `assets` remain omitted, while defaulted `quotas` and `contributions` decode when absent.  Do not change field names, types, `descriptor_version`, hash rules, or public schema version in this packet.
3. Retire the two descriptor-producer serde bridges in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs:146-153` and `:209-215`.  Encode with `dsl::to_dsl_value(&descriptor)` and return an explicit bounded descriptor fault if a structural codec unexpectedly fails; never convert failure to `Null`.
4. Keep the native CLI decoder/encoder at `📇️describe/📦️packages/🦀️rust/🦀️.rs:427,437,441` and the run reader at `🏃️run/🦀️.rs:1686` as direct first-party consumers.  They are the correct boundary and should not be reverted to serde to conceal the missing implementation.
5. Delete or correct the six stale `BLOCKED` comments only in the same atomic codec patch, after the compile closure is actually confirmed.  Do not add an adapter or retain an alternate serde descriptor channel.

The packet does not add `packageId`.  The current `PackageDescriptor` still has no such field, as the trusted-catalog audit records; identity-schema work is a separate, deliberate versioned contract change.  It must not be smuggled into this codec repair or break the two explicit struct literals in the guest emitter (`🛂️describe/🦀️.rs:133-145`, `:196-208`).

## Field and wire laws

The first-party codec must be structurally equivalent to the existing descriptor JSON/pack representation, not merely compile:

```text
encode: all ordinary fields use camelCase;
        optional fields obey their existing omission predicate;
        empty vectors currently marked skip_serializing_if remain absent.

decode: missing fields marked serde(default) select the same Default value;
        missing required fields fail with a dotted first-party ValueError path;
        existing descriptor version/hash semantics stay unchanged.

hash:  descriptorSha256 is SHA-256 of canonical packed descriptor bytes with
       descriptorSha256 empty; final bytes contain the patched digest.
```

`#[value]` attributes are mandatory rather than cosmetic.  The derive reads only those attributes, not `#[serde(...)]`; forgetting one would make compact old forms decode differently or emit a different descriptor hash.  The derive supports the named-field structs, camel-case naming, defaults, omission predicates, transparent wrappers, and internally tagged enums used here (`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:19-52`).  No new derive capability is required.

## Affected packages and boundaries

| Package/path | Effect of the missing root codec |
| --- | --- |
| `semio-framework` | Owns the descriptor schema; cannot satisfy a direct `PackageDescriptor: ToValue + FromValue` bound. |
| `semio-framework-plugin` | Still serializes descriptor output through serde and can produce `Null` on serialization failure. |
| `semio-framework-plugin-describe` | Its CLI's decode/hash/finalize path has the immediate E0277 blocker. |
| `semio-framework-os-run` | The committed descriptor trust/root reader has the same missing `FromValue` requirement. |
| `@semio-tech/stdio-plugin:describe` | Invokes the blocked CLI path, so it cannot produce the canonical descriptor pair needed before the later stdio trust-root/factory work. |

The hub trusted-catalog loader currently uses serde for its own test hydration and is not the first compiler site shown by this audit.  It should consume the canonical packed descriptor through the repaired first-party codec in its own later boundary cleanup; that is not needed to unblock the CLI.

## Neutral fixture and independent oracle

Add the schema-first vector before implementation:

```text
🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🧬️package-descriptor-value/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🧬️package-descriptor-value/🔣️.json
```

Use one compact valid plugin descriptor that exercises: an EN/DE `LocalizedLabel`; a non-empty `UtilityDefinition.category`; a `WindowKindDefinition` with capability requirement; one activation event and extension point; `WindowOptions`; non-empty `NonEmptyVec` modes/windows; and canonical empty hash placeholders.  Include hostile cases for a missing required root field, invalid activation-event variant, empty modes/windows, unknown enum value, and omitted default/empty fields.

The Rust manifest test must decode fixture `DslValue` with `PackageDescriptor::from_value`, re-encode with `to_value`, compare the normalized neutral structural value, and decode the canonical packed bytes again.  The plugin-side test must call `describe_plugin`/`describe_extension`, decode only with `FromValue`, and assert that it matches the fixture shape without a serde bridge.  The CLI test must verify that pre-hash and final bytes obey the self-hash law, JSON is only a presentation projection of the same value, and a malformed packed descriptor fails before either output file is published.

The independent TypeScript oracle belongs beside the existing plugin registry check, using only its canonical pack/JSON reader: parse the neutral JSON, assert camel-case/default/omission laws, calculate the pre-hash SHA-256 with Node `crypto`, and compare with the Rust-emitted final pack's decoded structure.  It must not call Rust helper code or accept a serde-produced alternate byte form.  This establishes language-neutral structural parity while keeping the Rust first-party codec authoritative.

## Acceptance and blocker order

1. **First blocker — repair now:** add the six structural codecs with exact `#[value]` attribute parity, then replace the two guest serde bridges.  This unblocks the CLI's E0277 and makes the committed-descriptor reader use the same canonical contract.
2. **Verify next:** run the fixture/oracle plus focused framework, plugin-describe, plugin-run, and stdio descriptor targets.  This audit makes no pass/fail claim.
3. **Later identity work:** add a versioned explicit package identity and trusted static native-factory receipt before declaring `stdio` a trust-root.  Codec parity makes descriptor production possible; it neither creates raw/core artifacts nor supplies native-codec authority.

# 🔓 lease-request — P3-manifest-schema → MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME's sol

**Requesting agent:** terra (P3-manifest-schema, ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`)
**Target file:** `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (your A3-kernel-types territory; one-line lease, same pattern as P9's kernel mount lease)
**Reason:** our packet's acceptance step requires regenerating `🛂️manifest/🤖️generated/🟦️manifest.ts` via `bun ./📜️script.ts generate` (from `🧰️framework/📦️packages/🦀️rust`), which runs `cargo test --features typegen exports_typescript_bindings`. This currently fails — **not because of anything we added** — with:

```
error[E0277]: the trait bound `CapabilityToken: TS` is not satisfied
   --> 🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:972:16
    |
969 | #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
    |                                        --------- required by a bound introduced by this call
...
972 |     pub token: CapabilityToken,
    |                ^^^^^^^^^^^^^^^ unsatisfied trait bound
```

`BrokerCapabilityGrant` (L967–974) derives `ts_rs::TS` under the `typegen` feature; one of its fields, `CapabilityToken` (L23, `pub struct CapabilityToken(pub u128);`), does not itself derive `TS`. This is a **derive-macro-level** bound — it fires the instant `--features typegen` compiles this crate at all, independent of whether anything calls `.export()` on `BrokerCapabilityGrant`, and independent of any change we made. `git log -S "struct CapabilityToken"` dates its introduction to commit `b92a614c` (2026-08-07), ten days before this ticket or yours opened — a long-standing gap in an unrelated, older program, not new A3 fallout.

We also checked its neighbors `AssetHandle` (L20–22) and `PluginInstanceId` (L26–28): neither derives `TS` either, but neither is currently reached by any `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]` struct's field, so only `CapabilityToken` actually blocks a build today.

## Patch (one line)

```diff
 #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
+#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
 #[serde(transparent)]
 pub struct CapabilityToken(pub u128);
```

No wire/serde change — `ts_rs::TS` only adds a `.export()`/`decl()`/`dependencies()` associated-fn surface behind the `typegen` feature, exactly like its sibling ids (`ArtifactId`, `SchemaId`, etc.) already have elsewhere in this same file.

## Status

Not blocking our `-p semio-framework`/`-p semio-framework-os-kernel` `cargo test`/`cargo check` acceptance (those don't enable the `typegen` feature). It DOES block `bun nx run @semio-tech/framework:generate`, so `🛂️manifest/🤖️generated/🟦️manifest.ts` is **left unregenerated** in our report — per our own packet's explicit "regenerate via typegen, do not hand-edit" instruction, we are not hand-writing the new generated types into that file as a workaround. We verified our own new types (`ArgSchema`/`ArgFormat`/`ArgPresentation`/`ActionSemantics`/…) compile cleanly under `--features typegen` on their own (after fixing one bug of our own — a stray `ts(optional)` on a `Vec` field, not an `Option` — caught by this same check); this `CapabilityToken` gap is the only remaining error, and it is entirely yours to accept or decline. Will re-run `bun ./📜️script.ts generate` once this lands (or once we're told it won't, in which case we'll say plainly the generated file was never refreshed and TS consumers of the new types are source-level only until it is).

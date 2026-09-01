# 🌱️ Serde Restoration Wave — unblocking `semio-s-plugin-stdio` for `wasm32-wasip2`

## Why this was needed

`s` cannot be rebuilt until `semio-s-plugin-stdio` compiles for `wasm32-wasip2`. That build was red
not inside stdio but in its dependency chain — `replication` → `os-kernel` → `semio-framework` →
`semio-framework-plugin` — because the peer ticket
`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` is mid-sweep: it removes
`serde::Serialize`/`Deserialize` from a type and hand-writes a `ToValue`/`FromValue` twin, but the
type's *consumers* still derive serde. Every such half-converted type is a hard build break.

## What was done

For each type the compiler named, its **full item was spliced back from `67fb4216b2`** (the
pre-conversion commit) — derives *and* field-level `#[serde(...)]` attributes — rather than
hand-adding a bare `#[derive(Serialize, Deserialize)]`. That distinction matters: several types carry
wire-shaping attributes (`rename_all = "camelCase"`, `deny_unknown_fields`, `transparent`,
`skip_serializing_if`) whose loss would silently change the wire format while still compiling. The
restored derives sit *alongside* the peer's hand-written `ToValue`/`FromValue` twins, which is the
transitional state their own `📓️serde-fanout-playbook.md` prescribes ("add alongside, do not
blind-swap").

The splice is guarded: it refuses when the item body differs from the base modulo serde tokens and
doc comments, so a type the peer legitimately *reshaped* is reported and skipped, never reverted.
Tooling: `restore_types.py` + `loop.sh` (restore → re-check → repeat until the error class is gone).

Types restored: `MutationId` `ActorId` `ArtifactId` `SchemaId` `SchemaVersion` `ArtifactVersion`
`PayloadHash` `HybridLogicalTimestamp` `SelectionMode` `DomainSelection` `DomainHover` `MergeMode`
`SelectionMethod` `HierarchyProvider` `HoverSpec` `SelectionSpec` `InteractionState`
`InteractionTarget` `MutationMessage` `MutationMeta` `MutationOrigin` `MutationLeafDescriptor`
`ForeignTarget` `ForeignStep` `Edit` `MutationOutcomeClass` `MutationComposition`
`MutationInvertibility` `MutationDiffParticipation` `MutationLanguageSurface`.

## One substantive correction to the peer's work

`OrderedMap`'s `Serialize` impl (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️component.rs`) had been
narrowed to `#[cfg(test)]` on the stated grounds that its "only repo-wide consumer … is its own
differential test — no production call site anywhere". That premise is **false**: it was read off a
scan of `🧰️framework/🔨️modules` only, which does not cover the os product tree. `Dictionary` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs:26` derives `Serialize` over
an `OrderedMap<Value>` field, so the gate breaks `os-kernel` for `wasm32-wasip2`. The impl was
un-gated and the note corrected in place. Dropping the derive from `Dictionary` instead is not
viable yet — that file still carries 13 serde-derived types (`Value`, `ValueType`, …).

## Live-conflict note

`🎮️mutation/🦀️.rs` is being edited concurrently. During one loop run the same types
(`ForeignTarget`, `MutationMessage`, `Edit`, `MutationMeta`, `MutationOrigin`, the `Mutation*`
enums) needed re-restoring on consecutive rounds — the peer re-removing serde in bursts while this
loop added it back. Confirmed by direct observation rather than inference: sampling the file's mtime
and md5 over 2 min showed it stable and holding 12 `serde::Serialize` derives once their burst
ended. Anyone re-running this should expect to re-apply `restore_types.py` after a peer burst; it is
idempotent and refuses to touch genuinely-reshaped items.

## Result

`semio-s-plugin-stdio` compiles clean for `wasm32-wasip2` — `cargo check -p semio-s-plugin-stdio
--target wasm32-wasip2 --lib --keep-going` exits 0 with 0 errors. This was the gate on rebuilding
`s`. The framework chain reached a stable green first (44 restorations holding with zero churn across
six consecutive attempts, after an earlier period where a peer burst reverted all 44 between rounds),
and the remaining 60 errors inside stdio's own artifact tree were cleared by a six-agent fleet.

### What the fleet found beyond the mechanical conversion

Two findings worth keeping, both root causes rather than symptom fixes:

- **`value_derive` macro hygiene bug** (`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs`). For an
  internally-tagged enum with named fields and no `content` wrapper, the generated code used a bare
  local accumulator named `entries`, then destructured the enum's own fields into the same scope. Any
  variant with a field literally named `entries` — exactly `SemioValue::Map` — had its field shadowed
  by the accumulator, so `to_value(entries)` saw the accumulator's `Vec<(String, DslValue)>` instead
  of the field. rustc blamed the field's span, which is why this read as a call-site type mismatch.
  Fixed by renaming the accumulator to `__out_entries`, matching the `__`-prefix convention the rest
  of the macro already uses for exactly this reason. The sibling `content_entries` accumulator in the
  `Some(content)` arm carries the same latent risk and was deliberately left alone (nothing exercises
  it) rather than fixed speculatively.
- **Missing blanket impls**, not missing per-type impls: `ToValue`/`FromValue` had a 2-tuple impl and
  a `BTreeMap<String, T>` impl but no 3-tuple and no `HashMap`. Six errors across deflate, las and ifc
  were all one gap each. Added next to the trait definition rather than per call site.

### Direction-of-travel note

Roughly half the fleet's slices concluded that restoring serde was the WRONG fix and converted the
consumer instead — because the type's field graph (e.g. pptx's `XmlDocument`/`XmlNode` tree, zip's
`ZipEntry`) had already been fully migrated by a peer, so restoring serde on the outer type would
have dragged serde back down through finished work. The other half restored derives verbatim after
confirming the body was byte-identical to `67fb4216b2`. That split is the useful signal: the correct
fix is not uniform, and it depends on how far the sweep has already reached into each type's fields.

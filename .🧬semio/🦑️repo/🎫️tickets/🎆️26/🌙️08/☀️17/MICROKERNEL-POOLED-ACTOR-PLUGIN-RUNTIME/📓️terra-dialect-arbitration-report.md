# terra / dialect-arbitration — report

Packet: `dialect-arbitration`. Owned paths: `✏️s/🔌️plugins/🌍️gis/**`, `✏️s/🔌️plugins/🏭️process/**`, `✏️s/🔌️plugins/🌀️procedural/**` (excluding `🚪️io/` and `✏️editor/🦀️component.rs`, owned by peer `io-async-signatures`). `🗄️stdio` untouched (off-limits).

## Verdict: **(d) — not actually broken. The audit misread the registry's scope. No code changed.**

`s.stdio.dwg@ac1018/*` is claimed via `.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), …))` in three plugins (`🌀️procedural`, `🌍️gis`, `🏭️process`), exactly as `📓️luna-dialect-audit.md` reported. But the uniqueness rule the audit cites never sees more than one of these claims at once — each plugin is built through its **own, freshly-constructed** `ArtifactDefinitionRegistry`, and nothing in the repo ever merges two plugins' registries together. The "collision" is a false positive.

## 1. Site verification — all three CONFIRMED, zero drift

Re-measured with `python3` `os.walk` over `✏️s/🔌️plugins/` (76 files reference `s.stdio.dwg` across the tree; the emoji-path grep pitfall from prior packets did not recur because I read every hit, not just a count).

| Plugin | File:Line | Status |
|---|---|---|
| 🌀️procedural | `🌀️procedural/🗿️artifacts/🧊️procedural3d/🦀️component.rs:96-97` | **CONFIRMED** — line numbers unchanged from audit |
| 🌍️gis | `🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs:238-239` | **CONFIRMED** — line numbers unchanged from audit |
| 🏭️process | `🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs:1028-1029` | **CONFIRMED** — line numbers unchanged from audit |

All three read verbatim:
```rust
.descriptor(b"s.stdio.dwg@ac1018/*")?
.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)?,
```

**New finding the audit missed**: this pattern is not unique to `dwg`. The same three (plus others) independently `.claim()` several other `s.stdio.*` dialects that are shared across *many* plugins simultaneously — e.g. `s.stdio.svg@1.1/*` is claimed by `🌀️procedural2d` (`🦀️component.rs:80`), `🌍️gis/🗺️gismap` (`:219`), `🎞️animate/🎬️present` (`:342`), and `💡️reasoning/🔌️wires` (`:348`) — **four** independent plugins. `s.stdio.json@rfc8259/*` is claimed by at least ten: `🌊️flow`, `🏭️process3d`, `💡️reasoning/wires`, `🎬️sequence`, `✒️writer`, `🪐️space/home`, `🌀️procedural2d`, `🌍️gis/gismap`, `🌿️vcs`, `🏛️architect/program`. If the registry rule were a global cross-plugin uniqueness check, the fleet would already be broken on a dozen formats, not just `dwg`. It isn't — which is itself strong corroborating evidence for the mechanism finding below.

## 2. The registry rule — quoted, and its real scope

**Rule text** (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:2530-2554`, `ArtifactDefinitionRegistry::register`):
```rust
pub fn register(&mut self, definition: ArtifactDefinition) -> Result<(), ArtifactDefinitionError> {
    definition.validate()?;
    if let Some(existing) = self.definitions.get(definition.identity()) {
        if existing.identical_to(&definition) { return Ok(()); }
        return Err(ArtifactDefinitionError::new("artifact-definition.conflicting-artifact", ...));
    }
    ...
    for capability in definition.capabilities() {
        for claim in capability.claims() {
            if let Some(previous) = self.claims.get(claim) {
                return Err(ArtifactDefinitionError::new("artifact-definition.conflicting-claim", ...));
            }
            ...
        }
    }
    ...
}
```
This part of the audit is accurate in isolation: two `ArtifactDefinition`s registered **into the same `ArtifactDefinitionRegistry` instance** cannot claim the same `(namespace, value)`.

**What the audit never checked: where the instance comes from and how many exist.** Traced every non-test constructor repo-wide (`grep -rn "ArtifactDefinitionRegistry::new" ` over `🧰️framework/` and `✏️s/` — only two real call sites, both in the plugin-assembly crate):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:534`, inside `PluginBuilder::try_build(self)`:
  ```rust
  let mut definitions = ArtifactDefinitionRegistry::new();
  for definition in artifact_definitions { definitions.register(definition)...; }
  for declaration in &artifacts { declaration.preflight(&plugin_id, &mut definitions)?; }
  ```
  `try_build()` is called **once per plugin assembly** (one crate = one `Plugin::builder(id)…try_build()` call — confirmed `🌍️gis/🦀️component.rs`, `🏭️process/🦀️component.rs`, `🌀️procedural/🦀️component.rs` each have exactly one such entry point). A brand-new, empty registry is created at the top of every call.
- `ArtifactDeclaration::preflight` (`🦀️component.rs:3065`) → `self.register_definitions(definitions)` (`:3056-3062`) → `registry.register(self.definition.clone())`. `self.definition` is the single `ArtifactDefinition` this ONE declared artifact (e.g. `gismap`, or `process3d`) built via `ArtifactDeclaration::builder(definition)` (`🦀️component.rs:2692`, called from `gismap/🦀️component.rs:258`, `process3d/🦀️component.rs:1061`, `procedural3d/🦀️component.rs:134`).
- I grepped repo-wide (`.rs` and `.ts`) for any caller of `Plugin::artifact_definitions()` (the only accessor that exposes a built plugin's populated registry, `🦀️component.rs:12640-12641`) and for any code that constructs an `ArtifactDefinitionRegistry` and registers more than one plugin's definitions into it. **Zero hits.** The accessor is dead — nothing anywhere merges two plugins' registries.

So the rule forbids exactly one thing: **two artifacts declared by the *same* plugin assembly claiming the same value.** It says nothing about, and structurally cannot see, two *different* plugins each claiming the same dialect — they are validated through two disjoint, sequentially-discarded registry instances that never share state. The audit's "the registry only needs ONE `.claim()` per dialect to reserve the namespace" (§5) is an assumption about semantics the code does not implement; there is no shared namespace to reserve.

The rule also does not distinguish declaration from reference in the sense the audit worried about (§3.4) — but not because that distinction is unenforced; it's moot, because a plain composer-metadata reference (a `("dialect", "…")` tuple with no accompanying `.claim()` call) never reaches `register()` at all. Only an actual `.claim()` call produces something the registry can conflict on, and `.claim()` calls only conflict within one plugin's own build.

## 3. The decisive comparison: 🗒️note and `s.stdio.svg@1.1/*`

This is where the audit's own evidence, read further, contradicts its conclusion.

`🗒️note/🗿️artifacts/🗒️note/🦀️component.rs:21` still has a `pub fn definition()` that **does** call `.claim()` for `s.stdio.svg@1.1/*`, `s.stdio.dwg@ac1018/*`, and four other stdio dialects (lines 44-49, looped through at line 63). Read in isolation this looks identical to gis/process/procedural's pattern. But:

- `definition()`'s own doc comment (lines 10-16) says outright: *"the capability rows themselves are inert now, kept only because nothing on this pass's boundary reads or removes `definition()`'s callers."*
- I grepped for every call site of `definition()` and of `note::definition` anywhere in the repo: the only call is a **local, unused-by-anything-live** reference — `note`'s plugin root (`🗒️note/🦀️component.rs:28`) builds its artifact exclusively via `.declare_artifact(crate::artifacts::note::artifact())`, and `artifact()` (`🗒️note/🗿️artifacts/🗒️note/🦀️component.rs:80-84`) constructs the **new-tree** `app::declarations::ArtifactDeclaration { kind, localization, standards }` directly — a different Rust type (`semio_framework_plugin::app::declarations::ArtifactDeclaration`, not `semio_framework_plugin::ArtifactDeclaration`) that carries no format claims at all. `definition()` is dead code; its `.claim()` on svg/dwg never executes.
- Note's *live* dwg touchpoint is `DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY }` in its io deserializer (`…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs:13`) — pure routing data, never fed to `ArtifactDefinitionRegistry`.

So note is not, as the audit claimed, an example of "the correct reference pattern that avoids claiming." It's a plugin that has **fully migrated off** the old declaration channel (its own doc comment: `.declare_artifact(…)` "replaces the old `.artifact_kind(…)`/`.artifact(…)`/…"). gis/process/procedural have **not** migrated — they still use the old `.artifact(ArtifactDeclaration::builder(definition()?)…)` channel, where the `.claim()` is live, required, and per-plugin scoped. Both states are internally consistent; neither is broken.

## 4. Why the `.claim()` exists at all (if not to reserve a namespace)

Per note's own comment trail (line ~20-27 of `🗒️note/🗿️artifacts/🗒️note/🦀️component.rs`, describing a bug it hit on the same construct): `PluginBuilder::declare(…).composers(entries)` requires a declared composer *capability* whose `dialect` claim matches **every composer entry's `writes` coordinate** — i.e. the `.claim()` is a **self-consistency check within one plugin** (does this plugin's own declared capability list match its own composer table?), not a cross-plugin exclusivity mechanism. That is exactly what `ArtifactDeclaration::preflight`'s per-declaration checks (`🦀️component.rs:3065-3096`) implement: composer/subset/migration/inference ownership checks, all keyed to `self.kind` (the *artifact's own* kind), never to a fleet-wide namespace.

## 5. Layout's `DwgSnapshot`/`DwgDecodeStatus` drift — different root cause, confirmed still fixed

Re-read `📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs` directly (current tree state, not from the audit's quote). It still carries the fix the audit described: a doc comment explaining the old `SentinelOnly`/`decode_status` field no longer exists on stdio's real R2004+ `DwgSnapshot`, and the live body routes through `svg_to_dwg_bytes` → `decode_dwg` — the same honest pipeline `🗒️note`'s sibling serializer uses. This is unrelated to the arbitration question: it's API drift against stdio's evolved snapshot shape, not a registry claim conflict, and it was already fixed before this packet started (not touched by me — the file is outside my owned paths in any case, since `📏️layout` isn't gis/process/procedural).

## What I changed

**Nothing.** No files in `🌍️gis`, `🏭️process`, or `🌀️procedural` were edited. Removing the "duplicate" `.claim()` calls per the audit's recommended fix would have been actively wrong: it would silently break each plugin's own composer-capability/composer-entry self-consistency check (§4) for zero benefit, since no real conflict exists to resolve.

## Lease-requests

None. No mechanism change is warranted — see verdict.

## Acceptance

**UNRUN.** No code was modified, so there is nothing for `cargo check` to validate against a behavior change; a compile check of unmodified files would only reconfirm the pre-existing green state of files three other packets may be touching concurrently, which is out of scope for this packet's mandate.

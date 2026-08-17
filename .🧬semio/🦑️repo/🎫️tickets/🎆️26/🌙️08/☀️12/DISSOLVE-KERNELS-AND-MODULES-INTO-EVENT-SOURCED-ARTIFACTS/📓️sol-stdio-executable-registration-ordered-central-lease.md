# Stdio Executable-Registration Ordered Central Lease

## Decision

Luna's referrer map proves that the earlier “single joint SCC” framing was incorrect. The implementation must proceed through these ordered ownership boundaries:

1. **Schema and TypeScript contract** — root `📜️script.ts` plus the schema-owned field shape.
2. **Stdio assembly code SCC** — registry plus the artifact roots it imports and the source definitions whose runtime-capability records authorize those roots.
3. **Framework capability API** — only after the current dirty framework owner releases a coherent API state.

The framework files are explicitly excluded from the current lease. No artifact definition or stdio registry source was written by this central lease.

## Central Stage 1: Current Contract State

The root validator has the required schema-first semantics in its current staged state:

- `runtime_capabilities` is an exact required definition field with a typed `id`, category, descriptor, and exact claims list.
- Codecs, mutations, and inferences validate the canonical relation `executable_registration == (status is implemented or verified)` rather than treating every declared/planned leaf as executable.
- Canonical versioned leaves, runtime claim uniqueness, representation claim ownership, and catalog-wide identity completeness are validated.
- Capability ledger counts keep declaration, registration, implementation, and verification separate.

The current root validator fingerprint is:

```text
c35904f0f488f984a0f4781bd2322e59fa26dc6b48d8a7671a1660dfe786a4cc  📜️script.ts
```

Validation executed after the source rehash:

```text
bun ./📜️script.ts stdio quick
[stdio] quick passed (36 artifacts, 40 dialects, 6 codecs).
```

This validates field and status semantics only. It does not claim that native runtime registration or framework compilation is green.

## Observed Evidence, Not Policy

The active glTF artifact definition fingerprint is `001fdce3e211a1dc1cb845eafff136ca18a616cfc70285714e8c7feda2458d78`. Its observed working-tree roster is six codecs, eighteen mutations, and fifteen inferences. Every one is currently `unimplemented` with `executable_registration: false`. These values are evidence for the handoff; they are not an exception or a license to fabricate registrations.

The current registry fingerprint is `51482847fca20f0ee2ef9188866491c4fe5ad608f1112b2887eca3416bd3279f`. The registry's empty executable mapping table means a future `true` leaf row must be accompanied by a real mapping in the same atomic SCC.

## Terra Stage 2: Stdio Assembly SCC Packet

### Lease owner and writable source boundary

One Terra lease owns exactly these source/consumer groups, after rehashing all of them and rereading the applicable `AGENTS.md` files:

1. `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs` — source parsing, catalog validation, executable-mapping completeness, generic runtime capability construction, MIME/extension claim construction, and the focused assembly test.
2. `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` and `✏️s/🔌️plugins/🗄️stdio/🛂️manifest/🦀️component.rs` — only if the registry result shape or assertion surface changes; otherwise they remain referrer/validation paths, not speculative edits.
3. The cataloged runtime artifact roots and their source JSON definitions:

```text
📰️xml, 🗜️deflate, 🎒️zip, 🔣️json, 📊️csv, 📝️md, 🧊️gltf, 🧊️obj,
🟪️stl, ☁️ply, ☁️las, 📐️step, 🖊️dwg, 🖊️dxf, 🎨️svg, 📷️png,
📷️jpg, 🖼️tiff, 📄️pdf, 📜️docx, 🎞️pptx, 📕️xlsx, 💬️bcf,
🎥️mp4, 📼️avi, 🎵️mp3
```

For each listed artifact, the owned source pair is:

```text
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>/🧬️schema/📜️artifact-definition.json
```

4. The ten definition-only artifacts are referrer/read-only validation scope unless the registry's schema contract shows their definition must change:

```text
💾️binary, 📄txt, 🏗️ifc, 🎞️gif, 🖼️bmp, 🧿️semio, 🔊️wav,
🌦️epw, 📑️tsv, 🌐️html
```

The artifact catalog `📇️registry/📇️catalog.json`, root `📜️script.ts`, all framework plugin files, Cargo manifests/lockfiles, taxonomy, launch configuration, and generated paths are **not writable** in Stage 2. The catalog is a read-only complete-root assertion, not an entrypoint for a hand-edited registration list.

### Required implementation result

The lease must make each source runtime record represent a real declaration facet. Its claims must be the same claims consumed by the relevant `ArtifactDeclaration` builder call; a runtime leaf cannot be a disconnected descriptor. The registry must then:

1. Derive the typed framework `ArtifactCapability` from that record with its descriptor and exact claims.
2. Construct representation capabilities with their MIME and extension claims, so format/language registrations can resolve the capability that owns them.
3. Keep `executable_mappings` exactly bijective with rows whose registration boolean is true. A mapping must point to an actual native executable; a no-op marker is invalid.
4. Add a focused source-owned test that drives `artifact_definitions() → artifact_assemblies() → stdio::plugin() → PluginBuilder::try_library()`, and asserts glTF capability identities, claims, and typed executable identities.
5. Preserve definitions with no runtime declaration as definition-only; never promote a source merely to make a count look complete.

The 26 runtime roots form the registry's compilation/referrer SCC because the registry imports every root assembly and each root calls registry assembly helpers. The lease must hold all of them even where a correct solution needs no textual modification in a given root.

## Framework Stage 3: Deferred Blocker

The typed framework owner and its builder are currently dirty and incoherent: the independent map's `cargo check -p semio-s-plugin-stdio --lib` reaches upstream framework registration-plan/builder signature drift before stdio assembly. Stage 2 must not compensate by weakening source validation or adding an stdio-specific adapter.

After the framework owner releases its source hashes, a separate central lease must prove the final framework API accepts and retains the exact typed runtime capability identities consumed by Stage 2. If that API requires a new public contract, record the decision before editing its source. Do not put framework paths in the Terra Stage 2 write set.

## Validation Order

1. Before Stage 2 edit, record the registry/root/definition source hashes and exact artifact roster from the catalog.
2. Run `bun ./📜️script.ts stdio quick` after every schema-record batch.
3. After Stage 2's source and referrer changes, run:

```text
bun ./📜️script.ts stdio quick
bun nx run @semio-tech/stdio-js:test-quick --skip-nx-cache
RUSTC_WRAPPER= cargo check -p semio-s-plugin-stdio --lib
```

4. If Cargo remains blocked by framework drift, capture its exact first framework error and hold Stage 2 release; do not call it a stdio failure or patch around it.
5. Once Stage 3 releases, rerun the focused runtime test, the full stdio assembly path, and:

```text
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
bun ./📜️script.ts verify taxonomy enforce --scope s.stdio.gltf
```

## Lease Result

Stage 1 field semantics are structurally verified. Stage 2 is ready to assign only after the parent records the listed source hashes as its lease baseline; Stage 3 is deliberately blocked on the dirty framework owner. No exception, baseline, compatibility alias, or direct generated edit is allowed.


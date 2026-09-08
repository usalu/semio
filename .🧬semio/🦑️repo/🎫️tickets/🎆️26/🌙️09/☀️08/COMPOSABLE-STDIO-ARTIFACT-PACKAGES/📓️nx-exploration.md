# Nx Exploration: Composable Stdio Artifact Packages

## Confirmed current state

`🗄️stdio` has exactly 36 artifact roots. Every root already owns both a Rust module
(`🦀️.rs`) and a TypeScript facade (`🟦️.ts`), while the only first-party Rust package is
the monolith at `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust`:

- Cargo package: `semio-s-plugin-stdio`, a root-workspace member in `/Cargo.toml`.
- Its glue crate is approximately 900 KB and mounts all artifact files through `#[path]`.
- Its `full-artifact-catalog` feature assembles all 36 definitions and all 26 native codecs.
- The TypeScript package is a 36-export barrel at `📦️packages/🟦️typescript`; it is not
  listed in the root Bun workspaces array, despite having `package.json`.

The artifact roots are:

```text
binary txt json xml csv md deflate zip step ifc las gltf obj stl ply dwg dxf
svg bmp png jpg gif tiff pdf docx pptx xlsx bcf semio mp4 avi mp3 wav epw tsv html
```

`📇️registry/🔣️.json` is already the ordered, schema-owned list of their 36
`🧬️schema/📜️artifact-definition.json` documents. It is the correct aggregate composition
inventory. `📇️registry/🧬️schema/📜️native-codec-factories.json` already supplies the
26-codec subset for native composition.

## Nx behavior that enables the split

The custom workspace Nx plugin is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`.
It discovers every `📋️project.json` and every Cargo manifest, so no Nx plugin change is
needed to create one project per artifact package.

For a Cargo manifest, it automatically creates cacheable `build`, `check`, and `test`
targets. It follows `lib.path`, explicit test targets, and literal Rust `#[path]` mounts to
construct `nativeSources` / `nativeTestSources`. The special ownership rule for a project
below `📦️packages` includes the sibling package-neutral owner tree in the hash. Therefore a
manifest at:

```text
🗿️artifacts/<artifact>/📦️packages/🦀️rust/Cargo.toml
```

will cache against that artifact's real Rust/schema sources while retaining declarations-only
package folders. Direct Cargo path dependencies become Nx dependencies and their precise native
source hashes replace the generic `^nativeSources` input.

`dist/build` is the only cached build output. The Cargo runner stages Cargo's declared final
artifacts there and deliberately does not cache `target/`. This is the desired boundary:
Nx restores packaged deliverables, while Cargo and sccache retain compiler-level reuse.

Cargo targets currently have `parallelism: false` because every target shares the workspace
Cargo target directory. Keep that safety rule for the first split: it avoids Cargo-lock stalls
while still removing the all-artifact compilation frontier. Do not assign each artifact an
unrelated `CARGO_TARGET_DIR` just to get Nx parallelism; that duplicates dependency compilation.
Parallel execution can be reconsidered only after a measured, shared-cache-safe target-store
design exists.

## Required package topology

For every artifact `<id>`, add declarations only at:

```text
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<emoji-id>/
  📦️packages/
    🦀️rust/Cargo.toml
    🦀️rust/📋️project.json
    🦀️rust/📜️script.ts
    🟦️typescript/package.json
    🟦️typescript/📋️project.json
    🟦️typescript/📜️script.ts
```

The Cargo `lib.path` must point out of `📦️packages` to a Rust package root in the same
artifact taxonomy tree; TypeScript exports must similarly point to that artifact's existing
`🟦️.ts`. Package folders must not receive Rust/TypeScript implementation files. Package names
should be mechanically derived from the schema artifact identity, for example
`s.stdio.pdf` -> `semio-s-artifact-stdio-pdf` and `@semio-tech/stdio-pdf`.

Keep the existing implementation-local `[workspace]` Cargo manifests below `🏭️generator` and
`🔬️probes` unchanged. They are independent third-party/oracle executables, not the first-party
artifact package being introduced.

The existing plugin package becomes a composition package. Its Cargo manifest depends on
artifact crates behind one feature per artifact; `full-artifact-catalog` is the explicit
all-36 feature. Consumers select a minimal feature closure, and the plugin's component target
uses exactly the selected closure. The registry's full-catalog validation remains only on the
all-36 composition feature. Existing consumers that request `full-artifact-catalog` keep the
same semantic result until they choose a smaller composition.

## Implementation constraint discovered

Pointing a new Cargo `lib.path` directly at today's artifact `🦀️.rs` cannot work yet. For
example, `png`, `pdf`, and `json` call `crate::registry::{runtime_assembly,
format_descriptors_for}` and use `crate::artifacts::<artifact>::...`. The current registry
itself constructs the complete 36-artifact catalog. Direct-path manifests would therefore
either fail or pull the monolith back into every artifact build.

Refactor the current package wiring into package-neutral taxonomy code before declaring the
artifact manifests:

1. Make each artifact's existing root a self-contained crate root: own `standards` tree plus
   artifact-local assembly/format helper interfaces.
2. Split the all-artifact registry into an artifact-local registry projection and a separate
   composition registry. The local projection may read only that artifact definition and its
   own codec receipt; the composition projection enumerates selected artifact crates.
3. Move the current monolithic plugin glue out of `📦️packages/🦀️rust/🦀️.rs` into the stdio
   taxonomy tree and make it a thin feature-gated composition of package dependencies.
4. Preserve cross-artifact codecs as explicit Cargo dependencies. This lets Nx track their
   true closure and prevents hidden `crate::artifacts` coupling.

That satisfies the declarations-only package rule and creates real independent compilation
units instead of 36 aliases for one crate.

## Schema-first inventory and validation

Do not hand-maintain a second artifact package list. Extend each artifact's existing
`🧬️schema/📜️artifact-definition.json` schema with its package identity and optional
composition capability, then use `📇️registry/🔣️.json` as the ordered composition selector.
Add a schema beside `📇️registry/🧬️schema/` for that composition projection. The generated
validation must prove all of the following:

1. The registry references each of the 36 schema documents exactly once.
2. Every definition yields exactly one Rust and one TypeScript package declaration at the
   canonical topology above, with names derived from its identity.
3. The Cargo workspace member list and Bun workspace list equal that schema inventory.
4. A selected composition's Cargo features, Rust exports, TypeScript barrel exports, definitions,
   and native codec receipts have the same ordered set.

The generator belongs in the existing root `📜️script.ts` router, with the package-local
`📜️script.ts` files delegating only their own `check`, `test`, and composition commands. This
keeps `📋️project.json` as an Nx command declaration and follows the repository script rule.

## Test and launch integration

Use the repository's language-agnostic test system rather than test-only manifest assertions.
It creates one virtual Nx project for each `🧪️tests/**/*.feature` case and caches durable results
under `.🧬semio/🦑️repo/⚡️cache/tests/tasks`. Add a package-bound feature that selects one
artifact, builds its Rust and TypeScript package, and verifies its definition/codec projection.
Use existing `🏭️generator` and `🔬️probes` crates as the required third-party oracle for at
least one selected codec result; they intentionally share no first-party implementation code.

Add one composition feature that selects two artifacts with a known dependency and verifies
that Nx's project graph exposes the edge and the composed registry contains exactly those two
definitions. Add an all-catalog feature test that preserves the current 36-definition,
26-codec commitment. This validates both selective and full composition without reintroducing
a monolithic artifact build as the normal case.

Register launch configurations in the existing Nx lifecycle/cache section of
`.vscode/launch.json`, in its `4_build` and `4_gate` ordering: per-artifact build/check/test
commands and a selected-composition build command. Generate that block from the schema inventory
through the root script so the 36 entries cannot drift from the registry.

## Files that execution must coordinate

- `/Cargo.toml` — add the 36 Rust package members and replace the monolith-only membership.
- `/package.json` — add the 36 TypeScript workspaces plus the composed package if retained.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📋️project.json,📜️script.ts}` —
  transform the present all-artifact package into feature-gated composition.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🟦️typescript/{package.json,🟦️.ts,📋️project.json,📜️script.ts}` —
  make it the selected-composition barrel.
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/{🔣️.json,🦀️.rs}` and its schema directory — inventory
  and split registry responsibilities.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**` — artifact-local package roots and declarations.
- `/📜️script.ts` and `/.vscode/launch.json` — schema-driven validation/generation and commands.

No commands were executed for this read-only audit; no build/test result is claimed.

# The Last Six — Report

Real, pasted, scope `🧰️framework/🔨️modules/🖼️assets`, baseline `bb06c41f73f0122fbed315b7487428b976f99921`:

```
before: moves=1089 edits=54 unresolved=6
after:  moves=1089 edits=54 unresolved=5
```

**1 cleared, 5 remain. Not unresolved=0 — I cannot say apply is unblocked for this scope.**

**(B) obj singleton — FIXED, verified.** `ORIGINAL_UNKNOWN_STATEMENTS` in `mutate-obj-3-0/🦀️.rs` was a
7-line duplicate of the fixture's own `#`-comment lines (grep-verified: the fixture has exactly 7
lines starting with `#`, 1:1, in order). Replaced with `original_unknown_statements(base)`, filtering
`base` (already-loaded pristine fixture bytes, in scope at the call site) for `#`-prefixed lines — no
subject-crate link, same I/O pattern `mutable_input` already uses. The row disappeared with **zero
new `edits` entries** (54→54): elimination, not exemption — the token no longer exists in source, the
scanner is untouched. Rejected the sketched `frozen-rust-source-coordinates-v1` grammar as
disproportionate for one row. rustfmt-clean at my edit region (repo's own `rustfmt.toml`).

**(A) wgpu bundle, 4 rows — diagnosed, NOT fixed. The frame is not simply "unregistered."** A
`wgpu-frame-worker` generator contract already exists with `outputRoots` for both bundles — but all
six of its output paths (verified: none exist on disk) are the POST-31-move destination, and its own
`reason` field says so explicitly: "first generation requires the complete 31-move ... projection."
Registering the CURRENT path wouldn't help either way: `generatorPlanning`'s
`packageOutputVerification` only activates a package contract when the plan's `--scope` overlaps that
package's own root (renderer/os, not assets), and none of the asset paths are in this contract's fixed
`inputPatterns`. Traced the actual token's origin: it flows from `🔌️plugin/📇️registry/🤖️generated/
🟦️playgrounds.ts` (an output of `plugin-registry`, which IS already regenerating in this plan) through
Bun's bundler into `frame-worker.js` — a two-hop generator chain this scope-limited plan doesn't
model. And the regeneration command itself (`generate-wgpu`, `Bun.build` on `frame-worker.ts`) is
blocked by the same pre-existing `playwright-core`/Node-builtins break already on record — confirmed
that break sits ON this exact target, not a separate/broader one. Two independent, stacked blockers;
neither is assets-scope work.

**(C) python row — diagnosed, hypothesis DISPROVEN, reverted.** Tested "narrow
`historicalEvidenceBoundaryOwns` to the manifest's own ecosystem extensions" (schema-driven, using
`fixedFilenameContracts.scope.ecosystemId` + `fileKinds`). It broke a real, deliberate, already-passing
test: `…/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/generate_w1_a_gltf_create_scene.mjs`,
a `.mjs` sibling of that ticket's `Cargo.toml`, is a genuinely live generator script joining
`process.cwd()` with a real repo path and writing generated source at run time. `extract_positions.py`
has the identical shape: `SRC = "🧰️.../🧊️pattern-sphere.glb"; open(SRC, "rb")` is real, load-bearing
I/O, not dead prose. "Shares a directory with a manifest" is the deliberately safe, already-tested
boundary — narrowing it by extension is wrong, would have been a silent regression. Reverted the code
change; kept only a docstring recording why, so the next session doesn't retry it. The real fix is
Python-adapter rewritability (teach `pythonTokens` the `NAME = "path"` → `open(NAME)` indirection,
alongside its existing `VECTOR_ROOT`/`stem` special cases) — sized, not attempted this slice.

Files touched: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🧪️tests/mutate-obj-3-0/🦀️.rs` (real fix);
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` (docstring only, net
behavior unchanged, reverted after disproof).

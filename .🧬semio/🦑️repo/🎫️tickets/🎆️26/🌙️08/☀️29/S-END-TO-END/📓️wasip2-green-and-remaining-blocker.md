# ✅️ `wasm32-wasip2` compiles clean — one non-code blocker remains

## Where this stands

`cargo build -p semio-s-plugin-space --target wasm32-wasip2 --lib` now reports **zero compile
errors** across the entire dependency graph: `replication`, `os-kernel`, `os`, `os-infinite`,
`os-kernel-neural-engine`, `ui`, `ui-contract`, `pack`, `framework`, `framework-plugin`,
`s-plugin-stdio`, and `s-plugin-space` itself. That is the whole Rust side of "get `s` working end to
end".

The build still exits non-zero, but **not** from any Rust code. `semio-framework-graph`'s build
script runs the repo's taxonomy validator, and one generator contract fails it:

```
generatorContracts["wgpu-frame-worker"] tracked output
  ".../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🏗️builder/🦀️.rs" is missing.
  ".../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/💾️binary/🦀️.rs" is missing.
```

## Why that message is misleading, and what actually blocks it

The "tracked output is missing" wording is a red herring. Those outputs are *allowed* to be missing —
`nestedCargoGeneratedPrestate` exists precisely to tolerate a not-yet-materialized projection. It is
returning `false`, and it does so inside a bare `catch { return false; }`, so the real reason never
reaches the log. Probing it directly (`🔬️probe-wgpu-prestate.ts`) shows the cause:

`semanticPackageSourceOutputPhase` requires the set of git-admitted files under the projection's
source root to equal the catalog's mapping set exactly. It does not. One file differs:

```
in git but NOT in catalog:
  + .../📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🌐️index.html
```

`git status` reports that file as `" D"` — **deleted in the working tree but still present in the
index**, and its catalog mappings have already been removed. So a peer is mid-way through deleting
it: the working-tree half and the catalog half are done, the index half is not. `git ls-files
--cached` still lists it, so the sets cannot match.

**The only thing that resolves this is staging that deletion**, which is a git index operation. This
repo's `CLAUDE.md` forbids modifying git state from a session, precisely because several sessions
share this tree and an auto-commit is running. So this one is deliberately left for whoever is doing
the deletion. It is a single `git rm --cached` away and needs no code change.

## The tamper-evident chain, and the tool for it

Getting the other three contracts (`jco-package-adapter`, `external-cargo-locks`, and the catalog
itself) to pass meant repairing a chain that the concurrent rename and `Number` sweeps had left
inconsistent. It has three layers, each pinning the next:

1. `🔣️taxonomy.json` → `semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogSha256` pins the catalog fixture's bytes.
2. The catalog fixture pins a `sourceHash` + `sourceSize` for every mapped source file.
3. A derived registration leaf pins the `originSourceHash` of the source it was derived from, which must agree with layer 2.

Editing a mapped source without re-recording all three makes `semanticPackageGenerationAuthority`
throw — and every one of those throws is swallowed by the same bare `catch`, surfacing only as an
unrelated-looking "tracked output is missing" while hard-failing **every cargo command repo-wide**.
Three different real errors (`source preimage drift`, `catalog digest drift`, `derived registration
authority drift`) all presented identically.

`🔁️rerecord-projection-preimages.py` in this ticket folder re-records all three layers. It is
idempotent — run it after any edit to a projection-mapped source, and it prints a zero count when the
chain is already consistent.

## Corrections made to in-flight peer work

Two narrowings rested on the same mistaken premise and had to be undone, both documented in place:

- `OrderedMap`'s `Serialize` (`🌱️value/🗂️ordered/🦀️.rs`) was gated `#[cfg(test)]` on the grounds that
  its only consumer, `Dictionary`, had itself gone test-only.
- `Dictionary`'s `Serialize` (`💻️os/🧠️neural/⚙️engine/🦀️.rs`) was gated the same way.

`Dictionary` is not yet convertible: three production sites in that same file still require it — the
`Value` enum's derive (it has a `Dictionary` variant), `Neuron`'s derive (it holds a `Value`), and
the `serde_json::to_string(&merged)` call in the evaluator's pending-extension branch. Both impls are
unconditional again, and both notes now say what has to move before they can be re-gated together.

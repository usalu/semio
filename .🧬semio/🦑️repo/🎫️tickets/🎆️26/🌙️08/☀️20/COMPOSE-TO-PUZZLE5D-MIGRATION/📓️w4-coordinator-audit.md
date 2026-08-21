# Wave 4 — coordinator audit of the fleet's fixtures

Run by the coordinator against the whole repo, independently of `fixtures lint` and independently of
each lane's own self-report.

## 1 · File completeness — walks `git ls-files`, not the lint
```
mutation leaves            : 1558
leaves with no 🧪️tests     : 0
test cases                 : 1558   (applied 1487 · rejected 71)
incomplete cases           : 0
byte-identical test bodies : 0 in 0 groups
```
**FINAL: every mutation leaf in the repository has at least one handcrafted test case, and every
case is file-complete.**
Every case carries all five core files; every applied case carries `🔺️diff/🔣️component.json`; every
rejected case carries `🔺️diff/🚫️component.absent`. **0 incomplete cases.**

## 2 · Are the tests actually handcrafted? — byte-identity check
All 1558 test files hashed. **0 byte-identical pairs, 0 duplicate groups.**
No lane templated its output, which was the main risk of fanning this out to a fleet.

## 3 · Do the committed diffs match their builders? — static cross-check
For every leaf whose `🔺️diff/🦀️component.rs` builds a statically-parseable `…Diff { … }` literal,
the committed diff JSON's non-null key set was compared against the fields that builder can set.
```
cases cross-checked: 1131
non-null keys not constructible by the builder: 8
leaves skipped (builder shape not statically parseable): 365
```
All 8 were inspected and are **false positives of the checker's regex**, not fixture errors: each is
a cascade that sets a second collection through a conditional
(`fasteners: if severed.is_empty() { None } else { Some(...) }`), which the pattern under-detects.
`🗑delete-part` and `➖remove-part-grip` were confirmed by reading their builders directly — both do
set `parts` **and** `fasteners`, exactly as the fixtures say. Same shape for remodel's
`🪓delete-stream` (`streams` + `gcps`), puzzle3d's `🗑delete-node` / `➖remove-node-handle`, and
procedural's `🎲change-seed`.

**Genuine mismatches: 0.**

## 4 · Do rejected cases declare a code their own builder raises?
```
rejected cases whose declared code IS raised by its own diff builder: 66
not found in the builder: 1
```
The one outlier, flow's `👯️duplicate-widget`, is correct: it is a **composite** leaf with a
`🧩️plan` directory instead of a `🔺️diff`, and a `PlanError` folds into a Fatal
`mutation.invariant`. Its test asserts precisely that, with a comment explaining why the code differs
from the `mutation.duplicate-id` a bare `create-widget` would raise. The checker's assumption that
every leaf has a `🔺️diff` is what failed, not the fixture.

**Genuine mismatches: 0 of 67.**

## 5 · Production code the fleet touched, and why
Wiring files aside (per-plugin `📦️glue.rs`, per-artifact mutations-root `🦀️component.rs`), only two
substantive production changes were made, both by lanes that reported them:
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️component.rs` — extracted `cache_forms_steps`, the
  id-keyed twin of `forms_children_from_steps`, mirroring dag's `cache_dag_content` and playbook's
  `cache_playbook_steps`. Pure refactor plus one new public helper; without it no persisted forms
  handle resolves and `create-step` had no authorable case.
- `🌿️vcs` / `🖍️draw` mutations roots — added `apply_*_mutation` / `inverse_*_mutation` entry points,
  verbatim in dag's existing shape, because neither artifact had one.

`Cargo.toml` also shows as modified — that is the **peer session's** `semio-framework-job` addition
from `INTERACTIVE-JOB-RUNTIME-REFACTOR`, staged inside their 864-file changeset. Not this ticket's.

## 6 · What is still NOT verified
`cargo` remains unusable (`semio-framework-os-infinite`, `semio-s-plugin-stdio` broken by the peer's
de-async sweep). **No test has been executed. None is claimed to pass.** Everything above is
structural and transcription-level verification only.

---

## 7 · ⚠️ On disk ≠ in the build — 113 tests are NOT wired

A test file that no module tree references is never compiled and never runs. Audited by collecting
every `#[path = "…🧪️tests…"]` in every `.rs` in the repo and matching it against the cases on disk:

```
test cases wired into a module tree      : 1445
test cases NOT wired (won't compile/run) :  113   — all in 🗄️stdio
```

All 113 are `🗄️stdio/🧊️gltf`. The cause is pre-existing and not something this ticket created:

- gltf has **130 leaf directories** but `📦️glue.rs` mounts only **42** paths from that tree, and only
  **7 leaves** are mounted as production modules. The other ~113 leaves are **dead code that is not in
  the crate's module tree at all**.
- Several of those unmounted leaves **cannot compile as written**. Verified directly:
  `bind-primitive-indices/🦠️mutation/🦀️component.rs:12` references
  `crate::artifacts::gltf::engine::GltfComponentType::F32`, but that enum
  (`🚪️io/🦀️component.rs:85`) declares only `Byte`, `UnsignedByte`, `Short`, `UnsignedShort`,
  `UnsignedInt`, `Float` — **there is no `F32` variant**. Nobody noticed because the leaf is never
  compiled. The lane also reported `top_level_collections_private` vs glue's `top_level_private`,
  an undefined local `diff` in three inverses, and `GltfCameraProjection` deriving no serde impls.

The lane mounted test modules for the 7 leaves that ARE in the module tree and deliberately left the
rest unmounted, because mounting them would drag the broken leaves in and **break `cargo test` for
the whole of `semio-s-plugin-stdio`**. That was the correct call: the alternative is fixing
pre-existing production defects in a plugin a peer session is concurrently sweeping.

The 113 test files are written against their leaves' real entry points and become live the moment
their leaves are wired — but until then they are inert.

### Honest statement of coverage
- **1558 / 1558** mutation leaves have a handcrafted, file-complete test case. ✅
- **1445 / 1558** of those tests are in the build and will run once the workspace compiles.
- **113 / 1558** are inert pending gltf's dead leaves being wired and repaired — a separate task,
  and a pre-existing repo defect this exercise surfaced rather than introduced.

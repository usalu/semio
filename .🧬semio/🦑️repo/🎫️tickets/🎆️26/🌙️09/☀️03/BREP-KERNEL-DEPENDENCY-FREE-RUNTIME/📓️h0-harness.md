# 🔬️ H0 — Standalone Brep Kernel Test Harness

**STATUS: READY — now mounts the FULL `⚙️engine/🦀️.rs` (Brep/BrepKernel façade). `cargo check --lib`
surfaces the real, root-workspace-equivalent `✳️brep` errors (5, all itemized below); `cargo test`
adds 5 more (10 total) from `#[cfg(test)]`-only code paths. Already adopted: W1-B, W1-D1, W1-E
have their own `🗑️generated/w1{b,d1,e}-harness-*.txt` runs against this harness.**

`TICKET/🔬️harness/` is a standalone Cargo package (own `[workspace]`, own `.cargo/config.toml`
target-dir) that mounts the REAL `✳️brep` (+ the `✳️base` slice it needs) source files verbatim via
`#[path]`. It is not a member of the root workspace, so it never touches the root `Cargo.lock` or
target-dir lock — proven necessary twice now: while building this, four other concurrent workers'
own build attempts against the ROOT workspace were observed sitting on `Blocking waiting for file
lock on build directory` (their own `w1a-check.txt` / `w1a-check-fw3d.txt` / `w1d2-check-early.txt`
/ `w1h-check.txt`), and the coordinator's follow-up reports 20+ duplicate root `cargo check -p
semio-s-plugin-stdio` runs queued on the shared lock, each 10+ minutes.

## How to run

```bash
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME/🔬️harness"
RUSTC_WRAPPER="" cargo check --lib --message-format short   # fastest — production code only, no #[cfg(test)]
RUSTC_WRAPPER="" cargo check --message-format short         # includes #[cfg(test)] mod tests
RUSTC_WRAPPER="" cargo test                                 # build + run
RUSTC_WRAPPER="" cargo test -- <module>::<test_name>         # e.g. `-- classification::tests::`
```

`.cargo/config.toml` pins `CARGO_TARGET_DIR` to `TICKET/🗑️generated/harness-target` so runs never
contend the root workspace's target dir either. Always pass `RUSTC_WRAPPER=""` (sccache serializes
concurrent builds across ALL sessions, isolated target-dir or not).

## What is mounted (verbatim `#[path]`, never copied)

`crate::artifacts::semio::standards::v1::subsets::brep::schema::`
- **`engine`** — the FULL `⚙️engine/🦀️.rs` (`Brep`/`BrepKernel`/`GeometryHandle`/`Entity`/…),
  widened from contract-only per the coordinator's follow-up. Self-mounts its own `mesh_io`
  (`📦️mesh-io/🦀️.rs`), `step` (`📄️step/🦀️.rs`) and `contract` (`🔖️contract/🦀️.rs`) submodules
  via the file's own internal relative `#[path]`s — same single-line mount the real crate root
  uses.
- `snapshot::{vector, curve, polynomial, surface, arena, tolerance, error, topology}`
- `diff::{primitives, boolean, euler, intersect, offset, blend, sweep, transform, sew}` —
  `transform` is W1-B's new `🔁️transform` module (self-contained kernel imports only, mounted
  clean). `sew` is mounted because `engine.rs` itself `use`s `diff::sew::{convert_to_nurbs,
  defeature, heal_solid, sew_faces}` — the module must exist for that import to resolve at the
  module-path level, even though `sew.rs`'s own `heal_solid` still calls `validate_body` (not
  mounted — see "Known failing").
- `inferences::{classification, bounding_volume, mass_properties, tessellation}`

`crate::artifacts::semio::standards::v1::subsets::base::schema::{geometry, triples}` — only these
two; see "Not mounted" for why the rest of `✳️base` stays out.

Dependencies added this pass (`🔬️harness/Cargo.toml`): `semio-framework-hash`
(`GeometryHandle`'s `semio_framework_hash::hash_bytes` in engine.rs), `semio-framework-mesh-engine`
(`mesh_io.rs`'s OBJ/STL/GLB helpers), `serde_json` moved from dev- to a real dependency
(`engine.rs`'s `validate_sync` builds a `serde_json::json!({…})` report in real, non-test code).
Previously-added: `semio-framework-number`, `semio-framework-os-kernel`,
`semio-framework-value-derive`, `serde` (geometry.rs's `SemioTransform`); dev-only
`semio-framework-async-macros` (`#[async_test]`, incl. `engine.rs`'s own async test suite),
`semio-framework-geometry`, `pack` (`semio-framework-pack`, aliased like the real stdio root).

`extern crate semio_framework_os_kernel as {dsl, protocol, store};` and
`extern crate semio_framework_value_derive as value_derive;` mirror the real stdio crate root's
aliases.

**Deliberately left UNRESOLVED (two symbols), by design, not oversight:**
- `crate::artifacts::dwg` (`mesh_io.rs:22`) — `dwg` itself needs `crate::registry`
  (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs`), and `registry.rs` references ~69 OTHER artifact
  types (`crate::artifacts::gltf`, `binary`, `txt`, `xml`, `deflate`, `zip`, `json`, `csv`, …) —
  confirmed by inspection, not mounted. This produces exactly ONE localized `E0432` inside
  `mesh_io.rs`'s own functions (`export_solid_dwg`/`import_dwg_to_body`); it does NOT block the
  rest of `engine.rs` — the target `Entity`/`BrepKernel` code still typechecks independently in
  the SAME `cargo check` run (see "Known failing" below — this was the hypothesis tested and
  confirmed this session).
- `crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::validation_report`
  — see "Not mounted" for the full cascade (SemioBrepSnapshot/schema::ArtifactSchema/io/STEP-
  artifact/plugin). Produces one localized `E0432` at each of its 4 real `use` sites
  (`engine.rs:62`, `sew.rs:14`, plus `euler.rs`/`sweep.rs`/`primitives.rs`'s own `#[cfg(test)]`
  modules under `cargo test`). Same non-blocking behavior confirmed.

## Not mounted, and why

1. **`💡️inferences/✅validation-report/🦀️.rs`** (`validate_body`) — its home file ALSO implements
   `store::InferredField<SemioBrepSnapshot>`, pulling: the artifact-layer `SemioBrepSnapshot`
   (needs `schema::ArtifactSchema` + more of `base::schema` than geometry/triples),
   `brep::io::check_brep_referential_integrity` (needs `semio_framework_plugin` AND the brep-owned
   STEP serializers, which need the SEPARATE standalone `crate::artifacts::step` artifact). Tried
   mounting this whole chain in the prior pass: `semio-framework-{os-kernel,schema,plugin}`
   compiled fine standalone, but `✳️base`'s own `io/🦀️.rs`/`📸️snapshot/🦀️.rs`/`🔺️diff/🦀️.rs`/
   `🧬️mutations/🦀️.rs` (pulled in because `base::schema`'s component root re-exports them) is a
   generic cross-artifact composition registry referencing EVERY other semio subset — ~180
   further unresolved-module errors. Reverted. **Recommendation for W1-F / the ticket lead:**
   split `validate_body` (pure `fn(&Body) -> Vec<ValidationIssue>`, no `SemioBrepSnapshot` in its
   signature) into its own kernel-scope file, decoupled from the `InferredField` wrapper — this
   would let both this harness AND the real crate's kernel layer stop depending on the STEP/
   plugin/schema stack for a pure structural-invariant check.
2. **`crate::artifacts::dwg`** (needed only by `mesh_io.rs`) — needs `crate::registry`, which
   needs ~69 other artifact types. Left unresolved (localized failure, see above) rather than
   chased further — mounting `registry.rs` would pollute the error output with dozens of
   completely unrelated errors (gltf/binary/txt/xml/…), making it HARDER to eyeball "is this the
   same brep error the root check shows."
3. `🚪️io/🦀️.rs`, `🧬️mutations/🦀️.rs` — only reachable via the validation-report/dwg chains above;
   not needed by anything else in scope.
4. `viewer/**`, `editor/**` (Wave 3A) and the flow extension crate
   (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep`) — never referenced by anything above.

None of the above reintroduces the ROOT workspace's actual blocker (~1200 E0277 from the
serde-elimination wave in OTHER subsets — json/xml/zip/txt/binary/audio/animation/…, see
`🗑️generated/baseline-check-stdio-2.txt`): none of those subsets are reachable from this harness.

## Known failing / current errors (snapshot 2026-09-03 18:00 CEST — live 9-worker tree, re-run)

`cargo check --lib` — **5 errors**:

| file:line | code | what | owner |
|---|---|---|---|
| `⚙️engine/📦️mesh-io/🦀️.rs:22` | E0432 | `crate::artifacts::dwg` unresolved | by design (harness scope, see above) |
| `⚙️engine/🦀️.rs:1267` | E0023 | `Entity::Curve(_)` pattern: 1 field given, variant now has 2 | **W1-C** (Entity enum shape change not yet propagated to `closest_point_sync`) |
| `⚙️engine/🦀️.rs:1271` | E0023 | `Entity::Surface(_)` pattern: same arity mismatch | **W1-C**, same fn |
| `⚙️engine/🦀️.rs:62` | E0432 | `inferences::validation_report` unresolved | by design (see "Not mounted" §1) |
| `🔺️diff/🧵️sew/🦀️.rs:14` | E0432 | same | by design |

The two `E0023`s ARE the "`Entity::Wire(..)` arity" class of error the coordinator flagged — this
run confirms the hypothesis that mounting `engine.rs` in full, even with the two intentionally-
unresolved imports present, surfaces W1-C's real cross-file breakage (the unresolved imports each
produce exactly one contained `E0432`, they do not suppress or shadow unrelated typecheck errors
elsewhere in the same file — rustc checks every item independently). Re-run for the current line
numbers/count if W1-C has since fixed the `Entity` call sites — this is a live tree.

`cargo test` adds these 5 more (10 total), all inside `#[cfg(test)]`:
- `🔺️diff/{🔺️euler:426, ➡️sweep:439, 🧱️primitives:752}` — `use …validation_report::…` in their
  own test modules (E0432, same root cause as above).
- `📸️snapshot/🕸️topology/🦀️.rs:752-753` — `serde_round_trips_a_whole_body` test needs
  `Body: Serialize`/`Deserialize`; `Body` currently derives only `value_derive::ToValue`/
  `FromValue` (E0277). Pre-existing gap from the serde-elimination wave, not a harness issue —
  flag to whoever owns that wave.

The transient W1-D2 `closest_point`/`closest_uv`/`orient2d` breakage reported in the previous
version of this file (across `curve-ops.rs`, `surface-ops.rs`, `mass-properties.rs`,
`classification.rs`, `tessellation.rs`) has been RESOLVED since — none of those errors reproduce
in this run. Confirms the harness reflects live fixes correctly.

## Rules followed

Never ran a git write command. Never touched `.🧬semio/🦑️repo/🎫️tickets/…/🎫️ticket.json` (no
close/reopen). No source file outside `TICKET/🔬️harness/` was edited. All build logs are under
`TICKET/🗑️generated/` (`harness-check-engine2.txt`, `harness-test-engine1.txt` are this pass's
final evidence; superseded intermediate ones from both this and the prior pass were deleted). Ran
every `cargo check`/`cargo test` in the FOREGROUND with the Bash `timeout: 600000` parameter, per
the coordinator's explicit instruction — never left work running in the background.

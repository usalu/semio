# `app_commands!` Fleet Re-audit

Date: 2026-09-02

## Outcome

The reported serde regression class is closed in the current repository state. All three
`semio_framework_plugin::app_commands!` arms derive only `Clone`, `Debug`, `PartialEq`, first-party
`ToValue`/`FromValue`, and `dsl::DslOps`. No serde JSON wire exists in the generated implementation,
so a serde opt-out would be the wrong API: serde is globally absent from the macro contract.

The removal was introduced by `f15dbf0d83a` on 2026-09-01 and remains present in all three macro
arms. This re-audit adds a stronger
compile-time regression fixture: all four fake payload types in `app_commands_tests` now omit
`Serialize`/`Deserialize`. Reintroducing serde derives on any generated enum arm will therefore fail
the framework test build because its wrapped payloads do not implement those traits.

## Current Fleet Inventory

The original textual count conflated mentions with expansions. Current Rust source has 146 files
containing the string `app_commands!`, but only 50 files match the invocation form
`app_commands!\s*\{`. Those 50 expansions belong to 29 plugin crates.

Every one of the 29 owning manifests currently has no direct `serde` dependency:

| Expansions | Package |
| ---: | --- |
| 15 | `semio-s-plugin-norm` |
| 3 | `semio-s-plugin-block` |
| 3 | `semio-s-plugin-space` |
| 2 | `semio-s-plugin-fem` |
| 2 | `semio-s-plugin-gis` |
| 2 | `semio-s-plugin-procedural` |
| 1 | `semio-s-plugin-animate` |
| 1 | `semio-s-plugin-architect` |
| 1 | `semio-s-plugin-cad` |
| 1 | `semio-s-plugin-dag` |
| 1 | `semio-s-plugin-demonstrator` |
| 1 | `semio-s-plugin-draw` |
| 1 | `semio-s-plugin-flow` |
| 1 | `semio-s-plugin-forms` |
| 1 | `semio-s-plugin-imperative` |
| 1 | `semio-s-plugin-layout` |
| 1 | `semio-s-plugin-lowpoly` |
| 1 | `semio-s-plugin-mathematical` |
| 1 | `semio-s-plugin-note` |
| 1 | `semio-s-plugin-playbook` |
| 1 | `semio-s-plugin-process` |
| 1 | `semio-s-plugin-raster` |
| 1 | `semio-s-plugin-reasoning-mindmap` |
| 1 | `semio-s-plugin-remodel` |
| 1 | `semio-s-plugin-sequence` |
| 1 | `semio-s-plugin-shooting` |
| 1 | `semio-s-plugin-sourcing` |
| 1 | `semio-s-plugin-vcs` |
| 1 | `semio-s-plugin-writer` |

`semio-s-plugin-procedural` alone retains a direct `serde_json` dependency, for a boundary unrelated
to `app_commands!`; none of the other 28 invoking manifests declares either serde-family crate.

The reproducible inventory lives in the ticket's `📜️script.ts` and runs with:

```text
bun 📜️script.ts audit
```

## Mathematical and FEM State

Both acceptance crates are now fully past the workaround state described by
`📓️final-plugin-manifests.md`:

- `semio-s-plugin-mathematical`: no direct `serde` or `serde_json` manifest dependency and no
  macro-imposed serde requirement. The command payloads temporarily restored for the old macro
  contract are first-party-codec-only again. A nested development generator retains its own
  independent JSON implementation; it is not part of the invoking plugin crate's manifest.
- `semio-s-plugin-fem`: no direct `serde` or `serde_json` manifest dependency and no production
  serde derive/attribute/API reference. The 37 payload structs plus `FemDof` and
  `FemCombinationTerm` are first-party-codec-only again.

The first FEM check also exposed a separate real stale-dependency regression outside
`app_commands!`: the shared FEM engine still imported/derived `serde` and called `serde_json` for
job checkpoints and previews even though the FEM plugin manifest declared neither crate. The
engine now uses its existing first-party `ToValue`/`FromValue` and packed wire-value path for the
persisted checkpoint/preview types. Internal types that are not persisted no longer carry stale
serde derives. A source scan of `✏️s/🔨️modules/🏗️fem/⚙️engine` now finds no `serde::`, `serde_json::`,
`#[serde(...)]`, or serde derive.

## Verification

All checks used a ticket-scoped isolated Cargo target because other agents were compiling and
editing the shared workspace concurrently.

The required first gate initially passed:

```text
cargo check -p semio-framework-plugin
Finished `dev` profile
```

That clean interval allowed both requested downstream checks to run.

`semio-s-plugin-mathematical` reached the plugin and then failed with 2,292 errors from the
concurrent mutation taxonomy/async conversion (`MdBlock`, missing `mutation` modules, changed job
and CAS APIs, and related type mismatches). No error named serde, `Serialize`, or `Deserialize`.

`semio-s-plugin-fem` initially failed with 232 errors. This run exposed the stale shared-engine
serde uses described above. After replacing them with the native value codec, an intermediate run
found one unsupported derive on the `VecD` newtype; explicit `ToValue`/`FromValue` implementations
fixed it. The final focused run failed with 147 errors, all under the concurrent FEM plugin
mutation/DSL conversion. It reported no error in `✏️s/🔨️modules/🏗️fem/⚙️engine` and no serde error.

The stronger macro regression test was attempted with:

```text
cargo test -p semio-framework-plugin app_commands_tests --lib
```

During its cold test-only dependency build, the shared workspace changed again. The attempt stopped
before the plugin test harness in unrelated `semio-framework` interaction/manifest code. At that
instant there were 47 missing serde/value trait errors; a final `cargo check -p
semio-framework-plugin` moments later reported 39 errors in the same active manifest wave, which
confirms the moving concurrent state. This ticket did not overwrite those files.

Therefore the strongest available signals are:

1. one passing framework plugin check during a clean interval;
2. compile-time inspection that every macro arm is serde-free and every regression-fixture payload
   lacks serde traits;
3. both requested downstream packages reached their own sources without the reported
   `app_commands!` serde failure class;
4. the FEM engine replacement type-checked without any engine-local errors on the final run.

Neither complete downstream package check is green because of the concurrent taxonomy/DSL work,
and the dedicated macro test did not execute because its upstream test build became broken.

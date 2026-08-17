# Wave 5 — Norm Glue Integrator Report

Ticket: `26/08/08/ARTIFACT-SCHEMA-FACETS`  
Plugin: `semio-s-plugin-norm` at `✏️s/🔌️plugins/📕️norm/`  
Role: shared glue + finish line for fifteen sibling-refactored artifacts.

## Uniform glue form (§15 lowpoly pilot)

Leaf-prefixed modules with `#[path = "."]` grouping in `📦️packages/🦀️rust/📦️glue.rs`, per artifact:

| Module | Path |
| --- | --- |
| `pub mod schema` | `🧬️schema/` |
| `pub mod snapshot { schema, pack }` | `📸️snapshot/🧬️schema` + `📸️snapshot/🎒️pack` |
| `pub mod diff { component + pub use; schema }` | `🔺️diff/` + `🔺️diff/🧬️schema` |
| Mutations | `🧬️mutations/📄set-snapshot/...` |
| Root `pack` | **removed** (relocated under snapshot) |

Artifact roots only `pub use …Snapshot` + `artifact_kind()` — they do **not** remount `schema`/`snapshot` (avoids glue conflicts).

Shared wiring also includes:

- `semio-framework-schema` in `Cargo.toml` + `extern crate … as schema`
- TypeScript mirror exports in `📦️packages/🟦️typescript/📦️index.ts`
- `🔌️plugin/🔧️setup`: `register_pilot_languages` + `register_artifact_schema` for all 15
- `🖥️app-surface`: `.snapshot` views, `commit_snapshot`, wrap-closure `import_media`
- `📄️document`: `impl_norm_set_snapshot_ops!` for all 15 mutation roots

## Fifteen artifacts

| folder | key | prefix |
| --- | --- | --- |
| `📓️iso16757` … `📔️vdi3805`, `📕️din4108`, `📗️din16798`, `📙️din18599`, `📘️en1990`–`en1999` | matching keys | `Iso16757` … `En1999` |

## Sibling incompletes finished by glue

- DIN/ISO runtime: `XSnapshot` / `SetSnapshot` / `ArtifactEngine`, apps `DocumentApp::Snapshot`, pack under `snapshot::pack`
- EN1990–1994 apps: `setSnapshot` command wiring + keyword guards
- EN1995–1999: codecs `DocumentDsl`/`DocumentPack`, `LanguageRole::Document`, `DocumentStore` in SPR tests
- Shared renames: `projection` → `snapshot` on views/apps; `store::os_store::test_support::*`; envelope ids `norm.demo` / `norm.config`
- Example fixtures: regenerated placeholders (din/iso/vdi/en1997); unit-stripped EN fixtures to match schemas; en1998 rebuilt from default print with asserted overrides
- Schema parity: `En1990Snapshot` first top-level type (QkEntry after); GraphQL `!` on artifact/snapshot required fields; diff GraphQL left optional; `selectedCheckIndex` optional; en1995–1999 diff proto `optional … artifact`

## Gate tails (verbatim)

### 1. `cargo check -p semio-s-plugin-norm`

```
warning: `semio-s-plugin-norm` (lib) generated 189 warnings (run `cargo fix --lib -p semio-s-plugin-norm` to apply 189 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3.83s
```

Exit: **0**

### 2. `cargo test -p semio-s-plugin-norm --lib`

```
test result: ok. 834 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```

Exit: **0**

### 3. `bun ./📜️script.ts policy 2>&1 | rg -i 'norm|en199|din|iso16757|vdi3805'`

```
```

(empty — CLI stdout is DEBUG-only when piped; no filtered breach lines)

Direct confirm (`🧪wave5-norm-glue-policy-probe.ts`):

```
norm artifact-schema breaches: 0
norm total policy breaches: 246
```

(246 = non–artifact-schema residue: emoji/taxonomy/migration noise outside this wave’s facet contract, same class as other wave-5 plugins.)

## Not validated

- Interactive UI / playground runtime
- TypeScript vitest for the norm package
- `cross-fem` feature path (`evaluate_fem_path` is `#[cfg(feature = "cross-fem")]`)
- Exhaustive cleanup of the 246 non-ASB policy hits

## Ticket tooling (kept)

- `🧪wave5-norm-glue-integrate.py` (+ tails)
- `🧪wave5-norm-glue-policy-probe.ts`
- Gate logs: `🧪wave5-norm-glue-final-*.{out,err}`, `🧪wave5-norm-glue-gate-policy-*.txt`
- Fixture dumps: `🧪wave5-norm-glue-fixture-*.dsl.semio`

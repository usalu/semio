# Research & Discovery: DWG AC1018 / AC1024 Schema-ID Collision in Stdio Plugin

## 1. Problem Statement
The policy scanner rule `policyStdioCodecIdUniquenessBreaches` (added 2026-08-11 in ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) reports 2 policy breaches in `semio-s-plugin-stdio`:
- Both DWG standards (`ac1018` and `ac1024`) register their document codecs in `store::register_document_codec` using the identical schema id string `"stdio.dwg"`:
  - `ac1018`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/⚙️engine/🦀️component.rs:31`
  - `ac1024`: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/⚙️engine/🦀️component.rs:491`

`store::register_document_codec` uses a HashMap keyed on the schema id string. Because it silently overwrites duplicate schema IDs (last-write wins), whichever standard registers second overwrites the first one's codec registration.

## 2. Repo Audit of `"stdio.dwg"` References
A full search across `.rs` files was performed.
- Other plugins (`procedural2d`, `procedural3d`, `gisterrain`, `gismap`, `shooting`, `process3d`, `lowpoly`, etc.) use `artifact_kind: "s.stdio.dwg"` and `standard: StandardId("ac1018")` or `StandardId("ac1024")`. None of them reference `stdio.dwg` as a codec schema id directly.
- In `ac1018`'s own engine and snapshot files, `register_schema_specs` already registers under `"stdio.dwg.ac1018"` and `"stdio.dwg.ac1018#diff"`. However, `register_document_codec`, `register_language`, `passthrough_hooks`, `envelope_id()`, and test/mutation fixtures were still using `"stdio.dwg"`.

## 3. Plan & Decision
Per Decision #5 (from ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION):
- `ac1024` is the actively maintained, canonical standard for DWG, so it keeps the primary schema id `"stdio.dwg"`.
- `ac1018` is a frozen legacy shim and should be assigned the distinct schema id `"stdio.dwg.ac1018"`.

All coupled places in `ac1018` will be updated simultaneously:
1. `ac1018/⚙️engine/🦀️component.rs`:
   - `STDIO_DWG_AC1018_DOCUMENT_SCHEMA`: `pub const STDIO_DWG_AC1018_DOCUMENT_SCHEMA: &str = "stdio.dwg.ac1018";`
   - `store::register_document_codec(...)` -> use `STDIO_DWG_AC1018_DOCUMENT_SCHEMA`
   - `dsl::register_language(...)` -> `id: "stdio.dwg.ac1018"`, `hooks: dsl::passthrough_hooks("stdio.dwg.ac1018")`
   - Update tests checking `snapshot.schema`
2. `ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
   - `DwgSnapshot::default()` schema -> `STDIO_DWG_AC1018_DOCUMENT_SCHEMA` / `"stdio.dwg.ac1018"`
   - `decode_dwg()` schema -> `STDIO_DWG_AC1018_DOCUMENT_SCHEMA` / `"stdio.dwg.ac1018"`
   - `ArtifactDsl::envelope_id()` -> `"stdio.dwg.ac1018"`
3. `ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`:
   - `artifact-mark = "stdio.dwg.ac1018"`
4. `ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`:
   - `demo_mutation_cases()`, `base_snapshot()`, `sweep_a()`, `sweep_b()` schema literals -> `"stdio.dwg.ac1018"`

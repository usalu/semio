# Sourcing `kind_mesh_json` MeshData Reverification (2026-09-02)

## Finding

The shared checkout already contains the production conversion, so no source edit was needed:

- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:237` returns `dsl::DslValue`, not `serde_json::Value`.
- Its `data` member is produced by `dsl::ToValue::to_value(&mesh)`; no `serde_json::json!` or `MeshData: Serialize` requirement remains on this path.
- Its preview and grid consumers serialize the accumulated `DslValue` tree with `dsl::json::to_json_string`.

`MeshData` itself was inspected only, not edited. Its first-party `ToValue` conversion delegates through the pack JSON representation. That representation preserves the camelCase fields, and its `u32` conversion produces `pack::json::Number::UInt`, so mesh indices remain unsigned integers rather than floats.

## Verification

Ran the required foreground check with the supplied warm isolated target directory and no Rust wrapper:

```sh
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/isolated-target2
export RUSTC_WRAPPER=""
cargo check -p semio-s-plugin-sourcing --message-format short
```

The command exited `101`. `grep -cE ': error(\[|:)'` reported **3** matched Rust error lines, all unrelated `CuratedItem` serde bounds in `create-curated-item/🦀️.rs` (one `Serialize` and two `Deserialize` bounds). The captured output had **no** `MeshData`/serde error lines.

No files in `mesh-engine` were modified. The temporary compiler log was kept under this ticket's `🗑️generated` folder only while counting and was removed afterward.

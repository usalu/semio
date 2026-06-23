"""Strip legacy surfaces from compose/client/lib/rs/lib.rs and golden ops fixture."""
import re
from pathlib import Path

lib = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
lines = lib.read_text(encoding="utf-8").splitlines(keepends=True)


def drop_names_arm(macro_name: str, apply_marker: str) -> None:
    global lines
    macro_i = next(i for i, ln in enumerate(lines) if f"macro_rules! {macro_name}" in ln)
    start = next(i for i in range(macro_i, len(lines)) if lines[i].strip() == "(@names) => {")
    apply_i = next(i for i in range(start + 1, len(lines)) if apply_marker in lines[i])
    del lines[start:apply_i]


drop_names_arm("gap_surface_family_name_list", "        (@apply_families) => {")
drop_names_arm("gap_surface_existing_relay_name_list", "        (@apply_relays) => {")

text = "".join(lines)
text = text.replace(
    "    async fn legacy_created_fixed_piece_to_kit_op(input: &serde_json::Value)",
    "    async fn stored_create_fixed_piece_operation(input: &serde_json::Value)",
)
text = text.replace(
    "return legacy_created_fixed_piece_to_kit_op(input).await;",
    "return stored_create_fixed_piece_operation(input).await;",
)
old_stored = """    pub(crate) async fn kit_operation_from_stored(kind: &str, input: &serde_json::Value) -> Result<crate::operation::Operation, ComposeError> {
        if kind == "createdFixedPiece" {
            return legacy_created_fixed_piece_to_kit_op(input).await;
        }
        kit_operation_from_step_json(input)
    }"""
new_stored = """    pub(crate) async fn kit_operation_from_stored(kind: &str, input: &serde_json::Value) -> Result<crate::operation::Operation, ComposeError> {
        match kind {
            "createFixedPiece" => stored_create_fixed_piece_operation(input).await,
            _ => kit_operation_from_step_json(input),
        }
    }"""
if old_stored not in text:
    raise SystemExit("kit_operation_from_stored block not found")
text = text.replace(old_stored, new_stored)
text = text.replace(
    "    /// @emoji 📑 US-001 golden JSON: top-level `operations` array, or legacy key `ops` (see `kit-store.golden.ops.compose.json`).\n    pub fn golden_operation_records_ref",
    "    /// @emoji 📑 US-001 golden JSON: top-level `operations` array.\n    pub fn golden_operation_records_ref",
)
text = text.replace(
    'src.get("operations").and_then(|v| v.as_array()).or_else(|| src.get("ops").and_then(|v| v.as_array())).ok_or_else(|| ComposeError::invalid("golden operations missing `operations` or `ops` array"))',
    'src.get("operations").and_then(|v| v.as_array()).ok_or_else(|| ComposeError::invalid("golden operations missing `operations` array"))',
)
text = re.sub(
    r"\n        pub async fn stub_ok\(\) -> Self \{[^}]+\}\n",
    "\n",
    text,
    count=1,
    flags=re.DOTALL,
)
text = text.replace(
    "crate::operation::CommandResponse::stub_ok().await",
    'crate::operation::CommandResponse::fail_msg("not implemented").await',
)
text = text.replace(
    "Ok(crate::operation::CommandResponse::stub_ok().await.into())",
    'Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())',
)
text = text.replace('if kind != "createdFixedPiece"', 'if kind != "createFixedPiece"')
text = text.replace('match kind {\n                    "createdFixedPiece" =>', 'match kind {\n                    "createFixedPiece" =>')
text = text.replace('.expect("operations|ops")', '.expect("operations")')

lib.write_text(text, encoding="utf-8", newline="\n")
print("lib.rs updated")

golden = Path(r"c:\git\compose\compose\assets\compose\kit-store.golden.ops.compose.json")
g = golden.read_text(encoding="utf-8")
g = g.replace('"ops":', '"operations":')
g = g.replace('"kind": "createdFixedPiece"', '"kind": "createFixedPiece"')
golden.write_text(g, encoding="utf-8", newline="\n")
print("golden ops updated")

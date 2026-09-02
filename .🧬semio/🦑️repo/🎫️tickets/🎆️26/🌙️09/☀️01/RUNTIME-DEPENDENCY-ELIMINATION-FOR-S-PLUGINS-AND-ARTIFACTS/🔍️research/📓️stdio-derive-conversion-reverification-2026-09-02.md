# 🗄️ stdio — derive/`#[serde(...)]` conversion: re-verification (2026-09-02)

Dispatched with the same scope as `📓️stdio-derive-conversion-wave.md` (this ticket): convert
PRODUCTION `#[derive(Serialize, Deserialize)]` / `#[serde(...)]` / `use serde::{...}` sites in
`✏️s/🔌️plugins/🗄️stdio` (crate `semio-s-plugin-stdio`) to `#[derive(ToValue, FromValue)]` /
`#[value(...)]`, excluding `🧪️oracle/`, `🧪️tests/`, `tests/`, `🔬️probes/`, `🏭️generator/`,
`🧫️fixtures/`, and `#[cfg(test)]` code. **No `cargo` command was run this session** (explicit
instruction — the workspace has 40-60 concurrent rustc processes; a single central verification
pass is run by the coordinator). Everything below is static/textual verification only.

## Headline: the mechanical conversion is already complete

Independently re-derived the file list from scratch (not trusting the prior doc), using a
Python regex that correctly handles multi-line `#[derive(...)]` bodies and distinguishes
`#[cfg_attr(test, derive(Serialize, Deserialize))]` (sanctioned) from an unconditional derive
(the naive single-line `grep` a prior pass used cannot make this distinction and over-reports).
Walked every `*.rs` under `✏️s/🔌️plugins/🗄️stdio`, pruning `🧪️oracle`, `🧪️tests`, `tests`,
`🔬️probes`, `🏭️generator`, `🧫️fixtures`, `target` subtrees at the directory-walk level.

Result: **6 files** carry an unconditional `Serialize`/`Deserialize` derive or `#[serde(...)]`
attribute in production scope. All 6 already have an in-file doc comment recording a specific,
verifiable technical reason, matching `📓️stdio-derive-conversion-wave.md`'s disposition table
exactly. Re-verified each rationale by reading the surrounding code, not just trusting the
comment:

| file | why it must stay dual-derived / serde-only |
|---|---|
| `🧊️gltf/…/🚪️io/💡️inferences/📝️text/🦀️.rs` (`GltfInferenceLeafEnvelope`) | field `value: serde_json::Value` — genuinely built from `encode_result() -> Result<serde_json::Value, _>` and read back via `serde_json::Value::{as_array,as_str,to_string}` in the crate root's `infer_gltf_leaf_cold`. `serde_json::Value` has no `ToValue` impl. Confirmed: no `ToValue`/`FromValue` on this one struct. |
| `🧊️gltf/…/🧬️schema/📸️snapshot/🦀️.rs` (`GltfDocument` + ~32 nested types) | the literal wire model for real `.gltf`/`.glb` bytes, round-tripped through `serde_json::to_vec`/`from_str`/`to_vec_pretty` in the sibling `🚪️io/🦀️.rs` (`parse_gltf_document`, `serialize_gltf_document`, `encode_glb`, `decode_glb`). Confirmed every listed struct/enum carries BOTH `Serialize, Deserialize` AND `value_derive::ToValue, value_derive::FromValue` side by side, with paired `#[serde(...)]`/`#[value(...)]` field attributes — additive, not leftover. The 4 hand-rolled inner `Wire`/`Wire<'a>` helper structs (`GltfCameraProjection`/`GltfCamera`'s serialize/deserialize impls) are serde-only by design (internal wire shims, never exposed). |
| `🧿️semio/…/✳️brep/…/✅validation-report/🦀️.rs` | `store::InferredField::Value` (framework trait) bounds on `Serialize + DeserializeOwned` for its byte-cache codec — a genuine, permanent framework requirement, not leftover. |
| `🧿️semio/…/✳️graph/…/🔗connectivity/🦀️.rs` | same `InferredField::Value` rationale. |
| `🧿️semio/…/✳️table/…/🎲entropy/🦀️.rs` | same `InferredField::Value` rationale. |
| `🧿️semio/…/✳️table/…/📊moments/🦀️.rs` | same `InferredField::Value` rationale. |

No `flatten`/`with`/`skip` sites need to be newly flagged: the one `#[serde(with =
"ordered_attr_map")]` in the gltf snapshot file (`GltfMorphTarget`) sits inside the
already-documented gltf exception, already paired with a hand-written `dsl::ToValue`/
`dsl::FromValue` impl below it (not a derive) — nothing to convert there.

## Everything else in production scope

- Every other production type across the plugin (glTF's `🔺️diff`, `🧬️mutations/…/change-node-name`,
  `🎒️zip/📦️opc`'s 5 OPC types, semio-kit/semio-object schema+snapshot, semio-mesh `📦aabb`,
  semio-drawing `🎛flattened-scene`, semio-any `🧮️geometry`) is either `ToValue`/`FromValue`-only,
  or uses the sanctioned `#[cfg_attr(test, derive(Serialize, Deserialize))]` +
  `#[cfg_attr(test, serde(...))]` pattern (serde only for a same-file `#[cfg(test)]` differential
  oracle against `serde_json`), with `use serde::{...}` correctly moved behind `#[cfg(test)]`.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`'s `impl_serde_op_codec!` macro is a **naming
  leftover only** — its body uses exclusively `pack::json_to_string`/`pack::json_from_dsl_value`/
  `dsl::ToValue`/`dsl::FromValue`, zero real `serde` crate usage. Not renamed (out of this task's
  scope — a rename would need every call-site macro invocation double-checked, and isn't a
  derive/attribute conversion).
- Several `#[serde(` / `Serialize`/`Deserialize` grep hits in `🔺️diff/🦀️.rs`,
  `🧬️mutations/…/change-node-name/🦀️.rs`, semio-kit/`object` schema files, `📦aabb`, and
  `🎛flattened-scene` are false positives: backtick-quoted code examples inside doc comments
  (e.g. `` `#[serde(default)]` `` used prose-side to explain why the framework's own
  `#[value(...)]` diverges), not real attributes. Verified by reading each site directly.

## Separately noted, not fixed (per instruction — later wave)

- **`serde_json::` production call-site usage** (function calls, not derives) is far larger and
  explicitly out of this wave's scope. Confirmed still present, dominated by:
  - the glTF inference-leaf `encode_result()` family (~55 near-identical leaf functions) and the
    RFC 8785 canonical-JSON writer (`write_canonical_json`/`canonical_number`) in
    `🚪️io/💡️inferences/📝️text/🦀️.rs` and the real `.gltf`/`.glb` codec in `🚪️io/🦀️.rs` — both
    already flagged with an explicit follow-up in `📓️stdio-derive-conversion-wave.md`.
  - `🪟️windows/🪟️main` viewer/editor files building ad hoc JSON via `serde_json::json!`/
    `to_string` (not derived serialization).
- Neither category involves a `#[derive(...)]` or `#[serde(...)]` attribute, so neither is this
  task's "convert derive/attribute" scope even though it is real `serde_json` dependency surface.

## What was NOT done this session

- No `cargo check`/`build`/`test` — forbidden by instruction.
- No edits — nothing to change; the prior session's conversion (see
  `📓️stdio-derive-conversion-wave.md`, files listed there: `🧮️geometry/🦀️.rs`,
  `✳️kit/🧬️schema/📸️snapshot/🦀️.rs`, `🎒️zip/📦️opc/🦀️.rs`) is intact on disk and re-verified
  correct by independent multi-line-aware static analysis, not merely re-trusted.
- No `Cargo.toml` touched (per standing instruction: don't drop `serde`/`serde_json` until real
  call-site usage — noted above — also reaches zero, which is a separate, larger wave).

## Method (for anyone re-running this)

```
python3 - <<'EOF'
import re, os
exclude_re = re.compile(r'(^|/)(🧪️oracle|🧪️tests|tests|🔬️probes|🏭️generator|🧫️fixtures|target)(/|$)')
root = "✏️s/🔌️plugins/🗄️stdio"
for dirpath, dirnames, filenames in os.walk(root):
    if exclude_re.search(dirpath + "/"):
        dirnames[:] = []
        continue
    for fn in filenames:
        if not fn.endswith(".rs"):
            continue
        p = os.path.join(dirpath, fn)
        text = open(p, encoding='utf-8').read()
        for m in re.finditer(r'#\[derive\(([^)]*)\)\]', text, re.DOTALL):
            body = m.group(1)
            if 'Serialize' in body or 'Deserialize' in body:
                start = m.start()
                is_cfg_test = 'cfg_attr(test' in text[max(0,start-60):start]
                if not is_cfg_test:
                    print(p, text[:start].count('\n')+1, body.strip()[:80])
EOF
```

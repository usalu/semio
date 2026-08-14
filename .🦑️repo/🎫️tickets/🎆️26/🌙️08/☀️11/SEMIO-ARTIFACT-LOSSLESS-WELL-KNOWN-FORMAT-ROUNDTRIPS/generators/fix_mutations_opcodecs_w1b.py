#!/usr/bin/env python3
# 📎 W1b closer: adds NoMutation + hand-rolled OpText/OpBinary (JSON round-trip, see brep's own
# comment for full rationale) to the remaining 20 scaffolded mutation files, matching the pattern
# already verified compiling clean on semio/brep. Purely mechanical — same transform, 21 files.
import re

FILES = [
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️workflow/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs",
]

REPO = "/Users/ueli/Documents/semio/"

ENUM_RE = re.compile(
    r"#\[derive\(Clone, Debug, PartialEq, Serialize, Deserialize\)\]\n"
    r'#\[serde\(tag = "mutation", rename_all = "camelCase"\)\]\n'
    r"pub enum (\w+) \{\n"
    r"(    /// .*\n)?"
    r"    SetSnapshot \{ snapshot: (\w+) \},\n"
    r"\}\n"
)

DIFF_FN_RE = re.compile(
    r"    fn diff\(&self, base: &(\w+)\) -> Self::Diff \{\n"
    r"        let _ = base;\n"
    r"        match self \{\n"
    r"            (\w+)::SetSnapshot \{ snapshot \} => (\w+) \{ replacement: Some\(snapshot\.clone\(\)\) \},\n"
    r"        \}\n"
    r"    \}\n"
)

INVERSE_FN_RE = re.compile(
    r"    fn inverse\(&self, base: &(\w+)\) -> Vec<Self> \{\n"
    r"        vec!\[(\w+)::SetSnapshot \{ snapshot: base\.clone\(\) \}\]\n"
    r"    \}\n"
)

MUT_REGION_END_RE = re.compile(r"\n//#endregion 🔖️Mutation\n")

for rel in FILES:
    path = REPO + rel
    with open(path, encoding="utf-8") as f:
        content = f.read()
    orig = content

    m = ENUM_RE.search(content)
    if not m:
        print("ENUM NO MATCH:", rel)
        continue
    mutation_ty, doc_line, snapshot_ty = m.group(1), m.group(2) or "", m.group(3)
    doc_line = doc_line or "    /// 🚧 Full-snapshot replace — the only real variant until W2's per-field vocabulary lands.\n"
    new_enum = (
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]\n"
        '#[serde(tag = "mutation", rename_all = "camelCase")]\n'
        f"pub enum {mutation_ty} {{\n"
        "    #[default]\n"
        "    NoMutation,\n"
        f"{doc_line}"
        f"    SetSnapshot {{ snapshot: {snapshot_ty} }},\n"
        "}\n"
    )
    content = ENUM_RE.sub(lambda _m: new_enum, content, count=1)

    dm = DIFF_FN_RE.search(content)
    if not dm:
        print("DIFF NO MATCH:", rel)
        continue
    snap_ty2, mut_ty2, diff_ty = dm.group(1), dm.group(2), dm.group(3)
    new_diff = (
        f"    fn diff(&self, _base: &{snap_ty2}) -> Self::Diff {{\n"
        "        match self {\n"
        f"            {mut_ty2}::NoMutation => {diff_ty}::default(),\n"
        f"            {mut_ty2}::SetSnapshot {{ snapshot }} => {diff_ty} {{ replacement: Some(snapshot.clone()) }},\n"
        "        }\n"
        "    }\n"
    )
    content = DIFF_FN_RE.sub(lambda _m: new_diff, content, count=1)

    im = INVERSE_FN_RE.search(content)
    if not im:
        print("INVERSE NO MATCH:", rel)
        continue
    snap_ty3, mut_ty3 = im.group(1), im.group(2)
    new_inverse = (
        f"    fn inverse(&self, base: &{snap_ty3}) -> Vec<Self> {{\n"
        "        match self {\n"
        f"            {mut_ty3}::NoMutation => vec![{mut_ty3}::NoMutation],\n"
        f"            {mut_ty3}::SetSnapshot {{ .. }} => vec![{mut_ty3}::SetSnapshot {{ snapshot: base.clone() }}],\n"
        "        }\n"
        "    }\n"
    )
    content = INVERSE_FN_RE.sub(lambda _m: new_inverse, content, count=1)

    opcodecs = f'''
//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` — 🚧 scaffolded by W1b: plain `serde_json` round-trip of
/// the whole enum (one line of compact JSON per op), the same "JSON-pack passthrough" honesty
/// boundary the subset's own `ArtifactPack` impl already uses (see that file's doc comment).
/// Deliberately NOT `#[derive(dsl::DslOps)]` + `#[dsl(block)]` (the grammar/hand-rolled-op-triple
/// path every OTHER artifact's real mutation vocabulary uses) — that path requires the embedded
/// snapshot type to itself implement `dsl::DslField` (via `dsl::DslRecord`), which is real work
/// spanning every nested type in the snapshot tree and squarely W2's job, not a wiring fix. W2
/// replaces this whole region when it replaces `SetSnapshot` with the real per-field vocabulary.
impl protocol::OpText for {mutation_ty} {{
    fn parse_op(line: &str) -> Result<Self, store::TextError> {{
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }}
    fn print_op(&self) -> String {{
        serde_json::to_string(self).unwrap_or_default()
    }}
}}

impl protocol::OpBinary for {mutation_ty} {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }}
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }}
}}
//#endregion OpCodecs
'''
    content, n = MUT_REGION_END_RE.subn("\n//#endregion 🔖️Mutation\n" + opcodecs, content, count=1)
    if n != 1:
        print("MUTATION-END NO MATCH:", rel)
        continue

    # Append an op_text_binary_roundtrip_law test right before the final "}\n//#endregion 🔖️Tests"
    test_re = re.compile(r"(\n)\}\n//#endregion 🔖️Tests\n\Z")
    tm = test_re.search(content)
    if not tm:
        print("TESTS-END NO MATCH:", rel)
        continue
    snap_default = snapshot_ty
    new_test = f'''
    /// 🧪️ op_text_binary_roundtrip_law: handcrafted `OpText`/`OpBinary` JSON round-trip.
    #[test]
    fn op_text_binary_roundtrip_law() {{
        let base = {snap_default}::default();
        for m in [{mutation_ty}::NoMutation, {mutation_ty}::SetSnapshot {{ snapshot: base.clone() }}] {{
            let printed = m.print_op();
            assert!(!printed.contains('\\n'), "print_op must be one line, got {{printed:?}}");
            let parsed = {mutation_ty}::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({{printed:?}}) failed: {{e}}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {{m:?}}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({{m:?}}) failed: {{e}}"));
            let decoded = {mutation_ty}::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {{e}}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {{m:?}}");
        }}
    }}
'''
    content = content[: tm.start(1)] + new_test + "}\n//#endregion 🔖️Tests\n"

    if content == orig:
        print("NO CHANGE:", rel)
        continue
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print("fixed:", rel)

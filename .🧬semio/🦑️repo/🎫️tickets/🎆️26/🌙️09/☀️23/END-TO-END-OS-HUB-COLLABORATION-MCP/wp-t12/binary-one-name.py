"""✂️ One name for stdio binary's byte-range replacement: the leaf, wire tag, DSL keyword, grammars, catalog, manifest,
fixture directory, feature, adapter and oracle all speak `replace-byte-range` (kebab) / `replaceByteRange` (wire camelCase) /
`ReplaceByteRange` (Rust/schema). The retired `splice` spelling stays only where it names the diff's byte operation
(`ByteSplice`, `BinaryDiff.splices`) or `Vec::splice`. `--dry` reports without writing."""
import os, sys
root = "/Users/ueli/Documents/semio/"
S = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/"
dry = "--dry" in sys.argv
changed, problems = [], []

def edit(rel, pairs):
    path = root + S + rel
    text = open(path, encoding="utf-8").read()
    new = text
    for old, rep, count in pairs:
        n = new.count(old)
        if n != count:
            problems.append(f"{rel}: expected {count} of {old[:70]!r}, found {n}")
            return
        new = new.replace(old, rep)
    if new != text:
        changed.append(rel)
        if not dry: open(path, "w", encoding="utf-8").write(new)

M = "🧬️schema/🧬️mutations/"
edit(M + "🦀️.rs", [
    ("""    /// ✂️ Replaces `[offset, offset+remove_len)` with `insert`. The variant is named for its
    /// approved verb (`replace`) because `#[derive(dsl::Mutations)]` asserts the leaf descriptor's
    /// `semanticKind` equals `to_kebab(VariantIdent)` and rejects a single-word kind; the wire tag
    /// stays `splice`, which is what the catalog, the feature file and the committed fixtures speak.
    #[value(rename = "splice")]
    ReplaceByteRange""", """    /// ✂️ Replaces `[offset, offset+remove_len)` with `insert`.
    ReplaceByteRange""", 1),
    ('pub const KINDS: &[&str] = &["set-snapshot", "splice", "append-bytes", "truncate-at"];', 'pub const KINDS: &[&str] = &["set-snapshot", "replace-byte-range", "append-bytes", "truncate-at"];', 1),
])
edit(M + "✂️replace-byte-range/🦀️.rs", [
    ("""//! delegates straight through to this leaf's own record, keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed. The variant was renamed
//! `ReplaceByteRange` (`#[value(rename = "splice")]` on the aggregate variant), but the DSL
//! keyword stays `splice` — that is what the committed grammar/protocol facets and the catalog
//! still speak.
""", """//! delegates straight through to this leaf's own record. Leaf, wire tag, DSL keyword, grammars and
//! catalog all speak one name: `replace-byte-range`.
""", 1),
    ('#[dsl(keyword = "splice")]', '#[dsl(keyword = "replace-byte-range")]', 1),
    ('protocol::LocalizedLabel::native("splice", "Spleiß")', 'protocol::LocalizedLabel::native("replace-byte-range", "Bytebereich ersetzen")', 1),
])
edit(M + "📝️text/📖️.grammar.semio", [
    ('#   splice offset=1 remove-len=2 insert="qrvM"', '#   replace-byte-range offset=1 remove-len=2 insert="qrvM"', 1),
    ("`set-snapshot`, `AppendBytes`->`append-bytes`", "`set-snapshot`, `ReplaceByteRange`->`replace-byte-range`, `AppendBytes`->`append-bytes`", 1),
    ("document = no-mutation | set-snapshot | splice | append-bytes | truncate-at", "document = no-mutation | set-snapshot | replace-byte-range | append-bytes | truncate-at", 1),
    ('splice = "splice" "offset"', 'replace-byte-range = "replace-byte-range" "offset"', 1),
])
edit(M + "📝️text/🔤️.ebnf", [("""'"splice"'""", """'"replaceByteRange"'""", 1)])
edit(M + "📝️text/🅰️.g4", [("""'"splice"'""", """'"replaceByteRange"'""", 1)])
edit(M + "🟦️.ts", [("mutation: 'splice';", "mutation: 'replaceByteRange';", 1)])
edit(M + "🛰️.proto", [("  SPLICE = 2;", "  REPLACE_BYTE_RANGE = 2;", 1)])
edit(M + "🧪️tests/🔬️unit/🦀️.rs", [
    ("only the deliberate short-snapshot/out-of-range splice pair", "only the deliberate short-snapshot/out-of-range replace-byte-range pair", 1),
    ('assert!(BinaryMutation::parse_op("replace-byte-range offset=1 remove-len=2 insert=\\"qrvM\\"").is_err());', 'assert!(BinaryMutation::parse_op("splice offset=1 remove-len=2 insert=\\"qrvM\\"").is_err());', 1),
    ('assert!(BinaryMutation::parse_op("splice-extra offset=1 remove-len=2 insert=\\"qrvM\\"").is_err());', 'assert!(BinaryMutation::parse_op("replace-byte-range-extra offset=1 remove-len=2 insert=\\"qrvM\\"").is_err());', 1),
    ('BinaryMutation::ReplaceByteRange(_) => "splice",', 'BinaryMutation::ReplaceByteRange(_) => "replace-byte-range",', 1),
])

O = "🔮️oracles/"
edit(O + "🔣️.json", [
    ("what an out-of-range splice returns", "what an out-of-range byte-range replacement returns", 1),
    ("(offset/remove_len splice, append, truncate, snapshot)", "(offset/remove_len byte-range replacement, append, truncate, snapshot)", 1),
    ('''        "set-snapshot",
        "splice",
        "append-bytes",''', '''        "set-snapshot",
        "replace-byte-range",
        "append-bytes",''', 1),
    ('''          "id": "splice",
          "capability": "binary-raw-mutate",
          "payloadSchema": "🧬️schema/🔣️.json",
          "outcomes": [
            "applied"
          ],
          "productionDispatch": {
            "operation": "splice",''', '''          "id": "replace-byte-range",
          "capability": "binary-raw-mutate",
          "payloadSchema": "🧬️schema/🔣️.json",
          "outcomes": [
            "applied",
            "rejected"
          ],
          "productionDispatch": {
            "operation": "replace-byte-range",''', 1),
    ('"id": "splice-applied",', '"id": "replace-byte-range-applied",', 1),
    ('"mutation": "splice",', '"mutation": "replace-byte-range",', 1),
    ("../🧫️fixtures/✂️splice/", "../🧫️fixtures/✂️replace-byte-range/", 2),
    ("ReplaceByteRange{offset:2,removeLen:2,insert:[99,99]} — wire tag `splice`, per this leaf's own `#[value(rename=\\\"splice\\\")]` — replaces", "ReplaceByteRange{offset:2,removeLen:2,insert:[99,99]} replaces", 1),
])
edit(O + "🦀️.rs", [
    ("//! written splice/append/truncate implementation", "//! written byte-range replacement/append/truncate implementation", 1),
    ("//! `set-snapshot`, `splice`, `append-bytes`, `truncate-at`.", "//! `set-snapshot`, `replace-byte-range`, `append-bytes`, `truncate-at`.", 1),
    ("//#region 🔖️Splice\n/// ✂️ The specification's own contract for one splice,", "//#region 🔖️ReplaceByteRange\n/// ✂️ The specification's own contract for one byte-range replacement,", 1),
    ("fn splice(buffer: &mut Vec<u8>, offset: usize, remove_len: usize, insert: &[u8])", "fn replace_range(buffer: &mut Vec<u8>, offset: usize, remove_len: usize, insert: &[u8])", 1),
    ('format!("splice offset {offset}', 'format!("replace-byte-range offset {offset}', 1),
    ('format!("splice remove_len {remove_len}', 'format!("replace-byte-range remove_len {remove_len}', 1),
    ("//#endregion 🔖️Splice", "//#endregion 🔖️ReplaceByteRange", 1),
    ('        "splice" => {', '        "replace-byte-range" => {', 1),
    ("            splice(&mut out, offset, remove_len, &insert)?;", "            replace_range(&mut out, offset, remove_len, &insert)?;", 1),
    ("            splice(&mut out, len, 0, &data)?;", "            replace_range(&mut out, len, 0, &data)?;", 1),
])
edit(O + "🧪️tests/🔬️unit/🦀️.rs", [
    ("fn splice_replaces_the_named_range()", "fn replace_byte_range_replaces_the_named_range()", 1),
    ("fn splice_out_of_range_offset_is_rejected_without_corrupting()", "fn replace_byte_range_out_of_range_offset_is_rejected_without_corrupting()", 1),
    ("fn splice_remove_len_past_the_end_is_rejected()", "fn replace_byte_range_remove_len_past_the_end_is_rejected()", 1),
    ('spec("splice", params)', 'spec("replace-byte-range", params)', 3),
])

T = "🧪️tests/🔀️mutate-binary-raw/"
edit(T + "🥒️.feature", [
    ("matter here precisely because this subset does NOT parse structure: a splice at offset 6 below", "matter here precisely because this subset does NOT parse structure: a byte-range replacement at offset 6 below", 1),
    ("The specification-vector scenarios below state plainly which byte-splice edge cases this subset\n  defines as VALID (a zero-length splice, an offset of exactly 0, an offset of exactly the buffer's\n  length, a splice spanning the whole buffer,",
     "The specification-vector scenarios below state plainly which byte-range replacement edge cases this\n  subset defines as VALID (a zero-length replacement, an offset of exactly 0, an offset of exactly the\n  buffer's length, a replacement spanning the whole buffer,", 1),
    ("      | splice       | {\"offset\":6,\"removeLen\":5,\"insert\":[65,66,67]}                   |", "      | replace-byte-range | {\"offset\":6,\"removeLen\":5,\"insert\":[65,66,67]}             |", 2),
    ("      | zero-length-splice        | splice      | {\"offset\":100000,\"removeLen\":0,\"insert\":[]}         |", "      | zero-length-replacement        | replace-byte-range | {\"offset\":100000,\"removeLen\":0,\"insert\":[]}         |", 1),
    ("      | splice-at-offset-zero     | splice      | {\"offset\":0,\"removeLen\":2,\"insert\":[255,217]}       |", "      | replacement-at-offset-zero     | replace-byte-range | {\"offset\":0,\"removeLen\":2,\"insert\":[255,217]}       |", 1),
    ("      | splice-at-exact-end       | splice      | {\"offset\":483496,\"removeLen\":0,\"insert\":[90,90,90]} |", "      | replacement-at-exact-end       | replace-byte-range | {\"offset\":483496,\"removeLen\":0,\"insert\":[90,90,90]} |", 1),
    ("      | splice-spans-whole-buffer | splice      | {\"offset\":0,\"removeLen\":483496,\"insert\":[88,89]}    |", "      | replacement-spans-whole-buffer | replace-byte-range | {\"offset\":0,\"removeLen\":483496,\"insert\":[88,89]}    |", 1),
    ("      | truncate-to-zero          | truncate-at | {\"offset\":0}                                        |", "      | truncate-to-zero               | truncate-at        | {\"offset\":0}                                        |", 1),
    ("      | truncate-beyond-length    | truncate-at | {\"offset\":999999999}                                |", "      | truncate-beyond-length         | truncate-at        | {\"offset\":999999999}                                |", 1),
    ("      | id                        | kind        | params                                              |", "      | id                             | kind               | params                                              |", 1),
    ("      | id           | params                                                            |", "      | id                 | params                                                      |", 2),
    ("      | set-snapshot | {\"snapshot\":{\"bytes\":[82,69,80,76,65,67,69,68]}}                 |", "      | set-snapshot       | {\"snapshot\":{\"bytes\":[82,69,80,76,65,67,69,68]}}           |", 2),
    ("      | append-bytes | {\"data\":[84,82,65,73,76,69,82]}                                  |", "      | append-bytes       | {\"data\":[84,82,65,73,76,69,82]}                            |", 2),
    ("      | truncate-at  | {\"offset\":200000}                                                |", "      | truncate-at        | {\"offset\":200000}                                          |", 2),
    ("  @id-invalid-splice\n", "  @id-invalid-replace-byte-range\n", 1),
    ("  Scenario Outline: An invalid splice fails cleanly — <id>", "  Scenario Outline: An invalid byte-range replacement fails cleanly — <id>", 1),
    ("    When the splice mutation is attempted with its parameters", "    When the replace-byte-range mutation is attempted with its parameters", 1),
    ('      {"kind": "splice", "params": <params>}', '      {"kind": "replace-byte-range", "params": <params>}', 1),
])
edit(T + "🦀️.rs", [
    ('        "splice" => {\n            let offset = usize_field(params, "offset").unwrap_or(0).min(input.len());', '        "replace-byte-range" => {\n            let offset = usize_field(params, "offset").unwrap_or(0).min(input.len());', 1),
    ('json_spec("splice", json_obj(', 'json_spec("replace-byte-range", json_obj(', 2),
    ("/// 🔮️ An invalid splice must be REJECTED,", "/// 🔮️ An invalid byte-range replacement must be REJECTED,", 1),
    ("fn invalid_splice_oracle(ctx: &Context)", "fn invalid_replacement_oracle(ctx: &Context)", 1),
    ('expected the invalid splice to be rejected', 'expected the invalid byte-range replacement to be rejected', 2),
    ("schema::mutations::{append_bytes, apply_binary_mutation, set_snapshot, splice, truncate_at, BinaryMutation};", "schema::mutations::{append_bytes, apply_binary_mutation, replace_byte_range, set_snapshot, truncate_at, BinaryMutation};", 1),
    ('            "splice" => BinaryMutation::ReplaceByteRange(', '            "replace-byte-range" => BinaryMutation::ReplaceByteRange(', 1),
    ("    pub fn invalid_splice(ctx: &Context)", "    pub fn invalid_replacement(ctx: &Context)", 1),
    ("""    for id in ["zero-length-splice", "splice-at-offset-zero", "splice-at-exact-end", "splice-spans-whole-buffer", "truncate-to-zero", "truncate-beyond-length"] {
        built = built.oracle(&format!("vector-{id}"), vector_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("vector-{id}"), subject::vector);
        }
    }
""", """    built = built.oracle("vector", vector_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("vector", subject::vector);
    }
""", 1),
    ("""    for id in ["offset-beyond-buffer", "remove-len-exceeds-buffer"] {
        built = built.oracle(&format!("invalid-splice-{id}"), invalid_splice_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("invalid-splice-{id}"), subject::invalid_splice);
        }
    }
""", """    built = built.oracle("invalid-replace-byte-range", invalid_replacement_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("invalid-replace-byte-range", subject::invalid_replacement);
    }
""", 1),
])

src, dst = root + S + "🧫️fixtures/✂️splice", root + S + "🧫️fixtures/✂️replace-byte-range"
if os.path.isdir(src) and not os.path.exists(dst):
    changed.append("🧫️fixtures/✂️splice → ✂️replace-byte-range")
    if not dry: os.rename(src, dst)
elif not os.path.isdir(dst): problems.append("fixture directory ✂️splice missing")

print(f"{'would change' if dry else 'changed'} {len(changed)}; {len(problems)} problem(s)")
for c in changed: print("  ", c)
for p in problems: print("PROBLEM", p)

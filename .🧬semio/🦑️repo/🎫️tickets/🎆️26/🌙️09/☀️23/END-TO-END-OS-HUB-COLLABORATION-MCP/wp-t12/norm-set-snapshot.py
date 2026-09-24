"""📕️ din18599/en1990 `setSnapshot` honours its declared argument: the manifest declares `snapshot` as the document's
camelCase JSON (like the 13 sibling norms), but the `text` arm of `norm_command_from_action!` handed that JSON to the
DSL-text parser (`invalid document text: expected Enum/Float, found Absent at 1:1`, S15 matrix). The arm now decodes the
declared JSON with the artifact's own `decode_*_snapshot_json` and carries the document as its own escaped DSL text on
the op wire (unchanged payload, unchanged precision guarantee). `escape_op_text_field` becomes production code; the
undeclared `text` argument key is dropped. Laws: the committed ➡️after fixture reaches the handler through the rail."""
import sys
root = "/Users/ueli/Documents/semio/"
N = root + "✏️s/🔌️plugins/📕️norm/"
problems = []

def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if text.count(old) != 1:
            problems.append(f"{path[len(root):]}: {text.count(old)} of {old[:80]!r}")
            continue
        text = text.replace(old, new)
    return path, text

edits = []
edits.append(edit(N + "🖥️app-surface/🦀️.rs", [
    ("""    // 📝️ The `text` shape. Thirteen norm editors declare `ReplaceSnapshot { snapshot: XSnapshot }`
    // and decode the shell's camelCase JSON into it; `⚖️en1990` and `⚡️din18599` declare
    // `ReplaceSnapshot { text: String }` instead, because their snapshot types stopped implementing
    // `dsl::DslField` when `q_k`/`climate` became composed `ArtifactChild<S>` slots, so their payload
    // carries the artifact's own `.en1990`/`.din18599` DSL text on one op-text line. S9's macro knew
    // only the first shape, which is why those two crates failed to check (U3b's 106-crate sweep,
    // 2026-09-21). This arm MUST precede the `$decode:path` arm — `text` would otherwise match
    // `$decode:path` and expand into the wrong body.
    //
    // The argument is taken verbatim: the handler runs `unescape_op_text_field` over it, which is the
    // identity for text carrying no backslash escapes, so a caller passing the document's plain DSL
    // text and a caller passing it in the escaped one-line op-text form both arrive correctly.
    ($command:ident, text) => {
        /// 🌉️ Resolves the React/wgpu shells' `{action, args}` pair into this editor's typed command,
        /// for the two editors whose `setSnapshot` payload carries DSL TEXT rather than a decoded
        /// snapshot struct.
        fn command_from_action""", """    // 📝️ The `text` shape. Thirteen norm editors declare `ReplaceSnapshot { snapshot: XSnapshot }`
    // and decode the shell's camelCase JSON into it; `⚖️en1990` and `⚡️din18599` declare
    // `ReplaceSnapshot { text: String }` instead, because their snapshot types stopped implementing
    // `dsl::DslField` when `q_k`/`climate` became composed `ArtifactChild<S>` slots, so their payload
    // carries the artifact's own `.en1990`/`.din18599` DSL text on one op-text line. The ARGUMENT is
    // the same for all fifteen: `snapshot`, the document's camelCase JSON the manifest declares, decoded
    // here with `$decode` and re-printed as the document's own escaped DSL text. This arm MUST precede
    // the `$decode:path` arm — `text` would otherwise match `$decode:path` and expand into the wrong body.
    ($command:ident, text, $decode:path) => {
        /// 🌉️ Resolves the React/wgpu shells' `{action, args}` pair into this editor's typed command,
        /// for the two editors whose `setSnapshot` payload carries DSL TEXT rather than a decoded
        /// snapshot struct.
        fn command_from_action"""),
    ("""                "setSnapshot" => {
                    let text = args
                        .and_then(|value| value.get("text").or_else(|| value.get("snapshot")))
                        .and_then(|value| if let dsl::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(dsl::json::to_json_string(value)) })
                        .ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.set-snapshot-arg-missing"), "setSnapshot needs a 'text' argument carrying the document's own DSL text"))?;
                    Ok($command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { text }))
                }""", """                "setSnapshot" => {
                    let json = args
                        .and_then(|value| value.get("snapshot"))
                        .and_then(|value| if let dsl::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(dsl::json::to_json_string(value)) })
                        .ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.set-snapshot-arg-missing"), "setSnapshot needs a 'snapshot' argument carrying the document's camelCase JSON"))?;
                    let snapshot = $decode(&json).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.set-snapshot-arg-invalid"), error))?;
                    Ok($command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { text: $crate::document::escape_op_text_field(&store::ArtifactDsl::print_dsl(&snapshot)) }))
                }"""),
]))
edits.append(edit(N + "⚖️compliance/🦀️.rs", [
    ("""#[cfg(any(test, feature = "compliance-testing"))]
pub fn escape_op_text_field(value: &str) -> String {""", """pub fn escape_op_text_field(value: &str) -> String {"""),
]))
for art, name in (("⚡️din18599", "din18599"), ("⚖️en1990", "en1990")):
    cmd = "Din18599Command" if name == "din18599" else "En1990Command"
    edits.append(edit(N + f"🗿️artifacts/{art}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", [
        (f"semio_s_artifact_norm_contract::norm_command_from_action!({cmd}, text);", f"semio_s_artifact_norm_contract::norm_command_from_action!({cmd}, text, crate::standards::v1::subsets::any::schema::snapshot::decode_{name}_snapshot_json);"),
    ]))
if problems:
    for p in problems: print("PROBLEM", p)
    sys.exit(1)
if "--dry" not in sys.argv:
    for path, text in edits: open(path, "w", encoding="utf-8").write(text)
print(("would edit " if "--dry" in sys.argv else "edited ") + str(len(edits)))

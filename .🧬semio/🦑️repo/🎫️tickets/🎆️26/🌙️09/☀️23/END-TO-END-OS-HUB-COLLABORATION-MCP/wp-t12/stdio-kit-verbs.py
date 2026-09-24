#!/usr/bin/env python3
"""🪟️ T12 / S15 stdio ×9: registers each stdio editor's window-kit verb (`set-cell`, `replace-text`,
`set-node`) as an app-owned retained route, so the reactor's `qualified_tool_proof` finds an exact
factory instead of answering `interactive-job.missing-factory`. `handle` and the retained reducer
share one pure `<prefix>_emit`. Usage: stdio-kit-verbs.py [--write]"""
import difflib
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
EDITORS = [
    ("📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any", "csv", "CSV", "CsvEditorCommand", "CsvSnapshot", "set-cell", "the `TableWindowKit`"),
    ("📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any", "tsv", "TSV", "TsvEditorCommand", "TsvSnapshot", "set-cell", "the `TableWindowKit`"),
    ("🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any", "txt", "TXT", "TxtEditorCommand", "TxtSnapshot", "replace-text", "the `TextWindowKit`"),
    ("📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any", "md", "MD", "MdEditCommand", "MdSnapshot", "replace-text", "the `TextWindowKit`"),
    ("🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any", "html", "HTML", "HtmlEditCommand", "HtmlSnapshot", "replace-text", "the `TextWindowKit`"),
    ("🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base", "json_any", "JSON_ANY", "JsonAnyEditorCommand", "JsonSnapshot", "set-node", "the `TreeWindowKit`"),
    ("🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json", "json_i_json", "JSON_I_JSON", "JsonIJsonIJsonEditorCommand", "JsonSnapshot", "set-node", "the `TreeWindowKit`"),
    ("📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base", "xml_any", "XML_ANY", "XmlAnyEditorCommand", "XmlSnapshot", "set-node", "the `TreeWindowKit`"),
    ("📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid", "xml_valid", "XML_VALID", "XmlValidEditorCommand", "XmlSnapshot", "set-node", "the `TreeWindowKit`"),
]
TEXT_VERB_RAW_BYTES = "32_768"


def once(text: str, old: str, new: str, what: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{what}: expected exactly one match, found {count}")
    return text.replace(old, new)


def convert(text: str, fn: str, const: str, command: str, snapshot: str, verb: str, kit: str) -> str:
    kit_const = f"{const}_KIT_ACTION_ID"
    text = text.replace("🎬️ExampleSwitch", "🧵️RetainedRoutes")

    tool_job_doc = re.compile(r"    /// 🎯️ The app-owned retained routes this command channel carries[^\n]*\n(?:    ///[^\n]*\n)*?(?=    const TOOL_JOB_IDS)")
    if len(tool_job_doc.findall(text)) != 1:
        raise SystemExit(f"{fn}: TOOL_JOB_IDS doc")
    text = tool_job_doc.sub(
        "    /// 🎯️ The app-owned retained routes this command channel carries — the join key\n"
        "    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. "
        f"{kit[0].upper()}{kit[1:]}\n"
        f"    /// mints `{verb}`, but only this editor can reduce it into its own mutation, so it is an\n"
        "    /// app-owned route exactly like the example switch.\n",
        text,
    )

    roster_doc = re.compile(r"/// 🧵️ The ONE app-owned retained route this editor declares\.[^\n]*\n(?:///[^\n]*\n)*?(?=const " + const + r"_RETAINED_TOOL_IDS)")
    if len(roster_doc.findall(text)) != 1:
        raise SystemExit(f"{fn}: roster doc")
    text = roster_doc.sub(
        f"/// 🪟️ The verb {kit} mints for `🪟️main` — declared by the framework, reduced only here.\n"
        f"const {kit_const}: &str = \"{verb}\";\n"
        f"/// 🧵️ The app-owned retained routes this editor declares: the example switch and `{verb}`.\n"
        "/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`\n"
        "/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the\n"
        "/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same\n"
        f"/// ids. Without the kit verb's row the reactor refused every `{verb}` with\n"
        "/// `interactive-job.missing-factory`.\n",
        text,
    )
    text = once(
        text,
        f"const {const}_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];",
        f"const {const}_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, {kit_const}];",
        f"{fn}: roster",
    )

    if verb == "replace-text":
        text = once(
            text,
            f"const {const}_RETAINED_RAW_BYTES: usize = 8_192;",
            f"/// 📏️ `{verb}` carries the whole buffer, so the wire bound is the largest document this route\n"
            "/// admits — kept under the guest's 64 KiB contiguous-request ceiling.\n"
            f"const {const}_RETAINED_RAW_BYTES: usize = {TEXT_VERB_RAW_BYTES};",
            f"{fn}: raw bytes",
        )

    text = once(
        text,
        "/// 🚦️ The example switch publishes into NO document lane: it hands the host one\n"
        "/// `Effect::LoadDocument`, so its only lane is `HostOnly`.\n"
        f"const {const}_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] =\n"
        "    &[ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];\n",
        "/// 🚦️ The example switch publishes into NO document lane: it hands the host one\n"
        f"/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `{verb}` publishes the artifact\n"
        "/// mutation it reduces into, so its only lane is `Artifact`.\n"
        f"const {const}_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[\n"
        "    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },\n"
        f"    ArtifactToolPublicationContract {{ tool_id: {kit_const}, lanes: &[ArtifactToolPublicationLane::Artifact] }},\n"
        "];\n",
        f"{fn}: publication contracts",
    )

    text = once(text, f'"{verb}" => Ok(', f"{kit_const} => Ok(", f"{fn}: command_from_action arm")
    text = once(text, f'=> "{verb}",', f"=> {kit_const},", f"{fn}: command_id arm")

    text = once(
        text,
        f"fn {fn}_retained_extent(command: &{command}, _snapshot: &{snapshot}, _interaction: &protocol::InteractionState) -> Option<usize> {{\n"
        f"    matches!(command, {command}::SetActiveExample {{ .. }}).then_some(1)\n}}\n",
        f"fn {fn}_retained_extent(_command: &{command}, _snapshot: &{snapshot}, _interaction: &protocol::InteractionState) -> Option<usize> {{\n"
        "    Some(1)\n}\n",
        f"{fn}: extent",
    )

    handle = re.compile(r"(    fn handle\(\n(?:        [^\n]*\n)*?    \) -> Result<Emit<[^\n]*>, Fault> \{\n)((?:(?:        [^\n]*|)\n)*?)(    \}\n)")
    found = handle.findall(text)
    if len(found) != 1:
        raise SystemExit(f"{fn}: handle ({len(found)})")
    header, body, _ = found[0]
    if "Self" in body:
        raise SystemExit(f"{fn}: handle body names Self")
    new_header = header.replace("        _doc: &ArtifactView", "        doc: &ArtifactView")
    text = text.replace(header + body + "    }\n", new_header + f"        {fn}_emit(command, doc.snapshot)\n    }}\n", 1)
    emit_body = "".join(line[4:] + "\n" if line.strip() else "\n" for line in body.rstrip("\n").split("\n")).replace("doc.snapshot", "snapshot")
    if verb == "replace-text" and "Err(_) => Ok(Emit::default())," in emit_body:
        emit_body = emit_body.replace(
            "Err(_) => Ok(Emit::default()),",
            f'Err(error) => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.{fn}.invalid-text"), error.to_string())),',
        )
    snapshot_param = "_snapshot" if "        _doc: &ArtifactView" in header else "snapshot"
    emit_type = f"Emit<{mutation_of(text, fn)}, NoConfigMutation, NoDraftMutation>"
    emit_fn = (
        f"/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host\n"
        f"/// its document, `{verb}` becomes this artifact's own mutation.\n"
        "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\n"
        f"fn {fn}_emit(command: &{command}, {snapshot_param}: &{snapshot}) -> Result<{emit_type}, Fault> {{\n"
        f"{emit_body}"
        "}\n\n"
    )
    reduce = re.compile(r"(#\[expect\(clippy::too_many_arguments[^\n]*\n(?://[^\n]*\n)*fn " + fn + r"_retained_reduce\(\n)((?:    [^\n]*\n)*?)(\) -> Result<Emit<[^\n]*>, Fault> \{\n)(?:[^\n]*\n)*?(\}\n)")
    found = reduce.findall(text)
    if len(found) != 1:
        raise SystemExit(f"{fn}: reducer ({len(found)})")
    match = reduce.search(text)
    params = match.group(2)
    if params.count(f"    _snapshot: &{snapshot},\n") != 1:
        raise SystemExit(f"{fn}: reducer snapshot param")
    params = params.replace(f"    _snapshot: &{snapshot},\n", f"    snapshot: &{snapshot},\n")
    text = text[: match.start()] + emit_fn + match.group(1) + params + match.group(3) + f"    {fn}_emit(command, snapshot)\n}}\n" + text[match.end():]

    text = once(text, "        let operation = AppOperationContext {\n", f"        let tool_id = {fn}_command_id(&request.command);\n        let operation = AppOperationContext {{\n", f"{fn}: tool id binding")
    text = once(
        text,
        f"BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, {fn}_retained_reduce, {fn}_retained_extent)",
        f"BoundedArtifactCommandWork::new(tool_id, {fn}_retained_reduce, {fn}_retained_extent)",
        f"{fn}: bounded work",
    )
    text = once(text, 'tools: ["setActiveExample"]', f'tools: ["setActiveExample", "{verb}"]', f"{fn}: proofs")
    return text


def mutation_of(text: str, fn: str) -> str:
    found = re.findall(r"fn " + fn + r"_retained_reduce\(\n(?:    [^\n]*\n)*?\) -> Result<Emit<([A-Za-z]+), NoConfigMutation, NoDraftMutation>, Fault>", text)
    if len(found) != 1:
        raise SystemExit(f"{fn}: mutation type")
    return found[0]


def main() -> None:
    write = "--write" in sys.argv
    for relative, fn, const, command, snapshot, verb, kit in EDITORS:
        path = ROOT / relative / "✏️editor" / "🦀️.rs"
        before = path.read_text()
        after = convert(before, fn, const, command, snapshot, verb, kit)
        if write:
            path.write_text(after)
            print(f"wrote {path.relative_to(ROOT)}")
        else:
            sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"a/{fn}", f"b/{fn}", n=1))


if __name__ == "__main__":
    main()

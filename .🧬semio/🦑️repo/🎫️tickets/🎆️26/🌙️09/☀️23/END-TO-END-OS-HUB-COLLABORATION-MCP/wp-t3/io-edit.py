"""WP-T3 helper: drop import dialects from a legacy per-artifact `🚪️io/🦀️.rs` DerivedComposition.

usage: python3 io-edit.py <io-🦀️.rs> [--drop-export fmt]... [--txt-export OWNER_DIALECT] [--kinds import|export "stdio.a,stdio.b"]... DEP_A DEP_B ...
Removes each `const DEP_X` line, its entry in `reads()`, and its `if source.dialect == DEP_X { … }` block;
`--txt-export` registers the txt export composer entry; `--kinds` rewrites the declared stdio kind list.
"""
import re
import sys

path, *args = sys.argv[1:]
text = open(path, encoding="utf8").read()
deps, txt_owner, kinds, drops = [], None, {}, []
while args:
    head = args.pop(0)
    if head == "--txt-export":
        txt_owner = args.pop(0)
    elif head == "--drop-export":
        drops.append(args.pop(0))
    elif head == "--kinds":
        direction, listed = args.pop(0), args.pop(0)
        kinds[direction] = listed
    else:
        deps.append(head)
for fmt in drops:
    upper = fmt.upper()
    text, n = re.subn(rf"^\s*const EXPORT_{upper}_DIALECT: Dialect = .*\n", "", text, flags=re.M)
    if n != 1:
        raise SystemExit(f"EXPORT_{upper}_DIALECT not found exactly once ({n})")
    start = text.find(f"    fn compose_export_{fmt}(")
    if start == -1:
        raise SystemExit(f"compose_export_{fmt} not found")
    depth, i, opened = 0, start, False
    while True:
        if text[i] == "{":
            depth += 1
            opened = True
        elif text[i] == "}":
            depth -= 1
            if opened and depth == 0:
                break
        i += 1
    text = text[:start] + text[text.find("\n", i) + 1 :]
    text, n = re.subn(rf"^\s*ComposerEntry {{ writes: EXPORT_{upper}_DIALECT, .*\n", "", text, flags=re.M)
    if n != 1:
        raise SystemExit(f"entry for EXPORT_{upper}_DIALECT not found exactly once ({n})")
for direction, listed in kinds.items():
    body = ", ".join(f'"{k}"' for k in listed.split(",") if k)
    text, n = re.subn(rf"(pub fn {direction}_stdio_kinds\(\) -> &'static \[&'static str\] \{{\n    &\[)[^\]]*(\])", rf"\g<1>{body}\g<2>", text)
    if n != 1:
        raise SystemExit(f"{direction}_stdio_kinds not found")
if txt_owner:
    if "compose_export_txt" in text:
        raise SystemExit("txt export already registered")
    anchor = "    //#endregion 🔖️ExportEntries"
    if text.count(anchor) != 1:
        raise SystemExit("ExportEntries region not found")
    prefix = re.search(r"(crate(?:::[a-z0-9_]+)*)::export::serializers::artifacts::", text).group(1)
    awaited = ".await" if "rebuild_native_snapshot(sources).await?" in text else ""
    text = text.replace(anchor, """    const EXPORT_TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    fn compose_export_txt(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources){AWAIT}?;
            let bytes = {PREFIX}::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_TXT_DIALECT, payload: IoPayload::Text(String::from_utf8(bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?), diagnostics: Vec::new(), confidence: IoConfidence::High })
        })
    }
""".replace("{PREFIX}", prefix).replace("{AWAIT}", awaited) + anchor)
    last = list(re.finditer(rf"                    ComposerEntry {{ writes: EXPORT_[A-Z0-9_]+_DIALECT, reads: &\[{txt_owner}\], compose: compose_export_[a-z0-9_]+ }},\n", text))
    if not last:
        raise SystemExit("no export ComposerEntry rows found")
    at = last[-1].end()
    text = text[:at] + f"                    ComposerEntry {{ writes: EXPORT_TXT_DIALECT, reads: &[{txt_owner}], compose: compose_export_txt }},\n" + text[at:]
for dep in deps:
    const = re.compile(rf"^\s*const {dep}: Dialect = .*\n", re.M)
    if len(const.findall(text)) != 1:
        raise SystemExit(f"{dep}: const not found exactly once")
    text = const.sub("", text)
    text, n = re.subn(rf", {dep}\b", "", text)
    if n != 1:
        raise SystemExit(f"{dep}: reads() entry not found exactly once ({n})")
    start = text.find(f"if source.dialect == {dep} {{")
    if start == -1:
        raise SystemExit(f"{dep}: compose branch not found")
    line_start = text.rfind("\n", 0, start) + 1
    depth, i = 0, start
    while True:
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                break
        i += 1
    end = text.find("\n", i) + 1
    text = text[:line_start] + text[end:]
open(path, "w", encoding="utf8").write(text)
print(f"dropped {', '.join(deps) or 'nothing'}; exports dropped {drops or 'none'}; txt export {'added' if txt_owner else 'unchanged'}; kinds {kinds or 'unchanged'}")

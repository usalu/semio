"""🪆️ Turns a per-kind handler factory `pub fn <name>(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> { move |ctx: &Context| { … } }`
into a plain handler that reads its kind from the scenario's own doc string (`{"kind": …}`), so one registration under the
Scenario Outline's base id serves every row.

With `<name>:row` the kind is the scenario's own Examples row id (`ctx.row()`), for outlines whose doc string carries no kind.

Usage: unfactory-handlers.py <adapter.rs> <name>[:row]..."""
import re, sys
path, names = sys.argv[1], sys.argv[2:]
text = open(path, encoding="utf-8").read()
for spec in names:
    name, _, source = spec.partition(":")
    head = re.search(rf"( *)pub fn {name}\(kind: &'static str\) -> impl Fn\(&Context\) -> Result<Outcome, String> \{{\n\1    move \|ctx: &Context\| \{{\n", text)
    assert head, name
    indent = head.group(1)
    start = head.end()
    depth, i = 1, start
    while depth:
        if text[i] == "{": depth += 1
        elif text[i] == "}": depth -= 1
        i += 1
    body_end = i
    closure_body = text[start:body_end - 1]
    outer_close = text.index("}", body_end) + 1
    lines = closure_body.rstrip().split("\n")
    body = "\n".join(line[4:] if line.startswith(indent + "        ") else line for line in lines[:-0 or None])
    first = body.split("\n", 1)
    reads_doc = None if source == "row" else re.match(r"\s*let (\w+) = ctx\.doc_json\(\)\?;", first[0])
    kind_line = f"{indent}    let kind = {reads_doc.group(1)}.str(\"kind\");\n{indent}    let kind = kind.as_str();" if reads_doc else (f"{indent}    let kind = ctx.row()?;" if source == "row" else f"{indent}    let spec = ctx.doc_json()?;\n{indent}    let kind = spec.str(\"kind\");\n{indent}    let kind = kind.as_str();")
    new_body = (first[0] + "\n" + kind_line + ("\n" + first[1] if len(first) > 1 else "")) if reads_doc else (kind_line + "\n" + body)
    replacement = f"{indent}pub fn {name}(ctx: &Context) -> Result<Outcome, String> {{\n{new_body.rstrip()}\n{indent}}}"
    text = text[:head.start()] + replacement + text[outer_close:]
open(path, "w", encoding="utf-8").write(text)
print("ok", path.split("/🧪️tests/")[-1])

import pathlib

root = pathlib.Path(__file__).resolve().parents[7]
probe = pathlib.Path(__file__).with_name("schema_short.py").read_text(encoding="utf-8")
schema_line = next(line for line in probe.splitlines() if line.startswith("schema_path = "))
namespace = {"root": root, "pathlib": pathlib}
exec(schema_line, namespace)
schema_path = namespace["schema_path"]
quiz = schema_path.parents[1]
validation_ts = next(quiz.rglob("validation/*.ts")) if False else None
modules = next(child for child in quiz.iterdir() if child.name.endswith("modules") or "modules" in child.name)
validation = next(child for child in modules.iterdir() if "validation" in child.name)
ts = next(validation.glob("*.ts"))
rs = next(validation.glob("*.rs"))
deploy = next(path for path in (root / next(child.name for child in root.iterdir() if child.name.endswith("teaching"))).rglob("*.ts") if path.name.endswith(".ts") and "deploy" in str(path.parent) and path.parent.parent.name.endswith("tests"))

def sub(path: pathlib.Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    if old not in text:
        raise SystemExit(f"missing pattern in {path}: {old[:80]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")
    print("patched", path)

sub(
    ts,
    'const json = object(quiz, "", report, ["schema", "id", "emoji", "title", "description", "tasks"], ["$schema"]);\n    if (!json) return;\n    if (Object.hasOwn(json, "$schema")) string(json.$schema, "/$schema", report);\n    if (Object.hasOwn(json, "schema") && json.schema !== QUIZ_SCHEMA) report("/schema", "value-invalid");\n    if (Object.hasOwn(json, "id")) slug(json.id, "/id", report);\n    if (Object.hasOwn(json, "emoji")) string(json.emoji, "/emoji", report, 1, 16);\n    if (Object.hasOwn(json, "title")) text(json.title, "/title", report);',
    'const json = object(quiz, "", report, ["schema", "id", "emoji", "title", "description", "tasks"], ["$schema", "short"]);\n    if (!json) return;\n    if (Object.hasOwn(json, "$schema")) string(json.$schema, "/$schema", report);\n    if (Object.hasOwn(json, "schema") && json.schema !== QUIZ_SCHEMA) report("/schema", "value-invalid");\n    if (Object.hasOwn(json, "id")) slug(json.id, "/id", report);\n    if (Object.hasOwn(json, "emoji")) string(json.emoji, "/emoji", report, 1, 16);\n    if (Object.hasOwn(json, "title")) text(json.title, "/title", report);\n    short(json, "", report);',
)
sub(
    ts,
    'const json = object(catalog, "", report, ["schema", "id", "title", "introduction", "quizzes", "badges"], ["$schema"]);\n    if (!json) return;\n    if (Object.hasOwn(json, "$schema")) string(json.$schema, "/$schema", report);\n    if (Object.hasOwn(json, "schema") && json.schema !== CATALOG_SCHEMA) report("/schema", "value-invalid");\n    if (Object.hasOwn(json, "id")) slug(json.id, "/id", report);\n    if (Object.hasOwn(json, "title")) text(json.title, "/title", report);',
    'const json = object(catalog, "", report, ["schema", "id", "title", "introduction", "quizzes", "badges"], ["$schema", "short"]);\n    if (!json) return;\n    if (Object.hasOwn(json, "$schema")) string(json.$schema, "/$schema", report);\n    if (Object.hasOwn(json, "schema") && json.schema !== CATALOG_SCHEMA) report("/schema", "value-invalid");\n    if (Object.hasOwn(json, "id")) slug(json.id, "/id", report);\n    if (Object.hasOwn(json, "title")) text(json.title, "/title", report);\n    short(json, "", report);',
)
sub(
    rs,
    '    issues.length("/emoji".to_string(), &quiz.emoji, 1, 16);\n    issues.text("/title", &quiz.title);\n    issues.text("/description", &quiz.description);',
    '    issues.length("/emoji".to_string(), &quiz.emoji, 1, 16);\n    issues.text("/title", &quiz.title);\n    issues.short("", quiz.short.as_ref());\n    issues.text("/description", &quiz.description);',
)
sub(
    rs,
    '    issues.slug("/id".to_string(), &catalog.id);\n    issues.text("/title", &catalog.title);\n    issues.text("/introduction/title", &catalog.introduction.title);',
    '    issues.slug("/id".to_string(), &catalog.id);\n    issues.text("/title", &catalog.title);\n    issues.short("", catalog.short.as_ref());\n    issues.text("/introduction/title", &catalog.introduction.title);',
)
sub(
    deploy,
    'expect(deploymentDrift(drifted("package.json", (text) => text.replace("@teaching/architecture-quiz:docker-stack-check", "@teaching/architecture-quiz:stack-check")))).toEqual(["package.json names @teaching/architecture-quiz:stack-check, which is not a target of the package"]);',
    'expect(deploymentDrift(drifted("package.json", (text) => `${text}\\n{"run":"@teaching/architecture-quiz:stack-check"}\\n`))).toEqual(["package.json names @teaching/architecture-quiz:stack-check, which is not a target of the package"]);',
)
doc = next(path for path in quiz.rglob("*.ts") if path.parent.name.endswith("document-validation") or "document-validation" in str(path.parent))
sub(
    doc,
    '["a short label on a task", (quiz) => (quiz.tasks[1].short = T("Power")), [{ path: "/tasks/1/short", code: "property-unknown" }]],',
    '["a short quiz label of 41 code points", (quiz) => (quiz.short = T("t".repeat(41), "Physik")), [{ path: "/short/en", code: "length-invalid" }]],\n    ["a short label on a task", (quiz) => (quiz.tasks[1].short = T("Power")), [{ path: "/tasks/1/short", code: "property-unknown" }]],',
)
unit = next(path for path in validation.rglob("*.rs") if path.parent.name.endswith("unit"))
sub(
    unit,
    'fn quiz_emoji_is_required_and_holds_one_to_sixteen_code_points() {',
    '''fn quiz_and_catalog_short_labels_hold_at_most_forty_code_points() {
    let mut valid = quiz();
    valid.short = Some(Text { en: "Physics".to_string(), de: "Physik".to_string() });
    assert_eq!(quiz_issues(&valid), []);
    let mut invalid = quiz();
    invalid.short = Some(Text { en: "p".repeat(41), de: "Physik".to_string() });
    assert_eq!(quiz_issues(&invalid), [issue("/short/en", IssueCode::LengthInvalid)]);
    let mut document = catalog();
    document.short = Some(Text { en: "A&T".to_string(), de: "A&T".to_string() });
    assert_eq!(catalog_issues(&document, &[quiz()]), []);
    document.short = Some(Text { en: "A&T".to_string(), de: "q".repeat(41) });
    assert_eq!(catalog_issues(&document, &[quiz()]), [issue("/short/de", IssueCode::LengthInvalid)]);
}

#[test]
fn quiz_emoji_is_required_and_holds_one_to_sixteen_code_points() {''',
)
for path in quiz.rglob("*.rs"):
    text = path.read_text(encoding="utf-8")
    updated = text.replace(
        'title: text("Architecture"),\n        introduction:',
        'title: text("Architecture"),\n        short: None,\n        introduction:',
    ).replace(
        'title: text("Energy"),\n        description: text("About energy"),',
        'title: text("Energy"),\n        short: None,\n        description: text("About energy"),',
    )
    if updated != text:
        path.write_text(updated, encoding="utf-8")
        print("constructors", path)

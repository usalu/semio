import json
import pathlib

root = pathlib.Path(__file__).resolve().parents[7]
schema_path = root / "🧰️framework" / "🛍️products" / "❓️quiz" / "🧬️schema" / "🔣️.json"
schema = json.loads(schema_path.read_text(encoding="utf-8"))
defs = schema.get("$defs") or {}
print("defs", len(defs))
quiz = schema_path.parents[1]
modules = next(path for path in quiz.iterdir() if path.name.endswith("modules"))
validation = next(path for path in modules.iterdir() if path.name.endswith("validation"))
ts = next(path for path in validation.iterdir() if path.name.endswith(".ts"))
rs = next(path for path in validation.iterdir() if path.name.endswith(".rs"))
teaching = next(path for path in root.iterdir() if path.name.endswith("teaching"))
deploy = next(path for path in teaching.rglob("*.ts") if path.parent.name.endswith("deploy") and "tests" in str(path.parent.parent))
doc = next(path for path in quiz.rglob("*.ts") if path.parent.name.endswith("document-validation"))
unit = next(path for path in validation.rglob("*.rs") if path.parent.name.endswith("unit"))

def sub(path: pathlib.Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    if old not in text:
        raise SystemExit(f"missing pattern in {path.name}: {old[:60]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")
    print("patched", path.name)

sub(ts, '["schema", "id", "emoji", "title", "description", "tasks"], ["$schema"]', '["schema", "id", "emoji", "title", "description", "tasks"], ["$schema", "short"]')
sub(ts, 'if (Object.hasOwn(json, "title")) text(json.title, "/title", report);\n    if (Object.hasOwn(json, "description")) text(json.description, "/description", report);', 'if (Object.hasOwn(json, "title")) text(json.title, "/title", report);\n    short(json, "", report);\n    if (Object.hasOwn(json, "description")) text(json.description, "/description", report);')
sub(ts, '["schema", "id", "title", "introduction", "quizzes", "badges"], ["$schema"]', '["schema", "id", "title", "introduction", "quizzes", "badges"], ["$schema", "short"]')
sub(ts, 'if (Object.hasOwn(json, "title")) text(json.title, "/title", report);\n    if (Object.hasOwn(json, "introduction"))', 'if (Object.hasOwn(json, "title")) text(json.title, "/title", report);\n    short(json, "", report);\n    if (Object.hasOwn(json, "introduction"))')
sub(rs, 'issues.text("/title", &quiz.title);\n    issues.text("/description", &quiz.description);', 'issues.text("/title", &quiz.title);\n    issues.short("", quiz.short.as_ref());\n    issues.text("/description", &quiz.description);')
sub(rs, 'issues.text("/title", &catalog.title);\n    issues.text("/introduction/title", &catalog.introduction.title);', 'issues.text("/title", &catalog.title);\n    issues.short("", catalog.short.as_ref());\n    issues.text("/introduction/title", &catalog.introduction.title);')
sub(deploy, 'text.replace("@teaching/architecture-quiz:docker-stack-check", "@teaching/architecture-quiz:stack-check")', '`${text}\\n{"run":"@teaching/architecture-quiz:stack-check"}\\n`')
sub(doc, '["a short label on a task",', '["a short quiz label of 41 code points", (quiz) => (quiz.short = T("t".repeat(41), "Physik")), [{ path: "/short/en", code: "length-invalid" }]],\n    ["a short label on a task",')
marker = "fn quiz_emoji_is_required_and_holds_one_to_sixteen_code_points() {"
insert = '''fn quiz_and_catalog_short_labels_hold_at_most_forty_code_points() {
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
'''
sub(unit, marker, insert + marker)
for path in quiz.rglob("*.rs"):
    text = path.read_text(encoding="utf-8")
    updated = text.replace('title: text("Architecture"),\n        introduction:', 'title: text("Architecture"),\n        short: None,\n        introduction:').replace('title: text("Energy"),\n        description: text("About energy"),', 'title: text("Energy"),\n        short: None,\n        description: text("About energy"),')
    if updated != text:
        path.write_text(updated, encoding="utf-8")
        print("constructors", path.name, path.parent.parent.name)

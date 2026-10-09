import pathlib

root = pathlib.Path(__file__).resolve().parents[7]
probe = pathlib.Path(__file__).with_name("schema_short.py").read_text(encoding="utf-8")
schema_line = next(line for line in probe.splitlines() if line.startswith("schema_path = "))
namespace = {"root": root, "pathlib": pathlib}
exec(schema_line, namespace)
quiz = namespace["schema_path"].parents[1]
modules = next(path for path in quiz.iterdir() if path.name.endswith("modules"))
validation = next(path for path in modules.iterdir() if path.name.endswith("validation"))
ts = next(path for path in validation.iterdir() if path.name.endswith(".ts")).read_text(encoding="utf-8")
start = ts.find("export function quizIssues")
print(ts[start:start + 700])
print("---CATALOG---")
start = ts.find("export function catalogIssues")
print(ts[start:start + 500])
teaching = next(path for path in root.iterdir() if path.name.endswith("teaching"))
deploy = next(path for path in teaching.rglob("*.ts") if path.parent.name.endswith("deploy") and "tests" in str(path.parent.parent))
lines = deploy.read_text(encoding="utf-8").splitlines()
for i, line in enumerate(lines, 1):
    if "stack-check" in line:
        print(i, line)

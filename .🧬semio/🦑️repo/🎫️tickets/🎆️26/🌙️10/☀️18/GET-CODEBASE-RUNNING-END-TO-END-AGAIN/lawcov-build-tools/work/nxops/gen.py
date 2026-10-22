"""📝 Authoring aid: writes the Nx operation argument corpus (corpus.json)."""
import itertools, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
bases = [[], ["--base=main"], ["--base", "main"], ["--base"], ["--head=feature"], ["--head", "feature"]]
files = [[], ["--files=a.ts,b.ts"], ["--files", "c.ts"], ["--files"]]
changes = [[], ["--uncommitted"], ["--untracked"], ["--uncommitted", "--untracked"], ["--uncommitted=false", "--untracked"], ["--all"], ["--all=false"], ["--untracked=false"]]
extras = [[], ["--target=build"], ["--parallel", "4"], ["--verbose"]]
affected = []
for base, file, change, extra in itertools.product(bases, files, changes, extras):
    if len(base) + len(file) + len(change) + len(extra) > 6:
        continue
    affected.append(["affected", *base, *file, *change, *extra])
for tail in (["--", "--base=zzz"], ["--", "--all"], ["--", "x", "--files=y"]):
    affected.append(["affected", "--base=main", *tail])
    affected.append(["affected", "--all", *tail])
refused = [["affected", "--projects=a"], ["affected", "-p", "a"], ["affected", "-p=a"], ["affected", "--projects", "a", "--base=x"], ["affected", "--base=x", "--projects"]]
operations = [
    {"args": ["cache", "clear"], "expected": ["reset", "--onlyCache"]},
    {"args": ["cache"], "expected": None}, {"args": ["cache", "other"], "expected": None}, {"args": ["cache", "clear", "extra"], "expected": ["reset", "--onlyCache"]},
    {"args": ["inputs", "p:t"], "expected": ["show", "target", "inputs", "p:t", "--json"]},
    {"args": ["inputs", "p:t:c"], "expected": ["show", "target", "inputs", "p:t:c", "--json"]},
    {"args": ["inputs", "@s/p:t"], "expected": ["show", "target", "inputs", "@s/p:t", "--json"]},
    {"args": ["inputs", "--help"], "expected": None}, {"args": ["inputs", "p:t", "-h"], "expected": None}, {"args": ["inputs", "-h"], "expected": None},
    {"args": ["inputs"], "error": "Expected repo inputs <project:target[:configuration]>"},
    {"args": ["inputs", "p"], "error": "Expected repo inputs <project:target[:configuration]>"},
    {"args": ["inputs", "p:t:c:d"], "error": "Expected repo inputs <project:target[:configuration]>"},
    {"args": ["inputs", "p t:c"], "error": "Expected repo inputs <project:target[:configuration]>"},
    {"args": ["inputs", ":t"], "error": "Expected repo inputs <project:target[:configuration]>"},
    {"args": ["inputs", "p:"], "error": "Expected repo inputs <project:target[:configuration]>"},
    {"args": ["inputs", "p:t", "extra"], "error": "Expected repo inputs <project:target[:configuration]>"},
]
c = {"version": 1, "affected": affected, "refused": refused, "operations": operations,
     "exec": {"environment": {"NX_PROJECT_NAME": "project-x", "NX_PROJECT_ROOT_PATH": "owners/x"}, "mapping": {"NX_PROJECT_NAME": "SEMIO_PROJECT_NAME", "NX_PROJECT_ROOT_PATH": "SEMIO_PROJECT_ROOT"}, "exitCodes": [0, 3, 7]}}
c["authored"] = [
    {"args": ["affected"], "expected": ["affected", "--base=HEAD"]},
    {"args": ["affected", "--base=main"], "expected": ["affected", "--base=main"]},
    {"args": ["affected", "--base", "main", "--head=f"], "expected": ["affected", "--base=main", "--head=f"]},
    {"args": ["affected", "--uncommitted"], "expected": ["affected", "--uncommitted"]},
    {"args": ["affected", "--untracked"], "expected": ["affected", "--untracked"]},
    {"args": ["affected", "--uncommitted", "--untracked"], "expected": ["affected", "--base=HEAD"]},
    {"args": ["affected", "--files=a,b", "--base=main"], "expected": ["affected", "--files=a,b"]},
    {"args": ["affected", "--all", "--target=build"], "expected": ["run-many", "--target=build"]},
    {"args": ["affected", "--target=test", "--", "--base=z"], "expected": ["affected", "--target=test", "--base=HEAD", "--", "--base=z"]},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False)
print(len(affected))

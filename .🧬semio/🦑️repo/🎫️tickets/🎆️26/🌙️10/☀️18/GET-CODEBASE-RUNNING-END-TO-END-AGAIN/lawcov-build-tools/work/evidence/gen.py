"""📝 Authoring aid: writes the Bazel evidence corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
BAD = "SEMIO_BUILD_EVIDENCE_DIR must be an absolute path of at most 200 characters"


def d(name, env, expected, error=""):
    return {"name": name, "env": env, "expected": expected, "error": error}


c = {"version": 1}
c["directories"] = [
    d("not set", {"kind": "unset"}, "none"),
    d("empty is the same as not set", {"kind": "empty"}, "none"),
    d("an absolute directory", {"kind": "absolute", "name": "evidence"}, "same"),
    d("a relative directory is refused", {"kind": "relative", "name": "evidence"}, "none", BAD),
    d("exactly 200 characters are accepted", {"kind": "absolute", "name": "e", "padTo": 200}, "same"),
    d("201 characters are refused", {"kind": "absolute", "name": "e", "padTo": 201}, "none", BAD),
    d("a dot directory is relative", {"kind": "relative", "name": "."}, "none", BAD),
]
plan = lambda tasks, selected: {"tasks": tasks, "selected": selected}
t = lambda label, env=None: {"label": label, "env": env}
c["closures"] = [
    {"name": "the closure query, the label map and the declared environments", "plan": plan({"build": t("repo_build", ["B", "A"]), "test": t("repo_test", [])}, ["build", "test"]),
     "answer": "//:repo_build (abc123)\n//:repo_test\n//:repo_helper   (def456)  \n", "expression": 'kind("repo_", deps(set(//:repo_build //:repo_test)))'},
    {"name": "duplicates and non-label lines are dropped and the rest is sorted", "plan": plan({"a": t("repo_a")}, ["a"]),
     "answer": "INFO: noise\n//:z\n//:a\n//:z\n\n  //:m (x)  \nWARNING: other\n", "expression": 'kind("repo_", deps(set(//:repo_a)))'},
    {"name": "an empty answer yields an empty closure", "plan": plan({"a": t("repo_a")}, ["a"]), "answer": "", "expression": 'kind("repo_", deps(set(//:repo_a)))'},
    {"name": "tasks without environment declare none", "plan": plan({"a": t("repo_a", None), "b": t("repo_b", ["X"])}, ["b"]), "answer": "//:repo_b\n", "expression": 'kind("repo_", deps(set(//:repo_b)))'},
    {"name": "unicode labels survive sorting", "plan": plan({"é": t("repo_é")}, ["é"]), "answer": "//:repo_é (1)\n//:repo_a\n", "expression": 'kind("repo_", deps(set(//:repo_é)))'},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)

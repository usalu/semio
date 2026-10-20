#!/usr/bin/env python3
"""Builds staged/ from src/ and gen/: substitutes the path tokens with the exact names found on disk and records the plan.

The authored sources spell every emoji path component as a token (@TS@, @SCHEMA@, ...), because a typed emoji may drop its
variation selector and then name a directory that does not exist. Tokens are resolved here, by comparing the names of the
real directories with the selector stripped, so the staged files carry the names of the live tree byte for byte.

  python3 build.py --root <checkout>        writes staged/ (files, targets.json with anchored edits and base hashes)

Stdlib only; runs on macOS, Linux, Windows and in the devcontainer.
"""
import argparse, hashlib, json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
VS16 = "️"


def strip(name):
    return name.replace(VS16, "")


def real(root, relative):
    """Resolves a path whose components may lack variation selectors to the names that exist."""
    path = root
    for part in relative.split("/"):
        match = [n for n in os.listdir(path) if n == part or strip(n) == strip(part)]
        if len(match) != 1:
            raise SystemExit(f"cannot resolve {part!r} below {path}: {match}")
        path = os.path.join(path, match[0])
    return os.path.relpath(path, root).replace(os.sep, "/")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=os.path.abspath(os.path.join(HERE, *[".."] * 8)))
    args = parser.parse_args()
    root = args.root
    B = real(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build")
    Z = B + "/" + real(os.path.join(root, B), "🔧️tools/🏗️bazel")
    base_b, base_z = os.path.join(root, B), os.path.join(root, Z)
    remote_tests = real(base_b, "🌐️remote-cache/🧪️tests")
    receipt = real(base_b, "🧾️store-receipt").split("/")[-1]
    emoji = receipt[: receipt.index("store-receipt")]
    validator = real(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts").split("/", 1)[1]
    tokens = {
        "@TS@": real(base_b, "🌐️remote-cache/🟦️.ts").split("/")[-1],
        "@JSON@": real(base_b, f"{remote_tests}/🧫️fixtures/🔣️.json").split("/")[-1],
        "@SCHEMA@": real(base_b, "🧬️schema").split("/")[-1],
        "@TESTS@": remote_tests.split("/")[-1],
        "@FIX@": real(base_b, f"{remote_tests}/🧫️fixtures").split("/")[-1],
        "@SCRIPT@": real(base_z, "📜️script.ts").split("/")[-1],
        "@CIREPORT@": emoji + "ci-report",
        "@TESTRESULTS@": emoji + "test-results",
        "@OPERATIONS@": real(base_z, "⚙️operations").split("/")[-1],
        "@VALIDATOR@": validator,
    }
    tokens["@CI_REPORT_PATH@"] = f"{B}/{tokens['@CIREPORT@']}/{tokens['@SCRIPT@']}"

    def render(text):
        for key in sorted(tokens, key=len, reverse=True):
            text = text.replace(key, tokens[key])
        left = re.findall(r"@[A-Z_]+@", text)
        if left:
            raise SystemExit(f"unresolved tokens {sorted(set(left))}")
        return text

    out = os.path.join(HERE, "staged")
    if os.path.isdir(out):
        shutil.rmtree(out)
    plan = {"version": 1, "groups": {"report": {"new": []}, "adapter": {"new": [], "edits": []}, "workflow": {"new": [], "replace": [], "edits": []}}, "tokens": tokens}

    def stage(group, source, name, live, kind="new", data=None, entry=None):
        text = data if data is not None else render(open(os.path.join(HERE, source), encoding="utf8").read())
        target = os.path.join(out, group, name)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "w", encoding="utf8", newline="\n") as handle:
            handle.write(text)
        row = {"staged": f"{group}/{name}", "live": live, "sha256": sha(text.encode("utf8"))}
        if entry:
            row.update(entry)
        plan["groups"][group][kind].append(row)

    ci = f"{B}/{tokens['@CIREPORT@']}"
    stage("report", "src/ci-report/module.ts", "module.ts", f"{ci}/{tokens['@TS@']}")
    stage("report", "src/ci-report/script.ts", "script.ts", f"{ci}/{tokens['@SCRIPT@']}")
    stage("report", "src/ci-report/schema.json", "schema.json", f"{ci}/{tokens['@SCHEMA@']}/{tokens['@JSON@']}")
    stage("report", "src/ci-report/test.ts", "test.ts", f"{ci}/{tokens['@TESTS@']}/{tokens['@TS@']}")
    stage("report", "gen/ci-report-fixture.json", "fixture.json", f"{ci}/{tokens['@TESTS@']}/{tokens['@FIX@']}/{tokens['@JSON@']}", data=open(os.path.join(HERE, "gen/ci-report-fixture.json"), encoding="utf8").read())
    tr = f"{Z}/{tokens['@TESTRESULTS@']}"
    stage("adapter", "src/test-results/module.ts", "module.ts", f"{tr}/{tokens['@TS@']}")
    stage("adapter", "src/test-results/test.ts", "test.ts", f"{tr}/{tokens['@TESTS@']}/{tokens['@TS@']}")
    stage("adapter", "gen/test-results-fixture.json", "fixture.json", f"{tr}/{tokens['@TESTS@']}/{tokens['@FIX@']}/{tokens['@JSON@']}", data=open(os.path.join(HERE, "gen/test-results-fixture.json"), encoding="utf8").read())
    for name in ("bun-repo", "cache-proof"):
        stage("workflow", f"src/workflow/actions/{name}/action.yml", f"actions/{name}/action.yml", f".github/actions/{name}/action.yml")
    live_workflow = os.path.join(root, ".github/workflows/build-system.yml")
    base = sha(open(live_workflow, "rb").read())
    stage("workflow", "src/workflow/build-system.yml", "build-system.yml", ".github/workflows/build-system.yml", kind="replace", entry={"base": base})

    operations = f"{Z}/{tokens['@OPERATIONS@']}/{tokens['@TS@']}"
    live_operations = open(os.path.join(root, operations), encoding="utf8").read()
    import_line = re.search(r"^import \{bazelEvidenceDirectoryV1,bazelEvidenceEventsPathV1\} from [^\n]*$", live_operations, re.M)
    copy_line = "  if(evidence!==undefined)await copyFile(events,bazelEvidenceEventsPathV1(evidence));\n"
    if import_line is None or live_operations.count(copy_line) != 1:
        raise SystemExit("operations anchors moved; adapt build.py")
    new_import = f'\nimport {{bazelTestResultsDirectoryV1,exportBazelTestResultsV1}} from "../{tokens["@TESTRESULTS@"]}/{tokens["@TS@"]}";'
    new_call = "  const testResults=bazelTestResultsDirectoryV1(process.env);\n  if(testResults!==undefined)await exportBazelTestResultsV1(text,testResults);\n"
    plan["groups"]["adapter"]["edits"] += [
        {"live": operations, "old": import_line.group(0), "new": import_line.group(0) + new_import, "name": "operations-import"},
        {"live": operations, "old": copy_line, "new": copy_line + new_call, "name": "operations-export"},
    ]
    dependabot = open(os.path.join(root, ".github/dependabot.yml"), encoding="utf8").read()
    anchor = '  - package-ecosystem: "github-actions"\n    directories:\n      - "/"\n'
    if dependabot.count(anchor) != 1:
        raise SystemExit("dependabot anchor moved; adapt build.py")
    plan["groups"]["workflow"]["edits"].append({"live": ".github/dependabot.yml", "old": anchor, "new": anchor + '      - "/.github/actions/bun-repo"\n      - "/.github/actions/cache-proof"\n', "name": "dependabot-local-actions"})
    with open(os.path.join(out, "targets.json"), "w", encoding="utf8", newline="\n") as handle:
        json.dump(plan, handle, indent=1, ensure_ascii=False)
    longest = max(len(row["live"].encode("utf8")) for group in plan["groups"].values() for kind in ("new", "replace") for row in group.get(kind, []))
    print("staged", sum(len(group.get("new", [])) + len(group.get("replace", [])) for group in plan["groups"].values()), "files; longest live path", longest, "bytes (limit 256)")
    assert longest <= 256


if __name__ == "__main__":
    sys.exit(main())

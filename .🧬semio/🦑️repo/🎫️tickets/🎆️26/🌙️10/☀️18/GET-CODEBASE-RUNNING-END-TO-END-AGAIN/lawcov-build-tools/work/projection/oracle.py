"""📐️ Independent Python oracle of the dependency projection (standard library only): it filters workspace links and writes the two installation inputs with the same compact JSON text. Requests arrive as JSON on stdin."""
import json, sys


def dumps(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def pins(dependencies):
    return {name: version for name, version in dependencies.items() if version != "workspace:*"}


def project(recipe, lock):
    package = dumps({"name": recipe["name"], "private": recipe["private"], "packageManager": recipe["packageManager"], "dependencies": pins(recipe["dependencies"])}) + "\n"
    captured = {key: value for key, value in lock.items() if key != "patchedDependencies"}
    return {"package.json": package, "bun.lock": dumps(captured) + "\n"}


def main():
    request = json.load(sys.stdin)
    if request["op"] == "pins":
        print(json.dumps([pins(row) for row in request["rows"]]))
    else:
        print(json.dumps([project(row["recipe"], row["lock"]) for row in request["rows"]]))


main()

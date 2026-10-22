"""📝 Authoring aid: writes the extra projection corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
BASE = {"name": "@semio-tech/nx-bootstrap-tools", "private": True, "packageManager": "bun@1.3.14", "dependencies": {"nx": "23.2.0"}, "semio": {"toolPatches": {"nx@23.2.0": "patches/current.patch"}}}
LOCK = {"lockfileVersion": 1, "workspaces": {"": {"name": "x"}}, "patchedDependencies": {"nx@23.2.0": "patches/current.patch"}, "packages": {"nx": ["nx@23.2.0", "", {}, "sha512-pinned"]}}


def case(name, recipe, lock, error="", package=None, lockText=None):
    return {"name": name, "recipe": recipe, "lock": lock, "error": error, "package": package, "lockText": lockText}


def recipe(**changes):
    row = json.loads(json.dumps(BASE))
    row.update(changes)
    return row


CLOSED = "Closed pinned dependency recipe required"
c = {"version": 1}
c["pins"] = [
    {"name": "workspace links are dropped", "dependencies": {"a": "1.0.0", "b": "workspace:*", "c": "2.0.0"}, "expected": {"a": "1.0.0", "c": "2.0.0"}},
    {"name": "only the exact workspace star is a link", "dependencies": {"a": "workspace:^", "b": "workspace:*x"}, "expected": {"a": "workspace:^", "b": "workspace:*x"}},
    {"name": "nothing to drop", "dependencies": {"a": "1.0.0"}, "expected": {"a": "1.0.0"}},
    {"name": "no dependencies", "dependencies": {}, "expected": {}},
]
c["projections"] = [
    case("the pinned inputs are the recipe's identity and the lock without patch rows", recipe(), LOCK, "",
         '{"name":"@semio-tech/nx-bootstrap-tools","private":true,"packageManager":"bun@1.3.14","dependencies":{"nx":"23.2.0"}}\n',
         '{"lockfileVersion":1,"workspaces":{"":{"name":"x"}},"packages":{"nx":["nx@23.2.0","",{},"sha512-pinned"]}}\n'),
    case("a first-party workspace link is not an external pin", recipe(dependencies={"nx": "23.2.0", "@semio-tech/local": "workspace:*"}), LOCK, "",
         '{"name":"@semio-tech/nx-bootstrap-tools","private":true,"packageManager":"bun@1.3.14","dependencies":{"nx":"23.2.0"}}\n', None),
    case("dependency order is kept", recipe(dependencies={"z": "1.0.0", "a": "2.0.0", "m": "3.0.0"}), LOCK, "", None, None),
    case("a lock without patch rows is unchanged", recipe(), {k: v for k, v in LOCK.items() if k != "patchedDependencies"}, "", None,
         '{"lockfileVersion":1,"workspaces":{"":{"name":"x"}},"packages":{"nx":["nx@23.2.0","",{},"sha512-pinned"]}}\n'),
    case("unicode in the lock is written as is", recipe(), {**LOCK, "note": "é🧪"}, "", None, None),
    case("a recipe that is not private is refused", recipe(private=False), LOCK, CLOSED),
    case("a recipe without dependencies is refused", recipe(dependencies={}), LOCK, CLOSED),
    case("a range version is refused", recipe(dependencies={"nx": "^23.2.0"}), LOCK, CLOSED),
    case("a package manager other than bun is refused", recipe(packageManager="npm@10.0.0"), LOCK, CLOSED),
    case("an unknown recipe field is refused", {**recipe(), "scripts": {}}, LOCK, CLOSED),
    case("a recipe without the semio patches is refused", {k: v for k, v in recipe().items() if k != "semio"}, LOCK, CLOSED),
    case("a null lock is refused", recipe(), None, CLOSED),
    case("an array lock is refused", recipe(), [], CLOSED),
    case("a string lock is refused", recipe(), "lock", CLOSED),
    case("inputs above the byte ceiling are refused", recipe(), {"lockfileVersion": 1, "blob": "@16MiB"}, "Pinned dependency input byte ceiling exceeded"),
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)

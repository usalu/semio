"""📝 Authoring aid: writes the binding plan corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
DATA = "Canonical package binding data required"
OWNER = "Actual repository package owner required"
PORTABLE = "Canonical portable artifact source owner required"


def case(name, package, mode="prototype", error="", workspace="/repo", products="/generated/products", packageName="@semio-tech/fixture", posixOnly=False):
    return {"name": name, "workspace": workspace, "package": package, "packageName": packageName, "mode": mode, "products": products, "error": error, "posixOnly": posixOnly}


c = {"version": 1}
c["cases"] = [
    case("a package below the workspace in prototype mode", "/repo/owners/example/📦️packages/🟦️typescript"),
    case("the same package in release mode", "/repo/owners/example/📦️packages/🟦️typescript", "release"),
    case("a package one level below the workspace", "/repo/pkg"),
    case("a unicode package path", "/repo/é/🧪️x"),
    case("the workspace itself is not a package", "/repo", error=OWNER),
    case("a package outside the workspace is refused", "/foreign/📦️packages/🟦️typescript", error=OWNER),
    case("a sibling with the workspace name as prefix is outside", "/repository/pkg", error=OWNER),
    case("a package path that is not canonical is refused", "/repo/./pkg", error=DATA),
    case("a package path with a parent segment is refused", "/repo/a/../pkg", error=DATA),
    case("a trailing slash is not canonical", "/repo/pkg/", error=DATA),
    case("a relative package path is refused", "repo/pkg", error=DATA),
    case("a relative products directory is refused", "/repo/pkg", products="generated/products", error=DATA),
    case("a relative workspace is refused", "/repo/pkg", workspace="repo", error=DATA),
    case("an unknown mode is refused", "/repo/pkg", "debug", error=DATA),
    case("an empty package name is refused", "/repo/pkg", packageName="", error=DATA),
    case("a package name of 256 characters is kept", "/repo/pkg", packageName="n" * 256),
    case("a package name of 257 characters is refused", "/repo/pkg", packageName="n" * 257, error=DATA),
    case("an owner with a colon is not portable", "/repo/a:b", error=PORTABLE, posixOnly=True),
    case("an owner with a backslash is not portable", "/repo/a\\b", error=PORTABLE, posixOnly=True),
    case("an owner with a control character is not portable", "/repo/a\u0001b", error=PORTABLE, posixOnly=True),
]
c["receipts"] = [
    {"name": "a receipt of one mode", "value": {"version": 1, "protocol": "typescript-package-bindings", "package": "owners/example", "packageName": "@semio-tech/fixture", "activeMode": "prototype", "modes": ["prototype"]}, "valid": True},
    {"name": "a receipt of both modes", "value": {"version": 1, "protocol": "typescript-package-bindings", "package": "owners/example", "packageName": "@semio-tech/fixture", "activeMode": "release", "modes": ["prototype", "release"]}, "valid": True},
    {"name": "a repeated mode is refused", "value": {"version": 1, "protocol": "typescript-package-bindings", "package": "a", "packageName": "n", "activeMode": "release", "modes": ["release", "release"]}, "valid": False},
    {"name": "no mode is refused", "value": {"version": 1, "protocol": "typescript-package-bindings", "package": "a", "packageName": "n", "activeMode": "release", "modes": []}, "valid": False},
    {"name": "another protocol is refused", "value": {"version": 1, "protocol": "other", "package": "a", "packageName": "n", "activeMode": "release", "modes": ["release"]}, "valid": False},
    {"name": "an extra field is refused", "value": {"version": 1, "protocol": "typescript-package-bindings", "package": "a", "packageName": "n", "activeMode": "release", "modes": ["release"], "extra": 1}, "valid": False},
    {"name": "an owner of 257 characters is refused", "value": {"version": 1, "protocol": "typescript-package-bindings", "package": "a" * 257, "packageName": "n", "activeMode": "release", "modes": ["release"]}, "valid": False},
]
c["capacity"] = {"workspace": "/repo", "products": "/generated/products", "from": 150, "to": 260}
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)

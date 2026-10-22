"""📝 Authoring aid: writes the extra funded-workspace corpus (corpus.json) in the unit's own case shape."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def case(name, mode, directory, temporary, platform="posix", root=None, expected=True, error="", version=1):
    root = root or ("/owned" if platform == "posix" else "C:\\owned")
    return {"name": name, "request": {"version": version, "root": root, "mode": mode, "directory": directory, "temporary": temporary, "platform": platform}, "expected": expected, "error": error}


CLOSED = "Closed funded compiler workspace path required"
LAYOUT = "Canonical compiler layout root and policy required"
c = {"version": 1, "cases": [
    case("a unit directory directly below the units directory", "prototype", "/owned/b/p/units/u", "/owned/b/p/units/u.tmp"),
    case("a unit named like a parent is still a child", "release", "/owned/b/r/units/..sibling", "/owned/b/r/units/..sibling.tmp"),
    case("a dotted unit name", "prototype", "/owned/b/p/units/u.nit", "/owned/b/p/units/u.nit.tmp"),
    case("the units directory itself is not a unit", "prototype", "/owned/b/p/units", "/owned/b/p/units.tmp", expected=False),
    case("the parent of the units directory is not a unit", "prototype", "/owned/b/p", "/owned/b/p.tmp", expected=False),
    case("a sibling of the units directory is not a unit", "prototype", "/owned/b/p/other/u", "/owned/b/p/other/u.tmp", expected=False),
    case("the other mode's tree is not this mode's", "prototype", "/owned/b/r/units/u", "/owned/b/r/units/u.tmp", expected=False),
    case("a directory that only normalizes into the units directory is accepted", "prototype", "/owned/b/p/x/../units/u", "/owned/b/p/x/../units/u.tmp"),
    case("a temporary directory that is the unit itself is refused", "prototype", "/owned/b/p/units/u", "/owned/b/p/units/u", expected=False),
    case("a temporary directory beside another unit is refused", "prototype", "/owned/b/p/units/u", "/owned/b/p/units/v.tmp", expected=False),
    case("a temporary suffix with a different case is refused", "prototype", "/owned/b/p/units/u", "/owned/b/p/units/u.TMP", expected=False),
    case("windows ignores the case of the root", "prototype", "C:\\OWNED\\b\\p\\units\\u", "C:\\OWNED\\b\\p\\units\\u.tmp", "win32"),
    case("windows accepts forward slashes", "release", "C:/owned/b/r/units/u", "C:/owned/b/r/units/u.tmp", "win32"),
    case("windows on another drive is refused", "prototype", "D:\\owned\\b\\p\\units\\u", "D:\\owned\\b\\p\\units\\u.tmp", "win32", expected=False),
    case("windows nested units are refused", "prototype", "C:\\owned\\b\\p\\units\\u\\n", "C:\\owned\\b\\p\\units\\u\\n.tmp", "win32", expected=False),
    case("a relative root cannot make a layout", "prototype", "owned/b/p/units/u", "owned/b/p/units/u.tmp", root="owned", error=LAYOUT),
    case("another request version is not closed", "prototype", "/owned/b/p/units/u", "/owned/b/p/units/u.tmp", version=2, error=CLOSED),
    case("an unknown platform is not closed", "prototype", "/owned/b/p/units/u", "/owned/b/p/units/u.tmp", platform="darwin", error=CLOSED),
    case("an empty directory is not closed", "prototype", "", ".tmp", error=CLOSED),
    case("an unknown mode is not closed", "debug", "/owned/b/p/units/u", "/owned/b/p/units/u.tmp", error=CLOSED),
]}
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)

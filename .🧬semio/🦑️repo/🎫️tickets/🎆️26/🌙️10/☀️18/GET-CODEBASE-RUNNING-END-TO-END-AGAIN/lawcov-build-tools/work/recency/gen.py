"""📝 Authoring aid: writes the extra recency corpus (corpus.json) beside the unit's own cases."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def r(path, bytes_="1", a="0", m="0", c="0", device="1", inode="1"):
    return {"path": path, "device": device, "inode": inode, "bytes": bytes_, "atimeNs": a, "mtimeNs": m, "ctimeNs": c}


BOUND = "Bounded recency metadata required"
EXACT = "Exact bounded regular file facts required"
c = {"version": 1}
c["summaries"] = [
    {"name": "no files have no maximum", "rows": [], "error": ""},
    {"name": "rows are ordered by path whatever order they come in", "rows": [r("b", "2", "5", "5"), r("a", "3", "5", "5")], "error": ""},
    {"name": "the change time never counts as use", "rows": [r("a", "1", "1", "2", "999")], "error": ""},
    {"name": "at most eight sources are named", "rows": [r("f%02d" % i, "1", "7", "7") for i in range(10)], "error": ""},
    {"name": "nanosecond values beyond 64 bits keep exact precision", "rows": [r("a", "18446744073709551615", "99999999999999999999999999999999", "1"), r("b", "1", "99999999999999999999999999999998", "1")], "error": ""},
    {"name": "more than 4096 rows are refused", "rows": [], "generate": 4097, "error": BOUND},
    {"name": "exactly 4096 rows are summarized", "rows": [], "generate": 4096, "error": ""},
    {"name": "an empty path is refused", "rows": [r("")], "error": EXACT},
    {"name": "a path of 257 bytes is refused", "rows": [r("p" * 257)], "error": EXACT},
    {"name": "a path of 256 bytes is kept", "rows": [r("p" * 256)], "error": ""},
    {"name": "a multibyte path is measured in bytes", "rows": [r("é" * 129)], "error": EXACT},
    {"name": "a repeated path is refused", "rows": [r("a"), r("a", "2")], "error": EXACT},
    {"name": "a negative time is refused", "rows": [r("a", "1", "-1")], "error": EXACT},
    {"name": "a fractional byte count is refused", "rows": [r("a", "1.5")], "error": EXACT},
    {"name": "a 33 digit value is refused", "rows": [r("a", "1", "1" * 33)], "error": EXACT},
    {"name": "an empty value is refused", "rows": [r("a", "")], "error": EXACT},
    {"name": "a value with a leading plus is refused", "rows": [r("a", "+1")], "error": EXACT},
    {"name": "a value with a space is refused", "rows": [r("a", "1", " 1")], "error": EXACT},
    {"name": "a device that is not decimal is refused", "rows": [r("a", device="0x1")], "error": EXACT},
]
c["funding"] = [
    {"used": 0, "bytes": 1, "expected": True}, {"used": 1048575, "bytes": 1, "expected": True}, {"used": 1048576, "bytes": 1, "expected": False},
    {"used": 0, "bytes": 2048, "expected": True}, {"used": 0, "bytes": 2049, "expected": False}, {"used": 1046528, "bytes": 2048, "expected": True},
    {"used": 1046529, "bytes": 2048, "expected": False}, {"used": -1, "bytes": 1, "expected": False}, {"used": 0, "bytes": -1, "expected": False},
    {"used": 0.5, "bytes": 1, "expected": False}, {"used": 0, "bytes": 1.5, "expected": False}, {"used": 9007199254740992, "bytes": 1, "expected": False},
]
unit = lambda path, bytes_, recency, lock=False, source=None: {"path": path, "bytes": bytes_, "recencyMs": recency, "lockHeld": lock, **({"recencySource": source} if source else {})}
src = {**r("a/b", "4", "5", "1", "9"), "field": "atimeNs"}
c["describe"] = [
    {"name": "a vanished unit is reported as missing", "root": "/cache", "expected": unit("u", 10, 100), "current": None},
    {"name": "nothing changed except a lock", "root": "/cache", "expected": unit("u", 10, 100), "current": unit("u", 10, 100, True)},
    {"name": "bytes and recency changed", "root": "/cache", "expected": unit("u", 10, 100, False, src), "current": unit("u", 11, 200, False, src)},
    {"name": "the captured source is kept while the report is small", "root": "/cache", "expected": unit("u", 10, 100, False, src), "current": unit("u", 10, 100)},
    {"name": "an oversized report drops the sources", "root": "/cache", "expected": unit("u", 10, 100, False, {**src, "path": "s" * 5000}), "current": unit("u", 11, 100, False, {**src, "path": "t" * 5000})},
    {"name": "long names are cut to 256 characters", "root": "r" * 400, "expected": unit("u" * 400, 1, 1), "current": unit("u" * 400, 1, 1)},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)

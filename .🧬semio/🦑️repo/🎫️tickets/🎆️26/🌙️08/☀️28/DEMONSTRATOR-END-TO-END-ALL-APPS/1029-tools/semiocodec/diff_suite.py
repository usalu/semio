#!/usr/bin/env python3
"""usage: diff_suite.py <suite 🐍️.py> <dsl uri> <pack uri>   — deep-diffs parse_pack(pack) against parse_dsl(dsl) of one adapter."""
import importlib.util
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
from run_suite_stub import Context  # noqa: E402

suite = pathlib.Path(sys.argv[1]).resolve()
spec = importlib.util.spec_from_file_location("suite_under_test", suite)
loaded = importlib.util.module_from_spec(spec)
sys.modules["suite_under_test"] = loaded
spec.loader.exec_module(loaded)
ctx = Context(suite.parents[2], "diff")


def diff(a, b, path=""):
    if isinstance(a, dict) and isinstance(b, dict):
        for key in sorted(set(a) | set(b)):
            if key not in a:
                return "%s/%s only in dsl: %r" % (path, key, b[key])
            if key not in b:
                return "%s/%s only in pack: %r" % (path, key, a[key])
            found = diff(a[key], b[key], "%s/%s" % (path, key))
            if found:
                return found
        return None
    if isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            return "%s length pack %d vs dsl %d" % (path, len(a), len(b))
        for index, (x, y) in enumerate(zip(a, b)):
            found = diff(x, y, "%s[%d]" % (path, index))
            if found:
                return found
        return None
    if a != b or type(a) is not type(b) and not (isinstance(a, (int, float)) and isinstance(b, (int, float))):
        return "%s pack %r vs dsl %r" % (path, a, b)
    return None


parse_dsl = getattr(loaded, "parse_dsl")
document = parse_dsl(ctx.input_bytes(sys.argv[2]).decode("utf-8"))
print(diff(loaded.parse_pack(ctx.input_bytes(sys.argv[3])), document) or "equal")

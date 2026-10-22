"""📝 Authoring aid: writes the preflight matrix corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
CLOSED_R = "Closed build preflight requirements required"
CLOSED_F = "Closed build preflight facts required"
base = {"platform": "linux", "architecture": "x64", "availableBytes": 100, "tools": [], "exchange": "supported", "caseSensitive": "yes"}
req = lambda **k: {"architectures": ["x64"], "minimumAvailableBytes": 100, "tools": [], "exchange": False, **k}
c = {"version": 1}
c["requirements"] = [
    {"name": "linux", "policy": "linux"},
    {"name": "darwin", "policy": "darwin"},
    {"name": "win32", "policy": "win32"},
    {"name": "no exchange and no tools", "value": req()},
    {"name": "arm64 only with one tool and exchange", "value": req(architectures=["arm64"], minimumAvailableBytes=1, tools=["a"], exchange=True)},
]
c["axes"] = {"architecture": ["x64", "arm64", "ia32"], "byteOffsets": [-1, 0, 1], "toolPool": ["cc", "pkg-config", "hdiutil", "a", "extra"], "exchange": ["supported", "unsupported", "unprobed"], "caseSensitive": ["yes", "no", "unprobed"]}
c["invalidFacts"] = [
    {"name": "negative bytes", "patch": {"availableBytes": -1}},
    {"name": "fractional bytes", "patch": {"availableBytes": 1.5}},
    {"name": "unknown exchange", "patch": {"exchange": "maybe"}},
    {"name": "unknown case state", "patch": {"caseSensitive": "sometimes"}},
    {"name": "repeated tools", "patch": {"tools": ["cc", "cc"]}},
    {"name": "an extra field", "patch": {"extra": 1}},
    {"name": "a missing field", "drop": "platform"},
    {"name": "tools that are not strings", "patch": {"tools": [1]}},
]
c["invalidRequirements"] = [
    {"name": "no architectures", "requirement": req(architectures=[])},
    {"name": "an unknown architecture", "requirement": req(architectures=["ia32"])},
    {"name": "repeated architectures", "requirement": req(architectures=["x64", "x64"])},
    {"name": "a zero minimum", "requirement": req(minimumAvailableBytes=0)},
    {"name": "an upper-case tool", "requirement": req(tools=["CC"])},
    {"name": "a tool with a space", "requirement": req(tools=["a b"])},
    {"name": "a tool starting with a digit", "requirement": req(tools=["1a"])},
    {"name": "repeated tools", "requirement": req(tools=["a", "a"])},
    {"name": "an exchange that is not boolean", "requirement": req(exchange="yes")},
    {"name": "an unknown field", "requirement": {**req(), "skip": True}},
]
c["arguments"] = {"accepted": [["preflight"]], "rejected": [[], ["preflight", "--scope"], ["x"], ["preflight", "preflight"], ["Preflight"], ["preflight "]]}
c["base"] = base
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)

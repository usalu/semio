#!/usr/bin/env python3
"""🀄️ S4-INFRA: idempotent completion of the four WFC delegated native SQLite codecs (`🛬️native/🦀️.rs`).

Runs the general `🧪️s4-infra-sqlite-abi.py` migration, then the codec-local rules: the String-taking `error` helper
becomes the kind-preserving positioner of a `ValueError`, controlled codec calls keep their typed refusal,
`TextError` sources convert to their owned `ValueError` cause, and the module gains its own `ValueError`/`invalid`.
Usage: `python3 🧪️s4-infra-wfc-native-abi.py [--apply]`.
"""
import importlib.util
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SPEC = importlib.util.spec_from_file_location("abi", os.path.join(HERE, "🧪️s4-infra-sqlite-abi.py"))
ABI = importlib.util.module_from_spec(SPEC)
sys_argv, sys.argv = sys.argv, [sys.argv[0]]
SPEC.loader.exec_module(ABI)
sys.argv = sys_argv
APPLY = "--apply" in sys.argv
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts"
FILES = [
    f"{ROOT}/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🛬️native/🦀️.rs",
    f"{ROOT}/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🛬️native/🦀️.rs",
    f"{ROOT}/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🛬️native/🦀️.rs",
    f"{ROOT}/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛬️native/🦀️.rs",
]
ERROR = "fn error(error:ValueError)->TextError{TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}"
HEADER = "use semio_framework_value::ValueError;"
INVALID = "fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}"


def migrate(text):
    text = ABI.migrate(text)
    text = re.sub(r"fn error\(s:impl Into<String>\)->TextError\{[^\n]*?TextSpan::at\(1,1\)\)\}", ERROR, text)
    text = text.replace(".map_err(positioned)", ".map_err(error)")
    text = re.sub(r"(_controlled\((?:[^()]|\([^()]*\))*\))\.map_err\(\|e\|invalid\(e\.to_string\(\)\)\)", r"\1", text)
    text = text.replace(".map_err(|e|e.message)", ".map_err(|e|ValueError::new(e.kind,e.message))")
    lines = text.split("\n")
    anchor = max(index for index, line in enumerate(lines[:20]) if line.startswith("use "))
    if HEADER not in lines:
        lines.insert(anchor + 1, HEADER)
        anchor += 1
    if "invalid(" in text and INVALID not in lines:
        lines.insert(anchor + 1, INVALID)
    return "\n".join(lines)


for path in FILES:
    text = open(path, encoding="utf-8").read()
    after = migrate(text)
    status = "clean" if after == text else ("migrated" if APPLY else "pending")
    if APPLY and after != text:
        open(path, "w", encoding="utf-8").write(after)
    print(f"[s4-infra] {status} {os.path.relpath(path, ROOT)}")

#!/usr/bin/env python3
"""Where each flat staged file lives in the repository. NEW files are written whole; EDIT files are produced from the live text by build-staged.py."""
import os
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.environ.get("BZL_TESTS_ROOT", "/Users/ueli/Documents/semio")
F = "🧰️framework/🔨️modules/🏃️process"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
BZ = B + "/🔧️tools/🏗️bazel"
P = F + "/🧪️testing/🚦️protocol"
TC = BZ + "/🚦️test-classes"
W = TC + "/🧾️witness"
NEW = {
    "protocol.ts": P + "/🟦️.ts",
    "protocol.json": P + "/🔣️.json",
    "protocol-schema.json": P + "/🧬️schema/🔣️.json",
    "protocol-test.ts": P + "/🧪️tests/🟦️.ts",
    "protocol-fixture.json": P + "/🧪️tests/🧫️fixtures/🔣️.json",
    "protocol-oracle.py": P + "/🧪️tests/🐍️oracle/🐍️.py",
    "classes.ts": TC + "/🟦️.ts",
    "classes.json": TC + "/🔣️.json",
    "classes-schema.json": TC + "/🧬️schema/🔣️.json",
    "classes-test.ts": TC + "/🧪️tests/🟦️.ts",
    "classes-fixture.json": TC + "/🧪️tests/🧫️fixtures/🔣️.json",
    "classes-oracle.py": TC + "/🧪️tests/🐍️oracle/🐍️.py",
    "classes-measure.py": TC + "/📐️measure/🐍️.py",
    "witness.ts": W + "/🟦️.ts",
    "witness-test.ts": W + "/🧪️tests/🟦️.ts",
}
EDIT = {
    "script.ts": BZ + "/📜️script.ts",
    "schema.json": BZ + "/🧬️schema/🔣️.json",
    "commands.ts": BZ + "/🧭️commands/🟦️.ts",
    "commands-schema.json": BZ + "/🧭️commands/🧬️schema/🔣️.json",
    "commands-test.ts": BZ + "/🧭️commands/🧪️tests/🟦️.ts",
    "commands-fixture.json": BZ + "/🧭️commands/🧪️tests/🧫️fixtures/🔣️.json",
    "actions-fixture.json": BZ + "/🧪️tests/📥️actions/🔣️.json",
    "environment-policy.json": L + "/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🌍️environment/📋️policy/🔣️.json",
    "execution.ts": F + "/🧪️testing/🎛️execution/🟦️.ts",
    "capture.ts": F + "/🎛️owned-execution/🟦️.ts",
    "owned.ts": L + "/🏃️process/🎛️owned-execution/🟦️.ts",
}
def dest(flat):
    return ROOT + "/" + (NEW.get(flat) or EDIT[flat])

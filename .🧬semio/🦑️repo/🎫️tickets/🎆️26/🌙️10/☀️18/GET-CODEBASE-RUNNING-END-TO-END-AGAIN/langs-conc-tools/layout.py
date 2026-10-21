# 🗺️ Staged flat name -> live destination map and shared path constants for the langs-conc set. LC_ROOT redirects every path to an overlay (default: the real checkout).
import os
REAL = "/Users/ueli/Documents/semio"
ROOT = os.environ.get("LC_ROOT") or REAL
F = ROOT + "/🧰️framework"
PROC = F + "/🔨️modules/🏃️process"
PKG = PROC + "/📦️packages/🟦️typescript"
REPO = F + "/🛍️products/🦑️repo/🔨️modules"
L = REPO + "/📚️library"
B = L + "/🏗️build"
BZ = B + "/🔧️tools/🏗️bazel"
CACHING = L + "/⚡️caching"
BOOT = CACHING + "/🚀️bootstrap"
TOOLS = BOOT + "/🛠️tools"
T = PKG + "/🏗️build/🧰️toolchains"
N = TOOLS + "/🧰️native"
NEW = {
    "toolchains.ts": T + "/🟦️.ts",
    "toolchains-policy.json": T + "/🔣️.json",
    "toolchains-schema.json": T + "/🧬️schema/🔣️.json",
    "toolchains-test.ts": T + "/🧪️tests/🟦️.ts",
    "toolchains-fixture.json": T + "/🧪️tests/🔣️.json",
    "toolchains-fixture-schema.json": T + "/🧪️tests/🧬️schema/🔣️.json",
    "toolchains-oracle.py": T + "/🧪️tests/🐍️oracle/🐍️.py",
    "native.ts": N + "/🟦️.ts",
    "native-script.ts": N + "/📜️script.ts",
    "native-port.ts": N + "/🔌️port/🟦️.ts",
    "native-test.ts": N + "/🧪️tests/🟦️.ts",
    "bazel-native.ts": BZ + "/🧰️toolchains/🟦️.ts",
    "bazel-native-policy.json": BZ + "/🧰️toolchains/🔣️.json",
    "bazel-native-schema.json": BZ + "/🧰️toolchains/🧬️schema/🔣️.json",
    "bazel-native-test.ts": BZ + "/🧰️toolchains/🧪️tests/🟦️.ts",
    "bazel-native-fixture.json": BZ + "/🧰️toolchains/🧪️tests/🔣️.json",
    "global.json": ROOT + "/global.json",
    "Directory.Build.props": ROOT + "/Directory.Build.props",
    "pins-test.ts": L + "/🧪️tests/🧰️toolchain-pins/🟦️.ts",
}

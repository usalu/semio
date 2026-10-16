import os, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E, applylib, genmirror, fixtures

O = "🧰️framework/🛍️products/💻️os/🔨️modules/"
D = O + "📡️spr/🎮️command/"
ctx = applylib.Ctx("os-command", "semio_framework_os_command", D + "📦️packages/🦀️rust/Cargo.toml", "semio-framework-os-command")
R = E.ROOT
C = D + "🧪️tests/🧷️companion/"
PK = R + "/" + C + "📦️packages/🦀️rust"
keys = ["(root)@registry_fixture", "(root)@mutation_laws_fixture", "(root)", "(root)@mutation_payload_tests"]
assert set(keys) == set(ctx.mounts), set(ctx.mounts)
test_files = sorted({f for k in keys for f in ctx.subtree(k)})
if "--p" in sys.argv:
    ctx.remove_mounts()
    pc = D + "📦️packages/🦀️rust/Cargo.toml"
    t = E.read(pc)
    i = t.index("\n[dev-dependencies]\n")
    E.edit(pc, t[i:], "\n" if False else "")
    log = []
    n = fixtures.convert(test_files, PK, log=log)
    print("fixture reads converted:", n, [(os.path.basename(os.path.dirname(f)), c) for f, c in log])
dsl = 'dsl_derive = { path = "../../../../../../🗣️dsl/✨️derive/📦️packages/🦀️rust", package = "semio-framework-os-kernel-dsl-derive" }'
if "--show" in sys.argv:
    print(ctx.render_root(R + "/" + C, keys, "Companion tests of os-command: the command protocol unit tests, the mutation payload law and the registry and mutation-law fixtures.", "🧷️"))
if "--create" in sys.argv:
    print(ctx.companion(C, keys, "semio-framework-os-command-tests", "os-command-tests", "@semio-tech/os-command-tests-rs", "Companion tests of os-command: command protocol unit tests, the mutation payload law and the registry and mutation-law fixtures", "Companion tests of os-command: the command protocol unit tests, the mutation payload law and the registry and mutation-law fixtures.", "🧷️", p_features=("mutation-testing",), extra_deps=(dsl,)))

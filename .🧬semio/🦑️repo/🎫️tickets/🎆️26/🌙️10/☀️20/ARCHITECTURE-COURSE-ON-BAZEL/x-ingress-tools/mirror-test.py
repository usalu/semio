import os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = sys.argv[1]
MIRROR = os.path.join(SCRATCH, "mirror")
TEST = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🚪️ingress/🟦️.ts"
MODULES = "🧰️framework/🔨️modules"
apply_py = os.path.join(HERE, "apply.py")
extra = []
for _ in range(40):
    text = open(apply_py, encoding="utf-8").read()
    for module in extra:
        if module not in text:
            text = text.replace("FULL = [LIBRARY,", 'FULL = [LIBRARY, "%s",' % module, 1)
    open(apply_py, "w", encoding="utf-8").write(text)
    extra = []
    subprocess.run([sys.executable, apply_py, "mirror", MIRROR], capture_output=True)
    env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=SCRATCH)
    run = subprocess.run(["bun", "test", "./" + TEST] + sys.argv[2:], cwd=MIRROR, env=env, capture_output=True, text=True)
    output = run.stdout + run.stderr
    match = re.search(r"Cannot find module '([^']+)' from '([^']+)'", output)
    if not match:
        print(output[-6000:])
        break
    target = os.path.normpath(os.path.join(os.path.dirname(match.group(2)), match.group(1)))
    relative = os.path.relpath(target, MIRROR)
    parts = relative.split(os.sep)
    if relative.startswith(MODULES) and len(parts) > 2:
        extra.append("/".join(parts[:3]))
        print("add", extra[-1])
    else:
        print("unresolved", relative)
        print(output[-3000:])
        break

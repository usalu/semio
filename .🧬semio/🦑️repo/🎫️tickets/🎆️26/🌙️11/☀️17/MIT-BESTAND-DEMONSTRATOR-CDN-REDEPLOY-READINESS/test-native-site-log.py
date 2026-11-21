"""📦️ Check publication of Vite captures through concurrent shell invocations."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ticket = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as work:
    root = Path(work)
    wrapper = root / "a/b/c/d/e/f/g"
    wrapper.mkdir(parents=True)
    for name in ["native-wasm.sh", "captured-site.py"]:
        (wrapper / name).symlink_to(ticket / name)
    (root / "♻️mit-bestand/🧺️demonstrator").mkdir(parents=True)
    tools = root / "bin"
    tools.mkdir()
    (tools / "bun").write_text("""#!/bin/sh
case "$1" in
 *native-wire.ts) printf '%s\\n' wire providers ;;
 *native-receipt.mjs) if [ "$2" = issue ]; then echo '{"TEST_RECEIPT":"issued"}'; fi ;;
esac
""")
    (tools / "repo").write_text("""#!/bin/sh
touch "$ENTERED"
while [ ! -f "$PROCEED" ]; do sleep 0.05; done
echo "$EVENT"
exit "$STATUS"
""")
    for tool in tools.iterdir():
        tool.chmod(0o755)
    generated = wrapper / "🗑️generated"
    generated.mkdir()
    last = generated / "native-site.log"
    last.write_text("previous")
    def start(label, status):
        output = root / ".cache/b/p/m/vite/release" / label / "out"
        output.mkdir(parents=True)
        (output / "index.html").touch()
        event = "[script] " + json.dumps({"command": "vite:entry", "activity": {"resource": f"artifact-capture:{output}"}})
        env = dict(os.environ, PATH=str(tools) + os.pathsep + os.environ["PATH"], BUN=str(tools / "bun"), EVENT=event,
                   ENTERED=str(root / (label + ".entered")), PROCEED=str(root / label), STATUS=str(status),
                   NATIVE_SITE_RUN_LOG=str(generated / label))
        return subprocess.Popen(["bash", str(wrapper / "native-wasm.sh"), "site"], env=env, stdout=subprocess.PIPE), event
    import time
    slow, first = start("first", 0)
    fast, second = start("second", 0)
    failed, _ = start("failed", 1)
    for _ in range(200):
        if all((root / (x + ".entered")).exists() for x in ["first", "second", "failed"]):
            break
        time.sleep(0.05)
    else:
        raise RuntimeError("missing site processes")
    assert last.read_text() == "previous"
    for label, proc, expected, code in [("second", fast, second, 0), ("failed", failed, second, 1), ("first", slow, first, 0)]:
        (root / label).touch()
        assert proc.communicate(timeout=15) is not None and proc.returncode == code
        assert last.read_text().strip() == expected
    assert (generated / "second").read_text().strip() == second
print("Concurrent Vite capture publication and failed-run preservation passed.")

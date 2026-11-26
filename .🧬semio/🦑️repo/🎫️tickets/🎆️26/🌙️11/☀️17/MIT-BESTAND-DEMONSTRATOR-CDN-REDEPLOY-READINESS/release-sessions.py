"""🎮️ Publish the actual Demonstrator session variants and verify their catalog projections."""
from concurrent.futures import ThreadPoolExecutor
from hashlib import sha256
import json
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
runtime = root / "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime"
generated = ticket / "🗑️generated"
catalog = root / "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"
registry = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"
staged = root / ".cache/b/p/m/browser/release/sessions"
projection = {name: json.loads((catalog / name).read_bytes()) for name in ("🔌️plugins.json", "🎠️playgrounds.json")}
variants = sorted({json.loads((runtime / "🧫️pipeline.json").read_bytes())["primaryVariant"],
                   *(row["runtimeVariant"] for row in json.loads((runtime / "🔣️.json").read_bytes())["panes"])})


def expected_files():
    command = ["repo", "tools", "exec", "--light", "bun", "--", str(ticket / "release-session-digests.ts"), str(catalog), *variants]
    return json.loads(subprocess.check_output(command, cwd=root))


expected = expected_files()


def matches(variant):
    try:
        return all(sha256((staged / variant / name).read_bytes()).hexdigest() == digest for name, digest in expected[variant].items())
    except (OSError, ValueError):
        return False


def publish(variant):
    if matches(variant):
        return
    if "--check" in sys.argv:
        raise ValueError(f"Stale or missing session {variant}")
    env = dict(os.environ, SEMIO_BUILD_MODE="release", SEMIO_WORKSPACE_ROOT=str(root), SEMIO_PLUGIN_CATALOG=str(catalog))
    with (generated / f"session-{variant}.log").open("w") as stream:
        subprocess.run(["repo", str(ticket / "native-launch.ts"), registry, "repo", "./📜️script.ts", "session", variant],
                       cwd=root, env=env, stdout=stream, stderr=subprocess.STDOUT, check=True)
    if not matches(variant):
        raise ValueError(f"Session projection changed during publication of {variant}")


with ThreadPoolExecutor(max_workers=2) as pool:
    list(pool.map(publish, variants))
for name, rows in projection.items():
    if json.loads((catalog / name).read_bytes()) != rows:
        raise ValueError(f"Catalog changed during session publication: {name}")
print(f"Verified {len(variants)} release session projections: {', '.join(variants)}")

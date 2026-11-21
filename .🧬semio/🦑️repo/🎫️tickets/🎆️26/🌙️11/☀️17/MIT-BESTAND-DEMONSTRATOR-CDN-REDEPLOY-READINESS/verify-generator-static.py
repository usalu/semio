"""🌐️ Verify both Generator locales against one captured Vite release directory."""

import argparse
import os
from pathlib import Path
import subprocess
import sys
import time
from urllib.parse import quote
from urllib.request import urlopen


def main():
    """🧾️ Attest the exact staged core on its HTTP route before running Playwright."""
    parser = argparse.ArgumentParser()
    parser.add_argument("build_log", type=Path)
    parser.add_argument("core_sha")
    parser.add_argument("--port", type=int, default=8820)
    parser.add_argument("--label", choices=["optimized", "after-retirement"], default="optimized")
    args = parser.parse_args()
    ticket = Path(__file__).resolve().parent
    root = ticket.parents[6]
    generated = ticket / "🗑️generated"
    demo = root / "♻️mit-bestand" / "🧺️demonstrator"
    node = ["repo", "tools", "exec", "--light", "node", "--"]
    capture = subprocess.check_output([sys.executable, ticket / "captured-site.py", root, args.build_log], text=True)
    site = Path(capture.strip())
    asset = "🔌️plugin-modules/🌀️procedural/semio_hub_procedural_component.core.wasm"
    origin = f"http://127.0.0.1:{args.port}"
    result = f"generator-static-{args.label}"
    with (generated / f"{result}-server.log").open("w") as log:
        server = subprocess.Popen([sys.executable, ticket / "static-origin.py", site, str(args.port)], stdout=log, stderr=subprocess.STDOUT)
        try:
            for _ in range(60):
                if server.poll() is not None:
                    raise RuntimeError(f"Generator static server exited {server.returncode}")
                try:
                    with urlopen(origin + "/", timeout=1) as response:
                        if response.status == 200:
                            break
                except OSError:
                    time.sleep(0.5)
            else:
                raise TimeoutError("Generator static server did not open")
            digest = subprocess.check_output([*node, ticket / "probe-custody.mjs", "verify", site / asset, f"{origin}/{quote(asset)}"], text=True)
            print(digest.strip(), flush=True)
            if digest.split()[0] != args.core_sha:
                raise RuntimeError(f"Expected the static site to serve core {args.core_sha}")
            locator = 'const path=require("node:path"); console.log(path.join(path.dirname(require.resolve("@playwright/test/package.json")),"cli.js"))'
            cli = subprocess.check_output([*node, "-e", locator], cwd=demo, text=True).strip()
            env = dict(
                os.environ,
                DEMONSTRATOR_VERIFY_CDN="1",
                PLAYWRIGHT_BASE_URL=f"http://cdn.localhost:{args.port}/",
                PLAYWRIGHT_BROWSERS_PATH=str(root / ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"),
                PLAYWRIGHT_JSON_OUTPUT_FILE=str(generated / f"{result}.json"),
            )
            command = [
                "repo", "capacity", "run", "--memory=4096", "--jobs=1", "--", *node,
                cli, "test", "--config", "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts",
                "--workers=1", "--project", "en", "--project", "de", "--grep", "generator boot$",
                "--output", generated / f"{result}-pw", "--reporter=list,json",
            ]
            return subprocess.run(command, cwd=demo, env=env, check=False).returncode
        finally:
            server.terminate()
            server.wait(timeout=5)


if __name__ == "__main__":
    raise SystemExit(main())

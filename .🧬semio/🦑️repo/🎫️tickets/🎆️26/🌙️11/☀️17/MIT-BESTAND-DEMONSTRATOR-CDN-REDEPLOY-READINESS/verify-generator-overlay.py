"""🌐️ Verify the Generator boot against exact candidate Procedural browser assets."""
import os
from pathlib import Path
import socket
import subprocess
import sys
import time
from urllib.parse import quote
from urllib.request import urlopen

def main():
    """🧪️ Serve the candidate module over the unchanged static renderer and run both Generator cases in both locales."""
    site, module = (Path(item).resolve(strict=True) for item in sys.argv[1:3])
    port = int(sys.argv[3]) if len(sys.argv) > 3 else 8799
    ticket = Path(__file__).resolve().parent
    root = ticket.parents[6]
    generated = ticket / "🗑️generated"
    demo = root / "♻️mit-bestand" / "🧺️demonstrator"
    node = ["repo", "tools", "exec", "--light", "node", "--"]
    origin = f"http://127.0.0.1:{port}"
    with socket.socket() as probe:
        if probe.connect_ex(("127.0.0.1", port)) == 0:
            raise RuntimeError(f"Generator overlay port {port} is already in use")
    probe_dir = generated / "generator-overlay-empty"
    probe_dir.mkdir(parents=True, exist_ok=True)
    folder = "🌀️procedural"
    prefix = f"🔌️plugin-modules/{folder}/"
    server_env = dict(os.environ, SEMIO_OVERLAY_EXACT_MODULE="1")
    with (generated / "generator-overlay-origin.log").open("w") as log:
        server = subprocess.Popen([*node, ticket / "overlay-serve.mjs", site, probe_dir, str(port), folder, module], stdout=log, stderr=subprocess.STDOUT, env=server_env)
        try:
            for _ in range(60):
                if server.poll() is not None:
                    raise RuntimeError(f"Generator overlay server exited {server.returncode}")
                try:
                    with urlopen(origin + "/", timeout=1):
                        break
                except OSError:
                    time.sleep(0.2)
            else:
                raise TimeoutError("Generator overlay server did not open")
            assets = ["🔣️.json", "🌉️bridge.js", "🟨️.js", "semio_hub_procedural_component.js"]
            assets += sorted(asset.name for asset in module.glob("*.wasm"))
            if not any(name.endswith(".core.wasm") for name in assets):
                raise RuntimeError("Generator core missing")
            for name in assets:
                subprocess.run([*node, ticket / "probe-custody.mjs", "verify", module / name, f"{origin}/{quote(prefix + name)}"], check=True)
            locate = 'const p=require("node:path"); console.log(p.join(p.dirname(require.resolve("@playwright/test/package.json")),"cli.js"))'
            cli = subprocess.check_output([*node, "-e", locate], cwd=demo, text=True).strip()
            env = dict(
                os.environ,
                DEMONSTRATOR_VERIFY_CDN="1",
                PLAYWRIGHT_BASE_URL=f"http://cdn.localhost:{port}/",
                PLAYWRIGHT_BROWSERS_PATH=str(root / ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"),
                PLAYWRIGHT_JSON_OUTPUT_FILE=str(generated / "generator-overlay.json"),
            )
            command = [
                "repo", "capacity", "run", "--memory=4096", "--jobs=1", "--", *node, cli,
                "test", "--config", "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts", "--workers=1",
                "--project", "en", "--project", "de", "--grep", "generator (boot|undo-redo)$",
                "--output", generated / "generator-overlay-pw", "--reporter=list,json",
            ]
            return subprocess.run(command, cwd=demo, env=env, check=False).returncode
        finally:
            server.terminate()
            server.wait(timeout=8)

if __name__ == "__main__":
    raise SystemExit(main())

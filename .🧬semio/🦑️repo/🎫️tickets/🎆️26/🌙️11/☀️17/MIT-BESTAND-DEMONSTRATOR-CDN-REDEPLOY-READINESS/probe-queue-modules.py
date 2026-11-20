"""🌐️ Run the affected browser boot cases when a new component passes the queue."""
import hashlib
import os
from pathlib import Path
import subprocess
import time

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
G = TICKET / "🗑️generated"
DEMO = ROOT / "♻️mit-bestand/🧺️demonstrator"
PANES = {
    "energy": "energie",
    "procedural": "generator",
    "demonstrator": "koordinator|aggregator|aussuchen|bearbeiten|verfolgen",
    "fem": "statik",
}


def main():
    cli = subprocess.check_output(
        ["node", "-p", 'require.resolve("@playwright/test/package.json")'], cwd=DEMO, text=True
    ).strip()
    environment = dict(os.environ, PLAYWRIGHT_BASE_URL="http://cdn.localhost:8767/",
                       DEMONSTRATOR_VERIFY_CDN="1",
                       PLAYWRIGHT_BROWSERS_PATH=str(ROOT / ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"))
    observations = {}
    while True:
        for module, panes in PANES.items():
            witness = G / "confirmed" / (module + ".json")
            if not witness.exists():
                continue
            identity = hashlib.sha256(witness.read_bytes()).hexdigest()
            if observations.get(module, (None,))[0] == identity:
                continue
            path = G / f"current-boot-{module}.log"
            print(f"probing {module} {identity[:12]}", flush=True)
            with path.open("w") as log:
                result = subprocess.run(
                    ["node", str(Path(cli).with_name("cli.js")), "test",
                     "--config", "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts", "--workers=1",
                     "--grep", f"({panes}) boot$", "--output", str(G / f"pw-current-{module}"),
                     "--reporter=list"], cwd=DEMO, env=environment, stdout=log, stderr=subprocess.STDOUT)
            observations[module] = (identity, result.returncode)
            print(f"{module} browser exit {result.returncode} {path}", flush=True)
        if len(observations) == len(PANES) and all(status == 0 for _, status in observations.values()):
            return
        time.sleep(5)


if __name__ == "__main__":
    main()

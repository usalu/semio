"""🔁 Exercise the runner and report after a good result is superseded by a timeout."""

from datetime import datetime, timezone
from pathlib import Path
from subprocess import Popen
from urllib.error import HTTPError
from urllib.request import Request, urlopen
from uuid import uuid4
import json
import os
import runpy
import time

ticket = Path(__file__).resolve().parent
base = 5810
generated = ticket / "🗑️generated" / f"matrix-{base}"
generated.mkdir(parents=True, exist_ok=True)
env = {**os.environ, "SEMIO_MATRIX_CONTROL_PORT": str(base)}
control = f"http://127.0.0.1:{base}"
results = f"http://127.0.0.1:{base+4}/__matrix_result"
app = {"app": "cad", "locale": "de", "width": 375, "height": 900}
now = datetime.now(timezone.utc).isoformat()
reporter = runpy.run_path(str(ticket / "matrix-report.py"))


def post(url, body):
    """📮 Return the HTTP status of one recorded test action."""
    req = Request(url, method="POST", data=json.dumps(body).encode(), headers={"content-type": "application/json"})
    try:
        with urlopen(req, timeout=5) as reply:
            return reply.status
    except HTTPError as error:
        return error.code


def wait_for_runner(child):
    """🕒 Wait for the isolated loopback runner to bind its HTTP routes."""
    for _ in range(60):
        if child.poll() is not None:
            raise RuntimeError("Isolated runner exited")
        try:
            with urlopen(control + "/previews", timeout=1) as reply:
                if reply.status == 200:
                    return
        except OSError:
            time.sleep(.25)
    raise RuntimeError("Isolated runner did not start")


def main():
    """🧪 Check accepted results, superseding attempts and stale-write rejection."""
    with (generated / "attempt-smoke.log").open("w", encoding="utf8") as log:
        child = Popen(["repo", "tools", "exec", "--light", "node", "--", str(ticket / "runner-server.cjs")],
                      cwd=ticket, env=env, stdout=log, stderr=log)
    try:
        wait_for_runner(child)
        first = {**app, "attempt": str(uuid4())}
        second = {**app, "attempt": str(uuid4())}
        result = {**first, "observedLocale": "de", "actualWidth": 375, "actualHeight": 900, "at": now,
                  "targets": 1, "focusChecks": 1, "failures": 0, "stories": [{"name": "current page", "targets": 1, "failing": 0}],
                  "errors": [], "report": []}
        read_cell = reporter["read_cell"]
        assert post(control + "/attempt", {**first, "state": "pending"}) == 200
        assert post(results, result) == 200
        assert read_cell(generated, "cad", "de", 375)[0] == "Pass"
        assert post(control + "/attempt", {**first, "state": "timeout"}) == 409
        assert post(control + "/attempt", {**second, "state": "pending"}) == 200
        assert read_cell(generated, "cad", "de", 375)[0] == "Pending"
        assert post(results, result) == 409
        assert post(control + "/attempt", {**second, "state": "timeout", "message": "Readiness expired"}) == 200
        status, _, reasons = read_cell(generated, "cad", "de", 375)
        assert (status, reasons) == ("Error", ["Readiness expired"])
        assert post(results, {**result, "attempt": second["attempt"]}) == 409
    finally:
        child.terminate()
        child.wait(timeout=10)


if __name__ == "__main__":
    main()

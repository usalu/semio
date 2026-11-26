"""🧪 Exercise Marketplace in the same accepted origin and manifest window."""
from http.client import HTTPConnection
import json
from pathlib import Path
import runpy
from subprocess import CalledProcessError, CompletedProcess
from tempfile import TemporaryDirectory
from unittest.mock import patch
from urllib.parse import urlsplit

ticket = Path(__file__).resolve().parent
release = runpy.run_path(str(ticket / "accept-site.py"))
gates = runpy.run_path(str(ticket / "verify-boot-matrix.py"))
primary = json.loads(gates["CASES"].read_bytes())
extra = json.loads(gates["BEHAVIOUR_CASES"].read_bytes())


def report(cases, discoveries):
    return {"errors": [], "config": {"projects": primary["projects"]},
            "suites": [{"specs": [{"title": title, "tests": [
                {"projectName": project["name"], "expectedStatus": "passed",
                 "results": [] if discoveries else [{"status": "passed"}]}
                for project in primary["projects"]]} for title in cases["titles"]]}]}


with TemporaryDirectory(dir=release["GENERATED"], prefix="mc-") as directory:
    root = Path(directory)
    site = root / "site"
    site.mkdir()
    (site / "index.html").write_text("<title>Marketplace fixture</title>", encoding="utf-8")
    generated = root / "run"
    events = []

    def launch(args, *, env, **options):
        address = env["PLAYWRIGHT_BASE_URL"]
        if "--list" in args:
            phase = "discovery"
        elif str(ticket / "probe-cdn-marketplace.cjs") in args:
            assert args[-1] == address
            assert options["stdout"].name.endswith("/marketplace.log")
            assert "PLAYWRIGHT_JSON_OUTPUT_FILE" not in env
            phase = "marketplace"
        else:
            phase = "suite"
        events.append((phase, address))
        if phase != "marketplace":
            cases = extra if any("🧭️behaviours" in arg for arg in args) else primary
            Path(env["PLAYWRIGHT_JSON_OUTPUT_FILE"]).write_text(
                json.dumps(report(cases, phase == "discovery")), encoding="utf-8")
        if phase != "discovery":
            url = urlsplit(address)
            connection = HTTPConnection("127.0.0.1", url.port, timeout=5)
            try:
                connection.request("GET", "/", headers={"Host":url.netloc})
                response = connection.getresponse()
                assert response.status == 200
                response.read()
            finally:
                connection.close()
        return CompletedProcess(args, 9 if phase == "marketplace" and refuse else 0)

    verify = lambda run: release["verify_matrix"](site, run, 0, browser_memory=512)
    for refuse in (False, True):
        events.clear()
        with patch("subprocess.check_output", return_value="fixture-playwright"), patch("subprocess.run", side_effect=launch):
            try:
                release["accept"](site, release["ROOT"], generated, verify)
            except CalledProcessError as error:
                assert refuse and error.returncode == 9
            else:
                assert not refuse
        assert (generated / "static-accepted.sha256").is_file() != refuse
        assert [phase for phase, _ in events] == ["discovery","discovery","suite","suite","marketplace"]
        assert len({origin for _, origin in events}) == 1
    print("Shared static origin and exact 96 synthetic report cases; Marketplace refusal prevents the accepted manifest.")

"""🌐 Verify the Demonstrator acceptance suite against one unchanged CDN site."""
import argparse
from functools import partial
from hashlib import sha256
from html.parser import HTMLParser
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys
from tempfile import mkdtemp
from threading import Thread
from urllib.parse import unquote, urlsplit


TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if (parent / "♻️mit-bestand/🧺️demonstrator/📋️project.json").is_file())
GENERATED = TICKET / "🗑️generated"
DEMONSTRATOR = ROOT / "♻️mit-bestand/🧺️demonstrator"


class SiteEntry(HTMLParser):
    def __init__(self, path):
        super().__init__()
        self.title = ""
        self.reading_title = False
        self.mount = False
        self.modules = []
        self.feed(path.read_text(encoding="utf-8"))
        self.close()

    def handle_starttag(self, tag, attrs):
        attributes = dict(attrs)
        if tag == "title":
            self.reading_title = True
        if attributes.get("id") == "root":
            self.mount = True
        if tag == "script" and attributes.get("type") == "module" and attributes.get("src"):
            self.modules.append(attributes["src"])

    def handle_endtag(self, tag):
        if tag == "title":
            self.reading_title = False

    def handle_data(self, data):
        if self.reading_title:
            self.title += data


def verify_site_entry(site, project=DEMONSTRATOR):
    catalog = json.loads((project / "🔨️modules/🧩️runtime/🔣️.json").read_bytes())
    host = catalog["host"]
    cname = (site / "CNAME").read_text(encoding="utf-8").splitlines()
    if cname != [host]:
        raise ValueError(f"Expected CNAME {host}, found {cname}")
    if (site / ".nojekyll").read_bytes() != b"":
        raise ValueError("Expected an empty .nojekyll marker")
    authored = SiteEntry(project / "🌐️.html")
    served = SiteEntry(site / "index.html")
    if not authored.title.strip() or served.title.strip() != authored.title.strip() or not (authored.mount and served.mount):
        raise ValueError("Built index does not match the authored Demonstrator title and mount")
    for source in served.modules:
        url = urlsplit(source)
        local = (site / unquote(url.path).lstrip("/")).resolve()
        if not (url.scheme or url.netloc) and local.is_relative_to(site) and local.suffix in (".js", ".mjs") and local.is_file():
            print(f"Verified {host}, .nojekyll and the Demonstrator index", flush=True)
            return
    raise ValueError("Built index has no existing local module bootstrap")


def acceptance_snapshot(project):
    files = [project / "🪧️brand.ts"]
    for directory in ("🧪️tests/🎭️acceptance", "🔨️modules/🧪️e2e"):
        files.extend(path for path in (project / directory).rglob("*") if path.is_file())
    return [(path.relative_to(project), sha256(path.read_bytes()).digest()) for path in sorted(files)]


def accept(site, workspace, generated, verify):
    generated.mkdir(parents=True, exist_ok=True)
    lock = generated / "static-verification.lock"
    lock.mkdir()
    try:
        accepted = generated / "static-accepted.sha256"
        accepted.unlink(missing_ok=True)
        run = Path(mkdtemp(dir=generated, prefix="cdn-"))
        manifest = runpy.run_path(str(TICKET / "site-manifest.py"))["write_manifest"]
        before, after = run / "before.sha256", run / "after.sha256"
        _, size = manifest(site, workspace, before)
        check_pages_bytes = runpy.run_path(str(TICKET / "package-site.py"))["check_pages_bytes"]
        check_pages_bytes(size + before.stat().st_size)
        verify(run)
        manifest(site, workspace, after)
        if before.read_bytes() != after.read_bytes():
            raise ValueError("Site changed during CDN verification")
        after.replace(accepted)
    finally:
        lock.rmdir()


def verify_matrix(site, run, port, browser_memory=4096):
    if not (site / "index.html").is_file():
        raise FileNotFoundError(site / "index.html")
    sources = acceptance_snapshot(DEMONSTRATOR)
    marketplace = TICKET / "probe-cdn-marketplace.cjs"
    marketplace_digest = sha256(marketplace.read_bytes()).digest()
    static = runpy.run_path(str(TICKET / "static-origin.py"))
    origin_log = run / "origin.log"
    baseline_discovery = run / "baseline-roster.json"
    behaviour_discovery = run / "behaviour-roster.json"
    baseline_report = run / "playwright.json"
    behaviour_report = run / "behaviours.json"
    gates = runpy.run_path(str(TICKET / "verify-boot-matrix.py"))
    with origin_log.open("w", encoding="utf-8") as log, static["StaticOrigin"](
        ("127.0.0.1", port),
        partial(static["StaticAsset"], directory=str(site), access_logger=static["access_logger"](log)),
    ) as server:
        thread = Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            address = f"http://cdn.localhost:{server.server_port}/"
            print(f"Verifying {site} at {address}\nMatrix evidence: {run}", flush=True)
            env = dict(os.environ, DEMONSTRATOR_VERIFY_CDN="1", PLAYWRIGHT_BASE_URL=address)
            env.setdefault("PLAYWRIGHT_BROWSERS_PATH", str(ROOT / ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"))
            cli = subprocess.check_output(
                ["repo", "tools", "exec", "--light", "node", "--", "-p", 'require.resolve("@playwright/test/cli")'],
                cwd=DEMONSTRATOR,
                encoding="utf-8",
            ).strip()
            command = ["repo", "tools", "exec", "--light", "node", "--", cli, "test", "--config"]
            baseline = [*command, "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts"]
            behaviours = [*command, "./🧪️tests/🎭️acceptance/🧭️behaviours/🎚️config/🟦️.ts"]
            for suite, discovery in [(baseline, baseline_discovery), (behaviours, behaviour_discovery)]:
                subprocess.run(
                    [*suite, "--list", "--reporter=json"], cwd=DEMONSTRATOR,
                    env=dict(env, PLAYWRIGHT_JSON_OUTPUT_FILE=str(discovery)), check=True,
                )
            rosters = gates["release_rosters"](json.loads(baseline_discovery.read_bytes()), json.loads(behaviour_discovery.read_bytes()))
            executions = []
            for suite, result, folder in [(baseline, baseline_report, "pw"), (behaviours, behaviour_report, "bh")]:
                executions.append(subprocess.run(
                    ["repo", "capacity", "run", f"--memory={browser_memory}", "--", *suite,
                     "--retries=0", "--output", str(run / folder), "--reporter=list,json"],
                    cwd=DEMONSTRATOR,
                    env=dict(env, PLAYWRIGHT_JSON_OUTPUT_FILE=str(result)),
                ))
            with (run / "marketplace.log").open("w", encoding="utf-8") as output:
                executions.append(subprocess.run(
                    ["repo", "capacity", "run", f"--memory={min(browser_memory, 2048)}", "--",
                     "repo", "tools", "exec", "--light", "node", "--", str(marketplace), address],
                    cwd=DEMONSTRATOR, env=env, stdout=output, stderr=subprocess.STDOUT,
                ))
        finally:
            server.shutdown()
            thread.join()
    for execution in executions:
        execution.check_returncode()
    gates["verify_release"](rosters, json.loads(baseline_report.read_bytes()), json.loads(behaviour_report.read_bytes()),
                            origin_log.read_text(encoding="utf-8"))
    if acceptance_snapshot(DEMONSTRATOR) != sources or sha256(marketplace.read_bytes()).digest() != marketplace_digest:
        raise ValueError("Acceptance sources changed during the browser run")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("site", type=Path, nargs="?")
    parser.add_argument("--port", type=int, default=0)
    args = parser.parse_args()
    if args.site is None:
        capture = runpy.run_path(str(TICKET / "captured-site.py"))["captured_site"]
        with (GENERATED / "native-site.log").open(encoding="utf-8", errors="replace") as log:
            site = capture(ROOT, log)
    else:
        site = args.site.resolve()
    receipts = [sys.executable, str(TICKET / "release-receipts.py"), "verify",
                str(ROOT / ".cache/b/p/m/browser/release/modules"), str(GENERATED / "confirmed"),
                "--site", str(site)]
    def verify(run):
        verify_site_entry(site)
        subprocess.run(receipts, check=True)
        with (run / "actor-activation.log").open("w", encoding="utf-8") as log:
            subprocess.run(["repo", "capacity", "run", "--memory=2048", "--", "repo", "tools", "exec",
                            "--light", "node", "--", str(TICKET / "probe-release-actors.cjs"), str(site)],
                           cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=True)
        with (run / "flow-operators.log").open("w", encoding="utf-8") as log:
            subprocess.run(["repo", "capacity", "run", "--memory=2048", "--jobs=1", "--", "repo", "tools", "exec",
                            "--light", "bun", "--", str(TICKET / "probe-flow-addon-cases.ts"), str(site)],
                           cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=True)
        verify_matrix(site, run, args.port)
        verify_site_entry(site)
        subprocess.run(receipts, check=True)

    accept(site, ROOT, GENERATED, verify)
    print(f"ACCEPTED {site}")


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        raise SystemExit(str(error))

"""🔭 Probe newly staged release guests through private compact overlays."""
import argparse
from functools import partial
import json
import os
from pathlib import Path
import runpy
import subprocess
from tempfile import mkdtemp
from threading import Thread

ticket = Path(__file__).resolve().parent
release = runpy.run_path(str(ticket / "release-receipts.py"))
acceptance = runpy.run_path(str(ticket / "accept-site.py"))
static = runpy.run_path(str(ticket / "static-origin.py"))
matrix = runpy.run_path(str(ticket / "verify-boot-matrix.py"))


class OverlayAsset(static["StaticAsset"]):
    def __init__(self, *args, overlay, mounts, **kwargs):
        self.overlay = overlay
        self.mounts = mounts
        super().__init__(*args, **kwargs)

    def translate_path(self, path):
        captured = Path(super().translate_path(path))
        relative = captured.relative_to(self.directory)
        if any(relative.is_relative_to(mount) for mount in self.mounts):
            return str(self.overlay / relative)
        return str(captured)


def verify_owner_requests(access, folders):
    requests = matrix["verify_origin"](access)
    records = matrix["origin_requests"](access)
    missing = []
    for folder in folders:
        hits = [(method, path, status) for method, path, status in records if path.startswith(f"/{folder}/")]
        cores = sum(method == "GET" and path.endswith(".core.wasm.gz.bin") and status == 200
                    for method, path, status in hits)
        print(f"[DEBUG] {folder}: {len(hits)} requests, {cores} encoded core fetches")
        if not cores:
            missing.append(folder)
    if missing:
        raise ValueError(f"Selected guest core was not fetched: {', '.join(missing)}")
    return requests


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("site", type=Path)
    parser.add_argument("modules", nargs="+")
    parser.add_argument("--grep", required=True)
    parser.add_argument("--project", choices=("en", "de"), default="en")
    parser.add_argument("--memory", type=int, default=2048)
    parser.add_argument("--behaviours", action="store_true")
    parser.add_argument("--raw", type=Path)
    parser.add_argument("--raw-sha256")
    parser.add_argument("--transpiled", type=Path)
    args = parser.parse_args()
    if bool(args.raw) != bool(args.raw_sha256) or (args.raw and len(args.modules) != 1) or (args.transpiled and not args.raw):
        parser.error("raw diagnostic requires one module and its exact SHA-256")
    cases = json.loads(release["CASES"].read_bytes())
    available = {module for module, _ in release["roster"](cases)}
    if len(args.modules) != len(set(args.modules)) or not set(args.modules).issubset(available):
        parser.error("expected distinct shipped release module IDs")

    site = args.site.resolve(strict=True)
    if not (site / "index.html").is_file():
        raise FileNotFoundError(site / "index.html")
    generated = acceptance["GENERATED"]
    generated.mkdir(exist_ok=True)
    work = Path(mkdtemp(dir=generated, prefix="ov-"))
    overlay = work / "x"
    staging = acceptance["ROOT"] / ".cache/b/p/m/browser/release/modules"
    confirmations = generated / "confirmed"
    selected = []
    if args.raw:
        metadata = subprocess.check_output(
            ["repo", "capacity", "run", f"--memory={args.memory}", "--", "repo", "tools", "exec", "--light", "bun", "--",
             str(ticket / "probe-raw-owner.ts"), str(overlay), args.modules[0], str(args.raw.resolve(strict=True)), args.raw_sha256,
             *([str(args.transpiled.resolve(strict=True))] if args.transpiled else [])],
            cwd=acceptance["ROOT"], encoding="utf-8",
        )
    else:
        for module in args.modules:
            staged = release["receipt_path"](staging, module)
            confirmation = confirmations / f"{module}.json"
            receipt, inventory = release["read_confirmation"](confirmation, module)
            if release["read_receipt"](staged, module) != receipt:
                raise ValueError(f"Confirmed staged release changed: {module}")
            release["verify_recorded_cores"](staged.parent, inventory)
            selected.append((module, staged, receipt, inventory))
        metadata = subprocess.check_output(
            ["repo", "tools", "exec", "--light", "bun", "--", str(ticket / "probe-owner-assets.ts"), str(overlay),
             *(staged.parent.name for _, staged, _, _ in selected)], cwd=acceptance["ROOT"], encoding="utf-8",
        )
    plan = json.loads(metadata.splitlines()[-1])
    if len(args.modules) != len(plan["modules"]):
        raise ValueError("Incomplete private module overlay")
    if args.raw:
        if plan["raw"] != args.raw_sha256 or not plan["semantic"]:
            raise ValueError("Incomplete diagnostic raw component evidence")
        print(f"[DEBUG] Diagnostic guest {args.modules[0]} raw {plan['raw']}, descriptor {plan['semantic']}, unoptimized.", flush=True)
    for (module, staged, expected, inventory), folder in zip(selected, plan["modules"]):
        deployed = release["receipt_path"](overlay / Path(folder).parent, module)
        if any(release["read_receipt"](path, module) != expected for path in (staged, deployed)):
            raise ValueError(f"Private diagnostic materialization changed receipt: {module}")
        release["verify_recorded_cores"](deployed.parent, inventory)
        release["verify_deployed_module"](staged.parent, deployed.parent)
    release["verify_deployed_shard_worker"](overlay)
    mounts = [Path(folder) for folder in plan["mounts"]]
    (work / "mounts.json").write_text(json.dumps(plan, ensure_ascii=False), encoding="utf-8")
    project = acceptance["DEMONSTRATOR"]
    cli = subprocess.check_output(
        ["repo", "tools", "exec", "--light", "node", "--", "-p", 'require.resolve("@playwright/test/cli")'],
        cwd=project, encoding="utf-8",
    ).strip()
    config = ("./🧪️tests/🎭️acceptance/🧭️behaviours/🎚️config/🟦️.ts" if args.behaviours
              else "./🔨️modules/🧪️e2e/🎚️config/🟦️.ts")
    report = work / "playwright.json"
    origin = work / "origin.log"
    with origin.open("w", encoding="utf-8") as log:
        handler = partial(OverlayAsset, directory=str(site), overlay=overlay, mounts=mounts, access_logger=static["access_logger"](log))
        with static["StaticOrigin"](("127.0.0.1", 0), handler) as server:
            thread = Thread(target=server.serve_forever, daemon=True)
            thread.start()
            try:
                address = f"http://cdn.localhost:{server.server_port}/"
                env = dict(os.environ, DEMONSTRATOR_VERIFY_CDN="1", PLAYWRIGHT_BASE_URL=address,
                           PLAYWRIGHT_JSON_OUTPUT_FILE=str(report))
                env.setdefault("PLAYWRIGHT_BROWSERS_PATH", str(acceptance["ROOT"] / ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"))
                print(f"[DEBUG] Private guest probe at {address}. Evidence: {work}", flush=True)
                result = subprocess.run(
                    ["repo", "capacity", "run", f"--memory={args.memory}", "--", "repo", "tools", "exec", "--light", "node", "--",
                     cli, "test", "--config", config, "--project", args.project, "--grep", args.grep, "--output", str(work / "pw"),
                     "--retries=0", "--reporter=list,json"], cwd=project, env=env,
                )
            finally:
                server.shutdown()
                thread.join()
    access = origin.read_text(encoding="utf-8")
    requests = verify_owner_requests(access, plan["modules"])
    print(f"[DEBUG] {requests} origin requests. Probe exit code {result.returncode}.")
    raise SystemExit(result.returncode)


if __name__ == "__main__":
    main()

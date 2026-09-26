#!/usr/bin/env python3
"""🛬️ P8 landing window, one command: every prepared patch set in order, each proven dry-run clean first, then written and
compile-gated with `cargo check -p` of exactly the crates it touches (plus one native guest per affected family after the
SDK change). A failing gate restores that patch's files — only those still byte-equal to what this run wrote — and stops,
so the tree is never left half-landed; patches before it stay landed (their gates were green).

  python3 p8-land.py                  dry-run every set, print the plan (default; writes nothing)
  python3 p8-land.py --write          land all in order: agent-lane, law, flow, cad, space-studio, space-home
  python3 p8-land.py --write --from flow           resume at one set
  python3 p8-land.py --write --only cad,space-home land a subset
  python3 p8-land.py --test           after the gates (or alone, on a landed tree) run the laws the sets changed
  python3 p8-land.py --restore flow   put one set's pre-landing files back (only files nobody edited since)
  python3 p8-land.py --gates-only --only agent-lane   re-run a landed set's compile gates (nothing written or restored)
  python3 p8-land.py --write --only orphan   HELD set, not in the default order: land it only with the class fix
                                             for content-addressed children (it turns the reasoning law red today)

Every cargo runs at normal priority (session 13 rule 3: landing slices), `CARGO_INCREMENTAL=0`, and waits while more than 14
rustc run; binaries go to the landing slice's private target `.tmp-ticket/wp-lc/target`, intermediates to the landing
build-dir `build-landing` (session 13 rule 27). Logs, backups
(`land-backup/<set>/`) and the cargo target live under `.🧬semio/🌐hub/s12-p8-land/` — the ticket's gitignored
`generated/` and `target/` were swept by a restore at 12:50, and a sweep mid-window must not take the backups with it."""
import json
import os
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = Path("/Users/ueli/Documents/semio")
PATCHES = HERE / "patches"
GENERATED = ROOT / ".🧬semio/🌐hub/s13-lc-p8-land"
BACKUP = GENERATED / "land-backup"
PLUGIN = ["-p", "semio-framework-plugin", "--features", "artifact-app-testing"]
VERDICT_TWIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⚖️declared-verb-verdicts/🟦️.ts"

SETS = [
    ("agent-lane", [["-p", "semio-framework", "-p", "semio-framework-plugin", "-p", "semio-s-artifact-flow-flow", "-p", "semio-s-artifact-cad-cad", "-p", "semio-s-artifact-sequence-sequence", "-p", "semio-s-artifact-architect-program", "--features", "semio-framework-plugin/artifact-app-testing", "--lib", "--tests"]]),
    ("law", [PLUGIN + ["--lib", "--tests"]]),
    ("flow", [["-p", "semio-s-artifact-flow-flow", "-p", "semio-s-plugin-flow", "--lib", "--tests"]]),
    ("cad", [["-p", "semio-s-artifact-cad-cad", "--lib", "--tests"]]),
    ("space-studio", [["-p", "semio-s-plugin-space", "--lib", "--tests"]]),
    ("space-home", [["-p", "semio-s-artifact-space-home", "--lib", "--tests"]]),
]
HELD = [("orphan", [PLUGIN + ["--lib", "--tests"]])]
LAWS = [
    ("agent-lane", ["-p", "semio-s-artifact-reasoning-wires", "--lib", "--", "declared_verb_laws"]),
    ("agent-lane", ["-p", "semio-s-artifact-architect-program", "--lib", "--", "declared_verb_laws"]),
    ("law", PLUGIN + ["--lib", "--", "declared_verb_verdict"]),
    ("orphan", PLUGIN + ["--lib", "--", "declared_verb_verdict"]),
    ("flow", ["-p", "semio-s-artifact-flow-flow", "--lib", "--", "--test-threads", "4"]),
    ("cad", ["-p", "semio-s-artifact-cad-cad", "--lib", "--", "import_cad_file", "--test-threads", "4"]),
    ("space-studio", ["-p", "semio-s-plugin-space", "--lib", "--", "retained_command_catalog", "--test-threads", "4"]),
    ("space-home", ["-p", "semio-s-artifact-space-home", "--lib", "--", "retained", "--test-threads", "4"]),
]


def patch_script(name: str) -> Path:
    return PATCHES / f"p8-{name}.py"


def run_patch(name: str, mode: str) -> tuple[int, list[str]]:
    result = subprocess.run([sys.executable, str(patch_script(name)), mode], cwd=PATCHES, capture_output=True, text=True)
    files = [line.strip() for line in result.stdout.splitlines() if line.startswith("   ")]
    return result.returncode, files if result.returncode == 0 else [result.stdout + result.stderr]


def wait_for_rustc(limit: int = 14) -> None:
    while True:
        count = subprocess.run(["pgrep", "-x", "rustc"], capture_output=True, text=True).stdout.split()
        if len(count) <= limit:
            return
        print(f"  … {len(count)} rustc running, waiting", flush=True)
        time.sleep(30)


def cargo(verb: str, args: list[str], log: Path) -> bool:
    wait_for_rustc()
    env = dict(os.environ, CARGO_INCREMENTAL="0", CARGO_TARGET_DIR=str(ROOT / ".tmp-ticket/wp-lc/target"), CARGO_BUILD_BUILD_DIR=str(ROOT / ".🧬semio/🦑️repo/⚡️cache/cargo/build-landing"), RUST_MIN_STACK="33554432")
    split = args.index("--") if "--" in args else len(args)
    command = ["cargo", verb, *args[:split]]
    command += ["--message-format", "short"] if verb == "check" else ["--no-fail-fast", *args[split:]]
    log.parent.mkdir(parents=True, exist_ok=True)
    started = time.time()
    with log.open("w", encoding="utf-8") as sink:
        code = subprocess.run(command, cwd=ROOT, env=env, stdout=sink, stderr=subprocess.STDOUT).returncode
    text = log.read_text(encoding="utf-8", errors="replace")
    warnings = sum(1 for line in text.splitlines() if "warning" in line)
    summary = [line for line in text.splitlines() if line.startswith("test result") or ": error" in line or line.startswith("error")]
    print(f"  cargo {verb} {' '.join(args)} → exit {code}, {warnings} warning lines, {time.time() - started:.0f}s ({log.name})", flush=True)
    for line in summary[:12]:
        print(f"    {line[:220]}")
    return code == 0


def backup(name: str, files: list[str]) -> None:
    target = BACKUP / name
    target.mkdir(parents=True, exist_ok=True)
    manifest = {}
    for index, rel in enumerate(files):
        source = ROOT / rel
        stored = target / f"{index}.orig"
        stored.write_bytes(source.read_bytes() if source.exists() else b"")
        manifest[rel] = {"orig": stored.name, "existed": source.exists()}
    (target / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2), encoding="utf-8")


def record_written(name: str) -> None:
    target = BACKUP / name
    manifest = json.loads((target / "manifest.json").read_text(encoding="utf-8"))
    for index, rel in enumerate(manifest):
        (target / f"{index}.landed").write_bytes((ROOT / rel).read_bytes())


def restore(name: str) -> bool:
    target = BACKUP / name
    manifest = json.loads((target / "manifest.json").read_text(encoding="utf-8"))
    clean = True
    for index, (rel, entry) in enumerate(manifest.items()):
        current, landed = ROOT / rel, target / f"{index}.landed"
        if landed.exists() and current.exists() and current.read_bytes() != landed.read_bytes():
            print(f"  ✗ {rel} changed after landing (a peer edit?) — left as is, restore it by hand from {target / entry['orig']}")
            clean = False
            continue
        if entry["existed"]:
            current.write_bytes((target / entry["orig"]).read_bytes())
        elif current.exists():
            current.unlink()
        print(f"  ↩ {rel}")
    return clean


def selected(argv: list[str]) -> list[tuple[str, list[list[str]]]]:
    names = [name for name, _ in SETS]
    if "--only" in argv:
        wanted = argv[argv.index("--only") + 1].split(",")
        known = SETS + HELD
        unknown = [name for name in wanted if name not in [entry[0] for entry in known]]
        if unknown:
            sys.exit(f"unknown set(s): {unknown}; known: {[entry[0] for entry in known]}")
        return [entry for entry in known if entry[0] in wanted]
    if "--from" in argv:
        start = argv[argv.index("--from") + 1]
        if start not in names:
            sys.exit(f"unknown set {start}; known: {names}")
        return SETS[names.index(start):]
    return SETS


def main(argv: list[str]) -> int:
    if "--restore" in argv:
        return 0 if restore(argv[argv.index("--restore") + 1]) else 1
    plan = selected(argv)
    write, test = "--write" in argv, "--test" in argv
    if "--gates-only" in argv:
        for name, gates in plan:
            for index, gate in enumerate(gates):
                if not cargo("check", gate, GENERATED / f"regate-{name}-check-{index}.txt"):
                    print(f"gate failed for {name} (nothing restored: --gates-only)")
                    return 1
            print(f"  ✓ {name} compiles")
        return 0
    if write or not test:
        print("preflight: every set dry-run clean on the tree right now")
        failed = False
        for name, _ in plan:
            code, files = run_patch(name, "--dry-run")
            print(f"  {'✓' if code == 0 else '✗'} {name}: {len(files) if code == 0 else 'FAILED'} file(s)")
            if code != 0:
                print(files[0])
                failed = True
        if failed:
            print("nothing written — rebuild the failing set from the clone (`p8-hunks.py`) or re-anchor it")
            return 1
    if write:
        for name, gates in plan:
            print(f"landing {name}")
            _, files = run_patch(name, "--dry-run")
            backup(name, files)
            code, output = run_patch(name, "--write")
            if code != 0:
                print(output[0])
                return 1
            record_written(name)
            for index, gate in enumerate(gates):
                if not cargo("check", gate, GENERATED / f"land-{name}-check-{index}.txt"):
                    print(f"gate failed after {name}: restoring its files and stopping")
                    restore(name)
                    return 1
            print(f"  ✓ {name} landed and compiles")
    if test:
        names = {name for name, _ in plan}
        ok = True
        if names & {"law", "orphan"}:
            twin = subprocess.run(["bun", str(VERDICT_TWIN)], cwd=ROOT, capture_output=True, text=True)
            print(f"  bun declared-verb verdict AJV twin → exit {twin.returncode}: {(twin.stdout + twin.stderr).strip()[-300:]}")
            ok = twin.returncode == 0 and ok
        for index, (name, args) in enumerate(LAWS):
            if name in names:
                ok = cargo("test", args, GENERATED / f"land-{name}-test-{index}.txt") and ok
        return 0 if ok else 1
    if not write:
        print("dry run only — land with: python3 " + str(Path(__file__)) + " --write --test")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

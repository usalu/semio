"""🦀 Continue the Animate tests with pinned native Cargo when compiler admission stalls."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUTPUT = TICKET / "🗑️generated/motion2"
PACKAGE = "semio-s-artifact-animate-presentation-"
MANIFEST = ROOT / "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/Cargo.toml"
RECEIPT = OUTPUT / "cargo-profile.json"
FILTERS = {
    "mutations": ["compile_presentation_site_writes_static_bundle", "compile_scene_to_assets_writes_mp4", "title_cards_match_the_neutral_xml_oracle"],
    "animate": ["export_video_from_deck_emits_the_decks_video_program"],
    "video": ["writer_buffers_frame_and_finalizes_a_real_decodable_mp4", "concatenated_movies_decode_each_partial_and_keep_gif_sidecars",
              "gif_only_preserves_the_exact_owner_and_chip_colors"],
    "v1": [],
}


def capture(pid):
    """🎛️ Reuse the admitted Cargo target and profile arguments without recording credentials."""
    argv = subprocess.check_output(["ps", "-ww", "-o", "command=", "-p", pid], text=True).strip()
    assert f"-p {PACKAGE}" in argv, argv[:200]
    cargo = argv.split(" test ", 1)[0]
    assert Path(cargo).name == "cargo" and Path(cargo).is_file(), cargo
    settings = re.findall(r"--config (.*?)(?= --config|$)", argv)
    env = subprocess.check_output(["ps", "eww", "-p", pid], text=True)
    permitted = {}
    for key in ("CARGO_TARGET_DIR", "CARGO_BUILD_BUILD_DIR", "CARGO_HOME", "RUSTUP_TOOLCHAIN"):
        match = re.search(r"(?:^|\s)" + key + r"=(.*?)(?=\s+[A-Za-z_][A-Za-z0-9_]*=|$)", env)
        assert match, key
        permitted[key] = match.group(1)
    RECEIPT.write_text(json.dumps({"cargo": cargo, "settings": settings, "env": permitted}))


def run(reuse):
    """🧪 Compile the admitted native test once, then run each output contract explicitly."""
    profile = json.loads(RECEIPT.read_text())
    cargo = Path(profile["cargo"])
    runtime = OUTPUT / "t"
    runtime.mkdir(exist_ok=True)
    env = {**os.environ, **profile["env"], "RUSTC": str(cargo.with_name("rustc")), "RUSTC_WRAPPER": "",
           "RUSTC_WORKSPACE_WRAPPER": "", "TMPDIR": str(runtime)}
    options = [arg for setting in profile["settings"] for arg in ("--config", setting)]
    packages = [arg for suffix in FILTERS for arg in ("-p", PACKAGE + suffix)]
    command = [str(cargo), "test", "--manifest-path", str(MANIFEST), "--locked", "--offline", "--jobs", "2",
               "--profile", "prototype", *packages, "--lib", "--no-run", "--verbose", "--message-format=json-render-diagnostics", *options]
    stream = OUTPUT / "cargo-final.jsonl"
    diagnostics = OUTPUT / "native-final.log"
    if not reuse:
        with stream.open("w") as results, diagnostics.open("w") as log:
            build = subprocess.run(command, cwd=ROOT, env=env, stdout=results, stderr=log)
        if build.returncode:
            print(diagnostics.read_text()[-6000:])
            return build.returncode
    artifacts = [event for line in stream.read_text().splitlines() if line.startswith("{")
                 and (event := json.loads(line)).get("reason") == "compiler-artifact"]
    executables = {event["target"]["name"].replace("_", "-"): event
                   for event in artifacts if event.get("executable") and event["profile"]["test"]}
    with diagnostics.open("a") as log:
        results = []
        for suffix, patterns in FILTERS.items():
            artifact = executables[PACKAGE + suffix]
            executable = artifact["executable"]
            test_env = {**env, "CARGO_MANIFEST_DIR": str(Path(artifact["manifest_path"]).parent)}
            for pattern in patterns:
                test = subprocess.run([executable, pattern, "--nocapture"], cwd=runtime, env=test_env,
                                      check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
                log.write(test.stdout)
                results.append(test.returncode or int("running 1 test\n" not in test.stdout))
            results.append(subprocess.run([executable, "--test-threads=2"], cwd=runtime, env=test_env,
                                          stdout=log, stderr=subprocess.STDOUT).returncode)
    print(diagnostics.read_text()[-5000:])
    return max(results)


if __name__ == "__main__":
    OUTPUT.mkdir(parents=True, exist_ok=True)
    if len(sys.argv) == 3 and sys.argv[1] == "capture":
        capture(sys.argv[2])
    else:
        sys.exit(run(reuse=sys.argv[1:] == ["reuse"]))

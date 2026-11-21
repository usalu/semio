"""🔗 Link visible PDF qualification against the actual published native Save package."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "visible-save"
LIBRARIES = ("semio_s_artifact_stdio_pdf_native_save", "serde_json")
CACHE = Path(sys.argv[1]).resolve()
MODE = sys.argv[2]
WASI = len(sys.argv) > 3 and sys.argv[3] == "wasi"


def run():
    """🧪 Compile the exact candidate or unchanged proof against coherent native dependencies."""
    assert MODE in ("red", "green")
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "law.rs").write_bytes((TICKET / "📜️pdf-save-visible-graph-law.rs").read_bytes())
    (OUT / "law-output.rs").write_bytes((TICKET / "📜️pdf-save-law-output.rs").read_bytes())
    (OUT / "law.json").write_bytes((TICKET / "🔣️pdf-save-visible-graph.json").read_bytes())
    if MODE == "green":
        ((path, before, candidate),) = json.loads((GENERATED / "pdf-save-visible-graph-overlay.json").read_text())
        assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == before
        (OUT / "proof.rs").write_text(candidate, encoding="utf8")
    target = OUT / (MODE + (".wasm" if WASI else ".exe" if os.name == "nt" else ""))
    args = ["--edition=2024", "--crate-name", "pdf_retained_visible_graph", str(OUT / "law.rs"), "-o", str(target)]
    args += (["--target=wasm32-wasip2", "-Clink-arg=-zstack-size=8388608"] if WASI else ["--test"])
    if MODE == "green":
        args.extend(("--cfg", "candidate"))
    for name in LIBRARIES:
        choices = list(CACHE.glob(f"*/*/out/lib{name}-*.rlib"))
        assert len(choices) == 1, (name, choices)
        selected = choices[0]
        args.extend(("--extern", f"{name}={selected.with_suffix('.rmeta')}", "--extern", f"{name}={selected}"))
    caches = {CACHE}
    if WASI:
        caches.add(CACHE.parents[2] / "prototype" / "build")
        metadata = OUT / ("md-" + CACHE.parents[2].name)
        metadata.mkdir(exist_ok=True)
        for cache in caches:
            for source in cache.glob("*/*/out/*.rmeta"):
                linked = metadata / source.name
                if linked.exists():
                    assert linked.read_bytes() == source.read_bytes(), linked
                else:
                    try:
                        linked.symlink_to(source)
                    except (NotImplementedError, OSError):
                        linked.write_bytes(source.read_bytes())
        args.extend(("-L", f"dependency={metadata}"))
    for cache in sorted(caches):
        for path in sorted(cache.glob("*/*/out")):
            args.extend(("-L", f"dependency={path}"))
    response = OUT / ("args-" + MODE)
    response.write_text("\n".join(args) + "\n", encoding="utf8")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
    env = {**os.environ, "SEMIO_PDF_VISIBLE_FIXTURE": str(OUT), "RUST_MIN_STACK": "134217728"}
    command = [str(target), "--nocapture"]
    if WASI:
        command = ["repo", "tools", "exec", "--light", "jco", "--", "run", str(target), "--jco-dir", str(OUT / (MODE + "-run"))]
    run = subprocess.run(command, cwd=OUT, env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    print(run.stdout, end="")
    if MODE == "red":
        assert run.returncode != 0 and "retained qualified visible graph" in run.stdout
    else:
        run.check_returncode()


if __name__ == "__main__":
    run()

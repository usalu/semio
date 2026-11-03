import importlib, json, os, tempfile, threading, time
from pathlib import Path
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
errors, results = [], []
def fake(command, progress=None):
    out = Path(command[command.index("--out") + 1])
    time.sleep(0.05)
    for cast in command[command.index("--ids") + 1].split(","):
        (out / cast).mkdir(parents=True)
        for n in range(30):
            (out / cast / f"{n:04d}.svg").write_text("<svg/>", encoding="utf-8")
        (out / cast / "🔣️.json").write_text(json.dumps({"cast": cast}), encoding="utf-8")
with tempfile.TemporaryDirectory() as root:
    os.environ["SEMIO_TEACHING_CACHE_ROOT"] = root
    S._run, S._repo = fake, lambda: "repo"
    ids = list(S._manifest()["casts"])[:3]
    gate = threading.Barrier(8)
    def work():
        gate.wait()
        try:
            folders = S.ensure_casts(ids)
            results.append(all((f / "🔣️.json").is_file() and len(list(f.glob("*.svg"))) == 30 for f in folders.values()))
        except BaseException as error:
            errors.append(repr(error))
    threads = [threading.Thread(target=work) for _ in range(8)]
    [t.start() for t in threads]; [t.join() for t in threads]
    print("errors", errors, "ok", results.count(True), "of", len(results))
    print("leftovers", [p.name for p in Path(root, "staffage").iterdir() if p.name.startswith(".")])

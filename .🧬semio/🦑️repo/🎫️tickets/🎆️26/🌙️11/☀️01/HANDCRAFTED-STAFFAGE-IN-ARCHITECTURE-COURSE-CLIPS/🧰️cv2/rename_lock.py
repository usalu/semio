import importlib
import json
import os
import tempfile
import threading
import time
from pathlib import Path

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
HOLD = 0.4


def scenario(hold: float) -> str:
    with tempfile.TemporaryDirectory() as root:
        os.environ["SEMIO_TEACHING_CACHE_ROOT"] = root
        released = threading.Event()

        def fake(command, progress=None):
            out = Path(command[command.index("--out") + 1])
            for cast in command[command.index("--ids") + 1].split(","):
                (out / cast).mkdir(parents=True)
                (out / cast / "0000.svg").write_text("<svg/>", encoding="utf-8")
                (out / cast / "🔣️.json").write_text(json.dumps({"cast": cast}), encoding="utf-8")
                handle = open(out / cast / "0000.svg", "rb")

                def release(handle=handle):
                    time.sleep(hold)
                    handle.close()
                    released.set()

                threading.Thread(target=release, daemon=True).start()

        S._run, S._repo = fake, lambda: "repo"
        try:
            folders = S.ensure_casts(["shared-lamp-neutral"])
            return "ok " + str((next(iter(folders.values())) / "🔣️.json").is_file())
        except BaseException as error:
            return "FAILED " + repr(error)[:200]
        finally:
            released.wait(5)


print("scanner holds the folder for", HOLD, "s:", scenario(HOLD))
print("no lock:", scenario(0.0))

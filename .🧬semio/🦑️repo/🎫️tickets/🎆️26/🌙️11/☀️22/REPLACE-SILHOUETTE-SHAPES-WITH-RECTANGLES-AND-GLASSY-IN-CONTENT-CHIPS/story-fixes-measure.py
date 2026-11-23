"""📏 Prints the measured DOM rectangles of the production teaching window frame. usage: --script this.py"""
import importlib, json, os, sys, tempfile
sys.path.insert(0, os.getcwd())
P = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
tests = importlib.import_module(P + ".📐️geometry.🧪️tests.🐍️")
browser, hierarchy = (importlib.import_module(P + name) for name in (".🌐️browser.🐍️", ".🥞️hierarchy.🐍️"))
contract = importlib.import_module(P + ".🖼️posters.🧪️tests.🧰️synthetic.🐍️").catalog()
video = hierarchy.ordered_chapters()[0]
with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as directory:
    out = os.path.join(directory, "m.png")
    batch = {"schema": "semio-presentation-browser/v1", "catalog": contract, "work_directory": directory, "language": "de", "viewport": {"width": 1600, "height": 900},
             "frames": [{"output": out, "media": [], "hierarchy": {"opened_path": hierarchy.path(video["id"]), "revealed_path": [], "camera": {"x": 0, "y": 0}}, "cursor": {"point": None, "kind": "arrow", "pressed": False}}]}
    frame = browser.render_batch(batch)["metadata"][0]
    window = next(w for w in frame["windows"] if w["id"] == video["id"])
    print(json.dumps({k: window[k] for k in ("rect", "title_rect", "title_chip_rect", "control_rect", "content_rect", "chip_regions")}, indent=1))
    print(json.dumps({"navbar": frame["navbar"], "footer": frame["footer"]}))

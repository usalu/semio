"""🧪 Check supplementary evidence with finite independent PyAV/Pillow samples."""
import importlib.util
import json
from pathlib import Path

import av
from PIL import Image

TICKET = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("evidence", TICKET / "inspection-evidence.py")
evidence = importlib.util.module_from_spec(spec)
spec.loader.exec_module(evidence)
work = TICKET / "🗑️generated/iv/evidence-selftest"
work.mkdir(parents=True, exist_ok=True)
source = work / "native.mp4"
with av.open(str(source), "w") as output:
    stream = output.add_stream("libx264", rate=4)
    stream.width, stream.height, stream.pix_fmt = 480, 270, "yuv420p"
    stream.options = {"crf": "0"}
    for color in ("red", "green", "blue"):
        frame = av.VideoFrame.from_image(Image.new("RGB", (480, 270), color))
        for packet in stream.encode(frame):
            output.mux(packet)
    for packet in stream.encode():
        output.mux(packet)
with av.open(str(source)) as media:
    frames = [frame.to_image().convert("RGB") for frame in media.decode(video=0)]
sheet = work / "sheet.png"
image = Image.new("RGB", (1440, 296), "#222222")
for index, frame in enumerate(frames):
    image.paste(frame, (index * 480, 0))
image.save(sheet)
requests = [{"path": sheet, "box": (index * 480, 0, (index + 1) * 480, 270), "seconds": index / 4}
            for index in range(3)]
passed = []
evidence.compare_samples(source, requests)
passed.append("exact-native-samples")
image.putpixel((480, 0), (255, 255, 255))
image.save(sheet)
try:
    evidence.compare_samples(source, requests)
except AssertionError:
    passed.append("modified-sheet")
else:
    raise AssertionError("Changed evidence tile was accepted")
image.paste(frames[1], (480, 0))
image.save(sheet)
try:
    evidence.compare_samples(source, [{**requests[0], "seconds": .125}])
except AssertionError:
    passed.append("wrong-native-clock")
else:
    raise AssertionError("Non-native sample time was accepted")
for label, actual, expected in (("missing-clause", ["a"], ["a", "b"]),
                                ("duplicate-stage", [("a", "early")] * 2, [("a", "early"), ("a", "late")])):
    try:
        evidence.exact_coverage(actual, expected)
    except AssertionError:
        passed.append(label)
    else:
        raise AssertionError("Incomplete or duplicate visual coverage was accepted")
assert passed == evidence.CONFIG["controls"], passed
(TICKET / "inspection-evidence-test.md").write_text(
    "# Supplementary Evidence Controls\n\nIndependent PyAV encoded three finite native frames; Pillow assembled their contact sheet. "
    "Exact decoded tile pixels passed. A one-pixel modification, non-native timestamp, missing clause and duplicate stage were rejected. "
    "All writes stayed under generated/iv/evidence-selftest and this report.\n", encoding="utf-8")
print("[DEBUG] Supplementary evidence five controls passed", flush=True)

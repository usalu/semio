"""📐 Align the actual failed physical chapter with its canonical decoded body."""
from collections import Counter, defaultdict
import hashlib
import importlib.util
import itertools
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("collection_verify", TICKET / "inspection-chapter-verify.py")
v = importlib.util.module_from_spec(spec)
spec.loader.exec_module(v)
raw = v.BASE / "1/v/PhysicalFundamentals_FullSection.mp4"
chapter = v.chapter_path("PhysicalFundamentals_FullSection")
lookup = defaultdict(list)
for index, frame in enumerate(v.STREAMS.frames(raw)):
    lookup[hashlib.sha256(frame.tobytes()).digest()].append(index)
offsets = Counter()
landmarks = []
for index, frame in enumerate(v.STREAMS.frames(chapter)):
    candidates = lookup.get(hashlib.sha256(frame.tobytes()).digest(), [])
    if len(candidates) == 1:
        offsets[index - candidates[0]] += 1
        landmarks.append([index, candidates[0]])
shift, support = offsets.most_common(1)[0]
maximum, matched, bad = 0.0, 0, []
for actual, expected in zip(itertools.islice(v.STREAMS.frames(chapter), shift, shift + v.STREAMS.info(raw)["frames"]), v.STREAMS.frames(raw), strict=True):
    delta = v.STREAMS.error(actual, expected)[0]
    maximum = max(maximum, delta)
    if delta >= 1:
        bad.append([matched, matched + shift, delta])
    matched += 1
result = {"offset_histogram": offsets.most_common(), "unique_landmarks": len(landmarks),
          "first_landmarks": landmarks[:5], "last_landmarks": landmarks[-5:],
          "different_frames": bad, "body_start": shift, "matched_body_frames": matched, "body_max_mean_error": maximum,
          "opening_frames": shift, "closing_frames": v.STREAMS.info(chapter)["frames"] - shift - matched}
text = json.dumps(result, indent=2)
(TICKET / "inspection-chapter-frame-alignment.md").write_text("# Actual Physical Chapter Frame Alignment\n\n" + "\n".join("    " + line for line in text.splitlines()) + "\n", encoding="utf-8")
print(text)

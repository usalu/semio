"""🎞️ Verify all independent BIM preview outputs with an external media decoder."""
import json
from pathlib import Path
import av

ticket = Path(__file__).resolve().parent
root = ticket / "🗑️generated/f"
ids = ("information", "objects", "representations", "models", "exchange", "consistency")
records = []
for language in ("en", "de"):
    for audience in ("speaker", "spectator"):
        folder = root / language / audience / "rendered"
        files = sorted(folder.glob("BIM_*.mp4"))
        assert len(files) == len(ids), (language, audience, len(files))
        for topic in ids:
            found = [file for file in files if file.name.startswith(f"BIM_{topic}_{audience}_{language}_")]
            assert len(found) == 1, (language, audience, topic)
            file = found[0]
            with av.open(str(file)) as movie:
                stream = movie.streams.video[0]
                assert (stream.width, stream.height) == (640, 360)
                frames = sum(1 for _ in movie.decode(video=0))
                assert frames > 24
                records.append(dict(language=language, audience=audience, topic=topic, frames=frames,
                                    rate=float(stream.average_rate), audio=bool(movie.streams.audio), bytes=file.stat().st_size))
for language in ("en", "de"):
    for topic in ids:
        pair = [record for record in records if record["language"] == language and record["topic"] == topic]
        assert len({record["frames"] for record in pair}) == 1, pair
paths = list((ticket / "🗑️generated").rglob("*"))
longest = max(len(str(path)) for path in paths)
assert longest <= 256, longest
print(json.dumps(dict(outputs=len(records), maximum_path_length=longest, records=records), indent=2))

"""🔬 Independently map stable EB07 decoded frames to its current native assembled body."""
from collections import Counter
from difflib import SequenceMatcher
import hashlib
import importlib.util
import json
from pathlib import Path
import sys

import av

TICKET = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("collection_oracle", TICKET / "inspection-chapter-verify.py")
V = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(V)
SCENE = "EnergyBalance_07_TheorieVsPraxis"


def decode(path):
    """🎞️ Hash every decoded RGB frame and retain actual clocks and decoder context variants."""
    hashes, clocks, formats = [], [], Counter()
    with av.open(str(path)) as media:
        stream = media.streams.video[0]
        stream.codec_context.thread_count = 1
        for frame in media.decode(stream):
            hashes.append(hashlib.sha256(frame.to_ndarray(format="rgb24").tobytes()).hexdigest())
            clocks.append(float(frame.pts * frame.time_base))
            formats[str((frame.width, frame.height, frame.format.name, frame.color_range,
                         frame.colorspace, getattr(frame, "color_primaries", None), getattr(frame, "color_trc", None),
                         stream.codec_context.color_primaries, stream.codec_context.color_trc,
                         str(stream.codec_context.sample_aspect_ratio), tuple(str(item) for item in frame.side_data)))] += 1
    return hashes, clocks, dict(formats)


def packets(path):
    """⏱️ Retain exact packet PTS/DTS and detect sorted clock gaps independently of decoded frame indices."""
    result = []
    with av.open(str(path)) as media:
        stream = media.streams.video[0]
        codec = {"name": stream.codec_context.name, "profile": stream.codec_context.profile,
                 "time_base": str(stream.time_base), "extradata_sha256": hashlib.sha256(stream.codec_context.extradata or b"").hexdigest()}
        for packet in media.demux(stream):
            if packet.size:
                result.append({"pts": float(packet.pts * packet.time_base) if packet.pts is not None else None,
                               "dts": float(packet.dts * packet.time_base) if packet.dts is not None else None,
                               "duration": float(packet.duration * packet.time_base), "key": packet.is_keyframe})
    clocks = {name: sorted(item[name] for item in result if item[name] is not None) for name in ("pts", "dts")}
    gaps = {name: [(index, left, right) for index, (left, right) in enumerate(zip(values, values[1:])) if abs(right - left - .25) > 1e-8]
            for name, values in clocks.items()}
    return {"codec": codec, "gaps": gaps, "packets": result}


def main():
    """🧾 Preserve source/certificate identities before investigating a stable media-law failure."""
    manifest = V.read(V.STREAMS.MANIFEST)["videos"]
    video = next(item for item in manifest if any(chapter["scene"] == SCENE for chapter in item["chapters"]))
    position, entries = V.STREAMS.plan(manifest, V.BASE)[video["id"]]
    raw = V.BASE / str(position) / "v" / (SCENE + ".mp4")
    published = V.chapter_path(SCENE)
    final = Path(sys.argv[1]) if len(sys.argv) > 1 else published
    paths = [raw, final, raw.with_suffix(".render.json"), published.with_suffix(".render.json"), published.with_suffix(".chapter.json")]
    before = {str(path): V.STREAMS.sha(path) for path in paths}
    result = {"scene": SCENE, "raw": str(raw), "final": str(final), "files": before,
              "signature": V.signature(final) if final == published else [(str(final), final.stat().st_size, final.stat().st_mtime_ns)],
              "declared_clock": V.read(published.with_suffix(".chapter.json")),
              "selected_sources": [{"source": str(item["source"]), "selected": str(item["selected"]),
                                    "scene": item["segment"]["scene"]} for item in entries if item["chapter"] == SCENE]}
    print("[DEBUG] Stable EB07 identity", json.dumps(result, ensure_ascii=False), flush=True)
    body, body_pts, body_formats = decode(raw)
    movie, movie_pts, movie_formats = decode(final)
    unique = {digest: index for index, digest in enumerate(body) if body.count(digest) == 1}
    pairs = [(unique[digest], index) for index, digest in enumerate(movie) if digest in unique]
    offsets = Counter(final_index - body_index for body_index, final_index in pairs)
    edits = SequenceMatcher(a=body, b=movie, autojunk=False).get_opcodes()
    result.update({"raw_frames": len(body), "final_frames": len(movie), "raw_pts": [body_pts[0], body_pts[-1]],
                   "final_pts": [movie_pts[0], movie_pts[-1]], "raw_formats": body_formats, "final_formats": movie_formats,
                   "unique_matches": len(pairs), "offset_histogram": dict(offsets), "first_unique_pairs": pairs[:20],
                   "last_unique_pairs": pairs[-20:], "exact_rgb_sequence_edits": edits,
                   "raw_packets": packets(raw), "final_packets": packets(final)})
    for offset, _ in offsets.most_common(3):
        missing = [index for index, digest in enumerate(body) if not (0 <= index + offset < len(movie) and movie[index + offset] == digest)]
        result.setdefault("alignments", []).append({"offset": offset, "matching": len(body) - len(missing),
                                                    "missing_raw_frames": missing})
    assert before == {str(path): V.STREAMS.sha(path) for path in paths}, "EB07 changed during diagnosis"
    name = "eb07-forensics" if final == published else "eb07-" + final.parent.name + "-forensics"
    output = TICKET / ("🗑️generated/iv/" + name + ".json")
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    (TICKET / ("inspection-" + name + ".md")).write_text(
        "# Stable EB07 Frame Loss\n\nIndependent PyAV RGB-frame and PTS analysis of the unchanged native body and published chapter. No renderer or encoder was invoked.\n\n```json\n"
        + json.dumps(result, ensure_ascii=False, indent=2) + "\n```\n", encoding="utf-8")
    print("[DEBUG] EB07 mapping", json.dumps({key: result[key] for key in ("raw_frames", "final_frames", "raw_formats", "final_formats", "offset_histogram", "exact_rgb_sequence_edits")}), flush=True)


if __name__ == "__main__":
    main()

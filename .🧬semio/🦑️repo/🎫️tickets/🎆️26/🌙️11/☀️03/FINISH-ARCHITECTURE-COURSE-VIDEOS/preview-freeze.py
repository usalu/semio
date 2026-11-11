"""🔐 Verify post-decode course media against current canonical source state."""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
MEDIA = HERE.parent.parent / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🧰️r3-media"
sys.path.insert(0, str(MEDIA))
import lib
import chapter_titles


def key(value):
    return hashlib.sha256(lib.canonical(value).encode()).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def verify_entry(name, path, sha, media, manifest, joined):
    for label, entry, field in (("manifest", manifest, "file"), ("receipt", joined, "master")):
        for actual, expected, attribute in ((entry.get(field), path, "path"), (entry.get("sha256"), sha, "sha256"), (entry.get("media"), media, "media")):
            require(actual == expected, f"{name}: {label} {attribute} mismatch for {path}: {actual!r} != {expected!r}")


def main(language):
    require(language in ("en", "de"), f"unsupported language: {language}")
    folder = lib.base(language, "speaker")
    manifest_path = folder / "media-manifest.json"
    preview = folder / "preview" / f"Architecture_course_speaker_{language}_480p15.mp4"
    receipt_path = preview.with_suffix(".json")
    document = json.loads(manifest_path.read_text(encoding="utf-8"))
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    contract = json.loads((HERE / "course-preview-contract.json").read_text(encoding="utf-8"))
    videos = lib.runtime().load_manifest()
    titles = chapter_titles.titles(videos, language)
    order = [video["id"] for video in videos]
    require(len(order) == 26 and order == contract["lessons"], f"{manifest_path}: curriculum order mismatch")
    require(document.get("version") == 1 and document.get("language") == language and document.get("audience") == "speaker", f"{manifest_path}: version or settings mismatch")
    require(receipt.get("language") == language and receipt.get("audience") == "speaker" and [x.get("id") for x in receipt["lessons"]] == order and not receipt.get("missing"), f"{receipt_path}: lesson order/settings mismatch")
    states = {}
    keys = []
    for video, joined in zip(videos, receipt["lessons"], strict=True):
        name = video["id"]
        require(joined.get("title") == titles[name], f"{name}: receipt localized title mismatch")
        status = lib.video_status(name, language, "speaker", video.get("output", name), states)
        path = status["master"]
        require(lib.master_fresh(status), f"{name}: stale canonical master {path}")
        provenance = {"casts": {}, "records": {}}
        for segment in status["segments"]:
            record = lib.provenance(segment["state"], segment["fresh"])
            provenance["casts"].update(record["casts"])
            provenance["records"].update(record.get("records", {}))
        sha = lib.digest(path)
        media = key({"sha256": sha, "states": [s["state_sha256"] for s in status["segments"]], "provenance": provenance, "version": 1})
        current = document["videos"][name]["master"]
        require(current.get("fresh") is True, f"{name}: manifest stale {manifest_path}")
        verify_entry(name, str(path.relative_to(lib.ROOT)), sha, media, current, joined)
        keys.append(media)
        print(f"[DEBUG] verified {name}: {path}", flush=True)
    sha = lib.digest(preview)
    require(receipt.get("sha256") == sha, f"{preview}: preview sha256 mismatch")
    require(receipt.get("media") == key({"sha256": sha, "lessons": keys}), f"{preview}: preview media key mismatch")
    print(f"[DEBUG] verified post-decode freeze: {preview} ({len(order)} lessons)", flush=True)


if __name__ == "__main__":
    try:
        require(len(sys.argv) == 2, "usage: preview-freeze.py <en|de>")
        main(sys.argv[1])
    except Exception as error:
        print(f"post-decode freeze failed: {type(error).__name__}: {error}", file=sys.stderr)
        sys.exit(1)

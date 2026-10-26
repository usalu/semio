from hashlib import sha256
from pathlib import Path
from urllib.request import urlopen
from zipfile import ZipFile

ticket = Path(__file__).parent
destination = ticket / "🗑️generated" / "f"
destination.mkdir(parents=True, exist_ok=True)
url = "https://github.com/GyanD/codexffmpeg/releases/download/9.0.2/ffmpeg-9.0.2-essentials_build.zip"
expected = "60f467265b1e312373dbcd92200c2618a74850f98d3d078e94296bb3fa2047ba"
archive = destination / "archive.zip"
digest = sha256()
with urlopen(url, timeout=30) as response, archive.open("wb") as output:
    count = 0
    mark = 0
    while block := response.read(1024 * 1024):
        output.write(block)
        digest.update(block)
        count += len(block)
        if count - mark >= 10 * 1024 * 1024:
            print(f"[DEBUG] Downloaded FFmpeg essentials: {count // (1024 * 1024)} MiB", flush=True)
            mark = count
if digest.hexdigest() != expected:
    raise RuntimeError("FFmpeg essentials SHA-256 mismatch")
with ZipFile(archive) as source:
    for executable in ["ffprobe.exe", "ffmpeg.exe"]:
        matches = [name for name in source.namelist() if name.endswith("/bin/" + executable)]
        if len(matches) != 1:
            raise RuntimeError(f"Expected one archive entry for {executable}")
        (destination / executable).write_bytes(source.read(matches[0]))
archive.unlink()
with (ticket / "dependency.md").open("a", encoding="utf-8") as report:
    report.write(f"\nFFmpeg and ffprobe were provisioned from the [Gyan release essentials build](https://www.gyan.dev/ffmpeg/builds/) into `{destination}`. The archive SHA-256 matched the publisher checksum `{expected}` before extraction. Only the two executable files were extracted; the downloaded archive was removed.\n")
print(f"[DEBUG] ffprobe ready: {destination / 'ffprobe.exe'}", flush=True)

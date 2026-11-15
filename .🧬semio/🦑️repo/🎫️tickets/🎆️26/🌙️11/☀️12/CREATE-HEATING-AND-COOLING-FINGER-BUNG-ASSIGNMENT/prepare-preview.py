from pathlib import Path
import io
import json
import os
import platform
import tarfile
import urllib.request

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
release = json.load(urllib.request.urlopen("https://api.github.com/repos/tectonic-typesetting/tectonic/releases/tags/tectonic%400.16.9"))
system = platform.system()
machine = platform.machine()
target = ("aarch64" if machine in ("arm64", "aarch64") else "x86_64") + ("-apple-darwin" if system == "Darwin" else "-unknown-linux-gnu")
asset = next(a for a in release["assets"] if target in a["name"] and a["name"].endswith(".tar.gz"))
payload = urllib.request.urlopen(asset["browser_download_url"]).read()
with tarfile.open(fileobj=io.BytesIO(payload), mode="r:gz") as archive:
    member = next(m for m in archive.getmembers() if Path(m.name).name == "tectonic" and m.isfile())
    executable = generated / "tectonic"
    executable.write_bytes(archive.extractfile(member).read())
    executable.chmod(0o755)
print("[DEBUG] Prepared registered compiler override:", executable)
fonts = generated / "fonts"
fonts.mkdir(exist_ok=True)
root = Path.cwd() / "🧰️framework/🛍️products/📓️print/🖼️assets/🔤️font"
for source in root.rglob("*"):
    if source.suffix in (".ttf", ".otf"):
        name = source.name.split("️", 1)[-1]
        destination = fonts / name
        if not destination.exists():
            os.symlink(source.resolve(), destination)
print("[DEBUG] Linked compiler font names:", sorted(p.name for p in fonts.iterdir()))

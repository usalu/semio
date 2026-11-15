import os
import re
import sys
from pathlib import Path

energy = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
alias = re.compile(rb"^set_vo_language = \w+\.set_vo_language\r?\n", re.M)
call = re.compile(rb"^set_vo_language\(\"de\"\)\r?\n", re.M)
old_comment = "# 🗣️ VO reads the German subtitles; measured clause durations".encode()
new_comment = "# 🗣️ Measured clause durations".encode()
changed, failed = [], []


def publish(path: Path, data: bytes) -> None:
    temporary = path.with_name(path.name + ".tmp-vo")
    temporary.write_bytes(data)
    os.replace(temporary, path)


for path in energy.rglob("*.py"):
    s = path.as_posix()
    if ".venv" in s or "site-packages" in s or "__pycache__" in s or "/.🧬semio/" in s:
        continue
    data = path.read_bytes()
    if b"set_vo_language" not in data:
        continue
    new = call.sub(b"", alias.sub(b"", data)).replace(old_comment, new_comment)
    if new == data:
        continue
    try:
        publish(path, new)
        changed.append(path.relative_to(energy).as_posix())
    except OSError as error:
        failed.append((path.relative_to(energy).as_posix(), str(error)))
print("changed", len(changed), "failed", len(failed))
for item in failed:
    print(item[0].encode("ascii", "backslashreplace").decode(), item[1][:80])
sys.exit(1 if failed else 0)

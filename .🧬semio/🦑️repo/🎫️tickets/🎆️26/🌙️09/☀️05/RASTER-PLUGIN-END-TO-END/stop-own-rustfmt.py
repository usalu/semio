from pathlib import Path
import os
import signal

targets = ("🌿️vcs/🔗️read/🦀️.rs", "⏪️time-travel/🔀️supersession-ledger/🦀️.rs", "🧬️native/🔗️history/🦀️.rs")
for entry in Path("/proc").iterdir():
    if not entry.name.isdecimal():
        continue
    try:
        args = (entry / "cmdline").read_bytes().decode().replace("\0", " ")
        if "repo tools exec --light rustfmt" in args and "--emit stdout" in args and any(target in args for target in targets):
            os.kill(int(entry.name), signal.SIGINT)
            print(f"Stopped own queued syntax check {entry.name}")
    except (OSError, UnicodeError):
        pass

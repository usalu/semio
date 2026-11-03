import json
import os
import tempfile


def write_json_atomic(path, value):
    """Write `value` as JSON so a concurrent reader sees the old or the new file, never half of it (temp file in the same folder, then os.replace)."""
    folder = os.path.dirname(os.path.abspath(path))
    fd, temporary = tempfile.mkstemp(dir=folder, suffix=".tmp")
    try:
        with os.fdopen(fd, "w", encoding="utf8", newline="\n") as handle:
            json.dump(value, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
        os.replace(temporary, path)
    except BaseException:
        if os.path.exists(temporary):
            os.remove(temporary)
        raise

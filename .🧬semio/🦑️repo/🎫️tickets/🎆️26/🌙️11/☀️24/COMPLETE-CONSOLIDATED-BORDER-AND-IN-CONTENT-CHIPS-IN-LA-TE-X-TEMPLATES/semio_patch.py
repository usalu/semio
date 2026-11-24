"""🩹 Exact-text patch helper: every pair must match the expected number of times; the write retries while the shared-tree replica holds the file."""
import io
import time


def patch(path, pairs):
    text = io.open(path, encoding="utf-8", newline="").read()
    for old, new, *count in pairs:
        expected = count[0] if count else 1
        assert text.count(old) == expected, (old[:80], text.count(old))
        text = text.replace(old, new)
    error = None
    for _ in range(240):
        try:
            with io.open(path, "r+", encoding="utf-8", newline="") as handle:
                handle.seek(0)
                handle.write(text)
                handle.truncate()
            return
        except OSError as caught:
            error = caught
            time.sleep(0.25)
    raise error

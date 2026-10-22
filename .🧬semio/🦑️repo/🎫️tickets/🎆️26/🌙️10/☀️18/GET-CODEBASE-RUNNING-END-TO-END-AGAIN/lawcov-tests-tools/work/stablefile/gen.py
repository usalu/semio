"""📝 Authoring aid: writes the stable build file corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
CHUNK = 64 * 1024


def case(name, kind, size, maximum, remaining, **kw):
    return {"name": name, "kind": kind, "size": size, "maximum": maximum, "remaining": remaining, **kw}


cases = [
    case("a small file is captured whole", "file", 11, 1024, 5000, ok=True, remainingAfter=4989, checks=2),
    case("an empty file is captured as empty bytes", "file", 0, 1024, 5000, ok=True, remainingAfter=5000, checks=1),
    case("a file of exactly the maximum is admitted", "file", 1024, 1024, 5000, ok=True, remainingAfter=3976, checks=2),
    case("a file one byte over the maximum is refused", "file", 1025, 1024, 5000, error="over the 1024-byte bound", remainingAfter=5000, checks=1),
    case("a file that fills the aggregate budget exactly is admitted", "file", 700, 1024, 700, ok=True, remainingAfter=0, checks=2),
    case("a file that does not fit the aggregate budget is refused and leaves it intact", "file", 701, 1024, 700, error="file changed or exceeds aggregate bound", remainingAfter=700, checks=1),
    case("a file of one chunk is read in one step", "file", CHUNK, 1 << 20, 1 << 20, ok=True, remainingAfter=(1 << 20) - CHUNK, checks=2),
    case("a file of one chunk and one byte is read in two steps", "file", CHUNK + 1, 1 << 20, 1 << 20, ok=True, remainingAfter=(1 << 20) - CHUNK - 1, checks=3),
    case("a file of three and a half chunks is read in four steps", "file", 3 * CHUNK + CHUNK // 2, 1 << 22, 1 << 22, ok=True, remainingAfter=(1 << 22) - 3 * CHUNK - CHUNK // 2, checks=5),
    case("a symbolic link to a file is refused", "symlink", 5, 1024, 5000, error="not a regular file", remainingAfter=5000, checks=1),
    case("a directory is refused", "directory", 0, 1024, 5000, error="not a regular file", remainingAfter=5000, checks=1),
    case("a missing file is refused by the file system", "missing", 0, 1024, 5000, errorCode="ENOENT", remainingAfter=5000, checks=1),
    case("a cancelled capture stops before any file system access", "file", 5, 1024, 5000, cancelAt=1, error="cancelled at check 1", remainingAfter=5000, checks=1),
    case("a capture cancelled between chunks stops after the budget was spent", "file", 2 * CHUNK, 1 << 20, 1 << 20, cancelAt=3, error="cancelled at check 3", remainingAfter=(1 << 20) - 2 * CHUNK, checks=3),
    case("a file appended to between chunks is reported as changed", "file", 2 * CHUNK, 1 << 20, 1 << 20, mutateAt=3, mutation="append", error="file changed while reading", remainingAfter=(1 << 20) - 2 * CHUNK, checks=3),
    case("a file rewritten in place between chunks is reported as changed", "file", 2 * CHUNK, 1 << 20, 1 << 20, mutateAt=3, mutation="touch", error="file changed while reading", remainingAfter=(1 << 20) - 2 * CHUNK, checks=3),
    case("a file truncated between chunks is reported as shortened after one more checkpoint", "file", 2 * CHUNK, 1 << 20, 1 << 20, mutateAt=3, mutation="truncate", error="file shortened", remainingAfter=(1 << 20) - 2 * CHUNK, checks=4),
]
json.dump({"version": 1, "chunk": CHUNK, "cases": cases}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(cases))

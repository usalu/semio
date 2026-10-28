"""🎓️ Independent Python model of Vitest's `--shard=<index>/<count>` assignment (vitest 4 BaseSequencer.shard): the root-relative path of every test file (leading slash) is hashed with SHA-1, files are ordered by the hex digest and cut into `count` consecutive slices whose sizes differ by at most one (the first `files % count` slices are one bigger). Reads {"files":[…],"count":n} on stdin, answers {"shards":[[…],…]} with every shard sorted by path."""
import hashlib, json, sys

def shards(files, count):
    ordered = sorted(files, key=lambda path: hashlib.sha1(("/" + path).encode("utf-8")).hexdigest())
    base, extra = divmod(len(ordered), count)
    out, position = [], 0
    for index in range(1, count + 1):
        size = base + 1 if index <= extra else base
        out.append(sorted(ordered[position:position + size]))
        position += size
    return out

request = json.load(sys.stdin)
json.dump({"shards": shards(request["files"], request["count"])}, sys.stdout)

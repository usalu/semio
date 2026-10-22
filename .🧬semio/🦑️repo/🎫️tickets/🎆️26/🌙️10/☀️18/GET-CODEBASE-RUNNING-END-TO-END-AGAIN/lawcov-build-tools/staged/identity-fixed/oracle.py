"""🪪️ Independent Python oracle of the Nx tooling install identity (standard library only): the key is the sha256 over a version tag and then the platform triple and every file name and content as length-prefixed fields (decimal byte count, a colon, the bytes), in the order given. Requests arrive as JSON on stdin."""
import hashlib, json, sys


def key(environment, files):
    digest = hashlib.sha256()
    digest.update(b"nx-tooling-install-v4\0")

    def field(text):
        data = text.encode("utf8")
        digest.update(str(len(data)).encode() + b":" + data)

    for part in (environment["platform"], environment["architecture"], environment["libc"]):
        field(part)
    for name, text in files:
        field(name)
        field(text)
    return digest.hexdigest()


request = json.load(sys.stdin)
print(json.dumps([key(row["environment"], row["files"]) for row in request["rows"]]))

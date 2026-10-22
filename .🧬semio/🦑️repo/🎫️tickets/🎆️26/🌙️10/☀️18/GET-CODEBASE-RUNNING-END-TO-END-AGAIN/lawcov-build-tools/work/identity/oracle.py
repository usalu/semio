"""🪪️ Independent Python oracle of the Nx tooling install identity (standard library only): the key is the sha256 over a version tag, the platform triple and every file as name NUL bytes NUL, in the order given. Requests arrive as JSON on stdin."""
import hashlib, json, sys


def key(environment, files):
    digest = hashlib.sha256()
    digest.update(("nx-tooling-install-v3\0" + environment["platform"] + "\0" + environment["architecture"] + "\0" + environment["libc"] + "\0").encode("utf8"))
    for name, text in files:
        digest.update(name.encode("utf8") + b"\0" + text.encode("utf8") + b"\0")
    return digest.hexdigest()


request = json.load(sys.stdin)
print(json.dumps([key(row["environment"], row["files"]) for row in request["rows"]]))

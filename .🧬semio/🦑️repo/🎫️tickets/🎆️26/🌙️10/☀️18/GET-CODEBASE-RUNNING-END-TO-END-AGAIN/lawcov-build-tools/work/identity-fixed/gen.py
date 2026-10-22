"""📝 Authoring aid: writes the install identity corpus (corpus.json); expected keys are computed here once from the declared inputs."""
import hashlib, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def key(env, files):
    h = hashlib.sha256(b"nx-tooling-install-v4\0")

    def field(value):
        data = value.encode()
        h.update(str(len(data)).encode() + b":" + data)

    for value in (env["platform"], env["architecture"], env["libc"]):
        field(value)
    for name, text in files:
        field(name)
        field(text)
    return h.hexdigest()


def case(name, env, files):
    return {"name": name, "environment": env, "files": [{"name": n, "text": t} for n, t in files], "expected": key(env, files)}


linux = {"platform": "linux", "architecture": "x64", "libc": "glibc"}
c = {"version": 1}
c["cases"] = [
    case("an installation of two files", linux, [("package.json", "{}\n"), ("bun.lock", "{}\n")]),
    case("no files", linux, []),
    case("macOS on arm64", {"platform": "darwin", "architecture": "arm64", "libc": "none"}, [("package.json", "{}\n")]),
    case("windows", {"platform": "win32", "architecture": "x64", "libc": "none"}, [("package.json", "{}\n")]),
    case("musl is another key than glibc", {**linux, "libc": "musl"}, [("package.json", "{}\n")]),
    case("an empty file", linux, [("empty", "")]),
    case("unicode names and bytes", linux, [("é/🧪.json", "é🧪\n")]),
    case("the order of files is part of the key", linux, [("bun.lock", "{}\n"), ("package.json", "{}\n")]),
    case("a patch beside the dependencies", linux, [("package.json", "{}\n"), ("bun.lock", "{}\n"), ("patches/current.patch", "diff\n")]),
]
c["boundaries"] = [
    {"name": "a byte moved from the name into the content", "left": [["ab", "c"]], "right": [["a", "bc"]], "environment": linux},
    {"name": "an empty name is not no file", "left": [["", ""]], "right": [], "environment": linux},
    {"name": "a content holding NUL bytes cannot impersonate two files", "left": [["a", "x"], ["b", "y"]], "right": [["a", "x\u0000b\u0000y"]], "environment": linux},
    {"name": "a content holding a length prefix cannot impersonate two files", "left": [["a", "x"], ["b", "y"]], "right": [["a", "x1:b1:y"]], "environment": linux},
    {"name": "a name holding a length prefix cannot impersonate a file and its content", "left": [["a", "xx"]], "right": [["a2:xx", ""]], "environment": linux},
    {"name": "a platform triple cannot be shifted into the first file", "left": [], "right": [["linux", ""]], "environment": linux},
    {"name": "a libc of one character differs from a libc of nothing and a file named like it", "left": [["g", ""]], "right": [], "environment": {**linux, "libc": "glibcg"}},
]
c["environments"] = {"platforms": ["linux", "darwin", "win32"], "architectures": ["x64", "arm64"], "libcs": ["glibc", "musl", "none"]}
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)

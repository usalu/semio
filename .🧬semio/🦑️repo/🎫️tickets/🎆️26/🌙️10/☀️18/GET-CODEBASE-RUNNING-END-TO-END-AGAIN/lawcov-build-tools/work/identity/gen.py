"""📝 Authoring aid: writes the install identity corpus (corpus.json); expected keys are computed here once from the declared inputs."""
import hashlib, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def key(env, files):
    h = hashlib.sha256(("nx-tooling-install-v3\0%s\0%s\0%s\0" % (env["platform"], env["architecture"], env["libc"])).encode())
    for name, text in files:
        h.update((name + "\0").encode())
        h.update(text.encode())
        h.update(b"\0")
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
]
c["knownDefects"] = [{"name": "a content holding NUL bytes cannot impersonate two files", "left": [["a", "x"], ["b", "y"]], "right": [["a", "x\u0000b\u0000y"]], "environment": linux}]
c["environments"] = {"platforms": ["linux", "darwin", "win32"], "architectures": ["x64", "arm64"], "libcs": ["glibc", "musl", "none"]}
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)

#!/usr/bin/env python3
"""🔎 Independent oracle for the resident service: hashlib/hmac vectors, jsonschema verdicts on a mutated frame corpus, owner digests of a directory.
usage: oracle.py fixtures OUT.json | oracle.py digest ROOT RELPATH..."""
import hashlib, hmac, json, os, random, sys, base64

HERE = os.path.dirname(os.path.abspath(__file__))
SCHEMA = json.load(open(os.path.join(HERE, "..", "schema", "schema.json"), encoding="utf-8"))


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def b64(data):
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()


def mint(key, caveats, ident):
    mac = hmac.new(key, ident.encode(), hashlib.sha256).digest()
    for caveat in caveats:
        mac = hmac.new(mac, canonical(caveat).encode(), hashlib.sha256).digest()
    payload = json.dumps({"id": ident, "caveats": caveats}, separators=(",", ":"), ensure_ascii=False)
    return "v1." + b64(payload.encode()) + "." + b64(mac)


def validator(name):
    from jsonschema import Draft7Validator
    return Draft7Validator({"$ref": "#/definitions/" + name, "definitions": SCHEMA["definitions"]})


SAMPLES = {
    "ClientFrame": [
        {"t": "hello", "v": 1, "client": {"name": "repo", "pid": 12, "lane": "a"}, "languages": ["de", "en"], "cap": "v1.abcd." + "A" * 43},
        {"t": "hello", "v": 1, "client": {"name": "repo", "pid": 0, "lane": "a"}, "languages": ["en"]},
        {"t": "request", "id": 1, "command": "run", "args": ["p:build"], "cwd": "/w", "env": {"A": "b"}, "deadlineMs": 1000, "detach": False},
        {"t": "request", "id": 4294967295, "command": "run-many", "args": [], "cwd": "/w"},
        {"t": "cancel", "id": 3},
        {"t": "ping", "n": 0},
    ],
    "ServiceFrame": [
        {"t": "welcome", "v": 1, "service": {"pid": 5, "instance": "0123456789abcdef", "startedAt": 1, "implementation": "0" * 64, "generation": None, "watcher": {"backend": "fsevents", "trust": "exact", "cursor": "12"}}, "limits": {"maximumFrameBytes": 1048576, "maximumInFlight": 8}},
        {"t": "accepted", "id": 1, "queue": 0},
        {"t": "progress", "id": 1, "phase": "plan", "done": 1, "total": 2, "message": {"en": "x", "de": "y"}},
        {"t": "output", "id": 1, "stream": "stdout", "text": "hi"},
        {"t": "output", "id": 1, "stream": "stderr", "b64": "aGk="},
        {"t": "result", "id": 1, "exit": 0, "signal": None, "served": "memo", "timing": {"totalMs": 1.5, "fenceMs": 1}, "summary": {"en": "ok"}},
        {"t": "refused", "id": 1, "code": "busy", "message": {"en": "a", "de": "b"}, "retry": "later"},
        {"t": "refused", "code": "capability", "message": {"en": "a", "de": "b"}, "retry": "never"},
        {"t": "notice", "kind": "draining", "message": {"de": "x"}},
        {"t": "pong", "n": 3},
    ],
    "WatcherFrame": [
        {"v": 1, "t": "ready", "backend": "fsevents", "root": "/w", "cursor": "1", "uuid": "U", "replay": False, "latencyMs": 5, "native": True},
        {"v": 1, "t": "batch", "n": 1, "at": 1, "cursor": "2", "c": [["a", 0, "x/y"], ["m", 1, "x"]]},
        {"v": 1, "t": "lost", "at": 1, "reason": "overflow", "scope": ""},
        {"v": 1, "t": "history-done", "cursor": "9"},
        {"v": 1, "t": "error", "code": "c", "message": "m"},
        {"v": 1, "t": "bye"},
        {"v": 1, "t": "version", "name": "semio-repo-watch", "protocol": 1, "os": "macos"},
    ],
    "Descriptor": [
        {"v": 1, "pid": 4, "instance": "0123456789abcdef", "startedAt": 5, "endpoint": "/s", "protocol": 1, "implementation": "f" * 64, "root": "/w"},
        {"v": 1, "pid": 4, "instance": "0123456789abcdef", "startedAt": 5, "endpoint": "/s", "protocol": 1, "implementation": "f" * 64, "root": "/w", "stopping": True},
    ],
    "Caveat": [{"k": "scope", "v": ["run"]}, {"k": "projects", "v": ["a*"]}, {"k": "notAfter", "v": 5}, {"k": "cwd", "v": "/w"}],
}
ALTERNATES = [None, "", "x", -1, 0, 1, 2 ** 40, 1.5, True, [], {}, [1], {"a": 1}, "a" * 300]


def mutations(value, rng):
    out = []

    def walk(node, path):
        if isinstance(node, dict):
            for key in list(node):
                out.append(("drop " + "/".join(path + [key]), path, key, "drop"))
                walk(node[key], path + [key])
            out.append(("extra " + "/".join(path), path, "zz_unexpected", "extra"))
        elif isinstance(node, list):
            for index in range(len(node)):
                walk(node[index], path + [index])
    walk(value, [])
    cases = []
    import copy
    for label, path, key, kind in out:
        clone = copy.deepcopy(value)
        target = clone
        for step in path:
            target = target[step]
        if kind == "drop":
            del target[key]
        else:
            target[key] = 1
        cases.append(clone)
    def leaves(node, path):
        if isinstance(node, dict):
            for key in node:
                yield from leaves(node[key], path + [key])
        elif isinstance(node, list):
            for index in range(len(node)):
                yield from leaves(node[index], path + [index])
        yield path
    for path in leaves(value, []):
        if not path:
            continue
        for alt in rng.sample(ALTERNATES, 6):
            clone = copy.deepcopy(value)
            target = clone
            for step in path[:-1]:
                target = target[step]
            target[path[-1]] = alt
            cases.append(clone)
    return cases


def fixtures(out):
    rng = random.Random(11)
    data = {"sha256": [], "hmac": [], "capabilities": [], "endpoints": [], "frames": []}
    for index in range(20):
        message = bytes(rng.randrange(256) for _ in range(rng.choice([0, 1, 55, 56, 63, 64, 65, 200])))
        key = bytes(rng.randrange(256) for _ in range(rng.choice([1, 32, 64, 65, 131])))
        data["sha256"].append({"hex": message.hex(), "digest": hashlib.sha256(message).hexdigest()})
        data["hmac"].append({"key": key.hex(), "message": message.hex(), "mac": hmac.new(key, message, hashlib.sha256).hexdigest()})
    key = bytes(range(32))
    chains = [[], [{"k": "scope", "v": ["run", "status"]}], [{"k": "scope", "v": ["status", "query", "run", "watch"]}, {"k": "notAfter", "v": 1792537630001}], [{"k": "projects", "v": ["ä*", "p:b"]}, {"k": "cwd", "v": "/w"}, {"k": "scope", "v": ["admin"]}]]
    for index, chain in enumerate(chains):
        ident = "%016x" % (0x0123456789abcdef + index)
        data["capabilities"].append({"key": key.hex(), "id": ident, "caveats": chain, "token": mint(key, chain, ident)})
    for root in ["/work/semio", "/w", "/ünï/çode", "/a/b c/d"]:
        data["endpoints"].append({"root": root, "id": hashlib.sha256(("semio-repo-service-v1\0" + root).encode()).hexdigest()[:12]})
    for name, samples in SAMPLES.items():
        v = validator(name)
        for sample in samples:
            for case in [sample] + mutations(sample, rng):
                data["frames"].append({"definition": name, "value": case, "valid": v.is_valid(case)})
    json.dump(data, open(out, "w", encoding="utf-8"), ensure_ascii=False)
    print(len(data["frames"]), "frame cases,", sum(1 for f in data["frames"] if f["valid"]), "valid")


def digest(root, paths):
    lines = []
    for path in sorted(paths, key=lambda p: p.encode("utf-8")):
        lines.append(path + "\0" + hashlib.sha256(open(os.path.join(root, path), "rb").read()).hexdigest() + "\n")
    print(hashlib.sha256("".join(lines).encode("utf-8")).hexdigest())


if __name__ == "__main__":
    if sys.argv[1] == "fixtures":
        fixtures(sys.argv[2])
    elif sys.argv[1] == "digest":
        digest(sys.argv[2], sys.argv[3:])

#!/usr/bin/env python3
"""🐍️ Independent reference of the agent-session derivation (python3 oracle.py <fixture.json>): prints the derived rows as JSON. Shares no code with the TypeScript module; only the authored policy is read from the schema."""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
schema = json.load(open(os.path.join(HERE, "..", "🧬️schema", "🔣️.json"), encoding="utf8"))
P = schema["definitions"]["Policy"]["const"]
I = P["identity"]
DOMAIN = I["domain"]


def digest(*parts):
    return hashlib.sha256((DOMAIN + "\0" + "\0".join(str(p) for p in parts)).encode("utf8")).hexdigest()


def base36(n):
    digits = "0123456789abcdefghijklmnopqrstuvwxyz"
    out = ""
    while n:
        n, r = divmod(n, 36)
        out = digits[r] + out
    return out or "0"


def sanitize(raw):
    return re.sub(r"[^A-Za-z0-9._-]", "_", raw)[:64]


def session_id(agent):
    if agent == I["defaultAgent"]:
        return I["defaultSession"]
    return "a" + base36(int(digest("session", agent)[:12], 16))[:10]


def client_of(text, fallback):
    lower = text.lower()
    for row in I["clients"]:
        if row["id"] in lower:
            return row["id"]
    return fallback


def client_process(command):
    text = command.strip()
    if text.startswith('"') and text.find('"', 1) > 0:
        end = text.index('"', 1)
        tokens = [text[1:end]] + text[end + 1:].split()
    else:
        tokens = text.split()
    if not tokens:
        return None

    def base(path):
        name = re.split(r"[\\/]", path)[-1].lower()
        return name[:-4] if name.endswith(".exe") else name

    exe = tokens[0]
    if base(exe) in I["interpreters"]:
        rest = [t for t in tokens[1:] if t and not t.startswith("-")]
        if not rest:
            return None
        subject = rest[0]
    else:
        subject = base(exe)
    m = re.search(I["clientProcessPattern"], subject, re.I)
    return client_of(m.group(1), "human") if m else None


def identity(seed):
    if "agent" in seed:
        agent = sanitize(seed["agent"])
        if agent == "":
            return {"error": "invalidAgent"}
        if seed.get("origin") == "service":
            return {"agent": agent, "client": "service", "origin": "service"}
        client = client_of(seed.get("client", ""), "human" if "client" not in seed else "mcp")
        return {"agent": agent, "client": client, "origin": "explicit"}
    for row in I["clients"]:
        name = row.get("environment")
        value = seed["environment"].get(name) if name else None
        if value and sanitize(value) != "":
            return {"agent": sanitize(value), "client": row["id"], "origin": "environment"}
    c = seed.get("connection")
    if c:
        name = sanitize(c["client"].lower())[: I["connectionNameCharacters"]] or "agent"
        h = digest("connection", c["client"], c["clientProcess"], c["process"])[: I["digestCharacters"]]
        return {"agent": f"{name}-{h}", "client": client_of(c["client"], "mcp"), "origin": "connection"}
    for a in seed["ancestry"]:
        cl = client_process(a["command"])
        if cl:
            h = digest("process", a["pid"], a["started"])[: I["digestCharacters"]]
            return {"agent": f"{cl}-{h}", "client": cl, "origin": "process", "anchor": {"pid": a["pid"], "started": a["started"]}}
    return {"agent": I["defaultAgent"], "client": "human", "origin": "default"}


def derive(seed):
    ident = identity(seed)
    if "error" in ident:
        return ident
    sid = session_id(ident["agent"])
    row = {
        "agent": ident["agent"], "client": ident["client"], "origin": ident["origin"], "session": sid,
        "lane": sid,
        "testsRoot": sid[: P["resources"]["testsRootCharacters"]],
        "bazel": int(digest("bazel", sid)[:12], 16) % P["resources"]["bazelPreferenceModulus"],
        "nx": digest("nx", sid)[: P["resources"]["nxNamespaceCharacters"]],
        "leaseOwner": ident["agent"][: P["resources"]["leaseOwnerAgentCharacters"]] + "/" + sid,
    }
    anchor = ident.get("anchor") or ({"pid": seed["ownerPid"], "started": ""} if "ownerPid" in seed else None)
    if anchor:
        row["anchor"] = anchor
    return row


def state(f):
    L = P["liveness"]
    if f["isDefault"] or f["ageSeconds"] <= L["idleSeconds"]:
        return "live"
    if (f["anchorAlive"] and f["ageSeconds"] <= L["anchoredSeconds"]) or f["ageSeconds"] <= L["collectSeconds"]:
        return "idle"
    return "gone"


def shares(rows):
    whole = P["resources"]["shareBasisPoints"]
    total = sum(r["weight"] for r in rows)
    if total == 0:
        return {}
    parts = [{"id": r["id"], "floor": r["weight"] * whole // total, "rest": (r["weight"] * whole) % total} for r in rows]
    left = whole - sum(p["floor"] for p in parts)
    for p in sorted(parts, key=lambda p: (-p["rest"], p["id"])):
        if left > 0 and p["rest"] > 0:
            p["floor"] += 1
            left -= 1
    return {p["id"]: p["floor"] for p in parts}


fixture = json.load(open(sys.argv[1], encoding="utf8"))
print(json.dumps({
    "identity": [derive(r["seed"]) for r in fixture["identity"]],
    "clientProcess": [client_process(r["command"]) for r in fixture["clientProcess"]],
    "state": [state(r["facts"]) for r in fixture["state"]],
    "shares": [shares(r["rows"]) for r in fixture["shares"]],
}, ensure_ascii=False))

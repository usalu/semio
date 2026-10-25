#!/usr/bin/env python3
"""Independent Python model of the contributors module: identity lines (a regular expression instead of the hand written
scanner), author lines, alias resolution over the real contributor documents, the checkpoint log, session kinds and ids.

Writes (or with --check verifies) 🧪️tests/🧫️fixtures/🔬️unit-vectors.json of the contributors module.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
REPO = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules"
MODULE = REPO / "🧑️contributors"
FIXTURES = MODULE / "🧪️tests/🧫️fixtures"
ORACLE_SCRIPT = str(Path(__file__).resolve().relative_to(ROOT))
TARGET = FIXTURES / "🔬️unit-vectors.json"
EMOJIS = json.loads((REPO / "🪪️identity/🧬️schema/🔣️entity-emojis.json").read_text(encoding="utf8"))
RUNNING_WINDOW = 30 * 60


def simple_lower(text):
    return "".join(c.lower() if len(c.lower()) == 1 else c for c in text)


def emoji_text(emoji):
    base = emoji.replace("︎", "").replace("️", "")
    for text_default in EMOJIS["textDefaultEmojis"]:
        if text_default in base:
            return base.replace(text_default, text_default + "️")
    return base


def flat(text):
    return simple_lower("".join(c for c in text if (c.isascii() and c.isalnum()) or ord(c) > 0x7F))


def entity(name):
    return EMOJIS["entities"][name]


def segment(name, value):
    return emoji_text(entity(name)) + value


IDENTITY = re.compile(r"(?<![0-9])[0-9]{4}(?![0-9])\s+([^<]*)<([^>]*)>")


def identity(line):
    for position in range(len(line)):
        match = IDENTITY.match(line, position)
        if match is None:
            continue
        name, email = match.group(1), match.group(2)
        if name.strip() == "" or email == "":
            continue
        return [name.strip(), email.strip()]
    return None


def git_author(line):
    index = line.find(" <")
    if index < 0:
        return {"name": line, "email": ""}
    rest = line[index + 2:]
    return {"name": line[:index].strip(), "email": rest[:-1] if rest.endswith(">") else rest}


def contributors(documents):
    people = []
    for entry in sorted(documents, key=lambda d: d["directory"].encode("utf8")):
        value = json.loads(entry["json"])
        value.setdefault("alias", "")
        value.setdefault("github", "")
        alias = value["alias"] or entry["directory"]
        github = value["github"] or entry["directory"]
        people.append({"alias": alias, "github": github, "name": value.get("name", ""), "names": value.get("names", []), "email": value.get("email", ""), "emails": value.get("emails", [])})
    return people


def resolve(people, name, email):
    for person in people:
        if email != "" and any(simple_lower(c) == simple_lower(email) for c in [person["email"]] + person["emails"]):
            return person["alias"]
        if name != "" and any(simple_lower(c) == simple_lower(name) for c in [person["name"]] + person["names"]):
            return person["alias"]
    if name and email:
        return f"{name} <{email}>"
    return name or email


def find(people, line):
    parsed = git_author(line)
    if parsed["name"] == "" and parsed["email"] == "":
        return "unknown"
    return resolve(people, parsed["name"], parsed["email"])


def checkpoints(log):
    rows = []
    for line in log.split("\n"):
        parts = line.split("|")
        if len(parts) < 4:
            continue
        rows.append({"id": parts[0], "sha": parts[0], "title": "|".join(parts[3:]), "authorId": parts[1] or None, "date": parts[2]})
    return rows


def checkpoint_id(row):
    contributor = segment("contributor", flat(row["authorId"])) if row["authorId"] else ""
    return contributor + segment("checkpoint", row["sha"])


def derive_kind(session):
    files = session["files"]
    age = session["ageSeconds"]
    meta = None
    if "session.json" in files:
        try:
            meta = json.loads(files["session.json"])
        except ValueError:
            meta = None
        if isinstance(meta, dict):
            events = meta.get("events", [])
            if any(isinstance(e.get("event"), dict) and e["event"].get("kind") == "agent.ended" for e in events):
                return "completed"
            if age is not None and age < RUNNING_WINDOW:
                return "running"
            if events:
                return "interrupted"
    names = sorted(files)
    if any("agent-ended" in n for n in names):
        return "completed"
    if not names:
        return "interrupted"
    if age is not None and age < RUNNING_WINDOW:
        return "running"
    return "interrupted"


def event_files(session):
    for name in sorted(session["files"]):
        if name == "session.json" or not name.endswith(".json"):
            continue
        try:
            yield json.loads(session["files"][name])
        except ValueError:
            continue


def meta_of(session):
    try:
        value = json.loads(session["files"].get("session.json", "null"))
    except ValueError:
        return None
    return value if isinstance(value, dict) else None


def derive_client(session):
    meta = meta_of(session)
    if meta:
        if meta.get("client"):
            return meta["client"]
        for entry in meta.get("events", []):
            if isinstance(entry.get("event"), dict) and entry["event"].get("client"):
                return entry["event"]["client"]
    for value in event_files(session):
        event = value.get("event") if isinstance(value, dict) else None
        if isinstance(event, dict) and event.get("client"):
            return event["client"]
    return ""


def derive_second(session):
    meta = meta_of(session)
    if meta:
        if meta.get("second"):
            return meta["second"]
        seconds = [e["event"]["second"] for e in meta.get("events", []) if isinstance(e.get("event"), dict) and e["event"].get("second")]
        if seconds:
            return min(seconds)
    seconds = []
    for value in event_files(session):
        event = value.get("event") if isinstance(value, dict) else None
        if isinstance(event, dict) and event.get("second"):
            seconds.append(event["second"])
    return min(seconds) if seconds else ""


def session_id(session, checkpoint):
    if checkpoint:
        parent = segment("checkpoint", checkpoint)
    else:
        parent = segment("year", "%02d" % session["year"]) + segment("month", "%02d" % session["month"]) + segment("day", "%02d" % session["day"])
    return parent + segment("session", flat(session["uuid"]))


def build():
    documents = json.loads((FIXTURES / "🧑️‍💻️contributor-documents.json").read_text(encoding="utf8"))["documents"]
    log = json.loads((FIXTURES / "🏁️checkpoint-log.json").read_text(encoding="utf8"))
    sessions = json.loads((FIXTURES / "📡️session-vectors.json").read_text(encoding="utf8"))["sessions"]
    people = contributors(documents)

    identity_lines = list(log["identities"]) + list(log["malformed"]) + [
        "2026 <a@b>", "2026   <a@b>", "2026 Name <>", "2026 Name   <a@b>  ", "x 2026 Name <a@b> y", "1999 A <b@c> 2026 D <d@e>", "2026 Ünï Çödé <ü@ex.org>", "2026\tTab\t<t@ex.org>", "12026 Name <a@b>",
        "2026Name <a@b>", "2026 Name (x) <a@b>", "2026 Name < b > <c>", "99 Name <a@b>", "0001 First <f@x>", "2026 Name <a@b", "2026 Name a@b>", "   599\tUeli Saluz <ueli@semio-tech.com>",
    ]
    author_lines = [f"{p['name']} <{p['email']}>" for p in people] + [f"{p['name']} <{e}>" for p in people for e in p["emails"]] + [
        "Unknown Person <nobody@example.org>", "UELI SALUZ <UELI@SEMIO-TECH.COM>", "ueli.saluz", "Nobody", "", " <only@email>", "Name <e@x> <y>", "  Padded Name  <e@x>", "Name <e@x", "Name<e@x>",
    ] + [p["name"].upper() for p in people if p["name"]]
    for person in people:
        for alternative in person["names"]:
            author_lines.append(f"{alternative} <>")

    extra_log = log["log"] + "\nabc123|Pipe Author|2026-01-02T03:04:05+00:00|title | with | pipes\nabc124||2026-01-02T03:04:06+00:00|no author\n\nshort|line\nabc125|A|d|"
    rows = checkpoints(extra_log)
    session_rows = []
    for session in sessions:
        meta = meta_of(session)
        checkpoint = meta.get("checkpoint", "") if meta else ""
        session_rows.append({
            "uuid": session["uuid"], "kind": derive_kind(session), "client": derive_client(session), "second": derive_second(session), "checkpoint": checkpoint,
            "id": session_id(session, checkpoint), "uri": "repo://session/" + segment("session", flat(session["uuid"])),
        })
    return {
        "schema": "semio.repo.contributors.unit-vectors/1",
        "$comment": "🔬️ Expected answers for the cargo unit tests, computed by an independent Python model (a regular expression for the identity rule, the standard library otherwise; no code shared with the Rust or Go implementation) over the real contributor documents, the real checkpoint log and the session vectors. Regenerate with the model; never edit by hand.",
        "oracle": {"script": ORACLE_SCRIPT, "tool": "python3", "libraries": ["re", "json"], "model": "identity lines, author lines, alias resolution, checkpoint log, session kinds and ids"},
        "identities": [{"line": line, "parsed": identity(line)} for line in identity_lines],
        "authors": [{"line": line, "name": git_author(line)["name"], "email": git_author(line)["email"], "alias": find(people, line)} for line in author_lines],
        "contributors": people,
        "checkpointLog": extra_log,
        "checkpoints": [dict(row, identifier=checkpoint_id(row)) for row in rows],
        "sessions": session_rows,
    }


def main():
    text = json.dumps(build(), ensure_ascii=False, indent=1) + "\n"
    if "--check" in sys.argv:
        same = TARGET.exists() and TARGET.read_text(encoding="utf8") == text
        print("SAME" if same else "DIFFERENT")
        sys.exit(0 if same else 1)
    TARGET.write_text(text, encoding="utf8")
    print("written", TARGET)


main()

#!/usr/bin/env python3
"""One-time conversion of pre-schema `🎫️ticket.json` documents to the current ticket schema (`🎫️tickets/🧬️schema/🔣️.json` TicketDocument), kept only in this ticket folder.
Output is the exact text of the Go codec's `EncodeTicketDocument` (member order, indentation, HTML escaping, no trailing newline), so a converted file is a fixed point of decode then encode.

Rules (legacy -> current):
  prompt                       -> description (only when description is absent)
  interactions[k] (dates, commit, diff, ...) -> `ticket.open` (k = 0) or `ticket.reopen` at dates.started, with author, system, client, prompt, llm;
                                  when dates.finished exists a `ticket.close` at dates.finished with checkpoint = commit, files = diff.files paths (deleted, modified, added) and, on the last session of a closed ticket, the ticket summary;
                                  without dates.finished the commit becomes the checkpoint of the open record
  sessions as objects          -> one `ticket.reopen` per object entry (date, client, llm, prompt; note -> summary); string entries stay in sessions
  top-level client/llm/files/dates{opened,closed} (single-session shape) -> `ticket.open` at dates.opened and `ticket.close` at dates.closed (summary, files)
  started, finished (ticket level) -> kept as the date of the first `ticket.open` and of the last `ticket.close` (RFC 3339, full precision) whenever they name the same second as the session dates
  agents, parent, any other undeclared member -> dropped (parent is never written by the codec)
Dropped with the interaction diff: line counts, bundle, folder, section and definition statistics (no home in the current Interaction); the originals stay in git history.
usage: tickets.py list <repo-root> | convert <file> | check <repo-root>"""
import json, os, re, subprocess, sys

SCHEMA_ORDER = ["title", "emoji", "status", "description", "summary", "github", "goal", "plan", "sessions", "interactions"]


def go_string(value):
    out = ['"']
    for ch in value:
        o = ord(ch)
        if ch == '"': out.append('\\"')
        elif ch == "\\": out.append("\\\\")
        elif ch == "\n": out.append("\\n")
        elif ch == "\r": out.append("\\r")
        elif ch == "\t": out.append("\\t")
        elif ch == "<": out.append("\\u003c")
        elif ch == ">": out.append("\\u003e")
        elif ch == "&": out.append("\\u0026")
        elif o == 0x2028: out.append("\\u2028")
        elif o == 0x2029: out.append("\\u2029")
        elif o < 0x20: out.append("\\u%04x" % o)
        else: out.append(ch)
    out.append('"')
    return "".join(out)


def render(members, depth):
    if not members:
        return "{}"
    indent, closing, lines = "  " * (depth + 1), "  " * depth, []
    for name, kind, value in members:
        if kind == "text":
            lines.append(indent + go_string(name) + ": " + go_string(value))
        elif kind == "object":
            lines.append(indent + go_string(name) + ": " + render(value, depth + 1))
        elif kind == "strings":
            element = "  " * (depth + 2)
            lines.append(indent + go_string(name) + ": [\n" + ",\n".join(element + go_string(v) for v in value) + "\n" + indent + "]")
        else:
            element = "  " * (depth + 2)
            lines.append(indent + go_string(name) + ": [\n" + ",\n".join(element + render(v, depth + 2) for v in value) + "\n" + indent + "]")
    return "{\n" + ",\n".join(lines) + "\n" + closing + "}"


def interaction_members(ix):
    members = [(k, "text", ix.get(k, "")) for k in ("kind", "date", "author", "system", "client", "checkpoint")]
    members += [(k, "text", ix[k]) for k in ("prompt", "summary", "llm", "effort") if ix.get(k)]
    if ix.get("files"):
        members.append(("files", "objects", [[("path", "text", f["path"])] + [(k, "text", f[k]) for k in ("id", "uri") if f.get(k)] for f in ix["files"]]))
    return members


def encode(doc):
    members = [("title", "text", doc.get("title", ""))]
    if doc.get("emoji"): members.append(("emoji", "text", doc["emoji"]))
    members.append(("status", "text", doc["status"]))
    for key in ("description", "summary"):
        if doc.get(key): members.append((key, "text", doc[key]))
    if doc.get("github") is not None: members.append(("github", "object", [("issue", "text", doc["github"]["issue"])] if doc["github"].get("issue") else []))
    if doc.get("goal"): members.append(("goal", "text", doc["goal"]))
    if doc.get("plan") is not None:
        members.append(("plan", "object", [(k, "text", doc["plan"][k]) for k in ("client", "id", "source", "local") if doc["plan"].get(k)]))
    if doc.get("sessions"): members.append(("sessions", "strings", doc["sessions"]))
    if doc.get("interactions"): members.append(("interactions", "objects", [interaction_members(ix) for ix in doc["interactions"]]))
    return render(members, 0)


def is_legacy(doc):
    if "prompt" in doc or "agents" in doc:
        return True
    sessions = doc.get("sessions")
    if isinstance(sessions, list) and any(not isinstance(s, str) for s in sessions):
        return True
    return any("dates" in ix or "kind" not in ix or isinstance(ix.get("author"), dict) or isinstance(ix.get("system"), dict) for ix in doc.get("interactions", []))


def same_second(rfc3339, stamp):
    return bool(rfc3339) and bool(stamp) and rfc3339[:19].replace("T", " ") == stamp


def diff_files(diff):
    files = (diff or {}).get("files") or {}
    paths = []
    for category in ("deleted", "modified", "added"):
        for entry in files.get(category) or []:
            path = entry if isinstance(entry, str) else entry.get("path", "")
            if path and path not in paths:
                paths.append(path)
    return [{"path": path} for path in paths]


def stamp(value, fallback):
    return value if value and " " in value else ((value + " 00:00:00") if value else fallback)


def convert_document(doc, path):
    out = {key: doc[key] for key in ("title", "emoji", "status", "summary", "github", "goal", "plan") if key in doc and doc[key] not in (None, "")}
    if "github" in doc and doc["github"] is not None: out["github"] = doc["github"]
    description = doc.get("description") or doc.get("prompt") or ""
    if description: out["description"] = description
    sessions = doc.get("sessions")
    session_ids = [s for s in sessions if isinstance(s, str)] if isinstance(sessions, list) else []
    session_logs = [s for s in sessions if isinstance(s, dict)] if isinstance(sessions, list) else []
    if session_ids: out["sessions"] = session_ids
    interactions = []
    closed = doc.get("status") == "closed"
    summary = doc.get("summary", "")
    legacy = doc.get("interactions", [])
    for index, ix in enumerate(legacy):
        if "kind" in ix:
            interactions.append(ix)
            continue
        dates = ix.get("dates", {})
        who = {"author": ix.get("author", ""), "system": ix.get("system", ""), "client": ix.get("client", "")}
        finished = dates.get("finished")
        started = dates.get("started", "")
        if not interactions and same_second(doc.get("started"), started):
            started = doc["started"]
        opened = {"kind": "ticket.open" if not interactions else "ticket.reopen", "date": started, **who, "checkpoint": "" if finished else ix.get("commit", ""), "prompt": ix.get("prompt", ""), "llm": ix.get("llm", "")}
        interactions.append(opened)
        if finished:
            last = index == len(legacy) - 1
            if last and same_second(doc.get("finished"), finished):
                finished = doc["finished"]
            interactions.append({"kind": "ticket.close", "date": finished, **who, "checkpoint": ix.get("commit", ""), "summary": summary if (last and closed) else "", "files": diff_files(ix.get("diff"))})
    if session_logs:
        fallback = path_date(path)
        for entry in session_logs:
            interactions.append({"kind": "ticket.reopen", "date": stamp(entry.get("date"), fallback), "author": "", "system": "", "client": entry.get("client", ""), "checkpoint": "", "prompt": entry.get("prompt", ""), "summary": entry.get("note", ""), "llm": entry.get("llm", "")})
    if not legacy and "dates" in doc and ("client" in doc or "llm" in doc):
        dates = doc["dates"]
        interactions.append({"kind": "ticket.open", "date": stamp(dates.get("opened"), path_date(path)), "author": "", "system": "", "client": doc.get("client", ""), "checkpoint": "", "prompt": doc.get("prompt", ""), "llm": doc.get("llm", "")})
        if closed:
            interactions.append({"kind": "ticket.close", "date": stamp(dates.get("closed"), path_date(path)), "author": "", "system": "", "client": doc.get("client", ""), "checkpoint": "", "summary": summary, "files": [{"path": f} for f in doc.get("files", [])]})
    if interactions: out["interactions"] = interactions
    return out


def path_date(path):
    match = re.search(r"🎆️?(\d{2})/🌙️?(\d{2})/☀️?(\d{2})/", path)
    return "20%s-%s-%s 00:00:00" % match.groups() if match else ""


def convert_text(text, path):
    return encode(convert_document(json.loads(text), path))


def tickets(root):
    base = os.path.join(root, ".🧬semio", "🦑️repo", "🎫️tickets")
    for current, _, names in os.walk(base):
        if "🎫️ticket.json" in names:
            yield os.path.relpath(os.path.join(current, "🎫️ticket.json"), root).replace(os.sep, "/")


def legacy_paths(root):
    found = []
    for path in sorted(tickets(root)):
        try:
            doc = json.load(open(os.path.join(root, path), encoding="utf8"))
        except ValueError:
            continue
        if isinstance(doc, dict) and is_legacy(doc):
            found.append(path)
    return found


if __name__ == "__main__":
    mode, target = sys.argv[1], sys.argv[2]
    if mode == "list":
        print("\n".join(legacy_paths(target)))
    elif mode == "convert":
        print(convert_text(open(target, encoding="utf8").read(), target), end="")
    elif mode == "check":
        same = other = 0
        for path in sorted(tickets(target)):
            text = open(os.path.join(target, path), encoding="utf8").read()
            try:
                doc = json.loads(text)
            except ValueError:
                continue
            if not isinstance(doc, dict) or doc.get("status") not in ("open", "closed") or is_legacy(doc) or set(doc) - set(SCHEMA_ORDER) - {"parent"}:
                continue
            if encode(doc) == text: same += 1
            else: other += 1
        print("renderer fixed points among schema-only documents: %d identical, %d different" % (same, other))

#!/usr/bin/env python3
"""Independent Python model of the goals identifier scheme and goal tree.

Writes (or with --check verifies) 🧪️tests/🧫️fixtures/🔬️unit-vectors.json of the goals module.
The model shares no code with the Rust or Go implementation: regular expressions and the Python
standard library only.  Go's strings.ToUpper/ToLower are the *simple* case mappings, emulated per code point.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
MODULE = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals"
ORACLE_SCRIPT = str(Path(__file__).resolve().relative_to(ROOT))
FIXTURES = MODULE / "🧪️tests/🧫️fixtures"

GOAL = "\U0001F3AF"
TICKET = "\U0001F3AB"


def simple_upper(text):
    return "".join(c.upper() if len(c.upper()) == 1 else c for c in text)


def simple_lower(text):
    return "".join(c.lower() if len(c.lower()) == 1 else c for c in text)


def slugify(text):
    spaced = re.sub(r"(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", "-", text)
    return re.sub(r"[^A-Z0-9]+", "-", simple_upper(spaced)).strip("-")


def flat(text):
    return simple_lower("".join(c for c in text if c.isascii() and c.isalnum() or ord(c) > 0x7F))


def to_compose(path):
    return "" if path == "" else "".join(GOAL + flat(part) for part in path.split("/"))


def compose_to_path(compose):
    if compose == "":
        return ""
    trimmed = compose[len(GOAL):] if compose.startswith(GOAL) else compose
    return "/".join(part for part in trimmed.split(GOAL) if part)


def filesystem(goal_id):
    goal_id = goal_id.strip()
    if goal_id == "":
        return ""
    if "/" in goal_id:
        return goal_id
    if GOAL in goal_id:
        path = compose_to_path(goal_id)
        if path and path != goal_id:
            return path
    return goal_id


def parent_of(goal_id):
    goal_id = filesystem(goal_id)
    return goal_id[: goal_id.rindex("/")] if "/" in goal_id else ""


def root_of(goal_id):
    goal_id = filesystem(goal_id)
    return goal_id.split("/", 1)[0]


def trailing_number(value):
    for candidate in (value, value.split("/")[-1]):
        if re.fullmatch(r"[+-]?[0-9]+", candidate):
            return int(candidate)
    return None


def id_answers(vectors):
    paths = []
    for path in vectors["paths"]:
        depth = path.count("/")
        composed = to_compose(path)
        paths.append({
            "path": path, "depth": depth, "root": depth == 0, "firstGen": depth == 1, "deeper": depth >= 2,
            "filesystem": filesystem(path), "rootId": root_of(path), "parentId": parent_of(path),
            "compose": composed, "composeBack": compose_to_path(composed),
        })
    titles = []
    for vector in vectors["titles"]:
        slug = slugify(vector["title"])
        titles.append({"parent": vector["parent"], "title": vector["title"], "id": slug if vector["parent"] == "" else vector["parent"] + "/" + slug})
    references = [{"reference": r, "milestone": trailing_number(r), "issue": trailing_number(r)} for r in vectors["references"]]
    return paths, titles, references


def sort_key_nodes(nodes):
    def cmp(left, right):
        if left["dueDate"] != right["dueDate"]:
            if left["dueDate"] == "":
                return 1
            if right["dueDate"] == "":
                return -1
            return -1 if left["dueDate"] < right["dueDate"] else 1
        return (left["id"] > right["id"]) - (left["id"] < right["id"])

    import functools
    nodes.sort(key=functools.cmp_to_key(cmp))
    for node in nodes:
        sort_key_nodes(node["children"])


def nest(tickets):
    ids = [t["id"] for t in tickets]
    by_id = {t["id"]: dict(t, children=[]) for t in tickets}
    roots = []
    for t in tickets:
        if t["parent"] != "" and t["parent"] in ids and t["parent"] != t["id"]:
            by_id[t["parent"]]["children"].append(by_id[t["id"]])
        else:
            roots.append(by_id[t["id"]])
    return roots


def build(goals, tickets):
    nodes = {g["id"]: dict(g, children=[], tickets=[]) for g in goals}
    known = sorted(nodes)
    ticket_nodes = []
    for t in tickets:
        ticket_nodes.append(dict(t, uri="repo://ticket/" + TICKET + flat(t["slug"])))
    per_goal, orphans = {}, []
    for t in ticket_nodes:
        if t["goal"] != "" and t["goal"] in known:
            per_goal.setdefault(t["goal"], []).append(t)
        else:
            orphans.append(t)
    for goal_id, owned in per_goal.items():
        nodes[goal_id]["tickets"] = nest(owned)
    roots = []
    for goal_id in known:
        parent = goal_id[: goal_id.rindex("/")] if "/" in goal_id else ""
        if parent and parent in known:
            nodes[parent]["children"].append(nodes[goal_id])
        else:
            roots.append(nodes[goal_id])
    sort_key_nodes(roots)
    if orphans:
        roots.append({"id": "", "title": "No Goal", "status": "", "dueDate": "", "children": [], "tickets": nest(orphans)})
    return roots


def goal_line(node):
    return "goal %s %s [%s] %s" % (node["id"], node["title"], node["status"], node["dueDate"])


def ticket_line(node):
    return "ticket %s %s [%s] %s" % (node["slug"], node["title"], node["status"], node["uri"])


def render(roots, markdown):
    out = []

    def line(prefix, content, is_last, is_root):
        if markdown:
            out.append(prefix + "- " + content)
            return prefix + "  "
        connector = "" if is_root else ("└️─️─️ " if is_last else "├️─️─️ ")
        out.append(prefix + connector + content)
        if is_root:
            return prefix
        return prefix + "    " if is_last else prefix + "│️   "

    def ticket(node, prefix, is_last):
        nxt = line(prefix, ticket_line(node), is_last, False)
        for i, child in enumerate(node["children"]):
            ticket(child, nxt, i == len(node["children"]) - 1)

    def goal(node, prefix, is_last, is_root):
        nxt = line(prefix, goal_line(node), is_last, is_root)
        total = len(node["children"]) + len(node["tickets"])
        for i, child in enumerate(node["children"]):
            goal(child, nxt, i == total - 1, False)
        for i, t in enumerate(node["tickets"]):
            ticket(t, nxt, len(node["children"]) + i == total - 1)

    for i, root in enumerate(roots):
        goal(root, "", i == len(roots) - 1, True)
    return "".join(l + "\n" for l in out)


def open_subgoals(node):
    return sum((1 if c["status"] == "open" else 0) + open_subgoals(c) for c in node["children"])


def open_tickets(node):
    def tickets(items):
        return sum((1 if t["status"] == "open" else 0) + tickets(t["children"]) for t in items)

    return tickets(node["tickets"]) + sum(open_tickets(c) for c in node["children"])


def counts(roots):
    out = []

    def walk(node):
        out.append({"id": node["id"], "openSubgoals": open_subgoals(node), "openTickets": open_tickets(node)})
        for c in node["children"]:
            walk(c)

    for r in roots:
        walk(r)
    return out


def main():
    ids = json.loads((FIXTURES / "🪪️id-vectors.json").read_text(encoding="utf8"))
    tree = json.loads((FIXTURES / "🌳️tree-vectors.json").read_text(encoding="utf8"))
    paths, titles, references = id_answers(ids)
    roots = build(tree["goals"], tree["tickets"])
    result = {
        "schema": "semio.repo.goals.unit-vectors/1",
        "$comment": "🔬️ Expected answers for the cargo unit tests, computed by an independent Python model (regular expressions and the standard library, no code shared with the Rust or Go implementation) over the inputs of 🪪️id-vectors.json and 🌳️tree-vectors.json. Regenerate with the model; never edit by hand.",
        "oracle": {"script": ORACLE_SCRIPT, "tool": "python3", "libraries": ["re", "json"], "model": "goals identifier scheme, goal forest and its plain-line rendering"},
        "paths": paths,
        "titles": titles,
        "references": references,
        "tree": {"text": render(roots, False), "markdown": render(roots, True), "counts": counts(roots)},
    }
    text = json.dumps(result, ensure_ascii=False, indent=2) + "\n"
    target = FIXTURES / "🔬️unit-vectors.json"
    if "--check" in sys.argv:
        if not target.exists() or target.read_text(encoding="utf8") != text:
            print("DIFFERENT")
            sys.exit(1)
        print("SAME")
    else:
        target.write_text(text, encoding="utf8")
        print("written", target)


main()

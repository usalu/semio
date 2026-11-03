"""Wave 2 features v2: makes the remaining prose `🥒️.feature` files of the shared crates, app surface and artifact scenes parse under the restricted profile.
- Features discovered as cases (their directory is `🧪️tests/<case>/`): the scenarios move into the feature description as "Further specification assertions" (as in v1); no scenario without an adapter stays.
- Features directly in a `🧪️tests` directory (not discovered): every scenario gets `@id-`, `@level-quick`, `@mode-conformance` and every Examples row an `id` cell, so they parse clean until a case owner promotes them to `🧪️tests/<case>/`.
Usage: .venv/bin/python -I wave2-features-v2-normalise-prose.py <feature> ..."""
import re
import sys
from pathlib import Path


def kebab(text):
    return re.sub(r"[^a-z0-9]+", "-", re.sub(r"[<>\"']", "", text).lower()).strip("-")[:60].strip("-")


def parse(text):
    lines = text.splitlines()
    feature = next(i for i, l in enumerate(lines) if l.startswith("Feature:"))
    head_tags = [t for l in lines[:feature] for t in l.split() if t.startswith("@")]
    first = next((i for i, l in enumerate(lines) if i > feature and re.match(r"\s*(@|Scenario)", l)), len(lines))
    description = lines[feature + 1:first]
    blocks, current, pending = [], None, []
    for l in lines[first:]:
        if re.match(r"\s*@", l):
            pending.extend(l.split())
        elif re.match(r"\s*Scenario", l):
            current = {"head": l.strip(), "tags": pending, "body": []}
            pending = []
            blocks.append(current)
        elif current is not None:
            current["body"].append(l)
    return head_tags, lines[feature], description, blocks


def keep(tags):
    return [t for t in tags if t.startswith(("@capability-", "@oracle-", "@no-oracle-", "@comparison-"))]


def steps_of(block):
    return " ".join(l.strip() for l in block["body"] if re.match(r"\s*(Given|When|Then|And|But)\b", l))


def render_table(rows):
    widths = [max(len(r[i]) for r in rows) for i in range(len(rows[0]))]
    return ["      | " + " | ".join(c.ljust(w) for c, w in zip(r, widths)) + " |" for r in rows]


def as_description(path):
    tags, feature, description, blocks = parse(path.read_text(encoding="utf8"))
    out = [" ".join(keep(tags)), feature] + [l for l in description if l.strip() or True]
    while out and not out[-1].strip():
        out.pop()
    out += ["", "  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:"]
    out += ["  - %s: %s" % (b["head"].split(":", 1)[1].strip(), steps_of(b)) for b in blocks]
    path.write_text("\n".join(out) + "\n", encoding="utf8")


def as_valid(path):
    tags, feature, description, blocks = parse(path.read_text(encoding="utf8"))
    out = [t for t in keep(tags)]
    out = ["\n".join(out), feature] + description
    while out and not out[-1].strip():
        out.pop()
    seen = {}
    for b in blocks:
        base = kebab(b["head"].split(":", 1)[1]) or "scenario"
        seen[base] = seen.get(base, 0) + 1
        ident = base if seen[base] == 1 else "%s-%d" % (base, seen[base])
        extra = [t for t in b["tags"] if not t.startswith(("@id-", "@level-", "@mode-"))]
        out += [""] + ["  " + t for t in extra] + ["  @id-" + ident, "  @level-quick", "  @mode-conformance", "  " + b["head"]]
        body = b["body"]
        table = [i for i, l in enumerate(body) if l.strip().startswith("|")]
        if "Outline" in b["head"] and table:
            rows = [[c.strip() for c in body[i].strip().strip("|").split("|")] for i in table]
            if rows[0][0] != "id":
                ids = []
                for n, row in enumerate(rows[1:]):
                    key = kebab("-".join(c for c in row if c)) or "row-%d" % (n + 1)
                    ids.append(key if key not in ids else "%s-%d" % (key, n + 1))
                rows = [["id"] + rows[0]] + [[i] + r for i, r in zip(ids, rows[1:])]
            out_body = body[:table[0]] + render_table(rows) + body[table[-1] + 1:]
        else:
            out_body = body
        out += [l for l in out_body]
    path.write_text("\n".join(out).rstrip() + "\n", encoding="utf8")


for arg in sys.argv[1:]:
    path = Path(arg)
    discovered = path.parent.parent.name == "🧪️tests"
    (as_description if discovered else as_valid)(path)

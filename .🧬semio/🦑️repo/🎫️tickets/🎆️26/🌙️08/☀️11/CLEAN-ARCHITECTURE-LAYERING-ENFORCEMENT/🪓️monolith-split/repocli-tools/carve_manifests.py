"""📦 Wave-4 repo CLI carve, manifest step: writes the five producer manifests and project files, and rewrites the parent manifest (dependency sets derived from the carved sources)."""
import os, re, sys, json

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from strip import strip

MODULES = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
CLI = MODULES + "⌨️cli/"
PARENT_TOML = CLI + "📦️packages/🦀️rust/Cargo.toml"

CRATES = [
    dict(key="event_stream", node="📡️event-stream", name="event-stream", src="📡️event-stream/🦀️.rs", deps_cli=[],
         desc="semio repo CLI event stream: the typed event records, the Go-twin number formatting and the NDJSON, human and Markdown renderers every verb result flows through"),
    dict(key="command_framework", node="🧭️repo-cli/🎮️command-framework", name="command-framework", src="🧭️repo-cli/🎮️command-framework/🦀️.rs", deps_cli=["event_stream"],
         desc="semio repo CLI command framework: typed flags and operands, the command declaration, the argv parser, the invocation and its configuration"),
    dict(key="tree_source_and_search", node="🧭️repo-cli/🗂️tree-source-and-search", name="tree-source-and-search", src="🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs", deps_cli=["command_framework", "event_stream"],
         desc="semio repo CLI tree source: the filesystem tree source, the model ids and the list-verb stream options"),
    dict(key="mcp_verb", node="🔌️mcp-verb", name="mcp-verb", src="🔌️mcp-verb/🦀️.rs", deps_cli=["event_stream"],
         desc="semio repo CLI MCP repository: the production repository the repo MCP server and the mcp verb serve"),
    dict(key="verbs", node="🧭️repo-cli/🚦️verbs", name="verbs", src="🧭️repo-cli/🚦️verbs/🦀️.rs", deps_cli=["command_framework", "event_stream", "mcp_verb", "tree_source_and_search"],
         desc="semio repo CLI verbs: one handler per verb, each a thin front for the domain crate that owns the aggregate"),
]
BY_KEY = {c["key"]: c for c in CRATES}
PARENT_SRC = ["🧭️repo-cli/🌳️command-tree/🦀️.rs", "🧭️repo-cli/🚀️run-and-projection/🦀️.rs", "🧭️repo-cli/🦀️.rs", "🦀️.rs", "📦️packages/🦀️rust/📦️cli-main.rs", "📦️packages/🦀️rust/📦️mcp-main.rs"]
PARENT_DEPS_CLI = ["command_framework", "event_stream", "mcp_verb", "tree_source_and_search", "verbs"]


def crate_name(c):
    return "semio-framework-repo-cli-" + c["name"]


def parse_parent():
    toml = open(PARENT_TOML, encoding="utf-8").read()
    rows = {}
    for m in re.finditer(r'^(semio-framework-repo-[a-z-]+) = \{ path = "\.\./\.\./\.\./([^"]+)" \}$', toml, re.M):
        rows[m.group(1)] = m.group(2)
    return toml, rows


def external_deps(files, known):
    found = set()
    for f in files:
        t = strip(open(CLI + f, encoding="utf-8").read())
        for m in re.finditer(r"(?<![A-Za-z0-9_])(semio_framework_repo_[a-z_]+?)(?=::|\s|;|,|\)|\}|$)", t):
            n = m.group(1).replace("_", "-")
            if n in known:
                found.add(n)
        if re.search(r"(?<![A-Za-z0-9_])serde_json(?![A-Za-z0-9_])", t):
            found.add("serde_json")
        if re.search(r"(?<![A-Za-z0-9_])serde::|#\[serde|derive\([^)]*(Serialize|Deserialize)", t):
            found.add("serde")
    return found


def child_toml(c, rows, deps):
    ups = "/".join([".."] * (len(c["node"].split("/")) + 7))
    depth_to_modules = len(c["node"].split("/")) + 3
    lines = [
        "[package]",
        f'workspace = "{ups}"',
        f'name = "{crate_name(c)}"',
        'version = "0.1.0"',
        'edition = "2021"',
        "rust-version.workspace = true",
        f'description = "{c["desc"]}"',
        "",
        "[package.metadata.semio]",
        'role = "library"',
        f'id = "repo-cli-{c["name"]}"',
        "",
        "[package.metadata.semio.rust-source-boundary]",
        "maxSourceBytes = 262144",
        "maxSourceFiles = 32",
        "maxActiveLines = 4000",
        "",
        "[lints]",
        "workspace = true",
        "",
        "[lib]",
        'path = "../../🦀️.rs"',
        "",
        "[dependencies]",
    ]
    entries = []
    for d in sorted(deps):
        if d in ("serde", "serde_json"):
            entries.append((d, f"{d} = {{ workspace = true }}"))
        elif d.startswith("semio-framework-repo-cli-"):
            other = next(x for x in CRATES if crate_name(x) == d)
            rel = os.path.relpath(CLI + other["node"] + "/📦️packages/🦀️rust", CLI + c["node"] + "/📦️packages/🦀️rust")
            entries.append((d, f'{d} = {{ path = "{rel}" }}'))
        else:
            rel = "../" * depth_to_modules + rows[d]
            entries.append((d, f'{d} = {{ path = "{rel}" }}'))
    lines += [e[1] for e in entries]
    lines += ["", "[dev-dependencies]", ""]
    return "\n".join(lines)


def main():
    toml, rows = parse_parent()
    known = set(rows)
    plan = {}
    for c in CRATES:
        deps = external_deps([c["src"]], known)
        if c["key"] == "event_stream":
            assert "semio-framework-repo-cli-event-stream" not in deps
        deps |= {crate_name(BY_KEY[k]) for k in c["deps_cli"]}
        for k in c["deps_cli"]:
            assert BY_KEY[k]["key"] != c["key"]
        plan[c["key"]] = deps
    pdeps = external_deps(PARENT_SRC, known)
    pdeps |= {crate_name(BY_KEY[k]) for k in PARENT_DEPS_CLI}
    for c in CRATES:
        base = CLI + c["node"] + "/📦️packages/🦀️rust/"
        text = child_toml(c, rows, plan[c["key"]])
        proj = json.dumps({"name": f"@semio-tech/repo-cli-{c['name']}-rs", "tags": ["scope:framework", "language:rust"]}, ensure_ascii=False, indent=2) + "\n"
        print(c["key"], sorted(plan[c["key"]]))
        if "--write" in sys.argv:
            os.makedirs(base, exist_ok=True)
            assert not os.path.exists(base + "Cargo.toml") or "--force" in sys.argv
            open(base + "Cargo.toml", "w", encoding="utf-8").write(text)
            open(base + "📋️project.json", "w", encoding="utf-8").write(proj)
    print("parent", sorted(pdeps))
    if "--write" in sys.argv:
        head = toml[: toml.index("[dependencies]")]
        out = ["[dependencies]"]
        ent = []
        for d in sorted(pdeps):
            if d in ("serde", "serde_json"):
                ent.append(f"{d} = {{ workspace = true }}")
            elif d.startswith("semio-framework-repo-cli-"):
                other = next(x for x in CRATES if crate_name(x) == d)
                rel = os.path.relpath(CLI + other["node"] + "/📦️packages/🦀️rust", CLI + "📦️packages/🦀️rust")
                ent.append(f'{d} = {{ path = "{rel}" }}')
            else:
                ent.append(f'{d} = {{ path = "../../../{rows[d]}" }}')
        open(PARENT_TOML, "w", encoding="utf-8").write(head + "\n".join(out + ent) + "\n")


main()

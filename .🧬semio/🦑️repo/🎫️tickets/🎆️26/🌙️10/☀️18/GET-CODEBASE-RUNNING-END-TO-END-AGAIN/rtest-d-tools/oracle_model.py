#!/usr/bin/env python3
"""Independent Python model of the repo model's slug vocabulary, kind derivations and section/definition ids.

Writes (or with --check verifies) 🧪️tests/🧫️fixtures/🔬️unit-vectors.json of the model module.  Regular expressions and
the standard library only; Go's simple case mappings are emulated per code point.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
REPO = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules"
MODULE = REPO / "📐️model"
FIXTURES = MODULE / "🧪️tests/🧫️fixtures"
ORACLE_SCRIPT = str(Path(__file__).resolve().relative_to(ROOT))
ALLOWED = json.loads((MODULE / "🧬️schema/🔣️allowed-values.json").read_text(encoding="utf8"))
EMOJIS = json.loads((REPO / "🪪️identity/🧬️schema/🔣️entity-emojis.json").read_text(encoding="utf8"))
TEXT_DEFAULT = EMOJIS["textDefaultEmojis"]


def simple_upper(text):
    return "".join(c.upper() if len(c.upper()) == 1 else c for c in text)


def simple_lower(text):
    return "".join(c.lower() if len(c.lower()) == 1 else c for c in text)


def slugify(text):
    spaced = re.sub(r"(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", "-", text)
    return re.sub(r"[^A-Z0-9]+", "-", simple_upper(spaced)).strip("-")


def normalise(text):
    return simple_lower(slugify(text))


def resolve(field, text):
    if field == "effort" and text.strip() == "":
        return ""
    slug = normalise(text)
    best = ""
    for candidate in ALLOWED[field + "s"]:
        if normalise(candidate) in slug and len(candidate) > len(best):
            best = candidate
    return best if best else None


def emoji_text(emoji):
    base = emoji.replace("︎", "").replace("️", "")
    for text_default in TEXT_DEFAULT:
        if text_default in base:
            return base.replace(text_default, text_default + "️")
    return base


def flat(text):
    return simple_lower("".join(c for c in text if (c.isascii() and c.isalnum()) or ord(c) > 0x7F))


def definition_kind(raw):
    key = simple_lower(raw)
    if key in {"interface", "type", "trait", "abstract", "extend type", "extend interface", "extend enum", "extend union", "extend input", "input", "union", "scalar", "delegate", "record"}:
        return "interface"
    if key in {"const", "constant", "enum", "var", "let", "static"}:
        return "constant"
    return "test" if key == "test" else "implementation"


SCRIPT = {".sh", ".bash", ".zsh", ".fish", ".bat", ".cmd", ".ps1", ".psm1"}
CONFIG = {".json", ".yaml", ".yml", ".toml", ".xml", ".ini", ".conf", ".config", ".env", ".properties", ".cfg", ".hcl", ".tf", ".tfvars", ".gitignore", ".gitattributes", ".gitmodules", ".dockerignore", ".editorconfig", ".eslintignore", ".prettierignore", ".npmignore", ".browserslistrc"}
DOCS = {".md", ".txt", ".rst", ".adoc", ".asciidoc", ".org", ".rdoc"}
TEMPLATE = {".tpl", ".tmpl", ".gotmpl", ".mustache", ".hbs", ".handlebars", ".ejs", ".njk", ".nunjucks", ".jinja", ".jinja2", ".j2", ".liquid", ".pug", ".jade", ".slim", ".haml"}
RESOURCE = {".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico", ".webp", ".bmp", ".tiff", ".tif", ".ttf", ".woff", ".woff2", ".eot", ".otf", ".mp3", ".mp4", ".wav", ".ogg", ".webm", ".avi", ".mov", ".pdf", ".zip", ".tar", ".gz", ".bz2", ".xz", ".7z", ".rar", ".bin", ".dat", ".db", ".sqlite", ".sqlite3", ".wasm", ".map"}
CODE = {".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".mts", ".cts", ".py", ".pyi", ".pyw", ".cs", ".csx", ".go", ".c", ".cpp", ".cc", ".cxx", ".h", ".hpp", ".hxx", ".rs", ".java", ".kt", ".kts", ".scala", ".swift", ".m", ".mm", ".rb", ".erb", ".php", ".lua", ".r", ".jl", ".ex", ".exs", ".hs", ".lhs", ".ml", ".mli", ".clj", ".cljs", ".cljc", ".dart", ".v", ".sv", ".vhd", ".vhdl", ".zig", ".nim", ".cr", ".d", ".f", ".f90", ".f95", ".f03", ".pl", ".pm", ".css", ".scss", ".less", ".sass", ".styl", ".html", ".htm", ".vue", ".svelte", ".astro", ".sql", ".graphql", ".gql", ".proto", ".thrift", ".avsc", ".g4"}
NAMED_CONFIG = {"makefile", "justfile", "rakefile", "gemfile", "vagrantfile", "procfile", "brewfile", "cmakelists.txt", ".babelrc", ".eslintrc", ".prettierrc", ".stylelintrc", ".nycrc", ".npmrc", ".nvmrc", ".yarnrc", ".ruby-version", ".python-version", ".node-version", ".tool-versions", ".cursorrules", ".clinerules", "nx.json", "tsconfig.json", "tslint.json", "composer.json", "cargo.lock", "package-lock.json", "yarn.lock", "pnpm-lock.yaml", "go.sum", "uv.lock"}


def file_kind(name):
    idx = name.rfind(".")
    ext = simple_lower(name[idx:]) if idx >= 0 else ""
    lower = simple_lower(name)
    bare = lower[: len(lower) - len(ext)] if ext and lower.endswith(ext) else lower
    if "license" in lower or "licence" in lower:
        return "license"
    if any(bare.endswith(s) for s in (".test", ".spec", ".tests", ".requirements", "_test", "_tests", "_spec", "_benchmark", ".benchmark", ".stories", ".story")):
        return "lab"
    if lower.startswith("test_") or lower.startswith("test.") or lower == "conftest.py":
        return "lab"
    if any(bare.endswith(s) for s in (".config", ".conf", ".rc", ".cfg")):
        return "config"
    for kind, table in (("script", SCRIPT), ("config", CONFIG), ("docs", DOCS), ("template", TEMPLATE), ("resource", RESOURCE), ("code", CODE)):
        if ext in table:
            return kind
    if lower.startswith("dockerfile") or lower in NAMED_CONFIG:
        return "config"
    return "resource"


def technology_kind(name):
    if name == "compose":
        return "user"
    if name == "repo":
        return "infrastructure"
    if name == "coda":
        return "research"
    if name.startswith("@"):
        return technology_kind(name[1:])
    return "user"


def is_test_function_name(name):
    for prefix in ("Test", "Benchmark", "Fuzz"):
        if name.startswith(prefix) and len(name) > len(prefix) and "A" <= name[len(prefix)] <= "Z":
            return True
    return name.startswith("test_")


FILE_NAMES = [
    "main.rs", "index.ts", "App.tsx", "script.py", "tool.GO", "build.sh", "run.ps1", "setup.BAT", "package.json", "Cargo.toml", "ci.yml", "settings.YAML", "README.md", "notes.txt",
    "guide.rst", "page.hbs", "mail.j2", "logo.png", "font.woff2", "bundle.wasm", "archive.tar", "Makefile", "Dockerfile", "Dockerfile.dev", "LICENSE", "LICENSE.md", "licence.txt",
    "widget.test.ts", "widget.spec.js", "api_test.go", "api_tests.py", "bench_spec.rb", "perf.benchmark.ts", "button.stories.tsx", "test_models.py", "test.json", "conftest.py",
    "vite.config.ts", "tool.conf", "app.rc", "app.cfg", ".gitignore", ".editorconfig", ".babelrc", ".nvmrc", "nx.json", "Cargo.lock", "pnpm-lock.yaml", "styles.css", "query.sql",
    "schema.graphql", "service.proto", "unknown.xyz", "noextension", "archive.tar.gz", "a.b.c.d", "UPPER.RS", "Test.java", "x.test", ".env", "justfile", "image.SVG",
]
KEYWORDS_EXTRA = ["Interface", "TYPE", "Trait", "EXTEND TYPE", "Const", "ENUM", "Test", "TEST", "fn", "macro", ""]
TECH_NAMES = ["compose", "repo", "coda", "@compose", "@repo", "@coda", "@@repo", "@unknown", "semio", "", "@", "Repo"]
FUNCTION_NAMES = ["TestFoo", "Testfoo", "Test", "BenchmarkSort", "FuzzParse", "Fuzzy", "test_foo", "testFoo", "TestÄpfel", "helper", "TEST_UPPER"]
SECTION_CASES = [
    {"fileId": "FILE", "path": []},
    {"fileId": "FILE", "path": ["", ""]},
    {"fileId": "FILE", "path": ["Intro"]},
    {"fileId": "FILE", "path": ["Intro", "Deep Section", "Third-Level"]},
    {"fileId": "FILE", "path": ["\U0001F4A1Idea", "Plain"]},
    {"fileId": "FILE", "path": ["\U0001F4A1️Idea"]},
    {"fileId": "FILE", "path": ["Ünï Cödé 42"]},
]
DEFINITION_CASES = [
    {"fileId": "F", "path": [], "name": "Main", "kind": "implementation"},
    {"fileId": "F", "path": ["S"], "name": "Alpha Beta", "kind": "interface"},
    {"fileId": "F", "path": [], "name": "LIMIT", "kind": "constant"},
    {"fileId": "F", "path": [], "name": "TestThing", "kind": "implementation"},
    {"fileId": "F\U0001F97C", "path": [], "name": "TestThing", "kind": "implementation"},
    {"fileId": "F\U0001F97C", "path": [], "name": "test_thing", "kind": "implementation"},
    {"fileId": "F\U0001F97C", "path": [], "name": "helper", "kind": "implementation"},
    {"fileId": "F\U0001F97C", "path": [], "name": "TestThing", "kind": "interface"},
    {"fileId": "F\U0001F97C", "path": ["Sec"], "name": "BenchmarkIt", "kind": "implementation"},
]


def section_id(file_id, path):
    result = file_id
    for segment in (s for s in path if s):
        m = re.match(r"^([\U0001F300-\U0001FAFF☀-➿]️?)(.*)$", segment, re.S)
        emoji, name = (m.group(1), m.group(2)) if m else ("", segment)
        emoji = emoji or EMOJIS["entities"]["section"]
        result += emoji_text(emoji) + flat(name)
    return result


def definition_id(file_id, path, name, kind):
    effective = kind
    if kind == "implementation" and emoji_text(EMOJIS["entities"]["file-lab"]) in file_id and is_test_function_name(name):
        effective = "test"
    emoji = {"interface": "definition-interface", "constant": "definition-constant", "test": "definition-test", "implementation": "definition-impl"}[effective]
    return section_id(file_id, path) + emoji_text(EMOJIS["entities"][emoji]) + flat(name)


def main():
    slugs = json.loads((FIXTURES / "🔤️slug-normalisation/🔣️vectors.json").read_text(encoding="utf8"))
    kinds = json.loads((FIXTURES / "🧬️definition-kind-derivation/🔣️vectors.json").read_text(encoding="utf8"))
    slug_answers = {}
    for field in ("llm", "effort", "client"):
        group = slugs[field]
        slug_answers[field] = {
            "normalise": [{"input": s, "slug": normalise(s)} for s in group["normalise"]],
            "resolve": [{"input": s, "resolved": resolve(field, s)} for s in group["resolve"]],
            "reject": [{"input": s, "resolved": resolve(field, s)} for s in group["reject"]],
        }
    keywords = list(kinds["keywords"]) + KEYWORDS_EXTRA
    result = {
        "schema": "semio.repo.model.unit-vectors/1",
        "$comment": "🔬️ Expected answers for the cargo unit tests, computed by an independent Python model (regular expressions and the standard library; no code shared with the Rust or Go implementation) over the inputs of 🔤️slug-normalisation, 🧬️definition-kind-derivation and the vectors below. `resolved: null` means the input is refused. Regenerate with the model; never edit by hand.",
        "oracle": {"script": ORACLE_SCRIPT, "tool": "python3", "libraries": ["re", "json"], "model": "slug vocabulary, kind derivations, section and definition ids"},
        "slugs": slug_answers,
        "definitionKinds": [{"keyword": k, "kind": definition_kind(k)} for k in keywords],
        "fileKinds": [{"name": n, "kind": file_kind(n)} for n in FILE_NAMES],
        "technologyKinds": [{"name": n, "kind": technology_kind(n)} for n in TECH_NAMES],
        "testFunctionNames": [{"name": n, "test": is_test_function_name(n)} for n in FUNCTION_NAMES],
        "sectionIds": [dict(c, id=section_id(c["fileId"], c["path"])) for c in SECTION_CASES],
        "definitionIds": [dict(c, id=definition_id(c["fileId"], c["path"], c["name"], c["kind"])) for c in DEFINITION_CASES],
    }
    text = json.dumps(result, ensure_ascii=False, indent=2) + "\n"
    target = FIXTURES / "🔬️unit-vectors.json"
    if "--check" in sys.argv:
        same = target.exists() and target.read_text(encoding="utf8") == text
        print("SAME" if same else "DIFFERENT")
        sys.exit(0 if same else 1)
    target.write_text(text, encoding="utf8")
    print("written", target)


main()

#!/usr/bin/env python3
"""Shared helpers of the docs-1020e tools: workspace root, feature probes, fragment rendering (read only, standard library only).

A fragment (staged/en/*.md, staged/de/*.md) may contain three macros that are resolved against the LIVE tree when the edits are generated and
checked again when they are applied, so a statement about a staged feature can never be applied after the feature went live (or the other way round):
  {{@id}}                  the state phrase of feature `id` ("live" or "staged")
  {{?id::live::staged}}    inline text that depends on the state of `id` (either part may be empty; no nesting, no newline)
  {{^id::today::staged}}   live: the staged text; staged: "today text; staged: staged text" (a trailing full stop of `today` is dropped)
"""
import json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
if not os.path.isdir(os.path.join(ROOT, "🧰️framework")):
    ROOT = "/Users/ueli/Documents/semio"
for index, argument in enumerate(os.sys.argv):
    if argument == "--root" and index + 1 < len(os.sys.argv):
        ROOT = os.path.abspath(os.sys.argv[index + 1])
PRODUCT = "🧰️framework/🛍️products/🦑️repo"
LIB = PRODUCT + "/🔨️modules/📚️library"
PROCESS = "🧰️framework/🔨️modules/🏃️process"
README = os.path.join(ROOT, PRODUCT, "README.md")
for index, argument in enumerate(os.sys.argv):
    if argument == "--readme" and index + 1 < len(os.sys.argv):
        README = os.path.abspath(os.sys.argv[index + 1])
RECORD = os.path.join(HERE, "applied-rehearsal.json" if "--readme" in os.sys.argv else "applied.json")
BACKUP = os.path.join(HERE, "backup")
FORCE_LIVE = {x for x in os.environ.get("DOCS1020E_REHEARSE_LIVE", "").split(",") if x}
CATALOG = os.path.join(ROOT, LIB, "🏗️build/⚙️operations/🔣️catalog.json")
POLICY = os.path.join(ROOT, LIB, "🏗️build/🔣️policy.json")

STATE = {
    "en": {"live": "live", "staged": "staged", "today": "staged: "},
    "de": {"live": "live", "staged": "vorbereitet", "today": "vorbereitet: "},
}

PROBES = {
    "capacity": ("operation", "capacity"),
    "capacity-ramp": ("contains", POLICY, "rampMilliseconds"),
    "leases": ("operation", "leases"),
    "incremental": ("operation", "incremental"),
    "concurrency": ("operation", "concurrency"),
    "cache-ledger": ("subcommand", "cache", "gc"),
    "graph-policy": ("flag", "--graph-policy"),
    "graph-generations": ("exists", LIB + "/🏗️build/🕸️graph/🕰️generations"),
    "bazel-sessions": ("exists", LIB + "/🏗️build/🔧️tools/🏗️bazel/🧵️sessions"),
    "cargo-lanes": ("exists", LIB + "/⚡️caching/🦀️cargo/🗃️unit-cache"),
    "nx-generation": ("exists", LIB + "/⚡️caching/🕸️graph/🔏️seal/📌️generation"),
    "nx-daemons": ("exists", LIB + "/⚡️caching/🕸️graph/🏁️startup/🚏️admission"),
    "nx-put-slots": ("exists", LIB + "/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🎟️put-slots"),
    "cargo-buildscript": ("exists", LIB + "/⚡️caching/🦀️cargo/🗃️unit-cache/🏗️buildscript"),
    "deps-concurrent-admission": ("contains", LIB + "/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🟦️.ts", "admissionRefused"),
    "ci": ("exists", ".github/actions/bun-repo"),
    "deps-concurrent": ("exists", LIB + "/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent"),
    "sync-incr": ("exists", LIB + "/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental"),
    "crate-keys": ("exists", LIB + "/🏗️build/📥️inputs/🦀️resolution"),
    "input-precision": ("exists", LIB + "/🏗️build/📥️inputs/🎯️precision"),
    "ts-units": ("exists", LIB + "/🏗️build/🟦️typescript/🧩️units"),
    "sandbox": ("exists", PROCESS + "/🧪️testing/🏖️sandbox"),
    "parity": ("exists", LIB + "/🏗️build/🪞️parity"),
    "boot-sweep": ("exists", PROCESS + "/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🩺️heal"),
    "hermetic-classes": ("exists", LIB + "/🏗️build/🏃️execution/🔒️hermetic"),
    "default-flip": ("policy", "defaultTool", "bazel"),
}


def catalog():
    return json.load(open(CATALOG, encoding="utf8"))


def probe_states():
    """Maps every feature id to True (live) or False (staged), and lists probes whose parent directory is missing (a mistyped path)."""
    cat = catalog()
    operations = {o["name"]: o for o in cat["operations"]}
    flags = {f["name"] for f in cat["flags"]}
    states, suspicious = {}, []
    for identifier, spec in PROBES.items():
        kind = spec[0]
        if kind == "operation":
            states[identifier] = spec[1] in operations
        elif kind == "subcommand":
            states[identifier] = any(s["name"] == spec[2] for s in operations.get(spec[1], {}).get("subcommands", []))
        elif kind == "flag":
            states[identifier] = spec[1] in flags
        elif kind == "exists":
            full = os.path.join(ROOT, spec[1])
            states[identifier] = os.path.isdir(full)
            if not states[identifier] and not os.path.isdir(os.path.dirname(os.path.dirname(full))):
                suspicious.append(identifier)
        elif kind == "contains":
            target = spec[1] if os.path.isabs(spec[1]) else os.path.join(ROOT, spec[1])
            states[identifier] = os.path.isfile(target) and spec[2] in open(target, encoding="utf8").read()
            if not os.path.isfile(target) and not os.path.isdir(os.path.dirname(os.path.dirname(target))):
                suspicious.append(identifier)
        elif kind == "policy":
            states[identifier] = json.load(open(POLICY, encoding="utf8")).get(spec[1]) == spec[2]
    for identifier in FORCE_LIVE:
        states[identifier] = True
    return states, suspicious


def render(text, language, states):
    """Resolves the macros of one fragment; an unknown id is an error."""
    words = STATE[language]

    def state(identifier):
        if identifier not in states:
            raise KeyError("unknown feature id: " + identifier)
        return states[identifier]

    def phrase(match):
        return words["live"] if state(match.group(1)) else words["staged"]

    def conditional(match):
        parts = match.group(2).split("::")
        if len(parts) != 2:
            raise ValueError("{{?}} needs exactly live::staged: " + match.group(0)[:80])
        return parts[0] if state(match.group(1)) else parts[1]

    def today(match):
        parts = match.group(2).split("::")
        if len(parts) != 2:
            raise ValueError("{{^}} needs exactly today::staged: " + match.group(0)[:80])
        if state(match.group(1)):
            return parts[1]
        staged = parts[1]
        if language == "en" and len(staged) > 1 and staged[0].isupper() and staged[1].islower():
            staged = staged[0].lower() + staged[1:]
        return parts[0].rstrip(".") + "; " + words["today"] + staged

    text = re.sub(r"\{\{@([a-z0-9-]+)\}\}", phrase, text)
    text = re.sub(r"\{\{\?([a-z0-9-]+)::(.*?)\}\}", conditional, text)
    text = re.sub(r"\{\{\^([a-z0-9-]+)::(.*?)\}\}", today, text)
    if "{{" in text or "}}" in text:
        raise ValueError("unresolved macro: " + text[text.index("{{") - 40 if "{{" in text else 0:][:120])
    return text


def sections(language):
    folder = os.path.join(HERE, "staged", language)
    names = sorted(n for n in os.listdir(folder) if n.endswith(".md"))
    return [(n, open(os.path.join(folder, n), encoding="utf8").read().rstrip("\n")) for n in names]

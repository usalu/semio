#!/usr/bin/env python3
"""W2 generator: stdio plugin + binary/txt/json reference artifacts."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]  # may be wrong with emoji depth
# Resolve from cwd
ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]

PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
BUILDER = TOKENS["builder"]
DECOMPOSER = TOKENS["decomposer"]
TEXT = TOKENS["text"]
BINARY_REP = TOKENS["binary"]
DESER = TOKENS["deserializers"]
SER = TOKENS["serializers"]
GRAMMAR = TOKENS["grammar"]
PROTOCOL = TOKENS["protocol"]
EBNF = TOKENS["ebnf"]
G4 = TOKENS["g4"]
ABNF = TOKENS["abnf"]
KSY = TOKENS["ksy"]
SPICY = TOKENS["spicy"]

SCHEMA_LEAVES = [
    ("🦀️component.rs", "rs"),
    ("🟦️component.ts", "ts"),
    ("🔗️component.graphql", "graphql"),
    ("🔣️component.json", "json"),
    ("🛰️component.proto", "proto"),
]

TEXT_LEAVES = [
    GRAMMAR,
    EBNF,
    G4,
    "🔗️component.graphql",
    "🔣️component.json",
    "🛰️component.proto",
    "🦀️component.rs",
    "
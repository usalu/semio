#!/usr/bin/env python3
"""Writes staged/concurrent.fixture.json: the language-neutral cases both the TypeScript implementation and the Python oracle must answer identically."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
HOUR = 3600000
NOW = 1_800_000_000_000


def gen(letter, tail=""):
    return (letter * 64 + tail)[:64]


def pointer_rows(retired):
    return {"retired": [{"generation": gen(letter), "atMs": NOW - age} for letter, age in retired]}


fixture = {
    "version": 1,
    "pointer": [
        {"name": "first publish", "scope": ".", "steps": [{"generation": gen("1"), "nowMs": 1000}]},
        {"name": "switch retires the previous generation", "scope": "🏢️semio-tech", "steps": [{"generation": gen("1"), "nowMs": 1000}, {"generation": gen("2"), "nowMs": 5000}]},
        {"name": "republishing the same generation changes nothing", "scope": ".", "steps": [{"generation": gen("1"), "nowMs": 1000}, {"generation": gen("1"), "nowMs": 9000}]},
        {"name": "rollback drops the generation from the retired list", "scope": "✏️s", "steps": [{"generation": gen("1"), "nowMs": 1000}, {"generation": gen("2"), "nowMs": 2000}, {"generation": gen("3"), "nowMs": 3000}, {"generation": gen("1"), "nowMs": 4000}]},
        {"name": "the clock never moves backwards", "scope": ".", "steps": [{"generation": gen("1"), "nowMs": 5000}, {"generation": gen("2"), "nowMs": 1000}]},
        {"name": "only the newest eight retirements are kept", "scope": ".", "steps": [{"generation": gen(c), "nowMs": 1000 * (index + 1)} for index, c in enumerate("0123456789abcdef"[:12])]},
    ],
    "collect": [
        {"name": "young unreferenced generation is kept for the grace period", "nowMs": NOW, "freeFraction": 0.9, "pointers": [], "rows": [{"name": "a" * 20, "modifiedAtMs": NOW - HOUR}]},
        {"name": "old unlisted generation is collected", "nowMs": NOW, "freeFraction": 0.9, "pointers": [], "rows": [{"name": "a" * 20, "modifiedAtMs": NOW - 7 * HOUR}]},
        {"name": "retirement time counts, not build time", "nowMs": NOW, "freeFraction": 0.9, "pointers": [pointer_rows([("a", HOUR)])], "rows": [{"name": "a" * 20, "modifiedAtMs": NOW - 30 * HOUR}]},
        {"name": "the newest retired generation is the rollback target for a day", "nowMs": NOW, "freeFraction": 0.9, "pointers": [pointer_rows([("a", 10 * HOUR), ("b", 20 * HOUR)])], "rows": [{"name": "a" * 20, "modifiedAtMs": 0}, {"name": "b" * 20, "modifiedAtMs": 0}]},
        {"name": "rollback target is collected after a day", "nowMs": NOW, "freeFraction": 0.9, "pointers": [pointer_rows([("a", 25 * HOUR)])], "rows": [{"name": "a" * 20, "modifiedAtMs": 0}]},
        {"name": "pressure drops every grace", "nowMs": NOW, "freeFraction": 0.1, "pointers": [pointer_rows([("a", 60000)])], "rows": [{"name": "a" * 20, "modifiedAtMs": NOW - 1000}, {"name": "b" * 20, "modifiedAtMs": NOW - 2000}]},
        {"name": "exactly the grace period is due", "nowMs": NOW, "freeFraction": 0.5, "pointers": [], "rows": [{"name": "c" * 20, "modifiedAtMs": NOW - 6 * HOUR}, {"name": "d" * 20, "modifiedAtMs": NOW - 6 * HOUR + 1}]},
        {"name": "one run collects at most sixteen, oldest first", "nowMs": NOW, "freeFraction": 0.9, "pointers": [], "rows": [{"name": ("%020x" % index), "modifiedAtMs": NOW - (7 + (index * 7) % 20) * HOUR} for index in range(20)]},
        {"name": "two scopes retire independently", "nowMs": NOW, "freeFraction": 0.9, "pointers": [pointer_rows([("a", 7 * HOUR)]), pointer_rows([("b", 7 * HOUR), ("c", 30 * HOUR)])], "rows": [{"name": "a" * 20, "modifiedAtMs": 0}, {"name": "b" * 20, "modifiedAtMs": 0}, {"name": "c" * 20, "modifiedAtMs": 0}]},
    ],
    "wait": [
        {"name": "fresh waiter probes only", "elapsedMs": 100, "sinceRecheckMs": 100, "sinceProgressMs": 100, "canRecheck": True, "maximumMs": 60000},
        {"name": "recheck is due after five seconds", "elapsedMs": 5000, "sinceRecheckMs": 5000, "sinceProgressMs": 5000, "canRecheck": True, "maximumMs": 60000},
        {"name": "no recheck without a readiness port", "elapsedMs": 9000, "sinceRecheckMs": 9000, "sinceProgressMs": 9000, "canRecheck": False, "maximumMs": 60000},
        {"name": "progress is due after ten seconds", "elapsedMs": 10000, "sinceRecheckMs": 100, "sinceProgressMs": 10000, "canRecheck": True, "maximumMs": 60000},
        {"name": "expired at the maximum", "elapsedMs": 60000, "sinceRecheckMs": 0, "sinceProgressMs": 0, "canRecheck": True, "maximumMs": 60000},
    ],
    "language": [
        {"name": "empty environment takes the first declared language", "environment": {}},
        {"name": "locale selects german", "environment": {"LANG": "de_DE.UTF-8"}},
        {"name": "explicit variable wins over the locale", "environment": {"SEMIO_LANGUAGE": "en", "LANG": "de_DE.UTF-8"}},
        {"name": "undeclared language is skipped", "environment": {"LC_ALL": "fr_FR", "LANG": "de_CH"}},
        {"name": "tag is case and separator insensitive", "environment": {"LC_MESSAGES": "DE@euro"}},
    ],
    "resources": {
        "bindings": "javascript-dependency-bindings:",
        "build": "javascript-dependency-build:",
        "writer": "javascript-dependency-generation-writer:",
        "pin": "javascript-dependency-pin:",
    },
}
with open(os.path.join(HERE, "staged", "concurrent.fixture.json"), "w", encoding="utf-8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
print("fixture written")

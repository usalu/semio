"""Increment 3 (c5): the authoring catalog; the German texts are in specs/de-authoring.txt, the skeleton in specs/authoring.spec."""
from edits_catalogs import migrate_catalog
from lib import *
from scopes3 import catalog_config


def prepare(tree):
    f = L + "/🏗️authoring/🎮️command/🟦️.ts"
    old = 'if (budget?.verdict === "near-limit" || budget?.verdict === "already-over") console.warn(`new mutation: ${budget.crate} is ${budget.verdict === "already-over" ? "already over" : "next to"} the source gate (lib ${budget.lib.files} files / ${budget.lib.lines} lines, test ${budget.test.files} files); plan a carve with \\`repo ./📜️script.ts new crates\\``);'
    new = 'if (budget?.verdict === "already-over") console.warn(`new mutation: ${budget.crate} is already over the source gate (lib ${budget.lib.files} files / ${budget.lib.lines} lines, test ${budget.test.files} files); plan a carve with \\`repo ./📜️script.ts new crates\\``);\n        else if (budget?.verdict === "near-limit") console.warn(`new mutation: ${budget.crate} is next to the source gate (lib ${budget.lib.files} files / ${budget.lib.lines} lines, test ${budget.test.files} files); plan a carve with \\`repo ./📜️script.ts new crates\\``);'
    tree.edit(f, old, new)


def run(tree):
    prepare(tree)
    migrate_catalog(tree, "authoring", catalog_config("authoring", tree))

"""Increment 3 (c1): the commands catalog; the German texts are in specs/de-commands.txt, the skeleton in specs/commands.spec."""
from edits_catalogs import migrate_catalog
from lib import *
from scopes3 import catalog_config


def prepare(tree):
    f = L + "/🎛️commands/🏭️generate/🟦️.ts"
    tree.edit(f, "console.log(`[generate plugin-glue] ${dry ? \"dry-run\" : \"wrote\"} inventory for ${emitted} subset refs`);", "if (dry) console.log(`[generate plugin-glue] dry-run inventory for ${emitted} subset refs`);\n    else console.log(`[generate plugin-glue] wrote inventory for ${emitted} subset refs`);")


def run(tree):
    prepare(tree)
    migrate_catalog(tree, "commands", catalog_config("commands", tree))

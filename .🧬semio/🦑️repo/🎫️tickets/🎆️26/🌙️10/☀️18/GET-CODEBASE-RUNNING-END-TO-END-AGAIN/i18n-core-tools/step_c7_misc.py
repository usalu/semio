"""Increment 3 (c7): the misc catalog; the German texts are in specs/de-misc.txt, the skeleton in specs/misc.spec."""
from edits_catalogs import migrate_catalog
from scopes3 import catalog_config


def run(tree):
    migrate_catalog(tree, "misc", catalog_config("misc", tree))

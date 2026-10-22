"""Increment 3 (c2): the caching catalog; the German texts are in specs/de-caching.txt, the skeleton in specs/caching.spec."""
from edits_catalogs import migrate_catalog
from scopes3 import catalog_config


def run(tree):
    migrate_catalog(tree, "caching", catalog_config("caching", tree))

"""Increment 3 (c3): the dependencies catalog; the German texts are in specs/de-dependencies.txt, the skeleton in specs/dependencies.spec."""
from edits_catalogs import migrate_catalog
from scopes3 import catalog_config


def run(tree):
    migrate_catalog(tree, "dependencies", catalog_config("dependencies", tree))

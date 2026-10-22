"""Increment 3 (c6): the cleanup catalog; the German texts are in specs/de-cleanup.txt, the skeleton in specs/cleanup.spec."""
from edits_catalogs import migrate_catalog
from scopes3 import catalog_config


def run(tree):
    migrate_catalog(tree, "cleanup", catalog_config("cleanup", tree))

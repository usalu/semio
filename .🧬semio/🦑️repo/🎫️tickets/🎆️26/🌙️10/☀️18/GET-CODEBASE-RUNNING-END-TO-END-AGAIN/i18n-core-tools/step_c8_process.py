"""Increment 3 (c8): the process catalog; the German texts are in specs/de-process.txt, the skeleton in specs/process.spec."""
from edits_catalogs import migrate_catalog
from scopes3 import catalog_config


def run(tree):
    migrate_catalog(tree, "process", catalog_config("process", tree))

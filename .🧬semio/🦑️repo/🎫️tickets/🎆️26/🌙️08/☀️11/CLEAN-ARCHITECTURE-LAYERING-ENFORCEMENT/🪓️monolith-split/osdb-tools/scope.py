"""🔭 Original name scope of the db tree: top-level definitions per module and the crate-root glob scope.

Read-only analysis over the pristine snapshot.
"""
import os
import re
import collections

from lib import *

TOP_KIND = {'fn', 'struct', 'enum', 'trait', 'type', 'static', 'const', 'union', 'macro_rules!', 'mod'}

FACADE_FAMILY = {
    'ids': 'db_ids', 'durability': 'db_durability', 'policy': 'db_policy', 'actor': 'db_actor', 'state': 'db_state',
    'storage': 'db_storage', 'wal': 'db_wal', 'snapshot': 'db_snapshot', 'index': 'db_index', 'conflict': 'db_conflict',
    'projection': 'db_projection', 'query': 'db_query', 'preview': 'db_preview', 'security': 'db_security',
    'document': 'db_artifact', 'compact': 'db_compact', 'sync': 'db_sync', 'cluster': 'db_cluster',
    'observe': 'db_observe', 'engine': 'db_engine', 'storage_sqlite': 'db_storage_sqlite',
    'storage_postgres': 'db_storage_postgres', 'storage_neo4j': 'db_storage_neo4j',
}

STANDALONE_MODULES = {
    'db_storage_driver_runtime': '🗄️storage/🧵️driver-runtime',
    'db_storage_postgres': '🗄️storage/🐘️postgres',
    'db_storage_sqlite': '🗄️storage/🪶️sqlite',
    'db_storage_neo4j': '🗄️storage/🌐️neo4j',
    'db_fault_testing': '🧪️tests/🧯️fault-storage',
}


def module_path(ident, base=PRISTINE):
    if ident in IDENT_MODULE:
        return f'{base}/{MODULE_DIRS[IDENT_MODULE[ident]]}/🦀️.rs'
    return f'{base}/{STANDALONE_MODULES[ident]}/🦀️.rs'


ALL_MODULE_IDENTS = list(MODULE_IDENT.values()) + list(STANDALONE_MODULES)


def top_defs(path):
    """Top-level named definitions: name -> (kind, visibility, first_line, last_line, header_line)."""
    text = read(path)
    lines = text.split('\n')
    out = {}
    for kind, nm, s, e, h, vis in outline(path):
        if kind in ('use', 'impl', 'extern'):
            continue
        m = re.match(r'\s*([A-Za-z_][A-Za-z0-9_]*)', nm or '')
        if kind == 'macro_rules!':
            m = re.match(r'\s*([A-Za-z_][A-Za-z0-9_]*)', nm or '')
        if m:
            out[m.group(1)] = (kind, vis.strip(), s, e, h)
    return out


def pub_names(defs):
    return {n for n, (k, v, s, e, h) in defs.items() if v.startswith('pub') and '(' not in v}


class Scope:
    def __init__(self):
        self.defs = {}
        for ident in ALL_MODULE_IDENTS:
            self.defs[ident] = top_defs(module_path(ident))
        self.root = self._root()
        self.root_low = {k: v for k, v in self.root.items() if not (v[0] == 'mod' and v[1] == 'db_engine')}

    def root_for(self, lib):
        return self.root if lib in ('db_cli', 'db_fault_testing', 'db') else self.root_low

    def _root(self):
        """Name -> ('mod', module_ident) or ('ext', path) for the crate-root glob scope."""
        r = {}
        engine_pub = [
            'take_database_capability_open_terminal', 'take_database_catalog_read_terminal', 'take_next_database_capability_open_terminal',
            'ArtifactHandle', 'ArtifactHistoryTerminalConstructionFault', 'ArtifactHistoryTerminalHandle', 'ArtifactSpec', 'CatalogEntry',
            'CatalogView', 'CheckpointPublicationSnapshot', 'CommandReceipt', 'Consistency', 'Database', 'DatabaseCapabilityOpenCloseStep',
            'DatabaseCapabilityOpenFuture', 'DatabaseCapabilityOpenProgress', 'DatabaseCapabilityOpenRejected', 'DatabaseCapabilityOpenResult',
            'DatabaseCapabilityOpenTerminalHandle', 'DatabaseCapabilityOpenTerminalResult', 'DatabaseCatalogReadCloseStep', 'DatabaseCatalogReadFuture',
            'DatabaseCatalogReadProgress', 'DatabaseCatalogReadRejected', 'DatabaseCatalogReadResult', 'DatabaseCatalogReadTerminalHandle',
            'DatabaseCatalogReadTerminalResult', 'DatabaseCatalogRootKey', 'DatabaseDocumentOpenRejected', 'DatabaseRetainedActivityRejected',
            'DatabaseShutdownBlock', 'DatabaseShutdownControl', 'DatabaseShutdownPhase', 'DatabaseShutdownProgress', 'DbHealth', 'HistoryEntry',
            'HistoryView', 'LiveQuery', 'LiveQuerySpec', 'PreviewHandle', 'Query', 'QueryResultEntry', 'QueryStream', 'SnapshotFuture',
            'SnapshotKind', 'SnapshotReceipt', 'SubmitFuture',
        ]
        for n in engine_pub:
            r[n] = ('mod', 'db_engine')
        for n in pub_names(self.defs['db_durability']):
            r[n] = ('mod', 'db_durability')
        for n in pub_names(self.defs['db_policy']):
            r[n] = ('mod', 'db_policy')
        r['Frontier'] = ('mod', 'db_durability')
        for n in ('check_len', 'ActorId', 'ArtifactId', 'DbError', 'DbLimits', 'GenerationId'):
            r[n] = ('mod', 'db_ids')
        for n in ('DbCapabilities', 'DbConfig', 'Priority', 'Profile'):
            r[n] = ('mod', 'db_policy')
        for n in ('Emit', 'EmitEvent', 'EmitField', 'NullEmit'):
            r[n] = ('mod', 'db_observe')
        r['ContentHash'] = ('ext', 'pack::ContentHash')
        return r


if __name__ == '__main__':
    s = Scope()
    for k, v in s.defs.items():
        print(k, len(v), sum(1 for x in v.values() if x[1].startswith('pub') and '(' not in x[1]))
    print(len(s.root), sorted(s.root)[:200])

"""📋 The crate plan: crate list, item-to-crate assignment for split modules, test assignment.

All assignments are derived from the pristine tree (header lines are 1-based pristine line numbers).
"""
import collections
import os
import re

from lib import *
from model import parse_items
from rs import strip, idents
from sources import load_all, Src
from graph import load as gload, edges as gedges
from scc2 import sccs

PKG_PREFIX = 'semio-framework-os-kernel-db-'
ID_PREFIX = 'os-db-'


class Crate:
    def __init__(self, lib, module, node, kind='whole', suffix=None, desc='', gated=False, emoji='', features=None, doc_owner=False):
        self.lib = lib
        self.module = module
        self.node = node
        self.kind = kind
        self.suffix = suffix or lib[3:].replace('_', '-')
        self.desc = desc
        self.gated = gated
        self.features = features or {}
        self.doc_owner = doc_owner

    @property
    def package(self):
        return PKG_PREFIX + self.suffix

    @property
    def project(self):
        return f'@semio-tech/{ID_PREFIX}{self.suffix}-rs'

    @property
    def id(self):
        return ID_PREFIX + self.suffix

    @property
    def pkg_dir(self):
        return f'{self.node}/📦️packages/🦀️rust'


def mod_node(name):
    return MODULE_DIRS[name]


CRATES = []


def whole(lib, node, desc, gated=False):
    CRATES.append(Crate(lib, MODULE_IDENT_BY_LIB.get(lib, lib), node, 'whole', desc=desc, gated=gated))


MODULE_IDENT_BY_LIB = {}


def chunk(lib, module, node, desc, suffix=None, gated=False, doc_owner=False):
    CRATES.append(Crate(lib, module, node, 'chunks', suffix=suffix, desc=desc, gated=gated, doc_owner=doc_owner))


S = '🗄️storage'
whole('db_ids', '🆔️ids', 'Db identity types, DbError and resource limits')
whole('db_durability', '💾️durability', 'Db durability classes, frontiers and epoch fences')
whole('db_policy', '🎚️policy', 'Db profiles, capabilities and configuration')
whole('db_observe', '👁️observe', 'Db structured and audit event sinks, metrics, spans and health')
whole('db_actor', '🎭️actor', 'Db bounded-priority mailbox actor runtime')
whole('db_security', '🔒️security', 'Db multi-granularity authorization, replay guard, budgets and redaction')
whole('db_state', '🔘️state', 'Db persistent structurally-shared overlay state')
whole('db_conflict', '⚔️conflict', 'Db conflict detection for concurrent commands')
whole('db_preview', '🔮️preview', 'Db ephemeral speculative document overlays')
whole('db_projection', '📽️projection', 'Db typed versioned projection engine')
whole('db_cluster', '🌐️cluster', 'Db sharding, ownership leases and replication routing')
whole('db_snapshot', '📸️snapshot', 'Db pack-file based document snapshots')
whole('db_index', '🔢️index', 'Db secondary index engine')
whole('db_query', '🔍️query', 'Db consistency resolution, query IR, planner and executor')
whole('db_compact', '🗜️compact', 'Db WAL retention, payload GC and index compaction')
whole('db_sync', '🔄️sync', 'Db server side of the wire protocol: frontier exchange and bootstrap')
whole('db_cli', '⌨️cli', 'Db command line inspection and repair tooling', gated=True)
whole('db_storage_sqlite', f'{S}/🪶️sqlite', 'Db SQLite storage backend')
whole('db_storage_postgres', f'{S}/🐘️postgres', 'Db Postgres storage backend')
whole('db_storage_neo4j', f'{S}/🌐️neo4j', 'Db Neo4j storage backend')
whole('db_storage_driver_runtime', f'{S}/🧵️driver-runtime', 'Db bounded runtime for the external storage drivers', gated=True)

chunk('db_wal_log', 'db_wal', '📝️wal/🧾️log', 'Db write-ahead log records, payload transforms, group commit, sink and recovery', suffix='wal-log', doc_owner=True)
chunk('db_wal', 'db_wal', '📝️wal', 'Db write-ahead log segments and the per-document ArtifactWal')
chunk('db_artifact_contract', 'db_artifact', '🗿️artifact/🧬️contract', 'Db artifact commands, diffs, conflict records and durable-group receipts', suffix='artifact-contract')
chunk('db_artifact_history', 'db_artifact', '🗿️artifact/📜️history', 'Db artifact history replay', suffix='artifact-history')
chunk('db_artifact_engine', 'db_artifact', '🗿️artifact/⚙️engine', 'Db artifact engine: document state, query source, state pages and ledger tail', suffix='artifact-engine')
chunk('db_artifact', 'db_artifact', '🗿️artifact', 'Db document authority actor and runner', doc_owner=True)
chunk('db_engine_base', 'db_engine', '⚙️engine/🧱️base', 'Db engine shared types, catalog codec and document mount registry', suffix='engine-base')
chunk('db_engine_capability_open', 'db_engine', '⚙️engine/🔭️capability-open', 'Db engine retained capability open operation', suffix='engine-capability-open')
chunk('db_engine_catalog_read', 'db_engine', '⚙️engine/📖️catalog-read', 'Db engine retained catalog root read operation', suffix='engine-catalog-read')
chunk('db_engine_catalog_bootstrap', 'db_engine', '⚙️engine/🥾️catalog-bootstrap', 'Db engine retained catalog bootstrap operation', suffix='engine-catalog-bootstrap')
chunk('db_engine_catalog_create', 'db_engine', '⚙️engine/🏗️catalog-create', 'Db engine retained create-document catalog operation', suffix='engine-catalog-create')
chunk('db_engine_handle', 'db_engine', '⚙️engine/🎫️handle', 'Db engine artifact handle: submit, history and query operations', suffix='engine-handle')
chunk('db_engine', 'db_engine', '⚙️engine', 'Db engine supervisor: Database, shutdown and document mounting', doc_owner=True, gated=True)
chunk('db_io_core', 'db_storage', f'{S}/🧱️io-core', 'Db I/O kernel core: credit ledger, page arena, task model and lost-owner parking', suffix='io-core')
chunk('db_io_registry', 'db_storage', f'{S}/📇️io-registry', 'Db I/O kernel backend registry, close and retirement turns', suffix='io-registry')
chunk('db_io_driver', 'db_storage', f'{S}/🚦️io-driver', 'Db I/O kernel driver: task submission, drive loop, WAL writer table and the storage traits', suffix='io-driver')
chunk('db_storage_memory', 'db_storage', f'{S}/🗃️memory', 'Db in-memory storage backend', suffix='storage-memory')
chunk('db_storage_fs', 'db_storage', f'{S}/📁️fs', 'Db filesystem storage backend', suffix='storage-fs')
chunk('db_storage', 'db_storage', f'{S}', 'Db storage dispatch: DbBackend and the facet reference enums', doc_owner=True)

chunk('db_fault_testing', 'db_fault_testing', '🧪️tests/🧯️fault-storage', 'Db fault and deterministic-simulation test support: seeded simulation runtime, crash harness and the cross-crate laws', suffix='fault-testing', gated=True, doc_owner=True)

BY_LIB = {c.lib: c for c in CRATES}
for c in CRATES:
    pass

WHOLE_LIBS = {c.lib for c in CRATES if c.kind == 'whole'}
SPLIT_MODULES = {'db_wal', 'db_artifact', 'db_engine', 'db_storage', 'db_fault_testing'}
CHUNK_LIBS = {m: [c.lib for c in CRATES if c.kind == 'chunks' and c.module == m] for m in SPLIT_MODULES}
ALL_LIBS = [c.lib for c in CRATES]

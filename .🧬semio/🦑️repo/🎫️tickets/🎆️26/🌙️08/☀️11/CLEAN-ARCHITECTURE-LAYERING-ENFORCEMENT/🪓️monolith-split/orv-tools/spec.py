"""Split specifications of the orv targets (ranges are 1-based inclusive lines of orig/<name>.rs.txt)."""


def C(folder, emoji, mod, title, doc, ranges, macro_use=False, fixes=(), imports=(), **kw):
    d = dict(folder=folder, emoji=emoji, mod=mod, title=title, doc=doc, ranges=ranges, macro_use=macro_use, fixes=list(fixes), imports=list(imports))
    d.update(kw)
    return d


SPEC = {
    "run": dict(anchor=65, children=[
        C("🚧️run-errors", "🚧️", "run_errors", "Run errors", "the run failure, the document-load progress record and the activation refusal.", [(32, 32), (67, 197)]),
        C("📡️channel-ports", "📡️", "channel_ports", "Channel ports", "the app channel host and media cache ports plus the file and in-memory blob stores.", [(199, 327)]),
        C("🎞️media", "🎞️", "media", "Media", "media-to-artifact conversion, reply decoding helpers and the media converter registry.", [(329, 580)]),
        C("🧾️run-context", "🧾️", "run_context", "Run context", "the run sink that keeps a run's mutated documents and configs apart from its source.", [(582, 658)]),
        C("🗃️space-bundle", "🗃️", "space_bundle", "Space bundle", "the studio bundle: workflow, packages, documents, configs and the node lookups a run reads.", [(660, 943)]),
        C("🧭️topology-and-edges", "🧭️", "topology_and_edges", "Topology and edges", "topological node order and edge media-kind validation.", [(945, 1028)]),
        C("🏃️space-runner", "🏃️", "space_runner", "Space runner", "the headless run driver that walks the workflow and drives each node through the channel.", [(1030, 1492)]),
        C("🧠️wasmtime-node-host", "🧠️", "wasmtime_node_host", "Wasmtime node host", "the native node host that opens, drives and closes each node's app instance.", [(1494, 2457)]),
    ]),
    "vcs": dict(anchor=23, children=[
        C("🪪️checkpoint-records", "🪪️", "checkpoint_records", "Checkpoint records", "the author, change, composition pin, checkpoint and alternative records of the version graph.", [(28, 137)], into=1640),
        C("📚️history-page-stack", "📚️", "history_page_stack", "History page stack", "the paged, bounded-allocation stack that stores one history catalog and its serialization.", [(139, 612)], into=1640),
        C("🪟️group-visibility", "🪟️", "group_visibility", "Group visibility", "the shared decision bit with which a staged history group becomes visible to every direct reader at once.", [(614, 690)], into=1640),
        C("🗃️history-ledger", "🗃️", "history_ledger", "History ledger", "the generation-keyed history ledger, its reservations, iterators and trait implementations.", [(692, 1555)], into=1640),
        C("🌳️artifact-vcs", "🌳️", "artifact_vcs", "Artifact version graph", "the artifact version graph with its read view and value conversions.", [(1557, 1639)], into=1640),
        C("🚨️vcs-errors", "🚨️", "vcs_errors", "Version graph errors", "the version graph error with its display, fault and conversion implementations.", [(1641, 1830)], into=1898, wrap=True),
        C("🧮️diff-and-mutation", "🧮️", "diff_and_mutation", "Diff and mutation", "the sparse collection diff, the identified and patchable item traits and the mutation replay step.", [(1831, 1895)], into=1898, wrap=True),
    ]),
    "pack-value": dict(anchor=31, children=[
        C("🧱️canonical-forms", "🧱️", "canonical_forms", "Canonical forms", "the shape probes, the symbol table walk and the encode context every wire path shares.", [(33, 309)]),
        C("📝️record-encode", "📝️", "record_encode", "Record encode", "the canonical field, value, sequence, map, statement and wire encoders.", [(311, 611)]),
        C("📖️record-decode", "📖️", "record_decode", "Record decode", "the decode context, the materialization admission and the field, value, sequence, map, statement and wire decoders.", [(613, 1033)]),
        C("🧮️table-soa", "🧮️", "table_soa", "Table SoA", "the columnar `TableSoA` element tags, encoder and decoder.", [(1035, 1288)]),
        C("🔑️schema-hash", "🔑️", "schema_graph", "Schema hash", "the canonical schema graph and the schema hash over it.", [(1290, 1346), (1349, 1666)]),
        C("📄️document-codec", "📄️", "document_codec", "Document codec", "the document and record body encode and decode entry points with the decode report.", [(1668, 1885)]),
    ]),
    "neural-model": dict(anchor=24, children=[
        C("📚️dictionary", "📚️", "dictionary", "Dictionary", "the immutable key-value dictionary with its value and atom vocabulary.", [(25, 41), (46, 182), (186, 427)]),
        C("🏷️schema-model", "🏷️", "schema_model", "Schema model", "the schema, field specification and reference records, the schema errors and the value type names.", [(428, 673)]),
        C("🌳️neuron-tree", "🌳️", "neuron_tree", "Neuron tree", "the neuron and synapse tree with its channel contract constants.", [(674, 745), (748, 841)]),
        C("⚙️operator-record", "⚙️", "operator_record", "Operator record", "the evaluation errors, the operator and job traits, the cardinality and the channel specification.", [(842, 888), (891, 1378)]),
        C("🔏️hash-and-dirty-set", "🔏️", "hash_and_dirty_set", "Hash and dirty set", "the neuron and subtree hashing and the dirty-set propagation.", [(1379, 1500)]),
        C("🧮️evaluator", "🧮️", "evaluator", "Evaluator", "the evaluation channels and the input and output validation.", [(1501, 1622)]),
    ]),
    "io": dict(anchor=0, drop_lines=[745, 746, 1726], items_lost_ok=1, children=[
        C("📇️composer-entries", "📇️", "composer_entries", "Composer entries", "the composer entry preflight, registration and listing.", [(127, 176)], into=730),
        C("🌉️compose-dispatch", "🌉️", "compose_dispatch", "Compose dispatch", "the local-first compose dispatch with its settable cross-plugin fallback.", [(177, 264)], into=730),
        C("🛡️subset-validator", "🛡️", "subset_validator", "Subset validator", "the subset validator registry and the subset validation run.", [(265, 400)], into=730),
        C("🎚️io-fidelity", "🎚️", "io_fidelity", "IO fidelity", "the fidelity class operations and the manifest-facing fidelity declaration.", [(401, 459)], into=730),
        C("🔌️wire-framing", "🔌️", "wire_framing", "Wire framing", "the bounded wire framing of compose, decode and list requests.", [(460, 646)], into=730),
        C("📚️format-registry", "📚️", "format_registry", "Format registry", "the atomic format descriptor registration, lookup and file-picker filter.", [(647, 716), (719, 729)], into=730),
        C("🧭️io-mechanism", "🧭️", "io_mechanism", "IO mechanism", "the typed serializer and deserializer vtable registry, routing, execution and identification.", [(747, 1725)], into=1727, tight=True, vis="pub", glob=False, super_glob=False, inline_body=True, dedent=4, widen=False,
          head_doc="//! This file is the body of the `io_mechanism` module; the root mounts it as a module and keeps its documentation."),
    ]),
}

#!/usr/bin/env python3
"""🧾️ Finalizes the hub lib carve: manifests, project files, shared registries, formatting parse and flush."""
import os, sys, json, re, collections, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_lib, regs, fmtcheck, hcmod
from plan_lib import CRATES, P
from carve import REPO

DESC = {
    'semio_hub_testing': 'Writable artifact root every descriptor-rooted hub law stages its fixtures under',
    'semio_hub_refusal': 'The one typed shape of every answer the hub gives that is not a success',
    'semio_hub_directory_contract': 'Directory contract: errors, bounded constants, events, records and capability vocabulary shared by every directory backend',
    'semio_hub_artifact_authority': 'Hub-owned canonical artifact authority, failure-atomic immutable publication ports and the artifact chunk CAS',
    'semio_hub_artifact_creation': 'Host-owned artifact creation facts, operation schema and physical commitments',
    'semio_hub_directory': 'Backend-agnostic hub identity and tenancy directory port with its wire validators',
    'semio_hub_directory_sqlite': 'HubDirectory over SQLite (rusqlite)',
    'semio_hub_directory_postgres': 'HubDirectory over PostgreSQL (sqlx)',
    'semio_hub_directory_neo4j_rows': 'Neo4j directory rows: the Cypher row decoders and writers, retry classification and effect-commit guards',
    'semio_hub_directory_neo4j': 'HubDirectory over Neo4j (neo4rs)',
    'semio_hub_directory_dispatch': 'The closed HubDirectories dispatch over the sqlite, postgres and neo4j backends',
    'semio_hub_directory_service': 'The serialized directory writer, command decider, projection rebuild and verified checkpoint publisher',
    'semio_hub_directory_laws': 'Directory laws: the writer and decider unit laws and the per-backend artifact-creation laws',
    'semio_hub_trusted_catalog_contract': 'Trusted-catalog contract: bundle schema, opened roots, browser actor and plugin module sources',
    'semio_hub_trusted_catalog_descriptor': 'Trusted-catalog descriptor validation, document-open targets and retained asset handles',
    'semio_hub_trusted_catalog': 'Trusted-catalog loader, guest residency and the verified catalog',
    'semio_hub_trusted_catalog_laws': 'Trusted-catalog laws over the verified catalog',
    'semio_hub_trusted_catalog_fixtures': 'Feature-gated real GIS Map profile builder for the hub binary laws',
    'semio_hub_component_codec_laws': 'Component codec laws over caller-selected runtimes',
    'semio_hub_artifact_creation_service': 'Artifact creation service coordinating durable facts, one factory execution and the sole genesis publication',
    'semio_hub_artifact_checkin': 'Hub-materialized document Check In authority and job registry',
    'semio_hub_native_openable': 'Hub-private static native openable catalog provider',
    'semio_hub_observability': 'Counters and latency percentiles an administrator reads',
    'semio_hub_auth': 'Credential sign-in, session minting policy and the route rate limiter',
    'semio_hub_stores': 'Hub as instance #1 of the server product with its four durable storage roles',
    'semio_hub_lag_rebootstrap': 'Verified lag rebootstrap source and canonical checkpoint pair',
    'semio_hub_local_bootstrap': 'Local bootstrap service and its inherited transport',
    'semio_hub_inference': 'Hub-private inference lifecycle: schema, canonical command and write-ahead proof',
    'semio_hub_inference_ledger': 'Owner-private inference job ledger over SQLite',
    'semio_hub_inference_catalog': 'Frozen inference binding verification and live author authorization',
    'semio_hub_inference_runtime_contract': 'Inference runtime contract: approval ports, composed-map binding and job vocabulary',
    'semio_hub_inference_committer': 'Retained approval committer for composed-map proposals',
    'semio_hub_inference_runtime': 'Owner-private proposal runtime, routes and checkpoint control',
    'semio_hub_inference_laws': 'Inference laws over the proposal runtime and the retained approval committer',
    'semio_hub_inference_contract_laws': 'Inference contract laws: the write-ahead proof, schema, approval and canonical command laws',
    'semio_hub_trusted_catalog_contract_laws': 'Trusted-catalog contract laws: plugin-module index, opened-root publication and browser actor',
    'semio_hub_directory_backend_laws': 'Directory backend laws: the per-backend unit laws of the sqlite, postgres and neo4j directories',
    'semio_hub_runtime': 'Hub runtime foundations: startup supervisors, readiness, transport middleware, lifecycle and shared HTTP support',
    'semio_hub_ledgers': 'Hub ledgers: socket grants, document open plans, authentication guard types, pair blobs and access-grant vocabulary',
    'semio_hub_route_models': 'Hub route models: admin, artifact creation, directory, document socket, check in, session and state seam vocabulary',
    'semio_hub_state': 'The shared HubState every hub route handler runs over',
    'semio_hub_session_routes': 'Hub session routes: guard, readiness probes, transport limits, access grants, agent sessions and directory commands',
    'semio_hub_document_routes': 'Hub document routes: document sockets, directory events, check in, administration, artifact creation, inference and assets',
    'semio_hub_routes': 'The hub router composing every route family',
    'semio_hub_route_testkit': 'Shared fixtures of the hub route laws: test states, servers, sockets, sessions and document-open plan authorities',
    'semio_hub_boot_laws': 'Boot laws: startup, readiness, hostile routes, check in, inference and canonical pair routes over a real hub router',
    'semio_hub_open_plan_laws': 'Open-plan laws: document-open plans, execution targets, supersession, socket grants and scoped directory sockets',
    'semio_hub_directory_route_laws': 'Directory route laws: administration, directory events and commands, space pages and presence',
    'semio_hub_session_route_laws': 'Session route laws: credential sign-in, agent delegation, origin and proxy posture, observability and shutdown',
    'semio_hub_presence_route_laws': 'Presence route laws: presence normalization, lease, roster and liveness over real sockets',
    'semio_hub_hostile_input_laws': 'Hostile input laws: the generative hostile-input suites and the held count lease',
}
EXTRA_FEATURES = {}


OPTIONAL = {
    'semio_hub_directory_postgres': {'db': {'features': ['claimed-backend-laws']}},
    'semio_hub_directory_neo4j': {'db': {'features': ['claimed-backend-laws']}},
}
FEATURE_EXTRA = {
    'semio_hub_directory_postgres': {'testkit': ['dep:db']},
    'semio_hub_directory_neo4j': {'testkit': ['dep:db']},
}


def manifest_texts(model, roots):
    table = {k: dict(name=v['name'], root=v['root']) for k, v in CRATES.items()}
    crate_features = {k: regs.used_features(model, k) for k in table}
    tests_of = {k: regs.has_tests(model, k) for k in table}
    out = {}
    for ident, info in table.items():
        if ident == 'semio_hub':
            continue
        info = dict(info, description=DESC.get(ident, ident))
        text, usage = regs.build_cargo(model, ident, info, table, crate_features, tests_of, features_extra=FEATURE_EXTRA.get(ident), optional_deps=OPTIONAL.get(ident))
        out[ident] = (text, usage)
    return out


if __name__ == '__main__':
    ws, model, roots = plan_lib.load_state()
    texts = manifest_texts(model, roots)
    for ident, (text, usage) in texts.items():
        print('=====', ident)
        print(text[text.index('[dependencies]'):])

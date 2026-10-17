"""🗺️ c4p38 split plans for CENSUS4-PKG-38 (hub rust). `n` is the snapshot number in `orig/`, `rel` the repo path of the target.
`mode` "mod": production roots; children are `#[path]` modules (`use super::*;`, root `<vis> use <mod>::*;`, private definitions that another file names become `pub(super)`).
`mode` "include": test roots; children are `include!` fragments, so the module identity and every test path stay identical.
`ranges` are 1-based inclusive line ranges of the snapshot (docs, attributes and region markers included). `force_vis` makes the root re-export a child whose items a macro generates. `hoist` moves root lines to just after a header line."""

H = "🌎️hub"
CAS = f"{H}/🗿️artifact-authority/🧱️chunk-cas"
BOOT = f"{H}/🚀️local-bootstrap"
SPACE = f"{H}/🧩️compositions/🪐️space/⚙️engine/🪐️space"
BIN = f"{H}/🧪️tests/🔬️bin-unit"

PLANS = {
    "cas": dict(
        n=1, rel=f"{CAS}/🦀️.rs", mode="mod", lib=f"{H}/🗿️artifact-authority/🦀️.rs", pkg="semio-hub-artifact-authority",
        hoist=(2, [677]),
        children=[
            dict(folder="🧬️object-model", mod="object_model", ranges=[(11, 191)],
                 doc="🧬️ Artifact CAS object model: limits and domains, chunk and manifest records, object keys, ownership plans, reservations and the delete fence."),
            dict(folder="🚪️storage-port", mod="storage_port", ranges=[(193, 245)],
                 doc="🚪️ The artifact CAS storage port: put and delete outcomes, the storage trait and its shared-handle forwarding."),
            dict(folder="🧮️manifest-codec", mod="manifest_codec", ranges=[(247, 582)],
                 doc="🧮️ The canonical manifest and ownership codecs: field cursor, prepare, validate, encode, decode and the manifest locator."),
            dict(folder="🧠️memory-storage", mod="memory_storage", ranges=[(584, 669)],
                 doc="🧠️ The collision-checked, dependency-free memory CAS."),
            dict(folder="📁️filesystem-storage", mod="filesystem_storage", ranges=[(671, 675), (679, 1018)],
                 doc="📁️ The dedicated filesystem CAS: leaf opening, fences, symlink and parent validation and the storage port over files."),
            dict(folder="🗄️sqlite-storage", mod="sqlite_storage", ranges=[(1020, 1217)],
                 doc="🗄️ The dedicated SQLite artifact CAS table and its storage port."),
            dict(folder="🐘️postgres-storage", mod="postgres_storage", ranges=[(1219, 1383)],
                 doc="🐘️ The dedicated Postgres artifact CAS and its storage port."),
            dict(folder="🕸️neo4j-storage", mod="neo4j_storage", ranges=[(1385, 1497)],
                 doc="🕸️ The dedicated Neo4j artifact CAS and its storage port."),
            dict(folder="🔀️store-selector", mod="store_selector", ranges=[(1499, 1558)],
                 doc="🔀️ The runtime-selected artifact CAS enum and its storage port without trait-object futures."),
            dict(folder="🫙️blob-store", mod="blob_store", ranges=[(1560, 1641)],
                 doc="🫙️ The manifest-based immutable blob adapter used by publication and rebootstrap."),
        ]),
    "boot": dict(
        n=2, rel=f"{BOOT}/🦀️.rs", mode="mod", lib=f"{BOOT}/🦀️.rs", pkg="semio-hub-local-bootstrap",
        children=[
            dict(folder="🧾️wire-frames", mod="wire_frames", ranges=[(35, 230), (894, 934)],
                 doc="🧾️ The bootstrap wire: signed and unsigned frames, the credential envelope and the HMAC proof helpers."),
            dict(folder="🔁️exchange-window", mod="exchange_window", ranges=[(240, 294)],
                 doc="🔁️ The bounded replay window and the outstanding-request slots of exchange ids."),
            dict(folder="🚰️transport-state", mod="transport_state", ranges=[(232, 238), (296, 380)],
                 doc="🚰️ The inherited transport state: the secret channel key, the closed signal, the pump control and the frame pump."),
            dict(folder="🛰️transport-session", mod="transport_session", ranges=[(382, 683)],
                 doc="🛰️ The inherited transport: opening over the descriptor, hello, signing, request admission, issue and reject responses."),
            dict(folder="🔌️transport-port", mod="transport_port", ranges=[(685, 770)],
                 doc="🔌️ The local bootstrap transport port over the inherited pipe and the request and transport controls."),
            dict(folder="🎫️issuance-service", mod="issuance_service", ranges=[(772, 892), (1068, 1078)],
                 doc="🎫️ The bounded issuance service: serving the pipe, issuing local sessions, validating initialize and frames and the error vocabulary."),
            dict(folder="📨️frame-io", mod="frame_io", ranges=[(936, 1066)],
                 doc="📨️ Framed pipe I/O: deadlines, bounded frame reads and writes and the inherited descriptor handle."),
        ]),
    "space": dict(
        n=3, rel=f"{SPACE}/🦀️.rs", mode="mod", lib=f"{SPACE}/🦀️.rs", pkg="semio-hub-space-app",
        children=[
            dict(folder="🧰️document-helpers", mod="document_helpers", ranges=[(56, 183)],
                 doc="🧰️ Document helpers shared by two or more command modules: play actions, UI values, selection, config mutations and the workflow context menu."),
            dict(folder="🕹️space-command", mod="space_command", ranges=[(185, 237)], force_vis="pub ",
                 doc="🕹️ The sole dispatch surface of the studio app: the generated command enum, one variant per declared action."),
            dict(folder="🏠️space-app", mod="space_app", ranges=[(239, 775)],
                 doc="🏠️ The studio app instance: bounded retained command jobs, config store preparation and the artifact app implementation."),
            dict(folder="📜️space-manifest", mod="space_manifest", ranges=[(777, 1016)],
                 doc="📜️ The studio app manifest: play layout, modes, windows, panels, actions and the proof catalog."),
        ]),
    "hostile": dict(
        n=4, rel=f"{BIN}/🎲️hostile-input/🔬️unit/🦀️.rs", mode="include", lib=f"{BIN}/🎲️hostile-input/🏛️laws/🦀️.rs", pkg="semio-hub-hostile-input-laws",
        children=[
            dict(folder="🧰️law-harness", mod="law_harness", ranges=[(26, 209)],
                 doc="🧰️ Law harness: the process-isolated law protocol and the check-in fixtures, commits, replica pairs and cold pairs."),
            dict(folder="⚡️quick-hostile-laws", mod="quick_hostile_laws", ranges=[(214, 946)],
                 doc="⚡️ The quick hostile-input laws module: socket grants, admin revocation, check-in policy and route admission."),
            dict(folder="🐢️long-hostile-laws", mod="long_hostile_laws", ranges=[(948, 1369)],
                 doc="🐢️ The long hostile-input laws module: bounded admin pages, the socket grant ledger and process-isolated saturation."),
        ]),
    "session": dict(
        n=5, rel=f"{BIN}/🔐️session/🔬️unit/🦀️.rs", mode="include", lib=f"{BIN}/🔐️session/🏛️laws/🦀️.rs", pkg="semio-hub-session-route-laws",
        children=[
            dict(folder="🔑️credential-sign-in-laws", mod="credential_sign_in_laws", ranges=[(47, 336)],
                 doc="🔑️ Laws of credential sign-in: failing stores, wrong credentials, lockout, rate limits, sign-out and expiry."),
            dict(folder="🤖️agent-delegation-laws", mod="agent_delegation_laws", ranges=[(338, 838)],
                 doc="🤖️ Laws of agent delegation: minted sessions, revocation, withdrawn delegations, audience bounds and vigilant concurrent edits."),
            dict(folder="🛡️production-posture-laws", mod="production_posture_laws", ranges=[(840, 1093)],
                 doc="🛡️ Laws of the production posture: origins, proxies, forwarding headers, credential authority and shutdown."),
            dict(folder="📝️observability-laws", mod="observability_laws", ranges=[(1095, 1257)],
                 doc="📝️ Laws of observability: session spans, the admin route, route templates, limiter counters and the readiness record."),
            dict(folder="🗄️instance-store-laws", mod="instance_store_laws", ranges=[(1259, 1413)],
                 doc="🗄️ Laws of instance stores: sessions across restart, principal revocation, the saga drain and the wire frontier names."),
        ]),
    "openplan": dict(
        n=6, rel=f"{BIN}/🧭️open-plan/🔬️unit/🦀️.rs", mode="include", lib=f"{BIN}/🧭️open-plan/🏛️laws/🦀️.rs", pkg="semio-hub-open-plan-laws",
        children=[
            dict(folder="🧰️plan-fixtures", mod="plan_fixtures", ranges=[(48, 122)],
                 doc="🧰️ Plan fixtures: openable document publication, the plan secret and the plan authorities for a scope and a session."),
            dict(folder="🎟️plan-ledger-and-exchange-laws", mod="plan_ledger_and_exchange_laws", ranges=[(124, 327)],
                 doc="🎟️ Laws of the document-open plan ledger and the receipt exchange of a bounded socket grant."),
            dict(folder="🎯️execution-target-laws", mod="execution_target_laws", ranges=[(329, 611)],
                 doc="🎯️ Laws of execution-target asset routes, the selection final fence and refusals without genesis."),
            dict(folder="🛣️plan-route-laws", mod="plan_route_laws", ranges=[(613, 1032)],
                 doc="🛣️ Laws of the plan issue and exchange routes, socket consume, late invalid receipts and admin revocation."),
            dict(folder="🔁️supersession-laws", mod="supersession_laws", ranges=[(1034, 1421)],
                 doc="🔁️ Laws of foreign history transitions, admitted supersessions and concurrent supersession convergence."),
            dict(folder="🔌️socket-grant-laws", mod="socket_grant_laws", ranges=[(1423, 1627)],
                 doc="🔌️ Laws of socket grant routes: exact replay, revoke linearization, lag and broadcast fences and directory hello."),
            dict(folder="🗂️scoped-directory-socket-laws", mod="scoped_directory_socket_laws", ranges=[(1629, 1841)],
                 doc="🗂️ Laws of the scoped directory socket: in-flight commands, membership ledger, message matching, scope substitution and removal."),
        ]),
    "testkit": dict(
        n=7, rel=f"{BIN}/🧰️testkit/🦀️.rs", mode="mod", lib=f"{BIN}/🧰️testkit/🦀️.rs", pkg="semio-hub-route-testkit",
        children=[
            dict(folder="🎁️stdio-bundle", mod="stdio_bundle", ranges=[(54, 230)],
                 doc="🎁️ The native openable stdio bundle fixture: a published trusted-catalog generation under a fixture-owned data root."),
            dict(folder="⏱️law-guards", mod="law_guards", ranges=[(232, 256), (268, 343)],
                 doc="⏱️ Law guards: fresh temp directories, the socket test runner thread and the hang watchdogs with their bounded steps."),
            dict(folder="🏗️test-states", mod="test_states", ranges=[(258, 266), (345, 512)],
                 doc="🏗️ Test states: observed, capacity-bounded, directory-backed and lag-ready hub states."),
            dict(folder="🌱️document-seeding", mod="document_seeding", ranges=[(51, 52), (514, 759)],
                 doc="🌱️ Document seeding: spaces, members, descriptors, announced documents, checkpoints and genesis publication."),
            dict(folder="🌐️servers-and-sockets", mod="servers_and_sockets", ranges=[(761, 1013)],
                 doc="🌐️ Servers and sockets: spawned servers, raw HTTP, server frames, close codes, socket hellos and requests."),
            dict(folder="🪪️sessions-and-open-plans", mod="sessions_and_open_plans", ranges=[(1015, 1235)],
                 doc="🪪️ Sessions and document-open plans: issued sessions, admin tokens, plan authorities, catalogs and directory commands."),
        ]),
}
ORDER = ["cas", "boot", "space", "hostile", "session", "openplan", "testkit"]

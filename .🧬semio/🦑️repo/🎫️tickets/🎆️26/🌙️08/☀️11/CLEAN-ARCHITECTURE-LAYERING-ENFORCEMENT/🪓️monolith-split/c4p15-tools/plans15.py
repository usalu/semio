"""🗺️ c4p15 split plans for CENSUS4-PKG-15 (hub rust). `n` is the snapshot number in `orig/`, `rel` the repo path of the target.
`mode` "mod": production roots; children are `#[path]` modules (`use super::*;`, root `<vis> use <mod>::*;`, private definitions that another file names become `pub(super)`).
`mode` "include": test roots; children are `include!` fragments, so the module identity and every test path stay identical.
`ranges` are 1-based inclusive line ranges of the snapshot (docs, attributes and region markers included). `members` are ranges inside the inherent `impl` whose head is line `impl`; the child wraps them in that head and a closing brace, and the root drops head and closing line (`impl_span`)."""

H = "🌎️hub"
TC = f"{H}/🗿️artifact-authority/🔏️trusted-catalog"
INF = f"{H}/💡️inference"
DIR = f"{H}/📇️directory"
ADM = f"{H}/🔨️modules/🛡️admin"

PLANS = {
    "tc": dict(
        n=5, rel=f"{TC}/🦀️.rs", mode="mod", lib=f"{TC}/🦀️.rs", pkg="semio-hub-trusted-catalog",
        children=[
            dict(folder="📮️publisher", mod="publisher", ranges=[(28, 94)],
                 doc="📮️ Trusted-catalog publication: the cooperating-writer fence that publishes only a fully verified selection and its durable or unconfirmed outcome."),
            dict(folder="🧊️guest-residency", mod="guest_residency", ranges=[(96, 403), (1579, 1585)],
                 doc="🧊️ Compiled-guest residency: the codec budget, the memory-budgeted residency ledger with its slots and the footprint port."),
            dict(folder="📈️load-progress", mod="load_progress", ranges=[(405, 453)],
                 doc="📈️ The live load progress of one catalog, written by its load and its background verification and read for readiness and observability."),
            dict(folder="🗜️guest-codec", mod="guest_codec", ranges=[(455, 676)],
                 doc="🗜️ Guest codec calls: a verified package's lazily compiled actor, its pinned codec rows and the off-worker interpreter with cancellation."),
            dict(folder="🔗️native-codec", mod="native_codec", ranges=[(678, 766)],
                 doc="🔗️ One verified artifact identity bound to its executable: the native or guest codec behind the trusted-codec ports."),
            dict(folder="📚️verified-catalog", mod="verified_catalog", ranges=[(768, 998)],
                 doc="📚️ The process-lifetime verified catalog: packages, codecs, open targets, plugin modules, creation selection and document-open resolution."),
            dict(folder="🗃️verification-cache", mod="verification_cache", ranges=[(1000, 1084)],
                 doc="🗃️ The content-addressed memory of guest codec verifications and the verification concurrency bound."),
            dict(folder="🏗️loader", mod="loader", ranges=[(1086, 1493)],
                 doc="🏗️ The trusted-catalog loader: current-generation load, fixture load and the verification of one selected bundle into a verified catalog."),
            dict(folder="🎭️fixture-builders", mod="fixture_builders", ranges=[(1495, 1577)], cfg='#[cfg(any(test, feature = "integration-fixtures"))]',
                 doc="🎭️ Fixture builders: never-executed plugin module files, component receipts and the headless stdio fixture package."),
        ]),
    "desc": dict(
        n=7, rel=f"{TC}/🧾️descriptor/🦀️.rs", mode="mod", lib=f"{TC}/🧾️descriptor/🦀️.rs", pkg="semio-hub-trusted-catalog-descriptor",
        children=[
            dict(folder="🔌️native-provider", mod="native_provider", ranges=[(27, 88), (439, 451), (1016, 1029)],
                 doc="🔌️ The native codec provider vocabulary: explicit bindings, the borrowed package identity, the requirement, the provider port and the binding validation."),
            dict(folder="💾️retained-assets", mod="retained_assets", ranges=[(90, 437)],
                 doc="💾️ Retained assets of a verified package: hash-verified files read on demand, asset streams, plugin modules and the verified package and selection records."),
            dict(folder="🎯️open-targets", mod="open_targets", ranges=[(453, 676)],
                 doc="🎯️ Document-open target derivation: the one pairing rule, hosted codec targets, creation preference, the most general dialect and the sort key."),
            dict(folder="📋️bundle-validation", mod="bundle_validation", ranges=[(21, 25), (678, 1014)],
                 doc="📋️ Bundle selection and validation: the selected closure, profile generation ids and the whole trust-bundle checks."),
            dict(folder="🔍️descriptor-validation", mod="descriptor_validation", ranges=[(1031, 1145)],
                 doc="🔍️ Package descriptor validation: the semantic descriptor against its trust record, strict decoding and the load progress report."),
        ]),
    "ledger": dict(
        n=2, rel=f"{INF}/🪶️sqlite/🦀️.rs", mode="mod", lib=f"{INF}/🪶️sqlite/🦀️.rs", pkg="semio-hub-inference-ledger",
        impl=312, impl_span=(312, 1393),
        children=[
            dict(folder="🧱️ledger-storage", mod="ledger_storage", ranges=[(11, 112), (273, 310)],
                 doc="🧱️ Ledger storage: the bounded SQL schema and the small row and error helpers."),
            dict(folder="🧾️ledger-types", mod="ledger_types", ranges=[(114, 145), (223, 271)],
                 doc="🧾️ Ledger rows and views: the ledger handle, job rows, run claims, progress and event rows, event pages and the reader."),
            dict(folder="🎬️job-lifecycle", mod="job_lifecycle", ranges=[], members=[(313, 791), (1355, 1392)],
                 doc="🎬️ Job lifecycle: open, accept, start, heartbeat, terminal transitions, reconcile, events and reads."),
            dict(folder="✅️approvals", mod="approvals", ranges=[(164, 203)], members=[(793, 1019)],
                 doc="✅️ Approval outbox: the reconciliation vocabulary, prepared approvals, recovery by mutation and reconciliation of committed approvals."),
            dict(folder="🗺️gis-map-undo", mod="gis_map_undo", ranges=[(147, 162), (205, 221)], members=[(1021, 1353)],
                 doc="🗺️ GIS map approval undo: the undo target, admission and recovery records and their ledger transitions."),
        ]),
    "runtime": dict(
        n=1, rel=f"{INF}/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs", mode="include", lib=f"{INF}/🏃️runtime/🧪️tests/🏛️laws/🦀️.rs", pkg="semio-hub-inference-laws",
        children=[
            dict(folder="🧰️law-fixtures", mod="law_fixtures", ranges=[(20, 361)],
                 doc="🧰️ Law fixtures: ledger and identity builders, canonical approval and map inputs, ordered publishers and the approval poll harness."),
            dict(folder="🤝️handoff-and-owner-laws", mod="handoff_and_owner_laws", ranges=[(363, 550)],
                 doc="🤝️ Laws of abandoned approval handoff and cleanup, proposal ownership and the fail-closed composition transaction."),
            dict(folder="📮️committed-publication-laws", mod="committed_publication_laws", ranges=[(552, 863)],
                 doc="📮️ Law of a committed approval event reaching the actor frontier and the public checkpoint before the ledger applies it."),
            dict(folder="🔚️terminal-and-comparison-laws", mod="terminal_and_comparison_laws", ranges=[(865, 1077)],
                 doc="🔚️ Laws of terminal close fencing new admission and of the frozen comparison limits and error vocabulary."),
            dict(folder="⚡️quick-and-binding-laws", mod="quick_and_binding_laws", ranges=[(1079, 1611)],
                 doc="⚡️ The quick laws module and the non-default proposal binding law."),
        ]),
    "dirunit": dict(
        n=3, rel=f"{DIR}/🧪️tests/🔬️unit/🦀️.rs", mode="include", lib=f"{DIR}/🧪️tests/🏛️laws/🦀️.rs", pkg="semio-hub-directory-laws",
        children=[
            dict(folder="🪪️identity-laws", mod="identity_laws", ranges=[(24, 136)],
                 doc="🪪️ Laws of typed capabilities and of the identity verifier port."),
            dict(folder="🧰️artifact-fixtures", mod="artifact_fixtures", ranges=[(138, 499)],
                 doc="🧰️ Artifact fixtures: descriptors, checkpoints, ownership plans, genesis publication and the CAS and rebuild probes."),
            dict(folder="📒️admin-and-event-log-laws", mod="admin_and_event_log_laws", ranges=[(501, 688)],
                 doc="📒️ Laws of the admin operation audit, bounded overviews, the shared writer guard and event log replay."),
            dict(folder="🧺️artifact-cas-laws", mod="artifact_cas_laws", ranges=[(690, 1227)],
                 doc="🧺️ Laws of artifact chunk CAS retention, sweeps and races, checkpoint publication and the exact bounds of public scalars."),
            dict(folder="🏘️space-and-membership-laws", mod="space_and_membership_laws", ranges=[(1229, 1392)],
                 doc="🏘️ Laws of document descriptors, space authorship and archive, membership and invites."),
            dict(folder="🎟️invite-and-index-laws", mod="invite_and_index_laws", ranges=[(1394, 1779)],
                 doc="🎟️ Laws of invite redemption and archive projections, uncertain commits, the exhaustive module and the document index."),
        ]),
    "tcunit": dict(
        n=6, rel=f"{TC}/🧪️tests/🔬️unit/🦀️.rs", mode="include", lib=f"{TC}/🧪️tests/🏛️laws/🦀️.rs", pkg="semio-hub-trusted-catalog-laws",
        children=[
            dict(folder="🧊️guest-residency-and-codec-laws", mod="guest_residency_and_codec_laws", ranges=[(15, 463)],
                 doc="🧊️ Laws of guest residency bounds, the verification memory, the residency ledger and off-worker guest codec calls."),
            dict(folder="📮️publication-laws", mod="publication_laws", ranges=[(465, 708)],
                 doc="📮️ Laws of descriptor wire materialization and of trusted publication revisions, races and leaf substitution."),
            dict(folder="🎭️fixture-harness", mod="fixture_harness", ranges=[(710, 1279)],
                 doc="🎭️ Fixture harness: control and provider doubles, descriptor and bundle builders, linked-fixture guards and prepared fixtures."),
            dict(folder="🧭️provider-selection-laws", mod="provider_selection_laws", ranges=[(1281, 1455)],
                 doc="🧭️ Laws of native provider selection, descriptor verification, failure substitution and the neutral dependency-order fixture."),
            dict(folder="🛡️loader-trust-laws", mod="loader_trust_laws", ranges=[(1457, 1912)],
                 doc="🛡️ Laws of exact byte retention, execution-target assets, document-open resolution, opened roots and trust failures before activation."),
            dict(folder="🧪️owned-guest-laws", mod="owned_guest_laws", ranges=[(1914, 2188)],
                 doc="🧪️ Laws of owned test guests: served codec rows, guest tables and the per-package load progress."),
            dict(folder="🎯️open-target-laws", mod="open_target_laws", ranges=[(2190, 2528)],
                 doc="🎯️ Laws of the one pairing rule for open targets, bundle declarations, profile generation and the long laws module."),
            dict(folder="📦️plugin-module-laws", mod="plugin_module_laws", ranges=[(2530, 2769)],
                 doc="📦️ Laws of served plugin module files, hosted open targets, creation preference and the most general dialect."),
        ]),
    "dist": dict(
        n=4, rel=f"{ADM}/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs", mode="include", lib=f"{ADM}/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs", pkg=None,
        children=[
            dict(folder="🧰️shard-harness", mod="shard_harness", ranges=[(5, 103)],
                 doc="🧰️ Shard harness: the loopback probe, pump and decode helpers, envelope encoders and test job doubles."),
            dict(folder="🚀️pump-and-spawn-laws", mod="pump_and_spawn_laws", ranges=[(105, 362)],
                 doc="🚀️ Laws of the shard pump, unregistered actors, spawned and cancelled job effects."),
            dict(folder="⏸️suspend-resume-laws", mod="suspend_resume_laws", ranges=[(364, 616)],
                 doc="⏸️ Laws of suspend and resume placement, checkpoints, cancellation and lifecycle barriers."),
            dict(folder="📨️frame-budget-and-bridge-laws", mod="frame_budget_and_bridge_laws", ranges=[(618, 982)],
                 doc="📨️ Laws of shard frame round trips, granted budgets, register and unregister frames and the budget bridge."),
            dict(folder="🚦️lane-priority-and-replay-laws", mod="lane_priority_and_replay_laws", ranges=[(984, 1476)],
                 doc="🚦️ Laws of lane priority, epoch yield, fixed owner rings, terminal capacity and mounted replay."),
        ]),
}
ORDER = ["tc", "desc", "ledger", "runtime", "dirunit", "tcunit", "dist"]

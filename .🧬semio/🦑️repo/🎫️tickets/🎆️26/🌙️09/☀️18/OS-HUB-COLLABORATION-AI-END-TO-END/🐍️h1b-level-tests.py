"""🎚️ Moves named hub laws into their measured `mod quick|long|exhaustive` level submodule.

Every law keeps its attributes and body verbatim; only its enclosing module changes, which is what
`runCargoTestBudgeted`'s cumulative `--skip <level>::` filters read. Run from the repository root.
"""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

PLAN: dict[str, list[tuple[str, str]]] = {}


def plan(level: str, path: str, *names: str) -> None:
    PLAN.setdefault(path, []).extend((level, name) for name in names)


AUTH = "🌎️hub/🔐️auth/🧪️tests/🔬️unit/🦀️.rs"
CAS = "🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🧪️tests/🔬️unit/🦀️.rs"
PROVIDER = "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️tests/🔬️unit/🦀️.rs"
CATALOG = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs"
RUNTIME = "🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs"
WAL = "🌎️hub/💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs"
CHAIN = "🌎️hub/💡️inference/🧾️wal/🧪️tests/⛓️chain/🦀️.rs"
ORACLE = "🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🔮️oracles/🦀️.rs"
CREATION = "🌎️hub/📇️directory/🪶️sqlite/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs"
DIRECTORY = "🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs"
BIN = "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"

plan("quick", PROVIDER,
     "linked_consumer_descriptors_bind_their_actual_compiled_stdio_dependency_and_catalog",
     "native_openable_provider_rejects_identity_hash_schema_and_factory_substitution",
     "native_openable_provider_rejects_missing_extra_and_duplicate_receipts_without_publication",
     "linked_provider_set_previews_only_the_selected_packages_of_a_stdio_gis_or_stdio_gis_vcs_profile")
plan("quick", AUTH, "pbkdf2_sha256_matches_third_party_vectors")
plan("long", AUTH, "the_rust_decoders_and_the_json_schema_admit_exactly_the_same_wire")
plan("quick", CAS,
     "artifact_chunk_cas_filesystem_roundtrip_restart_and_collision_checks",
     "artifact_chunk_cas_memory_roundtrip_crosses_legacy_payload_ceiling",
     "artifact_chunk_cas_sqlite_roundtrip_crosses_legacy_payload_ceiling")
plan("long", CAS, "artifact_chunk_cas_neutral_fixture_matches_repository_sha256")
plan("long", CATALOG,
     "gis_map_binding_constructs_from_loaded_catalog_and_retains_verified_bytes",
     "linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure")
plan("quick", RUNTIME, "gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer")
plan("quick", WAL, "inference_wal_proof_dropped_caller_cancels_and_finishes_retained_replay_before_release")
plan("quick", CHAIN, "inference_wal_chain_cancellation_retires_hashing_and_compacted_suffix_is_not_a_genesis_proof")
plan("quick", ORACLE, "hub_socket_grant_fixture_serde_parity")
plan("quick", CREATION, "genesis_neutral_transactions_are_atomic_replayable_and_authorized")
plan("long", CREATION, "genesis_accepted_only_recovery_has_no_prepared_or_public_side_effects")
plan("exhaustive", DIRECTORY, "artifact_chunk_cas_opaque_continuation_converges_after_page_overflow_cancel_and_resume")
plan("quick", BIN,
     "socket_grant_revoke_before_command_admission_has_no_storage_effect",
     "retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact",
     "native_openable_stdio_provider_is_the_only_atomic_readiness_transition",
     "checkpoint_publication_route_is_author_owned_actor_fenced_idempotent_and_cancellation_safe",
     "scoped_directory_socket_removal_and_delivery_have_one_total_membership_order",
     "checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication",
     "socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke")
plan("long", BIN,
     "admin_response_pages_stop_before_exact_byte_max_and_reject_one_oversized_row",
     "socket_grant_ledger_is_bounded_single_consume_restart_scoped_and_revoke_race_safe",
     "artifact_cas_maintenance_checkpoint_reaches_tail_after_sixteen_requests")

LEVEL_ORDER = ["quick", "long", "exhaustive"]


def extract(lines: list[str], name: str) -> tuple[int, int, list[str]]:
    head = None
    for index, line in enumerate(lines):
        if line.startswith(f"fn {name}(") or line.startswith(f"async fn {name}("):
            head = index
            break
    if head is None:
        raise SystemExit(f"law not found at column zero: {name}")
    start = head
    while start > 0 and (lines[start - 1].startswith("#[") or lines[start - 1].startswith("///")):
        start -= 1
    end = head
    while end < len(lines) and lines[end] != "}":
        end += 1
    if end == len(lines):
        raise SystemExit(f"law has no column-zero terminator: {name}")
    block = lines[start:end + 1]
    if any('r#"' in line for line in block):
        raise SystemExit(f"law carries a raw string literal, refuse to reindent: {name}")
    return start, end + 1, block


def main() -> None:
    for path, entries in PLAN.items():
        file = ROOT / path
        source = file.read_text(encoding="utf-8")
        lines = source.split("\n")
        moved: dict[str, list[list[str]]] = {}
        for level, name in entries:
            start, stop, block = extract(lines, name)
            del lines[start:stop]
            while start > 0 and lines[start - 1] == "" and (start >= len(lines) or lines[start] == ""):
                del lines[start - 1]
                start -= 1
            moved.setdefault(level, []).append(block)
        while lines and lines[-1] == "":
            lines.pop()
        for level in LEVEL_ORDER:
            if level not in moved:
                continue
            lines.append("")
            lines.append(f"mod {level} {{")
            lines.append("    use super::*;")
            for block in moved[level]:
                lines.append("")
                lines.extend("    " + line if line else "" for line in block)
            lines.append("}")
        lines.append("")
        file.write_text("\n".join(lines), encoding="utf-8")
        print(f"{path}: {', '.join(f'{level}={len(blocks)}' for level, blocks in sorted(moved.items()))}")


if __name__ == "__main__":
    sys.exit(main())

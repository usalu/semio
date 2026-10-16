"""hdc (HUB-DIRECTORY-CAPABILITIES) shared helpers: repo paths, the capability partition of the HubDirectory port, module tree and trait-scope analysis."""
import os, re, subprocess, collections, json

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
SC = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
DIR = ROOT + "/🌎️hub/📇️directory"
BACKENDS = {"🪶️sqlite": "SqliteDirectory", "🐘️postgres": "PostgresDirectory", "🌐️neo4j": "Neo4jDirectory"}

# section -> ordered list of (trait, [methods], optional default-bodied method names that stay in the trait)
# Each trait lists every method name in port order. DEFAULTED = methods with a body in the trait that no backend implements.
SECTIONS = collections.OrderedDict()
SECTIONS["🎟️capabilities"] = [
    ("ShareTokenDirectory", ["issue_share_token_as", "issue_share_token_as_with_admin_effect", "issue_share_token", "revoke_share_token_as", "revoke_share_token_as_with_admin_effect", "revoke_share_token", "authenticate_share", "authenticate_share_binding", "socket_share_binding"]),
    ("InviteDirectory", ["issue_invite_as", "issue_invite_as_with_admin_effect", "issue_invite", "invite_redemption_scope_hint", "redeem_invite_atomic", "revoke_invite_as", "revoke_invite_as_with_admin_effect", "revoke_invite", "list_invites", "list_space_administration_invites_page"]),
]
SECTIONS["👤️accounts"] = [
    ("UserDirectory", ["create_user", "get_user", "get_user_by_email", "get_user_by_sso_subject", "list_users"]),
    ("AuthSessionDirectory", ["issue_auth_session", "authenticate_session", "socket_session_binding", "revoke_auth_session", "revoke_auth_sessions_for_user", "revoke_auth_sessions_for_user_with_admin_effect", "revoke_auth_sessions_for_identity", "list_auth_audit"]),
    ("CredentialDirectory", ["append_credential_audit", "set_password_credential"]),
    ("AgentDelegationDirectory", ["create_agent_delegation", "list_agent_delegations", "revoke_agent_delegation", "load_agent_delegation", "agent_delegation"]),
]
SECTIONS["🏘️spaces"] = [
    ("SpaceDirectory", ["get_space", "list_spaces_for_user", "list_spaces", "list_admin_space_summaries_page", "list_visible_space_summaries", "list_admin_space_members_page", "list_members", "get_role", "list_space_administration_members_page"]),
    ("DocumentDirectory", ["get_document_descriptor", "list_document_descriptors", "list_document_descriptors_page", "get_artifact_checkpoint", "get_verified_artifact_checkpoint", "get_active_artifact_checkpoint", "get_artifact_retention", "artifact_checkpoint_count", "list_artifact_checkpoint_lineage"]),
]
SECTIONS["🏭️artifact-creation"] = [
    ("ArtifactCreationDirectory", ["claim_artifact_creation", "read_artifact_creation", "append_artifact_creation_fact", "append_document_genesis", "artifact_creation_recovery_candidates", "artifact_creation_terminate_uncommitted"]),
]
SECTIONS["🧺️artifact-cas"] = [
    ("ArtifactCasDirectory", ["reserve_artifact_cas", "append_reserved_artifact_checkpoint", "artifact_cas_ledger_generation", "artifact_cas_coordinator_id", "artifact_cas_sweep_candidates", "artifact_cas_delete_preview_protected", "acquire_artifact_cas_delete_fence", "validate_artifact_cas_delete_fence", "renew_artifact_cas_delete_fence", "release_artifact_cas_delete_fence"]),
]
SECTIONS["🛡️administration"] = [
    ("AdminOperationDirectory", ["admin_overview_counts", "append_admin_operation_audit", "admin_operation_audit_for_request", "admin_operation_audit_for_operation", "admin_operation_effect_receipt", "list_admin_operation_audit"]),
    ("ReceiptDirectory", ["claim_or_read_directory_command_receipt", "complete_directory_command_receipt", "release_directory_command_receipt", "claim_or_read_checkpoint_publication", "release_checkpoint_publication"]),
]
SECTIONS["🔄️sync-sessions"] = [
    ("SyncSessionDirectory", ["record_sync_session_open", "record_sync_session_close", "list_sync_sessions_for_document", "list_active_sync_sessions_page", "list_active_sync_sessions", "close_all_sync_sessions"]),
]
SECTIONS["📜️event-log"] = [
    ("EventLog", ["append_events", "append_decided_events", "append_decided_events_with_admin_effect", "events_since", "head_seq"]),
]
SECTIONS["📽️projections"] = [
    ("ProjectionRebuild", ["rebuild_projections", "rebuild_projections_controlled"]),
]
# blanket extension trait, not implemented by backends
EXTENSIONS = [("PrincipalAuthority", ["principal_ceiling", "principal_role"])]
# methods whose trait definition carries a body and that backends never implement
TRAIT_DEFAULTED = {"issue_share_token", "revoke_share_token", "issue_invite", "revoke_invite", "list_active_sync_sessions", "append_events"}

TRAIT_OF = {}
SECTION_OF_TRAIT = {}
for sec, traits in SECTIONS.items():
    for t, ms in traits:
        SECTION_OF_TRAIT[t] = sec
        for m in ms:
            assert m not in TRAIT_OF, m
            TRAIT_OF[m] = t
for t, ms in EXTENSIONS:
    for m in ms:
        assert m not in TRAIT_OF, m
        TRAIT_OF[m] = t
ALL_TRAITS = [t for sec in SECTIONS.values() for t, _ in sec] + [t for t, _ in EXTENSIONS]


def read(p):
    with open(p, encoding="utf-8") as f:
        return f.read()


def write(p, text):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w", encoding="utf-8") as f:
        f.write(text)


def hub_files():
    a = subprocess.run(["git", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "--", "🌎️hub/*.rs"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    b = subprocess.run(["git", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "--others", "--exclude-standard", "--", "🌎️hub"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    fs = sorted({f for f in a + b if f.endswith(".rs") and f and "/node_modules/" not in f})
    return [f for f in fs if os.path.exists(os.path.join(ROOT, f))]


import functools, hashlib, pickle, atexit

_MC = os.path.join(SC, "hdc-mask-cache.pkl")
_MASKS = {}
_DIRTY = [False]
try:
    with open(_MC, "rb") as _f:
        _MASKS = pickle.load(_f)
except Exception:
    _MASKS = {}


def _save_masks():
    if _DIRTY[0]:
        os.makedirs(SC, exist_ok=True)
        with open(_MC, "wb") as f:
            pickle.dump(_MASKS, f)


atexit.register(_save_masks)


def mask(text):
    """Blank comments, string and char literals (length preserving); memoised on disk by content hash."""
    key = hashlib.sha1(text.encode("utf-8")).hexdigest()
    hit = _MASKS.get(key)
    if hit is None:
        hit = _mask(text)
        _MASKS[key] = hit
        _DIRTY[0] = True
    return hit


def _mask(text):
    """Blank comments, string and char literals (length preserving)."""
    out = list(text)
    n = len(text)
    i = 0
    while i < n:
        c = text[i]
        if c == "/" and text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                out[k] = " "
            i = j
            continue
        if c == "/" and text.startswith("/*", i):
            d, j = 1, i + 2
            while j < n and d:
                if text.startswith("/*", j):
                    d += 1; j += 2
                elif text.startswith("*/", j):
                    d -= 1; j += 2
                else:
                    j += 1
            for k in range(i, j):
                if out[k] != "\n":
                    out[k] = " "
            i = j
            continue
        m = re.match(r'b?r(#*)"', text[i:i + 40]) if c in "br" else None
        if m and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            close = '"' + m.group(1)
            j = text.find(close, i + m.end())
            j = n if j < 0 else j + len(close)
            for k in range(i, j):
                if out[k] != "\n":
                    out[k] = " "
            i = j
            continue
        if c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j = min(n, j + 1)
            for k in range(i, j):
                if out[k] != "\n":
                    out[k] = " "
            i = j
            continue
        if c == "'":
            m = re.match(r"'(\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", text[i:i + 16])
            if m:
                for k in range(i, i + m.end()):
                    out[k] = " "
                i += m.end()
                continue
        i += 1
    return "".join(out)


def split_top(s):
    parts, cur, depth = [], "", 0
    for ch in s:
        if ch == "{":
            depth += 1
        if ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    return parts


def merge_use_list(line, remove, add):
    """`use a::b::{x, y};` on one line: drop `remove` names, add `add` names, ASCII-sort when every item is a plain identifier."""
    m = re.match(r"^(\s*(?:pub(?:\([^)]*\))?\s+)?use\s+[^{;]*::)\{(.*)\};\s*$", line)
    assert m, line
    items = [x for x in split_top(m.group(2)) if x not in remove]
    for a in add:
        if a not in items:
            items.append(a)
    if all(re.match(r"^\w+$", x) for x in items):
        items = sorted(items)
    return m.group(1) + "{" + ", ".join(items) + "};"

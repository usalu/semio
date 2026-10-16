"""Generates the capability-trait port from the original HubDirectory port into a stage dir (never into the repo)."""
import os, re, sys, shutil
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
os.environ["HDC_PORT_ORIG"] = SC + "/hdc-orig/🌎️hub/📇️directory/🔌️port/🦀️.rs"
from portparse import *

STAGE = SC + "/hdc-stage"

TRAIT_DOCS = {
    "ShareTokenDirectory": "🔗️ Document share tokens: issue, revoke and authenticate the capabilities that grant access to one document scope.",
    "InviteDirectory": "🎟️ Space invitations: issue, redeem and revoke the capabilities that grant membership of one space.",
    "UserDirectory": "👤️ User accounts: creation and the lookups by id, email and single-sign-on subject.",
    "AuthSessionDirectory": "🔐️ Authentication sessions: issue, authenticate and revoke them, bind sockets to them, and read their audit log.",
    "CredentialDirectory": "🔑️ Credential lifecycle: the sign-in and credential-change audit facts and the password credential itself.",
    "AgentDelegationDirectory": "🤖️ Agent delegations: a user's capability to let an agent act in one space. A backend without delegation storage opts out with an empty impl and keeps these defaults.",
    "SpaceDirectory": "🏘️ Spaces and memberships: the lookups, rosters and administration pages over spaces.",
    "DocumentDirectory": "📄️ Document descriptors and the published artifact checkpoints of a document scope.",
    "PrincipalAuthority": "🧢️ The space authority of one principal: its role ceiling and the role it holds. Implemented for every directory that offers [`SpaceDirectory`] and [`AgentDelegationDirectory`].",
    "ArtifactCreationDirectory": "🏭️ Artifact creation: the claim, fact and genesis log of one creation request and its recovery.",
    "ArtifactCasDirectory": "🧺️ The artifact CAS ledger: reservations, sweep candidates and delete fences over private reachability.",
    "AdminOperationDirectory": "🛡️ Administrator operations: the overview counts, the operation audit and the private effect receipts.",
    "ReceiptDirectory": "🧾️ Idempotency receipts: directory command receipts and checkpoint publication claims.",
    "SyncSessionDirectory": "🔄️ Realtime sync sessions: the open and close bookkeeping and the presence reads.",
    "EventLog": "📜️ The append-only directory event log: ordered appends and the reads that replay it.",
    "ProjectionRebuild": "📽️ Projection rebuild: truncates every projection table and replays the whole event log.",
}
SECTION_DOCS = {
    "🎟️capabilities": "🎟️ Capability traits for share tokens and invitations.",
    "👤️accounts": "👤️ Capability traits for users, authentication sessions, credentials and agent delegations.",
    "🏘️spaces": "🏘️ Capability traits for spaces, memberships, document descriptors and artifact checkpoints.",
    "🏭️artifact-creation": "🏭️ The capability trait of artifact creation.",
    "🧺️artifact-cas": "🧺️ The capability trait of the artifact CAS ledger.",
    "🛡️administration": "🛡️ Capability traits for administrator operations and idempotency receipts.",
    "🔄️sync-sessions": "🔄️ The capability trait of realtime sync sessions.",
    "📜️event-log": "📜️ The capability trait of the directory event log.",
    "📽️projections": "📽️ The capability trait of the projection rebuild.",
}
KEEP_DEFAULT_TRAITS = {"AgentDelegationDirectory", "PrincipalAuthority"} | set()
PURE_DEFAULTS = TRAIT_DEFAULTED


def strip_underscore_params(sig_lines):
    out = []
    for l in sig_lines:
        out.append(re.sub(r"\b_([a-z][a-z0-9_]*)\s*:", r"\1:", l))
    return out


def required_form(member):
    """Turns an 'unavailable' default into a required method signature."""
    lines = member["lines"]
    text = "\n".join(lines)
    k = text.rindex("{") if False else None
    # find the opening brace of the body (first '{' after the signature ends with ') -> ... {')
    sig_end = None
    depth = 0
    m_text = mask(text)
    for i, ch in enumerate(m_text):
        if ch in "([":
            depth += 1
        elif ch in ")]":
            depth -= 1
        elif ch == "{" and depth == 0:
            sig_end = i
            break
    sig = text[:sig_end].rstrip()
    sig = "\n".join(strip_underscore_params(sig.split("\n")))
    return sig + ";"


def render_member(member, trait, extra_doc_fix):
    doc = [extra_doc_fix(l) for l in member["doc"]]
    name = member["name"]
    if member["has_body"] and name not in PURE_DEFAULTS and trait not in KEEP_DEFAULT_TRAITS:
        body = required_form(member).split("\n")
        return doc + body
    return doc + [extra_doc_fix(l) for l in member["lines"]]


def fix_doc(line):
    line = line.replace("[`HubDirectory::list_admin_space_summaries_page`]", "[`SpaceDirectory::list_admin_space_summaries_page`]")
    return line


def main():
    head, items, tail = port_members()
    members = {it["name"]: it for it in items if it["kind"] == "member"}
    assert len(members) == 99
    shutil.rmtree(STAGE, ignore_errors=True)
    # imports of the original port (kept for the children)
    imports = []
    for l in head:
        if l.startswith("use ") or l.startswith("//!") or l == "":
            imports.append(l)
    use_lines = [l for l in head if l.startswith("use ")]
    sections_out = {}
    for sec, traits in SECTIONS.items():
        out = ["//! " + SECTION_DOCS[sec], "", "use super::*;", ""]
        for trait, names in traits:
            out.append("//#region " + trait)
            out.append("/// " + TRAIT_DOCS[trait])
            out.append("pub trait %s: Send + Sync + 'static {" % trait)
            for n in names:
                out += render_member(members[n], trait, fix_doc)
            out.append("}")
            out.append("//#endregion " + trait)
            out.append("")
            if sec == "🏘️spaces" and trait == "DocumentDirectory":
                for t2, n2 in EXTENSIONS:
                    out.append("//#region " + t2)
                    out.append("/// " + TRAIT_DOCS[t2])
                    out.append("pub trait %s: SpaceDirectory + AgentDelegationDirectory {" % t2)
                    for n in n2:
                        out += render_member(members[n], t2, fix_doc)
                    out.append("}")
                    out.append("")
                    out.append("impl<T: SpaceDirectory + AgentDelegationDirectory> %s for T {}" % t2)
                    out.append("//#endregion " + t2)
                    out.append("")
        while out and out[-1] == "":
            out.pop()
        sections_out[sec] = "\n".join(out) + "\n"
    for sec, text in sections_out.items():
        write(os.path.join(STAGE, "🔌️port", sec, "🦀️.rs"), text)
    # port root
    root = ["//! 🔌️ The capability traits every persistence backend implements. The port is partitioned along the sections the backends share, so a caller bounds itself on exactly the capabilities it uses, for example `D: SpaceDirectory + EventLog`.", ""]
    root += [""] if False else []
    root += ["use super::*;"]
    root += [l for l in use_lines if l != "use super::*;"]
    root.append("")
    exports = []
    for sec, traits in SECTIONS.items():
        mod = re.sub(r"^[^\w]+", "", sec).replace("-", "_")
        mod = re.sub(r"[^\w]", "", sec.encode("ascii", "ignore").decode()).replace("-", "_") if False else sec
    MODNAME = {"🎟️capabilities": "capabilities", "👤️accounts": "accounts", "🏘️spaces": "spaces", "🏭️artifact-creation": "artifact_creation", "🧺️artifact-cas": "artifact_cas", "🛡️administration": "administration", "🔄️sync-sessions": "sync_sessions", "📜️event-log": "event_log", "📽️projections": "projections"}
    for sec, traits in SECTIONS.items():
        names = [t for t, _ in traits]
        if sec == "🏘️spaces":
            names += [t for t, _ in EXTENSIONS]
        root.append('#[path = "%s/🦀️.rs"]' % sec)
        root.append("mod %s;" % MODNAME[sec])
        root.append("pub use %s::{%s};" % (MODNAME[sec], ", ".join(names)))
        root.append("")
    while root and root[-1] == "":
        root.pop()
    write(os.path.join(STAGE, "🔌️port/🦀️.rs"), "\n".join(root) + "\n")
    print("staged", STAGE)


if __name__ == "__main__":
    main()

"""📟 Stages the native Store backbone inbox with separately funded receive continuations."""

import hashlib
import json
from pathlib import Path
import runpy

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
TRANSPORT = runpy.run_path(str(TICKET / "raw-spr-transport-stage.py"))
NATIVE = TRANSPORT["NATIVE"]
INBOX = NATIVE.with_name("📥️atomic-inbox.rs")
SENDER = NATIVE.with_name("📤️atomic-actor-send.rs")
OUT = TICKET / "🗑️generated/raw-store-inbox-overlay.json"


def stage():
    pre = {}
    post = {}

    def put(path, value):
        pre[str(path)] = hashlib.sha256((ROOT / path).read_bytes()).hexdigest() if (ROOT / path).exists() else None
        post[str(path)] = value

    source = (ROOT / NATIVE).read_text(encoding="utf8") if (ROOT / NATIVE).exists() else (TICKET / "raw-store-atomic-backbone.rs").read_text(encoding="utf8")
    anchor = "use std::mem::{ManuallyDrop, size_of};"
    assert source.count(anchor) == 1
    source = source.replace(anchor, anchor + '\n\n#[path = "📥️atomic-inbox.rs"]\nmod inbox;\npub use inbox::{AtomicBackboneInbox, AtomicDocumentBackboneWork, AtomicInboxStep};')
    anchor = "mod inbox;"
    source = source.replace(anchor, anchor + '\n#[path = "📤️atomic-actor-send.rs"]\nmod actor_send;')
    source = source.replace("    frame: ManuallyDrop<Option<Vec<u8>>>,", "    frame: ManuallyDrop<Option<Vec<u8>>>,\n    actor_wire: ManuallyDrop<Option<Vec<u8>>>,")
    source = source.replace("frame: ManuallyDrop::new(None), sent: false", "frame: ManuallyDrop::new(None), actor_wire: ManuallyDrop::new(None), sent: false")
    source = source.replace("        let Some(frame) = self.frame.as_ref().filter(|_| !self.sent) else",
                            "        if self.actor_wire.is_some() { return Err(VcsError::InverseRefused(invalid())); }\n        let Some(frame) = self.frame.as_ref().filter(|_| !self.sent) else")
    source = source.replace("        if self.frame.is_some() { self.frame_retirement() } else { self.sender.close_demand() }",
                            "        if self.actor_wire.is_some() { return self.actor_wire_close_demand(); }\n        if self.frame.is_some() { self.frame_retirement() } else { self.sender.close_demand() }")
    source = source.replace("        if self.frame.is_some() { self.retire_frame(effect) } else { self.sender.close(effect) }",
                            "        if self.actor_wire.is_some() { return self.close_actor_wire(effect); }\n        if self.frame.is_some() { self.retire_frame(effect) } else { self.sender.close(effect) }")
    source = source.replace("self.frame.is_none() && self.sender.terminal_is_empty()", "self.actor_wire.is_none() && self.frame.is_none() && self.sender.terminal_is_empty()")
    source = source.replace("self.frame.is_none() || std::thread::panicking()", "(self.frame.is_none() && self.actor_wire.is_none()) || std::thread::panicking()")
    anchor = "    pub fn seal(&mut self, binding: &[u8; 32], document: &str, actor: &str, effect: &mut RetainedAllocationEffect<'_>)\n        -> Result<(Option<VerifiedAtomicEdit>, RetainedCloneProgress), VcsError> {"
    assert source.count(anchor) == 1
    source = source.replace(anchor, """    pub fn can_seal(&self) -> bool { self.digest.is_some() || self.sections.as_ref().is_some_and(|owner| owner.complete().is_some()) }

    pub fn is_sealed(&self) -> bool { self.sealed }

    pub fn seal_at_document(&mut self, binding: &[u8; 32], document: &str, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<VerifiedAtomicEdit>, RetainedCloneProgress), VcsError> {
        self.seal_for_binding(binding, document, None, effect)
    }

""" + anchor + """
        self.seal_for_binding(binding, document, Some(actor), effect)
    }

    fn seal_for_binding(&mut self, binding: &[u8; 32], document: &str, actor: Option<&str>, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(Option<VerifiedAtomicEdit>, RetainedCloneProgress), VcsError> {""")
    source = source.replace("        let edit = decode_verified_atomic_backbone(&verified, document, actor).map_err(|error| VcsError::Deserialize(error.to_string()))?;",
        """        let edit = match actor {
            Some(actor) => decode_verified_atomic_backbone(&verified, document, actor),
            None => semio_os_spr::decode_verified_atomic_backbone_at_document(&verified, document),
        }.map_err(|error| VcsError::Deserialize(error.to_string()))?;""")
    put(NATIVE, source)
    put(INBOX, (TICKET / "raw-store-atomic-inbox.rs").read_text(encoding="utf8"))
    put(SENDER, (TICKET / "raw-store-atomic-actor-send.rs").read_text(encoding="utf8"))
    OUT.write_text(json.dumps({"preimages": pre, "candidate": post}, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(post)} native Store inbox continuation postimages")


if __name__ == "__main__":
    stage()

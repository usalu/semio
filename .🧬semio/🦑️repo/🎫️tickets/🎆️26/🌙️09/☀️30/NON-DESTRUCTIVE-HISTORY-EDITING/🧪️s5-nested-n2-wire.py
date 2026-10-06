#!/usr/bin/env python3
"""📮 S5-NESTED wave N2 (design §21.9, channel 23 "wave C"): the member address on the wire carries its owner path.

Usage: python3 🧪️s5-nested-n2-wire.py check|land|restore [--without-frames]
  check   -> verifies every anchor count, on the live files or — while N1 is not on disk — on N1's planned result
  land    -> needs N1 on disk; keeps pre-images under 🗑️generated/s5-nested/pre-n2/ and writes every file
  restore -> puts every pre-image back
`--without-frames` leaves out the two producers and the one consumer of `ChildPackEntry.owner` / `ChildHeadPackEntry.owner`
(the frame fields S5-CHANNEL adds); without the flag they are part of the wave and need CHANNEL's struct fields on disk to compile.
Exact-string replacements with counted anchors; any drift fails closed before the first write.
"""
import importlib.util
import pathlib
import shutil
import sys

TICKET = pathlib.Path(__file__).resolve().parent
REPO = TICKET.parents[6]
PRE = TICKET / "🗑️generated/s5-nested/pre-n2"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
RUNTIME = f"{PLUGIN}/🦀️.rs"
OPERATION_TEST = f"{PLUGIN}/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
SYNC_TEST = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs"

PATH_METHODS_ANCHOR = "        /// 🥾️ Every step from the document down to the member.\n"
PATH_METHODS = '''        /// 🧗️ The text of the owner's path — every step but the member's own; empty for a member of the document.
        pub fn owner_text(&self) -> String {
            Self(self.0[..self.0.len().saturating_sub(1)].to_vec()).to_string()
        }

        /// 🪢️ The path of the member at `(slot, child_id)` under the owner whose path text is `owner` (empty: the document).
        pub fn under(owner: &str, slot: impl Into<String>, child_id: impl Into<String>) -> Option<Self> {
            let owner = if owner.is_empty() { Self(Vec::new()) } else { Self::parse(owner)? };
            Some(owner.join(slot, child_id))
        }

'''

SEND_LANE_OLD = '''        /// 📮️ Sends one member's events on the document's backbone under that member's lane. The lane of a member another
        /// member owns has no address on the wire before channel 23 (design §21.9, wave N2): its events are refused here,
        /// never sent under a root member's lane and never dropped.
        async fn send_member_lane(&mut self, lane: &MemberKey, payload: Vec<u8>) -> Result<(), Fault> {
            if payload.is_empty() || self.store.backbone_ref().is_none() {
                return Ok(());
            }
            if !lane.owner.is_empty() {
                return Err(plugin_sdk_fault(format!("the events of member {}/{} of member {} have no lane on the backbone", lane.slot, lane.child_id, lane.owner)));
            }
            self.store.send_member_mutations(&lane.slot, &lane.child_id, payload).await.map_err(|error| error.into_fault())
        }
'''
SEND_LANE_NEW = '''        /// 📮️ Sends one member's events on the document's backbone under that member's lane: the owner-path text of the
        /// member that owns it (empty for the document itself) and its step there (design §21.9). A member whose owner is
        /// not live has no lane: its events are refused here, never sent under another lane and never dropped.
        async fn send_member_lane(&mut self, lane: &MemberKey, payload: Vec<u8>) -> Result<(), Fault> {
            if payload.is_empty() || self.store.backbone_ref().is_none() {
                return Ok(());
            }
            let Some(owner) = self.children.path_of(lane.borrowed()).map(|path| path.owner_text()) else {
                return Err(plugin_sdk_fault(format!("the events of member {}/{} of member {} have no lane on the backbone", lane.slot, lane.child_id, lane.owner)));
            };
            self.store.send_member_mutations(&owner, &lane.slot, &lane.child_id, payload).await.map_err(|error| error.into_fault())
        }
'''

INBOUND_OLD = '''            for (slot, child_id, envelopes) in inbound {
                let Some(entry) = self.children.get_mut(&(slot.clone(), child_id.clone())) else {
                    let projection = store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                    if !self.derivable_follow_awaits_retirement() && (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared.to_string() == slot && fields.child_id.to_string() == child_id) {
                        return Err(plugin_sdk_fault(format!("backbone member lane {slot}/{child_id} names no live composed member of this replica")));
                    }
                    continue;
                };
'''
INBOUND_NEW = '''            for (owner, slot, child_id, envelopes) in inbound {
                let lane = MemberPath::under(&owner, slot.as_str(), child_id.as_str()).and_then(|path| self.children.resolve(&path).map(|(key, _)| key));
                let entry = match lane.as_ref() {
                    Some(key) => self.children.member_mut(key.borrowed()),
                    None => None,
                };
                let Some(entry) = entry else {
                    let declared = if owner.is_empty() {
                        let projection = store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                        (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot.as_str() && fields.child_id == child_id.as_str())
                    } else {
                        match MemberPath::parse(&owner).and_then(|path| self.children.resolve(&path)) {
                            Some((_, owning)) => {
                                let projection = owning.member.child_restore_projection().map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                                (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot.as_str() && fields.child_id == child_id.as_str())
                            }
                            None => false,
                        }
                    };
                    if declared && !self.derivable_follow_awaits_retirement() {
                        let lane = if owner.is_empty() { format!("{slot}/{child_id}") } else { format!("{owner}/{slot}/{child_id}") };
                        return Err(plugin_sdk_fault(format!("backbone member lane {lane} names no live composed member of this replica")));
                    }
                    continue;
                };
'''

CHILD_PACKS_OLD = '''            for entry in self.children.entries() {
                let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
                entries.push(protocol::ChildPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), envelope_pack });
            }
            entries.sort_by(|left, right| (&left.slot, &left.child_id).cmp(&(&right.slot, &right.child_id)));
'''
CHILD_PACKS_NEW = '''            for (key, entry) in self.children.keyed_entries() {
                let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
                let owner = self.children.path_of(key).map(|path| path.owner_text()).unwrap_or_default();
                entries.push(protocol::ChildPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), envelope_pack, owner });
            }
            entries.sort_by(|left, right| (&left.owner, &left.slot, &left.child_id).cmp(&(&right.owner, &right.slot, &right.child_id)));
'''
CHILD_HEADS_OLD = '''            for entry in self.children.entries() {
                let head_pack = entry.member.document_pack_bytes().await.map_err(|error| error.into_fault())?;
                entries.push(protocol::ChildHeadPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), head_pack });
            }
            entries.sort_by(|left, right| (&left.slot, &left.child_id).cmp(&(&right.slot, &right.child_id)));
'''
CHILD_HEADS_NEW = '''            for (key, entry) in self.children.keyed_entries() {
                let head_pack = entry.member.document_pack_bytes().await.map_err(|error| error.into_fault())?;
                let owner = self.children.path_of(key).map(|path| path.owner_text()).unwrap_or_default();
                entries.push(protocol::ChildHeadPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), head_pack, owner });
            }
            entries.sort_by(|left, right| (&left.owner, &left.slot, &left.child_id).cmp(&(&right.owner, &right.slot, &right.child_id)));
'''
LOAD_CHILDREN_OLD = '''                        for entry in &entries {
                            let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), error))?;
'''
LOAD_CHILDREN_NEW = '''                        for entry in &entries {
                            if !entry.owner.is_empty() {
                                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), format!("child pack {}/{} belongs to member {}: a member's own children load with the document archive", entry.slot, entry.child_id, entry.owner)));
                            }
                            let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), error))?;
'''

RUNTIME_REPLACEMENTS = [
    (PATH_METHODS_ANCHOR, PATH_METHODS + PATH_METHODS_ANCHOR, 1),
    (
        "    /// A refused operation leaves that prefix unchanged and retains its typed owner in preparation.\n    #[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]\n    pub struct ChildEmit {\n        pub slot: String,\n",
        "    /// A refused operation leaves that prefix unchanged and retains its typed owner in preparation.\n    /// `owner` is the owner-path text ([`MemberPath`]) of the member that owns the target; empty when the document itself does.\n    #[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]\n    pub struct ChildEmit {\n        pub owner: String,\n        pub slot: String,\n",
        1,
    ),
    ("            ChildEmit { slot: slot.into(), child_id: child_id.into(), ops: Vec::with_capacity(capacity),", "            ChildEmit { owner: String::new(), slot: slot.into(), child_id: child_id.into(), ops: Vec::with_capacity(capacity),", 1),
    ("for value in [&mut self.op_schema.0,&mut self.child_id,&mut self.slot]{", "for value in [&mut self.op_schema.0,&mut self.child_id,&mut self.slot,&mut self.owner]{", 1),
    ("[&self.op_schema.0,&self.child_id,&self.slot].into_iter()", "[&self.op_schema.0,&self.child_id,&self.slot,&self.owner].into_iter()", 1),
    (SEND_LANE_OLD, SEND_LANE_NEW, 1),
    ("                self.store.send_member_mutations(slot, child_id, payload).await.map_err(|error| error.into_fault())?;\n", "                self.store.send_member_mutations(\"\", slot, child_id, payload).await.map_err(|error| error.into_fault())?;\n", 1),
    (INBOUND_OLD, INBOUND_NEW, 1),
    (
        "            for child in emit.child_emits.iter() {\n                if self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none() {\n",
        "            for child in emit.child_emits.iter() {\n                if !child.owner.is_empty() || self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none() {\n",
        1,
    ),
    (
        "            if let Some(missing) = children.iter().find(|child| self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none()) {\n",
        "            if let Some(missing) = children.iter().find(|child| !child.owner.is_empty() || self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none()) {\n",
        1,
    ),
]
FRAME_REPLACEMENTS = [
    (CHILD_PACKS_OLD, CHILD_PACKS_NEW, 1),
    (CHILD_HEADS_OLD, CHILD_HEADS_NEW, 1),
    (LOAD_CHILDREN_OLD, LOAD_CHILDREN_NEW, 1),
]

OPERATION_TEST_REPLACEMENTS = [
    ("        let mut child = ChildEmit {\n            slot: \"slot\".into(),\n", "        let mut child = ChildEmit {\n            owner: String::new(),\n            slot: \"slot\".into(),\n", 1),
]

STORE_REPLACEMENTS = [
    (
        "    /// member's exact `(slot, child_id)` lane identity. A composed document has exactly one replica\n",
        "    /// member's exact lane identity: the owner-path text of the member that owns it (`owner`, empty for the document\n    /// itself; design §21.9) and its `(slot, child_id)` step there. A composed document has exactly one replica\n",
        1,
    ),
    ("    Member {\n        slot: String,\n        child_id: String,\n        #[dsl(base64)]\n        envelopes: Vec<u8>,\n    },\n", "    Member {\n        owner: String,\n        slot: String,\n        child_id: String,\n        #[dsl(base64)]\n        envelopes: Vec<u8>,\n    },\n", 1),
    (
        "    pub async fn send_member_mutations(&mut self, slot: &str, child_id: &str, envelopes: Vec<u8>) -> Result<(), VcsError> {\n",
        "    pub async fn send_member_mutations(&mut self, owner: &str, slot: &str, child_id: &str, envelopes: Vec<u8>) -> Result<(), VcsError> {\n",
        1,
    ),
    (
        "backbone.send(BackboneMessage::Member { slot: slot.to_string(), child_id: child_id.to_string(), envelopes }).await;",
        "backbone.send(BackboneMessage::Member { owner: owner.to_string(), slot: slot.to_string(), child_id: child_id.to_string(), envelopes }).await;",
        1,
    ),
    ("    /// `(slot, child_id, envelopes)` — the composing app resolves the lane and folds it.\n", "    /// `(owner, slot, child_id, envelopes)` — the composing app resolves the lane and folds it.\n", 1),
    ("    pub fn take_member_inbound(&mut self) -> Vec<(String, String, Vec<u8>)> {\n", "    pub fn take_member_inbound(&mut self) -> Vec<(String, String, String, Vec<u8>)> {\n", 1),
    ("                BackboneMessage::Member { slot, child_id, envelopes } => Some((slot, child_id, envelopes)),\n", "                BackboneMessage::Member { owner, slot, child_id, envelopes } => Some((owner, slot, child_id, envelopes)),\n", 1),
    (
        "                BackboneMessage::Member { slot, child_id, envelopes } => {\n                    if !envelopes.is_empty() {\n                        *self.bytes = Some(std::mem::take(envelopes));\n                        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });\n                    }\n",
        "                BackboneMessage::Member { owner, slot, child_id, envelopes } => {\n                    if !envelopes.is_empty() {\n                        *self.bytes = Some(std::mem::take(envelopes));\n                        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });\n                    }\n                    if let Some(bytes) = Self::take_string(owner) {\n                        *self.bytes = Some(bytes);\n                        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });\n                    }\n",
        1,
    ),
]

SYNC_TEST_REPLACEMENTS = [
    ('BackboneMessage::Member { slot: "content".into(), child_id: "child-a".into(),', 'BackboneMessage::Member { owner: String::new(), slot: "content".into(), child_id: "child-a".into(),', 1),
]


def n1_module():
    spec = importlib.util.spec_from_file_location("s5_nested_n1", TICKET / "🧪️s5-nested-n1-keys.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def base_text(n1, relative: str) -> tuple[str, bool]:
    text = (REPO / relative).read_text(encoding="utf-8")
    for candidate, replacements, regions, marker in n1.FILES:
        if candidate == relative and marker not in text:
            return n1.planned(relative, text, replacements, regions), False
    return text, True


def planned(name: str, text: str, replacements) -> str:
    for old, new, count in replacements:
        found = text.count(old)
        if found != count:
            raise SystemExit(f"{name}: anchor drift: expected {count}, found {found}: {old[:140]!r}")
        text = text.replace(old, new)
    return text


def pre_image(relative: str) -> pathlib.Path:
    return PRE / relative.replace("/", "__")


def main() -> None:
    arguments = [argument for argument in sys.argv[1:] if not argument.startswith("--")]
    frames = "--without-frames" not in sys.argv
    mode = arguments[0] if arguments else "check"
    files = [
        (RUNTIME, RUNTIME_REPLACEMENTS + (FRAME_REPLACEMENTS if frames else []), "pub fn owner_text(&self) -> String {"),
        (OPERATION_TEST, OPERATION_TEST_REPLACEMENTS, "            owner: String::new(),\n            slot: \"slot\".into(),"),
        (STORE, STORE_REPLACEMENTS, "BackboneMessage::Member { owner: owner.to_string(),"),
        (SYNC_TEST, SYNC_TEST_REPLACEMENTS, "BackboneMessage::Member { owner: String::new(),"),
    ]
    if mode == "restore":
        for relative, _, _ in files:
            if pre_image(relative).exists():
                shutil.copyfile(pre_image(relative), REPO / relative)
                print(f"restored {relative}")
        return
    n1 = n1_module()
    results = []
    on_disk = True
    for relative, replacements, marker in files:
        text, landed = base_text(n1, relative)
        on_disk = on_disk and landed
        if marker in text:
            print(f"already landed: {relative}")
            continue
        results.append((relative, text, planned(relative, text, replacements)))
        print(f"{relative}: {sum(count for _, _, count in replacements)} replacements on {'the live file' if landed else 'N1 planned'}; {len(text)} -> {len(results[-1][2])} bytes")
    if mode == "land":
        if not on_disk:
            raise SystemExit("N1 is not on disk: land N1 first")
        PRE.mkdir(parents=True, exist_ok=True)
        for relative, text, result in results:
            pre_image(relative).write_text(text, encoding="utf-8")
            (REPO / relative).write_text(result, encoding="utf-8")
            print(f"landed {relative}")


if __name__ == "__main__":
    main()

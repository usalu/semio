"""📏️ Apply the exact original queue and mounted preview cancellation quotes."""
from pathlib import Path

changes = [
    (
        "🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs",
        "    pub fn admit_owned_demand<T:RetireOwned>(&self,value:&T)->Result<RetirementDemand,ValueError>{let mut demand=retirement::controlled::controlled_retirement_birth_demand(value)?;",
        """    /// 📏️ Quotes the fixed frame layout and placement for a ready original of this type.
    pub fn admit_owned_slot_demand<T:RetireOwned>(&self)->Result<RetirementDemand,ValueError>{
        let capacity_bytes=Self::frame_birth_bytes::<T>();
        self.with_placement_demand(RetirementDemand{copy_bytes:size_of::<Option<T>>()+capacity_bytes,capacity_bytes,depth:1,..Default::default()})
    }
    pub fn admit_owned_demand<T:RetireOwned>(&self,value:&T)->Result<RetirementDemand,ValueError>{
        self.with_placement_demand(retirement::controlled::controlled_retirement_birth_demand(value)?)
    }
    fn with_placement_demand(&self,mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{""",
    ),
    (
        "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧾️eval-session-state/🦀️.rs",
        "        Ok(RetirementDemand{capacity_bytes:RetirementQueue::frame_birth_bytes::<SessionCollectionOwner>(),depth:self.owners.len()+1,..Default::default()})",
        "        self.owners.admit_owned_slot_demand::<SessionCollectionOwner>()",
    ),
    (
        "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏱️tick-and-resolve/🦀️.rs",
        "            Some(host) if !host.baseline_publication_terminal_is_empty() => host.next_baseline_publication_demands(copy),",
        """            Some(host) if !host.baseline_publication_terminal_is_empty() => host.next_baseline_publication_demands(copy),
            _ if !session.preview_cancellation_terminal_is_empty() => session.next_preview_cancellation_demands(copy),""",
    ),
    (
        "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏱️tick-and-resolve/🦀️.rs",
        "        if self.host.is_none() && !session.preview_cancellation_terminal_is_empty() {\n            return Ok((None, session.preview_cancellation_step(effect)?.progress()));\n        }\n",
        "",
    ),
    (
        "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏱️tick-and-resolve/🦀️.rs",
        "            return Ok((None, progress?));\n        }\n        if let Some(host) = self.host.as_mut() {",
        """            return Ok((None, progress?));
        }
        if !session.preview_cancellation_terminal_is_empty() {
            return Ok((None, session.preview_cancellation_step(effect)?.progress()));
        }
        if let Some(host) = self.host.as_mut() {""",
    ),
]

pending = {}
for location, before, after in changes:
    path = Path(location)
    source = pending[path] if path in pending else path.read_text()
    if source.count(before) != 1:
        raise AssertionError(f"Expected one live match in {path}")
    pending[path] = source.replace(before, after)

if __name__ == "__main__":
    import sys
    if sys.argv[1:] == ["--apply"]:
        for path, source in pending.items():
            path.write_text(source)
    else:
        for path in pending:
            print(path)

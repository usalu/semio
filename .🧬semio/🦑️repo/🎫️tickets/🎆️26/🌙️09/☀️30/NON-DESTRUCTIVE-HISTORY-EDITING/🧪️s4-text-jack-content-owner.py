#!/usr/bin/env python3
"""🧸️ S4-TEXT (session 4): jack's composed `content` handle owns a `JackContentOwner` (the rich `s.stdio.semio@v1/graph`
snapshot), but the wire-runtime retirement and clone authority still downcast the retired `JackWorkingScene` owner, so every
snapshot retirement reported `Blocked` and every store drop asserted. The retirement now retires the actual content owner through
the graph's own bounded cursor, the clone authority shares the actual owner, and the owner laws read it. Every replacement
asserts its count; staged then written (`--check` = dry run)."""
import sys
from pathlib import Path

J = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack")
S = J / "🏅️standards/🔖️1/🪆️subsets/✳️any"
WR = S / "🧬️schema/🛜️wire-runtime/🦀️.rs"
EDITS = {
    J / "🪆️content/🦀️.rs": [
        (" pub fn snapshot(&self)->&SemioGraphSnapshot{self.snapshot.as_ref().expect(\"live Jack Semio child\")}\n}", " pub fn snapshot(&self)->&SemioGraphSnapshot{self.snapshot.as_ref().expect(\"live Jack Semio child\")}\n /// ♻️ Hands the owned graph to a bounded retirement cursor; the emptied owner then drops without retiring.\n pub fn take_snapshot(&mut self)->Option<SemioGraphSnapshot>{self.snapshot.take()}\n}", 1),
    ],
    WR: [
        ("    SceneRoot(std::sync::Arc<crate::JackWorkingScene>),\n    Scene(crate::JackWorkingScene),\n", "    ContentRoot(std::sync::Arc<crate::JackContentOwner>),\n    Content(Box<dyn store::ErasedSnapshotRetirement>),\n", 1),
        ("""                    let scene = match value.content.take_local_owner::<crate::JackWorkingScene>() {
                        Ok(scene) => scene,
                        Err(_) => return store::SnapshotRetirementStep::Blocked,
                    };
                    self.phase = 1;
                    scene.map_or(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }, |scene| Self::spawn(&mut self.active, JackRetirementOwner::SceneRoot(scene)))""", """                    let content = match value.content.take_local_owner::<crate::JackContentOwner>() {
                        Ok(content) => content,
                        Err(_) => return store::SnapshotRetirementStep::Blocked,
                    };
                    self.phase = 1;
                    content.map_or(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }, |content| Self::spawn(&mut self.active, JackRetirementOwner::ContentRoot(content)))""", 1),
        ("""            JackRetirementOwner::SceneRoot(_) => {
                if maximum_items == 0 {
                    return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
                }
                let scene = match self.owner.take() {
                    Some(JackRetirementOwner::SceneRoot(scene)) => scene,
                    _ => unreachable!("Jack scene root owner remains exact"),
                };
                match std::sync::Arc::try_unwrap(scene) {
                    Ok(scene) => {
                        *self.owner = Some(JackRetirementOwner::Scene(scene));
                        store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                    }
                    Err(scene) => {
                        drop(scene);
                        store::SnapshotRetirementStep::Complete
                    }
                }
            }
            JackRetirementOwner::Scene(scene) => {
                if let Some(node) = scene.nodes.pop() {
                    return Self::spawn(&mut self.active, JackRetirementOwner::Node(node));
                }
                if let Some(edge) = scene.edges.pop() {
                    return Self::spawn(&mut self.active, JackRetirementOwner::Edge(edge));
                }
                drop(self.owner.take());
                store::SnapshotRetirementStep::Complete
            }""", """            JackRetirementOwner::ContentRoot(_) => {
                if maximum_items == 0 {
                    return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
                }
                let content = match self.owner.take() {
                    Some(JackRetirementOwner::ContentRoot(content)) => content,
                    _ => unreachable!("Jack content root owner remains exact"),
                };
                match std::sync::Arc::try_unwrap(content).map(|mut content| content.take_snapshot()) {
                    Ok(Some(snapshot)) => {
                        *self.owner = Some(JackRetirementOwner::Content(semio_framework_value::retirement::owned_retirement(snapshot)));
                        store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                    }
                    Ok(None) => store::SnapshotRetirementStep::Complete,
                    Err(content) => {
                        drop(content);
                        store::SnapshotRetirementStep::Complete
                    }
                }
            }
            JackRetirementOwner::Content(cursor) => {
                let step = cursor.close_step(maximum_items, maximum_bytes);
                let terminal = cursor.terminal_is_empty();
                match step {
                    Ok(store::SnapshotRetirementStep::Complete) if terminal => {
                        drop(self.owner.take());
                        store::SnapshotRetirementStep::Complete
                    }
                    Ok(store::SnapshotRetirementStep::Complete) | Err(_) => store::SnapshotRetirementStep::Blocked,
                    Ok(step) => step,
                }
            }""", 1),
        ("                    if let Some(owner) = source.content.local_owner::<crate::JackWorkingScene>() {", "                    if let Some(owner) = source.content.local_owner::<crate::JackContentOwner>() {", 1),
    ],
    J / "🧪️tests/🔬️unit/🦀️.rs": [
        ('        "ownedHasScene": owned.local_owner::<JackWorkingScene>().is_some(),', '        "ownedHasScene": owned.local_owner::<crate::JackContentOwner>().is_some(),', 1),
        ('        "wireHasScene": reconstructed.local_owner::<JackWorkingScene>().is_some(),', '        "wireHasScene": reconstructed.local_owner::<crate::JackContentOwner>().is_some(),', 1),
    ],
    S / "🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs": [
        ('    let source_owner = source.content.local_owner::<crate::JackWorkingScene>().expect("source scene owner");', '    let source_owner = source.content.local_owner::<crate::JackContentOwner>().expect("source content owner");', 1),
        ('    assert_eq!(source.content.local_owner::<crate::JackWorkingScene>().expect("source survives cancellation").nodes.len(), source_owner.nodes.len());', '    assert_eq!(source.content.local_owner::<crate::JackContentOwner>().expect("source survives cancellation").snapshot().nodes.len(), source_owner.snapshot().nodes.len());', 1),
    ],
    S / "🧬️schema/🛜️wire-runtime/🧪️tests/🔬️unit/🦀️.rs": [
        ('    let source_owner = source.content.local_owner::<crate::JackWorkingScene>().expect("source scene owner");', '    let source_owner = source.content.local_owner::<crate::JackContentOwner>().expect("source content owner");', 1),
        ('    let clone_owner = clone.content.local_owner::<crate::JackWorkingScene>().expect("clone scene owner");', '    let clone_owner = clone.content.local_owner::<crate::JackContentOwner>().expect("clone content owner");', 1),
        ('    assert_eq!(source.content.local_owner::<crate::JackWorkingScene>().expect("live source owner remains").nodes[0].id, "live");', '    assert_eq!(source.content.local_owner::<crate::JackContentOwner>().expect("live source owner remains").snapshot().nodes[0].id.value, "live");', 1),
    ],
}
staged = {}
for path, pairs in EDITS.items():
    text = path.read_text()
    for old, new, count in pairs:
        if text.count(old) != count:
            sys.exit(f"{path}: expected {count} of {old[:90]!r}, found {text.count(old)}")
        text = text.replace(old, new)
    staged[path] = text
if "--check" not in sys.argv:
    for path, text in staged.items():
        path.write_text(text)
print(f"jack content owner: {len(staged)} files {'checked' if '--check' in sys.argv else 'written'}")

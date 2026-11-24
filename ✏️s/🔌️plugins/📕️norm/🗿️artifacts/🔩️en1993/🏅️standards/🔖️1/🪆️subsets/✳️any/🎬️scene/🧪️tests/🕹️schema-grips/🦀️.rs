//! 🕹️ EN 1993 scene grips: every numeric leaf of the connection panel and cold-formed rows named by the snapshot schema has a drag grip; the schema text is read literally via `include_str!`, so the check lives in its own crate and keeps the scene crate under its source boundary. See [`semio_s_artifact_norm_en1993_scene_members`].

extern crate semio_s_artifact_norm_en1993_model as model;
extern crate semio_s_artifact_norm_en1993_scene_members as scene_members;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[cfg(test)]
mod tests {
    use super::model::En1993Snapshot;
    use super::scene::projection_kit::test_support::assert_handle_laws;
    use super::scene::{EntityKind, EntityRef};
    use std::collections::BTreeSet;

    #[test]
    fn every_numeric_leaf_of_the_connection_panel_and_cold_formed_rows_has_a_grip() {
        let schema: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/📸️snapshot/🔣️.json")).expect("the snapshot schema parses");
        let documents = [semio_s_artifact_norm_en1993_showcase::full_showcase(), En1993Snapshot::special_parts_showcase(), En1993Snapshot::compliant_heb240_frame()];
        let mut seen = BTreeSet::new();
        for document in &documents {
            let rows: [(&str, &str, EntityKind, Vec<&str>); 5] = [
                ("joints", "SteelJoint", EntityKind::Group, document.joints.iter().map(|r| r.id.as_str()).collect()),
                ("jointPlates", "JointPlate", EntityKind::Group, document.joint_plates.iter().map(|r| r.id.as_str()).collect()),
                ("hollowJoints", "HollowJoint", EntityKind::Group, document.hollow_joints.iter().map(|r| r.id.as_str()).collect()),
                ("platedPanels", "PlatedPanel", EntityKind::Plate, document.plated_panels.iter().map(|r| r.id.as_str()).collect()),
                ("coldFormedMembers", "ColdFormedMember", EntityKind::Member, document.cold_formed_members.iter().map(|r| r.id.as_str()).collect()),
            ];
            for (collection, definition, kind, ids) in rows {
                let leaves: Vec<&str> = schema["$defs"][definition]["properties"].as_object().expect("definition").iter().filter(|(_, p)| matches!(p["type"].as_str(), Some("number" | "integer"))).map(|(name, _)| name.as_str()).collect();
                for id in ids {
                    let all = scene_members::handles(document, &[], &[EntityRef::new(kind, id, format!("{collection}[id={id}]"))]);
                    assert_handle_laws(&all);
                    let written: BTreeSet<&str> = all.iter().flat_map(|h| h.writes.iter().map(|w| w.path.as_str())).collect();
                    for leaf in &leaves {
                        assert!(written.contains(format!("{collection}[id={id}].{leaf}").as_str()), "{collection}[{id}].{leaf} has no grip");
                    }
                    seen.insert(collection);
                }
            }
        }
        assert_eq!(seen.len(), 5, "the fixtures cover every grouped collection: {seen:?}");
    }
}

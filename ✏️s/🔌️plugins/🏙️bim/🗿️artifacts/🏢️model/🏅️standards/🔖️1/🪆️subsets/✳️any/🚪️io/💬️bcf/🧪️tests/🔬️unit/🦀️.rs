//! 🧪️ The BCF 2.1 exchange: guids, moments, the topics of a model, the container bytes, the round trip into the same model and the committed container of the frame.

use super::*;
use crate::{IssueViewpoint, Point2};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const FRAME: &str = include_str!("../../../../🧫️fixtures/💡️inferences/🧨️clash-sets/🏢️frame/📸️snapshot/🔣️.json");

fn camera() -> ViewCamera {
    ViewCamera { target: Point2 { x: 3.0, y: 2.0 }, target_height: 1.5, azimuth: 0.8, pitch: 0.5, distance: 12.0 }
}

fn model() -> ModelSnapshot {
    let mut model: ModelSnapshot = from_json_str(FRAME, JsonMemberPolicy::Reject).expect("the frame decodes");
    let viewpoint = IssueViewpoint { camera: camera(), section: Some(SectionBox { min: Point3 { x: 0.0, y: 0.0, z: 0.0 }, max: Point3 { x: 6.0, y: 4.0, z: 4.0 } }), isolate: vec!["b-2".into()] };
    let issue = |title: &str, status: IssueStatus, priority: IssuePriority, assignee: &str, elements: &[&str], viewpoint: Option<IssueViewpoint>| Issue {
        title: title.into(),
        description: format!("About {title}"),
        status,
        priority,
        assignee: assignee.into(),
        author: "UG".into(),
        created: "2026-10-09T08:30:00Z".into(),
        labels: vec!["Clash".into(), "Structure".into()],
        elements: elements.iter().map(|id| id.to_string()).collect(),
        clash: None,
        viewpoint,
    };
    model.issues.insert("is-1".into(), issue("Beam through the wall", IssueStatus::InProgress, IssuePriority::High, "structure", &["b-2", "w-1"], Some(viewpoint)));
    model.issues.insert("is-2".into(), issue("Duct needs a sleeve", IssueStatus::Open, IssuePriority::Normal, "", &["d-1"], None));
    model.issues.insert("0b2a4b6e-5c1d-4d7e-9a3c-1f2e3d4c5b6a".into(), issue("Already a UUID", IssueStatus::Closed, IssuePriority::Low, "", &[], None));
    model.issue_comments.insert("ic-1".into(), IssueComment { issue: "is-1".into(), author: "structure".into(), date: "2026-10-09T09:30:00Z".into(), text: "Will check.".into() });
    model.issue_comments.insert("ic-2".into(), IssueComment { issue: "is-1".into(), author: "UG".into(), date: "2026-10-09".into(), text: "Thanks.".into() });
    model
}

#[test]
fn a_guid_is_the_id_when_it_is_a_uuid_and_a_stable_uuid_otherwise() {
    assert_eq!(guid_of("0B2A4B6E-5C1D-4D7E-9A3C-1F2E3D4C5B6A"), "0b2a4b6e-5c1d-4d7e-9a3c-1f2e3d4c5b6a");
    let guid = guid_of("is-1");
    assert!(is_uuid(&guid) && guid == guid_of("is-1") && guid != guid_of("is-2"));
    assert_eq!(&guid[14..15], "5", "version 5");
    assert!(matches!(&guid[19..20], "8" | "9" | "a" | "b"), "variant 1: {guid}");
    assert_eq!(guid_of(&guid), guid, "a derived guid is stable under another hop");
    assert_eq!(ifc_guid("w-1"), global_id("w-1"));
    assert_eq!(ifc_guid("w-1").len(), 22);
}

#[test]
fn a_moment_becomes_an_xs_date_time() {
    assert_eq!(date_time("2026-10-09"), "2026-10-09T00:00:00+00:00");
    assert_eq!(date_time("2026-10-09T08:30:00Z"), "2026-10-09T08:30:00Z");
    assert_eq!(date_time("2026-10-09T08:30:00+02:00"), "2026-10-09T08:30:00+02:00");
    assert_eq!(date_time("2026-10-09T08:30:00"), "2026-10-09T08:30:00+00:00");
    for moment in ["2026-10-09", "2026-10-09T08:30:00", "2026-10-09T08:30:00-05:30"] {
        assert!(crate::valid_timestamp(&date_time(moment)), "{moment}");
    }
}

#[test]
fn the_topics_carry_the_authored_fields_the_comments_in_writing_order_and_the_viewpoint() {
    let document = document_of(&model());
    assert_eq!((document.version.as_str(), document.topics.len()), ("2.1", 3));
    let topic = document.topics.iter().find(|topic| topic.title == "Beam through the wall").expect("the topic");
    assert_eq!((topic.status.as_str(), topic.priority.as_str(), topic.creation_author.as_str(), topic.assigned_to.as_str()), ("In Progress", "High", "UG", "structure"));
    assert_eq!(topic.labels, ["Clash", "Structure"]);
    assert_eq!(topic.comments.iter().map(|comment| comment.text.as_str()).collect::<Vec<_>>(), ["Thanks.", "Will check."], "by moment: the dated one is the earlier");
    assert_eq!(topic.comments[0].date, "2026-10-09T00:00:00+00:00");
    let viewpoint = &topic.viewpoints[0];
    let camera_row = viewpoint.camera.as_ref().expect("a perspective camera");
    let (eye, target) = camera().eye_and_target();
    assert!((camera_row.eye[0] - eye[0]).abs() < 1e-9 && (camera_row.eye[2] - eye[2]).abs() < 1e-9);
    let direction = camera_row.direction;
    let length = (direction[0].powi(2) + direction[1].powi(2) + direction[2].powi(2)).sqrt();
    let dot: f64 = (0..3).map(|axis| direction[axis] * camera_row.up[axis]).sum();
    assert!((length - 1.0).abs() < 1e-9 && dot.abs() < 1e-9);
    assert!((eye[0] + 12.0 * direction[0] - target[0]).abs() < 1e-9, "the camera looks at the target from its distance");
    assert_eq!(camera_row.field_of_view, FIELD_OF_VIEW);
    assert_eq!(viewpoint.selection, [ifc_guid("b-2"), ifc_guid("w-1")]);
    assert!(!viewpoint.default_visibility && viewpoint.exceptions == [ifc_guid("b-2")]);
    assert_eq!(viewpoint.clipping.len(), 6);
    let bare = document.topics.iter().find(|topic| topic.title == "Already a UUID").expect("the topic");
    assert_eq!(bare.guid, "0b2a4b6e-5c1d-4d7e-9a3c-1f2e3d4c5b6a");
    assert!(bare.viewpoints.is_empty(), "no elements and no viewpoint: no viewpoint file");
}

#[test]
fn the_section_box_is_six_outward_planes_and_the_planes_give_the_box_back() {
    let section = SectionBox { min: Point3 { x: -1.0, y: 0.5, z: 0.0 }, max: Point3 { x: 6.0, y: 4.0, z: 3.25 } };
    let planes = planes_of(&section);
    assert_eq!(planes.len(), 6);
    assert!(planes.iter().all(|plane| plane.direction.iter().map(|part| part.abs()).sum::<f64>() == 1.0));
    assert_eq!(box_of(&planes), Some(section.clone()));
    assert_eq!(box_of(&planes[..5]), None, "five planes are no box");
    let mut tilted = planes.clone();
    tilted[0].direction = [-0.6, 0.8, 0.0];
    assert_eq!(box_of(&tilted), None, "a tilted plane is no box");
}

#[test]
fn the_container_is_a_zip_of_the_version_and_per_topic_the_markup_and_the_viewpoint_files() {
    let bytes = export_bcf(&model()).expect("encodes");
    assert!(bytes.starts_with(b"PK"));
    let archive = ZipArchive::parse(&bytes).expect("a zip");
    let names: BTreeSet<&str> = archive.entries().iter().map(|entry| entry.name.as_str()).collect();
    assert!(names.contains("bcf.version"));
    assert_eq!(names.iter().filter(|name| name.ends_with("/markup.bcf")).count(), 3);
    assert_eq!(names.iter().filter(|name| name.ends_with(".bcfv")).count(), 2, "the topic without elements and viewpoint has none");
    assert!(names.iter().all(|name| !name.ends_with("snapshot.png")));
    let decoded = read_document(&bytes).expect("decodes");
    assert_eq!((decoded.version.as_str(), decoded.topics.len()), ("2.1", 3));
    assert_eq!(decoded.topics.iter().map(|topic| topic.comments.len()).sum::<usize>(), 2);
    let mut expected = document_of(&model());
    expected.topics.sort_by(|a, b| a.guid.cmp(&b.guid));
    assert_eq!(decoded.topics.iter().map(|topic| (topic.guid.clone(), topic.title.clone(), topic.comments.clone(), topic.viewpoints.len())).collect::<Vec<_>>(), expected.topics.iter().map(|topic| (topic.guid.clone(), topic.title.clone(), topic.comments.clone(), topic.viewpoints.len())).collect::<Vec<_>>());
}

#[test]
fn an_export_imports_into_the_issues_it_was_made_of() {
    let model = model();
    let bytes = export_bcf(&model).expect("encodes");
    let imported = import_bcf(&bytes, &resolver_of(&model)).expect("decodes");
    assert!(imported.notes.is_empty(), "{:?}", imported.notes);
    assert_eq!(imported.issues.len(), 3);
    let original = &model.issues["is-1"];
    let back = &imported.issues[&guid_of("is-1")];
    assert_eq!((back.title.as_str(), back.status, back.priority, back.assignee.as_str(), back.author.as_str()), (original.title.as_str(), original.status, original.priority, original.assignee.as_str(), original.author.as_str()));
    assert_eq!((back.labels.clone(), back.elements.clone()), (original.labels.clone(), original.elements.clone()));
    let (viewpoint, expected) = (back.viewpoint.as_ref().expect("viewpoint"), original.viewpoint.as_ref().expect("viewpoint"));
    assert_eq!(viewpoint.section, expected.section);
    assert_eq!(viewpoint.isolate, expected.isolate);
    assert!((viewpoint.camera.azimuth - expected.camera.azimuth).abs() < 1e-6 && (viewpoint.camera.pitch - expected.camera.pitch).abs() < 1e-6);
    let comments: Vec<&IssueComment> = imported.comments.values().filter(|comment| comment.issue == guid_of("is-1")).collect();
    assert_eq!(comments.len(), 2);
    assert!(imported.issues.values().all(|issue| crate::valid_timestamp(&issue.created)));
}

#[test]
fn a_second_round_is_stable() {
    let model = model();
    let first = import_bcf(&export_bcf(&model).expect("encodes"), &resolver_of(&model)).expect("decodes");
    let again = ModelSnapshot { issues: first.issues.clone(), issue_comments: first.comments.clone(), ..model.clone() };
    let second = import_bcf(&export_bcf(&again).expect("encodes"), &resolver_of(&model)).expect("decodes");
    assert_eq!(second.issues.keys().collect::<Vec<_>>(), first.issues.keys().collect::<Vec<_>>(), "the guids survive another hop");
    assert_eq!(second.comments.keys().collect::<Vec<_>>(), first.comments.keys().collect::<Vec<_>>());
    for (id, issue) in &first.issues {
        let other = &second.issues[id];
        assert_eq!((&issue.title, issue.status, issue.priority, &issue.elements, &issue.labels, &issue.assignee), (&other.title, other.status, other.priority, &other.elements, &other.labels, &other.assignee));
    }
}

#[test]
fn components_that_name_no_element_are_dropped_and_counted() {
    let model = model();
    let bytes = export_bcf(&model).expect("encodes");
    let imported = import_bcf(&bytes, &|_| None).expect("decodes");
    assert!(imported.issues.values().all(|issue| issue.elements.is_empty()));
    assert_eq!(imported.notes, ["3 selected component(s) name no element of the model and were dropped."]);
}

#[test]
fn foreign_values_are_mapped_to_the_nearest_status_and_priority() {
    assert_eq!((status_of("Active"), status_of("fixed"), status_of("closed"), status_of("whatever"), status_of("In-Progress")), (IssueStatus::InProgress, IssueStatus::Resolved, IssueStatus::Closed, IssueStatus::Open, IssueStatus::InProgress));
    assert_eq!((priority_of("Major"), priority_of("Minor"), priority_of("Blocker"), priority_of("Medium")), (IssuePriority::High, IssuePriority::Low, IssuePriority::Critical, IssuePriority::Normal));
}

#[test]
fn a_broken_container_is_refused() {
    assert!(read_document(b"not a zip").is_err());
    let mut zip = ZipWriter::new();
    zip.add("other.txt", b"x").expect("adds");
    assert!(read_document(&zip.finish().expect("finishes")).is_err(), "no bcf.version");
}

#[semio_framework_async_macros::async_test]
async fn the_serializer_writes_the_container_and_the_deserializer_reads_it_back_without_elements() {
    let model = model();
    let payload = <ModelIntoBcf as Serializer<ModelSnapshot>>::serialize(&model, &ArchiveChildren::empty()).await.expect("serializes").value;
    assert_eq!(<BcfIntoModel as Deserializer<ModelSnapshot>>::sniff(&payload).await, Confidence::High);
    assert_eq!(<BcfIntoModel as Deserializer<ModelSnapshot>>::sniff(&IoPayload::Binary(b"ISO-10303-21;".to_vec())).await, Confidence::None);
    let read = <BcfIntoModel as Deserializer<ModelSnapshot>>::deserialize(&payload).await.expect("deserializes").value;
    assert_eq!((read.issues.len(), read.issue_comments.len()), (3, 2));
    assert!(read.walls.is_empty());
}

#[test]
fn the_committed_container_is_the_current_export_of_the_frame() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🚪️bcf/🏢️frame");
    let bytes = export_bcf(&model()).expect("encodes");
    if std::env::var_os("BIM_BLESS").is_some() {
        std::fs::create_dir_all(&path).expect("directory");
        std::fs::write(path.join("issues.bcf"), &bytes).expect("writes the container");
        std::fs::write(path.join("model.json"), semio_framework_pack_json::to_json_string(&model())).expect("writes the model");
    }
    assert_eq!(std::fs::read(path.join("issues.bcf")).expect("the committed container (BIM_BLESS=1 writes it)"), bytes);
}

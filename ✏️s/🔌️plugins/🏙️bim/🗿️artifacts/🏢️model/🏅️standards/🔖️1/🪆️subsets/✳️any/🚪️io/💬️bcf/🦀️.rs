//! 💬️ `s.bim.model@1/*` ⇄ BCF 2.1: the issues of the model as the topics of a BCF-XML 2.1 container, a flat zip of `bcf.version` and, per topic, a folder named by the topic guid with `markup.bcf` and one `<viewpoint guid>.bcfv`.
//! The XML is written and read with the XML tree and text codec of the stdio `xml` artifact (the one place that names it is [`crate::standards::v1::subsets::any::io::export::gbxml::codec`]); the container is the
//! first-party zip of the framework `deflate` module (the stdio `bcf` and `zip` artifacts are not used: they do not compile in this tree, see the report of the package).
//! * topic: guid (the issue id when it is a UUID, else a stable UUID derived from it), title, description, status, priority, labels, `AssignedTo`, creation moment and author; comments: guid, moment, author, text;
//! * viewpoint: the orbit camera as a `PerspectiveCamera` (position, direction, up vector, field of view 60), the elements of the issue as selected components by IFC GlobalId (the one the IFC export writes), the isolate
//!   set as the visibility exceptions of a default visibility `false`, the section box as the six `ClippingPlane`s of its faces (the normal points to the clipped side);
//! * no `snapshot.png`: the stdio `png` artifact encodes images but no raster view of the model exists to take it from.
//!
//! 🔖 `IoFidelity::Lossy`: snapshots are not written, a topic keeps its first viewpoint with a camera (the selection of all viewpoints becomes its elements), clipping planes that do not form an axis aligned box are dropped.
//! 📎 https://github.com/buildingSMART/BCF-XML/tree/release_2_1

use crate::standards::v1::subsets::any::io::export::gbxml::codec::{attr, document_text, element, leaf, number};
use crate::standards::v1::subsets::any::io::export::ifc::writer::global_id;
use crate::{ClashRef, Issue, IssueComment, IssuePriority, IssueStatus, IssueViewpoint, ModelSnapshot, Point3, SectionBox, ViewCamera};
use semio_framework::io_schema::{Confidence, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_deflate::zip_archive::{ZipArchive, ZipWriter};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Deserializer, Serializer};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
use std::collections::{BTreeMap, BTreeSet};

/// 🪪️ The BCF 2.1 dialect the issues are written in.
pub const BCF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId::ANY };

/// 📐️ The field of view of an exported camera in degrees.
pub const FIELD_OF_VIEW: f64 = 60.0;

/// 📏️ The largest entry of a container the reader accepts, in bytes.
pub const MAX_ENTRY: usize = 64 * 1024 * 1024;

//#region 🔖️Values
/// 🎥️ A perspective camera of a viewpoint.
#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub eye: [f64; 3],
    pub direction: [f64; 3],
    pub up: [f64; 3],
    pub field_of_view: f64,
}

/// ✂️ One clipping plane: a point on it and its normal, which points to the clipped side.
#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    pub location: [f64; 3],
    pub direction: [f64; 3],
}

/// 👁️ One viewpoint file of a topic.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Viewpoint {
    pub guid: String,
    pub camera: Option<Camera>,
    pub selection: Vec<String>,
    pub exceptions: Vec<String>,
    pub default_visibility: bool,
    pub clipping: Vec<Plane>,
}

/// 💬️ One comment of a topic.
#[derive(Clone, Debug, PartialEq)]
pub struct Comment {
    pub guid: String,
    pub date: String,
    pub author: String,
    pub text: String,
}

/// 🗂️ One topic: its markup and its viewpoints.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Topic {
    pub guid: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub labels: Vec<String>,
    pub assigned_to: String,
    pub creation_date: String,
    pub creation_author: String,
    pub comments: Vec<Comment>,
    pub viewpoints: Vec<Viewpoint>,
}

/// 📦️ A BCF container: its version and topics.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    pub version: String,
    pub topics: Vec<Topic>,
}
//#endregion 🔖️Values

//#region 🔖️Guids
fn fnv(bytes: &[u8], seed: u64) -> u64 {
    bytes.iter().fold(seed, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3))
}

fn is_uuid(text: &str) -> bool {
    let parts: Vec<&str> = text.split('-').collect();
    parts.len() == 5 && parts.iter().zip([8, 4, 4, 4, 12]).all(|(part, width)| part.len() == width && part.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

/// 🔑️ The BCF guid of the record `id`: the id itself (lower case) when it already is a UUID, else a stable version 5 style UUID derived from it.
pub fn guid_of(id: &str) -> String {
    if is_uuid(id) {
        return id.to_ascii_lowercase();
    }
    let high = fnv(id.as_bytes(), 0xcbf2_9ce4_8422_2325);
    let low = fnv(id.as_bytes(), 0x8422_2325_cbf2_9ce4).rotate_left(23) ^ high;
    let bits = (u128::from(high) << 64) | u128::from(low);
    let bits = (bits & !(0xf_u128 << 76) | (0x5_u128 << 76)) & !(0x3_u128 << 62) | (0x2_u128 << 62);
    let hex = format!("{bits:032x}");
    format!("{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32])
}

/// 🔑️ The IFC GlobalId the IFC export writes for the element `id`.
pub fn ifc_guid(id: &str) -> String {
    global_id(id)
}
//#endregion 🔖️Guids

//#region 🔖️Export
fn status_name(status: IssueStatus) -> &'static str {
    match status {
        IssueStatus::Open => "Open",
        IssueStatus::InProgress => "In Progress",
        IssueStatus::Resolved => "Resolved",
        IssueStatus::Closed => "Closed",
    }
}

fn priority_name(priority: IssuePriority) -> &'static str {
    match priority {
        IssuePriority::Low => "Low",
        IssuePriority::Normal => "Normal",
        IssuePriority::High => "High",
        IssuePriority::Critical => "Critical",
    }
}

/// 📅️ The `xs:dateTime` of a moment: a date alone becomes midnight UTC, a time without zone is UTC.
pub fn date_time(moment: &str) -> String {
    match moment.split_once('T') {
        None => format!("{moment}T00:00:00+00:00"),
        Some((_, time)) if time.ends_with('Z') || time.contains('+') || time.rsplit_once('-').is_some_and(|(_, offset)| offset.contains(':') && !offset.contains('.') && offset.len() == 5) => moment.to_string(),
        Some(_) => format!("{moment}+00:00"),
    }
}

fn unit(values: [f64; 3]) -> [f64; 3] {
    let length = values.iter().map(|part| part * part).sum::<f64>().sqrt().max(1e-12);
    values.map(|part| part / length)
}

fn camera_of(camera: &ViewCamera) -> Camera {
    let (eye, target) = camera.eye_and_target();
    let look = unit([target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]]);
    let up = [camera.pitch.sin() * camera.azimuth.cos(), camera.pitch.sin() * camera.azimuth.sin(), camera.pitch.cos()];
    Camera { eye, direction: look, up, field_of_view: FIELD_OF_VIEW }
}

/// ✂️ The six clipping planes of a section box: on each face, the normal pointing out of the box.
pub fn planes_of(section: &SectionBox) -> Vec<Plane> {
    let centre = [(section.min.x + section.max.x) / 2.0, (section.min.y + section.max.y) / 2.0, (section.min.z + section.max.z) / 2.0];
    let (low, high) = ([section.min.x, section.min.y, section.min.z], [section.max.x, section.max.y, section.max.z]);
    let mut planes = Vec::with_capacity(6);
    for axis in 0..3 {
        for (bound, sign) in [(low[axis], -1.0), (high[axis], 1.0)] {
            let mut location = centre;
            location[axis] = bound;
            let mut direction = [0.0; 3];
            direction[axis] = sign;
            planes.push(Plane { location, direction });
        }
    }
    planes
}

fn viewpoint_of(guid: &str, issue: &Issue) -> Option<Viewpoint> {
    let selection: Vec<String> = issue.elements.iter().map(|id| ifc_guid(id)).collect();
    if issue.viewpoint.is_none() && selection.is_empty() {
        return None;
    }
    let isolate: Vec<String> = issue.viewpoint.iter().flat_map(|viewpoint| viewpoint.isolate.iter().map(|id| ifc_guid(id))).collect();
    Some(Viewpoint {
        guid: guid.to_string(),
        camera: issue.viewpoint.as_ref().map(|viewpoint| camera_of(&viewpoint.camera)),
        selection,
        default_visibility: isolate.is_empty(),
        exceptions: isolate,
        clipping: issue.viewpoint.as_ref().and_then(|viewpoint| viewpoint.section.as_ref()).map(planes_of).unwrap_or_default(),
    })
}

/// 💬️ The topics of the issues of `model`, in issue id order; a comment is a comment of its topic in writing order.
pub fn document_of(model: &ModelSnapshot) -> Document {
    let topics = model
        .issues
        .iter()
        .map(|(id, issue)| Topic {
            guid: guid_of(id),
            title: issue.title.clone(),
            description: issue.description.clone(),
            status: status_name(issue.status).into(),
            priority: priority_name(issue.priority).into(),
            labels: issue.labels.clone(),
            assigned_to: issue.assignee.clone(),
            creation_date: date_time(&issue.created),
            creation_author: issue.author.clone(),
            comments: crate::comments_of(model, id).into_iter().map(|(comment_id, comment)| Comment { guid: guid_of(comment_id), date: date_time(&comment.date), author: comment.author.clone(), text: comment.text.clone() }).collect(),
            viewpoints: viewpoint_of(&guid_of(&format!("{id}#viewpoint")), issue).into_iter().collect(),
        })
        .collect();
    Document { version: "2.1".into(), topics }
}
//#endregion 🔖️Export

//#region 🔖️Write
fn triple(name: &str, values: [f64; 3]) -> XmlNode {
    element(name, Vec::new(), vec![leaf("X", Vec::new(), &number(values[0])), leaf("Y", Vec::new(), &number(values[1])), leaf("Z", Vec::new(), &number(values[2]))])
}

fn components(guids: &[String]) -> Vec<XmlNode> {
    guids.iter().map(|guid| element("Component", vec![attr("IfcGuid", guid)], Vec::new())).collect()
}

fn viewpoint_xml(viewpoint: &Viewpoint) -> XmlNode {
    let mut children = Vec::new();
    let mut parts = Vec::new();
    if !viewpoint.selection.is_empty() {
        parts.push(element("Selection", Vec::new(), components(&viewpoint.selection)));
    }
    parts.push(element("Visibility", vec![attr("DefaultVisibility", viewpoint.default_visibility)], if viewpoint.exceptions.is_empty() { Vec::new() } else { vec![element("Exceptions", Vec::new(), components(&viewpoint.exceptions))] }));
    children.push(element("Components", Vec::new(), parts));
    if let Some(camera) = &viewpoint.camera {
        children.push(element("PerspectiveCamera", Vec::new(), vec![triple("CameraViewPoint", camera.eye), triple("CameraDirection", camera.direction), triple("CameraUpVector", camera.up), leaf("FieldOfView", Vec::new(), &number(camera.field_of_view))]));
    }
    if !viewpoint.clipping.is_empty() {
        children.push(element("ClippingPlanes", Vec::new(), viewpoint.clipping.iter().map(|plane| element("ClippingPlane", Vec::new(), vec![triple("Location", plane.location), triple("Direction", plane.direction)])).collect()));
    }
    element("VisualizationInfo", vec![attr("Guid", &viewpoint.guid)], children)
}

fn markup_xml(topic: &Topic) -> XmlNode {
    let mut fields = vec![leaf("Title", Vec::new(), &topic.title)];
    if !topic.priority.is_empty() {
        fields.push(leaf("Priority", Vec::new(), &topic.priority));
    }
    fields.extend(topic.labels.iter().map(|label| leaf("Labels", Vec::new(), label)));
    fields.push(leaf("CreationDate", Vec::new(), &topic.creation_date));
    fields.push(leaf("CreationAuthor", Vec::new(), &topic.creation_author));
    if !topic.assigned_to.is_empty() {
        fields.push(leaf("AssignedTo", Vec::new(), &topic.assigned_to));
    }
    if !topic.description.is_empty() {
        fields.push(leaf("Description", Vec::new(), &topic.description));
    }
    let mut children = vec![element("Header", Vec::new(), Vec::new()), element("Topic", vec![attr("Guid", &topic.guid), attr("TopicStatus", &topic.status)], fields)];
    for comment in &topic.comments {
        children.push(element("Comment", vec![attr("Guid", &comment.guid)], vec![leaf("Date", Vec::new(), &comment.date), leaf("Author", Vec::new(), &comment.author), leaf("Comment", Vec::new(), &comment.text)]));
    }
    for viewpoint in &topic.viewpoints {
        children.push(element("Viewpoints", vec![attr("Guid", &viewpoint.guid)], vec![leaf("Viewpoint", Vec::new(), &format!("{}.bcfv", viewpoint.guid))]));
    }
    element("Markup", Vec::new(), children)
}

/// 📦️ The bytes of the container of `document`: `bcf.version`, then per topic `markup.bcf` and its viewpoint files.
pub fn write_document(document: &Document) -> Result<Vec<u8>, String> {
    let mut zip = ZipWriter::new();
    let version = element("Version", vec![attr("VersionId", &document.version)], vec![leaf("DetailedVersion", Vec::new(), &document.version)]);
    zip.add("bcf.version", document_text(version)?.as_bytes()).map_err(|error| error.to_string())?;
    for topic in &document.topics {
        zip.add(&format!("{}/markup.bcf", topic.guid), document_text(markup_xml(topic))?.as_bytes()).map_err(|error| error.to_string())?;
        for viewpoint in &topic.viewpoints {
            zip.add(&format!("{}/{}.bcfv", topic.guid, viewpoint.guid), document_text(viewpoint_xml(viewpoint))?.as_bytes()).map_err(|error| error.to_string())?;
        }
    }
    zip.finish().map_err(|error| error.to_string())
}

/// 💬️ The BCF 2.1 container bytes of the issues of `model`.
pub fn export_bcf(model: &ModelSnapshot) -> Result<Vec<u8>, String> {
    write_document(&document_of(model))
}
//#endregion 🔖️Write

//#region 🔖️Read
fn child<'a>(node: &'a XmlNode, name: &str) -> Option<&'a XmlNode> {
    children(node).into_iter().find(|candidate| matches!(candidate, XmlNode::Element { name: found, .. } if found == name))
}

fn children(node: &XmlNode) -> Vec<&XmlNode> {
    match node {
        XmlNode::Element { children, .. } => children.iter().filter(|child| matches!(child, XmlNode::Element { .. })).collect(),
        _ => Vec::new(),
    }
}

fn named<'a>(node: &'a XmlNode, name: &str) -> Vec<&'a XmlNode> {
    children(node).into_iter().filter(|candidate| matches!(candidate, XmlNode::Element { name: found, .. } if found == name)).collect()
}

fn attribute(node: &XmlNode, name: &str) -> Option<String> {
    match node {
        XmlNode::Element { attrs, .. } => attrs.iter().find(|attribute| attribute.name == name).map(|attribute| attribute.value.clone()),
        _ => None,
    }
}

fn text_of(node: &XmlNode) -> String {
    match node {
        XmlNode::Element { children, .. } => children.iter().map(text_of).collect::<String>(),
        XmlNode::Text { text } | XmlNode::CData { text } => text.clone(),
        _ => String::new(),
    }
}

fn field(node: &XmlNode, name: &str) -> String {
    child(node, name).map(|found| text_of(found).trim().to_string()).unwrap_or_default()
}

fn vector(node: Option<&XmlNode>) -> Option<[f64; 3]> {
    let node = node?;
    Some([field(node, "X").parse().ok()?, field(node, "Y").parse().ok()?, field(node, "Z").parse().ok()?])
}

fn parse(bytes: &[u8]) -> Result<XmlNode, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("a BCF part is not UTF-8: {error}"))?;
    xml_document_from_text(text.trim_start_matches('\u{feff}'))?.root.ok_or_else(|| "a BCF part has no root element".to_string())
}

fn component_guids(node: Option<&XmlNode>) -> Vec<String> {
    node.map(|parent| named(parent, "Component").into_iter().filter_map(|component| attribute(component, "IfcGuid")).collect()).unwrap_or_default()
}

fn viewpoint_of_xml(guid: &str, root: &XmlNode) -> Viewpoint {
    let components = child(root, "Components");
    let visibility = components.and_then(|node| child(node, "Visibility"));
    let camera = child(root, "PerspectiveCamera").or_else(|| child(root, "OrthogonalCamera")).and_then(|node| {
        Some(Camera { eye: vector(child(node, "CameraViewPoint"))?, direction: vector(child(node, "CameraDirection"))?, up: vector(child(node, "CameraUpVector"))?, field_of_view: field(node, "FieldOfView").parse().unwrap_or(FIELD_OF_VIEW) })
    });
    let clipping = child(root, "ClippingPlanes").map(|planes| named(planes, "ClippingPlane").into_iter().filter_map(|plane| Some(Plane { location: vector(child(plane, "Location"))?, direction: vector(child(plane, "Direction"))? })).collect()).unwrap_or_default();
    Viewpoint {
        guid: guid.to_string(),
        camera,
        selection: component_guids(components.and_then(|node| child(node, "Selection"))),
        exceptions: component_guids(visibility.and_then(|node| child(node, "Exceptions"))),
        default_visibility: visibility.and_then(|node| attribute(node, "DefaultVisibility")).is_none_or(|flag| flag != "false"),
        clipping,
    }
}

/// 📥️ The topics of a BCF container. Entries are read through the central directory and checked against their CRC-32; a topic folder without `markup.bcf` is not a topic.
pub fn read_document(bytes: &[u8]) -> Result<Document, String> {
    let archive = ZipArchive::parse(bytes).map_err(|error| error.to_string())?;
    let version = archive.read("bcf.version", MAX_ENTRY).map_err(|error| error.to_string()).and_then(|bytes| parse(&bytes))?;
    let mut document = Document { version: attribute(&version, "VersionId").unwrap_or_default(), topics: Vec::new() };
    let folders: BTreeSet<String> = archive.entries().iter().filter_map(|entry| entry.name.strip_suffix("/markup.bcf").map(str::to_string)).collect();
    for folder in folders {
        let markup = parse(&archive.read(&format!("{folder}/markup.bcf"), MAX_ENTRY).map_err(|error| error.to_string())?)?;
        let Some(topic) = child(&markup, "Topic") else { continue };
        let mut row = Topic {
            guid: attribute(topic, "Guid").unwrap_or_else(|| folder.clone()),
            title: field(topic, "Title"),
            description: field(topic, "Description"),
            status: attribute(topic, "TopicStatus").unwrap_or_default(),
            priority: field(topic, "Priority"),
            labels: named(topic, "Labels").into_iter().map(|label| text_of(label).trim().to_string()).collect(),
            assigned_to: field(topic, "AssignedTo"),
            creation_date: field(topic, "CreationDate"),
            creation_author: field(topic, "CreationAuthor"),
            comments: named(&markup, "Comment").into_iter().map(|comment| Comment { guid: attribute(comment, "Guid").unwrap_or_default(), date: field(comment, "Date"), author: field(comment, "Author"), text: field(comment, "Comment") }).collect(),
            viewpoints: Vec::new(),
        };
        for reference in named(&markup, "Viewpoints") {
            let guid = attribute(reference, "Guid").unwrap_or_default();
            let file = Some(field(reference, "Viewpoint")).filter(|name| !name.is_empty()).unwrap_or_else(|| format!("{guid}.bcfv"));
            if let Ok(bytes) = archive.read(&format!("{folder}/{file}"), MAX_ENTRY) {
                row.viewpoints.push(viewpoint_of_xml(&guid, &parse(&bytes)?));
            }
        }
        document.topics.push(row);
    }
    Ok(document)
}
//#endregion 🔖️Read

//#region 🔖️Import
/// 📥️ What a BCF container holds, as authored records keyed by id (the guid of the topic or comment) and what could not be taken over.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImportedIssues {
    pub issues: BTreeMap<String, Issue>,
    pub comments: BTreeMap<String, IssueComment>,
    pub notes: Vec<String>,
}

fn status_of(text: &str) -> IssueStatus {
    match text.replace([' ', '-', '_'], "").to_ascii_lowercase().as_str() {
        "inprogress" | "active" | "assigned" => IssueStatus::InProgress,
        "resolved" | "fixed" => IssueStatus::Resolved,
        "closed" | "done" => IssueStatus::Closed,
        _ => IssueStatus::Open,
    }
}

fn priority_of(text: &str) -> IssuePriority {
    match text.to_ascii_lowercase().as_str() {
        "low" | "minor" => IssuePriority::Low,
        "high" | "major" => IssuePriority::High,
        "critical" | "blocker" | "urgent" => IssuePriority::Critical,
        _ => IssuePriority::Normal,
    }
}

fn camera_from(camera: &Camera) -> ViewCamera {
    let look = unit(camera.direction);
    let distance = 10.0;
    ViewCamera::from_eye_and_target(camera.eye, [camera.eye[0] + distance * look[0], camera.eye[1] + distance * look[1], camera.eye[2] + distance * look[2]])
}

/// 📦️ The section box six axis aligned clipping planes (normals pointing out of the box) enclose, none for any other set.
pub fn box_of(planes: &[Plane]) -> Option<SectionBox> {
    if planes.len() != 6 {
        return None;
    }
    let (mut low, mut high) = ([None; 3], [None; 3]);
    for plane in planes {
        let axis = (0..3).find(|axis| plane.direction[*axis].abs() > 1.0 - 1e-9 && (0..3).filter(|other| other != axis).all(|other| plane.direction[other].abs() < 1e-9))?;
        let slot = if plane.direction[axis] < 0.0 { &mut low[axis] } else { &mut high[axis] };
        if slot.replace(plane.location[axis]).is_some() {
            return None;
        }
    }
    let [Some(x0), Some(y0), Some(z0)] = low else { return None };
    let [Some(x1), Some(y1), Some(z1)] = high else { return None };
    (x0 < x1 && y0 < y1 && z0 < z1).then_some(SectionBox { min: Point3 { x: x0, y: y0, z: z0 }, max: Point3 { x: x1, y: y1, z: z1 } })
}

/// 📥️ The issues and comments of a BCF document. `resolve` maps an IFC GlobalId to the id of an element of the model (none when the model has no such element: the reference is dropped and counted in the notes).
pub fn issues_of(document: &Document, resolve: &dyn Fn(&str) -> Option<String>) -> ImportedIssues {
    let mut imported = ImportedIssues::default();
    let mut dropped = 0usize;
    for topic in &document.topics {
        let mut elements: Vec<String> = Vec::new();
        let mut isolate: Vec<String> = Vec::new();
        let mut camera = None;
        let mut section = None;
        for viewpoint in &topic.viewpoints {
            camera = camera.or(viewpoint.camera.as_ref().map(camera_from));
            section = section.or_else(|| box_of(&viewpoint.clipping));
            for guid in &viewpoint.selection {
                match resolve(guid) {
                    Some(id) if !elements.contains(&id) => elements.push(id),
                    Some(_) => {}
                    None => dropped += 1,
                }
            }
            if !viewpoint.default_visibility {
                for id in viewpoint.exceptions.iter().filter_map(|guid| resolve(guid)) {
                    if !isolate.contains(&id) {
                        isolate.push(id);
                    }
                }
            }
        }
        let mut labels: Vec<String> = Vec::new();
        for label in topic.labels.iter().filter(|label| !label.trim().is_empty()) {
            if !labels.contains(label) {
                labels.push(label.clone());
            }
        }
        let valid = |moment: &str| if crate::valid_timestamp(moment) { moment.to_string() } else { "1970-01-01".to_string() };
        imported.issues.insert(
            topic.guid.clone(),
            Issue {
                title: if topic.title.trim().is_empty() { "Untitled".into() } else { topic.title.clone() },
                description: topic.description.clone(),
                status: status_of(&topic.status),
                priority: priority_of(&topic.priority),
                assignee: topic.assigned_to.clone(),
                author: if topic.creation_author.trim().is_empty() { "unknown".into() } else { topic.creation_author.clone() },
                created: valid(&topic.creation_date),
                labels,
                elements,
                clash: None::<ClashRef>,
                viewpoint: camera.map(|camera| IssueViewpoint { camera, section, isolate }),
            },
        );
        for comment in &topic.comments {
            imported.comments.insert(
                comment.guid.clone(),
                IssueComment { issue: topic.guid.clone(), author: if comment.author.trim().is_empty() { "unknown".into() } else { comment.author.clone() }, date: valid(&comment.date), text: if comment.text.trim().is_empty() { "-".into() } else { comment.text.clone() } },
            );
        }
    }
    if dropped > 0 {
        imported.notes.push(format!("{dropped} selected component(s) name no element of the model and were dropped."));
    }
    imported
}

/// 📥️ The issues and comments of BCF container bytes, resolving IFC GlobalIds with `resolve`.
pub fn import_bcf(bytes: &[u8], resolve: &dyn Fn(&str) -> Option<String>) -> Result<ImportedIssues, String> {
    read_document(bytes).map(|document| issues_of(&document, resolve))
}

/// 🔎️ The resolver of IFC GlobalIds of `model`: the GlobalId the IFC export writes for each of its elements, spaces and zones.
pub fn resolver_of(model: &ModelSnapshot) -> impl Fn(&str) -> Option<String> {
    let ids: BTreeSet<String> = crate::classified(model).into_iter().map(|(id, _)| id).chain(model.spaces.keys().cloned()).chain(model.zones.keys().cloned()).collect();
    let by_guid: BTreeMap<String, String> = ids.into_iter().map(|id| (ifc_guid(&id), id)).collect();
    move |guid: &str| by_guid.get(guid).cloned()
}
//#endregion 🔖️Import

//#region 🔖️Table
/// ⚖️ The table the third-party oracle reads out of the container, computed from the issues of the model: `{ <title>: { status, priority, labels, assignee, author, comments, camera, selection, isolate, section } }` with
/// the camera as eye, viewing direction and up vector of the orbit camera and the components as sorted IFC GlobalIds.
pub fn table_json(model: &ModelSnapshot) -> String {
    use semio_framework_value::DslValue as V;
    let vector = |values: [f64; 3]| V::Array(values.iter().map(|value| V::float(*value)).collect());
    let guids = |ids: &[String]| {
        let mut guids: Vec<String> = ids.iter().map(|id| ifc_guid(id)).collect();
        guids.sort();
        V::Array(guids.into_iter().map(V::String).collect())
    };
    let rows = model.issues.iter().map(|(id, issue)| {
        let camera = issue.viewpoint.as_ref().map_or(V::Null, |viewpoint| {
            let camera = camera_of(&viewpoint.camera);
            V::object([("eye".to_string(), vector(camera.eye)), ("look".to_string(), vector(camera.direction)), ("up".to_string(), vector(camera.up))])
        });
        let section = issue.viewpoint.as_ref().and_then(|viewpoint| viewpoint.section.as_ref()).map_or(V::Null, |section| V::object([("min".to_string(), vector([section.min.x, section.min.y, section.min.z])), ("max".to_string(), vector([section.max.x, section.max.y, section.max.z]))]));
        let comments = crate::comments_of(model, id).into_iter().map(|(_, comment)| V::object([("author".to_string(), V::String(comment.author.clone())), ("text".to_string(), V::String(comment.text.clone()))]));
        let isolate = issue.viewpoint.as_ref().map_or_else(Vec::new, |viewpoint| viewpoint.isolate.clone());
        let row = V::object([
            ("status".to_string(), V::String(status_name(issue.status).to_string())),
            ("priority".to_string(), V::String(priority_name(issue.priority).to_string())),
            ("labels".to_string(), V::Array(issue.labels.iter().cloned().map(V::String).collect())),
            ("assignee".to_string(), V::String(issue.assignee.clone())),
            ("author".to_string(), V::String(issue.author.clone())),
            ("comments".to_string(), V::Array(comments.collect())),
            ("camera".to_string(), camera),
            ("selection".to_string(), guids(&issue.elements)),
            ("isolate".to_string(), guids(&isolate)),
            ("section".to_string(), section),
        ]);
        (issue.title.clone(), row)
    });
    semio_framework_pack_json::to_json_string(&V::object(rows))
}
//#endregion 🔖️Table

//#region 🔖️Serializer
fn refused(leaf: &str, message: impl std::fmt::Display) -> IoError {
    IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("{leaf}: {message}")))
}

/// 💬️ The BCF 2.1 serializer of the issues of the BIM model.
pub struct ModelIntoBcf;

impl Serializer<ModelSnapshot> for ModelIntoBcf {
    const INTO: Dialect = BCF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren, _: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let bytes = export_bcf(from).map_err(|message| refused("ModelIntoBcf", message))?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}

/// 💬️ The BCF 2.1 deserializer into the BIM model: a model that holds only the issues and comments of the container (elements cannot be resolved without a model, so the references are dropped and counted).
pub struct BcfIntoModel;

impl Deserializer<ModelSnapshot> for BcfIntoModel {
    const FROM: Dialect = BCF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;

    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Binary(bytes) if bytes.starts_with(b"PK") && read_document(bytes).is_ok() => Confidence::High,
            _ => Confidence::None,
        }
    }

    async fn deserialize(payload: &IoPayload, _: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<ModelSnapshot> {
        let IoPayload::Binary(bytes) = payload else { return Err(refused("BcfIntoModel", "a BCF container is binary")) };
        let imported = import_bcf(bytes, &|_| None).map_err(|message| refused("BcfIntoModel", message))?;
        let model = ModelSnapshot { issues: imported.issues, issue_comments: imported.comments, ..ModelSnapshot::default() };
        Ok(IoOutcome { value: model, diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

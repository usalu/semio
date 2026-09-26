#!/usr/bin/env python3
"""🌐️ U5 — prepared Rust half of the per-user preference lane (ticket 26/09/23 U5), held back by the hard guest freeze
(preamble rule 20): the directory schema Rust twin is guest-linked, and every hub edit below names its new variants.
Anchored edits on the CURRENT tree (robust to unrelated peer edits); `--check` only verifies every anchor is present
exactly once. Run from the repo root: python3 .tmp-ticket/wp-u5/patches/u5-preference-lane-apply.py [--check]
After applying: cargo check the hub crates and the framework directory crate, add any remaining no-op match arms the
compiler names, run the hub directory + bootstrap + access-policy tests and the Rust twin law."""
import json
import sys

CHECK = "--check" in sys.argv
FW = "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
HUB = "🌎️hub"
problems = []
writes = {}

def edit(path, old, new):
    text = writes.get(path)
    if text is None:
        text = open(path, encoding="utf-8").read()
    count = text.count(old)
    if count != 1:
        problems.append(f"{path}: anchor found {count}x: {old[:90]!r}")
        return
    writes[path] = text.replace(old, new)

SCHEMA = f"{FW}/🧬️schema/🦀️.rs"
edit(SCHEMA, '''    #[value(rename = "artifact.retention-advanced")]
    ArtifactRetentionAdvanced { retention: ArtifactRetention },
}''', '''    #[value(rename = "artifact.retention-advanced")]
    ArtifactRetentionAdvanced { retention: ArtifactRetention },
    /// 🎚️ One preference change of ONE user (`schema` names the vocabulary, `mutation` is its JSON object text — the hub
    /// never reads it). Only that user sees it, and only on the preference lane (`DIRECTORY_PREFERENCE_PAGE_PATH_V1`): the
    /// default page and every guest fold never carry it (ticket 26/09/23 U5).
    #[value(rename = "user.preference-recorded")]
    UserPreferenceRecorded { user_id: String, schema: String, mutation: String },
}''')
edit(SCHEMA, '''    AnnounceDocument { descriptor: Box<DocumentDescriptor> },
}''', '''    AnnounceDocument { descriptor: Box<DocumentDescriptor> },
    RecordUserPreference { schema: String, mutation: String },
}''')
edit(SCHEMA, '''pub const DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES: usize = 48 * 1024;''', '''pub const DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES: usize = 48 * 1024;

/// 🎚️ The preference lane's page route: the principal's own `user.preference-recorded` events on the page machinery of
/// `/directory/event-page/v1`, nothing else.
pub const DIRECTORY_PREFERENCE_PAGE_PATH_V1: &str = "/directory/preference-page/v1";
pub const USER_PREFERENCE_SCHEMA_ID_MAX_BYTES: usize = 128;
pub const USER_PREFERENCE_MUTATION_MAX_BYTES: usize = 4096;

/// 🛡️ A preference vocabulary id (1..=128 bytes of `[a-z0-9.-]`, alphanumeric at both ends) and one JSON object text of
/// at most 4096 bytes — the TypeScript twin's `validUserPreferenceRecordV1`, both pinned by
/// `🧫️fixtures/🎚️user-preference-record/🔣️.json`.
pub fn valid_user_preference_record_v1(schema: &str, mutation: &str) -> bool {
    let edge = |byte: Option<&u8>| byte.is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit());
    let schema_ok = !schema.is_empty()
        && schema.len() <= USER_PREFERENCE_SCHEMA_ID_MAX_BYTES
        && schema.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'.' || byte == b'-')
        && edge(schema.as_bytes().first())
        && edge(schema.as_bytes().last());
    schema_ok
        && mutation.len() >= 2
        && mutation.len() <= USER_PREFERENCE_MUTATION_MAX_BYTES
        && matches!(crate::os_pack::json::from_json_str::<crate::DslValue>(mutation), Ok(crate::DslValue::Object(_)))
}''')
edit(SCHEMA, '''    if event.seq == 0 || event.seq > DOCUMENT_OPEN_MAX_SAFE_INTEGER || encoded.len() > DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES''', '''    if let DirectoryEventBody::UserPreferenceRecorded { user_id, schema, mutation } = &event.body {
        if user_id.is_empty() || event.space_id.is_some() || event.user_id.as_deref() != Some(user_id.as_str()) || !valid_user_preference_record_v1(schema, mutation) {
            return Err(DirectoryEventPageErrorV1::Invalid);
        }
    }
    if event.seq == 0 || event.seq > DOCUMENT_OPEN_MAX_SAFE_INTEGER || encoded.len() > DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES''')

SCHEMA_TEST = f"{FW}/🧬️schema/🧪️tests/🔬️unit/🦀️.rs"
schema_test = open(SCHEMA_TEST, encoding="utf-8").read()
if "user_preference_record_v1_matches_the_shared_fixture" not in schema_test:
    writes[SCHEMA_TEST] = schema_test.rstrip("\n") + '''

#[test]
fn user_preference_record_v1_matches_the_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️user-preference-record/🔣️.json")).expect("user preference record fixture");
    let rows = fixture["rows"].as_array().expect("rows");
    assert!(rows.len() >= 15);
    for row in rows {
        let schema = row["schema"].as_str().expect("schema");
        let mutation = row["mutation"].as_str().expect("mutation");
        assert_eq!(valid_user_preference_record_v1(schema, mutation), row["valid"].as_bool().expect("valid"), "{}", row["id"]);
    }
    let event = |user_id: &str, space_id: Option<&str>, owner: Option<&str>| DirectoryEvent {
        seq: 1,
        id: "e1".into(),
        hlc: Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::User, id: user_id.into() },
        space_id: space_id.map(Into::into),
        user_id: owner.map(Into::into),
        body: DirectoryEventBody::UserPreferenceRecorded { user_id: user_id.into(), schema: "os.config.ui-preferences.v1".into(), mutation: "{}".into() },
        recorded_at_ms: 1,
    };
    assert!(validate_directory_event_page_event(&event("u1", None, Some("u1"))).is_ok());
    assert!(validate_directory_event_page_event(&event("u1", None, Some("u2"))).is_err(), "the event's owner is the preference's user");
    assert!(validate_directory_event_page_event(&event("u1", Some("s1"), Some("u1"))).is_err(), "a preference belongs to no space");
}
'''

edit(f"{FW}/🦀️.rs", '''        DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::ArtifactRetentionAdvanced { .. } => {}''', '''        DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::ArtifactRetentionAdvanced { .. } | DirectoryEventBody::UserPreferenceRecorded { .. } => {}''')

DIR = f"{HUB}/📇️directory/🦀️.rs"
edit(DIR, '''        DirectoryCommand::AnnounceDocument { descriptor } => {
            validate_document_descriptor(&descriptor)?;''', '''        DirectoryCommand::RecordUserPreference { schema, mutation } => {
            if !::directory::os_directory::valid_user_preference_record_v1(&schema, &mutation) {
                return Err(DirectoryError::Conflict("user preference record is invalid".into()));
            }
            let user_id = actor_user_id(actor)?.to_string();
            Ok(single(clock, actor, None, Some(user_id.clone()), DirectoryEventBody::UserPreferenceRecorded { user_id, schema, mutation }))
        }
        DirectoryCommand::AnnounceDocument { descriptor } => {
            validate_document_descriptor(&descriptor)?;''')
edit(DIR, '''            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                validate_checkpoint_shape(checkpoint)?;''', '''            DirectoryEventBody::UserPreferenceRecorded { .. } => {}
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                validate_checkpoint_shape(checkpoint)?;''')
for backend in ("🪶️sqlite", "🐘️postgres", "🌐️neo4j"):
    edit(f"{HUB}/📇️directory/{backend}/🦀️.rs", '''            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {''', '''            DirectoryEventBody::UserPreferenceRecorded { .. } => {}
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {''')

BOOT = f"{HUB}/🏗️bootstrap/🦀️.rs"
edit(BOOT, '''        DirectoryCommand::AnnounceDocument { .. } => HubAccessActionV1::DocumentAnnounce,
    }''', '''        DirectoryCommand::AnnounceDocument { .. } => HubAccessActionV1::DocumentAnnounce,
        DirectoryCommand::RecordUserPreference { .. } => HubAccessActionV1::PreferenceRecord,
    }''')
edit(BOOT, '''        DirectoryCommand::AnnounceDocument { descriptor } => Some(&descriptor.space_id),
    }''', '''        DirectoryCommand::AnnounceDocument { descriptor } => Some(&descriptor.space_id),
        DirectoryCommand::RecordUserPreference { .. } => None,
    }''')
edit(BOOT, '''        DirectoryCommand::AnnounceDocument { .. } => "announce-document",
    }''', '''        DirectoryCommand::AnnounceDocument { .. } => "announce-document",
        DirectoryCommand::RecordUserPreference { .. } => "record-user-preference",
    }''')
edit(BOOT, '''        DirectoryCommand::AnnounceDocument { descriptor } => Some(descriptor.space_id.clone()),
    }''', '''        DirectoryCommand::AnnounceDocument { descriptor } => Some(descriptor.space_id.clone()),
        DirectoryCommand::RecordUserPreference { .. } => None,
    }''')
edit(BOOT, '''fn directory_event_page_event_visible(member_spaces: &BTreeSet<String>, event: &DirectoryEvent, caller: &AuthedUser) -> bool {''', '''/// 🌐️ Which lane a directory event page serves: the directory (every event the caller may see, never a preference) or the
/// caller's own preferences (`user.preference-recorded` of the caller, nothing else). Both share one seq, one receipt and
/// one page machinery; an event of the other lane is skipped like an invisible one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirectoryEventLaneV1 {
    Directory,
    Preferences,
}

/// 🌐️ The page lane an event belongs to: a `user.preference-recorded` event rides only the preference lane.
fn directory_event_lane_v1(event: &DirectoryEvent) -> DirectoryEventLaneV1 {
    match event.body {
        os_directory::DirectoryEventBody::UserPreferenceRecorded { .. } => DirectoryEventLaneV1::Preferences,
        _ => DirectoryEventLaneV1::Directory,
    }
}

fn directory_event_page_event_visible(member_spaces: &BTreeSet<String>, event: &DirectoryEvent, caller: &AuthedUser) -> bool {''')
edit(BOOT, '''async fn build_directory_event_page_v1(state: &HubState, caller: &AuthedUser, after: u64, control: &DirectoryEventPageHttpControl) -> Result<DirectoryEventPageV1, StatusCode> {''', '''async fn build_directory_event_page_v1(state: &HubState, caller: &AuthedUser, after: u64, control: &DirectoryEventPageHttpControl, lane: DirectoryEventLaneV1) -> Result<DirectoryEventPageV1, StatusCode> {''')
edit(BOOT, '''        if !directory_event_page_event_visible(&member_spaces, &event, &caller) {''', '''        if directory_event_lane_v1(&event) != lane || !directory_event_page_event_visible(&member_spaces, &event, &caller) {''')
edit(BOOT, '''async fn get_directory_event_page_v1(OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    let after = directory_event_page_request_admission(&uri)?;''', '''async fn get_directory_event_page_v1(OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    serve_directory_event_page_v1(uri, headers, state, DirectoryEventLaneV1::Directory).await
}

/// 🌐️ `GET /directory/preference-page/v1?after=` — the caller's own preference lane on the directory page machinery.
async fn get_directory_preference_page_v1(OriginalUri(uri): OriginalUri, headers: HeaderMap, State(state): State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    serve_directory_event_page_v1(uri, headers, state, DirectoryEventLaneV1::Preferences).await
}

async fn serve_directory_event_page_v1(uri: axum::http::Uri, headers: HeaderMap, state: HubState, lane: DirectoryEventLaneV1) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode> {
    let after = directory_event_page_request_admission(&uri)?;''')
edit(BOOT, '''        let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
        build_directory_event_page_v1(&state, &caller, after, control.as_ref()).await.map(DirectoryJson)''', '''        let caller = resolve_bearer_user(&state, bearer(&headers).as_deref()).await.ok_or(StatusCode::UNAUTHORIZED)?;
        if lane == DirectoryEventLaneV1::Preferences && !hub_access_permits(&[HubAccessRoleV1::Authenticated], HubAccessActionV1::PreferenceRead, None) {
            return Err(StatusCode::FORBIDDEN);
        }
        build_directory_event_page_v1(&state, &caller, after, control.as_ref(), lane).await.map(DirectoryJson)''')
edit(BOOT, '''        .route("/directory/event-page/v1", get(get_directory_event_page_v1))''', '''        .route("/directory/event-page/v1", get(get_directory_event_page_v1))
        .route(os_directory::DIRECTORY_PREFERENCE_PAGE_PATH_V1, get(get_directory_preference_page_v1))''')

POLICY = f"{HUB}/🔐️auth/🛡️access-policy"
edit(f"{POLICY}/🦀️.rs", '''    #[serde(rename = "blob.write")]
    BlobWrite,
}''', '''    #[serde(rename = "blob.write")]
    BlobWrite,
    #[serde(rename = "preference.record")]
    PreferenceRecord,
    #[serde(rename = "preference.read")]
    PreferenceRead,
}''')
edit(f"{POLICY}/🟦️.ts", '''  | "blob.read"
  | "blob.write";''', '''  | "blob.read"
  | "blob.write"
  | "preference.record"
  | "preference.read";''')
edit(f"{POLICY}/🔣️.json", '''      "roles": [
        "authenticated"
      ],
      "actions": [
        "space.create"
      ]''', '''      "roles": [
        "authenticated"
      ],
      "actions": [
        "space.create",
        "preference.record",
        "preference.read"
      ]''')

VECTORS = f"{POLICY}/🧫️fixtures/🔣️.json"
if not problems:
    vectors = json.loads(writes.get(VECTORS) or open(VECTORS, encoding="utf-8").read())
    names = {vector["name"] for vector in vectors["vectors"]}
    for roles, action, permitted in ((["authenticated"], "preference.record", True), (["authenticated"], "preference.read", True), (["share"], "preference.record", False), (["share"], "preference.read", False)):
        name = f"{'+'.join(roles)} {action} (per-user preference lane)"
        if name not in names:
            vectors["vectors"].append({"name": name, "roles": roles, "action": action, "permitted": permitted})
    writes[VECTORS] = json.dumps(vectors, ensure_ascii=False, indent=2) + "\n"

if problems:
    print("\n".join(problems))
    sys.exit(1)
if CHECK:
    print(f"all anchors present in {len(writes)} files")
    sys.exit(0)
for path, text in writes.items():
    open(path, "w", encoding="utf-8").write(text)
print(f"applied to {len(writes)} files: " + ", ".join(writes))

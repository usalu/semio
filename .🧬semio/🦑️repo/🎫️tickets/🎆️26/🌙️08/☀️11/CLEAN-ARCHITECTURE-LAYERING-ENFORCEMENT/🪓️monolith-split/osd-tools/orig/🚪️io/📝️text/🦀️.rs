//! 🚪️ Physical Directory wire admission, canonical emission and bounded receipts.

use super::super::schema::*;
use semio_framework_value_derive::ToValue;

#[derive(Clone, Debug, PartialEq, ToValue)]
#[value(tag = "access", rename_all = "lowercase", rename_all_fields = "camelCase")]
enum DirectorySpaceAdministrationReceiptV1 {
    Public {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: PublicSpaceViewV1,
        documents: DirectorySpaceAdministrationPublicDocumentWindowV1,
    },
    Member {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: MemberSpaceViewV1,
        members: DirectorySpaceAdministrationMemberWindowV1,
        documents: DirectorySpaceAdministrationDocumentWindowV1,
    },
    Author {
        schema: String,
        session_binding_sha256: String,
        authorization_generation: u64,
        space_id: String,
        space: MemberSpaceViewV1,
        members: DirectorySpaceAdministrationMemberWindowV1,
        documents: DirectorySpaceAdministrationDocumentWindowV1,
        invites: DirectorySpaceAdministrationInviteWindowV1,
        capabilities: DirectorySpaceAdministrationCapabilitiesV1,
    },
}

#[derive(Clone, Debug, PartialEq, ToValue)]
#[value(rename_all = "camelCase")]
struct DirectoryCommandReceiptUnsignedV1 {
    schema: String,
    request_id: String,
    command_sha256: String,
    outcome: DirectoryCommandOutcomeV1,
    events: Vec<DirectoryEvent>,
    result: DirectoryCommandResultV1,
}

#[derive(Clone, Debug, PartialEq, ToValue)]
#[value(rename_all = "camelCase")]
struct DirectoryEventPageReceiptV1 {
    schema: String,
    session_binding_sha256: String,
    authorization_generation: u64,
    after_seq_exclusive: u64,
    through_seq_inclusive: u64,
    has_more: bool,
    events: Vec<DirectoryEvent>,
}

/// 🛡️ A preference vocabulary id (1..=128 bytes of `[a-z0-9.-]`, alphanumeric at both ends) and one JSON object text of
/// at most 4096 bytes — the TypeScript twin's `validUserPreferenceRecordV1`, both pinned by
/// `🧫️fixtures/🎚️user-preference-record/🔣️.json`.
    /// 🛡️ Valid user preference record v1.
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
        && matches!(semio_framework_pack_json::from_json_str::<crate::DslValue>(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject), Ok(crate::DslValue::Object(_)))
}

/// 🛡️ Admits one fully assigned event into the durable directory log and bounded page protocol.
    /// 🛡️ Validate directory event page event.
    pub fn validate_directory_event_page_event(event: &DirectoryEvent) -> Result<(), DirectoryEventPageErrorV1> {
    let encoded = semio_framework_pack_json::to_json_string(event);
    if let DirectoryEventBody::DocumentIndexed { scope, descriptor_digest_v1, entry } = &event.body {
        if !entry.validate() || descriptor_digest_v1.0 == [0; 32] || event.space_id.as_deref() != Some(scope.space_id.as_str()) || event.user_id.as_ref().is_none_or(|author| author.is_empty() || author.len() > 256) {
            return Err(DirectoryEventPageErrorV1::Invalid);
        }
    }
    if let DirectoryEventBody::UserPreferenceRecorded { user_id, schema, mutation } = &event.body {
        if user_id.is_empty() || event.space_id.is_some() || event.user_id.as_deref() != Some(user_id.as_str()) || !valid_user_preference_record_v1(schema, mutation) {
            return Err(DirectoryEventPageErrorV1::Invalid);
        }
    }
    if event.seq == 0 || event.seq > DOCUMENT_OPEN_MAX_SAFE_INTEGER || encoded.len() > DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES || directory_event_page_has_control(&crate::ToValue::to_value(event)) {
        Err(DirectoryEventPageErrorV1::Invalid)
    } else {
        Ok(())
    }
}

/// 🔐️ The one canonical command digest both the hub and every client derive independently.
    /// 🛡️ Directory command sha256.
    pub fn directory_command_sha256(command: &DirectoryCommand) -> String {
    semio_framework_hash::sha256_hex(semio_framework_pack_json::to_json_string(command).as_bytes())
}

impl DirectoryEventPageV1 {
    /// 📤️ Canonical unsigned json.
    pub fn canonical_unsigned_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&DirectoryEventPageReceiptV1 {
            schema: self.schema.clone(),
            session_binding_sha256: self.session_binding_sha256.clone(),
            authorization_generation: self.authorization_generation,
            after_seq_exclusive: self.after_seq_exclusive,
            through_seq_inclusive: self.through_seq_inclusive,
            has_more: self.has_more,
            events: self.events.clone(),
        })
    }

    /// 🛡️ Receipt matches.
    pub fn receipt_matches(&self) -> bool {
        self.receipt_sha256 == semio_framework_hash::sha256_hex(self.canonical_unsigned_json().as_bytes())
    }

    /// 🛡️ Validate.
    pub fn validate(&self) -> Result<(), DirectoryEventPageErrorV1> {
        if self.schema != "semio.directory.event-page.v1"
            || !valid_document_open_hash(&self.session_binding_sha256)
            || self.authorization_generation == 0
            || self.authorization_generation > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.after_seq_exclusive > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.through_seq_inclusive > DOCUMENT_OPEN_MAX_SAFE_INTEGER
            || self.after_seq_exclusive > self.through_seq_inclusive
            || self.events.len() > DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS
            || self.receipt_sha256.len() != 64
            || !self.receipt_sha256.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            return Err(DirectoryEventPageErrorV1::Invalid);
        }
        let mut previous = self.after_seq_exclusive;
        for event in &self.events {
            if event.seq <= previous || event.seq > self.through_seq_inclusive || validate_directory_event_page_event(event).is_err() {
                return Err(DirectoryEventPageErrorV1::Invalid);
            }
            previous = event.seq;
        }
        if !self.receipt_matches() {
            return Err(DirectoryEventPageErrorV1::ReceiptMismatch);
        }
        if semio_framework_pack_json::to_json_string(self).len() > DIRECTORY_EVENT_PAGE_MAX_BYTES {
            return Err(DirectoryEventPageErrorV1::TooLarge);
        }
        Ok(())
    }

    /// 📥️ Parse canonical json.
    pub fn parse_canonical_json(json: &str) -> Result<Self, DirectoryEventPageErrorV1> {
        if json.len() > DIRECTORY_EVENT_PAGE_MAX_BYTES {
            return Err(DirectoryEventPageErrorV1::TooLarge);
        }
        let page: Self = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| DirectoryEventPageErrorV1::Invalid)?;
        if semio_framework_pack_json::to_json_string(&page) != json {
            return Err(DirectoryEventPageErrorV1::Invalid);
        }
        page.validate()?;
        Ok(page)
    }
}

impl DirectoryCommandRequestV1 {
    /// 📤️ Canonical json.
    pub fn canonical_json(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }

    /// 🛡️ Validate.
    pub fn validate(&self) -> Result<(), DirectoryCommandErrorCodeV1> {
        if self.schema != "semio.directory.command-request.v1" || !valid_directory_command_request_id(&self.request_id) {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        if self.canonical_json().len() > DIRECTORY_COMMAND_REQUEST_MAX_BYTES {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        Ok(())
    }

    /// 📥️ Parse canonical json.
    pub fn parse_canonical_json(json: &str) -> Result<Self, DirectoryCommandErrorCodeV1> {
        if json.len() > DIRECTORY_COMMAND_REQUEST_MAX_BYTES {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        let request: Self = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| DirectoryCommandErrorCodeV1::Invalid)?;
        if request.canonical_json() != json {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        request.validate()?;
        Ok(request)
    }
}

impl DirectoryCommandReceiptV1 {
    /// 📤️ Canonical unsigned json.
    pub fn canonical_unsigned_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&DirectoryCommandReceiptUnsignedV1 {
            schema: self.schema.clone(),
            request_id: self.request_id.clone(),
            command_sha256: self.command_sha256.clone(),
            outcome: self.outcome,
            events: self.events.clone(),
            result: self.result.clone(),
        })
    }

    /// 🛡️ Seal.
    pub fn seal(request_id: impl Into<String>, command_sha256: impl Into<String>, outcome: DirectoryCommandOutcomeV1, events: Vec<DirectoryEvent>, result: DirectoryCommandResultV1) -> Self {
        let mut receipt = Self { schema: "semio.directory.command-receipt.v1".into(), request_id: request_id.into(), command_sha256: command_sha256.into(), outcome, events, result, receipt_sha256: String::new() };
        receipt.receipt_sha256 = semio_framework_hash::sha256_hex(receipt.canonical_unsigned_json().as_bytes());
        receipt
    }

    /// 🛡️ Receipt matches.
    pub fn receipt_matches(&self) -> bool {
        self.receipt_sha256 == semio_framework_hash::sha256_hex(self.canonical_unsigned_json().as_bytes())
    }

    /// 🛡️ Validate.
    pub fn validate(&self) -> Result<(), DirectoryCommandErrorCodeV1> {
        let token = match &self.result {
            DirectoryCommandResultV1::None => None,
            DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.as_str()),
        };
        if self.schema != "semio.directory.command-receipt.v1"
            || !valid_directory_command_request_id(&self.request_id)
            || !valid_document_open_hash(&self.command_sha256)
            || !valid_document_open_hash(&self.receipt_sha256)
            || self.events.len() > DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS
            || token.is_some_and(|token| token.is_empty() || token.len() > DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES || token.chars().any(char::is_control))
            || (self.outcome != DirectoryCommandOutcomeV1::Accepted && token.is_some())
        {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        let mut previous = 0;
        for event in &self.events {
            if event.seq <= previous || validate_directory_event_page_event(event).is_err() {
                return Err(DirectoryCommandErrorCodeV1::Invalid);
            }
            previous = event.seq;
        }
        if self.outcome != DirectoryCommandOutcomeV1::Accepted && !self.events.is_empty() {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        if !self.receipt_matches() {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        if semio_framework_pack_json::to_json_string(self).len() > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        Ok(())
    }

    /// 📥️ Parse canonical json.
    pub fn parse_canonical_json(json: &str, request: &DirectoryCommandRequestV1) -> Result<Self, DirectoryCommandErrorCodeV1> {
        if json.len() > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        let receipt: Self = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| DirectoryCommandErrorCodeV1::Invalid)?;
        if semio_framework_pack_json::to_json_string(&receipt) != json || receipt.request_id != request.request_id || receipt.command_sha256 != directory_command_sha256(&request.command) {
            return Err(DirectoryCommandErrorCodeV1::Invalid);
        }
        receipt.validate()?;
        Ok(receipt)
    }
}

impl DirectorySpaceAdministrationPageV1 {
    /// 📤️ Canonical unsigned json.
    pub fn canonical_unsigned_json(&self) -> String {
        let receipt = match self {
            Self::Public { schema, session_binding_sha256, authorization_generation, space_id, space, documents, .. } => DirectorySpaceAdministrationReceiptV1::Public {
                schema: schema.clone(),
                session_binding_sha256: session_binding_sha256.clone(),
                authorization_generation: *authorization_generation,
                space_id: space_id.clone(),
                space: space.clone(),
                documents: documents.clone(),
            },
            Self::Member { schema, session_binding_sha256, authorization_generation, space_id, space, members, documents, .. } => DirectorySpaceAdministrationReceiptV1::Member {
                schema: schema.clone(),
                session_binding_sha256: session_binding_sha256.clone(),
                authorization_generation: *authorization_generation,
                space_id: space_id.clone(),
                space: space.clone(),
                members: members.clone(),
                documents: documents.clone(),
            },
            Self::Author { schema, session_binding_sha256, authorization_generation, space_id, space, members, documents, invites, capabilities, .. } => DirectorySpaceAdministrationReceiptV1::Author {
                schema: schema.clone(),
                session_binding_sha256: session_binding_sha256.clone(),
                authorization_generation: *authorization_generation,
                space_id: space_id.clone(),
                space: space.clone(),
                members: members.clone(),
                documents: documents.clone(),
                invites: invites.clone(),
                capabilities: *capabilities,
            },
        };
        semio_framework_pack_json::to_json_string(&receipt)
    }

    /// 🛡️ Receipt matches.
    pub fn receipt_matches(&self) -> bool {
        self.receipt_sha256() == semio_framework_hash::sha256_hex(self.canonical_unsigned_json().as_bytes())
    }

    /// 🛡️ Validate.
    pub fn validate(&self) -> Result<(), DirectorySpaceAdministrationPageErrorV1> {
        let (schema, binding, generation, space_id) = match self {
            Self::Public { schema, session_binding_sha256, authorization_generation, space_id, .. }
            | Self::Member { schema, session_binding_sha256, authorization_generation, space_id, .. }
            | Self::Author { schema, session_binding_sha256, authorization_generation, space_id, .. } => (schema, session_binding_sha256, *authorization_generation, space_id),
        };
        let anonymous = generation == 0 && binding.bytes().all(|byte| byte == b'0');
        let bound = (1..=DOCUMENT_OPEN_MAX_SAFE_INTEGER).contains(&generation) && !binding.bytes().all(|byte| byte == b'0');
        // 🔓️ The anonymous binding IS the all-zero word (`space_administration_session_binding_v1`
        // returns `[0u8; 32]` when there is no caller), and `valid_document_open_hash` rejects exactly
        // that word — so requiring it here made every anonymous read of a PUBLIC space unconstructible
        // and the route answered 500. The shape is checked here; whether an all-zero or a real digest is
        // the admissible one is already decided by `anonymous || bound` below, and `Member`/`Author` still
        // demand `bound`.
        let binding_is_hex = binding.len() == 64 && binding.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'));
        if schema != DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA
            || !binding_is_hex
            || !valid_document_open_hash(self.receipt_sha256())
            || !directory_space_administration_text_valid(space_id)
            || !(anonymous || bound)
            || (!matches!(self, Self::Public { .. }) && !bound)
        {
            return Err(DirectorySpaceAdministrationPageErrorV1::Invalid);
        }
        let ok = match self {
            Self::Public { space, documents, .. } => {
                space.id == *space_id && space.visibility == DirectorySpaceVisibility::Public && documents.rows.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS && directory_space_administration_cursor_valid(&documents.next_cursor)
            }
            Self::Member { space, members, documents, .. } => {
                space.id == *space_id
                    && space.role == DirectorySpaceRole::Spectator
                    && directory_space_administration_members_valid(members)
                    && documents.rows.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS
                    && directory_space_administration_cursor_valid(&documents.next_cursor)
            }
            Self::Author { space, members, documents, invites, .. } => {
                space.id == *space_id
                    && space.role == DirectorySpaceRole::Author
                    && directory_space_administration_members_valid(members)
                    && documents.rows.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS
                    && directory_space_administration_cursor_valid(&documents.next_cursor)
                    && directory_space_administration_invites_valid(invites)
            }
        };
        if !ok {
            return Err(DirectorySpaceAdministrationPageErrorV1::Invalid);
        }
        if !self.receipt_matches() {
            return Err(DirectorySpaceAdministrationPageErrorV1::ReceiptMismatch);
        }
        if semio_framework_pack_json::to_json_string(self).len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES {
            return Err(DirectorySpaceAdministrationPageErrorV1::TooLarge);
        }
        Ok(())
    }

    /// 📥️ Parse canonical json.
    pub fn parse_canonical_json(json: &str) -> Result<Self, DirectorySpaceAdministrationPageErrorV1> {
        if json.len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES {
            return Err(DirectorySpaceAdministrationPageErrorV1::TooLarge);
        }
        let page: Self = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| DirectorySpaceAdministrationPageErrorV1::Invalid)?;
        if semio_framework_pack_json::to_json_string(&page) != json {
            return Err(DirectorySpaceAdministrationPageErrorV1::Invalid);
        }
        page.validate()?;
        Ok(page)
    }
}


#[path = "🪪️session-authority-v1/🦀️.rs"]
pub mod session_authority;

#[path = "🌱️space-artifact-creation-v1/🦀️.rs"]
pub mod space_artifact_creation;

#[path = "📌️document-check-in-v1/🦀️.rs"]
pub mod document_check_in;

#[path = "../../🛡️access-policy/🚪️io/📝️text/🦀️.rs"]
pub mod access_policy;

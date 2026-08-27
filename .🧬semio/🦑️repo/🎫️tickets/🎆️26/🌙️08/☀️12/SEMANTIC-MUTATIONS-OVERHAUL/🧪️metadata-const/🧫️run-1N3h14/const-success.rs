//#region 🪪️MutationLeafDescriptor
/// 🧷️ Schema vocabulary for one direct mutation's inversion behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationInvertibility {
    #[serde(rename = "self")]
    SelfInvertible,
    ExplicitMutation,
    Plan,
    NonInvertible,
}

/// 🧷️ Schema vocabulary for one direct mutation's diff participation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationDiffParticipation {
    Detect,
    ApplyOnly,
    Plan,
    None,
}

/// 🧷️ Schema vocabulary for one direct mutation's observable outcomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationOutcomeClass {
    Applied,
    Info,
    Warning,
    Error,
    Fatal,
}

/// 🧷️ Schema vocabulary for one direct mutation's composition form.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationComposition {
    Atomic,
    Composite,
}

/// 🧷️ Schema vocabulary for a direct mutation's required language surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationLanguageSurface {
    Rust,
    Typescript,
    Graphql,
    Protobuf,
    JsonSchema,
    Text,
    Binary,
}

/// 🧷️ Exact fourteen-field static metadata contract for one direct mutation leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationLeafDescriptor {
    pub schema_version: u32,
    pub owner: &'static str,
    pub semantic_kind: &'static str,
    pub display_name: &'static str,
    pub emoji: &'static str,
    pub aggregate_variant: &'static str,
    pub payload_schema: &'static str,
    pub text_opcode: Option<&'static str>,
    pub binary_tag: Option<u32>,
    pub invertibility: MutationInvertibility,
    pub diff_participation: MutationDiffParticipation,
    pub outcome_classes: &'static [MutationOutcomeClass],
    pub composition: MutationComposition,
    pub required_language_surfaces: &'static [MutationLanguageSurface],
}

/// 🧷️ Identifies a static descriptor field that violates the language-neutral schema contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafDescriptorValidationError {
    pub field: &'static str,
    pub requirement: &'static str,
}

/// 🧷️ Identifies a same-owner static roster violation and its duplicate positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafDescriptorRosterValidationError {
    pub owner: &'static str,
    pub field: &'static str,
    pub first_index: usize,
    pub index: usize,
}

impl MutationLeafDescriptor {
    /// 🧷️ Validates this exact static descriptor against the fourteen-field schema contract.
    pub const fn validate(&self) -> Result<(), MutationLeafDescriptorValidationError> {
        validate_mutation_leaf_descriptor(self)
    }
}

/// 🧷️ Validates one static descriptor without introducing defaults or partial metadata.
pub const fn validate_mutation_leaf_descriptor(descriptor: &MutationLeafDescriptor) -> Result<(), MutationLeafDescriptorValidationError> {
    if descriptor.schema_version != 1 {
        return Err(MutationLeafDescriptorValidationError { field: "schemaVersion", requirement: "must equal 1" });
    }
    if !mutation_leaf_descriptor_owner(descriptor.owner) {
        return Err(MutationLeafDescriptorValidationError { field: "owner", requirement: "must name a non-compose direct mutation leaf" });
    }
    if !mutation_leaf_descriptor_kebab(descriptor.semantic_kind) {
        return Err(MutationLeafDescriptorValidationError { field: "semanticKind", requirement: "must be a two-or-more-segment kebab identifier" });
    }
    if descriptor.display_name.is_empty() {
        return Err(MutationLeafDescriptorValidationError { field: "displayName", requirement: "must be non-empty" });
    }
    if descriptor.emoji.is_empty() {
        return Err(MutationLeafDescriptorValidationError { field: "emoji", requirement: "must be non-empty" });
    }
    if !mutation_leaf_descriptor_pascal(descriptor.aggregate_variant) {
        return Err(MutationLeafDescriptorValidationError { field: "aggregateVariant", requirement: "must be an ASCII Pascal identifier" });
    }
    if descriptor.payload_schema.is_empty() {
        return Err(MutationLeafDescriptorValidationError { field: "payloadSchema", requirement: "must be non-empty" });
    }
    if let Some(opcode) = descriptor.text_opcode {
        if !mutation_leaf_descriptor_kebab(opcode) {
            return Err(MutationLeafDescriptorValidationError { field: "textOpcode", requirement: "must be null or a two-or-more-segment kebab identifier" });
        }
    }
    if descriptor.outcome_classes.is_empty() || !mutation_leaf_descriptor_outcomes_unique(descriptor.outcome_classes) {
        return Err(MutationLeafDescriptorValidationError { field: "outcomeClasses", requirement: "must be a non-empty unique array" });
    }
    if descriptor.required_language_surfaces.is_empty() || !mutation_leaf_descriptor_surfaces_unique(descriptor.required_language_surfaces) || !mutation_leaf_descriptor_has_rust(descriptor.required_language_surfaces) {
        return Err(MutationLeafDescriptorValidationError { field: "requiredLanguageSurfaces", requirement: "must be a non-empty unique array containing rust" });
    }
    Ok(())
}

/// 🧷️ Validates exact descriptor uniqueness within one explicit owner roster.
pub const fn validate_mutation_leaf_descriptor_roster(mutation_root: &'static str, descriptors: &'static [MutationLeafDescriptor]) -> Result<(), MutationLeafDescriptorRosterValidationError> {
    if !mutation_leaf_descriptor_root(mutation_root) {
        return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: 0, index: 0 });
    }
    let mut index = 0;
    while index < descriptors.len() {
        let descriptor = &descriptors[index];
        if let Err(error) = validate_mutation_leaf_descriptor(descriptor) {
            return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: error.field, first_index: index, index });
        }
        if !mutation_leaf_descriptor_direct_child(mutation_root, descriptor.owner) {
            return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: index, index });
        }
        index += 1;
    }
    let mut index = 0;
    while index < descriptors.len() {
        let descriptor = &descriptors[index];
        let mut duplicate = index + 1;
        while duplicate < descriptors.len() {
            let other = &descriptors[duplicate];
            if mutation_leaf_descriptor_str_eq(descriptor.semantic_kind, other.semantic_kind) {
                return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "semanticKind", first_index: index, index: duplicate });
            }
            if mutation_leaf_descriptor_str_eq(descriptor.owner, other.owner) {
                return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: index, index: duplicate });
            }
            if let (Some(left), Some(right)) = (descriptor.text_opcode, other.text_opcode) {
                if mutation_leaf_descriptor_str_eq(left, right) {
                    return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "textOpcode", first_index: index, index: duplicate });
                }
            }
            if let (Some(left), Some(right)) = (descriptor.binary_tag, other.binary_tag) {
                if left == right {
                    return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "binaryTag", first_index: index, index: duplicate });
                }
            }
            duplicate += 1;
        }
        index += 1;
    }
    Ok(())
}

const MUTATION_ROOT_MARKER: &[u8] = "/🧬️mutations/".as_bytes();
const MUTATION_ROOT_SUFFIX: &[u8] = "/🧬️mutations".as_bytes();

const fn mutation_leaf_descriptor_owner(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() { return false; }
    let mut index = 0;
    let mut marker = false;
    while index < bytes.len() {
        if bytes[index] == b'\n' || bytes[index] == b'\r' || (index + 2 < bytes.len() && bytes[index] == 0xe2 && bytes[index + 1] == 0x80 && (bytes[index + 2] == 0xa8 || bytes[index + 2] == 0xa9)) { return false; }
        if (index == 0 || bytes[index - 1] == b'/') && mutation_leaf_descriptor_bytes_at(bytes, index, b"compose") && (index + 7 == bytes.len() || bytes[index + 7] == b'/') { return false; }
        if mutation_leaf_descriptor_bytes_at(bytes, index, MUTATION_ROOT_MARKER) && index > 0 && index + MUTATION_ROOT_MARKER.len() < bytes.len() { marker = true; }
        index += 1;
    }
    marker
}

const fn mutation_leaf_descriptor_root(value: &str) -> bool {
    let bytes = value.as_bytes();
    mutation_leaf_descriptor_relative_path(bytes) && mutation_leaf_descriptor_path_safe(bytes) && bytes.len() > MUTATION_ROOT_SUFFIX.len() && mutation_leaf_descriptor_bytes_at(bytes, bytes.len() - MUTATION_ROOT_SUFFIX.len(), MUTATION_ROOT_SUFFIX)
}

const fn mutation_leaf_descriptor_direct_child(root: &str, owner: &str) -> bool {
    let root = root.as_bytes();
    let owner = owner.as_bytes();
    if owner.len() <= root.len() + 1 || !mutation_leaf_descriptor_bytes_at(owner, 0, root) || owner[root.len()] != b'/' { return false; }
    let start = root.len() + 1;
    let mut index = start;
    while index < owner.len() {
        if owner[index] == b'/' || owner[index] == b'\\' { return false; }
        index += 1;
    }
    !(owner.len() == start + 1 && owner[start] == b'.') && !(owner.len() == start + 2 && owner[start] == b'.' && owner[start + 1] == b'.')
}

const fn mutation_leaf_descriptor_relative_path(bytes: &[u8]) -> bool {
    if bytes.is_empty() || bytes[0] == b'/' || bytes[0] == b'\\' { return false; }
    let mut start = 0;
    let mut index = 0;
    while index <= bytes.len() {
        if index == bytes.len() || bytes[index] == b'/' {
            let length = index - start;
            if length == 0 || (length == 1 && bytes[start] == b'.') || (length == 2 && bytes[start] == b'.' && bytes[start + 1] == b'.') { return false; }
            start = index + 1;
        } else if bytes[index] == b'\\' { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_path_safe(bytes: &[u8]) -> bool {
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\n' || bytes[index] == b'\r' || (index + 2 < bytes.len() && bytes[index] == 0xe2 && bytes[index + 1] == 0x80 && (bytes[index + 2] == 0xa8 || bytes[index + 2] == 0xa9)) { return false; }
        if (index == 0 || bytes[index - 1] == b'/') && mutation_leaf_descriptor_bytes_at(bytes, index, b"compose") && (index + 7 == bytes.len() || bytes[index + 7] == b'/') { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() < 3 || !mutation_leaf_descriptor_ascii_lower(bytes[0]) { return false; }
    let mut index = 1;
    let mut hyphen = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'-' {
            if index + 1 == bytes.len() || bytes[index + 1] == b'-' { return false; }
            hyphen = true;
        } else if !mutation_leaf_descriptor_ascii_lower(byte) && !mutation_leaf_descriptor_ascii_digit(byte) { return false; }
        index += 1;
    }
    hyphen
}

const fn mutation_leaf_descriptor_pascal(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() || !mutation_leaf_descriptor_ascii_upper(bytes[0]) { return false; }
    let mut index = 1;
    while index < bytes.len() {
        let byte = bytes[index];
        if !mutation_leaf_descriptor_ascii_upper(byte) && !mutation_leaf_descriptor_ascii_lower(byte) && !mutation_leaf_descriptor_ascii_digit(byte) { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_outcomes_unique(values: &[MutationOutcomeClass]) -> bool {
    let mut index = 0;
    while index < values.len() {
        let mut duplicate = index + 1;
        while duplicate < values.len() {
            if values[index] as u8 == values[duplicate] as u8 { return false; }
            duplicate += 1;
        }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_surfaces_unique(values: &[MutationLanguageSurface]) -> bool {
    let mut index = 0;
    while index < values.len() {
        let mut duplicate = index + 1;
        while duplicate < values.len() {
            if values[index] as u8 == values[duplicate] as u8 { return false; }
            duplicate += 1;
        }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_has_rust(values: &[MutationLanguageSurface]) -> bool {
    let mut index = 0;
    while index < values.len() {
        if values[index] as u8 == MutationLanguageSurface::Rust as u8 { return true; }
        index += 1;
    }
    false
}

const fn mutation_leaf_descriptor_bytes_at(bytes: &[u8], index: usize, needle: &[u8]) -> bool {
    if index + needle.len() > bytes.len() { return false; }
    let mut offset = 0;
    while offset < needle.len() {
        if bytes[index + offset] != needle[offset] { return false; }
        offset += 1;
    }
    true
}

const fn mutation_leaf_descriptor_str_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() { return false; }
    mutation_leaf_descriptor_bytes_at(left, 0, right)
}

const fn mutation_leaf_descriptor_ascii_lower(byte: u8) -> bool { byte >= b'a' && byte <= b'z' }
const fn mutation_leaf_descriptor_ascii_upper(byte: u8) -> bool { byte >= b'A' && byte <= b'Z' }
const fn mutation_leaf_descriptor_ascii_digit(byte: u8) -> bool { byte >= b'0' && byte <= b'9' }
//#endregion 🪪️MutationLeafDescriptor
static ROOT: &str = "✏️s/🔌️plugins/🧪️probe/🧬️mutations"; static OUTCOMES: [MutationOutcomeClass; 1] = [MutationOutcomeClass::Applied]; static SURFACES: [MutationLanguageSurface; 1] = [MutationLanguageSurface::Rust]; static FIRST: MutationLeafDescriptor = MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page", semantic_kind: "insert-page", display_name: "Insert Page", emoji: "➕️", aggregate_variant: "InsertPage", payload_schema: "🦀️.rs#InsertPage", text_opcode: Some("insert-page"), binary_tag: Some(1), invertibility: MutationInvertibility::ExplicitMutation, diff_participation: MutationDiffParticipation::ApplyOnly, outcome_classes: &OUTCOMES, composition: MutationComposition::Atomic, required_language_surfaces: &SURFACES }; static SECOND: MutationLeafDescriptor = MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page", semantic_kind: "remove-page", display_name: "Remove Page", emoji: "➖️", aggregate_variant: "RemovePage", payload_schema: "🦀️.rs#RemovePage", text_opcode: Some("remove-page"), binary_tag: Some(2), ..FIRST };
static ROSTER: [MutationLeafDescriptor; 2] = [FIRST, SECOND]; const _: () = match validate_mutation_leaf_descriptor(&FIRST) { Ok(()) => (), Err(_) => panic!("invalid") }; const _: () = match validate_mutation_leaf_descriptor_roster(ROOT, &ROSTER) { Ok(()) => (), Err(_) => panic!("duplicate") };
fn main() {}

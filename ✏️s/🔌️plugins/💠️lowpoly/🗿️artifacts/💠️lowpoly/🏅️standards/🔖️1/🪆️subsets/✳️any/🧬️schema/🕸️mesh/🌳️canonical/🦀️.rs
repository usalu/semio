//! 🌳️ Canonical JSON trees of the managed-mesh wire (`🔣️json`): the tagged intrinsic facet and the records that carry it.
//! Every view borrows the original owner in place; the 64-bit float word is produced nibble by nibble from a static table.

use super::{LowpolyMeshAttribute, LowpolyMeshMaterial};
use semio_framework_pack_json::{ArtifactCanonicalDecimalI64, ArtifactCanonicalDecimalU64, ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::paged::Utf8Text;
use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind};

const NIBBLES: &str = "0123456789abcdef";

fn refusal(reason: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, reason)
}

/// 🔢️ One binary64 as its sixteen lowercase hexadecimal digits, spelled chunk by chunk from a static table.
#[repr(transparent)]
struct FloatWord(f64);

impl FloatWord {
    fn from_ref(value: &f64) -> &Self {
        unsafe { &*(value as *const f64).cast::<Self>() }
    }
}

impl Utf8Text for FloatWord {
    fn text_bytes(&self) -> usize {
        16
    }
    fn text_chunk_count(&self) -> usize {
        16
    }
    fn text_chunk(&self, index: usize) -> Option<&str> {
        (index < 16).then(|| {
            let nibble = ((self.0.to_bits() >> (60 - 4 * index)) & 0xf) as usize;
            &NIBBLES[nibble..=nibble]
        })
    }
}

impl Tree for FloatWord {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Text(Text::Native(self)))
    }
}

/// 🏷️ One intrinsic value in its tagged facet: `{kind}` or `{kind, value | items | members}`.
#[repr(transparent)]
pub(super) struct Tagged(DslValue);

impl Tagged {
    pub(super) fn from_ref(value: &DslValue) -> &Self {
        unsafe { &*(value as *const DslValue).cast::<Self>() }
    }
    fn kind(&self) -> &'static &'static str {
        match &self.0 {
            DslValue::Null => &"null",
            DslValue::Bool(_) => &"boolean",
            DslValue::Number(Number::UInt(_)) => &"unsigned",
            DslValue::Number(Number::Int(_)) => &"signed",
            DslValue::Number(Number::Float(_)) => &"float",
            DslValue::String(_) => &"text",
            DslValue::Bytes(_) => &"bytes",
            DslValue::Array(_) => &"array",
            DslValue::Object(_) => &"object",
        }
    }
}

#[repr(transparent)]
struct TaggedList(Vec<DslValue>);

#[repr(transparent)]
struct TaggedMember((String, DslValue));

#[repr(transparent)]
struct TaggedMembers(Vec<(String, DslValue)>);

impl Tree for Tagged {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Object(if matches!(self.0, DslValue::Null) { 1 } else { 2 }))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        if ordinal == 0 {
            return Ok(self.kind());
        }
        Ok(match (&self.0, ordinal) {
            (DslValue::Bool(value), 1) => value,
            (DslValue::Number(Number::UInt(value)), 1) => ArtifactCanonicalDecimalU64::from_ref(value),
            (DslValue::Number(Number::Int(value)), 1) => ArtifactCanonicalDecimalI64::from_ref(value),
            (DslValue::Number(Number::Float(value)), 1) => FloatWord::from_ref(value),
            (DslValue::String(value), 1) => value,
            (DslValue::Bytes(value), 1) => value,
            (DslValue::Array(items), 1) => unsafe { &*(items as *const Vec<DslValue>).cast::<TaggedList>() },
            (DslValue::Object(members), 1) => unsafe { &*(members as *const Vec<(String, DslValue)>).cast::<TaggedMembers>() },
            _ => return Err(refusal("tagged intrinsic value has no such child")),
        })
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        Ok(Text::Contiguous(match (&self.0, ordinal) {
            (_, 0) => "kind",
            (DslValue::Array(_), 1) => "items",
            (DslValue::Object(_), 1) => "members",
            (DslValue::Null, _) => return Err(refusal("tagged null has only a kind")),
            (_, 1) => "value",
            _ => return Err(refusal("tagged intrinsic value has no such key")),
        }))
    }
}

impl Tree for TaggedList {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Array(self.0.len()))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        self.0.get(ordinal).map(|value| Tagged::from_ref(value) as &dyn Tree).ok_or_else(|| refusal("tagged item ordinal is absent"))
    }
}

impl Tree for TaggedMembers {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Array(self.0.len()))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        self.0.get(ordinal).map(|member| unsafe { &*(member as *const (String, DslValue)).cast::<TaggedMember>() } as &dyn Tree).ok_or_else(|| refusal("tagged member ordinal is absent"))
    }
}

impl Tree for TaggedMember {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Object(2))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match ordinal {
            0 => Ok(&self.0 .0),
            1 => Ok(Tagged::from_ref(&self.0 .1)),
            _ => Err(refusal("tagged member has two fields")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        match ordinal {
            0 => Ok(Text::Contiguous("name")),
            1 => Ok(Text::Contiguous("value")),
            _ => Err(refusal("tagged member has two keys")),
        }
    }
}

impl Tree for LowpolyMeshAttribute {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Object(6))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match ordinal {
            0 => Ok(&self.name),
            1 => Ok(&self.domain),
            2 => Ok(&self.semantic),
            3 => Ok(&self.interpolation),
            4 => Ok(unsafe { &*(&self.values as *const Vec<DslValue>).cast::<TaggedList>() }),
            5 => Ok(&self.indices),
            _ => Err(refusal("managed mesh attribute has six fields")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        ["name", "domain", "semantic", "interpolation", "values", "indices"].get(ordinal).map(|key| Text::Contiguous(key)).ok_or_else(|| refusal("managed mesh attribute has six keys"))
    }
}

impl Tree for LowpolyMeshMaterial {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Object(2))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match ordinal {
            0 => Ok(&self.name),
            1 => Ok(Tagged::from_ref(&self.value)),
            _ => Err(refusal("managed mesh material has two fields")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        ["name", "value"].get(ordinal).map(|key| Text::Contiguous(key)).ok_or_else(|| refusal("managed mesh material has two keys"))
    }
}

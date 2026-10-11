//! 🌳️ Canonical JSON tree of the dictionary wire: `{entries:[{questionId, value}]}` with the limb-tagged intrinsic facet.
//! Every view borrows the original owner in place; 32-bit limbs of the 64-bit words are computed per node.

use super::{FormDictionary, FormDictionaryEntry};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind};

fn refusal(reason: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, reason)
}

/// 🔢️ One word's upper or lower 32-bit limb.
#[repr(transparent)]
struct Limb<const HIGH: bool>(u64);

impl<const HIGH: bool> Tree for Limb<HIGH> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::U64(if HIGH { self.0 >> 32 } else { self.0 & 0xffff_ffff }))
    }
}

/// 🔢️ A signed word is spelled as its two's-complement bits.
#[repr(transparent)]
struct SignedLimb<const HIGH: bool>(i64);

impl<const HIGH: bool> Tree for SignedLimb<HIGH> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        let word = self.0 as u64;
        Ok(Node::U64(if HIGH { word >> 32 } else { word & 0xffff_ffff }))
    }
}

/// 🔢️ A binary64 is spelled as its bit pattern limbs.
#[repr(transparent)]
struct FloatLimb<const HIGH: bool>(f64);

impl<const HIGH: bool> Tree for FloatLimb<HIGH> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        let word = self.0.to_bits();
        Ok(Node::U64(if HIGH { word >> 32 } else { word & 0xffff_ffff }))
    }
}

#[repr(transparent)]
struct Tagged(DslValue);

#[repr(transparent)]
struct TaggedList(Vec<DslValue>);

#[repr(transparent)]
struct TaggedMember((String, DslValue));

#[repr(transparent)]
struct TaggedMembers(Vec<(String, DslValue)>);

impl Tagged {
    fn from_ref(value: &DslValue) -> &Self {
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
    fn fields(&self) -> usize {
        match &self.0 {
            DslValue::Null => 1,
            DslValue::Number(_) => 3,
            _ => 2,
        }
    }
}

impl Tree for Tagged {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Object(self.fields()))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        if ordinal == 0 {
            return Ok(self.kind());
        }
        Ok(match (&self.0, ordinal) {
            (DslValue::Bool(value), 1) => value,
            (DslValue::Number(Number::UInt(value)), 1) => unsafe { &*(value as *const u64).cast::<Limb<true>>() },
            (DslValue::Number(Number::UInt(value)), 2) => unsafe { &*(value as *const u64).cast::<Limb<false>>() },
            (DslValue::Number(Number::Int(value)), 1) => unsafe { &*(value as *const i64).cast::<SignedLimb<true>>() },
            (DslValue::Number(Number::Int(value)), 2) => unsafe { &*(value as *const i64).cast::<SignedLimb<false>>() },
            (DslValue::Number(Number::Float(value)), 1) => unsafe { &*(value as *const f64).cast::<FloatLimb<true>>() },
            (DslValue::Number(Number::Float(value)), 2) => unsafe { &*(value as *const f64).cast::<FloatLimb<false>>() },
            (DslValue::String(value), 1) => value,
            (DslValue::Bytes(value), 1) => value,
            (DslValue::Array(items), 1) => unsafe { &*(items as *const Vec<DslValue>).cast::<TaggedList>() },
            (DslValue::Object(members), 1) => unsafe { &*(members as *const Vec<(String, DslValue)>).cast::<TaggedMembers>() },
            _ => return Err(refusal("dictionary tagged value has no such child")),
        })
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        Ok(Text::Contiguous(match (&self.0, ordinal) {
            (_, 0) => "kind",
            (DslValue::Array(_), 1) => "items",
            (DslValue::Object(_), 1) => "members",
            (DslValue::Number(_), 1) => "high",
            (DslValue::Number(_), 2) => "low",
            (DslValue::Null, _) => return Err(refusal("dictionary tagged null has only a kind")),
            (_, 1) => "value",
            _ => return Err(refusal("dictionary tagged value has no such key")),
        }))
    }
}

impl Tree for TaggedList {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Array(self.0.len()))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        self.0.get(ordinal).map(|value| Tagged::from_ref(value) as &dyn Tree).ok_or_else(|| refusal("dictionary tagged item ordinal is absent"))
    }
}

impl Tree for TaggedMembers {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Array(self.0.len()))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        self.0.get(ordinal).map(|member| unsafe { &*(member as *const (String, DslValue)).cast::<TaggedMember>() } as &dyn Tree).ok_or_else(|| refusal("dictionary tagged member ordinal is absent"))
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
            _ => Err(refusal("dictionary tagged member has two fields")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        ["name", "value"].get(ordinal).map(|key| Text::Contiguous(key)).ok_or_else(|| refusal("dictionary tagged member has two keys"))
    }
}

impl Tree for FormDictionaryEntry {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Object(2))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match ordinal {
            0 => Ok(&self.question_id),
            1 => Ok(Tagged::from_ref(&self.value)),
            _ => Err(refusal("dictionary entry has two fields")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        ["questionId", "value"].get(ordinal).map(|key| Text::Contiguous(key)).ok_or_else(|| refusal("dictionary entry has two keys"))
    }
}

impl Tree for FormDictionary {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(Node::Object(1))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match ordinal {
            0 => Ok(&self.entries),
            _ => Err(refusal("dictionary has one field")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        match ordinal {
            0 => Ok(Text::Contiguous("entries")),
            _ => Err(refusal("dictionary has one key")),
        }
    }
}

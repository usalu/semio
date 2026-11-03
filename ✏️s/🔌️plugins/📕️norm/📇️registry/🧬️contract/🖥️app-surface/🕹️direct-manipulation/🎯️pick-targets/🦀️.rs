//! 🎯️ Pick targets: sub-part ids of picked entities, handle precedence over entities and the handle set of a selection. See [`super`].

use semio_s_plugin_norm_structural_scene::{EntityRef, Handle};

/// 🧩️ The part of an entity a pick hit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickPart {
    Whole,
    Face(u32),
    Edge(u32),
    Vertex(u32),
    Row(u32),
}

/// 🎯️ One pick: the raw entity id plus the part, spelled `id` or `id#face:3`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PickTarget {
    pub entity_id: String,
    pub part: PickPart,
}

/// 🏁️ What one pointer hit resolves to; a grip always wins over the entity behind it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PickOutcome {
    Handle(String),
    Entity(PickTarget),
    Nothing,
}

impl PickPart {
    fn token(self) -> Option<String> {
        match self {
            Self::Whole => None,
            Self::Face(n) => Some(format!("face:{n}")),
            Self::Edge(n) => Some(format!("edge:{n}")),
            Self::Vertex(n) => Some(format!("vertex:{n}")),
            Self::Row(n) => Some(format!("row:{n}")),
        }
    }

    fn parse(token: &str) -> Option<Self> {
        let (kind, index) = token.split_once(':')?;
        let n = index.parse().ok()?;
        match kind {
            "face" => Some(Self::Face(n)),
            "edge" => Some(Self::Edge(n)),
            "vertex" => Some(Self::Vertex(n)),
            "row" => Some(Self::Row(n)),
            _ => None,
        }
    }
}

impl PickTarget {
    pub fn whole(entity_id: impl Into<String>) -> Self {
        Self { entity_id: entity_id.into(), part: PickPart::Whole }
    }

    /// 🔡️ Parses `id` or `id#part:n`; `None` for an empty id or an unknown part.
    pub fn parse(raw: &str) -> Option<Self> {
        let (id, part) = match raw.split_once('#') {
            Some((id, token)) => (id, PickPart::parse(token)?),
            None => (raw, PickPart::Whole),
        };
        (!id.is_empty()).then(|| Self { entity_id: id.to_string(), part })
    }

    /// 🔡️ The wire spelling; `parse(format())` is the identity.
    pub fn format(&self) -> String {
        self.part.token().map_or_else(|| self.entity_id.clone(), |token| format!("{}#{token}", self.entity_id))
    }

    /// 🕹️ True when `handle` is placed on this target: same entity and, for a sub-part, a `/part` segment in its id.
    pub fn owns(&self, handle: &Handle) -> bool {
        handle.entity.id == self.entity_id && self.part.token().is_none_or(|token| handle.id.split('/').any(|segment| segment == token))
    }
}

/// 🏁️ Resolves one hit: a hit grip wins, else the entity hit, else nothing.
pub fn resolve_pick(handle_hit: Option<&str>, entity_hit: Option<&str>) -> PickOutcome {
    match (handle_hit, entity_hit.and_then(PickTarget::parse)) {
        (Some(id), _) => PickOutcome::Handle(id.to_string()),
        (None, Some(target)) => PickOutcome::Entity(target),
        (None, None) => PickOutcome::Nothing,
    }
}

/// 🕹️ The handles of the selected entities in selection order, then handle id: the stable Tab order.
pub fn handles_for<'a>(handles: &'a [Handle], selection: &[EntityRef]) -> Vec<&'a Handle> {
    let mut out: Vec<&Handle> = Vec::new();
    for entity in selection {
        let mut own: Vec<&Handle> = handles.iter().filter(|h| h.entity.id == entity.id && h.entity.kind == entity.kind && out.iter().all(|seen| seen.id != h.id)).collect();
        own.sort_by(|a, b| a.id.cmp(&b.id));
        out.extend(own);
    }
    out
}

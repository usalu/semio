//! 🕰️ The vocabulary of construction phases shared by every consumer: the four phases in their natural order, their stable keys, the parse of a typed name and the default phase of a freshly authored element.
//! The phase of an element is authored (`phase` on walls, curtain walls, columns, beams, slabs, roofs, stairs, railings and spaces; an opening takes the phase of its host); which elements a view shows per phase is inferred.

use super::values::Phase;

impl Phase {
    /// 🔢️ Every phase in the order of a project's life: what stands, what is built, what is torn down, what only serves the works.
    pub const ALL: [Phase; 4] = [Phase::Existing, Phase::New, Phase::Demolished, Phase::Temporary];

    /// 🏷️ The stable lowercase key of the phase (the keys of the per-phase quantity tables and of the view phase).
    pub const fn key(self) -> &'static str {
        match self {
            Phase::Existing => "existing",
            Phase::New => "new",
            Phase::Demolished => "demolished",
            Phase::Temporary => "temporary",
        }
    }

    /// 🔎️ The phase a typed name denotes, ignoring case and surrounding blanks; `None` for anything else.
    pub fn parse(text: &str) -> Option<Phase> {
        Phase::ALL.into_iter().find(|phase| phase.key().eq_ignore_ascii_case(text.trim()))
    }
}

/// 🆕 A freshly authored element is new construction.
impl Default for Phase {
    fn default() -> Self {
        Phase::New
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//! 🕰️ The vocabulary of construction phases shared by every consumer: the four phases in their natural order, their stable keys, the parse of a typed name and the default phase of a freshly authored element.
//! The phase of an element is authored (`phase` on walls, curtain walls, columns, beams, slabs, roofs, stairs, railings and spaces; an opening takes the phase of its host); which elements a view shows per phase is inferred.

use super::values::Phase;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

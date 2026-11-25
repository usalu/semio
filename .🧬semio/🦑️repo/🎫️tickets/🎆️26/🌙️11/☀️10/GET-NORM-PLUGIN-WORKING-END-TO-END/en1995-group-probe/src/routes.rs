//! 🧬 Selector rows derived from the interleaved prototype roster.

use super::{Descriptor, Group};

pub(super) const DESCRIPTORS: &[Descriptor] = &[
    Descriptor { tag: "ChangeDeckSpan", number: 3, route: Group::Deck(0) },
    Descriptor { tag: "ChangeTaperTaperAngle", number: 1, route: Group::Taper(0) },
    Descriptor { tag: "ChangeDeckDeckWidth", number: 4, route: Group::Deck(1) },
    Descriptor { tag: "ChangeTaperApexDepth", number: 2, route: Group::Taper(1) },
];

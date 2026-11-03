//! 🏷️ Annotations: dimensions, labels, leaders, clause badges, failure glyphs and legends layered over entities.

use super::*;

records! {
    /// 🗺️ Colour-key entry set explaining one colour channel.
    pub struct Legend {
        pub id: String,
        pub title: LocalizedCopy,
        pub scale: LegendScale,
    }

    /// 🏷️ One role swatch of a role legend.
    pub struct LegendRole {
        pub role: MaterialRole,
        pub label: LocalizedCopy,
    }
}

choices! {
    /// 🎭️ Emphasis of an annotation text.
    pub enum Tone {
        Neutral,
        Info,
        Warning,
        Error,
    }

    /// 💢️ Failure mechanism glyph class.
    pub enum FailureMode {
        LateralTorsionalBuckling,
        FlexuralBuckling,
        Yielding,
        Crushing,
        Punching,
        SlipCircle,
        ShearCone,
        BlockTearOut,
        Overturning,
        Sliding,
        Fatigue,
        Other,
    }
}

variants! {
    /// 📝️ Overlay item attached to a position or entity.
    pub enum Annotation {
        Dimension { id: String, from: Vec3, to: Vec3, offset: Vec3, value: Quantity, binding: Option<FieldBinding> },
        Label { id: String, anchor: Vec3, text: LocalizedCopy, tone: Tone },
        Leader { id: String, anchor: Vec3, text: LocalizedCopy, offset_px: Vec<i32> },
        Clause { id: String, anchor: EntityRef, clause: ClauseId },
        FailureGlyph { id: String, at: EntityRef, mode: FailureMode, extent: Option<Polygon3> },
    }

    /// 📐️ What a legend maps.
    pub enum LegendScale {
        Utilisation { map: ColourMapRef },
        Roles { entries: Vec<LegendRole> },
        Scalar { field_id: String, map: ColourMapRef, unit: QuantityKind },
    }
}

impl Annotation {
    pub fn id(&self) -> &str {
        match self {
            Self::Dimension { id, .. } | Self::Label { id, .. } | Self::Leader { id, .. } | Self::Clause { id, .. } | Self::FailureGlyph { id, .. } => id,
        }
    }
}

impl Validate for LegendRole {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.role.check(&at(path, "role"), out);
        self.label.check(&at(path, "label"), out);
    }
}

impl Validate for LegendScale {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Utilisation { map } => map.check(&at(path, "map"), out),
            Self::Roles { entries } => {
                expect(out, &at(path, "entries"), !entries.is_empty(), "non-empty");
                each(entries, path, "entries", out);
            }
            Self::Scalar { field_id, map, .. } => {
                non_empty(out, &at(path, "fieldId"), field_id);
                map.check(&at(path, "map"), out);
            }
        }
    }
}

impl Validate for Legend {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.title.check(&at(path, "title"), out);
        self.scale.check(&at(path, "scale"), out);
    }
}

impl Validate for Annotation {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), self.id());
        match self {
            Self::Dimension { from, to, offset, value, .. } => {
                from.check(&at(path, "from"), out);
                to.check(&at(path, "to"), out);
                offset.check(&at(path, "offset"), out);
                value.check(&at(path, "value"), out);
                expect(out, path, from.distance(to) > 0.0, "non-degenerate");
            }
            Self::Label { anchor, text, .. } => {
                anchor.check(&at(path, "anchor"), out);
                text.check(&at(path, "text"), out);
            }
            Self::Leader { anchor, text, offset_px, .. } => {
                anchor.check(&at(path, "anchor"), out);
                text.check(&at(path, "text"), out);
                expect(out, &at(path, "offsetPx"), offset_px.len() == 2, "two-components");
            }
            Self::Clause { anchor, clause, .. } => {
                anchor.check(&at(path, "anchor"), out);
                clause.check(&at(path, "clause"), out);
            }
            Self::FailureGlyph { at: target, extent, .. } => {
                target.check(&at(path, "at"), out);
                extent.check(&at(path, "extent"), out);
            }
        }
    }
}

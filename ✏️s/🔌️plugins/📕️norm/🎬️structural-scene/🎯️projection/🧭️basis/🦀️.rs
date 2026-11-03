//! 🧭️ Basis: the annex and the standard edition a scene and its diagrams were computed under, carried as one badge in the legends of the scene (shown in the 3D viewport) and as the header note of every diagram.
//!
//! 🔗️ Fed by the document annex through [`StructuralProjection::basis`](super::StructuralProjection::basis); drawn by the viewport lowering and the diagrams window of the app surface.

use crate::{Diagram2d, DiagramNote, Legend, LegendRole, LegendScale, LocalizedCopy, MaterialRole, StructuralScene, Tone, Vec2, BASIS_NOTE_ID};
use norm::{AnnexChoice, NormFamilyId};

/// 🧭️ Id of the scene legend that carries the basis badge.
pub const BASIS_LEGEND_ID: &str = "basis";

/// 🧭️ The basis of one projection: the family with its standard edition and the annex the document selects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Basis {
    pub family: NormFamilyId,
    pub annex: AnnexChoice,
}

impl Basis {
    pub fn new(family: NormFamilyId, annex: AnnexChoice) -> Self {
        Self { family, annex }
    }

    /// 📅️ The standard edition of the family, its label where no edition is recorded.
    pub fn standard(&self) -> &'static str {
        self.family.edition().unwrap_or_else(|| self.family.label())
    }

    /// 🏷️ The badge text in both languages: `Basis: annex EN · EN 1993-1-1:2005+A1:2014`.
    pub fn badge(&self) -> LocalizedCopy {
        let (annex, standard) = (self.annex.label(), self.standard());
        LocalizedCopy::new(format!("Basis: annex {annex} · {standard}"), format!("Grundlage: Anhang {annex} · {standard}"))
    }

    /// 🗺️ The scene legend of the badge, one neutral swatch labelled with the badge.
    pub fn legend(&self) -> Legend {
        let badge = self.badge();
        Legend { id: BASIS_LEGEND_ID.into(), title: LocalizedCopy::new("Basis", "Grundlage"), scale: LegendScale::Roles { entries: vec![LegendRole { role: MaterialRole::Neutral, label: badge }] } }
    }

    /// 📌️ The header note of a diagram; it sits at the origin and is never drawn inside the plot (see [`Diagram2d::plotted_notes`]).
    pub fn note(&self) -> DiagramNote {
        DiagramNote { id: BASIS_NOTE_ID.into(), at: Vec2::ZERO, text: self.badge(), tone: Tone::Neutral }
    }

    /// 🎬️ Puts the badge into the scene's legends, replacing a previous one so the badge follows the annex of the latest document.
    pub fn apply(&self, scene: &mut StructuralScene) {
        scene.legends.retain(|l| l.id != BASIS_LEGEND_ID);
        scene.legends.push(self.legend());
    }

    /// 📈️ Puts the badge into the header of every diagram, replacing a previous one.
    pub fn apply_to_diagrams(&self, diagrams: &mut [Diagram2d]) {
        for diagram in diagrams {
            diagram.annotations.retain(|n| n.id != BASIS_NOTE_ID);
            diagram.annotations.push(self.note());
        }
    }
}

/// 📈️ `diagrams` with the basis badge in every header.
pub fn with_basis(mut diagrams: Vec<Diagram2d>, basis: &Basis) -> Vec<Diagram2d> {
    basis.apply_to_diagrams(&mut diagrams);
    diagrams
}

/// 🧭️ The badge text of a scene, if its projection fed one.
pub fn basis_title(scene: &StructuralScene) -> Option<&LocalizedCopy> {
    scene.legends.iter().find(|l| l.id == BASIS_LEGEND_ID).and_then(|l| match &l.scale {
        LegendScale::Roles { entries } => entries.first().map(|e| &e.label),
        _ => None,
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

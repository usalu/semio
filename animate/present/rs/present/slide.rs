//! 🎭 Scene-based presentation document types for slide/section timelines.

use animate_core::Section;
use serde::{Deserialize, Serialize};

pub const PRESENT_SCENE_SCHEMA: &str = "animate.present.scene";

/// 🖼️ One slide within a presentation section — may reference a compiled animate scene hash.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentSlide {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timeline_sections: Vec<Section>,
}

/// 📚 Vertical column of slides (reveal.js sequence analogue).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentSection {
    pub id: String,
    pub title: String,
    pub slides: Vec<PresentSlide>,
}

/// 🎬 Full scene-based presentation document — sections of slides plus optional tile deck overlay.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentScene {
    pub schema: String,
    pub title: String,
    pub sections: Vec<PresentSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deck: Option<crate::PresentDeck>,
}

impl PresentScene {
    pub fn empty(title: impl Into<String>) -> Self {
        Self {
            schema: PRESENT_SCENE_SCHEMA.into(),
            title: title.into(),
            sections: Vec::new(),
            deck: None,
        }
    }

    pub fn slide_count(&self) -> usize {
        self.sections.iter().map(|section| section.slides.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_scene_counts_slides() {
        let scene = PresentScene {
            schema: PRESENT_SCENE_SCHEMA.into(),
            title: "Demo".into(),
            sections: vec![
                PresentSection {
                    id: "s1".into(),
                    title: "Intro".into(),
                    slides: vec![
                        PresentSlide {
                            id: "a".into(),
                            title: "A".into(),
                            scene_hash: None,
                            timeline_sections: Vec::new(),
                        },
                        PresentSlide {
                            id: "b".into(),
                            title: "B".into(),
                            scene_hash: Some("abc123".into()),
                            timeline_sections: vec![Section::new("main", 0.0, 5.0)],
                        },
                    ],
                },
            ],
            deck: None,
        };
        assert_eq!(scene.slide_count(), 2);
    }
}

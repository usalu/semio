//! 📑 Named sections for partial movie output and navigation.

use serde::{Deserialize, Serialize};

/// 🏷️ Single named section within a scene timeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub start_time: f64,
    pub end_time: f64,
    pub skip_animations: bool,
}

impl Section {
    pub fn new(name: impl Into<String>, start_time: f64, end_time: f64) -> Self {
        Self { name: name.into(), start_time, end_time, skip_animations: false }
    }

    pub fn duration(&self) -> f64 {
        (self.end_time - self.start_time).max(0.0)
    }

    pub fn contains_time(&self, t: f64) -> bool {
        t >= self.start_time && t <= self.end_time
    }
}

/// 📚 Ordered section list attached to a scene.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SectionList {
    pub sections: Vec<Section>,
    open: Option<Section>,
}

impl SectionList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_section(&mut self, name: impl Into<String>, skip_animations: bool) {
        self.open = Some(Section { name: name.into(), start_time: 0.0, end_time: 0.0, skip_animations });
    }

    pub fn end_section(&mut self, end_time: f64) {
        if let Some(mut s) = self.open.take() {
            s.end_time = end_time;
            self.sections.push(s);
        }
    }

    pub fn push(&mut self, section: Section) {
        self.sections.push(section);
    }

    pub fn find_at_time(&self, t: f64) -> Option<&Section> {
        self.sections.iter().find(|s| s.contains_time(t))
    }

    pub fn names(&self) -> Vec<&str> {
        self.sections.iter().map(|s| s.name.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_duration_is_non_negative() {
        let s = Section::new("intro", 0.0, 2.5);
        assert!((s.duration() - 2.5).abs() < 1e-9);
    }

    #[test]
    fn section_list_tracks_open_close() {
        let mut list = SectionList::new();
        list.begin_section("main", false);
        list.end_section(10.0);
        assert_eq!(list.sections.len(), 1);
        assert_eq!(list.sections[0].name, "main");
    }
}

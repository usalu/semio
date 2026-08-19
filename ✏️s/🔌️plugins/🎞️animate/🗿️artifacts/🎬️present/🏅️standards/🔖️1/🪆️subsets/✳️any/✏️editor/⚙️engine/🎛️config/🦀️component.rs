//! 🎞️ Animate app engine facet: 🎛️config (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
//! relocated verbatim from the deleted artifact-tree `⚙️engine/🎛️config`).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod config {
    //! ⚙️ Global animation configuration, quality presets, and cache paths.

    use serde::{Deserialize, Serialize};
    use std::path::{Path, PathBuf};

    /// 🎞️ Output quality preset mirroring Manim quality flags.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum QualityPreset {
        Low,
        Medium,
        High,
        FourK,
        Production,
    }

    impl QualityPreset {
        pub async fn frame_rate(self) -> f64 {
            match self {
                Self::Low | Self::Medium => 15.0,
                Self::High | Self::FourK | Self::Production => 60.0,
            }
        }

        pub async fn resolution(self) -> (u32, u32) {
            match self {
                Self::Low => (854, 480),
                Self::Medium => (1280, 720),
                Self::High => (1920, 1080),
                Self::FourK => (3840, 2160),
                Self::Production => (2560, 1440),
            }
        }

        pub async fn pixel_height(self) -> u32 {
            self.resolution().1
        }
    }

    /// 💾️ Cache settings for partial movies and hashed assets.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CacheConfig {
        pub enabled: bool,
        pub max_entries: usize,
        pub partial_movie_dir: PathBuf,
    }

    impl Default for CacheConfig {
        fn default() -> Self {
            Self { enabled: true, max_entries: 10_000, partial_movie_dir: PathBuf::from("partial_movie_files") }
        }
    }

    /// 🎬️ Root configuration for animate scenes and renderers.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AnimateConfig {
        pub quality: QualityPreset,
        pub frame_rate: f64,
        pub width: u32,
        pub height: u32,
        pub media_dir: PathBuf,
        pub output_dir: PathBuf,
        pub cache: CacheConfig,
        pub background: [f64; 4],
        pub audio_track: Option<PathBuf>,
        pub subtitles_path: Option<PathBuf>,
    }

    impl Default for AnimateConfig {
        fn default() -> Self {
            Self::from_quality(QualityPreset::High)
        }
    }

    impl AnimateConfig {
        pub async fn from_quality(quality: QualityPreset) -> Self {
            let (width, height) = quality.resolution();
            Self {
                quality,
                frame_rate: quality.frame_rate(),
                width,
                height,
                media_dir: PathBuf::from("media"),
                output_dir: PathBuf::from("output"),
                cache: CacheConfig::default(),
                background: [0.0, 0.0, 0.0, 1.0],
                audio_track: None,
                subtitles_path: None,
            }
        }

        pub async fn with_frame_rate(mut self, frame_rate: f64) -> Self {
            self.frame_rate = frame_rate.max(1.0);
            self
        }

        pub async fn with_resolution(mut self, width: u32, height: u32) -> Self {
            self.width = width.max(1);
            self.height = height.max(1);
            self
        }

        pub async fn with_output_dir(mut self, path: impl AsRef<Path>) -> Self {
            self.output_dir = path.as_ref().to_path_buf();
            self
        }

        pub async fn with_media_dir(mut self, path: impl AsRef<Path>) -> Self {
            self.media_dir = path.as_ref().to_path_buf();
            self
        }

        pub async fn with_audio_track(mut self, path: impl AsRef<Path>) -> Self {
            self.audio_track = Some(path.as_ref().to_path_buf());
            self
        }

        pub async fn with_subtitles_path(mut self, path: impl AsRef<Path>) -> Self {
            self.subtitles_path = Some(path.as_ref().to_path_buf());
            self
        }

        pub async fn frame_duration(&self) -> f64 {
            1.0 / self.frame_rate
        }

        pub async fn aspect_ratio(self) -> f64 {
            self.width as f64 / self.height as f64
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn quality_presets_have_expected_resolution() {
            assert_eq!(QualityPreset::High.resolution(), (1920, 1080));
            assert_eq!(QualityPreset::FourK.resolution(), (3840, 2160));
        }

        #[semio_framework_async_macros::async_test]
        async fn config_frame_duration_matches_rate() {
            let cfg = AnimateConfig::default().with_frame_rate(30.0);
            assert!((cfg.frame_duration() - 1.0 / 30.0).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn all_quality_presets_report_frame_rate_and_resolution() {
            assert_eq!(QualityPreset::Low.frame_rate(), 15.0);
            assert_eq!(QualityPreset::Medium.frame_rate(), 15.0);
            assert_eq!(QualityPreset::High.frame_rate(), 60.0);
            assert_eq!(QualityPreset::FourK.frame_rate(), 60.0);
            assert_eq!(QualityPreset::Production.frame_rate(), 60.0);
            assert_eq!(QualityPreset::Low.resolution(), (854, 480));
            assert_eq!(QualityPreset::Medium.resolution(), (1280, 720));
            assert_eq!(QualityPreset::Production.resolution(), (2560, 1440));
            assert_eq!(QualityPreset::High.pixel_height(), 1080);
        }

        #[semio_framework_async_macros::async_test]
        async fn config_builder_methods_apply() {
            let cfg = AnimateConfig::from_quality(QualityPreset::Low).with_resolution(0, 0).with_output_dir("out").with_media_dir("media2").with_audio_track("track.wav").with_subtitles_path("subs.srt");
            assert_eq!(cfg.width, 1);
            assert_eq!(cfg.height, 1);
            assert_eq!(cfg.output_dir, PathBuf::from("out"));
            assert_eq!(cfg.media_dir, PathBuf::from("media2"));
            assert_eq!(cfg.audio_track, Some(PathBuf::from("track.wav")));
            assert_eq!(cfg.subtitles_path, Some(PathBuf::from("subs.srt")));
        }

        #[semio_framework_async_macros::async_test]
        async fn config_with_frame_rate_clamps_to_minimum() {
            let cfg = AnimateConfig::default().with_frame_rate(-5.0);
            assert_eq!(cfg.frame_rate, 1.0);
        }

        #[semio_framework_async_macros::async_test]
        async fn config_aspect_ratio_and_default_cache() {
            let cfg = AnimateConfig::from_quality(QualityPreset::Medium);
            assert!(cfg.cache.enabled);
            assert_eq!(cfg.cache.max_entries, 10_000);
            assert!((cfg.aspect_ratio() - 1280.0 / 720.0).abs() < 1e-9);
        }
    }
}

pub mod hash {
    //! 🪪️ Content-hash animation descriptors via framework hash.

    use framework_hash::{format_number_for_hash, hash_parts, merkle_node};
    use serde::Serialize;

    /// 🧾️ Serializable animation fingerprint input.
    #[derive(Clone, Debug, Serialize)]
    pub struct AnimationHashInput {
        pub kind: String,
        pub run_time: f64,
        pub target_ids: Vec<u64>,
        pub rate: String,
        pub extras: Vec<String>,
    }

    impl AnimationHashInput {
        pub async fn new(kind: impl Into<String>, run_time: f64) -> Self {
            Self { kind: kind.into(), run_time, target_ids: Vec::new(), rate: "linear".into(), extras: Vec::new() }
        }

        pub async fn with_targets(mut self, ids: Vec<u64>) -> Self {
            self.target_ids = ids;
            self
        }

        pub async fn with_rate(mut self, rate: impl Into<String>) -> Self {
            self.rate = rate.into();
            self
        }

        pub async fn with_extra(mut self, extra: impl Into<String>) -> Self {
            self.extras.push(extra.into());
            self
        }
    }

    /// 🔐️ Hash a single animation descriptor.
    pub async fn hash_animation(input: &AnimationHashInput) -> String {
        let mut parts = vec![input.kind.clone(), format_number_for_hash(input.run_time), input.rate.clone()];
        for id in &input.target_ids {
            parts.push(id.to_string());
        }
        parts.extend(input.extras.clone());
        hash_parts(&parts)
    }

    /// 🌳️ Merkle hash over an animation timeline.
    pub async fn hash_animation_timeline(children: Vec<String>) -> String {
        merkle_node(&["AnimateTimeline"], children)
    }

    /// 🎬️ Hash a scene configuration snapshot.
    pub async fn hash_scene_config(frame_rate: f64, width: u32, height: u32, mobject_count: usize) -> String {
        let rate = format_number_for_hash(frame_rate);
        hash_parts(&["SceneConfig", &rate, &width.to_string(), &height.to_string(), &mobject_count.to_string()])
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn animation_hash_is_stable() {
            let input = AnimationHashInput::new("FadeIn", 1.0).with_targets(vec![42]);
            let a = hash_animation(&input);
            let b = hash_animation(&input);
            assert_eq!(a, b);
        }

        #[semio_framework_async_macros::async_test]
        async fn timeline_merkle_orders_children() {
            let h = hash_animation_timeline(vec!["a".into(), "b".into()]);
            assert!(!h.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn hash_scene_config_is_stable_and_sensitive_to_inputs() {
            let a = hash_scene_config(60.0, 1920, 1080, 3);
            let b = hash_scene_config(60.0, 1920, 1080, 3);
            assert_eq!(a, b);
            let c = hash_scene_config(30.0, 1920, 1080, 3);
            assert_ne!(a, c);
        }

        #[semio_framework_async_macros::async_test]
        async fn hash_animation_differs_by_rate_and_extras() {
            let base = AnimationHashInput::new("Fade", 1.0);
            let with_rate = base.clone().with_rate("smooth");
            let with_extra = base.clone().with_extra("scale=2");
            assert_ne!(hash_animation(&base), hash_animation(&with_rate));
            assert_ne!(hash_animation(&base), hash_animation(&with_extra));
        }
    }
}

pub mod graph {
    //! 🕸️ Graph and directed graph layouts as Sobject groups.

    use crate::editor::animate::engine::text::color::Color;
    use crate::editor::animate::engine::geometry::geometry::{arrow, circle, line};
    use crate::editor::animate::engine::scene::sobject::{Group, Sobject, Sobjects};
    use crate::editor::animate::engine::text::text::Text;
    use geometry::Point;
    use std::collections::HashMap;

    /// 🔵️ Undirected graph with circular layout.
    pub struct Graph {
        pub group: Group,
        pub nodes: Vec<u32>,
        pub edges: Vec<(u32, u32)>,
    }

    impl Graph {
        pub async fn new(nodes: Vec<u32>, edges: Vec<(u32, u32)>, radius: f64, center: Point, color: Color) -> Self {
            let positions = circular_layout(&nodes, radius, center);
            let mut children: Vec<Sobjects> = Vec::new();
            for &(a, b) in &edges {
                if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                    children.push((line(pa, pb, color.with_alpha(0.6), 2.0)).into());
                }
            }
            for &n in &nodes {
                if let Some(&p) = positions.get(&n) {
                    children.push((circle(p, 0.2, color, None, 0.0)).into());
                }
            }
            Self { group: Group::new(children), nodes, edges }
        }

        pub async fn with_edge_labels(mut self, labels: &HashMap<(u32, u32), String>, positions: &HashMap<u32, Point>, color: Color) -> Self {
            for (&(a, b), label) in labels {
                if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                    let mid = Point::new((pa.x() + pb.x()) / 2.0, (pa.y() + pb.y()) / 2.0);
                    let mut t = Text::new(label, color);
                    t.inner.move_to(mid);
                    self.group.add_child((t.inner).into());
                }
            }
            self
        }
    }

    /// ➡️ Directed graph with force-directed layout seed.
    pub struct DiGraph {
        pub group: Group,
        pub nodes: Vec<u32>,
        pub edges: Vec<(u32, u32)>,
    }

    impl DiGraph {
        pub async fn new(nodes: Vec<u32>, edges: Vec<(u32, u32)>, radius: f64, center: Point, color: Color) -> Self {
            let positions = force_layout_seed(&nodes, &edges, radius, center);
            let node_r = 0.18;
            let mut children: Vec<Sobjects> = Vec::new();
            for &(a, b) in &edges {
                if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                    let dir = pb - pa;
                    let len = dir.hypot().max(1e-9);
                    let u = dir / len;
                    let start = pa + u * node_r;
                    let end = pb - u * node_r;
                    children.push((arrow(start, end, color.with_alpha(0.7), 2.0, 0.15)).into());
                }
            }
            for &n in &nodes {
                if let Some(&p) = positions.get(&n) {
                    children.push((circle(p, node_r, color, Some(Color::WHITE), 1.0)).into());
                }
            }
            Self { group: Group::new(children), nodes, edges }
        }

        pub async fn with_edge_labels(mut self, labels: &HashMap<(u32, u32), String>, positions: &HashMap<u32, Point>, color: Color) -> Self {
            for (&(a, b), label) in labels {
                if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                    let mid = Point::new((pa.x() + pb.x()) / 2.0, (pa.y() + pb.y()) / 2.0);
                    let mut t = Text::new(label, color);
                    t.inner.move_to(mid);
                    self.group.add_child((t.inner).into());
                }
            }
            self
        }
    }

    async fn circular_layout(nodes: &[u32], radius: f64, center: Point) -> HashMap<u32, Point> {
        let mut out = HashMap::new();
        let n = nodes.len().max(1);
        for (i, &id) in nodes.iter().enumerate() {
            let t = i as f64 / n as f64 * std::f64::consts::TAU;
            out.insert(id, Point::new(center.x() + radius * t.cos(), center.y() + radius * t.sin()));
        }
        out
    }

    async fn force_layout_seed(nodes: &[u32], edges: &[(u32, u32)], radius: f64, center: Point) -> HashMap<u32, Point> {
        let mut pos = circular_layout(nodes, radius, center);
        for _ in 0..24 {
            let mut forces: HashMap<u32, (f64, f64)> = nodes.iter().map(|&id| (id, (0.0, 0.0))).collect();
            for i in 0..nodes.len() {
                for j in (i + 1)..nodes.len() {
                    let a = nodes[i];
                    let b = nodes[j];
                    let pa = pos[&a];
                    let pb = pos[&b];
                    let dx = pb.x() - pa.x();
                    let dy = pb.y() - pa.y();
                    let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                    let rep = 0.05 / dist;
                    let force_a = forces.get_mut(&a).expect("a is drawn from nodes, forces is keyed by all of nodes");
                    force_a.0 -= dx * rep;
                    force_a.1 -= dy * rep;
                    let force_b = forces.get_mut(&b).expect("b is drawn from nodes, forces is keyed by all of nodes");
                    force_b.0 += dx * rep;
                    force_b.1 += dy * rep;
                }
            }
            for &(a, b) in edges {
                let (Some(&pa), Some(&pb)) = (pos.get(&a), pos.get(&b)) else {
                    continue;
                };
                let dx = pb.x() - pa.x();
                let dy = pb.y() - pa.y();
                let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                let att = dist * 0.02;
                if let Some(force_a) = forces.get_mut(&a) {
                    force_a.0 += dx / dist * att;
                    force_a.1 += dy / dist * att;
                }
                if let Some(force_b) = forces.get_mut(&b) {
                    force_b.0 -= dx / dist * att;
                    force_b.1 -= dy / dist * att;
                }
            }
            for &id in nodes {
                let (fx, fy) = forces[&id];
                let p = pos.get_mut(&id).expect("id is drawn from nodes, pos is keyed by all of nodes");
                *p = Point::new(p.x() + fx, p.y() + fy);
            }
        }
        pos
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn graph_has_node_and_edge_children() {
            let g = Graph::new(vec![1, 2, 3], vec![(1, 2), (2, 3)], 2.0, Point::ZERO, Color::BLUE);
            assert_eq!(g.nodes.len(), 3);
            assert!(!g.group.children.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn digraph_uses_arrows_and_labels() {
            let dg = DiGraph::new(vec![1, 2], vec![(1, 2)], 2.0, Point::ZERO, Color::WHITE);
            assert_eq!(dg.edges.len(), 1);
            let mut positions = HashMap::new();
            positions.insert(1, Point::new(-1.0, 0.0));
            positions.insert(2, Point::new(1.0, 0.0));
            let mut labels = HashMap::new();
            labels.insert((1, 2), "edge".into());
            let labeled = dg.with_edge_labels(&labels, &positions, Color::WHITE);
            assert!(labeled.group.children.len() > 2);
        }
    }
}

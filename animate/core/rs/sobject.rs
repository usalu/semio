use mathematical_geometry::{Affine, Arc, BezPath, Line, Point, Rect, RoundedRect};
use std::collections::BTreeMap;

/// 🏷️ Stable Sobject identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SobjectId(pub u64);

/// 🧭 Whether an Sobject participates in static-background caching.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mobility {
    Static,
    Moving,
}

/// 🎨 Solid fill paint.
#[derive(Clone, Debug, PartialEq)]
pub struct PaintStyle {
    pub color: [f32; 4],
}

/// ✏️ Stroke paint.
#[derive(Clone, Debug, PartialEq)]
pub struct StrokeStyle {
    pub color: [f32; 4],
    pub width: f64,
}

/// 📐 Supported 2D primitive shapes for the first video-engine slice.
#[derive(Clone, Debug, PartialEq)]
pub enum SobjectShape {
    Circle { center: Point, radius: f64 },
    Rect { rect: Rect },
    RoundedRect { rect: RoundedRect },
    Line { line: Line },
    Arc { arc: Arc },
    Path { path: BezPath },
}

/// 🔷 Scene-graph drawable with transform, style, and mobility.
#[derive(Clone, Debug, PartialEq)]
pub struct Sobject {
    pub id: SobjectId,
    pub shape: SobjectShape,
    pub transform: Affine,
    pub fill: Option<PaintStyle>,
    pub stroke: Option<StrokeStyle>,
    pub z_index: i32,
    pub mobility: Mobility,
}

/// 🗂️ Ordered Sobject collection (Manim mobjects).
#[derive(Clone, Debug, Default)]
pub struct MobjectStore {
    next_id: u64,
    objects: BTreeMap<SobjectId, Sobject>,
}

impl MobjectStore {
    /// ➕ Inserts a new Sobject and returns its id.
    pub fn add(&mut self, mut sobject: Sobject) -> SobjectId {
        let id = SobjectId(self.next_id);
        self.next_id += 1;
        sobject.id = id;
        self.objects.insert(id, sobject);
        id
    }

    /// ➖ Removes an Sobject by id.
    pub fn remove(&mut self, id: SobjectId) -> Option<Sobject> {
        self.objects.remove(&id)
    }

    /// 🔍 Lookup by id.
    pub fn get(&self, id: SobjectId) -> Option<&Sobject> {
        self.objects.get(&id)
    }

    /// 📋 All Sobjects sorted by z-index.
    pub fn sorted(&self) -> Vec<&Sobject> {
        let mut items: Vec<_> = self.objects.values().collect();
        items.sort_by_key(|s| s.z_index);
        items
    }

    /// 🪨 Static Sobjects for background caching.
    pub fn static_objects(&self) -> Vec<&Sobject> {
        self.sorted().into_iter().filter(|s| s.mobility == Mobility::Static).collect()
    }

    /// 🏃 Moving Sobjects rendered every frame.
    pub fn moving_objects(&self) -> Vec<&Sobject> {
        self.sorted().into_iter().filter(|s| s.mobility == Mobility::Moving).collect()
    }

    /// 🔢 Sobject count.
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// ∅ Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// 🧬 Clone snapshot for a frame.
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

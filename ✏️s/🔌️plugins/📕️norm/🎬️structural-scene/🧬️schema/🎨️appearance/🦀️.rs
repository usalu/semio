//! 🎨️ Appearance: material roles, colour channels, style states, layers and the utilisation binding; colour maps live outside the scene.

use super::*;

records! {
    /// 🖍️ 8-bit sRGB colour.
    pub struct Rgb {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    /// 🎛️ How an entity is shaded; the theme turns role and channel into concrete colours.
    pub struct Appearance {
        pub role: MaterialRole,
        pub channel: ColourChannel,
        pub style: StyleState,
        pub opacity: f64,
        pub hatch: Option<Hatch>,
        pub deformation: Option<DeformationOverlay>,
    }

    /// 🌡️ Per-entity result colouring input: the governing ratio, not a colour.
    pub struct UtilisationBinding {
        pub check_id: String,
        pub ratio: f64,
        pub status: CheckStatus,
        pub per_station: Option<Vec<StationValue>>,
        pub stale: bool,
    }

    /// 📍️ A scalar sampled at a relative position along a member.
    pub struct StationValue {
        pub at: f64,
        pub value: f64,
    }

    /// 🧭️ Displacement of one sampled point of the base entity.
    pub struct DisplacementSample {
        pub at: f64,
        pub delta: Vec3,
    }

    /// 👻️ Deformed or modal shape drawn over the undeformed ghost of the same entity.
    pub struct DeformationOverlay {
        pub kind: DeformationKind,
        pub index: u32,
        pub frequency: Option<Quantity>,
        pub participation: f64,
        pub scale: f64,
        pub samples: Vec<DisplacementSample>,
        pub surface: Vec<SurfaceSample>,
        pub motion: Motion,
        pub load_factor: Option<f64>,
    }

    /// 🌊️ Displacement of one point of a plate or shell for two-dimensional mode shapes; `at` is in metres of the surface itself: plane coordinates of a plate, and for a shell the arc length along the columns and along the rows from the first node.
    pub struct SurfaceSample {
        pub at: Vec2,
        pub delta: Vec3,
    }

    /// 🔢️ One named scalar field of a physical kind over the samples of its entity.
    pub struct ScalarField {
        pub id: String,
        pub unit: QuantityKind,
        pub samples: FieldSamples,
    }

    /// 🗂️ Toggleable entity group.
    pub struct Layer {
        pub id: String,
        pub name: LocalizedCopy,
        pub visible: bool,
        pub locked: bool,
    }

    /// 🗺️ Reference to a theme colour map and the value domain it is applied to.
    pub struct ColourMapRef {
        pub id: String,
        pub min: f64,
        pub max: f64,
    }

    /// ⏳️ A scrubbable parameter of a scene: fire time, creep age, mode index, oscillation phase or combination index; `keyframes` bind parameter values to scalar fields (blended between neighbours for `Time` and `Age`, snapped for `Mode` and `Combination`) and to layers that are shown only while their keyframe is current.
    pub struct ParameterTrack {
        pub id: String,
        pub kind: TrackKind,
        pub label: LocalizedCopy,
        pub unit: QuantityKind,
        pub min: f64,
        pub max: f64,
        pub step: f64,
        pub looped: bool,
        pub keyframes: Vec<Keyframe>,
    }

    /// 🔖️ One stop of a parameter track: the field shown and the layer revealed at parameter value `at`.
    pub struct Keyframe {
        pub at: f64,
        pub caption: LocalizedCopy,
        pub field: Option<String>,
        pub layer: Option<String>,
    }
}

variants! {
    /// 🧱️ Semantic material or object class used for default colouring.
    pub enum MaterialRole {
        Steel,
        Concrete,
        Rebar,
        Timber,
        Masonry,
        Soil { class: String },
        Aluminium,
        Glass,
        Weld,
        Bolt,
        Load { kind: LoadKind },
        Support,
        Neutral,
    }

    /// 🧮️ Where a scalar field is sampled: per mesh vertex (row-major for grids), per relative station along a member, or per pattern instance.
    pub enum FieldSamples {
        Vertex { values: Vec<f64> },
        Station { values: Vec<StationValue> },
        Instance { values: Vec<f64> },
    }

    /// 🌈️ What drives an entity's colour.
    pub enum ColourChannel {
        Role,
        Utilisation,
        Status,
        Fields { active: String, fields: Vec<ScalarField> },
        Custom { rgb: Rgb },
    }
}

choices! {
    /// 🔆️ Interaction or review state of an entity.
    pub enum StyleState {
        Normal,
        Hover,
        Selected,
        Dimmed,
        Ghost,
        Error,
        Stale,
    }

    /// 🏁️ Redundant non-colour cue pattern; `Horizontal` and `Vertical` are the two directional patterns: lines along and across the local `x` axis of the drawn area (grain of a layer along or across the span).
    pub enum Hatch {
        Diagonal,
        CrossDiagonal,
        Horizontal,
        Dots,
        Vertical,
    }

    /// 🚦️ Outcome class of a check, independent of any norm.
    pub enum CheckStatus {
        Pass,
        Warning,
        Fail,
        NotApplicable,
    }

    /// ⏱️ How a deformation overlay is played: frozen, oscillating as `sin(2 pi f t)` with its frequency, or ramped from rest to full scale.
    pub enum Motion {
        Static,
        Harmonic,
        Ramp,
    }

    /// 🧭️ What a parameter track scrubs: time (fire duration, char growth), age (creep, shrinkage), mode index (eigenmodes), phase (oscillation, 0 to 1 per cycle) or combination index (EN 1990).
    pub enum TrackKind {
        Time,
        Age,
        Mode,
        Phase,
        Combination,
    }

    /// 🎞️ Which displacement field an overlay carries.
    pub enum DeformationKind {
        Deformed,
        Mode,
    }
}

impl Appearance {
    pub fn role(role: MaterialRole) -> Self {
        Self { role, channel: ColourChannel::Role, style: StyleState::Normal, opacity: 1.0, hatch: None, deformation: None }
    }
}

impl Validate for Rgb {
    fn check(&self, _path: &str, _out: &mut Vec<Violation>) {}
}

impl Validate for MaterialRole {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        if let Self::Soil { class } = self {
            non_empty(out, &at(path, "class"), class);
        }
    }
}

impl Validate for ColourChannel {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        if let Self::Fields { active, fields } = self {
            non_empty(out, &at(path, "active"), active);
            each(fields, path, "fields", out);
            expect(out, &at(path, "active"), fields.iter().any(|f| &f.id == active), "active-field-exists");
            expect(out, &at(path, "fields"), fields.iter().map(|f| &f.id).collect::<std::collections::BTreeSet<_>>().len() == fields.len(), "unique-field-ids");
        }
    }
}

impl Validate for FieldSamples {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Vertex { values } | Self::Instance { values } => {
                expect(out, &at(path, "values"), !values.is_empty() && values.iter().all(|v| v.is_finite()), "finite-non-empty");
            }
            Self::Station { values } => {
                expect(out, &at(path, "values"), !values.is_empty() && values.windows(2).all(|w| w[0].at < w[1].at), "strictly-ascending");
                each(values, path, "values", out);
            }
        }
    }
}

impl Validate for ScalarField {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.samples.check(&at(path, "samples"), out);
    }
}

impl Validate for SurfaceSample {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.at.check(&at(path, "at"), out);
        self.delta.check(&at(path, "delta"), out);
    }
}

impl Validate for Appearance {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.role.check(&at(path, "role"), out);
        self.channel.check(&at(path, "channel"), out);
        expect(out, &at(path, "opacity"), self.opacity.is_finite() && (0.0..=1.0).contains(&self.opacity), "unit-interval");
        self.deformation.check(&at(path, "deformation"), out);
    }
}

impl Validate for StationValue {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, &at(path, "at"), self.at.is_finite() && (0.0..=1.0).contains(&self.at), "unit-interval");
        finite(out, &at(path, "value"), &[self.value]);
    }
}

impl Validate for UtilisationBinding {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "checkId"), &self.check_id);
        non_negative(out, &at(path, "ratio"), self.ratio);
        if let Some(stations) = &self.per_station {
            each(stations, path, "perStation", out);
            expect(out, &at(path, "perStation"), stations.windows(2).all(|w| w[0].at < w[1].at), "strictly-ascending");
        }
    }
}

impl Validate for DisplacementSample {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, &at(path, "at"), self.at.is_finite() && self.at >= 0.0, "non-negative");
        self.delta.check(&at(path, "delta"), out);
    }
}

impl Validate for DeformationOverlay {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, &at(path, "scale"), &[self.scale, self.participation]);
        self.frequency.check(&at(path, "frequency"), out);
        expect(out, &at(path, "samples"), !self.samples.is_empty() || !self.surface.is_empty(), "non-empty");
        each(&self.samples, path, "samples", out);
        each(&self.surface, path, "surface", out);
        if let Some(alpha) = self.load_factor {
            positive(out, &at(path, "loadFactor"), alpha);
        }
    }
}

impl Validate for Layer {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.name.check(&at(path, "name"), out);
    }
}

impl Validate for ColourMapRef {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        finite(out, path, &[self.min, self.max]);
        expect(out, path, self.min < self.max, "ordered-domain");
    }
}

impl Validate for Keyframe {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, &at(path, "at"), &[self.at]);
        self.caption.check(&at(path, "caption"), out);
        expect(out, path, self.field.is_some() || self.layer.is_some(), "field-or-layer");
    }
}

impl Validate for ParameterTrack {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.label.check(&at(path, "label"), out);
        finite(out, path, &[self.min, self.max, self.step]);
        expect(out, &at(path, "range"), self.min < self.max, "ordered-domain");
        positive(out, &at(path, "step"), self.step);
        each(&self.keyframes, path, "keyframes", out);
        expect(out, &at(path, "keyframes"), self.keyframes.windows(2).all(|w| w[0].at < w[1].at) && self.keyframes.iter().all(|k| (self.min..=self.max).contains(&k.at)), "ascending-within-range");
        expect(out, &at(path, "keyframes"), matches!(self.kind, TrackKind::Phase) || !self.keyframes.is_empty(), "non-empty");
    }
}

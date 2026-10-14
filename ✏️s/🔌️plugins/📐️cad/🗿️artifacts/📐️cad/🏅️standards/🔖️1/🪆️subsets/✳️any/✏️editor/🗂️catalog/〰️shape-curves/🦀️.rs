//! 〰️ The `spatial.shape` curve interactions.


/// 📚️ `model definitions` assets of this catalog, embedded at build time.
pub const MODEL_DEFINITIONS: &[&str] = &[
];

/// 📚️ `typologies` assets of this catalog, embedded at build time.
pub const TYPOLOGIES: &[(&str, &str)] = &[
];

/// 📚️ `attribute definitions` assets of this catalog, embedded at build time.
pub const ATTRIBUTE_DEFINITIONS: &[(&str, &str)] = &[
];

/// 📚️ `interactions` assets of this catalog, embedded at build time.
pub const INTERACTIONS: &[(&str, &str)] = &[
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🌙️arc.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/⭕️circle.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/〰️constructCurve.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🎛️controlPointCurve.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🌊️interpolateCurve.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🖊️line.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/📈️polyline.json")),
];

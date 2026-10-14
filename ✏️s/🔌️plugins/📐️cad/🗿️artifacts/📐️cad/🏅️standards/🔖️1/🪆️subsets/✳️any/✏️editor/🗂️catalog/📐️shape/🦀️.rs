//! 📐️ The `spatial.shape` model definition and its typologies.


/// 📚️ `model definitions` assets of this catalog, embedded at build time.
pub const MODEL_DEFINITIONS: &[&str] = &[
    include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🔣️modelDefinition.json"),
];

/// 📚️ `typologies` assets of this catalog, embedded at build time.
pub const TYPOLOGIES: &[(&str, &str)] = &[
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/➡️ExtrudeCurve/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/⭕️Circle/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/〰️ControlPointCurve/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌀️Sweep1/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌊️InterpolateCurve/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌐️Sphere/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌙️Arc/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🌪️Sweep2/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🎢️Loft/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/📏️Line/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/📐️Polyline/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/📦️Box/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🕸️NetworkSurface/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🗺️Plane/🔣️typology.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🗂️typologies/🥫️Cylinder/🔣️typology.json")),
];

/// 📚️ `attribute definitions` assets of this catalog, embedded at build time.
pub const ATTRIBUTE_DEFINITIONS: &[(&str, &str)] = &[
];

/// 📚️ `interactions` assets of this catalog, embedded at build time.
pub const INTERACTIONS: &[(&str, &str)] = &[
];

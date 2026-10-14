//! ✂️ The `spatial.shape` edit, transform, measure and anchor interactions.


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
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🟨️area.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🪓️chamfer.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/📋️copy.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/⚓️createAnchor.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/💥️explode.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🍥️fillet.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🔗️join.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/📏️length.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🪞️mirror.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🚚️move.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🔃️rotate.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/↕️scale1d.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/🪗️scale3d.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/✂️split.json")),
    ("spatial.shape", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/✏️trim.json")),
];

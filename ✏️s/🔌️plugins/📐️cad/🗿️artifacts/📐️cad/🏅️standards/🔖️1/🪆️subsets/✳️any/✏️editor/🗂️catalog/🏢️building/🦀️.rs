//! 🏢️ The `aec.building` model definition: its typologies, attribute definitions and interactions.


/// 📚️ `model definitions` assets of this catalog, embedded at build time.
pub const MODEL_DEFINITIONS: &[&str] = &[
    include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🔣️modelDefinition.json"),
];

/// 📚️ `typologies` assets of this catalog, embedded at build time.
pub const TYPOLOGIES: &[(&str, &str)] = &[
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/⬆️Ceiling/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🏛️Column/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🏠️Roof/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🚧️Railing/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🚪️Door/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🛡️Wall/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🧱️Slab/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🪜️Stair/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🪟️Window/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🪨️Foundation/🔣️typology.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🗂️typologies/🪵️Beam/🔣️typology.json")),
];

/// 📚️ `attribute definitions` assets of this catalog, embedded at build time.
pub const ATTRIBUTE_DEFINITIONS: &[(&str, &str)] = &[
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🏷️attributeDefinitions/🚪️opening.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🏷️attributeDefinitions/🪨️material.json")),
];

/// 📚️ `interactions` assets of this catalog, embedded at build time.
pub const INTERACTIONS: &[(&str, &str)] = &[
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🪵️placeBeam.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/⬆️placeCeiling.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🏛️placeColumn.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🚪️placeDoor.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🪨️placeFoundation.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🚧️placeRailing.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🏠️placeRoof.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🧱️placeSlab.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🪜️placeStair.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🛡️placeWall.json")),
    ("aec.building", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/🕹️interactions/🪟️placeWindow.json")),
];

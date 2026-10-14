//! 🔥️ The `aec.building.energy` model definition: its typologies, attribute definitions and interactions.


/// 📚️ `model definitions` assets of this catalog, embedded at build time.
pub const MODEL_DEFINITIONS: &[&str] = &[
    include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🔣️modelDefinition.json"),
];

/// 📚️ `typologies` assets of this catalog, embedded at build time.
pub const TYPOLOGIES: &[(&str, &str)] = &[
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🏠️Roof/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🐚️Hull/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🧱️ExternalWall/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🪟️Windows/🔣️typology.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🗂️typologies/🪨️BasePlate/🔣️typology.json")),
];

/// 📚️ `attribute definitions` assets of this catalog, embedded at build time.
pub const ATTRIBUTE_DEFINITIONS: &[(&str, &str)] = &[
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🏷️attributeDefinitions/🌡️uvalue.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🏷️attributeDefinitions/🔆️gvalue.json")),
];

/// 📚️ `interactions` assets of this catalog, embedded at build time.
pub const INTERACTIONS: &[(&str, &str)] = &[
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🧱️constructBasePlate.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🚧️constructExternalWall.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🚢️constructHull.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🏠️constructRoof.json")),
    ("aec.building.energy", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🕹️interactions/🪟️constructWindows.json")),
];

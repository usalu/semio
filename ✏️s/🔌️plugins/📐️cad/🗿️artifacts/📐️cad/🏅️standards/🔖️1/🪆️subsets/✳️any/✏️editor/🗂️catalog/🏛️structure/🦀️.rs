//! 🏛️ Structural model definitions: the classic reinforced-concrete structure, its attribute definitions and the concrete definition.


/// 📚️ `model definitions` assets of this catalog, embedded at build time.
pub const MODEL_DEFINITIONS: &[&str] = &[
    include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🔣️modelDefinition.json"),
    include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🔣️modelDefinition.json"),
    include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🧱️aec.building.concrete/🔣️modelDefinition.json"),
];

/// 📚️ `typologies` assets of this catalog, embedded at build time.
pub const TYPOLOGIES: &[(&str, &str)] = &[
    ("aec.building.structure.classic", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🗂️typologies/🏛️ReinforcedConcreteColumn/🔣️typology.json")),
    ("aec.building.structure.classic", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🗂️typologies/🚧️ReinforcedConcreteInternal/🔣️typology.json")),
    ("aec.building.structure.classic", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🗂️typologies/🛡️ReinforcedConcreteExternal/🔣️typology.json")),
    ("aec.building.structure.classic", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🗂️typologies/🧱️OneWayReinforcedConcreteSlab/🔣️typology.json")),
];

/// 📚️ `attribute definitions` assets of this catalog, embedded at build time.
pub const ATTRIBUTE_DEFINITIONS: &[(&str, &str)] = &[
    ("aec.building.structure", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🌡️uvalue.json")),
    ("aec.building.structure", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🌦️exposure.json")),
    ("aec.building.structure", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🔆️gvalue.json")),
    ("aec.building.structure", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🔗️bondable.json")),
    ("aec.building.structure", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🚪️opening.json")),
    ("aec.building.structure", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure/🏷️attributeDefinitions/🪨️material.json")),
];

/// 📚️ `interactions` assets of this catalog, embedded at build time.
pub const INTERACTIONS: &[(&str, &str)] = &[
    ("aec.building.structure.classic", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🕹️interactions/🧱️constructOneWayRe-72a083.json")),
    ("aec.building.structure.classic", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🕹️interactions/🏛️constructReinforc-411bd6.json")),
    ("aec.building.structure.classic", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🕹️interactions/🛡️constructReinforc-c38891.json")),
    ("aec.building.structure.classic", include_str!("../../../📚️examples/🖼️assets/🏗️modelDefinitions/🌉️aec.building/🕹️interactions/🚧️constructReinforc-e8fc67.json")),
];

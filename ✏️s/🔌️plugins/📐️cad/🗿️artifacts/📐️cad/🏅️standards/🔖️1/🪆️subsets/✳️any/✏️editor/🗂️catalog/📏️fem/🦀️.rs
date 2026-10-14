//! 📏️ The structural FEM model definitions: line, surface and solid elements.


/// 📚️ `model definitions` assets of this catalog, embedded at build time.
pub const MODEL_DEFINITIONS: &[&str] = &[
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📏️aec.building/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🗺️aec.building/🔣️modelDefinition.json"),
    include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🧊️aec.building/🔣️modelDefinition.json"),
];

/// 📚️ `typologies` assets of this catalog, embedded at build time.
pub const TYPOLOGIES: &[(&str, &str)] = &[
    ("aec.building.structure.fem.line", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📏️aec.building/🗂️typologies/📏️LineElement/🔣️typology.json")),
    ("aec.building.structure.fem.surface", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🗺️aec.building/🗂️typologies/🗺️SurfaceElement/🔣️typology.json")),
    ("aec.building.structure.fem.solid", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🧊️aec.building/🗂️typologies/🧊️SolidElement/🔣️typology.json")),
];

/// 📚️ `attribute definitions` assets of this catalog, embedded at build time.
pub const ATTRIBUTE_DEFINITIONS: &[(&str, &str)] = &[
];

/// 📚️ `interactions` assets of this catalog, embedded at build time.
pub const INTERACTIONS: &[(&str, &str)] = &[
    ("aec.building.structure.fem.line", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/📏️aec.building/🕹️interactions/🔣️constructLineElem-0d404b.json")),
    ("aec.building.structure.fem.solid", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🧊️aec.building/🕹️interactions/🔣️constructSolidEle-105046.json")),
    ("aec.building.structure.fem.surface", include_str!("../../../../../../../../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🗺️aec.building/🕹️interactions/🔣️constructSurfaceE-7da86f.json")),
];

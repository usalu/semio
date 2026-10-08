CREATE TABLE cad_document (
 id INTEGER PRIMARY KEY CHECK(id = 1),
 schema TEXT NOT NULL,
 cad_id TEXT NOT NULL
);
CREATE TABLE cad_model_child (
 id INTEGER PRIMARY KEY CHECK(id > 0),
 document_id INTEGER NOT NULL REFERENCES cad_document(id),
 slot TEXT NOT NULL CHECK(slot IN ('shapeModel','buildingModel','energyModel','structureClassicModel')),
 child_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 artifact_kind TEXT NOT NULL,
 standard TEXT NOT NULL,
 subset TEXT NOT NULL
);
CREATE TABLE cad_drawing_child (
 id INTEGER PRIMARY KEY CHECK(id > 0),
 document_id INTEGER NOT NULL REFERENCES cad_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 child_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 artifact_kind TEXT NOT NULL,
 standard TEXT NOT NULL,
 subset TEXT NOT NULL
);
CREATE TABLE cad_brep_child (
 id INTEGER PRIMARY KEY CHECK(id > 0),
 document_id INTEGER NOT NULL REFERENCES cad_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 child_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 artifact_kind TEXT NOT NULL,
 standard TEXT NOT NULL,
 subset TEXT NOT NULL
);
CREATE TABLE cad_reference_group (
 id INTEGER PRIMARY KEY CHECK(id > 0),
 document_id INTEGER NOT NULL REFERENCES cad_document(id),
 model_definition_id TEXT NOT NULL
);
CREATE TABLE cad_reference (
 id INTEGER PRIMARY KEY CHECK(id > 0),
 group_id INTEGER NOT NULL REFERENCES cad_reference_group(id),
 ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 reference_id TEXT NOT NULL,
 source_url TEXT NOT NULL,
 media_kind TEXT NOT NULL,
 origin_x REAL,
 origin_y REAL,
 origin_z REAL,
 orientation_present INTEGER NOT NULL CHECK(orientation_present IN (0,1)),
 orientation_x REAL,
 orientation_y REAL,
 orientation_z REAL,
 orientation_w REAL,
 scale REAL,
 width_world REAL,
 hidden INTEGER NOT NULL CHECK(hidden IN (0,1)),
 locked INTEGER NOT NULL CHECK(locked IN (0,1)),
 opacity REAL,
 origin_x_bits INTEGER NOT NULL,
 origin_x_class TEXT NOT NULL CHECK(origin_x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 origin_y_bits INTEGER NOT NULL,
 origin_y_class TEXT NOT NULL CHECK(origin_y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 origin_z_bits INTEGER NOT NULL,
 origin_z_class TEXT NOT NULL CHECK(origin_z_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 orientation_x_bits INTEGER,
 orientation_x_class TEXT CHECK(orientation_x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 orientation_y_bits INTEGER,
 orientation_y_class TEXT CHECK(orientation_y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 orientation_z_bits INTEGER,
 orientation_z_class TEXT CHECK(orientation_z_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 orientation_w_bits INTEGER,
 orientation_w_class TEXT CHECK(orientation_w_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 scale_bits INTEGER,
 scale_class TEXT CHECK(scale_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 width_world_bits INTEGER NOT NULL,
 width_world_class TEXT NOT NULL CHECK(width_world_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 opacity_bits INTEGER,
 opacity_class TEXT CHECK(opacity_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 CHECK((orientation_present = 0 AND orientation_x IS NULL AND orientation_y IS NULL AND orientation_z IS NULL AND orientation_w IS NULL AND orientation_x_bits IS NULL AND orientation_x_class IS NULL AND orientation_y_bits IS NULL AND orientation_y_class IS NULL AND orientation_z_bits IS NULL AND orientation_z_class IS NULL AND orientation_w_bits IS NULL AND orientation_w_class IS NULL) OR (orientation_present = 1 AND orientation_x_bits IS NOT NULL AND orientation_x_class IS NOT NULL AND orientation_y_bits IS NOT NULL AND orientation_y_class IS NOT NULL AND orientation_z_bits IS NOT NULL AND orientation_z_class IS NOT NULL AND orientation_w_bits IS NOT NULL AND orientation_w_class IS NOT NULL)),
 CHECK((scale IS NULL AND scale_bits IS NULL AND scale_class IS NULL) OR (scale_bits IS NOT NULL AND scale_class IS NOT NULL)),
 CHECK((opacity IS NULL AND opacity_bits IS NULL AND opacity_class IS NULL) OR (opacity_bits IS NOT NULL AND opacity_class IS NOT NULL))
);
CREATE TABLE cad_node (
 id INTEGER PRIMARY KEY CHECK(id > 0),
 document_id INTEGER NOT NULL REFERENCES cad_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 node_id TEXT NOT NULL,
 label TEXT NOT NULL,
 kind TEXT NOT NULL
);

CREATE TABLE wfc2d_document (id INTEGER PRIMARY KEY CHECK(id=1), schema_id TEXT NOT NULL, seed TEXT NOT NULL);
CREATE TABLE wfc2d_tile (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, label TEXT, weight REAL,
 media_kind TEXT NOT NULL CHECK(media_kind IN ('empty','bitmap','vector','image')),
 weight_ieee754_bits INTEGER NOT NULL, weight_numeric_class TEXT NOT NULL CHECK(weight_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE wfc2d_bitmap (
 id INTEGER PRIMARY KEY REFERENCES wfc2d_tile(id), width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK(height BETWEEN 0 AND 4294967295), palette_indices_base64 TEXT NOT NULL
);
CREATE TABLE wfc2d_palette (
 id INTEGER PRIMARY KEY, bitmap_id INTEGER NOT NULL REFERENCES wfc2d_bitmap(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 4294967295), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 4294967295), blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 4294967295), alpha INTEGER NOT NULL CHECK(alpha BETWEEN 0 AND 4294967295)
);
CREATE TABLE wfc2d_vector (id INTEGER PRIMARY KEY REFERENCES wfc2d_tile(id));
CREATE TABLE wfc2d_path (
 id INTEGER PRIMARY KEY, vector_id INTEGER NOT NULL REFERENCES wfc2d_vector(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), stroke_width REAL,
 stroke_width_ieee754_bits INTEGER NOT NULL, stroke_width_numeric_class TEXT NOT NULL CHECK(stroke_width_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE wfc2d_path_color (
 id INTEGER PRIMARY KEY, path_id INTEGER NOT NULL REFERENCES wfc2d_path(id), role TEXT NOT NULL CHECK(role IN ('fill','stroke')),
 red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 4294967295), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 4294967295), blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 4294967295), alpha INTEGER NOT NULL CHECK(alpha BETWEEN 0 AND 4294967295)
);
CREATE TABLE wfc2d_segment (
 id INTEGER PRIMARY KEY, path_id INTEGER NOT NULL REFERENCES wfc2d_path(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 kind TEXT NOT NULL CHECK(kind IN ('Move','Line','Quad','Cubic','Close')),
 to_x REAL, to_y REAL, control1_x REAL, control1_y REAL, control2_x REAL, control2_y REAL,
 to_x_ieee754_bits INTEGER, to_x_numeric_class TEXT, to_y_ieee754_bits INTEGER, to_y_numeric_class TEXT,
 control1_x_ieee754_bits INTEGER, control1_x_numeric_class TEXT, control1_y_ieee754_bits INTEGER, control1_y_numeric_class TEXT,
 control2_x_ieee754_bits INTEGER, control2_x_numeric_class TEXT, control2_y_ieee754_bits INTEGER, control2_y_numeric_class TEXT
);
CREATE TABLE wfc2d_image (
 id INTEGER PRIMARY KEY REFERENCES wfc2d_tile(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL
);
CREATE TABLE wfc2d_slot (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL,
 x REAL, y REAL, width REAL, height REAL, pinned_tile_id INTEGER REFERENCES wfc2d_tile(id),
 x_ieee754_bits INTEGER NOT NULL, x_numeric_class TEXT NOT NULL, y_ieee754_bits INTEGER NOT NULL, y_numeric_class TEXT NOT NULL,
 width_ieee754_bits INTEGER NOT NULL, width_numeric_class TEXT NOT NULL, height_ieee754_bits INTEGER NOT NULL, height_numeric_class TEXT NOT NULL
);
CREATE TABLE wfc2d_slot_edge (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL,
 from_slot_id INTEGER NOT NULL REFERENCES wfc2d_slot(id), to_slot_id INTEGER NOT NULL REFERENCES wfc2d_slot(id), relation TEXT NOT NULL
);
CREATE TABLE wfc2d_rule (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL,
 tile_a_id INTEGER NOT NULL REFERENCES wfc2d_tile(id), tile_b_id INTEGER NOT NULL REFERENCES wfc2d_tile(id), relation TEXT, allowed INTEGER NOT NULL CHECK(allowed IN (0,1))
);

CREATE TABLE wfc_grid2d_document (
 id INTEGER PRIMARY KEY CHECK(id=1), schema_id TEXT NOT NULL, seed TEXT NOT NULL,
 width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK(height BETWEEN 0 AND 4294967295),
 cell_width REAL, cell_height REAL, periodic_x INTEGER NOT NULL CHECK(periodic_x IN (0,1)), periodic_y INTEGER NOT NULL CHECK(periodic_y IN (0,1)),
 cell_width_ieee754_bits INTEGER NOT NULL, cell_width_numeric_class TEXT NOT NULL CHECK(cell_width_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 cell_height_ieee754_bits INTEGER NOT NULL, cell_height_numeric_class TEXT NOT NULL CHECK(cell_height_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE wfc_grid2d_tile (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc_grid2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, label TEXT, weight REAL,
 media_kind TEXT NOT NULL CHECK(media_kind IN ('bitmap','vector','image')),
 weight_ieee754_bits INTEGER NOT NULL, weight_numeric_class TEXT NOT NULL CHECK(weight_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE wfc_grid2d_bitmap (
 id INTEGER PRIMARY KEY REFERENCES wfc_grid2d_tile(id), width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK(height BETWEEN 0 AND 4294967295), palette_indices_base64 TEXT NOT NULL
);
CREATE TABLE wfc_grid2d_palette (
 id INTEGER PRIMARY KEY, bitmap_id INTEGER NOT NULL REFERENCES wfc_grid2d_bitmap(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 4294967295), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 4294967295), blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 4294967295), alpha INTEGER NOT NULL CHECK(alpha BETWEEN 0 AND 4294967295)
);
CREATE TABLE wfc_grid2d_vector (id INTEGER PRIMARY KEY REFERENCES wfc_grid2d_tile(id));
CREATE TABLE wfc_grid2d_path (
 id INTEGER PRIMARY KEY, vector_id INTEGER NOT NULL REFERENCES wfc_grid2d_vector(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), stroke_width REAL,
 stroke_width_ieee754_bits INTEGER NOT NULL, stroke_width_numeric_class TEXT NOT NULL CHECK(stroke_width_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE wfc_grid2d_path_color (
 id INTEGER PRIMARY KEY, path_id INTEGER NOT NULL REFERENCES wfc_grid2d_path(id), role TEXT NOT NULL CHECK(role IN ('fill','stroke')),
 red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 4294967295), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 4294967295), blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 4294967295), alpha INTEGER NOT NULL CHECK(alpha BETWEEN 0 AND 4294967295)
);
CREATE TABLE wfc_grid2d_segment (
 id INTEGER PRIMARY KEY, path_id INTEGER NOT NULL REFERENCES wfc_grid2d_path(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 kind TEXT NOT NULL CHECK(kind IN ('moveTo','lineTo','quadTo','cubicTo','close')),
 to_x REAL, to_y REAL, control1_x REAL, control1_y REAL, control2_x REAL, control2_y REAL,
 to_x_ieee754_bits INTEGER, to_x_numeric_class TEXT, to_y_ieee754_bits INTEGER, to_y_numeric_class TEXT,
 control1_x_ieee754_bits INTEGER, control1_x_numeric_class TEXT, control1_y_ieee754_bits INTEGER, control1_y_numeric_class TEXT,
 control2_x_ieee754_bits INTEGER, control2_x_numeric_class TEXT, control2_y_ieee754_bits INTEGER, control2_y_numeric_class TEXT
);
CREATE TABLE wfc_grid2d_image (
 id INTEGER PRIMARY KEY REFERENCES wfc_grid2d_tile(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL
);
CREATE TABLE wfc_grid2d_rule (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc_grid2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL,
 tile_a_id INTEGER NOT NULL REFERENCES wfc_grid2d_tile(id), tile_b_id INTEGER NOT NULL REFERENCES wfc_grid2d_tile(id), direction TEXT NOT NULL CHECK(direction IN ('LEFT','RIGHT','TOP','BOTTOM')), allowed INTEGER NOT NULL CHECK(allowed IN (0,1))
);
CREATE TABLE wfc_grid2d_pin (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc_grid2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 x INTEGER NOT NULL CHECK(x BETWEEN 0 AND 4294967295), y INTEGER NOT NULL CHECK(y BETWEEN 0 AND 4294967295), tile_id INTEGER NOT NULL REFERENCES wfc_grid2d_tile(id)
);
CREATE TABLE wfc_grid2d_mask (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc_grid2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 x INTEGER NOT NULL CHECK(x BETWEEN 0 AND 4294967295), y INTEGER NOT NULL CHECK(y BETWEEN 0 AND 4294967295)
);

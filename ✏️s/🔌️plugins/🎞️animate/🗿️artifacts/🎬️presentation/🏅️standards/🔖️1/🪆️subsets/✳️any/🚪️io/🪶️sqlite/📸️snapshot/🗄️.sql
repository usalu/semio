CREATE TABLE presentation_document (id INTEGER PRIMARY KEY CHECK(id=1),schema TEXT NOT NULL);
CREATE TABLE presentation_source (
 id INTEGER PRIMARY KEY CHECK(id=1),document_id INTEGER NOT NULL REFERENCES presentation_document(id) CHECK(document_id=1),
 src TEXT NOT NULL,kind TEXT NOT NULL,x REAL,y REAL,width REAL,height REAL,source_aspect REAL,pdf_page INTEGER CHECK(pdf_page BETWEEN 0 AND 4294967295),
 x_bits INTEGER NOT NULL,x_class TEXT NOT NULL CHECK(x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 y_bits INTEGER NOT NULL,y_class TEXT NOT NULL CHECK(y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 width_bits INTEGER NOT NULL,width_class TEXT NOT NULL CHECK(width_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 height_bits INTEGER NOT NULL,height_class TEXT NOT NULL CHECK(height_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 source_aspect_bits INTEGER,source_aspect_class TEXT CHECK(source_aspect_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 CHECK((source_aspect IS NULL AND source_aspect_bits IS NULL AND source_aspect_class IS NULL) OR (source_aspect_bits IS NOT NULL AND source_aspect_class IS NOT NULL))
);
CREATE TABLE presentation_tile (
 id INTEGER PRIMARY KEY CHECK(id>0),document_id INTEGER NOT NULL REFERENCES presentation_document(id) CHECK(document_id=1),ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 tile_id TEXT NOT NULL,name TEXT NOT NULL,x REAL,y REAL,width REAL,height REAL,
 x_bits INTEGER NOT NULL,x_class TEXT NOT NULL CHECK(x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 y_bits INTEGER NOT NULL,y_class TEXT NOT NULL CHECK(y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 width_bits INTEGER NOT NULL,width_class TEXT NOT NULL CHECK(width_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 height_bits INTEGER NOT NULL,height_class TEXT NOT NULL CHECK(height_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE presentation_child (
 id INTEGER PRIMARY KEY CHECK(id>0),document_id INTEGER NOT NULL REFERENCES presentation_document(id) CHECK(document_id=1),slot TEXT NOT NULL CHECK(slot IN ('presentation','animation')),
 child_id TEXT NOT NULL,artifact_id TEXT NOT NULL,artifact_kind TEXT NOT NULL,standard TEXT NOT NULL,subset TEXT NOT NULL
);


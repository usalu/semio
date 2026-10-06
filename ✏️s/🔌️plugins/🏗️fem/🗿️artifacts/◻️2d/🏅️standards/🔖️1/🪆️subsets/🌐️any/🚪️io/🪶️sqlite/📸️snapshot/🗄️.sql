CREATE TABLE fem2d_document (
 id INTEGER PRIMARY KEY CHECK(id=1), modal_count TEXT NOT NULL, buckling_count TEXT NOT NULL, deformation_scale REAL,
 deformation_scale_ieee754_bits INTEGER NOT NULL, deformation_scale_numeric_class TEXT NOT NULL CHECK(deformation_scale_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_node (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES fem2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, x REAL, y REAL,
 x_ieee754_bits INTEGER NOT NULL, x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 y_ieee754_bits INTEGER NOT NULL, y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_element (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES fem2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 kind TEXT NOT NULL CHECK(kind IN ('bar','beam')), authored_id TEXT NOT NULL, start_node_id TEXT NOT NULL, end_node_id TEXT NOT NULL, material_id TEXT NOT NULL, section_id TEXT NOT NULL
);
CREATE TABLE fem2d_region (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES fem2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, name TEXT NOT NULL,
 thickness REAL, material_id TEXT NOT NULL, mesh_size REAL,
 thickness_ieee754_bits INTEGER NOT NULL, thickness_numeric_class TEXT NOT NULL CHECK(thickness_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 mesh_size_ieee754_bits INTEGER NOT NULL, mesh_size_numeric_class TEXT NOT NULL CHECK(mesh_size_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_outline_vertex (
 id INTEGER PRIMARY KEY, region_id INTEGER NOT NULL REFERENCES fem2d_region(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), x REAL, y REAL,
 x_ieee754_bits INTEGER NOT NULL, x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 y_ieee754_bits INTEGER NOT NULL, y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_hole (id INTEGER PRIMARY KEY, region_id INTEGER NOT NULL REFERENCES fem2d_region(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0));
CREATE TABLE fem2d_hole_vertex (
 id INTEGER PRIMARY KEY, hole_id INTEGER NOT NULL REFERENCES fem2d_hole(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), x REAL, y REAL,
 x_ieee754_bits INTEGER NOT NULL, x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 y_ieee754_bits INTEGER NOT NULL, y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_material (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES fem2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, name TEXT NOT NULL, e REAL, nu REAL, rho REAL,
 e_ieee754_bits INTEGER NOT NULL, e_numeric_class TEXT NOT NULL CHECK(e_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 nu_ieee754_bits INTEGER NOT NULL, nu_numeric_class TEXT NOT NULL CHECK(nu_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 rho_ieee754_bits INTEGER NOT NULL, rho_numeric_class TEXT NOT NULL CHECK(rho_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_section (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES fem2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, name TEXT NOT NULL, area REAL, iy REAL,
 area_ieee754_bits INTEGER NOT NULL, area_numeric_class TEXT NOT NULL CHECK(area_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 iy_ieee754_bits INTEGER NOT NULL, iy_numeric_class TEXT NOT NULL CHECK(iy_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_support (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES fem2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, node_id TEXT NOT NULL
);
CREATE TABLE fem2d_support_dof (
 id INTEGER PRIMARY KEY, support_id INTEGER NOT NULL REFERENCES fem2d_support(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), dof TEXT NOT NULL CHECK(dof IN ('Tx','Ty','Tz','Rx','Ry','Rz'))
);
CREATE TABLE fem2d_load_case (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES fem2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, name TEXT NOT NULL, self_weight INTEGER NOT NULL CHECK(self_weight IN (0,1))
);
CREATE TABLE fem2d_load (
 id INTEGER PRIMARY KEY, load_case_id INTEGER NOT NULL REFERENCES fem2d_load_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('nodal','memberUdl','area'))
);
CREATE TABLE fem2d_nodal_load (
 id INTEGER PRIMARY KEY REFERENCES fem2d_load(id), node_id TEXT NOT NULL, dof TEXT NOT NULL CHECK(dof IN ('Tx','Ty','Tz','Rx','Ry','Rz')), value REAL,
 value_ieee754_bits INTEGER NOT NULL, value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_member_udl (
 id INTEGER PRIMARY KEY REFERENCES fem2d_load(id), element_id TEXT NOT NULL, wx REAL, wy REAL,
 wx_ieee754_bits INTEGER NOT NULL, wx_numeric_class TEXT NOT NULL CHECK(wx_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 wy_ieee754_bits INTEGER NOT NULL, wy_numeric_class TEXT NOT NULL CHECK(wy_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_area_load (
 id INTEGER PRIMARY KEY REFERENCES fem2d_load(id), region_id TEXT NOT NULL, pressure REAL,
 pressure_ieee754_bits INTEGER NOT NULL, pressure_numeric_class TEXT NOT NULL CHECK(pressure_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE fem2d_combination (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES fem2d_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), authored_id TEXT NOT NULL, name TEXT NOT NULL
);
CREATE TABLE fem2d_combination_term (
 id INTEGER PRIMARY KEY, combination_id INTEGER NOT NULL REFERENCES fem2d_combination(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), case_id TEXT NOT NULL, factor REAL,
 factor_ieee754_bits INTEGER NOT NULL, factor_numeric_class TEXT NOT NULL CHECK(factor_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);

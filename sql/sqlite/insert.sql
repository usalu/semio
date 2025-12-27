-- Insert Statements for semio Kit Database
-- All statements use parameterized queries (?) for security and reusability

-- semio metadata
-- Parameters: release, engine, created
INSERT INTO semio (release, engine, created) VALUES (?, ?, ?);

-- Kit
-- Parameters: guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated
INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Quality
-- Parameters: guid, key, name, kind, default_value, formula, default_si_unit, default_imperial_unit, min_value, min_excluded, max_value, max_excluded, can_scale, definition, kit_guid
INSERT INTO quality (guid, key, name, kind, default_value, formula, default_si_unit, default_imperial_unit, min_value, min_excluded, max_value, max_excluded, can_scale, definition, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Benchmark
-- Parameters: guid, name, icon, min_value, min_excluded, max_value, max_excluded, definition, quality_guid
INSERT INTO benchmark (guid, name, icon, min_value, min_excluded, max_value, max_excluded, definition, quality_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Interface
-- Parameters: guid, name, description, icon, kit_guid
INSERT INTO port (guid, name, description, icon, kit_guid) VALUES (?, ?, ?, ?, ?);

-- Interface Compatibility
-- Parameters: port_guid, compatible_port_guid
INSERT INTO port_compatibility (port_guid, compatible_port_guid) VALUES (?, ?);

-- Folder
-- Parameters: guid, name, parent_guid, created, updated, kit_guid
INSERT INTO folder (guid, name, parent_guid, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?);

-- File
-- Parameters: guid, name, folder_guid, size, hash, remote_url, created, updated, kit_guid
INSERT INTO file (guid, name, folder_guid, size, hash, remote_url, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Author
-- Parameters: guid, name, email, kit_guid
INSERT INTO author (guid, name, email, kit_guid) VALUES (?, ?, ?, ?);

-- Type
-- Parameters: guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid
INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Model
-- Parameters: guid, file, name, description, type_guid
INSERT INTO model (guid, file, name, description, type_guid) VALUES (?, ?, ?, ?, ?);

-- Model Tag
-- Parameters: model_guid, tag
INSERT INTO model_tag (model_guid, tag) VALUES (?, ?);

-- Prop
-- Parameters: guid, key, value, unit, quality_guid, connector_guid
INSERT INTO prop (guid, key, value, unit, quality_guid, connector_guid) VALUES (?, ?, ?, ?, ?, ?);

-- Connector
-- Parameters: guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid
INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Design
-- Parameters: guid, name, parent_guid, variant, view_center_u, view_center_v, view_zoom, unit, location_guid, active_layer_guid, description, icon, image, created, updated, kit_guid
INSERT INTO design (guid, name, parent_guid, variant, view_center_u, view_center_v, view_zoom, unit, location_guid, active_layer_guid, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Layer
-- Parameters: guid, path, is_hidden, is_locked, color, description, design_guid
INSERT INTO layer (guid, path, is_hidden, is_locked, color, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?);

-- Piece
-- Parameters: guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z, mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z, mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z, is_hidden, is_locked, color, description, design_guid
INSERT INTO piece (guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z, mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z, mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z, is_hidden, is_locked, color, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Piece Prop
-- Parameters: piece_guid, prop_guid
INSERT INTO piece_prop (piece_guid, prop_guid) VALUES (?, ?);

-- Group
-- Parameters: guid, name, color, description, design_guid
INSERT INTO "group" (guid, name, color, description, design_guid) VALUES (?, ?, ?, ?, ?);

-- Group Piece
-- Parameters: group_guid, piece_guid
INSERT INTO group_piece (group_guid, piece_guid) VALUES (?, ?);

-- Connection
-- Parameters: guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid, connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid, gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid
INSERT INTO connection (guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid, connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid, gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- Stat
-- Parameters: guid, quality_guid, min_value, max_value, unit, design_guid
INSERT INTO stat (guid, quality_guid, min_value, max_value, unit, design_guid) VALUES (?, ?, ?, ?, ?, ?);

-- Concept (kit-level)
-- Parameters: kit_guid, value
INSERT INTO concept (kit_guid, value) VALUES (?, ?);

-- Type Concept
-- Parameters: type_guid, concept
INSERT INTO type_concept (type_guid, concept) VALUES (?, ?);

-- Design Concept
-- Parameters: design_guid, concept
INSERT INTO design_concept (design_guid, concept) VALUES (?, ?);

-- Attribute
-- Parameters: guid, key, value, definition, quality_guid, benchmark_guid, port_guid, folder_guid, file_guid, author_guid, model_guid, prop_guid, connector_guid, type_guid, layer_guid, piece_guid, group_guid, connection_guid, stat_guid, design_guid, kit_guid
INSERT INTO attribute (guid, key, value, definition, quality_guid, benchmark_guid, port_guid, folder_guid, file_guid, author_guid, model_guid, prop_guid, connector_guid, type_guid, layer_guid, piece_guid, group_guid, connection_guid, stat_guid, design_guid, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

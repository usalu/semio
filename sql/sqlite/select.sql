-- Select Statements for Semio Kit Database
-- All statements use parameterized queries (?) for security and reusability

-- Select all from kit (typically LIMIT 1)
SELECT * FROM kit LIMIT 1;

-- Select types by kit_guid
-- Parameters: kit_guid
SELECT * FROM type WHERE kit_guid = ?;

-- Select models by type_guid
-- Parameters: type_guid
SELECT * FROM model WHERE type_guid = ?;

-- Select model tags by model_guid
-- Parameters: model_guid
SELECT tag FROM model_tag WHERE model_guid = ?;

-- Select connectors by type_guid
-- Parameters: type_guid
SELECT * FROM connector WHERE type_guid = ?;

-- Select props by connector_guid
-- Parameters: connector_guid
SELECT * FROM prop WHERE connector_guid = ?;

-- Select designs by kit_guid
-- Parameters: kit_guid
SELECT * FROM design WHERE kit_guid = ?;

-- Select pieces by design_guid
-- Parameters: design_guid
SELECT * FROM piece WHERE design_guid = ?;

-- Select piece props by piece_guid
-- Parameters: piece_guid
SELECT p.* FROM prop p JOIN piece_prop pp ON p.guid = pp.prop_guid WHERE pp.piece_guid = ?;

-- Select connections by design_guid
-- Parameters: design_guid
SELECT * FROM connection WHERE design_guid = ?;

-- Select layers by design_guid
-- Parameters: design_guid
SELECT * FROM layer WHERE design_guid = ?;

-- Select groups by design_guid
-- Parameters: design_guid
SELECT * FROM "group" WHERE design_guid = ?;

-- Select group pieces by group_guid
-- Parameters: group_guid
SELECT piece_guid FROM group_piece WHERE group_guid = ?;

-- Select stats by design_guid
-- Parameters: design_guid
SELECT * FROM stat WHERE design_guid = ?;

-- Select interfaces by kit_guid
-- Parameters: kit_guid
SELECT * FROM interface WHERE kit_guid = ?;

-- Select compatible interfaces by interface_guid
-- Parameters: interface_guid
SELECT compatible_interface_guid FROM interface_compatibility WHERE interface_guid = ?;

-- Select qualities by kit_guid
-- Parameters: kit_guid
SELECT * FROM quality WHERE kit_guid = ?;

-- Select benchmarks by quality_guid
-- Parameters: quality_guid
SELECT * FROM benchmark WHERE quality_guid = ?;

-- Select files by kit_guid
-- Parameters: kit_guid
SELECT * FROM file WHERE kit_guid = ?;

-- Select folders by kit_guid
-- Parameters: kit_guid
SELECT * FROM folder WHERE kit_guid = ?;

-- Select authors by kit_guid
-- Parameters: kit_guid
SELECT * FROM author WHERE kit_guid = ?;

-- Select kit concepts by kit_guid
-- Parameters: kit_guid
SELECT value FROM concept WHERE kit_guid = ?;

-- Select type concepts by type_guid
-- Parameters: type_guid
SELECT concept FROM type_concept WHERE type_guid = ?;

-- Select design concepts by design_guid
-- Parameters: design_guid
SELECT concept FROM design_concept WHERE design_guid = ?;

-- Select attributes by kit_guid
-- Parameters: kit_guid
SELECT * FROM attribute WHERE kit_guid = ?;

-- Select attributes by type_guid
-- Parameters: type_guid
SELECT * FROM attribute WHERE type_guid = ?;

-- Select attributes by design_guid
-- Parameters: design_guid
SELECT * FROM attribute WHERE design_guid = ?;

-- Select attributes by model_guid
-- Parameters: model_guid
SELECT * FROM attribute WHERE model_guid = ?;

-- Select attributes by connector_guid
-- Parameters: connector_guid
SELECT * FROM attribute WHERE connector_guid = ?;

-- Select attributes by piece_guid
-- Parameters: piece_guid
SELECT * FROM attribute WHERE piece_guid = ?;

-- Select attributes by connection_guid
-- Parameters: connection_guid
SELECT * FROM attribute WHERE connection_guid = ?;

-- Select attributes by layer_guid
-- Parameters: layer_guid
SELECT * FROM attribute WHERE layer_guid = ?;

-- Select attributes by group_guid
-- Parameters: group_guid
SELECT * FROM attribute WHERE group_guid = ?;

-- Select attributes by stat_guid
-- Parameters: stat_guid
SELECT * FROM attribute WHERE stat_guid = ?;

-- Select attributes by quality_guid
-- Parameters: quality_guid
SELECT * FROM attribute WHERE quality_guid = ?;

-- Select attributes by benchmark_guid
-- Parameters: benchmark_guid
SELECT * FROM attribute WHERE benchmark_guid = ?;

-- Select attributes by interface_guid
-- Parameters: interface_guid
SELECT * FROM attribute WHERE interface_guid = ?;

-- Select attributes by folder_guid
-- Parameters: folder_guid
SELECT * FROM attribute WHERE folder_guid = ?;

-- Select attributes by file_guid
-- Parameters: file_guid
SELECT * FROM attribute WHERE file_guid = ?;

-- Select attributes by author_guid
-- Parameters: author_guid
SELECT * FROM attribute WHERE author_guid = ?;

-- Select attributes by prop_guid
-- Parameters: prop_guid
SELECT * FROM attribute WHERE prop_guid = ?;

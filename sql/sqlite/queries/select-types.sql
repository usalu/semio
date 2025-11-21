-- Select all types for a kit
SELECT guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, location_guid, description, icon, image, created, updated, kit_guid
FROM type
WHERE kit_guid = ?;

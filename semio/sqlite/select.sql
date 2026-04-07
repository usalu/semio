-- #region 🧲Header
-- semio/sqlite/select.sql
-- 2025 Ueli Saluz <ueli@semio-tech.com>
-- This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
-- #endregion 🧲Header

SELECT * FROM kit LIMIT 1;

SELECT * FROM type WHERE kit_guid = ?;

SELECT * FROM model WHERE type_guid = ?;

SELECT tag FROM model_tag WHERE model_guid = ?;

SELECT * FROM connector WHERE type_guid = ?;

SELECT * FROM prop WHERE connector_guid = ?;

SELECT * FROM design WHERE kit_guid = ?;

SELECT * FROM piece WHERE design_guid = ?;

SELECT p.* FROM prop p JOIN piece_prop pp ON p.guid = pp.prop_guid WHERE pp.piece_guid = ?;

SELECT * FROM connection WHERE design_guid = ?;

SELECT * FROM layer WHERE design_guid = ?;

SELECT * FROM "group" WHERE design_guid = ?;

SELECT piece_guid FROM group_piece WHERE group_guid = ?;

SELECT * FROM stat WHERE design_guid = ?;

SELECT * FROM port WHERE kit_guid = ?;

SELECT compatible_port_guid FROM port_compatibility WHERE port_guid = ?;

SELECT * FROM quality WHERE kit_guid = ?;

SELECT * FROM benchmark WHERE quality_guid = ?;

SELECT * FROM file WHERE kit_guid = ?;

SELECT * FROM folder WHERE kit_guid = ?;

SELECT * FROM author WHERE kit_guid = ?;

SELECT value FROM concept WHERE kit_guid = ?;

SELECT concept FROM type_concept WHERE type_guid = ?;

SELECT concept FROM design_concept WHERE design_guid = ?;

SELECT * FROM attribute WHERE kit_guid = ?;

SELECT * FROM attribute WHERE type_guid = ?;

SELECT * FROM attribute WHERE design_guid = ?;

SELECT * FROM attribute WHERE model_guid = ?;

SELECT * FROM attribute WHERE connector_guid = ?;

SELECT * FROM attribute WHERE piece_guid = ?;

SELECT * FROM attribute WHERE connection_guid = ?;

SELECT * FROM attribute WHERE layer_guid = ?;

SELECT * FROM attribute WHERE group_guid = ?;

SELECT * FROM attribute WHERE stat_guid = ?;

SELECT * FROM attribute WHERE quality_guid = ?;

SELECT * FROM attribute WHERE benchmark_guid = ?;

SELECT * FROM attribute WHERE port_guid = ?;

SELECT * FROM attribute WHERE folder_guid = ?;

SELECT * FROM attribute WHERE file_guid = ?;

SELECT * FROM attribute WHERE author_guid = ?;

SELECT * FROM attribute WHERE prop_guid = ?;

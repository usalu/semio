// #region 🔖Header
// [👤semio📚go💻kitsqlite](repo://p/u/semio/b/l/go/f/kit_sqlite.go)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// SQLite-backed persistence layer for kit import and export operations.

// #endregion 🔖Header

// #region 🔖SQLite Kit Operations
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations)
// SQLite kit operations. MUST provide serialization and deserialization of Kit to and from SQLite and zip formats.

package semio

import (
	"archive/zip"
	"bytes"
	"database/sql"
	"encoding/base64"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	_ "github.com/mattn/go-sqlite3"
)

// KitFromSqlite reads a Kit from a SQLite database file
// Callers MUST provide a valid path to an existing SQLite database
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️kitfromsqlite](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/KitFromSqlite)
func KitFromSqlite(dbPath string) (*Kit, error) {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, err
	}
	defer db.Close()

	kit := &Kit{}

	row := db.QueryRow("SELECT guid, name, version, description, icon, image, preview, remote, homepage, license FROM kit LIMIT 1")
	var version, description, icon, image, preview, remote, homepage, license sql.NullString
	if err := row.Scan(&kit.Guid, &kit.Name, &version, &description, &icon, &image, &preview, &remote, &homepage, &license); err != nil {
		return nil, fmt.Errorf("failed to scan kit: %w", err)
	}
	if version.Valid {
		kit.Version = version.String
	}
	if description.Valid {
		kit.Description = &description.String
	}
	if icon.Valid {
		kit.Icon = &icon.String
	}
	if image.Valid {
		kit.Image = &image.String
	}
	if preview.Valid {
		kit.Preview = &preview.String
	}
	if remote.Valid {
		kit.Remote = &remote.String
	}
	if homepage.Valid {
		kit.Homepage = &homepage.String
	}
	if license.Valid {
		kit.License = &license.String
	}

	types, err := loadTypes(db, kit.Guid)
	if err != nil {
		return nil, err
	}
	kit.Types = types

	designs, err := loadDesigns(db, kit.Guid)
	if err != nil {
		return nil, err
	}
	kit.Designs = designs

	return kit, nil
}

// loadTypes loads all types belonging to a kit from the database
// Callers MUST provide a valid open database connection and kit GUID
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️loadtypes](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/loadTypes)
func loadTypes(db *sql.DB, kitGuid string) ([]Type, error) {
	rows, err := db.Query("SELECT guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image FROM type WHERE kit_guid = ?", kitGuid)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var types []Type
	for rows.Next() {
		var t Type
		var parentGuid, folder, unit, description, icon, image sql.NullString
		var stock sql.NullInt32
		var isAbstract, virtual sql.NullBool
		if err := rows.Scan(&t.Guid, &t.Name, &parentGuid, &isAbstract, &folder, &stock, &virtual, &unit, &description, &icon, &image); err != nil {
			return nil, err
		}
		if parentGuid.Valid {
			t.Parent = &TypeId{Guid: parentGuid.String}
		}
		if folder.Valid {
			t.Folder = &folder.String
		}
		if stock.Valid {
			s := int(stock.Int32)
			t.Stock = &s
		}
		if unit.Valid {
			t.Unit = &unit.String
		}
		if description.Valid {
			t.Description = &description.String
		}
		if icon.Valid {
			t.Icon = &icon.String
		}
		if image.Valid {
			t.Image = &image.String
		}

		if isAbstract.Valid {
			t.IsAbstract = &isAbstract.Bool
		}
		if virtual.Valid {
			t.Virtual = &virtual.Bool
		}

		connectors, err := loadConnectors(db, t.Guid)
		if err != nil {
			return nil, err
		}
		t.Connectors = connectors

		types = append(types, t)
	}
	return types, nil
}

// loadDesigns loads all designs belonging to a kit from the database
// Callers MUST provide a valid open database connection and kit GUID
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️loaddesigns](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/loadDesigns)
func loadDesigns(db *sql.DB, kitGuid string) ([]Design, error) {
	rows, err := db.Query(`SELECT guid, name, parent_guid, variant, unit, folder, 
        is_abstract, can_scale, can_mirror, description, icon, image, created, updated 
        FROM design WHERE kit_guid = ?`, kitGuid)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var designs []Design
	for rows.Next() {
		var d Design
		var parentGuid, variant, unit, folder, description, icon, image sql.NullString
		var isAbstract, canScale, canMirror sql.NullBool
		var created, updated string
		if err := rows.Scan(&d.Guid, &d.Name, &parentGuid, &variant, &unit, &folder,
			&isAbstract, &canScale, &canMirror, &description, &icon, &image, &created, &updated); err != nil {
			return nil, err
		}
		if parentGuid.Valid {
			d.Parent = &DesignId{Guid: parentGuid.String}
		}
		if unit.Valid {
			d.Unit = &unit.String
		}
		if folder.Valid {
			d.Folder = &folder.String
		}
		if isAbstract.Valid {
			d.IsAbstract = &isAbstract.Bool
		}
		if canScale.Valid {
			d.CanScale = &canScale.Bool
		}
		if canMirror.Valid {
			d.CanMirror = &canMirror.Bool
		}
		if description.Valid {
			d.Description = &description.String
		}
		if icon.Valid {
			d.Icon = &icon.String
		}
		if image.Valid {
			d.Image = &image.String
		}
		d.CreatedAt = created
		d.UpdatedAt = updated

		pieces, err := loadPieces(db, d.Guid)
		if err != nil {
			return nil, err
		}
		d.Pieces = pieces

		connections, err := loadConnections(db, d.Guid)
		if err != nil {
			return nil, err
		}
		d.Connections = connections

		designs = append(designs, d)
	}
	return designs, nil
}

// loadPieces loads all pieces belonging to a design from the database
// Callers MUST provide a valid open database connection and design GUID
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️loadpieces](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/loadPieces)
func loadPieces(db *sql.DB, designGuid string) ([]Piece, error) {
	rows, err := db.Query(`SELECT guid, name, type_guid, design_guid_ref,
        plane_origin_x, plane_origin_y, plane_origin_z,
        plane_x_axis_x, plane_x_axis_y, plane_x_axis_z,
        plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
        center_u, center_v, scale, is_hidden, is_locked, color, description
        FROM piece WHERE design_guid = ?`, designGuid)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var pieces []Piece
	for rows.Next() {
		var p Piece
		var name, typeGuid, designGuidRef, color, description sql.NullString
		var originX, originY, originZ, xAxisX, xAxisY, xAxisZ, yAxisX, yAxisY, yAxisZ sql.NullFloat64
		var centerU, centerV, scale sql.NullFloat64
		var isHidden, isLocked bool
		if err := rows.Scan(&p.Guid, &name, &typeGuid, &designGuidRef,
			&originX, &originY, &originZ, &xAxisX, &xAxisY, &xAxisZ, &yAxisX, &yAxisY, &yAxisZ,
			&centerU, &centerV, &scale, &isHidden, &isLocked, &color, &description); err != nil {
			return nil, err
		}
		if name.Valid {
			p.Name = &name.String
		}
		if typeGuid.Valid {
			p.Type = &TypeId{Guid: typeGuid.String}
		}
		if designGuidRef.Valid {
			p.Design = &DesignId{Guid: designGuidRef.String}
		}
		if originX.Valid {
			p.Plane = &Plane{
				Origin: Point{X: originX.Float64, Y: originY.Float64, Z: originZ.Float64},
				XAxis:  Vector{X: xAxisX.Float64, Y: xAxisY.Float64, Z: xAxisZ.Float64},
				YAxis:  Vector{X: yAxisX.Float64, Y: yAxisY.Float64, Z: yAxisZ.Float64},
			}
		}
		if centerU.Valid && centerV.Valid {
			p.Center = &Coord{U: centerU.Float64, V: centerV.Float64}
		}
		if scale.Valid {
			p.Scale = &scale.Float64
		}
		p.IsHidden = &isHidden
		p.IsLocked = &isLocked
		if color.Valid {
			p.Color = &color.String
		}
		if description.Valid {
			p.Description = &description.String
		}
		pieces = append(pieces, p)
	}
	return pieces, nil
}

// loadConnections loads all connections belonging to a design from the database
// Callers MUST provide a valid open database connection and design GUID
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️loadconnections](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/loadConnections)
func loadConnections(db *sql.DB, designGuid string) ([]Connection, error) {
	rows, err := db.Query(`SELECT guid, connected_piece_guid, connected_connector_guid,
        connecting_piece_guid, connecting_connector_guid,
        gap, shift, rise, rotation, turn, tilt, u, v, description
        FROM connection WHERE design_guid = ?`, designGuid)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var connections []Connection
	for rows.Next() {
		var c Connection
		var connectedConnectorGuid, connectingConnectorGuid sql.NullString
		var u, v sql.NullFloat64
		var description sql.NullString
		var gap, shift, rise, rotation, turn, tilt float64
		if err := rows.Scan(&c.Guid, &c.Connected.Piece.Guid, &connectedConnectorGuid,
			&c.Connecting.Piece.Guid, &connectingConnectorGuid,
			&gap, &shift, &rise, &rotation, &turn, &tilt, &u, &v, &description); err != nil {
			return nil, err
		}
		if connectedConnectorGuid.Valid {
			c.Connected.Connector = &ConnectorId{Guid: connectedConnectorGuid.String}
		}
		if connectingConnectorGuid.Valid {
			c.Connecting.Connector = &ConnectorId{Guid: connectingConnectorGuid.String}
		}
		c.Gap = gap
		c.Shift = shift
		c.Rise = rise
		c.Rotation = rotation
		c.Turn = turn
		c.Tilt = tilt
		if u.Valid {
			c.U = u.Float64
		}
		if v.Valid {
			c.V = v.Float64
		}
		if description.Valid {
			c.Description = &description.String
		}
		connections = append(connections, c)
	}
	return connections, nil
}

// loadConnectors loads all connectors belonging to a type from the database
// Callers MUST provide a valid open database connection and type GUID
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️loadconnectors](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/loadConnectors)
func loadConnectors(db *sql.DB, typeGuid string) ([]Connector, error) {
	rows, err := db.Query(`SELECT guid, name, point_x, point_y, point_z,
        direction_x, direction_y, direction_z, t, mandatory, port_guid, description
        FROM connector WHERE type_guid = ?`, typeGuid)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var connectors []Connector
	for rows.Next() {
		var c Connector
		var name, portGuid, description sql.NullString
		var pointX, pointY, pointZ, dirX, dirY, dirZ, t float64
		var mandatory bool
		if err := rows.Scan(&c.Guid, &name, &pointX, &pointY, &pointZ,
			&dirX, &dirY, &dirZ, &t, &mandatory, &portGuid, &description); err != nil {
			return nil, err
		}
		if name.Valid {
			c.Name = &name.String
		}
		c.Point = Point{X: pointX, Y: pointY, Z: pointZ}
		c.Direction = Vector{X: dirX, Y: dirY, Z: dirZ}
		c.T = t
		c.Mandatory = &mandatory
		if portGuid.Valid {
			c.Port = &PortId{Guid: portGuid.String}
		}
		if description.Valid {
			c.Description = &description.String
		}
		connectors = append(connectors, c)
	}
	return connectors, nil
}

// KitToSqlite writes a Kit to a SQLite database file
// Callers MUST provide a valid Kit, writable database path, and schema SQL
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️kittosqlite](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/KitToSqlite)
func KitToSqlite(kit *Kit, dbPath string, schemaSQL string) error {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return err
	}
	defer db.Close()

	if _, err := db.Exec(schemaSQL); err != nil {
		return fmt.Errorf("failed to create schema: %w", err)
	}

	if _, err := db.Exec("PRAGMA foreign_keys = OFF"); err != nil {
		return fmt.Errorf("failed to disable foreign keys: %w", err)
	}

	if _, err := db.Exec(`INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))`,
		kit.Guid, kit.Name, kit.Version, kit.Description, kit.Icon, kit.Image, kit.Preview, kit.Remote, kit.Homepage, kit.License); err != nil {
		return fmt.Errorf("failed to insert kit: %w", err)
	}

	for _, t := range kit.Types {
		var parentGuid *string
		if t.Parent != nil {
			parentGuid = &t.Parent.Guid
		}

		virtualVal := false
		if t.Virtual != nil {
			virtualVal = *t.Virtual
		}
		isAbstractVal := false
		if t.IsAbstract != nil {
			isAbstractVal = *t.IsAbstract
		}
		if _, err := db.Exec(`INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'), ?)`,
			t.Guid, t.Name, parentGuid, isAbstractVal, t.Folder, t.Stock, virtualVal, t.Unit, t.Description, t.Icon, t.Image, kit.Guid); err != nil {
			return fmt.Errorf("failed to insert type %s: %w", t.Guid, err)
		}
		for _, c := range t.Connectors {
			var portGuid *string
			if c.Port != nil {
				portGuid = &c.Port.Guid
			}
			if _, err := db.Exec(`INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
				c.Guid, c.Name, c.Point.X, c.Point.Y, c.Point.Z, c.Direction.X, c.Direction.Y, c.Direction.Z, c.T, c.Mandatory, portGuid, c.Description, t.Guid); err != nil {
				return fmt.Errorf("failed to insert connector %s: %w", c.Guid, err)
			}
		}
	}

	for _, d := range kit.Designs {
		var parentGuid *string
		if d.Parent != nil {
			parentGuid = &d.Parent.Guid
		}
		if _, err := db.Exec(`INSERT INTO design (guid, name, parent_guid, unit, folder, is_abstract, can_scale, can_mirror, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'), ?)`,
			d.Guid, d.Name, parentGuid, d.Unit, d.Folder, d.IsAbstract, d.CanScale, d.CanMirror, d.Description, d.Icon, d.Image, kit.Guid); err != nil {
			return fmt.Errorf("failed to insert design %s: %w", d.Guid, err)
		}
		for _, p := range d.Pieces {
			var typeGuid, designRef *string
			if p.Type != nil {
				typeGuid = &p.Type.Guid
			}
			if p.Design != nil {
				designRef = &p.Design.Guid
			}
			var ox, oy, oz, xx, xy, xz, yx, yy, yz *float64
			if p.Plane != nil {
				ox, oy, oz = &p.Plane.Origin.X, &p.Plane.Origin.Y, &p.Plane.Origin.Z
				xx, xy, xz = &p.Plane.XAxis.X, &p.Plane.XAxis.Y, &p.Plane.XAxis.Z
				yx, yy, yz = &p.Plane.YAxis.X, &p.Plane.YAxis.Y, &p.Plane.YAxis.Z
			}
			var cu, cv *float64
			if p.Center != nil {
				cu, cv = &p.Center.U, &p.Center.V
			}
			if _, err := db.Exec(`INSERT INTO piece (guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, is_hidden, is_locked, color, description, design_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
				p.Guid, p.Name, typeGuid, designRef, ox, oy, oz, xx, xy, xz, yx, yy, yz, cu, cv, p.Scale, p.IsHidden, p.IsLocked, p.Color, p.Description, d.Guid); err != nil {
				return fmt.Errorf("failed to insert piece %s: %w", p.Guid, err)
			}
		}
		for _, c := range d.Connections {
			var cdConnGuid, cgConnGuid *string
			if c.Connected.Connector != nil {
				cdConnGuid = &c.Connected.Connector.Guid
			}
			if c.Connecting.Connector != nil {
				cgConnGuid = &c.Connecting.Connector.Guid
			}
			if _, err := db.Exec(`INSERT INTO connection (guid, connected_piece_guid, connected_connector_guid, connecting_piece_guid, connecting_connector_guid, gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
				c.Guid, c.Connected.Piece.Guid, cdConnGuid, c.Connecting.Piece.Guid, cgConnGuid, c.Gap, c.Shift, c.Rise, c.Rotation, c.Turn, c.Tilt, c.U, c.V, c.Description, d.Guid); err != nil {
				return fmt.Errorf("failed to insert connection %s: %w", c.Guid, err)
			}
		}
	}

	if _, err := db.Exec("PRAGMA foreign_keys = ON"); err != nil {
		return fmt.Errorf("failed to re-enable foreign keys: %w", err)
	}

	return nil
}

// KitFromZip extracts a Kit and its files from a zip archive
// Callers MUST provide a valid path to an existing zip file containing kit.json
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️kitfromzip](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/KitFromZip)
func KitFromZip(zipPath string) (*Kit, map[string][]byte, error) {
	r, err := zip.OpenReader(zipPath)
	if err != nil {
		return nil, nil, err
	}
	defer r.Close()

	var kitJSON []byte
	files := make(map[string][]byte)

	for _, f := range r.File {
		if f.FileInfo().IsDir() {
			continue
		}
		rc, err := f.Open()
		if err != nil {
			return nil, nil, err
		}
		data, err := io.ReadAll(rc)
		rc.Close()
		if err != nil {
			return nil, nil, err
		}

		if f.Name == "kit.json" {
			kitJSON = data
		} else if !strings.HasPrefix(f.Name, ".semio/") {
			files[f.Name] = data
		}
	}

	if kitJSON == nil {
		return nil, nil, fmt.Errorf("kit.json not found in zip")
	}

	kit, err := DeserializeKit(kitJSON)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to deserialize kit.json: %w", err)
	}

	for i := range kit.Files {
		filePath := buildFilePath(&kit, &kit.Files[i])
		if data, ok := files[filePath]; ok {
			encoded := blobEncode(data, kit.Files[i].Name)
			kit.Files[i].Blob = &encoded
		}
	}

	return &kit, files, nil
}

// buildFilePath constructs the file path from the folder hierarchy and file name
// buildFilePath MUST perform the buildFilePath operation.
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️buildfilepath](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/buildFilePath)
func buildFilePath(kit *Kit, file *File) string {
	if file.Folder == nil {
		return file.Name
	}
	folderPath := buildFolderPath(kit, file.Folder.Guid)
	if folderPath == "" {
		return file.Name
	}
	return folderPath + "/" + file.Name
}

// buildFolderPath constructs the folder path from the folder hierarchy
// buildFolderPath MUST perform the buildFolderPath operation.
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️buildfolderpath](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/buildFolderPath)
func buildFolderPath(kit *Kit, folderGuid string) string {
	for _, f := range kit.Folders {
		if f.Guid == folderGuid {
			if f.Parent == nil {
				return f.Name
			}
			parentPath := buildFolderPath(kit, f.Parent.Guid)
			if parentPath == "" {
				return f.Name
			}
			return parentPath + "/" + f.Name
		}
	}
	return ""
}

// blobEncode encodes bytes to a data URI string with the mime type inferred from filename.
// Falls back to "application/octet-stream" when the extension is unknown.
// blobEncode MUST perform the blobEncode operation.
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️blobencode](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/blobEncode)
func blobEncode(data []byte, filename string) string {
	mimeStr := mimeFromFilename(filename)
	return "data:" + mimeStr + ";base64," + base64.StdEncoding.EncodeToString(data)
}

// mimeFromFilename returns the mime type for a given filename based on its extension.
// Returns "application/octet-stream" when the extension is unknown.
func mimeFromFilename(filename string) string {
	ext := strings.ToLower(filepath.Ext(filename))
	mimes := map[string]string{
		".stl":  "model/stl",
		".obj":  "model/obj",
		".glb":  "model/gltf-binary",
		".gltf": "model/gltf+json",
		".3dm":  "model/vnd.3dm",
		".png":  "image/png",
		".jpg":  "image/jpeg",
		".jpeg": "image/jpeg",
		".svg":  "image/svg+xml",
		".pdf":  "application/pdf",
		".zip":  "application/zip",
		".json": "application/json",
		".csv":  "text/csv",
		".txt":  "text/plain",
	}
	if m, ok := mimes[ext]; ok {
		return m
	}
	return "application/octet-stream"
}

// blobDecode decodes a data URI string to bytes.
// Supports "data:<mime>;base64,<data>" format as well as raw base64 for backwards compatibility.
// blobDecode MUST perform the blobDecode operation.
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️blobdecode](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/blobDecode)
func blobDecode(s string) ([]byte, error) {
	if strings.HasPrefix(s, "data:") {
		commaIdx := strings.Index(s, ",")
		if commaIdx < 0 {
			return nil, fmt.Errorf("invalid data URI: missing comma")
		}
		return base64.StdEncoding.DecodeString(s[commaIdx+1:])
	}
	return base64.StdEncoding.DecodeString(s)
}

// KitToZip packages a Kit and its files into a zip archive
// Callers MUST provide a valid Kit, writable zip path
// [👤semio📚go💻kitsqlite🔖sqlitekitoperations🛠️kittozip](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/SQLite%20Kit%20Operations/d/i/KitToZip)
func KitToZip(kit *Kit, files map[string][]byte, zipPath string, schemaSQL string) error {

	kitForZip := *kit
	kitForZip.Files = make([]File, len(kit.Files))
	copy(kitForZip.Files, kit.Files)
	for i := range kitForZip.Files {
		kitForZip.Files[i].Blob = nil
	}

	kitJSON, err := SerializeKit(kitForZip)
	if err != nil {
		return fmt.Errorf("failed to serialize kit: %w", err)
	}

	zipFile, err := os.Create(zipPath)
	if err != nil {
		return err
	}
	defer zipFile.Close()

	w := zip.NewWriter(zipFile)
	defer w.Close()

	kitWriter, err := w.Create("kit.json")
	if err != nil {
		return err
	}
	if _, err := kitWriter.Write(kitJSON); err != nil {
		return err
	}

	for name, data := range files {
		fw, err := w.Create(name)
		if err != nil {
			return err
		}
		if _, err := fw.Write(data); err != nil {
			return err
		}
	}

	return nil
}

// #region 🔖Kit Workflow Operations
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations)
// Kit workflow operations MUST provide direct import, export, and edit flows for file, folder, archive, remote, and temporary kit kinds.

// ImportFileKit reads a JSON file kit from disk.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️importfilekit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/ImportFileKit)
func ImportFileKit(path string) (*Kit, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	kit, err := DeserializeKit(data)
	if err != nil {
		return nil, err
	}
	return &kit, nil
}

// ExportFileKit writes a JSON file kit to disk.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️exportfilekit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/ExportFileKit)
func ExportFileKit(kit Kit, path string) error {
	data, err := SerializeKit(kit)
	if err != nil {
		return err
	}
	return os.WriteFile(path, data, 0o644)
}

// ImportArchiveKit reads an archive kit from a zip file.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️importarchivekit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/ImportArchiveKit)
func ImportArchiveKit(path string) (*Kit, map[string][]byte, error) {
	return KitFromZip(path)
}

// ExportArchiveKit writes an archive kit to a zip file.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️exportarchivekit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/ExportArchiveKit)
func ExportArchiveKit(kit *Kit, files map[string][]byte, path string) error {
	return KitToZip(kit, ensureKitFiles(kit, files), path, "")
}

// ImportFolderKit reads a folder kit from a local folder containing .semio/kit.db and asset files.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️importfolderkit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/ImportFolderKit)
func ImportFolderKit(folderPath string) (*Kit, map[string][]byte, error) {
	dbPath := filepath.Join(folderPath, ".semio", "kit.db")
	kit, err := KitFromSqlite(dbPath)
	if err != nil {
		return nil, nil, err
	}
	files := map[string][]byte{}
	err = filepath.WalkDir(folderPath, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		if d.IsDir() {
			if path == filepath.Join(folderPath, ".semio") {
				return filepath.SkipDir
			}
			return nil
		}
		relPath, err := filepath.Rel(folderPath, path)
		if err != nil {
			return err
		}
		relPath = filepath.ToSlash(relPath)
		data, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		files[relPath] = data
		return nil
	})
	if err != nil {
		return nil, nil, err
	}
	hydrateKitFiles(kit, files)
	return kit, files, nil
}

// ExportFolderKit writes a folder kit to a local folder containing .semio/kit.db and asset files.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️exportfolderkit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/ExportFolderKit)
func ExportFolderKit(kit *Kit, files map[string][]byte, folderPath string) error {
	semioPath := filepath.Join(folderPath, ".semio")
	if err := os.MkdirAll(semioPath, 0o755); err != nil {
		return err
	}
	dbPath := filepath.Join(semioPath, "kit.db")
	if err := os.RemoveAll(dbPath); err != nil && !os.IsNotExist(err) {
		return err
	}
	if err := KitToSqlite(kit, dbPath, mustReadKitSchemaSQL()); err != nil {
		return err
	}
	for name, data := range ensureKitFiles(kit, files) {
		fullPath := filepath.Join(folderPath, filepath.FromSlash(name))
		if err := os.MkdirAll(filepath.Dir(fullPath), 0o755); err != nil {
			return err
		}
		if err := os.WriteFile(fullPath, data, 0o644); err != nil {
			return err
		}
	}
	return nil
}

// ImportRemoteKit reads a remote kit from HTTP(S), supporting both JSON and ZIP sources.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️importremotekit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/ImportRemoteKit)
func ImportRemoteKit(rawURL string) (*Kit, map[string][]byte, error) {
	response, err := http.Get(rawURL)
	if err != nil {
		return nil, nil, err
	}
	defer response.Body.Close()
	if response.StatusCode < 200 || response.StatusCode >= 300 {
		return nil, nil, fmt.Errorf("failed to fetch remote kit %s: %s", rawURL, response.Status)
	}
	body, err := io.ReadAll(response.Body)
	if err != nil {
		return nil, nil, err
	}
	contentType := strings.ToLower(response.Header.Get("Content-Type"))
	trimmed := bytes.TrimSpace(body)
	if strings.HasSuffix(strings.ToLower(rawURL), ".zip") || strings.Contains(contentType, "zip") || strings.Contains(contentType, "octet-stream") || bytes.HasPrefix(body, []byte("PK\x03\x04")) {
		tmpFile, err := os.CreateTemp("", "semio-remote-*.zip")
		if err != nil {
			return nil, nil, err
		}
		defer os.Remove(tmpFile.Name())
		if _, err := tmpFile.Write(body); err != nil {
			tmpFile.Close()
			return nil, nil, err
		}
		if err := tmpFile.Close(); err != nil {
			return nil, nil, err
		}
		return KitFromZip(tmpFile.Name())
	}
	if len(trimmed) > 0 && trimmed[0] == '{' {
		kit, err := DeserializeKit(body)
		if err != nil {
			return nil, nil, err
		}
		return &kit, map[string][]byte{}, nil
	}
	return nil, nil, fmt.Errorf("remote kit %s is neither JSON nor ZIP", rawURL)
}

// EditTemporaryKit applies a diff to an in-memory kit value and returns the edited kit.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️edittemporarykit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/EditTemporaryKit)
func EditTemporaryKit(kit Kit, diff KitDiff) Kit {
	return ApplyKitDiff(kit, diff)
}

// EditFileKit edits a file kit in place and returns the edited kit.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️editfilekit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/EditFileKit)
func EditFileKit(path string, diff KitDiff) (*Kit, error) {
	kit, err := ImportFileKit(path)
	if err != nil {
		return nil, err
	}
	edited := EditTemporaryKit(*kit, diff)
	if err := ExportFileKit(edited, path); err != nil {
		return nil, err
	}
	return &edited, nil
}

// EditFolderKit edits a folder kit in place and returns the edited kit.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️editfolderkit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/EditFolderKit)
func EditFolderKit(folderPath string, diff KitDiff) (*Kit, error) {
	kit, files, err := ImportFolderKit(folderPath)
	if err != nil {
		return nil, err
	}
	edited := EditTemporaryKit(*kit, diff)
	if err := ExportFolderKit(&edited, files, folderPath); err != nil {
		return nil, err
	}
	return &edited, nil
}

// EditArchiveKit edits an archive kit in place and returns the edited kit.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️editarchivekit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/EditArchiveKit)
func EditArchiveKit(path string, diff KitDiff) (*Kit, error) {
	kit, files, err := ImportArchiveKit(path)
	if err != nil {
		return nil, err
	}
	edited := EditTemporaryKit(*kit, diff)
	if err := ExportArchiveKit(&edited, files, path); err != nil {
		return nil, err
	}
	return &edited, nil
}

// EditRemoteKit imports a remote kit and applies a diff in memory.
// [👤semio📚go💻kitsqlite🔖kitworkflowoperations🛠️editremotekit](repo://p/u/semio/b/l/go/f/kit_sqlite.go/s/Kit%20Workflow%20Operations/d/i/EditRemoteKit)
func EditRemoteKit(rawURL string, diff KitDiff) (*Kit, error) {
	kit, _, err := ImportRemoteKit(rawURL)
	if err != nil {
		return nil, err
	}
	edited := EditTemporaryKit(*kit, diff)
	return &edited, nil
}

func ensureKitFiles(kit *Kit, files map[string][]byte) map[string][]byte {
	if files != nil {
		return files
	}
	collected := map[string][]byte{}
	for i := range kit.Files {
		if kit.Files[i].Blob == nil {
			continue
		}
		data, err := blobDecode(*kit.Files[i].Blob)
		if err != nil {
			continue
		}
		collected[buildFilePath(kit, &kit.Files[i])] = data
	}
	return collected
}

func hydrateKitFiles(kit *Kit, files map[string][]byte) {
	for i := range kit.Files {
		filePath := buildFilePath(kit, &kit.Files[i])
		if data, ok := files[filePath]; ok {
			encoded := blobEncode(data, kit.Files[i].Name)
			kit.Files[i].Blob = &encoded
		}
	}
}

func mustReadKitSchemaSQL() string {
	candidatePaths := []string{
		"../sqlite/schema.sql",
		"../../sqlite/schema.sql",
		"sqlite/schema.sql",
	}
	for _, candidate := range candidatePaths {
		if data, err := os.ReadFile(candidate); err == nil {
			return string(data)
		}
	}
	panic("failed to locate sqlite/schema.sql for kit workflow operations")
}

// #endregion 🔖Kit Workflow Operations

// #endregion 🔖SQLite Kit Operations

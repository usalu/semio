// #region 🔖Header

// [👤semio📚go💻kitsqlitego](semiorepo://file/semio/go/kit_sqlite.go)

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

// [👤semio📚go💻kitsqlitego🔖sqlitekitoperations](semiorepo://section/semio/go/kit_sqlite.go/SQLite%20Kit%20Operations)
// SQLite kit operations. MUST provide serialization and deserialization of Kit to and from SQLite and zip formats.

package semio

import (
	"archive/zip"
	"database/sql"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	_ "github.com/mattn/go-sqlite3"
)

// KitFromSqlite reads a Kit from a SQLite database file
// Callers MUST provide a valid path to an existing SQLite database
// [👤semio📚go💻kitsqlitego🔖sqlitekitoperations🛠️kitfromsqlite](semiorepo://definition/semio/go/kit_sqlite.go/SQLite%20Kit%20Operations/KitFromSqlite)
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
	if version.Valid { kit.Version = version.String }
	if description.Valid { kit.Description = &description.String }
	if icon.Valid { kit.Icon = &icon.String }
	if image.Valid { kit.Image = &image.String }
	if preview.Valid { kit.Preview = &preview.String }
	if remote.Valid { kit.Remote = &remote.String }
	if homepage.Valid { kit.Homepage = &homepage.String }
	if license.Valid { kit.License = &license.String }

    types, err := loadTypes(db, kit.Guid)
    if err != nil { return nil, err }
    kit.Types = types

    designs, err := loadDesigns(db, kit.Guid)
    if err != nil { return nil, err }
    kit.Designs = designs

	return kit, nil
}

// loadTypes loads all types belonging to a kit from the database
// Callers MUST provide a valid open database connection and kit GUID
func loadTypes(db *sql.DB, kitGuid string) ([]Type, error) {
    rows, err := db.Query("SELECT guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image FROM type WHERE kit_guid = ?", kitGuid)
    if err != nil { return nil, err }
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
        if parentGuid.Valid { t.Parent = &TypeId{Guid: parentGuid.String} }
        if folder.Valid { t.Folder = &folder.String }
        if stock.Valid { s := int(stock.Int32); t.Stock = &s }
        if unit.Valid { t.Unit = &unit.String }
        if description.Valid { t.Description = &description.String }
        if icon.Valid { t.Icon = &icon.String }
        if image.Valid { t.Image = &image.String }
        
        if isAbstract.Valid { t.IsAbstract = &isAbstract.Bool }
        if virtual.Valid { t.Virtual = &virtual.Bool }

        connectors, err := loadConnectors(db, t.Guid)
        if err != nil { return nil, err }
        t.Connectors = connectors
        
        types = append(types, t)
    }
    return types, nil
}

// loadDesigns loads all designs belonging to a kit from the database
// Callers MUST provide a valid open database connection and kit GUID
func loadDesigns(db *sql.DB, kitGuid string) ([]Design, error) {
    rows, err := db.Query(`SELECT guid, name, parent_guid, variant, unit, folder, 
        is_abstract, can_scale, can_mirror, description, icon, image, created, updated 
        FROM design WHERE kit_guid = ?`, kitGuid)
    if err != nil { return nil, err }
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
        if parentGuid.Valid { d.Parent = &DesignId{Guid: parentGuid.String} }
        if unit.Valid { d.Unit = &unit.String }
        if folder.Valid { d.Folder = &folder.String }
        if isAbstract.Valid { d.IsAbstract = &isAbstract.Bool }
        if canScale.Valid { d.CanScale = &canScale.Bool }
        if canMirror.Valid { d.CanMirror = &canMirror.Bool }
        if description.Valid { d.Description = &description.String }
        if icon.Valid { d.Icon = &icon.String }
        if image.Valid { d.Image = &image.String }
        d.CreatedAt = created
        d.UpdatedAt = updated

        pieces, err := loadPieces(db, d.Guid)
        if err != nil { return nil, err }
        d.Pieces = pieces

        connections, err := loadConnections(db, d.Guid)
        if err != nil { return nil, err }
        d.Connections = connections

        designs = append(designs, d)
    }
    return designs, nil
}

// loadPieces loads all pieces belonging to a design from the database
// Callers MUST provide a valid open database connection and design GUID
func loadPieces(db *sql.DB, designGuid string) ([]Piece, error) {
    rows, err := db.Query(`SELECT guid, name, type_guid, design_guid_ref,
        plane_origin_x, plane_origin_y, plane_origin_z,
        plane_x_axis_x, plane_x_axis_y, plane_x_axis_z,
        plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
        center_u, center_v, scale, is_hidden, is_locked, color, description
        FROM piece WHERE design_guid = ?`, designGuid)
    if err != nil { return nil, err }
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
        if name.Valid { p.Name = &name.String }
        if typeGuid.Valid { p.Type = &TypeId{Guid: typeGuid.String} }
        if designGuidRef.Valid { p.Design = &DesignId{Guid: designGuidRef.String} }
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
        if scale.Valid { p.Scale = &scale.Float64 }
        p.IsHidden = &isHidden
        p.IsLocked = &isLocked
        if color.Valid { p.Color = &color.String }
        if description.Valid { p.Description = &description.String }
        pieces = append(pieces, p)
    }
    return pieces, nil
}

// loadConnections loads all connections belonging to a design from the database
// Callers MUST provide a valid open database connection and design GUID
func loadConnections(db *sql.DB, designGuid string) ([]Connection, error) {
    rows, err := db.Query(`SELECT guid, connected_piece_guid, connected_connector_guid,
        connecting_piece_guid, connecting_connector_guid,
        gap, shift, rise, rotation, turn, tilt, u, v, description
        FROM connection WHERE design_guid = ?`, designGuid)
    if err != nil { return nil, err }
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
        if connectedConnectorGuid.Valid { c.Connected.Connector = &ConnectorId{Guid: connectedConnectorGuid.String} }
        if connectingConnectorGuid.Valid { c.Connecting.Connector = &ConnectorId{Guid: connectingConnectorGuid.String} }
        c.Gap = gap
        c.Shift = shift
        c.Rise = rise
        c.Rotation = rotation
        c.Turn = turn
        c.Tilt = tilt
        if u.Valid { c.U = u.Float64 }
        if v.Valid { c.V = v.Float64 }
        if description.Valid { c.Description = &description.String }
        connections = append(connections, c)
    }
    return connections, nil
}

// loadConnectors loads all connectors belonging to a type from the database
// Callers MUST provide a valid open database connection and type GUID
func loadConnectors(db *sql.DB, typeGuid string) ([]Connector, error) {
    rows, err := db.Query(`SELECT guid, name, point_x, point_y, point_z,
        direction_x, direction_y, direction_z, t, mandatory, port_guid, description
        FROM connector WHERE type_guid = ?`, typeGuid)
    if err != nil { return nil, err }
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
        if name.Valid { c.Name = &name.String }
        c.Point = Point{X: pointX, Y: pointY, Z: pointZ}
        c.Direction = Vector{X: dirX, Y: dirY, Z: dirZ}
        c.T = t
        c.Mandatory = &mandatory
        if portGuid.Valid { c.Port = &PortId{Guid: portGuid.String} }
        if description.Valid { c.Description = &description.String }
        connectors = append(connectors, c)
    }
    return connectors, nil
}

// KitToSqlite writes a Kit to a SQLite database file
// Callers MUST provide a valid Kit, writable database path, and schema SQL
// [👤semio📚go💻kitsqlitego🔖sqlitekitoperations🛠️kittosqlite](semiorepo://definition/semio/go/kit_sqlite.go/SQLite%20Kit%20Operations/KitToSqlite)
func KitToSqlite(kit *Kit, dbPath string, schemaSQL string) error {
    db, err := sql.Open("sqlite3", dbPath)
    if err != nil { return err }
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
        if t.Parent != nil { parentGuid = &t.Parent.Guid }

        virtualVal := false
        if t.Virtual != nil { virtualVal = *t.Virtual }
        isAbstractVal := false
        if t.IsAbstract != nil { isAbstractVal = *t.IsAbstract }
        if _, err := db.Exec(`INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'), ?)`,
            t.Guid, t.Name, parentGuid, isAbstractVal, t.Folder, t.Stock, virtualVal, t.Unit, t.Description, t.Icon, t.Image, kit.Guid); err != nil {
            return fmt.Errorf("failed to insert type %s: %w", t.Guid, err)
        }
        for _, c := range t.Connectors {
            var portGuid *string
            if c.Port != nil { portGuid = &c.Port.Guid }
            if _, err := db.Exec(`INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
                c.Guid, c.Name, c.Point.X, c.Point.Y, c.Point.Z, c.Direction.X, c.Direction.Y, c.Direction.Z, c.T, c.Mandatory, portGuid, c.Description, t.Guid); err != nil {
                return fmt.Errorf("failed to insert connector %s: %w", c.Guid, err)
            }
        }
    }

    for _, d := range kit.Designs {
        var parentGuid *string
        if d.Parent != nil { parentGuid = &d.Parent.Guid }
        if _, err := db.Exec(`INSERT INTO design (guid, name, parent_guid, unit, folder, is_abstract, can_scale, can_mirror, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'), ?)`,
            d.Guid, d.Name, parentGuid, d.Unit, d.Folder, d.IsAbstract, d.CanScale, d.CanMirror, d.Description, d.Icon, d.Image, kit.Guid); err != nil {
            return fmt.Errorf("failed to insert design %s: %w", d.Guid, err)
        }
        for _, p := range d.Pieces {
            var typeGuid, designRef *string
            if p.Type != nil { typeGuid = &p.Type.Guid }
            if p.Design != nil { designRef = &p.Design.Guid }
            var ox, oy, oz, xx, xy, xz, yx, yy, yz *float64
            if p.Plane != nil {
                ox, oy, oz = &p.Plane.Origin.X, &p.Plane.Origin.Y, &p.Plane.Origin.Z
                xx, xy, xz = &p.Plane.XAxis.X, &p.Plane.XAxis.Y, &p.Plane.XAxis.Z
                yx, yy, yz = &p.Plane.YAxis.X, &p.Plane.YAxis.Y, &p.Plane.YAxis.Z
            }
            var cu, cv *float64
            if p.Center != nil { cu, cv = &p.Center.U, &p.Center.V }
            if _, err := db.Exec(`INSERT INTO piece (guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z, plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z, center_u, center_v, scale, is_hidden, is_locked, color, description, design_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
                p.Guid, p.Name, typeGuid, designRef, ox, oy, oz, xx, xy, xz, yx, yy, yz, cu, cv, p.Scale, p.IsHidden, p.IsLocked, p.Color, p.Description, d.Guid); err != nil {
                return fmt.Errorf("failed to insert piece %s: %w", p.Guid, err)
            }
        }
        for _, c := range d.Connections {
            var cdConnGuid, cgConnGuid *string
            if c.Connected.Connector != nil { cdConnGuid = &c.Connected.Connector.Guid }
            if c.Connecting.Connector != nil { cgConnGuid = &c.Connecting.Connector.Guid }
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
// Callers MUST provide a valid path to an existing zip file containing kit.db
// [👤semio📚go💻kitsqlitego🔖sqlitekitoperations🛠️kitfromzip](semiorepo://definition/semio/go/kit_sqlite.go/SQLite%20Kit%20Operations/KitFromZip)
func KitFromZip(zipPath string) (*Kit, map[string][]byte, error) {
    tmpDir, err := os.MkdirTemp("", "semio-kit-*")
    if err != nil { return nil, nil, err }
    defer os.RemoveAll(tmpDir)

    files := make(map[string][]byte)
    r, err := zip.OpenReader(zipPath)
    if err != nil { return nil, nil, err }
    defer r.Close()

    for _, f := range r.File {
        if f.FileInfo().IsDir() { continue }
        rc, err := f.Open()
        if err != nil { return nil, nil, err }
        
        destPath := filepath.Join(tmpDir, f.Name)
        os.MkdirAll(filepath.Dir(destPath), 0755)
        
        outFile, err := os.Create(destPath)
        if err != nil { rc.Close(); return nil, nil, err }
        
        _, err = io.Copy(outFile, rc)
        outFile.Close()
        rc.Close()
        if err != nil { return nil, nil, err }

        if !strings.HasPrefix(f.Name, ".semio/") {
            data, _ := os.ReadFile(destPath)
            files[f.Name] = data
        }
    }

    dbPath := filepath.Join(tmpDir, ".semio", "kit.db")
    if _, err := os.Stat(dbPath); os.IsNotExist(err) {
        return nil, nil, fmt.Errorf("kit.db not found in zip")
    }

    kit, err := KitFromSqlite(dbPath)
    if err != nil { return nil, nil, err }

    return kit, files, nil
}

// KitToZip packages a Kit and its files into a zip archive
// Callers MUST provide a valid Kit, file map, writable zip path, and schema SQL
// [👤semio📚go💻kitsqlitego🔖sqlitekitoperations🛠️kittozip](semiorepo://definition/semio/go/kit_sqlite.go/SQLite%20Kit%20Operations/KitToZip)
func KitToZip(kit *Kit, files map[string][]byte, zipPath string, schemaSQL string) error {
    tmpDir, err := os.MkdirTemp("", "semio-kit-*")
    if err != nil { return err }
    defer os.RemoveAll(tmpDir)

    semioDir := filepath.Join(tmpDir, ".semio")
    os.MkdirAll(semioDir, 0755)
    dbPath := filepath.Join(semioDir, "kit.db")

    if err := KitToSqlite(kit, dbPath, schemaSQL); err != nil {
        return err
    }

    zipFile, err := os.Create(zipPath)
    if err != nil { return err }
    defer zipFile.Close()

    w := zip.NewWriter(zipFile)
    defer w.Close()

    dbData, err := os.ReadFile(dbPath)
    if err != nil { return err }
    dbWriter, err := w.Create(".semio/kit.db")
    if err != nil { return err }
    if _, err := dbWriter.Write(dbData); err != nil { return err }

    for name, data := range files {
        fw, err := w.Create(name)
        if err != nil { return err }
        if _, err := fw.Write(data); err != nil { return err }
    }

    return nil
}

// #endregion 🔖SQLite Kit Operations

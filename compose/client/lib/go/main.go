// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Core domain library in Go implementing the compose data model, SQLite kit I/O, and operations.

// #endregion 🧲Header

// #region 🔌Adapters

package compose

import (
	"bytes"
	"crypto/rand"
	"crypto/sha256"
	"encoding/base64"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"path"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"

	"archive/zip"
	"database/sql"
	"io"
	"net/http"

	_ "modernc.org/sqlite"

	"gonum.org/v1/gonum/mat"
)

// #endregion 🔌Adapters

// #region 🎞️Constants

const (
	IconWidth = 24
	Tolerance = 0.0001
)

const AssetsPath = "../asset/compose"

// #endregion 🎞️Constants

// #region 📦Utilities

// 🎲Id generates a new random 128-bit hex-encoded unique identifier.
// 📎ptrString returns a pointer to the given string value.
func ptrString(s string) *string { return &s }

func ptrFloat64(f float64) *float64 { return &f }

func floatEqual(a, b, tolerance float64) bool {
	return math.Abs(a-b) < tolerance
}

func optFloatEqual(a, b *float64) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return floatEqual(*a, *b, 1e-9)
}

func optBoolEqual(a, b *bool) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return *a == *b
}

func optStringEqual(a, b *string) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return *a == *b
}

func areLocationIdsEqual(a, b *LocationId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Id == b.Id
}

func areTypeIdsEqual(a, b *TypeId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Id == b.Id
}

func areDesignIdsEqual(a, b *DesignId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Id == b.Id
}

func areTypeIdSlicesEqual(a, b []TypeId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Id != b[i].Id {
			return false
		}
	}
	return true
}

func areDesignIdSlicesEqual(a, b []DesignId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Id != b[i].Id {
			return false
		}
	}
	return true
}

func arePortIdsEqual(a, b *PortId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Id == b.Id
}

func areLayerIdsEqual(a, b *LayerId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Id == b.Id
}

func normalizeOptInt(p *int) int {
	if p == nil {
		return 0
	}
	return *p
}

func areAuthorIdsEqual(a, b []AuthorId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Id != b[i].Id {
			return false
		}
	}
	return true
}

func areConceptIdsEqual(a, b []ConceptId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Id != b[i].Id {
			return false
		}
	}
	return true
}

func arePortIdSlicesEqual(a, b []PortId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Id != b[i].Id {
			return false
		}
	}
	return true
}

func areFamilyIdSlicesEqual(a, b []FamilyId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Id != b[i].Id {
			return false
		}
	}
	return true
}

func areAttributesEqual(a, b []Attribute) bool {
	if len(a) != len(b) {
		return false
	}
	aMap := make(map[string]Attribute)
	for _, attr := range a {
		aMap[attr.Id] = attr
	}
	for _, attr := range b {
		other, ok := aMap[attr.Id]
		if !ok {
			return false
		}
		if attr.Key != other.Key {
			return false
		}
		if !optStringEqual(attr.Value, other.Value) {
			return false
		}
		if !optStringEqual(attr.Definition, other.Definition) {
			return false
		}
	}
	return true
}

func arePropsEqual(a, b []Prop) bool {
	if len(a) != len(b) {
		return false
	}
	aMap := make(map[string]Prop)
	for _, p := range a {
		aMap[p.Id] = p
	}
	for _, p := range b {
		other, ok := aMap[p.Id]
		if !ok {
			return false
		}
		if p.Quality.Id != other.Quality.Id || p.Value != other.Value || normalizeStr(p.Unit) != normalizeStr(other.Unit) {
			return false
		}
	}
	return true
}

func Id() string {
	bytes := make([]byte, 16)
	rand.Read(bytes)
	return hex.EncodeToString(bytes)
}

// 🔡Normalize converts a string to lowercase trimmed form.
func Normalize(s string) string {
	return strings.ToLower(strings.TrimSpace(s))
}

// 🔢Round rounds a float64 to the specified number of decimal places.
func Round(value float64, decimals int) float64 {
	shift := 1.0
	for i := 0; i < decimals; i++ {
		shift *= 10
	}
	return float64(int64(value*shift+0.5)) / shift
}

// ⚖️DeepEqual compares two values for deep equality via JSON serialization.
func DeepEqual(a, b interface{}) bool {
	aJSON, _ := json.Marshal(a)
	bJSON, _ := json.Marshal(b)
	return string(aJSON) == string(bJSON)
}

// #endregion 📦Utilities

// #region 🐍Entity IDs
// Entity IDs MUST define identifier types for all compose domain entities.

// 💎AttributeId identifies an attribute entity by ID.
type AttributeId struct {
	Id string `json:"id"`
}

// 📍LocationId identifies a location entity by ID.
type LocationId struct {
	Id string `json:"id"`
}

// ✍️AuthorId identifies an author entity by ID.
type AuthorId struct {
	Id string `json:"id"`
}

// 📄FileId identifies a file entity by ID.
type FileId struct {
	Id string `json:"id"`
}

// 📁FolderId identifies a folder entity by ID.
type FolderId struct {
	Id string `json:"id"`
}

// 📏BenchmarkId identifies a benchmark entity by ID.
type BenchmarkId struct {
	Id string `json:"id"`
}

// 🔬QualityId identifies a quality entity by ID.
type QualityId struct {
	Id string `json:"id"`
}

// ⚓PortId identifies a port entity by ID.
type PortId struct {
	Id string `json:"id"`
}

// 👪FamilyId identifies a first-class family entity by ID.
type FamilyId struct {
	Id string `json:"id"`
}

// 🏛️TypologyId identifies a typology entity that owns types and designs.
type TypologyId struct {
	Id string `json:"id"`
}

// 📊PropId identifies a prop entity by ID.
type PropId struct {
	Id string `json:"id"`
}

// 🏷️TagId identifies a tag entity by ID.
type TagId struct {
	Id string `json:"id"`
}

// 💡ConceptId identifies a concept entity by ID.
type ConceptId struct {
	Id string `json:"id"`
}

// 🗿RepresentationId identifies a representation entity by ID.
type RepresentationId struct {
	Id string `json:"id"`
}

// 🔌ConnectorId identifies a connector entity by ID.
type ConnectorId struct {
	Id string `json:"id"`
}

// 🧱TypeId identifies a type entity by ID.
type TypeId struct {
	Id string `json:"id"`
}

// 🎨LayerId identifies a layer entity by ID.
type LayerId struct {
	Id string `json:"id"`
}

// 🧩PieceId identifies a piece entity by ID.
type PieceId struct {
	Id string `json:"id"`
}

// 👥GroupId identifies a group entity by ID.
type GroupId struct {
	Id string `json:"id"`
}

// ↔️SideId identifies a connection side by piece, design piece and connector references.
type SideId struct {
	Piece       PieceId      `json:"piece"`
	DesignPiece *PieceId     `json:"designPiece,omitempty"`
	Connector   *ConnectorId `json:"connector,omitempty"`
}

// 🔗ConnectionId identifies a connection entity by ID.
type ConnectionId struct {
	Id string `json:"id"`
}

// 📈StatId identifies a stat entity by ID.
type StatId struct {
	Id string `json:"id"`
}

// 📐DesignId identifies a design entity by ID.
type DesignId struct {
	Id string `json:"id"`
}

// 📦KitId identifies a kit entity by ID.
type KitId struct {
	Id string `json:"id"`
}

// #endregion 🐍Entity IDs

// #region 🖥️Weak Entities
// Weak Entities MUST define value types that exist only as part of parent entities.

// 📺Coordinate represents a 2D coordinate with U and V components.
type Coordinate struct {
	U float64 `json:"u"`
	V float64 `json:"v"`
}

// ➡️Vec represents a 2D vector with U and V components.
type Vec struct {
	U float64 `json:"u"`
	V float64 `json:"v"`
}

// ✖️Point represents a 3D point with X, Y and Z components.
type Point struct {
	X float64 `json:"x"`
	Y float64 `json:"y"`
	Z float64 `json:"z"`
}

// ↗️Vector represents a 3D vector with X, Y and Z components.
type Vector struct {
	X float64 `json:"x"`
	Y float64 `json:"y"`
	Z float64 `json:"z"`
}

// ◻️Plane represents a 3D plane defined by origin, X-axis and Y-axis.
type Plane struct {
	Origin Point  `json:"origin"`
	XAxis  Vector `json:"xAxis"`
	YAxis  Vector `json:"yAxis"`
}

// 🎥Camera represents a 3D camera with position, forward and up vectors.
type Camera struct {
	Position Point  `json:"position"`
	Forward  Vector `json:"forward"`
	Up       Vector `json:"up"`
}

// #endregion 🖥️Weak Entities

// #region 💎Attribute

// 💎Attribute represents a key-value metadata entry with optional definition.
type Attribute struct {
	Id       string  `json:"id"`
	Key        string  `json:"key"`
	Value      *string `json:"value,omitempty"`
	Definition *string `json:"definition,omitempty"`
}

// ✏️AttributeDiff represents a partial update to an attribute's key, value or definition.
type AttributeDiff struct {
	Key        *string `json:"key,omitempty"`
	Value      *string `json:"value,omitempty"`
	Definition *string `json:"definition,omitempty"`
}

// 🗂️AttributesDiff represents batched attribute additions, removals and per-attribute updates.
type AttributesDiff struct {
	Removed []AttributeId `json:"removed,omitempty"`
	Updated []struct {
		Attribute AttributeId   `json:"attribute"`
		Diff      AttributeDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Attribute `json:"added,omitempty"`
}

// 📇AttributeMeta represents the scalar-only view of an attribute excluding nested arrays.
type AttributeMeta struct {
	Id       string  `json:"id"`
	Key        string  `json:"key"`
	Value      *string `json:"value,omitempty"`
	Definition *string `json:"definition,omitempty"`
}

// #endregion 💎Attribute

// #region 📍Location

// 📍Location represents a geographic point with longitude, latitude and optional altitude.
type Location struct {
	Id       string      `json:"id"`
	Longitude  float64     `json:"longitude"`
	Latitude   float64     `json:"latitude"`
	Altitude   *float64    `json:"altitude,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
}

// 🗺️LocationDiff represents a partial update to a location's coordinates, altitude or attributes.
type LocationDiff struct {
	Longitude  *float64        `json:"longitude,omitempty"`
	Latitude   *float64        `json:"latitude,omitempty"`
	Altitude   *float64        `json:"altitude,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// #endregion 📍Location

// #region ✍️Author

// ✍️Author represents a named contributor with optional email and timestamps.
type Author struct {
	Id       string      `json:"id"`
	Name       string      `json:"name"`
	Email      *string     `json:"email,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
	CreatedAt  string      `json:"createdAt,omitempty"`
	UpdatedAt  string      `json:"updatedAt,omitempty"`
}

// 🖊️AuthorDiff represents a partial update to an author's name, email or attributes.
type AuthorDiff struct {
	Name       *string         `json:"name,omitempty"`
	Email      *string         `json:"email,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// 👪AuthorsDiff represents batched author additions, removals and per-author updates.
type AuthorsDiff struct {
	Removed []AuthorId `json:"removed,omitempty"`
	Updated []struct {
		Author AuthorId   `json:"author"`
		Diff   AuthorDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Author `json:"added,omitempty"`
}

// 👤AuthorMeta represents the scalar-only view of an author excluding the attributes array.
type AuthorMeta struct {
	Id      string  `json:"id"`
	Name      string  `json:"name"`
	Email     *string `json:"email,omitempty"`
	CreatedAt string  `json:"createdAt,omitempty"`
	UpdatedAt string  `json:"updatedAt,omitempty"`
}

// #endregion ✍️Author

// #region 📄File

// 📄File represents a file reference entity with name, remote URL and metadata.
type File struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Remote      *string     `json:"remote,omitempty"`
	Folder      *FolderId   `json:"folder,omitempty"`
	Size        *int64      `json:"size,omitempty"`
	Hash        *string     `json:"hash,omitempty"`
	Blob        *string     `json:"blob,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 📝FileDiff represents a partial update to a file's name, URL, hash, blob or description.
type FileDiff struct {
	Name        *string         `json:"name,omitempty"`
	Remote      *string         `json:"remote,omitempty"`
	Size        *int64          `json:"size,omitempty"`
	Hash        *string         `json:"hash,omitempty"`
	Blob        *string         `json:"blob,omitempty"`
	Description *string         `json:"description,omitempty"`
	Folder      *FolderId       `json:"folder,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 📑FilesDiff represents batched file additions, removals and per-file updates.
type FilesDiff struct {
	Removed []FileId `json:"removed,omitempty"`
	Updated []struct {
		File FileId   `json:"file"`
		Diff FileDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []File `json:"added,omitempty"`
}

// 🗒️FileMeta represents the scalar-only view of a file excluding blob data and attributes.
type FileMeta struct {
	Id        string    `json:"id"`
	Name        string    `json:"name"`
	Remote      *string   `json:"remote,omitempty"`
	Folder      *FolderId `json:"folder,omitempty"`
	Size        *int64    `json:"size,omitempty"`
	Hash        *string   `json:"hash,omitempty"`
	Description *string   `json:"description,omitempty"`
	CreatedAt   string    `json:"createdAt,omitempty"`
	UpdatedAt   string    `json:"updatedAt,omitempty"`
}

// #endregion 📄File

// #region 📁Folder

// 📁Folder represents a folder hierarchy entity with name and parent reference.
type Folder struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Parent      *FolderId   `json:"parent,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 📂FolderDiff represents a partial update to a folder's name, parent or description.
type FolderDiff struct {
	Name        *string         `json:"name,omitempty"`
	Parent      *FolderId       `json:"parent,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 🗄️FoldersDiff represents batched folder additions, removals and per-folder updates.
type FoldersDiff struct {
	Removed []FolderId `json:"removed,omitempty"`
	Updated []struct {
		Folder FolderId   `json:"folder"`
		Diff   FolderDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Folder `json:"added,omitempty"`
}

// 🏠FolderMeta represents the scalar-only view of a folder excluding attributes.
type FolderMeta struct {
	Id        string    `json:"id"`
	Name        string    `json:"name"`
	Parent      *FolderId `json:"parent,omitempty"`
	Description *string   `json:"description,omitempty"`
	CreatedAt   string    `json:"createdAt,omitempty"`
	UpdatedAt   string    `json:"updatedAt,omitempty"`
}

// #endregion 📁Folder

// #region 📏Benchmark

// 📏Benchmark represents a named metric range with min/max bounds and optional icon.
type Benchmark struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Icon        *string     `json:"icon,omitempty"`
	Min         *float64    `json:"min,omitempty"`
	MinExcluded *bool       `json:"minExcluded,omitempty"`
	Max         *float64    `json:"max,omitempty"`
	MaxExcluded *bool       `json:"maxExcluded,omitempty"`
	Definition  *string     `json:"definition,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// ↕️BenchmarkDiff represents a partial update to a benchmark's name, bounds, icon or definition.
type BenchmarkDiff struct {
	Name        *string         `json:"name,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Min         *float64        `json:"min,omitempty"`
	MinExcluded *bool           `json:"minExcluded,omitempty"`
	Max         *float64        `json:"max,omitempty"`
	MaxExcluded *bool           `json:"maxExcluded,omitempty"`
	Definition  *string         `json:"definition,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 🧮BenchmarksDiff represents batched benchmark additions, removals and per-benchmark updates.
type BenchmarksDiff struct {
	Removed []BenchmarkId `json:"removed,omitempty"`
	Updated []struct {
		Benchmark BenchmarkId   `json:"benchmark"`
		Diff      BenchmarkDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Benchmark `json:"added,omitempty"`
}

// #endregion 📏Benchmark

// #region 🔬Quality

// 🔭QualityKind is a bitfield enum for quality scope classification.
type QualityKind int

const (
	QualityKindGeneral QualityKind = 1 << iota
	QualityKindType
	QualityKindDesign
	QualityKindPiece
	QualityKindConnection
	QualityKindConnector
)

// 🔬Quality represents a measurable property with formula, units and benchmarks.
type Quality struct {
	Id                string      `json:"id"`
	Key                 string      `json:"key"`
	Name                string      `json:"name"`
	Description         *string     `json:"description,omitempty"`
	Uri                 *string     `json:"uri,omitempty"`
	Kind                QualityKind `json:"kind,omitempty"`
	CanScale            *bool       `json:"canScale,omitempty"`
	DefaultSiUnit       *string     `json:"defaultSiUnit,omitempty"`
	DefaultImperialUnit *string     `json:"defaultImperialUnit,omitempty"`
	Min                 *float64    `json:"min,omitempty"`
	IsMinExcluded       *bool       `json:"isMinExcluded,omitempty"`
	Max                 *float64    `json:"max,omitempty"`
	IsMaxExcluded       *bool       `json:"isMaxExcluded,omitempty"`
	DefaultValue        *float64    `json:"defaultValue,omitempty"`
	Formula             *string     `json:"formula,omitempty"`
	Icon                *string     `json:"icon,omitempty"`
	Image               *string     `json:"image,omitempty"`
	Unit                *string     `json:"unit,omitempty"`
	Benchmarks          []Benchmark `json:"benchmarks,omitempty"`
	Attributes          []Attribute `json:"attributes,omitempty"`
	CreatedAt           string      `json:"createdAt,omitempty"`
	UpdatedAt           string      `json:"updatedAt,omitempty"`
}

// 🧪QualityDiff represents a partial update to a quality's formula, bounds, units or benchmarks.
type QualityDiff struct {
	Key                 *string         `json:"key,omitempty"`
	Name                *string         `json:"name,omitempty"`
	Description         *string         `json:"description,omitempty"`
	Uri                 *string         `json:"uri,omitempty"`
	Kind                *QualityKind    `json:"kind,omitempty"`
	CanScale            *bool           `json:"canScale,omitempty"`
	DefaultSiUnit       *string         `json:"defaultSiUnit,omitempty"`
	DefaultImperialUnit *string         `json:"defaultImperialUnit,omitempty"`
	Min                 *float64        `json:"min,omitempty"`
	IsMinExcluded       *bool           `json:"isMinExcluded,omitempty"`
	Max                 *float64        `json:"max,omitempty"`
	IsMaxExcluded       *bool           `json:"isMaxExcluded,omitempty"`
	DefaultValue        *float64        `json:"defaultValue,omitempty"`
	Formula             *string         `json:"formula,omitempty"`
	Icon                *string         `json:"icon,omitempty"`
	Image               *string         `json:"image,omitempty"`
	Unit                *string         `json:"unit,omitempty"`
	Benchmarks          *BenchmarksDiff `json:"benchmarks,omitempty"`
	Attributes          *AttributesDiff `json:"attributes,omitempty"`
}

// ⚗️QualitiesDiff represents batched quality additions, removals and per-quality updates.
type QualitiesDiff struct {
	Removed []QualityId `json:"removed,omitempty"`
	Updated []struct {
		Quality QualityId   `json:"quality"`
		Diff    QualityDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Quality `json:"added,omitempty"`
}

// 🔎QualityMeta represents the scalar-only view of a quality excluding benchmarks and attributes.
type QualityMeta struct {
	Id                string      `json:"id"`
	Key                 string      `json:"key"`
	Name                string      `json:"name"`
	Description         *string     `json:"description,omitempty"`
	Uri                 *string     `json:"uri,omitempty"`
	Kind                QualityKind `json:"kind,omitempty"`
	CanScale            *bool       `json:"canScale,omitempty"`
	DefaultSiUnit       *string     `json:"defaultSiUnit,omitempty"`
	DefaultImperialUnit *string     `json:"defaultImperialUnit,omitempty"`
	Min                 *float64    `json:"min,omitempty"`
	IsMinExcluded       *bool       `json:"isMinExcluded,omitempty"`
	Max                 *float64    `json:"max,omitempty"`
	IsMaxExcluded       *bool       `json:"isMaxExcluded,omitempty"`
	DefaultValue        *float64    `json:"defaultValue,omitempty"`
	Formula             *string     `json:"formula,omitempty"`
	Icon                *string     `json:"icon,omitempty"`
	Image               *string     `json:"image,omitempty"`
	Unit                *string     `json:"unit,omitempty"`
	CreatedAt           string      `json:"createdAt,omitempty"`
	UpdatedAt           string      `json:"updatedAt,omitempty"`
}

// #endregion 🔬Quality

// #region ⚓Port

// ⚓Port represents a named connector port category with compatible port references.
type Port struct {
	Id            string      `json:"id"`
	Name            string      `json:"name"`
	Description     *string     `json:"description,omitempty"`
	Icon            *string     `json:"icon,omitempty"`
	MaxChildren     *int        `json:"maxChildren,omitempty"`
	CompatiblePorts []PortId    `json:"compatiblePorts,omitempty"`
	Attributes      []Attribute `json:"attributes,omitempty"`
	CreatedAt       string      `json:"createdAt,omitempty"`
	UpdatedAt       string      `json:"updatedAt,omitempty"`
}

// ⚙️PortDiff represents a partial update to a port's name, icon or compatible ports.
type PortDiff struct {
	Name            *string         `json:"name,omitempty"`
	Description     *string         `json:"description,omitempty"`
	Icon            *string         `json:"icon,omitempty"`
	MaxChildren     *int            `json:"maxChildren,omitempty"`
	CompatiblePorts []PortId        `json:"compatiblePorts,omitempty"`
	Attributes      *AttributesDiff `json:"attributes,omitempty"`
	setFields       map[string]bool `json:"-"`
}

// 📥UnmarshalJSON deserializes PortDiff JSON while tracking which fields are explicitly set.
func (d *PortDiff) UnmarshalJSON(data []byte) error {
	type Alias PortDiff
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(d),
	}
	var rawMap map[string]json.RawMessage
	if err := json.Unmarshal(data, &rawMap); err != nil {
		return err
	}
	d.setFields = make(map[string]bool)
	for key := range rawMap {
		d.setFields[key] = true
	}
	return json.Unmarshal(data, aux)
}

// 🔍HasField checks whether a specific JSON field was present during PortDiff deserialization.
func (d *PortDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// ⛵PortsDiff represents batched port additions, removals and per-port updates.
type PortsDiff struct {
	Removed []PortId `json:"removed,omitempty"`
	Updated []struct {
		Port PortId   `json:"port"`
		Diff PortDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Port `json:"added,omitempty"`
}

// 🪝PortMeta represents the scalar-only view of a port excluding compatible ports and attributes.
type PortMeta struct {
	Id        string  `json:"id"`
	Name        string  `json:"name"`
	Description *string `json:"description,omitempty"`
	Icon        *string `json:"icon,omitempty"`
	MaxChildren *int    `json:"maxChildren,omitempty"`
	CreatedAt   string  `json:"createdAt,omitempty"`
	UpdatedAt   string  `json:"updatedAt,omitempty"`
}

// #endregion ⚓Port

// #region 👪Family

// 👪Family represents a composable artifact family that owns its connector ports.
type Family struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Description *string     `json:"description,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Ports       []Port      `json:"ports,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 🧬FamilyDiff represents a partial update to a family and its ports.
type FamilyDiff struct {
	Name        *string         `json:"name,omitempty"`
	Description *string         `json:"description,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Ports       *PortsDiff      `json:"ports,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// 📩UnmarshalJSON deserializes FamilyDiff JSON while tracking explicit fields.
func (d *FamilyDiff) UnmarshalJSON(data []byte) error {
	type Alias FamilyDiff
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(d),
	}
	var rawMap map[string]json.RawMessage
	if err := json.Unmarshal(data, &rawMap); err != nil {
		return err
	}
	d.setFields = make(map[string]bool)
	for key := range rawMap {
		d.setFields[key] = true
	}
	return json.Unmarshal(data, aux)
}

// 🔎HasField checks whether a family diff field was present in JSON.
func (d *FamilyDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// 👨‍👩‍👧FamiliesDiff represents batched family additions, removals and updates.
type FamiliesDiff struct {
	Removed []FamilyId `json:"removed,omitempty"`
	Updated []struct {
		Family FamilyId   `json:"family"`
		Diff   FamilyDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Family `json:"added,omitempty"`
}

// 🪪FamilyMeta represents the scalar-only view of a family.
type FamilyMeta struct {
	Id        string  `json:"id"`
	Name        string  `json:"name"`
	Description *string `json:"description,omitempty"`
	Icon        *string `json:"icon,omitempty"`
	CreatedAt   string  `json:"createdAt,omitempty"`
	UpdatedAt   string  `json:"updatedAt,omitempty"`
}

// 🧾FamilyShallow represents a family including its port metadata.
type FamilyShallow struct {
	Id        string     `json:"id"`
	Name        string     `json:"name"`
	Description *string    `json:"description,omitempty"`
	Icon        *string    `json:"icon,omitempty"`
	Ports       []PortMeta `json:"ports,omitempty"`
	CreatedAt   string     `json:"createdAt,omitempty"`
	UpdatedAt   string     `json:"updatedAt,omitempty"`
}

// #endregion 👪Family

// #region 🏛️Typology

// 🏛️Typology groups types and designs under one kit partition; families stay at kit root.
type Typology struct {
	Id          string      `json:"id"`
	Name        string      `json:"name"`
	Description *string     `json:"description,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Folder      *string     `json:"folder,omitempty"`
	Types       []Type      `json:"types,omitempty"`
	Designs     []Design    `json:"designs,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 🧬TypologyDiff represents a partial update to a typology and its owned types and designs.
type TypologyDiff struct {
	Name        *string         `json:"name,omitempty"`
	Description *string         `json:"description,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Folder      *string         `json:"folder,omitempty"`
	Types       *TypesDiff      `json:"types,omitempty"`
	Designs     *DesignsDiff    `json:"designs,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// 📩UnmarshalJSON deserializes TypologyDiff JSON while tracking explicit fields.
func (d *TypologyDiff) UnmarshalJSON(data []byte) error {
	type Alias TypologyDiff
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(d),
	}
	var rawMap map[string]json.RawMessage
	if err := json.Unmarshal(data, &rawMap); err != nil {
		return err
	}
	d.setFields = make(map[string]bool)
	for key := range rawMap {
		d.setFields[key] = true
	}
	return json.Unmarshal(data, aux)
}

// 🔎HasField checks whether a typology diff field was present in JSON.
func (d *TypologyDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// 🏛️TypologiesDiff represents batched typology additions, removals and updates.
type TypologiesDiff struct {
	Removed []TypologyId `json:"removed,omitempty"`
	Updated []struct {
		Typology TypologyId   `json:"typology"`
		Diff     TypologyDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Typology `json:"added,omitempty"`
}

// 🪪TypologyMeta represents the scalar-only view of a typology.
type TypologyMeta struct {
	Id          string  `json:"id"`
	Name        string  `json:"name"`
	Description *string `json:"description,omitempty"`
	Icon        *string `json:"icon,omitempty"`
	Folder      *string `json:"folder,omitempty"`
	CreatedAt   string  `json:"createdAt,omitempty"`
	UpdatedAt   string  `json:"updatedAt,omitempty"`
}

// 🧾TypologyShallow represents a typology including type and design metadata.
type TypologyShallow struct {
	Id          string         `json:"id"`
	Name        string         `json:"name"`
	Description *string        `json:"description,omitempty"`
	Icon        *string        `json:"icon,omitempty"`
	Folder      *string        `json:"folder,omitempty"`
	Types       []TypeMeta     `json:"types,omitempty"`
	Designs     []DesignMeta   `json:"designs,omitempty"`
	CreatedAt   string         `json:"createdAt,omitempty"`
	UpdatedAt   string         `json:"updatedAt,omitempty"`
}

// #endregion 🏛️Typology

// #region 📊Prop

// 📊Prop represents a quality measurement value with optional unit.
type Prop struct {
	Id       string      `json:"id"`
	Quality    QualityId   `json:"quality"`
	Value      string      `json:"value"`
	Unit       *string     `json:"unit,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
}

// 🔧PropDiff represents a partial update to a prop's quality reference, value or unit.
type PropDiff struct {
	Quality    *QualityId      `json:"quality,omitempty"`
	Value      *string         `json:"value,omitempty"`
	Unit       *string         `json:"unit,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// 📜PropsDiff represents batched prop additions, removals and per-prop updates.
type PropsDiff struct {
	Removed []PropId `json:"removed,omitempty"`
	Updated []struct {
		Prop PropId   `json:"prop"`
		Diff PropDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Prop `json:"added,omitempty"`
}

// 🏺PropMeta represents the scalar-only view of a prop excluding attributes.
type PropMeta struct {
	Id    string    `json:"id"`
	Quality QualityId `json:"quality"`
	Value   string    `json:"value"`
	Unit    *string   `json:"unit,omitempty"`
}

// #endregion 📊Prop

// #region 🏷️Tag

// 🏷️Tag represents a named classification label with optional description and icon.
type Tag struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Description *string     `json:"description,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 🏳️TagDiff represents a partial update to a tag's name, description or icon.
type TagDiff struct {
	Name        *string         `json:"name,omitempty"`
	Description *string         `json:"description,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// 📩UnmarshalJSON deserializes TagDiff JSON while tracking which fields are explicitly set.
func (d *TagDiff) UnmarshalJSON(data []byte) error {
	type Alias TagDiff
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(d),
	}
	var rawMap map[string]json.RawMessage
	if err := json.Unmarshal(data, &rawMap); err != nil {
		return err
	}
	d.setFields = make(map[string]bool)
	for key := range rawMap {
		d.setFields[key] = true
	}
	return json.Unmarshal(data, aux)
}

// 🕵️HasField checks whether a specific JSON field was present during TagDiff deserialization.
func (d *TagDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// 🏴TagsDiff represents batched tag additions, removals and per-tag updates.
type TagsDiff struct {
	Removed []TagId `json:"removed,omitempty"`
	Updated []struct {
		Tag  TagId   `json:"tag"`
		Diff TagDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Tag `json:"added,omitempty"`
}

// 🎗️TagMeta represents the scalar-only view of a tag excluding attributes.
type TagMeta struct {
	Id        string  `json:"id"`
	Name        string  `json:"name"`
	Description *string `json:"description,omitempty"`
	Icon        *string `json:"icon,omitempty"`
	CreatedAt   string  `json:"createdAt,omitempty"`
	UpdatedAt   string  `json:"updatedAt,omitempty"`
}

// #endregion 🏷️Tag

// #region 💡Concept

// 💡Concept represents a named categorization concept with optional description and icon.
type Concept struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Description *string     `json:"description,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 🔮ConceptDiff represents a partial update to a concept's name, description or icon.
type ConceptDiff struct {
	Name        *string         `json:"name,omitempty"`
	Description *string         `json:"description,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// 📨UnmarshalJSON deserializes ConceptDiff JSON while tracking which fields are explicitly set.
func (d *ConceptDiff) UnmarshalJSON(data []byte) error {
	type Alias ConceptDiff
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(d),
	}
	var rawMap map[string]json.RawMessage
	if err := json.Unmarshal(data, &rawMap); err != nil {
		return err
	}
	d.setFields = make(map[string]bool)
	for key := range rawMap {
		d.setFields[key] = true
	}
	return json.Unmarshal(data, aux)
}

// 👁️HasField checks whether a specific JSON field was present during ConceptDiff deserialization.
func (d *ConceptDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// 💭ConceptsDiff represents batched concept additions, removals and per-concept updates.
type ConceptsDiff struct {
	Removed []ConceptId `json:"removed,omitempty"`
	Updated []struct {
		Concept ConceptId   `json:"concept"`
		Diff    ConceptDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Concept `json:"added,omitempty"`
}

// 🧠ConceptMeta represents the scalar-only view of a concept excluding attributes.
type ConceptMeta struct {
	Id        string  `json:"id"`
	Name        string  `json:"name"`
	Description *string `json:"description,omitempty"`
	Icon        *string `json:"icon,omitempty"`
	CreatedAt   string  `json:"createdAt,omitempty"`
	UpdatedAt   string  `json:"updatedAt,omitempty"`
}

// #endregion 💡Concept

// #region 🗿Representation

// 🗿Representation represents a 3D representation reference linking a file with tags and description.
type Representation struct {
	Id        string      `json:"id"`
	File        FileId      `json:"file"`
	Name        *string     `json:"name,omitempty"`
	Tags        []TagId     `json:"tags,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// 🖼️RepresentationDiff represents a partial update to a representation's file, name, tags or description.
type RepresentationDiff struct {
	File        *FileId         `json:"file,omitempty"`
	Name        *string         `json:"name,omitempty"`
	Tags        []TagId         `json:"tags,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 🎴RepresentationsDiff represents batched representation additions, removals and per-representation updates.
type RepresentationsDiff struct {
	Removed []RepresentationId `json:"removed,omitempty"`
	Updated []struct {
		Representation RepresentationId   `json:"representation"`
		Diff  RepresentationDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Representation `json:"added,omitempty"`
}

// 🎭RepresentationMeta represents the scalar-only view of a representation excluding tags and attributes.
type RepresentationMeta struct {
	Id        string  `json:"id"`
	File        FileId  `json:"file"`
	Name        *string `json:"name,omitempty"`
	Description *string `json:"description,omitempty"`
}

// #endregion 🗿Representation

// #region 🔌Connector

// 🔌Connector represents a spatial connection point on a type with position and direction.
type Connector struct {
	Id        string      `json:"id"`
	Name        *string     `json:"name,omitempty"`
	Point       Point       `json:"point"`
	Direction   Vector      `json:"direction"`
	T           float64     `json:"t"`
	Mandatory   *bool       `json:"mandatory,omitempty"`
	MaxChildren *int        `json:"maxChildren,omitempty"`
	Port        *PortId     `json:"port,omitempty"`
	Props       []Prop      `json:"props,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// 📌PointDiff represents a partial update to a 3D point's X, Y or Z coordinate.
type PointDiff struct {
	X *float64 `json:"x,omitempty"`
	Y *float64 `json:"y,omitempty"`
	Z *float64 `json:"z,omitempty"`
}

// 🧭VectorDiff represents a partial update to a 3D vector's X, Y or Z component.
type VectorDiff struct {
	X *float64 `json:"x,omitempty"`
	Y *float64 `json:"y,omitempty"`
	Z *float64 `json:"z,omitempty"`
}

// 🛠️ConnectorDiff represents a partial update to a connector's position, direction, port or props.
type ConnectorDiff struct {
	Name        *string         `json:"name,omitempty"`
	Point       *PointDiff      `json:"point,omitempty"`
	Direction   *VectorDiff     `json:"direction,omitempty"`
	T           *float64        `json:"t,omitempty"`
	Mandatory   *bool           `json:"mandatory,omitempty"`
	MaxChildren *int            `json:"maxChildren,omitempty"`
	Port        *PortId         `json:"port,omitempty"`
	Props       *PropsDiff      `json:"props,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 🔋ConnectorsDiff represents batched connector additions, removals and per-connector updates.
type ConnectorsDiff struct {
	Removed []ConnectorId `json:"removed,omitempty"`
	Updated []struct {
		Connector ConnectorId   `json:"connector"`
		Diff      ConnectorDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Connector `json:"added,omitempty"`
}

// 📎ConnectorMeta represents the scalar-only view of a connector excluding props and attributes.
type ConnectorMeta struct {
	Id        string  `json:"id"`
	Name        *string `json:"name,omitempty"`
	Point       Point   `json:"point"`
	Direction   Vector  `json:"direction"`
	T           float64 `json:"t"`
	Mandatory   *bool   `json:"mandatory,omitempty"`
	MaxChildren *int    `json:"maxChildren,omitempty"`
	Port        *PortId `json:"port,omitempty"`
	Description *string `json:"description,omitempty"`
}

// #endregion 🔌Connector

// #region 🧱Type

// 🧱Type represents a component blueprint with representations, connectors and hierarchical inheritance.
type Type struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Typology    TypologyId  `json:"typology"`
	Families    []FamilyId  `json:"families,omitempty"`
	IsAbstract  *bool       `json:"isAbstract,omitempty"`
	Virtual     *bool       `json:"virtual,omitempty"`
	Unit        *string     `json:"unit,omitempty"`
	Stock       *int        `json:"stock,omitempty"`
	Location    *LocationId `json:"location,omitempty"`
	Folder      *string     `json:"folder,omitempty"`
	Representations      []Representation     `json:"representations,omitempty"`
	Connectors  []Connector `json:"connectors,omitempty"`
	Props       []Prop      `json:"props,omitempty"`
	Authors     []AuthorId  `json:"authors,omitempty"`
	Concepts    []ConceptId `json:"concepts,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Image       *string     `json:"image,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// ⚒️TypeDiff represents a partial update to a type's name, representations, connectors or props.
type TypeDiff struct {
	Name        *string         `json:"name,omitempty"`
	Typology    *TypologyId     `json:"typology,omitempty"`
	Families    []FamilyId      `json:"families,omitempty"`
	IsAbstract  *bool           `json:"isAbstract,omitempty"`
	Virtual     *bool           `json:"virtual,omitempty"`
	Unit        *string         `json:"unit,omitempty"`
	Stock       *int            `json:"stock,omitempty"`
	Location    *LocationId     `json:"location,omitempty"`
	Folder      *string         `json:"folder,omitempty"`
	Representations      *RepresentationsDiff     `json:"representations,omitempty"`
	Connectors  *ConnectorsDiff `json:"connectors,omitempty"`
	Props       *PropsDiff      `json:"props,omitempty"`
	Authors     []AuthorId      `json:"authors,omitempty"`
	Concepts    []ConceptId     `json:"concepts,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Image       *string         `json:"image,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// 📬UnmarshalJSON deserializes TypeDiff JSON while tracking which fields are explicitly set.
func (d *TypeDiff) UnmarshalJSON(data []byte) error {
	type Alias TypeDiff
	aux := &struct {
		*Alias
	}{
		Alias: (*Alias)(d),
	}
	var rawMap map[string]json.RawMessage
	if err := json.Unmarshal(data, &rawMap); err != nil {
		return err
	}
	d.setFields = make(map[string]bool)
	for key := range rawMap {
		d.setFields[key] = true
	}
	return json.Unmarshal(data, aux)
}

// 🔦HasField checks whether a specific JSON field was present during TypeDiff deserialization.
func (d *TypeDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// 🏗️TypesDiff represents batched type additions, removals and per-type updates.
type TypesDiff struct {
	Removed []TypeId `json:"removed,omitempty"`
	Updated []struct {
		Type TypeId   `json:"type"`
		Diff TypeDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Type `json:"added,omitempty"`
}

// 🧊TypeMeta represents the scalar-only view of a type excluding representations, connectors, props and attributes.
type TypeMeta struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Typology    TypologyId  `json:"typology"`
	Families    []FamilyId  `json:"families,omitempty"`
	IsAbstract  *bool       `json:"isAbstract,omitempty"`
	Virtual     *bool       `json:"virtual,omitempty"`
	Unit        *string     `json:"unit,omitempty"`
	Stock       *int        `json:"stock,omitempty"`
	Location    *LocationId `json:"location,omitempty"`
	Folder      *string     `json:"folder,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Image       *string     `json:"image,omitempty"`
	Description *string     `json:"description,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 🔖TypeShallow represents a Type with slice fields replaced by Meta item slices.
type TypeShallow struct {
	Id        string          `json:"id"`
	Name        string          `json:"name"`
	Typology    TypologyId      `json:"typology"`
	Families    []FamilyId      `json:"families,omitempty"`
	IsAbstract  *bool           `json:"isAbstract,omitempty"`
	Virtual     *bool           `json:"virtual,omitempty"`
	Unit        *string         `json:"unit,omitempty"`
	Stock       *int            `json:"stock,omitempty"`
	Location    *LocationId     `json:"location,omitempty"`
	Folder      *string         `json:"folder,omitempty"`
	Representations      []RepresentationMeta     `json:"representations,omitempty"`
	Connectors  []ConnectorMeta `json:"connectors,omitempty"`
	Props       []PropMeta      `json:"props,omitempty"`
	Authors     []AuthorId      `json:"authors,omitempty"`
	Concepts    []ConceptId     `json:"concepts,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Image       *string         `json:"image,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  []AttributeMeta `json:"attributes,omitempty"`
	CreatedAt   string          `json:"createdAt,omitempty"`
	UpdatedAt   string          `json:"updatedAt,omitempty"`
}

// #endregion 🧱Type

// #region 🎨Layer

// 🎨Layer represents a named layer with visibility, lock and color properties.
type Layer struct {
	Id        string      `json:"id"`
	Path        string      `json:"path"`
	IsHidden    *bool       `json:"isHidden,omitempty"`
	IsLocked    *bool       `json:"isLocked,omitempty"`
	Color       *string     `json:"color,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// 🖌️LayerDiff represents a partial update to a layer's path, visibility, lock state or color.
type LayerDiff struct {
	Path        *string         `json:"path,omitempty"`
	IsHidden    *bool           `json:"isHidden,omitempty"`
	IsLocked    *bool           `json:"isLocked,omitempty"`
	Color       *string         `json:"color,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 🖍️LayersDiff represents batched layer additions, removals and per-layer updates.
type LayersDiff struct {
	Removed []LayerId `json:"removed,omitempty"`
	Updated []struct {
		Layer LayerId   `json:"layer"`
		Diff  LayerDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Layer `json:"added,omitempty"`
}

// 🪟LayerMeta represents the scalar-only view of a layer excluding attributes.
type LayerMeta struct {
	Id        string  `json:"id"`
	Path        string  `json:"path"`
	IsHidden    *bool   `json:"isHidden,omitempty"`
	IsLocked    *bool   `json:"isLocked,omitempty"`
	Color       *string `json:"color,omitempty"`
	Description *string `json:"description,omitempty"`
}

// #endregion 🎨Layer

// #region 🧩Piece

// 🧩Piece represents a positioned component instance within a design with optional transform.
type Piece struct {
	Id        string      `json:"id"`
	Name        *string     `json:"name,omitempty"`
	Type        *TypeId     `json:"type,omitempty"`
	Design      *DesignId   `json:"design,omitempty"`
	Plane       *Plane      `json:"plane,omitempty"`
	Center      *Coordinate      `json:"center,omitempty"`
	Scale       *float64    `json:"scale,omitempty"`
	MirrorPlane *Plane      `json:"mirrorPlane,omitempty"`
	Props       []Prop      `json:"props,omitempty"`
	IsHidden    *bool       `json:"isHidden,omitempty"`
	IsLocked    *bool       `json:"isLocked,omitempty"`
	Color       *string     `json:"color,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// 🎯CoordinateDiff represents a partial update to a 2D coordinate's U or V value.
type CoordinateDiff struct {
	U *float64 `json:"u,omitempty"`
	V *float64 `json:"v,omitempty"`
}

// 🔲PlaneDiff represents a partial update to a plane's origin, X-axis or Y-axis.
type PlaneDiff struct {
	Origin *PointDiff  `json:"origin,omitempty"`
	XAxis  *VectorDiff `json:"xAxis,omitempty"`
	YAxis  *VectorDiff `json:"yAxis,omitempty"`
}

// 🔩PieceDiff represents a partial update to a piece's type, plane, scale, center or props.
type PieceDiff struct {
	Name        *string         `json:"name,omitempty"`
	Type        *TypeId         `json:"type,omitempty"`
	Design      *DesignId       `json:"design,omitempty"`
	Plane       *PlaneDiff      `json:"plane,omitempty"`
	Center      *CoordinateDiff      `json:"center,omitempty"`
	Scale       *float64        `json:"scale,omitempty"`
	MirrorPlane *PlaneDiff      `json:"mirrorPlane,omitempty"`
	Props       *PropsDiff      `json:"props,omitempty"`
	IsHidden    *bool           `json:"isHidden,omitempty"`
	IsLocked    *bool           `json:"isLocked,omitempty"`
	Color       *string         `json:"color,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 🎪PiecesDiff represents batched piece additions, removals and per-piece updates.
type PiecesDiff struct {
	Removed []PieceId `json:"removed,omitempty"`
	Updated []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Piece `json:"added,omitempty"`
}

// 🧲PieceMeta represents the scalar-only view of a piece excluding props and attributes.
type PieceMeta struct {
	Id        string    `json:"id"`
	Name        *string   `json:"name,omitempty"`
	Type        *TypeId   `json:"type,omitempty"`
	Design      *DesignId `json:"design,omitempty"`
	Plane       *Plane    `json:"plane,omitempty"`
	Center      *Coordinate    `json:"center,omitempty"`
	Scale       *float64  `json:"scale,omitempty"`
	MirrorPlane *Plane    `json:"mirrorPlane,omitempty"`
	IsHidden    *bool     `json:"isHidden,omitempty"`
	IsLocked    *bool     `json:"isLocked,omitempty"`
	Color       *string   `json:"color,omitempty"`
	Description *string   `json:"description,omitempty"`
}

// #endregion 🧩Piece

// #region 👥Group

// 👥Group represents a named collection of pieces within a design.
type Group struct {
	Id        string      `json:"id"`
	Pieces      []PieceId   `json:"pieces,omitempty"`
	Name        *string     `json:"name,omitempty"`
	Color       *string     `json:"color,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// 🤝GroupDiff represents a partial update to a group's pieces, name or color.
type GroupDiff struct {
	Pieces      []PieceId       `json:"pieces,omitempty"`
	Name        *string         `json:"name,omitempty"`
	Color       *string         `json:"color,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 🏘️GroupsDiff represents batched group additions, removals and per-group updates.
type GroupsDiff struct {
	Removed []GroupId `json:"removed,omitempty"`
	Updated []struct {
		Group GroupId   `json:"group"`
		Diff  GroupDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Group `json:"added,omitempty"`
}

// 🗃️GroupMeta represents the scalar-only view of a group excluding pieces and attributes.
type GroupMeta struct {
	Id        string  `json:"id"`
	Name        *string `json:"name,omitempty"`
	Color       *string `json:"color,omitempty"`
	Description *string `json:"description,omitempty"`
}

// #endregion 👥Group

// #region ↔️Side

// ↔️Side represents one end of a connection referencing a piece and optional connector.
type Side struct {
	Piece       PieceId      `json:"piece"`
	DesignPiece *PieceId     `json:"designPiece,omitempty"`
	Connector   *ConnectorId `json:"connector,omitempty"`
}

// ↩️SideDiff represents a partial update to a side's piece or connector reference.
type SideDiff struct {
	Piece       *PieceId     `json:"piece,omitempty"`
	DesignPiece *PieceId     `json:"designPiece,omitempty"`
	Connector   *ConnectorId `json:"connector,omitempty"`
}

// #endregion ↔️Side

// #region 🔗Connection

// 🔗Connection represents a spatial relationship between two pieces with gap, shift and rotation.
type Connection struct {
	Id        string      `json:"id"`
	Parent    Side        `json:"parent"`
	Child     Side        `json:"child"`
	Gap         float64     `json:"gap"`
	Shift       float64     `json:"shift"`
	Rise        float64     `json:"rise"`
	Rotation    float64     `json:"rotation"`
	Turn        float64     `json:"turn"`
	Tilt        float64     `json:"tilt"`
	U           float64     `json:"u"`
	V           float64     `json:"v"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// ⛓️ConnectionDiff represents a partial update to a connection's sides, gap, shift, rotation or tilt.
type ConnectionDiff struct {
	Parent   *SideDiff       `json:"parent,omitempty"`
	Child    *SideDiff       `json:"child,omitempty"`
	Gap         *float64        `json:"gap,omitempty"`
	Shift       *float64        `json:"shift,omitempty"`
	Rise        *float64        `json:"rise,omitempty"`
	Rotation    *float64        `json:"rotation,omitempty"`
	Turn        *float64        `json:"turn,omitempty"`
	Tilt        *float64        `json:"tilt,omitempty"`
	U           *float64        `json:"u,omitempty"`
	V           *float64        `json:"v,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// 🔀ConnectionsDiff represents batched connection additions, removals and per-connection updates.
type ConnectionsDiff struct {
	Removed []ConnectionId `json:"removed,omitempty"`
	Updated []struct {
		Connection ConnectionId   `json:"connection"`
		Diff       ConnectionDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Connection `json:"added,omitempty"`
}

// 🧷ConnectionMeta represents the scalar-only view of a connection excluding attributes.
type ConnectionMeta struct {
	Id        string  `json:"id"`
	Parent    Side    `json:"parent"`
	Child     Side    `json:"child"`
	Gap         float64 `json:"gap"`
	Shift       float64 `json:"shift"`
	Rise        float64 `json:"rise"`
	Rotation    float64 `json:"rotation"`
	Turn        float64 `json:"turn"`
	Tilt        float64 `json:"tilt"`
	U           float64 `json:"u"`
	V           float64 `json:"v"`
	Description *string `json:"description,omitempty"`
}

// #endregion 🔗Connection

// #region 📈Stat

// 📈Stat represents a statistical quality measurement with min/max bounds and unit.
type Stat struct {
	Id        string      `json:"id"`
	Quality     QualityId   `json:"quality"`
	Min         *float64    `json:"min,omitempty"`
	MinExcluded *bool       `json:"minExcluded,omitempty"`
	Max         *float64    `json:"max,omitempty"`
	MaxExcluded *bool       `json:"maxExcluded,omitempty"`
	Unit        *string     `json:"unit,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// 📉StatDiff represents a partial update to a stat's quality reference, bounds or unit.
type StatDiff struct {
	Quality    *QualityId      `json:"quality,omitempty"`
	Min        *float64        `json:"min,omitempty"`
	Max        *float64        `json:"max,omitempty"`
	Unit       *string         `json:"unit,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// 📋StatsDiff represents batched stat additions, removals and per-stat updates.
type StatsDiff struct {
	Removed []StatId `json:"removed,omitempty"`
	Updated []struct {
		Stat StatId   `json:"stat"`
		Diff StatDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Stat `json:"added,omitempty"`
}

// 🪙StatMeta represents the scalar-only view of a stat excluding attributes.
type StatMeta struct {
	Id    string    `json:"id"`
	Quality QualityId `json:"quality"`
	Min     *float64  `json:"min,omitempty"`
	Max     *float64  `json:"max,omitempty"`
	Unit    *string   `json:"unit,omitempty"`
}

// #endregion 📈Stat

// #region 📐Design

// 📐Design represents an assembly of pieces, connections, layers and groups.
type Design struct {
	Id        string       `json:"id"`
	Name        string       `json:"name"`
	Typology    TypologyId   `json:"typology"`
	Families    []FamilyId   `json:"families,omitempty"`
	IsAbstract  *bool        `json:"isAbstract,omitempty"`
	Unit        *string      `json:"unit,omitempty"`
	Folder      *string      `json:"folder,omitempty"`
	CanScale    *bool        `json:"canScale,omitempty"`
	CanMirror   *bool        `json:"canMirror,omitempty"`
	View        *Camera      `json:"view,omitempty"`
	Pieces      []Piece      `json:"pieces,omitempty"`
	Connections []Connection `json:"connections,omitempty"`
	Stats       []Stat       `json:"stats,omitempty"`
	Props       []Prop       `json:"props,omitempty"`
	Layers      []Layer      `json:"layers,omitempty"`
	ActiveLayer *LayerId     `json:"activeLayer,omitempty"`
	Groups      []Group      `json:"groups,omitempty"`
	Location    *LocationId  `json:"location,omitempty"`
	Authors     []AuthorId   `json:"authors,omitempty"`
	Concepts    []ConceptId  `json:"concepts,omitempty"`
	Icon        *string      `json:"icon,omitempty"`
	Image       *string      `json:"image,omitempty"`
	Description *string      `json:"description,omitempty"`
	Attributes  []Attribute  `json:"attributes,omitempty"`
	CreatedAt   string       `json:"createdAt,omitempty"`
	UpdatedAt   string       `json:"updatedAt,omitempty"`
}

// 🎬CameraDiff represents a partial update to a camera's position, forward or up vector.
type CameraDiff struct {
	Position *PointDiff  `json:"position,omitempty"`
	Forward  *VectorDiff `json:"forward,omitempty"`
	Up       *VectorDiff `json:"up,omitempty"`
}

// ✒️DesignDiff represents a partial update to a design's name, pieces, connections or layers.
type DesignDiff struct {
	Name        *string          `json:"name,omitempty"`
	Typology    *TypologyId      `json:"typology,omitempty"`
	Families    []FamilyId       `json:"families,omitempty"`
	IsAbstract  *bool            `json:"isAbstract,omitempty"`
	Unit        *string          `json:"unit,omitempty"`
	Folder      *string          `json:"folder,omitempty"`
	CanScale    *bool            `json:"canScale,omitempty"`
	CanMirror   *bool            `json:"canMirror,omitempty"`
	View        *CameraDiff      `json:"view,omitempty"`
	Pieces      *PiecesDiff      `json:"pieces,omitempty"`
	Connections *ConnectionsDiff `json:"connections,omitempty"`
	Stats       *StatsDiff       `json:"stats,omitempty"`
	Props       *PropsDiff       `json:"props,omitempty"`
	Layers      *LayersDiff      `json:"layers,omitempty"`
	ActiveLayer *LayerId         `json:"activeLayer,omitempty"`
	Groups      *GroupsDiff      `json:"groups,omitempty"`
	Location    *LocationId      `json:"location,omitempty"`
	Authors     []AuthorId       `json:"authors,omitempty"`
	Concepts    []ConceptId      `json:"concepts,omitempty"`
	Icon        *string          `json:"icon,omitempty"`
	Image       *string          `json:"image,omitempty"`
	Description *string          `json:"description,omitempty"`
	Attributes  *AttributesDiff  `json:"attributes,omitempty"`
}

// #region 🎯ComposeReport

// 📋OperationNote is a human-readable remark on a ComposeReport (warning, info, or error).
type OperationNote struct {
	Code    string `json:"code,omitempty"`
	Message string `json:"message"`
}

// 📋ComposeReport is the canonical algorithm output: ok, diff, warnings, infos, errors (tool-friendly JSON).
type ComposeReport[T any] struct {
	Ok       bool            `json:"ok"`
	Diff     *T              `json:"diff"`
	Warnings []OperationNote `json:"warnings"`
	Infos    []OperationNote `json:"infos"`
	Errors   []OperationNote `json:"errors"`
}

func composeReportOk[T any](diff T) ComposeReport[T] {
	d := diff
	return ComposeReport[T]{
		Ok:       true,
		Diff:     &d,
		Warnings: []OperationNote{},
		Infos:    []OperationNote{},
		Errors:   []OperationNote{},
	}
}

func composeReportErr[T any](errs []OperationNote) ComposeReport[T] {
	return ComposeReport[T]{
		Ok:       false,
		Diff:     nil,
		Warnings: []OperationNote{},
		Infos:    []OperationNote{},
		Errors:   errs,
	}
}

// 📋composeReportOkWithNotes is a successful report carrying merged flatten warnings/infos.
func composeReportOkWithNotes[T any](diff T, warnings []OperationNote, infos []OperationNote) ComposeReport[T] {
	d := diff
	w := warnings
	i := infos
	if w == nil {
		w = []OperationNote{}
	}
	if i == nil {
		i = []OperationNote{}
	}
	return ComposeReport[T]{
		Ok:       true,
		Diff:     &d,
		Warnings: w,
		Infos:    i,
		Errors:   []OperationNote{},
	}
}

// #endregion 🎯ComposeReport

// 🏛️DesignsDiff represents batched design additions, removals and per-design updates.
type DesignsDiff struct {
	Removed []DesignId `json:"removed,omitempty"`
	Updated []struct {
		Design DesignId   `json:"design"`
		Diff   DesignDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Design `json:"added,omitempty"`
}

// 🏙️DesignMeta represents the scalar-only view of a design excluding pieces, connections and layers.
type DesignMeta struct {
	Id        string      `json:"id"`
	Name        string      `json:"name"`
	Typology    TypologyId  `json:"typology"`
	Families    []FamilyId  `json:"families,omitempty"`
	IsAbstract  *bool       `json:"isAbstract,omitempty"`
	Unit        *string     `json:"unit,omitempty"`
	Folder      *string     `json:"folder,omitempty"`
	CanScale    *bool       `json:"canScale,omitempty"`
	CanMirror   *bool       `json:"canMirror,omitempty"`
	View        *Camera     `json:"view,omitempty"`
	ActiveLayer *LayerId    `json:"activeLayer,omitempty"`
	Location    *LocationId `json:"location,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Image       *string     `json:"image,omitempty"`
	Description *string     `json:"description,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 📔DesignShallow represents a design overview with nested arrays replaced by scalar-only items.
type DesignShallow struct {
	Id        string           `json:"id"`
	Name        string           `json:"name"`
	Typology    TypologyId       `json:"typology"`
	Families    []FamilyId       `json:"families,omitempty"`
	IsAbstract  *bool            `json:"isAbstract,omitempty"`
	Unit        *string          `json:"unit,omitempty"`
	Folder      *string          `json:"folder,omitempty"`
	CanScale    *bool            `json:"canScale,omitempty"`
	CanMirror   *bool            `json:"canMirror,omitempty"`
	View        *Camera          `json:"view,omitempty"`
	Pieces      []PieceMeta      `json:"pieces,omitempty"`
	Connections []ConnectionMeta `json:"connections,omitempty"`
	Stats       []StatMeta       `json:"stats,omitempty"`
	Props       []PropMeta       `json:"props,omitempty"`
	Layers      []LayerMeta      `json:"layers,omitempty"`
	ActiveLayer *LayerId         `json:"activeLayer,omitempty"`
	Groups      []GroupMeta      `json:"groups,omitempty"`
	Location    *LocationId      `json:"location,omitempty"`
	Authors     []AuthorId       `json:"authors,omitempty"`
	Concepts    []ConceptId      `json:"concepts,omitempty"`
	Icon        *string          `json:"icon,omitempty"`
	Image       *string          `json:"image,omitempty"`
	Description *string          `json:"description,omitempty"`
	Attributes  []AttributeMeta  `json:"attributes,omitempty"`
	CreatedAt   string           `json:"createdAt,omitempty"`
	UpdatedAt   string           `json:"updatedAt,omitempty"`
}

// #endregion 📐Design

// #region ⏱️Kit

// #region 🧬KitKind
// KitKind discriminates the five persistence/transport forms of a Kit.

// 🧬KitKind represents the five persistence/transport forms of a kit.
// Specs: Exactly five kit kinds exist:
//   - KitKindDev: Self-contained JSON file (.kit.json)
//   - KitKindLocal: Local folder with .compose/kit.db SQLite file and asset files
//   - KitKindArchive: ZIP file packaging a LocalKit structure
//   - KitKindRemote: URL-addressable kit served over HTTP(S)
//   - KitKindTransport: Static JSON string for serialization/deserialization
type KitKind string

const (
	// KitKindDev is a self-contained JSON file (.kit.json).
	KitKindDev KitKind = "dev"
	// KitKindLocal is a local folder with .compose/kit.db SQLite file.
	KitKindLocal KitKind = "local"
	// KitKindArchive is a ZIP file packaging a LocalKit structure.
	KitKindArchive KitKind = "archive"
	// KitKindRemote is a URL-addressable kit served over HTTP(S).
	KitKindRemote KitKind = "remote"
	// KitKindTransport is a static JSON string for serialization/deserialization.
	KitKindTransport KitKind = "transport"
)

// 📜AllKitKinds contains the complete list of valid KitKind values.
var AllKitKinds = []KitKind{KitKindDev, KitKindLocal, KitKindArchive, KitKindRemote, KitKindTransport}

// ✔️IsValidKitKind checks if a KitKind value is one of the five valid kinds.
func IsValidKitKind(kind KitKind) bool {
	for _, k := range AllKitKinds {
		if k == kind {
			return true
		}
	}
	return false
}

// #endregion 🧬KitKind

// #region 🧳Kit Kind Types

// 🚚TransportKit wraps a static JSON string for kit serialization/deserialization.
type TransportKit struct {
	JSON string
}

// 📥ToKit deserializes the JSON string into a Kit.
func (t *TransportKit) ToKit() (Kit, error) {
	return DeserializeKit([]byte(t.JSON))
}

// 📤TransportKitFromKit creates a TransportKit from a Kit.
func TransportKitFromKit(kit Kit) (*TransportKit, error) {
	data, err := SerializeKit(kit)
	if err != nil {
		return nil, err
	}
	return &TransportKit{JSON: string(data)}, nil
}

// 📦ArchiveKit wraps a static zipped local kit.
type ArchiveKit struct {
	Data []byte
}

// 🔄SyncKit interface for synchronized kit kinds.
type SyncKit interface {
	Kit() *Kit
	Apply(diff *KitDiff)
	ImportTransport(transport *TransportKit) error
	ExportTransport() (*TransportKit, error)
	Close()
}

// 💻DevKit is a synchronized JSON file kit.
type DevKit struct {
	kit Kit
}

// 🏗️NewDevKit creates a new DevKit from a Kit.
func NewDevKit(kit Kit) *DevKit { return &DevKit{kit: kit} }

// 📦Kit returns a pointer to the underlying Kit.
func (d *DevKit) Kit() *Kit { return &d.kit }

// ▶️Apply applies a KitDiff to the underlying Kit in place.
func (d *DevKit) Apply(diff *KitDiff) {
	if diff == nil {
		return
	}
	_, _ = CommitKitGraphChange(&d.kit, *diff, &KitCommitOptions{SkipGlobalHistory: true, NotifyBackbone: KitNotifyDisable()})
}

// 📥ImportTransport imports a TransportKit by computing and applying the diff.
func (d *DevKit) ImportTransport(t *TransportKit) error {
	imported, err := t.ToKit()
	if err != nil {
		return err
	}
	diff := GetKitDiff(d.kit, imported)
	ApplyKitDiff(&d.kit, &diff)
	return nil
}

// 📤ExportTransport exports the Kit as a TransportKit.
func (d *DevKit) ExportTransport() (*TransportKit, error) { return TransportKitFromKit(d.kit) }

// 🔒Close is a no-operation for DevKit.
func (d *DevKit) Close() {}

// 📂LocalKit is a synchronized folder with .compose/kit.db SQLite database.
type LocalKit struct {
	kit Kit
}

// 🏗️NewLocalKit creates a new LocalKit from a Kit.
func NewLocalKit(kit Kit) *LocalKit { return &LocalKit{kit: kit} }

// 📦Kit returns a pointer to the underlying Kit.
func (l *LocalKit) Kit() *Kit { return &l.kit }

// ▶️Apply applies a KitDiff to the underlying Kit in place.
func (l *LocalKit) Apply(diff *KitDiff) {
	if diff == nil {
		return
	}
	_, _ = CommitKitGraphChange(&l.kit, *diff, &KitCommitOptions{SkipGlobalHistory: true, NotifyBackbone: KitNotifyDisable()})
}

// 📥ImportTransport imports a TransportKit by computing and applying the diff.
func (l *LocalKit) ImportTransport(t *TransportKit) error {
	imported, err := t.ToKit()
	if err != nil {
		return err
	}
	diff := GetKitDiff(l.kit, imported)
	ApplyKitDiff(&l.kit, &diff)
	return nil
}

// 📤ExportTransport exports the Kit as a TransportKit.
func (l *LocalKit) ExportTransport() (*TransportKit, error) { return TransportKitFromKit(l.kit) }

// 🔒Close is a no-operation for LocalKit.
func (l *LocalKit) Close() {}

// 🌐RemoteKit is a synchronized websocket connection to compose/hub.
type RemoteKit struct {
	kit Kit
}

// 🏗️NewRemoteKit creates a new RemoteKit from a Kit.
func NewRemoteKit(kit Kit) *RemoteKit { return &RemoteKit{kit: kit} }

// 📦Kit returns a pointer to the underlying Kit.
func (r *RemoteKit) Kit() *Kit { return &r.kit }

// ▶️Apply applies a KitDiff to the underlying Kit in place.
func (r *RemoteKit) Apply(diff *KitDiff) {
	if diff == nil {
		return
	}
	_, _ = CommitKitGraphChange(&r.kit, *diff, &KitCommitOptions{SkipGlobalHistory: true, NotifyBackbone: KitNotifyDisable()})
}

// 📥ImportTransport imports a TransportKit by computing and applying the diff.
func (r *RemoteKit) ImportTransport(t *TransportKit) error {
	imported, err := t.ToKit()
	if err != nil {
		return err
	}
	diff := GetKitDiff(r.kit, imported)
	ApplyKitDiff(&r.kit, &diff)
	return nil
}

// 📤ExportTransport exports the Kit as a TransportKit.
func (r *RemoteKit) ExportTransport() (*TransportKit, error) { return TransportKitFromKit(r.kit) }

// 🔒Close is a no-operation for RemoteKit.
func (r *RemoteKit) Close() {}

// #endregion 🧳Kit Kind Types

// 📦Kit represents the root container for all domain entities.
type Kit struct {
	Id          string      `json:"id"`
	Name        string      `json:"name"`
	Version     string      `json:"version"`
	Typologies  []Typology  `json:"typologies,omitempty"`
	Types       []Type      `json:"-"`
	Designs     []Design    `json:"-"`
	Tags        []Tag       `json:"tags,omitempty"`
	Concepts    []Concept   `json:"concepts,omitempty"`
	Families    []Family    `json:"families,omitempty"`
	Qualities   []Quality   `json:"qualities,omitempty"`
	Files       []File      `json:"files,omitempty"`
	Folders     []Folder    `json:"folders,omitempty"`
	Authors     []Author    `json:"authors,omitempty"`
	Remote      *string     `json:"remote,omitempty"`
	Homepage    *string     `json:"homepage,omitempty"`
	License     *string     `json:"license,omitempty"`
	Preview     *string     `json:"preview,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Image       *string     `json:"image,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`

	// Runtime session (json:"-"). Graph mutation APIs use *Kit.
	graphMu          sync.Mutex                                 `json:"-"`
	backbone         Backbone                                   `json:"-"`
	strictMode       bool                                       `json:"-"`
	conflicted       bool                                       `json:"-"`
	conflictErrors   []KitDiffValidationNote                    `json:"-"`
	conflictWarnings []KitDiffValidationNote                    `json:"-"`
	openTransactions map[string]*kitOpenTransaction             `json:"-"`
	historyPast      []KitGraphChange                           `json:"-"`
	historyFuture    []KitGraphChange                           `json:"-"`
	flattenMerkle    map[string]map[string]FlatMerkleCacheEntry `json:"-"`
}

// 🔏KitDiff represents a partial update to a kit's name, version, entities or metadata.
type KitDiff struct {
	Name        *string         `json:"name,omitempty"`
	Version     *string         `json:"version,omitempty"`
	Typologies  *TypologiesDiff `json:"typologies,omitempty"`
	Tags        *TagsDiff       `json:"tags,omitempty"`
	Concepts    *ConceptsDiff   `json:"concepts,omitempty"`
	Families    *FamiliesDiff   `json:"families,omitempty"`
	Qualities   *QualitiesDiff  `json:"qualities,omitempty"`
	Files       *FilesDiff      `json:"files,omitempty"`
	Folders     *FoldersDiff    `json:"folders,omitempty"`
	Authors     *AuthorsDiff    `json:"authors,omitempty"`
	Remote      *string         `json:"remote,omitempty"`
	Homepage    *string         `json:"homepage,omitempty"`
	License     *string         `json:"license,omitempty"`
	Preview     *string         `json:"preview,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Image       *string         `json:"image,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	UpdatedAt   *string         `json:"updatedAt,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// 📮UnmarshalJSON deserializes KitDiff JSON while tracking which fields are explicitly set.
func (d *KitDiff) UnmarshalJSON(data []byte) error {
	type Alias KitDiff
	aux := &struct{ *Alias }{Alias: (*Alias)(d)}
	var rawMap map[string]json.RawMessage
	if err := json.Unmarshal(data, &rawMap); err != nil {
		return err
	}
	d.setFields = make(map[string]bool)
	for key := range rawMap {
		d.setFields[key] = true
	}
	return json.Unmarshal(data, aux)
}

// 👀HasField checks whether a specific JSON field was present during KitDiff deserialization.
func (d *KitDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// 🎁KitsDiff represents batched kit additions, removals and per-kit updates.
type KitsDiff struct {
	Removed []KitId `json:"removed,omitempty"`
	Updated []struct {
		Kit  KitId   `json:"kit"`
		Diff KitDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Kit `json:"added,omitempty"`
}

// 🎀KitMeta represents the scalar-only view of a kit excluding types, designs and entity arrays.
type KitMeta struct {
	Id        string  `json:"id"`
	Name        string  `json:"name"`
	Version     string  `json:"version"`
	Remote      *string `json:"remote,omitempty"`
	Homepage    *string `json:"homepage,omitempty"`
	License     *string `json:"license,omitempty"`
	Preview     *string `json:"preview,omitempty"`
	Icon        *string `json:"icon,omitempty"`
	Image       *string `json:"image,omitempty"`
	Description *string `json:"description,omitempty"`
	CreatedAt   string  `json:"createdAt,omitempty"`
	UpdatedAt   string  `json:"updatedAt,omitempty"`
}

// 📓KitShallow represents a kit overview with nested arrays replaced by scalar-only items.
type KitShallow struct {
	Id        string          `json:"id"`
	Name        string          `json:"name"`
	Version     string          `json:"version"`
	Typologies  []TypologyShallow `json:"typologies,omitempty"`
	Tags        []TagMeta         `json:"tags,omitempty"`
	Concepts    []ConceptMeta   `json:"concepts,omitempty"`
	Families    []FamilyShallow `json:"families,omitempty"`
	Qualities   []QualityMeta   `json:"qualities,omitempty"`
	Files       []FileMeta      `json:"files,omitempty"`
	Folders     []FolderMeta    `json:"folders,omitempty"`
	Authors     []AuthorMeta    `json:"authors,omitempty"`
	Remote      *string         `json:"remote,omitempty"`
	Homepage    *string         `json:"homepage,omitempty"`
	License     *string         `json:"license,omitempty"`
	Preview     *string         `json:"preview,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Image       *string         `json:"image,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  []AttributeMeta `json:"attributes,omitempty"`
	CreatedAt   string          `json:"createdAt,omitempty"`
	UpdatedAt   string          `json:"updatedAt,omitempty"`
}

// 🏛️KitFlattenTypesDesigns rebuilds denormalized Types and Designs from Typologies.
func KitFlattenTypesDesigns(k *Kit) {
	if k == nil {
		return
	}
	k.Types = nil
	k.Designs = nil
	for ti := range k.Typologies {
		topo := &k.Typologies[ti]
		for i := range topo.Types {
			t := topo.Types[i]
			if t.Typology.Id == "" {
				t.Typology = TypologyId{Id: topo.Id}
			}
			k.Types = append(k.Types, t)
		}
		for i := range topo.Designs {
			d := topo.Designs[i]
			if d.Typology.Id == "" {
				d.Typology = TypologyId{Id: topo.Id}
			}
			k.Designs = append(k.Designs, d)
		}
	}
}

// 🏛️KitPackTypologiesFromFlat moves kit-level Types and Designs into a single default typology.
func KitPackTypologiesFromFlat(k *Kit) {
	if k == nil || len(k.Typologies) > 0 {
		return
	}
	if len(k.Types) == 0 && len(k.Designs) == 0 {
		return
	}
	topoID := Id()
	if len(k.Types) > 0 && k.Types[0].Typology.Id != "" {
		topoID = k.Types[0].Typology.Id
	} else if len(k.Designs) > 0 && k.Designs[0].Typology.Id != "" {
		topoID = k.Designs[0].Typology.Id
	}
	types := make([]Type, len(k.Types))
	copy(types, k.Types)
	designs := make([]Design, len(k.Designs))
	copy(designs, k.Designs)
	for i := range types {
		types[i].Typology = TypologyId{Id: topoID}
	}
	for i := range designs {
		designs[i].Typology = TypologyId{Id: topoID}
	}
	k.Typologies = []Typology{{
		Id:      topoID,
		Name:    "Default",
		Types:   types,
		Designs: designs,
	}}
	KitFlattenTypesDesigns(k)
}

// 🏛️KitEnsureTypologies guarantees Typologies exist and flat Types/Designs stay in sync.
func KitEnsureTypologies(k *Kit) {
	if k == nil {
		return
	}
	if len(k.Typologies) == 0 {
		KitPackTypologiesFromFlat(k)
	}
	KitFlattenTypesDesigns(k)
}

type kitJSON struct {
	Id          string      `json:"id"`
	Name        string      `json:"name"`
	Version     string      `json:"version"`
	Typologies  []Typology  `json:"typologies,omitempty"`
	Types       []Type      `json:"types,omitempty"`
	Designs     []Design    `json:"designs,omitempty"`
	Tags        []Tag       `json:"tags,omitempty"`
	Concepts    []Concept   `json:"concepts,omitempty"`
	Families    []Family    `json:"families,omitempty"`
	Qualities   []Quality   `json:"qualities,omitempty"`
	Files       []File      `json:"files,omitempty"`
	Folders     []Folder    `json:"folders,omitempty"`
	Authors     []Author    `json:"authors,omitempty"`
	Remote      *string     `json:"remote,omitempty"`
	Homepage    *string     `json:"homepage,omitempty"`
	License     *string     `json:"license,omitempty"`
	Preview     *string     `json:"preview,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Image       *string     `json:"image,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// 📮MarshalJSON serializes a kit with typologies as the storage root for types and designs.
func (k Kit) MarshalJSON() ([]byte, error) {
	KitEnsureTypologies(&k)
	out := kitJSON{
		Id: k.Id, Name: k.Name, Version: k.Version, Typologies: k.Typologies,
		Tags: k.Tags, Concepts: k.Concepts, Families: k.Families, Qualities: k.Qualities,
		Files: k.Files, Folders: k.Folders, Authors: k.Authors, Remote: k.Remote,
		Homepage: k.Homepage, License: k.License, Preview: k.Preview, Icon: k.Icon,
		Image: k.Image, Description: k.Description, Attributes: k.Attributes,
		CreatedAt: k.CreatedAt, UpdatedAt: k.UpdatedAt,
	}
	return json.Marshal(out)
}

// 📬UnmarshalJSON hydrates typologies and rebuilds flat Types and Designs.
func (k *Kit) UnmarshalJSON(data []byte) error {
	var raw kitJSON
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	k.Id = raw.Id
	k.Name = raw.Name
	k.Version = raw.Version
	k.Typologies = raw.Typologies
	k.Tags = raw.Tags
	k.Concepts = raw.Concepts
	k.Families = raw.Families
	k.Qualities = raw.Qualities
	k.Files = raw.Files
	k.Folders = raw.Folders
	k.Authors = raw.Authors
	k.Remote = raw.Remote
	k.Homepage = raw.Homepage
	k.License = raw.License
	k.Preview = raw.Preview
	k.Icon = raw.Icon
	k.Image = raw.Image
	k.Description = raw.Description
	k.Attributes = raw.Attributes
	k.CreatedAt = raw.CreatedAt
	k.UpdatedAt = raw.UpdatedAt
	if len(k.Typologies) == 0 && (len(raw.Types) > 0 || len(raw.Designs) > 0) {
		k.Types = raw.Types
		k.Designs = raw.Designs
		KitPackTypologiesFromFlat(k)
	} else {
		KitFlattenTypesDesigns(k)
	}
	return nil
}

// #endregion ⏱️Kit

// ⚖️Change represents a reversible entity change with forward and backward diffs.
type Change[TEntity any, TDiff any] struct {
	Forward  TDiff    `json:"forward"`
	Backward TDiff    `json:"backward"`
	Author   *string  `json:"author,omitempty"`
	Time     *string  `json:"time,omitempty"`
	Before   *TEntity `json:"before,omitempty"`
	After    *TEntity `json:"after,omitempty"`
}

type AttributeChange = Change[Attribute, AttributeDiff]

type LocationChange = Change[Location, LocationDiff]

type AuthorChange = Change[Author, AuthorDiff]

type FileChange = Change[File, FileDiff]

type FolderChange = Change[Folder, FolderDiff]

type BenchmarkChange = Change[Benchmark, BenchmarkDiff]

type QualityChange = Change[Quality, QualityDiff]

type PortChange = Change[Port, PortDiff]

type FamilyChange = Change[Family, FamilyDiff]

type TypologyChange = Change[Typology, TypologyDiff]

type PropChange = Change[Prop, PropDiff]

type TagChange = Change[Tag, TagDiff]

type ConceptChange = Change[Concept, ConceptDiff]

type RepresentationChange = Change[Representation, RepresentationDiff]

type ConnectorChange = Change[Connector, ConnectorDiff]

type TypeChange = Change[Type, TypeDiff]

type LayerChange = Change[Layer, LayerDiff]

type PieceChange = Change[Piece, PieceDiff]

type GroupChange = Change[Group, GroupDiff]

type SideChange = Change[Side, SideDiff]

type ConnectionChange = Change[Connection, ConnectionDiff]

type StatChange = Change[Stat, StatDiff]

type DesignChange = Change[Design, DesignDiff]

type KitChange = Change[Kit, KitDiff]

// 🔌DeletePiecesAndConnectionsInDesign deletes pieces and connections from a design, returning a canonical ComposeReport of DesignDiff.
// Removes stale connections referencing deleted pieces.
// 🔧Updates pieces that become fixed (parent connection removed) with flat plane and center from the flattened design.
func DeletePiecesAndConnectionsInDesign(kit *Kit, design Design, pieceIds []string, connectionIds []string) ComposeReport[DesignDiff] {
	deletedPieceSet := make(map[string]bool)
	for _, g := range pieceIds {
		deletedPieceSet[g] = true
	}

	// Find stale connections: connections referencing any deleted piece
	staleConnectionIds := make(map[string]bool)
	for _, conn := range design.Connections {
		if deletedPieceSet[conn.Parent.Piece.Id] || deletedPieceSet[conn.Child.Piece.Id] {
			staleConnectionIds[conn.Id] = true
		}
	}

	// All removed connections = explicit + stale
	allRemovedConnectionIds := make(map[string]bool)
	for _, g := range connectionIds {
		allRemovedConnectionIds[g] = true
	}
	for g := range staleConnectionIds {
		allRemovedConnectionIds[g] = true
	}

	// Find pieces that become fixed
	fixedPieceIds := []string{}
	fixedPieceSet := make(map[string]bool)
	for connId := range allRemovedConnectionIds {
		var conn *Connection
		for i := range design.Connections {
			if design.Connections[i].Id == connId {
				conn = &design.Connections[i]
				break
			}
		}
		if conn == nil {
			continue
		}
		connectingId := conn.Child.Piece.Id
		if deletedPieceSet[connectingId] {
			continue
		}
		// Check if this piece has another parent connection not in the removed set
		hasOtherParent := false
		for _, c := range design.Connections {
			if c.Child.Piece.Id == connectingId && !allRemovedConnectionIds[c.Id] {
				hasOtherParent = true
				break
			}
		}
		if !hasOtherParent && !fixedPieceSet[connectingId] {
			fixedPieceIds = append(fixedPieceIds, connectingId)
			fixedPieceSet[connectingId] = true
		}
	}

	// 🚚Build the diff
	var piecesRemoved []PieceId
	for _, g := range pieceIds {
		piecesRemoved = append(piecesRemoved, PieceId{Id: g})
	}

	// Flatten the design to get absolute plane and center for each piece (canonical report merges warnings/infos).
	flatRep := FlattenDesign(kit, design.Id)
	if !flatRep.Ok {
		return composeReportErr[DesignDiff](flatRep.Errors)
	}
	flatDiff := flatRep.Diff.Forward
	flatDesign := deepCloneDesign(design)
	ApplyDesignDiff(&flatDesign, &flatDiff)
	flatPieceMap := make(map[string]*Piece)
	for i := range flatDesign.Pieces {
		flatPieceMap[flatDesign.Pieces[i].Id] = &flatDesign.Pieces[i]
	}

	zero := 0.0
	one := 1.0
	identityPlaneDiff := &PlaneDiff{
		Origin: &PointDiff{X: &zero, Y: &zero, Z: &zero},
		XAxis:  &VectorDiff{X: &one, Y: &zero, Z: &zero},
		YAxis:  &VectorDiff{X: &zero, Y: &one, Z: &zero},
	}
	zeroCenterDiff := &CoordinateDiff{U: &zero, V: &zero}

	var piecesUpdated []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	}
	for _, g := range fixedPieceIds {
		planeDiff := identityPlaneDiff
		centerDiff := zeroCenterDiff
		if flatPiece, ok := flatPieceMap[g]; ok {
			if flatPiece.Plane != nil {
				ox, oy, oz := flatPiece.Plane.Origin.X, flatPiece.Plane.Origin.Y, flatPiece.Plane.Origin.Z
				xax, xay, xaz := flatPiece.Plane.XAxis.X, flatPiece.Plane.XAxis.Y, flatPiece.Plane.XAxis.Z
				yax, yay, yaz := flatPiece.Plane.YAxis.X, flatPiece.Plane.YAxis.Y, flatPiece.Plane.YAxis.Z
				planeDiff = &PlaneDiff{
					Origin: &PointDiff{X: &ox, Y: &oy, Z: &oz},
					XAxis:  &VectorDiff{X: &xax, Y: &xay, Z: &xaz},
					YAxis:  &VectorDiff{X: &yax, Y: &yay, Z: &yaz},
				}
			}
			if flatPiece.Center != nil {
				cu, cv := flatPiece.Center.U, flatPiece.Center.V
				centerDiff = &CoordinateDiff{U: &cu, V: &cv}
			}
		}
		piecesUpdated = append(piecesUpdated, struct {
			Piece PieceId   `json:"piece"`
			Diff  PieceDiff `json:"diff"`
		}{
			Piece: PieceId{Id: g},
			Diff: PieceDiff{
				Plane:  planeDiff,
				Center: centerDiff,
			},
		})
	}

	// Sort removed connections by id
	sortedConnectionIds := make([]string, 0, len(allRemovedConnectionIds))
	for g := range allRemovedConnectionIds {
		sortedConnectionIds = append(sortedConnectionIds, g)
	}
	sort.Strings(sortedConnectionIds)
	var connectionsRemoved []ConnectionId
	for _, g := range sortedConnectionIds {
		connectionsRemoved = append(connectionsRemoved, ConnectionId{Id: g})
	}

	diff := DesignDiff{}
	if len(piecesRemoved) > 0 || len(piecesUpdated) > 0 {
		diff.Pieces = &PiecesDiff{
			Removed: piecesRemoved,
			Updated: piecesUpdated,
		}
	}
	if len(connectionsRemoved) > 0 {
		diff.Connections = &ConnectionsDiff{
			Removed: connectionsRemoved,
		}
	}

	return composeReportOkWithNotes(diff, flatRep.Warnings, flatRep.Infos)
}

func GetDesignChange(before, after Design, author *string, time *string) DesignChange {
	forward := getDesignDiff(before, after)
	backward := inverseDesignDiff(before, forward)
	return DesignChange{Forward: forward, Backward: backward, Author: author, Time: time, Before: &before, After: &after}
}

func GetKitChange(before, after Kit, author *string, time *string) KitChange {
	forward := GetKitDiff(before, after)
	backward := InverseKitDiff(before, forward)
	return KitChange{Forward: forward, Backward: backward, Author: author, Time: time, Before: &before, After: &after}
}

// #region ⏰Serialization

// 📤SerializeKit marshals a Kit to indented JSON bytes.
func SerializeKit(kit Kit) ([]byte, error) {
	return json.MarshalIndent(kit, "", "  ")
}

// 📥DeserializeKit unmarshals JSON bytes into a Kit.
func DeserializeKit(data []byte) (Kit, error) {
	var kit Kit
	err := json.Unmarshal(data, &kit)
	return kit, err
}

// ✉️SerializeKitDiff marshals a KitDiff to indented JSON bytes.
func SerializeKitDiff(diff KitDiff) ([]byte, error) {
	return json.MarshalIndent(diff, "", "  ")
}

// 📩DeserializeKitDiff unmarshals JSON bytes into a KitDiff.
func DeserializeKitDiff(data []byte) (KitDiff, error) {
	var diff KitDiff
	err := json.Unmarshal(data, &diff)
	return diff, err
}

// #endregion ⏰Serialization

// #region 🔑Meta And Shallow

// 💎ToAttributeMeta converts an Attribute to its scalar-only Meta view.
func ToAttributeMeta(a Attribute) AttributeMeta {
	return AttributeMeta{Id: a.Id, Key: a.Key, Value: a.Value, Definition: a.Definition}
}

// ✍️ToAuthorMeta converts an Author to its scalar-only Meta view.
func ToAuthorMeta(a Author) AuthorMeta {
	return AuthorMeta{Id: a.Id, Name: a.Name, Email: a.Email, CreatedAt: a.CreatedAt, UpdatedAt: a.UpdatedAt}
}

// 📄ToFileMeta converts a File to its scalar-only Meta view.
func ToFileMeta(f File) FileMeta {
	return FileMeta{Id: f.Id, Name: f.Name, Remote: f.Remote, Folder: f.Folder, Size: f.Size, Hash: f.Hash, Description: f.Description, CreatedAt: f.CreatedAt, UpdatedAt: f.UpdatedAt}
}

// 📁ToFolderMeta converts a Folder to its scalar-only Meta view.
func ToFolderMeta(f Folder) FolderMeta {
	return FolderMeta{Id: f.Id, Name: f.Name, Parent: f.Parent, Description: f.Description, CreatedAt: f.CreatedAt, UpdatedAt: f.UpdatedAt}
}

// 🔬ToQualityMeta converts a Quality to its scalar-only Meta view.
func ToQualityMeta(q Quality) QualityMeta {
	return QualityMeta{Id: q.Id, Key: q.Key, Name: q.Name, Description: q.Description, Uri: q.Uri, Kind: q.Kind, CanScale: q.CanScale, DefaultSiUnit: q.DefaultSiUnit, DefaultImperialUnit: q.DefaultImperialUnit, Min: q.Min, IsMinExcluded: q.IsMinExcluded, Max: q.Max, IsMaxExcluded: q.IsMaxExcluded, DefaultValue: q.DefaultValue, Formula: q.Formula, Icon: q.Icon, Image: q.Image, Unit: q.Unit, CreatedAt: q.CreatedAt, UpdatedAt: q.UpdatedAt}
}

// ⚓ToPortMeta converts a Port to its scalar-only Meta view.
func ToPortMeta(p Port) PortMeta {
	return PortMeta{Id: p.Id, Name: p.Name, Description: p.Description, Icon: p.Icon, MaxChildren: p.MaxChildren, CreatedAt: p.CreatedAt, UpdatedAt: p.UpdatedAt}
}

// 🏛️ToTypologyMeta converts a Typology to its scalar-only Meta view.
func ToTypologyMeta(t Typology) TypologyMeta {
	return TypologyMeta{Id: t.Id, Name: t.Name, Description: t.Description, Icon: t.Icon, Folder: t.Folder, CreatedAt: t.CreatedAt, UpdatedAt: t.UpdatedAt}
}

// 👪ToFamilyMeta converts a Family to its scalar-only Meta view.
func ToFamilyMeta(f Family) FamilyMeta {
	return FamilyMeta{Id: f.Id, Name: f.Name, Description: f.Description, Icon: f.Icon, CreatedAt: f.CreatedAt, UpdatedAt: f.UpdatedAt}
}

// 🧾ToFamilyShallow converts a Family to its Shallow overview with port metadata.
func ToFamilyShallow(f Family) FamilyShallow {
	ports := make([]PortMeta, len(f.Ports))
	for i, p := range f.Ports {
		ports[i] = ToPortMeta(p)
	}
	return FamilyShallow{Id: f.Id, Name: f.Name, Description: f.Description, Icon: f.Icon, Ports: ports, CreatedAt: f.CreatedAt, UpdatedAt: f.UpdatedAt}
}

// 📊ToPropMeta converts a Prop to its scalar-only Meta view.
func ToPropMeta(p Prop) PropMeta {
	return PropMeta{Id: p.Id, Quality: p.Quality, Value: p.Value, Unit: p.Unit}
}

// 🏷️ToTagMeta converts a Tag to its scalar-only Meta view.
func ToTagMeta(t Tag) TagMeta {
	return TagMeta{Id: t.Id, Name: t.Name, Description: t.Description, Icon: t.Icon, CreatedAt: t.CreatedAt, UpdatedAt: t.UpdatedAt}
}

// 💡ToConceptMeta converts a Concept to its scalar-only Meta view.
func ToConceptMeta(c Concept) ConceptMeta {
	return ConceptMeta{Id: c.Id, Name: c.Name, Description: c.Description, Icon: c.Icon, CreatedAt: c.CreatedAt, UpdatedAt: c.UpdatedAt}
}

// 🗿ToRepresentationMeta converts a Representation to its scalar-only Meta view.
func ToRepresentationMeta(m Representation) RepresentationMeta {
	return RepresentationMeta{Id: m.Id, File: m.File, Name: m.Name, Description: m.Description}
}

// 🔌ToConnectorMeta converts a Connector to its scalar-only Meta view.
func ToConnectorMeta(c Connector) ConnectorMeta {
	return ConnectorMeta{Id: c.Id, Name: c.Name, Point: c.Point, Direction: c.Direction, T: c.T, Mandatory: c.Mandatory, Port: c.Port, Description: c.Description}
}

// 🎨ToLayerMeta converts a Layer to its scalar-only Meta view.
func ToLayerMeta(l Layer) LayerMeta {
	return LayerMeta{Id: l.Id, Path: l.Path, IsHidden: l.IsHidden, IsLocked: l.IsLocked, Color: l.Color, Description: l.Description}
}

// 🧩ToPieceMeta converts a Piece to its scalar-only Meta view.
func ToPieceMeta(p Piece) PieceMeta {
	return PieceMeta{Id: p.Id, Name: p.Name, Type: p.Type, Design: p.Design, Plane: p.Plane, Center: p.Center, Scale: p.Scale, MirrorPlane: p.MirrorPlane, IsHidden: p.IsHidden, IsLocked: p.IsLocked, Color: p.Color, Description: p.Description}
}

// 👥ToGroupMeta converts a Group to its scalar-only Meta view.
func ToGroupMeta(g Group) GroupMeta {
	return GroupMeta{Id: g.Id, Name: g.Name, Color: g.Color, Description: g.Description}
}

// 🔗ToConnectionMeta converts a Connection to its scalar-only Meta view.
func ToConnectionMeta(c Connection) ConnectionMeta {
	return ConnectionMeta{Id: c.Id, Parent: c.Parent, Child: c.Child, Gap: c.Gap, Shift: c.Shift, Rise: c.Rise, Rotation: c.Rotation, Turn: c.Turn, Tilt: c.Tilt, U: c.U, V: c.V, Description: c.Description}
}

// 📈ToStatMeta converts a Stat to its scalar-only Meta view.
func ToStatMeta(s Stat) StatMeta {
	return StatMeta{Id: s.Id, Quality: s.Quality, Min: s.Min, Max: s.Max, Unit: s.Unit}
}

// 🧱ToTypeMeta converts a Type to its scalar-only Meta view.
func ToTypeMeta(t Type) TypeMeta {
	return TypeMeta{Id: t.Id, Name: t.Name, Typology: t.Typology, Families: t.Families, IsAbstract: t.IsAbstract, Virtual: t.Virtual, Unit: t.Unit, Stock: t.Stock, Location: t.Location, Folder: t.Folder, Icon: t.Icon, Image: t.Image, Description: t.Description, CreatedAt: t.CreatedAt, UpdatedAt: t.UpdatedAt}
}

// 🏗️ToTypeShallow converts a Type to its Shallow overview with scalar-only nested items.
func ToTypeShallow(t Type) TypeShallow {
	representations := make([]RepresentationMeta, len(t.Representations))
	for i, m := range t.Representations {
		representations[i] = ToRepresentationMeta(m)
	}
	connectors := make([]ConnectorMeta, len(t.Connectors))
	for i, c := range t.Connectors {
		connectors[i] = ToConnectorMeta(c)
	}
	props := make([]PropMeta, len(t.Props))
	for i, p := range t.Props {
		props[i] = ToPropMeta(p)
	}
	attributes := make([]AttributeMeta, len(t.Attributes))
	for i, a := range t.Attributes {
		attributes[i] = ToAttributeMeta(a)
	}
	return TypeShallow{Id: t.Id, Name: t.Name, Typology: t.Typology, Families: t.Families, IsAbstract: t.IsAbstract, Virtual: t.Virtual, Unit: t.Unit, Stock: t.Stock, Location: t.Location, Folder: t.Folder, Representations: representations, Connectors: connectors, Props: props, Authors: t.Authors, Concepts: t.Concepts, Icon: t.Icon, Image: t.Image, Description: t.Description, Attributes: attributes, CreatedAt: t.CreatedAt, UpdatedAt: t.UpdatedAt}
}

// 📐ToDesignMeta converts a Design to its scalar-only Meta view.
func ToDesignMeta(d Design) DesignMeta {
	return DesignMeta{Id: d.Id, Name: d.Name, Typology: d.Typology, Families: d.Families, IsAbstract: d.IsAbstract, Unit: d.Unit, Folder: d.Folder, CanScale: d.CanScale, CanMirror: d.CanMirror, View: d.View, ActiveLayer: d.ActiveLayer, Location: d.Location, Icon: d.Icon, Image: d.Image, Description: d.Description, CreatedAt: d.CreatedAt, UpdatedAt: d.UpdatedAt}
}

// 🏕️ToDesignShallow converts a Design to its Shallow overview with scalar-only nested items.
func ToDesignShallow(d Design) DesignShallow {
	pieces := make([]PieceMeta, len(d.Pieces))
	for i, p := range d.Pieces {
		pieces[i] = ToPieceMeta(p)
	}
	connections := make([]ConnectionMeta, len(d.Connections))
	for i, c := range d.Connections {
		connections[i] = ToConnectionMeta(c)
	}
	stats := make([]StatMeta, len(d.Stats))
	for i, s := range d.Stats {
		stats[i] = ToStatMeta(s)
	}
	props := make([]PropMeta, len(d.Props))
	for i, p := range d.Props {
		props[i] = ToPropMeta(p)
	}
	layers := make([]LayerMeta, len(d.Layers))
	for i, l := range d.Layers {
		layers[i] = ToLayerMeta(l)
	}
	groups := make([]GroupMeta, len(d.Groups))
	for i, g := range d.Groups {
		groups[i] = ToGroupMeta(g)
	}
	attributes := make([]AttributeMeta, len(d.Attributes))
	for i, a := range d.Attributes {
		attributes[i] = ToAttributeMeta(a)
	}
	return DesignShallow{Id: d.Id, Name: d.Name, Typology: d.Typology, Families: d.Families, IsAbstract: d.IsAbstract, Unit: d.Unit, Folder: d.Folder, CanScale: d.CanScale, CanMirror: d.CanMirror, View: d.View, Pieces: pieces, Connections: connections, Stats: stats, Props: props, Layers: layers, ActiveLayer: d.ActiveLayer, Groups: groups, Location: d.Location, Authors: d.Authors, Concepts: d.Concepts, Icon: d.Icon, Image: d.Image, Description: d.Description, Attributes: attributes, CreatedAt: d.CreatedAt, UpdatedAt: d.UpdatedAt}
}

// 📦ToKitMeta converts a Kit to its scalar-only Meta view.
func ToKitMeta(k Kit) KitMeta {
	return KitMeta{Id: k.Id, Name: k.Name, Version: k.Version, Remote: k.Remote, Homepage: k.Homepage, License: k.License, Preview: k.Preview, Icon: k.Icon, Image: k.Image, Description: k.Description, CreatedAt: k.CreatedAt, UpdatedAt: k.UpdatedAt}
}

// 📦ToKitShallow converts a Kit to its Shallow overview with scalar-only nested items.
func ToKitShallow(k Kit) KitShallow {
	KitEnsureTypologies(&k)
	typologies := make([]TypologyShallow, len(k.Typologies))
	for i, topo := range k.Typologies {
		types := make([]TypeMeta, len(topo.Types))
		for j, t := range topo.Types {
			types[j] = ToTypeMeta(t)
		}
		designs := make([]DesignMeta, len(topo.Designs))
		for j, d := range topo.Designs {
			designs[j] = ToDesignMeta(d)
		}
		typologies[i] = TypologyShallow{
			Id: topo.Id, Name: topo.Name, Description: topo.Description, Icon: topo.Icon,
			Folder: topo.Folder, Types: types, Designs: designs, CreatedAt: topo.CreatedAt, UpdatedAt: topo.UpdatedAt,
		}
	}
	tags := make([]TagMeta, len(k.Tags))
	for i, t := range k.Tags {
		tags[i] = ToTagMeta(t)
	}
	concepts := make([]ConceptMeta, len(k.Concepts))
	for i, c := range k.Concepts {
		concepts[i] = ToConceptMeta(c)
	}
	families := make([]FamilyShallow, len(k.Families))
	for i, f := range k.Families {
		families[i] = ToFamilyShallow(f)
	}
	qualities := make([]QualityMeta, len(k.Qualities))
	for i, q := range k.Qualities {
		qualities[i] = ToQualityMeta(q)
	}
	files := make([]FileMeta, len(k.Files))
	for i, f := range k.Files {
		files[i] = ToFileMeta(f)
	}
	folders := make([]FolderMeta, len(k.Folders))
	for i, f := range k.Folders {
		folders[i] = ToFolderMeta(f)
	}
	authors := make([]AuthorMeta, len(k.Authors))
	for i, a := range k.Authors {
		authors[i] = ToAuthorMeta(a)
	}
	attributes := make([]AttributeMeta, len(k.Attributes))
	for i, a := range k.Attributes {
		attributes[i] = ToAttributeMeta(a)
	}
	return KitShallow{Id: k.Id, Name: k.Name, Version: k.Version, Typologies: typologies, Tags: tags, Concepts: concepts, Families: families, Qualities: qualities, Files: files, Folders: folders, Authors: authors, Remote: k.Remote, Homepage: k.Homepage, License: k.License, Preview: k.Preview, Icon: k.Icon, Image: k.Image, Description: k.Description, Attributes: attributes, CreatedAt: k.CreatedAt, UpdatedAt: k.UpdatedAt}
}

// #endregion 🔑Meta And Shallow

// #region 🖥️Hash
// Merkle hash functions for all entities. Each hash function computes a deterministic
// SHA-256 hex digest. Collections are hashed by sorting child hashes alphabetically.
// Field order is alphabetical by JSON field name. Missing/null fields are skipped.
// Number format: integer if no fractional part, else shortest decimal representation.

// #region 🌩️HashWriter
// 💾hashWriter accumulates binary data for deterministic SHA-256 hashing.
type hashWriter struct {
	buf bytes.Buffer
}

func (w *hashWriter) writeString(s string) {
	b := []byte(s)
	lb := make([]byte, 4)
	binary.BigEndian.PutUint32(lb, uint32(len(b)))
	w.buf.Write(lb)
	w.buf.Write(b)
}

func (w *hashWriter) writeNumber(n float64) {
	w.writeString(FormatNumberForHash(n))
}

func (w *hashWriter) writeIntNumber(n int) {
	w.writeString(strconv.Itoa(n))
}

func (w *hashWriter) writeBool(b bool) {
	if b {
		w.buf.WriteByte(1)
	} else {
		w.buf.WriteByte(0)
	}
}

func (w *hashWriter) writeHash(h string) {
	w.writeString(h)
}

func (w *hashWriter) writeHashList(hashes []string) {
	sorted := make([]string, len(hashes))
	copy(sorted, hashes)
	sort.Strings(sorted)
	lb := make([]byte, 4)
	binary.BigEndian.PutUint32(lb, uint32(len(sorted)))
	w.buf.Write(lb)
	for _, h := range sorted {
		w.writeString(h)
	}
}

func (w *hashWriter) writeIdList(ids []string) {
	sorted := make([]string, len(ids))
	copy(sorted, ids)
	sort.Strings(sorted)
	lb := make([]byte, 4)
	binary.BigEndian.PutUint32(lb, uint32(len(sorted)))
	w.buf.Write(lb)
	for _, g := range sorted {
		w.writeString(g)
	}
}

func (w *hashWriter) digest() string {
	h := sha256.Sum256(w.buf.Bytes())
	return hex.EncodeToString(h[:])
}

// 🔢FormatNumberForHash formats a number deterministically for hashing.
func FormatNumberForHash(n float64) string {
	if n == math.Trunc(n) && !math.IsInf(n, 0) && math.Abs(n) < 1e15 {
		return strconv.FormatInt(int64(n), 10)
	}
	abs := math.Abs(n)
	// Match JavaScript's Number.prototype.toString():
	// Scientific notation for abs < 1e-6 or abs >= 1e21.
	if abs > 0 && (abs < 1e-6 || abs >= 1e21) {
		s := strconv.FormatFloat(n, 'e', -1, 64)
		// Remove leading zeros from exponent: e-07 → e-7, e+02 → e+2
		idx := strings.LastIndex(s, "e")
		if idx != -1 {
			exp := s[idx+1:]
			sign := string(exp[0])
			digits := strings.TrimLeft(exp[1:], "0")
			if digits == "" {
				digits = "0"
			}
			return s[:idx] + "e" + sign + digits
		}
		return s
	}
	return strconv.FormatFloat(n, 'f', -1, 64)
}

// #endregion 🌩️HashWriter

// #region 🎵Hash Value Types

// 📺HashCoordinate computes SHA-256 hash of a Coordinate value.
func HashCoordinate(c Coordinate) string {
	w := &hashWriter{}
	w.writeString("Coordinate")
	w.writeString("u")
	w.writeNumber(c.U)
	w.writeString("v")
	w.writeNumber(c.V)
	return w.digest()
}

// ➡️HashVec computes SHA-256 hash of a Vec value.
func HashVec(v Vec) string {
	w := &hashWriter{}
	w.writeString("Vec")
	w.writeString("u")
	w.writeNumber(v.U)
	w.writeString("v")
	w.writeNumber(v.V)
	return w.digest()
}

// ✖️HashPoint computes SHA-256 hash of a Point value.
func HashPoint(p Point) string {
	w := &hashWriter{}
	w.writeString("Point")
	w.writeString("x")
	w.writeNumber(p.X)
	w.writeString("y")
	w.writeNumber(p.Y)
	w.writeString("z")
	w.writeNumber(p.Z)
	return w.digest()
}

// ↗️HashVector computes SHA-256 hash of a Vector value.
func HashVector(v Vector) string {
	w := &hashWriter{}
	w.writeString("Vector")
	w.writeString("x")
	w.writeNumber(v.X)
	w.writeString("y")
	w.writeNumber(v.Y)
	w.writeString("z")
	w.writeNumber(v.Z)
	return w.digest()
}

// ◻️HashPlane computes SHA-256 hash of a Plane value.
func HashPlane(p Plane) string {
	w := &hashWriter{}
	w.writeString("Plane")
	w.writeString("origin")
	w.writeHash(HashPoint(p.Origin))
	w.writeString("xAxis")
	w.writeHash(HashVector(p.XAxis))
	w.writeString("yAxis")
	w.writeHash(HashVector(p.YAxis))
	return w.digest()
}

// 🎥HashCamera computes SHA-256 hash of a Camera value.
func HashCamera(c Camera) string {
	w := &hashWriter{}
	w.writeString("Camera")
	w.writeString("forward")
	w.writeHash(HashVector(c.Forward))
	w.writeString("position")
	w.writeHash(HashPoint(c.Position))
	w.writeString("up")
	w.writeHash(HashVector(c.Up))
	return w.digest()
}

// #endregion 🎵Hash Value Types

// #region 🎩Hash Entities

// 💎HashAttribute computes SHA-256 hash of an Attribute entity.
func HashAttribute(a Attribute) string {
	w := &hashWriter{}
	w.writeString("Attribute")
	if a.Definition != nil {
		w.writeString("definition")
		w.writeString(*a.Definition)
	}
	w.writeString("id")
	w.writeString(a.Id)
	w.writeString("key")
	w.writeString(a.Key)
	if a.Value != nil {
		w.writeString("value")
		w.writeString(*a.Value)
	}
	return w.digest()
}

// 📍HashLocation computes SHA-256 hash of a Location entity.
func HashLocation(l Location) string {
	w := &hashWriter{}
	w.writeString("Location")
	if l.Altitude != nil {
		w.writeString("altitude")
		w.writeNumber(*l.Altitude)
	}
	if len(l.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(l.Attributes))
		for i, a := range l.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	w.writeString("id")
	w.writeString(l.Id)
	w.writeString("latitude")
	w.writeNumber(l.Latitude)
	w.writeString("longitude")
	w.writeNumber(l.Longitude)
	return w.digest()
}

// ✍️HashAuthor computes SHA-256 hash of an Author entity.
func HashAuthor(a Author) string {
	w := &hashWriter{}
	w.writeString("Author")
	if len(a.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(a.Attributes))
		for i, attr := range a.Attributes {
			hashes[i] = HashAttribute(attr)
		}
		w.writeHashList(hashes)
	}
	if a.Email != nil && *a.Email != "" {
		w.writeString("email")
		w.writeString(*a.Email)
	}
	w.writeString("id")
	w.writeString(a.Id)
	w.writeString("name")
	w.writeString(a.Name)
	return w.digest()
}

// 📄HashFile computes SHA-256 hash of a File entity.
func HashFile(f File) string {
	w := &hashWriter{}
	w.writeString("File")
	if f.Blob != nil {
		w.writeString("blob")
		w.writeString(*f.Blob)
	}
	if f.Folder != nil {
		w.writeString("folder")
		w.writeString(f.Folder.Id)
	}
	w.writeString("id")
	w.writeString(f.Id)
	if f.Hash != nil {
		w.writeString("hash")
		w.writeString(*f.Hash)
	}
	w.writeString("name")
	w.writeString(f.Name)
	if f.Remote != nil {
		w.writeString("remote")
		w.writeString(*f.Remote)
	}
	if f.Size != nil {
		w.writeString("size")
		w.writeIntNumber(int(*f.Size))
	}
	return w.digest()
}

// 📁HashFolder computes SHA-256 hash of a Folder entity.
func HashFolder(f Folder) string {
	w := &hashWriter{}
	w.writeString("Folder")
	if len(f.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(f.Attributes))
		for i, a := range f.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if f.Description != nil {
		w.writeString("description")
		w.writeString(*f.Description)
	}
	w.writeString("id")
	w.writeString(f.Id)
	w.writeString("name")
	w.writeString(f.Name)
	if f.Parent != nil {
		w.writeString("parent")
		w.writeString(f.Parent.Id)
	}
	return w.digest()
}

// 📏HashBenchmark computes SHA-256 hash of a Benchmark entity.
func HashBenchmark(b Benchmark) string {
	w := &hashWriter{}
	w.writeString("Benchmark")
	if len(b.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(b.Attributes))
		for i, a := range b.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	w.writeString("id")
	w.writeString(b.Id)
	if b.Icon != nil {
		w.writeString("icon")
		w.writeString(*b.Icon)
	}
	if b.Max != nil {
		w.writeString("max")
		w.writeNumber(*b.Max)
	}
	if b.MaxExcluded != nil {
		w.writeString("maxExcluded")
		w.writeBool(*b.MaxExcluded)
	}
	if b.Min != nil {
		w.writeString("min")
		w.writeNumber(*b.Min)
	}
	if b.MinExcluded != nil {
		w.writeString("minExcluded")
		w.writeBool(*b.MinExcluded)
	}
	w.writeString("name")
	w.writeString(b.Name)
	return w.digest()
}

// 🔬HashQuality computes SHA-256 hash of a Quality entity.
func HashQuality(q Quality) string {
	w := &hashWriter{}
	w.writeString("Quality")
	if len(q.Benchmarks) > 0 {
		w.writeString("benchmarks")
		hashes := make([]string, len(q.Benchmarks))
		for i, b := range q.Benchmarks {
			hashes[i] = HashBenchmark(b)
		}
		w.writeHashList(hashes)
	}
	if q.CanScale != nil {
		w.writeString("canScale")
		w.writeBool(*q.CanScale)
	}
	if q.DefaultImperialUnit != nil {
		w.writeString("defaultImperialUnit")
		w.writeString(*q.DefaultImperialUnit)
	}
	if q.DefaultSiUnit != nil {
		w.writeString("defaultSiUnit")
		w.writeString(*q.DefaultSiUnit)
	}
	if q.DefaultValue != nil {
		w.writeString("defaultValue")
		w.writeNumber(*q.DefaultValue)
	}
	if q.Description != nil {
		w.writeString("description")
		w.writeString(*q.Description)
	}
	if q.Formula != nil {
		w.writeString("formula")
		w.writeString(*q.Formula)
	}
	w.writeString("id")
	w.writeString(q.Id)
	if q.Icon != nil {
		w.writeString("icon")
		w.writeString(*q.Icon)
	}
	if q.Image != nil {
		w.writeString("image")
		w.writeString(*q.Image)
	}
	if q.IsMaxExcluded != nil {
		w.writeString("isMaxExcluded")
		w.writeBool(*q.IsMaxExcluded)
	}
	if q.IsMinExcluded != nil {
		w.writeString("isMinExcluded")
		w.writeBool(*q.IsMinExcluded)
	}
	w.writeString("key")
	w.writeString(q.Key)
	if q.Kind != 0 {
		w.writeString("kind")
		w.writeIntNumber(int(q.Kind))
	}
	if q.Max != nil {
		w.writeString("max")
		w.writeNumber(*q.Max)
	}
	if q.Min != nil {
		w.writeString("min")
		w.writeNumber(*q.Min)
	}
	w.writeString("name")
	w.writeString(q.Name)
	if q.Unit != nil {
		w.writeString("unit")
		w.writeString(*q.Unit)
	}
	if q.Uri != nil {
		w.writeString("uri")
		w.writeString(*q.Uri)
	}
	return w.digest()
}

// ⚓HashPort computes SHA-256 hash of a Port entity.
func HashPort(p Port) string {
	w := &hashWriter{}
	w.writeString("Port")
	if len(p.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(p.Attributes))
		for i, a := range p.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if len(p.CompatiblePorts) > 0 {
		w.writeString("compatiblePorts")
		ids := make([]string, len(p.CompatiblePorts))
		for i, cp := range p.CompatiblePorts {
			ids[i] = cp.Id
		}
		w.writeIdList(ids)
	}
	if p.Description != nil {
		w.writeString("description")
		w.writeString(*p.Description)
	}
	w.writeString("id")
	w.writeString(p.Id)
	if p.Icon != nil {
		w.writeString("icon")
		w.writeString(*p.Icon)
	}
	w.writeString("name")
	w.writeString(p.Name)
	return w.digest()
}

// 👪HashFamily computes SHA-256 hash of a Family entity and its owned ports.
func HashFamily(f Family) string {
	w := &hashWriter{}
	w.writeString("Family")
	if len(f.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(f.Attributes))
		for i, a := range f.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if f.Description != nil {
		w.writeString("description")
		w.writeString(*f.Description)
	}
	w.writeString("id")
	w.writeString(f.Id)
	if f.Icon != nil {
		w.writeString("icon")
		w.writeString(*f.Icon)
	}
	w.writeString("name")
	w.writeString(f.Name)
	if len(f.Ports) > 0 {
		w.writeString("ports")
		hashes := make([]string, len(f.Ports))
		for i, p := range f.Ports {
			hashes[i] = HashPort(p)
		}
		w.writeHashList(hashes)
	}
	return w.digest()
}

// 🏛️HashTypology computes SHA-256 hash of a Typology and its owned types and designs.
func HashTypology(t Typology) string {
	w := &hashWriter{}
	w.writeString("Typology")
	if t.Description != nil {
		w.writeString("description")
		w.writeString(*t.Description)
	}
	if len(t.Designs) > 0 {
		w.writeString("designs")
		hashes := make([]string, len(t.Designs))
		for i, d := range t.Designs {
			hashes[i] = HashDesign(d)
		}
		w.writeHashList(hashes)
	}
	if t.Folder != nil {
		w.writeString("folder")
		w.writeString(*t.Folder)
	}
	w.writeString("id")
	w.writeString(t.Id)
	if t.Icon != nil {
		w.writeString("icon")
		w.writeString(*t.Icon)
	}
	w.writeString("name")
	w.writeString(t.Name)
	if len(t.Types) > 0 {
		w.writeString("types")
		hashes := make([]string, len(t.Types))
		for i, ty := range t.Types {
			hashes[i] = HashType(ty)
		}
		w.writeHashList(hashes)
	}
	return w.digest()
}

// 📊HashProp computes SHA-256 hash of a Prop entity.
func HashProp(p Prop) string {
	w := &hashWriter{}
	w.writeString("Prop")
	if len(p.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(p.Attributes))
		for i, a := range p.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	w.writeString("id")
	w.writeString(p.Id)
	w.writeString("quality")
	w.writeString(p.Quality.Id)
	if p.Unit != nil {
		w.writeString("unit")
		w.writeString(*p.Unit)
	}
	w.writeString("value")
	w.writeString(p.Value)
	return w.digest()
}

// 🏷️HashTag computes SHA-256 hash of a Tag entity.
func HashTag(t Tag) string {
	w := &hashWriter{}
	w.writeString("Tag")
	if len(t.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(t.Attributes))
		for i, a := range t.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if t.Description != nil {
		w.writeString("description")
		w.writeString(*t.Description)
	}
	w.writeString("id")
	w.writeString(t.Id)
	if t.Icon != nil {
		w.writeString("icon")
		w.writeString(*t.Icon)
	}
	w.writeString("name")
	w.writeString(t.Name)
	return w.digest()
}

// 💡HashConcept computes SHA-256 hash of a Concept entity.
func HashConcept(c Concept) string {
	w := &hashWriter{}
	w.writeString("Concept")
	if len(c.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(c.Attributes))
		for i, a := range c.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if c.Description != nil {
		w.writeString("description")
		w.writeString(*c.Description)
	}
	w.writeString("id")
	w.writeString(c.Id)
	if c.Icon != nil {
		w.writeString("icon")
		w.writeString(*c.Icon)
	}
	w.writeString("name")
	w.writeString(c.Name)
	return w.digest()
}

// 🗿HashRepresentation computes SHA-256 hash of a Representation entity.
func HashRepresentation(m Representation) string {
	w := &hashWriter{}
	w.writeString("Representation")
	if len(m.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(m.Attributes))
		for i, a := range m.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if m.Description != nil {
		w.writeString("description")
		w.writeString(*m.Description)
	}
	w.writeString("file")
	w.writeString(m.File.Id)
	w.writeString("id")
	w.writeString(m.Id)
	if m.Name != nil {
		w.writeString("name")
		w.writeString(*m.Name)
	}
	if len(m.Tags) > 0 {
		w.writeString("tags")
		ids := make([]string, len(m.Tags))
		for i, t := range m.Tags {
			ids[i] = t.Id
		}
		w.writeIdList(ids)
	}
	return w.digest()
}

// 🔌HashConnector computes SHA-256 hash of a Connector entity.
func HashConnector(c Connector) string {
	w := &hashWriter{}
	w.writeString("Connector")
	if len(c.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(c.Attributes))
		for i, a := range c.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if c.Description != nil {
		w.writeString("description")
		w.writeString(*c.Description)
	}
	w.writeString("direction")
	w.writeHash(HashVector(c.Direction))
	w.writeString("id")
	w.writeString(c.Id)
	if c.Mandatory != nil {
		w.writeString("mandatory")
		w.writeBool(*c.Mandatory)
	}
	if c.Name != nil {
		w.writeString("name")
		w.writeString(*c.Name)
	}
	w.writeString("point")
	w.writeHash(HashPoint(c.Point))
	if c.Port != nil {
		w.writeString("port")
		w.writeString(c.Port.Id)
	}
	if len(c.Props) > 0 {
		w.writeString("props")
		hashes := make([]string, len(c.Props))
		for i, p := range c.Props {
			hashes[i] = HashProp(p)
		}
		w.writeHashList(hashes)
	}
	w.writeString("t")
	w.writeNumber(c.T)
	return w.digest()
}

// 🧱HashType computes SHA-256 hash of a Type entity.
func HashType(t Type) string {
	w := &hashWriter{}
	w.writeString("Type")
	if len(t.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(t.Attributes))
		for i, a := range t.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if len(t.Authors) > 0 {
		w.writeString("authors")
		ids := make([]string, len(t.Authors))
		for i, a := range t.Authors {
			ids[i] = a.Id
		}
		w.writeIdList(ids)
	}
	if len(t.Concepts) > 0 {
		w.writeString("concepts")
		ids := make([]string, len(t.Concepts))
		for i, c := range t.Concepts {
			ids[i] = c.Id
		}
		w.writeIdList(ids)
	}
	if len(t.Connectors) > 0 {
		w.writeString("connectors")
		hashes := make([]string, len(t.Connectors))
		for i, c := range t.Connectors {
			hashes[i] = HashConnector(c)
		}
		w.writeHashList(hashes)
	}
	if t.Description != nil {
		w.writeString("description")
		w.writeString(*t.Description)
	}
	if t.Folder != nil {
		w.writeString("folder")
		w.writeString(*t.Folder)
	}
	w.writeString("id")
	w.writeString(t.Id)
	if t.Icon != nil {
		w.writeString("icon")
		w.writeString(*t.Icon)
	}
	if t.Image != nil {
		w.writeString("image")
		w.writeString(*t.Image)
	}
	if t.IsAbstract != nil {
		w.writeString("isAbstract")
		w.writeBool(*t.IsAbstract)
	}
	if t.Location != nil {
		w.writeString("location")
		w.writeString(t.Location.Id)
	}
	if t.Typology.Id != "" {
		w.writeString("typology")
		w.writeString(t.Typology.Id)
	}
	if len(t.Representations) > 0 {
		w.writeString("representations")
		hashes := make([]string, len(t.Representations))
		for i, m := range t.Representations {
			hashes[i] = HashRepresentation(m)
		}
		w.writeHashList(hashes)
	}
	w.writeString("name")
	w.writeString(t.Name)
	if len(t.Families) > 0 {
		w.writeString("families")
		ids := make([]string, len(t.Families))
		for i, family := range t.Families {
			ids[i] = family.Id
		}
		w.writeIdList(ids)
	}
	if len(t.Props) > 0 {
		w.writeString("props")
		hashes := make([]string, len(t.Props))
		for i, p := range t.Props {
			hashes[i] = HashProp(p)
		}
		w.writeHashList(hashes)
	}
	if t.Stock != nil {
		w.writeString("stock")
		w.writeIntNumber(*t.Stock)
	}
	if t.Unit != nil {
		w.writeString("unit")
		w.writeString(*t.Unit)
	}
	if t.Virtual != nil {
		w.writeString("virtual")
		w.writeBool(*t.Virtual)
	}
	return w.digest()
}

// 🎨HashLayer computes SHA-256 hash of a Layer entity.
func HashLayer(l Layer) string {
	w := &hashWriter{}
	w.writeString("Layer")
	if len(l.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(l.Attributes))
		for i, a := range l.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if l.Color != nil {
		w.writeString("color")
		w.writeString(*l.Color)
	}
	if l.Description != nil {
		w.writeString("description")
		w.writeString(*l.Description)
	}
	w.writeString("id")
	w.writeString(l.Id)
	if l.IsHidden != nil {
		w.writeString("isHidden")
		w.writeBool(*l.IsHidden)
	}
	if l.IsLocked != nil {
		w.writeString("isLocked")
		w.writeBool(*l.IsLocked)
	}
	w.writeString("path")
	w.writeString(l.Path)
	return w.digest()
}

// 📈HashStat computes SHA-256 hash of a Stat entity.
func HashStat(s Stat) string {
	w := &hashWriter{}
	w.writeString("Stat")
	w.writeString("id")
	w.writeString(s.Id)
	if s.Max != nil {
		w.writeString("max")
		w.writeNumber(*s.Max)
	}
	if s.MaxExcluded != nil {
		w.writeString("maxExcluded")
		w.writeBool(*s.MaxExcluded)
	}
	if s.Min != nil {
		w.writeString("min")
		w.writeNumber(*s.Min)
	}
	if s.MinExcluded != nil {
		w.writeString("minExcluded")
		w.writeBool(*s.MinExcluded)
	}
	w.writeString("quality")
	w.writeString(s.Quality.Id)
	if s.Unit != nil {
		w.writeString("unit")
		w.writeString(*s.Unit)
	}
	return w.digest()
}

// 👥HashGroup computes SHA-256 hash of a Group entity.
func HashGroup(g Group) string {
	w := &hashWriter{}
	w.writeString("Group")
	if len(g.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(g.Attributes))
		for i, a := range g.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if g.Color != nil {
		w.writeString("color")
		w.writeString(*g.Color)
	}
	if g.Description != nil {
		w.writeString("description")
		w.writeString(*g.Description)
	}
	w.writeString("id")
	w.writeString(g.Id)
	if g.Name != nil {
		w.writeString("name")
		w.writeString(*g.Name)
	}
	w.writeString("pieces")
	ids := make([]string, len(g.Pieces))
	for i, p := range g.Pieces {
		ids[i] = p.Id
	}
	w.writeIdList(ids)
	return w.digest()
}

// ↔️HashSide computes SHA-256 hash of a Side value.
func HashSide(s Side) string {
	w := &hashWriter{}
	w.writeString("Side")
	if s.Connector != nil {
		w.writeString("connector")
		w.writeString(s.Connector.Id)
	}
	if s.DesignPiece != nil {
		w.writeString("designPiece")
		w.writeString(s.DesignPiece.Id)
	}
	w.writeString("piece")
	w.writeString(s.Piece.Id)
	return w.digest()
}

// 🔌HashConnection computes SHA-256 hash of a Connection entity.
func HashConnection(c Connection) string {
	w := &hashWriter{}
	w.writeString("Connection")
	if len(c.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(c.Attributes))
		for i, a := range c.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	w.writeString("parent")
	w.writeHash(HashSide(c.Parent))
	w.writeString("child")
	w.writeHash(HashSide(c.Child))
	if c.Description != nil {
		w.writeString("description")
		w.writeString(*c.Description)
	}
	// Connection float fields are non-optional in Go but optional in TS.
	// For hash compatibility, always write them (they're always present in JSON).
	w.writeString("gap")
	w.writeNumber(c.Gap)
	w.writeString("id")
	w.writeString(c.Id)
	w.writeString("rise")
	w.writeNumber(c.Rise)
	w.writeString("rotation")
	w.writeNumber(c.Rotation)
	w.writeString("shift")
	w.writeNumber(c.Shift)
	w.writeString("tilt")
	w.writeNumber(c.Tilt)
	w.writeString("turn")
	w.writeNumber(c.Turn)
	w.writeString("u")
	w.writeNumber(c.U)
	w.writeString("v")
	w.writeNumber(c.V)
	return w.digest()
}

// 🧩HashPiece computes SHA-256 hash of a Piece entity.
func HashPiece(p Piece) string {
	w := &hashWriter{}
	w.writeString("Piece")
	if len(p.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(p.Attributes))
		for i, a := range p.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if p.Center != nil {
		w.writeString("center")
		w.writeHash(HashCoordinate(*p.Center))
	}
	if p.Color != nil {
		w.writeString("color")
		w.writeString(*p.Color)
	}
	if p.Description != nil {
		w.writeString("description")
		w.writeString(*p.Description)
	}
	if p.Design != nil {
		w.writeString("design")
		w.writeString(p.Design.Id)
	}
	w.writeString("id")
	w.writeString(p.Id)
	if p.IsHidden != nil {
		w.writeString("isHidden")
		w.writeBool(*p.IsHidden)
	}
	if p.IsLocked != nil {
		w.writeString("isLocked")
		w.writeBool(*p.IsLocked)
	}
	if p.MirrorPlane != nil {
		w.writeString("mirrorPlane")
		w.writeHash(HashPlane(*p.MirrorPlane))
	}
	if p.Name != nil {
		w.writeString("name")
		w.writeString(*p.Name)
	}
	if p.Plane != nil {
		w.writeString("plane")
		w.writeHash(HashPlane(*p.Plane))
	}
	if len(p.Props) > 0 {
		w.writeString("props")
		hashes := make([]string, len(p.Props))
		for i, prop := range p.Props {
			hashes[i] = HashProp(prop)
		}
		w.writeHashList(hashes)
	}
	if p.Scale != nil {
		w.writeString("scale")
		w.writeNumber(*p.Scale)
	}
	if p.Type != nil {
		w.writeString("type")
		w.writeString(p.Type.Id)
	}
	return w.digest()
}

// 📐HashDesign computes SHA-256 Merkle hash of a Design entity.
func HashDesign(d Design) string {
	w := &hashWriter{}
	w.writeString("Design")
	if d.ActiveLayer != nil {
		w.writeString("activeLayer")
		w.writeString(d.ActiveLayer.Id)
	}
	if len(d.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(d.Attributes))
		for i, a := range d.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if len(d.Authors) > 0 {
		w.writeString("authors")
		ids := make([]string, len(d.Authors))
		for i, a := range d.Authors {
			ids[i] = a.Id
		}
		w.writeIdList(ids)
	}
	if d.CanMirror != nil {
		w.writeString("canMirror")
		w.writeBool(*d.CanMirror)
	}
	if d.CanScale != nil {
		w.writeString("canScale")
		w.writeBool(*d.CanScale)
	}
	if len(d.Concepts) > 0 {
		w.writeString("concepts")
		ids := make([]string, len(d.Concepts))
		for i, c := range d.Concepts {
			ids[i] = c.Id
		}
		w.writeIdList(ids)
	}
	if len(d.Connections) > 0 {
		w.writeString("connections")
		hashes := make([]string, len(d.Connections))
		for i, c := range d.Connections {
			hashes[i] = HashConnection(c)
		}
		w.writeHashList(hashes)
	}
	if d.Description != nil {
		w.writeString("description")
		w.writeString(*d.Description)
	}
	if d.Folder != nil {
		w.writeString("folder")
		w.writeString(*d.Folder)
	}
	if len(d.Groups) > 0 {
		w.writeString("groups")
		hashes := make([]string, len(d.Groups))
		for i, g := range d.Groups {
			hashes[i] = HashGroup(g)
		}
		w.writeHashList(hashes)
	}
	w.writeString("id")
	w.writeString(d.Id)
	if d.Icon != nil {
		w.writeString("icon")
		w.writeString(*d.Icon)
	}
	if d.Image != nil {
		w.writeString("image")
		w.writeString(*d.Image)
	}
	if d.IsAbstract != nil {
		w.writeString("isAbstract")
		w.writeBool(*d.IsAbstract)
	}
	if d.Typology.Id != "" {
		w.writeString("typology")
		w.writeString(d.Typology.Id)
	}
	if len(d.Layers) > 0 {
		w.writeString("layers")
		hashes := make([]string, len(d.Layers))
		for i, l := range d.Layers {
			hashes[i] = HashLayer(l)
		}
		w.writeHashList(hashes)
	}
	if d.Location != nil {
		w.writeString("location")
		w.writeString(d.Location.Id)
	}
	w.writeString("name")
	w.writeString(d.Name)
	if len(d.Families) > 0 {
		w.writeString("families")
		ids := make([]string, len(d.Families))
		for i, family := range d.Families {
			ids[i] = family.Id
		}
		w.writeIdList(ids)
	}
	if len(d.Pieces) > 0 {
		w.writeString("pieces")
		hashes := make([]string, len(d.Pieces))
		for i, p := range d.Pieces {
			hashes[i] = HashPiece(p)
		}
		w.writeHashList(hashes)
	}
	if len(d.Props) > 0 {
		w.writeString("props")
		hashes := make([]string, len(d.Props))
		for i, p := range d.Props {
			hashes[i] = HashProp(p)
		}
		w.writeHashList(hashes)
	}
	if len(d.Stats) > 0 {
		w.writeString("stats")
		hashes := make([]string, len(d.Stats))
		for i, s := range d.Stats {
			hashes[i] = HashStat(s)
		}
		w.writeHashList(hashes)
	}
	if d.Unit != nil {
		w.writeString("unit")
		w.writeString(*d.Unit)
	}
	return w.digest()
}

// 📦HashKit computes SHA-256 Merkle hash of a Kit entity.
func HashKit(k Kit) string {
	w := &hashWriter{}
	w.writeString("Kit")
	if len(k.Attributes) > 0 {
		w.writeString("attributes")
		hashes := make([]string, len(k.Attributes))
		for i, a := range k.Attributes {
			hashes[i] = HashAttribute(a)
		}
		w.writeHashList(hashes)
	}
	if len(k.Authors) > 0 {
		w.writeString("authors")
		hashes := make([]string, len(k.Authors))
		for i, a := range k.Authors {
			hashes[i] = HashAuthor(a)
		}
		w.writeHashList(hashes)
	}
	if len(k.Concepts) > 0 {
		w.writeString("concepts")
		hashes := make([]string, len(k.Concepts))
		for i, c := range k.Concepts {
			hashes[i] = HashConcept(c)
		}
		w.writeHashList(hashes)
	}
	if k.Description != nil {
		w.writeString("description")
		w.writeString(*k.Description)
	}
	if len(k.Typologies) > 0 {
		w.writeString("typologies")
		hashes := make([]string, len(k.Typologies))
		for i, topo := range k.Typologies {
			hashes[i] = HashTypology(topo)
		}
		w.writeHashList(hashes)
	}
	if len(k.Files) > 0 {
		w.writeString("files")
		hashes := make([]string, len(k.Files))
		for i, f := range k.Files {
			hashes[i] = HashFile(f)
		}
		w.writeHashList(hashes)
	}
	if len(k.Folders) > 0 {
		w.writeString("folders")
		hashes := make([]string, len(k.Folders))
		for i, f := range k.Folders {
			hashes[i] = HashFolder(f)
		}
		w.writeHashList(hashes)
	}
	w.writeString("id")
	w.writeString(k.Id)
	if k.Homepage != nil {
		w.writeString("homepage")
		w.writeString(*k.Homepage)
	}
	if k.Icon != nil {
		w.writeString("icon")
		w.writeString(*k.Icon)
	}
	if k.Image != nil {
		w.writeString("image")
		w.writeString(*k.Image)
	}
	if k.License != nil {
		w.writeString("license")
		w.writeString(*k.License)
	}
	w.writeString("name")
	w.writeString(k.Name)
	if len(k.Families) > 0 {
		w.writeString("families")
		hashes := make([]string, len(k.Families))
		for i, f := range k.Families {
			hashes[i] = HashFamily(f)
		}
		w.writeHashList(hashes)
	}
	if k.Preview != nil {
		w.writeString("preview")
		w.writeString(*k.Preview)
	}
	if len(k.Qualities) > 0 {
		w.writeString("qualities")
		hashes := make([]string, len(k.Qualities))
		for i, q := range k.Qualities {
			hashes[i] = HashQuality(q)
		}
		w.writeHashList(hashes)
	}
	if k.Remote != nil {
		w.writeString("remote")
		w.writeString(*k.Remote)
	}
	if len(k.Tags) > 0 {
		w.writeString("tags")
		hashes := make([]string, len(k.Tags))
		for i, t := range k.Tags {
			hashes[i] = HashTag(t)
		}
		w.writeHashList(hashes)
	}
	if k.Version != "" {
		w.writeString("version")
		w.writeString(k.Version)
	}
	return w.digest()
}

// #endregion 🎩Hash Entities

// #region 🔗Hash Diffs
// Deterministic SHA-256 Merkle hash functions for all diff types.

// #region 🐹Hash Diff Value Types
// Helper functions for writing diff fields.

func writeNullableStringDiff(w *hashWriter, key string, val *string, isSet bool) {
	if val != nil {
		w.writeString(key)
		w.writeString(*val)
	} else if isSet {
		w.writeString(key)
		w.writeBool(false)
	}
}

func writeOptStringDiff(w *hashWriter, key string, val *string) {
	if val != nil {
		w.writeString(key)
		w.writeString(*val)
	}
}

func writeOptNumberDiff(w *hashWriter, key string, val *float64) {
	if val != nil {
		w.writeString(key)
		w.writeNumber(*val)
	}
}

func writeOptIntNumberDiff(w *hashWriter, key string, val *int) {
	if val != nil {
		w.writeString(key)
		w.writeIntNumber(*val)
	}
}

func writeOptBoolDiff(w *hashWriter, key string, val *bool) {
	if val != nil {
		w.writeString(key)
		w.writeBool(*val)
	}
}

func hashCollectionDiffGeneric(
	tag string, updateTag string, entityKeyName string,
	hashEntityFn func(interface{}) string,
	hashDiffFn func(interface{}) string,
	removed []string,
	updated []struct {
		key  string
		diff interface{}
	},
	added []interface{},
) string {
	w := &hashWriter{}
	w.writeString(tag)
	if len(added) > 0 {
		w.writeString("added")
		hashes := make([]string, len(added))
		for i, e := range added {
			hashes[i] = hashEntityFn(e)
		}
		w.writeHashList(hashes)
	}
	if len(removed) > 0 {
		w.writeString("removed")
		w.writeIdList(removed)
	}
	if len(updated) > 0 {
		w.writeString("updated")
		keys := []string{entityKeyName, "diff"}
		sort.Strings(keys)
		updateHashes := make([]string, len(updated))
		for i, u := range updated {
			uw := &hashWriter{}
			uw.writeString(updateTag)
			for _, k := range keys {
				if k == "diff" {
					uw.writeString("diff")
					uw.writeHash(hashDiffFn(u.diff))
				} else {
					uw.writeString(k)
					uw.writeString(u.key)
				}
			}
			updateHashes[i] = uw.digest()
		}
		w.writeHashList(updateHashes)
	}
	return w.digest()
}

// #endregion 🐹Hash Diff Value Types

// #region ⚗️Hash Diff Entities
// Hash functions for all diff entity types.

func HashCoordinateDiff(d CoordinateDiff) string {
	w := &hashWriter{}
	w.writeString("CoordinateDiff")
	writeOptNumberDiff(w, "u", d.U)
	writeOptNumberDiff(w, "v", d.V)
	return w.digest()
}

func HashPointDiff(d PointDiff) string {
	w := &hashWriter{}
	w.writeString("PointDiff")
	writeOptNumberDiff(w, "x", d.X)
	writeOptNumberDiff(w, "y", d.Y)
	writeOptNumberDiff(w, "z", d.Z)
	return w.digest()
}

func HashVectorDiff(d VectorDiff) string {
	w := &hashWriter{}
	w.writeString("VectorDiff")
	writeOptNumberDiff(w, "x", d.X)
	writeOptNumberDiff(w, "y", d.Y)
	writeOptNumberDiff(w, "z", d.Z)
	return w.digest()
}

func HashPlaneDiff(d PlaneDiff) string {
	w := &hashWriter{}
	w.writeString("PlaneDiff")
	if d.Origin != nil {
		w.writeString("origin")
		w.writeHash(HashPointDiff(*d.Origin))
	}
	if d.XAxis != nil {
		w.writeString("xAxis")
		w.writeHash(HashVectorDiff(*d.XAxis))
	}
	if d.YAxis != nil {
		w.writeString("yAxis")
		w.writeHash(HashVectorDiff(*d.YAxis))
	}
	return w.digest()
}

func HashCameraDiff(d CameraDiff) string {
	w := &hashWriter{}
	w.writeString("CameraDiff")
	if d.Forward != nil {
		w.writeString("forward")
		w.writeHash(HashVectorDiff(*d.Forward))
	}
	if d.Position != nil {
		w.writeString("position")
		w.writeHash(HashPointDiff(*d.Position))
	}
	if d.Up != nil {
		w.writeString("up")
		w.writeHash(HashVectorDiff(*d.Up))
	}
	return w.digest()
}

func HashAttributeDiff(d AttributeDiff) string {
	w := &hashWriter{}
	w.writeString("AttributeDiff")
	writeOptStringDiff(w, "definition", d.Definition)
	writeOptStringDiff(w, "key", d.Key)
	writeOptStringDiff(w, "value", d.Value)
	return w.digest()
}

func HashAttributesDiff(d AttributesDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Attribute.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("AttributesDiff", "AttributeDiffUpdate", "attribute",
		func(e interface{}) string { return HashAttribute(e.(Attribute)) },
		func(d interface{}) string { return HashAttributeDiff(d.(AttributeDiff)) },
		removed, updated, added)
}

func HashLocationDiff(d LocationDiff) string {
	w := &hashWriter{}
	w.writeString("LocationDiff")
	writeOptNumberDiff(w, "altitude", d.Altitude)
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptNumberDiff(w, "latitude", d.Latitude)
	writeOptNumberDiff(w, "longitude", d.Longitude)
	return w.digest()
}

func HashAuthorDiff(d AuthorDiff) string {
	w := &hashWriter{}
	w.writeString("AuthorDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptStringDiff(w, "email", d.Email)
	writeOptStringDiff(w, "name", d.Name)
	return w.digest()
}

func HashAuthorsDiff(d AuthorsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Author.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("AuthorsDiff", "AuthorDiffUpdate", "author",
		func(e interface{}) string { return HashAuthor(e.(Author)) },
		func(d interface{}) string { return HashAuthorDiff(d.(AuthorDiff)) },
		removed, updated, added)
}

func HashFileDiff(d FileDiff) string {
	w := &hashWriter{}
	w.writeString("FileDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptStringDiff(w, "blob", d.Blob)
	writeOptStringDiff(w, "description", d.Description)
	if d.Folder != nil {
		w.writeString("folder")
		w.writeString(d.Folder.Id)
	}
	writeOptStringDiff(w, "hash", d.Hash)
	writeOptStringDiff(w, "name", d.Name)
	writeOptStringDiff(w, "remote", d.Remote)
	if d.Size != nil {
		w.writeString("size")
		w.writeIntNumber(int(*d.Size))
	}
	return w.digest()
}

func HashFilesDiff(d FilesDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.File.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("FilesDiff", "FileDiffUpdate", "file",
		func(e interface{}) string { return HashFile(e.(File)) },
		func(d interface{}) string { return HashFileDiff(d.(FileDiff)) },
		removed, updated, added)
}

func HashFolderDiff(d FolderDiff) string {
	w := &hashWriter{}
	w.writeString("FolderDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptStringDiff(w, "description", d.Description)
	writeOptStringDiff(w, "name", d.Name)
	if d.Parent != nil {
		w.writeString("parent")
		w.writeString(d.Parent.Id)
	}
	return w.digest()
}

func HashFoldersDiff(d FoldersDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Folder.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("FoldersDiff", "FolderDiffUpdate", "folder",
		func(e interface{}) string { return HashFolder(e.(Folder)) },
		func(d interface{}) string { return HashFolderDiff(d.(FolderDiff)) },
		removed, updated, added)
}

func HashBenchmarkDiff(d BenchmarkDiff) string {
	w := &hashWriter{}
	w.writeString("BenchmarkDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptStringDiff(w, "definition", d.Definition)
	writeOptStringDiff(w, "icon", d.Icon)
	writeOptNumberDiff(w, "max", d.Max)
	writeOptBoolDiff(w, "maxExcluded", d.MaxExcluded)
	writeOptNumberDiff(w, "min", d.Min)
	writeOptBoolDiff(w, "minExcluded", d.MinExcluded)
	writeOptStringDiff(w, "name", d.Name)
	return w.digest()
}

func HashBenchmarksDiff(d BenchmarksDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Benchmark.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("BenchmarksDiff", "BenchmarkDiffUpdate", "benchmark",
		func(e interface{}) string { return HashBenchmark(e.(Benchmark)) },
		func(d interface{}) string { return HashBenchmarkDiff(d.(BenchmarkDiff)) },
		removed, updated, added)
}

func HashQualityDiff(d QualityDiff) string {
	w := &hashWriter{}
	w.writeString("QualityDiff")
	if d.Benchmarks != nil {
		w.writeString("benchmarks")
		w.writeHash(HashBenchmarksDiff(*d.Benchmarks))
	}
	writeOptBoolDiff(w, "canScale", d.CanScale)
	writeOptStringDiff(w, "defaultImperialUnit", d.DefaultImperialUnit)
	writeOptStringDiff(w, "defaultSiUnit", d.DefaultSiUnit)
	writeOptNumberDiff(w, "defaultValue", d.DefaultValue)
	writeOptStringDiff(w, "description", d.Description)
	writeOptStringDiff(w, "formula", d.Formula)
	writeOptStringDiff(w, "icon", d.Icon)
	writeOptStringDiff(w, "image", d.Image)
	writeOptBoolDiff(w, "isMaxExcluded", d.IsMaxExcluded)
	writeOptBoolDiff(w, "isMinExcluded", d.IsMinExcluded)
	writeOptStringDiff(w, "key", d.Key)
	if d.Kind != nil {
		w.writeString("kind")
		w.writeIntNumber(int(*d.Kind))
	}
	writeOptNumberDiff(w, "max", d.Max)
	writeOptNumberDiff(w, "min", d.Min)
	writeOptStringDiff(w, "name", d.Name)
	writeOptStringDiff(w, "unit", d.Unit)
	writeOptStringDiff(w, "uri", d.Uri)
	return w.digest()
}

func HashQualitiesDiff(d QualitiesDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Quality.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("QualitiesDiff", "QualityDiffUpdate", "quality",
		func(e interface{}) string { return HashQuality(e.(Quality)) },
		func(d interface{}) string { return HashQualityDiff(d.(QualityDiff)) },
		removed, updated, added)
}

func HashPortDiff(d PortDiff) string {
	w := &hashWriter{}
	w.writeString("PortDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	if len(d.CompatiblePorts) > 0 {
		w.writeString("compatiblePorts")
		ids := make([]string, len(d.CompatiblePorts))
		for i, cp := range d.CompatiblePorts {
			ids[i] = cp.Id
		}
		w.writeIdList(ids)
	}
	writeNullableStringDiff(w, "description", d.Description, d.HasField("description"))
	writeNullableStringDiff(w, "icon", d.Icon, d.HasField("icon"))
	if d.MaxChildren != nil || d.HasField("maxChildren") {
		w.writeString("maxChildren")
		if d.MaxChildren != nil {
			w.writeIntNumber(*d.MaxChildren)
		} else {
			w.writeString("null")
		}
	}
	writeOptStringDiff(w, "name", d.Name)
	return w.digest()
}

func HashPortsDiff(d PortsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Port.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("PortsDiff", "PortDiffUpdate", "port",
		func(e interface{}) string { return HashPort(e.(Port)) },
		func(d interface{}) string { return HashPortDiff(d.(PortDiff)) },
		removed, updated, added)
}

func HashFamilyDiff(d FamilyDiff) string {
	w := &hashWriter{}
	w.writeString("FamilyDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeNullableStringDiff(w, "description", d.Description, d.HasField("description"))
	writeNullableStringDiff(w, "icon", d.Icon, d.HasField("icon"))
	writeOptStringDiff(w, "name", d.Name)
	if d.Ports != nil {
		w.writeString("ports")
		w.writeHash(HashPortsDiff(*d.Ports))
	}
	return w.digest()
}

func HashFamiliesDiff(d FamiliesDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Family.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("FamiliesDiff", "FamilyDiffUpdate", "family",
		func(e interface{}) string { return HashFamily(e.(Family)) },
		func(d interface{}) string { return HashFamilyDiff(d.(FamilyDiff)) },
		removed, updated, added)
}

func HashTypologyDiff(d TypologyDiff) string {
	w := &hashWriter{}
	w.writeString("TypologyDiff")
	writeOptStringDiff(w, "description", d.Description)
	if d.Designs != nil {
		w.writeString("designs")
		w.writeHash(HashDesignsDiff(*d.Designs))
	}
	writeOptStringDiff(w, "folder", d.Folder)
	writeOptStringDiff(w, "icon", d.Icon)
	writeOptStringDiff(w, "name", d.Name)
	if d.Types != nil {
		w.writeString("types")
		w.writeHash(HashTypesDiff(*d.Types))
	}
	return w.digest()
}

func HashTypologiesDiff(d TypologiesDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Typology.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("TypologiesDiff", "TypologyDiffUpdate", "typology",
		func(e interface{}) string { return HashTypology(e.(Typology)) },
		func(d interface{}) string { return HashTypologyDiff(d.(TypologyDiff)) },
		removed, updated, added)
}

func HashPropDiff(d PropDiff) string {
	w := &hashWriter{}
	w.writeString("PropDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	if d.Quality != nil {
		w.writeString("quality")
		w.writeString(d.Quality.Id)
	}
	writeOptStringDiff(w, "unit", d.Unit)
	writeOptStringDiff(w, "value", d.Value)
	return w.digest()
}

func HashPropsDiff(d PropsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Prop.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("PropsDiff", "PropDiffUpdate", "prop",
		func(e interface{}) string { return HashProp(e.(Prop)) },
		func(d interface{}) string { return HashPropDiff(d.(PropDiff)) },
		removed, updated, added)
}

func HashTagDiff(d TagDiff) string {
	w := &hashWriter{}
	w.writeString("TagDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeNullableStringDiff(w, "description", d.Description, d.HasField("description"))
	writeNullableStringDiff(w, "icon", d.Icon, d.HasField("icon"))
	writeOptStringDiff(w, "name", d.Name)
	return w.digest()
}

func HashTagsDiff(d TagsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Tag.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("TagsDiff", "TagDiffUpdate", "tag",
		func(e interface{}) string { return HashTag(e.(Tag)) },
		func(d interface{}) string { return HashTagDiff(d.(TagDiff)) },
		removed, updated, added)
}

func HashConceptDiff(d ConceptDiff) string {
	w := &hashWriter{}
	w.writeString("ConceptDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeNullableStringDiff(w, "description", d.Description, d.HasField("description"))
	writeNullableStringDiff(w, "icon", d.Icon, d.HasField("icon"))
	writeOptStringDiff(w, "name", d.Name)
	return w.digest()
}

func HashConceptsDiff(d ConceptsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Concept.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("ConceptsDiff", "ConceptDiffUpdate", "concept",
		func(e interface{}) string { return HashConcept(e.(Concept)) },
		func(d interface{}) string { return HashConceptDiff(d.(ConceptDiff)) },
		removed, updated, added)
}

func HashRepresentationDiff(d RepresentationDiff) string {
	w := &hashWriter{}
	w.writeString("RepresentationDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptStringDiff(w, "description", d.Description)
	if d.File != nil {
		w.writeString("file")
		w.writeString(d.File.Id)
	}
	writeOptStringDiff(w, "name", d.Name)
	if len(d.Tags) > 0 {
		w.writeString("tags")
		ids := make([]string, len(d.Tags))
		for i, t := range d.Tags {
			ids[i] = t.Id
		}
		w.writeIdList(ids)
	}
	return w.digest()
}

func HashRepresentationsDiff(d RepresentationsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Representation.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("RepresentationsDiff", "RepresentationDiffUpdate", "representation",
		func(e interface{}) string { return HashRepresentation(e.(Representation)) },
		func(d interface{}) string { return HashRepresentationDiff(d.(RepresentationDiff)) },
		removed, updated, added)
}

func HashConnectorDiff(d ConnectorDiff) string {
	w := &hashWriter{}
	w.writeString("ConnectorDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptStringDiff(w, "description", d.Description)
	if d.Direction != nil {
		w.writeString("direction")
		w.writeHash(HashVectorDiff(*d.Direction))
	}
	writeOptBoolDiff(w, "mandatory", d.Mandatory)
	writeOptStringDiff(w, "name", d.Name)
	if d.Point != nil {
		w.writeString("point")
		w.writeHash(HashPointDiff(*d.Point))
	}
	if d.Port != nil {
		w.writeString("port")
		w.writeString(d.Port.Id)
	}
	if d.Props != nil {
		w.writeString("props")
		w.writeHash(HashPropsDiff(*d.Props))
	}
	writeOptNumberDiff(w, "t", d.T)
	return w.digest()
}

func HashConnectorsDiff(d ConnectorsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Connector.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("ConnectorsDiff", "ConnectorDiffUpdate", "connector",
		func(e interface{}) string { return HashConnector(e.(Connector)) },
		func(d interface{}) string { return HashConnectorDiff(d.(ConnectorDiff)) },
		removed, updated, added)
}

func HashTypeDiff(d TypeDiff) string {
	w := &hashWriter{}
	w.writeString("TypeDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	if len(d.Authors) > 0 {
		w.writeString("authors")
		ids := make([]string, len(d.Authors))
		for i, a := range d.Authors {
			ids[i] = a.Id
		}
		w.writeIdList(ids)
	} else if d.HasField("authors") {
		w.writeString("authors")
		w.writeBool(false)
	}
	if len(d.Concepts) > 0 {
		w.writeString("concepts")
		ids := make([]string, len(d.Concepts))
		for i, c := range d.Concepts {
			ids[i] = c.Id
		}
		w.writeIdList(ids)
	} else if d.HasField("concepts") {
		w.writeString("concepts")
		w.writeBool(false)
	}
	if d.Connectors != nil {
		w.writeString("connectors")
		w.writeHash(HashConnectorsDiff(*d.Connectors))
	}
	writeNullableStringDiff(w, "description", d.Description, d.HasField("description"))
	writeNullableStringDiff(w, "folder", d.Folder, d.HasField("folder"))
	writeNullableStringDiff(w, "icon", d.Icon, d.HasField("icon"))
	writeNullableStringDiff(w, "image", d.Image, d.HasField("image"))
	writeOptBoolDiff(w, "isAbstract", d.IsAbstract)
	if d.Location != nil {
		w.writeString("location")
		w.writeString(d.Location.Id)
	} else if d.HasField("location") {
		w.writeString("location")
		w.writeBool(false)
	}
	if d.Representations != nil {
		w.writeString("representations")
		w.writeHash(HashRepresentationsDiff(*d.Representations))
	}
	writeOptStringDiff(w, "name", d.Name)
	if len(d.Families) > 0 {
		w.writeString("families")
		ids := make([]string, len(d.Families))
		for i, family := range d.Families {
			ids[i] = family.Id
		}
		w.writeIdList(ids)
	} else if d.HasField("families") {
		w.writeString("families")
		w.writeBool(false)
	}
	if d.Props != nil {
		w.writeString("props")
		w.writeHash(HashPropsDiff(*d.Props))
	}
	writeOptIntNumberDiff(w, "stock", d.Stock)
	writeOptStringDiff(w, "unit", d.Unit)
	writeOptBoolDiff(w, "virtual", d.Virtual)
	return w.digest()
}

func HashTypesDiff(d TypesDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Type.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("TypesDiff", "TypeDiffUpdate", "type",
		func(e interface{}) string { return HashType(e.(Type)) },
		func(d interface{}) string { return HashTypeDiff(d.(TypeDiff)) },
		removed, updated, added)
}

func HashSideDiff(d SideDiff) string {
	w := &hashWriter{}
	w.writeString("SideDiff")
	if d.Connector != nil {
		w.writeString("connector")
		w.writeString(d.Connector.Id)
	}
	if d.DesignPiece != nil {
		w.writeString("designPiece")
		w.writeString(d.DesignPiece.Id)
	}
	if d.Piece != nil {
		w.writeString("piece")
		w.writeString(d.Piece.Id)
	}
	return w.digest()
}

func HashLayerDiff(d LayerDiff) string {
	w := &hashWriter{}
	w.writeString("LayerDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptStringDiff(w, "color", d.Color)
	writeOptStringDiff(w, "description", d.Description)
	writeOptBoolDiff(w, "isHidden", d.IsHidden)
	writeOptBoolDiff(w, "isLocked", d.IsLocked)
	writeOptStringDiff(w, "path", d.Path)
	return w.digest()
}

func HashLayersDiff(d LayersDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Layer.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("LayersDiff", "LayerDiffUpdate", "layer",
		func(e interface{}) string { return HashLayer(e.(Layer)) },
		func(d interface{}) string { return HashLayerDiff(d.(LayerDiff)) },
		removed, updated, added)
}

func HashGroupDiff(d GroupDiff) string {
	w := &hashWriter{}
	w.writeString("GroupDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptStringDiff(w, "color", d.Color)
	writeOptStringDiff(w, "description", d.Description)
	writeOptStringDiff(w, "name", d.Name)
	if len(d.Pieces) > 0 {
		w.writeString("pieces")
		ids := make([]string, len(d.Pieces))
		for i, p := range d.Pieces {
			ids[i] = p.Id
		}
		w.writeIdList(ids)
	}
	return w.digest()
}

func HashGroupsDiff(d GroupsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Group.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("GroupsDiff", "GroupDiffUpdate", "group",
		func(e interface{}) string { return HashGroup(e.(Group)) },
		func(d interface{}) string { return HashGroupDiff(d.(GroupDiff)) },
		removed, updated, added)
}

func HashStatDiff(d StatDiff) string {
	w := &hashWriter{}
	w.writeString("StatDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	writeOptNumberDiff(w, "max", d.Max)
	writeOptNumberDiff(w, "min", d.Min)
	if d.Quality != nil {
		w.writeString("quality")
		w.writeString(d.Quality.Id)
	}
	writeOptStringDiff(w, "unit", d.Unit)
	return w.digest()
}

func HashStatsDiff(d StatsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Stat.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("StatsDiff", "StatDiffUpdate", "stat",
		func(e interface{}) string { return HashStat(e.(Stat)) },
		func(d interface{}) string { return HashStatDiff(d.(StatDiff)) },
		removed, updated, added)
}

func HashConnectionDiff(d ConnectionDiff) string {
	w := &hashWriter{}
	w.writeString("ConnectionDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	if d.Parent != nil {
		w.writeString("parent")
		w.writeHash(HashSideDiff(*d.Parent))
	}
	if d.Child != nil {
		w.writeString("child")
		w.writeHash(HashSideDiff(*d.Child))
	}
	writeOptStringDiff(w, "description", d.Description)
	writeOptNumberDiff(w, "gap", d.Gap)
	writeOptNumberDiff(w, "rise", d.Rise)
	writeOptNumberDiff(w, "rotation", d.Rotation)
	writeOptNumberDiff(w, "shift", d.Shift)
	writeOptNumberDiff(w, "tilt", d.Tilt)
	writeOptNumberDiff(w, "turn", d.Turn)
	writeOptNumberDiff(w, "u", d.U)
	writeOptNumberDiff(w, "v", d.V)
	return w.digest()
}

func HashConnectionsDiff(d ConnectionsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Connection.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("ConnectionsDiff", "ConnectionDiffUpdate", "connection",
		func(e interface{}) string { return HashConnection(e.(Connection)) },
		func(d interface{}) string { return HashConnectionDiff(d.(ConnectionDiff)) },
		removed, updated, added)
}

func HashPieceDiff(d PieceDiff) string {
	w := &hashWriter{}
	w.writeString("PieceDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	if d.Center != nil {
		w.writeString("center")
		w.writeHash(HashCoordinateDiff(*d.Center))
	}
	writeOptStringDiff(w, "color", d.Color)
	writeOptStringDiff(w, "description", d.Description)
	if d.Design != nil {
		w.writeString("design")
		w.writeString(d.Design.Id)
	}
	writeOptBoolDiff(w, "isHidden", d.IsHidden)
	writeOptBoolDiff(w, "isLocked", d.IsLocked)
	if d.MirrorPlane != nil {
		w.writeString("mirrorPlane")
		w.writeHash(HashPlaneDiff(*d.MirrorPlane))
	}
	writeOptStringDiff(w, "name", d.Name)
	if d.Plane != nil {
		w.writeString("plane")
		w.writeHash(HashPlaneDiff(*d.Plane))
	}
	if d.Props != nil {
		w.writeString("props")
		w.writeHash(HashPropsDiff(*d.Props))
	}
	writeOptNumberDiff(w, "scale", d.Scale)
	if d.Type != nil {
		w.writeString("type")
		w.writeString(d.Type.Id)
	}
	return w.digest()
}

func HashPiecesDiff(d PiecesDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Piece.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("PiecesDiff", "PieceDiffUpdate", "piece",
		func(e interface{}) string { return HashPiece(e.(Piece)) },
		func(d interface{}) string { return HashPieceDiff(d.(PieceDiff)) },
		removed, updated, added)
}

func HashDesignDiff(d DesignDiff) string {
	w := &hashWriter{}
	w.writeString("DesignDiff")
	if d.ActiveLayer != nil {
		w.writeString("activeLayer")
		w.writeString(d.ActiveLayer.Id)
	}
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	if len(d.Authors) > 0 {
		w.writeString("authors")
		ids := make([]string, len(d.Authors))
		for i, a := range d.Authors {
			ids[i] = a.Id
		}
		w.writeIdList(ids)
	}
	writeOptBoolDiff(w, "canMirror", d.CanMirror)
	writeOptBoolDiff(w, "canScale", d.CanScale)
	if len(d.Concepts) > 0 {
		w.writeString("concepts")
		ids := make([]string, len(d.Concepts))
		for i, c := range d.Concepts {
			ids[i] = c.Id
		}
		w.writeIdList(ids)
	}
	if d.Connections != nil {
		w.writeString("connections")
		w.writeHash(HashConnectionsDiff(*d.Connections))
	}
	writeOptStringDiff(w, "description", d.Description)
	writeOptStringDiff(w, "folder", d.Folder)
	if d.Groups != nil {
		w.writeString("groups")
		w.writeHash(HashGroupsDiff(*d.Groups))
	}
	writeOptStringDiff(w, "icon", d.Icon)
	writeOptStringDiff(w, "image", d.Image)
	writeOptBoolDiff(w, "isAbstract", d.IsAbstract)
	if d.Layers != nil {
		w.writeString("layers")
		w.writeHash(HashLayersDiff(*d.Layers))
	}
	if d.Location != nil {
		w.writeString("location")
		w.writeString(d.Location.Id)
	}
	writeOptStringDiff(w, "name", d.Name)
	if len(d.Families) > 0 {
		w.writeString("families")
		ids := make([]string, len(d.Families))
		for i, family := range d.Families {
			ids[i] = family.Id
		}
		w.writeIdList(ids)
	}
	if d.Pieces != nil {
		w.writeString("pieces")
		w.writeHash(HashPiecesDiff(*d.Pieces))
	}
	if d.Props != nil {
		w.writeString("props")
		w.writeHash(HashPropsDiff(*d.Props))
	}
	if d.Stats != nil {
		w.writeString("stats")
		w.writeHash(HashStatsDiff(*d.Stats))
	}
	writeOptStringDiff(w, "unit", d.Unit)
	if d.View != nil {
		w.writeString("view")
		w.writeHash(HashCameraDiff(*d.View))
	}
	return w.digest()
}

func HashDesignsDiff(d DesignsDiff) string {
	removed := make([]string, len(d.Removed))
	for i, r := range d.Removed {
		removed[i] = r.Id
	}
	var updated []struct {
		key  string
		diff interface{}
	}
	for _, u := range d.Updated {
		updated = append(updated, struct {
			key  string
			diff interface{}
		}{key: u.Design.Id, diff: u.Diff})
	}
	var added []interface{}
	for _, a := range d.Added {
		added = append(added, a)
	}
	return hashCollectionDiffGeneric("DesignsDiff", "DesignDiffUpdate", "design",
		func(e interface{}) string { return HashDesign(e.(Design)) },
		func(d interface{}) string { return HashDesignDiff(d.(DesignDiff)) },
		removed, updated, added)
}

func HashKitDiff(d KitDiff) string {
	w := &hashWriter{}
	w.writeString("KitDiff")
	if d.Attributes != nil {
		w.writeString("attributes")
		w.writeHash(HashAttributesDiff(*d.Attributes))
	}
	if d.Authors != nil {
		w.writeString("authors")
		w.writeHash(HashAuthorsDiff(*d.Authors))
	}
	if d.Concepts != nil {
		w.writeString("concepts")
		w.writeHash(HashConceptsDiff(*d.Concepts))
	}
	writeNullableStringDiff(w, "description", d.Description, d.HasField("description"))
	if d.Files != nil {
		w.writeString("files")
		w.writeHash(HashFilesDiff(*d.Files))
	}
	if d.Folders != nil {
		w.writeString("folders")
		w.writeHash(HashFoldersDiff(*d.Folders))
	}
	writeNullableStringDiff(w, "homepage", d.Homepage, d.HasField("homepage"))
	writeNullableStringDiff(w, "icon", d.Icon, d.HasField("icon"))
	writeNullableStringDiff(w, "image", d.Image, d.HasField("image"))
	writeNullableStringDiff(w, "license", d.License, d.HasField("license"))
	writeOptStringDiff(w, "name", d.Name)
	if d.Families != nil {
		w.writeString("families")
		w.writeHash(HashFamiliesDiff(*d.Families))
	}
	if d.Typologies != nil {
		w.writeString("typologies")
		w.writeHash(HashTypologiesDiff(*d.Typologies))
	}
	writeNullableStringDiff(w, "preview", d.Preview, d.HasField("preview"))
	if d.Qualities != nil {
		w.writeString("qualities")
		w.writeHash(HashQualitiesDiff(*d.Qualities))
	}
	writeNullableStringDiff(w, "remote", d.Remote, d.HasField("remote"))
	if d.Tags != nil {
		w.writeString("tags")
		w.writeHash(HashTagsDiff(*d.Tags))
	}
	writeOptStringDiff(w, "version", d.Version)
	return w.digest()
}

// #endregion ⚗️Hash Diff Entities

// #endregion 🔗Hash Diffs

// #endregion 🖥️Hash

// #region 🔍Helpers

// 🧱FindTypeInKit returns a pointer to the type with the given ID or nil.
func FindTypeInKit(kit *Kit, typeId string) *Type {
	for i := range kit.Types {
		if kit.Types[i].Id == typeId {
			return &kit.Types[i]
		}
	}
	return nil
}

// 📐FindDesignInKit returns a pointer to the design with the given ID or nil.
func FindDesignInKit(kit *Kit, designId string) *Design {
	for i := range kit.Designs {
		if kit.Designs[i].Id == designId {
			return &kit.Designs[i]
		}
	}
	return nil
}

// 🧩FindPieceInDesign returns a pointer to the piece with the given ID or nil.
func FindPieceInDesign(design *Design, pieceId string) *Piece {
	for i := range design.Pieces {
		if design.Pieces[i].Id == pieceId {
			return &design.Pieces[i]
		}
	}
	return nil
}

// 🔗FindConnectionInDesign returns a pointer to the connection with the given ID or nil.
func FindConnectionInDesign(design *Design, connectionId string) *Connection {
	for i := range design.Connections {
		if design.Connections[i].Id == connectionId {
			return &design.Connections[i]
		}
	}
	return nil
}

// 🔌FindConnectorInType returns a pointer to the connector with the given ID or nil.
func FindConnectorInType(typ *Type, connectorId string) *Connector {
	for i := range typ.Connectors {
		if typ.Connectors[i].Id == connectorId {
			return &typ.Connectors[i]
		}
	}
	return nil
}

// 📄FindFileInKit returns a pointer to the file with the given ID or nil.
func FindFileInKit(kit *Kit, fileId string) *File {
	for i := range kit.Files {
		if kit.Files[i].Id == fileId {
			return &kit.Files[i]
		}
	}
	return nil
}

// 📁FindFolderInKit returns a pointer to the folder with the given ID or nil.
func FindFolderInKit(kit *Kit, folderId string) *Folder {
	for i := range kit.Folders {
		if kit.Folders[i].Id == folderId {
			return &kit.Folders[i]
		}
	}
	return nil
}

// 🔬FindQualityInKit returns a pointer to the quality with the given ID or nil.
func FindQualityInKit(kit *Kit, qualityId string) *Quality {
	for i := range kit.Qualities {
		if kit.Qualities[i].Id == qualityId {
			return &kit.Qualities[i]
		}
	}
	return nil
}

// ⚓FindPortInKit returns a pointer to the port with the given ID or nil.
func FindPortInKit(kit *Kit, interfaceId string) *Port {
	for familyIndex := range kit.Families {
		for portIndex := range kit.Families[familyIndex].Ports {
			if kit.Families[familyIndex].Ports[portIndex].Id == interfaceId {
				return &kit.Families[familyIndex].Ports[portIndex]
			}
		}
	}
	return nil
}

// 👪FindFamilyInKit returns a pointer to the family with the given ID or nil.
func FindFamilyInKit(kit *Kit, familyId string) *Family {
	for i := range kit.Families {
		if kit.Families[i].Id == familyId {
			return &kit.Families[i]
		}
	}
	return nil
}

// ⚓AllPortsInKit returns the family-owned ports as a flat read-only slice.
func AllPortsInKit(kit *Kit) []Port {
	ports := make([]Port, 0)
	for _, family := range kit.Families {
		ports = append(ports, family.Ports...)
	}
	return ports
}

// 🏷️FindTagInKit returns a pointer to the tag with the given ID or nil.
func FindTagInKit(kit *Kit, tagId string) *Tag {
	for i := range kit.Tags {
		if kit.Tags[i].Id == tagId {
			return &kit.Tags[i]
		}
	}
	return nil
}

// 💡FindConceptInKit returns a pointer to the concept with the given ID or nil.
func FindConceptInKit(kit *Kit, conceptId string) *Concept {
	for i := range kit.Concepts {
		if kit.Concepts[i].Id == conceptId {
			return &kit.Concepts[i]
		}
	}
	return nil
}

// ✍️FindAuthorInKit returns a pointer to the author with the given ID or nil.
func FindAuthorInKit(kit *Kit, authorId string) *Author {
	for i := range kit.Authors {
		if kit.Authors[i].Id == authorId {
			return &kit.Authors[i]
		}
	}
	return nil
}

// 🔬For each piece, uses the piece-level prop if present, otherwise falls back to the type-level prop.
func SumQualityInDesign(kit *Kit, designId string, qualityId string) float64 {
	design := FindDesignInKit(kit, designId)
	if design == nil {
		return 0
	}
	total := 0.0
	for _, piece := range design.Pieces {
		var found bool
		for _, prop := range piece.Props {
			if prop.Quality.Id == qualityId {
				val, err := strconv.ParseFloat(prop.Value, 64)
				if err == nil {
					total += val
				}
				found = true
				break
			}
		}
		if found {
			continue
		}
		if piece.Type == nil {
			continue
		}
		typ := FindTypeInKit(kit, piece.Type.Id)
		if typ == nil {
			continue
		}
		for _, prop := range typ.Props {
			if prop.Quality.Id == qualityId {
				val, err := strconv.ParseFloat(prop.Value, 64)
				if err == nil {
					total += val
				}
				break
			}
		}
	}
	return total
}

// #endregion 🔍Helpers

// #region 🗡️Factories

// 🆕NewKit creates a new kit with the given name and a generated ID.
func NewKit(name string) Kit {
	now := ""
	return Kit{
		Id:      Id(),
		Name:      name,
		Version:   "0.0.1",
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// 🧱NewType creates a new type with the given name and a generated ID.
func NewType(name string) Type {
	now := ""
	return Type{
		Id:      Id(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// 📐NewDesign creates a new design with the given name and a generated ID.
func NewDesign(name string) Design {
	now := ""
	return Design{
		Id:      Id(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// 🧩NewPiece creates a new piece with a generated ID.
func NewPiece() Piece {
	return Piece{
		Id: Id(),
	}
}

// 🔗NewConnection creates a new connection between two pieces by their IDs.
func NewConnection(parentPieceId, childPieceId string) Connection {
	return Connection{
		Id:     Id(),
		Parent: Side{Piece: PieceId{Id: parentPieceId}},
		Child:  Side{Piece: PieceId{Id: childPieceId}},
	}
}

// 🎛️NewConnector creates a new connector with position, direction and parameter t.
func NewConnector(point Point, direction Vector, t float64) Connector {
	return Connector{
		Id:      Id(),
		Point:     point,
		Direction: direction,
		T:         t,
	}
}

// 📄NewFile creates a new file with the given name and a generated ID.
func NewFile(name string) File {
	now := ""
	return File{
		Id:      Id(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// 📁NewFolder creates a new folder with the given name and a generated ID.
func NewFolder(name string) Folder {
	now := ""
	return Folder{
		Id:      Id(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// 🔬NewQuality creates a new quality with the given key, name and a generated ID.
func NewQuality(key, name string) Quality {
	now := ""
	return Quality{
		Id:      Id(),
		Key:       key,
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// ⚓NewPort creates a new port with the given name and a generated ID.
func NewPort(name string) Port {
	now := ""
	return Port{
		Id:      Id(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// 🏷️NewTag creates a new tag with the given name and a generated ID.
func NewTag(name string) Tag {
	now := ""
	return Tag{
		Id:      Id(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// 💡NewConcept creates a new concept with the given name and a generated ID.
func NewConcept(name string) Concept {
	now := ""
	return Concept{
		Id:      Id(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// ✍️NewAuthor creates a new author with the given name and a generated ID.
func NewAuthor(name string) Author {
	now := ""
	return Author{
		Id:      Id(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// #endregion 🗡️Factories

// #region 🎪Kit Operations
// Kit Operations MUST provide comparison, diffing, and application of kit changes.

// 🧱AreKitsEqual compares two kits for structural equality.
func AreKitsEqual(a, b Kit) bool {
	if a.Id != b.Id || a.Name != b.Name || a.Version != b.Version {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if normalizeStr(a.Icon) != normalizeStr(b.Icon) {
		return false
	}
	if normalizeStr(a.Image) != normalizeStr(b.Image) {
		return false
	}
	if normalizeStr(a.Remote) != normalizeStr(b.Remote) {
		return false
	}
	if normalizeStr(a.Homepage) != normalizeStr(b.Homepage) {
		return false
	}
	if normalizeStr(a.License) != normalizeStr(b.License) {
		return false
	}
	if normalizeStr(a.Preview) != normalizeStr(b.Preview) {
		return false
	}
	if len(a.Types) != len(b.Types) {
		return false
	}
	for _, ta := range a.Types {
		found := false
		for _, tb := range b.Types {
			if ta.Id == tb.Id {
				if !areTypesEqual(ta, tb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Designs) != len(b.Designs) {
		return false
	}
	for _, da := range a.Designs {
		found := false
		for _, db := range b.Designs {
			if da.Id == db.Id {
				if !areDesignsEqual(da, db) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Tags) != len(b.Tags) {
		return false
	}
	for _, ta := range a.Tags {
		found := false
		for _, tb := range b.Tags {
			if ta.Id == tb.Id {
				if !areTagsEqual(ta, tb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Concepts) != len(b.Concepts) {
		return false
	}
	for _, ca := range a.Concepts {
		found := false
		for _, cb := range b.Concepts {
			if ca.Id == cb.Id {
				if !areConceptsEqual(ca, cb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Families) != len(b.Families) {
		return false
	}
	for _, fa := range a.Families {
		found := false
		for _, fb := range b.Families {
			if fa.Id == fb.Id {
				if !areFamiliesEqual(fa, fb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Files) != len(b.Files) {
		return false
	}
	for _, fa := range a.Files {
		found := false
		for _, fb := range b.Files {
			if fa.Id == fb.Id {
				if !areFilesEqual(fa, fb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Folders) != len(b.Folders) {
		return false
	}
	for _, fa := range a.Folders {
		found := false
		for _, fb := range b.Folders {
			if fa.Id == fb.Id {
				if !areFoldersEqual(fa, fb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Authors) != len(b.Authors) {
		return false
	}
	for _, aa := range a.Authors {
		found := false
		for _, ab := range b.Authors {
			if aa.Id == ab.Id {
				if !areAuthorsEqual(aa, ab) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

// 📦AreKitDiffsEqual compares two kit diffs for structural equality.
func AreKitDiffsEqual(a, b KitDiff) bool {
	if (a.Name == nil) != (b.Name == nil) {
		return false
	}
	if a.Name != nil && *a.Name != *b.Name {
		return false
	}
	if (a.Version == nil) != (b.Version == nil) {
		return false
	}
	if a.Version != nil && *a.Version != *b.Version {
		return false
	}
	if (a.Description == nil) != (b.Description == nil) {
		return false
	}
	if a.Description != nil && *a.Description != *b.Description {
		return false
	}
	if !areTypologiesDiffsEqual(a.Typologies, b.Typologies) {
		return false
	}
	if !areTagsDiffsEqual(a.Tags, b.Tags) {
		return false
	}
	if !areConceptsDiffsEqual(a.Concepts, b.Concepts) {
		return false
	}
	if !areFamiliesDiffsEqual(a.Families, b.Families) {
		return false
	}
	if !areFilesDiffsEqual(a.Files, b.Files) {
		return false
	}
	if !areFoldersDiffsEqual(a.Folders, b.Folders) {
		return false
	}
	if !areAuthorsDiffsEqual(a.Authors, b.Authors) {
		return false
	}
	return true
}

func areTypesDiffsEqual(a, b *TypesDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Type.Id != b.Updated[i].Type.Id {
			return false
		}
	}
	return true
}

func areDesignsDiffsEqual(a, b *DesignsDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Design.Id != b.Updated[i].Design.Id {
			return false
		}
	}
	return true
}

func areTagsDiffsEqual(a, b *TagsDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Tag.Id != b.Updated[i].Tag.Id {
			return false
		}
	}
	return true
}

func areConceptsDiffsEqual(a, b *ConceptsDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Concept.Id != b.Updated[i].Concept.Id {
			return false
		}
	}
	return true
}

func arePortsDiffsEqual(a, b *PortsDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Port.Id != b.Updated[i].Port.Id {
			return false
		}
	}
	return true
}

func areFamiliesDiffsEqual(a, b *FamiliesDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Family.Id != b.Updated[i].Family.Id {
			return false
		}
	}
	return true
}

func areTypologiesDiffsEqual(a, b *TypologiesDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Typology.Id != b.Updated[i].Typology.Id {
			return false
		}
	}
	return true
}

func areFilesDiffsEqual(a, b *FilesDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].File.Id != b.Updated[i].File.Id {
			return false
		}
	}
	return true
}

func areFoldersDiffsEqual(a, b *FoldersDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Folder.Id != b.Updated[i].Folder.Id {
			return false
		}
	}
	return true
}

func areAuthorsDiffsEqual(a, b *AuthorsDiff) bool {
	if (a == nil) != (b == nil) {
		return false
	}
	if a == nil {
		return true
	}
	if len(a.Added) != len(b.Added) {
		return false
	}
	for i := range a.Added {
		if a.Added[i].Id != b.Added[i].Id {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Id != b.Removed[i].Id {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Author.Id != b.Updated[i].Author.Id {
			return false
		}
	}
	return true
}

// 📦GetKitDiff computes the differential between a before and after kit state.
func GetKitDiff(before, after Kit) KitDiff {
	KitEnsureTypologies(&before)
	KitEnsureTypologies(&after)
	diff := KitDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if before.Version != after.Version {
		diff.Version = &after.Version
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	if normalizeStr(before.Icon) != normalizeStr(after.Icon) {
		diff.Icon = after.Icon
	}
	if normalizeStr(before.Image) != normalizeStr(after.Image) {
		diff.Image = after.Image
	}
	if normalizeStr(before.Remote) != normalizeStr(after.Remote) {
		diff.Remote = after.Remote
	}
	if normalizeStr(before.Homepage) != normalizeStr(after.Homepage) {
		diff.Homepage = after.Homepage
	}
	if normalizeStr(before.License) != normalizeStr(after.License) {
		diff.License = after.License
	}
	if normalizeStr(before.Preview) != normalizeStr(after.Preview) {
		diff.Preview = after.Preview
	}
	typologiesDiff := getTypologiesDiff(before.Typologies, after.Typologies)
	if len(typologiesDiff.Added) > 0 || len(typologiesDiff.Removed) > 0 || len(typologiesDiff.Updated) > 0 {
		diff.Typologies = &typologiesDiff
	}
	tagsDiff := getTagsDiff(before.Tags, after.Tags)
	if len(tagsDiff.Added) > 0 || len(tagsDiff.Removed) > 0 || len(tagsDiff.Updated) > 0 {
		diff.Tags = &tagsDiff
	}
	conceptsDiff := getConceptsDiff(before.Concepts, after.Concepts)
	if len(conceptsDiff.Added) > 0 || len(conceptsDiff.Removed) > 0 || len(conceptsDiff.Updated) > 0 {
		diff.Concepts = &conceptsDiff
	}
	familiesDiff := getFamiliesDiff(before.Families, after.Families)
	if len(familiesDiff.Added) > 0 || len(familiesDiff.Removed) > 0 || len(familiesDiff.Updated) > 0 {
		diff.Families = &familiesDiff
	}
	filesDiff := getFilesDiff(before.Files, after.Files)
	if len(filesDiff.Added) > 0 || len(filesDiff.Removed) > 0 || len(filesDiff.Updated) > 0 {
		diff.Files = &filesDiff
	}
	foldersDiff := getFoldersDiff(before.Folders, after.Folders)
	if len(foldersDiff.Added) > 0 || len(foldersDiff.Removed) > 0 || len(foldersDiff.Updated) > 0 {
		diff.Folders = &foldersDiff
	}
	authorsDiff := getAuthorsDiff(before.Authors, after.Authors)
	if len(authorsDiff.Added) > 0 || len(authorsDiff.Removed) > 0 || len(authorsDiff.Updated) > 0 {
		diff.Authors = &authorsDiff
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func getTypesDiff(before, after []Type) TypesDiff {
	diff := TypesDiff{}
	beforeMap := make(map[string]Type)
	for _, t := range before {
		beforeMap[t.Id] = t
	}
	afterMap := make(map[string]Type)
	for _, t := range after {
		afterMap[t.Id] = t
	}
	for _, t := range before {
		if _, ok := afterMap[t.Id]; !ok {
			diff.Removed = append(diff.Removed, TypeId{Id: t.Id})
		}
	}
	for _, t := range after {
		if _, ok := beforeMap[t.Id]; !ok {
			diff.Added = append(diff.Added, t)
		} else {
			typeDiff := getTypeDiff(beforeMap[t.Id], t)
			if !isTypeDiffEmpty(typeDiff) {
				diff.Updated = append(diff.Updated, struct {
					Type TypeId   `json:"type"`
					Diff TypeDiff `json:"diff"`
				}{Type: TypeId{Id: t.Id}, Diff: typeDiff})
			}
		}
	}
	return diff
}

func getTypeDiff(before, after Type) TypeDiff {
	diff := TypeDiff{}
	diff.setFields = make(map[string]bool)
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if !areFamilyIdSlicesEqual(before.Families, after.Families) {
		diff.Families = after.Families
		diff.setFields["families"] = true
	}
	if before.Typology.Id != after.Typology.Id {
		diff.Typology = &after.Typology
		diff.setFields["typology"] = true
	}
	if !optBoolEqual(before.IsAbstract, after.IsAbstract) {
		diff.IsAbstract = after.IsAbstract
	}
	if !optBoolEqual(before.Virtual, after.Virtual) {
		diff.Virtual = after.Virtual
		diff.setFields["virtual"] = true
	}
	if normalizeStr(before.Unit) != normalizeStr(after.Unit) {
		diff.Unit = after.Unit
		diff.setFields["unit"] = true
	}
	if normalizeOptInt(before.Stock) != normalizeOptInt(after.Stock) {
		diff.Stock = after.Stock
	}
	if !areLocationIdsEqual(before.Location, after.Location) {
		diff.Location = after.Location
	}
	if normalizeStr(before.Folder) != normalizeStr(after.Folder) {
		diff.Folder = after.Folder
	}
	if normalizeStr(before.Icon) != normalizeStr(after.Icon) {
		diff.Icon = after.Icon
	}
	if normalizeStr(before.Image) != normalizeStr(after.Image) {
		diff.Image = after.Image
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
		diff.setFields["description"] = true
	}
	if !areAuthorIdsEqual(before.Authors, after.Authors) {
		diff.Authors = after.Authors
	}
	if !areConceptIdsEqual(before.Concepts, after.Concepts) {
		diff.Concepts = after.Concepts
	}
	connDiff := getConnectorsDiff(before.Connectors, after.Connectors)
	if len(connDiff.Added) > 0 || len(connDiff.Removed) > 0 || len(connDiff.Updated) > 0 {
		diff.Connectors = &connDiff
	}
	representationsDiff := getRepresentationsDiff(before.Representations, after.Representations)
	if len(representationsDiff.Added) > 0 || len(representationsDiff.Removed) > 0 || len(representationsDiff.Updated) > 0 {
		diff.Representations = &representationsDiff
	}
	propsDiff := getPropsDiff(before.Props, after.Props)
	if len(propsDiff.Added) > 0 || len(propsDiff.Removed) > 0 || len(propsDiff.Updated) > 0 {
		diff.Props = &propsDiff
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isTypeDiffEmpty(diff TypeDiff) bool {
	return diff.Name == nil && diff.Typology == nil && diff.Families == nil && diff.IsAbstract == nil && diff.Virtual == nil && diff.Unit == nil && diff.Stock == nil && diff.Location == nil && diff.Folder == nil && diff.Icon == nil && diff.Image == nil && diff.Description == nil && diff.Authors == nil && diff.Concepts == nil && diff.Connectors == nil && diff.Representations == nil && diff.Props == nil && diff.Attributes == nil
}

func getDesignsDiff(before, after []Design) DesignsDiff {
	diff := DesignsDiff{}
	beforeMap := make(map[string]Design)
	for _, d := range before {
		beforeMap[d.Id] = d
	}
	afterMap := make(map[string]Design)
	for _, d := range after {
		afterMap[d.Id] = d
	}
	for _, d := range before {
		if _, ok := afterMap[d.Id]; !ok {
			diff.Removed = append(diff.Removed, DesignId{Id: d.Id})
		}
	}
	for _, d := range after {
		if _, ok := beforeMap[d.Id]; !ok {
			diff.Added = append(diff.Added, d)
		} else {
			designDiff := getDesignDiff(beforeMap[d.Id], d)
			if !isDesignDiffEmpty(designDiff) {
				diff.Updated = append(diff.Updated, struct {
					Design DesignId   `json:"design"`
					Diff   DesignDiff `json:"diff"`
				}{Design: DesignId{Id: d.Id}, Diff: designDiff})
			}
		}
	}
	return diff
}

func getDesignDiff(before, after Design) DesignDiff {
	diff := DesignDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if !areFamilyIdSlicesEqual(before.Families, after.Families) {
		diff.Families = after.Families
	}
	if before.Typology.Id != after.Typology.Id {
		diff.Typology = &after.Typology
	}
	if !optBoolEqual(before.IsAbstract, after.IsAbstract) {
		diff.IsAbstract = after.IsAbstract
	}
	if normalizeStr(before.Unit) != normalizeStr(after.Unit) {
		diff.Unit = after.Unit
	}
	if normalizeStr(before.Folder) != normalizeStr(after.Folder) {
		diff.Folder = after.Folder
	}
	if !optBoolEqual(before.CanScale, after.CanScale) {
		diff.CanScale = after.CanScale
	}
	if !optBoolEqual(before.CanMirror, after.CanMirror) {
		diff.CanMirror = after.CanMirror
	}
	if !areLayerIdsEqual(before.ActiveLayer, after.ActiveLayer) {
		diff.ActiveLayer = after.ActiveLayer
	}
	if !areLocationIdsEqual(before.Location, after.Location) {
		diff.Location = after.Location
	}
	if normalizeStr(before.Icon) != normalizeStr(after.Icon) {
		diff.Icon = after.Icon
	}
	if normalizeStr(before.Image) != normalizeStr(after.Image) {
		diff.Image = after.Image
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	if !areAuthorIdsEqual(before.Authors, after.Authors) {
		diff.Authors = after.Authors
	}
	if !areConceptIdsEqual(before.Concepts, after.Concepts) {
		diff.Concepts = after.Concepts
	}
	piecesDiff := getPiecesDiff(before.Pieces, after.Pieces)
	if len(piecesDiff.Added) > 0 || len(piecesDiff.Removed) > 0 || len(piecesDiff.Updated) > 0 {
		diff.Pieces = &piecesDiff
	}
	connsDiff := getConnectionsDiff(before.Connections, after.Connections)
	if len(connsDiff.Added) > 0 || len(connsDiff.Removed) > 0 || len(connsDiff.Updated) > 0 {
		diff.Connections = &connsDiff
	}
	statsDiff := getStatsDiff(before.Stats, after.Stats)
	if len(statsDiff.Added) > 0 || len(statsDiff.Removed) > 0 || len(statsDiff.Updated) > 0 {
		diff.Stats = &statsDiff
	}
	propsDiff := getPropsDiff(before.Props, after.Props)
	if len(propsDiff.Added) > 0 || len(propsDiff.Removed) > 0 || len(propsDiff.Updated) > 0 {
		diff.Props = &propsDiff
	}
	layersDiff := getLayersDiff(before.Layers, after.Layers)
	if len(layersDiff.Added) > 0 || len(layersDiff.Removed) > 0 || len(layersDiff.Updated) > 0 {
		diff.Layers = &layersDiff
	}
	groupsDiff := getGroupsDiff(before.Groups, after.Groups)
	if len(groupsDiff.Added) > 0 || len(groupsDiff.Removed) > 0 || len(groupsDiff.Updated) > 0 {
		diff.Groups = &groupsDiff
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isDesignDiffEmpty(diff DesignDiff) bool {
	return diff.Name == nil && diff.Typology == nil && diff.Families == nil && diff.IsAbstract == nil && diff.Unit == nil && diff.Folder == nil && diff.CanScale == nil && diff.CanMirror == nil && diff.ActiveLayer == nil && diff.Location == nil && diff.Icon == nil && diff.Image == nil && diff.Description == nil && diff.Authors == nil && diff.Concepts == nil && diff.Pieces == nil && diff.Connections == nil && diff.Stats == nil && diff.Props == nil && diff.Layers == nil && diff.Groups == nil && diff.Attributes == nil
}

func getTagsDiff(before, after []Tag) TagsDiff {
	diff := TagsDiff{}
	beforeMap := make(map[string]Tag)
	for _, t := range before {
		beforeMap[t.Id] = t
	}
	afterMap := make(map[string]Tag)
	for _, t := range after {
		afterMap[t.Id] = t
	}
	for _, t := range before {
		if _, ok := afterMap[t.Id]; !ok {
			diff.Removed = append(diff.Removed, TagId{Id: t.Id})
		}
	}
	for _, t := range after {
		if _, ok := beforeMap[t.Id]; !ok {
			diff.Added = append(diff.Added, t)
		} else {
			tagDiff := getTagDiff(beforeMap[t.Id], t)
			if !isTagDiffEmpty(tagDiff) {
				diff.Updated = append(diff.Updated, struct {
					Tag  TagId   `json:"tag"`
					Diff TagDiff `json:"diff"`
				}{Tag: TagId{Id: t.Id}, Diff: tagDiff})
			}
		}
	}
	return diff
}

func getTagDiff(before, after Tag) TagDiff {
	diff := TagDiff{}
	diff.setFields = make(map[string]bool)
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
		diff.setFields["description"] = true
	}
	if normalizeStr(before.Icon) != normalizeStr(after.Icon) {
		diff.Icon = after.Icon
		diff.setFields["icon"] = true
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isTagDiffEmpty(diff TagDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Icon == nil && diff.Attributes == nil
}

func getConceptsDiff(before, after []Concept) ConceptsDiff {
	diff := ConceptsDiff{}
	beforeMap := make(map[string]Concept)
	for _, c := range before {
		beforeMap[c.Id] = c
	}
	afterMap := make(map[string]Concept)
	for _, c := range after {
		afterMap[c.Id] = c
	}
	for _, c := range before {
		if _, ok := afterMap[c.Id]; !ok {
			diff.Removed = append(diff.Removed, ConceptId{Id: c.Id})
		}
	}
	for _, c := range after {
		if _, ok := beforeMap[c.Id]; !ok {
			diff.Added = append(diff.Added, c)
		} else {
			conceptDiff := getConceptDiff(beforeMap[c.Id], c)
			if !isConceptDiffEmpty(conceptDiff) {
				diff.Updated = append(diff.Updated, struct {
					Concept ConceptId   `json:"concept"`
					Diff    ConceptDiff `json:"diff"`
				}{Concept: ConceptId{Id: c.Id}, Diff: conceptDiff})
			}
		}
	}
	return diff
}

func getConceptDiff(before, after Concept) ConceptDiff {
	diff := ConceptDiff{}
	diff.setFields = make(map[string]bool)
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
		diff.setFields["description"] = true
	}
	if normalizeStr(before.Icon) != normalizeStr(after.Icon) {
		diff.Icon = after.Icon
		diff.setFields["icon"] = true
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isConceptDiffEmpty(diff ConceptDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Icon == nil && diff.Attributes == nil
}

func getPortsDiff(before, after []Port) PortsDiff {
	diff := PortsDiff{}
	beforeMap := make(map[string]Port)
	for _, i := range before {
		beforeMap[i.Id] = i
	}
	afterMap := make(map[string]Port)
	for _, i := range after {
		afterMap[i.Id] = i
	}
	for _, i := range before {
		if _, ok := afterMap[i.Id]; !ok {
			diff.Removed = append(diff.Removed, PortId{Id: i.Id})
		}
	}
	for _, i := range after {
		if _, ok := beforeMap[i.Id]; !ok {
			diff.Added = append(diff.Added, i)
		} else {
			interfaceDiff := getPortDiff(beforeMap[i.Id], i)
			if !isPortDiffEmpty(interfaceDiff) {
				diff.Updated = append(diff.Updated, struct {
					Port PortId   `json:"port"`
					Diff PortDiff `json:"diff"`
				}{Port: PortId{Id: i.Id}, Diff: interfaceDiff})
			}
		}
	}
	return diff
}

func getPortDiff(before, after Port) PortDiff {
	diff := PortDiff{}
	diff.setFields = make(map[string]bool)
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
		diff.setFields["description"] = true
	}
	if normalizeStr(before.Icon) != normalizeStr(after.Icon) {
		diff.Icon = after.Icon
		diff.setFields["icon"] = true
	}
	if (before.MaxChildren == nil) != (after.MaxChildren == nil) || (before.MaxChildren != nil && *before.MaxChildren != *after.MaxChildren) {
		diff.MaxChildren = after.MaxChildren
		diff.setFields["maxChildren"] = true
	}
	if !arePortIdSlicesEqual(before.CompatiblePorts, after.CompatiblePorts) {
		diff.CompatiblePorts = after.CompatiblePorts
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isPortDiffEmpty(diff PortDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Icon == nil && diff.MaxChildren == nil && diff.CompatiblePorts == nil && diff.Attributes == nil
}

func getFamiliesDiff(before, after []Family) FamiliesDiff {
	diff := FamiliesDiff{}
	beforeMap := make(map[string]Family)
	for _, f := range before {
		beforeMap[f.Id] = f
	}
	afterMap := make(map[string]Family)
	for _, f := range after {
		afterMap[f.Id] = f
	}
	for _, f := range before {
		if _, ok := afterMap[f.Id]; !ok {
			diff.Removed = append(diff.Removed, FamilyId{Id: f.Id})
		}
	}
	for _, f := range after {
		if _, ok := beforeMap[f.Id]; !ok {
			diff.Added = append(diff.Added, f)
		} else {
			familyDiff := getFamilyDiff(beforeMap[f.Id], f)
			if !isFamilyDiffEmpty(familyDiff) {
				diff.Updated = append(diff.Updated, struct {
					Family FamilyId   `json:"family"`
					Diff   FamilyDiff `json:"diff"`
				}{Family: FamilyId{Id: f.Id}, Diff: familyDiff})
			}
		}
	}
	return diff
}

func getFamilyDiff(before, after Family) FamilyDiff {
	diff := FamilyDiff{}
	diff.setFields = make(map[string]bool)
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
		diff.setFields["description"] = true
	}
	if normalizeStr(before.Icon) != normalizeStr(after.Icon) {
		diff.Icon = after.Icon
		diff.setFields["icon"] = true
	}
	portsDiff := getPortsDiff(before.Ports, after.Ports)
	if len(portsDiff.Added) > 0 || len(portsDiff.Removed) > 0 || len(portsDiff.Updated) > 0 {
		diff.Ports = &portsDiff
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isFamilyDiffEmpty(diff FamilyDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Icon == nil && diff.Ports == nil && diff.Attributes == nil
}

func getTypologiesDiff(before, after []Typology) TypologiesDiff {
	diff := TypologiesDiff{}
	beforeMap := make(map[string]Typology)
	for _, t := range before {
		beforeMap[t.Id] = t
	}
	afterMap := make(map[string]Typology)
	for _, t := range after {
		afterMap[t.Id] = t
	}
	for _, t := range before {
		if _, ok := afterMap[t.Id]; !ok {
			diff.Removed = append(diff.Removed, TypologyId{Id: t.Id})
		}
	}
	for _, t := range after {
		if _, ok := beforeMap[t.Id]; !ok {
			diff.Added = append(diff.Added, t)
		} else {
			topoDiff := getTypologyDiff(beforeMap[t.Id], t)
			if !isTypologyDiffEmpty(topoDiff) {
				diff.Updated = append(diff.Updated, struct {
					Typology TypologyId   `json:"typology"`
					Diff     TypologyDiff `json:"diff"`
				}{Typology: TypologyId{Id: t.Id}, Diff: topoDiff})
			}
		}
	}
	return diff
}

func getTypologyDiff(before, after Typology) TypologyDiff {
	diff := TypologyDiff{}
	diff.setFields = make(map[string]bool)
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
		diff.setFields["description"] = true
	}
	if normalizeStr(before.Icon) != normalizeStr(after.Icon) {
		diff.Icon = after.Icon
		diff.setFields["icon"] = true
	}
	if normalizeStr(before.Folder) != normalizeStr(after.Folder) {
		diff.Folder = after.Folder
		diff.setFields["folder"] = true
	}
	typesDiff := getTypesDiff(before.Types, after.Types)
	if len(typesDiff.Added) > 0 || len(typesDiff.Removed) > 0 || len(typesDiff.Updated) > 0 {
		diff.Types = &typesDiff
	}
	designsDiff := getDesignsDiff(before.Designs, after.Designs)
	if len(designsDiff.Added) > 0 || len(designsDiff.Removed) > 0 || len(designsDiff.Updated) > 0 {
		diff.Designs = &designsDiff
	}
	return diff
}

func isTypologyDiffEmpty(diff TypologyDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Icon == nil && diff.Folder == nil && diff.Types == nil && diff.Designs == nil
}

func getFilesDiff(before, after []File) FilesDiff {
	diff := FilesDiff{}
	beforeMap := make(map[string]File)
	for _, f := range before {
		beforeMap[f.Id] = f
	}
	afterMap := make(map[string]File)
	for _, f := range after {
		afterMap[f.Id] = f
	}
	for _, f := range before {
		if _, ok := afterMap[f.Id]; !ok {
			diff.Removed = append(diff.Removed, FileId{Id: f.Id})
		}
	}
	for _, f := range after {
		if _, ok := beforeMap[f.Id]; !ok {
			diff.Added = append(diff.Added, f)
		} else {
			fileDiff := getFileDiff(beforeMap[f.Id], f)
			if !isFileDiffEmpty(fileDiff) {
				diff.Updated = append(diff.Updated, struct {
					File FileId   `json:"file"`
					Diff FileDiff `json:"diff"`
				}{File: FileId{Id: f.Id}, Diff: fileDiff})
			}
		}
	}
	return diff
}

func getFileDiff(before, after File) FileDiff {
	diff := FileDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Remote) != normalizeStr(after.Remote) {
		diff.Remote = after.Remote
	}
	if !areFolderIdsEqual(before.Folder, after.Folder) {
		diff.Folder = after.Folder
	}
	if normalizeInt64(before.Size) != normalizeInt64(after.Size) {
		diff.Size = after.Size
	}
	if normalizeStr(before.Hash) != normalizeStr(after.Hash) {
		diff.Hash = after.Hash
	}
	if normalizeStr(before.Blob) != normalizeStr(after.Blob) {
		diff.Blob = after.Blob
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	attrDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrDiff) {
		diff.Attributes = &attrDiff
	}
	return diff
}

func isFileDiffEmpty(diff FileDiff) bool {
	return diff.Name == nil && diff.Remote == nil && diff.Folder == nil && diff.Size == nil && diff.Hash == nil && diff.Blob == nil && diff.Description == nil && diff.Attributes == nil
}

func getFoldersDiff(before, after []Folder) FoldersDiff {
	diff := FoldersDiff{}
	beforeMap := make(map[string]Folder)
	for _, f := range before {
		beforeMap[f.Id] = f
	}
	afterMap := make(map[string]Folder)
	for _, f := range after {
		afterMap[f.Id] = f
	}
	for _, f := range before {
		if _, ok := afterMap[f.Id]; !ok {
			diff.Removed = append(diff.Removed, FolderId{Id: f.Id})
		}
	}
	for _, f := range after {
		if _, ok := beforeMap[f.Id]; !ok {
			diff.Added = append(diff.Added, f)
		} else {
			folderDiff := getFolderDiff(beforeMap[f.Id], f)
			if !isFolderDiffEmpty(folderDiff) {
				diff.Updated = append(diff.Updated, struct {
					Folder FolderId   `json:"folder"`
					Diff   FolderDiff `json:"diff"`
				}{Folder: FolderId{Id: f.Id}, Diff: folderDiff})
			}
		}
	}
	return diff
}

func getFolderDiff(before, after Folder) FolderDiff {
	diff := FolderDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if !areFolderIdsEqual(before.Parent, after.Parent) {
		diff.Parent = after.Parent
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isFolderDiffEmpty(diff FolderDiff) bool {
	return diff.Name == nil && diff.Parent == nil && diff.Description == nil && diff.Attributes == nil
}

func getAuthorsDiff(before, after []Author) AuthorsDiff {
	diff := AuthorsDiff{}
	beforeMap := make(map[string]Author)
	for _, a := range before {
		beforeMap[a.Id] = a
	}
	afterMap := make(map[string]Author)
	for _, a := range after {
		afterMap[a.Id] = a
	}
	for _, a := range before {
		if _, ok := afterMap[a.Id]; !ok {
			diff.Removed = append(diff.Removed, AuthorId{Id: a.Id})
		}
	}
	for _, a := range after {
		if _, ok := beforeMap[a.Id]; !ok {
			diff.Added = append(diff.Added, a)
		} else {
			authorDiff := getAuthorDiff(beforeMap[a.Id], a)
			if !isAuthorDiffEmpty(authorDiff) {
				diff.Updated = append(diff.Updated, struct {
					Author AuthorId   `json:"author"`
					Diff   AuthorDiff `json:"diff"`
				}{Author: AuthorId{Id: a.Id}, Diff: authorDiff})
			}
		}
	}
	return diff
}

func getAuthorDiff(before, after Author) AuthorDiff {
	diff := AuthorDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Email) != normalizeStr(after.Email) {
		diff.Email = after.Email
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isAuthorDiffEmpty(diff AuthorDiff) bool {
	return diff.Name == nil && diff.Email == nil && diff.Attributes == nil
}

// 📦InverseKitDiff computes the reverse diff that undoes a previously applied diff.
func InverseKitDiff(original Kit, appliedDiff KitDiff) KitDiff {
	KitEnsureTypologies(&original)
	inverse := KitDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Version != nil {
		inverse.Version = &original.Version
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Icon != nil {
		inverse.Icon = original.Icon
	}
	if appliedDiff.Image != nil {
		inverse.Image = original.Image
	}
	if appliedDiff.Remote != nil {
		inverse.Remote = original.Remote
	}
	if appliedDiff.Homepage != nil {
		inverse.Homepage = original.Homepage
	}
	if appliedDiff.License != nil {
		inverse.License = original.License
	}
	if appliedDiff.Preview != nil {
		inverse.Preview = original.Preview
	}
	if appliedDiff.Typologies != nil {
		typologiesDiff := inverseTypologiesDiff(original.Typologies, *appliedDiff.Typologies)
		inverse.Typologies = &typologiesDiff
	}
	if appliedDiff.Tags != nil {
		tagsDiff := inverseTagsDiff(original.Tags, *appliedDiff.Tags)
		inverse.Tags = &tagsDiff
	}
	if appliedDiff.Concepts != nil {
		conceptsDiff := inverseConceptsDiff(original.Concepts, *appliedDiff.Concepts)
		inverse.Concepts = &conceptsDiff
	}
	if appliedDiff.Families != nil {
		familiesDiff := inverseFamiliesDiff(original.Families, *appliedDiff.Families)
		inverse.Families = &familiesDiff
	}
	if appliedDiff.Files != nil {
		filesDiff := inverseFilesDiff(original.Files, *appliedDiff.Files)
		inverse.Files = &filesDiff
	}
	if appliedDiff.Folders != nil {
		foldersDiff := inverseFoldersDiff(original.Folders, *appliedDiff.Folders)
		inverse.Folders = &foldersDiff
	}
	if appliedDiff.Authors != nil {
		authorsDiff := inverseAuthorsDiff(original.Authors, *appliedDiff.Authors)
		inverse.Authors = &authorsDiff
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseTypesDiff(original []Type, appliedDiff TypesDiff) TypesDiff {
	inverse := TypesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, TypeId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, t := range original {
			if t.Id == removed.Id {
				inverse.Added = append(inverse.Added, t)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, t := range original {
			if t.Id == updated.Type.Id {
				inverseDiff := inverseTypeDiff(t, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Type TypeId   `json:"type"`
					Diff TypeDiff `json:"diff"`
				}{Type: TypeId{Id: t.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseTypeDiff(original Type, appliedDiff TypeDiff) TypeDiff {
	inverse := TypeDiff{}
	inverse.setFields = make(map[string]bool)
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Families != nil {
		inverse.Families = original.Families
		inverse.setFields["families"] = true
	}
	if appliedDiff.IsAbstract != nil {
		inverse.IsAbstract = original.IsAbstract
	}
	if appliedDiff.HasField("virtual") {
		inverse.Virtual = original.Virtual
		inverse.setFields["virtual"] = true
	}
	if appliedDiff.HasField("unit") {
		inverse.Unit = original.Unit
		inverse.setFields["unit"] = true
	}
	if appliedDiff.Stock != nil {
		inverse.Stock = original.Stock
	}
	if appliedDiff.Location != nil {
		inverse.Location = original.Location
	}
	if appliedDiff.Folder != nil {
		inverse.Folder = original.Folder
	}
	if appliedDiff.Icon != nil {
		inverse.Icon = original.Icon
	}
	if appliedDiff.Image != nil {
		inverse.Image = original.Image
	}
	if appliedDiff.HasField("description") {
		inverse.Description = original.Description
		inverse.setFields["description"] = true
	}
	if appliedDiff.Authors != nil {
		inverse.Authors = original.Authors
	}
	if appliedDiff.Concepts != nil {
		inverse.Concepts = original.Concepts
	}
	if appliedDiff.Representations != nil {
		representationsDiff := inverseRepresentationsDiff(original.Representations, *appliedDiff.Representations)
		inverse.Representations = &representationsDiff
	}
	if appliedDiff.Connectors != nil {
		connDiff := inverseConnectorsDiff(original.Connectors, *appliedDiff.Connectors)
		inverse.Connectors = &connDiff
	}
	if appliedDiff.Props != nil {
		propsDiff := inversePropsDiff(original.Props, *appliedDiff.Props)
		inverse.Props = &propsDiff
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseDesignsDiff(original []Design, appliedDiff DesignsDiff) DesignsDiff {
	inverse := DesignsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, DesignId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, d := range original {
			if d.Id == removed.Id {
				inverse.Added = append(inverse.Added, d)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, d := range original {
			if d.Id == updated.Design.Id {
				inverseDiff := inverseDesignDiff(d, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Design DesignId   `json:"design"`
					Diff   DesignDiff `json:"diff"`
				}{Design: DesignId{Id: d.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseDesignDiff(original Design, appliedDiff DesignDiff) DesignDiff {
	inverse := DesignDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Families != nil {
		inverse.Families = original.Families
	}
	if appliedDiff.IsAbstract != nil {
		inverse.IsAbstract = original.IsAbstract
	}
	if appliedDiff.Unit != nil {
		inverse.Unit = original.Unit
	}
	if appliedDiff.Folder != nil {
		inverse.Folder = original.Folder
	}
	if appliedDiff.CanScale != nil {
		inverse.CanScale = original.CanScale
	}
	if appliedDiff.CanMirror != nil {
		inverse.CanMirror = original.CanMirror
	}
	if appliedDiff.ActiveLayer != nil {
		inverse.ActiveLayer = original.ActiveLayer
	}
	if appliedDiff.Location != nil {
		inverse.Location = original.Location
	}
	if appliedDiff.Icon != nil {
		inverse.Icon = original.Icon
	}
	if appliedDiff.Image != nil {
		inverse.Image = original.Image
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Authors != nil {
		inverse.Authors = original.Authors
	}
	if appliedDiff.Concepts != nil {
		inverse.Concepts = original.Concepts
	}
	if appliedDiff.Pieces != nil {
		piecesDiff := inversePiecesDiff(original.Pieces, *appliedDiff.Pieces)
		inverse.Pieces = &piecesDiff
	}
	if appliedDiff.Connections != nil {
		connsDiff := inverseConnectionsDiff(original.Connections, *appliedDiff.Connections)
		inverse.Connections = &connsDiff
	}
	if appliedDiff.Stats != nil {
		statsDiff := inverseStatsDiff(original.Stats, *appliedDiff.Stats)
		inverse.Stats = &statsDiff
	}
	if appliedDiff.Props != nil {
		propsDiff := inversePropsDiff(original.Props, *appliedDiff.Props)
		inverse.Props = &propsDiff
	}
	if appliedDiff.Layers != nil {
		layersDiff := inverseLayersDiff(original.Layers, *appliedDiff.Layers)
		inverse.Layers = &layersDiff
	}
	if appliedDiff.Groups != nil {
		groupsDiff := inverseGroupsDiff(original.Groups, *appliedDiff.Groups)
		inverse.Groups = &groupsDiff
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseTagsDiff(original []Tag, appliedDiff TagsDiff) TagsDiff {
	inverse := TagsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, TagId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, t := range original {
			if t.Id == removed.Id {
				inverse.Added = append(inverse.Added, t)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, t := range original {
			if t.Id == updated.Tag.Id {
				inverseDiff := inverseTagDiff(t, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Tag  TagId   `json:"tag"`
					Diff TagDiff `json:"diff"`
				}{Tag: TagId{Id: t.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseTagDiff(original Tag, appliedDiff TagDiff) TagDiff {
	inverse := TagDiff{}
	inverse.setFields = make(map[string]bool)
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
		inverse.setFields["description"] = true
	}
	if appliedDiff.Icon != nil {
		inverse.Icon = original.Icon
		inverse.setFields["icon"] = true
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseConceptsDiff(original []Concept, appliedDiff ConceptsDiff) ConceptsDiff {
	inverse := ConceptsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, ConceptId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, c := range original {
			if c.Id == removed.Id {
				inverse.Added = append(inverse.Added, c)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, c := range original {
			if c.Id == updated.Concept.Id {
				inverseDiff := inverseConceptDiff(c, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Concept ConceptId   `json:"concept"`
					Diff    ConceptDiff `json:"diff"`
				}{Concept: ConceptId{Id: c.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseConceptDiff(original Concept, appliedDiff ConceptDiff) ConceptDiff {
	inverse := ConceptDiff{}
	inverse.setFields = make(map[string]bool)
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
		inverse.setFields["description"] = true
	}
	if appliedDiff.Icon != nil {
		inverse.Icon = original.Icon
		inverse.setFields["icon"] = true
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inversePortsDiff(original []Port, appliedDiff PortsDiff) PortsDiff {
	inverse := PortsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, PortId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, i := range original {
			if i.Id == removed.Id {
				inverse.Added = append(inverse.Added, i)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, i := range original {
			if i.Id == updated.Port.Id {
				inverseDiff := inversePortDiff(i, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Port PortId   `json:"port"`
					Diff PortDiff `json:"diff"`
				}{Port: PortId{Id: i.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inversePortDiff(original Port, appliedDiff PortDiff) PortDiff {
	inverse := PortDiff{}
	inverse.setFields = make(map[string]bool)
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
		inverse.setFields["description"] = true
	}
	if appliedDiff.Icon != nil {
		inverse.Icon = original.Icon
		inverse.setFields["icon"] = true
	}
	if appliedDiff.MaxChildren != nil {
		inverse.MaxChildren = original.MaxChildren
		inverse.setFields["maxChildren"] = true
	}
	if appliedDiff.CompatiblePorts != nil {
		inverse.CompatiblePorts = original.CompatiblePorts
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseTypologiesDiff(original []Typology, appliedDiff TypologiesDiff) TypologiesDiff {
	inverse := TypologiesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, TypologyId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, t := range original {
			if t.Id == removed.Id {
				inverse.Added = append(inverse.Added, t)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, t := range original {
			if t.Id == updated.Typology.Id {
				inverseDiff := inverseTypologyDiff(t, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Typology TypologyId   `json:"typology"`
					Diff     TypologyDiff `json:"diff"`
				}{Typology: TypologyId{Id: t.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseTypologyDiff(original Typology, appliedDiff TypologyDiff) TypologyDiff {
	inverse := TypologyDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.HasField("description") {
		inverse.Description = original.Description
		inverse.setFields = map[string]bool{"description": true}
	}
	if appliedDiff.HasField("icon") {
		inverse.Icon = original.Icon
		if inverse.setFields == nil {
			inverse.setFields = map[string]bool{}
		}
		inverse.setFields["icon"] = true
	}
	if appliedDiff.HasField("folder") {
		inverse.Folder = original.Folder
		if inverse.setFields == nil {
			inverse.setFields = map[string]bool{}
		}
		inverse.setFields["folder"] = true
	}
	if appliedDiff.Types != nil {
		typesDiff := inverseTypesDiff(original.Types, *appliedDiff.Types)
		inverse.Types = &typesDiff
	}
	if appliedDiff.Designs != nil {
		designsDiff := inverseDesignsDiff(original.Designs, *appliedDiff.Designs)
		inverse.Designs = &designsDiff
	}
	return inverse
}

func inverseFamiliesDiff(original []Family, appliedDiff FamiliesDiff) FamiliesDiff {
	inverse := FamiliesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, FamilyId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, f := range original {
			if f.Id == removed.Id {
				inverse.Added = append(inverse.Added, f)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, f := range original {
			if f.Id == updated.Family.Id {
				inverseDiff := inverseFamilyDiff(f, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Family FamilyId   `json:"family"`
					Diff   FamilyDiff `json:"diff"`
				}{Family: FamilyId{Id: f.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseFamilyDiff(original Family, appliedDiff FamilyDiff) FamilyDiff {
	inverse := FamilyDiff{}
	inverse.setFields = make(map[string]bool)
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
		inverse.setFields["description"] = true
	}
	if appliedDiff.Icon != nil {
		inverse.Icon = original.Icon
		inverse.setFields["icon"] = true
	}
	if appliedDiff.Ports != nil {
		portsDiff := inversePortsDiff(original.Ports, *appliedDiff.Ports)
		inverse.Ports = &portsDiff
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseFilesDiff(original []File, appliedDiff FilesDiff) FilesDiff {
	inverse := FilesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, FileId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, f := range original {
			if f.Id == removed.Id {
				inverse.Added = append(inverse.Added, f)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, f := range original {
			if f.Id == updated.File.Id {
				inverseDiff := inverseFileDiff(f, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					File FileId   `json:"file"`
					Diff FileDiff `json:"diff"`
				}{File: FileId{Id: f.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseFileDiff(original File, appliedDiff FileDiff) FileDiff {
	inverse := FileDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Remote != nil {
		inverse.Remote = original.Remote
	}
	if appliedDiff.Folder != nil {
		inverse.Folder = original.Folder
	}
	if appliedDiff.Size != nil {
		inverse.Size = original.Size
	}
	if appliedDiff.Hash != nil {
		inverse.Hash = original.Hash
	}
	if appliedDiff.Blob != nil {
		inverse.Blob = original.Blob
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Attributes != nil {
		attrDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrDiff
	}
	return inverse
}

func inverseFoldersDiff(original []Folder, appliedDiff FoldersDiff) FoldersDiff {
	inverse := FoldersDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, FolderId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, f := range original {
			if f.Id == removed.Id {
				inverse.Added = append(inverse.Added, f)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, f := range original {
			if f.Id == updated.Folder.Id {
				inverseDiff := inverseFolderDiff(f, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Folder FolderId   `json:"folder"`
					Diff   FolderDiff `json:"diff"`
				}{Folder: FolderId{Id: f.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseFolderDiff(original Folder, appliedDiff FolderDiff) FolderDiff {
	inverse := FolderDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Parent != nil {
		inverse.Parent = original.Parent
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseAuthorsDiff(original []Author, appliedDiff AuthorsDiff) AuthorsDiff {
	inverse := AuthorsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, AuthorId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, a := range original {
			if a.Id == removed.Id {
				inverse.Added = append(inverse.Added, a)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, a := range original {
			if a.Id == updated.Author.Id {
				inverseDiff := inverseAuthorDiff(a, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Author AuthorId   `json:"author"`
					Diff   AuthorDiff `json:"diff"`
				}{Author: AuthorId{Id: a.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseAuthorDiff(original Author, appliedDiff AuthorDiff) AuthorDiff {
	inverse := AuthorDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Email != nil {
		inverse.Email = original.Email
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseConnectorsDiff(original []Connector, appliedDiff ConnectorsDiff) ConnectorsDiff {
	inverse := ConnectorsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, ConnectorId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, c := range original {
			if c.Id == removed.Id {
				inverse.Added = append(inverse.Added, c)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, c := range original {
			if c.Id == updated.Connector.Id {
				inverseDiff := inverseConnectorDiff(c, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Connector ConnectorId   `json:"connector"`
					Diff      ConnectorDiff `json:"diff"`
				}{Connector: ConnectorId{Id: c.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseConnectorDiff(original Connector, appliedDiff ConnectorDiff) ConnectorDiff {
	inverse := ConnectorDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Port != nil {
		inverse.Port = original.Port
	}
	if appliedDiff.Mandatory != nil {
		inverse.Mandatory = original.Mandatory
	}
	if appliedDiff.T != nil {
		inverse.T = &original.T
	}
	if appliedDiff.Point != nil {
		p := &PointDiff{}
		if appliedDiff.Point.X != nil {
			v := -*appliedDiff.Point.X
			p.X = &v
		}
		if appliedDiff.Point.Y != nil {
			v := -*appliedDiff.Point.Y
			p.Y = &v
		}
		if appliedDiff.Point.Z != nil {
			v := -*appliedDiff.Point.Z
			p.Z = &v
		}
		inverse.Point = p
	}
	if appliedDiff.Direction != nil {
		d := &VectorDiff{}
		if appliedDiff.Direction.X != nil {
			v := -*appliedDiff.Direction.X
			d.X = &v
		}
		if appliedDiff.Direction.Y != nil {
			v := -*appliedDiff.Direction.Y
			d.Y = &v
		}
		if appliedDiff.Direction.Z != nil {
			v := -*appliedDiff.Direction.Z
			d.Z = &v
		}
		inverse.Direction = d
	}
	if appliedDiff.Props != nil {
		propsDiff := inversePropsDiff(original.Props, *appliedDiff.Props)
		inverse.Props = &propsDiff
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseRepresentationsDiff(original []Representation, appliedDiff RepresentationsDiff) RepresentationsDiff {
	inverse := RepresentationsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, RepresentationId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, m := range original {
			if m.Id == removed.Id {
				inverse.Added = append(inverse.Added, m)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, m := range original {
			if m.Id == updated.Representation.Id {
				inverseDiff := inverseRepresentationDiff(m, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Representation RepresentationId   `json:"representation"`
					Diff  RepresentationDiff `json:"diff"`
				}{Representation: RepresentationId{Id: m.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseRepresentationDiff(original Representation, appliedDiff RepresentationDiff) RepresentationDiff {
	inverse := RepresentationDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = original.Name
	}
	if appliedDiff.File != nil {
		inverse.File = &original.File
	}
	if appliedDiff.Tags != nil {
		inverse.Tags = original.Tags
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inversePiecesDiff(original []Piece, appliedDiff PiecesDiff) PiecesDiff {
	inverse := PiecesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, PieceId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, p := range original {
			if p.Id == removed.Id {
				inverse.Added = append(inverse.Added, p)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, p := range original {
			if p.Id == updated.Piece.Id {
				inverseDiff := inversePieceDiff(p, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Piece PieceId   `json:"piece"`
					Diff  PieceDiff `json:"diff"`
				}{Piece: PieceId{Id: p.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inversePieceDiff(original Piece, appliedDiff PieceDiff) PieceDiff {
	inverse := PieceDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = original.Name
	}
	if appliedDiff.Type != nil {
		inverse.Type = original.Type
	}
	if appliedDiff.Design != nil {
		inverse.Design = original.Design
	}
	if appliedDiff.Plane != nil {
		if original.Plane != nil {

			ox := original.Plane.Origin.X
			oy := original.Plane.Origin.Y
			oz := original.Plane.Origin.Z
			xx := original.Plane.XAxis.X
			xy := original.Plane.XAxis.Y
			xz := original.Plane.XAxis.Z
			yx := original.Plane.YAxis.X
			yy := original.Plane.YAxis.Y
			yz := original.Plane.YAxis.Z
			inverse.Plane = &PlaneDiff{
				Origin: &PointDiff{X: &ox, Y: &oy, Z: &oz},
				XAxis:  &VectorDiff{X: &xx, Y: &xy, Z: &xz},
				YAxis:  &VectorDiff{X: &yx, Y: &yy, Z: &yz},
			}
		}
	}
	if appliedDiff.Center != nil {
		if original.Center != nil {
			inverse.Center = &CoordinateDiff{U: &original.Center.U, V: &original.Center.V}
		}
	}
	if appliedDiff.Scale != nil {
		inverse.Scale = original.Scale
	}
	if appliedDiff.MirrorPlane != nil {
		if original.MirrorPlane != nil {
			ox := original.MirrorPlane.Origin.X
			oy := original.MirrorPlane.Origin.Y
			oz := original.MirrorPlane.Origin.Z
			xx := original.MirrorPlane.XAxis.X
			xy := original.MirrorPlane.XAxis.Y
			xz := original.MirrorPlane.XAxis.Z
			yx := original.MirrorPlane.YAxis.X
			yy := original.MirrorPlane.YAxis.Y
			yz := original.MirrorPlane.YAxis.Z
			inverse.MirrorPlane = &PlaneDiff{
				Origin: &PointDiff{X: &ox, Y: &oy, Z: &oz},
				XAxis:  &VectorDiff{X: &xx, Y: &xy, Z: &xz},
				YAxis:  &VectorDiff{X: &yx, Y: &yy, Z: &yz},
			}
		}
	}
	if appliedDiff.IsHidden != nil {
		inverse.IsHidden = original.IsHidden
	}
	if appliedDiff.IsLocked != nil {
		inverse.IsLocked = original.IsLocked
	}
	if appliedDiff.Color != nil {
		inverse.Color = original.Color
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Props != nil {
		propsDiff := inversePropsDiff(original.Props, *appliedDiff.Props)
		inverse.Props = &propsDiff
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseConnectionsDiff(original []Connection, appliedDiff ConnectionsDiff) ConnectionsDiff {
	inverse := ConnectionsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, ConnectionId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, c := range original {
			if c.Id == removed.Id {
				inverse.Added = append(inverse.Added, c)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, c := range original {
			if c.Id == updated.Connection.Id {
				inverseDiff := inverseConnectionDiff(c, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Connection ConnectionId   `json:"connection"`
					Diff       ConnectionDiff `json:"diff"`
				}{Connection: ConnectionId{Id: c.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseConnectionDiff(original Connection, appliedDiff ConnectionDiff) ConnectionDiff {
	inverse := ConnectionDiff{}
	if appliedDiff.Parent != nil {
		inverse.Parent = inverseSideDiff(original.Parent, *appliedDiff.Parent)
	}
	if appliedDiff.Child != nil {
		inverse.Child = inverseSideDiff(original.Child, *appliedDiff.Child)
	}
	if appliedDiff.Gap != nil {
		v := -*appliedDiff.Gap
		inverse.Gap = &v
	}
	if appliedDiff.Shift != nil {
		v := -*appliedDiff.Shift
		inverse.Shift = &v
	}
	if appliedDiff.Rise != nil {
		v := -*appliedDiff.Rise
		inverse.Rise = &v
	}
	if appliedDiff.Rotation != nil {
		v := -*appliedDiff.Rotation
		inverse.Rotation = &v
	}
	if appliedDiff.Turn != nil {
		v := -*appliedDiff.Turn
		inverse.Turn = &v
	}
	if appliedDiff.Tilt != nil {
		v := -*appliedDiff.Tilt
		inverse.Tilt = &v
	}
	if appliedDiff.U != nil {
		v := -*appliedDiff.U
		inverse.U = &v
	}
	if appliedDiff.V != nil {
		v := -*appliedDiff.V
		inverse.V = &v
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseSideDiff(original Side, appliedDiff SideDiff) *SideDiff {
	inverse := &SideDiff{}
	if appliedDiff.Piece != nil {
		inverse.Piece = &original.Piece
	}
	if appliedDiff.DesignPiece != nil {
		inverse.DesignPiece = original.DesignPiece
	}
	if appliedDiff.Connector != nil {
		inverse.Connector = original.Connector
	}
	return inverse
}

func inverseAttributeDiff(original Attribute, appliedDiff AttributeDiff) AttributeDiff {
	inverse := AttributeDiff{}
	if appliedDiff.Key != nil {
		inverse.Key = &original.Key
	}
	if appliedDiff.Value != nil {
		inverse.Value = original.Value
	}
	if appliedDiff.Definition != nil {
		inverse.Definition = original.Definition
	}
	return inverse
}

func inverseAttributesDiff(original []Attribute, appliedDiff AttributesDiff) AttributesDiff {
	inverse := AttributesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, AttributeId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, a := range original {
			if a.Id == removed.Id {
				inverse.Added = append(inverse.Added, a)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, a := range original {
			if a.Id == updated.Attribute.Id {
				inverseDiff := inverseAttributeDiff(a, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Attribute AttributeId   `json:"attribute"`
					Diff      AttributeDiff `json:"diff"`
				}{Attribute: AttributeId{Id: a.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inversePropsDiff(original []Prop, appliedDiff PropsDiff) PropsDiff {
	inverse := PropsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, PropId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, p := range original {
			if p.Id == removed.Id {
				inverse.Added = append(inverse.Added, p)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, p := range original {
			if p.Id == updated.Prop.Id {
				inverseDiff := inversePropDiff(p, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Prop PropId   `json:"prop"`
					Diff PropDiff `json:"diff"`
				}{Prop: PropId{Id: p.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inversePropDiff(original Prop, appliedDiff PropDiff) PropDiff {
	inverse := PropDiff{}
	if appliedDiff.Quality != nil {
		inverse.Quality = &original.Quality
	}
	if appliedDiff.Value != nil {
		inverse.Value = &original.Value
	}
	if appliedDiff.Unit != nil {
		inverse.Unit = original.Unit
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseStatsDiff(original []Stat, appliedDiff StatsDiff) StatsDiff {
	inverse := StatsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, StatId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, s := range original {
			if s.Id == removed.Id {
				inverse.Added = append(inverse.Added, s)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, s := range original {
			if s.Id == updated.Stat.Id {
				inverseDiff := inverseStatDiff(s, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Stat StatId   `json:"stat"`
					Diff StatDiff `json:"diff"`
				}{Stat: StatId{Id: s.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseStatDiff(original Stat, appliedDiff StatDiff) StatDiff {
	inverse := StatDiff{}
	if appliedDiff.Quality != nil {
		inverse.Quality = &original.Quality
	}
	if appliedDiff.Min != nil {
		inverse.Min = original.Min
	}
	if appliedDiff.Max != nil {
		inverse.Max = original.Max
	}
	if appliedDiff.Unit != nil {
		inverse.Unit = original.Unit
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseLayersDiff(original []Layer, appliedDiff LayersDiff) LayersDiff {
	inverse := LayersDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, LayerId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, l := range original {
			if l.Id == removed.Id {
				inverse.Added = append(inverse.Added, l)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, l := range original {
			if l.Id == updated.Layer.Id {
				inverseDiff := inverseLayerDiff(l, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Layer LayerId   `json:"layer"`
					Diff  LayerDiff `json:"diff"`
				}{Layer: LayerId{Id: l.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseLayerDiff(original Layer, appliedDiff LayerDiff) LayerDiff {
	inverse := LayerDiff{}
	if appliedDiff.Path != nil {
		inverse.Path = &original.Path
	}
	if appliedDiff.IsHidden != nil {
		inverse.IsHidden = original.IsHidden
	}
	if appliedDiff.IsLocked != nil {
		inverse.IsLocked = original.IsLocked
	}
	if appliedDiff.Color != nil {
		inverse.Color = original.Color
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func inverseGroupsDiff(original []Group, appliedDiff GroupsDiff) GroupsDiff {
	inverse := GroupsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, GroupId{Id: added.Id})
	}
	for _, removed := range appliedDiff.Removed {
		for _, g := range original {
			if g.Id == removed.Id {
				inverse.Added = append(inverse.Added, g)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, g := range original {
			if g.Id == updated.Group.Id {
				inverseDiff := inverseGroupDiff(g, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Group GroupId   `json:"group"`
					Diff  GroupDiff `json:"diff"`
				}{Group: GroupId{Id: g.Id}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseGroupDiff(original Group, appliedDiff GroupDiff) GroupDiff {
	inverse := GroupDiff{}
	if appliedDiff.Pieces != nil {
		inverse.Pieces = original.Pieces
	}
	if appliedDiff.Name != nil {
		inverse.Name = original.Name
	}
	if appliedDiff.Color != nil {
		inverse.Color = original.Color
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

func normalizeStr(s *string) string {
	if s == nil {
		return ""
	}
	return *s
}

func normalizeInt64(p *int64) int64 {
	if p == nil {
		return 0
	}
	return *p
}

func areFolderIdsEqual(a, b *FolderId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Id == b.Id
}

func getAttributeDiff(before, after Attribute) AttributeDiff {
	diff := AttributeDiff{}
	if before.Key != after.Key {
		diff.Key = &after.Key
	}
	if !optStringEqual(before.Value, after.Value) {
		diff.Value = after.Value
	}
	if !optStringEqual(before.Definition, after.Definition) {
		diff.Definition = after.Definition
	}
	return diff
}

func isAttributeDiffEmpty(diff AttributeDiff) bool {
	return diff.Key == nil && diff.Value == nil && diff.Definition == nil
}

func getAttributesDiff(before, after []Attribute) AttributesDiff {
	diff := AttributesDiff{}
	beforeMap := make(map[string]Attribute)
	for _, a := range before {
		beforeMap[a.Id] = a
	}
	afterMap := make(map[string]Attribute)
	for _, a := range after {
		afterMap[a.Id] = a
	}
	for _, a := range before {
		if _, ok := afterMap[a.Id]; !ok {
			diff.Removed = append(diff.Removed, AttributeId{Id: a.Id})
		}
	}
	for _, a := range after {
		if _, ok := beforeMap[a.Id]; !ok {
			diff.Added = append(diff.Added, a)
		} else {
			attrDiff := getAttributeDiff(beforeMap[a.Id], a)
			if !isAttributeDiffEmpty(attrDiff) {
				diff.Updated = append(diff.Updated, struct {
					Attribute AttributeId   `json:"attribute"`
					Diff      AttributeDiff `json:"diff"`
				}{Attribute: AttributeId{Id: a.Id}, Diff: attrDiff})
			}
		}
	}
	return diff
}

func isAttributesDiffEmpty(diff AttributesDiff) bool {
	return len(diff.Added) == 0 && len(diff.Removed) == 0 && len(diff.Updated) == 0
}

func getPropsDiff(before, after []Prop) PropsDiff {
	diff := PropsDiff{}
	beforeMap := make(map[string]Prop)
	for _, p := range before {
		beforeMap[p.Id] = p
	}
	afterMap := make(map[string]Prop)
	for _, p := range after {
		afterMap[p.Id] = p
	}
	for _, p := range before {
		if _, ok := afterMap[p.Id]; !ok {
			diff.Removed = append(diff.Removed, PropId{Id: p.Id})
		}
	}
	for _, p := range after {
		if _, ok := beforeMap[p.Id]; !ok {
			diff.Added = append(diff.Added, p)
		} else {
			propDiff := getPropDiff(beforeMap[p.Id], p)
			if !isPropDiffEmpty(propDiff) {
				diff.Updated = append(diff.Updated, struct {
					Prop PropId   `json:"prop"`
					Diff PropDiff `json:"diff"`
				}{Prop: PropId{Id: p.Id}, Diff: propDiff})
			}
		}
	}
	return diff
}

func getPropDiff(before, after Prop) PropDiff {
	diff := PropDiff{}
	if before.Quality.Id != after.Quality.Id {
		diff.Quality = &after.Quality
	}
	if before.Value != after.Value {
		diff.Value = &after.Value
	}
	if normalizeStr(before.Unit) != normalizeStr(after.Unit) {
		diff.Unit = after.Unit
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isPropDiffEmpty(diff PropDiff) bool {
	return diff.Quality == nil && diff.Value == nil && diff.Unit == nil && diff.Attributes == nil
}

func getStatsDiff(before, after []Stat) StatsDiff {
	diff := StatsDiff{}
	beforeMap := make(map[string]Stat)
	for _, s := range before {
		beforeMap[s.Id] = s
	}
	afterMap := make(map[string]Stat)
	for _, s := range after {
		afterMap[s.Id] = s
	}
	for _, s := range before {
		if _, ok := afterMap[s.Id]; !ok {
			diff.Removed = append(diff.Removed, StatId{Id: s.Id})
		}
	}
	for _, s := range after {
		if _, ok := beforeMap[s.Id]; !ok {
			diff.Added = append(diff.Added, s)
		} else {
			statDiff := getStatDiff(beforeMap[s.Id], s)
			if !isStatDiffEmpty(statDiff) {
				diff.Updated = append(diff.Updated, struct {
					Stat StatId   `json:"stat"`
					Diff StatDiff `json:"diff"`
				}{Stat: StatId{Id: s.Id}, Diff: statDiff})
			}
		}
	}
	return diff
}

func getStatDiff(before, after Stat) StatDiff {
	diff := StatDiff{}
	if before.Quality.Id != after.Quality.Id {
		diff.Quality = &after.Quality
	}
	if !optFloatEqual(before.Min, after.Min) {
		diff.Min = after.Min
	}
	if !optFloatEqual(before.Max, after.Max) {
		diff.Max = after.Max
	}
	if normalizeStr(before.Unit) != normalizeStr(after.Unit) {
		diff.Unit = after.Unit
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isStatDiffEmpty(diff StatDiff) bool {
	return diff.Quality == nil && diff.Min == nil && diff.Max == nil && diff.Unit == nil && diff.Attributes == nil
}

func getLayersDiff(before, after []Layer) LayersDiff {
	diff := LayersDiff{}
	beforeMap := make(map[string]Layer)
	for _, l := range before {
		beforeMap[l.Id] = l
	}
	afterMap := make(map[string]Layer)
	for _, l := range after {
		afterMap[l.Id] = l
	}
	for _, l := range before {
		if _, ok := afterMap[l.Id]; !ok {
			diff.Removed = append(diff.Removed, LayerId{Id: l.Id})
		}
	}
	for _, l := range after {
		if _, ok := beforeMap[l.Id]; !ok {
			diff.Added = append(diff.Added, l)
		} else {
			layerDiff := getLayerDiff(beforeMap[l.Id], l)
			if !isLayerDiffEmpty(layerDiff) {
				diff.Updated = append(diff.Updated, struct {
					Layer LayerId   `json:"layer"`
					Diff  LayerDiff `json:"diff"`
				}{Layer: LayerId{Id: l.Id}, Diff: layerDiff})
			}
		}
	}
	return diff
}

func getLayerDiff(before, after Layer) LayerDiff {
	diff := LayerDiff{}
	if before.Path != after.Path {
		diff.Path = &after.Path
	}
	if !optBoolEqual(before.IsHidden, after.IsHidden) {
		diff.IsHidden = after.IsHidden
	}
	if !optBoolEqual(before.IsLocked, after.IsLocked) {
		diff.IsLocked = after.IsLocked
	}
	if normalizeStr(before.Color) != normalizeStr(after.Color) {
		diff.Color = after.Color
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isLayerDiffEmpty(diff LayerDiff) bool {
	return diff.Path == nil && diff.IsHidden == nil && diff.IsLocked == nil && diff.Color == nil && diff.Description == nil && diff.Attributes == nil
}

func getGroupsDiff(before, after []Group) GroupsDiff {
	diff := GroupsDiff{}
	beforeMap := make(map[string]Group)
	for _, g := range before {
		beforeMap[g.Id] = g
	}
	afterMap := make(map[string]Group)
	for _, g := range after {
		afterMap[g.Id] = g
	}
	for _, g := range before {
		if _, ok := afterMap[g.Id]; !ok {
			diff.Removed = append(diff.Removed, GroupId{Id: g.Id})
		}
	}
	for _, g := range after {
		if _, ok := beforeMap[g.Id]; !ok {
			diff.Added = append(diff.Added, g)
		} else {
			groupDiff := getGroupDiff(beforeMap[g.Id], g)
			if !isGroupDiffEmpty(groupDiff) {
				diff.Updated = append(diff.Updated, struct {
					Group GroupId   `json:"group"`
					Diff  GroupDiff `json:"diff"`
				}{Group: GroupId{Id: g.Id}, Diff: groupDiff})
			}
		}
	}
	return diff
}

func getGroupDiff(before, after Group) GroupDiff {
	diff := GroupDiff{}
	if normalizeStr(before.Name) != normalizeStr(after.Name) {
		diff.Name = after.Name
	}
	if normalizeStr(before.Color) != normalizeStr(after.Color) {
		diff.Color = after.Color
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isGroupDiffEmpty(diff GroupDiff) bool {
	return diff.Pieces == nil && diff.Name == nil && diff.Color == nil && diff.Description == nil && diff.Attributes == nil
}

func applyAttributeDiff(item *Attribute, diff *AttributeDiff) {
	if diff.Key != nil {
		item.Key = *diff.Key
	}
	if diff.Value != nil {
		item.Value = diff.Value
	}
	if diff.Definition != nil {
		item.Definition = diff.Definition
	}
}

func applyAttributesDiff(items *[]Attribute, diff *AttributesDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, a := range *items {
			if !removedIds[a.Id] {
				filtered = append(filtered, a)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Attribute.Id {
					applyAttributeDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyPropsDiff(items *[]Prop, diff *PropsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, p := range *items {
			if !removedIds[p.Id] {
				filtered = append(filtered, p)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Prop.Id {
					applyPropDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyPropDiff(item *Prop, diff *PropDiff) {
	if diff.Quality != nil {
		item.Quality = *diff.Quality
	}
	if diff.Value != nil {
		item.Value = *diff.Value
	}
	if diff.Unit != nil {
		item.Unit = diff.Unit
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyStatsDiff(items *[]Stat, diff *StatsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, s := range *items {
			if !removedIds[s.Id] {
				filtered = append(filtered, s)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Stat.Id {
					applyStatDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyStatDiff(item *Stat, diff *StatDiff) {
	if diff.Quality != nil {
		item.Quality = *diff.Quality
	}
	if diff.Min != nil {
		item.Min = diff.Min
	}
	if diff.Max != nil {
		item.Max = diff.Max
	}
	if diff.Unit != nil {
		item.Unit = diff.Unit
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyLayersDiff(items *[]Layer, diff *LayersDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, l := range *items {
			if !removedIds[l.Id] {
				filtered = append(filtered, l)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Layer.Id {
					applyLayerDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyLayerDiff(item *Layer, diff *LayerDiff) {
	if diff.Path != nil {
		item.Path = *diff.Path
	}
	if diff.IsHidden != nil {
		item.IsHidden = diff.IsHidden
	}
	if diff.IsLocked != nil {
		item.IsLocked = diff.IsLocked
	}
	if diff.Color != nil {
		item.Color = diff.Color
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyGroupsDiff(items *[]Group, diff *GroupsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, g := range *items {
			if !removedIds[g.Id] {
				filtered = append(filtered, g)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Group.Id {
					applyGroupDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyGroupDiff(item *Group, diff *GroupDiff) {
	if diff.Pieces != nil {
		item.Pieces = diff.Pieces
	}
	if diff.Name != nil {
		item.Name = diff.Name
	}
	if diff.Color != nil {
		item.Color = diff.Color
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func getConnectorsDiff(before, after []Connector) ConnectorsDiff {
	diff := ConnectorsDiff{}
	beforeMap := make(map[string]Connector)
	for _, c := range before {
		beforeMap[c.Id] = c
	}
	afterMap := make(map[string]Connector)
	for _, c := range after {
		afterMap[c.Id] = c
	}
	for _, c := range before {
		if _, ok := afterMap[c.Id]; !ok {
			diff.Removed = append(diff.Removed, ConnectorId{Id: c.Id})
		}
	}
	for _, c := range after {
		if _, ok := beforeMap[c.Id]; !ok {
			diff.Added = append(diff.Added, c)
		} else {
			connDiff := getConnectorDiff(beforeMap[c.Id], c)
			if !isConnectorDiffEmpty(connDiff) {
				diff.Updated = append(diff.Updated, struct {
					Connector ConnectorId   `json:"connector"`
					Diff      ConnectorDiff `json:"diff"`
				}{Connector: ConnectorId{Id: c.Id}, Diff: connDiff})
			}
		}
	}
	return diff
}

func getConnectorDiff(before, after Connector) ConnectorDiff {
	diff := ConnectorDiff{}
	if normalizeStr(before.Name) != normalizeStr(after.Name) {
		diff.Name = after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	if !arePortIdsEqual(before.Port, after.Port) {
		diff.Port = after.Port
	}
	if !optBoolEqual(before.Mandatory, after.Mandatory) {
		diff.Mandatory = after.Mandatory
	}
	if before.T != after.T {
		diff.T = &after.T
	}
	if before.Point.X != after.Point.X || before.Point.Y != after.Point.Y || before.Point.Z != after.Point.Z {
		dx := after.Point.X - before.Point.X
		dy := after.Point.Y - before.Point.Y
		dz := after.Point.Z - before.Point.Z
		diff.Point = &PointDiff{}
		if dx != 0 {
			diff.Point.X = &dx
		}
		if dy != 0 {
			diff.Point.Y = &dy
		}
		if dz != 0 {
			diff.Point.Z = &dz
		}
	}
	if before.Direction.X != after.Direction.X || before.Direction.Y != after.Direction.Y || before.Direction.Z != after.Direction.Z {
		dx := after.Direction.X - before.Direction.X
		dy := after.Direction.Y - before.Direction.Y
		dz := after.Direction.Z - before.Direction.Z
		diff.Direction = &VectorDiff{}
		if dx != 0 {
			diff.Direction.X = &dx
		}
		if dy != 0 {
			diff.Direction.Y = &dy
		}
		if dz != 0 {
			diff.Direction.Z = &dz
		}
	}
	propsDiff := getPropsDiff(before.Props, after.Props)
	if len(propsDiff.Added) > 0 || len(propsDiff.Removed) > 0 || len(propsDiff.Updated) > 0 {
		diff.Props = &propsDiff
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isConnectorDiffEmpty(diff ConnectorDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Port == nil && diff.Mandatory == nil && diff.T == nil && diff.Point == nil && diff.Direction == nil && diff.Props == nil && diff.Attributes == nil
}

func getRepresentationDiff(before, after Representation) RepresentationDiff {
	diff := RepresentationDiff{}
	if normalizeStr(before.Name) != normalizeStr(after.Name) {
		diff.Name = after.Name
	}
	if before.File.Id != after.File.Id {
		diff.File = &after.File
	}
	tagsEqual := len(before.Tags) == len(after.Tags)
	if tagsEqual {
		for i, t := range before.Tags {
			if t.Id != after.Tags[i].Id {
				tagsEqual = false
				break
			}
		}
	}
	if !tagsEqual {
		diff.Tags = after.Tags
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func getRepresentationsDiff(before, after []Representation) RepresentationsDiff {
	diff := RepresentationsDiff{}
	beforeMap := make(map[string]Representation)
	for _, m := range before {
		beforeMap[m.Id] = m
	}
	afterMap := make(map[string]Representation)
	for _, m := range after {
		afterMap[m.Id] = m
	}
	for _, m := range before {
		if _, ok := afterMap[m.Id]; !ok {
			diff.Removed = append(diff.Removed, RepresentationId{Id: m.Id})
		}
	}
	for _, m := range after {
		if bm, ok := beforeMap[m.Id]; !ok {
			diff.Added = append(diff.Added, m)
		} else {
			representationDiff := getRepresentationDiff(bm, m)
			if representationDiff.Name != nil || representationDiff.File != nil || representationDiff.Tags != nil || representationDiff.Description != nil || representationDiff.Attributes != nil {
				diff.Updated = append(diff.Updated, struct {
					Representation RepresentationId   `json:"representation"`
					Diff  RepresentationDiff `json:"diff"`
				}{
					Representation: RepresentationId{Id: m.Id},
					Diff:  representationDiff,
				})
			}
		}
	}
	return diff
}

func getPiecesDiff(before, after []Piece) PiecesDiff {
	diff := PiecesDiff{}
	beforeMap := make(map[string]Piece)
	for _, p := range before {
		beforeMap[p.Id] = p
	}
	afterMap := make(map[string]Piece)
	for _, p := range after {
		afterMap[p.Id] = p
	}
	for _, p := range before {
		if _, ok := afterMap[p.Id]; !ok {
			diff.Removed = append(diff.Removed, PieceId{Id: p.Id})
		}
	}
	for _, p := range after {
		if _, ok := beforeMap[p.Id]; !ok {
			diff.Added = append(diff.Added, p)
		} else {
			pieceDiff := getPieceDiff(beforeMap[p.Id], p)
			if !isPieceDiffEmpty(pieceDiff) {
				diff.Updated = append(diff.Updated, struct {
					Piece PieceId   `json:"piece"`
					Diff  PieceDiff `json:"diff"`
				}{Piece: PieceId{Id: p.Id}, Diff: pieceDiff})
			}
		}
	}
	return diff
}

func getPieceDiff(before, after Piece) PieceDiff {
	diff := PieceDiff{}
	if normalizeStr(before.Name) != normalizeStr(after.Name) {
		diff.Name = after.Name
	}
	if !areTypeIdsEqual(before.Type, after.Type) {
		diff.Type = after.Type
	}
	if !areDesignIdsEqual(before.Design, after.Design) {
		diff.Design = after.Design
	}
	if !optBoolEqual(before.IsHidden, after.IsHidden) {
		diff.IsHidden = after.IsHidden
	}
	if !optBoolEqual(before.IsLocked, after.IsLocked) {
		diff.IsLocked = after.IsLocked
	}
	if normalizeStr(before.Color) != normalizeStr(after.Color) {
		diff.Color = after.Color
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	if !optFloatEqual(before.Scale, after.Scale) {
		diff.Scale = after.Scale
	}
	if (before.Center == nil) != (after.Center == nil) || (before.Center != nil && after.Center != nil && (before.Center.U != after.Center.U || before.Center.V != after.Center.V)) {
		if after.Center != nil {
			diff.Center = &CoordinateDiff{U: &after.Center.U, V: &after.Center.V}
		}
	}
	if (before.MirrorPlane == nil) != (after.MirrorPlane == nil) || (before.MirrorPlane != nil && after.MirrorPlane != nil && !arePlanesEqual(*before.MirrorPlane, *after.MirrorPlane)) {
		if after.MirrorPlane != nil {
			diff.MirrorPlane = &PlaneDiff{
				Origin: &PointDiff{X: &after.MirrorPlane.Origin.X, Y: &after.MirrorPlane.Origin.Y, Z: &after.MirrorPlane.Origin.Z},
				XAxis:  &VectorDiff{X: &after.MirrorPlane.XAxis.X, Y: &after.MirrorPlane.XAxis.Y, Z: &after.MirrorPlane.XAxis.Z},
				YAxis:  &VectorDiff{X: &after.MirrorPlane.YAxis.X, Y: &after.MirrorPlane.YAxis.Y, Z: &after.MirrorPlane.YAxis.Z},
			}
		}
	}
	if (before.Plane == nil) != (after.Plane == nil) || (before.Plane != nil && after.Plane != nil && !arePlanesEqual(*before.Plane, *after.Plane)) {
		if after.Plane != nil {
			diff.Plane = &PlaneDiff{
				Origin: &PointDiff{X: &after.Plane.Origin.X, Y: &after.Plane.Origin.Y, Z: &after.Plane.Origin.Z},
				XAxis:  &VectorDiff{X: &after.Plane.XAxis.X, Y: &after.Plane.XAxis.Y, Z: &after.Plane.XAxis.Z},
				YAxis:  &VectorDiff{X: &after.Plane.YAxis.X, Y: &after.Plane.YAxis.Y, Z: &after.Plane.YAxis.Z},
			}
		}
	}
	propsDiff := getPropsDiff(before.Props, after.Props)
	if len(propsDiff.Added) > 0 || len(propsDiff.Removed) > 0 || len(propsDiff.Updated) > 0 {
		diff.Props = &propsDiff
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func arePlanesEqual(a, b Plane) bool {
	return a.Origin.X == b.Origin.X && a.Origin.Y == b.Origin.Y && a.Origin.Z == b.Origin.Z &&
		a.XAxis.X == b.XAxis.X && a.XAxis.Y == b.XAxis.Y && a.XAxis.Z == b.XAxis.Z &&
		a.YAxis.X == b.YAxis.X && a.YAxis.Y == b.YAxis.Y && a.YAxis.Z == b.YAxis.Z
}

func isPieceDiffEmpty(diff PieceDiff) bool {
	return diff.Name == nil && diff.Type == nil && diff.Design == nil && diff.Plane == nil && diff.Center == nil && diff.Scale == nil && diff.MirrorPlane == nil && diff.IsHidden == nil && diff.IsLocked == nil && diff.Color == nil && diff.Description == nil && diff.Props == nil && diff.Attributes == nil
}

func getConnectionsDiff(before, after []Connection) ConnectionsDiff {
	diff := ConnectionsDiff{}
	beforeMap := make(map[string]Connection)
	for _, c := range before {
		beforeMap[c.Id] = c
	}
	afterMap := make(map[string]Connection)
	for _, c := range after {
		afterMap[c.Id] = c
	}
	for _, c := range before {
		if _, ok := afterMap[c.Id]; !ok {
			diff.Removed = append(diff.Removed, ConnectionId{Id: c.Id})
		}
	}
	for _, c := range after {
		if _, ok := beforeMap[c.Id]; !ok {
			diff.Added = append(diff.Added, c)
		} else {
			connDiff := getConnectionDiff(beforeMap[c.Id], c)
			if !isConnectionDiffEmpty(connDiff) {
				diff.Updated = append(diff.Updated, struct {
					Connection ConnectionId   `json:"connection"`
					Diff       ConnectionDiff `json:"diff"`
				}{Connection: ConnectionId{Id: c.Id}, Diff: connDiff})
			}
		}
	}
	return diff
}

func getSideDiff(before, after Side) *SideDiff {
	diff := SideDiff{}
	changed := false
	if before.Piece.Id != after.Piece.Id {
		diff.Piece = &after.Piece
		changed = true
	}
	if (before.DesignPiece == nil) != (after.DesignPiece == nil) || (before.DesignPiece != nil && after.DesignPiece != nil && before.DesignPiece.Id != after.DesignPiece.Id) {
		diff.DesignPiece = after.DesignPiece
		changed = true
	}
	if (before.Connector == nil) != (after.Connector == nil) || (before.Connector != nil && after.Connector != nil && before.Connector.Id != after.Connector.Id) {
		diff.Connector = after.Connector
		changed = true
	}
	if !changed {
		return nil
	}
	return &diff
}

func getConnectionDiff(before, after Connection) ConnectionDiff {
	diff := ConnectionDiff{}
	connectedDiff := getSideDiff(before.Parent, after.Parent)
	if connectedDiff != nil {
		diff.Parent = connectedDiff
	}
	connectingDiff := getSideDiff(before.Child, after.Child)
	if connectingDiff != nil {
		diff.Child = connectingDiff
	}
	if before.Gap != after.Gap {
		d := after.Gap - before.Gap
		diff.Gap = &d
	}
	if before.Shift != after.Shift {
		d := after.Shift - before.Shift
		diff.Shift = &d
	}
	if before.Rise != after.Rise {
		d := after.Rise - before.Rise
		diff.Rise = &d
	}
	if before.Rotation != after.Rotation {
		d := after.Rotation - before.Rotation
		diff.Rotation = &d
	}
	if before.Turn != after.Turn {
		d := after.Turn - before.Turn
		diff.Turn = &d
	}
	if before.Tilt != after.Tilt {
		d := after.Tilt - before.Tilt
		diff.Tilt = &d
	}
	if before.U != after.U {
		d := after.U - before.U
		diff.U = &d
	}
	if before.V != after.V {
		d := after.V - before.V
		diff.V = &d
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

func isConnectionDiffEmpty(diff ConnectionDiff) bool {
	return diff.Parent == nil && diff.Child == nil && diff.Gap == nil && diff.Shift == nil && diff.Rise == nil && diff.Rotation == nil && diff.Turn == nil && diff.Tilt == nil && diff.U == nil && diff.V == nil && diff.Description == nil && diff.Attributes == nil
}

func areTypesEqual(a, b Type) bool {
	if a.Name != b.Name {
		return false
	}
	if a.Typology.Id != b.Typology.Id {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if !areFamilyIdSlicesEqual(a.Families, b.Families) {
		return false
	}
	if !optBoolEqual(a.IsAbstract, b.IsAbstract) {
		return false
	}
	if !optBoolEqual(a.Virtual, b.Virtual) {
		return false
	}
	if normalizeStr(a.Unit) != normalizeStr(b.Unit) {
		return false
	}
	if normalizeOptInt(a.Stock) != normalizeOptInt(b.Stock) {
		return false
	}
	if (a.Location == nil) != (b.Location == nil) {
		return false
	}
	if a.Location != nil && a.Location.Id != b.Location.Id {
		return false
	}
	if normalizeStr(a.Folder) != normalizeStr(b.Folder) {
		return false
	}
	if normalizeStr(a.Icon) != normalizeStr(b.Icon) {
		return false
	}
	if normalizeStr(a.Image) != normalizeStr(b.Image) {
		return false
	}
	if !areAuthorIdsEqual(a.Authors, b.Authors) {
		return false
	}
	if !areConceptIdsEqual(a.Concepts, b.Concepts) {
		return false
	}
	if len(a.Connectors) != len(b.Connectors) {
		return false
	}
	for _, ca := range a.Connectors {
		found := false
		for _, cb := range b.Connectors {
			if ca.Id == cb.Id {
				if !areConnectorsEqual(ca, cb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Representations) != len(b.Representations) {
		return false
	}
	for _, ma := range a.Representations {
		found := false
		for _, mb := range b.Representations {
			if ma.Id == mb.Id {
				if !areRepresentationsEqual(ma, mb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if !arePropsEqual(a.Props, b.Props) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areConnectorsEqual(a, b Connector) bool {
	if normalizeStr(a.Name) != normalizeStr(b.Name) {
		return false
	}
	if !floatEqual(a.Point.X, b.Point.X, 1e-9) || !floatEqual(a.Point.Y, b.Point.Y, 1e-9) || !floatEqual(a.Point.Z, b.Point.Z, 1e-9) {
		return false
	}
	if !floatEqual(a.Direction.X, b.Direction.X, 1e-9) || !floatEqual(a.Direction.Y, b.Direction.Y, 1e-9) || !floatEqual(a.Direction.Z, b.Direction.Z, 1e-9) {
		return false
	}
	if !floatEqual(a.T, b.T, 1e-9) {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if (a.Port == nil) != (b.Port == nil) {
		return false
	}
	if a.Port != nil && a.Port.Id != b.Port.Id {
		return false
	}
	if !optBoolEqual(a.Mandatory, b.Mandatory) {
		return false
	}
	if !arePropsEqual(a.Props, b.Props) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areRepresentationsEqual(a, b Representation) bool {
	if normalizeStr(a.Name) != normalizeStr(b.Name) {
		return false
	}
	if a.File.Id != b.File.Id {
		return false
	}
	if len(a.Tags) != len(b.Tags) {
		return false
	}
	for i, t := range a.Tags {
		if t.Id != b.Tags[i].Id {
			return false
		}
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areDesignsEqual(a, b Design) bool {
	if a.Name != b.Name {
		return false
	}
	if a.Typology.Id != b.Typology.Id {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if !areFamilyIdSlicesEqual(a.Families, b.Families) {
		return false
	}
	if !optBoolEqual(a.IsAbstract, b.IsAbstract) {
		return false
	}
	if normalizeStr(a.Unit) != normalizeStr(b.Unit) {
		return false
	}
	if normalizeStr(a.Folder) != normalizeStr(b.Folder) {
		return false
	}
	if !optBoolEqual(a.CanScale, b.CanScale) {
		return false
	}
	if !optBoolEqual(a.CanMirror, b.CanMirror) {
		return false
	}
	if (a.ActiveLayer == nil) != (b.ActiveLayer == nil) {
		return false
	}
	if a.ActiveLayer != nil && a.ActiveLayer.Id != b.ActiveLayer.Id {
		return false
	}
	if (a.Location == nil) != (b.Location == nil) {
		return false
	}
	if a.Location != nil && a.Location.Id != b.Location.Id {
		return false
	}
	if normalizeStr(a.Icon) != normalizeStr(b.Icon) {
		return false
	}
	if normalizeStr(a.Image) != normalizeStr(b.Image) {
		return false
	}
	if !areAuthorIdsEqual(a.Authors, b.Authors) {
		return false
	}
	if !areConceptIdsEqual(a.Concepts, b.Concepts) {
		return false
	}
	if len(a.Pieces) != len(b.Pieces) {
		return false
	}
	for _, pa := range a.Pieces {
		found := false
		for _, pb := range b.Pieces {
			if pa.Id == pb.Id {
				if !arePiecesEqual(pa, pb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if len(a.Connections) != len(b.Connections) {
		return false
	}
	for _, ca := range a.Connections {
		found := false
		for _, cb := range b.Connections {
			if ca.Id == cb.Id {
				if !areConnectionsEqual(ca, cb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if !areStatsEqual(a.Stats, b.Stats) {
		return false
	}
	if !arePropsEqual(a.Props, b.Props) {
		return false
	}
	if !areLayersEqual(a.Layers, b.Layers) {
		return false
	}
	if !areGroupsEqual(a.Groups, b.Groups) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func arePiecesEqual(a, b Piece) bool {
	if normalizeStr(a.Name) != normalizeStr(b.Name) {
		return false
	}
	if (a.Type == nil) != (b.Type == nil) {
		return false
	}
	if a.Type != nil && a.Type.Id != b.Type.Id {
		return false
	}
	if (a.Design == nil) != (b.Design == nil) {
		return false
	}
	if a.Design != nil && a.Design.Id != b.Design.Id {
		return false
	}
	if !optFloatEqual(a.Scale, b.Scale) {
		return false
	}
	if (a.Plane == nil) != (b.Plane == nil) {
		return false
	}
	if a.Plane != nil && !arePlanesEqual(*a.Plane, *b.Plane) {
		return false
	}
	if !areCoordinatesEqual(a.Center, b.Center) {
		return false
	}
	if (a.MirrorPlane == nil) != (b.MirrorPlane == nil) {
		return false
	}
	if a.MirrorPlane != nil && !arePlanesEqual(*a.MirrorPlane, *b.MirrorPlane) {
		return false
	}
	if !optBoolEqual(a.IsHidden, b.IsHidden) {
		return false
	}
	if !optBoolEqual(a.IsLocked, b.IsLocked) {
		return false
	}
	if normalizeStr(a.Color) != normalizeStr(b.Color) {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if !arePropsEqual(a.Props, b.Props) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areConnectionsEqual(a, b Connection) bool {
	if a.Parent.Piece.Id != b.Parent.Piece.Id {
		return false
	}
	if a.Child.Piece.Id != b.Child.Piece.Id {
		return false
	}
	if !areSidesEqual(a.Parent, b.Parent) {
		return false
	}
	if !areSidesEqual(a.Child, b.Child) {
		return false
	}
	if !floatEqual(a.Gap, b.Gap, 1e-9) {
		return false
	}
	if !floatEqual(a.Shift, b.Shift, 1e-9) {
		return false
	}
	if !floatEqual(a.Rise, b.Rise, 1e-9) {
		return false
	}
	if !floatEqual(a.Rotation, b.Rotation, 1e-9) {
		return false
	}
	if !floatEqual(a.Turn, b.Turn, 1e-9) {
		return false
	}
	if !floatEqual(a.Tilt, b.Tilt, 1e-9) {
		return false
	}
	if !floatEqual(a.U, b.U, 1e-9) {
		return false
	}
	if !floatEqual(a.V, b.V, 1e-9) {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areTagsEqual(a, b Tag) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if normalizeStr(a.Icon) != normalizeStr(b.Icon) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areConceptsEqual(a, b Concept) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if normalizeStr(a.Icon) != normalizeStr(b.Icon) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func arePortsEqual(a, b Port) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if normalizeStr(a.Icon) != normalizeStr(b.Icon) {
		return false
	}
	if (a.MaxChildren == nil) != (b.MaxChildren == nil) {
		return false
	}
	if a.MaxChildren != nil && *a.MaxChildren != *b.MaxChildren {
		return false
	}
	if !arePortIdSlicesEqual(a.CompatiblePorts, b.CompatiblePorts) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areFamiliesEqual(a, b Family) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if normalizeStr(a.Icon) != normalizeStr(b.Icon) {
		return false
	}
	if len(a.Ports) != len(b.Ports) {
		return false
	}
	for _, pa := range a.Ports {
		found := false
		for _, pb := range b.Ports {
			if pa.Id == pb.Id {
				if !arePortsEqual(pa, pb) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areFilesEqual(a, b File) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Remote) != normalizeStr(b.Remote) {
		return false
	}
	if normalizeStr(a.Blob) != normalizeStr(b.Blob) {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areFoldersEqual(a, b Folder) bool {
	if a.Name != b.Name {
		return false
	}
	if (a.Parent == nil) != (b.Parent == nil) {
		return false
	}
	if a.Parent != nil && a.Parent.Id != b.Parent.Id {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areAuthorsEqual(a, b Author) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Email) != normalizeStr(b.Email) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

func areCoordinatesEqual(a, b *Coordinate) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return floatEqual(a.U, b.U, 1e-9) && floatEqual(a.V, b.V, 1e-9)
}

func areSidesEqual(a, b Side) bool {
	if a.Piece.Id != b.Piece.Id {
		return false
	}
	if (a.DesignPiece == nil) != (b.DesignPiece == nil) {
		return false
	}
	if a.DesignPiece != nil && a.DesignPiece.Id != b.DesignPiece.Id {
		return false
	}
	if (a.Connector == nil) != (b.Connector == nil) {
		return false
	}
	if a.Connector != nil && a.Connector.Id != b.Connector.Id {
		return false
	}
	return true
}

func areStatsEqual(a, b []Stat) bool {
	if len(a) != len(b) {
		return false
	}
	for _, sa := range a {
		found := false
		for _, sb := range b {
			if sa.Id == sb.Id {
				if sa.Quality.Id != sb.Quality.Id {
					return false
				}
				if !optFloatEqual(sa.Min, sb.Min) || !optFloatEqual(sa.Max, sb.Max) {
					return false
				}
				if normalizeStr(sa.Unit) != normalizeStr(sb.Unit) {
					return false
				}
				if !areAttributesEqual(sa.Attributes, sb.Attributes) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

func areLayersEqual(a, b []Layer) bool {
	if len(a) != len(b) {
		return false
	}
	for _, la := range a {
		found := false
		for _, lb := range b {
			if la.Id == lb.Id {
				if la.Path != lb.Path {
					return false
				}
				if !optBoolEqual(la.IsHidden, lb.IsHidden) {
					return false
				}
				if !optBoolEqual(la.IsLocked, lb.IsLocked) {
					return false
				}
				if normalizeStr(la.Color) != normalizeStr(lb.Color) {
					return false
				}
				if normalizeStr(la.Description) != normalizeStr(lb.Description) {
					return false
				}
				if !areAttributesEqual(la.Attributes, lb.Attributes) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

func areGroupsEqual(a, b []Group) bool {
	if len(a) != len(b) {
		return false
	}
	for _, ga := range a {
		found := false
		for _, gb := range b {
			if ga.Id == gb.Id {
				if normalizeStr(ga.Name) != normalizeStr(gb.Name) {
					return false
				}
				if normalizeStr(ga.Color) != normalizeStr(gb.Color) {
					return false
				}
				if normalizeStr(ga.Description) != normalizeStr(gb.Description) {
					return false
				}
				if !areAttributesEqual(ga.Attributes, gb.Attributes) {
					return false
				}
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

// ▶️ApplyKitDiff applies a forward diff to a kit, mutating it in place.
func ApplyKitDiff(kit *Kit, diff *KitDiff) {
	if diff.Name != nil {
		kit.Name = *diff.Name
	}
	if diff.Version != nil {
		kit.Version = *diff.Version
	}
	if diff.Description != nil {
		kit.Description = diff.Description
	}
	if diff.Icon != nil {
		kit.Icon = diff.Icon
	}
	if diff.Image != nil {
		kit.Image = diff.Image
	}
	if diff.Remote != nil {
		kit.Remote = diff.Remote
	}
	if diff.Homepage != nil {
		kit.Homepage = diff.Homepage
	}
	if diff.License != nil {
		kit.License = diff.License
	}
	if diff.Preview != nil {
		kit.Preview = diff.Preview
	}
	if diff.Typologies != nil {
		applyTypologiesDiff(kit, diff.Typologies)
		KitFlattenTypesDesigns(kit)
	}
	if diff.Tags != nil {
		applyTagsDiff(&kit.Tags, diff.Tags)
	}
	if diff.Concepts != nil {
		applyConceptsDiff(&kit.Concepts, diff.Concepts)
	}
	if diff.Families != nil {
		applyFamiliesDiff(&kit.Families, diff.Families)
	}
	if diff.Files != nil {
		applyFilesDiff(&kit.Files, diff.Files)
	}
	if diff.Folders != nil {
		applyFoldersDiff(&kit.Folders, diff.Folders)
	}
	if diff.Authors != nil {
		applyAuthorsDiff(&kit.Authors, diff.Authors)
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&kit.Attributes, diff.Attributes)
	}
}

// KitDiffValidationNote is one machine-addressable validation message.
type KitDiffValidationNote struct {
	Code    string `json:"code,omitempty"`
	Message string `json:"message"`
}

// KitDiffValidationResult is returned by ValidateKitDiff.
type KitDiffValidationResult struct {
	Ok       bool                    `json:"ok"`
	Errors   []KitDiffValidationNote `json:"errors"`
	Warnings []KitDiffValidationNote `json:"warnings"`
	Diff     *KitDiff                `json:"diff,omitempty"`
}

type kitDiffValidateCtx struct {
	errors   []KitDiffValidationNote
	warnings []KitDiffValidationNote
	heal     bool
}

func kitdiffPush(ctx *kitDiffValidateCtx, kind string, code, msg string) {
	n := KitDiffValidationNote{Code: code, Message: msg}
	if kind == "errors" {
		ctx.errors = append(ctx.errors, n)
	} else {
		ctx.warnings = append(ctx.warnings, n)
	}
}

func kitdiffDeepEqualJSON(a, b any) bool {
	ja, e1 := json.Marshal(a)
	jb, e2 := json.Marshal(b)
	return e1 == nil && e2 == nil && string(ja) == string(jb)
}

func validateIdCollectionDiffGo(ctx *kitDiffValidateCtx, path, idKey string, base []map[string]any, raw map[string]any, onUpdated func(item map[string]any, diff map[string]any, p string)) map[string]any {
	if raw == nil {
		return nil
	}
	baseBy := map[string]map[string]any{}
	for _, it := range base {
		if g, ok := it["id"].(string); ok {
			baseBy[g] = it
		}
	}
	removedSet := map[string]bool{}
	if arr, ok := raw["removed"].([]any); ok {
		for _, r := range arr {
			if rm, ok := r.(map[string]any); ok {
				if g, ok := rm["id"].(string); ok {
					removedSet[g] = true
				}
			}
		}
	}
	afterRemove := map[string]bool{}
	for g := range baseBy {
		if !removedSet[g] {
			afterRemove[g] = true
		}
	}
	var hRem, hUpd, hAdd []any
	if ctx.heal {
		if x, ok := raw["removed"].([]any); ok {
			hRem = append([]any(nil), x...)
		}
		if x, ok := raw["updated"].([]any); ok {
			hUpd = append([]any(nil), x...)
		}
		if x, ok := raw["added"].([]any); ok {
			hAdd = append([]any(nil), x...)
		}
	}
	if arr, ok := raw["removed"].([]any); ok {
		for _, r := range arr {
			rm, ok := r.(map[string]any)
			if !ok {
				continue
			}
			rg, _ := rm["id"].(string)
			if _, ok := baseBy[rg]; !ok {
				kitdiffPush(ctx, "warnings", "kitdiff.remove.missing-target", path+": remove references missing "+idKey+" "+rg)
				if ctx.heal && hRem != nil {
					nr := hRem[:0]
					for _, x := range hRem {
						if m, ok := x.(map[string]any); ok && m["id"] == rg {
							continue
						}
						nr = append(nr, x)
					}
					hRem = nr
				}
			}
		}
	}
	addBy := map[string]map[string]any{}
	if arr, ok := raw["added"].([]any); ok {
		for _, a := range arr {
			if am, ok := a.(map[string]any); ok {
				if g, ok := am["id"].(string); ok {
					addBy[g] = am
				}
			}
		}
	}
	if arr, ok := raw["removed"].([]any); ok {
		for _, r := range arr {
			rm, ok := r.(map[string]any)
			if !ok {
				continue
			}
			rg, _ := rm["id"].(string)
			orig := baseBy[rg]
			add := addBy[rg]
			if orig != nil && add != nil && kitdiffDeepEqualJSON(orig, add) {
				kitdiffPush(ctx, "warnings", "kitdiff.cycle.no-operation-restore", path+": removed and re-added "+idKey+" "+rg+" are deeply equal (no effective change)")
				if ctx.heal {
					if hRem != nil {
						nr := hRem[:0]
						for _, x := range hRem {
							if m, ok := x.(map[string]any); ok && m["id"] == rg {
								continue
							}
							nr = append(nr, x)
						}
						hRem = nr
					}
					if hAdd != nil {
						na := hAdd[:0]
						for _, x := range hAdd {
							if m, ok := x.(map[string]any); ok && m["id"] == rg {
								continue
							}
							na = append(na, x)
						}
						hAdd = na
					}
				}
			}
		}
	}
	seenAdd := map[string]bool{}
	if arr, ok := raw["added"].([]any); ok {
		for _, a := range arr {
			am, ok := a.(map[string]any)
			if !ok {
				continue
			}
			ag, _ := am["id"].(string)
			if seenAdd[ag] {
				kitdiffPush(ctx, "errors", "kitdiff.add.duplicate-in-diff", path+": duplicate added "+idKey+" id "+ag)
				if ctx.heal && hAdd != nil {
					first := true
					na := hAdd[:0]
					for _, x := range hAdd {
						m, ok := x.(map[string]any)
						if !ok {
							na = append(na, x)
							continue
						}
						if g, _ := m["id"].(string); g == ag {
							if first {
								na = append(na, x)
								first = false
							}
							continue
						}
						na = append(na, x)
					}
					hAdd = na
				}
			}
			seenAdd[ag] = true
			if afterRemove[ag] {
				kitdiffPush(ctx, "errors", "kitdiff.add.duplicate-id", path+": cannot add "+idKey+" "+ag+" that still exists after removes")
				if ctx.heal && hAdd != nil {
					na := hAdd[:0]
					for _, x := range hAdd {
						if m, ok := x.(map[string]any); ok && m["id"] == ag {
							continue
						}
						na = append(na, x)
					}
					hAdd = na
				}
			}
		}
	}
	if arr, ok := raw["updated"].([]any); ok {
		for _, u := range arr {
			um, ok := u.(map[string]any)
			if !ok {
				continue
			}
			idObj, ok := um[idKey].(map[string]any)
			if !ok {
				continue
			}
			gid, _ := idObj["id"].(string)
			p := path + "." + idKey + "[" + gid + "]"
			if gid == "" {
				kitdiffPush(ctx, "errors", "kitdiff.update.bad-id", p+": missing "+idKey+" id")
				if ctx.heal && hUpd != nil {
					hUpd = filterUpdatesById(hUpd, idKey, gid)
				}
				continue
			}
			if !afterRemove[gid] {
				kitdiffPush(ctx, "errors", "kitdiff.update.missing-target", p+": update targets "+idKey+" not present after removes")
				if ctx.heal && hUpd != nil {
					hUpd = filterUpdatesById(hUpd, idKey, gid)
				}
				continue
			}
			item := baseBy[gid]
			if item == nil {
				kitdiffPush(ctx, "errors", "kitdiff.update.missing-base", p+": "+idKey+" not found in base kit")
				if ctx.heal && hUpd != nil {
					hUpd = filterUpdatesById(hUpd, idKey, gid)
				}
				continue
			}
			dm, _ := um["diff"].(map[string]any)
			if onUpdated != nil {
				onUpdated(item, dm, p)
			}
		}
	}
	if !ctx.heal {
		return raw
	}
	out := map[string]any{}
	if len(hRem) > 0 {
		out["removed"] = hRem
	}
	if len(hUpd) > 0 {
		out["updated"] = hUpd
	}
	if len(hAdd) > 0 {
		out["added"] = hAdd
	}
	if len(out) == 0 {
		return nil
	}
	return out
}

func filterUpdatesById(updates []any, idKey, gid string) []any {
	n := updates[:0]
	for _, u := range updates {
		um, ok := u.(map[string]any)
		if !ok {
			n = append(n, u)
			continue
		}
		idObj, ok := um[idKey].(map[string]any)
		if !ok {
			n = append(n, u)
			continue
		}
		if g, _ := idObj["id"].(string); g == gid {
			continue
		}
		n = append(n, u)
	}
	return n
}

func kitToMap(k Kit) map[string]any {
	b, _ := json.Marshal(k)
	var m map[string]any
	_ = json.Unmarshal(b, &m)
	return m
}

func kitDiffToMap(d KitDiff) map[string]any {
	b, _ := json.Marshal(d)
	var m map[string]any
	_ = json.Unmarshal(b, &m)
	return m
}

func mapToKitDiff(m map[string]any) KitDiff {
	b, _ := json.Marshal(m)
	var d KitDiff
	_ = json.Unmarshal(b, &d)
	return d
}

func validateDesignDiffNestedGo(ctx *kitDiffValidateCtx, kitMap map[string]any, path string, design map[string]any, diff map[string]any, refs map[string]map[string]bool) {
	typeIds := refs["typeIds"]
	designIds := refs["designIds"]
	authorIds := refs["authorIds"]
	if da, ok := diff["authors"]; ok {
		if arr, ok := da.([]any); ok {
			for _, a := range arr {
				am, ok := a.(map[string]any)
				if !ok {
					continue
				}
				if g, ok := am["id"].(string); ok && g != "" && !authorIds[g] {
					kitdiffPush(ctx, "errors", "kitdiff.ref.author-missing", path+": author "+g+" not in kit")
				}
			}
		} else if dm, ok := da.(map[string]any); ok {
			authArr := toMapSlice(kitMap["authors"])
			validateIdCollectionDiffGo(ctx, path+".authors", "author", authArr, dm, nil)
		}
	}
	if pd, ok := diff["pieces"].(map[string]any); ok {
		pieces := toMapSlice(design["pieces"])
		validateIdCollectionDiffGo(ctx, path+".pieces", "piece", pieces, pd, nil)
		if arr, ok := pd["added"].([]any); ok {
			for _, a := range arr {
				am, ok := a.(map[string]any)
				if !ok {
					continue
				}
				var tg string
				if t, ok := am["type"].(map[string]any); ok {
					tg, _ = t["id"].(string)
				}
				if tg != "" && !typeIds[tg] {
					kitdiffPush(ctx, "errors", "kitdiff.ref.piece-type-missing", path+".pieces.added: type "+tg+" not in kit")
				}
				var dg string
				if d, ok := am["design"].(map[string]any); ok {
					dg, _ = d["id"].(string)
				}
				if dg != "" && !designIds[dg] {
					kitdiffPush(ctx, "errors", "kitdiff.ref.piece-design-missing", path+".pieces.added: subdesign "+dg+" not in kit")
				}
			}
		}
	}
}

func toMapSlice(v any) []map[string]any {
	arr, ok := v.([]any)
	if !ok {
		return nil
	}
	out := make([]map[string]any, 0, len(arr))
	for _, x := range arr {
		if m, ok := x.(map[string]any); ok {
			out = append(out, m)
		}
	}
	return out
}

func idSetFromKitEntities(v any) map[string]bool {
	s := map[string]bool{}
	for _, m := range toMapSlice(v) {
		if g, ok := m["id"].(string); ok {
			s[g] = true
		}
	}
	return s
}

// ValidateKitDiff checks whether a KitDiff can be applied faithfully to kit; heal returns a JSON-scrubbed diff.
func ValidateKitDiff(kit Kit, diff KitDiff, heal bool) KitDiffValidationResult {
	ctx := &kitDiffValidateCtx{heal: heal}
	km := kitToMap(kit)
	dm := kitDiffToMap(diff)
	var outDiff map[string]any
	if heal {
		b, _ := json.Marshal(dm)
		_ = json.Unmarshal(b, &outDiff)
	}
	refs := map[string]map[string]bool{
		"typeIds":    idSetFromKitEntities(km["types"]),
		"designIds":  idSetFromKitEntities(km["designs"]),
		"qualityIds": idSetFromKitEntities(km["qualities"]),
		"fileIds":    idSetFromKitEntities(km["files"]),
		"portIds":    idSetFromKitEntities(km["ports"]),
		"conceptIds": idSetFromKitEntities(km["concepts"]),
		"authorIds":  idSetFromKitEntities(km["authors"]),
	}
	runColl := func(key, idKey, arrKey string, onUpd func(item map[string]any, ddf map[string]any, p string)) {
		part, ok := dm[key].(map[string]any)
		if !ok || part == nil {
			return
		}
		fixed := validateIdCollectionDiffGo(ctx, key, idKey, toMapSlice(km[arrKey]), part, onUpd)
		if heal && outDiff != nil {
			if fixed != nil && len(fixed) > 0 {
				outDiff[key] = fixed
			} else {
				delete(outDiff, key)
			}
		}
	}
	runColl("types", "type", "types", nil)
	runColl("designs", "design", "designs", func(item map[string]any, ddf map[string]any, p string) {
		validateDesignDiffNestedGo(ctx, km, p, item, ddf, refs)
	})
	runColl("tags", "tag", "tags", nil)
	runColl("concepts", "concept", "concepts", nil)
	runColl("ports", "port", "ports", nil)
	runColl("qualities", "quality", "qualities", nil)
	runColl("files", "file", "files", nil)
	runColl("folders", "folder", "folders", nil)
	runColl("authors", "author", "authors", nil)
	if a, ok := dm["attributes"].(map[string]any); ok {
		validateIdCollectionDiffGo(ctx, "kit.attributes", "attribute", toMapSlice(km["attributes"]), a, nil)
	}
	res := KitDiffValidationResult{Ok: len(ctx.errors) == 0, Errors: ctx.errors, Warnings: ctx.warnings}
	if heal && outDiff != nil {
		d := mapToKitDiff(outDiff)
		res.Diff = &d
	}
	return res
}

func applyTypesDiff(items *[]Type, diff *TypesDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, t := range *items {
			if !removedIds[t.Id] {
				filtered = append(filtered, t)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Type.Id {
					applyTypeDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyTypeDiff(item *Type, diff *TypeDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.Families != nil {
		item.Families = diff.Families
	}
	if diff.IsAbstract != nil {
		item.IsAbstract = diff.IsAbstract
	}
	if diff.HasField("virtual") {
		item.Virtual = diff.Virtual
	}
	if diff.HasField("unit") {
		item.Unit = diff.Unit
	}
	if diff.Stock != nil {
		item.Stock = diff.Stock
	}
	if diff.Location != nil {
		item.Location = diff.Location
	}
	if diff.Folder != nil {
		item.Folder = diff.Folder
	}
	if diff.Icon != nil {
		item.Icon = diff.Icon
	}
	if diff.Image != nil {
		item.Image = diff.Image
	}
	if diff.HasField("description") {
		item.Description = diff.Description
	}
	if diff.Authors != nil {
		item.Authors = diff.Authors
	}
	if diff.Concepts != nil {
		item.Concepts = diff.Concepts
	}
	if diff.Representations != nil {
		applyRepresentationsDiff(&item.Representations, diff.Representations)
	}
	if diff.Connectors != nil {
		applyConnectorsDiff(&item.Connectors, diff.Connectors)
	}
	if diff.Props != nil {
		applyPropsDiff(&item.Props, diff.Props)
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyConnectorsDiff(items *[]Connector, diff *ConnectorsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, c := range *items {
			if !removedIds[c.Id] {
				filtered = append(filtered, c)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Connector.Id {
					applyConnectorDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyConnectorDiff(item *Connector, diff *ConnectorDiff) {
	if diff.Name != nil {
		item.Name = diff.Name
	}
	if diff.T != nil {
		item.T = *diff.T
	}
	if diff.Point != nil {
		if diff.Point.X != nil {
			item.Point.X += *diff.Point.X
		}
		if diff.Point.Y != nil {
			item.Point.Y += *diff.Point.Y
		}
		if diff.Point.Z != nil {
			item.Point.Z += *diff.Point.Z
		}
	}
	if diff.Direction != nil {
		if diff.Direction.X != nil {
			item.Direction.X += *diff.Direction.X
		}
		if diff.Direction.Y != nil {
			item.Direction.Y += *diff.Direction.Y
		}
		if diff.Direction.Z != nil {
			item.Direction.Z += *diff.Direction.Z
		}
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Port != nil {
		item.Port = diff.Port
	}
	if diff.Mandatory != nil {
		item.Mandatory = diff.Mandatory
	}
	if diff.Props != nil {
		applyPropsDiff(&item.Props, diff.Props)
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyRepresentationsDiff(items *[]Representation, diff *RepresentationsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, m := range *items {
			if !removedIds[m.Id] {
				filtered = append(filtered, m)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Representation.Id {
					applyRepresentationDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyRepresentationDiff(item *Representation, diff *RepresentationDiff) {
	if diff.Name != nil {
		item.Name = diff.Name
	}
	if diff.File != nil {
		item.File = *diff.File
	}
	if diff.Tags != nil {
		item.Tags = diff.Tags
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyDesignsDiff(items *[]Design, diff *DesignsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, d := range *items {
			if !removedIds[d.Id] {
				filtered = append(filtered, d)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Design.Id {
					applyDesignDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyDesignDiff(item *Design, diff *DesignDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.Families != nil {
		item.Families = diff.Families
	}
	if diff.IsAbstract != nil {
		item.IsAbstract = diff.IsAbstract
	}
	if diff.Unit != nil {
		item.Unit = diff.Unit
	}
	if diff.Folder != nil {
		item.Folder = diff.Folder
	}
	if diff.CanScale != nil {
		item.CanScale = diff.CanScale
	}
	if diff.CanMirror != nil {
		item.CanMirror = diff.CanMirror
	}
	if diff.ActiveLayer != nil {
		item.ActiveLayer = diff.ActiveLayer
	}
	if diff.Location != nil {
		item.Location = diff.Location
	}
	if diff.Icon != nil {
		item.Icon = diff.Icon
	}
	if diff.Image != nil {
		item.Image = diff.Image
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Authors != nil {
		item.Authors = diff.Authors
	}
	if diff.Concepts != nil {
		item.Concepts = diff.Concepts
	}
	if diff.Pieces != nil {
		applyPiecesDiff(&item.Pieces, diff.Pieces)
	}
	if diff.Connections != nil {
		applyConnectionsDiff(&item.Connections, diff.Connections)
	}
	if diff.Stats != nil {
		applyStatsDiff(&item.Stats, diff.Stats)
	}
	if diff.Props != nil {
		applyPropsDiff(&item.Props, diff.Props)
	}
	if diff.Layers != nil {
		applyLayersDiff(&item.Layers, diff.Layers)
	}
	if diff.Groups != nil {
		applyGroupsDiff(&item.Groups, diff.Groups)
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

// 📌DesignWithDiff creates a mixed design keeping old entities with diff status annotations.
// annotate each with a compose.diffStatus attribute (unchanged/modified/removed/added),
// 🗑️keep deleted entities in place marked as removed, and append added entities marked as added.
func DesignWithDiff(base Design, diff DesignDiff) Design {
	statusAttr := func(status string) Attribute {
		return Attribute{
			Id:  "compose.diffStatus." + status,
			Key:   "compose.diffStatus",
			Value: ptrString(status),
		}
	}

	removedPieceIds := make(map[string]bool)
	updatedPieceMap := make(map[string]PieceDiff)
	if diff.Pieces != nil {
		for _, r := range diff.Pieces.Removed {
			removedPieceIds[r.Id] = true
		}
		for _, u := range diff.Pieces.Updated {
			updatedPieceMap[u.Piece.Id] = u.Diff
		}
	}

	removedConnIds := make(map[string]bool)
	updatedConnMap := make(map[string]ConnectionDiff)
	if diff.Connections != nil {
		for _, r := range diff.Connections.Removed {
			removedConnIds[r.Id] = true
		}
		for _, u := range diff.Connections.Updated {
			updatedConnMap[u.Connection.Id] = u.Diff
		}
	}

	resultPieces := make([]Piece, 0, len(base.Pieces))
	for _, p := range base.Pieces {
		pc := p
		if removedPieceIds[pc.Id] {
			attrs := append([]Attribute{}, pc.Attributes...)
			attrs = append(attrs, statusAttr("removed"))
			pc.Attributes = attrs
		} else if pDiff, ok := updatedPieceMap[pc.Id]; ok {
			basePlane := pc.Plane
			baseCenter := pc.Center
			applyPieceDiff(&pc, &pDiff)
			// 📌Preserve base geometry so modified pieces stay in place and only get recolored.
			pc.Plane = basePlane
			pc.Center = baseCenter
			attrs := append([]Attribute{}, pc.Attributes...)
			attrs = append(attrs, statusAttr("modified"))
			pc.Attributes = attrs
		} else {
			attrs := append([]Attribute{}, pc.Attributes...)
			attrs = append(attrs, statusAttr("unchanged"))
			pc.Attributes = attrs
		}
		resultPieces = append(resultPieces, pc)
	}
	if diff.Pieces != nil {
		for _, added := range diff.Pieces.Added {
			ac := added
			attrs := append([]Attribute{}, ac.Attributes...)
			attrs = append(attrs, statusAttr("added"))
			ac.Attributes = attrs
			resultPieces = append(resultPieces, ac)
		}
	}

	resultConns := make([]Connection, 0, len(base.Connections))
	for _, c := range base.Connections {
		cc := c
		if removedConnIds[cc.Id] {
			attrs := append([]Attribute{}, cc.Attributes...)
			attrs = append(attrs, statusAttr("removed"))
			cc.Attributes = attrs
		} else if cDiff, ok := updatedConnMap[cc.Id]; ok {
			applyConnectionDiff(&cc, &cDiff)
			attrs := append([]Attribute{}, cc.Attributes...)
			attrs = append(attrs, statusAttr("modified"))
			cc.Attributes = attrs
		} else {
			attrs := append([]Attribute{}, cc.Attributes...)
			attrs = append(attrs, statusAttr("unchanged"))
			cc.Attributes = attrs
		}
		resultConns = append(resultConns, cc)
	}
	if diff.Connections != nil {
		for _, added := range diff.Connections.Added {
			ac := added
			attrs := append([]Attribute{}, ac.Attributes...)
			attrs = append(attrs, statusAttr("added"))
			ac.Attributes = attrs
			resultConns = append(resultConns, ac)
		}
	}

	result := base
	result.Pieces = resultPieces
	result.Connections = resultConns
	return result
}

func applyPiecesDiff(items *[]Piece, diff *PiecesDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, p := range *items {
			if !removedIds[p.Id] {
				filtered = append(filtered, p)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Piece.Id {
					applyPieceDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyPieceDiff(item *Piece, diff *PieceDiff) {
	if diff.Name != nil {
		item.Name = diff.Name
	}
	if diff.Type != nil {
		item.Type = diff.Type
	}
	if diff.Design != nil {
		item.Design = diff.Design
	}
	if diff.Scale != nil {
		item.Scale = diff.Scale
	}
	if diff.Plane != nil {
		if item.Plane == nil {
			item.Plane = &Plane{}
		}
		if diff.Plane.Origin != nil {
			if diff.Plane.Origin.X != nil {
				item.Plane.Origin.X = *diff.Plane.Origin.X
			}
			if diff.Plane.Origin.Y != nil {
				item.Plane.Origin.Y = *diff.Plane.Origin.Y
			}
			if diff.Plane.Origin.Z != nil {
				item.Plane.Origin.Z = *diff.Plane.Origin.Z
			}
		}
		if diff.Plane.XAxis != nil {
			if diff.Plane.XAxis.X != nil {
				item.Plane.XAxis.X = *diff.Plane.XAxis.X
			}
			if diff.Plane.XAxis.Y != nil {
				item.Plane.XAxis.Y = *diff.Plane.XAxis.Y
			}
			if diff.Plane.XAxis.Z != nil {
				item.Plane.XAxis.Z = *diff.Plane.XAxis.Z
			}
		}
		if diff.Plane.YAxis != nil {
			if diff.Plane.YAxis.X != nil {
				item.Plane.YAxis.X = *diff.Plane.YAxis.X
			}
			if diff.Plane.YAxis.Y != nil {
				item.Plane.YAxis.Y = *diff.Plane.YAxis.Y
			}
			if diff.Plane.YAxis.Z != nil {
				item.Plane.YAxis.Z = *diff.Plane.YAxis.Z
			}
		}
	}
	if diff.Center != nil {
		if item.Center == nil {
			item.Center = &Coordinate{}
		}
		if diff.Center.U != nil {
			item.Center.U = *diff.Center.U
		}
		if diff.Center.V != nil {
			item.Center.V = *diff.Center.V
		}
	}
	if diff.MirrorPlane != nil {
		if item.MirrorPlane == nil {
			item.MirrorPlane = &Plane{}
		}
		if diff.MirrorPlane.Origin != nil {
			if diff.MirrorPlane.Origin.X != nil {
				item.MirrorPlane.Origin.X = *diff.MirrorPlane.Origin.X
			}
			if diff.MirrorPlane.Origin.Y != nil {
				item.MirrorPlane.Origin.Y = *diff.MirrorPlane.Origin.Y
			}
			if diff.MirrorPlane.Origin.Z != nil {
				item.MirrorPlane.Origin.Z = *diff.MirrorPlane.Origin.Z
			}
		}
		if diff.MirrorPlane.XAxis != nil {
			if diff.MirrorPlane.XAxis.X != nil {
				item.MirrorPlane.XAxis.X = *diff.MirrorPlane.XAxis.X
			}
			if diff.MirrorPlane.XAxis.Y != nil {
				item.MirrorPlane.XAxis.Y = *diff.MirrorPlane.XAxis.Y
			}
			if diff.MirrorPlane.XAxis.Z != nil {
				item.MirrorPlane.XAxis.Z = *diff.MirrorPlane.XAxis.Z
			}
		}
		if diff.MirrorPlane.YAxis != nil {
			if diff.MirrorPlane.YAxis.X != nil {
				item.MirrorPlane.YAxis.X = *diff.MirrorPlane.YAxis.X
			}
			if diff.MirrorPlane.YAxis.Y != nil {
				item.MirrorPlane.YAxis.Y = *diff.MirrorPlane.YAxis.Y
			}
			if diff.MirrorPlane.YAxis.Z != nil {
				item.MirrorPlane.YAxis.Z = *diff.MirrorPlane.YAxis.Z
			}
		}
	}
	if diff.IsHidden != nil {
		item.IsHidden = diff.IsHidden
	}
	if diff.IsLocked != nil {
		item.IsLocked = diff.IsLocked
	}
	if diff.Color != nil {
		item.Color = diff.Color
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Props != nil {
		applyPropsDiff(&item.Props, diff.Props)
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyConnectionsDiff(items *[]Connection, diff *ConnectionsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, c := range *items {
			if !removedIds[c.Id] {
				filtered = append(filtered, c)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Connection.Id {
					applyConnectionDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyConnectionDiff(item *Connection, diff *ConnectionDiff) {
	if diff.Parent != nil {
		applySideDiff(&item.Parent, diff.Parent)
	}
	if diff.Child != nil {
		applySideDiff(&item.Child, diff.Child)
	}
	if diff.Gap != nil {
		item.Gap = item.Gap + *diff.Gap
	}
	if diff.Shift != nil {
		item.Shift = item.Shift + *diff.Shift
	}
	if diff.Rise != nil {
		item.Rise = item.Rise + *diff.Rise
	}
	if diff.Rotation != nil {
		item.Rotation = item.Rotation + *diff.Rotation
	}
	if diff.Turn != nil {
		item.Turn = item.Turn + *diff.Turn
	}
	if diff.Tilt != nil {
		item.Tilt = item.Tilt + *diff.Tilt
	}
	if diff.U != nil {
		item.U = item.U + *diff.U
	}
	if diff.V != nil {
		item.V = item.V + *diff.V
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applySideDiff(item *Side, diff *SideDiff) {
	if diff.Piece != nil {
		item.Piece = *diff.Piece
	}
	if diff.DesignPiece != nil {
		item.DesignPiece = diff.DesignPiece
	}
	if diff.Connector != nil {
		item.Connector = diff.Connector
	}
}

func applyTagsDiff(items *[]Tag, diff *TagsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, t := range *items {
			if !removedIds[t.Id] {
				filtered = append(filtered, t)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Tag.Id {
					applyTagDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyTagDiff(item *Tag, diff *TagDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.HasField("description") {
		item.Description = diff.Description
	}
	if diff.HasField("icon") {
		item.Icon = diff.Icon
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyConceptsDiff(items *[]Concept, diff *ConceptsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, c := range *items {
			if !removedIds[c.Id] {
				filtered = append(filtered, c)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Concept.Id {
					applyConceptDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyConceptDiff(item *Concept, diff *ConceptDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.HasField("description") {
		item.Description = diff.Description
	}
	if diff.HasField("icon") {
		item.Icon = diff.Icon
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyPortsDiff(items *[]Port, diff *PortsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, i := range *items {
			if !removedIds[i.Id] {
				filtered = append(filtered, i)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Port.Id {
					applyPortDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyPortDiff(item *Port, diff *PortDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.HasField("description") {
		item.Description = diff.Description
	}
	if diff.HasField("icon") {
		item.Icon = diff.Icon
	}
	if diff.HasField("maxChildren") {
		item.MaxChildren = diff.MaxChildren
	}
	if diff.CompatiblePorts != nil {
		item.CompatiblePorts = diff.CompatiblePorts
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyTypologiesDiff(kit *Kit, diff *TypologiesDiff) {
	if diff == nil || kit == nil {
		return
	}
	items := &kit.Typologies
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, t := range *items {
			if !removedIds[t.Id] {
				filtered = append(filtered, t)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Typology.Id {
					applyTypologyDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyTypologyDiff(item *Typology, diff *TypologyDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.HasField("description") {
		item.Description = diff.Description
	}
	if diff.HasField("icon") {
		item.Icon = diff.Icon
	}
	if diff.HasField("folder") {
		item.Folder = diff.Folder
	}
	if diff.Types != nil {
		applyTypesDiff(&item.Types, diff.Types)
	}
	if diff.Designs != nil {
		applyDesignsDiff(&item.Designs, diff.Designs)
	}
}

func applyFamiliesDiff(items *[]Family, diff *FamiliesDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, f := range *items {
			if !removedIds[f.Id] {
				filtered = append(filtered, f)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Family.Id {
					applyFamilyDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyFamilyDiff(item *Family, diff *FamilyDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.HasField("description") {
		item.Description = diff.Description
	}
	if diff.HasField("icon") {
		item.Icon = diff.Icon
	}
	if diff.Ports != nil {
		applyPortsDiff(&item.Ports, diff.Ports)
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyFilesDiff(items *[]File, diff *FilesDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, f := range *items {
			if !removedIds[f.Id] {
				filtered = append(filtered, f)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.File.Id {
					applyFileDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyFileDiff(item *File, diff *FileDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.Remote != nil {
		item.Remote = diff.Remote
	}
	if diff.Folder != nil {
		item.Folder = diff.Folder
	}
	if diff.Size != nil {
		item.Size = diff.Size
	}
	if diff.Hash != nil {
		item.Hash = diff.Hash
	}
	if diff.Blob != nil {
		item.Blob = diff.Blob
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyFoldersDiff(items *[]Folder, diff *FoldersDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, f := range *items {
			if !removedIds[f.Id] {
				filtered = append(filtered, f)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Folder.Id {
					applyFolderDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyFolderDiff(item *Folder, diff *FolderDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.Parent != nil {
		item.Parent = diff.Parent
	}
	if diff.Description != nil {
		item.Description = diff.Description
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

func applyAuthorsDiff(items *[]Author, diff *AuthorsDiff) {
	if diff.Removed != nil {
		removedIds := make(map[string]bool)
		for _, r := range diff.Removed {
			removedIds[r.Id] = true
		}
		filtered := (*items)[:0]
		for _, a := range *items {
			if !removedIds[a.Id] {
				filtered = append(filtered, a)
			}
		}
		*items = filtered
	}
	if diff.Updated != nil {
		for _, u := range diff.Updated {
			for i := range *items {
				if (*items)[i].Id == u.Author.Id {
					applyAuthorDiff(&(*items)[i], &u.Diff)
					break
				}
			}
		}
	}
	if diff.Added != nil {
		*items = append(*items, diff.Added...)
	}
}

func applyAuthorDiff(item *Author, diff *AuthorDiff) {
	if diff.Name != nil {
		item.Name = *diff.Name
	}
	if diff.Email != nil {
		item.Email = diff.Email
	}
	if diff.Attributes != nil {
		applyAttributesDiff(&item.Attributes, diff.Attributes)
	}
}

// 🧹FilterDesignsWithoutParent returns all designs (Parent field removed).
func FilterDesignsWithoutParent(designs []Design) []Design {
	return designs
}

func selectBestRepresentationForFilter(representations []Representation, selectedTagIds []string) *Representation {
	if len(representations) == 0 {
		return nil
	}
	if len(selectedTagIds) == 0 {
		for i := range representations {
			if len(representations[i].Tags) == 0 {
				return &representations[i]
			}
		}
		return &representations[0]
	}

	filtered := make([]Representation, 0)
	for _, representation := range representations {
		matches := true
		for _, selectedTagId := range selectedTagIds {
			found := false
			for _, tag := range representation.Tags {
				if tag.Id == selectedTagId {
					found = true
					break
				}
			}
			if !found {
				matches = false
				break
			}
		}
		if matches {
			filtered = append(filtered, representation)
		}
	}
	if len(filtered) == 0 {
		return nil
	}

	bestIndex := 0
	bestScore := -1.0
	for i, representation := range filtered {
		tagSet := make(map[string]bool)
		selectedSet := make(map[string]bool)
		for _, tag := range representation.Tags {
			tagSet[tag.Id] = true
		}
		for _, selectedTagId := range selectedTagIds {
			selectedSet[selectedTagId] = true
		}
		intersection := 0
		union := len(tagSet)
		for id := range tagSet {
			if selectedSet[id] {
				intersection++
			}
		}
		for id := range selectedSet {
			if !tagSet[id] {
				union++
			}
		}
		score := 0.0
		if union > 0 {
			score = float64(intersection) / float64(union)
		}
		if score > bestScore {
			bestScore = score
			bestIndex = i
		}
	}
	return &filtered[bestIndex]
}

// #region 🎠Filter

// 🧩GlobFilter provides include/exclude glob patterns for name-based entity filtering.
// If Include is non-empty, only names matching at least one include pattern are kept.
// 🔍Names matching any Exclude pattern are always removed.
type GlobFilter struct {
	Include []string `json:"include,omitempty"`
	Exclude []string `json:"exclude,omitempty"`
}

// 🧹KitFilter provides general-purpose filtering combining design-based transitive filtering with glob-based name filtering.
// When DesignId is set, first performs transitive design-scoped subset extraction.
// 🏷️Glob filters on each entity kind are applied afterwards.
type KitFilter struct {
	DesignId string      `json:"designId,omitempty"`
	RepresentationTags  []string    `json:"representationTags,omitempty"`
	Designs    *GlobFilter `json:"designs,omitempty"`
	Types      *GlobFilter `json:"types,omitempty"`
	Families   *GlobFilter `json:"families,omitempty"`
	Ports      *GlobFilter `json:"ports,omitempty"`
	Files      *GlobFilter `json:"files,omitempty"`
	Tags       *GlobFilter `json:"tags,omitempty"`
	Concepts   *GlobFilter `json:"concepts,omitempty"`
	Qualities  *GlobFilter `json:"qualities,omitempty"`
	Authors    *GlobFilter `json:"authors,omitempty"`
	Folders    *GlobFilter `json:"folders,omitempty"`
}

// 🎯GlobMatch matches a name against a glob pattern supporting * and ?. Case-insensitive.
// 🔤Uses path.Match semantics but converts both name and pattern to lowercase first.
func GlobMatch(name, pattern string) bool {
	matched, err := path.Match(strings.ToLower(pattern), strings.ToLower(name))
	if err != nil {
		return false
	}
	return matched
}

// ✔️MatchesGlobFilter checks if a name passes a GlobFilter. Returns true if filter is nil or name matches.
func MatchesGlobFilter(name string, filter *GlobFilter) bool {
	if filter == nil {
		return true
	}
	if len(filter.Include) > 0 {
		matched := false
		for _, p := range filter.Include {
			if GlobMatch(name, p) {
				matched = true
				break
			}
		}
		if !matched {
			return false
		}
	}
	for _, p := range filter.Exclude {
		if GlobMatch(name, p) {
			return false
		}
	}
	return true
}

// 📦filterKitByDesign filters a kit to only include entities transitively related to a design.
// Removes types not used by pieces, designs not the target, families without used ports,
// 📄files not used by selected representations, tags/concepts only if referenced, and selects one representation per type based on tags.
func filterKitByDesign(kit Kit, designId string, tags []string) Kit {
	var design *Design
	for i := range kit.Designs {
		if kit.Designs[i].Id == designId {
			design = &kit.Designs[i]
			break
		}
	}
	if design == nil {
		return Kit{Id: kit.Id, Name: kit.Name, Version: kit.Version}
	}

	pieces := design.Pieces

	usedTypeIds := make(map[string]bool)
	usedDesignIds := make(map[string]bool)
	usedFamilyIds := make(map[string]bool)
	usedDesignIds[designId] = true
	for _, family := range design.Families {
		usedFamilyIds[family.Id] = true
	}

	for _, piece := range pieces {
		if piece.Type != nil {
			usedTypeIds[piece.Type.Id] = true
		}
		if piece.Design != nil {
			usedDesignIds[piece.Design.Id] = true
		}
	}

	typeById := make(map[string]*Type)
	for i := range kit.Types {
		typeById[kit.Types[i].Id] = &kit.Types[i]
	}
	for i := range kit.Types {
		for _, family := range kit.Types[i].Families {
			if usedFamilyIds[family.Id] {
				usedTypeIds[kit.Types[i].Id] = true
			}
		}
	}
	for i := range kit.Designs {
		for _, family := range kit.Designs[i].Families {
			if usedFamilyIds[family.Id] {
				usedDesignIds[kit.Designs[i].Id] = true
			}
		}
	}

	resolvedTagIds := make([]string, 0)
	for _, tagValue := range tags {
		for _, tag := range kit.Tags {
			if tag.Id == tagValue {
				resolvedTagIds = append(resolvedTagIds, tag.Id)
				break
			}
		}
		for _, tag := range kit.Tags {
			if tag.Name == tagValue {
				resolvedTagIds = append(resolvedTagIds, tag.Id)
			}
		}
	}

	usedPortIds := make(map[string]bool)
	usedFileIds := make(map[string]bool)
	usedTagIds := make(map[string]bool)
	usedConceptIds := make(map[string]bool)
	usedQualityIds := make(map[string]bool)
	usedAuthorIds := make(map[string]bool)
	usedFolderNames := make(map[string]bool)

	collectQualityFromProps := func(props []Prop) {
		for _, prop := range props {
			if prop.Quality.Id != "" {
				usedQualityIds[prop.Quality.Id] = true
			}
		}
	}

	selectedRepresentations := make(map[string]*Representation)
	for typeId := range usedTypeIds {
		t, ok := typeById[typeId]
		if !ok {
			continue
		}
		if t.Folder != nil && *t.Folder != "" {
			usedFolderNames[*t.Folder] = true
		}
		for _, connector := range t.Connectors {
			if connector.Port != nil {
				usedPortIds[connector.Port.Id] = true
			}
			collectQualityFromProps(connector.Props)
		}
		collectQualityFromProps(t.Props)
		for _, authorId := range t.Authors {
			usedAuthorIds[authorId.Id] = true
		}
		for _, conceptId := range t.Concepts {
			usedConceptIds[conceptId.Id] = true
		}

		if len(t.Representations) > 0 {
			best := selectBestRepresentationLike(t.Representations, resolvedTagIds)
			if best != nil {
				selectedRepresentations[typeId] = best
				usedFileIds[best.File.Id] = true
				for _, tagId := range best.Tags {
					usedTagIds[tagId.Id] = true
				}
			}
		}
	}

	for _, piece := range pieces {
		collectQualityFromProps(piece.Props)
	}
	for _, conceptId := range design.Concepts {
		usedConceptIds[conceptId.Id] = true
	}
	for _, authorId := range design.Authors {
		usedAuthorIds[authorId.Id] = true
	}

	portSnapshot := make([]string, 0)
	for portId := range usedPortIds {
		portSnapshot = append(portSnapshot, portId)
	}
	for _, portId := range portSnapshot {
		for _, family := range kit.Families {
			for _, port := range family.Ports {
				if port.Id != portId {
					continue
				}
				for _, compat := range port.CompatiblePorts {
					usedPortIds[compat.Id] = true
				}
			}
		}
	}

	for _, tagId := range resolvedTagIds {
		usedTagIds[tagId] = true
	}

	result := Kit{
		Id:        kit.Id,
		Name:        kit.Name,
		Version:     kit.Version,
		Description: kit.Description,
		Icon:        kit.Icon,
		Image:       kit.Image,
		Preview:     kit.Preview,
		Remote:      kit.Remote,
		Homepage:    kit.Homepage,
		License:     kit.License,
		Attributes:  kit.Attributes,
		CreatedAt:   kit.CreatedAt,
		UpdatedAt:   kit.UpdatedAt,
	}

	for _, t := range kit.Types {
		if !usedTypeIds[t.Id] {
			continue
		}
		filteredType := t
		if representation, ok := selectedRepresentations[t.Id]; ok {
			filteredType.Representations = []Representation{*representation}
		} else {
			filteredType.Representations = []Representation{}
		}
		result.Types = append(result.Types, filteredType)
	}

	for _, d := range kit.Designs {
		if usedDesignIds[d.Id] {
			result.Designs = append(result.Designs, d)
		}
	}
	for _, family := range kit.Families {
		filteredFamily := family
		filteredFamily.Ports = nil
		for _, port := range family.Ports {
			if usedPortIds[port.Id] {
				filteredFamily.Ports = append(filteredFamily.Ports, port)
			}
		}
		if len(filteredFamily.Ports) > 0 {
			result.Families = append(result.Families, filteredFamily)
		}
	}
	for _, f := range kit.Files {
		if usedFileIds[f.Id] {
			result.Files = append(result.Files, f)
		}
	}
	for _, t := range kit.Tags {
		if usedTagIds[t.Id] {
			result.Tags = append(result.Tags, t)
		}
	}
	for _, c := range kit.Concepts {
		if usedConceptIds[c.Id] {
			result.Concepts = append(result.Concepts, c)
		}
	}
	for _, q := range kit.Qualities {
		if usedQualityIds[q.Id] {
			result.Qualities = append(result.Qualities, q)
		}
	}
	for _, a := range kit.Authors {
		if usedAuthorIds[a.Id] {
			result.Authors = append(result.Authors, a)
		}
	}
	for _, f := range kit.Folders {
		if usedFolderNames[f.Name] {
			result.Folders = append(result.Folders, f)
		}
	}

	return result
}

// 🎨FilterKit applies general-purpose filtering to a kit. Combines optional design-based transitive filtering
// with glob-based name filtering. When DesignId is set, first performs transitive design-scoped subset extraction.
// 🧩Glob filters (include/exclude patterns on names) are applied to each entity kind afterwards.
func FilterKit(kit Kit, filter KitFilter) Kit {
	var base Kit
	if filter.DesignId != "" {
		base = filterKitByDesign(kit, filter.DesignId, filter.RepresentationTags)
	} else {
		base = kit
	}

	hasGlobFilters := filter.Designs != nil || filter.Types != nil || filter.Families != nil || filter.Ports != nil || filter.Files != nil ||
		filter.Tags != nil || filter.Concepts != nil || filter.Qualities != nil || filter.Authors != nil || filter.Folders != nil
	if !hasGlobFilters {
		return base
	}

	result := Kit{
		Id:        base.Id,
		Name:        base.Name,
		Version:     base.Version,
		Description: base.Description,
		Icon:        base.Icon,
		Image:       base.Image,
		Preview:     base.Preview,
		Remote:      base.Remote,
		Homepage:    base.Homepage,
		License:     base.License,
		Attributes:  base.Attributes,
		CreatedAt:   base.CreatedAt,
		UpdatedAt:   base.UpdatedAt,
	}

	for _, t := range base.Types {
		if MatchesGlobFilter(t.Name, filter.Types) {
			result.Types = append(result.Types, t)
		}
	}
	for _, d := range base.Designs {
		if MatchesGlobFilter(d.Name, filter.Designs) {
			result.Designs = append(result.Designs, d)
		}
	}
	for _, family := range base.Families {
		if !MatchesGlobFilter(family.Name, filter.Families) {
			continue
		}
		filteredFamily := family
		filteredFamily.Ports = nil
		for _, port := range family.Ports {
			if MatchesGlobFilter(port.Name, filter.Ports) {
				filteredFamily.Ports = append(filteredFamily.Ports, port)
			}
		}
		result.Families = append(result.Families, filteredFamily)
	}
	for _, f := range base.Files {
		if MatchesGlobFilter(f.Name, filter.Files) {
			result.Files = append(result.Files, f)
		}
	}
	for _, t := range base.Tags {
		if MatchesGlobFilter(t.Name, filter.Tags) {
			result.Tags = append(result.Tags, t)
		}
	}
	for _, c := range base.Concepts {
		if MatchesGlobFilter(c.Name, filter.Concepts) {
			result.Concepts = append(result.Concepts, c)
		}
	}
	for _, q := range base.Qualities {
		if MatchesGlobFilter(q.Name, filter.Qualities) {
			result.Qualities = append(result.Qualities, q)
		}
	}
	for _, a := range base.Authors {
		if MatchesGlobFilter(a.Name, filter.Authors) {
			result.Authors = append(result.Authors, a)
		}
	}
	for _, f := range base.Folders {
		if MatchesGlobFilter(f.Name, filter.Folders) {
			result.Folders = append(result.Folders, f)
		}
	}

	return result
}

// 🏷️selectBestRepresentationLike selects the best representation based on tag matching using Jaccard similarity.
// 🛠️Helper for filterKitByDesign.
func selectBestRepresentationLike(representations []Representation, selectedTagIds []string) *Representation {
	if len(representations) == 0 {
		return nil
	}
	if len(selectedTagIds) == 0 {
		for _, m := range representations {
			if len(m.Tags) == 0 {
				return &m
			}
		}
		return &representations[0]
	}

	var filtered []Representation
	for _, m := range representations {
		representationTagIds := make(map[string]bool)
		for _, tag := range m.Tags {
			representationTagIds[tag.Id] = true
		}
		allSelected := true
		for _, id := range selectedTagIds {
			if !representationTagIds[id] {
				allSelected = false
				break
			}
		}
		if allSelected {
			filtered = append(filtered, m)
		}
	}

	if len(filtered) == 0 {
		return nil
	}

	best := filtered[0]
	bestScore := jaccardTagIdsGo(best.Tags, selectedTagIds)
	for _, m := range filtered[1:] {
		score := jaccardTagIdsGo(m.Tags, selectedTagIds)
		if score > bestScore {
			best = m
			bestScore = score
		}
	}
	return &best
}

// 🏷️jaccardTagIdsGo computes Jaccard similarity coefficient between representation tags and selected tags.
// 🔑Helper for filterKitByDesign.
func jaccardTagIdsGo(representationTags []TagId, selectedTagIds []string) float64 {
	representationTagSet := make(map[string]bool)
	for _, tag := range representationTags {
		representationTagSet[tag.Id] = true
	}
	selectedSet := make(map[string]bool)
	for _, id := range selectedTagIds {
		selectedSet[id] = true
	}

	intersection := 0
	union := 0
	for id := range selectedSet {
		if representationTagSet[id] {
			intersection++
		}
		union++
	}
	for id := range representationTagSet {
		if !selectedSet[id] {
			union++
		}
	}

	if union == 0 {
		return 0
	}
	return float64(intersection) / float64(union)
}

// #endregion 🎠Filter

// #endregion 🎪Kit Operations

// #region 🌊Kit Change Helpers
// Kit Change Helpers MUST provide convenience functions for single-entity kit changes.

func kitTypologyDiffForTypes(topoID string, typesDiff TypesDiff) *TypologiesDiff {
	return &TypologiesDiff{
		Updated: []struct {
			Typology TypologyId   `json:"typology"`
			Diff     TypologyDiff `json:"diff"`
		}{{Typology: TypologyId{Id: topoID}, Diff: TypologyDiff{Types: &typesDiff}}},
	}
}

func kitTypologyDiffForDesigns(topoID string, designsDiff DesignsDiff) *TypologiesDiff {
	return &TypologiesDiff{
		Updated: []struct {
			Typology TypologyId   `json:"typology"`
			Diff     TypologyDiff `json:"diff"`
		}{{Typology: TypologyId{Id: topoID}, Diff: TypologyDiff{Designs: &designsDiff}}},
	}
}

func resolveTypologyIdForType(kit *Kit, typ Type) string {
	KitEnsureTypologies(kit)
	if typ.Typology.Id != "" {
		return typ.Typology.Id
	}
	if len(kit.Typologies) > 0 {
		return kit.Typologies[0].Id
	}
	return Id()
}

func resolveTypologyIdForDesign(kit *Kit, design Design) string {
	KitEnsureTypologies(kit)
	if design.Typology.Id != "" {
		return design.Typology.Id
	}
	if len(kit.Typologies) > 0 {
		return kit.Typologies[0].Id
	}
	return Id()
}

// 🆕AddTypeToKit creates a change that adds a single type to a kit.
func AddTypeToKit(kit Kit, typ Type) KitChange {
	topoID := resolveTypologyIdForType(&kit, typ)
	typ.Typology = TypologyId{Id: topoID}
	forward := KitDiff{Typologies: kitTypologyDiffForTypes(topoID, TypesDiff{Added: []Type{typ}})}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// 🚚RemoveTypeFromKit creates a change that removes a type by ID.
func RemoveTypeFromKit(kit Kit, typeId string) KitChange {
	KitEnsureTypologies(&kit)
	topoID := kit.Typologies[0].Id
	for _, t := range kit.Types {
		if t.Id == typeId {
			topoID = t.Typology.Id
			break
		}
	}
	forward := KitDiff{Typologies: kitTypologyDiffForTypes(topoID, TypesDiff{Removed: []TypeId{{Id: typeId}}})}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// ➕AddDesignToKit creates a change that adds a single design to a kit.
func AddDesignToKit(kit Kit, design Design) KitChange {
	topoID := resolveTypologyIdForDesign(&kit, design)
	design.Typology = TypologyId{Id: topoID}
	forward := KitDiff{Typologies: kitTypologyDiffForDesigns(topoID, DesignsDiff{Added: []Design{design}})}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// ➖RemoveDesignFromKit creates a change that removes a design by ID.
func RemoveDesignFromKit(kit Kit, designId string) KitChange {
	KitEnsureTypologies(&kit)
	topoID := kit.Typologies[0].Id
	for _, d := range kit.Designs {
		if d.Id == designId {
			topoID = d.Typology.Id
			break
		}
	}
	forward := KitDiff{Typologies: kitTypologyDiffForDesigns(topoID, DesignsDiff{Removed: []DesignId{{Id: designId}}})}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// 📄AddFileToKit creates a change that adds a single file to a kit.
func AddFileToKit(kit Kit, file File) KitChange {
	forward := KitDiff{
		Files: &FilesDiff{
			Added: []File{file},
		},
	}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// ❌RemoveFileFromKit creates a change that removes a file by ID.
func RemoveFileFromKit(kit Kit, fileId string) KitChange {
	forward := KitDiff{
		Files: &FilesDiff{
			Removed: []FileId{{Id: fileId}},
		},
	}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// ➕AddFamilyToKit creates a change that adds a first-class family to a kit.
func AddFamilyToKit(kit Kit, family Family) KitChange {
	forward := KitDiff{
		Families: &FamiliesDiff{
			Added: []Family{family},
		},
	}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// 👪RemoveFamilyFromKit creates a change that removes a family by ID.
func RemoveFamilyFromKit(kit Kit, familyId string) KitChange {
	forward := KitDiff{
		Families: &FamiliesDiff{
			Removed: []FamilyId{{Id: familyId}},
		},
	}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// 🏷️AddTagToKit creates a change that adds a single tag to a kit.
func AddTagToKit(kit Kit, tag Tag) KitChange {
	forward := KitDiff{
		Tags: &TagsDiff{
			Added: []Tag{tag},
		},
	}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// 🪹RemoveTagFromKit creates a change that removes a tag by ID.
func RemoveTagFromKit(kit Kit, tagId string) KitChange {
	forward := KitDiff{
		Tags: &TagsDiff{
			Removed: []TagId{{Id: tagId}},
		},
	}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// 💡AddConceptToKit creates a change that adds a single concept to a kit.
func AddConceptToKit(kit Kit, concept Concept) KitChange {
	forward := KitDiff{
		Concepts: &ConceptsDiff{
			Added: []Concept{concept},
		},
	}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// 💡RemoveConceptFromKit creates a change that removes a concept by ID.
func RemoveConceptFromKit(kit Kit, conceptId string) KitChange {
	forward := KitDiff{
		Concepts: &ConceptsDiff{
			Removed: []ConceptId{{Id: conceptId}},
		},
	}
	after := deepCloneKit(kit)
	ApplyKitDiff(&after, &forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// #endregion 🌊Kit Change Helpers

// #region 🔍Find Replaceable Types In Designs
// Find Replaceable Types In Designs MUST suggest replaceable types and designs for selected pieces.
// Specs: Looks at pieces in the selection and finds all connections attached to them.
// If connections found: looks at the types of the connected (other) pieces and finds all connectors
// within those types. Gets the ports of these connectors and finds all compatible ports.
// Suggests types that have connectors with compatible ports.
// Only suggests if a replaceable type/design exists for every connection.
// If no connections found: suggests types with same or compatible ports as the selected pieces.

// FindReplaceableTypesInDesignsForPiecesInDesign finds replaceable types and designs for the selected pieces.
func FindReplaceableTypesInDesignsForPiecesInDesign(design Design, designs []Design, types []Type, ports []Port, selectionPieces []string) (typeIds []string, designIds []string) {
	selectedPieceSet := make(map[string]bool)
	for _, pg := range selectionPieces {
		selectedPieceSet[pg] = true
	}

	pieceMap := make(map[string]Piece)
	for _, piece := range design.Pieces {
		pieceMap[piece.Id] = piece
	}

	portMap := make(map[string]Port)
	for _, p := range ports {
		portMap[p.Id] = p
	}

	typeMap := make(map[string]Type)
	for _, t := range types {
		typeMap[t.Id] = t
	}

	checkPortCompatibility := func(candidatePortId, requiredPortId string) bool {
		if candidatePortId == "" || requiredPortId == "" {
			return false
		}
		if candidatePortId == requiredPortId {
			return true
		}
		candidatePort, okCandidate := portMap[candidatePortId]
		requiredPort, okRequired := portMap[requiredPortId]
		if !okCandidate || !okRequired {
			return false
		}
		for _, compatiblePort := range candidatePort.CompatiblePorts {
			if compatiblePort.Id == requiredPortId {
				return true
			}
		}
		for _, compatiblePort := range requiredPort.CompatiblePorts {
			if compatiblePort.Id == candidatePortId {
				return true
			}
		}
		return false
	}

	getConnectorPortId := func(typeId, connectorId string) string {
		if typeId == "" || connectorId == "" {
			return ""
		}
		candidateType, ok := typeMap[typeId]
		if !ok {
			return ""
		}
		for _, connector := range candidateType.Connectors {
			if connector.Id == connectorId {
				if connector.Port != nil {
					return connector.Port.Id
				}
				return ""
			}
		}
		return ""
	}

	getOwnRequirementPortIds := func(pieceId string) []string {
		piece, ok := pieceMap[pieceId]
		if !ok || piece.Type == nil || piece.Type.Id == "" {
			return []string{}
		}
		candidateType, ok := typeMap[piece.Type.Id]
		if !ok {
			return []string{}
		}
		requirementPortIds := make([]string, 0, len(candidateType.Connectors))
		for _, connector := range candidateType.Connectors {
			if connector.Port != nil {
				requirementPortIds = append(requirementPortIds, connector.Port.Id)
			} else {
				requirementPortIds = append(requirementPortIds, "")
			}
		}
		return requirementPortIds
	}

	getBoundaryRequirementPortIds := func() []string {
		requirementPortIds := []string{}
		for _, conn := range design.Connections {
			connectedSelected := selectedPieceSet[conn.Parent.Piece.Id]
			connectingSelected := selectedPieceSet[conn.Child.Piece.Id]
			if connectedSelected == connectingSelected {
				continue
			}

			otherSide := conn.Parent
			if connectedSelected {
				otherSide = conn.Child
			}
			otherPiece, ok := pieceMap[otherSide.Piece.Id]
			if !ok || otherPiece.Type == nil {
				requirementPortIds = append(requirementPortIds, "")
				continue
			}
			connectorId := ""
			if otherSide.Connector != nil {
				connectorId = otherSide.Connector.Id
			}
			requirementPortIds = append(requirementPortIds, getConnectorPortId(otherPiece.Type.Id, connectorId))
		}
		return requirementPortIds
	}

	getSelectionOwnRequirementPortIds := func() []string {
		requirementPortIds := []string{}
		for _, pieceId := range selectionPieces {
			requirementPortIds = append(requirementPortIds, getOwnRequirementPortIds(pieceId)...)
		}
		return requirementPortIds
	}

	requiredPortIds := getBoundaryRequirementPortIds()
	if len(requiredPortIds) == 0 {
		requiredPortIds = getSelectionOwnRequirementPortIds()
	}

	canSatisfyRequirements := func(requiredPortIds []string, availablePortIds []string) bool {
		if len(requiredPortIds) == 0 {
			return true
		}
		if len(availablePortIds) < len(requiredPortIds) {
			return false
		}

		type requirementOption struct {
			connectorIndexes []int
		}

		requirementOptions := make([]requirementOption, 0, len(requiredPortIds))
		for _, requiredPortId := range requiredPortIds {
			connectorIndexes := []int{}
			for connectorIndex, availablePortId := range availablePortIds {
				if checkPortCompatibility(availablePortId, requiredPortId) {
					connectorIndexes = append(connectorIndexes, connectorIndex)
				}
			}
			if len(connectorIndexes) == 0 {
				return false
			}
			requirementOptions = append(requirementOptions, requirementOption{connectorIndexes: connectorIndexes})
		}
		sort.Slice(requirementOptions, func(i, j int) bool {
			return len(requirementOptions[i].connectorIndexes) < len(requirementOptions[j].connectorIndexes)
		})

		usedConnectorIndexes := make([]bool, len(availablePortIds))
		var matchRequirements func(int) bool
		matchRequirements = func(requirementIndex int) bool {
			if requirementIndex >= len(requirementOptions) {
				return true
			}
			for _, connectorIndex := range requirementOptions[requirementIndex].connectorIndexes {
				if usedConnectorIndexes[connectorIndex] {
					continue
				}
				usedConnectorIndexes[connectorIndex] = true
				if matchRequirements(requirementIndex + 1) {
					return true
				}
				usedConnectorIndexes[connectorIndex] = false
			}
			return false
		}
		return matchRequirements(0)
	}

	candidateTypeAvailablePortIds := func(candidateType Type) []string {
		availablePortIds := make([]string, 0, len(candidateType.Connectors))
		for _, connector := range candidateType.Connectors {
			if connector.Port != nil {
				availablePortIds = append(availablePortIds, connector.Port.Id)
			} else {
				availablePortIds = append(availablePortIds, "")
			}
		}
		return availablePortIds
	}

	candidateDesignAvailablePortIds := func(candidateDesign Design) []string {
		consumedConnectorKeys := make(map[string]bool)
		for _, conn := range candidateDesign.Connections {
			for _, side := range []Side{conn.Parent, conn.Child} {
				if side.Connector != nil && side.Connector.Id != "" {
					consumedConnectorKeys[side.Piece.Id+"::"+side.Connector.Id] = true
				}
			}
		}

		availablePortIds := []string{}
		for _, piece := range candidateDesign.Pieces {
			if piece.Type == nil || piece.Type.Id == "" {
				continue
			}
			candidateType, ok := typeMap[piece.Type.Id]
			if !ok {
				continue
			}
			for _, connector := range candidateType.Connectors {
				if consumedConnectorKeys[piece.Id+"::"+connector.Id] {
					continue
				}
				if connector.Port != nil {
					availablePortIds = append(availablePortIds, connector.Port.Id)
				} else {
					availablePortIds = append(availablePortIds, "")
				}
			}
		}
		return availablePortIds
	}

	if len(selectionPieces) == 0 {
		for _, candidateType := range types {
			if len(candidateTypeAvailablePortIds(candidateType)) == 0 {
				typeIds = append(typeIds, candidateType.Id)
			}
		}
		for _, candidateDesign := range designs {
			if len(candidateDesignAvailablePortIds(candidateDesign)) == 0 {
				designIds = append(designIds, candidateDesign.Id)
			}
		}
		return typeIds, designIds
	}

	isValidCandidate := func(availablePortIds []string) bool {
		return canSatisfyRequirements(requiredPortIds, availablePortIds)
	}

	for _, candidateType := range types {
		if isValidCandidate(candidateTypeAvailablePortIds(candidateType)) {
			typeIds = append(typeIds, candidateType.Id)
		}
	}

	for _, candidateDesign := range designs {
		if isValidCandidate(candidateDesignAvailablePortIds(candidateDesign)) {
			designIds = append(designIds, candidateDesign.Id)
		}
	}

	if typeIds == nil {
		typeIds = []string{}
	}
	if designIds == nil {
		designIds = []string{}
	}
	return typeIds, designIds
}

// #endregion 🔍Find Replaceable Types In Designs

// #region 📋Copy Paste Design
// Copy Paste Design MUST provide copy and paste functionality for designs.
// Specs: CopyDesign extracts selected pieces and connections from a design. PasteDesign inserts them into a target design.

// deepClonePiece deep-clones a Piece via JSON marshal/unmarshal.
func deepClonePiece(p Piece) Piece {
	data, _ := json.Marshal(p)
	var cloned Piece
	json.Unmarshal(data, &cloned)
	return cloned
}

// deepCloneConnection deep-clones a Connection via JSON marshal/unmarshal.
func deepCloneConnection(c Connection) Connection {
	data, _ := json.Marshal(c)
	var cloned Connection
	json.Unmarshal(data, &cloned)
	return cloned
}

// deepCloneDesign deep-clones a Design via JSON marshal/unmarshal (snapshot before in-place flatten).
func deepCloneDesign(d Design) Design {
	data, _ := json.Marshal(d)
	var cloned Design
	json.Unmarshal(data, &cloned)
	return cloned
}

// deepCloneKit deep-clones a Kit via JSON marshal/unmarshal.
func deepCloneKit(k Kit) Kit {
	data, _ := json.Marshal(k)
	var cloned Kit
	json.Unmarshal(data, &cloned)
	return cloned
}

// 📋CopyDesign extracts selected pieces and connections from a design into a new Design.
// Specs: Selected pieces are classified as internal-fixed, internal-connected, or parent-piece-exclusive parent-connection-inclusive.
// Internal pieces are copied as-is. Parent-piece-exclusive parent-connection-inclusive pieces get compose.center and compose.plane attributes.
// Non-internal connections include their external pieces marked with compose.piece.origin = "external".
func CopyDesign(kit *Kit, design Design, pieceIds []string, connectionIds []string) Design {
	selectedPieceSet := make(map[string]bool)
	for _, g := range pieceIds {
		selectedPieceSet[g] = true
	}
	selectedConnectionSet := make(map[string]bool)
	for _, g := range connectionIds {
		selectedConnectionSet[g] = true
	}

	// Build parent map: child id -> (parent id, connection)
	type parentInfo struct {
		parentId string
		connection Connection
	}
	parentMap := make(map[string]parentInfo)
	for _, conn := range design.Connections {
		parentMap[conn.Child.Piece.Id] = parentInfo{conn.Parent.Piece.Id, conn}
	}

	// Flatten the design to get absolute planes/centers
	flatDiff := FlattenDesignDiff(kit, design.Id)
	flatDesign := deepCloneDesign(design)
	ApplyDesignDiff(&flatDesign, &flatDiff)
	flatPieceMap := make(map[string]*Piece)
	for i := range flatDesign.Pieces {
		flatPieceMap[flatDesign.Pieces[i].Id] = &flatDesign.Pieces[i]
	}

	var copyPieces []Piece
	addedPieceIds := make(map[string]bool)
	var copyConnections []Connection

	// Process selected pieces
	for _, pieceId := range pieceIds {
		var piece *Piece
		for i := range design.Pieces {
			if design.Pieces[i].Id == pieceId {
				piece = &design.Pieces[i]
				break
			}
		}
		if piece == nil {
			continue
		}

		isFixed := piece.Plane != nil
		pInfo, isConnected := parentMap[pieceId]

		isInternalConnected := false
		isInternalFixed := isFixed && selectedPieceSet[pieceId]
		isPpExclPcIncl := false

		if isConnected {
			parentPieceSelected := selectedPieceSet[pInfo.parentId]
			parentConnSelected := selectedConnectionSet[pInfo.connection.Id]
			isInternalConnected = parentPieceSelected && parentConnSelected
			isPpExclPcIncl = !parentPieceSelected && parentConnSelected
		}

		if isInternalFixed || isInternalConnected {
			copyPieces = append(copyPieces, deepClonePiece(*piece))
			addedPieceIds[pieceId] = true
		} else if isPpExclPcIncl {
			copied := deepClonePiece(*piece)
			if flatPiece, ok := flatPieceMap[pieceId]; ok {
				centerValue := `{"u":0,"v":0}`
				if flatPiece.Center != nil {
					data, _ := json.Marshal(flatPiece.Center)
					centerValue = string(data)
				}
				planeValue := `{"origin":{"x":0,"y":0,"z":0},"xAxis":{"x":1,"y":0,"z":0},"yAxis":{"x":0,"y":1,"z":0}}`
				if flatPiece.Plane != nil {
					data, _ := json.Marshal(flatPiece.Plane)
					planeValue = string(data)
				}
				copied.Attributes = append(copied.Attributes,
					Attribute{Key: "compose.center", Value: &centerValue},
					Attribute{Key: "compose.plane", Value: &planeValue},
				)
			}
			copyPieces = append(copyPieces, copied)
			addedPieceIds[pieceId] = true
		}
	}

	// Process selected connections
	for _, connId := range connectionIds {
		var conn *Connection
		for i := range design.Connections {
			if design.Connections[i].Id == connId {
				conn = &design.Connections[i]
				break
			}
		}
		if conn == nil {
			continue
		}

		connectedId := conn.Parent.Piece.Id
		connectingId := conn.Child.Piece.Id
		connectedSelected := selectedPieceSet[connectedId]
		connectingSelected := selectedPieceSet[connectingId]

		isInternal := connectedSelected && connectingSelected

		if isInternal {
			copyConnections = append(copyConnections, deepCloneConnection(*conn))
		} else {
			// Orphaned, parent-excl-child-incl, or parent-incl-child-excl
			copyConnections = append(copyConnections, deepCloneConnection(*conn))

			var externalIds []string
			if !connectedSelected {
				externalIds = append(externalIds, connectedId)
			}
			if !connectingSelected {
				externalIds = append(externalIds, connectingId)
			}

			for _, extId := range externalIds {
				if !addedPieceIds[extId] {
					var extPiece *Piece
					for i := range design.Pieces {
						if design.Pieces[i].Id == extId {
							extPiece = &design.Pieces[i]
							break
						}
					}
					if extPiece != nil {
						cloned := deepClonePiece(*extPiece)
						extVal := "external"
						extAttrs := []Attribute{
							{Key: "compose.piece.origin", Value: &extVal},
						}
						if flatPiece, ok := flatPieceMap[extId]; ok {
							centerValue := `{"u":0,"v":0}`
							if flatPiece.Center != nil {
								data, _ := json.Marshal(flatPiece.Center)
								centerValue = string(data)
							}
							extAttrs = append(extAttrs, Attribute{Key: "compose.center", Value: &centerValue})
						}
						cloned.Attributes = append(cloned.Attributes, extAttrs...)
						copyPieces = append(copyPieces, cloned)
						addedPieceIds[extId] = true
					}
				}
			}
		}
	}

	return Design{
		Pieces:      copyPieces,
		Connections: copyConnections,
	}
}

// 📋PasteDesign pastes a copied design into a target design, returning a DesignDiff.
// Specs: Anchoring determines the reference point within the bounding rectangle of the source.
// Fixed pieces get -anchor offset applied to center; if coordinate is given, +coordinate offset is also applied.
// Connected pieces with non-external parents are added as-is.
// Connected pieces with external-origin parents: if a matching piece with a matching connector is found in target,
// the parent connection is remapped; otherwise treated as fixed using compose.center/compose.plane attributes.
// With coordinate, remapped stub-bridge u/v use the target matched parent’s diagram center: parent.center − (coordinate + (anchor − child.center));
// other internal clipboard connections keep deep-cloned u/v.
func PasteDesign(kit *Kit, source Design, target Design, anchoring string, coordinate *Coordinate) DesignDiff {
	typesMap := make(map[string]*Type)
	for i := range kit.Types {
		typesMap[kit.Types[i].Id] = &kit.Types[i]
	}
	portsMap := make(map[string]*Port)
	for familyIndex := range kit.Families {
		for portIndex := range kit.Families[familyIndex].Ports {
			portsMap[kit.Families[familyIndex].Ports[portIndex].Id] = &kit.Families[familyIndex].Ports[portIndex]
		}
	}

	// Classify source pieces
	externalOriginIds := make(map[string]bool)
	for _, piece := range source.Pieces {
		for _, attr := range piece.Attributes {
			if attr.Key == "compose.piece.origin" && attr.Value != nil && *attr.Value == "external" {
				externalOriginIds[piece.Id] = true
			}
		}
	}

	sourcePieceMap := make(map[string]*Piece)
	for i := range source.Pieces {
		sourcePieceMap[source.Pieces[i].Id] = &source.Pieces[i]
	}

	type parentInfo struct {
		parentId string
		connection Connection
	}
	sourceParentMap := make(map[string]parentInfo)
	for _, conn := range source.Connections {
		childId := conn.Child.Piece.Id
		parentId := conn.Parent.Piece.Id
		prev, exists := sourceParentMap[childId]
		if !exists {
			sourceParentMap[childId] = parentInfo{parentId, conn}
			continue
		}
		prevStub := externalOriginIds[prev.parentId]
		nextStub := externalOriginIds[parentId]
		if prevStub != nextStub && nextStub {
			sourceParentMap[childId] = parentInfo{parentId, conn}
		}
	}

	// Compute bounding rectangle from flat centers
	var centerCoordinates []Coordinate
	for _, piece := range source.Pieces {
		if externalOriginIds[piece.Id] {
			continue
		}
		var center *Coordinate
		if piece.Center != nil {
			center = piece.Center
		}
		if center == nil {
			for _, attr := range piece.Attributes {
				if attr.Key == "compose.center" && attr.Value != nil {
					var c Coordinate
					if err := json.Unmarshal([]byte(*attr.Value), &c); err == nil {
						center = &c
					}
				}
			}
		}
		if center != nil {
			centerCoordinates = append(centerCoordinates, *center)
		}
	}

	if len(centerCoordinates) == 0 {
		centerCoordinates = append(centerCoordinates, Coordinate{})
	}

	minU, maxU := centerCoordinates[0].U, centerCoordinates[0].U
	minV, maxV := centerCoordinates[0].V, centerCoordinates[0].V
	for _, c := range centerCoordinates[1:] {
		if c.U < minU {
			minU = c.U
		}
		if c.U > maxU {
			maxU = c.U
		}
		if c.V < minV {
			minV = c.V
		}
		if c.V > maxV {
			maxV = c.V
		}
	}

	var anchor Coordinate
	switch anchoring {
	case "middle":
		anchor = Coordinate{U: (minU + maxU) / 2, V: (minV + maxV) / 2}
	case "centroid":
		sumU, sumV := 0.0, 0.0
		for _, c := range centerCoordinates {
			sumU += c.U
			sumV += c.V
		}
		n := float64(len(centerCoordinates))
		anchor = Coordinate{U: sumU / n, V: sumV / n}
	case "bottomLeft":
		anchor = Coordinate{U: minU, V: minV}
	case "bottomRight":
		anchor = Coordinate{U: maxU, V: minV}
	case "topLeft":
		anchor = Coordinate{U: minU, V: maxV}
	case "topRight":
		anchor = Coordinate{U: maxU, V: maxV}
	default: // "original"
		anchor = Coordinate{U: 0, V: 0}
	}

	// Build target piece maps for matching
	targetPiecesByName := make(map[string][]Piece)
	for _, tp := range target.Pieces {
		if tp.Name != nil {
			targetPiecesByName[*tp.Name] = append(targetPiecesByName[*tp.Name], tp)
		}
	}

	// Helper: check port compatibility
	arePortsCompatible := func(portId1, portId2 string) bool {
		if portId1 == "" || portId2 == "" {
			return false
		}
		if portId1 == portId2 {
			return true
		}
		port1, ok1 := portsMap[portId1]
		port2, ok2 := portsMap[portId2]
		if !ok1 || !ok2 {
			return false
		}
		for _, cp := range port1.CompatiblePorts {
			if cp.Id == portId2 {
				return true
			}
		}
		for _, cp := range port2.CompatiblePorts {
			if cp.Id == portId1 {
				return true
			}
		}
		return false
	}

	// Helper: check connector compatibility
	areConnectorsCompatible := func(c1, c2 Connector) bool {
		pg1, pg2 := "", ""
		if c1.Port != nil {
			pg1 = c1.Port.Id
		}
		if c2.Port != nil {
			pg2 = c2.Port.Id
		}
		return arePortsCompatible(pg1, pg2)
	}

	// Helper: find matching connector on a type
	findMatchingConnector := func(typeId string, sourceConnector Connector) *Connector {
		t, ok := typesMap[typeId]
		if !ok {
			return nil
		}
		srcName := ""
		if sourceConnector.Name != nil {
			srcName = *sourceConnector.Name
		}
		for i := range t.Connectors {
			cName := ""
			if t.Connectors[i].Name != nil {
				cName = *t.Connectors[i].Name
			}
			if cName == srcName && areConnectorsCompatible(t.Connectors[i], sourceConnector) {
				return &t.Connectors[i]
			}
		}
		return nil
	}

	var addedPieces []Piece
	var addedConnections []Connection

	// Process source pieces
	for _, piece := range source.Pieces {
		if externalOriginIds[piece.Id] {
			continue
		}

		isFixed := piece.Plane != nil
		pInfo, isConnected := sourceParentMap[piece.Id]

		if isFixed && !isConnected {
			// Fixed piece: apply -anchor offset, then +coordinate if given
			copied := deepClonePiece(piece)
			center := Coordinate{}
			if copied.Center != nil {
				center = *copied.Center
			}
			center = Coordinate{U: center.U - anchor.U, V: center.V - anchor.V}
			if coordinate != nil {
				center = Coordinate{U: center.U + coordinate.U, V: center.V + coordinate.V}
			}
			copied.Center = &center
			addedPieces = append(addedPieces, copied)
		} else if isConnected {
			if externalOriginIds[pInfo.parentId] {
				// Parent is external-origin: try to match in target
				externalParent := sourcePieceMap[pInfo.parentId]
				matched := false

				extName := ""
				if externalParent.Name != nil {
					extName = *externalParent.Name
				}

				if candidates, ok := targetPiecesByName[extName]; ok && extName != "" {
					parentConn := pInfo.connection
					isParentConnected := parentConn.Parent.Piece.Id == pInfo.parentId
					parentConnectorId := ""
					if isParentConnected {
						if parentConn.Parent.Connector != nil {
							parentConnectorId = parentConn.Parent.Connector.Id
						}
					} else {
						if parentConn.Child.Connector != nil {
							parentConnectorId = parentConn.Child.Connector.Id
						}
					}

					// Find the source parent connector
					var sourceParentConnector *Connector
					if externalParent.Type != nil {
						if parentType, ok := typesMap[externalParent.Type.Id]; ok {
							for i := range parentType.Connectors {
								if parentType.Connectors[i].Id == parentConnectorId {
									sourceParentConnector = &parentType.Connectors[i]
									break
								}
							}
						}
					}

					if sourceParentConnector != nil {
						for _, candidate := range candidates {
							if candidate.Type == nil {
								continue
							}
							matchingConnector := findMatchingConnector(candidate.Type.Id, *sourceParentConnector)
							if matchingConnector != nil {
								matched = true
								copied := deepClonePiece(piece)
								addedPieces = append(addedPieces, copied)

								copiedConn := deepCloneConnection(parentConn)
								if isParentConnected {
									copiedConn.Parent = Side{
										Piece:     PieceId{Id: candidate.Id},
										Connector: &ConnectorId{Id: matchingConnector.Id},
									}
								} else {
									copiedConn.Child = Side{
										Piece:     PieceId{Id: candidate.Id},
										Connector: &ConnectorId{Id: matchingConnector.Id},
									}
								}
								if coordinate != nil {
									connectedStub := externalOriginIds[parentConn.Parent.Piece.Id]
									connectingStub := externalOriginIds[parentConn.Child.Piece.Id]
									connMatchesParentage := (parentConn.Child.Piece.Id == piece.Id && parentConn.Parent.Piece.Id == pInfo.parentId) ||
										(parentConn.Parent.Piece.Id == piece.Id && parentConn.Child.Piece.Id == pInfo.parentId)
									// Specs: Coordinate may shift diagram u/v only for the remapped bridge to a clipboard external stub;
									// internal–internal source edges (neither side a stub) must keep cloned u/v.
									if connMatchesParentage && connectedStub != connectingStub {
										flatParentCenter := Coordinate{}
										hasParentCenter := false
										if candidate.Center != nil {
											flatParentCenter = *candidate.Center
											hasParentCenter = true
										}
										if !hasParentCenter {
											for _, attr := range candidate.Attributes {
												if attr.Key == "compose.center" && attr.Value != nil {
													if err := json.Unmarshal([]byte(*attr.Value), &flatParentCenter); err == nil {
														hasParentCenter = true
														break
													}
												}
											}
										}
										if !hasParentCenter {
											for _, attr := range externalParent.Attributes {
												if attr.Key == "compose.center" && attr.Value != nil {
													if err := json.Unmarshal([]byte(*attr.Value), &flatParentCenter); err == nil {
														hasParentCenter = true
														break
													}
												}
											}
										}
										if !hasParentCenter && externalParent.Center != nil {
											flatParentCenter = *externalParent.Center
											hasParentCenter = true
										}
										flatChildCenter := Coordinate{}
										hasChildCenter := false
										for _, attr := range piece.Attributes {
											if attr.Key == "compose.center" && attr.Value != nil {
												if err := json.Unmarshal([]byte(*attr.Value), &flatChildCenter); err == nil {
													hasChildCenter = true
												}
											}
										}
										if !hasChildCenter && piece.Center != nil {
											flatChildCenter = *piece.Center
											hasChildCenter = true
										}
										if hasParentCenter && hasChildCenter {
											offsetU := flatParentCenter.U - (coordinate.U + (anchor.U - flatChildCenter.U))
											offsetV := flatParentCenter.V - (coordinate.V + (anchor.V - flatChildCenter.V))
											copiedConn.U = offsetU
											copiedConn.V = offsetV
										}
									}
								}
								addedConnections = append(addedConnections, copiedConn)
								break
							}
						}
					}
				}

				if !matched {
					// Treat as fixed piece using compose.center and compose.plane attributes
					copied := deepClonePiece(piece)
					for _, attr := range piece.Attributes {
						if attr.Key == "compose.center" && attr.Value != nil {
							var c Coordinate
							if err := json.Unmarshal([]byte(*attr.Value), &c); err == nil {
								copied.Center = &c
							}
						}
						if attr.Key == "compose.plane" && attr.Value != nil {
							var p Plane
							if err := json.Unmarshal([]byte(*attr.Value), &p); err == nil {
								copied.Plane = &p
							}
						}
					}
					center := Coordinate{}
					if copied.Center != nil {
						center = *copied.Center
					}
					center = Coordinate{U: center.U - anchor.U, V: center.V - anchor.V}
					if coordinate != nil {
						center = Coordinate{U: center.U + coordinate.U, V: center.V + coordinate.V}
					}
					copied.Center = &center
					addedPieces = append(addedPieces, copied)
				}
			} else {
				// Parent is not external: add connected piece as-is
				addedPieces = append(addedPieces, deepClonePiece(piece))
			}
		}
	}

	// Process source connections (non-external internal connections)
	addedPieceIds := make(map[string]bool)
	for _, p := range addedPieces {
		addedPieceIds[p.Id] = true
	}
	for _, conn := range source.Connections {
		connectedId := conn.Parent.Piece.Id
		connectingId := conn.Child.Piece.Id

		if externalOriginIds[connectedId] || externalOriginIds[connectingId] {
			continue
		}

		if !addedPieceIds[connectedId] || !addedPieceIds[connectingId] {
			continue
		}

		addedConnections = append(addedConnections, deepCloneConnection(conn))
	}

	diff := DesignDiff{}
	if len(addedPieces) > 0 {
		diff.Pieces = &PiecesDiff{Added: addedPieces}
	}
	if len(addedConnections) > 0 {
		diff.Connections = &ConnectionsDiff{Added: addedConnections}
	}
	return diff
}

// #endregion 📋Copy Paste Design

// #region 🛡️Validation

// 🏛️ComposeEntityKind enumerates the kinds of compose domain entities.
type ComposeEntityKind string

const (
	EntityKindKit        ComposeEntityKind = "Kit"
	EntityKindType       ComposeEntityKind = "Type"
	EntityKindDesign     ComposeEntityKind = "Design"
	EntityKindPiece      ComposeEntityKind = "Piece"
	EntityKindConnection ComposeEntityKind = "Connection"
	EntityKindConnector  ComposeEntityKind = "Connector"
	EntityKindAttribute  ComposeEntityKind = "Attribute"
	EntityKindFile       ComposeEntityKind = "File"
	EntityKindFolder     ComposeEntityKind = "Folder"
	EntityKindQuality    ComposeEntityKind = "Quality"
	EntityKindFamily     ComposeEntityKind = "Family"
	EntityKindPort       ComposeEntityKind = "Port"
	EntityKindProp       ComposeEntityKind = "Prop"
	EntityKindRepresentation      ComposeEntityKind = "Representation"
	EntityKindLayer      ComposeEntityKind = "Layer"
	EntityKindGroup      ComposeEntityKind = "Group"
	EntityKindStat       ComposeEntityKind = "Stat"
	EntityKindTag        ComposeEntityKind = "Tag"
	EntityKindConcept    ComposeEntityKind = "Concept"
	EntityKindAuthor     ComposeEntityKind = "Author"
)

// 📇Severity enumerates validation problem severity levels.
type Severity string

const (
	SeverityError   Severity = "error"
	SeverityWarning Severity = "warning"
)

// 📍DomainLocation identifies the entity and field where a validation problem occurs.
type DomainLocation struct {
	EntityKind ComposeEntityKind `json:"entityKind"`
	EntityId string          `json:"entityId,omitempty"`
	Field      string          `json:"field,omitempty"`
}

// 🔧Fix represents a suggested correction for a validation problem.
type Fix struct {
	Title string  `json:"title"`
	Diff  KitDiff `json:"diff"`
}

// 🔒Problem represents a single validation constraint breach.
type Problem struct {
	ConstraintId string         `json:"constraintId"`
	Severity     Severity       `json:"severity,omitempty"`
	Message      string         `json:"message"`
	Location     DomainLocation `json:"entityKind,omitempty"`
	RelatedIds []string       `json:"relatedIds,omitempty"`
	Fixes        []Fix          `json:"fixes"`
}

// ✅ValidationResult contains all problems found during kit validation.
type ValidationResult struct {
	Problems []Problem `json:"problems"`
}

// 🗃️ValidationContext provides indexed access to kit entities for constraint evaluation.
type ValidationContext struct {
	Kit           Kit
	TypesById   map[string]*Type
	DesignsById map[string]*Design
	PiecesById  map[string]struct {
		DesignId string
		Piece      *Piece
	}
	ConnectorsByTypeId map[string][]Connector
	RepresentationsByTypeId     map[string][]Representation
}

// ⚡Constraint is a function that evaluates a validation rule against a kit context.
type Constraint func(ctx *ValidationContext) []Problem

func buildValidationContext(kit Kit) *ValidationContext {
	ctx := &ValidationContext{
		Kit:           kit,
		TypesById:   make(map[string]*Type),
		DesignsById: make(map[string]*Design),
		PiecesById: make(map[string]struct {
			DesignId string
			Piece      *Piece
		}),
		ConnectorsByTypeId: make(map[string][]Connector),
		RepresentationsByTypeId:     make(map[string][]Representation),
	}
	for i := range kit.Types {
		t := &kit.Types[i]
		ctx.TypesById[t.Id] = t
		ctx.ConnectorsByTypeId[t.Id] = t.Connectors
		ctx.RepresentationsByTypeId[t.Id] = t.Representations
	}
	for i := range kit.Designs {
		d := &kit.Designs[i]
		ctx.DesignsById[d.Id] = d
		for j := range d.Pieces {
			p := &d.Pieces[j]
			ctx.PiecesById[p.Id] = struct {
				DesignId string
				Piece      *Piece
			}{DesignId: d.Id, Piece: p}
		}
	}
	return ctx
}

func generateUniqueName(baseName string, existingNames []string) string {
	nameSet := make(map[string]bool)
	for _, n := range existingNames {
		nameSet[n] = true
	}
	for i := 2; ; i++ {
		candidate := fmt.Sprintf("%s %d", baseName, i)
		if !nameSet[candidate] {
			return candidate
		}
	}
}

func makeFix(ctx *ValidationContext, title string, mutate func(clone *Kit)) Fix {
	cloneData, _ := SerializeKit(ctx.Kit)
	var clone Kit
	json.Unmarshal(cloneData, &clone)
	mutate(&clone)
	diff := GetKitDiff(ctx.Kit, clone)
	return Fix{Title: title, Diff: diff}
}

// ✔️IdUniquenessConstraint checks that all entity IDs are unique within a kit.
func IdUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	seen := make(map[string]ComposeEntityKind)
	check := func(entityKind ComposeEntityKind, entityId string) {
		if _, exists := seen[entityId]; exists {
			problem := Problem{
				ConstraintId: "id-unique",
				Severity:     SeverityError,
				Message:      fmt.Sprintf("Duplicate ID \"%s\". Entity IDs are immutable; resolve by removing or replacing the duplicate entity (first occurrence kept).", entityId),
				Location:     DomainLocation{EntityKind: entityKind, EntityId: entityId, Field: "id"},
				RelatedIds: []string{entityId},
				Fixes: []Fix{
					makeFix(ctx, "Regenerate ID", func(clone *Kit) {
						newId := Id()
						updateIdEverywhere(clone, entityId, newId)
					}),
				},
			}
			problems = append(problems, problem)
		} else {
			seen[entityId] = entityKind
		}
	}
	check(EntityKindKit, ctx.Kit.Id)
	for _, t := range ctx.Kit.Types {
		check(EntityKindType, t.Id)
	}
	for _, d := range ctx.Kit.Designs {
		check(EntityKindDesign, d.Id)
		for _, p := range d.Pieces {
			check(EntityKindPiece, p.Id)
		}
		for _, c := range d.Connections {
			check(EntityKindConnection, c.Id)
		}
		for _, s := range d.Stats {
			check(EntityKindStat, s.Id)
		}
	}
	for _, q := range ctx.Kit.Qualities {
		check(EntityKindQuality, q.Id)
	}
	for _, family := range ctx.Kit.Families {
		check(EntityKindFamily, family.Id)
		for _, port := range family.Ports {
			check(EntityKindPort, port.Id)
		}
	}
	for _, f := range ctx.Kit.Files {
		check(EntityKindFile, f.Id)
	}
	for _, f := range ctx.Kit.Folders {
		check(EntityKindFolder, f.Id)
	}
	return problems
}

func updateIdEverywhere(kit *Kit, oldId, newId string) {
	if kit.Id == oldId {
		kit.Id = newId
	}
	for i := range kit.Types {
		t := &kit.Types[i]
		if t.Id == oldId {
			t.Id = newId
		}
		for j := range t.Families {
			if t.Families[j].Id == oldId {
				t.Families[j].Id = newId
			}
		}
		for j := range t.Connectors {
			if t.Connectors[j].Id == oldId {
				t.Connectors[j].Id = newId
			}
		}
		for j := range t.Representations {
			if t.Representations[j].Id == oldId {
				t.Representations[j].Id = newId
			}
		}
	}
	for i := range kit.Designs {
		d := &kit.Designs[i]
		if d.Id == oldId {
			d.Id = newId
		}
		for j := range d.Families {
			if d.Families[j].Id == oldId {
				d.Families[j].Id = newId
			}
		}
		for j := range d.Pieces {
			p := &d.Pieces[j]
			if p.Id == oldId {
				p.Id = newId
			}
			if p.Type != nil && p.Type.Id == oldId {
				p.Type.Id = newId
			}
			if p.Design != nil && p.Design.Id == oldId {
				p.Design.Id = newId
			}
		}
		for j := range d.Connections {
			c := &d.Connections[j]
			if c.Id == oldId {
				c.Id = newId
			}
			if c.Parent.Piece.Id == oldId {
				c.Parent.Piece.Id = newId
			}
			if c.Child.Piece.Id == oldId {
				c.Child.Piece.Id = newId
			}
			if c.Parent.Connector != nil && c.Parent.Connector.Id == oldId {
				c.Parent.Connector.Id = newId
			}
			if c.Child.Connector != nil && c.Child.Connector.Id == oldId {
				c.Child.Connector.Id = newId
			}
		}
	}
	for i := range kit.Families {
		if kit.Families[i].Id == oldId {
			kit.Families[i].Id = newId
		}
		for j := range kit.Families[i].Ports {
			if kit.Families[i].Ports[j].Id == oldId {
				kit.Families[i].Ports[j].Id = newId
			}
			for k := range kit.Families[i].Ports[j].CompatiblePorts {
				if kit.Families[i].Ports[j].CompatiblePorts[k].Id == oldId {
					kit.Families[i].Ports[j].CompatiblePorts[k].Id = newId
				}
			}
		}
	}
	for i := range kit.Qualities {
		if kit.Qualities[i].Id == oldId {
			kit.Qualities[i].Id = newId
		}
	}
	for i := range kit.Files {
		if kit.Files[i].Id == oldId {
			kit.Files[i].Id = newId
		}
	}
	for i := range kit.Folders {
		if kit.Folders[i].Id == oldId {
			kit.Folders[i].Id = newId
		}
		if kit.Folders[i].Parent != nil && kit.Folders[i].Parent.Id == oldId {
			kit.Folders[i].Parent.Id = newId
		}
	}
}

// 🧱TypeNameUniquenessConstraint checks that sibling type names are unique within a kit.
func TypeNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	byParent := make(map[string][]Type)
	for _, t := range ctx.Kit.Types {
		byParent[""] = append(byParent[""], t)
	}
	for _, siblings := range byParent {
		names := make(map[string][]Type)
		for _, t := range siblings {
			name := t.Name
			names[name] = append(names[name], t)
		}
		for name, group := range names {
			if len(group) <= 1 {
				continue
			}
			siblingNames := make([]string, len(siblings))
			for i, s := range siblings {
				siblingNames[i] = s.Name
			}
			for i := 1; i < len(group); i++ {
				typ := group[i]
				relatedIds := make([]string, len(group))
				for j, g := range group {
					relatedIds[j] = g.Id
				}
				problem := Problem{
					ConstraintId: "type-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate type name \"%s\" among siblings.", name),
					Location:     DomainLocation{EntityKind: EntityKindType, EntityId: typ.Id, Field: "name"},
					RelatedIds: relatedIds,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename \"%s\"", name), func(clone *Kit) {
							for j := range clone.Types {
								if clone.Types[j].Id == typ.Id {
									clone.Types[j].Name = generateUniqueName(name, siblingNames)
									break
								}
							}
						}),
					},
				}
				problems = append(problems, problem)
			}
		}
	}
	return problems
}

// 📐DesignNameUniquenessConstraint checks that sibling design names are unique within a kit.
func DesignNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	byParent := make(map[string][]Design)
	for _, d := range ctx.Kit.Designs {
		byParent[""] = append(byParent[""], d)
	}
	for _, siblings := range byParent {
		names := make(map[string][]Design)
		for _, d := range siblings {
			name := d.Name
			names[name] = append(names[name], d)
		}
		for name, group := range names {
			if len(group) <= 1 {
				continue
			}
			siblingNames := make([]string, len(siblings))
			for i, s := range siblings {
				siblingNames[i] = s.Name
			}
			for i := 1; i < len(group); i++ {
				design := group[i]
				relatedIds := make([]string, len(group))
				for j, g := range group {
					relatedIds[j] = g.Id
				}
				problem := Problem{
					ConstraintId: "design-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate design name \"%s\" among siblings.", name),
					Location:     DomainLocation{EntityKind: EntityKindDesign, EntityId: design.Id, Field: "name"},
					RelatedIds: relatedIds,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename \"%s\"", name), func(clone *Kit) {
							for j := range clone.Designs {
								if clone.Designs[j].Id == design.Id {
									clone.Designs[j].Name = generateUniqueName(name, siblingNames)
									break
								}
							}
						}),
					},
				}
				problems = append(problems, problem)
			}
		}
	}
	return problems
}

// 🧩PieceNameUniquenessConstraint checks that piece names are unique within each design.
func PieceNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	for _, design := range ctx.Kit.Designs {
		if len(design.Pieces) == 0 {
			continue
		}
		names := make(map[string][]Piece)
		for _, p := range design.Pieces {
			name := ""
			if p.Name != nil {
				name = *p.Name
			}
			names[name] = append(names[name], p)
		}
		allNames := make([]string, len(design.Pieces))
		for i, p := range design.Pieces {
			if p.Name != nil {
				allNames[i] = *p.Name
			}
		}
		for name, group := range names {
			if len(group) <= 1 {
				continue
			}
			for i := 1; i < len(group); i++ {
				piece := group[i]
				relatedIds := make([]string, len(group))
				for j, g := range group {
					relatedIds[j] = g.Id
				}
				designId := design.Id
				problem := Problem{
					ConstraintId: "piece-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate piece name \"%s\" inside design \"%s\".", name, design.Name),
					Location:     DomainLocation{EntityKind: EntityKindPiece, EntityId: piece.Id, Field: "name"},
					RelatedIds: relatedIds,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename piece \"%s\"", name), func(clone *Kit) {
							for j := range clone.Designs {
								if clone.Designs[j].Id == designId {
									for k := range clone.Designs[j].Pieces {
										if clone.Designs[j].Pieces[k].Id == piece.Id {
											newName := generateUniqueName(name, allNames)
											clone.Designs[j].Pieces[k].Name = &newName
											break
										}
									}
									break
								}
							}
						}),
					},
				}
				problems = append(problems, problem)
			}
		}
	}
	return problems
}

// 🔬QualityNameUniquenessConstraint checks that quality names are unique within a kit.
func QualityNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	names := make(map[string][]Quality)
	for _, q := range ctx.Kit.Qualities {
		name := q.Name
		names[name] = append(names[name], q)
	}
	allNames := make([]string, len(ctx.Kit.Qualities))
	for i, q := range ctx.Kit.Qualities {
		allNames[i] = q.Name
	}
	for name, group := range names {
		if len(group) <= 1 {
			continue
		}
		for i := 1; i < len(group); i++ {
			quality := group[i]
			relatedIds := make([]string, len(group))
			for j, g := range group {
				relatedIds[j] = g.Id
			}
			problem := Problem{
				ConstraintId: "quality-name-unique",
				Severity:     SeverityError,
				Message:      fmt.Sprintf("Duplicate quality name \"%s\".", name),
				Location:     DomainLocation{EntityKind: EntityKindQuality, EntityId: quality.Id, Field: "name"},
				RelatedIds: relatedIds,
				Fixes: []Fix{
					makeFix(ctx, fmt.Sprintf("Rename quality \"%s\"", name), func(clone *Kit) {
						for j := range clone.Qualities {
							if clone.Qualities[j].Id == quality.Id {
								clone.Qualities[j].Name = generateUniqueName(name, allNames)
								break
							}
						}
					}),
				},
			}
			problems = append(problems, problem)
		}
	}
	return problems
}

// ⚓PortNameUniquenessConstraint checks that port names are unique within a kit.
func PortNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	names := make(map[string][]Port)
	allPorts := AllPortsInKit(&ctx.Kit)
	for _, p := range allPorts {
		name := p.Name
		names[name] = append(names[name], p)
	}
	allNames := make([]string, len(allPorts))
	for i, p := range allPorts {
		allNames[i] = p.Name
	}
	for name, group := range names {
		if len(group) <= 1 {
			continue
		}
		for i := 1; i < len(group); i++ {
			iface := group[i]
			relatedIds := make([]string, len(group))
			for j, g := range group {
				relatedIds[j] = g.Id
			}
			problem := Problem{
				ConstraintId: "port-name-unique",
				Severity:     SeverityError,
				Message:      fmt.Sprintf("Duplicate port name \"%s\".", name),
				Location:     DomainLocation{EntityKind: EntityKindPort, EntityId: iface.Id, Field: "name"},
				RelatedIds: relatedIds,
				Fixes: []Fix{
					makeFix(ctx, fmt.Sprintf("Rename port \"%s\"", name), func(clone *Kit) {
						for familyIndex := range clone.Families {
							for portIndex := range clone.Families[familyIndex].Ports {
								if clone.Families[familyIndex].Ports[portIndex].Id == iface.Id {
									clone.Families[familyIndex].Ports[portIndex].Name = generateUniqueName(name, allNames)
									return
								}
							}
						}
					}),
				},
			}
			problems = append(problems, problem)
		}
	}
	return problems
}

// 📄FileNameUniquenessConstraint checks that file names are unique within a kit.
func FileNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	names := make(map[string][]File)
	for _, f := range ctx.Kit.Files {
		name := f.Name
		names[name] = append(names[name], f)
	}
	allNames := make([]string, len(ctx.Kit.Files))
	for i, f := range ctx.Kit.Files {
		allNames[i] = f.Name
	}
	for name, group := range names {
		if len(group) <= 1 {
			continue
		}
		for i := 1; i < len(group); i++ {
			file := group[i]
			relatedIds := make([]string, len(group))
			for j, g := range group {
				relatedIds[j] = g.Id
			}
			problem := Problem{
				ConstraintId: "file-name-unique",
				Severity:     SeverityError,
				Message:      fmt.Sprintf("Duplicate file name \"%s\".", name),
				Location:     DomainLocation{EntityKind: EntityKindFile, EntityId: file.Id, Field: "name"},
				RelatedIds: relatedIds,
				Fixes: []Fix{
					makeFix(ctx, fmt.Sprintf("Rename file \"%s\"", name), func(clone *Kit) {
						for j := range clone.Files {
							if clone.Files[j].Id == file.Id {
								clone.Files[j].Name = generateUniqueName(name, allNames)
								break
							}
						}
					}),
				},
			}
			problems = append(problems, problem)
		}
	}
	return problems
}

// 📁FolderNameUniquenessConstraint checks that sibling folder names are unique.
func FolderNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	byParent := make(map[string][]Folder)
	for _, f := range ctx.Kit.Folders {
		parentId := ""
		if f.Parent != nil {
			parentId = f.Parent.Id
		}
		byParent[parentId] = append(byParent[parentId], f)
	}
	for _, siblings := range byParent {
		names := make(map[string][]Folder)
		for _, f := range siblings {
			name := f.Name
			names[name] = append(names[name], f)
		}
		siblingNames := make([]string, len(siblings))
		for i, s := range siblings {
			siblingNames[i] = s.Name
		}
		for name, group := range names {
			if len(group) <= 1 {
				continue
			}
			for i := 1; i < len(group); i++ {
				folder := group[i]
				relatedIds := make([]string, len(group))
				for j, g := range group {
					relatedIds[j] = g.Id
				}
				problem := Problem{
					ConstraintId: "folder-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate folder name \"%s\" among siblings.", name),
					Location:     DomainLocation{EntityKind: EntityKindFolder, EntityId: folder.Id, Field: "name"},
					RelatedIds: relatedIds,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename folder \"%s\"", name), func(clone *Kit) {
							for j := range clone.Folders {
								if clone.Folders[j].Id == folder.Id {
									clone.Folders[j].Name = generateUniqueName(name, siblingNames)
									break
								}
							}
						}),
					},
				}
				problems = append(problems, problem)
			}
		}
	}
	return problems
}

// 🔌ConnectorNameUniquenessConstraint checks that connector names are unique within each type.
func ConnectorNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	for typeId, connectors := range ctx.ConnectorsByTypeId {
		if len(connectors) == 0 {
			continue
		}
		names := make(map[string][]Connector)
		for _, c := range connectors {
			name := ""
			if c.Name != nil {
				name = *c.Name
			}
			names[name] = append(names[name], c)
		}
		allNames := make([]string, len(connectors))
		for i, c := range connectors {
			if c.Name != nil {
				allNames[i] = *c.Name
			}
		}
		typ := ctx.TypesById[typeId]
		typeName := ""
		if typ != nil {
			typeName = typ.Name
		}
		for name, group := range names {
			if len(group) <= 1 {
				continue
			}
			for i := 1; i < len(group); i++ {
				connector := group[i]
				relatedIds := make([]string, len(group))
				for j, g := range group {
					relatedIds[j] = g.Id
				}
				tId := typeId
				problem := Problem{
					ConstraintId: "connector-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate connector name \"%s\" inside type \"%s\".", name, typeName),
					Location:     DomainLocation{EntityKind: EntityKindConnector, EntityId: connector.Id, Field: "name"},
					RelatedIds: relatedIds,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename connector \"%s\"", name), func(clone *Kit) {
							for j := range clone.Types {
								if clone.Types[j].Id == tId {
									for k := range clone.Types[j].Connectors {
										if clone.Types[j].Connectors[k].Id == connector.Id {
											clone.Types[j].Connectors[k].Name = ptrString(generateUniqueName(name, allNames))
											break
										}
									}
									break
								}
							}
						}),
					},
				}
				problems = append(problems, problem)
			}
		}
	}
	return problems
}

// 🗿RepresentationNameUniquenessConstraint checks that representation names are unique within each type.
func RepresentationNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	for typeId, representations := range ctx.RepresentationsByTypeId {
		if len(representations) == 0 {
			continue
		}
		names := make(map[string][]Representation)
		for _, m := range representations {
			name := ""
			if m.Name != nil {
				name = *m.Name
			}
			names[name] = append(names[name], m)
		}
		allNames := make([]string, len(representations))
		for i, m := range representations {
			if m.Name != nil {
				allNames[i] = *m.Name
			}
		}
		typ := ctx.TypesById[typeId]
		typeName := ""
		if typ != nil {
			typeName = typ.Name
		}
		for name, group := range names {
			if len(group) <= 1 {
				continue
			}
			for i := 1; i < len(group); i++ {
				representation := group[i]
				relatedIds := make([]string, len(group))
				for j, g := range group {
					relatedIds[j] = g.Id
				}
				tId := typeId
				problem := Problem{
					ConstraintId: "representation-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate representation name \"%s\" inside type \"%s\".", name, typeName),
					Location:     DomainLocation{EntityKind: EntityKindRepresentation, EntityId: representation.Id, Field: "name"},
					RelatedIds: relatedIds,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename representation \"%s\"", name), func(clone *Kit) {
							for j := range clone.Types {
								if clone.Types[j].Id == tId {
									for k := range clone.Types[j].Representations {
										if clone.Types[j].Representations[k].Id == representation.Id {
											newName := generateUniqueName(name, allNames)
											clone.Types[j].Representations[k].Name = &newName
											break
										}
									}
									break
								}
							}
						}),
					},
				}
				problems = append(problems, problem)
			}
		}
	}
	return problems
}

// 🛤️LayerPathUniquenessConstraint checks that layer paths are unique within each design.
func LayerPathUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	for _, design := range ctx.Kit.Designs {
		if len(design.Layers) == 0 {
			continue
		}
		paths := make(map[string][]Layer)
		for _, l := range design.Layers {
			path := l.Path
			paths[path] = append(paths[path], l)
		}
		allPaths := make([]string, len(design.Layers))
		for i, l := range design.Layers {
			allPaths[i] = l.Path
		}
		for path, group := range paths {
			if len(group) <= 1 {
				continue
			}
			for i := 1; i < len(group); i++ {
				layer := group[i]
				designId := design.Id
				problem := Problem{
					ConstraintId: "layer-path-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate layer path \"%s\" inside design \"%s\".", path, design.Name),
					Location:     DomainLocation{EntityKind: EntityKindLayer, EntityId: layer.Id, Field: "path"},
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename layer \"%s\"", path), func(clone *Kit) {
							for j := range clone.Designs {
								if clone.Designs[j].Id == designId {
									for k := range clone.Designs[j].Layers {
										if clone.Designs[j].Layers[k].Id == layer.Id {
											clone.Designs[j].Layers[k].Path = generateUniqueName(path, allPaths)
											break
										}
									}
									break
								}
							}
						}),
					},
				}
				problems = append(problems, problem)
			}
		}
	}
	return problems
}

// 📜DefaultConstraints lists all built-in validation constraints.
var DefaultConstraints = []Constraint{
	IdUniquenessConstraint,
	TypeNameUniquenessConstraint,
	DesignNameUniquenessConstraint,
	PieceNameUniquenessConstraint,
	QualityNameUniquenessConstraint,
	PortNameUniquenessConstraint,
	FileNameUniquenessConstraint,
	FolderNameUniquenessConstraint,
	ConnectorNameUniquenessConstraint,
	RepresentationNameUniquenessConstraint,
	LayerPathUniquenessConstraint,
}

// 🗃️ValidateKit validates a kit using the default set of constraints.
func ValidateKit(kit Kit) ValidationResult {
	return ValidateKitWithConstraints(kit, DefaultConstraints)
}

// 🔍ValidateKitWithConstraints validates a kit using the provided set of constraints.
func ValidateKitWithConstraints(kit Kit, constraints []Constraint) ValidationResult {
	ctx := buildValidationContext(kit)
	var problems []Problem
	for _, constraint := range constraints {
		problems = append(problems, constraint(ctx)...)
	}
	return ValidationResult{Problems: problems}
}

// ❌HasErrors returns true if the validation result contains any error-severity problems.
func HasErrors(result ValidationResult) bool {
	for _, p := range result.Problems {
		if p.Severity == SeverityError || p.Severity == "" {
			return true
		}
	}
	return false
}

// #region 🌧️Validation Serialization
// Validation Serialization MUST provide serializable representations of validation results.

// ⚠️ProblemSerialized is the JSON-serializable representation of a validation problem.
type ProblemSerialized struct {
	ConstraintId string `json:"constraintId"`
	Severity     string `json:"severity,omitempty"`
	Message      string `json:"message"`
	EntityKind   string `json:"entityKind"`
	EntityId   string `json:"entityId"`
	Fixes        []Fix  `json:"fixes"`
}

// ✅ValidationResultSerialized is the JSON-serializable representation of a validation result.
type ValidationResultSerialized struct {
	Problems []ProblemSerialized `json:"problems"`
}

// ✅ToValidationResult converts a validation result to its JSON-serializable form.
func ToValidationResult(result ValidationResult) ValidationResultSerialized {
	problems := make([]ProblemSerialized, len(result.Problems))
	for i, p := range result.Problems {
		severity := string(p.Severity)
		if severity == "" {
			severity = "error"
		}
		problems[i] = ProblemSerialized{
			ConstraintId: p.ConstraintId,
			Severity:     severity,
			Message:      p.Message,
			EntityKind:   string(p.Location.EntityKind),
			EntityId:   p.Location.EntityId,
			Fixes:        p.Fixes,
		}
	}
	return ValidationResultSerialized{Problems: problems}
}

// ⚖️AreValidationResultsEqual compares two serialized validation results for structural equality.
func AreValidationResultsEqual(a, b ValidationResultSerialized) bool {
	if len(a.Problems) != len(b.Problems) {
		return false
	}
	sortProblems := func(problems []ProblemSerialized) {
		sort.Slice(problems, func(i, j int) bool {
			if problems[i].ConstraintId != problems[j].ConstraintId {
				return problems[i].ConstraintId < problems[j].ConstraintId
			}
			return problems[i].EntityId < problems[j].EntityId
		})
	}
	sortedA := make([]ProblemSerialized, len(a.Problems))
	copy(sortedA, a.Problems)
	sortProblems(sortedA)
	sortedB := make([]ProblemSerialized, len(b.Problems))
	copy(sortedB, b.Problems)
	sortProblems(sortedB)
	for i := range sortedA {
		if sortedA[i].ConstraintId != sortedB[i].ConstraintId ||
			sortedA[i].Message != sortedB[i].Message ||
			sortedA[i].EntityKind != sortedB[i].EntityKind ||
			sortedA[i].EntityId != sortedB[i].EntityId {
			return false
		}
	}
	return true
}

// #endregion 🌧️Validation Serialization

// #endregion 🛡️Validation

// #region 🌤️Flatten Design
// 💾Flatten Design MUST compute absolute piece planes from relative connections.
func planeToMatrix(p Plane) *mat.Dense {
	xAxis := []float64{p.XAxis.X, p.XAxis.Y, p.XAxis.Z}
	yAxis := []float64{p.YAxis.X, p.YAxis.Y, p.YAxis.Z}
	zAxis := cross(xAxis, yAxis)
	normalize(zAxis)
	m := mat.NewDense(4, 4, []float64{
		xAxis[0], yAxis[0], zAxis[0], p.Origin.X,
		xAxis[1], yAxis[1], zAxis[1], p.Origin.Y,
		xAxis[2], yAxis[2], zAxis[2], p.Origin.Z,
		0, 0, 0, 1,
	})
	return m
}

func matrixToPlane(m *mat.Dense) Plane {
	return Plane{
		Origin: Point{X: m.At(0, 3), Y: m.At(1, 3), Z: m.At(2, 3)},
		XAxis:  Vector{X: m.At(0, 0), Y: m.At(1, 0), Z: m.At(2, 0)},
		YAxis:  Vector{X: m.At(0, 1), Y: m.At(1, 1), Z: m.At(2, 1)},
	}
}

func cross(a, b []float64) []float64 {
	return []float64{
		a[1]*b[2] - a[2]*b[1],
		a[2]*b[0] - a[0]*b[2],
		a[0]*b[1] - a[1]*b[0],
	}
}

func normalize(v []float64) {
	length := math.Sqrt(v[0]*v[0] + v[1]*v[1] + v[2]*v[2])
	if length > 0 {
		v[0] /= length
		v[1] /= length
		v[2] /= length
	}
}

func dot(a, b []float64) float64 {
	return a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}

func vecLength(v []float64) float64 {
	return math.Sqrt(v[0]*v[0] + v[1]*v[1] + v[2]*v[2])
}

func degToRad(deg float64) float64 {
	return deg * math.Pi / 180.0
}

func roundFloat(val float64, precision int) float64 {
	ratio := math.Pow(10, float64(precision))
	return math.Round(val*ratio) / ratio
}

func roundPlane(p Plane) Plane {
	const prec = 6
	return Plane{
		Origin: Point{X: roundFloat(p.Origin.X, prec), Y: roundFloat(p.Origin.Y, prec), Z: roundFloat(p.Origin.Z, prec)},
		XAxis:  Vector{X: roundFloat(p.XAxis.X, prec), Y: roundFloat(p.XAxis.Y, prec), Z: roundFloat(p.XAxis.Z, prec)},
		YAxis:  Vector{X: roundFloat(p.YAxis.X, prec), Y: roundFloat(p.YAxis.Y, prec), Z: roundFloat(p.YAxis.Z, prec)},
	}
}

func makeRotationAxis(axis []float64, angle float64) *mat.Dense {
	c := math.Cos(angle)
	s := math.Sin(angle)
	t := 1 - c
	x, y, z := axis[0], axis[1], axis[2]
	return mat.NewDense(4, 4, []float64{
		t*x*x + c, t*x*y - s*z, t*x*z + s*y, 0,
		t*x*y + s*z, t*y*y + c, t*y*z - s*x, 0,
		t*x*z - s*y, t*y*z + s*x, t*z*z + c, 0,
		0, 0, 0, 1,
	})
}

func makeTranslation(x, y, z float64) *mat.Dense {
	return mat.NewDense(4, 4, []float64{
		1, 0, 0, x,
		0, 1, 0, y,
		0, 0, 1, z,
		0, 0, 0, 1,
	})
}

func quaternionFromAxisAngle(axis []float64, angle float64) []float64 {
	halfAngle := angle / 2
	s := math.Sin(halfAngle)
	return []float64{axis[0] * s, axis[1] * s, axis[2] * s, math.Cos(halfAngle)}
}

func quaternionFromUnitVectors(vFrom, vTo []float64) []float64 {
	r := dot(vFrom, vTo) + 1
	var quat []float64
	if r < 0.000001 {
		if math.Abs(vFrom[0]) > math.Abs(vFrom[2]) {
			quat = []float64{-vFrom[1], vFrom[0], 0, 0}
		} else {
			quat = []float64{0, -vFrom[2], vFrom[1], 0}
		}
	} else {
		crossV := cross(vFrom, vTo)
		quat = []float64{crossV[0], crossV[1], crossV[2], r}
	}
	length := math.Sqrt(quat[0]*quat[0] + quat[1]*quat[1] + quat[2]*quat[2] + quat[3]*quat[3])
	return []float64{quat[0] / length, quat[1] / length, quat[2] / length, quat[3] / length}
}

func quaternionToMatrix(q []float64) *mat.Dense {
	x, y, z, w := q[0], q[1], q[2], q[3]
	x2, y2, z2 := x+x, y+y, z+z
	xx, xy, xz := x*x2, x*y2, x*z2
	yy, yz, zz := y*y2, y*z2, z*z2
	wx, wy, wz := w*x2, w*y2, w*z2
	return mat.NewDense(4, 4, []float64{
		1 - (yy + zz), xy - wz, xz + wy, 0,
		xy + wz, 1 - (xx + zz), yz - wx, 0,
		xz - wy, yz + wx, 1 - (xx + yy), 0,
		0, 0, 0, 1,
	})
}

func multiplyMatrices(a, b *mat.Dense) *mat.Dense {
	result := mat.NewDense(4, 4, nil)
	result.Mul(a, b)
	return result
}

func applyMatrix4ToVec3(m *mat.Dense, v []float64) []float64 {
	return []float64{
		m.At(0, 0)*v[0] + m.At(0, 1)*v[1] + m.At(0, 2)*v[2],
		m.At(1, 0)*v[0] + m.At(1, 1)*v[1] + m.At(1, 2)*v[2],
		m.At(2, 0)*v[0] + m.At(2, 1)*v[1] + m.At(2, 2)*v[2],
	}
}

func computeChildPlane(parentPlane Plane, parentConnector, childConnector Connector, connection Connection) Plane {
	parentMatrix := planeToMatrix(parentPlane)
	parentPoint := []float64{parentConnector.Point.X, parentConnector.Point.Y, parentConnector.Point.Z}
	parentDirection := []float64{parentConnector.Direction.X, parentConnector.Direction.Y, parentConnector.Direction.Z}
	normalize(parentDirection)
	childPoint := []float64{childConnector.Point.X, childConnector.Point.Y, childConnector.Point.Z}
	childDirection := []float64{childConnector.Direction.X, childConnector.Direction.Y, childConnector.Direction.Z}
	normalize(childDirection)

	gap := connection.Gap
	shift := connection.Shift
	rise := connection.Rise
	rotationRad := degToRad(connection.Rotation)
	turnRad := degToRad(connection.Turn)
	tiltRad := degToRad(connection.Tilt)

	reverseChildDirection := []float64{-childDirection[0], -childDirection[1], -childDirection[2]}

	var alignQuat []float64
	crossVec := cross(parentDirection, reverseChildDirection)
	crossLen := vecLength(crossVec)
	if crossLen < 0.01 {
		if math.Abs(parentDirection[2]) < Tolerance {
			alignQuat = quaternionFromAxisAngle([]float64{0, 0, 1}, math.Pi)
		} else {
			axis := cross([]float64{0, 0, 1}, parentDirection)
			normalize(axis)
			alignQuat = quaternionFromAxisAngle(axis, math.Pi)
		}
	} else {
		alignQuat = quaternionFromUnitVectors(reverseChildDirection, parentDirection)
	}

	directionT := quaternionToMatrix(alignQuat)

	yAxis := []float64{0, 1, 0}
	parentConnectorQuat := quaternionFromUnitVectors(yAxis, parentDirection)
	parentRotationT := quaternionToMatrix(parentConnectorQuat)

	gapDirection := applyMatrix4ToVec3(parentRotationT, []float64{0, 1, 0})
	shiftDirection := applyMatrix4ToVec3(parentRotationT, []float64{1, 0, 0})
	raiseDirection := applyMatrix4ToVec3(parentRotationT, []float64{0, 0, 1})
	turnAxis := applyMatrix4ToVec3(parentRotationT, []float64{0, 0, 1})
	tiltAxis := applyMatrix4ToVec3(parentRotationT, []float64{1, 0, 0})

	orientationT := directionT

	rotateT := makeRotationAxis(parentDirection, -rotationRad)
	orientationT = multiplyMatrices(rotateT, orientationT)

	turnAxis = applyMatrix4ToVec3(rotateT, turnAxis)
	tiltAxis = applyMatrix4ToVec3(rotateT, tiltAxis)

	turnT := makeRotationAxis(turnAxis, turnRad)
	orientationT = multiplyMatrices(turnT, orientationT)

	tiltT := makeRotationAxis(tiltAxis, tiltRad)
	orientationT = multiplyMatrices(tiltT, orientationT)

	centerChildT := makeTranslation(-childPoint[0], -childPoint[1], -childPoint[2])
	transform := multiplyMatrices(orientationT, centerChildT)

	gapTransform := makeTranslation(gapDirection[0]*gap, gapDirection[1]*gap, gapDirection[2]*gap)
	shiftTransform := makeTranslation(shiftDirection[0]*shift, shiftDirection[1]*shift, shiftDirection[2]*shift)
	raiseTransform := makeTranslation(raiseDirection[0]*rise, raiseDirection[1]*rise, raiseDirection[2]*rise)

	translationT := multiplyMatrices(raiseTransform, multiplyMatrices(shiftTransform, gapTransform))
	transform = multiplyMatrices(translationT, transform)
	moveToParentT := makeTranslation(parentPoint[0], parentPoint[1], parentPoint[2])
	transform = multiplyMatrices(moveToParentT, transform)
	finalMatrix := multiplyMatrices(parentMatrix, transform)

	return matrixToPlane(finalMatrix)
}

type pieceNode struct {
	piece *Piece
	plane *Plane
}

func getConnector(typesDict map[string]*Type, typ *Type, connectorId *string) *Connector {
	if typ == nil {
		return nil
	}
	if connectorId == nil || *connectorId == "" {
		if len(typ.Connectors) > 0 {
			return &typ.Connectors[0]
		}
		return nil
	}
	for i := range typ.Connectors {
		if typ.Connectors[i].Id == *connectorId {
			return &typ.Connectors[i]
		}
	}
	if len(typ.Connectors) > 0 {
		return &typ.Connectors[0]
	}
	return nil
}

// 🌤️FlattenDesignDiff computes absolute planes and centers for all pieces in a design (hot path; no report wrapper).
func FlattenDesignDiff(kit *Kit, designId string) DesignDiff {
	design := FindDesignInKit(kit, designId)
	if design == nil || len(design.Pieces) == 0 {
		return DesignDiff{}
	}

	removedConnList := make([]ConnectionId, 0, len(design.Connections))
	for i := range design.Connections {
		removedConnList = append(removedConnList, ConnectionId{Id: design.Connections[i].Id})
	}

	typesDict := make(map[string]*Type)
	for i := range kit.Types {
		typesDict[kit.Types[i].Id] = &kit.Types[i]
	}

	pieceMap := make(map[string]*Piece)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Id] = &design.Pieces[i]
	}

	piecePlanes := make(map[string]*Plane)
	adjacency := make(map[string][]struct {
		neighborId string
		connection   *Connection
	})

	for i := range design.Connections {
		conn := &design.Connections[i]
		srcId := conn.Parent.Piece.Id
		tgtId := conn.Child.Piece.Id
		if pieceMap[srcId] == nil || pieceMap[tgtId] == nil {
			continue
		}
		adjacency[srcId] = append(adjacency[srcId], struct {
			neighborId string
			connection   *Connection
		}{tgtId, conn})
		adjacency[tgtId] = append(adjacency[tgtId], struct {
			neighborId string
			connection   *Connection
		}{srcId, conn})
	}

	// Save original centers before BFS modifies pieces in-place.
	// pieceMap shares pointers with design.Pieces, so after BFS
	// piece.Center and pieceMap[id].Center are the same pointer.
	originalCenters := make(map[string]*Coordinate)
	for _, p := range design.Pieces {
		if p.Center != nil {
			c := *p.Center
			originalCenters[p.Id] = &c
		}
	}

	visited := make(map[string]bool)
	piecePaths := make(map[string]string)
	var bfs func(rootId string)
	bfs = func(rootId string) {
		queue := []string{rootId}
		visited[rootId] = true
		piecePaths[rootId] = rootId
		rootPiece := pieceMap[rootId]
		if rootPiece.Plane != nil && rootPiece.Center != nil {
			piecePlanes[rootId] = rootPiece.Plane
		} else {
			identityPlane := Plane{
				Origin: Point{X: 0, Y: 0, Z: 0},
				XAxis:  Vector{X: 1, Y: 0, Z: 0},
				YAxis:  Vector{X: 0, Y: 1, Z: 0},
			}
			piecePlanes[rootId] = &identityPlane
		}

		for len(queue) > 0 {
			currentId := queue[0]
			queue = queue[1:]
			currentPlane := piecePlanes[currentId]
			currentPiece := pieceMap[currentId]

			for _, neighbor := range adjacency[currentId] {
				if visited[neighbor.neighborId] {
					continue
				}
				visited[neighbor.neighborId] = true
				neighborPiece := pieceMap[neighbor.neighborId]
				conn := neighbor.connection

				var parentSide, childSide *Side
				if conn.Parent.Piece.Id == currentId {
					parentSide = &conn.Parent
					childSide = &conn.Child
				} else {
					parentSide = &conn.Child
					childSide = &conn.Parent
				}

				var parentType, childType *Type
				if currentPiece.Type != nil {
					parentType = typesDict[currentPiece.Type.Id]
				}
				if neighborPiece.Type != nil {
					childType = typesDict[neighborPiece.Type.Id]
				}

				var parentConnectorId, childConnectorId *string
				if parentSide.Connector != nil {
					parentConnectorId = &parentSide.Connector.Id
				}
				if childSide.Connector != nil {
					childConnectorId = &childSide.Connector.Id
				}

				parentConnector := getConnector(typesDict, parentType, parentConnectorId)
				childConnector := getConnector(typesDict, childType, childConnectorId)

				if parentConnector == nil || childConnector == nil {
					continue
				}

				childPlane := roundPlane(computeChildPlane(*currentPlane, *parentConnector, *childConnector, *conn))
				piecePlanes[neighbor.neighborId] = &childPlane

				radius := 2.697
				verticalVExtra := 1.0
				horizontalScale := 3.0633
				var parentCenter Coordinate
				if currentPiece.Center != nil {
					parentCenter = *currentPiece.Center
				}
				connectionU := conn.U
				connectionV := conn.V

				var childU, childV float64
				if parentCenter.U == 0 && parentCenter.V == 0 {
					angle := 2 * math.Pi * parentConnector.T
					childU = radius * math.Sin(angle)
					childV = radius * math.Cos(angle)
				} else {
					isVerticalConnection := math.Abs(parentConnector.Direction.Z) > 0.5
					if isVerticalConnection {
						childU = parentCenter.U + connectionU
						childV = parentCenter.V + connectionV + verticalVExtra
					} else {
						childU = parentCenter.U + connectionU*horizontalScale
						childV = parentCenter.V + connectionV*horizontalScale
					}
				}

				childCenter := &Coordinate{U: roundFloat(childU, 6), V: roundFloat(childV, 6)}
				neighborPiece.Center = childCenter
				piecePaths[neighbor.neighborId] = piecePaths[currentId] + "," + neighbor.neighborId

				queue = append(queue, neighbor.neighborId)
			}
		}
	}

	for _, piece := range design.Pieces {
		if !visited[piece.Id] {
			bfs(piece.Id)
		}
	}

	var updatedPieces []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	}

	for i := range design.Pieces {
		piece := &design.Pieces[i]
		plane := piecePlanes[piece.Id]
		if plane == nil {
			continue
		}
		diff := PieceDiff{}
		hasChanges := false

		if piece.Plane == nil || !planesEqualApprox(*plane, *piece.Plane) {
			diff.Plane = &PlaneDiff{
				Origin: &PointDiff{X: &plane.Origin.X, Y: &plane.Origin.Y, Z: &plane.Origin.Z},
				XAxis:  &VectorDiff{X: &plane.XAxis.X, Y: &plane.XAxis.Y, Z: &plane.XAxis.Z},
				YAxis:  &VectorDiff{X: &plane.YAxis.X, Y: &plane.YAxis.Y, Z: &plane.YAxis.Z},
			}
			hasChanges = true
		}

		pieceFromMap := pieceMap[piece.Id]
		if pieceFromMap.Center != nil {
			origCenter := originalCenters[piece.Id]
			if origCenter == nil || pieceFromMap.Center.U != origCenter.U || pieceFromMap.Center.V != origCenter.V {
				diff.Center = &CoordinateDiff{U: &pieceFromMap.Center.U, V: &pieceFromMap.Center.V}
				hasChanges = true
			}
		}

		if hasChanges {
			if path, ok := piecePaths[piece.Id]; ok {
				pathValue := path
				diff.Attributes = &AttributesDiff{
					Added: []Attribute{{Id: Id(), Key: "compose.path", Value: &pathValue}},
				}
			}
			updatedPieces = append(updatedPieces, struct {
				Piece PieceId   `json:"piece"`
				Diff  PieceDiff `json:"diff"`
			}{Piece: PieceId{Id: piece.Id}, Diff: diff})
		}
	}

	result := DesignDiff{}
	if len(updatedPieces) > 0 {
		result.Pieces = &PiecesDiff{Updated: updatedPieces}
	}
	if len(removedConnList) > 0 {
		result.Connections = &ConnectionsDiff{Removed: removedConnList}
	}
	return result
}

// 🌤️FlattenDesign returns the canonical ComposeReport with forward/backward DesignChange (merkle-cached on *Kit).
func FlattenDesign(kit *Kit, designId string) ComposeReport[DesignChange] {
	if kit == nil {
		return composeReportErr[DesignChange]([]OperationNote{{Message: "nil kit"}})
	}
	design := FindDesignInKit(kit, designId)
	if design == nil {
		return composeReportErr[DesignChange]([]OperationNote{{Code: "flatten.design-not-found", Message: fmt.Sprintf("Design %q not found in kit", designId)}})
	}
	if len(design.Pieces) == 0 {
		z := DesignChange{Forward: DesignDiff{}, Backward: DesignDiff{}}
		return ComposeReport[DesignChange]{
			Ok:       true,
			Diff:     &z,
			Warnings: []OperationNote{},
			Infos:    []OperationNote{{Code: "flatten.empty-pieces", Message: "No pieces to flatten; returning empty forward and backward diffs."}},
			Errors:   []OperationNote{},
		}
	}
	kit.graphMu.Lock()
	kit.ensureGraphMaps()
	prev := kit.flattenMerkle[designId]
	rep, next := FlattenDesignCached(kit, designId, prev)
	kit.flattenMerkle[designId] = next
	kit.graphMu.Unlock()
	return rep
}

func planesEqualApprox(a, b Plane) bool {
	const tol = 0.0001
	return math.Abs(a.Origin.X-b.Origin.X) < tol &&
		math.Abs(a.Origin.Y-b.Origin.Y) < tol &&
		math.Abs(a.Origin.Z-b.Origin.Z) < tol &&
		math.Abs(a.XAxis.X-b.XAxis.X) < tol &&
		math.Abs(a.XAxis.Y-b.XAxis.Y) < tol &&
		math.Abs(a.XAxis.Z-b.XAxis.Z) < tol &&
		math.Abs(a.YAxis.X-b.YAxis.X) < tol &&
		math.Abs(a.YAxis.Y-b.YAxis.Y) < tol &&
		math.Abs(a.YAxis.Z-b.YAxis.Z) < tol
}

// ✒️ApplyDesignDiff applies a design diff to a design, mutating it in place.
func ApplyDesignDiff(design *Design, diff *DesignDiff) {
	applyDesignDiff(design, diff)
}

// MoveVector carries gap/shift/rise deltas in the piece plane frame (gap along yAxis, shift along xAxis, rise along normal).
type MoveVector struct {
	Gap   float64 `json:"gap"`
	Shift float64 `json:"shift"`
	Rise  float64 `json:"rise"`
}

func movePlaneOriginDelta(plane *Plane, mv MoveVector) PointDiff {
	if plane == nil {
		return PointDiff{}
	}
	xAxis := []float64{plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z}
	yAxis := []float64{plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z}
	normalize(xAxis)
	normalize(yAxis)
	zAxis := cross(xAxis, yAxis)
	normalize(zAxis)
	tx := mv.Shift*xAxis[0] + mv.Gap*yAxis[0] + mv.Rise*zAxis[0]
	ty := mv.Shift*xAxis[1] + mv.Gap*yAxis[1] + mv.Rise*zAxis[1]
	tz := mv.Shift*xAxis[2] + mv.Gap*yAxis[2] + mv.Rise*zAxis[2]
	nx := plane.Origin.X + tx
	ny := plane.Origin.Y + ty
	nz := plane.Origin.Z + tz
	return PointDiff{X: &nx, Y: &ny, Z: &nz}
}

// moveTranslationWorld computes world-space translation from a piece plane and a MoveVector.
func moveTranslationWorld(plane Plane, mv MoveVector) []float64 {
	xAxis := []float64{plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z}
	yAxis := []float64{plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z}
	normalize(xAxis)
	normalize(yAxis)
	zAxis := cross(xAxis, yAxis)
	if zAxis[0]*zAxis[0]+zAxis[1]*zAxis[1]+zAxis[2]*zAxis[2] < 1e-12 {
		return []float64{0, 0, 0}
	}
	normalize(zAxis)
	return []float64{
		mv.Shift*xAxis[0] + mv.Gap*yAxis[0] + mv.Rise*zAxis[0],
		mv.Shift*xAxis[1] + mv.Gap*yAxis[1] + mv.Rise*zAxis[1],
		mv.Shift*xAxis[2] + mv.Gap*yAxis[2] + mv.Rise*zAxis[2],
	}
}

// identityPlaneForStructuralMove returns an identity plane (origin at 0, xAxis = +X, yAxis = +Y).
func identityPlaneForStructuralMove() Plane {
	return Plane{
		Origin: Point{X: 0, Y: 0, Z: 0},
		XAxis:  Vector{X: 1, Y: 0, Z: 0},
		YAxis:  Vector{X: 0, Y: 1, Z: 0},
	}
}

// connectionPlacementTranslationBasis returns gap/shift/rise unit vectors from a parent connector direction.
func connectionPlacementTranslationBasis(parentConnector Connector) (gapDir, shiftDir, raiseDir []float64) {
	parentDirection := []float64{parentConnector.Direction.X, parentConnector.Direction.Y, parentConnector.Direction.Z}
	normalize(parentDirection)
	yAxis := []float64{0, 1, 0}
	parentConnectorQuat := quaternionFromUnitVectors(yAxis, parentDirection)
	parentRotationT := quaternionToMatrix(parentConnectorQuat)
	gapDir = applyMatrix4ToVec3(parentRotationT, []float64{0, 1, 0})
	normalize(gapDir)
	shiftDir = applyMatrix4ToVec3(parentRotationT, []float64{1, 0, 0})
	normalize(shiftDir)
	raiseDir = applyMatrix4ToVec3(parentRotationT, []float64{0, 0, 1})
	normalize(raiseDir)
	return
}

// childConnectorOriginWorld computes the child plane origin in world space.
func childConnectorOriginWorld(parentPlane Plane, parentConnector, childConnector Connector, connection Connection) []float64 {
	plane := computeChildPlane(parentPlane, parentConnector, childConnector, connection)
	return []float64{plane.Origin.X, plane.Origin.Y, plane.Origin.Z}
}

// connectionNumericAt returns the value of a connection's numeric field by key name.
func connectionNumericAt(connection Connection, key string) float64 {
	switch key {
	case "gap":
		return connection.Gap
	case "shift":
		return connection.Shift
	case "rise":
		return connection.Rise
	case "rotation":
		return connection.Rotation
	case "turn":
		return connection.Turn
	case "tilt":
		return connection.Tilt
	}
	return 0
}

// connectionWithNumericDelta returns a copy of the connection with a numeric field perturbed by delta.
func connectionWithNumericDelta(connection Connection, key string, delta float64) Connection {
	c := connection
	switch key {
	case "gap":
		c.Gap += delta
	case "shift":
		c.Shift += delta
	case "rise":
		c.Rise += delta
	case "rotation":
		c.Rotation += delta
	case "turn":
		c.Turn += delta
	case "tilt":
		c.Tilt += delta
	}
	return c
}

// solveConnectionOriginMinNorm solves the min-norm problem: find deltas such that J*deltas ≈ t, using (JJ^T)^{-1}.
func solveConnectionOriginMinNorm(cols [][]float64, t []float64) []float64 {
	if len(cols) == 0 {
		return nil
	}
	// Build 3x3 JJ^T matrix (row-major)
	var jjt [9]float64
	for c := 0; c < 3; c++ {
		for r := 0; r < 3; r++ {
			s := 0.0
			for _, col := range cols {
				s += col[r] * col[c]
			}
			jjt[r+c*3] = s
		}
	}
	// Regularization
	jjt[0] += 1e-14
	jjt[4] += 1e-14
	jjt[8] += 1e-14
	// Determinant
	det := jjt[0]*(jjt[4]*jjt[8]-jjt[7]*jjt[5]) -
		jjt[3]*(jjt[1]*jjt[8]-jjt[7]*jjt[2]) +
		jjt[6]*(jjt[1]*jjt[5]-jjt[4]*jjt[2])
	if math.Abs(det) < 1e-22 {
		return nil
	}
	// 3x3 inverse (column-major like THREE.Matrix3: elements[r+c*3])
	invDet := 1.0 / det
	var inv [9]float64
	inv[0] = (jjt[4]*jjt[8] - jjt[5]*jjt[7]) * invDet
	inv[1] = (jjt[2]*jjt[7] - jjt[1]*jjt[8]) * invDet
	inv[2] = (jjt[1]*jjt[5] - jjt[2]*jjt[4]) * invDet
	inv[3] = (jjt[5]*jjt[6] - jjt[3]*jjt[8]) * invDet
	inv[4] = (jjt[0]*jjt[8] - jjt[2]*jjt[6]) * invDet
	inv[5] = (jjt[2]*jjt[3] - jjt[0]*jjt[5]) * invDet
	inv[6] = (jjt[3]*jjt[7] - jjt[4]*jjt[6]) * invDet
	inv[7] = (jjt[1]*jjt[6] - jjt[0]*jjt[7]) * invDet
	inv[8] = (jjt[0]*jjt[4] - jjt[1]*jjt[3]) * invDet
	if math.IsInf(inv[0], 0) || math.IsNaN(inv[0]) {
		return nil
	}
	// u = inv * t (column-major multiply: u[r] = sum_c inv[r+c*3]*t[c])
	u := []float64{
		inv[0]*t[0] + inv[3]*t[1] + inv[6]*t[2],
		inv[1]*t[0] + inv[4]*t[1] + inv[7]*t[2],
		inv[2]*t[0] + inv[5]*t[1] + inv[8]*t[2],
	}
	// deltas[i] = cols[i] · u
	deltas := make([]float64, len(cols))
	for i, col := range cols {
		deltas[i] = col[0]*u[0] + col[1]*u[1] + col[2]*u[2]
	}
	return deltas
}

// connectionDiffTranslationFallback projects translation onto connector gap/shift/rise basis, residual onto parent plane x/y.
func connectionDiffTranslationFallback(parentPlane Plane, parentConnector Connector, t []float64) ConnectionDiff {
	gapDir, shiftDir, raiseDir := connectionPlacementTranslationBasis(parentConnector)
	dgap := dot(t, gapDir)
	dshift := dot(t, shiftDir)
	drise := dot(t, raiseDir)
	res := []float64{
		t[0] - dgap*gapDir[0] - dshift*shiftDir[0] - drise*raiseDir[0],
		t[1] - dgap*gapDir[1] - dshift*shiftDir[1] - drise*raiseDir[1],
		t[2] - dgap*gapDir[2] - dshift*shiftDir[2] - drise*raiseDir[2],
	}
	px := []float64{parentPlane.XAxis.X, parentPlane.XAxis.Y, parentPlane.XAxis.Z}
	py := []float64{parentPlane.YAxis.X, parentPlane.YAxis.Y, parentPlane.YAxis.Z}
	diff := ConnectionDiff{}
	eps := 1e-9
	if math.Abs(dgap) > eps {
		diff.Gap = &dgap
	}
	if math.Abs(dshift) > eps {
		diff.Shift = &dshift
	}
	if math.Abs(drise) > eps {
		diff.Rise = &drise
	}
	pxSq := px[0]*px[0] + px[1]*px[1] + px[2]*px[2]
	pySq := py[0]*py[0] + py[1]*py[1] + py[2]*py[2]
	if pxSq > 1e-24 && pySq > 1e-24 {
		pxN := []float64{px[0] / math.Sqrt(pxSq), px[1] / math.Sqrt(pxSq), px[2] / math.Sqrt(pxSq)}
		pyN := []float64{py[0] / math.Sqrt(pySq), py[1] / math.Sqrt(pySq), py[2] / math.Sqrt(pySq)}
		du := dot(res, pxN)
		dv := dot(res, pyN)
		if math.Abs(du) > eps {
			diff.U = &du
		}
		if math.Abs(dv) > eps {
			diff.V = &dv
		}
	}
	return diff
}

// connectionDiffFromStructuralMoveVector computes connection diffs using a numerical Jacobian of computeChildPlane.
func connectionDiffFromStructuralMoveVector(
	parentPlane Plane,
	parentConnector Connector,
	childConnector *Connector,
	connection Connection,
	childPlane *Plane,
	vector MoveVector,
) ConnectionDiff {
	child := identityPlaneForStructuralMove()
	if childPlane != nil {
		child = *childPlane
	}
	tw := moveTranslationWorld(child, vector)
	tSq := tw[0]*tw[0] + tw[1]*tw[1] + tw[2]*tw[2]
	if tSq < 1e-24 {
		return ConnectionDiff{}
	}
	if childConnector == nil {
		return connectionDiffTranslationFallback(parentPlane, parentConnector, tw)
	}
	jacobianKeys := []string{"gap", "shift", "rise", "rotation", "turn", "tilt"}
	jacobianEps := map[string]float64{
		"gap": 1e-6, "shift": 1e-6, "rise": 1e-6,
		"rotation": 1e-4, "turn": 1e-4, "tilt": 1e-4,
	}
	o0 := childConnectorOriginWorld(parentPlane, parentConnector, *childConnector, connection)
	cols := make([][]float64, len(jacobianKeys))
	for i, key := range jacobianKeys {
		eps := jacobianEps[key]
		perturbed := connectionWithNumericDelta(connection, key, eps)
		o1 := childConnectorOriginWorld(parentPlane, parentConnector, *childConnector, perturbed)
		cols[i] = []float64{
			(o1[0] - o0[0]) / eps,
			(o1[1] - o0[1]) / eps,
			(o1[2] - o0[2]) / eps,
		}
	}
	deltas := solveConnectionOriginMinNorm(cols, tw)
	diff := ConnectionDiff{}
	epsOut := 1e-9
	if deltas != nil {
		for i, key := range jacobianKeys {
			if math.Abs(deltas[i]) > epsOut {
				v := deltas[i]
				switch key {
				case "gap":
					diff.Gap = &v
				case "shift":
					diff.Shift = &v
				case "rise":
					diff.Rise = &v
				case "rotation":
					diff.Rotation = &v
				case "turn":
					diff.Turn = &v
				case "tilt":
					diff.Tilt = &v
				}
			}
		}
		// Compute residual
		pred := []float64{0, 0, 0}
		for i, col := range cols {
			pred[0] += col[0] * deltas[i]
			pred[1] += col[1] * deltas[i]
			pred[2] += col[2] * deltas[i]
		}
		res := []float64{tw[0] - pred[0], tw[1] - pred[1], tw[2] - pred[2]}
		px := []float64{parentPlane.XAxis.X, parentPlane.XAxis.Y, parentPlane.XAxis.Z}
		py := []float64{parentPlane.YAxis.X, parentPlane.YAxis.Y, parentPlane.YAxis.Z}
		pxSq := px[0]*px[0] + px[1]*px[1] + px[2]*px[2]
		pySq := py[0]*py[0] + py[1]*py[1] + py[2]*py[2]
		if pxSq > 1e-24 && pySq > 1e-24 {
			pxN := []float64{px[0] / math.Sqrt(pxSq), px[1] / math.Sqrt(pxSq), px[2] / math.Sqrt(pxSq)}
			pyN := []float64{py[0] / math.Sqrt(pySq), py[1] / math.Sqrt(pySq), py[2] / math.Sqrt(pySq)}
			du := dot(res, pxN)
			dv := dot(res, pyN)
			if math.Abs(du) > epsOut {
				diff.U = &du
			}
			if math.Abs(dv) > epsOut {
				diff.V = &dv
			}
		}
		return diff
	}
	return connectionDiffTranslationFallback(parentPlane, parentConnector, tw)
}

// MovePiecesInDesign computes a DesignDiff that translates root piece planes and adjusts connection diffs using a numerical Jacobian for selected child movers.
// A piece's parent connection is the connection where it is the child side.
func MovePiecesInDesign(kit Kit, design Design, pieces Design, vector MoveVector) DesignDiff {
	typesDict := make(map[string]*Type)
	for i := range kit.Types {
		typesDict[kit.Types[i].Id] = &kit.Types[i]
	}
	selectedIds := make(map[string]bool)
	for _, p := range pieces.Pieces {
		selectedIds[p.Id] = true
	}
	parentMap := make(map[string]struct{ connectionId, parentId string })
	for _, c := range design.Connections {
		parentMap[c.Child.Piece.Id] = struct{ connectionId, parentId string }{c.Id, c.Parent.Piece.Id}
	}
	fixedIds := make(map[string]bool)
	for id := range selectedIds {
		if _, hasParent := parentMap[id]; !hasParent {
			fixedIds[id] = true
		}
	}
	var pieceUpdates []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	}
	pieceMap := make(map[string]*Piece)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Id] = &design.Pieces[i]
	}
	for id := range fixedIds {
		p, ok := pieceMap[id]
		if !ok || p.Plane == nil {
			continue
		}
		orig := movePlaneOriginDelta(p.Plane, vector)
		pieceUpdates = append(pieceUpdates, struct {
			Piece PieceId   `json:"piece"`
			Diff  PieceDiff `json:"diff"`
		}{
			Piece: PieceId{Id: id},
			Diff:  PieceDiff{Plane: &PlaneDiff{Origin: &orig}},
		})
	}
	connMap := make(map[string]*Connection)
	for i := range design.Connections {
		connMap[design.Connections[i].Id] = &design.Connections[i]
	}
	var connectionUpdates []struct {
		Connection ConnectionId   `json:"connection"`
		Diff       ConnectionDiff `json:"diff"`
	}
	for id := range selectedIds {
		if fixedIds[id] {
			continue
		}
		isDescendant := false
		current := id
		for {
			p, ok := parentMap[current]
			if !ok {
				break
			}
			if selectedIds[p.parentId] {
				isDescendant = true
				break
			}
			current = p.parentId
		}
		if isDescendant {
			continue
		}
		parent, ok := parentMap[id]
		if !ok {
			continue
		}
		connection := connMap[parent.connectionId]
		if connection == nil {
			continue
		}
		parentPiece := pieceMap[parent.parentId]
		childPiece := pieceMap[id]
		if parentPiece == nil || childPiece == nil {
			continue
		}
		if parentPiece.Type == nil || childPiece.Type == nil {
			continue
		}
		parentType := typesDict[parentPiece.Type.Id]
		childType := typesDict[childPiece.Type.Id]
		parentConnector := getConnector(typesDict, parentType, func() *string {
			if connection.Parent.Connector != nil {
				return &connection.Parent.Connector.Id
			}
			return nil
		}())
		childConnector := getConnector(typesDict, childType, func() *string {
			if connection.Child.Connector != nil {
				return &connection.Child.Connector.Id
			}
			return nil
		}())
		if parentConnector == nil {
			continue
		}
		parentPlane := identityPlaneForStructuralMove()
		if parentPiece.Plane != nil {
			parentPlane = *parentPiece.Plane
		}
		connDiff := connectionDiffFromStructuralMoveVector(parentPlane, *parentConnector, childConnector, *connection, childPiece.Plane, vector)
		hasFields := connDiff.Gap != nil || connDiff.Shift != nil || connDiff.Rise != nil ||
			connDiff.Rotation != nil || connDiff.Turn != nil || connDiff.Tilt != nil ||
			connDiff.U != nil || connDiff.V != nil
		if !hasFields {
			continue
		}
		connectionUpdates = append(connectionUpdates, struct {
			Connection ConnectionId   `json:"connection"`
			Diff       ConnectionDiff `json:"diff"`
		}{
			Connection: ConnectionId{Id: parent.connectionId},
			Diff:       connDiff,
		})
		if vector.Rise != 0 {
			r := vector.Rise
			connectionUpdates[len(connectionUpdates)-1].Diff.Rise = &r
		}
	}
	diff := DesignDiff{}
	if len(pieceUpdates) > 0 {
		diff.Pieces = &PiecesDiff{Updated: pieceUpdates}
	}
	if len(connectionUpdates) > 0 {
		diff.Connections = &ConnectionsDiff{Updated: connectionUpdates}
	}
	return diff
}

// 🔌DragPiecesInDesign computes a DesignDiff that offsets selected piece centers and adjusts orphan connections.
// 🔗A piece's parent connection is the connection where it is the child side.
func DragPiecesInDesign(design Design, pieces Design, offset Coordinate) DesignDiff {
	selectedIds := make(map[string]bool)
	for _, p := range pieces.Pieces {
		selectedIds[p.Id] = true
	}
	parentMap := make(map[string]struct{ connectionId, parentId string })
	for _, c := range design.Connections {
		parentMap[c.Child.Piece.Id] = struct{ connectionId, parentId string }{c.Id, c.Parent.Piece.Id}
	}
	fixedIds := make(map[string]bool)
	for id := range selectedIds {
		if _, hasParent := parentMap[id]; !hasParent {
			fixedIds[id] = true
		}
	}
	var pieceUpdates []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	}
	pieceMap := make(map[string]*Piece)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Id] = &design.Pieces[i]
	}
	for id := range fixedIds {
		if p, ok := pieceMap[id]; ok && p.Center != nil {
			newU := p.Center.U + offset.U
			newV := p.Center.V + offset.V
			pieceUpdates = append(pieceUpdates, struct {
				Piece PieceId   `json:"piece"`
				Diff  PieceDiff `json:"diff"`
			}{
				Piece: PieceId{Id: id},
				Diff:  PieceDiff{Center: &CoordinateDiff{U: &newU, V: &newV}},
			})
		}
	}
	var connectionUpdates []struct {
		Connection ConnectionId   `json:"connection"`
		Diff       ConnectionDiff `json:"diff"`
	}
	for id := range selectedIds {
		if fixedIds[id] {
			continue
		}
		isDescendant := false
		current := id
		for {
			p, ok := parentMap[current]
			if !ok {
				break
			}
			if selectedIds[p.parentId] {
				isDescendant = true
				break
			}
			current = p.parentId
		}
		if isDescendant {
			continue
		}
		parent, ok := parentMap[id]
		if !ok {
			continue
		}
		connU := offset.U
		connV := offset.V
		connectionUpdates = append(connectionUpdates, struct {
			Connection ConnectionId   `json:"connection"`
			Diff       ConnectionDiff `json:"diff"`
		}{
			Connection: ConnectionId{Id: parent.connectionId},
			Diff:       ConnectionDiff{U: &connU, V: &connV},
		})
	}
	diff := DesignDiff{}
	if len(pieceUpdates) > 0 {
		diff.Pieces = &PiecesDiff{Updated: pieceUpdates}
	}
	if len(connectionUpdates) > 0 {
		diff.Connections = &ConnectionsDiff{Updated: connectionUpdates}
	}
	return diff
}

// #region 🌳Flatten Merkle Hashes
// 💾Per-piece merkle hashes for plane and center computations so subsequent flatten calls can skip unchanged chains.

// 🌳FlatMerkleHashes bundles the per-piece merkle hashes computed for a flattened design.
type FlatMerkleHashes struct {
	PlaneHash  string `json:"planeHash"`
	CenterHash string `json:"centerHash"`
}

// 🧠FlatMerkleCacheEntry pairs the hashes with the last resolved plane/center so callers can reuse values across flatten runs.
type FlatMerkleCacheEntry struct {
	PlaneHash  string `json:"planeHash"`
	CenterHash string `json:"centerHash"`
	Plane      *Plane `json:"plane,omitempty"`
	Center     *Coordinate `json:"center,omitempty"`
}

// 🌱hashPlaneRoot computes the root plane hash from only the piece id and its fixed plane components.
func hashPlaneRoot(id string, plane *Plane) string {
	w := &hashWriter{}
	if plane == nil {
		w.writeString("plane.root.identity")
		w.writeString(id)
		return w.digest()
	}
	w.writeString("plane.root")
	w.writeString(id)
	w.writeNumber(plane.Origin.X)
	w.writeNumber(plane.Origin.Y)
	w.writeNumber(plane.Origin.Z)
	w.writeNumber(plane.XAxis.X)
	w.writeNumber(plane.XAxis.Y)
	w.writeNumber(plane.XAxis.Z)
	w.writeNumber(plane.YAxis.X)
	w.writeNumber(plane.YAxis.Y)
	w.writeNumber(plane.YAxis.Z)
	return w.digest()
}

// 🔗hashPlaneChain computes a child plane hash from the parent hash and all inputs consumed by computeChildPlane.
func hashPlaneChain(parentHash string, parentConnector, childConnector Connector, connection Connection) string {
	w := &hashWriter{}
	w.writeString("plane.chain")
	w.writeHash(parentHash)
	w.writeNumber(parentConnector.Point.X)
	w.writeNumber(parentConnector.Point.Y)
	w.writeNumber(parentConnector.Point.Z)
	w.writeNumber(parentConnector.Direction.X)
	w.writeNumber(parentConnector.Direction.Y)
	w.writeNumber(parentConnector.Direction.Z)
	w.writeNumber(childConnector.Point.X)
	w.writeNumber(childConnector.Point.Y)
	w.writeNumber(childConnector.Point.Z)
	w.writeNumber(childConnector.Direction.X)
	w.writeNumber(childConnector.Direction.Y)
	w.writeNumber(childConnector.Direction.Z)
	w.writeNumber(connection.Gap)
	w.writeNumber(connection.Shift)
	w.writeNumber(connection.Rise)
	w.writeNumber(connection.Rotation)
	w.writeNumber(connection.Turn)
	w.writeNumber(connection.Tilt)
	return w.digest()
}

// 🌱hashCenterRoot computes the root center hash from only the piece id and its fixed center (identity when absent).
func hashCenterRoot(id string, center *Coordinate) string {
	w := &hashWriter{}
	if center == nil {
		w.writeString("center.root.identity")
		w.writeString(id)
		return w.digest()
	}
	w.writeString("center.root")
	w.writeString(id)
	w.writeNumber(center.U)
	w.writeNumber(center.V)
	return w.digest()
}

// 🔗hashCenterChain computes a child center hash from the parent hash plus the inputs consumed by the child center computation.
func hashCenterChain(parentHash string, parentConnector Connector, connection Connection) string {
	w := &hashWriter{}
	w.writeString("center.chain")
	w.writeHash(parentHash)
	w.writeNumber(parentConnector.Direction.Z)
	w.writeNumber(parentConnector.T)
	w.writeNumber(connection.U)
	w.writeNumber(connection.V)
	return w.digest()
}

// 🌳ComputeFlatHashes returns {planeHash, centerHash} for every piece reachable from a root in each connected component of the design.
func ComputeFlatHashes(kit *Kit, designId string) map[string]FlatMerkleHashes {
	design := FindDesignInKit(kit, designId)
	if design == nil || len(design.Pieces) == 0 {
		return map[string]FlatMerkleHashes{}
	}
	typesDict := make(map[string]*Type)
	for i := range kit.Types {
		typesDict[kit.Types[i].Id] = &kit.Types[i]
	}
	pieceMap := make(map[string]*Piece)
	pieceIndex := make(map[string]int)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Id] = &design.Pieces[i]
		pieceIndex[design.Pieces[i].Id] = i
	}
	adjacency := make(map[string][]struct {
		neighborId string
		connection   *Connection
	})
	for i := range design.Connections {
		conn := &design.Connections[i]
		srcId := conn.Parent.Piece.Id
		tgtId := conn.Child.Piece.Id
		if pieceMap[srcId] == nil || pieceMap[tgtId] == nil {
			continue
		}
		adjacency[srcId] = append(adjacency[srcId], struct {
			neighborId string
			connection   *Connection
		}{tgtId, conn})
		adjacency[tgtId] = append(adjacency[tgtId], struct {
			neighborId string
			connection   *Connection
		}{srcId, conn})
	}

	componentOf := make(map[string]int)
	var components [][]string
	for i := range design.Pieces {
		id := design.Pieces[i].Id
		if _, ok := componentOf[id]; ok {
			continue
		}
		idx := len(components)
		queue := []string{id}
		componentOf[id] = idx
		members := []string{id}
		for len(queue) > 0 {
			cur := queue[0]
			queue = queue[1:]
			for _, nb := range adjacency[cur] {
				if _, seen := componentOf[nb.neighborId]; seen {
					continue
				}
				componentOf[nb.neighborId] = idx
				members = append(members, nb.neighborId)
				queue = append(queue, nb.neighborId)
			}
		}
		components = append(components, members)
	}

	planeHashes := make(map[string]string)
	centerHashes := make(map[string]string)

	for _, members := range components {
		memberSet := make(map[string]bool, len(members))
		for _, g := range members {
			memberSet[g] = true
		}
		var rootId string
		for i := range design.Pieces {
			p := &design.Pieces[i]
			if !memberSet[p.Id] {
				continue
			}
			if p.Plane != nil && p.Center != nil {
				rootId = p.Id
				break
			}
		}
		if rootId == "" {
			sorted := make([]string, len(members))
			copy(sorted, members)
			sort.Strings(sorted)
			if len(sorted) == 0 {
				continue
			}
			rootId = sorted[0]
		}
		rootPiece := pieceMap[rootId]
		planeHashes[rootId] = hashPlaneRoot(rootId, rootPiece.Plane)
		centerHashes[rootId] = hashCenterRoot(rootId, rootPiece.Center)

		visited := map[string]bool{rootId: true}
		queue := []string{rootId}
		for len(queue) > 0 {
			current := queue[0]
			queue = queue[1:]
			currentPiece := pieceMap[current]
			for _, nb := range adjacency[current] {
				if visited[nb.neighborId] {
					continue
				}
				visited[nb.neighborId] = true
				childId := nb.neighborId
				conn := nb.connection
				var parentSide, childSide *Side
				if conn.Parent.Piece.Id == current {
					parentSide = &conn.Parent
					childSide = &conn.Child
				} else {
					parentSide = &conn.Child
					childSide = &conn.Parent
				}
				childPiece := pieceMap[childId]
				var parentType, childType *Type
				if currentPiece != nil && currentPiece.Type != nil {
					parentType = typesDict[currentPiece.Type.Id]
				}
				if childPiece != nil && childPiece.Type != nil {
					childType = typesDict[childPiece.Type.Id]
				}
				var parentConnectorId, childConnectorId *string
				if parentSide.Connector != nil {
					parentConnectorId = &parentSide.Connector.Id
				}
				if childSide.Connector != nil {
					childConnectorId = &childSide.Connector.Id
				}
				parentConnector := getConnector(typesDict, parentType, parentConnectorId)
				childConnector := getConnector(typesDict, childType, childConnectorId)
				if parentConnector == nil || childConnector == nil {
					continue
				}
				planeHashes[childId] = hashPlaneChain(planeHashes[current], *parentConnector, *childConnector, *conn)
				centerHashes[childId] = hashCenterChain(centerHashes[current], *parentConnector, *conn)
				queue = append(queue, childId)
			}
		}
	}

	result := make(map[string]FlatMerkleHashes, len(planeHashes))
	for id, ph := range planeHashes {
		result[id] = FlatMerkleHashes{PlaneHash: ph, CenterHash: centerHashes[id]}
	}
	return result
}

// 🧠FlattenDesignCached runs FlattenDesignDiff but reuses cached plane/center values whenever the merkle hash for a piece is unchanged.
func FlattenDesignCached(kit *Kit, designId string, cache map[string]FlatMerkleCacheEntry) (ComposeReport[DesignChange], map[string]FlatMerkleCacheEntry) {
	design := FindDesignInKit(kit, designId)
	var before Design
	hasBefore := design != nil && len(design.Pieces) > 0
	if hasBefore {
		before = deepCloneDesign(*design)
	}
	newHashes := ComputeFlatHashes(kit, designId)
	diff := FlattenDesignDiff(kit, designId)
	var backward DesignDiff
	if hasBefore {
		backward = inverseDesignDiff(before, diff)
	}
	updatedById := make(map[string]PieceDiff)
	if diff.Pieces != nil {
		for _, entry := range diff.Pieces.Updated {
			updatedById[entry.Piece.Id] = entry.Diff
		}
	}
	extractPlane := func(pd PieceDiff) *Plane {
		if pd.Plane == nil || pd.Plane.Origin == nil || pd.Plane.XAxis == nil || pd.Plane.YAxis == nil {
			return nil
		}
		if pd.Plane.Origin.X == nil || pd.Plane.Origin.Y == nil || pd.Plane.Origin.Z == nil ||
			pd.Plane.XAxis.X == nil || pd.Plane.XAxis.Y == nil || pd.Plane.XAxis.Z == nil ||
			pd.Plane.YAxis.X == nil || pd.Plane.YAxis.Y == nil || pd.Plane.YAxis.Z == nil {
			return nil
		}
		return &Plane{
			Origin: Point{X: *pd.Plane.Origin.X, Y: *pd.Plane.Origin.Y, Z: *pd.Plane.Origin.Z},
			XAxis:  Vector{X: *pd.Plane.XAxis.X, Y: *pd.Plane.XAxis.Y, Z: *pd.Plane.XAxis.Z},
			YAxis:  Vector{X: *pd.Plane.YAxis.X, Y: *pd.Plane.YAxis.Y, Z: *pd.Plane.YAxis.Z},
		}
	}
	extractCenter := func(pd PieceDiff) *Coordinate {
		if pd.Center == nil || pd.Center.U == nil || pd.Center.V == nil {
			return nil
		}
		return &Coordinate{U: *pd.Center.U, V: *pd.Center.V}
	}
	nextCache := make(map[string]FlatMerkleCacheEntry, len(newHashes))
	for id, hashes := range newHashes {
		updated, hasUpdated := updatedById[id]
		var prev *FlatMerkleCacheEntry
		if cache != nil {
			if v, ok := cache[id]; ok {
				pv := v
				prev = &pv
			}
		}
		if prev == nil || !hasUpdated {
			if hasUpdated {
				nextCache[id] = FlatMerkleCacheEntry{
					PlaneHash:  hashes.PlaneHash,
					CenterHash: hashes.CenterHash,
					Plane:      extractPlane(updated),
					Center:     extractCenter(updated),
				}
			}
			continue
		}
		reusedPlane := extractPlane(updated)
		if prev.PlaneHash == hashes.PlaneHash {
			reusedPlane = prev.Plane
		}
		reusedCenter := extractCenter(updated)
		if prev.CenterHash == hashes.CenterHash {
			reusedCenter = prev.Center
		}
		nextCache[id] = FlatMerkleCacheEntry{
			PlaneHash:  hashes.PlaneHash,
			CenterHash: hashes.CenterHash,
			Plane:      reusedPlane,
			Center:     reusedCenter,
		}
	}
	return composeReportOk(DesignChange{Forward: diff, Backward: backward}), nextCache
}

// #endregion 🌳Flatten Merkle Hashes

// #endregion 🌤️Flatten Design

// #region 🔩Kit Representation Export

// 📤ExportRepresentationFormats maps supported export format extensions.
var ExportRepresentationFormats = map[string]string{
	".glb":  ".glb",
	".gltf": ".gltf",
}

// #region 🔧Kit Representation Export Helpers

// 📤exportMeshData holds extracted or generated mesh geometry for a single type.
type exportMeshData struct {
	positionBytes []byte
	indexBytes    []byte
	vertexCount   int
	indexCount    int
	posMin        [3]float32
	posMax        [3]float32
	indexCompKind int
}

// 🔲exportPlaneToGltfMatrix converts a Plane to a column-major 4x4 matrix for glTF.
func exportPlaneToGltfMatrix(plane Plane) [16]float64 {
	ox, oy, oz := plane.Origin.X, plane.Origin.Y, plane.Origin.Z
	xx, xy, xz := plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z
	yx, yy, yz := plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z
	zx := xy*yz - xz*yy
	zy := xz*yx - xx*yz
	zz := xx*yy - xy*yx
	zLen := math.Sqrt(zx*zx + zy*zy + zz*zz)
	if zLen > 0 {
		zx /= zLen
		zy /= zLen
		zz /= zLen
	}
	xLen := math.Sqrt(xx*xx + xy*xy + xz*xz)
	if xLen > 0 {
		xx /= xLen
		xy /= xLen
		xz /= xLen
	}
	yx = zy*xz - zz*xy
	yy = zz*xx - zx*xz
	yz = zx*xy - zy*xx
	yLen := math.Sqrt(yx*yx + yy*yy + yz*yz)
	if yLen > 0 {
		yx /= yLen
		yy /= yLen
		yz /= yLen
	}
	return exportApplyComposeToGltfBasis([16]float64{xx, xy, xz, 0, yx, yy, yz, 0, zx, zy, zz, 0, ox, oy, oz, 1})
}

// 🗃️exportDenseToGltfMatrix converts a gonum mat.Dense (row-major) to column-major glTF matrix.
func exportDenseToGltfMatrix(m *mat.Dense) [16]float64 {
	return exportApplyComposeToGltfBasis([16]float64{
		m.At(0, 0), m.At(1, 0), m.At(2, 0), m.At(3, 0),
		m.At(0, 1), m.At(1, 1), m.At(2, 1), m.At(3, 1),
		m.At(0, 2), m.At(1, 2), m.At(2, 2), m.At(3, 2),
		m.At(0, 3), m.At(1, 3), m.At(2, 3), m.At(3, 3),
	})
}

func exportApplyComposeToGltfBasis(matrix [16]float64) [16]float64 {
	basis := [16]float64{
		1, 0, 0, 0,
		0, 0, -1, 0,
		0, 1, 0, 0,
		0, 0, 0, 1,
	}
	basisInv := [16]float64{
		1, 0, 0, 0,
		0, 0, 1, 0,
		0, -1, 0, 0,
		0, 0, 0, 1,
	}
	left := exportMultiplyColumnMajor4x4(basis, matrix)
	return exportMultiplyColumnMajor4x4(left, basisInv)
}

func exportMultiplyColumnMajor4x4(left [16]float64, right [16]float64) [16]float64 {
	var result [16]float64
	for column := 0; column < 4; column++ {
		for row := 0; row < 4; row++ {
			sum := 0.0
			for k := 0; k < 4; k++ {
				sum += left[k*4+row] * right[column*4+k]
			}
			result[column*4+row] = sum
		}
	}
	return result
}

// 🆕exportCreateBoxMesh generates a unit box placeholder mesh (1x1x1 centered at origin).
func exportCreateBoxMesh() *exportMeshData {
	s := float32(0.5)
	vertices := [][3]float32{
		{-s, -s, -s}, {s, -s, -s}, {s, s, -s}, {-s, s, -s},
		{-s, -s, s}, {s, -s, s}, {s, s, s}, {-s, s, s},
	}
	indices := []uint16{
		0, 1, 2, 0, 2, 3,
		4, 6, 5, 4, 7, 6,
		0, 4, 5, 0, 5, 1,
		3, 2, 6, 3, 6, 7,
		0, 3, 7, 0, 7, 4,
		1, 5, 6, 1, 6, 2,
	}
	posBuf := new(bytes.Buffer)
	for _, v := range vertices {
		binary.Write(posBuf, binary.LittleEndian, v[0])
		binary.Write(posBuf, binary.LittleEndian, v[1])
		binary.Write(posBuf, binary.LittleEndian, v[2])
	}
	idxBuf := new(bytes.Buffer)
	for _, idx := range indices {
		binary.Write(idxBuf, binary.LittleEndian, idx)
	}
	return &exportMeshData{
		positionBytes: posBuf.Bytes(),
		indexBytes:    idxBuf.Bytes(),
		vertexCount:   len(vertices),
		indexCount:    len(indices),
		posMin:        [3]float32{-s, -s, -s},
		posMax:        [3]float32{s, s, s},
		indexCompKind: 5123,
	}
}

// 🔗exportDecodeBlobToBytes strips a data URI prefix and base64 decodes the blob content.
func exportDecodeBlobToBytes(blob string) ([]byte, error) {
	if idx := strings.Index(blob, ","); idx >= 0 {
		blob = blob[idx+1:]
	}
	decoded, err := base64.StdEncoding.DecodeString(blob)
	if err != nil {
		decoded, err = base64.RawStdEncoding.DecodeString(blob)
		if err != nil {
			return nil, fmt.Errorf("base64 decode failed: %w", err)
		}
	}
	return decoded, nil
}

// 🧲exportParseGLBMesh parses a GLB binary file and extracts the first mesh's geometry data.
func exportParseGLBMesh(data []byte) (*exportMeshData, error) {
	if len(data) < 12 {
		return nil, fmt.Errorf("GLB too short")
	}
	magic := binary.LittleEndian.Uint32(data[0:4])
	if magic != 0x46546C67 {
		return nil, fmt.Errorf("not a GLB file")
	}

	offset := 12
	if offset+8 > len(data) {
		return nil, fmt.Errorf("missing JSON chunk header")
	}
	jsonChunkLen := int(binary.LittleEndian.Uint32(data[offset : offset+4]))
	jsonChunkKind := binary.LittleEndian.Uint32(data[offset+4 : offset+8])
	if jsonChunkKind != 0x4E4F534A {
		return nil, fmt.Errorf("expected JSON chunk")
	}
	offset += 8
	if offset+jsonChunkLen > len(data) {
		return nil, fmt.Errorf("JSON chunk overflow")
	}
	jsonData := data[offset : offset+jsonChunkLen]
	offset += jsonChunkLen

	var binData []byte
	if offset+8 <= len(data) {
		binChunkLen := int(binary.LittleEndian.Uint32(data[offset : offset+4]))
		binChunkKind := binary.LittleEndian.Uint32(data[offset+4 : offset+8])
		if binChunkKind == 0x004E4942 {
			offset += 8
			if offset+binChunkLen <= len(data) {
				binData = data[offset : offset+binChunkLen]
			}
		}
	}

	var gltf map[string]interface{}
	if err := json.Unmarshal(jsonData, &gltf); err != nil {
		return nil, fmt.Errorf("failed to parse glTF JSON: %w", err)
	}
	return exportParseGltfToMeshData(gltf, binData)
}

// 🔬exportParseGltfToMeshData extracts merged mesh geometry from a glTF JSON map and binary buffer.
func exportParseGltfToMeshData(gltf map[string]interface{}, binData []byte) (*exportMeshData, error) {
	meshesRaw, ok := gltf["meshes"].([]interface{})
	if !ok || len(meshesRaw) == 0 {
		return nil, fmt.Errorf("no meshes in glTF")
	}
	accessorsRaw, _ := gltf["accessors"].([]interface{})
	bufferViewsRaw, _ := gltf["bufferViews"].([]interface{})

	glbInt := func(m map[string]interface{}, key string) int {
		if v, ok := m[key]; ok {
			if f, ok := v.(float64); ok {
				return int(f)
			}
		}
		return 0
	}

	getBufferView := func(index int) (map[string]interface{}, error) {
		if index < 0 || index >= len(bufferViewsRaw) {
			return nil, fmt.Errorf("bufferView out of range")
		}
		bufferView, ok := bufferViewsRaw[index].(map[string]interface{})
		if !ok {
			return nil, fmt.Errorf("invalid bufferView")
		}
		return bufferView, nil
	}

	getAccessor := func(index int) (map[string]interface{}, error) {
		if index < 0 || index >= len(accessorsRaw) {
			return nil, fmt.Errorf("accessor out of range")
		}
		accessor, ok := accessorsRaw[index].(map[string]interface{})
		if !ok {
			return nil, fmt.Errorf("invalid accessor")
		}
		return accessor, nil
	}

	readAccessorBytes := func(accessor map[string]interface{}, elementSize int) ([]byte, int, error) {
		bufferViewIndex := glbInt(accessor, "bufferView")
		bufferView, err := getBufferView(bufferViewIndex)
		if err != nil {
			return nil, 0, err
		}
		count := glbInt(accessor, "count")
		bufferViewOffset := glbInt(bufferView, "byteOffset")
		accessorOffset := glbInt(accessor, "byteOffset")
		stride := glbInt(bufferView, "byteStride")
		if stride == 0 {
			stride = elementSize
		}
		start := bufferViewOffset + accessorOffset
		if start < 0 || start > len(binData) {
			return nil, 0, fmt.Errorf("accessor data out of bounds")
		}
		out := make([]byte, count*elementSize)
		for i := 0; i < count; i++ {
			src := start + i*stride
			if src+elementSize > len(binData) {
				return nil, 0, fmt.Errorf("accessor data out of bounds at element %d", i)
			}
			copy(out[i*elementSize:(i+1)*elementSize], binData[src:src+elementSize])
		}
		return out, count, nil
	}

	readIndices := func(accessor map[string]interface{}) ([]uint32, error) {
		componentKind := glbInt(accessor, "componentType")
		bytesPerIndex := 2
		switch componentKind {
		case 5121:
			bytesPerIndex = 1
		case 5123:
			bytesPerIndex = 2
		case 5125:
			bytesPerIndex = 4
		default:
			return nil, fmt.Errorf("unsupported index component type %d", componentKind)
		}
		idxBytes, count, err := readAccessorBytes(accessor, bytesPerIndex)
		if err != nil {
			return nil, err
		}
		indices := make([]uint32, count)
		for i := 0; i < count; i++ {
			switch componentKind {
			case 5121:
				indices[i] = uint32(idxBytes[i])
			case 5123:
				indices[i] = uint32(binary.LittleEndian.Uint16(idxBytes[i*2 : i*2+2]))
			case 5125:
				indices[i] = binary.LittleEndian.Uint32(idxBytes[i*4 : i*4+4])
			}
		}
		return indices, nil
	}

	posMin := [3]float32{float32(math.MaxFloat32), float32(math.MaxFloat32), float32(math.MaxFloat32)}
	posMax := [3]float32{-float32(math.MaxFloat32), -float32(math.MaxFloat32), -float32(math.MaxFloat32)}
	positionBytes := new(bytes.Buffer)
	indexBytes := new(bytes.Buffer)
	totalVertices := 0
	totalIndices := 0

	for _, meshRaw := range meshesRaw {
		mesh, ok := meshRaw.(map[string]interface{})
		if !ok {
			continue
		}
		primitivesRaw, _ := mesh["primitives"].([]interface{})
		for _, primitiveRaw := range primitivesRaw {
			prim, ok := primitiveRaw.(map[string]interface{})
			if !ok {
				continue
			}
			attrs, _ := prim["attributes"].(map[string]interface{})
			posAccessorIndexFloat, ok := attrs["POSITION"].(float64)
			if !ok {
				continue
			}
			posAccessor, err := getAccessor(int(posAccessorIndexFloat))
			if err != nil {
				continue
			}
			posBytes, vertexCount, err := readAccessorBytes(posAccessor, 12)
			if err != nil || vertexCount == 0 {
				continue
			}
			vertexBase := uint32(totalVertices)
			positionBytes.Write(posBytes)
			for vertexIndex := 0; vertexIndex < vertexCount; vertexIndex++ {
				x := math.Float32frombits(binary.LittleEndian.Uint32(posBytes[vertexIndex*12 : vertexIndex*12+4]))
				y := math.Float32frombits(binary.LittleEndian.Uint32(posBytes[vertexIndex*12+4 : vertexIndex*12+8]))
				z := math.Float32frombits(binary.LittleEndian.Uint32(posBytes[vertexIndex*12+8 : vertexIndex*12+12]))
				if x < posMin[0] {
					posMin[0] = x
				}
				if y < posMin[1] {
					posMin[1] = y
				}
				if z < posMin[2] {
					posMin[2] = z
				}
				if x > posMax[0] {
					posMax[0] = x
				}
				if y > posMax[1] {
					posMax[1] = y
				}
				if z > posMax[2] {
					posMax[2] = z
				}
			}
			var indices []uint32
			if indicesValue, ok := prim["indices"]; ok {
				indexAccessor, err := getAccessor(int(indicesValue.(float64)))
				if err != nil {
					continue
				}
				indices, err = readIndices(indexAccessor)
				if err != nil || len(indices) == 0 {
					continue
				}
			} else {
				triangleVertexCount := vertexCount - (vertexCount % 3)
				if triangleVertexCount == 0 {
					continue
				}
				indices = make([]uint32, triangleVertexCount)
				for i := 0; i < triangleVertexCount; i++ {
					indices[i] = uint32(i)
				}
			}
			for _, index := range indices {
				binary.Write(indexBytes, binary.LittleEndian, vertexBase+index)
			}
			totalVertices += vertexCount
			totalIndices += len(indices)
		}
	}

	if totalVertices == 0 || totalIndices == 0 {
		return nil, fmt.Errorf("no triangle mesh primitives in GLB")
	}

	return &exportMeshData{
		positionBytes: positionBytes.Bytes(),
		indexBytes:    indexBytes.Bytes(),
		vertexCount:   totalVertices,
		indexCount:    totalIndices,
		posMin:        posMin,
		posMax:        posMax,
		indexCompKind: 5125,
	}, nil
}

// 🧹exportFindRepresentationForKind finds the best matching representation for a type given tag filters.
func exportFindRepresentationForKind(typ *Type, tags []string, tagsDict map[string]*Tag) *Representation {
	if len(typ.Representations) == 0 {
		return nil
	}
	if len(tags) == 0 {
		for i := range typ.Representations {
			if len(typ.Representations[i].Tags) == 0 {
				return &typ.Representations[i]
			}
		}
		return &typ.Representations[0]
	}
	selectedTagIds := make(map[string]bool)
	for _, t := range tags {
		if _, ok := tagsDict[t]; ok {
			selectedTagIds[t] = true
			continue
		}
		for _, tag := range tagsDict {
			if tag.Name == t {
				selectedTagIds[tag.Id] = true
			}
		}
	}
	bestRepresentation := (*Representation)(nil)
	bestScore := -1.0
	for i := range typ.Representations {
		representation := &typ.Representations[i]
		representationTagIds := make(map[string]bool)
		for _, tid := range representation.Tags {
			representationTagIds[tid.Id] = true
		}
		containsAll := true
		intersection := 0
		for id := range selectedTagIds {
			if !representationTagIds[id] {
				containsAll = false
				break
			}
			intersection++
		}
		if !containsAll {
			continue
		}
		union := len(selectedTagIds)
		for id := range representationTagIds {
			if !selectedTagIds[id] {
				union++
			}
		}
		score := 0.0
		if union > 0 {
			score = float64(intersection) / float64(union)
		}
		if score > bestScore {
			bestScore = score
			bestRepresentation = representation
		}
	}
	if bestRepresentation != nil {
		return bestRepresentation
	}
	return &typ.Representations[0]
}

// #endregion 🔧Kit Representation Export Helpers

// 📐ExportDesignRepresentation exports the 3D representation of a design to GLB or glTF format.
func ExportDesignRepresentation(kit *Kit, designId string, format string, tags []string, options map[string]interface{}) ([]byte, error) {
	if _, ok := ExportRepresentationFormats[format]; !ok {
		return nil, fmt.Errorf("unsupported format: %s", format)
	}

	design := FindDesignInKit(kit, designId)
	if design == nil {
		return nil, fmt.Errorf("design not found: %s", designId)
	}
	if len(design.Pieces) == 0 {
		return nil, fmt.Errorf("design has no pieces")
	}

	typesDict := make(map[string]*Type)
	for i := range kit.Types {
		typesDict[kit.Types[i].Id] = &kit.Types[i]
	}
	filesDict := make(map[string]*File)
	for i := range kit.Files {
		filesDict[kit.Files[i].Id] = &kit.Files[i]
	}
	tagsDict := make(map[string]*Tag)
	for i := range kit.Tags {
		tagsDict[kit.Tags[i].Id] = &kit.Tags[i]
	}
	pieceMap := make(map[string]*Piece)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Id] = &design.Pieces[i]
	}

	// #region 🌦️Kit Representation Export BFS
	piecePlanes := make(map[string]*Plane)
	parentOf := make(map[string]string)
	childrenOf := make(map[string][]string)
	var rootPieceIds []string

	adjacency := make(map[string][]struct {
		neighborId string
		connection   *Connection
	})
	for i := range design.Connections {
		conn := &design.Connections[i]
		srcId := conn.Parent.Piece.Id
		tgtId := conn.Child.Piece.Id
		if pieceMap[srcId] == nil || pieceMap[tgtId] == nil {
			continue
		}
		adjacency[srcId] = append(adjacency[srcId], struct {
			neighborId string
			connection   *Connection
		}{tgtId, conn})
		adjacency[tgtId] = append(adjacency[tgtId], struct {
			neighborId string
			connection   *Connection
		}{srcId, conn})
	}

	visited := make(map[string]bool)
	var bfsExport func(rootId string)
	bfsExport = func(rootId string) {
		queue := []string{rootId}
		visited[rootId] = true
		rootPieceIds = append(rootPieceIds, rootId)
		rootPiece := pieceMap[rootId]
		if rootPiece.Plane != nil && rootPiece.Center != nil {
			piecePlanes[rootId] = rootPiece.Plane
		} else {
			p := Plane{
				Origin: Point{X: 0, Y: 0, Z: 0},
				XAxis:  Vector{X: 1, Y: 0, Z: 0},
				YAxis:  Vector{X: 0, Y: 1, Z: 0},
			}
			piecePlanes[rootId] = &p
		}
		for len(queue) > 0 {
			currentId := queue[0]
			queue = queue[1:]
			currentPlane := piecePlanes[currentId]
			currentPiece := pieceMap[currentId]

			for _, neighbor := range adjacency[currentId] {
				if visited[neighbor.neighborId] {
					continue
				}
				visited[neighbor.neighborId] = true
				neighborPiece := pieceMap[neighbor.neighborId]
				conn := neighbor.connection

				var parentSide, childSide *Side
				if conn.Parent.Piece.Id == currentId {
					parentSide = &conn.Parent
					childSide = &conn.Child
				} else {
					parentSide = &conn.Child
					childSide = &conn.Parent
				}

				var parentType, childType *Type
				if currentPiece.Type != nil {
					parentType = typesDict[currentPiece.Type.Id]
				}
				if neighborPiece.Type != nil {
					childType = typesDict[neighborPiece.Type.Id]
				}

				var parentConnectorId, childConnectorId *string
				if parentSide.Connector != nil {
					parentConnectorId = &parentSide.Connector.Id
				}
				if childSide.Connector != nil {
					childConnectorId = &childSide.Connector.Id
				}

				parentConnector := getConnector(typesDict, parentType, parentConnectorId)
				childConnector := getConnector(typesDict, childType, childConnectorId)
				if parentConnector == nil || childConnector == nil {
					continue
				}

				childPlane := computeChildPlane(*currentPlane, *parentConnector, *childConnector, *conn)
				piecePlanes[neighbor.neighborId] = &childPlane
				parentOf[neighbor.neighborId] = currentId
				childrenOf[currentId] = append(childrenOf[currentId], neighbor.neighborId)

				queue = append(queue, neighbor.neighborId)
			}
		}
	}
	for _, piece := range design.Pieces {
		if !visited[piece.Id] {
			bfsExport(piece.Id)
		}
	}
	// #endregion 🌦️Kit Representation Export BFS

	// #region ⚙️Kit Representation Export MeshData
	usedTypes := make(map[string]bool)
	for _, piece := range design.Pieces {
		if piece.Type != nil {
			usedTypes[piece.Type.Id] = true
		}
	}
	typeMeshData := make(map[string]*exportMeshData)
	typeMeshNames := make(map[string]string)
	for typeId := range usedTypes {
		typ := typesDict[typeId]
		if typ == nil {
			continue
		}
		representation := exportFindRepresentationForKind(typ, tags, tagsDict)
		if representation == nil {
			continue
		}
		file := filesDict[representation.File.Id]
		if file == nil || file.Blob == nil || *file.Blob == "" {
			continue
		}
		typeMeshNames[typeId] = file.Name
		glbData, err := exportDecodeBlobToBytes(*file.Blob)
		if err != nil || len(glbData) < 4 {
			continue
		}
		if binary.LittleEndian.Uint32(glbData[0:4]) != 0x46546C67 {
			continue
		}
		meshData, err := exportParseGLBMesh(glbData)
		if err != nil {
			continue
		}
		typeMeshData[typeId] = meshData
	}
	// #endregion ⚙️Kit Representation Export MeshData

	// #region 💻Kit Representation Export BuildGLTF
	typeOrder := make([]string, 0, len(usedTypes))
	for typeId := range typeMeshData {
		typeOrder = append(typeOrder, typeId)
	}
	sort.Strings(typeOrder)
	typeMeshIndex := make(map[string]int)
	for i, typeId := range typeOrder {
		typeMeshIndex[typeId] = i
	}

	var binBuf bytes.Buffer
	type exportBufView struct {
		byteOffset int
		byteLength int
		target     int
	}
	var bufViews []exportBufView
	type exportAccessor struct {
		bufferView    int
		componentKind int
		count         int
		accessorKind  string
		min           []float32
		max           []float32
	}
	var accs []exportAccessor
	type exportMesh struct {
		positionAcc int
		indexAcc    int
		hasIndices  bool
		name        string
	}
	var gltfMeshList []exportMesh

	for _, typeId := range typeOrder {
		md := typeMeshData[typeId]

		for binBuf.Len()%4 != 0 {
			binBuf.WriteByte(0)
		}
		posOffset := binBuf.Len()
		binBuf.Write(md.positionBytes)
		posBVIdx := len(bufViews)
		bufViews = append(bufViews, exportBufView{
			byteOffset: posOffset,
			byteLength: len(md.positionBytes),
			target:     34962,
		})
		posAccIdx := len(accs)
		accs = append(accs, exportAccessor{
			bufferView:    posBVIdx,
			componentKind: 5126,
			count:         md.vertexCount,
			accessorKind:  "VEC3",
			min:           md.posMin[:],
			max:           md.posMax[:],
		})

		mi := exportMesh{positionAcc: posAccIdx}
		if meshName, ok := typeMeshNames[typeId]; ok {
			mi.name = meshName
		}
		if md.indexCount > 0 {
			for binBuf.Len()%4 != 0 {
				binBuf.WriteByte(0)
			}
			idxOffset := binBuf.Len()
			binBuf.Write(md.indexBytes)
			idxBVIdx := len(bufViews)
			bufViews = append(bufViews, exportBufView{
				byteOffset: idxOffset,
				byteLength: len(md.indexBytes),
				target:     34963,
			})
			idxAccIdx := len(accs)
			accs = append(accs, exportAccessor{
				bufferView:    idxBVIdx,
				componentKind: md.indexCompKind,
				count:         md.indexCount,
				accessorKind:  "SCALAR",
			})
			mi.indexAcc = idxAccIdx
			mi.hasIndices = true
		}
		gltfMeshList = append(gltfMeshList, mi)
	}
	for binBuf.Len()%4 != 0 {
		binBuf.WriteByte(0)
	}

	pieceNodeIndex := make(map[string]int)
	for i, piece := range design.Pieces {
		pieceNodeIndex[piece.Id] = i
	}

	type exportNode struct {
		meshIndex int
		matrix    [16]float64
		children  []int
		name      string
	}
	nodes := make([]exportNode, len(design.Pieces))
	for i, piece := range design.Pieces {
		plane := piecePlanes[piece.Id]
		if plane == nil {
			p := Plane{
				Origin: Point{X: 0, Y: 0, Z: 0},
				XAxis:  Vector{X: 1, Y: 0, Z: 0},
				YAxis:  Vector{X: 0, Y: 1, Z: 0},
			}
			plane = &p
		}

		var matrix [16]float64
		if parentId, hasParent := parentOf[piece.Id]; hasParent {
			parentPlane := piecePlanes[parentId]
			if parentPlane != nil {
				parentMat := planeToMatrix(*parentPlane)
				childMat := planeToMatrix(*plane)
				var inv mat.Dense
				if err := inv.Inverse(parentMat); err == nil {
					var relative mat.Dense
					relative.Mul(&inv, childMat)
					matrix = exportDenseToGltfMatrix(&relative)
				} else {
					matrix = exportPlaneToGltfMatrix(*plane)
				}
			} else {
				matrix = exportPlaneToGltfMatrix(*plane)
			}
		} else {
			matrix = exportPlaneToGltfMatrix(*plane)
		}

		meshIdx := -1
		if piece.Type != nil {
			if idx, ok := typeMeshIndex[piece.Type.Id]; ok {
				meshIdx = idx
			}
		}

		name := piece.Id
		if piece.Name != nil && *piece.Name != "" {
			name = *piece.Name
		}

		var childIndices []int
		for _, childId := range childrenOf[piece.Id] {
			if idx, ok := pieceNodeIndex[childId]; ok {
				childIndices = append(childIndices, idx)
			}
		}

		nodes[i] = exportNode{
			meshIndex: meshIdx,
			matrix:    matrix,
			children:  childIndices,
			name:      name,
		}
	}

	var sceneRootNodes []int
	for _, rootId := range rootPieceIds {
		if idx, ok := pieceNodeIndex[rootId]; ok {
			sceneRootNodes = append(sceneRootNodes, idx)
		}
	}

	gltfNodes := make([]interface{}, len(nodes))
	for i, n := range nodes {
		node := map[string]interface{}{
			"name":   n.name,
			"matrix": n.matrix,
		}
		if n.meshIndex >= 0 {
			node["mesh"] = n.meshIndex
		}
		if len(n.children) > 0 {
			node["children"] = n.children
		}
		gltfNodes[i] = node
	}

	gltfBufViews := make([]interface{}, len(bufViews))
	for i, bv := range bufViews {
		gltfBufViews[i] = map[string]interface{}{
			"buffer":     0,
			"byteOffset": bv.byteOffset,
			"byteLength": bv.byteLength,
			"target":     bv.target,
		}
	}

	gltfAccs := make([]interface{}, len(accs))
	for i, acc := range accs {
		a := map[string]interface{}{
			"bufferView":    acc.bufferView,
			"componentType": acc.componentKind,
			"count":         acc.count,
			"type":          acc.accessorKind,
		}
		if acc.min != nil {
			a["min"] = acc.min
		}
		if acc.max != nil {
			a["max"] = acc.max
		}
		gltfAccs[i] = a
	}

	gltfMeshes := make([]interface{}, len(gltfMeshList))
	for i, m := range gltfMeshList {
		primAttrs := map[string]interface{}{
			"POSITION": m.positionAcc,
		}
		prim := map[string]interface{}{
			"attributes": primAttrs,
		}
		if m.hasIndices {
			prim["indices"] = m.indexAcc
		}
		gltfMeshes[i] = map[string]interface{}{
			"name":       m.name,
			"primitives": []interface{}{prim},
		}
	}

	gltfDoc := map[string]interface{}{
		"asset":       map[string]interface{}{"version": "2.0", "generator": "compose"},
		"scene":       0,
		"scenes":      []interface{}{map[string]interface{}{"nodes": sceneRootNodes}},
		"nodes":       gltfNodes,
		"meshes":      gltfMeshes,
		"accessors":   gltfAccs,
		"bufferViews": gltfBufViews,
	}

	binBytes := binBuf.Bytes()

	if format == ".gltf" {
		gltfDoc["buffers"] = []interface{}{map[string]interface{}{
			"byteLength": len(binBytes),
			"uri":        "data:application/octet-stream;base64," + base64.StdEncoding.EncodeToString(binBytes),
		}}
		return json.Marshal(gltfDoc)
	}

	gltfDoc["buffers"] = []interface{}{map[string]interface{}{
		"byteLength": len(binBytes),
	}}
	jsonBytes, err := json.Marshal(gltfDoc)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal glTF JSON: %w", err)
	}
	for len(jsonBytes)%4 != 0 {
		jsonBytes = append(jsonBytes, ' ')
	}
	for len(binBytes)%4 != 0 {
		binBytes = append(binBytes, 0)
	}

	totalLength := 12 + 8 + len(jsonBytes) + 8 + len(binBytes)
	out := new(bytes.Buffer)
	out.Grow(totalLength)

	binary.Write(out, binary.LittleEndian, uint32(0x46546C67))
	binary.Write(out, binary.LittleEndian, uint32(2))
	binary.Write(out, binary.LittleEndian, uint32(totalLength))

	binary.Write(out, binary.LittleEndian, uint32(len(jsonBytes)))
	binary.Write(out, binary.LittleEndian, uint32(0x4E4F534A))
	out.Write(jsonBytes)

	binary.Write(out, binary.LittleEndian, uint32(len(binBytes)))
	binary.Write(out, binary.LittleEndian, uint32(0x004E4942))
	out.Write(binBytes)

	return out.Bytes(), nil
	// #endregion 💻Kit Representation Export BuildGLTF
}

// #endregion 🔩Kit Representation Export

// #region ❄️Geometric Insights
// Key performance indicators for GLB/GLTF representation geometry. Representation MUST be glb/gltf.

// 📏GeometricInsights holds computed geometric KPIs for a GLB/GLTF representation in compose coordinate system (compose x=glb x, compose y=-glb x, compose z=glb y).
type GeometricInsights struct {
	BoundingBoxMin      Point
	BoundingBoxMax      Point
	DimensionX          float64
	DimensionY          float64
	DimensionZ          float64
	CharacteristicLen   float64
	FootprintArea       float64
	TotalSurfaceArea    float64
	EnclosedVolume      float64
	SurfaceToVolume     float64
	AspectRatioXY       float64
	AspectRatioXZ       float64
	AspectRatioYZ       float64
	Slenderness         float64
	Centroid            Point
	VertexCount         int
	FaceCount           int
	EulerCharacteristic int
}

func geometricInsightsFromMeshData(md *exportMeshData) GeometricInsights {
	out := GeometricInsights{}
	if md == nil || md.vertexCount == 0 {
		return out
	}
	pos := md.positionBytes
	idx := md.indexBytes
	// Compose coordinates: x = glb.x, y = -glb.x, z = glb.y
	sxMin, syMin, szMin := math.MaxFloat64, math.MaxFloat64, math.MaxFloat64
	sxMax, syMax, szMax := -math.MaxFloat64, -math.MaxFloat64, -math.MaxFloat64
	var sumSx, sumSy, sumSz float64
	for i := 0; i < md.vertexCount; i++ {
		xg := math.Float32frombits(binary.LittleEndian.Uint32(pos[i*12 : i*12+4]))
		yg := math.Float32frombits(binary.LittleEndian.Uint32(pos[i*12+4 : i*12+8]))
		_ = math.Float32frombits(binary.LittleEndian.Uint32(pos[i*12+8 : i*12+12]))
		sx, sy, sz := float64(xg), float64(-xg), float64(yg)
		if sx < sxMin {
			sxMin = sx
		}
		if sx > sxMax {
			sxMax = sx
		}
		if sy < syMin {
			syMin = sy
		}
		if sy > syMax {
			syMax = sy
		}
		if sz < szMin {
			szMin = sz
		}
		if sz > szMax {
			szMax = sz
		}
		sumSx += sx
		sumSy += sy
		sumSz += sz
	}
	out.BoundingBoxMin = Point{X: sxMin, Y: syMin, Z: szMin}
	out.BoundingBoxMax = Point{X: sxMax, Y: syMax, Z: szMax}
	out.DimensionX = sxMax - sxMin
	out.DimensionY = syMax - syMin
	out.DimensionZ = szMax - szMin
	volBox := out.DimensionX * out.DimensionY * out.DimensionZ
	if volBox > 0 {
		out.CharacteristicLen = math.Cbrt(volBox)
	}
	out.FootprintArea = out.DimensionX * out.DimensionZ
	out.VertexCount = md.vertexCount
	out.FaceCount = md.indexCount / 3
	n := float64(md.vertexCount)
	out.Centroid = Point{X: sumSx / n, Y: sumSy / n, Z: sumSz / n}
	var area float64
	var volume float64
	for i := 0; i+2 < md.indexCount; i += 3 {
		if len(idx) < (i+3)*4 {
			break
		}
		i0 := binary.LittleEndian.Uint32(idx[i*4 : i*4+4])
		i1 := binary.LittleEndian.Uint32(idx[(i+1)*4 : (i+1)*4+4])
		i2 := binary.LittleEndian.Uint32(idx[(i+2)*4 : (i+2)*4+4])
		ax := math.Float32frombits(binary.LittleEndian.Uint32(pos[i0*12 : i0*12+4]))
		ay := math.Float32frombits(binary.LittleEndian.Uint32(pos[i0*12+4 : i0*12+8]))
		az := math.Float32frombits(binary.LittleEndian.Uint32(pos[i0*12+8 : i0*12+12]))
		bx := math.Float32frombits(binary.LittleEndian.Uint32(pos[i1*12 : i1*12+4]))
		by := math.Float32frombits(binary.LittleEndian.Uint32(pos[i1*12+4 : i1*12+8]))
		bz := math.Float32frombits(binary.LittleEndian.Uint32(pos[i1*12+8 : i1*12+12]))
		cx := math.Float32frombits(binary.LittleEndian.Uint32(pos[i2*12 : i2*12+4]))
		cy := math.Float32frombits(binary.LittleEndian.Uint32(pos[i2*12+4 : i2*12+8]))
		cz := math.Float32frombits(binary.LittleEndian.Uint32(pos[i2*12+8 : i2*12+12]))
		abx := float64(bx - ax)
		aby := float64(by - ay)
		abz := float64(bz - az)
		acx := float64(cx - ax)
		acy := float64(cy - ay)
		acz := float64(cz - az)
		crossX := aby*acz - abz*acy
		crossY := abz*acx - abx*acz
		crossZ := abx*acy - aby*acx
		area += 0.5 * math.Sqrt(crossX*crossX+crossY*crossY+crossZ*crossZ)
		volume += (1.0 / 6.0) * (float64(ax)*(float64(by)*float64(cz)-float64(bz)*float64(cy)) +
			float64(ay)*(float64(bz)*float64(cx)-float64(bx)*float64(cz)) +
			float64(az)*(float64(bx)*float64(cy)-float64(by)*float64(cx)))
	}
	out.TotalSurfaceArea = area
	volume = math.Abs(volume)
	out.EnclosedVolume = volume
	if volume > 1e-20 && area > 0 {
		out.SurfaceToVolume = area / volume
	}
	if out.DimensionY > 1e-10 && out.DimensionX > 1e-10 {
		out.AspectRatioXY = out.DimensionX / out.DimensionY
	}
	if out.DimensionZ > 1e-10 && out.DimensionX > 1e-10 {
		out.AspectRatioXZ = out.DimensionX / out.DimensionZ
	}
	if out.DimensionZ > 1e-10 && out.DimensionY > 1e-10 {
		out.AspectRatioYZ = out.DimensionY / out.DimensionZ
	}
	maxExt := out.DimensionX
	if out.DimensionY > maxExt {
		maxExt = out.DimensionY
	}
	if out.DimensionZ > maxExt {
		maxExt = out.DimensionZ
	}
	if maxExt > 1e-10 && area > 0 {
		out.Slenderness = maxExt / math.Cbrt(area*maxExt)
	}
	out.EulerCharacteristic = out.VertexCount - (3*out.FaceCount)/2 + out.FaceCount
	return out
}

// 📏GetGeometricInsightsForRepresentation computes key performance indicators for the geometry of a GLB/GLTF representation.
func GetGeometricInsightsForRepresentation(representation interface{}) (GeometricInsights, error) {
	var md *exportMeshData
	var err error
	switch v := representation.(type) {
	case string:
		data, errRead := os.ReadFile(v)
		if errRead != nil {
			return GeometricInsights{}, fmt.Errorf("read representation file: %w", errRead)
		}
		lower := strings.ToLower(v)
		if strings.HasSuffix(lower, ".glb") {
			md, err = exportParseGLBMesh(data)
		} else if strings.HasSuffix(lower, ".gltf") {
			var gltf map[string]interface{}
			if errJSON := json.Unmarshal(data, &gltf); errJSON != nil {
				return GeometricInsights{}, fmt.Errorf("parse glTF JSON: %w", errJSON)
			}
			buffersRaw, _ := gltf["buffers"].([]interface{})
			var binData []byte
			if len(buffersRaw) > 0 {
				if buf, ok := buffersRaw[0].(map[string]interface{}); ok {
					if uri, _ := buf["uri"].(string); uri != "" {
						if strings.HasPrefix(uri, "data:") {
							idx := strings.Index(uri, ",")
							if idx >= 0 {
								b64 := strings.Map(func(r rune) rune {
									if r == ' ' || r == '\n' || r == '\r' || r == '\t' {
										return -1
									}
									return r
								}, uri[idx+1:])
								binData, _ = base64.StdEncoding.DecodeString(b64)
							}
						} else {
							dir := filepath.Dir(v)
							binPath := filepath.Join(dir, uri)
							binData, _ = os.ReadFile(binPath)
						}
					}
				}
			}
			md, err = exportParseGltfToMeshData(gltf, binData)
		} else {
			return GeometricInsights{}, fmt.Errorf("representation MUST be .glb or .gltf, got %s", v)
		}
	case []byte:
		if len(v) >= 4 && binary.LittleEndian.Uint32(v[0:4]) == 0x46546C67 {
			md, err = exportParseGLBMesh(v)
		} else {
			var gltf map[string]interface{}
			if errJSON := json.Unmarshal(v, &gltf); errJSON != nil {
				return GeometricInsights{}, fmt.Errorf("parse glTF JSON: %w", errJSON)
			}
			buffersRaw, _ := gltf["buffers"].([]interface{})
			var binData []byte
			if len(buffersRaw) > 0 {
				if buf, ok := buffersRaw[0].(map[string]interface{}); ok {
					if uri, _ := buf["uri"].(string); uri != "" && strings.HasPrefix(uri, "data:") {
						idx := strings.Index(uri, ",")
						if idx >= 0 {
							b64 := strings.Map(func(r rune) rune {
								if r == ' ' || r == '\n' || r == '\r' || r == '\t' {
									return -1
								}
								return r
							}, uri[idx+1:])
							binData, _ = base64.StdEncoding.DecodeString(b64)
						}
					}
				}
			}
			md, err = exportParseGltfToMeshData(gltf, binData)
		}
	default:
		return GeometricInsights{}, fmt.Errorf("representation must be string path or []byte, got %T", representation)
	}
	if err != nil {
		return GeometricInsights{}, err
	}
	return geometricInsightsFromMeshData(md), nil
}

// #endregion ❄️Geometric Insights

// #region 📡SQLite
// SQLite kit operations. MUST provide serialization and deserialization of Kit to and from SQLite and zip formats.

// ComposeKitSqliteSchemaVersion matches [`compose::io::sqlite::SCHEMA_VERSION`] in `compose/rs/lib.rs`.
const ComposeKitSqliteSchemaVersion = "2026-04-23-kit-vcs-sqlite-roundtrip"

// ComposeKitSqliteSchemaEngine matches [`compose::io::sqlite::SCHEMA_ENGINE`] in `compose/rs/lib.rs`.
const ComposeKitSqliteSchemaEngine = "compose-rs"

// 🗄️KitFromSqlite reads a Kit from a SQLite database file (normalized `compose/sqlite/🛢️schema.sql`).
func KitFromSqlite(dbPath string) (*Kit, error) {
	db, err := sql.Open("sqlite", dbPath)
	if err != nil {
		return nil, err
	}
	defer db.Close()

	kit := &Kit{}

	row := db.QueryRow(`SELECT id, name, description, icon, image, preview, remote, homepage, license, uri, created_at, updated_at FROM kit LIMIT 1`)
	var description, icon, image, preview, remote, homepage, license, uri, createdAt, updatedAt sql.NullString
	if err := row.Scan(&kit.Id, &kit.Name, &description, &icon, &image, &preview, &remote, &homepage, &license, &uri, &createdAt, &updatedAt); err != nil {
		return nil, fmt.Errorf("failed to scan kit: %w", err)
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
	if createdAt.Valid {
		kit.CreatedAt = createdAt.String
	}
	if updatedAt.Valid {
		kit.UpdatedAt = updatedAt.String
	}
	_ = uri

	families, err := loadFamilies(db, kit.Id)
	if err != nil {
		return nil, err
	}
	kit.Families = families

	typologies, err := loadTypologies(db, kit.Id)
	if err != nil {
		return nil, err
	}
	kit.Typologies = typologies
	KitFlattenTypesDesigns(kit)

	return kit, nil
}

// 👪loadFamilies loads kit-level families and their ports.
func loadFamilies(db *sql.DB, kitId string) ([]Family, error) {
	rows, err := db.Query(`SELECT id, name, description, icon FROM family WHERE kit_id = ? ORDER BY ordinal`, kitId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var out []Family
	for rows.Next() {
		var f Family
		var description, icon sql.NullString
		if err := rows.Scan(&f.Id, &f.Name, &description, &icon); err != nil {
			return nil, err
		}
		if description.Valid {
			f.Description = &description.String
		}
		if icon.Valid {
			f.Icon = &icon.String
		}
		ports, err := loadPortsForFamily(db, kitId, f.Id)
		if err != nil {
			return nil, err
		}
		f.Ports = ports
		out = append(out, f)
	}
	return out, nil
}

func loadPortsForFamily(db *sql.DB, kitId, familyId string) ([]Port, error) {
	prows, err := db.Query(`SELECT id, name, icon, mandatory, t, description,
		point_x, point_y, point_z, direction_x, direction_y, direction_z
		FROM port WHERE kit_id = ? AND parent_family_id = ? ORDER BY ordinal`, kitId, familyId)
	if err != nil {
		return nil, err
	}
	defer prows.Close()

	var ports []Port
	for prows.Next() {
		var p Port
		var icon, description sql.NullString
		var mandatory sql.NullInt64
		var t sql.NullFloat64
		var px, py, pz, dx, dy, dz sql.NullFloat64
		if err := prows.Scan(&p.Id, &p.Name, &icon, &mandatory, &t, &description,
			&px, &py, &pz, &dx, &dy, &dz); err != nil {
			return nil, err
		}
		_ = mandatory
		_ = t
		_ = px
		_ = py
		_ = pz
		_ = dx
		_ = dy
		_ = dz
		if icon.Valid {
			p.Icon = &icon.String
		}
		if description.Valid {
			p.Description = &description.String
		}
		cprows, err := db.Query(`SELECT compatible_port_id FROM port_compatible_port WHERE port_id = ? ORDER BY ordinal`, p.Id)
		if err != nil {
			return nil, err
		}
		for cprows.Next() {
			var cpid string
			if err := cprows.Scan(&cpid); err != nil {
				cprows.Close()
				return nil, err
			}
			p.CompatiblePorts = append(p.CompatiblePorts, PortId{Id: cpid})
		}
		cprows.Close()
		ports = append(ports, p)
	}
	return ports, nil
}

// 🏛️loadTypologies loads typologies and nested types and designs for a kit.
func loadTypologies(db *sql.DB, kitId string) ([]Typology, error) {
	rows, err := db.Query(`SELECT id, name, description, icon, folder_id FROM typology WHERE kit_id = ? ORDER BY ordinal`, kitId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var out []Typology
	for rows.Next() {
		var topo Typology
		var description, icon, folderID sql.NullString
		if err := rows.Scan(&topo.Id, &topo.Name, &description, &icon, &folderID); err != nil {
			return nil, err
		}
		if description.Valid {
			topo.Description = &description.String
		}
		if icon.Valid {
			topo.Icon = &icon.String
		}
		if folderID.Valid {
			topo.Folder = &folderID.String
		}
		types, err := loadTypes(db, topo.Id)
		if err != nil {
			return nil, err
		}
		topo.Types = types
		designs, err := loadDesigns(db, topo.Id, types)
		if err != nil {
			return nil, err
		}
		topo.Designs = designs
		out = append(out, topo)
	}
	return out, nil
}

// 🏷️loadTypes loads all types belonging to a typology from the database
func loadTypes(db *sql.DB, typologyId string) ([]Type, error) {
	rows, err := db.Query(`SELECT id, name, description, icon, image, stock, virtual, unit, location_id, created_at, updated_at
		FROM type WHERE typology_id = ? ORDER BY ordinal`, typologyId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var types []Type
	for rows.Next() {
		var t Type
		var description, icon, image, unit, locationID, createdAt, updatedAt sql.NullString
		var stock, virtual sql.NullInt64
		if err := rows.Scan(&t.Id, &t.Name, &description, &icon, &image, &stock, &virtual, &unit, &locationID, &createdAt, &updatedAt); err != nil {
			return nil, err
		}
		if stock.Valid {
			s := int(stock.Int64)
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
		if virtual.Valid {
			v := virtual.Int64 != 0
			t.Virtual = &v
		}
		if locationID.Valid {
			t.Location = &LocationId{Id: locationID.String}
		}
		t.CreatedAt = createdAt.String
		t.UpdatedAt = updatedAt.String
		t.Typology = TypologyId{Id: typologyId}

		tfr, err := db.Query(`SELECT family_id FROM type_family WHERE type_id = ? ORDER BY ordinal`, t.Id)
		if err != nil {
			return nil, err
		}
		for tfr.Next() {
			var fid string
			if err := tfr.Scan(&fid); err != nil {
				tfr.Close()
				return nil, err
			}
			t.Families = append(t.Families, FamilyId{Id: fid})
		}
		tfr.Close()

		connectors, err := loadConnectors(db, t.Id)
		if err != nil {
			return nil, err
		}
		t.Connectors = connectors

		types = append(types, t)
	}
	return types, nil
}

// ➕loadDesigns loads all designs belonging to a typology from the database
func loadDesigns(db *sql.DB, typologyId string, types []Type) ([]Design, error) {
	rows, err := db.Query(`SELECT id, name, description, icon, image, location_id, unit, created_at, updated_at
        FROM design WHERE typology_id = ? ORDER BY ordinal`, typologyId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var designs []Design
	for rows.Next() {
		var d Design
		var description, icon, image, locationID, unit, createdAt, updatedAt sql.NullString
		if err := rows.Scan(&d.Id, &d.Name, &description, &icon, &image, &locationID, &unit, &createdAt, &updatedAt); err != nil {
			return nil, err
		}
		if unit.Valid {
			d.Unit = &unit.String
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
		if locationID.Valid {
			d.Location = &LocationId{Id: locationID.String}
		}
		d.CreatedAt = createdAt.String
		d.UpdatedAt = updatedAt.String
		d.Typology = TypologyId{Id: typologyId}

		dfr, err := db.Query(`SELECT family_id FROM design_family WHERE design_id = ? ORDER BY ordinal`, d.Id)
		if err != nil {
			return nil, err
		}
		for dfr.Next() {
			var fid string
			if err := dfr.Scan(&fid); err != nil {
				dfr.Close()
				return nil, err
			}
			d.Families = append(d.Families, FamilyId{Id: fid})
		}
		dfr.Close()

		pieces, err := loadPieces(db, d.Id)
		if err != nil {
			return nil, err
		}
		d.Pieces = pieces

		connections, err := loadConnections(db, d.Id, pieces, types)
		if err != nil {
			return nil, err
		}
		d.Connections = connections

		designs = append(designs, d)
	}
	return designs, nil
}

// 🧩loadPieces loads all pieces belonging to a design from the database
func loadPieces(db *sql.DB, designId string) ([]Piece, error) {
	rows, err := db.Query(`SELECT id, name, description,
        plane_origin_x, plane_origin_y, plane_origin_z,
        plane_x_axis_x, plane_x_axis_y, plane_x_axis_z,
        plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
        center_x, center_y, center_z,
        scale, hidden, locked, color, type_id, design_ref_id, design_id
        FROM piece WHERE design_id = ? ORDER BY ordinal`, designId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var pieces []Piece
	for rows.Next() {
		var p Piece
		var name, typeId, designIdRef, color, description sql.NullString
		var originX, originY, originZ, xAxisX, xAxisY, xAxisZ, yAxisX, yAxisY, yAxisZ sql.NullFloat64
		var centerX, centerY, centerZ, scale sql.NullFloat64
		var hidden, locked sql.NullInt64
		var designID string
		if err := rows.Scan(&p.Id, &name, &description,
			&originX, &originY, &originZ, &xAxisX, &xAxisY, &xAxisZ, &yAxisX, &yAxisY, &yAxisZ,
			&centerX, &centerY, &centerZ,
			&scale, &hidden, &locked, &color, &typeId, &designIdRef, &designID); err != nil {
			return nil, err
		}
		_ = designID
		if name.Valid {
			p.Name = &name.String
		}
		if typeId.Valid {
			p.Type = &TypeId{Id: typeId.String}
		}
		if designIdRef.Valid {
			p.Design = &DesignId{Id: designIdRef.String}
		}
		if originX.Valid && originY.Valid && originZ.Valid &&
			xAxisX.Valid && xAxisY.Valid && xAxisZ.Valid &&
			yAxisX.Valid && yAxisY.Valid && yAxisZ.Valid {
			p.Plane = &Plane{
				Origin: Point{X: originX.Float64, Y: originY.Float64, Z: originZ.Float64},
				XAxis:  Vector{X: xAxisX.Float64, Y: xAxisY.Float64, Z: xAxisZ.Float64},
				YAxis:  Vector{X: yAxisX.Float64, Y: yAxisY.Float64, Z: yAxisZ.Float64},
			}
		}
		if centerX.Valid && centerY.Valid {
			p.Center = &Coordinate{U: centerX.Float64, V: centerY.Float64}
		}
		if scale.Valid {
			p.Scale = &scale.Float64
		}
		if hidden.Valid {
			h := hidden.Int64 != 0
			p.IsHidden = &h
		}
		if locked.Valid {
			l := locked.Int64 != 0
			p.IsLocked = &l
		}
		if color.Valid {
			p.Color = &color.String
		}
		if description.Valid {
			p.Description = &description.String
		}
		_ = centerZ
		pieces = append(pieces, p)
	}
	return pieces, nil
}

// 🔌loadConnections loads all connections belonging to a design from the database.
func loadConnections(db *sql.DB, designId string, pieces []Piece, types []Type) ([]Connection, error) {
	pieceType := make(map[string]string)
	for _, p := range pieces {
		if p.Type != nil {
			pieceType[p.Id] = p.Type.Id
		}
	}
	rows, err := db.Query(`SELECT id,
		parent_side_id, parent_piece_id, parent_port_id, parent_design_piece_id,
		child_side_id, child_piece_id, child_port_id, child_design_piece_id,
		gap, shift, rise, rotation, turn, tilt, x, y, description
		FROM connection WHERE design_id = ? ORDER BY ordinal`, designId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var connections []Connection
	for rows.Next() {
		var c Connection
		var id, cPiece, gPiece string
		var cPort, cDesPiece, gPort, gDesPiece sql.NullString
		var gap, shift, rise, rotation, turn, tilt, x, y sql.NullFloat64
		var description sql.NullString
		var cSide, gSide string
		if err := rows.Scan(&id, &cSide, &cPiece, &cPort, &cDesPiece, &gSide, &gPiece, &gPort, &gDesPiece,
			&gap, &shift, &rise, &rotation, &turn, &tilt, &x, &y, &description); err != nil {
			return nil, err
		}
		_ = cSide
		_ = gSide
		c.Id = id
		c.Parent.Piece = PieceId{Id: cPiece}
		if cDesPiece.Valid {
			c.Parent.DesignPiece = &PieceId{Id: cDesPiece.String}
		}
		if connID := connectorIDForTypePort(types, pieceType[cPiece], cPort); connID != nil {
			c.Parent.Connector = connID
		}
		c.Child.Piece = PieceId{Id: gPiece}
		if gDesPiece.Valid {
			c.Child.DesignPiece = &PieceId{Id: gDesPiece.String}
		}
		if connID := connectorIDForTypePort(types, pieceType[gPiece], gPort); connID != nil {
			c.Child.Connector = connID
		}
		if gap.Valid {
			c.Gap = gap.Float64
		}
		if shift.Valid {
			c.Shift = shift.Float64
		}
		if rise.Valid {
			c.Rise = rise.Float64
		}
		if rotation.Valid {
			c.Rotation = rotation.Float64
		}
		if turn.Valid {
			c.Turn = turn.Float64
		}
		if tilt.Valid {
			c.Tilt = tilt.Float64
		}
		if x.Valid {
			c.U = x.Float64
		}
		if y.Valid {
			c.V = y.Float64
		}
		if description.Valid {
			c.Description = &description.String
		}
		connections = append(connections, c)
	}
	return connections, nil
}

func connectorIDForTypePort(types []Type, typeID string, portID sql.NullString) *ConnectorId {
	if !portID.Valid || typeID == "" {
		return nil
	}
	for _, typ := range types {
		if typ.Id != typeID {
			continue
		}
		for i := range typ.Connectors {
			co := &typ.Connectors[i]
			if co.Port != nil && co.Port.Id == portID.String {
				return &ConnectorId{Id: co.Id}
			}
		}
	}
	return nil
}

// 🔌loadConnectors loads all connectors belonging to a type from the database
func loadConnectors(db *sql.DB, typeId string) ([]Connector, error) {
	rows, err := db.Query(`SELECT id, name, description, port_id FROM connector WHERE type_id = ? ORDER BY ordinal`, typeId)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var connectors []Connector
	for rows.Next() {
		var c Connector
		var name, description, portID sql.NullString
		if err := rows.Scan(&c.Id, &name, &description, &portID); err != nil {
			return nil, err
		}
		if name.Valid {
			c.Name = &name.String
		}
		c.Point = Point{}
		c.Direction = Vector{}
		c.T = 0
		if portID.Valid {
			c.Port = &PortId{Id: portID.String}
		}
		if description.Valid {
			c.Description = &description.String
		}
		connectors = append(connectors, c)
	}
	return connectors, nil
}

func kitSqliteTimestamps(k *Kit) (created, updated string) {
	now := time.Now().UTC().Format(time.RFC3339Nano)
	created = k.CreatedAt
	if created == "" {
		created = now
	}
	updated = k.UpdatedAt
	if updated == "" {
		updated = now
	}
	return created, updated
}

func portIDForConnector(types []Type, connectorID string) (string, error) {
	for _, typ := range types {
		for i := range typ.Connectors {
			c := typ.Connectors[i]
			if c.Id == connectorID {
				if c.Port == nil {
					return "", fmt.Errorf("connector %s has no port reference", connectorID)
				}
				return c.Port.Id, nil
			}
		}
	}
	return "", fmt.Errorf("connector %s not found", connectorID)
}

func connectionSidePortID(types []Type, side Side) (*string, error) {
	if side.Connector == nil {
		return nil, nil
	}
	pid, err := portIDForConnector(types, side.Connector.Id)
	if err != nil {
		return nil, err
	}
	return &pid, nil
}

func insertPortRow(db *sql.DB, kitID string, familyID *string, port Port, ordinal int) error {
	var fam any
	if familyID != nil {
		fam = *familyID
	}
	if _, err := db.Exec(`INSERT INTO port (
			id, ordinal, name, icon, mandatory, t, description,
			point_x, point_y, point_z, direction_x, direction_y, direction_z,
			kit_id, parent_family_id
		) VALUES (?, ?, ?, ?, NULL, NULL, ?, NULL, NULL, NULL, NULL, NULL, NULL, ?, ?)`,
		port.Id, ordinal, port.Name, port.Icon, port.Description, kitID, fam,
	); err != nil {
		return fmt.Errorf("failed to insert port %s: %w", port.Id, err)
	}
	for cpi, cp := range port.CompatiblePorts {
		if _, err := db.Exec(`INSERT INTO port_compatible_port (port_id, ordinal, compatible_port_id) VALUES (?, ?, ?)`,
			port.Id, cpi, cp.Id); err != nil {
			return fmt.Errorf("port_compatible_port: %w", err)
		}
	}
	return nil
}

func insertPieceRow(db *sql.DB, designID string, p Piece, ordinal int) error {
	var ox, oy, oz, xx, xy, xz, yx, yy, yz *float64
	if p.Plane != nil {
		ox = &p.Plane.Origin.X
		oy = &p.Plane.Origin.Y
		oz = &p.Plane.Origin.Z
		xx = &p.Plane.XAxis.X
		xy = &p.Plane.XAxis.Y
		xz = &p.Plane.XAxis.Z
		yx = &p.Plane.YAxis.X
		yy = &p.Plane.YAxis.Y
		yz = &p.Plane.YAxis.Z
	}
	var cx, cy, cz *float64
	if p.Center != nil {
		cx = &p.Center.U
		cy = &p.Center.V
	}
	var mox, moy, moz, mxx, mxy, mxz, myx, myy, myz *float64
	if p.MirrorPlane != nil {
		mox = &p.MirrorPlane.Origin.X
		moy = &p.MirrorPlane.Origin.Y
		moz = &p.MirrorPlane.Origin.Z
		mxx = &p.MirrorPlane.XAxis.X
		mxy = &p.MirrorPlane.XAxis.Y
		mxz = &p.MirrorPlane.XAxis.Z
		myx = &p.MirrorPlane.YAxis.X
		myy = &p.MirrorPlane.YAxis.Y
		myz = &p.MirrorPlane.YAxis.Z
	}
	hidden := sql.NullInt64{}
	if p.IsHidden != nil {
		v := int64(0)
		if *p.IsHidden {
			v = 1
		}
		hidden = sql.NullInt64{Int64: v, Valid: true}
	}
	locked := sql.NullInt64{}
	if p.IsLocked != nil {
		v := int64(0)
		if *p.IsLocked {
			v = 1
		}
		locked = sql.NullInt64{Int64: v, Valid: true}
	}
	var typeID, designRef *string
	if p.Type != nil {
		typeID = &p.Type.Id
	}
	if p.Design != nil {
		designRef = &p.Design.Id
	}
	_, err := db.Exec(`INSERT INTO piece (
			id, ordinal, name, description,
			plane_origin_x, plane_origin_y, plane_origin_z,
			plane_x_axis_x, plane_x_axis_y, plane_x_axis_z,
			plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
			center_x, center_y, center_z, scale,
			mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z,
			mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z,
			mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z,
			hidden, locked, color, type_id, design_ref_id, design_id
		) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		p.Id, ordinal, p.Name, p.Description,
		ox, oy, oz, xx, xy, xz, yx, yy, yz,
		cx, cy, cz, p.Scale,
		mox, moy, moz, mxx, mxy, mxz, myx, myy, myz,
		hidden, locked, p.Color, typeID, designRef, designID,
	)
	if err != nil {
		return fmt.Errorf("failed to insert piece %s: %w", p.Id, err)
	}
	return nil
}

// ✏️KitToSqlite writes a Kit to a SQLite database file
func KitToSqlite(kit *Kit, dbPath string, schemaSQL string) error {
	if kit == nil {
		return fmt.Errorf("kit is nil")
	}
	db, err := sql.Open("sqlite", dbPath)
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

	created, updated := kitSqliteTimestamps(kit)
	vcsInitial := "{}"
	if _, err := db.Exec(`INSERT INTO compose_schema (schema_version, engine, created_at) VALUES (?, ?, datetime('now'))`,
		ComposeKitSqliteSchemaVersion, ComposeKitSqliteSchemaEngine); err != nil {
		return fmt.Errorf("failed to insert compose_schema: %w", err)
	}
	if _, err := db.Exec(`INSERT INTO kit (
			id, name, description, icon, image, preview, remote, homepage, license, uri, created_at, updated_at, vcs_initial_json
		) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		kit.Id, kit.Name, kit.Description, kit.Icon, kit.Image, kit.Preview, kit.Remote, kit.Homepage, kit.License, nil, created, updated, vcsInitial,
	); err != nil {
		return fmt.Errorf("failed to insert kit: %w", err)
	}

	for fi := range kit.Families {
		fam := kit.Families[fi]
		if _, err := db.Exec(`INSERT INTO family (id, ordinal, name, description, icon, kit_id) VALUES (?, ?, ?, ?, ?, ?)`,
			fam.Id, fi, fam.Name, fam.Description, fam.Icon, kit.Id); err != nil {
			return fmt.Errorf("failed to insert family %s: %w", fam.Id, err)
		}
		fid := fam.Id
		for pi := range fam.Ports {
			if err := insertPortRow(db, kit.Id, &fid, fam.Ports[pi], pi); err != nil {
				return err
			}
		}
	}

	KitEnsureTypologies(kit)
	for topoIdx := range kit.Typologies {
		topo := kit.Typologies[topoIdx]
		if _, err := db.Exec(`INSERT INTO typology (id, ordinal, name, description, icon, folder_id, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?)`,
			topo.Id, topoIdx, topo.Name, topo.Description, topo.Icon, topo.Folder, kit.Id); err != nil {
			return fmt.Errorf("failed to insert typology %s: %w", topo.Id, err)
		}
		for ti := range topo.Types {
			t := topo.Types[ti]
			if t.Typology.Id == "" {
				t.Typology = TypologyId{Id: topo.Id}
			}
			virtualVal := sql.NullInt64{}
			if t.Virtual != nil {
				v := int64(0)
				if *t.Virtual {
					v = 1
				}
				virtualVal = sql.NullInt64{Int64: v, Valid: true}
			}
			var stock sql.NullInt64
			if t.Stock != nil {
				stock = sql.NullInt64{Int64: int64(*t.Stock), Valid: true}
			}
			var locID *string
			if t.Location != nil {
				locID = &t.Location.Id
			}
			var createdT, updatedT any
			createdT = t.CreatedAt
			if t.CreatedAt == "" {
				createdT = nil
			}
			updatedT = t.UpdatedAt
			if t.UpdatedAt == "" {
				updatedT = nil
			}
			if _, err := db.Exec(`INSERT INTO type (
					id, ordinal, name, description, icon, image, stock, virtual, unit, location_id, created_at, updated_at, typology_id
				) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
				t.Id, ti, t.Name, t.Description, t.Icon, t.Image, stock, virtualVal, t.Unit, locID, createdT, updatedT, topo.Id,
			); err != nil {
				return fmt.Errorf("failed to insert type %s: %w", t.Id, err)
			}
			for fi := range t.Families {
				ref := t.Families[fi]
				if _, err := db.Exec(`INSERT INTO type_family (type_id, family_id, ordinal) VALUES (?, ?, ?)`, t.Id, ref.Id, fi); err != nil {
					return fmt.Errorf("failed to insert type_family for type %s: %w", t.Id, err)
				}
			}
			for ci := range t.Connectors {
				c := t.Connectors[ci]
				if c.Port == nil {
					return fmt.Errorf("connector %s on type %s needs port_id for SQLite", c.Id, t.Id)
				}
				cname := ""
				if c.Name != nil {
					cname = *c.Name
				}
				if _, err := db.Exec(`INSERT INTO connector (id, ordinal, name, description, port_id, type_id) VALUES (?, ?, ?, ?, ?, ?)`,
					c.Id, ci, cname, c.Description, c.Port.Id, t.Id); err != nil {
					return fmt.Errorf("failed to insert connector %s: %w", c.Id, err)
				}
			}
		}
		for di := range topo.Designs {
			d := topo.Designs[di]
			if d.Typology.Id == "" {
				d.Typology = TypologyId{Id: topo.Id}
			}
			var locID *string
			if d.Location != nil {
				locID = &d.Location.Id
			}
			var createdD, updatedD any
			createdD = d.CreatedAt
			if d.CreatedAt == "" {
				createdD = nil
			}
			updatedD = d.UpdatedAt
			if d.UpdatedAt == "" {
				updatedD = nil
			}
			if _, err := db.Exec(`INSERT INTO design (
					id, ordinal, name, description, icon, image, location_id, unit, created_at, updated_at, typology_id
				) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
				d.Id, di, d.Name, d.Description, d.Icon, d.Image, locID, d.Unit, createdD, updatedD, topo.Id,
			); err != nil {
				return fmt.Errorf("failed to insert design %s: %w", d.Id, err)
			}
			for fi := range d.Families {
				ref := d.Families[fi]
				if _, err := db.Exec(`INSERT INTO design_family (design_id, family_id, ordinal) VALUES (?, ?, ?)`, d.Id, ref.Id, fi); err != nil {
					return fmt.Errorf("failed to insert design_family: %w", err)
				}
			}
			for pi := range d.Pieces {
				if err := insertPieceRow(db, d.Id, d.Pieces[pi], pi); err != nil {
					return err
				}
			}
			for ci := range d.Connections {
				c := d.Connections[ci]
				cpid, err := connectionSidePortID(kit.Types, c.Parent)
				if err != nil {
					return fmt.Errorf("connection %s parent side: %w", c.Id, err)
				}
				gpid, err := connectionSidePortID(kit.Types, c.Child)
				if err != nil {
					return fmt.Errorf("connection %s child side: %w", c.Id, err)
				}
				var cdes, gdes *string
				if c.Parent.DesignPiece != nil {
					cdes = &c.Parent.DesignPiece.Id
				}
				if c.Child.DesignPiece != nil {
					gdes = &c.Child.DesignPiece.Id
				}
				if _, err := db.Exec(`INSERT INTO connection (
						id, ordinal,
						parent_side_id, parent_piece_id, parent_port_id, parent_design_piece_id,
						child_side_id, child_piece_id, child_port_id, child_design_piece_id,
						gap, shift, rise, rotation, turn, tilt, x, y, description, design_id
					) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
					c.Id, ci,
					fmt.Sprintf("%s:parent", c.Id), c.Parent.Piece.Id, cpid, cdes,
					fmt.Sprintf("%s:child", c.Id), c.Child.Piece.Id, gpid, gdes,
					c.Gap, c.Shift, c.Rise, c.Rotation, c.Turn, c.Tilt, c.U, c.V, c.Description, d.Id,
				); err != nil {
					return fmt.Errorf("failed to insert connection %s: %w", c.Id, err)
				}
			}
		}
	}

	if _, err := db.Exec("PRAGMA foreign_keys = ON"); err != nil {
		return fmt.Errorf("failed to re-enable foreign keys: %w", err)
	}

	return nil
}

// 🧲KitFromZip extracts a Kit and its files from a zip archive
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
		} else if !strings.HasPrefix(f.Name, ".compose/") {
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

// 📁buildFilePath constructs the file path from the folder hierarchy and file name
func buildFilePath(kit *Kit, file *File) string {
	if file.Folder == nil {
		return file.Name
	}
	folderPath := buildFolderPath(kit, file.Folder.Id)
	if folderPath == "" {
		return file.Name
	}
	return folderPath + "/" + file.Name
}

// 🧱buildFolderPath constructs the folder path from the folder hierarchy
func buildFolderPath(kit *Kit, folderId string) string {
	for _, f := range kit.Folders {
		if f.Id == folderId {
			if f.Parent == nil {
				return f.Name
			}
			parentPath := buildFolderPath(kit, f.Parent.Id)
			if parentPath == "" {
				return f.Name
			}
			return parentPath + "/" + f.Name
		}
	}
	return ""
}

// 🔤blobEncode encodes bytes to a data URI string with the mime type inferred from filename.
// ❓Falls back to "application/octet-stream" when the extension is unknown.
func blobEncode(data []byte, filename string) string {
	mimeStr := mimeFromFilename(filename)
	return "data:" + mimeStr + ";base64," + base64.StdEncoding.EncodeToString(data)
}

// 📄mimeFromFilename returns the mime type for a given filename based on its extension.
// 🔤Returns "application/octet-stream" when the extension is unknown.
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

// 🔗blobDecode decodes a data URI string to bytes.
// 🖊️Supports "data:<mime>;base64,<data>" format as well as raw base64 for backwards compatibility.
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

// 📦KitToZip packages a Kit and its files into a zip archive
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

// #region 🔄Kit Workflow
// Kit workflow operations MUST provide direct import, export, and edit flows for file, folder, archive, remote, and temporary kit kinds.

// 📥ImportFileKit reads a JSON file kit from disk.
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

// 📤ExportFileKit writes a JSON file kit to disk.
func ExportFileKit(kit Kit, path string) error {
	data, err := SerializeKit(kit)
	if err != nil {
		return err
	}
	return os.WriteFile(path, data, 0o644)
}

// 📄ImportArchiveKit reads an archive kit from a zip file.
func ImportArchiveKit(path string) (*Kit, map[string][]byte, error) {
	return KitFromZip(path)
}

// ✏️ExportArchiveKit writes an archive kit to a zip file.
func ExportArchiveKit(kit *Kit, files map[string][]byte, path string) error {
	return KitToZip(kit, ensureKitFiles(kit, files), path, "")
}

// 📁ImportFolderKit reads a folder kit from a local folder containing .compose/kit.db and asset files.
func ImportFolderKit(folderPath string) (*Kit, map[string][]byte, error) {
	dbPath := filepath.Join(folderPath, ".compose", "kit.db")
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
			if path == filepath.Join(folderPath, ".compose") {
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

// 🖼️ExportFolderKit writes a folder kit to a local folder containing .compose/kit.db and asset files.
func ExportFolderKit(kit *Kit, files map[string][]byte, folderPath string) error {
	composePath := filepath.Join(folderPath, ".compose")
	if err := os.MkdirAll(composePath, 0o755); err != nil {
		return err
	}
	dbPath := filepath.Join(composePath, "kit.db")
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

// 📦ImportRemoteKit reads a remote kit from HTTP(S), supporting both JSON and ZIP sources.
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
		tmpFile, err := os.CreateTemp("", "compose-remote-*.zip")
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

// 📦EditTemporaryKit applies a diff to an in-memory kit value, mutating it in place.
func EditTemporaryKit(kit *Kit, diff *KitDiff) {
	ApplyKitDiff(kit, diff)
}

// 📦EditFileKit edits a file kit in place and returns the edited kit.
func EditFileKit(path string, diff KitDiff) (*Kit, error) {
	kit, err := ImportFileKit(path)
	if err != nil {
		return nil, err
	}
	EditTemporaryKit(kit, &diff)
	if err := ExportFileKit(*kit, path); err != nil {
		return nil, err
	}
	return kit, nil
}

// 📦EditFolderKit edits a folder kit in place and returns the edited kit.
func EditFolderKit(folderPath string, diff KitDiff) (*Kit, error) {
	kit, files, err := ImportFolderKit(folderPath)
	if err != nil {
		return nil, err
	}
	EditTemporaryKit(kit, &diff)
	if err := ExportFolderKit(kit, files, folderPath); err != nil {
		return nil, err
	}
	return kit, nil
}

// 📦EditArchiveKit edits an archive kit in place and returns the edited kit.
func EditArchiveKit(path string, diff KitDiff) (*Kit, error) {
	kit, files, err := ImportArchiveKit(path)
	if err != nil {
		return nil, err
	}
	EditTemporaryKit(kit, &diff)
	if err := ExportArchiveKit(kit, files, path); err != nil {
		return nil, err
	}
	return kit, nil
}

// 📦EditRemoteKit imports a remote kit and applies a diff in memory.
func EditRemoteKit(rawURL string, diff KitDiff) (*Kit, error) {
	kit, _, err := ImportRemoteKit(rawURL)
	if err != nil {
		return nil, err
	}
	EditTemporaryKit(kit, &diff)
	return kit, nil
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
		"../schema/sqlite/🛢️schema.sql",
		"../../schema/sqlite/🛢️schema.sql",
		"../../../schema/sqlite/🛢️schema.sql",
		"../sqlite/🛢️schema.sql",
		"../../sqlite/🛢️schema.sql",
		"sqlite/🛢️schema.sql",
	}
	for _, candidate := range candidatePaths {
		if data, err := os.ReadFile(candidate); err == nil {
			return string(data)
		}
	}
	panic("failed to locate sqlite/🛢️schema.sql for kit workflow operations")
}

// #endregion 🔄Kit Workflow

// #endregion 📡SQLite

// #region kit_graph
// #region Kit graph session (TypeScript parity: commitKitGraphChange, backbone, transactions, history)

// Backbone receives outbound graph commits; implementations may run Changed asynchronously.
type Backbone interface {
	Changed(KitGraphChange) error
}

// BackboneAttach adds inbound diff delivery (optional).
type BackboneAttach interface {
	Backbone
	Attach(kit *Kit, onInbound func(KitDiff)) error
}

// KitGraphChange bundles forward/backward diffs with validation (TypeScript KitChange).
type KitGraphChange struct {
	Forward    KitDiff
	Backward   KitDiff
	Validation KitDiffValidationResult
}

// KitCommitOptions configures CommitKitGraphChange.
type KitCommitOptions struct {
	Origin            string
	TransactionID     string
	NotifyBackbone    *bool
	SkipGlobalHistory bool
}

type kitOpenTransaction struct {
	startKit Kit
	steps    []KitGraphChange
	redo     []KitGraphChange
}

func (k *Kit) ensureGraphMaps() {
	if k.openTransactions == nil {
		k.openTransactions = make(map[string]*kitOpenTransaction)
	}
	if k.flattenMerkle == nil {
		k.flattenMerkle = make(map[string]map[string]FlatMerkleCacheEntry)
	}
}

// kitStripRuntime copies domain fields only (no mutex / session), for JSON snapshot.
func kitStripRuntime(k *Kit) Kit {
	if k == nil {
		return Kit{}
	}
	return Kit{
		Id:          k.Id,
		Name:        k.Name,
		Version:     k.Version,
		Typologies:  k.Typologies,
		Types:       k.Types,
		Designs:     k.Designs,
		Tags:        k.Tags,
		Concepts:    k.Concepts,
		Families:    k.Families,
		Qualities:   k.Qualities,
		Files:       k.Files,
		Folders:     k.Folders,
		Authors:     k.Authors,
		Remote:      k.Remote,
		Homepage:    k.Homepage,
		License:     k.License,
		Preview:     k.Preview,
		Icon:        k.Icon,
		Image:       k.Image,
		Description: k.Description,
		Attributes:  k.Attributes,
		CreatedAt:   k.CreatedAt,
		UpdatedAt:   k.UpdatedAt,
	}
}

func kitSnapshotKit(k *Kit) (Kit, error) {
	if k == nil {
		return Kit{}, fmt.Errorf("nil kit")
	}
	slim := kitStripRuntime(k)
	data, err := json.Marshal(slim)
	if err != nil {
		return Kit{}, err
	}
	var out Kit
	if err := json.Unmarshal(data, &out); err != nil {
		return Kit{}, err
	}
	return out, nil
}

// CommitKitGraphChange validates, inverts, applies, records history/transaction, notifies backbone (see KitCommitOptions).
func CommitKitGraphChange(kit *Kit, diff KitDiff, opts *KitCommitOptions) (KitGraphChange, error) {
	if kit == nil {
		return KitGraphChange{}, fmt.Errorf("nil kit")
	}
	o := KitCommitOptions{}
	if opts != nil {
		o = *opts
	}
	return kit.commitGraphChange(diff, o)
}

// SetBackbone attaches a backbone and optionally runs Attach for inbound diffs.
func (k *Kit) SetBackbone(b Backbone) {
	k.graphMu.Lock()
	k.backbone = b
	k.graphMu.Unlock()
	if b == nil {
		return
	}
	if att, ok := b.(BackboneAttach); ok {
		go func() {
			_ = att.Attach(k, func(in KitDiff) {
				_, _ = CommitKitGraphChange(k, in, &KitCommitOptions{NotifyBackbone: KitNotifyDisable()})
			})
		}()
	}
}

func ptrBool(v bool) *bool { return &v }

// KitNotifyDisable returns a *bool false for KitCommitOptions.NotifyBackbone.
func KitNotifyDisable() *bool { f := false; return &f }

// SetStrictMode when true treats validation warnings like errors.
func (k *Kit) SetStrictMode(strict bool) {
	k.graphMu.Lock()
	k.strictMode = strict
	k.graphMu.Unlock()
}

// ClearConflict clears the conflict lock without mutating entities.
func (k *Kit) ClearConflict() {
	k.graphMu.Lock()
	k.conflicted = false
	k.conflictErrors = nil
	k.conflictWarnings = nil
	k.graphMu.Unlock()
}

func (k *Kit) IsConflict() bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	return k.conflicted
}

func (k *Kit) ValidationSnapshot() KitDiffValidationResult {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	return KitDiffValidationResult{
		Ok:       !k.conflicted,
		Errors:   append([]KitDiffValidationNote(nil), k.conflictErrors...),
		Warnings: append([]KitDiffValidationNote(nil), k.conflictWarnings...),
	}
}

// StartTransaction opens a new transaction; multiple may be open. Pass ID to CommitKitGraphChange.
func (k *Kit) StartTransaction() (string, error) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return "", fmt.Errorf("kit conflicted; call ClearConflict first")
	}
	k.ensureGraphMaps()
	start, err := kitSnapshotKit(k)
	if err != nil {
		return "", err
	}
	id := Id()
	k.openTransactions[id] = &kitOpenTransaction{startKit: start, steps: nil, redo: nil}
	return id, nil
}

func (k *Kit) AbortTransaction(txID string) error {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return fmt.Errorf("kit conflicted")
	}
	tx, ok := k.openTransactions[txID]
	if !ok {
		return fmt.Errorf("unknown transaction %q", txID)
	}
	for i := len(tx.steps) - 1; i >= 0; i-- {
		ApplyKitDiff(k, &tx.steps[i].Backward)
	}
	delete(k.openTransactions, txID)
	k.conflicted = false
	k.conflictErrors = nil
	k.conflictWarnings = nil
	return nil
}

// FinalizeTransaction squashes net diff vs start snapshot, validates, pushes global history, notifies backbone once.
func (k *Kit) FinalizeTransaction(txID string) (KitGraphChange, error) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return KitGraphChange{}, fmt.Errorf("kit conflicted")
	}
	tx, ok := k.openTransactions[txID]
	if !ok {
		return KitGraphChange{}, fmt.Errorf("unknown transaction %q", txID)
	}

	forward := GetKitDiff(tx.startKit, *k)
	val := ValidateKitDiff(tx.startKit, forward, false)
	if !val.Ok || len(val.Errors) > 0 {
		return KitGraphChange{}, fmt.Errorf("finalize validation failed: %v", val.Errors)
	}
	if k.strictMode && len(val.Warnings) > 0 {
		return KitGraphChange{}, fmt.Errorf("finalize warnings (strict): %v", val.Warnings)
	}
	diffToApply := forward
	if val.Diff != nil {
		diffToApply = *val.Diff
	}
	backward := InverseKitDiff(tx.startKit, diffToApply)
	squashed := KitGraphChange{Forward: diffToApply, Backward: backward, Validation: val}

	delete(k.openTransactions, txID)
	k.historyPast = append(k.historyPast, squashed)
	k.historyFuture = nil

	if k.backbone != nil {
		ch := squashed
		go func() { _ = k.backbone.Changed(ch) }()
	}
	return squashed, nil
}

func (k *Kit) UndoWithinTransaction(txID string) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	tx, ok := k.openTransactions[txID]
	if !ok || len(tx.steps) == 0 {
		return
	}
	ch := tx.steps[len(tx.steps)-1]
	tx.steps = tx.steps[:len(tx.steps)-1]
	ApplyKitDiff(k, &ch.Backward)
	tx.redo = append(tx.redo, ch)
}

func (k *Kit) RedoWithinTransaction(txID string) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	tx, ok := k.openTransactions[txID]
	if !ok || len(tx.redo) == 0 {
		return
	}
	ch := tx.redo[len(tx.redo)-1]
	tx.redo = tx.redo[:len(tx.redo)-1]
	ApplyKitDiff(k, &ch.Forward)
	tx.steps = append(tx.steps, ch)
}

func (k *Kit) CanUndoWithinTransaction(txID string) bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	tx, ok := k.openTransactions[txID]
	return ok && len(tx.steps) > 0
}

func (k *Kit) CanRedoWithinTransaction(txID string) bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	tx, ok := k.openTransactions[txID]
	return ok && len(tx.redo) > 0
}

func (k *Kit) UndoHistory() error {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return fmt.Errorf("kit conflicted")
	}
	if len(k.historyPast) == 0 {
		return nil
	}
	ch := k.historyPast[len(k.historyPast)-1]
	k.historyPast = k.historyPast[:len(k.historyPast)-1]
	ApplyKitDiff(k, &ch.Backward)
	k.historyFuture = append(k.historyFuture, ch)
	return nil
}

func (k *Kit) RedoHistory() error {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return fmt.Errorf("kit conflicted")
	}
	if len(k.historyFuture) == 0 {
		return nil
	}
	ch := k.historyFuture[len(k.historyFuture)-1]
	k.historyFuture = k.historyFuture[:len(k.historyFuture)-1]
	ApplyKitDiff(k, &ch.Forward)
	k.historyPast = append(k.historyPast, ch)
	return nil
}

func (k *Kit) CanUndoHistory() bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	return len(k.historyPast) > 0
}

func (k *Kit) CanRedoHistory() bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	return len(k.historyFuture) > 0
}

// TransactFinalized runs fn with a transaction id, then finalizes or aborts on error.
func (k *Kit) TransactFinalized(fn func(txID string) error) error {
	id, err := k.StartTransaction()
	if err != nil {
		return err
	}
	if err := fn(id); err != nil {
		_ = k.AbortTransaction(id)
		return err
	}
	_, err = k.FinalizeTransaction(id)
	return err
}

func (k *Kit) commitGraphChange(diff KitDiff, o KitCommitOptions) (KitGraphChange, error) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return KitGraphChange{}, fmt.Errorf("kit conflicted; call ClearConflict first")
	}
	val := ValidateKitDiff(*k, diff, false)
	if !val.Ok || len(val.Errors) > 0 {
		k.conflicted = true
		k.conflictErrors = val.Errors
		k.conflictWarnings = val.Warnings
		return KitGraphChange{}, fmt.Errorf("validation failed: %v", val.Errors)
	}
	if k.strictMode && len(val.Warnings) > 0 {
		k.conflicted = true
		k.conflictErrors = val.Errors
		k.conflictWarnings = val.Warnings
		return KitGraphChange{}, fmt.Errorf("validation warnings (strict): %v", val.Warnings)
	}
	diffToApply := diff
	if val.Diff != nil {
		diffToApply = *val.Diff
	}
	backward := InverseKitDiff(*k, diffToApply)
	ch := KitGraphChange{Forward: diffToApply, Backward: backward, Validation: val}
	ApplyKitDiff(k, &diffToApply)

	k.ensureGraphMaps()
	if o.TransactionID != "" {
		tx, ok := k.openTransactions[o.TransactionID]
		if !ok {
			return KitGraphChange{}, fmt.Errorf("unknown transaction %q", o.TransactionID)
		}
		tx.steps = append(tx.steps, ch)
		tx.redo = nil
	} else if !o.SkipGlobalHistory {
		k.historyPast = append(k.historyPast, ch)
		k.historyFuture = nil
	}

	notify := o.TransactionID == ""
	if o.NotifyBackbone != nil {
		notify = *o.NotifyBackbone
	}
	if o.TransactionID != "" {
		notify = false
	}
	if notify && k.backbone != nil {
		b := k.backbone
		c := ch
		go func() { _ = b.Changed(c) }()
	}

	k.conflicted = false
	k.conflictErrors = nil
	k.conflictWarnings = nil
	return ch, nil
}

// #endregion
// #endregion kit_graph

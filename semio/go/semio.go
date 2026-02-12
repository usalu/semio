// #region 🔖Header

// 💻semio/go/semio.go

// 2026 Ueli Saluz <ueli@semio-tech.de>

// #region 🔖License

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header


// #region 🔖Imports
// Imports MUST include all required packages for the semio domain library.

package semio

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"strings"

	"gonum.org/v1/gonum/mat"
)

// #endregion 🔖Imports

// #region 🔖Constants
// Constants MUST define shared constant values for the semio domain.

const (
	IconWidth = 24
	Tolerance = 0.0001
)

// #endregion 🔖Constants

// #region 🔖Utils
// Utils MUST provide general-purpose utility functions for the semio domain.

// Guid MUST return a cryptographically random 128-bit hex string.
// Guid generates a new random 128-bit hex-encoded unique identifier.
func Guid() string {
	bytes := make([]byte, 16)
	rand.Read(bytes)
	return hex.EncodeToString(bytes)
}

// Normalize MUST trim whitespace and convert to lowercase.
// Normalize converts a string to lowercase trimmed form.
func Normalize(s string) string {
	return strings.ToLower(strings.TrimSpace(s))
}

// Round MUST return the value rounded to exactly the given decimal places.
// Round rounds a float64 to the specified number of decimal places.
func Round(value float64, decimals int) float64 {
	shift := 1.0
	for i := 0; i < decimals; i++ {
		shift *= 10
	}
	return float64(int64(value*shift+0.5)) / shift
}

// DeepEqual MUST return true only when both values produce identical JSON.
// DeepEqual compares two values for deep equality via JSON serialization.
func DeepEqual(a, b interface{}) bool {
	aJSON, _ := json.Marshal(a)
	bJSON, _ := json.Marshal(b)
	return string(aJSON) == string(bJSON)
}

// #endregion 🔖Utils

// #region 🔖Entity IDs
// Entity IDs MUST define identifier types for all semio domain entities.

// AttributeId identifies an attribute entity by GUID.
type AttributeId struct {
	Guid string `json:"guid"`
}

// LocationId identifies a location entity by GUID.
type LocationId struct {
	Guid string `json:"guid"`
}

// AuthorId identifies an author entity by GUID.
type AuthorId struct {
	Guid string `json:"guid"`
}

// FileId identifies a file entity by GUID.
type FileId struct {
	Guid string `json:"guid"`
}

// FolderId identifies a folder entity by GUID.
type FolderId struct {
	Guid string `json:"guid"`
}

// BenchmarkId identifies a benchmark entity by GUID.
type BenchmarkId struct {
	Guid string `json:"guid"`
}

// QualityId identifies a quality entity by GUID.
type QualityId struct {
	Guid string `json:"guid"`
}

// PortId identifies a port entity by GUID.
type PortId struct {
	Guid string `json:"guid"`
}

// PropId identifies a prop entity by GUID.
type PropId struct {
	Guid string `json:"guid"`
}

// TagId identifies a tag entity by GUID.
type TagId struct {
	Guid string `json:"guid"`
}

// ConceptId identifies a concept entity by GUID.
type ConceptId struct {
	Guid string `json:"guid"`
}

// ModelId identifies a model entity by GUID.
type ModelId struct {
	Guid string `json:"guid"`
}

// ConnectorId identifies a connector entity by GUID.
type ConnectorId struct {
	Guid string `json:"guid"`
}

// TypeId identifies a type entity by GUID.
type TypeId struct {
	Guid string `json:"guid"`
}

// LayerId identifies a layer entity by GUID.
type LayerId struct {
	Guid string `json:"guid"`
}

// PieceId identifies a piece entity by GUID.
type PieceId struct {
	Guid string `json:"guid"`
}

// GroupId identifies a group entity by GUID.
type GroupId struct {
	Guid string `json:"guid"`
}

// SideId identifies a connection side by piece, design piece and connector references.
type SideId struct {
	Piece          PieceId      `json:"piece"`
	DesignPiece    *PieceId     `json:"designPiece,omitempty"`
	Connector      *ConnectorId `json:"connector,omitempty"`
}

// ConnectionId identifies a connection entity by GUID.
type ConnectionId struct {
	Guid string `json:"guid"`
}

// StatId identifies a stat entity by GUID.
type StatId struct {
	Guid string `json:"guid"`
}

// DesignId identifies a design entity by GUID.
type DesignId struct {
	Guid string `json:"guid"`
}

// KitId identifies a kit entity by GUID.
type KitId struct {
	Guid string `json:"guid"`
}

// #endregion 🔖Entity IDs

// #region 🔖Weak Entities
// Weak Entities MUST define value types that exist only as part of parent entities.

// Coord represents a 2D coordinate with U and V components.
type Coord struct {
	U float64 `json:"u"`
	V float64 `json:"v"`
}

// Vec represents a 2D vector with U and V components.
type Vec struct {
	U float64 `json:"u"`
	V float64 `json:"v"`
}

// Point represents a 3D point with X, Y and Z components.
type Point struct {
	X float64 `json:"x"`
	Y float64 `json:"y"`
	Z float64 `json:"z"`
}

// Vector represents a 3D vector with X, Y and Z components.
type Vector struct {
	X float64 `json:"x"`
	Y float64 `json:"y"`
	Z float64 `json:"z"`
}

// Plane represents a 3D plane defined by origin, X-axis and Y-axis.
type Plane struct {
	Origin Point  `json:"origin"`
	XAxis  Vector `json:"xAxis"`
	YAxis  Vector `json:"yAxis"`
}

// Camera represents a 3D camera with position, forward and up vectors.
type Camera struct {
	Position Point  `json:"position"`
	Forward  Vector `json:"forward"`
	Up       Vector `json:"up"`
}

// #endregion 🔖Weak Entities

// #region 🔖Attribute
// Attribute MUST define the key-value metadata entity and its diff types.

// Attribute represents a key-value metadata entry with optional definition.
type Attribute struct {
	Guid       string  `json:"guid"`
	Key        string  `json:"key"`
	Value      *string `json:"value,omitempty"`
	Definition *string `json:"definition,omitempty"`
}

// AttributeDiff represents changes to an attribute entity.
type AttributeDiff struct {
	Key        *string `json:"key,omitempty"`
	Value      *string `json:"value,omitempty"`
	Definition *string `json:"definition,omitempty"`
}

// AttributesDiff represents a collection of attribute additions, removals and updates.
type AttributesDiff struct {
	Removed []AttributeId `json:"removed,omitempty"`
	Updated []struct {
		Attribute AttributeId   `json:"attribute"`
		Diff      AttributeDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Attribute `json:"added,omitempty"`
}

// #endregion 🔖Attribute

// #region 🔖Location
// Location MUST define geographic location entities and their diff types.

// Location represents a geographic location with longitude, latitude and optional altitude.
type Location struct {
	Guid       string      `json:"guid"`
	Longitude  float64     `json:"longitude"`
	Latitude   float64     `json:"latitude"`
	Altitude   *float64    `json:"altitude,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
}

// LocationDiff represents changes to a location entity.
type LocationDiff struct {
	Longitude  *float64        `json:"longitude,omitempty"`
	Latitude   *float64        `json:"latitude,omitempty"`
	Altitude   *float64        `json:"altitude,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// #endregion 🔖Location

// #region 🔖Author
// Author MUST define authorship entities and their diff types.

// Author represents a named authorship entity with optional email.
type Author struct {
	Guid       string      `json:"guid"`
	Name       string      `json:"name"`
	Email      *string     `json:"email,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
	CreatedAt  string      `json:"createdAt,omitempty"`
	UpdatedAt  string      `json:"updatedAt,omitempty"`
}

// AuthorDiff represents changes to an author entity.
type AuthorDiff struct {
	Name       *string         `json:"name,omitempty"`
	Email      *string         `json:"email,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// AuthorsDiff represents a collection of author additions, removals and updates.
type AuthorsDiff struct {
	Removed []AuthorId `json:"removed,omitempty"`
	Updated []struct {
		Author AuthorId   `json:"author"`
		Diff   AuthorDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Author `json:"added,omitempty"`
}

// #endregion 🔖Author

// #region 🔖File
// File MUST define file reference entities and their diff types.

// File represents a file reference entity with name, remote URL and metadata.
type File struct {
	Guid        string      `json:"guid"`
	Name        string      `json:"name"`
	Remote      *string     `json:"remote,omitempty"`
	Size        *int64      `json:"size,omitempty"`
	Hash        *string     `json:"hash,omitempty"`
	Description *string     `json:"description,omitempty"`
	Folder      *FolderId   `json:"folder,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// FileDiff represents changes to a file entity.
type FileDiff struct {
	Name        *string         `json:"name,omitempty"`
	Remote      *string         `json:"remote,omitempty"`
	Size        *int64          `json:"size,omitempty"`
	Hash        *string         `json:"hash,omitempty"`
	Description *string         `json:"description,omitempty"`
	Folder      *FolderId       `json:"folder,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// FilesDiff represents a collection of file additions, removals and updates.
type FilesDiff struct {
	Removed []FileId `json:"removed,omitempty"`
	Updated []struct {
		File FileId   `json:"file"`
		Diff FileDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []File `json:"added,omitempty"`
}

// #endregion 🔖File

// #region 🔖Folder
// Folder MUST define folder hierarchy entities and their diff types.

// Folder represents a folder hierarchy entity with name and parent reference.
type Folder struct {
	Guid        string      `json:"guid"`
	Name        string      `json:"name"`
	Parent      *FolderId   `json:"parent,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// FolderDiff represents changes to a folder entity.
type FolderDiff struct {
	Name        *string         `json:"name,omitempty"`
	Parent      *FolderId       `json:"parent,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// FoldersDiff represents a collection of folder additions, removals and updates.
type FoldersDiff struct {
	Removed []FolderId `json:"removed,omitempty"`
	Updated []struct {
		Folder FolderId   `json:"folder"`
		Diff   FolderDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Folder `json:"added,omitempty"`
}

// #endregion 🔖Folder

// #region 🔖Benchmark
// Benchmark MUST define benchmark threshold entities and their diff types.

// Benchmark represents a named metric threshold with min and max bounds.
type Benchmark struct {
	Guid        string      `json:"guid"`
	Name        string      `json:"name"`
	Icon        *string     `json:"icon,omitempty"`
	Min         *float64    `json:"min,omitempty"`
	MinExcluded *bool       `json:"minExcluded,omitempty"`
	Max         *float64    `json:"max,omitempty"`
	MaxExcluded *bool       `json:"maxExcluded,omitempty"`
	Definition  *string     `json:"definition,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// BenchmarkDiff represents changes to a benchmark entity.
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

// BenchmarksDiff represents a collection of benchmark additions, removals and updates.
type BenchmarksDiff struct {
	Removed []BenchmarkId `json:"removed,omitempty"`
	Updated []struct {
		Benchmark BenchmarkId   `json:"benchmark"`
		Diff      BenchmarkDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Benchmark `json:"added,omitempty"`
}

// #endregion 🔖Benchmark

// #region 🔖Quality
// Quality MUST define measurable quality entities and their diff types.

// QualityKind is a bitfield enum for quality scope classification.
type QualityKind int

const (
	QualityKindGeneral QualityKind = 1 << iota
	QualityKindType
	QualityKindDesign
	QualityKindPiece
	QualityKindConnection
	QualityKindConnector
)

// Quality represents a measurable property with formula, units and benchmarks.
type Quality struct {
	Guid                string      `json:"guid"`
	Key                 string      `json:"key"`
	Name                string      `json:"name"`
	Kind                QualityKind `json:"kind,omitempty"`
	Default             *float64    `json:"default,omitempty"`
	Formula             *string     `json:"formula,omitempty"`
	DefaultSiUnit       *string     `json:"defaultSiUnit,omitempty"`
	DefaultImperialUnit *string     `json:"defaultImperialUnit,omitempty"`
	Min                 *float64    `json:"min,omitempty"`
	MinExcluded         *bool       `json:"minExcluded,omitempty"`
	Max                 *float64    `json:"max,omitempty"`
	MaxExcluded         *bool       `json:"maxExcluded,omitempty"`
	CanScale            *bool       `json:"canScale,omitempty"`
	Benchmarks          []Benchmark `json:"benchmarks,omitempty"`
	Definition          *string     `json:"definition,omitempty"`
	Attributes          []Attribute `json:"attributes,omitempty"`
	CreatedAt           string      `json:"createdAt,omitempty"`
	UpdatedAt           string      `json:"updatedAt,omitempty"`
}

// QualityDiff represents changes to a quality entity.
type QualityDiff struct {
	Key                 *string         `json:"key,omitempty"`
	Name                *string         `json:"name,omitempty"`
	Kind                *QualityKind    `json:"kind,omitempty"`
	Default             *float64        `json:"default,omitempty"`
	Formula             *string         `json:"formula,omitempty"`
	DefaultSiUnit       *string         `json:"defaultSiUnit,omitempty"`
	DefaultImperialUnit *string         `json:"defaultImperialUnit,omitempty"`
	Min                 *float64        `json:"min,omitempty"`
	MinExcluded         *bool           `json:"minExcluded,omitempty"`
	Max                 *float64        `json:"max,omitempty"`
	MaxExcluded         *bool           `json:"maxExcluded,omitempty"`
	CanScale            *bool           `json:"canScale,omitempty"`
	Benchmarks          *BenchmarksDiff `json:"benchmarks,omitempty"`
	Definition          *string         `json:"definition,omitempty"`
	Attributes          *AttributesDiff `json:"attributes,omitempty"`
}

// QualitiesDiff represents a collection of quality additions, removals and updates.
type QualitiesDiff struct {
	Removed []QualityId `json:"removed,omitempty"`
	Updated []struct {
		Quality QualityId   `json:"quality"`
		Diff    QualityDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Quality `json:"added,omitempty"`
}

// #endregion 🔖Quality

// #region 🔖Port
// Port MUST define connector port entities and their diff types.

// Port represents a named connector port with compatible port references.
type Port struct {
	Guid                 string        `json:"guid"`
	Name                 string        `json:"name"`
	Description          *string       `json:"description,omitempty"`
	Icon                 *string       `json:"icon,omitempty"`
	CompatiblePorts []PortId `json:"compatiblePorts,omitempty"`
	Attributes           []Attribute   `json:"attributes,omitempty"`
	CreatedAt            string        `json:"createdAt,omitempty"`
	UpdatedAt            string        `json:"updatedAt,omitempty"`
}

// PortDiff represents changes to a port entity.
type PortDiff struct {
	Name                 *string         `json:"name,omitempty"`
	Description          *string         `json:"description,omitempty"`
	Icon                 *string         `json:"icon,omitempty"`
	CompatiblePorts []PortId   `json:"compatiblePorts,omitempty"`
	Attributes           *AttributesDiff `json:"attributes,omitempty"`
	setFields            map[string]bool `json:"-"`
}

// UnmarshalJSON MUST populate the setFields map for all present JSON keys.
// UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.
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

// HasField MUST return false when setFields is nil.
// HasField returns whether a JSON field was present in the unmarshaled data.
func (d *PortDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// PortsDiff represents a collection of port additions, removals and updates.
type PortsDiff struct {
	Removed []PortId `json:"removed,omitempty"`
	Updated []struct {
		Port PortId   `json:"port"`
		Diff PortDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Port `json:"added,omitempty"`
}

// #endregion 🔖Port

// #region 🔖Prop
// Prop MUST define property value entities and their diff types.

// Prop represents a quality property value with optional unit.
type Prop struct {
	Guid       string      `json:"guid"`
	Quality    QualityId   `json:"quality"`
	Value      string      `json:"value"`
	Unit       *string     `json:"unit,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
}

// PropDiff represents changes to a prop entity.
type PropDiff struct {
	Quality    *QualityId      `json:"quality,omitempty"`
	Value      *string         `json:"value,omitempty"`
	Unit       *string         `json:"unit,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// PropsDiff represents a collection of prop additions, removals and updates.
type PropsDiff struct {
	Removed []PropId `json:"removed,omitempty"`
	Updated []struct {
		Prop PropId   `json:"prop"`
		Diff PropDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Prop `json:"added,omitempty"`
}

// #endregion 🔖Prop

// #region 🔖Tag
// Tag MUST define tag classification entities and their diff types.

// Tag represents a named classification tag with optional description and icon.
type Tag struct {
	Guid        string      `json:"guid"`
	Name        string      `json:"name"`
	Description *string     `json:"description,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// TagDiff represents changes to a tag entity.
type TagDiff struct {
	Name        *string         `json:"name,omitempty"`
	Description *string         `json:"description,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// UnmarshalJSON MUST populate the setFields map for all present JSON keys.
// UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.
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

// HasField MUST return false when setFields is nil.
// HasField returns whether a JSON field was present in the unmarshaled data.
func (d *TagDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// TagsDiff represents a collection of tag additions, removals and updates.
type TagsDiff struct {
	Removed []TagId `json:"removed,omitempty"`
	Updated []struct {
		Tag  TagId   `json:"tag"`
		Diff TagDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Tag `json:"added,omitempty"`
}

// #endregion 🔖Tag

// #region 🔖Concept
// Concept MUST define concept categorization entities and their diff types.

// Concept represents a named categorization concept with optional description.
type Concept struct {
	Guid        string      `json:"guid"`
	Name        string      `json:"name"`
	Description *string     `json:"description,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// ConceptDiff represents changes to a concept entity.
type ConceptDiff struct {
	Name        *string         `json:"name,omitempty"`
	Description *string         `json:"description,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// UnmarshalJSON MUST populate the setFields map for all present JSON keys.
// UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.
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

// HasField MUST return false when setFields is nil.
// HasField returns whether a JSON field was present in the unmarshaled data.
func (d *ConceptDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// ConceptsDiff represents a collection of concept additions, removals and updates.
type ConceptsDiff struct {
	Removed []ConceptId `json:"removed,omitempty"`
	Updated []struct {
		Concept ConceptId   `json:"concept"`
		Diff    ConceptDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Concept `json:"added,omitempty"`
}

// #endregion 🔖Concept

// #region 🔖Model
// Model MUST define 3D model reference entities and their diff types.

// Model represents a 3D model reference associated with a file and tags.
type Model struct {
	Guid        string      `json:"guid"`
	File        FileId      `json:"file"`
	Name        *string     `json:"name,omitempty"`
	Tags        []TagId     `json:"tags,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// ModelDiff represents changes to a model entity.
type ModelDiff struct {
	File        *FileId         `json:"file,omitempty"`
	Name        *string         `json:"name,omitempty"`
	Tags        []TagId         `json:"tags,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// ModelsDiff represents a collection of model additions, removals and updates.
type ModelsDiff struct {
	Removed []ModelId `json:"removed,omitempty"`
	Updated []struct {
		Model ModelId   `json:"model"`
		Diff  ModelDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Model `json:"added,omitempty"`
}

// #endregion 🔖Model

// #region 🔖Connector
// Connector MUST define spatial connector entities and their diff types.

// Connector represents a spatial connection point on a type with position and direction.
type Connector struct {
	Guid        string       `json:"guid"`
	Name        *string      `json:"name,omitempty"`
	Point       Point        `json:"point"`
	Direction   Vector       `json:"direction"`
	T           float64      `json:"t"`
	Mandatory   *bool        `json:"mandatory,omitempty"`
	Port        *PortId `json:"port,omitempty"`
	Props       []Prop       `json:"props,omitempty"`
	Description *string      `json:"description,omitempty"`
	Attributes  []Attribute  `json:"attributes,omitempty"`
}

// PointDiff represents changes to a 3D point.
type PointDiff struct {
	X *float64 `json:"x,omitempty"`
	Y *float64 `json:"y,omitempty"`
	Z *float64 `json:"z,omitempty"`
}

// VectorDiff represents changes to a 3D vector.
type VectorDiff struct {
	X *float64 `json:"x,omitempty"`
	Y *float64 `json:"y,omitempty"`
	Z *float64 `json:"z,omitempty"`
}

// ConnectorDiff represents changes to a connector entity.
type ConnectorDiff struct {
	Name        *string         `json:"name,omitempty"`
	Point       *PointDiff      `json:"point,omitempty"`
	Direction   *VectorDiff     `json:"direction,omitempty"`
	T           *float64        `json:"t,omitempty"`
	Mandatory   *bool           `json:"mandatory,omitempty"`
	Port        *PortId    `json:"port,omitempty"`
	Props       *PropsDiff      `json:"props,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// ConnectorsDiff represents a collection of connector additions, removals and updates.
type ConnectorsDiff struct {
	Removed []ConnectorId `json:"removed,omitempty"`
	Updated []struct {
		Connector ConnectorId   `json:"connector"`
		Diff      ConnectorDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Connector `json:"added,omitempty"`
}

// #endregion 🔖Connector

// #region 🔖Type
// Type MUST define component type entities and their diff types.

// Type represents a component type with models, connectors and hierarchical inheritance.
type Type struct {
	Guid        string      `json:"guid"`
	Name        string      `json:"name"`
	Parent      *TypeId     `json:"parent,omitempty"`
	IsAbstract  *bool       `json:"isAbstract,omitempty"`
	Virtual     *bool       `json:"virtual,omitempty"`
	Unit        *string     `json:"unit,omitempty"`
	Stock       *int        `json:"stock,omitempty"`
	Location    *LocationId `json:"location,omitempty"`
	Folder      *string     `json:"folder,omitempty"`
	Models      []Model     `json:"models,omitempty"`
	Connectors  []Connector `json:"connectors,omitempty"`
	Props       []Prop      `json:"props,omitempty"`
	Authors     []AuthorId  `json:"authors,omitempty"`
	Concepts    []string    `json:"concepts,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Image       *string     `json:"image,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// TypeDiff represents changes to a type entity.
type TypeDiff struct {
	Name        *string         `json:"name,omitempty"`
	Parent      *TypeId         `json:"parent,omitempty"`
	IsAbstract  *bool           `json:"isAbstract,omitempty"`
	Virtual     *bool           `json:"virtual,omitempty"`
	Unit        *string         `json:"unit,omitempty"`
	Stock       *int            `json:"stock,omitempty"`
	Location    *LocationId     `json:"location,omitempty"`
	Folder      *string         `json:"folder,omitempty"`
	Models      *ModelsDiff     `json:"models,omitempty"`
	Connectors  *ConnectorsDiff `json:"connectors,omitempty"`
	Props       *PropsDiff      `json:"props,omitempty"`
	Authors     []AuthorId      `json:"authors,omitempty"`
	Concepts    []string        `json:"concepts,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Image       *string         `json:"image,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// UnmarshalJSON MUST populate the setFields map for all present JSON keys.
// UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.
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

// HasField MUST return false when setFields is nil.
// HasField returns whether a JSON field was present in the unmarshaled data.
func (d *TypeDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// TypesDiff represents a collection of type additions, removals and updates.
type TypesDiff struct {
	Removed []TypeId `json:"removed,omitempty"`
	Updated []struct {
		Type TypeId   `json:"type"`
		Diff TypeDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Type `json:"added,omitempty"`
}

// #endregion 🔖Type

// #region 🔖Layer
// Layer MUST define layer hierarchy entities and their diff types.

// Layer represents a named layer with visibility, lock and color properties.
type Layer struct {
	Guid        string      `json:"guid"`
	Path        string      `json:"path"`
	IsHidden    *bool       `json:"isHidden,omitempty"`
	IsLocked    *bool       `json:"isLocked,omitempty"`
	Color       *string     `json:"color,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// LayerDiff represents changes to a layer entity.
type LayerDiff struct {
	Path        *string         `json:"path,omitempty"`
	IsHidden    *bool           `json:"isHidden,omitempty"`
	IsLocked    *bool           `json:"isLocked,omitempty"`
	Color       *string         `json:"color,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// LayersDiff represents a collection of layer additions, removals and updates.
type LayersDiff struct {
	Removed []LayerId `json:"removed,omitempty"`
	Updated []struct {
		Layer LayerId   `json:"layer"`
		Diff  LayerDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Layer `json:"added,omitempty"`
}

// #endregion 🔖Layer

// #region 🔖Piece
// Piece MUST define placed piece entities and their diff types.

// Piece represents a placed component instance within a design.
type Piece struct {
	Guid        string      `json:"guid"`
	Name        *string     `json:"name,omitempty"`
	Type        *TypeId     `json:"type,omitempty"`
	Design      *DesignId   `json:"design,omitempty"`
	Plane       *Plane      `json:"plane,omitempty"`
	Center      *Coord      `json:"center,omitempty"`
	Scale       *float64    `json:"scale,omitempty"`
	MirrorPlane *Plane      `json:"mirrorPlane,omitempty"`
	Props       []Prop      `json:"props,omitempty"`
	IsHidden    *bool       `json:"isHidden,omitempty"`
	IsLocked    *bool       `json:"isLocked,omitempty"`
	Color       *string     `json:"color,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// CoordDiff represents changes to a 2D coordinate.
type CoordDiff struct {
	U *float64 `json:"u,omitempty"`
	V *float64 `json:"v,omitempty"`
}

// PlaneDiff represents changes to a 3D plane.
type PlaneDiff struct {
	Origin *PointDiff  `json:"origin,omitempty"`
	XAxis  *VectorDiff `json:"xAxis,omitempty"`
	YAxis  *VectorDiff `json:"yAxis,omitempty"`
}

// PieceDiff represents changes to a piece entity.
type PieceDiff struct {
	Name        *string         `json:"name,omitempty"`
	Type        *TypeId         `json:"type,omitempty"`
	Design      *DesignId       `json:"design,omitempty"`
	Plane       *PlaneDiff      `json:"plane,omitempty"`
	Center      *CoordDiff      `json:"center,omitempty"`
	Scale       *float64        `json:"scale,omitempty"`
	MirrorPlane *PlaneDiff      `json:"mirrorPlane,omitempty"`
	Props       *PropsDiff      `json:"props,omitempty"`
	IsHidden    *bool           `json:"isHidden,omitempty"`
	IsLocked    *bool           `json:"isLocked,omitempty"`
	Color       *string         `json:"color,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// PiecesDiff represents a collection of piece additions, removals and updates.
type PiecesDiff struct {
	Removed []PieceId `json:"removed,omitempty"`
	Updated []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Piece `json:"added,omitempty"`
}

// #endregion 🔖Piece

// #region 🔖Group
// Group MUST define piece grouping entities and their diff types.

// Group represents a named collection of pieces within a design.
type Group struct {
	Guid        string      `json:"guid"`
	Pieces      []PieceId   `json:"pieces,omitempty"`
	Name        *string     `json:"name,omitempty"`
	Color       *string     `json:"color,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// GroupDiff represents changes to a group entity.
type GroupDiff struct {
	Pieces      []PieceId       `json:"pieces,omitempty"`
	Name        *string         `json:"name,omitempty"`
	Color       *string         `json:"color,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// GroupsDiff represents a collection of group additions, removals and updates.
type GroupsDiff struct {
	Removed []GroupId `json:"removed,omitempty"`
	Updated []struct {
		Group GroupId   `json:"group"`
		Diff  GroupDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Group `json:"added,omitempty"`
}

// #endregion 🔖Group

// #region 🔖Side
// Side MUST define connection side reference entities and their diff types.

// Side represents one end of a connection referencing a piece and optional connector.
type Side struct {
	Piece       PieceId      `json:"piece"`
	DesignPiece *PieceId     `json:"designPiece,omitempty"`
	Connector   *ConnectorId `json:"connector,omitempty"`
}

// SideDiff represents changes to a connection side.
type SideDiff struct {
	Piece       *PieceId     `json:"piece,omitempty"`
	DesignPiece *PieceId     `json:"designPiece,omitempty"`
	Connector   *ConnectorId `json:"connector,omitempty"`
}

// #endregion 🔖Side

// #region 🔖Connection
// Connection MUST define spatial connection entities and their diff types.

// Connection represents a spatial relationship between two pieces with transform parameters.
type Connection struct {
	Guid        string      `json:"guid"`
	Connected   Side        `json:"connected"`
	Connecting  Side        `json:"connecting"`
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

// ConnectionDiff represents changes to a connection entity.
type ConnectionDiff struct {
	Connected   *SideDiff       `json:"connected,omitempty"`
	Connecting  *SideDiff       `json:"connecting,omitempty"`
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

// ConnectionsDiff represents a collection of connection additions, removals and updates.
type ConnectionsDiff struct {
	Removed []ConnectionId `json:"removed,omitempty"`
	Updated []struct {
		Connection ConnectionId   `json:"connection"`
		Diff       ConnectionDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Connection `json:"added,omitempty"`
}

// #endregion 🔖Connection

// #region 🔖Stat
// Stat MUST define statistical measure entities and their diff types.

// Stat represents a statistical quality measurement with min and max bounds.
type Stat struct {
	Guid       string      `json:"guid"`
	Quality    QualityId   `json:"quality"`
	Min        *float64    `json:"min,omitempty"`
	Max        *float64    `json:"max,omitempty"`
	Unit       *string     `json:"unit,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
}

// StatDiff represents changes to a stat entity.
type StatDiff struct {
	Quality    *QualityId      `json:"quality,omitempty"`
	Min        *float64        `json:"min,omitempty"`
	Max        *float64        `json:"max,omitempty"`
	Unit       *string         `json:"unit,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// StatsDiff represents a collection of stat additions, removals and updates.
type StatsDiff struct {
	Removed []StatId `json:"removed,omitempty"`
	Updated []struct {
		Stat StatId   `json:"stat"`
		Diff StatDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Stat `json:"added,omitempty"`
}

// #endregion 🔖Stat

// #region 🔖Design
// Design MUST define assembly design entities and their diff types.

// Design represents an assembly of pieces, connections, layers and groups.
type Design struct {
	Guid        string       `json:"guid"`
	Name        string       `json:"name"`
	Parent      *DesignId    `json:"parent,omitempty"`
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
	Concepts    []string     `json:"concepts,omitempty"`
	Icon        *string      `json:"icon,omitempty"`
	Image       *string      `json:"image,omitempty"`
	Description *string      `json:"description,omitempty"`
	Attributes  []Attribute  `json:"attributes,omitempty"`
	CreatedAt   string       `json:"createdAt,omitempty"`
	UpdatedAt   string       `json:"updatedAt,omitempty"`
}

// CameraDiff represents changes to a camera view.
type CameraDiff struct {
	Position *PointDiff  `json:"position,omitempty"`
	Forward  *VectorDiff `json:"forward,omitempty"`
	Up       *VectorDiff `json:"up,omitempty"`
}

// DesignDiff represents changes to a design entity.
type DesignDiff struct {
	Name        *string          `json:"name,omitempty"`
	Parent      *DesignId        `json:"parent,omitempty"`
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
	Concepts    []string         `json:"concepts,omitempty"`
	Icon        *string          `json:"icon,omitempty"`
	Image       *string          `json:"image,omitempty"`
	Description *string          `json:"description,omitempty"`
	Attributes  *AttributesDiff  `json:"attributes,omitempty"`
}

// DesignsDiff represents a collection of design additions, removals and updates.
type DesignsDiff struct {
	Removed []DesignId `json:"removed,omitempty"`
	Updated []struct {
		Design DesignId   `json:"design"`
		Diff   DesignDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Design `json:"added,omitempty"`
}

// #endregion 🔖Design

// #region 🔖Kit
// Kit MUST define the root kit container entity and its diff types.

// Kit represents the root container for all domain entities.
type Kit struct {
	Guid        string      `json:"guid"`
	Name        string      `json:"name"`
	Version     string      `json:"version"`
	Types       []Type      `json:"types,omitempty"`
	Designs     []Design    `json:"designs,omitempty"`
	Tags        []Tag       `json:"tags,omitempty"`
	Concepts    []Concept   `json:"concepts,omitempty"`
	Ports       []Port `json:"ports,omitempty"`
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

// KitDiff represents changes to a kit entity.
type KitDiff struct {
	Name        *string         `json:"name,omitempty"`
	Version     *string         `json:"version,omitempty"`
	Types       *TypesDiff      `json:"types,omitempty"`
	Designs     *DesignsDiff    `json:"designs,omitempty"`
	Tags        *TagsDiff       `json:"tags,omitempty"`
	Concepts    *ConceptsDiff   `json:"concepts,omitempty"`
	Ports       *PortsDiff `json:"ports,omitempty"`
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
}

// KitsDiff represents a collection of kit additions, removals and updates.
type KitsDiff struct {
	Removed []KitId `json:"removed,omitempty"`
	Updated []struct {
		Kit  KitId   `json:"kit"`
		Diff KitDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Kit `json:"added,omitempty"`
}

// #endregion 🔖Kit

// #region 🔖Serialization
// Serialization MUST provide JSON marshaling and unmarshaling for kit data.

// SerializeKit MUST return valid JSON with two-space indentation.
// SerializeKit marshals a kit to indented JSON bytes.
func SerializeKit(kit Kit) ([]byte, error) {
	return json.MarshalIndent(kit, "", "  ")
}

// DeserializeKit MUST return an error if the data is not valid kit JSON.
// DeserializeKit unmarshals JSON bytes into a kit.
func DeserializeKit(data []byte) (Kit, error) {
	var kit Kit
	err := json.Unmarshal(data, &kit)
	return kit, err
}

// SerializeKitDiff MUST return valid JSON with two-space indentation.
// SerializeKitDiff marshals a kit diff to indented JSON bytes.
func SerializeKitDiff(diff KitDiff) ([]byte, error) {
	return json.MarshalIndent(diff, "", "  ")
}

// DeserializeKitDiff MUST return an error if the data is not valid kit diff JSON.
// DeserializeKitDiff unmarshals JSON bytes into a kit diff.
func DeserializeKitDiff(data []byte) (KitDiff, error) {
	var diff KitDiff
	err := json.Unmarshal(data, &diff)
	return diff, err
}

// #endregion 🔖Serialization

// #region 🔖Helpers
// Helpers MUST provide lookup functions for finding entities within kits.

// FindTypeInKit MUST return nil when no type matches the GUID.
// FindTypeInKit returns a pointer to the type with the given GUID or nil.
func FindTypeInKit(kit *Kit, typeGuid string) *Type {
	for i := range kit.Types {
		if kit.Types[i].Guid == typeGuid {
			return &kit.Types[i]
		}
	}
	return nil
}

// FindDesignInKit MUST return nil when no design matches the GUID.
// FindDesignInKit returns a pointer to the design with the given GUID or nil.
func FindDesignInKit(kit *Kit, designGuid string) *Design {
	for i := range kit.Designs {
		if kit.Designs[i].Guid == designGuid {
			return &kit.Designs[i]
		}
	}
	return nil
}

// FindPieceInDesign MUST return nil when no piece matches the GUID.
// FindPieceInDesign returns a pointer to the piece with the given GUID or nil.
func FindPieceInDesign(design *Design, pieceGuid string) *Piece {
	for i := range design.Pieces {
		if design.Pieces[i].Guid == pieceGuid {
			return &design.Pieces[i]
		}
	}
	return nil
}

// FindConnectionInDesign MUST return nil when no connection matches the GUID.
// FindConnectionInDesign returns a pointer to the connection with the given GUID or nil.
func FindConnectionInDesign(design *Design, connectionGuid string) *Connection {
	for i := range design.Connections {
		if design.Connections[i].Guid == connectionGuid {
			return &design.Connections[i]
		}
	}
	return nil
}

// FindConnectorInType MUST return nil when no connector matches the GUID.
// FindConnectorInType returns a pointer to the connector with the given GUID or nil.
func FindConnectorInType(typ *Type, connectorGuid string) *Connector {
	for i := range typ.Connectors {
		if typ.Connectors[i].Guid == connectorGuid {
			return &typ.Connectors[i]
		}
	}
	return nil
}

// FindFileInKit MUST return nil when no file matches the GUID.
// FindFileInKit returns a pointer to the file with the given GUID or nil.
func FindFileInKit(kit *Kit, fileGuid string) *File {
	for i := range kit.Files {
		if kit.Files[i].Guid == fileGuid {
			return &kit.Files[i]
		}
	}
	return nil
}

// FindFolderInKit MUST return nil when no folder matches the GUID.
// FindFolderInKit returns a pointer to the folder with the given GUID or nil.
func FindFolderInKit(kit *Kit, folderGuid string) *Folder {
	for i := range kit.Folders {
		if kit.Folders[i].Guid == folderGuid {
			return &kit.Folders[i]
		}
	}
	return nil
}

// FindQualityInKit MUST return nil when no quality matches the GUID.
// FindQualityInKit returns a pointer to the quality with the given GUID or nil.
func FindQualityInKit(kit *Kit, qualityGuid string) *Quality {
	for i := range kit.Qualities {
		if kit.Qualities[i].Guid == qualityGuid {
			return &kit.Qualities[i]
		}
	}
	return nil
}

// FindPortInKit MUST return nil when no port matches the GUID.
// FindPortInKit returns a pointer to the port with the given GUID or nil.
func FindPortInKit(kit *Kit, interfaceGuid string) *Port {
	for i := range kit.Ports {
		if kit.Ports[i].Guid == interfaceGuid {
			return &kit.Ports[i]
		}
	}
	return nil
}

// FindTagInKit MUST return nil when no tag matches the GUID.
// FindTagInKit returns a pointer to the tag with the given GUID or nil.
func FindTagInKit(kit *Kit, tagGuid string) *Tag {
	for i := range kit.Tags {
		if kit.Tags[i].Guid == tagGuid {
			return &kit.Tags[i]
		}
	}
	return nil
}

// FindConceptInKit MUST return nil when no concept matches the GUID.
// FindConceptInKit returns a pointer to the concept with the given GUID or nil.
func FindConceptInKit(kit *Kit, conceptGuid string) *Concept {
	for i := range kit.Concepts {
		if kit.Concepts[i].Guid == conceptGuid {
			return &kit.Concepts[i]
		}
	}
	return nil
}

// FindAuthorInKit MUST return nil when no author matches the GUID.
// FindAuthorInKit returns a pointer to the author with the given GUID or nil.
func FindAuthorInKit(kit *Kit, authorGuid string) *Author {
	for i := range kit.Authors {
		if kit.Authors[i].Guid == authorGuid {
			return &kit.Authors[i]
		}
	}
	return nil
}

// #endregion 🔖Helpers

// #region 🔖Factories
// Factories MUST provide constructor functions for creating new domain entities.

// NewKit MUST generate a unique GUID and set version to 0.0.1.
// NewKit creates a new kit with the given name and a generated GUID.
func NewKit(name string) Kit {
	now := ""
	return Kit{
		Guid:      Guid(),
		Name:      name,
		Version:   "0.0.1",
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewType MUST generate a unique GUID for the new type.
// NewType creates a new type with the given name and a generated GUID.
func NewType(name string) Type {
	now := ""
	return Type{
		Guid:      Guid(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewDesign MUST generate a unique GUID for the new design.
// NewDesign creates a new design with the given name and a generated GUID.
func NewDesign(name string) Design {
	now := ""
	return Design{
		Guid:      Guid(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewPiece MUST generate a unique GUID for the new piece.
// NewPiece creates a new piece with a generated GUID.
func NewPiece() Piece {
	return Piece{
		Guid: Guid(),
	}
}

// NewConnection MUST generate a unique GUID and set both connected and connecting sides.
// NewConnection creates a new connection between two pieces by their GUIDs.
func NewConnection(connectedPieceGuid, connectingPieceGuid string) Connection {
	return Connection{
		Guid:       Guid(),
		Connected:  Side{Piece: PieceId{Guid: connectedPieceGuid}},
		Connecting: Side{Piece: PieceId{Guid: connectingPieceGuid}},
	}
}

// NewConnector MUST generate a unique GUID for the new connector.
// NewConnector creates a new connector with position, direction and parameter t.
func NewConnector(point Point, direction Vector, t float64) Connector {
	return Connector{
		Guid:      Guid(),
		Point:     point,
		Direction: direction,
		T:         t,
	}
}

// NewFile MUST generate a unique GUID for the new file.
// NewFile creates a new file with the given name and a generated GUID.
func NewFile(name string) File {
	now := ""
	return File{
		Guid:      Guid(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewFolder MUST generate a unique GUID for the new folder.
// NewFolder creates a new folder with the given name and a generated GUID.
func NewFolder(name string) Folder {
	now := ""
	return Folder{
		Guid:      Guid(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewQuality MUST generate a unique GUID for the new quality.
// NewQuality creates a new quality with the given key, name and a generated GUID.
func NewQuality(key, name string) Quality {
	now := ""
	return Quality{
		Guid:      Guid(),
		Key:       key,
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewPort MUST generate a unique GUID for the new port.
// NewPort creates a new port with the given name and a generated GUID.
func NewPort(name string) Port {
	now := ""
	return Port{
		Guid:      Guid(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewTag MUST generate a unique GUID for the new tag.
// NewTag creates a new tag with the given name and a generated GUID.
func NewTag(name string) Tag {
	now := ""
	return Tag{
		Guid:      Guid(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewConcept MUST generate a unique GUID for the new concept.
// NewConcept creates a new concept with the given name and a generated GUID.
func NewConcept(name string) Concept {
	now := ""
	return Concept{
		Guid:      Guid(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// NewAuthor MUST generate a unique GUID for the new author.
// NewAuthor creates a new author with the given name and a generated GUID.
func NewAuthor(name string) Author {
	now := ""
	return Author{
		Guid:      Guid(),
		Name:      name,
		CreatedAt: now,
		UpdatedAt: now,
	}
}

// #endregion 🔖Factories

// #region 🔖Kit Operations
// Kit Operations MUST provide comparison, diffing, and application of kit changes.

// AreKitsEqual MUST compare all entities by GUID and structural fields.
// AreKitsEqual compares two kits for structural equality.
func AreKitsEqual(a, b Kit) bool {
	if a.Guid != b.Guid || a.Name != b.Name || a.Version != b.Version {
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
			if ta.Guid == tb.Guid {
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
			if da.Guid == db.Guid {
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
			if ta.Guid == tb.Guid {
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
			if ca.Guid == cb.Guid {
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
	if len(a.Ports) != len(b.Ports) {
		return false
	}
	for _, ia := range a.Ports {
		found := false
		for _, ib := range b.Ports {
			if ia.Guid == ib.Guid {
				if !arePortsEqual(ia, ib) {
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
			if fa.Guid == fb.Guid {
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
			if fa.Guid == fb.Guid {
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
			if aa.Guid == ab.Guid {
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

// AreKitDiffsEqual MUST compare all diff fields including nested entity diffs.
// AreKitDiffsEqual compares two kit diffs for structural equality.
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
	if !areTypesDiffsEqual(a.Types, b.Types) {
		return false
	}
	if !areDesignsDiffsEqual(a.Designs, b.Designs) {
		return false
	}
	if !areTagsDiffsEqual(a.Tags, b.Tags) {
		return false
	}
	if !areConceptsDiffsEqual(a.Concepts, b.Concepts) {
		return false
	}
	if !arePortsDiffsEqual(a.Ports, b.Ports) {
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
		if a.Added[i].Guid != b.Added[i].Guid {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Guid != b.Removed[i].Guid {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Type.Guid != b.Updated[i].Type.Guid {
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
		if a.Added[i].Guid != b.Added[i].Guid {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Guid != b.Removed[i].Guid {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Design.Guid != b.Updated[i].Design.Guid {
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
		if a.Added[i].Guid != b.Added[i].Guid {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Guid != b.Removed[i].Guid {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Tag.Guid != b.Updated[i].Tag.Guid {
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
		if a.Added[i].Guid != b.Added[i].Guid {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Guid != b.Removed[i].Guid {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Concept.Guid != b.Updated[i].Concept.Guid {
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
		if a.Added[i].Guid != b.Added[i].Guid {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Guid != b.Removed[i].Guid {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Port.Guid != b.Updated[i].Port.Guid {
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
		if a.Added[i].Guid != b.Added[i].Guid {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Guid != b.Removed[i].Guid {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].File.Guid != b.Updated[i].File.Guid {
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
		if a.Added[i].Guid != b.Added[i].Guid {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Guid != b.Removed[i].Guid {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Folder.Guid != b.Updated[i].Folder.Guid {
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
		if a.Added[i].Guid != b.Added[i].Guid {
			return false
		}
	}
	if len(a.Removed) != len(b.Removed) {
		return false
	}
	for i := range a.Removed {
		if a.Removed[i].Guid != b.Removed[i].Guid {
			return false
		}
	}
	if len(a.Updated) != len(b.Updated) {
		return false
	}
	for i := range a.Updated {
		if a.Updated[i].Author.Guid != b.Updated[i].Author.Guid {
			return false
		}
	}
	return true
}

// GetKitDiff MUST return a diff that when applied to before produces after.
// GetKitDiff computes the diff between a before and after kit state.
func GetKitDiff(before, after Kit) KitDiff {
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
	typesDiff := getTypesDiff(before.Types, after.Types)
	if len(typesDiff.Added) > 0 || len(typesDiff.Removed) > 0 || len(typesDiff.Updated) > 0 {
		diff.Types = &typesDiff
	}
	designsDiff := getDesignsDiff(before.Designs, after.Designs)
	if len(designsDiff.Added) > 0 || len(designsDiff.Removed) > 0 || len(designsDiff.Updated) > 0 {
		diff.Designs = &designsDiff
	}
	tagsDiff := getTagsDiff(before.Tags, after.Tags)
	if len(tagsDiff.Added) > 0 || len(tagsDiff.Removed) > 0 || len(tagsDiff.Updated) > 0 {
		diff.Tags = &tagsDiff
	}
	conceptsDiff := getConceptsDiff(before.Concepts, after.Concepts)
	if len(conceptsDiff.Added) > 0 || len(conceptsDiff.Removed) > 0 || len(conceptsDiff.Updated) > 0 {
		diff.Concepts = &conceptsDiff
	}
	interfacesDiff := getPortsDiff(before.Ports, after.Ports)
	if len(interfacesDiff.Added) > 0 || len(interfacesDiff.Removed) > 0 || len(interfacesDiff.Updated) > 0 {
		diff.Ports = &interfacesDiff
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
	return diff
}

func getTypesDiff(before, after []Type) TypesDiff {
	diff := TypesDiff{}
	beforeMap := make(map[string]Type)
	for _, t := range before {
		beforeMap[t.Guid] = t
	}
	afterMap := make(map[string]Type)
	for _, t := range after {
		afterMap[t.Guid] = t
	}
	for _, t := range before {
		if _, ok := afterMap[t.Guid]; !ok {
			diff.Removed = append(diff.Removed, TypeId{Guid: t.Guid})
		}
	}
	for _, t := range after {
		if _, ok := beforeMap[t.Guid]; !ok {
			diff.Added = append(diff.Added, t)
		} else {
			typeDiff := getTypeDiff(beforeMap[t.Guid], t)
			if !isTypeDiffEmpty(typeDiff) {
				diff.Updated = append(diff.Updated, struct {
					Type TypeId   `json:"type"`
					Diff TypeDiff `json:"diff"`
				}{Type: TypeId{Guid: t.Guid}, Diff: typeDiff})
			}
		}
	}
	return diff
}

func getTypeDiff(before, after Type) TypeDiff {
	diff := TypeDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	return diff
}

func isTypeDiffEmpty(diff TypeDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Virtual == nil && diff.Unit == nil && diff.Connectors == nil && diff.Models == nil
}

func getDesignsDiff(before, after []Design) DesignsDiff {
	diff := DesignsDiff{}
	beforeMap := make(map[string]Design)
	for _, d := range before {
		beforeMap[d.Guid] = d
	}
	afterMap := make(map[string]Design)
	for _, d := range after {
		afterMap[d.Guid] = d
	}
	for _, d := range before {
		if _, ok := afterMap[d.Guid]; !ok {
			diff.Removed = append(diff.Removed, DesignId{Guid: d.Guid})
		}
	}
	for _, d := range after {
		if _, ok := beforeMap[d.Guid]; !ok {
			diff.Added = append(diff.Added, d)
		} else {
			designDiff := getDesignDiff(beforeMap[d.Guid], d)
			if !isDesignDiffEmpty(designDiff) {
				diff.Updated = append(diff.Updated, struct {
					Design DesignId   `json:"design"`
					Diff   DesignDiff `json:"diff"`
				}{Design: DesignId{Guid: d.Guid}, Diff: designDiff})
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
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	return diff
}

func isDesignDiffEmpty(diff DesignDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Pieces == nil && diff.Connections == nil
}

func getTagsDiff(before, after []Tag) TagsDiff {
	diff := TagsDiff{}
	beforeMap := make(map[string]Tag)
	for _, t := range before {
		beforeMap[t.Guid] = t
	}
	afterMap := make(map[string]Tag)
	for _, t := range after {
		afterMap[t.Guid] = t
	}
	for _, t := range before {
		if _, ok := afterMap[t.Guid]; !ok {
			diff.Removed = append(diff.Removed, TagId{Guid: t.Guid})
		}
	}
	for _, t := range after {
		if _, ok := beforeMap[t.Guid]; !ok {
			diff.Added = append(diff.Added, t)
		} else {
			tagDiff := getTagDiff(beforeMap[t.Guid], t)
			if !isTagDiffEmpty(tagDiff) {
				diff.Updated = append(diff.Updated, struct {
					Tag  TagId   `json:"tag"`
					Diff TagDiff `json:"diff"`
				}{Tag: TagId{Guid: t.Guid}, Diff: tagDiff})
			}
		}
	}
	return diff
}

func getTagDiff(before, after Tag) TagDiff {
	diff := TagDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	return diff
}

func isTagDiffEmpty(diff TagDiff) bool {
	return diff.Name == nil && diff.Description == nil
}

func getConceptsDiff(before, after []Concept) ConceptsDiff {
	diff := ConceptsDiff{}
	beforeMap := make(map[string]Concept)
	for _, c := range before {
		beforeMap[c.Guid] = c
	}
	afterMap := make(map[string]Concept)
	for _, c := range after {
		afterMap[c.Guid] = c
	}
	for _, c := range before {
		if _, ok := afterMap[c.Guid]; !ok {
			diff.Removed = append(diff.Removed, ConceptId{Guid: c.Guid})
		}
	}
	for _, c := range after {
		if _, ok := beforeMap[c.Guid]; !ok {
			diff.Added = append(diff.Added, c)
		} else {
			conceptDiff := getConceptDiff(beforeMap[c.Guid], c)
			if !isConceptDiffEmpty(conceptDiff) {
				diff.Updated = append(diff.Updated, struct {
					Concept ConceptId   `json:"concept"`
					Diff    ConceptDiff `json:"diff"`
				}{Concept: ConceptId{Guid: c.Guid}, Diff: conceptDiff})
			}
		}
	}
	return diff
}

func getConceptDiff(before, after Concept) ConceptDiff {
	diff := ConceptDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	return diff
}

func isConceptDiffEmpty(diff ConceptDiff) bool {
	return diff.Name == nil && diff.Description == nil
}

func getPortsDiff(before, after []Port) PortsDiff {
	diff := PortsDiff{}
	beforeMap := make(map[string]Port)
	for _, i := range before {
		beforeMap[i.Guid] = i
	}
	afterMap := make(map[string]Port)
	for _, i := range after {
		afterMap[i.Guid] = i
	}
	for _, i := range before {
		if _, ok := afterMap[i.Guid]; !ok {
			diff.Removed = append(diff.Removed, PortId{Guid: i.Guid})
		}
	}
	for _, i := range after {
		if _, ok := beforeMap[i.Guid]; !ok {
			diff.Added = append(diff.Added, i)
		} else {
			interfaceDiff := getPortDiff(beforeMap[i.Guid], i)
			if !isPortDiffEmpty(interfaceDiff) {
				diff.Updated = append(diff.Updated, struct {
					Port PortId   `json:"port"`
					Diff PortDiff `json:"diff"`
				}{Port: PortId{Guid: i.Guid}, Diff: interfaceDiff})
			}
		}
	}
	return diff
}

func getPortDiff(before, after Port) PortDiff {
	diff := PortDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if normalizeStr(before.Description) != normalizeStr(after.Description) {
		diff.Description = after.Description
	}
	return diff
}

func isPortDiffEmpty(diff PortDiff) bool {
	return diff.Name == nil && diff.Description == nil
}

func getFilesDiff(before, after []File) FilesDiff {
	diff := FilesDiff{}
	beforeMap := make(map[string]File)
	for _, f := range before {
		beforeMap[f.Guid] = f
	}
	afterMap := make(map[string]File)
	for _, f := range after {
		afterMap[f.Guid] = f
	}
	for _, f := range before {
		if _, ok := afterMap[f.Guid]; !ok {
			diff.Removed = append(diff.Removed, FileId{Guid: f.Guid})
		}
	}
	for _, f := range after {
		if _, ok := beforeMap[f.Guid]; !ok {
			diff.Added = append(diff.Added, f)
		} else {
			fileDiff := getFileDiff(beforeMap[f.Guid], f)
			if !isFileDiffEmpty(fileDiff) {
				diff.Updated = append(diff.Updated, struct {
					File FileId   `json:"file"`
					Diff FileDiff `json:"diff"`
				}{File: FileId{Guid: f.Guid}, Diff: fileDiff})
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
	return diff
}

func isFileDiffEmpty(diff FileDiff) bool {
	return diff.Name == nil && diff.Remote == nil
}

func getFoldersDiff(before, after []Folder) FoldersDiff {
	diff := FoldersDiff{}
	beforeMap := make(map[string]Folder)
	for _, f := range before {
		beforeMap[f.Guid] = f
	}
	afterMap := make(map[string]Folder)
	for _, f := range after {
		afterMap[f.Guid] = f
	}
	for _, f := range before {
		if _, ok := afterMap[f.Guid]; !ok {
			diff.Removed = append(diff.Removed, FolderId{Guid: f.Guid})
		}
	}
	for _, f := range after {
		if _, ok := beforeMap[f.Guid]; !ok {
			diff.Added = append(diff.Added, f)
		} else {
			folderDiff := getFolderDiff(beforeMap[f.Guid], f)
			if !isFolderDiffEmpty(folderDiff) {
				diff.Updated = append(diff.Updated, struct {
					Folder FolderId   `json:"folder"`
					Diff   FolderDiff `json:"diff"`
				}{Folder: FolderId{Guid: f.Guid}, Diff: folderDiff})
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
	return diff
}

func isFolderDiffEmpty(diff FolderDiff) bool {
	return diff.Name == nil
}

func getAuthorsDiff(before, after []Author) AuthorsDiff {
	diff := AuthorsDiff{}
	beforeMap := make(map[string]Author)
	for _, a := range before {
		beforeMap[a.Guid] = a
	}
	afterMap := make(map[string]Author)
	for _, a := range after {
		afterMap[a.Guid] = a
	}
	for _, a := range before {
		if _, ok := afterMap[a.Guid]; !ok {
			diff.Removed = append(diff.Removed, AuthorId{Guid: a.Guid})
		}
	}
	for _, a := range after {
		if _, ok := beforeMap[a.Guid]; !ok {
			diff.Added = append(diff.Added, a)
		} else {
			authorDiff := getAuthorDiff(beforeMap[a.Guid], a)
			if !isAuthorDiffEmpty(authorDiff) {
				diff.Updated = append(diff.Updated, struct {
					Author AuthorId   `json:"author"`
					Diff   AuthorDiff `json:"diff"`
				}{Author: AuthorId{Guid: a.Guid}, Diff: authorDiff})
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
	return diff
}

func isAuthorDiffEmpty(diff AuthorDiff) bool {
	return diff.Name == nil && diff.Email == nil
}

// InverseKitDiff MUST return a diff that when applied restores the original state.
// InverseKitDiff computes the reverse diff that undoes an applied diff.
func InverseKitDiff(original Kit, appliedDiff KitDiff) KitDiff {
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
	if appliedDiff.Types != nil {
		typesDiff := inverseTypesDiff(original.Types, *appliedDiff.Types)
		inverse.Types = &typesDiff
	}
	if appliedDiff.Designs != nil {
		designsDiff := inverseDesignsDiff(original.Designs, *appliedDiff.Designs)
		inverse.Designs = &designsDiff
	}
	if appliedDiff.Tags != nil {
		tagsDiff := inverseTagsDiff(original.Tags, *appliedDiff.Tags)
		inverse.Tags = &tagsDiff
	}
	if appliedDiff.Concepts != nil {
		conceptsDiff := inverseConceptsDiff(original.Concepts, *appliedDiff.Concepts)
		inverse.Concepts = &conceptsDiff
	}
	if appliedDiff.Ports != nil {
		interfacesDiff := inversePortsDiff(original.Ports, *appliedDiff.Ports)
		inverse.Ports = &interfacesDiff
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
	return inverse
}

func inverseTypesDiff(original []Type, appliedDiff TypesDiff) TypesDiff {
	inverse := TypesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, TypeId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, t := range original {
			if t.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, t)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, t := range original {
			if t.Guid == updated.Type.Guid {
				inverseDiff := inverseTypeDiff(t, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Type TypeId   `json:"type"`
					Diff TypeDiff `json:"diff"`
				}{Type: TypeId{Guid: t.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseTypeDiff(original Type, appliedDiff TypeDiff) TypeDiff {
	inverse := TypeDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	return inverse
}

func inverseDesignsDiff(original []Design, appliedDiff DesignsDiff) DesignsDiff {
	inverse := DesignsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, DesignId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, d := range original {
			if d.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, d)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, d := range original {
			if d.Guid == updated.Design.Guid {
				inverseDiff := inverseDesignDiff(d, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Design DesignId   `json:"design"`
					Diff   DesignDiff `json:"diff"`
				}{Design: DesignId{Guid: d.Guid}, Diff: inverseDiff})
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
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	return inverse
}

func inverseTagsDiff(original []Tag, appliedDiff TagsDiff) TagsDiff {
	inverse := TagsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, TagId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, t := range original {
			if t.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, t)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, t := range original {
			if t.Guid == updated.Tag.Guid {
				inverseDiff := inverseTagDiff(t, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Tag  TagId   `json:"tag"`
					Diff TagDiff `json:"diff"`
				}{Tag: TagId{Guid: t.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseTagDiff(original Tag, appliedDiff TagDiff) TagDiff {
	inverse := TagDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	return inverse
}

func inverseConceptsDiff(original []Concept, appliedDiff ConceptsDiff) ConceptsDiff {
	inverse := ConceptsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, ConceptId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, c := range original {
			if c.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, c)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, c := range original {
			if c.Guid == updated.Concept.Guid {
				inverseDiff := inverseConceptDiff(c, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Concept ConceptId   `json:"concept"`
					Diff    ConceptDiff `json:"diff"`
				}{Concept: ConceptId{Guid: c.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inverseConceptDiff(original Concept, appliedDiff ConceptDiff) ConceptDiff {
	inverse := ConceptDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	return inverse
}

func inversePortsDiff(original []Port, appliedDiff PortsDiff) PortsDiff {
	inverse := PortsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, PortId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, i := range original {
			if i.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, i)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, i := range original {
			if i.Guid == updated.Port.Guid {
				inverseDiff := inversePortDiff(i, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Port PortId   `json:"port"`
					Diff PortDiff `json:"diff"`
				}{Port: PortId{Guid: i.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

func inversePortDiff(original Port, appliedDiff PortDiff) PortDiff {
	inverse := PortDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Description != nil {
		inverse.Description = original.Description
	}
	return inverse
}

func inverseFilesDiff(original []File, appliedDiff FilesDiff) FilesDiff {
	inverse := FilesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, FileId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, f := range original {
			if f.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, f)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, f := range original {
			if f.Guid == updated.File.Guid {
				inverseDiff := inverseFileDiff(f, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					File FileId   `json:"file"`
					Diff FileDiff `json:"diff"`
				}{File: FileId{Guid: f.Guid}, Diff: inverseDiff})
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
	return inverse
}

func inverseFoldersDiff(original []Folder, appliedDiff FoldersDiff) FoldersDiff {
	inverse := FoldersDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, FolderId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, f := range original {
			if f.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, f)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, f := range original {
			if f.Guid == updated.Folder.Guid {
				inverseDiff := inverseFolderDiff(f, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Folder FolderId   `json:"folder"`
					Diff   FolderDiff `json:"diff"`
				}{Folder: FolderId{Guid: f.Guid}, Diff: inverseDiff})
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
	return inverse
}

func inverseAuthorsDiff(original []Author, appliedDiff AuthorsDiff) AuthorsDiff {
	inverse := AuthorsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, AuthorId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, a := range original {
			if a.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, a)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, a := range original {
			if a.Guid == updated.Author.Guid {
				inverseDiff := inverseAuthorDiff(a, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Author AuthorId   `json:"author"`
					Diff   AuthorDiff `json:"diff"`
				}{Author: AuthorId{Guid: a.Guid}, Diff: inverseDiff})
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
	return inverse
}

func normalizeStr(s *string) string {
	if s == nil {
		return ""
	}
	return *s
}

func areTypesEqual(a, b Type) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if len(a.Connectors) != len(b.Connectors) {
		return false
	}
	for _, ca := range a.Connectors {
		found := false
		for _, cb := range b.Connectors {
			if ca.Guid == cb.Guid {
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
	if len(a.Models) != len(b.Models) {
		return false
	}
	for _, ma := range a.Models {
		found := false
		for _, mb := range b.Models {
			if ma.Guid == mb.Guid {
				if !areModelsEqual(ma, mb) {
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

func areConnectorsEqual(a, b Connector) bool {
	if normalizeStr(a.Name) != normalizeStr(b.Name) {
		return false
	}
	if a.Point.X != b.Point.X || a.Point.Y != b.Point.Y || a.Point.Z != b.Point.Z {
		return false
	}
	if a.Direction.X != b.Direction.X || a.Direction.Y != b.Direction.Y || a.Direction.Z != b.Direction.Z {
		return false
	}
	if a.T != b.T {
		return false
	}
	return true
}

func areModelsEqual(a, b Model) bool {
	if normalizeStr(a.Name) != normalizeStr(b.Name) {
		return false
	}
	if a.File.Guid != b.File.Guid {
		return false
	}
	return true
}

func areDesignsEqual(a, b Design) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if len(a.Pieces) != len(b.Pieces) {
		return false
	}
	for _, pa := range a.Pieces {
		found := false
		for _, pb := range b.Pieces {
			if pa.Guid == pb.Guid {
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
			if ca.Guid == cb.Guid {
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
	return true
}

func arePiecesEqual(a, b Piece) bool {
	if normalizeStr(a.Name) != normalizeStr(b.Name) {
		return false
	}
	if (a.Type == nil) != (b.Type == nil) {
		return false
	}
	if a.Type != nil && a.Type.Guid != b.Type.Guid {
		return false
	}
	if (a.Scale == nil) != (b.Scale == nil) {
		return false
	}
	if a.Scale != nil && *a.Scale != *b.Scale {
		return false
	}
	return true
}

func areConnectionsEqual(a, b Connection) bool {
	if a.Connected.Piece.Guid != b.Connected.Piece.Guid {
		return false
	}
	if a.Connecting.Piece.Guid != b.Connecting.Piece.Guid {
		return false
	}
	if a.Gap != b.Gap || a.Shift != b.Shift || a.Rise != b.Rise {
		return false
	}
	if a.Rotation != b.Rotation || a.Turn != b.Turn || a.Tilt != b.Tilt {
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
	return true
}

func areConceptsEqual(a, b Concept) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
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
	return true
}

func areFilesEqual(a, b File) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Remote) != normalizeStr(b.Remote) {
		return false
	}
	return true
}

func areFoldersEqual(a, b Folder) bool {
	if a.Name != b.Name {
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
	return true
}

// ApplyKitDiff MUST apply all additions, removals and updates from the diff.
// ApplyKitDiff applies a diff to a base kit producing the updated kit.
func ApplyKitDiff(base Kit, diff KitDiff) Kit {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Version != nil {
		result.Version = *diff.Version
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Icon != nil {
		result.Icon = diff.Icon
	}
	if diff.Image != nil {
		result.Image = diff.Image
	}
	if diff.Remote != nil {
		result.Remote = diff.Remote
	}
	if diff.Homepage != nil {
		result.Homepage = diff.Homepage
	}
	if diff.License != nil {
		result.License = diff.License
	}
	if diff.Preview != nil {
		result.Preview = diff.Preview
	}
	if diff.Types != nil {
		result.Types = applyTypesDiff(base.Types, *diff.Types)
	}
	if diff.Designs != nil {
		result.Designs = applyDesignsDiff(base.Designs, *diff.Designs)
	}
	if diff.Tags != nil {
		result.Tags = applyTagsDiff(base.Tags, *diff.Tags)
	}
	if diff.Concepts != nil {
		result.Concepts = applyConceptsDiff(base.Concepts, *diff.Concepts)
	}
	if diff.Ports != nil {
		result.Ports = applyPortsDiff(base.Ports, *diff.Ports)
	}
	if diff.Files != nil {
		result.Files = applyFilesDiff(base.Files, *diff.Files)
	}
	if diff.Folders != nil {
		result.Folders = applyFoldersDiff(base.Folders, *diff.Folders)
	}
	if diff.Authors != nil {
		result.Authors = applyAuthorsDiff(base.Authors, *diff.Authors)
	}
	return result
}

func applyTypesDiff(base []Type, diff TypesDiff) []Type {
	result := make([]Type, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]TypeDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Type.Guid] = u.Diff
	}
	for _, t := range base {
		if removedGuids[t.Guid] {
			continue
		}
		if d, ok := updatedDiffs[t.Guid]; ok {
			result = append(result, applyTypeDiff(t, d))
		} else {
			result = append(result, t)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyTypeDiff(base Type, diff TypeDiff) Type {
	result := base
	if diff.HasField("name") && diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.HasField("description") {
		result.Description = diff.Description
	}
	if diff.HasField("virtual") {
		result.Virtual = diff.Virtual
	}
	if diff.HasField("unit") {
		result.Unit = diff.Unit
	}
	if diff.Connectors != nil {
		result.Connectors = applyConnectorsDiff(base.Connectors, *diff.Connectors)
	}
	if diff.Models != nil {
		result.Models = applyModelsDiff(base.Models, *diff.Models)
	}
	return result
}

func applyConnectorsDiff(base []Connector, diff ConnectorsDiff) []Connector {
	result := make([]Connector, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]ConnectorDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Connector.Guid] = u.Diff
	}
	for _, c := range base {
		if removedGuids[c.Guid] {
			continue
		}
		if d, ok := updatedDiffs[c.Guid]; ok {
			result = append(result, applyConnectorDiff(c, d))
		} else {
			result = append(result, c)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyConnectorDiff(base Connector, diff ConnectorDiff) Connector {
	result := base
	if diff.Name != nil {
		result.Name = diff.Name
	}
	if diff.T != nil {
		result.T = *diff.T
	}
	if diff.Point != nil {
		if diff.Point.X != nil {
			result.Point.X = *diff.Point.X
		}
		if diff.Point.Y != nil {
			result.Point.Y = *diff.Point.Y
		}
		if diff.Point.Z != nil {
			result.Point.Z = *diff.Point.Z
		}
	}
	if diff.Direction != nil {
		if diff.Direction.X != nil {
			result.Direction.X = *diff.Direction.X
		}
		if diff.Direction.Y != nil {
			result.Direction.Y = *diff.Direction.Y
		}
		if diff.Direction.Z != nil {
			result.Direction.Z = *diff.Direction.Z
		}
	}
	return result
}

func applyModelsDiff(base []Model, diff ModelsDiff) []Model {
	result := make([]Model, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]ModelDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Model.Guid] = u.Diff
	}
	for _, m := range base {
		if removedGuids[m.Guid] {
			continue
		}
		if d, ok := updatedDiffs[m.Guid]; ok {
			result = append(result, applyModelDiff(m, d))
		} else {
			result = append(result, m)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyModelDiff(base Model, diff ModelDiff) Model {
	result := base
	if diff.Name != nil {
		result.Name = diff.Name
	}
	if diff.File != nil {
		result.File = *diff.File
	}
	return result
}

func applyDesignsDiff(base []Design, diff DesignsDiff) []Design {
	result := make([]Design, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]DesignDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Design.Guid] = u.Diff
	}
	for _, d := range base {
		if removedGuids[d.Guid] {
			continue
		}
		if df, ok := updatedDiffs[d.Guid]; ok {
			result = append(result, applyDesignDiff(d, df))
		} else {
			result = append(result, d)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyDesignDiff(base Design, diff DesignDiff) Design {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Pieces != nil {
		result.Pieces = applyPiecesDiff(base.Pieces, *diff.Pieces)
	}
	if diff.Connections != nil {
		result.Connections = applyConnectionsDiff(base.Connections, *diff.Connections)
	}
	return result
}

func applyPiecesDiff(base []Piece, diff PiecesDiff) []Piece {
	result := make([]Piece, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]PieceDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Piece.Guid] = u.Diff
	}
	for _, p := range base {
		if removedGuids[p.Guid] {
			continue
		}
		if d, ok := updatedDiffs[p.Guid]; ok {
			result = append(result, applyPieceDiff(p, d))
		} else {
			result = append(result, p)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyPieceDiff(base Piece, diff PieceDiff) Piece {
	result := base
	if diff.Name != nil {
		result.Name = diff.Name
	}
	if diff.Type != nil {
		result.Type = diff.Type
	}
	if diff.Scale != nil {
		result.Scale = diff.Scale
	}
	if diff.Plane != nil {
		if result.Plane == nil {
			result.Plane = &Plane{}
		}
		if diff.Plane.Origin != nil {
			if diff.Plane.Origin.X != nil {
				result.Plane.Origin.X = *diff.Plane.Origin.X
			}
			if diff.Plane.Origin.Y != nil {
				result.Plane.Origin.Y = *diff.Plane.Origin.Y
			}
			if diff.Plane.Origin.Z != nil {
				result.Plane.Origin.Z = *diff.Plane.Origin.Z
			}
		}
		if diff.Plane.XAxis != nil {
			if diff.Plane.XAxis.X != nil {
				result.Plane.XAxis.X = *diff.Plane.XAxis.X
			}
			if diff.Plane.XAxis.Y != nil {
				result.Plane.XAxis.Y = *diff.Plane.XAxis.Y
			}
			if diff.Plane.XAxis.Z != nil {
				result.Plane.XAxis.Z = *diff.Plane.XAxis.Z
			}
		}
		if diff.Plane.YAxis != nil {
			if diff.Plane.YAxis.X != nil {
				result.Plane.YAxis.X = *diff.Plane.YAxis.X
			}
			if diff.Plane.YAxis.Y != nil {
				result.Plane.YAxis.Y = *diff.Plane.YAxis.Y
			}
			if diff.Plane.YAxis.Z != nil {
				result.Plane.YAxis.Z = *diff.Plane.YAxis.Z
			}
		}
	}
	if diff.Center != nil {
		if result.Center == nil {
			result.Center = &Coord{}
		}
		if diff.Center.U != nil {
			result.Center.U = *diff.Center.U
		}
		if diff.Center.V != nil {
			result.Center.V = *diff.Center.V
		}
	}
	return result
}

func applyConnectionsDiff(base []Connection, diff ConnectionsDiff) []Connection {
	result := make([]Connection, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]ConnectionDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Connection.Guid] = u.Diff
	}
	for _, c := range base {
		if removedGuids[c.Guid] {
			continue
		}
		if d, ok := updatedDiffs[c.Guid]; ok {
			result = append(result, applyConnectionDiff(c, d))
		} else {
			result = append(result, c)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyConnectionDiff(base Connection, diff ConnectionDiff) Connection {
	result := base
	if diff.Gap != nil {
		result.Gap = *diff.Gap
	}
	if diff.Shift != nil {
		result.Shift = *diff.Shift
	}
	if diff.Rise != nil {
		result.Rise = *diff.Rise
	}
	if diff.Rotation != nil {
		result.Rotation = *diff.Rotation
	}
	if diff.Turn != nil {
		result.Turn = *diff.Turn
	}
	if diff.Tilt != nil {
		result.Tilt = *diff.Tilt
	}
	return result
}

func applyTagsDiff(base []Tag, diff TagsDiff) []Tag {
	result := make([]Tag, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]TagDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Tag.Guid] = u.Diff
	}
	for _, t := range base {
		if removedGuids[t.Guid] {
			continue
		}
		if d, ok := updatedDiffs[t.Guid]; ok {
			result = append(result, applyTagDiff(t, d))
		} else {
			result = append(result, t)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyTagDiff(base Tag, diff TagDiff) Tag {
	result := base
	if diff.HasField("name") && diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.HasField("description") {
		result.Description = diff.Description
	}
	return result
}

func applyConceptsDiff(base []Concept, diff ConceptsDiff) []Concept {
	result := make([]Concept, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]ConceptDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Concept.Guid] = u.Diff
	}
	for _, c := range base {
		if removedGuids[c.Guid] {
			continue
		}
		if d, ok := updatedDiffs[c.Guid]; ok {
			result = append(result, applyConceptDiff(c, d))
		} else {
			result = append(result, c)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyConceptDiff(base Concept, diff ConceptDiff) Concept {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.HasField("description") {
		result.Description = diff.Description
	}
	return result
}

func applyPortsDiff(base []Port, diff PortsDiff) []Port {
	result := make([]Port, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]PortDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Port.Guid] = u.Diff
	}
	for _, i := range base {
		if removedGuids[i.Guid] {
			continue
		}
		if d, ok := updatedDiffs[i.Guid]; ok {
			result = append(result, applyPortDiff(i, d))
		} else {
			result = append(result, i)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyPortDiff(base Port, diff PortDiff) Port {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.HasField("description") {
		result.Description = diff.Description
	}
	return result
}

func applyFilesDiff(base []File, diff FilesDiff) []File {
	result := make([]File, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]FileDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.File.Guid] = u.Diff
	}
	for _, f := range base {
		if removedGuids[f.Guid] {
			continue
		}
		if d, ok := updatedDiffs[f.Guid]; ok {
			result = append(result, applyFileDiff(f, d))
		} else {
			result = append(result, f)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyFileDiff(base File, diff FileDiff) File {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Remote != nil {
		result.Remote = diff.Remote
	}
	return result
}

func applyFoldersDiff(base []Folder, diff FoldersDiff) []Folder {
	result := make([]Folder, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]FolderDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Folder.Guid] = u.Diff
	}
	for _, f := range base {
		if removedGuids[f.Guid] {
			continue
		}
		if d, ok := updatedDiffs[f.Guid]; ok {
			result = append(result, applyFolderDiff(f, d))
		} else {
			result = append(result, f)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyFolderDiff(base Folder, diff FolderDiff) Folder {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	return result
}

func applyAuthorsDiff(base []Author, diff AuthorsDiff) []Author {
	result := make([]Author, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]AuthorDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Author.Guid] = u.Diff
	}
	for _, a := range base {
		if removedGuids[a.Guid] {
			continue
		}
		if d, ok := updatedDiffs[a.Guid]; ok {
			result = append(result, applyAuthorDiff(a, d))
		} else {
			result = append(result, a)
		}
	}
	result = append(result, diff.Added...)
	return result
}

func applyAuthorDiff(base Author, diff AuthorDiff) Author {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Email != nil {
		result.Email = diff.Email
	}
	return result
}

// FilterDesignsWithoutParent MUST exclude all designs that have a non-nil parent.
// FilterDesignsWithoutParent returns only root-level designs with no parent.
func FilterDesignsWithoutParent(designs []Design) []Design {
	result := make([]Design, 0)
	for _, d := range designs {
		if d.Parent == nil {
			result = append(result, d)
		}
	}
	return result
}

// #endregion 🔖Kit Operations

// #region 🔖Kit Diff Helpers
// Kit Diff Helpers MUST provide convenience functions for single-entity kit diffs.

// AddTypeToKit MUST return a diff with exactly one added type.
// AddTypeToKit creates a diff that adds a single type to a kit.
func AddTypeToKit(typ Type) KitDiff {
	return KitDiff{
		Types: &TypesDiff{
			Added: []Type{typ},
		},
	}
}

// RemoveTypeFromKit MUST return a diff with exactly one removed type ID.
// RemoveTypeFromKit creates a diff that removes a type by GUID.
func RemoveTypeFromKit(typeGuid string) KitDiff {
	return KitDiff{
		Types: &TypesDiff{
			Removed: []TypeId{{Guid: typeGuid}},
		},
	}
}

// AddDesignToKit MUST return a diff with exactly one added design.
// AddDesignToKit creates a diff that adds a single design to a kit.
func AddDesignToKit(design Design) KitDiff {
	return KitDiff{
		Designs: &DesignsDiff{
			Added: []Design{design},
		},
	}
}

// RemoveDesignFromKit MUST return a diff with exactly one removed design ID.
// RemoveDesignFromKit creates a diff that removes a design by GUID.
func RemoveDesignFromKit(designGuid string) KitDiff {
	return KitDiff{
		Designs: &DesignsDiff{
			Removed: []DesignId{{Guid: designGuid}},
		},
	}
}

// AddFileToKit MUST return a diff with exactly one added file.
// AddFileToKit creates a diff that adds a single file to a kit.
func AddFileToKit(file File) KitDiff {
	return KitDiff{
		Files: &FilesDiff{
			Added: []File{file},
		},
	}
}

// RemoveFileFromKit MUST return a diff with exactly one removed file ID.
// RemoveFileFromKit creates a diff that removes a file by GUID.
func RemoveFileFromKit(fileGuid string) KitDiff {
	return KitDiff{
		Files: &FilesDiff{
			Removed: []FileId{{Guid: fileGuid}},
		},
	}
}

// AddPortToKit MUST return a diff with exactly one added port.
// AddPortToKit creates a diff that adds a single port to a kit.
func AddPortToKit(iface Port) KitDiff {
	return KitDiff{
		Ports: &PortsDiff{
			Added: []Port{iface},
		},
	}
}

// RemovePortFromKit MUST return a diff with exactly one removed port ID.
// RemovePortFromKit creates a diff that removes a port by GUID.
func RemovePortFromKit(interfaceGuid string) KitDiff {
	return KitDiff{
		Ports: &PortsDiff{
			Removed: []PortId{{Guid: interfaceGuid}},
		},
	}
}

// AddTagToKit MUST return a diff with exactly one added tag.
// AddTagToKit creates a diff that adds a single tag to a kit.
func AddTagToKit(tag Tag) KitDiff {
	return KitDiff{
		Tags: &TagsDiff{
			Added: []Tag{tag},
		},
	}
}

// RemoveTagFromKit MUST return a diff with exactly one removed tag ID.
// RemoveTagFromKit creates a diff that removes a tag by GUID.
func RemoveTagFromKit(tagGuid string) KitDiff {
	return KitDiff{
		Tags: &TagsDiff{
			Removed: []TagId{{Guid: tagGuid}},
		},
	}
}

// AddConceptToKit MUST return a diff with exactly one added concept.
// AddConceptToKit creates a diff that adds a single concept to a kit.
func AddConceptToKit(concept Concept) KitDiff {
	return KitDiff{
		Concepts: &ConceptsDiff{
			Added: []Concept{concept},
		},
	}
}

// RemoveConceptFromKit MUST return a diff with exactly one removed concept ID.
// RemoveConceptFromKit creates a diff that removes a concept by GUID.
func RemoveConceptFromKit(conceptGuid string) KitDiff {
	return KitDiff{
		Concepts: &ConceptsDiff{
			Removed: []ConceptId{{Guid: conceptGuid}},
		},
	}
}

// #endregion 🔖Kit Diff Helpers

// #region 🔖Validation
// Validation MUST provide constraint-based validation of kit data integrity.

// SemioEntityKind enumerates the kinds of semio domain entities.
type SemioEntityKind string

const (
	EntityKindKit        SemioEntityKind = "Kit"
	EntityKindType       SemioEntityKind = "Type"
	EntityKindDesign     SemioEntityKind = "Design"
	EntityKindPiece      SemioEntityKind = "Piece"
	EntityKindConnection SemioEntityKind = "Connection"
	EntityKindConnector  SemioEntityKind = "Connector"
	EntityKindAttribute  SemioEntityKind = "Attribute"
	EntityKindFile       SemioEntityKind = "File"
	EntityKindFolder     SemioEntityKind = "Folder"
	EntityKindQuality    SemioEntityKind = "Quality"
	EntityKindPort  SemioEntityKind = "Port"
	EntityKindProp       SemioEntityKind = "Prop"
	EntityKindModel      SemioEntityKind = "Model"
	EntityKindLayer      SemioEntityKind = "Layer"
	EntityKindGroup      SemioEntityKind = "Group"
	EntityKindStat       SemioEntityKind = "Stat"
	EntityKindTag        SemioEntityKind = "Tag"
	EntityKindConcept    SemioEntityKind = "Concept"
	EntityKindAuthor     SemioEntityKind = "Author"
)

// Severity enumerates validation problem severity levels.
type Severity string

const (
	SeverityError   Severity = "error"
	SeverityWarning Severity = "warning"
)

// DomainLocation identifies the entity and field where a validation problem occurs.
type DomainLocation struct {
	EntityKind SemioEntityKind `json:"entityKind"`
	EntityGuid string          `json:"entityGuid,omitempty"`
	Field      string          `json:"field,omitempty"`
}

// Fix represents a suggested correction for a validation problem.
type Fix struct {
	Title string  `json:"title"`
	Diff  KitDiff `json:"diff"`
}

// Problem represents a single validation constraint violation.
type Problem struct {
	ConstraintId string         `json:"constraintId"`
	Severity     Severity       `json:"severity,omitempty"`
	Message      string         `json:"message"`
	Location     DomainLocation `json:"entityKind,omitempty"`
	RelatedGuids []string       `json:"relatedGuids,omitempty"`
	Fixes        []Fix          `json:"fixes"`
}

// ValidationResult contains all problems found during kit validation.
type ValidationResult struct {
	Problems []Problem `json:"problems"`
}

// ValidationContext provides indexed access to kit entities for constraint evaluation.
type ValidationContext struct {
	Kit                Kit
	TypesByGuid        map[string]*Type
	DesignsByGuid      map[string]*Design
	PiecesByGuid       map[string]struct {
		DesignGuid string
		Piece      *Piece
	}
	ConnectorsByTypeGuid map[string][]Connector
	ModelsByTypeGuid     map[string][]Model
}

// Constraint is a function that evaluates a validation rule against a kit context.
type Constraint func(ctx *ValidationContext) []Problem

func buildValidationContext(kit Kit) *ValidationContext {
	ctx := &ValidationContext{
		Kit:                  kit,
		TypesByGuid:          make(map[string]*Type),
		DesignsByGuid:        make(map[string]*Design),
		PiecesByGuid:         make(map[string]struct{ DesignGuid string; Piece *Piece }),
		ConnectorsByTypeGuid: make(map[string][]Connector),
		ModelsByTypeGuid:     make(map[string][]Model),
	}
	for i := range kit.Types {
		t := &kit.Types[i]
		ctx.TypesByGuid[t.Guid] = t
		ctx.ConnectorsByTypeGuid[t.Guid] = t.Connectors
		ctx.ModelsByTypeGuid[t.Guid] = t.Models
	}
	for i := range kit.Designs {
		d := &kit.Designs[i]
		ctx.DesignsByGuid[d.Guid] = d
		for j := range d.Pieces {
			p := &d.Pieces[j]
			ctx.PiecesByGuid[p.Guid] = struct {
				DesignGuid string
				Piece      *Piece
			}{DesignGuid: d.Guid, Piece: p}
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

// GuidUniquenessConstraint MUST report each duplicate GUID as a separate problem.
// GuidUniquenessConstraint checks that all entity GUIDs are unique within a kit.
func GuidUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	seen := make(map[string]SemioEntityKind)
	check := func(entityKind SemioEntityKind, entityGuid string) {
		if _, exists := seen[entityGuid]; exists {
			problem := Problem{
				ConstraintId: "guid-unique",
				Severity:     SeverityError,
				Message:      fmt.Sprintf("Duplicate GUID \"%s\". First occurrence kept.", entityGuid),
				Location:     DomainLocation{EntityKind: entityKind, EntityGuid: entityGuid, Field: "guid"},
				RelatedGuids: []string{entityGuid},
				Fixes: []Fix{
					makeFix(ctx, "Regenerate GUID", func(clone *Kit) {
						newGuid := Guid()
						updateGuidEverywhere(clone, entityGuid, newGuid)
					}),
				},
			}
			problems = append(problems, problem)
		} else {
			seen[entityGuid] = entityKind
		}
	}
	check(EntityKindKit, ctx.Kit.Guid)
	for _, t := range ctx.Kit.Types {
		check(EntityKindType, t.Guid)
	}
	for _, d := range ctx.Kit.Designs {
		check(EntityKindDesign, d.Guid)
		for _, p := range d.Pieces {
			check(EntityKindPiece, p.Guid)
		}
		for _, c := range d.Connections {
			check(EntityKindConnection, c.Guid)
		}
		for _, s := range d.Stats {
			check(EntityKindStat, s.Guid)
		}
	}
	for _, q := range ctx.Kit.Qualities {
		check(EntityKindQuality, q.Guid)
	}
	for _, i := range ctx.Kit.Ports {
		check(EntityKindPort, i.Guid)
	}
	for _, f := range ctx.Kit.Files {
		check(EntityKindFile, f.Guid)
	}
	for _, f := range ctx.Kit.Folders {
		check(EntityKindFolder, f.Guid)
	}
	return problems
}

func updateGuidEverywhere(kit *Kit, oldGuid, newGuid string) {
	if kit.Guid == oldGuid {
		kit.Guid = newGuid
	}
	for i := range kit.Types {
		t := &kit.Types[i]
		if t.Guid == oldGuid {
			t.Guid = newGuid
		}
		if t.Parent != nil && t.Parent.Guid == oldGuid {
			t.Parent.Guid = newGuid
		}
		for j := range t.Connectors {
			if t.Connectors[j].Guid == oldGuid {
				t.Connectors[j].Guid = newGuid
			}
		}
		for j := range t.Models {
			if t.Models[j].Guid == oldGuid {
				t.Models[j].Guid = newGuid
			}
		}
	}
	for i := range kit.Designs {
		d := &kit.Designs[i]
		if d.Guid == oldGuid {
			d.Guid = newGuid
		}
		if d.Parent != nil && d.Parent.Guid == oldGuid {
			d.Parent.Guid = newGuid
		}
		for j := range d.Pieces {
			p := &d.Pieces[j]
			if p.Guid == oldGuid {
				p.Guid = newGuid
			}
			if p.Type != nil && p.Type.Guid == oldGuid {
				p.Type.Guid = newGuid
			}
			if p.Design != nil && p.Design.Guid == oldGuid {
				p.Design.Guid = newGuid
			}
		}
		for j := range d.Connections {
			c := &d.Connections[j]
			if c.Guid == oldGuid {
				c.Guid = newGuid
			}
			if c.Connected.Piece.Guid == oldGuid {
				c.Connected.Piece.Guid = newGuid
			}
			if c.Connecting.Piece.Guid == oldGuid {
				c.Connecting.Piece.Guid = newGuid
			}
			if c.Connected.Connector != nil && c.Connected.Connector.Guid == oldGuid {
				c.Connected.Connector.Guid = newGuid
			}
			if c.Connecting.Connector != nil && c.Connecting.Connector.Guid == oldGuid {
				c.Connecting.Connector.Guid = newGuid
			}
		}
	}
	for i := range kit.Ports {
		if kit.Ports[i].Guid == oldGuid {
			kit.Ports[i].Guid = newGuid
		}
		for j := range kit.Ports[i].CompatiblePorts {
			if kit.Ports[i].CompatiblePorts[j].Guid == oldGuid {
				kit.Ports[i].CompatiblePorts[j].Guid = newGuid
			}
		}
	}
	for i := range kit.Qualities {
		if kit.Qualities[i].Guid == oldGuid {
			kit.Qualities[i].Guid = newGuid
		}
	}
	for i := range kit.Files {
		if kit.Files[i].Guid == oldGuid {
			kit.Files[i].Guid = newGuid
		}
	}
	for i := range kit.Folders {
		if kit.Folders[i].Guid == oldGuid {
			kit.Folders[i].Guid = newGuid
		}
		if kit.Folders[i].Parent != nil && kit.Folders[i].Parent.Guid == oldGuid {
			kit.Folders[i].Parent.Guid = newGuid
		}
	}
}

// TypeNameUniquenessConstraint MUST report duplicate names among types with the same parent.
// TypeNameUniquenessConstraint checks that sibling type names are unique.
func TypeNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	byParent := make(map[string][]Type)
	for _, t := range ctx.Kit.Types {
		parentGuid := ""
		if t.Parent != nil {
			parentGuid = t.Parent.Guid
		}
		byParent[parentGuid] = append(byParent[parentGuid], t)
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
				relatedGuids := make([]string, len(group))
				for j, g := range group {
					relatedGuids[j] = g.Guid
				}
				problem := Problem{
					ConstraintId: "type-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate type name \"%s\" among siblings.", name),
					Location:     DomainLocation{EntityKind: EntityKindType, EntityGuid: typ.Guid, Field: "name"},
					RelatedGuids: relatedGuids,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename \"%s\"", name), func(clone *Kit) {
							for j := range clone.Types {
								if clone.Types[j].Guid == typ.Guid {
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

// DesignNameUniquenessConstraint MUST report duplicate names among designs with the same parent.
// DesignNameUniquenessConstraint checks that sibling design names are unique.
func DesignNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	byParent := make(map[string][]Design)
	for _, d := range ctx.Kit.Designs {
		parentGuid := ""
		if d.Parent != nil {
			parentGuid = d.Parent.Guid
		}
		byParent[parentGuid] = append(byParent[parentGuid], d)
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
				relatedGuids := make([]string, len(group))
				for j, g := range group {
					relatedGuids[j] = g.Guid
				}
				problem := Problem{
					ConstraintId: "design-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate design name \"%s\" among siblings.", name),
					Location:     DomainLocation{EntityKind: EntityKindDesign, EntityGuid: design.Guid, Field: "name"},
					RelatedGuids: relatedGuids,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename \"%s\"", name), func(clone *Kit) {
							for j := range clone.Designs {
								if clone.Designs[j].Guid == design.Guid {
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

// PieceNameUniquenessConstraint MUST report duplicate piece names within each design.
// PieceNameUniquenessConstraint checks that piece names are unique within each design.
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
				relatedGuids := make([]string, len(group))
				for j, g := range group {
					relatedGuids[j] = g.Guid
				}
				designGuid := design.Guid
				problem := Problem{
					ConstraintId: "piece-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate piece name \"%s\" inside design \"%s\".", name, design.Name),
					Location:     DomainLocation{EntityKind: EntityKindPiece, EntityGuid: piece.Guid, Field: "name"},
					RelatedGuids: relatedGuids,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename piece \"%s\"", name), func(clone *Kit) {
							for j := range clone.Designs {
								if clone.Designs[j].Guid == designGuid {
									for k := range clone.Designs[j].Pieces {
										if clone.Designs[j].Pieces[k].Guid == piece.Guid {
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

// QualityNameUniquenessConstraint MUST report each duplicate quality name.
// QualityNameUniquenessConstraint checks that quality names are unique within a kit.
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
			relatedGuids := make([]string, len(group))
			for j, g := range group {
				relatedGuids[j] = g.Guid
			}
			problem := Problem{
				ConstraintId: "quality-name-unique",
				Severity:     SeverityError,
				Message:      fmt.Sprintf("Duplicate quality name \"%s\".", name),
				Location:     DomainLocation{EntityKind: EntityKindQuality, EntityGuid: quality.Guid, Field: "name"},
				RelatedGuids: relatedGuids,
				Fixes: []Fix{
					makeFix(ctx, fmt.Sprintf("Rename quality \"%s\"", name), func(clone *Kit) {
						for j := range clone.Qualities {
							if clone.Qualities[j].Guid == quality.Guid {
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

// PortNameUniquenessConstraint MUST report each duplicate port name.
// PortNameUniquenessConstraint checks that port names are unique within a kit.
func PortNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	names := make(map[string][]Port)
	for _, p := range ctx.Kit.Ports {
		name := p.Name
		names[name] = append(names[name], p)
	}
	allNames := make([]string, len(ctx.Kit.Ports))
	for i, p := range ctx.Kit.Ports {
		allNames[i] = p.Name
	}
	for name, group := range names {
		if len(group) <= 1 {
			continue
		}
		for i := 1; i < len(group); i++ {
			iface := group[i]
			relatedGuids := make([]string, len(group))
			for j, g := range group {
				relatedGuids[j] = g.Guid
			}
			problem := Problem{
				ConstraintId: "port-name-unique",
				Severity:     SeverityError,
				Message:      fmt.Sprintf("Duplicate port name \"%s\".", name),
				Location:     DomainLocation{EntityKind: EntityKindPort, EntityGuid: iface.Guid, Field: "name"},
				RelatedGuids: relatedGuids,
				Fixes: []Fix{
					makeFix(ctx, fmt.Sprintf("Rename port \"%s\"", name), func(clone *Kit) {
						for j := range clone.Ports {
							if clone.Ports[j].Guid == iface.Guid {
								clone.Ports[j].Name = generateUniqueName(name, allNames)
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

// FileNameUniquenessConstraint MUST report each duplicate file name.
// FileNameUniquenessConstraint checks that file names are unique within a kit.
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
			relatedGuids := make([]string, len(group))
			for j, g := range group {
				relatedGuids[j] = g.Guid
			}
			problem := Problem{
				ConstraintId: "file-name-unique",
				Severity:     SeverityError,
				Message:      fmt.Sprintf("Duplicate file name \"%s\".", name),
				Location:     DomainLocation{EntityKind: EntityKindFile, EntityGuid: file.Guid, Field: "name"},
				RelatedGuids: relatedGuids,
				Fixes: []Fix{
					makeFix(ctx, fmt.Sprintf("Rename file \"%s\"", name), func(clone *Kit) {
						for j := range clone.Files {
							if clone.Files[j].Guid == file.Guid {
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

// FolderNameUniquenessConstraint MUST report duplicate names among folders with the same parent.
// FolderNameUniquenessConstraint checks that sibling folder names are unique.
func FolderNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	byParent := make(map[string][]Folder)
	for _, f := range ctx.Kit.Folders {
		parentGuid := ""
		if f.Parent != nil {
			parentGuid = f.Parent.Guid
		}
		byParent[parentGuid] = append(byParent[parentGuid], f)
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
				relatedGuids := make([]string, len(group))
				for j, g := range group {
					relatedGuids[j] = g.Guid
				}
				problem := Problem{
					ConstraintId: "folder-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate folder name \"%s\" among siblings.", name),
					Location:     DomainLocation{EntityKind: EntityKindFolder, EntityGuid: folder.Guid, Field: "name"},
					RelatedGuids: relatedGuids,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename folder \"%s\"", name), func(clone *Kit) {
							for j := range clone.Folders {
								if clone.Folders[j].Guid == folder.Guid {
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

// ConnectorNameUniquenessConstraint MUST report duplicate connector names within each type.
// ConnectorNameUniquenessConstraint checks that connector names are unique within each type.
func ConnectorNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	for typeGuid, connectors := range ctx.ConnectorsByTypeGuid {
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
		typ := ctx.TypesByGuid[typeGuid]
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
				relatedGuids := make([]string, len(group))
				for j, g := range group {
					relatedGuids[j] = g.Guid
				}
				tGuid := typeGuid
				problem := Problem{
					ConstraintId: "connector-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate connector name \"%s\" inside type \"%s\".", name, typeName),
					Location:     DomainLocation{EntityKind: EntityKindConnector, EntityGuid: connector.Guid, Field: "name"},
					RelatedGuids: relatedGuids,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename connector \"%s\"", name), func(clone *Kit) {
							for j := range clone.Types {
								if clone.Types[j].Guid == tGuid {
									for k := range clone.Types[j].Connectors {
										if clone.Types[j].Connectors[k].Guid == connector.Guid {
											newName := generateUniqueName(name, allNames)
											clone.Types[j].Connectors[k].Name = &newName
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

// ModelNameUniquenessConstraint MUST report duplicate model names within each type.
// ModelNameUniquenessConstraint checks that model names are unique within each type.
func ModelNameUniquenessConstraint(ctx *ValidationContext) []Problem {
	var problems []Problem
	for typeGuid, models := range ctx.ModelsByTypeGuid {
		if len(models) == 0 {
			continue
		}
		names := make(map[string][]Model)
		for _, m := range models {
			name := ""
			if m.Name != nil {
				name = *m.Name
			}
			names[name] = append(names[name], m)
		}
		allNames := make([]string, len(models))
		for i, m := range models {
			if m.Name != nil {
				allNames[i] = *m.Name
			}
		}
		typ := ctx.TypesByGuid[typeGuid]
		typeName := ""
		if typ != nil {
			typeName = typ.Name
		}
		for name, group := range names {
			if len(group) <= 1 {
				continue
			}
			for i := 1; i < len(group); i++ {
				model := group[i]
				relatedGuids := make([]string, len(group))
				for j, g := range group {
					relatedGuids[j] = g.Guid
				}
				tGuid := typeGuid
				problem := Problem{
					ConstraintId: "model-name-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate model name \"%s\" inside type \"%s\".", name, typeName),
					Location:     DomainLocation{EntityKind: EntityKindModel, EntityGuid: model.Guid, Field: "name"},
					RelatedGuids: relatedGuids,
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename model \"%s\"", name), func(clone *Kit) {
							for j := range clone.Types {
								if clone.Types[j].Guid == tGuid {
									for k := range clone.Types[j].Models {
										if clone.Types[j].Models[k].Guid == model.Guid {
											newName := generateUniqueName(name, allNames)
											clone.Types[j].Models[k].Name = &newName
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

// LayerPathUniquenessConstraint MUST report duplicate layer paths within each design.
// LayerPathUniquenessConstraint checks that layer paths are unique within each design.
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
				designGuid := design.Guid
				problem := Problem{
					ConstraintId: "layer-path-unique",
					Severity:     SeverityError,
					Message:      fmt.Sprintf("Duplicate layer path \"%s\" inside design \"%s\".", path, design.Name),
					Location:     DomainLocation{EntityKind: EntityKindLayer, EntityGuid: layer.Guid, Field: "path"},
					Fixes: []Fix{
						makeFix(ctx, fmt.Sprintf("Rename layer \"%s\"", path), func(clone *Kit) {
							for j := range clone.Designs {
								if clone.Designs[j].Guid == designGuid {
									for k := range clone.Designs[j].Layers {
										if clone.Designs[j].Layers[k].Guid == layer.Guid {
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

// DefaultConstraints lists all built-in validation constraints.
var DefaultConstraints = []Constraint{
	GuidUniquenessConstraint,
	TypeNameUniquenessConstraint,
	DesignNameUniquenessConstraint,
	PieceNameUniquenessConstraint,
	QualityNameUniquenessConstraint,
	PortNameUniquenessConstraint,
	FileNameUniquenessConstraint,
	FolderNameUniquenessConstraint,
	ConnectorNameUniquenessConstraint,
	ModelNameUniquenessConstraint,
	LayerPathUniquenessConstraint,
}

// ValidateKit MUST apply all default constraints and return all found problems.
// ValidateKit validates a kit using the default set of constraints.
func ValidateKit(kit Kit) ValidationResult {
	return ValidateKitWithConstraints(kit, DefaultConstraints)
}

// ValidateKitWithConstraints MUST apply each constraint and aggregate all problems.
// ValidateKitWithConstraints validates a kit using the provided constraints.
func ValidateKitWithConstraints(kit Kit, constraints []Constraint) ValidationResult {
	ctx := buildValidationContext(kit)
	var problems []Problem
	for _, constraint := range constraints {
		problems = append(problems, constraint(ctx)...)
	}
	return ValidationResult{Problems: problems}
}

// HasErrors MUST return true when any problem has error severity or empty severity.
// HasErrors returns true if the validation result contains any error-severity problems.
func HasErrors(result ValidationResult) bool {
	for _, p := range result.Problems {
		if p.Severity == SeverityError || p.Severity == "" {
			return true
		}
	}
	return false
}

// #region 🔖Validation Serialization
// Validation Serialization MUST provide serializable representations of validation results.

// ProblemSerialized is the JSON-serializable representation of a validation problem.
type ProblemSerialized struct {
	ConstraintId string `json:"constraintId"`
	Severity     string `json:"severity,omitempty"`
	Message      string `json:"message"`
	EntityKind   string `json:"entityKind"`
	EntityGuid   string `json:"entityGuid"`
	Fixes        []Fix  `json:"fixes"`
}

// ValidationResultSerialized is the JSON-serializable representation of a validation result.
type ValidationResultSerialized struct {
	Problems []ProblemSerialized `json:"problems"`
}

// ToValidationResult MUST default empty severity to error.
// ToValidationResult converts a validation result to its serializable form.
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
			EntityGuid:   p.Location.EntityGuid,
			Fixes:        p.Fixes,
		}
	}
	return ValidationResultSerialized{Problems: problems}
}

// AreValidationResultsEqual MUST compare problems regardless of their order.
// AreValidationResultsEqual compares two serialized validation results for equality.
func AreValidationResultsEqual(a, b ValidationResultSerialized) bool {
	if len(a.Problems) != len(b.Problems) {
		return false
	}
	sortProblems := func(problems []ProblemSerialized) {
		sort.Slice(problems, func(i, j int) bool {
			if problems[i].ConstraintId != problems[j].ConstraintId {
				return problems[i].ConstraintId < problems[j].ConstraintId
			}
			return problems[i].EntityGuid < problems[j].EntityGuid
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
			sortedA[i].EntityGuid != sortedB[i].EntityGuid {
			return false
		}
	}
	return true
}

// #endregion 🔖Validation Serialization

// #endregion 🔖Validation

// #region 🔖Flatten Design
// Flatten Design MUST compute absolute piece planes from relative connections.

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

func getConnector(typesDict map[string]*Type, typ *Type, connectorGuid *string) *Connector {
	if typ == nil {
		return nil
	}
	if connectorGuid == nil || *connectorGuid == "" {
		if len(typ.Connectors) > 0 {
			return &typ.Connectors[0]
		}
		if typ.Parent != nil {
			parentType := typesDict[typ.Parent.Guid]
			return getConnector(typesDict, parentType, connectorGuid)
		}
		return nil
	}
	for i := range typ.Connectors {
		if typ.Connectors[i].Guid == *connectorGuid {
			return &typ.Connectors[i]
		}
	}
	if typ.Parent != nil {
		parentType := typesDict[typ.Parent.Guid]
		if connector := getConnector(typesDict, parentType, connectorGuid); connector != nil {
			return connector
		}
	}
	if len(typ.Connectors) > 0 {
		return &typ.Connectors[0]
	}
	return nil
}

// FlattenDesign MUST traverse the connection graph via BFS to compute piece transforms.
// FlattenDesign computes absolute planes and centers for all pieces in a design.
func FlattenDesign(kit *Kit, designGuid string) DesignDiff {
	design := FindDesignInKit(kit, designGuid)
	if design == nil || len(design.Pieces) == 0 {
		return DesignDiff{}
	}

	typesDict := make(map[string]*Type)
	for i := range kit.Types {
		typesDict[kit.Types[i].Guid] = &kit.Types[i]
	}

	pieceMap := make(map[string]*Piece)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Guid] = &design.Pieces[i]
	}

	piecePlanes := make(map[string]*Plane)
	adjacency := make(map[string][]struct {
		neighborGuid string
		connection   *Connection
	})

	for i := range design.Connections {
		conn := &design.Connections[i]
		srcGuid := conn.Connected.Piece.Guid
		tgtGuid := conn.Connecting.Piece.Guid
		if pieceMap[srcGuid] == nil || pieceMap[tgtGuid] == nil {
			continue
		}
		adjacency[srcGuid] = append(adjacency[srcGuid], struct {
			neighborGuid string
			connection   *Connection
		}{tgtGuid, conn})
		adjacency[tgtGuid] = append(adjacency[tgtGuid], struct {
			neighborGuid string
			connection   *Connection
		}{srcGuid, conn})
	}

	visited := make(map[string]bool)
	var bfs func(rootGuid string)
	bfs = func(rootGuid string) {
		queue := []string{rootGuid}
		visited[rootGuid] = true
		rootPiece := pieceMap[rootGuid]
		if rootPiece.Plane != nil {
			piecePlanes[rootGuid] = rootPiece.Plane
		} else {
			identityPlane := Plane{
				Origin: Point{X: 0, Y: 0, Z: 0},
				XAxis:  Vector{X: 1, Y: 0, Z: 0},
				YAxis:  Vector{X: 0, Y: 1, Z: 0},
			}
			piecePlanes[rootGuid] = &identityPlane
		}

		for len(queue) > 0 {
			currentGuid := queue[0]
			queue = queue[1:]
			currentPlane := piecePlanes[currentGuid]
			currentPiece := pieceMap[currentGuid]

			for _, neighbor := range adjacency[currentGuid] {
				if visited[neighbor.neighborGuid] {
					continue
				}
				visited[neighbor.neighborGuid] = true
				neighborPiece := pieceMap[neighbor.neighborGuid]
				conn := neighbor.connection

				var parentSide, childSide *Side
				if conn.Connected.Piece.Guid == currentGuid {
					parentSide = &conn.Connected
					childSide = &conn.Connecting
				} else {
					parentSide = &conn.Connecting
					childSide = &conn.Connected
				}

				var parentType, childType *Type
				if currentPiece.Type != nil {
					parentType = typesDict[currentPiece.Type.Guid]
				}
				if neighborPiece.Type != nil {
					childType = typesDict[neighborPiece.Type.Guid]
				}

				var parentConnectorGuid, childConnectorGuid *string
				if parentSide.Connector != nil {
					parentConnectorGuid = &parentSide.Connector.Guid
				}
				if childSide.Connector != nil {
					childConnectorGuid = &childSide.Connector.Guid
				}

				parentConnector := getConnector(typesDict, parentType, parentConnectorGuid)
				childConnector := getConnector(typesDict, childType, childConnectorGuid)

				if parentConnector == nil || childConnector == nil {
					continue
				}

				childPlane := roundPlane(computeChildPlane(*currentPlane, *parentConnector, *childConnector, *conn))
				piecePlanes[neighbor.neighborGuid] = &childPlane

				radius := 2.697
				verticalVExtra := 1.0
				horizontalScale := 3.0633
				var parentCenter Coord
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

				childCenter := &Coord{U: roundFloat(childU, 6), V: roundFloat(childV, 6)}
				neighborPiece.Center = childCenter

				queue = append(queue, neighbor.neighborGuid)
			}
		}
	}

	for _, piece := range design.Pieces {
		if !visited[piece.Guid] {
			bfs(piece.Guid)
		}
	}

	var updatedPieces []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	}

	for i := range design.Pieces {
		piece := &design.Pieces[i]
		plane := piecePlanes[piece.Guid]
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

		pieceFromMap := pieceMap[piece.Guid]
		if pieceFromMap.Center != nil {
			if piece.Center == nil || pieceFromMap.Center.U != piece.Center.U || pieceFromMap.Center.V != piece.Center.V {
				diff.Center = &CoordDiff{U: &pieceFromMap.Center.U, V: &pieceFromMap.Center.V}
				hasChanges = true
			}
		}

		if hasChanges {
			updatedPieces = append(updatedPieces, struct {
				Piece PieceId   `json:"piece"`
				Diff  PieceDiff `json:"diff"`
			}{Piece: PieceId{Guid: piece.Guid}, Diff: diff})
		}
	}

	result := DesignDiff{}
	if len(updatedPieces) > 0 {
		result.Pieces = &PiecesDiff{Updated: updatedPieces}
	}
	return result
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

// ApplyDesignDiff MUST apply all piece, connection and property changes from the diff.
// ApplyDesignDiff applies a design diff to a base design.
func ApplyDesignDiff(base Design, diff DesignDiff) Design {
	return applyDesignDiff(base, diff)
}

// #endregion 🔖Flatten Design

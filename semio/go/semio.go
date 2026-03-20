// #region 🔖Header
// [👤semio📚go💻semio](repo://p/u/semio/b/l/go/f/semio.go)

// 2026 Ueli Saluz  <ueli@semio-tech.de>

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

// Core domain library in Go implementing the semio data model and operations.

// #endregion 🔖Header

// #region 🔖Imports
// [👤semio📚go💻semio🔖imports](repo://p/u/semio/b/l/go/f/semio.go/s/Imports)
// Imports MUST include all required packages for the semio domain library.

package semio

import (
	"bytes"
	"crypto/rand"
	"encoding/base64"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"

	"gonum.org/v1/gonum/mat"
)

// #endregion 🔖Imports

// #region 🔖Constants
// [👤semio📚go💻semio🔖constants](repo://p/u/semio/b/l/go/f/semio.go/s/Constants)
// Constants MUST define shared constant values for the semio domain.

const (
	IconWidth = 24
	Tolerance = 0.0001
)

// AssetsPath holds the data fields for a AssetsPath record.
// [👤semio📚go💻semio🔖constants🪨assetspath](repo://p/u/semio/b/l/go/f/semio.go/s/Constants/d/i/AssetsPath)
const AssetsPath = "../assets/semio"

// #endregion 🔖Constants

// #region 🔖Utils
// [👤semio📚go💻semio🔖utils](repo://p/u/semio/b/l/go/f/semio.go/s/Utils)
// Utils MUST provide general-purpose utility functions for the semio domain.

// Guid MUST return a cryptographically random 128-bit hex string.
// Guid generates a new random 128-bit hex-encoded unique identifier.
// [👤semio📚go💻semio🔖utils🛠️ptrstring](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/ptrString)
// ptrString returns a pointer to the given string value.
func ptrString(s string) *string { return &s }

// [👤semio📚go💻semio🔖utils🛠️ptrfloat64](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/ptrFloat64)
// ptrFloat64 holds the data fields for a ptrFloat64 record.
// ptrFloat64 MUST perform the ptrFloat64 operation.
func ptrFloat64(f float64) *float64 { return &f }

// [👤semio📚go💻semio🔖utils🛠️floatequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/floatEqual)
// floatEqual holds the data fields for a floatEqual record.
// floatEqual MUST perform the floatEqual operation.
func floatEqual(a, b, tolerance float64) bool {
	return math.Abs(a-b) < tolerance
}

// [👤semio📚go💻semio🔖utils🛠️optfloatequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/optFloatEqual)
// optFloatEqual holds the data fields for a optFloatEqual record.
// optFloatEqual MUST perform the optFloatEqual operation.
func optFloatEqual(a, b *float64) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return floatEqual(*a, *b, 1e-9)
}

// [👤semio📚go💻semio🔖utils🛠️optboolequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/optBoolEqual)
// optBoolEqual holds the data fields for a optBoolEqual record.
// optBoolEqual MUST perform the optBoolEqual operation.
func optBoolEqual(a, b *bool) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return *a == *b
}

// optStringEqual holds the data fields for a optStringEqual record.
// optStringEqual MUST perform the optStringEqual operation.
// [👤semio📚go💻semio🔖utils🛠️optstringequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/optStringEqual)
func optStringEqual(a, b *string) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return *a == *b
}

// [👤semio📚go💻semio🔖utils🛠️arelocationidsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/areLocationIdsEqual)
// areLocationIdsEqual holds the data fields for a areLocationIdsEqual record.
// areLocationIdsEqual MUST perform the areLocationIdsEqual operation.
func areLocationIdsEqual(a, b *LocationId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Guid == b.Guid
}

// [👤semio📚go💻semio🔖utils🛠️aretypeidsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/areTypeIdsEqual)
// areTypeIdsEqual holds the data fields for a areTypeIdsEqual record.
// areTypeIdsEqual MUST perform the areTypeIdsEqual operation.
func areTypeIdsEqual(a, b *TypeId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Guid == b.Guid
}

// [👤semio📚go💻semio🔖utils🛠️aredesignidsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/areDesignIdsEqual)
// areDesignIdsEqual holds the data fields for a areDesignIdsEqual record.
// areDesignIdsEqual MUST perform the areDesignIdsEqual operation.
func areDesignIdsEqual(a, b *DesignId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Guid == b.Guid
}

// [👤semio📚go💻semio🔖utils🛠️areportidsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/arePortIdsEqual)
// arePortIdsEqual holds the data fields for a arePortIdsEqual record.
// arePortIdsEqual MUST perform the arePortIdsEqual operation.
func arePortIdsEqual(a, b *PortId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Guid == b.Guid
}

// [👤semio📚go💻semio🔖utils🛠️arelayeridsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/areLayerIdsEqual)
// areLayerIdsEqual holds the data fields for a areLayerIdsEqual record.
// areLayerIdsEqual MUST perform the areLayerIdsEqual operation.
func areLayerIdsEqual(a, b *LayerId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Guid == b.Guid
}

// [👤semio📚go💻semio🔖utils🛠️normalizeoptint](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/normalizeOptInt)
// normalizeOptInt holds the data fields for a normalizeOptInt record.
// normalizeOptInt MUST perform the normalizeOptInt operation.
func normalizeOptInt(p *int) int {
	if p == nil {
		return 0
	}
	return *p
}

// [👤semio📚go💻semio🔖utils🛠️areauthoridsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/areAuthorIdsEqual)
// areAuthorIdsEqual holds the data fields for a areAuthorIdsEqual record.
// areAuthorIdsEqual MUST perform the areAuthorIdsEqual operation.
func areAuthorIdsEqual(a, b []AuthorId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Guid != b[i].Guid {
			return false
		}
	}
	return true
}

// [👤semio📚go💻semio🔖utils🛠️areconceptidsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/areConceptIdsEqual)
// areConceptIdsEqual holds the data fields for a areConceptIdsEqual record.
// areConceptIdsEqual MUST perform the areConceptIdsEqual operation.
func areConceptIdsEqual(a, b []ConceptId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Guid != b[i].Guid {
			return false
		}
	}
	return true
}

// [👤semio📚go💻semio🔖utils🛠️areportidslicesequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/arePortIdSlicesEqual)
// arePortIdSlicesEqual holds the data fields for a arePortIdSlicesEqual record.
// arePortIdSlicesEqual MUST perform the arePortIdSlicesEqual operation.
func arePortIdSlicesEqual(a, b []PortId) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Guid != b[i].Guid {
			return false
		}
	}
	return true
}

// [👤semio📚go💻semio🔖utils🛠️areattributesequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/areAttributesEqual)
// areAttributesEqual holds the data fields for a areAttributesEqual record.
// areAttributesEqual MUST perform the areAttributesEqual operation.
func areAttributesEqual(a, b []Attribute) bool {
	if len(a) != len(b) {
		return false
	}
	aMap := make(map[string]Attribute)
	for _, attr := range a {
		aMap[attr.Guid] = attr
	}
	for _, attr := range b {
		other, ok := aMap[attr.Guid]
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

// [👤semio📚go💻semio🔖utils🛠️arepropsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/arePropsEqual)
// arePropsEqual holds the data fields for a arePropsEqual record.
// arePropsEqual MUST perform the arePropsEqual operation.
func arePropsEqual(a, b []Prop) bool {
	if len(a) != len(b) {
		return false
	}
	aMap := make(map[string]Prop)
	for _, p := range a {
		aMap[p.Guid] = p
	}
	for _, p := range b {
		other, ok := aMap[p.Guid]
		if !ok {
			return false
		}
		if p.Quality.Guid != other.Quality.Guid || p.Value != other.Value || normalizeStr(p.Unit) != normalizeStr(other.Unit) {
			return false
		}
	}
	return true
}

// [👤semio📚go💻semio🔖utils🛠️guid](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/Guid)
// Guid holds the data fields for a Guid record.
// Guid MUST perform the Guid operation.
func Guid() string {
	bytes := make([]byte, 16)
	rand.Read(bytes)
	return hex.EncodeToString(bytes)
}

// Normalize MUST trim whitespace and convert to lowercase.
// Normalize converts a string to lowercase trimmed form.
// [👤semio📚go💻semio🔖utils🛠️normalize](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/Normalize)
func Normalize(s string) string {
	return strings.ToLower(strings.TrimSpace(s))
}

// Round MUST return the value rounded to exactly the given decimal places.
// Round rounds a float64 to the specified number of decimal places.
// [👤semio📚go💻semio🔖utils🛠️round](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/Round)
func Round(value float64, decimals int) float64 {
	shift := 1.0
	for i := 0; i < decimals; i++ {
		shift *= 10
	}
	return float64(int64(value*shift+0.5)) / shift
}

// DeepEqual MUST return true only when both values produce identical JSON.
// DeepEqual compares two values for deep equality via JSON serialization.
// [👤semio📚go💻semio🔖utils🛠️deepequal](repo://p/u/semio/b/l/go/f/semio.go/s/Utils/d/i/DeepEqual)
func DeepEqual(a, b interface{}) bool {
	aJSON, _ := json.Marshal(a)
	bJSON, _ := json.Marshal(b)
	return string(aJSON) == string(bJSON)
}

// #endregion 🔖Utils

// #region 🔖Entity IDs
// [👤semio📚go💻semio🔖entityids](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs)
// Entity IDs MUST define identifier types for all semio domain entities.

// AttributeId identifies an attribute entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️attributeid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/AttributeId)
type AttributeId struct {
	Guid string `json:"guid"`
}

// LocationId identifies a location entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️locationid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/LocationId)
type LocationId struct {
	Guid string `json:"guid"`
}

// AuthorId identifies an author entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️authorid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/AuthorId)
type AuthorId struct {
	Guid string `json:"guid"`
}

// FileId identifies a file entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️fileid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/FileId)
type FileId struct {
	Guid string `json:"guid"`
}

// FolderId identifies a folder entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️folderid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/FolderId)
type FolderId struct {
	Guid string `json:"guid"`
}

// BenchmarkId identifies a benchmark entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️benchmarkid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/BenchmarkId)
type BenchmarkId struct {
	Guid string `json:"guid"`
}

// QualityId identifies a quality entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️qualityid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/QualityId)
type QualityId struct {
	Guid string `json:"guid"`
}

// PortId identifies a port entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️portid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/PortId)
type PortId struct {
	Guid string `json:"guid"`
}

// PropId identifies a prop entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️propid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/PropId)
type PropId struct {
	Guid string `json:"guid"`
}

// TagId identifies a tag entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️tagid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/TagId)
type TagId struct {
	Guid string `json:"guid"`
}

// ConceptId identifies a concept entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️conceptid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/ConceptId)
type ConceptId struct {
	Guid string `json:"guid"`
}

// ModelId identifies a model entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️modelid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/ModelId)
type ModelId struct {
	Guid string `json:"guid"`
}

// ConnectorId identifies a connector entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️connectorid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/ConnectorId)
type ConnectorId struct {
	Guid string `json:"guid"`
}

// TypeId identifies a type entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️typeid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/TypeId)
type TypeId struct {
	Guid string `json:"guid"`
}

// LayerId identifies a layer entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️layerid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/LayerId)
type LayerId struct {
	Guid string `json:"guid"`
}

// PieceId identifies a piece entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️pieceid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/PieceId)
type PieceId struct {
	Guid string `json:"guid"`
}

// GroupId identifies a group entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️groupid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/GroupId)
type GroupId struct {
	Guid string `json:"guid"`
}

// SideId identifies a connection side by piece, design piece and connector references.
// [👤semio📚go💻semio🔖entityids✂️sideid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/SideId)
type SideId struct {
	Piece       PieceId      `json:"piece"`
	DesignPiece *PieceId     `json:"designPiece,omitempty"`
	Connector   *ConnectorId `json:"connector,omitempty"`
}

// ConnectionId identifies a connection entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️connectionid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/ConnectionId)
type ConnectionId struct {
	Guid string `json:"guid"`
}

// StatId identifies a stat entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️statid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/StatId)
type StatId struct {
	Guid string `json:"guid"`
}

// DesignId identifies a design entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️designid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/DesignId)
type DesignId struct {
	Guid string `json:"guid"`
}

// KitId identifies a kit entity by GUID.
// [👤semio📚go💻semio🔖entityids✂️kitid](repo://p/u/semio/b/l/go/f/semio.go/s/Entity%20IDs/d/i/KitId)
type KitId struct {
	Guid string `json:"guid"`
}

// #endregion 🔖Entity IDs

// #region 🔖Weak Entities
// [👤semio📚go💻semio🔖weakentities](repo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities)
// Weak Entities MUST define value types that exist only as part of parent entities.

// Coord represents a 2D coordinate with U and V components.
// [👤semio📚go💻semio🔖weakentities✂️coord](repo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities/d/i/Coord)
type Coord struct {
	U float64 `json:"u"`
	V float64 `json:"v"`
}

// Vec represents a 2D vector with U and V components.
// [👤semio📚go💻semio🔖weakentities✂️vec](repo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities/d/i/Vec)
type Vec struct {
	U float64 `json:"u"`
	V float64 `json:"v"`
}

// Point represents a 3D point with X, Y and Z components.
// [👤semio📚go💻semio🔖weakentities✂️point](repo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities/d/i/Point)
type Point struct {
	X float64 `json:"x"`
	Y float64 `json:"y"`
	Z float64 `json:"z"`
}

// Vector represents a 3D vector with X, Y and Z components.
// [👤semio📚go💻semio🔖weakentities✂️vector](repo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities/d/i/Vector)
type Vector struct {
	X float64 `json:"x"`
	Y float64 `json:"y"`
	Z float64 `json:"z"`
}

// Plane represents a 3D plane defined by origin, X-axis and Y-axis.
// [👤semio📚go💻semio🔖weakentities✂️plane](repo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities/d/i/Plane)
type Plane struct {
	Origin Point  `json:"origin"`
	XAxis  Vector `json:"xAxis"`
	YAxis  Vector `json:"yAxis"`
}

// Camera represents a 3D camera with position, forward and up vectors.
// [👤semio📚go💻semio🔖weakentities✂️camera](repo://p/u/semio/b/l/go/f/semio.go/s/Weak%20Entities/d/i/Camera)
type Camera struct {
	Position Point  `json:"position"`
	Forward  Vector `json:"forward"`
	Up       Vector `json:"up"`
}

// #endregion 🔖Weak Entities

// #region 🔖Attribute
// [👤semio📚go💻semio🔖attribute](repo://p/u/semio/b/l/go/f/semio.go/s/Attribute)
// Attribute MUST define the key-value metadata entity and its diff types.

// Attribute represents a key-value metadata entry with optional definition.
// [👤semio📚go💻semio🔖attribute✂️attribute](repo://p/u/semio/b/l/go/f/semio.go/s/Attribute/d/i/Attribute)
type Attribute struct {
	Guid       string  `json:"guid"`
	Key        string  `json:"key"`
	Value      *string `json:"value,omitempty"`
	Definition *string `json:"definition,omitempty"`
}

// AttributeDiff represents changes to an attribute entity.
// [👤semio📚go💻semio🔖attribute✂️attributediff](repo://p/u/semio/b/l/go/f/semio.go/s/Attribute/d/i/AttributeDiff)
type AttributeDiff struct {
	Key        *string `json:"key,omitempty"`
	Value      *string `json:"value,omitempty"`
	Definition *string `json:"definition,omitempty"`
}

// AttributesDiff represents a collection of attribute additions, removals and updates.
// [👤semio📚go💻semio🔖attribute✂️attributesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Attribute/d/i/AttributesDiff)
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
// [👤semio📚go💻semio🔖location](repo://p/u/semio/b/l/go/f/semio.go/s/Location)
// Location MUST define geographic location entities and their diff types.

// Location represents a geographic location with longitude, latitude and optional altitude.
// [👤semio📚go💻semio🔖location✂️location](repo://p/u/semio/b/l/go/f/semio.go/s/Location/d/i/Location)
type Location struct {
	Guid       string      `json:"guid"`
	Longitude  float64     `json:"longitude"`
	Latitude   float64     `json:"latitude"`
	Altitude   *float64    `json:"altitude,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
}

// LocationDiff represents changes to a location entity.
// [👤semio📚go💻semio🔖location✂️locationdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Location/d/i/LocationDiff)
type LocationDiff struct {
	Longitude  *float64        `json:"longitude,omitempty"`
	Latitude   *float64        `json:"latitude,omitempty"`
	Altitude   *float64        `json:"altitude,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// #endregion 🔖Location

// #region 🔖Author
// [👤semio📚go💻semio🔖author](repo://p/u/semio/b/l/go/f/semio.go/s/Author)
// Author MUST define authorship entities and their diff types.

// Author represents a named authorship entity with optional email.
// [👤semio📚go💻semio🔖author✂️author](repo://p/u/semio/b/l/go/f/semio.go/s/Author/d/i/Author)
type Author struct {
	Guid       string      `json:"guid"`
	Name       string      `json:"name"`
	Email      *string     `json:"email,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
	CreatedAt  string      `json:"createdAt,omitempty"`
	UpdatedAt  string      `json:"updatedAt,omitempty"`
}

// AuthorDiff represents changes to an author entity.
// [👤semio📚go💻semio🔖author✂️authordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Author/d/i/AuthorDiff)
type AuthorDiff struct {
	Name       *string         `json:"name,omitempty"`
	Email      *string         `json:"email,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// AuthorsDiff represents a collection of author additions, removals and updates.
// [👤semio📚go💻semio🔖author✂️authorsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Author/d/i/AuthorsDiff)
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
// [👤semio📚go💻semio🔖file](repo://p/u/semio/b/l/go/f/semio.go/s/File)
// File MUST define file reference entities and their diff types.

// File represents a file reference entity with name, remote URL and metadata.
// [👤semio📚go💻semio🔖file✂️file](repo://p/u/semio/b/l/go/f/semio.go/s/File/d/i/File)
type File struct {
	Guid        string      `json:"guid"`
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

// FileDiff represents changes to a file entity.
// [👤semio📚go💻semio🔖file✂️filediff](repo://p/u/semio/b/l/go/f/semio.go/s/File/d/i/FileDiff)
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

// FilesDiff represents a collection of file additions, removals and updates.
// [👤semio📚go💻semio🔖file✂️filesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/File/d/i/FilesDiff)
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
// [👤semio📚go💻semio🔖folder](repo://p/u/semio/b/l/go/f/semio.go/s/Folder)
// Folder MUST define folder hierarchy entities and their diff types.

// Folder represents a folder hierarchy entity with name and parent reference.
// [👤semio📚go💻semio🔖folder✂️folder](repo://p/u/semio/b/l/go/f/semio.go/s/Folder/d/i/Folder)
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
// [👤semio📚go💻semio🔖folder✂️folderdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Folder/d/i/FolderDiff)
type FolderDiff struct {
	Name        *string         `json:"name,omitempty"`
	Parent      *FolderId       `json:"parent,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// FoldersDiff represents a collection of folder additions, removals and updates.
// [👤semio📚go💻semio🔖folder✂️foldersdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Folder/d/i/FoldersDiff)
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
// [👤semio📚go💻semio🔖benchmark](repo://p/u/semio/b/l/go/f/semio.go/s/Benchmark)
// Benchmark MUST define benchmark threshold entities and their diff types.

// Benchmark represents a named metric threshold with min and max bounds.
// [👤semio📚go💻semio🔖benchmark✂️benchmark](repo://p/u/semio/b/l/go/f/semio.go/s/Benchmark/d/i/Benchmark)
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
// [👤semio📚go💻semio🔖benchmark✂️benchmarkdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Benchmark/d/i/BenchmarkDiff)
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
// [👤semio📚go💻semio🔖benchmark✂️benchmarksdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Benchmark/d/i/BenchmarksDiff)
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
// [👤semio📚go💻semio🔖quality](repo://p/u/semio/b/l/go/f/semio.go/s/Quality)
// Quality MUST define measurable quality entities and their diff types.

// QualityKind is a bitfield enum for quality scope classification.
// [👤semio📚go💻semio🔖quality✂️qualitykind](repo://p/u/semio/b/l/go/f/semio.go/s/Quality/d/i/QualityKind)
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
// [👤semio📚go💻semio🔖quality✂️quality](repo://p/u/semio/b/l/go/f/semio.go/s/Quality/d/i/Quality)
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
// [👤semio📚go💻semio🔖quality✂️qualitydiff](repo://p/u/semio/b/l/go/f/semio.go/s/Quality/d/i/QualityDiff)
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
// [👤semio📚go💻semio🔖quality✂️qualitiesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Quality/d/i/QualitiesDiff)
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
// [👤semio📚go💻semio🔖port](repo://p/u/semio/b/l/go/f/semio.go/s/Port)
// Port MUST define connector port entities and their diff types.

// Port represents a named connector port with compatible port references.
// [👤semio📚go💻semio🔖port✂️port](repo://p/u/semio/b/l/go/f/semio.go/s/Port/d/i/Port)
type Port struct {
	Guid            string      `json:"guid"`
	Name            string      `json:"name"`
	Description     *string     `json:"description,omitempty"`
	Icon            *string     `json:"icon,omitempty"`
	CompatiblePorts []PortId    `json:"compatiblePorts,omitempty"`
	Attributes      []Attribute `json:"attributes,omitempty"`
	CreatedAt       string      `json:"createdAt,omitempty"`
	UpdatedAt       string      `json:"updatedAt,omitempty"`
}

// PortDiff represents changes to a port entity.
// [👤semio📚go💻semio🔖port✂️portdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Port/d/i/PortDiff)
type PortDiff struct {
	Name            *string         `json:"name,omitempty"`
	Description     *string         `json:"description,omitempty"`
	Icon            *string         `json:"icon,omitempty"`
	CompatiblePorts []PortId        `json:"compatiblePorts,omitempty"`
	Attributes      *AttributesDiff `json:"attributes,omitempty"`
	setFields       map[string]bool `json:"-"`
}

// UnmarshalJSON MUST populate the setFields map for all present JSON keys.
// UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.
// [👤semio📚go💻semio🔖port🛠️unmarshaljson](repo://p/u/semio/b/l/go/f/semio.go/s/Port/d/i/UnmarshalJSON)
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
// [👤semio📚go💻semio🔖port🛠️hasfield](repo://p/u/semio/b/l/go/f/semio.go/s/Port/d/i/HasField)
func (d *PortDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// PortsDiff represents a collection of port additions, removals and updates.
// [👤semio📚go💻semio🔖port✂️portsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Port/d/i/PortsDiff)
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
// [👤semio📚go💻semio🔖prop](repo://p/u/semio/b/l/go/f/semio.go/s/Prop)
// Prop MUST define property value entities and their diff types.

// Prop represents a quality property value with optional unit.
// [👤semio📚go💻semio🔖prop✂️prop](repo://p/u/semio/b/l/go/f/semio.go/s/Prop/d/i/Prop)
type Prop struct {
	Guid       string      `json:"guid"`
	Quality    QualityId   `json:"quality"`
	Value      string      `json:"value"`
	Unit       *string     `json:"unit,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
}

// PropDiff represents changes to a prop entity.
// [👤semio📚go💻semio🔖prop✂️propdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Prop/d/i/PropDiff)
type PropDiff struct {
	Quality    *QualityId      `json:"quality,omitempty"`
	Value      *string         `json:"value,omitempty"`
	Unit       *string         `json:"unit,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// PropsDiff represents a collection of prop additions, removals and updates.
// [👤semio📚go💻semio🔖prop✂️propsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Prop/d/i/PropsDiff)
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
// [👤semio📚go💻semio🔖tag](repo://p/u/semio/b/l/go/f/semio.go/s/Tag)
// Tag MUST define tag classification entities and their diff types.

// Tag represents a named classification tag with optional description and icon.
// [👤semio📚go💻semio🔖tag✂️tag](repo://p/u/semio/b/l/go/f/semio.go/s/Tag/d/i/Tag)
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
// [👤semio📚go💻semio🔖tag✂️tagdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Tag/d/i/TagDiff)
type TagDiff struct {
	Name        *string         `json:"name,omitempty"`
	Description *string         `json:"description,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// UnmarshalJSON MUST populate the setFields map for all present JSON keys.
// UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.
// [👤semio📚go💻semio🔖tag🛠️unmarshaljson](repo://p/u/semio/b/l/go/f/semio.go/s/Tag/d/i/UnmarshalJSON)
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
// [👤semio📚go💻semio🔖tag🛠️hasfield](repo://p/u/semio/b/l/go/f/semio.go/s/Tag/d/i/HasField)
func (d *TagDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// TagsDiff represents a collection of tag additions, removals and updates.
// [👤semio📚go💻semio🔖tag✂️tagsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Tag/d/i/TagsDiff)
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
// [👤semio📚go💻semio🔖concept](repo://p/u/semio/b/l/go/f/semio.go/s/Concept)
// Concept MUST define concept categorization entities and their diff types.

// Concept represents a named categorization concept with optional description.
// [👤semio📚go💻semio🔖concept✂️concept](repo://p/u/semio/b/l/go/f/semio.go/s/Concept/d/i/Concept)
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
// [👤semio📚go💻semio🔖concept✂️conceptdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Concept/d/i/ConceptDiff)
type ConceptDiff struct {
	Name        *string         `json:"name,omitempty"`
	Description *string         `json:"description,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// UnmarshalJSON MUST populate the setFields map for all present JSON keys.
// UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.
// [👤semio📚go💻semio🔖concept🛠️unmarshaljson](repo://p/u/semio/b/l/go/f/semio.go/s/Concept/d/i/UnmarshalJSON)
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
// [👤semio📚go💻semio🔖concept🛠️hasfield](repo://p/u/semio/b/l/go/f/semio.go/s/Concept/d/i/HasField)
func (d *ConceptDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// ConceptsDiff represents a collection of concept additions, removals and updates.
// [👤semio📚go💻semio🔖concept✂️conceptsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Concept/d/i/ConceptsDiff)
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
// [👤semio📚go💻semio🔖model](repo://p/u/semio/b/l/go/f/semio.go/s/Model)
// Model MUST define 3D model reference entities and their diff types.

// Model represents a 3D model reference associated with a file and tags.
// [👤semio📚go💻semio🔖model✂️model](repo://p/u/semio/b/l/go/f/semio.go/s/Model/d/i/Model)
type Model struct {
	Guid        string      `json:"guid"`
	File        FileId      `json:"file"`
	Name        *string     `json:"name,omitempty"`
	Tags        []TagId     `json:"tags,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// ModelDiff represents changes to a model entity.
// [👤semio📚go💻semio🔖model✂️modeldiff](repo://p/u/semio/b/l/go/f/semio.go/s/Model/d/i/ModelDiff)
type ModelDiff struct {
	File        *FileId         `json:"file,omitempty"`
	Name        *string         `json:"name,omitempty"`
	Tags        []TagId         `json:"tags,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// ModelsDiff represents a collection of model additions, removals and updates.
// [👤semio📚go💻semio🔖model✂️modelsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Model/d/i/ModelsDiff)
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
// [👤semio📚go💻semio🔖connector](repo://p/u/semio/b/l/go/f/semio.go/s/Connector)
// Connector MUST define spatial connector entities and their diff types.

// Connector represents a spatial connection point on a type with position and direction.
// [👤semio📚go💻semio🔖connector✂️connector](repo://p/u/semio/b/l/go/f/semio.go/s/Connector/d/i/Connector)
type Connector struct {
	Guid        string      `json:"guid"`
	Name        *string     `json:"name,omitempty"`
	Point       Point       `json:"point"`
	Direction   Vector      `json:"direction"`
	T           float64     `json:"t"`
	Mandatory   *bool       `json:"mandatory,omitempty"`
	Port        *PortId     `json:"port,omitempty"`
	Props       []Prop      `json:"props,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// PointDiff represents changes to a 3D point.
// [👤semio📚go💻semio🔖connector✂️pointdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Connector/d/i/PointDiff)
type PointDiff struct {
	X *float64 `json:"x,omitempty"`
	Y *float64 `json:"y,omitempty"`
	Z *float64 `json:"z,omitempty"`
}

// VectorDiff represents changes to a 3D vector.
// [👤semio📚go💻semio🔖connector✂️vectordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Connector/d/i/VectorDiff)
type VectorDiff struct {
	X *float64 `json:"x,omitempty"`
	Y *float64 `json:"y,omitempty"`
	Z *float64 `json:"z,omitempty"`
}

// ConnectorDiff represents changes to a connector entity.
// [👤semio📚go💻semio🔖connector✂️connectordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Connector/d/i/ConnectorDiff)
type ConnectorDiff struct {
	Name        *string         `json:"name,omitempty"`
	Point       *PointDiff      `json:"point,omitempty"`
	Direction   *VectorDiff     `json:"direction,omitempty"`
	T           *float64        `json:"t,omitempty"`
	Mandatory   *bool           `json:"mandatory,omitempty"`
	Port        *PortId         `json:"port,omitempty"`
	Props       *PropsDiff      `json:"props,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// ConnectorsDiff represents a collection of connector additions, removals and updates.
// [👤semio📚go💻semio🔖connector✂️connectorsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Connector/d/i/ConnectorsDiff)
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
// [👤semio📚go💻semio🔖type](repo://p/u/semio/b/l/go/f/semio.go/s/Type)
// Type MUST define component type entities and their diff types.

// Type represents a component type with models, connectors and hierarchical inheritance.
// [👤semio📚go💻semio🔖type✂️type](repo://p/u/semio/b/l/go/f/semio.go/s/Type/d/i/Type)
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
	Concepts    []ConceptId `json:"concepts,omitempty"`
	Icon        *string     `json:"icon,omitempty"`
	Image       *string     `json:"image,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
	CreatedAt   string      `json:"createdAt,omitempty"`
	UpdatedAt   string      `json:"updatedAt,omitempty"`
}

// TypeDiff represents changes to a type entity.
// [👤semio📚go💻semio🔖type✂️typediff](repo://p/u/semio/b/l/go/f/semio.go/s/Type/d/i/TypeDiff)
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
	Concepts    []ConceptId     `json:"concepts,omitempty"`
	Icon        *string         `json:"icon,omitempty"`
	Image       *string         `json:"image,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
	setFields   map[string]bool `json:"-"`
}

// UnmarshalJSON MUST populate the setFields map for all present JSON keys.
// UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.
// [👤semio📚go💻semio🔖type🛠️unmarshaljson](repo://p/u/semio/b/l/go/f/semio.go/s/Type/d/i/UnmarshalJSON)
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
// [👤semio📚go💻semio🔖type🛠️hasfield](repo://p/u/semio/b/l/go/f/semio.go/s/Type/d/i/HasField)
func (d *TypeDiff) HasField(field string) bool {
	if d.setFields == nil {
		return false
	}
	return d.setFields[field]
}

// TypesDiff represents a collection of type additions, removals and updates.
// [👤semio📚go💻semio🔖type✂️typesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Type/d/i/TypesDiff)
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
// [👤semio📚go💻semio🔖layer](repo://p/u/semio/b/l/go/f/semio.go/s/Layer)
// Layer MUST define layer hierarchy entities and their diff types.

// Layer represents a named layer with visibility, lock and color properties.
// [👤semio📚go💻semio🔖layer✂️layer](repo://p/u/semio/b/l/go/f/semio.go/s/Layer/d/i/Layer)
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
// [👤semio📚go💻semio🔖layer✂️layerdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Layer/d/i/LayerDiff)
type LayerDiff struct {
	Path        *string         `json:"path,omitempty"`
	IsHidden    *bool           `json:"isHidden,omitempty"`
	IsLocked    *bool           `json:"isLocked,omitempty"`
	Color       *string         `json:"color,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// LayersDiff represents a collection of layer additions, removals and updates.
// [👤semio📚go💻semio🔖layer✂️layersdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Layer/d/i/LayersDiff)
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
// [👤semio📚go💻semio🔖piece](repo://p/u/semio/b/l/go/f/semio.go/s/Piece)
// Piece MUST define placed piece entities and their diff types.

// Piece represents a placed component instance within a design.
// [👤semio📚go💻semio🔖piece✂️piece](repo://p/u/semio/b/l/go/f/semio.go/s/Piece/d/i/Piece)
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
// [👤semio📚go💻semio🔖piece✂️coorddiff](repo://p/u/semio/b/l/go/f/semio.go/s/Piece/d/i/CoordDiff)
type CoordDiff struct {
	U *float64 `json:"u,omitempty"`
	V *float64 `json:"v,omitempty"`
}

// PlaneDiff represents changes to a 3D plane.
// [👤semio📚go💻semio🔖piece✂️planediff](repo://p/u/semio/b/l/go/f/semio.go/s/Piece/d/i/PlaneDiff)
type PlaneDiff struct {
	Origin *PointDiff  `json:"origin,omitempty"`
	XAxis  *VectorDiff `json:"xAxis,omitempty"`
	YAxis  *VectorDiff `json:"yAxis,omitempty"`
}

// PieceDiff represents changes to a piece entity.
// [👤semio📚go💻semio🔖piece✂️piecediff](repo://p/u/semio/b/l/go/f/semio.go/s/Piece/d/i/PieceDiff)
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
// [👤semio📚go💻semio🔖piece✂️piecesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Piece/d/i/PiecesDiff)
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
// [👤semio📚go💻semio🔖group](repo://p/u/semio/b/l/go/f/semio.go/s/Group)
// Group MUST define piece grouping entities and their diff types.

// Group represents a named collection of pieces within a design.
// [👤semio📚go💻semio🔖group✂️group](repo://p/u/semio/b/l/go/f/semio.go/s/Group/d/i/Group)
type Group struct {
	Guid        string      `json:"guid"`
	Pieces      []PieceId   `json:"pieces,omitempty"`
	Name        *string     `json:"name,omitempty"`
	Color       *string     `json:"color,omitempty"`
	Description *string     `json:"description,omitempty"`
	Attributes  []Attribute `json:"attributes,omitempty"`
}

// GroupDiff represents changes to a group entity.
// [👤semio📚go💻semio🔖group✂️groupdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Group/d/i/GroupDiff)
type GroupDiff struct {
	Pieces      []PieceId       `json:"pieces,omitempty"`
	Name        *string         `json:"name,omitempty"`
	Color       *string         `json:"color,omitempty"`
	Description *string         `json:"description,omitempty"`
	Attributes  *AttributesDiff `json:"attributes,omitempty"`
}

// GroupsDiff represents a collection of group additions, removals and updates.
// [👤semio📚go💻semio🔖group✂️groupsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Group/d/i/GroupsDiff)
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
// [👤semio📚go💻semio🔖side](repo://p/u/semio/b/l/go/f/semio.go/s/Side)
// Side MUST define connection side reference entities and their diff types.

// Side represents one end of a connection referencing a piece and optional connector.
// [👤semio📚go💻semio🔖side✂️side](repo://p/u/semio/b/l/go/f/semio.go/s/Side/d/i/Side)
type Side struct {
	Piece       PieceId      `json:"piece"`
	DesignPiece *PieceId     `json:"designPiece,omitempty"`
	Connector   *ConnectorId `json:"connector,omitempty"`
}

// SideDiff represents changes to a connection side.
// [👤semio📚go💻semio🔖side✂️sidediff](repo://p/u/semio/b/l/go/f/semio.go/s/Side/d/i/SideDiff)
type SideDiff struct {
	Piece       *PieceId     `json:"piece,omitempty"`
	DesignPiece *PieceId     `json:"designPiece,omitempty"`
	Connector   *ConnectorId `json:"connector,omitempty"`
}

// #endregion 🔖Side

// #region 🔖Connection
// [👤semio📚go💻semio🔖connection](repo://p/u/semio/b/l/go/f/semio.go/s/Connection)
// Connection MUST define spatial connection entities and their diff types.

// Connection represents a spatial relationship between two pieces with transform parameters.
// [👤semio📚go💻semio🔖connection✂️connection](repo://p/u/semio/b/l/go/f/semio.go/s/Connection/d/i/Connection)
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
// [👤semio📚go💻semio🔖connection✂️connectiondiff](repo://p/u/semio/b/l/go/f/semio.go/s/Connection/d/i/ConnectionDiff)
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
// [👤semio📚go💻semio🔖connection✂️connectionsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Connection/d/i/ConnectionsDiff)
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
// [👤semio📚go💻semio🔖stat](repo://p/u/semio/b/l/go/f/semio.go/s/Stat)
// Stat MUST define statistical measure entities and their diff types.

// Stat represents a statistical quality measurement with min and max bounds.
// [👤semio📚go💻semio🔖stat✂️stat](repo://p/u/semio/b/l/go/f/semio.go/s/Stat/d/i/Stat)
type Stat struct {
	Guid       string      `json:"guid"`
	Quality    QualityId   `json:"quality"`
	Min        *float64    `json:"min,omitempty"`
	Max        *float64    `json:"max,omitempty"`
	Unit       *string     `json:"unit,omitempty"`
	Attributes []Attribute `json:"attributes,omitempty"`
}

// StatDiff represents changes to a stat entity.
// [👤semio📚go💻semio🔖stat✂️statdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Stat/d/i/StatDiff)
type StatDiff struct {
	Quality    *QualityId      `json:"quality,omitempty"`
	Min        *float64        `json:"min,omitempty"`
	Max        *float64        `json:"max,omitempty"`
	Unit       *string         `json:"unit,omitempty"`
	Attributes *AttributesDiff `json:"attributes,omitempty"`
}

// StatsDiff represents a collection of stat additions, removals and updates.
// [👤semio📚go💻semio🔖stat✂️statsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Stat/d/i/StatsDiff)
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
// [👤semio📚go💻semio🔖design](repo://p/u/semio/b/l/go/f/semio.go/s/Design)
// Design MUST define assembly design entities and their diff types.

// Design represents an assembly of pieces, connections, layers and groups.
// [👤semio📚go💻semio🔖design✂️design](repo://p/u/semio/b/l/go/f/semio.go/s/Design/d/i/Design)
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
	Concepts    []ConceptId  `json:"concepts,omitempty"`
	Icon        *string      `json:"icon,omitempty"`
	Image       *string      `json:"image,omitempty"`
	Description *string      `json:"description,omitempty"`
	Attributes  []Attribute  `json:"attributes,omitempty"`
	CreatedAt   string       `json:"createdAt,omitempty"`
	UpdatedAt   string       `json:"updatedAt,omitempty"`
}

// CameraDiff represents changes to a camera view.
// [👤semio📚go💻semio🔖design✂️cameradiff](repo://p/u/semio/b/l/go/f/semio.go/s/Design/d/i/CameraDiff)
type CameraDiff struct {
	Position *PointDiff  `json:"position,omitempty"`
	Forward  *VectorDiff `json:"forward,omitempty"`
	Up       *VectorDiff `json:"up,omitempty"`
}

// DesignDiff represents changes to a design entity.
// [👤semio📚go💻semio🔖design✂️designdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Design/d/i/DesignDiff)
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
	Concepts    []ConceptId      `json:"concepts,omitempty"`
	Icon        *string          `json:"icon,omitempty"`
	Image       *string          `json:"image,omitempty"`
	Description *string          `json:"description,omitempty"`
	Attributes  *AttributesDiff  `json:"attributes,omitempty"`
}

// DesignsDiff represents a collection of design additions, removals and updates.
// [👤semio📚go💻semio🔖design✂️designsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Design/d/i/DesignsDiff)
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
// [👤semio📚go💻semio🔖kit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit)
// Kit MUST define the root kit container entity and its diff types.

// Kit represents the root container for all domain entities.
// [👤semio📚go💻semio🔖kit✂️kit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/Kit)
type Kit struct {
	Guid        string      `json:"guid"`
	Name        string      `json:"name"`
	Version     string      `json:"version"`
	Types       []Type      `json:"types,omitempty"`
	Designs     []Design    `json:"designs,omitempty"`
	Tags        []Tag       `json:"tags,omitempty"`
	Concepts    []Concept   `json:"concepts,omitempty"`
	Ports       []Port      `json:"ports,omitempty"`
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
// [👤semio📚go💻semio🔖kit✂️kitdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/KitDiff)
type KitDiff struct {
	Name        *string         `json:"name,omitempty"`
	Version     *string         `json:"version,omitempty"`
	Types       *TypesDiff      `json:"types,omitempty"`
	Designs     *DesignsDiff    `json:"designs,omitempty"`
	Tags        *TagsDiff       `json:"tags,omitempty"`
	Concepts    *ConceptsDiff   `json:"concepts,omitempty"`
	Ports       *PortsDiff      `json:"ports,omitempty"`
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
// [👤semio📚go💻semio🔖kit✂️kitsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/KitsDiff)
type KitsDiff struct {
	Removed []KitId `json:"removed,omitempty"`
	Updated []struct {
		Kit  KitId   `json:"kit"`
		Diff KitDiff `json:"diff"`
	} `json:"updated,omitempty"`
	Added []Kit `json:"added,omitempty"`
}

// Change represents a reversible entity change with forward and backward diffs.
// [👤semio📚go💻semio🔖kit✂️change](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/Change)
type Change[TEntity any, TDiff any] struct {
	Forward  TDiff    `json:"forward"`
	Backward TDiff    `json:"backward"`
	Author   *string  `json:"author,omitempty"`
	Time     *string  `json:"time,omitempty"`
	Before   *TEntity `json:"before,omitempty"`
	After    *TEntity `json:"after,omitempty"`
}

// [👤semio📚go💻semio🔖kit✂️attributechange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/AttributeChange)
// AttributeChange holds the data fields for a AttributeChange record.
type AttributeChange = Change[Attribute, AttributeDiff]

// [👤semio📚go💻semio🔖kit✂️locationchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/LocationChange)
// LocationChange holds the data fields for a LocationChange record.
type LocationChange = Change[Location, LocationDiff]

// [👤semio📚go💻semio🔖kit✂️authorchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/AuthorChange)
// AuthorChange holds the data fields for a AuthorChange record.
type AuthorChange = Change[Author, AuthorDiff]

// [👤semio📚go💻semio🔖kit✂️filechange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/FileChange)
// FileChange holds the data fields for a FileChange record.
type FileChange = Change[File, FileDiff]

// [👤semio📚go💻semio🔖kit✂️folderchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/FolderChange)
// FolderChange holds the data fields for a FolderChange record.
type FolderChange = Change[Folder, FolderDiff]

// [👤semio📚go💻semio🔖kit✂️benchmarkchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/BenchmarkChange)
// BenchmarkChange holds the data fields for a BenchmarkChange record.
type BenchmarkChange = Change[Benchmark, BenchmarkDiff]

// [👤semio📚go💻semio🔖kit✂️qualitychange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/QualityChange)
// QualityChange holds the data fields for a QualityChange record.
type QualityChange = Change[Quality, QualityDiff]

// [👤semio📚go💻semio🔖kit✂️portchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/PortChange)
// PortChange holds the data fields for a PortChange record.
type PortChange = Change[Port, PortDiff]

// [👤semio📚go💻semio🔖kit✂️propchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/PropChange)
// PropChange holds the data fields for a PropChange record.
type PropChange = Change[Prop, PropDiff]

// [👤semio📚go💻semio🔖kit✂️tagchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/TagChange)
// TagChange holds the data fields for a TagChange record.
type TagChange = Change[Tag, TagDiff]

// [👤semio📚go💻semio🔖kit✂️conceptchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/ConceptChange)
// ConceptChange holds the data fields for a ConceptChange record.
type ConceptChange = Change[Concept, ConceptDiff]

// [👤semio📚go💻semio🔖kit✂️modelchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/ModelChange)
// ModelChange holds the data fields for a ModelChange record.
type ModelChange = Change[Model, ModelDiff]

// [👤semio📚go💻semio🔖kit✂️connectorchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/ConnectorChange)
// ConnectorChange holds the data fields for a ConnectorChange record.
type ConnectorChange = Change[Connector, ConnectorDiff]

// [👤semio📚go💻semio🔖kit✂️typechange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/TypeChange)
// TypeChange holds the data fields for a TypeChange record.
type TypeChange = Change[Type, TypeDiff]

// [👤semio📚go💻semio🔖kit✂️layerchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/LayerChange)
// LayerChange holds the data fields for a LayerChange record.
type LayerChange = Change[Layer, LayerDiff]

// [👤semio📚go💻semio🔖kit✂️piecechange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/PieceChange)
// PieceChange holds the data fields for a PieceChange record.
type PieceChange = Change[Piece, PieceDiff]

// [👤semio📚go💻semio🔖kit✂️groupchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/GroupChange)
// GroupChange holds the data fields for a GroupChange record.
type GroupChange = Change[Group, GroupDiff]

// [👤semio📚go💻semio🔖kit✂️sidechange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/SideChange)
// SideChange holds the data fields for a SideChange record.
type SideChange = Change[Side, SideDiff]

// [👤semio📚go💻semio🔖kit✂️connectionchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/ConnectionChange)
// ConnectionChange holds the data fields for a ConnectionChange record.
type ConnectionChange = Change[Connection, ConnectionDiff]

// [👤semio📚go💻semio🔖kit✂️statchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/StatChange)
// StatChange holds the data fields for a StatChange record.
type StatChange = Change[Stat, StatDiff]

// [👤semio📚go💻semio🔖kit✂️designchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/DesignChange)
// DesignChange holds the data fields for a DesignChange record.
type DesignChange = Change[Design, DesignDiff]

// [👤semio📚go💻semio🔖kit✂️kitchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/KitChange)
// KitChange holds the data fields for a KitChange record.
type KitChange = Change[Kit, KitDiff]

// [👤semio📚go💻semio🔖kit🛠️getdesignchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/GetDesignChange)
// GetDesignChange holds the data fields for a GetDesignChange record.
// GetDesignChange MUST perform the GetDesignChange operation.
func GetDesignChange(before, after Design, author *string, time *string) DesignChange {
	forward := getDesignDiff(before, after)
	backward := inverseDesignDiff(before, forward)
	return DesignChange{Forward: forward, Backward: backward, Author: author, Time: time, Before: &before, After: &after}
}

// [👤semio📚go💻semio🔖kit🛠️getkitchange](repo://p/u/semio/b/l/go/f/semio.go/s/Kit/d/i/GetKitChange)
// GetKitChange holds the data fields for a GetKitChange record.
// GetKitChange MUST perform the GetKitChange operation.
func GetKitChange(before, after Kit, author *string, time *string) KitChange {
	forward := GetKitDiff(before, after)
	backward := InverseKitDiff(before, forward)
	return KitChange{Forward: forward, Backward: backward, Author: author, Time: time, Before: &before, After: &after}
}

// #endregion 🔖Kit

// #region 🔖Serialization
// [👤semio📚go💻semio🔖serialization](repo://p/u/semio/b/l/go/f/semio.go/s/Serialization)
// Serialization MUST provide JSON marshaling and unmarshaling for kit data.

// SerializeKit MUST return valid JSON with two-space indentation.
// SerializeKit marshals a kit to indented JSON bytes.
// [👤semio📚go💻semio🔖serialization🛠️serializekit](repo://p/u/semio/b/l/go/f/semio.go/s/Serialization/d/i/SerializeKit)
func SerializeKit(kit Kit) ([]byte, error) {
	return json.MarshalIndent(kit, "", "  ")
}

// DeserializeKit MUST return an error if the data is not valid kit JSON.
// DeserializeKit unmarshals JSON bytes into a kit.
// [👤semio📚go💻semio🔖serialization🛠️deserializekit](repo://p/u/semio/b/l/go/f/semio.go/s/Serialization/d/i/DeserializeKit)
func DeserializeKit(data []byte) (Kit, error) {
	var kit Kit
	err := json.Unmarshal(data, &kit)
	return kit, err
}

// SerializeKitDiff MUST return valid JSON with two-space indentation.
// SerializeKitDiff marshals a kit diff to indented JSON bytes.
// [👤semio📚go💻semio🔖serialization🛠️serializekitdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Serialization/d/i/SerializeKitDiff)
func SerializeKitDiff(diff KitDiff) ([]byte, error) {
	return json.MarshalIndent(diff, "", "  ")
}

// DeserializeKitDiff MUST return an error if the data is not valid kit diff JSON.
// DeserializeKitDiff unmarshals JSON bytes into a kit diff.
// [👤semio📚go💻semio🔖serialization🛠️deserializekitdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Serialization/d/i/DeserializeKitDiff)
func DeserializeKitDiff(data []byte) (KitDiff, error) {
	var diff KitDiff
	err := json.Unmarshal(data, &diff)
	return diff, err
}

// #endregion 🔖Serialization

// #region 🔖Helpers
// [👤semio📚go💻semio🔖helpers](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers)
// Helpers MUST provide lookup functions for finding entities within kits.

// FindTypeInKit MUST return nil when no type matches the GUID.
// FindTypeInKit returns a pointer to the type with the given GUID or nil.
// [👤semio📚go💻semio🔖helpers🛠️findtypeinkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindTypeInKit)
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
// [👤semio📚go💻semio🔖helpers🛠️finddesigninkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindDesignInKit)
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
// [👤semio📚go💻semio🔖helpers🛠️findpieceindesign](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindPieceInDesign)
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
// [👤semio📚go💻semio🔖helpers🛠️findconnectionindesign](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindConnectionInDesign)
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
// [👤semio📚go💻semio🔖helpers🛠️findconnectorintype](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindConnectorInType)
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
// [👤semio📚go💻semio🔖helpers🛠️findfileinkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindFileInKit)
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
// [👤semio📚go💻semio🔖helpers🛠️findfolderinkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindFolderInKit)
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
// [👤semio📚go💻semio🔖helpers🛠️findqualityinkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindQualityInKit)
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
// [👤semio📚go💻semio🔖helpers🛠️findportinkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindPortInKit)
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
// [👤semio📚go💻semio🔖helpers🛠️findtaginkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindTagInKit)
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
// [👤semio📚go💻semio🔖helpers🛠️findconceptinkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindConceptInKit)
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
// [👤semio📚go💻semio🔖helpers🛠️findauthorinkit](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/FindAuthorInKit)
func FindAuthorInKit(kit *Kit, authorGuid string) *Author {
	for i := range kit.Authors {
		if kit.Authors[i].Guid == authorGuid {
			return &kit.Authors[i]
		}
	}
	return nil
}

// SumQualityInDesign MUST sum up the values of a quality across all pieces in a design.
// For each piece, uses the piece-level prop if present, otherwise falls back to the type-level prop.
// [👤semio📚go💻semio🔖helpers🛠️sumqualityindesign](repo://p/u/semio/b/l/go/f/semio.go/s/Helpers/d/i/SumQualityInDesign)
func SumQualityInDesign(kit *Kit, designGuid string, qualityGuid string) float64 {
	design := FindDesignInKit(kit, designGuid)
	if design == nil {
		return 0
	}
	total := 0.0
	for _, piece := range design.Pieces {
		var found bool
		for _, prop := range piece.Props {
			if prop.Quality.Guid == qualityGuid {
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
		typ := FindTypeInKit(kit, piece.Type.Guid)
		if typ == nil {
			continue
		}
		for _, prop := range typ.Props {
			if prop.Quality.Guid == qualityGuid {
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

// #endregion 🔖Helpers

// #region 🔖Factories
// [👤semio📚go💻semio🔖factories](repo://p/u/semio/b/l/go/f/semio.go/s/Factories)
// Factories MUST provide constructor functions for creating new domain entities.

// NewKit MUST generate a unique GUID and set version to 0.0.1.
// NewKit creates a new kit with the given name and a generated GUID.
// [👤semio📚go💻semio🔖factories🛠️newkit](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewKit)
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
// [👤semio📚go💻semio🔖factories🛠️newtype](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewType)
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
// [👤semio📚go💻semio🔖factories🛠️newdesign](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewDesign)
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
// [👤semio📚go💻semio🔖factories🛠️newpiece](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewPiece)
func NewPiece() Piece {
	return Piece{
		Guid: Guid(),
	}
}

// NewConnection MUST generate a unique GUID and set both connected and connecting sides.
// NewConnection creates a new connection between two pieces by their GUIDs.
// [👤semio📚go💻semio🔖factories🛠️newconnection](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewConnection)
func NewConnection(connectedPieceGuid, connectingPieceGuid string) Connection {
	return Connection{
		Guid:       Guid(),
		Connected:  Side{Piece: PieceId{Guid: connectedPieceGuid}},
		Connecting: Side{Piece: PieceId{Guid: connectingPieceGuid}},
	}
}

// NewConnector MUST generate a unique GUID for the new connector.
// NewConnector creates a new connector with position, direction and parameter t.
// [👤semio📚go💻semio🔖factories🛠️newconnector](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewConnector)
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
// [👤semio📚go💻semio🔖factories🛠️newfile](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewFile)
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
// [👤semio📚go💻semio🔖factories🛠️newfolder](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewFolder)
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
// [👤semio📚go💻semio🔖factories🛠️newquality](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewQuality)
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
// [👤semio📚go💻semio🔖factories🛠️newport](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewPort)
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
// [👤semio📚go💻semio🔖factories🛠️newtag](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewTag)
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
// [👤semio📚go💻semio🔖factories🛠️newconcept](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewConcept)
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
// [👤semio📚go💻semio🔖factories🛠️newauthor](repo://p/u/semio/b/l/go/f/semio.go/s/Factories/d/i/NewAuthor)
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
// [👤semio📚go💻semio🔖kitoperations](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations)
// Kit Operations MUST provide comparison, diffing, and application of kit changes.

// AreKitsEqual MUST compare all entities by GUID and structural fields.
// AreKitsEqual compares two kits for structural equality.
// [👤semio📚go💻semio🔖kitoperations🛠️arekitsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/AreKitsEqual)
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
// [👤semio📚go💻semio🔖kitoperations🛠️arekitdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/AreKitDiffsEqual)
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

// [👤semio📚go💻semio🔖kitoperations🛠️aretypesdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areTypesDiffsEqual)
// areTypesDiffsEqual holds the data fields for a areTypesDiffsEqual record.
// areTypesDiffsEqual MUST perform the areTypesDiffsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️aredesignsdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areDesignsDiffsEqual)
// areDesignsDiffsEqual holds the data fields for a areDesignsDiffsEqual record.
// areDesignsDiffsEqual MUST perform the areDesignsDiffsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️aretagsdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areTagsDiffsEqual)
// areTagsDiffsEqual holds the data fields for a areTagsDiffsEqual record.
// areTagsDiffsEqual MUST perform the areTagsDiffsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️areconceptsdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areConceptsDiffsEqual)
// areConceptsDiffsEqual holds the data fields for a areConceptsDiffsEqual record.
// areConceptsDiffsEqual MUST perform the areConceptsDiffsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️areportsdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/arePortsDiffsEqual)
// arePortsDiffsEqual holds the data fields for a arePortsDiffsEqual record.
// arePortsDiffsEqual MUST perform the arePortsDiffsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️arefilesdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areFilesDiffsEqual)
// areFilesDiffsEqual holds the data fields for a areFilesDiffsEqual record.
// areFilesDiffsEqual MUST perform the areFilesDiffsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️arefoldersdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areFoldersDiffsEqual)
// areFoldersDiffsEqual holds the data fields for a areFoldersDiffsEqual record.
// areFoldersDiffsEqual MUST perform the areFoldersDiffsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️areauthorsdiffsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areAuthorsDiffsEqual)
// areAuthorsDiffsEqual holds the data fields for a areAuthorsDiffsEqual record.
// areAuthorsDiffsEqual MUST perform the areAuthorsDiffsEqual operation.
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
// [👤semio📚go💻semio🔖kitoperations🛠️getkitdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/GetKitDiff)
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
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️gettypesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getTypesDiff)
// getTypesDiff holds the data fields for a getTypesDiff record.
// getTypesDiff MUST perform the getTypesDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️gettypediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getTypeDiff)
// getTypeDiff holds the data fields for a getTypeDiff record.
// getTypeDiff MUST perform the getTypeDiff operation.
func getTypeDiff(before, after Type) TypeDiff {
	diff := TypeDiff{}
	diff.setFields = make(map[string]bool)
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if !areTypeIdsEqual(before.Parent, after.Parent) {
		diff.Parent = after.Parent
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
	modelsDiff := getModelsDiff(before.Models, after.Models)
	if len(modelsDiff.Added) > 0 || len(modelsDiff.Removed) > 0 || len(modelsDiff.Updated) > 0 {
		diff.Models = &modelsDiff
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

// [👤semio📚go💻semio🔖kitoperations🛠️istypediffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isTypeDiffEmpty)
// isTypeDiffEmpty holds the data fields for a isTypeDiffEmpty record.
// isTypeDiffEmpty MUST perform the isTypeDiffEmpty operation.
func isTypeDiffEmpty(diff TypeDiff) bool {
	return diff.Name == nil && diff.Parent == nil && diff.IsAbstract == nil && diff.Virtual == nil && diff.Unit == nil && diff.Stock == nil && diff.Location == nil && diff.Folder == nil && diff.Icon == nil && diff.Image == nil && diff.Description == nil && diff.Authors == nil && diff.Concepts == nil && diff.Connectors == nil && diff.Models == nil && diff.Props == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getdesignsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getDesignsDiff)
// getDesignsDiff holds the data fields for a getDesignsDiff record.
// getDesignsDiff MUST perform the getDesignsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️getdesigndiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getDesignDiff)
// getDesignDiff holds the data fields for a getDesignDiff record.
// getDesignDiff MUST perform the getDesignDiff operation.
func getDesignDiff(before, after Design) DesignDiff {
	diff := DesignDiff{}
	if before.Name != after.Name {
		diff.Name = &after.Name
	}
	if !areDesignIdsEqual(before.Parent, after.Parent) {
		diff.Parent = after.Parent
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

// [👤semio📚go💻semio🔖kitoperations🛠️isdesigndiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isDesignDiffEmpty)
// isDesignDiffEmpty holds the data fields for a isDesignDiffEmpty record.
// isDesignDiffEmpty MUST perform the isDesignDiffEmpty operation.
func isDesignDiffEmpty(diff DesignDiff) bool {
	return diff.Name == nil && diff.Parent == nil && diff.IsAbstract == nil && diff.Unit == nil && diff.Folder == nil && diff.CanScale == nil && diff.CanMirror == nil && diff.ActiveLayer == nil && diff.Location == nil && diff.Icon == nil && diff.Image == nil && diff.Description == nil && diff.Authors == nil && diff.Concepts == nil && diff.Pieces == nil && diff.Connections == nil && diff.Stats == nil && diff.Props == nil && diff.Layers == nil && diff.Groups == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️gettagsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getTagsDiff)
// getTagsDiff holds the data fields for a getTagsDiff record.
// getTagsDiff MUST perform the getTagsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️gettagdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getTagDiff)
// getTagDiff holds the data fields for a getTagDiff record.
// getTagDiff MUST perform the getTagDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️istagdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isTagDiffEmpty)
// isTagDiffEmpty holds the data fields for a isTagDiffEmpty record.
// isTagDiffEmpty MUST perform the isTagDiffEmpty operation.
func isTagDiffEmpty(diff TagDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Icon == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getconceptsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getConceptsDiff)
// getConceptsDiff holds the data fields for a getConceptsDiff record.
// getConceptsDiff MUST perform the getConceptsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️getconceptdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getConceptDiff)
// getConceptDiff holds the data fields for a getConceptDiff record.
// getConceptDiff MUST perform the getConceptDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️isconceptdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isConceptDiffEmpty)
// isConceptDiffEmpty holds the data fields for a isConceptDiffEmpty record.
// isConceptDiffEmpty MUST perform the isConceptDiffEmpty operation.
func isConceptDiffEmpty(diff ConceptDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Icon == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getportsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getPortsDiff)
// getPortsDiff holds the data fields for a getPortsDiff record.
// getPortsDiff MUST perform the getPortsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️getportdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getPortDiff)
// getPortDiff holds the data fields for a getPortDiff record.
// getPortDiff MUST perform the getPortDiff operation.
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
	if !arePortIdSlicesEqual(before.CompatiblePorts, after.CompatiblePorts) {
		diff.CompatiblePorts = after.CompatiblePorts
	}
	attrsDiff := getAttributesDiff(before.Attributes, after.Attributes)
	if !isAttributesDiffEmpty(attrsDiff) {
		diff.Attributes = &attrsDiff
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️isportdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isPortDiffEmpty)
// isPortDiffEmpty holds the data fields for a isPortDiffEmpty record.
// isPortDiffEmpty MUST perform the isPortDiffEmpty operation.
func isPortDiffEmpty(diff PortDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Icon == nil && diff.CompatiblePorts == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getfilesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getFilesDiff)
// getFilesDiff holds the data fields for a getFilesDiff record.
// getFilesDiff MUST perform the getFilesDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️getfilediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getFileDiff)
// getFileDiff holds the data fields for a getFileDiff record.
// getFileDiff MUST perform the getFileDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️isfilediffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isFileDiffEmpty)
// isFileDiffEmpty holds the data fields for a isFileDiffEmpty record.
// isFileDiffEmpty MUST perform the isFileDiffEmpty operation.
func isFileDiffEmpty(diff FileDiff) bool {
	return diff.Name == nil && diff.Remote == nil && diff.Folder == nil && diff.Size == nil && diff.Hash == nil && diff.Blob == nil && diff.Description == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getfoldersdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getFoldersDiff)
// getFoldersDiff holds the data fields for a getFoldersDiff record.
// getFoldersDiff MUST perform the getFoldersDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️getfolderdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getFolderDiff)
// getFolderDiff holds the data fields for a getFolderDiff record.
// getFolderDiff MUST perform the getFolderDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️isfolderdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isFolderDiffEmpty)
// isFolderDiffEmpty holds the data fields for a isFolderDiffEmpty record.
// isFolderDiffEmpty MUST perform the isFolderDiffEmpty operation.
func isFolderDiffEmpty(diff FolderDiff) bool {
	return diff.Name == nil && diff.Parent == nil && diff.Description == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getauthorsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getAuthorsDiff)
// getAuthorsDiff holds the data fields for a getAuthorsDiff record.
// getAuthorsDiff MUST perform the getAuthorsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️getauthordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getAuthorDiff)
// getAuthorDiff holds the data fields for a getAuthorDiff record.
// getAuthorDiff MUST perform the getAuthorDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️isauthordiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isAuthorDiffEmpty)
// isAuthorDiffEmpty holds the data fields for a isAuthorDiffEmpty record.
// isAuthorDiffEmpty MUST perform the isAuthorDiffEmpty operation.
func isAuthorDiffEmpty(diff AuthorDiff) bool {
	return diff.Name == nil && diff.Email == nil && diff.Attributes == nil
}

// InverseKitDiff MUST return a diff that when applied restores the original state.
// InverseKitDiff computes the reverse diff that undoes an applied diff.
// [👤semio📚go💻semio🔖kitoperations🛠️inversekitdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/InverseKitDiff)
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
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inversetypesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseTypesDiff)
// inverseTypesDiff holds the data fields for a inverseTypesDiff record.
// inverseTypesDiff MUST perform the inverseTypesDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversetypediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseTypeDiff)
// inverseTypeDiff holds the data fields for a inverseTypeDiff record.
// inverseTypeDiff MUST perform the inverseTypeDiff operation.
func inverseTypeDiff(original Type, appliedDiff TypeDiff) TypeDiff {
	inverse := TypeDiff{}
	inverse.setFields = make(map[string]bool)
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Parent != nil {
		inverse.Parent = original.Parent
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
	if appliedDiff.Models != nil {
		modelsDiff := inverseModelsDiff(original.Models, *appliedDiff.Models)
		inverse.Models = &modelsDiff
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversedesignsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseDesignsDiff)
// inverseDesignsDiff holds the data fields for a inverseDesignsDiff record.
// inverseDesignsDiff MUST perform the inverseDesignsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversedesigndiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseDesignDiff)
// inverseDesignDiff holds the data fields for a inverseDesignDiff record.
// inverseDesignDiff MUST perform the inverseDesignDiff operation.
func inverseDesignDiff(original Design, appliedDiff DesignDiff) DesignDiff {
	inverse := DesignDiff{}
	if appliedDiff.Name != nil {
		inverse.Name = &original.Name
	}
	if appliedDiff.Parent != nil {
		inverse.Parent = original.Parent
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversetagsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseTagsDiff)
// inverseTagsDiff holds the data fields for a inverseTagsDiff record.
// inverseTagsDiff MUST perform the inverseTagsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversetagdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseTagDiff)
// inverseTagDiff holds the data fields for a inverseTagDiff record.
// inverseTagDiff MUST perform the inverseTagDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseconceptsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseConceptsDiff)
// inverseConceptsDiff holds the data fields for a inverseConceptsDiff record.
// inverseConceptsDiff MUST perform the inverseConceptsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseconceptdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseConceptDiff)
// inverseConceptDiff holds the data fields for a inverseConceptDiff record.
// inverseConceptDiff MUST perform the inverseConceptDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseportsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inversePortsDiff)
// inversePortsDiff holds the data fields for a inversePortsDiff record.
// inversePortsDiff MUST perform the inversePortsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseportdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inversePortDiff)
// inversePortDiff holds the data fields for a inversePortDiff record.
// inversePortDiff MUST perform the inversePortDiff operation.
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
	if appliedDiff.CompatiblePorts != nil {
		inverse.CompatiblePorts = original.CompatiblePorts
	}
	if appliedDiff.Attributes != nil {
		attrsDiff := inverseAttributesDiff(original.Attributes, *appliedDiff.Attributes)
		inverse.Attributes = &attrsDiff
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inversefilesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseFilesDiff)
// inverseFilesDiff holds the data fields for a inverseFilesDiff record.
// inverseFilesDiff MUST perform the inverseFilesDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversefilediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseFileDiff)
// inverseFileDiff holds the data fields for a inverseFileDiff record.
// inverseFileDiff MUST perform the inverseFileDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversefoldersdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseFoldersDiff)
// inverseFoldersDiff holds the data fields for a inverseFoldersDiff record.
// inverseFoldersDiff MUST perform the inverseFoldersDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversefolderdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseFolderDiff)
// inverseFolderDiff holds the data fields for a inverseFolderDiff record.
// inverseFolderDiff MUST perform the inverseFolderDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseauthorsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseAuthorsDiff)
// inverseAuthorsDiff holds the data fields for a inverseAuthorsDiff record.
// inverseAuthorsDiff MUST perform the inverseAuthorsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseauthordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseAuthorDiff)
// inverseAuthorDiff MUST perform the inverseAuthorDiff operation.
// inverseAuthorDiff performs the inverseAuthorDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseconnectorsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseConnectorsDiff)
// inverseConnectorsDiff holds the data fields for a inverseConnectorsDiff record.
// inverseConnectorsDiff MUST perform the inverseConnectorsDiff operation.
func inverseConnectorsDiff(original []Connector, appliedDiff ConnectorsDiff) ConnectorsDiff {
	inverse := ConnectorsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, ConnectorId{Guid: added.Guid})
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
			if c.Guid == updated.Connector.Guid {
				inverseDiff := inverseConnectorDiff(c, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Connector ConnectorId   `json:"connector"`
					Diff      ConnectorDiff `json:"diff"`
				}{Connector: ConnectorId{Guid: c.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inverseconnectordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseConnectorDiff)
// inverseConnectorDiff holds the data fields for a inverseConnectorDiff record.
// inverseConnectorDiff MUST perform the inverseConnectorDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversemodelsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseModelsDiff)
// inverseModelsDiff holds the data fields for a inverseModelsDiff record.
// inverseModelsDiff MUST perform the inverseModelsDiff operation.
func inverseModelsDiff(original []Model, appliedDiff ModelsDiff) ModelsDiff {
	inverse := ModelsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, ModelId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, m := range original {
			if m.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, m)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, m := range original {
			if m.Guid == updated.Model.Guid {
				inverseDiff := inverseModelDiff(m, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Model ModelId   `json:"model"`
					Diff  ModelDiff `json:"diff"`
				}{Model: ModelId{Guid: m.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inversemodeldiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseModelDiff)
// inverseModelDiff holds the data fields for a inverseModelDiff record.
// inverseModelDiff MUST perform the inverseModelDiff operation.
func inverseModelDiff(original Model, appliedDiff ModelDiff) ModelDiff {
	inverse := ModelDiff{}
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversepiecesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inversePiecesDiff)
// inversePiecesDiff holds the data fields for a inversePiecesDiff record.
// inversePiecesDiff MUST perform the inversePiecesDiff operation.
func inversePiecesDiff(original []Piece, appliedDiff PiecesDiff) PiecesDiff {
	inverse := PiecesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, PieceId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, p := range original {
			if p.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, p)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, p := range original {
			if p.Guid == updated.Piece.Guid {
				inverseDiff := inversePieceDiff(p, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Piece PieceId   `json:"piece"`
					Diff  PieceDiff `json:"diff"`
				}{Piece: PieceId{Guid: p.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inversepiecediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inversePieceDiff)
// inversePieceDiff holds the data fields for a inversePieceDiff record.
// inversePieceDiff MUST perform the inversePieceDiff operation.
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
			inverse.Center = &CoordDiff{U: &original.Center.U, V: &original.Center.V}
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseconnectionsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseConnectionsDiff)
// inverseConnectionsDiff holds the data fields for a inverseConnectionsDiff record.
// inverseConnectionsDiff MUST perform the inverseConnectionsDiff operation.
func inverseConnectionsDiff(original []Connection, appliedDiff ConnectionsDiff) ConnectionsDiff {
	inverse := ConnectionsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, ConnectionId{Guid: added.Guid})
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
			if c.Guid == updated.Connection.Guid {
				inverseDiff := inverseConnectionDiff(c, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Connection ConnectionId   `json:"connection"`
					Diff       ConnectionDiff `json:"diff"`
				}{Connection: ConnectionId{Guid: c.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inverseconnectiondiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseConnectionDiff)
// inverseConnectionDiff holds the data fields for a inverseConnectionDiff record.
// inverseConnectionDiff MUST perform the inverseConnectionDiff operation.
func inverseConnectionDiff(original Connection, appliedDiff ConnectionDiff) ConnectionDiff {
	inverse := ConnectionDiff{}
	if appliedDiff.Connected != nil {
		inverse.Connected = inverseSideDiff(original.Connected, *appliedDiff.Connected)
	}
	if appliedDiff.Connecting != nil {
		inverse.Connecting = inverseSideDiff(original.Connecting, *appliedDiff.Connecting)
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversesidediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseSideDiff)
// inverseSideDiff holds the data fields for a inverseSideDiff record.
// inverseSideDiff MUST perform the inverseSideDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseattributediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseAttributeDiff)
// inverseAttributesDiff holds the data fields for a inverseAttributesDiff record.
// inverseAttributesDiff MUST perform the inverseAttributesDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverseattributesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseAttributesDiff)
// inverseAttributesDiff MUST perform the inverseAttributesDiff operation.
// inverseAttributesDiff performs the inverseAttributesDiff operation.
func inverseAttributesDiff(original []Attribute, appliedDiff AttributesDiff) AttributesDiff {
	inverse := AttributesDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, AttributeId{Guid: added.Guid})
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
			if a.Guid == updated.Attribute.Guid {
				inverseDiff := inverseAttributeDiff(a, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Attribute AttributeId   `json:"attribute"`
					Diff      AttributeDiff `json:"diff"`
				}{Attribute: AttributeId{Guid: a.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inversepropsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inversePropsDiff)
// inversePropsDiff holds the data fields for a inversePropsDiff record.
// inversePropsDiff MUST perform the inversePropsDiff operation.
func inversePropsDiff(original []Prop, appliedDiff PropsDiff) PropsDiff {
	inverse := PropsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, PropId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, p := range original {
			if p.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, p)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, p := range original {
			if p.Guid == updated.Prop.Guid {
				inverseDiff := inversePropDiff(p, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Prop PropId   `json:"prop"`
					Diff PropDiff `json:"diff"`
				}{Prop: PropId{Guid: p.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inversepropdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inversePropDiff)
// inversePropDiff holds the data fields for a inversePropDiff record.
// inversePropDiff MUST perform the inversePropDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inversestatsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseStatsDiff)
// inverseStatsDiff holds the data fields for a inverseStatsDiff record.
// inverseStatsDiff MUST perform the inverseStatsDiff operation.
func inverseStatsDiff(original []Stat, appliedDiff StatsDiff) StatsDiff {
	inverse := StatsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, StatId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, s := range original {
			if s.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, s)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, s := range original {
			if s.Guid == updated.Stat.Guid {
				inverseDiff := inverseStatDiff(s, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Stat StatId   `json:"stat"`
					Diff StatDiff `json:"diff"`
				}{Stat: StatId{Guid: s.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inversestatdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseStatDiff)
// inverseStatDiff holds the data fields for a inverseStatDiff record.
// inverseStatDiff MUST perform the inverseStatDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️inverselayersdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseLayersDiff)
// inverseLayersDiff holds the data fields for a inverseLayersDiff record.
// inverseLayersDiff MUST perform the inverseLayersDiff operation.
func inverseLayersDiff(original []Layer, appliedDiff LayersDiff) LayersDiff {
	inverse := LayersDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, LayerId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, l := range original {
			if l.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, l)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, l := range original {
			if l.Guid == updated.Layer.Guid {
				inverseDiff := inverseLayerDiff(l, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Layer LayerId   `json:"layer"`
					Diff  LayerDiff `json:"diff"`
				}{Layer: LayerId{Guid: l.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// inverseLayerDiff MUST perform the inverseLayerDiff operation.
// [👤semio📚go💻semio🔖kitoperations🛠️inverselayerdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseLayerDiff)
// inverseLayerDiff performs the inverseLayerDiff operation.
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

// inverseGroupsDiff holds the data fields for a inverseGroupsDiff record.
// inverseGroupsDiff MUST perform the inverseGroupsDiff operation.
// [👤semio📚go💻semio🔖kitoperations🛠️inversegroupsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseGroupsDiff)
func inverseGroupsDiff(original []Group, appliedDiff GroupsDiff) GroupsDiff {
	inverse := GroupsDiff{}
	for _, added := range appliedDiff.Added {
		inverse.Removed = append(inverse.Removed, GroupId{Guid: added.Guid})
	}
	for _, removed := range appliedDiff.Removed {
		for _, g := range original {
			if g.Guid == removed.Guid {
				inverse.Added = append(inverse.Added, g)
				break
			}
		}
	}
	for _, updated := range appliedDiff.Updated {
		for _, g := range original {
			if g.Guid == updated.Group.Guid {
				inverseDiff := inverseGroupDiff(g, updated.Diff)
				inverse.Updated = append(inverse.Updated, struct {
					Group GroupId   `json:"group"`
					Diff  GroupDiff `json:"diff"`
				}{Group: GroupId{Guid: g.Guid}, Diff: inverseDiff})
				break
			}
		}
	}
	return inverse
}

// [👤semio📚go💻semio🔖kitoperations🛠️inversegroupdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/inverseGroupDiff)
// inverseGroupDiff holds the data fields for a inverseGroupDiff record.
// inverseGroupDiff MUST perform the inverseGroupDiff operation.
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

// normalizeStr MUST perform the normalizeStr operation.
// normalizeStr holds the data fields for a normalizeStr record.
// [👤semio📚go💻semio🔖kitoperations🛠️normalizestr](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/normalizeStr)
func normalizeStr(s *string) string {
	if s == nil {
		return ""
	}
	return *s
}

// [👤semio📚go💻semio🔖kitoperations🛠️normalizeint64](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/normalizeInt64)
// normalizeInt64 holds the data fields for a normalizeInt64 record.
// normalizeInt64 MUST perform the normalizeInt64 operation.
func normalizeInt64(p *int64) int64 {
	if p == nil {
		return 0
	}
	return *p
}

// [👤semio📚go💻semio🔖kitoperations🛠️arefolderidsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areFolderIdsEqual)
// areFolderIdsEqual holds the data fields for a areFolderIdsEqual record.
// areFolderIdsEqual MUST perform the areFolderIdsEqual operation.
func areFolderIdsEqual(a, b *FolderId) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Guid == b.Guid
}

// [👤semio📚go💻semio🔖kitoperations🛠️getattributediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getAttributeDiff)
// getAttributesDiff holds the data fields for a getAttributesDiff record.
// getAttributesDiff MUST perform the getAttributesDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️isattributediffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isAttributeDiffEmpty)
// isAttributeDiffEmpty MUST perform the isAttributeDiffEmpty operation.
// isAttributeDiffEmpty performs the isAttributeDiffEmpty operation.
func isAttributeDiffEmpty(diff AttributeDiff) bool {
	return diff.Key == nil && diff.Value == nil && diff.Definition == nil
}

// getAttributesDiff holds the data fields for a getAttributesDiff record.
// getAttributesDiff MUST perform the getAttributesDiff operation.
// [👤semio📚go💻semio🔖kitoperations🛠️getattributesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getAttributesDiff)
func getAttributesDiff(before, after []Attribute) AttributesDiff {
	diff := AttributesDiff{}
	beforeMap := make(map[string]Attribute)
	for _, a := range before {
		beforeMap[a.Guid] = a
	}
	afterMap := make(map[string]Attribute)
	for _, a := range after {
		afterMap[a.Guid] = a
	}
	for _, a := range before {
		if _, ok := afterMap[a.Guid]; !ok {
			diff.Removed = append(diff.Removed, AttributeId{Guid: a.Guid})
		}
	}
	for _, a := range after {
		if _, ok := beforeMap[a.Guid]; !ok {
			diff.Added = append(diff.Added, a)
		} else {
			attrDiff := getAttributeDiff(beforeMap[a.Guid], a)
			if !isAttributeDiffEmpty(attrDiff) {
				diff.Updated = append(diff.Updated, struct {
					Attribute AttributeId   `json:"attribute"`
					Diff      AttributeDiff `json:"diff"`
				}{Attribute: AttributeId{Guid: a.Guid}, Diff: attrDiff})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️isattributesdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isAttributesDiffEmpty)
// isAttributesDiffEmpty holds the data fields for a isAttributesDiffEmpty record.
// isAttributesDiffEmpty MUST perform the isAttributesDiffEmpty operation.
func isAttributesDiffEmpty(diff AttributesDiff) bool {
	return len(diff.Added) == 0 && len(diff.Removed) == 0 && len(diff.Updated) == 0
}

// [👤semio📚go💻semio🔖kitoperations🛠️getpropsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getPropsDiff)
// getPropsDiff holds the data fields for a getPropsDiff record.
// getPropsDiff MUST perform the getPropsDiff operation.
func getPropsDiff(before, after []Prop) PropsDiff {
	diff := PropsDiff{}
	beforeMap := make(map[string]Prop)
	for _, p := range before {
		beforeMap[p.Guid] = p
	}
	afterMap := make(map[string]Prop)
	for _, p := range after {
		afterMap[p.Guid] = p
	}
	for _, p := range before {
		if _, ok := afterMap[p.Guid]; !ok {
			diff.Removed = append(diff.Removed, PropId{Guid: p.Guid})
		}
	}
	for _, p := range after {
		if _, ok := beforeMap[p.Guid]; !ok {
			diff.Added = append(diff.Added, p)
		} else {
			propDiff := getPropDiff(beforeMap[p.Guid], p)
			if !isPropDiffEmpty(propDiff) {
				diff.Updated = append(diff.Updated, struct {
					Prop PropId   `json:"prop"`
					Diff PropDiff `json:"diff"`
				}{Prop: PropId{Guid: p.Guid}, Diff: propDiff})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getpropdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getPropDiff)
// getPropDiff holds the data fields for a getPropDiff record.
// getPropDiff MUST perform the getPropDiff operation.
func getPropDiff(before, after Prop) PropDiff {
	diff := PropDiff{}
	if before.Quality.Guid != after.Quality.Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️ispropdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isPropDiffEmpty)
// isPropDiffEmpty holds the data fields for a isPropDiffEmpty record.
// isPropDiffEmpty MUST perform the isPropDiffEmpty operation.
func isPropDiffEmpty(diff PropDiff) bool {
	return diff.Quality == nil && diff.Value == nil && diff.Unit == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getstatsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getStatsDiff)
// getStatsDiff holds the data fields for a getStatsDiff record.
// getStatsDiff MUST perform the getStatsDiff operation.
func getStatsDiff(before, after []Stat) StatsDiff {
	diff := StatsDiff{}
	beforeMap := make(map[string]Stat)
	for _, s := range before {
		beforeMap[s.Guid] = s
	}
	afterMap := make(map[string]Stat)
	for _, s := range after {
		afterMap[s.Guid] = s
	}
	for _, s := range before {
		if _, ok := afterMap[s.Guid]; !ok {
			diff.Removed = append(diff.Removed, StatId{Guid: s.Guid})
		}
	}
	for _, s := range after {
		if _, ok := beforeMap[s.Guid]; !ok {
			diff.Added = append(diff.Added, s)
		} else {
			statDiff := getStatDiff(beforeMap[s.Guid], s)
			if !isStatDiffEmpty(statDiff) {
				diff.Updated = append(diff.Updated, struct {
					Stat StatId   `json:"stat"`
					Diff StatDiff `json:"diff"`
				}{Stat: StatId{Guid: s.Guid}, Diff: statDiff})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getstatdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getStatDiff)
// getStatDiff holds the data fields for a getStatDiff record.
// getStatDiff MUST perform the getStatDiff operation.
func getStatDiff(before, after Stat) StatDiff {
	diff := StatDiff{}
	if before.Quality.Guid != after.Quality.Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️isstatdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isStatDiffEmpty)
// isStatDiffEmpty holds the data fields for a isStatDiffEmpty record.
// isStatDiffEmpty MUST perform the isStatDiffEmpty operation.
func isStatDiffEmpty(diff StatDiff) bool {
	return diff.Quality == nil && diff.Min == nil && diff.Max == nil && diff.Unit == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getlayersdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getLayersDiff)
// getLayersDiff holds the data fields for a getLayersDiff record.
// getLayersDiff MUST perform the getLayersDiff operation.
func getLayersDiff(before, after []Layer) LayersDiff {
	diff := LayersDiff{}
	beforeMap := make(map[string]Layer)
	for _, l := range before {
		beforeMap[l.Guid] = l
	}
	afterMap := make(map[string]Layer)
	for _, l := range after {
		afterMap[l.Guid] = l
	}
	for _, l := range before {
		if _, ok := afterMap[l.Guid]; !ok {
			diff.Removed = append(diff.Removed, LayerId{Guid: l.Guid})
		}
	}
	for _, l := range after {
		if _, ok := beforeMap[l.Guid]; !ok {
			diff.Added = append(diff.Added, l)
		} else {
			layerDiff := getLayerDiff(beforeMap[l.Guid], l)
			if !isLayerDiffEmpty(layerDiff) {
				diff.Updated = append(diff.Updated, struct {
					Layer LayerId   `json:"layer"`
					Diff  LayerDiff `json:"diff"`
				}{Layer: LayerId{Guid: l.Guid}, Diff: layerDiff})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getlayerdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getLayerDiff)
// getLayerDiff holds the data fields for a getLayerDiff record.
// getLayerDiff MUST perform the getLayerDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️islayerdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isLayerDiffEmpty)
// isLayerDiffEmpty holds the data fields for a isLayerDiffEmpty record.
// isLayerDiffEmpty MUST perform the isLayerDiffEmpty operation.
func isLayerDiffEmpty(diff LayerDiff) bool {
	return diff.Path == nil && diff.IsHidden == nil && diff.IsLocked == nil && diff.Color == nil && diff.Description == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getgroupsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getGroupsDiff)
// getGroupsDiff holds the data fields for a getGroupsDiff record.
// getGroupsDiff MUST perform the getGroupsDiff operation.
func getGroupsDiff(before, after []Group) GroupsDiff {
	diff := GroupsDiff{}
	beforeMap := make(map[string]Group)
	for _, g := range before {
		beforeMap[g.Guid] = g
	}
	afterMap := make(map[string]Group)
	for _, g := range after {
		afterMap[g.Guid] = g
	}
	for _, g := range before {
		if _, ok := afterMap[g.Guid]; !ok {
			diff.Removed = append(diff.Removed, GroupId{Guid: g.Guid})
		}
	}
	for _, g := range after {
		if _, ok := beforeMap[g.Guid]; !ok {
			diff.Added = append(diff.Added, g)
		} else {
			groupDiff := getGroupDiff(beforeMap[g.Guid], g)
			if !isGroupDiffEmpty(groupDiff) {
				diff.Updated = append(diff.Updated, struct {
					Group GroupId   `json:"group"`
					Diff  GroupDiff `json:"diff"`
				}{Group: GroupId{Guid: g.Guid}, Diff: groupDiff})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getgroupdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getGroupDiff)
// getGroupDiff holds the data fields for a getGroupDiff record.
// getGroupDiff MUST perform the getGroupDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️isgroupdiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isGroupDiffEmpty)
// isGroupDiffEmpty holds the data fields for a isGroupDiffEmpty record.
// isGroupDiffEmpty MUST perform the isGroupDiffEmpty operation.
func isGroupDiffEmpty(diff GroupDiff) bool {
	return diff.Pieces == nil && diff.Name == nil && diff.Color == nil && diff.Description == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyattributediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyAttributeDiff)
// applyAttributesDiff holds the data fields for a applyAttributesDiff record.
// applyAttributesDiff MUST perform the applyAttributesDiff operation.
func applyAttributeDiff(base Attribute, diff AttributeDiff) Attribute {
	result := base
	if diff.Key != nil {
		result.Key = *diff.Key
	}
	if diff.Value != nil {
		result.Value = diff.Value
	}
	if diff.Definition != nil {
		result.Definition = diff.Definition
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyattributesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyAttributesDiff)
// applyAttributesDiff holds the data fields for a applyAttributesDiff record.
// applyAttributesDiff MUST perform the applyAttributesDiff operation.
func applyAttributesDiff(base []Attribute, diff AttributesDiff) []Attribute {
	result := make([]Attribute, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]AttributeDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Attribute.Guid] = u.Diff
	}
	for _, a := range base {
		if removedGuids[a.Guid] {
			continue
		}
		if d, ok := updatedDiffs[a.Guid]; ok {
			result = append(result, applyAttributeDiff(a, d))
		} else {
			result = append(result, a)
		}
	}
	result = append(result, diff.Added...)
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applypropsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyPropsDiff)
// applyPropsDiff holds the data fields for a applyPropsDiff record.
// applyPropsDiff MUST perform the applyPropsDiff operation.
func applyPropsDiff(base []Prop, diff PropsDiff) []Prop {
	result := make([]Prop, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]PropDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Prop.Guid] = u.Diff
	}
	for _, p := range base {
		if removedGuids[p.Guid] {
			continue
		}
		if d, ok := updatedDiffs[p.Guid]; ok {
			result = append(result, applyPropDiff(p, d))
		} else {
			result = append(result, p)
		}
	}
	result = append(result, diff.Added...)
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applypropdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyPropDiff)
// applyPropDiff holds the data fields for a applyPropDiff record.
// applyPropDiff MUST perform the applyPropDiff operation.
func applyPropDiff(base Prop, diff PropDiff) Prop {
	result := base
	if diff.Quality != nil {
		result.Quality = *diff.Quality
	}
	if diff.Value != nil {
		result.Value = *diff.Value
	}
	if diff.Unit != nil {
		result.Unit = diff.Unit
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applystatsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyStatsDiff)
// applyStatsDiff holds the data fields for a applyStatsDiff record.
// applyStatsDiff MUST perform the applyStatsDiff operation.
func applyStatsDiff(base []Stat, diff StatsDiff) []Stat {
	result := make([]Stat, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]StatDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Stat.Guid] = u.Diff
	}
	for _, s := range base {
		if removedGuids[s.Guid] {
			continue
		}
		if d, ok := updatedDiffs[s.Guid]; ok {
			result = append(result, applyStatDiff(s, d))
		} else {
			result = append(result, s)
		}
	}
	result = append(result, diff.Added...)
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applystatdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyStatDiff)
// applyStatDiff holds the data fields for a applyStatDiff record.
// applyStatDiff MUST perform the applyStatDiff operation.
func applyStatDiff(base Stat, diff StatDiff) Stat {
	result := base
	if diff.Quality != nil {
		result.Quality = *diff.Quality
	}
	if diff.Min != nil {
		result.Min = diff.Min
	}
	if diff.Max != nil {
		result.Max = diff.Max
	}
	if diff.Unit != nil {
		result.Unit = diff.Unit
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applylayersdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyLayersDiff)
// applyLayersDiff holds the data fields for a applyLayersDiff record.
// applyLayersDiff MUST perform the applyLayersDiff operation.
func applyLayersDiff(base []Layer, diff LayersDiff) []Layer {
	result := make([]Layer, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]LayerDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Layer.Guid] = u.Diff
	}
	for _, l := range base {
		if removedGuids[l.Guid] {
			continue
		}
		if d, ok := updatedDiffs[l.Guid]; ok {
			result = append(result, applyLayerDiff(l, d))
		} else {
			result = append(result, l)
		}
	}
	result = append(result, diff.Added...)
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applylayerdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyLayerDiff)
// applyLayerDiff holds the data fields for a applyLayerDiff record.
// applyLayerDiff MUST perform the applyLayerDiff operation.
func applyLayerDiff(base Layer, diff LayerDiff) Layer {
	result := base
	if diff.Path != nil {
		result.Path = *diff.Path
	}
	if diff.IsHidden != nil {
		result.IsHidden = diff.IsHidden
	}
	if diff.IsLocked != nil {
		result.IsLocked = diff.IsLocked
	}
	if diff.Color != nil {
		result.Color = diff.Color
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applygroupsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyGroupsDiff)
// applyGroupsDiff holds the data fields for a applyGroupsDiff record.
// applyGroupsDiff MUST perform the applyGroupsDiff operation.
func applyGroupsDiff(base []Group, diff GroupsDiff) []Group {
	result := make([]Group, 0)
	removedGuids := make(map[string]bool)
	for _, r := range diff.Removed {
		removedGuids[r.Guid] = true
	}
	updatedDiffs := make(map[string]GroupDiff)
	for _, u := range diff.Updated {
		updatedDiffs[u.Group.Guid] = u.Diff
	}
	for _, g := range base {
		if removedGuids[g.Guid] {
			continue
		}
		if d, ok := updatedDiffs[g.Guid]; ok {
			result = append(result, applyGroupDiff(g, d))
		} else {
			result = append(result, g)
		}
	}
	result = append(result, diff.Added...)
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applygroupdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyGroupDiff)
// applyGroupDiff holds the data fields for a applyGroupDiff record.
// applyGroupDiff MUST perform the applyGroupDiff operation.
func applyGroupDiff(base Group, diff GroupDiff) Group {
	result := base
	if diff.Pieces != nil {
		result.Pieces = diff.Pieces
	}
	if diff.Name != nil {
		result.Name = diff.Name
	}
	if diff.Color != nil {
		result.Color = diff.Color
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️getconnectorsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getConnectorsDiff)
// getConnectorsDiff holds the data fields for a getConnectorsDiff record.
// getConnectorsDiff MUST perform the getConnectorsDiff operation.
func getConnectorsDiff(before, after []Connector) ConnectorsDiff {
	diff := ConnectorsDiff{}
	beforeMap := make(map[string]Connector)
	for _, c := range before {
		beforeMap[c.Guid] = c
	}
	afterMap := make(map[string]Connector)
	for _, c := range after {
		afterMap[c.Guid] = c
	}
	for _, c := range before {
		if _, ok := afterMap[c.Guid]; !ok {
			diff.Removed = append(diff.Removed, ConnectorId{Guid: c.Guid})
		}
	}
	for _, c := range after {
		if _, ok := beforeMap[c.Guid]; !ok {
			diff.Added = append(diff.Added, c)
		} else {
			connDiff := getConnectorDiff(beforeMap[c.Guid], c)
			if !isConnectorDiffEmpty(connDiff) {
				diff.Updated = append(diff.Updated, struct {
					Connector ConnectorId   `json:"connector"`
					Diff      ConnectorDiff `json:"diff"`
				}{Connector: ConnectorId{Guid: c.Guid}, Diff: connDiff})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getconnectordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getConnectorDiff)
// getConnectorDiff holds the data fields for a getConnectorDiff record.
// getConnectorDiff MUST perform the getConnectorDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️isconnectordiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isConnectorDiffEmpty)
// isConnectorDiffEmpty holds the data fields for a isConnectorDiffEmpty record.
// isConnectorDiffEmpty MUST perform the isConnectorDiffEmpty operation.
func isConnectorDiffEmpty(diff ConnectorDiff) bool {
	return diff.Name == nil && diff.Description == nil && diff.Port == nil && diff.Mandatory == nil && diff.T == nil && diff.Point == nil && diff.Direction == nil && diff.Props == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getmodeldiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getModelDiff)
// getModelsDiff holds the data fields for a getModelsDiff record.
// getModelsDiff MUST perform the getModelsDiff operation.
func getModelDiff(before, after Model) ModelDiff {
	diff := ModelDiff{}
	if normalizeStr(before.Name) != normalizeStr(after.Name) {
		diff.Name = after.Name
	}
	if before.File.Guid != after.File.Guid {
		diff.File = &after.File
	}
	tagsEqual := len(before.Tags) == len(after.Tags)
	if tagsEqual {
		for i, t := range before.Tags {
			if t.Guid != after.Tags[i].Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️getmodelsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getModelsDiff)
// getModelsDiff holds the data fields for a getModelsDiff record.
// getModelsDiff MUST perform the getModelsDiff operation.
func getModelsDiff(before, after []Model) ModelsDiff {
	diff := ModelsDiff{}
	beforeMap := make(map[string]Model)
	for _, m := range before {
		beforeMap[m.Guid] = m
	}
	afterMap := make(map[string]Model)
	for _, m := range after {
		afterMap[m.Guid] = m
	}
	for _, m := range before {
		if _, ok := afterMap[m.Guid]; !ok {
			diff.Removed = append(diff.Removed, ModelId{Guid: m.Guid})
		}
	}
	for _, m := range after {
		if bm, ok := beforeMap[m.Guid]; !ok {
			diff.Added = append(diff.Added, m)
		} else {
			modelDiff := getModelDiff(bm, m)
			if modelDiff.Name != nil || modelDiff.File != nil || modelDiff.Tags != nil || modelDiff.Description != nil || modelDiff.Attributes != nil {
				diff.Updated = append(diff.Updated, struct {
					Model ModelId   `json:"model"`
					Diff  ModelDiff `json:"diff"`
				}{
					Model: ModelId{Guid: m.Guid},
					Diff:  modelDiff,
				})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getpiecesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getPiecesDiff)
// getPiecesDiff holds the data fields for a getPiecesDiff record.
// getPiecesDiff MUST perform the getPiecesDiff operation.
func getPiecesDiff(before, after []Piece) PiecesDiff {
	diff := PiecesDiff{}
	beforeMap := make(map[string]Piece)
	for _, p := range before {
		beforeMap[p.Guid] = p
	}
	afterMap := make(map[string]Piece)
	for _, p := range after {
		afterMap[p.Guid] = p
	}
	for _, p := range before {
		if _, ok := afterMap[p.Guid]; !ok {
			diff.Removed = append(diff.Removed, PieceId{Guid: p.Guid})
		}
	}
	for _, p := range after {
		if _, ok := beforeMap[p.Guid]; !ok {
			diff.Added = append(diff.Added, p)
		} else {
			pieceDiff := getPieceDiff(beforeMap[p.Guid], p)
			if !isPieceDiffEmpty(pieceDiff) {
				diff.Updated = append(diff.Updated, struct {
					Piece PieceId   `json:"piece"`
					Diff  PieceDiff `json:"diff"`
				}{Piece: PieceId{Guid: p.Guid}, Diff: pieceDiff})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getpiecediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getPieceDiff)
// getPieceDiff holds the data fields for a getPieceDiff record.
// getPieceDiff MUST perform the getPieceDiff operation.
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
			diff.Center = &CoordDiff{U: &after.Center.U, V: &after.Center.V}
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

// [👤semio📚go💻semio🔖kitoperations🛠️areplanesequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/arePlanesEqual)
// arePlanesEqual holds the data fields for a arePlanesEqual record.
// arePlanesEqual MUST perform the arePlanesEqual operation.
func arePlanesEqual(a, b Plane) bool {
	return a.Origin.X == b.Origin.X && a.Origin.Y == b.Origin.Y && a.Origin.Z == b.Origin.Z &&
		a.XAxis.X == b.XAxis.X && a.XAxis.Y == b.XAxis.Y && a.XAxis.Z == b.XAxis.Z &&
		a.YAxis.X == b.YAxis.X && a.YAxis.Y == b.YAxis.Y && a.YAxis.Z == b.YAxis.Z
}

// [👤semio📚go💻semio🔖kitoperations🛠️ispiecediffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isPieceDiffEmpty)
// isPieceDiffEmpty holds the data fields for a isPieceDiffEmpty record.
// isPieceDiffEmpty MUST perform the isPieceDiffEmpty operation.
func isPieceDiffEmpty(diff PieceDiff) bool {
	return diff.Name == nil && diff.Type == nil && diff.Design == nil && diff.Plane == nil && diff.Center == nil && diff.Scale == nil && diff.MirrorPlane == nil && diff.IsHidden == nil && diff.IsLocked == nil && diff.Color == nil && diff.Description == nil && diff.Props == nil && diff.Attributes == nil
}

// [👤semio📚go💻semio🔖kitoperations🛠️getconnectionsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getConnectionsDiff)
// getConnectionsDiff holds the data fields for a getConnectionsDiff record.
// getConnectionsDiff MUST perform the getConnectionsDiff operation.
func getConnectionsDiff(before, after []Connection) ConnectionsDiff {
	diff := ConnectionsDiff{}
	beforeMap := make(map[string]Connection)
	for _, c := range before {
		beforeMap[c.Guid] = c
	}
	afterMap := make(map[string]Connection)
	for _, c := range after {
		afterMap[c.Guid] = c
	}
	for _, c := range before {
		if _, ok := afterMap[c.Guid]; !ok {
			diff.Removed = append(diff.Removed, ConnectionId{Guid: c.Guid})
		}
	}
	for _, c := range after {
		if _, ok := beforeMap[c.Guid]; !ok {
			diff.Added = append(diff.Added, c)
		} else {
			connDiff := getConnectionDiff(beforeMap[c.Guid], c)
			if !isConnectionDiffEmpty(connDiff) {
				diff.Updated = append(diff.Updated, struct {
					Connection ConnectionId   `json:"connection"`
					Diff       ConnectionDiff `json:"diff"`
				}{Connection: ConnectionId{Guid: c.Guid}, Diff: connDiff})
			}
		}
	}
	return diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getsidediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getSideDiff)
// getSideDiff holds the data fields for a getSideDiff record.
// getSideDiff MUST perform the getSideDiff operation.
func getSideDiff(before, after Side) *SideDiff {
	diff := SideDiff{}
	changed := false
	if before.Piece.Guid != after.Piece.Guid {
		diff.Piece = &after.Piece
		changed = true
	}
	if (before.DesignPiece == nil) != (after.DesignPiece == nil) || (before.DesignPiece != nil && after.DesignPiece != nil && before.DesignPiece.Guid != after.DesignPiece.Guid) {
		diff.DesignPiece = after.DesignPiece
		changed = true
	}
	if (before.Connector == nil) != (after.Connector == nil) || (before.Connector != nil && after.Connector != nil && before.Connector.Guid != after.Connector.Guid) {
		diff.Connector = after.Connector
		changed = true
	}
	if !changed {
		return nil
	}
	return &diff
}

// [👤semio📚go💻semio🔖kitoperations🛠️getconnectiondiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/getConnectionDiff)
// getConnectionDiff holds the data fields for a getConnectionDiff record.
// getConnectionDiff MUST perform the getConnectionDiff operation.
func getConnectionDiff(before, after Connection) ConnectionDiff {
	diff := ConnectionDiff{}
	connectedDiff := getSideDiff(before.Connected, after.Connected)
	if connectedDiff != nil {
		diff.Connected = connectedDiff
	}
	connectingDiff := getSideDiff(before.Connecting, after.Connecting)
	if connectingDiff != nil {
		diff.Connecting = connectingDiff
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

// [👤semio📚go💻semio🔖kitoperations🛠️isconnectiondiffempty](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/isConnectionDiffEmpty)
// isConnectionDiffEmpty holds the data fields for a isConnectionDiffEmpty record.
// isConnectionDiffEmpty MUST perform the isConnectionDiffEmpty operation.
func isConnectionDiffEmpty(diff ConnectionDiff) bool {
	return diff.Connected == nil && diff.Connecting == nil && diff.Gap == nil && diff.Shift == nil && diff.Rise == nil && diff.Rotation == nil && diff.Turn == nil && diff.Tilt == nil && diff.U == nil && diff.V == nil && diff.Description == nil && diff.Attributes == nil
}

// areTypesEqual holds the data fields for a areTypesEqual record.
// [👤semio📚go💻semio🔖kitoperations🛠️aretypesequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areTypesEqual)
// areTypesEqual MUST perform the areTypesEqual operation.
func areTypesEqual(a, b Type) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if (a.Parent == nil) != (b.Parent == nil) {
		return false
	}
	if a.Parent != nil && a.Parent.Guid != b.Parent.Guid {
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
	if a.Location != nil && a.Location.Guid != b.Location.Guid {
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
	if !arePropsEqual(a.Props, b.Props) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

// [👤semio📚go💻semio🔖kitoperations🛠️areconnectorsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areConnectorsEqual)
// areConnectorsEqual holds the data fields for a areConnectorsEqual record.
// areConnectorsEqual MUST perform the areConnectorsEqual operation.
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
	if a.Port != nil && a.Port.Guid != b.Port.Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️aremodelsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areModelsEqual)
// areModelsEqual holds the data fields for a areModelsEqual record.
// areModelsEqual MUST perform the areModelsEqual operation.
func areModelsEqual(a, b Model) bool {
	if normalizeStr(a.Name) != normalizeStr(b.Name) {
		return false
	}
	if a.File.Guid != b.File.Guid {
		return false
	}
	if len(a.Tags) != len(b.Tags) {
		return false
	}
	for i, t := range a.Tags {
		if t.Guid != b.Tags[i].Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️aredesignsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areDesignsEqual)
// areDesignsEqual holds the data fields for a areDesignsEqual record.
// areDesignsEqual MUST perform the areDesignsEqual operation.
func areDesignsEqual(a, b Design) bool {
	if a.Name != b.Name {
		return false
	}
	if normalizeStr(a.Description) != normalizeStr(b.Description) {
		return false
	}
	if (a.Parent == nil) != (b.Parent == nil) {
		return false
	}
	if a.Parent != nil && a.Parent.Guid != b.Parent.Guid {
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
	if a.ActiveLayer != nil && a.ActiveLayer.Guid != b.ActiveLayer.Guid {
		return false
	}
	if (a.Location == nil) != (b.Location == nil) {
		return false
	}
	if a.Location != nil && a.Location.Guid != b.Location.Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️arepiecesequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/arePiecesEqual)
// arePiecesEqual holds the data fields for a arePiecesEqual record.
// arePiecesEqual MUST perform the arePiecesEqual operation.
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
	if (a.Design == nil) != (b.Design == nil) {
		return false
	}
	if a.Design != nil && a.Design.Guid != b.Design.Guid {
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
	if !areCoordsEqual(a.Center, b.Center) {
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

// [👤semio📚go💻semio🔖kitoperations🛠️areconnectionsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areConnectionsEqual)
// areConnectionsEqual holds the data fields for a areConnectionsEqual record.
// areConnectionsEqual MUST perform the areConnectionsEqual operation.
func areConnectionsEqual(a, b Connection) bool {
	if a.Connected.Piece.Guid != b.Connected.Piece.Guid {
		return false
	}
	if a.Connecting.Piece.Guid != b.Connecting.Piece.Guid {
		return false
	}
	if !areSidesEqual(a.Connected, b.Connected) {
		return false
	}
	if !areSidesEqual(a.Connecting, b.Connecting) {
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

// [👤semio📚go💻semio🔖kitoperations🛠️aretagsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areTagsEqual)
// areTagsEqual holds the data fields for a areTagsEqual record.
// areTagsEqual MUST perform the areTagsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️areconceptsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areConceptsEqual)
// areConceptsEqual holds the data fields for a areConceptsEqual record.
// areConceptsEqual MUST perform the areConceptsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️areportsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/arePortsEqual)
// arePortsEqual holds the data fields for a arePortsEqual record.
// arePortsEqual MUST perform the arePortsEqual operation.
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
	if !arePortIdSlicesEqual(a.CompatiblePorts, b.CompatiblePorts) {
		return false
	}
	if !areAttributesEqual(a.Attributes, b.Attributes) {
		return false
	}
	return true
}

// [👤semio📚go💻semio🔖kitoperations🛠️arefilesequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areFilesEqual)
// areFilesEqual holds the data fields for a areFilesEqual record.
// areFilesEqual MUST perform the areFilesEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️arefoldersequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areFoldersEqual)
// areFoldersEqual holds the data fields for a areFoldersEqual record.
// areFoldersEqual MUST perform the areFoldersEqual operation.
func areFoldersEqual(a, b Folder) bool {
	if a.Name != b.Name {
		return false
	}
	if (a.Parent == nil) != (b.Parent == nil) {
		return false
	}
	if a.Parent != nil && a.Parent.Guid != b.Parent.Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️areauthorsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areAuthorsEqual)
// areAuthorsEqual holds the data fields for a areAuthorsEqual record.
// areAuthorsEqual MUST perform the areAuthorsEqual operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️arecoordsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areCoordsEqual)
// areCoordsEqual holds the data fields for a areCoordsEqual record.
// areCoordsEqual MUST perform the areCoordsEqual operation.
func areCoordsEqual(a, b *Coord) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return floatEqual(a.U, b.U, 1e-9) && floatEqual(a.V, b.V, 1e-9)
}

// [👤semio📚go💻semio🔖kitoperations🛠️aresidesequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areSidesEqual)
// areSidesEqual holds the data fields for a areSidesEqual record.
// areSidesEqual MUST perform the areSidesEqual operation.
func areSidesEqual(a, b Side) bool {
	if a.Piece.Guid != b.Piece.Guid {
		return false
	}
	if (a.DesignPiece == nil) != (b.DesignPiece == nil) {
		return false
	}
	if a.DesignPiece != nil && a.DesignPiece.Guid != b.DesignPiece.Guid {
		return false
	}
	if (a.Connector == nil) != (b.Connector == nil) {
		return false
	}
	if a.Connector != nil && a.Connector.Guid != b.Connector.Guid {
		return false
	}
	return true
}

// [👤semio📚go💻semio🔖kitoperations🛠️arestatsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areStatsEqual)
// areStatsEqual holds the data fields for a areStatsEqual record.
// areStatsEqual MUST perform the areStatsEqual operation.
func areStatsEqual(a, b []Stat) bool {
	if len(a) != len(b) {
		return false
	}
	for _, sa := range a {
		found := false
		for _, sb := range b {
			if sa.Guid == sb.Guid {
				if sa.Quality.Guid != sb.Quality.Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️arelayersequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areLayersEqual)
// areLayersEqual holds the data fields for a areLayersEqual record.
// areLayersEqual MUST perform the areLayersEqual operation.
func areLayersEqual(a, b []Layer) bool {
	if len(a) != len(b) {
		return false
	}
	for _, la := range a {
		found := false
		for _, lb := range b {
			if la.Guid == lb.Guid {
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

// [👤semio📚go💻semio🔖kitoperations🛠️aregroupsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/areGroupsEqual)
// areGroupsEqual holds the data fields for a areGroupsEqual record.
// areGroupsEqual MUST perform the areGroupsEqual operation.
func areGroupsEqual(a, b []Group) bool {
	if len(a) != len(b) {
		return false
	}
	for _, ga := range a {
		found := false
		for _, gb := range b {
			if ga.Guid == gb.Guid {
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

// ApplyKitDiff MUST apply all additions, removals and updates from the diff.
// ApplyKitDiff applies a diff to a base kit producing the updated kit.
// [👤semio📚go💻semio🔖kitoperations🛠️applykitdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/ApplyKitDiff)
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
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applytypesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyTypesDiff)
// applyTypesDiff holds the data fields for a applyTypesDiff record.
// applyTypesDiff MUST perform the applyTypesDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applytypediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyTypeDiff)
// applyTypeDiff holds the data fields for a applyTypeDiff record.
// applyTypeDiff MUST perform the applyTypeDiff operation.
func applyTypeDiff(base Type, diff TypeDiff) Type {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Parent != nil {
		result.Parent = diff.Parent
	}
	if diff.IsAbstract != nil {
		result.IsAbstract = diff.IsAbstract
	}
	if diff.HasField("virtual") {
		result.Virtual = diff.Virtual
	}
	if diff.HasField("unit") {
		result.Unit = diff.Unit
	}
	if diff.Stock != nil {
		result.Stock = diff.Stock
	}
	if diff.Location != nil {
		result.Location = diff.Location
	}
	if diff.Folder != nil {
		result.Folder = diff.Folder
	}
	if diff.Icon != nil {
		result.Icon = diff.Icon
	}
	if diff.Image != nil {
		result.Image = diff.Image
	}
	if diff.HasField("description") {
		result.Description = diff.Description
	}
	if diff.Authors != nil {
		result.Authors = diff.Authors
	}
	if diff.Concepts != nil {
		result.Concepts = diff.Concepts
	}
	if diff.Models != nil {
		result.Models = applyModelsDiff(base.Models, *diff.Models)
	}
	if diff.Connectors != nil {
		result.Connectors = applyConnectorsDiff(base.Connectors, *diff.Connectors)
	}
	if diff.Props != nil {
		result.Props = applyPropsDiff(base.Props, *diff.Props)
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyconnectorsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyConnectorsDiff)
// applyConnectorsDiff holds the data fields for a applyConnectorsDiff record.
// applyConnectorsDiff MUST perform the applyConnectorsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applyconnectordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyConnectorDiff)
// applyConnectorDiff holds the data fields for a applyConnectorDiff record.
// applyConnectorDiff MUST perform the applyConnectorDiff operation.
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
			result.Point.X += *diff.Point.X
		}
		if diff.Point.Y != nil {
			result.Point.Y += *diff.Point.Y
		}
		if diff.Point.Z != nil {
			result.Point.Z += *diff.Point.Z
		}
	}
	if diff.Direction != nil {
		if diff.Direction.X != nil {
			result.Direction.X += *diff.Direction.X
		}
		if diff.Direction.Y != nil {
			result.Direction.Y += *diff.Direction.Y
		}
		if diff.Direction.Z != nil {
			result.Direction.Z += *diff.Direction.Z
		}
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Port != nil {
		result.Port = diff.Port
	}
	if diff.Mandatory != nil {
		result.Mandatory = diff.Mandatory
	}
	if diff.Props != nil {
		result.Props = applyPropsDiff(base.Props, *diff.Props)
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applymodelsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyModelsDiff)
// applyModelsDiff holds the data fields for a applyModelsDiff record.
// applyModelsDiff MUST perform the applyModelsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applymodeldiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyModelDiff)
// applyModelDiff holds the data fields for a applyModelDiff record.
// applyModelDiff MUST perform the applyModelDiff operation.
func applyModelDiff(base Model, diff ModelDiff) Model {
	result := base
	if diff.Name != nil {
		result.Name = diff.Name
	}
	if diff.File != nil {
		result.File = *diff.File
	}
	if diff.Tags != nil {
		result.Tags = diff.Tags
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applydesignsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyDesignsDiff)
// applyDesignsDiff holds the data fields for a applyDesignsDiff record.
// applyDesignsDiff MUST perform the applyDesignsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applydesigndiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyDesignDiff)
// applyDesignDiff holds the data fields for a applyDesignDiff record.
// applyDesignDiff MUST perform the applyDesignDiff operation.
func applyDesignDiff(base Design, diff DesignDiff) Design {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Parent != nil {
		result.Parent = diff.Parent
	}
	if diff.IsAbstract != nil {
		result.IsAbstract = diff.IsAbstract
	}
	if diff.Unit != nil {
		result.Unit = diff.Unit
	}
	if diff.Folder != nil {
		result.Folder = diff.Folder
	}
	if diff.CanScale != nil {
		result.CanScale = diff.CanScale
	}
	if diff.CanMirror != nil {
		result.CanMirror = diff.CanMirror
	}
	if diff.ActiveLayer != nil {
		result.ActiveLayer = diff.ActiveLayer
	}
	if diff.Location != nil {
		result.Location = diff.Location
	}
	if diff.Icon != nil {
		result.Icon = diff.Icon
	}
	if diff.Image != nil {
		result.Image = diff.Image
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Authors != nil {
		result.Authors = diff.Authors
	}
	if diff.Concepts != nil {
		result.Concepts = diff.Concepts
	}
	if diff.Pieces != nil {
		result.Pieces = applyPiecesDiff(base.Pieces, *diff.Pieces)
	}
	if diff.Connections != nil {
		result.Connections = applyConnectionsDiff(base.Connections, *diff.Connections)
	}
	if diff.Stats != nil {
		result.Stats = applyStatsDiff(base.Stats, *diff.Stats)
	}
	if diff.Props != nil {
		result.Props = applyPropsDiff(base.Props, *diff.Props)
	}
	if diff.Layers != nil {
		result.Layers = applyLayersDiff(base.Layers, *diff.Layers)
	}
	if diff.Groups != nil {
		result.Groups = applyGroupsDiff(base.Groups, *diff.Groups)
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applypiecesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyPiecesDiff)
// applyPiecesDiff holds the data fields for a applyPiecesDiff record.
// applyPiecesDiff MUST perform the applyPiecesDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applypiecediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyPieceDiff)
// applyPieceDiff holds the data fields for a applyPieceDiff record.
// applyPieceDiff MUST perform the applyPieceDiff operation.
func applyPieceDiff(base Piece, diff PieceDiff) Piece {
	result := base
	if diff.Name != nil {
		result.Name = diff.Name
	}
	if diff.Type != nil {
		result.Type = diff.Type
	}
	if diff.Design != nil {
		result.Design = diff.Design
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
	if diff.MirrorPlane != nil {
		if result.MirrorPlane == nil {
			result.MirrorPlane = &Plane{}
		}
		if diff.MirrorPlane.Origin != nil {
			if diff.MirrorPlane.Origin.X != nil {
				result.MirrorPlane.Origin.X = *diff.MirrorPlane.Origin.X
			}
			if diff.MirrorPlane.Origin.Y != nil {
				result.MirrorPlane.Origin.Y = *diff.MirrorPlane.Origin.Y
			}
			if diff.MirrorPlane.Origin.Z != nil {
				result.MirrorPlane.Origin.Z = *diff.MirrorPlane.Origin.Z
			}
		}
		if diff.MirrorPlane.XAxis != nil {
			if diff.MirrorPlane.XAxis.X != nil {
				result.MirrorPlane.XAxis.X = *diff.MirrorPlane.XAxis.X
			}
			if diff.MirrorPlane.XAxis.Y != nil {
				result.MirrorPlane.XAxis.Y = *diff.MirrorPlane.XAxis.Y
			}
			if diff.MirrorPlane.XAxis.Z != nil {
				result.MirrorPlane.XAxis.Z = *diff.MirrorPlane.XAxis.Z
			}
		}
		if diff.MirrorPlane.YAxis != nil {
			if diff.MirrorPlane.YAxis.X != nil {
				result.MirrorPlane.YAxis.X = *diff.MirrorPlane.YAxis.X
			}
			if diff.MirrorPlane.YAxis.Y != nil {
				result.MirrorPlane.YAxis.Y = *diff.MirrorPlane.YAxis.Y
			}
			if diff.MirrorPlane.YAxis.Z != nil {
				result.MirrorPlane.YAxis.Z = *diff.MirrorPlane.YAxis.Z
			}
		}
	}
	if diff.IsHidden != nil {
		result.IsHidden = diff.IsHidden
	}
	if diff.IsLocked != nil {
		result.IsLocked = diff.IsLocked
	}
	if diff.Color != nil {
		result.Color = diff.Color
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Props != nil {
		result.Props = applyPropsDiff(base.Props, *diff.Props)
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyconnectionsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyConnectionsDiff)
// applyConnectionsDiff holds the data fields for a applyConnectionsDiff record.
// applyConnectionsDiff MUST perform the applyConnectionsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applyconnectiondiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyConnectionDiff)
// applyConnectionDiff holds the data fields for a applyConnectionDiff record.
// applyConnectionDiff MUST perform the applyConnectionDiff operation.
func applyConnectionDiff(base Connection, diff ConnectionDiff) Connection {
	result := base
	if diff.Connected != nil {
		result.Connected = applySideDiff(base.Connected, *diff.Connected)
	}
	if diff.Connecting != nil {
		result.Connecting = applySideDiff(base.Connecting, *diff.Connecting)
	}
	if diff.Gap != nil {
		result.Gap = base.Gap + *diff.Gap
	}
	if diff.Shift != nil {
		result.Shift = base.Shift + *diff.Shift
	}
	if diff.Rise != nil {
		result.Rise = base.Rise + *diff.Rise
	}
	if diff.Rotation != nil {
		result.Rotation = base.Rotation + *diff.Rotation
	}
	if diff.Turn != nil {
		result.Turn = base.Turn + *diff.Turn
	}
	if diff.Tilt != nil {
		result.Tilt = base.Tilt + *diff.Tilt
	}
	if diff.U != nil {
		result.U = base.U + *diff.U
	}
	if diff.V != nil {
		result.V = base.V + *diff.V
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applysidediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applySideDiff)
// applySideDiff holds the data fields for a applySideDiff record.
// applySideDiff MUST perform the applySideDiff operation.
func applySideDiff(base Side, diff SideDiff) Side {
	result := base
	if diff.Piece != nil {
		result.Piece = *diff.Piece
	}
	if diff.DesignPiece != nil {
		result.DesignPiece = diff.DesignPiece
	}
	if diff.Connector != nil {
		result.Connector = diff.Connector
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applytagsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyTagsDiff)
// applyTagsDiff holds the data fields for a applyTagsDiff record.
// applyTagsDiff MUST perform the applyTagsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applytagdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyTagDiff)
// applyTagDiff holds the data fields for a applyTagDiff record.
// applyTagDiff MUST perform the applyTagDiff operation.
func applyTagDiff(base Tag, diff TagDiff) Tag {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.HasField("description") {
		result.Description = diff.Description
	}
	if diff.HasField("icon") {
		result.Icon = diff.Icon
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyconceptsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyConceptsDiff)
// applyConceptsDiff holds the data fields for a applyConceptsDiff record.
// applyConceptsDiff MUST perform the applyConceptsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applyconceptdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyConceptDiff)
// applyConceptDiff holds the data fields for a applyConceptDiff record.
// applyConceptDiff MUST perform the applyConceptDiff operation.
func applyConceptDiff(base Concept, diff ConceptDiff) Concept {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.HasField("description") {
		result.Description = diff.Description
	}
	if diff.HasField("icon") {
		result.Icon = diff.Icon
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyportsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyPortsDiff)
// applyPortsDiff holds the data fields for a applyPortsDiff record.
// applyPortsDiff MUST perform the applyPortsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applyportdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyPortDiff)
// applyPortDiff holds the data fields for a applyPortDiff record.
// applyPortDiff MUST perform the applyPortDiff operation.
func applyPortDiff(base Port, diff PortDiff) Port {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.HasField("description") {
		result.Description = diff.Description
	}
	if diff.HasField("icon") {
		result.Icon = diff.Icon
	}
	if diff.CompatiblePorts != nil {
		result.CompatiblePorts = diff.CompatiblePorts
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyfilesdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyFilesDiff)
// applyFilesDiff MUST perform the applyFilesDiff operation.
// applyFilesDiff performs the applyFilesDiff operation.
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

// applyFileDiff holds the data fields for a applyFileDiff record.
// applyFileDiff MUST perform the applyFileDiff operation.
// [👤semio📚go💻semio🔖kitoperations🛠️applyfilediff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyFileDiff)
func applyFileDiff(base File, diff FileDiff) File {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Remote != nil {
		result.Remote = diff.Remote
	}
	if diff.Folder != nil {
		result.Folder = diff.Folder
	}
	if diff.Size != nil {
		result.Size = diff.Size
	}
	if diff.Hash != nil {
		result.Hash = diff.Hash
	}
	if diff.Blob != nil {
		result.Blob = diff.Blob
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyfoldersdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyFoldersDiff)
// applyFoldersDiff holds the data fields for a applyFoldersDiff record.
// applyFoldersDiff MUST perform the applyFoldersDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applyfolderdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyFolderDiff)
// applyFolderDiff holds the data fields for a applyFolderDiff record.
// applyFolderDiff MUST perform the applyFolderDiff operation.
func applyFolderDiff(base Folder, diff FolderDiff) Folder {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Parent != nil {
		result.Parent = diff.Parent
	}
	if diff.Description != nil {
		result.Description = diff.Description
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// [👤semio📚go💻semio🔖kitoperations🛠️applyauthorsdiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyAuthorsDiff)
// applyAuthorsDiff holds the data fields for a applyAuthorsDiff record.
// applyAuthorsDiff MUST perform the applyAuthorsDiff operation.
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

// [👤semio📚go💻semio🔖kitoperations🛠️applyauthordiff](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/applyAuthorDiff)
// applyAuthorDiff holds the data fields for a applyAuthorDiff record.
// applyAuthorDiff MUST perform the applyAuthorDiff operation.
func applyAuthorDiff(base Author, diff AuthorDiff) Author {
	result := base
	if diff.Name != nil {
		result.Name = *diff.Name
	}
	if diff.Email != nil {
		result.Email = diff.Email
	}
	if diff.Attributes != nil {
		result.Attributes = applyAttributesDiff(base.Attributes, *diff.Attributes)
	}
	return result
}

// FilterDesignsWithoutParent MUST exclude all designs that have a non-nil parent.
// FilterDesignsWithoutParent returns only root-level designs with no parent.
// [👤semio📚go💻semio🔖kitoperations🛠️filterdesignswithoutparent](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Operations/d/i/FilterDesignsWithoutParent)
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

// #region 🔖Kit Change Helpers
// [👤semio📚go💻semio🔖kitchangehelpers](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers)
// Kit Change Helpers MUST provide convenience functions for single-entity kit changes.

// AddTypeToKit MUST return a change with exactly one added type.
// AddTypeToKit creates a change that adds a single type to a kit.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️addtypetokit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/AddTypeToKit)
func AddTypeToKit(kit Kit, typ Type) KitChange {
	forward := KitDiff{
		Types: &TypesDiff{
			Added: []Type{typ},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// RemoveTypeFromKit MUST return a change with exactly one removed type ID.
// RemoveTypeFromKit creates a change that removes a type by GUID.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️removetypefromkit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/RemoveTypeFromKit)
func RemoveTypeFromKit(kit Kit, typeGuid string) KitChange {
	forward := KitDiff{
		Types: &TypesDiff{
			Removed: []TypeId{{Guid: typeGuid}},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// AddDesignToKit MUST return a change with exactly one added design.
// AddDesignToKit creates a change that adds a single design to a kit.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️adddesigntokit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/AddDesignToKit)
func AddDesignToKit(kit Kit, design Design) KitChange {
	forward := KitDiff{
		Designs: &DesignsDiff{
			Added: []Design{design},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// RemoveDesignFromKit MUST return a change with exactly one removed design ID.
// RemoveDesignFromKit creates a change that removes a design by GUID.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️removedesignfromkit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/RemoveDesignFromKit)
func RemoveDesignFromKit(kit Kit, designGuid string) KitChange {
	forward := KitDiff{
		Designs: &DesignsDiff{
			Removed: []DesignId{{Guid: designGuid}},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// AddFileToKit MUST return a change with exactly one added file.
// AddFileToKit creates a change that adds a single file to a kit.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️addfiletokit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/AddFileToKit)
func AddFileToKit(kit Kit, file File) KitChange {
	forward := KitDiff{
		Files: &FilesDiff{
			Added: []File{file},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// RemoveFileFromKit MUST return a change with exactly one removed file ID.
// RemoveFileFromKit creates a change that removes a file by GUID.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️removefilefromkit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/RemoveFileFromKit)
func RemoveFileFromKit(kit Kit, fileGuid string) KitChange {
	forward := KitDiff{
		Files: &FilesDiff{
			Removed: []FileId{{Guid: fileGuid}},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// AddPortToKit MUST return a change with exactly one added port.
// AddPortToKit creates a change that adds a single port to a kit.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️addporttokit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/AddPortToKit)
func AddPortToKit(kit Kit, iface Port) KitChange {
	forward := KitDiff{
		Ports: &PortsDiff{
			Added: []Port{iface},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// RemovePortFromKit MUST return a change with exactly one removed port ID.
// RemovePortFromKit creates a change that removes a port by GUID.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️removeportfromkit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/RemovePortFromKit)
func RemovePortFromKit(kit Kit, interfaceGuid string) KitChange {
	forward := KitDiff{
		Ports: &PortsDiff{
			Removed: []PortId{{Guid: interfaceGuid}},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// AddTagToKit MUST return a change with exactly one added tag.
// AddTagToKit creates a change that adds a single tag to a kit.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️addtagtokit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/AddTagToKit)
func AddTagToKit(kit Kit, tag Tag) KitChange {
	forward := KitDiff{
		Tags: &TagsDiff{
			Added: []Tag{tag},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// RemoveTagFromKit MUST return a change with exactly one removed tag ID.
// RemoveTagFromKit creates a change that removes a tag by GUID.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️removetagfromkit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/RemoveTagFromKit)
func RemoveTagFromKit(kit Kit, tagGuid string) KitChange {
	forward := KitDiff{
		Tags: &TagsDiff{
			Removed: []TagId{{Guid: tagGuid}},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// AddConceptToKit MUST return a change with exactly one added concept.
// AddConceptToKit creates a change that adds a single concept to a kit.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️addconcepttokit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/AddConceptToKit)
func AddConceptToKit(kit Kit, concept Concept) KitChange {
	forward := KitDiff{
		Concepts: &ConceptsDiff{
			Added: []Concept{concept},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// RemoveConceptFromKit MUST return a change with exactly one removed concept ID.
// RemoveConceptFromKit creates a change that removes a concept by GUID.
// [👤semio📚go💻semio🔖kitchangehelpers🛠️removeconceptfromkit](repo://p/u/semio/b/l/go/f/semio.go/s/Kit%20Change%20Helpers/d/i/RemoveConceptFromKit)
func RemoveConceptFromKit(kit Kit, conceptGuid string) KitChange {
	forward := KitDiff{
		Concepts: &ConceptsDiff{
			Removed: []ConceptId{{Guid: conceptGuid}},
		},
	}
	after := ApplyKitDiff(kit, forward)
	backward := InverseKitDiff(kit, forward)
	return KitChange{Forward: forward, Backward: backward, Before: &kit, After: &after}
}

// #endregion 🔖Kit Change Helpers

// #region 🔖Validation
// [👤semio📚go💻semio🔖validation](repo://p/u/semio/b/l/go/f/semio.go/s/Validation)
// Validation MUST provide constraint-based validation of kit data integrity.

// SemioEntityKind enumerates the kinds of semio domain entities.
// [👤semio📚go💻semio🔖validation✂️semioentitykind](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/SemioEntityKind)
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
	EntityKindPort       SemioEntityKind = "Port"
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
// [👤semio📚go💻semio🔖validation✂️severity](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/Severity)
type Severity string

const (
	SeverityError   Severity = "error"
	SeverityWarning Severity = "warning"
)

// DomainLocation identifies the entity and field where a validation problem occurs.
// [👤semio📚go💻semio🔖validation✂️domainlocation](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/DomainLocation)
type DomainLocation struct {
	EntityKind SemioEntityKind `json:"entityKind"`
	EntityGuid string          `json:"entityGuid,omitempty"`
	Field      string          `json:"field,omitempty"`
}

// Fix represents a suggested correction for a validation problem.
// [👤semio📚go💻semio🔖validation✂️fix](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/Fix)
type Fix struct {
	Title string  `json:"title"`
	Diff  KitDiff `json:"diff"`
}

// Problem represents a single validation constraint breach.
// [👤semio📚go💻semio🔖validation✂️problem](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/Problem)
type Problem struct {
	ConstraintId string         `json:"constraintId"`
	Severity     Severity       `json:"severity,omitempty"`
	Message      string         `json:"message"`
	Location     DomainLocation `json:"entityKind,omitempty"`
	RelatedGuids []string       `json:"relatedGuids,omitempty"`
	Fixes        []Fix          `json:"fixes"`
}

// ValidationResult contains all problems found during kit validation.
// [👤semio📚go💻semio🔖validation✂️validationresult](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/ValidationResult)
type ValidationResult struct {
	Problems []Problem `json:"problems"`
}

// ValidationContext provides indexed access to kit entities for constraint evaluation.
// [👤semio📚go💻semio🔖validation✂️validationcontext](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/ValidationContext)
type ValidationContext struct {
	Kit           Kit
	TypesByGuid   map[string]*Type
	DesignsByGuid map[string]*Design
	PiecesByGuid  map[string]struct {
		DesignGuid string
		Piece      *Piece
	}
	ConnectorsByTypeGuid map[string][]Connector
	ModelsByTypeGuid     map[string][]Model
}

// Constraint is a function that evaluates a validation rule against a kit context.
// [👤semio📚go💻semio🔖validation✂️constraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/Constraint)
type Constraint func(ctx *ValidationContext) []Problem

// [👤semio📚go💻semio🔖validation🛠️buildvalidationcontext](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/buildValidationContext)
// buildValidationContext holds the data fields for a buildValidationContext record.
// buildValidationContext MUST perform the buildValidationContext operation.
func buildValidationContext(kit Kit) *ValidationContext {
	ctx := &ValidationContext{
		Kit:           kit,
		TypesByGuid:   make(map[string]*Type),
		DesignsByGuid: make(map[string]*Design),
		PiecesByGuid: make(map[string]struct {
			DesignGuid string
			Piece      *Piece
		}),
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

// [👤semio📚go💻semio🔖validation🛠️generateuniquename](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/generateUniqueName)
// generateUniqueName holds the data fields for a generateUniqueName record.
// generateUniqueName MUST perform the generateUniqueName operation.
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

// [👤semio📚go💻semio🔖validation🛠️makefix](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/makeFix)
// makeFix holds the data fields for a makeFix record.
// makeFix MUST perform the makeFix operation.
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
// [👤semio📚go💻semio🔖validation🛠️guiduniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/GuidUniquenessConstraint)
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

// [👤semio📚go💻semio🔖validation🛠️updateguideverywhere](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/updateGuidEverywhere)
// updateGuidEverywhere holds the data fields for a updateGuidEverywhere record.
// updateGuidEverywhere MUST perform the updateGuidEverywhere operation.
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
// [👤semio📚go💻semio🔖validation🛠️typenameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/TypeNameUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🛠️designnameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/DesignNameUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🛠️piecenameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/PieceNameUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🛠️qualitynameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/QualityNameUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🛠️portnameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/PortNameUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🛠️filenameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/FileNameUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🛠️foldernameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/FolderNameUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🛠️connectornameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/ConnectorNameUniquenessConstraint)
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

// ModelNameUniquenessConstraint MUST report duplicate model names within each type.
// ModelNameUniquenessConstraint checks that model names are unique within each type.
// [👤semio📚go💻semio🔖validation🛠️modelnameuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/ModelNameUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🛠️layerpathuniquenessconstraint](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/LayerPathUniquenessConstraint)
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
// [👤semio📚go💻semio🔖validation🪨defaultconstraints](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/DefaultConstraints)
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
// [👤semio📚go💻semio🔖validation🛠️validatekit](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/ValidateKit)
func ValidateKit(kit Kit) ValidationResult {
	return ValidateKitWithConstraints(kit, DefaultConstraints)
}

// ValidateKitWithConstraints MUST apply each constraint and aggregate all problems.
// ValidateKitWithConstraints validates a kit using the provided constraints.
// [👤semio📚go💻semio🔖validation🛠️validatekitwithconstraints](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/ValidateKitWithConstraints)
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
// [👤semio📚go💻semio🔖validation🛠️haserrors](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/d/i/HasErrors)
func HasErrors(result ValidationResult) bool {
	for _, p := range result.Problems {
		if p.Severity == SeverityError || p.Severity == "" {
			return true
		}
	}
	return false
}

// #region 🔖Validation Serialization
// [👤semio📚go💻semio🔖validation🔖validationserialization](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/s/Validation%20Serialization)
// Validation Serialization MUST provide serializable representations of validation results.

// ProblemSerialized is the JSON-serializable representation of a validation problem.
// [👤semio📚go💻semio🔖validation🔖validationserialization✂️problemserialized](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/s/Validation%20Serialization/d/i/ProblemSerialized)
type ProblemSerialized struct {
	ConstraintId string `json:"constraintId"`
	Severity     string `json:"severity,omitempty"`
	Message      string `json:"message"`
	EntityKind   string `json:"entityKind"`
	EntityGuid   string `json:"entityGuid"`
	Fixes        []Fix  `json:"fixes"`
}

// ValidationResultSerialized is the JSON-serializable representation of a validation result.
// [👤semio📚go💻semio🔖validation🔖validationserialization✂️validationresultserialized](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/s/Validation%20Serialization/d/i/ValidationResultSerialized)
type ValidationResultSerialized struct {
	Problems []ProblemSerialized `json:"problems"`
}

// ToValidationResult MUST default empty severity to error.
// ToValidationResult converts a validation result to its serializable form.
// [👤semio📚go💻semio🔖validation🔖validationserialization🛠️tovalidationresult](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/s/Validation%20Serialization/d/i/ToValidationResult)
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
// [👤semio📚go💻semio🔖validation🔖validationserialization🛠️arevalidationresultsequal](repo://p/u/semio/b/l/go/f/semio.go/s/Validation/s/Validation%20Serialization/d/i/AreValidationResultsEqual)
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
// [👤semio📚go💻semio🔖flattendesign](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design)
// Flatten Design MUST compute absolute piece planes from relative connections.

// [👤semio📚go💻semio🔖flattendesign🛠️planetomatrix](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/planeToMatrix)
// planeToMatrix holds the data fields for a planeToMatrix record.
// planeToMatrix MUST perform the planeToMatrix operation.
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

// [👤semio📚go💻semio🔖flattendesign🛠️matrixtoplane](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/matrixToPlane)
// matrixToPlane holds the data fields for a matrixToPlane record.
// matrixToPlane MUST perform the matrixToPlane operation.
func matrixToPlane(m *mat.Dense) Plane {
	return Plane{
		Origin: Point{X: m.At(0, 3), Y: m.At(1, 3), Z: m.At(2, 3)},
		XAxis:  Vector{X: m.At(0, 0), Y: m.At(1, 0), Z: m.At(2, 0)},
		YAxis:  Vector{X: m.At(0, 1), Y: m.At(1, 1), Z: m.At(2, 1)},
	}
}

// [👤semio📚go💻semio🔖flattendesign🛠️cross](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/cross)
// cross holds the data fields for a cross record.
// cross MUST perform the cross operation.
func cross(a, b []float64) []float64 {
	return []float64{
		a[1]*b[2] - a[2]*b[1],
		a[2]*b[0] - a[0]*b[2],
		a[0]*b[1] - a[1]*b[0],
	}
}

// [👤semio📚go💻semio🔖flattendesign🛠️normalize](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/normalize)
// normalize holds the data fields for a normalize record.
// normalize MUST perform the normalize operation.
func normalize(v []float64) {
	length := math.Sqrt(v[0]*v[0] + v[1]*v[1] + v[2]*v[2])
	if length > 0 {
		v[0] /= length
		v[1] /= length
		v[2] /= length
	}
}

// [👤semio📚go💻semio🔖flattendesign🛠️dot](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/dot)
// dot holds the data fields for a dot record.
// dot MUST perform the dot operation.
func dot(a, b []float64) float64 {
	return a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
}

// [👤semio📚go💻semio🔖flattendesign🛠️veclength](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/vecLength)
// vecLength holds the data fields for a vecLength record.
// vecLength MUST perform the vecLength operation.
func vecLength(v []float64) float64 {
	return math.Sqrt(v[0]*v[0] + v[1]*v[1] + v[2]*v[2])
}

// [👤semio📚go💻semio🔖flattendesign🛠️degtorad](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/degToRad)
// degToRad holds the data fields for a degToRad record.
// degToRad MUST perform the degToRad operation.
func degToRad(deg float64) float64 {
	return deg * math.Pi / 180.0
}

// [👤semio📚go💻semio🔖flattendesign🛠️roundfloat](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/roundFloat)
// roundFloat holds the data fields for a roundFloat record.
// roundFloat MUST perform the roundFloat operation.
func roundFloat(val float64, precision int) float64 {
	ratio := math.Pow(10, float64(precision))
	return math.Round(val*ratio) / ratio
}

// [👤semio📚go💻semio🔖flattendesign🛠️roundplane](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/roundPlane)
// roundPlane holds the data fields for a roundPlane record.
// roundPlane MUST perform the roundPlane operation.
func roundPlane(p Plane) Plane {
	const prec = 6
	return Plane{
		Origin: Point{X: roundFloat(p.Origin.X, prec), Y: roundFloat(p.Origin.Y, prec), Z: roundFloat(p.Origin.Z, prec)},
		XAxis:  Vector{X: roundFloat(p.XAxis.X, prec), Y: roundFloat(p.XAxis.Y, prec), Z: roundFloat(p.XAxis.Z, prec)},
		YAxis:  Vector{X: roundFloat(p.YAxis.X, prec), Y: roundFloat(p.YAxis.Y, prec), Z: roundFloat(p.YAxis.Z, prec)},
	}
}

// [👤semio📚go💻semio🔖flattendesign🛠️makerotationaxis](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/makeRotationAxis)
// makeRotationAxis holds the data fields for a makeRotationAxis record.
// makeRotationAxis MUST perform the makeRotationAxis operation.
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

// [👤semio📚go💻semio🔖flattendesign🛠️maketranslation](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/makeTranslation)
// makeTranslation holds the data fields for a makeTranslation record.
// makeTranslation MUST perform the makeTranslation operation.
func makeTranslation(x, y, z float64) *mat.Dense {
	return mat.NewDense(4, 4, []float64{
		1, 0, 0, x,
		0, 1, 0, y,
		0, 0, 1, z,
		0, 0, 0, 1,
	})
}

// [👤semio📚go💻semio🔖flattendesign🛠️quaternionfromaxisangle](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/quaternionFromAxisAngle)
// quaternionFromAxisAngle holds the data fields for a quaternionFromAxisAngle record.
// quaternionFromAxisAngle MUST perform the quaternionFromAxisAngle operation.
func quaternionFromAxisAngle(axis []float64, angle float64) []float64 {
	halfAngle := angle / 2
	s := math.Sin(halfAngle)
	return []float64{axis[0] * s, axis[1] * s, axis[2] * s, math.Cos(halfAngle)}
}

// [👤semio📚go💻semio🔖flattendesign🛠️quaternionfromunitvectors](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/quaternionFromUnitVectors)
// quaternionFromUnitVectors holds the data fields for a quaternionFromUnitVectors record.
// quaternionFromUnitVectors MUST perform the quaternionFromUnitVectors operation.
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

// [👤semio📚go💻semio🔖flattendesign🛠️quaterniontomatrix](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/quaternionToMatrix)
// quaternionToMatrix holds the data fields for a quaternionToMatrix record.
// quaternionToMatrix MUST perform the quaternionToMatrix operation.
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

// [👤semio📚go💻semio🔖flattendesign🛠️multiplymatrices](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/multiplyMatrices)
// multiplyMatrices holds the data fields for a multiplyMatrices record.
// multiplyMatrices MUST perform the multiplyMatrices operation.
func multiplyMatrices(a, b *mat.Dense) *mat.Dense {
	result := mat.NewDense(4, 4, nil)
	result.Mul(a, b)
	return result
}

// [👤semio📚go💻semio🔖flattendesign🛠️applymatrix4tovec3](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/applyMatrix4ToVec3)
// applyMatrix4ToVec3 holds the data fields for a applyMatrix4ToVec3 record.
// applyMatrix4ToVec3 MUST perform the applyMatrix4ToVec3 operation.
func applyMatrix4ToVec3(m *mat.Dense, v []float64) []float64 {
	return []float64{
		m.At(0, 0)*v[0] + m.At(0, 1)*v[1] + m.At(0, 2)*v[2],
		m.At(1, 0)*v[0] + m.At(1, 1)*v[1] + m.At(1, 2)*v[2],
		m.At(2, 0)*v[0] + m.At(2, 1)*v[1] + m.At(2, 2)*v[2],
	}
}

// [👤semio📚go💻semio🔖flattendesign🛠️computechildplane](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/computeChildPlane)
// computeChildPlane holds the data fields for a computeChildPlane record.
// computeChildPlane MUST perform the computeChildPlane operation.
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

// [👤semio📚go💻semio🔖flattendesign✂️piecenode](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/pieceNode)
// pieceNode holds the data fields for a pieceNode record.
type pieceNode struct {
	piece *Piece
	plane *Plane
}

// [👤semio📚go💻semio🔖flattendesign🛠️getconnector](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/getConnector)
// getConnector holds the data fields for a getConnector record.
// getConnector MUST perform the getConnector operation.
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
// [👤semio📚go💻semio🔖flattendesign🛠️flattendesign](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/FlattenDesign)
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

// [👤semio📚go💻semio🔖flattendesign🛠️planesequalapprox](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/planesEqualApprox)
// planesEqualApprox holds the data fields for a planesEqualApprox record.
// planesEqualApprox MUST perform the planesEqualApprox operation.
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
// [👤semio📚go💻semio🔖flattendesign🛠️applydesigndiff](repo://p/u/semio/b/l/go/f/semio.go/s/Flatten%20Design/d/i/ApplyDesignDiff)
func ApplyDesignDiff(base Design, diff DesignDiff) Design {
	return applyDesignDiff(base, diff)
}

// DragPiecesInDesign computes a DesignDiff that offsets selected piece centers and adjusts orphan connections.
// DragPiecesInDesign MUST return piece center offsets for root movers and u/v offsets for orphan connections.
// [👤semio📚go💻semiogo🔖flattendesign🛠️dragpiecesindesign](repo://definition/SEMIO/GO/SEMIO.GO/FLATTEN-DESIGN/DRAG-PIECES-IN-DESIGN)
func DragPiecesInDesign(design Design, pieces Design, offset Coord) DesignDiff {
	selectedGuids := make(map[string]bool)
	for _, p := range pieces.Pieces {
		selectedGuids[p.Guid] = true
	}
	parentMap := make(map[string]struct{ connectionGuid, parentGuid string })
	childrenMap := make(map[string][]string)
	for _, c := range design.Connections {
		parentMap[c.Connected.Piece.Guid] = struct{ connectionGuid, parentGuid string }{c.Guid, c.Connecting.Piece.Guid}
		childrenMap[c.Connecting.Piece.Guid] = append(childrenMap[c.Connecting.Piece.Guid], c.Connected.Piece.Guid)
	}
	rootMovers := make(map[string]bool)
	for _, p := range pieces.Pieces {
		for _, dp := range design.Pieces {
			if dp.Guid == p.Guid && dp.Center != nil {
				rootMovers[p.Guid] = true
				break
			}
		}
	}
	movingSet := make(map[string]bool)
	queue := make([]string, 0, len(rootMovers))
	for guid := range rootMovers {
		queue = append(queue, guid)
	}
	for len(queue) > 0 {
		guid := queue[len(queue)-1]
		queue = queue[:len(queue)-1]
		if movingSet[guid] {
			continue
		}
		movingSet[guid] = true
		for _, child := range childrenMap[guid] {
			queue = append(queue, child)
		}
	}
	var pieceUpdates []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	}
	u := offset.U
	v := offset.V
	for guid := range rootMovers {
		pieceUpdates = append(pieceUpdates, struct {
			Piece PieceId   `json:"piece"`
			Diff  PieceDiff `json:"diff"`
		}{
			Piece: PieceId{Guid: guid},
			Diff:  PieceDiff{Center: &CoordDiff{U: &u, V: &v}},
		})
	}
	var connectionUpdates []struct {
		Connection ConnectionId   `json:"connection"`
		Diff       ConnectionDiff `json:"diff"`
	}
	for guid := range selectedGuids {
		if movingSet[guid] {
			continue
		}
		parent, ok := parentMap[guid]
		if !ok {
			continue
		}
		connectionUpdates = append(connectionUpdates, struct {
			Connection ConnectionId   `json:"connection"`
			Diff       ConnectionDiff `json:"diff"`
		}{
			Connection: ConnectionId{Guid: parent.connectionGuid},
			Diff:       ConnectionDiff{U: &u, V: &v},
		})
	}
	diff := DesignDiff{}
	if len(pieceUpdates) > 0 || len(connectionUpdates) > 0 {
		if len(pieceUpdates) > 0 {
			diff.Pieces = &PiecesDiff{Updated: pieceUpdates}
		}
		if len(connectionUpdates) > 0 {
			diff.Connections = &ConnectionsDiff{Updated: connectionUpdates}
		}
	}
	return diff
}

// #endregion 🔖Flatten Design

// #region 🔖ExportDesignModel
// [👤semio📚go💻semio🔖exportdesignmodel](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model)
// ExportDesignModel MUST export a design's 3D model to GLB or glTF format.

// ExportModelFormats maps supported export format extensions.
// [👤semio📚go💻semio🔖exportdesignmodel✂️exportmodelformats](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/ExportModelFormats)
var ExportModelFormats = map[string]string{
	".glb":  ".glb",
	".gltf": ".gltf",
}

// #region 🔖ExportDesignModel/Helpers

// exportMeshData holds extracted or generated mesh geometry for a single type.
// [👤semio📚go💻semio🔖exportdesignmodel✂️exportmeshdata](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportMeshData)
type exportMeshData struct {
	positionBytes []byte
	indexBytes    []byte
	vertexCount   int
	indexCount    int
	posMin        [3]float32
	posMax        [3]float32
	indexCompKind int
}

// exportPlaneToGltfMatrix converts a Plane to a column-major 4x4 matrix for glTF.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportplanetogltfmatrix](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportPlaneToGltfMatrix)
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
	return exportApplySemioToGltfBasis([16]float64{xx, xy, xz, 0, yx, yy, yz, 0, zx, zy, zz, 0, ox, oy, oz, 1})
}

// exportDenseToGltfMatrix converts a gonum mat.Dense (row-major) to column-major glTF matrix.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportdensetogltfmatrix](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportDenseToGltfMatrix)
func exportDenseToGltfMatrix(m *mat.Dense) [16]float64 {
	return exportApplySemioToGltfBasis([16]float64{
		m.At(0, 0), m.At(1, 0), m.At(2, 0), m.At(3, 0),
		m.At(0, 1), m.At(1, 1), m.At(2, 1), m.At(3, 1),
		m.At(0, 2), m.At(1, 2), m.At(2, 2), m.At(3, 2),
		m.At(0, 3), m.At(1, 3), m.At(2, 3), m.At(3, 3),
	})
}

func exportApplySemioToGltfBasis(matrix [16]float64) [16]float64 {
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

// exportCreateBoxMesh generates a unit box placeholder mesh (1x1x1 centered at origin).
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportcreateboxmesh](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportCreateBoxMesh)
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

// exportDecodeBlobToBytes strips a data URI prefix and base64 decodes the blob content.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportdecodeblobtobytes](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportDecodeBlobToBytes)
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

// exportParseGLBMesh parses a GLB binary file and extracts the first mesh's geometry data.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportparseglbmesh](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportParseGLBMesh)
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

// exportParseGltfToMeshData extracts merged mesh geometry from a glTF JSON map and binary buffer.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportparsegltftomeshdata](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportParseGltfToMeshData)
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

// exportFindModelForKind finds the best matching model for a type given tag filters.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportfindmodelforkind](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportFindModelForKind)
func exportFindModelForKind(typ *Type, tags []string, tagsDict map[string]*Tag) *Model {
	if len(typ.Models) == 0 {
		return nil
	}
	if len(tags) == 0 {
		for i := range typ.Models {
			if len(typ.Models[i].Tags) == 0 {
				return &typ.Models[i]
			}
		}
		return &typ.Models[0]
	}
	selectedTagGuids := make(map[string]bool)
	for _, t := range tags {
		if _, ok := tagsDict[t]; ok {
			selectedTagGuids[t] = true
			continue
		}
		for _, tag := range tagsDict {
			if tag.Name == t {
				selectedTagGuids[tag.Guid] = true
			}
		}
	}
	bestModel := (*Model)(nil)
	bestScore := -1.0
	for i := range typ.Models {
		model := &typ.Models[i]
		modelTagGuids := make(map[string]bool)
		for _, tid := range model.Tags {
			modelTagGuids[tid.Guid] = true
		}
		containsAll := true
		intersection := 0
		for guid := range selectedTagGuids {
			if !modelTagGuids[guid] {
				containsAll = false
				break
			}
			intersection++
		}
		if !containsAll {
			continue
		}
		union := len(selectedTagGuids)
		for guid := range modelTagGuids {
			if !selectedTagGuids[guid] {
				union++
			}
		}
		score := 0.0
		if union > 0 {
			score = float64(intersection) / float64(union)
		}
		if score > bestScore {
			bestScore = score
			bestModel = model
		}
	}
	if bestModel != nil {
		return bestModel
	}
	return &typ.Models[0]
}

// #endregion 🔖ExportDesignModel/Helpers

// ExportDesignModel exports the 3D model of a design to GLB or glTF format.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportdesignmodel](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/ExportDesignModel)
func ExportDesignModel(kit *Kit, designGuid string, format string, tags []string, options map[string]interface{}) ([]byte, error) {
	if _, ok := ExportModelFormats[format]; !ok {
		return nil, fmt.Errorf("unsupported format: %s", format)
	}

	design := FindDesignInKit(kit, designGuid)
	if design == nil {
		return nil, fmt.Errorf("design not found: %s", designGuid)
	}
	if len(design.Pieces) == 0 {
		return nil, fmt.Errorf("design has no pieces")
	}

	typesDict := make(map[string]*Type)
	for i := range kit.Types {
		typesDict[kit.Types[i].Guid] = &kit.Types[i]
	}
	filesDict := make(map[string]*File)
	for i := range kit.Files {
		filesDict[kit.Files[i].Guid] = &kit.Files[i]
	}
	tagsDict := make(map[string]*Tag)
	for i := range kit.Tags {
		tagsDict[kit.Tags[i].Guid] = &kit.Tags[i]
	}
	pieceMap := make(map[string]*Piece)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Guid] = &design.Pieces[i]
	}

	// #region 🔖ExportDesignModel/BFS
	piecePlanes := make(map[string]*Plane)
	parentOf := make(map[string]string)
	childrenOf := make(map[string][]string)
	var rootPieceGuids []string

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
	var bfsExport func(rootGuid string)
	bfsExport = func(rootGuid string) {
		queue := []string{rootGuid}
		visited[rootGuid] = true
		rootPieceGuids = append(rootPieceGuids, rootGuid)
		rootPiece := pieceMap[rootGuid]
		if rootPiece.Plane != nil {
			piecePlanes[rootGuid] = rootPiece.Plane
		} else {
			p := Plane{
				Origin: Point{X: 0, Y: 0, Z: 0},
				XAxis:  Vector{X: 1, Y: 0, Z: 0},
				YAxis:  Vector{X: 0, Y: 1, Z: 0},
			}
			piecePlanes[rootGuid] = &p
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

				childPlane := computeChildPlane(*currentPlane, *parentConnector, *childConnector, *conn)
				piecePlanes[neighbor.neighborGuid] = &childPlane
				parentOf[neighbor.neighborGuid] = currentGuid
				childrenOf[currentGuid] = append(childrenOf[currentGuid], neighbor.neighborGuid)

				queue = append(queue, neighbor.neighborGuid)
			}
		}
	}
	for _, piece := range design.Pieces {
		if !visited[piece.Guid] {
			bfsExport(piece.Guid)
		}
	}
	// #endregion 🔖ExportDesignModel/BFS

	// #region 🔖ExportDesignModel/MeshData
	usedTypes := make(map[string]bool)
	for _, piece := range design.Pieces {
		if piece.Type != nil {
			usedTypes[piece.Type.Guid] = true
		}
	}
	typeMeshData := make(map[string]*exportMeshData)
	typeMeshNames := make(map[string]string)
	for typeGuid := range usedTypes {
		typ := typesDict[typeGuid]
		if typ == nil {
			continue
		}
		model := exportFindModelForKind(typ, tags, tagsDict)
		if model == nil {
			continue
		}
		file := filesDict[model.File.Guid]
		if file == nil || file.Blob == nil || *file.Blob == "" {
			continue
		}
		typeMeshNames[typeGuid] = file.Name
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
		typeMeshData[typeGuid] = meshData
	}
	// #endregion 🔖ExportDesignModel/MeshData

	// #region 🔖ExportDesignModel/BuildGLTF
	typeOrder := make([]string, 0, len(usedTypes))
	for typeGuid := range typeMeshData {
		typeOrder = append(typeOrder, typeGuid)
	}
	sort.Strings(typeOrder)
	typeMeshIndex := make(map[string]int)
	for i, typeGuid := range typeOrder {
		typeMeshIndex[typeGuid] = i
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

	for _, typeGuid := range typeOrder {
		md := typeMeshData[typeGuid]

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
		if meshName, ok := typeMeshNames[typeGuid]; ok {
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
		pieceNodeIndex[piece.Guid] = i
	}

	type exportNode struct {
		meshIndex int
		matrix    [16]float64
		children  []int
		name      string
	}
	nodes := make([]exportNode, len(design.Pieces))
	for i, piece := range design.Pieces {
		plane := piecePlanes[piece.Guid]
		if plane == nil {
			p := Plane{
				Origin: Point{X: 0, Y: 0, Z: 0},
				XAxis:  Vector{X: 1, Y: 0, Z: 0},
				YAxis:  Vector{X: 0, Y: 1, Z: 0},
			}
			plane = &p
		}

		var matrix [16]float64
		if parentGuid, hasParent := parentOf[piece.Guid]; hasParent {
			parentPlane := piecePlanes[parentGuid]
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
			if idx, ok := typeMeshIndex[piece.Type.Guid]; ok {
				meshIdx = idx
			}
		}

		name := piece.Guid
		if piece.Name != nil && *piece.Name != "" {
			name = *piece.Name
		}

		var childIndices []int
		for _, childGuid := range childrenOf[piece.Guid] {
			if idx, ok := pieceNodeIndex[childGuid]; ok {
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
	for _, rootGuid := range rootPieceGuids {
		if idx, ok := pieceNodeIndex[rootGuid]; ok {
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
		"asset":       map[string]interface{}{"version": "2.0", "generator": "semio"},
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
	// #endregion 🔖ExportDesignModel/BuildGLTF
}

// #endregion 🔖ExportDesignModel

// #region 🔖Geometric Insights
// [👤semio📚go💻semio🔖geometricinsights](repo://p/u/semio/b/l/go/f/semio.go/s/Geometric%20Insights)
// Key performance indicators for GLB/GLTF model geometry. Model MUST be glb/gltf.

// GeometricInsights holds computed geometric KPIs for a GLB/GLTF model in semio coordinate system (semio x=glb x, semio y=-glb x, semio z=glb y).
// [👤semio📚go💻semio🔖geometricinsights🪨geometricinsights](repo://p/u/semio/b/l/go/f/semio.go/s/Geometric%20Insights/d/i/GeometricInsights)
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
	// Semio coords: x = glb.x, y = -glb.x, z = glb.y
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

// GetGeometricInsightsForModel computes key performance indicators for the geometry of a GLB/GLTF model.
// Model MUST be path (string) or raw bytes ([]byte). Uses GLB or GLTF parsing.
// [👤semio📚go💻semio🔖geometricinsights🛠️getgeometricinsightsformodel](repo://p/u/semio/b/l/go/f/semio.go/s/Geometric%20Insights/d/i/GetGeometricInsightsForModel)
func GetGeometricInsightsForModel(model interface{}) (GeometricInsights, error) {
	var md *exportMeshData
	var err error
	switch v := model.(type) {
	case string:
		data, errRead := os.ReadFile(v)
		if errRead != nil {
			return GeometricInsights{}, fmt.Errorf("read model file: %w", errRead)
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
			return GeometricInsights{}, fmt.Errorf("model MUST be .glb or .gltf, got %s", v)
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
		return GeometricInsights{}, fmt.Errorf("model must be string path or []byte, got %T", model)
	}
	if err != nil {
		return GeometricInsights{}, err
	}
	return geometricInsightsFromMeshData(md), nil
}

// #endregion 🔖Geometric Insights

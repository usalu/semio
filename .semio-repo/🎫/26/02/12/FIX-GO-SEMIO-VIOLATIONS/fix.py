import re

filepath = "/workspaces/semio/semio/go/semio.go"

with open(filepath, "r") as f:
    lines = f.readlines()

insertions = {}


def add_insert(line_num, text):
    if line_num not in insertions:
        insertions[line_num] = []
    insertions[line_num].append(text)


# === 1. Fix orphan definitions: wrap package + import in a section ===
# Line 30 is `package semio`, line 32 starts `import (`
# Need to insert `// #region 🔖Imports` before line 30 and `// #endregion 🔖Imports` after the import block
# The import block ends at line 41 `)`, then line 42 is blank
# Actually, let me find the exact lines
for i, line in enumerate(lines):
    if line.strip() == "package semio":
        pkg_line = i + 1  # 1-based
    if line.strip().startswith("import ("):
        import_start = i + 1
    if i > 0 and lines[i - 1].strip().startswith("import ("):
        pass

# Find import close
import_close = None
for i in range(len(lines)):
    if lines[i].strip() == "package semio":
        pkg_line_idx = i
    if lines[i].strip() == "import (" or lines[i].strip().startswith("import ("):
        for j in range(i + 1, len(lines)):
            if lines[j].strip() == ")":
                import_close = j
                break
        break

# We'll insert region markers around package + import
# Before package line (pkg_line_idx), insert region start + summary
# After import close (import_close), insert region end and blank line

add_insert(
    pkg_line_idx,
    "// #region 🔖Imports\n// Imports MUST include all required packages for the semio domain library.\n\n",
)
add_insert(import_close + 1, "\n// #endregion 🔖Imports\n")

# === 2. Section summaries ===
section_summaries = {
    "Constants": "Constants MUST define shared constant values for the semio domain.",
    "Utils": "Utils MUST provide general-purpose utility functions for the semio domain.",
    "Entity IDs": "Entity IDs MUST define identifier types for all semio domain entities.",
    "Weak Entities": "Weak Entities MUST define value types that exist only as part of parent entities.",
    "Attribute": "Attribute MUST define the key-value metadata entity and its diff types.",
    "Location": "Location MUST define geographic location entities and their diff types.",
    "Author": "Author MUST define authorship entities and their diff types.",
    "File": "File MUST define file reference entities and their diff types.",
    "Folder": "Folder MUST define folder hierarchy entities and their diff types.",
    "Benchmark": "Benchmark MUST define benchmark threshold entities and their diff types.",
    "Quality": "Quality MUST define measurable quality entities and their diff types.",
    "Port": "Port MUST define connector port entities and their diff types.",
    "Prop": "Prop MUST define property value entities and their diff types.",
    "Tag": "Tag MUST define tag classification entities and their diff types.",
    "Concept": "Concept MUST define concept categorization entities and their diff types.",
    "Model": "Model MUST define 3D model reference entities and their diff types.",
    "Connector": "Connector MUST define spatial connector entities and their diff types.",
    "Type": "Type MUST define component type entities and their diff types.",
    "Layer": "Layer MUST define layer hierarchy entities and their diff types.",
    "Piece": "Piece MUST define placed piece entities and their diff types.",
    "Group": "Group MUST define piece grouping entities and their diff types.",
    "Side": "Side MUST define connection side reference entities and their diff types.",
    "Connection": "Connection MUST define spatial connection entities and their diff types.",
    "Stat": "Stat MUST define statistical measure entities and their diff types.",
    "Design": "Design MUST define assembly design entities and their diff types.",
    "Kit": "Kit MUST define the root kit container entity and its diff types.",
    "Serialization": "Serialization MUST provide JSON marshaling and unmarshaling for kit data.",
    "Helpers": "Helpers MUST provide lookup functions for finding entities within kits.",
    "Factories": "Factories MUST provide constructor functions for creating new domain entities.",
    "Kit Operations": "Kit Operations MUST provide comparison, diffing, and application of kit changes.",
    "Kit Diff Helpers": "Kit Diff Helpers MUST provide convenience functions for single-entity kit diffs.",
    "Validation": "Validation MUST provide constraint-based validation of kit data integrity.",
    "Validation Serialization": "Validation Serialization MUST provide serializable representations of validation results.",
    "Flatten Design": "Flatten Design MUST compute absolute piece planes from relative connections.",
}

for i, line in enumerate(lines):
    m = re.match(r"^// #region 🔖(.+)$", line.strip())
    if m:
        section_name = m.group(1)
        if section_name in section_summaries:
            # Check if next line is already a summary (not a blank line or region)
            next_line = lines[i + 1].strip() if i + 1 < len(lines) else ""
            if not next_line.startswith("//") or next_line.startswith("// #"):
                add_insert(i + 1, "// " + section_summaries[section_name] + "\n")

# === 3. Definition summaries and specs ===
# We need to add comments above exported definitions
# For type definitions: summary only
# For func definitions: spec + summary (spec above summary)

def_summaries = {
    # Utils functions
    "func Guid": (
        "Guid generates a new random 128-bit hex-encoded unique identifier.",
        "Guid MUST return a cryptographically random 128-bit hex string.",
    ),
    "func Normalize": (
        "Normalize converts a string to lowercase trimmed form.",
        "Normalize MUST trim whitespace and convert to lowercase.",
    ),
    "func Round": (
        "Round rounds a float64 to the specified number of decimal places.",
        "Round MUST return the value rounded to exactly the given decimal places.",
    ),
    "func DeepEqual": (
        "DeepEqual compares two values for deep equality via JSON serialization.",
        "DeepEqual MUST return true only when both values produce identical JSON.",
    ),
    # Entity ID types
    "type AttributeId": ("AttributeId identifies an attribute entity by GUID.",),
    "type LocationId": ("LocationId identifies a location entity by GUID.",),
    "type AuthorId": ("AuthorId identifies an author entity by GUID.",),
    "type FileId": ("FileId identifies a file entity by GUID.",),
    "type FolderId": ("FolderId identifies a folder entity by GUID.",),
    "type BenchmarkId": ("BenchmarkId identifies a benchmark entity by GUID.",),
    "type QualityId": ("QualityId identifies a quality entity by GUID.",),
    "type PortId": ("PortId identifies a port entity by GUID.",),
    "type PropId": ("PropId identifies a prop entity by GUID.",),
    "type TagId": ("TagId identifies a tag entity by GUID.",),
    "type ConceptId": ("ConceptId identifies a concept entity by GUID.",),
    "type ModelId": ("ModelId identifies a model entity by GUID.",),
    "type ConnectorId": ("ConnectorId identifies a connector entity by GUID.",),
    "type TypeId": ("TypeId identifies a type entity by GUID.",),
    "type LayerId": ("LayerId identifies a layer entity by GUID.",),
    "type PieceId": ("PieceId identifies a piece entity by GUID.",),
    "type GroupId": ("GroupId identifies a group entity by GUID.",),
    "type SideId": (
        "SideId identifies a connection side by piece, design piece and connector references.",
    ),
    "type ConnectionId": ("ConnectionId identifies a connection entity by GUID.",),
    "type StatId": ("StatId identifies a stat entity by GUID.",),
    "type DesignId": ("DesignId identifies a design entity by GUID.",),
    "type KitId": ("KitId identifies a kit entity by GUID.",),
    # Weak entity types
    "type Coord": ("Coord represents a 2D coordinate with U and V components.",),
    "type Vec": ("Vec represents a 2D vector with U and V components.",),
    "type Point": ("Point represents a 3D point with X, Y and Z components.",),
    "type Vector": ("Vector represents a 3D vector with X, Y and Z components.",),
    "type Plane": (
        "Plane represents a 3D plane defined by origin, X-axis and Y-axis.",
    ),
    "type Camera": (
        "Camera represents a 3D camera with position, forward and up vectors.",
    ),
    # Attribute
    "type Attribute ": (
        "Attribute represents a key-value metadata entry with optional definition.",
    ),
    "type AttributeDiff": ("AttributeDiff represents changes to an attribute entity.",),
    "type AttributesDiff": (
        "AttributesDiff represents a collection of attribute additions, removals and updates.",
    ),
    # Location
    "type Location ": (
        "Location represents a geographic location with longitude, latitude and optional altitude.",
    ),
    "type LocationDiff": ("LocationDiff represents changes to a location entity.",),
    # Author
    "type Author ": (
        "Author represents a named authorship entity with optional email.",
    ),
    "type AuthorDiff": ("AuthorDiff represents changes to an author entity.",),
    "type AuthorsDiff": (
        "AuthorsDiff represents a collection of author additions, removals and updates.",
    ),
    # File
    "type File ": (
        "File represents a file reference entity with name, remote URL and metadata.",
    ),
    "type FileDiff": ("FileDiff represents changes to a file entity.",),
    "type FilesDiff": (
        "FilesDiff represents a collection of file additions, removals and updates.",
    ),
    # Folder
    "type Folder ": (
        "Folder represents a folder hierarchy entity with name and parent reference.",
    ),
    "type FolderDiff": ("FolderDiff represents changes to a folder entity.",),
    "type FoldersDiff": (
        "FoldersDiff represents a collection of folder additions, removals and updates.",
    ),
    # Benchmark
    "type Benchmark ": (
        "Benchmark represents a named metric threshold with min and max bounds.",
    ),
    "type BenchmarkDiff": ("BenchmarkDiff represents changes to a benchmark entity.",),
    "type BenchmarksDiff": (
        "BenchmarksDiff represents a collection of benchmark additions, removals and updates.",
    ),
    # Quality
    "type QualityKind": (
        "QualityKind is a bitfield enum for quality scope classification.",
    ),
    "type Quality ": (
        "Quality represents a measurable property with formula, units and benchmarks.",
    ),
    "type QualityDiff": ("QualityDiff represents changes to a quality entity.",),
    "type QualitiesDiff": (
        "QualitiesDiff represents a collection of quality additions, removals and updates.",
    ),
    # Port
    "type Port ": (
        "Port represents a named connector port with compatible port references.",
    ),
    "type PortDiff ": ("PortDiff represents changes to a port entity.",),
    "type PortsDiff": (
        "PortsDiff represents a collection of port additions, removals and updates.",
    ),
    # Prop
    "type Prop ": ("Prop represents a quality property value with optional unit.",),
    "type PropDiff": ("PropDiff represents changes to a prop entity.",),
    "type PropsDiff": (
        "PropsDiff represents a collection of prop additions, removals and updates.",
    ),
    # Tag
    "type Tag ": (
        "Tag represents a named classification tag with optional description and icon.",
    ),
    "type TagDiff ": ("TagDiff represents changes to a tag entity.",),
    "type TagsDiff": (
        "TagsDiff represents a collection of tag additions, removals and updates.",
    ),
    # Concept
    "type Concept ": (
        "Concept represents a named categorization concept with optional description.",
    ),
    "type ConceptDiff ": ("ConceptDiff represents changes to a concept entity.",),
    "type ConceptsDiff": (
        "ConceptsDiff represents a collection of concept additions, removals and updates.",
    ),
    # Model
    "type Model ": (
        "Model represents a 3D model reference associated with a file and tags.",
    ),
    "type ModelDiff": ("ModelDiff represents changes to a model entity.",),
    "type ModelsDiff": (
        "ModelsDiff represents a collection of model additions, removals and updates.",
    ),
    # Connector
    "type Connector ": (
        "Connector represents a spatial connection point on a type with position and direction.",
    ),
    "type PointDiff": ("PointDiff represents changes to a 3D point.",),
    "type VectorDiff": ("VectorDiff represents changes to a 3D vector.",),
    "type ConnectorDiff": ("ConnectorDiff represents changes to a connector entity.",),
    "type ConnectorsDiff": (
        "ConnectorsDiff represents a collection of connector additions, removals and updates.",
    ),
    # Type
    "type Type ": (
        "Type represents a component type with models, connectors and hierarchical inheritance.",
    ),
    "type TypeDiff ": ("TypeDiff represents changes to a type entity.",),
    "type TypesDiff": (
        "TypesDiff represents a collection of type additions, removals and updates.",
    ),
    # Layer
    "type Layer ": (
        "Layer represents a named layer with visibility, lock and color properties.",
    ),
    "type LayerDiff": ("LayerDiff represents changes to a layer entity.",),
    "type LayersDiff": (
        "LayersDiff represents a collection of layer additions, removals and updates.",
    ),
    # Piece
    "type Piece ": ("Piece represents a placed component instance within a design.",),
    "type CoordDiff": ("CoordDiff represents changes to a 2D coordinate.",),
    "type PlaneDiff": ("PlaneDiff represents changes to a 3D plane.",),
    "type PieceDiff": ("PieceDiff represents changes to a piece entity.",),
    "type PiecesDiff": (
        "PiecesDiff represents a collection of piece additions, removals and updates.",
    ),
    # Group
    "type Group ": ("Group represents a named collection of pieces within a design.",),
    "type GroupDiff": ("GroupDiff represents changes to a group entity.",),
    "type GroupsDiff": (
        "GroupsDiff represents a collection of group additions, removals and updates.",
    ),
    # Side
    "type Side ": (
        "Side represents one end of a connection referencing a piece and optional connector.",
    ),
    "type SideDiff": ("SideDiff represents changes to a connection side.",),
    # Connection
    "type Connection ": (
        "Connection represents a spatial relationship between two pieces with transform parameters.",
    ),
    "type ConnectionDiff": (
        "ConnectionDiff represents changes to a connection entity.",
    ),
    "type ConnectionsDiff": (
        "ConnectionsDiff represents a collection of connection additions, removals and updates.",
    ),
    # Stat
    "type Stat ": (
        "Stat represents a statistical quality measurement with min and max bounds.",
    ),
    "type StatDiff": ("StatDiff represents changes to a stat entity.",),
    "type StatsDiff": (
        "StatsDiff represents a collection of stat additions, removals and updates.",
    ),
    # Design
    "type Design ": (
        "Design represents an assembly of pieces, connections, layers and groups.",
    ),
    "type CameraDiff": ("CameraDiff represents changes to a camera view.",),
    "type DesignDiff ": ("DesignDiff represents changes to a design entity.",),
    "type DesignsDiff": (
        "DesignsDiff represents a collection of design additions, removals and updates.",
    ),
    # Kit
    "type Kit ": ("Kit represents the root container for all domain entities.",),
    "type KitDiff ": ("KitDiff represents changes to a kit entity.",),
    "type KitsDiff": (
        "KitsDiff represents a collection of kit additions, removals and updates.",
    ),
    # Serialization functions
    "func SerializeKit(kit Kit)": (
        "SerializeKit marshals a kit to indented JSON bytes.",
        "SerializeKit MUST return valid JSON with two-space indentation.",
    ),
    "func DeserializeKit(data": (
        "DeserializeKit unmarshals JSON bytes into a kit.",
        "DeserializeKit MUST return an error if the data is not valid kit JSON.",
    ),
    "func SerializeKitDiff(diff": (
        "SerializeKitDiff marshals a kit diff to indented JSON bytes.",
        "SerializeKitDiff MUST return valid JSON with two-space indentation.",
    ),
    "func DeserializeKitDiff(data": (
        "DeserializeKitDiff unmarshals JSON bytes into a kit diff.",
        "DeserializeKitDiff MUST return an error if the data is not valid kit diff JSON.",
    ),
    # Helpers
    "func FindTypeInKit": (
        "FindTypeInKit returns a pointer to the type with the given GUID or nil.",
        "FindTypeInKit MUST return nil when no type matches the GUID.",
    ),
    "func FindDesignInKit": (
        "FindDesignInKit returns a pointer to the design with the given GUID or nil.",
        "FindDesignInKit MUST return nil when no design matches the GUID.",
    ),
    "func FindPieceInDesign": (
        "FindPieceInDesign returns a pointer to the piece with the given GUID or nil.",
        "FindPieceInDesign MUST return nil when no piece matches the GUID.",
    ),
    "func FindConnectionInDesign": (
        "FindConnectionInDesign returns a pointer to the connection with the given GUID or nil.",
        "FindConnectionInDesign MUST return nil when no connection matches the GUID.",
    ),
    "func FindConnectorInType": (
        "FindConnectorInType returns a pointer to the connector with the given GUID or nil.",
        "FindConnectorInType MUST return nil when no connector matches the GUID.",
    ),
    "func FindFileInKit": (
        "FindFileInKit returns a pointer to the file with the given GUID or nil.",
        "FindFileInKit MUST return nil when no file matches the GUID.",
    ),
    "func FindFolderInKit": (
        "FindFolderInKit returns a pointer to the folder with the given GUID or nil.",
        "FindFolderInKit MUST return nil when no folder matches the GUID.",
    ),
    "func FindQualityInKit": (
        "FindQualityInKit returns a pointer to the quality with the given GUID or nil.",
        "FindQualityInKit MUST return nil when no quality matches the GUID.",
    ),
    "func FindPortInKit": (
        "FindPortInKit returns a pointer to the port with the given GUID or nil.",
        "FindPortInKit MUST return nil when no port matches the GUID.",
    ),
    "func FindTagInKit": (
        "FindTagInKit returns a pointer to the tag with the given GUID or nil.",
        "FindTagInKit MUST return nil when no tag matches the GUID.",
    ),
    "func FindConceptInKit": (
        "FindConceptInKit returns a pointer to the concept with the given GUID or nil.",
        "FindConceptInKit MUST return nil when no concept matches the GUID.",
    ),
    "func FindAuthorInKit": (
        "FindAuthorInKit returns a pointer to the author with the given GUID or nil.",
        "FindAuthorInKit MUST return nil when no author matches the GUID.",
    ),
    # Factories
    "func NewKit": (
        "NewKit creates a new kit with the given name and a generated GUID.",
        "NewKit MUST generate a unique GUID and set version to 0.0.1.",
    ),
    "func NewType": (
        "NewType creates a new type with the given name and a generated GUID.",
        "NewType MUST generate a unique GUID for the new type.",
    ),
    "func NewDesign": (
        "NewDesign creates a new design with the given name and a generated GUID.",
        "NewDesign MUST generate a unique GUID for the new design.",
    ),
    "func NewPiece": (
        "NewPiece creates a new piece with a generated GUID.",
        "NewPiece MUST generate a unique GUID for the new piece.",
    ),
    "func NewConnection": (
        "NewConnection creates a new connection between two pieces by their GUIDs.",
        "NewConnection MUST generate a unique GUID and set both connected and connecting sides.",
    ),
    "func NewConnector": (
        "NewConnector creates a new connector with position, direction and parameter t.",
        "NewConnector MUST generate a unique GUID for the new connector.",
    ),
    "func NewFile": (
        "NewFile creates a new file with the given name and a generated GUID.",
        "NewFile MUST generate a unique GUID for the new file.",
    ),
    "func NewFolder": (
        "NewFolder creates a new folder with the given name and a generated GUID.",
        "NewFolder MUST generate a unique GUID for the new folder.",
    ),
    "func NewQuality": (
        "NewQuality creates a new quality with the given key, name and a generated GUID.",
        "NewQuality MUST generate a unique GUID for the new quality.",
    ),
    "func NewPort": (
        "NewPort creates a new port with the given name and a generated GUID.",
        "NewPort MUST generate a unique GUID for the new port.",
    ),
    "func NewTag": (
        "NewTag creates a new tag with the given name and a generated GUID.",
        "NewTag MUST generate a unique GUID for the new tag.",
    ),
    "func NewConcept": (
        "NewConcept creates a new concept with the given name and a generated GUID.",
        "NewConcept MUST generate a unique GUID for the new concept.",
    ),
    "func NewAuthor": (
        "NewAuthor creates a new author with the given name and a generated GUID.",
        "NewAuthor MUST generate a unique GUID for the new author.",
    ),
    # Kit Operations
    "func AreKitsEqual": (
        "AreKitsEqual compares two kits for structural equality.",
        "AreKitsEqual MUST compare all entities by GUID and structural fields.",
    ),
    "func AreKitDiffsEqual": (
        "AreKitDiffsEqual compares two kit diffs for structural equality.",
        "AreKitDiffsEqual MUST compare all diff fields including nested entity diffs.",
    ),
    "func GetKitDiff": (
        "GetKitDiff computes the diff between a before and after kit state.",
        "GetKitDiff MUST return a diff that when applied to before produces after.",
    ),
    "func InverseKitDiff": (
        "InverseKitDiff computes the reverse diff that undoes an applied diff.",
        "InverseKitDiff MUST return a diff that when applied restores the original state.",
    ),
    "func ApplyKitDiff": (
        "ApplyKitDiff applies a diff to a base kit producing the updated kit.",
        "ApplyKitDiff MUST apply all additions, removals and updates from the diff.",
    ),
    "func FilterDesignsWithoutParent": (
        "FilterDesignsWithoutParent returns only root-level designs with no parent.",
        "FilterDesignsWithoutParent MUST exclude all designs that have a non-nil parent.",
    ),
    # Kit Diff Helpers
    "func AddTypeToKit": (
        "AddTypeToKit creates a diff that adds a single type to a kit.",
        "AddTypeToKit MUST return a diff with exactly one added type.",
    ),
    "func RemoveTypeFromKit": (
        "RemoveTypeFromKit creates a diff that removes a type by GUID.",
        "RemoveTypeFromKit MUST return a diff with exactly one removed type ID.",
    ),
    "func AddDesignToKit": (
        "AddDesignToKit creates a diff that adds a single design to a kit.",
        "AddDesignToKit MUST return a diff with exactly one added design.",
    ),
    "func RemoveDesignFromKit": (
        "RemoveDesignFromKit creates a diff that removes a design by GUID.",
        "RemoveDesignFromKit MUST return a diff with exactly one removed design ID.",
    ),
    "func AddFileToKit": (
        "AddFileToKit creates a diff that adds a single file to a kit.",
        "AddFileToKit MUST return a diff with exactly one added file.",
    ),
    "func RemoveFileFromKit": (
        "RemoveFileFromKit creates a diff that removes a file by GUID.",
        "RemoveFileFromKit MUST return a diff with exactly one removed file ID.",
    ),
    "func AddPortToKit": (
        "AddPortToKit creates a diff that adds a single port to a kit.",
        "AddPortToKit MUST return a diff with exactly one added port.",
    ),
    "func RemovePortFromKit": (
        "RemovePortFromKit creates a diff that removes a port by GUID.",
        "RemovePortFromKit MUST return a diff with exactly one removed port ID.",
    ),
    "func AddTagToKit": (
        "AddTagToKit creates a diff that adds a single tag to a kit.",
        "AddTagToKit MUST return a diff with exactly one added tag.",
    ),
    "func RemoveTagFromKit": (
        "RemoveTagFromKit creates a diff that removes a tag by GUID.",
        "RemoveTagFromKit MUST return a diff with exactly one removed tag ID.",
    ),
    "func AddConceptToKit": (
        "AddConceptToKit creates a diff that adds a single concept to a kit.",
        "AddConceptToKit MUST return a diff with exactly one added concept.",
    ),
    "func RemoveConceptFromKit": (
        "RemoveConceptFromKit creates a diff that removes a concept by GUID.",
        "RemoveConceptFromKit MUST return a diff with exactly one removed concept ID.",
    ),
    # Validation types
    "type SemioEntityKind": (
        "SemioEntityKind enumerates the kinds of semio domain entities.",
    ),
    "type Severity": ("Severity enumerates validation problem severity levels.",),
    "type DomainLocation": (
        "DomainLocation identifies the entity and field where a validation problem occurs.",
    ),
    "type Fix": ("Fix represents a suggested correction for a validation problem.",),
    "type Problem": ("Problem represents a single validation constraint breach.",),
    "type ValidationResult": (
        "ValidationResult contains all problems found during kit validation.",
    ),
    "type ValidationContext": (
        "ValidationContext provides indexed access to kit entities for constraint evaluation.",
    ),
    "type Constraint": (
        "Constraint is a function that evaluates a validation rule against a kit context.",
    ),
    # Validation functions
    "func GuidUniquenessConstraint": (
        "GuidUniquenessConstraint checks that all entity GUIDs are unique within a kit.",
        "GuidUniquenessConstraint MUST report each duplicate GUID as a separate problem.",
    ),
    "func TypeNameUniquenessConstraint": (
        "TypeNameUniquenessConstraint checks that sibling type names are unique.",
        "TypeNameUniquenessConstraint MUST report duplicate names among types with the same parent.",
    ),
    "func DesignNameUniquenessConstraint": (
        "DesignNameUniquenessConstraint checks that sibling design names are unique.",
        "DesignNameUniquenessConstraint MUST report duplicate names among designs with the same parent.",
    ),
    "func PieceNameUniquenessConstraint": (
        "PieceNameUniquenessConstraint checks that piece names are unique within each design.",
        "PieceNameUniquenessConstraint MUST report duplicate piece names within each design.",
    ),
    "func QualityNameUniquenessConstraint": (
        "QualityNameUniquenessConstraint checks that quality names are unique within a kit.",
        "QualityNameUniquenessConstraint MUST report each duplicate quality name.",
    ),
    "func PortNameUniquenessConstraint": (
        "PortNameUniquenessConstraint checks that port names are unique within a kit.",
        "PortNameUniquenessConstraint MUST report each duplicate port name.",
    ),
    "func FileNameUniquenessConstraint": (
        "FileNameUniquenessConstraint checks that file names are unique within a kit.",
        "FileNameUniquenessConstraint MUST report each duplicate file name.",
    ),
    "func FolderNameUniquenessConstraint": (
        "FolderNameUniquenessConstraint checks that sibling folder names are unique.",
        "FolderNameUniquenessConstraint MUST report duplicate names among folders with the same parent.",
    ),
    "func ConnectorNameUniquenessConstraint": (
        "ConnectorNameUniquenessConstraint checks that connector names are unique within each type.",
        "ConnectorNameUniquenessConstraint MUST report duplicate connector names within each type.",
    ),
    "func ModelNameUniquenessConstraint": (
        "ModelNameUniquenessConstraint checks that model names are unique within each type.",
        "ModelNameUniquenessConstraint MUST report duplicate model names within each type.",
    ),
    "func LayerPathUniquenessConstraint": (
        "LayerPathUniquenessConstraint checks that layer paths are unique within each design.",
        "LayerPathUniquenessConstraint MUST report duplicate layer paths within each design.",
    ),
    # Validation vars/funcs
    "var DefaultConstraints": (
        "DefaultConstraints lists all built-in validation constraints.",
    ),
    "func ValidateKit(kit Kit)": (
        "ValidateKit validates a kit using the default set of constraints.",
        "ValidateKit MUST apply all default constraints and return all found problems.",
    ),
    "func ValidateKitWithConstraints": (
        "ValidateKitWithConstraints validates a kit using the provided constraints.",
        "ValidateKitWithConstraints MUST apply each constraint and aggregate all problems.",
    ),
    "func HasErrors": (
        "HasErrors returns true if the validation result contains any error-severity problems.",
        "HasErrors MUST return true when any problem has error severity or empty severity.",
    ),
    # Validation Serialization types and funcs
    "type ProblemSerialized": (
        "ProblemSerialized is the JSON-serializable representation of a validation problem.",
    ),
    "type ValidationResultSerialized": (
        "ValidationResultSerialized is the JSON-serializable representation of a validation result.",
    ),
    "func ToValidationResult": (
        "ToValidationResult converts a validation result to its serializable form.",
        "ToValidationResult MUST default empty severity to error.",
    ),
    "func AreValidationResultsEqual": (
        "AreValidationResultsEqual compares two serialized validation results for equality.",
        "AreValidationResultsEqual MUST compare problems regardless of their order.",
    ),
    # Flatten Design
    "func FlattenDesign": (
        "FlattenDesign computes absolute planes and centers for all pieces in a design.",
        "FlattenDesign MUST traverse the connection graph via BFS to compute piece transforms.",
    ),
    "func ApplyDesignDiff": (
        "ApplyDesignDiff applies a design diff to a base design.",
        "ApplyDesignDiff MUST apply all piece, connection and property changes from the diff.",
    ),
}

# Method-style definitions with receiver
method_summaries = {
    # PortDiff methods
    ("PortDiff", "UnmarshalJSON"): (
        "UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.",
        "UnmarshalJSON MUST populate the setFields map for all present JSON keys.",
    ),
    ("PortDiff", "HasField"): (
        "HasField returns whether a JSON field was present in the unmarshaled data.",
        "HasField MUST return false when setFields is nil.",
    ),
    # TagDiff methods
    ("TagDiff", "UnmarshalJSON"): (
        "UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.",
        "UnmarshalJSON MUST populate the setFields map for all present JSON keys.",
    ),
    ("TagDiff", "HasField"): (
        "HasField returns whether a JSON field was present in the unmarshaled data.",
        "HasField MUST return false when setFields is nil.",
    ),
    # ConceptDiff methods
    ("ConceptDiff", "UnmarshalJSON"): (
        "UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.",
        "UnmarshalJSON MUST populate the setFields map for all present JSON keys.",
    ),
    ("ConceptDiff", "HasField"): (
        "HasField returns whether a JSON field was present in the unmarshaled data.",
        "HasField MUST return false when setFields is nil.",
    ),
    # TypeDiff methods
    ("TypeDiff", "UnmarshalJSON"): (
        "UnmarshalJSON deserializes JSON while tracking which fields were explicitly set.",
        "UnmarshalJSON MUST populate the setFields map for all present JSON keys.",
    ),
    ("TypeDiff", "HasField"): (
        "HasField returns whether a JSON field was present in the unmarshaled data.",
        "HasField MUST return false when setFields is nil.",
    ),
}

# Now process each line
for i, line in enumerate(lines):
    stripped = line.strip()

    # Check for method definitions (func (d *ReceiverType) MethodName)
    method_match = re.match(r"^func \((\w+) \*(\w+)\) (\w+)\(", stripped)
    if method_match:
        receiver_type = method_match.group(2)
        method_name = method_match.group(3)
        if method_name[0].isupper():
            key = (receiver_type, method_name)
            if key in method_summaries:
                info = method_summaries[key]
                if len(info) == 2:
                    add_insert(i, "// " + info[1] + "\n")
                    add_insert(i, "// " + info[0] + "\n")
                else:
                    add_insert(i, "// " + info[0] + "\n")
        continue

    # Check for function definitions
    func_match = re.match(r"^func (\w+)", stripped)
    if func_match:
        func_name = func_match.group(1)
        if func_name[0].isupper():
            # Find matching key
            matched_key = None
            for key in def_summaries:
                if (
                    key.startswith("func ")
                    and key.split("(")[0].split()[-1] == func_name
                ):
                    # More specific match: check if the line itself starts with the key
                    key_start = key.replace("func ", "func ")
                    if stripped.startswith(key_start.replace("func ", "func ")):
                        matched_key = key
                        break
            if not matched_key:
                for key in def_summaries:
                    if key.startswith("func ") and func_name in key:
                        matched_key = key
                        break
            if matched_key:
                info = def_summaries[matched_key]
                if len(info) == 2:
                    add_insert(i, "// " + info[1] + "\n")
                    add_insert(i, "// " + info[0] + "\n")
                else:
                    add_insert(i, "// " + info[0] + "\n")
        continue

    # Check for type definitions
    type_match = re.match(r"^type (\w+)", stripped)
    if type_match:
        type_name = type_match.group(1)
        if type_name[0].isupper():
            matched_key = None
            for key in def_summaries:
                if key.startswith("type "):
                    key_type_name = key.split()[1]
                    if key_type_name == type_name:
                        matched_key = key
                        break
            if not matched_key:
                # Try partial match (e.g. "type Attribute " matches "type Attribute struct")
                for key in def_summaries:
                    if key.startswith("type ") and stripped.startswith(key.strip()):
                        matched_key = key
                        break
            if matched_key:
                info = def_summaries[matched_key]
                add_insert(i, "// " + info[0] + "\n")
        continue

    # Check for var definitions
    var_match = re.match(r"^var (\w+)", stripped)
    if var_match:
        var_name = var_match.group(1)
        if var_name[0].isupper():
            key = "var " + var_name
            if key in def_summaries:
                info = def_summaries[key]
                add_insert(i, "// " + info[0] + "\n")
        continue

# Now apply all insertions, from bottom to top to preserve line numbers
result_lines = list(lines)
for line_idx in sorted(insertions.keys(), reverse=True):
    for text in reversed(insertions[line_idx]):
        result_lines.insert(line_idx, text)

with open(filepath, "w") as f:
    f.writelines(result_lines)

print(f"Done. Applied insertions at {len(insertions)} locations.")
print(f"Total lines: {len(lines)} -> {len(result_lines)}")

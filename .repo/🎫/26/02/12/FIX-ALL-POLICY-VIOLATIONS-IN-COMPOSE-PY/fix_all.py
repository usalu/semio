#!/usr/bin/env python3
"""Script to fix all policy breachs in compose/py/compose.py."""

import re
import sys

FILE = "/workspaces/semio/compose/py/compose.py"

# Read the file
with open(FILE, "r") as f:
    lines = f.readlines()

# --- PASS 1: Fix orphan definition at line 29 (from __future__ import annotations) ---
# Move it into the Imports region by swapping "from __future__..." and "# region Imports"
# Currently:
# line 29: from __future__ import annotations
# line 30: # region Imports
# We need:
# line 29: # #region 🔖Header Preamble  -- actually, simpler: move 'from __future__' after '# region Imports'

# Find the orphan line and the region start
orphan_idx = None
region_imports_idx = None
for i, line in enumerate(lines):
    if line.strip() == "from __future__ import annotations" and orphan_idx is None:
        orphan_idx = i
    if line.strip() == "# region Imports" and region_imports_idx is None:
        region_imports_idx = i

if (
    orphan_idx is not None
    and region_imports_idx is not None
    and orphan_idx < region_imports_idx
):
    # Remove orphan line, insert it after the region line
    orphan_line = lines.pop(orphan_idx)
    # After removal, region_imports_idx shifted by -1
    region_imports_idx -= 1
    lines.insert(region_imports_idx + 1, orphan_line)
    print(f"Fixed orphan: moved 'from __future__' into Imports region")

# --- PASS 2: Add section summaries ---
# Find all # region XYZ lines and add a summary on the next line if missing

section_summaries = {
    "Imports": "# Standard library, third-party and framework imports.",
    "Type Hints": "# Custom type hint aliases used throughout the module.",
    "Constants": "# Global constants for limits, paths, encodings and configuration.",
    "Utility": "# General-purpose utility functions for encoding, formatting and transformation.",
    "Logging": "# Module-level logger configuration.",
    "Exceptions": "# Custom exception hierarchy for server, client and specification errors.",
    "Primitives": "# Abstract base classes for models, fields, ids, inputs, outputs and entities.",
    "Graphql": "# GraphQL node base classes for pydantic, sqlalchemy and relay integration.",
    "Attribute": "# Attribute entity with key-value pairs and definitions.",
    "Tag": "# Tag entity for categorizing and labeling kit elements.",
    "Concept": "# Concept entity for semantic grouping of design elements.",
    "Coord": "# Coordinate primitive for three-dimensional values.",
    "Point": "# Point primitive representing a position in 3D space.",
    "Vector": "# Vector primitive representing a direction in 3D space.",
    "Plane": "# Plane primitive representing an oriented coordinate frame in 3D space.",
    "Location": "# Location entity for geographic coordinates with longitude, latitude and altitude.",
    "Author": "# Author entity for tracking contributor identity and rank.",
    "ArtifactAuthor": "# Artifact-author association entity linking artifacts to authors by email.",
    "File": "# File entity for managing binary assets with metadata and hashing.",
    "Folder": "# Folder entity for hierarchical organization of kit content.",
    "Benchmark": "# Benchmark entity for defining performance metrics with min-max bounds.",
    "Quality": "# Quality entity for defining measurable properties with units and constraints.",
    "Prop": "# Prop entity for key-value property pairs with units.",
    "Model": "# Model entity for 3D geometry representations linked to files.",
    "Port": "# Port entity for defining connection interfaces on types.",
    "CompatiblePort": "# Compatible port entity for specifying allowed port pairings on connectors.",
    "Type": "# Type entity for defining reusable parametric building blocks.",
    "Layer": "# Layer entity for organizing design elements into visibility groups.",
    "Piece": "# Piece entity for placed instances of types within a design.",
    "Group": "# Group entity for named collections of pieces in a design.",
    "Side": "# Side primitive for identifying a specific connector on a specific piece.",
    "Connection": "# Connection entity for linking two pieces through their connectors.",
    "Stat": "# Stat entity for recording computed statistics with bounds.",
    "Design": "# Design entity for composing pieces and connections into assemblies.",
    "Kit": "# Kit entity for packaging types, designs, qualities and metadata.",
    "Design Family Helpers": "# Helper functions for querying design hierarchies and families.",
    "Type Family Helpers": "# Helper functions for querying type hierarchies and families.",
    "Moved Graphene Nodes": "# Graphene node definitions moved here due to forward-reference resolution order.",
    "Validation": "# Validation logic for checking kit constraints and uniqueness rules.",
    "Dict-based Validation": "# Dictionary-based validation functions for kit data integrity.",
    "Graph Operations": "# Graph construction and traversal for piece connectivity analysis.",
    "FlattenDesign": "# Design flattening to resolve nested sub-designs into a single coordinate space.",
    "Kit Diff Operations": "# Diffing and patching operations for comparing and merging kit versions.",
    "Kit Import/Export": "# Import and export utilities for kit serialization and deserialization.",
    "Spatial Math": "# Spatial math utilities for vector normalization and plane computation.",
}

i = 0
insertions = 0
while i < len(lines):
    line = lines[i]
    stripped = line.strip()
    # Match # region XYZ or # #region 🔖XYZ
    m = re.match(r"^# (?:#)?region\s+(?:🔖)?(.+)$", stripped)
    if m:
        section_name = m.group(1).strip()
        if section_name in section_summaries:
            # Check if next line is already a summary comment (not a region, not blank, starts with #)
            next_idx = i + 1
            if next_idx < len(lines):
                next_stripped = lines[next_idx].strip()
                # If next line is not a comment or is a region marker or is blank, insert summary
                is_summary = (
                    next_stripped.startswith("#")
                    and not next_stripped.startswith("# region")
                    and not next_stripped.startswith("# #region")
                    and not next_stripped.startswith("# endregion")
                    and not next_stripped.startswith("# #endregion")
                )
                if not is_summary:
                    summary_line = section_summaries[section_name] + "\n"
                    lines.insert(next_idx, summary_line)
                    insertions += 1
    i += 1

print(f"Added {insertions} section summaries")

# --- PASS 3: Add definition summaries and requirements ---
# We need to find all exported definitions (class, def, async def) that need summaries/requirements
# and add the appropriate comments above them.

# Parse breachs from files to know exact names and current line numbers
# Instead, we'll re-scan the modified lines

# Build a map of definition name -> (summary_text, spec_text)
# For classes: summary describes the class, spec describes what it MUST do
# For functions: summary describes the function, spec describes what it MUST do


def make_summary(name, kind, parent_classes="", body_hint=""):
    """Generate a meaningful summary based on definition name and context."""
    # Exception classes
    if (
        "Error" in name
        or "NotFound" in name
        or "NotYetSupported" in name
        or "NotAssigned" in name
        or "AlreadyExists" in name
        or "Unreachable" in name
    ):
        if name == "Error":
            return "# Base exception for all compose errors."
        if name == "ServerError":
            return "# Base exception for server-side errors."
        if name == "ClientError":
            return "# Base exception for client-side errors."
        if name == "CodeUnreachable":
            return "# Exception for code paths that should never be reached."
        if name == "FeatureNotYetSupported":
            return "# Exception for unimplemented features."
        if name == "RemoteKitsNotYetSupported":
            return "# Exception for unsupported remote kit access."
        if name == "NotFound":
            return "# Base exception for entities not found in the store."
        if name == "SpecificationError":
            return "# Base exception for specification constraint breachs."
        if name == "NoParentAssigned":
            return "# Base exception for entities missing a required parent."
        if name == "NoTypeOrDesignAssigned":
            return "# Exception for entities missing both type and design parent."
        if name == "NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned":
            return "# Exception for entities missing all possible parent assignments."
        if name == "AlreadyExists":
            return "# Base exception for duplicate entity creation attempts."
        if name == "NoModelAssigned":
            return "# Exception for entities missing a required model assignment."
        if name == "ConnectorNotFound":
            return "# Exception for a connector not found on a type."
        if name == "TypeNotFound":
            return "# Exception for a type not found in the kit."
        if name == "NoTypeAssigned":
            return "# Exception for entities missing a required type assignment."
        if name == "TypeHasNotAllUsedConnectors":
            return (
                "# Exception for a type missing connectors referenced by connections."
            )
        if name == "NoDesignAssigned":
            return "# Exception for entities missing a required design assignment."
        if name == "KitNotFound":
            return "# Exception for a kit not found at the given URI."
        if name == "NoKitToDelete":
            return "# Exception for deletion of a non-existent kit."
        if name == "KitZipDoesNotContainComposeFolder":
            return "# Exception for a remote zip kit missing the compose folder."
        if name == "OnlyRemoteKitsCanBeCached":
            return "# Exception for attempting to cache a non-remote kit."
        if name == "KitUriNotValid":
            return "# Base exception for invalid kit URI formats."
        if name == "LocalKitUriNotValid":
            return "# Base exception for invalid local kit URI formats."
        if name == "LocalKitUriIsNotAbsolute":
            return "# Exception for a local kit URI that is not absolute."
        if name == "LocalKitUriIsNotDirectory":
            return "# Exception for a local kit URI that is not a directory."
        if name == "NoKitAssigned":
            return "# Exception for entities missing a required kit assignment."
        if name == "KitAlreadyExists":
            return "# Exception for attempting to create a kit that already exists."
        # Generic error/exception fallback
        return f"# Exception for {name.replace('Error', '').replace('NotFound', ' not found').replace('NotYetSupported', ' not yet supported')} conditions."

    # Field classes
    if name.endswith("Field"):
        entity = name[:-5]
        # Parse entity and field name
        # e.g. AttributeKeyField -> Attribute entity, Key field
        # Find the entity prefix
        for prefix in [
            "Attribute",
            "Tag",
            "Concept",
            "Coord",
            "Point",
            "Vector",
            "Plane",
            "Location",
            "Author",
            "ArtifactAuthor",
            "File",
            "Folder",
            "Benchmark",
            "Quality",
            "Prop",
            "Model",
            "Port",
            "CompatiblePort",
            "Connector",
            "Type",
            "Layer",
            "Piece",
            "Group",
            "Side",
            "Connection",
            "Stat",
            "Design",
            "Kit",
        ]:
            if name.startswith(prefix):
                field_name = name[len(prefix) : -5]
                return f"# Field mixin for the {camel_to_words(field_name).lower()} of a {camel_to_words(prefix).lower()}."
        return f"# Field mixin for the {entity.lower()} property."

    # Id classes
    if name.endswith("Id") and name != "Id":
        entity = name[:-2]
        return f"# Identity fields for uniquely identifying a {camel_to_words(entity).lower()}."

    # Props classes
    if name.endswith("Props") and name != "Props":
        entity = name[:-5]
        return f"# Property fields for a {camel_to_words(entity).lower()}."

    # Input classes
    if name.endswith("Input") and name != "Input":
        entity = name[:-5]
        return f"# Input fields for creating or updating a {camel_to_words(entity).lower()}."

    # Context classes
    if name.endswith("Context") and name != "Context":
        entity = name[:-7]
        return f"# Context fields for understanding a {camel_to_words(entity).lower()} by an LLM."

    # Output classes
    if name.endswith("Output") and name != "Output":
        entity = name[:-6]
        return f"# Output fields returned when fetching a {camel_to_words(entity).lower()}."

    # Prediction classes
    if name.endswith("Prediction") and name != "Prediction":
        entity = name[:-10]
        return f"# Prediction fields for LLM-based {camel_to_words(entity).lower()} inference."

    # InputNode classes
    if name.endswith("InputNode"):
        entity = name[:-9]
        return f"# GraphQL input node for {camel_to_words(entity).lower()} mutations."

    # IdInputNode classes
    if name.endswith("IdInputNode"):
        entity = name[:-11]
        return (
            f"# GraphQL input node for {camel_to_words(entity).lower()} identification."
        )

    # Node classes (but not InputNode/IdInputNode)
    if (
        name.endswith("Node")
        and not name.endswith("InputNode")
        and not name.endswith("IdInputNode")
    ):
        entity = name[:-4]
        if entity in ["Relay", "Table", "TableEntity"]:
            pass  # handled below
        else:
            return f"# GraphQL node exposing {camel_to_words(entity).lower()} data."

    # Functions
    if kind == "def" or kind == "async def":
        summaries = {
            "encode": "# Encode a string to be URL-safe using percent-encoding.",
            "decode": "# Decode a percent-encoded URL-safe string.",
            "encodeList": "# Encode a list of strings into a comma-separated URL-safe string.",
            "decodeList": "# Decode a comma-separated URL-safe string into a list of strings.",
            "encodeRecursiveAnyList": "# Recursively encode a nested list into a flat URL-safe string.",
            "create_id": "# Create a unique identifier from a value or recursive list.",
            "pretty": "# Format a floating-point number with significant digits and no trailing zeros.",
            "changeValues": "# Recursively change values for a given key in nested dicts and lists.",
            "changeKeys": "# Recursively transform all keys in nested dicts and lists.",
            "normalizeAngle": "# Normalize an angle to the range [0, 360) degrees.",
            "areValidationResultsEqual": "# Check whether two validation results are semantically equal.",
            "parseValidationResult": "# Parse a validation result from a dictionary representation.",
            "validateGuidUniqueness": "# Validate that all GUIDs within a collection are unique.",
            "validateTypeNameUniqueness": "# Validate that all type names within a kit are unique.",
            "validateDesignNameUniqueness": "# Validate that all design names within a kit are unique.",
            "validatePieceNameUniqueness": "# Validate that all piece names within a design are unique.",
            "validatePortNameUniqueness": "# Validate that all port names within a type are unique.",
            "validateModelNameUniqueness": "# Validate that all model names within a type are unique.",
            "validateQualityNameUniqueness": "# Validate that all quality names within a kit are unique.",
            "validateFileNameUniqueness": "# Validate that all file names within a kit are unique.",
            "validateFolderNameUniqueness": "# Validate that all folder names within a kit are unique.",
            "validateLayerPathUniqueness": "# Validate that all layer paths within a design are unique.",
            "validateKit": "# Validate a kit entity against all constraint rules.",
            "validateKitDict": "# Validate a kit dictionary against all constraint rules.",
            "buildPieceGraph": "# Build a networkx graph from pieces and connections.",
            "findFixedPieces": "# Find all pieces that are fixed in the design hierarchy.",
            "getConnectedComponents": "# Get connected components of the piece graph.",
            "getPieceHierarchy": "# Get the hierarchical ordering of pieces from root to leaf.",
            "getTypeByGuid": "# Look up a type by its GUID within a kit dictionary.",
            "getConnectorFromType": "# Look up a connector by name from a type dictionary.",
            "planeToMatrixDict": "# Convert a plane dictionary to a 4x4 transformation matrix.",
            "matrixToPlaneDict": "# Convert a 4x4 transformation matrix to a plane dictionary.",
            "quaternionFromUnitVectorsDict": "# Compute a quaternion rotating one unit vector onto another.",
            "quaternionFromAxisAngleDict": "# Compute a quaternion from an axis-angle representation.",
            "quaternionToMatrixDict": "# Convert a quaternion to a 3x3 rotation matrix.",
            "makeRotationAxisDict": "# Create a 4x4 rotation matrix around an arbitrary axis.",
            "makeTranslationDict": "# Create a 4x4 translation matrix from a displacement vector.",
            "applyMatrix4ToVec3Dict": "# Apply a 4x4 matrix to a 3D vector dictionary.",
            "computeChildPlaneDict": "# Compute the world-space plane of a child piece from parent and local planes.",
            "flattenDesignDict": "# Flatten a nested design hierarchy into a single flat coordinate space.",
            "areAttributesEqualDict": "# Check whether two attribute dictionaries are equal.",
            "arePropsEqualDict": "# Check whether two prop dictionaries are equal.",
            "arePortsEqualDict": "# Check whether two port dictionaries are equal.",
            "areModelsEqualDict": "# Check whether two model dictionaries are equal.",
            "areTypesEqualDict": "# Check whether two type dictionaries are equal.",
            "arePiecesEqualDict": "# Check whether two piece dictionaries are equal.",
            "areConnectionsEqualDict": "# Check whether two connection dictionaries are equal.",
            "areDesignsEqualDict": "# Check whether two design dictionaries are equal.",
            "areQualitiesEqualDict": "# Check whether two quality dictionaries are equal.",
            "areFilesEqualDict": "# Check whether two file dictionaries are equal.",
            "areFoldersEqualDict": "# Check whether two folder dictionaries are equal.",
            "areAuthorsEqualDict": "# Check whether two author dictionaries are equal.",
            "areConceptsEqualDict": "# Check whether two concept dictionaries are equal.",
            "areTagsEqualDict": "# Check whether two tag dictionaries are equal.",
            "areKitsDictEqual": "# Check whether two kit dictionaries are semantically equal.",
            "getKitDiffDict": "# Compute the difference between two kit dictionaries.",
            "applyKitDiffDict": "# Apply a kit diff to a kit dictionary to produce an updated kit.",
            "inverseKitDiffDict": "# Invert a kit diff to reverse its effect.",
            "areKitDiffsDictEqual": "# Check whether two kit diff dictionaries are equal.",
            "import_kit": "# Import a kit from a local or remote URI into memory.",
            "export_kit": "# Export a kit from memory to a local file path.",
            "normalizeVector": "# Normalize a 3D vector to unit length.",
            "planeFromYAxis": "# Construct a plane from an origin point and a Y-axis direction.",
            "computeChildPlane": "# Compute the world-space plane of a child from parent and local planes.",
        }
        if name in summaries:
            return summaries[name]
        return f"# {name.replace('_', ' ').capitalize()} operation."

    # Specific named classes
    class_summaries = {
        "Compose": "# Metadata table recording the database release and engine version.",
        "SModel": "# Abstract base model for all compose pydantic models.",
        "Field": "# Abstract base for a single field mixin of a model.",
        "RealField": "# Abstract base for a required field mixin of a model.",
        "MaskedField": "# Abstract base for an optional masked field mixin of a model.",
        "Base": "# Abstract base for composite model groupings.",
        "Id": "# Abstract base for identity field groupings.",
        "Props": "# Abstract base for property field groupings.",
        "Input": "# Abstract base for input field groupings.",
        "Context": "# Abstract base for context field groupings.",
        "Output": "# Abstract base for output field groupings.",
        "Prediction": "# Abstract base for prediction field groupings.",
        "Entity": "# Abstract base for all domain entities with identity and hierarchy.",
        "Table": "# Abstract base for database table models.",
        "TableEntity": "# Abstract base for entities persisted as database tables.",
        "Node": "# Base GraphQL object type for non-table pydantic models.",
        "InputNode": "# Base GraphQL input type for pydantic models.",
        "RelayNode": "# Relay-compliant GraphQL node interface.",
        "TableNode": "# Base GraphQL object type for SQLAlchemy table models.",
        "TableEntityNode": "# Base GraphQL object type for table entities with Relay interface.",
        "Attribute": "# Attribute entity storing a key-value pair with an optional definition.",
        "AttributeInputNode": "# GraphQL input node for attribute mutations.",
        "Tag": "# Tag entity for labeling kit elements with name, icon and order.",
        "Concept": "# Concept entity for semantic grouping with name, icon and order.",
        "Coord": "# Three-dimensional coordinate with x, y and z values.",
        "CoordInput": "# Input fields for creating a coordinate.",
        "CoordContext": "# Context fields for a coordinate.",
        "CoordOutput": "# Output fields for a coordinate.",
        "CoordPrediction": "# Prediction fields for a coordinate.",
        "CoordNode": "# GraphQL node for coordinate data.",
        "CoordInputNode": "# GraphQL input node for coordinate mutations.",
        "Point": "# Point in 3D space with x, y and z coordinates.",
        "PointInput": "# Input fields for creating a point.",
        "PointContext": "# Context fields for a point.",
        "PointOutput": "# Output fields for a point.",
        "PointPrediction": "# Prediction fields for a point.",
        "PointNode": "# GraphQL node for point data.",
        "PointInputNode": "# GraphQL input node for point mutations.",
        "Vector": "# Direction vector in 3D space with x, y and z components.",
        "VectorInput": "# Input fields for creating a vector.",
        "VectorContext": "# Context fields for a vector.",
        "VectorOutput": "# Output fields for a vector.",
        "VectorPrediction": "# Prediction fields for a vector.",
        "VectorNode": "# GraphQL node for vector data.",
        "VectorInputNode": "# GraphQL input node for vector mutations.",
        "Plane": "# Oriented coordinate frame in 3D space with origin and axes.",
        "PlaneInput": "# Input fields for creating a plane.",
        "PlaneContext": "# Context fields for a plane.",
        "PlaneOutput": "# Output fields for a plane.",
        "PlaneInputNode": "# GraphQL input node for plane mutations.",
        "PlaneNode": "# GraphQL node for plane data.",
        "Location": "# Geographic location with longitude, latitude and altitude.",
        "LocationInput": "# Input fields for creating a location.",
        "LocationOutput": "# Output fields for a location.",
        "LocationContext": "# Context fields for a location.",
        "LocationPrediction": "# Prediction fields for a location.",
        "LocationNode": "# GraphQL node for location data.",
        "LocationInputNode": "# GraphQL input node for location mutations.",
        "Author": "# Author entity with name, email and contribution rank.",
        "AuthorInputNode": "# GraphQL input node for author mutations.",
        "ArtifactAuthor": "# Association entity linking an artifact to an author by email.",
        "File": "# File entity for binary assets with metadata, hashing and timestamps.",
        "FileInput": "# Input fields for creating a file.",
        "FileContext": "# Context fields for a file.",
        "FileOutput": "# Output fields for a file.",
        "FileInputNode": "# GraphQL input node for file mutations.",
        "Folder": "# Folder entity for hierarchical content organization.",
        "FolderInput": "# Input fields for creating a folder.",
        "FolderContext": "# Context fields for a folder.",
        "FolderOutput": "# Output fields for a folder.",
        "FolderInputNode": "# GraphQL input node for folder mutations.",
        "Benchmark": "# Benchmark entity for performance metrics with min-max bounds.",
        "BenchmarkInput": "# Input fields for creating a benchmark.",
        "BenchmarkOutput": "# Output fields for a benchmark.",
        "Quality": "# Quality entity with units, constraints, formula and folder classification.",
        "QualityInput": "# Input fields for creating a quality.",
        "QualityContext": "# Context fields for a quality.",
        "QualityOutput": "# Output fields for a quality.",
        "Prop": "# Prop entity for key-value properties with optional units.",
        "PropInput": "# Input fields for creating a prop.",
        "PropOutput": "# Output fields for a prop.",
        "PropInputNode": "# GraphQL input node for prop mutations.",
        "Model": "# Model entity for 3D geometry with name, URL and file reference.",
        "ModelInput": "# Input fields for creating a model.",
        "ModelContext": "# Context fields for a model.",
        "ModelOutput": "# Output fields for a model.",
        "Port": "# Port entity defining a named connection interface on a type.",
        "PortInput": "# Input fields for creating a port.",
        "PortOutput": "# Output fields for a port.",
        "PortInputNode": "# GraphQL input node for port mutations.",
        "CompatiblePort": "# Compatible port entity specifying an allowed port pairing.",
        "Connector": "# Connector entity defining a localized connection point on a type.",
        "ConnectorInput": "# Input fields for creating a connector.",
        "ConnectorContext": "# Context fields for a connector.",
        "ConnectorOutput": "# Output fields for a connector.",
        "ConnectorInputNode": "# GraphQL input node for connector mutations.",
        "ConnectorIdInputNode": "# GraphQL input node for connector identification.",
        "Type": "# Type entity defining a reusable parametric building block.",
        "TypeInput": "# Input fields for creating a type.",
        "TypeOutput": "# Output fields for a type.",
        "TypeContext": "# Context fields for a type.",
        "TypeInputNode": "# GraphQL input node for type mutations.",
        "TypeIdInputNode": "# GraphQL input node for type identification.",
        "Layer": "# Layer entity for grouping design elements with visibility and locking.",
        "LayerInput": "# Input fields for creating a layer.",
        "LayerOutput": "# Output fields for a layer.",
        "Piece": "# Piece entity for a placed instance of a type within a design.",
        "PieceInput": "# Input fields for creating a piece.",
        "PieceContext": "# Context fields for a piece.",
        "PieceOutput": "# Output fields for a piece.",
        "PiecePrediction": "# Prediction fields for a piece.",
        "PieceInputNode": "# GraphQL input node for piece mutations.",
        "PieceIdInputNode": "# GraphQL input node for piece identification.",
        "Group": "# Group entity for named collections of pieces.",
        "GroupInput": "# Input fields for creating a group.",
        "GroupOutput": "# Output fields for a group.",
        "Side": "# Side primitive identifying a specific connector on a specific piece.",
        "SideInput": "# Input fields for creating a side.",
        "SideContext": "# Context fields for a side.",
        "SideOutput": "# Output fields for a side.",
        "SidePrediction": "# Prediction fields for a side.",
        "SideNode": "# GraphQL node for side data.",
        "SideInputNode": "# GraphQL input node for side mutations.",
        "Connection": "# Connection entity linking two pieces through their connectors.",
        "ConnectionInput": "# Input fields for creating a connection.",
        "ConnectionContext": "# Context fields for a connection.",
        "ConnectionOutput": "# Output fields for a connection.",
        "ConnectionPrediction": "# Prediction fields for a connection.",
        "ConnectionInputNode": "# GraphQL input node for connection mutations.",
        "Stat": "# Stat entity for recording computed statistics with bounds.",
        "StatInput": "# Input fields for creating a stat.",
        "StatOutput": "# Output fields for a stat.",
        "Design": "# Design entity composing pieces and connections into an assembly.",
        "DesignInput": "# Input fields for creating a design.",
        "DesignContext": "# Context fields for a design.",
        "DesignOutput": "# Output fields for a design.",
        "DesignPrediction": "# Prediction fields for a design.",
        "DesignInputNode": "# GraphQL input node for design mutations.",
        "DesignIdInputNode": "# GraphQL input node for design identification.",
        "Kit": "# Kit entity packaging types, designs, qualities and metadata.",
        "KitInput": "# Input fields for creating a kit.",
        "KitContext": "# Context fields for a kit.",
        "KitOutput": "# Output fields for a kit.",
        "KitInputNode": "# GraphQL input node for kit mutations.",
        "KitNode": "# GraphQL node for kit data.",
        "AttributeNode": "# GraphQL node for attribute data.",
        "AuthorNode": "# GraphQL node for author data.",
        "ModelNode": "# GraphQL node for model data.",
        "ConnectorNode": "# GraphQL node for connector data.",
        "TypeNode": "# GraphQL node for type data.",
        "PieceNode": "# GraphQL node for piece data.",
        "ConnectionNode": "# GraphQL node for connection data.",
        "DesignNode": "# GraphQL node for design data.",
        "ValidationFix": "# A proposed fix for a validation problem with a title and diff.",
        "Problem": "# A validation problem with a constraint identifier and message.",
        "ValidationResult": "# A validation result aggregating problems and fixes for an entity.",
        "KitData": "# Data container for in-memory kit content during import and export.",
    }
    if name in class_summaries:
        return class_summaries[name]

    # Generic class fallback
    return f"# {camel_to_words(name)} definition."


def make_spec(name, kind):
    """Generate a meaningful spec based on definition name and context."""
    # Exception classes - they must raise with a descriptive message
    if (
        "Error" in name
        or "NotFound" in name
        or "NotYetSupported" in name
        or "NotAssigned" in name
        or "AlreadyExists" in name
        or "Unreachable" in name
    ):
        return f"# {name} MUST provide a descriptive error message via __str__."

    # Field classes
    if name.endswith("Field"):
        return f"# {name} MUST declare exactly one field with appropriate constraints."

    # Id, Props, Input, Context, Output, Prediction classes
    if name.endswith("Id") and name != "Id":
        entity = name[:-2]
        return f"# {name} MUST contain all fields that uniquely identify a {camel_to_words(entity).lower()}."
    if name.endswith("Props") and name != "Props":
        return f"# {name} MUST contain all non-relational property fields."
    if name.endswith("Input") and name != "Input":
        return f"# {name} MUST contain all fields required for creation."
    if name.endswith("Context") and name != "Context":
        return f"# {name} MUST contain all fields needed for LLM understanding."
    if name.endswith("Output") and name != "Output":
        return f"# {name} MUST contain all fields returned on fetch."
    if name.endswith("Prediction") and name != "Prediction":
        return f"# {name} MUST contain all fields for LLM inference."

    # Node classes
    if name.endswith("InputNode"):
        return f"# {name} MUST expose the input model via Meta."
    if name.endswith("IdInputNode"):
        return f"# {name} MUST expose the id model via Meta."
    if name.endswith("Node"):
        return f"# {name} MUST expose the model via Meta."

    # Entity classes
    entity_names = [
        "Attribute",
        "Tag",
        "Concept",
        "Location",
        "Author",
        "ArtifactAuthor",
        "File",
        "Folder",
        "Benchmark",
        "Quality",
        "Prop",
        "Model",
        "Port",
        "CompatiblePort",
        "Connector",
        "Type",
        "Layer",
        "Piece",
        "Group",
        "Connection",
        "Stat",
        "Design",
        "Kit",
        "Compose",
    ]
    if name in entity_names:
        return f"# {name} MUST implement idMembers and inherit from the appropriate field mixins."

    # Abstract bases
    abstract_names = {
        "SModel",
        "Field",
        "RealField",
        "MaskedField",
        "Base",
        "Id",
        "Props",
        "Input",
        "Context",
        "Output",
        "Prediction",
        "Entity",
        "Table",
        "TableEntity",
        "Node",
        "InputNode",
        "RelayNode",
        "TableNode",
        "TableEntityNode",
        "Error",
        "ServerError",
        "ClientError",
        "NotFound",
        "SpecificationError",
        "NoParentAssigned",
        "AlreadyExists",
        "KitUriNotValid",
        "LocalKitUriNotValid",
    }
    if name in abstract_names:
        return f"# {name} MUST be subclassed and MUST NOT be instantiated directly."

    # Coord/Point/Vector/Plane/Side primitives
    primitives = {"Coord", "Point", "Vector", "Plane", "Side"}
    if name in primitives:
        return f"# {name} MUST contain all coordinate or geometry fields."

    # Functions
    if kind == "def" or kind == "async def":
        func_requirements = {
            "encode": "# encode MUST return a percent-encoded string safe for URL paths.",
            "decode": "# decode MUST return the original string from a percent-encoded input.",
            "encodeList": "# encodeList MUST encode each item and join them with commas.",
            "decodeList": "# decodeList MUST split by comma and decode each item.",
            "encodeRecursiveAnyList": "# encodeRecursiveAnyList MUST recursively encode nested lists into a flat string.",
            "create_id": "# create_id MUST produce a deterministic identifier from any value or nested list.",
            "pretty": "# pretty MUST format the number with up to 5 significant digits.",
            "changeValues": "# changeValues MUST apply the function to all occurrences of the key recursively.",
            "changeKeys": "# changeKeys MUST apply the function to all dictionary keys recursively.",
            "normalizeAngle": "# normalizeAngle MUST return an angle in the range [0, 360).",
            "areValidationResultsEqual": "# areValidationResultsEqual MUST compare all problems and fixes.",
            "parseValidationResult": "# parseValidationResult MUST return a ValidationResult from a dict.",
            "validateGuidUniqueness": "# validateGuidUniqueness MUST report duplicate GUIDs as problems.",
            "validateTypeNameUniqueness": "# validateTypeNameUniqueness MUST report duplicate type names as problems.",
            "validateDesignNameUniqueness": "# validateDesignNameUniqueness MUST report duplicate design names as problems.",
            "validatePieceNameUniqueness": "# validatePieceNameUniqueness MUST report duplicate piece names as problems.",
            "validatePortNameUniqueness": "# validatePortNameUniqueness MUST report duplicate port names as problems.",
            "validateModelNameUniqueness": "# validateModelNameUniqueness MUST report duplicate model names as problems.",
            "validateQualityNameUniqueness": "# validateQualityNameUniqueness MUST report duplicate quality names as problems.",
            "validateFileNameUniqueness": "# validateFileNameUniqueness MUST report duplicate file names as problems.",
            "validateFolderNameUniqueness": "# validateFolderNameUniqueness MUST report duplicate folder names as problems.",
            "validateLayerPathUniqueness": "# validateLayerPathUniqueness MUST report duplicate layer paths as problems.",
            "validateKit": "# validateKit MUST run all validation checks and return aggregated results.",
            "validateKitDict": "# validateKitDict MUST validate a kit dictionary and return results.",
            "buildPieceGraph": "# buildPieceGraph MUST return a networkx graph with pieces as nodes.",
            "findFixedPieces": "# findFixedPieces MUST return pieces that have no incoming connections.",
            "getConnectedComponents": "# getConnectedComponents MUST return disjoint piece groups.",
            "getPieceHierarchy": "# getPieceHierarchy MUST return a topological ordering of pieces.",
            "getTypeByGuid": "# getTypeByGuid MUST return the type dict or raise if not found.",
            "getConnectorFromType": "# getConnectorFromType MUST return the matching connector dict.",
            "planeToMatrixDict": "# planeToMatrixDict MUST produce a valid 4x4 homogeneous matrix.",
            "matrixToPlaneDict": "# matrixToPlaneDict MUST extract origin, xAxis and yAxis from the matrix.",
            "quaternionFromUnitVectorsDict": "# quaternionFromUnitVectorsDict MUST compute the shortest rotation quaternion.",
            "quaternionFromAxisAngleDict": "# quaternionFromAxisAngleDict MUST compute the quaternion for the given rotation.",
            "quaternionToMatrixDict": "# quaternionToMatrixDict MUST produce a valid 3x3 rotation matrix.",
            "makeRotationAxisDict": "# makeRotationAxisDict MUST return a 4x4 rotation matrix around the axis.",
            "makeTranslationDict": "# makeTranslationDict MUST return a 4x4 translation matrix.",
            "applyMatrix4ToVec3Dict": "# applyMatrix4ToVec3Dict MUST apply the full affine transformation.",
            "computeChildPlaneDict": "# computeChildPlaneDict MUST compose parent and local transformations correctly.",
            "flattenDesignDict": "# flattenDesignDict MUST resolve all nested designs into world coordinates.",
            "areAttributesEqualDict": "# areAttributesEqualDict MUST compare all attribute fields for equality.",
            "arePropsEqualDict": "# arePropsEqualDict MUST compare all prop fields for equality.",
            "arePortsEqualDict": "# arePortsEqualDict MUST compare all port fields for equality.",
            "areModelsEqualDict": "# areModelsEqualDict MUST compare all model fields for equality.",
            "areTypesEqualDict": "# areTypesEqualDict MUST compare all type fields including children for equality.",
            "arePiecesEqualDict": "# arePiecesEqualDict MUST compare all piece fields for equality.",
            "areConnectionsEqualDict": "# areConnectionsEqualDict MUST compare all connection fields for equality.",
            "areDesignsEqualDict": "# areDesignsEqualDict MUST compare all design fields including children for equality.",
            "areQualitiesEqualDict": "# areQualitiesEqualDict MUST compare all quality fields for equality.",
            "areFilesEqualDict": "# areFilesEqualDict MUST compare all file fields for equality.",
            "areFoldersEqualDict": "# areFoldersEqualDict MUST compare all folder fields for equality.",
            "areAuthorsEqualDict": "# areAuthorsEqualDict MUST compare all author fields for equality.",
            "areConceptsEqualDict": "# areConceptsEqualDict MUST compare all concept fields for equality.",
            "areTagsEqualDict": "# areTagsEqualDict MUST compare all tag fields for equality.",
            "areKitsDictEqual": "# areKitsDictEqual MUST compare all kit fields and children recursively.",
            "getKitDiffDict": "# getKitDiffDict MUST identify all added, removed and changed entities.",
            "applyKitDiffDict": "# applyKitDiffDict MUST apply additions, removals and changes correctly.",
            "inverseKitDiffDict": "# inverseKitDiffDict MUST swap additions and removals to reverse the diff.",
            "areKitDiffsDictEqual": "# areKitDiffsDictEqual MUST compare all diff entries for equality.",
            "import_kit": "# import_kit MUST handle both local directories and remote zip archives.",
            "export_kit": "# export_kit MUST write the kit database and files to the target path.",
            "normalizeVector": "# normalizeVector MUST return a unit-length vector or raise on zero length.",
            "planeFromYAxis": "# planeFromYAxis MUST derive orthogonal x and z axes from the y axis.",
            "computeChildPlane": "# computeChildPlane MUST compose parent and local plane transformations.",
        }
        if name in func_requirements:
            return func_requirements[name]
        return f"# {name} MUST complete its operation and return a valid result."

    # Validation-related classes
    if name == "ValidationFix":
        return "# ValidationFix MUST contain a non-empty title and a valid diff dictionary."
    if name == "Problem":
        return "# Problem MUST contain a non-empty constraint identifier and message."
    if name == "ValidationResult":
        return "# ValidationResult MUST aggregate all problems and fixes for a single entity."
    if name == "KitData":
        return "# KitData MUST hold all kit entities in memory for import and export operations."

    return f"# {name} MUST fulfill its documented contract."


def camel_to_words(name):
    """Convert CamelCase to space-separated words."""
    words = re.sub(r"([A-Z])", r" \1", name).strip()
    return words


# Now process the file to add summaries and requirements above definitions
# We need to find all class/def/async def lines that are in the breach list

# Build a set of (name, line_number) pairs that need summary
needs_summary = set()
with open("/tmp/all_def_summary.txt") as f:
    for line in f:
        parts = line.strip().split(":")
        if len(parts) == 2:
            needs_summary.add(parts[0])

# Build a set of (name, line_number) pairs that need requirements
needs_requirements = set()
with open("/tmp/all_def_requirements.txt") as f:
    for line in f:
        parts = line.strip().split(":")
        if len(parts) == 2:
            needs_requirements.add(parts[0])

# Process lines from bottom to top to avoid index shifting issues
# First, find all definition positions and their decorators
def_pattern = re.compile(r"^(\s*)(class|def|async def)\s+(\w+)")
decorator_pattern = re.compile(r"^(\s*)@")

definitions = []  # list of (def_line_idx, name, kind, insert_idx)

i = 0
while i < len(lines):
    m = def_pattern.match(lines[i])
    if m:
        indent = m.group(1)
        kind = m.group(2)
        name = m.group(3)
        # Find the topmost decorator for this definition
        insert_idx = i
        j = i - 1
        while j >= 0:
            stripped = lines[j].strip()
            if stripped.startswith("@"):
                insert_idx = j
                j -= 1
            elif stripped == "" or stripped.startswith("#"):
                # Skip blank lines and comments above decorators to find more decorators
                # But only if there's a decorator above
                k = j - 1
                found_decorator = False
                while k >= 0:
                    ks = lines[k].strip()
                    if ks.startswith("@"):
                        found_decorator = True
                        break
                    elif ks == "" or ks.startswith("#"):
                        k -= 1
                    else:
                        break
                if found_decorator:
                    j -= 1
                else:
                    break
            else:
                break
        definitions.append((i, name, kind, insert_idx))
    i += 1

# Process from bottom to top
changes = 0
for def_line_idx, name, kind, insert_idx in reversed(definitions):
    need_summary = name in needs_summary
    need_spec = name in needs_requirements

    if not need_summary and not need_spec:
        continue

    # Check what's already above the insert point
    # The line above insert_idx
    above_idx = insert_idx - 1
    existing_comments = []
    while above_idx >= 0:
        stripped = lines[above_idx].strip()
        if (
            stripped.startswith("#")
            and not stripped.startswith("# region")
            and not stripped.startswith("# #region")
            and not stripped.startswith("# endregion")
            and not stripped.startswith("# #endregion")
        ):
            existing_comments.insert(0, stripped)
            above_idx -= 1
        else:
            break

    # Determine indentation from the definition line
    indent = ""
    m2 = re.match(r"^(\s*)", lines[insert_idx])
    if m2:
        indent = m2.group(1)

    lines_to_insert = []
    if need_spec:
        spec_text = make_spec(name, kind)
        lines_to_insert.append(indent + spec_text + "\n")
    if need_summary:
        summary_text = make_summary(name, kind)
        lines_to_insert.append(indent + summary_text + "\n")

    for line_to_insert in reversed(lines_to_insert):
        lines.insert(insert_idx, line_to_insert)
        changes += 1

print(f"Added {changes} definition comment lines")

# Write the file
with open(FILE, "w") as f:
    f.writelines(lines)

print(f"Done. Total changes: {insertions + changes + 1}")

using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.Data.SqlTypes;
using System.Linq;
using Force.DeepCloner;
using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.Newtonsoft;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using QuikGraph;
using QuikGraph.Algorithms;
using Semio.Properties;

// TODO: Replace GetHashcode() with a proper hash function.
// TODO: Add logging mechanism to all API calls if they fail.
// TODO: Add a more detailed message system when a model is invalid.

#region Constants

public static class Constants
{
    public const int NameLengthLimit = 255;
    public const int UrlLengthLimit = 2048;
    public const int DescriptionLengthLimit = 4096;
}

#endregion

#region Copilot
//type Query
//{
//loadLocalKit(directory: String!): LoadLocalKitResponse
//  designToSceneFromLocalKit(directory: String!, designIdInput: DesignIdInput!): DesignToSceneFromLocalKitResponse
//}

//type LoadLocalKitResponse
//{
//kit: Kit
//  error: LoadLocalKitError
//}

//"""🗃️ A kit is a collection of types and designs."""
//type Kit
//{
//name: String!
//  description: String!
//  icon: String!
//  createdAt: DateTime!
//  lastUpdateAt: DateTime!
//  url: String!
//  homepage: String!
//  types: [Type!]!
//  designs: [Design!]!
//}

//"""
//The `DateTime` scalar type represents a DateTime
//value as specified by
//[iso8601](https://en.wikipedia.org/wiki/ISO_8601).
//"""
//scalar DateTime

//"""
//🧩 A type is a reusable element that can be connected with other types over ports.
//"""
//type Type {
//  name: String!
//  description: String!
//  icon: String!
//  variant: String!
//  unit: String!
//  createdAt: DateTime!
//  lastUpdateAt: DateTime!
//  kit: Kit
//  representations: [Representation!]!
//  ports: [Port!]!
//  qualities: [Quality!]!
//  pieces: [Piece!]!
//}

//"""
//💾 A representation is a link to a file that describes a type for a certain level of detail and tags.
//"""
//type Representation
//{
//url: String!
//  mime: String!
//  lod: String!
//  type: Type
//  tags: [String!]!
//}

//"""
//🔌 A port is a conceptual connection point (with a direction) of a type.
//"""
//type Port
//{
//type: Type
//  locators: [Locator!]!
//  connecteds: [Connection!]!
//  connectings: [Connection!]!
//  id: String!
//  point: Point!
//  direction: Vector!
//  plane: Plane!
//}

//"""🗺️ A locator is meta-data for grouping ports."""
//type Locator
//{
//group: String!
//  subgroup: String!
//  port: Port
//}

//"""🖇️ A connection between two pieces of a design."""
//type Connection
//{
//offset: Float!
//  rotation: Float!
//  design: Design
//  connected: Side!
//  connecting: Side!
//}

//"""🏙️ A design is a collection of pieces that are connected."""
//type Design
//{
//name: String!
//  description: String!
//  icon: String!
//  variant: String!
//  unit: String!
//  createdAt: DateTime!
//  lastUpdateAt: DateTime!
//  kit: Kit
//  pieces: [Piece!]!
//  connections: [Connection!]!
//  qualities: [Quality!]!
//}

//"""⭕ A piece is a 3d-instance of a type in a design."""
//type Piece
//{
//type: Type
//  design: Design
//  connectings: [Connection!]!
//  connecteds: [Connection!]!
//  id: String!
//  root: PieceRoot
//  diagram: PieceDiagram!
//}

//"""🌱 The root indesign of a piece."""
//type PieceRoot
//{
//plane: Plane!
//}

//"""◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""
//type Plane
//{
//origin: Point!
//  xAxis: Vector!
//  yAxis: Vector!
//}

//"""✖️ A 3d-point (xyz) of floating point numbers."""
//type Point
//{
//x: Float!
//  y: Float!
//  z: Float!
//}

//"""➡️ A 3d-vector (xyz) of floating point numbers."""
//type Vector
//{
//x: Float!
//  y: Float!
//  z: Float!
//}

//"""✏️ The diagram indesign of a piece."""
//type PieceDiagram
//{
//point: ScreenPoint!
//}

//"""📺 A 2d-point (xy) of integers in screen plane."""
//type ScreenPoint
//{
//x: Int!
//  y: Int!
//}

//"""📏 A quality is meta-data for decision making."""
//type Quality
//{
//name: String!
//  value: String!
//  unit: String!
//  definition: String!
//  type: Type
//  design: Design
//}

//"""🧱 A side of a piece in a connection."""
//type Side
//{
//piece: SidePiece!
//}

//"""
//⭕ The piece indesign of a side. A piece is identified by an id (emtpy=default)).
//"""
//type SidePiece
//{
//id: String!
//  type: SidePieceType!
//}

//"""🧩 The type indesign of a piece of a side."""
//type SidePieceType
//{
//port: Port
//}

//enum LoadLocalKitError
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_READ_KIT
//}

//type DesignToSceneFromLocalKitResponse
//{
//    scene: Scene
//  error: DesignToSceneFromLocalKitResponseError
//}

//"""🌆 A scene is a collection of objects."""
//type Scene
//{
//    objects: [Object]!
//  design: Design
//}

//"""
//🗿 An object is a piece with a plane and a parent object (unless the piece is a root).
//"""
//type Object
//{
//    plane: Plane!
//  piece: Piece
//  parent: Object
//}

//type DesignToSceneFromLocalKitResponseError
//{
//    code: DesignToSceneFromLocalKitResponseErrorCode!
//  message: String
//}

//enum DesignToSceneFromLocalKitResponseErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_READ_KIT
//  DESIGN_DOES_NOT_EXIST
//}

//"""🏙️ A design is identified by a name and optional variant."""
//input DesignIdInput
//{
//    name: String!
//  variant: String = ""
//}

//type Mutation
//{
//    createLocalKit(directory: String!, kitInput: KitInput!): CreateLocalKitMutation
//  updateLocalKitMetadata(directory: String!, kitMetadataInput: KitMetadataInput!): UpdateLocalKitMetadataMutation
//  deleteLocalKit(directory: String!): DeleteLocalKitMutation
//  addTypeToLocalKit(directory: String!, typeInput: TypeInput!): AddTypeToLocalKitMutation
//  removeTypeFromLocalKit(directory: String!, typeId: TypeIdInput!): RemoveTypeFromLocalKitMutation
//  addDesignToLocalKit(directory: String!, designInput: DesignInput!): AddDesignToLocalKitMutation
//  removeDesignFromLocalKit(directory: String!, designId: DesignIdInput!): RemoveDesignFromLocalKitMutation
//}

//type CreateLocalKitMutation
//{
//    kit: Kit
//  error: CreateLocalKitError
//}

//type CreateLocalKitError
//{
//    code: CreateLocalKitErrorCode!
//  message: String
//}

//enum CreateLocalKitErrorCode
//{
//    DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_ALREADY_CONTAINS_A_KIT
//  NO_PERMISSION_TO_CREATE_DIRECTORY
//  NO_PERMISSION_TO_CREATE_KIT
//  KIT_INPUT_IS_INVALID
//}

//"""🗃️ A kit is a collection of types and designs."""
//input KitInput
//{
//    name: String!
//  description: String
//  icon: String
//  url: String
//  homepage: String
//  types: [TypeInput!]
//    designs: [DesignInput!]
//}

//"""
//🧩 A type is a reusable element that can be connected with other types over ports.
//"""
//input TypeInput
//{
//    name: String!
//  description: String
//  icon: String
//  variant: String = ""
//  unit: String!
//  representations: [RepresentationInput!]!
//  ports: [PortInput!]!
//  qualities: [QualityInput!]
//}

//"""
//💾 A representation is a link to a file that describes a type for a certain level of detail and tags.
//"""
//input RepresentationInput
//{
//    url: String!
//  mime: String
//  lod: String
//  tags: [String!]
//}

//"""
//🔌 A port is a conceptual connection point (with a direction) of a type.
//"""
//input PortInput
//{
//    id: String = ""
//  point: PointInput!
//  direction: VectorInput!
//  locators: [LocatorInput!]
//}

//"""✖️ A 3d-point (xyz) of floating point numbers."""
//input PointInput
//{
//    x: Float = 0
//  y: Float = 0
//  z: Float = 0
//}

//"""➡️ A 3d-vector (xyz) of floating point numbers."""
//input VectorInput
//{
//    x: Float = 0
//  y: Float = 0
//  z: Float = 0
//}

//"""🗺️ A locator is meta-data for grouping ports."""
//input LocatorInput
//{
//    group: String!
//  subgroup: String
//}

//"""📏 A quality is meta-data for decision making."""
//input QualityInput
//{
//    name: String!
//  value: String
//  unit: String
//  definition: String
//}

//"""🏙️ A design is a collection of pieces that are connected."""
//input DesignInput
//{
//    name: String!
//  description: String
//  icon: String
//  variant: String = ""
//  unit: String!
//  pieces: [PieceInput!]!
//  connections: [ConnectionInput!]!
//  qualities: [QualityInput!]
//}

//"""⭕ A piece is a 3d-instance of a type in a design."""
//input PieceInput
//{
//    id: String!
//  type: TypeIdInput!
//  root: PieceRootInput = null
//  diagram: PieceDiagramInput!
//}

//"""🧩 A type is identified by a name and variant (empty=default)."""
//input TypeIdInput
//{
//    name: String!
//  variant: String = ""
//}

//"""🌱 The root indesign of a piece."""
//input PieceRootInput
//{
//    plane: PlaneInput!
//}

//"""◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""
//input PlaneInput
//{
//    origin: PointInput!
//  xAxis: VectorInput!
//  yAxis: VectorInput!
//}

//"""✏️ The diagram indesign of a piece."""
//input PieceDiagramInput
//{
//    point: ScreenPointInput!
//}

//"""📺 A 2d-point (xy) of integers in screen plane."""
//input ScreenPointInput
//{
//    x: Int = 0
//  y: Int = 0
//}

//"""🖇️ A connection between two pieces of a design."""
//input ConnectionInput
//{
//    connecting: SideInput!
//  connected: SideInput!
//  offset: Float = 0
//  rotation: Float = 0
//}

//"""🧱 A side of a piece in a connection."""
//input SideInput
//{
//    piece: SidePieceInput!
//}

//"""
//⭕ The piece indesign of a side. A piece is identified by an id (emtpy=default)).
//"""
//input SidePieceInput
//{
//    id: String!
//  type: SidePieceTypeInput = null
//}

//"""🧩 The type indesign of a piece of a side."""
//input SidePieceTypeInput
//{
//    port: PortIdInput = null
//}

//"""🔌 A port is identified by an id (emtpy=default))."""
//input PortIdInput
//{
//    id: String = ""
//}

//type UpdateLocalKitMetadataMutation
//{
//    kit: Kit
//  error: UpdateLocalKitMetadataError
//}

//type UpdateLocalKitMetadataError
//{
//    code: UpdateLocalKitMetadataErrorCode!
//  message: String
//}

//enum UpdateLocalKitMetadataErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_UPDATE_KIT
//  KIT_METADATA_IS_INVALID
//}

//"""🗃️ Meta-data of a kit."""
//input KitMetadataInput
//{
//    name: String
//  description: String
//  icon: String
//  url: String
//  homepage: String
//}

//type DeleteLocalKitMutation
//{
//    error: DeleteLocalKitError
//}

//enum DeleteLocalKitError
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_DELETE_KIT
//}

//type AddTypeToLocalKitMutation
//{
//    type: Type
//  error: AddTypeToLocalKitError
//}

//type AddTypeToLocalKitError
//{
//    code: AddTypeToLocalKitErrorCode!
//  message: String
//}

//enum AddTypeToLocalKitErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_MODIFY_KIT
//  TYPE_INPUT_IS_INVALID
//}

//type RemoveTypeFromLocalKitMutation
//{
//    error: RemoveTypeFromLocalKitError
//}

//type RemoveTypeFromLocalKitError
//{
//    code: RemoveTypeFromLocalKitErrorCode!
//  message: String
//}

//enum RemoveTypeFromLocalKitErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_MODIFY_KIT
//  TYPE_DOES_NOT_EXIST
//  DESIGN_DEPENDS_ON_TYPE
//}

//type AddDesignToLocalKitMutation
//{
//    design: Design
//  error: AddDesignToLocalKitError
//}

//type AddDesignToLocalKitError
//{
//    code: AddDesignToLocalKitErrorCode!
//  message: String
//}

//enum AddDesignToLocalKitErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_MODIFY_KIT
//  DESIGN_INPUT_IS_INVALID
//}

//type RemoveDesignFromLocalKitMutation
//{
//    error: RemoveDesignFromLocalKitError
//}

//type RemoveDesignFromLocalKitError
//{
//    code: RemoveDesignFromLocalKitErrorCode!
//  message: String
//}

//enum RemoveDesignFromLocalKitErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_MODIFY_KIT
//  DESIGN_DOES_NOT_EXIST
//}
#endregion

#region Utility

public static class Generator
{
    public static string GenerateRandomId(int seed)
    {
        var adjectives = Resources.adjectives.Deserialize<List<string>>();
        var animals = Resources.animals.Deserialize<List<string>>();
        var random = new Random(seed);
        var adjective = adjectives[random.Next(adjectives.Count)];
        var animal = animals[random.Next(animals.Count)];
        var number = random.Next(0, 999);
        adjective = char.ToUpper(adjective[0]) + adjective.Substring(1);
        animal = char.ToUpper(animal[0]) + animal.Substring(1);
        return $"{adjective}{animal}{number}";
    }
}

public static class MimeParser
{
    public static string ParseFromUrl(string url)
    {
        var mimes = new Dictionary<string, string>
        {
            {".stl", "model/stl"},
            {".obj", "model/obj"},
            {".glb", "model/gltf-binary"},
            {".gltf", "model/gltf+json"},
            {".3dm", "model/vnd.3dm"},
            {".png", "image/png"},
            {".jpg", "image/jpeg"},
            {".jpeg", "image/jpeg"},
            {".svg", "image/svg+xml"},
            {".pdf", "application/pdf"},
            {".zip", "application/zip"},
            {".json", "application/json"},
            {".csv", "text/csv"},
            {".txt", "text/plain"}
        };
        try
        {
            return mimes[System.IO.Path.GetExtension(url)];
        }
        catch (KeyNotFoundException)
        {
            return "application/octet-stream";
        }
    }
}

#endregion

#region Models

public class IdAttribute : Attribute
{
}

public class StringListLengthAttribute : ValidationAttribute
{
    private readonly int _maxLength;

    public StringListLengthAttribute(int maxLength)
    {
        _maxLength = maxLength;
    }

    protected override ValidationResult IsValid(object value, ValidationContext validationContext)
    {
        var list = value as List<string>;
        if (list != null)
        {
            foreach (var str in list)
            {
                if (str.Length > _maxLength)
                {
                    return new ValidationResult($"Each string in the list must be at most {_maxLength} characters long.");
                }
            }
        }
        return ValidationResult.Success;
    }
}
public interface IDeepCloneable<T>
{
    T DeepClone();
}

public abstract record Base<T>() : IDeepCloneable<T> where T : Base<T>
{
    public T DeepClone()
    {
        return this.DeepClone() as T;
    }

    public (bool, List<string>) validate()
    {
        var validationErrors = new List<ValidationResult>();
        var valid = Validator.TryValidateObject(this, new ValidationContext(this), validationErrors,
            validateAllProperties: true);
        return (valid, validationErrors.Select(e => e.ErrorMessage).ToList());

    }

public record Representation() : Base<Representation>
{
    [Id]
    [StringLength(Constants.UrlLengthLimit)]
    [Required(ErrorMessage= "Unique Resource Locator (URL) is required.")]
    public string Url { get; set; } = "";
    [StringLength(Constants.NameLengthLimit)]
    [Required(ErrorMessage = "Multipurpose Internet Mail Extensions (MIME) is required.")]
    public string Mime { get; set; } = "";
    [StringLength(Constants.NameLengthLimit)]
    public string Lod { get; set; } = "";
    [StringListLength(Constants.NameLengthLimit)]
    public List<string> Tags { get; set; } = new();

}

public record Locator() : Base<Locator>
{ 
    [Required]
    public string Group { get; set; } = "";
    public string Subgroup { get; set; } = "";

}

public record ScreenPoint() : Base<ScreenPoint>
{
    public int X { get; set; } = 0;
    public int Y { get; set; } = 0;

}

public record Point() : Base<Point>
{
    public float X { get; set; } = 0;
    public float Y { get; set; } = 0;
    public float Z { get; set; } = 0;

    public bool IsZero()
    {
        return X == 0 && Y == 0 && Z == 0;
    }
}

public record Vector() : Base<Vector>
{
    public float X { get; set; } = 0;
    public float Y { get; set; } = 0;
    public float Z { get; set; } = 0;

    public bool IsZero()
    {
        return X == 0 && Y == 0 && Z == 0;
    }
}

public record Plane() : Base<Plane>
{
    public Point Origin { get; set; } = new();
    public Vector XAxis { get; set; } = new();
    public Vector YAxis { get; set; } = new();

}

public record Port() : Base<Port>
{
    public string Id { get; set; } = "";
    public Point Point { get; set; } = new();
    public Vector Direction { get; set; } = new();
    public List<Locator> Locators { get; set; } = new();
}

public record PortId() : Base<PortId>
{
    public string Id { get; set; } = "";

}


public record Quality() : Base<Quality>
{
    public string Name { get; set; } = "";
    public string Value { get; set; } = "";
    public string Unit { get; set; } = "";
    public string Definition { get; set; } = "";

}

public record Type() : Base<Type>
{
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Variant { get; set; } = "";
    public string Unit { get; set; } = "";
    public List<Representation> Representations { get; set; } = new();
    public List<Port> Ports { get; set; } = new();
    public List<Quality> Qualities { get; set; } = new();

}

public record TypeId() : Base<TypeId>
{
    public string Name { get; set; } = "";
    public string Variant { get; set; } = "";

}

public record PieceRoot() : Base<PieceRoot>
{
    public Plane Plane { get; set; } = new();
}

public record PieceDiagram() : Base<PieceDiagram>
{
    public ScreenPoint Point { get; set; } = new();

}

public record Piece() : Base<Piece>
{
    public string Id { get; set; } = "";
    public TypeId Type { get; set; } = new();
    public PieceRoot? Root { get; set; } = null;
    public PieceDiagram Diagram { get; set; } = new();

}

public record PieceId() : Base<PieceId>
{
    public string Id { get; set; } = "";

}

public record SidePieceType() : Base<SidePieceType>
{
    public PortId Port { get; set; } = new();

}

public record SidePiece() : Base<SidePiece>
{
    public string Id { get; set; } = "";
    public SidePieceType Type { get; set; } = new();

    
}

public record Side() : Base<Side>
{
    public SidePiece Piece { get; set; } = new();

}


public record Connection() : Base<Connection>
{
    public Side Connected { get; set; } = new();
    public Side Connecting { get; set; } = new();
    public float Offset { get; set; } = 0;
    public float Rotation { get; set; } = 0;

}

public record Design() : Base<Design>
{
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Variant { get; set; } = "";
    public string Unit { get; set; } = "";
    public List<Piece> Pieces { get; set; } = new();
    public List<Connection> Connections { get; set; } = new();
    public List<Quality> Qualities { get; set; } = new();

    public Design Flatten(Type[] types = null)
    {
        Design flattenedDesign = this.DeepClone();
        if (Pieces.Count <= 1 || Connections.Count == 0)
            return flattenedDesign;
        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var piece in Pieces)
            graph.AddVertex(piece.Id);
        foreach (var connection in Connections)
            graph.AddEdge(new Edge<string>(connection.Connected.Piece.Id, connection.Connecting.Piece.Id));
        var root = Pieces.First(p => p.Root != null) ?? Pieces.First();
        var components = new Dictionary<string, int>();
        graph.ConnectedComponents(components);
        return flattenedDesign;

    }
}

public record DesignId() : Base<DesignId>
{
    public string Name { get; set; } = "";
    public string Variant { get; set; } = "";

}


public record Kit() : Base<Kit>
{
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Url { get; set; } = "";
    public string Homepage { get; set; } = "";
    public List<Type> Types { get; set; } = new();
    public List<Design> Designs { get; set; } = new();

}

public record KitProps : Base<KitProps>
{
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Url { get; set; }
    public string? Homepage { get; set; }
}

#endregion

public static class Serializer
{
    public static string Serialize(this object obj)
    {
        return JsonConvert.SerializeObject(
            obj, Formatting.Indented, new JsonSerializerSettings
            {
                ContractResolver = new CamelCasePropertyNamesContractResolver()
            });
    }
}

public static class Deserializer
{
    public static T Deserialize<T>(this string json)
    {
        return JsonConvert.DeserializeObject<T>(
            json, new JsonSerializerSettings
            {
                ContractResolver = new CamelCasePropertyNamesContractResolver()
            });
    }
}

#region Api

public class LoadLocalKitResponse
{
    public Kit? Kit { get; set; }
    public string? Error { get; set; }
}

public class LoadLocalKitResponseContainer
{
    public LoadLocalKitResponse LoadLocalKit { get; set; }
}

public enum CreateLocalKitErrorCode
{
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_ALREADY_CONTAINS_A_KIT,
    NO_PERMISSION_TO_CREATE_DIRECTORY,
    NO_PERMISSION_TO_CREATE_KIT,
    KIT_INPUT_IS_INVALID
}

public class CreateLocalKitError
{
    public CreateLocalKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class CreateLocalKitResponse
{
    public Kit? Kit { get; set; }
    public CreateLocalKitError? Error { get; set; }
}

public class CreateLocalKitResponseContainer
{
    public CreateLocalKitResponse CreateLocalKit { get; set; }
}

public enum UpdateLocalKitMetadataErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_UPDATE_KIT,
    KIT_METADATA_IS_INVALID
}

public class UpdateLocalKitMetadataError
{
    public UpdateLocalKitMetadataErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class UpdateLocalKitMetadataResponse
{
    public KitMetadata? Kit { get; set; }
    public UpdateLocalKitMetadataError? Error { get; set; }
}

public class UpdateLocalKitMetadataResponseContainer
{
    public UpdateLocalKitMetadataResponse UpdateLocalKitMetadata { get; set; }
}

public enum DeleteLocalKitError
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_DELETE_KIT
}

public class DeleteLocalKitResponse
{
    public DeleteLocalKitError? Error { get; set; }
}

public class DeleteLocalKitResponseContainer
{
    public DeleteLocalKitResponse DeleteLocalKit { get; set; }
}

public enum AddTypeToLocalKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    TYPE_INPUT_IS_INVALID
}

public class AddTypeToLocalKitError
{
    public AddTypeToLocalKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class AddTypeToLocalKitResponse
{
    public Type? Type { get; set; }
    public AddTypeToLocalKitError? Error { get; set; }
}

public class AddTypeToLocalKitResponseContainer
{
    public AddTypeToLocalKitResponse AddTypeToLocalKit { get; set; }
}

public enum RemoveTypeFromLocalKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    TYPE_DOES_NOT_EXIST,
    DESIGN_DEPENDS_ON_TYPE
}

public class RemoveTypeFromLocalKitError
{
    public RemoveTypeFromLocalKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class RemoveTypeFromLocalKitResponse
{
    public RemoveTypeFromLocalKitError? Error { get; set; }
}

public class RemoveTypeFromLocalKitResponseContainer
{
    public RemoveTypeFromLocalKitResponse RemoveTypeFromLocalKit { get; set; }
}

public enum AddDesignToLocalKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    DESIGN_INPUT_IS_INVALID
}

public class AddDesignToLocalKitError
{
    public AddDesignToLocalKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class AddDesignToLocalKitResponse
{
    public Design? Design { get; set; }
    public AddDesignToLocalKitError? Error { get; set; }
}

public class AddDesignToLocalKitResponseContainer
{
    public AddDesignToLocalKitResponse AddDesignToLocalKit { get; set; }
}

public enum RemoveDesignFromLocalKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    DESIGN_DOES_NOT_EXIST
}

public class RemoveDesignFromLocalKitError
{
    public RemoveDesignFromLocalKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class RemoveDesignFromLocalKitResponse
{
    public RemoveDesignFromLocalKitError? Error { get; set; }
}

public class RemoveDesignFromLocalKitResponseContainer
{
    public RemoveDesignFromLocalKitResponse RemoveDesignFromLocalKit { get; set; }
}

public enum DesignToSceneFromLocalKitResponseErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_READ_KIT,
    DESIGN_DOES_NOT_EXIST
}

public class DesignToSceneFromLocalKitResponseError
{
    public DesignToSceneFromLocalKitResponseErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class DesignToSceneFromLocalKitResponse
{
    public Scene? Scene { get; set; }
    public DesignToSceneFromLocalKitResponseError? Error { get; set; }
}

public class DesignToSceneFromLocalKitResponseContainer
{
    public DesignToSceneFromLocalKitResponse DesignToSceneFromLocalKit { get; set; }
}

public class Api : ICloneable
{
    public Api()
    {
        Endpoint = "http://127.0.0.1:5052/graphql";
        Token = "";
        Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
    }

    public Api(string endpoint, string token)
    {
        Endpoint = endpoint;
        Token = token;
        Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
        Client.HttpClient.DefaultRequestHeaders.Add("Authorization", $"Bearer {Token}");
    }

    public GraphQLHttpClient Client { get; set; }
    public string Endpoint { get; set; }
    public string Token { get; set; }

    public object Clone()
    {
        return new Api(Endpoint, Token);
    }

    public override string ToString()
    {
        return $"Api(Endpoint: {Endpoint}, Token: {Token})";
    }

    public LoadLocalKitResponse? LoadLocalKit(string directory)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.loadLocalKit,
            OperationName = "LoadLocalKit",
            Variables = new { directory }
        };
        var response = Client.SendQueryAsync<LoadLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.LoadLocalKit;
    }

    public CreateLocalKitResponse? CreateLocalKit(string directory, Kit kit)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.createLocalKit,
            OperationName = "CreateLocalKit",
            Variables = new { directory, kit }
        };
        var response = Client.SendQueryAsync<CreateLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.CreateLocalKit;
    }

    public UpdateLocalKitMetadataResponse? UpdateLocalKitMetadata(string directory, KitMetadata kit)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.updateLocalKitMetadata,
            OperationName = "UpdateLocalKitMetadata",
            Variables = new { directory, kit }
        };
        var response = Client.SendQueryAsync<UpdateLocalKitMetadataResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.UpdateLocalKitMetadata;
    }

    public DeleteLocalKitResponse? DeleteLocalKit(string directory)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.deleteLocalKit,
            OperationName = "DeleteLocalKit",
            Variables = new { directory }
        };
        var response = Client.SendQueryAsync<DeleteLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.DeleteLocalKit;
    }

    public AddTypeToLocalKitResponse? AddTypeToLocalKit(string directory, Type type)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.addTypeToLocalKit,
            OperationName = "AddTypeToLocalKit",
            Variables = new { directory, type }
        };
        var response = Client.SendQueryAsync<AddTypeToLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.AddTypeToLocalKit;
    }

    public RemoveTypeFromLocalKitResponse? RemoveTypeFromLocalKit(string directory, TypeId type)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.removeTypeFromLocalKit,
            OperationName = "RemoveTypeFromLocalKit",
            Variables = new { directory, type }
        };
        var response = Client.SendQueryAsync<RemoveTypeFromLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.RemoveTypeFromLocalKit;
    }

    public AddDesignToLocalKitResponse? AddDesignToLocalKit(string directory, Design design)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.addDesignToLocalKit,
            OperationName = "AddDesignToLocalKit",
            Variables = new { directory, design }
        };
        var response = Client.SendQueryAsync<AddDesignToLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.AddDesignToLocalKit;
    }

    public RemoveDesignFromLocalKitResponse? RemoveDesignFromLocalKit(string directory, DesignId design)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.removeDesignFromLocalKit,
            OperationName = "RemoveDesignFromLocalKit",
            Variables = new { directory, design }
        };
        var response = Client.SendQueryAsync<RemoveDesignFromLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.RemoveDesignFromLocalKit;
    }

    public DesignToSceneFromLocalKitResponse? DesignToSceneFromLocalKit(string directory,
        DesignId design)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.designToSceneFromLocalKit,
            OperationName = "DesignToSceneFromLocalKit",
            Variables = new { directory, design }
        };
        var response = Client.SendQueryAsync<DesignToSceneFromLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.DesignToSceneFromLocalKit;
    }
}

#endregion
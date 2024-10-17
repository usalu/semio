using System;
using System.Collections.Generic;
using System.Linq;
using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.Newtonsoft;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using Semio;

// TODO: Replace GetHashcode() with a proper hash function.
// TODO: Add logging mechanism to all API calls if they fail.
// TODO: Add a more detailed message system when a model is invalid.

#region Constants

public static class Constants
{
    public const int NameLengthLimit = 64;
    public const int IdLengthLimit = 128;
    public const int UrlLengthLimit = 2048;
    public const int DescriptionLengthLimit = 4096;
    public const string Release = "r24.10-3";
    public const int EnginePort = 24103;
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

public interface IDeepCloneable<T>
{
    T DeepClone();
}

public interface IEntity
{
    string ToString();
    bool IsInvalid();
}

public class Representation : IDeepCloneable<Representation>, IEntity
{
    public Representation()
    {
        Url = "";
        Mime = "";
        Lod = "";
        Tags = new List<string>();
    }

    public string Url { get; set; }
    public string Mime { get; set; }
    public string Lod { get; set; }
    public List<string> Tags { get; set; }

    public Representation DeepClone()
    {
        return new Representation
        {
            Url = Url,
            Mime = Mime,
            Lod = Lod,
            Tags = new List<string>(Tags)
        };
    }

    public override string ToString()
    {
        return $"Representation(Url:{Url})";
    }

    public bool IsInvalid()
    {
        return Url == "" || Mime == "";
    }
}

public class Locator : IDeepCloneable<Locator>, IEntity
{
    public Locator()
    {
        Group = "";
        Subgroup = "";
    }

    public string Group { get; set; }
    public string Subgroup { get; set; }

    public Locator DeepClone()
    {
        return new Locator
        {
            Group = Group,
            Subgroup = Subgroup
        };
    }

    public override string ToString()
    {
        return $"Locator(Group:{Group}" + (Subgroup != "" ? $",Subgroup:{Subgroup})" : ")");
    }

    public bool IsInvalid()
    {
        return Group == "";
    }
}

public class ScreenPoint : IDeepCloneable<ScreenPoint>, IEntity
{
    public ScreenPoint()
    {
        X = 0;
        Y = 0;
    }

    public int X { get; set; }
    public int Y { get; set; }

    public ScreenPoint DeepClone()
    {
        return new ScreenPoint
        {
            X = X,
            Y = Y
        };
    }

    public override string ToString()
    {
        return $"Point(X:{X},Y:{Y})";
    }

    public bool IsInvalid()
    {
        return false;
    }
}

public class Point : IDeepCloneable<Point>, IEntity
{
    public Point()
    {
        X = 0;
        Y = 0;
        Z = 0;
    }

    public float X { get; set; }
    public float Y { get; set; }
    public float Z { get; set; }

    public Point DeepClone()
    {
        return new Point
        {
            X = X,
            Y = Y,
            Z = Z
        };
    }

    public override string ToString()
    {
        return $"Point(X:{X},Y:{Y},Z:{Z})";
    }

    public bool IsInvalid()
    {
        return false;
    }

    public bool IsZero()
    {
        return X == 0 && Y == 0 && Z == 0;
    }
}

public class Vector : IDeepCloneable<Vector>, IEntity
{
    public Vector()
    {
        X = 0;
        Y = 0;
        Z = 0;
    }

    public float X { get; set; }
    public float Y { get; set; }
    public float Z { get; set; }

    public Vector DeepClone()
    {
        return new Vector
        {
            X = X,
            Y = Y,
            Z = Z
        };
    }

    public override string ToString()
    {
        return $"Vector(X:{X},Y:{Y},Z:{Z})";
    }

    public bool IsInvalid()
    {
        return false;
    }

    public bool IsZero()
    {
        return X == 0 && Y == 0 && Z == 0;
    }
}

public class Plane : IDeepCloneable<Plane>, IEntity
{
    public Plane()
    {
        Origin = new Point();
        XAxis = new Vector();
        YAxis = new Vector();
    }

    public Point Origin { get; set; }
    public Vector XAxis { get; set; }
    public Vector YAxis { get; set; }

    public Plane DeepClone()
    {
        return new Plane
        {
            Origin = Origin.DeepClone(),
            XAxis = XAxis.DeepClone(),
            YAxis = YAxis.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Plane(Origin:{Origin},XAxis:{XAxis},YAxis: {YAxis})";
    }

    public bool IsInvalid()
    {
        // TODO: Check if axes are normalized and orthogonal.
        return Origin.IsZero() && XAxis.IsZero() && YAxis.IsZero();
    }
}

public class Port : IDeepCloneable<Port>, IEntity
{
    public Port()
    {
        Id = "";
        Point = new Point();
        Direction = new Vector();
        Locators = new List<Locator>();
    }

    public string Id { get; set; }
    public Point Point { get; set; }
    public Vector Direction { get; set; }
    public List<Locator> Locators { get; set; }

    public Port DeepClone()
    {
        return new Port
        {
            Id = Id,
            Point = Point.DeepClone(),
            Direction = Direction.DeepClone(),
            Locators = new List<Locator>(Locators.Select(s => s.DeepClone()))
        };
    }

    public override string ToString()
    {
        return "Port(" + (Id != "" ? $"Id:{Id})" : ")");
    }

    public bool IsInvalid()
    {
        return Id == "" || Point.IsInvalid() || Direction.IsInvalid() || Locators.Any(s => s.IsInvalid());
    }
}

public class PortId : IDeepCloneable<PortId>, IEntity
{
    public PortId()
    {
        Id = "";
    }

    public string Id { get; set; }

    public PortId DeepClone()
    {
        return new PortId
        {
            Id = Id
        };
    }

    public override string ToString()
    {
        return "Port(" + (Id != "" ? $"Id:{Id})" : ")");
    }

    public bool IsInvalid()
    {
        return false;
    }
}


public class Quality : IDeepCloneable<Quality>, IEntity
{
    public Quality()
    {
        Name = "";
        Value = "";
        Unit = "";
        Definition = "";
    }

    public string Name { get; set; }
    public string Value { get; set; }
    public string Unit { get; set; }
    public string Definition { get; set; }

    public Quality DeepClone()
    {
        return new Quality
        {
            Name = Name,
            Value = Value,
            Unit = Unit,
            Definition = Definition
        };
    }

    public override string ToString()
    {
        return $"Quality(Name:{Name})";
    }

    public bool IsInvalid()
    {
        return Name == "";
    }
}

public class Type : IDeepCloneable<Type>, IEntity
{
    public Type()
    {
        Name = "";
        Description = "";
        Icon = "";
        Variant = "";
        Unit = "";
        Representations = new List<Representation>();
        Ports = new List<Port>();
        Qualities = new List<Quality>();
    }

    public string Name { get; set; }
    public string Description { get; set; }
    public string Icon { get; set; }
    public string Variant { get; set; }
    public string Unit { get; set; }
    public List<Representation> Representations { get; set; }
    public List<Port> Ports { get; set; }
    public List<Quality> Qualities { get; set; }

    public Type DeepClone()
    {
        return new Type
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Variant = Variant,
            Unit = Unit,
            Representations = new List<Representation>(Representations.Select(r => r.DeepClone())),
            Ports = new List<Port>(Ports.Select(p => p.DeepClone())),
            Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Type(Name:{Name}" + (Variant != "" ? $",Variant:{Variant})" : ")");
    }

    public bool IsInvalid()
    {
        return Name == "" || Unit == "" || Representations.Any(r => r.IsInvalid()) || Ports.Any(p => p.IsInvalid()) ||
               Qualities.Any(q => q.IsInvalid());
    }
}

public class TypeId : IDeepCloneable<TypeId>, IEntity
{
    public TypeId()
    {
        Name = "";
        Variant = "";
    }

    public string Name { get; set; }
    public string Variant { get; set; }

    public TypeId DeepClone()
    {
        return new TypeId
        {
            Name = Name,
            Variant = Variant
        };
    }

    public override string ToString()
    {
        return $"Type(Name:{Name}" + (Variant != "" ? $",Variant:{Variant})" : ")");
    }

    public bool IsInvalid()
    {
        return Name == "";
    }
}

public class PieceRoot : IDeepCloneable<PieceRoot>, IEntity
{
    public PieceRoot()
    {
        Plane = new Plane();
    }

    public Plane Plane { get; set; }

    public PieceRoot DeepClone()
    {
        return new PieceRoot
        {
            Plane = Plane.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Root({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Plane.IsInvalid();
    }
}

public class PieceDiagram : IDeepCloneable<PieceDiagram>, IEntity
{
    public PieceDiagram()
    {
        Point = new ScreenPoint();
    }

    public ScreenPoint Point { get; set; }

    public PieceDiagram DeepClone()
    {
        return new PieceDiagram
        {
            Point = Point.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Diagram({Point})";
    }

    public bool IsInvalid()
    {
        return Point.IsInvalid();
    }
}

public class Piece : IDeepCloneable<Piece>, IEntity
{
    public Piece()
    {
        Id = "";
        Type = new TypeId();
        Root = null;
        Diagram = new PieceDiagram();
    }

    public string Id { get; set; }
    public TypeId Type { get; set; }
    public PieceRoot? Root { get; set; }
    public PieceDiagram Diagram { get; set; }

    public Piece DeepClone()
    {
        return new Piece
        {
            Id = Id,
            Type = Type.DeepClone(),
            Root = Root?.DeepClone(),
            Diagram = Diagram.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Piece(Id:{Id})";
    }

    public bool IsInvalid()
    {
        return Id == "" || Type.IsInvalid() || (Root?.IsInvalid() ?? false) || Diagram.IsInvalid();
    }
}

public class PieceId : IDeepCloneable<PieceId>, IEntity
{
    public PieceId()
    {
        Id = "";
    }

    public string Id { get; set; }

    public PieceId DeepClone()
    {
        return new PieceId
        {
            Id = Id
        };
    }

    public override string ToString()
    {
        return $"Piece(Id:{Id})";
    }

    public bool IsInvalid()
    {
        return Id == "";
    }
}

public class SidePieceType : IDeepCloneable<SidePieceType>, IEntity
{
    public SidePieceType()
    {
        Port = new PortId();
    }

    public PortId Port { get; set; }

    public SidePieceType DeepClone()
    {
        return new SidePieceType
        {
            Port = Port.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Type({Port})";
    }

    public bool IsInvalid()
    {
        return Port.IsInvalid();
    }
}

public class SidePiece : IDeepCloneable<SidePiece>, IEntity
{
    public SidePiece()
    {
        Id = "";
        Type = new SidePieceType();
    }

    public string Id { get; set; }
    public SidePieceType Type { get; set; }

    public SidePiece DeepClone()
    {
        return new SidePiece
        {
            Id = Id,
            Type = Type.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Piece(Id:{Id}" + (Type.Port.Id != "" ? $",{Type})" : ")");
    }

    public bool IsInvalid()
    {
        return Id == "" || Type.IsInvalid();
    }
}

public class Side : IDeepCloneable<Side>, IEntity
{
    public Side()
    {
        Piece = new SidePiece();
    }

    public SidePiece Piece { get; set; }

    public Side DeepClone()
    {
        return new Side
        {
            Piece = Piece.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Side({Piece})";
    }

    public bool IsInvalid()
    {
        return Piece.IsInvalid();
    }
}


public class Connection : IDeepCloneable<Connection>, IEntity
{
    public Connection()
    {
        Connected = new Side();
        Connecting = new Side();
        Offset = 0;
        Rotation = 0;
    }

    public Side Connected { get; set; }
    public Side Connecting { get; set; }
    public float Offset { get; set; }
    public float Rotation { get; set; }

    public Connection DeepClone()
    {
        return new Connection
        {
            Connected = Connected.DeepClone(),
            Connecting = Connecting.DeepClone(),
            Offset = Offset,
            Rotation = Rotation
        };
    }

    public override string ToString()
    {
        return $"Connection(Connected({Connected}),Connecting({Connecting}),Offset:{Offset},Rotation:{Rotation})";
    }

    public bool IsInvalid()
    {
        return Connecting.IsInvalid() || Connected.IsInvalid() || Connecting.Piece.Id == Connected.Piece.Id;
    }
}

public class Design : IDeepCloneable<Design>, IEntity
{
    public Design()
    {
        Name = "";
        Description = "";
        Icon = "";
        Variant = "";
        Unit = "";
        Pieces = new List<Piece>();
        Connections = new List<Connection>();
        Qualities = new List<Quality>();
    }

    public string Name { get; set; }
    public string Description { get; set; }
    public string Icon { get; set; }
    public string Variant { get; set; }
    public string Unit { get; set; }
    public List<Piece> Pieces { get; set; }
    public List<Connection> Connections { get; set; }
    public List<Quality> Qualities { get; set; }

    public Design DeepClone()
    {
        return new Design
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Variant = Variant,
            Unit = Unit,
            Pieces = new List<Piece>(Pieces.Select(p => p.DeepClone())),
            Connections = new List<Connection>(Connections.Select(a => a.DeepClone())),
            Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Design(Name:{Name}" + (Variant != "" ? $",Variant: {Variant})" : ")");
    }

    public bool IsInvalid()
    {
        return Name == "" || Unit == "" || Pieces.Any(p => p.IsInvalid()) || Connections.Any(a => a.IsInvalid()) ||
               Qualities.Any(q => q.IsInvalid());
    }
}

public class DesignId : IDeepCloneable<DesignId>, IEntity
{
    public DesignId()
    {
        Name = "";
        Variant = "";
    }

    public string Name { get; set; }
    public string Variant { get; set; }

    public DesignId DeepClone()
    {
        return new DesignId
        {
            Name = Name,
            Variant = Variant
        };
    }

    public override string ToString()
    {
        return $"Design(Name:{Name}" + (Variant != "" ? $",Variant:{Variant})" : ")");
    }

    public bool IsInvalid()
    {
        return Name == "";
    }
}

public class ObjectPieceType : IDeepCloneable<ObjectPieceType>, IEntity
{
    public ObjectPieceType()
    {
        Representations = new List<Representation>();
    }

    public List<Representation> Representations { get; set; }

    public ObjectPieceType DeepClone()
    {
        return new ObjectPieceType
        {
            Representations = new List<Representation>(Representations.Select(f => f.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Type({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Representations.Any(r => r.IsInvalid());
    }
}

public class ObjectPiece : IDeepCloneable<ObjectPiece>, IEntity
{
    public ObjectPiece()
    {
        Id = "";
        Type = new ObjectPieceType();
    }

    public string Id { get; set; }
    public ObjectPieceType Type { get; set; }

    public ObjectPiece DeepClone()
    {
        return new ObjectPiece
        {
            Id = Id,
            Type = Type.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Piece(Id:{Id})";
    }

    public bool IsInvalid()
    {
        return Id == "" || Type.IsInvalid();
    }
}


public class ObjectParent : IDeepCloneable<ObjectParent>, IEntity
{
    public ObjectParent()
    {
        Piece = new PieceId();
    }

    public PieceId Piece { get; set; }

    public ObjectParent DeepClone()
    {
        return new ObjectParent
        {
            Piece = Piece.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Parent({Piece})";
    }

    public bool IsInvalid()
    {
        return Piece.IsInvalid();
    }
}

public class Object : IDeepCloneable<Object>, IEntity
{
    public Object()
    {
        Piece = new ObjectPiece();
        Plane = new Plane();
        Parent = null;
    }

    public ObjectPiece Piece { get; set; }
    public Plane Plane { get; set; }
    public ObjectParent? Parent { get; set; }

    public Object DeepClone()
    {
        return new Object
        {
            Piece = Piece.DeepClone(),
            Plane = Plane.DeepClone(),
            Parent = Parent?.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Object({Piece})";
    }

    public bool IsInvalid()
    {
        return Piece.IsInvalid() || Plane.IsInvalid() || (Parent?.IsInvalid() ?? false);
    }
}

public class Scene : IDeepCloneable<Scene>, IEntity
{
    public Scene()
    {
        Design = new DesignId();
        Objects = new List<Object>();
    }

    public DesignId Design { get; set; }
    public List<Object> Objects { get; set; }

    public Scene DeepClone()
    {
        return new Scene
        {
            Design = Design.DeepClone(),
            Objects = new List<Object>(Objects.Select(o => o.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Scene({Design})";
    }

    public bool IsInvalid()
    {
        return Design.IsInvalid() || Objects.Any(o => o.IsInvalid());
    }
}

public class Kit : IDeepCloneable<Kit>, IEntity
{
    public Kit()
    {
        Name = "";
        Description = "";
        Icon = "";
        Url = "";
        Homepage = "";
        Types = new List<Type>();
        Designs = new List<Design>();
    }

    public string Name { get; set; }
    public string Description { get; set; }
    public string Icon { get; set; }
    public string Url { get; set; }
    public string Homepage { get; set; }
    public List<Type> Types { get; set; }
    public List<Design> Designs { get; set; }

    public Kit DeepClone()
    {
        return new Kit
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Url = Url,
            Homepage = Homepage,
            Types = new List<Type>(Types.Select(t => t.DeepClone())),
            Designs = new List<Design>(Designs.Select(f => f.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Kit(Name:{Name}, {GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Name == "" || Types.Any(t => t.IsInvalid()) || Designs.Any(f => f.IsInvalid());
    }
}

public class KitMetadata : IDeepCloneable<KitMetadata>, IEntity
{
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Url { get; set; }
    public string? Homepage { get; set; }

    public KitMetadata DeepClone()
    {
        var kitMetadata = new KitMetadata();
        if (Name != null) kitMetadata.Name = Name;
        if (Description != null) kitMetadata.Description = Description;
        if (Icon != null) kitMetadata.Icon = Icon;
        if (Url != null) kitMetadata.Url = Url;
        if (Homepage != null) kitMetadata.Homepage = Homepage;
        return kitMetadata;
    }

    public override string ToString()
    {
        return $"Kit(Name:{Name})";
    }

    public bool IsInvalid()
    {
        return false;
    }
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
        Endpoint = $"http://127.0.0.1:{Constants.EnginePort}/graphql";
        Token = "";
        Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
    }

    public Api(string endpoint, string token)
    {
        Endpoint = endpoint;
        Token = token;
        Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
        if(Token!="")
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
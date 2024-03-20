using System;
using System.Collections.Generic;
using System.Linq;
using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.Newtonsoft;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using Semio.Properties;

// TODO: Replace GetHashcode() with a proper hash function
// TODO: Add logging mechanism to all API calls if they fail

#region Copilot
//type Query
//{
//loadLocalKit(directory: String!): LoadLocalKitResponse
//  sceneFromFormationFromLocalKit(directory: String!, formationInput: FormationInput!): SceneFromFormationFromLocalKitResponse
//}

//type LoadLocalKitResponse
//{
//kit: Kit
//  error: LoadLocalKitError
//}

//type Kit
//{
//name: String!
//  description: String!
//  icon: String!
//  createdAt: DateTime!
//  lastUpdateAt: DateTime!
//  url: String!
//  types: [Type!]!
//  formations: [Formation!]!
//}

//"""
//The `DateTime` scalar type represents a DateTime
//value as specified by
//[iso8601](https://en.wikipedia.org/wiki/ISO_8601).
//"""
//scalar DateTime

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

//type Representation
//{
//url: String!
//  lod: String!
//  type: Type
//  tags: [String!]!
//}

//type Port
//{
//plane: Plane
//  type: Type
//  specifiers: [Specifier!]!
//  attractings: [Attraction!]!
//  attracteds: [Attraction!]!
//}

//type Plane
//{
//port: Port
//  rootPiece: Piece
//  origin: Point!
//  xAxis: Vector!
//  yAxis: Vector!
//}

//type Piece
//{
//type: Type
//  formation: Formation
//  attractings: [Attraction!]!
//  attracteds: [Attraction!]!
//  id: String!
//  root: RootPiece!
//  diagram: DiagramPiece!
//}

//type Formation
//{
//name: String!
//  description: String!
//  icon: String!
//  variant: String!
//  unit: String!
//  createdAt: DateTime!
//  lastUpdateAt: DateTime!
//  volatile: Boolean!
//  kit: Kit
//  pieces: [Piece!]!
//  attractions: [Attraction!]!
//  qualities: [Quality!]!
//}

//type Attraction
//{
//formation: Formation
//  attracting: Side!
//  attracted: Side!
//}

//"""A side of an attraction."""
//type Side
//{
//piece: PieceSide!
//}

//"""The piece of a side of an attraction."""
//type PieceSide
//{
//id: String!
//  type: TypePieceSide!
//}

//"""The port of a type of a piece of a side of an attraction."""
//type TypePieceSide
//{
//port: Port
//}

//type Quality
//{
//name: String!
//  value: String!
//  unit: String!
//  type: Type
//  formation: Formation
//}

//"""The plane of the root piece of a formation."""
//type RootPiece
//{
//plane: Plane!
//}

//"""The point of a diagram of a piece."""
//type DiagramPiece
//{
//point: ScreenPoint!
//}

//type ScreenPoint
//{
//x: Int!
//  y: Int!
//}

//type Point
//{
//x: Float!
//  y: Float!
//  z: Float!
//}

//type Vector
//{
//x: Float!
//  y: Float!
//  z: Float!
//}

//type Specifier
//{
//context: String!
//  group: String!
//  port: Port
//}

//enum LoadLocalKitError
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_READ_KIT
//}

//type SceneFromFormationFromLocalKitResponse
//{
//    scene: Scene
//  error: SceneFromFormationFromLocalKitResponseError
//}

//type Scene
//{
//    objects: [Object]!
//}

//type Object
//{
//    piece: Piece
//  plane: Plane
//  parent: Object
//}

//type SceneFromFormationFromLocalKitResponseError
//{
//    code: SceneFromFormationFromLocalKitResponseErrorCode!
//  message: String
//}

//enum SceneFromFormationFromLocalKitResponseErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_READ_KIT
//  FORMATION_DOES_NOT_EXIST
//}

//input FormationInput
//{
//    name: String!
//  description: String
//  icon: String
//  variant: String
//  unit: String!
//  pieces: [PieceInput!]!
//  attractions: [AttractionInput!]!
//  qualities: [QualityInput!]
//}

//input PieceInput
//{
//    id: String!
//  type: TypeIdInput!
//  root: RootPieceInput = null
//  diagram: DiagramPieceInput!
//}

//input TypeIdInput
//{
//    name: String!
//  variant: String
//}

//input RootPieceInput
//{
//    plane: PlaneInput!
//}

//input PlaneInput
//{
//    origin: PointInput!
//  xAxis: VectorInput!
//  yAxis: VectorInput!
//}

//input PointInput
//{
//    x: Float!
//  y: Float!
//  z: Float!
//}

//input VectorInput
//{
//    x: Float!
//  y: Float!
//  z: Float!
//}

//input DiagramPieceInput
//{
//    point: ScreenPointInput!
//}

//input ScreenPointInput
//{
//    x: Int!
//  y: Int!
//}

//input AttractionInput
//{
//    attracting: SideInput!
//  attracted: SideInput!
//}

//input SideInput
//{
//    piece: PieceSideInput!
//}

//input PieceSideInput
//{
//    id: String!
//  type: TypePieceSideInput!
//}

//input TypePieceSideInput
//{
//    port: PortIdInput!
//}

//input PortIdInput
//{
//    specifiers: [SpecifierInput!]
//}

//input SpecifierInput
//{
//    context: String!
//}

//input QualityInput
//{
//    name: String!
//  value: String!
//  unit: String
//}

//type Mutation
//{
//    createLocalKit(directory: String!, kitInput: KitInput!): CreateLocalKitMutation
//  updateLocalKitMetadata(directory: String!, kitMetadataInput: KitMetadataInput!): UpdateLocalKitMetadataMutation
//  deleteLocalKit(directory: String!): DeleteLocalKitMutation
//  addTypeToLocalKit(directory: String!, typeInput: TypeInput!): AddTypeToLocalKitMutation
//  removeTypeFromLocalKit(directory: String!, typeId: TypeIdInput!): RemoveTypeFromLocalKitMutation
//  addFormationToLocalKit(directory: String!, formationInput: FormationInput!): AddFormationToLocalKitMutation
//  removeFormationFromLocalKit(directory: String!, formationId: FormationIdInput!): RemoveFormationFromLocalKitMutation
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

//input KitInput
//{
//    name: String!
//  description: String
//  icon: String
//  url: String
//  types: [TypeInput!]
//    formations: [FormationInput!]
//}

//input TypeInput
//{
//    name: String!
//  description: String
//  icon: String
//  variant: String
//  unit: String!
//  representations: [RepresentationInput!]!
//  ports: [PortInput!]!
//  qualities: [QualityInput!]
//}

//input RepresentationInput
//{
//    url: String!
//  lod: String
//  tags: [String!]
//}

//input PortInput
//{
//    plane: PlaneInput!
//  specifiers: [SpecifierInput!]
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

//input KitMetadataInput
//{
//    name: String
//  description: String
//  icon: String
//  url: String
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
//  FORMATION_DEPENDS_ON_TYPE
//}

//type AddFormationToLocalKitMutation
//{
//    formation: Formation
//  error: AddFormationToLocalKitError
//}

//type AddFormationToLocalKitError
//{
//    code: AddFormationToLocalKitErrorCode!
//  message: String
//}

//enum AddFormationToLocalKitErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_MODIFY_KIT
//  FORMATION_INPUT_IS_INVALID
//}

//type RemoveFormationFromLocalKitMutation
//{
//    error: RemoveFormationFromLocalKitError
//}

//type RemoveFormationFromLocalKitError
//{
//    code: RemoveFormationFromLocalKitErrorCode!
//  message: String
//}

//enum RemoveFormationFromLocalKitErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_MODIFY_KIT
//  FORMATION_DOES_NOT_EXIST
//}

//input FormationIdInput
//{
//    name: String!
//  variant: String
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
        Lod = "";
        Tags = new List<string>();
    }

    public string Url { get; set; }
    public string Lod { get; set; }
    public List<string> Tags { get; set; }

    public Representation DeepClone()
    {
        return new Representation
        {
            Url = Url,
            Lod = Lod,
            Tags = new List<string>(Tags)
        };
    }

    public override string ToString()
    {
        return $"Representation(Url: {Url})";
    }

    public bool IsInvalid()
    {
        return Url == "";
    }
}

public class Specifier : IDeepCloneable<Specifier>, IEntity
{
    public Specifier()
    {
        Context = "";
        Group = "";
    }

    public string Context { get; set; }
    public string Group { get; set; }

    public Specifier DeepClone()
    {
        return new Specifier
        {
            Context = Context,
            Group = Group
        };
    }

    public override string ToString()
    {
        return $"Specifier(Context: {Context})";
    }

    public bool IsInvalid()
    {
        return Context == "";
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
        return $"Point(X: {X}, Y: {Y})";
    }

    public bool IsInvalid()
    {
        return false;
    }

    public bool IsZero()
    {
        return X == 0 && Y == 0;
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
        return $"Point(X: {X}, Y: {Y}, Z: {Z})";
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
        return $"Vector(X: {X}, Y: {Y}, Z: {Z})";
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
        return $"Plane(Origin: {Origin}, XAxis: {XAxis}, YAxis: {YAxis})";
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
        Plane = new Plane();
        Specifiers = new List<Specifier>();
    }

    public Plane Plane { get; set; }
    public List<Specifier> Specifiers { get; set; }

    public Port DeepClone()
    {
        return new Port
        {
            Plane = Plane?.DeepClone(),
            Specifiers = new List<Specifier>(Specifiers.Select(s => s.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Port({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Plane.IsInvalid() || Specifiers.Any(s => s.IsInvalid());
    }
}

public class PortId : IDeepCloneable<PortId>, IEntity
{
    public PortId()
    {
        Specifiers = new List<Specifier>();
    }

    public List<Specifier> Specifiers { get; set; }

    public PortId DeepClone()
    {
        return new PortId
        {
            Specifiers = new List<Specifier>(Specifiers.Select(s => s.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"PortId({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Specifiers.Any(s => s.IsInvalid());
    }
}


public class Quality : IDeepCloneable<Quality>, IEntity
{
    public Quality()
    {
        Name = "";
        Value = "";
        Unit = "";
    }

    public string Name { get; set; }
    public string Value { get; set; }
    public string Unit { get; set; }

    public Quality DeepClone()
    {
        return new Quality
        {
            Name = Name,
            Value = Value,
            Unit = Unit
        };
    }

    public override string ToString()
    {
        return $"Quality(Name: {Name})";
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
        return $"Type(Name: {Name}, Variant: {Variant})";
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
        return $"TypeId(Name: {Name}, Variant: {Variant})";
    }

    public bool IsInvalid()
    {
        return Name == "";
    }
}

public class RootPiece : IDeepCloneable<RootPiece>, IEntity
{
    public RootPiece()
    {
        Plane = new Plane();
    }

    public Plane Plane { get; set; }

    public RootPiece DeepClone()
    {
        return new RootPiece
        {
            Plane = Plane.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"RootPiece({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Plane.IsInvalid();
    }
}

public class DiagramPiece : IDeepCloneable<DiagramPiece>, IEntity
{
    public DiagramPiece()
    {
        Point = new ScreenPoint();
    }

    public ScreenPoint Point { get; set; }

    public DiagramPiece DeepClone()
    {
        return new DiagramPiece
        {
            Point = Point.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"DiagramPiece({GetHashCode()})";
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
        Diagram = new DiagramPiece();
    }

    public string Id { get; set; }
    public TypeId Type { get; set; }
    public RootPiece? Root { get; set; }
    public DiagramPiece Diagram { get; set; }

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
        return $"Piece(Id: {Id})";
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
        return $"PieceId(Id: {Id})";
    }

    public bool IsInvalid()
    {
        return Id == "";
    }
}

public class TypePieceSide : IDeepCloneable<TypePieceSide>, IEntity
{
    public TypePieceSide()
    {
        Port = new PortId();
    }

    public PortId Port { get; set; }

    public TypePieceSide DeepClone()
    {
        return new TypePieceSide
        {
            Port = Port.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"TypePieceSide({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Port.IsInvalid();
    }
}

public class PieceSide : IDeepCloneable<PieceSide>, IEntity
{
    public PieceSide()
    {
        Id = "";
        Type = new TypePieceSide();
    }

    public string Id { get; set; }
    public TypePieceSide Type { get; set; }

    public PieceSide DeepClone()
    {
        return new PieceSide
        {
            Id = Id,
            Type = Type.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"PieceSide(Id: {Id})";
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
        Piece = new PieceSide();
    }

    public PieceSide Piece { get; set; }

    public Side DeepClone()
    {
        return new Side
        {
            Piece = Piece.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Side({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Piece.IsInvalid();
    }
}


public class Attraction : IDeepCloneable<Attraction>, IEntity
{
    public Attraction()
    {
        Attracting = new Side();
        Attracted = new Side();
    }

    public Side Attracting { get; set; }
    public Side Attracted { get; set; }

    public Attraction DeepClone()
    {
        return new Attraction
        {
            Attracting = Attracting.DeepClone(),
            Attracted = Attracted.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Attraction(Attracting(Piece: {Attracting.Piece.Id}), Attracted(Piece: {Attracted.Piece.Id}))";
    }

    public bool IsInvalid()
    {
        return Attracting.IsInvalid() || Attracted.IsInvalid() || Attracting.Piece.Id == Attracted.Piece.Id;
    }
}

public class Formation : IDeepCloneable<Formation>, IEntity
{
    public Formation()
    {
        Name = "";
        Description = "";
        Icon = "";
        Variant = "";
        Unit = "";
        Pieces = new List<Piece>();
        Attractions = new List<Attraction>();
        Qualities = new List<Quality>();
    }

    public string Name { get; set; }
    public string Description { get; set; }
    public string Icon { get; set; }
    public string Variant { get; set; }
    public string Unit { get; set; }
    public List<Piece> Pieces { get; set; }
    public List<Attraction> Attractions { get; set; }
    public List<Quality> Qualities { get; set; }

    public Formation DeepClone()
    {
        return new Formation
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Variant = Variant,
            Unit = Unit,
            Pieces = new List<Piece>(Pieces.Select(p => p.DeepClone())),
            Attractions = new List<Attraction>(Attractions.Select(a => a.DeepClone())),
            Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Formation(Name: {Name}, Variant: {Variant})";
    }

    public bool IsInvalid()
    {
        return Name == "" || Unit == "" || Pieces.Any(p => p.IsInvalid()) || Attractions.Any(a => a.IsInvalid()) ||
               Qualities.Any(q => q.IsInvalid());
    }
}

public class FormationId : IDeepCloneable<FormationId>, IEntity
{
    public FormationId()
    {
        Name = "";
        Variant = "";
    }

    public string Name { get; set; }
    public string Variant { get; set; }

    public FormationId DeepClone()
    {
        return new FormationId
        {
            Name = Name,
            Variant = Variant
        };
    }

    public override string ToString()
    {
        return $"FormationId(Name: {Name}, Variant: {Variant})";
    }

    public bool IsInvalid()
    {
        return Name == "";
    }
}

public class TypePieceObject : IDeepCloneable<TypePieceObject>, IEntity
{
    public TypePieceObject()
    {
        Representations = new List<Representation>();
    }

    public List<Representation> Representations { get; set; }

    public TypePieceObject DeepClone()
    {
        return new TypePieceObject
        {
            Representations = new List<Representation>(Representations.Select(f => f.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"TypePieceObject({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Representations.Any(r => r.IsInvalid());
    }
}

public class PieceObject : IDeepCloneable<PieceObject>, IEntity
{
    public PieceObject()
    {
        Id = "";
        Type = new TypePieceObject();
    }

    public string Id { get; set; }
    public TypePieceObject Type { get; set; }

    public PieceObject DeepClone()
    {
        return new PieceObject
        {
            Id = Id,
            Type = Type.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"PieceObject(Id: {Id})";
    }

    public bool IsInvalid()
    {
        return Id == "" || Type.IsInvalid();
    }
}


public class ParentObject : IDeepCloneable<ParentObject>, IEntity
{
    public ParentObject()
    {
        Piece = new PieceId();
    }

    public PieceId Piece { get; set; }

    public ParentObject DeepClone()
    {
        return new ParentObject
        {
            Piece = Piece.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"ParentObject({GetHashCode()})";
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
        Piece = new PieceObject();
        Plane = new Plane();
        Parent = null;
    }

    public PieceObject Piece { get; set; }
    public Plane Plane { get; set; }
    public ParentObject? Parent { get; set; }

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
        return $"Object({GetHashCode()})";
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
        Objects = new List<Object>();
    }

    public List<Object> Objects { get; set; }

    public Scene DeepClone()
    {
        return new Scene
        {
            Objects = new List<Object>(Objects.Select(o => o.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Scene({GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Objects.Any(o => o.IsInvalid());
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
        Types = new List<Type>();
        Formations = new List<Formation>();
    }

    public string Name { get; set; }
    public string Description { get; set; }
    public string Icon { get; set; }
    public string Url { get; set; }
    public List<Type> Types { get; set; }
    public List<Formation> Formations { get; set; }

    public Kit DeepClone()
    {
        return new Kit
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Url = Url,
            Types = new List<Type>(Types.Select(t => t.DeepClone())),
            Formations = new List<Formation>(Formations.Select(f => f.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Kit(Name: {Name}, {GetHashCode()})";
    }

    public bool IsInvalid()
    {
        return Name == "" || Types.Any(t => t.IsInvalid()) || Formations.Any(f => f.IsInvalid());
    }
}

public class KitMetadata : IDeepCloneable<KitMetadata>, IEntity
{
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Url { get; set; }

    public KitMetadata DeepClone()
    {
        var kitMetadata = new KitMetadata();
        if (Name != null) kitMetadata.Name = Name;
        if (Description != null) kitMetadata.Description = Description;
        if (Icon != null) kitMetadata.Icon = Icon;
        if (Url != null) kitMetadata.Url = Url;
        return kitMetadata;
    }

    public override string ToString()
    {
        return $"KitMetadata(Name: {Name})";
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
    FORMATION_DEPENDS_ON_TYPE
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

public enum AddFormationToLocalKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    FORMATION_INPUT_IS_INVALID
}

public class AddFormationToLocalKitError
{
    public AddFormationToLocalKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class AddFormationToLocalKitResponse
{
    public Formation? Formation { get; set; }
    public AddFormationToLocalKitError? Error { get; set; }
}

public class AddFormationToLocalKitResponseContainer
{
    public AddFormationToLocalKitResponse AddFormationToLocalKit { get; set; }
}

public enum RemoveFormationFromLocalKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    FORMATION_DOES_NOT_EXIST
}

public class RemoveFormationFromLocalKitError
{
    public RemoveFormationFromLocalKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class RemoveFormationFromLocalKitResponse
{
    public RemoveFormationFromLocalKitError? Error { get; set; }
}

public class RemoveFormationFromLocalKitResponseContainer
{
    public RemoveFormationFromLocalKitResponse RemoveFormationFromLocalKit { get; set; }
}

public enum SceneFromFormationFromLocalKitResponseErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_READ_KIT,
    FORMATION_DOES_NOT_EXIST
}

public class SceneFromFormationFromLocalKitResponseError
{
    public SceneFromFormationFromLocalKitResponseErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class SceneFromFormationFromLocalKitResponse
{
    public Scene? Scene { get; set; }
    public SceneFromFormationFromLocalKitResponseError? Error { get; set; }
}

public class SceneFromFormationFromLocalKitResponseContainer
{
    public SceneFromFormationFromLocalKitResponse SceneFromFormationFromLocalKit { get; set; }
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

    public AddFormationToLocalKitResponse? AddFormationToLocalKit(string directory, Formation formation)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.addFormationToLocalKit,
            OperationName = "AddFormationToLocalKit",
            Variables = new { directory, formation }
        };
        var response = Client.SendQueryAsync<AddFormationToLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.AddFormationToLocalKit;
    }

    public RemoveFormationFromLocalKitResponse? RemoveFormationFromLocalKit(string directory, FormationId formation)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.removeFormationFromLocalKit,
            OperationName = "RemoveFormationFromLocalKit",
            Variables = new { directory, formation }
        };
        var response = Client.SendQueryAsync<RemoveFormationFromLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.RemoveFormationFromLocalKit;
    }

    public SceneFromFormationFromLocalKitResponse? SceneFromFormationFromLocalKit(string directory,
        FormationId formation)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.formationToSceneFromLocalKit,
            OperationName = "SceneFromFormationFromLocalKit",
            Variables = new { directory, formation }
        };
        var response = Client.SendQueryAsync<SceneFromFormationFromLocalKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.SceneFromFormationFromLocalKit;
    }
}

#endregion
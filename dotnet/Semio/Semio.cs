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

#region Copilot

//type Query
//{
//loadLocalKit(directory: String!): LoadLocalKitResponse
//  formationToSceneFromLocalKit(directory: String!, formationIdInput: FormationIdInput!): FormationToSceneFromLocalKitResponse
//}

//type LoadLocalKitResponse
//{
//kit: Kit
//  error: LoadLocalKitError
//}

//type Kit implements Artifact {
//  name: String!
//  explanation: String
//  icon: String
//  createdAt: DateTime!
//  modifiedAt: DateTime!
//  url: String
//  types: [Type!]!
//  formations: [Formation!]!
//  parent: Artifact
//  children: [Artifact!]!
//  references: [Artifact!]!
//  referencedBy: [Artifact!]!
//  relatedTo: [Artifact!]!
//}

//interface Artifact
//{
//    name: String!
//  explanation: String
//  icon: String
//  parent: Artifact
//  children: [Artifact!]!
//  references: [Artifact!]!
//  referencedBy: [Artifact!]!
//  relatedTo: [Artifact!]!
//}

//"""
//The `DateTime` scalar type represents a DateTime
//value as specified by
//[iso8601](https://en.wikipedia.org/wiki/ISO_8601).
//"""
//scalar DateTime

//type Type implements Artifact {
//  name: String!
//  explanation: String
//  icon: String
//  createdAt: DateTime!
//  modifiedAt: DateTime!
//  kit: Kit
//  representations: [Representation!]!
//  ports: [Port!]!
//  qualities: [Quality!]!
//  pieces: [Piece!]!
//  parent: Artifact
//  children: [Artifact!]!
//  references: [Artifact!]!
//  referencedBy: [Artifact!]!
//  relatedTo: [Artifact!]!
//}

//type Representation
//{
//    url: String!
//  lod: String
//  type: Type
//  tags: [String!]!
//}

//type Port
//{
//    type: Type
//  specifiers: [Specifier!]!
//  attractings: [Attraction!]!
//  attracteds: [Attraction!]!
//  plane: Plane
//}

//type Specifier
//{
//    context: String!
//  group: String!
//  port: Port
//}

//type Attraction
//{
//    formation: Formation
//  attracting: Side!
//  attracted: Side!
//}

//type Formation implements Artifact {
//  name: String!
//  explanation: String
//  icon: String
//  createdAt: DateTime!
//  modifiedAt: DateTime!
//  kit: Kit
//  pieces: [Piece!]!
//  attractions: [Attraction!]!
//  qualities: [Quality!]!
//  parent: Artifact
//  children: [Artifact!]!
//  references: [Artifact!]!
//  referencedBy: [Artifact!]!
//  relatedTo: [Artifact!]!
//}

//type Piece
//{
//    type: Type
//  formation: Formation
//  attractings: [Attraction!]!
//  attracteds: [Attraction!]!
//  id: String!
//}

//type Quality
//{
//    name: String!
//  value: String!
//  unit: String
//  type: Type
//  formation: Formation
//}

//type Side
//{
//    piece: PieceSide!
//}

//type PieceSide
//{
//    id: String!
//  type: TypePieceSide!
//}

//type TypePieceSide
//{
//    port: Port
//}

//type Plane
//{
//    origin: Point!
//  xAxis: Vector!
//  yAxis: Vector!
//}

//type Point
//{
//    x: Float!
//  y: Float!
//  z: Float!
//}

//type Vector
//{
//    x: Float!
//  y: Float!
//  z: Float!
//}

//enum LoadLocalKitError
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_READ_KIT
//}
//type FormationToSceneFromLocalKitResponse
//{
//    scene: Scene
//    error: FormationToSceneFromLocalKitResponseError
//}

//type Scene
//{
//    objects: [Object]!
//}

//type Object
//{
//    piece: Piece
//    plane: Plane
//    parent: Object
//}

//type FormationToSceneFromLocalKitResponseError
//{
//    code: FormationToSceneFromLocalKitResponseErrorCode!
//    message: String
//}

//enum FormationToSceneFromLocalKitResponseErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_READ_KIT
//  FORMATION_DOES_NOT_EXIST
//}

//input FormationIdInput
//{
//    name: String!
//  qualities: [QualityInput!]
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
//  explanation: String
//  icon: String
//  url: String
//  types: [TypeInput!]
//    formations: [FormationInput!]
//}

//input TypeInput
//{
//    name: String!
//  explanation: String
//  icon: String
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

//input SpecifierInput
//{
//    context: String!
//  group: String!
//}

//input FormationInput
//{
//    name: String!
//  explanation: String
//  icon: String
//  pieces: [PieceInput!]!
//  attractions: [AttractionInput!]!
//  qualities: [QualityInput!]
//}

//input PieceInput
//{
//    id: String!
//  type: TypeIdInput!
//}

//input TypeIdInput
//{
//    name: String!
//  qualities: [QualityInput!]
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
//  explanation: String
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

public class Representation : IDeepCloneable<Representation>
{
    public string Url { get; set; }
    public string? Lod { get; set; }
    public List<string>? Tags { get; set; }

    public Representation DeepClone()
    {
        var representation = new Representation
        {
            Url = Url
        };
        if (Lod != null) representation.Lod = Lod;
        if (Tags != null) representation.Tags = new List<string>(Tags);
        return representation;
    }

    public override string ToString()
    {
        return $"Representation(Url: {Url})";
    }
}

public class Specifier : IDeepCloneable<Specifier>
{
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
}

public class Point : IDeepCloneable<Point>
{
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
}

public class Vector : IDeepCloneable<Vector>
{
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
}

public class Plane : IDeepCloneable<Plane>
{
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
}

public class Port : IDeepCloneable<Port>
{
    public Plane Plane { get; set; }
    public List<Specifier> Specifiers { get; set; }

    public Port DeepClone()
    {
        return new Port
        {
            Plane = Plane.DeepClone(),
            Specifiers = new List<Specifier>(Specifiers.Select(s => s.DeepClone()))
        };
    }

    public override string ToString()
    {
        return $"Port({GetHashCode()})";
    }
}

public class PortId : IDeepCloneable<PortId>
{
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
}


public class Quality : IDeepCloneable<Quality>
{
    public string Name { get; set; }
    public string Value { get; set; }
    public string? Unit { get; set; }

    public Quality DeepClone()
    {
        var quality = new Quality
        {
            Name = Name,
            Value = Value
        };
        if (Unit != null) quality.Unit = Unit;
        return quality;
    }

    public override string ToString()
    {
        return $"Quality(Name: {Name})";
    }
}

public class Type : IDeepCloneable<Type>
{
    public string Name { get; set; }
    public string? Explanation { get; set; }
    public string? Icon { get; set; }
    public List<Representation> Representations { get; set; }
    public List<Port> Ports { get; set; }
    public List<Quality>? Qualities { get; set; }

    public Type DeepClone()
    {
        var type = new Type
        {
            Name = Name,
            Representations = new List<Representation>(Representations.Select(r => r.DeepClone())),
            Ports = new List<Port>(Ports.Select(p => p.DeepClone())),
        };
        if (Explanation != null) type.Explanation = Explanation;
        if (Icon != null) type.Icon = Icon;
        if (Qualities != null) type.Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()));
        return type;
    }

    public override string ToString()
    {
        return $"Type(Name: {Name}, {GetHashCode()})";
    }
}

public class TypeId : IDeepCloneable<TypeId>
{
    public string Name { get; set; }
    public List<Quality>? Qualities { get; set; }

    public TypeId DeepClone()
    {
        var typeId = new TypeId
        {
            Name = Name,
        };
        if (Qualities != null) typeId.Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()));
        return typeId;
    }

    public override string ToString()
    {
        return $"TypeId(Name: {Name}, {GetHashCode()})";
    }
}

public class Piece : IDeepCloneable<Piece>
{
    public string Id { get; set; }
    public TypeId Type { get; set; }

    public Piece DeepClone()
    {
        return new Piece
        {
            Id = Id,
            Type = Type.DeepClone()
        };
    }

    public override string ToString()
    {
        return $"Piece(Id: {Id})";
    }
}

public class PieceId : IDeepCloneable<PieceId>
{
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
}

public class TypePieceSide : IDeepCloneable<TypePieceSide>
{
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
}

public class PieceSide : IDeepCloneable<PieceSide>
{
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
}

public class Side : IDeepCloneable<Side>
{
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
}


public class Attraction : IDeepCloneable<Attraction>
{
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
}

public class Formation : IDeepCloneable<Formation>
{
    public string Name { get; set; }
    public string? Explanation { get; set; }
    public string? Icon { get; set; }
    public List<Piece> Pieces { get; set; }
    public List<Attraction> Attractions { get; set; }
    public List<Quality>? Qualities { get; set; }

    public Formation DeepClone()
    {
        var formation = new Formation
        {
            Name = Name,
            Pieces = new List<Piece>(Pieces.Select(p => p.DeepClone())),
            Attractions = new List<Attraction>(Attractions.Select(a => a.DeepClone())),
        };
        if (Explanation != null) formation.Explanation = Explanation;

        if (Icon != null) formation.Icon = Icon;
        if (Qualities != null) formation.Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()));
        return formation;
    }

    public override string ToString()
    {
        return $"Formation(Name: {Name}, {GetHashCode()})";
    }
}

public class FormationId : IDeepCloneable<FormationId>
{
    public string Name { get; set; }
    public List<Quality>? Qualities { get; set; }

    public FormationId DeepClone()
    {
        var formationId = new FormationId
        {
            Name = Name,
        };
        if (Qualities != null) formationId.Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()));
        return formationId;
    }

    public override string ToString()
    {
        return $"FormationId(Name: {Name}, {GetHashCode()})";
    }
}

public class TypePieceObject : IDeepCloneable<TypePieceObject>
{
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
}

public class PieceObject : IDeepCloneable<PieceObject>
{
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
}


public class ParentObject : IDeepCloneable<ParentObject>
{
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
}

public class Object : IDeepCloneable<Object>
{
    public PieceObject Piece { get; set; }
    public Plane Plane { get; set; }

    public ParentObject? Parent { get; set; }

    public Object DeepClone()
    {
        var obj = new Object
        {
            Piece = Piece.DeepClone(),
            Plane = Plane.DeepClone()
        };
        if (Parent != null) obj.Parent = Parent.DeepClone();
        return obj;
    }

    public override string ToString()
    {
        return $"Object({GetHashCode()})";
    }
}

public class Scene : IDeepCloneable<Scene>
{
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
}

public class Kit : IDeepCloneable<Kit>
{
    public string Name { get; set; }
    public string? Explanation { get; set; }
    public string? Icon { get; set; }
    public string? Url { get; set; }
    public List<Type> Types { get; set; }
    public List<Formation> Formations { get; set; }

    public Kit DeepClone()
    {
        var kit = new Kit
        {
            Name = Name,
            Types = new List<Type>(Types.Select(t => t.DeepClone())),
            Formations = new List<Formation>(Formations.Select(f => f.DeepClone()))
        };
        if (Explanation != null) kit.Explanation = Explanation;

        if (Icon != null) kit.Icon = Icon;

        if (Url != null) kit.Url = Url;
        return kit;
    }

    public override string ToString()
    {
        return $"Kit(Name: {Name}, {GetHashCode()})";
    }
}

public class KitMetadata : IDeepCloneable<KitMetadata>
{
    public string? Name { get; set; }
    public string? Explanation { get; set; }
    public string? Icon { get; set; }
    public string? Url { get; set; }

    public KitMetadata DeepClone()
    {
        var kitMetadata = new KitMetadata();
        if (Name != null) kitMetadata.Name = Name;

        if (Explanation != null) kitMetadata.Explanation = Explanation;

        if (Icon != null) kitMetadata.Icon = Icon;

        if (Url != null) kitMetadata.Url = Url;
        return kitMetadata;
    }

    public override string ToString()
    {
        return $"KitMetadata(Name: {Name})";
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
    public Kit Kit { get; set; }
    public string Error { get; set; }
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
    public Kit Kit { get; set; }
    public CreateLocalKitError Error { get; set; }
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
    public Kit Kit { get; set; }
    public UpdateLocalKitMetadataError Error { get; set; }
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
    public Type Type { get; set; }
    public AddTypeToLocalKitError Error { get; set; }
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
    public RemoveTypeFromLocalKitError Error { get; set; }
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
    public Formation Formation { get; set; }
    public AddFormationToLocalKitError Error { get; set; }
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
    public RemoveFormationFromLocalKitError Error { get; set; }
}

public class RemoveFormationFromLocalKitResponseContainer
{
    public RemoveFormationFromLocalKitResponse RemoveFormationFromLocalKit { get; set; }
}

public enum FormationToSceneFromLocalKitResponseErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_READ_KIT,
    FORMATION_DOES_NOT_EXIST
}

public class FormationToSceneFromLocalKitResponseError
{
    public FormationToSceneFromLocalKitResponseErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class FormationToSceneFromLocalKitResponse
{
    public Scene Scene { get; set; }
    public FormationToSceneFromLocalKitResponseError Error { get; set; }
}

public class FormationToSceneFromLocalKitResponseContainer
{
    public FormationToSceneFromLocalKitResponse FormationToSceneFromLocalKit { get; set; }
}

public class Api : ICloneable
{
    public Api()
    {
        Endpoint = "http://127.0.0.1:5000/graphql";
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

    public Kit LoadLocalKit(string directory)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.loadLocalKit,
            OperationName = "LoadLocalKit",
            Variables = new { directory }
        };
        var response = Client.SendQueryAsync<LoadLocalKitResponseContainer>(query).Result;
        return response.Data.LoadLocalKit.Kit;
    }

    public CreateLocalKitResponse CreateLocalKit(string directory, Kit kit)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.createLocalKit,
            OperationName = "CreateLocalKit",
            Variables = new { directory, kit }
        };
        var response = Client.SendQueryAsync<CreateLocalKitResponseContainer>(query).Result;
        return response.Data.CreateLocalKit;
    }

    public UpdateLocalKitMetadataResponse UpdateLocalKitMetadata(string directory, KitMetadata kit)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.updateLocalKitMetadata,
            OperationName = "UpdateLocalKitMetadata",
            Variables = new { directory, kit }
        };
        var response = Client.SendQueryAsync<UpdateLocalKitMetadataResponseContainer>(query).Result;
        return response.Data.UpdateLocalKitMetadata;
    }

    public DeleteLocalKitResponse DeleteLocalKit(string directory)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.deleteLocalKit,
            OperationName = "DeleteLocalKit",
            Variables = new { directory }
        };
        var response = Client.SendQueryAsync<DeleteLocalKitResponseContainer>(query).Result;
        return response.Data.DeleteLocalKit;
    }

    public AddTypeToLocalKitResponse AddTypeToLocalKit(string directory, Type type)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.addTypeToLocalKit,
            OperationName = "AddTypeToLocalKit",
            Variables = new { directory, type }
        };
        var response = Client.SendQueryAsync<AddTypeToLocalKitResponseContainer>(query).Result;
        return response.Data.AddTypeToLocalKit;
    }

    public RemoveTypeFromLocalKitResponse RemoveTypeFromLocalKit(string directory, TypeId type)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.removeTypeFromLocalKit,
            OperationName = "RemoveTypeFromLocalKit",
            Variables = new { directory, type }
        };
        var response = Client.SendQueryAsync<RemoveTypeFromLocalKitResponseContainer>(query).Result;
        return response.Data.RemoveTypeFromLocalKit;
    }

    public AddFormationToLocalKitResponse AddFormationToLocalKit(string directory, Formation formation)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.addFormationToLocalKit,
            OperationName = "AddFormationToLocalKit",
            Variables = new { directory, formation }
        };
        var response = Client.SendQueryAsync<AddFormationToLocalKitResponseContainer>(query).Result;
        return response.Data.AddFormationToLocalKit;
    }

    public RemoveFormationFromLocalKitResponse RemoveFormationFromLocalKit(string directory, FormationId formation)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.removeFormationFromLocalKit,
            OperationName = "RemoveFormationFromLocalKit",
            Variables = new { directory, formation }
        };
        var response = Client.SendQueryAsync<RemoveFormationFromLocalKitResponseContainer>(query).Result;
        return response.Data.RemoveFormationFromLocalKit;
    }

    public FormationToSceneFromLocalKitResponse FormationToSceneFromLocalKit(string directory, FormationId formation)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.formationToSceneFromLocalKit,
            OperationName = "FormationToSceneFromLocalKit",
            Variables = new { directory, formation }
        };
        var response = Client.SendQueryAsync<FormationToSceneFromLocalKitResponseContainer>(query).Result;
        return response.Data.FormationToSceneFromLocalKit;
    }
}

#endregion
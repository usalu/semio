using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.ComponentModel.DataAnnotations;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Windows.Forms;
using FluentValidation;
using Force.DeepCloner;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;

// TODO: Replace GetHashcode() with a proper hash function.
// TODO: Add logging mechanism to all API calls if they fail.
// TODO: Add a more detailed message system when a model is invalid.


namespace Semio;

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
//  updateLocalKitProps(directory: String!, kitMetadataInput: KitPropsInput!): UpdateLocalKitPropsMutation
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

//type UpdateLocalKitPropsMutation
//{
//    kit: Kit
//  error: UpdateLocalKitPropsError
//}

//type UpdateLocalKitPropsError
//{
//    code: UpdateLocalKitPropsErrorCode!
//  message: String
//}

//enum UpdateLocalKitPropsErrorCode
//{
//    DIRECTORY_DOES_NOT_EXIST
//  DIRECTORY_IS_NOT_A_DIRECTORY
//  DIRECTORY_HAS_NO_KIT
//  NO_PERMISSION_TO_UPDATE_KIT
//  KIT_METADATA_IS_INVALID
//}

//"""🗃️ Meta-data of a kit."""
//input KitPropsInput
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

//public static class Generator
//{
//    public static string GenerateRandomId(int seed)
//    {
//        var adjectives = Resources.adjectives.Deserialize<List<string>>();
//        var animals = Resources.animals.Deserialize<List<string>>();
//        var random = new Random(seed);
//        var adjective = adjectives[random.Next(adjectives.Count)];
//        var animal = animals[random.Next(animals.Count)];
//        var number = random.Next(0, 999);
//        adjective = char.ToUpper(adjective[0]) + adjective.Substring(1);
//        animal = char.ToUpper(animal[0]) + animal.Substring(1);
//        return $"{adjective}{animal}{number}";
//    }
//}

public static class MimeParser
{
    public static string ParseFromUrl(string url)
    {
        var mimes = new Dictionary<string, string>
        {
            { ".stl", "model/stl" },
            { ".obj", "model/obj" },
            { ".glb", "model/gltf-binary" },
            { ".gltf", "model/gltf+json" },
            { ".3dm", "model/vnd.3dm" },
            { ".png", "image/png" },
            { ".jpg", "image/jpeg" },
            { ".jpeg", "image/jpeg" },
            { ".svg", "image/svg+xml" },
            { ".pdf", "application/pdf" },
            { ".zip", "application/zip" },
            { ".json", "application/json" },
            { ".csv", "text/csv" },
            { ".txt", "text/plain" }
        };
        try
        {
            return mimes[Path.GetExtension(url)];
        }
        catch (KeyNotFoundException)
        {
            return "application/octet-stream";
        }
    }
}

#endregion

//#region Models
//Emoji,Code,Abbreviation,Name,Description
//🧲,Cd,Cod,Connected,The connected piece of the side.
//🔩,Cg,Cog,Connecting,The connecting piece of the side.
//🖇️,Co,Con,Connection,A connection between two pieces in a design.
//💬,Dc?,Dsc,Description,An optional human description of the {{NAME}}.
//✏️,Dg,Dgm,Diagram,All diagram-related information of the piece.
//📁,Di,Dir,Directory,The directory of the kit.
//🏙️,Dn,Dsn,Design,A design is a collection of pieces that are connected.
//👪,Gr,Grp,Group,The group of the locator.
//🏠,Hp,Hmp,Homepage,The url of the homepage of the kit.
//🖼️,Ic,Ico,Icon,The icon [emoji | text | image | svg] of the {{NAME}}.
//🆔,Id,Idn,Identification,The local identification of the {{NAME}} within the {{PARENT_NAME}}.
//🗃️,Kt,Kit,Kit,A kit is a collection of designs that use types.
//🗺️,Lc,Loc,Locator,A locator is metadata for grouping ports.
//🔍,Ld,Lod,Level of Detail,The Level of Detail/Development/Design (LoD) of the representation.
//📛,Na,Nam,Name,The name of the {{NAME}}.
//🏷️,Mm,Mim,Mime,The Multipurpose Internet Mail Extensions (MIME) type of the content of the file of the representation.
//⌱,Og,Org,Origin,The origin of the plane.
//⭕,Pc,Pce,Piece,A piece is a 3d-instance of a type in a design.
//🔌,Po,Por,Port,A port is a connection point (with a direction) of a type.
//◳,Pn,Ple,Plane,A plane is an origin (point) and an orientation (x-axis and y-axis).
//✖️,Pt,Pnt,Point,A 3d-point (xyz) of floating point numbers.
//📏,Ql,Qal,Quality,A quality is meta-data for decision making.
//💾,Rp,Rep,Representation,A representation is a link to a file that describes a type for a certain level of detail and tags.
//🌱,Rt,Rot,Root,The root-related information of a piece.
//🧱,Sd,Sde,Side,A side of a piece in a connection.
//📌,SG,SGr,Subgroup,The sub-group of the locator.
//📺,SP,SPt,Screen Point,The 2d-point (xy) of integers in screen plane of the diagram of the piece.
//✅,Su,Suc,Success,{{NAME}} was successful.
//▦,Tf,Trf,Transform,A 4x4 translation and rotation transformation matrix (no scaling or shearing).
//🔖,Tg,Tag,Tag,A tag is metadata for grouping representations.
//🧩,Ty,Typ,Type,A type is a reusable element that can be connected with other types over ports.
//🔗,Ur,Url,Unique Resource Locator,Unique Resource Locator of the representation. Either a relative file path or link.
//Ⓜ️,Ut,Unt,Unit,The length unit for all distance-related information of the {{PARENT_NAME}}.
//➡️,Vc,Vec,Vector,A 3d-vector (xyz) of floating point numbers.
//🔢,Vl,Val,Value,The value of the quality.
//🔀,Vn,Vnt,Variant,An optional variant of the {{NAME}}.

public abstract class ConceptAttribute : Attribute
{
    public ConceptAttribute(string emoji, string code, string abbreviation, string description)
    {
        Emoji = emoji;
        Code = code;
        Abbreviation = abbreviation;
        Description = description;
    }
    public string Emoji { get; set; }
    public string Code { get; set; }
    public string Abbreviation { get; set; }
    public string Description { get; set; }
}

[AttributeUsage(AttributeTargets.Class)]
public class ModelAttribute : ConceptAttribute
{
    public ModelAttribute(string emoji, string code, string abbreviation, string description) : base(emoji, code,
        abbreviation, description)
    {
    }
}

public enum PropImportance
{
    OPTIONAL,
    REQUIRED,
    ID
}


[AttributeUsage(AttributeTargets.Property)]
public abstract class PropAttribute : ConceptAttribute
{
    public PropImportance Importance { get; set; }
    public bool IsDefaultValid { get; set; }
    public PropAttribute(string emoji, string code, string abbreviation, string description, PropImportance importance, bool isDefaultValid) : base(emoji, code,
        abbreviation, description)
    {
        Importance = importance;
        IsDefaultValid = isDefaultValid;
    }
}

public abstract class TextAttribute : PropAttribute
{
    public int LengthLimit { get; set; }

    public TextAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance, bool isDefaultValid, int lengthLimit) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid)
    {
        LengthLimit = lengthLimit;
    }
}
public class NameAttribute : TextAttribute
{
    public NameAttribute(string emoji, string code, string abbreviation, string description, PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = false) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, Constants.NameLengthLimit)
    {
    }

}

public class UrlAttribute : TextAttribute
{
    public UrlAttribute(string emoji, string code, string abbreviation, string description, PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, Constants.UrlLengthLimit)
    {
    }

}

public class DescriptionAttribute : TextAttribute
{
    public DescriptionAttribute(string emoji, string code, string abbreviation, string description, PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, Constants.DescriptionLengthLimit)
    {
    }

}

public class IntPropAttribute : PropAttribute
{
    public IntPropAttribute(string emoji, string code, string abbreviation, string description, PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid)
    {
    }
}

public class ModelPropAttribute : PropAttribute
{
    public ModelPropAttribute(string emoji, string code, string abbreviation, string description, PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid)
    {
    }
}


public abstract class Model<T> where T : Model<T>
{
    public override string ToString()
    {
        var modelAttribute = GetType().GetCustomAttribute<ModelAttribute>();
        var nonEmptyIdProperties = GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(p => p.GetCustomAttribute<PropAttribute>()?.Importance == PropImportance.ID &&
                        (string)p.GetValue(this) != "")
            .Select(p => p.Name);
        var nonEmptyIdPropertiesValues = nonEmptyIdProperties.Select(p => GetType().GetProperty(p)?.GetValue(this)).Cast<string>().ToList();
        return $"{modelAttribute.Abbreviation}({string.Join(", ", nonEmptyIdPropertiesValues)})";

    }
    public override bool Equals(object obj)
    {
        if (obj == null || GetType() != obj.GetType())
            return false;

        var other = obj;
        return GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .All(prop => PropertiesAreEqual(prop, this, other));
    }

    private bool PropertiesAreEqual(PropertyInfo prop, object obj1, object obj2)
    {
        var value1 = prop.GetValue(obj1);
        var value2 = prop.GetValue(obj2);

        if (value1 is IEnumerable enumerable1 && value2 is IEnumerable enumerable2)
            return enumerable1.Cast<object>().SequenceEqual(enumerable2.Cast<object>());

        return Equals(value1, value2);
    }

    public override int GetHashCode()
    {
        return GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Select(prop => prop.GetValue(this))
            .Where(value => value != null)
            .Aggregate(17, (current, value) => current * 31 + value.GetHashCode());
    }

    public static bool operator ==(Model<T> left, Model<T> right)
    {
        if (ReferenceEquals(left, right))
            return true;

        if (left is null || right is null)
            return false;

        return left.Equals(right);
    }

    public static bool operator !=(Model<T> left, Model<T> right)
    {
        return !(left == right);
    }

    public Model<T> DeepClone()
    {
        if (DeepClonerExtensions.DeepClone(this) is not { } deepClone)
            throw new Exception("DeepClone failed.");
        return deepClone;
    }

    public virtual (bool, List<string>) Validate()
    {
        var validator = new ModelValidator<T>();
        var result = validator.Validate((T)this);
        return (result.IsValid, result.Errors.Select(e => e.ToString()).ToList());
    }
}

public class ModelValidator<T>: AbstractValidator<T> where T : Model<T>
{
    public ModelValidator()
    {
        foreach (var property in typeof(T).GetProperties())
        {
            // check if property is list


            if (property.PropertyType == typeof(string))
            {
                var textAttribute = property.GetCustomAttribute<TextAttribute>();

                RuleFor(model => property.GetValue(model) as string)
                    .NotEmpty()
                    .When(m => (textAttribute.Importance != PropImportance.OPTIONAL) || !textAttribute.IsDefaultValid)
                    .WithMessage($"The {property.Name}({textAttribute.Code}) must not be empty.")
                    .MaximumLength(textAttribute.LengthLimit)
                    .WithMessage(model =>
                    {
                        var value = property.GetValue(model) as string;
                        var preview = value?.Length > 10 ? value.Substring(0, 10) + "..." : value;
                        return $"The {property.Name}({textAttribute.Code}) must be at most {textAttribute.LengthLimit} characters long. The provided text ({preview}) has {value?.Length} characters.";
                    });
            }
            else if (property.PropertyType == typeof(List<string>))
            {
                var textAttribute = property.GetCustomAttribute<TextAttribute>();

                RuleFor(model => property.GetValue(model) as List<string>)
                    .NotEmpty()
                    .When(m => (textAttribute.Importance != PropImportance.OPTIONAL))
                    .ForEach(item =>
                    {
                        item
                        .NotEmpty()
                        .When(m => !textAttribute.IsDefaultValid)
                        .WithMessage(item =>
                        {
                            return $"An element of {property.Name}({textAttribute.Code}) must not be empty.";
                        })
                        .MaximumLength(textAttribute.LengthLimit)
                        .WithMessage((model, item) =>
                        {
                            var preview = item?.Length > 10 ? item.Substring(0, 10) + "..." : item;
                            return $"An element of {property.Name}({textAttribute.Code}) must be at most {textAttribute.LengthLimit} characters long. The provided text ({preview}) has {item?.Length} characters.";
                        });

                    })
                    .OverridePropertyName(property.Name);

            }
            else if (property.PropertyType == typeof(int))
            {
                
            }
            
            
        }
    }
}


/// <summary>
/// 💾 A representation is an url that describes a type for a certain level of detail and tags.
/// </summary>
[Model("💾", "Rp", "Rep",
    "A representation is a linked file that describes a type for a certain level of detail and tags.")]
public class Representation : Model<Representation>
{
    /// <summary>
    /// 🔗 The Unique Resource Locator (URL) to another resource outside of semio.
    /// absolute file path or a link.
    /// </summary>
    [Url("🔗", "Ur", "Url", "The Unique Resource Locator (URL) to another file outside of semio.", PropImportance.ID)]
    public string Url { get; set; } = "";

    /// <summary>
    /// 🏷️ The Multipurpose Internet Mail Extensions (MIME) type of the content of the file of the representation.
    /// </summary>
    [Name("🏷️", "Mm", "Mim", "The Multipurpose Internet Mail Extensions (MIME) type of the content of the file of the representation.", PropImportance.REQUIRED)]
    public string Mime { get; set; } = "";
    /// <summary>
    /// 🔍 The optional Level of Detail/Development/Design (LoD) of the representation.
    /// </summary>

    [Name("🔍", "Ld?", "Lod", "The optional Level of Detail/Development/Design (LoD) of the representation.", isDefaultValid: true)]
    public string Lod { get; set; } = "";
    /// <summary>
    /// 🔖 Optional tags to group representations.
    /// </summary>

    [Name("🔖", "Tg*", "Tags", "Optional tags to group representations.", isDefaultValid:false)]
    public List<string> Tags { get; set; } = new();
}
[Model("🗺️","Lc","Loc","A locator is metadata for grouping ports.")]
public class Locator : Model<Locator>
{
    /// <summary>
    /// 👪 The group of the locator.
    /// </summary>
    [Name("👪", "Gr", "Grp", "The group of the locator.", PropImportance.ID)]
    public string Group { get; set; } = "";
    /// <summary>
    /// 📌 An optional sub-group of the locator. No sub-group means true.
    /// </summary>
    [Name("📌", "SG", "SGr", "The optional sub-group of the locator. No sub-group means true.", PropImportance.ID)]
    public string Subgroup { get; set; } = "";

}

/// <summary>
/// 📺 A 2d-point (xy) of integers in screen plane.
/// </summary>
[Model("📺", "SP", "SPt", "A 2d-point (xy) of integers in screen plane.")]
public class ScreenPoint : Model<ScreenPoint>
{
    [IntProp("📺", "X", "XCo", "The x-coordinate of the screen point.", PropImportance.REQUIRED)]
    public int X { get; set; } = 0;
    [IntProp("📺", "Y", "YCo", "The y-coordinate of the screen point.", PropImportance.REQUIRED)]
    public int Y { get; set; } = 0;

}

//public class Point() : Model
//{
//    public float X { get; set; } = 0;
//    public float Y { get; set; } = 0;
//    public float Z { get; set; } = 0;

//    public bool IsZero()
//    {
//        return X == 0 && Y == 0 && Z == 0;
//    }
//}

//public class Vector() : Model
//{
//    public float X { get; set; } = 0;
//    public float Y { get; set; } = 0;
//    public float Z { get; set; } = 0;

//    public bool IsZero()
//    {
//        return X == 0 && Y == 0 && Z == 0;
//    }
//}

//public class Plane() : Model
//{
//    public Point Origin { get; set; } = new();
//    public Vector XAxis { get; set; } = new();
//    public Vector YAxis { get; set; } = new();

//}

//public class Port() : Model
//{
//    [Id("🆔", "Id", "Idn", "Local identification of the port within the type.")]
//    public string Id { get; set; } = "";
//    [Required(ErrorMessage = "A port needs a point.")]
//    public Point Point { get; set; } = new();
//    public Vector Direction { get; set; } = new();
//    public List<Locator> Locators { get; set; } = new();
//}

//    public class PortId() : Model
//    {
//        public string Id { get; set; } = "";

//    }

/// <summary>
/// 📏 A quality is meta-data for decision making.
/// </summary>
[Model("📏", "Ql", "Qal", "A quality is meta-data for decision making.")]
public class Quality : Model<Quality>
{
    /// <summary>
    /// 📛 The name of the quality.
    /// </summary>
    [Name("📏", "Na", "Nam", "The name of the quality.", PropImportance.ID)]
    public string Name { get; set; } = "";
    /// <summary>
    /// 🔢 An optional value of the quality. No value is equivalent to true for the name.
    /// </summary>
    [Name("🔢", "Vl?", "Val", "An optional value of the quality. No value is equivalent to true for the name.")]
    public string Value { get; set; } = "";

    /// <summary>
    /// Ⓜ️ The unit of the value of the quality.
    /// </summary>
    [Name("Ⓜ️", "Ut", "Unt", "The unit of the value of the quality.")]
    public string Unit { get; set; } = "";
    public string Definition { get; set; } = "";
}

/// <summary>
/// 🧩 A type is a reusable element that can be connected with other types over ports.
/// </summary>
[Model("🧩", "Ty", "Typ", "A type is a reusable element that can be connected with other types over ports.")]
public class Type : Model<Type>
{
    /// <summary>
    /// 📛 Name of the type.
    /// </summary>
    [Name("📛", "Na", "Nam",  "The name of the type.",PropImportance.ID)]
    public string Name { get; set; } = "";
    /// <summary>
    /// 💬 An optional human description of the type.
    /// </summary>
    [Description("💬", "Dc?", "Dsc", "An optional human description of the type.")]
    public string Description { get; set; } = "";
    /// <summary>
    /// 🖼️ An optional icon [emoji | text | image | svg] of the type.
    /// </summary>
    [Url("🖼️", "Ic?", "Ico", "An optional icon [emoji | text | image | svg] of the type.")]
    public string Icon { get; set; } = "";
    /// <summary>
    /// 🔀 An optional variant of the type.
    /// </summary>
    [Name("🔀", "Vn?", "Vnt", "An optional variant of the type.", PropImportance.ID,isDefaultValid:true)]
    public string Variant { get; set; } = "";
    /// <summary>
    /// Ⓜ️ The length unit for all distance-related information of the type.
    /// </summary>
    [Name("Ⓜ️", "Ut", "Unt", "The length unit for all distance-related information of the type.", PropImportance.REQUIRED)]
    public string Unit { get; set; } = "";
    /// <summary>
    /// 💾 The representations of the type.
    /// </summary>
    [ModelProp("💾", "Rp+", "Reps", "The representations of the type.",PropImportance.REQUIRED)]
    public List<Representation> Representations { get; set; } = new();
    //public List<Port> Ports { get; set; } = new();
    /// <summary>
    /// 📏 The optional qualities of the type.
    /// </summary>
    [ModelProp("📏", "Ql*", "Qualities", "The optional qualities of the type.")]
    public List<Quality> Qualities { get; set; } = new();

}

//    public class TypeId() : Model
//    {
//        public string Name { get; set; } = "";
//        public string Variant { get; set; } = "";

//    }

//    public class PieceRoot() : Model
//    {
//        public Plane Plane { get; set; } = new();
//    }

//    public class PieceDiagram() : Model
//    {
//        public ScreenPoint Point { get; set; } = new();

//    }

//    public class Piece() : Model
//    {
//        public string Id { get; set; } = "";
//        public TypeId Type { get; set; } = new();
//        public PieceRoot? Root { get; set; } = null;
//        public PieceDiagram Diagram { get; set; } = new();

//    }

//    public class PieceId() : Model
//    {
//        public string Id { get; set; } = "";

//    }

//    public class SidePieceType() : Model
//    {
//        public PortId Port { get; set; } = new();

//    }

//    public class SidePiece() : Model
//    {
//        public string Id { get; set; } = "";
//        public SidePieceType Type { get; set; } = new();

//    }

//    public class Side() : Model
//    {
//        public SidePiece Piece { get; set; } = new();

//    }

//    public class Connection() : Model
//    {
//        public Side Connected { get; set; } = new();
//        public Side Connecting { get; set; } = new();
//        public float Offset { get; set; } = 0;
//        public float Rotation { get; set; } = 0;

//    }

//    public class Design() : Model
//    {
//        public string Name { get; set; } = "";
//        public string Description { get; set; } = "";
//        public string Icon { get; set; } = "";
//        public string Variant { get; set; } = "";
//        public string Unit { get; set; } = "";
//        public List<Piece> Pieces { get; set; } = new();
//        public List<Connection> Connections { get; set; } = new();
//        public List<Quality> Qualities { get; set; } = new();

//    public Design Flatten(Type[] types = null)
//    {
//        Design flattenedDesign = this.DeepClone();
//        if (Pieces.Count <= 1 || Connections.Count == 0)
//            return flattenedDesign;
//        var graph = new UndirectedGraph<string, Edge<string>>();
//        foreach (var piece in Pieces)
//            graph.AddVertex(piece.Id);
//        foreach (var connection in Connections)
//            graph.AddEdge(new Edge<string>(connection.Connected.Piece.Id, connection.Connecting.Piece.Id));
//        var root = Pieces.First(p => p.Root != null) ?? Pieces.First();
//        var components = new Dictionary<string, int>();
//        graph.ConnectedComponents(components);
//        return flattenedDesign;

//    }
//}

//    public class DesignId() : Model
//    {
//        public string Name { get; set; } = "";
//        public string Variant { get; set; } = "";

//    }

//    public class Kit() : Model
//    {
//        public string Name { get; set; } = "";
//        public string Description { get; set; } = "";
//        public string Icon { get; set; } = "";
//        public string Url { get; set; } = "";
//        public string Homepage { get; set; } = "";
//        public List<Type> Types { get; set; } = new();
//        public List<Design> Designs { get; set; } = new();

//    }

//    public class KitProps : Model
//    {
//        public string? Name { get; set; }
//        public string? Description { get; set; }
//        public string? Icon { get; set; }
//        public string? Url { get; set; }
//        public string? Homepage { get; set; }
//    }

//    #endregion


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

//    #region Api

//    public class LoadLocalKitResponse
//    {
//        public Kit? Kit { get; set; }
//        public string? Error { get; set; }
//    }

//    public class LoadLocalKitResponseContainer
//    {
//        public LoadLocalKitResponse LoadLocalKit { get; set; }
//    }

//    public enum CreateLocalKitErrorCode
//    {
//        DIRECTORY_IS_NOT_A_DIRECTORY,
//        DIRECTORY_ALREADY_CONTAINS_A_KIT,
//        NO_PERMISSION_TO_CREATE_DIRECTORY,
//        NO_PERMISSION_TO_CREATE_KIT,
//        KIT_INPUT_IS_INVALID
//    }

//    public class CreateLocalKitError
//    {
//        public CreateLocalKitErrorCode Code { get; set; }
//        public string Message { get; set; }
//    }

//    public class CreateLocalKitResponse
//    {
//        public Kit? Kit { get; set; }
//        public CreateLocalKitError? Error { get; set; }
//    }

//    public class CreateLocalKitResponseContainer
//    {
//        public CreateLocalKitResponse CreateLocalKit { get; set; }
//    }

//    public enum UpdateLocalKitPropsErrorCode
//    {
//        DIRECTORY_DOES_NOT_EXIST,
//        DIRECTORY_IS_NOT_A_DIRECTORY,
//        DIRECTORY_HAS_NO_KIT,
//        NO_PERMISSION_TO_UPDATE_KIT,
//        KIT_METADATA_IS_INVALID
//    }

//    public class UpdateLocalKitPropsError
//    {
//        public UpdateLocalKitPropsErrorCode Code { get; set; }
//        public string Message { get; set; }
//    }

//    public class UpdateLocalKitPropsResponse
//    {
//        public KitProps? Kit { get; set; }
//        public UpdateLocalKitPropsError? Error { get; set; }
//    }

//    public class UpdateLocalKitPropsResponseContainer
//    {
//        public UpdateLocalKitPropsResponse UpdateLocalKitProps { get; set; }
//    }

//    public enum DeleteLocalKitError
//    {
//        DIRECTORY_DOES_NOT_EXIST,
//        DIRECTORY_HAS_NO_KIT,
//        NO_PERMISSION_TO_DELETE_KIT
//    }

//    public class DeleteLocalKitResponse
//    {
//        public DeleteLocalKitError? Error { get; set; }
//    }

//    public class DeleteLocalKitResponseContainer
//    {
//        public DeleteLocalKitResponse DeleteLocalKit { get; set; }
//    }

//    public enum AddTypeToLocalKitErrorCode
//    {
//        DIRECTORY_DOES_NOT_EXIST,
//        DIRECTORY_IS_NOT_A_DIRECTORY,
//        DIRECTORY_HAS_NO_KIT,
//        NO_PERMISSION_TO_MODIFY_KIT,
//        TYPE_INPUT_IS_INVALID
//    }

//    public class AddTypeToLocalKitError
//    {
//        public AddTypeToLocalKitErrorCode Code { get; set; }
//        public string Message { get; set; }
//    }

//    public class AddTypeToLocalKitResponse
//    {
//        public Type? Type { get; set; }
//        public AddTypeToLocalKitError? Error { get; set; }
//    }

//    public class AddTypeToLocalKitResponseContainer
//    {
//        public AddTypeToLocalKitResponse AddTypeToLocalKit { get; set; }
//    }

//    public enum RemoveTypeFromLocalKitErrorCode
//    {
//        DIRECTORY_DOES_NOT_EXIST,
//        DIRECTORY_IS_NOT_A_DIRECTORY,
//        DIRECTORY_HAS_NO_KIT,
//        NO_PERMISSION_TO_MODIFY_KIT,
//        TYPE_DOES_NOT_EXIST,
//        DESIGN_DEPENDS_ON_TYPE
//    }

//    public class RemoveTypeFromLocalKitError
//    {
//        public RemoveTypeFromLocalKitErrorCode Code { get; set; }
//        public string Message { get; set; }
//    }

//    public class RemoveTypeFromLocalKitResponse
//    {
//        public RemoveTypeFromLocalKitError? Error { get; set; }
//    }

//    public class RemoveTypeFromLocalKitResponseContainer
//    {
//        public RemoveTypeFromLocalKitResponse RemoveTypeFromLocalKit { get; set; }
//    }

//    public enum AddDesignToLocalKitErrorCode
//    {
//        DIRECTORY_DOES_NOT_EXIST,
//        DIRECTORY_IS_NOT_A_DIRECTORY,
//        DIRECTORY_HAS_NO_KIT,
//        NO_PERMISSION_TO_MODIFY_KIT,
//        DESIGN_INPUT_IS_INVALID
//    }

//    public class AddDesignToLocalKitError
//    {
//        public AddDesignToLocalKitErrorCode Code { get; set; }
//        public string Message { get; set; }
//    }

//    public class AddDesignToLocalKitResponse
//    {
//        public Design? Design { get; set; }
//        public AddDesignToLocalKitError? Error { get; set; }
//    }

//    public class AddDesignToLocalKitResponseContainer
//    {
//        public AddDesignToLocalKitResponse AddDesignToLocalKit { get; set; }
//    }

//    public enum RemoveDesignFromLocalKitErrorCode
//    {
//        DIRECTORY_DOES_NOT_EXIST,
//        DIRECTORY_IS_NOT_A_DIRECTORY,
//        DIRECTORY_HAS_NO_KIT,
//        NO_PERMISSION_TO_MODIFY_KIT,
//        DESIGN_DOES_NOT_EXIST
//    }

//    public class RemoveDesignFromLocalKitError
//    {
//        public RemoveDesignFromLocalKitErrorCode Code { get; set; }
//        public string Message { get; set; }
//    }

//    public class RemoveDesignFromLocalKitResponse
//    {
//        public RemoveDesignFromLocalKitError? Error { get; set; }
//    }

//    public class RemoveDesignFromLocalKitResponseContainer
//    {
//        public RemoveDesignFromLocalKitResponse RemoveDesignFromLocalKit { get; set; }
//    }


//    public class Api : ICloneable
//    {
//        public Api()
//        {
//            Endpoint = "http://127.0.0.1:5052/graphql";
//            Token = "";
//            Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
//        }

//        public Api(string endpoint, string token)
//        {
//            Endpoint = endpoint;
//            Token = token;
//            Client = new GraphQLHttpClient(Endpoint, new NewtonsoftJsonSerializer());
//            Client.HttpClient.DefaultRequestHeaders.Add("Authorization", $"Bearer {Token}");
//        }

//        public GraphQLHttpClient Client { get; set; }
//        public string Endpoint { get; set; }
//        public string Token { get; set; }

//        public object Clone()
//        {
//            return new Api(Endpoint, Token);
//        }

//        public override string ToString()
//        {
//            return $"Api(Endpoint: {Endpoint}, Token: {Token})";
//        }

//        public LoadLocalKitResponse? LoadLocalKit(string directory)
//        {
//            var query = new GraphQLRequest
//            {
//                Query = Resources.loadLocalKit,
//                OperationName = "LoadLocalKit",
//                Variables = new { directory }
//            };
//            var response = Client.SendQueryAsync<LoadLocalKitResponseContainer>(query).Result;
//            if (response.Errors != null) return null;
//            return response.Data.LoadLocalKit;
//        }

//        public CreateLocalKitResponse? CreateLocalKit(string directory, Kit kit)
//        {
//            var query = new GraphQLRequest
//            {
//                Query = Resources.createLocalKit,
//                OperationName = "CreateLocalKit",
//                Variables = new { directory, kit }
//            };
//            var response = Client.SendQueryAsync<CreateLocalKitResponseContainer>(query).Result;
//            if (response.Errors != null) return null;
//            return response.Data.CreateLocalKit;
//        }

//        public UpdateLocalKitPropsResponse? UpdateLocalKitProps(string directory, KitProps kit)
//        {
//            var query = new GraphQLRequest
//            {
//                Query = Resources.updateLocalKitMetadata,
//                OperationName = "UpdateLocalKitProps",
//                Variables = new { directory, kit }
//            };
//            var response = Client.SendQueryAsync<UpdateLocalKitPropsResponseContainer>(query).Result;
//            if (response.Errors != null) return null;
//            return response.Data.UpdateLocalKitProps;
//        }

//        public DeleteLocalKitResponse? DeleteLocalKit(string directory)
//        {
//            var query = new GraphQLRequest
//            {
//                Query = Resources.deleteLocalKit,
//                OperationName = "DeleteLocalKit",
//                Variables = new { directory }
//            };
//            var response = Client.SendQueryAsync<DeleteLocalKitResponseContainer>(query).Result;
//            if (response.Errors != null) return null;
//            return response.Data.DeleteLocalKit;
//        }

//        public AddTypeToLocalKitResponse? AddTypeToLocalKit(string directory, Type type)
//        {
//            var query = new GraphQLRequest
//            {
//                Query = Resources.addTypeToLocalKit,
//                OperationName = "AddTypeToLocalKit",
//                Variables = new { directory, type }
//            };
//            var response = Client.SendQueryAsync<AddTypeToLocalKitResponseContainer>(query).Result;
//            if (response.Errors != null) return null;
//            return response.Data.AddTypeToLocalKit;
//        }

//        public RemoveTypeFromLocalKitResponse? RemoveTypeFromLocalKit(string directory, TypeId type)
//        {
//            var query = new GraphQLRequest
//            {
//                Query = Resources.removeTypeFromLocalKit,
//                OperationName = "RemoveTypeFromLocalKit",
//                Variables = new { directory, type }
//            };
//            var response = Client.SendQueryAsync<RemoveTypeFromLocalKitResponseContainer>(query).Result;
//            if (response.Errors != null) return null;
//            return response.Data.RemoveTypeFromLocalKit;
//        }

//        public AddDesignToLocalKitResponse? AddDesignToLocalKit(string directory, Design design)
//        {
//            var query = new GraphQLRequest
//            {
//                Query = Resources.addDesignToLocalKit,
//                OperationName = "AddDesignToLocalKit",
//                Variables = new { directory, design }
//            };
//            var response = Client.SendQueryAsync<AddDesignToLocalKitResponseContainer>(query).Result;
//            if (response.Errors != null) return null;
//            return response.Data.AddDesignToLocalKit;
//        }

//        public RemoveDesignFromLocalKitResponse? RemoveDesignFromLocalKit(string directory, DesignId design)
//        {
//            var query = new GraphQLRequest
//            {
//                Query = Resources.removeDesignFromLocalKit,
//                OperationName = "RemoveDesignFromLocalKit",
//                Variables = new { directory, design }
//            };
//            var response = Client.SendQueryAsync<RemoveDesignFromLocalKitResponseContainer>(query).Result;
//            if (response.Errors != null) return null;
//            return response.Data.RemoveDesignFromLocalKit;
//        }

//    }


//#endregion


public static class Meta
{
    /// <summary>
    /// Name of the model : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, System.Type> Type;
    /// <summary>
    /// Name of the model : ModelAttribute
    /// </summary>
    public static readonly ImmutableDictionary<string, ModelAttribute> Model;
    /// <summary>
    /// Name of the model : Name of the property : PropertyInfo
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableDictionary<string, PropertyInfo>> Property;
    /// <summary>
    /// Name of the model : Name of the property : PropAttribute
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableDictionary<string, PropAttribute>> Prop;
    /// <summary>
    /// Name of the model : Name of the property : IsList
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableDictionary<string, bool>> IsPropertyList;
    /// <summary>
    /// Name of the model : Name of the property : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableDictionary<string, System.Type>> PropertyItemType;
    /// <summary>
    /// Name of the model : Name of the property : IsModel
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableDictionary<string, bool>> IsPropertyModel;

    static Meta()
    {
        var type = new Dictionary<string, System.Type>();
        var model = new Dictionary<string, ModelAttribute>();
        var property = new Dictionary<string, Dictionary<string, PropertyInfo>>();
        var prop = new Dictionary<string, Dictionary<string, PropAttribute>>();
        var isPropertyList = new Dictionary<string, Dictionary<string, bool>>();
        var propertyItemType = new Dictionary<string, Dictionary<string, System.Type>>();
        var isPropertyModel = new Dictionary<string, Dictionary<string, bool>>();

        var modelTypes = Assembly.GetExecutingAssembly()
            .GetTypes()
            .Where(t => t.GetCustomAttribute<ModelAttribute>() != null);
        foreach (var mt in modelTypes)
        {
            type[mt.Name] = mt;
            model[mt.Name] = mt.GetCustomAttribute<ModelAttribute>();
            property[mt.Name] = new Dictionary<string, PropertyInfo>();
            prop[mt.Name] = new Dictionary<string, PropAttribute>();
            isPropertyList[mt.Name] = new Dictionary<string, bool>();
            propertyItemType[mt.Name] = new Dictionary<string, System.Type>();
            isPropertyModel[mt.Name] = new Dictionary<string, bool>();

            foreach (var mtp in mt.GetProperties())
            {
                property[mt.Name][mtp.Name] = mtp;
                prop[mt.Name][mtp.Name] = mtp.GetCustomAttribute<PropAttribute>();
                var imtpl = mtp.PropertyType.IsGenericType &&
                            mtp.PropertyType.GetGenericTypeDefinition() == typeof(List<>);
                isPropertyList[mt.Name][mtp.Name] = imtpl;
                propertyItemType[mt.Name][mtp.Name] = imtpl ? mtp.PropertyType.GetGenericArguments()[0] : mtp.PropertyType;
                isPropertyModel[mt.Name][mtp.Name] = mtp.GetCustomAttribute<ModelPropAttribute>() != null;
            }
        }
        Type = type.ToImmutableDictionary();
        Model = model.ToImmutableDictionary();
        Property = property.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableDictionary());
        Prop = prop.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableDictionary());
        IsPropertyList = isPropertyList.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableDictionary());
        PropertyItemType = propertyItemType.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableDictionary());
        IsPropertyModel = isPropertyModel.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableDictionary());
    }
}
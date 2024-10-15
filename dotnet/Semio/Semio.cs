using System.Collections;
using System.Collections.Immutable;
using System.Linq.Expressions;
using System.Reflection;
using FluentValidation;
using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.Newtonsoft;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;

// TODO: Replace GetHashcode() with a proper hash function.
// TODO: Add logging mechanism to all API calls if they fail.
// TODO: Add a more detailed message system when a model is invalid.


namespace Semio;

#region Constants

public static class Constants
{
    public const int NameLengthLimit = 64;
    public const int IdLengthLimit = 128;
    public const int UrlLengthLimit = 2048;
    public const int DescriptionLengthLimit = 4096;
}

#endregion

#region Copilot

//GraphQL

//Dictionary

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
    public ModelAttribute(string emoji, string code, string abbreviation, string description)
        : base(emoji, code,
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
    public PropAttribute(string emoji, string code, string abbreviation, string description, PropImportance importance,
        bool isDefaultValid) : base(emoji, code,
        abbreviation, description)
    {
        Importance = importance;
        IsDefaultValid = isDefaultValid;
    }

    public PropImportance Importance { get; set; }
    public bool IsDefaultValid { get; set; }
}

public abstract class TextAttribute : PropAttribute
{
    public TextAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance, bool isDefaultValid, int lengthLimit) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid)
    {
        LengthLimit = lengthLimit;
    }

    public int LengthLimit { get; set; }
}

public class NameAttribute : TextAttribute
{
    public NameAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = false) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, Constants.NameLengthLimit)
    {
    }
}

public class IdAttribute : TextAttribute
{
    public IdAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.ID, bool isDefaultValid = false) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, Constants.IdLengthLimit)
    {
    }
}

public class UrlAttribute : TextAttribute
{
    public UrlAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, Constants.UrlLengthLimit)
    {
    }
}

public class DescriptionAttribute : TextAttribute
{
    public DescriptionAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, Constants.DescriptionLengthLimit)
    {
    }
}

public class IntPropAttribute : PropAttribute
{
    public IntPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid)
    {
    }
}

public class NumberPropAttribute : PropAttribute
{
    public NumberPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid)
    {
    }
}

public class ModelPropAttribute : PropAttribute
{
    public ModelPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.REQUIRED, bool isDefaultValid = true) : base(emoji, code,
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
        var nonEmptyIdPropertiesValues = nonEmptyIdProperties.Select(p => GetType().GetProperty(p)?.GetValue(this))
            .Cast<string>().ToList();
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
        return JsonConvert.DeserializeObject<T>(JsonConvert.SerializeObject(this));
    }

    public virtual (bool, List<string>) Validate()
    {
        var validator = new ModelValidator<T>();
        var result = validator.Validate((T)this);
        return (result.IsValid, result.Errors.Select(e => e.ToString()).ToList());
    }
}

public class ModelValidator<T> : AbstractValidator<T> where T : Model<T>
{
    public ModelValidator()
    {
        var modelTypeName = typeof(T).Name;
        var properties = Meta.Property[modelTypeName];
        for (var i = 0; i < properties.Length; i++)
        {
            var property = properties[i];
            var isPropertyList = Meta.IsPropertyList[modelTypeName][i];
            var isPropertyModel = Meta.IsPropertyModel[modelTypeName][i];
            if (isPropertyList)
            {
                var propAttribute = property.GetCustomAttribute<PropAttribute>();
                RuleFor(model => property.GetValue(model))
                    .NotEmpty()
                    .WithMessage($"The {property.Name} ({propAttribute.Code}) must have at least one.")
                    .When(m => propAttribute.Importance != PropImportance.OPTIONAL);
            }
            if (property.PropertyType == typeof(string))
            {
                var textAttribute = property.GetCustomAttribute<TextAttribute>();

                RuleFor(model => property.GetValue(model) as string)
                    .NotEmpty()
                    .When(m => !(textAttribute.Importance == PropImportance.OPTIONAL || textAttribute.IsDefaultValid))
                    .WithMessage($"The {property.Name}({textAttribute.Code}) must not be empty.")
                    .MaximumLength(textAttribute.LengthLimit)
                    .WithMessage(model =>
                    {
                        var value = property.GetValue(model) as string;
                        var preview = value?.Length > 10 ? value.Substring(0, 10) + "..." : value;
                        return
                            $"The {property.Name}({textAttribute.Code}) must be at most {textAttribute.LengthLimit} characters long. The provided text ({preview}) has {value?.Length} characters.";
                    });
            }
            else if (property.PropertyType == typeof(List<string>))
            {
                // TODO: Fix bug where multiple items fail for the same rule
                // On ["","","toooLonnngg","alsoToooLong"], only the first notEmtpy and the firstMaxLength are shown.

                var textAttribute = property.GetCustomAttribute<TextAttribute>();
                RuleForEach(list => property.GetValue(list) as List<string>)
                    .NotEmpty()
                    .When(m => !textAttribute.IsDefaultValid)
                    .WithMessage(item =>
                    {
                        return $"An element of {property.Name} ({textAttribute.Code}) must not be empty.";
                    })
                    .MaximumLength(textAttribute.LengthLimit)
                    .WithMessage((list, item) =>
                    {
                        var preview = item?.Length > 10 ? item.Substring(0, 10) + "..." : item;
                        return
                            $"An element of {property.Name} ({textAttribute.Code}) must be at most {textAttribute.LengthLimit} characters long. The provided one ({preview}) has {item?.Length} characters.";
                    })
                    .OverridePropertyName(property.Name);
            }
            else if (isPropertyModel && !isPropertyList)
            {
                // TODO: Implement validation for model properties
                //var validatorType = typeof(ModelValidator<>).MakeGenericType(property.PropertyType);
                //RuleFor(model => property.GetValue(model)).SetValidator((dynamic)Activator.CreateInstance(validatorType));

            }
            else if (isPropertyModel && isPropertyList)
            {
                
            }
        }
    }
}

/// <summary>
///     💾 A representation is an url that describes a type for a certain level of detail and tags.
/// </summary>
[Model("💾", "Rp", "Rep",
    "A representation is a linked file that describes a type for a certain level of detail and tags.")]
public class Representation : Model<Representation>
{
    /// <summary>
    ///     🔗 The Unique Resource Locator (URL) to another resource outside of semio.
    ///     absolute file path or a link.
    /// </summary>
    [Url("🔗", "Ur", "Url", "The Unique Resource Locator (URL) to another file outside of semio.", PropImportance.ID)]
    public string Url { get; set; } = "";

    /// <summary>
    ///     🏷️ The Multipurpose Internet Mail Extensions (MIME) type of the content of the file of the representation.
    /// </summary>
    [Id("🏷️", "Mm", "Mim",
        "The Multipurpose Internet Mail Extensions (MIME) type of the content of the file of the representation.",
        PropImportance.REQUIRED)]
    public string Mime { get; set; } = "";

    /// <summary>
    ///     🔍 The optional Level of Detail/Development/Design (LoD) of the representation.
    /// </summary>

    [Name("🔍", "Ld?", "Lod", "The optional Level of Detail/Development/Design (LoD) of the representation.",
        isDefaultValid: true)]
    public string Lod { get; set; } = "";

    /// <summary>
    ///     🔖 Optional tags to group representations.
    /// </summary>

    [Name("🔖", "Tg*", "Tags", "Optional tags to group representations.", isDefaultValid: false)]
    public List<string> Tags { get; set; } = new();
}

/// <summary>
///     🗺️ A locator is metadata for grouping ports.
/// </summary>
[Model("🗺️", "Lc", "Loc", "A locator is metadata for grouping ports.")]
public class Locator : Model<Locator>
{
    /// <summary>
    ///     👪 The group of the locator.
    /// </summary>
    [Name("👪", "Gr", "Grp", "The group of the locator.", PropImportance.ID)]
    public string Group { get; set; } = "";

    /// <summary>
    ///     📌 The optional sub-group of the locator. No sub-group means true.
    /// </summary>
    [Name("📌", "SG", "SGr", "The optional sub-group of the locator. No sub-group means true.", isDefaultValid: true)]
    public string Subgroup { get; set; } = "";
}

/// <summary>
///     📺 A 2d-point (xy) of integers in screen plane.
/// </summary>
[Model("📺", "SP", "SPt", "A 2d-point (xy) of integers in screen plane.")]
public class ScreenPoint : Model<ScreenPoint>
{
    [IntProp("🏁", "X", "X", "The x-coordinate of the screen point.", PropImportance.REQUIRED)]
    public int X { get; set; } = 0;

    [IntProp("🏁", "Y", "Y", "The y-coordinate of the screen point.", PropImportance.REQUIRED)]
    public int Y { get; set; } = 0;
}

/// <summary>
///     ❌ A 3-point (xyz) of floating point numbers.
/// </summary>
[Model("✖️", "Pt", "Pnt", "A 3-point (xyz) of floating point numbers.")]
public class Point : Model<Point>
{
    /// <summary>
    ///     🎚️ The x-coordinate of the point.
    /// </summary>
    [NumberProp("🎚️", "X", "X", "The x-coordinate of the point.", PropImportance.REQUIRED)]
    public float X { get; set; } = 0;

    /// <summary>
    ///     🎚️ The y-coordinate of the point.
    /// </summary>
    [NumberProp("🎚️", "Y", "Y", "The y-coordinate of the point.", PropImportance.REQUIRED)]
    public float Y { get; set; } = 0;

    /// <summary>
    ///     🎚️ The z-coordinate of the point.
    /// </summary>
    [NumberProp("🎚️", "Z", "Z", "The z-coordinate of the point.", PropImportance.REQUIRED)]
    public float Z { get; set; } = 0;
}

/// <summary>
///     ➡️ A 3d-vector (xyz) of floating point numbers.
/// </summary>
[Model("➡️", "Vc", "Vec", "A 3d-vector (xyz) of floating point numbers.")]
public class Vector : Model<Vector>
{
    /// <summary>
    ///     🎚️ The x-coordinate of the vector.
    /// </summary>
    [NumberProp("🎚️", "X", "X", "The x-coordinate of the vector.", PropImportance.REQUIRED)]
    public float X { get; set; } = 0;

    /// <summary>
    ///     🎚️ The y-coordinate of the vector.
    /// </summary>
    [NumberProp("🎚️", "Y", "Y", "The y-coordinate of the vector.", PropImportance.REQUIRED)]
    public float Y { get; set; } = 0;

    /// <summary>
    ///     🎚️ The z-coordinate of the vector.
    /// </summary>
    [NumberProp("🎚️", "Z", "Z", "The z-coordinate of the vector.", PropImportance.REQUIRED)]
    public float Z { get; set; } = 0;
}

/// <summary>
///     ◳ A plane is an origin (point) and an orientation (x-axis and y-axis).
/// </summary>
[Model("◳", "Pn", "Pln", "A plane is an origin (point) and an orientation (x-axis and y-axis).")]
public class Plane : Model<Plane>
{
    /// <summary>
    ///     ⌱ The origin of the plane.
    /// </summary>
    [ModelProp("⌱", "Og", "Org", "The origin of the plane.")]
    public Point Origin { get; set; } = new();

    /// <summary>
    ///     ➡️ The x-axis of the plane.
    /// </summary>
    [ModelProp("➡️", "XA", "XAx", "The x-axis of the plane.")]
    public Vector XAxis { get; set; } = new();

    /// <summary>
    ///     ➡️ The y-axis of the plane.
    /// </summary>
    [ModelProp("➡️", "YA", "YAx", "The y-axis of the plane.")]
    public Vector YAxis { get; set; } = new();
}

/// <summary>
///     🔌 A port is a connection point (with a direction) of a type.
/// </summary>
[Model("🔌", "Po", "Por", "A port is a connection point (with a direction) of a type.")]
public class Port : Model<Port>
{
    /// <summary>
    ///     🆔 The local identifier of the port within the type.
    /// </summary>
    [Id("🆔", "Id", "Idn", " identifier of the port within the type.")]
    public string Id { get; set; } = "";

    /// <summary>
    ///     ❌ The point of the port.
    /// </summary>
    [ModelProp("✖️", "Pt", "Pnt", "The point of the port.")]
    public Point Point { get; set; } = new();

    /// <summary>
    ///     ➡️ The direction of the port.
    /// </summary>
    [ModelProp("➡️", "Vc", "Vec", "The direction of the port.")]
    public Vector Direction { get; set; } = new();

    /// <summary>
    ///     🗺️ The optional locators of the port.
    /// </summary>
    [ModelProp("🗺️", "Lc*", "Locs", "The optional locators of the port.", PropImportance.OPTIONAL)]
    public List<Locator> Locators { get; set; } = new();
}

/// <summary>
///     🔌  identifier of the port within the type.
/// </summary>
[Model("🔌", "Po", "Por", " identifier of the port within the type.")]
public class PortId : Model<PortId>
{
    /// <summary>
    ///     🆔 The local identifier of the port within the type.
    /// </summary>
    [Id("🆔", "Id", "Id", " identifier of the port within the type.")]
    public string Id { get; set; } = "";

    public static implicit operator PortId(Port port)
    {
        return new PortId
        {
            Id = port.Id
        };
    }
}

/// <summary>
///     📏 A quality is meta-data for decision making.
/// </summary>
[Model("📏", "Ql", "Qal", "A quality is meta-data for decision making.")]
public class Quality : Model<Quality>
{
    /// <summary>
    ///     📛 The name of the quality.
    /// </summary>
    [Name("📏", "Na", "Nam", "The name of the quality.", PropImportance.ID)]
    public string Name { get; set; } = "";

    /// <summary>
    ///     🔢 The optional value of the quality. No value is equivalent to true for the name.
    /// </summary>
    [Url("🔢", "Vl?", "Val", "The optional value of the quality. No value is equivalent to true for the name.")]
    public string Value { get; set; } = "";

    /// <summary>
    ///     Ⓜ️ The optional unit of the value of the quality.
    /// </summary>
    [Name("Ⓜ️", "Ut", "Unt", "The optional unit of the value of the quality.", isDefaultValid: true)]
    public string Unit { get; set; } = "";

    /// <summary>
    ///     📖 The optional definition [text | url] of the quality.
    /// </summary>
    [Description("📖", "Df?", "Def", "The optional definition [text | url] of the quality.")]
    public string Definition { get; set; } = "";
}

/// <summary>
///     🧩 A type is a reusable element that can be connected with other types over ports.
/// </summary>
[Model("🧩", "Ty", "Typ", "A type is a reusable element that can be connected with other types over ports.")]
public class Type : Model<Type>
{
    /// <summary>
    ///     📛 Name of the type.
    /// </summary>
    [Name("📛", "Na", "Nam", "The name of the type.", PropImportance.ID)]
    public string Name { get; set; } = "";

    /// <summary>
    ///     💬 The optional human description of the type.
    /// </summary>
    [Description("💬", "Dc?", "Dsc", "The optional human description of the type.")]
    public string Description { get; set; } = "";

    /// <summary>
    ///     🖼️ The optional icon [emoji | text | image | svg] of the type.
    /// </summary>
    [Url("🖼️", "Ic?", "Ico", "The optional icon [emoji | text | image | svg] of the type.")]
    public string Icon { get; set; } = "";

    /// <summary>
    ///     🔀 The optional value of the type.
    /// </summary>
    [Name("🔀", "Vn?", "Vnt", "The optional value of the type.", PropImportance.ID, true)]
    public string Variant { get; set; } = "";

    /// <summary>
    ///     Ⓜ️ The length unit for all distance-related information of the type.
    /// </summary>
    [Name("Ⓜ️", "Ut", "Unt", "The length unit for all distance-related information of the type.",
        PropImportance.REQUIRED)]
    public string Unit { get; set; } = "";

    /// <summary>
    ///     🔌 The ports of the type.
    /// </summary>
    [ModelProp("🔌", "Po+", "Pors", "The ports of the type.", PropImportance.OPTIONAL)]
    public List<Port> Ports { get; set; } = new();

    /// <summary>
    ///     💾 The representations of the type.
    /// </summary>
    [ModelProp("💾", "Rp+", "Reps", "The representations of the type.", PropImportance.OPTIONAL)]
    public List<Representation> Representations { get; set; } = new();

    /// <summary>
    ///     📏 The optional qualities of the type.
    /// </summary>
    [ModelProp("📏", "Ql*", "Qualities", "The optional qualities of the type.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();
}

/// <summary>
///     🔌  identifier of the type within the kit.
/// </summary>
[Model("🧩", "Ty", "Typ", " identifier of the type within the kit.")]
public class TypeId : Model<TypeId>
{
    /// <summary>
    ///     📛 Name of the type.
    /// </summary>
    [Name("📛", "Na", "Nam", "The name of the type.", PropImportance.ID)]
    public string Name { get; set; } = "";

    /// <summary>
    ///     🔀 The optional value of the type.
    /// </summary>
    [Name("🔀", "Vn?", "Vnt", "The optional value of the type.", PropImportance.ID, true)]
    public string Variant { get; set; } = "";

    public static implicit operator TypeId(Type type)
    {
        return new TypeId
        {
            Name = type.Name,
            Variant = type.Variant
        };
    }
}

/// <summary>
///     ⭕ A piece is a 3d-instance of a type in a design.
/// </summary>
[Model("⭕", "Pc", "Pce", "A piece is a 3d-instance of a type in a design.")]
public class Piece : Model<Piece>
{
    /// <summary>
    ///     🆔 The local identifier of the piece within the design.
    /// </summary>
    [Id("🆔", "Id", "Id", "The local identifier of the piece within the design.")]
    public string Id { get; set; } = "";

    /// <summary>
    ///     🧩  identifier of the type within the kit.
    /// </summary>
    [ModelProp("🧩", "Ty", "Typ", "The local identifier of the type within the kit.")]
    public TypeId Type { get; set; } = new();

    /// <summary>
    ///     ◳ The optional plane of the piece. When pieces are connected only one piece can have a plane.
    /// </summary>
    [ModelProp("◳", "Pn", "Pln", "The optional plane of the piece. When pieces are connected only one piece can have a plane.", PropImportance.OPTIONAL)]
    public Plane Plane { get; set; } = new();

    /// <summary>
    ///     📺 The 2d-point (xy) of integers in screen plane of the center of the icon in the diagram of the piece.
    /// </summary>
    [ModelProp("📺", "SP", "SPt",
        "The 2d-point (xy) of integers in screen plane of the center of the icon in the diagram of the piece.")]
    public ScreenPoint ScreenPoint { get; set; } = new();
}

/// <summary>
///     🧩 The type-related information of the piece.
/// </summary>
[Model("🧩", "Ty", "Typ", "The type-related information of the piece in the side.")]
public class SidePieceType : Model<SidePieceType>
{
    /// <summary>
    ///     🔌 The local identification of the port within the type.
    /// </summary>
    [ModelProp("🔌", "Po", "Por", "The local identifier of the port within the type.")]
    public PortId Port { get; set; } = new();

    public static implicit operator SidePieceType(Port port)
    {
        return new SidePieceType
        {
            Port = port
        };
    }
}

/// <summary>
///     ⭕ The piece-related information of the side.
/// </summary>
[Model("⭕", "Pc", "Pce", "The piece-related information of the side.")]
public class SidePiece : Model<SidePiece>
{
    /// <summary>
    ///     🆔 The local identifier of the piece within the design.
    /// </summary>
    [Id("🆔", "Id", "Id", "The local identifier of the piece within the design.")]
    public string Id { get; set; } = "";

    /// <summary>
    ///     🆔 The type-related information of the piece.
    /// </summary>
    [ModelProp("🆔", "Ty", "Typ", "The type-related information of the piece.")]
    public SidePieceType Type { get; set; } = new();

    public static implicit operator SidePiece(Piece piece)
    {
        return new SidePiece
        {
            Id = piece.Id
        };
    }
}

/// <summary>
///     🧱 A side of a piece in a connection.
/// </summary>
[Model("🧱", "Sd", "Sde", "A side of a piece in a connection.")]
public class Side : Model<Side>
{
    /// <summary>
    ///     ⭕ The piece-related information of the side.
    /// </summary>
    [ModelProp("⭕", "Pc", "Pce", "The piece-related information of the side.")]
    public SidePiece Piece { get; set; } = new();
}

/// <summary>
///     🔗 A connection between two pieces in a design.
/// </summary>
[Model("🔗", "Cn", "Con", "A connection between two pieces in a design.")]
public class Connection : Model<Connection>
{
    /// <summary>
    ///     🧲 The connected side of the piece of the connection.
    /// </summary>
    [ModelProp("🧲", "Cd", "Cnd", "The connected side of the piece of the connection.")]
    public Side Connected { get; set; } = new();

    /// <summary>
    ///     🧲 The connected side of the piece of the connection.
    /// </summary>
    [ModelProp("🧲", "Cg", "Cng", "The connected side of the piece of the connection.")]
    public Side Connecting { get; set; } = new();

    /// <summary>
    ///     🔄 The optional rotation between the connected and the connecting piece in degrees.
    /// </summary>
    [NumberProp("🔄", "Rt", "Rot", "The optional rotation between the connected and the connecting piece in degrees.")]
    public float Rotation { get; set; } = 0;

    /// <summary>
    ///     🔄 The optional tilt (applied after rotation) between the connected and the connecting piece in degrees.
    /// </summary>
    [NumberProp("↗️", "Tl", "Tlt",
        "The optional tilt (applied after rotation) between the connected and the connecting piece in degrees.")]
    public float Tilt { get; set; } = 0;

    /// <summary>
    ///     🔄 The optional offset distance (in port direction after rotation and tilt) between the connected and the connecting
    ///     piece.
    /// </summary>
    [NumberProp("↕️", "Of", "Ofs",
        "The optional offset distance (in port direction after rotation and tilt) between the connected and the connecting piece.")]
    public float Offset { get; set; } = 0;
}

/// <summary>
///     🏙️ A design is a collection of pieces that are connected.
/// </summary>
[Model("🏙️", "Dn", "Dsn", "A design is a collection of pieces that are connected.")]
public class Design : Model<Design>
{
    /// <summary>
    ///     📛 The name of the design.
    /// </summary>
    [Name("📛", "Na", "Nam", "The name of the design.", PropImportance.ID)]
    public string Name { get; set; } = "";

    /// <summary>
    ///     💬 The optional human description of the design.
    /// </summary>
    [Description("💬", "Dc?", "Dsc", "The optional human description of the design.")]
    public string Description { get; set; } = "";

    /// <summary>
    ///     🖼️ The optional icon [emoji | text | image | svg] of the design.
    /// </summary>
    [Url("🖼️", "Ic?", "Ico", "The optional icon [emoji | text | image | svg] of the design.")]
    public string Icon { get; set; } = "";

    /// <summary>
    ///     🔀 The optional value of the design.
    /// </summary>
    [Name("🔀", "Vn?", "Vnt", "The optional value of the design.", PropImportance.ID, true)]
    public string Variant { get; set; } = "";

    /// <summary>
    ///     Ⓜ️ The length unit for all distance-related information of the design.
    /// </summary>
    [Name("Ⓜ️", "Ut", "Unt", "The length unit for all distance-related information of the design.",
        PropImportance.REQUIRED)]
    public string Unit { get; set; } = "";

    /// <summary>
    ///     ⭕ The pieces of the design.
    /// </summary>
    [ModelProp("⭕", "Pc+", "Pcs", "The pieces of the design.", PropImportance.OPTIONAL)]
    public List<Piece> Pieces { get; set; } = new();

    /// <summary>
    ///     🔗 The optional connections of the design.
    /// </summary>
    [ModelProp("🔗", "Co+", "Cons", "The optional connections of the design.", PropImportance.OPTIONAL)]
    public List<Connection> Connections { get; set; } = new();

    /// <summary>
    ///     📏 The optional qualities of the design.
    /// </summary>
    [ModelProp("📏", "Ql*", "Qualities", "The optional qualities of the design.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();

    //public Design Flatten(Type[] types = null)
    //{
    //    Design flattenedDesign = this.DeepClone();
    //    if (Pieces.Count <= 1 || Connections.Count == 0)
    //        return flattenedDesign;
    //    var graph = new UndirectedGraph<string, Edge<string>>();
    //    foreach (var piece in Pieces)
    //        graph.AddVertex(piece.Id);
    //    foreach (var connection in Connections)
    //        graph.AddEdge(new Edge<string>(connection.Connected.Piece.Id, connection.Connecting.Piece.Id));
    //    var root = Pieces.First(p => p.Root != null) ?? Pieces.First();
    //    var components = new Dictionary<string, int>();
    //    graph.ConnectedComponents(components);
    //    return flattenedDesign;
    //}
}

/// <summary>
///     🧰 A kit is a collection of types and designs.
/// </summary>
[Model("🧰", "Kt", "Kit", "A kit is a collection of types and designs.")]
public class Kit : Model<Kit>
{
    /// <summary>
    ///     📛 Name of the kit.
    /// </summary>
    [Name("📛", "Na", "Nam", "The name of the kit.", PropImportance.ID)]
    public string Name { get; set; } = "";

    /// <summary>
    ///     💬 The optional human description of the kit.
    /// </summary>
    [Description("💬", "Dc?", "Dsc", "The optional human description of the kit.")]
    public string Description { get; set; } = "";

    /// <summary>
    ///     🖼️ The optional icon [emoji | text | image | svg] of the design.
    /// </summary>
    [Url("🖼️", "Ic?", "Ico", "The optional icon [emoji | text | image | svg] of the design.")]
    public string Icon { get; set; } = "";

    /// <summary>
    ///     🔗 The optional Unique Resource Locator (URL) where to fetch the kit.
    /// </summary>
    [Url("🔗", "Ur?", "Url", "The optional Unique Resource Locator (URL) where to fetch the kit.")]
    public string Url { get; set; } = "";

    /// <summary>
    ///     🏠 The optional Unique Resource Locator (URL) of the homepage of the kit.
    /// </summary>
    [Url("🏠", "Hp?", "Hmp", "The optional Unique Resource Locator (URL) of the homepage of the kit.")]
    public string Homepage { get; set; } = "";

    /// <summary>
    ///     🧩 The optional types of the kit.
    /// </summary>
    [ModelProp("🧩", "Ty*", "Typs", "The optional types of the kit.", PropImportance.OPTIONAL)]
    public List<Type> Types { get; set; } = new();

    /// <summary>
    ///     🏙️ The optional designs of the kit.
    /// </summary>
    [ModelProp("🏙️", "Dn*", "Dsns", "The optional designs of the kit.", PropImportance.OPTIONAL)]
    public List<Design> Designs { get; set; } = new();
}

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

#region Api

public class LoadKitResponse
{
    public Kit? Kit { get; set; }
    public string? Error { get; set; }
}

public class LoadKitResponseContainer
{
    public LoadKitResponse LoadKit { get; set; }
}

public enum CreateKitErrorCode
{
    KIT_URL_IS_INVALID,
    NO_PERMISSION_TO_CREATE_KIT,
    KIT_INPUT_IS_INVALID
}

public class CreateKitError
{
    public CreateKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class CreateKitResponse
{
    public Kit? Kit { get; set; }
    public CreateKitError? Error { get; set; }
}

public class CreateKitResponseContainer
{
    public CreateKitResponse CreateKit { get; set; }
}

public enum UpdateKitPropsErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_UPDATE_KIT,
    KIT_METADATA_IS_INVALID
}

public class UpdateKitPropsError
{
    public UpdateKitPropsErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class UpdateKitPropsResponse
{
    public KitProps? Kit { get; set; }
    public UpdateKitPropsError? Error { get; set; }
}

public class UpdateKitPropsResponseContainer
{
    public UpdateKitPropsResponse UpdateKitProps { get; set; }
}

public enum DeleteKitError
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_DELETE_KIT
}

public class DeleteKitResponse
{
    public DeleteKitError? Error { get; set; }
}

public class DeleteKitResponseContainer
{
    public DeleteKitResponse DeleteKit { get; set; }
}

public enum AddTypeToKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    TYPE_INPUT_IS_INVALID
}

public class AddTypeToKitError
{
    public AddTypeToKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class AddTypeToKitResponse
{
    public Type? Type { get; set; }
    public AddTypeToKitError? Error { get; set; }
}

public class AddTypeToKitResponseContainer
{
    public AddTypeToKitResponse AddTypeToKit { get; set; }
}

public enum RemoveTypeFromKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    TYPE_DOES_NOT_EXIST,
    DESIGN_DEPENDS_ON_TYPE
}

public class RemoveTypeFromKitError
{
    public RemoveTypeFromKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class RemoveTypeFromKitResponse
{
    public RemoveTypeFromKitError? Error { get; set; }
}

public class RemoveTypeFromKitResponseContainer
{
    public RemoveTypeFromKitResponse RemoveTypeFromKit { get; set; }
}

public enum AddDesignToKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    DESIGN_INPUT_IS_INVALID
}

public class AddDesignToKitError
{
    public AddDesignToKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class AddDesignToKitResponse
{
    public Design? Design { get; set; }
    public AddDesignToKitError? Error { get; set; }
}

public class AddDesignToKitResponseContainer
{
    public AddDesignToKitResponse AddDesignToKit { get; set; }
}

public enum RemoveDesignFromKitErrorCode
{
    DIRECTORY_DOES_NOT_EXIST,
    DIRECTORY_IS_NOT_A_DIRECTORY,
    DIRECTORY_HAS_NO_KIT,
    NO_PERMISSION_TO_MODIFY_KIT,
    DESIGN_DOES_NOT_EXIST
}

public class RemoveDesignFromKitError
{
    public RemoveDesignFromKitErrorCode Code { get; set; }
    public string Message { get; set; }
}

public class RemoveDesignFromKitResponse
{
    public RemoveDesignFromKitError? Error { get; set; }
}

public class RemoveDesignFromKitResponseContainer
{
    public RemoveDesignFromKitResponse RemoveDesignFromKit { get; set; }
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

    public LoadKitResponse? LoadKit(string directory)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.loadKit,
            OperationName = "LoadKit",
            Variables = new { directory }
        };
        var response = Client.SendQueryAsync<LoadKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.LoadKit;
    }

    public CreateKitResponse? CreateKit(string directory, Kit kit)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.createKit,
            OperationName = "CreateKit",
            Variables = new { directory, kit }
        };
        var response = Client.SendQueryAsync<CreateKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.CreateKit;
    }

    public UpdateKitPropsResponse? UpdateKitProps(string directory, KitProps kit)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.updateKitMetadata,
            OperationName = "UpdateKitProps",
            Variables = new { directory, kit }
        };
        var response = Client.SendQueryAsync<UpdateKitPropsResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.UpdateKitProps;
    }

    public DeleteKitResponse? DeleteKit(string directory)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.deleteKit,
            OperationName = "DeleteKit",
            Variables = new { directory }
        };
        var response = Client.SendQueryAsync<DeleteKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.DeleteKit;
    }

    public AddTypeToKitResponse? AddTypeToKit(string directory, Type type)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.addTypeToKit,
            OperationName = "AddTypeToKit",
            Variables = new { directory, type }
        };
        var response = Client.SendQueryAsync<AddTypeToKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.AddTypeToKit;
    }

    public RemoveTypeFromKitResponse? RemoveTypeFromKit(string directory, TypeId type)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.removeTypeFromKit,
            OperationName = "RemoveTypeFromKit",
            Variables = new { directory, type }
        };
        var response = Client.SendQueryAsync<RemoveTypeFromKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.RemoveTypeFromKit;
    }

    public AddDesignToKitResponse? AddDesignToKit(string directory, Design design)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.addDesignToKit,
            OperationName = "AddDesignToKit",
            Variables = new { directory, design }
        };
        var response = Client.SendQueryAsync<AddDesignToKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.AddDesignToKit;
    }

    public RemoveDesignFromKitResponse? RemoveDesignFromKit(string directory, DesignId design)
    {
        var query = new GraphQLRequest
        {
            Query = Resources.removeDesignFromKit,
            OperationName = "RemoveDesignFromKit",
            Variables = new { directory, design }
        };
        var response = Client.SendQueryAsync<RemoveDesignFromKitResponseContainer>(query).Result;
        if (response.Errors != null) return null;
        return response.Data.RemoveDesignFromKit;
    }

}

#endregion

public static class Meta
{
    /// <summary>
    ///     Name of the model : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, System.Type> Type;

    /// <summary>
    ///     Name of the model : ModelAttribute
    /// </summary>
    public static readonly ImmutableDictionary<string, ModelAttribute> Model;

    /// <summary>
    ///     Name of the model : Index of the property : PropertyInfo
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<PropertyInfo>> Property;

    /// <summary>
    ///     Name of the model : Index of the property : PropAttribute
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<PropAttribute>> Prop;

    /// <summary>
    ///     Name of the model : Index of the property : IsList
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<bool>> IsPropertyList;

    /// <summary>
    ///     Name of the model : Index of the property : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<System.Type>> PropertyItemType;

    /// <summary>
    ///     Name of the model : Index of the property : IsModel
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<bool>> IsPropertyModel;

    static Meta()
    {
        var type = new Dictionary<string, System.Type>();
        var model = new Dictionary<string, ModelAttribute>();
        var property = new Dictionary<string, List<PropertyInfo>>();
        var prop = new Dictionary<string, List<PropAttribute>>();
        var isPropertyList = new Dictionary<string, List<bool>>();
        var propertyItemType = new Dictionary<string, List<System.Type>>();
        var isPropertyModel = new Dictionary<string, List<bool>>();

        var modelTypes = Assembly.GetExecutingAssembly()
            .GetTypes()
            .Where(t => t.GetCustomAttribute<ModelAttribute>() != null);
        foreach (var mt in modelTypes)
        {
            type[mt.Name] = mt;
            model[mt.Name] = mt.GetCustomAttribute<ModelAttribute>();
            property[mt.Name] = new List<PropertyInfo>();
            prop[mt.Name] = new List<PropAttribute>();
            isPropertyList[mt.Name] = new List<bool>();
            propertyItemType[mt.Name] = new List<System.Type>();
            isPropertyModel[mt.Name] = new List<bool>();

            foreach (var mtp in mt.GetProperties()
                         .Where(mtp => mtp.GetCustomAttribute<PropAttribute>() != null))
            {
                property[mt.Name].Add(mtp);
                prop[mt.Name].Add(mtp.GetCustomAttribute<PropAttribute>());
                var imtpl = mtp.PropertyType.IsGenericType &&
                            mtp.PropertyType.GetGenericTypeDefinition() == typeof(List<>);
                isPropertyList[mt.Name].Add(imtpl);
                propertyItemType[mt.Name].Add(imtpl ? mtp.PropertyType.GetGenericArguments()[0] : mtp.PropertyType);
                isPropertyModel[mt.Name].Add(mtp.GetCustomAttribute<ModelPropAttribute>() != null);
            }
        }

        Type = type.ToImmutableDictionary();
        Model = model.ToImmutableDictionary();
        Property = property.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        Prop = prop.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        IsPropertyList = isPropertyList.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        PropertyItemType = propertyItemType.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        IsPropertyModel = isPropertyModel.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
    }
}
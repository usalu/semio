//Semio.cs
//Copyright (C) 2024 Ueli Saluz

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU Affero General Public License as
//published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU Affero General Public License for more details.

//You should have received a copy of the GNU Affero General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.

using System.Collections;
using System.Collections.Immutable;
using System.Net;
using System.Reflection;
using FluentValidation;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using QuikGraph;
using QuikGraph.Algorithms;
using QuikGraph.Algorithms.Search;
using Refit;

// TODO: Replace GetHashcode() with a proper hash function.
// TODO: Add logging mechanism to all API calls if they fail.
// TODO: Implement reflexive validation for model properties.
// TODO: Add index to prop and add to list based on index not on source code order.
// TODO: See if Utility.Encode(uri) can be added by attribute on parameters.

namespace Semio;

#region Constants

public static class Constants
{
    public const int NameLengthLimit = 64;
    public const int IdLengthLimit = 128;
    public const int UrlLengthLimit = 2048;
    public const int DescriptionLengthLimit = 4096;
    public const string Release = "r24.11-1";
    public const int EnginePort = 24111;
    public const string EngineAddress = "http://127.0.0.1:24111";
    public const float Tolerance = 1e-5f;
}

#endregion

#region Copilot

//GraphQL

//Dictionary

#endregion

#region Utility

public static class Utility
{
    public static string ParseMimeFromUrl(string url)
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

    public static string Encode(string text)
    {
        return Uri.EscapeDataString(text);
    }

    public static string Decode(string text)
    {
        return Uri.UnescapeDataString(text);
    }

    public static string Serialize(this object obj, bool indented = false)
    {
        return JsonConvert.SerializeObject(
            obj, indented ? Formatting.Indented : Formatting.None, new JsonSerializerSettings
            {
                ContractResolver = new CamelCasePropertyNamesContractResolver()
            });
    }

    public static T Deserialize<T>(this string json)
    {
        return JsonConvert.DeserializeObject<T>(
            json, new JsonSerializerSettings
            {
                ContractResolver = new CamelCasePropertyNamesContractResolver()
            });
    }


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
        bool isDefaultValid, bool skipValidation) : base(emoji, code,
        abbreviation, description)
    {
        Importance = importance;
        IsDefaultValid = isDefaultValid;
        SkipValidation = skipValidation;
    }

    public PropImportance Importance { get; set; }
    public bool IsDefaultValid { get; set; }
    public bool SkipValidation { get; set; }
}

public abstract class TextAttribute : PropAttribute
{
    public TextAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance, bool isDefaultValid, bool skipValidation, int lengthLimit) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, skipValidation)
    {
        LengthLimit = lengthLimit;
    }

    public int LengthLimit { get; set; }
}

public class NameAttribute : TextAttribute
{
    public NameAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = false, bool skipValidation = false) :
        base(emoji, code,
            abbreviation, description, importance, isDefaultValid, skipValidation, Constants.NameLengthLimit)
    {
    }
}

public class IdAttribute : TextAttribute
{
    public IdAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.ID, bool isDefaultValid = false, bool skipValidation = false) : base(
        emoji, code,
        abbreviation, description, importance, isDefaultValid, skipValidation, Constants.IdLengthLimit)
    {
    }
}

public class UrlAttribute : TextAttribute
{
    public UrlAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = false, bool skipValidation = false) :
        base(emoji, code,
            abbreviation, description, importance, isDefaultValid, skipValidation, Constants.UrlLengthLimit)
    {
    }
}

public class DescriptionAttribute : TextAttribute
{
    public DescriptionAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code,
            abbreviation, description, importance, isDefaultValid, skipValidation, Constants.DescriptionLengthLimit)
    {
    }
}

public class BoolAttribute : PropAttribute
{
    public BoolAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code,
            abbreviation, description, importance, isDefaultValid, skipValidation)
    {
    }
}

public class IntPropAttribute : PropAttribute
{
    public IntPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code,
            abbreviation, description, importance, isDefaultValid, skipValidation)
    {
    }
}

public class NumberPropAttribute : PropAttribute
{
    public NumberPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code,
            abbreviation, description, importance, isDefaultValid, skipValidation)
    {
    }
}

public class AnglePropAttribute : NumberPropAttribute
{
    public AnglePropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code,
            abbreviation, description, importance, isDefaultValid, skipValidation)
    {
    }
}

public class ModelPropAttribute : PropAttribute
{
    public ModelPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.REQUIRED, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code,
            abbreviation, description, importance, isDefaultValid, skipValidation)
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
        if (nonEmptyIdPropertiesValues.Count!=0)
            return $"{modelAttribute.Abbreviation}({string.Join(", ", nonEmptyIdPropertiesValues)})";
        var requiredProperties = GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(p => p.GetCustomAttribute<PropAttribute>()?.Importance == PropImportance.REQUIRED)
            .Select(p => p.Name);
        var requiredPropertiesValues = requiredProperties.Select(p => GetType().GetProperty(p)?.GetValue(this))
            .Select(v=>v.ToString()).ToList();
        return $"{modelAttribute.Abbreviation}({string.Join(", ", requiredPropertiesValues)})";
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

    public T DeepClone()
    {
        return this.Serialize().Deserialize<T>();
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
            ValidateProperty(property, isPropertyList, isPropertyModel);
        }
    }

    private void ValidateProperty(PropertyInfo property, bool isPropertyList, bool isPropertyModel)
    {
        var propAttribute = property.GetCustomAttribute<PropAttribute>();
        if (propAttribute.SkipValidation)
            return;
        if (isPropertyList)
            RuleFor(model => property.GetValue(model))
                .NotEmpty()
                .WithMessage($"The {property.Name.ToLower()} must have at least one.")
                .When(m => propAttribute.Importance != PropImportance.OPTIONAL);

        if (property.PropertyType == typeof(float))
        {
            var numberAttribute = property.GetCustomAttribute<NumberPropAttribute>();
            var isAngle = property.GetCustomAttribute<AnglePropAttribute>() != null;
            if (isAngle)
                RuleFor(model => property.GetValue(model) as float?)
                    .GreaterThanOrEqualTo(0)
                    .WithMessage($"The {property.Name.ToLower()} must be at least 0 degrees.")
                    .LessThan(360)
                    .WithMessage($"The {property.Name.ToLower()} must be less than 360 degrees.");
        }
        else if (property.PropertyType == typeof(string))
        {
            var textAttribute = property.GetCustomAttribute<TextAttribute>();

            RuleFor(model => property.GetValue(model) as string)
                .NotEmpty()
                .When(m => !(textAttribute.Importance == PropImportance.OPTIONAL || textAttribute.IsDefaultValid))
                .WithMessage($"The {property.Name.ToLower()} must not be empty.")
                .MaximumLength(textAttribute.LengthLimit)
                .WithMessage(model =>
                {
                    var value = property.GetValue(model) as string;
                    var preview = value?.Length > 10 ? value.Substring(0, 10) + "..." : value;
                    return
                        $"The {property.Name.ToLower()} must be at most {textAttribute.LengthLimit} characters long. The provided text ({preview}) has {value?.Length} characters.";
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
                    var singularPropertyName = property.Name.ToLower().TrimEnd('s');
                    return $"A {singularPropertyName} must not be empty.";
                })
                .MaximumLength(textAttribute.LengthLimit)
                .WithMessage((list, item) =>
                {
                    var preview = item?.Length > 10 ? item.Substring(0, 10) + "..." : item;
                    var singularPropertyName = property.Name.ToLower().TrimEnd('s');
                    return
                        $"A {singularPropertyName} must be at most {textAttribute.LengthLimit} characters long. The provided {singularPropertyName} ({preview}) has {item?.Length} characters.";
                })
                .OverridePropertyName(property.Name);
        }
        else if (isPropertyModel && !isPropertyList)
        {
            // TODO: Implement reflexive validation for model properties.
            //var validatorType = typeof(ModelValidator<>).MakeGenericType(property.PropertyType);
            //RuleFor(model => property.GetValue(model)).SetValidator((dynamic)Activator.CreateInstance(validatorType));
        }
        else if (isPropertyModel && isPropertyList)
        {
            // TODO: Implement reflexive validation for model properties.
        }
    }
}

/// <summary>
///     💾 A representation is a link to a resource that describes a type for a certain level of detail and tags.
/// </summary>
[Model("💾", "Rp", "Rep",
    "A representation is a link to a resource that describes a type for a certain level of detail and tags.")]
public class Representation : Model<Representation>
{
    /// <summary>
    ///     🔗 The Unique Resource Locator (URL) to the resource of the representation.
    /// </summary>
    [Url("🔗", "Ur", "Url", "The Unique Resource Locator (URL) to the resource of the representation.")]
    public string Url { get; set; } = "";

    /// <summary>
    ///     ✉️ The Multipurpose Internet Mail Extensions (MIME) type of the content of the resource of the representation.
    /// </summary>
    [Id("✉️", "Mm", "Mim",
        "The Multipurpose Internet Mail Extensions (MIME) type of the content of the resource of the representation.")]
    public string Mime { get; set; } = "";

    /// <summary>
    ///     🔍 The optional Level of Detail/Development/Design (LoD) of the representation. No lod means default.
    /// </summary>

    [Name("🔍", "Ld?", "Lod",
        "The optional Level of Detail/Development/Design (LoD) of the representation. No lod means default.",
        PropImportance.ID,
        true)]
    public string Lod { get; set; } = "";

    /// <summary>
    ///     🏷️ The optional tags to group representations. No tags means default.
    /// </summary>

    [Name("🏷️", "Tg*", "Tags", "The optional tags to group representations. No tags means default.", PropImportance.ID,
        skipValidation: true)]
    public List<string> Tags { get; set; } = new();

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var tag in Tags)
        {
            if (tag.Length == 0)
            {
                isValid = false;
                errors.Add("The tag must not be empty.");
            }

            if (tag.Length > Constants.NameLengthLimit)
            {
                isValid = false;
                var preview = tag.Length > 10 ? tag.Substring(0, 10) + "..." : tag;
                errors.Add(
                    $"A tag must be at most {Constants.NameLengthLimit} characters long. The provided tag ({preview}) has {tag.Length} characters.");
            }
        }

        return (isValid, errors);
    }

    public override string ToString()
    {
        var lod = Lod == "" ? "" : ", " + Lod;
        var tags = Tags.Count == 0 ? "" : ", " + string.Join(" ", Tags);
        return $"Rep({Mime}{lod}{tags})";
    }
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
    [Name("📌", "SG?", "SGr", "The optional sub-group of the locator. No sub-group means true.", isDefaultValid: true)]
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
    public float X { get; set; } = 1;

    /// <summary>
    ///     🎚️ The y-coordinate of the vector.
    /// </summary>
    [NumberProp("🎚️", "Y", "Y", "The y-coordinate of the vector.", PropImportance.REQUIRED)]
    public float Y { get; set; }

    /// <summary>
    ///     🎚️ The z-coordinate of the vector.
    /// </summary>
    [NumberProp("🎚️", "Z", "Z", "The z-coordinate of the vector.", PropImportance.REQUIRED)]
    public float Z { get; set; } = 0;

    public static float DotProduct(Vector a, Vector b)
    {
        return a.X * b.X + a.Y * b.Y + a.Z * b.Z;
    }

    public static bool IsOrthogonal(Vector a, Vector b)
    {
        return Math.Abs(DotProduct(a, b)) < Constants.Tolerance;
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (Math.Abs(X) < Constants.Tolerance && Math.Abs(Y) < Constants.Tolerance && Math.Abs(Z) < Constants.Tolerance)
        {
            isValid = false;
            errors.Add("The vector must not be the zero vector.");
        }

        if (Math.Abs(Math.Sqrt(X * X + Y * Y + Z * Z) - 1) > Constants.Tolerance)
        {
            isValid = false;
            errors.Add("The vector must be a unit vector.");
        }

        return (isValid, errors);
    }
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
    public Vector YAxis { get; set; } = new() { Y = 1 };

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        var (isValidOrigin,errorsOrigin) = Origin.Validate();
        isValid = isValid && isValidOrigin;
        errors.AddRange(errorsOrigin.Select(e => "The origin is invalid: " + e));
        var (isValidXAxis, errorsXAxis) = XAxis.Validate();
        isValid = isValid && isValidXAxis;
        errors.AddRange(errorsXAxis.Select(e => "The x-axis is invalid: " + e));
        var (isValidYAxis, errorsYAxis) = YAxis.Validate();
        isValid = isValid && isValidYAxis;
        errors.AddRange(errorsYAxis.Select(e => "The y-axis is invalid: " + e));
        if (!Vector.IsOrthogonal(XAxis, YAxis))
        {
            isValid = false;
            errors.Add("The x-axis and y-axis must be orthogonal.");
        }

        return (isValid, errors);
    }
}

/// <summary>
///     🔌 A port is a connection point (with a direction) of a type.
/// </summary>
[Model("🔌", "Po", "Por", "A port is a connection point (with a direction) of a type.")]
public class Port : Model<Port>
{
    /// <summary>
    ///     🆔 The optional local identifier of the port within the type. No id means the default port.
    /// </summary>
    [Id("🆔", "Id?", "Idn", "The optional local identifier of the port within the type. No id means the default port.",
        isDefaultValid: true)]
    [JsonProperty("id_")]
    public string Id { get; set; } = "";

    /// <summary>
    ///     ❌ The connection point of the port that is attracted to another connection point.
    /// </summary>
    [ModelProp("✖️", "Pt", "Pnt", "The connection point of the port that is attracted to another connection point.")]
    public Point? Point { get; set; } = null;

    /// <summary>
    ///     ➡️ The direction of the port. The direction of the other port will be flipped and then the pieces will be aligned.
    /// </summary>
    [ModelProp("➡️", "Dr", "Drn",
        "The direction of the port. The direction of the other port will be flipped and then the pieces will be aligned.")]
    public Vector? Direction { get; set; } = null;

    /// <summary>
    ///     🗺️ The optional locators of the port.
    /// </summary>
    [ModelProp("🗺️", "Lc*", "Locs", "The optional locators of the port.", PropImportance.OPTIONAL)]
    public List<Locator> Locators { get; set; } = new();

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (Point != null)
        {
            var (isValidPoint, errorsPoint) = Point.Validate();
            isValid = isValid && isValidPoint;
            errors.AddRange(errorsPoint.Select(e => "The point is invalid: " + e));
        }
        else
        {
            isValid = false;
            errors.Add("The point must not be null.");
        }

        if (Direction != null)
        {
            var (isValidDirection, errorsDirection) = Direction.Validate();
            isValid = isValid && isValidDirection;
            errors.AddRange(errorsDirection.Select(e => "The direction is invalid: " + e));
        }
        else
        {
            isValid = false;
            errors.Add("The direction must not be null.");
        }

        if (Locators.Count != 0)
        {
            foreach (var locator in Locators)
            {
                var (isValidLocator, errorsLocator) = locator.Validate();
                isValid = isValid && isValidLocator;
                errors.AddRange(errorsLocator.Select(e => "A locator is invalid: " + e));
            }
        }

        return (isValid, errors);
    }
}

/// <summary>
///     🔌 The optional local identifier of the port within the type. No id means the default port.
/// </summary>
[Model("🔌", "Po", "Por", "The optional local identifier of the port within the type. No id means the default port.")]
public class PortId : Model<PortId>
{
    /// <summary>
    ///     🆔 The optional local identifier of the port within the type. No id means the default port.
    /// </summary>
    [Id("🆔", "Id?", "Id", "The local identifier of the port within the type.", isDefaultValid: true)]
    [JsonProperty("id_")]
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
    ///     🔢 The optional value [ text | url ] of the quality. No value is equivalent to true for the name.
    /// </summary>
    [Description("🔢", "Vl?", "Val",
        "The optional value [ text | url ] of the quality. No value is equivalent to true for the name.")]
    public string Value { get; set; } = "";

    /// <summary>
    ///     Ⓜ️ The optional unit of the value of the quality.
    /// </summary>
    [Name("Ⓜ️", "Ut", "Unt", "The optional unit of the value of the quality.", isDefaultValid: true)]
    public string Unit { get; set; } = "";

    /// <summary>
    ///     📖 The optional definition [ text | url ] of the quality.
    /// </summary>
    [Description("📖", "Df?", "Def", "The optional definition [ text | url ] of the quality.")]
    public string Definition { get; set; } = "";
}

public class TypeProps : Model<Type>
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
    ///     🖼️ The optional icon [ emoji | name | url ] of the type.
    /// </summary>
    [Url("🖼️", "Ic?", "Ico", "The optional icon [ emoji | name | url ] of the type.")]
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
}

/// <summary>
///     🧩 A type is a reusable element that can be connected with other types over ports.
/// </summary>
[Model("🧩", "Ty", "Typ", "A type is a reusable element that can be connected with other types over ports.")]
public class Type : TypeProps
{
    /// <summary>
    ///     💾 The representations of the type.
    /// </summary>
    [ModelProp("💾", "Rp+", "Reps", "The representations of the type.")]
    public List<Representation> Representations { get; set; } = new();

    /// <summary>
    ///     🔌 The ports of the type.
    /// </summary>
    [ModelProp("🔌", "Po+", "Pors", "The ports of the type.")]
    public List<Port> Ports { get; set; } = new();

    /// <summary>
    ///     📏 The optional qualities of the type.
    /// </summary>
    [ModelProp("📏", "Ql*", "Qualities", "The optional qualities of the type.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var port in Ports)
        {
            var(isValidPort, errorsPort) = port.Validate();
            isValid = isValid && isValidPort;
            errors.AddRange(errorsPort.Select(e => "A port is invalid: " + e));
        }

        foreach (var representation in Representations)
        {
            var(isValidRepresentation, errorsRepresentation) = representation.Validate();
            isValid = isValid && isValidRepresentation;
            errors.AddRange(errorsRepresentation.Select(e => "A representation is invalid: " + e));
        }

        foreach (var quality in Qualities)
        {
            var (isValidQuality, errorsQuality) = quality.Validate();
            isValid = isValid && isValidQuality;
            errors.AddRange(errorsQuality.Select(e => "A quality is invalid: " + e));
        }

        return (isValid, errors);
    }
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
    ///     🆔 The optional local identifier of the piece within the design. No id means the default piece.
    /// </summary>
    [Id("🆔", "Id?", "Id",
        "The optional local identifier of the piece within the design. No id means the default piece.",
        isDefaultValid: true)]
    [JsonProperty("id_")]
    public string Id { get; set; } = "";

    /// <summary>
    ///     🧩 The local identifier of the type of the piece within the kit.
    /// </summary>
    [ModelProp("🧩", "Ty", "Typ", "The local identifier of the type of the piece within the kit.")]
    public TypeId Type { get; set; } = new();

    /// <summary>
    ///     ◳ The optional plane of the piece. When pieces are connected only one piece can have a plane.
    /// </summary>
    [ModelProp("◳", "Pn?", "Pln",
        "The optional plane of the piece. When pieces are connected only one piece can have a plane.",
        PropImportance.OPTIONAL)]
    public Plane? Plane { get; set; }

    /// <summary>
    ///     📺 The 2d-point (xy) of integers in screen plane of the center of the icon in the diagram of the piece.
    /// </summary>
    [ModelProp("📺", "SP", "SPt",
        "The 2d-point (xy) of integers in screen plane of the center of the icon in the diagram of the piece.",
        PropImportance.OPTIONAL)]
    public ScreenPoint ScreenPoint { get; set; } = new();

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        var (isValidType, errorsType) = Type.Validate();
        isValid = isValid && isValidType;
        errors.AddRange(errorsType.Select(e => "The type is invalid: " + e));
        if (Plane != null)
        {
            var (isValidPlane, errorsPlane) = Plane.Validate();
            isValid = isValid && isValidPlane;
            errors.AddRange(errorsPlane.Select(e => "The plane is invalid: " + e));
        }
        return (isValid, errors);
    }
}

[Model("⭕", "Pc", "Pce",
    "The optional local identifier of the piece within the design. No id means the default piece.")]
public class PieceId : Model<PieceId>
{
    /// <summary>
    ///     🆔 The optional local identifier of the piece within the design. No id means the default piece.
    /// </summary>
    [Id("🆔", "Id?", "Id",
        "The optional local identifier of the piece within the design. No id means the default piece.",
        isDefaultValid: true)]
    [JsonProperty("id_")]
    public string Id { get; set; } = "";
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
    public PieceId Piece { get; set; } = new();

    /// <summary>
    ///     🔌 The local identification of the port within the type.
    /// </summary>
    [ModelProp("🔌", "Po", "Por", "The local identifier of the port within the type.")]
    public PortId Port { get; set; } = new();

    public override string ToString()
    {
        return $"Sde({Piece.Id}" + (Port.Id != "" ? ":" + Port.Id : "") + ")";
    }
}

/// <summary>
///     🔗 A connection between two pieces in a design.
/// </summary>
[Model("🔗", "Co", "Con", "A connection between two pieces in a design.")]
public class Connection : Model<Connection>
{
    private float _rotation;
    private float _tilt;

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
    [AngleProp("🔄", "Rt?", "Rot", "The optional rotation between the connected and the connecting piece in degrees.")]
    public float Rotation
    {
        get => _rotation;
        set => _rotation = (value % 360 + 360) % 360;
    }

    /// <summary>
    ///     🔄 The optional tilt (applied after rotation) between the connected and the connecting piece in degrees.
    /// </summary>
    [AngleProp("↗️", "Tl?", "Tlt",
        "The optional tilt (applied after rotation) between the connected and the connecting piece in degrees.")]
    public float Tilt
    {
        get => _tilt;
        set => _tilt = (value % 360 + 360) % 360;
    }

    /// <summary>
    ///     🔄 The optional offset distance (in port direction after rotation and tilt) between the connected and the
    ///     connecting
    ///     piece.
    /// </summary>
    [NumberProp("↕️", "Of?", "Ofs",
        "The optional offset distance (applied after rotation and tilt in port direction) between the connected and the connecting piece.")]
    public float Offset { get; set; } = 0;

    public override string ToString()
    {
        var ctd = Connected.Piece.Id + (Connected.Port.Id != "" ? ":" + Connected.Port.Id : "");
        var cng = (Connecting.Port.Id != "" ? Connecting.Port.Id + ":" : "") +
                  Connecting.Piece.Id;
        return $"Con({ctd}--{cng})";
    }

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (Connected.Piece.Id == Connecting.Piece.Id)
        {
            isValid = false;
            errors.Add("The connected and connecting pieces must be different.");
        }

        return (isValid, errors);
    }
}

public class DesignProps : Model<Design>
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
    ///     🖼️ The optional icon [ emoji | name | url ] of the design.
    /// </summary>
    [Url("🖼️", "Ic?", "Ico", "The optional icon [ emoji | name | url ] of the design.")]
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
}

/// <summary>
///     🏙️ A design is a collection of pieces that are connected.
/// </summary>
[Model("🏙️", "Dn", "Dsn", "A design is a collection of pieces that are connected.")]
public class Design : DesignProps
{
    /// <summary>
    ///     ⭕ The optional pieces of the design.
    /// </summary>
    [ModelProp("⭕", "Pc?", "Pcs", "The optional pieces of the design.", PropImportance.OPTIONAL)]
    public List<Piece> Pieces { get; set; } = new();

    /// <summary>
    ///     🔗 The optional connections of the design.
    /// </summary>
    [ModelProp("🔗", "Co?", "Cons", "The optional connections of the design.", PropImportance.OPTIONAL)]
    public List<Connection> Connections { get; set; } = new();

    /// <summary>
    ///     📏 The optional qualities of the design.
    /// </summary>
    [ModelProp("📏", "Ql*", "Qualities", "The optional qualities of the design.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();

    public Design Flatten(Type[] types,
        Func<Plane, Point, Vector, Point, Vector, float, float, float, Plane> computeChildPlane)
    {
        var clone = DeepClone();
        if (clone.Pieces.Count > 1 && clone.Connections.Count > 0)
        {
            var pieces = clone.Pieces.ToDictionary(p => p.Id);
            var ports = new Dictionary<string, Dictionary<string, Dictionary<string, Port>>>();
            foreach (var type in types)
            {
                if (!ports.ContainsKey(type.Name))
                    ports[type.Name] = new Dictionary<string, Dictionary<string, Port>>();
                if (!ports[type.Name].ContainsKey(type.Variant))
                    ports[type.Name][type.Variant] = new Dictionary<string, Port>();
                foreach (var port in type.Ports)
                    ports[type.Name][type.Variant][port.Id] = port;
            }

            var graph = new UndirectedGraph<string, Edge<string>>();
            foreach (var piece in clone.Pieces)
                graph.AddVertex(piece.Id);
            foreach (var connection in clone.Connections)
                graph.AddEdge(new Edge<string>(connection.Connected.Piece.Id, connection.Connecting.Piece.Id));
            var components = new Dictionary<string, int>();
            graph.ConnectedComponents(components);
            var componentPieces = new Dictionary<int, Dictionary<string, Piece>>();
            foreach (var kvp in components)
            {
                if (!componentPieces.ContainsKey(kvp.Value))
                    componentPieces[kvp.Value] = new Dictionary<string, Piece>();
                componentPieces[kvp.Value][kvp.Key] = pieces[kvp.Key];
            }

            foreach (var component in componentPieces)
            {
                var subGraph = new UndirectedGraph<string, Edge<string>>();
                foreach (var piece in component.Value)
                    subGraph.AddVertex(piece.Key);
                foreach (var connection in clone.Connections)
                    if (component.Value.ContainsKey(connection.Connected.Piece.Id) &&
                        component.Value.ContainsKey(connection.Connecting.Piece.Id))
                        subGraph.AddEdge(new Edge<string>(connection.Connected.Piece.Id, connection.Connecting.Piece.Id));
                var root = subGraph.Vertices.FirstOrDefault(p => pieces[p].Plane != null);
                if (root == null)
                {
                    root = subGraph.Vertices.First();
                    pieces[root].Plane = new Plane();
                }
                var bfs = new UndirectedBreadthFirstSearchAlgorithm<string, Edge<string>>(subGraph);
                bfs.SetRootVertex(root);
                bfs.TreeEdge += (g, edge) =>
                {
                    var parent = pieces[edge.Source];
                    var child = pieces[edge.Target];
                    var connection = clone.Connections.First(c =>
                        (c.Connected.Piece.Id == parent.Id && c.Connecting.Piece.Id == child.Id) ||
                        (c.Connected.Piece.Id == child.Id && c.Connecting.Piece.Id == parent.Id));
                    var isParentConnected = connection.Connected.Piece.Id == parent.Id;
                    var parentPlane = parent.Plane;
                    var parentPort =
                        ports[parent.Type.Name][parent.Type.Variant][
                            isParentConnected ? connection.Connected.Port.Id : connection.Connecting.Port.Id];
                    var childPort =
                        ports[child.Type.Name][child.Type.Variant][
                            isParentConnected ? connection.Connecting.Port.Id : connection.Connected.Port.Id];
                    var childPlane = computeChildPlane(parentPlane, parentPort.Point, parentPort.Direction, childPort.Point,
                        childPort.Direction, connection.Rotation, connection.Tilt, connection.Offset);
                    child.Plane = childPlane;
                };
                bfs.Compute();
            }
        }

        clone.Connections = new List<Connection>();

        return clone;
    }

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var piece in Pieces)
        {
            var (isValidPiece, errorsPiece) = piece.Validate();
            isValid = isValid && isValidPiece;
            errors.AddRange(errorsPiece.Select(e => "A piece is invalid: " + e));
        }

        var connectionValidator = new ModelValidator<Connection>();
        foreach (var connection in Connections)
        {
            var (isValidConnection, errorsConnection) = connection.Validate();
            isValid = isValid && isValidConnection;
            errors.AddRange(errorsConnection.Select(e => "A connection is invalid: " + e));
        }

        var qualityValidator = new ModelValidator<Quality>();
        foreach (var quality in Qualities)
        {
            var (isValidQuality, errorsQuality) = quality.Validate();
            isValid = isValid && isValidQuality;
            errors.AddRange(errorsQuality.Select(e => "A quality is invalid: " + e));
        }

        var pieceIds = Pieces.Select(p => p.Id);

        var duplicatePieceIds = pieceIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicatePieceIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicatePieceId in duplicatePieceIds)
                errors.Add($"A piece is invalid: There are multiple pieces with id ({duplicatePieceId}).");
        }

        var nonExistingConnectedPieces = Connections.Where(c => !pieceIds.Contains(c.Connected.Piece.Id)).ToList().Select(c=>c.Connected.Piece.Id).ToArray();
        if (nonExistingConnectedPieces.Length!=0)
        {
            isValid = false;
            foreach (var nonExistingConnectedPiece in nonExistingConnectedPieces)
                errors.Add($"A connection is invalid: The referenced connected piece ({nonExistingConnectedPiece}) is not part of the design.");
        }
        var nonExistingConnectingPieces = Connections.Where(c => !pieceIds.Contains(c.Connecting.Piece.Id)).ToList().Select(c => c.Connecting.Piece.Id).ToArray();
        if (nonExistingConnectingPieces.Length != 0)
        {
            isValid = false;
            foreach (var nonExistingConnectingPiece in nonExistingConnectingPieces)
                errors.Add($"A connection is invalid: The referenced connecting piece ({nonExistingConnectingPiece}) is not part of the design.");
        }
        
    return (isValid, errors);
    }
}

/// <summary>
///     🏙️ The local identifier of the design within the kit.
/// </summary>
[Model("🏙️", "Dn", "Dsn", "The local identifier of the design within the kit.")]
public class DesignId : Model<DesignId>
{
    /// <summary>
    ///     📛 Name of the design.
    /// </summary>
    [Name("📛", "Na", "Nam", "The name of the design.", PropImportance.ID)]
    public string Name { get; set; } = "";

    /// <summary>
    ///     🔀 The optional value of the design.
    /// </summary>
    [Name("🔀", "Vn?", "Vnt", "The optional value of the design.", PropImportance.ID, true)]
    public string Variant { get; set; } = "";

    public static implicit operator DesignId(DesignProps design)
    {
        return new DesignId
        {
            Name = design.Name,
            Variant = design.Variant
        };
    }

    public static implicit operator DesignId(Design design)
    {
        return new DesignId
        {
            Name = design.Name,
            Variant = design.Variant
        };
    }
}

public class KitProps : Model<Kit>
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
    ///     🖼️ The optional icon [ emoji | name | url ] of the design.
    /// </summary>
    [Url("🖼️", "Ic?", "Ico", "The optional icon [ emoji | name | url ] of the design.")]
    public string Icon { get; set; } = "";

    /// <summary>
    ///     ☁️ The optional Unique Resource Locator (URL) where to fetch the kit remotely.
    /// </summary>
    [Url("☁️", "Rm?", "Rmt", "The optional Unique Resource Locator (URL) where to fetch the kit remotely.")]
    public string Remote { get; set; } = "";

    /// <summary>
    ///     🏠 The optional Unique Resource Locator (URL) of the homepage of the kit.
    /// </summary>
    [Url("🏠", "Hp?", "Hmp", "The optional Unique Resource Locator (URL) of the homepage of the kit.")]
    public string Homepage { get; set; } = "";
}

/// <summary>
///     🗃️ A kit is a collection of types and designs.
/// </summary>
[Model("🗃️", "Kt", "Kit", "A kit is a collection of types and designs.")]
public class Kit : KitProps
{
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

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var type in Types)
        {
            var (isValidType, errorsType) = type.Validate();
            isValid = isValid && isValidType;
            errors.AddRange(errorsType.Select(e => "A type is invalid: " + e));
        }
        foreach (var design in Designs)
        {
            var (isValidDesign, errorsDesign) = design.Validate();
            isValid = isValid && isValidDesign;
            errors.AddRange(errorsDesign.Select(e => "A design is invalid: " + e));
        }
        return (isValid, errors);
    }
}

#endregion

#region Api

public class ApiException : Exception
{
    public ApiException(string message) : base(message)
    {
    }
}

public class ServerException : ApiException
{
    public ServerException(string message) : base(message)
    {
    }
}

public class ClientException : ApiException
{
    public ClientException(string message) : base(message)
    {
    }
}

public interface IApi
{
    [Get("/kits/{encodedKitUri}")]
    Task<ApiResponse<Kit>> GetKit(string encodedKitUri);

    [Put("/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> CreateKit(string encodedKitUri, [Body] Kit input);

    [Delete("/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> DeleteKit(string encodedKitUri);


    [Put("/kits/{encodedKitUri}/types/{encodedTypeNameAndVariant}")]
    Task<ApiResponse<bool>> PutType(string encodedKitUri, string encodedTypeNameAndVariant, [Body] Type input);

    [Delete("/kits/{encodedKitUri}/types/{encodedTypeNameAndVariant}")]
    Task<ApiResponse<bool>> RemoveType(string encodedKitUri, string encodedTypeNameAndVariant);

    [Put("/kits/{encodedKitUri}/designs/{encodedDesignNameAndVariant}")]
    Task<ApiResponse<bool>> PutDesign(string encodedKitUri, string encodedDesignNameAndVariant,
        [Body] Design input);

    [Delete("/kits/{encodedKitUri}/designs/{encodedDesignNameAndVariant}")]
    Task<ApiResponse<bool>> RemoveDesign(string encodedKitUri, string encodedDesignNameAndVariant);
}

public static class Api
{
    private static IApi GetApi()
    {
        return RestService.For<IApi>(Constants.EngineAddress, new RefitSettings
        {
            ContentSerializer = new NewtonsoftJsonContentSerializer(
                new JsonSerializerSettings
                {
                    ContractResolver = new CamelCasePropertyNamesContractResolver()
                }
            )
        });
    }

    private static string UnsuccessfullResponseToString<T>(ApiResponse<T> response)
    {
        return JsonConvert.SerializeObject(new
        {
            StatusCode = response.StatusCode.ToString(),
            Message = response.Error.Content ?? "null",
            Request = response.RequestMessage.ToString(),
            Headers = response.Headers.ToString()
        });
    }

    private static void HandleErrors<T>(ApiResponse<T> response)
    {
        if (response.StatusCode == HttpStatusCode.BadRequest)
            throw new ClientException(response.Error.Content);
        if (!response.IsSuccessStatusCode)
            throw new ServerException(UnsuccessfullResponseToString(response));
    }

    public static string EncodeNameAndVariant(string name, string variant = "")
    {
        return Utility.Encode(name) + "," + Utility.Encode(variant);
    }

    public static Kit GetKit(string uri)
    {
        var response = GetApi().GetKit(Utility.Encode(uri)).Result;
        if (response.IsSuccessStatusCode)
            return response.Content;
        HandleErrors(response);
        return null; // This line will never be reached, but is required to satisfy the compiler.
    }

    public static void CreateKit(string uri, Kit input)
    {
        var response = GetApi().CreateKit(Utility.Encode(uri), input).Result;
        HandleErrors(response);
    }

    public static void DeleteKit(string uri)
    {
        var response = GetApi().DeleteKit(Utility.Encode(uri)).Result;
        HandleErrors(response);
    }

    public static void PutType(string kitUrl, Type input)
    {
        var response = GetApi()
            .PutType(Utility.Encode(kitUrl), EncodeNameAndVariant(input.Name, input.Variant), input).Result;
        HandleErrors(response);
    }

    public static void RemoveType(string kitUrl, TypeId id)
    {
        var response = GetApi()
            .RemoveType(Utility.Encode(kitUrl), EncodeNameAndVariant(id.Name, id.Variant)).Result;
        HandleErrors(response);
    }

    public static void PutDesign(string kitUrl, Design input)
    {
        var response = GetApi()
            .PutDesign(Utility.Encode(kitUrl), EncodeNameAndVariant(input.Name, input.Variant), input).Result;
        HandleErrors(response);
    }

    public static void RemoveDesign(string kitUrl, DesignId id)
    {
        var response = GetApi()
            .RemoveDesign(Utility.Encode(kitUrl), EncodeNameAndVariant(id.Name, id.Variant)).Result;
        HandleErrors(response);
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

            // TODO: Add index to prop and add to list based on index not on source code order.
            // GetProperties() returns parent last
            var propertyParents = new List<PropertyInfo>();
            var propParents = new List<PropAttribute>();
            var isPropertyListParents = new List<bool>();
            var propertyItemTypeParents = new List<System.Type>();
            var isPropertyModelParents = new List<bool>();
            foreach (var mtp in mt.GetProperties()
                         .Where(mtp => mtp.GetCustomAttribute<PropAttribute>() != null))
            {
                var mtpProp = mtp.GetCustomAttribute<PropAttribute>();
                var imtpl = mtp.PropertyType.IsGenericType &&
                            mtp.PropertyType.GetGenericTypeDefinition() == typeof(List<>);
                var mtpPropertyItemType = imtpl ? mtp.PropertyType.GetGenericArguments()[0] : mtp.PropertyType;
                var mtpIsPropertyModel = mtp.GetCustomAttribute<ModelPropAttribute>() != null;

                if (mtp.DeclaringType.FullName != mt.FullName)
                {
                    propertyParents.Add(mtp);
                    propParents.Add(mtpProp);
                    isPropertyListParents.Add(imtpl);
                    propertyItemTypeParents.Add(mtpPropertyItemType);
                    isPropertyModelParents.Add(mtpIsPropertyModel);
                }
                else
                {
                    property[mt.Name].Add(mtp);
                    prop[mt.Name].Add(mtpProp);
                    isPropertyList[mt.Name].Add(imtpl);
                    propertyItemType[mt.Name].Add(mtpPropertyItemType);
                    isPropertyModel[mt.Name].Add(mtpIsPropertyModel);
                }
            }

            property[mt.Name].InsertRange(0, propertyParents);
            prop[mt.Name].InsertRange(0, propParents);
            isPropertyList[mt.Name].InsertRange(0, isPropertyListParents);
            propertyItemType[mt.Name].InsertRange(0, propertyItemTypeParents);
            isPropertyModel[mt.Name].InsertRange(0, isPropertyModelParents);
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
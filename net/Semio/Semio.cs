#region Header

//Semio.cs
//2020-2025 Ueli Saluz

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU Lesser General Public License as
//published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU Lesser General Public License for more details.

//You should have received a copy of the GNU Lesser General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion

#region TODOs

// TODO: Make remote uris work for diagram.
// TODO: Remove computeChildPlane and separate the flatten diagram and flatten planes parts.
// TODO: Refactor all ToSring() to use ToIdString() and add ABREVIATION(ID) to model.
// TODO: Develop a validation template for urls.
// TODO: Replace GetHashcode() with a proper hash function.
// TODO: Add logging mechanism to all API calls if they fail.
// TODO: Implement reflexive validation for model properties.
// TODO: Add index to prop and add to list based on index not on source code order.
// TODO: See if Utility.Encode(uri) can be added by attribute on parameters.
// TODO: Turn inplace and leave clone to the user of the function.
// TODO: Parametrize colors for diagram

#endregion

using System.Collections;
using System.Collections.Immutable;
using System.Drawing;
using System.Globalization;
using System.Net;
using System.Net.Http;
using System.Reflection;
using System.Text;
using System.Xml;
using FluentValidation;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using QuikGraph;
using QuikGraph.Algorithms;
using QuikGraph.Algorithms.Search;
using Refit;
using Svg;
using Svg.Transforms;
using UnitsNet;
using Formatting = Newtonsoft.Json.Formatting;

namespace Semio;

#region Constants

public static class Constants
{
    public const string Name = "semio";
    public const string Email = "ueli@semio-tech.com";
    public const string Release = "r25.07-1";
    public const string EngineHost = "http://127.0.0.1";
    public const int EnginePort = 2507;
    public const string EngineAddress = "http://127.0.0.1:2507";
    public const int NameLengthLimit = 64;
    public const int IdLengthLimit = 128;
    public const int UrlLengthLimit = 1024;
    public const int UriLengthLimit = 2048;
    public const int QualitiesMax = 64;
    public const int TagsMax = 8;
    public const int RepresentationsMax = 32;
    public const int TypesMax = 256;
    public const int PiecesMax = 512;
    public const int DesignsMax = 128;
    public const int KitsMax = 64;
    public const int DescriptionLengthLimit = 512;
    public const float Tolerance = 1e-5f;
}

public enum ImageExtensions
{
    png,
    jpg,
    jpeg,
    svg
}

public enum IconKind
{
    Logogram,
    Filepath,
    RemoteUrl
}

public enum EncodeMode
{
    Urlsafe,
    Base64,
    DictionaryOnly
}

#endregion

#region Utility

public static class Utility
{
    public static bool UriIsNotAbsoluteFilePath(string uri)
    {
        return !(Uri.IsWellFormedUriString(uri, UriKind.Relative) || uri.StartsWith("http"));
    }
    public static bool IsValidMime(string mime)
    {
        var validMimes = new List<string>
        {
            "model/stl",
            "model/obj",
            "model/gltf-binary",
            "model/gltf+json",
            "model/vnd.3dm",
            "image/png",
            "image/jpeg",
            "image/svg+xml",
            "application/pdf",
            "application/zip",
            "application/json",
            "text/csv",
            "text/plain"
        };
        return validMimes.Contains(mime);
    }

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

    public static IconKind ParseIconKind(string icon)
    {
        if (icon.StartsWith("http")) return IconKind.RemoteUrl;
        try
        {
            var uri = new Uri(icon, UriKind.Relative);
            var ext = Path.GetExtension(icon);
            if (Enum.IsDefined(typeof(ImageExtensions), ext.ToLower().Substring(1)))
                return IconKind.Filepath;
        }
        catch (Exception) { }
        return IconKind.Logogram;
    }

    public static string DatastringFromUrl(string url)
    {
        string mime;
        byte[] content;
        if (url.StartsWith("http"))
        {
            using (var client = new HttpClient())
            {
                var response = client.GetAsync(url).Result;
                response.EnsureSuccessStatusCode();
                mime = response.Content.Headers.ContentType.MediaType;
                content = response.Content.ReadAsByteArrayAsync().Result;
            }
        }
        else
        {
            var osAwareUrl = url.Replace("/", Path.DirectorySeparatorChar.ToString());
            content = File.ReadAllBytes(osAwareUrl);
            mime = ParseMimeFromUrl(osAwareUrl);
        }
        return $"data:{mime};base64,{Convert.ToBase64String(content)}";
    }

    public static string ReadAndEncode(string filename) => $"data:{ParseMimeFromUrl(filename)};base64,{Convert.ToBase64String(File.ReadAllBytes(filename))}";
    public static string Encode(string text, EncodeMode mode = EncodeMode.Urlsafe,
        Tuple<List<string>, List<string>>? replace = null)
    {
        var encoded = text;
        if (mode == EncodeMode.Urlsafe) encoded = Uri.EscapeDataString(text);
        if (mode == EncodeMode.Base64) encoded = Convert.ToBase64String(Encoding.UTF8.GetBytes(text));
        if (replace != null)
        {
            var keys = replace.Item1;
            var values = replace.Item2;
            if (keys.Count != values.Count) throw new ArgumentException("Both replace lists must have the same length.");
            for (var i = 0; i < keys.Count; i++)
            {
                var key = keys[i];
                var value = values[i];
                encoded = encoded.Replace(key, value);
            }
        }
        return encoded;
    }

    public static string Decode(string text, EncodeMode mode = EncodeMode.Urlsafe,
        Tuple<List<string>, List<string>>? replace = null)
    {
        var decoded = text;
        if (replace != null)
        {
            var keys = replace.Item1;
            var values = replace.Item2;
            if (keys.Count != values.Count) throw new ArgumentException("Both replace lists must have the same length.");
            for (var i = 0; i < keys.Count; i++)
            {
                var key = keys[i];
                var value = values[i];
                decoded = decoded.Replace(key, value);
            }
        }
        if (mode == EncodeMode.Urlsafe) decoded = Uri.UnescapeDataString(decoded);
        if (mode == EncodeMode.Base64) decoded = Encoding.UTF8.GetString(Convert.FromBase64String(decoded));
        return decoded;
    }

    public static string Serialize(this object obj, string indent = "")
    {
        var isTabbed = indent.StartsWith("\t");
        var formatting = string.IsNullOrEmpty(indent) ? Formatting.None : Formatting.Indented;
        var settings = new JsonSerializerSettings { ContractResolver = new CamelCasePropertyNamesContractResolver(), Formatting = formatting };
        if (formatting == Formatting.None) return JsonConvert.SerializeObject(obj, settings);
        var stringWriter = new StringWriter();
        using (var jsonWriter = new JsonTextWriter(stringWriter))
        {
            jsonWriter.Formatting = Formatting.Indented;
            jsonWriter.IndentChar = isTabbed ? '\t' : ' ';
            jsonWriter.Indentation = indent.Length;
            JsonSerializer.Create(settings).Serialize(jsonWriter, obj);
        }
        return stringWriter.ToString();
    }


    public static T Deserialize<T>(this string json) => JsonConvert.DeserializeObject<T>(json, new JsonSerializerSettings { ContractResolver = new CamelCasePropertyNamesContractResolver() });

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

    public static class Units
    {
        public static float Convert(float value, string fromUnit, string toUnit)
        {
            var convertModel = new PowerToysRunUnitConverter.ConvertModel(value, fromUnit, toUnit);
            var results = PowerToysRunUnitConverter.UnitHandler.Convert(convertModel);
            if (results.Count() == 0) return float.NaN;
            return (float)results.First().ConvertedValue;
        }

        /// <summary>
        ///     Adapted from
        ///     https://github.com/microsoft/PowerToys/tree/95919508758e71dca88632add8a03c089a822d1c/src/modules/launcher/Plugins/Community.PowerToys.Run.Plugin.UnitConverter
        /// </summary>
        private class PowerToysRunUnitConverter
        {
            internal class ConvertModel
            {
                internal ConvertModel() { }
                internal ConvertModel(double value, string fromUnit, string toUnit) => (Value, FromUnit, ToUnit) = (value, fromUnit, toUnit);
                internal double Value { get; }
                internal string FromUnit { get; }
                internal string ToUnit { get; }
            }

            internal class UnitConversionResult
            {
                internal UnitConversionResult(double convertedValue, string unitName, QuantityInfo quantityInfo) => (ConvertedValue, UnitName, QuantityInfo) = (convertedValue, unitName, quantityInfo);
                internal static string TitleFormat { get; set; } = "G14";
                internal static string CopyFormat { get; set; } = "R";
                internal double ConvertedValue { get; }
                internal string UnitName { get; }
                internal QuantityInfo QuantityInfo { get; }
            }

            internal static class UnitHandler
            {
                private static readonly QuantityInfo[] _included =
                {
                    Length.Info,
                    Area.Info,
                    Volume.Info,
                    Duration.Info,
                    Energy.Info,
                    Power.Info,
                    Pressure.Info,
                    Mass.Info,
                    Angle.Info,
                    Temperature.Info,
                    Acceleration.Info,
                    Speed.Info,
                    Information.Info
                };
                private static Enum GetUnitEnum(string unit, QuantityInfo unitInfo)
                {
                    var first = Array.Find(unitInfo.UnitInfos, info => string.Equals(unit, info.Name, StringComparison.OrdinalIgnoreCase) || string.Equals(unit, info.PluralName, StringComparison.OrdinalIgnoreCase));
                    if (first != null) return first.Value;
                    if (UnitsNetSetup.Default.UnitParser.TryParse(unit, unitInfo.UnitType, out var enum_unit)) return enum_unit;
                    var cultureInfoEnglish = new CultureInfo("en-US");
                    if (UnitsNetSetup.Default.UnitParser.TryParse(unit, unitInfo.UnitType, cultureInfoEnglish, out var enum_unit_en)) return enum_unit_en;
                    return null;
                }
                internal static double ConvertInput(ConvertModel convertModel, QuantityInfo quantityInfo)
                {
                    var fromUnit = GetUnitEnum(convertModel.FromUnit, quantityInfo);
                    var toUnit = GetUnitEnum(convertModel.ToUnit, quantityInfo);
                    if (fromUnit != null && toUnit != null) return UnitConverter.Convert(convertModel.Value, fromUnit, toUnit);
                    return double.NaN;
                }
                internal static IEnumerable<UnitConversionResult> Convert(ConvertModel convertModel)
                {
                    var results = new List<UnitConversionResult>();
                    foreach (var quantityInfo in _included)
                    {
                        var convertedValue = ConvertInput(convertModel, quantityInfo);
                        if (!double.IsNaN(convertedValue)) results.Add(new UnitConversionResult(convertedValue, convertModel.ToUnit, quantityInfo));
                    }
                    return results;
                }
            }
        }
    }

    public static class Grammar
    {
        public static string GetArticle(string word) => string.IsNullOrEmpty(word) ? string.Empty : "aeiou".IndexOf(word.ToLower()[0]) >= 0 ? "an" : "a";
    }
}

#endregion

#region Modeling

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
    { }
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
    public PropAttribute(string emoji, string code, string abbreviation, string description, PropImportance importance, bool isDefaultValid, bool skipValidation) : base(emoji, code, abbreviation, description)
        => (Importance, IsDefaultValid, SkipValidation) = (importance, isDefaultValid, skipValidation);
    public PropImportance Importance { get; set; }
    public bool IsDefaultValid { get; set; }
    public bool SkipValidation { get; set; }
}

public abstract class TextAttribute : PropAttribute
{
    public TextAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance, bool isDefaultValid, bool skipValidation, int lengthLimit) : base(emoji, code,
        abbreviation, description, importance, isDefaultValid, skipValidation)
        => LengthLimit = lengthLimit;
    public int LengthLimit { get; set; }
}

public class NameAttribute : TextAttribute
{
    public NameAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = false, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, Constants.NameLengthLimit)
    { }
}

public class IdAttribute : TextAttribute
{
    public IdAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.ID, bool isDefaultValid = false, bool skipValidation = false) : base(
        emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, Constants.IdLengthLimit)
    { }
}

public class EmailAttribute : TextAttribute
{
    public EmailAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = false, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, Constants.IdLengthLimit)
    { }
}

public class UrlAttribute : TextAttribute
{
    public UrlAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = false, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, Constants.UrlLengthLimit)
    { }
}

public class DescriptionAttribute : TextAttribute
{
    public DescriptionAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, Constants.DescriptionLengthLimit)
    { }
}

public class FalseOrTrueAttribute : PropAttribute
{
    public FalseOrTrueAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation)
    { }
}

public class IntPropAttribute : PropAttribute
{
    public IntPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation)
    { }
}

public class NumberPropAttribute : PropAttribute
{
    public NumberPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation)
    { }
}

public class AnglePropAttribute : NumberPropAttribute
{
    public AnglePropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation)
    { }
}

public class ModelPropAttribute : PropAttribute
{
    public ModelPropAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.REQUIRED, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation)
    { }
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
        if (nonEmptyIdPropertiesValues.Count != 0)
            return $"{modelAttribute.Abbreviation}({string.Join(",", nonEmptyIdPropertiesValues)})";
        var requiredProperties = GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(p => p.GetCustomAttribute<PropAttribute>()?.Importance == PropImportance.REQUIRED)
            .Select(p => p.Name);
        var requiredPropertiesValues = requiredProperties.Select(p => GetType().GetProperty(p)?.GetValue(this))
            .Select(v => v.ToString()).ToList();
        return $"{modelAttribute.Abbreviation}({string.Join(",", requiredPropertiesValues)})";
    }

    public override bool Equals(object obj)
    {
        if (obj == null || GetType() != obj.GetType()) return false;
        return GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance).All(prop => PropertiesAreEqual(prop, this, obj));
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
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null) return false;
        return left.Equals(right);
    }

    public static bool operator !=(Model<T> left, Model<T> right) => !(left == right);

    public T DeepClone() => this.Serialize().Deserialize<T>();

    public virtual (bool, List<string>) Validate()
    {
        var result = new ModelValidator<T>().Validate((T)this);
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
        if (propAttribute.SkipValidation) return;
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
            // All non-description text
            if (property.GetCustomAttribute<DescriptionAttribute>() == null)
                RuleFor(model => property.GetValue(model) as string)
                    .Matches(@"^\S.*$")
                    .When(m => property.GetValue(m) != "")
                    .WithMessage($"The {property.Name.ToLower()} must not start with a space.")
                    .Matches(@"^.*\S$")
                    .When(m => property.GetValue(m) != "")
                    .WithMessage($"The {property.Name.ToLower()} must not end with a space.")
                    .Matches(@"^[^\r\n]*$")
                    .When(m => property.GetValue(m) != "")
                    .WithMessage($"The {property.Name.ToLower()} must not contain newlines.");

            if (property.GetCustomAttribute<NameAttribute>() != null)
            { }
            else if (property.GetCustomAttribute<IdAttribute>() != null)
            { }
            else if (property.GetCustomAttribute<EmailAttribute>() != null)
            {
                RuleFor(model => property.GetValue(model) as string)
                    .EmailAddress().WithMessage($"The {property.Name.ToLower()} is not a valid email address.");
            }
            else if (property.GetCustomAttribute<UrlAttribute>() != null)
            { }
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

#region Models

/// <summary>
/// <see href="https://github.com/usalu/semio#-quality-"/>
/// </summary>
[Model("📏", "Ql", "Qal", "A quality is a named value with a unit and a definition.")]
public class Quality : Model<Quality>
{
    [Name("📏", "Na", "Nam", "The name of the quality.", PropImportance.ID)]
    public string Name { get; set; } = "";

    [Description("🔢", "Vl?", "Val?",
        "The optional value [ text | url ] of the quality. No value is equivalent to true for the name.")]
    public string Value { get; set; } = "";

    [Name("Ⓜ️", "Ut?", "Unt?", "The optional unit of the value of the quality.", isDefaultValid: true)]
    public string Unit { get; set; } = "";

    [Description("📖", "Df?", "Def?", "The optional definition [ text | uri ] of the quality.")]
    public string Definition { get; set; } = "";

    public string ToIdString() => $"{Name}";

    public string ToHumanIdString() => $"{ToIdString()}";

    public override string ToString() => $"Qal({ToHumanIdString()})";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-representation-"/>
/// </summary>
[Model("💾", "Rp", "Rep",
    "A representation is a link to a resource that describes a type for a certain level of detail and tags.")]
public class Representation : Model<Representation>
{
    [Url("🔗", "Ur", "Url", "The Unique Resource Locator (URL) to the resource of the representation.", PropImportance.REQUIRED)]
    public string Url { get; set; } = "";

    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the representation.")]
    public string Description { get; set; } = "";

    [Name("🏷️", "Tg*", "Tags*", "The optional tags to group representations. No tags means default.", PropImportance.ID, skipValidation: true)]
    public List<string> Tags { get; set; } = new();

    [ModelProp("📏", "Ql*", "Qals", "The optional qualities of the representation.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();

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

            foreach (var quality in Qualities)
            {
                var (isValidQuality, errorsQuality) = quality.Validate();
                isValid = isValid && isValidQuality;
                errors.AddRange(errorsQuality.Select(e => $"A quality({quality.ToHumanIdString()}) is invalid: " + e));
            }
        }

        return (isValid, errors);
    }

    public string ToIdString() => $"{string.Join(",", Tags.Select(t => Utility.Encode(t)))}";

    public string ToHumanIdString() => string.Join(", ", Tags);

    public override string ToString() => $"Rep({ToHumanIdString()})";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-diagram-point-"/>
/// </summary>
[Model("📺", "DP", "DPt", "A 2d-point (xy) of floats in the diagram. One unit is equal the width of a piece icon.")]
public class DiagramPoint : Model<DiagramPoint>
{
    [NumberProp("🎚️", "X", "X", "The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.", PropImportance.REQUIRED)]
    public float X { get; set; }

    [NumberProp("🎚️", "Y", "Y", "The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.", PropImportance.REQUIRED)]
    public float Y { get; set; }

    public DiagramPoint Normalize()
    {
        var length = (float)Math.Sqrt(X * X + Y * Y);
        return new DiagramPoint { X = X / length, Y = Y / length };
    }
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-point-"/>
/// </summary>
[Model("✖️", "Pt", "Pnt", "A 3-point (xyz) of floating point numbers.")]
public class Point : Model<Point>
{
    [NumberProp("🎚️", "X", "X", "The x-coordinate of the point.", PropImportance.REQUIRED)]
    public float X { get; set; } = 0;
    [NumberProp("🎚️", "Y", "Y", "The y-coordinate of the point.", PropImportance.REQUIRED)]
    public float Y { get; set; } = 0;
    [NumberProp("🎚️", "Z", "Z", "The z-coordinate of the point.", PropImportance.REQUIRED)]
    public float Z { get; set; } = 0;
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-vector-"/>
/// </summary>
[Model("➡️", "Vc", "Vec", "A 3d-vector (xyz) of floating point numbers.")]
public class Vector : Model<Vector>
{
    [NumberProp("🎚️", "X", "X", "The x-coordinate of the vector.", PropImportance.REQUIRED)]
    public float X { get; set; } = 1;
    [NumberProp("🎚️", "Y", "Y", "The y-coordinate of the vector.", PropImportance.REQUIRED)]
    public float Y { get; set; }

    [NumberProp("🎚️", "Z", "Z", "The z-coordinate of the vector.", PropImportance.REQUIRED)]
    public float Z { get; set; } = 0;

    public static float DotProduct(Vector a, Vector b) => a.X * b.X + a.Y * b.Y + a.Z * b.Z;

    public static bool IsOrthogonal(Vector a, Vector b) => Math.Abs(DotProduct(a, b)) < Constants.Tolerance;

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
/// <see href="https://github.com/usalu/semio#-plane-"/>
/// </summary>
[Model("◳", "Pn", "Pln", "A plane is an origin (point) and an orientation (x-axis and y-axis).")]
public class Plane : Model<Plane>
{
    [ModelProp("⌱", "Og", "Org", "The origin of the plane.")]
    public Point Origin { get; set; } = new();

    [ModelProp("➡️", "XA", "XAx", "The x-axis of the plane.")]
    public Vector XAxis { get; set; } = new();

    [ModelProp("➡️", "YA", "YAx", "The y-axis of the plane.")]
    public Vector YAxis { get; set; } = new() { Y = 1 };

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        var (isValidOrigin, errorsOrigin) = Origin.Validate();
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
/// <see href="https://github.com/usalu/semio#-port-"/>
/// </summary>
[Model("🔌", "Po", "Por", "A port is a connection point (with a direction) of a type.")]
public class Port : Model<Port>
{
    [Id("🆔", "Id?", "Idn?", "The optional local identifier of the port within the type. No id means the default port.", isDefaultValid: true)]
    [JsonProperty("id_")]
    public string Id { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the port.")]
    public string Description { get; set; } = "";
    [FalseOrTrue("💯", "Ma?", "Man?", "Whether the port is mandatory. A mandatory port must be connected in a design.")]
    public bool Mandatory { get; set; } = false;
    [Name("👨‍👩‍👧‍👦", "Fa?", "Fam?", "The optional family of the port. This allows to define explicit compatibility with other ports.")]
    public string Family { get; set; } = "";
    [Name("✅", "CF*", "CFas*", "The optional other compatible families of the port. An empty list means this port is compatible with all other ports.")]
    public List<string> CompatibleFamilies { get; set; } = new();
    [ModelProp("✖️", "Pt", "Pnt", "The connection point of the port that is attracted to another connection point.")]
    public Point? Point { get; set; } = null;
    [ModelProp("➡️", "Dr", "Drn", "The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned.")]
    public Vector? Direction { get; set; } = null;
    [NumberProp("💍", "T", "T", "The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise.", PropImportance.REQUIRED)]
    public float T { get; set; } = 0;
    [ModelProp("📏", "Ql*", "Qals", "The optional qualities of the port.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Por({ToHumanIdString()})";

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
        foreach (var quality in Qualities)
        {
            var (isValidQuality, errorsQuality) = quality.Validate();
            isValid = isValid && isValidQuality;
            errors.AddRange(errorsQuality.Select(e => $"A quality({quality.ToHumanIdString()}) is invalid: " + e));
        }
        return (isValid, errors);
    }
}

[Model("🔌", "Po", "Por", "The optional local identifier of the port within the type. No id means the default port.")]
public class PortId : Model<PortId>
{
    [Id("🆔", "Id?", "Id?", "The local identifier of the port within the type.", isDefaultValid: true)]
    [JsonProperty("id_")]
    public string Id { get; set; } = "";
    public static implicit operator PortId(Port port) => new() { Id = port.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Por({ToHumanIdString()})";
}

[Model("👤", "Au", "Aut", "The information about the author.")]
public class Author : Model<Author>
{
    [Name("📛", "Na", "Nam", "The name of the author.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Email("📧", "Em", "Eml", "The email of the author.", PropImportance.ID)]
    public string Email { get; set; } = "";
    public string ToIdString() => $"{Email}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Aut({ToHumanIdString()})";

    public override (bool, List<string>) Validate()
    {
        // TODO: proper email validation
        var (isValid, errors) = base.Validate();
        if (!Email.Contains("@"))
        {
            isValid = false;
            errors.Add("The email must contain an @.");
        }

        return (isValid, errors);
    }
}

[Model("📍", "Lc", "Loc", "A location on the earth surface (longitude, latitude).")]
public class Location : Model<Location>
{
    [NumberProp("↔️", "Lo", "Lon", "The longitude of the location in degrees.", PropImportance.REQUIRED)]
    public float Longitude { get; set; }
    [NumberProp("↕️", "La", "Lat", "The latitude of the location in degrees.", PropImportance.REQUIRED)]
    public float Latitude { get; set; }
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-type-"/>
/// </summary>
[Model("🧩", "Ty", "Typ", "A type is a reusable element that can be connected with other types over ports.")]
public class Type : Model<Type>
{
    [Name("📛", "Na", "Nam", "The name of the type.", PropImportance.ID)]
    public string Name { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the type.")]
    public string Description { get; set; } = "";
    [Url("🪙", "Ic?", "Ico?", "The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.")]
    public string Icon { get; set; } = "";
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.")]
    public string Image { get; set; } = "";
    [Name("🔀", "Vn?", "Vnt?", "The optional variant of the type. No variant means the default variant. ", PropImportance.ID, true)]
    public string Variant { get; set; } = "";
    [IntProp("📦", "St?", "Stk?", "The optional number of items in stock. 2147483647 (=2^31-1) means infinite stock.")]
    public int Stock { get; set; } = 2147483647;
    [FalseOrTrue("👻", "Vi?", "Vir?", "Whether the type is virtual. A virtual type is not physically present but is used in conjunction with other virtual types to form a larger physical type.")]
    public bool Virtual { get; set; } = false;
    [ModelProp("📍", "Lo?", "Loc?", "The optional location of the type.", PropImportance.OPTIONAL)]
    public Location? Location { get; set; }
    [Name("Ⓜ️", "Ut", "Unt", "The length unit of the point and the direction of the ports of the type.", PropImportance.REQUIRED)]
    public string Unit { get; set; } = "";
    [ModelProp("💾", "Rp*", "Reps*", "The optional representations of the type.", PropImportance.OPTIONAL)]
    public List<Representation> Representations { get; set; } = new();
    [ModelProp("🔌", "Po*", "Pors*", "The optional ports of the type.", PropImportance.OPTIONAL)]
    public List<Port> Ports { get; set; } = new();
    [ModelProp("👥", "Au*", "Auts*", "The optional authors of the type.", PropImportance.OPTIONAL)]
    public List<Author> Authors { get; set; } = new();
    [ModelProp("📏", "Ql*", "Qals*", "The optional qualities of the type.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();

    public string ToIdString() => $"{Name}#{Variant}";

    public string ToHumanIdString() => $"{Name}" + (Variant.Length == 0 ? "" : $", {Variant}");

    public override string ToString() => $"Typ({ToHumanIdString()})";

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var port in Ports)
        {
            var (isValidPort, errorsPort) = port.Validate();
            isValid = isValid && isValidPort;
            errors.AddRange(errorsPort.Select(e => $"A port({port.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var representation in Representations)
        {
            var (isValidRepresentation, errorsRepresentation) = representation.Validate();
            isValid = isValid && isValidRepresentation;
            errors.AddRange(errorsRepresentation.Select(e =>
                $"A representation({representation.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var author in Authors)
        {
            var (isValidAuthor, errorsAuthor) = author.Validate();
            isValid = isValid && isValidAuthor;
            errors.AddRange(errorsAuthor.Select(e => $"An author({author.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var quality in Qualities)
        {
            var (isValidQuality, errorsQuality) = quality.Validate();
            isValid = isValid && isValidQuality;
            errors.AddRange(errorsQuality.Select(e => $"A quality({quality.ToHumanIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }

    public static Dictionary<string, Dictionary<string, Type>> EnumerableToDict(IEnumerable<Type> types)
    {
        var typesDict = new Dictionary<string, Dictionary<string, Type>>();
        foreach (var type in types)
        {
            if (!typesDict.ContainsKey(type.Name)) typesDict[type.Name] = new Dictionary<string, Type>();
            typesDict[type.Name][type.Variant] = type;
        }

        return typesDict;
    }
}

[Model("🧩", "Ty", "Typ", " identifier of the type within the kit.")]
public class TypeId : Model<TypeId>
{
    [Name("📛", "Na", "Nam", "The name of the type.", PropImportance.ID)]
    public string Name { get; set; } = ""; [Name("🔀", "Vn?", "Vnt?", "The optional variant of the type. No variant means the default variant. ", PropImportance.ID, true)]
    public string Variant { get; set; } = "";
    public string ToIdString() => $"{Name}#{Variant}";
    public string ToHumanIdString() => $"{Name}" + (Variant.Length == 0 ? "" : $", {Variant}");
    public override string ToString() => $"Typ({ToHumanIdString()})";

    public static implicit operator TypeId(Type type) => new() { Name = type.Name, Variant = type.Variant };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-piece-"/>
/// </summary>
[Model("⭕", "Pc", "Pce", "A piece is a 3d-instance of a type in a design.")]
public class Piece : Model<Piece>
{
    [Id("🆔", "Id?", "Id", "The optional local identifier of the piece within the design. No id means the default piece.", isDefaultValid: true)]
    [JsonProperty("id_")]
    public string Id { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the piece.")]
    public string Description { get; set; } = "";
    [ModelProp("🧩", "Ty?", "Typ?", "The optional type of the piece. Either type or design must be set.", PropImportance.OPTIONAL)]
    public TypeId? Type { get; set; }
    [ModelProp("🏙️", "Dn?", "Dsn?", "The optional design of this piece. Either type or design must be set.", PropImportance.OPTIONAL)]
    public DesignId? Design { get; set; }
    [ModelProp("◳", "Pn?", "Pln?", "The optional plane of the piece. When pieces are connected only one piece can have a plane.", PropImportance.OPTIONAL)]
    public Plane? Plane { get; set; }
    [ModelProp("⌖", "Ce?", "Cen?", "The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.", PropImportance.OPTIONAL)]
    public DiagramPoint? Center { get; set; }
    [FalseOrTrue("👻", "Hi?", "Hid?", "Whether the piece is hidden. A hidden piece is not visible in the model.")]
    public bool Hidden { get; set; } = false;
    [FalseOrTrue("🔒", "Lk?", "Lck?", "Whether the piece is locked. A locked piece cannot be edited.")]
    public bool Locked { get; set; } = false;
    [ModelProp("📏", "Ql*", "Qals*", "The optional qualities of the piece.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        var hasType = Type is not null && Type.Name != "";
        var hasDesign = Design is not null && Design.Name != "";
        if (!(hasType ^ hasDesign))
        {
            isValid = false;
            errors.Add("Exactly one of type or design must be set on a piece.");
        }
        if (hasType && Type is not null)
        {
            var (isValidType, errorsType) = Type.Validate();
            isValid = isValid && isValidType;
            errors.AddRange(errorsType.Select(e => $"The type({Type.ToHumanIdString()}) is invalid: " + e));
        }
        if (hasDesign && Design is not null)
        {
            var (isValidDesign, errorsDesign) = Design.Validate();
            isValid = isValid && isValidDesign;
            errors.AddRange(errorsDesign.Select(e => $"The design({Design.ToHumanIdString()}) is invalid: " + e));
        }
        if (Plane is not null)
        {
            var (isValidPlane, errorsPlane) = Plane.Validate();
            isValid = isValid && isValidPlane;
            errors.AddRange(errorsPlane.Select(e => "The plane is invalid: " + e));
        }
        if (Center is not null)
        {
            var (isValidCenter, errorsCenter) = Center.Validate();
            isValid = isValid && isValidCenter;
            errors.AddRange(errorsCenter.Select(e => "The center is invalid: " + e));
        }
        foreach (var quality in Qualities)
        {
            var (isValidQuality, errorsQuality) = quality.Validate();
            isValid = isValid && isValidQuality;
            errors.AddRange(errorsQuality.Select(e => $"A quality({quality.ToHumanIdString()}) is invalid: " + e));
        }
        return (isValid, errors);
    }
}

[Model("⭕", "Pc", "Pce",
    "The optional local identifier of the piece within the design. No id means the default piece.")]
public class PieceId : Model<PieceId>
{
    [Id("🆔", "Id?", "Id?", "The optional local identifier of the piece within the design. No id means the default piece.", isDefaultValid: true)]
    [JsonProperty("id_")]
    public string Id { get; set; } = "";
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Pce({ToHumanIdString()})";
}

[Model("🧱", "Sd", "Sde", "A side of a piece in a connection.")]
public class Side : Model<Side>
{
    [ModelProp("⭕", "Pc", "Pce", "The piece-related information of the side.")]
    public PieceId Piece { get; set; } = new();
    [ModelProp("🏙️", "DP?", "DPc?", "The optional id of the piece inside the referenced design piece.", PropImportance.OPTIONAL)]
    public PieceId? DesignPiece { get; set; } = null;
    [ModelProp("🔌", "Po", "Por", "The local identifier of the port within the type.")]
    public PortId Port { get; set; } = new();
    public override string ToString() => $"Sde({Piece.Id}" + (Port.Id != "" ? ":" + Port.Id : "") + ")";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-connection-"/>
/// </summary>
[Model("🔗", "Co", "Con", "A bidirectional connection between two pieces of a design.")]
public class Connection : Model<Connection>
{
    private float _rotation;
    private float _tilt;
    private float _turn;

    [ModelProp("🧲", "Cd", "Cnd", "The connected side of the piece of the connection.")]
    public Side Connected { get; set; } = new();
    [ModelProp("🧲", "Cg", "Cng", "The connected side of the piece of the connection.")]
    public Side Connecting { get; set; } = new();
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the connection.")]
    public string Description { get; set; } = "";
    [NumberProp("↕️", "Gp?", "Gap?", "The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece.")]
    public float Gap { get; set; } = 0;
    [NumberProp("↔️", "Sf?", "Sft?", "The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between the connected and the connecting piece.")]
    public float Shift { get; set; } = 0;
    [NumberProp("🪜", "Rs", "Ris", "The optional vertical rise in port direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result.")]
    public float Rise { get; set; } = 0;
    [AngleProp("🔄", "Rt?", "Rot?", "The optional horizontal rotation in port direction between the connected and the connecting piece in degrees.")]
    public float Rotation
    {
        get => _rotation;
        set => _rotation = (value % 360 + 360) % 360;
    }
    [AngleProp("🛞", "Tu", "Tur", "The optional turn perpendicular to the port direction(applied after rotation and the turn) between the connected and the connecting piece in degrees.Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result.")]
    public float Turn
    {
        get => _turn;
        set => _turn = (value % 360 + 360) % 360;
    }
    [AngleProp("∡", "Tl?", "Tlt?",
        "The optional horizontal tilt perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.")]
    public float Tilt
    {
        get => _tilt;
        set => _tilt = (value % 360 + 360) % 360;
    }

    [NumberProp("➡️", "X?", "X?", "The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.")]
    public float X { get; set; }
    [NumberProp("⬆️", "Y?", "Y?", "The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.")]
    public float Y { get; set; } = 1;
    [ModelProp("📏", "Ql*", "Qals*", "The optional qualities of the connection.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();

    public string ToIdString() => $"{Connected.Piece.Id + (Connected.Port.Id != "" ? ":" + Connected.Port.Id : "")}--{(Connecting.Port.Id != "" ? Connecting.Port.Id + ":" : "") + Connecting.Piece.Id}";

    public string ToHumanIdString() => $"{ToIdString()}";

    public override string ToString() => $"Con({ToIdString()})";

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (Connected.Piece.Id == Connecting.Piece.Id)
        {
            isValid = false;
            errors.Add("The connected and connecting pieces must be different.");
        }

        if (Math.Abs(X) < Constants.Tolerance && Math.Abs(Y) < Constants.Tolerance)
        {
            isValid = false;
            errors.Add("The offset (x,y) must not be the zero vector.");
        }

        foreach (var quality in Qualities)
        {
            var (isValidQuality, errorsQuality) = quality.Validate();
            isValid = isValid && isValidQuality;
            errors.AddRange(errorsQuality.Select(e => $"A quality({quality.ToHumanIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-design-"/>
/// </summary>
[Model("🏙️", "Dn", "Dsn", "A design is a collection of pieces that are connected.")]
public class Design : Model<Design>
{
    [Name("📛", "Na", "Nam", "The name of the design.", PropImportance.ID)]
    public string Name { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the design.")]
    public string Description { get; set; } = "";
    [Url("🪙", "Ic?", "Ico?", "The optional icon [ emoji | logogram | url ] of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.")]
    public string Icon { get; set; } = "";
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.")]
    public string Image { get; set; } = "";
    [Name("🔀", "Vn?", "Vnt?", "The optional variant of the design. No variant means the default variant.", PropImportance.ID, true)]
    public string Variant { get; set; } = "";
    [Name("🥽", "Vw?", "Vew?", "The optional view of the design. No view means the default view.", PropImportance.ID, true)]
    public string View { get; set; } = "";
    [ModelProp("📍", "Lo?", "Loc?", "The optional location of the design.", PropImportance.OPTIONAL)]
    public Location? Location { get; set; }
    [Name("Ⓜ️", "Ut", "Unt", "The length unit for all distance-related information of the design.", PropImportance.REQUIRED)]
    public string Unit { get; set; } = "";
    [ModelProp("⭕", "Pc*", "Pcs*", "The optional pieces of the design.", PropImportance.OPTIONAL)]
    public List<Piece> Pieces { get; set; } = new();
    [ModelProp("🔗", "Co*", "Cons*", "The optional connections of the design.", PropImportance.OPTIONAL)]
    public List<Connection> Connections { get; set; } = new();
    [ModelProp("👥", "Au*", "Auts*", "The optional authors of the design.", PropImportance.OPTIONAL)]
    public List<Author> Authors { get; set; } = new();
    [ModelProp("📏", "Ql*", "Qals*", "The optional qualities of the design.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();

    public string ToIdString() => $"{Name}#{Variant}#{View}";
    public string ToHumanIdString() => $"{Name}" + (Variant.Length == 0 ? "" : $", {Variant}") + (View.Length == 0 ? "" : $", {View}");
    public override string ToString() => $"Dsn({ToHumanIdString()})";
    public void Bfs(Action<Piece> onRoot, Action<Piece, Piece, Connection> onConnection)
    {
        var pieces = Pieces.ToDictionary(p => p.Id);
        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var piece in Pieces)
            graph.AddVertex(piece.Id);
        foreach (var connection in Connections)
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
            foreach (var connection in Connections)
                if (component.Value.ContainsKey(connection.Connected.Piece.Id) &&
                    component.Value.ContainsKey(connection.Connecting.Piece.Id))
                    subGraph.AddEdge(
                        new Edge<string>(connection.Connected.Piece.Id, connection.Connecting.Piece.Id));
            var root = subGraph.Vertices.FirstOrDefault(p => pieces[p].Plane != null);
            if (root == null)
                root = subGraph.Vertices.First();

            onRoot(pieces[root]);

            var bfs = new UndirectedBreadthFirstSearchAlgorithm<string, Edge<string>>(subGraph);
            bfs.SetRootVertex(root);
            bfs.TreeEdge += (g, edge) =>
            {
                var parent = pieces[edge.Source];
                var child = pieces[edge.Target];
                var connection = Connections.First(c =>
                    (c.Connected.Piece.Id == parent.Id && c.Connecting.Piece.Id == child.Id) ||
                    (c.Connected.Piece.Id == child.Id && c.Connecting.Piece.Id == parent.Id));
                onConnection(parent, child, connection);
            };
            bfs.Compute();
        }
    }

    public Design Flatten(IEnumerable<Type> types,
        Func<Plane, Point, Vector, Point, Vector, float, float, float, float, float, float, Plane> computeChildPlane)
    {
        if (Pieces.Count > 1 && Connections.Count > 0)
        {
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

            foreach (var piece in Pieces)
            {
                if (piece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({piece.Id}) has no type.");
                if (!types.Any(t => t.Name == piece.Type.Name && t.Variant == piece.Type.Variant))
                    throw new Exception(
                        $"The type {piece.Type.ToHumanIdString()} of the piece {piece.ToHumanIdString()} is not provided.");
            }
            foreach (var connection in Connections)
            {
                var connectedPiece = Pieces.First(p => p.Id == connection.Connected.Piece.Id);
                if (connectedPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectedPiece.Id}) has no type.");
                var connectedType = types.First(t => t.Name == connectedPiece.Type.Name && t.Variant == connectedPiece.Type.Variant);
                if (!ports[connectedType.Name].ContainsKey(connectedType.Variant))
                    throw new Exception(
                        $"The type {connectedType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the port {connection.Connected.Port.Id}.");
                var connectingPiece = Pieces.First(p => p.Id == connection.Connecting.Piece.Id);
                if (connectingPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectingPiece.Id}) has no type.");
                var connectingType = types.First(t => t.Name == connectingPiece.Type.Name && t.Variant == connectingPiece.Type.Variant);
                if (!ports[connectingType.Name].ContainsKey(connectingType.Variant))
                    throw new Exception(
                        $"The type {connectingType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the port {connection.Connecting.Port.Id}.");
            }

            var onRoot = new Action<Piece>(piece =>
            {
                if (piece.Plane == null) piece.Plane = new Plane();
                if (piece.Center == null) piece.Center = new DiagramPoint();
            });
            var onConnection = new Action<Piece, Piece, Connection>((parent, child, connection) =>
            {
                var isParentConnected = connection.Connected.Piece.Id == parent.Id;
                var parentPlane = parent.Plane;
                var parentPort =
                    ports[parent.Type.Name][parent.Type.Variant][
                        isParentConnected ? connection.Connected.Port.Id : connection.Connecting.Port.Id];
                var childPort =
                    ports[child.Type.Name][child.Type.Variant][
                        isParentConnected ? connection.Connecting.Port.Id : connection.Connected.Port.Id];
                var childPlane = computeChildPlane(parentPlane, parentPort.Point, parentPort.Direction,
                    childPort.Point, childPort.Direction,
                    connection.Gap, connection.Shift, connection.Rise,
                    connection.Rotation, connection.Turn, connection.Tilt);
                child.Plane = childPlane;

                var direction = new DiagramPoint
                {
                    X = connection.X,
                    Y = connection.Y
                }.Normalize();
                var childCenter = new DiagramPoint
                {
                    X = parent.Center.X + connection.X + direction.X,
                    Y = parent.Center.Y + connection.Y + direction.Y
                };
                child.Center = childCenter;
                var semioQuality = child.Qualities.FirstOrDefault(q => q.Name == "semio.parent");
                if (semioQuality != null)
                {
                    semioQuality.Value = parent.Id;
                }
                else
                {
                    child.Qualities.Add(new Quality
                    {
                        Name = "semio.parent",
                        Value = parent.Id
                    });
                }
            });
            Bfs(onRoot, onConnection);
        }

        Connections = new List<Connection>();

        return this;
    }
    public Design Sort()
    {
        var sortedPieces = new List<Piece>();
        var sortedConnections = new List<Connection>();

        Bfs(
            piece => { sortedPieces.Add(piece); },
            (parent, child, connection) =>
            {
                sortedPieces.Add(child);
                if (connection.Connected.Piece.Id != parent.Id)
                {
                    connection.Connected.Piece = new PieceId { Id = child.Id };
                    connection.Connecting.Piece = new PieceId { Id = parent.Id };
                }

                sortedConnections.Add(connection);
            });

        Pieces = sortedPieces;
        Connections = sortedConnections;

        return this;
    }

    public Piece Piece(string id) => Pieces.Find(piece => piece.Id == id);
    private Design FlatToSvgCoordinates(float iconWidth, float iconWidthMax, float margin)
    {
        // scale to iconWidth and change coordinate system
        foreach (var piece in Pieces)
        {
            piece.Center.X = piece.Center.X * iconWidth;
            piece.Center.Y = -(piece.Center.Y * iconWidth);
        }

        foreach (var connection in Connections)
        {
            connection.X = connection.X * iconWidth;
            connection.Y = -(connection.Y * iconWidth);
        }

        // recenter
        var maxIconOffset = iconWidthMax - iconWidth;
        var minX = Pieces.Min(piece => piece.Center.X) - (margin + maxIconOffset);
        var minY = Pieces.Min(piece => piece.Center.Y) - (margin + maxIconOffset);
        var minXSign = Math.Sign(minX);
        var minYSign = Math.Sign(minY);
        var offsetX = minXSign == 0 ? 0 : -minX;
        var offsetY = minYSign == 0 ? 0 : -minY;
        foreach (var piece in Pieces)
        {
            piece.Center.X += offsetX;
            piece.Center.Y += offsetY;
        }

        return this;
    }

    // TODO: Remove computeChildPlane and separate the flatten diagram and flatten planes parts.
    // TODO: Parametrize colors for diagram
    // TODO: Make remote uris work for diagram.
    public string Diagram(
        IEnumerable<Type> types,
        Func<Plane, Point, Vector, Point, Vector, float, float, float, float, float, float, Plane> computeChildPlane,
        string kitDirectory = "",
        float iconWidth = 48, float iconStroke = 1f, float connectionStroke = 2f, float margin = 0)
    {
        var typesDict = Type.EnumerableToDict(types);

        var usedTypes = new List<Type>();
        foreach (var type in types)
            if (Pieces.Exists(piece => piece.Type.Name == type.Name && piece.Type.Variant == type.Variant))
                usedTypes.Add(type);

        var flatCloneInSvgCoordinates = DeepClone().Flatten(types, computeChildPlane)
            .FlatToSvgCoordinates(iconWidth, iconWidth + 2 * iconStroke, margin);

        var svgDoc = new SvgDocument
        {
            Width = flatCloneInSvgCoordinates.Pieces.Max(piece => piece.Center.X) + margin * 2 + iconWidth +
                    2 * iconStroke,
            Height = flatCloneInSvgCoordinates.Pieces.Max(piece => piece.Center.Y) + margin * 2 + iconWidth +
                     2 * iconStroke
        };

        var defs = new SvgDefinitionList();

        var iconCircle = new SvgCircle
        {
            ID = "icon",
            CenterX = iconWidth / 2,
            CenterY = iconWidth / 2,
            Radius = iconWidth / 2 - iconStroke / 2,
            Fill = new SvgColourServer(Color.White),
            Stroke = new SvgColourServer(Color.Black),
            StrokeWidth = iconStroke
        };
        defs.Children.Add(iconCircle);

        var root = new SvgCircle
        {
            ID = "root",
            CenterX = iconWidth / 2,
            CenterY = iconWidth / 2,
            Radius = iconWidth / 2 + iconStroke,
            Fill = new SvgColourServer(Color.White),
            Stroke = new SvgColourServer(Color.Black),
            StrokeWidth = iconStroke
        };
        defs.Children.Add(root);

        var iconMask = new SvgMask
        {
            ID = "iconMask",
            Children =
            {
                new SvgCircle
                {
                    CenterX = iconWidth / 2 - iconStroke,
                    CenterY = iconWidth / 2 - iconStroke,
                    Radius = iconWidth / 2 - iconStroke,
                    Fill = new SvgColourServer(Color.White)
                }
            }
        };
        defs.Children.Add(iconMask);

        foreach (var type in usedTypes)
        {
            var typeDef = new SvgGroup
            {
                ID = type.ToIdString()
            };
            var icon = type.Icon;
            var iconKind = Utility.ParseIconKind(icon);
            if (iconKind == IconKind.Logogram)
            {
                // TODO: Variable font size to fit logogram text to width
                var fontSize = iconWidth / 2;
                var text = new SvgText
                {
                    Text = icon,
                    FontSize = fontSize,
                    TextAnchor = SvgTextAnchor.Middle,
                    Fill = new SvgColourServer(Color.Black),
                    // TODO: Mask the icon logogram text
                    CustomAttributes =
                    {
                        // { "mask", "url(#iconMask)" }
                    }
                };
                var textTransformed = new SvgGroup
                {
                    Children = { text }
                };
                var textTransform = new SvgTransformCollection
                {
                    new SvgTranslate(iconWidth / 2, iconStroke + iconWidth / 2 + fontSize / 4)
                };
                textTransformed.Transforms = textTransform;
                typeDef.Children.Add(new SvgUse { CustomAttributes = { { "href", "#icon" } } });
                typeDef.Children.Add(textTransformed);
            }
            else
            {
                if (iconKind == IconKind.Filepath)
                    icon = Path.Combine(kitDirectory, icon);

                var image = new SvgImage
                {
                    Width = iconWidth - 2 * iconStroke,
                    Height = iconWidth - 2 * iconStroke,
                    CustomAttributes =
                    {
                        { "href", Utility.DatastringFromUrl(icon) },
                        { "mask", "url(#iconMask)" }
                    }
                };
                var imageTransformed = new SvgGroup
                {
                    Children = { image }
                };
                var imageTransform = new SvgTransformCollection
                {
                    new SvgTranslate(iconStroke, iconStroke)
                };
                imageTransformed.Transforms = imageTransform;
                typeDef.Children.Add(new SvgUse { CustomAttributes = { { "href", "#icon" } } });
                typeDef.Children.Add(imageTransformed);
            }

            defs.Children.Add(typeDef);
        }

        svgDoc.Children.Add(defs);

        var connections = new SvgGroup { ID = "connections" };

        foreach (var connection in Connections)
        {
            var connectedPieceFlat = flatCloneInSvgCoordinates.Piece(connection.Connected.Piece.Id);
            var connectingPieceFlat = flatCloneInSvgCoordinates.Piece(connection.Connecting.Piece.Id);
            var connectionLine = new SvgLine
            {
                StartX = connectedPieceFlat.Center.X + iconWidth / 2,
                StartY = connectedPieceFlat.Center.Y + iconWidth / 2,
                EndX = connectingPieceFlat.Center.X + iconWidth / 2,
                EndY = connectingPieceFlat.Center.Y + iconWidth / 2,
                Stroke = new SvgColourServer(Color.Black),
                StrokeWidth = connectionStroke,
                Children = { new SvgTitle { Content = connection.ToIdString() } }
            };
            connections.Children.Add(connectionLine);
        }

        svgDoc.Children.Add(connections);

        var pieces = new SvgGroup { ID = "pieces" };

        foreach (var piece in Pieces)
        {
            var flatPiece = flatCloneInSvgCoordinates.Piece(piece.Id);
            if (piece.Center != null)
            {
                var rootPiece = new SvgUse
                {
                    CustomAttributes = { { "href", "#root" } },
                    X = flatPiece.Center.X,
                    Y = flatPiece.Center.Y
                };
                pieces.Children.Add(rootPiece);
            }

            var pieceIcon = new SvgUse
            {
                CustomAttributes =
                    { { "href", "#" + typesDict[flatPiece.Type.Name][flatPiece.Type.Variant].ToIdString() } },
                X = flatPiece.Center.X,
                Y = flatPiece.Center.Y,
                Children = { new SvgTitle { Content = flatPiece.Id } }
            };
            pieces.Children.Add(pieceIcon);
        }

        svgDoc.Children.Add(pieces);

        var svg = svgDoc.GetXML();

        var xml = new XmlDocument();
        xml.LoadXml(svg);
        var styleElement = xml.CreateElement("style");
        styleElement.InnerXml = @"
@font-face {
  font-family: ""Anta"";
  src: url(""data:application/truetype;base64," + Resources.Anta + @""");
}

@font-face {
  font-family: ""Noto Emoji"";
  src: url(""data:application/truetype;base64," + Resources.NotoEmoji + @""");
}

text {
  font-family: ""Anta"", ""Noto Emoji"";
}";
        xml.DocumentElement.PrependChild(styleElement);
        return xml.OuterXml.Replace(" xmlns=\"\"", "");
    }


    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var piece in Pieces)
        {
            var (isValidPiece, errorsPiece) = piece.Validate();
            isValid = isValid && isValidPiece;
            errors.AddRange(errorsPiece.Select(e => $"A piece({piece.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var connection in Connections)
        {
            var (isValidConnection, errorsConnection) = connection.Validate();
            isValid = isValid && isValidConnection;
            errors.AddRange(errorsConnection.Select(e =>
                $"A connection({connection.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var author in Authors)
        {
            var (isValidAuthor, errorsAuthor) = author.Validate();
            isValid = isValid && isValidAuthor;
            errors.AddRange(errorsAuthor.Select(e => $"An author({author.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var quality in Qualities)
        {
            var (isValidQuality, errorsQuality) = quality.Validate();
            isValid = isValid && isValidQuality;
            errors.AddRange(errorsQuality.Select(e => $"A quality({quality.ToHumanIdString()}) is invalid: " + e));
        }

        var pieceIds = Pieces.Select(p => p.Id);
        var duplicatePieceIds = pieceIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicatePieceIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicatePieceId in duplicatePieceIds)
                errors.Add($"A piece is invalid: There are multiple pieces with id ({duplicatePieceId}).");
        }

        var nonExistingConnectedPieces = Connections.Where(c => !pieceIds.Contains(c.Connected.Piece.Id)).ToList()
            .Select(c => c.Connected.Piece.Id).ToArray();
        if (nonExistingConnectedPieces.Length != 0)
        {
            isValid = false;
            foreach (var nonExistingConnectedPiece in nonExistingConnectedPieces)
            {
                var connection = Connections.First(c => c.Connected.Piece.Id == nonExistingConnectedPiece);
                errors.Add(
                    $"A connection({connection.ToHumanIdString()}) is invalid: The referenced connected piece ({nonExistingConnectedPiece}) is not part of the design.");
            }
        }

        var nonExistingConnectingPieces = Connections.Where(c => !pieceIds.Contains(c.Connecting.Piece.Id)).ToList()
            .Select(c => c.Connecting.Piece.Id).ToArray();
        if (nonExistingConnectingPieces.Length != 0)
        {
            isValid = false;
            foreach (var nonExistingConnectingPiece in nonExistingConnectingPieces)
            {
                var connection = Connections.First(c => c.Connecting.Piece.Id == nonExistingConnectingPiece);
                errors.Add(
                    $"A connection({connection.ToHumanIdString()}) is invalid: The referenced connecting piece ({nonExistingConnectingPiece}) is not part of the design.");
            }
        }

        var connectionKeys = Connections
            .Select(c => (
                ConnectedPieceId: c.Connected.Piece.Id,
                ConnectedDesignPieceId: c.Connected.DesignPiece?.Id ?? "",
                ConnectingPieceId: c.Connecting.Piece.Id,
                ConnectingDesignPieceId: c.Connecting.DesignPiece?.Id ?? ""))
            .ToList();
        var duplicateConnections = connectionKeys
            .GroupBy(k => k)
            .Where(g => g.Count() > 1)
            .Select(g => g.Key)
            .ToArray();
        if (duplicateConnections.Length != 0)
        {
            isValid = false;
            foreach (var key in duplicateConnections)
                errors.Add($"A connection is duplicated for ({key.ConnectedPieceId},{key.ConnectedDesignPieceId},{key.ConnectingPieceId},{key.ConnectingDesignPieceId}).");
        }

        return (isValid, errors);
    }
}

[Model("🏙️", "Dn", "Dsn", "The local identifier of the design within the kit.")]
public class DesignId : Model<DesignId>
{
    [Name("📛", "Na", "Nam", "The name of the design.", PropImportance.ID)]
    public string Name { get; set; } = "";
    [Name("🔀", "Vn?", "Vnt?", "The optional variant of the design. No variant means the default variant.", PropImportance.ID, true)]
    public string Variant { get; set; } = "";
    [Name("🥽", "Vw?", "Vew?", "The optional view of the design. No view means the default view.", PropImportance.ID, true)]
    public string View { get; set; } = "";
    public static implicit operator DesignId(Design design) => new() { Name = design.Name, Variant = design.Variant, View = design.View };

    public string ToHumanIdString() => $"{Name}{(Variant == "" ? "" : ", " + Variant)}{(View == "" ? "" : ", " + View)}";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-kit-"/>
/// </summary>
[Model("🗃️", "Kt", "Kit", "A kit is a collection of types and designs.")]
public class Kit : Model<Kit>
{
    [Name("📛", "Na", "Nam", "The name of the kit.", PropImportance.ID)]
    public string Name { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the kit.")]
    public string Description { get; set; } = "";
    [Url("🪙", "Ic?", "Ico?", "The optional icon [ emoji | logogram | url ] of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.")]
    public string Icon { get; set; } = "";
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.")]
    public string Image { get; set; } = "";
    [Url("🔮", "Pv?", "Prv?", "The optional url of the preview image of the kit. The url must point to a landscape image [ png | jpg | svg ] which will be cropped by a 2x1 rectangle. The image must be at least 1920x960 pixels and smaller than 15 MB.")]
    public string Preview { get; set; } = "";
    [Name("🔀", "Vr?", "Ver?", "The optional version of the kit. No version means the latest version.", PropImportance.ID, true)]
    public string Version { get; set; } = "";
    [Url("☁️", "Rm?", "Rmt?", "The optional Unique Resource Locator (URL) where to fetch the kit remotely.")]
    public string Remote { get; set; } = "";
    [Url("🏠", "Hp?", "Hmp?", "The optional Unique Resource Locator (URL) of the homepage of the kit.")]
    public string Homepage { get; set; } = "";
    [Url("⚖️", "Li?", "Lic?", "The optional license [ spdx id | url ] of the kit.")]
    public string License { get; set; } = "";
    [ModelProp("🧩", "Ty*", "Typs*", "The optional types of the kit.", PropImportance.OPTIONAL)]
    public List<Type> Types { get; set; } = new();
    [ModelProp("🏙️", "Dn*", "Dsns*", "The optional designs of the kit.", PropImportance.OPTIONAL)]
    public List<Design> Designs { get; set; } = new();
    [ModelProp("📏", "Ql*", "Qals*", "The optional qualities of the kit.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();

    // TODO: Implement reflexive validation for model properties.
    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        // TODO: Develop a validation template for urls.
        //if (Icon != "" && Utility.UriIsNotAbsoluteFilePath(Icon))
        //{
        //    isValid = false;
        //    errors.Add("The icon url can't be absolute.");
        //}
        //if (Image != "" && Utility.UriIsNotAbsoluteFilePath(Image))
        //{
        //    isValid = false;
        //    errors.Add("The image url can't be absolute.");
        //}
        //if (Preview != "" && Utility.UriIsNotAbsoluteFilePath(Preview))
        //{
        //    isValid = false;
        //    errors.Add("The preview url can't be absolute.");
        //}
        foreach (var type in Types)
        {
            var (isValidType, errorsType) = type.Validate();
            isValid = isValid && isValidType;
            errors.AddRange(errorsType.Select(e => $"A type ({type.ToIdString()}) is invalid: " + e));
        }
        foreach (var design in Designs)
        {
            var (isValidDesign, errorsDesign) = design.Validate();
            isValid = isValid && isValidDesign;
            errors.AddRange(errorsDesign.Select(e => $"A design ({design.ToIdString()}) is invalid: " + e));
        }
        var typeIds = Types.Select(t => (t.Name, t.Variant));
        var duplicateTypeIds = typeIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicateTypeIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicateVariant in duplicateTypeIds)
            {
                var message = $"There are multiple identical types ({duplicateVariant.Name}) with ";
                message += duplicateVariant.Variant == ""
                    ? "the default variant."
                    : $"variant({duplicateVariant.Variant}).";
                errors.Add(message);
            }
        }
        var designIds = Designs.Select(d => (d.Name, d.Variant, d.View));
        var duplicateDesignIds = designIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicateDesignIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicateDesignId in duplicateDesignIds)
            {
                var message = $"There are multiple identical designs ({duplicateDesignId.Name}) with ";
                message += duplicateDesignId.Variant == ""
                    ? "the default variant "
                    : $"variant({duplicateDesignId.Variant}) ";
                message += duplicateDesignId.View == ""
                    ? "with the default view."
                    : $"with view({duplicateDesignId.View}).";
                errors.Add(message);
            }
        }
        foreach (var quality in Qualities)
        {
            var (isValidQuality, errorsQuality) = quality.Validate();
            isValid = isValid && isValidQuality;
            errors.AddRange(errorsQuality.Select(e => $"A quality ({quality.ToIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }
}

#endregion

#endregion

#region Api

public class ApiException : Exception { public ApiException(string message) : base(message) { } }
public class ServerException : ApiException { public ServerException(string message) : base(message) { } }
public class ClientException : ApiException { public ClientException(string message) : base(message) { } }

public class PredictDesignBody { public string Description { get; set; } public Type[] Types { get; set; } public Design? Design { get; set; } }

public interface IApi
{
    [Get("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<Kit>> GetKit(string encodedKitUri);

    [Put("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> CreateKit(string encodedKitUri, [Body] Kit input);

    [Delete("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> DeleteKit(string encodedKitUri);


    [Put("/api/kits/{encodedKitUri}/types/{encodedTypeNameAndVariant}")]
    Task<ApiResponse<bool>> PutType(string encodedKitUri, string encodedTypeNameAndVariant, [Body] Type input);

    [Delete("/api/kits/{encodedKitUri}/types/{encodedTypeNameAndVariant}")]
    Task<ApiResponse<bool>> RemoveType(string encodedKitUri, string encodedTypeNameAndVariant);

    [Put("/api/kits/{encodedKitUri}/designs/{encodedDesignNameAndVariantAndView}")]
    Task<ApiResponse<bool>> PutDesign(string encodedKitUri, string encodedDesignNameAndVariantAndView,
        [Body] Design input);

    [Delete("/api/kits/{encodedKitUri}/designs/{encodedDesignNameAndVariantAndView}")]
    Task<ApiResponse<bool>> RemoveDesign(string encodedKitUri, string encodedDesignNameAndVariantAndView);

    [Get("/api/assistant/predictDesign")]
    Task<ApiResponse<Design>> PredictDesign([Body] PredictDesignBody body);
}

public static class Api
{
    private static IApi GetApi()
    {
        var httpClient = new HttpClient
        {
            BaseAddress = new Uri(Constants.EngineAddress),
            Timeout = TimeSpan.FromMinutes(3)
        };
        return RestService.For<IApi>(httpClient, new RefitSettings
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
        if (response.StatusCode == HttpStatusCode.BadRequest) throw new ClientException(response.Error.Content);
        if (!response.IsSuccessStatusCode) throw new ServerException(UnsuccessfullResponseToString(response));
    }

    public static string EncodeNameAndVariant(string name, string variant = "") => Utility.Encode(name) + "," + Utility.Encode(variant);

    public static string EncodeNameAndVariantAndView(string name, string variant = "", string view = "") => EncodeNameAndVariant(name, variant) + "," + Utility.Encode(view);

    public static Kit GetKit(string uri)
    {
        var response = GetApi().GetKit(Utility.Encode(uri)).Result;
        if (response.IsSuccessStatusCode)
            return response.Content;
        HandleErrors(response);
        return null; // This line will never be reached, but is required to satisfy the compiler.
    }

    public static void CreateKit(string uri, Kit input) => HandleErrors(GetApi().CreateKit(Utility.Encode(uri), input).Result);

    public static void DeleteKit(string uri) => HandleErrors(GetApi().DeleteKit(Utility.Encode(uri)).Result);

    public static void PutType(string kitUrl, Type input) => HandleErrors(GetApi().PutType(Utility.Encode(kitUrl), EncodeNameAndVariant(input.Name, input.Variant), input).Result);

    public static void RemoveType(string kitUrl, TypeId id) => HandleErrors(GetApi().RemoveType(Utility.Encode(kitUrl), EncodeNameAndVariant(id.Name, id.Variant)).Result);

    public static void PutDesign(string kitUrl, Design input) => HandleErrors(GetApi().PutDesign(Utility.Encode(kitUrl), EncodeNameAndVariantAndView(input.Name, input.Variant, input.View), input).Result);

    public static void RemoveDesign(string kitUrl, DesignId id) => HandleErrors(GetApi().RemoveDesign(Utility.Encode(kitUrl), EncodeNameAndVariantAndView(id.Name, id.Variant, id.View)).Result);


    public static Design PredictDesign(string description, Type[] types, Design design)
    {
        var response = GetApi().PredictDesign(new PredictDesignBody
        { Description = description, Types = types, Design = design }).Result;
        if (response.IsSuccessStatusCode)
            return response.Content;
        HandleErrors(response);
        return null; // This line will never be reached, but is required to satisfy the compiler.
    }
}

#endregion

#region Meta

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

#endregion
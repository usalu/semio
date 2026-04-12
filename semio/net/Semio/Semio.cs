#region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Core .NET library implementing the semio domain model and serialization.

#endregion 🧲Header






#region ⛩️Imports
// Callers MUST import all required namespaces listed here.
using System.Collections;
using System.Collections.Immutable;
using System.Drawing;
using System.Globalization;
using System.Net;
using System.Net.Http;
using System.Numerics;
using System.Reflection;
using System.Text;
using System.Xml;
using System.IO.Compression;
using Microsoft.Data.Sqlite;
using FluentValidation;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Serialization;
using System.Runtime.Serialization;
using QuikGraph;
using QuikGraph.Algorithms;
using QuikGraph.Algorithms.ConnectedComponents;
using QuikGraph.Algorithms.Search;
using Refit;
using Svg;
using Svg.Transforms;
using UnitsNet;
using Formatting = Newtonsoft.Json.Formatting;
using SharpGLTF.Geometry;
using SharpGLTF.Geometry.VertexTypes;
using SharpGLTF.Materials;
using SharpGLTF.Scenes;
using GltfModel = SharpGLTF.Schema2.ModelRoot;
using GltfNode = SharpGLTF.Schema2.Node;

#endregion ⛩️Imports






#region 🏠Namespace
// Implementations MUST reside in this namespace.
namespace Semio;
#endregion 🏠Namespace






#region 🎞️Constants
// Consumers MUST use these shared constants for configuration.

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
    public const int ExpressionLengthLimit = 4096;
    public const int ValueLengthLimit = 512;
    public const int AttributesMax = 64;
    public const int QualityMax = 1024;
    public const int TagsMax = 8;
    public const int EntitysMax = 32;
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

public enum DiffStatus
{
    Unchanged,
    Added,
    Removed,
    Modified
}

#endregion 🎞️Constants






#region 📦Utilities
// Callers MUST use these utility functions for encoding and serialization.

public static class Utility
{
    public static string Normalize(string val) => string.IsNullOrEmpty(val) ? "" : val;

    public static float Jaccard(IEnumerable<string> a, IEnumerable<string> b)
    {
        var listA = a?.ToList() ?? new List<string>();
        var listB = b?.ToList() ?? new List<string>();

        if (listA.Count == 0 && listB.Count == 0) return 1f;

        var setA = new HashSet<string>(listA);
        var setB = new HashSet<string>(listB);
        var intersection = setA.Intersect(setB).Count();
        var union = setA.Union(setB).Count();

        if (union == 0) return 0f;
        return (float)intersection / union;
    }
    public static bool UriIsNotAbsoluteFilePath(string uri)
    {
        return !(Uri.IsWellFormedUriString(uri, UriKind.Relative) || uri.StartsWith("http"));
    }
    public static bool IsValidMime(string mime)
    {
        var validMimes = new List<string>
        {
            "entity/stl",
            "entity/obj",
            "entity/gltf-binary",
            "entity/gltf+json",
            "entity/vnd.3dm",
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
            { ".stl", "entity/stl" },
            { ".obj", "entity/obj" },
            { ".glb", "entity/gltf-binary" },
            { ".gltf", "entity/gltf+json" },
            { ".3dm", "entity/vnd.3dm" },
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
                mime = response.Content.Headers.ContentType?.MediaType ?? "";
                content = response.Content.ReadAsByteArrayAsync().Result;
            }
        }
        else
        {
            var osAwareUrl = url.Replace("/", Path.DirectorySeparatorChar.ToString());
            content = System.IO.File.ReadAllBytes(osAwareUrl);
            mime = ParseMimeFromUrl(osAwareUrl);
        }
        return $"data:{mime};base64,{Convert.ToBase64String(content)}";
    }

    public static string ReadAndEncode(string filename) => $"data:{ParseMimeFromUrl(filename)};base64,{Convert.ToBase64String(System.IO.File.ReadAllBytes(filename))}";
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

    public static string Serialize(object obj, string indent = "")
    {
        var isTabbed = indent.StartsWith("\t");
        var formatting = string.IsNullOrEmpty(indent) ? Formatting.None : Formatting.Indented;
        var settings = new JsonSerializerSettings { ContractResolver = new SemioContractResolver(), Formatting = formatting };
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

    public class SemioContractResolver : CamelCasePropertyNamesContractResolver
    {
        protected override JsonProperty CreateProperty(MemberInfo member, MemberSerialization memberSerialization)
        {
            var property = base.CreateProperty(member, memberSerialization);
            var declaringType = member.DeclaringType;
            if (declaringType != null)
            {
                var shouldSerializeMethod = declaringType.GetMethod($"ShouldSerialize{member.Name}", BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance);
                if (shouldSerializeMethod != null && shouldSerializeMethod.ReturnType == typeof(bool))
                {
                    property.ShouldSerialize = instance => (bool)(shouldSerializeMethod.Invoke(instance, null) ?? true);
                }
            }
            return property;
        }
    }

    public static T? Deserialize<T>(string json) => JsonConvert.DeserializeObject<T>(json, new JsonSerializerSettings { ContractResolver = new CamelCasePropertyNamesContractResolver(), ObjectCreationHandling = ObjectCreationHandling.Replace });

    public static string GenerateRandomId(int seed)
    {
        var adjectives = Utility.Deserialize<List<string>>(Resources.adjectives);
        var animals = Utility.Deserialize<List<string>>(Resources.animals);
        if (adjectives is null || animals is null) throw new InvalidOperationException("Failed to deserialize resources");
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
            var convertEntity = new PowerToysRunUnitConverter.ConvertEntity(value, fromUnit, toUnit);
            var results = PowerToysRunUnitConverter.UnitHandler.Convert(convertEntity);
            if (results.Count() == 0) return float.NaN;
            return (float)results.First().ConvertedValue;
        }

        private class PowerToysRunUnitConverter
        {
            internal class ConvertEntity
            {
                internal ConvertEntity() { FromUnit = ""; ToUnit = ""; }
                internal ConvertEntity(double value, string fromUnit, string toUnit) => (Value, FromUnit, ToUnit) = (value, fromUnit, toUnit);
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
                    UnitsNet.Length.Info,
                    Area.Info,
                    Volume.Info,
                    Duration.Info,
                    Energy.Info,
                    UnitsNet.Power.Info,
                    Pressure.Info,
                    Mass.Info,
                    Angle.Info,
                    Temperature.Info,
                    Acceleration.Info,
                    Speed.Info,
                    Information.Info
                };
                private static Enum? GetUnitEnum(string unit, QuantityInfo unitInfo)
                {
                    var first = Array.Find(unitInfo.UnitInfos, info => string.Equals(unit, info.Name, StringComparison.OrdinalIgnoreCase) || string.Equals(unit, info.PluralName, StringComparison.OrdinalIgnoreCase));
                    if (first != null) return first.Value;
                    if (UnitsNetSetup.Default.UnitParser.TryParse(unit, unitInfo.UnitType, out var enum_unit)) return enum_unit;
                    var cultureInfoEnglish = new CultureInfo("en-US");
                    if (UnitsNetSetup.Default.UnitParser.TryParse(unit, unitInfo.UnitType, cultureInfoEnglish, out var enum_unit_en)) return enum_unit_en;
                    return null;
                }
                internal static double ConvertInput(ConvertEntity convertEntity, QuantityInfo quantityInfo)
                {
                    var fromUnit = GetUnitEnum(convertEntity.FromUnit, quantityInfo);
                    var toUnit = GetUnitEnum(convertEntity.ToUnit, quantityInfo);
                    if (fromUnit != null && toUnit != null) return UnitConverter.Convert(convertEntity.Value, fromUnit, toUnit);
                    return double.NaN;
                }
                internal static IEnumerable<UnitConversionResult> Convert(ConvertEntity convertEntity)
                {
                    var results = new List<UnitConversionResult>();
                    foreach (var quantityInfo in _included)
                    {
                        var convertedValue = ConvertInput(convertEntity, quantityInfo);
                        if (!double.IsNaN(convertedValue)) results.Add(new UnitConversionResult(convertedValue, convertEntity.ToUnit, quantityInfo));
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

#region ❄️Expressions
// Implementations MUST evaluate expression trees through the Operator.Apply contract.

/// <summary>🌳Abstract base for all expression tree nodes.</summary>
/// <remarks>
/// </remarks>
public abstract class Symbol { }
public abstract class Term : Symbol { }
public abstract class Constant : Term { }

public class UnitValue
{
    public float Value { get; set; }
    public string Unit { get; set; }

    public UnitValue(float value, string unit = "")
    {
        Value = value;
        Unit = unit ?? "";
    }

    public float ConvertTo(string targetUnit)
    {
        if (string.IsNullOrEmpty(Unit) || string.IsNullOrEmpty(targetUnit) || Unit == targetUnit)
            return Value;
        return Utility.Units.Convert(Value, Unit, targetUnit);
    }

    public override string ToString() => string.IsNullOrEmpty(Unit) ? Value.ToString("G9", CultureInfo.InvariantCulture) : $"'{Value.ToString("G9", CultureInfo.InvariantCulture)} {Unit}'";
}

public class NumberConstant : Constant
{
    public UnitValue UnitValue { get; set; }
    public NumberConstant(float value, string unit = "") { UnitValue = new UnitValue(value, unit); }
    public NumberConstant(UnitValue unitValue) { UnitValue = unitValue ?? new UnitValue(0); }
    public override string ToString() => UnitValue.ToString();
}

public class StringConstant : Constant
{
    public string Value { get; set; }
    public StringConstant(string value) { Value = value ?? string.Empty; }
    public override string ToString() => $"\"{Value}\"";
}

public class Variable : Term
{
    public string Name { get; set; }
    public Variable(string name) { Name = name; }
    public override string ToString() => Name;
}

public abstract class Operator : Symbol
{
    public abstract string Keyword { get; }
    public abstract object Apply(object[] args, string targetUnit = "");

    protected static UnitValue ConvertToUnitValue(object arg)
    {
        return arg switch
        {
            UnitValue uv => uv,
            float f => new UnitValue(f),
            _ => throw new ArgumentException($"Cannot convert {arg?.GetType().Name ?? "null"} to UnitValue")
        };
    }

    protected static UnitValue[] ConvertArgsToUnitValues(object[] args)
    {
        return args.Select(ConvertToUnitValue).ToArray();
    }

    protected static string DetermineCommonUnit(UnitValue[] values)
    {
        var nonEmptyUnits = values.Where(v => !string.IsNullOrEmpty(v.Unit)).ToArray();
        if (nonEmptyUnits.Length == 0) return "";
        return nonEmptyUnits[0].Unit;
    }
}

public class Sum : Operator
{
    public override string Keyword => "sum";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) return new UnitValue(0);

        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(unitValues) : targetUnit;
        float sum = 0;

        foreach (var uv in unitValues)
        {
            if (string.IsNullOrEmpty(commonUnit))
                sum += uv.Value;
            else
                sum += uv.ConvertTo(commonUnit);
        }

        return new UnitValue(sum, commonUnit);
    }
}

public class Multiply : Operator
{
    public override string Keyword => "multiply";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) return new UnitValue(1);

        float result = 1f;
        var units = new List<string>();

        foreach (var uv in unitValues)
        {
            result *= uv.Value;
            if (!string.IsNullOrEmpty(uv.Unit))
                units.Add(uv.Unit);
        }

        var combinedUnit = string.Join("·", units);
        return new UnitValue(result, combinedUnit);
    }
}

public class Subtract : Operator
{
    public override string Keyword => "subtract";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length < 2) throw new ArgumentException("subtract requires at least 2 operands");

        var commonUnit = DetermineCommonUnit(unitValues);
        float result = string.IsNullOrEmpty(commonUnit) ? unitValues[0].Value : unitValues[0].ConvertTo(commonUnit);

        for (int i = 1; i < unitValues.Length; i++)
        {
            result -= string.IsNullOrEmpty(commonUnit) ? unitValues[i].Value : unitValues[i].ConvertTo(commonUnit);
        }

        return new UnitValue(result, commonUnit);
    }
}

public class Divide : Operator
{
    public override string Keyword => "divide";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length < 2) throw new ArgumentException("divide requires at least 2 operands");

        float acc = unitValues[0].Value;
        var numeratorUnit = unitValues[0].Unit;
        var denominatorUnits = new List<string>();

        for (int i = 1; i < unitValues.Length; i++)
        {
            if (unitValues[i].Value == 0f) throw new DivideByZeroException("division by zero");
            acc /= unitValues[i].Value;
            if (!string.IsNullOrEmpty(unitValues[i].Unit))
                denominatorUnits.Add(unitValues[i].Unit);
        }

        var resultUnit = "";
        if (!string.IsNullOrEmpty(numeratorUnit) || denominatorUnits.Count > 0)
        {
            var denominatorPart = denominatorUnits.Count > 0 ? string.Join("·", denominatorUnits) : "";
            if (!string.IsNullOrEmpty(numeratorUnit) && !string.IsNullOrEmpty(denominatorPart))
                resultUnit = $"{numeratorUnit}/{denominatorPart}";
            else if (!string.IsNullOrEmpty(numeratorUnit))
                resultUnit = numeratorUnit;
            else if (!string.IsNullOrEmpty(denominatorPart))
                resultUnit = $"1/{denominatorPart}";
        }

        return new UnitValue(acc, resultUnit);
    }
}

public class Negate : Operator
{
    public override string Keyword => "negate";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("negate requires exactly 1 operand");
        var unitValue = ConvertToUnitValue(args[0]);
        return new UnitValue(-unitValue.Value, unitValue.Unit);
    }
}

public class SquareRoot : Operator
{
    public override string Keyword => "sqrt";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("sqrt requires exactly 1 operand");
        var unitValue = ConvertToUnitValue(args[0]);
        if (unitValue.Value < 0f) throw new ArgumentException("sqrt requires non-negative operand");
        var resultUnit = string.IsNullOrEmpty(unitValue.Unit) ? "" : $"√({unitValue.Unit})";
        return new UnitValue((float)Math.Sqrt(unitValue.Value), resultUnit);
    }
}

public class Power : Operator
{
    public override string Keyword => "power";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("power requires exactly 2 operands");
        var baseValue = ConvertToUnitValue(args[0]);
        var exponent = ConvertToUnitValue(args[1]);
        var resultUnit = string.IsNullOrEmpty(baseValue.Unit) ? "" : $"({baseValue.Unit})^{exponent.Value}";
        return new UnitValue((float)Math.Pow(baseValue.Value, exponent.Value), resultUnit);
    }
}

public class Min : Operator
{
    public override string Keyword => "min";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) throw new ArgumentException("min requires at least 1 operand");

        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(unitValues) : targetUnit;
        float minValue = float.MaxValue;

        foreach (var uv in unitValues)
        {
            var value = string.IsNullOrEmpty(commonUnit) ? uv.Value : uv.ConvertTo(commonUnit);
            if (value < minValue) minValue = value;
        }

        return new UnitValue(minValue, commonUnit);
    }
}

public class Max : Operator
{
    public override string Keyword => "max";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) throw new ArgumentException("max requires at least 1 operand");

        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(unitValues) : targetUnit;
        float maxValue = float.MinValue;

        foreach (var uv in unitValues)
        {
            var value = string.IsNullOrEmpty(commonUnit) ? uv.Value : uv.ConvertTo(commonUnit);
            if (value > maxValue) maxValue = value;
        }

        return new UnitValue(maxValue, commonUnit);
    }
}

public class Average : Operator
{
    public override string Keyword => "average";
    public override object Apply(object[] args, string targetUnit = "")
    {
        var unitValues = ConvertArgsToUnitValues(args);
        if (unitValues.Length == 0) throw new ArgumentException("average requires at least 1 operand");

        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(unitValues) : targetUnit;
        float sum = 0;

        foreach (var uv in unitValues)
        {
            sum += string.IsNullOrEmpty(commonUnit) ? uv.Value : uv.ConvertTo(commonUnit);
        }

        return new UnitValue(sum / unitValues.Length, commonUnit);
    }
}

public class Modulo : Operator
{
    public override string Keyword => "mod";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("mod requires exactly 2 operands");
        var value1 = ConvertToUnitValue(args[0]);
        var value2 = ConvertToUnitValue(args[1]);
        var commonUnit = string.IsNullOrEmpty(targetUnit) ? DetermineCommonUnit(new[] { value1, value2 }) : targetUnit;

        var val1 = string.IsNullOrEmpty(commonUnit) ? value1.Value : value1.ConvertTo(commonUnit);
        var val2 = string.IsNullOrEmpty(commonUnit) ? value2.Value : value2.ConvertTo(commonUnit);

        return new UnitValue(val1 % val2, commonUnit);
    }
}

public class And : Operator
{
    public override string Keyword => "and";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length < 2) throw new ArgumentException("and requires at least 2 operands");
        return args.Cast<float>().All(x => x != 0f) ? 1f : 0f;
    }
}

public class Or : Operator
{
    public override string Keyword => "or";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length < 2) throw new ArgumentException("or requires at least 2 operands");
        return args.Cast<float>().Any(x => x != 0f) ? 1f : 0f;
    }
}

public class ExclusiveOr : Operator
{
    public override string Keyword => "xor";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("xor requires exactly 2 operands");
        bool a = (float)args[0] != 0f;
        bool b = (float)args[1] != 0f;
        return (a ^ b) ? 1f : 0f;
    }
}

public class Invert : Operator
{
    public override string Keyword => "not";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("not requires exactly 1 operand");
        var value = ConvertToUnitValue(args[0]);
        return new UnitValue(value.Value == 0f ? 1f : 0f);
    }
}

public class Equal : Operator
{
    public override string Keyword => "equal";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("equal requires exactly 2 operands");

        if (args[0] is UnitValue uv1 && args[1] is UnitValue uv2)
        {
            var commonUnit = DetermineCommonUnit(new[] { uv1, uv2 });
            var val1 = string.IsNullOrEmpty(commonUnit) ? uv1.Value : uv1.ConvertTo(commonUnit);
            var val2 = string.IsNullOrEmpty(commonUnit) ? uv2.Value : uv2.ConvertTo(commonUnit);
            return new UnitValue(Math.Abs(val1 - val2) < float.Epsilon ? 1f : 0f);
        }

        if (args[0] is float f1 && args[1] is float f2)
            return new UnitValue(Math.Abs(f1 - f2) < float.Epsilon ? 1f : 0f);

        if (args[0] is string s1 && args[1] is string s2)
            return new UnitValue(string.Equals(s1, s2, StringComparison.Ordinal) ? 1f : 0f);

        return new UnitValue(0f);
    }
}

public class GreaterThan : Operator
{
    public override string Keyword => "greater";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("greater requires exactly 2 operands");
        return (float)args[0] > (float)args[1] ? 1f : 0f;
    }
}

public class LessThan : Operator
{
    public override string Keyword => "less";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("less requires exactly 2 operands");
        return (float)args[0] < (float)args[1] ? 1f : 0f;
    }
}

public class GreaterThanOrEqual : Operator
{
    public override string Keyword => "greater-equal";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("greater-equal requires exactly 2 operands");
        return (float)args[0] >= (float)args[1] ? 1f : 0f;
    }
}

public class LessThanOrEqual : Operator
{
    public override string Keyword => "less-equal";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("less-equal requires exactly 2 operands");
        return (float)args[0] <= (float)args[1] ? 1f : 0f;
    }
}

public class If : Operator
{
    public override string Keyword => "if";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("if requires exactly 3 operands: condition, true-value, false-value");
        return (float)args[0] != 0f ? args[1] : args[2];
    }
}

public class Absolute : Operator
{
    public override string Keyword => "abs";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("abs requires exactly 1 operand");
        var uv = ConvertToUnitValue(args[0]);
        return new UnitValue(Math.Abs(uv.Value), uv.Unit);
    }
}

public class Floor : Operator
{
    public override string Keyword => "floor";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("floor requires exactly 1 operand");
        return (float)Math.Floor((float)args[0]);
    }
}

public class Ceiling : Operator
{
    public override string Keyword => "ceil";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("ceil requires exactly 1 operand");
        return (float)Math.Ceiling((float)args[0]);
    }
}

public class Round : Operator
{
    public override string Keyword => "round";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("round requires exactly 1 operand");
        return (float)Math.Round((float)args[0]);
    }
}

public class Length : Operator
{
    public override string Keyword => "length";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("length requires exactly 1 operand");
        return (float)((string)args[0]).Length;
    }
}

public class StartsWith : Operator
{
    public override string Keyword => "startswith";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("startswith requires exactly 2 operands");
        return ((string)args[0]).StartsWith((string)args[1], StringComparison.Ordinal) ? 1f : 0f;
    }
}

public class EndsWith : Operator
{
    public override string Keyword => "endswith";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("endswith requires exactly 2 operands");
        return ((string)args[0]).EndsWith((string)args[1], StringComparison.Ordinal) ? 1f : 0f;
    }
}

public class Contains : Operator
{
    public override string Keyword => "contains";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 2) throw new ArgumentException("contains requires exactly 2 operands");
        return ((string)args[0]).Contains((string)args[1]) ? 1f : 0f;
    }
}

public class Substring : Operator
{
    public override string Keyword => "substring";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length < 2 || args.Length > 3) throw new ArgumentException("substring requires 2 or 3 operands");
        string str = (string)args[0];
        int start = (int)(float)args[1];
        if (args.Length == 3)
        {
            int length = (int)(float)args[2];
            return str.Substring(start, length);
        }
        return str.Substring(start);
    }
}

public class Concat : Operator
{
    public override string Keyword => "concat";
    public override object Apply(object[] args, string targetUnit = "")
    {
        return string.Concat(args.Cast<string>());
    }
}

public class ToUpper : Operator
{
    public override string Keyword => "upper";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("upper requires exactly 1 operand");
        return ((string)args[0]).ToUpper();
    }
}

public class ToLower : Operator
{
    public override string Keyword => "lower";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("lower requires exactly 1 operand");
        return ((string)args[0]).ToLower();
    }
}

public class Trim : Operator
{
    public override string Keyword => "trim";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("trim requires exactly 1 operand");
        return ((string)args[0]).Trim();
    }
}

public class Replace : Operator
{
    public override string Keyword => "replace";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("replace requires exactly 3 operands");
        return ((string)args[0]).Replace((string)args[1], (string)args[2]);
    }
}

public class ToNumber : Operator
{
    public override string Keyword => "number";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("number requires exactly 1 operand");
        if (args[0] is string str)
        {
            if (float.TryParse(str, NumberStyles.Float, CultureInfo.InvariantCulture, out float result))
                return result;
            throw new FormatException($"Cannot convert '{str}' to number");
        }
        return (float)args[0];
    }
}

public class ToText : Operator
{
    public override string Keyword => "text";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("text requires exactly 1 operand");
        if (args[0] is float f)
            return f.ToString(CultureInfo.InvariantCulture);
        return (string)args[0];
    }
}

public class ToBoolean : Operator
{
    public override string Keyword => "boolean";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("boolean requires exactly 1 operand");
        if (args[0] is float f)
            return f != 0f ? 1f : 0f;
        if (args[0] is string s)
            return string.IsNullOrEmpty(s) ? 0f : 1f;
        return 0f;
    }
}

public class Clamp : Operator
{
    public override string Keyword => "clamp";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("clamp requires exactly 3 operands: value, min, max");
        float value = (float)args[0];
        float min = (float)args[1];
        float max = (float)args[2];
        return Math.Max(min, Math.Min(max, value));
    }
}

public class Lerp : Operator
{
    public override string Keyword => "lerp";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("lerp requires exactly 3 operands: a, b, t");
        float a = (float)args[0];
        float b = (float)args[1];
        float t = (float)args[2];
        return a + (b - a) * t;
    }
}

public class Sign : Operator
{
    public override string Keyword => "sign";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("sign requires exactly 1 operand");
        return (float)Math.Sign((float)args[0]);
    }
}

public class IsEmpty : Operator
{
    public override string Keyword => "isempty";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 1) throw new ArgumentException("isempty requires exactly 1 operand");
        if (args[0] is string str)
            return string.IsNullOrEmpty(str) ? 1f : 0f;
        return 0f;
    }
}

public class Operation : Term
{
    public Operator Operator { get; set; }
    public Term[] Operands { get; set; }

    public Operation(Operator op, params Term[] operands)
    {
        Operator = op ?? throw new ArgumentNullException(nameof(op));
        Operands = operands ?? Array.Empty<Term>();
    }

    public object Evaluate(Dictionary<string, object>? context = null, string targetUnit = "")
    {
        object[] values = Operands.Select(o => EvaluateTerm(o, context, targetUnit)).ToArray();
        return Operator.Apply(values, targetUnit);
    }

    private static object EvaluateTerm(Term t, Dictionary<string, object>? ctx, string targetUnit = "")
    {
        switch (t)
        {
            case NumberConstant c:
                return c.UnitValue;
            case StringConstant sc:
                return sc.Value;
            case Variable v:
                if (ctx == null || !ctx.TryGetValue(v.Name, out var val))
                    throw new KeyNotFoundException($"No value provided for variable '{v.Name}'.");
                return val;
            case Operation op:
                return op.Evaluate(ctx, targetUnit);
            default:
                throw new InvalidOperationException($"Unknown term type: {t?.GetType().Name ?? "null"}");
        }
    }
}

public class Expression
{
    public Term? Root { get; private set; }
    private readonly Dictionary<string, Func<Operator>> _operators;

    public Expression()
    {
        _operators = new Dictionary<string, Func<Operator>>(StringComparer.OrdinalIgnoreCase)
        {

            { "sum", () => new Sum() },
            { "multiply", () => new Multiply() },
            { "subtract", () => new Subtract() },
            { "divide", () => new Divide() },
            { "negate", () => new Negate() },
            { "power", () => new Power() },
            { "sqrt", () => new SquareRoot() },
            { "min", () => new Min() },
            { "max", () => new Max() },
            { "abs", () => new Absolute() },
            { "floor", () => new Floor() },
            { "ceil", () => new Ceiling() },
            { "round", () => new Round() },
            { "average", () => new Average() },
            { "mod", () => new Modulo() },

            { "and", () => new And() },
            { "or", () => new Or() },
            { "xor", () => new ExclusiveOr() },
            { "not", () => new Invert() },

            { "equal", () => new Equal() },
            { "greater", () => new GreaterThan() },
            { "less", () => new LessThan() },
            { "greater-equal", () => new GreaterThanOrEqual() },
            { "less-equal", () => new LessThanOrEqual() },

            { "if", () => new If() },

            { "length", () => new Length() },
            { "startswith", () => new StartsWith() },
            { "endswith", () => new EndsWith() },
            { "contains", () => new Contains() },
            { "substring", () => new Substring() },
            { "concat", () => new Concat() },
            { "upper", () => new ToUpper() },
            { "lower", () => new ToLower() },
            { "trim", () => new Trim() },
            { "replace", () => new Replace() },

            { "number", () => new ToNumber() },
            { "text", () => new ToText() },
            { "boolean", () => new ToBoolean() },

            { "clamp", () => new Clamp() },
            { "lerp", () => new Lerp() },
            { "sign", () => new Sign() },
            { "isempty", () => new IsEmpty() }
        };
    }

    public Expression[] Pop()
    {
        if (Root == null) throw new InvalidOperationException("Expression has no root term.");

        if (Root is Operation operation)
        {
            return operation.Operands.Select(operand => new Expression { Root = operand }).ToArray();
        }

        throw new InvalidOperationException("Root term is not an operation, cannot pop operands.");
    }

    public object Calculate(Dictionary<string, object>? context = null, string targetUnit = "")
    {
        if (Root == null) throw new InvalidOperationException("Expression has no root term.");
        return Root switch
        {
            NumberConstant c => string.IsNullOrEmpty(targetUnit) ? c.UnitValue : new UnitValue(c.UnitValue.ConvertTo(targetUnit)),
            StringConstant sc => sc.Value,
            Variable v => context != null && context.TryGetValue(v.Name, out var val)
                            ? val
                            : throw new KeyNotFoundException($"No value provided for variable '{v.Name}'."),
            Operation o => o.Evaluate(context, targetUnit),
            _ => throw new InvalidOperationException("Unknown root term.")
        };
    }

    public string Serialize()
    {
        if (Root == null) return string.Empty;
        var sb = new StringBuilder();
        SerializeTerm(Root, sb);
        return sb.ToString();
    }

    public Expression Deserialize(string expression)
    {
        if (expression == null) throw new ArgumentNullException(nameof(expression));
        var tokens = Tokenize(expression);
        int index = 0;
        Root = ParseExpr(tokens, ref index);
        if (index != tokens.Count)
            throw new FormatException($"Unexpected token '{tokens[index].Text}' at position {tokens[index].Position}.");
        return this;
    }

    private void SerializeTerm(Term term, StringBuilder sb)
    {
        switch (term)
        {
            case NumberConstant c:
                sb.Append(c.UnitValue.ToString());
                break;
            case StringConstant sc:
                sb.Append('"');
                sb.Append(sc.Value.Replace("\"", "\\\""));
                sb.Append('"');
                break;
            case Variable v:
                sb.Append(v.Name);
                break;
            case Operation op:
                sb.Append(op.Operator.Keyword);
                sb.Append(" ( ");
                for (int i = 0; i < op.Operands.Length; i++)
                {
                    if (i > 0) sb.Append(' ');
                    SerializeTerm(op.Operands[i], sb);
                }
                sb.Append(" )");
                break;
            default:
                throw new InvalidOperationException($"Unknown term type for serialization: {term?.GetType().Name ?? "null"}");
        }
    }

    private enum TokenKind { Identifier, Number, String, UnitLiteral, LeftParenthesis, RightParenthesis }

    private readonly struct Token
    {
        public TokenKind Kind { get; }
        public string Text { get; }
        public int Position { get; }
        public Token(TokenKind k, string t, int pos) { Kind = k; Text = t; Position = pos; }
        public override string ToString() => $"{Kind}:{Text}";
    }

    private static readonly HashSet<char> IdentifierExtraChars = new HashSet<char> { '.', '-', '_' };

    private static List<Token> Tokenize(string input)
    {
        var tokens = new List<Token>();
        int i = 0;
        while (i < input.Length)
        {
            char c = input[i];

            if (char.IsWhiteSpace(c)) { i++; continue; }

            if (c == '(') { tokens.Add(new Token(TokenKind.LeftParenthesis, "(", i)); i++; continue; }
            if (c == ')') { tokens.Add(new Token(TokenKind.RightParenthesis, ")", i)); i++; continue; }

            if (c == '"')
            {
                int start = i;
                i++;
                var sb = new StringBuilder();
                while (i < input.Length && input[i] != '"')
                {
                    if (input[i] == '\\' && i + 1 < input.Length)
                    {
                        i++;
                        switch (input[i])
                        {
                            case '"': sb.Append('"'); break;
                            case '\\': sb.Append('\\'); break;
                            case 'n': sb.Append('\n'); break;
                            case 't': sb.Append('\t'); break;
                            case 'r': sb.Append('\r'); break;
                            default: sb.Append(input[i]); break;
                        }
                    }
                    else
                    {
                        sb.Append(input[i]);
                    }
                    i++;
                }
                if (i >= input.Length) throw new FormatException($"Unterminated string literal starting at {start}.");
                i++;
                tokens.Add(new Token(TokenKind.String, sb.ToString(), start));
                continue;
            }

            if (c == '\'')
            {
                int start = i;
                i++;
                var sb = new StringBuilder();
                while (i < input.Length && input[i] != '\'')
                {
                    if (input[i] == '\\' && i + 1 < input.Length)
                    {
                        i++;
                        switch (input[i])
                        {
                            case '\'': sb.Append('\''); break;
                            case '\\': sb.Append('\\'); break;
                            case 'n': sb.Append('\n'); break;
                            case 't': sb.Append('\t'); break;
                            case 'r': sb.Append('\r'); break;
                            default: sb.Append(input[i]); break;
                        }
                    }
                    else
                    {
                        sb.Append(input[i]);
                    }
                    i++;
                }
                if (i >= input.Length) throw new FormatException($"Unterminated unit literal starting at {start}.");
                i++;
                tokens.Add(new Token(TokenKind.UnitLiteral, sb.ToString(), start));
                continue;
            }

            if (char.IsDigit(c) || (c == '.' && i + 1 < input.Length && char.IsDigit(input[i + 1])))
            {
                int start = i;
                i++;
                while (i < input.Length && (char.IsDigit(input[i]) || input[i] == '.')) i++;

                if (i < input.Length && (input[i] == 'e' || input[i] == 'E'))
                {
                    int ePos = i++;
                    if (i < input.Length && (input[i] == '+' || input[i] == '-')) i++;
                    bool hasDigit = false;
                    while (i < input.Length && char.IsDigit(input[i])) { hasDigit = true; i++; }
                    if (!hasDigit) throw new FormatException($"Invalid exponent starting at {ePos}.");
                }
                tokens.Add(new Token(TokenKind.Number, input.Substring(start, i - start), start));
                continue;
            }

            if (char.IsLetter(c) || c == '_')
            {
                int start = i;
                i++;
                while (i < input.Length)
                {
                    char d = input[i];
                    if (char.IsLetterOrDigit(d) || IdentifierExtraChars.Contains(d)) { i++; }
                    else break;
                }
                tokens.Add(new Token(TokenKind.Identifier, input.Substring(start, i - start), start));
                continue;
            }

            throw new FormatException($"Unexpected character '{c}' at position {i}.");
        }
        return tokens;
    }

    private Term ParseExpr(List<Token> tokens, ref int index)
    {
        if (index >= tokens.Count) throw new FormatException("Unexpected end of input.");

        var t = tokens[index];

        if (t.Kind == TokenKind.Number)
        {
            index++;
            if (!float.TryParse(t.Text, NumberStyles.Float | NumberStyles.AllowThousands, CultureInfo.InvariantCulture, out var val))
                throw new FormatException($"Invalid number '{t.Text}' at {t.Position}.");
            return new NumberConstant(val);
        }

        if (t.Kind == TokenKind.String)
        {
            index++;
            return new StringConstant(t.Text);
        }

        if (t.Kind == TokenKind.UnitLiteral)
        {
            index++;
            var parts = t.Text.Trim().Split(new char[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length == 0) throw new FormatException($"Empty unit literal at {t.Position}.");

            if (parts.Length == 1)
            {

                if (!float.TryParse(parts[0], NumberStyles.Float | NumberStyles.AllowThousands, CultureInfo.InvariantCulture, out var val))
                    throw new FormatException($"Invalid number '{parts[0]}' in unit literal at {t.Position}.");
                return new NumberConstant(val);
            }
            else
            {

                if (!float.TryParse(parts[0], NumberStyles.Float | NumberStyles.AllowThousands, CultureInfo.InvariantCulture, out var val))
                    throw new FormatException($"Invalid number '{parts[0]}' in unit literal at {t.Position}.");
                var unit = string.Join(" ", parts.Skip(1));
                return new NumberConstant(val, unit);
            }
        }

        if (t.Kind == TokenKind.Identifier)
        {

            string ident = t.Text;
            int idPos = t.Position;
            index++;

            if (index < tokens.Count && tokens[index].Kind == TokenKind.LeftParenthesis)
            {

                index++;
                var args = new List<Term>();
                while (index < tokens.Count && tokens[index].Kind != TokenKind.RightParenthesis)
                {

                    args.Add(ParseExpr(tokens, ref index));

                }
                if (index >= tokens.Count || tokens[index].Kind != TokenKind.RightParenthesis)
                    throw new FormatException($"Missing closing ')' for call starting at {idPos}.");
                index++;

                var op = InstantiateOperator(ident, idPos);

                if (op is Divide && args.Count < 2)
                    throw new FormatException("divide requires at least 2 operands.");

                return new Operation(op, args.ToArray());
            }
            else
            {

                return new Variable(ident);
            }
        }

        if (t.Kind == TokenKind.LeftParenthesis)
        {

            index++;
            var inner = ParseExpr(tokens, ref index);
            if (index >= tokens.Count || tokens[index].Kind != TokenKind.RightParenthesis)
                throw new FormatException($"Missing ')' for parenthesized expression starting at {t.Position}.");
            index++;
            return inner;
        }

        throw new FormatException($"Unexpected token '{t.Text}' at position {t.Position}.");
    }

    private Operator InstantiateOperator(string keyword, int pos)
    {
        if (_operators.TryGetValue(keyword, out var ctor))
            return ctor();

        throw new KeyNotFoundException($"Unknown operator '{keyword}' at position {pos}.");
    }
}

#endregion ❄️Expressions

#endregion 📦Utilities






#region 🔓Entitying
// Implementations MUST extend Entity for equality, validation, and diff support.

/// Abstract generic base class providing equality, hashing, cloning, and validation.
public abstract class Entity<T> where T : Entity<T>
{
    public override string ToString() => GetType().Name;

    public override bool Equals(object? obj)
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
            .Aggregate(17, (current, value) => current * 31 + value!.GetHashCode());
    }

    public static bool operator ==(Entity<T> left, Entity<T> right)
    {
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null) return false;
        return left.Equals(right);
    }

    public static bool operator !=(Entity<T> left, Entity<T> right) => !(left == right);

    public static T? DeepClone(T entity) => Utility.Deserialize<T>(Utility.Serialize(entity));

    public virtual (bool, List<string>) Validate()
    {
        var result = new EntityValidator<T>().Validate((T)this);
        return (result.IsValid, result.Errors.Select(e => e.ToString()).ToList());
    }

    public static (bool, List<string>) Validate(T entity) => entity.Validate();
}

/// FluentValidation validator base for Entity subclasses.
public class EntityValidator<T> : AbstractValidator<T> where T : Entity<T>
{
    public EntityValidator()
    {
    }
}

#endregion 🔓Entitying






#region ✨SemioValidation
// Callers MUST use ValidationResult to report kit-level validation issues.

public class SemioValidationFix
{
    public string Title { get; set; } = "";
    public object? Diff { get; set; }
}

public class Issue
{
    public string ConstraintId { get; set; } = "";
    public string Message { get; set; } = "";
    public string EntityKind { get; set; } = "";
    public string EntityGuid { get; set; } = "";
    public List<SemioValidationFix> Fixes { get; set; } = new();
}

public class ValidationResult
{
    public List<Issue> Issues { get; set; } = new();

    public bool HasErrors() => Issues.Count > 0;

    public string Serialize()
    {
        var sorted = Issues.OrderBy(i => i.ConstraintId).ThenBy(i => i.EntityGuid).ToList();
        var result = new { issues = sorted.Select(i => new { constraintId = i.ConstraintId, message = i.Message, entityKind = i.EntityKind, entityGuid = i.EntityGuid, fixes = i.Fixes.Select(f => new { title = f.Title, diff = f.Diff }) }) };
        return JsonConvert.SerializeObject(result, Formatting.Indented, new JsonSerializerSettings { ContractResolver = new CamelCasePropertyNamesContractResolver() });
    }

    public static ValidationResult Parse(string json)
    {
        var data = JsonConvert.DeserializeObject<Newtonsoft.Json.Linq.JObject>(json);
        var result = new ValidationResult();
        var problemsToken = data?["problems"] ?? data?["issues"];
        if (problemsToken == null) return result;
        foreach (var issue in problemsToken)
        {
            var fixes = new List<SemioValidationFix>();
            var fixesToken = issue["fixes"];
            if (fixesToken != null)
            {
                foreach (var fix in fixesToken)
                {
                    fixes.Add(new SemioValidationFix { Title = (string?)fix["title"] ?? "", Diff = fix["diff"] });
                }
            }
            result.Issues.Add(new Issue
            {
                ConstraintId = (string?)issue["constraintId"] ?? "",
                Message = (string?)issue["message"] ?? "",
                EntityKind = (string?)issue["entityKind"] ?? "",
                EntityGuid = (string?)issue["entityGuid"] ?? "",
                Fixes = fixes
            });
        }
        return result;
    }

    private static string NormalizeGuids(string json)
    {
        return System.Text.RegularExpressions.Regex.Replace(json, @"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", "<GUID>", System.Text.RegularExpressions.RegexOptions.IgnoreCase);
    }

    public static bool AreEqual(ValidationResult a, ValidationResult b)
    {
        if (a.Issues.Count != b.Issues.Count) return false;
        var sortedA = a.Issues.OrderBy(i => i.ConstraintId).ThenBy(i => i.EntityGuid).ToList();
        var sortedB = b.Issues.OrderBy(i => i.ConstraintId).ThenBy(i => i.EntityGuid).ToList();
        for (var i = 0; i < sortedA.Count; i++)
        {
            var ia = sortedA[i];
            var ib = sortedB[i];
            if (ia.ConstraintId != ib.ConstraintId || ia.Message != ib.Message || ia.EntityKind != ib.EntityKind || ia.EntityGuid != ib.EntityGuid)
                return false;

        }
        return true;
    }
}

public static class SemioValidator
{
    public static ValidationResult ValidateKit(Kit kit)
    {
        var issues = new List<Issue>();
        var seen = new Dictionary<string, string>();

        void CheckGuid(string entityKind, string entityGuid)
        {
            if (seen.ContainsKey(entityGuid))
            {
                issues.Add(new Issue { ConstraintId = "guid-unique", Message = $"Duplicate GUID \"{entityGuid}\". First occurrence kept.", EntityKind = entityKind, EntityGuid = entityGuid });
            }
            else
            {
                seen[entityGuid] = entityKind;
            }
        }

        CheckGuid("Kit", kit.Guid);
        foreach (var t in kit.Types)
        {
            CheckGuid("Type", t.Guid);
            foreach (var connector in t.Connectors) CheckGuid("Connector", connector.Guid);
            foreach (var model in t.Models) CheckGuid("Model", model.Guid);
        }
        foreach (var d in kit.Designs)
        {
            CheckGuid("Design", d.Guid);
            foreach (var p in d.Pieces) CheckGuid("Piece", p.Guid);
            foreach (var c in d.Connections) CheckGuid("Connection", c.Guid);

        }
        foreach (var q in kit.Qualities) CheckGuid("Quality", q.Guid);
        foreach (var i in kit.Ports) CheckGuid("Port", i.Guid);
        foreach (var f in kit.Files) CheckGuid("File", f.Guid);
        foreach (var fo in kit.Folders) CheckGuid("Folder", fo.Guid);

        var typesByParent = kit.Types.GroupBy(t => t.Parent?.Guid);
        foreach (var group in typesByParent)
        {
            var nameGroups = group.GroupBy(t => t.Name ?? "");
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var t in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "type-name-unique", Message = $"Duplicate type name \"{nameGroup.Key}\" among siblings.", EntityKind = "Type", EntityGuid = t.Guid });
                    }
                }
            }
        }

        var designsByParent = kit.Designs.GroupBy(d => d.Parent?.Guid);
        foreach (var group in designsByParent)
        {
            var nameGroups = group.GroupBy(d => d.Name ?? "");
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var d in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "design-name-unique", Message = $"Duplicate design name \"{nameGroup.Key}\" among siblings.", EntityKind = "Design", EntityGuid = d.Guid });
                    }
                }
            }
        }

        foreach (var design in kit.Designs)
        {
            var nameGroups = design.Pieces.Where(p => !string.IsNullOrEmpty(p.Name)).GroupBy(p => p.Name);
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var p in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "piece-name-unique", Message = $"Duplicate piece name \"{nameGroup.Key}\" inside design \"{design.Name}\".", EntityKind = "Piece", EntityGuid = p.Guid });
                    }
                }
            }
        }

        foreach (var t in kit.Types)
        {
            var nameGroups = t.Connectors.Where(p => !string.IsNullOrEmpty(p.Name)).GroupBy(p => p.Name);
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var connector in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "connector-name-unique", Message = $"Duplicate connector name \"{nameGroup.Key}\" inside type \"{t.Name}\".", EntityKind = "Connector", EntityGuid = connector.Guid });
                    }
                }
            }
        }

        foreach (var t in kit.Types)
        {
            var nameGroups = t.Models.Where(m => !string.IsNullOrEmpty(m.Name)).GroupBy(m => m.Name);
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var entity in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "model-name-unique", Message = $"Duplicate model name \"{nameGroup.Key}\" inside type \"{t.Name}\".", EntityKind = "Model", EntityGuid = entity.Guid });
                    }
                }
            }
        }

        var qualityNameGroups = kit.Qualities.GroupBy(q => q.Name ?? "");
        foreach (var nameGroup in qualityNameGroups)
        {
            var list = nameGroup.ToList();
            if (list.Count > 1)
            {
                foreach (var q in list.Skip(1))
                {
                    issues.Add(new Issue { ConstraintId = "quality-name-unique", Message = $"Duplicate quality name \"{nameGroup.Key}\".", EntityKind = "Quality", EntityGuid = q.Guid });
                }
            }
        }

        var portNameGroups = kit.Ports.GroupBy(i => i.Name ?? "");
        foreach (var nameGroup in portNameGroups)
        {
            var list = nameGroup.ToList();
            if (list.Count > 1)
            {
                foreach (var iface in list.Skip(1))
                {
                    issues.Add(new Issue { ConstraintId = "port-name-unique", Message = $"Duplicate port name \"{nameGroup.Key}\".", EntityKind = "Port", EntityGuid = iface.Guid });
                }
            }
        }

        var fileNameGroups = kit.Files.GroupBy(f => f.Name ?? "");
        foreach (var nameGroup in fileNameGroups)
        {
            var list = nameGroup.ToList();
            if (list.Count > 1)
            {
                foreach (var f in list.Skip(1))
                {
                    issues.Add(new Issue { ConstraintId = "file-name-unique", Message = $"Duplicate file name \"{nameGroup.Key}\".", EntityKind = "File", EntityGuid = f.Guid });
                }
            }
        }

        var foldersByParent = kit.Folders.GroupBy(f => f.Parent);
        foreach (var group in foldersByParent)
        {
            var nameGroups = group.GroupBy(f => f.Name ?? "");
            foreach (var nameGroup in nameGroups)
            {
                var list = nameGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var fo in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "folder-name-unique", Message = $"Duplicate folder name \"{nameGroup.Key}\" among siblings.", EntityKind = "Folder", EntityGuid = fo.Guid });
                    }
                }
            }
        }

        foreach (var design in kit.Designs)
        {
            var pathGroups = design.Layers.GroupBy(l => l.Path ?? "");
            foreach (var pathGroup in pathGroups)
            {
                var list = pathGroup.ToList();
                if (list.Count > 1)
                {
                    foreach (var layer in list.Skip(1))
                    {
                        issues.Add(new Issue { ConstraintId = "layer-path-unique", Message = $"Duplicate layer path \"{pathGroup.Key}\" inside design \"{design.Name}\".", EntityKind = "Layer", EntityGuid = layer.Guid });
                    }
                }
            }
        }

        return new ValidationResult { Issues = issues };
    }
}

#endregion ✨SemioValidation






#region 🖥️Weak Entities

#region 📺Coord
// Implementations MUST share X, Y, Z coordinate fields for spatial types.

public class Coord : Entity<Coord>
{
    public double U { get; set; }
    public double V { get; set; }

    public Coord Normalize()
    {
        var length = (double)Math.Sqrt(U * U + V * V);
        return new Coord { U = U / length, V = V / length };
    }
}

#endregion 📺Coord


#region 📦MoveVector
// Implementations MUST carry gap/shift/rise deltas in the piece plane frame for move operations.

public class MoveVector
{
    public double Gap { get; set; }
    public double Shift { get; set; }
    public double Rise { get; set; }
}

#endregion 📦MoveVector






#region ✖️Point
// Implementations MUST represent a 3D point with X, Y, Z coordinates.

public class Point : Entity<Point>
{
    public double X { get; set; } = 0;
    public double Y { get; set; } = 0;
    public double Z { get; set; } = 0;
}

#endregion ✖️Point





#region ↗️Vector
// Implementations MUST represent a 3D vector with X, Y, Z components.

public class Vector : Entity<Vector>
{
    public double X { get; set; } = 1;
    public double Y { get; set; }
    public double Z { get; set; } = 0;

    public static double DotProduct(Vector a, Vector b) => a.X * b.X + a.Y * b.Y + a.Z * b.Z;

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

#endregion ↗️Vector





#region ◻️Plane
// Implementations MUST define a 3D plane by origin and X/Y direction vectors.

public class Plane : Entity<Plane>
{
    public Point Origin { get; set; } = new();
    public Vector XAxis { get; set; } = new() { X = 1 };
    public Vector YAxis { get; set; } = new() { Y = 1 };

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

#endregion ◻️Plane





#endregion 🖥️Weak Entities






public class AttributeDiffUpdate
{
    [JsonProperty("attribute")]
    public AttributeId Attribute { get; set; } = new();
    public AttributeDiff? Diff { get; set; }
}

public class AuthorDiffUpdate
{
    [JsonProperty("author")]
    public AuthorId Author { get; set; } = new();
    public AuthorDiff? Diff { get; set; }
}

public class FileDiffUpdate
{
    [JsonProperty("file")]
    public FileId File { get; set; } = new();
    public FileDiff? Diff { get; set; }
}

public class FolderDiffUpdate
{
    [JsonProperty("folder")]
    public FolderId Folder { get; set; } = new();
    public FolderDiff? Diff { get; set; }
}

public class TagDiffUpdate
{
    [JsonProperty("tag")]
    public TagId Tag { get; set; } = new();
    public TagDiff? Diff { get; set; }
}

public class ConceptDiffUpdate
{
    [JsonProperty("concept")]
    public ConceptId Concept { get; set; } = new();
    public ConceptDiff? Diff { get; set; }
}

public class PortDiffUpdate
{
    [JsonProperty("port")]
    public PortId Port { get; set; } = new();
    public PortDiff? Diff { get; set; }
}

public class PropDiffUpdate
{
    [JsonProperty("prop")]
    public PropId Prop { get; set; } = new();
    public PropDiff? Diff { get; set; }
}

public class ModelDiffUpdate
{
    [JsonProperty("model")]
    public ModelId Model { get; set; } = new();
    public ModelDiff? Diff { get; set; }
}

public class ConnectorDiffUpdate
{
    [JsonProperty("connector")]
    public ConnectorId Connector { get; set; } = new();
    public ConnectorDiff? Diff { get; set; }
}

public class TypeDiffUpdate
{
    [JsonProperty("type")]
    public TypeId Type { get; set; } = new();
    public TypeDiff? Diff { get; set; }
}

public class LayerDiffUpdate
{
    [JsonProperty("layer")]
    public LayerId Layer { get; set; } = new();
    public LayerDiff? Diff { get; set; }
}

public class GroupDiffUpdate
{
    [JsonProperty("group")]
    public GroupId Group { get; set; } = new();
    public GroupDiff? Diff { get; set; }
}

public class PieceDiffUpdate
{
    [JsonProperty("piece")]
    public PieceId Piece { get; set; } = new();
    public PieceDiff? Diff { get; set; }
}

public class ConnectionDiffUpdate
{
    [JsonProperty("connection")]
    public ConnectionId Connection { get; set; } = new();
    public ConnectionDiff? Diff { get; set; }
}

public class StatDiffUpdate
{
    [JsonProperty("stat")]
    public StatId Stat { get; set; } = new();
    public StatDiff? Diff { get; set; }
}

public class QualityDiffUpdate
{
    [JsonProperty("quality")]
    public QualityId Quality { get; set; } = new();
    public QualityDiff? Diff { get; set; }
}

public class BenchmarkDiffUpdate
{
    [JsonProperty("benchmark")]
    public BenchmarkId Benchmark { get; set; } = new();
    public BenchmarkDiff? Diff { get; set; }
}

public class DesignDiffUpdate
{
    [JsonProperty("design")]
    public DesignId Design { get; set; } = new();
    public DesignDiff? Diff { get; set; }
}

public class KitDiffUpdate
{
    [JsonProperty("kit")]
    public KitId Kit { get; set; } = new();
    public KitDiff? Diff { get; set; }
}
/// <summary>💿Change holds the data fields for a Change record.</summary>
public class Change<TEntity, TDiff>
{
    public TDiff Forward { get; set; } = default!;
    public TDiff Backward { get; set; } = default!;
    public string? Author { get; set; }
    public DateTime? Time { get; set; }
    public TEntity? Before { get; set; }
    public TEntity? After { get; set; }
}

public class AttributeChange : Change<Attribute, AttributeDiff> { }
public class AuthorChange : Change<Author, AuthorDiff> { }
public class FileChange : Change<File, FileDiff> { }
public class FolderChange : Change<Folder, FolderDiff> { }
public class BenchmarkChange : Change<Benchmark, BenchmarkDiff> { }
public class QualityChange : Change<Quality, QualityDiff> { }
public class PortChange : Change<Port, PortDiff> { }
public class PropChange : Change<Prop, PropDiff> { }
public class TagChange : Change<Tag, TagDiff> { }
public class ConceptChange : Change<Concept, ConceptDiff> { }
public class ModelChange : Change<Model, ModelDiff> { }
public class ConnectorChange : Change<Connector, ConnectorDiff> { }
public class TypeChange : Change<Type, TypeDiff> { }
public class LayerChange : Change<Layer, LayerDiff> { }
public class PieceChange : Change<Piece, PieceDiff> { }
public class GroupChange : Change<Group, GroupDiff> { }
public class SideChange : Change<Side, SideDiff> { }
public class ConnectionChange : Change<Connection, ConnectionDiff> { }
public class StatChange : Change<Stat, StatDiff> { }
public class DesignChange : Change<Design, DesignDiff> { }
public class KitChange : Change<Kit, KitDiff> { }

#region 💎Attribute
// Implementations MUST provide key-value metadata for annotating entities.

public class AttributeId : Entity<AttributeId>
{
    public string Guid { get; set; } = "";

    public static implicit operator AttributeId(Attribute attribute) => new() { Guid = attribute.Guid };
    public static implicit operator AttributeId(AttributeDiff diff) => new() { Guid = diff.Guid ?? "" };
}

public class AttributeDiff : Entity<AttributeDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _key;
    private string? _value;
    private string? _definition;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Key { get => _key; set { _key = value; _setProperties.Add("Key"); } }
    public string? Value { get => _value; set { _value = value; _setProperties.Add("Value"); } }
    public string? Definition { get => _definition; set { _definition = value; _setProperties.Add("Definition"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeKey() => _setProperties.Contains("Key");
    public bool ShouldSerializeValue() => _setProperties.Contains("Value");
    public bool ShouldSerializeDefinition() => _setProperties.Contains("Definition");

    public static implicit operator AttributeDiff(AttributeId id) => new() { Guid = id.Guid };
    public static implicit operator AttributeDiff(Attribute attribute) => new() { Guid = attribute.Guid, Key = attribute.Key, Value = attribute.Value, Definition = attribute.Definition };

    public AttributeDiff MergeDiff(AttributeDiff other)
    {
        return new AttributeDiff
        {
            Guid = other.Guid ?? Guid,
            Key = string.IsNullOrEmpty(other.Key) ? Key : other.Key,
            Value = string.IsNullOrEmpty(other.Value) ? Value : other.Value,
            Definition = string.IsNullOrEmpty(other.Definition) ? Definition : other.Definition
        };
    }
}

public class AttributesDiff : Entity<AttributesDiff>
{
    public List<AttributeId> Removed { get; set; } = new();
    public List<Attribute> Added { get; set; } = new();
    public List<AttributeDiffUpdate> Updated { get; set; } = new();

    public AttributesDiff MergeDiff(AttributesDiff other)
    {
        return new AttributesDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }

    public static implicit operator AttributesDiff(List<Attribute> attributes) => new() { Updated = attributes.Select(a => new AttributeDiffUpdate { Attribute = a, Diff = (AttributeDiff)a }).ToList() };

    public static List<Attribute> Apply(List<Attribute> original, AttributesDiff diff)
    {
        var result = original.Where(a => !(diff.Removed?.Any(r => r.Guid == a.Guid) ?? false)).ToList();
        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var attr = result.FirstOrDefault(a => a.Guid == update.Attribute.Guid);
                if (attr != null && update.Diff != null)
                    Attribute.ApplyDiff(attr, update.Diff);
            }
        }
        if (diff.Added != null)
            result.AddRange(diff.Added);
        return result;
    }
}

public class Attribute : Entity<Attribute>
{
    public string Guid { get; set; } = "";
    public string Key { get; set; } = "";
    public string? Value { get; set; }
    public string? Definition { get; set; }

    public static implicit operator Attribute(AttributeId id) => new() { Guid = id.Guid };
    public static implicit operator Attribute(AttributeDiff diff) => new() { Guid = diff.Guid ?? "", Key = diff.Key, Value = diff.Value, Definition = diff.Definition };

    public static Attribute ApplyDiff(Attribute attribute, AttributeDiff diff)
    {
        return new Attribute
        {
            Guid = attribute.Guid,
            Key = !string.IsNullOrEmpty(diff.Key) ? diff.Key : attribute.Key,
            Value = !string.IsNullOrEmpty(diff.Value) ? diff.Value : attribute.Value,
            Definition = !string.IsNullOrEmpty(diff.Definition) ? diff.Definition : attribute.Definition
        };
    }
    public static AttributeDiff CreateDiff(Attribute attribute)
    {
        return new AttributeDiff
        {
            Guid = attribute.Guid,
            Key = attribute.Key,
            Value = attribute.Value,
            Definition = attribute.Definition
        };
    }
    public static AttributeDiff InverseDiff(Attribute attribute, AttributeDiff appliedDiff)
    {
        return new AttributeDiff
        {
            Guid = attribute.Guid,
            Key = !string.IsNullOrEmpty(appliedDiff.Key) ? attribute.Key : "",
            Value = !string.IsNullOrEmpty(appliedDiff.Value) ? attribute.Value : "",
            Definition = !string.IsNullOrEmpty(appliedDiff.Definition) ? attribute.Definition : ""
        };
    }

    public string ToIdString() => $"{Key}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Atr({ToHumanIdString()})";
}

#endregion 💎Attribute


#region 📍Location
// Implementations MUST combine a plane with rotation and elevation for placement.

public class LocationId : Entity<LocationId>
{
    public string Guid { get; set; } = "";
    public static implicit operator LocationId(Location location) => new() { Guid = location.Guid };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"LocI({ToHumanIdString()})";
}

public class Location : Entity<Location>
{
    public string Guid { get; set; } = "";
    public double Longitude { get; set; }
    public double Latitude { get; set; }
    public double? Altitude { get; set; }
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Loc({ToHumanIdString()})";
}

#endregion 📍Location






#region ✍️Author
// Implementations MUST provide author identity with name and contact.

public class AuthorId : Entity<AuthorId>
{
    public string Guid { get; set; } = "";
    public static implicit operator AuthorId(Author author) => new() { Guid = author.Guid };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Aut({ToHumanIdString()})";
}

public class ArtifactAuthor : Entity<ArtifactAuthor>
{
    public string AuthorEmail { get; set; } = "";
    public TypeId? TypeId { get; set; }
    public DesignId? DesignId { get; set; }

    public string ToIdString() => $"{AuthorEmail}#{(TypeId?.ToIdString() ?? DesignId?.ToIdString() ?? "")}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"ArtAuth({ToHumanIdString()})";

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (TypeId is null && DesignId is null)
        {
            isValid = false;
            errors.Add("Either TypeId or DesignId must be set.");
        }

        if (TypeId is not null && DesignId is not null)
        {
            isValid = false;
            errors.Add("Either TypeId or DesignId must be set, but not both.");
        }

        return (isValid, errors);
    }
}

public class AuthorDiff : Entity<AuthorDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _email;
    private List<Attribute>? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Email { get => _email; set { _email = value; _setProperties.Add("Email"); } }
    public List<Attribute>? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeEmail() => _setProperties.Contains("Email");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator AuthorDiff(Author author) => new() { Guid = author.Guid, Name = author.Name, Email = author.Email, Attributes = author.Attributes };

    public AuthorDiff MergeDiff(AuthorDiff other)
    {
        return new AuthorDiff
        {
            Guid = other.Guid ?? Guid,
            Name = other.Name ?? Name,
            Email = other.Email ?? Email,
            Attributes = other.Attributes ?? Attributes
        };
    }
}

public class Author : Entity<Author>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Email { get; set; } = "";
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Email}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Aut({ToHumanIdString()})";

    public static implicit operator Author(AuthorId id) => new() { Guid = id.Guid };

    public override (bool, List<string>) Validate()
    {

        var (isValid, errors) = base.Validate();
        if (!Email.Contains("@"))
        {
            isValid = false;
            errors.Add("The email must contain an @.");
        }

        return (isValid, errors);
    }
}

public class AuthorsDiff : Entity<AuthorsDiff>
{
    public List<AuthorId> Removed { get; set; } = new();
    public List<Author> Added { get; set; } = new();
    public List<AuthorDiffUpdate> Updated { get; set; } = new();

    public AuthorsDiff MergeDiff(AuthorsDiff other)
    {
        return new AuthorsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }

    public static implicit operator AuthorsDiff(List<Author> authors) => new() { Updated = authors.Select(a => new AuthorDiffUpdate { Author = a, Diff = (AuthorDiff)a }).ToList() };
}

#endregion ✍️Author






#region 📄File
// Implementations MUST reference a file with URI, MIME type, and optional content.

public class FileId : Entity<FileId>
{
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"FilId({ToHumanIdString()})";

    public static implicit operator FileId(File file) => new() { Guid = file.Guid };
    public static implicit operator FileId(FileDiff diff) => new() { Guid = diff.Guid ?? "" };
}

public class FileDiff : Entity<FileDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _remote;
    private FolderId? _folder;
    private int? _size;
    private string? _hash;
    private string? _blob;
    private DateTime? _createdAt;
    private string? _createdBy;
    private DateTime? _updatedAt;
    private string? _updatedBy;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Remote { get => _remote; set { _remote = value; _setProperties.Add("Remote"); } }
    public FolderId? Folder { get => _folder; set { _folder = value; _setProperties.Add("Folder"); } }
    public int? Size { get => _size; set { _size = value; _setProperties.Add("Size"); } }
    public string? Hash { get => _hash; set { _hash = value; _setProperties.Add("Hash"); } }
    public string? Blob { get => _blob; set { _blob = value; _setProperties.Add("Blob"); } }
    public DateTime? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public string? CreatedBy { get => _createdBy; set { _createdBy = value; _setProperties.Add("CreatedBy"); } }
    public DateTime? UpdatedAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("UpdatedAt"); } }
    public string? UpdatedBy { get => _updatedBy; set { _updatedBy = value; _setProperties.Add("UpdatedBy"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeRemote() => _setProperties.Contains("Remote");
    public bool ShouldSerializeFolder() => _setProperties.Contains("Folder");
    public bool ShouldSerializeSize() => _setProperties.Contains("Size");
    public bool ShouldSerializeHash() => _setProperties.Contains("Hash");
    public bool ShouldSerializeBlob() => _setProperties.Contains("Blob");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeCreatedBy() => _setProperties.Contains("CreatedBy");
    public bool ShouldSerializeUpdatedAt() => _setProperties.Contains("UpdatedAt");
    public bool ShouldSerializeUpdatedBy() => _setProperties.Contains("UpdatedBy");

    public FileDiff MergeDiff(FileDiff other)
    {
        return new FileDiff
        {
            Guid = other.Guid ?? Guid,
            Name = other.Name ?? Name,
            Remote = other.Remote ?? Remote,
            Folder = other.Folder ?? Folder,
            Size = other.Size ?? Size,
            Hash = other.Hash ?? Hash,
            Blob = other.Blob ?? Blob,
            CreatedAt = other.CreatedAt ?? CreatedAt,
            CreatedBy = other.CreatedBy ?? CreatedBy,
            UpdatedAt = other.UpdatedAt ?? UpdatedAt,
            UpdatedBy = other.UpdatedBy ?? UpdatedBy
        };
    }
}

public class FilesDiff : Entity<FilesDiff>
{
    public List<FileId> Removed { get; set; } = new();
    public List<FileDiffUpdate> Updated { get; set; } = new();
    public List<File> Added { get; set; } = new();

    public static implicit operator FilesDiff(List<File> files) => new() { Updated = files.Select(f => new FileDiffUpdate { File = f, Diff = (FileDiff)f }).ToList() };
}

public class File : Entity<File>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Mime { get; set; }
    public string? Remote { get; set; }
    public FolderId? Folder { get; set; }
    public int? Size { get; set; }
    public string? Hash { get; set; }
    public string? Blob { get; set; }
    public DateTime CreatedAt { get; set; }
    public string? CreatedBy { get; set; }
    public DateTime UpdatedAt { get; set; }
    public string? UpdatedBy { get; set; }
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Name}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"Fil({ToHumanIdString()})";

    public static implicit operator File(FileId id) => new() { Guid = id.Guid };
    public static implicit operator File(FileDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Remote = diff.Remote, Folder = diff.Folder, Size = diff.Size, Hash = diff.Hash, Blob = diff.Blob, CreatedAt = diff.CreatedAt ?? default, CreatedBy = diff.CreatedBy, UpdatedAt = diff.UpdatedAt ?? default, UpdatedBy = diff.UpdatedBy };
    public static implicit operator FileDiff(File file) => new() { Guid = file.Guid, Name = file.Name, Remote = file.Remote, Folder = file.Folder, Size = file.Size, Hash = file.Hash, Blob = file.Blob, CreatedAt = file.CreatedAt, CreatedBy = file.CreatedBy, UpdatedAt = file.UpdatedAt, UpdatedBy = file.UpdatedBy };
}
#endregion 📄File






#region 📁Folder
// Implementations MUST reference a folder with name and optional parent.

public class FolderId : Entity<FolderId>
{
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"FolderId({ToHumanIdString()})";

    public static implicit operator FolderId(Folder folder) => new() { Guid = folder.Guid };
    public static implicit operator FolderId(FolderDiff diff) => new() { Guid = diff.Guid ?? "" };
}

public class FolderDiff : Entity<FolderDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private FolderId? _parent;
    private string? _description;
    private List<Attribute>? _attributes;
    private string? _createdAt;
    private string? _createdBy;
    private string? _updatedAt;
    private string? _updatedBy;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public FolderId? Parent { get => _parent; set { _parent = value; _setProperties.Add("Parent"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public List<Attribute>? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }
    public string? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public string? CreatedBy { get => _createdBy; set { _createdBy = value; _setProperties.Add("CreatedBy"); } }
    public string? UpdatedAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("UpdatedAt"); } }
    public string? UpdatedBy { get => _updatedBy; set { _updatedBy = value; _setProperties.Add("UpdatedBy"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeParent() => _setProperties.Contains("Parent");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeCreatedBy() => _setProperties.Contains("CreatedBy");
    public bool ShouldSerializeUpdatedAt() => _setProperties.Contains("UpdatedAt");
    public bool ShouldSerializeUpdatedBy() => _setProperties.Contains("UpdatedBy");

    public FolderDiff MergeDiff(FolderDiff other)
    {
        return new FolderDiff
        {
            Guid = other.Guid ?? Guid,
            Name = other.Name ?? Name,
            Parent = other.Parent ?? Parent,
            Description = other.Description ?? Description,
            Attributes = other.Attributes ?? Attributes,
            CreatedAt = other.CreatedAt ?? CreatedAt,
            CreatedBy = other.CreatedBy ?? CreatedBy,
            UpdatedAt = other.UpdatedAt ?? UpdatedAt,
            UpdatedBy = other.UpdatedBy ?? UpdatedBy
        };
    }
}

public class FoldersDiff : Entity<FoldersDiff>
{
    public List<FolderId> Removed { get; set; } = new();
    public List<FolderDiffUpdate> Updated { get; set; } = new();
    public List<Folder> Added { get; set; } = new();

    public static implicit operator FoldersDiff(List<Folder> folders) => new() { Updated = folders.Select(f => new FolderDiffUpdate { Folder = f, Diff = (FolderDiff)f }).ToList() };
}

public class Folder : Entity<Folder>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public FolderId? Parent { get; set; }
    public string? Description { get; set; }
    public List<Attribute> Attributes { get; set; } = new();
    public string CreatedAt { get; set; } = "";
    public string? CreatedBy { get; set; }
    public string UpdatedAt { get; set; } = "";
    public string? UpdatedBy { get; set; }

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Fol({ToHumanIdString()})";

    public static implicit operator Folder(FolderId id) => new() { Guid = id.Guid };
    public static implicit operator Folder(FolderDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Parent = diff.Parent, Description = diff.Description ?? "", Attributes = diff.Attributes ?? new(), CreatedAt = diff.CreatedAt ?? "", CreatedBy = diff.CreatedBy, UpdatedAt = diff.UpdatedAt ?? "", UpdatedBy = diff.UpdatedBy };
    public static implicit operator FolderDiff(Folder folder) => new() { Guid = folder.Guid, Name = folder.Name, Parent = folder.Parent, Description = folder.Description, Attributes = folder.Attributes, CreatedAt = folder.CreatedAt, CreatedBy = folder.CreatedBy, UpdatedAt = folder.UpdatedAt, UpdatedBy = folder.UpdatedBy };

    public static Folder ApplyDiff(Folder folder, FolderDiff diff)
    {
        return new Folder
        {
            Guid = diff.Guid ?? folder.Guid,
            Name = diff.Name ?? folder.Name,
            Parent = diff.Parent ?? folder.Parent,
            Description = diff.Description ?? folder.Description,
            Attributes = diff.Attributes ?? folder.Attributes,
            CreatedAt = diff.CreatedAt ?? folder.CreatedAt,
            CreatedBy = diff.CreatedBy ?? folder.CreatedBy,
            UpdatedAt = diff.UpdatedAt ?? folder.UpdatedAt,
            UpdatedBy = diff.UpdatedBy ?? folder.UpdatedBy
        };
    }
}

#endregion 📁Folder






#region 📏Benchmark
// Implementations MUST capture benchmark metadata for performance measurement.

public class BenchmarkId : Entity<BenchmarkId>
{
    public string Guid { get; set; } = "";
    public static implicit operator BenchmarkId(Benchmark benchmark) => new() { Guid = benchmark.Guid };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"BmI({ToHumanIdString()})";
}

public class Benchmark : Entity<Benchmark>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Icon { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Bmk({ToHumanIdString()})";
}

public class BenchmarkDiff : Entity<BenchmarkDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _icon;
    private double? _min;
    private bool? _minExcluded;
    private double? _max;
    private bool? _maxExcluded;
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public double? Min { get => _min; set { _min = value; _setProperties.Add("Min"); } }
    public bool? MinExcluded { get => _minExcluded; set { _minExcluded = value; _setProperties.Add("MinExcluded"); } }
    public double? Max { get => _max; set { _max = value; _setProperties.Add("Max"); } }
    public bool? MaxExcluded { get => _maxExcluded; set { _maxExcluded = value; _setProperties.Add("MaxExcluded"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeMin() => _setProperties.Contains("Min");
    public bool ShouldSerializeMinExcluded() => _setProperties.Contains("MinExcluded");
    public bool ShouldSerializeMax() => _setProperties.Contains("Max");
    public bool ShouldSerializeMaxExcluded() => _setProperties.Contains("MaxExcluded");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

#endregion 📏Benchmark






#region 🖨️QualityKind
// Implementations MUST categorize quality metrics by kind.

[Flags]
public enum QualityKind
{
    General = 0,
    Design = 1,
    Type = 2,
    Piece = 4,
    Connection = 8,
    Connector = 16,
}

#endregion 🖨️QualityKind






#region 🔬Quality
// Implementations MUST combine kind, name, value, and unit for quality metrics.

public class QualityId : Entity<QualityId>
{
    public string Guid { get; set; } = "";

    public static implicit operator QualityId(Quality quality) => new() { Guid = quality.Guid };
    public static implicit operator QualityId(QualityDiff diff) => new() { Guid = diff.Guid ?? "" };
}

public class QualityDiff : Entity<QualityDiff>
{
    public string? Guid { get; set; }
    public string Key { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Uri { get; set; }
    public bool? Scalable { get; set; }
    public QualityKind Kind { get; set; } = QualityKind.General;
    public string? SI { get; set; }
    public string? Imperial { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public double? Default { get; set; }
    public string? Formula { get; set; }
    public List<Benchmark> Benchmarks { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator QualityDiff(QualityId id) => new() { Guid = id.Guid };

    public static implicit operator QualityDiff(Quality quality) => new() { Guid = quality.Guid, Key = quality.Key, Name = quality.Name, Description = quality.Description, Uri = quality.Uri, Scalable = quality.Scalable, Kind = quality.Kind, SI = quality.SI, Imperial = quality.Imperial, Min = quality.Min, MinExcluded = quality.MinExcluded, Max = quality.Max, MaxExcluded = quality.MaxExcluded, Default = quality.Default, Formula = quality.Formula, Benchmarks = quality.Benchmarks, Attributes = quality.Attributes };
}

public class Quality : Entity<Quality>
{
    public string Guid { get; set; } = "";
    public string Key { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Uri { get; set; }
    public string? Folder { get; set; }
    public bool? Scalable { get; set; }
    public QualityKind Kind { get; set; } = QualityKind.General;
    public string? SI { get; set; }
    public string? Imperial { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public double? Default { get; set; }
    public string? Formula { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string? Unit { get; set; }
    public List<Benchmark> Benchmarks { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Quality(QualityId id) => new() { Guid = id.Guid };
    public static implicit operator Quality(QualityDiff diff) => new()
    {
        Guid = diff.Guid ?? "",
        Key = diff.Key,
        Name = diff.Name,
        Description = diff.Description,
        Uri = diff.Uri,
        Scalable = diff.Scalable,
        Kind = diff.Kind,
        SI = diff.SI,
        Imperial = diff.Imperial,
        Min = diff.Min,
        MinExcluded = diff.MinExcluded,
        Max = diff.Max,
        MaxExcluded = diff.MaxExcluded,
        Default = diff.Default,
        Formula = diff.Formula,
        Benchmarks = diff.Benchmarks,
        Attributes = diff.Attributes
    };

}

#endregion 🔬Quality






#region ⚓Port
// Implementations MUST define connection ports as typed interfaces on a type.

public class PortId : Entity<PortId>
{
    public string Guid { get; set; } = "";

    public static implicit operator PortId(Port iface) => new() { Guid = iface.Guid };
    public static implicit operator PortId(PortDiff diff) => new() { Guid = diff.Guid };
}

public class PortDiff : Entity<PortDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string _guid = "";
    private string? _name;
    private string? _description;
    private string? _icon;
    private List<PortId>? _compatiblePorts;
    private int? _maxChildren;
    private List<Attribute>? _attributes;

    public string Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public List<PortId>? CompatiblePorts { get => _compatiblePorts; set { _compatiblePorts = value; _setProperties.Add("CompatiblePorts"); } }
    public int? MaxChildren { get => _maxChildren; set { _maxChildren = value; _setProperties.Add("MaxChildren"); } }
    public List<Attribute>? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeCompatiblePorts() => _setProperties.Contains("CompatiblePorts");
    public bool ShouldSerializeMaxChildren() => _setProperties.Contains("MaxChildren");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator PortDiff(PortId id) => new() { Guid = id.Guid };
    public static implicit operator PortDiff(Port iface) => new() { Guid = iface.Guid, Name = iface.Name, Description = iface.Description, Icon = iface.Icon, CompatiblePorts = iface.CompatiblePorts?.Select(i => (PortId)i).ToList(), MaxChildren = iface.MaxChildren, Attributes = iface.Attributes };
}

public class PortsDiff : Entity<PortsDiff>
{
    public List<PortId> Removed { get; set; } = new();
    public List<Port> Added { get; set; } = new();
    public List<PortDiffUpdate> Updated { get; set; } = new();

    public static implicit operator PortsDiff(List<Port> ports) => new() { Updated = ports.Select(i => new PortDiffUpdate { Port = i, Diff = (PortDiff)i }).ToList() };
}

public class Port : Entity<Port>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public List<PortId> CompatiblePorts { get; set; } = new();
    public int? MaxChildren { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Port(PortId id) => new() { Guid = id.Guid };
    public static implicit operator Port(PortDiff diff) => new()
    {
        Guid = diff.Guid,
        Name = diff.Name ?? "",
        Description = diff.Description ?? "",
        Icon = diff.Icon ?? "",
        CompatiblePorts = diff.CompatiblePorts ?? new(),
        MaxChildren = diff.MaxChildren,
        Attributes = diff.Attributes ?? new()
    };

    public static Port ApplyDiff(Port port, PortDiff diff)
    {
        return new Port
        {
            Guid = diff.Guid ?? port.Guid,
            Name = diff.Name ?? port.Name,
            Description = diff.Description ?? port.Description,
            Icon = diff.Icon ?? port.Icon,
            CompatiblePorts = diff.CompatiblePorts ?? port.CompatiblePorts,
            MaxChildren = diff.MaxChildren ?? port.MaxChildren,
            Attributes = diff.Attributes ?? port.Attributes
        };
    }

    public static PortDiff CreateDiff(Port port)
    {
        return new PortDiff
        {
            Guid = port.Guid,
            Name = port.Name,
            Description = port.Description,
            Icon = port.Icon,
            CompatiblePorts = port.CompatiblePorts,
            MaxChildren = port.MaxChildren,
            Attributes = port.Attributes
        };
    }

    public static PortDiff InverseDiff(Port port, PortDiff appliedDiff)
    {
        return new PortDiff
        {
            Guid = !string.IsNullOrEmpty(appliedDiff.Guid) ? port.Guid : "",
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? port.Name : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? port.Description : null,
            Icon = !string.IsNullOrEmpty(appliedDiff.Icon) ? port.Icon : null,
            CompatiblePorts = appliedDiff.CompatiblePorts?.Any() == true ? port.CompatiblePorts : null,
            MaxChildren = appliedDiff.MaxChildren.HasValue ? port.MaxChildren : null,
            Attributes = appliedDiff.Attributes?.Any() == true ? port.Attributes : null
        };
    }
}

#endregion ⚓Port






#region 📊Prop
// Implementations MUST bind a property name to an expression value.

public class PropId : Entity<PropId>
{
    public string Guid { get; set; } = "";
    public static implicit operator PropId(Prop prop) => new() { Guid = prop.Guid };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"PrpI({ToHumanIdString()})";
}

public class Prop : Entity<Prop>
{
    public string Guid { get; set; } = "";
    public QualityId Quality { get; set; } = new();
    public string Value { get; set; } = "";
    public string? Unit { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Prp({ToHumanIdString()})";
}

public class PropDiff : Entity<PropDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private QualityId? _quality;
    private string? _value;
    private string? _unit;
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public QualityId? Quality { get => _quality; set { _quality = value; _setProperties.Add("Quality"); } }
    public string? Value { get => _value; set { _value = value; _setProperties.Add("Value"); } }
    public string? Unit { get => _unit; set { _unit = value; _setProperties.Add("Unit"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeQuality() => _setProperties.Contains("Quality");
    public bool ShouldSerializeValue() => _setProperties.Contains("Value");
    public bool ShouldSerializeUnit() => _setProperties.Contains("Unit");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

#endregion 📊Prop






#region 🏷️Tag
// Implementations MUST provide lightweight labels for categorizing entities.

public class TagId : Entity<TagId>
{
    public string Guid { get; set; } = "";

    public static implicit operator TagId(Tag tag) => new() { Guid = tag.Guid };
}

public class Tag : Entity<Tag>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Tag(TagId id) => new() { Guid = id.Guid };

    public static Tag Find(List<Tag> tags, string guid)
    {
        var tag = tags.FirstOrDefault(t => t.Guid == guid);
        if (tag == null) throw new Exception($"Tag {guid} not found in tags");
        return tag;
    }
}

public class TagDiff : Entity<TagDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _description;
    private string? _icon;
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

public class TagsDiff : Entity<TagsDiff>
{
    public List<TagId> Removed { get; set; } = new();
    public List<Tag> Added { get; set; } = new();
    public List<TagDiffUpdate> Updated { get; set; } = new();
}

#endregion 🏷️Tag






#region 💡Concept
// Implementations MUST link a semantic concept name to description and icon.

public class ConceptId : Entity<ConceptId>
{
    public string Guid { get; set; } = "";

    public static implicit operator ConceptId(Concept concept) => new() { Guid = concept.Guid };
}

public class Concept : Entity<Concept>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Concept(ConceptId id) => new() { Guid = id.Guid };

    public static Concept Find(List<Concept> concepts, string guid)
    {
        var concept = concepts.FirstOrDefault(c => c.Guid == guid);
        if (concept == null) throw new Exception($"Concept {guid} not found in concepts");
        return concept;
    }
}

public class ConceptDiff : Entity<ConceptDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _description;
    private string? _icon;
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

public class ConceptsDiff : Entity<ConceptsDiff>
{
    public List<ConceptId> Removed { get; set; } = new();
    public List<Concept> Added { get; set; } = new();
    public List<ConceptDiffUpdate> Updated { get; set; } = new();

    public ConceptsDiff MergeDiff(ConceptsDiff other)
    {
        return new ConceptsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }
}

#endregion 💡Concept






#region 🗿Model
// Implementations MUST reference a 3D model with URI, MIME type, and local plane.

public class ModelId : Entity<ModelId>
{
    public string Guid { get; set; } = "";
    public static implicit operator ModelId(Model model) => new() { Guid = model.Guid };
    public static implicit operator ModelId(ModelDiff diff) => new() { Guid = diff.Guid ?? "" };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Guid}";
    public override string ToString() => $"Rep({ToHumanIdString()})";
}

public class ModelDiff : Entity<ModelDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private FileId? _file;
    private string? _description;
    private List<TagId> _tags = new();
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public FileId? File { get => _file; set { _file = value; _setProperties.Add("File"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public List<TagId> Tags { get => _tags; set { _tags = value; _setProperties.Add("Tags"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeFile() => _setProperties.Contains("File");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeTags() => _setProperties.Contains("Tags");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator ModelDiff(ModelId id) => new() { Guid = id.Guid };
    public static implicit operator ModelDiff(Model model) => new() { Guid = model.Guid, Name = model.Name, File = model.File, Description = model.Description, Tags = model.Tags, Attributes = model.Attributes };

    public ModelDiff MergeDiff(ModelDiff other)
    {
        return new ModelDiff
        {
            Guid = other.Guid ?? Guid,
            Name = string.IsNullOrEmpty(other.Name) ? Name : other.Name,
            File = other.File ?? File,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Tags = other.Tags.Any() ? other.Tags : Tags,
            Attributes = other.Attributes != null ? other.Attributes : Attributes
        };
    }
}

public class ModelsDiff : Entity<ModelsDiff>
{
    public List<ModelId> Removed { get; set; } = new();
    public List<Model> Added { get; set; } = new();
    public List<ModelDiffUpdate> Updated { get; set; } = new();

    public ModelsDiff MergeDiff(ModelsDiff other)
    {
        return new ModelsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }

    public static implicit operator ModelsDiff(List<Model> models) => new() { Updated = models.Select(r => new ModelDiffUpdate { Model = r, Diff = (ModelDiff)r }).ToList() };
}

public class Model : Entity<Model>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public FileId File { get; set; } = new();
    public string? Description { get; set; }
    public List<TagId> Tags { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Model(ModelId id) => new() { Guid = id.Guid };
    public static implicit operator Model(ModelDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", File = diff.File ?? new(), Description = diff.Description, Tags = diff.Tags, Attributes = diff.Attributes?.Added ?? new() };

    public static Model ApplyDiff(Model model, ModelDiff diff)
    {
        return new Model
        {
            Guid = model.Guid,
            Name = string.IsNullOrEmpty(diff.Name) ? model.Name : diff.Name,
            File = diff.File ?? model.File,
            Description = string.IsNullOrEmpty(diff.Description) ? model.Description : diff.Description,
            Tags = diff.Tags?.Any() == true ? diff.Tags : model.Tags,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(model.Attributes, diff.Attributes) : model.Attributes
        };
    }

    public static ModelDiff CreateDiff(Model model)
    {
        return new ModelDiff
        {
            Guid = model.Guid,
            Name = model.Name,
            File = model.File,
            Description = model.Description,
            Tags = model.Tags,
            Attributes = model.Attributes
        };
    }

    public static ModelDiff InverseDiff(Model model, ModelDiff appliedDiff)
    {
        return new ModelDiff
        {
            Guid = model.Guid,
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? model.Name : null,
            File = appliedDiff.File != null ? model.File : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? model.Description : "",
            Tags = appliedDiff.Tags.Any() ? model.Tags : new List<TagId>(),
            Attributes = appliedDiff.Attributes != null ? model.Attributes : null
        };
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }

    public string ToIdString() => $"{Guid}";

    public string ToHumanIdString() => $"{Name}";

    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();

    public override string ToString() => $"Mod({ToHumanIdString()})";

    public static Model Find(List<Model> models, List<string> tagGuids)
    {
        var model = models.FirstOrDefault(m => tagGuids.All(id => m.Tags.Any(t => t.Guid == id)));
        if (model == null) throw new Exception($"Model with tags {string.Join(", ", tagGuids)} not found in models");
        return model;
    }
}

#endregion 🗿Model






#region 🔌Connector
// Implementations MUST define located interface points on a type.

public class ConnectorId : Entity<ConnectorId>
{
    public string Guid { get; set; } = "";
    public static implicit operator ConnectorId(Connector connector) => new() { Guid = connector.Guid };
    public static implicit operator ConnectorId(ConnectorDiff diff) => new() { Guid = diff.Guid ?? "" };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"Por({ToHumanIdString()})";
}

public class ConnectorDiff : Entity<ConnectorDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _description;
    private PortId? _port;
    private bool? _mandatory;
    private int? _maxChildren;
    private double? _t;
    private Point? _point;
    private Vector? _direction;
    private List<Prop>? _props;
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public PortId? Port { get => _port; set { _port = value; _setProperties.Add("Port"); } }
    public bool? Mandatory { get => _mandatory; set { _mandatory = value; _setProperties.Add("Mandatory"); } }
    public int? MaxChildren { get => _maxChildren; set { _maxChildren = value; _setProperties.Add("MaxChildren"); } }
    public double? T { get => _t; set { _t = value; _setProperties.Add("T"); } }
    public Point? Point { get => _point; set { _point = value; _setProperties.Add("Point"); } }
    public Vector? Direction { get => _direction; set { _direction = value; _setProperties.Add("Direction"); } }
    public List<Prop>? Props { get => _props; set { _props = value; _setProperties.Add("Props"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializePort() => _setProperties.Contains("Port");
    public bool ShouldSerializeMandatory() => _setProperties.Contains("Mandatory");
    public bool ShouldSerializeMaxChildren() => _setProperties.Contains("MaxChildren");
    public bool ShouldSerializeT() => _setProperties.Contains("T");
    public bool ShouldSerializePoint() => _setProperties.Contains("Point");
    public bool ShouldSerializeDirection() => _setProperties.Contains("Direction");
    public bool ShouldSerializeProps() => _setProperties.Contains("Props");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator ConnectorDiff(ConnectorId id) => new() { Guid = id.Guid };
    public static implicit operator ConnectorDiff(Connector connector) => new() { Guid = connector.Guid, Description = connector.Description, Port = connector.Port, Mandatory = connector.Mandatory, MaxChildren = connector.MaxChildren, T = connector.T, Point = connector.Point, Direction = connector.Direction, Props = connector.Props };

    public ConnectorDiff MergeDiff(ConnectorDiff other)
    {
        return new ConnectorDiff
        {
            Guid = other.Guid ?? Guid,
            Description = other.Description ?? Description,
            Port = other.Port ?? Port,
            Mandatory = other.Mandatory ?? Mandatory,
            MaxChildren = other.MaxChildren ?? MaxChildren,
            T = other.T ?? T,
            Point = other.Point ?? Point,
            Direction = other.Direction ?? Direction,
            Props = other.Props ?? Props,
            Attributes = other.Attributes ?? Attributes
        };
    }
}

public class ConnectorsDiff : Entity<ConnectorsDiff>
{
    public List<ConnectorId> Removed { get; set; } = new();
    public List<Connector> Added { get; set; } = new();
    public List<ConnectorDiffUpdate> Updated { get; set; } = new();

    public ConnectorsDiff MergeDiff(ConnectorsDiff other)
    {
        return new ConnectorsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }

    public static implicit operator ConnectorsDiff(List<Connector> connectors) => new() { Updated = connectors.Select(p => new ConnectorDiffUpdate { Connector = p, Diff = (ConnectorDiff)p }).ToList() };
}

public class Connector : Entity<Connector>
{
    public string Guid { get; set; } = "";
    public string? Name { get; set; }
    public string? Description { get; set; }
    public bool? Mandatory { get; set; }
    public int? MaxChildren { get; set; }
    public PortId? Port { get; set; }
    public Point? Point { get; set; } = null;
    public Vector? Direction { get; set; } = null;
    public double T { get; set; } = 0;
    public List<Prop> Props { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Por({ToHumanIdString()})";

    public static implicit operator Connector(ConnectorId id) => new() { Guid = id.Guid };
    public static implicit operator Connector(ConnectorDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name, Description = diff.Description, Port = diff.Port, Mandatory = diff.Mandatory, MaxChildren = diff.MaxChildren, T = diff.T ?? 0, Point = diff.Point, Direction = diff.Direction, Attributes = diff.Attributes?.Added ?? new() };
    public static implicit operator string(Connector connector) => connector.Guid;
    public static implicit operator Connector(string guid) => new() { Guid = guid };

    public static Connector ApplyDiff(Connector connector, ConnectorDiff diff)
    {
        return new Connector
        {
            Guid = diff.Guid ?? connector.Guid,
            Name = diff.Name ?? connector.Name,
            Description = diff.Description ?? connector.Description,
            Port = diff.Port ?? connector.Port,
            Mandatory = diff.Mandatory ?? connector.Mandatory,
            MaxChildren = diff.MaxChildren ?? connector.MaxChildren,
            T = diff.T ?? connector.T,
            Point = diff.Point ?? connector.Point,
            Direction = diff.Direction ?? connector.Direction,
            Props = diff.Props ?? connector.Props,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(connector.Attributes, diff.Attributes) : connector.Attributes
        };
    }

    public static ConnectorDiff CreateDiff(Connector connector)
    {
        return new ConnectorDiff
        {
            Guid = connector.Guid,
            Name = connector.Name,
            Description = connector.Description,
            Port = connector.Port,
            Mandatory = connector.Mandatory,
            MaxChildren = connector.MaxChildren,
            T = connector.T,
            Point = connector.Point,
            Direction = connector.Direction,
            Props = connector.Props,
            Attributes = connector.Attributes
        };
    }

    public static ConnectorDiff InverseDiff(Connector connector, ConnectorDiff appliedDiff)
    {
        return new ConnectorDiff
        {
            Guid = !string.IsNullOrEmpty(appliedDiff.Guid) ? connector.Guid : "",
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? connector.Name : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? connector.Description : "",
            Port = appliedDiff.Port is not null ? connector.Port : null,
            Mandatory = appliedDiff.Mandatory.HasValue ? connector.Mandatory : null,
            MaxChildren = appliedDiff.MaxChildren.HasValue ? connector.MaxChildren : null,
            T = appliedDiff.T.HasValue ? connector.T : null,
            Point = appliedDiff.Point is not null ? connector.Point : null,
            Direction = appliedDiff.Direction is not null ? connector.Direction : null,
            Props = appliedDiff.Props?.Any() == true ? connector.Props : new List<Prop>(),
            Attributes = appliedDiff.Attributes != null ? connector.Attributes : null
        };
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        if (Point is not null)
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
        if (Direction is not null)
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
        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }
        return (isValid, errors);
    }

    public bool IsCompatibleWith(Connector otherConnector)
    {
        if (Port is null || otherConnector.Port is null) return true;
        if (Port.Guid == otherConnector.Port.Guid) return true;
        return false;
    }

    public bool IsCompatibleWith(Connector otherConnector, Kit kit)
    {
        if (Port is null || otherConnector.Port is null) return true;
        if (Port.Guid == otherConnector.Port.Guid) return true;

        var thisPort = kit.Ports?.FirstOrDefault(i => i.Guid == Port.Guid);
        var otherPort = kit.Ports?.FirstOrDefault(i => i.Guid == otherConnector.Port.Guid);

        if (thisPort is null || otherPort is null) return false;

        if (thisPort.CompatiblePorts?.Count == 0 || otherPort.CompatiblePorts?.Count == 0) return true;

        return thisPort.CompatiblePorts?.Any(ci => ci.Guid == otherConnector.Port.Guid) == true ||
               otherPort.CompatiblePorts?.Any(ci => ci.Guid == Port.Guid) == true;
    }

    public static bool IsSameAs(Connector connector, Connector other)
    {
        return Utility.Normalize(connector.Guid) == Utility.Normalize(other.Guid);
    }

    public static string FindAttributeValue(Connector connector, string name, string defaultValue = "")
    {
        var attribute = connector.Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in connector {connector.Guid}");
        return attribute?.Value ?? defaultValue;
    }

    public static Connector SetAttribute(Connector connector, Attribute attribute)
    {
        var attributes = new List<Attribute>(connector.Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Connector
        {
            Guid = connector.Guid,
            Name = connector.Name,
            Description = connector.Description,
            Mandatory = connector.Mandatory,
            Port = connector.Port,
            Point = connector.Point,
            Direction = connector.Direction,
            T = connector.T,
            Props = connector.Props,
            Attributes = attributes
        };
    }

    public static Connector Find(List<Connector> connectors, string connectorGuid)
    {
        var connector = connectors.FirstOrDefault(p => p.Guid == connectorGuid);
        if (connector == null) throw new Exception($"Connector {connectorGuid} not found in connectors");
        return connector;
    }

    public static Connector FindInType(Type type, string connectorGuid)
    {
        return Find(type.Connectors ?? new List<Connector>(), connectorGuid);
    }

    public static Connector? FindForPieceInConnection(Type type, Connection connection, string pieceGuid)
    {
        string? connectorGuid = connection.Connected.Piece.Guid == pieceGuid ? connection.Connected.Connector?.Guid : connection.Connecting.Connector?.Guid;
        if (string.IsNullOrEmpty(connectorGuid)) return null;
        return FindInType(type, connectorGuid);
    }
}

#endregion 🔌Connector






#region 🧱Type
// Implementations MUST compose ports, connectors, and models into a parametric type.

public class TypeId : Entity<TypeId>
{
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Guid}";
    public override string ToString() => $"Typ({ToHumanIdString()})";
    public static implicit operator TypeId(Type type) => new() { Guid = type.Guid };
    public static implicit operator TypeId(TypeDiff diff) => new() { Guid = diff.Guid ?? "" };
}

public class TypeDiff : Entity<TypeDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private TypeId? _parent;
    private bool? _isAbstract;
    private string? _folder;
    private string? _description;
    private string? _icon;
    private string? _image;
    private int? _stock;
    private bool? _virtual;
    private string _uri = "";
    private string _unit = "";
    private Location? _location;
    private ModelsDiff? _models;
    private ConnectorsDiff? _connectors;
    private List<AuthorId>? _authors;
    private AttributesDiff? _attributes;
    private List<ConceptId>? _concepts;
    private DateTime? _createdAt;
    private DateTime? _updatedAt;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public TypeId? Parent { get => _parent; set { _parent = value; _setProperties.Add("Parent"); } }
    public bool? IsAbstract { get => _isAbstract; set { _isAbstract = value; _setProperties.Add("IsAbstract"); } }
    public string? Folder { get => _folder; set { _folder = value; _setProperties.Add("Folder"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public string? Image { get => _image; set { _image = value; _setProperties.Add("Image"); } }
    public int? Stock { get => _stock; set { _stock = value; _setProperties.Add("Stock"); } }
    public bool? Virtual { get => _virtual; set { _virtual = value; _setProperties.Add("Virtual"); } }
    public string Uri { get => _uri; set { _uri = value; _setProperties.Add("Uri"); } }
    public string Unit { get => _unit; set { _unit = value; _setProperties.Add("Unit"); } }
    public Location? Location { get => _location; set { _location = value; _setProperties.Add("Location"); } }
    public ModelsDiff? Models { get => _models; set { _models = value; _setProperties.Add("Models"); } }
    public ConnectorsDiff? Connectors { get => _connectors; set { _connectors = value; _setProperties.Add("Connectors"); } }
    public List<AuthorId>? Authors { get => _authors; set { _authors = value; _setProperties.Add("Authors"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }
    public List<ConceptId>? Concepts { get => _concepts; set { _concepts = value; _setProperties.Add("Concepts"); } }
    public DateTime? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public DateTime? UpdatedAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("UpdatedAt"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeParent() => _setProperties.Contains("Parent");
    public bool ShouldSerializeIsAbstract() => _setProperties.Contains("IsAbstract");
    public bool ShouldSerializeFolder() => _setProperties.Contains("Folder");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeImage() => _setProperties.Contains("Image");
    public bool ShouldSerializeStock() => _setProperties.Contains("Stock");
    public bool ShouldSerializeVirtual() => _setProperties.Contains("Virtual");
    public bool ShouldSerializeUri() => _setProperties.Contains("Uri");
    public bool ShouldSerializeUnit() => _setProperties.Contains("Unit");
    public bool ShouldSerializeLocation() => _setProperties.Contains("Location");
    public bool ShouldSerializeModels() => _setProperties.Contains("Models");
    public bool ShouldSerializeConnectors() => _setProperties.Contains("Connectors");
    public bool ShouldSerializeAuthors() => _setProperties.Contains("Authors");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
    public bool ShouldSerializeConcepts() => _setProperties.Contains("Concepts");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeUpdatedAt() => _setProperties.Contains("UpdatedAt");

    public TypeDiff MergeDiff(TypeDiff other)
    {
        return new TypeDiff
        {
            Name = string.IsNullOrEmpty(other.Name) ? Name : other.Name,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Icon = string.IsNullOrEmpty(other.Icon) ? Icon : other.Icon,
            Image = string.IsNullOrEmpty(other.Image) ? Image : other.Image,
            Stock = other.Stock ?? Stock,
            Virtual = other.Virtual ?? Virtual,
            Uri = string.IsNullOrEmpty(other.Uri) ? Uri : other.Uri,
            Unit = string.IsNullOrEmpty(other.Unit) ? Unit : other.Unit,
            Location = other.Location ?? Location,
            Models = other.Models is not null ? (other.Models.MergeDiff(Models ?? new ModelsDiff())) : Models,
            Connectors = other.Connectors is not null ? (other.Connectors.MergeDiff(Connectors ?? new ConnectorsDiff())) : Connectors,
            Authors = other.Authors is not null && other.Authors.Any() ? other.Authors : Authors,
            Attributes = other.Attributes is not null ? other.Attributes.MergeDiff(Attributes ?? new AttributesDiff()) : Attributes,
            Concepts = other.Concepts is not null && other.Concepts.Any() ? other.Concepts : Concepts
        };
    }

    public static implicit operator TypeDiff(TypeId id) => new() { Guid = id.Guid };
    public static implicit operator TypeDiff(Type type) => new() { Name = type.Name, Description = type.Description, Icon = type.Icon, Image = type.Image, Stock = type.Stock, Virtual = type.Virtual, Uri = type.Uri, Unit = type.Unit, Location = type.Location, Models = new ModelsDiff { Added = new List<Model>(), Removed = new List<ModelId>(), Updated = type.Models.Select(m => new ModelDiffUpdate { Model = m, Diff = Model.CreateDiff(m) }).ToList() }, Connectors = new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Updated = type.Connectors.Select(p => new ConnectorDiffUpdate { Connector = p, Diff = Connector.CreateDiff(p) }).ToList() }, Authors = type.Authors, Concepts = type.Concepts };
}

public class TypesDiff : Entity<TypesDiff>
{
    public List<TypeId> Removed { get; set; } = new();
    public List<Type> Added { get; set; } = new();
    public List<TypeDiffUpdate> Updated { get; set; } = new();

    public static implicit operator TypesDiff(List<Type> types) => new() { Updated = types.Select(t => new TypeDiffUpdate { Type = t, Diff = (TypeDiff)t }).ToList() };
}

public class Type : Entity<Type>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public TypeId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public int? Stock { get; set; }
    public bool? Virtual { get; set; }
    public string? Uri { get; set; }
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public List<Model> Models { get; set; } = new();
    public List<Connector> Connectors { get; set; } = new();
    public List<Prop> Props { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public List<ConceptId> Concepts { get; set; } = new();
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }

    public string ToIdString() => $"{Guid}";

    public string ToHumanIdString() => $"{Name}";

    public override string ToString() => $"Typ({ToHumanIdString()})";

    public static implicit operator Type(TypeId id) => new() { Guid = id.Guid, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };
    public static implicit operator Type(TypeDiff diff) => new()
    {
        Guid = diff.Guid ?? "",
        Name = diff.Name ?? "",
        Parent = diff.Parent,
        IsAbstract = diff.IsAbstract,
        Folder = diff.Folder,
        Description = diff.Description,
        Icon = diff.Icon,
        Image = diff.Image,
        Stock = diff.Stock,
        Virtual = diff.Virtual,
        Uri = diff.Uri,
        Unit = diff.Unit,
        Location = diff.Location,
        Models = diff.Models?.Added ?? new(),
        Connectors = diff.Connectors?.Added ?? new(),
        Authors = diff.Authors ?? new(),
        Attributes = diff.Attributes?.Added ?? new(),
        Concepts = diff.Concepts ?? new(),
        CreatedAt = diff.CreatedAt ?? DateTime.UtcNow,
        UpdatedAt = diff.UpdatedAt ?? DateTime.UtcNow
    };
    public static implicit operator string(Type type) => type.Name;
    public static implicit operator Type(string name) => new() { Name = name, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };

    public static Type ApplyDiff(Type type, TypeDiff diff)
    {
        var models = type.Models;
        var connectors = type.Connectors;

        if (diff.Models is not null)
            models = ApplyModelsDiff(type.Models, diff.Models);
        if (diff.Connectors is not null)
            connectors = ApplyConnectorsDiff(type.Connectors, diff.Connectors);

        return new Type
        {
            Guid = type.Guid,
            Name = string.IsNullOrEmpty(diff.Name) ? type.Name : diff.Name,
            Description = diff.Description ?? type.Description,
            Icon = diff.Icon ?? type.Icon,
            Image = diff.Image ?? type.Image,
            Stock = diff.Stock ?? type.Stock,
            Virtual = diff.Virtual ?? type.Virtual,
            Uri = diff.Uri ?? type.Uri,
            Unit = diff.Unit ?? type.Unit,
            Location = diff.Location ?? type.Location,
            Models = models,
            Connectors = connectors,
            Authors = diff.Authors is not null && diff.Authors.Any() ? diff.Authors : type.Authors,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(type.Attributes, diff.Attributes) : type.Attributes,
            Concepts = diff.Concepts is not null && diff.Concepts.Any() ? diff.Concepts : type.Concepts,
            Props = type.Props,
            CreatedAt = type.CreatedAt,
            UpdatedAt = DateTime.UtcNow
        };
    }

    private static List<Model> ApplyModelsDiff(List<Model> original, ModelsDiff diff)
    {
        var result = original.Where(m => !diff.Removed.Any(r => r.Guid == m.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(m => m.Guid == updated.Model.Guid);
            if (index >= 0 && updated.Diff != null)
                result[index] = Model.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static List<Connector> ApplyConnectorsDiff(List<Connector> original, ConnectorsDiff diff)
    {
        var result = original.Where(p => !diff.Removed.Any(r => r.Guid == p.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(p => p.Guid == updated.Connector.Guid);
            if (index >= 0 && updated.Diff != null)
                result[index] = Connector.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    public static TypeDiff CreateDiff(Type type)
    {
        return new TypeDiff
        {
            Guid = type.Guid,
            Name = type.Name,
            Description = type.Description,
            Icon = type.Icon,
            Image = type.Image,
            Stock = type.Stock,
            Virtual = type.Virtual,
            Uri = type.Uri,
            Unit = type.Unit,
            Location = type.Location,
            Models = new ModelsDiff { Added = new List<Model>(), Removed = new List<ModelId>(), Updated = type.Models.Select(m => new ModelDiffUpdate { Model = m, Diff = Model.CreateDiff(m) }).ToList() },
            Connectors = new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Updated = type.Connectors.Select(p => new ConnectorDiffUpdate { Connector = p, Diff = Connector.CreateDiff(p) }).ToList() },
            Authors = type.Authors,
            Attributes = type.Attributes,
            Concepts = type.Concepts
        };
    }

    public static TypeDiff InverseDiff(Type type, TypeDiff appliedDiff)
    {
        return new TypeDiff
        {
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? type.Name : "",
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? type.Description : "",
            Icon = !string.IsNullOrEmpty(appliedDiff.Icon) ? type.Icon : "",
            Image = !string.IsNullOrEmpty(appliedDiff.Image) ? type.Image : "",
            Stock = appliedDiff.Stock.HasValue ? type.Stock : null,
            Virtual = appliedDiff.Virtual.HasValue ? type.Virtual : null,
            Uri = !string.IsNullOrEmpty(appliedDiff.Uri) ? type.Uri : "",
            Unit = !string.IsNullOrEmpty(appliedDiff.Unit) ? type.Unit : "",
            Location = appliedDiff.Location is not null ? type.Location : null,
            Models = appliedDiff.Models is not null ? new ModelsDiff { Added = new List<Model>(), Removed = new List<ModelId>(), Updated = type.Models.Select(m => new ModelDiffUpdate { Model = m, Diff = Model.CreateDiff(m) }).ToList() } : null,
            Connectors = appliedDiff.Connectors is not null ? new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Updated = type.Connectors.Select(p => new ConnectorDiffUpdate { Connector = p, Diff = Connector.CreateDiff(p) }).ToList() } : null,
            Authors = appliedDiff.Authors is not null && appliedDiff.Authors.Any() ? type.Authors : null,
            Attributes = appliedDiff.Attributes is not null ? type.Attributes : null
        };
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();
        foreach (var connector in Connectors)
        {
            var (isValidConnector, errorsPort) = connector.Validate();
            isValid = isValid && isValidConnector;
            errors.AddRange(errorsPort.Select(e => $"A connector({connector.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var model in Models)
        {
            var (isValidModel, errorsModel) = model.Validate();
            isValid = isValid && isValidModel;
            errors.AddRange(errorsModel.Select(e =>
                $"A model({model.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var author in Authors)
        {
            var (isValidAuthor, errorsAuthor) = author.Validate();
            isValid = isValid && isValidAuthor;
            errors.AddRange(errorsAuthor.Select(e => $"An author({author.ToHumanIdString()}) is invalid: " + e));
        }

        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }

    public static Dictionary<string, Type> EnumerableToDict(IEnumerable<Type> types)
    {
        var typesDict = new Dictionary<string, Type>();
        foreach (var type in types)
        {
            typesDict[type.Name] = type;
        }

        return typesDict;
    }

    public static bool IsSameAs(Type type, Type other)
    {
        return type.Name == other.Name;
    }

    public Connector FindConnector(string connectorId)
    {
        var connector = Connectors?.FirstOrDefault(p => Utility.Normalize(p.Guid) == Utility.Normalize(connectorId));
        if (connector is null) throw new InvalidOperationException($"Connector {connectorId} not found in type {Name}");
        return connector;
    }

    public Model FindModel(List<string> tags)
    {
        if (Models == null || Models.Count == 0)
            throw new ArgumentException($"No models available in type {Name}");

        var indices = Models.Select(r => Utility.Jaccard(r.Tags.Select(t => t.Guid), tags)).ToList();
        var maxIndex = indices.Max();
        var maxIndexIndex = indices.IndexOf(maxIndex);
        return Models[maxIndexIndex];
    }

    public string FindAttributeValue(string name, string defaultValue = "")
    {
        var attribute = Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in type {Name}");
        return attribute?.Value ?? defaultValue;
    }

    public Type SetAttribute(Attribute attribute)
    {
        var attributes = new List<Attribute>(Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Type
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Stock = Stock,
            Virtual = Virtual,
            Location = Location,
            Unit = Unit,
            Models = Models,
            Connectors = Connectors,
            Props = Props,
            Authors = Authors,
            Attributes = attributes
        };
    }
}

#endregion 🧱Type






#region 🎨Layer
// Implementations MUST organize pieces into named layers within a design.

public class LayerId : Entity<LayerId>
{
    public string Guid { get; set; } = "";
    public static implicit operator LayerId(Layer layer) => new() { Guid = layer.Guid };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"LyrI({ToHumanIdString()})";
}

public class Layer : Entity<Layer>
{
    public string Guid { get; set; } = "";
    public string Path { get; set; } = "";
    public bool? IsHidden { get; set; }
    public bool? IsLocked { get; set; }
    public string? Color { get; set; }
    public string? Description { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Path}";
    public override string ToString() => $"Lyr({ToHumanIdString()})";
}

public class LayerDiff : Entity<LayerDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _path;
    private bool? _isHidden;
    private bool? _isLocked;
    private string? _color;
    private string? _description;
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Path { get => _path; set { _path = value; _setProperties.Add("Path"); } }
    public bool? IsHidden { get => _isHidden; set { _isHidden = value; _setProperties.Add("IsHidden"); } }
    public bool? IsLocked { get => _isLocked; set { _isLocked = value; _setProperties.Add("IsLocked"); } }
    public string? Color { get => _color; set { _color = value; _setProperties.Add("Color"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializePath() => _setProperties.Contains("Path");
    public bool ShouldSerializeIsHidden() => _setProperties.Contains("IsHidden");
    public bool ShouldSerializeIsLocked() => _setProperties.Contains("IsLocked");
    public bool ShouldSerializeColor() => _setProperties.Contains("Color");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

#endregion 🎨Layer






#region 🧩Piece
// Implementations MUST place an instantiated type within a design hierarchy.

public class PieceId : Entity<PieceId>
{
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    public static implicit operator PieceId(PieceDiff diff) => new() { Guid = diff.Guid ?? "" };
    public static implicit operator PieceId(Piece piece) => new() { Guid = piece.Guid };
}

public class PiecesDiff : Entity<PiecesDiff>
{
    public List<PieceId> Removed { get; set; } = new();
    public List<PieceDiffUpdate> Updated { get; set; } = new();
    public List<Piece> Added { get; set; } = new();

    public PiecesDiff MergeDiff(PiecesDiff other)
    {
        return new PiecesDiff
        {
            Removed = other.Removed.Concat(Removed).Distinct().ToList(),
            Updated = other.Updated.Concat(Updated).GroupBy(m => m.Piece.Guid).Select(g => g.Last()).ToList(),
            Added = other.Added.Concat(Added).GroupBy(a => a.Guid).Select(g => g.Last()).ToList()
        };
    }

    public static implicit operator PiecesDiff(List<Piece> pieces) => new() { Updated = pieces.Select(p => new PieceDiffUpdate { Piece = p, Diff = Piece.CreateDiff(p) }).ToList() };
}

public class PieceDiff : Entity<PieceDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _description;
    private TypeId? _type;
    private DesignId? _design;
    private Plane? _plane;
    private Coord? _center;
    private double? _scale;
    private Plane? _mirrorPlane;
    private bool? _isHidden;
    private bool? _isLocked;
    private string? _color;
    private List<Prop>? _props;
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public TypeId? Type { get => _type; set { _type = value; _setProperties.Add("Type"); } }
    public DesignId? Design { get => _design; set { _design = value; _setProperties.Add("Design"); } }
    public Plane? Plane { get => _plane; set { _plane = value; _setProperties.Add("Plane"); } }
    public Coord? Center { get => _center; set { _center = value; _setProperties.Add("Center"); } }
    public double? Scale { get => _scale; set { _scale = value; _setProperties.Add("Scale"); } }
    public Plane? MirrorPlane { get => _mirrorPlane; set { _mirrorPlane = value; _setProperties.Add("MirrorPlane"); } }
    public bool? IsHidden { get => _isHidden; set { _isHidden = value; _setProperties.Add("IsHidden"); } }
    public bool? IsLocked { get => _isLocked; set { _isLocked = value; _setProperties.Add("IsLocked"); } }
    public string? Color { get => _color; set { _color = value; _setProperties.Add("Color"); } }
    public List<Prop>? Props { get => _props; set { _props = value; _setProperties.Add("Props"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeType() => _setProperties.Contains("Type");
    public bool ShouldSerializeDesign() => _setProperties.Contains("Design");
    public bool ShouldSerializePlane() => _setProperties.Contains("Plane");
    public bool ShouldSerializeCenter() => _setProperties.Contains("Center");
    public bool ShouldSerializeScale() => _setProperties.Contains("Scale");
    public bool ShouldSerializeMirrorPlane() => _setProperties.Contains("MirrorPlane");
    public bool ShouldSerializeIsHidden() => _setProperties.Contains("IsHidden");
    public bool ShouldSerializeIsLocked() => _setProperties.Contains("IsLocked");
    public bool ShouldSerializeColor() => _setProperties.Contains("Color");
    public bool ShouldSerializeProps() => _setProperties.Contains("Props");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator PieceDiff(PieceId id) => new() { Guid = id.Guid };
    public static implicit operator PieceDiff(Piece piece) => new() { Guid = piece.Guid, Name = piece.Name, Description = piece.Description, Type = piece.Type, Design = piece.Design, Plane = piece.Plane, Center = piece.Center, Scale = piece.Scale, MirrorPlane = piece.MirrorPlane, IsHidden = piece.IsHidden, IsLocked = piece.IsLocked, Color = piece.Color, Props = piece.Props, Attributes = piece.Attributes };
}

public class Piece : Entity<Piece>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public TypeId? Type { get; set; }
    public DesignId? Design { get; set; }
    public Plane? Plane { get; set; }
    public Coord? Center { get; set; }
    public double? Scale { get; set; }
    public Plane? MirrorPlane { get; set; }
    public bool? IsHidden { get; set; }
    public bool? IsLocked { get; set; }
    public string? Color { get; set; }
    public List<Prop>? Props { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Guid}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    public static implicit operator Piece(PieceId id) => new() { Guid = id.Guid };
    public static implicit operator Piece(PieceDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Description = diff.Description ?? "", Type = diff.Type, Design = diff.Design, Plane = diff.Plane, Center = diff.Center, Scale = diff.Scale, MirrorPlane = diff.MirrorPlane, IsHidden = diff.IsHidden, IsLocked = diff.IsLocked, Color = diff.Color, Props = diff.Props, Attributes = diff.Attributes?.Added ?? new() };

    public static Piece ApplyDiff(Piece piece, PieceDiff diff)
    {
        return new Piece
        {
            Guid = diff.Guid ?? piece.Guid,
            Name = diff.Name ?? piece.Name,
            Description = diff.Description ?? piece.Description,
            Type = diff.Type ?? piece.Type,
            Design = diff.Design ?? piece.Design,
            Plane = diff.Plane ?? piece.Plane,
            Center = diff.Center ?? piece.Center,
            Scale = diff.Scale ?? piece.Scale,
            MirrorPlane = diff.MirrorPlane ?? piece.MirrorPlane,
            IsHidden = diff.IsHidden ?? piece.IsHidden,
            IsLocked = diff.IsLocked ?? piece.IsLocked,
            Color = diff.Color ?? piece.Color,
            Props = diff.Props ?? piece.Props,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(piece.Attributes, diff.Attributes) : piece.Attributes
        };
    }

    public static PieceDiff CreateDiff(Piece piece)
    {
        return new PieceDiff
        {
            Guid = piece.Guid,
            Name = piece.Name,
            Description = piece.Description,
            Type = piece.Type,
            Design = piece.Design,
            Plane = piece.Plane,
            Center = piece.Center,
            Scale = piece.Scale,
            MirrorPlane = piece.MirrorPlane,
            IsHidden = piece.IsHidden,
            IsLocked = piece.IsLocked,
            Color = piece.Color,
            Props = piece.Props,
            Attributes = piece.Attributes
        };
    }

    public static Piece Find(List<Piece> pieces, string pieceGuid)
    {
        var piece = pieces.FirstOrDefault(p => p.Guid == pieceGuid);
        if (piece == null) throw new Exception($"Piece {pieceGuid} not found in pieces");
        return piece;
    }

    public static Piece FindInDesign(Design design, string pieceGuid)
    {
        return Find(design.Pieces ?? new List<Piece>(), pieceGuid);
    }
}

#endregion 🧩Piece






#region 👥Group
// Implementations MUST group pieces by name within a design.

public class GroupId : Entity<GroupId>
{
    public string Guid { get; set; } = "";
    public static implicit operator GroupId(Group group) => new() { Guid = group.Guid };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"GrpI({ToHumanIdString()})";
}

public class Group : Entity<Group>
{
    public string Guid { get; set; } = "";
    public string? Name { get; set; }
    public string? Description { get; set; }
    public List<PieceId> Pieces { get; set; } = new();
    public string? Color { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Name ?? Guid}";
    public override string ToString() => $"Grp({ToHumanIdString()})";
}

public class GroupDiff : Entity<GroupDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _description;
    private List<PieceId>? _pieces;
    private string? _color;
    private AttributesDiff? _attributes;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public List<PieceId>? Pieces { get => _pieces; set { _pieces = value; _setProperties.Add("Pieces"); } }
    public string? Color { get => _color; set { _color = value; _setProperties.Add("Color"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializePieces() => _setProperties.Contains("Pieces");
    public bool ShouldSerializeColor() => _setProperties.Contains("Color");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
}

#endregion 👥Group





#region ↔️Side
// Implementations MUST reference a piece and connector as a connection endpoint.

public class SideDiff : Entity<SideDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private PieceId? _piece;
    private PieceId? _designPiece = null;
    private ConnectorId? _connector;
    private string? _description;

    public PieceId? Piece { get => _piece; set { _piece = value; _setProperties.Add("Piece"); } }
    public PieceId? DesignPiece { get => _designPiece; set { _designPiece = value; _setProperties.Add("DesignPiece"); } }
    public ConnectorId? Connector { get => _connector; set { _connector = value; _setProperties.Add("Connector"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }

    public bool ShouldSerializePiece() => _setProperties.Contains("Piece");
    public bool ShouldSerializeDesignPiece() => _setProperties.Contains("DesignPiece");
    public bool ShouldSerializeConnector() => _setProperties.Contains("Connector");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");

    public static implicit operator SideDiff(Side side) => new() { Piece = side.Piece, DesignPiece = side.DesignPiece, Connector = side.Connector };

    public SideDiff MergeDiff(SideDiff other)
    {
        return new SideDiff
        {
            Piece = other.Piece ?? Piece,
            DesignPiece = other.DesignPiece ?? DesignPiece,
            Connector = other.Connector ?? Connector,
            Description = other.Description ?? Description
        };
    }
}

public class Side : Entity<Side>
{
    public PieceId Piece { get; set; } = new();
    public PieceId? DesignPiece { get; set; } = null;
    public ConnectorId Connector { get; set; } = new();

    public static implicit operator Side(SideDiff diff) => new() { Piece = diff.Piece ?? new(), DesignPiece = diff.DesignPiece, Connector = diff.Connector ?? new() };

    public static Side ApplyDiff(Side side, SideDiff diff)
    {
        return new Side
        {
            Piece = diff.Piece ?? side.Piece,
            DesignPiece = diff.DesignPiece ?? side.DesignPiece,
            Connector = diff.Connector ?? side.Connector
        };
    }

    public static SideDiff CreateDiff(Side side)
    {
        return new SideDiff
        {
            Piece = side.Piece,
            DesignPiece = side.DesignPiece,
            Connector = side.Connector
        };
    }

    public static SideDiff InverseDiff(Side side, SideDiff appliedDiff)
    {
        return new SideDiff
        {
            Piece = appliedDiff.Piece is not null ? side.Piece : null,
            DesignPiece = appliedDiff.DesignPiece is not null ? side.DesignPiece : null,
            Connector = appliedDiff.Connector is not null ? side.Connector : null
        };
    }

    public override bool Equals(object? obj)
    {
        if (obj is not Side other) return false;
        return Piece.Guid == other.Piece.Guid && DesignPiece?.Guid == other.DesignPiece?.Guid && Connector.Guid == other.Connector.Guid;
    }

    public override int GetHashCode()
    {
        unchecked
        {
            var hash = 17;
            hash = hash * 31 + (Piece.Guid?.GetHashCode() ?? 0);
            hash = hash * 31 + (DesignPiece?.Guid?.GetHashCode() ?? 0);
            hash = hash * 31 + (Connector.Guid?.GetHashCode() ?? 0);
            return hash;
        }
    }

    public override string ToString() => $"Sde({Piece.Guid}" + (Connector.Guid != "" ? ":" + Connector.Guid : "") + ")";
}

#endregion ↔️Side






#region 🔗Connection
// Implementations MUST link two sides to connect pieces in a design.

public class ConnectionId : Entity<ConnectionId>
{
    public string Guid { get; set; } = "";
    public Side Connected { get; set; } = new();
    public Side Connecting { get; set; } = new();

    public string ToIdString() => $"{Connected.Piece.Guid + (Connected.Connector.Guid != "" ? ":" + Connected.Connector.Guid : "")}--{(Connecting.Connector.Guid != "" ? Connecting.Connector.Guid + ":" : "") + Connecting.Piece.Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"ConId({ToHumanIdString()})";

    public static implicit operator ConnectionId(Connection connection) => new() { Connected = connection.Connected, Connecting = connection.Connecting };
    public static implicit operator ConnectionId(ConnectionDiff diff) => new() { Connected = diff.Connected ?? new(), Connecting = diff.Connecting ?? new() };
}

public class ConnectionDiff : Entity<ConnectionDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private SideDiff? _connected;
    private SideDiff? _connecting;
    private string? _description;
    private double? _gap;
    private double? _shift;
    private double? _rise;
    private double? _rotation;
    private double? _turn;
    private double? _tilt;
    private double? _u;
    private double? _v;
    private AttributesDiff? _attributes;

    public SideDiff? Connected { get => _connected; set { _connected = value; _setProperties.Add("Connected"); } }
    public SideDiff? Connecting { get => _connecting; set { _connecting = value; _setProperties.Add("Connecting"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public double? Gap { get => _gap; set { _gap = value; _setProperties.Add("Gap"); } }
    public double? Shift { get => _shift; set { _shift = value; _setProperties.Add("Shift"); } }
    public double? Rise { get => _rise; set { _rise = value; _setProperties.Add("Rise"); } }
    public double? Rotation { get => _rotation; set { _rotation = value; _setProperties.Add("Rotation"); } }
    public double? Turn { get => _turn; set { _turn = value; _setProperties.Add("Turn"); } }
    public double? Tilt { get => _tilt; set { _tilt = value; _setProperties.Add("Tilt"); } }
    public double? U { get => _u; set { _u = value; _setProperties.Add("U"); } }
    public double? V { get => _v; set { _v = value; _setProperties.Add("V"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }

    public bool ShouldSerializeConnected() => _setProperties.Contains("Connected");
    public bool ShouldSerializeConnecting() => _setProperties.Contains("Connecting");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeGap() => _setProperties.Contains("Gap");
    public bool ShouldSerializeShift() => _setProperties.Contains("Shift");
    public bool ShouldSerializeRise() => _setProperties.Contains("Rise");
    public bool ShouldSerializeRotation() => _setProperties.Contains("Rotation");
    public bool ShouldSerializeTurn() => _setProperties.Contains("Turn");
    public bool ShouldSerializeTilt() => _setProperties.Contains("Tilt");
    public bool ShouldSerializeU() => _setProperties.Contains("U");
    public bool ShouldSerializeV() => _setProperties.Contains("V");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");

    public static implicit operator ConnectionDiff(ConnectionId id) => new() { Connected = new SideDiff { Piece = id.Connected.Piece, DesignPiece = id.Connected.DesignPiece, Connector = id.Connected.Connector }, Connecting = new SideDiff { Piece = id.Connecting.Piece, DesignPiece = id.Connecting.DesignPiece, Connector = id.Connecting.Connector } };
    public static implicit operator ConnectionDiff(Connection connection) => new() { Connected = Side.CreateDiff(connection.Connected), Connecting = Side.CreateDiff(connection.Connecting), Description = connection.Description, Gap = connection.Gap, Shift = connection.Shift, Rise = connection.Rise, Rotation = connection.Rotation, Turn = connection.Turn, Tilt = connection.Tilt, U = connection.U, V = connection.V, Attributes = connection.Attributes };

    public ConnectionDiff MergeDiff(ConnectionDiff other)
    {
        return new ConnectionDiff
        {
            Connected = other.Connected is not null ? (other.Connected.MergeDiff(Connected ?? new SideDiff())) : Connected,
            Connecting = other.Connecting is not null ? (other.Connecting.MergeDiff(Connecting ?? new SideDiff())) : Connecting,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Gap = other.Gap ?? Gap,
            Shift = other.Shift ?? Shift,
            Rise = other.Rise ?? Rise,
            Rotation = other.Rotation ?? Rotation,
            Turn = other.Turn ?? Turn,
            Tilt = other.Tilt ?? Tilt,
            U = other.U ?? U,
            V = other.V ?? V,
            Attributes = other.Attributes ?? Attributes
        };
    }
}

public class ConnectionsDiff : Entity<ConnectionsDiff>
{
    public List<ConnectionId> Removed { get; set; } = new();
    public List<ConnectionDiffUpdate> Updated { get; set; } = new();
    public List<Connection> Added { get; set; } = new();

    public static implicit operator ConnectionsDiff(List<Connection> connections) => new() { Updated = connections.Select(c => new ConnectionDiffUpdate { Connection = c, Diff = (ConnectionDiff)c }).ToList() };

    public ConnectionsDiff MergeDiff(ConnectionsDiff other)
    {
        return new ConnectionsDiff
        {
            Removed = other.Removed.Concat(Removed).Distinct().ToList(),
            Updated = other.Updated.Concat(Updated).GroupBy(u => u.Connection.Guid).Select(g => g.Last()).ToList(),
            Added = other.Added.Concat(Added).GroupBy(a => a.Connected.Piece.Guid + "--" + a.Connecting.Piece.Guid).Select(g => g.Last()).ToList()
        };
    }
}

public class Connection : Entity<Connection>
{
    public string Guid { get; set; } = "";
    public Side Connected { get; set; } = new();
    public Side Connecting { get; set; } = new();
    public string? Description { get; set; }
    public double Gap { get; set; } = 0;
    public double Shift { get; set; } = 0;
    public double Rise { get; set; } = 0;
    public double Rotation { get; set; } = 0;
    public double Turn { get; set; } = 0;
    public double Tilt { get; set; } = 0;
    public double? U { get; set; }
    public double? V { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Connected.Piece.Guid + (Connected.Connector.Guid != "" ? ":" + Connected.Connector.Guid : "")}--{(Connecting.Connector.Guid != "" ? Connecting.Connector.Guid + ":" : "") + Connecting.Piece.Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Con({ToHumanIdString()})";

    public static implicit operator Connection(ConnectionId id) => new() { Connected = id.Connected, Connecting = id.Connecting };
    public static implicit operator Connection(ConnectionDiff diff) => new() { Connected = diff.Connected ?? new(), Connecting = diff.Connecting ?? new(), Description = diff.Description ?? "", Gap = diff.Gap ?? 0, Shift = diff.Shift ?? 0, Rise = diff.Rise ?? 0, Rotation = diff.Rotation ?? 0, Turn = diff.Turn ?? 0, Tilt = diff.Tilt ?? 0, U = diff.U, V = diff.V, Attributes = diff.Attributes?.Added ?? new() };

    public static Connection ApplyDiff(Connection connection, ConnectionDiff diff)
    {
        return new Connection
        {
            Connected = diff.Connected is not null ? Side.ApplyDiff(connection.Connected, diff.Connected) : connection.Connected,
            Connecting = diff.Connecting is not null ? Side.ApplyDiff(connection.Connecting, diff.Connecting) : connection.Connecting,
            Description = string.IsNullOrEmpty(diff.Description) ? connection.Description : diff.Description,
            Gap = diff.Gap ?? connection.Gap,
            Shift = diff.Shift ?? connection.Shift,
            Rise = diff.Rise ?? connection.Rise,
            Rotation = diff.Rotation ?? connection.Rotation,
            Turn = diff.Turn ?? connection.Turn,
            Tilt = diff.Tilt ?? connection.Tilt,
            U = diff.U ?? connection.U,
            V = diff.V ?? connection.V,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(connection.Attributes, diff.Attributes) : connection.Attributes
        };
    }

    public static ConnectionDiff CreateDiff(Connection connection)
    {
        return new ConnectionDiff
        {
            Connected = Side.CreateDiff(connection.Connected),
            Connecting = Side.CreateDiff(connection.Connecting),
            Description = connection.Description,
            Gap = connection.Gap,
            Shift = connection.Shift,
            Rise = connection.Rise,
            Rotation = connection.Rotation,
            Turn = connection.Turn,
            Tilt = connection.Tilt,
            U = connection.U,
            V = connection.V,
            Attributes = connection.Attributes
        };
    }

    public static ConnectionDiff InverseDiff(Connection connection, ConnectionDiff appliedDiff)
    {
        return new ConnectionDiff
        {
            Connected = appliedDiff.Connected is not null ? Side.CreateDiff(connection.Connected) : null,
            Connecting = appliedDiff.Connecting is not null ? Side.CreateDiff(connection.Connecting) : null,
            Description = appliedDiff.Description is not null ? connection.Description : "",
            Gap = appliedDiff.Gap.HasValue ? connection.Gap : null,
            Shift = appliedDiff.Shift.HasValue ? connection.Shift : null,
            Rise = appliedDiff.Rise.HasValue ? connection.Rise : null,
            Rotation = appliedDiff.Rotation.HasValue ? connection.Rotation : null,
            Turn = appliedDiff.Turn.HasValue ? connection.Turn : null,
            Tilt = appliedDiff.Tilt.HasValue ? connection.Tilt : null,
            U = appliedDiff.U.HasValue ? connection.U : null,
            V = appliedDiff.V.HasValue ? connection.V : null,
            Attributes = appliedDiff.Attributes is not null ? connection.Attributes : null
        };
    }

    public static bool IsSameAs(Connection connection, Connection other, bool strict = false)
    {
        if (other is null) return false;
        if (strict)
        {
            return connection.Connected.Piece.Guid == other.Connected.Piece.Guid &&
                   connection.Connected.Connector.Guid == other.Connected.Connector.Guid &&
                   connection.Connecting.Piece.Guid == other.Connecting.Piece.Guid &&
                   connection.Connecting.Connector.Guid == other.Connecting.Connector.Guid;
        }
        return (connection.Connected.Piece.Guid == other.Connected.Piece.Guid && connection.Connecting.Piece.Guid == other.Connecting.Piece.Guid) ||
               (connection.Connected.Piece.Guid == other.Connecting.Piece.Guid && connection.Connecting.Piece.Guid == other.Connected.Piece.Guid);
    }

    public static Connection SetAttribute(Connection connection, Attribute attribute)
    {
        var attributes = new List<Attribute>(connection.Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Connection
        {
            Connected = connection.Connected,
            Connecting = connection.Connecting,
            Description = connection.Description,
            Gap = connection.Gap,
            Shift = connection.Shift,
            Rise = connection.Rise,
            Rotation = connection.Rotation,
            Turn = connection.Turn,
            Tilt = connection.Tilt,
            U = connection.U,
            V = connection.V,
            Attributes = attributes
        };
    }

    public static Connection Find(List<Connection> connections, string connectionGuid)
    {
        var connection = connections.FirstOrDefault(c => c.Guid == connectionGuid);
        if (connection == null) throw new Exception($"Connection {connectionGuid} not found in connections");
        return connection;
    }

    public static List<Connection> FindByPiece(List<Connection> connections, string pieceGuid)
    {
        return connections.Where(c => c.Connected.Piece.Guid == pieceGuid || c.Connecting.Piece.Guid == pieceGuid).ToList();
    }

    public static Connection FindInDesign(Design design, string connectionGuid)
    {
        return Find(design.Connections ?? new List<Connection>(), connectionGuid);
    }

    public static List<Connection> FindManyInDesign(Design design, List<string> connectionGuids)
    {
        return connectionGuids.Select(g => FindInDesign(design, g)).ToList();
    }

    public static List<Connection> FindByPieceInDesign(Design design, string pieceGuid)
    {
        return FindByPiece(design.Connections ?? new List<Connection>(), pieceGuid);
    }

    public static (Piece connecting, Piece connected) FindPiecesInDesign(Design design, Connection connection)
    {
        return (
            Piece.FindInDesign(design, connection.Connecting.Piece.Guid),
            Piece.FindInDesign(design, connection.Connected.Piece.Guid)
        );
    }

    public static List<Connection> FindStaleInDesign(Design design)
    {
        return (design.Connections ?? new List<Connection>()).Where(c =>
        {
            try
            {
                Piece.FindInDesign(design, c.Connected.Piece.Guid);
                Piece.FindInDesign(design, c.Connecting.Piece.Guid);
                return false;
            }
            catch
            {
                return true;
            }
        }).ToList();
    }
}

#endregion 🔗Connection






#region 📈Stat
// Implementations MUST associate statistical metrics with a design.

public class StatId : Entity<StatId>
{
    public string Guid { get; set; } = "";
    public static implicit operator StatId(Stat stat) => new() { Guid = stat.Guid };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"SttI({ToHumanIdString()})";
}

public class Stat : Entity<Stat>
{
    public string Guid { get; set; } = "";
    public QualityId Quality { get; set; } = new();
    public string? Unit { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Stt({ToHumanIdString()})";
}

public class StatDiff : Entity<StatDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private QualityId? _quality;
    private string? _unit;
    private double? _min;
    private bool? _minExcluded;
    private double? _max;
    private bool? _maxExcluded;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public QualityId? Quality { get => _quality; set { _quality = value; _setProperties.Add("Quality"); } }
    public string? Unit { get => _unit; set { _unit = value; _setProperties.Add("Unit"); } }
    public double? Min { get => _min; set { _min = value; _setProperties.Add("Min"); } }
    public bool? MinExcluded { get => _minExcluded; set { _minExcluded = value; _setProperties.Add("MinExcluded"); } }
    public double? Max { get => _max; set { _max = value; _setProperties.Add("Max"); } }
    public bool? MaxExcluded { get => _maxExcluded; set { _maxExcluded = value; _setProperties.Add("MaxExcluded"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeQuality() => _setProperties.Contains("Quality");
    public bool ShouldSerializeUnit() => _setProperties.Contains("Unit");
    public bool ShouldSerializeMin() => _setProperties.Contains("Min");
    public bool ShouldSerializeMinExcluded() => _setProperties.Contains("MinExcluded");
    public bool ShouldSerializeMax() => _setProperties.Contains("Max");
    public bool ShouldSerializeMaxExcluded() => _setProperties.Contains("MaxExcluded");
}

#endregion 📈Stat






#region 📐Design
// Implementations MUST compose pieces, connections, and metadata into a layout.

public class DesignsDiff : Entity<DesignsDiff>
{
    public List<DesignId> Removed { get; set; } = new();
    public List<DesignDiffUpdate> Updated { get; set; } = new();
    public List<Design> Added { get; set; } = new();

    public static implicit operator DesignsDiff(List<Design> designs) => new() { Updated = designs.Select(d => new DesignDiffUpdate { Design = d, Diff = (DesignDiff)d }).ToList() };
}

public class DesignDiff : Entity<DesignDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private DesignId? _parent;
    private bool? _isAbstract;
    private string? _folder;
    private string? _description;
    private string? _icon;
    private string? _image;
    private Location? _location;
    private string? _unit;
    private bool? _canScale;
    private bool? _canMirror;
    private LayerId? _activeLayer;
    private PiecesDiff? _pieces;
    private ConnectionsDiff? _connections;
    private List<Prop>? _props;
    private List<Stat>? _stats;
    private List<Layer>? _layers;
    private List<Group>? _groups;
    private List<AuthorId>? _authors;
    private List<ConceptId>? _concepts;
    private AttributesDiff? _attributes;
    private DateTime? _createdAt;
    private DateTime? _updatedAt;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public DesignId? Parent { get => _parent; set { _parent = value; _setProperties.Add("Parent"); } }
    public bool? IsAbstract { get => _isAbstract; set { _isAbstract = value; _setProperties.Add("IsAbstract"); } }
    public string? Folder { get => _folder; set { _folder = value; _setProperties.Add("Folder"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public string? Image { get => _image; set { _image = value; _setProperties.Add("Image"); } }
    public Location? Location { get => _location; set { _location = value; _setProperties.Add("Location"); } }
    public string? Unit { get => _unit; set { _unit = value; _setProperties.Add("Unit"); } }
    public bool? CanScale { get => _canScale; set { _canScale = value; _setProperties.Add("CanScale"); } }
    public bool? CanMirror { get => _canMirror; set { _canMirror = value; _setProperties.Add("CanMirror"); } }
    public LayerId? ActiveLayer { get => _activeLayer; set { _activeLayer = value; _setProperties.Add("ActiveLayer"); } }
    public PiecesDiff? Pieces { get => _pieces; set { _pieces = value; _setProperties.Add("Pieces"); } }
    public ConnectionsDiff? Connections { get => _connections; set { _connections = value; _setProperties.Add("Connections"); } }
    public List<Prop>? Props { get => _props; set { _props = value; _setProperties.Add("Props"); } }
    public List<Stat>? Stats { get => _stats; set { _stats = value; _setProperties.Add("Stats"); } }
    public List<Layer>? Layers { get => _layers; set { _layers = value; _setProperties.Add("Layers"); } }
    public List<Group>? Groups { get => _groups; set { _groups = value; _setProperties.Add("Groups"); } }
    public List<AuthorId>? Authors { get => _authors; set { _authors = value; _setProperties.Add("Authors"); } }
    public List<ConceptId>? Concepts { get => _concepts; set { _concepts = value; _setProperties.Add("Concepts"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }
    public DateTime? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public DateTime? UpdatedAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("UpdatedAt"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeParent() => _setProperties.Contains("Parent");
    public bool ShouldSerializeIsAbstract() => _setProperties.Contains("IsAbstract");
    public bool ShouldSerializeFolder() => _setProperties.Contains("Folder");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeImage() => _setProperties.Contains("Image");
    public bool ShouldSerializeLocation() => _setProperties.Contains("Location");
    public bool ShouldSerializeUnit() => _setProperties.Contains("Unit");
    public bool ShouldSerializeCanScale() => _setProperties.Contains("CanScale");
    public bool ShouldSerializeCanMirror() => _setProperties.Contains("CanMirror");
    public bool ShouldSerializeActiveLayer() => _setProperties.Contains("ActiveLayer");
    public bool ShouldSerializePieces() => _setProperties.Contains("Pieces");
    public bool ShouldSerializeConnections() => _setProperties.Contains("Connections");
    public bool ShouldSerializeProps() => _setProperties.Contains("Props");
    public bool ShouldSerializeStats() => _setProperties.Contains("Stats");
    public bool ShouldSerializeLayers() => _setProperties.Contains("Layers");
    public bool ShouldSerializeGroups() => _setProperties.Contains("Groups");
    public bool ShouldSerializeAuthors() => _setProperties.Contains("Authors");
    public bool ShouldSerializeConcepts() => _setProperties.Contains("Concepts");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeUpdatedAt() => _setProperties.Contains("UpdatedAt");

    public static implicit operator DesignDiff(DesignId id) => new() { Guid = id.Guid };
    public static implicit operator DesignDiff(Design design) => new() { Guid = design.Guid, Name = design.Name, Parent = design.Parent, IsAbstract = design.IsAbstract, Folder = design.Folder, Description = design.Description, Icon = design.Icon, Image = design.Image, Location = design.Location, Unit = design.Unit, CanScale = design.CanScale, CanMirror = design.CanMirror, ActiveLayer = design.ActiveLayer, Pieces = new PiecesDiff { Removed = new List<PieceId>(), Updated = design.Pieces.Select(p => new PieceDiffUpdate { Piece = p, Diff = Piece.CreateDiff(p) }).ToList(), Added = new List<Piece>() }, Connections = new ConnectionsDiff { Removed = new List<ConnectionId>(), Updated = design.Connections.Select(c => new ConnectionDiffUpdate { Connection = c, Diff = Connection.CreateDiff(c) }).ToList(), Added = new List<Connection>() }, Props = design.Props, Stats = design.Stats, Layers = design.Layers, Groups = design.Groups, Authors = design.Authors, Concepts = design.Concepts, Attributes = design.Attributes, CreatedAt = design.CreatedAt, UpdatedAt = design.UpdatedAt };

    public DesignDiff MergeDiff(DesignDiff other)
    {
        return new DesignDiff
        {
            Guid = other.Guid ?? Guid,
            Name = other.Name ?? Name,
            Parent = other.Parent ?? Parent,
            IsAbstract = other.IsAbstract ?? IsAbstract,
            Folder = other.Folder ?? Folder,
            Description = other.Description ?? Description,
            Icon = other.Icon ?? Icon,
            Image = other.Image ?? Image,
            Location = other.Location ?? Location,
            Unit = other.Unit ?? Unit,
            CanScale = other.CanScale ?? CanScale,
            CanMirror = other.CanMirror ?? CanMirror,
            ActiveLayer = other.ActiveLayer ?? ActiveLayer,
            Pieces = other.Pieces is not null ? (other.Pieces.MergeDiff(Pieces ?? new PiecesDiff())) : Pieces,
            Connections = other.Connections is not null ? (other.Connections.MergeDiff(Connections ?? new ConnectionsDiff())) : Connections,
            Props = other.Props ?? Props,
            Stats = other.Stats ?? Stats,
            Layers = other.Layers ?? Layers,
            Groups = other.Groups ?? Groups,
            Authors = other.Authors ?? Authors,
            Concepts = other.Concepts ?? Concepts,
            Attributes = other.Attributes ?? Attributes,
            CreatedAt = other.CreatedAt ?? CreatedAt,
            UpdatedAt = other.UpdatedAt ?? UpdatedAt
        };
    }
}

public class DesignId : Entity<DesignId>
{
    public string Guid { get; set; } = "";
    public static implicit operator DesignId(Design design) => new() { Guid = design.Guid };
    public static implicit operator DesignId(DesignDiff diff) => new() { Guid = diff.Guid ?? "" };

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Guid}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"DsnId({ToHumanIdString()})";
}

public class Design : Entity<Design>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public DesignId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public List<ConceptId> Concepts { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public bool? CanScale { get; set; }
    public bool? CanMirror { get; set; }
    public List<Layer> Layers { get; set; } = new();
    public LayerId? ActiveLayer { get; set; }
    public List<Piece> Pieces { get; set; } = new();
    public List<Group> Groups { get; set; } = new();
    public List<Connection> Connections { get; set; } = new();
    public List<Prop> Props { get; set; } = new();
    public List<Stat> Stats { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Dsn({ToHumanIdString()})";

    public static implicit operator Design(DesignId id) => new() { Guid = id.Guid, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };
    public static implicit operator Design(DesignDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Parent = diff.Parent, IsAbstract = diff.IsAbstract, Folder = diff.Folder, Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Location = diff.Location, Unit = diff.Unit ?? "", CanScale = diff.CanScale, CanMirror = diff.CanMirror, ActiveLayer = diff.ActiveLayer, Attributes = diff.Attributes?.Added ?? new(), Authors = diff.Authors ?? new(), Concepts = diff.Concepts ?? new(), CreatedAt = diff.CreatedAt ?? DateTime.UtcNow, UpdatedAt = diff.UpdatedAt ?? DateTime.UtcNow };
    public static implicit operator string(Design design) => design.Name;
    public static implicit operator Design(string name) => new() { Name = name, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };

    public static Design ApplyDiff(Design design, DesignDiff diff)
    {
        var pieces = design.Pieces;
        var connections = design.Connections;

        if (diff.Pieces is not null)
        {
            pieces = ApplyPiecesDiff(design.Pieces, diff.Pieces);
        }
        if (diff.Connections is not null)
        {
            connections = ApplyConnectionsDiff(design.Connections, diff.Connections);
        }

        return new Design
        {
            Guid = diff.Guid ?? design.Guid,
            Name = diff.Name ?? design.Name,
            Parent = diff.Parent ?? design.Parent,
            IsAbstract = diff.IsAbstract ?? design.IsAbstract,
            Folder = diff.Folder ?? design.Folder,
            Description = diff.Description ?? design.Description,
            Icon = diff.Icon ?? design.Icon,
            Image = diff.Image ?? design.Image,
            Location = diff.Location ?? design.Location,
            Unit = diff.Unit ?? design.Unit,
            ActiveLayer = diff.ActiveLayer ?? design.ActiveLayer,
            Pieces = pieces,
            Connections = connections,
            Props = diff.Props ?? design.Props,
            Stats = diff.Stats ?? design.Stats,
            Layers = diff.Layers ?? design.Layers,
            Groups = diff.Groups ?? design.Groups,
            CanScale = diff.CanScale ?? design.CanScale,
            CanMirror = diff.CanMirror ?? design.CanMirror,
            Attributes = diff.Attributes is not null ? AttributesDiff.Apply(design.Attributes, diff.Attributes) : design.Attributes,
            Authors = diff.Authors ?? design.Authors,
            Concepts = diff.Concepts ?? design.Concepts,
            CreatedAt = diff.CreatedAt ?? design.CreatedAt,
            UpdatedAt = diff.UpdatedAt ?? design.UpdatedAt
        };
    }

    public static DesignDiff CreateDiff(Design design)
    {
        return new DesignDiff
        {
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = new PiecesDiff
            {
                Removed = new List<PieceId>(),
                Updated = design.Pieces.Select(p => new PieceDiffUpdate { Piece = p, Diff = Piece.CreateDiff(p) }).ToList(),
                Added = new List<Piece>()
            },
            Connections = new ConnectionsDiff
            {
                Removed = new List<ConnectionId>(),
                Updated = design.Connections.Select(c => new ConnectionDiffUpdate { Connection = c, Diff = Connection.CreateDiff(c) }).ToList(),
                Added = new List<Connection>()
            },
            Stats = design.Stats,
            Authors = design.Authors,
            Attributes = design.Attributes,
            Concepts = design.Concepts
        };
    }

    public static DesignDiff GetDesignDiff(Design design, Design other)
    {
        var diff = new DesignDiff();

        if (design.Name != other.Name) diff.Name = other.Name;
        if (design.Description != other.Description) diff.Description = other.Description;
        if (design.Icon != other.Icon) diff.Icon = other.Icon;
        if (design.Image != other.Image) diff.Image = other.Image;
        if (design.Unit != other.Unit) diff.Unit = other.Unit;
        if (design.Folder != other.Folder) diff.Folder = other.Folder;
        if (design.IsAbstract != other.IsAbstract) diff.IsAbstract = other.IsAbstract;
        if (design.CanScale != other.CanScale) diff.CanScale = other.CanScale;
        if (design.CanMirror != other.CanMirror) diff.CanMirror = other.CanMirror;
        if (design.ActiveLayer != other.ActiveLayer) diff.ActiveLayer = other.ActiveLayer;
        if ((design.Parent?.Guid ?? "") != (other.Parent?.Guid ?? "")) diff.Parent = other.Parent;
        if ((design.Location?.Guid ?? "") != (other.Location?.Guid ?? "")) diff.Location = other.Location;

        var piecesDiff = CreatePiecesDiff(design.Pieces, other.Pieces);
        if (piecesDiff.Removed.Any() || piecesDiff.Updated.Any() || piecesDiff.Added.Any())
            diff.Pieces = piecesDiff;

        var connectionsDiff = CreateConnectionsDiff(design.Connections, other.Connections);
        if (connectionsDiff.Removed.Any() || connectionsDiff.Updated.Any() || connectionsDiff.Added.Any())
            diff.Connections = connectionsDiff;

        return diff;
    }

    private static List<Piece> ApplyPiecesDiff(List<Piece> original, PiecesDiff diff)
    {
        var result = original.Where(p => !diff.Removed.Any(r => r.Guid == p.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(p => p.Guid == updated.Piece.Guid);
            if (index >= 0 && updated.Diff != null)
                result[index] = Piece.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added.Select(a => new Piece
        {
            Guid = a.Guid ?? "",
            Description = a.Description ?? "",
            Type = a.Type ?? new TypeId { Guid = "" },
            Plane = a.Plane,
            Center = a.Center,
            Attributes = a.Attributes ?? new List<Attribute>()
        }));
        return result;
    }

    private static PiecesDiff CreatePiecesDiff(List<Piece> original, List<Piece> modified)
    {
        var originalIds = original.Select(p => p.Guid).ToHashSet();
        var modifiedIds = modified.Select(p => p.Guid).ToHashSet();

        return new PiecesDiff
        {
            Removed = original.Where(p => !modifiedIds.Contains(p.Guid)).Select(p => new PieceId { Guid = p.Guid }).ToList(),
            Updated = original.Where(p => modifiedIds.Contains(p.Guid))
                .SelectMany(p =>
                {
                    var modifiedPiece = modified.First(m => m.Guid == p.Guid);
                    var diff = Piece.CreateDiff(p);
                    return !Equals(p, modifiedPiece) ? new[] { new PieceDiffUpdate { Piece = p, Diff = diff } } : Array.Empty<PieceDiffUpdate>();
                })
                .ToList(),
            Added = modified.Where(p => !originalIds.Contains(p.Guid)).ToList()
        };
    }

    private static List<Connection> ApplyConnectionsDiff(List<Connection> original, ConnectionsDiff diff)
    {
        var result = original.Where(c => !diff.Removed.Any(r => r.Guid == c.Guid)).ToList();

        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(c => c.Guid == updated.Connection.Guid);
            if (index >= 0 && updated.Diff != null)
                result[index] = Connection.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static ConnectionsDiff CreateConnectionsDiff(List<Connection> original, List<Connection> modified)
    {
        var originalGuids = original.Select(c => c.Guid).ToHashSet();
        var modifiedGuids = modified.Select(c => c.Guid).ToHashSet();

        return new ConnectionsDiff
        {
            Removed = original.Where(c => !modifiedGuids.Contains(c.Guid)).Select(c => new ConnectionId { Guid = c.Guid }).ToList(),
            Updated = original.Where(c => modifiedGuids.Contains(c.Guid))
                .SelectMany(c =>
                {
                    var modifiedConnection = modified.First(m => m.Guid == c.Guid);
                    var diff = Connection.CreateDiff(c);
                    return !Equals(c, modifiedConnection) ? new[] { new ConnectionDiffUpdate { Connection = c, Diff = diff } } : Array.Empty<ConnectionDiffUpdate>();
                })
                .ToList(),
            Added = modified.Where(c => !originalGuids.Contains(c.Guid)).ToList()
        };
    }

    public static void Bfs(Design design, Action<Piece> onRoot, Action<Piece, Piece, Connection> onConnection)
    {
        var pieces = design.Pieces.ToDictionary(p => p.Guid);
        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var piece in design.Pieces)
            graph.AddVertex(piece.Guid);
        foreach (var connection in design.Connections)
            graph.AddEdge(new Edge<string>(connection.Connected.Piece.Guid, connection.Connecting.Piece.Guid));
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
            foreach (var connection in design.Connections)
                if (component.Value.ContainsKey(connection.Connected.Piece.Guid) &&
                    component.Value.ContainsKey(connection.Connecting.Piece.Guid))
                    subGraph.AddEdge(
                        new Edge<string>(connection.Connected.Piece.Guid, connection.Connecting.Piece.Guid));
            var root = subGraph.Vertices.FirstOrDefault(p => pieces[p].Plane is not null);
            if (root is null)
                root = subGraph.Vertices.First();

            onRoot(pieces[root]);

            var bfs = new UndirectedBreadthFirstSearchAlgorithm<string, Edge<string>>(subGraph);
            bfs.SetRootVertex(root);
            bfs.TreeEdge += (g, edge) =>
            {
                var parent = pieces[edge.Source];
                var child = pieces[edge.Target];
                var connection = design.Connections.First(c =>
                    (c.Connected.Piece.Guid == parent.Guid && c.Connecting.Piece.Guid == child.Guid) ||
                    (c.Connected.Piece.Guid == child.Guid && c.Connecting.Piece.Guid == parent.Guid));
                onConnection(parent, child, connection);
            };
            bfs.Compute();
        }
    }

    public static Design Flatten(Design design, IEnumerable<Type> types,
        Func<Plane, Point, Vector, Point, Vector, double, double, double, double, double, double, Plane> computeChildPlane)
    {
        if (design.Pieces.Count > 1 && design.Connections.Count > 0)
        {
            var connectors = new Dictionary<string, Dictionary<string, Connector>>();
            foreach (var type in types)
            {
                if (!connectors.ContainsKey(type.Guid))
                    connectors[type.Guid] = new Dictionary<string, Connector>();
                foreach (var connector in type.Connectors)
                    connectors[type.Guid][connector.Guid] = connector;
            }

            foreach (var piece in design.Pieces)
            {
                if (piece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({piece.Guid}) has no type.");
                if (!types.Any(t => t.Guid == piece.Type.Guid))
                    throw new Exception(
                        $"The type {piece.Type.ToHumanIdString()} of the piece {piece.ToHumanIdString()} is not provided.");
            }
            foreach (var connection in design.Connections)
            {
                var connectedPiece = design.Pieces.First(p => p.Guid == connection.Connected.Piece.Guid);
                if (connectedPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectedPiece.Guid}) has no type.");
                var connectedType = types.First(t => t.Guid == connectedPiece.Type.Guid);
                if (!connectors[connectedType.Guid].ContainsKey(connection.Connected.Connector.Guid))
                    throw new Exception(
                        $"The type {connectedType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the connector {connection.Connected.Connector.Guid}.");
                var connectingPiece = design.Pieces.First(p => p.Guid == connection.Connecting.Piece.Guid);
                if (connectingPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectingPiece.Guid}) has no type.");
                var connectingType = types.First(t => t.Guid == connectingPiece.Type.Guid);
                if (!connectors[connectingType.Guid].ContainsKey(connection.Connecting.Connector.Guid))
                    throw new Exception(
                        $"The type {connectingType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the connector {connection.Connecting.Connector.Guid}.");
            }

            var piecePlanes = new Dictionary<string, Plane>();
            var pieceCenters = new Dictionary<string, Coord>();
            var pieceAttributes = new Dictionary<string, List<Attribute>>();

            var onRoot = new Action<Piece>(piece =>
            {
                piecePlanes[piece.Guid] = piece.Plane ?? new Plane();
                pieceCenters[piece.Guid] = piece.Center ?? new Coord();
            });
            var onConnection = new Action<Piece, Piece, Connection>((parent, child, connection) =>
            {
                var isParentConnected = connection.Connected.Piece.Guid == parent.Guid;
                if (!piecePlanes.TryGetValue(parent.Guid, out var parentPlane) || parent.Type is null || child.Type is null) return;
                var parentConnector =
                    connectors[parent.Type.Guid][
                        isParentConnected ? connection.Connected.Connector.Guid : connection.Connecting.Connector.Guid];
                var childConnector =
                    connectors[child.Type.Guid][
                        isParentConnected ? connection.Connecting.Connector.Guid : connection.Connected.Connector.Guid];
                if (parentConnector.Point is null || parentConnector.Direction is null || childConnector.Point is null || childConnector.Direction is null) return;
                var childPlane = computeChildPlane(parentPlane, parentConnector.Point, parentConnector.Direction,
                    childConnector.Point, childConnector.Direction,
                    connection.Gap, connection.Shift, connection.Rise,
                    connection.Rotation, connection.Turn, connection.Tilt);
                piecePlanes[child.Guid] = childPlane;

                var radius = 2.697;
                var verticalVExtra = 1.0;
                var horizontalScale = 3.0633;
                var parentCenter = pieceCenters.TryGetValue(parent.Guid, out var pc) ? pc : new Coord();
                var connectionU = connection.U ?? 0;
                var connectionV = connection.V ?? 0;

                double childU, childV;
                if (parentCenter.U == 0 && parentCenter.V == 0)
                {
                    var angle = 2 * Math.PI * parentConnector.T;
                    childU = radius * Math.Sin(angle);
                    childV = radius * Math.Cos(angle);
                }
                else
                {
                    var isVerticalConnection = Math.Abs(parentConnector.Direction.Z) > 0.5;
                    if (isVerticalConnection)
                    {
                        childU = parentCenter.U + connectionU;
                        childV = parentCenter.V + connectionV + verticalVExtra;
                    }
                    else
                    {
                        childU = parentCenter.U + connectionU * horizontalScale;
                        childV = parentCenter.V + connectionV * horizontalScale;
                    }
                }

                pieceCenters[child.Guid] = new Coord { U = Math.Round(childU, 6), V = Math.Round(childV, 6) };

                var existingAttributes = new List<Attribute>(child.Attributes);
                var semioAttribute = existingAttributes.FirstOrDefault(q => q.Key == "semio.parent");
                if (semioAttribute is not null)
                {
                    var idx = existingAttributes.IndexOf(semioAttribute);
                    existingAttributes[idx] = new Attribute { Guid = semioAttribute.Guid, Key = semioAttribute.Key, Value = parent.Guid, Definition = semioAttribute.Definition };
                }
                else
                {
                    existingAttributes.Add(new Attribute
                    {
                        Key = "semio.parent",
                        Value = parent.Guid
                    });
                }
                pieceAttributes[child.Guid] = existingAttributes;
            });
            Bfs(design, onRoot, onConnection);

            var newPieces = design.Pieces.Select(piece =>
            {
                var newPiece = Entity<Piece>.DeepClone(piece)!;
                if (piecePlanes.TryGetValue(piece.Guid, out var plane)) newPiece.Plane = plane;
                if (pieceCenters.TryGetValue(piece.Guid, out var center)) newPiece.Center = center;
                if (pieceAttributes.TryGetValue(piece.Guid, out var attrs)) newPiece.Attributes = attrs;
                return newPiece;
            }).ToList();

            return new Design
            {
                Guid = design.Guid,
                Name = design.Name,
                Parent = design.Parent,
                IsAbstract = design.IsAbstract,
                Folder = design.Folder,
                Description = design.Description,
                Icon = design.Icon,
                Image = design.Image,
                Location = design.Location,
                Unit = design.Unit,
                ActiveLayer = design.ActiveLayer,
                CanScale = design.CanScale,
                CanMirror = design.CanMirror,
                Pieces = newPieces,
                Connections = new List<Connection>(),
                Props = design.Props,
                Stats = design.Stats,
                Layers = design.Layers,
                Groups = design.Groups,
                Authors = design.Authors,
                Concepts = design.Concepts,
                Attributes = design.Attributes,
                CreatedAt = design.CreatedAt,
                UpdatedAt = design.UpdatedAt
            };
        }

        return new Design
        {
            Guid = design.Guid,
            Name = design.Name,
            Parent = design.Parent,
            IsAbstract = design.IsAbstract,
            Folder = design.Folder,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            ActiveLayer = design.ActiveLayer,
            CanScale = design.CanScale,
            CanMirror = design.CanMirror,
            Pieces = new List<Piece>(design.Pieces),
            Connections = new List<Connection>(),
            Props = design.Props,
            Stats = design.Stats,
            Layers = design.Layers,
            Groups = design.Groups,
            Authors = design.Authors,
            Concepts = design.Concepts,
            Attributes = design.Attributes,
            CreatedAt = design.CreatedAt,
            UpdatedAt = design.UpdatedAt
        };
    }

    public static Design Flatten(Design design, IEnumerable<Type> types) => Flatten(design, types, DefaultComputeChildPlane);

    public static Plane DefaultComputeChildPlane(
        Plane parentPlane,
        Point parentPoint,
        Vector parentDirection,
        Point childPoint,
        Vector childDirection,
        double gap,
        double shift,
        double rise,
        double rotation,
        double turn,
        double tilt)
    {
        var pMatrix = PlaneToMatrix(parentPlane);

        var pPoint = new System.Numerics.Vector3((float)parentPoint.X, (float)parentPoint.Y, (float)parentPoint.Z);
        var pDir = System.Numerics.Vector3.Normalize(new System.Numerics.Vector3((float)parentDirection.X, (float)parentDirection.Y, (float)parentDirection.Z));
        var cPoint = new System.Numerics.Vector3((float)childPoint.X, (float)childPoint.Y, (float)childPoint.Z);
        var cDir = System.Numerics.Vector3.Normalize(new System.Numerics.Vector3((float)childDirection.X, (float)childDirection.Y, (float)childDirection.Z));

        var rotationRad = DegreesToRadians(rotation);
        var turnRad = DegreesToRadians(turn);
        var tiltRad = DegreesToRadians(tilt);

        var reverseChildDirection = -cDir;

        System.Numerics.Quaternion alignQuat;
        var cross = System.Numerics.Vector3.Cross(pDir, reverseChildDirection);
        if (cross.LengthSquared() < 0.0001f)
        {
            var dotProduct = System.Numerics.Vector3.Dot(pDir, reverseChildDirection);
            if (dotProduct > 0)
            {
                alignQuat = System.Numerics.Quaternion.Identity;
            }
            else
            {
                if (Math.Abs(pDir.Z) < 1e-5f)
                {
                    alignQuat = System.Numerics.Quaternion.CreateFromAxisAngle(System.Numerics.Vector3.UnitZ, (float)Math.PI);
                }
                else
                {
                    var crossAxis = System.Numerics.Vector3.Cross(System.Numerics.Vector3.UnitZ, pDir);
                    System.Numerics.Vector3 axis;
                    if (crossAxis.LengthSquared() < 0.0001f)
                    {
                        axis = System.Numerics.Vector3.UnitX;
                    }
                    else
                    {
                        axis = System.Numerics.Vector3.Normalize(crossAxis);
                    }
                    alignQuat = System.Numerics.Quaternion.CreateFromAxisAngle(axis, (float)Math.PI);
                }
            }
        }
        else
        {
            alignQuat = CreateFromTwoVectors(reverseChildDirection, pDir);
        }

        var directionT = QuaternionToMatrix(alignQuat);

        var yAxis = System.Numerics.Vector3.UnitY;
        var parentConnectorQuat = CreateFromTwoVectors(yAxis, pDir);
        var parentRotationT = QuaternionToMatrix(parentConnectorQuat);

        var gapDirection = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitY);
        var shiftDirection = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitX);
        var raiseDirection = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitZ);
        var turnAxis = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitZ);
        var tiltAxis = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitX);

        var orientationT = directionT;
        var rotateT = MakeRotationAxis(pDir, -rotationRad);
        orientationT = MultiplyMatrices(rotateT, orientationT);

        turnAxis = ApplyMatrix4ToVec3(rotateT, turnAxis);
        tiltAxis = ApplyMatrix4ToVec3(rotateT, tiltAxis);

        var turnT = MakeRotationAxis(turnAxis, turnRad);
        orientationT = MultiplyMatrices(turnT, orientationT);

        var tiltT = MakeRotationAxis(tiltAxis, tiltRad);
        orientationT = MultiplyMatrices(tiltT, orientationT);

        var centerChildT = MakeTranslation(-cPoint.X, -cPoint.Y, -cPoint.Z);

        var transform = MultiplyMatrices(orientationT, centerChildT);

        var gapT = MakeTranslation(gapDirection.X * (float)gap, gapDirection.Y * (float)gap, gapDirection.Z * (float)gap);
        var shiftT = MakeTranslation(shiftDirection.X * (float)shift, shiftDirection.Y * (float)shift, shiftDirection.Z * (float)shift);
        var raiseT = MakeTranslation(raiseDirection.X * (float)rise, raiseDirection.Y * (float)rise, raiseDirection.Z * (float)rise);

        var translationT = MultiplyMatrices(raiseT, MultiplyMatrices(shiftT, gapT));
        transform = MultiplyMatrices(translationT, transform);

        var moveToParentT = MakeTranslation(pPoint.X, pPoint.Y, pPoint.Z);
        transform = MultiplyMatrices(moveToParentT, transform);

        var finalMatrix = MultiplyMatrices(pMatrix, transform);

        return MatrixToPlane(finalMatrix);
    }

    private static float DegreesToRadians(double deg) => (float)(deg * Math.PI / 180.0);

    private static System.Numerics.Quaternion CreateFromTwoVectors(System.Numerics.Vector3 u, System.Numerics.Vector3 v)
    {
        float dot = System.Numerics.Vector3.Dot(u, v);
        if (dot > 0.999999f) return System.Numerics.Quaternion.Identity;
        if (dot < -0.999999f)
        {
            var axis = System.Numerics.Vector3.Cross(System.Numerics.Vector3.UnitX, u);
            if (axis.LengthSquared() < 0.001f)
                axis = System.Numerics.Vector3.Cross(System.Numerics.Vector3.UnitY, u);
            axis = System.Numerics.Vector3.Normalize(axis);
            return System.Numerics.Quaternion.CreateFromAxisAngle(axis, (float)Math.PI);
        }

        var axisNorm = System.Numerics.Vector3.Cross(u, v);
        var q = new System.Numerics.Quaternion(axisNorm.X, axisNorm.Y, axisNorm.Z, 1 + dot);
        return System.Numerics.Quaternion.Normalize(q);
    }

    private static System.Numerics.Matrix4x4 PlaneToMatrix(Plane p)
    {
        var origin = new System.Numerics.Vector3((float)p.Origin.X, (float)p.Origin.Y, (float)p.Origin.Z);
        var x = System.Numerics.Vector3.Normalize(new System.Numerics.Vector3((float)p.XAxis.X, (float)p.XAxis.Y, (float)p.XAxis.Z));
        var yRaw = new System.Numerics.Vector3((float)p.YAxis.X, (float)p.YAxis.Y, (float)p.YAxis.Z);

        var z = System.Numerics.Vector3.Normalize(System.Numerics.Vector3.Cross(x, yRaw));
        var y = System.Numerics.Vector3.Normalize(System.Numerics.Vector3.Cross(z, x));

        return new System.Numerics.Matrix4x4(
            x.X, y.X, z.X, origin.X,
            x.Y, y.Y, z.Y, origin.Y,
            x.Z, y.Z, z.Z, origin.Z,
            0, 0, 0, 1
        );
    }

    private static System.Numerics.Vector3 ApplyMatrix4ToVec3(System.Numerics.Matrix4x4 m, System.Numerics.Vector3 v)
    {
        return new System.Numerics.Vector3(
            m.M11 * v.X + m.M12 * v.Y + m.M13 * v.Z,
            m.M21 * v.X + m.M22 * v.Y + m.M23 * v.Z,
            m.M31 * v.X + m.M32 * v.Y + m.M33 * v.Z
        );
    }

    private static System.Numerics.Matrix4x4 QuaternionToMatrix(System.Numerics.Quaternion q)
    {
        float x = q.X, y = q.Y, z = q.Z, w = q.W;
        float xx = x * x, yy = y * y, zz = z * z;
        float xy = x * y, xz = x * z, yz = y * z;
        float wx = w * x, wy = w * y, wz = w * z;

        return new System.Numerics.Matrix4x4(
            1 - 2 * (yy + zz), 2 * (xy - wz), 2 * (xz + wy), 0,
            2 * (xy + wz), 1 - 2 * (xx + zz), 2 * (yz - wx), 0,
            2 * (xz - wy), 2 * (yz + wx), 1 - 2 * (xx + yy), 0,
            0, 0, 0, 1
        );
    }

    private static System.Numerics.Matrix4x4 MakeTranslation(float x, float y, float z)
    {
        return new System.Numerics.Matrix4x4(
            1, 0, 0, x,
            0, 1, 0, y,
            0, 0, 1, z,
            0, 0, 0, 1
        );
    }

    private static System.Numerics.Matrix4x4 MakeRotationAxis(System.Numerics.Vector3 axis, float angle)
    {
        float c = (float)Math.Cos(angle);
        float s = (float)Math.Sin(angle);
        float t = 1 - c;
        float x = axis.X, y = axis.Y, z = axis.Z;

        return new System.Numerics.Matrix4x4(
            t * x * x + c, t * x * y - s * z, t * x * z + s * y, 0,
            t * x * y + s * z, t * y * y + c, t * y * z - s * x, 0,
            t * x * z - s * y, t * y * z + s * x, t * z * z + c, 0,
            0, 0, 0, 1
        );
    }

    private static System.Numerics.Matrix4x4 MultiplyMatrices(System.Numerics.Matrix4x4 a, System.Numerics.Matrix4x4 b)
    {
        return new System.Numerics.Matrix4x4(
            a.M11 * b.M11 + a.M12 * b.M21 + a.M13 * b.M31 + a.M14 * b.M41,
            a.M11 * b.M12 + a.M12 * b.M22 + a.M13 * b.M32 + a.M14 * b.M42,
            a.M11 * b.M13 + a.M12 * b.M23 + a.M13 * b.M33 + a.M14 * b.M43,
            a.M11 * b.M14 + a.M12 * b.M24 + a.M13 * b.M34 + a.M14 * b.M44,

            a.M21 * b.M11 + a.M22 * b.M21 + a.M23 * b.M31 + a.M24 * b.M41,
            a.M21 * b.M12 + a.M22 * b.M22 + a.M23 * b.M32 + a.M24 * b.M42,
            a.M21 * b.M13 + a.M22 * b.M23 + a.M23 * b.M33 + a.M24 * b.M43,
            a.M21 * b.M14 + a.M22 * b.M24 + a.M23 * b.M34 + a.M24 * b.M44,

            a.M31 * b.M11 + a.M32 * b.M21 + a.M33 * b.M31 + a.M34 * b.M41,
            a.M31 * b.M12 + a.M32 * b.M22 + a.M33 * b.M32 + a.M34 * b.M42,
            a.M31 * b.M13 + a.M32 * b.M23 + a.M33 * b.M33 + a.M34 * b.M43,
            a.M31 * b.M14 + a.M32 * b.M24 + a.M33 * b.M34 + a.M34 * b.M44,

            a.M41 * b.M11 + a.M42 * b.M21 + a.M43 * b.M31 + a.M44 * b.M41,
            a.M41 * b.M12 + a.M42 * b.M22 + a.M43 * b.M32 + a.M44 * b.M42,
            a.M41 * b.M13 + a.M42 * b.M23 + a.M43 * b.M33 + a.M44 * b.M43,
            a.M41 * b.M14 + a.M42 * b.M24 + a.M43 * b.M34 + a.M44 * b.M44
        );
    }

    private static Plane MatrixToPlane(System.Numerics.Matrix4x4 m)
    {
        var x = new System.Numerics.Vector3(m.M11, m.M21, m.M31);
        var y = new System.Numerics.Vector3(m.M12, m.M22, m.M32);
        var origin = new System.Numerics.Vector3(m.M14, m.M24, m.M34);

        return new Plane
        {
            Origin = new Point { X = origin.X, Y = origin.Y, Z = origin.Z },
            XAxis = new Vector { X = x.X, Y = x.Y, Z = x.Z },
            YAxis = new Vector { X = y.X, Y = y.Y, Z = y.Z }
        };
    }

    public static Design Sort(Design design)
    {
        var sortedPieces = new List<Piece>();
        var sortedConnections = new List<Connection>();

        Bfs(design,
            piece => { sortedPieces.Add(piece); },
            (parent, child, connection) =>
            {
                sortedPieces.Add(child);
                Connection sortedConnection;
                if (connection.Connected.Piece.Guid != parent.Guid)
                {
                    sortedConnection = new Connection
                    {
                        Guid = connection.Guid,
                        Connected = new Side { Piece = new PieceId { Guid = child.Guid }, Connector = connection.Connected.Connector },
                        Connecting = new Side { Piece = new PieceId { Guid = parent.Guid }, Connector = connection.Connecting.Connector },
                        Description = connection.Description,
                        Gap = connection.Gap,
                        Shift = connection.Shift,
                        Rise = connection.Rise,
                        Rotation = connection.Rotation,
                        Turn = connection.Turn,
                        Tilt = connection.Tilt,
                        U = connection.U,
                        V = connection.V,
                        Attributes = connection.Attributes
                    };
                }
                else
                {
                    sortedConnection = connection;
                }
                sortedConnections.Add(sortedConnection);
            });

        return new Design
        {
            Guid = design.Guid,
            Name = design.Name,
            Parent = design.Parent,
            IsAbstract = design.IsAbstract,
            Folder = design.Folder,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            ActiveLayer = design.ActiveLayer,
            CanScale = design.CanScale,
            CanMirror = design.CanMirror,
            Pieces = sortedPieces,
            Connections = sortedConnections,
            Props = design.Props,
            Stats = design.Stats,
            Layers = design.Layers,
            Groups = design.Groups,
            Authors = design.Authors,
            Concepts = design.Concepts,
            Attributes = design.Attributes,
            CreatedAt = design.CreatedAt,
            UpdatedAt = design.UpdatedAt
        };
    }

    public static Piece? GetPiece(Design design, string guid) => design.Pieces.Find(piece => piece.Guid == guid);
    private static Design FlatToSvgCoordinates(Design design, float iconWidth, float iconWidthMax, float margin)
    {
        var newPieces = design.Pieces.Select(piece =>
        {
            if (piece.Center is null) return piece;
            var newPiece = Entity<Piece>.DeepClone(piece)!;
            newPiece.Center = new Coord
            {
                U = piece.Center.U * iconWidth,
                V = -(piece.Center.V * iconWidth)
            };
            return newPiece;
        }).ToList();

        var newConnections = design.Connections.Select(connection =>
        {
            if (!connection.U.HasValue && !connection.V.HasValue) return connection;
            var newConn = Entity<Connection>.DeepClone(connection)!;
            if (connection.U.HasValue) newConn.U = connection.U * iconWidth;
            if (connection.V.HasValue) newConn.V = -(connection.V * iconWidth);
            return newConn;
        }).ToList();

        var maxIconOffset = iconWidthMax - iconWidth;
        var minX = newPieces.Where(p => p.Center is not null).Min(piece => piece.Center!.U) - (margin + maxIconOffset);
        var minY = newPieces.Where(p => p.Center is not null).Min(piece => piece.Center!.V) - (margin + maxIconOffset);
        var minXSign = Math.Sign(minX);
        var minYSign = Math.Sign(minY);
        var offsetX = minXSign == 0 ? 0 : -minX;
        var offsetY = minYSign == 0 ? 0 : -minY;

        newPieces = newPieces.Select(piece =>
        {
            if (piece.Center is null) return piece;
            var newPiece = Entity<Piece>.DeepClone(piece)!;
            newPiece.Center = new Coord { U = piece.Center.U + offsetX, V = piece.Center.V + offsetY };
            return newPiece;
        }).ToList();

        return new Design
        {
            Guid = design.Guid,
            Name = design.Name,
            Parent = design.Parent,
            IsAbstract = design.IsAbstract,
            Folder = design.Folder,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            ActiveLayer = design.ActiveLayer,
            CanScale = design.CanScale,
            CanMirror = design.CanMirror,
            Pieces = newPieces,
            Connections = newConnections,
            Props = design.Props,
            Stats = design.Stats,
            Layers = design.Layers,
            Groups = design.Groups,
            Authors = design.Authors,
            Concepts = design.Concepts,
            Attributes = design.Attributes,
            CreatedAt = design.CreatedAt,
            UpdatedAt = design.UpdatedAt
        };
    }

    public static string Diagram(
        Design design,
        IEnumerable<Type> types,
        Func<Plane, Point, Vector, Point, Vector, double, double, double, double, double, double, Plane> computeChildPlane,
        string kitDirectory = "",
        float iconWidth = 48, float iconStroke = 1f, float connectionStroke = 2f, float margin = 0)
    {
        var typesDict = Type.EnumerableToDict(types);

        var usedTypes = new List<Type>();
        foreach (var type in types)
            if (design.Pieces.Exists(piece => piece.Type is not null && piece.Type.Guid == type.Guid))
                usedTypes.Add(type);

        var flatCloneInSvgCoordinates = FlatToSvgCoordinates(Flatten(Entity<Design>.DeepClone(design)!, types, computeChildPlane), iconWidth, iconWidth + 2 * iconStroke, margin);

        var svgDoc = new SvgDocument
        {
            Width = (float)flatCloneInSvgCoordinates.Pieces.Where(p => p.Center is not null).Max(piece => piece.Center!.U) + margin * 2 + iconWidth +
                    2 * iconStroke,
            Height = (float)flatCloneInSvgCoordinates.Pieces.Where(p => p.Center is not null).Max(piece => piece.Center!.V) + margin * 2 + iconWidth +
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

                var fontSize = iconWidth / 2;
                var text = new SvgText
                {
                    Text = icon,
                    FontSize = fontSize,
                    TextAnchor = SvgTextAnchor.Middle,
                    Fill = new SvgColourServer(Color.Black),

                    CustomAttributes =
                    {

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

        foreach (var connection in design.Connections)
        {
            var connectedPieceFlat = GetPiece(flatCloneInSvgCoordinates, connection.Connected.Piece.Guid);
            var connectingPieceFlat = GetPiece(flatCloneInSvgCoordinates, connection.Connecting.Piece.Guid);
            if (connectedPieceFlat?.Center is null || connectingPieceFlat?.Center is null) continue;
            var connectionLine = new SvgLine
            {
                StartX = (float)connectedPieceFlat.Center.U + iconWidth / 2,
                StartY = (float)connectedPieceFlat.Center.V + iconWidth / 2,
                EndX = (float)connectingPieceFlat.Center.U + iconWidth / 2,
                EndY = (float)connectingPieceFlat.Center.V + iconWidth / 2,
                Stroke = new SvgColourServer(Color.Black),
                StrokeWidth = connectionStroke,
                Children = { new SvgTitle { Content = connection.ToIdString() } }
            };
            connections.Children.Add(connectionLine);
        }

        svgDoc.Children.Add(connections);

        var pieces = new SvgGroup { ID = "pieces" };

        foreach (var piece in design.Pieces)
        {
            var flatPiece = GetPiece(flatCloneInSvgCoordinates, piece.Guid);
            if (piece.Center is not null && flatPiece?.Center is not null)
            {
                var rootPiece = new SvgUse
                {
                    CustomAttributes = { { "href", "#root" } },
                    X = (float)flatPiece.Center.U,
                    Y = (float)flatPiece.Center.V
                };
                pieces.Children.Add(rootPiece);
            }

            var pieceType = flatPiece?.Type is not null ? types.FirstOrDefault(t => t.Guid == flatPiece.Type.Guid) : null;
            if (pieceType is not null && flatPiece?.Center is not null)
            {
                var pieceIcon = new SvgUse
                {
                    CustomAttributes =
                        { { "href", "#" + typesDict[pieceType.Name].ToIdString() } },
                    X = (float)flatPiece.Center.U,
                    Y = (float)flatPiece.Center.V,
                    Children = { new SvgTitle { Content = flatPiece.Guid } }
                };
                pieces.Children.Add(pieceIcon);
            }
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
        if (xml.DocumentElement is null) throw new InvalidOperationException("XML document has no root element");
        xml.DocumentElement.PrependChild(styleElement);
        return xml.OuterXml.Replace(" xmlns=\"\"", "");
    }

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

        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }

        var pieceIds = Pieces.Select(p => p.Guid);
        var duplicatePieceIds = pieceIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicatePieceIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicatePieceId in duplicatePieceIds)
                errors.Add($"A piece is invalid: There are multiple pieces with guid ({duplicatePieceId}).");
        }

        var nonExistingConnectedPieces = Connections.Where(c => !pieceIds.Contains(c.Connected.Piece.Guid)).ToList()
            .Select(c => c.Connected.Piece.Guid).ToArray();
        if (nonExistingConnectedPieces.Length != 0)
        {
            isValid = false;
            foreach (var nonExistingConnectedPiece in nonExistingConnectedPieces)
            {
                var connection = Connections.First(c => c.Connected.Piece.Guid == nonExistingConnectedPiece);
                errors.Add(
                    $"A connection({connection.ToHumanIdString()}) is invalid: The referenced connected piece ({nonExistingConnectedPiece}) is not part of the design.");
            }
        }

        var nonExistingConnectingPieces = Connections.Where(c => !pieceIds.Contains(c.Connecting.Piece.Guid)).ToList()
            .Select(c => c.Connecting.Piece.Guid).ToArray();
        if (nonExistingConnectingPieces.Length != 0)
        {
            isValid = false;
            foreach (var nonExistingConnectingPiece in nonExistingConnectingPieces)
            {
                var connection = Connections.First(c => c.Connecting.Piece.Guid == nonExistingConnectingPiece);
                errors.Add(
                    $"A connection({connection.ToHumanIdString()}) is invalid: The referenced connecting piece ({nonExistingConnectingPiece}) is not part of the design.");
            }
        }

        var connectionKeys = Connections
            .Select(c => (
                ConnectedPieceId: c.Connected.Piece.Guid,
                ConnectedDesignPieceId: c.Connected.DesignPiece?.Guid ?? "",
                ConnectingPieceId: c.Connecting.Piece.Guid,
                ConnectingDesignPieceId: c.Connecting.DesignPiece?.Guid ?? ""))
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

    public static bool IsSameAs(Design design, Design other)
    {
        if (other is null) return false;
        return design.Name == other.Name;
    }

    public static Piece FindPiece(Design design, string pieceGuid)
    {
        var piece = design.Pieces.FirstOrDefault(p => p.Guid == pieceGuid);
        if (piece is null) throw new ArgumentException($"Piece {pieceGuid} not found in design");
        return piece;
    }

    public static Connection FindConnection(Design design, Connection connectionToFind, bool strict = false)
    {
        var connection = design.Connections.FirstOrDefault(c => Connection.IsSameAs(c, connectionToFind, strict));
        if (connection is null)
            throw new ArgumentException($"Connection {connectionToFind.Connected.Piece.Guid} -> {connectionToFind.Connecting.Piece.Guid} not found in design");
        return connection;
    }

    public static List<Connection> FindPieceConnections(Design design, string pieceGuid)
    {
        return design.Connections.Where(c =>
            c.Connected.Piece.Guid == pieceGuid ||
            c.Connecting.Piece.Guid == pieceGuid).ToList();
    }

    public static Design AddPiece(Design design, Piece piece)
    {
        var newPieces = new List<Piece>(design.Pieces) { piece };
        return new Design
        {
            Guid = design.Guid,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = newPieces,
            Connections = new List<Connection>(design.Connections),
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = new List<Attribute>(design.Attributes)
        };
    }

    public static Design RemovePiece(Design design, string pieceGuid)
    {
        var newPieces = design.Pieces.Where(p => p.Guid != pieceGuid).ToList();
        var newConnections = design.Connections.Where(c =>
            c.Connected.Piece.Guid != pieceGuid &&
            c.Connecting.Piece.Guid != pieceGuid).ToList();
        return new Design
        {
            Guid = design.Guid,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = newPieces,
            Connections = newConnections,
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = new List<Attribute>(design.Attributes)
        };
    }

    public static Design AddConnection(Design design, Connection connection)
    {
        var newConnections = new List<Connection>(design.Connections) { connection };
        return new Design
        {
            Guid = design.Guid,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = new List<Piece>(design.Pieces),
            Connections = newConnections,
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = new List<Attribute>(design.Attributes)
        };
    }

    public static Design RemoveConnection(Design design, Connection connectionToRemove)
    {
        var newConnections = design.Connections.Where(c => !Connection.IsSameAs(c, connectionToRemove)).ToList();
        return new Design
        {
            Guid = design.Guid,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = new List<Piece>(design.Pieces),
            Connections = newConnections,
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = new List<Attribute>(design.Attributes)
        };
    }

    public static string FindAttributeValue(Design design, string key, string defaultValue = "")
    {
        var attribute = design.Attributes.FirstOrDefault(a => a.Key == key);
        return attribute?.Value ?? defaultValue;
    }

    public static Design SetAttribute(Design design, Attribute attribute)
    {
        var newAttributes = design.Attributes.Where(a => a.Key != attribute.Key).ToList();
        newAttributes.Add(attribute);
        return new Design
        {
            Guid = design.Guid,
            Name = design.Name,
            Description = design.Description,
            Icon = design.Icon,
            Image = design.Image,
            Location = design.Location,
            Unit = design.Unit,
            Pieces = new List<Piece>(design.Pieces),
            Connections = new List<Connection>(design.Connections),
            Props = new List<Prop>(design.Props),
            Stats = new List<Stat>(design.Stats),
            Authors = new List<AuthorId>(design.Authors),
            Attributes = newAttributes
        };
    }

    public static DesignDiff DragPiecesInDesign(Design design, Design pieces, Coord offset)
    {
        var designConnections = design.Connections;
        var selectedPieces = pieces.Pieces;
        var selectedGuids = new HashSet<string>(selectedPieces.Select(p => p.Guid));
        var connectionByChild = new Dictionary<string, Connection>();
        foreach (var conn in designConnections)
        {
            connectionByChild[conn.Connecting.Piece.Guid] = conn;
        }
        var fixedGuids = new HashSet<string>();
        foreach (var guid in selectedGuids)
        {
            if (!connectionByChild.ContainsKey(guid))
                fixedGuids.Add(guid);
        }
        var pieceMap = design.Pieces.ToDictionary(p => p.Guid);
        var pieceUpdates = new List<PieceDiffUpdate>();
        foreach (var guid in fixedGuids)
        {
            if (pieceMap.TryGetValue(guid, out var piece) && piece.Center != null)
            {
                pieceUpdates.Add(new PieceDiffUpdate
                {
                    Piece = new PieceId { Guid = guid },
                    Diff = new PieceDiff { Center = new Coord { U = piece.Center.U + offset.U, V = piece.Center.V + offset.V } },
                });
            }
        }
        var connectionUpdates = new List<ConnectionDiffUpdate>();
        foreach (var guid in selectedGuids)
        {
            if (fixedGuids.Contains(guid)) continue;
            var isDescendant = false;
            var current = guid;
            while (connectionByChild.TryGetValue(current, out var conn))
            {
                var parentGuid = conn.Connected.Piece.Guid;
                if (selectedGuids.Contains(parentGuid))
                {
                    isDescendant = true;
                    break;
                }
                current = parentGuid;
            }
            if (isDescendant) continue;
            if (connectionByChild.TryGetValue(guid, out var parentConn))
            {
                connectionUpdates.Add(new ConnectionDiffUpdate
                {
                    Connection = new ConnectionId { Guid = parentConn.Guid },
                    Diff = new ConnectionDiff { U = offset.U, V = offset.V },
                });
            }
        }
        var diff = new DesignDiff();
        if (pieceUpdates.Count > 0)
            diff.Pieces = new PiecesDiff { Updated = pieceUpdates };
        if (connectionUpdates.Count > 0)
            diff.Connections = new ConnectionsDiff { Updated = connectionUpdates };
        return diff;
    }

    private static Point MoveTranslationWorldFromPiecePlane(Plane plane, MoveVector vector)
    {
        var xRaw = new System.Numerics.Vector3((float)plane.XAxis.X, (float)plane.XAxis.Y, (float)plane.XAxis.Z);
        if (xRaw.LengthSquared() < 1e-12f) return new Point();
        var x = System.Numerics.Vector3.Normalize(xRaw);
        var yRaw = new System.Numerics.Vector3((float)plane.YAxis.X, (float)plane.YAxis.Y, (float)plane.YAxis.Z);
        if (yRaw.LengthSquared() < 1e-12f) return new Point();
        var y = System.Numerics.Vector3.Normalize(yRaw);
        var z = System.Numerics.Vector3.Cross(x, y);
        if (z.LengthSquared() < 1e-12f) return new Point();
        z = System.Numerics.Vector3.Normalize(z);
        var t = x * (float)vector.Shift + y * (float)vector.Gap + z * (float)vector.Rise;
        return new Point { X = t.X, Y = t.Y, Z = t.Z };
    }

    public static DesignDiff MovePiecesInDesign(Design design, Design pieces, MoveVector vector)
    {
        var designConnections = design.Connections;
        var selectedPieces = pieces.Pieces;
        var selectedGuids = new HashSet<string>(selectedPieces.Select(p => p.Guid));
        var connectionByChild = new Dictionary<string, Connection>();
        foreach (var conn in designConnections)
        {
            connectionByChild[conn.Connecting.Piece.Guid] = conn;
        }
        var fixedGuids = new HashSet<string>();
        foreach (var guid in selectedGuids)
        {
            if (!connectionByChild.ContainsKey(guid))
                fixedGuids.Add(guid);
        }
        var pieceMap = design.Pieces.ToDictionary(p => p.Guid);
        var pieceUpdates = new List<PieceDiffUpdate>();
        foreach (var guid in fixedGuids)
        {
            if (!pieceMap.TryGetValue(guid, out var piece) || piece.Plane == null) continue;
            var basePlane = piece.Plane;
            var t = MoveTranslationWorldFromPiecePlane(basePlane, vector);
            pieceUpdates.Add(new PieceDiffUpdate
            {
                Piece = new PieceId { Guid = guid },
                Diff = new PieceDiff
                {
                    Plane = new Plane
                    {
                        Origin = new Point
                        {
                            X = basePlane.Origin.X + t.X,
                            Y = basePlane.Origin.Y + t.Y,
                            Z = basePlane.Origin.Z + t.Z,
                        },
                        XAxis = new Vector { X = basePlane.XAxis.X, Y = basePlane.XAxis.Y, Z = basePlane.XAxis.Z },
                        YAxis = new Vector { X = basePlane.YAxis.X, Y = basePlane.YAxis.Y, Z = basePlane.YAxis.Z },
                    },
                },
            });
        }
        var connectionUpdates = new List<ConnectionDiffUpdate>();
        foreach (var guid in selectedGuids)
        {
            if (fixedGuids.Contains(guid)) continue;
            var isDescendant = false;
            var current = guid;
            while (connectionByChild.TryGetValue(current, out var conn))
            {
                var parentGuid = conn.Connected.Piece.Guid;
                if (selectedGuids.Contains(parentGuid))
                {
                    isDescendant = true;
                    break;
                }
                current = parentGuid;
            }
            if (isDescendant) continue;
            if (connectionByChild.TryGetValue(guid, out var parentConn))
            {
                connectionUpdates.Add(new ConnectionDiffUpdate
                {
                    Connection = new ConnectionId { Guid = parentConn.Guid },
                    Diff = new ConnectionDiff { Gap = vector.Gap, Shift = vector.Shift, Rise = vector.Rise },
                });
            }
        }
        var diff = new DesignDiff();
        if (pieceUpdates.Count > 0)
            diff.Pieces = new PiecesDiff { Updated = pieceUpdates };
        if (connectionUpdates.Count > 0)
            diff.Connections = new ConnectionsDiff { Updated = connectionUpdates };
        return diff;
    }

    /// <summary>
    /// Deletes pieces and connections from a design, returning a DesignDiff.
    /// Removes stale connections referencing deleted pieces.
    /// Updates pieces that become fixed (parent connection removed) with flat plane and center from the flattened design.
    /// </summary>
    public static DesignDiff DeletePiecesAndConnectionsInDesign(Kit kit, Design design, List<string> pieceGuids, List<string> connectionGuids)
    {
        var deletedPieceSet = new HashSet<string>(pieceGuids);

        // Find stale connections: connections referencing any deleted piece
        var staleConnectionGuids = new HashSet<string>();
        foreach (var conn in design.Connections)
        {
            if (deletedPieceSet.Contains(conn.Connected.Piece.Guid) ||
                deletedPieceSet.Contains(conn.Connecting.Piece.Guid))
            {
                staleConnectionGuids.Add(conn.Guid);
            }
        }

        // All removed connections = explicit + stale
        var allRemovedConnectionGuids = new HashSet<string>(connectionGuids);
        allRemovedConnectionGuids.UnionWith(staleConnectionGuids);

        // Find pieces that become fixed: pieces whose parent connection was removed
        // and are not themselves being deleted
        // A piece becomes fixed when the connection where it is the "connecting" side is removed
        // and it has no other remaining parent connection
        var fixedPieceGuids = new List<string>();
        foreach (var connGuid in allRemovedConnectionGuids)
        {
            var conn = design.Connections.FirstOrDefault(c => c.Guid == connGuid);
            if (conn == null) continue;
            var connectingGuid = conn.Connecting.Piece.Guid;
            if (deletedPieceSet.Contains(connectingGuid)) continue;
            // Check if this piece has another parent connection not in the removed set
            var hasOtherParent = design.Connections.Any(c =>
                c.Connecting.Piece.Guid == connectingGuid &&
                !allRemovedConnectionGuids.Contains(c.Guid));
            if (!hasOtherParent && !fixedPieceGuids.Contains(connectingGuid))
                fixedPieceGuids.Add(connectingGuid);
        }

        // Build the diff
        var piecesRemoved = pieceGuids.Select(g => new PieceId { Guid = g }).ToList();

        // Flatten the design to get absolute plane and center for each piece
        var flatResult = Kit.FlattenDesign(kit, design.Guid);
        var flatPieceMap = new Dictionary<string, (Plane? Plane, Coord? Center)>();
        foreach (var piece in design.Pieces)
        {
            if (piece.Plane != null)
                flatPieceMap[piece.Guid] = (piece.Plane, piece.Center);
        }
        if (flatResult.Pieces?.Updated != null)
        {
            foreach (var update in flatResult.Pieces.Updated)
            {
                var existing = flatPieceMap.ContainsKey(update.Piece.Guid)
                    ? flatPieceMap[update.Piece.Guid]
                    : ((Plane?)null, (Coord?)null);
                if (update.Diff?.Plane != null) existing.Item1 = update.Diff.Plane;
                if (update.Diff?.Center != null) existing.Item2 = update.Diff.Center;
                flatPieceMap[update.Piece.Guid] = existing;
            }
        }

        var piecesUpdated = fixedPieceGuids.Select(g =>
        {
            var flat = flatPieceMap.ContainsKey(g) ? flatPieceMap[g] : ((Plane?)null, (Coord?)null);
            return new PieceDiffUpdate
            {
                Piece = new PieceId { Guid = g },
                Diff = new PieceDiff
                {
                    Plane = flat.Item1 ?? new Plane(),
                    Center = flat.Item2 ?? new Coord()
                }
            };
        }).ToList();
        var connectionsRemoved = allRemovedConnectionGuids
            .OrderBy(g => g)
            .Select(g => new ConnectionId { Guid = g })
            .ToList();

        var diff = new DesignDiff();
        if (piecesRemoved.Count > 0 || piecesUpdated.Count > 0)
            diff.Pieces = new PiecesDiff { Removed = piecesRemoved, Updated = piecesUpdated };
        if (connectionsRemoved.Count > 0)
            diff.Connections = new ConnectionsDiff { Removed = connectionsRemoved };
        return diff;
    }

    /// <summary>
    /// 📋Copies a selection of pieces and connections from a design into a standalone design.
    /// </summary>
    /// <remarks>
    /// Specs: A selection is a set of piece guids and connection guids.
    /// - Fixed selected pieces are added as-is.
    /// - Internal-connected pieces (selected, parent piece selected, parent connection selected) are added as-is.
    /// - Parent-piece-exclusive parent-connection-inclusive pieces get semio.center and semio.plane attributes from the flat design.
    /// - Orphaned connections, parent-exclusive child-inclusive connections, and parent-inclusive child-exclusive connections
    ///   are added with their external pieces marked with semio.piece.origin = "external".
    /// - Internal connections are added as-is.
    /// </remarks>
    public static Design CopyDesign(Kit kit, Design design, List<string> pieceGuids, List<string> connectionGuids)
    {
        var selectedPieceSet = new HashSet<string>(pieceGuids);
        var selectedConnectionSet = new HashSet<string>(connectionGuids);

        // Build parent map: child guid -> (parent guid, connection)
        var parentMap = new Dictionary<string, (string parentGuid, Connection connection)>();
        foreach (var conn in design.Connections)
        {
            parentMap[conn.Connecting.Piece.Guid] = (conn.Connected.Piece.Guid, conn);
        }

        // Flatten the design to get absolute planes/centers
        var flatDiff = Kit.FlattenDesign(kit, design.Guid);
        var flatDesign = Design.ApplyDiff(Entity<Design>.DeepClone(design)!, flatDiff);
        var flatPieceMap = flatDesign.Pieces.ToDictionary(p => p.Guid);

        var copyPieces = new List<Piece>();
        var addedPieceGuids = new HashSet<string>();
        var copyConnections = new List<Connection>();

        // Process selected pieces
        foreach (var pieceGuid in pieceGuids)
        {
            var piece = design.Pieces.First(p => p.Guid == pieceGuid);
            var isFixed = piece.Plane is not null;
            var isConnected = parentMap.ContainsKey(pieceGuid);

            bool isInternalConnected = false;
            bool isInternalFixed = isFixed && selectedPieceSet.Contains(pieceGuid);
            bool isPpExclPcIncl = false;

            if (isConnected)
            {
                var (parentGuid, parentConn) = parentMap[pieceGuid];
                var parentPieceSelected = selectedPieceSet.Contains(parentGuid);
                var parentConnSelected = selectedConnectionSet.Contains(parentConn.Guid);
                isInternalConnected = parentPieceSelected && parentConnSelected;
                isPpExclPcIncl = !parentPieceSelected && parentConnSelected;
            }

            var isInternal = isInternalConnected || isInternalFixed;

            if (isInternalFixed || isInternalConnected)
            {
                copyPieces.Add(Entity<Piece>.DeepClone(piece)!);
                addedPieceGuids.Add(pieceGuid);
            }
            else if (isPpExclPcIncl)
            {
                var copied = Entity<Piece>.DeepClone(piece)!;
                // Add semio.center and semio.plane from flattened design
                if (flatPieceMap.TryGetValue(pieceGuid, out var flatPiece))
                {
                    var centerValue = flatPiece.Center is not null
                        ? Utility.Serialize(flatPiece.Center)
                        : Utility.Serialize(new Coord());
                    var planeValue = flatPiece.Plane is not null
                        ? Utility.Serialize(flatPiece.Plane)
                        : Utility.Serialize(new Plane());
                    copied.Attributes = new List<Attribute>(copied.Attributes)
                    {
                        new() { Key = "semio.center", Value = centerValue },
                        new() { Key = "semio.plane", Value = planeValue }
                    };
                }
                copyPieces.Add(copied);
                addedPieceGuids.Add(pieceGuid);
            }
        }

        // Process selected connections
        foreach (var connGuid in connectionGuids)
        {
            var conn = design.Connections.First(c => c.Guid == connGuid);
            var connectedGuid = conn.Connected.Piece.Guid;
            var connectingGuid = conn.Connecting.Piece.Guid;
            var connectedSelected = selectedPieceSet.Contains(connectedGuid);
            var connectingSelected = selectedPieceSet.Contains(connectingGuid);

            var isInternal = connectedSelected && connectingSelected;
            var isOrphaned = !connectedSelected && !connectingSelected;
            var isParentExclChildIncl = !connectedSelected && connectingSelected;
            var isParentInclChildExcl = connectedSelected && !connectingSelected;

            if (isInternal)
            {
                copyConnections.Add(Entity<Connection>.DeepClone(conn)!);
            }
            else if (isOrphaned || isParentExclChildIncl || isParentInclChildExcl)
            {
                copyConnections.Add(Entity<Connection>.DeepClone(conn)!);

                // Add external pieces
                var externalGuids = new List<string>();
                if (!connectedSelected) externalGuids.Add(connectedGuid);
                if (!connectingSelected) externalGuids.Add(connectingGuid);

                foreach (var extGuid in externalGuids)
                {
                    if (!addedPieceGuids.Contains(extGuid))
                    {
                        var extPiece = Entity<Piece>.DeepClone(design.Pieces.First(p => p.Guid == extGuid))!;
                        var extAttrs = new List<Attribute>(extPiece.Attributes)
                        {
                            new() { Key = "semio.piece.origin", Value = "external" }
                        };
                        if (flatPieceMap.TryGetValue(extGuid, out var flatExtPiece))
                        {
                            var extCenterValue = flatExtPiece.Center is not null
                                ? Utility.Serialize(flatExtPiece.Center)
                                : Utility.Serialize(new Coord());
                            extAttrs.Add(new Attribute { Key = "semio.center", Value = extCenterValue });
                        }
                        extPiece.Attributes = extAttrs;
                        copyPieces.Add(extPiece);
                        addedPieceGuids.Add(extGuid);
                    }
                }
            }
        }

        return new Design
        {
            Guid = "",
            Name = "",
            Pieces = copyPieces,
            Connections = copyConnections
        };
    }

    /// <summary>
    /// 📋Pastes a copied design into a target design, returning a DesignDiff.
    /// </summary>
    /// <remarks>
    /// Specs: Anchoring determines the reference point within the bounding rectangle of the source.
    /// - Fixed pieces get -anchor offset applied to center; if coord is given, +coord offset is also applied.
    /// - Connected pieces with non-external parents are added as-is.
    /// - Connected pieces with external-origin parents: if a matching piece with a matching connector is found in target,
    ///   the parent connection is remapped; otherwise treated as fixed using semio.center/semio.plane attributes.
    /// - Internal connections (neither piece is external) are added as-is.
    /// - Orphaned connections and external-origin pieces are not added.
    /// </remarks>
    public static DesignDiff PasteDesign(
        Kit kit,
        Design source,
        Design target,
        string anchoring = "bottomLeft",
        Coord? coord = null)
    {
        var types = (kit.Types ?? new List<Type>()).ToDictionary(t => t.Guid);
        var ports = (kit.Ports ?? new List<Port>()).ToDictionary(p => p.Guid);

        // Classify source pieces
        var externalOriginGuids = new HashSet<string>();
        foreach (var piece in source.Pieces)
        {
            if (piece.Attributes.Any(a => a.Key == "semio.piece.origin" && a.Value == "external"))
                externalOriginGuids.Add(piece.Guid);
        }

        var sourcePieceMap = source.Pieces.ToDictionary(p => p.Guid);
        var sourceParentMap = new Dictionary<string, (string parentGuid, Connection connection)>();
        foreach (var conn in source.Connections)
        {
            var childGuid = conn.Connecting.Piece.Guid;
            var parentGuid = conn.Connected.Piece.Guid;
            if (!sourceParentMap.TryGetValue(childGuid, out var prev))
            {
                sourceParentMap[childGuid] = (parentGuid, conn);
                continue;
            }
            var prevStub = externalOriginGuids.Contains(prev.parentGuid);
            var nextStub = externalOriginGuids.Contains(parentGuid);
            if (prevStub != nextStub && nextStub)
                sourceParentMap[childGuid] = (parentGuid, conn);
        }

        // Compute flat planes/centers for source pieces that need it
        // For pp_excl_pc_incl pieces, the semio.center and semio.plane attributes have the flat values

        // Compute bounding rectangle from flat centers
        var centerCoords = new List<Coord>();
        foreach (var piece in source.Pieces)
        {
            if (externalOriginGuids.Contains(piece.Guid)) continue;

            Coord? center = piece.Center;
            if (center is null)
            {
                // Try to get from semio.center attribute
                var centerAttr = piece.Attributes.FirstOrDefault(a => a.Key == "semio.center");
                if (centerAttr?.Value is not null)
                    center = Utility.Deserialize<Coord>(centerAttr.Value);
            }
            if (center is not null)
                centerCoords.Add(center);
        }

        // Also add centers for external pieces referenced by connections
        foreach (var conn in source.Connections)
        {
            var connectedGuid = conn.Connected.Piece.Guid;
            var connectingGuid = conn.Connecting.Piece.Guid;
            if (externalOriginGuids.Contains(connectedGuid) && sourcePieceMap.TryGetValue(connectedGuid, out var extPiece1))
            {
                Coord? c = extPiece1.Center;
                if (c is null)
                {
                    var attr = extPiece1.Attributes.FirstOrDefault(a => a.Key == "semio.center");
                    if (attr?.Value is not null) c = Utility.Deserialize<Coord>(attr.Value);
                }
                if (c is not null) centerCoords.Add(c);
            }
            if (externalOriginGuids.Contains(connectingGuid) && sourcePieceMap.TryGetValue(connectingGuid, out var extPiece2))
            {
                Coord? c = extPiece2.Center;
                if (c is null)
                {
                    var attr = extPiece2.Attributes.FirstOrDefault(a => a.Key == "semio.center");
                    if (attr?.Value is not null) c = Utility.Deserialize<Coord>(attr.Value);
                }
                if (c is not null) centerCoords.Add(c);
            }
        }

        if (centerCoords.Count == 0)
            centerCoords.Add(new Coord());

        var minU = centerCoords.Min(c => c.U);
        var maxU = centerCoords.Max(c => c.U);
        var minV = centerCoords.Min(c => c.V);
        var maxV = centerCoords.Max(c => c.V);

        Coord anchor;
        switch (anchoring)
        {
            case "middle":
                anchor = new Coord { U = (minU + maxU) / 2, V = (minV + maxV) / 2 };
                break;
            case "centroid":
                anchor = new Coord { U = centerCoords.Average(c => c.U), V = centerCoords.Average(c => c.V) };
                break;
            case "bottomLeft":
                anchor = new Coord { U = minU, V = minV };
                break;
            case "bottomRight":
                anchor = new Coord { U = maxU, V = minV };
                break;
            case "topLeft":
                anchor = new Coord { U = minU, V = maxV };
                break;
            case "topRight":
                anchor = new Coord { U = maxU, V = maxV };
                break;
            default: // "original"
                anchor = new Coord { U = 0, V = 0 };
                break;
        }

        // Build target piece maps for matching
        var targetPiecesByName = new Dictionary<string, List<Piece>>();
        foreach (var tp in target.Pieces)
        {
            if (!targetPiecesByName.ContainsKey(tp.Name))
                targetPiecesByName[tp.Name] = new List<Piece>();
            targetPiecesByName[tp.Name].Add(tp);
        }

        // Helper: check port compatibility
        bool ArePortsCompatible(string? portGuid1, string? portGuid2)
        {
            if (portGuid1 is null || portGuid2 is null) return false;
            if (portGuid1 == portGuid2) return true;
            if (!ports.TryGetValue(portGuid1, out var port1) || !ports.TryGetValue(portGuid2, out var port2))
                return false;
            return port1.CompatiblePorts.Any(cp => cp.Guid == portGuid2) ||
                   port2.CompatiblePorts.Any(cp => cp.Guid == portGuid1);
        }

        // Helper: check connector compatibility
        bool AreConnectorsCompatible(Connector c1, Connector c2)
        {
            return ArePortsCompatible(c1.Port?.Guid, c2.Port?.Guid);
        }

        // Helper: find matching connector on a type
        Connector? FindMatchingConnector(string typeGuid, Connector sourceConnector)
        {
            if (!types.TryGetValue(typeGuid, out var type)) return null;
            return type.Connectors.FirstOrDefault(c =>
                c.Name == sourceConnector.Name && AreConnectorsCompatible(c, sourceConnector));
        }

        var addedPieces = new List<Piece>();
        var addedConnections = new List<Connection>();
        var remappedPieces = new Dictionary<string, string>(); // source external guid -> target piece guid

        // Process source pieces
        foreach (var piece in source.Pieces)
        {
            if (externalOriginGuids.Contains(piece.Guid)) continue;

            var isFixed = piece.Plane is not null;
            var isConnected = sourceParentMap.ContainsKey(piece.Guid);

            if (isFixed && !isConnected)
            {
                // Fixed piece: apply -anchor offset, then +coord if given
                var copied = Entity<Piece>.DeepClone(piece)!;
                var center = copied.Center ?? new Coord();
                center = new Coord { U = center.U - anchor.U, V = center.V - anchor.V };
                if (coord is not null)
                    center = new Coord { U = center.U + coord.U, V = center.V + coord.V };
                copied.Center = center;
                addedPieces.Add(copied);
            }
            else if (isConnected)
            {
                var (parentGuid, parentConn) = sourceParentMap[piece.Guid];
                if (externalOriginGuids.Contains(parentGuid))
                {
                    // Parent is external-origin: try to match in target
                    var externalParent = sourcePieceMap[parentGuid];
                    var matched = false;

                    if (targetPiecesByName.TryGetValue(externalParent.Name, out var candidates))
                    {
                        var isParentConnected = parentConn.Connected.Piece.Guid == parentGuid;
                        var parentConnectorGuid = isParentConnected
                            ? parentConn.Connected.Connector.Guid
                            : parentConn.Connecting.Connector.Guid;

                        // Get the external parent's type to find the connector
                        Connector? sourceParentConnector = null;
                        if (externalParent.Type is not null && types.TryGetValue(externalParent.Type.Guid, out var parentType))
                        {
                            sourceParentConnector = parentType.Connectors.FirstOrDefault(c => c.Guid == parentConnectorGuid);
                        }

                        if (sourceParentConnector is not null)
                        {
                            foreach (var candidate in candidates)
                            {
                                if (candidate.Type is null) continue;
                                var matchingConnector = FindMatchingConnector(candidate.Type.Guid, sourceParentConnector);
                                if (matchingConnector is not null)
                                {
                                    // Found a match! Remap the connection
                                    matched = true;
                                    remappedPieces[parentGuid] = candidate.Guid;

                                    var copied = Entity<Piece>.DeepClone(piece)!;
                                    addedPieces.Add(copied);

                                    // Add the remapped connection
                                    var copiedConn = Entity<Connection>.DeepClone(parentConn)!;
                                    if (isParentConnected)
                                    {
                                        copiedConn.Connected = new Side
                                        {
                                            Piece = new PieceId { Guid = candidate.Guid },
                                            Connector = new ConnectorId { Guid = matchingConnector.Guid }
                                        };
                                    }
                                    else
                                    {
                                        copiedConn.Connecting = new Side
                                        {
                                            Piece = new PieceId { Guid = candidate.Guid },
                                            Connector = new ConnectorId { Guid = matchingConnector.Guid }
                                        };
                                    }

                                    if (coord is not null)
                                    {
                                        var connectedStub = externalOriginGuids.Contains(parentConn.Connected.Piece.Guid);
                                        var connectingStub = externalOriginGuids.Contains(parentConn.Connecting.Piece.Guid);
                                        var connMatchesParentage =
                                            (parentConn.Connecting.Piece.Guid == piece.Guid && parentConn.Connected.Piece.Guid == parentGuid) ||
                                            (parentConn.Connected.Piece.Guid == piece.Guid && parentConn.Connecting.Piece.Guid == parentGuid);
                                        // Specs: Coord may shift diagram u/v only for the remapped bridge to a clipboard external stub;
                                        // internal–internal source edges (neither side a stub) must keep cloned u/v.
                                        if (connMatchesParentage && connectedStub != connectingStub)
                                        {
                                            Coord? flatParentCenter = null;
                                            if (flatPiecesByGuid_PassThrough(candidate, out var candCenter))
                                                flatParentCenter = candCenter;
                                            else if (flatPiecesByGuid_PassThrough(externalParent, out var epCenter))
                                                flatParentCenter = epCenter;
                                            Coord? flatChildCenter = null;
                                            var childCenterAttr = piece.Attributes.FirstOrDefault(a => a.Key == "semio.center");
                                            if (childCenterAttr?.Value is not null)
                                                flatChildCenter = Utility.Deserialize<Coord>(childCenterAttr.Value);
                                            if (flatChildCenter is null && piece.Center is not null)
                                                flatChildCenter = piece.Center;

                                            if (flatParentCenter is not null && flatChildCenter is not null)
                                            {
                                                var offsetU = flatParentCenter.U - (coord.U + (anchor.U - flatChildCenter.U));
                                                var offsetV = flatParentCenter.V - (coord.V + (anchor.V - flatChildCenter.V));
                                                copiedConn.U = offsetU;
                                                copiedConn.V = offsetV;
                                            }
                                        }
                                    }

                                    addedConnections.Add(copiedConn);
                                    break;
                                }
                            }
                        }
                    }

                    if (!matched)
                    {
                        // Treat as fixed piece using semio.center and semio.plane attributes
                        var copied = Entity<Piece>.DeepClone(piece)!;
                        var centerAttr = piece.Attributes.FirstOrDefault(a => a.Key == "semio.center");
                        var planeAttr = piece.Attributes.FirstOrDefault(a => a.Key == "semio.plane");
                        if (centerAttr?.Value is not null)
                            copied.Center = Utility.Deserialize<Coord>(centerAttr.Value);
                        if (planeAttr?.Value is not null)
                            copied.Plane = Utility.Deserialize<Plane>(planeAttr.Value);
                        // Apply anchor offset
                        var center = copied.Center ?? new Coord();
                        center = new Coord { U = center.U - anchor.U, V = center.V - anchor.V };
                        if (coord is not null)
                            center = new Coord { U = center.U + coord.U, V = center.V + coord.V };
                        copied.Center = center;
                        addedPieces.Add(copied);
                    }
                }
                else
                {
                    // Parent is not external: add connected piece as-is
                    addedPieces.Add(Entity<Piece>.DeepClone(piece)!);
                }
            }
        }

        // Process source connections (non-external internal connections)
        foreach (var conn in source.Connections)
        {
            var connectedGuid = conn.Connected.Piece.Guid;
            var connectingGuid = conn.Connecting.Piece.Guid;

            // Skip if either piece is external-origin (these are handled during piece processing)
            if (externalOriginGuids.Contains(connectedGuid) || externalOriginGuids.Contains(connectingGuid))
                continue;

            // Skip orphaned connections (both pieces not in source non-external set)
            var connectedIsAdded = addedPieces.Any(p => p.Guid == connectedGuid);
            var connectingIsAdded = addedPieces.Any(p => p.Guid == connectingGuid);
            if (!connectedIsAdded || !connectingIsAdded) continue;

            addedConnections.Add(Entity<Connection>.DeepClone(conn)!);
        }

        // Build DesignDiff
        var diff = new DesignDiff();
        if (addedPieces.Count > 0 || addedConnections.Count > 0)
        {
            if (addedPieces.Count > 0)
                diff.Pieces = new PiecesDiff { Added = addedPieces };
            if (addedConnections.Count > 0)
                diff.Connections = new ConnectionsDiff { Added = addedConnections };
        }
        return diff;
    }

    private static bool flatPiecesByGuid_PassThrough(Piece piece, out Coord? center)
    {
        var centerAttr = piece.Attributes.FirstOrDefault(a => a.Key == "semio.center");
        if (centerAttr?.Value is not null)
        {
            center = Utility.Deserialize<Coord>(centerAttr.Value);
            return true;
        }
        center = piece.Center;
        return center is not null;
    }
}

#endregion 📐Design






#region ⏱️Kit
// Implementations MUST collect types and designs into a reusable library.

#region 🧬KitKind
// KitKind discriminates the five persistence/transport forms of a Kit.

/// <summary>
/// 🏷️Discriminator for the five kit persistence/transport forms.
/// </summary>
/// <remarks>
/// Specs: Exactly five kit kinds exist:
/// - File: Self-contained JSON file
/// - Folder: Local folder with .semio/kit.db SQLite and asset files
/// - Archive: ZIP file packaging a FolderKit structure
/// - Remote: URL-addressable kit served over HTTP(S)
/// - Temporary: In-memory ephemeral kit (no persistence)
/// </remarks>
[JsonConverter(typeof(StringEnumConverter))]
public enum KitKind
{
    [EnumMember(Value = "file")]
    File,
    [EnumMember(Value = "folder")]
    Folder,
    [EnumMember(Value = "archive")]
    Archive,
    [EnumMember(Value = "remote")]
    Remote,
    [EnumMember(Value = "temporary")]
    Temporary
}

/// <summary>🔷Helpers for KitKind.</summary>
public static class KitKinds
{
    /// <summary>🔑All valid KitKind values.</summary>
    public static readonly KitKind[] All = (KitKind[])Enum.GetValues(typeof(KitKind));
}

#endregion 🧬KitKind

public class KitDiff : Entity<KitDiff>
{
    private readonly HashSet<string> _setProperties = new();
    private string? _guid;
    private string? _name;
    private string? _description;
    private string? _icon;
    private string? _image;
    private string? _preview;
    private string? _version;
    private string? _remote;
    private string? _homepage;
    private string? _license;
    private TypesDiff? _types;
    private DesignsDiff? _designs;
    private TagsDiff? _tags;
    private FilesDiff? _files;
    private FoldersDiff? _folders;
    private PortsDiff? _ports;
    private AuthorsDiff? _authors;
    private AttributesDiff? _attributes;
    private ConceptsDiff? _concepts;
    private string? _createdAt;
    private string? _updatedAt;

    public string? Guid { get => _guid; set { _guid = value; _setProperties.Add("Guid"); } }
    public string? Name { get => _name; set { _name = value; _setProperties.Add("Name"); } }
    public string? Description { get => _description; set { _description = value; _setProperties.Add("Description"); } }
    public string? Icon { get => _icon; set { _icon = value; _setProperties.Add("Icon"); } }
    public string? Image { get => _image; set { _image = value; _setProperties.Add("Image"); } }
    public string? Preview { get => _preview; set { _preview = value; _setProperties.Add("Preview"); } }
    public string? Version { get => _version; set { _version = value; _setProperties.Add("Version"); } }
    public string? Remote { get => _remote; set { _remote = value; _setProperties.Add("Remote"); } }
    public string? Homepage { get => _homepage; set { _homepage = value; _setProperties.Add("Homepage"); } }
    public string? License { get => _license; set { _license = value; _setProperties.Add("License"); } }
    public TypesDiff? Types { get => _types; set { _types = value; _setProperties.Add("Types"); } }
    public DesignsDiff? Designs { get => _designs; set { _designs = value; _setProperties.Add("Designs"); } }
    public TagsDiff? Tags { get => _tags; set { _tags = value; _setProperties.Add("Tags"); } }
    public FilesDiff? Files { get => _files; set { _files = value; _setProperties.Add("Files"); } }
    public FoldersDiff? Folders { get => _folders; set { _folders = value; _setProperties.Add("Folders"); } }
    public PortsDiff? Ports { get => _ports; set { _ports = value; _setProperties.Add("Ports"); } }
    public AuthorsDiff? Authors { get => _authors; set { _authors = value; _setProperties.Add("Authors"); } }
    public AttributesDiff? Attributes { get => _attributes; set { _attributes = value; _setProperties.Add("Attributes"); } }
    public ConceptsDiff? Concepts { get => _concepts; set { _concepts = value; _setProperties.Add("Concepts"); } }
    public string? CreatedAt { get => _createdAt; set { _createdAt = value; _setProperties.Add("CreatedAt"); } }
    public string? UpdatedAt { get => _updatedAt; set { _updatedAt = value; _setProperties.Add("UpdatedAt"); } }

    public bool ShouldSerializeGuid() => _setProperties.Contains("Guid");
    public bool ShouldSerializeName() => _setProperties.Contains("Name");
    public bool ShouldSerializeDescription() => _setProperties.Contains("Description");
    public bool ShouldSerializeIcon() => _setProperties.Contains("Icon");
    public bool ShouldSerializeImage() => _setProperties.Contains("Image");
    public bool ShouldSerializePreview() => _setProperties.Contains("Preview");
    public bool ShouldSerializeVersion() => _setProperties.Contains("Version");
    public bool ShouldSerializeRemote() => _setProperties.Contains("Remote");
    public bool ShouldSerializeHomepage() => _setProperties.Contains("Homepage");
    public bool ShouldSerializeLicense() => _setProperties.Contains("License");
    public bool ShouldSerializeTypes() => _setProperties.Contains("Types");
    public bool ShouldSerializeDesigns() => _setProperties.Contains("Designs");
    public bool ShouldSerializeTags() => _setProperties.Contains("Tags");
    public bool ShouldSerializeFiles() => _setProperties.Contains("Files");
    public bool ShouldSerializeFolders() => _setProperties.Contains("Folders");
    public bool ShouldSerializePorts() => _setProperties.Contains("Ports");
    public bool ShouldSerializeAuthors() => _setProperties.Contains("Authors");
    public bool ShouldSerializeAttributes() => _setProperties.Contains("Attributes");
    public bool ShouldSerializeConcepts() => _setProperties.Contains("Concepts");
    public bool ShouldSerializeCreatedAt() => _setProperties.Contains("CreatedAt");
    public bool ShouldSerializeUpdatedAt() => _setProperties.Contains("UpdatedAt");

    public KitDiff MergeDiff(KitDiff other)
    {
        return new KitDiff
        {
            Guid = other.Guid ?? Guid,
            Name = other.Name ?? Name,
            Description = other.Description ?? Description,
            Icon = other.Icon ?? Icon,
            Image = other.Image ?? Image,
            Preview = other.Preview ?? Preview,
            Version = other.Version ?? Version,
            Remote = other.Remote ?? Remote,
            Homepage = other.Homepage ?? Homepage,
            License = other.License ?? License,
            Types = other.Types ?? Types,
            Designs = other.Designs ?? Designs,
            Files = other.Files ?? Files,
            Folders = other.Folders ?? Folders,
            Ports = other.Ports ?? Ports,
            Authors = other.Authors ?? Authors,
            Attributes = other.Attributes ?? Attributes,
            Concepts = other.Concepts ?? Concepts,
            CreatedAt = other.CreatedAt ?? CreatedAt,
            UpdatedAt = other.UpdatedAt ?? UpdatedAt
        };
    }

    public static implicit operator KitDiff(Kit kit) => new()
    {
        Guid = kit.Guid,
        Name = kit.Name,
        Description = kit.Description,
        Icon = kit.Icon,
        Image = kit.Image,
        Preview = kit.Preview,
        Version = kit.Version,
        Remote = kit.Remote,
        Homepage = kit.Homepage,
        License = kit.License,
        Concepts = new ConceptsDiff { Added = kit.Concepts, Removed = new List<ConceptId>(), Updated = new List<ConceptDiffUpdate>() },
        CreatedAt = kit.CreatedAt,
        UpdatedAt = kit.UpdatedAt
    };
}

public class KitId : Entity<KitId>
{
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"KitId({ToHumanIdString()})";

    public static implicit operator KitId(Kit kit) => new() { Guid = kit.Guid };
    public static implicit operator KitId(KitDiff diff) => new() { Guid = diff.Guid ?? "" };
}

public class KitsDiff : Entity<KitsDiff>
{
    public List<KitId> Removed { get; set; } = new();
    public List<KitDiffUpdate> Updated { get; set; } = new();
    public List<Kit> Added { get; set; } = new();

    public static implicit operator KitsDiff(List<Kit> kits) => new() { Updated = kits.Select(k => new KitDiffUpdate { Kit = k, Diff = (KitDiff)k }).ToList() };
}

public partial class Kit : Entity<Kit>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Version { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public List<Concept> Concepts { get; set; } = new();
    public List<Tag> Tags { get; set; } = new();
    public string? Remote { get; set; }
    public string? Homepage { get; set; }
    public string? License { get; set; }
    public List<Author> Authors { get; set; } = new();
    public List<Piece> Pieces { get; set; } = new();
    public List<Group> Groups { get; set; } = new();
    public List<Connection> Connections { get; set; } = new();
    public List<Prop> Props { get; set; } = new();
    public List<Stat> Stats { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public string? Preview { get; set; }
    public List<Quality> Qualities { get; set; } = new();
    [JsonProperty("ports")]
    public List<Port> Ports { get; set; } = new();
    public List<File> Files { get; set; } = new();
    public List<Folder> Folders { get; set; } = new();
    public List<Type> Types { get; set; } = new();
    public List<Design> Designs { get; set; } = new();
    public string CreatedAt { get; set; } = "";
    public string UpdatedAt { get; set; } = "";

    public static implicit operator Kit(KitDiff diff) => new() { Name = diff.Name ?? "", Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Preview = diff.Preview ?? "", Version = diff.Version ?? "", Remote = diff.Remote ?? "", Homepage = diff.Homepage ?? "", License = diff.License ?? "", Files = diff.Files?.Added ?? new(), Attributes = diff.Attributes?.Added ?? new() };
    public static implicit operator string(Kit kit) => kit.Name;
    public static implicit operator Kit(string name) => new() { Name = name };

    public static Kit ApplyDiff(Kit kit, KitDiff diff)
    {
        var types = kit.Types;
        var designs = kit.Designs;
        var files = kit.Files;
        var attributes = kit.Attributes;

        if (diff.Types is not null)
        {
            types = ApplyTypesDiff(kit.Types, diff.Types);
        }
        if (diff.Designs is not null)
        {
            designs = ApplyDesignsDiff(kit.Designs, diff.Designs);
        }
        if (diff.Files is not null)
        {
            files = ApplyFilesDiff(kit.Files, diff.Files);
        }
        if (diff.Attributes is not null)
        {
            attributes = ApplyAttributesDiff(kit.Attributes, diff.Attributes);
        }

        return new Kit
        {
            Name = string.IsNullOrEmpty(diff.Name) ? kit.Name : diff.Name,
            Description = string.IsNullOrEmpty(diff.Description) ? kit.Description : diff.Description,
            Icon = string.IsNullOrEmpty(diff.Icon) ? kit.Icon : diff.Icon,
            Image = string.IsNullOrEmpty(diff.Image) ? kit.Image : diff.Image,
            Preview = string.IsNullOrEmpty(diff.Preview) ? kit.Preview : diff.Preview,
            Version = string.IsNullOrEmpty(diff.Version) ? kit.Version : diff.Version,
            Remote = string.IsNullOrEmpty(diff.Remote) ? kit.Remote : diff.Remote,
            Homepage = string.IsNullOrEmpty(diff.Homepage) ? kit.Homepage : diff.Homepage,
            License = string.IsNullOrEmpty(diff.License) ? kit.License : diff.License,
            Authors = kit.Authors,
            Qualities = kit.Qualities,
            Files = files,
            Types = types,
            Designs = designs,
            Attributes = attributes
        };
    }

    private static List<Attribute> ApplyAttributesDiff(List<Attribute> original, AttributesDiff diff)
    {
        var result = original.Where(a => !diff.Removed.Any(r => r.Guid == a.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(a => a.Guid == updated.Attribute.Guid);
            if (index >= 0 && updated.Diff != null)
                result[index] = Attribute.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    public static KitDiff CreateDiff(Kit kit)
    {
        return new KitDiff
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = new TypesDiff
            {
                Removed = new List<TypeId>(),
                Updated = kit.Types.Select(t => new TypeDiffUpdate { Type = t, Diff = Type.CreateDiff(t) }).ToList(),
                Added = new List<Type>()
            },
            Designs = new DesignsDiff
            {
                Removed = new List<DesignId>(),
                Updated = kit.Designs.Select(d => new DesignDiffUpdate { Design = d, Diff = Design.CreateDiff(d) }).ToList(),
                Added = new List<Design>()
            },
            Files = new FilesDiff
            {
                Removed = new List<FileId>(),
                Updated = kit.Files.Select(f => new FileDiffUpdate { File = f, Diff = (FileDiff)f }).ToList(),
                Added = new List<File>()
            },
            Attributes = kit.Attributes
        };
    }

    private static List<Type> ApplyTypesDiff(List<Type> original, TypesDiff diff)
    {
        var result = original.Where(t => !diff.Removed.Any(r => r.Guid == t.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(t => t.Guid == updated.Type.Guid);
            if (index >= 0 && updated.Diff != null)
                result[index] = Type.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static TypesDiff CreateTypesDiff(List<Type> original, List<Type> modified)
    {
        var originalGuids = original.Select(t => t.Guid).ToHashSet();
        var modifiedGuids = modified.Select(t => t.Guid).ToHashSet();

        return new TypesDiff
        {
            Removed = original.Where(t => !modifiedGuids.Contains(t.Guid)).Select(t => new TypeId { Guid = t.Guid }).ToList(),
            Updated = original.Where(t => modifiedGuids.Contains(t.Guid))
                .SelectMany(t =>
                {
                    var modifiedType = modified.First(m => m.Guid == t.Guid);
                    var diff = Type.CreateDiff(t);
                    return !Equals(t, modifiedType) ? new[] { new TypeDiffUpdate { Type = t, Diff = diff } } : Array.Empty<TypeDiffUpdate>();
                })
                .ToList(),
            Added = modified.Where(t => !originalGuids.Contains(t.Guid)).ToList()
        };
    }

    private static List<Design> ApplyDesignsDiff(List<Design> original, DesignsDiff diff)
    {
        var result = original.Where(d => !diff.Removed.Any(r => r.Guid == d.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(d => d.Guid == updated.Design.Guid);
            if (index >= 0 && updated.Diff != null)
                result[index] = Design.ApplyDiff(result[index], updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private static DesignsDiff CreateDesignsDiff(List<Design> original, List<Design> modified)
    {
        var originalGuids = original.Select(d => d.Guid).ToHashSet();
        var modifiedGuids = modified.Select(d => d.Guid).ToHashSet();

        return new DesignsDiff
        {
            Removed = original.Where(d => !modifiedGuids.Contains(d.Guid)).Select(d => new DesignId { Guid = d.Guid }).ToList(),
            Updated = original.Where(d => modifiedGuids.Contains(d.Guid))
                .SelectMany(d =>
                {
                    var modifiedDesign = modified.First(m => m.Guid == d.Guid);
                    var diff = Design.GetDesignDiff(d, modifiedDesign);
                    return !Equals(d, modifiedDesign) ? new[] { new DesignDiffUpdate { Design = d, Diff = diff } } : Array.Empty<DesignDiffUpdate>();
                })
                .ToList(),
            Added = modified.Where(d => !originalGuids.Contains(d.Guid)).ToList()
        };
    }

    private static List<File> ApplyFilesDiff(List<File> original, FilesDiff diff)
    {
        var result = original.Where(f => !diff.Removed.Any(r => r.Guid == f.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(f => f.Guid == updated.File.Guid);
            if (index >= 0 && updated.Diff != null)
            {
                var file = result[index];
                result[index] = new File
                {
                    Guid = updated.Diff.Guid ?? file.Guid,
                    Name = updated.Diff.Name ?? file.Name,
                    Remote = updated.Diff.Remote ?? file.Remote,
                    Folder = updated.Diff.Folder ?? file.Folder,
                    Size = updated.Diff.Size ?? file.Size,
                    Hash = updated.Diff.Hash ?? file.Hash,
                    CreatedAt = updated.Diff.CreatedAt ?? file.CreatedAt,
                    CreatedBy = updated.Diff.CreatedBy ?? file.CreatedBy,
                    UpdatedAt = updated.Diff.UpdatedAt ?? file.UpdatedAt,
                    UpdatedBy = updated.Diff.UpdatedBy ?? file.UpdatedBy
                };
            }
        }
        result.AddRange(diff.Added);
        return result;
    }

    public override (bool, List<string>) Validate()
    {
        var (isValid, errors) = base.Validate();

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
        var typeIds = Types.Select(t => t.Name);
        var duplicateTypeIds = typeIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicateTypeIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicateName in duplicateTypeIds)
            {
                errors.Add($"There are multiple identical types ({duplicateName}).");
            }
        }
        var designIds = Designs.Select(d => d.Guid);
        var duplicateDesignIds = designIds.GroupBy(x => x).Where(g => g.Count() > 1).Select(g => g.Key).ToArray();
        if (duplicateDesignIds.Length != 0)
        {
            isValid = false;
            foreach (var duplicateName in duplicateDesignIds)
            {
                errors.Add($"There are multiple identical designs ({duplicateName}).");
            }
        }
        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute ({attribute.ToIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }

    public static bool IsSameAs(Kit kit, Kit other)
    {
        if (other is null) return false;
        return kit.Name == other.Name;
    }

    public static Type FindTypeByName(Kit kit, string typeName)
    {
        var type = kit.Types.FirstOrDefault(t => t.Name == typeName);
        if (type is null) throw new ArgumentException($"Type {typeName} not found in kit {kit.Name}");
        return type;
    }

    public static Design FindDesignByName(Kit kit, string designName)
    {
        var design = kit.Designs.FirstOrDefault(d => d.Name == designName);
        if (design is null) throw new ArgumentException($"Design {designName} not found in kit {kit.Name}");
        return design;
    }

    public static Kit AddType(Kit kit, Type type)
    {
        var newTypes = new List<Type>(kit.Types) { type };
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = newTypes,
            Designs = new List<Design>(kit.Designs),
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = new List<Attribute>(kit.Attributes)
        };
    }

    public static Kit RemoveType(Kit kit, string typeName)
    {
        var newTypes = kit.Types.Where(t => t.Name != typeName).ToList();
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = newTypes,
            Designs = new List<Design>(kit.Designs),
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = new List<Attribute>(kit.Attributes)
        };
    }

    public static Kit AddDesign(Kit kit, Design design)
    {
        var newDesigns = new List<Design>(kit.Designs) { design };
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = new List<Type>(kit.Types),
            Designs = newDesigns,
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = new List<Attribute>(kit.Attributes)
        };
    }

    public static Kit RemoveDesign(Kit kit, string designName)
    {
        var newDesigns = kit.Designs.Where(d => d.Name != designName).ToList();
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = new List<Type>(kit.Types),
            Designs = newDesigns,
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = new List<Attribute>(kit.Attributes)
        };
    }

    public static string FindAttributeValue(Kit kit, string key, string defaultValue = "")
    {
        var attribute = kit.Attributes.FirstOrDefault(a => a.Key == key);
        return attribute?.Value ?? defaultValue;
    }

    public static Kit SetAttribute(Kit kit, Attribute attribute)
    {
        var newAttributes = kit.Attributes.Where(a => a.Key != attribute.Key).ToList();
        newAttributes.Add(attribute);
        return new Kit
        {
            Name = kit.Name,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Version = kit.Version,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = new List<Type>(kit.Types),
            Designs = new List<Design>(kit.Designs),
            Authors = new List<Author>(kit.Authors),
            Qualities = new List<Quality>(kit.Qualities),
            Attributes = newAttributes
        };
    }

    #region 📻Design Family Helpers
    // Callers MUST use these helpers to traverse design parent-child hierarchies.

    public static Design FindDesignByGuid(Kit kit, string designGuid)
    {
        var design = kit.Designs.FirstOrDefault(d => d.Guid == designGuid);
        if (design is null) throw new ArgumentException($"Design {designGuid} not found in kit {kit.Name}");
        return design;
    }

    public static Design GetPrimitiveDesign(Kit kit, string designGuid)
    {
        var current = FindDesignByGuid(kit, designGuid);
        while (current.Parent?.Guid is not null)
        {
            current = FindDesignByGuid(kit, current.Parent.Guid);
        }
        return current;
    }

    public static List<Design> GetDesignFamily(Kit kit, string designGuid)
    {
        var primitive = GetPrimitiveDesign(kit, designGuid);
        var family = new List<Design>();
        CollectDesignDescendants(kit, primitive.Guid, family);
        return family;
    }

    private static void CollectDesignDescendants(Kit kit, string parentGuid, List<Design> family)
    {
        var parent = FindDesignByGuid(kit, parentGuid);
        family.Add(parent);
        var children = kit.Designs.Where(d => d.Parent?.Guid == parentGuid);
        foreach (var child in children)
        {
            CollectDesignDescendants(kit, child.Guid, family);
        }
    }

    public static bool AreDesignsInSameFamily(Kit kit, string designGuidA, string designGuidB)
    {
        var primitiveA = GetPrimitiveDesign(kit, designGuidA);
        var primitiveB = GetPrimitiveDesign(kit, designGuidB);
        return primitiveA.Guid == primitiveB.Guid;
    }

    public static bool CanUseDesignAsPiece(Kit kit, string containerDesignGuid, string pieceDesignGuid)
    {
        return !AreDesignsInSameFamily(kit, containerDesignGuid, pieceDesignGuid);
    }

    public static List<Piece> FindSameFamilyDesignPieces(Kit kit, string designGuid)
    {
        var design = FindDesignByGuid(kit, designGuid);
        return design.Pieces
            .Where(p => p.Design?.Guid is not null && AreDesignsInSameFamily(kit, designGuid, p.Design.Guid))
            .ToList();
    }

    #endregion 📻Design Family Helpers

    #region 🧊Type Family Helpers
    // Callers MUST use these helpers to traverse type parent-child hierarchies.

    public static Type FindTypeByGuid(Kit kit, string typeGuid)
    {
        var type = kit.Types.FirstOrDefault(t => t.Guid == typeGuid);
        if (type is null) throw new ArgumentException($"Type {typeGuid} not found in kit {kit.Name}");
        return type;
    }

    public static Type GetPrimitiveType(Kit kit, string typeGuid)
    {
        var current = FindTypeByGuid(kit, typeGuid);
        while (current.Parent?.Guid is not null)
        {
            current = FindTypeByGuid(kit, current.Parent.Guid);
        }
        return current;
    }

    public static List<Type> GetTypeFamily(Kit kit, string typeGuid)
    {
        var primitive = GetPrimitiveType(kit, typeGuid);
        var family = new List<Type>();
        CollectTypeDescendants(kit, primitive.Guid, family);
        return family;
    }

    private static void CollectTypeDescendants(Kit kit, string parentGuid, List<Type> family)
    {
        var parent = FindTypeByGuid(kit, parentGuid);
        family.Add(parent);
        var children = kit.Types.Where(t => t.Parent?.Guid == parentGuid);
        foreach (var child in children)
        {
            CollectTypeDescendants(kit, child.Guid, family);
        }
    }

    public static bool AreTypesInSameFamily(Kit kit, string typeGuidA, string typeGuidB)
    {
        var primitiveA = GetPrimitiveType(kit, typeGuidA);
        var primitiveB = GetPrimitiveType(kit, typeGuidB);
        return primitiveA.Guid == primitiveB.Guid;
    }

    #endregion 🧊Type Family Helpers

    #region 🔍Kit Finders
    // Callers MUST use these methods to locate entities within a kit by GUID.

    public static File FindFile(Kit kit, string fileGuid)
    {
        var file = kit.Files?.FirstOrDefault(f => f.Guid == fileGuid);
        if (file == null) throw new Exception($"File {fileGuid} not found in kit {kit.Name}");
        return file;
    }

    public static Tag FindTag(Kit kit, string tagGuid)
    {
        var tag = kit.Tags?.FirstOrDefault(t => t.Guid == tagGuid);
        if (tag == null) throw new Exception($"Tag {tagGuid} not found in kit {kit.Name}");
        return tag;
    }

    public static Concept FindConcept(Kit kit, string conceptGuid)
    {
        var concept = kit.Concepts?.FirstOrDefault(c => c.Guid == conceptGuid);
        if (concept == null) throw new Exception($"Concept {conceptGuid} not found in kit {kit.Name}");
        return concept;
    }

    public static Type FindType(Kit kit, string typeGuid)
    {
        var type = kit.Types?.FirstOrDefault(t => t.Guid == typeGuid);
        if (type == null) throw new Exception($"Type {typeGuid} not found in kit {kit.Name}");
        return type;
    }

    public static Design FindDesign(Kit kit, string designGuid)
    {
        var design = kit.Designs?.FirstOrDefault(d => d.Guid == designGuid);
        if (design == null) throw new Exception($"Design {designGuid} not found in kit {kit.Name}");
        return design;
    }

    public static Port FindPort(Kit kit, string portGuid)
    {
        var port = kit.Ports?.FirstOrDefault(p => p.Guid == portGuid);
        if (port == null) throw new Exception($"Port {portGuid} not found in kit {kit.Name}");
        return port;
    }

    public static Type FindPieceTypeInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesign(kit, designGuid);
        var piece = Piece.FindInDesign(design, pieceGuid);
        if (piece.Type == null) throw new Exception($"Piece {pieceGuid} has no type");
        return FindType(kit, piece.Type.Guid);
    }

    public static Piece FindParentPieceInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesign(kit, designGuid);
        var connection = Connection.FindByPieceInDesign(design, pieceGuid).FirstOrDefault(c => c.Connecting.Piece.Guid == pieceGuid);
        if (connection == null) throw new Exception($"No parent piece found for piece {pieceGuid}");
        return Piece.FindInDesign(design, connection.Connected.Piece.Guid);
    }

    public static Connection FindParentConnectionForPieceInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesign(kit, designGuid);
        var connection = Connection.FindByPieceInDesign(design, pieceGuid).FirstOrDefault(c => c.Connecting.Piece.Guid == pieceGuid);
        if (connection == null) throw new Exception($"No parent connection found for piece {pieceGuid}");
        return connection;
    }

    public static List<Piece> FindChildrenPiecesInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesign(kit, designGuid);
        var connections = Connection.FindByPieceInDesign(design, pieceGuid).Where(c => c.Connected.Piece.Guid == pieceGuid);
        return connections.Select(c => Piece.FindInDesign(design, c.Connecting.Piece.Guid)).ToList();
    }

    public static List<Connector> FindUsedConnectorsByPieceInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesign(kit, designGuid);
        var piece = Piece.FindInDesign(design, pieceGuid);
        var type = piece.Type != null ? FindType(kit, piece.Type.Guid) : null;
        if (type == null) return new List<Connector>();

        var connections = Connection.FindByPieceInDesign(design, pieceGuid);
        var connectors = new List<Connector>();
        foreach (var connection in connections)
        {
            var connector = Connector.FindForPieceInConnection(type, connection, pieceGuid);
            if (connector != null) connectors.Add(connector);
        }
        return connectors;
    }

    public static Type[] FindReplacableTypesForPieceInDesign(Kit kit, string designGuid, string pieceGuid, string[]? variants = null)
    {
        var design = FindDesign(kit, designGuid);
        var connections = Connection.FindByPieceInDesign(design, pieceGuid);
        var requiredConnectors = new List<Connector>();

        foreach (var connection in connections)
        {
            try
            {
                var otherPieceId = connection.Connected.Piece.Guid == pieceGuid ? connection.Connecting.Piece.Guid : connection.Connected.Piece.Guid;
                var otherPiece = Piece.FindInDesign(design, otherPieceId);
                if (otherPiece.Type == null) continue;

                var otherType = FindType(kit, otherPiece.Type.Guid);
                var otherPortId = connection.Connected.Piece.Guid == pieceGuid ? connection.Connecting.Connector?.Guid : connection.Connected.Connector?.Guid;
                var otherPort = Connector.FindInType(otherType, otherPortId ?? "");
                requiredConnectors.Add(otherPort);
            }
            catch
            {
                continue;
            }
        }

        return (kit.Types ?? new List<Type>()).Where(replacementType =>
        {
            if (replacementType.IsAbstract ?? false) return false;
            if (variants != null && !variants.Contains(replacementType.Parent?.Guid ?? "")) return false;
            if (replacementType.Connectors == null || replacementType.Connectors.Count == 0) return requiredConnectors.Count == 0;

            return requiredConnectors.All(requiredConnector =>
            {
                return replacementType.Connectors.Any(replacementConnector => replacementConnector.Guid == requiredConnector.Guid);
            });
        }).ToArray();
    }

    public static Type[] FindReplacableTypesForPiecesInDesign(Kit kit, string designGuid, string[] pieceGuids, string[]? variants = null)
    {
        var design = FindDesign(kit, designGuid);
        var pieces = pieceGuids.Select(id => Piece.FindInDesign(design, id)).ToList();
        var externalConnections = new List<(Connection connection, Connector requiredConnector)>();

        foreach (var piece in pieces)
        {
            var connections = Connection.FindByPieceInDesign(design, piece.Guid);
            foreach (var connection in connections)
            {
                var otherPieceId = connection.Connected.Piece.Guid == piece.Guid ? connection.Connecting.Piece.Guid : connection.Connected.Piece.Guid;
                if (!pieceGuids.Contains(otherPieceId))
                {
                    try
                    {
                        var otherPiece = Piece.FindInDesign(design, otherPieceId);
                        if (otherPiece.Type == null) continue;

                        var otherType = FindType(kit, otherPiece.Type.Guid);
                        var otherPortId = connection.Connected.Piece.Guid == piece.Guid ? connection.Connecting.Connector?.Guid : connection.Connected.Connector?.Guid;
                        var otherPort = Connector.FindInType(otherType, otherPortId ?? "");
                        externalConnections.Add((connection, otherPort));
                    }
                    catch
                    {
                        continue;
                    }
                }
            }
        }

        return (kit.Types ?? new List<Type>()).Where(replacementType =>
        {
            if (replacementType.IsAbstract ?? false) return false;
            if (variants != null && !variants.Contains(replacementType.Parent?.Guid ?? "")) return false;
            if (replacementType.Connectors == null || replacementType.Connectors.Count == 0) return externalConnections.Count == 0;

            return externalConnections.All(ec =>
            {
                return replacementType.Connectors.Any(replacementConnector => replacementConnector.Guid == ec.requiredConnector.Guid);
            });
        }).ToArray();
    }

    public static double SumQualityInDesign(Kit kit, string designGuid, string qualityGuid)
    {
        var design = FindDesign(kit, designGuid);
        double sum = 0;
        foreach (var piece in design.Pieces ?? new List<Piece>())
        {
            var pieceProp = piece.Props?.FirstOrDefault(p => p.Quality?.Guid == qualityGuid);
            if (pieceProp != null)
            {
                if (double.TryParse(pieceProp.Value, System.Globalization.NumberStyles.Float, System.Globalization.CultureInfo.InvariantCulture, out var val))
                    sum += val;
                continue;
            }
            if (piece.Type != null)
            {
                var type = kit.Types?.FirstOrDefault(t => t.Guid == piece.Type.Guid);
                if (type != null)
                {
                    var typeProp = type.Props?.FirstOrDefault(p => p.Quality?.Guid == qualityGuid);
                    if (typeProp != null && double.TryParse(typeProp.Value, System.Globalization.NumberStyles.Float, System.Globalization.CultureInfo.InvariantCulture, out var val))
                        sum += val;
                }
            }
        }
        return sum;
    }

    #endregion 🔍Kit Finders

    #region 🎠Filter
    // Filter MUST provide functions to produce a minimal kit subset scoped to a single design.

    /// <summary>🧩Glob filter with include and exclude patterns for name-based entity filtering.</summary>
    public class GlobFilter
    {
        public List<string>? Include { get; set; }
        public List<string>? Exclude { get; set; }
    }

    /// <summary>🧹General-purpose kit filter combining design-based transitive filtering with glob-based name filtering.</summary>
    public class KitFilter
    {
        public string? DesignGuid { get; set; }
        public string[]? ModelTags { get; set; }
        public GlobFilter? Designs { get; set; }
        public GlobFilter? Types { get; set; }
        public GlobFilter? Ports { get; set; }
        public GlobFilter? Files { get; set; }
        public GlobFilter? Tags { get; set; }
        public GlobFilter? Concepts { get; set; }
        public GlobFilter? Qualities { get; set; }
        public GlobFilter? Authors { get; set; }
        public GlobFilter? Folders { get; set; }
    }

    /// <summary>🔤Matches a name against a glob pattern supporting * and ?. Case-insensitive.</summary>
    public static bool GlobMatch(string name, string pattern)
    {
        var regexStr = "^";
        foreach (var c in pattern)
        {
            regexStr += c switch
            {
                '*' => ".*",
                '?' => ".",
                _ => System.Text.RegularExpressions.Regex.Escape(c.ToString())
            };
        }
        regexStr += "$";
        return System.Text.RegularExpressions.Regex.IsMatch(name, regexStr, System.Text.RegularExpressions.RegexOptions.IgnoreCase);
    }

    /// <summary>🧪Checks if a name passes a GlobFilter.</summary>
    public static bool MatchesGlobFilter(string name, GlobFilter? filter)
    {
        if (filter == null) return true;
        if (filter.Include is { Count: > 0 } && !filter.Include.Any(p => GlobMatch(name, p))) return false;
        if (filter.Exclude is { Count: > 0 } && filter.Exclude.Any(p => GlobMatch(name, p))) return false;
        return true;
    }

    /// <summary>📐Filters a kit to only include entities related to a specific design.</summary>
    /// <remarks>
    /// Removes types not used by pieces, designs not used by pieces, ports not used by connectors of used types,
    /// files not used by selected models, and keeps at most one model per type according to the optional tags.
    /// </remarks>
    private static Kit FilterKitByDesign(Kit kit, string designGuid, string[]? tags = null)
    {
        var design = (kit.Designs ?? new List<Design>()).FirstOrDefault(d => d.Guid == designGuid);
        if (design == null) return new Kit { Guid = kit.Guid, Name = kit.Name, Version = kit.Version };

        var usedTypeGuids = new HashSet<string>();
        var usedDesignGuids = new HashSet<string> { designGuid };
        foreach (var piece in design.Pieces ?? new List<Piece>())
        {
            if (!string.IsNullOrEmpty(piece.Type?.Guid)) usedTypeGuids.Add(piece.Type.Guid);
            if (!string.IsNullOrEmpty(piece.Design?.Guid)) usedDesignGuids.Add(piece.Design.Guid);
        }

        var typeByGuid = (kit.Types ?? new List<Type>()).ToDictionary(t => t.Guid, t => t);
        void CollectAncestors(string typeGuid)
        {
            if (!typeByGuid.TryGetValue(typeGuid, out var type) || string.IsNullOrEmpty(type.Parent?.Guid) || usedTypeGuids.Contains(type.Parent.Guid)) return;
            usedTypeGuids.Add(type.Parent.Guid);
            CollectAncestors(type.Parent.Guid);
        }
        foreach (var typeGuid in usedTypeGuids.ToList()) CollectAncestors(typeGuid);

        var resolvedTagGuids = new List<string>();
        foreach (var tagValue in tags ?? Array.Empty<string>())
        {
            var byGuid = (kit.Tags ?? new List<Tag>()).FirstOrDefault(t => t.Guid == tagValue);
            if (byGuid != null) { resolvedTagGuids.Add(byGuid.Guid); continue; }
            resolvedTagGuids.AddRange((kit.Tags ?? new List<Tag>()).Where(t => t.Name == tagValue).Select(t => t.Guid));
        }

        var usedPortGuids = new HashSet<string>();
        var usedFileGuids = new HashSet<string>();
        var usedTagGuids = new HashSet<string>();
        var usedConceptGuids = new HashSet<string>();
        var usedQualityGuids = new HashSet<string>();
        var usedAuthorGuids = new HashSet<string>();
        var usedFolderNames = new HashSet<string>();
        var selectedModels = new Dictionary<string, Model>();

        void CollectQualityFromProps(IEnumerable<Prop>? props)
        {
            foreach (var prop in props ?? new List<Prop>())
                if (!string.IsNullOrEmpty(prop.Quality?.Guid)) usedQualityGuids.Add(prop.Quality.Guid);
        }

        foreach (var typeGuid in usedTypeGuids)
        {
            if (!typeByGuid.TryGetValue(typeGuid, out var type)) continue;
            if (!string.IsNullOrEmpty(type.Folder)) usedFolderNames.Add(type.Folder);
            foreach (var connector in type.Connectors ?? new List<Connector>())
            {
                if (!string.IsNullOrEmpty(connector.Port?.Guid)) usedPortGuids.Add(connector.Port.Guid);
                CollectQualityFromProps(connector.Props);
            }
            CollectQualityFromProps(type.Props);
            foreach (var author in type.Authors ?? new List<AuthorId>())
                if (!string.IsNullOrEmpty(author.Guid)) usedAuthorGuids.Add(author.Guid);
            foreach (var concept in type.Concepts ?? new List<ConceptId>())
                if (!string.IsNullOrEmpty(concept.Guid)) usedConceptGuids.Add(concept.Guid);

            if ((type.Models?.Count ?? 0) > 0)
            {
                var best = ExportFindMatchingModel(kit, type, resolvedTagGuids.ToArray());
                if (best != null)
                {
                    selectedModels[typeGuid] = best;
                    if (!string.IsNullOrEmpty(best.File?.Guid)) usedFileGuids.Add(best.File.Guid);
                    foreach (var tag in best.Tags ?? new List<TagId>())
                        if (!string.IsNullOrEmpty(tag.Guid)) usedTagGuids.Add(tag.Guid);
                }
            }
        }

        foreach (var piece in design.Pieces ?? new List<Piece>()) CollectQualityFromProps(piece.Props);
        foreach (var concept in design.Concepts ?? new List<ConceptId>())
            if (!string.IsNullOrEmpty(concept.Guid)) usedConceptGuids.Add(concept.Guid);
        foreach (var author in design.Authors ?? new List<AuthorId>())
            if (!string.IsNullOrEmpty(author.Guid)) usedAuthorGuids.Add(author.Guid);
        foreach (var portGuid in usedPortGuids.ToList())
        {
            var port = (kit.Ports ?? new List<Port>()).FirstOrDefault(p => p.Guid == portGuid);
            foreach (var compatible in port?.CompatiblePorts ?? new List<PortId>())
                if (!string.IsNullOrEmpty(compatible.Guid)) usedPortGuids.Add(compatible.Guid);
        }
        foreach (var tagGuid in resolvedTagGuids) usedTagGuids.Add(tagGuid);

        var filteredTypes = (kit.Types ?? new List<Type>())
            .Where(t => usedTypeGuids.Contains(t.Guid))
            .Select(t =>
            {
                var clone = Entity<Type>.DeepClone(t)!;
                clone.Models = selectedModels.TryGetValue(t.Guid, out var model) ? new List<Model> { model } : new List<Model>();
                return clone;
            })
            .ToList();

        return new Kit
        {
            Guid = kit.Guid,
            Name = kit.Name,
            Version = kit.Version,
            Description = kit.Description,
            Icon = kit.Icon,
            Image = kit.Image,
            Preview = kit.Preview,
            Remote = kit.Remote,
            Homepage = kit.Homepage,
            License = kit.License,
            Types = filteredTypes,
            Designs = (kit.Designs ?? new List<Design>()).Where(d => usedDesignGuids.Contains(d.Guid)).ToList(),
            Ports = (kit.Ports ?? new List<Port>()).Where(p => usedPortGuids.Contains(p.Guid)).ToList(),
            Files = (kit.Files ?? new List<File>()).Where(f => usedFileGuids.Contains(f.Guid)).ToList(),
            Tags = (kit.Tags ?? new List<Tag>()).Where(t => usedTagGuids.Contains(t.Guid)).ToList(),
            Concepts = (kit.Concepts ?? new List<Concept>()).Where(c => usedConceptGuids.Contains(c.Guid)).ToList(),
            Qualities = (kit.Qualities ?? new List<Quality>()).Where(q => usedQualityGuids.Contains(q.Guid)).ToList(),
            Folders = (kit.Folders ?? new List<Folder>()).Where(f => usedFolderNames.Contains(f.Name)).ToList(),
            Authors = (kit.Authors ?? new List<Author>()).Where(a => usedAuthorGuids.Contains(a.Guid)).ToList(),
            Attributes = kit.Attributes,
            CreatedAt = kit.CreatedAt,
            UpdatedAt = kit.UpdatedAt,
        };
    }

    /// <summary>❓General-purpose kit filter combining optional design-based transitive filtering with glob-based name filtering.</summary>
    /// <remarks>
    /// </remarks>
    public static Kit FilterKit(Kit kit, KitFilter filter)
    {
        var baseKit = !string.IsNullOrEmpty(filter.DesignGuid)
            ? FilterKitByDesign(kit, filter.DesignGuid, filter.ModelTags)
            : kit;

        var hasGlobFilters = filter.Designs != null || filter.Types != null || filter.Ports != null ||
            filter.Files != null || filter.Tags != null || filter.Concepts != null ||
            filter.Qualities != null || filter.Authors != null || filter.Folders != null;

        if (!hasGlobFilters) return baseKit;

        return new Kit
        {
            Guid = baseKit.Guid,
            Name = baseKit.Name,
            Version = baseKit.Version,
            Description = baseKit.Description,
            Icon = baseKit.Icon,
            Image = baseKit.Image,
            Preview = baseKit.Preview,
            Remote = baseKit.Remote,
            Homepage = baseKit.Homepage,
            License = baseKit.License,
            Types = (baseKit.Types ?? new List<Type>()).Where(t => MatchesGlobFilter(t.Name, filter.Types)).ToList(),
            Designs = (baseKit.Designs ?? new List<Design>()).Where(d => MatchesGlobFilter(d.Name, filter.Designs)).ToList(),
            Ports = (baseKit.Ports ?? new List<Port>()).Where(p => MatchesGlobFilter(p.Name, filter.Ports)).ToList(),
            Files = (baseKit.Files ?? new List<File>()).Where(f => MatchesGlobFilter(f.Name, filter.Files)).ToList(),
            Tags = (baseKit.Tags ?? new List<Tag>()).Where(t => MatchesGlobFilter(t.Name, filter.Tags)).ToList(),
            Concepts = (baseKit.Concepts ?? new List<Concept>()).Where(c => MatchesGlobFilter(c.Name, filter.Concepts)).ToList(),
            Qualities = (baseKit.Qualities ?? new List<Quality>()).Where(q => MatchesGlobFilter(q.Name, filter.Qualities)).ToList(),
            Folders = (baseKit.Folders ?? new List<Folder>()).Where(f => MatchesGlobFilter(f.Name, filter.Folders)).ToList(),
            Authors = (baseKit.Authors ?? new List<Author>()).Where(a => MatchesGlobFilter(a.Name, filter.Authors)).ToList(),
            Attributes = baseKit.Attributes,
            CreatedAt = baseKit.CreatedAt,
            UpdatedAt = baseKit.UpdatedAt,
        };
    }

    #endregion 🎠Filter
}

#endregion ⏱️Kit


public partial class Kit
{





#region 🔑Meta And Shallow
// Meta classes strip List<> and heavy blob properties. Shallow classes replace List<> properties with Meta item lists.

#region 🎼Sub-entity Meta

public class AttributeMeta
{
    public string Guid { get; set; } = "";
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
    public string Definition { get; set; } = "";
}

public class PropMeta
{
    public string Guid { get; set; } = "";
    public QualityId Quality { get; set; } = new();
    public string Value { get; set; } = "";
    public string? Unit { get; set; }
}

public class TagMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
}

public class ConceptMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
}

public class AuthorMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Email { get; set; } = "";
}

public class FileMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Remote { get; set; }
    public FolderId? Folder { get; set; }
    public int? Size { get; set; }
    public string? Hash { get; set; }
    public DateTime CreatedAt { get; set; }
    public string? CreatedBy { get; set; }
    public DateTime UpdatedAt { get; set; }
    public string? UpdatedBy { get; set; }
}

public class FolderMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public FolderId? Parent { get; set; }
    public string? Description { get; set; }
    public string CreatedAt { get; set; } = "";
    public string? CreatedBy { get; set; }
    public string UpdatedAt { get; set; } = "";
    public string? UpdatedBy { get; set; }
}

public class QualityMeta
{
    public string Guid { get; set; } = "";
    public string Key { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Uri { get; set; }
    public string? Folder { get; set; }
    public bool? Scalable { get; set; }
    public QualityKind Kind { get; set; } = QualityKind.General;
    public string? SI { get; set; }
    public string? Imperial { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public double? Default { get; set; }
    public string? Formula { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string? Unit { get; set; }
}

public class PortMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public int? MaxChildren { get; set; }
}

public class ModelMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public FileId File { get; set; } = new();
    public string? Description { get; set; }
}

public class ConnectorMeta
{
    public string Guid { get; set; } = "";
    public string? Name { get; set; }
    public double T { get; set; } = 0;
    public Point? Point { get; set; }
    public Vector? Direction { get; set; }
    public string? Description { get; set; }
    public PortId? Port { get; set; }
    public bool? Mandatory { get; set; }
    public int? MaxChildren { get; set; }
}

public class LayerMeta
{
    public string Guid { get; set; } = "";
    public string Path { get; set; } = "";
    public bool IsHidden { get; set; } = false;
    public bool IsLocked { get; set; } = false;
    public string Color { get; set; } = "";
    public string? Description { get; set; }
}

public class PieceMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Description { get; set; }
    public TypeId? Type { get; set; }
    public DesignId? Design { get; set; }
    public Plane? Plane { get; set; }
    public Coord? Center { get; set; }
    public double? Scale { get; set; }
    public Plane? MirrorPlane { get; set; }
    public bool? IsHidden { get; set; }
    public bool? IsLocked { get; set; }
    public string? Color { get; set; }
}

public class GroupMeta
{
    public string Guid { get; set; } = "";
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Color { get; set; }
}

public class ConnectionMeta
{
    public string Guid { get; set; } = "";
    public Side Connected { get; set; } = new();
    public Side Connecting { get; set; } = new();
    public string? Description { get; set; }
    public double Gap { get; set; } = 0;
    public double Shift { get; set; } = 0;
    public double Rise { get; set; } = 0;
    public double Rotation { get; set; } = 0;
    public double Turn { get; set; } = 0;
    public double Tilt { get; set; } = 0;
    public double? U { get; set; }
    public double? V { get; set; }
}

public class StatMeta
{
    public string Guid { get; set; } = "";
    public QualityId Quality { get; set; } = new();
    public string? Unit { get; set; }
    public double? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public double? Max { get; set; }
    public bool? MaxExcluded { get; set; }
}

#endregion 🎼Sub-entity Meta

#region 🪁TypeMetaShallow

public class TypeMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public TypeId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public int? Stock { get; set; }
    public bool? Virtual { get; set; }
    public string? Uri { get; set; }
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
}

public class TypeShallow
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public TypeId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public int? Stock { get; set; }
    public bool? Virtual { get; set; }
    public string? Uri { get; set; }
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
    public List<ModelMeta> Models { get; set; } = new();
    public List<ConnectorMeta> Connectors { get; set; } = new();
    public List<PropMeta> Props { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public List<ConceptId> Concepts { get; set; } = new();
    public List<AttributeMeta> Attributes { get; set; } = new();
}

#endregion 🪁TypeMetaShallow

#region ✨DesignMetaShallow

public class DesignMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public DesignId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public Location? Location { get; set; }
    public string Unit { get; set; } = "";
    public bool? CanScale { get; set; }
    public bool? CanMirror { get; set; }
    public LayerId? ActiveLayer { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
}

public class DesignShallow
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public DesignId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public Location? Location { get; set; }
    public string Unit { get; set; } = "";
    public bool? CanScale { get; set; }
    public bool? CanMirror { get; set; }
    public LayerId? ActiveLayer { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
    public List<PieceMeta> Pieces { get; set; } = new();
    public List<ConnectionMeta> Connections { get; set; } = new();
    public List<StatMeta> Stats { get; set; } = new();
    public List<PropMeta> Props { get; set; } = new();
    public List<LayerMeta> Layers { get; set; } = new();
    public List<GroupMeta> Groups { get; set; } = new();
    public List<AttributeMeta> Attributes { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public List<ConceptId> Concepts { get; set; } = new();
}

#endregion ✨DesignMetaShallow

#region 🏗️KitMetaShallow

public class KitMeta
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Version { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string Remote { get; set; } = "";
    public string Homepage { get; set; } = "";
    public string License { get; set; } = "";
    public string Preview { get; set; } = "";
    public string CreatedAt { get; set; } = "";
    public string UpdatedAt { get; set; } = "";
}

public class KitShallow
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Version { get; set; } = "";
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string Remote { get; set; } = "";
    public string Homepage { get; set; } = "";
    public string License { get; set; } = "";
    public string Preview { get; set; } = "";
    public string CreatedAt { get; set; } = "";
    public string UpdatedAt { get; set; } = "";
    public List<TypeMeta> Types { get; set; } = new();
    public List<DesignMeta> Designs { get; set; } = new();
    public List<TagMeta> Tags { get; set; } = new();
    public List<ConceptMeta> Concepts { get; set; } = new();
    public List<PortMeta> Ports { get; set; } = new();
    public List<QualityMeta> Qualities { get; set; } = new();
    public List<FileMeta> Files { get; set; } = new();
    public List<FolderMeta> Folders { get; set; } = new();
    public List<AuthorMeta> Authors { get; set; } = new();
    public List<AttributeMeta> Attributes { get; set; } = new();
}

#endregion 🏗️KitMetaShallow

#region 📎Meta And Shallow Conversions

public static class MetaShallowConversions
{
    public static AttributeMeta ToMeta(this Attribute a) => new()
    {
        Guid = a.Guid,
        Key = a.Key,
        Value = a.Value,
        Definition = a.Definition
    };

    public static PropMeta ToMeta(this Prop p) => new()
    {
        Guid = p.Guid,
        Quality = p.Quality,
        Value = p.Value,
        Unit = p.Unit
    };

    public static TagMeta ToMeta(this Tag t) => new()
    {
        Guid = t.Guid,
        Name = t.Name,
        Description = t.Description,
        Icon = t.Icon
    };

    public static ConceptMeta ToMeta(this Concept c) => new()
    {
        Guid = c.Guid,
        Name = c.Name,
        Description = c.Description,
        Icon = c.Icon
    };

    public static AuthorMeta ToMeta(this Author a) => new()
    {
        Guid = a.Guid,
        Name = a.Name,
        Email = a.Email
    };

    public static FileMeta ToMeta(this File f) => new()
    {
        Guid = f.Guid,
        Name = f.Name,
        Remote = f.Remote,
        Folder = f.Folder,
        Size = f.Size,
        Hash = f.Hash,
        CreatedAt = f.CreatedAt,
        CreatedBy = f.CreatedBy,
        UpdatedAt = f.UpdatedAt,
        UpdatedBy = f.UpdatedBy
    };

    public static FolderMeta ToMeta(this Folder f) => new()
    {
        Guid = f.Guid,
        Name = f.Name,
        Parent = f.Parent,
        Description = f.Description,
        CreatedAt = f.CreatedAt,
        CreatedBy = f.CreatedBy,
        UpdatedAt = f.UpdatedAt,
        UpdatedBy = f.UpdatedBy
    };

    public static QualityMeta ToMeta(this Quality q) => new()
    {
        Guid = q.Guid,
        Key = q.Key,
        Name = q.Name,
        Description = q.Description,
        Uri = q.Uri,
        Folder = q.Folder,
        Scalable = q.Scalable,
        Kind = q.Kind,
        SI = q.SI,
        Imperial = q.Imperial,
        Min = q.Min,
        MinExcluded = q.MinExcluded,
        Max = q.Max,
        MaxExcluded = q.MaxExcluded,
        Default = q.Default,
        Formula = q.Formula,
        Icon = q.Icon,
        Image = q.Image,
        Unit = q.Unit
    };

    public static PortMeta ToMeta(this Port p) => new()
    {
        Guid = p.Guid,
        Name = p.Name,
        Description = p.Description,
        Icon = p.Icon
    };

    public static ModelMeta ToMeta(this Model m) => new()
    {
        Guid = m.Guid,
        Name = m.Name,
        File = m.File,
        Description = m.Description
    };

    public static ConnectorMeta ToMeta(this Connector c) => new()
    {
        Guid = c.Guid,
        Name = c.Name,
        T = c.T,
        Point = c.Point,
        Direction = c.Direction,
        Description = c.Description,
        Port = c.Port,
        Mandatory = c.Mandatory
    };

    public static LayerMeta ToMeta(this Layer l) => new()
    {
        Guid = l.Guid,
        Path = l.Path,
        IsHidden = l.IsHidden ?? false,
        IsLocked = l.IsLocked ?? false,
        Color = l.Color ?? "",
        Description = l.Description
    };

    public static PieceMeta ToMeta(this Piece p) => new()
    {
        Guid = p.Guid,
        Name = p.Name,
        Description = p.Description,
        Type = p.Type,
        Design = p.Design,
        Plane = p.Plane,
        Center = p.Center,
        Scale = p.Scale,
        MirrorPlane = p.MirrorPlane,
        IsHidden = p.IsHidden,
        IsLocked = p.IsLocked,
        Color = p.Color
    };

    public static GroupMeta ToMeta(this Group g) => new()
    {
        Guid = g.Guid,
        Name = g.Name,
        Description = g.Description,
        Color = g.Color
    };

    public static ConnectionMeta ToMeta(this Connection c) => new()
    {
        Guid = c.Guid,
        Connected = c.Connected,
        Connecting = c.Connecting,
        Description = c.Description,
        Gap = c.Gap,
        Shift = c.Shift,
        Rise = c.Rise,
        Rotation = c.Rotation,
        Turn = c.Turn,
        Tilt = c.Tilt,
        U = c.U,
        V = c.V
    };

    public static StatMeta ToMeta(this Stat s) => new()
    {
        Guid = s.Guid,
        Quality = s.Quality,
        Unit = s.Unit,
        Min = s.Min,
        MinExcluded = s.MinExcluded,
        Max = s.Max,
        MaxExcluded = s.MaxExcluded
    };

    public static TypeMeta ToMeta(this Type t) => new()
    {
        Guid = t.Guid,
        Name = t.Name,
        Parent = t.Parent,
        IsAbstract = t.IsAbstract,
        Folder = t.Folder,
        Description = t.Description,
        Icon = t.Icon,
        Image = t.Image,
        Stock = t.Stock,
        Virtual = t.Virtual,
        Uri = t.Uri,
        Location = t.Location,
        Unit = t.Unit,
        CreatedAt = t.CreatedAt,
        UpdatedAt = t.UpdatedAt
    };

    public static TypeShallow ToShallow(this Type t) => new()
    {
        Guid = t.Guid,
        Name = t.Name,
        Parent = t.Parent,
        IsAbstract = t.IsAbstract,
        Folder = t.Folder,
        Description = t.Description,
        Icon = t.Icon,
        Image = t.Image,
        Stock = t.Stock,
        Virtual = t.Virtual,
        Uri = t.Uri,
        Location = t.Location,
        Unit = t.Unit,
        CreatedAt = t.CreatedAt,
        UpdatedAt = t.UpdatedAt,
        Models = t.Models.Select(m => m.ToMeta()).ToList(),
        Connectors = t.Connectors.Select(c => c.ToMeta()).ToList(),
        Props = t.Props.Select(p => p.ToMeta()).ToList(),
        Authors = t.Authors,
        Concepts = t.Concepts,
        Attributes = t.Attributes.Select(a => a.ToMeta()).ToList()
    };

    public static DesignMeta ToMeta(this Design d) => new()
    {
        Guid = d.Guid,
        Name = d.Name,
        Parent = d.Parent,
        IsAbstract = d.IsAbstract,
        Folder = d.Folder,
        Description = d.Description,
        Icon = d.Icon,
        Image = d.Image,
        Location = d.Location,
        Unit = d.Unit,
        CanScale = d.CanScale,
        CanMirror = d.CanMirror,
        ActiveLayer = d.ActiveLayer,
        CreatedAt = d.CreatedAt,
        UpdatedAt = d.UpdatedAt
    };

    public static DesignShallow ToShallow(this Design d) => new()
    {
        Guid = d.Guid,
        Name = d.Name,
        Parent = d.Parent,
        IsAbstract = d.IsAbstract,
        Folder = d.Folder,
        Description = d.Description,
        Icon = d.Icon,
        Image = d.Image,
        Location = d.Location,
        Unit = d.Unit,
        CanScale = d.CanScale,
        CanMirror = d.CanMirror,
        ActiveLayer = d.ActiveLayer,
        CreatedAt = d.CreatedAt,
        UpdatedAt = d.UpdatedAt,
        Pieces = d.Pieces.Select(p => p.ToMeta()).ToList(),
        Connections = d.Connections.Select(c => c.ToMeta()).ToList(),
        Stats = d.Stats.Select(s => s.ToMeta()).ToList(),
        Props = d.Props.Select(p => p.ToMeta()).ToList(),
        Layers = d.Layers.Select(l => l.ToMeta()).ToList(),
        Groups = d.Groups.Select(g => g.ToMeta()).ToList(),
        Attributes = d.Attributes.Select(a => a.ToMeta()).ToList(),
        Authors = d.Authors,
        Concepts = d.Concepts
    };

    public static KitMeta ToMeta(this Kit k) => new()
    {
        Guid = k.Guid,
        Name = k.Name,
        Version = k.Version,
        Description = k.Description,
        Icon = k.Icon,
        Image = k.Image,
        Remote = k.Remote,
        Homepage = k.Homepage,
        License = k.License,
        Preview = k.Preview,
        CreatedAt = k.CreatedAt,
        UpdatedAt = k.UpdatedAt
    };

    public static KitShallow ToShallow(this Kit k) => new()
    {
        Guid = k.Guid,
        Name = k.Name,
        Version = k.Version,
        Description = k.Description,
        Icon = k.Icon,
        Image = k.Image,
        Remote = k.Remote,
        Homepage = k.Homepage,
        License = k.License,
        Preview = k.Preview,
        CreatedAt = k.CreatedAt,
        UpdatedAt = k.UpdatedAt,
        Types = k.Types.Select(t => t.ToMeta()).ToList(),
        Designs = k.Designs.Select(d => d.ToMeta()).ToList(),
        Tags = k.Tags.Select(t => t.ToMeta()).ToList(),
        Concepts = k.Concepts.Select(c => c.ToMeta()).ToList(),
        Ports = k.Ports.Select(p => p.ToMeta()).ToList(),
        Qualities = k.Qualities.Select(q => q.ToMeta()).ToList(),
        Files = k.Files.Select(f => f.ToMeta()).ToList(),
        Folders = k.Folders.Select(f => f.ToMeta()).ToList(),
        Authors = k.Authors.Select(a => a.ToMeta()).ToList(),
        Attributes = k.Attributes.Select(a => a.ToMeta()).ToList()
    };
}

#endregion 📎Meta And Shallow Conversions

#endregion 🔑Meta And Shallow






#region 🖥️Hash
// Merkle hash functions for all entities. Each hash function computes a deterministic
// SHA-256 hex digest. Collections are hashed by sorting child hashes alphabetically.
// Field order is alphabetical by JSON field name. Missing/null/empty fields are skipped.

public static class Hashing
{
    private class HashWriter
    {
        private readonly List<byte> _buf = new();

        public void WriteString(string s)
        {
            var bytes = Encoding.UTF8.GetBytes(s);
            var lb = BitConverter.GetBytes((uint)bytes.Length);
            if (BitConverter.IsLittleEndian) Array.Reverse(lb);
            _buf.AddRange(lb);
            _buf.AddRange(bytes);
        }

        public void WriteNumber(double n) => WriteString(FormatNumberForHash(n));

        public void WriteIntNumber(int n) => WriteString(n.ToString());

        public void WriteBool(bool b) => _buf.Add(b ? (byte)1 : (byte)0);

        public void WriteHash(string h) => WriteString(h);

        public void WriteHashList(List<string> hashes)
        {
            var sorted = hashes.OrderBy(h => h, StringComparer.Ordinal).ToList();
            var lb = BitConverter.GetBytes((uint)sorted.Count);
            if (BitConverter.IsLittleEndian) Array.Reverse(lb);
            _buf.AddRange(lb);
            foreach (var h in sorted)
                WriteString(h);
        }

        public void WriteGuidList(List<string> guids)
        {
            var sorted = guids.OrderBy(g => g, StringComparer.Ordinal).ToList();
            var lb = BitConverter.GetBytes((uint)sorted.Count);
            if (BitConverter.IsLittleEndian) Array.Reverse(lb);
            _buf.AddRange(lb);
            foreach (var g in sorted)
                WriteString(g);
        }

        public string Digest()
        {
            using var sha256 = System.Security.Cryptography.SHA256.Create();
            var hash = sha256.ComputeHash(_buf.ToArray());
            return BitConverter.ToString(hash).Replace("-", "").ToLowerInvariant();
        }
    }

    public static string FormatNumberForHash(double n)
    {
        if (n == Math.Truncate(n) && !double.IsInfinity(n) && Math.Abs(n) < 1e15)
            return ((long)n).ToString(CultureInfo.InvariantCulture);
        var abs = Math.Abs(n);
        var roundTrip = n.ToString("R", CultureInfo.InvariantCulture);
        if (abs > 0 && (abs < 1e-6 || abs >= 1e21))
        {
            return NormalizeScientificNumber(roundTrip);
        }
        return roundTrip.IndexOfAny(new[] { 'E', 'e' }) >= 0
            ? ExpandScientificNumber(roundTrip)
            : TrimFractionZeros(roundTrip);
    }

    private static string TrimFractionZeros(string value)
    {
        if (!value.Contains('.'))
            return value == "-0" ? "0" : value;

        var trimmed = value.TrimEnd('0').TrimEnd('.');
        return trimmed == "-0" ? "0" : trimmed;
    }

    private static string NormalizeScientificNumber(string value)
    {
        var exponentIndex = value.IndexOfAny(new[] { 'E', 'e' });
        if (exponentIndex < 0)
            return value;

        var mantissa = TrimFractionZeros(value[..exponentIndex]);
        var exponent = value[(exponentIndex + 1)..];
        var sign = exponent.StartsWith("-", StringComparison.Ordinal) ? "-" : "+";
        var digits = exponent.TrimStart('+', '-').TrimStart('0');
        if (digits == "")
            digits = "0";

        return $"{mantissa}e{sign}{digits}";
    }

    private static string ExpandScientificNumber(string value)
    {
        var exponentIndex = value.IndexOfAny(new[] { 'E', 'e' });
        if (exponentIndex < 0)
            return TrimFractionZeros(value);

        var mantissa = value[..exponentIndex];
        var prefix = "";
        if (mantissa.StartsWith("-", StringComparison.Ordinal))
        {
            prefix = "-";
            mantissa = mantissa[1..];
        }
        else if (mantissa.StartsWith("+", StringComparison.Ordinal))
        {
            mantissa = mantissa[1..];
        }

        var exponent = int.Parse(value[(exponentIndex + 1)..], CultureInfo.InvariantCulture);
        var decimalIndex = mantissa.IndexOf('.');
        var digits = mantissa.Replace(".", "");
        if (decimalIndex < 0)
            decimalIndex = digits.Length;

        var pointIndex = decimalIndex + exponent;
        string expanded;
        if (pointIndex <= 0)
        {
            expanded = "0." + new string('0', -pointIndex) + digits;
        }
        else if (pointIndex >= digits.Length)
        {
            expanded = digits + new string('0', pointIndex - digits.Length);
        }
        else
        {
            expanded = digits.Insert(pointIndex, ".");
        }

        expanded = TrimFractionZeros(expanded);
        if (expanded.StartsWith(".", StringComparison.Ordinal))
            expanded = "0" + expanded;
        if (expanded == "0")
            return expanded;

        return prefix + expanded;
    }

    public static string HashCoord(Coord c)
    {
        var w = new HashWriter();
        w.WriteString("Coord");
        w.WriteString("u");
        w.WriteNumber(c.U);
        w.WriteString("v");
        w.WriteNumber(c.V);
        return w.Digest();
    }

    public static string HashPoint(Point p)
    {
        var w = new HashWriter();
        w.WriteString("Point");
        w.WriteString("x");
        w.WriteNumber(p.X);
        w.WriteString("y");
        w.WriteNumber(p.Y);
        w.WriteString("z");
        w.WriteNumber(p.Z);
        return w.Digest();
    }

    public static string HashVector(Vector v)
    {
        var w = new HashWriter();
        w.WriteString("Vector");
        w.WriteString("x");
        w.WriteNumber(v.X);
        w.WriteString("y");
        w.WriteNumber(v.Y);
        w.WriteString("z");
        w.WriteNumber(v.Z);
        return w.Digest();
    }

    public static string HashPlane(Plane p)
    {
        var w = new HashWriter();
        w.WriteString("Plane");
        w.WriteString("origin");
        w.WriteHash(HashPoint(p.Origin));
        w.WriteString("xAxis");
        w.WriteHash(HashVector(p.XAxis));
        w.WriteString("yAxis");
        w.WriteHash(HashVector(p.YAxis));
        return w.Digest();
    }

    public static string HashAttribute(Attribute a)
    {
        var w = new HashWriter();
        w.WriteString("Attribute");
        if (a.Definition != null)
        {
            w.WriteString("definition");
            w.WriteString(a.Definition);
        }
        w.WriteString("guid");
        w.WriteString(a.Guid);
        w.WriteString("key");
        w.WriteString(a.Key);
        if (a.Value != null)
        {
            w.WriteString("value");
            w.WriteString(a.Value);
        }
        return w.Digest();
    }

    public static string HashLocation(Location l)
    {
        var w = new HashWriter();
        w.WriteString("Location");
        if (l.Altitude.HasValue)
        {
            w.WriteString("altitude");
            w.WriteNumber(l.Altitude.Value);
        }
        if (l.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(l.Attributes.Select(HashAttribute).ToList());
        }
        w.WriteString("guid");
        w.WriteString(l.Guid);
        w.WriteString("latitude");
        w.WriteNumber(l.Latitude);
        w.WriteString("longitude");
        w.WriteNumber(l.Longitude);
        return w.Digest();
    }

    public static string HashAuthor(Author a)
    {
        var w = new HashWriter();
        w.WriteString("Author");
        if (a.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(a.Attributes.Select(HashAttribute).ToList());
        }
        if (!string.IsNullOrEmpty(a.Email))
        {
            w.WriteString("email");
            w.WriteString(a.Email);
        }
        w.WriteString("guid");
        w.WriteString(a.Guid);
        w.WriteString("name");
        w.WriteString(a.Name);
        return w.Digest();
    }

    public static string HashFile(File f)
    {
        var w = new HashWriter();
        w.WriteString("File");
        if (f.Blob != null)
        {
            w.WriteString("blob");
            w.WriteString(f.Blob);
        }
        if (f.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(f.Folder.Guid);
        }
        w.WriteString("guid");
        w.WriteString(f.Guid);
        if (f.Hash != null)
        {
            w.WriteString("hash");
            w.WriteString(f.Hash);
        }
        w.WriteString("name");
        w.WriteString(f.Name);
        if (f.Remote != null)
        {
            w.WriteString("remote");
            w.WriteString(f.Remote);
        }
        if (f.Size.HasValue)
        {
            w.WriteString("size");
            w.WriteIntNumber(f.Size.Value);
        }
        return w.Digest();
    }

    public static string HashFolder(Folder f)
    {
        var w = new HashWriter();
        w.WriteString("Folder");
        if (f.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(f.Attributes.Select(HashAttribute).ToList());
        }
        if (f.Description != null)
        {
            w.WriteString("description");
            w.WriteString(f.Description);
        }
        w.WriteString("guid");
        w.WriteString(f.Guid);
        w.WriteString("name");
        w.WriteString(f.Name);
        if (f.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(f.Parent.Guid);
        }
        return w.Digest();
    }

    public static string HashBenchmark(Benchmark b)
    {
        var w = new HashWriter();
        w.WriteString("Benchmark");
        if (b.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(b.Attributes.Select(HashAttribute).ToList());
        }
        w.WriteString("guid");
        w.WriteString(b.Guid);
        if (b.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(b.Icon);
        }
        if (b.Max.HasValue)
        {
            w.WriteString("max");
            w.WriteNumber(b.Max.Value);
        }
        if (b.MaxExcluded.HasValue)
        {
            w.WriteString("maxExcluded");
            w.WriteBool(b.MaxExcluded.Value);
        }
        if (b.Min.HasValue)
        {
            w.WriteString("min");
            w.WriteNumber(b.Min.Value);
        }
        if (b.MinExcluded.HasValue)
        {
            w.WriteString("minExcluded");
            w.WriteBool(b.MinExcluded.Value);
        }
        w.WriteString("name");
        w.WriteString(b.Name);
        return w.Digest();
    }

    public static string HashQuality(Quality q)
    {
        var w = new HashWriter();
        w.WriteString("Quality");
        if (q.Benchmarks?.Count > 0)
        {
            w.WriteString("benchmarks");
            w.WriteHashList(q.Benchmarks.Select(HashBenchmark).ToList());
        }
        if (q.Scalable.HasValue)
        {
            w.WriteString("canScale");
            w.WriteBool(q.Scalable.Value);
        }
        if (!string.IsNullOrEmpty(q.Imperial))
        {
            w.WriteString("defaultImperialUnit");
            w.WriteString(q.Imperial);
        }
        if (!string.IsNullOrEmpty(q.SI))
        {
            w.WriteString("defaultSiUnit");
            w.WriteString(q.SI);
        }
        if (q.Default.HasValue)
        {
            w.WriteString("defaultValue");
            w.WriteNumber(q.Default.Value);
        }
        if (q.Description != null)
        {
            w.WriteString("description");
            w.WriteString(q.Description);
        }
        if (!string.IsNullOrEmpty(q.Formula))
        {
            w.WriteString("formula");
            w.WriteString(q.Formula);
        }
        w.WriteString("guid");
        w.WriteString(q.Guid);
        if (q.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(q.Icon);
        }
        if (q.Image != null)
        {
            w.WriteString("image");
            w.WriteString(q.Image);
        }
        if (q.MaxExcluded.HasValue)
        {
            w.WriteString("isMaxExcluded");
            w.WriteBool(q.MaxExcluded.Value);
        }
        if (q.MinExcluded.HasValue)
        {
            w.WriteString("isMinExcluded");
            w.WriteBool(q.MinExcluded.Value);
        }
        w.WriteString("key");
        w.WriteString(q.Key);
        if (q.Kind != QualityKind.General)
        {
            w.WriteString("kind");
            w.WriteIntNumber((int)q.Kind);
        }
        if (q.Max.HasValue)
        {
            w.WriteString("max");
            w.WriteNumber(q.Max.Value);
        }
        if (q.Min.HasValue)
        {
            w.WriteString("min");
            w.WriteNumber(q.Min.Value);
        }
        w.WriteString("name");
        w.WriteString(q.Name);
        if (q.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(q.Unit);
        }
        if (!string.IsNullOrEmpty(q.Uri))
        {
            w.WriteString("uri");
            w.WriteString(q.Uri);
        }
        return w.Digest();
    }

    public static string HashPort(Port p)
    {
        var w = new HashWriter();
        w.WriteString("Port");
        if (p.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(p.Attributes.Select(HashAttribute).ToList());
        }
        if (p.CompatiblePorts?.Count > 0)
        {
            w.WriteString("compatiblePorts");
            w.WriteGuidList(p.CompatiblePorts.Select(cp => cp.Guid).ToList());
        }
        if (p.Description != null)
        {
            w.WriteString("description");
            w.WriteString(p.Description);
        }
        w.WriteString("guid");
        w.WriteString(p.Guid);
        if (p.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(p.Icon);
        }
        w.WriteString("name");
        w.WriteString(p.Name);
        return w.Digest();
    }

    public static string HashProp(Prop p)
    {
        var w = new HashWriter();
        w.WriteString("Prop");
        if (p.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(p.Attributes.Select(HashAttribute).ToList());
        }
        w.WriteString("guid");
        w.WriteString(p.Guid);
        w.WriteString("quality");
        w.WriteString(p.Quality.Guid);
        if (p.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(p.Unit);
        }
        w.WriteString("value");
        w.WriteString(p.Value);
        return w.Digest();
    }

    public static string HashTag(Tag t)
    {
        var w = new HashWriter();
        w.WriteString("Tag");
        if (t.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(t.Attributes.Select(HashAttribute).ToList());
        }
        if (t.Description != null)
        {
            w.WriteString("description");
            w.WriteString(t.Description);
        }
        w.WriteString("guid");
        w.WriteString(t.Guid);
        if (t.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(t.Icon);
        }
        w.WriteString("name");
        w.WriteString(t.Name);
        return w.Digest();
    }

    public static string HashConcept(Concept c)
    {
        var w = new HashWriter();
        w.WriteString("Concept");
        if (c.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(c.Attributes.Select(HashAttribute).ToList());
        }
        if (c.Description != null)
        {
            w.WriteString("description");
            w.WriteString(c.Description);
        }
        w.WriteString("guid");
        w.WriteString(c.Guid);
        if (c.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(c.Icon);
        }
        w.WriteString("name");
        w.WriteString(c.Name);
        return w.Digest();
    }

    public static string HashModel(Model m)
    {
        var w = new HashWriter();
        w.WriteString("Model");
        if (m.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(m.Attributes.Select(HashAttribute).ToList());
        }
        if (m.Description != null)
        {
            w.WriteString("description");
            w.WriteString(m.Description);
        }
        w.WriteString("file");
        w.WriteString(m.File.Guid);
        w.WriteString("guid");
        w.WriteString(m.Guid);
        if (m.Name != null)
        {
            w.WriteString("name");
            w.WriteString(m.Name);
        }
        if (m.Tags?.Count > 0)
        {
            w.WriteString("tags");
            w.WriteGuidList(m.Tags.Select(t => t.Guid).ToList());
        }
        return w.Digest();
    }

    public static string HashConnector(Connector c)
    {
        var w = new HashWriter();
        w.WriteString("Connector");
        if (c.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(c.Attributes.Select(HashAttribute).ToList());
        }
        if (c.Description != null)
        {
            w.WriteString("description");
            w.WriteString(c.Description);
        }
        w.WriteString("direction");
        w.WriteHash(HashVector(c.Direction ?? new Vector()));
        w.WriteString("guid");
        w.WriteString(c.Guid);
        if (c.Mandatory.HasValue)
        {
            w.WriteString("mandatory");
            w.WriteBool(c.Mandatory.Value);
        }
        if (!string.IsNullOrEmpty(c.Name))
        {
            w.WriteString("name");
            w.WriteString(c.Name);
        }
        w.WriteString("point");
        w.WriteHash(HashPoint(c.Point ?? new Point()));
        if (c.Port != null)
        {
            w.WriteString("port");
            w.WriteString(c.Port.Guid);
        }
        if (c.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(c.Props.Select(HashProp).ToList());
        }
        w.WriteString("t");
        w.WriteNumber(c.T);
        return w.Digest();
    }

    public static string HashType(Type t)
    {
        var w = new HashWriter();
        w.WriteString("Type");
        if (t.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(t.Attributes.Select(HashAttribute).ToList());
        }
        if (t.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteGuidList(t.Authors.Select(a => a.Guid).ToList());
        }
        if (t.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteGuidList(t.Concepts.Select(c => c.Guid).ToList());
        }
        if (t.Connectors?.Count > 0)
        {
            w.WriteString("connectors");
            w.WriteHashList(t.Connectors.Select(HashConnector).ToList());
        }
        if (t.Description != null)
        {
            w.WriteString("description");
            w.WriteString(t.Description);
        }
        if (t.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(t.Folder);
        }
        w.WriteString("guid");
        w.WriteString(t.Guid);
        if (t.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(t.Icon);
        }
        if (t.Image != null)
        {
            w.WriteString("image");
            w.WriteString(t.Image);
        }
        if (t.IsAbstract.HasValue)
        {
            w.WriteString("isAbstract");
            w.WriteBool(t.IsAbstract.Value);
        }
        if (t.Location != null)
        {
            w.WriteString("location");
            w.WriteString(t.Location.Guid);
        }
        if (t.Models?.Count > 0)
        {
            w.WriteString("models");
            w.WriteHashList(t.Models.Select(HashModel).ToList());
        }
        w.WriteString("name");
        w.WriteString(t.Name);
        if (t.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(t.Parent.Guid);
        }
        if (t.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(t.Props.Select(HashProp).ToList());
        }
        if (t.Stock.HasValue)
        {
            w.WriteString("stock");
            w.WriteIntNumber(t.Stock.Value);
        }
        if (t.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(t.Unit);
        }
        if (t.Virtual.HasValue)
        {
            w.WriteString("virtual");
            w.WriteBool(t.Virtual.Value);
        }
        return w.Digest();
    }

    public static string HashLayer(Layer l)
    {
        var w = new HashWriter();
        w.WriteString("Layer");
        if (l.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(l.Attributes.Select(HashAttribute).ToList());
        }
        if (l.Color != null)
        {
            w.WriteString("color");
            w.WriteString(l.Color);
        }
        if (l.Description != null)
        {
            w.WriteString("description");
            w.WriteString(l.Description);
        }
        w.WriteString("guid");
        w.WriteString(l.Guid);
        if (l.IsHidden.HasValue)
        {
            w.WriteString("isHidden");
            w.WriteBool(l.IsHidden.Value);
        }
        if (l.IsLocked.HasValue)
        {
            w.WriteString("isLocked");
            w.WriteBool(l.IsLocked.Value);
        }
        w.WriteString("path");
        w.WriteString(l.Path);
        return w.Digest();
    }

    public static string HashStat(Stat s)
    {
        var w = new HashWriter();
        w.WriteString("Stat");
        w.WriteString("guid");
        w.WriteString(s.Guid);
        if (s.Max.HasValue)
        {
            w.WriteString("max");
            w.WriteNumber(s.Max.Value);
        }
        if (s.MaxExcluded.HasValue)
        {
            w.WriteString("maxExcluded");
            w.WriteBool(s.MaxExcluded.Value);
        }
        if (s.Min.HasValue)
        {
            w.WriteString("min");
            w.WriteNumber(s.Min.Value);
        }
        if (s.MinExcluded.HasValue)
        {
            w.WriteString("minExcluded");
            w.WriteBool(s.MinExcluded.Value);
        }
        w.WriteString("quality");
        w.WriteString(s.Quality.Guid);
        if (s.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(s.Unit);
        }
        return w.Digest();
    }

    public static string HashGroup(Group g)
    {
        var w = new HashWriter();
        w.WriteString("Group");
        if (g.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(g.Attributes.Select(HashAttribute).ToList());
        }
        if (g.Color != null)
        {
            w.WriteString("color");
            w.WriteString(g.Color);
        }
        if (g.Description != null)
        {
            w.WriteString("description");
            w.WriteString(g.Description);
        }
        w.WriteString("guid");
        w.WriteString(g.Guid);
        if (g.Name != null)
        {
            w.WriteString("name");
            w.WriteString(g.Name);
        }
        w.WriteString("pieces");
        w.WriteGuidList(g.Pieces?.Select(p => p.Guid).ToList() ?? new List<string>());
        return w.Digest();
    }

    public static string HashSide(Side s)
    {
        var w = new HashWriter();
        w.WriteString("Side");
        if (s.Connector != null)
        {
            w.WriteString("connector");
            w.WriteString(s.Connector.Guid);
        }
        if (s.DesignPiece != null)
        {
            w.WriteString("designPiece");
            w.WriteString(s.DesignPiece.Guid);
        }
        w.WriteString("piece");
        w.WriteString(s.Piece.Guid);
        return w.Digest();
    }

    public static string HashConnection(Connection c)
    {
        var w = new HashWriter();
        w.WriteString("Connection");
        if (c.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(c.Attributes.Select(HashAttribute).ToList());
        }
        w.WriteString("connected");
        w.WriteHash(HashSide(c.Connected));
        w.WriteString("connecting");
        w.WriteHash(HashSide(c.Connecting));
        if (c.Description != null)
        {
            w.WriteString("description");
            w.WriteString(c.Description);
        }
        w.WriteString("gap");
        w.WriteNumber(c.Gap);
        w.WriteString("guid");
        w.WriteString(c.Guid);
        w.WriteString("rise");
        w.WriteNumber(c.Rise);
        w.WriteString("rotation");
        w.WriteNumber(c.Rotation);
        w.WriteString("shift");
        w.WriteNumber(c.Shift);
        w.WriteString("tilt");
        w.WriteNumber(c.Tilt);
        w.WriteString("turn");
        w.WriteNumber(c.Turn);
        w.WriteString("u");
        w.WriteNumber(c.U ?? 0);
        w.WriteString("v");
        w.WriteNumber(c.V ?? 0);
        return w.Digest();
    }

    public static string HashPiece(Piece p)
    {
        var w = new HashWriter();
        w.WriteString("Piece");
        if (p.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(p.Attributes.Select(HashAttribute).ToList());
        }
        if (p.Center != null)
        {
            w.WriteString("center");
            w.WriteHash(HashCoord(p.Center));
        }
        if (p.Color != null)
        {
            w.WriteString("color");
            w.WriteString(p.Color);
        }
        if (p.Description != null)
        {
            w.WriteString("description");
            w.WriteString(p.Description);
        }
        if (p.Design != null)
        {
            w.WriteString("design");
            w.WriteString(p.Design.Guid);
        }
        w.WriteString("guid");
        w.WriteString(p.Guid);
        if (p.IsHidden.HasValue)
        {
            w.WriteString("isHidden");
            w.WriteBool(p.IsHidden.Value);
        }
        if (p.IsLocked.HasValue)
        {
            w.WriteString("isLocked");
            w.WriteBool(p.IsLocked.Value);
        }
        if (p.MirrorPlane != null)
        {
            w.WriteString("mirrorPlane");
            w.WriteHash(HashPlane(p.MirrorPlane));
        }
        if (p.Name != null)
        {
            w.WriteString("name");
            w.WriteString(p.Name);
        }
        if (p.Plane != null)
        {
            w.WriteString("plane");
            w.WriteHash(HashPlane(p.Plane));
        }
        if (p.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(p.Props.Select(HashProp).ToList());
        }
        if (p.Scale.HasValue)
        {
            w.WriteString("scale");
            w.WriteNumber(p.Scale.Value);
        }
        if (p.Type != null)
        {
            w.WriteString("type");
            w.WriteString(p.Type.Guid);
        }
        return w.Digest();
    }

    public static string HashDesign(Design d)
    {
        var w = new HashWriter();
        w.WriteString("Design");
        if (d.ActiveLayer != null)
        {
            w.WriteString("activeLayer");
            w.WriteString(d.ActiveLayer.Guid);
        }
        if (d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        if (d.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteGuidList(d.Authors.Select(a => a.Guid).ToList());
        }
        if (d.CanMirror.HasValue)
        {
            w.WriteString("canMirror");
            w.WriteBool(d.CanMirror.Value);
        }
        if (d.CanScale.HasValue)
        {
            w.WriteString("canScale");
            w.WriteBool(d.CanScale.Value);
        }
        if (d.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteGuidList(d.Concepts.Select(c => c.Guid).ToList());
        }
        if (d.Connections?.Count > 0)
        {
            w.WriteString("connections");
            w.WriteHashList(d.Connections.Select(HashConnection).ToList());
        }
        if (d.Description != null)
        {
            w.WriteString("description");
            w.WriteString(d.Description);
        }
        if (d.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(d.Folder);
        }
        if (d.Groups?.Count > 0)
        {
            w.WriteString("groups");
            w.WriteHashList(d.Groups.Select(HashGroup).ToList());
        }
        w.WriteString("guid");
        w.WriteString(d.Guid);
        if (d.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(d.Icon);
        }
        if (d.Image != null)
        {
            w.WriteString("image");
            w.WriteString(d.Image);
        }
        if (d.IsAbstract.HasValue)
        {
            w.WriteString("isAbstract");
            w.WriteBool(d.IsAbstract.Value);
        }
        if (d.Layers?.Count > 0)
        {
            w.WriteString("layers");
            w.WriteHashList(d.Layers.Select(HashLayer).ToList());
        }
        if (d.Location != null)
        {
            w.WriteString("location");
            w.WriteString(d.Location.Guid);
        }
        w.WriteString("name");
        w.WriteString(d.Name);
        if (d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(d.Parent.Guid);
        }
        if (d.Pieces?.Count > 0)
        {
            w.WriteString("pieces");
            w.WriteHashList(d.Pieces.Select(HashPiece).ToList());
        }
        if (d.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(d.Props.Select(HashProp).ToList());
        }
        if (d.Stats?.Count > 0)
        {
            w.WriteString("stats");
            w.WriteHashList(d.Stats.Select(HashStat).ToList());
        }
        if (d.Unit != null)
        {
            w.WriteString("unit");
            w.WriteString(d.Unit);
        }
        return w.Digest();
    }

    public static string HashKit(Kit k)
    {
        var w = new HashWriter();
        w.WriteString("Kit");
        if (k.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(k.Attributes.Select(HashAttribute).ToList());
        }
        if (k.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteHashList(k.Authors.Select(HashAuthor).ToList());
        }
        if (k.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteHashList(k.Concepts.Select(HashConcept).ToList());
        }
        if (k.Description != null)
        {
            w.WriteString("description");
            w.WriteString(k.Description);
        }
        if (k.Designs?.Count > 0)
        {
            w.WriteString("designs");
            w.WriteHashList(k.Designs.Select(HashDesign).ToList());
        }
        if (k.Files?.Count > 0)
        {
            w.WriteString("files");
            w.WriteHashList(k.Files.Select(HashFile).ToList());
        }
        if (k.Folders?.Count > 0)
        {
            w.WriteString("folders");
            w.WriteHashList(k.Folders.Select(HashFolder).ToList());
        }
        w.WriteString("guid");
        w.WriteString(k.Guid);
        if (k.Homepage != null)
        {
            w.WriteString("homepage");
            w.WriteString(k.Homepage);
        }
        if (k.Icon != null)
        {
            w.WriteString("icon");
            w.WriteString(k.Icon);
        }
        if (k.Image != null)
        {
            w.WriteString("image");
            w.WriteString(k.Image);
        }
        if (k.License != null)
        {
            w.WriteString("license");
            w.WriteString(k.License);
        }
        w.WriteString("name");
        w.WriteString(k.Name);
        if (k.Ports?.Count > 0)
        {
            w.WriteString("ports");
            w.WriteHashList(k.Ports.Select(HashPort).ToList());
        }
        if (k.Preview != null)
        {
            w.WriteString("preview");
            w.WriteString(k.Preview);
        }
        if (k.Qualities?.Count > 0)
        {
            w.WriteString("qualities");
            w.WriteHashList(k.Qualities.Select(HashQuality).ToList());
        }
        if (k.Remote != null)
        {
            w.WriteString("remote");
            w.WriteString(k.Remote);
        }
        if (k.Tags?.Count > 0)
        {
            w.WriteString("tags");
            w.WriteHashList(k.Tags.Select(HashTag).ToList());
        }
        if (k.Types?.Count > 0)
        {
            w.WriteString("types");
            w.WriteHashList(k.Types.Select(HashType).ToList());
        }
        if (k.Version != null && k.Version != "")
        {
            w.WriteString("version");
            w.WriteString(k.Version);
        }
        return w.Digest();
    }

    // #region 🔗Hash Diffs
    // Deterministic SHA-256 Merkle hash functions for all diff types.
    // Fields are ordered alphabetically by JSON field name. Guid is excluded from
    // diff hashes. Null markers (field name + WriteBool(false)) indicate explicit
    // null. Absent/unset fields are skipped entirely.

    private static void WriteDiffNullableString(HashWriter w, string key, string? value, bool shouldSerialize)
    {
        if (value != null) { w.WriteString(key); w.WriteString(value); }
        else if (shouldSerialize) { w.WriteString(key); w.WriteBool(false); }
    }

    private static void WriteDiffOptString(HashWriter w, string key, string? value, bool shouldSerialize)
    {
        if (shouldSerialize && value != null) { w.WriteString(key); w.WriteString(value); }
    }

    private static void WriteDiffOptNumber(HashWriter w, string key, double? value, bool shouldSerialize)
    {
        if (shouldSerialize && value.HasValue) { w.WriteString(key); w.WriteNumber(value.Value); }
    }

    private static void WriteDiffOptIntNumber(HashWriter w, string key, int? value, bool shouldSerialize)
    {
        if (shouldSerialize && value.HasValue) { w.WriteString(key); w.WriteIntNumber(value.Value); }
    }

    private static void WriteDiffOptBool(HashWriter w, string key, bool? value, bool shouldSerialize)
    {
        if (shouldSerialize && value.HasValue) { w.WriteString(key); w.WriteBool(value.Value); }
    }

    private static string HashCollectionDiffGeneric<TEntity, TDiff>(
        string tag, string updateTag, string entityKeyName,
        Func<TEntity, string> hashEntityFn,
        Func<TDiff, string> hashDiffFn,
        List<string> removedGuids,
        List<(string key, TDiff diff)> updates,
        List<TEntity> added)
    {
        var w = new HashWriter();
        w.WriteString(tag);
        if (added?.Count > 0)
        {
            w.WriteString("added");
            w.WriteHashList(added.Select(hashEntityFn).ToList());
        }
        if (removedGuids?.Count > 0)
        {
            w.WriteString("removed");
            w.WriteGuidList(removedGuids);
        }
        if (updates?.Count > 0)
        {
            w.WriteString("updated");
            var keys = new List<string> { entityKeyName, "diff" };
            keys.Sort(StringComparer.Ordinal);
            var updateHashes = updates.Select(u =>
            {
                var uw = new HashWriter();
                uw.WriteString(updateTag);
                foreach (var k in keys)
                {
                    if (k == "diff")
                    {
                        uw.WriteString("diff");
                        uw.WriteHash(hashDiffFn(u.diff));
                    }
                    else
                    {
                        uw.WriteString(k);
                        uw.WriteString(u.key);
                    }
                }
                return uw.Digest();
            }).ToList();
            w.WriteHashList(updateHashes);
        }
        return w.Digest();
    }

    // #region 🐹Hash Diff Value Types

    public static string HashCoordDiff(Coord c)
    {
        var w = new HashWriter();
        w.WriteString("CoordDiff");
        w.WriteString("u");
        w.WriteNumber(c.U);
        w.WriteString("v");
        w.WriteNumber(c.V);
        return w.Digest();
    }

    public static string HashPointDiff(Point p)
    {
        var w = new HashWriter();
        w.WriteString("PointDiff");
        w.WriteString("x");
        w.WriteNumber(p.X);
        w.WriteString("y");
        w.WriteNumber(p.Y);
        w.WriteString("z");
        w.WriteNumber(p.Z);
        return w.Digest();
    }

    public static string HashVectorDiff(Vector v)
    {
        var w = new HashWriter();
        w.WriteString("VectorDiff");
        w.WriteString("x");
        w.WriteNumber(v.X);
        w.WriteString("y");
        w.WriteNumber(v.Y);
        w.WriteString("z");
        w.WriteNumber(v.Z);
        return w.Digest();
    }

    public static string HashPlaneDiff(Plane p)
    {
        var w = new HashWriter();
        w.WriteString("PlaneDiff");
        w.WriteString("origin");
        w.WriteHash(HashPointDiff(p.Origin));
        w.WriteString("xAxis");
        w.WriteHash(HashVectorDiff(p.XAxis));
        w.WriteString("yAxis");
        w.WriteHash(HashVectorDiff(p.YAxis));
        return w.Digest();
    }

    // #endregion 🐹Hash Diff Value Types

    // #region ⚗️Hash Diff Entities

    public static string HashAttributeDiff(AttributeDiff d)
    {
        var w = new HashWriter();
        w.WriteString("AttributeDiff");
        WriteDiffOptString(w, "definition", d.Definition, d.ShouldSerializeDefinition());
        WriteDiffOptString(w, "key", d.Key, d.ShouldSerializeKey());
        WriteDiffOptString(w, "value", d.Value, d.ShouldSerializeValue());
        return w.Digest();
    }

    public static string HashAttributesDiff(AttributesDiff d)
    {
        return HashCollectionDiffGeneric(
            "AttributesDiff", "AttributeDiffUpdate", "attribute",
            (Attribute a) => HashAttribute(a),
            (AttributeDiff ad) => HashAttributeDiff(ad),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Attribute.Guid, u.Diff!)).ToList() ?? new List<(string, AttributeDiff)>(),
            d.Added ?? new List<Attribute>());
    }

    public static string HashAuthorDiff(AuthorDiff d)
    {
        var w = new HashWriter();
        w.WriteString("AuthorDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        WriteDiffOptString(w, "email", d.Email, d.ShouldSerializeEmail());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashAuthorsDiff(AuthorsDiff d)
    {
        return HashCollectionDiffGeneric(
            "AuthorsDiff", "AuthorDiffUpdate", "author",
            (Author a) => HashAuthor(a),
            (AuthorDiff ad) => HashAuthorDiff(ad),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Author.Guid, u.Diff!)).ToList() ?? new List<(string, AuthorDiff)>(),
            d.Added ?? new List<Author>());
    }

    public static string HashFileDiff(FileDiff d)
    {
        var w = new HashWriter();
        w.WriteString("FileDiff");
        WriteDiffOptString(w, "blob", d.Blob, d.ShouldSerializeBlob());
        if (d.ShouldSerializeFolder() && d.Folder != null)
        {
            w.WriteString("folder");
            w.WriteString(d.Folder.Guid);
        }
        WriteDiffOptString(w, "hash", d.Hash, d.ShouldSerializeHash());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        WriteDiffOptString(w, "remote", d.Remote, d.ShouldSerializeRemote());
        WriteDiffOptIntNumber(w, "size", d.Size, d.ShouldSerializeSize());
        return w.Digest();
    }

    public static string HashFilesDiff(FilesDiff d)
    {
        return HashCollectionDiffGeneric(
            "FilesDiff", "FileDiffUpdate", "file",
            (File f) => HashFile(f),
            (FileDiff fd) => HashFileDiff(fd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.File.Guid, u.Diff!)).ToList() ?? new List<(string, FileDiff)>(),
            d.Added ?? new List<File>());
    }

    public static string HashFolderDiff(FolderDiff d)
    {
        var w = new HashWriter();
        w.WriteString("FolderDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializeParent() && d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(d.Parent.Guid);
        }
        return w.Digest();
    }

    public static string HashFoldersDiff(FoldersDiff d)
    {
        return HashCollectionDiffGeneric(
            "FoldersDiff", "FolderDiffUpdate", "folder",
            (Folder f) => HashFolder(f),
            (FolderDiff fd) => HashFolderDiff(fd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Folder.Guid, u.Diff!)).ToList() ?? new List<(string, FolderDiff)>(),
            d.Added ?? new List<Folder>());
    }

    public static string HashBenchmarkDiff(BenchmarkDiff d)
    {
        var w = new HashWriter();
        w.WriteString("BenchmarkDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptNumber(w, "max", d.Max, d.ShouldSerializeMax());
        WriteDiffOptBool(w, "maxExcluded", d.MaxExcluded, d.ShouldSerializeMaxExcluded());
        WriteDiffOptNumber(w, "min", d.Min, d.ShouldSerializeMin());
        WriteDiffOptBool(w, "minExcluded", d.MinExcluded, d.ShouldSerializeMinExcluded());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashQualityDiff(QualityDiff d)
    {
        var w = new HashWriter();
        w.WriteString("QualityDiff");
        if (d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        if (d.Benchmarks?.Count > 0)
        {
            w.WriteString("benchmarks");
            w.WriteHashList(d.Benchmarks.Select(HashBenchmark).ToList());
        }
        if (d.Scalable.HasValue)
        {
            w.WriteString("canScale");
            w.WriteBool(d.Scalable.Value);
        }
        if (!string.IsNullOrEmpty(d.Imperial))
        {
            w.WriteString("defaultImperialUnit");
            w.WriteString(d.Imperial);
        }
        if (!string.IsNullOrEmpty(d.SI))
        {
            w.WriteString("defaultSiUnit");
            w.WriteString(d.SI);
        }
        if (d.Default.HasValue)
        {
            w.WriteString("defaultValue");
            w.WriteNumber(d.Default.Value);
        }
        if (d.Description != null)
        {
            w.WriteString("description");
            w.WriteString(d.Description);
        }
        if (!string.IsNullOrEmpty(d.Formula))
        {
            w.WriteString("formula");
            w.WriteString(d.Formula);
        }
        if (d.MaxExcluded.HasValue)
        {
            w.WriteString("isMaxExcluded");
            w.WriteBool(d.MaxExcluded.Value);
        }
        if (d.MinExcluded.HasValue)
        {
            w.WriteString("isMinExcluded");
            w.WriteBool(d.MinExcluded.Value);
        }
        if (!string.IsNullOrEmpty(d.Key))
        {
            w.WriteString("key");
            w.WriteString(d.Key);
        }
        if (d.Kind != QualityKind.General)
        {
            w.WriteString("kind");
            w.WriteIntNumber((int)d.Kind);
        }
        if (d.Max.HasValue)
        {
            w.WriteString("max");
            w.WriteNumber(d.Max.Value);
        }
        if (d.Min.HasValue)
        {
            w.WriteString("min");
            w.WriteNumber(d.Min.Value);
        }
        if (!string.IsNullOrEmpty(d.Name))
        {
            w.WriteString("name");
            w.WriteString(d.Name);
        }
        if (!string.IsNullOrEmpty(d.Uri))
        {
            w.WriteString("uri");
            w.WriteString(d.Uri);
        }
        return w.Digest();
    }

    public static string HashTagDiff(TagDiff d)
    {
        var w = new HashWriter();
        w.WriteString("TagDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashTagsDiff(TagsDiff d)
    {
        return HashCollectionDiffGeneric(
            "TagsDiff", "TagDiffUpdate", "tag",
            (Tag t) => HashTag(t),
            (TagDiff td) => HashTagDiff(td),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Tag.Guid, u.Diff!)).ToList() ?? new List<(string, TagDiff)>(),
            d.Added ?? new List<Tag>());
    }

    public static string HashConceptDiff(ConceptDiff d)
    {
        var w = new HashWriter();
        w.WriteString("ConceptDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashConceptsDiff(ConceptsDiff d)
    {
        return HashCollectionDiffGeneric(
            "ConceptsDiff", "ConceptDiffUpdate", "concept",
            (Concept c) => HashConcept(c),
            (ConceptDiff cd) => HashConceptDiff(cd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Concept.Guid, u.Diff!)).ToList() ?? new List<(string, ConceptDiff)>(),
            d.Added ?? new List<Concept>());
    }

    public static string HashPortDiff(PortDiff d)
    {
        var w = new HashWriter();
        w.WriteString("PortDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes?.Count > 0)
        {
            w.WriteString("attributes");
            w.WriteHashList(d.Attributes.Select(HashAttribute).ToList());
        }
        if (d.ShouldSerializeCompatiblePorts() && d.CompatiblePorts?.Count > 0)
        {
            w.WriteString("compatiblePorts");
            w.WriteGuidList(d.CompatiblePorts.Select(cp => cp.Guid).ToList());
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        return w.Digest();
    }

    public static string HashPortsDiff(PortsDiff d)
    {
        return HashCollectionDiffGeneric(
            "PortsDiff", "PortDiffUpdate", "port",
            (Port p) => HashPort(p),
            (PortDiff pd) => HashPortDiff(pd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Port.Guid, u.Diff!)).ToList() ?? new List<(string, PortDiff)>(),
            d.Added ?? new List<Port>());
    }

    public static string HashPropDiff(PropDiff d)
    {
        var w = new HashWriter();
        w.WriteString("PropDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeQuality() && d.Quality != null)
        {
            w.WriteString("quality");
            w.WriteString(d.Quality.Guid);
        }
        WriteDiffOptString(w, "unit", d.Unit, d.ShouldSerializeUnit());
        WriteDiffOptString(w, "value", d.Value, d.ShouldSerializeValue());
        return w.Digest();
    }

    public static string HashModelDiff(ModelDiff d)
    {
        var w = new HashWriter();
        w.WriteString("ModelDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        if (d.ShouldSerializeFile() && d.File != null)
        {
            w.WriteString("file");
            w.WriteString(d.File.Guid);
        }
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializeTags() && d.Tags?.Count > 0)
        {
            w.WriteString("tags");
            w.WriteGuidList(d.Tags.Select(t => t.Guid).ToList());
        }
        return w.Digest();
    }

    public static string HashModelsDiff(ModelsDiff d)
    {
        return HashCollectionDiffGeneric(
            "ModelsDiff", "ModelDiffUpdate", "model",
            (Model m) => HashModel(m),
            (ModelDiff md) => HashModelDiff(md),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Model.Guid, u.Diff!)).ToList() ?? new List<(string, ModelDiff)>(),
            d.Added ?? new List<Model>());
    }

    public static string HashConnectorDiff(ConnectorDiff d)
    {
        var w = new HashWriter();
        w.WriteString("ConnectorDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        if (d.ShouldSerializeDirection() && d.Direction != null)
        {
            w.WriteString("direction");
            w.WriteHash(HashVectorDiff(d.Direction));
        }
        WriteDiffOptBool(w, "mandatory", d.Mandatory, d.ShouldSerializeMandatory());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializePoint() && d.Point != null)
        {
            w.WriteString("point");
            w.WriteHash(HashPointDiff(d.Point));
        }
        if (d.ShouldSerializePort() && d.Port != null)
        {
            w.WriteString("port");
            w.WriteString(d.Port.Guid);
        }
        if (d.ShouldSerializeProps() && d.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(d.Props.Select(HashProp).ToList());
        }
        WriteDiffOptNumber(w, "t", d.T, d.ShouldSerializeT());
        return w.Digest();
    }

    public static string HashConnectorsDiff(ConnectorsDiff d)
    {
        return HashCollectionDiffGeneric(
            "ConnectorsDiff", "ConnectorDiffUpdate", "connector",
            (Connector c) => HashConnector(c),
            (ConnectorDiff cd) => HashConnectorDiff(cd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Connector.Guid, u.Diff!)).ToList() ?? new List<(string, ConnectorDiff)>(),
            d.Added ?? new List<Connector>());
    }

    public static string HashTypeDiff(TypeDiff d)
    {
        var w = new HashWriter();
        w.WriteString("TypeDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeAuthors() && d.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteGuidList(d.Authors.Select(a => a.Guid).ToList());
        }
        else if (d.ShouldSerializeAuthors())
        {
            w.WriteString("authors");
            w.WriteBool(false);
        }
        if (d.ShouldSerializeConcepts() && d.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteGuidList(d.Concepts.Select(c => c.Guid).ToList());
        }
        else if (d.ShouldSerializeConcepts())
        {
            w.WriteString("concepts");
            w.WriteBool(false);
        }
        if (d.ShouldSerializeConnectors() && d.Connectors != null)
        {
            w.WriteString("connectors");
            w.WriteHash(HashConnectorsDiff(d.Connectors));
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffNullableString(w, "folder", d.Folder, d.ShouldSerializeFolder());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffNullableString(w, "image", d.Image, d.ShouldSerializeImage());
        WriteDiffOptBool(w, "isAbstract", d.IsAbstract, d.ShouldSerializeIsAbstract());
        if (d.ShouldSerializeLocation() && d.Location != null)
        {
            w.WriteString("location");
            w.WriteString(d.Location.Guid);
        }
        else if (d.ShouldSerializeLocation())
        {
            w.WriteString("location");
            w.WriteBool(false);
        }
        if (d.ShouldSerializeModels() && d.Models != null)
        {
            w.WriteString("models");
            w.WriteHash(HashModelsDiff(d.Models));
        }
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializeParent() && d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(d.Parent.Guid);
        }
        else if (d.ShouldSerializeParent())
        {
            w.WriteString("parent");
            w.WriteBool(false);
        }
        WriteDiffOptIntNumber(w, "stock", d.Stock, d.ShouldSerializeStock());
        WriteDiffOptString(w, "unit", d.Unit != "" ? d.Unit : null, d.ShouldSerializeUnit());
        WriteDiffOptBool(w, "virtual", d.Virtual, d.ShouldSerializeVirtual());
        return w.Digest();
    }

    public static string HashTypesDiff(TypesDiff d)
    {
        return HashCollectionDiffGeneric(
            "TypesDiff", "TypeDiffUpdate", "type",
            (Type t) => HashType(t),
            (TypeDiff td) => HashTypeDiff(td),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Type.Guid, u.Diff!)).ToList() ?? new List<(string, TypeDiff)>(),
            d.Added ?? new List<Type>());
    }

    public static string HashSideDiff(SideDiff d)
    {
        var w = new HashWriter();
        w.WriteString("SideDiff");
        if (d.ShouldSerializeConnector() && d.Connector != null)
        {
            w.WriteString("connector");
            w.WriteString(d.Connector.Guid);
        }
        if (d.ShouldSerializeDesignPiece() && d.DesignPiece != null)
        {
            w.WriteString("designPiece");
            w.WriteString(d.DesignPiece.Guid);
        }
        if (d.ShouldSerializePiece() && d.Piece != null)
        {
            w.WriteString("piece");
            w.WriteString(d.Piece.Guid);
        }
        return w.Digest();
    }

    public static string HashLayerDiff(LayerDiff d)
    {
        var w = new HashWriter();
        w.WriteString("LayerDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "color", d.Color, d.ShouldSerializeColor());
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptBool(w, "isHidden", d.IsHidden, d.ShouldSerializeIsHidden());
        WriteDiffOptBool(w, "isLocked", d.IsLocked, d.ShouldSerializeIsLocked());
        WriteDiffOptString(w, "path", d.Path, d.ShouldSerializePath());
        return w.Digest();
    }

    public static string HashGroupDiff(GroupDiff d)
    {
        var w = new HashWriter();
        w.WriteString("GroupDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        WriteDiffOptString(w, "color", d.Color, d.ShouldSerializeColor());
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializePieces() && d.Pieces?.Count > 0)
        {
            w.WriteString("pieces");
            w.WriteGuidList(d.Pieces.Select(p => p.Guid).ToList());
        }
        return w.Digest();
    }

    public static string HashStatDiff(StatDiff d)
    {
        var w = new HashWriter();
        w.WriteString("StatDiff");
        WriteDiffOptNumber(w, "max", d.Max, d.ShouldSerializeMax());
        WriteDiffOptBool(w, "maxExcluded", d.MaxExcluded, d.ShouldSerializeMaxExcluded());
        WriteDiffOptNumber(w, "min", d.Min, d.ShouldSerializeMin());
        WriteDiffOptBool(w, "minExcluded", d.MinExcluded, d.ShouldSerializeMinExcluded());
        if (d.ShouldSerializeQuality() && d.Quality != null)
        {
            w.WriteString("quality");
            w.WriteString(d.Quality.Guid);
        }
        WriteDiffOptString(w, "unit", d.Unit, d.ShouldSerializeUnit());
        return w.Digest();
    }

    public static string HashConnectionDiff(ConnectionDiff d)
    {
        var w = new HashWriter();
        w.WriteString("ConnectionDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeConnected() && d.Connected != null)
        {
            w.WriteString("connected");
            w.WriteHash(HashSideDiff(d.Connected));
        }
        if (d.ShouldSerializeConnecting() && d.Connecting != null)
        {
            w.WriteString("connecting");
            w.WriteHash(HashSideDiff(d.Connecting));
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptNumber(w, "gap", d.Gap, d.ShouldSerializeGap());
        WriteDiffOptNumber(w, "rise", d.Rise, d.ShouldSerializeRise());
        WriteDiffOptNumber(w, "rotation", d.Rotation, d.ShouldSerializeRotation());
        WriteDiffOptNumber(w, "shift", d.Shift, d.ShouldSerializeShift());
        WriteDiffOptNumber(w, "tilt", d.Tilt, d.ShouldSerializeTilt());
        WriteDiffOptNumber(w, "turn", d.Turn, d.ShouldSerializeTurn());
        WriteDiffOptNumber(w, "u", d.U, d.ShouldSerializeU());
        WriteDiffOptNumber(w, "v", d.V, d.ShouldSerializeV());
        return w.Digest();
    }

    public static string HashConnectionsDiff(ConnectionsDiff d)
    {
        return HashCollectionDiffGeneric(
            "ConnectionsDiff", "ConnectionDiffUpdate", "connection",
            (Connection c) => HashConnection(c),
            (ConnectionDiff cd) => HashConnectionDiff(cd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Connection.Guid, u.Diff!)).ToList() ?? new List<(string, ConnectionDiff)>(),
            d.Added ?? new List<Connection>());
    }

    public static string HashPieceDiff(PieceDiff d)
    {
        var w = new HashWriter();
        w.WriteString("PieceDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeCenter() && d.Center != null)
        {
            w.WriteString("center");
            w.WriteHash(HashCoordDiff(d.Center));
        }
        WriteDiffOptString(w, "color", d.Color, d.ShouldSerializeColor());
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        if (d.ShouldSerializeDesign() && d.Design != null)
        {
            w.WriteString("design");
            w.WriteString(d.Design.Guid);
        }
        WriteDiffOptBool(w, "isHidden", d.IsHidden, d.ShouldSerializeIsHidden());
        WriteDiffOptBool(w, "isLocked", d.IsLocked, d.ShouldSerializeIsLocked());
        if (d.ShouldSerializeMirrorPlane() && d.MirrorPlane != null)
        {
            w.WriteString("mirrorPlane");
            w.WriteHash(HashPlaneDiff(d.MirrorPlane));
        }
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializePlane() && d.Plane != null)
        {
            w.WriteString("plane");
            w.WriteHash(HashPlaneDiff(d.Plane));
        }
        if (d.ShouldSerializeProps() && d.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(d.Props.Select(HashProp).ToList());
        }
        WriteDiffOptNumber(w, "scale", d.Scale, d.ShouldSerializeScale());
        if (d.ShouldSerializeType() && d.Type != null)
        {
            w.WriteString("type");
            w.WriteString(d.Type.Guid);
        }
        return w.Digest();
    }

    public static string HashPiecesDiff(PiecesDiff d)
    {
        return HashCollectionDiffGeneric(
            "PiecesDiff", "PieceDiffUpdate", "piece",
            (Piece p) => HashPiece(p),
            (PieceDiff pd) => HashPieceDiff(pd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Piece.Guid, u.Diff!)).ToList() ?? new List<(string, PieceDiff)>(),
            d.Added ?? new List<Piece>());
    }

    public static string HashDesignDiff(DesignDiff d)
    {
        var w = new HashWriter();
        w.WriteString("DesignDiff");
        if (d.ShouldSerializeActiveLayer() && d.ActiveLayer != null)
        {
            w.WriteString("activeLayer");
            w.WriteString(d.ActiveLayer.Guid);
        }
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeAuthors() && d.Authors?.Count > 0)
        {
            w.WriteString("authors");
            w.WriteGuidList(d.Authors.Select(a => a.Guid).ToList());
        }
        WriteDiffOptBool(w, "canMirror", d.CanMirror, d.ShouldSerializeCanMirror());
        WriteDiffOptBool(w, "canScale", d.CanScale, d.ShouldSerializeCanScale());
        if (d.ShouldSerializeConcepts() && d.Concepts?.Count > 0)
        {
            w.WriteString("concepts");
            w.WriteGuidList(d.Concepts.Select(c => c.Guid).ToList());
        }
        if (d.ShouldSerializeConnections() && d.Connections != null)
        {
            w.WriteString("connections");
            w.WriteHash(HashConnectionsDiff(d.Connections));
        }
        WriteDiffOptString(w, "description", d.Description, d.ShouldSerializeDescription());
        WriteDiffOptString(w, "folder", d.Folder, d.ShouldSerializeFolder());
        if (d.ShouldSerializeGroups() && d.Groups?.Count > 0)
        {
            w.WriteString("groups");
            w.WriteHashList(d.Groups.Select(HashGroup).ToList());
        }
        WriteDiffOptString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffOptString(w, "image", d.Image, d.ShouldSerializeImage());
        WriteDiffOptBool(w, "isAbstract", d.IsAbstract, d.ShouldSerializeIsAbstract());
        if (d.ShouldSerializeLayers() && d.Layers?.Count > 0)
        {
            w.WriteString("layers");
            w.WriteHashList(d.Layers.Select(HashLayer).ToList());
        }
        if (d.ShouldSerializeLocation() && d.Location != null)
        {
            w.WriteString("location");
            w.WriteString(d.Location.Guid);
        }
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializeParent() && d.Parent != null)
        {
            w.WriteString("parent");
            w.WriteString(d.Parent.Guid);
        }
        if (d.ShouldSerializePieces() && d.Pieces != null)
        {
            w.WriteString("pieces");
            w.WriteHash(HashPiecesDiff(d.Pieces));
        }
        if (d.ShouldSerializeProps() && d.Props?.Count > 0)
        {
            w.WriteString("props");
            w.WriteHashList(d.Props.Select(HashProp).ToList());
        }
        if (d.ShouldSerializeStats() && d.Stats?.Count > 0)
        {
            w.WriteString("stats");
            w.WriteHashList(d.Stats.Select(HashStat).ToList());
        }
        WriteDiffOptString(w, "unit", d.Unit, d.ShouldSerializeUnit());
        return w.Digest();
    }

    public static string HashDesignsDiff(DesignsDiff d)
    {
        return HashCollectionDiffGeneric(
            "DesignsDiff", "DesignDiffUpdate", "design",
            (Design ds) => HashDesign(ds),
            (DesignDiff dd) => HashDesignDiff(dd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Design.Guid, u.Diff!)).ToList() ?? new List<(string, DesignDiff)>(),
            d.Added ?? new List<Design>());
    }

    public static string HashKitDiff(KitDiff d)
    {
        var w = new HashWriter();
        w.WriteString("KitDiff");
        if (d.ShouldSerializeAttributes() && d.Attributes != null)
        {
            w.WriteString("attributes");
            w.WriteHash(HashAttributesDiff(d.Attributes));
        }
        if (d.ShouldSerializeAuthors() && d.Authors != null)
        {
            w.WriteString("authors");
            w.WriteHash(HashAuthorsDiff(d.Authors));
        }
        if (d.ShouldSerializeConcepts() && d.Concepts != null)
        {
            w.WriteString("concepts");
            w.WriteHash(HashConceptsDiff(d.Concepts));
        }
        WriteDiffNullableString(w, "description", d.Description, d.ShouldSerializeDescription());
        if (d.ShouldSerializeDesigns() && d.Designs != null)
        {
            w.WriteString("designs");
            w.WriteHash(HashDesignsDiff(d.Designs));
        }
        if (d.ShouldSerializeFiles() && d.Files != null)
        {
            w.WriteString("files");
            w.WriteHash(HashFilesDiff(d.Files));
        }
        if (d.ShouldSerializeFolders() && d.Folders != null)
        {
            w.WriteString("folders");
            w.WriteHash(HashFoldersDiff(d.Folders));
        }
        WriteDiffNullableString(w, "homepage", d.Homepage, d.ShouldSerializeHomepage());
        WriteDiffNullableString(w, "icon", d.Icon, d.ShouldSerializeIcon());
        WriteDiffNullableString(w, "image", d.Image, d.ShouldSerializeImage());
        WriteDiffNullableString(w, "license", d.License, d.ShouldSerializeLicense());
        WriteDiffOptString(w, "name", d.Name, d.ShouldSerializeName());
        if (d.ShouldSerializePorts() && d.Ports != null)
        {
            w.WriteString("ports");
            w.WriteHash(HashPortsDiff(d.Ports));
        }
        WriteDiffNullableString(w, "preview", d.Preview, d.ShouldSerializePreview());
        WriteDiffNullableString(w, "remote", d.Remote, d.ShouldSerializeRemote());
        if (d.ShouldSerializeTags() && d.Tags != null)
        {
            w.WriteString("tags");
            w.WriteHash(HashTagsDiff(d.Tags));
        }
        if (d.ShouldSerializeTypes() && d.Types != null)
        {
            w.WriteString("types");
            w.WriteHash(HashTypesDiff(d.Types));
        }
        WriteDiffOptString(w, "version", d.Version, d.ShouldSerializeVersion());
        return w.Digest();
    }

    public static string HashKitsDiff(KitsDiff d)
    {
        return HashCollectionDiffGeneric(
            "KitsDiff", "KitDiffUpdate", "kit",
            (Kit k) => HashKit(k),
            (KitDiff kd) => HashKitDiff(kd),
            d.Removed?.Select(r => r.Guid).ToList() ?? new List<string>(),
            d.Updated?.Where(u => u.Diff != null).Select(u => (u.Kit.Guid, u.Diff!)).ToList() ?? new List<(string, KitDiff)>(),
            d.Added ?? new List<Kit>());
    }

    // #endregion ⚗️Hash Diff Entities

    // #endregion 🔗Hash Diffs
}

#endregion 🖥️Hash





#region 🎪Api
// Callers MUST use these methods to communicate with the semio engine.

public class PredictDesignBody
{
    public string? Description { get; set; }
    public Type[]? Types { get; set; }
    public Design? Design { get; set; }
}

public interface IApi
{
    [Get("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<Kit>> GetKit(string encodedKitUri);

    [Put("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> CreateKit(string encodedKitUri, [Body]
        Kit input);

    [Delete("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> DeleteKit(string encodedKitUri);

    [Put("/api/kits/{encodedKitUri}/types/{encodedTypeName}")]
    Task<ApiResponse<bool>> PutType(string encodedKitUri, string encodedTypeName, [Body]
        Type input);

    [Delete("/api/kits/{encodedKitUri}/types/{encodedTypeName}")]
    Task<ApiResponse<bool>> RemoveType(string encodedKitUri, string encodedTypeName);

    [Put("/api/kits/{encodedKitUri}/designs/{encodedDesignName}")]
    Task<ApiResponse<bool>> PutDesign(string encodedKitUri, string encodedDesignName,
    [Body]
        Design input);

    [Delete("/api/kits/{encodedKitUri}/designs/{encodedDesignName}")]
    Task<ApiResponse<bool>> RemoveDesign(string encodedKitUri, string encodedDesignName);

    [Get("/api/assistant/predictDesign")]
    Task<ApiResponse<Design>> PredictDesign([Body]
        PredictDesignBody body);
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
            Message = response.Error?.Content ?? "null",
            Request = response.RequestMessage?.ToString() ?? "null",
            Headers = response.Headers?.ToString() ?? "null"
        });
    }

    private static void HandleErrors<T>(ApiResponse<T> response)
    {
        if (response.StatusCode == HttpStatusCode.BadRequest) throw new ClientException(response.Error?.Content ?? "Bad Request");
        if (!response.IsSuccessStatusCode) throw new ServerException(UnsuccessfullResponseToString(response));
    }

    public static string EncodeNameAndVariant(string name, string variant = "") => Utility.Encode(name) + "," + Utility.Encode(variant);

    public static string EncodeNameAndVariantAndView(string name, string variant = "", string view = "") => EncodeNameAndVariant(name, variant) + "," + Utility.Encode(view);

    public static Kit? GetKit(string uri)
    {
        var response = GetApi().GetKit(Utility.Encode(uri)).Result;
        if (response.IsSuccessStatusCode)
            return response.Content;
        HandleErrors(response);
        return null;
    }

    public static void CreateKit(string uri, Kit input) => HandleErrors(GetApi().CreateKit(Utility.Encode(uri), input).Result);

    public static void DeleteKit(string uri) => HandleErrors(GetApi().DeleteKit(Utility.Encode(uri)).Result);

    public static void PutType(string kitUrl, Type input) => HandleErrors(GetApi().PutType(Utility.Encode(kitUrl), Utility.Encode(input.Name), input).Result);

    public static void RemoveType(string kitUrl, TypeId id) => HandleErrors(GetApi().RemoveType(Utility.Encode(kitUrl), Utility.Encode(id.Guid)).Result);

    public static void PutDesign(string kitUrl, Design input) => HandleErrors(GetApi().PutDesign(Utility.Encode(kitUrl), Utility.Encode(input.Name), input).Result);

    public static void RemoveDesign(string kitUrl, DesignId id) => HandleErrors(GetApi().RemoveDesign(Utility.Encode(kitUrl), Utility.Encode(id.Guid)).Result);

    public static Design? PredictDesign(string description, Type[] types, Design design)
    {
        var response = GetApi().PredictDesign(new PredictDesignBody
        { Description = description, Types = types, Design = design }).Result;
        if (response.IsSuccessStatusCode)
            return response.Content;
        HandleErrors(response);
        return null;
    }
}

public class ClientException : Exception
{
    public ClientException(string message) : base(message) { }
}

public class ServerException : Exception
{
    public ServerException(string message) : base(message) { }
}

#endregion 🎪Api






#region 📦Kit Diff Validation

public sealed class KitDiffValidationNote
{
    [JsonProperty("code", NullValueHandling = NullValueHandling.Ignore)]
    public string? Code { get; set; }
    [JsonProperty("message")]
    public string Message { get; set; } = "";
}

public sealed class KitDiffValidationResult
{
    [JsonProperty("ok")]
    public bool Ok { get; set; }
    public List<KitDiffValidationNote> Errors { get; set; } = new();
    public List<KitDiffValidationNote> Warnings { get; set; } = new();
    [JsonProperty("diff", NullValueHandling = NullValueHandling.Ignore)]
    public KitDiff? Diff { get; set; }
}

#endregion 📦Kit Diff Validation


#region 🌤️Flatten Design
// Callers MUST use FlattenDesign to compute a DesignDiff that assigns world-space planes to all pieces.

public partial class Kit
{

    public static DesignDiff FlattenDesign(Kit kit, string designId)
    {
        var design = FindDesign(kit, designId);
        if (design.Pieces == null || design.Pieces.Count == 0) return new DesignDiff();

        var typesDict = (kit.Types ?? new List<Type>()).ToDictionary(t => t.Guid);

        Type? GetConnectorType(string typeGuid) => typesDict.TryGetValue(typeGuid, out var t) ? t : null;

        Connector? GetConnector(Type? type, string? connectorGuid)
        {
            if (type == null) return null;

            if (string.IsNullOrEmpty(connectorGuid))
            {
                if (type.Connectors != null && type.Connectors.Count > 0) return type.Connectors[0];
                if (!string.IsNullOrEmpty(type.Parent?.Guid))
                {
                    var parentType = GetConnectorType(type.Parent.Guid);
                    return GetConnector(parentType, connectorGuid);
                }
                return null;
            }

            if (type.Connectors != null && type.Connectors.Count > 0)
            {
                var connector = type.Connectors.FirstOrDefault(p => p.Guid == connectorGuid);
                if (connector != null) return connector;
            }

            if (!string.IsNullOrEmpty(type.Parent?.Guid))
            {
                var parentType = GetConnectorType(type.Parent.Guid);
                var connector = GetConnector(parentType, connectorGuid);
                if (connector != null) return connector;
            }

            if (type.Connectors != null && type.Connectors.Count > 0) return type.Connectors[0];

            return null;
        }

        var flatDesignJson = Utility.Serialize(design);
        var flatDesign = Utility.Deserialize<Design>(flatDesignJson);
        if (flatDesign == null) return new DesignDiff();

        if (flatDesign.Pieces == null) flatDesign.Pieces = new List<Piece>();

        var piecePlanes = new Dictionary<string, Plane>();
        var pieceMap = new Dictionary<string, Piece>();
        foreach (var p in flatDesign.Pieces)
        {
            if (!string.IsNullOrEmpty(p.Guid)) pieceMap[p.Guid] = p;
        }

        var filteredConnections = (flatDesign.Connections ?? new List<Connection>()).Where(connection =>
        {
            var sourceId = connection.Connected.Piece.Guid;
            var targetId = connection.Connecting.Piece.Guid;
            return pieceMap.ContainsKey(sourceId) && pieceMap.ContainsKey(targetId);
        }).ToList();

        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var p in flatDesign.Pieces) graph.AddVertex(p.Guid);
        foreach (var c in filteredConnections) graph.AddEdge(new Edge<string>(c.Connected.Piece.Guid, c.Connecting.Piece.Guid));

        var algorithm = new ConnectedComponentsAlgorithm<string, Edge<string>>(graph);
        algorithm.Compute();

        var components = algorithm.Components;
        var componentDict = new Dictionary<int, List<string>>();
        foreach (var kvp in components)
        {
            if (!componentDict.ContainsKey(kvp.Value)) componentDict[kvp.Value] = new List<string>();
            componentDict[kvp.Value].Add(kvp.Key);
        }

        Piece SetAttributes(Piece piece, IEnumerable<(string key, string value)> newAttrs)
        {
            var updatedAttrs = piece.Attributes?.ToList() ?? new List<Attribute>();
            foreach (var newAttr in newAttrs)
            {
                var existingIndex = updatedAttrs.FindIndex(a => a.Key == newAttr.key);
                if (existingIndex >= 0)
                    updatedAttrs[existingIndex].Value = newAttr.value;
                else
                    updatedAttrs.Add(new Attribute { Guid = System.Guid.NewGuid().ToString(), Key = newAttr.key, Value = newAttr.value });
            }
            piece.Attributes = updatedAttrs;
            return piece;
        }

        foreach (var component in componentDict.Values)
        {
            var roots = component.Where(nodeId =>
            {
                var piece = pieceMap.TryGetValue(nodeId, out var p) ? p : null;
                return piece?.Plane != null && piece?.Center != null;
            }).ToList();

            var rootNode = roots.Count > 0 ? roots[0] : (component.Count > 0 ? component[0] : null);
            if (string.IsNullOrEmpty(rootNode)) continue;

            var rootPiece = pieceMap[rootNode];
            if (string.IsNullOrEmpty(rootPiece.Guid)) continue;

            var updatedRootPiece = SetAttributes(rootPiece, new[]
            {
                ("semio.fixedPieceId", rootPiece.Guid),
                ("semio.depth", "0"),
                ("semio.path", rootPiece.Guid)
            });
            pieceMap[rootNode] = updatedRootPiece;

            Plane rootPlane = rootPiece.Plane ?? new Plane { XAxis = new Vector { X = 1, Y = 0, Z = 0 }, YAxis = new Vector { X = 0, Y = 1, Z = 0 }, Origin = new Point { X = 0, Y = 0, Z = 0 } };
            piecePlanes[rootPiece.Guid] = rootPlane;

            var rootPieceIndex = flatDesign.Pieces.FindIndex(p => p.Guid == rootPiece.Guid);
            if (rootPieceIndex != -1)
            {
                flatDesign.Pieces[rootPieceIndex].Plane = rootPlane;
                flatDesign.Pieces[rootPieceIndex].Center ??= new Coord { U = 0, V = 0 };
            }

            var bfs = new UndirectedBreadthFirstSearchAlgorithm<string, Edge<string>>(graph);
            var depths = new Dictionary<string, int>();
            depths[rootNode] = 0;

            bfs.TreeEdge += (sender, e) =>
            {
                var parentId = depths.ContainsKey(e.Source) ? e.Source : e.Target;
                var childId = parentId == e.Source ? e.Target : e.Source;
                depths[childId] = depths[parentId] + 1;

                var parentPiece = pieceMap.TryGetValue(parentId, out var pp) ? pp : null;
                var childPiece = pieceMap.TryGetValue(childId, out var cp) ? cp : null;
                if (parentPiece == null || childPiece == null || string.IsNullOrEmpty(parentPiece.Guid) || string.IsNullOrEmpty(childPiece.Guid)) return;
                if (piecePlanes.ContainsKey(childPiece.Guid)) return;
                if (!piecePlanes.TryGetValue(parentPiece.Guid, out var parentPlane)) return;

                var connection = filteredConnections.FirstOrDefault(c =>
                    (c.Connected.Piece.Guid == parentId && c.Connecting.Piece.Guid == childId) ||
                    (c.Connecting.Piece.Guid == parentId && c.Connected.Piece.Guid == childId));
                if (connection == null) return;

                var parentSide = connection.Connected.Piece.Guid == parentId ? connection.Connected : connection.Connecting;
                var childSide = connection.Connecting.Piece.Guid == childId ? connection.Connecting : connection.Connected;

                var parentType = parentPiece.Type != null ? GetConnectorType(parentPiece.Type.Guid) : null;
                var childType = childPiece.Type != null ? GetConnectorType(childPiece.Type.Guid) : null;

                var parentConnector = GetConnector(parentType, parentSide.Connector?.Guid);
                var childConnector = GetConnector(childType, childSide.Connector?.Guid);

                if (parentConnector == null || childConnector == null) return;
                if (parentConnector.Point == null || parentConnector.Direction == null || childConnector.Point == null || childConnector.Direction == null) return;

                var childPlane = Design.DefaultComputeChildPlane(
                    parentPlane, parentConnector.Point, parentConnector.Direction,
                    childConnector.Point, childConnector.Direction,
                    connection.Gap, connection.Shift, connection.Rise,
                    connection.Rotation, connection.Turn, connection.Tilt);
                piecePlanes[childPiece.Guid] = childPlane;

                var radius = 2.697;
                var verticalVExtra = 1.0;
                var horizontalScale = 3.0633;
                var parentCenter = parentPiece.Center ?? new Coord { U = 0, V = 0 };

                double childU, childV;
                if (parentCenter.U == 0 && parentCenter.V == 0)
                {
                    var angle = 2 * Math.PI * parentConnector.T;
                    childU = radius * Math.Sin(angle);
                    childV = radius * Math.Cos(angle);
                }
                else
                {
                    var isVerticalConnection = Math.Abs(parentConnector.Direction?.Z ?? 0) > 0.5;
                    if (isVerticalConnection)
                    {
                        childU = parentCenter.U + (connection.U ?? 0);
                        childV = parentCenter.V + (connection.V ?? 0) + verticalVExtra;
                    }
                    else
                    {
                        childU = parentCenter.U + (connection.U ?? 0) * horizontalScale;
                        childV = parentCenter.V + (connection.V ?? 0) * horizontalScale;
                    }
                }

                var childCenter = new Coord { U = Math.Round(childU, 6), V = Math.Round(childV, 6) };
                var fixedPieceId = parentPiece.Attributes?.FirstOrDefault(q => q.Key == "semio.fixedPieceId")?.Value ?? "";
                var parentPath = parentPiece.Attributes?.FirstOrDefault(q => q.Key == "semio.path")?.Value ?? "";

                childPiece.Plane = childPlane;
                childPiece.Center = childCenter;

                var flatChildPiece = SetAttributes(childPiece, new[]
                {
                    ("semio.fixedPieceId", fixedPieceId),
                    ("semio.parentPieceId", parentPiece.Guid),
                    ("semio.depth", depths[childId].ToString()),
                    ("semio.path", parentPath + "," + childPiece.Guid)
                });
                pieceMap[childId] = flatChildPiece;
            };

            bfs.Compute(rootNode);
        }

        flatDesign.Pieces = flatDesign.Pieces.Select(p => pieceMap.TryGetValue(p.Guid ?? "", out var mapped) ? mapped : p).ToList();
        flatDesign.Connections = new List<Connection>();

        var updatedPieces = flatDesign.Pieces.Select(flatPiece =>
        {
            var originalPiece = design.Pieces?.FirstOrDefault(p => p.Guid == flatPiece.Guid);
            if (originalPiece == null) return null;

            var pieceDiff = new PieceDiff();
            bool hasChanges = false;

            if (flatPiece.Plane != null && Utility.Serialize(flatPiece.Plane) != Utility.Serialize(originalPiece.Plane))
            {
                pieceDiff.Plane = flatPiece.Plane;
                hasChanges = true;
            }

            if (flatPiece.Center != null && Utility.Serialize(flatPiece.Center) != Utility.Serialize(originalPiece.Center))
            {
                pieceDiff.Center = flatPiece.Center;
                hasChanges = true;
            }

            if (Utility.Serialize(flatPiece.Attributes) != Utility.Serialize(originalPiece.Attributes))
            {
                pieceDiff.Attributes = flatPiece.Attributes.ToList();
                hasChanges = true;
            }

            if (!hasChanges) return null;

            return new PieceDiffUpdate
            {
                Piece = new PieceId { Guid = flatPiece.Guid },
                Diff = pieceDiff
            };
        }).Where(u => u != null).Cast<PieceDiffUpdate>().ToList();

        var removedConnections = (design.Connections ?? new List<Connection>())
            .Select(c => new ConnectionId { Guid = c.Guid })
            .ToList();

        var designDiff = new DesignDiff();
        if (updatedPieces.Count > 0) designDiff.Pieces = new PiecesDiff { Updated = updatedPieces };
        if (removedConnections.Count > 0) designDiff.Connections = new ConnectionsDiff { Removed = removedConnections };

        return designDiff;
    }

    public static DesignDiff ReplaceClusterWithDesign(Design originalDesign, List<string> clusterPieceIds, Design clusteredDesign, List<Connection> externalConnections)
    {
        var addedPieces = clusteredDesign.Pieces ?? new List<Piece>();
        var addedConnections = clusteredDesign.Connections ?? new List<Connection>();

        var addedClusteredConnections = externalConnections.Select(c =>
        {
            var newConnection = Utility.Deserialize<Connection>(Utility.Serialize(c));
            if (newConnection != null) newConnection.Guid = System.Guid.NewGuid().ToString();
            return newConnection;
        }).Where(c => c != null).Cast<Connection>().ToList();

        addedConnections.AddRange(addedClusteredConnections);

        return new DesignDiff
        {
            Pieces = new PiecesDiff
            {
                Removed = clusterPieceIds.Select(id => new PieceId { Guid = id }).ToList(),
                Added = addedPieces
            },
            Connections = new ConnectionsDiff
            {
                Removed = (originalDesign.Connections ?? new List<Connection>())
                    .Where(c => clusterPieceIds.Contains(c.Connected.Piece.Guid) || clusterPieceIds.Contains(c.Connecting.Piece.Guid))
                    .Select(c => new ConnectionId
                    {
                        Connected = new Side { Piece = new PieceId { Guid = c.Connected.Piece.Guid } },
                        Connecting = new Side { Piece = new PieceId { Guid = c.Connecting.Piece.Guid } }
                    }).ToList(),
                Added = addedConnections
            }
        };
    }

    #endregion 🌤️Flatten Design
}

#endregion 🌤️Flatten Design


#region 🔩Kit Model Export
// Callers MUST use ExportDesignModel to produce a valid 3D file from a design.

public partial class Kit
{

    /// <summary>📺Supported export formats keyed by file extension.</summary>
    public static Dictionary<string, string> ExportModelFormats => new()
    {
        { ".glb", "GL Transmission Format Binary" },
        { ".gltf", "GL Transmission Format" },
        { ".obj", "Wavefront OBJ" },
        { ".stl", "Stereolithography" },
    };

    /// <summary>
    /// Exports the 3D model of a design to the specified format.
    /// Uses block definitions for types and instances for pieces.
    /// Connection hierarchy is translated into a scene graph; planes become relative transformation matrices.
    /// </summary>
    public static byte[] ExportDesignModel(Kit kit, string designId, string format = ".glb", string[] tags = null, Dictionary<string, object> options = null)
    {
        if (tags == null) tags = Array.Empty<string>();
        if (options == null) options = new Dictionary<string, object>();
        if (!ExportModelFormats.ContainsKey(format))
            throw new ArgumentException($"Unsupported export format: {format}. Supported: {string.Join(", ", ExportModelFormats.Keys)}", nameof(format));

        var design = FindDesign(kit, designId);
        var pieces = design.Pieces ?? new List<Piece>();
        var connections = design.Connections ?? new List<Connection>();
        var types = kit.Types ?? new List<Type>();

        if (pieces.Count == 0)
            return ExportSceneBuilderToFormat(new SceneBuilder("empty"), format);

        var typesDict = new Dictionary<string, Type>();
        foreach (var t in types) typesDict[t.Guid] = t;
        var piecesDict = new Dictionary<string, Piece>();
        foreach (var p in pieces) piecesDict[p.Guid] = p;

        var adjacency = new Dictionary<string, List<(Connection connection, string neighborGuid)>>();
        foreach (var p in pieces) adjacency[p.Guid] = new List<(Connection, string)>();
        foreach (var conn in connections)
        {
            var connectedGuid = conn.Connected.Piece.Guid;
            var connectingGuid = conn.Connecting.Piece.Guid;
            if (adjacency.ContainsKey(connectedGuid))
                adjacency[connectedGuid].Add((conn, connectingGuid));
            if (adjacency.ContainsKey(connectingGuid))
                adjacency[connectingGuid].Add((conn, connectedGuid));
        }

        var piecePlanes = new Dictionary<string, Plane>();
        var parentOf = new Dictionary<string, string>();
        var childrenOf = new Dictionary<string, List<string>>();
        foreach (var p in pieces) childrenOf[p.Guid] = new List<string>();

        var visited = new HashSet<string>();
        var roots = new List<string>();
        var queue = new Queue<string>();

        Type GetType(string typeGuid) => typesDict.TryGetValue(typeGuid, out var t) ? t : null;
        Connector GetConnector(Type type, string connectorGuid)
        {
            if (type == null) return null;
            if (string.IsNullOrEmpty(connectorGuid))
                return type.Connectors?.Count > 0 ? type.Connectors[0] : null;
            return type.Connectors?.FirstOrDefault(c => c.Guid == connectorGuid);
        }

        foreach (var p in pieces)
        {
            if (p.Plane != null && p.Center != null)
            {
                piecePlanes[p.Guid] = p.Plane;
                visited.Add(p.Guid);
                queue.Enqueue(p.Guid);
                roots.Add(p.Guid);
            }
        }

        if (queue.Count == 0 && pieces.Count > 0)
        {
            var identityPlane = new Plane
            {
                Origin = new Point { X = 0, Y = 0, Z = 0 },
                XAxis = new Vector { X = 1, Y = 0, Z = 0 },
                YAxis = new Vector { X = 0, Y = 1, Z = 0 }
            };
            piecePlanes[pieces[0].Guid] = identityPlane;
            visited.Add(pieces[0].Guid);
            queue.Enqueue(pieces[0].Guid);
            roots.Add(pieces[0].Guid);
        }

        while (queue.Count > 0)
        {
            var currentGuid = queue.Dequeue();
            var currentPlane = piecePlanes[currentGuid];
            if (!adjacency.TryGetValue(currentGuid, out var edges)) continue;
            foreach (var edge in edges)
            {
                if (visited.Contains(edge.neighborGuid)) continue;
                var conn = edge.connection;
                var isParent = conn.Connected.Piece.Guid == currentGuid;
                if (!isParent) continue;

                var childGuid = edge.neighborGuid;
                var parentPiece = piecesDict[currentGuid];
                var childPiece = piecesDict[childGuid];
                var parentType = parentPiece.Type != null ? GetType(parentPiece.Type.Guid) : null;
                var childType = childPiece.Type != null ? GetType(childPiece.Type.Guid) : null;
                var parentConnector = GetConnector(parentType, conn.Connected.Connector?.Guid);
                var childConnector = GetConnector(childType, conn.Connecting.Connector?.Guid);

                if (parentConnector != null && childConnector != null &&
                    parentConnector.Point != null && parentConnector.Direction != null &&
                    childConnector.Point != null && childConnector.Direction != null)
                {
                    piecePlanes[childGuid] = Design.DefaultComputeChildPlane(
                        currentPlane, parentConnector.Point, parentConnector.Direction,
                        childConnector.Point, childConnector.Direction,
                        conn.Gap, conn.Shift, conn.Rise,
                        conn.Rotation, conn.Turn, conn.Tilt);
                }
                else
                {
                    piecePlanes[childGuid] = currentPlane;
                }

                parentOf[childGuid] = currentGuid;
                childrenOf[currentGuid].Add(childGuid);
                visited.Add(childGuid);
                queue.Enqueue(childGuid);
            }
        }

        foreach (var p in pieces)
        {
            if (!visited.Contains(p.Guid))
            {
                piecePlanes[p.Guid] = new Plane
                {
                    Origin = new Point { X = 0, Y = 0, Z = 0 },
                    XAxis = new Vector { X = 1, Y = 0, Z = 0 },
                    YAxis = new Vector { X = 0, Y = 1, Z = 0 }
                };
                roots.Add(p.Guid);
            }
        }

        var typeMeshBuilders = new Dictionary<string, IMeshBuilder<MaterialBuilder>>();
        foreach (var piece in pieces)
        {
            var typeGuid = piece.Type?.Guid;
            if (string.IsNullOrEmpty(typeGuid) || typeMeshBuilders.ContainsKey(typeGuid)) continue;
            if (!typesDict.TryGetValue(typeGuid, out var type)) continue;

            var model = ExportFindMatchingModel(kit, type, tags);
            if (model != null)
            {
                var file = kit.Files?.FirstOrDefault(f => f.Guid == model.File.Guid);
                if (file?.Blob != null)
                {
                    var fileBytes = ExportBlobToBytes(file.Blob);
                    var ext = System.IO.Path.GetExtension(file.Name).ToLowerInvariant();
                    if (ext == ".glb")
                    {
                        var mb = ExportGlbToMeshBuilder(fileBytes, file.Name);
                        if (mb != null) { typeMeshBuilders[typeGuid] = mb; continue; }
                    }
                }
            }
            typeMeshBuilders[typeGuid] = ExportCreateBoxMeshBuilder(type.Name);
        }

        var sceneBuilder = new SceneBuilder(design.Name ?? "design");

        void BuildNodeHierarchy(string pieceGuid, NodeBuilder parent)
        {
            var piece = piecesDict[pieceGuid];
            var worldPlane = piecePlanes[pieceGuid];

            NodeBuilder node;
            if (parent != null)
            {
                node = parent.CreateNode(piece.Name ?? piece.Guid);
                var parentWorld = ExportPlaneToMatrix4x4(piecePlanes[parentOf[pieceGuid]]);
                var childWorld = ExportPlaneToMatrix4x4(worldPlane);
                System.Numerics.Matrix4x4.Invert(parentWorld, out var parentInv);
                node.LocalMatrix = childWorld * parentInv;
            }
            else
            {
                node = new NodeBuilder(piece.Name ?? piece.Guid);
                node.LocalMatrix = ExportPlaneToMatrix4x4(worldPlane);
            }

            var meshTypeGuid = piece.Type?.Guid;
            if (!string.IsNullOrEmpty(meshTypeGuid) && typeMeshBuilders.TryGetValue(meshTypeGuid, out var meshBuilder))
                sceneBuilder.AddRigidMesh(meshBuilder, node);
            else
                sceneBuilder.AddNode(node);

            if (childrenOf.TryGetValue(pieceGuid, out var children))
            {
                foreach (var childGuid in children)
                    BuildNodeHierarchy(childGuid, node);
            }
        }

        foreach (var rootGuid in roots)
            BuildNodeHierarchy(rootGuid, null);

        return ExportSceneBuilderToFormat(sceneBuilder, format);
    }

    #region 🔧Kit Model Export Helpers

    private static System.Numerics.Matrix4x4 ExportPlaneToMatrix4x4(Plane p)
    {
        var origin = new System.Numerics.Vector3((float)p.Origin.X, (float)p.Origin.Y, (float)p.Origin.Z);
        var x = System.Numerics.Vector3.Normalize(new System.Numerics.Vector3((float)p.XAxis.X, (float)p.XAxis.Y, (float)p.XAxis.Z));
        var yRaw = new System.Numerics.Vector3((float)p.YAxis.X, (float)p.YAxis.Y, (float)p.YAxis.Z);
        var z = System.Numerics.Vector3.Normalize(System.Numerics.Vector3.Cross(x, yRaw));
        var y = System.Numerics.Vector3.Normalize(System.Numerics.Vector3.Cross(z, x));
        return ExportApplySemioToGltfBasis(new System.Numerics.Matrix4x4(
            x.X, x.Y, x.Z, 0,
            y.X, y.Y, y.Z, 0,
            z.X, z.Y, z.Z, 0,
            origin.X, origin.Y, origin.Z, 1));
    }

    private static System.Numerics.Matrix4x4 ExportApplySemioToGltfBasis(System.Numerics.Matrix4x4 matrix)
    {
        var basis = new System.Numerics.Matrix4x4(
            1, 0, 0, 0,
            0, 0, -1, 0,
            0, 1, 0, 0,
            0, 0, 0, 1);
        var inverse = new System.Numerics.Matrix4x4(
            1, 0, 0, 0,
            0, 0, 1, 0,
            0, -1, 0, 0,
            0, 0, 0, 1);
        return System.Numerics.Matrix4x4.Multiply(System.Numerics.Matrix4x4.Multiply(inverse, matrix), basis);
    }

    private static byte[] ExportBlobToBytes(string blob)
    {
        var base64 = blob;
        if (blob.StartsWith("data:"))
        {
            var commaIdx = blob.IndexOf(',');
            if (commaIdx >= 0) base64 = blob.Substring(commaIdx + 1);
        }
        return Convert.FromBase64String(base64);
    }

    public static Model ExportFindMatchingModel(Kit kit, Type type, string[] tags)
    {
        if (type.Models == null || type.Models.Count == 0) return null;
        if (tags == null || tags.Length == 0)
        {
            var defaultModel = type.Models.FirstOrDefault(m => m.Tags == null || m.Tags.Count == 0);
            return defaultModel ?? type.Models[0];
        }
        var kitTags = kit.Tags ?? new List<Tag>();
        var selectedTagGuids = new HashSet<string>();
        foreach (var tagValue in tags)
        {
            var byGuid = kitTags.FirstOrDefault(t => t.Guid == tagValue);
            if (byGuid != null)
            {
                selectedTagGuids.Add(byGuid.Guid);
                continue;
            }
            foreach (var tag in kitTags.Where(t => t.Name == tagValue))
                selectedTagGuids.Add(tag.Guid);
        }
        Model bestModel = null;
        double bestScore = -1;
        foreach (var model in type.Models)
        {
            var modelTagGuids = new HashSet<string>((model.Tags ?? new List<TagId>()).Select(t => t.Guid));
            if (!selectedTagGuids.All(modelTagGuids.Contains)) continue;
            var intersection = modelTagGuids.Intersect(selectedTagGuids).Count();
            var union = modelTagGuids.Union(selectedTagGuids).Count();
            var score = union > 0 ? (double)intersection / union : 0;
            if (score > bestScore)
            {
                bestScore = score;
                bestModel = model;
            }
        }
        if (bestModel != null) return bestModel;
        return type.Models[0];
    }

    private static IMeshBuilder<MaterialBuilder> ExportGlbToMeshBuilder(byte[] glbBytes, string name)
    {
        var srcModel = GltfModel.ReadGLB(new MemoryStream(glbBytes));
        var meshBuilder = new MeshBuilder<VertexPositionNormal>(name);

        foreach (var srcMesh in srcModel.LogicalMeshes)
        {
            foreach (var srcPrim in srcMesh.Primitives)
            {
                var posAccessor = srcPrim.GetVertexAccessor("POSITION");
                if (posAccessor == null) continue;
                var positions = posAccessor.AsVector3Array();

                var normAccessor = srcPrim.GetVertexAccessor("NORMAL");
                var normals = normAccessor?.AsVector3Array();

                var matBuilder = new MaterialBuilder(srcPrim.Material?.Name ?? "default")
                    .WithMetallicRoughnessShader();

                if (srcPrim.Material != null)
                {
                    var baseColor = srcPrim.Material.FindChannel("BaseColor");
                    if (baseColor.HasValue)
                        matBuilder.UseChannel(KnownChannel.BaseColor).Parameter = baseColor.Value.Color;
                }

                var prim = meshBuilder.UsePrimitive(matBuilder);

                var idxAccessor = srcPrim.IndexAccessor;
                if (idxAccessor != null)
                {
                    var indices = idxAccessor.AsIndicesArray();
                    for (int i = 0; i + 2 < indices.Count; i += 3)
                    {
                        var i0 = (int)indices[i];
                        var i1 = (int)indices[i + 1];
                        var i2 = (int)indices[i + 2];
                        prim.AddTriangle(
                            new VertexPositionNormal(positions[i0], normals != null ? normals[i0] : System.Numerics.Vector3.UnitZ),
                            new VertexPositionNormal(positions[i1], normals != null ? normals[i1] : System.Numerics.Vector3.UnitZ),
                            new VertexPositionNormal(positions[i2], normals != null ? normals[i2] : System.Numerics.Vector3.UnitZ));
                    }
                }
                else
                {
                    for (int i = 0; i + 2 < positions.Count; i += 3)
                    {
                        prim.AddTriangle(
                            new VertexPositionNormal(positions[i], normals != null ? normals[i] : System.Numerics.Vector3.UnitZ),
                            new VertexPositionNormal(positions[i + 1], normals != null ? normals[i + 1] : System.Numerics.Vector3.UnitZ),
                            new VertexPositionNormal(positions[i + 2], normals != null ? normals[i + 2] : System.Numerics.Vector3.UnitZ));
                    }
                }
            }
        }

        return meshBuilder;
    }

    private static IMeshBuilder<MaterialBuilder> ExportCreateBoxMeshBuilder(string name)
    {
        var meshBuilder = new MeshBuilder<VertexPositionNormal>(name);
        var material = new MaterialBuilder("default").WithUnlitShader();
        var prim = meshBuilder.UsePrimitive(material);

        const float h = 0.5f;
        var v = new System.Numerics.Vector3[]
        {
            new(-h, -h, -h), new(h, -h, -h), new(h, h, -h), new(-h, h, -h),
            new(-h, -h, h), new(h, -h, h), new(h, h, h), new(-h, h, h)
        };
        var uz = System.Numerics.Vector3.UnitZ;
        var ux = System.Numerics.Vector3.UnitX;
        var uy = System.Numerics.Vector3.UnitY;
        VertexPositionNormal V(System.Numerics.Vector3 pos, System.Numerics.Vector3 nrm) => new(pos, nrm);
        prim.AddTriangle(V(v[4], uz), V(v[5], uz), V(v[6], uz));
        prim.AddTriangle(V(v[4], uz), V(v[6], uz), V(v[7], uz));
        prim.AddTriangle(V(v[1], -uz), V(v[0], -uz), V(v[3], -uz));
        prim.AddTriangle(V(v[1], -uz), V(v[3], -uz), V(v[2], -uz));
        prim.AddTriangle(V(v[3], uy), V(v[7], uy), V(v[6], uy));
        prim.AddTriangle(V(v[3], uy), V(v[6], uy), V(v[2], uy));
        prim.AddTriangle(V(v[0], -uy), V(v[1], -uy), V(v[5], -uy));
        prim.AddTriangle(V(v[0], -uy), V(v[5], -uy), V(v[4], -uy));
        prim.AddTriangle(V(v[1], ux), V(v[2], ux), V(v[6], ux));
        prim.AddTriangle(V(v[1], ux), V(v[6], ux), V(v[5], ux));
        prim.AddTriangle(V(v[0], -ux), V(v[4], -ux), V(v[7], -ux));
        prim.AddTriangle(V(v[0], -ux), V(v[7], -ux), V(v[3], -ux));

        return meshBuilder;
    }

    private static byte[] ExportSceneBuilderToFormat(SceneBuilder sceneBuilder, string format)
    {
        var modelRoot = sceneBuilder.ToGltf2();
        modelRoot.Asset.Generator = "semio";

        switch (format)
        {
            case ".glb":
                {
                    using var ms = new MemoryStream();
                    modelRoot.WriteGLB(ms);
                    return ms.ToArray();
                }
            case ".gltf":
                {
                    var tmpDir = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "semio_gltf_" + System.Guid.NewGuid());
                    System.IO.Directory.CreateDirectory(tmpDir);
                    try
                    {
                        var gltfPath = System.IO.Path.Combine(tmpDir, "model.gltf");
                        modelRoot.SaveGLTF(gltfPath);
                        var gltfJson = Newtonsoft.Json.Linq.JObject.Parse(System.IO.File.ReadAllText(gltfPath));
                        foreach (var buffer in gltfJson["buffers"] as Newtonsoft.Json.Linq.JArray ?? new Newtonsoft.Json.Linq.JArray())
                        {
                            var uri = buffer["uri"]?.Value<string>();
                            if (string.IsNullOrWhiteSpace(uri) || uri.StartsWith("data:")) continue;
                            var path = System.IO.Path.Combine(tmpDir, uri);
                            if (!System.IO.File.Exists(path)) continue;
                            var bytes = System.IO.File.ReadAllBytes(path);
                            buffer["uri"] = "data:application/octet-stream;base64," + Convert.ToBase64String(bytes);
                        }
                        foreach (var image in gltfJson["images"] as Newtonsoft.Json.Linq.JArray ?? new Newtonsoft.Json.Linq.JArray())
                        {
                            var uri = image["uri"]?.Value<string>();
                            if (string.IsNullOrWhiteSpace(uri) || uri.StartsWith("data:")) continue;
                            var path = System.IO.Path.Combine(tmpDir, uri);
                            if (!System.IO.File.Exists(path)) continue;
                            var mime = image["mimeType"]?.Value<string>() ?? "application/octet-stream";
                            var bytes = System.IO.File.ReadAllBytes(path);
                            image["uri"] = $"data:{mime};base64,{Convert.ToBase64String(bytes)}";
                        }
                        return Encoding.UTF8.GetBytes(gltfJson.ToString(Formatting.None));
                    }
                    finally
                    {
                        System.IO.Directory.Delete(tmpDir, true);
                    }
                }
            case ".obj":
                return ExportModelRootToObj(modelRoot);
            case ".stl":
                return ExportModelRootToStl(modelRoot);
            default:
                throw new ArgumentException($"Unsupported export format: {format}");
        }
    }

    #region ❄️Geometric Insights
    // Key performance indicators for GLB/GLTF model geometry. Model MUST be glb/gltf.

    /// <summary>🔷Geometric KPIs for a GLB/GLTF model in semio coordinate system (semio x=glb x, semio y=-glb x, semio z=glb y).</summary>
    public class GeometricInsights
    {
        public Point? BoundingBoxMin { get; set; }
        public Point? BoundingBoxMax { get; set; }
        public double DimensionX { get; set; }
        public double DimensionY { get; set; }
        public double DimensionZ { get; set; }
        public double CharacteristicLength { get; set; }
        public double FootprintArea { get; set; }
        public double TotalSurfaceArea { get; set; }
        public double EnclosedVolume { get; set; }
        public double SurfaceToVolumeRatio { get; set; }
        public double AspectRatioXy { get; set; }
        public double AspectRatioXz { get; set; }
        public double AspectRatioYz { get; set; }
        public bool IsWatertight { get; set; }
        public Point? Centroid { get; set; }
        public double Slenderness { get; set; }
        public int VertexCount { get; set; }
        public int FaceCount { get; set; }
        public int EulerCharacteristic { get; set; }
    }

    public static GeometricInsights GetGeometricInsightsForModel(object model)
    {
        GltfModel root;
        if (model is string path)
        {
            if (!System.IO.File.Exists(path))
                throw new FileNotFoundException("Model file not found", path);
            var ext = System.IO.Path.GetExtension(path).ToLowerInvariant();
            if (ext != ".glb" && ext != ".gltf")
                throw new ArgumentException("Model MUST be .glb or .gltf", nameof(model));
            root = GltfModel.Load(path);
        }
        else if (model is byte[] bytes)
        {
            using var ms = new MemoryStream(bytes);
            if (bytes.Length >= 4 && Encoding.ASCII.GetString(bytes, 0, 4) == "glTF")
                root = GltfModel.ReadGLB(ms);
            else
            {
                var tmp = System.IO.Path.Combine(System.IO.Path.GetTempPath(), "semio_gltf_" + System.Guid.NewGuid().ToString("N") + ".gltf");
                try
                {
                    System.IO.File.WriteAllBytes(tmp, bytes);
                    root = GltfModel.Load(tmp);
                }
                finally { try { System.IO.File.Delete(tmp); } catch { } }
            }
        }
        else
            throw new ArgumentException("Model must be string path or byte[]", nameof(model));

        var out_ = new GeometricInsights();
        double sxMin = double.MaxValue, syMin = double.MaxValue, szMin = double.MaxValue;
        double sxMax = double.MinValue, syMax = double.MinValue, szMax = double.MinValue;
        double sumSx = 0, sumSy = 0, sumSz = 0;
        double totalArea = 0, totalVolume = 0;
        int vertexCount = 0, faceCount = 0;

        foreach (var mesh in root.LogicalMeshes)
        {
            foreach (var prim in mesh.Primitives)
            {
                var posAcc = prim.GetVertexAccessor("POSITION");
                if (posAcc == null) continue;
                var positions = posAcc.AsVector3Array();
                var idxAcc = prim.IndexAccessor;
                int n = positions.Count;
                for (int i = 0; i < n; i++)
                {
                    var p = positions[i];
                    double xg = p.X, yg = p.Y, _zg = p.Z;
                    double sx = xg, sy = -xg, sz = yg;
                    if (sx < sxMin) sxMin = sx; if (sx > sxMax) sxMax = sx;
                    if (sy < syMin) syMin = sy; if (sy > syMax) syMax = sy;
                    if (sz < szMin) szMin = sz; if (sz > szMax) szMax = sz;
                    sumSx += sx; sumSy += sy; sumSz += sz;
                }
                vertexCount += n;
                if (idxAcc != null)
                {
                    var indices = idxAcc.AsIndicesArray();
                    for (int i = 0; i + 2 < indices.Count; i += 3)
                    {
                        int i0 = (int)indices[i], i1 = (int)indices[i + 1], i2 = (int)indices[i + 2];
                        var a = positions[i0]; var b = positions[i1]; var c = positions[i2];
                        var ab = b - a; var ac = c - a;
                        var cross = Vector3.Cross(ab, ac);
                        totalArea += 0.5 * cross.Length();
                        totalVolume += (1.0 / 6.0) * Vector3.Dot(a, Vector3.Cross(b, c));
                        faceCount++;
                    }
                }
                else
                {
                    for (int i = 0; i + 2 < n; i += 3)
                    {
                        var a = positions[i]; var b = positions[i + 1]; var c = positions[i + 2];
                        var ab = b - a; var ac = c - a;
                        totalArea += 0.5 * Vector3.Cross(ab, ac).Length();
                        totalVolume += (1.0 / 6.0) * Vector3.Dot(a, Vector3.Cross(b, c));
                        faceCount++;
                    }
                }
            }
        }

        if (vertexCount == 0) return out_;

        out_.BoundingBoxMin = new Point { X = (float)sxMin, Y = (float)syMin, Z = (float)szMin };
        out_.BoundingBoxMax = new Point { X = (float)sxMax, Y = (float)syMax, Z = (float)szMax };
        out_.DimensionX = sxMax - sxMin;
        out_.DimensionY = syMax - syMin;
        out_.DimensionZ = szMax - szMin;
        out_.CharacteristicLength = Math.Pow(out_.DimensionX * out_.DimensionY * out_.DimensionZ, 1.0 / 3.0);
        out_.FootprintArea = out_.DimensionX * out_.DimensionZ;
        out_.TotalSurfaceArea = totalArea;
        out_.VertexCount = vertexCount;
        out_.FaceCount = faceCount;
        double nV = vertexCount;
        out_.Centroid = new Point { X = (float)(sumSx / nV), Y = (float)(sumSy / nV), Z = (float)(sumSz / nV) };
        totalVolume = Math.Abs(totalVolume);
        out_.EnclosedVolume = totalVolume;
        if (totalVolume > 1e-20 && totalArea > 0)
            out_.SurfaceToVolumeRatio = totalArea / totalVolume;
        if (out_.DimensionY > 1e-10 && out_.DimensionX > 1e-10) out_.AspectRatioXy = out_.DimensionX / out_.DimensionY;
        if (out_.DimensionZ > 1e-10 && out_.DimensionX > 1e-10) out_.AspectRatioXz = out_.DimensionX / out_.DimensionZ;
        if (out_.DimensionZ > 1e-10 && out_.DimensionY > 1e-10) out_.AspectRatioYz = out_.DimensionY / out_.DimensionZ;
        double maxExt = Math.Max(out_.DimensionX, Math.Max(out_.DimensionY, out_.DimensionZ));
        if (maxExt > 1e-10 && totalArea > 0)
            out_.Slenderness = maxExt / Math.Pow(totalArea * maxExt, 1.0 / 3.0);
        out_.EulerCharacteristic = vertexCount - (3 * faceCount) / 2 + faceCount;
        return out_;
    }

    #endregion ❄️Geometric Insights

    private static byte[] ExportModelRootToObj(GltfModel model)
    {
        var sb = new StringBuilder();
        sb.AppendLine("# Generated by semio");
        int vertexOffset = 1;
        int normalOffset = 1;

        foreach (var node in model.DefaultScene.VisualChildren)
            ExportNodeToObj(node, System.Numerics.Matrix4x4.Identity, sb, ref vertexOffset, ref normalOffset);

        return Encoding.UTF8.GetBytes(sb.ToString());
    }

    private static void ExportNodeToObj(GltfNode node, System.Numerics.Matrix4x4 parentWorld,
        StringBuilder sb, ref int vertexOffset, ref int normalOffset)
    {
        var worldMatrix = node.LocalMatrix * parentWorld;

        if (node.Mesh != null)
        {
            sb.AppendLine($"g {node.Name ?? "mesh"}");
            foreach (var srcPrim in node.Mesh.Primitives)
            {
                var posAccessor = srcPrim.GetVertexAccessor("POSITION");
                if (posAccessor == null) continue;
                var positions = posAccessor.AsVector3Array();
                var normAccessor = srcPrim.GetVertexAccessor("NORMAL");
                var normals = normAccessor?.AsVector3Array();

                int startVert = vertexOffset;
                int startNorm = normalOffset;

                foreach (var pos in positions)
                {
                    var wp = System.Numerics.Vector3.Transform(pos, worldMatrix);
                    sb.AppendLine(string.Format(System.Globalization.CultureInfo.InvariantCulture,
                        "v {0:G9} {1:G9} {2:G9}", wp.X, wp.Y, wp.Z));
                    vertexOffset++;
                }

                if (normals != null)
                {
                    foreach (var norm in normals)
                    {
                        var wn = System.Numerics.Vector3.Normalize(
                            System.Numerics.Vector3.TransformNormal(norm, worldMatrix));
                        sb.AppendLine(string.Format(System.Globalization.CultureInfo.InvariantCulture,
                            "vn {0:G9} {1:G9} {2:G9}", wn.X, wn.Y, wn.Z));
                        normalOffset++;
                    }
                }

                var idxAccessor = srcPrim.IndexAccessor;
                if (idxAccessor != null)
                {
                    var indices = idxAccessor.AsIndicesArray();
                    for (int i = 0; i + 2 < indices.Count; i += 3)
                    {
                        var i0 = (int)indices[i] + startVert;
                        var i1 = (int)indices[i + 1] + startVert;
                        var i2 = (int)indices[i + 2] + startVert;
                        if (normals != null)
                        {
                            var n0 = (int)indices[i] + startNorm;
                            var n1 = (int)indices[i + 1] + startNorm;
                            var n2 = (int)indices[i + 2] + startNorm;
                            sb.AppendLine($"f {i0}//{n0} {i1}//{n1} {i2}//{n2}");
                        }
                        else
                        {
                            sb.AppendLine($"f {i0} {i1} {i2}");
                        }
                    }
                }
            }
        }

        foreach (var child in node.VisualChildren)
            ExportNodeToObj(child, worldMatrix, sb, ref vertexOffset, ref normalOffset);
    }

    private static byte[] ExportModelRootToStl(GltfModel model)
    {
        var triangles = new List<(System.Numerics.Vector3 normal, System.Numerics.Vector3 v0, System.Numerics.Vector3 v1, System.Numerics.Vector3 v2)>();

        foreach (var node in model.DefaultScene.VisualChildren)
            ExportNodeToStlTriangles(node, System.Numerics.Matrix4x4.Identity, triangles);

        using var ms = new MemoryStream();
        using var writer = new BinaryWriter(ms);

        var header = new byte[80];
        Encoding.ASCII.GetBytes("semio STL", 0, 9, header, 0);
        writer.Write(header);
        writer.Write((uint)triangles.Count);

        foreach (var (normal, v0, v1, v2) in triangles)
        {
            writer.Write(normal.X); writer.Write(normal.Y); writer.Write(normal.Z);
            writer.Write(v0.X); writer.Write(v0.Y); writer.Write(v0.Z);
            writer.Write(v1.X); writer.Write(v1.Y); writer.Write(v1.Z);
            writer.Write(v2.X); writer.Write(v2.Y); writer.Write(v2.Z);
            writer.Write((ushort)0);
        }

        return ms.ToArray();
    }

    private static void ExportNodeToStlTriangles(GltfNode node, System.Numerics.Matrix4x4 parentWorld,
        List<(System.Numerics.Vector3, System.Numerics.Vector3, System.Numerics.Vector3, System.Numerics.Vector3)> triangles)
    {
        var worldMatrix = node.LocalMatrix * parentWorld;

        if (node.Mesh != null)
        {
            foreach (var srcPrim in node.Mesh.Primitives)
            {
                var posAccessor = srcPrim.GetVertexAccessor("POSITION");
                if (posAccessor == null) continue;
                var positions = posAccessor.AsVector3Array();

                var idxAccessor = srcPrim.IndexAccessor;
                if (idxAccessor != null)
                {
                    var indices = idxAccessor.AsIndicesArray();
                    for (int i = 0; i + 2 < indices.Count; i += 3)
                    {
                        var p0 = System.Numerics.Vector3.Transform(positions[(int)indices[i]], worldMatrix);
                        var p1 = System.Numerics.Vector3.Transform(positions[(int)indices[i + 1]], worldMatrix);
                        var p2 = System.Numerics.Vector3.Transform(positions[(int)indices[i + 2]], worldMatrix);
                        var normal = System.Numerics.Vector3.Normalize(
                            System.Numerics.Vector3.Cross(p1 - p0, p2 - p0));
                        if (float.IsNaN(normal.X)) normal = System.Numerics.Vector3.UnitZ;
                        triangles.Add((normal, p0, p1, p2));
                    }
                }
            }
        }

        foreach (var child in node.VisualChildren)
            ExportNodeToStlTriangles(child, worldMatrix, triangles);
    }

    #endregion 🔧Kit Model Export Helpers
}

#endregion 🔩Kit Model Export






#region 🎪ZipRoundtrip
// Callers MUST use these methods to import and export kits as ZIP archives.

public class KitImportResult
{
    public Kit Kit { get; set; } = new();
    public Dictionary<string, byte[]> Files { get; set; } = new();
}

public static class ZipRoundtrip
{
    private static string BuildFolderPath(Kit kit, string folderGuid)
    {
        if (kit.Folders == null) return "";
        foreach (var f in kit.Folders)
        {
            if (f.Guid == folderGuid)
            {
                if (f.Parent != null)
                {
                    var parentPath = BuildFolderPath(kit, f.Parent?.Guid);
                    if (!string.IsNullOrEmpty(parentPath))
                        return $"{parentPath}/{f.Name}";
                }
                return f.Name;
            }
        }
        return "";
    }

    private static string BuildFilePath(Kit kit, File file)
    {
        if (file.Folder != null && !string.IsNullOrEmpty(file.Folder.Guid))
        {
            var folderPath = BuildFolderPath(kit, file.Folder.Guid);
            if (!string.IsNullOrEmpty(folderPath))
                return $"{folderPath}/{file.Name}";
        }
        return file.Name;
    }

    public static KitImportResult ImportKit(string zipPath)
    {
        var result = new KitImportResult();
        var tempDir = Path.Combine(Path.GetTempPath(), $"semio-kit-{System.Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);

        try
        {
            ZipFile.ExtractToDirectory(zipPath, tempDir);

            var kitDbPath = Path.Combine(tempDir, ".semio", "kit.db");
            var kitJsonPath = Path.Combine(tempDir, "kit.json");

            if (System.IO.File.Exists(kitDbPath))
            {
                result.Kit = KitSqlite.LoadKit(tempDir);
            }
            else if (System.IO.File.Exists(kitJsonPath))
            {
                var kitJson = System.IO.File.ReadAllText(kitJsonPath);
                result.Kit = Utility.Deserialize<Kit>(kitJson)!;
            }
            else
            {
                throw new FileNotFoundException("No kit data found in zip (expected .semio/kit.db or kit.json)");
            }

            foreach (var file in Directory.GetFiles(tempDir, "*", SearchOption.AllDirectories))
            {
                var relativePath = file.Substring(tempDir.Length).TrimStart(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar).Replace("\\", "/");
                if (relativePath != "kit.json" && !relativePath.StartsWith(".semio/"))
                    result.Files[relativePath] = System.IO.File.ReadAllBytes(file);
            }

            if (result.Kit.Files != null)
            {
                foreach (var kitFile in result.Kit.Files)
                {
                    var filePath = BuildFilePath(result.Kit, kitFile);
                    if (result.Files.TryGetValue(filePath, out var bytes))
                    {
                        kitFile.Blob = $"data:application/octet-stream;base64,{Convert.ToBase64String(bytes)}";
                    }
                }
            }
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }

        return result;
    }

    public static void ExportKit(Kit kit, string zipPath)
    {
        var tempDir = Path.Combine(Path.GetTempPath(), $"semio-kit-{System.Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);

        try
        {

            var kitForZip = kitJson_StripBlobs(kit);
            var kitJsonStr = Utility.Serialize(kitForZip);
            System.IO.File.WriteAllText(Path.Combine(tempDir, "kit.json"), kitJsonStr);

            if (kit.Files != null)
            {
                foreach (var file in kit.Files)
                {
                    if (!string.IsNullOrEmpty(file.Blob))
                    {
                        var filePath = BuildFilePath(kit, file);
                        var fullPath = Path.Combine(tempDir, filePath);
                        var dir = Path.GetDirectoryName(fullPath);
                        if (!string.IsNullOrEmpty(dir))
                            Directory.CreateDirectory(dir);
                        var blobData = file.Blob.StartsWith("data:") && file.Blob.Contains(",")
                            ? file.Blob.Substring(file.Blob.IndexOf(',') + 1)
                            : file.Blob;
                        System.IO.File.WriteAllBytes(fullPath, Convert.FromBase64String(blobData));
                    }
                }
            }

            if (System.IO.File.Exists(zipPath))
                System.IO.File.Delete(zipPath);
            ZipFile.CreateFromDirectory(tempDir, zipPath);
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }
    }

    private static Kit kitJson_StripBlobs(Kit kit)
    {
        var json = Utility.Serialize(kit);
        var clone = Utility.Deserialize<Kit>(json)!;
        if (clone.Files != null)
        {
            foreach (var file in clone.Files)
            {
                file.Blob = null;
            }
        }
        return clone;
    }

    private static Kit LoadKitFromSqlite(string dbPath)
    {
        using var connection = new SqliteConnection($"Data Source={dbPath}");
        connection.Open();

        var kit = new Kit();

        using (var cmd = connection.CreateCommand())
        {
            cmd.CommandText = "SELECT guid, name, version, description, icon, image, preview, remote, homepage, license FROM kit LIMIT 1";
            using var reader = cmd.ExecuteReader();
            if (reader.Read())
            {
                kit.Guid = reader.GetString(0);
                kit.Name = reader.GetString(1);
                kit.Version = reader.IsDBNull(2) ? "" : reader.GetString(2);
                kit.Description = reader.IsDBNull(3) ? "" : reader.GetString(3);
                kit.Icon = reader.IsDBNull(4) ? "" : reader.GetString(4);
                kit.Image = reader.IsDBNull(5) ? "" : reader.GetString(5);
                kit.Preview = reader.IsDBNull(6) ? "" : reader.GetString(6);
                kit.Remote = reader.IsDBNull(7) ? "" : reader.GetString(7);
                kit.Homepage = reader.IsDBNull(8) ? "" : reader.GetString(8);
                kit.License = reader.IsDBNull(9) ? "" : reader.GetString(9);
            }
        }

        kit.Types = LoadTypes(connection, kit.Guid);
        kit.Designs = LoadDesigns(connection, kit.Guid);

        return kit;
    }

    private static List<Type> LoadTypes(SqliteConnection connection, string kitGuid)
    {
        var types = new List<Type>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image FROM type WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);

        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var t = new Type
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Parent = reader.IsDBNull(2) ? null : new TypeId { Guid = reader.GetString(2) },
                IsAbstract = !reader.IsDBNull(3) && reader.GetBoolean(3),
                Folder = reader.IsDBNull(4) ? "" : reader.GetString(4),
                Stock = reader.IsDBNull(5) ? 2147483647 : reader.GetInt32(5),
                Virtual = !reader.IsDBNull(6) && reader.GetBoolean(6),
                Unit = reader.IsDBNull(7) ? "" : reader.GetString(7),
                Description = reader.IsDBNull(8) ? "" : reader.GetString(8),
                Icon = reader.IsDBNull(9) ? "" : reader.GetString(9),
                Image = reader.IsDBNull(10) ? "" : reader.GetString(10)
            };
            types.Add(t);
        }
        return types;
    }

    private static List<Design> LoadDesigns(SqliteConnection connection, string kitGuid)
    {
        var designs = new List<Design>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, parent_guid, unit, folder, is_abstract, can_scale, can_mirror, description, icon, image FROM design WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);

        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var d = new Design
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Parent = reader.IsDBNull(2) ? null : new DesignId { Guid = reader.GetString(2) },
                Unit = reader.IsDBNull(3) ? "" : reader.GetString(3),
                Folder = reader.IsDBNull(4) ? "" : reader.GetString(4),
                IsAbstract = !reader.IsDBNull(5) && reader.GetBoolean(5),
                CanScale = reader.IsDBNull(6) || reader.GetBoolean(6),
                CanMirror = reader.IsDBNull(7) || reader.GetBoolean(7),
                Description = reader.IsDBNull(8) ? "" : reader.GetString(8),
                Icon = reader.IsDBNull(9) ? "" : reader.GetString(9),
                Image = reader.IsDBNull(10) ? "" : reader.GetString(10)
            };
            designs.Add(d);
        }
        return designs;
    }

    private static List<T> TopologicalSort<T>(IEnumerable<T> items, Func<T, string> getGuid, Func<T, string?> getParentGuid) where T : class
    {
        var itemsByGuid = items.ToDictionary(getGuid);
        var visited = new HashSet<string>();
        var result = new List<T>();

        void Visit(T item)
        {
            var guid = getGuid(item);
            if (visited.Contains(guid)) return;
            visited.Add(guid);

            var parentGuid = getParentGuid(item);
            if (parentGuid != null && itemsByGuid.TryGetValue(parentGuid, out var parent))
            {
                Visit(parent);
            }
            result.Add(item);
        }

        foreach (var item in items)
        {
            Visit(item);
        }

        return result;
    }

    private static void SaveKitToSqlite(Kit kit, string dbPath, string schemaSQL)
    {
        using var connection = new SqliteConnection($"Data Source={dbPath}");
        connection.Open();

        using (var cmd = connection.CreateCommand())
        {
            cmd.CommandText = schemaSQL;
            cmd.ExecuteNonQuery();
        }

        using (var cmd = connection.CreateCommand())
        {
            cmd.CommandText = @"INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated)
                VALUES (@guid, @name, @version, @description, @icon, @image, @preview, @remote, @homepage, @license, datetime('now'), datetime('now'))";
            cmd.Parameters.AddWithValue("@guid", kit.Guid);
            cmd.Parameters.AddWithValue("@name", kit.Name);
            cmd.Parameters.AddWithValue("@version", kit.Version);
            cmd.Parameters.AddWithValue("@description", kit.Description);
            cmd.Parameters.AddWithValue("@icon", kit.Icon);
            cmd.Parameters.AddWithValue("@image", kit.Image);
            cmd.Parameters.AddWithValue("@preview", kit.Preview);
            cmd.Parameters.AddWithValue("@remote", kit.Remote);
            cmd.Parameters.AddWithValue("@homepage", kit.Homepage);
            cmd.Parameters.AddWithValue("@license", kit.License);
            cmd.ExecuteNonQuery();
        }

        var sortedTypes = TopologicalSort(kit.Types, t => t.Guid, t => t.Parent?.Guid);
        foreach (var t in sortedTypes)
        {
            using var cmd = connection.CreateCommand();
            cmd.CommandText = @"INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid)
                VALUES (@guid, @name, @parent, @isAbstract, @folder, @stock, @virtual, @unit, @description, @icon, @image, datetime('now'), datetime('now'), @kitGuid)";
            cmd.Parameters.AddWithValue("@guid", t.Guid);
            cmd.Parameters.AddWithValue("@name", t.Name);
            cmd.Parameters.AddWithValue("@parent", (object?)t.Parent?.Guid ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@isAbstract", t.IsAbstract);
            cmd.Parameters.AddWithValue("@folder", t.Folder);
            cmd.Parameters.AddWithValue("@stock", t.Stock);
            cmd.Parameters.AddWithValue("@virtual", t.Virtual);
            cmd.Parameters.AddWithValue("@unit", t.Unit);
            cmd.Parameters.AddWithValue("@description", t.Description);
            cmd.Parameters.AddWithValue("@icon", t.Icon);
            cmd.Parameters.AddWithValue("@image", t.Image);
            cmd.Parameters.AddWithValue("@kitGuid", kit.Guid);
            cmd.ExecuteNonQuery();
        }

        var sortedDesigns = TopologicalSort(kit.Designs, d => d.Guid, d => d.Parent?.Guid);
        foreach (var d in sortedDesigns)
        {
            using var cmd = connection.CreateCommand();
            cmd.CommandText = @"INSERT INTO design (guid, name, parent_guid, unit, folder, is_abstract, can_scale, can_mirror, description, icon, image, created, updated, kit_guid)
                VALUES (@guid, @name, @parent, @unit, @folder, @isAbstract, @canScale, @canMirror, @description, @icon, @image, datetime('now'), datetime('now'), @kitGuid)";
            cmd.Parameters.AddWithValue("@guid", d.Guid);
            cmd.Parameters.AddWithValue("@name", d.Name);
            cmd.Parameters.AddWithValue("@parent", (object?)d.Parent?.Guid ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@unit", d.Unit);
            cmd.Parameters.AddWithValue("@folder", d.Folder);
            cmd.Parameters.AddWithValue("@isAbstract", d.IsAbstract);
            cmd.Parameters.AddWithValue("@canScale", d.CanScale);
            cmd.Parameters.AddWithValue("@canMirror", d.CanMirror);
            cmd.Parameters.AddWithValue("@description", d.Description);
            cmd.Parameters.AddWithValue("@icon", d.Icon);
            cmd.Parameters.AddWithValue("@image", d.Image);
            cmd.Parameters.AddWithValue("@kitGuid", kit.Guid);
            cmd.ExecuteNonQuery();
        }
    }
}

#endregion 🎪ZipRoundtrip





#region 🔓KitSqlite
// Callers MUST use KitSqlite for direct CRUD operations on local static SQLite kit databases.

/// <summary>🗄️Direct CRUD operations on local SQLite kit databases (.semio/kit.db).</summary>
public static class KitSqlite
{
    private static string GetDbPath(string kitDirectory) => Path.Combine(kitDirectory, ".semio", "kit.db");

    private static string GetSchemaSQL()
    {
        var possiblePaths = new[]
        {
            "../../../../../sqlite/schema.sql",
            "../../../../sqlite/schema.sql",
            "../../../sqlite/schema.sql",
            "../../sqlite/schema.sql",
            "../sqlite/schema.sql",
            "sqlite/schema.sql",
            "../../../../../sql/sqlite/semio/schema.sql",
            "../../../../sql/sqlite/semio/schema.sql",
            "../../../sql/sqlite/semio/schema.sql",
            "../../sql/sqlite/semio/schema.sql",
            "../sql/sqlite/semio/schema.sql",
            "sql/sqlite/semio/schema.sql"
        };

        foreach (var path in possiblePaths)
        {
            if (System.IO.File.Exists(path))
                return System.IO.File.ReadAllText(path);
        }

        var assemblyDir = Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location) ?? "";
        var assemblyPaths = new[]
        {
            Path.Combine(assemblyDir, "schema.sql"),
            Path.Combine(assemblyDir, "..", "sqlite", "schema.sql"),
            Path.Combine(assemblyDir, "..", "..", "sqlite", "schema.sql"),
            Path.Combine(assemblyDir, "..", "..", "..", "sqlite", "schema.sql"),
            Path.Combine(assemblyDir, "..", "..", "..", "..", "sqlite", "schema.sql"),
            Path.Combine(assemblyDir, "..", "..", "..", "..", "..", "sqlite", "schema.sql")
        };

        foreach (var path in assemblyPaths)
        {
            if (System.IO.File.Exists(path))
                return System.IO.File.ReadAllText(path);
        }

        throw new FileNotFoundException("Could not find schema.sql for SQLite kit operations");
    }

    private static SqliteConnection OpenConnection(string dbPath)
    {
        var connection = new SqliteConnection($"Data Source={dbPath}");
        connection.Open();
        using var pragma = connection.CreateCommand();
        pragma.CommandText = "PRAGMA foreign_keys = ON;";
        pragma.ExecuteNonQuery();
        return connection;
    }

    #region 🥈KitSqliteLoad
    // Load operations for reading a kit from a local SQLite database.

    public static Kit LoadKit(string kitDirectory)
    {
        var dbPath = GetDbPath(kitDirectory);
        if (!System.IO.File.Exists(dbPath))
            throw new FileNotFoundException($"Kit database not found at {dbPath}");

        using var connection = OpenConnection(dbPath);
        return LoadKitFromConnection(connection);
    }

    private static Kit LoadKitFromConnection(SqliteConnection connection)
    {
        var kit = new Kit();

        using (var cmd = connection.CreateCommand())
        {
            cmd.CommandText = "SELECT guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated FROM kit LIMIT 1";
            using var reader = cmd.ExecuteReader();
            if (reader.Read())
            {
                kit.Guid = reader.GetString(0);
                kit.Name = reader.GetString(1);
                kit.Version = reader.IsDBNull(2) ? "" : reader.GetString(2);
                kit.Description = reader.IsDBNull(3) ? "" : reader.GetString(3);
                kit.Icon = reader.IsDBNull(4) ? "" : reader.GetString(4);
                kit.Image = reader.IsDBNull(5) ? "" : reader.GetString(5);
                kit.Preview = reader.IsDBNull(6) ? "" : reader.GetString(6);
                kit.Remote = reader.IsDBNull(7) ? "" : reader.GetString(7);
                kit.Homepage = reader.IsDBNull(8) ? "" : reader.GetString(8);
                kit.License = reader.IsDBNull(9) ? "" : reader.GetString(9);
                kit.CreatedAt = reader.IsDBNull(10) ? "" : reader.GetString(10);
                kit.UpdatedAt = reader.IsDBNull(11) ? "" : reader.GetString(11);
            }
        }

        kit.Qualities = LoadQualities(connection, kit.Guid);
        kit.Ports = LoadPorts(connection, kit.Guid);
        kit.Tags = LoadTags(connection, kit.Guid);
        kit.Concepts = LoadConcepts(connection, kit.Guid);
        kit.Files = LoadFiles(connection, kit.Guid);
        kit.Folders = LoadFolders(connection, kit.Guid);
        kit.Authors = LoadAuthors(connection, kit.Guid);
        kit.Types = LoadTypes(connection, kit.Guid);
        kit.Designs = LoadDesigns(connection, kit.Guid);
        kit.Attributes = LoadAttributes(connection, "kit_guid", kit.Guid);

        return kit;
    }

    private static List<Quality> LoadQualities(SqliteConnection connection, string kitGuid)
    {
        var qualities = new List<Quality>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, key, name, kind, default_value, formula, default_si_unit, default_imperial_unit, min_value, min_excluded, max_value, max_excluded, can_scale, definition FROM quality WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var q = new Quality
            {
                Guid = reader.GetString(0),
                Key = reader.GetString(1),
                Name = reader.GetString(2),
                Kind = (QualityKind)reader.GetInt32(3),
                Default = reader.IsDBNull(4) ? 0 : reader.GetFloat(4),
                Formula = reader.IsDBNull(5) ? "" : reader.GetString(5),
                SI = reader.IsDBNull(6) ? "" : reader.GetString(6),
                Imperial = reader.IsDBNull(7) ? "" : reader.GetString(7),
                Min = reader.IsDBNull(8) ? 0 : reader.GetFloat(8),
                MinExcluded = !reader.IsDBNull(9) && reader.GetBoolean(9),
                Max = reader.IsDBNull(10) ? 0 : reader.GetFloat(10),
                MaxExcluded = !reader.IsDBNull(11) && reader.GetBoolean(11),
                Scalable = !reader.IsDBNull(12) && reader.GetBoolean(12)
            };
            q.Benchmarks = LoadBenchmarks(connection, q.Guid);
            q.Attributes = LoadAttributes(connection, "quality_guid", q.Guid);
            qualities.Add(q);
        }
        return qualities;
    }

    private static List<Benchmark> LoadBenchmarks(SqliteConnection connection, string qualityGuid)
    {
        var benchmarks = new List<Benchmark>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, icon, min_value, min_excluded, max_value, max_excluded, definition FROM benchmark WHERE quality_guid = @qualityGuid";
        cmd.Parameters.AddWithValue("@qualityGuid", qualityGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            benchmarks.Add(new Benchmark
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Icon = reader.IsDBNull(2) ? null : reader.GetString(2),
                Min = reader.IsDBNull(3) ? null : reader.GetFloat(3),
                MinExcluded = !reader.IsDBNull(4) && reader.GetBoolean(4),
                Max = reader.IsDBNull(5) ? null : reader.GetFloat(5),
                MaxExcluded = !reader.IsDBNull(6) && reader.GetBoolean(6)
            });
        }
        return benchmarks;
    }

    private static List<Port> LoadPorts(SqliteConnection connection, string kitGuid)
    {
        var ports = new List<Port>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, description, icon, max_children FROM port WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var p = new Port
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Description = reader.IsDBNull(2) ? null : reader.GetString(2),
                Icon = reader.IsDBNull(3) ? null : reader.GetString(3),
                MaxChildren = reader.IsDBNull(4) ? null : reader.GetInt32(4)
            };
            p.CompatiblePorts = LoadCompatiblePorts(connection, p.Guid);
            p.Attributes = LoadAttributes(connection, "port_guid", p.Guid);
            ports.Add(p);
        }
        return ports;
    }

    private static List<PortId> LoadCompatiblePorts(SqliteConnection connection, string portGuid)
    {
        var compatiblePorts = new List<PortId>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT compatible_port_guid FROM port_compatibility WHERE port_guid = @portGuid";
        cmd.Parameters.AddWithValue("@portGuid", portGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            compatiblePorts.Add(new PortId { Guid = reader.GetString(0) });
        }
        return compatiblePorts;
    }

    private static List<Tag> LoadTags(SqliteConnection connection, string kitGuid)
    {
        var tags = new List<Tag>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, description, icon FROM tag WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var t = new Tag
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Description = reader.IsDBNull(2) ? null : reader.GetString(2),
                Icon = reader.IsDBNull(3) ? null : reader.GetString(3)
            };
            t.Attributes = LoadAttributes(connection, "tag_guid", t.Guid);
            tags.Add(t);
        }
        return tags;
    }

    private static List<Concept> LoadConcepts(SqliteConnection connection, string kitGuid)
    {
        var concepts = new List<Concept>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, description, icon FROM concept WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var c = new Concept
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Description = reader.IsDBNull(2) ? null : reader.GetString(2),
                Icon = reader.IsDBNull(3) ? null : reader.GetString(3)
            };
            c.Attributes = LoadAttributes(connection, "concept_guid", c.Guid);
            concepts.Add(c);
        }
        return concepts;
    }

    private static List<File> LoadFiles(SqliteConnection connection, string kitGuid)
    {
        var files = new List<File>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, mime, folder_guid, size, hash, remote_url, created, updated FROM file WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var f = new File
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Mime = reader.IsDBNull(2) ? null : reader.GetString(2),
                Folder = reader.IsDBNull(3) ? null : new FolderId { Guid = reader.GetString(3) },
                Size = reader.IsDBNull(4) ? null : (int?)reader.GetInt64(4),
                Hash = reader.IsDBNull(5) ? null : reader.GetString(5),
                Remote = reader.IsDBNull(6) ? null : reader.GetString(6),
                CreatedAt = reader.IsDBNull(7) ? DateTime.MinValue : DateTime.Parse(reader.GetString(7), null, System.Globalization.DateTimeStyles.RoundtripKind),
                UpdatedAt = reader.IsDBNull(8) ? DateTime.MinValue : DateTime.Parse(reader.GetString(8), null, System.Globalization.DateTimeStyles.RoundtripKind)
            };
            files.Add(f);
        }
        return files;
    }

    private static List<Folder> LoadFolders(SqliteConnection connection, string kitGuid)
    {
        var folders = new List<Folder>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, parent_guid, created, updated FROM folder WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var f = new Folder
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Parent = reader.IsDBNull(2) ? null : new FolderId { Guid = reader.GetString(2) },
                CreatedAt = reader.IsDBNull(3) ? "" : reader.GetString(3),
                UpdatedAt = reader.IsDBNull(4) ? "" : reader.GetString(4)
            };
            f.Attributes = LoadAttributes(connection, "folder_guid", f.Guid);
            folders.Add(f);
        }
        return folders;
    }

    private static List<Author> LoadAuthors(SqliteConnection connection, string? kitGuid)
    {
        var authors = new List<Author>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, email FROM author WHERE kit_guid = @kitGuid";
        cmd.Parameters.AddWithValue("@kitGuid", (object?)kitGuid ?? DBNull.Value);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var a = new Author
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Email = reader.IsDBNull(2) ? null : reader.GetString(2)
            };
            a.Attributes = LoadAttributes(connection, "author_guid", a.Guid);
            authors.Add(a);
        }
        return authors;
    }

    private static List<Type> LoadTypes(SqliteConnection connection, string kitGuid)
    {
        var types = new List<Type>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, location_guid, description, icon, image, created, updated FROM type WHERE kit_guid = @kitGuid ORDER BY rowid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var t = new Type
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Parent = reader.IsDBNull(2) ? null : new TypeId { Guid = reader.GetString(2) },
                IsAbstract = reader.IsDBNull(3) ? (bool?)null : (reader.GetBoolean(3) ? true : (bool?)null),
                Folder = reader.IsDBNull(4) ? null : (reader.GetString(4) == "" ? null : reader.GetString(4)),
                Stock = reader.IsDBNull(5) ? 2147483647 : reader.GetInt32(5),
                Virtual = !reader.IsDBNull(6) && reader.GetBoolean(6),
                Unit = reader.IsDBNull(7) ? "" : reader.GetString(7),
                Location = reader.IsDBNull(8) ? null : LoadLocation(connection, reader.GetString(8)),
                Description = reader.IsDBNull(9) ? null : (reader.GetString(9) == "" ? null : reader.GetString(9)),
                Icon = reader.IsDBNull(10) ? null : (reader.GetString(10) == "" ? null : reader.GetString(10)),
                Image = reader.IsDBNull(11) ? null : (reader.GetString(11) == "" ? null : reader.GetString(11)),
                CreatedAt = reader.IsDBNull(12) ? DateTime.MinValue : DateTime.Parse(reader.GetString(12), null, System.Globalization.DateTimeStyles.RoundtripKind),
                UpdatedAt = reader.IsDBNull(13) ? DateTime.MinValue : DateTime.Parse(reader.GetString(13), null, System.Globalization.DateTimeStyles.RoundtripKind)
            };
            t.Connectors = LoadConnectors(connection, t.Guid);
            t.Models = LoadModels(connection, t.Guid);
            t.Props = LoadTypeProps(connection, t.Guid);
            t.Concepts = LoadTypeConcepts(connection, t.Guid);
            t.Authors = LoadTypeAuthors(connection, t.Guid);
            t.Attributes = LoadAttributes(connection, "type_guid", t.Guid);
            types.Add(t);
        }
        return types;
    }

    private static Location? LoadLocation(SqliteConnection connection, string locationGuid)
    {

        return null;
    }

    private static List<Connector> LoadConnectors(SqliteConnection connection, string typeGuid)
    {
        var connectors = new List<Connector>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, max_children, description FROM connector WHERE type_guid = @typeGuid ORDER BY rowid";
        cmd.Parameters.AddWithValue("@typeGuid", typeGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var c = new Connector
            {
                Guid = reader.GetString(0),
                Name = reader.IsDBNull(1) ? null : reader.GetString(1),
                Point = new Point
                {
                    X = reader.GetFloat(2),
                    Y = reader.GetFloat(3),
                    Z = reader.GetFloat(4)
                },
                Direction = new Vector
                {
                    X = reader.GetFloat(5),
                    Y = reader.GetFloat(6),
                    Z = reader.GetFloat(7)
                },
                T = reader.GetFloat(8),
                Mandatory = !reader.IsDBNull(9) && reader.GetBoolean(9),
                Port = reader.IsDBNull(10) ? null : new PortId { Guid = reader.GetString(10) },
                MaxChildren = reader.IsDBNull(11) ? null : reader.GetInt32(11),
                Description = reader.IsDBNull(12) ? null : reader.GetString(12)
            };
            c.Props = LoadConnectorProps(connection, c.Guid);
            c.Attributes = LoadAttributes(connection, "connector_guid", c.Guid);
            connectors.Add(c);
        }
        return connectors;
    }

    private static List<Prop> LoadConnectorProps(SqliteConnection connection, string connectorGuid)
    {
        var props = new List<Prop>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, key, value, unit, quality_guid FROM prop WHERE connector_guid = @connectorGuid";
        cmd.Parameters.AddWithValue("@connectorGuid", connectorGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var p = new Prop
            {
                Guid = reader.GetString(0),
                Quality = reader.IsDBNull(4) ? new QualityId() : new QualityId { Guid = reader.GetString(4) },
                Value = reader.GetFloat(2).ToString(),
                Unit = reader.IsDBNull(3) ? "" : reader.GetString(3)
            };
            props.Add(p);
        }
        return props;
    }

    private static List<Prop> LoadTypeProps(SqliteConnection connection, string typeGuid)
    {

        return new List<Prop>();
    }

    private static List<Model> LoadModels(SqliteConnection connection, string typeGuid)
    {
        var models = new List<Model>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, file_guid, name, description FROM model WHERE type_guid = @typeGuid ORDER BY rowid";
        cmd.Parameters.AddWithValue("@typeGuid", typeGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var m = new Model
            {
                Guid = reader.GetString(0),
                File = new FileId { Guid = reader.GetString(1) },
                Name = reader.IsDBNull(2) ? null : reader.GetString(2),
                Description = reader.IsDBNull(3) ? null : reader.GetString(3)
            };
            m.Tags = LoadModelTags(connection, m.Guid);
            m.Attributes = LoadAttributes(connection, "model_guid", m.Guid);
            models.Add(m);
        }
        return models;
    }

    private static List<TagId> LoadModelTags(SqliteConnection connection, string modelGuid)
    {
        var tags = new List<TagId>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT mt.tag_guid FROM model_tag mt JOIN tag t ON mt.tag_guid = t.guid WHERE mt.model_guid = @modelGuid ORDER BY t.rowid";
        cmd.Parameters.AddWithValue("@modelGuid", modelGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            tags.Add(new TagId { Guid = reader.GetString(0) });
        }
        return tags;
    }

    private static List<ConceptId> LoadTypeConcepts(SqliteConnection connection, string typeGuid)
    {
        var concepts = new List<ConceptId>();
        try
        {
            using var cmd = connection.CreateCommand();
            cmd.CommandText = "SELECT concept_guid FROM type_concept WHERE type_guid = @typeGuid";
            cmd.Parameters.AddWithValue("@typeGuid", typeGuid);
            using var reader = cmd.ExecuteReader();
            while (reader.Read())
            {
                concepts.Add(new ConceptId { Guid = reader.GetString(0) });
            }
        }
        catch (SqliteException)
        {

        }
        return concepts;
    }

    private static List<AuthorId> LoadTypeAuthors(SqliteConnection connection, string typeGuid)
    {
        var authors = new List<AuthorId>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT author_guid FROM type_author WHERE type_guid = @typeGuid ORDER BY rank";
        cmd.Parameters.AddWithValue("@typeGuid", typeGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            authors.Add(new AuthorId { Guid = reader.GetString(0) });
        }
        return authors;
    }

    private static List<Design> LoadDesigns(SqliteConnection connection, string kitGuid)
    {
        var designs = new List<Design>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, parent_guid, variant, view_center_u, view_center_v, view_zoom, unit, location_guid, active_layer_guid, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated FROM design WHERE kit_guid = @kitGuid ORDER BY rowid";
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var d = new Design
            {
                Guid = reader.GetString(0),
                Name = reader.GetString(1),
                Parent = reader.IsDBNull(2) ? null : new DesignId { Guid = reader.GetString(2) },
                Unit = reader.IsDBNull(7) ? "" : reader.GetString(7),
                Location = reader.IsDBNull(8) ? null : LoadLocation(connection, reader.GetString(8)),
                ActiveLayer = reader.IsDBNull(9) ? null : new LayerId { Guid = reader.GetString(9) },
                IsAbstract = reader.IsDBNull(10) ? (bool?)null : (reader.GetBoolean(10) ? true : (bool?)null),
                Folder = reader.IsDBNull(11) ? null : (reader.GetString(11) == "" ? null : reader.GetString(11)),
                CanScale = reader.IsDBNull(12) ? (bool?)null : (reader.GetBoolean(12) ? true : (bool?)null),
                CanMirror = reader.IsDBNull(13) ? (bool?)null : (reader.GetBoolean(13) ? true : (bool?)null),
                Description = reader.IsDBNull(14) ? null : (reader.GetString(14) == "" ? null : reader.GetString(14)),
                Icon = reader.IsDBNull(15) ? null : (reader.GetString(15) == "" ? null : reader.GetString(15)),
                Image = reader.IsDBNull(16) ? null : (reader.GetString(16) == "" ? null : reader.GetString(16)),
                CreatedAt = reader.IsDBNull(17) ? DateTime.MinValue : DateTime.Parse(reader.GetString(17), null, System.Globalization.DateTimeStyles.RoundtripKind),
                UpdatedAt = reader.IsDBNull(18) ? DateTime.MinValue : DateTime.Parse(reader.GetString(18), null, System.Globalization.DateTimeStyles.RoundtripKind)
            };
            d.Pieces = LoadPieces(connection, d.Guid);
            d.Connections = LoadConnections(connection, d.Guid);
            d.Layers = LoadLayers(connection, d.Guid);
            d.Groups = LoadGroups(connection, d.Guid);
            d.Stats = LoadStats(connection, d.Guid);
            d.Concepts = LoadDesignConcepts(connection, d.Guid);
            d.Authors = LoadDesignAuthors(connection, d.Guid);
            d.Attributes = LoadAttributes(connection, "design_guid", d.Guid);
            designs.Add(d);
        }
        return designs;
    }

    private static List<Piece> LoadPieces(SqliteConnection connection, string designGuid)
    {
        var pieces = new List<Piece>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"SELECT guid, name, type_guid, design_guid_ref,
            plane_origin_x, plane_origin_y, plane_origin_z,
            plane_x_axis_x, plane_x_axis_y, plane_x_axis_z,
            plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
            center_u, center_v, scale,
            mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z,
            mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z,
            mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z,
            is_hidden, is_locked, color, description
            FROM piece WHERE design_guid = @designGuid";
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var p = new Piece
            {
                Guid = reader.GetString(0),
                Name = reader.IsDBNull(1) ? null : reader.GetString(1),
                Type = reader.IsDBNull(2) ? null : new TypeId { Guid = reader.GetString(2) },
                Design = reader.IsDBNull(3) ? null : new DesignId { Guid = reader.GetString(3) },
                Plane = (reader.IsDBNull(4)) ? null : new Plane
                {
                    Origin = new Point { X = reader.GetFloat(4), Y = reader.GetFloat(5), Z = reader.GetFloat(6) },
                    XAxis = new Vector { X = reader.GetFloat(7), Y = reader.GetFloat(8), Z = reader.GetFloat(9) },
                    YAxis = new Vector { X = reader.GetFloat(10), Y = reader.GetFloat(11), Z = reader.GetFloat(12) }
                },
                Center = (reader.IsDBNull(13)) ? null : new Coord { U = reader.GetFloat(13), V = reader.GetFloat(14) },
                Scale = reader.IsDBNull(15) ? null : reader.GetFloat(15),
                MirrorPlane = (reader.IsDBNull(16)) ? null : new Plane
                {
                    Origin = new Point { X = reader.GetFloat(16), Y = reader.GetFloat(17), Z = reader.GetFloat(18) },
                    XAxis = new Vector { X = reader.GetFloat(19), Y = reader.GetFloat(20), Z = reader.GetFloat(21) },
                    YAxis = new Vector { X = reader.GetFloat(22), Y = reader.GetFloat(23), Z = reader.GetFloat(24) }
                },
                IsHidden = !reader.IsDBNull(25) && reader.GetBoolean(25),
                IsLocked = !reader.IsDBNull(26) && reader.GetBoolean(26),
                Color = reader.IsDBNull(27) ? null : reader.GetString(27),
                Description = reader.IsDBNull(28) ? null : reader.GetString(28)
            };
            p.Props = LoadPieceProps(connection, p.Guid);
            p.Attributes = LoadAttributes(connection, "piece_guid", p.Guid);
            pieces.Add(p);
        }
        return pieces;
    }

    private static List<Prop> LoadPieceProps(SqliteConnection connection, string pieceGuid)
    {
        var props = new List<Prop>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT p.guid, p.key, p.value, p.unit, p.quality_guid FROM prop p INNER JOIN piece_prop pp ON p.guid = pp.prop_guid WHERE pp.piece_guid = @pieceGuid";
        cmd.Parameters.AddWithValue("@pieceGuid", pieceGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var p = new Prop
            {
                Guid = reader.GetString(0),
                Quality = reader.IsDBNull(4) ? new QualityId() : new QualityId { Guid = reader.GetString(4) },
                Value = reader.GetFloat(2).ToString(),
                Unit = reader.IsDBNull(3) ? "" : reader.GetString(3)
            };
            props.Add(p);
        }
        return props;
    }

    private static List<Connection> LoadConnections(SqliteConnection connection, string designGuid)
    {
        var connections = new List<Connection>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"SELECT guid,
            connected_piece_guid, connected_design_piece_guid, connected_connector_guid,
            connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid,
            gap, shift, rise, rotation, turn, tilt, u, v, description
            FROM connection WHERE design_guid = @designGuid";
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            connections.Add(new Connection
            {
                Guid = reader.GetString(0),
                Connected = new Side
                {
                    Piece = new PieceId { Guid = reader.GetString(1) },
                    DesignPiece = reader.IsDBNull(2) ? null : new PieceId { Guid = reader.GetString(2) },
                    Connector = new ConnectorId { Guid = reader.GetString(3) }
                },
                Connecting = new Side
                {
                    Piece = new PieceId { Guid = reader.GetString(4) },
                    DesignPiece = reader.IsDBNull(5) ? null : new PieceId { Guid = reader.GetString(5) },
                    Connector = new ConnectorId { Guid = reader.GetString(6) }
                },
                Gap = reader.GetFloat(7),
                Shift = reader.GetFloat(8),
                Rise = reader.GetFloat(9),
                Rotation = reader.GetFloat(10),
                Turn = reader.GetFloat(11),
                Tilt = reader.GetFloat(12),
                U = reader.IsDBNull(13) ? null : reader.GetFloat(13),
                V = reader.IsDBNull(14) ? null : reader.GetFloat(14),
                Description = reader.IsDBNull(15) ? null : reader.GetString(15)
            });
        }
        foreach (var conn in connections)
        {
            conn.Attributes = LoadAttributes(connection, "connection_guid", conn.Guid);
        }
        return connections;
    }

    private static List<Layer> LoadLayers(SqliteConnection connection, string designGuid)
    {
        var layers = new List<Layer>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, path, is_hidden, is_locked, color, description FROM layer WHERE design_guid = @designGuid";
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var l = new Layer
            {
                Guid = reader.GetString(0),
                Path = reader.GetString(1),
                IsHidden = !reader.IsDBNull(2) && reader.GetBoolean(2),
                IsLocked = !reader.IsDBNull(3) && reader.GetBoolean(3),
                Color = reader.IsDBNull(4) ? null : reader.GetString(4),
                Description = reader.IsDBNull(5) ? null : reader.GetString(5)
            };
            l.Attributes = LoadAttributes(connection, "layer_guid", l.Guid);
            layers.Add(l);
        }
        return layers;
    }

    private static List<Group> LoadGroups(SqliteConnection connection, string designGuid)
    {
        var groups = new List<Group>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, name, color, description FROM \"group\" WHERE design_guid = @designGuid";
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            var g = new Group
            {
                Guid = reader.GetString(0),
                Name = reader.IsDBNull(1) ? null : reader.GetString(1),
                Color = reader.IsDBNull(2) ? null : reader.GetString(2),
                Description = reader.IsDBNull(3) ? null : reader.GetString(3)
            };
            g.Pieces = LoadGroupPieces(connection, g.Guid);
            g.Attributes = LoadAttributes(connection, "group_guid", g.Guid);
            groups.Add(g);
        }
        return groups;
    }

    private static List<PieceId> LoadGroupPieces(SqliteConnection connection, string groupGuid)
    {
        var pieces = new List<PieceId>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT piece_guid FROM group_piece WHERE group_guid = @groupGuid";
        cmd.Parameters.AddWithValue("@groupGuid", groupGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            pieces.Add(new PieceId { Guid = reader.GetString(0) });
        }
        return pieces;
    }

    private static List<Stat> LoadStats(SqliteConnection connection, string designGuid)
    {
        var stats = new List<Stat>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT guid, quality_guid, min_value, min_excluded, max_value, max_excluded, unit FROM stat WHERE design_guid = @designGuid";
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            stats.Add(new Stat
            {
                Guid = reader.GetString(0),
                Quality = new QualityId { Guid = reader.GetString(1) },
                Min = reader.IsDBNull(2) ? null : reader.GetFloat(2),
                MinExcluded = !reader.IsDBNull(3) && reader.GetBoolean(3),
                Max = reader.IsDBNull(4) ? null : reader.GetFloat(4),
                MaxExcluded = !reader.IsDBNull(5) && reader.GetBoolean(5),
                Unit = reader.IsDBNull(6) ? null : reader.GetString(6)
            });
        }
        return stats;
    }

    private static List<ConceptId> LoadDesignConcepts(SqliteConnection connection, string designGuid)
    {
        var concepts = new List<ConceptId>();
        try
        {
            using var cmd = connection.CreateCommand();
            cmd.CommandText = "SELECT concept_guid FROM design_concept WHERE design_guid = @designGuid";
            cmd.Parameters.AddWithValue("@designGuid", designGuid);
            using var reader = cmd.ExecuteReader();
            while (reader.Read())
            {
                concepts.Add(new ConceptId { Guid = reader.GetString(0) });
            }
        }
        catch (SqliteException)
        {

        }
        return concepts;
    }

    private static List<AuthorId> LoadDesignAuthors(SqliteConnection connection, string designGuid)
    {
        var authors = new List<AuthorId>();
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "SELECT author_guid FROM design_author WHERE design_guid = @designGuid ORDER BY rank";
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            authors.Add(new AuthorId { Guid = reader.GetString(0) });
        }
        return authors;
    }

    private static List<Attribute> LoadAttributes(SqliteConnection connection, string foreignKeyColumn, string foreignKeyValue)
    {
        var attributes = new List<Attribute>();
        try
        {
            using var cmd = connection.CreateCommand();
            cmd.CommandText = $"SELECT guid, key, value, definition FROM attribute WHERE {foreignKeyColumn} = @fk";
            cmd.Parameters.AddWithValue("@fk", foreignKeyValue);
            using var reader = cmd.ExecuteReader();
            while (reader.Read())
            {
                attributes.Add(new Attribute
                {
                    Guid = reader.GetString(0),
                    Key = reader.GetString(1),
                    Value = reader.IsDBNull(2) ? null : reader.GetString(2),
                    Definition = reader.IsDBNull(3) ? null : reader.GetString(3)
                });
            }
        }
        catch (SqliteException)
        {

        }
        return attributes;
    }

    #endregion 🥈KitSqliteLoad

    #region 🏗️KitSqliteSave
    // Save operations for writing a kit to a local SQLite database.

    public static void SaveKit(string kitDirectory, Kit kit)
    {
        var semioDir = Path.Combine(kitDirectory, ".semio");
        Directory.CreateDirectory(semioDir);
        var dbPath = GetDbPath(kitDirectory);

        if (System.IO.File.Exists(dbPath))
            System.IO.File.Delete(dbPath);

        using var connection = OpenConnection(dbPath);
        var schemaSQL = GetSchemaSQL();
        using (var cmd = connection.CreateCommand())
        {
            cmd.CommandText = schemaSQL;
            cmd.ExecuteNonQuery();
        }

        SaveKitToConnection(connection, kit);
    }

    private static void SaveKitToConnection(SqliteConnection connection, Kit kit)
    {
        // Disable FK enforcement during save: the schema has an FK mismatch
        // where attribute.connector_guid references connector(guid) but connector.guid
        // only has a composite UNIQUE (guid, type_guid), not a standalone UNIQUE constraint.
        using (var fkOff = connection.CreateCommand())
        {
            fkOff.CommandText = "PRAGMA foreign_keys = OFF;";
            fkOff.ExecuteNonQuery();
        }
        using var transaction = connection.BeginTransaction();

        using (var cmd = connection.CreateCommand())
        {
            cmd.CommandText = "INSERT INTO semio (release, engine, created) VALUES (@release, @engine, datetime('now'))";
            cmd.Parameters.AddWithValue("@release", Constants.Release);
            cmd.Parameters.AddWithValue("@engine", "net");
            cmd.ExecuteNonQuery();
        }

        using (var cmd = connection.CreateCommand())
        {
            cmd.CommandText = @"INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated)
                VALUES (@guid, @name, @version, @description, @icon, @image, @preview, @remote, @homepage, @license, datetime('now'), datetime('now'))";
            cmd.Parameters.AddWithValue("@guid", kit.Guid);
            cmd.Parameters.AddWithValue("@name", kit.Name);
            cmd.Parameters.AddWithValue("@version", (object?)kit.Version ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@description", (object?)kit.Description ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@icon", (object?)kit.Icon ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@image", (object?)kit.Image ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@preview", (object?)kit.Preview ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@remote", (object?)kit.Remote ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@homepage", (object?)kit.Homepage ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@license", (object?)kit.License ?? DBNull.Value);
            cmd.ExecuteNonQuery();
        }

        SaveAttributes(connection, kit.Attributes, "kit_guid", kit.Guid);

        foreach (var quality in kit.Qualities)
            SaveQuality(connection, quality, kit.Guid);

        foreach (var port in kit.Ports)
            SavePort(connection, port, kit.Guid);

        foreach (var port in kit.Ports)
            SavePortCompatibility(connection, port);

        foreach (var tag in kit.Tags)
            SaveTag(connection, tag, kit.Guid);

        foreach (var concept in kit.Concepts)
            SaveConcept(connection, concept, kit.Guid);

        foreach (var folder in TopologicalSort(kit.Folders, f => f.Guid, f => f.Parent?.Guid))
            SaveFolder(connection, folder, kit.Guid);

        foreach (var file in kit.Files)
            SaveFile(connection, file, kit.Guid);

        foreach (var author in kit.Authors)
            SaveAuthor(connection, author, kit.Guid, null, null);

        foreach (var type in TopologicalSort(kit.Types, t => t.Guid, t => t.Parent?.Guid))
            SaveType(connection, type, kit.Guid);

        foreach (var design in TopologicalSort(kit.Designs, d => d.Guid, d => d.Parent?.Guid))
            SaveDesign(connection, design, kit.Guid);

        transaction.Commit();

        using (var fkOn = connection.CreateCommand())
        {
            fkOn.CommandText = "PRAGMA foreign_keys = ON;";
            fkOn.ExecuteNonQuery();
        }
    }

    private static List<T> TopologicalSort<T>(IEnumerable<T> items, Func<T, string> getGuid, Func<T, string?> getParentGuid) where T : class
    {
        var itemsByGuid = items.ToDictionary(getGuid);
        var visited = new HashSet<string>();
        var result = new List<T>();

        void Visit(T item)
        {
            var guid = getGuid(item);
            if (visited.Contains(guid)) return;
            visited.Add(guid);
            var parentGuid = getParentGuid(item);
            if (parentGuid != null && itemsByGuid.TryGetValue(parentGuid, out var parent))
                Visit(parent);
            result.Add(item);
        }

        foreach (var item in items)
            Visit(item);

        return result;
    }

    private static void SaveQuality(SqliteConnection connection, Quality quality, string kitGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"INSERT INTO quality (guid, key, name, kind, default_value, formula, default_si_unit, default_imperial_unit, min_value, min_excluded, max_value, max_excluded, can_scale, definition, kit_guid)
            VALUES (@guid, @key, @name, @kind, @defaultValue, @formula, @defaultSiUnit, @defaultImperialUnit, @minValue, @minExcluded, @maxValue, @maxExcluded, @canScale, @definition, @kitGuid)";
        cmd.Parameters.AddWithValue("@guid", quality.Guid);
        cmd.Parameters.AddWithValue("@key", quality.Key);
        cmd.Parameters.AddWithValue("@name", quality.Name);
        cmd.Parameters.AddWithValue("@kind", (int)quality.Kind);
        cmd.Parameters.AddWithValue("@defaultValue", (object?)quality.Default ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@formula", (object?)quality.Formula ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@defaultSiUnit", (object?)quality.SI ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@defaultImperialUnit", (object?)quality.Imperial ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@minValue", (object?)quality.Min ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@minExcluded", (object?)quality.MinExcluded ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@maxValue", (object?)quality.Max ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@maxExcluded", (object?)quality.MaxExcluded ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@canScale", quality.Scalable ?? false);
        cmd.Parameters.AddWithValue("@definition", (object?)quality.Formula ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        cmd.ExecuteNonQuery();

        foreach (var benchmark in quality.Benchmarks ?? new List<Benchmark>())
            SaveBenchmark(connection, benchmark, quality.Guid);

        SaveAttributes(connection, quality.Attributes, "quality_guid", quality.Guid);
    }

    private static void SaveBenchmark(SqliteConnection connection, Benchmark benchmark, string qualityGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"INSERT INTO benchmark (guid, name, icon, min_value, min_excluded, max_value, max_excluded, definition, quality_guid)
            VALUES (@guid, @name, @icon, @minValue, @minExcluded, @maxValue, @maxExcluded, @definition, @qualityGuid)";
        cmd.Parameters.AddWithValue("@guid", benchmark.Guid);
        cmd.Parameters.AddWithValue("@name", benchmark.Name);
        cmd.Parameters.AddWithValue("@icon", (object?)benchmark.Icon ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@minValue", (object?)benchmark.Min ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@minExcluded", (object?)benchmark.MinExcluded ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@maxValue", (object?)benchmark.Max ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@maxExcluded", (object?)benchmark.MaxExcluded ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@definition", DBNull.Value);
        cmd.Parameters.AddWithValue("@qualityGuid", qualityGuid);
        cmd.ExecuteNonQuery();
    }

    private static void SavePort(SqliteConnection connection, Port port, string kitGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO port (guid, name, description, icon, max_children, kit_guid) VALUES (@guid, @name, @description, @icon, @maxChildren, @kitGuid)";
        cmd.Parameters.AddWithValue("@guid", port.Guid);
        cmd.Parameters.AddWithValue("@name", port.Name);
        cmd.Parameters.AddWithValue("@description", (object?)port.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@icon", (object?)port.Icon ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@maxChildren", (object?)port.MaxChildren ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        cmd.ExecuteNonQuery();

        SaveAttributes(connection, port.Attributes, "port_guid", port.Guid);
    }

    private static void SavePortCompatibility(SqliteConnection connection, Port port)
    {
        foreach (var compatible in port.CompatiblePorts ?? new List<PortId>())
        {
            using var compatCmd = connection.CreateCommand();
            compatCmd.CommandText = "INSERT OR IGNORE INTO port_compatibility (port_guid, compatible_port_guid) VALUES (@portGuid, @compatiblePortGuid)";
            compatCmd.Parameters.AddWithValue("@portGuid", port.Guid);
            compatCmd.Parameters.AddWithValue("@compatiblePortGuid", compatible.Guid);
            compatCmd.ExecuteNonQuery();
        }
    }

    private static void SaveTag(SqliteConnection connection, Tag tag, string kitGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO tag (guid, name, description, icon, kit_guid) VALUES (@guid, @name, @description, @icon, @kitGuid)";
        cmd.Parameters.AddWithValue("@guid", tag.Guid);
        cmd.Parameters.AddWithValue("@name", tag.Name);
        cmd.Parameters.AddWithValue("@description", (object?)tag.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@icon", (object?)tag.Icon ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        cmd.ExecuteNonQuery();
        SaveAttributes(connection, tag.Attributes, "tag_guid", tag.Guid);
    }

    private static void SaveConcept(SqliteConnection connection, Concept concept, string kitGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO concept (guid, name, description, icon, kit_guid) VALUES (@guid, @name, @description, @icon, @kitGuid)";
        cmd.Parameters.AddWithValue("@guid", concept.Guid);
        cmd.Parameters.AddWithValue("@name", concept.Name);
        cmd.Parameters.AddWithValue("@description", (object?)concept.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@icon", (object?)concept.Icon ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        cmd.ExecuteNonQuery();
        SaveAttributes(connection, concept.Attributes, "concept_guid", concept.Guid);
    }

    private static void SaveFolder(SqliteConnection connection, Folder folder, string kitGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO folder (guid, name, parent_guid, created, updated, kit_guid) VALUES (@guid, @name, @parentGuid, datetime('now'), datetime('now'), @kitGuid)";
        cmd.Parameters.AddWithValue("@guid", folder.Guid);
        cmd.Parameters.AddWithValue("@name", folder.Name);
        cmd.Parameters.AddWithValue("@parentGuid", (object?)folder.Parent ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        cmd.ExecuteNonQuery();
        SaveAttributes(connection, folder.Attributes, "folder_guid", folder.Guid);
    }

    private static void SaveFile(SqliteConnection connection, File file, string kitGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"INSERT INTO file (guid, name, mime, folder_guid, size, hash, remote_url, created, updated, kit_guid)
            VALUES (@guid, @name, @mime, @folderGuid, @size, @hash, @remoteUrl, datetime('now'), datetime('now'), @kitGuid)";
        cmd.Parameters.AddWithValue("@guid", file.Guid);
        cmd.Parameters.AddWithValue("@name", file.Name);
        cmd.Parameters.AddWithValue("@mime", (object?)file.Mime ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@folderGuid", (object?)file.Folder?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@size", (object?)file.Size ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@hash", (object?)file.Hash ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@remoteUrl", (object?)file.Remote ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        cmd.ExecuteNonQuery();
    }

    private static void SaveAuthor(SqliteConnection connection, Author author, string? kitGuid, string? typeGuid, string? designGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO author (guid, name, email, kit_guid, type_guid, design_guid) VALUES (@guid, @name, @email, @kitGuid, @typeGuid, @designGuid)";
        cmd.Parameters.AddWithValue("@guid", author.Guid);
        cmd.Parameters.AddWithValue("@name", author.Name);
        cmd.Parameters.AddWithValue("@email", (object?)author.Email ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", (object?)kitGuid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@typeGuid", (object?)typeGuid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@designGuid", (object?)designGuid ?? DBNull.Value);
        cmd.ExecuteNonQuery();
        SaveAttributes(connection, author.Attributes, "author_guid", author.Guid);
    }

    private static void SaveType(SqliteConnection connection, Type type, string kitGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, description, icon, image, created, updated, kit_guid)
            VALUES (@guid, @name, @parent, @isAbstract, @folder, @stock, @virtual, @unit, @description, @icon, @image, datetime('now'), datetime('now'), @kitGuid)";
        cmd.Parameters.AddWithValue("@guid", type.Guid);
        cmd.Parameters.AddWithValue("@name", type.Name);
        cmd.Parameters.AddWithValue("@parent", (object?)type.Parent?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@isAbstract", type.IsAbstract ?? false);
        cmd.Parameters.AddWithValue("@folder", (object?)type.Folder ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@stock", (object?)type.Stock ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@virtual", type.Virtual ?? false);
        cmd.Parameters.AddWithValue("@unit", (object?)type.Unit ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@description", (object?)type.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@icon", (object?)type.Icon ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@image", (object?)type.Image ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        cmd.ExecuteNonQuery();

        foreach (var connector in type.Connectors ?? new List<Connector>())
            SaveConnector(connection, connector, type.Guid);

        foreach (var model in type.Models ?? new List<Model>())
            SaveModel(connection, model, type.Guid);

        for (int i = 0; i < (type.Concepts?.Count ?? 0); i++)
        {
            using var conceptCmd = connection.CreateCommand();
            conceptCmd.CommandText = "INSERT INTO type_concept (type_guid, concept_guid) VALUES (@typeGuid, @conceptGuid)";
            conceptCmd.Parameters.AddWithValue("@typeGuid", type.Guid);
            conceptCmd.Parameters.AddWithValue("@conceptGuid", type.Concepts![i].Guid);
            conceptCmd.ExecuteNonQuery();
        }

        for (int i = 0; i < (type.Authors?.Count ?? 0); i++)
        {
            using var authorCmd = connection.CreateCommand();
            authorCmd.CommandText = "INSERT INTO type_author (type_guid, author_guid, rank) VALUES (@typeGuid, @authorGuid, @rank)";
            authorCmd.Parameters.AddWithValue("@typeGuid", type.Guid);
            authorCmd.Parameters.AddWithValue("@authorGuid", type.Authors![i].Guid);
            authorCmd.Parameters.AddWithValue("@rank", i);
            authorCmd.ExecuteNonQuery();
        }

        SaveAttributes(connection, type.Attributes, "type_guid", type.Guid);
    }

    private static void SaveConnector(SqliteConnection connection, Connector connector, string typeGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, max_children, description, type_guid)
            VALUES (@guid, @name, @px, @py, @pz, @dx, @dy, @dz, @t, @mandatory, @portGuid, @maxChildren, @description, @typeGuid)";
        cmd.Parameters.AddWithValue("@guid", connector.Guid);
        cmd.Parameters.AddWithValue("@name", (object?)connector.Name ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@px", connector.Point?.X ?? 0f);
        cmd.Parameters.AddWithValue("@py", connector.Point?.Y ?? 0f);
        cmd.Parameters.AddWithValue("@pz", connector.Point?.Z ?? 0f);
        cmd.Parameters.AddWithValue("@dx", connector.Direction?.X ?? 0f);
        cmd.Parameters.AddWithValue("@dy", connector.Direction?.Y ?? 0f);
        cmd.Parameters.AddWithValue("@dz", connector.Direction?.Z ?? 0f);
        cmd.Parameters.AddWithValue("@t", connector.T);
        cmd.Parameters.AddWithValue("@mandatory", (object?)connector.Mandatory ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@portGuid", (object?)connector.Port?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@maxChildren", (object?)connector.MaxChildren ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@description", (object?)connector.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@typeGuid", typeGuid);
        cmd.ExecuteNonQuery();

        foreach (var prop in connector.Props ?? new List<Prop>())
            SaveProp(connection, prop, connector.Guid);

        SaveAttributes(connection, connector.Attributes, "connector_guid", connector.Guid);
    }

    private static void SaveProp(SqliteConnection connection, Prop prop, string connectorGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO prop (guid, key, value, unit, quality_guid, connector_guid) VALUES (@guid, @key, @value, @unit, @qualityGuid, @connectorGuid)";
        cmd.Parameters.AddWithValue("@guid", prop.Guid);
        cmd.Parameters.AddWithValue("@key", prop.Quality?.Guid ?? "");
        cmd.Parameters.AddWithValue("@value", prop.Value);
        cmd.Parameters.AddWithValue("@unit", (object?)prop.Unit ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@qualityGuid", (object?)prop.Quality?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@connectorGuid", connectorGuid);
        cmd.ExecuteNonQuery();
    }

    private static void SaveModel(SqliteConnection connection, Model model, string typeGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO model (guid, file_guid, name, description, type_guid) VALUES (@guid, @fileGuid, @name, @description, @typeGuid)";
        cmd.Parameters.AddWithValue("@guid", model.Guid);
        cmd.Parameters.AddWithValue("@fileGuid", model.File?.Guid ?? "");
        cmd.Parameters.AddWithValue("@name", (object?)model.Name ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@description", (object?)model.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@typeGuid", typeGuid);
        cmd.ExecuteNonQuery();

        foreach (var tag in model.Tags ?? new List<TagId>())
        {
            using var tagCmd = connection.CreateCommand();
            tagCmd.CommandText = "INSERT INTO model_tag (model_guid, tag_guid) VALUES (@modelGuid, @tagGuid)";
            tagCmd.Parameters.AddWithValue("@modelGuid", model.Guid);
            tagCmd.Parameters.AddWithValue("@tagGuid", tag.Guid);
            tagCmd.ExecuteNonQuery();
        }

        SaveAttributes(connection, model.Attributes, "model_guid", model.Guid);
    }

    private static void SaveDesign(SqliteConnection connection, Design design, string kitGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"INSERT INTO design (guid, name, parent_guid, variant, view_center_u, view_center_v, view_zoom, unit, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_guid)
            VALUES (@guid, @name, @parent, @variant, @viewCenterU, @viewCenterV, @viewZoom, @unit, @isAbstract, @folder, @canScale, @canMirror, @description, @icon, @image, datetime('now'), datetime('now'), @kitGuid)";
        cmd.Parameters.AddWithValue("@guid", design.Guid);
        cmd.Parameters.AddWithValue("@name", design.Name);
        cmd.Parameters.AddWithValue("@parent", (object?)design.Parent?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@variant", DBNull.Value);
        cmd.Parameters.AddWithValue("@viewCenterU", DBNull.Value);
        cmd.Parameters.AddWithValue("@viewCenterV", DBNull.Value);
        cmd.Parameters.AddWithValue("@viewZoom", DBNull.Value);
        cmd.Parameters.AddWithValue("@unit", (object?)design.Unit ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@isAbstract", (object?)design.IsAbstract ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@folder", (object?)design.Folder ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@canScale", (object?)design.CanScale ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@canMirror", (object?)design.CanMirror ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@description", (object?)design.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@icon", (object?)design.Icon ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@image", (object?)design.Image ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@kitGuid", kitGuid);
        cmd.ExecuteNonQuery();

        foreach (var piece in design.Pieces ?? new List<Piece>())
            SavePiece(connection, piece, design.Guid);

        foreach (var conn in design.Connections ?? new List<Connection>())
            SaveConnection(connection, conn, design.Guid);

        foreach (var layer in design.Layers ?? new List<Layer>())
            SaveLayer(connection, layer, design.Guid);

        foreach (var group in design.Groups ?? new List<Group>())
            SaveGroup(connection, group, design.Guid);

        foreach (var stat in design.Stats ?? new List<Stat>())
            SaveStat(connection, stat, design.Guid);

        for (int i = 0; i < (design.Concepts?.Count ?? 0); i++)
        {
            using var conceptCmd = connection.CreateCommand();
            conceptCmd.CommandText = "INSERT INTO design_concept (design_guid, concept_guid) VALUES (@designGuid, @conceptGuid)";
            conceptCmd.Parameters.AddWithValue("@designGuid", design.Guid);
            conceptCmd.Parameters.AddWithValue("@conceptGuid", design.Concepts![i].Guid);
            conceptCmd.ExecuteNonQuery();
        }

        for (int i = 0; i < (design.Authors?.Count ?? 0); i++)
        {
            using var authorCmd = connection.CreateCommand();
            authorCmd.CommandText = "INSERT INTO design_author (design_guid, author_guid, rank) VALUES (@designGuid, @authorGuid, @rank)";
            authorCmd.Parameters.AddWithValue("@designGuid", design.Guid);
            authorCmd.Parameters.AddWithValue("@authorGuid", design.Authors![i].Guid);
            authorCmd.Parameters.AddWithValue("@rank", i);
            authorCmd.ExecuteNonQuery();
        }

        SaveAttributes(connection, design.Attributes, "design_guid", design.Guid);
    }

    private static void SavePiece(SqliteConnection connection, Piece piece, string designGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"INSERT INTO piece (guid, name, type_guid, design_guid_ref,
            plane_origin_x, plane_origin_y, plane_origin_z,
            plane_x_axis_x, plane_x_axis_y, plane_x_axis_z,
            plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
            center_u, center_v, scale,
            mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z,
            mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z,
            mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z,
            is_hidden, is_locked, color, description, design_guid)
            VALUES (@guid, @name, @typeGuid, @designGuidRef,
            @pox, @poy, @poz, @pxx, @pxy, @pxz, @pyx, @pyy, @pyz,
            @cu, @cv, @scale,
            @mpox, @mpoy, @mpoz, @mpxx, @mpxy, @mpxz, @mpyx, @mpyy, @mpyz,
            @isHidden, @isLocked, @color, @description, @designGuid)";
        cmd.Parameters.AddWithValue("@guid", piece.Guid);
        cmd.Parameters.AddWithValue("@name", (object?)piece.Name ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@typeGuid", (object?)piece.Type?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@designGuidRef", (object?)piece.Design?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@pox", (object?)piece.Plane?.Origin?.X ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@poy", (object?)piece.Plane?.Origin?.Y ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@poz", (object?)piece.Plane?.Origin?.Z ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@pxx", (object?)piece.Plane?.XAxis?.X ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@pxy", (object?)piece.Plane?.XAxis?.Y ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@pxz", (object?)piece.Plane?.XAxis?.Z ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@pyx", (object?)piece.Plane?.YAxis?.X ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@pyy", (object?)piece.Plane?.YAxis?.Y ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@pyz", (object?)piece.Plane?.YAxis?.Z ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@cu", (object?)piece.Center?.U ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@cv", (object?)piece.Center?.V ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@scale", (object?)piece.Scale ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpox", (object?)piece.MirrorPlane?.Origin?.X ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpoy", (object?)piece.MirrorPlane?.Origin?.Y ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpoz", (object?)piece.MirrorPlane?.Origin?.Z ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpxx", (object?)piece.MirrorPlane?.XAxis?.X ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpxy", (object?)piece.MirrorPlane?.XAxis?.Y ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpxz", (object?)piece.MirrorPlane?.XAxis?.Z ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpyx", (object?)piece.MirrorPlane?.YAxis?.X ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpyy", (object?)piece.MirrorPlane?.YAxis?.Y ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@mpyz", (object?)piece.MirrorPlane?.YAxis?.Z ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@isHidden", piece.IsHidden ?? false);
        cmd.Parameters.AddWithValue("@isLocked", piece.IsLocked ?? false);
        cmd.Parameters.AddWithValue("@color", (object?)piece.Color ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@description", (object?)piece.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        cmd.ExecuteNonQuery();

        foreach (var prop in piece.Props ?? new List<Prop>())
        {
            SaveProp(connection, prop, "");
            using var ppCmd = connection.CreateCommand();
            ppCmd.CommandText = "INSERT INTO piece_prop (piece_guid, prop_guid) VALUES (@pieceGuid, @propGuid)";
            ppCmd.Parameters.AddWithValue("@pieceGuid", piece.Guid);
            ppCmd.Parameters.AddWithValue("@propGuid", prop.Guid);
            ppCmd.ExecuteNonQuery();
        }

        SaveAttributes(connection, piece.Attributes, "piece_guid", piece.Guid);
    }

    private static void SaveConnection(SqliteConnection connection, Connection conn, string designGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = @"INSERT INTO connection (guid,
            connected_piece_guid, connected_design_piece_guid, connected_connector_guid,
            connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid,
            gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid)
            VALUES (@guid, @connectedPiece, @connectedDesignPiece, @connectedConnector,
            @connectingPiece, @connectingDesignPiece, @connectingConnector,
            @gap, @shift, @rise, @rotation, @turn, @tilt, @u, @v, @description, @designGuid)";
        cmd.Parameters.AddWithValue("@guid", conn.Guid);
        cmd.Parameters.AddWithValue("@connectedPiece", conn.Connected?.Piece?.Guid ?? "");
        cmd.Parameters.AddWithValue("@connectedDesignPiece", (object?)conn.Connected?.DesignPiece?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@connectedConnector", conn.Connected?.Connector?.Guid ?? "");
        cmd.Parameters.AddWithValue("@connectingPiece", conn.Connecting?.Piece?.Guid ?? "");
        cmd.Parameters.AddWithValue("@connectingDesignPiece", (object?)conn.Connecting?.DesignPiece?.Guid ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@connectingConnector", conn.Connecting?.Connector?.Guid ?? "");
        cmd.Parameters.AddWithValue("@gap", conn.Gap);
        cmd.Parameters.AddWithValue("@shift", conn.Shift);
        cmd.Parameters.AddWithValue("@rise", conn.Rise);
        cmd.Parameters.AddWithValue("@rotation", conn.Rotation);
        cmd.Parameters.AddWithValue("@turn", conn.Turn);
        cmd.Parameters.AddWithValue("@tilt", conn.Tilt);
        cmd.Parameters.AddWithValue("@u", (object?)conn.U ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@v", (object?)conn.V ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@description", (object?)conn.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        cmd.ExecuteNonQuery();
        SaveAttributes(connection, conn.Attributes, "connection_guid", conn.Guid);
    }

    private static void SaveLayer(SqliteConnection connection, Layer layer, string designGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO layer (guid, path, is_hidden, is_locked, color, description, design_guid) VALUES (@guid, @path, @isHidden, @isLocked, @color, @description, @designGuid)";
        cmd.Parameters.AddWithValue("@guid", layer.Guid);
        cmd.Parameters.AddWithValue("@path", layer.Path);
        cmd.Parameters.AddWithValue("@isHidden", layer.IsHidden ?? false);
        cmd.Parameters.AddWithValue("@isLocked", layer.IsLocked ?? false);
        cmd.Parameters.AddWithValue("@color", (object?)layer.Color ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@description", (object?)layer.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        cmd.ExecuteNonQuery();
        SaveAttributes(connection, layer.Attributes, "layer_guid", layer.Guid);
    }

    private static void SaveGroup(SqliteConnection connection, Group group, string designGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO \"group\" (guid, name, color, description, design_guid) VALUES (@guid, @name, @color, @description, @designGuid)";
        cmd.Parameters.AddWithValue("@guid", group.Guid);
        cmd.Parameters.AddWithValue("@name", (object?)group.Name ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@color", (object?)group.Color ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@description", (object?)group.Description ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        cmd.ExecuteNonQuery();

        foreach (var piece in group.Pieces ?? new List<PieceId>())
        {
            using var gpCmd = connection.CreateCommand();
            gpCmd.CommandText = "INSERT INTO group_piece (group_guid, piece_guid) VALUES (@groupGuid, @pieceGuid)";
            gpCmd.Parameters.AddWithValue("@groupGuid", group.Guid);
            gpCmd.Parameters.AddWithValue("@pieceGuid", piece.Guid);
            gpCmd.ExecuteNonQuery();
        }

        SaveAttributes(connection, group.Attributes, "group_guid", group.Guid);
    }

    private static void SaveStat(SqliteConnection connection, Stat stat, string designGuid)
    {
        using var cmd = connection.CreateCommand();
        cmd.CommandText = "INSERT INTO stat (guid, quality_guid, min_value, min_excluded, max_value, max_excluded, unit, design_guid) VALUES (@guid, @qualityGuid, @minValue, @minExcluded, @maxValue, @maxExcluded, @unit, @designGuid)";
        cmd.Parameters.AddWithValue("@guid", stat.Guid);
        cmd.Parameters.AddWithValue("@qualityGuid", stat.Quality?.Guid ?? "");
        cmd.Parameters.AddWithValue("@minValue", (object?)stat.Min ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@minExcluded", stat.MinExcluded);
        cmd.Parameters.AddWithValue("@maxValue", (object?)stat.Max ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@maxExcluded", stat.MaxExcluded);
        cmd.Parameters.AddWithValue("@unit", (object?)stat.Unit ?? DBNull.Value);
        cmd.Parameters.AddWithValue("@designGuid", designGuid);
        cmd.ExecuteNonQuery();
    }

    private static void SaveAttributes(SqliteConnection connection, List<Attribute>? attributes, string foreignKeyColumn, string foreignKeyValue)
    {
        if (attributes == null) return;
        foreach (var attr in attributes)
        {
            using var cmd = connection.CreateCommand();
            cmd.CommandText = $"INSERT INTO attribute (guid, key, value, definition, {foreignKeyColumn}) VALUES (@guid, @key, @value, @definition, @fk)";
            cmd.Parameters.AddWithValue("@guid", attr.Guid);
            cmd.Parameters.AddWithValue("@key", attr.Key);
            cmd.Parameters.AddWithValue("@value", (object?)attr.Value ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@definition", (object?)attr.Definition ?? DBNull.Value);
            cmd.Parameters.AddWithValue("@fk", foreignKeyValue);
            cmd.ExecuteNonQuery();
        }
    }

    #endregion 🏗️KitSqliteSave

    #region 🥁KitSqliteChange
    // Change-based CRUD commands returning KitChange for undo/redo support.

    public static KitChange ApplyKitDiff(string kitDirectory, KitDiff diff)
    {
        var before = LoadKit(kitDirectory);
        var after = SemioDiff.ApplyKitDiff(before, diff);
        SaveKit(kitDirectory, after);
        var backward = SemioDiff.InverseKitDiff(before, diff);
        return new KitChange { Forward = diff, Backward = backward, Before = before, After = after };
    }

    public static KitChange AddTypeToKit(string kitDirectory, Type type)
    {
        var diff = new KitDiff { Types = new TypesDiff { Added = new List<Type> { type } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange RemoveTypeFromKit(string kitDirectory, string typeGuid)
    {
        var diff = new KitDiff { Types = new TypesDiff { Removed = new List<TypeId> { new() { Guid = typeGuid } } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange SetTypeInKit(string kitDirectory, Type type)
    {
        var diff = new KitDiff { Types = new TypesDiff { Added = new List<Type> { type } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange AddDesignToKit(string kitDirectory, Design design)
    {
        var diff = new KitDiff { Designs = new DesignsDiff { Added = new List<Design> { design } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange RemoveDesignFromKit(string kitDirectory, string designGuid)
    {
        var diff = new KitDiff { Designs = new DesignsDiff { Removed = new List<DesignId> { new() { Guid = designGuid } } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange SetDesignInKit(string kitDirectory, Design design)
    {
        var diff = new KitDiff { Designs = new DesignsDiff { Added = new List<Design> { design } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange AddPortToKit(string kitDirectory, Port port)
    {
        var diff = new KitDiff { Ports = new PortsDiff { Added = new List<Port> { port } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange RemovePortFromKit(string kitDirectory, string portGuid)
    {
        var diff = new KitDiff { Ports = new PortsDiff { Removed = new List<PortId> { new() { Guid = portGuid } } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange AddTagToKit(string kitDirectory, Tag tag)
    {
        var diff = new KitDiff { Tags = new TagsDiff { Added = new List<Tag> { tag } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange RemoveTagFromKit(string kitDirectory, string tagGuid)
    {
        var diff = new KitDiff { Tags = new TagsDiff { Removed = new List<TagId> { new() { Guid = tagGuid } } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange AddConceptToKit(string kitDirectory, Concept concept)
    {
        var diff = new KitDiff { Concepts = new ConceptsDiff { Added = new List<Concept> { concept } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange RemoveConceptFromKit(string kitDirectory, string conceptGuid)
    {
        var diff = new KitDiff { Concepts = new ConceptsDiff { Removed = new List<ConceptId> { new() { Guid = conceptGuid } } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange AddFileToKit(string kitDirectory, File file)
    {
        var diff = new KitDiff { Files = new FilesDiff { Added = new List<File> { file } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange RemoveFileFromKit(string kitDirectory, string fileGuid)
    {
        var diff = new KitDiff { Files = new FilesDiff { Removed = new List<FileId> { new() { Guid = fileGuid } } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange AddAttributeToKit(string kitDirectory, Attribute attribute)
    {
        var diff = new KitDiff { Attributes = new AttributesDiff { Added = new List<Attribute> { attribute } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static KitChange RemoveAttributeFromKit(string kitDirectory, string attributeGuid)
    {
        var diff = new KitDiff { Attributes = new AttributesDiff { Removed = new List<AttributeId> { new() { Guid = attributeGuid } } } };
        return ApplyKitDiff(kitDirectory, diff);
    }

    public static bool KitExists(string kitDirectory)
    {
        return System.IO.File.Exists(GetDbPath(kitDirectory));
    }

    public static void CreateKit(string kitDirectory, Kit kit)
    {
        SaveKit(kitDirectory, kit);
    }

    public static void DeleteKit(string kitDirectory)
    {
        var dbPath = GetDbPath(kitDirectory);
        SqliteConnection.ClearAllPools();
        if (System.IO.File.Exists(dbPath))
            System.IO.File.Delete(dbPath);
    }

    #endregion 🥁KitSqliteChange
}

#endregion 🔓KitSqlite






#region 📷FileKit
// Callers MUST use FileKit for JSON file kit import, export, and edit operations.

public static class FileKit
{
    public static Kit Import(string path)
    {
        var json = System.IO.File.ReadAllText(path);
        return Utility.Deserialize<Kit>(json)!;
    }

    public static void Export(Kit kit, string path)
    {
        System.IO.File.WriteAllText(path, Utility.Serialize(kit));
    }

    public static Kit Edit(string path, KitDiff diff)
    {
        var edited = TemporaryKit.Edit(Import(path), diff);
        Export(edited, path);
        return edited;
    }
}

#endregion 📷FileKit






#region 🏰FolderKit
// Callers MUST use FolderKit for local folder kit import, export, and edit operations.

public static class FolderKit
{
    private static string BuildFolderPath(Kit kit, string folderGuid)
    {
        foreach (var folder in kit.Folders ?? new List<Folder>())
        {
            if (folder.Guid != folderGuid) continue;
            if (folder.Parent != null && !string.IsNullOrEmpty(folder.Parent.Guid))
            {
                var parentPath = BuildFolderPath(kit, folder.Parent.Guid);
                if (!string.IsNullOrEmpty(parentPath)) return $"{parentPath}/{folder.Name}";
            }
            return folder.Name;
        }
        return "";
    }

    private static string BuildFilePath(Kit kit, File file)
    {
        if (file.Folder != null && !string.IsNullOrEmpty(file.Folder.Guid))
        {
            var folderPath = BuildFolderPath(kit, file.Folder.Guid);
            if (!string.IsNullOrEmpty(folderPath)) return $"{folderPath}/{file.Name}";
        }
        return file.Name;
    }

    private static Dictionary<string, byte[]> CollectKitFiles(Kit kit)
    {
        var files = new Dictionary<string, byte[]>();
        foreach (var file in kit.Files ?? new List<File>())
        {
            if (string.IsNullOrEmpty(file.Blob)) continue;
            var blobData = file.Blob.StartsWith("data:") && file.Blob.Contains(",")
                ? file.Blob.Substring(file.Blob.IndexOf(',') + 1)
                : file.Blob;
            files[BuildFilePath(kit, file)] = Convert.FromBase64String(blobData);
        }
        return files;
    }

    private static void HydrateKitFiles(Kit kit, Dictionary<string, byte[]> files)
    {
        foreach (var file in kit.Files ?? new List<File>())
        {
            var path = BuildFilePath(kit, file);
            if (files.TryGetValue(path, out var bytes))
                file.Blob = $"data:application/octet-stream;base64,{Convert.ToBase64String(bytes)}";
        }
    }

    public static KitImportResult Import(string folderPath)
    {
        var kit = KitSqlite.LoadKit(folderPath);
        var files = new Dictionary<string, byte[]>();
        foreach (var file in Directory.GetFiles(folderPath, "*", SearchOption.AllDirectories))
        {
            var relativePath = file.Substring(folderPath.Length).TrimStart(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar).Replace("\\", "/");
            if (relativePath.StartsWith(".semio/")) continue;
            files[relativePath] = System.IO.File.ReadAllBytes(file);
        }
        HydrateKitFiles(kit, files);
        return new KitImportResult { Kit = kit, Files = files };
    }

    public static void Export(Kit kit, string folderPath)
    {
        Directory.CreateDirectory(folderPath);
        var dbPath = Path.Combine(folderPath, ".semio", "kit.db");
        if (System.IO.File.Exists(dbPath))
        {
            SqliteConnection.ClearAllPools();
            System.IO.File.Delete(dbPath);
        }
        KitSqlite.SaveKit(folderPath, kit);
        foreach (var entry in CollectKitFiles(kit))
        {
            var fullPath = Path.Combine(folderPath, entry.Key.Replace("/", Path.DirectorySeparatorChar.ToString()));
            Directory.CreateDirectory(Path.GetDirectoryName(fullPath)!);
            System.IO.File.WriteAllBytes(fullPath, entry.Value);
        }
    }

    public static Kit Edit(string folderPath, KitDiff diff)
    {
        var imported = Import(folderPath);
        var edited = TemporaryKit.Edit(imported.Kit, diff);
        Export(edited, folderPath);
        return edited;
    }
}

#endregion 🏰FolderKit






#region 📐ArchiveKit
// Callers MUST use ArchiveKit for ZIP archive import, export, and edit operations.

public static class ArchiveKit
{
    public static KitImportResult Import(string zipPath) => ZipRoundtrip.ImportKit(zipPath);

    public static void Export(Kit kit, string zipPath) => ZipRoundtrip.ExportKit(kit, zipPath);

    public static Kit Edit(string zipPath, KitDiff diff)
    {
        var imported = Import(zipPath);
        var edited = TemporaryKit.Edit(imported.Kit, diff);
        Export(edited, zipPath);
        return edited;
    }
}

#endregion 📐ArchiveKit






#region 🎆RemoteKit
// Callers MUST use RemoteKit for HTTP-based JSON and ZIP kit import and in-memory edits.

public static class RemoteKit
{
    public static KitImportResult Import(string url)
    {
        using var client = new HttpClient();
        using var response = client.GetAsync(url).GetAwaiter().GetResult();
        response.EnsureSuccessStatusCode();
        var contentType = response.Content.Headers.ContentType?.MediaType?.ToLowerInvariant() ?? "";
        var bytes = response.Content.ReadAsByteArrayAsync().GetAwaiter().GetResult();

        if (url.EndsWith(".zip", StringComparison.OrdinalIgnoreCase) || contentType.Contains("zip") || contentType.Contains("octet-stream") || (bytes.Length >= 4 && bytes[0] == (byte)'P' && bytes[1] == (byte)'K'))
        {
            var tempPath = Path.Combine(Path.GetTempPath(), $"semio-remote-{Guid.NewGuid():N}.zip");
            try
            {
                System.IO.File.WriteAllBytes(tempPath, bytes);
                return ArchiveKit.Import(tempPath);
            }
            finally
            {
                if (System.IO.File.Exists(tempPath)) System.IO.File.Delete(tempPath);
            }
        }

        var json = Encoding.UTF8.GetString(bytes);
        return new KitImportResult { Kit = Utility.Deserialize<Kit>(json)! };
    }

    public static Kit Edit(string url, KitDiff diff)
    {
        var imported = Import(url);
        return TemporaryKit.Edit(imported.Kit, diff);
    }
}

#endregion 🎆RemoteKit






#region 🔤TemporaryKit
// Callers MUST use TemporaryKit for in-memory kit edits without persistence.

public static class TemporaryKit
{
    public static Kit Edit(Kit kit, KitDiff diff)
    {
        var clone = Utility.Deserialize<Kit>(Utility.Serialize(kit))!;
        return SemioDiff.ApplyKitDiff(clone, diff);
    }
}

#endregion 🔤TemporaryKit






#region 📦KitImporter
// Callers MUST use ImportFromZip for high-level kit import.

public static class KitImporter
{
    public static KitImportResult ImportFromZip(string zipPath)
    {
        return ArchiveKit.Import(zipPath);
    }
}

#endregion 📦KitImporter






#region 🪁KitExporter
// Callers MUST use ExportToZip for high-level kit export.

public static class KitExporter
{
    public static void ExportToZip(Kit kit, string zipPath)
    {
        ArchiveKit.Export(kit, zipPath);
    }
}

#endregion 🪁KitExporter






#region ❄️SemioDiff
// Callers MUST use these methods for diff computation and application on kits.

public static class SemioDiff
{
    public static KitChange GetKitChange(Kit before, Kit after, string? author = null, DateTime? time = null)
    {
        var forward = GetKitDiff(before, after);
        var backward = InverseKitDiff(before, forward);
        return new KitChange { Forward = forward, Backward = backward, Author = author, Time = time, Before = before, After = after };
    }

    public static DesignChange GetDesignChange(Design before, Design after, string? author = null, DateTime? time = null)
    {
        var forward = GetDesignDiff(before, after) ?? new DesignDiff();
        var backward = GetDesignDiff(after, before) ?? new DesignDiff();
        return new DesignChange { Forward = forward, Backward = backward, Author = author, Time = time, Before = before, After = after };
    }

    public static KitDiff GetKitDiff(Kit before, Kit after)
    {
        var diff = new KitDiff();

        if (before.Name != after.Name) diff.Name = after.Name;
        if (before.Version != after.Version) diff.Version = after.Version;
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) diff.Description = after.Description;
        if (NormalizeString(before.Icon) != NormalizeString(after.Icon)) diff.Icon = after.Icon;
        if (NormalizeString(before.Image) != NormalizeString(after.Image)) diff.Image = after.Image;
        if (NormalizeString(before.Preview) != NormalizeString(after.Preview)) diff.Preview = after.Preview;
        if (NormalizeString(before.Remote) != NormalizeString(after.Remote)) diff.Remote = after.Remote;
        if (NormalizeString(before.Homepage) != NormalizeString(after.Homepage)) diff.Homepage = after.Homepage;
        if (NormalizeString(before.License) != NormalizeString(after.License)) diff.License = after.License;

        var typesDiff = GetTypesDiff(before.Types ?? new List<Type>(), after.Types ?? new List<Type>());
        if (typesDiff != null) diff.Types = typesDiff;
        var designsDiff = GetDesignsDiff(before.Designs ?? new List<Design>(), after.Designs ?? new List<Design>());
        if (designsDiff != null) diff.Designs = designsDiff;
        var tagsDiff = GetTagsDiff(before.Tags ?? new List<Tag>(), after.Tags ?? new List<Tag>());
        if (tagsDiff != null) diff.Tags = tagsDiff;
        var conceptsDiff = GetConceptsDiff(before.Concepts ?? new List<Concept>(), after.Concepts ?? new List<Concept>());
        if (conceptsDiff != null) diff.Concepts = conceptsDiff;
        var portsDiff = GetPortsDiff(before.Ports ?? new List<Port>(), after.Ports ?? new List<Port>());
        if (portsDiff != null) diff.Ports = portsDiff;
        var filesDiff = GetFilesDiff(before.Files ?? new List<File>(), after.Files ?? new List<File>());
        if (filesDiff != null) diff.Files = filesDiff;
        var foldersDiff = GetFoldersDiff(before.Folders ?? new List<Folder>(), after.Folders ?? new List<Folder>());
        if (foldersDiff != null) diff.Folders = foldersDiff;
        var kitAttributesDiff = GetAttributesDiff(before.Attributes ?? new List<Attribute>(), after.Attributes ?? new List<Attribute>());
        if (kitAttributesDiff != null) diff.Attributes = kitAttributesDiff;

        diff.Authors = GetAuthorsDiff(before.Authors ?? new List<Author>(), after.Authors ?? new List<Author>());

        return diff;
    }

    private static string? NormalizeString(string? value) => string.IsNullOrEmpty(value) ? null : value;

    private static TypesDiff? GetTypesDiff(List<Type> before, List<Type> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(t => new TypeId { Guid = t.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<TypeDiffUpdate>();

        foreach (var afterType in after)
        {
            var beforeType = before.FirstOrDefault(b => b.Guid == afterType.Guid);
            if (beforeType != null)
            {
                var typeDiff = GetTypeDiff(beforeType, afterType);
                if (typeDiff != null)
                    updated.Add(new TypeDiffUpdate { Type = afterType, Diff = typeDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new TypesDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static TypeDiff? GetTypeDiff(Type before, Type after)
    {
        var diff = new TypeDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (before.Parent?.Guid != after.Parent?.Guid) { diff.Parent = after.Parent; hasChanges = true; }
        if (before.IsAbstract != after.IsAbstract) { diff.IsAbstract = after.IsAbstract; hasChanges = true; }
        if (NormalizeString(before.Folder) != NormalizeString(after.Folder)) { diff.Folder = after.Folder; hasChanges = true; }
        if (before.Stock != after.Stock) { diff.Stock = after.Stock; hasChanges = true; }
        if (before.Virtual != after.Virtual) { diff.Virtual = after.Virtual; hasChanges = true; }
        if (before.Unit != after.Unit) { diff.Unit = after.Unit; hasChanges = true; }
        if (before.Location?.Guid != after.Location?.Guid) { diff.Location = after.Location; hasChanges = true; }
        if (NormalizeString(before.Icon) != NormalizeString(after.Icon)) { diff.Icon = after.Icon; hasChanges = true; }
        if (NormalizeString(before.Image) != NormalizeString(after.Image)) { diff.Image = after.Image; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }

        var beforeAuthors = (before.Authors ?? new List<AuthorId>()).Select(a => a.Guid).OrderBy(g => g).ToList();
        var afterAuthors = (after.Authors ?? new List<AuthorId>()).Select(a => a.Guid).OrderBy(g => g).ToList();
        if (!beforeAuthors.SequenceEqual(afterAuthors)) { diff.Authors = after.Authors; hasChanges = true; }

        var beforeConcepts = (before.Concepts ?? new List<ConceptId>()).Select(c => c.Guid).OrderBy(g => g).ToList();
        var afterConcepts = (after.Concepts ?? new List<ConceptId>()).Select(c => c.Guid).OrderBy(g => g).ToList();
        if (!beforeConcepts.SequenceEqual(afterConcepts)) { diff.Concepts = after.Concepts; hasChanges = true; }

        var connectorsDiff = GetConnectorsDiff(before.Connectors ?? new List<Connector>(), after.Connectors ?? new List<Connector>());
        if (connectorsDiff != null) { diff.Connectors = connectorsDiff; hasChanges = true; }

        var modelsDiff = GetModelsDiff(before.Models ?? new List<Model>(), after.Models ?? new List<Model>());
        if (modelsDiff != null) { diff.Models = modelsDiff; hasChanges = true; }

        var attributesDiff = GetAttributesDiff(before.Attributes ?? new List<Attribute>(), after.Attributes ?? new List<Attribute>());
        if (attributesDiff != null) { diff.Attributes = attributesDiff; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static ConnectorsDiff? GetConnectorsDiff(List<Connector> before, List<Connector> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(c => new ConnectorId { Guid = c.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<ConnectorDiffUpdate>();

        foreach (var afterConnector in after)
        {
            var beforeConnector = before.FirstOrDefault(b => b.Guid == afterConnector.Guid);
            if (beforeConnector != null)
            {
                var connectorDiff = GetConnectorDiff(beforeConnector, afterConnector);
                if (connectorDiff != null)
                    updated.Add(new ConnectorDiffUpdate { Connector = new ConnectorId { Guid = afterConnector.Guid }, Diff = connectorDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new ConnectorsDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static ConnectorDiff? GetConnectorDiff(Connector before, Connector after)
    {
        var diff = new ConnectorDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (before.Port?.Guid != after.Port?.Guid) { diff.Port = after.Port; hasChanges = true; }
        if (before.Mandatory != after.Mandatory) { diff.Mandatory = after.Mandatory; hasChanges = true; }
        if (before.T != after.T) { diff.T = after.T; hasChanges = true; }
        if (before.Point?.X != after.Point?.X || before.Point?.Y != after.Point?.Y || before.Point?.Z != after.Point?.Z)
        {
            diff.Point = GetPointDiff(before.Point, after.Point);
            hasChanges = true;
        }
        if (before.Direction?.X != after.Direction?.X || before.Direction?.Y != after.Direction?.Y || before.Direction?.Z != after.Direction?.Z)
        {
            diff.Direction = GetVectorDiff(before.Direction, after.Direction);
            hasChanges = true;
        }

        var attributesDiff = GetAttributesDiff(before.Attributes ?? new List<Attribute>(), after.Attributes ?? new List<Attribute>());
        if (attributesDiff != null) { diff.Attributes = attributesDiff; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static Point? GetPointDiff(Point? before, Point? after)
    {
        var bx = before?.X ?? 0; var by = before?.Y ?? 0; var bz = before?.Z ?? 0;
        var ax = after?.X ?? 0; var ay = after?.Y ?? 0; var az = after?.Z ?? 0;
        return new Point { X = ax - bx, Y = ay - by, Z = az - bz };
    }

    private static Vector? GetVectorDiff(Vector? before, Vector? after)
    {
        var bx = before?.X ?? 0; var by = before?.Y ?? 0; var bz = before?.Z ?? 0;
        var ax = after?.X ?? 0; var ay = after?.Y ?? 0; var az = after?.Z ?? 0;
        return new Vector { X = ax - bx, Y = ay - by, Z = az - bz };
    }

    private static Plane? GetPlaneDiff(Plane? before, Plane? after)
    {
        return new Plane
        {
            Origin = GetPointDiff(before?.Origin, after?.Origin),
            XAxis = GetVectorDiff(before?.XAxis, after?.XAxis),
            YAxis = GetVectorDiff(before?.YAxis, after?.YAxis)
        };
    }

    private static ModelsDiff? GetModelsDiff(List<Model> before, List<Model> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(m => new ModelId { Guid = m.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<ModelDiffUpdate>();

        foreach (var afterModel in after)
        {
            var beforeModel = before.FirstOrDefault(b => b.Guid == afterModel.Guid);
            if (beforeModel != null)
            {
                var modelDiff = GetModelDiff(beforeModel, afterModel);
                if (modelDiff != null)
                    updated.Add(new ModelDiffUpdate { Model = new ModelId { Guid = afterModel.Guid }, Diff = modelDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new ModelsDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static ModelDiff? GetModelDiff(Model before, Model after)
    {
        var diff = new ModelDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (before.File?.Guid != after.File?.Guid) { diff.File = after.File; hasChanges = true; }

        var beforeTags = (before.Tags ?? new List<TagId>()).Select(t => t.Guid).OrderBy(g => g).ToList();
        var afterTags = (after.Tags ?? new List<TagId>()).Select(t => t.Guid).OrderBy(g => g).ToList();
        if (!beforeTags.SequenceEqual(afterTags)) { diff.Tags = after.Tags ?? new List<TagId>(); hasChanges = true; }

        var attributesDiff = GetAttributesDiff(before.Attributes ?? new List<Attribute>(), after.Attributes ?? new List<Attribute>());
        if (attributesDiff != null) { diff.Attributes = attributesDiff; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static DesignsDiff? GetDesignsDiff(List<Design> before, List<Design> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(d => new DesignId { Guid = d.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<DesignDiffUpdate>();

        foreach (var afterDesign in after)
        {
            var beforeDesign = before.FirstOrDefault(b => b.Guid == afterDesign.Guid);
            if (beforeDesign != null)
            {
                var designDiff = GetDesignDiff(beforeDesign, afterDesign);
                if (designDiff != null)
                    updated.Add(new DesignDiffUpdate { Design = afterDesign, Diff = designDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new DesignsDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static DesignDiff? GetDesignDiff(Design before, Design after)
    {
        var diff = new DesignDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (before.Parent?.Guid != after.Parent?.Guid) { diff.Parent = after.Parent; hasChanges = true; }
        if (before.IsAbstract != after.IsAbstract) { diff.IsAbstract = after.IsAbstract; hasChanges = true; }
        if (NormalizeString(before.Folder) != NormalizeString(after.Folder)) { diff.Folder = after.Folder; hasChanges = true; }
        if (before.CanScale != after.CanScale) { diff.CanScale = after.CanScale; hasChanges = true; }
        if (before.CanMirror != after.CanMirror) { diff.CanMirror = after.CanMirror; hasChanges = true; }
        if (NormalizeString(before.Unit) != NormalizeString(after.Unit)) { diff.Unit = after.Unit; hasChanges = true; }
        if (before.ActiveLayer?.Guid != after.ActiveLayer?.Guid) { diff.ActiveLayer = after.ActiveLayer; hasChanges = true; }
        if (before.Location?.Guid != after.Location?.Guid) { diff.Location = after.Location; hasChanges = true; }
        if (NormalizeString(before.Icon) != NormalizeString(after.Icon)) { diff.Icon = after.Icon; hasChanges = true; }
        if (NormalizeString(before.Image) != NormalizeString(after.Image)) { diff.Image = after.Image; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }

        var beforeAuthors = (before.Authors ?? new List<AuthorId>()).Select(a => a.Guid).OrderBy(g => g).ToList();
        var afterAuthors = (after.Authors ?? new List<AuthorId>()).Select(a => a.Guid).OrderBy(g => g).ToList();
        if (!beforeAuthors.SequenceEqual(afterAuthors)) { diff.Authors = after.Authors; hasChanges = true; }

        var beforeConcepts = (before.Concepts ?? new List<ConceptId>()).Select(c => c.Guid).OrderBy(g => g).ToList();
        var afterConcepts = (after.Concepts ?? new List<ConceptId>()).Select(c => c.Guid).OrderBy(g => g).ToList();
        if (!beforeConcepts.SequenceEqual(afterConcepts)) { diff.Concepts = after.Concepts; hasChanges = true; }

        var piecesDiff = GetPiecesDiff(before.Pieces ?? new List<Piece>(), after.Pieces ?? new List<Piece>());
        if (piecesDiff != null) { diff.Pieces = piecesDiff; hasChanges = true; }

        var connectionsDiff = GetConnectionsDiff(before.Connections ?? new List<Connection>(), after.Connections ?? new List<Connection>());
        if (connectionsDiff != null) { diff.Connections = connectionsDiff; hasChanges = true; }

        var attributesDiff = GetAttributesDiff(before.Attributes ?? new List<Attribute>(), after.Attributes ?? new List<Attribute>());
        if (attributesDiff != null) { diff.Attributes = attributesDiff; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static PiecesDiff? GetPiecesDiff(List<Piece> before, List<Piece> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(p => new PieceId { Guid = p.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<PieceDiffUpdate>();

        foreach (var afterPiece in after)
        {
            var beforePiece = before.FirstOrDefault(b => b.Guid == afterPiece.Guid);
            if (beforePiece != null)
            {
                var pieceDiff = GetPieceDiff(beforePiece, afterPiece);
                if (pieceDiff != null)
                    updated.Add(new PieceDiffUpdate { Piece = new PieceId { Guid = afterPiece.Guid }, Diff = pieceDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new PiecesDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static PieceDiff? GetPieceDiff(Piece before, Piece after)
    {
        var diff = new PieceDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (before.Type?.Guid != after.Type?.Guid) { diff.Type = after.Type; hasChanges = true; }
        if (before.Design?.Guid != after.Design?.Guid) { diff.Design = after.Design; hasChanges = true; }
        if (!ArePlanesEqual(before.Plane, after.Plane))
        {
            diff.Plane = after.Plane != null ? GetPlaneDiff(before.Plane, after.Plane) : null;
            hasChanges = true;
        }
        if (!AreCoordsEqual(before.Center, after.Center)) { diff.Center = after.Center; hasChanges = true; }
        if (before.Scale != after.Scale) { diff.Scale = after.Scale; hasChanges = true; }
        if (!ArePlanesEqual(before.MirrorPlane, after.MirrorPlane)) { diff.MirrorPlane = after.MirrorPlane; hasChanges = true; }
        if (before.IsHidden != after.IsHidden) { diff.IsHidden = after.IsHidden; hasChanges = true; }
        if (before.IsLocked != after.IsLocked) { diff.IsLocked = after.IsLocked; hasChanges = true; }
        if (NormalizeString(before.Color) != NormalizeString(after.Color)) { diff.Color = after.Color; hasChanges = true; }

        var attributesDiff = GetAttributesDiff(before.Attributes ?? new List<Attribute>(), after.Attributes ?? new List<Attribute>());
        if (attributesDiff != null) { diff.Attributes = attributesDiff; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static bool ArePlanesEqual(Plane? a, Plane? b)
    {
        if (a == null && b == null) return true;
        if (a == null || b == null) return false;
        return a.Origin?.X == b.Origin?.X && a.Origin?.Y == b.Origin?.Y && a.Origin?.Z == b.Origin?.Z
            && a.XAxis?.X == b.XAxis?.X && a.XAxis?.Y == b.XAxis?.Y && a.XAxis?.Z == b.XAxis?.Z
            && a.YAxis?.X == b.YAxis?.X && a.YAxis?.Y == b.YAxis?.Y && a.YAxis?.Z == b.YAxis?.Z;
    }

    private static bool AreCoordsEqual(Coord? a, Coord? b)
    {
        if (a == null && b == null) return true;
        if (a == null || b == null) return false;
        return a.U == b.U && a.V == b.V;
    }

    private static ConnectionsDiff? GetConnectionsDiff(List<Connection> before, List<Connection> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(c => new ConnectionId { Guid = c.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<ConnectionDiffUpdate>();

        foreach (var afterConnection in after)
        {
            var beforeConnection = before.FirstOrDefault(b => b.Guid == afterConnection.Guid);
            if (beforeConnection != null)
            {
                var connectionDiff = GetConnectionDiff(beforeConnection, afterConnection);
                if (connectionDiff != null)
                    updated.Add(new ConnectionDiffUpdate { Connection = new ConnectionId { Guid = afterConnection.Guid }, Diff = connectionDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new ConnectionsDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static ConnectionDiff? GetConnectionDiff(Connection before, Connection after)
    {
        var diff = new ConnectionDiff();
        bool hasChanges = false;

        if (!before.Connected.Equals(after.Connected))
        {
            diff.Connected = GetSideDiff(before.Connected, after.Connected);
            hasChanges = true;
        }
        if (!before.Connecting.Equals(after.Connecting))
        {
            diff.Connecting = GetSideDiff(before.Connecting, after.Connecting);
            hasChanges = true;
        }
        if (before.Gap != after.Gap) { diff.Gap = after.Gap - before.Gap; hasChanges = true; }
        if (before.Shift != after.Shift) { diff.Shift = after.Shift - before.Shift; hasChanges = true; }
        if (before.Rise != after.Rise) { diff.Rise = after.Rise - before.Rise; hasChanges = true; }
        if (before.Rotation != after.Rotation) { diff.Rotation = after.Rotation - before.Rotation; hasChanges = true; }
        if (before.Turn != after.Turn) { diff.Turn = after.Turn - before.Turn; hasChanges = true; }
        if (before.Tilt != after.Tilt) { diff.Tilt = after.Tilt - before.Tilt; hasChanges = true; }
        if (before.U != after.U) { diff.U = (after.U ?? 0) - (before.U ?? 0); hasChanges = true; }
        if (before.V != after.V) { diff.V = (after.V ?? 0) - (before.V ?? 0); hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }

        var attributesDiff = GetAttributesDiff(before.Attributes ?? new List<Attribute>(), after.Attributes ?? new List<Attribute>());
        if (attributesDiff != null) { diff.Attributes = attributesDiff; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static SideDiff GetSideDiff(Side before, Side after)
    {
        var diff = new SideDiff();
        if (before.Piece?.Guid != after.Piece?.Guid) diff.Piece = after.Piece;
        if (before.DesignPiece?.Guid != after.DesignPiece?.Guid) diff.DesignPiece = after.DesignPiece;
        if (before.Connector?.Guid != after.Connector?.Guid) diff.Connector = after.Connector;
        return diff;
    }

    private static TagsDiff? GetTagsDiff(List<Tag> before, List<Tag> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(t => new TagId { Guid = t.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<TagDiffUpdate>();

        foreach (var afterTag in after)
        {
            var beforeTag = before.FirstOrDefault(b => b.Guid == afterTag.Guid);
            if (beforeTag != null)
            {
                var tagDiff = GetTagDiff(beforeTag, afterTag);
                if (tagDiff != null)
                    updated.Add(new TagDiffUpdate { Tag = new TagId { Guid = afterTag.Guid }, Diff = tagDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new TagsDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static TagDiff? GetTagDiff(Tag before, Tag after)
    {
        var diff = new TagDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (NormalizeString(before.Icon) != NormalizeString(after.Icon)) { diff.Icon = after.Icon; hasChanges = true; }

        var attributesDiff = GetAttributesDiff(before.Attributes ?? new List<Attribute>(), after.Attributes ?? new List<Attribute>());
        if (attributesDiff != null) { diff.Attributes = attributesDiff; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static ConceptsDiff? GetConceptsDiff(List<Concept> before, List<Concept> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(c => new ConceptId { Guid = c.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<ConceptDiffUpdate>();

        foreach (var afterConcept in after)
        {
            var beforeConcept = before.FirstOrDefault(b => b.Guid == afterConcept.Guid);
            if (beforeConcept != null)
            {
                var conceptDiff = GetConceptDiff(beforeConcept, afterConcept);
                if (conceptDiff != null)
                    updated.Add(new ConceptDiffUpdate { Concept = new ConceptId { Guid = afterConcept.Guid }, Diff = conceptDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new ConceptsDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static ConceptDiff? GetConceptDiff(Concept before, Concept after)
    {
        var diff = new ConceptDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (NormalizeString(before.Icon) != NormalizeString(after.Icon)) { diff.Icon = after.Icon; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static PortsDiff? GetPortsDiff(List<Port> before, List<Port> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(p => new PortId { Guid = p.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<PortDiffUpdate>();

        foreach (var afterPort in after)
        {
            var beforePort = before.FirstOrDefault(b => b.Guid == afterPort.Guid);
            if (beforePort != null)
            {
                var portDiff = GetPortDiff(beforePort, afterPort);
                if (portDiff != null)
                    updated.Add(new PortDiffUpdate { Port = new PortId { Guid = afterPort.Guid }, Diff = portDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new PortsDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static PortDiff? GetPortDiff(Port before, Port after)
    {
        var diff = new PortDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (NormalizeString(before.Icon) != NormalizeString(after.Icon)) { diff.Icon = after.Icon; hasChanges = true; }

        var beforePorts = (before.CompatiblePorts ?? new List<PortId>()).Select(p => p.Guid).OrderBy(g => g).ToList();
        var afterPorts = (after.CompatiblePorts ?? new List<PortId>()).Select(p => p.Guid).OrderBy(g => g).ToList();
        if (!beforePorts.SequenceEqual(afterPorts)) { diff.CompatiblePorts = after.CompatiblePorts; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static FilesDiff? GetFilesDiff(List<File> before, List<File> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(f => new FileId { Guid = f.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<FileDiffUpdate>();

        foreach (var afterFile in after)
        {
            var beforeFile = before.FirstOrDefault(b => b.Guid == afterFile.Guid);
            if (beforeFile != null)
            {
                var fileDiff = GetFileDiff(beforeFile, afterFile);
                if (fileDiff != null)
                    updated.Add(new FileDiffUpdate { File = new FileId { Guid = afterFile.Guid }, Diff = fileDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new FilesDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static FileDiff? GetFileDiff(File before, File after)
    {
        var diff = new FileDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (NormalizeString(before.Remote) != NormalizeString(after.Remote)) { diff.Remote = after.Remote; hasChanges = true; }
        if (before.Folder?.Guid != after.Folder?.Guid) { diff.Folder = after.Folder; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static FoldersDiff? GetFoldersDiff(List<Folder> before, List<Folder> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(f => new FolderId { Guid = f.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<FolderDiffUpdate>();

        foreach (var afterFolder in after)
        {
            var beforeFolder = before.FirstOrDefault(b => b.Guid == afterFolder.Guid);
            if (beforeFolder != null)
            {
                var folderDiff = GetFolderDiff(beforeFolder, afterFolder);
                if (folderDiff != null)
                    updated.Add(new FolderDiffUpdate { Folder = new FolderId { Guid = afterFolder.Guid }, Diff = folderDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new FoldersDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static FolderDiff? GetFolderDiff(Folder before, Folder after)
    {
        var diff = new FolderDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (before.Parent?.Guid != after.Parent?.Guid) { diff.Parent = after.Parent; hasChanges = true; }
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (before.CreatedAt != after.CreatedAt) { diff.CreatedAt = after.CreatedAt; hasChanges = true; }
        if (NormalizeString(before.CreatedBy) != NormalizeString(after.CreatedBy)) { diff.CreatedBy = after.CreatedBy; hasChanges = true; }
        if (before.UpdatedAt != after.UpdatedAt) { diff.UpdatedAt = after.UpdatedAt; hasChanges = true; }
        if (NormalizeString(before.UpdatedBy) != NormalizeString(after.UpdatedBy)) { diff.UpdatedBy = after.UpdatedBy; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static AttributesDiff? GetAttributesDiff(List<Attribute> before, List<Attribute> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(a => new AttributeId { Guid = a.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<AttributeDiffUpdate>();

        foreach (var afterAttr in after)
        {
            var beforeAttr = before.FirstOrDefault(b => b.Guid == afterAttr.Guid);
            if (beforeAttr != null)
            {
                var attrDiff = new AttributeDiff();
                bool attrHasChanges = false;
                if (beforeAttr.Key != afterAttr.Key) { attrDiff.Key = afterAttr.Key; attrHasChanges = true; }
                if (NormalizeString(beforeAttr.Value) != NormalizeString(afterAttr.Value)) { attrDiff.Value = afterAttr.Value; attrHasChanges = true; }
                if (NormalizeString(beforeAttr.Definition) != NormalizeString(afterAttr.Definition)) { attrDiff.Definition = afterAttr.Definition; attrHasChanges = true; }
                if (attrHasChanges)
                    updated.Add(new AttributeDiffUpdate { Attribute = new AttributeId { Guid = afterAttr.Guid }, Diff = attrDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new AttributesDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static AuthorsDiff? GetAuthorsDiff(List<Author> before, List<Author> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(a => new AuthorId { Guid = a.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<AuthorDiffUpdate>();

        foreach (var afterAuthor in after)
        {
            var beforeAuthor = before.FirstOrDefault(b => b.Guid == afterAuthor.Guid);
            if (beforeAuthor != null)
            {
                var authorDiff = GetAuthorDiff(beforeAuthor, afterAuthor);
                if (authorDiff != null)
                    updated.Add(new AuthorDiffUpdate { Author = new AuthorId { Guid = afterAuthor.Guid }, Diff = authorDiff });
            }
        }

        if (removed.Count == 0 && added.Count == 0 && updated.Count == 0) return null;
        return new AuthorsDiff { Removed = removed, Added = added, Updated = updated };
    }

    private static AuthorDiff? GetAuthorDiff(Author before, Author after)
    {
        var diff = new AuthorDiff();
        bool hasChanges = false;

        if (before.Name != after.Name) { diff.Name = after.Name; hasChanges = true; }
        if (NormalizeString(before.Email) != NormalizeString(after.Email)) { diff.Email = after.Email; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    public static KitDiff InverseKitDiff(Kit original, KitDiff appliedDiff)
    {
        var inverse = new KitDiff();

        if (appliedDiff.Name != null) inverse.Name = original.Name;
        if (appliedDiff.Version != null) inverse.Version = original.Version;
        if (appliedDiff.Description != null) inverse.Description = original.Description;
        if (appliedDiff.Icon != null) inverse.Icon = original.Icon;
        if (appliedDiff.Image != null) inverse.Image = original.Image;
        if (appliedDiff.Preview != null) inverse.Preview = original.Preview;
        if (appliedDiff.Remote != null) inverse.Remote = original.Remote;
        if (appliedDiff.Homepage != null) inverse.Homepage = original.Homepage;
        if (appliedDiff.License != null) inverse.License = original.License;

        if (appliedDiff.Types != null)
            inverse.Types = InverseTypesDiff(original.Types ?? new List<Type>(), appliedDiff.Types);

        if (appliedDiff.Designs != null)
            inverse.Designs = InverseDesignsDiff(original.Designs ?? new List<Design>(), appliedDiff.Designs);

        if (appliedDiff.Tags != null)
            inverse.Tags = InverseTagsDiff(original.Tags ?? new List<Tag>(), appliedDiff.Tags);

        if (appliedDiff.Files != null)
            inverse.Files = InverseFilesDiff(original.Files ?? new List<File>(), appliedDiff.Files);

        if (appliedDiff.Folders != null)
            inverse.Folders = InverseFoldersDiff(original.Folders ?? new List<Folder>(), appliedDiff.Folders);

        if (appliedDiff.Ports != null)
            inverse.Ports = InversePortsDiff(original.Ports ?? new List<Port>(), appliedDiff.Ports);

        if (appliedDiff.Authors != null)
            inverse.Authors = InverseAuthorsDiff(original.Authors ?? new List<Author>(), appliedDiff.Authors);

        if (appliedDiff.Concepts != null)
            inverse.Concepts = InverseConceptsDiff(original.Concepts ?? new List<Concept>(), appliedDiff.Concepts);

        if (appliedDiff.Attributes != null)
            inverse.Attributes = InverseAttributesDiff(original.Attributes ?? new List<Attribute>(), appliedDiff.Attributes);

        return inverse;
    }

    private static TypesDiff InverseTypesDiff(List<Type> original, TypesDiff appliedDiff)
    {
        var inverse = new TypesDiff
        {
            Removed = appliedDiff.Added?.Select(t => new TypeId { Guid = t.Guid }).ToList() ?? new List<TypeId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(t => t.Guid == id.Guid)).Where(t => t != null).Cast<Type>().ToList() ?? new List<Type>(),
            Updated = new List<TypeDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalType = original.FirstOrDefault(t => t.Guid == update.Type.Guid);
                if (originalType != null && update.Diff != null)
                {
                    var inverseDiff = new TypeDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalType.Name;
                    if (update.Diff.ShouldSerializeParent()) inverseDiff.Parent = originalType.Parent;
                    if (update.Diff.ShouldSerializeIsAbstract()) inverseDiff.IsAbstract = originalType.IsAbstract;
                    if (update.Diff.ShouldSerializeFolder()) inverseDiff.Folder = originalType.Folder;
                    if (update.Diff.ShouldSerializeStock()) inverseDiff.Stock = originalType.Stock;
                    if (update.Diff.ShouldSerializeVirtual()) inverseDiff.Virtual = originalType.Virtual;
                    if (update.Diff.ShouldSerializeUnit()) inverseDiff.Unit = originalType.Unit;
                    if (update.Diff.ShouldSerializeLocation()) inverseDiff.Location = originalType.Location;
                    if (update.Diff.Icon != null) inverseDiff.Icon = originalType.Icon;
                    if (update.Diff.Image != null) inverseDiff.Image = originalType.Image;
                    if (update.Diff.Description != null) inverseDiff.Description = originalType.Description;
                    if (update.Diff.ShouldSerializeAuthors()) inverseDiff.Authors = originalType.Authors?.Select(a => new AuthorId { Guid = a.Guid }).ToList();
                    if (update.Diff.ShouldSerializeConcepts()) inverseDiff.Concepts = originalType.Concepts?.Select(c => new ConceptId { Guid = c.Guid }).ToList();
                    if (update.Diff.Connectors != null) inverseDiff.Connectors = InverseConnectorsDiff(originalType.Connectors ?? new List<Connector>(), update.Diff.Connectors);
                    if (update.Diff.Models != null) inverseDiff.Models = InverseModelsDiff(originalType.Models ?? new List<Model>(), update.Diff.Models);
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = InverseAttributesDiff(originalType.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                    inverse.Updated.Add(new TypeDiffUpdate { Type = update.Type, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static ConnectorsDiff InverseConnectorsDiff(List<Connector> original, ConnectorsDiff appliedDiff)
    {
        var inverse = new ConnectorsDiff
        {
            Removed = appliedDiff.Added?.Select(c => new ConnectorId { Guid = c.Guid }).ToList() ?? new List<ConnectorId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(c => c.Guid == id.Guid)).Where(c => c != null).Cast<Connector>().ToList() ?? new List<Connector>(),
            Updated = new List<ConnectorDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalConnector = original.FirstOrDefault(c => c.Guid == update.Connector.Guid);
                if (originalConnector != null && update.Diff != null)
                {
                    var inverseDiff = new ConnectorDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalConnector.Name;
                    if (update.Diff.Description != null) inverseDiff.Description = originalConnector.Description;
                    if (update.Diff.ShouldSerializePort()) inverseDiff.Port = originalConnector.Port;
                    if (update.Diff.ShouldSerializeMandatory()) inverseDiff.Mandatory = originalConnector.Mandatory;
                    if (update.Diff.ShouldSerializeT()) inverseDiff.T = originalConnector.T;
                    if (update.Diff.ShouldSerializePoint())
                    {
                        var pd = update.Diff.Point;
                        inverseDiff.Point = new Point { X = -(pd?.X ?? 0) + 0.0f, Y = -(pd?.Y ?? 0) + 0.0f, Z = -(pd?.Z ?? 0) + 0.0f };
                    }
                    if (update.Diff.ShouldSerializeDirection())
                    {
                        var dd = update.Diff.Direction;
                        inverseDiff.Direction = new Vector { X = -(dd?.X ?? 0) + 0.0f, Y = -(dd?.Y ?? 0) + 0.0f, Z = -(dd?.Z ?? 0) + 0.0f };
                    }
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = InverseAttributesDiff(originalConnector.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                    inverse.Updated.Add(new ConnectorDiffUpdate { Connector = update.Connector, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static ModelsDiff InverseModelsDiff(List<Model> original, ModelsDiff appliedDiff)
    {
        var inverse = new ModelsDiff
        {
            Removed = appliedDiff.Added?.Select(m => new ModelId { Guid = m.Guid }).ToList() ?? new List<ModelId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(m => m.Guid == id.Guid)).Where(m => m != null).Cast<Model>().ToList() ?? new List<Model>(),
            Updated = new List<ModelDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalModel = original.FirstOrDefault(m => m.Guid == update.Model.Guid);
                if (originalModel != null && update.Diff != null)
                {
                    var inverseDiff = new ModelDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalModel.Name;
                    if (update.Diff.ShouldSerializeTags()) inverseDiff.Tags = originalModel.Tags;
                    if (update.Diff.ShouldSerializeFile()) inverseDiff.File = originalModel.File;
                    if (update.Diff.Description != null) inverseDiff.Description = originalModel.Description;
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = InverseAttributesDiff(originalModel.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                    inverse.Updated.Add(new ModelDiffUpdate { Model = update.Model, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static DesignsDiff InverseDesignsDiff(List<Design> original, DesignsDiff appliedDiff)
    {
        var inverse = new DesignsDiff
        {
            Removed = appliedDiff.Added?.Select(d => new DesignId { Guid = d.Guid }).ToList() ?? new List<DesignId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(d => d.Guid == id.Guid)).Where(d => d != null).Cast<Design>().ToList() ?? new List<Design>(),
            Updated = new List<DesignDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalDesign = original.FirstOrDefault(d => d.Guid == update.Design.Guid);
                if (originalDesign != null && update.Diff != null)
                {
                    var inverseDiff = new DesignDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalDesign.Name;
                    if (update.Diff.ShouldSerializeParent()) inverseDiff.Parent = originalDesign.Parent;
                    if (update.Diff.ShouldSerializeIsAbstract()) inverseDiff.IsAbstract = originalDesign.IsAbstract;
                    if (update.Diff.ShouldSerializeFolder()) inverseDiff.Folder = originalDesign.Folder;
                    if (update.Diff.ShouldSerializeCanScale()) inverseDiff.CanScale = originalDesign.CanScale;
                    if (update.Diff.ShouldSerializeCanMirror()) inverseDiff.CanMirror = originalDesign.CanMirror;
                    if (update.Diff.ShouldSerializeUnit()) inverseDiff.Unit = originalDesign.Unit;
                    if (update.Diff.ShouldSerializeActiveLayer()) inverseDiff.ActiveLayer = originalDesign.ActiveLayer;
                    if (update.Diff.ShouldSerializeLocation()) inverseDiff.Location = originalDesign.Location;
                    if (update.Diff.Icon != null) inverseDiff.Icon = originalDesign.Icon;
                    if (update.Diff.Image != null) inverseDiff.Image = originalDesign.Image;
                    if (update.Diff.Description != null) inverseDiff.Description = originalDesign.Description;
                    if (update.Diff.ShouldSerializeAuthors()) inverseDiff.Authors = originalDesign.Authors?.Select(a => new AuthorId { Guid = a.Guid }).ToList();
                    if (update.Diff.ShouldSerializeConcepts()) inverseDiff.Concepts = originalDesign.Concepts?.Select(c => new ConceptId { Guid = c.Guid }).ToList();
                    if (update.Diff.Pieces != null) inverseDiff.Pieces = InversePiecesDiff(originalDesign.Pieces ?? new List<Piece>(), update.Diff.Pieces);
                    if (update.Diff.Connections != null) inverseDiff.Connections = InverseConnectionsDiff(originalDesign.Connections ?? new List<Connection>(), update.Diff.Connections);
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = InverseAttributesDiff(originalDesign.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                    inverse.Updated.Add(new DesignDiffUpdate { Design = update.Design, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static PiecesDiff InversePiecesDiff(List<Piece> original, PiecesDiff appliedDiff)
    {
        var inverse = new PiecesDiff
        {
            Removed = appliedDiff.Added?.Select(p => new PieceId { Guid = p.Guid }).ToList() ?? new List<PieceId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(p => p.Guid == id.Guid)).Where(p => p != null).Cast<Piece>().ToList() ?? new List<Piece>(),
            Updated = new List<PieceDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalPiece = original.FirstOrDefault(p => p.Guid == update.Piece.Guid);
                if (originalPiece != null && update.Diff != null)
                {
                    var inverseDiff = new PieceDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalPiece.Name;
                    if (update.Diff.ShouldSerializeType()) inverseDiff.Type = originalPiece.Type;
                    if (update.Diff.ShouldSerializeDesign()) inverseDiff.Design = originalPiece.Design;
                    if (update.Diff.ShouldSerializePlane()) inverseDiff.Plane = originalPiece.Plane;
                    if (update.Diff.ShouldSerializeCenter()) inverseDiff.Center = originalPiece.Center;
                    if (update.Diff.ShouldSerializeScale()) inverseDiff.Scale = originalPiece.Scale;
                    if (update.Diff.ShouldSerializeMirrorPlane()) inverseDiff.MirrorPlane = originalPiece.MirrorPlane;
                    if (update.Diff.ShouldSerializeIsHidden()) inverseDiff.IsHidden = originalPiece.IsHidden;
                    if (update.Diff.ShouldSerializeIsLocked()) inverseDiff.IsLocked = originalPiece.IsLocked;
                    if (update.Diff.ShouldSerializeColor()) inverseDiff.Color = originalPiece.Color;
                    if (update.Diff.Description != null) inverseDiff.Description = originalPiece.Description;
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = InverseAttributesDiff(originalPiece.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                    inverse.Updated.Add(new PieceDiffUpdate { Piece = update.Piece, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static ConnectionsDiff InverseConnectionsDiff(List<Connection> original, ConnectionsDiff appliedDiff)
    {
        var inverse = new ConnectionsDiff
        {
            Removed = appliedDiff.Added?.Select(c => new ConnectionId { Guid = c.Guid }).ToList() ?? new List<ConnectionId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(c => c.Guid == id.Guid)).Where(c => c != null).Cast<Connection>().ToList() ?? new List<Connection>(),
            Updated = new List<ConnectionDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalConnection = original.FirstOrDefault(c => c.Guid == update.Connection.Guid);
                if (originalConnection != null && update.Diff != null)
                {
                    var inverseDiff = new ConnectionDiff();
                    if (update.Diff.ShouldSerializeConnected())
                    {
                        var invSide = new SideDiff();
                        if (update.Diff.Connected?.ShouldSerializePiece() == true) invSide.Piece = originalConnection.Connected?.Piece;
                        if (update.Diff.Connected?.ShouldSerializeDesignPiece() == true) invSide.DesignPiece = originalConnection.Connected?.DesignPiece;
                        if (update.Diff.Connected?.ShouldSerializeConnector() == true) invSide.Connector = originalConnection.Connected?.Connector;
                        inverseDiff.Connected = invSide;
                    }
                    if (update.Diff.ShouldSerializeConnecting())
                    {
                        var invSide = new SideDiff();
                        if (update.Diff.Connecting?.ShouldSerializePiece() == true) invSide.Piece = originalConnection.Connecting?.Piece;
                        if (update.Diff.Connecting?.ShouldSerializeDesignPiece() == true) invSide.DesignPiece = originalConnection.Connecting?.DesignPiece;
                        if (update.Diff.Connecting?.ShouldSerializeConnector() == true) invSide.Connector = originalConnection.Connecting?.Connector;
                        inverseDiff.Connecting = invSide;
                    }
                    if (update.Diff.ShouldSerializeGap()) inverseDiff.Gap = originalConnection.Gap != null && update.Diff.Gap != null ? -update.Diff.Gap + 0.0f : originalConnection.Gap;
                    if (update.Diff.ShouldSerializeShift()) inverseDiff.Shift = originalConnection.Shift != null && update.Diff.Shift != null ? -update.Diff.Shift + 0.0f : originalConnection.Shift;
                    if (update.Diff.ShouldSerializeRise()) inverseDiff.Rise = originalConnection.Rise != null && update.Diff.Rise != null ? -update.Diff.Rise + 0.0f : originalConnection.Rise;
                    if (update.Diff.ShouldSerializeRotation()) inverseDiff.Rotation = originalConnection.Rotation != null && update.Diff.Rotation != null ? -update.Diff.Rotation + 0.0f : originalConnection.Rotation;
                    if (update.Diff.ShouldSerializeTurn()) inverseDiff.Turn = originalConnection.Turn != null && update.Diff.Turn != null ? -update.Diff.Turn + 0.0f : originalConnection.Turn;
                    if (update.Diff.ShouldSerializeTilt()) inverseDiff.Tilt = originalConnection.Tilt != null && update.Diff.Tilt != null ? -update.Diff.Tilt + 0.0f : originalConnection.Tilt;
                    if (update.Diff.ShouldSerializeU()) inverseDiff.U = originalConnection.U != null && update.Diff.U != null ? -update.Diff.U + 0.0f : originalConnection.U;
                    if (update.Diff.ShouldSerializeV()) inverseDiff.V = originalConnection.V != null && update.Diff.V != null ? -update.Diff.V + 0.0f : originalConnection.V;
                    if (update.Diff.Description != null) inverseDiff.Description = originalConnection.Description;
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = InverseAttributesDiff(originalConnection.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                    inverse.Updated.Add(new ConnectionDiffUpdate { Connection = update.Connection, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static TagsDiff InverseTagsDiff(List<Tag> original, TagsDiff appliedDiff)
    {
        var inverse = new TagsDiff
        {
            Removed = appliedDiff.Added?.Select(t => new TagId { Guid = t.Guid }).ToList() ?? new List<TagId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(t => t.Guid == id.Guid)).Where(t => t != null).Cast<Tag>().ToList() ?? new List<Tag>(),
            Updated = new List<TagDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalTag = original.FirstOrDefault(t => t.Guid == update.Tag.Guid);
                if (originalTag != null && update.Diff != null)
                {
                    var inverseDiff = new TagDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalTag.Name;
                    if (update.Diff.Description != null) inverseDiff.Description = originalTag.Description;
                    if (update.Diff.Icon != null) inverseDiff.Icon = originalTag.Icon;
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = InverseAttributesDiff(originalTag.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                    inverse.Updated.Add(new TagDiffUpdate { Tag = update.Tag, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static FilesDiff InverseFilesDiff(List<File> original, FilesDiff appliedDiff)
    {
        var inverse = new FilesDiff
        {
            Removed = appliedDiff.Added?.Select(f => new FileId { Guid = f.Guid }).ToList() ?? new List<FileId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(f => f.Guid == id.Guid)).Where(f => f != null).Cast<File>().ToList() ?? new List<File>(),
            Updated = new List<FileDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalFile = original.FirstOrDefault(f => f.Guid == update.File.Guid);
                if (originalFile != null && update.Diff != null)
                {
                    var inverseDiff = new FileDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalFile.Name;
                    if (update.Diff.Remote != null) inverseDiff.Remote = originalFile.Remote;
                    if (update.Diff.Size != null) inverseDiff.Size = originalFile.Size;
                    if (update.Diff.Hash != null) inverseDiff.Hash = originalFile.Hash;
                    if (update.Diff.CreatedAt != null) inverseDiff.CreatedAt = originalFile.CreatedAt;
                    if (update.Diff.CreatedBy != null) inverseDiff.CreatedBy = originalFile.CreatedBy;
                    if (update.Diff.UpdatedAt != null) inverseDiff.UpdatedAt = originalFile.UpdatedAt;
                    if (update.Diff.UpdatedBy != null) inverseDiff.UpdatedBy = originalFile.UpdatedBy;
                    if (update.Diff.Folder != null) inverseDiff.Folder = originalFile.Folder;
                    inverse.Updated.Add(new FileDiffUpdate { File = update.File, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static FoldersDiff InverseFoldersDiff(List<Folder> original, FoldersDiff appliedDiff)
    {
        var inverse = new FoldersDiff
        {
            Removed = appliedDiff.Added?.Select(f => new FolderId { Guid = f.Guid }).ToList() ?? new List<FolderId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(f => f.Guid == id.Guid)).Where(f => f != null).Cast<Folder>().ToList() ?? new List<Folder>(),
            Updated = new List<FolderDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalFolder = original.FirstOrDefault(f => f.Guid == update.Folder.Guid);
                if (originalFolder != null && update.Diff != null)
                {
                    var inverseDiff = new FolderDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalFolder.Name;
                    if (update.Diff.Parent != null) inverseDiff.Parent = originalFolder.Parent;
                    if (update.Diff.Description != null) inverseDiff.Description = originalFolder.Description;
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = originalFolder.Attributes;
                    if (update.Diff.CreatedAt != null) inverseDiff.CreatedAt = originalFolder.CreatedAt;
                    if (update.Diff.CreatedBy != null) inverseDiff.CreatedBy = originalFolder.CreatedBy;
                    if (update.Diff.UpdatedAt != null) inverseDiff.UpdatedAt = originalFolder.UpdatedAt;
                    if (update.Diff.UpdatedBy != null) inverseDiff.UpdatedBy = originalFolder.UpdatedBy;
                    inverse.Updated.Add(new FolderDiffUpdate { Folder = update.Folder, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static PortsDiff InversePortsDiff(List<Port> original, PortsDiff appliedDiff)
    {
        var inverse = new PortsDiff
        {
            Removed = appliedDiff.Added?.Select(p => new PortId { Guid = p.Guid }).ToList() ?? new List<PortId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(p => p.Guid == id.Guid)).Where(p => p != null).Cast<Port>().ToList() ?? new List<Port>(),
            Updated = new List<PortDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalPort = original.FirstOrDefault(p => p.Guid == update.Port.Guid);
                if (originalPort != null && update.Diff != null)
                {
                    var inverseDiff = new PortDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalPort.Name;
                    if (update.Diff.Description != null) inverseDiff.Description = originalPort.Description;
                    if (update.Diff.Icon != null) inverseDiff.Icon = originalPort.Icon;
                    if (update.Diff.CompatiblePorts != null) inverseDiff.CompatiblePorts = originalPort.CompatiblePorts;
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = originalPort.Attributes;
                    inverse.Updated.Add(new PortDiffUpdate { Port = update.Port, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static AuthorsDiff InverseAuthorsDiff(List<Author> original, AuthorsDiff appliedDiff)
    {
        var inverse = new AuthorsDiff
        {
            Removed = appliedDiff.Added?.Select(a => new AuthorId { Guid = a.Guid }).ToList() ?? new List<AuthorId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(a => a.Guid == id.Guid)).Where(a => a != null).Cast<Author>().ToList() ?? new List<Author>(),
            Updated = new List<AuthorDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalAuthor = original.FirstOrDefault(a => a.Guid == update.Author.Guid);
                if (originalAuthor != null && update.Diff != null)
                {
                    var inverseDiff = new AuthorDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalAuthor.Name;
                    if (update.Diff.Email != null) inverseDiff.Email = originalAuthor.Email;
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = originalAuthor.Attributes;
                    inverse.Updated.Add(new AuthorDiffUpdate { Author = update.Author, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static ConceptsDiff InverseConceptsDiff(List<Concept> original, ConceptsDiff appliedDiff)
    {
        var inverse = new ConceptsDiff
        {
            Removed = appliedDiff.Added?.Select(c => new ConceptId { Guid = c.Guid }).ToList() ?? new List<ConceptId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(c => c.Guid == id.Guid)).Where(c => c != null).Cast<Concept>().ToList() ?? new List<Concept>(),
            Updated = new List<ConceptDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalConcept = original.FirstOrDefault(c => c.Guid == update.Concept.Guid);
                if (originalConcept != null && update.Diff != null)
                {
                    var inverseDiff = new ConceptDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalConcept.Name;
                    if (update.Diff.Description != null) inverseDiff.Description = originalConcept.Description;
                    if (update.Diff.Icon != null) inverseDiff.Icon = originalConcept.Icon;
                    if (update.Diff.Attributes != null) inverseDiff.Attributes = InverseAttributesDiff(originalConcept.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                    inverse.Updated.Add(new ConceptDiffUpdate { Concept = update.Concept, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    private static AttributesDiff InverseAttributesDiff(List<Attribute> original, AttributesDiff appliedDiff)
    {
        var inverse = new AttributesDiff
        {
            Removed = appliedDiff.Added?.Select(a => new AttributeId { Guid = a.Guid }).ToList() ?? new List<AttributeId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(a => a.Guid == id.Guid)).Where(a => a != null).Cast<Attribute>().ToList() ?? new List<Attribute>(),
            Updated = new List<AttributeDiffUpdate>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalAttr = original.FirstOrDefault(a => a.Guid == update.Attribute.Guid);
                if (originalAttr != null && update.Diff != null)
                {
                    var inverseDiff = new AttributeDiff
                    {
                        Key = update.Diff.Key != null ? originalAttr.Key : null,
                        Value = update.Diff.Value != null ? originalAttr.Value : null,
                        Definition = update.Diff.Definition != null ? originalAttr.Definition : null
                    };
                    inverse.Updated.Add(new AttributeDiffUpdate { Attribute = update.Attribute, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    public static KitDiffValidationResult ValidateKitDiff(Kit kit, KitDiff diff, bool heal = false)
    {
        var ctx = new KitDiffValidationContext { Heal = heal };
        var json = KitDiffValidationJson;
        var kitObj = JObject.Parse(JsonConvert.SerializeObject(kit, json));
        var diffObj = JObject.Parse(JsonConvert.SerializeObject(diff, json));
        JObject? outDiff = heal ? (JObject)diffObj.DeepClone() : null;
        var refs = RefSets.FromKit(kitObj);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "types", "type", "types", null, refs);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "designs", "design", "designs",
            (c, km, item, dm, p, r) => ValidateDesignDiffNested(c, km, item, dm, p, r), refs);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "tags", "tag", "tags", null, refs);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "concepts", "concept", "concepts", null, refs);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "ports", "port", "ports", null, refs);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "qualities", "quality", "qualities", null, refs);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "files", "file", "files", null, refs);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "folders", "folder", "folders", null, refs);
        RunTopLevelGuidCollection(ctx, kitObj, diffObj, outDiff, heal, "authors", "author", "authors", null, refs);
        if (diffObj["attributes"] is JObject attrPart)
        {
            var baseAttrs = kitObj["attributes"] as JArray;
            _ = ValidateGuidCollectionDiff(ctx, "kit.attributes", "attribute", baseAttrs, attrPart, null);
        }
        var ok = ctx.Errors.Count == 0;
        KitDiff? diffOut = null;
        if (heal && outDiff != null)
        {
            if (outDiff.Properties().Any())
                diffOut = JsonConvert.DeserializeObject<KitDiff>(outDiff.ToString(Formatting.None), json);
            else
                diffOut = new KitDiff();
        }
        return new KitDiffValidationResult
        {
            Ok = ok,
            Errors = ctx.Errors,
            Warnings = ctx.Warnings,
            Diff = diffOut
        };
    }

    private static readonly JsonSerializerSettings KitDiffValidationJson = new()
    {
        ContractResolver = new CamelCasePropertyNamesContractResolver(),
        NullValueHandling = NullValueHandling.Include
    };

    private sealed class KitDiffValidationContext
    {
        public bool Heal { get; init; }
        public List<KitDiffValidationNote> Errors { get; } = new();
        public List<KitDiffValidationNote> Warnings { get; } = new();
        public void Push(string kind, string code, string message)
        {
            var n = new KitDiffValidationNote { Code = string.IsNullOrEmpty(code) ? null : code, Message = message };
            if (kind == "errors") Errors.Add(n); else Warnings.Add(n);
        }
    }

    private readonly struct RefSets
    {
        public HashSet<string> TypeGuids { get; }
        public HashSet<string> DesignGuids { get; }
        public HashSet<string> AuthorGuids { get; }

        private RefSets(HashSet<string> t, HashSet<string> d, HashSet<string> a)
        {
            TypeGuids = t; DesignGuids = d; AuthorGuids = a;
        }

        public static RefSets FromKit(JObject kitObj) => new(
            GuidSetFromEntities(kitObj["types"]),
            GuidSetFromEntities(kitObj["designs"]),
            GuidSetFromEntities(kitObj["authors"]));
    }

    private static HashSet<string> GuidSetFromEntities(JToken? v)
    {
        var s = new HashSet<string>();
        if (v is not JArray arr) return s;
        foreach (var x in arr)
            if (x is JObject o && o["guid"]?.Value<string>() is { } g && !string.IsNullOrEmpty(g))
                s.Add(g);
        return s;
    }

    private static List<JObject> ToJObjectList(JArray? arr)
    {
        var list = new List<JObject>();
        if (arr == null) return list;
        foreach (var x in arr)
            if (x is JObject o) list.Add(o);
        return list;
    }

    private static bool KitDiffDeepEqual(JToken a, JToken b) => JToken.DeepEquals(a, b);

    private delegate void DesignNestedHandler(KitDiffValidationContext ctx, JObject kitMap, JObject designItem, JObject? diffMap, string path, RefSets refs);

    private static void RunTopLevelGuidCollection(KitDiffValidationContext ctx, JObject kitObj, JObject diffObj, JObject? outDiff, bool heal,
        string key, string idKey, string kitArrayKey, DesignNestedHandler? onDesign, RefSets refs)
    {
        if (diffObj[key] is not JObject part) return;
        var baseArr = kitObj[kitArrayKey] as JArray;
        var fixedObj = ValidateGuidCollectionDiff(ctx, key, idKey, baseArr, part,
            onDesign != null ? (c, item, dm, p) => onDesign(c, kitObj, item, dm, p, refs) : null);
        if (!heal || outDiff == null) return;
        if (fixedObj != null && fixedObj.Properties().Any()) outDiff[key] = fixedObj;
        else outDiff.Remove(key);
    }

    private static JArray? FilterUpdatesByGuid(JArray updates, string idKey, string gid)
    {
        var na = new JArray();
        foreach (var u in updates)
        {
            if (u is not JObject um) { na.Add(u); continue; }
            if (um[idKey] is JObject idObj && idObj["guid"]?.Value<string>() == gid) continue;
            na.Add(u);
        }
        return na;
    }

    private static JObject? ValidateGuidCollectionDiff(KitDiffValidationContext ctx, string path, string idKey, JArray? baseEntities, JObject? raw,
        Action<KitDiffValidationContext, JObject, JObject?, string>? onUpdated)
    {
        if (raw == null) return null;
        var baseBy = new Dictionary<string, JObject>();
        foreach (var ent in ToJObjectList(baseEntities))
            if (ent["guid"]?.Value<string>() is { } bg && !string.IsNullOrEmpty(bg))
                baseBy[bg] = ent;
        var removedSet = new HashSet<string>();
        if (raw["removed"] is JArray remArr)
            foreach (var r in remArr)
                if (r is JObject rm && rm["guid"]?.Value<string>() is { } rg)
                    removedSet.Add(rg);
        var afterRemove = new HashSet<string>();
        foreach (var g in baseBy.Keys)
            if (!removedSet.Contains(g)) afterRemove.Add(g);
        JArray? hRem = null, hUpd = null, hAdd = null;
        if (ctx.Heal)
        {
            if (raw["removed"] is JArray r0) hRem = (JArray)r0.DeepClone();
            if (raw["updated"] is JArray u0) hUpd = (JArray)u0.DeepClone();
            if (raw["added"] is JArray a0) hAdd = (JArray)a0.DeepClone();
        }
        if (raw["removed"] is JArray removedTok)
            foreach (var r in removedTok)
                if (r is JObject rm)
                {
                    var rg = rm["guid"]?.Value<string>() ?? "";
                    if (!baseBy.ContainsKey(rg))
                    {
                        ctx.Push("warnings", "kitdiff.remove.missing-target", $"{path}: remove references missing {idKey} {rg}");
                        if (hRem != null)
                        {
                            var nr = new JArray();
                            foreach (var x in hRem)
                                if (x is JObject xm && xm["guid"]?.Value<string>() == rg) continue;
                                else nr.Add(x);
                            hRem = nr;
                        }
                    }
                }
        var addBy = new Dictionary<string, JObject>();
        if (raw["added"] is JArray addArr)
            foreach (var a in addArr)
                if (a is JObject am && am["guid"]?.Value<string>() is { } ag)
                    addBy[ag] = am;
        if (raw["removed"] is JArray removedTok2)
            foreach (var r in removedTok2)
                if (r is JObject rm)
                {
                    var rg = rm["guid"]?.Value<string>() ?? "";
                    if (baseBy.TryGetValue(rg, out var orig) && addBy.TryGetValue(rg, out var add) && KitDiffDeepEqual(orig, add))
                    {
                        ctx.Push("warnings", "kitdiff.cycle.noop-restore", $"{path}: removed and re-added {idKey} {rg} are deeply equal (no effective change)");
                        if (ctx.Heal)
                        {
                            if (hRem != null)
                            {
                                var nr = new JArray();
                                foreach (var x in hRem)
                                    if (x is JObject xm && xm["guid"]?.Value<string>() == rg) continue;
                                    else nr.Add(x);
                                hRem = nr;
                            }
                            if (hAdd != null)
                            {
                                var na = new JArray();
                                foreach (var x in hAdd)
                                    if (x is JObject xm && xm["guid"]?.Value<string>() == rg) continue;
                                    else na.Add(x);
                                hAdd = na;
                            }
                        }
                    }
                }
        var seenAdd = new HashSet<string>();
        if (raw["added"] is JArray addedTok)
            foreach (var a in addedTok)
                if (a is JObject am)
                {
                    var ag = am["guid"]?.Value<string>() ?? "";
                    if (seenAdd.Contains(ag))
                    {
                        ctx.Push("errors", "kitdiff.add.duplicate-in-diff", $"{path}: duplicate added {idKey} guid {ag}");
                        if (hAdd != null)
                        {
                            var first = true;
                            var na = new JArray();
                            foreach (var x in hAdd)
                            {
                                if (x is JObject xm && xm["guid"]?.Value<string>() == ag)
                                {
                                    if (first) { na.Add(x); first = false; }
                                    continue;
                                }
                                na.Add(x);
                            }
                            hAdd = na;
                        }
                    }
                    seenAdd.Add(ag);
                    if (afterRemove.Contains(ag))
                    {
                        ctx.Push("errors", "kitdiff.add.duplicate-guid", $"{path}: cannot add {idKey} {ag} that still exists after removes");
                        if (hAdd != null)
                        {
                            var na = new JArray();
                            foreach (var x in hAdd)
                                if (x is JObject xm && xm["guid"]?.Value<string>() == ag) continue;
                                else na.Add(x);
                            hAdd = na;
                        }
                    }
                }
        if (raw["updated"] is JArray updTok)
            foreach (var u in updTok)
                if (u is JObject um && um[idKey] is JObject idObj)
                {
                    var gid = idObj["guid"]?.Value<string>() ?? "";
                    var p = $"{path}.{idKey}[{gid}]";
                    if (string.IsNullOrEmpty(gid))
                    {
                        ctx.Push("errors", "kitdiff.update.bad-id", $"{p}: missing {idKey} id");
                        if (hUpd != null) hUpd = FilterUpdatesByGuid(hUpd, idKey, gid);
                        continue;
                    }
                    if (!afterRemove.Contains(gid))
                    {
                        ctx.Push("errors", "kitdiff.update.missing-target", $"{p}: update targets {idKey} not present after removes");
                        if (hUpd != null) hUpd = FilterUpdatesByGuid(hUpd, idKey, gid);
                        continue;
                    }
                    if (!baseBy.TryGetValue(gid, out var item))
                    {
                        ctx.Push("errors", "kitdiff.update.missing-base", $"{p}: {idKey} not found in base kit");
                        if (hUpd != null) hUpd = FilterUpdatesByGuid(hUpd, idKey, gid);
                        continue;
                    }
                    var dm = um["diff"] as JObject;
                    onUpdated?.Invoke(ctx, item, dm, p);
                }
        if (!ctx.Heal) return null;
        var o = new JObject();
        if (hRem is { Count: > 0 }) o["removed"] = hRem;
        if (hUpd is { Count: > 0 }) o["updated"] = hUpd;
        if (hAdd is { Count: > 0 }) o["added"] = hAdd;
        return o.Properties().Any() ? o : null;
    }

    private static void ValidateDesignDiffNested(KitDiffValidationContext ctx, JObject kitMap, JObject design, JObject? diff, string path, RefSets refs)
    {
        if (diff == null) return;
        if (diff["parent"] is JObject pObj && pObj["guid"]?.Value<string>() is { } pg)
        {
            if (!string.IsNullOrEmpty(pg) && !refs.DesignGuids.Contains(pg))
                ctx.Push("errors", "kitdiff.ref.design-parent-missing", $"{path}: parent design {pg} not in kit");
            if (design["guid"]?.Value<string>() is { } dg && pg == dg)
                ctx.Push("errors", "kitdiff.ref.design-parent-self", $"{path}: design cannot be its own parent");
        }
        if (diff["authors"] != null)
        {
            var da = diff["authors"]!;
            if (da is JArray authArr)
                foreach (var a in authArr)
                    if (a is JObject am && am["guid"]?.Value<string>() is { } g && !string.IsNullOrEmpty(g) && !refs.AuthorGuids.Contains(g))
                        ctx.Push("errors", "kitdiff.ref.author-missing", $"{path}: author {g} not in kit");
            else if (da is JObject authObj)
            {
                var authBase = kitMap["authors"] as JArray;
                _ = ValidateGuidCollectionDiff(ctx, $"{path}.authors", "author", authBase, authObj, null);
            }
        }
        if (diff["pieces"] is JObject pd)
        {
            var piecesBase = design["pieces"] as JArray;
            _ = ValidateGuidCollectionDiff(ctx, $"{path}.pieces", "piece", piecesBase, pd, null);
            if (pd["added"] is JArray pAdd)
                foreach (var a in pAdd)
                    if (a is JObject am)
                    {
                        var tg = (am["type"] as JObject)?["guid"]?.Value<string>() ?? "";
                        if (!string.IsNullOrEmpty(tg) && !refs.TypeGuids.Contains(tg))
                            ctx.Push("errors", "kitdiff.ref.piece-type-missing", $"{path}.pieces.added: type {tg} not in kit");
                        var dsg = (am["design"] as JObject)?["guid"]?.Value<string>() ?? "";
                        if (!string.IsNullOrEmpty(dsg) && !refs.DesignGuids.Contains(dsg))
                            ctx.Push("errors", "kitdiff.ref.piece-design-missing", $"{path}.pieces.added: subdesign {dsg} not in kit");
                    }
        }
    }


    public static Kit ApplyKitDiff(Kit baseKit, KitDiff diff)
    {
        var result = Entity<Kit>.DeepClone(baseKit)!;

        if (diff.ShouldSerializeName()) result.Name = diff.Name ?? "";
        if (diff.ShouldSerializeVersion()) result.Version = diff.Version ?? "";
        if (diff.ShouldSerializeDescription()) result.Description = diff.Description;
        if (diff.ShouldSerializeIcon()) result.Icon = diff.Icon;
        if (diff.ShouldSerializeImage()) result.Image = diff.Image;
        if (diff.ShouldSerializePreview()) result.Preview = diff.Preview;
        if (diff.ShouldSerializeRemote()) result.Remote = diff.Remote;
        if (diff.ShouldSerializeHomepage()) result.Homepage = diff.Homepage;
        if (diff.ShouldSerializeLicense()) result.License = diff.License;
        if (diff.ShouldSerializeCreatedAt()) result.CreatedAt = diff.CreatedAt;
        if (diff.ShouldSerializeUpdatedAt()) result.UpdatedAt = diff.UpdatedAt;

        if (diff.Types != null)
            result.Types = ApplyTypesDiff(result.Types ?? new List<Type>(), diff.Types);

        if (diff.Designs != null)
            result.Designs = ApplyDesignsDiff(result.Designs ?? new List<Design>(), diff.Designs);

        if (diff.Tags != null)
            result.Tags = ApplyTagsDiff(result.Tags ?? new List<Tag>(), diff.Tags);

        if (diff.Folders != null)
            result.Folders = ApplyFoldersDiff(result.Folders ?? new List<Folder>(), diff.Folders);

        if (diff.Ports != null)
            result.Ports = ApplyPortsDiff(result.Ports ?? new List<Port>(), diff.Ports);

        if (diff.Concepts != null)
            result.Concepts = ApplyConceptsDiff(result.Concepts ?? new List<Concept>(), diff.Concepts);

        if (diff.Files != null)
            result.Files = ApplyFilesDiff(result.Files ?? new List<File>(), diff.Files);

        if (diff.Authors != null)
            result.Authors = ApplyAuthorsDiff(result.Authors ?? new List<Author>(), diff.Authors);

        if (diff.Attributes != null)
            result.Attributes = ApplyAttributesDiff(result.Attributes ?? new List<Attribute>(), diff.Attributes);

        return result;
    }

    private static List<Tag> ApplyTagsDiff(List<Tag> baseTags, TagsDiff diff)
    {
        var result = new List<Tag>(baseTags);

        if (diff.Removed != null)
            result.RemoveAll(t => diff.Removed.Any(r => r.Guid == t.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var tag = result.FirstOrDefault(t => t.Guid == update.Tag.Guid);
                if (tag != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) tag.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) tag.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) tag.Icon = update.Diff.Icon;
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Folder> ApplyFoldersDiff(List<Folder> baseFolders, FoldersDiff diff)
    {
        var result = new List<Folder>(baseFolders);

        if (diff.Removed != null)
            result.RemoveAll(f => diff.Removed.Any(r => r.Guid == f.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var folder = result.FirstOrDefault(f => f.Guid == update.Folder.Guid);
                if (folder != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) folder.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) folder.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeParent()) folder.Parent = update.Diff.Parent;
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Port> ApplyPortsDiff(List<Port> basePorts, PortsDiff diff)
    {
        var result = new List<Port>(basePorts);

        if (diff.Removed != null)
            result.RemoveAll(p => diff.Removed.Any(r => r.Guid == p.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var port = result.FirstOrDefault(p => p.Guid == update.Port.Guid);
                if (port != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) port.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) port.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) port.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeCompatiblePorts()) port.CompatiblePorts = update.Diff.CompatiblePorts;
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Concept> ApplyConceptsDiff(List<Concept> baseConcepts, ConceptsDiff diff)
    {
        var result = new List<Concept>(baseConcepts);

        if (diff.Removed != null)
            result.RemoveAll(c => diff.Removed.Any(r => r.Guid == c.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var concept = result.FirstOrDefault(c => c.Guid == update.Concept.Guid);
                if (concept != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) concept.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) concept.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) concept.Icon = update.Diff.Icon;
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<File> ApplyFilesDiff(List<File> baseFiles, FilesDiff diff)
    {
        var result = new List<File>(baseFiles);

        if (diff.Removed != null)
            result.RemoveAll(f => diff.Removed.Any(r => r.Guid == f.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var file = result.FirstOrDefault(f => f.Guid == update.File.Guid);
                if (file != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) file.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeRemote()) file.Remote = update.Diff.Remote;
                    if (update.Diff.ShouldSerializeFolder()) file.Folder = update.Diff.Folder;
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Author> ApplyAuthorsDiff(List<Author> baseAuthors, AuthorsDiff diff)
    {
        var result = new List<Author>(baseAuthors);

        if (diff.Removed != null)
            result.RemoveAll(a => diff.Removed.Any(r => r.Guid == a.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var author = result.FirstOrDefault(a => a.Guid == update.Author.Guid);
                if (author != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) author.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeEmail()) author.Email = update.Diff.Email ?? "";
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Attribute> ApplyAttributesDiff(List<Attribute> baseAttributes, AttributesDiff diff)
    {
        var result = new List<Attribute>(baseAttributes);

        if (diff.Removed != null)
            result.RemoveAll(a => diff.Removed.Any(r => r.Guid == a.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var attr = result.FirstOrDefault(a => a.Guid == update.Attribute.Guid);
                if (attr != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeValue()) attr.Value = update.Diff.Value;
                    if (update.Diff.ShouldSerializeDefinition()) attr.Definition = update.Diff.Definition;
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Type> ApplyTypesDiff(List<Type> baseTypes, TypesDiff diff)
    {
        var result = new List<Type>(baseTypes);

        if (diff.Removed != null)
            result.RemoveAll(t => diff.Removed.Any(r => r.Guid == t.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var type = result.FirstOrDefault(t => t.Guid == update.Type.Guid);
                if (type != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) type.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) type.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) type.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeImage()) type.Image = update.Diff.Image;
                    if (update.Diff.ShouldSerializeParent()) type.Parent = update.Diff.Parent;
                    if (update.Diff.ShouldSerializeIsAbstract()) type.IsAbstract = update.Diff.IsAbstract;
                    if (update.Diff.ShouldSerializeFolder()) type.Folder = update.Diff.Folder;
                    if (update.Diff.ShouldSerializeStock()) type.Stock = update.Diff.Stock ?? type.Stock;
                    if (update.Diff.ShouldSerializeVirtual()) type.Virtual = update.Diff.Virtual ?? type.Virtual;
                    if (update.Diff.ShouldSerializeUnit()) type.Unit = update.Diff.Unit;
                    if (update.Diff.ShouldSerializeLocation()) type.Location = update.Diff.Location;
                    if (update.Diff.ShouldSerializeAuthors()) type.Authors = update.Diff.Authors?.Select(a => new AuthorId { Guid = a.Guid }).ToList();
                    if (update.Diff.ShouldSerializeConcepts()) type.Concepts = update.Diff.Concepts?.Select(c => new ConceptId { Guid = c.Guid }).ToList();
                    if (update.Diff.Connectors != null)
                        type.Connectors = ApplyConnectorsDiff(type.Connectors ?? new List<Connector>(), update.Diff.Connectors);
                    if (update.Diff.Models != null)
                        type.Models = ApplyModelsDiff(type.Models ?? new List<Model>(), update.Diff.Models);
                    if (update.Diff.Attributes != null)
                        type.Attributes = ApplyAttributesDiff(type.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Connector> ApplyConnectorsDiff(List<Connector> baseConnectors, ConnectorsDiff diff)
    {
        var result = new List<Connector>(baseConnectors);

        if (diff.Removed != null)
            result.RemoveAll(c => diff.Removed.Any(r => r.Guid == c.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var connector = result.FirstOrDefault(c => c.Guid == update.Connector.Guid);
                if (connector != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) connector.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) connector.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializePort()) connector.Port = update.Diff.Port;
                    if (update.Diff.ShouldSerializeMandatory()) connector.Mandatory = update.Diff.Mandatory ?? connector.Mandatory;
                    if (update.Diff.ShouldSerializeT()) connector.T = update.Diff.T ?? connector.T;
                    if (update.Diff.ShouldSerializePoint())
                    {
                        var pd = update.Diff.Point;
                        var bp = connector.Point ?? new Point();
                        connector.Point = new Point { X = bp.X + (pd?.X ?? 0), Y = bp.Y + (pd?.Y ?? 0), Z = bp.Z + (pd?.Z ?? 0) };
                    }
                    if (update.Diff.ShouldSerializeDirection())
                    {
                        var dd = update.Diff.Direction;
                        var bd = connector.Direction ?? new Vector();
                        connector.Direction = new Vector { X = bd.X + (dd?.X ?? 0), Y = bd.Y + (dd?.Y ?? 0), Z = bd.Z + (dd?.Z ?? 0) };
                    }
                    if (update.Diff.Attributes != null)
                        connector.Attributes = ApplyAttributesDiff(connector.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Model> ApplyModelsDiff(List<Model> baseModels, ModelsDiff diff)
    {
        var result = new List<Model>(baseModels);

        if (diff.Removed != null)
            result.RemoveAll(m => diff.Removed.Any(r => r.Guid == m.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var model = result.FirstOrDefault(m => m.Guid == update.Model.Guid);
                if (model != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) model.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) model.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeFile()) model.File = update.Diff.File;
                    if (update.Diff.ShouldSerializeTags()) model.Tags = update.Diff.Tags;
                    if (update.Diff.Attributes != null)
                        model.Attributes = ApplyAttributesDiff(model.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Design> ApplyDesignsDiff(List<Design> baseDesigns, DesignsDiff diff)
    {
        var result = new List<Design>(baseDesigns);

        if (diff.Removed != null)
            result.RemoveAll(d => diff.Removed.Any(r => r.Guid == d.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var design = result.FirstOrDefault(d => d.Guid == update.Design.Guid);
                if (design != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) design.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) design.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) design.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeImage()) design.Image = update.Diff.Image;
                    if (update.Diff.ShouldSerializeParent()) design.Parent = update.Diff.Parent;
                    if (update.Diff.ShouldSerializeIsAbstract()) design.IsAbstract = update.Diff.IsAbstract;
                    if (update.Diff.ShouldSerializeFolder()) design.Folder = update.Diff.Folder;
                    if (update.Diff.ShouldSerializeCanScale()) design.CanScale = update.Diff.CanScale;
                    if (update.Diff.ShouldSerializeCanMirror()) design.CanMirror = update.Diff.CanMirror;
                    if (update.Diff.ShouldSerializeUnit()) design.Unit = update.Diff.Unit;
                    if (update.Diff.ShouldSerializeActiveLayer()) design.ActiveLayer = update.Diff.ActiveLayer;
                    if (update.Diff.ShouldSerializeLocation()) design.Location = update.Diff.Location;
                    if (update.Diff.ShouldSerializeAuthors()) design.Authors = update.Diff.Authors?.Select(a => new AuthorId { Guid = a.Guid }).ToList();
                    if (update.Diff.ShouldSerializeConcepts()) design.Concepts = update.Diff.Concepts?.Select(c => new ConceptId { Guid = c.Guid }).ToList();
                    if (update.Diff.Pieces != null)
                        design.Pieces = ApplyPiecesDiff(design.Pieces ?? new List<Piece>(), update.Diff.Pieces);
                    if (update.Diff.Connections != null)
                        design.Connections = ApplyConnectionsDiff(design.Connections ?? new List<Connection>(), update.Diff.Connections);
                    if (update.Diff.Attributes != null)
                        design.Attributes = ApplyAttributesDiff(design.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Piece> ApplyPiecesDiff(List<Piece> basePieces, PiecesDiff diff)
    {
        var result = new List<Piece>(basePieces);

        if (diff.Removed != null)
            result.RemoveAll(p => diff.Removed.Any(r => r.Guid == p.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var piece = result.FirstOrDefault(p => p.Guid == update.Piece.Guid);
                if (piece != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) piece.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) piece.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeType()) piece.Type = update.Diff.Type;
                    if (update.Diff.ShouldSerializeDesign()) piece.Design = update.Diff.Design;
                    if (update.Diff.ShouldSerializePlane()) piece.Plane = update.Diff.Plane;
                    if (update.Diff.ShouldSerializeCenter()) piece.Center = update.Diff.Center;
                    if (update.Diff.ShouldSerializeScale()) piece.Scale = update.Diff.Scale;
                    if (update.Diff.ShouldSerializeMirrorPlane()) piece.MirrorPlane = update.Diff.MirrorPlane;
                    if (update.Diff.ShouldSerializeIsHidden()) piece.IsHidden = update.Diff.IsHidden;
                    if (update.Diff.ShouldSerializeIsLocked()) piece.IsLocked = update.Diff.IsLocked;
                    if (update.Diff.ShouldSerializeColor()) piece.Color = update.Diff.Color;
                    if (update.Diff.Attributes != null)
                        piece.Attributes = ApplyAttributesDiff(piece.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    private static List<Connection> ApplyConnectionsDiff(List<Connection> baseConnections, ConnectionsDiff diff)
    {
        var result = new List<Connection>(baseConnections);

        if (diff.Removed != null)
            result.RemoveAll(c => diff.Removed.Any(r => r.Guid == c.Guid));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var connection = result.FirstOrDefault(c => c.Guid == update.Connection.Guid);
                if (connection != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeConnected() && update.Diff.Connected != null)
                    {
                        var s = connection.Connected ?? new Side();
                        if (update.Diff.Connected.ShouldSerializePiece()) s.Piece = update.Diff.Connected.Piece;
                        if (update.Diff.Connected.ShouldSerializeDesignPiece()) s.DesignPiece = update.Diff.Connected.DesignPiece;
                        if (update.Diff.Connected.ShouldSerializeConnector()) s.Connector = update.Diff.Connected.Connector;
                        connection.Connected = s;
                    }
                    if (update.Diff.ShouldSerializeConnecting() && update.Diff.Connecting != null)
                    {
                        var s = connection.Connecting ?? new Side();
                        if (update.Diff.Connecting.ShouldSerializePiece()) s.Piece = update.Diff.Connecting.Piece;
                        if (update.Diff.Connecting.ShouldSerializeDesignPiece()) s.DesignPiece = update.Diff.Connecting.DesignPiece;
                        if (update.Diff.Connecting.ShouldSerializeConnector()) s.Connector = update.Diff.Connecting.Connector;
                        connection.Connecting = s;
                    }
                    if (update.Diff.ShouldSerializeDescription()) connection.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeGap()) connection.Gap = connection.Gap + (update.Diff.Gap ?? 0f);
                    if (update.Diff.ShouldSerializeShift()) connection.Shift = connection.Shift + (update.Diff.Shift ?? 0f);
                    if (update.Diff.ShouldSerializeRise()) connection.Rise = connection.Rise + (update.Diff.Rise ?? 0f);
                    if (update.Diff.ShouldSerializeRotation()) connection.Rotation = connection.Rotation + (update.Diff.Rotation ?? 0f);
                    if (update.Diff.ShouldSerializeTurn()) connection.Turn = connection.Turn + (update.Diff.Turn ?? 0f);
                    if (update.Diff.ShouldSerializeTilt()) connection.Tilt = connection.Tilt + (update.Diff.Tilt ?? 0f);
                    if (update.Diff.ShouldSerializeU()) connection.U = (connection.U ?? 0f) + (update.Diff.U ?? 0f);
                    if (update.Diff.ShouldSerializeV()) connection.V = (connection.V ?? 0f) + (update.Diff.V ?? 0f);
                    if (update.Diff.Attributes != null)
                        connection.Attributes = ApplyAttributesDiff(connection.Attributes ?? new List<Attribute>(), update.Diff.Attributes);
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    public static bool AreKitsEqual(Kit a, Kit b)
    {
        const double Tolerance = 1e-5f;
        List<T> NormalizeArray<T>(List<T>? arr) => arr ?? new List<T>();
        string? NormalizeValue(string? value) => string.IsNullOrEmpty(value) ? null : value;
        bool? NormalizeBoolean(bool? value) => value == true ? true : null;
        bool FloatEqual(double valA, double valB) => Math.Abs(valA - valB) < Tolerance;
        bool NullableFloatEqual(double? valA, double? valB)
        {
            if (valA == null && valB == null) return true;
            if (valA == null || valB == null) return false;
            return Math.Abs(valA.Value - valB.Value) < Tolerance;
        }

        bool AreAttributesEqual(List<Attribute>? arrA, List<Attribute>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var attrA in listA)
            {
                var attrB = listB.FirstOrDefault(x => x.Guid == attrA.Guid);
                if (attrB == null) return false;
                if (attrA.Key != attrB.Key) return false;
                if (NormalizeValue(attrA.Value) != NormalizeValue(attrB.Value)) return false;
                if (NormalizeValue(attrA.Definition) != NormalizeValue(attrB.Definition)) return false;
            }
            return true;
        }

        bool ArePropsEqual(List<Prop>? arrA, List<Prop>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var propA in listA)
            {
                var propB = listB.FirstOrDefault(x => x.Guid == propA.Guid);
                if (propB == null) return false;
                if (propA.Quality?.Guid != propB.Quality?.Guid) return false;
                if (propA.Value != propB.Value) return false;
                if (NormalizeValue(propA.Unit) != NormalizeValue(propB.Unit)) return false;
                if (!AreAttributesEqual(propA.Attributes, propB.Attributes)) return false;
            }
            return true;
        }

        bool AreConnectorsEqual(List<Connector>? arrA, List<Connector>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var connA in listA)
            {
                var connB = listB.FirstOrDefault(x => x.Guid == connA.Guid);
                if (connB == null) return false;
                if (NormalizeValue(connA.Name) != NormalizeValue(connB.Name)) return false;
                if (!NullableFloatEqual(connA.Point?.X, connB.Point?.X)) return false;
                if (!NullableFloatEqual(connA.Point?.Y, connB.Point?.Y)) return false;
                if (!NullableFloatEqual(connA.Point?.Z, connB.Point?.Z)) return false;
                if (!NullableFloatEqual(connA.Direction?.X, connB.Direction?.X)) return false;
                if (!NullableFloatEqual(connA.Direction?.Y, connB.Direction?.Y)) return false;
                if (!NullableFloatEqual(connA.Direction?.Z, connB.Direction?.Z)) return false;
                if (!FloatEqual(connA.T, connB.T)) return false;
                if (NormalizeBoolean(connA.Mandatory) != NormalizeBoolean(connB.Mandatory)) return false;
                if (connA.Port?.Guid != connB.Port?.Guid) return false;
                if (!ArePropsEqual(connA.Props, connB.Props)) return false;
                if (!AreAttributesEqual(connA.Attributes, connB.Attributes)) return false;
            }
            return true;
        }

        bool AreModelsEqual(List<Model>? arrA, List<Model>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var modelA in listA)
            {
                var modelB = listB.FirstOrDefault(x => x.Guid == modelA.Guid);
                if (modelB == null) return false;
                if (NormalizeValue(modelA.Name) != NormalizeValue(modelB.Name)) return false;
                if (modelA.File?.Guid != modelB.File?.Guid) return false;
                var tagsA = NormalizeArray(modelA.Tags).Select(t => t.Guid).OrderBy(g => g).ToList();
                var tagsB = NormalizeArray(modelB.Tags).Select(t => t.Guid).OrderBy(g => g).ToList();
                if (!tagsA.SequenceEqual(tagsB)) return false;
                if (!AreAttributesEqual(modelA.Attributes, modelB.Attributes)) return false;
            }
            return true;
        }

        bool AreTypesEqual(List<Type>? arrA, List<Type>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var typeA in listA)
            {
                var typeB = listB.FirstOrDefault(t =>
                {
                    if (t.Guid != typeA.Guid) return false;
                    if (t.Parent == null && typeA.Parent == null) return true;
                    if (t.Parent == null || typeA.Parent == null) return false;
                    return t.Parent.Guid == typeA.Parent.Guid;
                });
                if (typeB == null) return false;
                if (typeA.Name != typeB.Name) return false;
                if (NormalizeValue(typeA.Description) != NormalizeValue(typeB.Description)) return false;
                if (NormalizeValue(typeA.Icon) != NormalizeValue(typeB.Icon)) return false;
                if (NormalizeValue(typeA.Image) != NormalizeValue(typeB.Image)) return false;
                if (NormalizeValue(typeA.Folder) != NormalizeValue(typeB.Folder)) return false;
                if (NormalizeValue(typeA.Unit) != NormalizeValue(typeB.Unit)) return false;
                if (typeA.Stock != typeB.Stock) return false;
                if (NormalizeBoolean(typeA.IsAbstract) != NormalizeBoolean(typeB.IsAbstract)) return false;
                if (NormalizeBoolean(typeA.Virtual) != NormalizeBoolean(typeB.Virtual)) return false;
                if (typeA.Location?.Guid != typeB.Location?.Guid) return false;
                var conceptsA = NormalizeArray(typeA.Concepts).Select(c => c.Guid).OrderBy(g => g).ToList();
                var conceptsB = NormalizeArray(typeB.Concepts).Select(c => c.Guid).OrderBy(g => g).ToList();
                if (!conceptsA.SequenceEqual(conceptsB)) return false;
                var authorsA = NormalizeArray(typeA.Authors).Select(a => a.Guid).OrderBy(g => g).ToList();
                var authorsB = NormalizeArray(typeB.Authors).Select(a => a.Guid).OrderBy(g => g).ToList();
                if (!authorsA.SequenceEqual(authorsB)) return false;
                if (!ArePropsEqual(typeA.Props, typeB.Props)) return false;
                if (!AreModelsEqual(typeA.Models, typeB.Models)) return false;
                if (!AreConnectorsEqual(typeA.Connectors, typeB.Connectors)) return false;
                if (!AreAttributesEqual(typeA.Attributes, typeB.Attributes)) return false;
            }
            return true;
        }

        bool ArePiecesEqual(List<Piece>? arrA, List<Piece>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var pieceA in listA)
            {
                var pieceB = listB.FirstOrDefault(x => x.Guid == pieceA.Guid);
                if (pieceB == null) return false;
                if (NormalizeValue(pieceA.Name) != NormalizeValue(pieceB.Name)) return false;
                if (pieceA.Type?.Guid != pieceB.Type?.Guid) return false;
                if (pieceA.Design?.Guid != pieceB.Design?.Guid) return false;
                if (pieceA.Plane != null && pieceB.Plane != null)
                {
                    if (!NullableFloatEqual(pieceA.Plane.Origin?.X, pieceB.Plane.Origin?.X)) return false;
                    if (!NullableFloatEqual(pieceA.Plane.Origin?.Y, pieceB.Plane.Origin?.Y)) return false;
                    if (!NullableFloatEqual(pieceA.Plane.Origin?.Z, pieceB.Plane.Origin?.Z)) return false;
                    if (!NullableFloatEqual(pieceA.Plane.XAxis?.X, pieceB.Plane.XAxis?.X)) return false;
                    if (!NullableFloatEqual(pieceA.Plane.XAxis?.Y, pieceB.Plane.XAxis?.Y)) return false;
                    if (!NullableFloatEqual(pieceA.Plane.XAxis?.Z, pieceB.Plane.XAxis?.Z)) return false;
                    if (!NullableFloatEqual(pieceA.Plane.YAxis?.X, pieceB.Plane.YAxis?.X)) return false;
                    if (!NullableFloatEqual(pieceA.Plane.YAxis?.Y, pieceB.Plane.YAxis?.Y)) return false;
                    if (!NullableFloatEqual(pieceA.Plane.YAxis?.Z, pieceB.Plane.YAxis?.Z)) return false;
                }
                else if (pieceA.Plane != null || pieceB.Plane != null)
                {
                    return false;
                }
                if (pieceA.Center != null && pieceB.Center != null)
                {
                    if (!NullableFloatEqual(pieceA.Center.U, pieceB.Center.U)) return false;
                    if (!NullableFloatEqual(pieceA.Center.V, pieceB.Center.V)) return false;
                }
                else if (pieceA.Center != null || pieceB.Center != null)
                {
                    return false;
                }
                if (!NullableFloatEqual(pieceA.Scale, pieceB.Scale)) return false;
                if (pieceA.MirrorPlane != null && pieceB.MirrorPlane != null)
                {
                    if (!NullableFloatEqual(pieceA.MirrorPlane.Origin?.X, pieceB.MirrorPlane.Origin?.X)) return false;
                    if (!NullableFloatEqual(pieceA.MirrorPlane.Origin?.Y, pieceB.MirrorPlane.Origin?.Y)) return false;
                    if (!NullableFloatEqual(pieceA.MirrorPlane.Origin?.Z, pieceB.MirrorPlane.Origin?.Z)) return false;
                    if (!NullableFloatEqual(pieceA.MirrorPlane.XAxis?.X, pieceB.MirrorPlane.XAxis?.X)) return false;
                    if (!NullableFloatEqual(pieceA.MirrorPlane.XAxis?.Y, pieceB.MirrorPlane.XAxis?.Y)) return false;
                    if (!NullableFloatEqual(pieceA.MirrorPlane.XAxis?.Z, pieceB.MirrorPlane.XAxis?.Z)) return false;
                    if (!NullableFloatEqual(pieceA.MirrorPlane.YAxis?.X, pieceB.MirrorPlane.YAxis?.X)) return false;
                    if (!NullableFloatEqual(pieceA.MirrorPlane.YAxis?.Y, pieceB.MirrorPlane.YAxis?.Y)) return false;
                    if (!NullableFloatEqual(pieceA.MirrorPlane.YAxis?.Z, pieceB.MirrorPlane.YAxis?.Z)) return false;
                }
                else if (pieceA.MirrorPlane != null || pieceB.MirrorPlane != null)
                {
                    return false;
                }
                if (NormalizeBoolean(pieceA.IsHidden) != NormalizeBoolean(pieceB.IsHidden)) return false;
                if (NormalizeBoolean(pieceA.IsLocked) != NormalizeBoolean(pieceB.IsLocked)) return false;
                if (NormalizeValue(pieceA.Color) != NormalizeValue(pieceB.Color)) return false;
                if (NormalizeValue(pieceA.Description) != NormalizeValue(pieceB.Description)) return false;
                if (!ArePropsEqual(pieceA.Props, pieceB.Props)) return false;
                if (!AreAttributesEqual(pieceA.Attributes, pieceB.Attributes)) return false;
            }
            return true;
        }

        bool AreConnectionsEqual(List<Connection>? arrA, List<Connection>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var connA in listA)
            {
                var connB = listB.FirstOrDefault(x => x.Guid == connA.Guid);
                if (connB == null) return false;
                if (connA.Connected?.Piece?.Guid != connB.Connected?.Piece?.Guid) return false;
                if (connA.Connected?.DesignPiece?.Guid != connB.Connected?.DesignPiece?.Guid) return false;
                if (connA.Connected?.Connector?.Guid != connB.Connected?.Connector?.Guid) return false;
                if (connA.Connecting?.Piece?.Guid != connB.Connecting?.Piece?.Guid) return false;
                if (connA.Connecting?.DesignPiece?.Guid != connB.Connecting?.DesignPiece?.Guid) return false;
                if (connA.Connecting?.Connector?.Guid != connB.Connecting?.Connector?.Guid) return false;
                if (!FloatEqual(connA.Gap, connB.Gap)) return false;
                if (!FloatEqual(connA.Shift, connB.Shift)) return false;
                if (!FloatEqual(connA.Rise, connB.Rise)) return false;
                if (!FloatEqual(connA.Rotation, connB.Rotation)) return false;
                if (!FloatEqual(connA.Turn, connB.Turn)) return false;
                if (!FloatEqual(connA.Tilt, connB.Tilt)) return false;
                if (!NullableFloatEqual(connA.U, connB.U)) return false;
                if (!NullableFloatEqual(connA.V, connB.V)) return false;
                if (NormalizeValue(connA.Description) != NormalizeValue(connB.Description)) return false;
                if (!AreAttributesEqual(connA.Attributes, connB.Attributes)) return false;
            }
            return true;
        }

        bool AreDesignsEqual(List<Design>? arrA, List<Design>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var designA in listA)
            {
                var designB = listB.FirstOrDefault(d =>
                {
                    if (d.Guid != designA.Guid) return false;
                    if (d.Parent == null && designA.Parent == null) return true;
                    if (d.Parent == null || designA.Parent == null) return false;
                    return d.Parent.Guid == designA.Parent.Guid;
                });
                if (designB == null) return false;
                if (designA.Name != designB.Name) return false;
                if (NormalizeValue(designA.Description) != NormalizeValue(designB.Description)) return false;
                if (NormalizeValue(designA.Icon) != NormalizeValue(designB.Icon)) return false;
                if (NormalizeValue(designA.Image) != NormalizeValue(designB.Image)) return false;
                var conceptsA = NormalizeArray(designA.Concepts).Select(c => c.Guid).OrderBy(g => g).ToList();
                var conceptsB = NormalizeArray(designB.Concepts).Select(c => c.Guid).OrderBy(g => g).ToList();
                if (!conceptsA.SequenceEqual(conceptsB)) return false;
                if (!ArePiecesEqual(designA.Pieces, designB.Pieces)) return false;
                if (!AreConnectionsEqual(designA.Connections, designB.Connections)) return false;
                if (!AreAttributesEqual(designA.Attributes, designB.Attributes)) return false;
            }
            return true;
        }

        bool ArePortsEqual(List<Port>? arrA, List<Port>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var portA in listA)
            {
                var portB = listB.FirstOrDefault(x => x.Guid == portA.Guid);
                if (portB == null) return false;
                if (portA.Name != portB.Name) return false;
                if (NormalizeValue(portA.Description) != NormalizeValue(portB.Description)) return false;
                if (!AreAttributesEqual(portA.Attributes, portB.Attributes)) return false;
            }
            return true;
        }

        bool AreQualitiesEqual(List<Quality>? arrA, List<Quality>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var qualA in listA)
            {
                var qualB = listB.FirstOrDefault(x => x.Guid == qualA.Guid);
                if (qualB == null) return false;
                if (qualA.Key != qualB.Key) return false;
                if (qualA.Name != qualB.Name) return false;
                if (!AreAttributesEqual(qualA.Attributes, qualB.Attributes)) return false;
            }
            return true;
        }

        bool AreFilesEqual(List<File>? arrA, List<File>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var fileA in listA)
            {
                var fileB = listB.FirstOrDefault(x => x.Guid == fileA.Guid);
                if (fileB == null) return false;
                if (fileA.Name != fileB.Name) return false;
            }
            return true;
        }

        bool AreFoldersEqual(List<Folder>? arrA, List<Folder>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var folderA in listA)
            {
                var folderB = listB.FirstOrDefault(x => x.Guid == folderA.Guid);
                if (folderB == null) return false;
                if (folderA.Name != folderB.Name) return false;
                if (!AreAttributesEqual(folderA.Attributes, folderB.Attributes)) return false;
            }
            return true;
        }

        bool AreAuthorsEqual(List<Author>? arrA, List<Author>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var authorA in listA)
            {
                var authorB = listB.FirstOrDefault(x => x.Guid == authorA.Guid);
                if (authorB == null) return false;
                if (authorA.Name != authorB.Name) return false;
                if (NormalizeValue(authorA.Email) != NormalizeValue(authorB.Email)) return false;
                if (!AreAttributesEqual(authorA.Attributes, authorB.Attributes)) return false;
            }
            return true;
        }

        bool AreConceptsEqual(List<Concept>? arrA, List<Concept>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var conceptA in listA)
            {
                var conceptB = listB.FirstOrDefault(x => x.Guid == conceptA.Guid);
                if (conceptB == null) return false;
                if (conceptA.Name != conceptB.Name) return false;
                if (NormalizeValue(conceptA.Description) != NormalizeValue(conceptB.Description)) return false;
                if (NormalizeValue(conceptA.Icon) != NormalizeValue(conceptB.Icon)) return false;
            }
            return true;
        }

        bool AreTagsEqual(List<Tag>? arrA, List<Tag>? arrB)
        {
            var listA = NormalizeArray(arrA);
            var listB = NormalizeArray(arrB);
            if (listA.Count != listB.Count) return false;
            foreach (var tagA in listA)
            {
                var tagB = listB.FirstOrDefault(x => x.Guid == tagA.Guid);
                if (tagB == null) return false;
                if (tagA.Name != tagB.Name) return false;
                if (NormalizeValue(tagA.Description) != NormalizeValue(tagB.Description)) return false;
                if (NormalizeValue(tagA.Icon) != NormalizeValue(tagB.Icon)) return false;
            }
            return true;
        }

        if (a.Guid != b.Guid) return false;
        if (a.Name != b.Name) return false;
        if (NormalizeValue(a.Version) != NormalizeValue(b.Version)) return false;
        if (NormalizeValue(a.Description) != NormalizeValue(b.Description)) return false;
        if (NormalizeValue(a.Icon) != NormalizeValue(b.Icon)) return false;
        if (NormalizeValue(a.Image) != NormalizeValue(b.Image)) return false;
        if (NormalizeValue(a.Preview) != NormalizeValue(b.Preview)) return false;
        if (NormalizeValue(a.Remote) != NormalizeValue(b.Remote)) return false;
        if (NormalizeValue(a.Homepage) != NormalizeValue(b.Homepage)) return false;
        if (NormalizeValue(a.License) != NormalizeValue(b.License)) return false;

        if (!AreConceptsEqual(a.Concepts, b.Concepts)) return false;
        if (!AreTagsEqual(a.Tags, b.Tags)) return false;
        if (!AreTypesEqual(a.Types, b.Types)) return false;
        if (!AreDesignsEqual(a.Designs, b.Designs)) return false;
        if (!ArePortsEqual(a.Ports, b.Ports)) return false;
        if (!AreQualitiesEqual(a.Qualities, b.Qualities)) return false;
        if (!AreFilesEqual(a.Files, b.Files)) return false;
        if (!AreFoldersEqual(a.Folders, b.Folders)) return false;
        if (!AreAuthorsEqual(a.Authors, b.Authors)) return false;
        if (!AreAttributesEqual(a.Attributes, b.Attributes)) return false;

        return true;
    }

    public static bool AreKitDiffsEqual(KitDiff a, KitDiff b)
    {
        return Utility.Serialize(a) == Utility.Serialize(b);
    }
}

#endregion ❄️SemioDiff

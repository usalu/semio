#region Header

// net/Semio/Semio.cs

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion Header

#region TODOs

// TODO: Make remote uris work for diagram.
// TODO: Remove computeChildPlane and separate the flatten diagram and flatten planes parts.
// TODO: Refactor all ToSring() to use ToIdString() and add ABREVIATION(ID) to entity.
// TODO: Develop a validation template for urls.
// TODO: Replace GetHashcode() with a proper hash function.
// TODO: Add logging mechanism to all API calls if they fail.
// TODO: Implement reflexive validation for entity properties.
// TODO: Add index to prop and add to list based on index not on source code order.
// TODO: See if Utility.Encode(uri) can be added by attribute on parameters.
// TODO: Turn inplace and leave clone to the user of the function.
// TODO: Parametrize colors for diagram

#endregion TODOs

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

#endregion Constants

#region Utility

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


    public static T? Deserialize<T>(this string json) => JsonConvert.DeserializeObject<T>(json, new JsonSerializerSettings { ContractResolver = new CamelCasePropertyNamesContractResolver() });

    public static string GenerateRandomId(int seed)
    {
        var adjectives = Resources.adjectives.Deserialize<List<string>>();
        var animals = Resources.animals.Deserialize<List<string>>();
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

#region Expressions
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

#endregion Expressions

#endregion Utility

#region Entitying

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

    public T? DeepClone() => this.Serialize().Deserialize<T>();

    public virtual (bool, List<string>) Validate()
    {
        var result = new EntityValidator<T>().Validate((T)this);
        return (result.IsValid, result.Errors.Select(e => e.ToString()).ToList());
    }
}

public class EntityValidator<T> : AbstractValidator<T> where T : Entity<T>
{
    public EntityValidator()
    {
    }
}

#region SemioValidation




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

#endregion SemioValidation

public class DiffUpdate<T>
{
    public string Id { get; set; } = "";
    public T? Diff { get; set; }
}

#region Attribute


public class AttributeId : Entity<AttributeId>
{
    public string Guid { get; set; } = "";

    public static implicit operator AttributeId(Attribute attribute) => new() { Guid = attribute.Guid };
    public static implicit operator AttributeId(AttributeDiff diff) => new() { Guid = diff.Guid ?? "" };
}


public class AttributeDiff : Entity<AttributeDiff>
{
    public string? Guid { get; set; }
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
    public string Definition { get; set; } = "";

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
    public List<DiffUpdate<AttributeDiff>> Updated { get; set; } = new();

    public AttributesDiff MergeDiff(AttributesDiff other)
    {
        return new AttributesDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }

    public static implicit operator AttributesDiff(List<Attribute> attributes) => new() { Updated = attributes.Select(a => new DiffUpdate<AttributeDiff> { Id = a.Guid, Diff = (AttributeDiff)a }).ToList() };
}





public class Attribute : Entity<Attribute>
{
    public string Guid { get; set; } = "";
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
    public string Definition { get; set; } = "";

    public static implicit operator Attribute(AttributeId id) => new() { Guid = id.Guid };
    public static implicit operator Attribute(AttributeDiff diff) => new() { Guid = diff.Guid ?? "", Key = diff.Key, Value = diff.Value, Definition = diff.Definition };

    public Attribute ApplyDiff(AttributeDiff diff)
    {
        return new Attribute
        {
            Guid = Guid,
            Key = !string.IsNullOrEmpty(diff.Key) ? diff.Key : Key,
            Value = !string.IsNullOrEmpty(diff.Value) ? diff.Value : Value,
            Definition = !string.IsNullOrEmpty(diff.Definition) ? diff.Definition : Definition
        };
    }
    public AttributeDiff CreateDiff()
    {
        return new AttributeDiff
        {
            Guid = Guid,
            Key = Key,
            Value = Value,
            Definition = Definition
        };
    }
    public AttributeDiff InverseDiff(AttributeDiff appliedDiff)
    {
        return new AttributeDiff
        {
            Guid = Guid,
            Key = !string.IsNullOrEmpty(appliedDiff.Key) ? Key : "",
            Value = !string.IsNullOrEmpty(appliedDiff.Value) ? Value : "",
            Definition = !string.IsNullOrEmpty(appliedDiff.Definition) ? Definition : ""
        };
    }

    public string ToIdString() => $"{Key}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Atr({ToHumanIdString()})";
}

#endregion Attribute

#region Coord





public class Coord : Entity<Coord>
{
    public float U { get; set; }
    public float V { get; set; }

    public Coord Normalize()
    {
        var length = (float)Math.Sqrt(U * U + V * V);
        return new Coord { U = U / length, V = V / length };
    }
}

#endregion Coord

#region Point





public class Point : Entity<Point>
{
    public float X { get; set; } = 0;
    public float Y { get; set; } = 0;
    public float Z { get; set; } = 0;
}

#endregion Point

#region Vector





public class Vector : Entity<Vector>
{
    public float X { get; set; } = 1;
    public float Y { get; set; }
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

#endregion Vector

#region Plane





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

#endregion Plane

#region Location


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
    public float Longitude { get; set; }
    public float Latitude { get; set; }
    public float? Altitude { get; set; }
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Loc({ToHumanIdString()})";
}

#endregion Location

#region Author


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
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public string? Email { get; set; }
    public List<Attribute>? Attributes { get; set; }

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
    public List<DiffUpdate<AuthorDiff>> Updated { get; set; } = new();

    public AuthorsDiff MergeDiff(AuthorsDiff other)
    {
        return new AuthorsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }

    public static implicit operator AuthorsDiff(List<Author> authors) => new() { Updated = authors.Select(a => new DiffUpdate<AuthorDiff> { Id = a.Guid, Diff = (AuthorDiff)a }).ToList() };
}

#endregion Author

#region File


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
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public string? Remote { get; set; }
    public FolderId? Folder { get; set; }
    public int? Size { get; set; }
    public string? Hash { get; set; }
    public DateTime? CreatedAt { get; set; }
    public string? CreatedBy { get; set; }
    public DateTime? UpdatedAt { get; set; }
    public string? UpdatedBy { get; set; }

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
    public List<DiffUpdate<FileDiff>> Updated { get; set; } = new();
    public List<File> Added { get; set; } = new();

    public static implicit operator FilesDiff(List<File> files) => new() { Updated = files.Select(f => new DiffUpdate<FileDiff> { Id = f.Guid, Diff = (FileDiff)f }).ToList() };
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
    public static implicit operator File(FileDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Remote = diff.Remote, Folder = diff.Folder, Size = diff.Size, Hash = diff.Hash, CreatedAt = diff.CreatedAt ?? default, CreatedBy = diff.CreatedBy, UpdatedAt = diff.UpdatedAt ?? default, UpdatedBy = diff.UpdatedBy };
    public static implicit operator FileDiff(File file) => new() { Guid = file.Guid, Name = file.Name, Remote = file.Remote, Folder = file.Folder, Size = file.Size, Hash = file.Hash, CreatedAt = file.CreatedAt, CreatedBy = file.CreatedBy, UpdatedAt = file.UpdatedAt, UpdatedBy = file.UpdatedBy };
}
#endregion File

#region Folder


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
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public string? Parent { get; set; }
    public string? Description { get; set; }
    public List<Attribute>? Attributes { get; set; }
    public string? CreatedAt { get; set; }
    public string? CreatedBy { get; set; }
    public string? UpdatedAt { get; set; }
    public string? UpdatedBy { get; set; }

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
    public List<DiffUpdate<FolderDiff>> Updated { get; set; } = new();
    public List<Folder> Added { get; set; } = new();

    public static implicit operator FoldersDiff(List<Folder> folders) => new() { Updated = folders.Select(f => new DiffUpdate<FolderDiff> { Id = f.Guid, Diff = (FolderDiff)f }).ToList() };
}


public class Folder : Entity<Folder>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string? Parent { get; set; }
    public string Description { get; set; } = "";
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

    public Folder ApplyDiff(FolderDiff diff)
    {
        return new Folder
        {
            Guid = diff.Guid ?? Guid,
            Name = diff.Name ?? Name,
            Parent = diff.Parent ?? Parent,
            Description = diff.Description ?? Description,
            Attributes = diff.Attributes ?? Attributes,
            CreatedAt = diff.CreatedAt ?? CreatedAt,
            CreatedBy = diff.CreatedBy ?? CreatedBy,
            UpdatedAt = diff.UpdatedAt ?? UpdatedAt,
            UpdatedBy = diff.UpdatedBy ?? UpdatedBy
        };
    }
}

#endregion Folder

#region Benchmark


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
    public float? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public float? Max { get; set; }
    public bool? MaxExcluded { get; set; }
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Bmk({ToHumanIdString()})";
}

#endregion Benchmark

#region QualityKind

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

#endregion QualityKind

#region Quality





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
    public string Description { get; set; } = "";
    public string Uri { get; set; } = "";
    public bool Scalable { get; set; } = false;
    public QualityKind Kind { get; set; } = QualityKind.General;
    public string SI { get; set; } = "";
    public string Imperial { get; set; } = "";
    public float Min { get; set; } = 0;
    public bool MinExcluded { get; set; } = true;
    public float Max { get; set; } = 0;
    public bool MaxExcluded { get; set; } = true;
    public float Default { get; set; } = 0;
    public string Formula { get; set; } = "";
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
    public string Description { get; set; } = "";
    public string Uri { get; set; } = "";
    public string? Folder { get; set; }
    public bool Scalable { get; set; } = false;
    public QualityKind Kind { get; set; } = QualityKind.General;
    public string SI { get; set; } = "";
    public string Imperial { get; set; } = "";
    public float Min { get; set; } = 0;
    public bool MinExcluded { get; set; } = true;
    public float Max { get; set; } = 0;
    public bool MaxExcluded { get; set; } = true;
    public float Default { get; set; } = 0;
    public string Formula { get; set; } = "";
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

#endregion Quality

#region Tag





public class TagId : Entity<TagId>
{
    public string Guid { get; set; } = "";

    public static implicit operator TagId(Tag tag) => new() { Guid = tag.Guid };
}





public class Tag : Entity<Tag>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Tag(TagId id) => new() { Guid = id.Guid };
}

#endregion Tag

#region Concept





public class ConceptId : Entity<ConceptId>
{
    public string Guid { get; set; } = "";

    public static implicit operator ConceptId(Concept concept) => new() { Guid = concept.Guid };
}





public class Concept : Entity<Concept>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Concept(ConceptId id) => new() { Guid = id.Guid };
}


public class ConceptDiff : Entity<ConceptDiff>
{
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public AttributesDiff? Attributes { get; set; }
}


public class ConceptsDiff : Entity<ConceptsDiff>
{
    public List<ConceptId> Removed { get; set; } = new();
    public List<Concept> Added { get; set; } = new();
    public List<DiffUpdate<ConceptDiff>> Updated { get; set; } = new();

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

#endregion Concept

#region Port





public class PortId : Entity<PortId>
{
    public string Guid { get; set; } = "";

    public static implicit operator PortId(Port iface) => new() { Guid = iface.Guid };
    public static implicit operator PortId(PortDiff diff) => new() { Guid = diff.Guid };
}


public class PortDiff : Entity<PortDiff>
{
    public string Guid { get; set; } = "";
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public List<PortId>? CompatiblePorts { get; set; }
    public List<Attribute>? Attributes { get; set; }

    public static implicit operator PortDiff(PortId id) => new() { Guid = id.Guid };
    public static implicit operator PortDiff(Port iface) => new() { Guid = iface.Guid, Name = iface.Name, Description = iface.Description, Icon = iface.Icon, CompatiblePorts = iface.CompatiblePorts?.Select(i => (PortId)i).ToList(), Attributes = iface.Attributes };
}


public class PortsDiff : Entity<PortsDiff>
{
    public List<PortId> Removed { get; set; } = new();
    public List<Port> Added { get; set; } = new();
    public List<DiffUpdate<PortDiff>> Updated { get; set; } = new();

    public static implicit operator PortsDiff(List<Port> ports) => new() { Updated = ports.Select(i => new DiffUpdate<PortDiff> { Id = i.Guid, Diff = (PortDiff)i }).ToList() };
}





public class Port : Entity<Port>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public List<PortId> CompatiblePorts { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Port(PortId id) => new() { Guid = id.Guid };
    public static implicit operator Port(PortDiff diff) => new()
    {
        Guid = diff.Guid,
        Name = diff.Name ?? "",
        Description = diff.Description ?? "",
        Icon = diff.Icon ?? "",
        CompatiblePorts = diff.CompatiblePorts ?? new(),
        Attributes = diff.Attributes ?? new()
    };

    public Port ApplyDiff(PortDiff diff)
    {
        return new Port
        {
            Guid = diff.Guid ?? Guid,
            Name = diff.Name ?? Name,
            Description = diff.Description ?? Description,
            Icon = diff.Icon ?? Icon,
            CompatiblePorts = diff.CompatiblePorts ?? CompatiblePorts,
            Attributes = diff.Attributes ?? Attributes
        };
    }

    public PortDiff CreateDiff()
    {
        return new PortDiff
        {
            Guid = Guid,
            Name = Name,
            Description = Description,
            Icon = Icon,
            CompatiblePorts = CompatiblePorts,
            Attributes = Attributes
        };
    }

    public PortDiff InverseDiff(PortDiff appliedDiff)
    {
        return new PortDiff
        {
            Guid = !string.IsNullOrEmpty(appliedDiff.Guid) ? Guid : "",
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? Name : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? Description : null,
            Icon = !string.IsNullOrEmpty(appliedDiff.Icon) ? Icon : null,
            CompatiblePorts = appliedDiff.CompatiblePorts?.Any() == true ? CompatiblePorts : null,
            Attributes = appliedDiff.Attributes?.Any() == true ? Attributes : null
        };
    }
}

#endregion Port

#region Prop


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
    public string Unit { get; set; } = "";
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Prp({ToHumanIdString()})";
}

#endregion Prop

#region Model


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
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public FileId? File { get; set; }
    public string Description { get; set; } = "";
    public List<TagId> Tags { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

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
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes
        };
    }
}


public class ModelsDiff : Entity<ModelsDiff>
{
    public List<ModelId> Removed { get; set; } = new();
    public List<Model> Added { get; set; } = new();
    public List<DiffUpdate<ModelDiff>> Updated { get; set; } = new();

    public ModelsDiff MergeDiff(ModelsDiff other)
    {
        return new ModelsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }

    public static implicit operator ModelsDiff(List<Model> models) => new() { Updated = models.Select(r => new DiffUpdate<ModelDiff> { Id = r.Guid, Diff = (ModelDiff)r }).ToList() };
}




public class Model : Entity<Model>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public FileId File { get; set; } = new();
    public string Description { get; set; } = "";
    public List<TagId> Tags { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Model(ModelId id) => new() { Guid = id.Guid };
    public static implicit operator Model(ModelDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", File = diff.File ?? new(), Description = diff.Description, Tags = diff.Tags, Attributes = diff.Attributes };

    public Model ApplyDiff(ModelDiff diff)
    {
        return new Model
        {
            Guid = Guid,
            Name = string.IsNullOrEmpty(diff.Name) ? Name : diff.Name,
            File = diff.File ?? File,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Tags = diff.Tags?.Any() == true ? diff.Tags : Tags,
            Attributes = diff.Attributes?.Any() == true ? diff.Attributes : Attributes
        };
    }

    public ModelDiff CreateDiff()
    {
        return new ModelDiff
        {
            Guid = Guid,
            Name = Name,
            File = File,
            Description = Description,
            Tags = Tags,
            Attributes = Attributes
        };
    }

    public ModelDiff InverseDiff(ModelDiff appliedDiff)
    {
        return new ModelDiff
        {
            Guid = Guid,
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? Name : null,
            File = appliedDiff.File != null ? File : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? Description : "",
            Tags = appliedDiff.Tags.Any() ? Tags : new List<TagId>(),
            Attributes = appliedDiff.Attributes.Any() ? Attributes : new List<Attribute>()
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
}

#endregion Model

#region Connector


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
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public string? Description { get; set; }
    public PortId? Port { get; set; }
    public bool? Mandatory { get; set; }
    public float? T { get; set; }
    public Point? Point { get; set; }
    public Vector? Direction { get; set; }
    public List<Prop>? Props { get; set; }
    public List<Attribute>? Attributes { get; set; }

    public static implicit operator ConnectorDiff(ConnectorId id) => new() { Guid = id.Guid };
    public static implicit operator ConnectorDiff(Connector connector) => new() { Guid = connector.Guid, Description = connector.Description, Port = connector.Port, Mandatory = connector.Mandatory, T = connector.T, Point = connector.Point, Direction = connector.Direction, Props = connector.Props, Attributes = connector.Attributes };

    public ConnectorDiff MergeDiff(ConnectorDiff other)
    {
        return new ConnectorDiff
        {
            Guid = other.Guid ?? Guid,
            Description = other.Description ?? Description,
            Port = other.Port ?? Port,
            Mandatory = other.Mandatory ?? Mandatory,
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
    public List<DiffUpdate<ConnectorDiff>> Updated { get; set; } = new();

    public ConnectorsDiff MergeDiff(ConnectorsDiff other)
    {
        return new ConnectorsDiff
        {
            Removed = Removed.Concat(other.Removed).Distinct().ToList(),
            Added = Added.Concat(other.Added).ToList(),
            Updated = Updated.Concat(other.Updated).ToList()
        };
    }

    public static implicit operator ConnectorsDiff(List<Connector> connectors) => new() { Updated = connectors.Select(p => new DiffUpdate<ConnectorDiff> { Id = p.Guid, Diff = (ConnectorDiff)p }).ToList() };
}





public class Connector : Entity<Connector>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public bool Mandatory { get; set; } = false;
    public PortId? Port { get; set; }
    public Point? Point { get; set; } = null;
    public Vector? Direction { get; set; } = null;
    public float T { get; set; } = 0;
    public List<Prop> Props { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Por({ToHumanIdString()})";

    public static implicit operator Connector(ConnectorId id) => new() { Guid = id.Guid };
    public static implicit operator Connector(ConnectorDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Description = diff.Description ?? "", Port = diff.Port, Mandatory = diff.Mandatory ?? false, T = diff.T ?? 0, Point = diff.Point, Direction = diff.Direction, Attributes = diff.Attributes ?? new() };
    public static implicit operator string(Connector connector) => connector.Guid;
    public static implicit operator Connector(string guid) => new() { Guid = guid };

    public Connector ApplyDiff(ConnectorDiff diff)
    {
        return new Connector
        {
            Guid = diff.Guid ?? Guid,
            Name = diff.Name ?? Name,
            Description = diff.Description ?? Description,
            Port = diff.Port ?? Port,
            Mandatory = diff.Mandatory ?? Mandatory,
            T = diff.T ?? T,
            Point = diff.Point ?? Point,
            Direction = diff.Direction ?? Direction,
            Props = diff.Props ?? Props,
            Attributes = diff.Attributes ?? Attributes
        };
    }

    public ConnectorDiff CreateDiff()
    {
        return new ConnectorDiff
        {
            Guid = Guid,
            Name = Name,
            Description = Description,
            Port = Port,
            Mandatory = Mandatory,
            T = T,
            Point = Point,
            Direction = Direction,
            Props = Props,
            Attributes = Attributes
        };
    }

    public ConnectorDiff InverseDiff(ConnectorDiff appliedDiff)
    {
        return new ConnectorDiff
        {
            Guid = !string.IsNullOrEmpty(appliedDiff.Guid) ? Guid : "",
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? Name : null,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? Description : "",
            Port = appliedDiff.Port is not null ? Port : null,
            Mandatory = appliedDiff.Mandatory.HasValue ? Mandatory : null,
            T = appliedDiff.T.HasValue ? T : null,
            Point = appliedDiff.Point is not null ? Point : null,
            Direction = appliedDiff.Direction is not null ? Direction : null,
            Props = appliedDiff.Props?.Any() == true ? Props : new List<Prop>(),
            Attributes = appliedDiff.Attributes?.Any() == true ? Attributes : new List<Attribute>()
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

    public bool IsSameAs(Connector other)
    {
        return Utility.Normalize(Guid) == Utility.Normalize(other.Guid);
    }

    public string FindAttributeValue(string name, string defaultValue = "")
    {
        var attribute = Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in connector {Guid}");
        return attribute?.Value ?? defaultValue;
    }

    public Connector SetAttribute(Attribute attribute)
    {
        var attributes = new List<Attribute>(Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Connector
        {
            Guid = Guid,
            Name = Name,
            Description = Description,
            Mandatory = Mandatory,
            Port = Port,
            Point = Point,
            Direction = Direction,
            T = T,
            Props = Props,
            Attributes = attributes
        };
    }
}

#endregion Connector

#region Type


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
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public TypeId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public int? Stock { get; set; }
    public bool? Virtual { get; set; }
    public string Uri { get; set; } = "";
    public string Unit { get; set; } = "";
    public Location? Location { get; set; }
    public ModelsDiff? Models { get; set; }
    public ConnectorsDiff? Connectors { get; set; }
    public List<AuthorId>? Authors { get; set; }
    public List<Attribute>? Attributes { get; set; }
    public List<ConceptId>? Concepts { get; set; }
    public DateTime? CreatedAt { get; set; }
    public DateTime? UpdatedAt { get; set; }

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
            Attributes = other.Attributes is not null && other.Attributes.Any() ? other.Attributes : Attributes,
            Concepts = other.Concepts is not null && other.Concepts.Any() ? other.Concepts : Concepts
        };
    }

    public static implicit operator TypeDiff(TypeId id) => new() { Guid = id.Guid };
    public static implicit operator TypeDiff(Type type) => new() { Name = type.Name, Description = type.Description, Icon = type.Icon, Image = type.Image, Stock = type.Stock, Virtual = type.Virtual, Uri = type.Uri, Unit = type.Unit, Location = type.Location, Models = new ModelsDiff { Added = new List<Model>(), Removed = new List<ModelId>(), Updated = type.Models.Select(m => new DiffUpdate<ModelDiff> { Id = m.Guid, Diff = m.CreateDiff() }).ToList() }, Connectors = new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Updated = type.Connectors.Select(p => new DiffUpdate<ConnectorDiff> { Id = p.Guid, Diff = p.CreateDiff() }).ToList() }, Authors = type.Authors, Attributes = type.Attributes, Concepts = type.Concepts };
}


public class TypesDiff : Entity<TypesDiff>
{
    public List<TypeId> Removed { get; set; } = new();
    public List<Type> Added { get; set; } = new();
    public List<DiffUpdate<TypeDiff>> Updated { get; set; } = new();

    public static implicit operator TypesDiff(List<Type> types) => new() { Updated = types.Select(t => new DiffUpdate<TypeDiff> { Id = t.Guid, Diff = (TypeDiff)t }).ToList() };
}





public class Type : Entity<Type>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public TypeId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Image { get; set; } = "";
    public int Stock { get; set; } = 2147483647;
    public bool Virtual { get; set; } = false;
    public string Uri { get; set; } = "";
    public Location? Location { get; set; }
    public string Unit { get; set; } = "";
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
        Description = diff.Description ?? "",
        Icon = diff.Icon ?? "",
        Image = diff.Image ?? "",
        Stock = diff.Stock ?? 2147483647,
        Virtual = diff.Virtual ?? false,
        Uri = diff.Uri ?? "",
        Unit = diff.Unit ?? "",
        Location = diff.Location,
        Models = diff.Models?.Added ?? new(),
        Connectors = diff.Connectors?.Added ?? new(),
        Authors = diff.Authors ?? new(),
        Attributes = diff.Attributes ?? new(),
        Concepts = diff.Concepts ?? new(),
        CreatedAt = diff.CreatedAt ?? DateTime.UtcNow,
        UpdatedAt = diff.UpdatedAt ?? DateTime.UtcNow
    };
    public static implicit operator string(Type type) => type.Name;
    public static implicit operator Type(string name) => new() { Name = name, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };

    public Type ApplyDiff(TypeDiff diff)
    {
        var models = Models;
        var connectors = Connectors;

        if (diff.Models is not null)
            models = ApplyModelsDiff(Models, diff.Models);
        if (diff.Connectors is not null)
            connectors = ApplyConnectorsDiff(Connectors, diff.Connectors);

        return new Type
        {
            Guid = Guid,
            Name = string.IsNullOrEmpty(diff.Name) ? Name : diff.Name,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Icon = string.IsNullOrEmpty(diff.Icon) ? Icon : diff.Icon,
            Image = string.IsNullOrEmpty(diff.Image) ? Image : diff.Image,
            Stock = diff.Stock ?? Stock,
            Virtual = diff.Virtual ?? Virtual,
            Uri = string.IsNullOrEmpty(diff.Uri) ? Uri : diff.Uri,
            Unit = string.IsNullOrEmpty(diff.Unit) ? Unit : diff.Unit,
            Location = diff.Location ?? Location,
            Models = models,
            Connectors = connectors,
            Authors = diff.Authors is not null && diff.Authors.Any() ? diff.Authors : Authors,
            Attributes = diff.Attributes is not null && diff.Attributes.Any() ? diff.Attributes : Attributes,
            Concepts = diff.Concepts is not null && diff.Concepts.Any() ? diff.Concepts : Concepts,
            Props = Props,
            CreatedAt = CreatedAt,
            UpdatedAt = DateTime.UtcNow
        };
    }

    private List<Model> ApplyModelsDiff(List<Model> original, ModelsDiff diff)
    {
        var result = original.Where(m => !diff.Removed.Any(r => r.Guid == m.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(m => m.Guid == updated.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = result[index].ApplyDiff(updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private List<Connector> ApplyConnectorsDiff(List<Connector> original, ConnectorsDiff diff)
    {
        var result = original.Where(p => !diff.Removed.Any(r => r.Guid == p.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(p => p.Guid == updated.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = result[index].ApplyDiff(updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    public TypeDiff CreateDiff()
    {
        return new TypeDiff
        {
            Guid = Guid,
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Stock = Stock,
            Virtual = Virtual,
            Uri = Uri,
            Unit = Unit,
            Location = Location,
            Models = new ModelsDiff { Added = new List<Model>(), Removed = new List<ModelId>(), Updated = Models.Select(m => new DiffUpdate<ModelDiff> { Id = m.Guid, Diff = m.CreateDiff() }).ToList() },
            Connectors = new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Updated = Connectors.Select(p => new DiffUpdate<ConnectorDiff> { Id = p.Guid, Diff = p.CreateDiff() }).ToList() },
            Authors = Authors,
            Attributes = Attributes,
            Concepts = Concepts
        };
    }

    public TypeDiff InverseDiff(TypeDiff appliedDiff)
    {
        return new TypeDiff
        {
            Name = !string.IsNullOrEmpty(appliedDiff.Name) ? Name : "",
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? Description : "",
            Icon = !string.IsNullOrEmpty(appliedDiff.Icon) ? Icon : "",
            Image = !string.IsNullOrEmpty(appliedDiff.Image) ? Image : "",
            Stock = appliedDiff.Stock.HasValue ? Stock : null,
            Virtual = appliedDiff.Virtual.HasValue ? Virtual : null,
            Uri = !string.IsNullOrEmpty(appliedDiff.Uri) ? Uri : "",
            Unit = !string.IsNullOrEmpty(appliedDiff.Unit) ? Unit : "",
            Location = appliedDiff.Location is not null ? Location : null,
            Models = appliedDiff.Models is not null ? new ModelsDiff { Added = new List<Model>(), Removed = new List<ModelId>(), Updated = Models.Select(m => new DiffUpdate<ModelDiff> { Id = m.Guid, Diff = m.CreateDiff() }).ToList() } : null,
            Connectors = appliedDiff.Connectors is not null ? new ConnectorsDiff { Added = new List<Connector>(), Removed = new List<ConnectorId>(), Updated = Connectors.Select(p => new DiffUpdate<ConnectorDiff> { Id = p.Guid, Diff = p.CreateDiff() }).ToList() } : null,
            Authors = appliedDiff.Authors is not null && appliedDiff.Authors.Any() ? Authors : null,
            Attributes = appliedDiff.Attributes is not null && appliedDiff.Attributes.Any() ? Attributes : null
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

    public bool IsSameAs(Type other)
    {
        return Name == other.Name;
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

#endregion Type

#region Layer


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
    public bool IsHidden { get; set; } = false;
    public bool IsLocked { get; set; } = false;
    public string Color { get; set; } = "";
    public string Description { get; set; } = "";
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Path}";
    public override string ToString() => $"Lyr({ToHumanIdString()})";
}

#endregion Layer

#region Group


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

#endregion Group

#region Piece


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
    public List<DiffUpdate<PieceDiff>> Updated { get; set; } = new();
    public List<Piece> Added { get; set; } = new();

    public PiecesDiff MergeDiff(PiecesDiff other)
    {
        return new PiecesDiff
        {
            Removed = other.Removed.Concat(Removed).Distinct().ToList(),
            Updated = other.Updated.Concat(Updated).GroupBy(m => m.Id).Select(g => g.Last()).ToList(),
            Added = other.Added.Concat(Added).GroupBy(a => a.Guid).Select(g => g.Last()).ToList()
        };
    }

    public static implicit operator PiecesDiff(List<Piece> pieces) => new() { Updated = pieces.Select(p => new DiffUpdate<PieceDiff> { Id = p.Guid, Diff = p.CreateDiff() }).ToList() };
}


public class PieceDiff : Entity<PieceDiff>
{
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public string? Description { get; set; }
    public TypeId? Type { get; set; }
    public DesignId? Design { get; set; }
    public Plane? Plane { get; set; }
    public Coord? Center { get; set; }
    public float? Scale { get; set; }
    public Plane? MirrorPlane { get; set; }
    public bool? IsHidden { get; set; }
    public bool? IsLocked { get; set; }
    public string? Color { get; set; }
    public List<Prop>? Props { get; set; }
    public List<Attribute>? Attributes { get; set; }

    public static implicit operator PieceDiff(PieceId id) => new() { Guid = id.Guid };
    public static implicit operator PieceDiff(Piece piece) => new() { Guid = piece.Guid, Name = piece.Name, Description = piece.Description, Type = piece.Type, Design = piece.Design, Plane = piece.Plane, Center = piece.Center, Scale = piece.Scale, MirrorPlane = piece.MirrorPlane, IsHidden = piece.IsHidden, IsLocked = piece.IsLocked, Color = piece.Color, Props = piece.Props, Attributes = piece.Attributes };
}





public class Piece : Entity<Piece>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public TypeId? Type { get; set; }
    public DesignId? Design { get; set; }
    public Plane? Plane { get; set; }
    public Coord? Center { get; set; }
    public float? Scale { get; set; }
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
    public static implicit operator Piece(PieceDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Description = diff.Description ?? "", Type = diff.Type, Design = diff.Design, Plane = diff.Plane, Center = diff.Center, Scale = diff.Scale, MirrorPlane = diff.MirrorPlane, IsHidden = diff.IsHidden, IsLocked = diff.IsLocked, Color = diff.Color, Props = diff.Props, Attributes = diff.Attributes ?? new() };

    public Piece ApplyDiff(PieceDiff diff)
    {
        return new Piece
        {
            Guid = diff.Guid ?? Guid,
            Name = diff.Name ?? Name,
            Description = diff.Description ?? Description,
            Type = diff.Type ?? Type,
            Design = diff.Design ?? Design,
            Plane = diff.Plane ?? Plane,
            Center = diff.Center ?? Center,
            Scale = diff.Scale ?? Scale,
            MirrorPlane = diff.MirrorPlane ?? MirrorPlane,
            IsHidden = diff.IsHidden ?? IsHidden,
            IsLocked = diff.IsLocked ?? IsLocked,
            Color = diff.Color ?? Color,
            Props = diff.Props ?? Props,
            Attributes = diff.Attributes ?? Attributes
        };
    }

    public PieceDiff CreateDiff()
    {
        return new PieceDiff
        {
            Guid = Guid,
            Name = Name,
            Description = Description,
            Type = Type,
            Design = Design,
            Plane = Plane,
            Center = Center,
            Scale = Scale,
            MirrorPlane = MirrorPlane,
            IsHidden = IsHidden,
            IsLocked = IsLocked,
            Color = Color,
            Props = Props,
            Attributes = Attributes
        };
    }
}

#endregion Piece
#region Side


public class SideDiff : Entity<SideDiff>
{
    public PieceId? Piece { get; set; }
    public PieceId? DesignPiece { get; set; } = null;
    public ConnectorId? Connector { get; set; }
    public string Description { get; set; } = "";

    public static implicit operator SideDiff(Side side) => new() { Piece = side.Piece, DesignPiece = side.DesignPiece, Connector = side.Connector };

    public SideDiff MergeDiff(SideDiff other)
    {
        return new SideDiff
        {
            Piece = other.Piece ?? Piece,
            DesignPiece = other.DesignPiece ?? DesignPiece,
            Connector = other.Connector ?? Connector,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description
        };
    }
}





public class Side : Entity<Side>
{
    public PieceId Piece { get; set; } = new();
    public PieceId? DesignPiece { get; set; } = null;
    public ConnectorId Connector { get; set; } = new();

    public static implicit operator Side(SideDiff diff) => new() { Piece = diff.Piece ?? new(), DesignPiece = diff.DesignPiece, Connector = diff.Connector ?? new() };

    public Side ApplyDiff(SideDiff diff)
    {
        return new Side
        {
            Piece = diff.Piece ?? Piece,
            DesignPiece = diff.DesignPiece ?? DesignPiece,
            Connector = diff.Connector ?? Connector
        };
    }

    public SideDiff CreateDiff()
    {
        return new SideDiff
        {
            Piece = Piece,
            DesignPiece = DesignPiece,
            Connector = Connector
        };
    }

    public SideDiff InverseDiff(SideDiff appliedDiff)
    {
        return new SideDiff
        {
            Piece = appliedDiff.Piece is not null ? Piece : null,
            DesignPiece = appliedDiff.DesignPiece is not null ? DesignPiece : null,
            Connector = appliedDiff.Connector is not null ? Connector : null
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

#endregion Side

#region Connection


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
    public SideDiff? Connected { get; set; }
    public SideDiff? Connecting { get; set; }
    public string Description { get; set; } = "";
    public float? Gap { get; set; }
    public float? Shift { get; set; }
    public float? Rise { get; set; }
    public float? Rotation { get; set; }
    public float? Turn { get; set; }
    public float? Tilt { get; set; }
    public float? U { get; set; }
    public float? V { get; set; }
    public List<Attribute>? Attributes { get; set; }

    public static implicit operator ConnectionDiff(ConnectionId id) => new() { Connected = new SideDiff { Piece = id.Connected.Piece, DesignPiece = id.Connected.DesignPiece, Connector = id.Connected.Connector }, Connecting = new SideDiff { Piece = id.Connecting.Piece, DesignPiece = id.Connecting.DesignPiece, Connector = id.Connecting.Connector } };
    public static implicit operator ConnectionDiff(Connection connection) => new() { Connected = connection.Connected.CreateDiff(), Connecting = connection.Connecting.CreateDiff(), Description = connection.Description, Gap = connection.Gap, Shift = connection.Shift, Rise = connection.Rise, Rotation = connection.Rotation, Turn = connection.Turn, Tilt = connection.Tilt, U = connection.U, V = connection.V, Attributes = connection.Attributes };

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
    public List<DiffUpdate<ConnectionDiff>> Updated { get; set; } = new();
    public List<Connection> Added { get; set; } = new();

    public static implicit operator ConnectionsDiff(List<Connection> connections) => new() { Updated = connections.Select(c => new DiffUpdate<ConnectionDiff> { Id = c.Guid, Diff = (ConnectionDiff)c }).ToList() };

    public ConnectionsDiff MergeDiff(ConnectionsDiff other)
    {
        return new ConnectionsDiff
        {
            Removed = other.Removed.Concat(Removed).Distinct().ToList(),
            Updated = other.Updated.Concat(Updated).GroupBy(u => u.Id).Select(g => g.Last()).ToList(),
            Added = other.Added.Concat(Added).GroupBy(a => a.Connected.Piece.Guid + "--" + a.Connecting.Piece.Guid).Select(g => g.Last()).ToList()
        };
    }
}





public class Connection : Entity<Connection>
{
    public string Guid { get; set; } = "";
    public Side Connected { get; set; } = new();
    public Side Connecting { get; set; } = new();
    public string Description { get; set; } = "";
    public float Gap { get; set; } = 0;
    public float Shift { get; set; } = 0;
    public float Rise { get; set; } = 0;
    public float Rotation { get; set; } = 0;
    public float Turn { get; set; } = 0;
    public float Tilt { get; set; } = 0;
    public float? U { get; set; }
    public float? V { get; set; }
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Connected.Piece.Guid + (Connected.Connector.Guid != "" ? ":" + Connected.Connector.Guid : "")}--{(Connecting.Connector.Guid != "" ? Connecting.Connector.Guid + ":" : "") + Connecting.Piece.Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Con({ToHumanIdString()})";

    public static implicit operator Connection(ConnectionId id) => new() { Connected = id.Connected, Connecting = id.Connecting };
    public static implicit operator Connection(ConnectionDiff diff) => new() { Connected = diff.Connected ?? new(), Connecting = diff.Connecting ?? new(), Description = diff.Description ?? "", Gap = diff.Gap ?? 0, Shift = diff.Shift ?? 0, Rise = diff.Rise ?? 0, Rotation = diff.Rotation ?? 0, Turn = diff.Turn ?? 0, Tilt = diff.Tilt ?? 0, U = diff.U, V = diff.V, Attributes = diff.Attributes ?? new() };

    public Connection ApplyDiff(ConnectionDiff diff)
    {
        return new Connection
        {
            Connected = diff.Connected is not null ? Connected.ApplyDiff(diff.Connected) : Connected,
            Connecting = diff.Connecting is not null ? Connecting.ApplyDiff(diff.Connecting) : Connecting,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Gap = diff.Gap ?? Gap,
            Shift = diff.Shift ?? Shift,
            Rise = diff.Rise ?? Rise,
            Rotation = diff.Rotation ?? Rotation,
            Turn = diff.Turn ?? Turn,
            Tilt = diff.Tilt ?? Tilt,
            U = diff.U ?? U,
            V = diff.V ?? V,
            Attributes = diff.Attributes ?? Attributes
        };
    }

    public ConnectionDiff CreateDiff()
    {
        return new ConnectionDiff
        {
            Connected = Connected.CreateDiff(),
            Connecting = Connecting.CreateDiff(),
            Description = Description,
            Gap = Gap,
            Shift = Shift,
            Rise = Rise,
            Rotation = Rotation,
            Turn = Turn,
            Tilt = Tilt,
            U = U,
            V = V,
            Attributes = Attributes
        };
    }

    public ConnectionDiff InverseDiff(ConnectionDiff appliedDiff)
    {
        return new ConnectionDiff
        {
            Connected = appliedDiff.Connected is not null ? Connected.CreateDiff() : null,
            Connecting = appliedDiff.Connecting is not null ? Connecting.CreateDiff() : null,
            Description = appliedDiff.Description is not null ? Description : "",
            Gap = appliedDiff.Gap.HasValue ? Gap : null,
            Shift = appliedDiff.Shift.HasValue ? Shift : null,
            Rise = appliedDiff.Rise.HasValue ? Rise : null,
            Rotation = appliedDiff.Rotation.HasValue ? Rotation : null,
            Turn = appliedDiff.Turn.HasValue ? Turn : null,
            Tilt = appliedDiff.Tilt.HasValue ? Tilt : null,
            U = appliedDiff.U.HasValue ? U : null,
            V = appliedDiff.V.HasValue ? V : null,
            Attributes = appliedDiff.Attributes is not null ? Attributes : null
        };
    }

    public bool IsSameAs(Connection other, bool strict = false)
    {
        if (other is null) return false;
        if (strict)
        {
            return Connected.Piece.Guid == other.Connected.Piece.Guid &&
                   Connected.Connector.Guid == other.Connected.Connector.Guid &&
                   Connecting.Piece.Guid == other.Connecting.Piece.Guid &&
                   Connecting.Connector.Guid == other.Connecting.Connector.Guid;
        }
        return (Connected.Piece.Guid == other.Connected.Piece.Guid && Connecting.Piece.Guid == other.Connecting.Piece.Guid) ||
               (Connected.Piece.Guid == other.Connecting.Piece.Guid && Connecting.Piece.Guid == other.Connected.Piece.Guid);
    }

    public Connection SetAttribute(Attribute attribute)
    {
        var attributes = new List<Attribute>(Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Connection
        {
            Connected = Connected,
            Connecting = Connecting,
            Description = Description,
            Gap = Gap,
            Shift = Shift,
            Rise = Rise,
            Rotation = Rotation,
            Turn = Turn,
            Tilt = Tilt,
            U = U,
            V = V,
            Attributes = attributes
        };
    }
}

#endregion Connection

#region Stat


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
    public float? Min { get; set; }
    public bool? MinExcluded { get; set; }
    public float? Max { get; set; }
    public bool? MaxExcluded { get; set; }

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Stt({ToHumanIdString()})";
}

#endregion Stat

#region Design


public class DesignsDiff : Entity<DesignsDiff>
{
    public List<DesignId> Removed { get; set; } = new();
    public List<DiffUpdate<DesignDiff>> Updated { get; set; } = new();
    public List<Design> Added { get; set; } = new();

    public static implicit operator DesignsDiff(List<Design> designs) => new() { Updated = designs.Select(d => new DiffUpdate<DesignDiff> { Id = d.Guid, Diff = (DesignDiff)d }).ToList() };
}


public class DesignDiff : Entity<DesignDiff>
{
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public DesignId? Parent { get; set; }
    public bool? IsAbstract { get; set; }
    public string? Folder { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public Location? Location { get; set; }
    public string? Unit { get; set; }
    public bool? CanScale { get; set; }
    public bool? CanMirror { get; set; }
    public string? ActiveLayer { get; set; }
    public PiecesDiff? Pieces { get; set; }
    public ConnectionsDiff? Connections { get; set; }
    public List<Prop>? Props { get; set; }
    public List<Stat>? Stats { get; set; }
    public List<Layer>? Layers { get; set; }
    public List<Group>? Groups { get; set; }
    public List<AuthorId>? Authors { get; set; }
    public List<ConceptId>? Concepts { get; set; }
    public List<Attribute>? Attributes { get; set; }
    public DateTime? CreatedAt { get; set; }
    public DateTime? UpdatedAt { get; set; }

    public static implicit operator DesignDiff(DesignId id) => new() { Guid = id.Guid };
    public static implicit operator DesignDiff(Design design) => new() { Guid = design.Guid, Name = design.Name, Parent = design.Parent, IsAbstract = design.IsAbstract, Folder = design.Folder, Description = design.Description, Icon = design.Icon, Image = design.Image, Location = design.Location, Unit = design.Unit, CanScale = design.CanScale, CanMirror = design.CanMirror, ActiveLayer = design.ActiveLayer, Pieces = new PiecesDiff { Removed = new List<PieceId>(), Updated = design.Pieces.Select(p => new DiffUpdate<PieceDiff> { Id = p.Guid, Diff = p.CreateDiff() }).ToList(), Added = new List<Piece>() }, Connections = new ConnectionsDiff { Removed = new List<ConnectionId>(), Updated = design.Connections.Select(c => new DiffUpdate<ConnectionDiff> { Id = c.Guid, Diff = c.CreateDiff() }).ToList(), Added = new List<Connection>() }, Props = design.Props, Stats = design.Stats, Layers = design.Layers, Groups = design.Groups, Authors = design.Authors, Concepts = design.Concepts, Attributes = design.Attributes, CreatedAt = design.CreatedAt, UpdatedAt = design.UpdatedAt };

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
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Image { get; set; } = "";
    public List<ConceptId> Concepts { get; set; } = new();
    public List<AuthorId> Authors { get; set; } = new();
    public Location? Location { get; set; }
    public string Unit { get; set; } = "";
    public bool? CanScale { get; set; }
    public bool? CanMirror { get; set; }
    public List<Layer> Layers { get; set; } = new();
    public string? ActiveLayer { get; set; }
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
    public static implicit operator Design(DesignDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Parent = diff.Parent, IsAbstract = diff.IsAbstract, Folder = diff.Folder, Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Location = diff.Location, Unit = diff.Unit ?? "", CanScale = diff.CanScale, CanMirror = diff.CanMirror, ActiveLayer = diff.ActiveLayer, Attributes = diff.Attributes ?? new(), Authors = diff.Authors ?? new(), Concepts = diff.Concepts ?? new(), CreatedAt = diff.CreatedAt ?? DateTime.UtcNow, UpdatedAt = diff.UpdatedAt ?? DateTime.UtcNow };
    public static implicit operator string(Design design) => design.Name;
    public static implicit operator Design(string name) => new() { Name = name, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };

    public Design ApplyDiff(DesignDiff diff)
    {
        var pieces = Pieces;
        var connections = Connections;

        if (diff.Pieces is not null)
        {
            pieces = ApplyPiecesDiff(Pieces, diff.Pieces);
        }
        if (diff.Connections is not null)
        {
            connections = ApplyConnectionsDiff(Connections, diff.Connections);
        }

        return new Design
        {
            Guid = diff.Guid ?? Guid,
            Name = diff.Name ?? Name,
            Parent = diff.Parent ?? Parent,
            IsAbstract = diff.IsAbstract ?? IsAbstract,
            Folder = diff.Folder ?? Folder,
            Description = diff.Description ?? Description,
            Icon = diff.Icon ?? Icon,
            Image = diff.Image ?? Image,
            Location = diff.Location ?? Location,
            Unit = diff.Unit ?? Unit,
            ActiveLayer = diff.ActiveLayer ?? ActiveLayer,
            Pieces = pieces,
            Connections = connections,
            Props = diff.Props ?? Props,
            Stats = diff.Stats ?? Stats,
            Layers = diff.Layers ?? Layers,
            Groups = diff.Groups ?? Groups,
            CanScale = diff.CanScale ?? CanScale,
            CanMirror = diff.CanMirror ?? CanMirror,
            Attributes = diff.Attributes ?? Attributes,
            Authors = diff.Authors ?? Authors,
            Concepts = diff.Concepts ?? Concepts,
            CreatedAt = diff.CreatedAt ?? CreatedAt,
            UpdatedAt = diff.UpdatedAt ?? UpdatedAt
        };
    }

    public DesignDiff CreateDiff()
    {
        return new DesignDiff
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Location = Location,
            Unit = Unit,
            Pieces = new PiecesDiff
            {
                Removed = new List<PieceId>(),
                Updated = Pieces.Select(p => new DiffUpdate<PieceDiff> { Id = p.Guid, Diff = p.CreateDiff() }).ToList(),
                Added = new List<Piece>()
            },
            Connections = new ConnectionsDiff
            {
                Removed = new List<ConnectionId>(),
                Updated = Connections.Select(c => new DiffUpdate<ConnectionDiff> { Id = c.Guid, Diff = c.CreateDiff() }).ToList(),
                Added = new List<Connection>()
            },
            Stats = Stats,
            Authors = Authors,
            Attributes = Attributes,
            Concepts = Concepts
        };
    }

    private List<Piece> ApplyPiecesDiff(List<Piece> original, PiecesDiff diff)
    {
        var result = original.Where(p => !diff.Removed.Any(r => r.Guid == p.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(p => p.Guid == updated.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = result[index].ApplyDiff(updated.Diff);
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

    private PiecesDiff CreatePiecesDiff(List<Piece> original, List<Piece> modified)
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
                    var diff = p.CreateDiff();
                    return !Equals(p, modifiedPiece) ? new[] { new DiffUpdate<PieceDiff> { Id = p.Guid, Diff = diff } } : Array.Empty<DiffUpdate<PieceDiff>>();
                })
                .ToList(),
            Added = modified.Where(p => !originalIds.Contains(p.Guid)).ToList()
        };
    }

    private List<Connection> ApplyConnectionsDiff(List<Connection> original, ConnectionsDiff diff)
    {
        var result = original.Where(c => !diff.Removed.Any(r => r.Guid == c.Guid)).ToList();

        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(c => c.Guid == updated.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = result[index].ApplyDiff(updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private ConnectionsDiff CreateConnectionsDiff(List<Connection> original, List<Connection> modified)
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
                    var diff = c.CreateDiff();
                    return !Equals(c, modifiedConnection) ? new[] { new DiffUpdate<ConnectionDiff> { Id = c.Guid, Diff = diff } } : Array.Empty<DiffUpdate<ConnectionDiff>>();
                })
                .ToList(),
            Added = modified.Where(c => !originalGuids.Contains(c.Guid)).ToList()
        };
    }

    public void Bfs(Action<Piece> onRoot, Action<Piece, Piece, Connection> onConnection)
    {
        var pieces = Pieces.ToDictionary(p => p.Guid);
        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var piece in Pieces)
            graph.AddVertex(piece.Guid);
        foreach (var connection in Connections)
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
            foreach (var connection in Connections)
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
                var connection = Connections.First(c =>
                    (c.Connected.Piece.Guid == parent.Guid && c.Connecting.Piece.Guid == child.Guid) ||
                    (c.Connected.Piece.Guid == child.Guid && c.Connecting.Piece.Guid == parent.Guid));
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
            var connectors = new Dictionary<string, Dictionary<string, Connector>>();
            foreach (var type in types)
            {
                if (!connectors.ContainsKey(type.Guid))
                    connectors[type.Guid] = new Dictionary<string, Connector>();
                foreach (var connector in type.Connectors)
                    connectors[type.Guid][connector.Guid] = connector;
            }

            foreach (var piece in Pieces)
            {
                if (piece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({piece.Guid}) has no type.");
                if (!types.Any(t => t.Guid == piece.Type.Guid))
                    throw new Exception(
                        $"The type {piece.Type.ToHumanIdString()} of the piece {piece.ToHumanIdString()} is not provided.");
            }
            foreach (var connection in Connections)
            {
                var connectedPiece = Pieces.First(p => p.Guid == connection.Connected.Piece.Guid);
                if (connectedPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectedPiece.Guid}) has no type.");
                var connectedType = types.First(t => t.Guid == connectedPiece.Type.Guid);
                if (!connectors[connectedType.Guid].ContainsKey(connection.Connected.Connector.Guid))
                    throw new Exception(
                        $"The type {connectedType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the connector {connection.Connected.Connector.Guid}.");
                var connectingPiece = Pieces.First(p => p.Guid == connection.Connecting.Piece.Guid);
                if (connectingPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectingPiece.Guid}) has no type.");
                var connectingType = types.First(t => t.Guid == connectingPiece.Type.Guid);
                if (!connectors[connectingType.Guid].ContainsKey(connection.Connecting.Connector.Guid))
                    throw new Exception(
                        $"The type {connectingType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the connector {connection.Connecting.Connector.Guid}.");
            }

            var onRoot = new Action<Piece>(piece =>
            {
                if (piece.Plane is null) piece.Plane = new Plane();
                if (piece.Center is null) piece.Center = new Coord();
            });
            var onConnection = new Action<Piece, Piece, Connection>((parent, child, connection) =>
            {
                var isParentConnected = connection.Connected.Piece.Guid == parent.Guid;
                var parentPlane = parent.Plane;
                if (parentPlane is null || parent.Type is null || child.Type is null) return;
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
                child.Plane = childPlane;

                var direction = new Coord
                {
                    U = connection.U ?? 0,
                    V = connection.V ?? 0
                }.Normalize();
                var childCenter = new Coord
                {
                    U = parent.Center!.U + (connection.U ?? 0) + direction.U,
                    V = parent.Center!.V + (connection.V ?? 0) + direction.V
                };
                child.Center = childCenter;
                var semioAttribute = child.Attributes.FirstOrDefault(q => q.Key == "semio.parent");
                if (semioAttribute is not null)
                {
                    semioAttribute.Value = parent.Guid;
                }
                else
                {
                    child.Attributes.Add(new Attribute
                    {
                        Key = "semio.parent",
                        Value = parent.Guid
                    });
                }
            });
            Bfs(onRoot, onConnection);
        }

        Connections = new List<Connection>();

        return this;
    }

    public Design Flatten(IEnumerable<Type> types) => Flatten(types, DefaultComputeChildPlane);

    public static Plane DefaultComputeChildPlane(
        Plane parentPlane,
        Point parentPoint,
        Vector parentDirection,
        Point childPoint,
        Vector childDirection,
        float gap,
        float shift,
        float rise,
        float rotation,
        float turn,
        float tilt)
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

        var directionT = System.Numerics.Matrix4x4.CreateFromQuaternion(alignQuat);

        var yAxis = System.Numerics.Vector3.UnitY;
        var parentConnectorQuat = CreateFromTwoVectors(yAxis, pDir);
        var parentRotationT = System.Numerics.Matrix4x4.CreateFromQuaternion(parentConnectorQuat);

        var gapDirection = System.Numerics.Vector3.Transform(System.Numerics.Vector3.UnitY, parentRotationT);
        var shiftDirection = System.Numerics.Vector3.Transform(System.Numerics.Vector3.UnitX, parentRotationT);
        var raiseDirection = System.Numerics.Vector3.Transform(System.Numerics.Vector3.UnitZ, parentRotationT);
        var turnAxis = System.Numerics.Vector3.Transform(System.Numerics.Vector3.UnitZ, parentRotationT);
        var tiltAxis = System.Numerics.Vector3.Transform(System.Numerics.Vector3.UnitX, parentRotationT);

        var orientationT = directionT;
        var rotateT = System.Numerics.Matrix4x4.CreateFromAxisAngle(pDir, -rotationRad);
        orientationT = rotateT * orientationT;

        turnAxis = System.Numerics.Vector3.Transform(turnAxis, rotateT);
        tiltAxis = System.Numerics.Vector3.Transform(tiltAxis, rotateT);

        var turnT = System.Numerics.Matrix4x4.CreateFromAxisAngle(turnAxis, turnRad);
        orientationT = turnT * orientationT;

        var tiltT = System.Numerics.Matrix4x4.CreateFromAxisAngle(tiltAxis, tiltRad);
        orientationT = tiltT * orientationT;

        var centerChildT = System.Numerics.Matrix4x4.CreateTranslation(-cPoint);

        var transform = orientationT * centerChildT;

        var translationVec = (gapDirection * gap) + (shiftDirection * shift) + (raiseDirection * rise);
        var translationT = System.Numerics.Matrix4x4.CreateTranslation(translationVec);

        transform = translationT * transform;

        var moveToParentT = System.Numerics.Matrix4x4.CreateTranslation(pPoint);
        transform = moveToParentT * transform;

        var finalMatrix = pMatrix * transform;

        return MatrixToPlane(finalMatrix);
    }

    private static float DegreesToRadians(float deg) => deg * (float)Math.PI / 180f;

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

        // Use strict column/row mapping based on testing. Reference was: x, y, z, origin for rows 1,2,3,4
        return new System.Numerics.Matrix4x4(
            x.X, x.Y, x.Z, 0,
            y.X, y.Y, y.Z, 0,
            z.X, z.Y, z.Z, 0,
            origin.X, origin.Y, origin.Z, 1
        );
    }

    private static Plane MatrixToPlane(System.Numerics.Matrix4x4 m)
    {
        var x = new System.Numerics.Vector3(m.M11, m.M12, m.M13);
        var y = new System.Numerics.Vector3(m.M21, m.M22, m.M23);
        var origin = new System.Numerics.Vector3(m.M41, m.M42, m.M43);

        return new Plane
        {
            Origin = new Point { X = origin.X, Y = origin.Y, Z = origin.Z },
            XAxis = new Vector { X = x.X, Y = x.Y, Z = x.Z },
            YAxis = new Vector { X = y.X, Y = y.Y, Z = y.Z }
        };
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
                if (connection.Connected.Piece.Guid != parent.Guid)
                {
                    connection.Connected.Piece = new PieceId { Guid = child.Guid };
                    connection.Connecting.Piece = new PieceId { Guid = parent.Guid };
                }

                sortedConnections.Add(connection);
            });

        Pieces = sortedPieces;
        Connections = sortedConnections;

        return this;
    }

    public Piece? Piece(string guid) => Pieces.Find(piece => piece.Guid == guid);
    private Design FlatToSvgCoordinates(float iconWidth, float iconWidthMax, float margin)
    {

        foreach (var piece in Pieces)
        {
            if (piece.Center is null) continue;
            piece.Center.U = piece.Center.U * iconWidth;
            piece.Center.V = -(piece.Center.V * iconWidth);
        }

        foreach (var connection in Connections)
        {
            if (connection.U.HasValue) connection.U = connection.U * iconWidth;
            if (connection.V.HasValue) connection.V = -(connection.V * iconWidth);
        }


        var maxIconOffset = iconWidthMax - iconWidth;
        var minX = Pieces.Where(p => p.Center is not null).Min(piece => piece.Center!.U) - (margin + maxIconOffset);
        var minY = Pieces.Where(p => p.Center is not null).Min(piece => piece.Center!.V) - (margin + maxIconOffset);
        var minXSign = Math.Sign(minX);
        var minYSign = Math.Sign(minY);
        var offsetX = minXSign == 0 ? 0 : -minX;
        var offsetY = minYSign == 0 ? 0 : -minY;
        foreach (var piece in Pieces)
        {
            if (piece.Center is null) continue;
            piece.Center.U += offsetX;
            piece.Center.V += offsetY;
        }

        return this;
    }




    public string Diagram(
        IEnumerable<Type> types,
        Func<Plane, Point, Vector, Point, Vector, float, float, float, float, float, float, Plane> computeChildPlane,
        string kitDirectory = "",
        float iconWidth = 48, float iconStroke = 1f, float connectionStroke = 2f, float margin = 0)
    {
        var typesDict = Type.EnumerableToDict(types);

        var usedTypes = new List<Type>();
        foreach (var type in types)
            if (Pieces.Exists(piece => piece.Type is not null && piece.Type.Guid == type.Guid))
                usedTypes.Add(type);

        var flatCloneInSvgCoordinates = DeepClone()!.Flatten(types, computeChildPlane)
            .FlatToSvgCoordinates(iconWidth, iconWidth + 2 * iconStroke, margin);

        var svgDoc = new SvgDocument
        {
            Width = flatCloneInSvgCoordinates.Pieces.Where(p => p.Center is not null).Max(piece => piece.Center!.U) + margin * 2 + iconWidth +
                    2 * iconStroke,
            Height = flatCloneInSvgCoordinates.Pieces.Where(p => p.Center is not null).Max(piece => piece.Center!.V) + margin * 2 + iconWidth +
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

        foreach (var connection in Connections)
        {
            var connectedPieceFlat = flatCloneInSvgCoordinates.Piece(connection.Connected.Piece.Guid);
            var connectingPieceFlat = flatCloneInSvgCoordinates.Piece(connection.Connecting.Piece.Guid);
            if (connectedPieceFlat?.Center is null || connectingPieceFlat?.Center is null) continue;
            var connectionLine = new SvgLine
            {
                StartX = connectedPieceFlat.Center.U + iconWidth / 2,
                StartY = connectedPieceFlat.Center.V + iconWidth / 2,
                EndX = connectingPieceFlat.Center.U + iconWidth / 2,
                EndY = connectingPieceFlat.Center.V + iconWidth / 2,
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
            var flatPiece = flatCloneInSvgCoordinates.Piece(piece.Guid);
            if (piece.Center is not null && flatPiece?.Center is not null)
            {
                var rootPiece = new SvgUse
                {
                    CustomAttributes = { { "href", "#root" } },
                    X = flatPiece.Center.U,
                    Y = flatPiece.Center.V
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
                    X = flatPiece.Center.U,
                    Y = flatPiece.Center.V,
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

    public bool IsSameAs(Design other)
    {
        if (other is null) return false;
        return Name == other.Name;
    }

    public Piece FindPiece(string pieceGuid)
    {
        var piece = Pieces.FirstOrDefault(p => p.Guid == pieceGuid);
        if (piece is null) throw new ArgumentException($"Piece {pieceGuid} not found in design");
        return piece;
    }

    public Connection FindConnection(Connection connectionToFind, bool strict = false)
    {
        var connection = Connections.FirstOrDefault(c => c.IsSameAs(connectionToFind, strict));
        if (connection is null)
            throw new ArgumentException($"Connection {connectionToFind.Connected.Piece.Guid} -> {connectionToFind.Connecting.Piece.Guid} not found in design");
        return connection;
    }

    public List<Connection> FindPieceConnections(string pieceGuid)
    {
        return Connections.Where(c =>
            c.Connected.Piece.Guid == pieceGuid ||
            c.Connecting.Piece.Guid == pieceGuid).ToList();
    }

    public Design AddPiece(Piece piece)
    {
        var newPieces = new List<Piece>(Pieces) { piece };
        return new Design
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Location = Location,
            Unit = Unit,
            Pieces = newPieces,
            Connections = new List<Connection>(Connections),
            Props = new List<Prop>(Props),
            Stats = new List<Stat>(Stats),
            Authors = new List<AuthorId>(Authors),
            Attributes = new List<Attribute>(Attributes)
        };
    }

    public Design RemovePiece(string pieceGuid)
    {
        var newPieces = Pieces.Where(p => p.Guid != pieceGuid).ToList();
        var newConnections = Connections.Where(c =>
            c.Connected.Piece.Guid != pieceGuid &&
            c.Connecting.Piece.Guid != pieceGuid).ToList();
        return new Design
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Location = Location,
            Unit = Unit,
            Pieces = newPieces,
            Connections = newConnections,
            Props = new List<Prop>(Props),
            Stats = new List<Stat>(Stats),
            Authors = new List<AuthorId>(Authors),
            Attributes = new List<Attribute>(Attributes)
        };
    }

    public Design AddConnection(Connection connection)
    {
        var newConnections = new List<Connection>(Connections) { connection };
        return new Design
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Location = Location,
            Unit = Unit,
            Pieces = new List<Piece>(Pieces),
            Connections = newConnections,
            Props = new List<Prop>(Props),
            Stats = new List<Stat>(Stats),
            Authors = new List<AuthorId>(Authors),
            Attributes = new List<Attribute>(Attributes)
        };
    }

    public Design RemoveConnection(Connection connectionToRemove)
    {
        var newConnections = Connections.Where(c => !c.IsSameAs(connectionToRemove)).ToList();
        return new Design
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Location = Location,
            Unit = Unit,
            Pieces = new List<Piece>(Pieces),
            Connections = newConnections,
            Props = new List<Prop>(Props),
            Stats = new List<Stat>(Stats),
            Authors = new List<AuthorId>(Authors),
            Attributes = new List<Attribute>(Attributes)
        };
    }

    public string FindAttributeValue(string key, string defaultValue = "")
    {
        var attribute = Attributes.FirstOrDefault(a => a.Key == key);
        return attribute?.Value ?? defaultValue;
    }

    public Design SetAttribute(Attribute attribute)
    {
        var newAttributes = Attributes.Where(a => a.Key != attribute.Key).ToList();
        newAttributes.Add(attribute);
        return new Design
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Location = Location,
            Unit = Unit,
            Pieces = new List<Piece>(Pieces),
            Connections = new List<Connection>(Connections),
            Props = new List<Prop>(Props),
            Stats = new List<Stat>(Stats),
            Authors = new List<AuthorId>(Authors),
            Attributes = newAttributes
        };
    }
}

#endregion Design

#region Kit


public class KitDiff : Entity<KitDiff>
{
    public string? Guid { get; set; }
    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Icon { get; set; }
    public string? Image { get; set; }
    public string? Preview { get; set; }
    public string? Version { get; set; }
    public string? Remote { get; set; }
    public string? Homepage { get; set; }
    public string? License { get; set; }
    public TypesDiff? Types { get; set; }
    public DesignsDiff? Designs { get; set; }
    public FilesDiff? Files { get; set; }
    public FoldersDiff? Folders { get; set; }
    public PortsDiff? Ports { get; set; }
    public AuthorsDiff? Authors { get; set; }
    public AttributesDiff? Attributes { get; set; }
    public ConceptsDiff? Concepts { get; set; }
    public string? CreatedAt { get; set; }
    public string? UpdatedAt { get; set; }

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
        Concepts = new ConceptsDiff { Added = kit.Concepts, Removed = new List<ConceptId>(), Updated = new List<DiffUpdate<ConceptDiff>>() },
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
    public List<DiffUpdate<KitDiff>> Updated { get; set; } = new();
    public List<Kit> Added { get; set; } = new();

    public static implicit operator KitsDiff(List<Kit> kits) => new() { Updated = kits.Select(k => new DiffUpdate<KitDiff> { Id = k.Guid, Diff = (KitDiff)k }).ToList() };
}





public class Kit : Entity<Kit>
{
    public string Guid { get; set; } = "";
    public string Name { get; set; } = "";
    public string Version { get; set; } = "";
    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Image { get; set; } = "";
    public List<Concept> Concepts { get; set; } = new();
    public List<Tag> Tags { get; set; } = new();
    public string Remote { get; set; } = "";
    public string Homepage { get; set; } = "";
    public string License { get; set; } = "";
    public List<Author> Authors { get; set; } = new();
    public List<Piece> Pieces { get; set; } = new();
    public List<Group> Groups { get; set; } = new();
    public List<Connection> Connections { get; set; } = new();
    public List<Prop> Props { get; set; } = new();
    public List<Stat> Stats { get; set; } = new();
    public List<Attribute> Attributes { get; set; } = new();
    public string Preview { get; set; } = "";
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

    public Kit ApplyDiff(KitDiff diff)
    {
        var types = Types;
        var designs = Designs;
        var files = Files;
        var attributes = Attributes;

        if (diff.Types is not null)
        {
            types = ApplyTypesDiff(Types, diff.Types);
        }
        if (diff.Designs is not null)
        {
            designs = ApplyDesignsDiff(Designs, diff.Designs);
        }
        if (diff.Files is not null)
        {
            files = ApplyFilesDiff(Files, diff.Files);
        }
        if (diff.Attributes is not null)
        {
            attributes = ApplyAttributesDiff(Attributes, diff.Attributes);
        }

        return new Kit
        {
            Name = string.IsNullOrEmpty(diff.Name) ? Name : diff.Name,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Icon = string.IsNullOrEmpty(diff.Icon) ? Icon : diff.Icon,
            Image = string.IsNullOrEmpty(diff.Image) ? Image : diff.Image,
            Preview = string.IsNullOrEmpty(diff.Preview) ? Preview : diff.Preview,
            Version = string.IsNullOrEmpty(diff.Version) ? Version : diff.Version,
            Remote = string.IsNullOrEmpty(diff.Remote) ? Remote : diff.Remote,
            Homepage = string.IsNullOrEmpty(diff.Homepage) ? Homepage : diff.Homepage,
            License = string.IsNullOrEmpty(diff.License) ? License : diff.License,
            Authors = Authors,
            Qualities = Qualities,
            Files = files,
            Types = types,
            Designs = designs,
            Attributes = attributes
        };
    }

    private List<Attribute> ApplyAttributesDiff(List<Attribute> original, AttributesDiff diff)
    {
        var result = original.Where(a => !diff.Removed.Any(r => r.Guid == a.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(a => a.Guid == updated.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = result[index].ApplyDiff(updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    public KitDiff CreateDiff()
    {
        return new KitDiff
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Preview = Preview,
            Version = Version,
            Remote = Remote,
            Homepage = Homepage,
            License = License,
            Types = new TypesDiff
            {
                Removed = new List<TypeId>(),
                Updated = Types.Select(t => new DiffUpdate<TypeDiff> { Id = t.Guid, Diff = t.CreateDiff() }).ToList(),
                Added = new List<Type>()
            },
            Designs = new DesignsDiff
            {
                Removed = new List<DesignId>(),
                Updated = Designs.Select(d => new DiffUpdate<DesignDiff> { Id = d.Guid, Diff = d.CreateDiff() }).ToList(),
                Added = new List<Design>()
            },
            Files = new FilesDiff
            {
                Removed = new List<FileId>(),
                Updated = Files.Select(f => new DiffUpdate<FileDiff> { Id = f.Guid, Diff = (FileDiff)f }).ToList(),
                Added = new List<File>()
            },
            Attributes = Attributes
        };
    }

    private List<Type> ApplyTypesDiff(List<Type> original, TypesDiff diff)
    {
        var result = original.Where(t => !diff.Removed.Any(r => r.Guid == t.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(t => t.Guid == updated.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = result[index].ApplyDiff(updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private TypesDiff CreateTypesDiff(List<Type> original, List<Type> modified)
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
                    var diff = t.CreateDiff();
                    return !Equals(t, modifiedType) ? new[] { new DiffUpdate<TypeDiff> { Id = t.Guid, Diff = diff } } : Array.Empty<DiffUpdate<TypeDiff>>();
                })
                .ToList(),
            Added = modified.Where(t => !originalGuids.Contains(t.Guid)).ToList()
        };
    }

    private List<Design> ApplyDesignsDiff(List<Design> original, DesignsDiff diff)
    {
        var result = original.Where(d => !diff.Removed.Any(r => r.Guid == d.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(d => d.Guid == updated.Id);
            if (index >= 0 && updated.Diff != null)
                result[index] = result[index].ApplyDiff(updated.Diff);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private DesignsDiff CreateDesignsDiff(List<Design> original, List<Design> modified)
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
                    var diff = d.CreateDiff();
                    return !Equals(d, modifiedDesign) ? new[] { new DiffUpdate<DesignDiff> { Id = d.Guid, Diff = diff } } : Array.Empty<DiffUpdate<DesignDiff>>();
                })
                .ToList(),
            Added = modified.Where(d => !originalGuids.Contains(d.Guid)).ToList()
        };
    }

    private List<File> ApplyFilesDiff(List<File> original, FilesDiff diff)
    {
        var result = original.Where(f => !diff.Removed.Any(r => r.Guid == f.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(f => f.Guid == updated.Id);
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

    public bool IsSameAs(Kit other)
    {
        if (other is null) return false;
        return Name == other.Name;
    }

    public Type FindType(string typeName)
    {
        var type = Types.FirstOrDefault(t => t.Name == typeName);
        if (type is null) throw new ArgumentException($"Type {typeName} not found in kit {Name}");
        return type;
    }

    public Design FindDesign(string designName)
    {
        var design = Designs.FirstOrDefault(d => d.Name == designName);
        if (design is null) throw new ArgumentException($"Design {designName} not found in kit {Name}");
        return design;
    }

    public Kit AddType(Type type)
    {
        var newTypes = new List<Type>(Types) { type };
        return new Kit
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Preview = Preview,
            Version = Version,
            Remote = Remote,
            Homepage = Homepage,
            License = License,
            Types = newTypes,
            Designs = new List<Design>(Designs),
            Authors = new List<Author>(Authors),
            Qualities = new List<Quality>(Qualities),
            Attributes = new List<Attribute>(Attributes)
        };
    }

    public Kit RemoveType(string typeName)
    {
        var newTypes = Types.Where(t => t.Name != typeName).ToList();
        return new Kit
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Preview = Preview,
            Version = Version,
            Remote = Remote,
            Homepage = Homepage,
            License = License,
            Types = newTypes,
            Designs = new List<Design>(Designs),
            Authors = new List<Author>(Authors),
            Qualities = new List<Quality>(Qualities),
            Attributes = new List<Attribute>(Attributes)
        };
    }

    public Kit AddDesign(Design design)
    {
        var newDesigns = new List<Design>(Designs) { design };
        return new Kit
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Preview = Preview,
            Version = Version,
            Remote = Remote,
            Homepage = Homepage,
            License = License,
            Types = new List<Type>(Types),
            Designs = newDesigns,
            Authors = new List<Author>(Authors),
            Qualities = new List<Quality>(Qualities),
            Attributes = new List<Attribute>(Attributes)
        };
    }

    public Kit RemoveDesign(string designName)
    {
        var newDesigns = Designs.Where(d => d.Name != designName).ToList();
        return new Kit
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Preview = Preview,
            Version = Version,
            Remote = Remote,
            Homepage = Homepage,
            License = License,
            Types = new List<Type>(Types),
            Designs = newDesigns,
            Authors = new List<Author>(Authors),
            Qualities = new List<Quality>(Qualities),
            Attributes = new List<Attribute>(Attributes)
        };
    }

    public string FindAttributeValue(string key, string defaultValue = "")
    {
        var attribute = Attributes.FirstOrDefault(a => a.Key == key);
        return attribute?.Value ?? defaultValue;
    }

    public Kit SetAttribute(Attribute attribute)
    {
        var newAttributes = Attributes.Where(a => a.Key != attribute.Key).ToList();
        newAttributes.Add(attribute);
        return new Kit
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Preview = Preview,
            Version = Version,
            Remote = Remote,
            Homepage = Homepage,
            License = License,
            Types = new List<Type>(Types),
            Designs = new List<Design>(Designs),
            Authors = new List<Author>(Authors),
            Qualities = new List<Quality>(Qualities),
            Attributes = newAttributes
        };
    }

    #region Design Family Helpers







    public Design FindDesignByGuid(string designGuid)
    {
        var design = Designs.FirstOrDefault(d => d.Guid == designGuid);
        if (design is null) throw new ArgumentException($"Design {designGuid} not found in kit {Name}");
        return design;
    }







    public Design GetPrimitiveDesign(string designGuid)
    {
        var current = FindDesignByGuid(designGuid);
        while (current.Parent?.Guid is not null)
        {
            current = FindDesignByGuid(current.Parent.Guid);
        }
        return current;
    }






    public List<Design> GetDesignFamily(string designGuid)
    {
        var primitive = GetPrimitiveDesign(designGuid);
        var family = new List<Design>();
        CollectDesignDescendants(primitive.Guid, family);
        return family;
    }

    private void CollectDesignDescendants(string parentGuid, List<Design> family)
    {
        var parent = FindDesignByGuid(parentGuid);
        family.Add(parent);
        var children = Designs.Where(d => d.Parent?.Guid == parentGuid);
        foreach (var child in children)
        {
            CollectDesignDescendants(child.Guid, family);
        }
    }







    public bool AreDesignsInSameFamily(string designGuidA, string designGuidB)
    {
        var primitiveA = GetPrimitiveDesign(designGuidA);
        var primitiveB = GetPrimitiveDesign(designGuidB);
        return primitiveA.Guid == primitiveB.Guid;
    }








    public bool CanUseDesignAsPiece(string containerDesignGuid, string pieceDesignGuid)
    {
        return !AreDesignsInSameFamily(containerDesignGuid, pieceDesignGuid);
    }






    public List<Piece> FindSameFamilyDesignPieces(string designGuid)
    {
        var design = FindDesignByGuid(designGuid);
        return design.Pieces
            .Where(p => p.Design?.Guid is not null && AreDesignsInSameFamily(designGuid, p.Design.Guid))
            .ToList();
    }

    #endregion Design Family Helpers

    #region Type Family Helpers







    public Type FindTypeByGuid(string typeGuid)
    {
        var type = Types.FirstOrDefault(t => t.Guid == typeGuid);
        if (type is null) throw new ArgumentException($"Type {typeGuid} not found in kit {Name}");
        return type;
    }







    public Type GetPrimitiveType(string typeGuid)
    {
        var current = FindTypeByGuid(typeGuid);
        while (current.Parent?.Guid is not null)
        {
            current = FindTypeByGuid(current.Parent.Guid);
        }
        return current;
    }






    public List<Type> GetTypeFamily(string typeGuid)
    {
        var primitive = GetPrimitiveType(typeGuid);
        var family = new List<Type>();
        CollectTypeDescendants(primitive.Guid, family);
        return family;
    }

    private void CollectTypeDescendants(string parentGuid, List<Type> family)
    {
        var parent = FindTypeByGuid(parentGuid);
        family.Add(parent);
        var children = Types.Where(t => t.Parent?.Guid == parentGuid);
        foreach (var child in children)
        {
            CollectTypeDescendants(child.Guid, family);
        }
    }







    public bool AreTypesInSameFamily(string typeGuidA, string typeGuidB)
    {
        var primitiveA = GetPrimitiveType(typeGuidA);
        var primitiveB = GetPrimitiveType(typeGuidB);
        return primitiveA.Guid == primitiveB.Guid;
    }

    #endregion Type Family Helpers
}

#endregion Kit

#region Api

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

#endregion Api

#region ZipRoundtrip

public class KitImportResult
{
    public Kit Kit { get; set; } = new();
    public Dictionary<string, byte[]> Files { get; set; } = new();
}

public static class ZipRoundtrip
{
    public static KitImportResult ImportKit(string zipPath)
    {
        var result = new KitImportResult();
        var tempDir = Path.Combine(Path.GetTempPath(), $"semio-kit-{Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);

        try
        {
            ZipFile.ExtractToDirectory(zipPath, tempDir);

            var dbPath = Path.Combine(tempDir, ".semio", "kit.db");
            if (!System.IO.File.Exists(dbPath))
                throw new FileNotFoundException("kit.db not found in zip");

            result.Kit = LoadKitFromSqlite(dbPath);

            foreach (var file in Directory.GetFiles(tempDir, "*", SearchOption.AllDirectories))
            {
                var relativePath = file.Substring(tempDir.Length).TrimStart(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar).Replace("\\", "/");
                if (!relativePath.StartsWith(".semio/"))
                    result.Files[relativePath] = System.IO.File.ReadAllBytes(file);
            }
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }

        return result;
    }

    public static void ExportKit(Kit kit, Dictionary<string, byte[]> files, string zipPath, string schemaSQL)
    {
        var tempDir = Path.Combine(Path.GetTempPath(), $"semio-kit-{Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);

        try
        {
            var semioDir = Path.Combine(tempDir, ".semio");
            Directory.CreateDirectory(semioDir);
            var dbPath = Path.Combine(semioDir, "kit.db");

            SaveKitToSqlite(kit, dbPath, schemaSQL);

            foreach (var kvp in files)
            {
                var fullPath = Path.Combine(tempDir, kvp.Key);
                var dir = Path.GetDirectoryName(fullPath);
                if (!string.IsNullOrEmpty(dir))
                    Directory.CreateDirectory(dir);
                System.IO.File.WriteAllBytes(fullPath, kvp.Value);
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
                Stock = reader.IsDBNull(5) ? 0 : reader.GetInt32(5),
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

        foreach (var t in kit.Types)
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

        foreach (var d in kit.Designs)
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

#endregion ZipRoundtrip

#region KitImporter

public static class KitImporter
{
    public static (Kit Kit, Dictionary<string, byte[]> Files) ImportFromZip(string zipPath)
    {
        var result = ZipRoundtrip.ImportKit(zipPath);
        return (result.Kit, result.Files);
    }
}

#endregion KitImporter

#region KitExporter

public static class KitExporter
{
    private static readonly string DefaultSchemaSQL = GetEmbeddedSchema();

    private static string GetEmbeddedSchema()
    {
        var possiblePaths = new[]
        {
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

        throw new FileNotFoundException("Could not find schema.sql for SQLite kit export");
    }

    public static void ExportToZip(Kit kit, Dictionary<string, byte[]> files, string zipPath)
    {
        ZipRoundtrip.ExportKit(kit, files, zipPath, DefaultSchemaSQL);
    }
}

#endregion KitExporter

#region SemioDiff

public static class SemioDiff
{
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

        diff.Types = GetTypesDiff(before.Types ?? new List<Type>(), after.Types ?? new List<Type>());
        diff.Designs = GetDesignsDiff(before.Designs ?? new List<Design>(), after.Designs ?? new List<Design>());

        return diff;
    }

    private static string? NormalizeString(string? value) => string.IsNullOrEmpty(value) ? null : value;

    private static TypesDiff? GetTypesDiff(List<Type> before, List<Type> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(t => new TypeId { Guid = t.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<DiffUpdate<TypeDiff>>();

        foreach (var afterType in after)
        {
            var beforeType = before.FirstOrDefault(b => b.Guid == afterType.Guid);
            if (beforeType != null)
            {
                var typeDiff = GetTypeDiff(beforeType, afterType);
                if (typeDiff != null)
                    updated.Add(new DiffUpdate<TypeDiff> { Id = afterType.Guid, Diff = typeDiff });
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
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (NormalizeString(before.Icon) != NormalizeString(after.Icon)) { diff.Icon = after.Icon; hasChanges = true; }
        if (NormalizeString(before.Image) != NormalizeString(after.Image)) { diff.Image = after.Image; hasChanges = true; }

        return hasChanges ? diff : null;
    }

    private static DesignsDiff? GetDesignsDiff(List<Design> before, List<Design> after)
    {
        var removed = before.Where(b => !after.Any(a => a.Guid == b.Guid)).Select(d => new DesignId { Guid = d.Guid }).ToList();
        var added = after.Where(a => !before.Any(b => b.Guid == a.Guid)).ToList();
        var updated = new List<DiffUpdate<DesignDiff>>();

        foreach (var afterDesign in after)
        {
            var beforeDesign = before.FirstOrDefault(b => b.Guid == afterDesign.Guid);
            if (beforeDesign != null)
            {
                var designDiff = GetDesignDiff(beforeDesign, afterDesign);
                if (designDiff != null)
                    updated.Add(new DiffUpdate<DesignDiff> { Id = afterDesign.Guid, Diff = designDiff });
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
        if (NormalizeString(before.Description) != NormalizeString(after.Description)) { diff.Description = after.Description; hasChanges = true; }
        if (NormalizeString(before.Icon) != NormalizeString(after.Icon)) { diff.Icon = after.Icon; hasChanges = true; }
        if (NormalizeString(before.Image) != NormalizeString(after.Image)) { diff.Image = after.Image; hasChanges = true; }

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

        return inverse;
    }

    private static TypesDiff InverseTypesDiff(List<Type> original, TypesDiff appliedDiff)
    {
        var inverse = new TypesDiff
        {
            Removed = appliedDiff.Added?.Select(t => new TypeId { Guid = t.Guid }).ToList() ?? new List<TypeId>(),
            Added = appliedDiff.Removed?.Select(id => original.FirstOrDefault(t => t.Guid == id.Guid)).Where(t => t != null).Cast<Type>().ToList() ?? new List<Type>(),
            Updated = new List<DiffUpdate<TypeDiff>>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalType = original.FirstOrDefault(t => t.Guid == update.Id);
                if (originalType != null && update.Diff != null)
                {
                    var inverseDiff = new TypeDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalType.Name;
                    if (update.Diff.Description != null) inverseDiff.Description = originalType.Description;
                    if (update.Diff.Icon != null) inverseDiff.Icon = originalType.Icon;
                    if (update.Diff.Image != null) inverseDiff.Image = originalType.Image;
                    inverse.Updated.Add(new DiffUpdate<TypeDiff> { Id = update.Id, Diff = inverseDiff });
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
            Updated = new List<DiffUpdate<DesignDiff>>()
        };

        if (appliedDiff.Updated != null)
        {
            foreach (var update in appliedDiff.Updated)
            {
                var originalDesign = original.FirstOrDefault(d => d.Guid == update.Id);
                if (originalDesign != null && update.Diff != null)
                {
                    var inverseDiff = new DesignDiff();
                    if (update.Diff.Name != null) inverseDiff.Name = originalDesign.Name;
                    if (update.Diff.Description != null) inverseDiff.Description = originalDesign.Description;
                    if (update.Diff.Icon != null) inverseDiff.Icon = originalDesign.Icon;
                    if (update.Diff.Image != null) inverseDiff.Image = originalDesign.Image;
                    inverse.Updated.Add(new DiffUpdate<DesignDiff> { Id = update.Id, Diff = inverseDiff });
                }
            }
        }

        return inverse;
    }

    public static Kit ApplyKitDiff(Kit baseKit, KitDiff diff)
    {
        var result = baseKit.DeepClone()!;

        if (diff.Name != null) result.Name = diff.Name;
        if (diff.Version != null) result.Version = diff.Version;
        if (diff.Description != null) result.Description = diff.Description;
        if (diff.Icon != null) result.Icon = diff.Icon;
        if (diff.Image != null) result.Image = diff.Image;
        if (diff.Preview != null) result.Preview = diff.Preview;
        if (diff.Remote != null) result.Remote = diff.Remote;
        if (diff.Homepage != null) result.Homepage = diff.Homepage;
        if (diff.License != null) result.License = diff.License;

        if (diff.Types != null)
            result.Types = ApplyTypesDiff(result.Types ?? new List<Type>(), diff.Types);

        if (diff.Designs != null)
            result.Designs = ApplyDesignsDiff(result.Designs ?? new List<Design>(), diff.Designs);

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
                var type = result.FirstOrDefault(t => t.Guid == update.Id);
                if (type != null && update.Diff != null)
                {
                    if (update.Diff.Name != null) type.Name = update.Diff.Name;
                    if (update.Diff.Description != null) type.Description = update.Diff.Description;
                    if (update.Diff.Icon != null) type.Icon = update.Diff.Icon;
                    if (update.Diff.Image != null) type.Image = update.Diff.Image;
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
                var design = result.FirstOrDefault(d => d.Guid == update.Id);
                if (design != null && update.Diff != null)
                {
                    if (update.Diff.Name != null) design.Name = update.Diff.Name;
                    if (update.Diff.Description != null) design.Description = update.Diff.Description;
                    if (update.Diff.Icon != null) design.Icon = update.Diff.Icon;
                    if (update.Diff.Image != null) design.Image = update.Diff.Image;
                }
            }
        }

        if (diff.Added != null)
            result.AddRange(diff.Added);

        return result;
    }

    public static bool AreKitsEqual(Kit a, Kit b)
    {
        return a.Serialize() == b.Serialize();
    }

    public static bool AreKitDiffsEqual(KitDiff a, KitDiff b)
    {
        return a.Serialize() == b.Serialize();
    }
}

#endregion SemioDiff

#endregion Entitying



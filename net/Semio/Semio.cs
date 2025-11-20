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
    public const int ExpressionLengthLimit = 4096;
    public const int ValueLengthLimit = 512;
    public const int AttributesMax = 64;
    public const int QualityMax = 1024;
    public const int TagsMax = 8;
    public const int ModelsMax = 32;
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

#endregion

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
                internal ConvertModel() { FromUnit = ""; ToUnit = ""; }
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

    public override string ToString() => string.IsNullOrEmpty(Unit) ? Value.ToString(CultureInfo.InvariantCulture) : $"'{Value.ToString(CultureInfo.InvariantCulture)} {Unit}'";
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

// Numeric Operators
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

// Boolean/Logical Operators (using 1.0f for true, 0.0f for false)
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

// Comparison Operators
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

// Conditional Operator
public class If : Operator
{
    public override string Keyword => "if";
    public override object Apply(object[] args, string targetUnit = "")
    {
        if (args.Length != 3) throw new ArgumentException("if requires exactly 3 operands: condition, true-value, false-value");
        return (float)args[0] != 0f ? args[1] : args[2];
    }
}

// Additional Math Operators
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

// Text/String Operators
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

// Conversion Operators
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

// Additional Utility Operators
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
            // Arithmetic operators
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
            
            // Boolean operators
            { "and", () => new And() },
            { "or", () => new Or() },
            { "xor", () => new ExclusiveOr() },
            { "not", () => new Invert() },
            
            // Comparison operators
            { "equal", () => new Equal() },
            { "greater", () => new GreaterThan() },
            { "less", () => new LessThan() },
            { "greater-equal", () => new GreaterThanOrEqual() },
            { "less-equal", () => new LessThanOrEqual() },
            
            // Conditional operator
            { "if", () => new If() },
            
            // Text operators
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
            
            // Conversion operators
            { "number", () => new ToNumber() },
            { "text", () => new ToText() },
            { "boolean", () => new ToBoolean() },
            
            // Utility operators
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
            NumberConstant c => string.IsNullOrEmpty(targetUnit) ? c.UnitValue : c.UnitValue.ConvertTo(targetUnit),
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

    // --- Serialization helpers ---

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

    // --- Parsing ---

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

            // skip whitespace
            if (char.IsWhiteSpace(c)) { i++; continue; }

            if (c == '(') { tokens.Add(new Token(TokenKind.LeftParenthesis, "(", i)); i++; continue; }
            if (c == ')') { tokens.Add(new Token(TokenKind.RightParenthesis, ")", i)); i++; continue; }

            // string literal
            if (c == '"')
            {
                int start = i;
                i++; // skip opening quote
                var sb = new StringBuilder();
                while (i < input.Length && input[i] != '"')
                {
                    if (input[i] == '\\' && i + 1 < input.Length)
                    {
                        i++; // skip backslash
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
                i++; // skip closing quote
                tokens.Add(new Token(TokenKind.String, sb.ToString(), start));
                continue;
            }

            // unit literal (single quotes)
            if (c == '\'')
            {
                int start = i;
                i++; // skip opening quote
                var sb = new StringBuilder();
                while (i < input.Length && input[i] != '\'')
                {
                    if (input[i] == '\\' && i + 1 < input.Length)
                    {
                        i++; // skip backslash
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
                i++; // skip closing quote
                tokens.Add(new Token(TokenKind.UnitLiteral, sb.ToString(), start));
                continue;
            }

            // number (supports leading sign and decimal point)
            if (char.IsDigit(c) || (c == '.' && i + 1 < input.Length && char.IsDigit(input[i + 1])))
            {
                int start = i;
                i++;
                while (i < input.Length && (char.IsDigit(input[i]) || input[i] == '.')) i++;
                // optional exponent
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

            // identifier: letters/digits/_ plus '.' and '-' allowed within
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

            // allow identifiers that start with digits if they contain dot/hyphen? Not typical; reject:
            throw new FormatException($"Unexpected character '{c}' at position {i}.");
        }
        return tokens;
    }

    // Grammar:
    //   expr := number
    //         | string
    //         | identifier
    //         | identifier '(' expr* ')'
    // Operands are space-separated; no commas required.
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
                // Just a number without unit: '2.3'
                if (!float.TryParse(parts[0], NumberStyles.Float | NumberStyles.AllowThousands, CultureInfo.InvariantCulture, out var val))
                    throw new FormatException($"Invalid number '{parts[0]}' in unit literal at {t.Position}.");
                return new NumberConstant(val);
            }
            else
            {
                // Number with unit: '2.3 m'
                if (!float.TryParse(parts[0], NumberStyles.Float | NumberStyles.AllowThousands, CultureInfo.InvariantCulture, out var val))
                    throw new FormatException($"Invalid number '{parts[0]}' in unit literal at {t.Position}.");
                var unit = string.Join(" ", parts.Skip(1));
                return new NumberConstant(val, unit);
            }
        }

        if (t.Kind == TokenKind.Identifier)
        {
            // lookahead to see if this is a call: <ident> '(' ... ')'
            string ident = t.Text;
            int idPos = t.Position;
            index++;

            if (index < tokens.Count && tokens[index].Kind == TokenKind.LeftParenthesis)
            {
                // operator application
                index++; // consume '('
                var args = new List<Term>();
                while (index < tokens.Count && tokens[index].Kind != TokenKind.RightParenthesis)
                {
                    // parse next expr
                    args.Add(ParseExpr(tokens, ref index));
                    // arguments are separated by whitespace; no special token to consume
                }
                if (index >= tokens.Count || tokens[index].Kind != TokenKind.RightParenthesis)
                    throw new FormatException($"Missing closing ')' for call starting at {idPos}.");
                index++; // consume ')'

                var op = InstantiateOperator(ident, idPos);
                // Optional arity checks per operator (divide >= 2)
                if (op is Divide && args.Count < 2)
                    throw new FormatException("divide requires at least 2 operands.");

                return new Operation(op, args.ToArray());
            }
            else
            {
                // plain variable
                return new Variable(ident);
            }
        }

        if (t.Kind == TokenKind.LeftParenthesis)
        {
            // Support parenthesized single expression: ( expr )
            index++; // '('
            var inner = ParseExpr(tokens, ref index);
            if (index >= tokens.Count || tokens[index].Kind != TokenKind.RightParenthesis)
                throw new FormatException($"Missing ')' for parenthesized expression starting at {t.Position}.");
            index++; // ')'
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

#region Modeling

public abstract class MetaAttribute : System.Attribute
{
    public MetaAttribute(string emoji, string code, string abbreviation, string description)
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
public class ModelAttribute : MetaAttribute
{
    public ModelAttribute(string emoji, string code, string abbreviation, string description)
        : base(emoji, code,
            abbreviation, description)
    { }
}

[AttributeUsage(AttributeTargets.Enum)]
public class EnumAttribute : MetaAttribute
{
    public EnumAttribute(string emoji, string code, string abbreviation, string description)
        : base(emoji, code, abbreviation, description)
    { }
}

public enum PropImportance
{
    OPTIONAL,
    REQUIRED,
    ID
}

[AttributeUsage(AttributeTargets.Property)]
public abstract class PropAttribute : MetaAttribute
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

public class ColorAttribute : TextAttribute
{
    public ColorAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, 7)
    { }
}

public class DescriptionAttribute : TextAttribute
{
    public DescriptionAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, Constants.DescriptionLengthLimit)
    { }
}

public class ValueAttribute : TextAttribute
{
    public ValueAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = true, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, Constants.ValueLengthLimit)
    { }
}

public class ExpressionAttribute : TextAttribute
{
    public ExpressionAttribute(string emoji, string code, string abbreviation, string description,
        PropImportance importance = PropImportance.OPTIONAL, bool isDefaultValid = false, bool skipValidation = false) :
        base(emoji, code, abbreviation, description, importance, isDefaultValid, skipValidation, Constants.ExpressionLengthLimit)
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
                        (p.GetValue(this) as string ?? "") != "")
            .Select(p => p.Name);
        var nonEmptyIdPropertiesValues = nonEmptyIdProperties.Select(p => GetType().GetProperty(p)?.GetValue(this))
            .Where(v => v != null)
            .Select(v => v!.ToString()).ToList();
        if (nonEmptyIdPropertiesValues.Count != 0)
            return $"{modelAttribute?.Abbreviation ?? ""}({string.Join(",", nonEmptyIdPropertiesValues)})";
        var requiredProperties = GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(p => p.GetCustomAttribute<PropAttribute>()?.Importance == PropImportance.REQUIRED)
            .Select(p => p.Name);
        var requiredPropertiesValues = requiredProperties.Select(p => GetType().GetProperty(p)?.GetValue(this))
            .Where(v => v != null)
            .Select(v => v!.ToString()).ToList();
        return $"{modelAttribute?.Abbreviation ?? ""}({string.Join(",", requiredPropertiesValues)})";
    }

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

    public static bool operator ==(Model<T> left, Model<T> right)
    {
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null) return false;
        return left.Equals(right);
    }

    public static bool operator !=(Model<T> left, Model<T> right) => !(left == right);

    public T? DeepClone() => this.Serialize().Deserialize<T>();

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
        if (propAttribute?.SkipValidation == true) return;
        if (isPropertyList)
            RuleFor(model => property.GetValue(model))
                .NotEmpty()
                .WithMessage($"The {property.Name.ToLower()} must have at least one.")
                .When(m => propAttribute?.Importance != PropImportance.OPTIONAL);
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
            if (textAttribute is null) return;
            RuleFor(model => property.GetValue(model) as string)
                .NotEmpty()
                .When(m => !(textAttribute.Importance == PropImportance.OPTIONAL || textAttribute.IsDefaultValid))
                .WithMessage($"The {property.Name.ToLower()} must not be empty.")
                .MaximumLength(textAttribute.LengthLimit)
                .WithMessage(model =>
                {
                    var value = property.GetValue(model) as string;
                    var preview = value?.Length > 10 ? value!.Substring(0, 10) + "..." : value ?? "";
                    return
                        $"The {property.Name.ToLower()} must be at most {textAttribute.LengthLimit} characters long. The provided text ({preview}) has {value?.Length ?? 0} characters.";
                });
            // All non-description text
            if (property.GetCustomAttribute<DescriptionAttribute>() == null)
            {
                RuleFor(model => property.GetValue(model) as string)
                    .Matches(@"^\S.*$")
                    .When(m => (property.GetValue(m) as string ?? "") != "")
                    .WithMessage($"The {property.Name.ToLower()} must not start with a space.")
                    .Matches(@"^.*\S$")
                    .When(m => (property.GetValue(m) as string ?? "") != "")
                    .WithMessage($"The {property.Name.ToLower()} must not end with a space.")
                    .Matches(@"^[^\r\n]*$")
                    .When(m => (property.GetValue(m) as string ?? "") != "")
                    .WithMessage($"The {property.Name.ToLower()} must not contain newlines.");
            }

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
            if (textAttribute is null) return;
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
                    var preview = item?.Length > 10 ? item.Substring(0, 10) + "..." : item ?? "";
                    var singularPropertyName = property.Name.ToLower().TrimEnd('s');
                    return
                        $"A {singularPropertyName} must be at most {textAttribute.LengthLimit} characters long. The provided {singularPropertyName} ({preview}) has {item?.Length ?? 0} characters.";
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

#region Attribute

[Model("🔐", "AI", "AtI", "The ID of the attribute.")]
public class AttributeId : Model<AttributeId>
{
    [Name("🔑", "Ke?", "Key?", "The optional key of the attribute.")]
    public string Key { get; set; } = "";

    public static implicit operator AttributeId(Attribute attribute) => new() { Key = attribute.Key };
    public static implicit operator AttributeId(AttributeDiff diff) => new() { Key = diff.Key };
}

[Model("🔐", "AD", "ADf", "A diff for attributes.")]
public class AttributeDiff : Model<AttributeDiff>
{
    [Name("🔑", "Ke?", "Key?", "The optional key of the attribute.")]
    public string Key { get; set; } = "";
    [Description("🔢", "Vl?", "Val?", "The optional value of the attribute.")]
    public string Value { get; set; } = "";
    [Description("📖", "Df?", "Def?", "The optional definition of the attribute.")]
    public string Definition { get; set; } = "";

    public static implicit operator AttributeDiff(AttributeId id) => new() { Key = id.Key };
    public static implicit operator AttributeDiff(Attribute attribute) => new() { Key = attribute.Key, Value = attribute.Value, Definition = attribute.Definition };

    public AttributeDiff MergeDiff(AttributeDiff other)
    {
        return new AttributeDiff
        {
            Key = string.IsNullOrEmpty(other.Key) ? Key : other.Key,
            Value = string.IsNullOrEmpty(other.Value) ? Value : other.Value,
            Definition = string.IsNullOrEmpty(other.Definition) ? Definition : other.Definition
        };
    }
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-attribute-"/>
/// </summary>
[Model("🏷️", "At", "Atr", "A attribute is a key value pair with an an optional definition.")]
public class Attribute : Model<Attribute>
{
    [Name("🔑", "Ke", "Key", "The key of the attribute.", PropImportance.ID)]
    public string Key { get; set; } = "";

    [Description("🔢", "Vl?", "Val?", "The optional value [ text | url ] of the attribute. No value is equivalent to true.")]
    public string Value { get; set; } = "";

    [Description("📖", "Df?", "Def?", "The optional definition [ text | uri ] of the attribute.")]
    public string Definition { get; set; } = "";

    public static implicit operator Attribute(AttributeId id) => new() { Key = id.Key };
    public static implicit operator Attribute(AttributeDiff diff) => new() { Key = diff.Key, Value = diff.Value, Definition = diff.Definition };

    public Attribute ApplyDiff(AttributeDiff diff)
    {
        return new Attribute
        {
            Key = !string.IsNullOrEmpty(diff.Key) ? diff.Key : Key,
            Value = !string.IsNullOrEmpty(diff.Value) ? diff.Value : Value,
            Definition = !string.IsNullOrEmpty(diff.Definition) ? diff.Definition : Definition
        };
    }
    public AttributeDiff CreateDiff()
    {
        return new AttributeDiff
        {
            Key = Key,
            Value = Value,
            Definition = Definition
        };
    }
    public AttributeDiff InverseDiff(AttributeDiff appliedDiff)
    {
        return new AttributeDiff
        {
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

/// <summary>
/// <see href="https://github.com/usalu/semio#-diagram-point-"/>
/// </summary>
[Model("📺", "DP", "DPt", "A 2d-point (uv) of floats in the diagram. One unit is equal the width of a piece icon.")]
public class Coord : Model<Coord>
{
    [NumberProp("🎚️", "U", "U", "The u-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.", PropImportance.REQUIRED)]
    public float U { get; set; }

    [NumberProp("🎚️", "V", "V", "The v-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.", PropImportance.REQUIRED)]
    public float V { get; set; }

    public Coord Normalize()
    {
        var length = (float)Math.Sqrt(U * U + V * V);
        return new Coord { U = U / length, V = V / length };
    }
}

#endregion Coord

#region Point

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

#endregion Point

#region Vector

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

#endregion Vector

#region Plane

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

#endregion Plane

#region Location

[Model("📍", "Lc", "Loc", "A location on the earth surface (longitude, latitude).")]
public class Location : Model<Location>
{
    [NumberProp("↔️", "Lo", "Lon", "The longitude of the location in degrees.", PropImportance.REQUIRED)]
    public float Longitude { get; set; }
    [NumberProp("↕️", "La", "Lat", "The latitude of the location in degrees.", PropImportance.REQUIRED)]
    public float Latitude { get; set; }
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the location.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
}

#endregion Location

#region Author

[Model("👤", "Au", "Aut", "The id of the author.")]
public class AuthorId : Model<AuthorId>
{
    [Email("📧", "Em", "Eml", "The email of the author.", PropImportance.ID)]
    public string Email { get; set; } = "";
    public static implicit operator AuthorId(Author author) => new() { Email = author.Email };
    public string ToIdString() => $"{Email}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Aut({ToHumanIdString()})";
}

[Model("🔗", "AA", "ArtAuth", "A many-to-many relationship between authors and artifacts (types/designs).")]
public class ArtifactAuthor : Model<ArtifactAuthor>
{
    [Email("📧", "AEm", "AEml", "The email of the author.", PropImportance.ID)]
    public string AuthorEmail { get; set; } = "";
    [Id("🧩", "TId?", "TyId?", "The optional type ID if this author is for a type.", isDefaultValid: true)]
    public TypeId? TypeId { get; set; }
    [Id("🏙️", "DId?", "DsId?", "The optional design ID if this author is for a design.", isDefaultValid: true)]
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

[Model("", "Au", "Aut", "The information about the author.")]
public class Author : Model<Author>
{
    [Id("🆔", "Gd", "Gui", "The guid of the author.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    [Name("📛", "Na", "Nam", "The name of the author.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Email("📧", "Em", "Eml", "The email of the author.", PropImportance.ID)]
    public string Email { get; set; } = "";
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the author.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Email}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Aut({ToHumanIdString()})";

    public static implicit operator Author(AuthorId id) => new() { Email = id.Email };

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

#endregion Author

#region File

[Model("📄", "Fl", "Fil", "The identifier of a file.")]
public class FileId : Model<FileId>
{
    [Id("🆔", "Gd", "Gui", "The guid of the file.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"FilId({ToHumanIdString()})";

    public static implicit operator FileId(File file) => new() { Guid = file.Guid };
    public static implicit operator FileId(FileDiff diff) => new() { Guid = diff.Guid ?? "" };
}

[Model("📄", "FD", "FDf", "A diff for files.")]
public class FileDiff : Model<FileDiff>
{
    [Id("🆔", "Gd?", "Gui?", "The optional guid of the file.")]
    public string? Guid { get; set; }
    [Name("📛", "Nm?", "Nam?", "The optional name of the file.")]
    public string? Name { get; set; }
    [Url("�", "Rm?", "Rem?", "The optional remote url of the file.")]
    public string? Remote { get; set; }
    [Name("📁", "Fo?", "Fol?", "The optional folder path of the file.")]
    public string? Folder { get; set; }
    [NumberProp("📏", "Sz?", "Siz?", "The optional size of the file in bytes.")]
    public int? Size { get; set; }
    [Name("🔐", "Hs?", "Has?", "The optional hash of the file.")]
    public string? Hash { get; set; }
    [Name("📅", "CA?", "CrA?", "The optional created at timestamp of the file.")]
    public DateTime? CreatedAt { get; set; }
    [Id("👤", "CB?", "CrB?", "The optional created by user guid of the file.")]
    public string? CreatedBy { get; set; }
    [Name("📅", "UA?", "UpA?", "The optional updated at timestamp of the file.")]
    public DateTime? UpdatedAt { get; set; }
    [Id("👤", "UB?", "UpB?", "The optional updated by user guid of the file.")]
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

[Model("📊", "FsD", "FsDf", "A diff for multiple files.")]
public class FilesDiff : Model<FilesDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed files.", PropImportance.OPTIONAL)]
    public List<FileId> Removed { get; set; } = new();
    [ModelProp("✏️", "Up*", "Upd*", "The optional updated files.", PropImportance.OPTIONAL)]
    public List<FileDiff> Updated { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added files.", PropImportance.OPTIONAL)]
    public List<File> Added { get; set; } = new();

    public static implicit operator FilesDiff(List<File> files) => new() { Updated = files.Select(f => (FileDiff)f).ToList() };
}

[Model("📄", "Fl", "Fil", "A file with content.")]
public class File : Model<File>
{
    [Id("🆔", "Gd", "Gui", "The guid of the file.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    [Name("�", "Nm", "Nam", "The name of the file.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Url("🔗", "Rm?", "Rem?", "The optional remote url of the file.")]
    public string? Remote { get; set; }
    [Name("📁", "Fo?", "Fol?", "The optional folder path of the file.")]
    public string? Folder { get; set; }
    [NumberProp("📏", "Sz?", "Siz?", "The optional size of the file in bytes.")]
    public int? Size { get; set; }
    [Name("🔐", "Hs?", "Has?", "The optional hash of the file.")]
    public string? Hash { get; set; }
    [Name("📅", "CA", "CrA", "The created at timestamp of the file.", PropImportance.REQUIRED)]
    public DateTime CreatedAt { get; set; }
    [Id("👤", "CB?", "CrB?", "The optional created by user guid of the file.")]
    public string? CreatedBy { get; set; }
    [Name("📅", "UA", "UpA", "The updated at timestamp of the file.", PropImportance.REQUIRED)]
    public DateTime UpdatedAt { get; set; }
    [Id("👤", "UB?", "UpB?", "The optional updated by user guid of the file.")]
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

[Model("�", "FId", "FolId", "The identifier for a folder.")]
public class FolderId : Model<FolderId>
{
    [Id("🆔", "Gd", "Gui", "The guid of the folder.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"FolderId({ToHumanIdString()})";

    public static implicit operator FolderId(Folder folder) => new() { Guid = folder.Guid };
    public static implicit operator FolderId(FolderDiff diff) => new() { Guid = diff.Guid ?? "" };
}

[Model("📁", "FD", "FolDf", "A diff for folders.")]
public class FolderDiff : Model<FolderDiff>
{
    [Id("🆔", "Gd?", "Gui?", "The optional guid of the folder.")]
    public string? Guid { get; set; }
    [Name("📛", "Na?", "Nam?", "The optional name of the folder.")]
    public string? Name { get; set; }
    [Id("📁", "Pa?", "Par?", "The optional parent folder guid.")]
    public string? Parent { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the folder.")]
    public string? Description { get; set; }
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the folder.", PropImportance.OPTIONAL)]
    public List<Attribute>? Attributes { get; set; }
    [Name("📅", "CA?", "CrA?", "The optional creation date.")]
    public string? CreatedAt { get; set; }
    [Name("👤", "CB?", "CrB?", "The optional user who created the folder.")]
    public string? CreatedBy { get; set; }
    [Name("📝", "UA?", "UpA?", "The optional last update date.")]
    public string? UpdatedAt { get; set; }
    [Name("👤", "UB?", "UpB?", "The optional user who last updated the folder.")]
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

[Model("📁", "FsD", "FolsDf", "A diff for multiple folders.")]
public class FoldersDiff : Model<FoldersDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed folders.", PropImportance.OPTIONAL)]
    public List<FolderId> Removed { get; set; } = new();
    [ModelProp("✏️", "Up*", "Upd*", "The optional updated folders.", PropImportance.OPTIONAL)]
    public List<FolderDiff> Updated { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added folders.", PropImportance.OPTIONAL)]
    public List<Folder> Added { get; set; } = new();

    public static implicit operator FoldersDiff(List<Folder> folders) => new() { Updated = folders.Select(f => (FolderDiff)f).ToList() };
}

[Model("📁", "Fol", "Folder", "A folder is an organizational container.")]
public class Folder : Model<Folder>
{
    [Id("🆔", "Gd", "Gui", "The guid of the folder.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    [Name("📛", "Na", "Nam", "The name of the folder.")]
    public string Name { get; set; } = "";
    [Id("📁", "Pa?", "Par?", "The optional parent folder guid.")]
    public string? Parent { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the folder.")]
    public string Description { get; set; } = "";
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the folder.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
    [Name("📅", "CA", "CrA", "The creation date of the folder.")]
    public string CreatedAt { get; set; } = "";
    [Name("👤", "CB?", "CrB?", "The optional user who created the folder.")]
    public string? CreatedBy { get; set; }
    [Name("📝", "UA", "UpA", "The last update date of the folder.")]
    public string UpdatedAt { get; set; } = "";
    [Name("👤", "UB?", "UpB?", "The optional user who last updated the folder.")]
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

/// <summary>
/// <see href="https://github.com/usalu/semio#-benchmark-"/>
/// </summary>
[Model("🔢", "Bm", "Bmk", "A benchmark is a value with an optional unit for a quality.")]
public class Benchmark : Model<Benchmark>
{
    [Name("📛", "Nm", "Name", "The name of the benchmark.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Url("🖼️", "Ic", "Ico", "The icon [ emoji | url ] of the benchmark.")]
    public string Icon { get; set; } = "";
    [NumberProp("⬇️", "Mi?", "Min?", "The optional minimum value of the benchmark.")]
    public float Min { get; set; } = 0;
    [FalseOrTrue("⬇️", "MiE?", "MiE?", "Whether the minimum value is excluded from the range.")]
    public bool MinExcluded { get; set; } = false;
    [NumberProp("⬆️", "Mx?", "Max?", "The optional maximum value of the benchmark.")]
    public float Max { get; set; } = 0;
    [FalseOrTrue("⬆️", "MxE?", "MxE?", "Whether the maximum value is excluded from the range.")]
    public bool MaxExcluded { get; set; } = false;
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the benchmark.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
}

#endregion Benchmark

#region QualityKind

[Flags]
[Enum("🏷️", "QK", "QlK", "The kind of quality indicating its scope and applicability.")]
public enum QualityKind
{
    General = 0,
    Design = 1,
    Type = 2,
    Piece = 4,
    Connection = 8,
    Port = 16,
}

#endregion QualityKind

#region Quality

/// <summary>
/// <see href="https://github.com/usalu/semio#-quality-"/>
/// </summary>
[Model("🔑", "Ql", "Qal", "A quality id is a key for a quality.")]
public class QualityId : Model<QualityId>
{
    [Id("🔑", "Ke", "Key", "The key of the quality.")]
    public string Key { get; set; } = "";

    public static implicit operator QualityId(Quality quality) => new() { Key = quality.Key };
    public static implicit operator QualityId(QualityDiff diff) => new() { Key = diff.Key };
}

[Model("📊", "QD", "QDf", "A diff for qualities.")]
public class QualityDiff : Model<QualityDiff>
{
    [Id("🔑", "Ke", "Key", "The key of the quality.")]
    public string Key { get; set; } = "";
    [Name("📛", "Nm", "Name", "The name of the quality.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the quality.")]
    public string Description { get; set; } = "";
    [Url("🔗", "Ur?", "Uri?", "The Unique Resource Identifier (URI) of the quality.")]
    public string Uri { get; set; } = "";
    [FalseOrTrue("🔢", "Sc?", "Sc?", "Whether the quality is scalable.")]
    public bool Scalable { get; set; } = false;
    [Name("🔢", "Kd", "Kn", "The kind of the quality.")]
    public QualityKind Kind { get; set; } = QualityKind.General;
    [Name("Ⓜ️", "SI?", "SI?", "The optional default SI unit of the quality.")]
    public string SI { get; set; } = "";
    [Name("🦶", "Im?", "Imp?", "The optional default imperial unit of the quality.")]
    public string Imperial { get; set; } = "";
    [NumberProp("⬇️", "Mi?", "Min?", "The optional minimum value of the quality.")]
    public float Min { get; set; } = 0;
    [FalseOrTrue("⬇️", "MiE?", "MiE?", "Whether the minimum value is excluded from the range.")]
    public bool MinExcluded { get; set; } = true;
    [NumberProp("⬆️", "Mx?", "Max?", "The optional maximum value of the quality.")]
    public float Max { get; set; } = 0;
    [FalseOrTrue("⬆️", "MxE?", "MxE?", "Whether the maximum value is excluded from the range.")]
    public bool MaxExcluded { get; set; } = true;
    [NumberProp("Ⓜ️", "Dl?", "Dfl?", "The optional default value of the quality. Either a default value or a formula can be set.")]
    public float Default { get; set; } = 0;
    [ModelProp("🟰", "Fo?", "For?", "The optional formula of the quality.")]
    public string Formula { get; set; } = "";
    [ModelProp("🔢", "Bm*", "Bmk*", "The optional benchmarks of the quality.", PropImportance.OPTIONAL)]
    public List<Benchmark> Benchmarks { get; set; } = new();
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the quality.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator QualityDiff(QualityId quality) => new() { Key = quality.Key };

    public static implicit operator QualityDiff(Quality quality) => new() { Key = quality.Key, Name = quality.Name, Description = quality.Description, Uri = quality.Uri, Scalable = quality.Scalable, Kind = quality.Kind, SI = quality.SI, Imperial = quality.Imperial, Min = quality.Min, MinExcluded = quality.MinExcluded, Max = quality.Max, MaxExcluded = quality.MaxExcluded, Default = quality.Default, Formula = quality.Formula, Benchmarks = quality.Benchmarks, Attributes = quality.Attributes };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-quality-"/>
/// </summary>
[Model("📃", "Ql", "Qal", "A quality is numeric metadata used for stats and benchmarks.")]
public class Quality : Model<Quality>
{
    [Id("🔑", "Ke", "Key", "The key of the quality.")]
    public string Key { get; set; } = "";
    [Name("📛", "Nm", "Name", "The name of the quality.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the quality.")]
    public string Description { get; set; } = "";
    [Url("🔗", "Ur?", "Uri?", "The Unique Resource Identifier (URI) of the quality.")]
    public string Uri { get; set; } = "";
    [FalseOrTrue("🔢", "Sc?", "Sc?", "Whether the quality is scalable.")]
    public bool Scalable { get; set; } = false;
    [Name("🔢", "Kd", "Kn", "The kind of the quality.")]
    public QualityKind Kind { get; set; } = QualityKind.General;
    [Name("Ⓜ️", "SI?", "SI?", "The optional default SI unit of the quality.")]
    public string SI { get; set; } = "";
    [Name("🦶", "Im?", "Imp?", "The optional default imperial unit of the quality.")]
    public string Imperial { get; set; } = "";
    [NumberProp("⬇️", "Mi?", "Min?", "The optional minimum value of the quality.")]
    public float Min { get; set; } = 0;
    [FalseOrTrue("⬇️", "MiE?", "MiE?", "Whether the minimum value is excluded from the range.")]
    public bool MinExcluded { get; set; } = true;
    [NumberProp("⬆️", "Mx?", "Max?", "The optional maximum value of the quality.")]
    public float Max { get; set; } = 0;
    [FalseOrTrue("⬆️", "MxE?", "MxE?", "Whether the maximum value is excluded from the range.")]
    public bool MaxExcluded { get; set; } = true;
    [NumberProp("Ⓜ️", "Dl?", "Dfl?", "The optional default value of the quality. Either a default value or a formula can be set.")]
    public float Default { get; set; } = 0;
    [ModelProp("🟰", "Fo?", "For?", "The optional formula of the quality.")]
    public string Formula { get; set; } = "";
    [ModelProp("🔢", "Bm*", "Bmk*", "The optional benchmarks of the quality.", PropImportance.OPTIONAL)]
    public List<Benchmark> Benchmarks { get; set; } = new();
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the quality.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Quality(QualityId id) => new() { Key = id.Key };
    public static implicit operator Quality(QualityDiff diff) => new()
    {
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

#region Prop

/// <summary>
/// <see href="https://github.com/usalu/semio#-property-"/>
/// </summary>
[Model("🏷️", "Pp", "Prp", "A property is a value with an optional unit for a quality.")]
public class Prop : Model<Prop>
{
    [Id("🔑", "Ke", "Key", "The key of the quality of the property.")]
    public string Key { get; set; } = "";
    [Value("🔢", "Vl", "Val", "The value [ number | text ] of the property.")]
    public string Value { get; set; } = "";
    [Name("Ⓜ️", "Ut?", "Unt?", "The optional unit of the property.")]
    public string Unit { get; set; } = "";
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the property.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
}

#endregion Prop

#region Model

[Model("💾", "Rp", "Rep", "The identifier of a model.")]
public class ModelId : Model<ModelId>
{
    [Name("🏷️", "Tg*", "Tags*", "The optional tags to group models. No tags means default.", PropImportance.ID, skipValidation: true)]
    public List<string> Tags { get; set; } = new();
    public static implicit operator ModelId(Model model) => new() { Tags = model.Tags };
    public static implicit operator ModelId(ModelDiff diff) => new() { Tags = diff.Tags };
    public string ToIdString() => $"{string.Join(",", Tags.Select(t => Utility.Encode(t)))}";
    public string ToHumanIdString() => string.Join(", ", Tags);
    public override string ToString() => $"Rep({ToHumanIdString()})";
}

[Model("📊", "RD", "RDf", "A diff for models.")]
public class ModelDiff : Model<ModelDiff>
{
    [Name("📄", "Fl?", "Fil?", "The optional file path to the resource of the model.")]
    public string File { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the model.")]
    public string Description { get; set; } = "";
    [Name("🏷️", "Tg*", "Tags*", "The optional tags to group models.", PropImportance.OPTIONAL)]
    public List<string> Tags { get; set; } = new();
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the model.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator ModelDiff(ModelId id) => new() { Tags = id.Tags };
    public static implicit operator ModelDiff(Model model) => new() { File = model.File, Description = model.Description, Tags = model.Tags, Attributes = model.Attributes };

    public ModelDiff MergeDiff(ModelDiff other)
    {
        return new ModelDiff
        {
            File = string.IsNullOrEmpty(other.File) ? File : other.File,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Tags = other.Tags.Any() ? other.Tags : Tags,
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes
        };
    }
}

[Model("📊", "RsD", "RsDf", "A diff for multiple models.")]
public class ModelsDiff : Model<ModelsDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed models.", PropImportance.OPTIONAL)]
    public List<ModelId> Removed { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added models.", PropImportance.OPTIONAL)]
    public List<ModelDiff> Added { get; set; } = new();
    [ModelProp("✏️", "Md*", "Mod*", "The optional modified models.", PropImportance.OPTIONAL)]
    public List<ModelDiff> Modified { get; set; } = new();

    public static implicit operator ModelsDiff(List<Model> models) => new() { Modified = models.Select(r => (ModelDiff)r).ToList() };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-model-"/>
/// </summary>
[Model("💾", "Rp", "Rep",
    "A model is a link to a resource that describes a type for a certain level of detail and tags.")]
public class Model : Model<Model>
{
    [Name("📄", "Fl", "Fil", "The file path to the resource of the model.", PropImportance.REQUIRED)]
    public string File { get; set; } = "";

    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the model.")]
    public string Description { get; set; } = "";

    [Name("🏷️", "Tg*", "Tags*", "The optional tags to group models. No tags means default.", PropImportance.ID, skipValidation: true)]
    public List<string> Tags { get; set; } = new();

    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the model.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Model(ModelId id) => new() { Tags = id.Tags };
    public static implicit operator Model(ModelDiff diff) => new() { File = diff.File, Description = diff.Description, Tags = diff.Tags, Attributes = diff.Attributes };

    public Model ApplyDiff(ModelDiff diff)
    {
        return new Model
        {
            File = string.IsNullOrEmpty(diff.File) ? File : diff.File,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Tags = diff.Tags?.Any() == true ? diff.Tags : Tags,
            Attributes = diff.Attributes?.Any() == true ? diff.Attributes : Attributes
        };
    }

    public ModelDiff CreateDiff()
    {
        return new ModelDiff
        {
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
            File = !string.IsNullOrEmpty(appliedDiff.File) ? File : "",
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? Description : "",
            Tags = appliedDiff.Tags.Any() ? Tags : new List<string>(),
            Attributes = appliedDiff.Attributes.Any() ? Attributes : new List<Attribute>()
        };
    }

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

            foreach (var attribute in Attributes)
            {
                var (isValidAttribute, errorsAttribute) = attribute.Validate();
                isValid = isValid && isValidAttribute;
                errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
            }
        }

        return (isValid, errors);
    }

    public string ToIdString() => $"{string.Join(",", Tags.Select(t => Utility.Encode(t)))}";

    public string ToHumanIdString() => string.Join(", ", Tags);

    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();

    public override string ToString() => $"Rep({ToHumanIdString()})";
}

#endregion Model

#region Port

[Model("🔌", "Po", "Por", "The optional local identifier of the port within the type. No id means the default port.")]
public class PortId : Model<PortId>
{
    [Id("🆔", "Gd", "Gui", "The guid of the port within the type.")]
    public string Guid { get; set; } = "";
    public static implicit operator PortId(Port port) => new() { Guid = port.Guid };
    public static implicit operator PortId(PortDiff diff) => new() { Guid = diff.Guid ?? "" };
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"Por({ToHumanIdString()})";
}

[Model("📊", "PD", "PDf", "A diff for ports.")]
public class PortDiff : Model<PortDiff>
{
    [Id("🆔", "Gd?", "Gui?", "The optional guid of the port.")]
    public string? Guid { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the port.")]
    public string? Description { get; set; }
    [Name("👨‍👩‍👧‍👦", "Fa?", "Fam?", "The optional interface of the port.")]
    public string? Interface { get; set; }
    [FalseOrTrue("💯", "Ma?", "Man?", "Whether the port is mandatory.")]
    public bool? Mandatory { get; set; }
    [NumberProp("💍", "T?", "T?", "The optional parameter t [0,1[.")]
    public float? T { get; set; }
    [Name("✅", "CF*", "CFas*", "The optional other compatible interfaces of the port.", PropImportance.OPTIONAL)]
    public List<string>? CompatibleInterfaces { get; set; }
    [ModelProp("✖️", "Pt?", "Pnt?", "The optional connection point of the port.", PropImportance.OPTIONAL)]
    public Point? Point { get; set; }
    [ModelProp("➡️", "Dr?", "Drn?", "The optional direction of the port.", PropImportance.OPTIONAL)]
    public Vector? Direction { get; set; }
    [ModelProp("🏷️", "Pp*", "Prp*", "The optional properties of the port.", PropImportance.OPTIONAL)]
    public List<Prop>? Props { get; set; }
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the port.", PropImportance.OPTIONAL)]
    public List<Attribute>? Attributes { get; set; }

    public static implicit operator PortDiff(PortId id) => new() { Guid = id.Guid };
    public static implicit operator PortDiff(Port port) => new() { Guid = port.Guid, Description = port.Description, Interface = port.Interface, Mandatory = port.Mandatory, T = port.T, CompatibleInterfaces = port.CompatibleInterfaces, Point = port.Point, Direction = port.Direction, Props = port.Props, Attributes = port.Attributes };

    public PortDiff MergeDiff(PortDiff other)
    {
        return new PortDiff
        {
            Guid = other.Guid ?? Guid,
            Description = other.Description ?? Description,
            Interface = other.Interface ?? Interface,
            Mandatory = other.Mandatory ?? Mandatory,
            T = other.T ?? T,
            CompatibleInterfaces = other.CompatibleInterfaces ?? CompatibleInterfaces,
            Point = other.Point ?? Point,
            Direction = other.Direction ?? Direction,
            Props = other.Props ?? Props,
            Attributes = other.Attributes ?? Attributes
        };
    }
}

[Model("📊", "PsD", "PsDf", "A diff for multiple ports.")]
public class PortsDiff : Model<PortsDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed ports.", PropImportance.OPTIONAL)]
    public List<PortId> Removed { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added ports.", PropImportance.OPTIONAL)]
    public List<PortDiff> Added { get; set; } = new();
    [ModelProp("✏️", "Md*", "Mod*", "The optional modified ports.", PropImportance.OPTIONAL)]
    public List<PortDiff> Modified { get; set; } = new();

    public static implicit operator PortsDiff(List<Port> ports) => new() { Modified = ports.Select(p => (PortDiff)p).ToList() };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-port-"/>
/// </summary>
[Model("🔌", "Po", "Por", "A port is a connection point (with a direction) of a type.")]
public class Port : Model<Port>
{
    [Id("🆔", "Gd", "Gui", "The guid of the port within the type.")]
    public string Guid { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the port.")]
    public string Description { get; set; } = "";
    [FalseOrTrue("💯", "Ma?", "Man?", "Whether the port is mandatory. A mandatory port must be connected in a design.")]
    public bool Mandatory { get; set; } = false;
    [Name("👨‍👩‍👧‍👦", "Fa?", "Fam?", "The optional interface of the port. This allows to define explicit compatibility with other ports.")]
    public string Interface { get; set; } = "";
    [Name("✅", "CF*", "CFas*", "The optional other compatible interfaces of the port. An empty list means this port is compatible with all other ports.")]
    public List<string> CompatibleInterfaces { get; set; } = new();
    [ModelProp("✖️", "Pt", "Pnt", "The connection point of the port that is attracted to another connection point.")]
    public Point? Point { get; set; } = null;
    [ModelProp("➡️", "Dr", "Drn", "The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned.")]
    public Vector? Direction { get; set; } = null;
    [NumberProp("💍", "T", "T", "The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise.", PropImportance.REQUIRED)]
    public float T { get; set; } = 0;
    [ModelProp("🏷️", "Pp*", "Prp*", "The optional properties of the port.", PropImportance.OPTIONAL)]
    public List<Prop> Props { get; set; } = new();
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the port.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Por({ToHumanIdString()})";

    public static implicit operator Port(PortId id) => new() { Guid = id.Guid };
    public static implicit operator Port(PortDiff diff) => new() { Guid = diff.Guid ?? "", Description = diff.Description ?? "", Interface = diff.Interface ?? "", Mandatory = diff.Mandatory ?? false, T = diff.T ?? 0, CompatibleInterfaces = diff.CompatibleInterfaces ?? new(), Point = diff.Point, Direction = diff.Direction, Attributes = diff.Attributes ?? new() };
    public static implicit operator string(Port port) => port.Guid;
    public static implicit operator Port(string guid) => new() { Guid = guid };

    public Port ApplyDiff(PortDiff diff)
    {
        return new Port
        {
            Guid = diff.Guid ?? Guid,
            Description = diff.Description ?? Description,
            Interface = diff.Interface ?? Interface,
            Mandatory = diff.Mandatory ?? Mandatory,
            T = diff.T ?? T,
            CompatibleInterfaces = diff.CompatibleInterfaces ?? CompatibleInterfaces,
            Point = diff.Point ?? Point,
            Direction = diff.Direction ?? Direction,
            Props = diff.Props ?? Props,
            Attributes = diff.Attributes ?? Attributes
        };
    }

    public PortDiff CreateDiff()
    {
        return new PortDiff
        {
            Guid = Guid,
            Description = Description,
            Interface = Interface,
            Mandatory = Mandatory,
            T = T,
            CompatibleInterfaces = CompatibleInterfaces,
            Point = Point,
            Direction = Direction,
            Props = Props,
            Attributes = Attributes
        };
    }

    public PortDiff InverseDiff(PortDiff appliedDiff)
    {
        return new PortDiff
        {
            Guid = !string.IsNullOrEmpty(appliedDiff.Guid) ? Guid : "",
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? Description : "",
            Interface = !string.IsNullOrEmpty(appliedDiff.Interface) ? Interface : "",
            Mandatory = appliedDiff.Mandatory.HasValue ? Mandatory : null,
            T = appliedDiff.T.HasValue ? T : null,
            CompatibleInterfaces = appliedDiff.CompatibleInterfaces?.Any() == true ? CompatibleInterfaces : new List<string>(),
            Point = appliedDiff.Point is not null ? Point : null,
            Direction = appliedDiff.Direction is not null ? Direction : null,
            Props = appliedDiff.Props?.Any() == true ? Props : new List<Prop>(),
            Attributes = appliedDiff.Attributes?.Any() == true ? Attributes : new List<Attribute>()
        };
    }

    // TODO: Implement reflexive validation for model properties.
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

    public bool IsCompatibleWith(Port otherPort)
    {
        var normalizedPortInterface = Utility.Normalize(Interface);
        var normalizedOtherPortInterface = Utility.Normalize(otherPort.Interface);
        if (normalizedPortInterface == "" || normalizedOtherPortInterface == "") return true;
        return (CompatibleInterfaces ?? new List<string>()).Contains(normalizedOtherPortInterface) ||
               (otherPort.CompatibleInterfaces ?? new List<string>()).Contains(normalizedPortInterface);
    }

    public bool IsSameAs(Port other)
    {
        return Utility.Normalize(Guid) == Utility.Normalize(other.Guid);
    }

    public string FindAttributeValue(string name, string defaultValue = "")
    {
        var attribute = Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in port {Guid}");
        return attribute?.Value ?? defaultValue;
    }

    public Port SetAttribute(Attribute attribute)
    {
        var attributes = new List<Attribute>(Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Port
        {
            Guid = Guid,
            Description = Description,
            Mandatory = Mandatory,
            Interface = Interface,
            CompatibleInterfaces = CompatibleInterfaces,
            Point = Point,
            Direction = Direction,
            T = T,
            Props = Props,
            Attributes = attributes
        };
    }
}

#endregion Port

#region Type

[Model("🧩", "Ty", "Typ", "The identifier of the type within the kit.")]
public class TypeId : Model<TypeId>
{
    [Id("🆔", "Gd", "Gui", "The guid of the type.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Guid}";
    public override string ToString() => $"Typ({ToHumanIdString()})";
    public static implicit operator TypeId(Type type) => new() { Guid = type.Guid };
    public static implicit operator TypeId(TypeDiff diff) => new() { Guid = diff.Guid ?? "" };
}

[Model("🧩", "TD", "TDf", "A diff for types.")]
public class TypeDiff : Model<TypeDiff>
{
    [Id("🆔", "Gd?", "Gui?", "The optional guid of the type.")]
    public string? Guid { get; set; }
    [Name("📛", "Na?", "Nam?", "The optional name of the type.")]
    public string? Name { get; set; }
    [Id("📁", "Pa?", "Par?", "The optional parent type guid.")]
    public string? Parent { get; set; }
    [FalseOrTrue("👻", "IA?", "IsA?", "Whether the type is abstract.")]
    public bool? IsAbstract { get; set; }
    [Id("📁", "Fo?", "Fol?", "The optional folder guid.")]
    public string? Folder { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the type.")]
    public string? Description { get; set; }
    [Url("🪙", "Ic?", "Ico?", "The optional icon of the type.")]
    public string? Icon { get; set; }
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the type.")]
    public string? Image { get; set; }
    [IntProp("📦", "St?", "Stk?", "The optional number of items in stock.")]
    public int? Stock { get; set; }
    [FalseOrTrue("👻", "Vi?", "Vir?", "Whether the type is virtual.")]
    public bool? Virtual { get; set; }
    [Url("🔗", "Ur?", "Uri?", "The optional Unique Resource Identifier (URI) of the type.")]
    public string Uri { get; set; } = "";
    [Name("Ⓜ️", "Ut?", "Unt?", "The optional length unit of the type.")]
    public string Unit { get; set; } = "";
    [ModelProp("📍", "Lo?", "Loc?", "The optional location of the type.", PropImportance.OPTIONAL)]
    public Location? Location { get; set; }
    [ModelProp("💾", "Rp*", "Reps*", "The optional models of the type.", PropImportance.OPTIONAL)]
    public List<Model> Models { get; set; } = new();
    [ModelProp("🔌", "Po*", "Pors*", "The optional ports of the type.", PropImportance.OPTIONAL)]
    public List<Port> Ports { get; set; } = new();
    [ModelProp("👥", "Au*", "Aut*", "The optional authors of the type.", PropImportance.OPTIONAL)]
    public List<AuthorId> Authors { get; set; } = new();
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the type.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
    [ModelProp("💡", "Co*", "Con*", "The optional concepts of the type.", PropImportance.OPTIONAL)]
    public List<string> Concepts { get; set; } = new();
    [Name("📅", "CA?", "CrA?", "The optional created at timestamp of the type.")]
    public DateTime? CreatedAt { get; set; }
    [Name("📅", "UA?", "UpA?", "The optional updated at timestamp of the type.")]
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
            Models = other.Models.Any() ? other.Models : Models,
            Ports = other.Ports.Any() ? other.Ports : Ports,
            Authors = other.Authors.Any() ? other.Authors : Authors,
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes,
            Concepts = other.Concepts.Any() ? other.Concepts : Concepts
        };
    }

    public static implicit operator TypeDiff(TypeId id) => new() { Guid = id.Guid };
    public static implicit operator TypeDiff(Type type) => new() { Name = type.Name, Description = type.Description, Icon = type.Icon, Image = type.Image, Stock = type.Stock, Virtual = type.Virtual, Uri = type.Uri, Unit = type.Unit, Location = type.Location, Models = type.Models, Ports = type.Ports, Authors = type.Authors, Attributes = type.Attributes, Concepts = type.Concepts };
}

[Model("📊", "TsD", "TsDf", "A diff for multiple types.")]
public class TypesDiff : Model<TypesDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed types.", PropImportance.OPTIONAL)]
    public List<TypeId> Removed { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added types.", PropImportance.OPTIONAL)]
    public List<TypeDiff> Added { get; set; } = new();
    [ModelProp("✏️", "Md*", "Mod*", "The optional modified types.", PropImportance.OPTIONAL)]
    public List<TypeDiff> Modified { get; set; } = new();

    public static implicit operator TypesDiff(List<Type> types) => new() { Modified = types.Select(t => (TypeDiff)t).ToList() };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-type-"/>
/// </summary>
[Model("🧩", "Ty", "Typ", "A type is a reusable element that can be connected with other types over ports.")]
public class Type : Model<Type>
{
    [Id("🆔", "Gd", "Gui", "The guid of the type.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    [Name("📛", "Na", "Nam", "The name of the type.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Id("📁", "Pa?", "Par?", "The optional parent type guid.")]
    public string? Parent { get; set; }
    [FalseOrTrue("👻", "IA?", "IsA?", "Whether the type is abstract.")]
    public bool? IsAbstract { get; set; }
    [Id("📁", "Fo?", "Fol?", "The optional folder guid.")]
    public string? Folder { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the type.")]
    public string Description { get; set; } = "";
    [Url("🪙", "Ic?", "Ico?", "The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.")]
    public string Icon { get; set; } = "";
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.")]
    public string Image { get; set; } = "";
    [IntProp("📦", "St?", "Stk?", "The optional number of items in stock. 2147483647 (=2^31-1) means infinite stock.")]
    public int Stock { get; set; } = 2147483647;
    [FalseOrTrue("👻", "Vi?", "Vir?", "Whether the type is virtual. A virtual type is not physically present but is used in conjunction with other virtual types to form a larger physical type.")]
    public bool Virtual { get; set; } = false;
    [Url("🔗", "Ur?", "Uri?", "The optional Unique Resource Identifier (URI) of the type.")]
    public string Uri { get; set; } = "";
    [ModelProp("📍", "Lo?", "Loc?", "The optional location of the type.", PropImportance.OPTIONAL)]
    public Location? Location { get; set; }
    [Name("Ⓜ️", "Ut", "Unt", "The length unit of the point and the direction of the ports of the type.", PropImportance.REQUIRED)]
    public string Unit { get; set; } = "";
    [ModelProp("💾", "Rp*", "Reps*", "The optional models of the type.", PropImportance.OPTIONAL)]
    public List<Model> Models { get; set; } = new();
    [ModelProp("🔌", "Po*", "Pors*", "The optional ports of the type.", PropImportance.OPTIONAL)]
    public List<Port> Ports { get; set; } = new();
    [ModelProp("🏷️", "Pp*", "Prp*", "The optional properties of the type.", PropImportance.OPTIONAL)]
    public List<Prop> Props { get; set; } = new();
    [ModelProp("👥", "Au*", "Aut*", "The optional authors of the type.", PropImportance.OPTIONAL)]
    public List<AuthorId> Authors { get; set; } = new();
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the type.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
    [ModelProp("💡", "Co*", "Con*", "The optional concepts of the type.", PropImportance.OPTIONAL)]
    public List<string> Concepts { get; set; } = new();
    [Name("📅", "CA", "CrA", "The created at timestamp of the type.", PropImportance.REQUIRED)]
    public DateTime CreatedAt { get; set; }
    [Name("📅", "UA", "UpA", "The updated at timestamp of the type.", PropImportance.REQUIRED)]
    public DateTime UpdatedAt { get; set; }

    public string ToIdString() => $"{Guid}";

    public string ToHumanIdString() => $"{Name}";

    public override string ToString() => $"Typ({ToHumanIdString()})";

    public static implicit operator Type(TypeId id) => new() { Guid = id.Guid, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };
    public static implicit operator Type(TypeDiff diff) => new() { Guid = diff.Guid ?? "", Name = diff.Name ?? "", Parent = diff.Parent, IsAbstract = diff.IsAbstract, Folder = diff.Folder, Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Stock = diff.Stock ?? 2147483647, Virtual = diff.Virtual ?? false, Uri = diff.Uri ?? "", Unit = diff.Unit ?? "", Location = diff.Location, Models = diff.Models ?? new(), Ports = diff.Ports ?? new(), Authors = diff.Authors ?? new(), Attributes = diff.Attributes ?? new(), Concepts = diff.Concepts ?? new(), CreatedAt = diff.CreatedAt ?? DateTime.UtcNow, UpdatedAt = diff.UpdatedAt ?? DateTime.UtcNow };
    public static implicit operator string(Type type) => type.Name;
    public static implicit operator Type(string name) => new() { Name = name, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };

    public Type ApplyDiff(TypeDiff diff)
    {
        return new Type
        {
            Name = string.IsNullOrEmpty(diff.Name) ? Name : diff.Name,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Icon = string.IsNullOrEmpty(diff.Icon) ? Icon : diff.Icon,
            Image = string.IsNullOrEmpty(diff.Image) ? Image : diff.Image,
            Stock = diff.Stock ?? Stock,
            Virtual = diff.Virtual ?? Virtual,
            Uri = string.IsNullOrEmpty(diff.Uri) ? Uri : diff.Uri,
            Unit = string.IsNullOrEmpty(diff.Unit) ? Unit : diff.Unit,
            Location = diff.Location ?? Location,
            Models = diff.Models?.Any() == true ? diff.Models : Models,
            Ports = diff.Ports?.Any() == true ? diff.Ports : Ports,
            Authors = diff.Authors?.Any() == true ? diff.Authors : Authors,
            Attributes = diff.Attributes?.Any() == true ? diff.Attributes : Attributes,
            Concepts = diff.Concepts?.Any() == true ? diff.Concepts : Concepts,
            Props = Props
        };
    }

    public TypeDiff CreateDiff()
    {
        return new TypeDiff
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Stock = Stock,
            Virtual = Virtual,
            Uri = Uri,
            Unit = Unit,
            Location = Location,
            Models = Models,
            Ports = Ports,
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
            Models = appliedDiff.Models.Any() ? Models : new List<Model>(),
            Ports = appliedDiff.Ports.Any() ? Ports : new List<Port>(),
            Authors = appliedDiff.Authors.Any() ? Authors : new List<AuthorId>(),
            Attributes = appliedDiff.Attributes.Any() ? Attributes : new List<Attribute>()
        };
    }

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

    public Port FindPort(string portId)
    {
        var port = Ports?.FirstOrDefault(p => Utility.Normalize(p.Guid) == Utility.Normalize(portId));
        if (port is null) throw new InvalidOperationException($"Port {portId} not found in type {Name}");
        return port;
    }

    public Model FindModel(List<string> tags)
    {
        if (Models == null || Models.Count == 0)
            throw new ArgumentException($"No models available in type {Name}");

        var indices = Models.Select(r => Utility.Jaccard(r.Tags, tags)).ToList();
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
            Ports = Ports,
            Props = Props,
            Authors = Authors,
            Attributes = attributes
        };
    }
}

#endregion Type

#region Layer

/// <summary>
/// <see href="https://github.com/usalu/semio#-layer-"/>
/// </summary>
[Model("📄", "Ly", "Lyr", "A layer for organizing design elements.")]
public class Layer : Model<Layer>
{
    [Name("📛", "Nm", "Nam", "The name of the layer.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the layer.")]
    public string Description { get; set; } = "";
    [Color("🎨", "Cl?", "Col?", "The hex color of the layer.")]
    public string Color { get; set; } = "";

    public string ToIdString() => $"{Name}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Lyr({ToHumanIdString()})";
}

#endregion Layer

#region Group

/// <summary>
/// <see href="https://github.com/usalu/semio#-group-"/>
/// </summary>
[Model("📁", "Gr", "Grp", "A group for organizing design elements.")]
public class Group : Model<Group>
{
    [Name("📛", "Nm", "Nam", "The optional name of the group.", PropImportance.OPTIONAL)]
    public string Name { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the group.")]
    public string Description { get; set; } = "";
    [ModelProp("⭕", "Pc*", "Pcs*", "The pieces in the group.", PropImportance.REQUIRED)]
    public List<PieceId> Pieces { get; set; } = new();
    [Color("🎨", "Cl?", "Col?", "The optional hex color of the group.")]
    public string Color { get; set; } = "";
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the group.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Name}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Grp({ToHumanIdString()})";
}

#endregion Group

#region Piece

[Model("⭕", "Pc", "Pce", "The optional local identifier of the piece within the design. No id means the default piece.")]
public class PieceId : Model<PieceId>
{
    [Id("🆔", "Gd", "Gui", "The guid of the piece.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    public static implicit operator PieceId(PieceDiff diff) => new() { Guid = diff.Guid ?? "" };
    public static implicit operator PieceId(Piece piece) => new() { Guid = piece.Guid };
}

[Model("📊", "PD", "PDf", "A diff for pieces.")]
public class PiecesDiff : Model<PiecesDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed pieces.", PropImportance.OPTIONAL)]
    public List<PieceId> Removed { get; set; } = new();
    [ModelProp("✏️", "Up*", "Upd*", "The optional updated pieces.", PropImportance.OPTIONAL)]
    public List<PieceDiff> Modified { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added pieces.", PropImportance.OPTIONAL)]
    public List<PieceDiff> Added { get; set; } = new();

    public PiecesDiff MergeDiff(PiecesDiff other)
    {
        return new PiecesDiff
        {
            Removed = other.Removed.Concat(Removed).Distinct().ToList(),
            Modified = other.Modified.Concat(Modified).GroupBy(m => m.Guid).Select(g => g.Last()).ToList(),
            Added = other.Added.Concat(Added).GroupBy(a => a.Guid).Select(g => g.Last()).ToList()
        };
    }

    public static implicit operator PiecesDiff(List<Piece> pieces) => new() { Modified = pieces.Select(p => p.CreateDiff()).ToList() };
}

[Model("📊", "PcD", "PcDf", "A diff for a piece.")]
public class PieceDiff : Model<PieceDiff>
{
    [Id("🆔", "Gd?", "Gui?", "The optional guid of the piece.")]
    public string? Guid { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the piece.")]
    public string? Description { get; set; }
    [ModelProp("🧩", "Ty?", "Typ?", "The optional type of the piece.", PropImportance.OPTIONAL)]
    public TypeId? Type { get; set; }
    [ModelProp("📺", "Pl?", "Pln?", "The optional plane of the piece.", PropImportance.OPTIONAL)]
    public Plane? Plane { get; set; }
    [ModelProp("📺", "Ce?", "Cnt?", "The optional center of the piece.", PropImportance.OPTIONAL)]
    public Coord? Center { get; set; }
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the piece.", PropImportance.OPTIONAL)]
    public List<Attribute>? Attributes { get; set; }

    public static implicit operator PieceDiff(PieceId id) => new() { Guid = id.Guid };
    public static implicit operator PieceDiff(Piece piece) => new() { Guid = piece.Guid, Description = piece.Description, Type = piece.Type, Plane = piece.Plane, Center = piece.Center, Attributes = piece.Attributes };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-piece-"/>
/// </summary>
[Model("⭕", "Pc", "Pce", "A piece is an instance of either a type or a design.")]
public class Piece : Model<Piece>
{
    [Id("🆔", "Gd", "Gui", "The guid of the piece.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the piece.")]
    public string Description { get; set; } = "";
    [ModelProp("🧩", "Ty?", "Typ?", "The optional type of the piece.", PropImportance.OPTIONAL)]
    public TypeId? Type { get; set; }
    [ModelProp("📺", "Pl?", "Pln?", "The optional plane of the piece.", PropImportance.OPTIONAL)]
    public Plane? Plane { get; set; }
    [ModelProp("📺", "Ce?", "Cnt?", "The optional center of the piece.", PropImportance.OPTIONAL)]
    public Coord? Center { get; set; }
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the piece.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Guid}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    public static implicit operator Piece(PieceId id) => new() { Guid = id.Guid };
    public static implicit operator Piece(PieceDiff diff) => new() { Guid = diff.Guid ?? "", Description = diff.Description ?? "", Type = diff.Type, Plane = diff.Plane, Center = diff.Center, Attributes = diff.Attributes ?? new() };

    public Piece ApplyDiff(PieceDiff diff)
    {
        return new Piece
        {
            Guid = diff.Guid ?? Guid,
            Description = diff.Description ?? Description,
            Type = diff.Type ?? Type,
            Plane = diff.Plane ?? Plane,
            Center = diff.Center ?? Center,
            Attributes = diff.Attributes ?? Attributes
        };
    }

    public PieceDiff CreateDiff()
    {
        return new PieceDiff
        {
            Guid = Guid,
            Description = Description,
            Type = Type,
            Plane = Plane,
            Center = Center,
            Attributes = Attributes
        };
    }
}

#endregion Piece
#region Side

[Model("📊", "SD", "SDf", "A diff for sides.")]
public class SideDiff : Model<SideDiff>
{
    [ModelProp("⭕", "Pc?", "Pce?", "The optional piece of the side.", PropImportance.OPTIONAL)]
    public PieceId? Piece { get; set; }
    [ModelProp("🏙️", "DP?", "DPc?", "The optional id of the piece inside the referenced design piece.", PropImportance.OPTIONAL)]
    public PieceId? DesignPiece { get; set; } = null;
    [ModelProp("🔌", "Po?", "Por?", "The optional port of the side.", PropImportance.OPTIONAL)]
    public PortId? Port { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the side.")]
    public string Description { get; set; } = "";

    public static implicit operator SideDiff(Side side) => new() { Piece = side.Piece, DesignPiece = side.DesignPiece, Port = side.Port };

    public SideDiff MergeDiff(SideDiff other)
    {
        return new SideDiff
        {
            Piece = other.Piece ?? Piece,
            DesignPiece = other.DesignPiece ?? DesignPiece,
            Port = other.Port ?? Port,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description
        };
    }
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-side-"/>
/// </summary>
[Model("🧱", "Sd", "Sde", "A side of a piece in a connection.")]
public class Side : Model<Side>
{
    [ModelProp("⭕", "Pc", "Pce", "The piece-related information of the side.")]
    public PieceId Piece { get; set; } = new();
    [ModelProp("🏙️", "DP?", "DPc?", "The optional id of the piece inside the referenced design piece.", PropImportance.OPTIONAL)]
    public PieceId? DesignPiece { get; set; } = null;
    [ModelProp("🔌", "Po", "Por", "The local identifier of the port within the type.")]
    public PortId Port { get; set; } = new();

    public static implicit operator Side(SideDiff diff) => new() { Piece = diff.Piece ?? new(), DesignPiece = diff.DesignPiece, Port = diff.Port ?? new() };

    public Side ApplyDiff(SideDiff diff)
    {
        return new Side
        {
            Piece = diff.Piece ?? Piece,
            DesignPiece = diff.DesignPiece ?? DesignPiece,
            Port = diff.Port ?? Port
        };
    }

    public SideDiff CreateDiff()
    {
        return new SideDiff
        {
            Piece = Piece,
            DesignPiece = DesignPiece,
            Port = Port
        };
    }

    public SideDiff InverseDiff(SideDiff appliedDiff)
    {
        return new SideDiff
        {
            Piece = appliedDiff.Piece is not null ? Piece : null,
            DesignPiece = appliedDiff.DesignPiece is not null ? DesignPiece : null,
            Port = appliedDiff.Port is not null ? Port : null
        };
    }

    public override string ToString() => $"Sde({Piece.Guid}" + (Port.Guid != "" ? ":" + Port.Guid : "") + ")";
}

#endregion Side

#region Connection

[Model("🧲", "Cn", "ConId", "The local identifier of the connection within the design.")]
public class ConnectionId : Model<ConnectionId>
{
    [ModelProp("🧲", "Cd", "Cnd", "The connected side of the piece.")]
    public Side Connected { get; set; } = new();
    [ModelProp("🧲", "Cg", "Cng", "The connecting side of the piece.")]
    public Side Connecting { get; set; } = new();

    public string ToIdString() => $"{Connected.Piece.Guid + (Connected.Port.Guid != "" ? ":" + Connected.Port.Guid : "")}--{(Connecting.Port.Guid != "" ? Connecting.Port.Guid + ":" : "") + Connecting.Piece.Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"ConId({ToHumanIdString()})";

    public static implicit operator ConnectionId(Connection connection) => new() { Connected = connection.Connected, Connecting = connection.Connecting };
    public static implicit operator ConnectionId(ConnectionDiff diff) => new() { Connected = diff.Connected ?? new(), Connecting = diff.Connecting ?? new() };
}

[Model("🔗", "CD", "CDf", "A diff for connections.")]
public class ConnectionDiff : Model<ConnectionDiff>
{
    [ModelProp("🧲", "Cd?", "Cnd?", "The optional connected side of the piece.", PropImportance.OPTIONAL)]
    public SideDiff? Connected { get; set; }
    [ModelProp("🧲", "Cg?", "Cng?", "The optional connecting side of the piece.", PropImportance.OPTIONAL)]
    public SideDiff? Connecting { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the connection.")]
    public string Description { get; set; } = "";
    [NumberProp("↕️", "Gp?", "Gap?", "The optional longitudinal gap.")]
    public float? Gap { get; set; }
    [NumberProp("↔️", "Sf?", "Shf?", "The optional lateral shift.")]
    public float? Shift { get; set; }
    [NumberProp("⬆️", "Rs?", "Rse?", "The optional vertical rise.")]
    public float? Rise { get; set; }
    [NumberProp("🔄", "Rt?", "Rot?", "The optional rotation around the y-axis.")]
    public float? Rotation { get; set; }
    [NumberProp("🔄", "Tn?", "Trn?", "The optional turn around the z-axis.")]
    public float? Turn { get; set; }
    [NumberProp("🔄", "Tl?", "Tlt?", "The optional tilt around the x-axis.")]
    public float? Tilt { get; set; }
    [NumberProp("↔️", "X?", "X?", "The optional x offset for diagram positioning.")]
    public float? X { get; set; }
    [NumberProp("↕️", "Y?", "Y?", "The optional y offset for diagram positioning.")]
    public float? Y { get; set; }
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the connection.", PropImportance.OPTIONAL)]
    public List<Attribute>? Attributes { get; set; }

    public static implicit operator ConnectionDiff(ConnectionId id) => new() { Connected = new SideDiff { Piece = id.Connected.Piece, DesignPiece = id.Connected.DesignPiece, Port = id.Connected.Port }, Connecting = new SideDiff { Piece = id.Connecting.Piece, DesignPiece = id.Connecting.DesignPiece, Port = id.Connecting.Port } };
    public static implicit operator ConnectionDiff(Connection connection) => new() { Connected = connection.Connected.CreateDiff(), Connecting = connection.Connecting.CreateDiff(), Description = connection.Description, Gap = connection.Gap, Shift = connection.Shift, Rise = connection.Rise, Rotation = connection.Rotation, Turn = connection.Turn, Tilt = connection.Tilt, X = connection.X, Y = connection.Y, Attributes = connection.Attributes };

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
            X = other.X ?? X,
            Y = other.Y ?? Y,
            Attributes = other.Attributes ?? Attributes
        };
    }
}

[Model("🔗", "CsD", "ConsDf", "A diff for multiple connections.")]
public class ConnectionsDiff : Model<ConnectionsDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed connections.", PropImportance.OPTIONAL)]
    public List<ConnectionId> Removed { get; set; } = new();
    [ModelProp("✏️", "Up*", "Upd*", "The optional updated connections.", PropImportance.OPTIONAL)]
    public List<ConnectionDiff> Updated { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added connections.", PropImportance.OPTIONAL)]
    public List<Connection> Added { get; set; } = new();

    public static implicit operator ConnectionsDiff(List<Connection> connections) => new() { Updated = connections.Select(c => (ConnectionDiff)c).ToList() };

    public ConnectionsDiff MergeDiff(ConnectionsDiff other)
    {
        return new ConnectionsDiff
        {
            Removed = other.Removed.Concat(Removed).Distinct().ToList(),
            Updated = other.Updated.Concat(Updated).GroupBy(u => u.Connected?.Piece?.Guid + "--" + u.Connecting?.Piece?.Guid).Select(g => g.Last()).ToList(),
            Added = other.Added.Concat(Added).GroupBy(a => a.Connected.Piece.Guid + "--" + a.Connecting.Piece.Guid).Select(g => g.Last()).ToList()
        };
    }
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-connection-"/>
/// </summary>
[Model("🔗", "Cn", "Con", "A connection between two pieces.")]
public class Connection : Model<Connection>
{
    [ModelProp("🧲", "Cd", "Cnd", "The connected side of the piece.")]
    public Side Connected { get; set; } = new();
    [ModelProp("🧲", "Cg", "Cng", "The connecting side of the piece.")]
    public Side Connecting { get; set; } = new();
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the connection.")]
    public string Description { get; set; } = "";
    [NumberProp("↕️", "Gp", "Gap", "The longitudinal gap.")]
    public float Gap { get; set; } = 0;
    [NumberProp("↔️", "Sf", "Shf", "The lateral shift.")]
    public float Shift { get; set; } = 0;
    [NumberProp("⬆️", "Rs", "Rse", "The vertical rise.")]
    public float Rise { get; set; } = 0;
    [NumberProp("🔄", "Rt", "Rot", "The rotation around the y-axis.")]
    public float Rotation { get; set; } = 0;
    [NumberProp("🔄", "Tn", "Trn", "The turn around the z-axis.")]
    public float Turn { get; set; } = 0;
    [NumberProp("🔄", "Tl", "Tlt", "The tilt around the x-axis.")]
    public float Tilt { get; set; } = 0;
    [NumberProp("↔️", "X", "X", "The x offset for diagram positioning.")]
    public float X { get; set; } = 0;
    [NumberProp("↕️", "Y", "Y", "The y offset for diagram positioning.")]
    public float Y { get; set; } = 0;
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the connection.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Connected.Piece.Guid + (Connected.Port.Guid != "" ? ":" + Connected.Port.Guid : "")}--{(Connecting.Port.Guid != "" ? Connecting.Port.Guid + ":" : "") + Connecting.Piece.Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Con({ToHumanIdString()})";

    public static implicit operator Connection(ConnectionId id) => new() { Connected = id.Connected, Connecting = id.Connecting };
    public static implicit operator Connection(ConnectionDiff diff) => new() { Connected = diff.Connected ?? new(), Connecting = diff.Connecting ?? new(), Description = diff.Description ?? "", Gap = diff.Gap ?? 0, Shift = diff.Shift ?? 0, Rise = diff.Rise ?? 0, Rotation = diff.Rotation ?? 0, Turn = diff.Turn ?? 0, Tilt = diff.Tilt ?? 0, X = diff.X ?? 0, Y = diff.Y ?? 0, Attributes = diff.Attributes ?? new() };

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
            X = diff.X ?? X,
            Y = diff.Y ?? Y,
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
            X = X,
            Y = Y,
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
            X = appliedDiff.X.HasValue ? X : null,
            Y = appliedDiff.Y.HasValue ? Y : null,
            Attributes = appliedDiff.Attributes is not null ? Attributes : null
        };
    }

    public bool IsSameAs(Connection other, bool strict = false)
    {
        if (other is null) return false;
        if (strict)
        {
            return Connected.Piece.Guid == other.Connected.Piece.Guid &&
                   Connected.Port.Guid == other.Connected.Port.Guid &&
                   Connecting.Piece.Guid == other.Connecting.Piece.Guid &&
                   Connecting.Port.Guid == other.Connecting.Port.Guid;
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
            X = X,
            Y = Y,
            Attributes = attributes
        };
    }
}

#endregion Connection

#region Stat

/// <summary>
/// <see href="https://github.com/usalu/semio#-stat-"/>
/// </summary>
[Model("🔢", "St", "Stt", "A stat about a quality on a design which is optionally bounded.")]
public class Stat : Model<Stat>
{
    [Id("🔑", "Ke", "Key", "The key of the stat.")]
    public string Key { get; set; } = "";
    [Name("Ⓜ️", "Ut?", "Unt?", "The optional unit of the stat.")]
    public string Unit { get; set; } = "";
    [NumberProp("⬇️", "Mi?", "Min?", "The optional minimum value of the stat.")]
    public float Min { get; set; } = 0;
    [FalseOrTrue("⬇️", "MiE?", "MiE?", "Whether the minimum value is excluded from the range.")]
    public bool MinExcluded { get; set; } = false;
    [NumberProp("⬆️", "Mx?", "Max?", "The optional maximum value of the stat.")]
    public float Max { get; set; } = 0;
    [FalseOrTrue("⬆️", "MxE?", "MxE?", "Whether the maximum value is excluded from the range.")]
    public bool MaxExcluded { get; set; } = false;
}

#endregion Stat

#region Design

[Model("📊", "DsD", "DsDf", "A diff for multiple designs.")]
public class DesignsDiff : Model<DesignsDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed designs.", PropImportance.OPTIONAL)]
    public List<DesignId> Removed { get; set; } = new();
    [ModelProp("✏️", "Up*", "Upd*", "The optional updated designs.", PropImportance.OPTIONAL)]
    public List<DesignDiff> Updated { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added designs.", PropImportance.OPTIONAL)]
    public List<Design> Added { get; set; } = new();

    public static implicit operator DesignsDiff(List<Design> designs) => new() { Updated = designs.Select(d => (DesignDiff)d).ToList() };
}

[Model("🏙️", "Dn", "Dsn", "The local identifier of the design within the kit.")]
public class DesignDiff : Model<DesignDiff>
{
    [Id("🆔", "Gd?", "Gui?", "The optional guid of the design.")]
    public string? Guid { get; set; }
    [Name("📛", "Na?", "Nam?", "The optional name of the design.")]
    public string? Name { get; set; }
    [Id("📁", "Pa?", "Par?", "The optional parent design guid.")]
    public string? Parent { get; set; }
    [FalseOrTrue("👻", "IA?", "IsA?", "Whether the design is abstract.")]
    public bool? IsAbstract { get; set; }
    [Id("📁", "Fo?", "Fol?", "The optional folder guid.")]
    public string? Folder { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the design.")]
    public string? Description { get; set; }
    [Url("🪙", "Ic?", "Ico?", "The optional icon [ emoji | logogram | url ] of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.")]
    public string? Icon { get; set; }
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.")]
    public string? Image { get; set; }
    [ModelProp("📍", "Lo?", "Loc?", "The optional location of the design.", PropImportance.OPTIONAL)]
    public Location? Location { get; set; }
    [Name("Ⓜ️", "Ut?", "Unt?", "The optional length unit for all distance-related information of the design.")]
    public string? Unit { get; set; }
    [FalseOrTrue("⚖️", "CS?", "CanS?", "Whether the design can be scaled.")]
    public bool? CanScale { get; set; }
    [FalseOrTrue("🪞", "CM?", "CanM?", "Whether the design can be mirrored.")]
    public bool? CanMirror { get; set; }
    [Id("🔖", "AL?", "ActL?", "The optional active layer guid.")]
    public string? ActiveLayer { get; set; }
    [ModelProp("⭕", "Pc*", "Pcs*", "The optional pieces of the design.", PropImportance.OPTIONAL)]
    public PiecesDiff? Pieces { get; set; }
    [ModelProp("🔗", "Co*", "Cons*", "The optional connections of the design.", PropImportance.OPTIONAL)]
    public ConnectionsDiff? Connections { get; set; }
    [ModelProp("🏷️", "Pp*", "Prp*", "The optional properties of the design.", PropImportance.OPTIONAL)]
    public List<Prop>? Props { get; set; }
    [ModelProp("🔢", "St*", "Stt*", "The optional stats of the design.", PropImportance.OPTIONAL)]
    public List<Stat>? Stats { get; set; }
    [ModelProp("🔗", "Ly*", "Lyr*", "The optional layers of the design.", PropImportance.OPTIONAL)]
    public List<Layer>? Layers { get; set; }
    [ModelProp("🗂️", "Gr*", "Grp*", "The optional groups of the design.", PropImportance.OPTIONAL)]
    public List<Group>? Groups { get; set; }
    [ModelProp("👥", "Au*", "Aut*", "The optional authors of the design.", PropImportance.OPTIONAL)]
    public List<AuthorId>? Authors { get; set; }
    [ModelProp("💡", "Co*", "Con*", "The optional concepts of the design.", PropImportance.OPTIONAL)]
    public List<string>? Concepts { get; set; }
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the design.", PropImportance.OPTIONAL)]
    public List<Attribute>? Attributes { get; set; }
    [Name("📅", "CA?", "CrA?", "The optional created at timestamp of the design.")]
    public DateTime? CreatedAt { get; set; }
    [Name("📅", "UA?", "UpA?", "The optional updated at timestamp of the design.")]
    public DateTime? UpdatedAt { get; set; }

    public static implicit operator DesignDiff(DesignId id) => new() { Guid = id.Guid };
    public static implicit operator DesignDiff(Design design) => new() { Guid = design.Guid, Name = design.Name, Parent = design.Parent, IsAbstract = design.IsAbstract, Folder = design.Folder, Description = design.Description, Icon = design.Icon, Image = design.Image, Location = design.Location, Unit = design.Unit, CanScale = design.CanScale, CanMirror = design.CanMirror, ActiveLayer = design.ActiveLayer, Pieces = new PiecesDiff { Removed = new List<PieceId>(), Modified = design.Pieces.Select(p => p.CreateDiff()).ToList(), Added = new List<PieceDiff>() }, Connections = new ConnectionsDiff { Removed = new List<ConnectionId>(), Updated = design.Connections.Select(c => c.CreateDiff()).ToList(), Added = new List<Connection>() }, Props = design.Props, Stats = design.Stats, Layers = design.Layers, Groups = design.Groups, Authors = design.Authors, Concepts = design.Concepts, Attributes = design.Attributes, CreatedAt = design.CreatedAt, UpdatedAt = design.UpdatedAt };

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

[Model("🏙️", "Dn", "Dsn", "The local identifier of the design within the kit.")]
public class DesignId : Model<DesignId>
{
    [Id("🆔", "Gd", "Gui", "The guid of the design.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    public static implicit operator DesignId(Design design) => new() { Guid = design.Guid };
    public static implicit operator DesignId(DesignDiff diff) => new() { Guid = diff.Guid ?? "" };

    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{Guid}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"DsnId({ToHumanIdString()})";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-design-"/>
/// </summary>
[Model("🏙️", "Dn", "Dsn", "A design is a collection of pieces that are connected.")]
public class Design : Model<Design>
{
    [Id("🆔", "Gd", "Gui", "The guid of the design.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    [Name("📛", "Na", "Nam", "The name of the design.", PropImportance.REQUIRED)]
    public string Name { get; set; } = "";
    [Id("📁", "Pa?", "Par?", "The optional parent design guid.")]
    public string? Parent { get; set; }
    [FalseOrTrue("👻", "IA?", "IsA?", "Whether the design is abstract.")]
    public bool? IsAbstract { get; set; }
    [Id("📁", "Fo?", "Fol?", "The optional folder guid.")]
    public string? Folder { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the design.")]
    public string Description { get; set; } = "";
    [Url("🪙", "Ic?", "Ico?", "The optional icon [ emoji | logogram | url ] of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.")]
    public string Icon { get; set; } = "";
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.")]
    public string Image { get; set; } = "";
    [ModelProp("💡", "Co*", "Con*", "The optional concepts of the design.", PropImportance.OPTIONAL)]
    public List<string> Concepts { get; set; } = new();
    [ModelProp("👥", "Au*", "Aut*", "The optional authors of the design.", PropImportance.OPTIONAL)]
    public List<AuthorId> Authors { get; set; } = new();
    [ModelProp("📍", "Lo?", "Loc?", "The optional location of the design.", PropImportance.OPTIONAL)]
    public Location? Location { get; set; }
    [Name("Ⓜ️", "Ut", "Unt", "The length unit for all distance-related information of the design.", PropImportance.REQUIRED)]
    public string Unit { get; set; } = "";
    [FalseOrTrue("⚖️", "CS?", "CanS?", "Whether the design can be scaled.")]
    public bool? CanScale { get; set; }
    [FalseOrTrue("🪞", "CM?", "CanM?", "Whether the design can be mirrored.")]
    public bool? CanMirror { get; set; }
    [ModelProp("🔗", "Ly*", "Lyr*", "The optional layers of the design.", PropImportance.OPTIONAL)]
    public List<Layer> Layers { get; set; } = new();
    [Id("🔖", "AL?", "ActL?", "The optional active layer guid.")]
    public string? ActiveLayer { get; set; }
    [ModelProp("⭕", "Pc*", "Pcs*", "The optional pieces of the design.", PropImportance.OPTIONAL)]
    public List<Piece> Pieces { get; set; } = new();
    [ModelProp("🗂️", "Gr*", "Grp*", "The optional groups of the design.", PropImportance.OPTIONAL)]
    public List<Group> Groups { get; set; } = new();
    [ModelProp("🔗", "Co*", "Cons*", "The optional connections of the design.", PropImportance.OPTIONAL)]
    public List<Connection> Connections { get; set; } = new();
    [ModelProp("🏷️", "Pp*", "Prp*", "The optional properties of the design.", PropImportance.OPTIONAL)]
    public List<Prop> Props { get; set; } = new();
    [ModelProp("🔢", "St*", "Stt*", "The optional stats of the design.", PropImportance.OPTIONAL)]
    public List<Stat> Stats { get; set; } = new();
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the design.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
    [Name("📅", "CA", "CrA", "The created at timestamp of the design.", PropImportance.REQUIRED)]
    public DateTime CreatedAt { get; set; }
    [Name("📅", "UA", "UpA", "The updated at timestamp of the design.", PropImportance.REQUIRED)]
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
                Modified = Pieces.Select(p => p.CreateDiff()).ToList(),
                Added = new List<PieceDiff>()
            },
            Connections = new ConnectionsDiff
            {
                Removed = new List<ConnectionId>(),
                Updated = Connections.Select(c => c.CreateDiff()).ToList(),
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
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(p => p.Guid == updated.Guid);
            if (index >= 0)
                result[index] = result[index].ApplyDiff(updated);
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
            Modified = original.Where(p => modifiedIds.Contains(p.Guid))
                .SelectMany(p =>
                {
                    var modifiedPiece = modified.First(m => m.Guid == p.Guid);
                    var diff = p.CreateDiff();
                    return !Equals(p, modifiedPiece) ? new[] { diff } : new PieceDiff[] { };
                })
                .ToList(),
            Added = modified.Where(p => !originalIds.Contains(p.Guid)).Select(p => new PieceDiff
            {
                Guid = p.Guid,
                Description = p.Description,
                Type = p.Type,
                Plane = p.Plane,
                Center = p.Center,
                Attributes = p.Attributes
            }).ToList()
        };
    }

    private List<Connection> ApplyConnectionsDiff(List<Connection> original, ConnectionsDiff diff)
    {
        var result = original.Where(c => !diff.Removed.Any(r =>
            r.Connected.Piece.Guid == c.Connected.Piece.Guid &&
            r.Connecting.Piece.Guid == c.Connecting.Piece.Guid)).ToList();

        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(c =>
                c.Connected.Piece.Guid == (updated.Connected?.Piece?.Guid ?? c.Connected.Piece.Guid) &&
                c.Connecting.Piece.Guid == (updated.Connecting?.Piece?.Guid ?? c.Connecting.Piece.Guid));
            if (index >= 0)
                result[index] = result[index].ApplyDiff(updated);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private ConnectionsDiff CreateConnectionsDiff(List<Connection> original, List<Connection> modified)
    {
        var originalKeys = original.Select(c => (c.Connected.Piece.Guid, c.Connecting.Piece.Guid)).ToHashSet();
        var modifiedKeys = modified.Select(c => (c.Connected.Piece.Guid, c.Connecting.Piece.Guid)).ToHashSet();

        return new ConnectionsDiff
        {
            Removed = original.Where(c => !modifiedKeys.Contains((c.Connected.Piece.Guid, c.Connecting.Piece.Guid)))
                .Select(c => new ConnectionId { Connected = c.Connected, Connecting = c.Connecting }).ToList(),
            Updated = original.Where(c => modifiedKeys.Contains((c.Connected.Piece.Guid, c.Connecting.Piece.Guid)))
                .SelectMany(c =>
                {
                    var modifiedConnection = modified.First(m => m.Connected.Piece.Guid == c.Connected.Piece.Guid && m.Connecting.Piece.Guid == c.Connecting.Piece.Guid);
                    var diff = c.CreateDiff();
                    return !Equals(c, modifiedConnection) ? new[] { diff } : new ConnectionDiff[] { };
                })
                .ToList(),
            Added = modified.Where(c => !originalKeys.Contains((c.Connected.Piece.Guid, c.Connecting.Piece.Guid))).ToList()
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
            var ports = new Dictionary<string, Dictionary<string, Port>>();
            foreach (var type in types)
            {
                if (!ports.ContainsKey(type.Guid))
                    ports[type.Guid] = new Dictionary<string, Port>();
                foreach (var port in type.Ports)
                    ports[type.Guid][port.Guid] = port;
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
                if (!ports[connectedType.Name].ContainsKey(connection.Connected.Port.Guid))
                    throw new Exception(
                        $"The type {connectedType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the port {connection.Connected.Port.Guid}.");
                var connectingPiece = Pieces.First(p => p.Guid == connection.Connecting.Piece.Guid);
                if (connectingPiece.Type is null)
                    throw new Exception($"Flatten requires all pieces to have a type. Piece ({connectingPiece.Guid}) has no type.");
                var connectingType = types.First(t => t.Guid == connectingPiece.Type.Guid);
                if (!ports[connectingType.Name].ContainsKey(connection.Connecting.Port.Guid))
                    throw new Exception(
                        $"The type {connectingType.ToHumanIdString()} of the connection {connection.ToHumanIdString()} doesn't have the port {connection.Connecting.Port.Guid}.");
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
                var parentPort =
                    ports[parent.Type.Guid][
                        isParentConnected ? connection.Connected.Port.Guid : connection.Connecting.Port.Guid];
                var childPort =
                    ports[child.Type.Guid][
                        isParentConnected ? connection.Connecting.Port.Guid : connection.Connected.Port.Guid];
                if (parentPort.Point is null || parentPort.Direction is null || childPort.Point is null || childPort.Direction is null) return;
                var childPlane = computeChildPlane(parentPlane, parentPort.Point, parentPort.Direction,
                    childPort.Point, childPort.Direction,
                    connection.Gap, connection.Shift, connection.Rise,
                    connection.Rotation, connection.Turn, connection.Tilt);
                child.Plane = childPlane;

                var direction = new Coord
                {
                    U = connection.X,
                    V = connection.Y
                }.Normalize();
                var childCenter = new Coord
                {
                    U = parent.Center!.U + connection.X + direction.U,
                    V = parent.Center!.V + connection.Y + direction.V
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
        // scale to iconWidth and change coordinate system
        foreach (var piece in Pieces)
        {
            if (piece.Center is null) continue;
            piece.Center.U = piece.Center.U * iconWidth;
            piece.Center.V = -(piece.Center.V * iconWidth);
        }

        foreach (var connection in Connections)
        {
            connection.X = connection.X * iconWidth;
            connection.Y = -(connection.Y * iconWidth);
        }

        // recenter
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

[Model("🗃️", "KD", "KDf", "A diff for kits.")]
public class KitDiff : Model<KitDiff>
{
    [Id("🆔", "Gd?", "Gui?", "The optional guid of the kit.")]
    public string? Guid { get; set; }
    [Name("📛", "Na?", "Nam?", "The optional name of the kit.")]
    public string? Name { get; set; }
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the kit.")]
    public string? Description { get; set; }
    [Url("🪙", "Ic?", "Ico?", "The optional icon of the kit.")]
    public string? Icon { get; set; }
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the kit.")]
    public string? Image { get; set; }
    [Url("🔮", "Pv?", "Prv?", "The optional url of the preview image of the kit.")]
    public string? Preview { get; set; }
    [Name("🔀", "Vr?", "Ver?", "The optional version of the kit.")]
    public string? Version { get; set; }
    [Url("☁️", "Rm?", "Rmt?", "The optional URL where to fetch the kit remotely.")]
    public string? Remote { get; set; }
    [Url("🏠", "Hp?", "Hmp?", "The optional URL of the homepage of the kit.")]
    public string? Homepage { get; set; }
    [Url("⚖️", "Li?", "Lic?", "The optional license of the kit.")]
    public string? License { get; set; }
    [ModelProp("🧩", "Ty*", "Typ*", "The optional types diff for the kit.", PropImportance.OPTIONAL)]
    public TypesDiff? Types { get; set; }
    [ModelProp("🏙️", "Dn*", "Dsn*", "The optional designs diff for the kit.", PropImportance.OPTIONAL)]
    public DesignsDiff? Designs { get; set; }
    [ModelProp("📄", "Fl*", "Fil*", "The optional files diff for the kit.", PropImportance.OPTIONAL)]
    public FilesDiff? Files { get; set; }
    [ModelProp("�", "Fo*", "Fol*", "The optional folders diff for the kit.", PropImportance.OPTIONAL)]
    public FoldersDiff? Folders { get; set; }
    [ModelProp("�🔐", "At*", "Atr*", "The optional attributes of the kit.", PropImportance.OPTIONAL)]
    public List<Attribute>? Attributes { get; set; }
    [Name("📅", "CA?", "CrA?", "The optional creation date.")]
    public string? CreatedAt { get; set; }
    [Name("📝", "UA?", "UpA?", "The optional last update date.")]
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
            Attributes = other.Attributes ?? Attributes,
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
        Attributes = kit.Attributes,
        CreatedAt = kit.CreatedAt,
        UpdatedAt = kit.UpdatedAt
    };
}

[Model("🗃️", "KId", "KitId", "The local identifier of the kit.")]
public class KitId : Model<KitId>
{
    [Id("🆔", "Gd", "Gui", "The guid of the kit.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    public string ToIdString() => $"{Guid}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"KitId({ToHumanIdString()})";

    public static implicit operator KitId(Kit kit) => new() { Guid = kit.Guid };
    public static implicit operator KitId(KitDiff diff) => new() { Guid = diff.Guid ?? "" };
}

[Model("📦", "KsD", "KsDf", "A diff for multiple kits.")]
public class KitsDiff : Model<KitsDiff>
{
    [ModelProp("➖", "Rm*", "Rem*", "The optional removed kits.", PropImportance.OPTIONAL)]
    public List<KitId> Removed { get; set; } = new();
    [ModelProp("✏️", "Up*", "Upd*", "The optional updated kits.", PropImportance.OPTIONAL)]
    public List<KitDiff> Updated { get; set; } = new();
    [ModelProp("➕", "Ad*", "Add*", "The optional added kits.", PropImportance.OPTIONAL)]
    public List<Kit> Added { get; set; } = new();

    public static implicit operator KitsDiff(List<Kit> kits) => new() { Updated = kits.Select(k => (KitDiff)k).ToList() };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-kit-"/>
/// </summary>
[Model("🗃️", "Kt", "Kit", "A kit is a collection of types and designs.")]
public class Kit : Model<Kit>
{
    [Id("🆔", "Gd", "Gui", "The guid of the kit.", PropImportance.ID)]
    public string Guid { get; set; } = "";
    [Name("📛", "Na", "Nam", "The name of the kit.", PropImportance.ID)]
    public string Name { get; set; } = "";
    [Name("🔀", "Vr?", "Ver?", "The optional version of the kit. No version means the latest version.", PropImportance.ID, true)]
    public string Version { get; set; } = "";
    [Description("💬", "Dc?", "Dsc?", "The optional human-readable description of the kit.")]
    public string Description { get; set; } = "";
    [Url("🪙", "Ic?", "Ico?", "The optional icon [ emoji | logogram | url ] of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.")]
    public string Icon { get; set; } = "";
    [Url("🖼️", "Im?", "Img?", "The optional url to the image of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.")]
    public string Image { get; set; } = "";
    [ModelProp("🏷️", "Cp*", "Cnp*", "The optional concepts of the kit.", PropImportance.OPTIONAL)]
    public List<string> Concepts { get; set; } = new();
    [Url("☁️", "Rm?", "Rmt?", "The optional Unique Resource Locator (URL) where to fetch the kit remotely.")]
    public string Remote { get; set; } = "";
    [Url("🏠", "Hp?", "Hmp?", "The optional Unique Resource Locator (URL) of the homepage of the kit.")]
    public string Homepage { get; set; } = "";
    [Url("⚖️", "Li?", "Lic?", "The optional license [ spdx id | url ] of the kit.")]
    public string License { get; set; } = "";
    [ModelProp("👥", "Au*", "Aut*", "The optional authors of the kit.", PropImportance.OPTIONAL)]
    public List<Author> Authors { get; set; } = new();
    [ModelProp("⭕", "Pc*", "Pcs*", "The optional pieces of the kit.", PropImportance.OPTIONAL)]
    public List<Piece> Pieces { get; set; } = new();
    [ModelProp("🗂️", "Gr*", "Grp*", "The optional groups of the kit.", PropImportance.OPTIONAL)]
    public List<Group> Groups { get; set; } = new();
    [ModelProp("🔗", "Co*", "Cons*", "The optional connections of the kit.", PropImportance.OPTIONAL)]
    public List<Connection> Connections { get; set; } = new();
    [ModelProp("🏷️", "Pp*", "Prp*", "The optional properties of the kit.", PropImportance.OPTIONAL)]
    public List<Prop> Props { get; set; } = new();
    [ModelProp("🔢", "St*", "Stt*", "The optional stats of the kit.", PropImportance.OPTIONAL)]
    public List<Stat> Stats { get; set; } = new();
    [ModelProp("🔐", "At*", "Atr*", "The optional attributes of the kit.", PropImportance.OPTIONAL)]
    public List<Attribute> Attributes { get; set; } = new();
    [Url("🔮", "Pv?", "Prv?", "The optional url of the preview image of the kit. The url must point to a landscape image [ png | jpg | svg ] which will be cropped by a 2x1 rectangle. The image must be at least 1920x960 pixels and smaller than 15 MB.")]
    public string Preview { get; set; } = "";
    [ModelProp("📃", "Ql*", "Qal*", "The optional qualities of the kit.", PropImportance.OPTIONAL)]
    public List<Quality> Qualities { get; set; } = new();
    [ModelProp("📄", "Fl*", "Fil*", "The optional files of the kit.", PropImportance.OPTIONAL)]
    public List<File> Files { get; set; } = new();
    [ModelProp("📁", "Fo*", "Fol*", "The optional folders of the kit.", PropImportance.OPTIONAL)]
    public List<Folder> Folders { get; set; } = new();
    [ModelProp("🧩", "Ty*", "Typ*", "The optional types of the kit.", PropImportance.OPTIONAL)]
    public List<Type> Types { get; set; } = new();
    [ModelProp("🏙️", "Dn*", "Dsn*", "The optional designs of the kit.", PropImportance.OPTIONAL)]
    public List<Design> Designs { get; set; } = new();
    [Name("📅", "CA", "CrA", "The creation date of the kit.")]
    public string CreatedAt { get; set; } = "";
    [Name("📝", "UA", "UpA", "The last update date of the kit.")]
    public string UpdatedAt { get; set; } = "";

    public static implicit operator Kit(KitDiff diff) => new() { Name = diff.Name ?? "", Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Preview = diff.Preview ?? "", Version = diff.Version ?? "", Remote = diff.Remote ?? "", Homepage = diff.Homepage ?? "", License = diff.License ?? "", Files = diff.Files?.Added ?? new(), Attributes = diff.Attributes ?? new() };
    public static implicit operator string(Kit kit) => kit.Name;
    public static implicit operator Kit(string name) => new() { Name = name };

    public Kit ApplyDiff(KitDiff diff)
    {
        var types = Types;
        var designs = Designs;
        var files = Files;

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
            Attributes = diff.Attributes?.Any() == true ? diff.Attributes : Attributes
        };
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
                Modified = Types.Select(t => t.CreateDiff()).ToList(),
                Added = new List<TypeDiff>()
            },
            Designs = new DesignsDiff
            {
                Removed = new List<DesignId>(),
                Updated = Designs.Select(d => d.CreateDiff()).ToList(),
                Added = new List<Design>()
            },
            Files = new FilesDiff
            {
                Removed = new List<FileId>(),
                Updated = Files.Select(f => (FileDiff)f).ToList(),
                Added = new List<File>()
            },
            Attributes = Attributes
        };
    }

    private List<Type> ApplyTypesDiff(List<Type> original, TypesDiff diff)
    {
        var result = original.Where(t => !diff.Removed.Any(r => r.Guid == t.Guid)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(t => t.Guid == (updated.Guid ?? t.Guid));
            if (index >= 0)
                result[index] = result[index].ApplyDiff(updated);
        }
        result.AddRange(diff.Added.Select(a => new Type
        {
            Name = a.Name ?? "",
            Description = a.Description ?? "",
            Icon = a.Icon ?? "",
            Image = a.Image ?? "",
            Stock = a.Stock ?? 2147483647,
            Virtual = a.Virtual ?? false,
            Unit = a.Unit,
            Location = a.Location,
            Models = a.Models,
            Ports = a.Ports,
            Authors = a.Authors.Select(auth => new AuthorId { Email = auth.Email }).ToList(),
            Attributes = a.Attributes ?? new List<Attribute>()
        }));
        return result;
    }

    private TypesDiff CreateTypesDiff(List<Type> original, List<Type> modified)
    {
        var originalKeys = original.Select(t => t.Name).ToHashSet();
        var modifiedKeys = modified.Select(t => t.Name).ToHashSet();

        return new TypesDiff
        {
            Removed = original.Where(t => !modifiedKeys.Contains(t.Name))
                .Select(t => new TypeId { Guid = t.Guid }).ToList(),
            Modified = original.Where(t => modifiedKeys.Contains(t.Name))
                .SelectMany(t =>
                {
                    var modifiedType = modified.First(m => m.Name == t.Name);
                    var diff = t.CreateDiff();
                    return !Equals(t, modifiedType) ? new[] { diff } : new TypeDiff[] { };
                })
                .ToList(),
            Added = modified.Where(t => !originalKeys.Contains(t.Name)).Select(t => new TypeDiff
            {
                Name = t.Name,
                Description = t.Description,
                Icon = t.Icon,
                Image = t.Image,
                Stock = t.Stock,
                Virtual = t.Virtual,
                Unit = t.Unit,
                Location = t.Location,
                Models = t.Models,
                Ports = t.Ports,
                Authors = t.Authors,
                Attributes = t.Attributes
            }).ToList()
        };
    }

    private List<Design> ApplyDesignsDiff(List<Design> original, DesignsDiff diff)
    {
        var result = original.Where(d => !diff.Removed.Any(r => r.Guid == d.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(d => d.Guid == (updated.Guid ?? d.Guid));
            if (index >= 0)
                result[index] = result[index].ApplyDiff(updated);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private DesignsDiff CreateDesignsDiff(List<Design> original, List<Design> modified)
    {
        var originalKeys = original.Select(d => d.Guid).ToHashSet();
        var modifiedKeys = modified.Select(d => d.Guid).ToHashSet();

        return new DesignsDiff
        {
            Removed = original.Where(d => !modifiedKeys.Contains(d.Guid))
                .Select(d => new DesignId { Guid = d.Guid }).ToList(),
            Updated = original.Where(d => modifiedKeys.Contains(d.Guid))
                .SelectMany(d =>
                {
                    var modifiedDesign = modified.First(m => m.Guid == d.Guid);
                    var diff = d.CreateDiff();
                    return !Equals(d, modifiedDesign) ? new[] { diff } : new DesignDiff[] { };
                })
                .ToList(),
            Added = modified.Where(d => !originalKeys.Contains(d.Guid)).ToList()
        };
    }

    private List<File> ApplyFilesDiff(List<File> original, FilesDiff diff)
    {
        var result = original.Where(f => !diff.Removed.Any(r => r.Guid == f.Guid)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(f => f.Guid == (updated.Guid ?? f.Guid));
            if (index >= 0)
            {
                var file = result[index];
                result[index] = new File
                {
                    Guid = updated.Guid ?? file.Guid,
                    Name = updated.Name ?? file.Name,
                    Remote = updated.Remote ?? file.Remote,
                    Folder = updated.Folder ?? file.Folder,
                    Size = updated.Size ?? file.Size,
                    Hash = updated.Hash ?? file.Hash,
                    CreatedAt = updated.CreatedAt ?? file.CreatedAt,
                    CreatedBy = updated.CreatedBy ?? file.CreatedBy,
                    UpdatedAt = updated.UpdatedAt ?? file.UpdatedAt,
                    UpdatedBy = updated.UpdatedBy ?? file.UpdatedBy
                };
            }
        }
        result.AddRange(diff.Added);
        return result;
    }

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
}

#endregion Kit

#region Api

public class PredictDesignBody { public string? Description { get; set; } public Type[]? Types { get; set; } public Design? Design { get; set; } }

public interface IApi
{
    [Get("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<Kit>> GetKit(string encodedKitUri);

    [Put("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> CreateKit(string encodedKitUri, [Body] Kit input);

    [Delete("/api/kits/{encodedKitUri}")]
    Task<ApiResponse<bool>> DeleteKit(string encodedKitUri);


    [Put("/api/kits/{encodedKitUri}/types/{encodedTypeName}")]
    Task<ApiResponse<bool>> PutType(string encodedKitUri, string encodedTypeName, [Body] Type input);

    [Delete("/api/kits/{encodedKitUri}/types/{encodedTypeName}")]
    Task<ApiResponse<bool>> RemoveType(string encodedKitUri, string encodedTypeName);

    [Put("/api/kits/{encodedKitUri}/designs/{encodedDesignName}")]
    Task<ApiResponse<bool>> PutDesign(string encodedKitUri, string encodedDesignName,
        [Body] Design input);

    [Delete("/api/kits/{encodedKitUri}/designs/{encodedDesignName}")]
    Task<ApiResponse<bool>> RemoveDesign(string encodedKitUri, string encodedDesignName);

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
            model[mt.Name] = mt.GetCustomAttribute<ModelAttribute>()!;
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

                if (mtp.DeclaringType?.FullName != mt.FullName)
                {
                    propertyParents.Add(mtp);
                    if (mtpProp is not null) propParents.Add(mtpProp);
                    isPropertyListParents.Add(imtpl);
                    propertyItemTypeParents.Add(mtpPropertyItemType);
                    isPropertyModelParents.Add(mtpIsPropertyModel);
                }
                else
                {
                    property[mt.Name].Add(mtp);
                    if (mtpProp is not null) prop[mt.Name].Add(mtpProp);
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

#endregion Meta

#endregion Modeling



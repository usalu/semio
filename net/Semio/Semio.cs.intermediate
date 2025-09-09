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
using System.ComponentModel.DataAnnotations;
using System.Drawing;
using System.Globalization;
using System.Net;
using System.Net.Http;
using System.Text;
using System.Xml;
using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using QuikGraph;
using QuikGraph.Algorithms;
using QuikGraph.Algorithms.Search;
using Refit;
using Svg;
using Svg.Transforms;
using UnitsNet;
using FluentValidation;
using FluentValidation.Results;
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
                mime = response.Content.Headers.ContentType.MediaType;
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
        if (adjectives is null || animals is null) return "Unknown0";

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
                internal ConvertModel()
                {
                    FromUnit = "";
                    ToUnit = "";
                }
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

#region Bases

public abstract class Base<TValidator> : where TValidator : new()
{
    public virtual FluentValidation.Results.ValidationResult Validate() => new TValidator().Validate(this);
}

public abstract class Validator<TBase> : AbstractValidator<TBase> where TBase : Base<TValidator>
{
}

public abstract class Id : Base<IdValidator>
{

}

public abstract class IdValidator : Validator<Id>
{
}

public abstract class Diff<TDiff> : Base<DiffValidator>
{
    public abstract TDiff Merge(TDiff other);
}

public abstract class DiffValidator : Validator<Diff>
{
}

public abstract class Model<TModel, TDiff> : Base<ModelValidator<TModel, TDiff>>
{
    public abstract TModel ApplyDiff(TDiff diff);
    public abstract TModel InverseDiff(TDiff diff);
}

public abstract class ModelValidator : Validator<Model<TModel, TDiff>>
{
}


#region Bases


#region Attributes
// https://github.com/usalu/semio#-attribute-

public class AttributeId : Id
{
    [JsonProperty("key_")]
    public string Key { get; set; } = "";

    public static implicit operator AttributeId(Attribute attribute) => new() { Key = attribute.Key };
    public static implicit operator AttributeId(AttributeDiff diff) => new() { Key = diff.Key ?? "" };

}

public class AttributeIdValidator : AbstractValidator<AttributeId>
{
    public AttributeIdValidator()
    {
        RuleFor(x => x.Key).NotEmpty().WithMessage("The key is required.");
    }
}

public class AttributeDiff : Diff<AttributeDiff>
{
    [JsonProperty("key_")]
    public string? Key { get; set; } = "";
    public string? Value { get; set; } = "";
    public string? Definition { get; set; } = "";

    public static implicit operator AttributeDiff(AttributeId id) => new() { Key = id.Key };
    public static implicit operator AttributeDiff(Attribute attribute) => new() { Key = attribute.Key, Value = attribute.Value, Definition = attribute.Definition };

    public override AttributeDiff Merge(AttributeDiff other)
    {
        return new AttributeDiff
        {
            Key = string.IsNullOrEmpty(other.Key) ? Key : other.Key,
            Value = string.IsNullOrEmpty(other.Value) ? Value : other.Value,
            Definition = string.IsNullOrEmpty(other.Definition) ? Definition : other.Definition
        };
    }
}

public class AttributeDiffValidator : AbstractValidator<AttributeDiff>
{
    public AttributeDiffValidator()
    {
        RuleFor(x => x.Key).NotEmpty().WithMessage("The key is required.");
    }
}
public class Attribute : Model
{
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
    public string Definition { get; set; } = "";

    public static implicit operator Attribute(AttributeId id) => new() { Key = id.Key };
    public static implicit operator Attribute(AttributeDiff diff) => new() { Key = diff.Key ?? "", Value = diff.Value ?? "", Definition = diff.Definition ?? "" };

    public Attribute ApplyDiff(AttributeDiff diff)
    {
        if (diff.Key is not null) Key = diff.Key;
        if (diff.Value is not null) Value = diff.Value;
        if (diff.Definition is not null) Definition = diff.Definition;
        return this;
    }
    public AttributeDiff InverseDiff(AttributeDiff appliedDiff)
    {
        return new AttributeDiff
        {
            Key = appliedDiff.Key is not null ? Key : null,
            Value = appliedDiff.Value is not null ? Value : null,
            Definition = appliedDiff.Definition is not null ? Definition : null
        }
        ;
    }

}

#endregion Attributes

/// <summary>
/// <see href="https://github.com/usalu/semio#-benchmark-"/>
/// </summary>
public class Benchmark : Model<Benchmark>
{
    public string Name { get; set; } = "";
    public string Icon { get; set; } = "";
    public float Min { get; set; } = 0;
    public bool MinExcluded { get; set; } = false;
    public float Max { get; set; } = 0;
    public bool MaxExcluded { get; set; } = false;
    public List<Attribute> Attributes { get; set; } = new();
}

[Flags]
public enum QualityKind
{
    General = 0,
    Design = 1,
    Type = 2,
    Piece = 4,
    Connection = 8,
    Port = 16,
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-quality-"/>
/// </summary>
public class QualityId : Model<QualityId>
{
    public string Key { get; set; } = "";

    public static implicit operator QualityId(Quality quality) => new() { Key = quality.Key };
    public static implicit operator QualityId(QualityDiff diff) => new() { Key = diff.Key };
}

public class QualityDiff : Model<QualityDiff>
{
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

    public static implicit operator QualityDiff(QualityId quality) => new() { Key = quality.Key };

    public static implicit operator QualityDiff(Quality quality) => new() { Key = quality.Key, Name = quality.Name, Description = quality.Description, Uri = quality.Uri, Scalable = quality.Scalable, Kind = quality.Kind, SI = quality.SI, Imperial = quality.Imperial, Min = quality.Min, MinExcluded = quality.MinExcluded, Max = quality.Max, MaxExcluded = quality.MaxExcluded, Default = quality.Default, Formula = quality.Formula, Benchmarks = quality.Benchmarks, Attributes = quality.Attributes };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-quality-"/>
/// </summary>
public class Quality : Model<Quality>
{
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

/// <summary>
/// <see href="https://github.com/usalu/semio#-property-"/>
/// </summary>
public class Prop : Model<Prop>
{
    public string Key { get; set; } = "";
    public string Value { get; set; } = "";
    public string Unit { get; set; } = "";
    public List<Attribute> Attributes { get; set; } = new();
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-stat-"/>
/// </summary>
public class Stat : Model<Stat>
{
    public string Key { get; set; } = "";
    public string Unit { get; set; } = "";
    public float Min { get; set; } = 0;
    public bool MinExcluded { get; set; } = false;
    public float Max { get; set; } = 0;
    public bool MaxExcluded { get; set; } = false;
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-layer-"/>
/// </summary>

public class Layer : Model<Layer>
{

    public string Name { get; set; } = "";

    public string Description { get; set; } = "";

    public string Color { get; set; } = "";

    public string ToIdString() => $"{Name}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Lyr({ToHumanIdString()})";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-group-"/>
/// </summary>

public class Group : Model<Group>
{

    public string Name { get; set; } = "";

    public string Description { get; set; } = "";

    public List<PieceId> Pieces { get; set; } = new();

    public string Color { get; set; } = "";

    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Name}";
    public string ToHumanIdString() => $"{Name}";
    public override string ToString() => $"Grp({ToHumanIdString()})";
}


public class RepresentationId : Model<RepresentationId>
{

    public List<string> Tags { get; set; } = new();
    public static implicit operator RepresentationId(Representation representation) => new() { Tags = representation.Tags };
    public static implicit operator RepresentationId(RepresentationDiff diff) => new() { Tags = diff.Tags };
    public string ToIdString() => $"{string.Join(",", Tags.Select(t => Utility.Encode(t)))}";
    public string ToHumanIdString() => string.Join(", ", Tags);
    public override string ToString() => $"Rep({ToHumanIdString()})";
}


public class RepresentationDiff : Model<RepresentationDiff>
{

    public string Url { get; set; } = "";

    public string Description { get; set; } = "";

    public List<string> Tags { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator RepresentationDiff(RepresentationId id) => new() { Tags = id.Tags };
    public static implicit operator RepresentationDiff(Representation representation) => new() { Url = representation.Url, Description = representation.Description, Tags = representation.Tags, Attributes = representation.Attributes };

    public RepresentationDiff MergeDiff(RepresentationDiff other)
    {
        return new RepresentationDiff
        {
            Url = string.IsNullOrEmpty(other.Url) ? Url : other.Url,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Tags = other.Tags.Any() ? other.Tags : Tags,
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes
        };
    }
}


public class RepresentationsDiff : Model<RepresentationsDiff>
{

    public List<RepresentationId> Removed { get; set; } = new();

    public List<RepresentationDiff> Added { get; set; } = new();

    public List<RepresentationDiff> Modified { get; set; } = new();

    public static implicit operator RepresentationsDiff(List<Representation> representations) => new() { Modified = representations.Select(r => (RepresentationDiff)r).ToList() };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-representation-"/>
/// </summary>
public class Representation : Model<Representation>
{

    public string Url { get; set; } = "";


    public string Description { get; set; } = "";


    public List<string> Tags { get; set; } = new();


    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator Representation(RepresentationId id) => new() { Tags = id.Tags };
    public static implicit operator Representation(RepresentationDiff diff) => new() { Url = diff.Url, Description = diff.Description, Tags = diff.Tags, Attributes = diff.Attributes };

    public Representation ApplyDiff(RepresentationDiff diff)
    {
        return new Representation
        {
            Url = string.IsNullOrEmpty(diff.Url) ? Url : diff.Url,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Tags = diff.Tags.Any() ? diff.Tags : Tags,
            Attributes = diff.Attributes.Any() ? diff.Attributes : Attributes
        };
    }

    public RepresentationDiff CreateDiff()
    {
        return new RepresentationDiff
        {
            Url = Url,
            Description = Description,
            Tags = Tags,
            Attributes = Attributes
        };
    }

    public RepresentationDiff InverseDiff(RepresentationDiff appliedDiff)
    {
        return new RepresentationDiff
        {
            Url = !string.IsNullOrEmpty(appliedDiff.Url) ? Url : "",
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


public class FileId : Model<FileId>
{

    public string Url { get; set; } = "";
    public string ToIdString() => $"{Url}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"FilId({ToHumanIdString()})";

    public static implicit operator FileId(File file) => new() { Url = file.Url };
    public static implicit operator FileId(FileDiff diff) => new() { Url = diff.Url ?? "" };
}


public class File : Model<File>
{

    public string Url { get; set; } = "";

    public string Data { get; set; } = "";

    public int? Size { get; set; }

    public string Hash { get; set; } = "";
    public string ToIdString() => $"{Url}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"Fil({ToHumanIdString()})";

    public static implicit operator File(FileId id) => new() { Url = id.Url };
    public static implicit operator File(FileDiff diff) => new() { Url = diff.Url ?? "", Data = diff.Data ?? "", Size = diff.Size, Hash = diff.Hash ?? "" };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-diagram-point-"/>
/// </summary>

public class DiagramPoint : Model<DiagramPoint>
{

    public float X { get; set; }


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

public class Point : Model<Point>
{

    public float X { get; set; } = 0;

    public float Y { get; set; } = 0;

    public float Z { get; set; } = 0;
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-vector-"/>
/// </summary>

public class Vector : Model<Vector>
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

/// <summary>
/// <see href="https://github.com/usalu/semio#-plane-"/>
/// </summary>

public class Plane : Model<Plane>
{

    public Point Origin { get; set; } = new();


    public Vector XAxis { get; set; } = new();


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


public class PortId : Model<PortId>
{

    [JsonProperty("id_")]
    public string Id { get; set; } = "";
    public static implicit operator PortId(Port port) => new() { Id = port.Id };
    public static implicit operator PortId(PortDiff diff) => new() { Id = diff.Id };
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"Por({ToHumanIdString()})";
}


public class PortDiff : Model<PortDiff>
{

    [JsonProperty("id_")]
    public string Id { get; set; } = "";

    public string Description { get; set; } = "";

    public string Family { get; set; } = "";

    public bool? Mandatory { get; set; }

    public float? T { get; set; }

    public List<string> CompatibleFamilies { get; set; } = new();

    public Point? Point { get; set; }

    public Vector? Direction { get; set; }

    public List<Prop> Props { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();

    public static implicit operator PortDiff(PortId id) => new() { Id = id.Id };
    public static implicit operator PortDiff(Port port) => new() { Id = port.Id, Description = port.Description, Family = port.Family, Mandatory = port.Mandatory, T = port.T, CompatibleFamilies = port.CompatibleFamilies, Point = port.Point, Direction = port.Direction, Props = port.Props, Attributes = port.Attributes };

    public PortDiff MergeDiff(PortDiff other)
    {
        return new PortDiff
        {
            Id = string.IsNullOrEmpty(other.Id) ? Id : other.Id,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Family = string.IsNullOrEmpty(other.Family) ? Family : other.Family,
            Mandatory = other.Mandatory ?? Mandatory,
            T = other.T ?? T,
            CompatibleFamilies = other.CompatibleFamilies.Any() ? other.CompatibleFamilies : CompatibleFamilies,
            Point = other.Point ?? Point,
            Direction = other.Direction ?? Direction,
            Props = other.Props.Any() ? other.Props : Props,
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes
        };
    }
}


public class PortsDiff : Model<PortsDiff>
{

    public List<PortId> Removed { get; set; } = new();

    public List<PortDiff> Added { get; set; } = new();

    public List<PortDiff> Modified { get; set; } = new();

    public static implicit operator PortsDiff(List<Port> ports) => new() { Modified = ports.Select(p => (PortDiff)p).ToList() };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-port-"/>
/// </summary>

public class Port : Model<Port>
{

    [JsonProperty("id_")]
    public string Id { get; set; } = "";

    public string Description { get; set; } = "";

    public bool Mandatory { get; set; } = false;

    public string Family { get; set; } = "";

    public List<string> CompatibleFamilies { get; set; } = new();

    public Point? Point { get; set; } = null;

    public Vector? Direction { get; set; } = null;

    public float T { get; set; } = 0;

    public List<Prop> Props { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Por({ToHumanIdString()})";

    public static implicit operator Port(PortId id) => new() { Id = id.Id };
    public static implicit operator Port(PortDiff diff) => new() { Id = diff.Id ?? "", Description = diff.Description ?? "", Family = diff.Family ?? "", Mandatory = diff.Mandatory ?? false, T = diff.T ?? 0, CompatibleFamilies = diff.CompatibleFamilies ?? new(), Point = diff.Point, Direction = diff.Direction, Attributes = diff.Attributes ?? new() };
    public static implicit operator string(Port port) => port.Id;
    public static implicit operator Port(string id) => new() { Id = id };

    public Port ApplyDiff(PortDiff diff)
    {
        return new Port
        {
            Id = string.IsNullOrEmpty(diff.Id) ? Id : diff.Id,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Family = string.IsNullOrEmpty(diff.Family) ? Family : diff.Family,
            Mandatory = diff.Mandatory ?? Mandatory,
            T = diff.T ?? T,
            CompatibleFamilies = diff.CompatibleFamilies.Any() ? diff.CompatibleFamilies : CompatibleFamilies,
            Point = diff.Point ?? Point,
            Direction = diff.Direction ?? Direction,
            Props = diff.Props.Any() ? diff.Props : Props,
            Attributes = diff.Attributes.Any() ? diff.Attributes : Attributes
        };
    }

    public PortDiff CreateDiff()
    {
        return new PortDiff
        {
            Id = Id,
            Description = Description,
            Family = Family,
            Mandatory = Mandatory,
            T = T,
            CompatibleFamilies = CompatibleFamilies,
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
            Id = !string.IsNullOrEmpty(appliedDiff.Id) ? Id : "",
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? Description : "",
            Family = !string.IsNullOrEmpty(appliedDiff.Family) ? Family : "",
            Mandatory = appliedDiff.Mandatory.HasValue ? Mandatory : null,
            T = appliedDiff.T.HasValue ? T : null,
            CompatibleFamilies = appliedDiff.CompatibleFamilies.Any() ? CompatibleFamilies : new List<string>(),
            Point = appliedDiff.Point is not null ? Point : null!,
            Direction = appliedDiff.Direction is not null ? Direction : null!,
            Props = appliedDiff.Props.Any() ? Props : new List<Prop>(),
            Attributes = appliedDiff.Attributes.Any() ? Attributes : new List<Attribute>()
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
        var normalizedPortFamily = Utility.Normalize(Family);
        var normalizedOtherPortFamily = Utility.Normalize(otherPort.Family);
        if (normalizedPortFamily == "" || normalizedOtherPortFamily == "") return true;
        return (CompatibleFamilies ?? new List<string>()).Contains(normalizedOtherPortFamily) ||
               (otherPort.CompatibleFamilies ?? new List<string>()).Contains(normalizedPortFamily);
    }

    public bool IsSameAs(Port other)
    {
        return Utility.Normalize(Id) == Utility.Normalize(other.Id);
    }

    public string FindAttributeValue(string name, string defaultValue = "")
    {
        var attribute = Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in port {Id}");
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
            Id = Id,
            Description = Description,
            Mandatory = Mandatory,
            Family = Family,
            CompatibleFamilies = CompatibleFamilies,
            Point = Point,
            Direction = Direction,
            T = T,
            Props = Props,
            Attributes = attributes
        };
    }
}


public class Author : Model<Author>
{

    public string Name { get; set; } = "";

    public string Email { get; set; } = "";

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

public class AuthorId : Model<AuthorId>
{

    public string Email { get; set; } = "";
    public static implicit operator AuthorId(Author author) => new() { Email = author.Email };
    public string ToIdString() => $"{Email}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Aut({ToHumanIdString()})";
}


public class Location : Model<Location>
{

    public float Longitude { get; set; }

    public float Latitude { get; set; }

    public List<Attribute> Attributes { get; set; } = new();
}


public class TypeId : Model<TypeId>
{

    public string Name { get; set; } = "";

    public string Variant { get; set; } = "";
    public string ToIdString() => $"{Name}#{Variant}";
    public string ToHumanIdString() => $"{Name}" + (Variant.Length == 0 ? "" : $", {Variant}");
    public override string ToString() => $"Typ({ToHumanIdString()})";
    public static implicit operator TypeId(Type type) => new() { Name = type.Name, Variant = type.Variant };
    public static implicit operator TypeId(TypeDiff diff) => new() { Name = diff.Name ?? "", Variant = diff.Variant ?? "" };
}


public class TypeDiff : Model<TypeDiff>
{

    public string Name { get; set; } = "";

    public string Description { get; set; } = "";

    public string Icon { get; set; } = "";

    public string Image { get; set; } = "";

    public string Variant { get; set; } = "";

    public int? Stock { get; set; }

    public bool? Virtual { get; set; }

    public bool? Scalable { get; set; }

    public bool? Mirrorable { get; set; }

    public string Uri { get; set; } = "";

    public string Unit { get; set; } = "";

    public Location? Location { get; set; }

    public List<Representation> Representations { get; set; } = new();

    public List<Port> Ports { get; set; } = new();

    public List<Author> Authors { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();

    public List<string> Concepts { get; set; } = new();

    public TypeDiff MergeDiff(TypeDiff other)
    {
        return new TypeDiff
        {
            Name = string.IsNullOrEmpty(other.Name) ? Name : other.Name,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Icon = string.IsNullOrEmpty(other.Icon) ? Icon : other.Icon,
            Image = string.IsNullOrEmpty(other.Image) ? Image : other.Image,
            Variant = string.IsNullOrEmpty(other.Variant) ? Variant : other.Variant,
            Stock = other.Stock ?? Stock,
            Virtual = other.Virtual ?? Virtual,
            Scalable = other.Scalable ?? Scalable,
            Mirrorable = other.Mirrorable ?? Mirrorable,
            Uri = string.IsNullOrEmpty(other.Uri) ? Uri : other.Uri,
            Unit = string.IsNullOrEmpty(other.Unit) ? Unit : other.Unit,
            Location = other.Location ?? Location,
            Representations = other.Representations.Any() ? other.Representations : Representations,
            Ports = other.Ports.Any() ? other.Ports : Ports,
            Authors = other.Authors.Any() ? other.Authors : Authors,
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes,
            Concepts = other.Concepts.Any() ? other.Concepts : Concepts
        };
    }

    public static implicit operator TypeDiff(TypeId id) => new() { Name = id.Name, Variant = id.Variant };
    public static implicit operator TypeDiff(Type type) => new() { Name = type.Name, Description = type.Description, Icon = type.Icon, Image = type.Image, Variant = type.Variant, Stock = type.Stock, Virtual = type.Virtual, Scalable = type.Scalable, Mirrorable = type.Mirrorable, Uri = type.Uri, Unit = type.Unit, Location = type.Location, Representations = type.Representations, Ports = type.Ports, Authors = type.Authors.Select(a => (Author)a).ToList(), Attributes = type.Attributes, Concepts = type.Concepts };
}


public class TypesDiff : Model<TypesDiff>
{

    public List<TypeId> Removed { get; set; } = new();

    public List<TypeDiff> Added { get; set; } = new();

    public List<TypeDiff> Modified { get; set; } = new();

    public static implicit operator TypesDiff(List<Type> types) => new() { Modified = types.Select(t => (TypeDiff)t).ToList() };
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-type-"/>
/// </summary>

public class Type : Model<Type>
{

    public string Name { get; set; } = "";

    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Image { get; set; } = "";

    public string Variant { get; set; } = "";

    public int Stock { get; set; } = 2147483647;

    public bool Virtual { get; set; } = false;

    public bool Scalable { get; set; } = false;

    public bool Mirrorable { get; set; } = false;

    public string Uri { get; set; } = "";

    public Location? Location { get; set; }

    public string Unit { get; set; } = "";

    public List<Representation> Representations { get; set; } = new();

    public List<Port> Ports { get; set; } = new();

    public List<Prop> Props { get; set; } = new();

    public List<AuthorId> Authors { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();

    public List<string> Concepts { get; set; } = new();

    public string ToIdString() => $"{Name}#{Variant}";

    public string ToHumanIdString() => $"{Name}" + (Variant.Length == 0 ? "" : $", {Variant}");

    public override string ToString() => $"Typ({ToHumanIdString()})";

    public static implicit operator Type(TypeId id) => new() { Name = id.Name, Variant = id.Variant };
    public static implicit operator Type(TypeDiff diff) => new() { Name = diff.Name ?? "", Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Variant = diff.Variant ?? "", Stock = diff.Stock ?? 2147483647, Virtual = diff.Virtual ?? false, Scalable = diff.Scalable ?? false, Mirrorable = diff.Mirrorable ?? false, Uri = diff.Uri ?? "", Unit = diff.Unit ?? "", Location = diff.Location, Representations = diff.Representations ?? new(), Ports = diff.Ports ?? new(), Authors = diff.Authors?.Select(a => (AuthorId)a).ToList() ?? new(), Attributes = diff.Attributes ?? new(), Concepts = diff.Concepts ?? new() };
    public static implicit operator string(Type type) => type.Name;
    public static implicit operator Type(string name) => new() { Name = name };

    public Type ApplyDiff(TypeDiff diff)
    {
        return new Type
        {
            Name = string.IsNullOrEmpty(diff.Name) ? Name : diff.Name,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Icon = string.IsNullOrEmpty(diff.Icon) ? Icon : diff.Icon,
            Image = string.IsNullOrEmpty(diff.Image) ? Image : diff.Image,
            Variant = string.IsNullOrEmpty(diff.Variant) ? Variant : diff.Variant,
            Stock = diff.Stock ?? Stock,
            Virtual = diff.Virtual ?? Virtual,
            Scalable = diff.Scalable ?? Scalable,
            Mirrorable = diff.Mirrorable ?? Mirrorable,
            Uri = string.IsNullOrEmpty(diff.Uri) ? Uri : diff.Uri,
            Unit = string.IsNullOrEmpty(diff.Unit) ? Unit : diff.Unit,
            Location = diff.Location ?? Location,
            Representations = diff.Representations.Any() ? diff.Representations : Representations,
            Ports = diff.Ports.Any() ? diff.Ports : Ports,
            Authors = diff.Authors.Any() ? diff.Authors.Select(a => new AuthorId { Email = a.Email }).ToList() : Authors,
            Attributes = diff.Attributes.Any() ? diff.Attributes : Attributes,
            Concepts = diff.Concepts.Any() ? diff.Concepts : Concepts,
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
            Variant = Variant,
            Stock = Stock,
            Virtual = Virtual,
            Scalable = Scalable,
            Mirrorable = Mirrorable,
            Uri = Uri,
            Unit = Unit,
            Location = Location,
            Representations = Representations,
            Ports = Ports,
            Authors = Authors.Select(a => new Author { Email = a.Email }).ToList(),
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
            Variant = !string.IsNullOrEmpty(appliedDiff.Variant) ? Variant : "",
            Stock = appliedDiff.Stock.HasValue ? Stock : null,
            Virtual = appliedDiff.Virtual.HasValue ? Virtual : null,
            Scalable = appliedDiff.Scalable.HasValue ? Scalable : null,
            Mirrorable = appliedDiff.Mirrorable.HasValue ? Mirrorable : null,
            Uri = !string.IsNullOrEmpty(appliedDiff.Uri) ? Uri : "",
            Unit = !string.IsNullOrEmpty(appliedDiff.Unit) ? Unit : "",
            Location = appliedDiff.Location is not null ? Location : null!,
            Representations = appliedDiff.Representations.Any() ? Representations : new List<Representation>(),
            Ports = appliedDiff.Ports.Any() ? Ports : new List<Port>(),
            Authors = appliedDiff.Authors.Any() ? Authors.Select(a => new Author { Email = a.Email }).ToList() : new List<Author>(),
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

        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
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

    public bool IsSameAs(Type other)
    {
        return Name == other.Name && Utility.Normalize(Variant) == Utility.Normalize(other.Variant);
    }

    public Port FindPort(string portId)
    {
        var port = Ports?.FirstOrDefault(p => Utility.Normalize(p.Id) == Utility.Normalize(portId));
        if (port is null) throw new InvalidOperationException($"Port {portId} not found in type {Name}");
        return port;
    }

    public Representation FindRepresentation(List<string> tags)
    {
        if (Representations == null || Representations.Count == 0)
            throw new ArgumentException($"No representations available in type {Name}");

        var indices = Representations.Select(r => Utility.Jaccard(r.Tags, tags)).ToList();
        var maxIndex = indices.Max();
        var maxIndexIndex = indices.IndexOf(maxIndex);
        return Representations[maxIndexIndex];
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
            Variant = Variant,
            Stock = Stock,
            Virtual = Virtual,
            Location = Location,
            Unit = Unit,
            Representations = Representations,
            Ports = Ports,
            Props = Props,
            Authors = Authors,
            Attributes = attributes
        };
    }
}

#region Diff Classes


public class ConnectionDiff : Model<ConnectionDiff>
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

    public float? X { get; set; }

    public float? Y { get; set; }

    public ConnectionDiff MergeDiff(ConnectionDiff other)
    {
        return new ConnectionDiff
        {
            Connected = other.Connected ?? Connected,
            Connecting = other.Connecting ?? Connecting,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Gap = other.Gap ?? Gap,
            Shift = other.Shift ?? Shift,
            Rise = other.Rise ?? Rise,
            Rotation = other.Rotation ?? Rotation,
            Turn = other.Turn ?? Turn,
            Tilt = other.Tilt ?? Tilt,
            X = other.X ?? X,
            Y = other.Y ?? Y
        };
    }

    public static implicit operator ConnectionDiff(ConnectionId id) => new() { Connected = new SideDiff { Piece = new PieceDiff { Id = id.Connected.Piece.Id }, Port = new PortDiff { Id = id.Connected.Port.Id } }, Connecting = new SideDiff { Piece = new PieceDiff { Id = id.Connecting.Piece.Id }, Port = new PortDiff { Id = id.Connecting.Port.Id } } };
    public static implicit operator ConnectionDiff(Connection connection) => new() { Connected = new SideDiff { Piece = new PieceDiff { Id = connection.Connected.Piece.Id }, Port = new PortDiff { Id = connection.Connected.Port.Id } }, Connecting = new SideDiff { Piece = new PieceDiff { Id = connection.Connecting.Piece.Id }, Port = new PortDiff { Id = connection.Connecting.Port.Id } }, Description = connection.Description, Gap = connection.Gap, Shift = connection.Shift, Rise = connection.Rise, Rotation = connection.Rotation, Turn = connection.Turn, Tilt = connection.Tilt, X = connection.X, Y = connection.Y };
}


public class ConnectionsDiff : Model<ConnectionsDiff>
{

    public List<ConnectionId> Removed { get; set; } = new();

    public List<ConnectionDiff> Updated { get; set; } = new();

    public List<Connection> Added { get; set; } = new();

    public static implicit operator ConnectionsDiff(List<Connection> connections) => new() { Updated = connections.Select(c => (ConnectionDiff)c).ToList() };
}


public class DesignDiff : Model<DesignDiff>
{

    public string Name { get; set; } = "";

    public string Description { get; set; } = "";

    public string Icon { get; set; } = "";

    public string Image { get; set; } = "";

    public string Variant { get; set; } = "";

    public string View { get; set; } = "";

    public Location? Location { get; set; }

    public string Unit { get; set; } = "";

    public PiecesDiff? Pieces { get; set; }

    public ConnectionsDiff? Connections { get; set; }

    public List<Stat> Stats { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();

    public List<Author> Authors { get; set; } = new();

    public List<string> Concepts { get; set; } = new();

    public DesignDiff MergeDiff(DesignDiff other)
    {
        return new DesignDiff
        {
            Name = string.IsNullOrEmpty(other.Name) ? Name : other.Name,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Icon = string.IsNullOrEmpty(other.Icon) ? Icon : other.Icon,
            Image = string.IsNullOrEmpty(other.Image) ? Image : other.Image,
            Variant = string.IsNullOrEmpty(other.Variant) ? Variant : other.Variant,
            View = string.IsNullOrEmpty(other.View) ? View : other.View,
            Location = other.Location ?? Location,
            Unit = string.IsNullOrEmpty(other.Unit) ? Unit : other.Unit,
            Pieces = other.Pieces ?? Pieces,
            Connections = other.Connections ?? Connections,
            Stats = other.Stats.Any() ? other.Stats : Stats,
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes,
            Authors = other.Authors.Any() ? other.Authors : Authors,
            Concepts = other.Concepts.Any() ? other.Concepts : Concepts
        };
    }

    public static implicit operator DesignDiff(DesignId id) => new() { Name = id.Name, Variant = id.Variant, View = id.View };
    public static implicit operator DesignDiff(Design design) => new() { Name = design.Name, Description = design.Description, Icon = design.Icon, Image = design.Image, Variant = design.Variant, View = design.View, Location = design.Location, Unit = design.Unit, Stats = design.Stats, Attributes = design.Attributes, Authors = design.Authors.Select(a => (Author)a).ToList(), Concepts = design.Concepts };
}


public class DesignsDiff : Model<DesignsDiff>
{

    public List<DesignId> Removed { get; set; } = new();

    public List<DesignDiff> Updated { get; set; } = new();

    public List<Design> Added { get; set; } = new();

    public static implicit operator DesignsDiff(List<Design> designs) => new() { Updated = designs.Select(d => (DesignDiff)d).ToList() };
}


public class FileDiff : Model<FileDiff>
{

    public string Url { get; set; } = "";

    public string Data { get; set; } = "";

    public int? Size { get; set; }

    public string Hash { get; set; } = "";

    public FileDiff MergeDiff(FileDiff other)
    {
        return new FileDiff
        {
            Url = string.IsNullOrEmpty(other.Url) ? Url : other.Url,
            Data = string.IsNullOrEmpty(other.Data) ? Data : other.Data,
            Size = other.Size ?? Size,
            Hash = string.IsNullOrEmpty(other.Hash) ? Hash : other.Hash
        };
    }

    public static implicit operator FileDiff(FileId id) => new() { Url = id.Url };
    public static implicit operator FileDiff(File file) => new() { Url = file.Url, Data = file.Data, Size = file.Size, Hash = file.Hash };
}


public class FilesDiff : Model<FilesDiff>
{

    public List<FileId> Removed { get; set; } = new();

    public List<FileDiff> Updated { get; set; } = new();

    public List<File> Added { get; set; } = new();

    public static implicit operator FilesDiff(List<File> files) => new() { Updated = files.Select(f => (FileDiff)f).ToList() };
}


public class KitDiff : Model<KitDiff>
{

    public string Name { get; set; } = "";

    public string Description { get; set; } = "";

    public string Icon { get; set; } = "";

    public string Image { get; set; } = "";

    public string Preview { get; set; } = "";

    public string Version { get; set; } = "";

    public string Remote { get; set; } = "";

    public string Homepage { get; set; } = "";

    public string License { get; set; } = "";

    public TypesDiff? Types { get; set; }

    public DesignsDiff? Designs { get; set; }

    public FilesDiff? Files { get; set; }

    public List<Attribute> Attributes { get; set; } = new();

    public KitDiff MergeDiff(KitDiff other)
    {
        return new KitDiff
        {
            Name = string.IsNullOrEmpty(other.Name) ? Name : other.Name,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Icon = string.IsNullOrEmpty(other.Icon) ? Icon : other.Icon,
            Image = string.IsNullOrEmpty(other.Image) ? Image : other.Image,
            Preview = string.IsNullOrEmpty(other.Preview) ? Preview : other.Preview,
            Version = string.IsNullOrEmpty(other.Version) ? Version : other.Version,
            Remote = string.IsNullOrEmpty(other.Remote) ? Remote : other.Remote,
            Homepage = string.IsNullOrEmpty(other.Homepage) ? Homepage : other.Homepage,
            License = string.IsNullOrEmpty(other.License) ? License : other.License,
            Types = other.Types ?? Types,
            Designs = other.Designs ?? Designs,
            Files = other.Files ?? Files,
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes
        };
    }

    public static implicit operator KitDiff(Kit kit) => new() { Name = kit.Name, Description = kit.Description, Icon = kit.Icon, Image = kit.Image, Preview = kit.Preview, Version = kit.Version, Remote = kit.Remote, Homepage = kit.Homepage, License = kit.License, Attributes = kit.Attributes };
}


public class KitId : Model<KitId>
{

    public string Name { get; set; } = "";
    public string ToIdString() => $"{Name}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"KitId({ToHumanIdString()})";

    public static implicit operator KitId(Kit kit) => new() { Name = kit.Name };
    public static implicit operator KitId(KitDiff diff) => new() { Name = diff.Name ?? "" };
}


public class KitsDiff : Model<KitsDiff>
{

    public List<KitId> Removed { get; set; } = new();

    public List<KitDiff> Updated { get; set; } = new();

    public List<Kit> Added { get; set; } = new();

    public static implicit operator KitsDiff(List<Kit> kits) => new() { Updated = kits.Select(k => (KitDiff)k).ToList() };
}

public enum DiffStatus
{
    Unchanged,
    Added,
    Removed,
    Modified
}

#endregion


public class PieceId : Model<PieceId>
{

    [JsonProperty("id_")]
    public string Id { get; set; } = "";
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    public static implicit operator PieceId(PieceDiff diff) => new() { Id = diff.Id ?? "" };
    public static implicit operator PieceId(Piece piece) => new() { Id = piece.Id };
}


public class PieceDiff : Model<PieceDiff>
{

    [JsonProperty("id_")]
    public string Id { get; set; } = "";

    public TypeId? Type { get; set; }

    public string Description { get; set; } = "";

    public Plane? Plane { get; set; }

    public DiagramPoint? Center { get; set; }

    public string Color { get; set; } = "";

    public float? Scale { get; set; }

    public Plane? MirrorPlane { get; set; }

    public List<Attribute> Attributes { get; set; } = new();

    public PieceDiff MergeDiff(PieceDiff other)
    {
        return new PieceDiff
        {
            Id = string.IsNullOrEmpty(other.Id) ? Id : other.Id,
            Type = other.Type ?? Type,
            Description = string.IsNullOrEmpty(other.Description) ? Description : other.Description,
            Plane = other.Plane ?? Plane,
            Center = other.Center ?? Center,
            Color = string.IsNullOrEmpty(other.Color) ? Color : other.Color,
            Scale = other.Scale ?? Scale,
            MirrorPlane = other.MirrorPlane ?? MirrorPlane,
            Attributes = other.Attributes.Any() ? other.Attributes : Attributes
        };
    }

    public static implicit operator PieceDiff(PieceId id) => new() { Id = id.Id };
    public static implicit operator PieceDiff(Piece piece) => new() { Id = piece.Id, Type = piece.Type, Description = piece.Description, Plane = piece.Plane, Center = piece.Center, Color = piece.Color, Scale = piece.Scale, MirrorPlane = piece.MirrorPlane, Attributes = piece.Attributes };
}


public class PiecesDiff : Model<PiecesDiff>
{

    public List<PieceId> Removed { get; set; } = new();

    public List<PieceDiff> Added { get; set; } = new();

    public List<PieceDiff> Modified { get; set; } = new();

    public static implicit operator PiecesDiff(List<Piece> pieces) => new() { Modified = pieces.Select(p => (PieceDiff)p).ToList() };
}


public class SideDiff : Model<SideDiff>
{

    public PieceId? Piece { get; set; }

    public PieceId? DesignPiece { get; set; } = null;

    public PortId? Port { get; set; }

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
/// <see href="https://github.com/usalu/semio#-piece-"/>
/// </summary>

public class Piece : Model<Piece>
{

    [JsonProperty("id_")]
    public string Id { get; set; } = "";

    public string Description { get; set; } = "";

    public TypeId? Type { get; set; }

    public DesignId? Design { get; set; }

    public Plane? Plane { get; set; }

    public DiagramPoint? Center { get; set; }

    public bool Hidden { get; set; } = false;

    public bool Locked { get; set; } = false;

    public string Color { get; set; } = "";

    public float Scale { get; set; } = 1.0f;

    public Plane? MirrorPlane { get; set; }

    public List<Prop> Props { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();
    public string ToIdString() => $"{Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"Pce({ToHumanIdString()})";

    public static implicit operator Piece(PieceId id) => new() { Id = id.Id };
    public static implicit operator Piece(PieceDiff diff) => new() { Id = diff.Id ?? "", Type = diff.Type, Design = null, Plane = diff.Plane, Center = diff.Center, Hidden = false, Locked = false, Color = diff.Color ?? "", Scale = diff.Scale ?? 1.0f, MirrorPlane = diff.MirrorPlane, Props = new(), Attributes = diff.Attributes ?? new() };
    public static implicit operator string(Piece piece) => piece.Id;
    public static implicit operator Piece(string id) => new() { Id = id };

    public Piece ApplyDiff(PieceDiff diff)
    {
        return new Piece
        {
            Id = Id,
            Type = diff.Type ?? Type,
            Design = Design,
            Plane = diff.Plane ?? Plane,
            Center = diff.Center ?? Center,
            Hidden = Hidden,
            Locked = Locked,
            Color = string.IsNullOrEmpty(diff.Color) ? Color : diff.Color,
            Scale = diff.Scale ?? Scale,
            MirrorPlane = diff.MirrorPlane ?? MirrorPlane,
            Props = Props,
            Attributes = diff.Attributes.Any() ? diff.Attributes : Attributes
        };
    }

    public PieceDiff CreateDiff()
    {
        return new PieceDiff
        {
            Id = Id,
            Type = Type,
            Plane = Plane,
            Center = Center,
            Color = Color,
            Scale = Scale,
            MirrorPlane = MirrorPlane,
            Attributes = Attributes
        };
    }

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
        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }
        return (isValid, errors);
    }

    public bool IsSameAs(Piece other)
    {
        return Utility.Normalize(Id) == Utility.Normalize(other.Id);
    }

    public bool IsFixed()
    {
        var isPlaneSet = Plane is not null;
        var isCenterSet = Center is not null;
        if (isPlaneSet != isCenterSet)
            throw new InvalidOperationException($"Piece {Id} has inconsistent plane and center");
        return isPlaneSet;
    }

    public string FindAttributeValue(string name, string defaultValue = "")
    {
        var attribute = Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in piece {Id}");
        return attribute?.Value ?? defaultValue;
    }

    public Piece SetAttribute(Attribute attribute)
    {
        var attributes = new List<Attribute>(Attributes ?? new List<Attribute>());
        var existingIndex = attributes.FindIndex(a => a.Key == attribute.Key);

        if (existingIndex >= 0)
            attributes[existingIndex] = attribute;
        else
            attributes.Add(attribute);

        return new Piece
        {
            Id = Id,
            Description = Description,
            Type = Type,
            Design = Design,
            Plane = Plane,
            Center = Center,
            Hidden = Hidden,
            Locked = Locked,
            Props = Props,
            Attributes = attributes
        };
    }
}


public class Side : Model<Side>
{

    public PieceId Piece { get; set; } = new();

    public PieceId? DesignPiece { get; set; } = null;

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
            Port = Port,
            Description = ""
        };
    }

    public SideDiff InverseDiff(SideDiff appliedDiff)
    {
        return new SideDiff
        {
            Piece = appliedDiff.Piece is not null ? Piece : null!,
            DesignPiece = appliedDiff.DesignPiece is not null ? DesignPiece : null!,
            Port = appliedDiff.Port is not null ? Port : null!,
            Description = !string.IsNullOrEmpty(appliedDiff.Description) ? "" : ""
        };
    }

    public override string ToString() => $"Sde({Piece.Id}" + (Port.Id != "" ? ":" + Port.Id : "") + ")";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-connection-"/>
/// </summary>

public class ConnectionId : Model<ConnectionId>
{

    public Side Connected { get; set; } = new();

    public Side Connecting { get; set; } = new();
    public static implicit operator ConnectionId(Connection connection) => new() { Connected = connection.Connected, Connecting = connection.Connecting };
    public static implicit operator ConnectionId(ConnectionDiff diff) => new() { Connected = new Side { Piece = diff.Connected?.Piece ?? new(), DesignPiece = diff.Connected?.DesignPiece, Port = diff.Connected?.Port ?? new() }, Connecting = new Side { Piece = diff.Connecting?.Piece ?? new(), DesignPiece = diff.Connecting?.DesignPiece, Port = diff.Connecting?.Port ?? new() } };
    public string ToIdString() => $"{Connected.Piece.Id + (Connected.Port.Id != "" ? ":" + Connected.Port.Id : "")}--{(Connecting.Port.Id != "" ? Connecting.Port.Id + ":" : "") + Connecting.Piece.Id}";
    public string ToHumanIdString() => $"{ToIdString()}";
    public override string ToString() => $"ConId({ToIdString()})";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-connection-"/>
/// </summary>

public class Connection : Model<Connection>
{
    private float _rotation;
    private float _tilt;
    private float _turn;


    public Side Connected { get; set; } = new();

    public Side Connecting { get; set; } = new();

    public string Description { get; set; } = "";

    public float Gap { get; set; } = 0;

    public float Shift { get; set; } = 0;

    public float Rise { get; set; } = 0;

    public float Rotation
    {
        get => _rotation;
        set => _rotation = (value % 360 + 360) % 360;
    }

    public float Turn
    {
        get => _turn;
        set => _turn = (value % 360 + 360) % 360;
    }
    public float Tilt
    {
        get => _tilt;
        set => _tilt = (value % 360 + 360) % 360;
    }


    public float X { get; set; }

    public float Y { get; set; } = 1;

    public List<Prop> Props { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Connected.Piece.Id + (Connected.Port.Id != "" ? ":" + Connected.Port.Id : "")}--{(Connecting.Port.Id != "" ? Connecting.Port.Id + ":" : "") + Connecting.Piece.Id}";

    public string ToHumanIdString() => $"{ToIdString()}";

    public override string ToString() => $"Con({ToIdString()})";

    public static implicit operator Connection(ConnectionId id) => new() { Connected = id.Connected, Connecting = id.Connecting };
    public static implicit operator Connection(ConnectionDiff diff) => new() { Connected = new Side { Piece = diff.Connected?.Piece ?? new PieceId(), DesignPiece = diff.Connected?.DesignPiece, Port = diff.Connected?.Port ?? new PortId() }, Connecting = new Side { Piece = diff.Connecting?.Piece ?? new PieceId(), DesignPiece = diff.Connecting?.DesignPiece, Port = diff.Connecting?.Port ?? new PortId() }, Description = diff.Description ?? "", Gap = diff.Gap ?? 0, Shift = diff.Shift ?? 0, Rise = diff.Rise ?? 0, Rotation = diff.Rotation ?? 0, Turn = diff.Turn ?? 0, Tilt = diff.Tilt ?? 0, X = diff.X ?? 0, Y = diff.Y ?? 1, Attributes = new() };

    public Connection ApplyDiff(ConnectionDiff diff)
    {
        return new Connection
        {
            Connected = diff.Connected is not null ? new Side
            {
                Piece = diff.Connected.Piece ?? Connected.Piece,
                DesignPiece = diff.Connected.DesignPiece ?? Connected.DesignPiece,
                Port = diff.Connected.Port ?? Connected.Port
            } : Connected,
            Connecting = diff.Connecting is not null ? new Side
            {
                Piece = diff.Connecting.Piece ?? Connecting.Piece,
                DesignPiece = diff.Connecting.DesignPiece ?? Connecting.DesignPiece,
                Port = diff.Connecting.Port ?? Connecting.Port
            } : Connecting,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Gap = diff.Gap ?? Gap,
            Shift = diff.Shift ?? Shift,
            Rise = diff.Rise ?? Rise,
            Rotation = diff.Rotation ?? Rotation,
            Turn = diff.Turn ?? Turn,
            Tilt = diff.Tilt ?? Tilt,
            X = diff.X ?? X,
            Y = diff.Y ?? Y,
            Props = Props,
            Attributes = Attributes
        };
    }

    public ConnectionDiff CreateDiff()
    {
        return new ConnectionDiff
        {
            Connected = new SideDiff
            {
                Piece = Connected.Piece,
                DesignPiece = Connected.DesignPiece,
                Port = Connected.Port
            },
            Connecting = new SideDiff
            {
                Piece = Connecting.Piece,
                DesignPiece = Connecting.DesignPiece,
                Port = Connecting.Port
            },
            Description = Description,
            Gap = Gap,
            Shift = Shift,
            Rise = Rise,
            Rotation = Rotation,
            Turn = Turn,
            Tilt = Tilt,
            X = X,
            Y = Y
        };
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

        if (Math.Abs(X) < Constants.Tolerance && Math.Abs(Y) < Constants.Tolerance)
        {
            isValid = false;
            errors.Add("The offset (x,y) must not be the zero vector.");
        }

        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
        }

        return (isValid, errors);
    }

    public bool IsSameAs(Connection other, bool strict = false)
    {
        var connectedPiece1 = Connected?.Piece?.Id ?? "";
        var connectingPiece1 = Connecting?.Piece?.Id ?? "";
        var connectedPiece2 = other.Connected?.Piece?.Id ?? "";
        var connectingPiece2 = other.Connecting?.Piece?.Id ?? "";

        var isExactMatch = connectingPiece1 == connectingPiece2 && connectedPiece1 == connectedPiece2;
        if (strict) return isExactMatch;
        var isSwappedMatch = connectingPiece1 == connectedPiece2 && connectedPiece1 == connectingPiece2;
        return isExactMatch || isSwappedMatch;
    }

    public string FindAttributeValue(string name, string defaultValue = "")
    {
        var attribute = Attributes?.FirstOrDefault(a => a.Key == name);
        if (attribute is null && defaultValue is null)
            throw new InvalidOperationException($"Attribute {name} not found in connection");
        return attribute?.Value ?? defaultValue;
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
            Props = Props,
            Attributes = attributes
        };
    }
}


public class DesignId : Model<DesignId>
{

    public string Name { get; set; } = "";

    public string Variant { get; set; } = "";

    public string View { get; set; } = "";
    public static implicit operator DesignId(Design design) => new() { Name = design.Name, Variant = design.Variant, View = design.View };
    public static implicit operator DesignId(DesignDiff diff) => new() { Name = diff.Name ?? "", Variant = diff.Variant ?? "", View = diff.View ?? "" };

    public string ToIdString() => $"{Name}#{Variant}#{View}";
    public string ToHumanIdString() => $"{Name}{(Variant == "" ? "" : ", " + Variant)}{(View == "" ? "" : ", " + View)}";
    public string ToId() => ToIdString();
    public string ToHumanId() => ToHumanIdString();
    public override string ToString() => $"DsnId({ToHumanIdString()})";
}

/// <summary>
/// <see href="https://github.com/usalu/semio#-design-"/>
/// </summary>

public class Design : Model<Design>
{

    public string Name { get; set; } = "";

    public string Variant { get; set; } = "";

    public string View { get; set; } = "";

    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Image { get; set; } = "";

    public List<string> Concepts { get; set; } = new();

    public List<AuthorId> Authors { get; set; } = new();

    public Location? Location { get; set; }

    public string Unit { get; set; } = "";

    public bool Scalable { get; set; } = true;

    public bool Mirrorable { get; set; } = true;

    public List<Layer> Layers { get; set; } = new();

    public List<Piece> Pieces { get; set; } = new();

    public List<Group> Groups { get; set; } = new();

    public List<Connection> Connections { get; set; } = new();

    public List<Prop> Props { get; set; } = new();

    public List<Stat> Stats { get; set; } = new();

    public List<Attribute> Attributes { get; set; } = new();

    public string ToIdString() => $"{Name}#{Variant}#{View}";
    public string ToHumanIdString() => $"{Name}" + (Variant.Length == 0 ? "" : $", {Variant}") + (View.Length == 0 ? "" : $", {View}");
    public override string ToString() => $"Dsn({ToHumanIdString()})";

    public static implicit operator Design(DesignId id) => new() { Name = id.Name, Variant = id.Variant, View = id.View };
    public static implicit operator Design(DesignDiff diff) => new() { Name = diff.Name ?? "", Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Variant = diff.Variant ?? "", View = diff.View ?? "", Location = diff.Location, Unit = diff.Unit ?? "", Attributes = diff.Attributes ?? new(), Authors = diff.Authors?.Select(a => (AuthorId)a).ToList() ?? new(), Concepts = diff.Concepts ?? new() };
    public static implicit operator string(Design design) => design.Name;
    public static implicit operator Design(string name) => new() { Name = name };

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
            Name = string.IsNullOrEmpty(diff.Name) ? Name : diff.Name,
            Description = string.IsNullOrEmpty(diff.Description) ? Description : diff.Description,
            Icon = string.IsNullOrEmpty(diff.Icon) ? Icon : diff.Icon,
            Image = string.IsNullOrEmpty(diff.Image) ? Image : diff.Image,
            Variant = string.IsNullOrEmpty(diff.Variant) ? Variant : diff.Variant,
            View = string.IsNullOrEmpty(diff.View) ? View : diff.View,
            Location = diff.Location ?? Location,
            Unit = string.IsNullOrEmpty(diff.Unit) ? Unit : diff.Unit,
            Pieces = pieces,
            Connections = connections,
            Props = Props,
            Stats = diff.Stats.Any() ? diff.Stats : Stats,
            Authors = diff.Authors.Any() ? diff.Authors.Select(a => new AuthorId { Email = a.Email }).ToList() : Authors,
            Attributes = diff.Attributes.Any() ? diff.Attributes : Attributes,
            Concepts = diff.Concepts.Any() ? diff.Concepts : Concepts
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
            Variant = Variant,
            View = View,
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
            Authors = Authors.Select(a => new Author { Name = "", Email = a.Email }).ToList(),
            Attributes = Attributes,
            Concepts = Concepts
        };
    }

    private List<Piece> ApplyPiecesDiff(List<Piece> original, PiecesDiff diff)
    {
        var result = original.Where(p => !diff.Removed.Any(r => r.Id == p.Id)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(p => p.Id == updated.Id);
            if (index >= 0)
                result[index] = result[index].ApplyDiff(updated);
        }
        result.AddRange(diff.Added.Select(a => new Piece
        {
            Id = a.Id,
            Description = a.Description,
            Type = a.Type,
            Plane = a.Plane,
            Center = a.Center,
            Attributes = a.Attributes
        }));
        return result;
    }

    private PiecesDiff CreatePiecesDiff(List<Piece> original, List<Piece> modified)
    {
        var originalIds = original.Select(p => p.Id).ToHashSet();
        var modifiedIds = modified.Select(p => p.Id).ToHashSet();

        return new PiecesDiff
        {
            Removed = original.Where(p => !modifiedIds.Contains(p.Id)).Select(p => new PieceId { Id = p.Id }).ToList(),
            Modified = original.Where(p => modifiedIds.Contains(p.Id))
                .SelectMany(p =>
                {
                    var modifiedPiece = modified.First(m => m.Id == p.Id);
                    var diff = p.CreateDiff();
                    return !Equals(p, modifiedPiece) ? new[] { diff } : new PieceDiff[] { };
                })
                .ToList(),
            Added = modified.Where(p => !originalIds.Contains(p.Id)).Select(p => new PieceDiff
            {
                Id = p.Id,
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
            r.Connected.Piece.Id == c.Connected.Piece.Id &&
            r.Connecting.Piece.Id == c.Connecting.Piece.Id)).ToList();

        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(c =>
                c.Connected.Piece.Id == (updated.Connected?.Piece?.Id ?? c.Connected.Piece.Id) &&
                c.Connecting.Piece.Id == (updated.Connecting?.Piece?.Id ?? c.Connecting.Piece.Id));
            if (index >= 0)
                result[index] = result[index].ApplyDiff(updated);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private ConnectionsDiff CreateConnectionsDiff(List<Connection> original, List<Connection> modified)
    {
        var originalKeys = original.Select(c => (c.Connected.Piece.Id, c.Connecting.Piece.Id)).ToHashSet();
        var modifiedKeys = modified.Select(c => (c.Connected.Piece.Id, c.Connecting.Piece.Id)).ToHashSet();

        return new ConnectionsDiff
        {
            Removed = original.Where(c => !modifiedKeys.Contains((c.Connected.Piece.Id, c.Connecting.Piece.Id)))
                .Select(c => new ConnectionId { Connected = c.Connected, Connecting = c.Connecting }).ToList(),
            Updated = original.Where(c => modifiedKeys.Contains((c.Connected.Piece.Id, c.Connecting.Piece.Id)))
                .SelectMany(c =>
                {
                    var modifiedConnection = modified.First(m => m.Connected.Piece.Id == c.Connected.Piece.Id && m.Connecting.Piece.Id == c.Connecting.Piece.Id);
                    var diff = c.CreateDiff();
                    return !Equals(c, modifiedConnection) ? new[] { diff } : new ConnectionDiff[] { };
                })
                .ToList(),
            Added = modified.Where(c => !originalKeys.Contains((c.Connected.Piece.Id, c.Connecting.Piece.Id))).ToList()
        };
    }

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
            var root = subGraph.Vertices.FirstOrDefault(p => pieces[p].Plane is not null);
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
                if (piece.Plane is null) piece.Plane = new Plane();
                if (piece.Center is null) piece.Center = new DiagramPoint();
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
                var semioAttribute = child.Attributes.FirstOrDefault(q => q.Key == "semio.parent");
                if (semioAttribute is not null)
                {
                    semioAttribute.Value = parent.Id;
                }
                else
                {
                    child.Attributes.Add(new Attribute
                    {
                        Key = "semio.parent",
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
            if (piece.Center is not null)
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

        foreach (var attribute in Attributes)
        {
            var (isValidAttribute, errorsAttribute) = attribute.Validate();
            isValid = isValid && isValidAttribute;
            errors.AddRange(errorsAttribute.Select(e => $"A attribute({attribute.ToHumanIdString()}) is invalid: " + e));
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

    public bool IsSameAs(Design other)
    {
        if (other == null) return false;
        return Name == other.Name &&
               Utility.Normalize(Variant) == Utility.Normalize(other.Variant) &&
               Utility.Normalize(View) == Utility.Normalize(other.View);
    }

    public Piece FindPiece(string pieceId)
    {
        var piece = Pieces.FirstOrDefault(p => p.Id == pieceId);
        if (piece == null) throw new ArgumentException($"Piece {pieceId} not found in design");
        return piece;
    }

    public Connection FindConnection(Connection connectionToFind, bool strict = false)
    {
        var connection = Connections.FirstOrDefault(c => c.IsSameAs(connectionToFind, strict));
        if (connection == null)
            throw new ArgumentException($"Connection {connectionToFind.Connected.Piece.Id} -> {connectionToFind.Connecting.Piece.Id} not found in design");
        return connection;
    }

    public List<Connection> FindPieceConnections(string pieceId)
    {
        return Connections.Where(c =>
            c.Connected.Piece.Id == pieceId ||
            c.Connecting.Piece.Id == pieceId).ToList();
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
            Variant = Variant,
            View = View,
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

    public Design RemovePiece(string pieceId)
    {
        var newPieces = Pieces.Where(p => p.Id != pieceId).ToList();
        var newConnections = Connections.Where(c =>
            c.Connected.Piece.Id != pieceId &&
            c.Connecting.Piece.Id != pieceId).ToList();
        return new Design
        {
            Name = Name,
            Description = Description,
            Icon = Icon,
            Image = Image,
            Variant = Variant,
            View = View,
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
            Variant = Variant,
            View = View,
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
            Variant = Variant,
            View = View,
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
            Variant = Variant,
            View = View,
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

/// <summary>
/// <see href="https://github.com/usalu/semio#-kit-"/>
/// </summary>

public class Kit : Model<Kit>
{

    public string Name { get; set; } = "";

    public string Version { get; set; } = "";

    public string Description { get; set; } = "";
    public string Icon { get; set; } = "";
    public string Image { get; set; } = "";

    public List<string> Concepts { get; set; } = new();

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

    public List<Type> Types { get; set; } = new();

    public List<Design> Designs { get; set; } = new();

    public static implicit operator Kit(KitDiff diff) => new() { Name = diff.Name ?? "", Description = diff.Description ?? "", Icon = diff.Icon ?? "", Image = diff.Image ?? "", Preview = diff.Preview ?? "", Version = diff.Version ?? "", Remote = diff.Remote ?? "", Homepage = diff.Homepage ?? "", License = diff.License ?? "", Attributes = diff.Attributes ?? new() };
    public static implicit operator string(Kit kit) => kit.Name;
    public static implicit operator Kit(string name) => new() { Name = name };

    public Kit ApplyDiff(KitDiff diff)
    {
        var types = Types;
        var designs = Designs;

        if (diff.Types is not null)
        {
            types = ApplyTypesDiff(Types, diff.Types);
        }
        if (diff.Designs is not null)
        {
            designs = ApplyDesignsDiff(Designs, diff.Designs);
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
            Types = types,
            Designs = designs,
            Attributes = diff.Attributes.Any() ? diff.Attributes : Attributes
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
            Attributes = Attributes
        };
    }

    private List<Type> ApplyTypesDiff(List<Type> original, TypesDiff diff)
    {
        var result = original.Where(t => !diff.Removed.Any(r => r.Name == t.Name && r.Variant == t.Variant)).ToList();
        foreach (var updated in diff.Modified)
        {
            var index = result.FindIndex(t => t.Name == (updated.Name ?? t.Name) && t.Variant == (updated.Variant ?? t.Variant));
            if (index >= 0)
                result[index] = result[index].ApplyDiff(updated);
        }
        result.AddRange(diff.Added.Select(a => new Type
        {
            Name = a.Name,
            Description = a.Description,
            Icon = a.Icon,
            Image = a.Image,
            Variant = a.Variant,
            Stock = a.Stock ?? 2147483647,
            Virtual = a.Virtual ?? false,
            Unit = a.Unit,
            Location = a.Location,
            Representations = a.Representations,
            Ports = a.Ports,
            Authors = a.Authors.Select(auth => new AuthorId { Email = auth.Email }).ToList(),
            Attributes = a.Attributes
        }));
        return result;
    }

    private TypesDiff CreateTypesDiff(List<Type> original, List<Type> modified)
    {
        var originalKeys = original.Select(t => (t.Name, t.Variant)).ToHashSet();
        var modifiedKeys = modified.Select(t => (t.Name, t.Variant)).ToHashSet();

        return new TypesDiff
        {
            Removed = original.Where(t => !modifiedKeys.Contains((t.Name, t.Variant)))
                .Select(t => new TypeId { Name = t.Name, Variant = t.Variant }).ToList(),
            Modified = original.Where(t => modifiedKeys.Contains((t.Name, t.Variant)))
                .SelectMany(t =>
                {
                    var modifiedType = modified.First(m => m.Name == t.Name && m.Variant == t.Variant);
                    var diff = t.CreateDiff();
                    return !Equals(t, modifiedType) ? new[] { diff } : new TypeDiff[] { };
                })
                .ToList(),
            Added = modified.Where(t => !originalKeys.Contains((t.Name, t.Variant))).Select(t => new TypeDiff
            {
                Name = t.Name,
                Description = t.Description,
                Icon = t.Icon,
                Image = t.Image,
                Variant = t.Variant,
                Stock = t.Stock,
                Virtual = t.Virtual,
                Unit = t.Unit,
                Location = t.Location,
                Representations = t.Representations,
                Ports = t.Ports,
                Authors = t.Authors.Select(a => new Author { Email = a.Email }).ToList(),
                Attributes = t.Attributes
            }).ToList()
        };
    }

    private List<Design> ApplyDesignsDiff(List<Design> original, DesignsDiff diff)
    {
        var result = original.Where(d => !diff.Removed.Any(r => r.Name == d.Name && r.Variant == d.Variant && r.View == d.View)).ToList();
        foreach (var updated in diff.Updated)
        {
            var index = result.FindIndex(d => d.Name == (updated.Name ?? d.Name) && d.Variant == (updated.Variant ?? d.Variant) && d.View == (updated.View ?? d.View));
            if (index >= 0)
                result[index] = result[index].ApplyDiff(updated);
        }
        result.AddRange(diff.Added);
        return result;
    }

    private DesignsDiff CreateDesignsDiff(List<Design> original, List<Design> modified)
    {
        var originalKeys = original.Select(d => (d.Name, d.Variant, d.View)).ToHashSet();
        var modifiedKeys = modified.Select(d => (d.Name, d.Variant, d.View)).ToHashSet();

        return new DesignsDiff
        {
            Removed = original.Where(d => !modifiedKeys.Contains((d.Name, d.Variant, d.View)))
                .Select(d => new DesignId { Name = d.Name, Variant = d.Variant, View = d.View }).ToList(),
            Updated = original.Where(d => modifiedKeys.Contains((d.Name, d.Variant, d.View)))
                .SelectMany(d =>
                {
                    var modifiedDesign = modified.First(m => m.Name == d.Name && m.Variant == d.Variant && m.View == d.View);
                    var diff = d.CreateDiff();
                    return !Equals(d, modifiedDesign) ? new[] { diff } : new DesignDiff[] { };
                })
                .ToList(),
            Added = modified.Where(d => !originalKeys.Contains((d.Name, d.Variant, d.View))).ToList()
        };
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
        if (other == null) return false;
        return Name == other.Name;
    }

    public Type FindType(string typeName, string variant = "")
    {
        var normalizedVariant = Utility.Normalize(variant);
        var type = Types.FirstOrDefault(t =>
            t.Name == typeName &&
            Utility.Normalize(t.Variant) == normalizedVariant);
        if (type == null) throw new ArgumentException($"Type {typeName} not found in kit {Name}");
        return type;
    }

    public Design FindDesign(string designName, string variant = "", string view = "")
    {
        var normalizedVariant = Utility.Normalize(variant);
        var normalizedView = Utility.Normalize(view);
        var design = Designs.FirstOrDefault(d =>
            d.Name == designName &&
            Utility.Normalize(d.Variant) == normalizedVariant &&
            Utility.Normalize(d.View) == normalizedView);
        if (design == null) throw new ArgumentException($"Design {designName} not found in kit {Name}");
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

    public Kit RemoveType(string typeName, string variant = "")
    {
        var normalizedVariant = Utility.Normalize(variant);
        var newTypes = Types.Where(t =>
            !(t.Name == typeName && Utility.Normalize(t.Variant) == normalizedVariant)).ToList();
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

    public Kit RemoveDesign(string designName, string variant = "", string view = "")
    {
        var normalizedVariant = Utility.Normalize(variant);
        var normalizedView = Utility.Normalize(view);
        var newDesigns = Designs.Where(d =>
            !(d.Name == designName &&
              Utility.Normalize(d.Variant) == normalizedVariant &&
              Utility.Normalize(d.View) == normalizedView)).ToList();
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


#endregion

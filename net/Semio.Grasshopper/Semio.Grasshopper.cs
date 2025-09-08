#region Header

//Semio.Grasshopper.cs
//2020-2025 Ueli Saluz

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

#endregion Header

#region TODOs

// TODO: Think of modeling components that are resilient to future schema changes.
// TODO: Refactor EngineComponent with GetInput and GetPersitanceInput etc. Very confusing. Probably no abstracting is better.
// TODO: GetProps and SetProps (includes children) is not consistent with the prop naming in python (does not include children).
// TODO: Add toplevel scanning for kits wherever a directory is given
// Maybe extension function for components. The repeated code looks something like this:
// if (!DA.GetData(_, ref path))
//      path = OnPingDocument().IsFilePathDefined
//          ? Path.GetDirectoryName(OnPingDocument().FilePath)
//          : Directory.GetCurrentDirectory();
// TODO: IsInvalid is used to check null state which is not clean.
// Think of a better way to handle this.
// The invalid check happen twice and code is duplicated.
// TODO: Figure out why cast from Piece to Text is not triggering the casts. ToString has somehow has precedence.
// TODO: NameM.ToLower() doesn't work for composite names. E.g. "DiagramPoint" -> "diagrampoint".
// TODO: Implement a status check and wait for the engine to be ready

#endregion

using System.Collections.Immutable;
using System.Diagnostics;
using System.Drawing;
using System.Linq;
using System.Reflection;
using System.Security.Cryptography;
using System.Text;
using System.Net.Http;
using GH_IO.Serialization;
using Grasshopper;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Parameters;
using Grasshopper.Kernel.Types;
using Humanizer;
using Rhino;
using Rhino.Geometry;
using Semio;
using System.Text.RegularExpressions;

namespace Semio.Grasshopper;

#region Constants

public static class Constants
{
    public const string Category = Semio.Constants.Name;
    public const string Version = "6.0.0";
}

public class Semio_GrasshopperInfo : GH_AssemblyInfo
{
    public override string Name => Semio.Constants.Name;
    public override Bitmap Icon => Resources.semio_24x24;
    public override Bitmap AssemblyIcon => Resources.semio_24x24;
    public override string Description => "semio within 🦗.";
    public override Guid Id => new("FE587CBF-5F7D-4091-AA6D-D9D30CF80B64");
    public override string Version => Constants.Version;
    public override string AuthorName => "Ueli Saluz";
    public override string AuthorContact => "ueli@semio-tech.com";
}

public class SemioCategoryIcon : GH_AssemblyPriority
{
    public override GH_LoadingInstruction PriorityLoad()
    {
        Instances.ComponentServer.AddCategoryIcon(Constants.Category, Resources.semio_24x24);
        Instances.ComponentServer.AddCategorySymbolName(Constants.Category, 'S');
        return GH_LoadingInstruction.Proceed;
    }
}

#endregion Constants

#region Utility

public static class Utility
{
    public static string ToIdString(this string name)
    {
        return Regex.Replace(name.ToLower(), @"[^a-z0-9]+", "-").Trim('-');
    }

    public static string ExtractPattern(string pattern, string text)
    {
        var regex = new Regex(pattern);
        var match = regex.Match(text);
        return match.Success ? match.Groups[1].Value : string.Empty;
    }

    internal static void SolveBasic(IGH_DataAccess DA, int inputIndex, int outputIndex, Func<object, object> solver)
    {
        object? input = null;
        DA.GetData(inputIndex, ref input);
        if (input is not null)
        {
            var output = solver(input);
            DA.SetData(outputIndex, output);
        }
    }

    internal static void SolveList<T>(IGH_DataAccess DA, int inputIndex, int outputIndex, Func<T, object> solver)
    {
        var inputs = new List<T>();
        DA.GetDataList(inputIndex, inputs);
        var outputs = inputs.Select(solver).ToList();
        DA.SetDataList(outputIndex, outputs);
    }
}

#endregion Utility

#region Converters

public static class RhinoConverter
{
    public static Point3d ToRhino(this Point point) => new(point.X, point.Y, point.Z);
    public static Point ToSemio(this Point3d point) => new() { X = (float)point.X, Y = (float)point.Y, Z = (float)point.Z };
    public static Vector3d ToRhino(this Vector vector) => new(vector.X, vector.Y, vector.Z);
    public static Vector ToSemio(this Vector3d vector) => new() { X = (float)vector.X, Y = (float)vector.Y, Z = (float)vector.Z };

    // TODO: Fix Plane conversion later - complex type mapping issues
    // public static Plane ToRhino(this Semio.Plane plane) 
    // public static Semio.Plane ToSemio(this Plane plane)
}

#endregion Converters

#region Bases

public abstract class ModelGoo<TModel> : GH_Goo<TModel> where TModel : Model<TModel>, new()
{
    public ModelGoo() { }
    public ModelGoo(TModel value) { Value = value; }
    public override bool IsValid => Value is not null;
    public override string TypeName => typeof(TModel).Name;
    public override string TypeDescription => GetModelDescription();

    protected abstract string GetModelDescription();

    public override IGH_Goo Duplicate()
    {
        var duplicate = (ModelGoo<TModel>)Activator.CreateInstance(GetType())!;
        if (Value is not null)
            duplicate.Value = Value.DeepClone();
        return duplicate;
    }

    public override string ToString() => Value?.ToString() ?? string.Empty;

    public override bool Write(GH_IWriter writer)
    {
        if (Value is not null)
            writer.SetString(typeof(TModel).Name, Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        var serialized = reader.GetString(typeof(TModel).Name);
        if (!string.IsNullOrEmpty(serialized))
        {
            var deserialized = serialized.Deserialize<TModel>();
            if (deserialized is not null)
                Value = deserialized;
        }
        return base.Read(reader);
    }

    protected virtual bool CustomCastTo<Q>(ref Q target) => false;
    protected virtual bool CustomCastFrom(object source) => false;

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TModel)) && Value is not null)
        {
            target = (Q)(object)Value;
            return true;
        }
        return CustomCastTo(ref target);
    }

    public override bool CastFrom(object source)
    {
        if (source is null) return false;
        if (source is TModel model)
        {
            Value = model;
            return true;
        }
        return CustomCastFrom(source);
    }
}

public abstract class ModelParam<TGoo, TModel> : GH_PersistentParam<TGoo>
    where TGoo : ModelGoo<TModel>
    where TModel : Model<TModel>, new()
{
    protected ModelParam(string name, string nickname, string description)
        : base(name, nickname, description, Constants.Category, "Params") { }

    protected override Bitmap Icon => GetParamIcon();
    protected abstract Bitmap GetParamIcon();
    protected override GH_GetterResult Prompt_Singular(ref TGoo value) => GH_GetterResult.cancel;
    protected override GH_GetterResult Prompt_Plural(ref List<TGoo> values) => GH_GetterResult.cancel;
}

public abstract class EnumGoo<TEnum> : GH_Goo<TEnum> where TEnum : struct, Enum
{
    public EnumGoo() { }
    public EnumGoo(TEnum value) { Value = value; }
    public override bool IsValid => true;
    public override string TypeName => typeof(TEnum).Name;
    public override string TypeDescription => typeof(TEnum).Name;

    public override IGH_Goo Duplicate()
    {
        var duplicate = (EnumGoo<TEnum>)Activator.CreateInstance(GetType())!;
        duplicate.Value = Value;
        return duplicate;
    }

    public override string ToString() => Value.ToString();

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString(typeof(TEnum).Name, Value.ToString());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        var enumString = reader.GetString(typeof(TEnum).Name);
        if (Enum.TryParse<TEnum>(enumString, out var enumValue))
            Value = enumValue;
        return base.Read(reader);
    }

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q) == typeof(string))
        {
            target = (Q)(object)Value.ToString();
            return true;
        }
        if (typeof(Q) == typeof(int))
        {
            target = (Q)(object)Convert.ToInt32(Value);
            return true;
        }
        return false;
    }

    public override bool CastFrom(object source)
    {
        if (source is null) return false;
        if (source is TEnum enumValue)
        {
            Value = enumValue;
            return true;
        }
        if (source is string stringValue && Enum.TryParse<TEnum>(stringValue, out var parsedValue))
        {
            Value = parsedValue;
            return true;
        }
        if (source is int intValue && Enum.IsDefined(typeof(TEnum), intValue))
        {
            Value = (TEnum)Enum.ToObject(typeof(TEnum), intValue);
            return true;
        }
        return false;
    }
}

public abstract class EnumParam<TEnumGoo, TEnum> : GH_Param<TEnumGoo>
    where TEnumGoo : EnumGoo<TEnum>, new()
    where TEnum : struct, Enum
{
    protected EnumParam(string name, string nickname, string description)
        : base(name, nickname, description, Constants.Category, "Params", GH_ParamAccess.item) { }

    public override Guid ComponentGuid => GetComponentGuid();
    protected abstract Guid GetComponentGuid();
    protected override Bitmap Icon => GetEnumIcon();
    protected abstract Bitmap GetEnumIcon();
}

public abstract class Component : GH_Component
{
    protected Component(string name, string nickname, string description, string subcategory)
        : base(name, nickname, description, Constants.Category, subcategory) { }
}

public abstract class ModelComponent<TParam, TGoo, TModel> : Component
    where TParam : ModelParam<TGoo, TModel>, new()
    where TGoo : ModelGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected ModelComponent(string name, string nickname, string description)
        : base(name, nickname, description, "Modeling") { }

    protected override Bitmap Icon => GetComponentIcon();
    protected abstract Bitmap GetComponentIcon();
    public override GH_Exposure Exposure => GH_Exposure.primary;
    public override Guid ComponentGuid => GetComponentGuid();
    protected abstract Guid GetComponentGuid();

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        RegisterModelInputs(pManager);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        RegisterModelOutputs(pManager);
    }

    protected abstract void RegisterModelInputs(GH_InputParamManager pManager);
    protected abstract void RegisterModelOutputs(GH_OutputParamManager pManager);

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        SolveModelInstance(DA);
    }

    protected abstract void SolveModelInstance(IGH_DataAccess DA);
}

public abstract class IdComponent<TParam, TGoo, TModel> : ModelComponent<TParam, TGoo, TModel>
    where TParam : ModelParam<TGoo, TModel>, new()
    where TGoo : ModelGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected IdComponent(string name, string nickname, string description)
        : base(name, nickname, description) { }

    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

public abstract class DiffComponent<TParam, TGoo, TModel> : ModelComponent<TParam, TGoo, TModel>
    where TParam : ModelParam<TGoo, TModel>, new()
    where TGoo : ModelGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected DiffComponent(string name, string nickname, string description)
        : base(name, nickname, description) { }

    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}

public abstract class ScriptingComponent : Component
{
    protected ScriptingComponent(string name, string nickname, string description, string subcategory)
        : base(name, nickname, description, subcategory) { }
}

public abstract class SerializeComponent<TParam, TGoo, TModel> : ScriptingComponent
    where TParam : ModelParam<TGoo, TModel>, new()
    where TGoo : ModelGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected SerializeComponent(string name, string nickname, string description)
        : base(name, nickname, description, "Serialization") { }

    public override GH_Exposure Exposure => GH_Exposure.secondary;
    public override Guid ComponentGuid => GetComponentGuid();
    protected abstract Guid GetComponentGuid();

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), GetModelTypeName(), GetModelNickname(), GetModelDescription(), GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "T", "Serialized text", GH_ParamAccess.item);
    }

    protected abstract string GetModelTypeName();
    protected abstract string GetModelNickname();
    protected abstract string GetModelDescription();

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var goo = default(TGoo);
        if (!DA.GetData(0, ref goo) || goo?.Value is null) return;

        var serialized = goo.Value.Serialize();
        DA.SetData(0, serialized);
    }
}

public abstract class DeserializeComponent<TParam, TGoo, TModel> : ScriptingComponent
    where TParam : ModelParam<TGoo, TModel>, new()
    where TGoo : ModelGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected DeserializeComponent(string name, string nickname, string description)
        : base(name, nickname, description, "Serialization") { }

    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    public override Guid ComponentGuid => GetComponentGuid();
    protected abstract Guid GetComponentGuid();

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "T", "Serialized text", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), GetModelTypeName(), GetModelNickname(), GetModelDescription(), GH_ParamAccess.item);
    }

    protected abstract string GetModelTypeName();
    protected abstract string GetModelNickname();
    protected abstract string GetModelDescription();

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        if (!DA.GetData(0, ref text) || string.IsNullOrEmpty(text)) return;

        var model = text.Deserialize<TModel>();
        if (model is not null)
        {
            var goo = new TGoo { Value = model };
            DA.SetData(0, goo);
        }
    }
}

#endregion Bases

#region Attribute

public class AttributeIdGoo : ModelGoo<AttributeId>
{
    public AttributeIdGoo() { }
    public AttributeIdGoo(AttributeId value) : base(value) { }
    protected override string GetModelDescription() => "The ID of the attribute.";
}

public class AttributeIdParam : ModelParam<AttributeIdGoo, AttributeId>
{
    public AttributeIdParam() : base("AttributeId", "AI", "The ID of the attribute.") { }
    protected override Bitmap GetParamIcon() => Resources.attribute_id_24x24;
    public override Guid ComponentGuid => new("1F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class AttributeIdComponent : IdComponent<AttributeIdParam, AttributeIdGoo, AttributeId>
{
    public AttributeIdComponent() : base("Attribute ID", "AI", "Create an attribute ID") { }

    protected override Bitmap GetComponentIcon() => Resources.attribute_id_24x24;
    protected override Guid GetComponentGuid() => new("2F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "Attribute key", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeIdParam(), "AttributeId", "AI", "The created attribute ID", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var key = "";
        if (!DA.GetData(0, ref key)) return;

        var attributeId = new AttributeId { Key = key };
        var goo = new AttributeIdGoo(attributeId);
        DA.SetData(0, goo);
    }
}

public class AttributeDiffGoo : ModelGoo<AttributeDiff>
{
    public AttributeDiffGoo() { }
    public AttributeDiffGoo(AttributeDiff value) : base(value) { }
    protected override string GetModelDescription() => "A diff for attributes.";
}

public class AttributeDiffParam : ModelParam<AttributeDiffGoo, AttributeDiff>
{
    public AttributeDiffParam() : base("AttributeDiff", "AD", "A diff for attributes.") { }
    protected override Bitmap GetParamIcon() => Resources.attribute_diff_24x24;
    public override Guid ComponentGuid => new("3F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class AttributeDiffComponent : DiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public AttributeDiffComponent() : base("Attribute Diff", "AD", "Create an attribute diff") { }

    protected override Bitmap GetComponentIcon() => Resources.attribute_diff_24x24;
    protected override Guid GetComponentGuid() => new("4F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "Attribute key", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "V", "Attribute value", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Definition", "D", "Attribute definition", GH_ParamAccess.item, "");
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeDiffParam(), "AttributeDiff", "AD", "The created attribute diff", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var key = "";
        var value = "";
        var definition = "";

        if (!DA.GetData(0, ref key)) return;
        DA.GetData(1, ref value);
        DA.GetData(2, ref definition);

        var attributeDiff = new AttributeDiff { Key = key, Value = value, Definition = definition };
        var goo = new AttributeDiffGoo(attributeDiff);
        DA.SetData(0, goo);
    }
}

public class AttributeGoo : ModelGoo<Attribute>
{
    public AttributeGoo() { }
    public AttributeGoo(Attribute value) : base(value) { }
    protected override string GetModelDescription() => "A attribute is a key value pair with an optional definition.";
}

public class AttributeParam : ModelParam<AttributeGoo, Attribute>
{
    public AttributeParam() : base("Attribute", "At", "A attribute is a key value pair with an optional definition.") { }
    protected override Bitmap GetParamIcon() => Resources.attribute_24x24;
    public override Guid ComponentGuid => new("5F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class AttributeComponent : ModelComponent<AttributeParam, AttributeGoo, Attribute>
{
    public AttributeComponent() : base("Attribute", "At", "Create an attribute") { }

    protected override Bitmap GetComponentIcon() => Resources.attribute_modify_24x24;
    protected override Guid GetComponentGuid() => new("6F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "Attribute key", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "V", "Attribute value", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Definition", "D", "Attribute definition", GH_ParamAccess.item, "");
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeParam(), "Attribute", "At", "The created attribute", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var key = "";
        var value = "";
        var definition = "";

        if (!DA.GetData(0, ref key)) return;
        DA.GetData(1, ref value);
        DA.GetData(2, ref definition);

        var attribute = new Attribute { Key = key, Value = value, Definition = definition };
        var goo = new AttributeGoo(attribute);
        DA.SetData(0, goo);
    }
}

public class SerializeAttributeComponent : SerializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public SerializeAttributeComponent() : base("Serialize Attribute", "SAt", "Serialize an attribute to text") { }

    protected override Bitmap Icon => Resources.attribute_serialize_24x24;
    protected override Guid GetComponentGuid() => new("7F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelNickname() => "At";
    protected override string GetModelDescription() => "The attribute to serialize";
}

public class DeserializeAttributeComponent : DeserializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public DeserializeAttributeComponent() : base("Deserialize Attribute", "DAt", "Deserialize text to an attribute") { }

    protected override Bitmap Icon => Resources.attribute_deserialize_24x24;
    protected override Guid GetComponentGuid() => new("8F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelNickname() => "At";
    protected override string GetModelDescription() => "The deserialized attribute";
}

#endregion Attribute

#region Quality

public class QualityKindGoo : EnumGoo<QualityKind>
{
    public QualityKindGoo() { }
    public QualityKindGoo(QualityKind value) : base(value) { }
}

public class QualityKindParam : EnumParam<QualityKindGoo, QualityKind>
{
    public QualityKindParam() : base("QualityKind", "QK", "The kind of quality indicating its scope and applicability.") { }
    protected override Guid GetComponentGuid() => new("9F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override Bitmap GetEnumIcon() => Resources.quality_24x24; // Use quality icon as fallback
}

public class QualityIdGoo : ModelGoo<QualityId>
{
    public QualityIdGoo() { }
    public QualityIdGoo(QualityId value) : base(value) { }
    protected override string GetModelDescription() => "A quality id is a key for a quality.";
}

public class QualityIdParam : ModelParam<QualityIdGoo, QualityId>
{
    public QualityIdParam() : base("QualityId", "QI", "A quality id is a key for a quality.") { }
    protected override Bitmap GetParamIcon() => Resources.quality_id_24x24;
    public override Guid ComponentGuid => new("AF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class QualityIdComponent : IdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public QualityIdComponent() : base("Quality ID", "QI", "Create a quality ID") { }

    protected override Bitmap GetComponentIcon() => Resources.quality_id_24x24;
    protected override Guid GetComponentGuid() => new("BF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "Quality key", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityIdParam(), "QualityId", "QI", "The created quality ID", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var key = "";
        if (!DA.GetData(0, ref key)) return;

        var qualityId = new QualityId { Key = key };
        var goo = new QualityIdGoo(qualityId);
        DA.SetData(0, goo);
    }
}

#endregion Quality

#region Benchmark

public class BenchmarkGoo : ModelGoo<Benchmark>
{
    public BenchmarkGoo() { }
    public BenchmarkGoo(Benchmark value) : base(value) { }
    protected override string GetModelDescription() => "A benchmark is a value with an optional unit for a quality.";
}

public class BenchmarkParam : ModelParam<BenchmarkGoo, Benchmark>
{
    public BenchmarkParam() : base("Benchmark", "Bm", "A benchmark is a value with an optional unit for a quality.") { }
    protected override Bitmap GetParamIcon() => Resources.quality_24x24; // Use quality icon as fallback
    public override Guid ComponentGuid => new("CF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class BenchmarkComponent : ModelComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public BenchmarkComponent() : base("Benchmark", "Bm", "Create a benchmark") { }

    protected override Bitmap GetComponentIcon() => Resources.quality_modify_24x24;
    protected override Guid GetComponentGuid() => new("DF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Benchmark name", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "I", "Benchmark icon", GH_ParamAccess.item, "");
        pManager.AddNumberParameter("Min", "Mi", "Minimum value", GH_ParamAccess.item, 0.0);
        pManager.AddBooleanParameter("MinExcluded", "MiX", "Minimum excluded", GH_ParamAccess.item, false);
        pManager.AddNumberParameter("Max", "Ma", "Maximum value", GH_ParamAccess.item, 100.0);
        pManager.AddBooleanParameter("MaxExcluded", "MaX", "Maximum excluded", GH_ParamAccess.item, false);
        pManager.AddParameter(new AttributeParam(), "Attributes", "A", "Benchmark attributes", GH_ParamAccess.list);
        pManager[6].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new BenchmarkParam(), "Benchmark", "Bm", "The created benchmark", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var name = "";
        var icon = "";
        var min = 0.0;
        var minExcluded = false;
        var max = 100.0;
        var maxExcluded = false;
        var attributeGoos = new List<AttributeGoo>();

        if (!DA.GetData(0, ref name)) return;
        DA.GetData(1, ref icon);
        DA.GetData(2, ref min);
        DA.GetData(3, ref minExcluded);
        DA.GetData(4, ref max);
        DA.GetData(5, ref maxExcluded);
        DA.GetDataList(6, attributeGoos);

        var attributes = attributeGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();

        var benchmark = new Benchmark
        {
            Name = name,
            Icon = icon,
            Min = (float)min,
            MinExcluded = minExcluded,
            Max = (float)max,
            MaxExcluded = maxExcluded,
            Attributes = attributes
        };
        var goo = new BenchmarkGoo(benchmark);
        DA.SetData(0, goo);
    }
}

#endregion Benchmark

#region Point

public class PointGoo : ModelGoo<Point>
{
    public PointGoo() { }
    public PointGoo(Point value) : base(value) { }
    protected override string GetModelDescription() => "A 3-point (xyz) of floating point numbers.";

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q) == typeof(Point3d) && Value is not null)
        {
            target = (Q)(object)Value.ToRhino();
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source is Point3d point3d)
        {
            Value = point3d.ToSemio();
            return true;
        }
        return false;
    }
}

public class PointParam : ModelParam<PointGoo, Point>
{
    public PointParam() : base("Point", "Pt", "A 3-point (xyz) of floating point numbers.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("EF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class PointComponent : ModelComponent<PointParam, PointGoo, Point>
{
    public PointComponent() : base("Point", "Pt", "Create a 3D point") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("FF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("X", "X", "X coordinate", GH_ParamAccess.item, 0.0);
        pManager.AddNumberParameter("Y", "Y", "Y coordinate", GH_ParamAccess.item, 0.0);
        pManager.AddNumberParameter("Z", "Z", "Z coordinate", GH_ParamAccess.item, 0.0);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PointParam(), "Point", "Pt", "The created point", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var x = 0.0;
        var y = 0.0;
        var z = 0.0;

        DA.GetData(0, ref x);
        DA.GetData(1, ref y);
        DA.GetData(2, ref z);

        var point = new Point { X = (float)x, Y = (float)y, Z = (float)z };
        var goo = new PointGoo(point);
        DA.SetData(0, goo);
    }
}

#endregion Point

#region Vector

public class VectorGoo : ModelGoo<Vector>
{
    public VectorGoo() { }
    public VectorGoo(Vector value) : base(value) { }
    protected override string GetModelDescription() => "A 3d-vector (xyz) of floating point numbers.";

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q) == typeof(Vector3d) && Value is not null)
        {
            target = (Q)(object)Value.ToRhino();
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source is Vector3d vector3d)
        {
            Value = vector3d.ToSemio();
            return true;
        }
        return false;
    }
}

public class VectorParam : ModelParam<VectorGoo, Vector>
{
    public VectorParam() : base("Vector", "Vc", "A 3d-vector (xyz) of floating point numbers.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("0F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class VectorComponent : ModelComponent<VectorParam, VectorGoo, Vector>
{
    public VectorComponent() : base("Vector", "Vc", "Create a 3D vector") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("1F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("X", "X", "X component", GH_ParamAccess.item, 0.0);
        pManager.AddNumberParameter("Y", "Y", "Y component", GH_ParamAccess.item, 0.0);
        pManager.AddNumberParameter("Z", "Z", "Z component", GH_ParamAccess.item, 1.0);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new VectorParam(), "Vector", "Vc", "The created vector", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var x = 0.0;
        var y = 0.0;
        var z = 1.0;

        DA.GetData(0, ref x);
        DA.GetData(1, ref y);
        DA.GetData(2, ref z);

        var vector = new Vector { X = (float)x, Y = (float)y, Z = (float)z };
        var goo = new VectorGoo(vector);
        DA.SetData(0, goo);
    }
}

#endregion Vector

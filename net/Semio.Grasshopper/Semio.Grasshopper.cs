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
    public override string TypeName => GetModelTypeName();
    public override string TypeDescription => GetModelDescription();

    protected abstract string GetModelDescription();
    protected abstract string GetModelTypeName();

    public override IGH_Goo Duplicate()
    {
        var duplicate = CreateDuplicate();
        if (Value is not null)
            duplicate.Value = Value.DeepClone();
        return duplicate;
    }

    protected abstract ModelGoo<TModel> CreateDuplicate();

    public override string ToString() => Value?.ToString() ?? string.Empty;

    public override bool Write(GH_IWriter writer)
    {
        if (Value is not null)
            writer.SetString(GetSerializationKey(), Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        var serialized = reader.GetString(GetSerializationKey());
        if (!string.IsNullOrEmpty(serialized))
        {
            var deserialized = serialized.Deserialize<TModel>();
            if (deserialized is not null)
                Value = deserialized;
        }
        return base.Read(reader);
    }

    protected abstract string GetSerializationKey();

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
    public override string TypeName => GetEnumTypeName();
    public override string TypeDescription => GetEnumTypeDescription();

    protected abstract string GetEnumTypeName();
    protected abstract string GetEnumTypeDescription();
    protected abstract EnumGoo<TEnum> CreateEnumDuplicate();
    protected abstract string GetEnumSerializationKey();

    public override IGH_Goo Duplicate()
    {
        var duplicate = CreateEnumDuplicate();
        duplicate.Value = Value;
        return duplicate;
    }

    public override string ToString() => Value.ToString();

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString(GetEnumSerializationKey(), Value.ToString());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        var enumString = reader.GetString(GetEnumSerializationKey());
        if (Enum.TryParse<TEnum>(enumString, out var enumValue))
            Value = enumValue;
        return base.Read(reader);
    }

    public override bool CastTo<Q>(ref Q target)
    {
        return CustomEnumCastTo(ref target);
    }

    protected virtual bool CustomEnumCastTo<Q>(ref Q target)
    {
        if (target is string)
        {
            target = (Q)(object)Value.ToString();
            return true;
        }
        if (target is int)
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
        return CustomEnumCastFrom(source);
    }

    protected virtual bool CustomEnumCastFrom(object source)
    {
        if (source is int intValue)
        {
            try
            {
                Value = (TEnum)(object)intValue;
                return true;
            }
            catch
            {
                return false;
            }
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
    protected override string GetModelTypeName() => "AttributeId";
    protected override ModelGoo<AttributeId> CreateDuplicate() => new AttributeIdGoo();
    protected override string GetSerializationKey() => "AttributeId";
}

public class AttributeIdParam : ModelParam<AttributeIdGoo, AttributeId>
{
    public AttributeIdParam() : base("AttributeId", "AI", "The ID of the attribute.") { }
    protected override Bitmap GetParamIcon() => Resources.attribute_id_24x24;
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B95");
}

public class AttributeIdComponent : IdComponent<AttributeIdParam, AttributeIdGoo, AttributeId>
{
    public AttributeIdComponent() : base("Attribute ID", "AI", "Create an attribute ID") { }

    protected override Bitmap GetComponentIcon() => Resources.attribute_id_24x24;
    protected override Guid GetComponentGuid() => new("431125C0-B98C-4122-9598-F72714AC9B96");

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
    protected override string GetModelTypeName() => "AttributeDiff";
    protected override ModelGoo<AttributeDiff> CreateDuplicate() => new AttributeDiffGoo();
    protected override string GetSerializationKey() => "AttributeDiff";
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
    protected override string GetModelTypeName() => "Attribute";
    protected override ModelGoo<Attribute> CreateDuplicate() => new AttributeGoo();
    protected override string GetSerializationKey() => "Attribute";
}

public class AttributeParam : ModelParam<AttributeGoo, Attribute>
{
    public AttributeParam() : base("Attribute", "At", "A attribute is a key value pair with an optional definition.") { }
    protected override Bitmap GetParamIcon() => Resources.attribute_24x24;
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B94");
}

public class AttributeComponent : ModelComponent<AttributeParam, AttributeGoo, Attribute>
{
    public AttributeComponent() : base("Attribute", "At", "Create an attribute") { }

    protected override Bitmap GetComponentIcon() => Resources.attribute_modify_24x24;
    protected override Guid GetComponentGuid() => new("51146B05-ACEB-4810-AD75-10AC3E029D39");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new AttributeParam(), "Attribute", "At?", "The optional attribute to deconstruct or modify.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Validate", "Vd?", "Whether the attribute should be validated.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke", "The key of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl?", "The optional value [ text | url ] of the attribute. No value is equivalent to true.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition [ text | uri ] of the attribute.", GH_ParamAccess.item);
        
        pManager[0].Optional = true;
        pManager[1].Optional = true;
        pManager[2].Optional = true;
        pManager[3].Optional = true;
        pManager[4].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeParam(), "Attribute", "At", "The constructed or modified attribute.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Valid", "Vd?", "True if the attribute is valid. Null if no validation was performed.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke", "The key of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl?", "The optional value [ text | url ] of the attribute. No value is equivalent to true.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition [ text | uri ] of the attribute.", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var attributeGoo = new AttributeGoo(new Attribute());
        var validate = false;
        var key = "";
        var value = "";
        var definition = "";

        if (DA.GetData(0, ref attributeGoo))
            attributeGoo = (AttributeGoo)attributeGoo.Duplicate();
        DA.GetData(1, ref validate);
        if (DA.GetData(2, ref key)) attributeGoo.Value.Key = key;
        if (DA.GetData(3, ref value)) attributeGoo.Value.Value = value;
        if (DA.GetData(4, ref definition)) attributeGoo.Value.Definition = definition;

        if (validate)
        {
            var (isValid, errors) = attributeGoo.Value.Validate();
            foreach (var error in errors)
                AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, error);
            DA.SetData(1, isValid);
        }

        DA.SetData(0, attributeGoo.Duplicate());
        DA.SetData(2, attributeGoo.Value.Key);
        DA.SetData(3, attributeGoo.Value.Value);
        DA.SetData(4, attributeGoo.Value.Definition);
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
    protected override string GetEnumTypeName() => "QualityKind";
    protected override string GetEnumTypeDescription() => "QualityKind";
    protected override EnumGoo<QualityKind> CreateEnumDuplicate() => new QualityKindGoo();
    protected override string GetEnumSerializationKey() => "QualityKind";
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
    protected override string GetModelTypeName() => "QualityId";
    protected override ModelGoo<QualityId> CreateDuplicate() => new QualityIdGoo();
    protected override string GetSerializationKey() => "QualityId";
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

public class QualityDiffGoo : ModelGoo<QualityDiff>
{
    public QualityDiffGoo() { }
    public QualityDiffGoo(QualityDiff value) : base(value) { }
    protected override string GetModelDescription() => "A diff for qualities.";
    protected override string GetModelTypeName() => "QualityDiff";
    protected override ModelGoo<QualityDiff> CreateDuplicate() => new QualityDiffGoo();
    protected override string GetSerializationKey() => "QualityDiff";
}

public class QualityDiffParam : ModelParam<QualityDiffGoo, QualityDiff>
{
    public QualityDiffParam() : base("QualityDiff", "QD", "A diff for qualities.") { }
    protected override Bitmap GetParamIcon() => Resources.quality_diff_24x24;
    public override Guid ComponentGuid => new("BF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class QualityDiffComponent : DiffComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public QualityDiffComponent() : base("Quality Diff", "QD", "Create a quality diff") { }

    protected override Bitmap GetComponentIcon() => Resources.quality_diff_24x24;
    protected override Guid GetComponentGuid() => new("CF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "Quality key", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "N", "Quality name", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Description", "D", "Quality description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Uri", "U", "Quality URI", GH_ParamAccess.item, "");
        pManager.AddBooleanParameter("Scalable", "S", "Is scalable", GH_ParamAccess.item, false);
        pManager.AddParameter(new QualityKindParam(), "Kind", "Ki", "Quality kind", GH_ParamAccess.item);
        pManager.AddTextParameter("SI", "SI", "SI unit", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Imperial", "I", "Imperial unit", GH_ParamAccess.item, "");
        pManager.AddNumberParameter("Min", "Mi", "Minimum value", GH_ParamAccess.item, 0.0);
        pManager.AddBooleanParameter("MinExcluded", "MiX", "Minimum excluded", GH_ParamAccess.item, true);
        pManager.AddNumberParameter("Max", "Ma", "Maximum value", GH_ParamAccess.item, 0.0);
        pManager.AddBooleanParameter("MaxExcluded", "MaX", "Maximum excluded", GH_ParamAccess.item, true);
        pManager.AddNumberParameter("Default", "De", "Default value", GH_ParamAccess.item, 0.0);
        pManager.AddTextParameter("Formula", "F", "Formula", GH_ParamAccess.item, "");
        pManager.AddParameter(new BenchmarkParam(), "Benchmarks", "B", "Benchmarks", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "A", "Attributes", GH_ParamAccess.list);
        pManager[5].Optional = true;
        pManager[14].Optional = true;
        pManager[15].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityDiffParam(), "QualityDiff", "QD", "The created quality diff", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var key = "";
        var name = "";
        var description = "";
        var uri = "";
        var scalable = false;
        var kindGoo = new QualityKindGoo(QualityKind.General);
        var si = "";
        var imperial = "";
        var min = 0.0;
        var minExcluded = true;
        var max = 0.0;
        var maxExcluded = true;
        var defaultValue = 0.0;
        var formula = "";
        var benchmarkGoos = new List<BenchmarkGoo>();
        var attributeGoos = new List<AttributeGoo>();

        if (!DA.GetData(0, ref key)) return;
        DA.GetData(1, ref name);
        DA.GetData(2, ref description);
        DA.GetData(3, ref uri);
        DA.GetData(4, ref scalable);
        DA.GetData(5, ref kindGoo);
        DA.GetData(6, ref si);
        DA.GetData(7, ref imperial);
        DA.GetData(8, ref min);
        DA.GetData(9, ref minExcluded);
        DA.GetData(10, ref max);
        DA.GetData(11, ref maxExcluded);
        DA.GetData(12, ref defaultValue);
        DA.GetData(13, ref formula);
        DA.GetDataList(14, benchmarkGoos);
        DA.GetDataList(15, attributeGoos);

        var benchmarks = benchmarkGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();
        var attributes = attributeGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();

        var qualityDiff = new QualityDiff
        {
            Key = key,
            Name = name,
            Description = description,
            Uri = uri,
            Scalable = scalable,
            Kind = kindGoo?.Value ?? QualityKind.General,
            SI = si,
            Imperial = imperial,
            Min = (float)min,
            MinExcluded = minExcluded,
            Max = (float)max,
            MaxExcluded = maxExcluded,
            Default = (float)defaultValue,
            Formula = formula,
            Benchmarks = benchmarks,
            Attributes = attributes
        };
        var goo = new QualityDiffGoo(qualityDiff);
        DA.SetData(0, goo);
    }
}

public class QualityGoo : ModelGoo<Quality>
{
    public QualityGoo() { }
    public QualityGoo(Quality value) : base(value) { }
    protected override string GetModelDescription() => "A quality is a named property that can be measured or assessed.";
    protected override string GetModelTypeName() => "Quality";
    protected override ModelGoo<Quality> CreateDuplicate() => new QualityGoo();
    protected override string GetSerializationKey() => "Quality";
}

public class QualityParam : ModelParam<QualityGoo, Quality>
{
    public QualityParam() : base("Quality", "Q", "A quality is a named property that can be measured or assessed.") { }
    protected override Bitmap GetParamIcon() => Resources.quality_24x24;
    public override Guid ComponentGuid => new("DF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class QualityComponent : ModelComponent<QualityParam, QualityGoo, Quality>
{
    public QualityComponent() : base("Quality", "Q", "Create a quality") { }

    protected override Bitmap GetComponentIcon() => Resources.quality_modify_24x24;
    protected override Guid GetComponentGuid() => new("EF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "Quality key", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "N", "Quality name", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Description", "D", "Quality description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Uri", "U", "Quality URI", GH_ParamAccess.item, "");
        pManager.AddBooleanParameter("Scalable", "S", "Is scalable", GH_ParamAccess.item, false);
        pManager.AddParameter(new QualityKindParam(), "Kind", "Ki", "Quality kind", GH_ParamAccess.item);
        pManager.AddTextParameter("SI", "SI", "SI unit", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Imperial", "I", "Imperial unit", GH_ParamAccess.item, "");
        pManager.AddNumberParameter("Min", "Mi", "Minimum value", GH_ParamAccess.item, 0.0);
        pManager.AddBooleanParameter("MinExcluded", "MiX", "Minimum excluded", GH_ParamAccess.item, true);
        pManager.AddNumberParameter("Max", "Ma", "Maximum value", GH_ParamAccess.item, 0.0);
        pManager.AddBooleanParameter("MaxExcluded", "MaX", "Maximum excluded", GH_ParamAccess.item, true);
        pManager.AddNumberParameter("Default", "De", "Default value", GH_ParamAccess.item, 0.0);
        pManager.AddTextParameter("Formula", "F", "Formula", GH_ParamAccess.item, "");
        pManager.AddParameter(new BenchmarkParam(), "Benchmarks", "B", "Benchmarks", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "A", "Attributes", GH_ParamAccess.list);
        pManager[5].Optional = true;
        pManager[14].Optional = true;
        pManager[15].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Q", "The created quality", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var key = "";
        var name = "";
        var description = "";
        var uri = "";
        var scalable = false;
        var kindGoo = new QualityKindGoo(QualityKind.General);
        var si = "";
        var imperial = "";
        var min = 0.0;
        var minExcluded = true;
        var max = 0.0;
        var maxExcluded = true;
        var defaultValue = 0.0;
        var formula = "";
        var benchmarkGoos = new List<BenchmarkGoo>();
        var attributeGoos = new List<AttributeGoo>();

        if (!DA.GetData(0, ref key)) return;
        DA.GetData(1, ref name);
        DA.GetData(2, ref description);
        DA.GetData(3, ref uri);
        DA.GetData(4, ref scalable);
        DA.GetData(5, ref kindGoo);
        DA.GetData(6, ref si);
        DA.GetData(7, ref imperial);
        DA.GetData(8, ref min);
        DA.GetData(9, ref minExcluded);
        DA.GetData(10, ref max);
        DA.GetData(11, ref maxExcluded);
        DA.GetData(12, ref defaultValue);
        DA.GetData(13, ref formula);
        DA.GetDataList(14, benchmarkGoos);
        DA.GetDataList(15, attributeGoos);

        var benchmarks = benchmarkGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();
        var attributes = attributeGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();

        var quality = new Quality
        {
            Key = key,
            Name = name,
            Description = description,
            Uri = uri,
            Scalable = scalable,
            Kind = kindGoo?.Value ?? QualityKind.General,
            SI = si,
            Imperial = imperial,
            Min = (float)min,
            MinExcluded = minExcluded,
            Max = (float)max,
            MaxExcluded = maxExcluded,
            Default = (float)defaultValue,
            Formula = formula,
            Benchmarks = benchmarks,
            Attributes = attributes
        };
        var goo = new QualityGoo(quality);
        DA.SetData(0, goo);
    }
}

public class SerializeQualityComponent : SerializeComponent<QualityParam, QualityGoo, Quality>
{
    public SerializeQualityComponent() : base("Serialize Quality", "SQ", "Serialize a quality to text") { }

    protected override Bitmap Icon => Resources.quality_serialize_24x24;
    protected override Guid GetComponentGuid() => new("FF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelNickname() => "Q";
    protected override string GetModelDescription() => "The quality to serialize";
}

public class DeserializeQualityComponent : DeserializeComponent<QualityParam, QualityGoo, Quality>
{
    public DeserializeQualityComponent() : base("Deserialize Quality", "DQ", "Deserialize text to a quality") { }

    protected override Bitmap Icon => Resources.quality_deserialize_24x24;
    protected override Guid GetComponentGuid() => new("0F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelNickname() => "Q";
    protected override string GetModelDescription() => "The deserialized quality";
}

#endregion Quality

#region Benchmark

public class BenchmarkGoo : ModelGoo<Benchmark>
{
    public BenchmarkGoo() { }
    public BenchmarkGoo(Benchmark value) : base(value) { }
    protected override string GetModelDescription() => "A benchmark is a value with an optional unit for a quality.";
    protected override string GetModelTypeName() => "Benchmark";
    protected override ModelGoo<Benchmark> CreateDuplicate() => new BenchmarkGoo();
    protected override string GetSerializationKey() => "Benchmark";
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
    protected override string GetModelTypeName() => "Point";
    protected override ModelGoo<Point> CreateDuplicate() => new PointGoo();
    protected override string GetSerializationKey() => "Point";

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (target is Point3d && Value is not null)
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
    protected override string GetModelTypeName() => "Vector";
    protected override ModelGoo<Vector> CreateDuplicate() => new VectorGoo();
    protected override string GetSerializationKey() => "Vector";

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (target is Vector3d && Value is not null)
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

#region Prop

public class PropGoo : ModelGoo<Prop>
{
    public PropGoo() { }
    public PropGoo(Prop value) : base(value) { }
    protected override string GetModelDescription() => "A prop is a key-value pair with an optional unit and attributes.";
    protected override string GetModelTypeName() => "Prop";
    protected override ModelGoo<Prop> CreateDuplicate() => new PropGoo();
    protected override string GetSerializationKey() => "Prop";
}

public class PropParam : ModelParam<PropGoo, Prop>
{
    public PropParam() : base("Prop", "Pr", "A prop is a key-value pair with an optional unit and attributes.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("2F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class PropComponent : ModelComponent<PropParam, PropGoo, Prop>
{
    public PropComponent() : base("Prop", "Pr", "Create a prop") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("3F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "Prop key", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "V", "Prop value", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Unit", "U", "Prop unit", GH_ParamAccess.item, "");
        pManager.AddParameter(new AttributeParam(), "Attributes", "A", "Prop attributes", GH_ParamAccess.list);
        pManager[3].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PropParam(), "Prop", "Pr", "The created prop", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var key = "";
        var value = "";
        var unit = "";
        var attributeGoos = new List<AttributeGoo>();

        if (!DA.GetData(0, ref key)) return;
        DA.GetData(1, ref value);
        DA.GetData(2, ref unit);
        DA.GetDataList(3, attributeGoos);

        var attributes = attributeGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();

        var prop = new Prop
        {
            Key = key,
            Value = value,
            Unit = unit,
            Attributes = attributes
        };
        var goo = new PropGoo(prop);
        DA.SetData(0, goo);
    }
}

public class SerializePropComponent : SerializeComponent<PropParam, PropGoo, Prop>
{
    public SerializePropComponent() : base("Serialize Prop", "SPr", "Serialize a prop to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("4F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Prop";
    protected override string GetModelNickname() => "Pr";
    protected override string GetModelDescription() => "The prop to serialize";
}

public class DeserializePropComponent : DeserializeComponent<PropParam, PropGoo, Prop>
{
    public DeserializePropComponent() : base("Deserialize Prop", "DPr", "Deserialize text to a prop") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("5F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Prop";
    protected override string GetModelNickname() => "Pr";
    protected override string GetModelDescription() => "The deserialized prop";
}

#endregion Prop

#region Stat

public class StatGoo : ModelGoo<Stat>
{
    public StatGoo() { }
    public StatGoo(Stat value) : base(value) { }
    protected override string GetModelDescription() => "A stat is a measurement with key, unit and min/max bounds.";
    protected override string GetModelTypeName() => "Stat";
    protected override ModelGoo<Stat> CreateDuplicate() => new StatGoo();
    protected override string GetSerializationKey() => "Stat";
}

public class StatParam : ModelParam<StatGoo, Stat>
{
    public StatParam() : base("Stat", "St", "A stat is a measurement with key, unit and min/max bounds.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("6F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class StatComponent : ModelComponent<StatParam, StatGoo, Stat>
{
    public StatComponent() : base("Stat", "St", "Create a stat") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("7F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Key", "K", "Stat key", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "U", "Stat unit", GH_ParamAccess.item, "");
        pManager.AddNumberParameter("Min", "Mi", "Minimum value", GH_ParamAccess.item, 0.0);
        pManager.AddBooleanParameter("MinExcluded", "MiX", "Minimum excluded", GH_ParamAccess.item, false);
        pManager.AddNumberParameter("Max", "Ma", "Maximum value", GH_ParamAccess.item, 0.0);
        pManager.AddBooleanParameter("MaxExcluded", "MaX", "Maximum excluded", GH_ParamAccess.item, false);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new StatParam(), "Stat", "St", "The created stat", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var key = "";
        var unit = "";
        var min = 0.0;
        var minExcluded = false;
        var max = 0.0;
        var maxExcluded = false;

        if (!DA.GetData(0, ref key)) return;
        DA.GetData(1, ref unit);
        DA.GetData(2, ref min);
        DA.GetData(3, ref minExcluded);
        DA.GetData(4, ref max);
        DA.GetData(5, ref maxExcluded);

        var stat = new Stat
        {
            Key = key,
            Unit = unit,
            Min = (float)min,
            MinExcluded = minExcluded,
            Max = (float)max,
            MaxExcluded = maxExcluded
        };
        var goo = new StatGoo(stat);
        DA.SetData(0, goo);
    }
}

public class SerializeStatComponent : SerializeComponent<StatParam, StatGoo, Stat>
{
    public SerializeStatComponent() : base("Serialize Stat", "SSt", "Serialize a stat to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("8F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Stat";
    protected override string GetModelNickname() => "St";
    protected override string GetModelDescription() => "The stat to serialize";
}

public class DeserializeStatComponent : DeserializeComponent<StatParam, StatGoo, Stat>
{
    public DeserializeStatComponent() : base("Deserialize Stat", "DSt", "Deserialize text to a stat") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("9F8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Stat";
    protected override string GetModelNickname() => "St";
    protected override string GetModelDescription() => "The deserialized stat";
}

#endregion Stat

#region Layer

public class LayerGoo : ModelGoo<Layer>
{
    public LayerGoo() { }
    public LayerGoo(Layer value) : base(value) { }
    protected override string GetModelDescription() => "A layer is a visual grouping with name, description and color.";
    protected override string GetModelTypeName() => "Layer";
    protected override ModelGoo<Layer> CreateDuplicate() => new LayerGoo();
    protected override string GetSerializationKey() => "Layer";
}

public class LayerParam : ModelParam<LayerGoo, Layer>
{
    public LayerParam() : base("Layer", "Ly", "A layer is a visual grouping with name, description and color.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("AF8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class LayerComponent : ModelComponent<LayerParam, LayerGoo, Layer>
{
    public LayerComponent() : base("Layer", "Ly", "Create a layer") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("BF8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Layer name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "Layer description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Color", "C", "Layer color", GH_ParamAccess.item, "");
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new LayerParam(), "Layer", "Ly", "The created layer", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var name = "";
        var description = "";
        var color = "";

        if (!DA.GetData(0, ref name)) return;
        DA.GetData(1, ref description);
        DA.GetData(2, ref color);

        var layer = new Layer
        {
            Name = name,
            Description = description,
            Color = color
        };
        var goo = new LayerGoo(layer);
        DA.SetData(0, goo);
    }
}

public class SerializeLayerComponent : SerializeComponent<LayerParam, LayerGoo, Layer>
{
    public SerializeLayerComponent() : base("Serialize Layer", "SLy", "Serialize a layer to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("CF8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Layer";
    protected override string GetModelNickname() => "Ly";
    protected override string GetModelDescription() => "The layer to serialize";
}

public class DeserializeLayerComponent : DeserializeComponent<LayerParam, LayerGoo, Layer>
{
    public DeserializeLayerComponent() : base("Deserialize Layer", "DLy", "Deserialize text to a layer") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("DF8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Layer";
    protected override string GetModelNickname() => "Ly";
    protected override string GetModelDescription() => "The deserialized layer";
}

#endregion Layer

#region Group

public class GroupGoo : ModelGoo<Group>
{
    public GroupGoo() { }
    public GroupGoo(Group value) : base(value) { }
    protected override string GetModelDescription() => "A group is a collection of pieces with name, description and color.";
    protected override string GetModelTypeName() => "Group";
    protected override ModelGoo<Group> CreateDuplicate() => new GroupGoo();
    protected override string GetSerializationKey() => "Group";
}

public class GroupParam : ModelParam<GroupGoo, Group>
{
    public GroupParam() : base("Group", "Gr", "A group is a collection of pieces with name, description and color.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("EF8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class GroupComponent : ModelComponent<GroupParam, GroupGoo, Group>
{
    public GroupComponent() : base("Group", "Gr", "Create a group") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("FF8B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Group name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "Group description", GH_ParamAccess.item, "");
        pManager.AddParameter(new PieceIdParam(), "Pieces", "P", "Piece IDs in group", GH_ParamAccess.list);
        pManager.AddTextParameter("Color", "C", "Group color", GH_ParamAccess.item, "");
        pManager[2].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new GroupParam(), "Group", "Gr", "The created group", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var name = "";
        var description = "";
        var pieceIdGoos = new List<PieceIdGoo>();
        var color = "";

        if (!DA.GetData(0, ref name)) return;
        DA.GetData(1, ref description);
        DA.GetDataList(2, pieceIdGoos);
        DA.GetData(3, ref color);

        var pieces = pieceIdGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();

        var group = new Group
        {
            Name = name,
            Description = description,
            Pieces = pieces,
            Color = color
        };
        var goo = new GroupGoo(group);
        DA.SetData(0, goo);
    }
}

public class SerializeGroupComponent : SerializeComponent<GroupParam, GroupGoo, Group>
{
    public SerializeGroupComponent() : base("Serialize Group", "SGr", "Serialize a group to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("0F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Group";
    protected override string GetModelNickname() => "Gr";
    protected override string GetModelDescription() => "The group to serialize";
}

public class DeserializeGroupComponent : DeserializeComponent<GroupParam, GroupGoo, Group>
{
    public DeserializeGroupComponent() : base("Deserialize Group", "DGr", "Deserialize text to a group") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("1F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Group";
    protected override string GetModelNickname() => "Gr";
    protected override string GetModelDescription() => "The deserialized group";
}

#endregion Group

#region PieceId

public class PieceIdGoo : ModelGoo<PieceId>
{
    public PieceIdGoo() { }
    public PieceIdGoo(PieceId value) : base(value) { }
    protected override string GetModelDescription() => "A piece ID identifies a specific piece in a design.";
    protected override string GetModelTypeName() => "PieceId";
    protected override ModelGoo<PieceId> CreateDuplicate() => new PieceIdGoo();
    protected override string GetSerializationKey() => "PieceId";
}

public class PieceIdParam : ModelParam<PieceIdGoo, PieceId>
{
    public PieceIdParam() : base("PieceId", "PI", "A piece ID identifies a specific piece in a design.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("2F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class PieceIdComponent : IdComponent<PieceIdParam, PieceIdGoo, PieceId>
{
    public PieceIdComponent() : base("Piece ID", "PI", "Create a piece ID") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("3F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "I", "Piece ID", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam(), "PieceId", "PI", "The created piece ID", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var id = "";
        if (!DA.GetData(0, ref id)) return;

        var pieceId = new PieceId { Id = id };
        var goo = new PieceIdGoo(pieceId);
        DA.SetData(0, goo);
    }
}

#endregion PieceId

#region DiagramPoint

public class DiagramPointGoo : ModelGoo<DiagramPoint>
{
    public DiagramPointGoo() { }
    public DiagramPointGoo(DiagramPoint value) : base(value) { }
    protected override string GetModelDescription() => "A 2D point (XY) for diagram layout.";
    protected override string GetModelTypeName() => "DiagramPoint";
    protected override ModelGoo<DiagramPoint> CreateDuplicate() => new DiagramPointGoo();
    protected override string GetSerializationKey() => "DiagramPoint";
}

public class DiagramPointParam : ModelParam<DiagramPointGoo, DiagramPoint>
{
    public DiagramPointParam() : base("DiagramPoint", "DP", "A 2D point (XY) for diagram layout.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("4F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class DiagramPointComponent : ModelComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public DiagramPointComponent() : base("Diagram Point", "DP", "Create a diagram point") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("5F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("X", "X", "X coordinate", GH_ParamAccess.item, 0.0);
        pManager.AddNumberParameter("Y", "Y", "Y coordinate", GH_ParamAccess.item, 0.0);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DiagramPointParam(), "DiagramPoint", "DP", "The created diagram point", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var x = 0.0;
        var y = 0.0;

        DA.GetData(0, ref x);
        DA.GetData(1, ref y);

        var diagramPoint = new DiagramPoint { X = (float)x, Y = (float)y };
        var goo = new DiagramPointGoo(diagramPoint);
        DA.SetData(0, goo);
    }
}

public class SerializeDiagramPointComponent : SerializeComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public SerializeDiagramPointComponent() : base("Serialize DiagramPoint", "SDP", "Serialize a diagram point to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("6F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "DiagramPoint";
    protected override string GetModelNickname() => "DP";
    protected override string GetModelDescription() => "The diagram point to serialize";
}

public class DeserializeDiagramPointComponent : DeserializeComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public DeserializeDiagramPointComponent() : base("Deserialize DiagramPoint", "DDP", "Deserialize text to a diagram point") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("7F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "DiagramPoint";
    protected override string GetModelNickname() => "DP";
    protected override string GetModelDescription() => "The deserialized diagram point";
}

#endregion DiagramPoint

#region Plane

public class PlaneGoo : ModelGoo<Plane>
{
    public PlaneGoo() { }
    public PlaneGoo(Plane value) : base(value) { }
    protected override string GetModelDescription() => "A plane defined by origin and XY axes.";
    protected override string GetModelTypeName() => "Plane";
    protected override ModelGoo<Plane> CreateDuplicate() => new PlaneGoo();
    protected override string GetSerializationKey() => "Plane";

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (target is Rhino.Geometry.Plane && Value is not null)
        {
            var rhinoPlane = new Rhino.Geometry.Plane(
                Value.Origin.ToRhino(),
                Value.XAxis.ToRhino(),
                Value.YAxis.ToRhino()
            );
            target = (Q)(object)rhinoPlane;
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source is Rhino.Geometry.Plane rhinoPlane)
        {
            Value = new Plane
            {
                Origin = rhinoPlane.Origin.ToSemio(),
                XAxis = rhinoPlane.XAxis.ToSemio(),
                YAxis = rhinoPlane.YAxis.ToSemio()
            };
            return true;
        }
        return false;
    }
}

public class PlaneParam : ModelParam<PlaneGoo, Plane>
{
    public PlaneParam() : base("Plane", "Pl", "A plane defined by origin and XY axes.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24; // Use generic icon as fallback
    public override Guid ComponentGuid => new("8F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class PlaneComponent : ModelComponent<PlaneParam, PlaneGoo, Plane>
{
    public PlaneComponent() : base("Plane", "Pl", "Create a plane") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("9F9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PointParam(), "Origin", "O", "Plane origin", GH_ParamAccess.item);
        pManager.AddParameter(new VectorParam(), "XAxis", "X", "X-axis vector", GH_ParamAccess.item);
        pManager.AddParameter(new VectorParam(), "YAxis", "Y", "Y-axis vector", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager[2].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PlaneParam(), "Plane", "Pl", "The created plane", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var originGoo = new PointGoo(new Point());
        var xAxisGoo = new VectorGoo(new Vector { X = 1 });
        var yAxisGoo = new VectorGoo(new Vector { Y = 1 });

        DA.GetData(0, ref originGoo);
        DA.GetData(1, ref xAxisGoo);
        DA.GetData(2, ref yAxisGoo);

        var plane = new Plane
        {
            Origin = originGoo?.Value ?? new Point(),
            XAxis = xAxisGoo?.Value ?? new Vector { X = 1 },
            YAxis = yAxisGoo?.Value ?? new Vector { Y = 1 }
        };
        var goo = new PlaneGoo(plane);
        DA.SetData(0, goo);
    }
}

public class SerializePlaneComponent : SerializeComponent<PlaneParam, PlaneGoo, Plane>
{
    public SerializePlaneComponent() : base("Serialize Plane", "SPl", "Serialize a plane to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AF9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Plane";
    protected override string GetModelNickname() => "Pl";
    protected override string GetModelDescription() => "The plane to serialize";
}

public class DeserializePlaneComponent : DeserializeComponent<PlaneParam, PlaneGoo, Plane>
{
    public DeserializePlaneComponent() : base("Deserialize Plane", "DPl", "Deserialize text to a plane") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("BF9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Plane";
    protected override string GetModelNickname() => "Pl";
    protected override string GetModelDescription() => "The deserialized plane";
}

#endregion Plane

#region RepresentationId

public class RepresentationIdGoo : ModelGoo<RepresentationId>
{
    public RepresentationIdGoo() { }
    public RepresentationIdGoo(RepresentationId value) : base(value) { }
    protected override string GetModelDescription() => "A representation ID identifies a representation by its tags.";
    protected override string GetModelTypeName() => "RepresentationId";
    protected override ModelGoo<RepresentationId> CreateDuplicate() => new RepresentationIdGoo();
    protected override string GetSerializationKey() => "RepresentationId";
}

public class RepresentationIdParam : ModelParam<RepresentationIdGoo, RepresentationId>
{
    public RepresentationIdParam() : base("RepresentationId", "RI", "A representation ID identifies a representation by its tags.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("CF9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class RepresentationIdComponent : IdComponent<RepresentationIdParam, RepresentationIdGoo, RepresentationId>
{
    public RepresentationIdComponent() : base("Representation ID", "RI", "Create a representation ID") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("DF9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Tags", "T", "Representation tags", GH_ParamAccess.list);
        pManager[0].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationIdParam(), "RepresentationId", "RI", "The created representation ID", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var tags = new List<string>();
        DA.GetDataList(0, tags);

        var representationId = new RepresentationId { Tags = tags };
        var goo = new RepresentationIdGoo(representationId);
        DA.SetData(0, goo);
    }
}

#endregion RepresentationId

#region RepresentationDiff

public class RepresentationDiffGoo : ModelGoo<RepresentationDiff>
{
    public RepresentationDiffGoo() { }
    public RepresentationDiffGoo(RepresentationDiff value) : base(value) { }
    protected override string GetModelDescription() => "A representation diff for modifying representations.";
    protected override string GetModelTypeName() => "RepresentationDiff";
    protected override ModelGoo<RepresentationDiff> CreateDuplicate() => new RepresentationDiffGoo();
    protected override string GetSerializationKey() => "RepresentationDiff";
}

public class RepresentationDiffParam : ModelParam<RepresentationDiffGoo, RepresentationDiff>
{
    public RepresentationDiffParam() : base("RepresentationDiff", "RD", "A representation diff for modifying representations.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("EF9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class RepresentationDiffComponent : DiffComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public RepresentationDiffComponent() : base("Representation Diff", "RD", "Create a representation diff") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("FF9B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Url", "U", "Representation URL", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "Representation description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Tags", "T", "Representation tags", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "A", "Representation attributes", GH_ParamAccess.list);
        pManager[2].Optional = true;
        pManager[3].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationDiffParam(), "RepresentationDiff", "RD", "The created representation diff", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var url = "";
        var description = "";
        var tags = new List<string>();
        var attributeGoos = new List<AttributeGoo>();

        if (!DA.GetData(0, ref url)) return;
        DA.GetData(1, ref description);
        DA.GetDataList(2, tags);
        DA.GetDataList(3, attributeGoos);

        var attributes = attributeGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();

        var representationDiff = new RepresentationDiff
        {
            Url = url,
            Description = description,
            Tags = tags,
            Attributes = attributes
        };
        var goo = new RepresentationDiffGoo(representationDiff);
        DA.SetData(0, goo);
    }
}

#endregion RepresentationDiff

#region Representation

public class RepresentationGoo : ModelGoo<Representation>
{
    public RepresentationGoo() { }
    public RepresentationGoo(Representation value) : base(value) { }
    protected override string GetModelDescription() => "A representation is a tagged resource URL with description and attributes.";
    protected override string GetModelTypeName() => "Representation";
    protected override ModelGoo<Representation> CreateDuplicate() => new RepresentationGoo();
    protected override string GetSerializationKey() => "Representation";
}

public class RepresentationParam : ModelParam<RepresentationGoo, Representation>
{
    public RepresentationParam() : base("Representation", "Rp", "A representation is a tagged resource URL with description and attributes.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("0FAB9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class RepresentationComponent : ModelComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public RepresentationComponent() : base("Representation", "Rp", "Create a representation") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("1FAB9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Url", "U", "Representation URL", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "Representation description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Tags", "T", "Representation tags", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "A", "Representation attributes", GH_ParamAccess.list);
        pManager[2].Optional = true;
        pManager[3].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Rp", "The created representation", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var url = "";
        var description = "";
        var tags = new List<string>();
        var attributeGoos = new List<AttributeGoo>();

        if (!DA.GetData(0, ref url)) return;
        DA.GetData(1, ref description);
        DA.GetDataList(2, tags);
        DA.GetDataList(3, attributeGoos);

        var attributes = attributeGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();

        var representation = new Representation
        {
            Url = url,
            Description = description,
            Tags = tags,
            Attributes = attributes
        };
        var goo = new RepresentationGoo(representation);
        DA.SetData(0, goo);
    }
}

public class SerializeRepresentationComponent : SerializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public SerializeRepresentationComponent() : base("Serialize Representation", "SRp", "Serialize a representation to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("2FAB9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelNickname() => "Rp";
    protected override string GetModelDescription() => "The representation to serialize";
}

public class DeserializeRepresentationComponent : DeserializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public DeserializeRepresentationComponent() : base("Deserialize Representation", "DRp", "Deserialize text to a representation") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("3FAB9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelNickname() => "Rp";
    protected override string GetModelDescription() => "The deserialized representation";
}

#endregion Representation

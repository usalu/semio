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

public abstract class IdGoo<TModel> : ModelGoo<TModel> where TModel : Model<TModel>, new()
{
    public IdGoo() { }
    public IdGoo(TModel value) : base(value) { }
}

public abstract class IdParam<TGoo, TModel> : ModelParam<TGoo, TModel> 
    where TGoo : IdGoo<TModel> 
    where TModel : Model<TModel>, new()
{
    protected IdParam(string name, string nickname, string description)
        : base(name, nickname, description) { }
}

public abstract class DiffGoo<TModel> : ModelGoo<TModel> where TModel : Model<TModel>, new()
{
    public DiffGoo() { }
    public DiffGoo(TModel value) : base(value) { }
}

public abstract class DiffParam<TGoo, TModel> : ModelParam<TGoo, TModel> 
    where TGoo : DiffGoo<TModel> 
    where TModel : Model<TModel>, new()
{
    protected DiffParam(string name, string nickname, string description)
        : base(name, nickname, description) { }
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

public abstract class SerializeDiffComponent<TParam, TGoo, TModel> : SerializeComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new()
    where TGoo : DiffGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected SerializeDiffComponent(string name, string nickname, string description)
        : base(name, nickname, description) { }

    public override GH_Exposure Exposure => GH_Exposure.hidden;
}

public abstract class DeserializeDiffComponent<TParam, TGoo, TModel> : DeserializeComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new()
    where TGoo : DiffGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected DeserializeDiffComponent(string name, string nickname, string description)
        : base(name, nickname, description) { }

    public override GH_Exposure Exposure => GH_Exposure.hidden;
}

public abstract class SerializeIdComponent<TParam, TGoo, TModel> : SerializeComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new()
    where TGoo : IdGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected SerializeIdComponent(string name, string nickname, string description)
        : base(name, nickname, description) { }

    public override GH_Exposure Exposure => GH_Exposure.hidden;
}

public abstract class DeserializeIdComponent<TParam, TGoo, TModel> : DeserializeComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new()
    where TGoo : IdGoo<TModel>, new()
    where TModel : Model<TModel>, new()
{
    protected DeserializeIdComponent(string name, string nickname, string description)
        : base(name, nickname, description) { }

    public override GH_Exposure Exposure => GH_Exposure.hidden;
}

#endregion Bases

#region Attribute

public class AttributeIdGoo : IdGoo<AttributeId>
{
    public AttributeIdGoo() { }
    public AttributeIdGoo(AttributeId value) : base(value) { }
    protected override string GetModelDescription() => "The ID of the attribute.";
    protected override string GetModelTypeName() => "AttributeId";
    protected override IdGoo<AttributeId> CreateDuplicate() => new AttributeIdGoo();
    protected override string GetSerializationKey() => "AttributeId";
}

public class AttributeIdParam : IdParam<AttributeIdGoo, AttributeId>
{
    public AttributeIdParam() : base("AttributeId", "AI", "The ID of the attribute.") { }
    protected override Bitmap GetParamIcon() => Resources.attribute_id_24x24;
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B95");
}

public class AttributeIdComponent : IdComponent<AttributeIdParam, AttributeIdGoo, AttributeId>
{
    public AttributeIdComponent() : base("Attribute ID", "AI", "Create an attribute ID") { }

    protected override Bitmap GetComponentIcon() => Resources.attribute_id_24x24;
    protected override Guid GetComponentGuid() => new("431125C0-B98C-4122-9598-F72714AC9B92");

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

public class AttributeDiffGoo : DiffGoo<AttributeDiff>
{
    public AttributeDiffGoo() { }
    public AttributeDiffGoo(AttributeDiff value) : base(value) { }
    protected override string GetModelDescription() => "A diff for attributes.";
    protected override string GetModelTypeName() => "AttributeDiff";
    protected override ModelGoo<AttributeDiff> CreateDuplicate() => new AttributeDiffGoo();
    protected override string GetSerializationKey() => "AttributeDiff";
}

public class AttributeDiffParam : DiffParam<AttributeDiffGoo, AttributeDiff>
{
    public AttributeDiffParam() : base("AttributeDiff", "AD", "A diff for attributes.") { }
    protected override Bitmap GetParamIcon() => Resources.attribute_diff_24x24;
    public override Guid ComponentGuid => new("3F7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class AttributeDiffComponent : DiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public AttributeDiffComponent() : base("Attribute Diff", "AD", "Create an attribute diff") { }

    protected override Bitmap GetComponentIcon() => Resources.attribute_diff_24x24;
    protected override Guid GetComponentGuid() => new("431125C0-B98C-4122-9598-F72714AC9B96");

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

public class SerializeAttributeDiffComponent : SerializeDiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public SerializeAttributeDiffComponent() : base("Serialize AttributeDiff", "SAD", "Serialize an attribute diff to text") { }

    protected override Bitmap Icon => Resources.attribute_diff_serialize_24x24;
    protected override Guid GetComponentGuid() => new("431125C0-B98C-4122-9598-F72714AC9B97");
    protected override string GetModelTypeName() => "AttributeDiff";
    protected override string GetModelNickname() => "AD";
    protected override string GetModelDescription() => "The attribute diff to serialize";
}

public class DeserializeAttributeDiffComponent : DeserializeDiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public DeserializeAttributeDiffComponent() : base("Deserialize AttributeDiff", "DAD", "Deserialize text to an attribute diff") { }

    protected override Bitmap Icon => Resources.attribute_diff_deserialize_24x24;
    protected override Guid GetComponentGuid() => new("431125C0-B98C-4122-9598-F72714AC9B98");
    protected override string GetModelTypeName() => "AttributeDiff";
    protected override string GetModelNickname() => "AD";
    protected override string GetModelDescription() => "The deserialized attribute diff";
}

public class SerializeAttributeComponent : SerializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public SerializeAttributeComponent() : base("Serialize Attribute", "SAt", "Serialize an attribute to text") { }

    protected override Bitmap Icon => Resources.attribute_serialize_24x24;
    protected override Guid GetComponentGuid() => new("C651F24C-BFF8-4821-8974-8588BCA75250");
    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelNickname() => "At";
    protected override string GetModelDescription() => "The attribute to serialize";
}

public class DeserializeAttributeComponent : DeserializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public DeserializeAttributeComponent() : base("Deserialize Attribute", "DAt", "Deserialize text to an attribute") { }

    protected override Bitmap Icon => Resources.attribute_deserialize_24x24;
    protected override Guid GetComponentGuid() => new("C651F24C-BFF8-4821-8975-8588BCA75250");
    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelNickname() => "At";
    protected override string GetModelDescription() => "The deserialized attribute";
}

public class SerializeAttributeIdComponent : SerializeIdComponent<AttributeIdParam, AttributeIdGoo, AttributeId>
{
    public SerializeAttributeIdComponent() : base("Serialize AttributeId", "SAI", "Serialize an attribute id to text") { }

    protected override Bitmap Icon => Resources.attribute_id_serialize_24x24;
    protected override Guid GetComponentGuid() => new("431125C0-B98C-4122-9598-F72714AC9B93");
    protected override string GetModelTypeName() => "AttributeId";
    protected override string GetModelNickname() => "AI";
    protected override string GetModelDescription() => "The attribute id to serialize";
}

public class DeserializeAttributeIdComponent : DeserializeIdComponent<AttributeIdParam, AttributeIdGoo, AttributeId>
{
    public DeserializeAttributeIdComponent() : base("Deserialize AttributeId", "DAI", "Deserialize text to an attribute id") { }

    protected override Bitmap Icon => Resources.attribute_id_deserialize_24x24;
    protected override Guid GetComponentGuid() => new("431125C0-B98C-4122-9598-F72714AC9B99");
    protected override string GetModelTypeName() => "AttributeId";
    protected override string GetModelNickname() => "AI";
    protected override string GetModelDescription() => "The deserialized attribute id";
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

public class QualityIdGoo : IdGoo<QualityId>
{
    public QualityIdGoo() { }
    public QualityIdGoo(QualityId value) : base(value) { }
    protected override string GetModelDescription() => "A quality id is a key for a quality.";
    protected override string GetModelTypeName() => "QualityId";
    protected override IdGoo<QualityId> CreateDuplicate() => new QualityIdGoo();
    protected override string GetSerializationKey() => "QualityId";
}

public class QualityIdParam : IdParam<QualityIdGoo, QualityId>
{
    public QualityIdParam() : base("QualityId", "QI", "A quality id is a key for a quality.") { }
    protected override Bitmap GetParamIcon() => Resources.quality_id_24x24;
    public override Guid ComponentGuid => new("AF7B9A1D-2E3F-4C5D-8A9B-0E1F2D3C4B5A");
}

public class QualityIdComponent : IdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public QualityIdComponent() : base("Quality ID", "QI", "Create a quality ID") { }

    protected override Bitmap GetComponentIcon() => Resources.quality_id_24x24;
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");

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

public class QualityDiffGoo : DiffGoo<QualityDiff>
{
    public QualityDiffGoo() { }
    public QualityDiffGoo(QualityDiff value) : base(value) { }
    protected override string GetModelDescription() => "A diff for qualities.";
    protected override string GetModelTypeName() => "QualityDiff";
    protected override ModelGoo<QualityDiff> CreateDuplicate() => new QualityDiffGoo();
    protected override string GetSerializationKey() => "QualityDiff";
}

public class QualityDiffParam : DiffParam<QualityDiffGoo, QualityDiff>
{
    public QualityDiffParam() : base("QualityDiff", "QD", "A diff for qualities.") { }
    protected override Bitmap GetParamIcon() => Resources.quality_diff_24x24;
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DA");
}

public class QualityDiffComponent : DiffComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public QualityDiffComponent() : base("Quality Diff", "QD", "Create a quality diff") { }

    protected override Bitmap GetComponentIcon() => Resources.quality_diff_24x24;
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DB");

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
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class QualityComponent : ModelComponent<QualityParam, QualityGoo, Quality>
{
    public QualityComponent() : base("Quality", "Q", "Create a quality") { }

    protected override Bitmap GetComponentIcon() => Resources.quality_modify_24x24;
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");

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
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C8");
    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelNickname() => "Q";
    protected override string GetModelDescription() => "The quality to serialize";
}

public class DeserializeQualityComponent : DeserializeComponent<QualityParam, QualityGoo, Quality>
{
    public DeserializeQualityComponent() : base("Deserialize Quality", "DQ", "Deserialize text to a quality") { }

    protected override Bitmap Icon => Resources.quality_deserialize_24x24;
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C9");
    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelNickname() => "Q";
    protected override string GetModelDescription() => "The deserialized quality";
}

public class SerializeQualityIdComponent : SerializeComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public SerializeQualityIdComponent() : base("Serialize QualityId", "SQI", "Serialize a quality id to text") { }

    protected override Bitmap Icon => Resources.quality_id_serialize_24x24;
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CA");
    protected override string GetModelTypeName() => "QualityId";
    protected override string GetModelNickname() => "QI";
    protected override string GetModelDescription() => "The quality id to serialize";
}

public class DeserializeQualityIdComponent : DeserializeComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public DeserializeQualityIdComponent() : base("Deserialize QualityId", "DQI", "Deserialize text to a quality id") { }

    protected override Bitmap Icon => Resources.quality_id_deserialize_24x24;
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CB");
    protected override string GetModelTypeName() => "QualityId";
    protected override string GetModelNickname() => "QI";
    protected override string GetModelDescription() => "The deserialized quality id";
}

public class SerializeQualityDiffComponent : SerializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public SerializeQualityDiffComponent() : base("Serialize QualityDiff", "SQD", "Serialize a quality diff to text") { }

    protected override Bitmap Icon => Resources.quality_diff_serialize_24x24;
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DC");
    protected override string GetModelTypeName() => "QualityDiff";
    protected override string GetModelNickname() => "QD";
    protected override string GetModelDescription() => "The quality diff to serialize";
}

public class DeserializeQualityDiffComponent : DeserializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public DeserializeQualityDiffComponent() : base("Deserialize QualityDiff", "DQD", "Deserialize text to a quality diff") { }

    protected override Bitmap Icon => Resources.quality_diff_deserialize_24x24;
    protected override Guid GetComponentGuid() => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DD");
    protected override string GetModelTypeName() => "QualityDiff";
    protected override string GetModelNickname() => "QD";
    protected override string GetModelDescription() => "The deserialized quality diff";
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
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class BenchmarkComponent : ModelComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public BenchmarkComponent() : base("Benchmark", "Bm", "Create a benchmark") { }

    protected override Bitmap GetComponentIcon() => Resources.quality_modify_24x24;
    protected override Guid GetComponentGuid() => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");

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

public class PieceIdGoo : IdGoo<PieceId>
{
    public PieceIdGoo() { }
    public PieceIdGoo(PieceId value) : base(value) { }
    protected override string GetModelDescription() => "A piece ID identifies a specific piece in a design.";
    protected override string GetModelTypeName() => "PieceId";
    protected override IdGoo<PieceId> CreateDuplicate() => new PieceIdGoo();
    protected override string GetSerializationKey() => "PieceId";
}

public class PieceIdParam : IdParam<PieceIdGoo, PieceId>
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
    protected override Guid GetComponentGuid() => new("61FB9BBE-64DE-42B2-B7EF-69CD97FDD9E3");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DiagramPointParam(), "DiagramPoint", "DP?", "The optional diagram point to deconstruct or modify.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Validate", "Vd?", "Whether the diagram point should be validated.", GH_ParamAccess.item);
        pManager.AddNumberParameter("X", "X", "The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Y", "Y", "The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.", GH_ParamAccess.item);

        pManager[0].Optional = true;
        pManager[1].Optional = true;
        pManager[2].Optional = true;
        pManager[3].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DiagramPointParam(), "DiagramPoint", "DP", "The constructed or modified diagram point.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Valid", "Vd?", "True if the diagram point is valid. Null if no validation was performed.", GH_ParamAccess.item);
        pManager.AddNumberParameter("X", "X", "The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Y", "Y", "The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var diagramPointGoo = new DiagramPointGoo(new DiagramPoint());
        var validate = false;
        var x = 0.0;
        var y = 0.0;

        if (DA.GetData(0, ref diagramPointGoo))
            diagramPointGoo = (DiagramPointGoo)diagramPointGoo.Duplicate();
        DA.GetData(1, ref validate);
        if (DA.GetData(2, ref x)) diagramPointGoo.Value.X = (float)x;
        if (DA.GetData(3, ref y)) diagramPointGoo.Value.Y = (float)y;

        if (validate)
        {
            var (isValid, errors) = diagramPointGoo.Value.Validate();
            foreach (var error in errors)
                AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, error);
            DA.SetData(1, isValid);
        }

        DA.SetData(0, diagramPointGoo.Duplicate());
        DA.SetData(2, diagramPointGoo.Value.X);
        DA.SetData(3, diagramPointGoo.Value.Y);
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

public class RepresentationIdGoo : IdGoo<RepresentationId>
{
    public RepresentationIdGoo() { }
    public RepresentationIdGoo(RepresentationId value) : base(value) { }
    protected override string GetModelDescription() => "A representation ID identifies a representation by its tags.";
    protected override string GetModelTypeName() => "RepresentationId";
    protected override IdGoo<RepresentationId> CreateDuplicate() => new RepresentationIdGoo();
    protected override string GetSerializationKey() => "RepresentationId";
}

public class RepresentationIdParam : IdParam<RepresentationIdGoo, RepresentationId>
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

#region File

public class FileIdGoo : IdGoo<FileId>
{
    public FileIdGoo() { }
    public FileIdGoo(FileId value) : base(value) { }
    protected override string GetModelDescription() => "A file ID identifies a file by its path.";
    protected override string GetModelTypeName() => "FileId";
    protected override IdGoo<FileId> CreateDuplicate() => new FileIdGoo();
    protected override string GetSerializationKey() => "FileId";
}

public class FileIdParam : IdParam<FileIdGoo, FileId>
{
    public FileIdParam() : base("FileId", "FI", "A file ID identifies a file by its path.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E7");
}

public class FileIdComponent : IdComponent<FileIdParam, FileIdGoo, FileId>
{
    public FileIdComponent() : base("File ID", "FI", "Create a file ID") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E8");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Url", "U", "The file URL", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FileIdParam(), "FileId", "FI", "The file ID", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string url = string.Empty;
        if (!DA.GetData(0, ref url))
            return;

        var fileId = new FileId { Url = url };
        DA.SetData(0, new FileIdGoo(fileId));
    }
}

public class FileDiffGoo : DiffGoo<FileDiff>
{
    public FileDiffGoo() { }
    public FileDiffGoo(FileDiff value) : base(value) { }
    protected override string GetModelDescription() => "A file diff for modifying files.";
    protected override string GetModelTypeName() => "FileDiff";
    protected override DiffGoo<FileDiff> CreateDuplicate() => new FileDiffGoo();
    protected override string GetSerializationKey() => "FileDiff";
}

public class FileDiffParam : DiffParam<FileDiffGoo, FileDiff>
{
    public FileDiffParam() : base("FileDiff", "FD", "A file diff for modifying files.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F0");
}

public class FileDiffComponent : DiffComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public FileDiffComponent() : base("File Diff", "FD", "Create a file diff") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F1");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Url", "U", "The file URL", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Data", "D", "The file data", GH_ParamAccess.item, string.Empty);
        pManager.AddIntegerParameter("Size", "S", "The file size", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Hash", "H", "The file hash", GH_ParamAccess.item, string.Empty);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FileDiffParam(), "FileDiff", "FD", "The file diff", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string url = string.Empty;
        string data = string.Empty;
        int size = 0;
        string hash = string.Empty;

        if (!DA.GetData(0, ref url))
            return;

        DA.GetData(1, ref data);
        bool hasSize = DA.GetData(2, ref size);
        DA.GetData(3, ref hash);

        var fileDiff = new FileDiff
        {
            Url = url,
            Data = data,
            Size = hasSize ? size : null,
            Hash = hash
        };
        DA.SetData(0, new FileDiffGoo(fileDiff));
    }
}

public class FileGoo : ModelGoo<File>
{
    public FileGoo() { }
    public FileGoo(File value) : base(value) { }
    protected override string GetModelDescription() => "A file with path and content.";
    protected override string GetModelTypeName() => "File";
    protected override ModelGoo<File> CreateDuplicate() => new FileGoo();
    protected override string GetSerializationKey() => "File";
}

public class FileParam : ModelParam<FileGoo, File>
{
    public FileParam() : base("File", "F", "A file with path and content.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F8");
}

public class FileComponent : ModelComponent<FileParam, FileGoo, File>
{
    public FileComponent() : base("File", "F", "Create a file") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F9");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new FileParam(), "File", "F", "The file to modify", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddBooleanParameter("Validate", "V", "Validate the file", GH_ParamAccess.item, true);
        pManager.AddTextParameter("Url", "U", "The file URL", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Data", "D", "The file data", GH_ParamAccess.item, string.Empty);
        pManager.AddIntegerParameter("Size", "S", "The file size", GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddTextParameter("Hash", "H", "The file hash", GH_ParamAccess.item, string.Empty);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FileParam(), "File", "F", "The file", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        FileGoo? fileGoo = null;
        bool validate = true;
        string url = string.Empty;
        string data = string.Empty;
        int size = 0;
        string hash = string.Empty;

        if (DA.GetData(0, ref fileGoo))
        {
            var existingFile = fileGoo?.Value;
            if (existingFile is not null)
            {
                url = existingFile.Url ?? string.Empty;
                data = existingFile.Data ?? string.Empty;
                size = existingFile.Size ?? 0;
                hash = existingFile.Hash ?? string.Empty;
            }
        }

        DA.GetData(1, ref validate);
        if (DA.GetData(2, ref url) && string.IsNullOrEmpty(url)) url = string.Empty;
        if (DA.GetData(3, ref data) && string.IsNullOrEmpty(data)) data = string.Empty;
        bool hasSize = DA.GetData(4, ref size);
        if (DA.GetData(5, ref hash) && string.IsNullOrEmpty(hash)) hash = string.Empty;

        var file = new File
        {
            Url = url,
            Data = data,
            Size = hasSize ? size : null,
            Hash = hash
        };

        if (validate)
        {
            var result = file.Validate();
            if (!result.Item1)
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, string.Join(", ", result.Item2));
                return;
            }
        }

        DA.SetData(0, new FileGoo(file));
    }
}

public class SerializeFileDiffComponent : SerializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public SerializeFileDiffComponent() : base("Serialize FileDiff", "SFD", "Serialize a file diff to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F2");
    protected override string GetModelTypeName() => "FileDiff";
    protected override string GetModelNickname() => "FD";
    protected override string GetModelDescription() => "The file diff to serialize";
}

public class DeserializeFileDiffComponent : DeserializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public DeserializeFileDiffComponent() : base("Deserialize FileDiff", "DFD", "Deserialize text to a file diff") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F3");
    protected override string GetModelTypeName() => "FileDiff";
    protected override string GetModelNickname() => "FD";
    protected override string GetModelDescription() => "The deserialized file diff";
}

public class SerializeFileComponent : SerializeComponent<FileParam, FileGoo, File>
{
    public SerializeFileComponent() : base("Serialize File", "SF", "Serialize a file to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FA");
    protected override string GetModelTypeName() => "File";
    protected override string GetModelNickname() => "F";
    protected override string GetModelDescription() => "The file to serialize";
}

public class DeserializeFileComponent : DeserializeComponent<FileParam, FileGoo, File>
{
    public DeserializeFileComponent() : base("Deserialize File", "DF", "Deserialize text to a file") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FB");
    protected override string GetModelTypeName() => "File";
    protected override string GetModelNickname() => "F";
    protected override string GetModelDescription() => "The deserialized file";
}

#endregion File

#region Port

public class PortIdGoo : IdGoo<PortId>
{
    public PortIdGoo() { }
    public PortIdGoo(PortId value) : base(value) { }
    protected override string GetModelDescription() => "A port ID identifies a port by its key.";
    protected override string GetModelTypeName() => "PortId";
    protected override IdGoo<PortId> CreateDuplicate() => new PortIdGoo();
    protected override string GetSerializationKey() => "PortId";
}

public class PortIdParam : IdParam<PortIdGoo, PortId>
{
    public PortIdParam() : base("PortId", "PI", "A port ID identifies a port by its key.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B1");
}

public class PortIdComponent : IdComponent<PortIdParam, PortIdGoo, PortId>
{
    public PortIdComponent() : base("Port ID", "PI", "Create a port ID") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B2");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "I", "The port id", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PortIdParam(), "PortId", "PI", "The port ID", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string id = string.Empty;
        if (!DA.GetData(0, ref id))
            return;

        var portId = new PortId { Id = id };
        DA.SetData(0, new PortIdGoo(portId));
    }
}

public class PortDiffGoo : DiffGoo<PortDiff>
{
    public PortDiffGoo() { }
    public PortDiffGoo(PortDiff value) : base(value) { }
    protected override string GetModelDescription() => "A port diff for modifying ports.";
    protected override string GetModelTypeName() => "PortDiff";
    protected override DiffGoo<PortDiff> CreateDuplicate() => new PortDiffGoo();
    protected override string GetSerializationKey() => "PortDiff";
}

public class PortDiffParam : ModelParam<PortDiffGoo, PortDiff>
{
    public PortDiffParam() : base("PortDiff", "PD", "A port diff for modifying ports.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B0");
}

public class PortDiffComponent : DiffComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public PortDiffComponent() : base("Port Diff", "PD", "Create a port diff") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B3");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "I", "The port id", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Description", "D", "The port description", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Family", "F", "The port family", GH_ParamAccess.item, string.Empty);
        pManager.AddBooleanParameter("Mandatory", "M", "Is the port mandatory", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddParameter(new PointParam(), "Point", "P", "The port point", GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddParameter(new VectorParam(), "Direction", "D", "The port direction", GH_ParamAccess.item);
        pManager[5].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PortDiffParam(), "PortDiff", "PD", "The port diff", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string id = string.Empty;
        string description = string.Empty;
        string family = string.Empty;
        bool mandatory = false;
        PointGoo? pointGoo = null;
        VectorGoo? vectorGoo = null;

        if (!DA.GetData(0, ref id))
            return;

        DA.GetData(1, ref description);
        DA.GetData(2, ref family);
        bool hasMandatory = DA.GetData(3, ref mandatory);
        bool hasPoint = DA.GetData(4, ref pointGoo);
        bool hasDirection = DA.GetData(5, ref vectorGoo);

        var portDiff = new PortDiff
        {
            Id = id,
            Description = description,
            Family = family,
            Mandatory = hasMandatory ? mandatory : null,
            Point = hasPoint ? pointGoo?.Value : null,
            Direction = hasDirection ? vectorGoo?.Value : null
        };
        DA.SetData(0, new PortDiffGoo(portDiff));
    }
}

public class PortGoo : ModelGoo<Port>
{
    public PortGoo() { }
    public PortGoo(Port value) : base(value) { }
    protected override string GetModelDescription() => "A port for connecting pieces.";
    protected override string GetModelTypeName() => "Port";
    protected override ModelGoo<Port> CreateDuplicate() => new PortGoo();
    protected override string GetSerializationKey() => "Port";
}

public class PortParam : ModelParam<PortGoo, Port>
{
    public PortParam() : base("Port", "P", "A port for connecting pieces.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB4");
}

public class PortComponent : ModelComponent<PortParam, PortGoo, Port>
{
    public PortComponent() : base("Port", "P", "Create a port") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PortParam(), "Port", "P", "The port to modify", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddBooleanParameter("Validate", "V", "Validate the port", GH_ParamAccess.item, true);
        pManager.AddTextParameter("Id", "I", "The port id", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Description", "D", "The port description", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Family", "F", "The port family", GH_ParamAccess.item, string.Empty);
        pManager.AddBooleanParameter("Mandatory", "M", "Is the port mandatory", GH_ParamAccess.item, false);
        pManager.AddParameter(new PointParam(), "Point", "P", "The port point", GH_ParamAccess.item);
        pManager[6].Optional = true;
        pManager.AddParameter(new VectorParam(), "Direction", "D", "The port direction", GH_ParamAccess.item);
        pManager[7].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PortParam(), "Port", "P", "The port", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        PortGoo? portGoo = null;
        bool validate = true;
        string id = string.Empty;
        string description = string.Empty;
        string family = string.Empty;
        bool mandatory = false;
        PointGoo? pointGoo = null;
        VectorGoo? vectorGoo = null;

        if (DA.GetData(0, ref portGoo))
        {
            var existingPort = portGoo?.Value;
            if (existingPort is not null)
            {
                id = existingPort.Id ?? string.Empty;
                description = existingPort.Description ?? string.Empty;
                family = existingPort.Family ?? string.Empty;
                mandatory = existingPort.Mandatory;
            }
        }

        DA.GetData(1, ref validate);
        if (DA.GetData(2, ref id) && string.IsNullOrEmpty(id)) id = string.Empty;
        if (DA.GetData(3, ref description) && string.IsNullOrEmpty(description)) description = string.Empty;
        if (DA.GetData(4, ref family) && string.IsNullOrEmpty(family)) family = string.Empty;
        DA.GetData(5, ref mandatory);
        DA.GetData(6, ref pointGoo);
        DA.GetData(7, ref vectorGoo);

        var port = new Port
        {
            Id = id,
            Description = description,
            Family = family,
            Mandatory = mandatory,
            Point = pointGoo?.Value,
            Direction = vectorGoo?.Value
        };

        if (validate)
        {
            var result = port.Validate();
            if (!result.Item1)
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, string.Join(", ", result.Item2));
                return;
            }
        }

        DA.SetData(0, new PortGoo(port));
    }
}

public class SerializePortDiffComponent : SerializeComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public SerializePortDiffComponent() : base("Serialize PortDiff", "SPD", "Serialize a port diff to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B4");
    protected override string GetModelTypeName() => "PortDiff";
    protected override string GetModelNickname() => "PD";
    protected override string GetModelDescription() => "The port diff to serialize";
}

public class DeserializePortDiffComponent : DeserializeComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public DeserializePortDiffComponent() : base("Deserialize PortDiff", "DPD", "Deserialize text to a port diff") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B5");
    protected override string GetModelTypeName() => "PortDiff";
    protected override string GetModelNickname() => "PD";
    protected override string GetModelDescription() => "The deserialized port diff";
}

public class SerializePortComponent : SerializeComponent<PortParam, PortGoo, Port>
{
    public SerializePortComponent() : base("Serialize Port", "SP", "Serialize a port to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("1A29F6ED-464D-490F-B072-3412B467F1B5");
    protected override string GetModelTypeName() => "Port";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "The port to serialize";
}

public class DeserializePortComponent : DeserializeComponent<PortParam, PortGoo, Port>
{
    public DeserializePortComponent() : base("Deserialize Port", "DP", "Deserialize text to a port") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("1A29F6ED-464D-490F-B072-3412B467F1B6");
    protected override string GetModelTypeName() => "Port";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "The deserialized port";
}

#endregion Port

#region Type

public class TypeIdGoo : IdGoo<TypeId>
{
    public TypeIdGoo() { }
    public TypeIdGoo(TypeId value) : base(value) { }
    protected override string GetModelDescription() => "A type ID identifies a type by its key.";
    protected override string GetModelTypeName() => "TypeId";
    protected override IdGoo<TypeId> CreateDuplicate() => new TypeIdGoo();
    protected override string GetSerializationKey() => "TypeId";
}

public class TypeIdParam : IdParam<TypeIdGoo, TypeId>
{
    public TypeIdParam() : base("TypeId", "TI", "A type ID identifies a type by its key.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C2");
}

public class TypeIdComponent : IdComponent<TypeIdParam, TypeIdGoo, TypeId>
{
    public TypeIdComponent() : base("Type ID", "TI", "Create a type ID") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C3");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "The type name", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "V", "The type variant", GH_ParamAccess.item, string.Empty);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeIdParam(), "TypeId", "TI", "The type ID", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string name = string.Empty;
        string variant = string.Empty;
        if (!DA.GetData(0, ref name))
            return;

        DA.GetData(1, ref variant);

        var typeId = new TypeId { Name = name, Variant = variant };
        DA.SetData(0, new TypeIdGoo(typeId));
    }
}

public class TypeDiffGoo : DiffGoo<TypeDiff>
{
    public TypeDiffGoo() { }
    public TypeDiffGoo(TypeDiff value) : base(value) { }
    protected override string GetModelDescription() => "A type diff for modifying types.";
    protected override string GetModelTypeName() => "TypeDiff";
    protected override DiffGoo<TypeDiff> CreateDuplicate() => new TypeDiffGoo();
    protected override string GetSerializationKey() => "TypeDiff";
}

public class TypeDiffParam : ModelParam<TypeDiffGoo, TypeDiff>
{
    public TypeDiffParam() : base("TypeDiff", "TD", "A type diff for modifying types.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C1");
}

public class TypeDiffComponent : DiffComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public TypeDiffComponent() : base("Type Diff", "TD", "Create a type diff") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C4");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "The type name", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Description", "D", "The type description", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Variant", "V", "The type variant", GH_ParamAccess.item, string.Empty);
        pManager.AddBooleanParameter("Virtual", "Vi", "Is the type virtual", GH_ParamAccess.item);
        pManager[3].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeDiffParam(), "TypeDiff", "TD", "The type diff", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string name = string.Empty;
        string description = string.Empty;
        string variant = string.Empty;
        bool virtualType = false;

        if (!DA.GetData(0, ref name))
            return;

        DA.GetData(1, ref description);
        DA.GetData(2, ref variant);
        bool hasVirtual = DA.GetData(3, ref virtualType);

        var typeDiff = new TypeDiff
        {
            Name = name,
            Description = description,
            Variant = variant,
            Virtual = hasVirtual ? virtualType : null
        };
        DA.SetData(0, new TypeDiffGoo(typeDiff));
    }
}

public class TypeGoo : ModelGoo<Type>
{
    public TypeGoo() { }
    public TypeGoo(Type value) : base(value) { }
    protected override string GetModelDescription() => "A type defines a reusable component.";
    protected override string GetModelTypeName() => "Type";
    protected override ModelGoo<Type> CreateDuplicate() => new TypeGoo();
    protected override string GetSerializationKey() => "Type";
}

public class TypeParam : ModelParam<TypeGoo, Type>
{
    public TypeParam() : base("Type", "T", "A type defines a reusable component.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A6");
}

public class TypeComponent : ModelComponent<TypeParam, TypeGoo, Type>
{
    public TypeComponent() : base("Type", "T", "Create a type") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "T", "The type to modify", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddBooleanParameter("Validate", "V", "Validate the type", GH_ParamAccess.item, true);
        pManager.AddTextParameter("Name", "N", "The type name", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Description", "D", "The type description", GH_ParamAccess.item, string.Empty);
        pManager.AddTextParameter("Variant", "Va", "The type variant", GH_ParamAccess.item, string.Empty);
        pManager.AddBooleanParameter("Virtual", "Vi", "Is the type virtual", GH_ParamAccess.item, false);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "T", "The type", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        TypeGoo? typeGoo = null;
        bool validate = true;
        string name = string.Empty;
        string description = string.Empty;
        string variant = string.Empty;
        bool virtualType = false;

        if (DA.GetData(0, ref typeGoo))
        {
            var existingType = typeGoo?.Value;
            if (existingType is not null)
            {
                name = existingType.Name ?? string.Empty;
                description = existingType.Description ?? string.Empty;
                variant = existingType.Variant ?? string.Empty;
                virtualType = existingType.Virtual;
            }
        }

        DA.GetData(1, ref validate);
        if (DA.GetData(2, ref name) && string.IsNullOrEmpty(name)) name = string.Empty;
        if (DA.GetData(3, ref description) && string.IsNullOrEmpty(description)) description = string.Empty;
        if (DA.GetData(4, ref variant) && string.IsNullOrEmpty(variant)) variant = string.Empty;
        DA.GetData(5, ref virtualType);

        var type = new Type
        {
            Name = name,
            Description = description,
            Variant = variant,
            Virtual = virtualType
        };

        if (validate)
        {
            var result = type.Validate();
            if (!result.Item1)
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, string.Join(", ", result.Item2));
                return;
            }
        }

        DA.SetData(0, new TypeGoo(type));
    }
}

public class SerializeTypeDiffComponent : SerializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public SerializeTypeDiffComponent() : base("Serialize TypeDiff", "STD", "Serialize a type diff to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C5");
    protected override string GetModelTypeName() => "TypeDiff";
    protected override string GetModelNickname() => "TD";
    protected override string GetModelDescription() => "The type diff to serialize";
}

public class DeserializeTypeDiffComponent : DeserializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public DeserializeTypeDiffComponent() : base("Deserialize TypeDiff", "DTD", "Deserialize text to a type diff") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C6");
    protected override string GetModelTypeName() => "TypeDiff";
    protected override string GetModelNickname() => "TD";
    protected override string GetModelDescription() => "The deserialized type diff";
}

public class SerializeTypeComponent : SerializeComponent<TypeParam, TypeGoo, Type>
{
    public SerializeTypeComponent() : base("Serialize Type", "ST", "Serialize a type to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("BD184BB8-8124-4604-835C-E7B7C199673A");
    protected override string GetModelTypeName() => "Type";
    protected override string GetModelNickname() => "T";
    protected override string GetModelDescription() => "The type to serialize";
}

public class DeserializeTypeComponent : DeserializeComponent<TypeParam, TypeGoo, Type>
{
    public DeserializeTypeComponent() : base("Deserialize Type", "DT", "Deserialize text to a type") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("BD184BB8-8124-4604-835C-E7B7C199673B");
    protected override string GetModelTypeName() => "Type";
    protected override string GetModelNickname() => "T";
    protected override string GetModelDescription() => "The deserialized type";
}

#endregion Type

#region Piece

public class PieceDiffGoo : DiffGoo<PieceDiff>
{
    public PieceDiffGoo() { }
    public PieceDiffGoo(PieceDiff value) : base(value) { }
    protected override string GetModelDescription() => "A piece diff for modifying pieces.";
    protected override string GetModelTypeName() => "PieceDiff";
    protected override DiffGoo<PieceDiff> CreateDuplicate() => new PieceDiffGoo();
    protected override string GetSerializationKey() => "PieceDiff";
}

public class PieceDiffParam : ModelParam<PieceDiffGoo, PieceDiff>
{
    public PieceDiffParam() : base("PieceDiff", "PD", "A piece diff for modifying pieces.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D4");
}

public class PieceDiffComponent : DiffComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public PieceDiffComponent() : base("Piece Diff", "PD", "Create a piece diff") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D5");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Id", "I", "The piece id", GH_ParamAccess.item, string.Empty);
        pManager.AddParameter(new TypeIdParam(), "TypeId", "TI", "The type id", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddParameter(new PlaneParam(), "Plane", "P", "The piece plane", GH_ParamAccess.item);
        pManager[2].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceDiffParam(), "PieceDiff", "PD", "The piece diff", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string id = string.Empty;
        TypeIdGoo? typeIdGoo = null;
        PlaneGoo? planeGoo = null;

        if (!DA.GetData(0, ref id))
            return;

        bool hasTypeId = DA.GetData(1, ref typeIdGoo);
        bool hasPlane = DA.GetData(2, ref planeGoo);

        var pieceDiff = new PieceDiff
        {
            Id = id,
            Type = hasTypeId ? typeIdGoo?.Value : null,
            Plane = hasPlane ? planeGoo?.Value : null
        };
        DA.SetData(0, new PieceDiffGoo(pieceDiff));
    }
}

public class PieceGoo : ModelGoo<Piece>
{
    public PieceGoo() { }
    public PieceGoo(Piece value) : base(value) { }
    protected override string GetModelDescription() => "A piece is an instance of a type.";
    protected override string GetModelTypeName() => "Piece";
    protected override ModelGoo<Piece> CreateDuplicate() => new PieceGoo();
    protected override string GetSerializationKey() => "Piece";
}

public class PieceParam : ModelParam<PieceGoo, Piece>
{
    public PieceParam() : base("Piece", "P", "A piece is an instance of a type.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA47");
}

public class PieceComponent : ModelComponent<PieceParam, PieceGoo, Piece>
{
    public PieceComponent() : base("Piece", "P", "Create a piece") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA48");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "P", "The piece to modify", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddBooleanParameter("Validate", "V", "Validate the piece", GH_ParamAccess.item, true);
        pManager.AddTextParameter("Id", "I", "The piece id", GH_ParamAccess.item, string.Empty);
        pManager.AddParameter(new TypeIdParam(), "TypeId", "TI", "The type id", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddParameter(new PlaneParam(), "Plane", "P", "The piece plane", GH_ParamAccess.item);
        pManager[4].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "P", "The piece", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        PieceGoo? pieceGoo = null;
        bool validate = true;
        string id = string.Empty;
        TypeIdGoo? typeIdGoo = null;
        PlaneGoo? planeGoo = null;

        if (DA.GetData(0, ref pieceGoo))
        {
            var existingPiece = pieceGoo?.Value;
            if (existingPiece is not null)
            {
                id = existingPiece.Id ?? string.Empty;
            }
        }

        DA.GetData(1, ref validate);
        if (DA.GetData(2, ref id) && string.IsNullOrEmpty(id)) id = string.Empty;
        DA.GetData(3, ref typeIdGoo);
        DA.GetData(4, ref planeGoo);

        var piece = new Piece
        {
            Id = id,
            Type = typeIdGoo?.Value,
            Plane = planeGoo?.Value
        };

        if (validate)
        {
            var result = piece.Validate();
            if (!result.Item1)
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, string.Join(", ", result.Item2));
                return;
            }
        }

        DA.SetData(0, new PieceGoo(piece));
    }
}

public class SerializePieceDiffComponent : SerializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public SerializePieceDiffComponent() : base("Serialize PieceDiff", "SPD", "Serialize a piece diff to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D6");
    protected override string GetModelTypeName() => "PieceDiff";
    protected override string GetModelNickname() => "PD";
    protected override string GetModelDescription() => "The piece diff to serialize";
}

public class DeserializePieceDiffComponent : DeserializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public DeserializePieceDiffComponent() : base("Deserialize PieceDiff", "DPD", "Deserialize text to a piece diff") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D7");
    protected override string GetModelTypeName() => "PieceDiff";
    protected override string GetModelNickname() => "PD";
    protected override string GetModelDescription() => "The deserialized piece diff";
}

public class SerializePieceComponent : SerializeComponent<PieceParam, PieceGoo, Piece>
{
    public SerializePieceComponent() : base("Serialize Piece", "SP", "Serialize a piece to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("A4EDA838-2246-4617-8298-9585ECFE00D9");
    protected override string GetModelTypeName() => "Piece";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "The piece to serialize";
}

public class DeserializePieceComponent : DeserializeComponent<PieceParam, PieceGoo, Piece>
{
    public DeserializePieceComponent() : base("Deserialize Piece", "DP", "Deserialize text to a piece") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("A4EDA838-2246-4617-8298-9585ECFE00DA");
    protected override string GetModelTypeName() => "Piece";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "The deserialized piece";
}

#endregion Piece

#region Connection

public class ConnectionIdGoo : IdGoo<ConnectionId>
{
    public ConnectionIdGoo() { }
    public ConnectionIdGoo(ConnectionId value) : base(value) { }
    protected override string GetModelDescription() => "A connection ID identifies a connection by its key.";
    protected override string GetModelTypeName() => "ConnectionId";
    protected override IdGoo<ConnectionId> CreateDuplicate() => new ConnectionIdGoo();
    protected override string GetSerializationKey() => "ConnectionId";
}

public class ConnectionIdParam : IdParam<ConnectionIdGoo, ConnectionId>
{
    public ConnectionIdParam() : base("ConnectionId", "CI", "A connection ID identifies a connection by its key.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D6");
}

public class ConnectionIdComponent : IdComponent<ConnectionIdParam, ConnectionIdGoo, ConnectionId>
{
    public ConnectionIdComponent() : base("Connection ID", "CI", "Create a connection ID") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D7");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Connected", "Cn", "The connected side description", GH_ParamAccess.item);
        pManager.AddTextParameter("Connecting", "Cg", "The connecting side description", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionIdParam(), "ConnectionId", "CI", "The connection ID", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string connected = string.Empty;
        string connecting = string.Empty;
        if (!DA.GetData(0, ref connected))
            return;
        if (!DA.GetData(1, ref connecting))
            return;

        // Simple implementation - in full version this would create proper Side objects
        var connectionId = new ConnectionId
        {
            Connected = new Side(),
            Connecting = new Side()
        };
        DA.SetData(0, new ConnectionIdGoo(connectionId));
    }
}

public class ConnectionDiffGoo : DiffGoo<ConnectionDiff>
{
    public ConnectionDiffGoo() { }
    public ConnectionDiffGoo(ConnectionDiff value) : base(value) { }
    protected override string GetModelDescription() => "A connection diff for modifying connections.";
    protected override string GetModelTypeName() => "ConnectionDiff";
    protected override DiffGoo<ConnectionDiff> CreateDuplicate() => new ConnectionDiffGoo();
    protected override string GetSerializationKey() => "ConnectionDiff";
}

public class ConnectionDiffParam : ModelParam<ConnectionDiffGoo, ConnectionDiff>
{
    public ConnectionDiffParam() : base("ConnectionDiff", "CD", "A connection diff for modifying connections.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F4");
}

public class ConnectionDiffComponent : DiffComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public ConnectionDiffComponent() : base("Connection Diff", "CD", "Create a connection diff") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F5");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Description", "D", "The connection description", GH_ParamAccess.item, string.Empty);
        pManager.AddNumberParameter("Gap", "G", "The connection gap", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddNumberParameter("Shift", "S", "The connection shift", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddNumberParameter("Rise", "R", "The connection rise", GH_ParamAccess.item);
        pManager[3].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionDiffParam(), "ConnectionDiff", "CD", "The connection diff", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        string description = string.Empty;
        double gap = 0;
        double shift = 0;
        double rise = 0;

        DA.GetData(0, ref description);
        bool hasGap = DA.GetData(1, ref gap);
        bool hasShift = DA.GetData(2, ref shift);
        bool hasRise = DA.GetData(3, ref rise);

        var connectionDiff = new ConnectionDiff
        {
            Description = description,
            Gap = hasGap ? (float)gap : null,
            Shift = hasShift ? (float)shift : null,
            Rise = hasRise ? (float)rise : null
        };
        DA.SetData(0, new ConnectionDiffGoo(connectionDiff));
    }
}

public class ConnectionGoo : ModelGoo<Connection>
{
    public ConnectionGoo() { }
    public ConnectionGoo(Connection value) : base(value) { }
    protected override string GetModelDescription() => "A connection between two pieces.";
    protected override string GetModelTypeName() => "Connection";
    protected override ModelGoo<Connection> CreateDuplicate() => new ConnectionGoo();
    protected override string GetSerializationKey() => "Connection";
}

public class ConnectionParam : ModelParam<ConnectionGoo, Connection>
{
    public ConnectionParam() : base("Connection", "C", "A connection between two pieces.") { }
    protected override Bitmap GetParamIcon() => Resources.deserialize_24x24;
    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827559");
}

public class ConnectionComponent : ModelComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public ConnectionComponent() : base("Connection", "C", "Create a connection") { }

    protected override Bitmap GetComponentIcon() => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AB212F90-124C-4985-B3EE-1C13D7827560");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "C", "The connection to modify", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddBooleanParameter("Validate", "V", "Validate the connection", GH_ParamAccess.item, true);
        pManager.AddTextParameter("Description", "D", "The connection description", GH_ParamAccess.item, string.Empty);
        pManager.AddNumberParameter("Gap", "G", "The connection gap", GH_ParamAccess.item, 0.0);
        pManager.AddNumberParameter("Shift", "S", "The connection shift", GH_ParamAccess.item, 0.0);
        pManager.AddNumberParameter("Rise", "R", "The connection rise", GH_ParamAccess.item, 0.0);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "C", "The connection", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        ConnectionGoo? connectionGoo = null;
        bool validate = true;
        string description = string.Empty;
        double gap = 0;
        double shift = 0;
        double rise = 0;

        if (DA.GetData(0, ref connectionGoo))
        {
            var existingConnection = connectionGoo?.Value;
            if (existingConnection is not null)
            {
                description = existingConnection.Description ?? string.Empty;
                gap = existingConnection.Gap;
                shift = existingConnection.Shift;
                rise = existingConnection.Rise;
            }
        }

        DA.GetData(1, ref validate);
        if (DA.GetData(2, ref description) && string.IsNullOrEmpty(description)) description = string.Empty;
        DA.GetData(3, ref gap);
        DA.GetData(4, ref shift);
        DA.GetData(5, ref rise);

        var connection = new Connection
        {
            Description = description,
            Gap = (float)gap,
            Shift = (float)shift,
            Rise = (float)rise
        };

        if (validate)
        {
            var result = connection.Validate();
            if (!result.Item1)
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, string.Join(", ", result.Item2));
                return;
            }
        }

        DA.SetData(0, new ConnectionGoo(connection));
    }
}

public class SerializeConnectionDiffComponent : SerializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public SerializeConnectionDiffComponent() : base("Serialize ConnectionDiff", "SCD", "Serialize a connection diff to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F6");
    protected override string GetModelTypeName() => "ConnectionDiff";
    protected override string GetModelNickname() => "CD";
    protected override string GetModelDescription() => "The connection diff to serialize";
}

public class DeserializeConnectionDiffComponent : DeserializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public DeserializeConnectionDiffComponent() : base("Deserialize ConnectionDiff", "DCD", "Deserialize text to a connection diff") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F7");
    protected override string GetModelTypeName() => "ConnectionDiff";
    protected override string GetModelNickname() => "CD";
    protected override string GetModelDescription() => "The deserialized connection diff";
}

public class SerializeConnectionComponent : SerializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public SerializeConnectionComponent() : base("Serialize Connection", "SC", "Serialize a connection to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("93FBA84E-79A1-4E32-BE61-A925F476DD60");
    protected override string GetModelTypeName() => "Connection";
    protected override string GetModelNickname() => "C";
    protected override string GetModelDescription() => "The connection to serialize";
}

public class DeserializeConnectionComponent : DeserializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public DeserializeConnectionComponent() : base("Deserialize Connection", "DC", "Deserialize text to a connection") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("93FBA84E-79A1-4E32-BE61-A925F476DD61");
    protected override string GetModelTypeName() => "Connection";
    protected override string GetModelNickname() => "C";
    protected override string GetModelDescription() => "The deserialized connection";
}

#endregion Connection

#region RepresentationDiff

public class RepresentationDiffGoo : DiffGoo<RepresentationDiff>
{
    public RepresentationDiffGoo() { }
    public RepresentationDiffGoo(RepresentationDiff value) : base(value) { }
    protected override string GetModelDescription() => "A representation diff for modifying representations.";
    protected override string GetModelTypeName() => "RepresentationDiff";
    protected override DiffGoo<RepresentationDiff> CreateDuplicate() => new RepresentationDiffGoo();
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
    protected override Guid GetComponentGuid() => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Rp?", "The optional representation to deconstruct or modify.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Validate", "Vd?", "Whether the representation should be validated.", GH_ParamAccess.item);
        pManager.AddTextParameter("Url", "Ur", "The Unique Resource Locator (URL) to the resource of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional human-readable description of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Tags", "Tg*", "The optional tags to group representations. No tags means default.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes of the representation.", GH_ParamAccess.list);

        pManager[0].Optional = true;
        pManager[1].Optional = true;
        pManager[2].Optional = true;
        pManager[3].Optional = true;
        pManager[4].Optional = true;
        pManager[5].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Rp", "The constructed or modified representation.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Valid", "Vd?", "True if the representation is valid. Null if no validation was performed.", GH_ParamAccess.item);
        pManager.AddTextParameter("Url", "Ur", "The Unique Resource Locator (URL) to the resource of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional human-readable description of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Tags", "Tg*", "The optional tags to group representations. No tags means default.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes of the representation.", GH_ParamAccess.list);
    }

    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        var representationGoo = new RepresentationGoo(new Representation());
        var validate = false;
        var url = "";
        var description = "";
        var tags = new List<string>();
        var attributeGoos = new List<AttributeGoo>();

        if (DA.GetData(0, ref representationGoo))
            representationGoo = (RepresentationGoo)representationGoo.Duplicate();
        DA.GetData(1, ref validate);
        if (DA.GetData(2, ref url)) representationGoo.Value.Url = url;
        if (DA.GetData(3, ref description)) representationGoo.Value.Description = description;
        if (DA.GetDataList(4, tags)) representationGoo.Value.Tags = tags;
        if (DA.GetDataList(5, attributeGoos))
        {
            var attributes = attributeGoos.Where(g => g?.Value is not null).Select(g => g.Value).ToList();
            representationGoo.Value.Attributes = attributes;
        }

        // ProcessModel logic from old system
        var mime = Semio.Utility.ParseMimeFromUrl(representationGoo.Value.Url);
        var firstTag = representationGoo.Value.Tags.FirstOrDefault();
        if (firstTag == null || (firstTag != null && mime != "" && !Semio.Utility.IsValidMime(firstTag)))
            representationGoo.Value.Tags.Insert(0, mime);
        representationGoo.Value.Url = representationGoo.Value.Url.Replace('\\', '/');
        if (firstTag != null && Semio.Utility.IsValidMime(firstTag))
            representationGoo.Value.Tags[0] = firstTag;

        if (validate)
        {
            var (isValid, errors) = representationGoo.Value.Validate();
            foreach (var error in errors)
                AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, error);
            DA.SetData(1, isValid);
        }

        DA.SetData(0, representationGoo.Duplicate());
        DA.SetData(2, representationGoo.Value.Url);
        DA.SetData(3, representationGoo.Value.Description);
        DA.SetDataList(4, representationGoo.Value.Tags);
        DA.SetDataList(5, representationGoo.Value.Attributes.Select(a => new AttributeGoo(a)));
    }
}

public class SerializeRepresentationComponent : SerializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public SerializeRepresentationComponent() : base("Serialize Representation", "SRp", "Serialize a representation to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32046");
    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelNickname() => "Rp";
    protected override string GetModelDescription() => "The representation to serialize";
}

public class DeserializeRepresentationComponent : DeserializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public DeserializeRepresentationComponent() : base("Deserialize Representation", "DRp", "Deserialize text to a representation") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32047");
    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelNickname() => "Rp";
    protected override string GetModelDescription() => "The deserialized representation";
}

#endregion Representation

public class AuthorIdGoo : IdGoo<AuthorId>
{
    public AuthorIdGoo() { }
    public AuthorIdGoo(AuthorId value) : base(value) { }
    protected override string GetModelTypeName() => "AuthorId";
    protected override string GetModelDescription() => "An author id is a key for an author.";
    protected override IdGoo<AuthorId> CreateDuplicate() => new AuthorIdGoo();
    protected override string GetSerializationKey() => "AuthorId";
}

public class AuthorIdParam : IdParam<AuthorIdGoo, AuthorId>
{
    public AuthorIdParam() : base("AuthorId", "AI", "An author id is a key for an author.") { }
    protected override Bitmap GetParamIcon() => Resources.author_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32060");
}

public class AuthorIdComponent : IdComponent<AuthorIdParam, AuthorIdGoo, AuthorId>
{
    public AuthorIdComponent() : base("Author ID", "AI", "Create an author ID") { }

    protected override Bitmap GetComponentIcon() => Resources.author_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32061");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Email", "E", "Author email", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Email", "E", "Author email", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var email = "";
        if (!da.GetData(0, ref email)) return;

        var authorId = new AuthorId { Email = email };
        var (isValid, errors) = authorId.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, email);
        da.SetData("ID", new AuthorIdGoo(authorId));
    }
}

public class AuthorGoo : ModelGoo<Author>
{
    public AuthorGoo() { }
    public AuthorGoo(Author value) : base(value) { }
    protected override string GetModelTypeName() => "Author";
    protected override string GetModelDescription() => "An author.";
    protected override ModelGoo<Author> CreateDuplicate() => new AuthorGoo();
    protected override string GetSerializationKey() => "Author";
}

public class AuthorParam : ModelParam<AuthorGoo, Author>
{
    public AuthorParam() : base("Author", "A", "An author.") { }
    protected override Bitmap GetParamIcon() => Resources.author_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32062");
}

public class AuthorComponent : ModelComponent<AuthorParam, AuthorGoo, Author>
{
    public AuthorComponent() : base("Author", "A", "Create an author") { }

    protected override Bitmap GetComponentIcon() => Resources.author_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32063");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Author name", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Email", "E", "Author email", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Author attributes", GH_ParamAccess.list);
        pManager[2].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Author name", GH_ParamAccess.item);
        pManager.AddTextParameter("Email", "E", "Author email", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Author attributes", GH_ParamAccess.list);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var name = "";
        var email = "";
        var attributes = new List<Attribute>();

        if (!da.GetData(0, ref name)) return;
        if (!da.GetData(1, ref email)) return;
        da.GetDataList(2, attributes);

        var author = new Author { Name = name, Email = email, Attributes = attributes };
        var (isValid, errors) = author.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, name);
        da.SetData(1, email);
        da.SetDataList(2, attributes.Select(a => new AttributeGoo(a)));
        da.SetData("Author", new AuthorGoo(author));
    }
}

public class SerializeAuthorIdComponent : SerializeComponent<AuthorIdParam, AuthorIdGoo, AuthorId>
{
    public SerializeAuthorIdComponent() : base("Serialize Author ID", "SAI", "Serialize an author ID to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32064");
    protected override string GetModelTypeName() => "AuthorId";
    protected override string GetModelNickname() => "AI";
    protected override string GetModelDescription() => "The author ID to serialize";
}

public class DeserializeAuthorIdComponent : DeserializeComponent<AuthorIdParam, AuthorIdGoo, AuthorId>
{
    public DeserializeAuthorIdComponent() : base("Deserialize Author ID", "DAI", "Deserialize text to an author ID") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32065");
    protected override string GetModelTypeName() => "AuthorId";
    protected override string GetModelNickname() => "AI";
    protected override string GetModelDescription() => "The deserialized author ID";
}

public class SerializeAuthorComponent : SerializeComponent<AuthorParam, AuthorGoo, Author>
{
    public SerializeAuthorComponent() : base("Serialize Author", "SA", "Serialize an author to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32066");
    protected override string GetModelTypeName() => "Author";
    protected override string GetModelNickname() => "A";
    protected override string GetModelDescription() => "The author to serialize";
}

public class DeserializeAuthorComponent : DeserializeComponent<AuthorParam, AuthorGoo, Author>
{
    public DeserializeAuthorComponent() : base("Deserialize Author", "DA", "Deserialize text to an author") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32067");
    protected override string GetModelTypeName() => "Author";
    protected override string GetModelNickname() => "A";
    protected override string GetModelDescription() => "The deserialized author";
}

public class LocationGoo : ModelGoo<Location>
{
    public LocationGoo() { }
    public LocationGoo(Location value) : base(value) { }
    protected override string GetModelTypeName() => "Location";
    protected override string GetModelDescription() => "A location.";
    protected override ModelGoo<Location> CreateDuplicate() => new LocationGoo();
    protected override string GetSerializationKey() => "Location";
}

public class LocationParam : ModelParam<LocationGoo, Location>
{
    public LocationParam() : base("Location", "L", "A location.") { }
    protected override Bitmap GetParamIcon() => Resources.location_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32068");
}

public class LocationComponent : ModelComponent<LocationParam, LocationGoo, Location>
{
    public LocationComponent() : base("Location", "L", "Create a location") { }

    protected override Bitmap GetComponentIcon() => Resources.location_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32069");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("Longitude", "Lo", "Location longitude", GH_ParamAccess.item);
        pManager.AddNumberParameter("Latitude", "La", "Location latitude", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Location attributes", GH_ParamAccess.list);
        pManager[2].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddNumberParameter("Longitude", "Lo", "Location longitude", GH_ParamAccess.item);
        pManager.AddNumberParameter("Latitude", "La", "Location latitude", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Location attributes", GH_ParamAccess.list);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var longitude = 0.0;
        var latitude = 0.0;
        var attributes = new List<Attribute>();

        if (!da.GetData(0, ref longitude)) return;
        if (!da.GetData(1, ref latitude)) return;
        da.GetDataList(2, attributes);

        var location = new Location { Longitude = (float)longitude, Latitude = (float)latitude, Attributes = attributes };
        var (isValid, errors) = location.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, longitude);
        da.SetData(1, latitude);
        da.SetDataList(2, attributes.Select(a => new AttributeGoo(a)));
        da.SetData("Location", new LocationGoo(location));
    }
}

public class SerializeLocationComponent : SerializeComponent<LocationParam, LocationGoo, Location>
{
    public SerializeLocationComponent() : base("Serialize Location", "SL", "Serialize a location to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32070");
    protected override string GetModelTypeName() => "Location";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "The location to serialize";
}

public class DeserializeLocationComponent : DeserializeComponent<LocationParam, LocationGoo, Location>
{
    public DeserializeLocationComponent() : base("Deserialize Location", "DL", "Deserialize text to a location") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32071");
    protected override string GetModelTypeName() => "Location";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "The deserialized location";
}

public class DesignIdGoo : IdGoo<DesignId>
{
    public DesignIdGoo() { }
    public DesignIdGoo(DesignId value) : base(value) { }
    protected override string GetModelTypeName() => "DesignId";
    protected override string GetModelDescription() => "A design id is a key for a design.";
    protected override IdGoo<DesignId> CreateDuplicate() => new DesignIdGoo();
    protected override string GetSerializationKey() => "DesignId";
}

public class DesignIdParam : IdParam<DesignIdGoo, DesignId>
{
    public DesignIdParam() : base("DesignId", "DI", "A design id is a key for a design.") { }
    protected override Bitmap GetParamIcon() => Resources.design_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32072");
}

public class DesignIdComponent : IdComponent<DesignIdParam, DesignIdGoo, DesignId>
{
    public DesignIdComponent() : base("Design ID", "DI", "Create a design ID") { }

    protected override Bitmap GetComponentIcon() => Resources.design_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32073");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Design name", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "V", "Design variant", GH_ParamAccess.item, "");
        pManager.AddTextParameter("View", "Vw", "Design view", GH_ParamAccess.item, "");
        pManager[1].Optional = true;
        pManager[2].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Design name", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "V", "Design variant", GH_ParamAccess.item);
        pManager.AddTextParameter("View", "Vw", "Design view", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var name = "";
        var variant = "";
        var view = "";

        if (!da.GetData(0, ref name)) return;
        da.GetData(1, ref variant);
        da.GetData(2, ref view);

        var designId = new DesignId { Name = name, Variant = variant, View = view };
        var (isValid, errors) = designId.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, name);
        da.SetData(1, variant);
        da.SetData(2, view);
        da.SetData("ID", new DesignIdGoo(designId));
    }
}

public class DesignDiffGoo : DiffGoo<DesignDiff>
{
    public DesignDiffGoo() { }
    public DesignDiffGoo(DesignDiff value) : base(value) { }
    protected override string GetModelTypeName() => "DesignDiff";
    protected override string GetModelDescription() => "A design diff.";
    protected override DiffGoo<DesignDiff> CreateDuplicate() => new DesignDiffGoo();
    protected override string GetSerializationKey() => "DesignDiff";
}

public class DesignDiffParam : ModelParam<DesignDiffGoo, DesignDiff>
{
    public DesignDiffParam() : base("DesignDiff", "DD", "A design diff.") { }
    protected override Bitmap GetParamIcon() => Resources.design_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32074");
}

public class DesignDiffComponent : DiffComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public DesignDiffComponent() : base("Design Diff", "DD", "Create a design diff") { }

    protected override Bitmap GetComponentIcon() => Resources.design_diff_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32075");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Design name", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Description", "D", "Design description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Icon", "I", "Design icon", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Image", "Im", "Design image", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Variant", "V", "Design variant", GH_ParamAccess.item, "");
        pManager.AddTextParameter("View", "Vw", "Design view", GH_ParamAccess.item, "");
        pManager.AddParameter(new LocationParam(), "Location", "L", "Design location", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "U", "Design unit", GH_ParamAccess.item, "");
        pManager.AddParameter(new StatParam(), "Stats", "St", "Design stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Design attributes", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorParam(), "Authors", "Au", "Design authors", GH_ParamAccess.list);
        pManager.AddTextParameter("Concepts", "C", "Design concepts", GH_ParamAccess.list);
        for (int i = 0; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Design name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "Design description", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "I", "Design icon", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im", "Design image", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "V", "Design variant", GH_ParamAccess.item);
        pManager.AddTextParameter("View", "Vw", "Design view", GH_ParamAccess.item);
        pManager.AddParameter(new LocationParam(), "Location", "L", "Design location", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "U", "Design unit", GH_ParamAccess.item);
        pManager.AddParameter(new StatParam(), "Stats", "St", "Design stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Design attributes", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorParam(), "Authors", "Au", "Design authors", GH_ParamAccess.list);
        pManager.AddTextParameter("Concepts", "C", "Design concepts", GH_ParamAccess.list);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var name = "";
        var description = "";
        var icon = "";
        var image = "";
        var variant = "";
        var view = "";
        Location? location = null;
        var unit = "";
        var stats = new List<Stat>();
        var attributes = new List<Attribute>();
        var authors = new List<Author>();
        var concepts = new List<string>();

        da.GetData(0, ref name);
        da.GetData(1, ref description);
        da.GetData(2, ref icon);
        da.GetData(3, ref image);
        da.GetData(4, ref variant);
        da.GetData(5, ref view);
        da.GetData(6, ref location);
        da.GetData(7, ref unit);
        da.GetDataList(8, stats);
        da.GetDataList(9, attributes);
        da.GetDataList(10, authors);
        da.GetDataList(11, concepts);

        var designDiff = new DesignDiff
        {
            Name = name,
            Description = description,
            Icon = icon,
            Image = image,
            Variant = variant,
            View = view,
            Location = location,
            Unit = unit,
            Stats = stats,
            Attributes = attributes,
            Authors = authors,
            Concepts = concepts
        };

        var (isValid, errors) = designDiff.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, name);
        da.SetData(1, description);
        da.SetData(2, icon);
        da.SetData(3, image);
        da.SetData(4, variant);
        da.SetData(5, view);
        da.SetData(6, location);
        da.SetData(7, unit);
        da.SetDataList(8, stats.Select(s => new StatGoo(s)));
        da.SetDataList(9, attributes.Select(a => new AttributeGoo(a)));
        da.SetDataList(10, authors.Select(a => new AuthorGoo(a)));
        da.SetDataList(11, concepts);
        da.SetData("Diff", new DesignDiffGoo(designDiff));
    }
}

public class DesignGoo : ModelGoo<Design>
{
    public DesignGoo() { }
    public DesignGoo(Design value) : base(value) { }
    protected override string GetModelTypeName() => "Design";
    protected override string GetModelDescription() => "A design.";
    protected override ModelGoo<Design> CreateDuplicate() => new DesignGoo();
    protected override string GetSerializationKey() => "Design";
}

public class DesignParam : ModelParam<DesignGoo, Design>
{
    public DesignParam() : base("Design", "D", "A design.") { }
    protected override Bitmap GetParamIcon() => Resources.design_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32076");
}

public class DesignComponent : ModelComponent<DesignParam, DesignGoo, Design>
{
    public DesignComponent() : base("Design", "D", "Create a design") { }

    protected override Bitmap GetComponentIcon() => Resources.design_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32077");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Design name", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "V", "Design variant", GH_ParamAccess.item, "");
        pManager.AddTextParameter("View", "Vw", "Design view", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Description", "D", "Design description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Icon", "I", "Design icon", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Image", "Im", "Design image", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Concepts", "C", "Design concepts", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au", "Design authors", GH_ParamAccess.list);
        pManager.AddParameter(new LocationParam(), "Location", "L", "Design location", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "U", "Design unit", GH_ParamAccess.item, "");
        pManager.AddParameter(new LayerParam(), "Layers", "Ly", "Design layers", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam(), "Pieces", "P", "Design pieces", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "G", "Design groups", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Cn", "Design connections", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Pr", "Design props", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "St", "Design stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Design attributes", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Design name", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "V", "Design variant", GH_ParamAccess.item);
        pManager.AddTextParameter("View", "Vw", "Design view", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "Design description", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "I", "Design icon", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im", "Design image", GH_ParamAccess.item);
        pManager.AddTextParameter("Concepts", "C", "Design concepts", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au", "Design authors", GH_ParamAccess.list);
        pManager.AddParameter(new LocationParam(), "Location", "L", "Design location", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "U", "Design unit", GH_ParamAccess.item);
        pManager.AddParameter(new LayerParam(), "Layers", "Ly", "Design layers", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam(), "Pieces", "P", "Design pieces", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "G", "Design groups", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Cn", "Design connections", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Pr", "Design props", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "St", "Design stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Design attributes", GH_ParamAccess.list);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var name = "";
        var variant = "";
        var view = "";
        var description = "";
        var icon = "";
        var image = "";
        var concepts = new List<string>();
        var authors = new List<AuthorId>();
        Location? location = null;
        var unit = "";
        var layers = new List<Layer>();
        var pieces = new List<Piece>();
        var groups = new List<Group>();
        var connections = new List<Connection>();
        var props = new List<Prop>();
        var stats = new List<Stat>();
        var attributes = new List<Attribute>();

        if (!da.GetData(0, ref name)) return;
        da.GetData(1, ref variant);
        da.GetData(2, ref view);
        da.GetData(3, ref description);
        da.GetData(4, ref icon);
        da.GetData(5, ref image);
        da.GetDataList(6, concepts);
        da.GetDataList(7, authors);
        da.GetData(8, ref location);
        da.GetData(9, ref unit);
        da.GetDataList(10, layers);
        da.GetDataList(11, pieces);
        da.GetDataList(12, groups);
        da.GetDataList(13, connections);
        da.GetDataList(14, props);
        da.GetDataList(15, stats);
        da.GetDataList(16, attributes);

        var design = new Design
        {
            Name = name,
            Variant = variant,
            View = view,
            Description = description,
            Icon = icon,
            Image = image,
            Concepts = concepts,
            Authors = authors,
            Location = location,
            Unit = unit,
            Layers = layers,
            Pieces = pieces,
            Groups = groups,
            Connections = connections,
            Props = props,
            Stats = stats,
            Attributes = attributes
        };

        var (isValid, errors) = design.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, name);
        da.SetData(1, variant);
        da.SetData(2, view);
        da.SetData(3, description);
        da.SetData(4, icon);
        da.SetData(5, image);
        da.SetDataList(6, concepts);
        da.SetDataList(7, authors.Select(a => new AuthorIdGoo(a)));
        da.SetData(8, location);
        da.SetData(9, unit);
        da.SetDataList(10, layers.Select(l => new LayerGoo(l)));
        da.SetDataList(11, pieces.Select(p => new PieceGoo(p)));
        da.SetDataList(12, groups.Select(g => new GroupGoo(g)));
        da.SetDataList(13, connections.Select(c => new ConnectionGoo(c)));
        da.SetDataList(14, props.Select(p => new PropGoo(p)));
        da.SetDataList(15, stats.Select(s => new StatGoo(s)));
        da.SetDataList(16, attributes.Select(a => new AttributeGoo(a)));
        da.SetData("Design", new DesignGoo(design));
    }
}

public class SerializeDesignIdComponent : SerializeComponent<DesignIdParam, DesignIdGoo, DesignId>
{
    public SerializeDesignIdComponent() : base("Serialize Design ID", "SDI", "Serialize a design ID to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32078");
    protected override string GetModelTypeName() => "DesignId";
    protected override string GetModelNickname() => "DI";
    protected override string GetModelDescription() => "The design ID to serialize";
}

public class DeserializeDesignIdComponent : DeserializeComponent<DesignIdParam, DesignIdGoo, DesignId>
{
    public DeserializeDesignIdComponent() : base("Deserialize Design ID", "DDI", "Deserialize text to a design ID") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32079");
    protected override string GetModelTypeName() => "DesignId";
    protected override string GetModelNickname() => "DI";
    protected override string GetModelDescription() => "The deserialized design ID";
}

public class SerializeDesignDiffComponent : SerializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public SerializeDesignDiffComponent() : base("Serialize Design Diff", "SDD", "Serialize a design diff to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32080");
    protected override string GetModelTypeName() => "DesignDiff";
    protected override string GetModelNickname() => "DD";
    protected override string GetModelDescription() => "The design diff to serialize";
}

public class DeserializeDesignDiffComponent : DeserializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public DeserializeDesignDiffComponent() : base("Deserialize Design Diff", "DDD", "Deserialize text to a design diff") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32081");
    protected override string GetModelTypeName() => "DesignDiff";
    protected override string GetModelNickname() => "DD";
    protected override string GetModelDescription() => "The deserialized design diff";
}

public class SerializeDesignComponent : SerializeComponent<DesignParam, DesignGoo, Design>
{
    public SerializeDesignComponent() : base("Serialize Design", "SD", "Serialize a design to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32082");
    protected override string GetModelTypeName() => "Design";
    protected override string GetModelNickname() => "D";
    protected override string GetModelDescription() => "The design to serialize";
}

public class DeserializeDesignComponent : DeserializeComponent<DesignParam, DesignGoo, Design>
{
    public DeserializeDesignComponent() : base("Deserialize Design", "DD", "Deserialize text to a design") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32083");
    protected override string GetModelTypeName() => "Design";
    protected override string GetModelNickname() => "D";
    protected override string GetModelDescription() => "The deserialized design";
}

public class KitIdGoo : IdGoo<KitId>
{
    public KitIdGoo() { }
    public KitIdGoo(KitId value) : base(value) { }
    protected override string GetModelTypeName() => "KitId";
    protected override string GetModelDescription() => "A kit id is a key for a kit.";
    protected override IdGoo<KitId> CreateDuplicate() => new KitIdGoo();
    protected override string GetSerializationKey() => "KitId";
}

public class KitIdParam : IdParam<KitIdGoo, KitId>
{
    public KitIdParam() : base("KitId", "KI", "A kit id is a key for a kit.") { }
    protected override Bitmap GetParamIcon() => Resources.kit_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32084");
}

public class KitIdComponent : IdComponent<KitIdParam, KitIdGoo, KitId>
{
    public KitIdComponent() : base("Kit ID", "KI", "Create a kit ID") { }

    protected override Bitmap GetComponentIcon() => Resources.kit_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32085");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Kit name", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Kit name", GH_ParamAccess.item);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var name = "";

        if (!da.GetData(0, ref name)) return;

        var kitId = new KitId { Name = name };
        var (isValid, errors) = kitId.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, name);
        da.SetData("ID", new KitIdGoo(kitId));
    }
}

public class KitDiffGoo : DiffGoo<KitDiff>
{
    public KitDiffGoo() { }
    public KitDiffGoo(KitDiff value) : base(value) { }
    protected override string GetModelTypeName() => "KitDiff";
    protected override string GetModelDescription() => "A kit diff.";
    protected override DiffGoo<KitDiff> CreateDuplicate() => new KitDiffGoo();
    protected override string GetSerializationKey() => "KitDiff";
}

public class KitDiffParam : ModelParam<KitDiffGoo, KitDiff>
{
    public KitDiffParam() : base("KitDiff", "KD", "A kit diff.") { }
    protected override Bitmap GetParamIcon() => Resources.kit_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32086");
}

public class KitDiffComponent : DiffComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public KitDiffComponent() : base("Kit Diff", "KD", "Create a kit diff") { }

    protected override Bitmap GetComponentIcon() => Resources.kit_diff_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32087");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Kit name", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Description", "D", "Kit description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Icon", "I", "Kit icon", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Image", "Im", "Kit image", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Preview", "P", "Kit preview", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Version", "V", "Kit version", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Remote", "R", "Kit remote", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Homepage", "H", "Kit homepage", GH_ParamAccess.item, "");
        pManager.AddTextParameter("License", "L", "Kit license", GH_ParamAccess.item, "");
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Kit attributes", GH_ParamAccess.list);
        for (int i = 0; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Kit name", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "Kit description", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "I", "Kit icon", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im", "Kit image", GH_ParamAccess.item);
        pManager.AddTextParameter("Preview", "P", "Kit preview", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "V", "Kit version", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "R", "Kit remote", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "H", "Kit homepage", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "L", "Kit license", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Kit attributes", GH_ParamAccess.list);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var name = "";
        var description = "";
        var icon = "";
        var image = "";
        var preview = "";
        var version = "";
        var remote = "";
        var homepage = "";
        var license = "";
        var attributes = new List<Attribute>();

        da.GetData(0, ref name);
        da.GetData(1, ref description);
        da.GetData(2, ref icon);
        da.GetData(3, ref image);
        da.GetData(4, ref preview);
        da.GetData(5, ref version);
        da.GetData(6, ref remote);
        da.GetData(7, ref homepage);
        da.GetData(8, ref license);
        da.GetDataList(9, attributes);

        var kitDiff = new KitDiff
        {
            Name = name,
            Description = description,
            Icon = icon,
            Image = image,
            Preview = preview,
            Version = version,
            Remote = remote,
            Homepage = homepage,
            License = license,
            Attributes = attributes
        };

        var (isValid, errors) = kitDiff.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, name);
        da.SetData(1, description);
        da.SetData(2, icon);
        da.SetData(3, image);
        da.SetData(4, preview);
        da.SetData(5, version);
        da.SetData(6, remote);
        da.SetData(7, homepage);
        da.SetData(8, license);
        da.SetDataList(9, attributes.Select(a => new AttributeGoo(a)));
        da.SetData("Diff", new KitDiffGoo(kitDiff));
    }
}

public class KitGoo : ModelGoo<Kit>
{
    public KitGoo() { }
    public KitGoo(Kit value) : base(value) { }
    protected override string GetModelTypeName() => "Kit";
    protected override string GetModelDescription() => "A kit.";
    protected override ModelGoo<Kit> CreateDuplicate() => new KitGoo();
    protected override string GetSerializationKey() => "Kit";
}

public class KitParam : ModelParam<KitGoo, Kit>
{
    public KitParam() : base("Kit", "K", "A kit.") { }
    protected override Bitmap GetParamIcon() => Resources.kit_24x24;
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32088");
}

public class KitComponent : ModelComponent<KitParam, KitGoo, Kit>
{
    public KitComponent() : base("Kit", "K", "Create a kit") { }

    protected override Bitmap GetComponentIcon() => Resources.kit_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32089");

    protected override void RegisterModelInputs(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Kit name", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "V", "Kit version", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Description", "D", "Kit description", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Icon", "I", "Kit icon", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Image", "Im", "Kit image", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Concepts", "C", "Kit concepts", GH_ParamAccess.list);
        pManager.AddTextParameter("Remote", "R", "Kit remote", GH_ParamAccess.item, "");
        pManager.AddTextParameter("Homepage", "H", "Kit homepage", GH_ParamAccess.item, "");
        pManager.AddTextParameter("License", "L", "Kit license", GH_ParamAccess.item, "");
        pManager.AddParameter(new AuthorParam(), "Authors", "Au", "Kit authors", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam(), "Pieces", "P", "Kit pieces", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "G", "Kit groups", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Cn", "Kit connections", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Pr", "Kit props", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "St", "Kit stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Kit attributes", GH_ParamAccess.list);
        pManager.AddTextParameter("Preview", "Pv", "Kit preview", GH_ParamAccess.item, "");
        pManager.AddParameter(new QualityParam(), "Qualities", "Q", "Kit qualities", GH_ParamAccess.list);
        pManager.AddParameter(new TypeParam(), "Types", "T", "Kit types", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Ds", "Kit designs", GH_ParamAccess.list);
        for (int i = 1; i < pManager.ParamCount; i++) pManager[i].Optional = true;
    }

    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Kit name", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "V", "Kit version", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "D", "Kit description", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "I", "Kit icon", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im", "Kit image", GH_ParamAccess.item);
        pManager.AddTextParameter("Concepts", "C", "Kit concepts", GH_ParamAccess.list);
        pManager.AddTextParameter("Remote", "R", "Kit remote", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "H", "Kit homepage", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "L", "Kit license", GH_ParamAccess.item);
        pManager.AddParameter(new AuthorParam(), "Authors", "Au", "Kit authors", GH_ParamAccess.list);
        pManager.AddParameter(new PieceParam(), "Pieces", "P", "Kit pieces", GH_ParamAccess.list);
        pManager.AddParameter(new GroupParam(), "Groups", "G", "Kit groups", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Cn", "Kit connections", GH_ParamAccess.list);
        pManager.AddParameter(new PropParam(), "Props", "Pr", "Kit props", GH_ParamAccess.list);
        pManager.AddParameter(new StatParam(), "Stats", "St", "Kit stats", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At", "Kit attributes", GH_ParamAccess.list);
        pManager.AddTextParameter("Preview", "Pv", "Kit preview", GH_ParamAccess.item);
        pManager.AddParameter(new QualityParam(), "Qualities", "Q", "Kit qualities", GH_ParamAccess.list);
        pManager.AddParameter(new TypeParam(), "Types", "T", "Kit types", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Ds", "Kit designs", GH_ParamAccess.list);
    }

    protected override void SolveModelInstance(IGH_DataAccess da)
    {
        var name = "";
        var version = "";
        var description = "";
        var icon = "";
        var image = "";
        var concepts = new List<string>();
        var remote = "";
        var homepage = "";
        var license = "";
        var authors = new List<Author>();
        var pieces = new List<Piece>();
        var groups = new List<Group>();
        var connections = new List<Connection>();
        var props = new List<Prop>();
        var stats = new List<Stat>();
        var attributes = new List<Attribute>();
        var preview = "";
        var qualities = new List<Quality>();
        var types = new List<Type>();
        var designs = new List<Design>();

        if (!da.GetData(0, ref name)) return;
        da.GetData(1, ref version);
        da.GetData(2, ref description);
        da.GetData(3, ref icon);
        da.GetData(4, ref image);
        da.GetDataList(5, concepts);
        da.GetData(6, ref remote);
        da.GetData(7, ref homepage);
        da.GetData(8, ref license);
        da.GetDataList(9, authors);
        da.GetDataList(10, pieces);
        da.GetDataList(11, groups);
        da.GetDataList(12, connections);
        da.GetDataList(13, props);
        da.GetDataList(14, stats);
        da.GetDataList(15, attributes);
        da.GetData(16, ref preview);
        da.GetDataList(17, qualities);
        da.GetDataList(18, types);
        da.GetDataList(19, designs);

        var kit = new Kit
        {
            Name = name,
            Version = version,
            Description = description,
            Icon = icon,
            Image = image,
            Concepts = concepts,
            Remote = remote,
            Homepage = homepage,
            License = license,
            Authors = authors,
            Pieces = pieces,
            Groups = groups,
            Connections = connections,
            Props = props,
            Stats = stats,
            Attributes = attributes,
            Preview = preview,
            Qualities = qualities,
            Types = types,
            Designs = designs
        };

        var (isValid, errors) = kit.Validate();
        if (!isValid) foreach (var error in errors) AddRuntimeMessage(GH_RuntimeMessageLevel.Error, error);

        da.SetData(0, name);
        da.SetData(1, version);
        da.SetData(2, description);
        da.SetData(3, icon);
        da.SetData(4, image);
        da.SetDataList(5, concepts);
        da.SetData(6, remote);
        da.SetData(7, homepage);
        da.SetData(8, license);
        da.SetDataList(9, authors.Select(a => new AuthorGoo(a)));
        da.SetDataList(10, pieces.Select(p => new PieceGoo(p)));
        da.SetDataList(11, groups.Select(g => new GroupGoo(g)));
        da.SetDataList(12, connections.Select(c => new ConnectionGoo(c)));
        da.SetDataList(13, props.Select(p => new PropGoo(p)));
        da.SetDataList(14, stats.Select(s => new StatGoo(s)));
        da.SetDataList(15, attributes.Select(a => new AttributeGoo(a)));
        da.SetData(16, preview);
        da.SetDataList(17, qualities.Select(q => new QualityGoo(q)));
        da.SetDataList(18, types.Select(t => new TypeGoo(t)));
        da.SetDataList(19, designs.Select(d => new DesignGoo(d)));
        da.SetData("Kit", new KitGoo(kit));
    }
}

public class SerializeKitIdComponent : SerializeComponent<KitIdParam, KitIdGoo, KitId>
{
    public SerializeKitIdComponent() : base("Serialize Kit ID", "SKI", "Serialize a kit ID to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32090");
    protected override string GetModelTypeName() => "KitId";
    protected override string GetModelNickname() => "KI";
    protected override string GetModelDescription() => "The kit ID to serialize";
}

public class DeserializeKitIdComponent : DeserializeComponent<KitIdParam, KitIdGoo, KitId>
{
    public DeserializeKitIdComponent() : base("Deserialize Kit ID", "DKI", "Deserialize text to a kit ID") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32091");
    protected override string GetModelTypeName() => "KitId";
    protected override string GetModelNickname() => "KI";
    protected override string GetModelDescription() => "The deserialized kit ID";
}

public class SerializeKitDiffComponent : SerializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public SerializeKitDiffComponent() : base("Serialize Kit Diff", "SKD", "Serialize a kit diff to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32092");
    protected override string GetModelTypeName() => "KitDiff";
    protected override string GetModelNickname() => "KD";
    protected override string GetModelDescription() => "The kit diff to serialize";
}

public class DeserializeKitDiffComponent : DeserializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public DeserializeKitDiffComponent() : base("Deserialize Kit Diff", "DKD", "Deserialize text to a kit diff") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32093");
    protected override string GetModelTypeName() => "KitDiff";
    protected override string GetModelNickname() => "KD";
    protected override string GetModelDescription() => "The deserialized kit diff";
}

public class SerializeKitComponent : SerializeComponent<KitParam, KitGoo, Kit>
{
    public SerializeKitComponent() : base("Serialize Kit", "SK", "Serialize a kit to text") { }

    protected override Bitmap Icon => Resources.serialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32094");
    protected override string GetModelTypeName() => "Kit";
    protected override string GetModelNickname() => "K";
    protected override string GetModelDescription() => "The kit to serialize";
}

public class DeserializeKitComponent : DeserializeComponent<KitParam, KitGoo, Kit>
{
    public DeserializeKitComponent() : base("Deserialize Kit", "DK", "Deserialize text to a kit") { }

    protected override Bitmap Icon => Resources.deserialize_24x24;
    protected override Guid GetComponentGuid() => new("AC6E381C-23EE-4A81-BE0F-3523AEE32095");
    protected override string GetModelTypeName() => "Kit";
    protected override string GetModelNickname() => "K";
    protected override string GetModelDescription() => "The deserialized kit";
}

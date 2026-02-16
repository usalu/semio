#region 🔖Header

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs](semiorepo://file/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Main Grasshopper plugin providing domain components for Rhino.

#endregion 🔖Header

#region 🔖Imports

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖imports](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/IMPORTS)
// Callers MUST import all required namespaces listed here.
using System.Diagnostics;
using System.Drawing;
using System.Linq;
using System.Reflection;
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

#endregion 🔖Imports

#region 🔖Namespace

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖namespace](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/NAMESPACE)
// Implementations MUST reside in this namespace.
namespace Semio.Grasshopper;
#endregion 🔖Namespace

#region 🔖Constants

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖constants](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/CONSTANTS)
// Consumers MUST use these shared constants for configuration.

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
        Instances.ComponentServer.AddCategoryIcon("semio", Resources.semio_24x24);
        Instances.ComponentServer.AddCategorySymbolName("semio", 'S');
        return GH_LoadingInstruction.Proceed;
    }
}

#endregion 🔖Constants

#region 🔖Utility

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖utility](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/UTILITY)
// Callers MUST use these utility functions for encoding and serialization.

public static class Utility
{
    public static bool IsValidLengthUnitSystem(string unit) => new[] { "nm", "mm", "cm", "dm", "m", "km", "µin", "in", "ft", "yd" }.Contains(unit);
    public static string LengthUnitSystemToAbbreviation(UnitSystem unitSystem)
    {
        var unit = unitSystem switch
        {
            UnitSystem.Nanometers => "nm",
            UnitSystem.Millimeters => "mm",
            UnitSystem.Centimeters => "cm",
            UnitSystem.Decimeters => "dm",
            UnitSystem.Meters => "m",
            UnitSystem.Kilometers => "km",
            UnitSystem.Microinches => "µin",
            UnitSystem.Inches => "in",
            UnitSystem.Feet => "ft",
            UnitSystem.Yards => "yd",
            _ => "unsupported length unit system"
        };
        if (IsValidLengthUnitSystem(unit) == false)
            throw new ArgumentException("Invalid length unit system", nameof(unitSystem));
        return unit;
    }

    public static Rhino.Geometry.Plane GetPlaneFromYAxis(Vector3d yAxis, float theta, Point3d origin)
    {
        var thetaRad = RhinoMath.ToRadians(theta);
        var orientation = Transform.Rotation(Vector3d.YAxis, yAxis, Point3d.Origin);
        var rotation = Transform.Rotation(thetaRad, yAxis, Point3d.Origin);
        var xAxis = Vector3d.XAxis;
        xAxis.Transform(rotation * orientation);
        return new Rhino.Geometry.Plane(origin, xAxis, yAxis);
    }

    public static Plane ComputeChildPlane(Plane parentPlane, Point parentPoint, Vector parentDirection,
        Point childPoint, Vector childDirection, float gap, float shift, float raise, float rotation, float turn,
        float tilt)
    {
        var parentPointR = new Vector3d(parentPoint.Convert());
        var parentDirectionR = parentDirection.Convert();
        var revertedChildPointR = new Vector3d(childPoint.Convert());
        revertedChildPointR.Reverse();

        var reverseChildDirectionR = childDirection.Convert();
        reverseChildDirectionR.Reverse();
        var rotationRad = RhinoMath.ToRadians(rotation);
        var turnRad = RhinoMath.ToRadians(turn);
        var tiltRad = RhinoMath.ToRadians(tilt);

        var areDirectionsSame = parentDirectionR.IsParallelTo(childDirection.Convert(), Semio.Constants.Tolerance) == 1;

        Transform directionT;
        if (areDirectionsSame)
        {

            if (Math.Abs(parentDirectionR.Z) < Semio.Constants.Tolerance)
                directionT = Transform.Rotation(RhinoMath.ToRadians(180), Vector3d.ZAxis, new Point3d());
            else
                directionT = Transform.Rotation(RhinoMath.ToRadians(180),
                    Vector3d.CrossProduct(Vector3d.ZAxis, parentDirectionR), new Point3d());
        }
        else
        {
            directionT = Transform.Rotation(reverseChildDirectionR, parentDirectionR, new Point3d());
        }

        var rotationAxis = Vector3d.YAxis;
        var turnAxis = Vector3d.ZAxis;
        var tiltAxis = Vector3d.XAxis;
        var gapDirection = Vector3d.YAxis;
        var shiftDirection = Vector3d.XAxis;
        var raiseDirection = Vector3d.ZAxis;

        var parentRotation = Transform.Rotation(Vector3d.YAxis, parentDirectionR, new Point3d());

        gapDirection.Transform(parentRotation);
        shiftDirection.Transform(parentRotation);
        raiseDirection.Transform(parentRotation);
        turnAxis.Transform(parentRotation);
        tiltAxis.Transform(parentRotation);

        var orientationT = directionT;

        var rotateT = Transform.Rotation(-rotationRad, parentDirectionR, new Point3d());
        orientationT = rotateT * orientationT;
        turnAxis.Transform(rotateT);
        tiltAxis.Transform(rotateT);

        var turnT = Transform.Rotation(turnRad, turnAxis, new Point3d());
        orientationT = turnT * orientationT;

        var tiltT = Transform.Rotation(tiltRad, tiltAxis, new Point3d());
        orientationT = tiltT * orientationT;

        var centerChild = Transform.Translation(revertedChildPointR);
        var moveToParent = Transform.Translation(parentPointR);
        var transform = orientationT * centerChild;

        var gapTransform = Transform.Translation(gapDirection * gap);
        var shiftTransform = Transform.Translation(shiftDirection * shift);
        var raiseTransform = Transform.Translation(raiseDirection * raise);
        var translation = gapTransform * shiftTransform;
        translation = raiseTransform * translation;

        transform = translation * transform;

        transform = moveToParent * transform;
        var childPlaneR = Rhino.Geometry.Plane.WorldXY;
        childPlaneR.Transform(transform);

        var parentPlaneR = parentPlane.Convert();
        var parentPlaneT = Transform.PlaneToPlane(Rhino.Geometry.Plane.WorldXY, parentPlaneR);
        childPlaneR.Transform(parentPlaneT);

        return childPlaneR.Convert();
    }
}

#endregion 🔖Utility

#region 🔖Converters

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖converters](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/CONVERTERS)
// Implementations MUST convert between semio and Grasshopper data types.

public static class RhinoConverter
{
    public static object Convert(this object value) => value;
    public static string Convert(this string value) => value;
    public static int Convert(this int value) => value;
    public static float Convert(this double value) => (float)value;
    public static Point3d Convert(this Point point) => new Point3d(point.X, point.Y, point.Z);
    public static Point Convert(this Point3d point) => new Point { X = (float)point.X, Y = (float)point.Y, Z = (float)point.Z };
    public static Vector3d Convert(this Vector vector) => new Vector3d(vector.X, vector.Y, vector.Z);
    public static Vector Convert(this Vector3d vector) => new Vector { X = (float)vector.X, Y = (float)vector.Y, Z = (float)vector.Z };
    public static Rhino.Geometry.Plane Convert(this Plane plane) => new(
        new Point3d(plane.Origin.X, plane.Origin.Y, plane.Origin.Z),
        new Vector3d(plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z),
        new Vector3d(plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z));
    public static Plane Convert(this Rhino.Geometry.Plane plane) => new()
    {
        Origin = new Point { X = (float)plane.OriginX, Y = (float)plane.OriginY, Z = (float)plane.OriginZ },
        XAxis = new Vector { X = (float)plane.XAxis.X, Y = (float)plane.XAxis.Y, Z = (float)plane.XAxis.Z },
        YAxis = new Vector { X = (float)plane.YAxis.X, Y = (float)plane.YAxis.Y, Z = (float)plane.YAxis.Z }
    };
}

#endregion 🔖Converters

#region 🔖Bases

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES)
// Implementations MUST extend these abstract base classes for Goo, Param, and Component.

/// Generic Grasshopper data wrapper for semio entity types.
/// Implementations MUST override CastFrom and CastTo for type conversion.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️goo](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/GOO)
public abstract class Goo<TEntity> : GH_Goo<TEntity> where TEntity : Entity<TEntity>, new()
{
    public Goo() { Value = new TEntity(); }
    public Goo(TEntity value) { Value = value; }
    public override bool IsValid => true;
    public override string TypeName => typeof(TEntity).Name;
    public override string TypeDescription => typeof(TEntity).Name;
    public override IGH_Goo Duplicate()
    {
        var duplicate = (Goo<TEntity>)(Activator.CreateInstance(GetType()) ?? throw new InvalidOperationException($"Could not create instance of {GetType()}"));
        duplicate.Value = Value.DeepClone() ?? throw new InvalidOperationException($"Could not clone {typeof(TEntity).Name}");
        return duplicate;
    }
    public override string ToString() => Value.ToString();
    public override bool Write(GH_IWriter writer)
    {
        writer.SetString(typeof(TEntity).Name, Value.Serialize());
        return base.Write(writer);
    }
    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString(typeof(TEntity).Name).Deserialize<TEntity>() ?? throw new InvalidOperationException($"Could not deserialize {typeof(TEntity).Name}");
        return base.Read(reader);
    }
    internal virtual bool CustomCastTo<Q>(ref Q target) => false;
    internal virtual bool CustomCastFrom(object source) => false;
    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TEntity)))
        {
            target = (Q)(object)this;
            return true;
        }
        return CustomCastTo(ref target);
    }

    public override bool CastFrom(object source)
    {
        if (source is null) return false;
        if (source is TEntity model)
        {
            Value = model;
            return true;
        }
        return CustomCastFrom(source);
    }
}

/// Generic Grasshopper parameter for semio entity types.
/// Implementations MUST provide component exposure and icon metadata.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️param](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/PARAM)
public abstract class Param<TGoo, TModel> : GH_PersistentParam<TGoo> where TGoo : Goo<TModel> where TModel : Entity<TModel>, new()
{
    protected abstract string ModelName { get; }
    protected abstract string ModelNickname { get; }
    protected abstract string ModelDescription { get; }
    protected abstract string IconResourceName { get; }
    protected Param() : base("", "", "", Constants.Category, "Params") { }
    public override string Name => ModelName;
    public override string NickName => ModelNickname;
    public override string Description => ModelDescription;
    protected override Bitmap Icon => (Bitmap)(Resources.ResourceManager.GetObject(IconResourceName) ?? throw new InvalidOperationException($"Resource {IconResourceName} not found"));

    protected override GH_GetterResult Prompt_Singular(ref TGoo value) => throw new NotImplementedException();
    protected override GH_GetterResult Prompt_Plural(ref List<TGoo> values) => throw new NotImplementedException();
}

/// Generic Grasshopper data wrapper for enum values.
/// Implementations MUST convert between string names and enum values.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️enumgoo](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENUM-GOO)
public abstract class EnumGoo<TEnum> : GH_Goo<TEnum> where TEnum : struct, Enum
{
    public EnumGoo() { }
    public EnumGoo(TEnum value) => Value = value;
    public override bool IsValid => true;
    public override IGH_Goo Duplicate() => (IGH_Goo)(Activator.CreateInstance(GetType(), Value) ?? throw new InvalidOperationException($"Could not create instance of {GetType()}"));
    public override bool CastFrom(object source)
    {
        if (source is TEnum enumValue) { Value = enumValue; return true; }
        if (source is string str && Enum.TryParse<TEnum>(str, true, out var parsed)) { Value = parsed; return true; }
        if (source is int intVal && Enum.IsDefined(typeof(TEnum), intVal)) { Value = (TEnum)Enum.ToObject(typeof(TEnum), intVal); return true; }
        return false;
    }
    public override bool CastTo<U>(ref U target)
    {
        if (typeof(U) == typeof(TEnum)) { target = (U)(object)Value; return true; }
        if (typeof(U) == typeof(string)) { target = (U)(object)Value.ToString(); return true; }
        if (typeof(U) == typeof(int)) { target = (U)(object)Convert.ToInt32(Value); return true; }
        return false;
    }
    public override string ToString() => Value.ToString();
    public override string TypeName => typeof(TEnum).Name;
    public override string TypeDescription => typeof(TEnum).Name;
}

/// Generic Grasshopper parameter for enum values.
/// Implementations MUST restrict input to valid enum members.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️enumparam](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENUM-PARAM)
public abstract class EnumParam<TEnumGoo, TEnum> : GH_Param<TEnumGoo>
    where TEnumGoo : EnumGoo<TEnum>, new()
    where TEnum : struct, Enum
{
    protected EnumParam(Guid guid) : base(typeof(TEnum).Name, typeof(TEnum).Name, typeof(TEnum).Name, "Semio", "Param", GH_ParamAccess.item)
    {
        ComponentGuid = guid;
    }
    public override Guid ComponentGuid { get; }
}
public abstract class Component : GH_Component
{
    public Component(string name, string nickname, string description, string subcategory) : base(
        name, nickname, description, Constants.Category, subcategory)
    { }
}

/// Abstract Grasshopper component that passes input through transformation.
/// Implementations MUST transform input data and output the result.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️passthroughcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/PASSTHROUGH-COMPONENT)
public abstract class PassthroughComponent<TParam, TGoo, TModel> : Component
    where TParam : Param<TGoo, TModel>, new() where TGoo : Goo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected abstract string ModelName { get; }
    protected abstract string ModelNickname { get; }
    protected abstract string ModelDescription { get; }
    protected abstract string IconResourceName { get; }

    protected PassthroughComponent() : base("", "", "", "Data") { }

    public override string Name => $"Passthrough {ModelName}";
    public override string NickName => $"~{ModelNickname}";
    public override string Description => ModelDescription;

    protected override Bitmap Icon =>
        (Bitmap)Resources.ResourceManager.GetObject($"{IconResourceName.Replace("_24x24", "")}_modify_24x24");

    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected virtual void RegisterModelInputParams(GH_InputParamManager pManager) { }
    protected virtual void RegisterModelOutputParams(GH_OutputParamManager pManager) { }
    protected virtual void GetModelData(IGH_DataAccess DA, TModel model) { }
    protected virtual void SetModelData(IGH_DataAccess DA, TModel model) { }

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), ModelName, ModelNickname + "?",
            $"The optional {ModelName.ToLower()} to deconstruct or modify.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Validate", "Vd?",
            $"Whether the {ModelName.ToLower()} should be validated.", GH_ParamAccess.item);
        RegisterModelInputParams(pManager);
        for (var i = 0; i < pManager.ParamCount; i++)
            pManager[i].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), ModelName, ModelNickname,
            $"The constructed or modified {ModelName.ToLower()}.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Valid", "Vd?",
            $"True if the {ModelName.ToLower()} is valid. Null if no validation was performed.", GH_ParamAccess.item);
        RegisterModelOutputParams(pManager);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var modelGoo = new TGoo();
        var validate = false;
        if (DA.GetData(0, ref modelGoo))
            modelGoo = (TGoo)modelGoo.Duplicate();
        DA.GetData(1, ref validate);

        GetModelData(DA, modelGoo.Value);
        modelGoo.Value = ProcessModel(modelGoo.Value);

        if (validate)
        {
            var (isValid, errors) = modelGoo.Value.Validate();
            foreach (var error in errors)
                AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, error);
            DA.SetData(1, isValid);
        }

        DA.SetData(0, modelGoo.Duplicate());
        SetModelData(DA, modelGoo.Value);
    }

    protected virtual TModel ProcessModel(TModel model) => model;
}

/// Generic Grasshopper data wrapper for entity ID types.
/// Implementations MUST wrap entity ID types for Grasshopper data flow.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️idgoo](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ID-GOO)
public abstract class IdGoo<TModel> : Goo<TModel> where TModel : Entity<TModel>, new()
{
    public IdGoo() : base() { }
    public IdGoo(TModel value) : base(value) { }
}

/// Generic Grasshopper parameter for entity ID types.
/// Implementations MUST provide type-safe parameter access for IDs.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️idparam](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ID-PARAM)
public abstract class IdParam<TGoo, TModel> : Param<TGoo, TModel> where TGoo : IdGoo<TModel> where TModel : Entity<TModel>, new()
{
    protected IdParam() : base() { }
    protected abstract string IdIconResourceName { get; }
    protected override Bitmap Icon => (Bitmap)(Resources.ResourceManager.GetObject(IdIconResourceName) ?? Resources.ResourceManager.GetObject(IconResourceName));
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

/// Abstract Grasshopper component for constructing entity IDs.
/// Implementations MUST register input parameters matching ID fields.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️idcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ID-COMPONENT)
public abstract class IdComponent<TParam, TGoo, TModel> : PassthroughComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected IdComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

/// Generic Grasshopper data wrapper for entity diff types.
/// Implementations MUST wrap entity diff types for Grasshopper data flow.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️diffgoo](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/DIFF-GOO)
public abstract class DiffGoo<TModel> : Goo<TModel> where TModel : Entity<TModel>, new()
{
    public DiffGoo() : base() { }
    public DiffGoo(TModel value) : base(value) { }
}

/// Generic Grasshopper parameter for entity diff types.
/// Implementations MUST provide type-safe parameter access for diffs.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️diffparam](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/DIFF-PARAM)
public abstract class DiffParam<TGoo, TModel> : Param<TGoo, TModel> where TGoo : DiffGoo<TModel> where TModel : Entity<TModel>, new()
{
    protected DiffParam() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}

/// Abstract Grasshopper component for constructing entity diffs.
/// Implementations MUST register input parameters matching diff fields.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️diffcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/DIFF-COMPONENT)
public abstract class DiffComponent<TParam, TGoo, TModel> : PassthroughComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected DiffComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}
/// Abstract Grasshopper component for serializing entities to JSON.
/// Implementations MUST convert entities to valid JSON strings.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️serializecomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/SERIALIZE-COMPONENT)
public abstract class SerializeComponent<TParam, TGoo, TModel> : ScriptingComponent
    where TParam : Param<TGoo, TModel>, new() where TGoo : Goo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected virtual string ModelName => typeof(TModel).Name;
    protected virtual string ModelNickname => typeof(TModel).Name.Substring(0, 3);

    protected SerializeComponent() : base("", "", "") { }

    public override string Name => $"Serialize {ModelName}";
    public override string NickName => $">{ModelNickname}";
    public override string Description => $"Serialize a {ModelName.ToLower()}.";
    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{typeof(TModel).Name.ToLower()}_serialize_24x24");
    public override GH_Exposure Exposure => GH_Exposure.secondary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), ModelName, ModelNickname, $"The {ModelName.ToLower()} to serialize.", GH_ParamAccess.item);
        pManager.AddTextParameter("Indent", "In?", $"The optional indent unit for the serialized {ModelName.ToLower()}. Empty text for no indent or spaces or tabs", GH_ParamAccess.item, "");
        pManager[1].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", $"Text of serialized {ModelName}.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var goo = new TGoo();
        var indent = "";
        DA.GetData(0, ref goo);
        DA.GetData(1, ref indent);
        var text = goo.Value.Serialize(indent);
        DA.SetData(0, text);
    }
}

/// Abstract Grasshopper component for deserializing entities from JSON.
/// Implementations MUST parse JSON strings into entity instances.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️deserializecomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/DESERIALIZE-COMPONENT)
public abstract class DeserializeComponent<TParam, TGoo, TModel> : ScriptingComponent
    where TParam : Param<TGoo, TModel>, new() where TGoo : Goo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected virtual string ModelName => typeof(TModel).Name;
    protected virtual string ModelNickname => typeof(TModel).Name.Substring(0, 3);

    protected DeserializeComponent() : base("", "", "") { }

    public override string Name => $"Deserialize {ModelName}";
    public override string NickName => $"<{ModelNickname}";
    public override string Description => $"Deserialize a {ModelName.ToLower()}.";
    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{typeof(TModel).Name.ToLower()}_deserialize_24x24");
    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", $"Text of serialized {ModelName}.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TParam(), ModelName, ModelNickname, $"Deserialized {ModelName}.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        DA.GetData(0, ref text);
        var value = text.Deserialize<TModel>() ?? throw new InvalidOperationException($"Could not deserialize {typeof(TModel).Name}");
        var goo = new TGoo();
        goo.Value = value;
        DA.SetData(0, goo);
    }
}

/// Abstract Grasshopper component for serializing diffs to JSON.
/// Implementations MUST convert diffs to valid JSON strings.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️serializediffcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/SERIALIZE-DIFF-COMPONENT)
public abstract class SerializeDiffComponent<TParam, TGoo, TModel> : SerializeComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected SerializeDiffComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{GetEntityName()}_diff_serialize_24x24");

    protected virtual string GetEntityName()
    {
        var typeName = typeof(TModel).Name.ToLower();
        return typeName.EndsWith("diff") ? typeName.Substring(0, typeName.Length - 4) :
               typeName.EndsWith("sdiff") ? typeName.Substring(0, typeName.Length - 5) : typeName;
    }
}

/// Abstract Grasshopper component for deserializing diffs from JSON.
/// Implementations MUST parse JSON strings into diff instances.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️deserializediffcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/DESERIALIZE-DIFF-COMPONENT)
public abstract class DeserializeDiffComponent<TParam, TGoo, TModel> : DeserializeComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected DeserializeDiffComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{GetEntityName()}_diff_deserialize_24x24");

    protected virtual string GetEntityName()
    {
        var typeName = typeof(TModel).Name.ToLower();
        return typeName.EndsWith("diff") ? typeName.Substring(0, typeName.Length - 4) :
               typeName.EndsWith("sdiff") ? typeName.Substring(0, typeName.Length - 5) : typeName;
    }
}

/// Abstract Grasshopper component for serializing entity IDs to JSON.
/// Implementations MUST convert entity IDs to valid JSON strings.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️serializeidcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/SERIALIZE-ID-COMPONENT)
public abstract class SerializeIdComponent<TParam, TGoo, TModel> : SerializeComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected SerializeIdComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{GetEntityName()}_id_serialize_24x24");

    protected virtual string GetEntityName()
    {
        var typeName = typeof(TModel).Name.ToLower();
        return typeName.EndsWith("id") ? typeName.Substring(0, typeName.Length - 2) : typeName;
    }
}

/// Abstract Grasshopper component for deserializing entity IDs from JSON.
/// Implementations MUST parse JSON strings into entity ID instances.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️deserializeidcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/DESERIALIZE-ID-COMPONENT)
public abstract class DeserializeIdComponent<TParam, TGoo, TModel> : DeserializeComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Entity<TModel>, new()
{
    protected DeserializeIdComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{GetEntityName()}_id_deserialize_24x24");

    protected virtual string GetEntityName()
    {
        var typeName = typeof(TModel).Name.ToLower();
        return typeName.EndsWith("id") ? typeName.Substring(0, typeName.Length - 2) : typeName;
    }
}

/// Generic Grasshopper data wrapper with built-in entity validation.
/// Implementations MUST validate entities before exposing them downstream.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitygoo](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-GOO)
public abstract class EntityGoo<TEntity, TEntityDiff, TEntityId> : Goo<TEntity>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    public EntityGoo() : base() { }
    public EntityGoo(TEntity value) : base(value) { }
}

/// Generic Grasshopper parameter with entity validation support.
/// Implementations MUST enforce entity validation on parameter access.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entityparam](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-PARAM)
public abstract class EntityParam<TGoo, TEntity, TEntityDiff, TEntityId> : Param<TGoo, TEntity>
    where TGoo : EntityGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityParam() : base() { }
}

/// Abstract Grasshopper component for constructing validated entities.
/// Implementations MUST validate constructed entities before output.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitycomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-COMPONENT)
public abstract class EntityComponent<TParam, TGoo, TEntity, TEntityDiff, TEntityId> : PassthroughComponent<TParam, TGoo, TEntity>
    where TParam : EntityParam<TGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TGoo : EntityGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityComponent() : base() { }
}

/// Generic Grasshopper data wrapper for validated entity ID types.
/// Implementations MUST validate entity IDs before exposing them downstream.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entityidgoo](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-ID-GOO)
public abstract class EntityIdGoo<TEntity, TEntityDiff, TEntityId> : IdGoo<TEntityId>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    public EntityIdGoo() : base() { }
    public EntityIdGoo(TEntityId value) : base(value) { }
}

/// Generic Grasshopper parameter for validated entity ID types.
/// Implementations MUST enforce entity ID validation on parameter access.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entityidparam](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-ID-PARAM)
public abstract class EntityIdParam<TIdGoo, TEntity, TEntityDiff, TEntityId> : IdParam<TIdGoo, TEntityId>
    where TIdGoo : EntityIdGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityIdParam() : base() { }
}

/// Abstract Grasshopper component for constructing validated entity IDs.
/// Implementations MUST validate constructed entity IDs before output.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entityidcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-ID-COMPONENT)
public abstract class EntityIdComponent<TIdParam, TIdGoo, TEntity, TEntityDiff, TEntityId> : IdComponent<TIdParam, TIdGoo, TEntityId>
    where TIdParam : EntityIdParam<TIdGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TIdGoo : EntityIdGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityIdComponent() : base() { }
}

/// Generic Grasshopper data wrapper for validated entity diff types.
/// Implementations MUST validate entity diffs before exposing them downstream.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitydiffgoo](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-DIFF-GOO)
public abstract class EntityDiffGoo<TEntity, TEntityDiff, TEntityId> : DiffGoo<TEntityDiff>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    public EntityDiffGoo() : base() { }
    public EntityDiffGoo(TEntityDiff value) : base(value) { }
}

/// Generic Grasshopper parameter for validated entity diff types.
/// Implementations MUST enforce entity diff validation on parameter access.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitydiffparam](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-DIFF-PARAM)
public abstract class EntityDiffParam<TDiffGoo, TEntity, TEntityDiff, TEntityId> : DiffParam<TDiffGoo, TEntityDiff>
    where TDiffGoo : EntityDiffGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityDiffParam() : base() { }
}

/// Abstract Grasshopper component for constructing validated entity diffs.
/// Implementations MUST validate constructed entity diffs before output.
/// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖bases🛠️entitydiffcomponent](semiorepo://definition/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BASES/ENTITY-DIFF-COMPONENT)
public abstract class EntityDiffComponent<TDiffParam, TDiffGoo, TEntity, TEntityDiff, TEntityId> : DiffComponent<TDiffParam, TDiffGoo, TEntityDiff>
    where TDiffParam : EntityDiffParam<TDiffGoo, TEntity, TEntityDiff, TEntityId>, new()
    where TDiffGoo : EntityDiffGoo<TEntity, TEntityDiff, TEntityId>, new()
    where TEntity : Entity<TEntity>, new()
    where TEntityDiff : Entity<TEntityDiff>, new()
    where TEntityId : Entity<TEntityId>, new()
{
    protected EntityDiffComponent() : base() { }
}

#endregion 🔖Bases

#region 🔖Attribute

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖attribute](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/ATTRIBUTE)
// Implementations MUST provide key-value metadata for annotating entities.

public class AttributeGoo : Goo<Attribute>
{
    public AttributeGoo() { }
    public AttributeGoo(Attribute value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(AttributeIdGoo)))
        {
            target = (Q)(object)new AttributeIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(AttributeDiffGoo)))
        {
            target = (Q)(object)new AttributeDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is AttributeIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is AttributeDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Attribute { Key = str };
            return true;
        }
        return false;
    }
}

public class AttributeParam : Param<AttributeGoo, Attribute>
{
    protected override string ModelName => "Attribute";
    protected override string ModelNickname => "Atr";
    protected override string ModelDescription => "Key-value metadata";
    protected override string IconResourceName => "attribute_24x24";
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B94");
}

public class AttributeComponent : PassthroughComponent<AttributeParam, AttributeGoo, Attribute>
{
    public override Guid ComponentGuid => new("51146B05-ACEB-4810-AD75-10AC3E029D39");
    protected override string ModelName => "Attribute";
    protected override string ModelNickname => "Atr";
    protected override string ModelDescription => "Construct, deconstruct or modify an attribute.";
    protected override string IconResourceName => "attribute_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke", "The key of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl?", "The optional value of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition of the attribute.", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke", "The key of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl?", "The optional value of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition of the attribute.", GH_ParamAccess.item);
    }

    protected override void GetModelData(IGH_DataAccess DA, Attribute model)
    {
        var guid = ""; var key = ""; var value = ""; var definition = "";
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref key)) model.Key = key;
        if (DA.GetData(4, ref value)) model.Value = value;
        if (DA.GetData(5, ref definition)) model.Definition = definition;
    }

    protected override void SetModelData(IGH_DataAccess DA, Attribute model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Key);
        DA.SetData(4, model.Value);
        DA.SetData(5, model.Definition);
    }
}

public class SerializeAttributeComponent : SerializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8974-8588BCA75250");
    protected override string ModelName => "Attribute";
    protected override string ModelNickname => "Atr";
}

public class DeserializeAttributeComponent : DeserializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8975-8588BCA75250");
    protected override string ModelName => "Attribute";
    protected override string ModelNickname => "Atr";
}

public class AttributeIdGoo : IdGoo<AttributeId>
{
    public AttributeIdGoo() { }
    public AttributeIdGoo(AttributeId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(AttributeGoo)))
        {
            target = (Q)(object)new AttributeGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(AttributeDiffGoo)))
        {
            target = (Q)(object)new AttributeDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is AttributeDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is AttributeGoo attrGoo)
        {
            Value = attrGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new AttributeId { Guid = str };
            return true;
        }
        return false;
    }
}

public class AttributeIdParam : IdParam<AttributeIdGoo, AttributeId>
{
    protected override string ModelName => "AttributeId";
    protected override string ModelNickname => "AId";
    protected override string ModelDescription => "Attribute identifier";
    protected override string IconResourceName => "attribute_24x24";
    protected override string IdIconResourceName => "attributeid_24x24";
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B93");
}

public class AttributeDiffGoo : DiffGoo<AttributeDiff>
{
    public AttributeDiffGoo() { }
    public AttributeDiffGoo(AttributeDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(AttributeIdGoo)))
        {
            target = (Q)(object)new AttributeIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(AttributeGoo)))
        {
            target = (Q)(object)new AttributeGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is AttributeIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is AttributeGoo attrGoo)
        {
            Value = attrGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<AttributeDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class AttributeDiffParam : DiffParam<AttributeDiffGoo, AttributeDiff>
{
    protected override string ModelName => "AttributeDiff";
    protected override string ModelNickname => "ADf";
    protected override string ModelDescription => "Attribute differences";
    protected override string IconResourceName => "attributediff_24x24";
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B95");
}

public class AttributeDiffComponent : DiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B96");
    protected override string ModelName => "AttributeDiff";
    protected override string ModelNickname => "ADf";
    protected override string ModelDescription => "Construct, deconstruct or modify an attribute diff.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd?", "The optional guid of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke?", "The optional key of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl?", "The optional value of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition of the attribute.", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd?", "The optional guid of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Key", "Ke?", "The optional key of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Vl?", "The optional value of the attribute.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "The optional definition of the attribute.", GH_ParamAccess.item);
    }

    protected override void GetModelData(IGH_DataAccess DA, AttributeDiff model)
    {
        string guid = null, key = "", value = "", definition = "";
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref key)) model.Key = key;
        if (DA.GetData(4, ref value)) model.Value = value;
        if (DA.GetData(5, ref definition)) model.Definition = definition;
    }

    protected override void SetModelData(IGH_DataAccess DA, AttributeDiff model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Key);
        DA.SetData(4, model.Value);
        DA.SetData(5, model.Definition);
    }
}

public class SerializeAttributeDiffComponent : SerializeComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public SerializeAttributeDiffComponent() { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B97");
}

public class DeserializeAttributeDiffComponent : DeserializeComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public DeserializeAttributeDiffComponent() { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B98");
}

#endregion 🔖Attribute

#region 🔖Coord

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖coord](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/COORD)
// Implementations MUST share X, Y, Z coordinate fields for spatial types.

public class CoordGoo : Goo<Coord>
{
    public CoordGoo() { }
    public CoordGoo(Coord value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.U, Value.V, 0));
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        Point3d point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new Coord { U = (float)point.X, V = (float)point.Y };
            return true;
        }
        return false;
    }
}

public class CoordParam : Param<CoordGoo, Coord>
{
    protected override string ModelName => "Coord";
    protected override string ModelNickname => "DPt";
    protected override string ModelDescription => "2D coordinate";
    protected override string IconResourceName => "coord_24x24";
    public override Guid ComponentGuid => new("4685CCE8-C629-4638-8DF6-F76A17571841");
}

public class CoordComponent : PassthroughComponent<CoordParam, CoordGoo, Coord>
{
    public override Guid ComponentGuid => new("61FB9BBE-64DE-42B2-B7EF-69CD97FDD9E3");
    protected override string ModelName => "Coord";
    protected override string ModelNickname => "DPt";
    protected override string ModelDescription => "Construct, deconstruct or modify a 2d coordinate.";
    protected override string IconResourceName => "coord_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("U", "U", "The u-coordinate.", GH_ParamAccess.item);
        pManager.AddNumberParameter("V", "V", "The v-coordinate.", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddNumberParameter("U", "U", "The u-coordinate.", GH_ParamAccess.item);
        pManager.AddNumberParameter("V", "V", "The v-coordinate.", GH_ParamAccess.item);
    }

    protected override void GetModelData(IGH_DataAccess DA, Coord model)
    {
        double u = 0, v = 0;
        if (DA.GetData(2, ref u)) model.U = (float)u;
        if (DA.GetData(3, ref v)) model.V = (float)v;
    }

    protected override void SetModelData(IGH_DataAccess DA, Coord model)
    {
        DA.SetData(2, model.U);
        DA.SetData(3, model.V);
    }
}

public class SerializeCoordComponent : SerializeComponent<CoordParam, CoordGoo, Coord>
{
    public SerializeCoordComponent() { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A99");
}

public class DeserializeCoordComponent : DeserializeComponent<CoordParam, CoordGoo, Coord>
{
    public DeserializeCoordComponent() { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A9A");
}

#endregion 🔖Coord

#region 🔖Location

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖location](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/LOCATION)
// Implementations MUST combine a plane with rotation and elevation for placement.

public class LocationGoo : Goo<Location>
{
    public LocationGoo() { }
    public LocationGoo(Location value) : base(value) { }
    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.Longitude, Value.Latitude, 0));
            return true;
        }
        return false;
    }
    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        var point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new Location { Longitude = (float)point.X, Latitude = (float)point.Y };
            return true;
        }
        return false;
    }
}

public class LocationParam : Param<LocationGoo, Location>
{
    protected override string ModelName => "Location";
    protected override string ModelNickname => "Loc";
    protected override string ModelDescription => "Geographic location";
    protected override string IconResourceName => "location_24x24";
    public override Guid ComponentGuid => new("CA9DA889-398E-469B-BF1B-AD2BDFCA7957");
}

public class LocationComponent : PassthroughComponent<LocationParam, LocationGoo, Location>
{
    public override Guid ComponentGuid => new("6F2EDF42-6E10-4944-8B05-4D41F4876ED0");
    protected override string ModelName => "Location";
    protected override string ModelNickname => "Loc";
    protected override string ModelDescription => "Construct, deconstruct or modify a location.";
    protected override string IconResourceName => "location_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("Longitude", "Lo", "The longitude in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Latitude", "La", "The latitude in degrees.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddNumberParameter("Longitude", "Lo", "The longitude in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Latitude", "La", "The latitude in degrees.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Location model)
    {
        double lon = 0, lat = 0;
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref lon)) model.Longitude = (float)lon;
        if (DA.GetData(3, ref lat)) model.Latitude = (float)lat;
        if (DA.GetDataList(4, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Location model)
    {
        DA.SetData(2, model.Longitude);
        DA.SetData(3, model.Latitude);
        DA.SetDataList(4, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeLocationComponent : SerializeComponent<LocationParam, LocationGoo, Location>
{
    public SerializeLocationComponent() { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D466");
}

public class DeserializeLocationComponent : DeserializeComponent<LocationParam, LocationGoo, Location>
{
    public DeserializeLocationComponent() { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D467");
}

#endregion 🔖Location

#region 🔖Author

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖author](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/AUTHOR)
// Implementations MUST provide author identity with name and contact.

public class AuthorGoo : Goo<Author>
{
    public AuthorGoo() { }
    public AuthorGoo(Author value) : base(value) { }
    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Email);
            return true;
        }
        return false;
    }
    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Author { Email = str };
            return true;
        }
        return false;
    }
}

public class AuthorParam : Param<AuthorGoo, Author>
{
    protected override string ModelName => "Author";
    protected override string ModelNickname => "Aut";
    protected override string ModelDescription => "Author information";
    protected override string IconResourceName => "author_24x24";
    public override Guid ComponentGuid => new("9F52380B-1812-42F7-9DAD-952C2F7A635A");
}

public class AuthorComponent : PassthroughComponent<AuthorParam, AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("5143ED92-0A2C-4D0C-84ED-F90CC8450894");
    protected override string ModelName => "Author";
    protected override string ModelNickname => "Aut";
    protected override string ModelDescription => "Construct, deconstruct or modify an author.";
    protected override string IconResourceName => "author_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the author.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the author.", GH_ParamAccess.item);
        pManager.AddTextParameter("Email", "Em", "The email of the author.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the author.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the author.", GH_ParamAccess.item);
        pManager.AddTextParameter("Email", "Em", "The email of the author.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Author model)
    {
        string guid = "", name = "", email = "";
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref email)) model.Email = email;
        if (DA.GetDataList(5, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Author model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Email);
        DA.SetDataList(5, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeAuthorComponent : SerializeComponent<AuthorParam, AuthorGoo, Author>
{
    public SerializeAuthorComponent() { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634878");
}

public class DeserializeAuthorComponent : DeserializeComponent<AuthorParam, AuthorGoo, Author>
{
    public DeserializeAuthorComponent() { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634879");
}

public class AuthorIdGoo : IdGoo<AuthorId>
{
    public AuthorIdGoo() { }
    public AuthorIdGoo(AuthorId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new AuthorId { Guid = str };
            return true;
        }
        return false;
    }
}

public class AuthorIdParam : IdParam<AuthorIdGoo, AuthorId>
{
    protected override string ModelName => "AuthorId";
    protected override string ModelNickname => "AuI";
    protected override string ModelDescription => "Author identifier";
    protected override string IconResourceName => "author_24x24";
    protected override string IdIconResourceName => "authorid_24x24";
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1C");
}

#endregion 🔖Author

#region 🔖File

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖file](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/FILE)
// Implementations MUST reference a file with URI, MIME type, and optional content.

public class FileGoo : Goo<File>
{
    public FileGoo() { }
    public FileGoo(File value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new File { Guid = str, Name = str, CreatedAt = DateTime.UtcNow, UpdatedAt = DateTime.UtcNow };
            return true;
        }
        return false;
    }
}

public class FileParam : Param<FileGoo, File>
{
    protected override string ModelName => "File";
    protected override string ModelNickname => "Fil";
    protected override string ModelDescription => "File reference";
    protected override string IconResourceName => "file_24x24";
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F8");
}

public class FileComponent : PassthroughComponent<FileParam, FileGoo, File>
{
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F9");
    protected override string ModelName => "File";
    protected override string ModelNickname => "Fil";
    protected override string ModelDescription => "Construct, deconstruct or modify a file.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder path.", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the file.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Folder", "Fo?", "The optional folder path.", GH_ParamAccess.item);
    }

    protected override void GetModelData(IGH_DataAccess DA, File model)
    {
        string guid = "", name = "", remote = "", folderGuid = "";
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref remote)) model.Remote = remote;
        if (DA.GetData(5, ref folderGuid)) model.Folder = new FolderId { Guid = folderGuid };
    }

    protected override void SetModelData(IGH_DataAccess DA, File model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Remote);
        DA.SetData(5, model.Folder?.Guid ?? "");
    }
}

public class SerializeFileComponent : SerializeComponent<FileParam, FileGoo, File>
{
    public SerializeFileComponent() { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FA");
}

public class DeserializeFileComponent : DeserializeComponent<FileParam, FileGoo, File>
{
    public DeserializeFileComponent() { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FB");
}

public class FileIdGoo : IdGoo<FileId>
{
    public FileIdGoo() { }
    public FileIdGoo(FileId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new FileId { Guid = str };
            return true;
        }
        return false;
    }
}

public class FileIdParam : IdParam<FileIdGoo, FileId>
{
    protected override string ModelName => "FileId";
    protected override string ModelNickname => "FId";
    protected override string ModelDescription => "File identifier";
    protected override string IconResourceName => "file_24x24";
    protected override string IdIconResourceName => "fileid_24x24";
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E7");
}

public class FileDiffGoo : DiffGoo<FileDiff>
{
    public FileDiffGoo() { }
    public FileDiffGoo(FileDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid ?? "");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<FileDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FileDiffParam : DiffParam<FileDiffGoo, FileDiff>
{
    protected override string ModelName => "FileDiff";
    protected override string ModelNickname => "FD";
    protected override string ModelDescription => "File diff";
    protected override string IconResourceName => "filediff_24x24";
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F0");
}

public class FileDiffComponent : DiffComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F1");
    protected override string ModelName => "FileDiff";
    protected override string ModelNickname => "FD";
    protected override string ModelDescription => "Construct, deconstruct or modify a file diff.";
    protected override string IconResourceName => "filediff_24x24";
}

public class SerializeFileDiffComponent : SerializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public SerializeFileDiffComponent() { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F2");
}

public class DeserializeFileDiffComponent : DeserializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public DeserializeFileDiffComponent() { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F3");
}

public class FilesDiffGoo : DiffGoo<FilesDiff>
{
    public FilesDiffGoo() { }
    public FilesDiffGoo(FilesDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("FilesDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<FilesDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FilesDiffParam : DiffParam<FilesDiffGoo, FilesDiff>
{
    protected override string ModelName => "FilesDiff";
    protected override string ModelNickname => "FDs";
    protected override string ModelDescription => "File collection diff";
    protected override string IconResourceName => "filesdiff_24x24";
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A1");
}

public class FilesDiffComponent : DiffComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A2");
    protected override string ModelName => "FilesDiff";
    protected override string ModelNickname => "FDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of file diffs.";
    protected override string IconResourceName => "filesdiff_24x24";
}

public class SerializeFilesDiffComponent : SerializeComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public SerializeFilesDiffComponent() { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A3");
}

public class DeserializeFilesDiffComponent : DeserializeComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public DeserializeFilesDiffComponent() { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A4");
}

#endregion 🔖File

#region 🔖Folder

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖folder](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/FOLDER)
// Implementations MUST reference a folder with name and optional parent.

public class FolderGoo : Goo<Folder>
{
    public FolderGoo() { }
    public FolderGoo(Folder value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(FolderIdGoo)))
        {
            target = (Q)(object)new FolderIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(FolderDiffGoo)))
        {
            target = (Q)(object)new FolderDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is FolderIdGoo folderIdGoo)
        {
            Value = folderIdGoo.Value;
            return true;
        }
        if (source is FolderDiffGoo folderDiffGoo)
        {
            Value = folderDiffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Folder { Guid = str };
            return true;
        }
        return false;
    }
}

public class FolderParam : Param<FolderGoo, Folder>
{
    protected override string ModelName => "Folder";
    protected override string ModelNickname => "Fld";
    protected override string ModelDescription => "Folder container";
    protected override string IconResourceName => "folder_24x24";
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A0");
}

public class FolderComponent : PassthroughComponent<FolderParam, FolderGoo, Folder>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A1");
    protected override string ModelName => "Folder";
    protected override string ModelNickname => "Fol";
    protected override string ModelDescription => "Construct, deconstruct or modify a folder.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Parent", "Pa?", "The optional parent folder guid.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the folder.", GH_ParamAccess.item);
        pManager.AddTextParameter("Parent", "Pa?", "The optional parent folder guid.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Folder model)
    {
        string guid = "", name = "", parent = "", description = "";
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref parent)) model.Parent = parent;
        if (DA.GetData(5, ref description)) model.Description = description;
        if (DA.GetDataList(6, attributes)) model.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Folder model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Parent ?? "");
        DA.SetData(5, model.Description);
        DA.SetDataList(6, model.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializeFolderComponent : SerializeComponent<FolderParam, FolderGoo, Folder>
{
    public SerializeFolderComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A2");
}

public class DeserializeFolderComponent : DeserializeComponent<FolderParam, FolderGoo, Folder>
{
    public DeserializeFolderComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A3");
}

public class FolderIdGoo : IdGoo<FolderId>
{
    public FolderIdGoo() { }
    public FolderIdGoo(FolderId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new FolderId { Guid = str };
            return true;
        }
        return false;
    }
}

public class FolderIdParam : IdParam<FolderIdGoo, FolderId>
{
    protected override string ModelName => "FolderId";
    protected override string ModelNickname => "FlI";
    protected override string ModelDescription => "Folder identifier";
    protected override string IconResourceName => "folder_24x24";
    protected override string IdIconResourceName => "folderid_24x24";
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A4");
}

public class FolderDiffGoo : DiffGoo<FolderDiff>
{
    public FolderDiffGoo() { }
    public FolderDiffGoo(FolderDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid ?? "");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<FolderDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FolderDiffParam : DiffParam<FolderDiffGoo, FolderDiff>
{
    protected override string ModelName => "FolderDiff";
    protected override string ModelNickname => "FD";
    protected override string ModelDescription => "Folder diff";
    protected override string IconResourceName => "folderdiff_24x24";
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A5");
}

public class FolderDiffComponent : DiffComponent<FolderDiffParam, FolderDiffGoo, FolderDiff>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A6");
    protected override string ModelName => "FolderDiff";
    protected override string ModelNickname => "FD";
    protected override string ModelDescription => "Construct, deconstruct or modify a folder diff.";
    protected override string IconResourceName => "folderdiff_24x24";
}

public class SerializeFolderDiffComponent : SerializeComponent<FolderDiffParam, FolderDiffGoo, FolderDiff>
{
    public SerializeFolderDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A7");
}

public class DeserializeFolderDiffComponent : DeserializeComponent<FolderDiffParam, FolderDiffGoo, FolderDiff>
{
    public DeserializeFolderDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A8");
}

public class FoldersDiffGoo : DiffGoo<FoldersDiff>
{
    public FoldersDiffGoo() { }
    public FoldersDiffGoo(FoldersDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("FoldersDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<FoldersDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FoldersDiffParam : DiffParam<FoldersDiffGoo, FoldersDiff>
{
    protected override string ModelName => "FoldersDiff";
    protected override string ModelNickname => "FDs";
    protected override string ModelDescription => "Folder collection diff";
    protected override string IconResourceName => "foldersdiff_24x24";
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A9");
}

public class FoldersDiffComponent : DiffComponent<FoldersDiffParam, FoldersDiffGoo, FoldersDiff>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AA");
    protected override string ModelName => "FoldersDiff";
    protected override string ModelNickname => "FDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of folder diffs.";
    protected override string IconResourceName => "foldersdiff_24x24";
}

public class SerializeFoldersDiffComponent : SerializeComponent<FoldersDiffParam, FoldersDiffGoo, FoldersDiff>
{
    public SerializeFoldersDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AB");
}

public class DeserializeFoldersDiffComponent : DeserializeComponent<FoldersDiffParam, FoldersDiffGoo, FoldersDiff>
{
    public DeserializeFoldersDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AC");
}

#endregion 🔖Folder

#region 🔖Benchmark

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖benchmark](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/BENCHMARK)
// Implementations MUST capture benchmark metadata for performance measurement.

public class BenchmarkGoo : Goo<Benchmark>
{
    public BenchmarkGoo() { }
    public BenchmarkGoo(Benchmark value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Benchmark { Name = str };
            return true;
        }
        return false;
    }
}

public class BenchmarkParam : Param<BenchmarkGoo, Benchmark>
{
    protected override string ModelName => "Benchmark";
    protected override string ModelNickname => "Bmk";
    protected override string ModelDescription => "Performance benchmark";
    protected override string IconResourceName => "benchmark_24x24";
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class BenchmarkComponent : PassthroughComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string ModelName => "Benchmark";
    protected override string ModelNickname => "Bmk";
    protected override string ModelDescription => "Construct, deconstruct or modify a benchmark.";
    protected override string IconResourceName => "benchmark_24x24";
}

public class SerializeBenchmarkComponent : SerializeComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public SerializeBenchmarkComponent() { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeBenchmarkComponent : DeserializeComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public DeserializeBenchmarkComponent() { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 🔖Benchmark

#region 🔖QualityKind

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖qualitykind](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/QUALITY-KIND)
// Implementations MUST categorize quality metrics by kind.

public class QualityKindGoo : EnumGoo<QualityKind>
{
    public QualityKindGoo() { }
    public QualityKindGoo(QualityKind value) : base(value) { }
}

public class QualityKindParam : EnumParam<QualityKindGoo, QualityKind>
{
    public QualityKindParam() : base(new("A1B2C3D4-E5F6-4A5B-9C8D-7E6F5A4B3C2D")) { }
}

#endregion 🔖QualityKind

#region 🔖Quality

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖quality](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/QUALITY)
// Implementations MUST combine kind, name, value, and unit for quality metrics.

public class QualityGoo : Goo<Quality>
{
    public QualityGoo() { }
    public QualityGoo(Quality value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(QualityDiffGoo)))
        {
            target = (Q)(object)new QualityDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(QualityIdGoo)))
        {
            target = (Q)(object)new QualityIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is QualityDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is QualityIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Quality>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityIdGoo(QualityGoo goo) => new((QualityId)goo.Value);
    public static implicit operator QualityGoo(QualityIdGoo idGoo) => new((Quality)idGoo.Value);
}

public class QualityParam : Param<QualityGoo, Quality>
{
    protected override string ModelName => "Quality";
    protected override string ModelNickname => "Qal";
    protected override string ModelDescription => "Quality measurement";
    protected override string IconResourceName => "quality_24x24";
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class QualityComponent : PassthroughComponent<QualityParam, QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string ModelName => "Quality";
    protected override string ModelNickname => "Qal";
    protected override string ModelDescription => "Construct, deconstruct or modify a quality.";
    protected override string IconResourceName => "quality_24x24";
}

public class SerializeQualityComponent : SerializeComponent<QualityParam, QualityGoo, Quality>
{
    public SerializeQualityComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C8");
}

public class DeserializeQualityComponent : DeserializeComponent<QualityParam, QualityGoo, Quality>
{
    public DeserializeQualityComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C9");
}

public class QualityIdGoo : IdGoo<QualityId>
{
    public QualityIdGoo() { }
    public QualityIdGoo(QualityId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(QualityGoo)))
        {
            target = (Q)(object)new QualityGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(QualityDiffGoo)))
        {
            target = (Q)(object)new QualityDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is QualityGoo qualityGoo)
        {
            Value = qualityGoo.Value;
            return true;
        }
        if (source is QualityDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<QualityId>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityGoo(QualityIdGoo idGoo) => new(idGoo.Value);
    public static implicit operator QualityIdGoo(QualityGoo goo) => new((QualityId)goo.Value);
}

public class QualityIdParam : IdParam<QualityIdGoo, QualityId>
{
    protected override string ModelName => "QualityId";
    protected override string ModelNickname => "QId";
    protected override string ModelDescription => "Quality identifier";
    protected override string IconResourceName => "quality_24x24";
    protected override string IdIconResourceName => "qualityid_24x24";
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class SerializeQualityIdComponent : SerializeIdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public SerializeQualityIdComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CA");
}

public class DeserializeQualityIdComponent : DeserializeIdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public DeserializeQualityIdComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CB");
}

public class QualityDiffGoo : DiffGoo<QualityDiff>
{
    public QualityDiffGoo() { }
    public QualityDiffGoo(QualityDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(QualityGoo)))
        {
            target = (Q)(object)new QualityGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(QualityIdGoo)))
        {
            target = (Q)(object)new QualityIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is QualityGoo qualityGoo)
        {
            Value = qualityGoo.Value;
            return true;
        }
        if (source is QualityIdGoo qualityIdGoo)
        {
            Value = qualityIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<QualityDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityIdGoo(QualityDiffGoo diffGoo) => new((QualityId)diffGoo.Value);
    public static implicit operator QualityGoo(QualityDiffGoo diffGoo) => new((Quality)diffGoo.Value);
    public static implicit operator QualityDiffGoo(QualityIdGoo idGoo) => new((QualityDiff)idGoo.Value);
    public static implicit operator QualityDiffGoo(QualityGoo goo) => new((QualityDiff)goo.Value);
}

public class QualityDiffParam : DiffParam<QualityDiffGoo, QualityDiff>
{
    protected override string ModelName => "QualityDiff";
    protected override string ModelNickname => "QD";
    protected override string ModelDescription => "Quality diff";
    protected override string IconResourceName => "qualitydiff_24x24";
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DA");
}

public class QualityDiffComponent : DiffComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DB");
    protected override string ModelName => "QualityDiff";
    protected override string ModelNickname => "QD";
    protected override string ModelDescription => "Construct, deconstruct or modify a quality diff.";
    protected override string IconResourceName => "qualitydiff_24x24";
}

public class SerializeQualityDiffComponent : SerializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public SerializeQualityDiffComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DC");
}

public class DeserializeQualityDiffComponent : DeserializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public DeserializeQualityDiffComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DD");
}

#endregion 🔖Quality

#region 🔖Tag

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖tag](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/TAG)
// Implementations MUST provide lightweight labels for categorizing entities.

public class TagGoo : Goo<Tag>
{
    public TagGoo() { }
    public TagGoo(Tag value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TagIdGoo)))
        {
            target = (Q)(object)new TagIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TagIdGoo tagIdGoo)
        {
            Value = tagIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Tag { Guid = str };
            return true;
        }
        return false;
    }
}

public class TagParam : Param<TagGoo, Tag>
{
    protected override string ModelName => "Tag";
    protected override string ModelNickname => "Tag";
    protected override string ModelDescription => "Model tag";
    protected override string IconResourceName => "tag_24x24";
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B0");
}

public class TagComponent : PassthroughComponent<TagParam, TagGoo, Tag>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B1");
    protected override string ModelName => "Tag";
    protected override string ModelNickname => "Tag";
    protected override string ModelDescription => "Construct, deconstruct or modify a tag.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the tag.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the tag.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the tag.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the tag.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Tag model)
    {
        string guid = "", name = "", description = "", icon = "";
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref description)) model.Description = description;
        if (DA.GetData(5, ref icon)) model.Icon = icon;
        if (DA.GetDataList(6, attributes)) model.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Tag model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Description);
        DA.SetData(5, model.Icon);
        DA.SetDataList(6, model.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializeTagComponent : SerializeComponent<TagParam, TagGoo, Tag>
{
    public SerializeTagComponent() { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B2");
}

public class DeserializeTagComponent : DeserializeComponent<TagParam, TagGoo, Tag>
{
    public DeserializeTagComponent() { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B3");
}

public class TagIdGoo : IdGoo<TagId>
{
    public TagIdGoo() { }
    public TagIdGoo(TagId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new TagId { Guid = str };
            return true;
        }
        return false;
    }
}

public class TagIdParam : IdParam<TagIdGoo, TagId>
{
    protected override string ModelName => "TagId";
    protected override string ModelNickname => "TId";
    protected override string ModelDescription => "Tag identifier";
    protected override string IconResourceName => "tag_24x24";
    protected override string IdIconResourceName => "tagid_24x24";
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B4");
}

#endregion 🔖Tag

#region 🔖Prop

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖prop](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/PROP)
// Implementations MUST bind a property name to an expression value.

public class PropGoo : Goo<Prop>
{
    public PropGoo() { }
    public PropGoo(Prop value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Quality.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Prop>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PropParam : Param<PropGoo, Prop>
{
    protected override string ModelName => "Prop";
    protected override string ModelNickname => "Prp";
    protected override string ModelDescription => "Connector property";
    protected override string IconResourceName => "prop_24x24";
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class PropComponent : PassthroughComponent<PropParam, PropGoo, Prop>
{
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string ModelName => "Prop";
    protected override string ModelNickname => "Prp";
    protected override string ModelDescription => "Construct, deconstruct or modify a prop.";
    protected override string IconResourceName => "prop_24x24";
}

public class SerializePropComponent : SerializeComponent<PropParam, PropGoo, Prop>
{
    public SerializePropComponent() { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializePropComponent : DeserializeComponent<PropParam, PropGoo, Prop>
{
    public DeserializePropComponent() { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 🔖Prop

#region 🔖Model

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖model](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/MODEL)
// Implementations MUST reference a 3D model with URI, MIME type, and local plane.

public class ModelGoo : Goo<Model>
{
    public ModelGoo() { }
    public ModelGoo(Model value) : base(value) { }
    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }
    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Model { Guid = str };
            return true;
        }
        return false;
    }
}

public class ModelParam : Param<ModelGoo, Model>
{
    protected override string ModelName => "Model";
    protected override string ModelNickname => "Mdl";
    protected override string ModelDescription => "3D model";
    protected override string IconResourceName => "model_24x24";
    public override Guid ComponentGuid => new("895BBC91-851A-4DFC-9C83-92DFE90029E8");
}

public class ModelComponent : PassthroughComponent<ModelParam, ModelGoo, Model>
{
    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");
    protected override string ModelName => "Model";
    protected override string ModelNickname => "Rep";
    protected override string ModelDescription => "Construct, deconstruct or modify a model.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the model.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the model.", GH_ParamAccess.item);
        pManager.AddTextParameter("FileGuid", "Fl", "The file guid of the model.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("TagGuids", "Tg*", "The optional tag guids.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the model.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the model.", GH_ParamAccess.item);
        pManager.AddTextParameter("FileGuid", "Fl", "The file guid of the model.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("TagGuids", "Tg*", "The optional tag guids.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Model model)
    {
        string guid = "", name = "", fileGuid = "", description = "";
        var tagGuids = new List<string>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref fileGuid)) model.File = new FileId { Guid = fileGuid };
        if (DA.GetData(5, ref description)) model.Description = description;
        if (DA.GetDataList(6, tagGuids)) model.Tags = tagGuids.Select(t => new TagId { Guid = t }).ToList();
        if (DA.GetDataList(7, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Model model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.File.Guid);
        DA.SetData(5, model.Description);
        DA.SetDataList(6, model.Tags.Select(t => t.Guid).ToList());
        DA.SetDataList(7, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }

    protected override Model ProcessModel(Model model)
    {
        return model;
    }
}

public class SerializeModelComponent : SerializeComponent<ModelParam, ModelGoo, Model>
{
    public SerializeModelComponent() { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32046");
}

public class DeserializeModelComponent : DeserializeComponent<ModelParam, ModelGoo, Model>
{
    public DeserializeModelComponent() { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32047");
}

public class ModelIdGoo : IdGoo<ModelId>
{
    public ModelIdGoo() { }
    public ModelIdGoo(ModelId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ModelDiffGoo)))
        {
            target = (Q)(object)new ModelDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ModelGoo)))
        {
            target = (Q)(object)new ModelGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ModelDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is ModelGoo modelGoo)
        {
            Value = modelGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new ModelId { Guid = str };
            return true;
        }
        return false;
    }
}

public class ModelIdParam : IdParam<ModelIdGoo, ModelId>
{
    protected override string ModelName => "ModelId";
    protected override string ModelNickname => "MId";
    protected override string ModelDescription => "Model identifier";
    protected override string IconResourceName => "model_24x24";
    protected override string IdIconResourceName => "modelid_24x24";
    public override Guid ComponentGuid => new("30A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
}

public class ModelDiffGoo : DiffGoo<ModelDiff>
{
    public ModelDiffGoo() { }
    public ModelDiffGoo(ModelDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(string.Join(",", Value.Tags));
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ModelDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ModelDiffParam : DiffParam<ModelDiffGoo, ModelDiff>
{
    protected override string ModelName => "ModelDiff";
    protected override string ModelNickname => "MD";
    protected override string ModelDescription => "Model diff";
    protected override string IconResourceName => "modeldiff_24x24";
    public override Guid ComponentGuid => new("7C8E9FA0-B1C2-D3E4-F5A6-B7C8D9E0F1A2");
}

public class ModelDiffComponent : DiffComponent<ModelDiffParam, ModelDiffGoo, ModelDiff>
{
    public override Guid ComponentGuid => new("8D9FA0B1-C2D3-E4F5-A6B7-C8D9E0F1A2B3");
    protected override string ModelName => "ModelDiff";
    protected override string ModelNickname => "MD";
    protected override string ModelDescription => "Construct, deconstruct or modify a model diff.";
    protected override string IconResourceName => "modeldiff_24x24";
}

public class SerializeModelDiffComponent : SerializeComponent<ModelDiffParam, ModelDiffGoo, ModelDiff>
{
    public SerializeModelDiffComponent() { }
    public override Guid ComponentGuid => new("71E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AB");
}

public class DeserializeModelDiffComponent : DeserializeComponent<ModelDiffParam, ModelDiffGoo, ModelDiff>
{
    public DeserializeModelDiffComponent() { }
    public override Guid ComponentGuid => new("AFB1C2D3-E4F5-A6B7-C8D9-E0F1A2B3C4D5");
}

public class ModelsDiffGoo : DiffGoo<ModelsDiff>
{
    public ModelsDiffGoo() { }
    public ModelsDiffGoo(ModelsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ModelsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ModelsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ModelsDiffParam : DiffParam<ModelsDiffGoo, ModelsDiff>
{
    protected override string ModelName => "ModelsDiff";
    protected override string ModelNickname => "MDs";
    protected override string ModelDescription => "Model collection diff";
    protected override string IconResourceName => "modelsdiff_24x24";
    public override Guid ComponentGuid => new("9EA0B1C2-D3E4-F5A6-B7C8-D9E0F1A2B3C4");
}

public class ModelsDiffComponent : DiffComponent<ModelsDiffParam, ModelsDiffGoo, ModelsDiff>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AD");
    protected override string ModelName => "ModelsDiff";
    protected override string ModelNickname => "MDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of model diffs.";
    protected override string IconResourceName => "modelsdiff_24x24";
}

public class SerializeModelsDiffComponent : SerializeComponent<ModelsDiffParam, ModelsDiffGoo, ModelsDiff>
{
    public SerializeModelsDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AE");
}

public class DeserializeModelsDiffComponent : DeserializeComponent<ModelsDiffParam, ModelsDiffGoo, ModelsDiff>
{
    public DeserializeModelsDiffComponent() { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AF");
}

#endregion 🔖Model

#region 🔖Connector

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖connector](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/CONNECTOR)
// Implementations MUST define located interface points on a type.

public class ConnectorGoo : Goo<Connector>
{
    public ConnectorGoo() { }
    public ConnectorGoo(Connector value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Plane)))
        {
            if (Value.Direction is null || Value.Point is null) return false;
            target = (Q)(object)new GH_Plane(Utility.GetPlaneFromYAxis(Value.Direction.Convert(), 0, Value.Point.Convert()));
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorIdGoo)))
        {
            target = (Q)(object)new ConnectorIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorDiffGoo)))
        {
            target = (Q)(object)new ConnectorDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectorIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is ConnectorDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        var plane = new Rhino.Geometry.Plane();
        if (GH_Convert.ToPlane(source, ref plane, GH_Conversion.Both))
        {
            Value.Point = plane.Origin.Convert();
            Value.Direction = plane.YAxis.Convert();
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            var deserialized = str.Deserialize<Connector>();
            if (deserialized is null) return false;
            Value = deserialized;
            return true;
        }
        return false;
    }
}

public class ConnectorParam : Param<ConnectorGoo, Connector>
{
    protected override string ModelName => "Connector";
    protected override string ModelNickname => "Con";
    protected override string ModelDescription => "Connection point";
    protected override string IconResourceName => "connector_24x24";
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1B");
}

public class ConnectorComponent : PassthroughComponent<ConnectorParam, ConnectorGoo, Connector>
{
    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");
    protected override string ModelName => "Connector";
    protected override string ModelNickname => "Por";
    protected override string ModelDescription => "Construct, deconstruct or modify a connector.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mandatory", "Ma?", "Whether the connector is mandatory.", GH_ParamAccess.item);
        pManager.AddPointParameter("Point", "Pt", "The connection point.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Direction", "Dr", "The direction of the connector.", GH_ParamAccess.item);
        pManager.AddNumberParameter("T", "T", "The t parameter [0,1[.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the connector.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Mandatory", "Ma?", "Whether the connector is mandatory.", GH_ParamAccess.item);
        pManager.AddPointParameter("Point", "Pt", "The connection point.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Direction", "Dr", "The direction of the connector.", GH_ParamAccess.item);
        pManager.AddNumberParameter("T", "T", "The t parameter [0,1[.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Connector model)
    {
        string guid = "", name = "", description = "";
        bool mandatory = false;
        Point3d point = Point3d.Origin;
        Vector3d direction = Vector3d.YAxis;
        double t = 0;
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref description)) model.Description = description;
        if (DA.GetData(5, ref mandatory)) model.Mandatory = mandatory;
        if (DA.GetData(6, ref point)) model.Point = RhinoConverter.Convert(point);
        if (DA.GetData(7, ref direction)) model.Direction = RhinoConverter.Convert(direction);
        if (DA.GetData(8, ref t)) model.T = (float)t;
        if (DA.GetDataList(9, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Connector model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Description);
        DA.SetData(5, model.Mandatory);
        DA.SetData(6, model.Point is not null ? RhinoConverter.Convert(model.Point) : Point3d.Origin);
        DA.SetData(7, model.Direction is not null ? RhinoConverter.Convert(model.Direction) : Vector3d.YAxis);
        DA.SetData(8, model.T);
        DA.SetDataList(9, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializePortComponent : SerializeComponent<ConnectorParam, ConnectorGoo, Connector>
{
    public SerializePortComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B5");
}

public class DeserializePortComponent : DeserializeComponent<ConnectorParam, ConnectorGoo, Connector>
{
    public DeserializePortComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B6");
}

public class ConnectorIdGoo : IdGoo<ConnectorId>
{
    public ConnectorIdGoo() { }
    public ConnectorIdGoo(ConnectorId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorDiffGoo)))
        {
            target = (Q)(object)new ConnectorDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorGoo)))
        {
            target = (Q)(object)new ConnectorGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectorGoo connectorGoo)
        {
            Value = connectorGoo.Value;
            return true;
        }
        if (source is ConnectorDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new ConnectorId { Guid = str };
            return true;
        }
        return false;
    }
}

public class ConnectorIdParam : IdParam<ConnectorIdGoo, ConnectorId>
{
    protected override string ModelName => "ConnectorId";
    protected override string ModelNickname => "CId";
    protected override string ModelDescription => "Connector identifier";
    protected override string IconResourceName => "connector_24x24";
    protected override string IdIconResourceName => "connectorid_24x24";
    public override Guid ComponentGuid => new("C1D2E3F4-A5B6-C7D8-E9F0-A1B2C3D4E5F6");
}

public class ConnectorDiffGoo : DiffGoo<ConnectorDiff>
{
    public ConnectorDiffGoo() { }
    public ConnectorDiffGoo(ConnectorDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorIdGoo)))
        {
            target = (Q)(object)new ConnectorIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectorGoo)))
        {
            target = (Q)(object)new ConnectorGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectorIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is ConnectorGoo connectorGoo)
        {
            Value = connectorGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectorDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ConnectorDiffParam : DiffParam<ConnectorDiffGoo, ConnectorDiff>
{
    protected override string ModelName => "ConnectorDiff";
    protected override string ModelNickname => "CD";
    protected override string ModelDescription => "Connector diff";
    protected override string IconResourceName => "connectordiff_24x24";
    public override Guid ComponentGuid => new("B0C1D2E3-F4A5-B6C7-D8E9-F0A1B2C3D4E5");
}

public class ConnectorDiffComponent : DiffComponent<ConnectorDiffParam, ConnectorDiffGoo, ConnectorDiff>
{
    public override Guid ComponentGuid => new("E3F4A5B6-C7D8-E9F0-A1B2-C3D4E5F6A7B8");
    protected override string ModelName => "ConnectorDiff";
    protected override string ModelNickname => "CD";
    protected override string ModelDescription => "Construct, deconstruct or modify a connector diff.";
    protected override string IconResourceName => "connectordiff_24x24";
}

public class SerializePortDiffComponent : SerializeComponent<ConnectorDiffParam, ConnectorDiffGoo, ConnectorDiff>
{
    public SerializePortDiffComponent() { }
    public override Guid ComponentGuid => new("F4A5B6C7-D8E9-F0A1-B2C3-D4E5F6A7B8C9");
}

public class DeserializePortDiffComponent : DeserializeComponent<ConnectorDiffParam, ConnectorDiffGoo, ConnectorDiff>
{
    public DeserializePortDiffComponent() { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B5");
}

public class ConnectorsDiffGoo : DiffGoo<ConnectorsDiff>
{
    public ConnectorsDiffGoo() { }
    public ConnectorsDiffGoo(ConnectorsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ConnectorsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectorsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ConnectorsDiffParam : DiffParam<ConnectorsDiffGoo, ConnectorsDiff>
{
    protected override string ModelName => "ConnectorsDiff";
    protected override string ModelNickname => "CDs";
    protected override string ModelDescription => "Connector collection diff";
    protected override string IconResourceName => "connectorsdiff_24x24";
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C0");
}

public class ConnectorsDiffComponent : DiffComponent<ConnectorsDiffParam, ConnectorsDiffGoo, ConnectorsDiff>
{
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C1");
    protected override string ModelName => "ConnectorsDiff";
    protected override string ModelNickname => "CDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of connector diffs.";
    protected override string IconResourceName => "connectorsdiff_24x24";
}

public class SerializePortsDiffComponent : SerializeComponent<ConnectorsDiffParam, ConnectorsDiffGoo, ConnectorsDiff>
{
    public SerializePortsDiffComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C2");
}

public class DeserializePortsDiffComponent : DeserializeComponent<ConnectorsDiffParam, ConnectorsDiffGoo, ConnectorsDiff>
{
    public DeserializePortsDiffComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C3");
}

#endregion 🔖Connector

#region 🔖Concept

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖concept](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/CONCEPT)
// Implementations MUST link a semantic concept name to description and icon.

public class ConceptGoo : Goo<Concept>
{
    public ConceptGoo() { }
    public ConceptGoo(Concept value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConceptIdGoo)))
        {
            target = (Q)(object)new ConceptIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConceptIdGoo conceptIdGoo)
        {
            Value = conceptIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Concept { Guid = str };
            return true;
        }
        return false;
    }
}

public class ConceptParam : Param<ConceptGoo, Concept>
{
    protected override string ModelName => "Concept";
    protected override string ModelNickname => "Cpt";
    protected override string ModelDescription => "Semantic concept";
    protected override string IconResourceName => "concept_24x24";
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C1");
}

public class ConceptComponent : PassthroughComponent<ConceptParam, ConceptGoo, Concept>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C2");
    protected override string ModelName => "Concept";
    protected override string ModelNickname => "Con";
    protected override string ModelDescription => "Construct, deconstruct or modify a concept.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the concept.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the concept.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the concept.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the concept.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Concept model)
    {
        string guid = "", name = "", description = "", icon = "";
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref description)) model.Description = description;
        if (DA.GetData(5, ref icon)) model.Icon = icon;
        if (DA.GetDataList(6, attributes)) model.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Concept model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Description);
        DA.SetData(5, model.Icon);
        DA.SetDataList(6, model.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializeConceptComponent : SerializeComponent<ConceptParam, ConceptGoo, Concept>
{
    public SerializeConceptComponent() { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C3");
}

public class DeserializeConceptComponent : DeserializeComponent<ConceptParam, ConceptGoo, Concept>
{
    public DeserializeConceptComponent() { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C4");
}

public class ConceptIdGoo : IdGoo<ConceptId>
{
    public ConceptIdGoo() { }
    public ConceptIdGoo(ConceptId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new ConceptId { Guid = str };
            return true;
        }
        return false;
    }
}

public class ConceptIdParam : IdParam<ConceptIdGoo, ConceptId>
{
    protected override string ModelName => "ConceptId";
    protected override string ModelNickname => "CId";
    protected override string ModelDescription => "Concept identifier";
    protected override string IconResourceName => "concept_24x24";
    protected override string IdIconResourceName => "conceptid_24x24";
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C5");
}

#endregion 🔖Concept

#region 🔖Port

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖port](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/PORT)
// Implementations MUST define connection ports as typed interfaces on a type.

public class PortGoo : Goo<Port>
{
    public PortGoo() { }
    public PortGoo(Port value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PortIdGoo)))
        {
            target = (Q)(object)new PortIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is PortIdGoo portIdGoo)
        {
            Value = portIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Port { Guid = str };
            return true;
        }
        return false;
    }
}

public class PortParam : Param<PortGoo, Port>
{
    protected override string ModelName => "Port";
    protected override string ModelNickname => "Ifc";
    protected override string ModelDescription => "Connector compatibility";
    protected override string IconResourceName => "interface_24x24";
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D2");
}

public class PortComponent : PassthroughComponent<PortParam, PortGoo, Port>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D3");
    protected override string ModelName => "Port";
    protected override string ModelNickname => "Ifc";
    protected override string ModelDescription => "Construct, deconstruct or modify an port.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam() { Access = GH_ParamAccess.list }, "CompatiblePorts", "CF*", "The optional compatible ports.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm", "The name of the port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddParameter(new PortIdParam() { Access = GH_ParamAccess.list }, "CompatiblePorts", "CF*", "The optional compatible ports.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Port model)
    {
        string guid = "", name = "", description = "", icon = "";
        var compatiblePorts = new List<PortIdGoo>();
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref description)) model.Description = description;
        if (DA.GetData(5, ref icon)) model.Icon = icon;
        if (DA.GetDataList(6, compatiblePorts)) model.CompatiblePorts = compatiblePorts.Select(i => i.Value).ToList();
        if (DA.GetDataList(7, attributes)) model.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Port model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Description);
        DA.SetData(5, model.Icon);
        DA.SetDataList(6, model.CompatiblePorts.Select(i => new PortIdGoo(i)).ToList());
        DA.SetDataList(7, model.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializePortComponent : SerializeComponent<PortParam, PortGoo, Port>
{
    public SerializePortComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D4");
}

public class DeserializePortComponent : DeserializeComponent<PortParam, PortGoo, Port>
{
    public DeserializePortComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D5");
}

public class PortIdGoo : IdGoo<PortId>
{
    public PortIdGoo() { }
    public PortIdGoo(PortId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new PortId { Guid = str };
            return true;
        }
        return false;
    }
}

public class PortIdParam : IdParam<PortIdGoo, PortId>
{
    protected override string ModelName => "PortId";
    protected override string ModelNickname => "IId";
    protected override string ModelDescription => "Port identifier";
    protected override string IconResourceName => "interface_24x24";
    protected override string IdIconResourceName => "interfaceid_24x24";
    public override Guid ComponentGuid => new("78187B1A-F476-44D9-A382-DE2C47019DB8");
}

#endregion 🔖Port

#region 🔖Type

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖type](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/TYPE)
// Implementations MUST compose ports, connectors, and models into a parametric type.

public class TypeGoo : Goo<Type>
{
    public TypeGoo() { }
    public TypeGoo(Type value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TypeDiffGoo)))
        {
            target = (Q)(object)new TypeDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(TypeIdGoo)))
        {
            target = (Q)(object)new TypeIdGoo(Value);
            return true;
        }
        if (target is PieceGoo piece)
        {
            piece.Value = new Piece
            {
                Guid = Semio.Utility.GenerateRandomId(new Random().Next()),
                Type = new TypeId { Guid = Value.Guid }
            };
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TypeDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is TypeIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is PieceGoo piece)
        {
            if (piece.Value.Type is null) return false;
            Value = new Type { Guid = piece.Value.Type.Guid, Name = piece.Value.Type.Guid };
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Type { Name = str };
            return true;
        }
        return false;
    }
}

public class TypeParam : Param<TypeGoo, Type>
{
    protected override string ModelName => "Type";
    protected override string ModelNickname => "Typ";
    protected override string ModelDescription => "Reusable component";
    protected override string IconResourceName => "type_24x24";
    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");
}

public class TypeComponent : PassthroughComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");
    protected override string ModelName => "Type";
    protected override string ModelNickname => "Typ";
    protected override string ModelDescription => "Construct, deconstruct or modify a type.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The length unit.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Virtual", "Vi?", "Whether the type is virtual.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Stock", "St?", "The stock quantity.", GH_ParamAccess.item);
        pManager.AddParameter(new ModelParam(), "Models", "Rp*", "The optional models.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectorParam(), "Connectors", "Po*", "The optional connectors.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The length unit.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Virtual", "Vi?", "Whether the type is virtual.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Stock", "St?", "The stock quantity.", GH_ParamAccess.item);
        pManager.AddParameter(new ModelParam(), "Models", "Rp*", "The optional models.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectorParam(), "Connectors", "Po*", "The optional connectors.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Type model)
    {
        string guid = "", name = "", description = "", icon = "", image = "", unit = "";
        bool virtual_ = false;
        int stock = 0;
        var models = new List<ModelGoo>();
        var connectors = new List<ConnectorGoo>();
        var authors = new List<AuthorIdGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref description)) model.Description = description;
        if (DA.GetData(5, ref icon)) model.Icon = icon;
        if (DA.GetData(6, ref image)) model.Image = image;
        if (DA.GetData(7, ref unit)) model.Unit = unit;
        if (DA.GetData(8, ref virtual_)) model.Virtual = virtual_;
        if (DA.GetData(9, ref stock)) model.Stock = stock;
        if (DA.GetDataList(10, models)) model.Models = models.Select(m => m.Value.DeepClone()).ToList();
        if (DA.GetDataList(11, connectors)) model.Connectors = connectors.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(12, authors)) model.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(13, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Type model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Description);
        DA.SetData(5, model.Icon);
        DA.SetData(6, model.Image);
        DA.SetData(7, model.Unit);
        DA.SetData(8, model.Virtual);
        DA.SetData(9, model.Stock);
        DA.SetDataList(10, model.Models?.Select(m => new ModelGoo(m.DeepClone())).ToList());
        DA.SetDataList(11, model.Connectors?.Select(p => new ConnectorGoo(p.DeepClone())).ToList());
        DA.SetDataList(12, model.Authors?.Select(a => new AuthorIdGoo(a.DeepClone())).ToList());
        DA.SetDataList(13, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }

    protected override Type ProcessModel(Type type)
    {
        if (type.Unit == "")
            try { type.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem); }
            catch (Exception) { type.Unit = "m"; }

        type.Icon = type.Icon.Replace('\\', '/');
        type.Image = type.Image.Replace('\\', '/');
        return type;
    }
}

public class SerializeTypeComponent : SerializeComponent<TypeParam, TypeGoo, Type>
{
    public SerializeTypeComponent() { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673A");
}

public class DeserializeTypeComponent : DeserializeComponent<TypeParam, TypeGoo, Type>
{
    public DeserializeTypeComponent() { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673B");
}

public class TypeIdGoo : IdGoo<TypeId>
{
    public TypeIdGoo() { }
    public TypeIdGoo(TypeId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TypeGoo)))
        {
            target = (Q)(object)new TypeGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(TypeDiffGoo)))
        {
            target = (Q)(object)new TypeDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TypeGoo typeGoo)
        {
            Value = typeGoo.Value;
            return true;
        }
        if (source is TypeDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new TypeId { Guid = str };
            return true;
        }
        return false;
    }
}

public class TypeIdParam : IdParam<TypeIdGoo, TypeId>
{
    protected override string ModelName => "TypeId";
    protected override string ModelNickname => "TId";
    protected override string ModelDescription => "Type identifier";
    protected override string IconResourceName => "type_24x24";
    protected override string IdIconResourceName => "typeid_24x24";
    public override Guid ComponentGuid => new("A1B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D6");
}

public class TypeDiffGoo : DiffGoo<TypeDiff>
{
    public TypeDiffGoo() { }
    public TypeDiffGoo(TypeDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(TypeGoo)))
        {
            target = (Q)(object)new TypeGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(TypeIdGoo)))
        {
            target = (Q)(object)new TypeIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is TypeGoo typeGoo)
        {
            Value = typeGoo.Value;
            return true;
        }
        if (source is TypeIdGoo typeIdGoo)
        {
            Value = typeIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<TypeDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class TypeDiffParam : DiffParam<TypeDiffGoo, TypeDiff>
{
    protected override string ModelName => "TypeDiff";
    protected override string ModelNickname => "TD";
    protected override string ModelDescription => "Type diff";
    protected override string IconResourceName => "typediff_24x24";
    public override Guid ComponentGuid => new("C3D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F8");
}

public class TypeDiffComponent : DiffComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public override Guid ComponentGuid => new("D4E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A9");
    protected override string ModelName => "TypeDiff";
    protected override string ModelNickname => "TD";
    protected override string ModelDescription => "Construct, deconstruct or modify a type diff.";
    protected override string IconResourceName => "typediff_24x24";
}

public class SerializeTypeDiffComponent : SerializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public SerializeTypeDiffComponent() { }
    public override Guid ComponentGuid => new("E5F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B0");
}

public class DeserializeTypeDiffComponent : DeserializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public DeserializeTypeDiffComponent() { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C6");
}

public class TypesDiffGoo : DiffGoo<TypesDiff>
{
    public TypesDiffGoo() { }
    public TypesDiffGoo(TypesDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("TypesDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<TypesDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class TypesDiffParam : DiffParam<TypesDiffGoo, TypesDiff>
{
    protected override string ModelName => "TypesDiff";
    protected override string ModelNickname => "TDs";
    protected override string ModelDescription => "Type collection diff";
    protected override string IconResourceName => "typesdiff_24x24";
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B6");
}

public class TypesDiffComponent : DiffComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B7");
    protected override string ModelName => "TypesDiff";
    protected override string ModelNickname => "TDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of type diffs.";
    protected override string IconResourceName => "typesdiff_24x24";
}

public class SerializeTypesDiffComponent : SerializeComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public SerializeTypesDiffComponent() { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B8");
}

public class DeserializeTypesDiffComponent : DeserializeComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public DeserializeTypesDiffComponent() { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B9");
}

#endregion 🔖Type

#region 🔖Layer

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖layer](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/LAYER)
// Implementations MUST organize pieces into named layers within a design.

public class LayerGoo : Goo<Layer>
{
    public LayerGoo() { }
    public LayerGoo(Layer value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Layer>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class LayerParam : Param<LayerGoo, Layer>
{
    protected override string ModelName => "Layer";
    protected override string ModelNickname => "Lyr";
    protected override string ModelDescription => "Design layer";
    protected override string IconResourceName => "layer_24x24";
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class LayerComponent : PassthroughComponent<LayerParam, LayerGoo, Layer>
{
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string ModelName => "Layer";
    protected override string ModelNickname => "Lyr";
    protected override string ModelDescription => "Construct, deconstruct or modify a layer.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the layer.", GH_ParamAccess.item);
        pManager.AddTextParameter("Path", "Pa", "The path of the layer.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsHidden", "Hd?", "Whether the layer is hidden.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsLocked", "Lk?", "Whether the layer is locked.", GH_ParamAccess.item);
        pManager.AddTextParameter("Color", "Cl?", "The optional hex color.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the layer.", GH_ParamAccess.item);
        pManager.AddTextParameter("Path", "Pa", "The path of the layer.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsHidden", "Hd?", "Whether the layer is hidden.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("IsLocked", "Lk?", "Whether the layer is locked.", GH_ParamAccess.item);
        pManager.AddTextParameter("Color", "Cl?", "The optional hex color.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam() { Access = GH_ParamAccess.list }, "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Layer model)
    {
        string guid = "", path = "", color = "", description = "";
        bool isHidden = false, isLocked = false;
        var attributes = new List<AttributeGoo>();
        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref path)) model.Path = path;
        if (DA.GetData(4, ref isHidden)) model.IsHidden = isHidden;
        if (DA.GetData(5, ref isLocked)) model.IsLocked = isLocked;
        if (DA.GetData(6, ref color)) model.Color = color;
        if (DA.GetData(7, ref description)) model.Description = description;
        if (DA.GetDataList(8, attributes)) model.Attributes = attributes.Select(a => a.Value).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Layer model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Path);
        DA.SetData(4, model.IsHidden);
        DA.SetData(5, model.IsLocked);
        DA.SetData(6, model.Color);
        DA.SetData(7, model.Description);
        DA.SetDataList(8, model.Attributes.Select(a => new AttributeGoo(a)).ToList());
    }
}

public class SerializeLayerComponent : SerializeComponent<LayerParam, LayerGoo, Layer>
{
    public SerializeLayerComponent() { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeLayerComponent : DeserializeComponent<LayerParam, LayerGoo, Layer>
{
    public DeserializeLayerComponent() { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 🔖Layer

#region 🔖Group

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖group](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/GROUP)
// Implementations MUST group pieces by name within a design.

public class GroupGoo : Goo<Group>
{
    public GroupGoo() { }
    public GroupGoo(Group value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Group>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class GroupParam : Param<GroupGoo, Group>
{
    protected override string ModelName => "Group";
    protected override string ModelNickname => "Grp";
    protected override string ModelDescription => "Piece grouping";
    protected override string IconResourceName => "group_24x24";
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class GroupComponent : PassthroughComponent<GroupParam, GroupGoo, Group>
{
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string ModelName => "Group";
    protected override string ModelNickname => "Grp";
    protected override string ModelDescription => "Construct, deconstruct or modify a group.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "Na?", "The optional name of the group.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "Pieces", "Pc*", "The pieces in the group.", GH_ParamAccess.list);
        pManager.AddTextParameter("Color", "Cl?", "The optional hex color.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "Na?", "The optional name of the group.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "Pieces", "Pc*", "The pieces in the group.", GH_ParamAccess.list);
        pManager.AddTextParameter("Color", "Cl?", "The optional hex color.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Group model)
    {
        string name = "", description = "", color = "";
        var pieces = new List<PieceIdGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref name)) model.Name = name;
        if (DA.GetData(3, ref description)) model.Description = description;
        if (DA.GetDataList(4, pieces)) model.Pieces = pieces.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetData(5, ref color)) model.Color = color;
        if (DA.GetDataList(6, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Group model)
    {
        DA.SetData(2, model.Name);
        DA.SetData(3, model.Description);
        DA.SetDataList(4, model.Pieces?.Select(p => new PieceIdGoo(p.DeepClone())).ToList());
        DA.SetData(5, model.Color);
        DA.SetDataList(6, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeGroupComponent : SerializeComponent<GroupParam, GroupGoo, Group>
{
    public SerializeGroupComponent() { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeGroupComponent : DeserializeComponent<GroupParam, GroupGoo, Group>
{
    public DeserializeGroupComponent() { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 🔖Group

#region 🔖Piece

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖piece](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/PIECE)
// Implementations MUST place an instantiated type within a design hierarchy.

public class PieceGoo : Goo<Piece>
{
    public PieceGoo() { }
    public PieceGoo(Piece value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PieceDiffGoo)))
        {
            target = (Q)(object)new PieceDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PieceIdGoo)))
        {
            target = (Q)(object)new PieceIdGoo(Value);
            return true;
        }
        if (target is TypeGoo type)
        {
            if (Value.Type is null) return false;
            type.Value = new Type { Guid = Value.Type.Guid, Name = Value.Type.Guid };
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is PieceDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is PieceIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is TypeGoo type)
        {
            Value = new Piece
            {
                Guid = Semio.Utility.GenerateRandomId(new Random().Next()),
                Type = new TypeId { Guid = type.Value.Guid }
            };
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Piece { Guid = str };
            return true;
        }
        return false;
    }
}

public class PieceParam : Param<PieceGoo, Piece>
{
    protected override string ModelName => "Piece";
    protected override string ModelNickname => "Pce";
    protected override string ModelDescription => "Design instance";
    protected override string IconResourceName => "piece_24x24";
    public override Guid ComponentGuid => new("76F583DC-4142-4346-B1E1-6C241AF26086");
}

public class PieceComponent : PassthroughComponent<PieceParam, PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA48");
    protected override string ModelName => "Piece";
    protected override string ModelNickname => "Pce";
    protected override string ModelDescription => "Construct, deconstruct or modify a piece.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Type", "Ty?", "The optional type of the piece.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pl?", "The optional plane of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new CoordParam(), "Center", "Ce?", "The optional center in the diagram.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Nm?", "The optional name of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeIdParam(), "Type", "Ty?", "The optional type of the piece.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pl?", "The optional plane of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new CoordParam(), "Center", "Ce?", "The optional center in the diagram.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Piece model)
    {
        string guid = "", name = "", description = "";
        var type = new TypeIdGoo();
        Rhino.Geometry.Plane plane = Rhino.Geometry.Plane.WorldXY;
        var center = new CoordGoo();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref description)) model.Description = description;
        if (DA.GetData(5, ref type)) model.Type = type.Value.DeepClone();
        if (DA.GetData(6, ref plane)) model.Plane = RhinoConverter.Convert(plane);
        if (DA.GetData(7, ref center)) model.Center = center.Value.DeepClone();
        if (DA.GetDataList(8, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Piece model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Description);
        DA.SetData(5, model.Type is not null ? new TypeIdGoo(model.Type.DeepClone()) : null);
        DA.SetData(6, model.Plane is not null ? RhinoConverter.Convert(model.Plane) : Rhino.Geometry.Plane.WorldXY);
        DA.SetData(7, model.Center is not null ? new CoordGoo(model.Center.DeepClone()) : null);
        DA.SetDataList(8, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializePieceComponent : SerializeComponent<PieceParam, PieceGoo, Piece>
{
    public SerializePieceComponent() { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00D9");
}

public class DeserializePieceComponent : DeserializeComponent<PieceParam, PieceGoo, Piece>
{
    public DeserializePieceComponent() { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00DA");
}

public class PieceIdGoo : IdGoo<PieceId>
{
    public PieceIdGoo() { }
    public PieceIdGoo(PieceId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PieceGoo)))
        {
            target = (Q)(object)new PieceGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PieceDiffGoo)))
        {
            target = (Q)(object)new PieceDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is PieceGoo pieceGoo)
        {
            Value = pieceGoo.Value;
            return true;
        }
        if (source is PieceDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new PieceId { Guid = str };
            return true;
        }
        return false;
    }
}

public class PieceIdParam : IdParam<PieceIdGoo, PieceId>
{
    protected override string ModelName => "PieceId";
    protected override string ModelNickname => "PId";
    protected override string ModelDescription => "Piece identifier";
    protected override string IconResourceName => "piece_24x24";
    protected override string IdIconResourceName => "pieceid_24x24";
    public override Guid ComponentGuid => new("F6A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C1");
}

public class PieceDiffGoo : DiffGoo<PieceDiff>
{
    public PieceDiffGoo() { }
    public PieceDiffGoo(PieceDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PieceGoo)))
        {
            target = (Q)(object)new PieceGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PieceIdGoo)))
        {
            target = (Q)(object)new PieceIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is PieceGoo pieceGoo)
        {
            Value = pieceGoo.Value;
            return true;
        }
        if (source is PieceIdGoo pieceIdGoo)
        {
            Value = pieceIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<PieceDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PieceDiffParam : DiffParam<PieceDiffGoo, PieceDiff>
{
    protected override string ModelName => "PieceDiff";
    protected override string ModelNickname => "PD";
    protected override string ModelDescription => "Piece diff";
    protected override string IconResourceName => "piecediff_24x24";
    public override Guid ComponentGuid => new("B8C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E3");
}

public class PieceDiffComponent : DiffComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public override Guid ComponentGuid => new("C9D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F4");
    protected override string ModelName => "PieceDiff";
    protected override string ModelNickname => "PD";
    protected override string ModelDescription => "Construct, deconstruct or modify a piece diff.";
    protected override string IconResourceName => "piecediff_24x24";
}

public class SerializePieceDiffComponent : SerializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public SerializePieceDiffComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D6");
}

public class DeserializePieceDiffComponent : DeserializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public DeserializePieceDiffComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D7");
}

public class PiecesDiffGoo : DiffGoo<PiecesDiff>
{
    public PiecesDiffGoo() { }
    public PiecesDiffGoo(PiecesDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("PiecesDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<PiecesDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PiecesDiffParam : DiffParam<PiecesDiffGoo, PiecesDiff>
{
    protected override string ModelName => "PiecesDiff";
    protected override string ModelNickname => "PDs";
    protected override string ModelDescription => "Piece collection diff";
    protected override string IconResourceName => "piecesdiff_24x24";
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C7");
}

public class PiecesDiffComponent : DiffComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C8");
    protected override string ModelName => "PiecesDiff";
    protected override string ModelNickname => "PDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of piece diffs.";
    protected override string IconResourceName => "piecesdiff_24x24";
}

public class SerializePiecesDiffComponent : SerializeComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public SerializePiecesDiffComponent() { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C9");
}

public class DeserializePiecesDiffComponent : DeserializeComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public DeserializePiecesDiffComponent() { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6CA");
}

#endregion 🔖Piece

#region 🔖Side

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖side](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/SIDE)
// Implementations MUST reference a piece and connector as a connection endpoint.

public class SideGoo : Goo<Side>
{
    public SideGoo() { }
    public SideGoo(Side value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Piece.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Side { Piece = new PieceId { Guid = str } };
            return true;
        }
        return false;
    }
}

public class SideParam : Param<SideGoo, Side>
{
    protected override string ModelName => "Side";
    protected override string ModelNickname => "Sid";
    protected override string ModelDescription => "Connection side";
    protected override string IconResourceName => "side_24x24";
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
}

public class SideComponent : PassthroughComponent<SideParam, SideGoo, Side>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E7");
    protected override string ModelName => "Side";
    protected override string ModelNickname => "Sde";
    protected override string ModelDescription => "Construct, deconstruct or modify a side.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam(), "Piece", "Pc", "The piece of the side.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "DesignPiece", "DP?", "The optional design piece.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectorIdParam(), "Connector", "Po", "The connector of the side.", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceIdParam(), "Piece", "Pc", "The piece of the side.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceIdParam(), "DesignPiece", "DP?", "The optional design piece.", GH_ParamAccess.item);
        pManager.AddParameter(new ConnectorIdParam(), "Connector", "Po", "The connector of the side.", GH_ParamAccess.item);
    }

    protected override void GetModelData(IGH_DataAccess DA, Side model)
    {
        var piece = new PieceIdGoo();
        var designPiece = new PieceIdGoo();
        var connector = new ConnectorIdGoo();

        if (DA.GetData(2, ref piece)) model.Piece = piece.Value.DeepClone();
        if (DA.GetData(3, ref designPiece)) model.DesignPiece = designPiece.Value.DeepClone();
        if (DA.GetData(4, ref connector)) model.Connector = connector.Value.DeepClone();
    }

    protected override void SetModelData(IGH_DataAccess DA, Side model)
    {
        DA.SetData(2, new PieceIdGoo(model.Piece.DeepClone()));
        DA.SetData(3, model.DesignPiece is not null ? new PieceIdGoo(model.DesignPiece.DeepClone()) : null);
        DA.SetData(4, new ConnectorIdGoo(model.Connector.DeepClone()));
    }
}

public class SerializeSideComponent : SerializeComponent<SideParam, SideGoo, Side>
{
    public SerializeSideComponent() { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E8");
}

public class DeserializeSideComponent : DeserializeComponent<SideParam, SideGoo, Side>
{
    public DeserializeSideComponent() { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E9");
}

public class SideDiffGoo : DiffGoo<SideDiff>
{
    public SideDiffGoo() { }
    public SideDiffGoo(SideDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("SideDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<SideDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class SideDiffParam : DiffParam<SideDiffGoo, SideDiff>
{
    protected override string ModelName => "SideDiff";
    protected override string ModelNickname => "SD";
    protected override string ModelDescription => "Side diff";
    protected override string IconResourceName => "sidediff_24x24";
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E3");
}

public class SideDiffComponent : DiffComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E4");
    protected override string ModelName => "SideDiff";
    protected override string ModelNickname => "SD";
    protected override string ModelDescription => "Construct, deconstruct or modify a side diff.";
    protected override string IconResourceName => "sidediff_24x24";
}

public class SerializeSideDiffComponent : SerializeComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public SerializeSideDiffComponent() { }
    public override Guid ComponentGuid => new("B1C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
}

public class DeserializeSideDiffComponent : DeserializeComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public DeserializeSideDiffComponent() { }
    public override Guid ComponentGuid => new("B2C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E6");
}

#endregion 🔖Side

#region 🔖Connection

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖connection](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/CONNECTION)
// Implementations MUST link two sides to connect pieces in a design.

public class ConnectionGoo : Goo<Connection>
{
    public ConnectionGoo() { }
    public ConnectionGoo(Connection value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionDiffGoo)))
        {
            target = (Q)(object)new ConnectionDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionIdGoo)))
        {
            target = (Q)(object)new ConnectionIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectionDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is ConnectionIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Connection>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator ConnectionIdGoo(ConnectionGoo goo) => new((ConnectionId)goo.Value);
    public static implicit operator ConnectionDiffGoo(ConnectionGoo goo) => new((ConnectionDiff)goo.Value);
    public static implicit operator ConnectionGoo(ConnectionIdGoo idGoo) => new((Connection)idGoo.Value);
    public static implicit operator ConnectionGoo(ConnectionDiffGoo diffGoo) => new((Connection)diffGoo.Value);
}

public class ConnectionParam : Param<ConnectionGoo, Connection>
{
    protected override string ModelName => "Connection";
    protected override string ModelNickname => "Cnx";
    protected override string ModelDescription => "Piece connection";
    protected override string IconResourceName => "connection_24x24";
    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");
}

public class ConnectionComponent : PassthroughComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");
    protected override string ModelName => "Connection";
    protected override string ModelNickname => "Con";
    protected override string ModelDescription => "Construct, deconstruct or modify a connection.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the connection.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connected", "Cd", "The connected side.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connecting", "Cg", "The connecting side.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "Gp", "The longitudinal gap.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "Sf", "The lateral shift.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "Rs", "The vertical rise.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt", "The rotation around y-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "Tn", "The turn around z-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl", "The tilt around x-axis.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the connection.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connected", "Cd", "The connected side.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connecting", "Cg", "The connecting side.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "Gp", "The longitudinal gap.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "Sf", "The lateral shift.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "Rs", "The vertical rise.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt", "The rotation around y-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "Tn", "The turn around z-axis.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl", "The tilt around x-axis.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Connection model)
    {
        string guid = "", description = "";
        var connected = new SideGoo();
        var connecting = new SideGoo();
        double gap = 0, shift = 0, rise = 0, rotation = 0, turn = 0, tilt = 0;
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref connected)) model.Connected = connected.Value.DeepClone();
        if (DA.GetData(4, ref connecting)) model.Connecting = connecting.Value.DeepClone();
        if (DA.GetData(5, ref description)) model.Description = description;
        if (DA.GetData(6, ref gap)) model.Gap = (float)gap;
        if (DA.GetData(7, ref shift)) model.Shift = (float)shift;
        if (DA.GetData(8, ref rise)) model.Rise = (float)rise;
        if (DA.GetData(9, ref rotation)) model.Rotation = (float)rotation;
        if (DA.GetData(10, ref turn)) model.Turn = (float)turn;
        if (DA.GetData(11, ref tilt)) model.Tilt = (float)tilt;
        if (DA.GetDataList(12, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Connection model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, new SideGoo(model.Connected.DeepClone()));
        DA.SetData(4, new SideGoo(model.Connecting.DeepClone()));
        DA.SetData(5, model.Description);
        DA.SetData(6, model.Gap);
        DA.SetData(7, model.Shift);
        DA.SetData(8, model.Rise);
        DA.SetData(9, model.Rotation);
        DA.SetData(10, model.Turn);
        DA.SetData(11, model.Tilt);
        DA.SetDataList(12, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }
}

public class SerializeConnectionComponent : SerializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public SerializeConnectionComponent() { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD60");
}

public class DeserializeConnectionComponent : DeserializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public DeserializeConnectionComponent() { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD61");
}

public class ConnectionIdGoo : IdGoo<ConnectionId>
{
    public ConnectionIdGoo() { }
    public ConnectionIdGoo(ConnectionId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionGoo)))
        {
            target = (Q)(object)new ConnectionGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionDiffGoo)))
        {
            target = (Q)(object)new ConnectionDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectionGoo connectionGoo)
        {
            Value = connectionGoo.Value;
            return true;
        }
        if (source is ConnectionDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectionId>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator ConnectionGoo(ConnectionIdGoo idGoo) => new(idGoo.Value);
    public static implicit operator ConnectionDiffGoo(ConnectionIdGoo idGoo) => new(idGoo.Value);
    public static implicit operator ConnectionIdGoo(ConnectionGoo goo) => new((ConnectionId)goo.Value);
    public static implicit operator ConnectionIdGoo(ConnectionDiffGoo diffGoo) => new((ConnectionId)diffGoo.Value);
}

public class ConnectionIdParam : IdParam<ConnectionIdGoo, ConnectionId>
{
    protected override string ModelName => "ConnectionId";
    protected override string ModelNickname => "CId";
    protected override string ModelDescription => "Connection identifier";
    protected override string IconResourceName => "connection_24x24";
    protected override string IdIconResourceName => "connectionid_24x24";
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D6");
}

public class ConnectionDiffGoo : DiffGoo<ConnectionDiff>
{
    public ConnectionDiffGoo() { }
    public ConnectionDiffGoo(ConnectionDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionGoo)))
        {
            target = (Q)(object)new ConnectionGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(ConnectionIdGoo)))
        {
            target = (Q)(object)new ConnectionIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ConnectionDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is ConnectionGoo connectionGoo)
        {
            Value = connectionGoo.Value;
            return true;
        }
        if (source is ConnectionIdGoo connectionIdGoo)
        {
            Value = connectionIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectionDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ConnectionDiffParam : DiffParam<ConnectionDiffGoo, ConnectionDiff>
{
    protected override string ModelName => "ConnectionDiff";
    protected override string ModelNickname => "CD";
    protected override string ModelDescription => "Connection diff";
    protected override string IconResourceName => "connectiondiff_24x24";
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F4");
}

public class ConnectionDiffComponent : DiffComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F5");
    protected override string ModelName => "ConnectionDiff";
    protected override string ModelNickname => "CD";
    protected override string ModelDescription => "Construct, deconstruct or modify a connection diff.";
    protected override string IconResourceName => "connectiondiff_24x24";
}

public class SerializeConnectionDiffComponent : SerializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public SerializeConnectionDiffComponent() { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F6");
}

public class DeserializeConnectionDiffComponent : DeserializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public DeserializeConnectionDiffComponent() { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F7");
}

public class ConnectionsDiffGoo : DiffGoo<ConnectionsDiff>
{
    public ConnectionsDiffGoo() { }
    public ConnectionsDiffGoo(ConnectionsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ConnectionsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<ConnectionsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class ConnectionsDiffParam : DiffParam<ConnectionsDiffGoo, ConnectionsDiff>
{
    protected override string ModelName => "ConnectionsDiff";
    protected override string ModelNickname => "CDs";
    protected override string ModelDescription => "Connection collection diff";
    protected override string IconResourceName => "connectionsdiff_24x24";
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D8");
}

public class ConnectionsDiffComponent : DiffComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D9");
    protected override string ModelName => "ConnectionsDiff";
    protected override string ModelNickname => "CDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of connection diffs.";
    protected override string IconResourceName => "connectionsdiff_24x24";
}

public class SerializeConnectionsDiffComponent : SerializeComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public SerializeConnectionsDiffComponent() { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7DA");
}

public class DeserializeConnectionsDiffComponent : DeserializeComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public DeserializeConnectionsDiffComponent() { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7DB");
}

#endregion 🔖Connection

#region 🔖Stat

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖stat](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/STAT)
// Implementations MUST associate statistical metrics with a design.

public class StatGoo : Goo<Stat>
{
    public StatGoo() { }
    public StatGoo(Stat value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToHumanIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<Stat>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class StatParam : Param<StatGoo, Stat>
{
    protected override string ModelName => "Stat";
    protected override string ModelNickname => "Sta";
    protected override string ModelDescription => "Design statistic";
    protected override string IconResourceName => "stat_24x24";
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class StatComponent : PassthroughComponent<StatParam, StatGoo, Stat>
{
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override string ModelName => "Stat";
    protected override string ModelNickname => "Stt";
    protected override string ModelDescription => "Construct, deconstruct or modify a stat.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new QualityIdParam(), "Quality", "Ql?", "The quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi?", "The optional minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE?", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx?", "The optional maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE?", "Whether max is excluded.", GH_ParamAccess.item);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityIdParam(), "Quality", "Ql?", "The quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut?", "The optional unit.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Min", "Mi?", "The optional minimum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MinExcluded", "MiE?", "Whether min is excluded.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Max", "Mx?", "The optional maximum value.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("MaxExcluded", "MxE?", "Whether max is excluded.", GH_ParamAccess.item);
    }

    protected override void GetModelData(IGH_DataAccess DA, Stat model)
    {
        QualityId quality = new();
        string unit = "";
        double min = 0, max = 0;
        bool minExcluded = false, maxExcluded = false;

        if (DA.GetData(2, ref quality)) model.Quality = quality;
        if (DA.GetData(3, ref unit)) model.Unit = unit;
        if (DA.GetData(4, ref min)) model.Min = (float)min;
        if (DA.GetData(5, ref minExcluded)) model.MinExcluded = minExcluded;
        if (DA.GetData(6, ref max)) model.Max = (float)max;
        if (DA.GetData(7, ref maxExcluded)) model.MaxExcluded = maxExcluded;
    }

    protected override void SetModelData(IGH_DataAccess DA, Stat model)
    {
        DA.SetData(2, new QualityIdGoo(model.Quality));
        DA.SetData(3, model.Unit);
        DA.SetData(4, model.Min);
        DA.SetData(5, model.MinExcluded);
        DA.SetData(6, model.Max);
        DA.SetData(7, model.MaxExcluded);
    }
}

public class SerializeStatComponent : SerializeComponent<StatParam, StatGoo, Stat>
{
    public SerializeStatComponent() { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeStatComponent : DeserializeComponent<StatParam, StatGoo, Stat>
{
    public DeserializeStatComponent() { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion 🔖Stat

#region 🔖Design

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖design](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/DESIGN)
// Implementations MUST compose pieces, connections, and metadata into a layout.

public class DesignGoo : Goo<Design>
{
    public DesignGoo() { }
    public DesignGoo(Design value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(DesignDiffGoo)))
        {
            target = (Q)(object)new DesignDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(DesignIdGoo)))
        {
            target = (Q)(object)new DesignIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is DesignDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is DesignIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Design { Name = str };
            return true;
        }
        return false;
    }
}

public class DesignParam : Param<DesignGoo, Design>
{
    protected override string ModelName => "Design";
    protected override string ModelNickname => "Des";
    protected override string ModelDescription => "Assembly design";
    protected override string IconResourceName => "design_24x24";
    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");
}

public class DesignComponent : PassthroughComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("AAD8D144-2EEE-48F1-A8A9-52977E86CB54");
    protected override string ModelName => "Design";
    protected override string ModelNickname => "Dsn";
    protected override string ModelDescription => "Construct, deconstruct or modify a design.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The length unit.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc*", "The optional pieces.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Co*", "The optional connections.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "The length unit.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc*", "The optional pieces.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Co*", "The optional connections.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorIdParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Design model)
    {
        string guid = "", name = "", description = "", icon = "", image = "", unit = "";
        var pieces = new List<PieceGoo>();
        var connections = new List<ConnectionGoo>();
        var authors = new List<AuthorIdGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref description)) model.Description = description;
        if (DA.GetData(5, ref icon)) model.Icon = icon;
        if (DA.GetData(6, ref image)) model.Image = image;
        if (DA.GetData(7, ref unit)) model.Unit = unit;
        if (DA.GetDataList(8, pieces)) model.Pieces = pieces.Select(p => p.Value.DeepClone()).ToList();
        if (DA.GetDataList(9, connections)) model.Connections = connections.Select(c => c.Value.DeepClone()).ToList();
        if (DA.GetDataList(10, authors)) model.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(11, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Design model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Description);
        DA.SetData(5, model.Icon);
        DA.SetData(6, model.Image);
        DA.SetData(7, model.Unit);
        DA.SetDataList(8, model.Pieces?.Select(p => new PieceGoo(p.DeepClone())).ToList());
        DA.SetDataList(9, model.Connections?.Select(c => new ConnectionGoo(c.DeepClone())).ToList());
        DA.SetDataList(10, model.Authors?.Select(a => new AuthorIdGoo(a.DeepClone())).ToList());
        DA.SetDataList(11, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }

    protected override Design ProcessModel(Design design)
    {
        if (design.Unit == "")
            try { design.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem); }
            catch (Exception) { design.Unit = "m"; }
        design.Icon = design.Icon.Replace('\\', '/');
        design.Image = design.Image.Replace('\\', '/');
        return design;
    }
}

public class SerializeDesignComponent : SerializeComponent<DesignParam, DesignGoo, Design>
{
    public SerializeDesignComponent() { }
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");
}

public class DeserializeDesignComponent : DeserializeComponent<DesignParam, DesignGoo, Design>
{
    public DeserializeDesignComponent() { }
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB59");
}

public class DesignIdGoo : IdGoo<DesignId>
{
    public DesignIdGoo() { }
    public DesignIdGoo(DesignId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(DesignGoo)))
        {
            target = (Q)(object)new DesignGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(DesignDiffGoo)))
        {
            target = (Q)(object)new DesignDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToHumanIdString());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is DesignGoo designGoo)
        {
            Value = designGoo.Value;
            return true;
        }
        if (source is DesignDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new DesignId { Guid = str };
            return true;
        }
        return false;
    }
}

public class DesignIdParam : IdParam<DesignIdGoo, DesignId>
{
    protected override string ModelName => "DesignId";
    protected override string ModelNickname => "DId";
    protected override string ModelDescription => "Design identifier";
    protected override string IconResourceName => "design_24x24";
    protected override string IdIconResourceName => "designid_24x24";
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A6");
}

public class DesignDiffGoo : DiffGoo<DesignDiff>
{
    public DesignDiffGoo() { }
    public DesignDiffGoo(DesignDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(DesignGoo)))
        {
            target = (Q)(object)new DesignGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(DesignIdGoo)))
        {
            target = (Q)(object)new DesignIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is DesignGoo designGoo)
        {
            Value = designGoo.Value;
            return true;
        }
        if (source is DesignIdGoo designIdGoo)
        {
            Value = designIdGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<DesignDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class DesignDiffParam : DiffParam<DesignDiffGoo, DesignDiff>
{
    protected override string ModelName => "DesignDiff";
    protected override string ModelNickname => "DD";
    protected override string ModelDescription => "Design diff";
    protected override string IconResourceName => "designdiff_24x24";
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A5");
}

public class DesignDiffComponent : DiffComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A8");
    protected override string ModelName => "DesignDiff";
    protected override string ModelNickname => "DD";
    protected override string ModelDescription => "Construct, deconstruct or modify a design diff.";
    protected override string IconResourceName => "designdiff_24x24";
}

public class SerializeDesignDiffComponent : SerializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public SerializeDesignDiffComponent() { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A9");
}

public class DeserializeDesignDiffComponent : DeserializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public DeserializeDesignDiffComponent() { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4AA");
}

public class DesignsDiffGoo : DiffGoo<DesignsDiff>
{
    public DesignsDiffGoo() { }
    public DesignsDiffGoo(DesignsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("DesignsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<DesignsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class DesignsDiffParam : DiffParam<DesignsDiffGoo, DesignsDiff>
{
    protected override string ModelName => "DesignsDiff";
    protected override string ModelNickname => "DDs";
    protected override string ModelDescription => "Design collection diff";
    protected override string IconResourceName => "designsdiff_24x24";
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8E9");
}

public class DesignsDiffComponent : DiffComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EA");
    protected override string ModelName => "DesignsDiff";
    protected override string ModelNickname => "DDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of design diffs.";
    protected override string IconResourceName => "designsdiff_24x24";
}

public class SerializeDesignsDiffComponent : SerializeComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public SerializeDesignsDiffComponent() { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EB");
}

public class DeserializeDesignsDiffComponent : DeserializeComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public DeserializeDesignsDiffComponent() { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EC");
}

#endregion 🔖Design

#region 🔖Kit

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖kit](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/KIT)
// Implementations MUST collect types and designs into a reusable library.

public class KitGoo : Goo<Kit>
{
    public KitGoo() { }
    public KitGoo(Kit value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(KitDiffGoo)))
        {
            target = (Q)(object)new KitDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is KitDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Kit { Name = str };
            return true;
        }
        return false;
    }
}

public class KitParam : Param<KitGoo, Kit>
{
    protected override string ModelName => "Kit";
    protected override string ModelNickname => "Kit";
    protected override string ModelDescription => "Component library";
    protected override string IconResourceName => "kit_24x24";
    public override Guid ComponentGuid => new("BA9F161E-AFE3-41D5-8644-964DD20B887B");
}

public class KitComponent : PassthroughComponent<KitParam, KitGoo, Kit>
{
    public override Guid ComponentGuid => new("987560A8-10D4-43F6-BEBE-D71DC2FD86AF");
    protected override string ModelName => "Kit";
    protected override string ModelNickname => "Kit";
    protected override string ModelDescription => "Construct, deconstruct or modify a kit.";

    protected override string IconResourceName => "file_24x24";

    protected override void RegisterModelInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "Vr?", "The optional version.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Hp?", "The optional homepage url.", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "Li?", "The optional license.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeParam(), "Types", "Ty*", "The optional types.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Dn*", "The optional designs.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void RegisterModelOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Guid", "Gd", "The guid of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "The name of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Version", "Vr?", "The optional version.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional description.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "The optional icon.", GH_ParamAccess.item);
        pManager.AddTextParameter("Image", "Im?", "The optional image url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Remote", "Rm?", "The optional remote url.", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Hp?", "The optional homepage url.", GH_ParamAccess.item);
        pManager.AddTextParameter("License", "Li?", "The optional license.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeParam(), "Types", "Ty*", "The optional types.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Dn*", "The optional designs.", GH_ParamAccess.list);
        pManager.AddParameter(new AuthorParam(), "Authors", "Au*", "The optional authors.", GH_ParamAccess.list);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes.", GH_ParamAccess.list);
    }

    protected override void GetModelData(IGH_DataAccess DA, Kit model)
    {
        string guid = "", name = "", version = "", description = "", icon = "", image = "", remote = "", homepage = "", license = "";
        var types = new List<TypeGoo>();
        var designs = new List<DesignGoo>();
        var authors = new List<AuthorGoo>();
        var attributes = new List<AttributeGoo>();

        if (DA.GetData(2, ref guid)) model.Guid = guid;
        if (DA.GetData(3, ref name)) model.Name = name;
        if (DA.GetData(4, ref version)) model.Version = version;
        if (DA.GetData(5, ref description)) model.Description = description;
        if (DA.GetData(6, ref icon)) model.Icon = icon;
        if (DA.GetData(7, ref image)) model.Image = image;
        if (DA.GetData(8, ref remote)) model.Remote = remote;
        if (DA.GetData(9, ref homepage)) model.Homepage = homepage;
        if (DA.GetData(10, ref license)) model.License = license;
        if (DA.GetDataList(11, types)) model.Types = types.Select(t => t.Value.DeepClone()).ToList();
        if (DA.GetDataList(12, designs)) model.Designs = designs.Select(d => d.Value.DeepClone()).ToList();
        if (DA.GetDataList(13, authors)) model.Authors = authors.Select(a => a.Value.DeepClone()).ToList();
        if (DA.GetDataList(14, attributes)) model.Attributes = attributes.Select(a => a.Value.DeepClone()).ToList();
    }

    protected override void SetModelData(IGH_DataAccess DA, Kit model)
    {
        DA.SetData(2, model.Guid);
        DA.SetData(3, model.Name);
        DA.SetData(4, model.Version);
        DA.SetData(5, model.Description);
        DA.SetData(6, model.Icon);
        DA.SetData(7, model.Image);
        DA.SetData(8, model.Remote);
        DA.SetData(9, model.Homepage);
        DA.SetData(10, model.License);
        DA.SetDataList(11, model.Types?.Select(t => new TypeGoo(t.DeepClone())).ToList());
        DA.SetDataList(12, model.Designs?.Select(d => new DesignGoo(d.DeepClone())).ToList());
        DA.SetDataList(13, model.Authors?.Select(a => new AuthorGoo(a.DeepClone())).ToList());
        DA.SetDataList(14, model.Attributes?.Select(a => new AttributeGoo(a.DeepClone())).ToList());
    }

    protected override Kit ProcessModel(Kit kit)
    {
        kit.Icon = kit.Icon.Replace('\\', '/');
        kit.Image = kit.Image.Replace('\\', '/');
        kit.Preview = kit.Preview.Replace('\\', '/');
        return kit;
    }
}

public class SerializeKitComponent : SerializeComponent<KitParam, KitGoo, Kit>
{
    public SerializeKitComponent() { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4165");
}

public class DeserializeKitComponent : DeserializeComponent<KitParam, KitGoo, Kit>
{
    public DeserializeKitComponent() { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4166");
}

public class KitIdGoo : IdGoo<KitId>
{
    public KitIdGoo() { }
    public KitIdGoo(KitId value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Guid);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is KitDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is KitGoo kitGoo)
        {
            Value = kitGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new KitId { Guid = str };
            return true;
        }
        return false;
    }
}

public class KitIdParam : IdParam<KitIdGoo, KitId>
{
    protected override string ModelName => "KitId";
    protected override string ModelNickname => "KId";
    protected override string ModelDescription => "Kit identifier";
    protected override string IconResourceName => "kit_24x24";
    protected override string IdIconResourceName => "kitid_24x24";
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B0");
}

public class KitDiffGoo : DiffGoo<KitDiff>
{
    public KitDiffGoo() { }
    public KitDiffGoo(KitDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(KitGoo)))
        {
            target = (Q)(object)new KitGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (source is KitGoo kitGoo)
        {
            Value = kitGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<KitDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class KitDiffParam : DiffParam<KitDiffGoo, KitDiff>
{
    protected override string ModelName => "KitDiff";
    protected override string ModelNickname => "KD";
    protected override string ModelDescription => "Kit diff";
    protected override string IconResourceName => "kitdiff_24x24";
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B2");
}

public class KitDiffComponent : DiffComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B3");
    protected override string ModelName => "KitDiff";
    protected override string ModelNickname => "KD";
    protected override string ModelDescription => "Construct, deconstruct or modify a kit diff.";
    protected override string IconResourceName => "kitdiff_24x24";
}

public class SerializeKitDiffComponent : SerializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public SerializeKitDiffComponent() { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B4");
}

public class DeserializeKitDiffComponent : DeserializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public DeserializeKitDiffComponent() { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B5");
}

public class KitsDiffGoo : DiffGoo<KitsDiff>
{
    public KitsDiffGoo() { }
    public KitsDiffGoo(KitsDiff value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("KitsDiff");
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source is null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                var deserialized = str.Deserialize<KitsDiff>();
                if (deserialized is null) return false;
                Value = deserialized;
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class KitsDiffParam : DiffParam<KitsDiffGoo, KitsDiff>
{
    protected override string ModelName => "KitsDiff";
    protected override string ModelNickname => "KDs";
    protected override string ModelDescription => "Kit collection diff";
    protected override string IconResourceName => "kitsdiff_24x24";
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C3");
}

public class KitsDiffComponent : DiffComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C4");
    protected override string ModelName => "KitsDiff";
    protected override string ModelNickname => "KDs";
    protected override string ModelDescription => "Construct, deconstruct or modify a collection of kit diffs.";
    protected override string IconResourceName => "kitsdiff_24x24";
}

public class SerializeKitsDiffComponent : SerializeComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public SerializeKitsDiffComponent() { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C5");
}

public class DeserializeKitsDiffComponent : DeserializeComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public DeserializeKitsDiffComponent() { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C6");
}

#endregion 🔖Kit

#region 🔖Scripting

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖scripting](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/SCRIPTING)
// Callers MUST use these helpers for C# script component integration.

public abstract class ScriptingComponent : Component
{
    public ScriptingComponent(string name, string nickname, string description)
        : base(name, nickname, description, "Scripting")
    { }
}

public class EncodeTextComponent : ScriptingComponent
{
    public EncodeTextComponent()
        : base("Encode Text", ">Txt", "Encode a text.")
    { }
    public override Guid ComponentGuid => new("FBDDF723-80BD-4AF9-A1EE-450A27D50ABE");

    protected override Bitmap Icon => Resources.encode_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text to encode.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Mode", "Mo", "0: url safe encoding ()\n1: base64 encoding\n2: replace only", GH_ParamAccess.item, 0);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Forbidden", "Fb", "Forbidden text that will be replaced after encoding.", GH_ParamAccess.list);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Replace", "Re", "Placeholder text that replaces the forbidden text after encoding.", GH_ParamAccess.list);
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Encoded Text", "En", "Encoded text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        var mode = 0;
        var forbidden = new List<string>();
        var replace = new List<string>();
        DA.GetData(0, ref text);
        DA.GetData(1, ref mode);
        DA.GetDataList(2, forbidden);
        DA.GetDataList(3, replace);
        DA.SetData(0, Semio.Utility.Encode(text, (EncodeMode)mode, new Tuple<List<string>, List<string>>(forbidden, replace)));
    }
}

public class DecodeTextComponent : ScriptingComponent
{
    public DecodeTextComponent() : base("Decode Text", "<Txt", "Decode a text.") { }
    public override Guid ComponentGuid => new("E7158D28-87DE-493F-8D78-923265C3E211");
    protected override Bitmap Icon => Resources.decode_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Encoded Text", "En", "Encoded text to decode.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Mode", "Mo", "0: url safe encoding ()\n1: base64 encoding\n2: replace only", GH_ParamAccess.item, 0);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Replace", "Re", "Placeholder text that was used to encode forbidden text after encoding and is restored before decoding. It will be applied sequentially. Make sure to invert the order of your original list.", GH_ParamAccess.list);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Forbidden", "Fb", "Forbidden text that gets restored before decoding.", GH_ParamAccess.list);
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Decoded text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        var mode = 0;
        var replace = new List<string>();
        var forbidden = new List<string>();
        DA.GetData(0, ref text);
        DA.GetData(1, ref mode);
        DA.GetDataList(2, replace);
        DA.GetDataList(3, forbidden);
        DA.SetData(0, Semio.Utility.Decode(text, (EncodeMode)mode, new Tuple<List<string>, List<string>>(replace, forbidden)));
    }
}

public class ObjectsToTextComponent : ScriptingComponent
{
    public ObjectsToTextComponent() : base("Objects to Text", "Objs→Txt", "Converts a list of objects to a human-readable text.") { }
    public override Guid ComponentGuid => new("3BE61561-8290-4965-A9A6-38ACB4EC5182");
    protected override Bitmap Icon => Resources.objects_convert_text_24x24;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddGenericParameter("Objects", "Ob+", "Objects to humanize.", GH_ParamAccess.list);
    }
    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Humanized Text", "Tx", "Human-readable text.", GH_ParamAccess.item);
    }
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var objects = new List<object>();
        DA.GetDataList(0, objects);
        var humanizedText = objects.Humanize();
        DA.SetData(0, humanizedText);
    }
}

public class NormalizeTextComponent : ScriptingComponent
{
    public NormalizeTextComponent() : base("Normalize Text", "⇒Txt", "Normalizes a text to different formats.") { }
    public override Guid ComponentGuid => new("1417BD04-7271-4EFD-A32C-99B1D2FC8A9E");
    protected override Bitmap Icon => Resources.text_normalize_24x24;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text to normalize.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Strict", "St", "Strictly alphanumerical text that either strips characters or turn them into underscores.", GH_ParamAccess.item);
        pManager.AddTextParameter("Title", "Ti", "Titelized text by capitalizing and unifying casing.", GH_ParamAccess.item);
        pManager.AddTextParameter("Underscore", "Un", "Underscorized text by lowercasing everything and replacing spaces with underscores.", GH_ParamAccess.item);
        pManager.AddTextParameter("Kebab", "Kb", "Kebaberized text by lowercasing everything and replacing spaces with dashes.", GH_ParamAccess.item);
        pManager.AddTextParameter("Pascal", "Pa", "Pascalized text by capitalizing and removing spaces.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        DA.GetData(0, ref text);
        var strict = Regex.Replace(text.Dehumanize().Underscore(), @"[^a-zA-Z0-9_]", "");
        var title = text.Titleize();
        var underscore = text.Underscore();
        var kebab = text.Kebaberize();
        var pascal = text.Pascalize();
        DA.SetData(0, strict);
        DA.SetData(1, title);
        DA.SetData(2, underscore);
        DA.SetData(3, kebab);
        DA.SetData(4, pascal);
    }
}

public class TruncateTextComponent : ScriptingComponent
{
    public TruncateTextComponent() : base("Truncate Text", "…Txt", "Truncates text by length and an optional termination.") { }
    public override Guid ComponentGuid => new("C15BFCE9-0EF7-4367-8310-EF47CE0B8013");
    protected override Bitmap Icon => Resources.text_truncate_24x24;
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text to truncate.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Length", "Le", "Maximum length of the text.", GH_ParamAccess.item);
        pManager.AddTextParameter("Termination", "Tr", "Optional termination to append to the truncated text.", GH_ParamAccess.item, "…");
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Strict", "St", "Fixed length truncated text including the truncation text length.", GH_ParamAccess.item);
        pManager.AddTextParameter("Characters", "Crs", "Fixed alphanumeric character length truncated text including the truncation text length", GH_ParamAccess.item);
        pManager.AddTextParameter("Words", "Wds", "Fixed word length truncated text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        string text = "";
        var length = 0;
        var termination = "…";
        DA.GetData(0, ref text);
        DA.GetData(1, ref length);
        DA.GetData(2, ref termination);
        var strict = text.Truncate(length, termination, Truncator.FixedLength);
        var characters = text.Truncate(length, termination, Truncator.FixedNumberOfCharacters);
        var words = text.Truncate(length, termination, Truncator.FixedNumberOfWords);
        DA.SetData(0, strict);
        DA.SetData(1, characters);
        DA.SetData(2, words);
    }
}

#endregion 🔖Scripting

#region 🔖Engine

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖engine](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/ENGINE)
// Implementations MUST communicate with the engine for kit and design operations.

public abstract class EngineComponent : Component
{
    protected EngineComponent(string name, string nickname, string description, string subcategory = "Persistence") : base(name, nickname, description, subcategory) { }
    protected virtual string RunDescription => "True to start the operation.";
    protected virtual string SuccessDescription => "True if the operation was successful.";
    protected virtual void RegisterEngineInputParams(GH_InputParamManager pManager) { }
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        RegisterEngineInputParams(pManager);
        pManager.AddBooleanParameter("Run", "R", RunDescription, GH_ParamAccess.item, false);
    }
    protected virtual void RegisterEngineOutputParams(GH_OutputParamManager pManager) { }
    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", SuccessDescription, GH_ParamAccess.item);
        RegisterEngineOutputParams(pManager);
    }
    protected virtual dynamic? GetInput(IGH_DataAccess DA) => null;
    protected abstract dynamic? Run(dynamic? input = null);
    protected virtual void SetOutput(IGH_DataAccess DA, dynamic response) { }
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var run = false;
        DA.GetData(Params.Input.Count - 1, ref run);
        if (!run) return;
        var input = GetInput(DA);
        try
        {
            var response = Run(input);
            SetOutput(DA, response);
            DA.SetData(0, true);
        }
        catch (ClientException e)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, e.Message);
            DA.SetData(0, false);
        }
        catch (ServerException e)
        {
            string serializedInput = input != null ? Semio.Utility.Serialize(input) : "";
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error,
                "The engine didn't like it ¯\\_(ツ)_/¯\n" +
                $"Or write me an email: {Semio.Constants.Email}\n\n" +
                "ServerError: " + e.Message + "\n" +
                "Semio.Release: " + Semio.Constants.Release + "\n" +
                "Semio.Grasshopper: " + Constants.Version + "\n" +
                (serializedInput != ""
                    ? "Input: " + (serializedInput.Length < 1000
                        ? serializedInput
                        : serializedInput.Substring(0, 1000) + "\n...\n")
                    : ""));
            DA.SetData(0, false);
        }
    }
    protected override void BeforeSolveInstance()
    {
        base.BeforeSolveInstance();
        var processes = Process.GetProcessesByName("semio-engine");
        if (processes.Length == 0)
        {
            var path = Path.Combine(Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location) ?? string.Empty,
                "semio-engine.exe");
            var engine = Process.Start(path);

            AppDomain.CurrentDomain.DomainUnload += (s, e) =>
            {
                engine.Kill();
                engine.WaitForExit();
            };
            AppDomain.CurrentDomain.ProcessExit += (s, e) =>
            {
                engine.Kill();
                engine.WaitForExit();
            };
            AppDomain.CurrentDomain.UnhandledException += (s, e) =>
            {
                engine.Kill();
                engine.WaitForExit();
            };
            // TODO: Implement a status check and wait for the engine to be ready
            Thread.Sleep(1000);
        }
    }
}

#region 🔖Persistence

// [👤semio📚gh🛅semiograsshopper💻semiograsshoppercs🔖engine🔖persistence](semiorepo://section/SEMIO/GH/SEMIO.GRASSHOPPER/SEMIO.GRASSHOPPER.CS/ENGINE/PERSISTENCE)
// Implementations MUST persist Grasshopper documents for saving and loading kits.

public abstract class PersistenceComponent : EngineComponent
{
    protected PersistenceComponent(string name, string nickname, string description, string subcategory = "Persistence") : base(name, nickname, description, subcategory) { }
    protected virtual void RegisterPersitenceInputParams(GH_InputParamManager pManager) { }
    protected override void RegisterEngineInputParams(GH_InputParamManager pManager)
    {
        RegisterPersitenceInputParams(pManager);
        var amountCustomParams = pManager.ParamCount;
        pManager.AddTextParameter("Uri", "Ur?",
            "Optional Unique Resource Identifier (URI) of the kit. This can be an absolute path to a local kit or a url to a remote kit.\n" +
            "If none is provided, it will try to see if the Grasshopper script is executed inside a local kit.",
            GH_ParamAccess.item);
        pManager[amountCustomParams].Optional = true;
    }
    protected virtual void RegisterPersitenceOutputParams(GH_OutputParamManager pManager) { }
    protected override void RegisterEngineOutputParams(GH_OutputParamManager pManager)
    {
        RegisterPersitenceOutputParams(pManager);
    }
    protected virtual dynamic? GetPersistentInput(IGH_DataAccess DA) => null;
    protected override dynamic? GetInput(IGH_DataAccess DA)
    {
        var uri = "";
        if (!DA.GetData(Params.Input.Count - 2, ref uri))
            uri = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        return new { Uri = uri, Input = GetPersistentInput(DA) };
    }
    protected abstract dynamic? RunOnKit(string url, dynamic? input);
    protected override dynamic? Run(dynamic? input = null) => input is not null ? RunOnKit(input.Uri, input.Input) : null;
}

public class LoadKitComponent : PersistenceComponent
{
    public LoadKitComponent() : base("Load Kit", "/Kit", "Load a kit.") { }
    protected override string RunDescription => "True to load the kit.";
    protected override string SuccessDescription => "True if the kit was successfully loaded. False otherwise.";
    public override Guid ComponentGuid => new("5BE3A651-581E-4595-8DAC-132F10BD87FC");
    protected override Bitmap Icon => Resources.kit_load_24x24;
    public override GH_Exposure Exposure => GH_Exposure.primary;
    protected override void RegisterPersitenceOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam());
        pManager.AddTextParameter("Local Directory", "Di",
            "The optional local directory of the kit. This applies to local kits and cached remote kits.",
            GH_ParamAccess.item);
    }

    protected override dynamic? RunOnKit(string uri, dynamic? input) => Api.GetKit(uri);

    protected override void SetOutput(IGH_DataAccess DA, dynamic response)
    {
        DA.SetData(1, new KitGoo(response));
        var uri = "";
        DA.GetData(0, ref uri);
        string directory;
        if (uri == "")
        {
            directory = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        }
        else if (uri.StartsWith("http") && uri.EndsWith(".zip"))
        {
            var encodedUri = Semio.Utility.Encode(uri);
            var userPath = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
            directory = Path.Combine(userPath, ".semio", "cache", encodedUri);
        }
        else directory = uri;
        DA.SetData(2, directory);
    }
}

#endregion 🔖Persistence

#endregion 🔖Engine

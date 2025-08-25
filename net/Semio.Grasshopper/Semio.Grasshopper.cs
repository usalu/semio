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
        Instances.ComponentServer.AddCategoryIcon("semio", Resources.semio_24x24);
        Instances.ComponentServer.AddCategorySymbolName("semio", 'S');
        return GH_LoadingInstruction.Proceed;
    }
}

#endregion Constants

#region Utility

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
        //var gapDirectionR = new Vector3d(parentDirectionR);
        var reverseChildDirectionR = childDirection.Convert();
        reverseChildDirectionR.Reverse();
        var rotationRad = RhinoMath.ToRadians(rotation);
        var turnRad = RhinoMath.ToRadians(turn);
        var tiltRad = RhinoMath.ToRadians(tilt);

        // orient

        // If directions are same, then there are infinite solutions.
        var areDirectionsSame = parentDirectionR.IsParallelTo(childDirection.Convert(), Semio.Constants.Tolerance) == 1;

        // Rhino tends to pick the "wrong" direction when the vectors are parallel.
        // E.g when z=0 it picks to flip the object around the y-axis instead of a rotation around the z-axis.
        Transform directionT;
        if (areDirectionsSame)
        {
            // Idea taken from: // https://github.com/dfki-ric/pytransform3d/blob/143943b028fc776adfc6939b1d7c2c6edeaa2d90/pytransform3d/rotations/_utils.py#L253
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
        //gapDirection.Transform(rotateT);

        var turnT = Transform.Rotation(turnRad, turnAxis, new Point3d());
        orientationT = turnT * orientationT;
        //gapDirection.Transform(turnT);

        var tiltT = Transform.Rotation(tiltRad, tiltAxis, new Point3d());
        orientationT = tiltT * orientationT;
        //gapDirection.Transform(tiltT);

        // move

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

        // to parent

        var parentPlaneR = parentPlane.Convert();
        var parentPlaneT = Transform.PlaneToPlane(Rhino.Geometry.Plane.WorldXY, parentPlaneR);
        childPlaneR.Transform(parentPlaneT);

        return childPlaneR.Convert();
    }
}

#endregion Utility

#region Converters

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

#endregion Converters

#region Models

public abstract class ModelGoo<T> : GH_Goo<T> where T : Model<T>, new()
{
    public ModelGoo() { Value = new T(); }
    public ModelGoo(T value) { Value = value; }
    public override bool IsValid => true;
    public override string TypeName => typeof(T).Name;
    public override string TypeDescription => ((ModelAttribute?)System.Attribute.GetCustomAttribute(typeof(T), typeof(ModelAttribute)))?.Description ?? "";
    public override IGH_Goo Duplicate()
    {
        var duplicate = (ModelGoo<T>?)Activator.CreateInstance(GetType());
        if (duplicate != null) duplicate.Value = Value.DeepClone();
        return duplicate!;
    }
    public override string ToString() => Value.ToString();
    public override bool Write(GH_IWriter writer)
    {
        writer.SetString(typeof(T).Name, Value.Serialize());
        return base.Write(writer);
    }
    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString(typeof(T).Name).Deserialize<T>();
        return base.Read(reader);
    }
    internal virtual bool CustomCastTo<Q>(ref Q target) => false;
    internal virtual bool CustomCastFrom(object source) => false;
    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(T)))
        {
            target = (Q)(object)this;
            return true;
        }
        return CustomCastTo(ref target);
    }

    public override bool CastFrom(object source)
    {
        if (source == null) return false;
        if (source is T model)
        {
            Value = model;
            return true;
        }
        return CustomCastFrom(source);
    }
}

public abstract class ModelParam<T, U> : GH_PersistentParam<T> where T : ModelGoo<U> where U : Model<U>, new()
{
    internal ModelParam() : base(typeof(U).Name,
        ((ModelAttribute?)System.Attribute.GetCustomAttribute(typeof(U), typeof(ModelAttribute)))?.Code ?? "",
        ((ModelAttribute?)System.Attribute.GetCustomAttribute(typeof(U), typeof(ModelAttribute)))?.Description ?? "",
        Constants.Category, "Params")
    { }
    protected override Bitmap Icon => (Bitmap?)Resources.ResourceManager.GetObject($"{typeof(U).Name.ToLower()}_24x24") ?? Resources.semio_24x24;
    protected override GH_GetterResult Prompt_Singular(ref T value) => throw new NotImplementedException();
    protected override GH_GetterResult Prompt_Plural(ref List<T> values) => throw new NotImplementedException();
    public override GH_Exposure Exposure => GH_Exposure.primary;
}

public abstract class Component : GH_Component
{
    public Component(string name, string nickname, string description, string subcategory) : base(
        name, nickname, description, Constants.Category, subcategory)
    { }
}

public abstract class ModelComponent<T, U, V> : Component
    where T : ModelParam<U, V> where U : ModelGoo<V> where V : Model<V>, new()
{
    public static readonly string NameM;
    public static readonly System.Type TypeM;
    public static readonly System.Type GooM;
    public static readonly System.Type ParamM;
    public static readonly ModelAttribute ModelM;
    public static readonly ImmutableArray<PropertyInfo> PropertyM;
    public static readonly ImmutableArray<PropAttribute> PropM;
    public static readonly ImmutableArray<bool> IsPropertyList;
    public static readonly ImmutableArray<bool> IsPropertyMapped;
    public static readonly ImmutableArray<System.Type> PropertyItemType;
    public static readonly ImmutableArray<bool> IsPropertyModel;
    public static readonly ImmutableArray<System.Type> PropertyGooM;
    public static readonly ImmutableArray<System.Type> PropertyParamM;
    public static readonly ImmutableArray<System.Type> PropertyItemGoo;

    static ModelComponent()
    {
        var dummyMetaGrasshopper = Meta.Goo;
        NameM = typeof(V).Name;
        TypeM = Semio.Meta.Type[NameM];
        GooM = Meta.Goo[NameM];
        ParamM = Meta.Param[NameM];
        ModelM = Semio.Meta.Model[NameM];
        PropertyM = Semio.Meta.Property[NameM];
        PropM = Semio.Meta.Prop[NameM];
        IsPropertyList = Semio.Meta.IsPropertyList[NameM];
        IsPropertyMapped = Meta.IsPropertyMapped[NameM];
        PropertyItemType = Semio.Meta.PropertyItemType[NameM];
        PropertyItemGoo = Meta.PropertyItemGoo[NameM];
        IsPropertyModel = Semio.Meta.IsPropertyModel[NameM];
        PropertyGooM = Meta.PropertyGoo[NameM];
        PropertyParamM = Meta.PropertyParam[NameM];
    }

    protected ModelComponent() : base($"Model {NameM}", $"~{ModelM.Abbreviation}",
        $"Construct, deconstruct or modify {Semio.Utility.Grammar.GetArticle(NameM)} {NameM.ToLower()}", "Modeling")
    { }

    protected override Bitmap Icon =>
        (Bitmap?)Resources.ResourceManager.GetObject($"{typeof(V).Name.ToLower()}_modify_24x24") ?? Resources.semio_24x24;

    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected virtual void AddModelProps(dynamic pManager)
    {
        for (var i = 0; i < PropertyM.Length; i++)
        {
            var property = PropertyM[i];
            var propAttribute = PropM[i];
            var param = (IGH_Param?)Activator.CreateInstance(PropertyParamM[i]);
            if (param != null)
                pManager.AddParameter(param, property.Name, propAttribute.Code, propAttribute.Description,
                    IsPropertyList[i] ? GH_ParamAccess.list : GH_ParamAccess.item);
        }
    }

    protected void AddModelParameters(dynamic pManager, bool isOutput = false)
    {
        var modelParam = (IGH_Param?)Activator.CreateInstance(ParamM);
        var description = isOutput
            ? $"The constructed or modified {NameM.ToLower()}."
            : $"The optional {NameM.ToLower()} to deconstruct or modify.";
        if (modelParam != null)
            pManager.AddParameter(modelParam, NameM, isOutput ? ModelM.Code : ModelM.Code + "?",
                description, GH_ParamAccess.item);
        pManager.AddBooleanParameter(isOutput ? "Valid" : "Validate", "Vd?",
            isOutput
                ? $"True if the {NameM.ToLower()} is valid. Null if no validation was performed."
                : $"Whether the {NameM.ToLower()} should be validated.", GH_ParamAccess.item);

        AddModelProps(pManager);

        if (!isOutput)
            for (var i = 0; i < pManager.ParamCount; i++)
                pManager[i].Optional = true;
    }

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        AddModelParameters(pManager);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        AddModelParameters(pManager, true);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        dynamic? modelGoo = Activator.CreateInstance(GooM);
        if (modelGoo == null) return;
        var validate = false;
        if (DA.GetData(0, ref modelGoo!)) GetProps(DA, modelGoo);
        DA.GetData(1, ref validate);
        modelGoo!.Value = ProcessModel(modelGoo.Value);
        if (validate) modelGoo.Value.Validate();
        SetData(DA, modelGoo);
    }

    protected virtual void GetProps(IGH_DataAccess DA, dynamic modelGoo)
    {
        for (var i = 0; i < PropertyM.Length; i++)
        {
            var property = PropertyM[i];
            var isList = IsPropertyList[i];
            var isPropertyModel = IsPropertyModel[i];
            var isPropertyMapped = IsPropertyMapped[i];
            if (isList)
            {
                if (isPropertyModel)
                {
                    var goosList = new List<object>();
                    if (DA.GetDataList(i + 2, goosList))
                    {
                        var list = (System.Collections.IList?)Activator.CreateInstance(typeof(List<>).MakeGenericType(PropertyItemType[i]));
                        if (list != null)
                        {
                            foreach (var goo in goosList)
                            {
                                var value = goo?.GetType().GetProperty("Value")?.GetValue(goo);
                                if (value != null)
                                {
                                    var deepCloneMethod = value.GetType().GetMethod("DeepClone");
                                    var clonedValue = deepCloneMethod?.Invoke(value, null);
                                    if (clonedValue != null) list.Add(clonedValue);
                                }
                            }
                            property.SetValue(modelGoo.Value, list);
                        }
                    }
                }
                else
                {
                    var itemsList = new List<object>();
                    if (DA.GetDataList(i + 2, itemsList))
                    {
                        var list = (System.Collections.IList?)Activator.CreateInstance(typeof(List<>).MakeGenericType(PropertyItemType[i]));
                        if (list != null)
                        {
                            foreach (var item in itemsList)
                                if (item != null) list.Add(item);
                            property.SetValue(modelGoo.Value, list);
                        }
                    }
                }
            }
            else
            {
                if (isPropertyModel)
                {
                    dynamic? goo = Activator.CreateInstance(PropertyGooM[i]);
                    if (goo != null && DA.GetData(i + 2, ref goo!))
                    {
                        var value = goo!.Value;
                        if (value != null)
                        {
                            var deepCloneMethod = value.GetType().GetMethod("DeepClone");
                            var clonedValue = deepCloneMethod?.Invoke(value, null);
                            if (clonedValue != null) property.SetValue(modelGoo.Value, clonedValue);
                        }
                    }
                }
                else if (isPropertyMapped)
                {
                    dynamic? data = Activator.CreateInstance(PropertyItemType[i]);
                    if (data != null && DA.GetData(i + 2, ref data!))
                    {
                        var convertMethod = typeof(RhinoConverter).GetMethod("Convert", new System.Type[] { data!.GetType() });
                        var convertedValue = convertMethod?.Invoke(null, new object[] { data });
                        if (convertedValue != null) property.SetValue(modelGoo.Value, convertedValue);
                    }
                }
                else
                {
                    dynamic? data = Activator.CreateInstance(PropertyItemType[i]);
                    if (data != null && DA.GetData(i + 2, ref data!)) property.SetValue(modelGoo.Value, data);
                }
            }
        }
    }

    protected virtual void SetData(IGH_DataAccess DA, dynamic modelGoo)
    {
        DA.SetData(0, modelGoo);
        DA.SetData(1, modelGoo.Value.IsValid);
        for (var i = 0; i < PropertyM.Length; i++)
        {
            var property = PropertyM[i];
            var isList = IsPropertyList[i];
            var isPropertyModel = IsPropertyModel[i];
            var isPropertyMapped = IsPropertyMapped[i];
            var value = property.GetValue(modelGoo.Value);
            if (value == null) continue;
            if (isList)
            {
                if (isPropertyModel)
                {
                    dynamic? list = Activator.CreateInstance(PropertyGooM[i]);
                    if (list != null)
                    {
                        foreach (var item in (System.Collections.IEnumerable)value)
                        {
                            var deepCloneMethod = item?.GetType().GetMethod("DeepClone");
                            var clonedItem = deepCloneMethod?.Invoke(item, null);
                            var itemGoo = Activator.CreateInstance(PropertyItemGoo[i], clonedItem);
                            list!.Add(itemGoo);
                        }
                        value = list;
                    }
                }
            }
            else if (isPropertyModel)
            {
                if (isPropertyMapped)
                {
                    var convertMethod = typeof(RhinoConverter).GetMethod("Convert", new System.Type[] { value.GetType() });
                    value = convertMethod?.Invoke(null, new[] { value });
                }
                else
                {
                    var deepCloneMethod = value.GetType().GetMethod("DeepClone");
                    var clonedValue = deepCloneMethod?.Invoke(value, null);
                    value = Activator.CreateInstance(PropertyItemGoo[i], clonedValue);
                }
            }
            if (value != null)
            {
                if (isList) DA.SetDataList(i + 2, (System.Collections.IEnumerable)value);
                else DA.SetData(i + 2, value);
            }
        }
    }
    protected virtual V ProcessModel(V model) => model;
}

public abstract class IdGoo<T> : ModelGoo<T> where T : Model<T>, new()
{
    public IdGoo() : base() { }
    public IdGoo(T value) : base(value) { }
}

public abstract class IdParam<T, U> : ModelParam<T, U> where T : IdGoo<U> where U : Model<U>, new()
{
    internal IdParam() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

public abstract class IdComponent<T, U, V> : ModelComponent<T, U, V>
    where T : IdParam<U, V> where U : IdGoo<V> where V : Model<V>, new()
{
    protected IdComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

public abstract class DiffGoo<T> : ModelGoo<T> where T : Model<T>, new()
{
    public DiffGoo() : base() { }
    public DiffGoo(T value) : base(value) { }
}

public abstract class DiffParam<T, U> : ModelParam<T, U> where T : DiffGoo<U> where U : Model<U>, new()
{
    internal DiffParam() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;

}

public abstract class DiffComponent<T, U, V> : ModelComponent<T, U, V>
    where T : DiffParam<U, V> where U : DiffGoo<V> where V : Model<V>, new()
{
    protected DiffComponent() : base() { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}

#endregion Models

#region Serialization

public abstract class SerializeComponent<T, U, V> : ScriptingComponent
    where T : ModelGoo<V>, new()
    where U : ModelParam<T, V>, new()
    where V : Model<V>, new()
{
    protected SerializeComponent(string name) : base($"Serialize {name}", $"{name}→Str", $"Serialize {name.ToLower()} to string.") { }
    protected override void RegisterInputParams(GH_InputParamManager pManager) => pManager.AddParameter(new U(), typeof(V).Name, typeof(V).Name.Substring(0, 2), $"{typeof(V).Name} to serialize.", GH_ParamAccess.item);
    protected override void RegisterOutputParams(GH_OutputParamManager pManager) => pManager.AddTextParameter("String", "Str", "Serialized string.", GH_ParamAccess.item);
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var input = default(T);
        DA.GetData(0, ref input);
        if (input == null || !input.IsValid) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, $"{typeof(V).Name} is invalid."); return; }
        DA.SetData(0, input.Value.Serialize());
    }
}

public abstract class DeserializeComponent<T, U, V> : ScriptingComponent
    where T : ModelGoo<V>, new()
    where U : ModelParam<T, V>, new()
    where V : Model<V>, new()
{
    protected DeserializeComponent(string name) : base($"Deserialize {name}", $"Str→{name}", $"Deserialize string to {name.ToLower()}.") { }
    protected override void RegisterInputParams(GH_InputParamManager pManager) => pManager.AddTextParameter("String", "Str", "String to deserialize.", GH_ParamAccess.item);
    protected override void RegisterOutputParams(GH_OutputParamManager pManager) => pManager.AddParameter(new U(), typeof(V).Name, typeof(V).Name.Substring(0, 2), $"Deserialized {typeof(V).Name.ToLower()}.", GH_ParamAccess.item);
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var input = "";
        DA.GetData(0, ref input);
        if (string.IsNullOrEmpty(input)) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, "String is null or empty."); return; }
        try { DA.SetData(0, (T)Activator.CreateInstance(typeof(T), input.Deserialize<V>())!); }
        catch (Exception ex) { AddRuntimeMessage(GH_RuntimeMessageLevel.Error, ex.Message); }
    }
}

#endregion Serialization

#region Attribute

public class AttributeDiffGoo : DiffGoo<AttributeDiff>
{
    public AttributeDiffGoo() { }
    public AttributeDiffGoo(AttributeDiff value) : base(value) { }
}

public class AttributeDiffParam : DiffParam<AttributeDiffGoo, AttributeDiff>
{
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B95");
}

public class AttributeDiffComponent : DiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B96");
}

public class AttributeGoo : ModelGoo<Attribute>
{
    public AttributeGoo() { }
    public AttributeGoo(Attribute value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Attribute { Key = str };
            return true;
        }
        return false;
    }
}

public class AttributeParam : ModelParam<AttributeGoo, Attribute>
{
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B94");
}

public class AttributeComponent : ModelComponent<AttributeParam, AttributeGoo, Attribute>
{
    public override Guid ComponentGuid => new("51146B05-ACEB-4810-AD75-10AC3E029D39");
}

public class SerializeAttributeComponent : SerializeComponent<AttributeGoo, AttributeParam, Attribute>
{
    public SerializeAttributeComponent() : base("Attribute") { }
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8974-8588BCA75250");
}

public class DeserializeAttributeComponent : DeserializeComponent<AttributeGoo, AttributeParam, Attribute>
{
    public DeserializeAttributeComponent() : base("Attribute") { }
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8975-8588BCA75250");
}

#endregion

#region Representation

public class RepresentationIdGoo : IdGoo<RepresentationId>
{
    public RepresentationIdGoo() { }
    public RepresentationIdGoo(RepresentationId value) : base(value) { }
}

public class RepresentationIdParam : IdParam<RepresentationIdGoo, RepresentationId>
{
    public override Guid ComponentGuid => new("30A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
}

public class RepresentationIdComponent : IdComponent<RepresentationIdParam, RepresentationIdGoo, RepresentationId>
{
    public override Guid ComponentGuid => new("30A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class RepresentationDiffGoo : DiffGoo<RepresentationDiff>
{
    public RepresentationDiffGoo() { }
    public RepresentationDiffGoo(RepresentationDiff value) : base(value) { }
}

public class RepresentationDiffParam : DiffParam<RepresentationDiffGoo, RepresentationDiff>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A9");
}

public class RepresentationDiffComponent : DiffComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AA");
}

public class RepresentationGoo : ModelGoo<Representation>
{
    public RepresentationGoo() { }
    public RepresentationGoo(Representation value) : base(value) { }
    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Url);
            return true;
        }
        return false;
    }
    internal override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Representation { Url = str };
            return true;
        }
        return false;
    }
}

public class RepresentationParam : ModelParam<RepresentationGoo, Representation>
{
    public override Guid ComponentGuid => new("895BBC91-851A-4DFC-9C83-92DFE90029E8");
}

public class RepresentationComponent : ModelComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");

    protected override Representation ProcessModel(Representation model)
    {
        var mime = Semio.Utility.ParseMimeFromUrl(model.Url);
        var firstTag = model.Tags.FirstOrDefault();
        if (firstTag == null || (firstTag != null && mime != "" && !Semio.Utility.IsValidMime(firstTag))) model.Tags.Insert(0, mime);
        model.Url = model.Url.Replace('\\', '/');
        if (firstTag != null && Semio.Utility.IsValidMime(firstTag)) model.Tags[0] = firstTag;
        return model;
    }
}

public class SerializeRepresentationComponent : SerializeComponent<RepresentationGoo, RepresentationParam, Representation>
{
    public SerializeRepresentationComponent() : base("Representation") { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32046");
}

public class DeserializeRepresentationComponent : DeserializeComponent<RepresentationGoo, RepresentationParam, Representation>
{
    public DeserializeRepresentationComponent() : base("Representation") { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32047");
}

#endregion Representation

#region File

public class FileIdGoo : IdGoo<FileId>
{
    public FileIdGoo() { }
    public FileIdGoo(FileId value) : base(value) { }
}

public class FileIdParam : IdParam<FileIdGoo, FileId>
{
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E7");
}

public class FileIdComponent : IdComponent<FileIdParam, FileIdGoo, FileId>
{
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E8");
}

public class FileDiffGoo : DiffGoo<FileDiff>
{
    public FileDiffGoo() { }
    public FileDiffGoo(FileDiff value) : base(value) { }
}

public class FileDiffParam : DiffParam<FileDiffGoo, FileDiff>
{
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F0");
}

public class FileDiffComponent : DiffComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F1");
}

public class FilesDiffGoo : DiffGoo<FilesDiff>
{
    public FilesDiffGoo() { }
    public FilesDiffGoo(FilesDiff value) : base(value) { }
}

public class FilesDiffParam : DiffParam<FilesDiffGoo, FilesDiff>
{
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A1");
}

public class SemioFileGoo : ModelGoo<SemioFile>
{
    public SemioFileGoo() { }
    public SemioFileGoo(SemioFile value) : base(value) { }
}

public class SemioFileParam : ModelParam<SemioFileGoo, SemioFile>
{
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F8");
}

public class SemioFileComponent : ModelComponent<SemioFileParam, SemioFileGoo, SemioFile>
{
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F9");
}

#endregion File

#region DiagramPoint

public class DiagramPointGoo : ModelGoo<DiagramPoint>
{
    public DiagramPointGoo() { }
    public DiagramPointGoo(DiagramPoint value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.X, Value.Y, 0));
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        Point3d point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new DiagramPoint { X = (float)point.X, Y = (float)point.Y };
            return true;
        }
        return false;
    }
}

public class DiagramPointParam : ModelParam<DiagramPointGoo, DiagramPoint>
{
    public override Guid ComponentGuid => new("4685CCE8-C629-4638-8DF6-F76A17571841");
}

public class DiagramPointComponent : ModelComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public override Guid ComponentGuid => new("61FB9BBE-64DE-42B2-B7EF-69CD97FDD9E3");
}

public class SerializeDiagramPointComponent : SerializeComponent<DiagramPointGoo, DiagramPointParam, DiagramPoint>
{
    public SerializeDiagramPointComponent() : base("Diagram Point") { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A99");
}

public class DeserializeDiagramPointComponent : DeserializeComponent<DiagramPointGoo, DiagramPointParam, DiagramPoint>
{
    public DeserializeDiagramPointComponent() : base("Diagram Point") { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A9A");
}

#endregion DiagramPoint

#region Port

public class PortIdGoo : IdGoo<PortId>
{
    public PortIdGoo() { }
    public PortIdGoo(PortId value) : base(value) { }
}

public class PortIdParam : IdParam<PortIdGoo, PortId>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B1");
}

public class PortIdComponent : IdComponent<PortIdParam, PortIdGoo, PortId>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B2");
}

public class PortDiffGoo : DiffGoo<PortDiff>
{
    public PortDiffGoo() { }
    public PortDiffGoo(PortDiff value) : base(value) { }
}

public class PortDiffParam : DiffParam<PortDiffGoo, PortDiff>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B0");
}

public class PortDiffComponent : DiffComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B3");
}

public class PortGoo : ModelGoo<Port>
{
    public PortGoo() { }
    public PortGoo(Port value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Plane)))
        {
            if (Value.Direction is null || Value.Point is null) return false;
            target = (Q)(object)new GH_Plane(Utility.GetPlaneFromYAxis(Value.Direction.Convert(), 0, Value.Point.Convert()));
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Serialize());
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        var plane = new Rhino.Geometry.Plane();
        if (GH_Convert.ToPlane(source, ref plane, GH_Conversion.Both))
        {
            Value.Point = plane.Origin.Convert();
            Value.Direction = plane.YAxis.Convert();
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = str.Deserialize<Port>();
            return true;
        }
        return false;
    }
}

public class PortParam : ModelParam<PortGoo, Port>
{
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1B");
}

public class PortComponent : ModelComponent<PortParam, PortGoo, Port>
{
    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");
}

public class SerializePortComponent : SerializeComponent<PortGoo, PortParam, Port>
{
    public SerializePortComponent() : base("Port") { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B5");
}

public class DeserializePortComponent : DeserializeComponent<PortGoo, PortParam, Port>
{
    public DeserializePortComponent() : base("Port") { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B6");
}

#endregion Port

#region Author

public class AuthorIdGoo : IdGoo<AuthorId>
{
    public AuthorIdGoo() { }
    public AuthorIdGoo(AuthorId value) : base(value) { }
}

public class AuthorIdParam : IdParam<AuthorIdGoo, AuthorId>
{
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1C");
}

public class AuthorIdComponent : IdComponent<AuthorIdParam, AuthorIdGoo, AuthorId>
{
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1D");
}

public class AuthorGoo : ModelGoo<Author>
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
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Author { Email = str };
            return true;
        }
        return false;
    }
}

public class AuthorParam : ModelParam<AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("9F52380B-1812-42F7-9DAD-952C2F7A635A");
}

public class AuthorComponent : ModelComponent<AuthorParam, AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("5143ED92-0A2C-4D0C-84ED-F90CC8450894");
}

public class SerializeAuthorComponent : SerializeComponent<AuthorGoo, AuthorParam, Author>
{
    public SerializeAuthorComponent() : base("Author") { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634878");
}

public class DeserializeAuthorComponent : DeserializeComponent<AuthorGoo, AuthorParam, Author>
{
    public DeserializeAuthorComponent() : base("Author") { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634879");
}

#endregion Author

#region Location

public class LocationGoo : ModelGoo<Location>
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
        if (source == null) return false;
        var point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new Location { Longitude = (float)point.X, Latitude = (float)point.Y };
            return true;
        }
        return false;
    }
}

public class LocationParam : ModelParam<LocationGoo, Location>
{
    public override Guid ComponentGuid => new("CA9DA889-398E-469B-BF1B-AD2BDFCA7957");
}

public class LocationComponent : ModelComponent<LocationParam, LocationGoo, Location>
{
    public override Guid ComponentGuid => new("6F2EDF42-6E10-4944-8B05-4D41F4876ED0");
}

public class SerializeLocationComponent : SerializeComponent<LocationGoo, LocationParam, Location>
{
    public SerializeLocationComponent() : base("Location") { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D466");
}

public class DeserializeLocationComponent : DeserializeComponent<LocationGoo, LocationParam, Location>
{
    public DeserializeLocationComponent() : base("Location") { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D467");
}

#endregion Location

#region Type

public class TypeIdGoo : IdGoo<TypeId>
{
    public TypeIdGoo() { }
    public TypeIdGoo(TypeId value) : base(value) { }
}

public class TypeIdParam : IdParam<TypeIdGoo, TypeId>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C2");
}

public class TypeIdComponent : IdComponent<TypeIdParam, TypeIdGoo, TypeId>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C3");
}

public class TypeDiffGoo : DiffGoo<TypeDiff>
{
    public TypeDiffGoo() { }
    public TypeDiffGoo(TypeDiff value) : base(value) { }
}

public class TypeDiffParam : DiffParam<TypeDiffGoo, TypeDiff>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C1");
}

public class TypeDiffComponent : DiffComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C4");
}

public class TypesDiffGoo : DiffGoo<TypesDiff>
{
    public TypesDiffGoo() { }
    public TypesDiffGoo(TypesDiff value) : base(value) { }
}

public class TypesDiffParam : DiffParam<TypesDiffGoo, TypesDiff>
{
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B6");
}

public class TypeGoo : ModelGoo<Type>
{
    public TypeGoo() { }
    public TypeGoo(Type value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (target is PieceGoo piece)
        {
            piece.Value = new Piece
            {
                Id = Semio.Utility.GenerateRandomId(new Random().Next()),
                Type = new TypeId { Name = Value.Name, Variant = Value.Variant }
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
        if (source == null) return false;
        if (source is PieceGoo piece)
        {
            if (piece.Value.Type is null) return false;
            Value = new Type { Name = piece.Value.Type.Name, Variant = piece.Value.Type.Variant };
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

public class TypeParam : ModelParam<TypeGoo, Type>
{
    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");
}

public class TypeComponent : ModelComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");

    protected override Type ProcessModel(Type type)
    {
        if (type.Unit == "") type.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem);
        return type;
    }
}

public class SerializeTypeComponent : SerializeComponent<TypeGoo, TypeParam, Type>
{
    public SerializeTypeComponent() : base("Type") { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673A");
}

public class DeserializeTypeComponent : DeserializeComponent<TypeGoo, TypeParam, Type>
{
    public DeserializeTypeComponent() : base("Type") { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673B");
}

#endregion Type

#region Piece

public class PieceIdGoo : IdGoo<PieceId>
{
    public PieceIdGoo() { }
    public PieceIdGoo(PieceId value) : base(value) { }
}

public class PieceIdParam : IdParam<PieceIdGoo, PieceId>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D3");
}

public class PieceIdComponent : IdComponent<PieceIdParam, PieceIdGoo, PieceId>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D4");
}

public class PieceDiffGoo : DiffGoo<PieceDiff>
{
    public PieceDiffGoo() { }
    public PieceDiffGoo(PieceDiff value) : base(value) { }
}

public class PieceDiffParam : DiffParam<PieceDiffGoo, PieceDiff>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D2");
}

public class PieceDiffComponent : DiffComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D5");
}

public class PiecesDiffGoo : DiffGoo<PiecesDiff>
{
    public PiecesDiffGoo() { }
    public PiecesDiffGoo(PiecesDiff value) : base(value) { }
}

public class PiecesDiffParam : DiffParam<PiecesDiffGoo, PiecesDiff>
{
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C7");
}

public class PieceGoo : ModelGoo<Piece>
{
    public PieceGoo() { }
    public PieceGoo(Piece value) : base(value) { }

    internal override bool CustomCastTo<Q>(ref Q target)
    {
        if (target is TypeGoo type)
        {
            if (Value.Type is null) return false;
            type.Value = new Type { Name = Value.Type.Name, Variant = Value.Type.Variant };
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    internal override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is TypeGoo type)
        {
            Value = new Piece
            {
                Id = Semio.Utility.GenerateRandomId(new Random().Next()),
                Type = new TypeId { Name = type.Value.Name, Variant = type.Value.Variant }
            };
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Piece { Id = str };
            return true;
        }
        return false;
    }
}

public class PieceParam : ModelParam<PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("76F583DC-4142-4346-B1E1-6C241AF26086");
}

public class PieceComponent : ModelComponent<PieceParam, PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA48");
}

public class SerializePieceComponent : SerializeComponent<PieceGoo, PieceParam, Piece>
{
    public SerializePieceComponent() : base("Piece") { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00D9");
}

public class DeserializePieceComponent : DeserializeComponent<PieceGoo, PieceParam, Piece>
{
    public DeserializePieceComponent() : base("Piece") { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00DA");
}

#endregion Piece

#region Side

public class SideDiffGoo : DiffGoo<SideDiff>
{
    public SideDiffGoo() { }
    public SideDiffGoo(SideDiff value) : base(value) { }
}

public class SideDiffParam : DiffParam<SideDiffGoo, SideDiff>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E3");
}

public class SideDiffComponent : DiffComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E4");
}

public class SideGoo : ModelGoo<Side>
{
    public SideGoo() { }
    public SideGoo(Side value) : base(value) { }
}

public class SideParam : ModelParam<SideGoo, Side>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
}

public class SideComponent : ModelComponent<SideParam, SideGoo, Side>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E6");
}

#endregion Side

#region Connection

public class ConnectionIdGoo : IdGoo<ConnectionId>
{
    public ConnectionIdGoo() { }
    public ConnectionIdGoo(ConnectionId value) : base(value) { }
}

public class ConnectionIdParam : IdParam<ConnectionIdGoo, ConnectionId>
{
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D6");
}

public class ConnectionIdComponent : IdComponent<ConnectionIdParam, ConnectionIdGoo, ConnectionId>
{
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D7");
}

public class ConnectionDiffGoo : DiffGoo<ConnectionDiff>
{
    public ConnectionDiffGoo() { }
    public ConnectionDiffGoo(ConnectionDiff value) : base(value) { }
}

public class ConnectionDiffParam : DiffParam<ConnectionDiffGoo, ConnectionDiff>
{
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F4");
}

public class ConnectionDiffComponent : DiffComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F5");
}

public class ConnectionsDiffGoo : DiffGoo<ConnectionsDiff>
{
    public ConnectionsDiffGoo() { }
    public ConnectionsDiffGoo(ConnectionsDiff value) : base(value) { }
}

public class ConnectionsDiffParam : DiffParam<ConnectionsDiffGoo, ConnectionsDiff>
{
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D8");
}

public class ConnectionGoo : ModelGoo<Connection>
{
    public ConnectionGoo() { }
    public ConnectionGoo(Connection value) : base(value) { }
}

public class ConnectionParam : ModelParam<ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");
}

public class ConnectionComponent : ModelComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");

    protected override void AddModelProps(dynamic pManager)
    {
        pManager.AddTextParameter("Connected Piece Id", "CdPc", "Id of the connected piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Connected Design Piece Id", "CdDP?", "Optional id of the piece inside the referenced design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Connected Piece Type Port Id", "CdPo?", "Optional id of the port of type of the piece. Otherwise the default port will be selected.", GH_ParamAccess.item);
        pManager.AddTextParameter("Connecting Piece Id", "CgPc", "Id of the connected piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Connecting Design Piece Id", "CgDP?", "Optional id of the piece inside the referenced design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Connecting Piece Type Port Id", "CgPo?", "Optional id of the port of type of the piece. Otherwise the default port will be selected.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "The optional human-readable description of the connection.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "Gp?", "The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "Sf?", "The optional lateral shift (applied after rotation and tilt in port direction) between the connected and the connecting piece.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rise", "Rs?", "The optional vertical rise in port direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt?", "The optional horizontal rotation in port direction between the connected and the connecting piece in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Turn", "Tu?", "The optional turn perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.  Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl?", "The optional horizontal tilt perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("X", "X?", "The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Y", "Y?", "The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.", GH_ParamAccess.item);
        pManager.AddParameter(new AttributeParam(), "Attributes", "At*", "The optional attributes of the connection.", GH_ParamAccess.list);
    }

    protected override void GetProps(IGH_DataAccess DA, dynamic connectionGoo)
    {
        var connectedPieceId = "";
        var connectedDesignPieceId = "";
        var connectedPortId = "";
        var connectingPieceId = "";
        var connectingDesignPieceId = "";
        var connectingPortId = "";
        var description = "";
        var gap = 0.0;
        var shift = 0.0;
        var raise = 0.0;
        var rotation = 0.0;
        var turn = 0.0;
        var tilt = 0.0;
        var x = 0.0;
        var y = 0.0;
        var attributesGoos = new List<AttributeGoo>();
        if (DA.GetData(2, ref connectedPieceId)) connectionGoo.Value.Connected.Piece.Id = connectedPieceId;
        if (DA.GetData(3, ref connectedDesignPieceId))
        {
            connectionGoo.Value.Connected.DesignPiece ??= new PieceId();
            connectionGoo.Value.Connected.DesignPiece.Id = connectedDesignPieceId;
        }
        if (DA.GetData(4, ref connectedPortId)) connectionGoo.Value.Connected.Port.Id = connectedPortId;
        if (DA.GetData(5, ref connectingPieceId))
            connectionGoo.Value.Connecting.Piece.Id = connectingPieceId;
        if (DA.GetData(6, ref connectingDesignPieceId))
        {
            connectionGoo.Value.Connecting.DesignPiece ??= new PieceId();
            connectionGoo.Value.Connecting.DesignPiece.Id = connectingDesignPieceId;
        }
        if (DA.GetData(7, ref connectingPortId)) connectionGoo.Value.Connecting.Port.Id = connectingPortId;
        if (DA.GetData(8, ref description)) connectionGoo.Value.Description = description;
        if (DA.GetData(9, ref gap)) connectionGoo.Value.Gap = (float)gap;
        if (DA.GetData(10, ref shift)) connectionGoo.Value.Shift = (float)shift;
        if (DA.GetData(11, ref raise)) connectionGoo.Value.Rise = (float)raise;
        if (DA.GetData(12, ref rotation)) connectionGoo.Value.Rotation = (float)rotation;
        if (DA.GetData(13, ref turn)) connectionGoo.Value.Turn = (float)turn;
        if (DA.GetData(14, ref tilt)) connectionGoo.Value.Tilt = (float)tilt;
        if (DA.GetData(15, ref x)) connectionGoo.Value.X = (float)x;
        if (DA.GetData(16, ref y)) connectionGoo.Value.Y = (float)y;
        if (DA.GetDataList(17, attributesGoos)) connectionGoo.Value.Attributes = attributesGoos.Select(q => q.Value).ToList();
    }

    protected override void SetData(IGH_DataAccess DA, dynamic connectionGoo)
    {
        DA.SetData(2, connectionGoo.Value.Connected.Piece.Id);
        DA.SetData(3, connectionGoo.Value.Connected.DesignPiece != null ? connectionGoo.Value.Connected.DesignPiece.Id : "");
        DA.SetData(4, connectionGoo.Value.Connected.Port.Id);
        DA.SetData(5, connectionGoo.Value.Connecting.Piece.Id);
        DA.SetData(6, connectionGoo.Value.Connecting.DesignPiece != null ? connectionGoo.Value.Connecting.DesignPiece.Id : "");
        DA.SetData(7, connectionGoo.Value.Connecting.Port.Id);
        DA.SetData(8, connectionGoo.Value.Description);
        DA.SetData(9, connectionGoo.Value.Gap);
        DA.SetData(10, connectionGoo.Value.Shift);
        DA.SetData(11, connectionGoo.Value.Rise);
        DA.SetData(12, connectionGoo.Value.Rotation);
        DA.SetData(13, connectionGoo.Value.Turn);
        DA.SetData(14, connectionGoo.Value.Tilt);
        DA.SetData(15, connectionGoo.Value.X);
        DA.SetData(16, connectionGoo.Value.Y);
        var attributeGoos = new List<AttributeGoo>();
        foreach (Attribute attribute in connectionGoo.Value.Attributes) attributeGoos.Add(new AttributeGoo(attribute.DeepClone()));
        DA.SetDataList(17, attributeGoos);
    }
}

public class SerializeConnectionComponent : SerializeComponent<ConnectionGoo, ConnectionParam, Connection>
{
    public SerializeConnectionComponent() : base("Connection") { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD60");
}

public class DeserializeConnectionComponent : DeserializeComponent<ConnectionGoo, ConnectionParam, Connection>
{
    public DeserializeConnectionComponent() : base("Connection") { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD61");
}

#endregion Connection

#region Design

public class DesignIdGoo : IdGoo<DesignId>
{
    public DesignIdGoo() { }
    public DesignIdGoo(DesignId value) : base(value) { }
}

public class DesignIdParam : IdParam<DesignIdGoo, DesignId>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A6");
}

public class DesignIdComponent : IdComponent<DesignIdParam, DesignIdGoo, DesignId>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A7");
}

public class DesignDiffGoo : DiffGoo<DesignDiff>
{
    public DesignDiffGoo() { }
    public DesignDiffGoo(DesignDiff value) : base(value) { }
}

public class DesignDiffParam : DiffParam<DesignDiffGoo, DesignDiff>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A5");
}

public class DesignDiffComponent : DiffComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A8");
}

public class DesignsDiffGoo : DiffGoo<DesignsDiff>
{
    public DesignsDiffGoo() { }
    public DesignsDiffGoo(DesignsDiff value) : base(value) { }
}

public class DesignsDiffParam : DiffParam<DesignsDiffGoo, DesignsDiff>
{
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8E9");
}

public class DesignGoo : ModelGoo<Design>
{
    public DesignGoo() { }
    public DesignGoo(Design value) : base(value) { }
}

public class DesignParam : ModelParam<DesignGoo, Design>
{
    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");
}

public class DesignComponent : ModelComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("AAD8D144-2EEE-48F1-A8A9-52977E86CB54");
    protected override Design ProcessModel(Design design)
    {
        if (design.Unit == "") design.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem);
        return design;
    }
}

public class SerializeDesignComponent : SerializeComponent<DesignGoo, DesignParam, Design>
{
    public SerializeDesignComponent() : base("Design") { }
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");
}

public class DeserializeDesignComponent : DeserializeComponent<DesignGoo, DesignParam, Design>
{
    public DeserializeDesignComponent() : base("Design") { }
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB59");
}

#endregion Design

#region Kit

public class KitDiffGoo : DiffGoo<KitDiff>
{
    public KitDiffGoo() { }
    public KitDiffGoo(KitDiff value) : base(value) { }
}

public class KitDiffParam : DiffParam<KitDiffGoo, KitDiff>
{
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B2");
}

public class KitDiffComponent : DiffComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B3");
}

public class KitGoo : ModelGoo<Kit>
{
    public KitGoo() { }
    public KitGoo(Kit value) : base(value) { }
}

public class KitParam : ModelParam<KitGoo, Kit>
{
    public override Guid ComponentGuid => new("BA9F161E-AFE3-41D5-8644-964DD20B887B");
}

public class KitComponent : ModelComponent<KitParam, KitGoo, Kit>
{
    public override Guid ComponentGuid => new("987560A8-10D4-43F6-BEBE-D71DC2FD86AF");
    protected override Kit ProcessModel(Kit kit)
    {
        kit.Name = kit.Name.Humanize(LetterCasing.Title);
        return kit;
    }
}

public class SerializeKitComponent : SerializeComponent<KitGoo, KitParam, Kit>
{
    public SerializeKitComponent() : base("Kit") { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4165");
}

public class DeserializeKitComponent : DeserializeComponent<KitGoo, KitParam, Kit>
{
    public DeserializeKitComponent() : base("Kit") { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4166");
}

#endregion Kit

#region Quality

public class QualityIdGoo : IdGoo<QualityId>
{
    public QualityIdGoo() { }
    public QualityIdGoo(QualityId value) : base(value) { }
}

public class QualityIdParam : IdParam<QualityIdGoo, QualityId>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class QualityIdComponent : IdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
}

public class QualityGoo : ModelGoo<Quality>
{
    public QualityGoo() { }
    public QualityGoo(Quality value) : base(value) { }
}

public class QualityParam : ModelParam<QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class QualityComponent : ModelComponent<QualityParam, QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

public class SerializeQualityComponent : SerializeComponent<QualityGoo, QualityParam, Quality>
{
    public SerializeQualityComponent() : base("Quality") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C8");
}

public class DeserializeQualityComponent : DeserializeComponent<QualityGoo, QualityParam, Quality>
{
    public DeserializeQualityComponent() : base("Quality") { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C9");
}

#endregion Quality

#region Benchmark

public class BenchmarkGoo : ModelGoo<Benchmark>
{
    public BenchmarkGoo() { }
    public BenchmarkGoo(Benchmark value) : base(value) { }
}

public class BenchmarkParam : ModelParam<BenchmarkGoo, Benchmark>
{
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class BenchmarkComponent : ModelComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
}

public class SerializeBenchmarkComponent : SerializeComponent<BenchmarkGoo, BenchmarkParam, Benchmark>
{
    public SerializeBenchmarkComponent() : base("Benchmark") { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeBenchmarkComponent : DeserializeComponent<BenchmarkGoo, BenchmarkParam, Benchmark>
{
    public DeserializeBenchmarkComponent() : base("Benchmark") { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion Benchmark

#region Prop

public class PropGoo : ModelGoo<Prop>
{
    public PropGoo() { }
    public PropGoo(Prop value) : base(value) { }
}

public class PropParam : ModelParam<PropGoo, Prop>
{
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class PropComponent : ModelComponent<PropParam, PropGoo, Prop>
{
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
}

public class SerializePropComponent : SerializeComponent<PropGoo, PropParam, Prop>
{
    public SerializePropComponent() : base("Prop") { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializePropComponent : DeserializeComponent<PropGoo, PropParam, Prop>
{
    public DeserializePropComponent() : base("Prop") { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion Prop

#region Stat

public class StatGoo : ModelGoo<Stat>
{
    public StatGoo() { }
    public StatGoo(Stat value) : base(value) { }
}

public class StatParam : ModelParam<StatGoo, Stat>
{
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
}

public class StatComponent : ModelComponent<StatParam, StatGoo, Stat>
{
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
}

public class SerializeStatComponent : SerializeComponent<StatGoo, StatParam, Stat>
{
    public SerializeStatComponent() : base("Stat") { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
}

public class DeserializeStatComponent : DeserializeComponent<StatGoo, StatParam, Stat>
{
    public DeserializeStatComponent() : base("Stat") { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
}

#endregion Stat

#region Scripting

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


#endregion Scripting

#region Engine

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
                "If you want, you can report this under: https://github.com/usalu/semio/issues\n" +
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
            // lightweight way to kill child process when parent process is killed
            // https://stackoverflow.com/questions/3342941/kill-child-process-when-parent-process-is-killed#4657392
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

#region Persistence

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
    protected override dynamic? Run(dynamic? input = null) => input != null ? RunOnKit(input.Uri, input.Input) : null;
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

#endregion

#endregion Engine

#region Meta

public static class Meta
{
    /// <summary>
    ///     Name of the model : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, System.Type> Goo;

    /// <summary>
    ///     Name of the model : Index of the property : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<System.Type>> PropertyGoo;

    /// <summary>
    ///     Name of the model : Index of the property : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<System.Type>> PropertyItemGoo;

    /// <summary>
    ///     Name of the model : Param
    /// </summary>
    public static readonly ImmutableDictionary<string, System.Type> Param;

    /// <summary>
    ///     Name of the model : Index of the property : Param
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<System.Type>> PropertyParam;

    /// <summary>
    ///     Name of the model : Index of the property : IsMapped
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<bool>> IsPropertyMapped;

    static Meta()
    {
        var dummyMeta = Semio.Meta.Model;
        var goo = new Dictionary<string, System.Type>();
        var propertyGoo = new Dictionary<string, List<System.Type>>();
        var propertyItemGoo = new Dictionary<string, List<System.Type>>();
        var param = new Dictionary<string, System.Type>();
        var propertyParam = new Dictionary<string, List<System.Type>>();
        var manualMappedTypes = new Dictionary<System.Type, (System.Type, System.Type)>
        {
            { typeof(string), (typeof(GH_String), typeof(Param_String)) },
            { typeof(bool), (typeof(GH_Boolean), typeof(Param_Boolean)) },
            { typeof(int), (typeof(GH_Integer), typeof(Param_Integer)) },
            { typeof(float), (typeof(GH_Number), typeof(Param_Number)) },
            { typeof(Point), (typeof(GH_Point), typeof(Param_Point)) },
            { typeof(Vector), (typeof(GH_Vector), typeof(Param_Vector)) },
            { typeof(Plane), (typeof(GH_Plane), typeof(Param_Plane)) }
        };
        var isPropertyMapped = new Dictionary<string, List<bool>>();
        foreach (var manualMappedTypeKvp in manualMappedTypes)
        {
            var name = manualMappedTypeKvp.Key.Name;
            goo[name] = manualMappedTypeKvp.Value.Item1;
            goo[name + "List"] = typeof(List<>).MakeGenericType(goo[name]);
            param[name] = manualMappedTypeKvp.Value.Item2;
        }
        foreach (var kvp in Semio.Meta.Type)
        {
            var baseName = typeof(Meta).Namespace + "." + kvp.Key;
            if (!goo.ContainsKey(kvp.Key))
            {
                var equivalentGooType = Assembly.GetExecutingAssembly().GetType(baseName + "Goo");
                if (equivalentGooType != null)
                {
                    goo[kvp.Key] = equivalentGooType;
                    goo[kvp.Key + "List"] = typeof(List<>).MakeGenericType(goo[kvp.Key]);
                }
                var equivalentParamType = Assembly.GetExecutingAssembly().GetType(baseName + "Param");
                if (equivalentParamType != null)
                    param[kvp.Key] = equivalentParamType;
            }
            propertyGoo[kvp.Key] = new List<System.Type>();
            propertyItemGoo[kvp.Key] = new List<System.Type>();
            propertyParam[kvp.Key] = new List<System.Type>();
            isPropertyMapped[kvp.Key] = new List<bool>();
        }
        foreach (var modelKvp in Semio.Meta.Property)
            for (var i = 0; i < modelKvp.Value.Length; i++)
            {
                var property = modelKvp.Value[i];
                var isPropertyList = Semio.Meta.IsPropertyList[modelKvp.Key][i];
                var propertyTypeName = isPropertyList
                    ? property.PropertyType.GetGenericArguments()[0].Name
                    : property.PropertyType.Name;
                bool isPropertyMappedValue;
                try
                {
                    isPropertyMappedValue = manualMappedTypes.ContainsKey(Semio.Meta.Type[propertyTypeName]);
                }
                catch
                {
                    isPropertyMappedValue = false;
                }
                isPropertyMapped[modelKvp.Key].Add(isPropertyMappedValue);
                try
                {
                    propertyGoo[modelKvp.Key].Add(goo[isPropertyList ? propertyTypeName + "List" : propertyTypeName]);
                    propertyItemGoo[modelKvp.Key].Add(goo[propertyTypeName]);
                }
                catch (Exception)
                {
                    // ignored
                }
                try
                {
                    propertyParam[modelKvp.Key].Add(param[propertyTypeName]);
                }
                catch (Exception)
                {
                    // ignored
                }
            }
        Goo = goo.ToImmutableDictionary();
        PropertyGoo = propertyGoo.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        PropertyItemGoo = propertyItemGoo.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        Param = param.ToImmutableDictionary();
        PropertyParam = propertyParam.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        IsPropertyMapped = isPropertyMapped.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
    }
}

#endregion Meta
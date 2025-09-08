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

using System;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Drawing;
using System.IO;
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

#region Bases

public abstract class ModelGoo<TModel> : GH_Goo<TModel> where TModel : Model<TModel>, new()
{
    public ModelGoo() { Value = new TModel(); }
    public ModelGoo(TModel value) { Value = value; }
    public override bool IsValid => Value is not null;
    public override string TypeName => GetModelTypeName();
    public override string TypeDescription => GetModelDescription();

    protected abstract string GetModelTypeName();
    protected abstract string GetModelDescription();
    protected abstract string GetSerializationKey();

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
        if (source == null) return false;
        if (source is TModel model)
        {
            Value = model;
            return true;
        }
        return CustomCastFrom(source);
    }
}

public abstract class ModelParam<TGoo, TModel> : GH_PersistentParam<TGoo> where TGoo : ModelGoo<TModel> where TModel : Model<TModel>, new()
{
    protected ModelParam(string name, string nickname, string description) : base(name, nickname, description, Constants.Category, "Params") { }
    protected override Bitmap Icon => GetParamIcon();
    protected abstract Bitmap GetParamIcon();
    protected override GH_GetterResult Prompt_Singular(ref TGoo value) => GH_GetterResult.cancel;
    protected override GH_GetterResult Prompt_Plural(ref List<TGoo> values) => GH_GetterResult.cancel;
}

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

public abstract class ModelComponent<TParam, TGoo, TModel> : Component
    where TParam : ModelParam<TGoo, TModel>, new() where TGoo : ModelGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected ModelComponent(string name, string nickname, string description) : base(name, nickname, description, "Modeling") { }

    protected override Bitmap Icon => GetComponentIcon();
    public override GH_Exposure Exposure => GH_Exposure.primary;
    public override Guid ComponentGuid => GetComponentGuid();

    protected abstract Bitmap GetComponentIcon();
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

public abstract class IdGoo<TModel> : ModelGoo<TModel> where TModel : Model<TModel>, new()
{
    public IdGoo() : base() { }
    public IdGoo(TModel value) : base(value) { }
}

public abstract class IdParam<TGoo, TModel> : ModelParam<TGoo, TModel> where TGoo : IdGoo<TModel> where TModel : Model<TModel>, new()
{
    protected IdParam(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

public abstract class IdComponent<TParam, TGoo, TModel> : ModelComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected IdComponent(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.secondary;
}

public abstract class DiffGoo<TModel> : ModelGoo<TModel> where TModel : Model<TModel>, new()
{
    public DiffGoo() : base() { }
    public DiffGoo(TModel value) : base(value) { }
}

public abstract class DiffParam<TGoo, TModel> : ModelParam<TGoo, TModel> where TGoo : DiffGoo<TModel> where TModel : Model<TModel>, new()
{
    protected DiffParam(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}

public abstract class DiffComponent<TParam, TGoo, TModel> : ModelComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected DiffComponent(string name, string nickname, string description) : base(name, nickname, description) { }
    public override GH_Exposure Exposure => GH_Exposure.tertiary;
}
public abstract class SerializeComponent<TParam, TGoo, TModel> : ScriptingComponent
    where TParam : ModelParam<TGoo, TModel>, new() where TGoo : ModelGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected SerializeComponent(string name, string nickname, string description) : base(name, nickname, description) { }

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
    where TParam : ModelParam<TGoo, TModel>, new() where TGoo : ModelGoo<TModel>, new() where TModel : Model<TModel>, new()
{
    protected DeserializeComponent(string name, string nickname, string description) : base(name, nickname, description) { }

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
            var goo = new TGoo();
            goo.Value = model;
            DA.SetData(0, goo);
        }
    }
}

public abstract class SerializeDiffComponent<TParam, TGoo, TModel> : SerializeComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Model<TModel>, new()
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

public abstract class DeserializeDiffComponent<TParam, TGoo, TModel> : DeserializeComponent<TParam, TGoo, TModel>
    where TParam : DiffParam<TGoo, TModel>, new() where TGoo : DiffGoo<TModel>, new() where TModel : Model<TModel>, new()
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

public abstract class SerializeIdComponent<TParam, TGoo, TModel> : SerializeComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Model<TModel>, new()
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

public abstract class DeserializeIdComponent<TParam, TGoo, TModel> : DeserializeComponent<TParam, TGoo, TModel>
    where TParam : IdParam<TGoo, TModel>, new() where TGoo : IdGoo<TModel>, new() where TModel : Model<TModel>, new()
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

public abstract class EntityGoo<TEntity, TEntityDiff, TEntityId> : ModelGoo<TEntity>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    public EntityGoo() : base() { }
    public EntityGoo(TEntity value) : base(value) { }
}

public abstract class EntityParam<TGoo, TEntity, TEntityDiff, TEntityId> : ModelParam<TGoo, TEntity>
    where TGoo : EntityGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    internal EntityParam() : base() { }
}

public abstract class EntityComponent<TParam, TGoo, TEntity, TEntityDiff, TEntityId> : ModelComponent<TParam, TGoo, TEntity>
    where TParam : EntityParam<TGoo, TEntity, TEntityDiff, TEntityId>
    where TGoo : EntityGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    protected EntityComponent() : base() { }
}

public abstract class EntityIdGoo<TEntity, TEntityDiff, TEntityId> : IdGoo<TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    public EntityIdGoo() : base() { }
    public EntityIdGoo(TEntityId value) : base(value) { }
}

public abstract class EntityIdParam<TIdGoo, TEntity, TEntityDiff, TEntityId> : IdParam<TIdGoo, TEntityId>
    where TIdGoo : EntityIdGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    internal EntityIdParam() : base() { }
}

public abstract class EntityIdComponent<TIdParam, TIdGoo, TEntity, TEntityDiff, TEntityId> : IdComponent<TIdParam, TIdGoo, TEntityId>
    where TIdParam : EntityIdParam<TIdGoo, TEntity, TEntityDiff, TEntityId>
    where TIdGoo : EntityIdGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    protected EntityIdComponent() : base() { }
}

public abstract class EntityDiffGoo<TEntity, TEntityDiff, TEntityId> : DiffGoo<TEntityDiff>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    public EntityDiffGoo() : base() { }
    public EntityDiffGoo(TEntityDiff value) : base(value) { }
}

public abstract class EntityDiffParam<TDiffGoo, TEntity, TEntityDiff, TEntityId> : DiffParam<TDiffGoo, TEntityDiff>
    where TDiffGoo : EntityDiffGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    internal EntityDiffParam() : base() { }
}

public abstract class EntityDiffComponent<TDiffParam, TDiffGoo, TEntity, TEntityDiff, TEntityId> : DiffComponent<TDiffParam, TDiffGoo, TEntityDiff>
    where TDiffParam : EntityDiffParam<TDiffGoo, TEntity, TEntityDiff, TEntityId>
    where TDiffGoo : EntityDiffGoo<TEntity, TEntityDiff, TEntityId>
    where TEntity : Model<TEntity>, new()
    where TEntityDiff : Model<TEntityDiff>, new()
    where TEntityId : Model<TEntityId>, new()
{
    protected EntityDiffComponent() : base() { }
}

#endregion Bases

#region Attribute

public class AttributeIdGoo : IdGoo<AttributeId>
{
    public AttributeIdGoo() { }
    public AttributeIdGoo(AttributeId value) : base(value) { }

    protected override string GetModelTypeName() => "AttributeId";
    protected override string GetModelDescription() => "The ID of the attribute.";
    protected override string GetSerializationKey() => "AttributeId";
    protected override ModelGoo<AttributeId> CreateDuplicate() => new AttributeIdGoo();

    protected override bool CustomCastTo<Q>(ref Q target)
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
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new AttributeId { Key = str };
            return true;
        }
        return false;
    }
}

public class AttributeIdParam : IdParam<AttributeIdGoo, AttributeId>
{
    public AttributeIdParam() : base("AttributeId", "AI", "AttributeId parameter") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B93");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AttributeIdComponent : IdComponent<AttributeIdParam, AttributeIdGoo, AttributeId>
{
    public AttributeIdComponent() : base("AttributeId", "AI", "AttributeId component") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B92");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeIdParam(), "AttributeId", "AI", "AttributeId", GH_ParamAccess.item);
    }
    protected override void SolveModelInstance(IGH_DataAccess DA) { }
}

public class AttributeDiffGoo : DiffGoo<AttributeDiff>
{
    public AttributeDiffGoo() { }
    public AttributeDiffGoo(AttributeDiff value) : base(value) { }

    protected override string GetModelTypeName() => "AttributeDiff";
    protected override string GetModelDescription() => "Difference between two Attributes";
    protected override string GetSerializationKey() => "attribute_diff";
    protected override ModelGoo<AttributeDiff> CreateDuplicate() => new AttributeDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<AttributeDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class AttributeDiffParam : DiffParam<AttributeDiffGoo, AttributeDiff>
{
    public AttributeDiffParam() : base("AttributeDiff", "AD", "AttributeDiff parameter") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B95");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AttributeDiffComponent : DiffComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public AttributeDiffComponent() : base("AttributeDiff", "AD", "AttributeDiff component") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B96");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeDiffParam(), "AttributeDiff", "AD", "AttributeDiff", GH_ParamAccess.item);
    }
    protected override void SolveModelInstance(IGH_DataAccess DA) { }
}

public class SerializeAttributeDiffComponent : SerializeComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public SerializeAttributeDiffComponent() : base("Serialize AttributeDiff", "SAD", "Serialize AttributeDiff to JSON") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B97");
    protected override string GetModelTypeName() => "AttributeDiff";
    protected override string GetModelDescription() => "Serialize AttributeDiff to JSON";
    protected override string GetModelNickname() => "SAD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeAttributeDiffComponent : DeserializeComponent<AttributeDiffParam, AttributeDiffGoo, AttributeDiff>
{
    public DeserializeAttributeDiffComponent() : base("Deserialize AttributeDiff", "DAD", "Deserialize JSON to AttributeDiff") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B98");
    protected override string GetModelTypeName() => "AttributeDiff";
    protected override string GetModelDescription() => "Deserialize JSON to AttributeDiff";
    protected override string GetModelNickname() => "DAD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class AttributeGoo : ModelGoo<Attribute>
{
    public AttributeGoo() { }
    public AttributeGoo(Attribute value) : base(value) { }

    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelDescription() => "A Semio Attribute";
    protected override string GetSerializationKey() => "attribute";
    protected override ModelGoo<Attribute> CreateDuplicate() => new AttributeGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
public class AttributeParam : ModelParam<AttributeGoo, Attribute>
{
    public AttributeParam() : base("Attribute", "A", "Attribute parameter") { }
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B94");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AttributeComponent : ModelComponent<AttributeParam, AttributeGoo, Attribute>
{
    public AttributeComponent() : base("Attribute", "A", "Attribute component") { }
    public override Guid ComponentGuid => new("51146B05-ACEB-4810-AD75-10AC3E029D39");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttributeParam(), "Attribute", "A", "Attribute", GH_ParamAccess.item);
    }
    protected override void SolveModelInstance(IGH_DataAccess DA) { }
}

public class SerializeAttributeComponent : SerializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public SerializeAttributeComponent() : base("Serialize Attribute", ">>A", "Serialize Attribute to JSON") { }
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8974-8588BCA75250");
    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelDescription() => "Serialize Attribute to JSON";
    protected override string GetModelNickname() => "SA";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeAttributeComponent : DeserializeComponent<AttributeParam, AttributeGoo, Attribute>
{
    public DeserializeAttributeComponent() : base("Deserialize Attribute", "<<A", "Deserialize JSON to Attribute") { }
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8975-8588BCA75250");
    protected override string GetModelTypeName() => "Attribute";
    protected override string GetModelDescription() => "Deserialize JSON to Attribute";
    protected override string GetModelNickname() => "DA";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion

#region Representation

public class RepresentationIdGoo : IdGoo<RepresentationId>
{
    public RepresentationIdGoo() { }
    public RepresentationIdGoo(RepresentationId value) : base(value) { }

    protected override string GetModelTypeName() => "RepresentationId";
    protected override string GetModelDescription() => "Identifier for a Representation";
    protected override string GetSerializationKey() => "representation_id";
    protected override ModelGoo<RepresentationId> CreateDuplicate() => new RepresentationIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(RepresentationDiffGoo)))
        {
            target = (Q)(object)new RepresentationDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(RepresentationGoo)))
        {
            target = (Q)(object)new RepresentationGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is RepresentationDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (source is RepresentationGoo reprGoo)
        {
            Value = reprGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new RepresentationId { Tags = new List<string> { str } };
            return true;
        }
        return false;
    }
}

public class RepresentationIdParam : IdParam<RepresentationIdGoo, RepresentationId>
{
    public RepresentationIdParam() : base("RepresentationId", "RI", "RepresentationId parameter") { }
    public override Guid ComponentGuid => new("30A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class RepresentationIdComponent : IdComponent<RepresentationIdParam, RepresentationIdGoo, RepresentationId>
{
    public RepresentationIdComponent() : base("RepresentationId", "RI", "RepresentationId component") { }
    public override Guid ComponentGuid => new("30A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationIdParam(), "RepresentationId", "RI", "RepresentationId", GH_ParamAccess.item);
    }
    protected override void SolveModelInstance(IGH_DataAccess DA) { }
}

public class RepresentationDiffGoo : DiffGoo<RepresentationDiff>
{
    public RepresentationDiffGoo() { }
    public RepresentationDiffGoo(RepresentationDiff value) : base(value) { }

    protected override string GetModelTypeName() => "RepresentationDiff";
    protected override string GetModelDescription() => "Difference between two Representations";
    protected override string GetSerializationKey() => "representation_diff";
    protected override ModelGoo<RepresentationDiff> CreateDuplicate() => new RepresentationDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(string.Join(",", Value.Tags));
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<RepresentationDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class RepresentationDiffParam : DiffParam<RepresentationDiffGoo, RepresentationDiff>
{
    public RepresentationDiffParam() : base("RepresentationDiff", "RD", "RepresentationDiff parameter") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8A9");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class RepresentationDiffComponent : DiffComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public RepresentationDiffComponent() : base("RepresentationDiff", "RD", "RepresentationDiff component") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AA");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationDiffParam(), "RepresentationDiff", "RD", "RepresentationDiff", GH_ParamAccess.item);
    }
    protected override void SolveModelInstance(IGH_DataAccess DA) { }
}

public class SerializeRepresentationDiffComponent : SerializeComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public SerializeRepresentationDiffComponent() : base("Serialize RepresentationDiff", "SRD", "Serialize RepresentationDiff to JSON") { }
    public override Guid ComponentGuid => new("71E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AB");
    protected override string GetModelTypeName() => "RepresentationDiff";
    protected override string GetModelDescription() => "Serialize RepresentationDiff to JSON";
    protected override string GetModelNickname() => "SRD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeRepresentationDiffComponent : DeserializeComponent<RepresentationDiffParam, RepresentationDiffGoo, RepresentationDiff>
{
    public DeserializeRepresentationDiffComponent() : base("Deserialize RepresentationDiff", "DRD", "Deserialize JSON to RepresentationDiff") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AC");
    protected override string GetModelTypeName() => "RepresentationDiff";
    protected override string GetModelDescription() => "Deserialize JSON to RepresentationDiff";
    protected override string GetModelNickname() => "DRD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class RepresentationsDiffGoo : DiffGoo<RepresentationsDiff>
{
    public RepresentationsDiffGoo() { }
    public RepresentationsDiffGoo(RepresentationsDiff value) : base(value) { }

    protected override string GetModelTypeName() => "RepresentationsDiff";
    protected override string GetModelDescription() => "Difference between two Representations collections";
    protected override string GetSerializationKey() => "representations_diff";
    protected override ModelGoo<RepresentationsDiff> CreateDuplicate() => new RepresentationsDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("RepresentationsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<RepresentationsDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class RepresentationsDiffParam : DiffParam<RepresentationsDiffGoo, RepresentationsDiff>
{
    public RepresentationsDiffParam() : base("RepresentationsDiff", "RsD", "RepresentationsDiff parameter") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AB");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class RepresentationsDiffComponent : DiffComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public RepresentationsDiffComponent() : base("RepresentationsDiff", "RsD", "RepresentationsDiff component") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AD");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationsDiffParam(), "RepresentationsDiff", "RsD", "RepresentationsDiff", GH_ParamAccess.item);
    }
    protected override void SolveModelInstance(IGH_DataAccess DA) { }
}

public class SerializeRepresentationsDiffComponent : SerializeComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public SerializeRepresentationsDiffComponent() : base("Serialize RepresentationsDiff", "SRsD", "Serialize RepresentationsDiff to JSON") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AE");
    protected override string GetModelTypeName() => "RepresentationsDiff";
    protected override string GetModelDescription() => "Serialize RepresentationsDiff to JSON";
    protected override string GetModelNickname() => "SRsD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeRepresentationsDiffComponent : DeserializeComponent<RepresentationsDiffParam, RepresentationsDiffGoo, RepresentationsDiff>
{
    public DeserializeRepresentationsDiffComponent() : base("Deserialize RepresentationsDiff", "DRsD", "Deserialize JSON to RepresentationsDiff") { }
    public override Guid ComponentGuid => new("70E5F6A7-B8C9-D0E1-F2A3-B4C5D6E7F8AF");
    protected override string GetModelTypeName() => "RepresentationsDiff";
    protected override string GetModelDescription() => "Deserialize JSON to RepresentationsDiff";
    protected override string GetModelNickname() => "DRsD";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class RepresentationGoo : ModelGoo<Representation>
{
    public RepresentationGoo() { }
    public RepresentationGoo(Representation value) : base(value) { }

    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelDescription() => "A Semio Representation";
    protected override string GetSerializationKey() => "representation";
    protected override ModelGoo<Representation> CreateDuplicate() => new RepresentationGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.ToIdString());
            return true;
        }
        return false;
    }
    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Representation { Tags = new List<string> { str } };
            return true;
        }
        return false;
    }
}

public class RepresentationParam : ModelParam<RepresentationGoo, Representation>
{
    public RepresentationParam() : base("Representation", "R", "Representation parameter") { }
    public override Guid ComponentGuid => new("895BBC91-851A-4DFC-9C83-92DFE90029E8");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class RepresentationComponent : ModelComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public RepresentationComponent() : base("Representation", "R", "Representation component") { }
    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "R", "Representation", GH_ParamAccess.item);
    }
    protected override void SolveModelInstance(IGH_DataAccess DA) { }
}

public class SerializeRepresentationComponent : SerializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public SerializeRepresentationComponent() : base("Serialize Representation", "SR", "Serialize Representation to JSON") { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32046");

    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelNickname() => "Rep";
    protected override string GetModelDescription() => "Serialize Representation";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeRepresentationComponent : DeserializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public DeserializeRepresentationComponent() : base("Deserialize Representation", "DR", "Deserialize JSON to Representation") { }
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32047");

    protected override string GetModelTypeName() => "Representation";
    protected override string GetModelNickname() => "Rep";
    protected override string GetModelDescription() => "Deserialize Representation";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Representation

#region File

public class FileIdGoo : IdGoo<FileId>
{
    public FileIdGoo() { }
    public FileIdGoo(FileId value) : base(value) { }

    protected override string GetModelTypeName() => "FileId";
    protected override string GetModelDescription() => "Identifier for a File";
    protected override string GetSerializationKey() => "file_id";
    protected override ModelGoo<FileId> CreateDuplicate() => new FileIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Url);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new FileId { Url = str };
            return true;
        }
        return false;
    }
}

public class FileIdParam : IdParam<FileIdGoo, FileId>
{
    public FileIdParam() : base("FileId", "FI", "FileId parameter") { }
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E7");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class FileIdComponent : IdComponent<FileIdParam, FileIdGoo, FileId>
{
    public FileIdComponent() : base("FileId", "FI", "FileId component") { }
    public override Guid ComponentGuid => new("50C3D4E5-F6A7-B8C9-D0E1-F2A3B4C5D6E8");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new FileIdParam(), "File ID", "FId", "File identifier", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new FileIdParam(), "File ID", "FId", "File identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out FileIdGoo input)) DA.SetData(0, input); }
}

public class FileDiffGoo : DiffGoo<FileDiff>
{
    public FileDiffGoo() { }
    public FileDiffGoo(FileDiff value) : base(value) { }

    protected override string GetModelTypeName() => "FileDiff";
    protected override string GetModelDescription() => "File difference";
    protected override string GetSerializationKey() => "file_diff";
    protected override ModelGoo<FileDiff> CreateDuplicate() => new FileDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Url);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<FileDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FileDiffParam : DiffParam<FileDiffGoo, FileDiff>
{
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F0");

    protected override Bitmap GetParamIcon() => Icons.File;
}

public class FileDiffComponent : DiffComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F1");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Icons.File;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new FileDiffParam(), "File Diff", "FDiff", "File difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new FileDiffParam(), "File Diff", "FDiff", "File difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out FileDiffGoo input)) DA.SetData(0, input); }
}

public class SerializeFileDiffComponent : SerializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public SerializeFileDiffComponent() { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F2");

    protected override string GetModelTypeName() => "FileDiff";
    protected override string GetModelNickname() => "FDiff";
    protected override string GetModelDescription() => "Serialize File difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeFileDiffComponent : DeserializeComponent<FileDiffParam, FileDiffGoo, FileDiff>
{
    public DeserializeFileDiffComponent() { }
    public override Guid ComponentGuid => new("20D6E7F8-A9B0-C1D2-E3F4-A5B6C7D8E9F3");

    protected override string GetModelTypeName() => "FileDiff";
    protected override string GetModelNickname() => "FDiff";
    protected override string GetModelDescription() => "Deserialize File difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class FilesDiffGoo : DiffGoo<FilesDiff>
{
    public FilesDiffGoo() { }
    public FilesDiffGoo(FilesDiff value) : base(value) { }

    protected override string GetModelTypeName() => "FilesDiff";
    protected override string GetModelDescription() => "Files difference";
    protected override string GetSerializationKey() => "files_diff";
    protected override ModelGoo<FilesDiff> CreateDuplicate() => new FilesDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("FilesDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<FilesDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class FilesDiffParam : DiffParam<FilesDiffGoo, FilesDiff>
{
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A1");

    protected override Bitmap GetParamIcon() => Icons.File;
}

public class FilesDiffComponent : DiffComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A2");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Icons.File;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new FilesDiffParam(), "Files Diff", "FsDiff", "Files difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new FilesDiffParam(), "Files Diff", "FsDiff", "Files difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out FilesDiffGoo input)) DA.SetData(0, input); }
}

public class SerializeFilesDiffComponent : SerializeComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public SerializeFilesDiffComponent() { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A3");

    protected override string GetModelTypeName() => "FilesDiff";
    protected override string GetModelNickname() => "FsDiff";
    protected override string GetModelDescription() => "Serialize Files difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeFilesDiffComponent : DeserializeComponent<FilesDiffParam, FilesDiffGoo, FilesDiff>
{
    public DeserializeFilesDiffComponent() { }
    public override Guid ComponentGuid => new("30E7F8A9-B0C1-D2E3-F4A5-B6C7D8E9F0A4");

    protected override string GetModelTypeName() => "FilesDiff";
    protected override string GetModelNickname() => "FsDiff";
    protected override string GetModelDescription() => "Deserialize Files difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class FileGoo : ModelGoo<File>
{
    public FileGoo() { }
    public FileGoo(File value) : base(value) { }

    protected override string GetModelTypeName() => "File";
    protected override string GetModelDescription() => "A file";
    protected override string GetSerializationKey() => "file";
    protected override ModelGoo<File> CreateDuplicate() => new FileGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Url);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new File { Url = str };
            return true;
        }
        return false;
    }
}

public class FileParam : ModelParam<FileGoo, File>
{
    public FileParam() : base("File", "F", "File parameter") { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F8");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class FileComponent : ModelComponent<FileParam, FileGoo, File>
{
    public FileComponent() : base("File", "F", "File component") { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7F9");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new FileParam(), "File", "F", "File", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new FileParam(), "File", "F", "File", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { FileGoo? input = null; if (DA.GetData(0, ref input)) DA.SetData(0, input); }
}

public class SerializeFileComponent : SerializeComponent<FileParam, FileGoo, File>
{
    public SerializeFileComponent() : base("Serialize File", ">>F", "Serialize File") { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FA");

    protected override string GetModelTypeName() => "File";
    protected override string GetModelNickname() => "F";
    protected override string GetModelDescription() => "Serialize File";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeFileComponent : DeserializeComponent<FileParam, FileGoo, File>
{
    public DeserializeFileComponent() : base("Deserialize File", "<<F", "Deserialize File") { }
    public override Guid ComponentGuid => new("60D4E5F6-A7B8-C9D0-E1F2-A3B4C5D6E7FB");

    protected override string GetModelTypeName() => "File";
    protected override string GetModelNickname() => "F";
    protected override string GetModelDescription() => "Deserialize File";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion File

#region DiagramPoint

public class DiagramPointGoo : ModelGoo<DiagramPoint>
{
    public DiagramPointGoo() { }
    public DiagramPointGoo(DiagramPoint value) : base(value) { }

    protected override string GetModelTypeName() => "DiagramPoint";
    protected override string GetModelDescription() => "A point in the diagram";
    protected override string GetSerializationKey() => "diagram_point";
    protected override ModelGoo<DiagramPoint> CreateDuplicate() => new DiagramPointGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.X, Value.Y, 0));
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
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
    public DiagramPointParam() : base("DiagramPoint", "DP", "DiagramPoint parameter") { }
    public override Guid ComponentGuid => new("4685CCE8-C629-4638-8DF6-F76A17571841");

    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DiagramPointComponent : ModelComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public DiagramPointComponent() : base("DiagramPoint", "DP", "DiagramPoint component") { }
    public override Guid ComponentGuid => new("61FB9BBE-64DE-42B2-B7EF-69CD97FDD9E3");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new DiagramPointParam(), "Diagram Point", "DP", "Diagram point", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new DiagramPointParam(), "Diagram Point", "DP", "Diagram point", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { DiagramPointGoo? input = null; if (DA.GetData(0, ref input)) DA.SetData(0, input); }
}

public class SerializeDiagramPointComponent : SerializeComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public SerializeDiagramPointComponent() { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A99");

    protected override string GetModelTypeName() => "DiagramPoint";
    protected override string GetModelNickname() => "DP";
    protected override string GetModelDescription() => "Serialize Diagram Point";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeDiagramPointComponent : DeserializeComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public DeserializeDiagramPointComponent() { }
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A9A");

    protected override string GetModelTypeName() => "DiagramPoint";
    protected override string GetModelNickname() => "DP";
    protected override string GetModelDescription() => "Deserialize Diagram Point";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion DiagramPoint

#region Port

public class PortIdGoo : IdGoo<PortId>
{
    public PortIdGoo() { }
    public PortIdGoo(PortId value) : base(value) { }

    protected override string GetModelTypeName() => "PortId";
    protected override string GetModelDescription() => "Identifier for a Port";
    protected override string GetSerializationKey() => "port_id";
    protected override ModelGoo<PortId> CreateDuplicate() => new PortIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PortDiffGoo)))
        {
            target = (Q)(object)new PortDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PortGoo)))
        {
            target = (Q)(object)new PortGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is PortGoo portGoo)
        {
            Value = portGoo.Value;
            return true;
        }
        if (source is PortDiffGoo diffGoo)
        {
            Value = diffGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new PortId { Id = str };
            return true;
        }
        return false;
    }
}

public class PortIdParam : IdParam<PortIdGoo, PortId>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B1");

    protected override Bitmap GetParamIcon() => Icons.Port;
}

public class PortIdComponent : IdComponent<PortIdParam, PortIdGoo, PortId>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B2");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Icons.Port;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PortIdParam(), "Port ID", "PId", "Port identifier", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PortIdParam(), "Port ID", "PId", "Port identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out PortIdGoo input)) DA.SetData(0, input); }
}

public class PortDiffGoo : DiffGoo<PortDiff>
{
    public PortDiffGoo() { }
    public PortDiffGoo(PortDiff value) : base(value) { }

    protected override string GetModelTypeName() => "PortDiff";
    protected override string GetModelDescription() => "Port difference";
    protected override string GetSerializationKey() => "port_diff";
    protected override ModelGoo<PortDiff> CreateDuplicate() => new PortDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(PortIdGoo)))
        {
            target = (Q)(object)new PortIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PortGoo)))
        {
            target = (Q)(object)new PortGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is PortIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is PortGoo portGoo)
        {
            Value = portGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<PortDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PortDiffParam : DiffParam<PortDiffGoo, PortDiff>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B0");

    protected override Bitmap GetParamIcon() => Icons.Port;
}

public class PortDiffComponent : DiffComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B3");

    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Icons.Port;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PortDiffParam(), "Port Diff", "PDiff", "Port difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PortDiffParam(), "Port Diff", "PDiff", "Port difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out PortDiffGoo input)) DA.SetData(0, input); }
}

public class SerializePortDiffComponent : SerializeComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public SerializePortDiffComponent() { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B4");

    protected override string GetModelTypeName() => "PortDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Serialize Port difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePortDiffComponent : DeserializeComponent<PortDiffParam, PortDiffGoo, PortDiff>
{
    public DeserializePortDiffComponent() { }
    public override Guid ComponentGuid => new("80F6A7B8-C9D0-E1F2-A3B4-C5D6E7F8A9B5");

    protected override string GetModelTypeName() => "PortDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Deserialize Port difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class PortGoo : ModelGoo<Port>
{
    public PortGoo() { }
    public PortGoo(Port value) : base(value) { }

    protected override string GetModelTypeName() => "Port";
    protected override string GetModelDescription() => "A port";
    protected override string GetSerializationKey() => "port";
    protected override ModelGoo<Port> CreateDuplicate() => new PortGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Plane)))
        {
            if (Value.Direction is null || Value.Point is null) return false;
            target = (Q)(object)new GH_Plane(Utility.GetPlaneFromYAxis(Value.Direction.Convert(), 0, Value.Point.Convert()));
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PortIdGoo)))
        {
            target = (Q)(object)new PortIdGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(PortDiffGoo)))
        {
            target = (Q)(object)new PortDiffGoo(Value);
            return true;
        }
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is PortIdGoo idGoo)
        {
            Value = idGoo.Value;
            return true;
        }
        if (source is PortDiffGoo diffGoo)
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
            Value = str.Deserialize<Port>();
            return true;
        }
        return false;
    }
}

public class PortParam : ModelParam<PortGoo, Port>
{
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1B");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PortComponent : ModelComponent<PortParam, PortGoo, Port>
{
    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PortParam(), "Port", "P", "A port", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PortParam(), "Port", "P", "A port", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out PortGoo input)) DA.SetData(0, input); }
}

public class SerializePortComponent : SerializeComponent<PortParam, PortGoo, Port>
{
    public SerializePortComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B5");
    protected override string GetModelTypeName() => "Port";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Serialize Port";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePortComponent : DeserializeComponent<PortParam, PortGoo, Port>
{
    public DeserializePortComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B6");
    protected override string GetModelTypeName() => "Port";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Deserialize Port";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class PortsDiffGoo : DiffGoo<PortsDiff>
{
    public PortsDiffGoo() { }
    public PortsDiffGoo(PortsDiff value) : base(value) { }

    protected override string GetModelTypeName() => "PortsDiff";
    protected override string GetModelDescription() => "Ports difference";
    protected override string GetSerializationKey() => "ports_diff";
    protected override ModelGoo<PortsDiff> CreateDuplicate() => new PortsDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("PortsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<PortsDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PortsDiffParam : DiffParam<PortsDiffGoo, PortsDiff>
{
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C0");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PortsDiffComponent : DiffComponent<PortsDiffParam, PortsDiffGoo, PortsDiff>
{
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C1");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PortsDiffParam(), "Ports Diff", "PDiff", "Ports difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PortsDiffParam(), "Ports Diff", "PDiff", "Ports difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out PortsDiffGoo input)) DA.SetData(0, input); }
}

public class SerializePortsDiffComponent : SerializeComponent<PortsDiffParam, PortsDiffGoo, PortsDiff>
{
    public SerializePortsDiffComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C2");
    protected override string GetModelTypeName() => "PortsDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Serialize Ports difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePortsDiffComponent : DeserializeComponent<PortsDiffParam, PortsDiffGoo, PortsDiff>
{
    public DeserializePortsDiffComponent() { }
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1C3");
    protected override string GetModelTypeName() => "PortsDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Deserialize Ports difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Port

#region Author

public class AuthorIdGoo : IdGoo<AuthorId>
{
    public AuthorIdGoo() { }
    public AuthorIdGoo(AuthorId value) : base(value) { }

    protected override string GetModelTypeName() => "AuthorId";
    protected override string GetModelDescription() => "Author identifier";
    protected override string GetSerializationKey() => "author_id";
    protected override ModelGoo<AuthorId> CreateDuplicate() => new AuthorIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Email);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new AuthorId { Email = str };
            return true;
        }
        return false;
    }
}

public class AuthorIdParam : IdParam<AuthorIdGoo, AuthorId>
{
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1C");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AuthorIdComponent : IdComponent<AuthorIdParam, AuthorIdGoo, AuthorId>
{
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1D");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new AuthorIdParam(), "Author ID", "AId", "Author identifier", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new AuthorIdParam(), "Author ID", "AId", "Author identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out AuthorIdGoo input)) DA.SetData(0, input); }
}

public class AuthorGoo : ModelGoo<Author>
{
    public AuthorGoo() { }
    public AuthorGoo(Author value) : base(value) { }

    protected override string GetModelTypeName() => "Author";
    protected override string GetModelDescription() => "An author";
    protected override string GetSerializationKey() => "author";
    protected override ModelGoo<Author> CreateDuplicate() => new AuthorGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Email);
            return true;
        }
        return false;
    }
    protected override bool CustomCastFrom(object source)
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
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class AuthorComponent : ModelComponent<AuthorParam, AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("5143ED92-0A2C-4D0C-84ED-F90CC8450894");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new AuthorParam(), "Author", "A", "An author", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new AuthorParam(), "Author", "A", "An author", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out AuthorGoo input)) DA.SetData(0, input); }
}

public class SerializeAuthorComponent : SerializeComponent<AuthorParam, AuthorGoo, Author>
{
    public SerializeAuthorComponent() { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634878");
    protected override string GetModelTypeName() => "Author";
    protected override string GetModelNickname() => "A";
    protected override string GetModelDescription() => "Serialize Author";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeAuthorComponent : DeserializeComponent<AuthorParam, AuthorGoo, Author>
{
    public DeserializeAuthorComponent() { }
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634879");
    protected override string GetModelTypeName() => "Author";
    protected override string GetModelNickname() => "A";
    protected override string GetModelDescription() => "Deserialize Author";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Author

#region Location

public class LocationGoo : ModelGoo<Location>
{
    public LocationGoo() { }
    public LocationGoo(Location value) : base(value) { }

    protected override string GetModelTypeName() => "Location";
    protected override string GetModelDescription() => "A location";
    protected override string GetSerializationKey() => "location";
    protected override ModelGoo<Location> CreateDuplicate() => new LocationGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            target = (Q)(object)new GH_Point(new Point3d(Value.Longitude, Value.Latitude, 0));
            return true;
        }
        return false;
    }
    protected override bool CustomCastFrom(object source)
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
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class LocationComponent : ModelComponent<LocationParam, LocationGoo, Location>
{
    public override Guid ComponentGuid => new("6F2EDF42-6E10-4944-8B05-4D41F4876ED0");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new LocationParam(), "Location", "L", "A location", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new LocationParam(), "Location", "L", "A location", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out LocationGoo input)) DA.SetData(0, input); }
}

public class SerializeLocationComponent : SerializeComponent<LocationParam, LocationGoo, Location>
{
    public SerializeLocationComponent() { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D466");
    protected override string GetModelTypeName() => "Location";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "Serialize Location";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeLocationComponent : DeserializeComponent<LocationParam, LocationGoo, Location>
{
    public DeserializeLocationComponent() { }
    public override Guid ComponentGuid => new("DB94C7FC-3F0F-4FB4-992E-7E069C17D467");
    protected override string GetModelTypeName() => "Location";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "Deserialize Location";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Location

#region Type

public class TypeIdGoo : IdGoo<TypeId>
{
    public TypeIdGoo() { }
    public TypeIdGoo(TypeId value) : base(value) { }

    protected override string GetModelTypeName() => "TypeId";
    protected override string GetModelDescription() => "Type identifier";
    protected override string GetSerializationKey() => "type_id";
    protected override ModelGoo<TypeId> CreateDuplicate() => new TypeIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new TypeId { Name = str };
            return true;
        }
        return false;
    }
}

public class TypeIdParam : IdParam<TypeIdGoo, TypeId>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C2");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class TypeIdComponent : IdComponent<TypeIdParam, TypeIdGoo, TypeId>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C3");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new TypeIdParam(), "Type ID", "TId", "Type identifier", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new TypeIdParam(), "Type ID", "TId", "Type identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out TypeIdGoo input)) DA.SetData(0, input); }
}

public class TypeDiffGoo : DiffGoo<TypeDiff>
{
    public TypeDiffGoo() { }
    public TypeDiffGoo(TypeDiff value) : base(value) { }

    protected override string GetModelTypeName() => "TypeDiff";
    protected override string GetModelDescription() => "Type difference";
    protected override string GetSerializationKey() => "type_diff";
    protected override ModelGoo<TypeDiff> CreateDuplicate() => new TypeDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<TypeDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class TypeDiffParam : DiffParam<TypeDiffGoo, TypeDiff>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C1");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class TypeDiffComponent : DiffComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C4");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new TypeDiffParam(), "Type Diff", "TDiff", "Type difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new TypeDiffParam(), "Type Diff", "TDiff", "Type difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out TypeDiffGoo input)) DA.SetData(0, input); }
}

public class SerializeTypeDiffComponent : SerializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public SerializeTypeDiffComponent() { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C5");
    protected override string GetModelTypeName() => "TypeDiff";
    protected override string GetModelNickname() => "TDiff";
    protected override string GetModelDescription() => "Type difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeTypeDiffComponent : DeserializeComponent<TypeDiffParam, TypeDiffGoo, TypeDiff>
{
    public DeserializeTypeDiffComponent() { }
    public override Guid ComponentGuid => new("90A7B8C9-D0E1-F2A3-B4C5-D6E7F8A9B0C6");
    protected override string GetModelTypeName() => "TypeDiff";
    protected override string GetModelNickname() => "TDiff";
    protected override string GetModelDescription() => "Type difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class TypesDiffGoo : DiffGoo<TypesDiff>
{
    public TypesDiffGoo() { }
    public TypesDiffGoo(TypesDiff value) : base(value) { }

    protected override string GetModelTypeName() => "TypesDiff";
    protected override string GetModelDescription() => "Types difference";
    protected override string GetSerializationKey() => "types_diff";
    protected override ModelGoo<TypesDiff> CreateDuplicate() => new TypesDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("TypesDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<TypesDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class TypesDiffParam : DiffParam<TypesDiffGoo, TypesDiff>
{
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B6");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class TypesDiffComponent : DiffComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new TypesDiffParam(), "Types Diff", "TsDiff", "Types difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new TypesDiffParam(), "Types Diff", "TsDiff", "Types difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out TypesDiffGoo input)) DA.SetData(0, input); }
}

public class SerializeTypesDiffComponent : SerializeComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public SerializeTypesDiffComponent() { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B8");
    protected override string GetModelTypeName() => "TypesDiff";
    protected override string GetModelNickname() => "TsDiff";
    protected override string GetModelDescription() => "Types difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeTypesDiffComponent : DeserializeComponent<TypesDiffParam, TypesDiffGoo, TypesDiff>
{
    public DeserializeTypesDiffComponent() { }
    public override Guid ComponentGuid => new("E0F2A3B4-C5D6-E7F8-A9B0-C1D2E3F4A5B9");
    protected override string GetModelTypeName() => "TypesDiff";
    protected override string GetModelNickname() => "TsDiff";
    protected override string GetModelDescription() => "Types difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class TypeGoo : ModelGoo<Type>
{
    public TypeGoo() { }
    public TypeGoo(Type value) : base(value) { }

    protected override string GetModelTypeName() => "Type";
    protected override string GetModelDescription() => "Type";
    protected override string GetSerializationKey() => "type";
    protected override ModelGoo<Type> CreateDuplicate() => new TypeGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class TypeComponent : ModelComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new TypeParam(), "Type", "T", "Type", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new TypeParam(), "Type", "T", "Type", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out TypeGoo input)) { var type = input.Value; if (type.Unit == "") try { type.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem); } catch (Exception) { type.Unit = "m"; } type.Icon = type.Icon.Replace('\\', '/'); type.Image = type.Image.Replace('\\', '/'); DA.SetData(0, new TypeGoo(type)); } }
}

public class SerializeTypeComponent : SerializeComponent<TypeParam, TypeGoo, Type>
{
    public SerializeTypeComponent() { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673A");
    protected override string GetModelTypeName() => "Type";
    protected override string GetModelNickname() => "T";
    protected override string GetModelDescription() => "Type";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeTypeComponent : DeserializeComponent<TypeParam, TypeGoo, Type>
{
    public DeserializeTypeComponent() { }
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673B");
    protected override string GetModelTypeName() => "Type";
    protected override string GetModelNickname() => "T";
    protected override string GetModelDescription() => "Type";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Type

#region Piece

public class PieceIdGoo : IdGoo<PieceId>
{
    public PieceIdGoo() { }
    public PieceIdGoo(PieceId value) : base(value) { }

    protected override string GetModelTypeName() => "PieceId";
    protected override string GetModelDescription() => "Piece identifier";
    protected override string GetSerializationKey() => "piece_id";
    protected override ModelGoo<PieceId> CreateDuplicate() => new PieceIdGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new PieceId { Id = str };
            return true;
        }
        return false;
    }
}

public class PieceIdParam : IdParam<PieceIdGoo, PieceId>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D3");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PieceIdComponent : IdComponent<PieceIdParam, PieceIdGoo, PieceId>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D4");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PieceIdParam(), "Piece Id", "PId", "Piece identifier", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PieceIdParam(), "Piece Id", "PId", "Piece identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out PieceIdGoo input)) DA.SetData(0, input); }
}

public class PieceDiffGoo : DiffGoo<PieceDiff>
{
    public PieceDiffGoo() { }
    public PieceDiffGoo(PieceDiff value) : base(value) { }

    protected override string GetModelTypeName() => "PieceDiff";
    protected override string GetModelDescription() => "Piece difference";
    protected override string GetSerializationKey() => "piece_diff";
    protected override ModelGoo<PieceDiff> CreateDuplicate() => new PieceDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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
            target = (Q)(object)new GH_String(Value.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<PieceDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PieceDiffParam : DiffParam<PieceDiffGoo, PieceDiff>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D2");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PieceDiffComponent : DiffComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PieceDiffParam(), "Piece Diff", "PDiff", "Piece difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PieceDiffParam(), "Piece Diff", "PDiff", "Piece difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out PieceDiffGoo input)) DA.SetData(0, input); }
}

public class SerializePieceDiffComponent : SerializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public SerializePieceDiffComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D6");
    protected override string GetModelTypeName() => "PieceDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Piece difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePieceDiffComponent : DeserializeComponent<PieceDiffParam, PieceDiffGoo, PieceDiff>
{
    public DeserializePieceDiffComponent() { }
    public override Guid ComponentGuid => new("A0B8C9D0-E1F2-A3B4-C5D6-E7F8A9B0C1D7");
    protected override string GetModelTypeName() => "PieceDiff";
    protected override string GetModelNickname() => "PDiff";
    protected override string GetModelDescription() => "Piece difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class PiecesDiffGoo : DiffGoo<PiecesDiff>
{
    public PiecesDiffGoo() { }
    public PiecesDiffGoo(PiecesDiff value) : base(value) { }

    protected override string GetModelTypeName() => "PiecesDiff";
    protected override string GetModelDescription() => "Pieces difference";
    protected override string GetSerializationKey() => "pieces_diff";
    protected override ModelGoo<PiecesDiff> CreateDuplicate() => new PiecesDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("PiecesDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<PiecesDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class PiecesDiffParam : DiffParam<PiecesDiffGoo, PiecesDiff>
{
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C7");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PiecesDiffComponent : DiffComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C8");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PiecesDiffParam(), "Pieces Diff", "PsDiff", "Pieces difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PiecesDiffParam(), "Pieces Diff", "PsDiff", "Pieces difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out PiecesDiffGoo input)) DA.SetData(0, input); }
}

public class SerializePiecesDiffComponent : SerializeComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public SerializePiecesDiffComponent() { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6C9");
    protected override string GetModelTypeName() => "PiecesDiff";
    protected override string GetModelNickname() => "PsDiff";
    protected override string GetModelDescription() => "Pieces difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePiecesDiffComponent : DeserializeComponent<PiecesDiffParam, PiecesDiffGoo, PiecesDiff>
{
    public DeserializePiecesDiffComponent() { }
    public override Guid ComponentGuid => new("F0A3B4C5-D6E7-F8A9-B0C1-D2E3F4A5B6CA");
    protected override string GetModelTypeName() => "PiecesDiff";
    protected override string GetModelNickname() => "PsDiff";
    protected override string GetModelDescription() => "Pieces difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class PieceGoo : ModelGoo<Piece>
{
    public PieceGoo() { }
    public PieceGoo(Piece value) : base(value) { }

    protected override string GetModelTypeName() => "Piece";
    protected override string GetModelDescription() => "Piece";
    protected override string GetSerializationKey() => "piece";
    protected override ModelGoo<Piece> CreateDuplicate() => new PieceGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PieceComponent : ModelComponent<PieceParam, PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA48");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PieceParam(), "Piece", "P", "Piece", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PieceParam(), "Piece", "P", "Piece", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out PieceGoo input)) DA.SetData(0, input); }
}

public class SerializePieceComponent : SerializeComponent<PieceParam, PieceGoo, Piece>
{
    public SerializePieceComponent() { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00D9");
    protected override string GetModelTypeName() => "Piece";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Piece";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePieceComponent : DeserializeComponent<PieceParam, PieceGoo, Piece>
{
    public DeserializePieceComponent() { }
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00DA");
    protected override string GetModelTypeName() => "Piece";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Piece";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Piece

#region Side

public class SideDiffGoo : DiffGoo<SideDiff>
{
    public SideDiffGoo() { }
    public SideDiffGoo(SideDiff value) : base(value) { }

    protected override string GetModelTypeName() => "SideDiff";
    protected override string GetModelDescription() => "Side difference";
    protected override string GetSerializationKey() => "side_diff";
    protected override ModelGoo<SideDiff> CreateDuplicate() => new SideDiffGoo(Value);

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("SideDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<SideDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
}

public class SideDiffParam : DiffParam<SideDiffGoo, SideDiff>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E3");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class SideDiffComponent : DiffComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E4");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new SideDiffParam(), "Side Diff", "SDiff", "Side difference", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new SideDiffParam(), "Side Diff", "SDiff", "Side difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) { if (DA.GetData(0, out SideDiffGoo input)) DA.SetData(0, input); }
}

public class SerializeSideDiffComponent : SerializeComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public SerializeSideDiffComponent() { }
    public override Guid ComponentGuid => new("B1C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
    protected override string GetModelTypeName() => "SideDiff";
    protected override string GetModelNickname() => "SDiff";
    protected override string GetModelDescription() => "Side difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeSideDiffComponent : DeserializeComponent<SideDiffParam, SideDiffGoo, SideDiff>
{
    public DeserializeSideDiffComponent() { }
    public override Guid ComponentGuid => new("B2C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E6");
    protected override string GetModelTypeName() => "SideDiff";
    protected override string GetModelNickname() => "SDiff";
    protected override string GetModelDescription() => "Side difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class SideGoo : ModelGoo<Side>
{
    public SideGoo() { }
    public SideGoo(Side value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Piece.Id);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Side { Piece = new PieceId { Id = str } };
            return true;
        }
        return false;
    }
    protected override string GetModelTypeName() => "Side";
    protected override string GetModelDescription() => "Side of a piece";
    protected override string GetSerializationKey() => "side";
    protected override ModelGoo<Side> CreateDuplicate() => new SideGoo(Value);
}

public class SideParam : ModelParam<SideGoo, Side>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E5");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class SideComponent : ModelComponent<SideParam, SideGoo, Side>
{
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E6");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new SideParam(), "Side", "S", "Side", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new SideGoo(new Side()));
}

public class SerializeSideComponent : SerializeComponent<SideParam, SideGoo, Side>
{
    public SerializeSideComponent() { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E7");
    protected override string GetModelTypeName() => "Side";
    protected override string GetModelNickname() => "S";
    protected override string GetModelDescription() => "Side of a piece";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeSideComponent : DeserializeComponent<SideParam, SideGoo, Side>
{
    public DeserializeSideComponent() { }
    public override Guid ComponentGuid => new("B0C9D0E1-F2A3-B4C5-D6E7-F8A9B0C1D2E8");
    protected override string GetModelTypeName() => "Side";
    protected override string GetModelNickname() => "S";
    protected override string GetModelDescription() => "Side of a piece";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Side

#region Connection

public class ConnectionIdGoo : IdGoo<ConnectionId>
{
    public ConnectionIdGoo() { }
    public ConnectionIdGoo(ConnectionId value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<ConnectionId>();
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
    protected override string GetModelTypeName() => "ConnectionId";
    protected override string GetModelDescription() => "Connection identifier";
    protected override string GetSerializationKey() => "connection_id";
    protected override ModelGoo<ConnectionId> CreateDuplicate() => new ConnectionIdGoo(Value);
}

public class ConnectionIdParam : IdParam<ConnectionIdGoo, ConnectionId>
{
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D6");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class ConnectionIdComponent : IdComponent<ConnectionIdParam, ConnectionIdGoo, ConnectionId>
{
    public override Guid ComponentGuid => new("40B2C3D4-E5F6-A7B8-C9D0-E1F2A3B4C5D7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new ConnectionIdParam(), "ConnectionId", "CId", "Connection identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new ConnectionIdGoo(new ConnectionId()));
}

public class ConnectionDiffGoo : DiffGoo<ConnectionDiff>
{
    public ConnectionDiffGoo() { }
    public ConnectionDiffGoo(ConnectionDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<ConnectionDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
    protected override string GetModelTypeName() => "ConnectionDiff";
    protected override string GetModelDescription() => "Connection difference";
    protected override string GetSerializationKey() => "connection_diff";
    protected override ModelGoo<ConnectionDiff> CreateDuplicate() => new ConnectionDiffGoo(Value);
}

public class ConnectionDiffParam : DiffParam<ConnectionDiffGoo, ConnectionDiff>
{
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class ConnectionDiffComponent : DiffComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new ConnectionDiffParam(), "ConnectionDiff", "CDiff", "Connection difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new ConnectionDiffGoo(new ConnectionDiff()));
}

public class SerializeConnectionDiffComponent : SerializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public SerializeConnectionDiffComponent() { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F6");
    protected override string GetModelTypeName() => "ConnectionDiff";
    protected override string GetModelNickname() => "CDiff";
    protected override string GetModelDescription() => "Connection difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeConnectionDiffComponent : DeserializeComponent<ConnectionDiffParam, ConnectionDiffGoo, ConnectionDiff>
{
    public DeserializeConnectionDiffComponent() { }
    public override Guid ComponentGuid => new("C0D0E1F2-A3B4-C5D6-E7F8-A9B0C1D2E3F7");
    protected override string GetModelTypeName() => "ConnectionDiff";
    protected override string GetModelNickname() => "CDiff";
    protected override string GetModelDescription() => "Connection difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class ConnectionsDiffGoo : DiffGoo<ConnectionsDiff>
{
    public ConnectionsDiffGoo() { }
    public ConnectionsDiffGoo(ConnectionsDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("ConnectionsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<ConnectionsDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
    protected override string GetModelTypeName() => "ConnectionsDiff";
    protected override string GetModelDescription() => "Connections difference";
    protected override string GetSerializationKey() => "connections_diff";
    protected override ModelGoo<ConnectionsDiff> CreateDuplicate() => new ConnectionsDiffGoo(Value);
}

public class ConnectionsDiffParam : DiffParam<ConnectionsDiffGoo, ConnectionsDiff>
{
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D8");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class ConnectionsDiffComponent : DiffComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7D9");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new ConnectionsDiffParam(), "ConnectionsDiff", "CsDiff", "Connections difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new ConnectionsDiffGoo(new ConnectionsDiff()));
}

public class SerializeConnectionsDiffComponent : SerializeComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public SerializeConnectionsDiffComponent() { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7DA");
    protected override string GetModelTypeName() => "ConnectionsDiff";
    protected override string GetModelNickname() => "CsDiff";
    protected override string GetModelDescription() => "Connections difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeConnectionsDiffComponent : DeserializeComponent<ConnectionsDiffParam, ConnectionsDiffGoo, ConnectionsDiff>
{
    public DeserializeConnectionsDiffComponent() { }
    public override Guid ComponentGuid => new("00B4C5D6-E7F8-A9B0-C1D2-E3F4A5B6C7DB");
    protected override string GetModelTypeName() => "ConnectionsDiff";
    protected override string GetModelNickname() => "CsDiff";
    protected override string GetModelDescription() => "Connections difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class ConnectionGoo : ModelGoo<Connection>
{
    public ConnectionGoo() { }
    public ConnectionGoo(Connection value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<Connection>();
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
    protected override string GetModelTypeName() => "Connection";
    protected override string GetModelDescription() => "Connection between pieces";
    protected override string GetSerializationKey() => "connection";
    protected override ModelGoo<Connection> CreateDuplicate() => new ConnectionGoo(Value);
}

public class ConnectionParam : ModelParam<ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class ConnectionComponent : ModelComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new ConnectionParam(), "Connection", "C", "Connection", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new ConnectionGoo(new Connection()));
}

public class SerializeConnectionComponent : SerializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public SerializeConnectionComponent() { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD60");
    protected override string GetModelTypeName() => "Connection";
    protected override string GetModelNickname() => "C";
    protected override string GetModelDescription() => "Connection between pieces";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeConnectionComponent : DeserializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public DeserializeConnectionComponent() { }
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD61");
    protected override string GetModelTypeName() => "Connection";
    protected override string GetModelNickname() => "C";
    protected override string GetModelDescription() => "Connection between pieces";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Connection

#region Design

public class DesignIdGoo : IdGoo<DesignId>
{
    public DesignIdGoo() { }
    public DesignIdGoo(DesignId value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new DesignId { Name = str };
            return true;
        }
        return false;
    }
    protected override string GetModelTypeName() => "DesignId";
    protected override string GetModelDescription() => "Design identifier";
    protected override string GetSerializationKey() => "design_id";
    protected override ModelGoo<DesignId> CreateDuplicate() => new DesignIdGoo(Value);
}

public class DesignIdParam : IdParam<DesignIdGoo, DesignId>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A6");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DesignIdComponent : IdComponent<DesignIdParam, DesignIdGoo, DesignId>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new DesignIdParam(), "DesignId", "DId", "Design identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new DesignIdGoo(new DesignId()));
}

public class DesignDiffGoo : DiffGoo<DesignDiff>
{
    public DesignDiffGoo() { }
    public DesignDiffGoo(DesignDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<DesignDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }
    protected override string GetModelTypeName() => "DesignDiff";
    protected override string GetModelDescription() => "Design difference";
    protected override string GetSerializationKey() => "design_diff";
    protected override ModelGoo<DesignDiff> CreateDuplicate() => new DesignDiffGoo(Value);
}

public class DesignDiffParam : DiffParam<DesignDiffGoo, DesignDiff>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A5");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DesignDiffComponent : DiffComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A8");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new DesignDiffParam(), "DesignDiff", "DDiff", "Design difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new DesignDiffGoo(new DesignDiff()));
}

public class SerializeDesignDiffComponent : SerializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public SerializeDesignDiffComponent() { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4A9");
    protected override string GetModelTypeName() => "DesignDiff";
    protected override string GetModelNickname() => "DDiff";
    protected override string GetModelDescription() => "Design difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeDesignDiffComponent : DeserializeComponent<DesignDiffParam, DesignDiffGoo, DesignDiff>
{
    public DeserializeDesignDiffComponent() { }
    public override Guid ComponentGuid => new("D0E1F2A3-B4C5-D6E7-F8A9-B0C1D2E3F4AA");
    protected override string GetModelTypeName() => "DesignDiff";
    protected override string GetModelNickname() => "DDiff";
    protected override string GetModelDescription() => "Design difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DesignsDiffGoo : DiffGoo<DesignsDiff>
{
    public DesignsDiffGoo() { }
    public DesignsDiffGoo(DesignsDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("DesignsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<DesignsDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override string GetModelTypeName() => "DesignsDiff";
    protected override string GetModelDescription() => "Designs difference";
    protected override string GetSerializationKey() => "designs_diff";
    protected override ModelGoo<DesignsDiff> CreateDuplicate() => new DesignsDiffGoo(Value);
}

public class DesignsDiffParam : DiffParam<DesignsDiffGoo, DesignsDiff>
{
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8E9");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DesignsDiffComponent : DiffComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EA");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new DesignsDiffParam(), "DesignsDiff", "DsDiff", "Designs difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new DesignsDiffGoo(new DesignsDiff()));
}

public class SerializeDesignsDiffComponent : SerializeComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public SerializeDesignsDiffComponent() { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EB");
    protected override string GetModelTypeName() => "DesignsDiff";
    protected override string GetModelNickname() => "DsDiff";
    protected override string GetModelDescription() => "Designs difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeDesignsDiffComponent : DeserializeComponent<DesignsDiffParam, DesignsDiffGoo, DesignsDiff>
{
    public DeserializeDesignsDiffComponent() { }
    public override Guid ComponentGuid => new("10C5D6E7-F8A9-B0C1-D2E3-F4A5B6C7D8EC");
    protected override string GetModelTypeName() => "DesignsDiff";
    protected override string GetModelNickname() => "DsDiff";
    protected override string GetModelDescription() => "Designs difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DesignGoo : ModelGoo<Design>
{
    public DesignGoo() { }
    public DesignGoo(Design value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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

    protected override string GetModelTypeName() => "Design";
    protected override string GetModelDescription() => "Design";
    protected override string GetSerializationKey() => "design";
    protected override ModelGoo<Design> CreateDuplicate() => new DesignGoo(Value);
}

public class DesignParam : ModelParam<DesignGoo, Design>
{
    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class DesignComponent : ModelComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("AAD8D144-2EEE-48F1-A8A9-52977E86CB54");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new DesignParam(), "Design", "D", "Design", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new DesignParam(), "Design", "D", "Processed design", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        DesignGoo designGoo = null;
        if (!DA.GetData(0, ref designGoo)) return;
        Design design = designGoo.Value;
        if (design.Unit == "")
            try { design.Unit = Utility.LengthUnitSystemToAbbreviation(RhinoDoc.ActiveDoc.ModelUnitSystem); }
            catch (Exception) { design.Unit = "m"; }
        design.Icon = design.Icon.Replace('\\', '/');
        design.Image = design.Image.Replace('\\', '/');
        DA.SetData(0, new DesignGoo(design));
    }
}

public class SerializeDesignComponent : SerializeComponent<DesignParam, DesignGoo, Design>
{
    public SerializeDesignComponent() { }
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");
    protected override string GetModelTypeName() => "Design";
    protected override string GetModelNickname() => "D";
    protected override string GetModelDescription() => "Design";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeDesignComponent : DeserializeComponent<DesignParam, DesignGoo, Design>
{
    public DeserializeDesignComponent() { }
    protected override string GetModelTypeName() => "Design";
    protected override string GetModelNickname() => "D";
    protected override string GetModelDescription() => "Design";
    protected override Guid GetComponentGuid() => ComponentGuid;
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB59");
}

#endregion Design

#region Kit

public class KitIdGoo : IdGoo<KitId>
{
    public KitIdGoo() { }
    public KitIdGoo(KitId value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
            Value = new KitId { Name = str };
            return true;
        }
        return false;
    }

    protected override string GetModelTypeName() => "KitId";
    protected override string GetModelDescription() => "Kit identifier";
    protected override string GetSerializationKey() => "kit_id";
    protected override ModelGoo<KitId> CreateDuplicate() => new KitIdGoo(Value);
}

public class KitIdParam : IdParam<KitIdGoo, KitId>
{
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B0");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class KitIdComponent : IdComponent<KitIdParam, KitIdGoo, KitId>
{
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B1");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new KitIdParam(), "KitId", "KId", "Kit identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new KitIdGoo(new KitId()));
}

public class KitDiffGoo : DiffGoo<KitDiff>
{
    public KitDiffGoo() { }
    public KitDiffGoo(KitDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (source is KitGoo kitGoo)
        {
            Value = kitGoo.Value;
            return true;
        }
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<KitDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override string GetModelTypeName() => "KitDiff";
    protected override string GetModelDescription() => "Kit difference";
    protected override string GetSerializationKey() => "kit_diff";
    protected override ModelGoo<KitDiff> CreateDuplicate() => new KitDiffGoo(Value);
}

public class KitDiffParam : DiffParam<KitDiffGoo, KitDiff>
{
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B2");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class KitDiffComponent : DiffComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B3");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new KitDiffParam(), "KitDiff", "KDiff", "Kit difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new KitDiffGoo(new KitDiff()));
}

public class SerializeKitDiffComponent : SerializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public SerializeKitDiffComponent() { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B4");
    protected override string GetModelTypeName() => "KitDiff";
    protected override string GetModelNickname() => "KDiff";
    protected override string GetModelDescription() => "Kit difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeKitDiffComponent : DeserializeComponent<KitDiffParam, KitDiffGoo, KitDiff>
{
    public DeserializeKitDiffComponent() { }
    public override Guid ComponentGuid => new("40F8A9B0-C1D2-E3F4-A5B6-C7D8E9F0A1B5");
    protected override string GetModelTypeName() => "KitDiff";
    protected override string GetModelNickname() => "KDiff";
    protected override string GetModelDescription() => "Kit difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class KitGoo : ModelGoo<Kit>
{
    public KitGoo() { }
    public KitGoo(Kit value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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

    protected override string GetModelTypeName() => "Kit";
    protected override string GetModelDescription() => "Kit";
    protected override string GetSerializationKey() => "kit";
    protected override ModelGoo<Kit> CreateDuplicate() => new KitGoo(Value);
}

public class KitParam : ModelParam<KitGoo, Kit>
{
    public override Guid ComponentGuid => new("BA9F161E-AFE3-41D5-8644-964DD20B887B");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class KitComponent : ModelComponent<KitParam, KitGoo, Kit>
{
    public override Guid ComponentGuid => new("987560A8-10D4-43F6-BEBE-D71DC2FD86AF");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new KitParam(), "Kit", "K", "Kit", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new KitParam(), "Kit", "K", "Processed kit", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        KitGoo kitGoo = null;
        if (!DA.GetData(0, ref kitGoo)) return;
        Kit kit = kitGoo.Value;
        kit.Icon = kit.Icon.Replace('\\', '/');
        kit.Image = kit.Image.Replace('\\', '/');
        kit.Preview = kit.Preview.Replace('\\', '/');
        DA.SetData(0, new KitGoo(kit));
    }
}

public class SerializeKitComponent : SerializeComponent<KitParam, KitGoo, Kit>
{
    public SerializeKitComponent() { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4165");
    protected override string GetModelTypeName() => "Kit";
    protected override string GetModelNickname() => "K";
    protected override string GetModelDescription() => "Kit";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeKitComponent : DeserializeComponent<KitParam, KitGoo, Kit>
{
    public DeserializeKitComponent() { }
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4166");
    protected override string GetModelTypeName() => "Kit";
    protected override string GetModelNickname() => "K";
    protected override string GetModelDescription() => "Kit";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class KitsDiffGoo : DiffGoo<KitsDiff>
{
    public KitsDiffGoo() { }
    public KitsDiffGoo(KitsDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String("KitsDiff");
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<KitsDiff>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override string GetModelTypeName() => "KitsDiff";
    protected override string GetModelDescription() => "Kits difference";
    protected override string GetSerializationKey() => "kits_diff";
    protected override ModelGoo<KitsDiff> CreateDuplicate() => new KitsDiffGoo(Value);
}

public class KitsDiffParam : DiffParam<KitsDiffGoo, KitsDiff>
{
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C3");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class KitsDiffComponent : DiffComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C4");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new KitsDiffParam(), "KitsDiff", "KsDiff", "Kits difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new KitsDiffGoo(new KitsDiff()));
}

public class SerializeKitsDiffComponent : SerializeComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public SerializeKitsDiffComponent() { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C5");
    protected override string GetModelTypeName() => "KitsDiff";
    protected override string GetModelNickname() => "KsDiff";
    protected override string GetModelDescription() => "Kits difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeKitsDiffComponent : DeserializeComponent<KitsDiffParam, KitsDiffGoo, KitsDiff>
{
    public DeserializeKitsDiffComponent() { }
    public override Guid ComponentGuid => new("50A9B0C1-D2E3-F4A5-B6C7-D8E9F0A1B2C6");
    protected override string GetModelTypeName() => "KitsDiff";
    protected override string GetModelNickname() => "KsDiff";
    protected override string GetModelDescription() => "Kits difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Kit

#region Quality

public class QualityKindGoo : EnumGoo<QualityKind>
{
    public QualityKindGoo() { }
    public QualityKindGoo(QualityKind value) : base(value) { }
}

public class QualityKindParam : EnumParam<QualityKindGoo, QualityKind>
{
    public QualityKindParam() : base(new("A1B2C3D4-E5F6-4A5B-9C8D-7E6F5A4B3C2D")) { }
}

public class QualityIdGoo : IdGoo<QualityId>
{
    public QualityIdGoo() { }
    public QualityIdGoo(QualityId value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<QualityId>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityGoo(QualityIdGoo idGoo) => new(idGoo.Value);
    public static implicit operator QualityIdGoo(QualityGoo goo) => new((QualityId)goo.Value);

    protected override string GetModelTypeName() => "QualityId";
    protected override string GetModelDescription() => "Quality identifier";
    protected override string GetSerializationKey() => "quality_id";
    protected override ModelGoo<QualityId> CreateDuplicate() => new QualityIdGoo(Value);
}

public class QualityIdParam : IdParam<QualityIdGoo, QualityId>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class QualityIdComponent : IdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new QualityIdParam(), "QualityId", "QId", "Quality identifier", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new QualityIdGoo(new QualityId()));
}

public class SerializeQualityIdComponent : SerializeIdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public SerializeQualityIdComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CA");
    protected override string GetModelTypeName() => "QualityId";
    protected override string GetModelNickname() => "QId";
    protected override string GetModelDescription() => "Quality identifier";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeQualityIdComponent : DeserializeIdComponent<QualityIdParam, QualityIdGoo, QualityId>
{
    public DeserializeQualityIdComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4CB");
    protected override string GetModelTypeName() => "QualityId";
    protected override string GetModelNickname() => "QId";
    protected override string GetModelDescription() => "Quality identifier";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class QualityDiffGoo : DiffGoo<QualityDiff>
{
    public QualityDiffGoo() { }
    public QualityDiffGoo(QualityDiff value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<QualityDiff>();
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

    protected override string GetModelTypeName() => "QualityDiff";
    protected override string GetModelDescription() => "Quality difference";
    protected override string GetSerializationKey() => "quality_diff";
    protected override ModelGoo<QualityDiff> CreateDuplicate() => new QualityDiffGoo(Value);
}

public class QualityDiffParam : DiffParam<QualityDiffGoo, QualityDiff>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DA");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class QualityDiffComponent : DiffComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DB");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) { }
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new QualityDiffParam(), "QualityDiff", "QDiff", "Quality difference", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA) => DA.SetData(0, new QualityDiffGoo(new QualityDiff()));
}

public class SerializeQualityDiffComponent : SerializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public SerializeQualityDiffComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DC");
    protected override string GetModelTypeName() => "QualityDiff";
    protected override string GetModelNickname() => "QDiff";
    protected override string GetModelDescription() => "Quality difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeQualityDiffComponent : DeserializeComponent<QualityDiffParam, QualityDiffGoo, QualityDiff>
{
    public DeserializeQualityDiffComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4DD");
    protected override string GetModelTypeName() => "QualityDiff";
    protected override string GetModelNickname() => "QDiff";
    protected override string GetModelDescription() => "Quality difference";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class QualityGoo : ModelGoo<Quality>
{
    public QualityGoo() { }
    public QualityGoo(Quality value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
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

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
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
                Value = str.Deserialize<Quality>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    public static implicit operator QualityIdGoo(QualityGoo goo) => new((QualityId)goo.Value);
    public static implicit operator QualityGoo(QualityIdGoo idGoo) => new((Quality)idGoo.Value);

    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelDescription() => "Quality";
    protected override string GetSerializationKey() => "quality";
    protected override ModelGoo<Quality> CreateDuplicate() => new QualityGoo(Value);
}

public class QualityParam : ModelParam<QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class QualityComponent : ModelComponent<QualityParam, QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new QualityParam(), "Quality", "Q", "Quality", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new QualityParam(), "Quality", "Q", "Quality", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        QualityGoo qualityGoo = null;
        if (!DA.GetData(0, ref qualityGoo)) return;
        DA.SetData(0, qualityGoo);
    }
}

public class SerializeQualityComponent : SerializeComponent<QualityParam, QualityGoo, Quality>
{
    public SerializeQualityComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C8");
    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelNickname() => "Q";
    protected override string GetModelDescription() => "Quality";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeQualityComponent : DeserializeComponent<QualityParam, QualityGoo, Quality>
{
    public DeserializeQualityComponent() { }
    public override Guid ComponentGuid => new("50A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C9");
    protected override string GetModelTypeName() => "Quality";
    protected override string GetModelNickname() => "Q";
    protected override string GetModelDescription() => "Quality";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Quality

#region Benchmark

public class BenchmarkGoo : ModelGoo<Benchmark>
{
    public BenchmarkGoo() { }
    public BenchmarkGoo(Benchmark value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            Value = new Benchmark { Name = str };
            return true;
        }
        return false;
    }

    protected override string GetModelTypeName() => "Benchmark";
    protected override string GetModelDescription() => "Benchmark";
    protected override string GetSerializationKey() => "benchmark";
    protected override ModelGoo<Benchmark> CreateDuplicate() => new BenchmarkGoo(Value);
}

public class BenchmarkParam : ModelParam<BenchmarkGoo, Benchmark>
{
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class BenchmarkComponent : ModelComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new BenchmarkParam(), "Benchmark", "B", "Benchmark", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new BenchmarkParam(), "Benchmark", "B", "Benchmark", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        BenchmarkGoo benchmarkGoo = null;
        if (!DA.GetData(0, ref benchmarkGoo)) return;
        DA.SetData(0, benchmarkGoo);
    }
}

public class SerializeBenchmarkComponent : SerializeComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public SerializeBenchmarkComponent() { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Benchmark";
    protected override string GetModelNickname() => "B";
    protected override string GetModelDescription() => "Benchmark";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeBenchmarkComponent : DeserializeComponent<BenchmarkParam, BenchmarkGoo, Benchmark>
{
    public DeserializeBenchmarkComponent() { }
    public override Guid ComponentGuid => new("60A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Benchmark";
    protected override string GetModelNickname() => "B";
    protected override string GetModelDescription() => "Benchmark";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Benchmark

#region Prop

public class PropGoo : ModelGoo<Prop>
{
    public PropGoo() { }
    public PropGoo(Prop value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<Prop>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override string GetModelTypeName() => "Prop";
    protected override string GetModelDescription() => "Property";
    protected override string GetSerializationKey() => "prop";
    protected override ModelGoo<Prop> CreateDuplicate() => new PropGoo(Value);
}

public class PropParam : ModelParam<PropGoo, Prop>
{
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class PropComponent : ModelComponent<PropParam, PropGoo, Prop>
{
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new PropParam(), "Prop", "P", "Property", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new PropParam(), "Prop", "P", "Property", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        PropGoo propGoo = null;
        if (!DA.GetData(0, ref propGoo)) return;
        DA.SetData(0, propGoo);
    }
}

public class SerializePropComponent : SerializeComponent<PropParam, PropGoo, Prop>
{
    public SerializePropComponent() { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Prop";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Property";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializePropComponent : DeserializeComponent<PropParam, PropGoo, Prop>
{
    public DeserializePropComponent() { }
    public override Guid ComponentGuid => new("70A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Prop";
    protected override string GetModelNickname() => "P";
    protected override string GetModelDescription() => "Property";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Prop

#region Stat

public class StatGoo : ModelGoo<Stat>
{
    public StatGoo() { }
    public StatGoo(Stat value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Key);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<Stat>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override string GetModelTypeName() => "Stat";
    protected override string GetModelDescription() => "Statistic";
    protected override string GetSerializationKey() => "stat";
    protected override ModelGoo<Stat> CreateDuplicate() => new StatGoo(Value);
}

public class StatParam : ModelParam<StatGoo, Stat>
{
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class StatComponent : ModelComponent<StatParam, StatGoo, Stat>
{
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new StatParam(), "Stat", "S", "Statistic", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new StatParam(), "Stat", "S", "Statistic", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        StatGoo statGoo = null;
        if (!DA.GetData(0, ref statGoo)) return;
        DA.SetData(0, statGoo);
    }
}

public class SerializeStatComponent : SerializeComponent<StatParam, StatGoo, Stat>
{
    public SerializeStatComponent() { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Stat";
    protected override string GetModelNickname() => "S";
    protected override string GetModelDescription() => "Statistic";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeStatComponent : DeserializeComponent<StatParam, StatGoo, Stat>
{
    public DeserializeStatComponent() { }
    public override Guid ComponentGuid => new("80A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Stat";
    protected override string GetModelNickname() => "S";
    protected override string GetModelDescription() => "Statistic";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Stat

#region Layer

public class LayerGoo : ModelGoo<Layer>
{
    public LayerGoo() { }
    public LayerGoo(Layer value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<Layer>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override string GetModelTypeName() => "Layer";
    protected override string GetModelDescription() => "Layer";
    protected override string GetSerializationKey() => "layer";
    protected override ModelGoo<Layer> CreateDuplicate() => new LayerGoo(Value);
}

public class LayerParam : ModelParam<LayerGoo, Layer>
{
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class LayerComponent : ModelComponent<LayerParam, LayerGoo, Layer>
{
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new LayerParam(), "Layer", "L", "Layer", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new LayerParam(), "Layer", "L", "Layer", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        LayerGoo layerGoo = null;
        if (!DA.GetData(0, ref layerGoo)) return;
        DA.SetData(0, layerGoo);
    }
}

public class SerializeLayerComponent : SerializeComponent<LayerParam, LayerGoo, Layer>
{
    public SerializeLayerComponent() { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Layer";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "Layer";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeLayerComponent : DeserializeComponent<LayerParam, LayerGoo, Layer>
{
    public DeserializeLayerComponent() { }
    public override Guid ComponentGuid => new("90A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Layer";
    protected override string GetModelNickname() => "L";
    protected override string GetModelDescription() => "Layer";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Layer

#region Group

public class GroupGoo : ModelGoo<Group>
{
    public GroupGoo() { }
    public GroupGoo(Group value) : base(value) { }

    protected override bool CustomCastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            target = (Q)(object)new GH_String(Value.Name);
            return true;
        }
        return false;
    }

    protected override bool CustomCastFrom(object source)
    {
        if (source == null) return false;
        if (GH_Convert.ToString(source, out string str, GH_Conversion.Both))
        {
            try
            {
                Value = str.Deserialize<Group>();
                return true;
            }
            catch { return false; }
        }
        return false;
    }

    protected override string GetModelTypeName() => "Group";
    protected override string GetModelDescription() => "Group";
    protected override string GetSerializationKey() => "group";
    protected override ModelGoo<Group> CreateDuplicate() => new GroupGoo(Value);
}

public class GroupParam : ModelParam<GroupGoo, Group>
{
    public GroupParam() : base("Group", "G", "Group parameter") { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C4");
    protected override Bitmap GetParamIcon() => Resources.semio_24x24;
}

public class GroupComponent : ModelComponent<GroupParam, GroupGoo, Group>
{
    public GroupComponent() : base("Group", "G", "Group component") { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C5");
    protected override Guid GetComponentGuid() => ComponentGuid;
    protected override Bitmap GetComponentIcon() => Resources.semio_24x24;
    protected override void RegisterModelInputs(GH_InputParamManager pManager) => pManager.AddParameter(new GroupParam(), "Group", "G", "Group", GH_ParamAccess.item);
    protected override void RegisterModelOutputs(GH_OutputParamManager pManager) => pManager.AddParameter(new GroupParam(), "Group", "G", "Group", GH_ParamAccess.item);
    protected override void SolveModelInstance(IGH_DataAccess DA)
    {
        GroupGoo? groupGoo = null;
        if (!DA.GetData(0, ref groupGoo)) return;
        DA.SetData(0, groupGoo);
    }
}

public class SerializeGroupComponent : SerializeComponent<GroupParam, GroupGoo, Group>
{
    public SerializeGroupComponent() : base("Serialize Group", ">>G", "Serialize a group") { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C6");
    protected override string GetModelTypeName() => "Group";
    protected override string GetModelNickname() => "G";
    protected override string GetModelDescription() => "Group";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

public class DeserializeGroupComponent : DeserializeComponent<GroupParam, GroupGoo, Group>
{
    public DeserializeGroupComponent() : base("Deserialize Group", "<<G", "Deserialize a group") { }
    public override Guid ComponentGuid => new("A0A1B2C3-D4E5-F6A7-B8C9-D0E1F2A3B4C7");
    protected override string GetModelTypeName() => "Group";
    protected override string GetModelNickname() => "G";
    protected override string GetModelDescription() => "Group";
    protected override Guid GetComponentGuid() => ComponentGuid;
}

#endregion Group

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
            { typeof(Plane), (typeof(GH_Plane), typeof(Param_Plane)) },
        };
        var assemblyTypes = Assembly.GetExecutingAssembly().GetTypes();
        var enumGooTypes = assemblyTypes.Where(t =>
            t.BaseType?.IsGenericType == true &&
            t.BaseType.GetGenericTypeDefinition() == typeof(EnumGoo<>)).ToList();
        var enumParamTypes = assemblyTypes.Where(t =>
            t.BaseType?.IsGenericType == true &&
            t.BaseType.GetGenericTypeDefinition() == typeof(EnumParam<,>)).ToList();
        foreach (var gooType in enumGooTypes)
        {
            var enumType = gooType.BaseType!.GetGenericArguments()[0];
            if (!enumType.IsDefined(typeof(EnumAttribute), false)) continue;
            var paramType = enumParamTypes.FirstOrDefault(p =>
                p.BaseType!.GetGenericArguments()[1] == enumType);
            if (paramType != null)
                manualMappedTypes[enumType] = (gooType, paramType);
        }
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
                isPropertyMappedValue = manualMappedTypes.ContainsKey(Semio.Meta.Type[propertyTypeName]);
                isPropertyMapped[modelKvp.Key].Add(isPropertyMappedValue);
                propertyGoo[modelKvp.Key].Add(goo[isPropertyList ? propertyTypeName + "List" : propertyTypeName]);
                propertyItemGoo[modelKvp.Key].Add(goo[propertyTypeName]);
                propertyParam[modelKvp.Key].Add(param[propertyTypeName]);
                System.IO.File.WriteAllText("C:\\git\\semio.tech\\semio\\temp\\properties\\" + propertyTypeName + ".txt", propertyTypeName + " " + isPropertyMapped[modelKvp.Key] + " " + propertyGoo[modelKvp.Key] + " " + propertyItemGoo[modelKvp.Key] + " " + propertyParam[modelKvp.Key]);

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

